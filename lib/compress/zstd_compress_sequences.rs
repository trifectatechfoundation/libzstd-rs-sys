use libc::size_t;

use crate::lib::common::bitstream::{
    BIT_addBits, BIT_closeCStream, BIT_flushBits, BIT_initCStream, BitContainerType,
    STREAM_ACCUMULATOR_MIN,
};
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{
    FSE_CState_t, FSE_CTable, FSE_bitCost, FSE_encodeSymbol, FSE_flushCState, FSE_initCState,
    FSE_initCState2, FSE_repeat, FSE_repeat_check, FSE_repeat_none, FSE_repeat_valid,
    FSE_writeU16Pair, FSE_BUILD_CTABLE_WORKSPACE_SIZE, FSE_CTABLE_SIZE_U32,
};
use crate::lib::common::mem::MEM_32bits;
use crate::lib::common::zstd_internal::{
    DefaultMaxOff, LLFSELog, LL_bits, LL_defaultNorm, LL_defaultNormLog, MLFSELog, ML_bits,
    ML_defaultNorm, ML_defaultNormLog, MaxLL, MaxML, OF_defaultNorm, OF_defaultNormLog, OffFSELog,
    SymbolEncodingType,
};
use crate::lib::compress::fse_compress::{
    FSE_buildCTable_rle, FSE_buildCTable_wksp, FSE_normalizeCount, FSE_optimalTableLog,
    FSE_writeNCount,
};
use crate::lib::compress::zstd_compress::{DefaultPolicy, SeqDef};
use crate::lib::zstd::{ZSTD_lazy, ZSTD_strategy};
use crate::ZSTD_isError;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_BuildCTableWksp {
    pub norm: [i16; 53],
    pub wksp: [u32; 285],
}

/// -log2(x / 256) lookup table for x in [0, 256).
/// If x == 0: Return 0
/// Else: Return floor(-log2(x / 256) * 256)
static kInverseProbabilityLog256: [core::ffi::c_uint; 256] = [
    0, 2048, 1792, 1642, 1536, 1453, 1386, 1329, 1280, 1236, 1197, 1162, 1130, 1100, 1073, 1047,
    1024, 1001, 980, 960, 941, 923, 906, 889, 874, 859, 844, 830, 817, 804, 791, 779, 768, 756,
    745, 734, 724, 714, 704, 694, 685, 676, 667, 658, 650, 642, 633, 626, 618, 610, 603, 595, 588,
    581, 574, 567, 561, 554, 548, 542, 535, 529, 523, 517, 512, 506, 500, 495, 489, 484, 478, 473,
    468, 463, 458, 453, 448, 443, 438, 434, 429, 424, 420, 415, 411, 407, 402, 398, 394, 390, 386,
    382, 377, 373, 370, 366, 362, 358, 354, 350, 347, 343, 339, 336, 332, 329, 325, 322, 318, 315,
    311, 308, 305, 302, 298, 295, 292, 289, 286, 282, 279, 276, 273, 270, 267, 264, 261, 258, 256,
    253, 250, 247, 244, 241, 239, 236, 233, 230, 228, 225, 222, 220, 217, 215, 212, 209, 207, 204,
    202, 199, 197, 194, 192, 190, 187, 185, 182, 180, 178, 175, 173, 171, 168, 166, 164, 162, 159,
    157, 155, 153, 151, 149, 146, 144, 142, 140, 138, 136, 134, 132, 130, 128, 126, 123, 121, 119,
    117, 115, 114, 112, 110, 108, 106, 104, 102, 100, 98, 96, 94, 93, 91, 89, 87, 85, 83, 82, 80,
    78, 76, 74, 73, 71, 69, 67, 66, 64, 62, 61, 59, 57, 55, 54, 52, 50, 49, 47, 46, 44, 42, 41, 39,
    37, 36, 34, 33, 31, 30, 28, 26, 25, 23, 22, 20, 19, 17, 16, 14, 13, 11, 10, 8, 7, 5, 4, 2, 1,
];

fn ZSTD_getFSEMaxSymbolValue(ctable: &[FSE_CTable]) -> u16 {
    let [_, _, a, b] = ctable[0].to_ne_bytes();
    u16::from_ne_bytes([a, b])
}

/// Returns true if we should use ncount=-1 else we should
/// use ncount=1 for low probability symbols instead.
fn ZSTD_useLowProbCount(nbSeq: size_t) -> bool {
    // Heuristic: This should cover most blocks <= 16K and
    // start to fade out after 16K to about 32K depending on
    // compressibility.
    nbSeq >= 2048
}

/// Returns the cost in bytes of encoding the normalized count header.
/// Returns an error if any of the helper functions return an error.
unsafe fn ZSTD_NCountCost(
    count: *const core::ffi::c_uint,
    max: u8,
    nbSeq: size_t,
    FSELog: core::ffi::c_uint,
) -> size_t {
    let mut wksp: [u8; 512] = [0; 512];
    let mut norm: [i16; 53] = [0; 53];
    let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
    let err_code = FSE_normalizeCount(
        &mut norm,
        tableLog,
        count,
        nbSeq,
        max,
        ZSTD_useLowProbCount(nbSeq),
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    FSE_writeNCount(
        wksp.as_mut_ptr() as *mut core::ffi::c_void,
        size_of::<[u8; 512]>(),
        &norm,
        max,
        tableLog,
    )
}

/// Returns the cost in bits of encoding the distribution described by count
/// using the entropy bound.
unsafe fn ZSTD_entropyCost(count: *const core::ffi::c_uint, max: u8, total: size_t) -> size_t {
    let mut cost = 0u32;
    for s in 0..=max {
        let mut norm =
            (256u32.wrapping_mul(*count.offset(s as isize)) as size_t / total) as core::ffi::c_uint;
        if *count.offset(s as isize) != 0 && norm == 0 {
            norm = 1;
        }
        cost = cost.wrapping_add(
            (*count.offset(s as isize)).wrapping_mul(kInverseProbabilityLog256[norm as usize]),
        );
    }
    (cost >> 8) as size_t
}

/// Returns the cost in bits of encoding the distribution in count using ctable.
/// Returns an error if ctable cannot represent all the symbols in count.
pub unsafe fn ZSTD_fseBitCost(
    ctable: &[FSE_CTable],
    count: *const core::ffi::c_uint,
    max: u8,
) -> size_t {
    let kAccuracyLog = 8;
    let mut cost = 0usize;
    let mut cstate = FSE_CState_t::default();
    FSE_initCState(&mut cstate, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < u16::from(max) {
        return Error::GENERIC.to_error_code();
    }
    for s in 0..u32::from(max) + 1 {
        let tableLog = cstate.stateLog;
        let badCost = tableLog.wrapping_add(1) << kAccuracyLog;
        let bitCost = FSE_bitCost(cstate.symbolTT, tableLog, s, kAccuracyLog);
        if *count.offset(s as isize) != 0 {
            if bitCost >= badCost {
                return Error::GENERIC.to_error_code();
            }
            cost = cost.wrapping_add(*count.offset(s as isize) as size_t * bitCost as size_t);
        }
    }
    cost >> kAccuracyLog
}

/// Returns the cost in bits of encoding the distribution in count using the
/// table described by norm. The max symbol support by norm is assumed >= max.
/// norm must be valid for every symbol with non-zero probability in count.
pub unsafe fn ZSTD_crossEntropyCost(
    norm: &[core::ffi::c_short],
    accuracyLog: core::ffi::c_uint,
    count: *const core::ffi::c_uint,
    max: u8,
) -> size_t {
    let shift = (8 as core::ffi::c_uint).wrapping_sub(accuracyLog);
    let mut cost = 0usize;
    for s in 0..u32::from(max) + 1 {
        let normAcc = if norm[s as usize] as core::ffi::c_int != -1 {
            norm[s as usize] as core::ffi::c_uint
        } else {
            1
        };
        let norm256 = normAcc << shift;
        cost = cost.wrapping_add(
            (*count.offset(s as isize)).wrapping_mul(kInverseProbabilityLog256[norm256 as usize])
                as size_t,
        );
    }
    cost >> 8
}

pub unsafe fn ZSTD_selectEncodingType(
    repeatMode: &mut FSE_repeat,
    count: *const core::ffi::c_uint,
    max: u8,
    mostFrequent: size_t,
    nbSeq: size_t,
    FSELog: core::ffi::c_uint,
    prevCTable: &[FSE_CTable],
    defaultNorm: &[core::ffi::c_short],
    defaultNormLog: u32,
    isDefaultAllowed: DefaultPolicy,
    strategy: ZSTD_strategy,
) -> SymbolEncodingType {
    if mostFrequent == nbSeq {
        *repeatMode = FSE_repeat_none;
        if isDefaultAllowed == DefaultPolicy::Allowed && nbSeq <= 2 {
            // Prefer SymbolEncodingType::Basic over SymbolEncodingType::Rle when there are 2 or fewer symbols,
            // since RLE uses 1 byte, but SymbolEncodingType::Basic uses 5-6 bits per symbol.
            // If basic encoding isn't possible, always choose RLE.
            return SymbolEncodingType::Basic;
        }
        return SymbolEncodingType::Rle;
    }
    if (strategy as core::ffi::c_uint) < ZSTD_lazy {
        if isDefaultAllowed == DefaultPolicy::Allowed {
            let staticFse_nbSeq_max = 1000;
            let mult =
                (10 as core::ffi::c_uint).wrapping_sub(strategy as core::ffi::c_uint) as size_t;
            let baseLog = 3;
            let dynamicFse_nbSeq_min = ((1 << defaultNormLog) * mult) >> baseLog;
            if *repeatMode == FSE_repeat_valid && nbSeq < staticFse_nbSeq_max {
                return SymbolEncodingType::Repeat;
            }
            if nbSeq < dynamicFse_nbSeq_min
                || mostFrequent < nbSeq >> defaultNormLog.wrapping_sub(1)
            {
                // The format allows default tables to be repeated, but it isn't useful.
                // When using simple heuristics to select encoding type, we don't want
                // to confuse these tables with dictionaries. When running more careful
                // analysis, we don't need to waste time checking both repeating tables
                // and default tables.
                *repeatMode = FSE_repeat_none;
                return SymbolEncodingType::Basic;
            }
        }
    } else {
        let basicCost = if isDefaultAllowed == DefaultPolicy::Allowed {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, count, max)
        } else {
            Error::GENERIC.to_error_code()
        };
        let repeatCost = if *repeatMode != FSE_repeat_none {
            ZSTD_fseBitCost(prevCTable, count, max)
        } else {
            Error::GENERIC.to_error_code()
        };
        let NCountCost = ZSTD_NCountCost(count, max, nbSeq, FSELog);
        let compressedCost = (NCountCost << 3).wrapping_add(ZSTD_entropyCost(count, max, nbSeq));

        if isDefaultAllowed == DefaultPolicy::Allowed {
            assert_eq!(ZSTD_isError(basicCost), 0);
            assert!(!(*repeatMode == FSE_repeat_valid && ZSTD_isError(repeatCost) != 0));
        }
        assert_eq!(ZSTD_isError(NCountCost), 0);
        if basicCost <= repeatCost && basicCost <= compressedCost {
            *repeatMode = FSE_repeat_none;
            return SymbolEncodingType::Basic;
        }
        if repeatCost <= compressedCost {
            return SymbolEncodingType::Repeat;
        }
    }
    *repeatMode = FSE_repeat_check;
    SymbolEncodingType::Compressed
}

pub(crate) const LL_DEFAULT_CTABLE_SIZE: usize =
    FSE_CTABLE_SIZE_U32(LL_defaultNormLog as usize, MaxLL as usize);
pub(crate) const OF_DEFAULT_CTABLE_SIZE: usize =
    FSE_CTABLE_SIZE_U32(OF_defaultNormLog as usize, DefaultMaxOff as usize);
pub(crate) const ML_DEFAULT_CTABLE_SIZE: usize =
    FSE_CTABLE_SIZE_U32(ML_defaultNormLog as usize, MaxML as usize);

pub(crate) static LL_defaultCTable: [FSE_CTable; LL_DEFAULT_CTABLE_SIZE] = [
    FSE_writeU16Pair(6, 35),
    // State table (64 u16 entries)
    FSE_writeU16Pair(64, 65),
    FSE_writeU16Pair(86, 107),
    FSE_writeU16Pair(66, 87),
    FSE_writeU16Pair(108, 88),
    FSE_writeU16Pair(109, 67),
    FSE_writeU16Pair(110, 68),
    FSE_writeU16Pair(89, 90),
    FSE_writeU16Pair(111, 69),
    FSE_writeU16Pair(112, 70),
    FSE_writeU16Pair(91, 92),
    FSE_writeU16Pair(113, 71),
    FSE_writeU16Pair(114, 72),
    FSE_writeU16Pair(93, 94),
    FSE_writeU16Pair(115, 73),
    FSE_writeU16Pair(116, 95),
    FSE_writeU16Pair(74, 117),
    FSE_writeU16Pair(75, 96),
    FSE_writeU16Pair(97, 118),
    FSE_writeU16Pair(76, 119),
    FSE_writeU16Pair(77, 98),
    FSE_writeU16Pair(99, 120),
    FSE_writeU16Pair(78, 121),
    FSE_writeU16Pair(79, 100),
    FSE_writeU16Pair(101, 122),
    FSE_writeU16Pair(80, 123),
    FSE_writeU16Pair(81, 102),
    FSE_writeU16Pair(103, 82),
    FSE_writeU16Pair(104, 83),
    FSE_writeU16Pair(105, 84),
    FSE_writeU16Pair(106, 85),
    FSE_writeU16Pair(127, 126),
    FSE_writeU16Pair(125, 124),
    // Symbol transformation table (36 symbols)
    (-4i32) as u32, 327552, // symbol 0
    1 as u32, 327584, // symbol 1
    5 as u32, 393088, // symbol 2
    7 as u32, 393088, // symbol 3
    9 as u32, 393088, // symbol 4
    11 as u32, 393088, // symbol 5
    13 as u32, 393088, // symbol 6
    15 as u32, 393088, // symbol 7
    17 as u32, 393088, // symbol 8
    19 as u32, 393088, // symbol 9
    21 as u32, 393088, // symbol 10
    23 as u32, 393088, // symbol 11
    25 as u32, 393088, // symbol 12
    28 as u32, 393152, // symbol 13
    29 as u32, 393152, // symbol 14
    30 as u32, 393152, // symbol 15
    30 as u32, 393088, // symbol 16
    32 as u32, 393088, // symbol 17
    34 as u32, 393088, // symbol 18
    36 as u32, 393088, // symbol 19
    38 as u32, 393088, // symbol 20
    40 as u32, 393088, // symbol 21
    42 as u32, 393088, // symbol 22
    44 as u32, 393088, // symbol 23
    46 as u32, 393088, // symbol 24
    47 as u32, 327584, // symbol 25
    51 as u32, 393088, // symbol 26
    54 as u32, 393152, // symbol 27
    55 as u32, 393152, // symbol 28
    56 as u32, 393152, // symbol 29
    57 as u32, 393152, // symbol 30
    58 as u32, 393152, // symbol 31
    59 as u32, 393152, // symbol 32
    60 as u32, 393152, // symbol 33
    61 as u32, 393152, // symbol 34
    62 as u32, 393152, // symbol 35
];

pub(crate) static OF_defaultCTable: [FSE_CTable; OF_DEFAULT_CTABLE_SIZE] = [
    FSE_writeU16Pair(5, 28),
    // State table (32 u16 entries)
    FSE_writeU16Pair(32, 55),
    FSE_writeU16Pair(46, 37),
    FSE_writeU16Pair(51, 42),
    FSE_writeU16Pair(33, 56),
    FSE_writeU16Pair(38, 47),
    FSE_writeU16Pair(43, 52),
    FSE_writeU16Pair(34, 57),
    FSE_writeU16Pair(48, 39),
    FSE_writeU16Pair(53, 44),
    FSE_writeU16Pair(35, 58),
    FSE_writeU16Pair(49, 40),
    FSE_writeU16Pair(54, 45),
    FSE_writeU16Pair(36, 50),
    FSE_writeU16Pair(41, 63),
    FSE_writeU16Pair(62, 61),
    FSE_writeU16Pair(60, 59),
    // Symbol transformation table (29 symbols)
    (-1i32) as u32, 327648, // symbol 0
    0 as u32, 327648, // symbol 1
    1 as u32, 327648, // symbol 2
    2 as u32, 327648, // symbol 3
    3 as u32, 327648, // symbol 4
    4 as u32, 327648, // symbol 5
    4 as u32, 327616, // symbol 6
    6 as u32, 327616, // symbol 7
    8 as u32, 327616, // symbol 8
    11 as u32, 327648, // symbol 9
    12 as u32, 327648, // symbol 10
    13 as u32, 327648, // symbol 11
    14 as u32, 327648, // symbol 12
    15 as u32, 327648, // symbol 13
    16 as u32, 327648, // symbol 14
    17 as u32, 327648, // symbol 15
    18 as u32, 327648, // symbol 16
    19 as u32, 327648, // symbol 17
    20 as u32, 327648, // symbol 18
    21 as u32, 327648, // symbol 19
    22 as u32, 327648, // symbol 20
    23 as u32, 327648, // symbol 21
    24 as u32, 327648, // symbol 22
    25 as u32, 327648, // symbol 23
    26 as u32, 327648, // symbol 24
    27 as u32, 327648, // symbol 25
    28 as u32, 327648, // symbol 26
    29 as u32, 327648, // symbol 27
    30 as u32, 327648, // symbol 28
];

pub(crate) static ML_defaultCTable: [FSE_CTable; ML_DEFAULT_CTABLE_SIZE] = [
    FSE_writeU16Pair(6, 52),
    // State table (64 u16 entries)
    FSE_writeU16Pair(64, 65),
    FSE_writeU16Pair(86, 107),
    FSE_writeU16Pair(108, 66),
    FSE_writeU16Pair(87, 109),
    FSE_writeU16Pair(67, 88),
    FSE_writeU16Pair(89, 110),
    FSE_writeU16Pair(68, 111),
    FSE_writeU16Pair(69, 90),
    FSE_writeU16Pair(91, 112),
    FSE_writeU16Pair(70, 113),
    FSE_writeU16Pair(92, 71),
    FSE_writeU16Pair(114, 93),
    FSE_writeU16Pair(72, 115),
    FSE_writeU16Pair(94, 73),
    FSE_writeU16Pair(116, 95),
    FSE_writeU16Pair(74, 117),
    FSE_writeU16Pair(96, 75),
    FSE_writeU16Pair(118, 97),
    FSE_writeU16Pair(76, 119),
    FSE_writeU16Pair(98, 77),
    FSE_writeU16Pair(120, 99),
    FSE_writeU16Pair(78, 100),
    FSE_writeU16Pair(79, 101),
    FSE_writeU16Pair(80, 102),
    FSE_writeU16Pair(81, 103),
    FSE_writeU16Pair(82, 104),
    FSE_writeU16Pair(83, 105),
    FSE_writeU16Pair(84, 106),
    FSE_writeU16Pair(85, 127),
    FSE_writeU16Pair(126, 125),
    FSE_writeU16Pair(124, 123),
    FSE_writeU16Pair(122, 121),
    // Symbol transformation table (53 symbols)
    (-1i32) as u32, 393152, // symbol 0
    (-3i32) as u32, 327552, // symbol 1
    2 as u32, 327584, // symbol 2
    6 as u32, 393088, // symbol 3
    8 as u32, 393088, // symbol 4
    10 as u32, 393088, // symbol 5
    12 as u32, 393088, // symbol 6
    14 as u32, 393088, // symbol 7
    16 as u32, 393088, // symbol 8
    19 as u32, 393152, // symbol 9
    20 as u32, 393152, // symbol 10
    21 as u32, 393152, // symbol 11
    22 as u32, 393152, // symbol 12
    23 as u32, 393152, // symbol 13
    24 as u32, 393152, // symbol 14
    25 as u32, 393152, // symbol 15
    26 as u32, 393152, // symbol 16
    27 as u32, 393152, // symbol 17
    28 as u32, 393152, // symbol 18
    29 as u32, 393152, // symbol 19
    30 as u32, 393152, // symbol 20
    31 as u32, 393152, // symbol 21
    32 as u32, 393152, // symbol 22
    33 as u32, 393152, // symbol 23
    34 as u32, 393152, // symbol 24
    35 as u32, 393152, // symbol 25
    36 as u32, 393152, // symbol 26
    37 as u32, 393152, // symbol 27
    38 as u32, 393152, // symbol 28
    39 as u32, 393152, // symbol 29
    40 as u32, 393152, // symbol 30
    41 as u32, 393152, // symbol 31
    42 as u32, 393152, // symbol 32
    43 as u32, 393152, // symbol 33
    44 as u32, 393152, // symbol 34
    45 as u32, 393152, // symbol 35
    46 as u32, 393152, // symbol 36
    47 as u32, 393152, // symbol 37
    48 as u32, 393152, // symbol 38
    49 as u32, 393152, // symbol 39
    50 as u32, 393152, // symbol 40
    51 as u32, 393152, // symbol 41
    52 as u32, 393152, // symbol 42
    53 as u32, 393152, // symbol 43
    54 as u32, 393152, // symbol 44
    55 as u32, 393152, // symbol 45
    56 as u32, 393152, // symbol 46
    57 as u32, 393152, // symbol 47
    58 as u32, 393152, // symbol 48
    59 as u32, 393152, // symbol 49
    60 as u32, 393152, // symbol 50
    61 as u32, 393152, // symbol 51
    62 as u32, 393152, // symbol 52
];

pub unsafe fn ZSTD_buildCTable(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    nextCTable: &mut [FSE_CTable],
    FSELog: u32,
    type_0: SymbolEncodingType,
    count: *mut core::ffi::c_uint,
    max: u8,
    codeTable: *const u8,
    nbSeq: size_t,
    defaultNorm: &[i16],
    defaultNormLog: u32,
    defaultMax: u8,
    prevCTable: &[FSE_CTable],
    entropyWorkspace: *mut core::ffi::c_void,
    entropyWorkspaceSize: size_t,
) -> size_t {
    let op = dst as *mut u8;
    let oend: *const u8 = op.add(dstCapacity);

    match type_0 {
        SymbolEncodingType::Rle => {
            let err_code = FSE_buildCTable_rle(nextCTable, max);
            if ERR_isError(err_code) {
                return err_code;
            }
            if dstCapacity == 0 {
                return Error::dstSize_tooSmall.to_error_code();
            }
            *op = *codeTable;
            1
        }
        SymbolEncodingType::Repeat => {
            nextCTable[..prevCTable.len()].copy_from_slice(prevCTable);
            0
        }
        SymbolEncodingType::Basic => {
            // A pointer-only match is only a safe stand-in for full slice equality
            // when the candidate slice has the same length as the canonical default
            // (otherwise a subslice sharing the same start pointer could falsely
            // match). Check length alongside the pointer as a cheap fast path, and
            // fall back to full value equality otherwise.
            let defaultCTable: Option<&[FSE_CTable]> = if defaultMax == MaxLL
                && defaultNormLog == LL_defaultNormLog
                && ((core::ptr::eq(defaultNorm.as_ptr(), LL_defaultNorm.as_ptr())
                    && defaultNorm.len() == LL_defaultNorm.len())
                    || defaultNorm == LL_defaultNorm)
            {
                Some(&LL_defaultCTable)
            } else if defaultMax == DefaultMaxOff
                && defaultNormLog == OF_defaultNormLog
                && ((core::ptr::eq(defaultNorm.as_ptr(), OF_defaultNorm.as_ptr())
                    && defaultNorm.len() == OF_defaultNorm.len())
                    || defaultNorm == OF_defaultNorm)
            {
                Some(&OF_defaultCTable)
            } else if defaultMax == MaxML
                && defaultNormLog == ML_defaultNormLog
                && ((core::ptr::eq(defaultNorm.as_ptr(), ML_defaultNorm.as_ptr())
                    && defaultNorm.len() == ML_defaultNorm.len())
                    || defaultNorm == ML_defaultNorm)
            {
                Some(&ML_defaultCTable)
            } else {
                None
            };

            if let Some(canonical) = defaultCTable {
                // Preserve the dynamic builder's workspace-size validation (see
                // `FSE_buildCTable_wksp`) even though the canonical copy path does
                // not itself touch `entropyWorkspace`, so callers relying on that
                // error behavior for an undersized workspace see identical results.
                if entropyWorkspaceSize
                    < FSE_BUILD_CTABLE_WORKSPACE_SIZE(defaultMax as usize, defaultNormLog as usize)
                {
                    return Error::tableLog_tooLarge.to_error_code();
                }
                if nextCTable.len() < canonical.len() {
                    return Error::tableLog_tooLarge.to_error_code();
                }
                nextCTable[..canonical.len()].copy_from_slice(canonical);
                0
            } else {
                let err_code_0 = FSE_buildCTable_wksp(
                    nextCTable,
                    defaultNorm,
                    defaultMax,
                    defaultNormLog,
                    entropyWorkspace,
                    entropyWorkspaceSize,
                );
                if ERR_isError(err_code_0) {
                    return err_code_0;
                }
                0
            }
        }
        SymbolEncodingType::Compressed => {
            let wksp = entropyWorkspace as *mut ZSTD_BuildCTableWksp;
            let mut nbSeq_1 = nbSeq;
            let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
            if *count.offset(*codeTable.add(nbSeq.wrapping_sub(1)) as isize) > 1 {
                let fresh0 = &mut (*count.offset(*codeTable.add(nbSeq.wrapping_sub(1)) as isize));
                *fresh0 = (*fresh0).wrapping_sub(1);
                nbSeq_1 = nbSeq_1.wrapping_sub(1);
            }
            let err_code_1 = FSE_normalizeCount(
                &mut (*wksp).norm,
                tableLog,
                count,
                nbSeq_1,
                max,
                ZSTD_useLowProbCount(nbSeq_1),
            );
            if ERR_isError(err_code_1) {
                return err_code_1;
            }
            let NCountSize = FSE_writeNCount(
                op as *mut core::ffi::c_void,
                oend.offset_from_unsigned(op),
                &(*wksp).norm,
                max,
                tableLog,
            );
            let err_code_2 = NCountSize;
            if ERR_isError(err_code_2) {
                return err_code_2;
            }
            let err_code_3 = FSE_buildCTable_wksp(
                nextCTable,
                &(*wksp).norm,
                max,
                tableLog,
                ((*wksp).wksp).as_mut_ptr() as *mut core::ffi::c_void,
                size_of::<[u32; 285]>(),
            );
            if ERR_isError(err_code_3) {
                return err_code_3;
            }
            NCountSize
        }
    }
}

unsafe fn ZSTD_encodeSequences_body(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    CTable_MatchLength: &[FSE_CTable; 363],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable; 193],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable; 329],
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: bool,
) -> size_t {
    let Ok(mut blockStream) = BIT_initCStream(dst, dstCapacity) else {
        return Error::dstSize_tooSmall.to_error_code();
    };

    let mut stateMatchLength = FSE_CState_t::default();
    let mut stateOffsetBits = FSE_CState_t::default();
    let mut stateLitLength = FSE_CState_t::default();

    // first symbols
    FSE_initCState2(
        &mut stateMatchLength,
        CTable_MatchLength,
        *mlCodeTable.add(nbSeq.wrapping_sub(1)) as u32,
    );
    FSE_initCState2(
        &mut stateOffsetBits,
        CTable_OffsetBits,
        *ofCodeTable.add(nbSeq.wrapping_sub(1)) as u32,
    );
    FSE_initCState2(
        &mut stateLitLength,
        CTable_LitLength,
        *llCodeTable.add(nbSeq.wrapping_sub(1)) as u32,
    );
    BIT_addBits(
        &mut blockStream,
        (*sequences.add(nbSeq.wrapping_sub(1))).litLength as BitContainerType,
        LL_bits[*llCodeTable.add(nbSeq.wrapping_sub(1)) as usize] as core::ffi::c_uint,
    );
    if MEM_32bits() {
        BIT_flushBits(&mut blockStream);
    }
    BIT_addBits(
        &mut blockStream,
        (*sequences.add(nbSeq.wrapping_sub(1))).mlBase as BitContainerType,
        ML_bits[*mlCodeTable.add(nbSeq.wrapping_sub(1)) as usize] as core::ffi::c_uint,
    );
    if MEM_32bits() {
        BIT_flushBits(&mut blockStream);
    }
    if longOffsets {
        let ofBits = *ofCodeTable.add(nbSeq.wrapping_sub(1)) as u32;
        let extraBits = ofBits.wrapping_sub(ofBits.min(STREAM_ACCUMULATOR_MIN - 1));
        if extraBits != 0 {
            BIT_addBits(
                &mut blockStream,
                (*sequences.add(nbSeq.wrapping_sub(1))).offBase as BitContainerType,
                extraBits,
            );
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            ((*sequences.add(nbSeq.wrapping_sub(1))).offBase >> extraBits) as BitContainerType,
            ofBits.wrapping_sub(extraBits),
        );
    } else {
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(nbSeq.wrapping_sub(1))).offBase as BitContainerType,
            *ofCodeTable.add(nbSeq.wrapping_sub(1)) as core::ffi::c_uint,
        );
    }
    BIT_flushBits(&mut blockStream);

    let mut n: size_t = 0;
    n = nbSeq.wrapping_sub(2);
    while n < nbSeq {
        let llCode = *llCodeTable.add(n);
        let ofCode = *ofCodeTable.add(n);
        let mlCode = *mlCodeTable.add(n);
        let llBits = LL_bits[llCode as usize] as u32;
        let ofBits_0 = ofCode as u32;
        let mlBits = ML_bits[mlCode as usize] as u32;
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateOffsetBits,
            ofCode as core::ffi::c_uint,
        );
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateMatchLength,
            mlCode as core::ffi::c_uint,
        );
        if MEM_32bits() {
            BIT_flushBits(&mut blockStream);
        }
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateLitLength,
            llCode as core::ffi::c_uint,
        );
        if MEM_32bits()
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits)
                >= 64 - 7 - (LLFSELog + MLFSELog + OffFSELog)
        {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(n)).litLength as BitContainerType,
            llBits,
        );
        if MEM_32bits() && llBits.wrapping_add(mlBits) > 24 {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(n)).mlBase as BitContainerType,
            mlBits,
        );
        if MEM_32bits() || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) > 56 {
            BIT_flushBits(&mut blockStream);
        }
        if longOffsets {
            let extraBits_0 = ofBits_0.wrapping_sub(ofBits_0.min(STREAM_ACCUMULATOR_MIN - 1));
            if extraBits_0 != 0 {
                BIT_addBits(
                    &mut blockStream,
                    (*sequences.add(n)).offBase as BitContainerType,
                    extraBits_0,
                );
                BIT_flushBits(&mut blockStream);
            }
            BIT_addBits(
                &mut blockStream,
                ((*sequences.add(n)).offBase >> extraBits_0) as BitContainerType,
                ofBits_0.wrapping_sub(extraBits_0),
            );
        } else {
            BIT_addBits(
                &mut blockStream,
                (*sequences.add(n)).offBase as BitContainerType,
                ofBits_0,
            );
        }
        BIT_flushBits(&mut blockStream);
        n = n.wrapping_sub(1);
    }

    FSE_flushCState(&mut blockStream, &stateMatchLength);
    FSE_flushCState(&mut blockStream, &stateOffsetBits);
    FSE_flushCState(&mut blockStream, &stateLitLength);

    let streamSize = BIT_closeCStream(&mut blockStream);
    if streamSize == 0 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    streamSize
}

unsafe fn ZSTD_encodeSequences_default(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    CTable_MatchLength: &[FSE_CTable; 363],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable; 193],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable; 329],
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: bool,
) -> size_t {
    ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    )
}

unsafe fn ZSTD_encodeSequences_bmi2(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    CTable_MatchLength: &[FSE_CTable; 363],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable; 193],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable; 329],
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: bool,
) -> size_t {
    ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    )
}

pub unsafe fn ZSTD_encodeSequences(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    CTable_MatchLength: &[FSE_CTable; 363],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable; 193],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable; 329],
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: bool,
    bmi2: core::ffi::c_int,
) -> size_t {
    if bmi2 != 0 {
        return ZSTD_encodeSequences_bmi2(
            dst,
            dstCapacity,
            CTable_MatchLength,
            mlCodeTable,
            CTable_OffsetBits,
            ofCodeTable,
            CTable_LitLength,
            llCodeTable,
            sequences,
            nbSeq,
            longOffsets,
        );
    }
    ZSTD_encodeSequences_default(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib::common::zstd_internal::{
        DefaultMaxOff, LL_defaultNorm, LL_defaultNormLog, ML_defaultNorm, ML_defaultNormLog,
        MaxLL, MaxML, OF_defaultNorm, OF_defaultNormLog,
    };

    // `FSE_buildCTable_wksp` casts the workspace pointer to `*mut u16`/`*mut u64`
    // and dereferences it directly, so the backing storage must be properly
    // aligned. A `[u8; N]` array only guarantees alignment 1; use a `u32`-typed
    // array (aligned to 4, sufficient for the u16/u32 accesses performed) sized
    // in bytes via `size_of_val` instead of a raw byte-array length.
    fn aligned_workspace() -> ([u32; 512], usize) {
        let wksp = [0u32; 512];
        let size = core::mem::size_of_val(&wksp);
        (wksp, size)
    }

    #[test]
    fn test_canonical_ctables_identical_to_dynamic_build() {
        let (mut wksp, wksp_size) = aligned_workspace();

        let mut ll_table = [0u32; 329];
        unsafe {
            let res = FSE_buildCTable_wksp(
                &mut ll_table,
                &LL_defaultNorm,
                MaxLL,
                LL_defaultNormLog,
                wksp.as_mut_ptr().cast(),
                wksp_size,
            );
            assert_eq!(res, 0);
        }
        assert_eq!(&ll_table[..LL_DEFAULT_CTABLE_SIZE], &LL_defaultCTable[..]);

        let mut of_table = [0u32; 193];
        unsafe {
            let res = FSE_buildCTable_wksp(
                &mut of_table,
                &OF_defaultNorm,
                DefaultMaxOff,
                OF_defaultNormLog,
                wksp.as_mut_ptr().cast(),
                wksp_size,
            );
            assert_eq!(res, 0);
        }
        assert_eq!(&of_table[..OF_DEFAULT_CTABLE_SIZE], &OF_defaultCTable[..]);

        let mut ml_table = [0u32; 363];
        unsafe {
            let res = FSE_buildCTable_wksp(
                &mut ml_table,
                &ML_defaultNorm,
                MaxML,
                ML_defaultNormLog,
                wksp.as_mut_ptr().cast(),
                wksp_size,
            );
            assert_eq!(res, 0);
        }
        assert_eq!(&ml_table[..ML_DEFAULT_CTABLE_SIZE], &ML_defaultCTable[..]);
    }

    #[test]
    fn test_canonical_table_headers_and_sizes() {
        assert_eq!(LL_defaultCTable.len(), LL_DEFAULT_CTABLE_SIZE);
        assert_eq!(OF_defaultCTable.len(), OF_DEFAULT_CTABLE_SIZE);
        assert_eq!(ML_defaultCTable.len(), ML_DEFAULT_CTABLE_SIZE);

        assert_eq!(
            LL_defaultCTable[0],
            FSE_writeU16Pair(LL_defaultNormLog as u16, MaxLL as u16)
        );
        assert_eq!(
            OF_defaultCTable[0],
            FSE_writeU16Pair(OF_defaultNormLog as u16, DefaultMaxOff as u16)
        );
        assert_eq!(
            ML_defaultCTable[0],
            FSE_writeU16Pair(ML_defaultNormLog as u16, MaxML as u16)
        );
    }

    #[test]
    fn test_zstd_build_ctable_basic_canonical_copy() {
        let mut dst = [0u8; 128];
        let (mut wksp, wksp_size) = aligned_workspace();

        // 1. LL table
        let mut ll_ctable = [0u32; 329];
        let ret_ll = unsafe {
            ZSTD_buildCTable(
                dst.as_mut_ptr().cast(),
                dst.len(),
                &mut ll_ctable,
                LL_defaultNormLog,
                SymbolEncodingType::Basic,
                core::ptr::null_mut(),
                MaxLL,
                core::ptr::null(),
                0,
                &LL_defaultNorm,
                LL_defaultNormLog,
                MaxLL,
                &[],
                wksp.as_mut_ptr().cast(),
                wksp_size,
            )
        };
        assert_eq!(ret_ll, 0);
        assert_eq!(&ll_ctable[..LL_DEFAULT_CTABLE_SIZE], &LL_defaultCTable[..]);

        // 2. OF table
        let mut of_ctable = [0u32; 193];
        let ret_of = unsafe {
            ZSTD_buildCTable(
                dst.as_mut_ptr().cast(),
                dst.len(),
                &mut of_ctable,
                OF_defaultNormLog,
                SymbolEncodingType::Basic,
                core::ptr::null_mut(),
                DefaultMaxOff,
                core::ptr::null(),
                0,
                &OF_defaultNorm,
                OF_defaultNormLog,
                DefaultMaxOff,
                &[],
                wksp.as_mut_ptr().cast(),
                wksp_size,
            )
        };
        assert_eq!(ret_of, 0);
        assert_eq!(&of_ctable[..OF_DEFAULT_CTABLE_SIZE], &OF_defaultCTable[..]);

        // 3. ML table
        let mut ml_ctable = [0u32; 363];
        let ret_ml = unsafe {
            ZSTD_buildCTable(
                dst.as_mut_ptr().cast(),
                dst.len(),
                &mut ml_ctable,
                ML_defaultNormLog,
                SymbolEncodingType::Basic,
                core::ptr::null_mut(),
                MaxML,
                core::ptr::null(),
                0,
                &ML_defaultNorm,
                ML_defaultNormLog,
                MaxML,
                &[],
                wksp.as_mut_ptr().cast(),
                wksp_size,
            )
        };
        assert_eq!(ret_ml, 0);
        assert_eq!(&ml_ctable[..ML_DEFAULT_CTABLE_SIZE], &ML_defaultCTable[..]);
    }

    #[test]
    fn test_zstd_build_ctable_basic_canonical_path_skips_workspace() {
        // The canonical copy fast path must not touch the scratch workspace at
        // all, whereas the dynamic `FSE_buildCTable_wksp` always writes into it
        // (via its `cumul`/`tableSymbol` scratch views). Filling the workspace
        // with a sentinel pattern and asserting it is untouched after a
        // Basic-mode build with the exact canonical inputs is a durable,
        // instrumentation-free way to prove the fast (canonical-copy) path
        // executed, without reintroducing any production hot-path counters.
        const SENTINEL: u32 = 0xAAAA_AAAA;
        let mut dst = [0u8; 128];

        // (defaultNorm, defaultNormLog, defaultMax, expected canonical table)
        let cases: [(&[i16], u32, u8, &[FSE_CTable]); 3] = [
            (&LL_defaultNorm, LL_defaultNormLog, MaxLL, &LL_defaultCTable),
            (
                &OF_defaultNorm,
                OF_defaultNormLog,
                DefaultMaxOff,
                &OF_defaultCTable,
            ),
            (&ML_defaultNorm, ML_defaultNormLog, MaxML, &ML_defaultCTable),
        ];

        for (defaultNorm, defaultNormLog, defaultMax, expected) in cases {
            let mut wksp = [SENTINEL; 512];
            let wksp_size = core::mem::size_of_val(&wksp);
            let mut nextCTable = [0u32; 363]; // large enough for every table kind

            let ret = unsafe {
                ZSTD_buildCTable(
                    dst.as_mut_ptr().cast(),
                    dst.len(),
                    &mut nextCTable[..expected.len()],
                    defaultNormLog,
                    SymbolEncodingType::Basic,
                    core::ptr::null_mut(),
                    defaultMax,
                    core::ptr::null(),
                    0,
                    defaultNorm,
                    defaultNormLog,
                    defaultMax,
                    &[],
                    wksp.as_mut_ptr().cast(),
                    wksp_size,
                )
            };

            assert_eq!(ret, 0);
            assert_eq!(&nextCTable[..expected.len()], expected);
            assert!(
                wksp.iter().all(|&w| w == SENTINEL),
                "canonical Basic-mode copy must not write to the scratch workspace"
            );
        }
    }

    #[test]
    fn test_zstd_build_ctable_basic_falls_back_for_altered_distribution() {
        // A distribution with the same shape (length/tableLog/maxSymbolValue) as
        // a canonical default, but different values, must NOT be matched by the
        // fast path's pointer-equality check and must fall through to the
        // dynamic builder, producing the same result the dynamic builder would
        // produce directly (and, in particular, NOT the canonical table).
        let mut altered = LL_defaultNorm;
        altered.swap(0, 1); // still sums to the table size, but a different shape

        let (mut wksp, wksp_size) = aligned_workspace();

        let mut expected = [0u32; LL_DEFAULT_CTABLE_SIZE];
        unsafe {
            let res = FSE_buildCTable_wksp(
                &mut expected,
                &altered,
                MaxLL,
                LL_defaultNormLog,
                wksp.as_mut_ptr().cast(),
                wksp_size,
            );
            assert_eq!(res, 0);
        }

        let mut dst = [0u8; 128];
        let mut actual = [0u32; LL_DEFAULT_CTABLE_SIZE];
        let ret = unsafe {
            ZSTD_buildCTable(
                dst.as_mut_ptr().cast(),
                dst.len(),
                &mut actual,
                LL_defaultNormLog,
                SymbolEncodingType::Basic,
                core::ptr::null_mut(),
                MaxLL,
                core::ptr::null(),
                0,
                &altered,
                LL_defaultNormLog,
                MaxLL,
                &[],
                wksp.as_mut_ptr().cast(),
                wksp_size,
            )
        };
        assert_eq!(ret, 0);
        assert_eq!(actual, expected);
        assert_ne!(
            &actual[..],
            &LL_defaultCTable[..],
            "altered distribution must not silently match the canonical LL table"
        );
    }
}

