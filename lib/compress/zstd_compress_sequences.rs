use libc::size_t;

use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::mem::{MEM_32bits, MEM_read16, MEM_writeLEST};
use crate::lib::compress::fse_compress::{
    FSE_buildCTable_rle, FSE_buildCTable_wksp, FSE_normalizeCount, FSE_optimalTableLog,
    FSE_writeNCount,
};
use crate::lib::compress::zstd_compress::SeqDef;
use crate::lib::zstd::*;

pub type ptrdiff_t = core::ffi::c_long;
pub type SymbolEncodingType_e = core::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type FSE_repeat = core::ffi::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = core::ffi::c_uint;
pub type BitContainerType = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_CStream_t {
    pub bitContainer: BitContainerType,
    pub bitPos: core::ffi::c_uint,
    pub startPtr: *mut core::ffi::c_char,
    pub ptr: *mut core::ffi::c_char,
    pub endPtr: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const core::ffi::c_void,
    pub symbolTT: *const core::ffi::c_void,
    pub stateLog: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: core::ffi::c_int,
    pub deltaNbBits: u32,
}
pub type ZSTD_DefaultPolicy_e = core::ffi::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_DefaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_DefaultPolicy_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_BuildCTableWksp {
    pub norm: [i16; 53],
    pub wksp: [u32; 285],
}
pub const MLFSELog: core::ffi::c_int = 9;
pub const LLFSELog: core::ffi::c_int = 9;
pub const OffFSELog: core::ffi::c_int = 8;
static LL_bits: [u8; 36] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16,
];
static ML_bits: [u8; 53] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
];
#[inline]
unsafe fn FSE_initCState(mut statePtr: *mut FSE_CState_t, mut ct: *const FSE_CTable) {
    let mut ptr = ct as *const core::ffi::c_void;
    let mut u16ptr = ptr as *const u16;
    let tableLog = MEM_read16(ptr) as u32;
    (*statePtr).value = (1) << tableLog;
    (*statePtr).stateTable = u16ptr.offset(2) as *const core::ffi::c_void;
    (*statePtr).symbolTT = ct.offset(1).offset(
        (if tableLog != 0 {
            (1) << tableLog.wrapping_sub(1)
        } else {
            1
        }) as isize,
    ) as *const core::ffi::c_void;
    (*statePtr).stateLog = tableLog;
}
#[inline]
unsafe fn FSE_initCState2(
    mut statePtr: *mut FSE_CState_t,
    mut ct: *const FSE_CTable,
    mut symbol: u32,
) {
    FSE_initCState(statePtr, ct);
    let symbolTT =
        *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let mut stateTable = (*statePtr).stateTable as *const u16;
    let mut nbBitsOut = (symbolTT.deltaNbBits).wrapping_add(((1) << 15) as u32) >> 16;
    (*statePtr).value = (nbBitsOut << 16).wrapping_sub(symbolTT.deltaNbBits) as ptrdiff_t;
    (*statePtr).value = *stateTable
        .offset((((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t) as isize)
        as ptrdiff_t;
}
#[inline]
unsafe fn FSE_encodeSymbol(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *mut FSE_CState_t,
    mut symbol: core::ffi::c_uint,
) {
    let symbolTT =
        *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable = (*statePtr).stateTable as *const u16;
    let nbBitsOut = (((*statePtr).value + symbolTT.deltaNbBits as ptrdiff_t) >> 16) as u32;
    BIT_addBits(bitC, (*statePtr).value as BitContainerType, nbBitsOut);
    (*statePtr).value = *stateTable
        .offset((((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t) as isize)
        as ptrdiff_t;
}
#[inline]
unsafe fn FSE_flushCState(mut bitC: *mut BIT_CStream_t, mut statePtr: *const FSE_CState_t) {
    BIT_addBits(
        bitC,
        (*statePtr).value as BitContainerType,
        (*statePtr).stateLog,
    );
    BIT_flushBits(bitC);
}
#[inline]
unsafe fn FSE_bitCost(
    mut symbolTTPtr: *const core::ffi::c_void,
    mut tableLog: u32,
    mut symbolValue: u32,
    mut accuracyLog: u32,
) -> u32 {
    let mut symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    let minNbBits = (*symbolTT.offset(symbolValue as isize)).deltaNbBits >> 16;
    let threshold = minNbBits.wrapping_add(1) << 16;
    let tableSize = ((1) << tableLog) as u32;
    let deltaFromThreshold = threshold.wrapping_sub(
        ((*symbolTT.offset(symbolValue as isize)).deltaNbBits).wrapping_add(tableSize),
    );
    let normalizedDeltaFromThreshold = deltaFromThreshold << accuracyLog >> tableLog;
    let bitMultiplier = ((1) << accuracyLog) as u32;
    (minNbBits.wrapping_add(1) * bitMultiplier).wrapping_sub(normalizedDeltaFromThreshold)
}
static BIT_mask: [core::ffi::c_uint; 32] = [
    0,
    1,
    3,
    7,
    0xf as core::ffi::c_int as core::ffi::c_uint,
    0x1f as core::ffi::c_int as core::ffi::c_uint,
    0x3f as core::ffi::c_int as core::ffi::c_uint,
    0x7f as core::ffi::c_int as core::ffi::c_uint,
    0xff as core::ffi::c_int as core::ffi::c_uint,
    0x1ff as core::ffi::c_int as core::ffi::c_uint,
    0x3ff as core::ffi::c_int as core::ffi::c_uint,
    0x7ff as core::ffi::c_int as core::ffi::c_uint,
    0xfff as core::ffi::c_int as core::ffi::c_uint,
    0x1fff as core::ffi::c_int as core::ffi::c_uint,
    0x3fff as core::ffi::c_int as core::ffi::c_uint,
    0x7fff as core::ffi::c_int as core::ffi::c_uint,
    0xffff as core::ffi::c_int as core::ffi::c_uint,
    0x1ffff as core::ffi::c_int as core::ffi::c_uint,
    0x3ffff as core::ffi::c_int as core::ffi::c_uint,
    0x7ffff as core::ffi::c_int as core::ffi::c_uint,
    0xfffff as core::ffi::c_int as core::ffi::c_uint,
    0x1fffff as core::ffi::c_int as core::ffi::c_uint,
    0x3fffff as core::ffi::c_int as core::ffi::c_uint,
    0x7fffff as core::ffi::c_int as core::ffi::c_uint,
    0xffffff as core::ffi::c_int as core::ffi::c_uint,
    0x1ffffff as core::ffi::c_int as core::ffi::c_uint,
    0x3ffffff as core::ffi::c_int as core::ffi::c_uint,
    0x7ffffff as core::ffi::c_int as core::ffi::c_uint,
    0xfffffff as core::ffi::c_int as core::ffi::c_uint,
    0x1fffffff as core::ffi::c_int as core::ffi::c_uint,
    0x3fffffff as core::ffi::c_int as core::ffi::c_uint,
    0x7fffffff as core::ffi::c_int as core::ffi::c_uint,
];
#[inline]
unsafe fn BIT_initCStream(
    mut bitC: *mut BIT_CStream_t,
    mut startPtr: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    (*bitC).bitContainer = 0;
    (*bitC).bitPos = 0;
    (*bitC).startPtr = startPtr as *mut core::ffi::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr)
        .add(dstCapacity)
        .offset(-(::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<BitContainerType>() as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    0
}
#[inline(always)]
unsafe fn BIT_getLowerBits(mut bitContainer: BitContainerType, nbBits: u32) -> BitContainerType {
    bitContainer & *BIT_mask.as_ptr().offset(nbBits as isize) as BitContainerType
}
#[inline]
unsafe fn BIT_addBits(
    mut bitC: *mut BIT_CStream_t,
    mut value: BitContainerType,
    mut nbBits: core::ffi::c_uint,
) {
    (*bitC).bitContainer |= BIT_getLowerBits(value, nbBits) << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}
#[inline]
unsafe fn BIT_addBitsFast(
    mut bitC: *mut BIT_CStream_t,
    mut value: BitContainerType,
    mut nbBits: core::ffi::c_uint,
) {
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}
#[inline]
unsafe fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut core::ffi::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
    if (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
    (*bitC).bitPos &= 7;
    (*bitC).bitContainer >>= nbBytes * 8;
}
#[inline]
unsafe fn BIT_closeCStream(mut bitC: *mut BIT_CStream_t) -> size_t {
    BIT_addBitsFast(bitC, 1, 1);
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0;
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as core::ffi::c_long as size_t)
        .wrapping_add(((*bitC).bitPos > 0) as core::ffi::c_int as size_t)
}
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
unsafe fn ZSTD_getFSEMaxSymbolValue(mut ctable: *const FSE_CTable) -> core::ffi::c_uint {
    let mut ptr = ctable as *const core::ffi::c_void;
    let mut u16ptr = ptr as *const u16;

    MEM_read16(u16ptr.offset(1) as *const core::ffi::c_void) as u32
}
unsafe fn ZSTD_useLowProbCount(nbSeq: size_t) -> core::ffi::c_uint {
    (nbSeq >= 2048) as core::ffi::c_int as core::ffi::c_uint
}
unsafe fn ZSTD_NCountCost(
    mut count: *const core::ffi::c_uint,
    max: core::ffi::c_uint,
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
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    FSE_writeNCount(
        wksp.as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[u8; 512]>() as size_t,
        norm.as_mut_ptr(),
        max,
        tableLog,
    )
}
unsafe fn ZSTD_entropyCost(
    mut count: *const core::ffi::c_uint,
    max: core::ffi::c_uint,
    total: size_t,
) -> size_t {
    let mut cost = 0 as core::ffi::c_uint;
    let mut s: core::ffi::c_uint = 0;
    s = 0;
    while s <= max {
        let mut norm = ((256 as core::ffi::c_uint).wrapping_mul(*count.offset(s as isize))
            as size_t
            / total) as core::ffi::c_uint;
        if *count.offset(s as isize) != 0 && norm == 0 {
            norm = 1;
        }
        cost = cost.wrapping_add(
            (*count.offset(s as isize))
                .wrapping_mul(*kInverseProbabilityLog256.as_ptr().offset(norm as isize)),
        );
        s = s.wrapping_add(1);
    }
    (cost >> 8) as size_t
}
pub unsafe fn ZSTD_fseBitCost(
    mut ctable: *const FSE_CTable,
    mut count: *const core::ffi::c_uint,
    max: core::ffi::c_uint,
) -> size_t {
    let kAccuracyLog = 8;
    let mut cost = 0 as size_t;
    let mut s: core::ffi::c_uint = 0;
    let mut cstate = FSE_CState_t {
        value: 0,
        stateTable: core::ptr::null::<core::ffi::c_void>(),
        symbolTT: core::ptr::null::<core::ffi::c_void>(),
        stateLog: 0,
    };
    FSE_initCState(&mut cstate, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < max {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    s = 0;
    while s <= max {
        let tableLog = cstate.stateLog;
        let badCost = tableLog.wrapping_add(1) << kAccuracyLog;
        let bitCost = FSE_bitCost(cstate.symbolTT, tableLog, s, kAccuracyLog);
        if *count.offset(s as isize) != 0 {
            if bitCost >= badCost {
                return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
            }
            cost = cost.wrapping_add(*count.offset(s as isize) as size_t * bitCost as size_t);
        }
        s = s.wrapping_add(1);
    }
    cost >> kAccuracyLog
}
pub unsafe fn ZSTD_crossEntropyCost(
    mut norm: *const core::ffi::c_short,
    mut accuracyLog: core::ffi::c_uint,
    mut count: *const core::ffi::c_uint,
    max: core::ffi::c_uint,
) -> size_t {
    let shift = (8 as core::ffi::c_uint).wrapping_sub(accuracyLog);
    let mut cost = 0 as size_t;
    let mut s: core::ffi::c_uint = 0;
    s = 0;
    while s <= max {
        let normAcc = if *norm.offset(s as isize) as core::ffi::c_int != -(1) {
            *norm.offset(s as isize) as core::ffi::c_uint
        } else {
            1
        };
        let norm256 = normAcc << shift;
        cost = cost.wrapping_add(
            (*count.offset(s as isize))
                .wrapping_mul(*kInverseProbabilityLog256.as_ptr().offset(norm256 as isize))
                as size_t,
        );
        s = s.wrapping_add(1);
    }
    cost >> 8
}
pub unsafe fn ZSTD_selectEncodingType(
    mut repeatMode: *mut FSE_repeat,
    mut count: *const core::ffi::c_uint,
    max: core::ffi::c_uint,
    mostFrequent: size_t,
    mut nbSeq: size_t,
    FSELog: core::ffi::c_uint,
    mut prevCTable: *const FSE_CTable,
    mut defaultNorm: *const core::ffi::c_short,
    mut defaultNormLog: u32,
    isDefaultAllowed: ZSTD_DefaultPolicy_e,
    strategy: ZSTD_strategy,
) -> SymbolEncodingType_e {
    if mostFrequent == nbSeq {
        *repeatMode = FSE_repeat_none;
        if isDefaultAllowed as core::ffi::c_uint != 0 && nbSeq <= 2 {
            return set_basic;
        }
        return set_rle;
    }
    if (strategy as core::ffi::c_uint) < ZSTD_lazy as core::ffi::c_int as core::ffi::c_uint {
        if isDefaultAllowed as u64 != 0 {
            let staticFse_nbSeq_max = 1000;
            let mult =
                (10 as core::ffi::c_uint).wrapping_sub(strategy as core::ffi::c_uint) as size_t;
            let baseLog = 3;
            let dynamicFse_nbSeq_min = (((1) << defaultNormLog) * mult) >> baseLog;
            if *repeatMode as core::ffi::c_uint
                == FSE_repeat_valid as core::ffi::c_int as core::ffi::c_uint
                && nbSeq < staticFse_nbSeq_max
            {
                return set_repeat;
            }
            if nbSeq < dynamicFse_nbSeq_min
                || mostFrequent < nbSeq >> defaultNormLog.wrapping_sub(1)
            {
                *repeatMode = FSE_repeat_none;
                return set_basic;
            }
        }
    } else {
        let basicCost = if isDefaultAllowed as core::ffi::c_uint != 0 {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, count, max)
        } else {
            -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t
        };
        let repeatCost = if *repeatMode as core::ffi::c_uint
            != FSE_repeat_none as core::ffi::c_int as core::ffi::c_uint
        {
            ZSTD_fseBitCost(prevCTable, count, max)
        } else {
            -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t
        };
        let NCountCost = ZSTD_NCountCost(count, max, nbSeq, FSELog);
        let compressedCost = (NCountCost << 3).wrapping_add(ZSTD_entropyCost(count, max, nbSeq));
        isDefaultAllowed as u64 != 0;
        if basicCost <= repeatCost && basicCost <= compressedCost {
            *repeatMode = FSE_repeat_none;
            return set_basic;
        }
        if repeatCost <= compressedCost {
            return set_repeat;
        }
    }
    *repeatMode = FSE_repeat_check;
    set_compressed
}
pub unsafe fn ZSTD_buildCTable(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut nextCTable: *mut FSE_CTable,
    mut FSELog: u32,
    mut type_0: SymbolEncodingType_e,
    mut count: *mut core::ffi::c_uint,
    mut max: u32,
    mut codeTable: *const u8,
    mut nbSeq: size_t,
    mut defaultNorm: *const i16,
    mut defaultNormLog: u32,
    mut defaultMax: u32,
    mut prevCTable: *const FSE_CTable,
    mut prevCTableSize: size_t,
    mut entropyWorkspace: *mut core::ffi::c_void,
    mut entropyWorkspaceSize: size_t,
) -> size_t {
    let mut op = dst as *mut u8;
    let oend: *const u8 = op.add(dstCapacity);
    match type_0 as core::ffi::c_uint {
        1 => {
            let err_code = FSE_buildCTable_rle(nextCTable, max as u8);
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if dstCapacity == 0 {
                return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
            }
            *op = *codeTable.offset(0);
            1
        }
        3 => {
            libc::memcpy(
                nextCTable as *mut core::ffi::c_void,
                prevCTable as *const core::ffi::c_void,
                prevCTableSize as libc::size_t,
            );
            0
        }
        0 => {
            let err_code_0 = FSE_buildCTable_wksp(
                nextCTable,
                defaultNorm,
                defaultMax,
                defaultNormLog,
                entropyWorkspace,
                entropyWorkspaceSize,
            );
            if ERR_isError(err_code_0) != 0 {
                return err_code_0;
            }
            0
        }
        2 => {
            let mut wksp = entropyWorkspace as *mut ZSTD_BuildCTableWksp;
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
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            let NCountSize = FSE_writeNCount(
                op as *mut core::ffi::c_void,
                oend.offset_from(op) as core::ffi::c_long as size_t,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
            );
            let err_code_2 = NCountSize;
            if ERR_isError(err_code_2) != 0 {
                return err_code_2;
            }
            let err_code_3 = FSE_buildCTable_wksp(
                nextCTable,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
                ((*wksp).wksp).as_mut_ptr() as *mut core::ffi::c_void,
                ::core::mem::size_of::<[u32; 285]>() as size_t,
            );
            if ERR_isError(err_code_3) != 0 {
                return err_code_3;
            }
            NCountSize
        }
        _ => -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
    }
}
unsafe fn ZSTD_encodeSequences_body(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: core::ffi::c_int,
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
    if ERR_isError(BIT_initCStream(&mut blockStream, dst, dstCapacity)) != 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
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
        *LL_bits
            .as_ptr()
            .offset(*llCodeTable.add(nbSeq.wrapping_sub(1)) as isize) as core::ffi::c_uint,
    );
    if MEM_32bits() != 0 {
        BIT_flushBits(&mut blockStream);
    }
    BIT_addBits(
        &mut blockStream,
        (*sequences.add(nbSeq.wrapping_sub(1))).mlBase as BitContainerType,
        *ML_bits
            .as_ptr()
            .offset(*mlCodeTable.add(nbSeq.wrapping_sub(1)) as isize) as core::ffi::c_uint,
    );
    if MEM_32bits() != 0 {
        BIT_flushBits(&mut blockStream);
    }
    if longOffsets != 0 {
        let ofBits = *ofCodeTable.add(nbSeq.wrapping_sub(1)) as u32;
        let extraBits = ofBits.wrapping_sub(
            if ofBits < ((if MEM_32bits() != 0 { 25 } else { 57 }) as u32).wrapping_sub(1) {
                ofBits
            } else {
                ((if MEM_32bits() != 0 { 25 } else { 57 }) as u32).wrapping_sub(1)
            },
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
        let llBits = *LL_bits.as_ptr().offset(llCode as isize) as u32;
        let ofBits_0 = ofCode as u32;
        let mlBits = *ML_bits.as_ptr().offset(mlCode as isize) as u32;
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
        if MEM_32bits() != 0 {
            BIT_flushBits(&mut blockStream);
        }
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateLitLength,
            llCode as core::ffi::c_uint,
        );
        if MEM_32bits() != 0
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits)
                >= (64 - 7 - (LLFSELog + MLFSELog + OffFSELog)) as u32
        {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(n)).litLength as BitContainerType,
            llBits,
        );
        if MEM_32bits() != 0 && llBits.wrapping_add(mlBits) > 24 {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.add(n)).mlBase as BitContainerType,
            mlBits,
        );
        if MEM_32bits() != 0 || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) > 56 {
            BIT_flushBits(&mut blockStream);
        }
        if longOffsets != 0 {
            let extraBits_0 = ofBits_0.wrapping_sub(
                if ofBits_0 < ((if MEM_32bits() != 0 { 25 } else { 57 }) as u32).wrapping_sub(1) {
                    ofBits_0
                } else {
                    ((if MEM_32bits() != 0 { 25 } else { 57 }) as u32).wrapping_sub(1)
                },
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
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    streamSize
}
unsafe fn ZSTD_encodeSequences_default(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: core::ffi::c_int,
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
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: core::ffi::c_int,
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
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: core::ffi::c_int,
    mut bmi2: core::ffi::c_int,
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
