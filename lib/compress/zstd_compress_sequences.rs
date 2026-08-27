use libc::size_t;

use crate::lib::common::bitstream::{
    BIT_CStream_t, BIT_addBits, BIT_closeCStream, BIT_flushBits, BIT_initCStream, BitContainerType,
};
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{
    FSE_CState_t, FSE_CTable, FSE_bitCost, FSE_encodeSymbol, FSE_flushCState, FSE_initCState,
    FSE_initCState2, FSE_repeat, FSE_repeat_check, FSE_repeat_none, FSE_repeat_valid,
};
use crate::lib::common::mem::MEM_32bits;
use crate::lib::common::zstd_internal::{
    LLFSELog, LL_bits, MLFSELog, ML_bits, OffFSELog, SymbolEncodingType,
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
        norm.as_mut_ptr(),
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
        norm.as_mut_ptr(),
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
    let mut cstate = FSE_CState_t {
        value: 0,
        stateTable: core::ptr::null::<core::ffi::c_void>(),
        symbolTT: core::ptr::null::<core::ffi::c_void>(),
        stateLog: 0,
    };
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
    norm: *const core::ffi::c_short,
    accuracyLog: core::ffi::c_uint,
    count: *const core::ffi::c_uint,
    max: u8,
) -> size_t {
    let shift = (8 as core::ffi::c_uint).wrapping_sub(accuracyLog);
    let mut cost = 0usize;
    for s in 0..u32::from(max) + 1 {
        let normAcc = if *norm.offset(s as isize) as core::ffi::c_int != -1 {
            *norm.offset(s as isize) as core::ffi::c_uint
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
    defaultNorm: *const core::ffi::c_short,
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
            assert!(ZSTD_isError(basicCost) == 0);
            assert!(!(*repeatMode == FSE_repeat_valid && ZSTD_isError(repeatCost) != 0));
        }
        assert!(ZSTD_isError(NCountCost) == 0);
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
    defaultNorm: *const i16,
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
            core::ptr::copy_nonoverlapping(
                prevCTable.as_ptr().cast::<u8>(),
                nextCTable.as_mut_ptr().cast::<u8>(),
                size_of_val(prevCTable),
            );
            0
        }
        SymbolEncodingType::Basic => {
            let err_code_0 = FSE_buildCTable_wksp(
                nextCTable.as_mut_ptr(),
                defaultNorm,
                u32::from(defaultMax),
                defaultNormLog,
                entropyWorkspace,
                entropyWorkspaceSize,
            );
            if ERR_isError(err_code_0) {
                return err_code_0;
            }
            0
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
                ((*wksp).norm).as_mut_ptr(),
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
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
            );
            let err_code_2 = NCountSize;
            if ERR_isError(err_code_2) {
                return err_code_2;
            }
            let err_code_3 = FSE_buildCTable_wksp(
                nextCTable.as_mut_ptr(),
                ((*wksp).norm).as_mut_ptr(),
                u32::from(max),
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
    CTable_MatchLength: &[FSE_CTable],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable],
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: bool,
) -> size_t {
    let mut blockStream = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: core::ptr::null_mut::<core::ffi::c_char>(),
        ptr: core::ptr::null_mut::<core::ffi::c_char>(),
        endPtr: core::ptr::null_mut::<core::ffi::c_char>(),
    };
    let mut stateMatchLength = FSE_CState_t {
        value: 0,
        stateTable: core::ptr::null::<core::ffi::c_void>(),
        symbolTT: core::ptr::null::<core::ffi::c_void>(),
        stateLog: 0,
    };
    let mut stateOffsetBits = FSE_CState_t {
        value: 0,
        stateTable: core::ptr::null::<core::ffi::c_void>(),
        symbolTT: core::ptr::null::<core::ffi::c_void>(),
        stateLog: 0,
    };
    let mut stateLitLength = FSE_CState_t {
        value: 0,
        stateTable: core::ptr::null::<core::ffi::c_void>(),
        symbolTT: core::ptr::null::<core::ffi::c_void>(),
        stateLog: 0,
    };

    if ERR_isError(BIT_initCStream(&mut blockStream, dst, dstCapacity)) {
        return Error::dstSize_tooSmall.to_error_code();
    }

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
        let extraBits = ofBits.wrapping_sub(
            ofBits.min(((if MEM_32bits() { 25 } else { 57 }) as u32).wrapping_sub(1)),
        );
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
            let extraBits_0 = ofBits_0.wrapping_sub(
                ofBits_0.min(((if MEM_32bits() { 25 } else { 57 }) as u32).wrapping_sub(1)),
            );
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
    CTable_MatchLength: &[FSE_CTable],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable],
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
    CTable_MatchLength: &[FSE_CTable],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable],
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
    CTable_MatchLength: &[FSE_CTable],
    mlCodeTable: *const u8,
    CTable_OffsetBits: &[FSE_CTable],
    ofCodeTable: *const u8,
    CTable_LitLength: &[FSE_CTable],
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
