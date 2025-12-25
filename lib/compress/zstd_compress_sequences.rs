use libc::size_t;

use crate::lib::common::bitstream::{
    BIT_CStream_t, BIT_addBits, BIT_closeCStream, BIT_flushBits, BIT_initCStream, BitContainerType,
};
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{
    FSE_CState_t, FSE_CTable, FSE_bitCost, FSE_encodeSymbol, FSE_flushCState, FSE_initCState,
    FSE_initCState2, FSE_repeat, FSE_repeat_check, FSE_repeat_none, FSE_repeat_valid,
};
use crate::lib::common::mem::{MEM_32bits, MEM_read16};
use crate::lib::common::zstd_internal::{LLFSELog, LL_bits, MLFSELog, ML_bits, OffFSELog};
use crate::lib::compress::fse_compress::{
    FSE_buildCTable_rle, FSE_buildCTable_wksp, FSE_normalizeCount, FSE_optimalTableLog,
    FSE_writeNCount,
};
use crate::lib::compress::zstd_compress::SeqDef;
use crate::lib::zstd::*;
use crate::ZSTD_isError;

pub type SymbolEncodingType_e = core::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type ZSTD_DefaultPolicy_e = core::ffi::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_DefaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_DefaultPolicy_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_BuildCTableWksp {
    pub norm: [i16; 53],
    pub wksp: [u32; 285],
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
unsafe fn ZSTD_getFSEMaxSymbolValue(ctable: *const FSE_CTable) -> core::ffi::c_uint {
    let ptr = ctable as *const core::ffi::c_void;
    let u16ptr = ptr as *const u16;

    u32::from(MEM_read16(u16ptr.add(1) as *const core::ffi::c_void))
}
unsafe fn ZSTD_useLowProbCount(nbSeq: size_t) -> core::ffi::c_uint {
    core::ffi::c_int::from(nbSeq >= 2048) as core::ffi::c_uint
}
unsafe fn ZSTD_NCountCost(
    count: *const core::ffi::c_uint,
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
    if ERR_isError(err_code) {
        return err_code;
    }
    FSE_writeNCount(
        wksp.as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[u8; 512]>(),
        norm.as_mut_ptr(),
        max,
        tableLog,
    )
}
unsafe fn ZSTD_entropyCost(
    count: *const core::ffi::c_uint,
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
    ctable: *const FSE_CTable,
    count: *const core::ffi::c_uint,
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
        return Error::GENERIC.to_error_code();
    }
    s = 0;
    while s <= max {
        let tableLog = cstate.stateLog;
        let badCost = tableLog.wrapping_add(1) << kAccuracyLog;
        let bitCost = FSE_bitCost(cstate.symbolTT, tableLog, s, kAccuracyLog);
        if *count.offset(s as isize) != 0 {
            if bitCost >= badCost {
                return Error::GENERIC.to_error_code();
            }
            cost = cost.wrapping_add(*count.offset(s as isize) as size_t * bitCost as size_t);
        }
        s = s.wrapping_add(1);
    }
    cost >> kAccuracyLog
}
pub unsafe fn ZSTD_crossEntropyCost(
    norm: *const core::ffi::c_short,
    accuracyLog: core::ffi::c_uint,
    count: *const core::ffi::c_uint,
    max: core::ffi::c_uint,
) -> size_t {
    let shift = (8 as core::ffi::c_uint).wrapping_sub(accuracyLog);
    let mut cost = 0 as size_t;
    let mut s: core::ffi::c_uint = 0;
    s = 0;
    while s <= max {
        let normAcc = if core::ffi::c_int::from(*norm.offset(s as isize)) != -(1) {
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
    repeatMode: *mut FSE_repeat,
    count: *const core::ffi::c_uint,
    max: core::ffi::c_uint,
    mostFrequent: size_t,
    nbSeq: size_t,
    FSELog: core::ffi::c_uint,
    prevCTable: *const FSE_CTable,
    defaultNorm: *const core::ffi::c_short,
    defaultNormLog: u32,
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
        if u64::from(isDefaultAllowed) != 0 {
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
            Error::GENERIC.to_error_code()
        };
        let repeatCost = if *repeatMode as core::ffi::c_uint
            != FSE_repeat_none as core::ffi::c_int as core::ffi::c_uint
        {
            ZSTD_fseBitCost(prevCTable, count, max)
        } else {
            Error::GENERIC.to_error_code()
        };
        let NCountCost = ZSTD_NCountCost(count, max, nbSeq, FSELog);
        let compressedCost = (NCountCost << 3).wrapping_add(ZSTD_entropyCost(count, max, nbSeq));
        if isDefaultAllowed != 0 {
            assert!(ZSTD_isError(basicCost) == 0);
            assert!(!(*repeatMode == FSE_repeat_valid && ZSTD_isError(repeatCost) != 0));
        }
        assert!(ZSTD_isError(NCountCost) == 0);
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
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    nextCTable: *mut FSE_CTable,
    FSELog: u32,
    type_0: SymbolEncodingType_e,
    count: *mut core::ffi::c_uint,
    max: u32,
    codeTable: *const u8,
    nbSeq: size_t,
    defaultNorm: *const i16,
    defaultNormLog: u32,
    defaultMax: u32,
    prevCTable: *const FSE_CTable,
    prevCTableSize: size_t,
    entropyWorkspace: *mut core::ffi::c_void,
    entropyWorkspaceSize: size_t,
) -> size_t {
    let op = dst as *mut u8;
    let oend: *const u8 = op.add(dstCapacity);
    match type_0 as core::ffi::c_uint {
        1 => {
            let err_code = FSE_buildCTable_rle(nextCTable, max as u8);
            if ERR_isError(err_code) {
                return err_code;
            }
            if dstCapacity == 0 {
                return Error::dstSize_tooSmall.to_error_code();
            }
            *op = *codeTable;
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
            if ERR_isError(err_code_0) {
                return err_code_0;
            }
            0
        }
        2 => {
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
                nextCTable,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
                ((*wksp).wksp).as_mut_ptr() as *mut core::ffi::c_void,
                ::core::mem::size_of::<[u32; 285]>(),
            );
            if ERR_isError(err_code_3) {
                return err_code_3;
            }
            NCountSize
        }
        _ => Error::GENERIC.to_error_code(),
    }
}
unsafe fn ZSTD_encodeSequences_body(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    CTable_MatchLength: *const FSE_CTable,
    mlCodeTable: *const u8,
    CTable_OffsetBits: *const FSE_CTable,
    ofCodeTable: *const u8,
    CTable_LitLength: *const FSE_CTable,
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: core::ffi::c_int,
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
    FSE_initCState2(
        &mut stateMatchLength,
        CTable_MatchLength,
        u32::from(*mlCodeTable.add(nbSeq.wrapping_sub(1))),
    );
    FSE_initCState2(
        &mut stateOffsetBits,
        CTable_OffsetBits,
        u32::from(*ofCodeTable.add(nbSeq.wrapping_sub(1))),
    );
    FSE_initCState2(
        &mut stateLitLength,
        CTable_LitLength,
        u32::from(*llCodeTable.add(nbSeq.wrapping_sub(1))),
    );
    BIT_addBits(
        &mut blockStream,
        (*sequences.add(nbSeq.wrapping_sub(1))).litLength as BitContainerType,
        core::ffi::c_uint::from(
            *LL_bits
                .as_ptr()
                .offset(*llCodeTable.add(nbSeq.wrapping_sub(1)) as isize),
        ),
    );
    if MEM_32bits() {
        BIT_flushBits(&mut blockStream);
    }
    BIT_addBits(
        &mut blockStream,
        (*sequences.add(nbSeq.wrapping_sub(1))).mlBase as BitContainerType,
        core::ffi::c_uint::from(
            *ML_bits
                .as_ptr()
                .offset(*mlCodeTable.add(nbSeq.wrapping_sub(1)) as isize),
        ),
    );
    if MEM_32bits() {
        BIT_flushBits(&mut blockStream);
    }
    if longOffsets != 0 {
        let ofBits = u32::from(*ofCodeTable.add(nbSeq.wrapping_sub(1)));
        let extraBits = ofBits.wrapping_sub(
            if ofBits < ((if MEM_32bits() { 25 } else { 57 }) as u32).wrapping_sub(1) {
                ofBits
            } else {
                ((if MEM_32bits() { 25 } else { 57 }) as u32).wrapping_sub(1)
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
            core::ffi::c_uint::from(*ofCodeTable.add(nbSeq.wrapping_sub(1))),
        );
    }
    BIT_flushBits(&mut blockStream);
    let mut n: size_t = 0;
    n = nbSeq.wrapping_sub(2);
    while n < nbSeq {
        let llCode = *llCodeTable.add(n);
        let ofCode = *ofCodeTable.add(n);
        let mlCode = *mlCodeTable.add(n);
        let llBits = u32::from(*LL_bits.as_ptr().offset(llCode as isize));
        let ofBits_0 = u32::from(ofCode);
        let mlBits = u32::from(*ML_bits.as_ptr().offset(mlCode as isize));
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateOffsetBits,
            core::ffi::c_uint::from(ofCode),
        );
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateMatchLength,
            core::ffi::c_uint::from(mlCode),
        );
        if MEM_32bits() {
            BIT_flushBits(&mut blockStream);
        }
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateLitLength,
            core::ffi::c_uint::from(llCode),
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
        if longOffsets != 0 {
            let extraBits_0 = ofBits_0.wrapping_sub(
                if ofBits_0 < ((if MEM_32bits() { 25 } else { 57 }) as u32).wrapping_sub(1) {
                    ofBits_0
                } else {
                    ((if MEM_32bits() { 25 } else { 57 }) as u32).wrapping_sub(1)
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
        return Error::dstSize_tooSmall.to_error_code();
    }
    streamSize
}
unsafe fn ZSTD_encodeSequences_default(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    CTable_MatchLength: *const FSE_CTable,
    mlCodeTable: *const u8,
    CTable_OffsetBits: *const FSE_CTable,
    ofCodeTable: *const u8,
    CTable_LitLength: *const FSE_CTable,
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: core::ffi::c_int,
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
    CTable_MatchLength: *const FSE_CTable,
    mlCodeTable: *const u8,
    CTable_OffsetBits: *const FSE_CTable,
    ofCodeTable: *const u8,
    CTable_LitLength: *const FSE_CTable,
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: core::ffi::c_int,
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
    CTable_MatchLength: *const FSE_CTable,
    mlCodeTable: *const u8,
    CTable_OffsetBits: *const FSE_CTable,
    ofCodeTable: *const u8,
    CTable_LitLength: *const FSE_CTable,
    llCodeTable: *const u8,
    sequences: *const SeqDef,
    nbSeq: size_t,
    longOffsets: core::ffi::c_int,
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
