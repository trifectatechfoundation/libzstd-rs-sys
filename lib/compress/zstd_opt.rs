use core::ptr;
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
#[repr(C)]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}

pub type ZSTD_dictMode_e = core::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
#[repr(C)]
pub struct repcodes_s {
    pub rep: [u32; 3],
}
pub type Repcodes_t = repcodes_s;
pub type ZSTD_getAllMatchesFn = Option<
    unsafe fn(
        *mut ZSTD_match_t,
        &mut ZSTD_MatchState_t,
        *mut u32,
        *const u8,
        *const u8,
        *const u32,
        u32,
        u32,
    ) -> u32,
>;
#[repr(C)]
pub struct ZSTD_optLdm_t {
    pub seqStore: RawSeqStore_t,
    pub startPosInBlock: u32,
    pub endPosInBlock: u32,
    pub offset: u32,
}
pub type base_directive_e = core::ffi::c_uint;
pub const base_1guaranteed: base_directive_e = 1;
pub const base_0possible: base_directive_e = 0;

use libc::size_t;

use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::fse::{
    FSE_CState_t, FSE_CTable, FSE_getMaxNbBits, FSE_initCState, FSE_repeat,
};
use crate::lib::common::huf::{HUF_CElt, HUF_repeat, HUF_repeat_valid};
use crate::lib::common::mem::MEM_read32;
use crate::lib::common::zstd_internal::{
    LL_bits, ML_bits, MaxLL, MaxLit, MaxML, MaxOff, MINMATCH, ZSTD_OPT_NUM, ZSTD_REP_NUM,
};
use crate::lib::compress::hist::HIST_count_simple;
use crate::lib::compress::huf_compress::HUF_getNbBitsFromCTable;
use crate::lib::compress::zstd_compress::{
    optState_t, rawSeq, ParamSwitch, RawSeqStore_t, SeqStore_t, ZSTD_MatchState_t, ZSTD_match_t,
    ZSTD_optimal_t, ZSTD_resetSeqStore,
};
use crate::lib::compress::zstd_compress_internal::{
    zop_dynamic, zop_predef, ZSTD_count, ZSTD_count_2segments, ZSTD_getLowestMatchIndex,
    ZSTD_hash3Ptr, ZSTD_hashPtr, ZSTD_index_overlap_check, ZSTD_storeSeq, ZSTD_updateRep,
};
use crate::lib::zstd::{ZSTD_compressionParameters, ZSTD_BLOCKSIZE_MAX};
static mut kNullRawSeqStore: RawSeqStore_t = RawSeqStore_t {
    seq: core::ptr::null_mut(),
    pos: 0,
    posInSequence: 0,
    size: 0,
    capacity: 0,
};
#[inline]
unsafe fn ZSTD_LLcode(litLength: u32) -> u32 {
    static LL_Code: [u8; 64] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20,
        20, 20, 20, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23,
        24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    ];
    static LL_deltaCode: u32 = 19;
    if litLength > 63 {
        (ZSTD_highbit32(litLength)).wrapping_add(LL_deltaCode)
    } else {
        *LL_Code.as_ptr().offset(litLength as isize) as core::ffi::c_uint
    }
}
#[inline]
unsafe fn ZSTD_MLcode(mlBase: u32) -> u32 {
    static ML_Code: [u8; 128] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 32, 33, 33, 34, 34, 35, 35, 36, 36, 36, 36, 37, 37, 37, 37,
        38, 38, 38, 38, 38, 38, 38, 38, 39, 39, 39, 39, 39, 39, 39, 39, 40, 40, 40, 40, 40, 40, 40,
        40, 40, 40, 40, 40, 40, 40, 40, 40, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41,
        41, 41, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
    ];
    static ML_deltaCode: u32 = 36;
    if mlBase > 127 {
        (ZSTD_highbit32(mlBase)).wrapping_add(ML_deltaCode)
    } else {
        *ML_Code.as_ptr().offset(mlBase as isize) as core::ffi::c_uint
    }
}

#[inline]
unsafe fn ZSTD_newRep(rep: *const u32, offBase: u32, ll0: u32) -> Repcodes_t {
    let mut newReps = repcodes_s { rep: [0; 3] };
    libc::memcpy(
        &mut newReps as *mut Repcodes_t as *mut core::ffi::c_void,
        rep as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>() as core::ffi::c_ulong as libc::size_t,
    );
    ZSTD_updateRep((newReps.rep).as_mut_ptr(), offBase, ll0);
    newReps
}

pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2)
    .wrapping_add(1);
pub const ZSTD_LITFREQ_ADD: core::ffi::c_int = 2;
pub const ZSTD_MAX_PRICE: core::ffi::c_int = (1) << 30;
pub const ZSTD_PREDEF_THRESHOLD: core::ffi::c_int = 8;
pub const BITCOST_ACCURACY: core::ffi::c_int = 8;
pub const BITCOST_MULTIPLIER: core::ffi::c_int = (1) << BITCOST_ACCURACY;
#[inline]
unsafe fn ZSTD_bitWeight(stat: u32) -> u32 {
    (ZSTD_highbit32(stat.wrapping_add(1))).wrapping_mul(BITCOST_MULTIPLIER as core::ffi::c_uint)
}
#[inline]
unsafe fn ZSTD_fracWeight(rawStat: u32) -> u32 {
    let stat = rawStat.wrapping_add(1);
    let hb = ZSTD_highbit32(stat);
    let BWeight = hb * BITCOST_MULTIPLIER as u32;
    let FWeight = stat << BITCOST_ACCURACY >> hb;

    BWeight.wrapping_add(FWeight)
}
unsafe fn ZSTD_compressedLiterals(optPtr: *const optState_t) -> core::ffi::c_int {
    ((*optPtr).literalCompressionMode != ParamSwitch::Disable) as core::ffi::c_int
}
unsafe fn ZSTD_setBasePrices(optPtr: *mut optState_t, optLevel: core::ffi::c_int) {
    if ZSTD_compressedLiterals(optPtr) != 0 {
        (*optPtr).litSumBasePrice = if optLevel != 0 {
            ZSTD_fracWeight((*optPtr).litSum)
        } else {
            ZSTD_bitWeight((*optPtr).litSum)
        };
    }
    (*optPtr).litLengthSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).litLengthSum)
    } else {
        ZSTD_bitWeight((*optPtr).litLengthSum)
    };
    (*optPtr).matchLengthSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).matchLengthSum)
    } else {
        ZSTD_bitWeight((*optPtr).matchLengthSum)
    };
    (*optPtr).offCodeSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).offCodeSum)
    } else {
        ZSTD_bitWeight((*optPtr).offCodeSum)
    };
}
unsafe fn sum_u32(table: *const core::ffi::c_uint, nbElts: size_t) -> u32 {
    let mut n: size_t = 0;
    let mut total = 0;
    n = 0;
    while n < nbElts {
        total = (total as core::ffi::c_uint).wrapping_add(*table.add(n)) as u32 as u32;
        n = n.wrapping_add(1);
    }
    total
}
unsafe fn ZSTD_downscaleStats(
    table: *mut core::ffi::c_uint,
    lastEltIndex: u32,
    shift: u32,
    base1: base_directive_e,
) -> u32 {
    let mut s: u32 = 0;
    let mut sum = 0;
    s = 0;
    while s < lastEltIndex.wrapping_add(1) {
        let base = (if base1 as core::ffi::c_uint != 0 {
            1
        } else {
            (*table.offset(s as isize) > 0) as core::ffi::c_int
        }) as core::ffi::c_uint;
        let newStat = base.wrapping_add(*table.offset(s as isize) >> shift);
        sum = (sum as core::ffi::c_uint).wrapping_add(newStat);
        *table.offset(s as isize) = newStat;
        s = s.wrapping_add(1);
    }
    sum
}
unsafe fn ZSTD_scaleStats(table: *mut core::ffi::c_uint, lastEltIndex: u32, logTarget: u32) -> u32 {
    let prevsum = sum_u32(
        table as *const core::ffi::c_uint,
        lastEltIndex.wrapping_add(1) as size_t,
    );
    let factor = prevsum >> logTarget;
    if factor <= 1 {
        return prevsum;
    }
    ZSTD_downscaleStats(
        table,
        lastEltIndex,
        ZSTD_highbit32(factor),
        base_1guaranteed,
    )
}
unsafe fn ZSTD_rescaleFreqs(
    optPtr: *mut optState_t,
    src: *const u8,
    srcSize: size_t,
    optLevel: core::ffi::c_int,
) {
    let compressedLiterals = ZSTD_compressedLiterals(optPtr);
    (*optPtr).priceType = zop_dynamic;
    if (*optPtr).litLengthSum == 0 {
        if srcSize <= ZSTD_PREDEF_THRESHOLD as size_t {
            (*optPtr).priceType = zop_predef;
        }
        if (*(*optPtr).symbolCosts).huf.repeatMode as core::ffi::c_uint
            == HUF_repeat_valid as core::ffi::c_int as core::ffi::c_uint
        {
            (*optPtr).priceType = zop_dynamic;
            if compressedLiterals != 0 {
                let mut lit: core::ffi::c_uint = 0;
                (*optPtr).litSum = 0;
                lit = 0;
                while lit <= MaxLit {
                    let scaleLog = 11u32;
                    let bitCost =
                        HUF_getNbBitsFromCTable(&((*(*optPtr).symbolCosts).huf.CTable), lit);
                    *((*optPtr).litFreq).offset(lit as isize) = (if bitCost != 0 {
                        (1) << scaleLog.wrapping_sub(bitCost)
                    } else {
                        1
                    })
                        as core::ffi::c_uint;
                    (*optPtr).litSum = ((*optPtr).litSum as core::ffi::c_uint)
                        .wrapping_add(*((*optPtr).litFreq).offset(lit as isize))
                        as u32 as u32;
                    lit = lit.wrapping_add(1);
                }
            }
            let mut ll: core::ffi::c_uint = 0;
            let mut llstate = FSE_CState_t {
                value: 0,
                stateTable: core::ptr::null::<core::ffi::c_void>(),
                symbolTT: core::ptr::null::<core::ffi::c_void>(),
                stateLog: 0,
            };
            FSE_initCState(
                &mut llstate,
                ((*(*optPtr).symbolCosts).fse.litlengthCTable).as_ptr(),
            );
            (*optPtr).litLengthSum = 0;
            ll = 0;
            while ll <= MaxLL {
                let scaleLog_0 = 10u32;
                let bitCost_0 = FSE_getMaxNbBits(llstate.symbolTT, ll);
                *((*optPtr).litLengthFreq).offset(ll as isize) = (if bitCost_0 != 0 {
                    (1) << scaleLog_0.wrapping_sub(bitCost_0)
                } else {
                    1
                })
                    as core::ffi::c_uint;
                (*optPtr).litLengthSum = ((*optPtr).litLengthSum as core::ffi::c_uint)
                    .wrapping_add(*((*optPtr).litLengthFreq).offset(ll as isize))
                    as u32 as u32;
                ll = ll.wrapping_add(1);
            }
            let mut ml: core::ffi::c_uint = 0;
            let mut mlstate = FSE_CState_t {
                value: 0,
                stateTable: core::ptr::null::<core::ffi::c_void>(),
                symbolTT: core::ptr::null::<core::ffi::c_void>(),
                stateLog: 0,
            };
            FSE_initCState(
                &mut mlstate,
                ((*(*optPtr).symbolCosts).fse.matchlengthCTable).as_ptr(),
            );
            (*optPtr).matchLengthSum = 0;
            ml = 0;
            while ml <= MaxML {
                let scaleLog_1 = 10u32;
                let bitCost_1 = FSE_getMaxNbBits(mlstate.symbolTT, ml);
                *((*optPtr).matchLengthFreq).offset(ml as isize) = (if bitCost_1 != 0 {
                    (1) << scaleLog_1.wrapping_sub(bitCost_1)
                } else {
                    1
                })
                    as core::ffi::c_uint;
                (*optPtr).matchLengthSum = ((*optPtr).matchLengthSum as core::ffi::c_uint)
                    .wrapping_add(*((*optPtr).matchLengthFreq).offset(ml as isize))
                    as u32 as u32;
                ml = ml.wrapping_add(1);
            }
            let mut of: core::ffi::c_uint = 0;
            let mut ofstate = FSE_CState_t {
                value: 0,
                stateTable: core::ptr::null::<core::ffi::c_void>(),
                symbolTT: core::ptr::null::<core::ffi::c_void>(),
                stateLog: 0,
            };
            FSE_initCState(
                &mut ofstate,
                ((*(*optPtr).symbolCosts).fse.offcodeCTable).as_ptr(),
            );
            (*optPtr).offCodeSum = 0;
            of = 0;
            while of <= MaxOff {
                let scaleLog_2 = 10u32;
                let bitCost_2 = FSE_getMaxNbBits(ofstate.symbolTT, of);
                *((*optPtr).offCodeFreq).offset(of as isize) = (if bitCost_2 != 0 {
                    (1) << scaleLog_2.wrapping_sub(bitCost_2)
                } else {
                    1
                })
                    as core::ffi::c_uint;
                (*optPtr).offCodeSum = ((*optPtr).offCodeSum as core::ffi::c_uint)
                    .wrapping_add(*((*optPtr).offCodeFreq).offset(of as isize))
                    as u32;
                of = of.wrapping_add(1);
            }
        } else {
            if compressedLiterals != 0 {
                let mut lit_0 = MaxLit;
                HIST_count_simple(
                    (*optPtr).litFreq,
                    &mut lit_0,
                    src as *const core::ffi::c_void,
                    srcSize,
                );
                (*optPtr).litSum =
                    ZSTD_downscaleStats((*optPtr).litFreq, MaxLit, 8, base_0possible);
            }
            let baseLLfreqs: [core::ffi::c_uint; 36] = [
                4, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1,
            ];
            libc::memcpy(
                (*optPtr).litLengthFreq as *mut core::ffi::c_void,
                baseLLfreqs.as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[core::ffi::c_uint; 36]>() as core::ffi::c_ulong
                    as libc::size_t,
            );
            (*optPtr).litLengthSum = sum_u32(baseLLfreqs.as_ptr(), (MaxLL + 1) as size_t);
            let mut ml_0: core::ffi::c_uint = 0;
            ml_0 = 0;
            while ml_0 <= MaxML {
                *((*optPtr).matchLengthFreq).offset(ml_0 as isize) = 1;
                ml_0 = ml_0.wrapping_add(1);
            }
            (*optPtr).matchLengthSum = MaxML + 1;
            let baseOFCfreqs: [core::ffi::c_uint; 32] = [
                6, 2, 1, 1, 2, 3, 4, 4, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1,
            ];
            libc::memcpy(
                (*optPtr).offCodeFreq as *mut core::ffi::c_void,
                baseOFCfreqs.as_ptr() as *const core::ffi::c_void,
                ::core::mem::size_of::<[core::ffi::c_uint; 32]>() as core::ffi::c_ulong
                    as libc::size_t,
            );
            (*optPtr).offCodeSum = sum_u32(baseOFCfreqs.as_ptr(), (MaxOff + 1) as size_t);
        }
    } else {
        if compressedLiterals != 0 {
            (*optPtr).litSum = ZSTD_scaleStats((*optPtr).litFreq, MaxLit, 12);
        }
        (*optPtr).litLengthSum = ZSTD_scaleStats((*optPtr).litLengthFreq, MaxLL, 11);
        (*optPtr).matchLengthSum = ZSTD_scaleStats((*optPtr).matchLengthFreq, MaxML, 11);
        (*optPtr).offCodeSum = ZSTD_scaleStats((*optPtr).offCodeFreq, MaxOff, 11);
    }
    ZSTD_setBasePrices(optPtr, optLevel);
}
unsafe fn ZSTD_rawLiteralsCost(
    literals: *const u8,
    litLength: u32,
    optPtr: *const optState_t,
    optLevel: core::ffi::c_int,
) -> u32 {
    if litLength == 0 {
        return 0;
    }
    if ZSTD_compressedLiterals(optPtr) == 0 {
        return (litLength << 3) * BITCOST_MULTIPLIER as u32;
    }
    if (*optPtr).priceType == zop_predef {
        return litLength * 6 * BITCOST_MULTIPLIER as u32;
    }
    let mut price = (*optPtr).litSumBasePrice * litLength;
    let litPriceMax = ((*optPtr).litSumBasePrice).wrapping_sub(BITCOST_MULTIPLIER as u32);
    let mut u: u32 = 0;
    u = 0;
    while u < litLength {
        let mut litPrice = if optLevel != 0 {
            ZSTD_fracWeight(*((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize))
        } else {
            ZSTD_bitWeight(*((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize))
        };
        if litPrice > litPriceMax {
            litPrice = litPriceMax;
        }
        price = price.wrapping_sub(litPrice);
        u = u.wrapping_add(1);
    }
    price
}
unsafe fn ZSTD_litLengthPrice(
    litLength: u32,
    optPtr: *const optState_t,
    optLevel: core::ffi::c_int,
) -> u32 {
    if (*optPtr).priceType == zop_predef {
        return if optLevel != 0 {
            ZSTD_fracWeight(litLength)
        } else {
            ZSTD_bitWeight(litLength)
        };
    }
    if litLength == ZSTD_BLOCKSIZE_MAX as u32 {
        return (BITCOST_MULTIPLIER as u32).wrapping_add(ZSTD_litLengthPrice(
            (ZSTD_BLOCKSIZE_MAX - 1) as u32,
            optPtr,
            optLevel,
        ));
    }
    let llCode = ZSTD_LLcode(litLength);
    ((*LL_bits.as_ptr().offset(llCode as isize) as core::ffi::c_int * BITCOST_MULTIPLIER) as u32)
        .wrapping_add((*optPtr).litLengthSumBasePrice)
        .wrapping_sub(if optLevel != 0 {
            ZSTD_fracWeight(*((*optPtr).litLengthFreq).offset(llCode as isize))
        } else {
            ZSTD_bitWeight(*((*optPtr).litLengthFreq).offset(llCode as isize))
        })
}
#[inline(always)]
unsafe fn ZSTD_getMatchPrice(
    offBase: u32,
    matchLength: u32,
    optPtr: *const optState_t,
    optLevel: core::ffi::c_int,
) -> u32 {
    let mut price: u32 = 0;
    let offCode = ZSTD_highbit32(offBase);
    let mlBase = matchLength.wrapping_sub(MINMATCH as u32);
    if (*optPtr).priceType == zop_predef {
        return (if optLevel != 0 {
            ZSTD_fracWeight(mlBase)
        } else {
            ZSTD_bitWeight(mlBase)
        })
        .wrapping_add(16u32.wrapping_add(offCode) * BITCOST_MULTIPLIER as u32);
    }
    price = (offCode * BITCOST_MULTIPLIER as u32).wrapping_add(
        ((*optPtr).offCodeSumBasePrice).wrapping_sub(if optLevel != 0 {
            ZSTD_fracWeight(*((*optPtr).offCodeFreq).offset(offCode as isize))
        } else {
            ZSTD_bitWeight(*((*optPtr).offCodeFreq).offset(offCode as isize))
        }),
    );
    if optLevel < 2 && offCode >= 20 {
        price = price.wrapping_add(offCode.wrapping_sub(19) * 2 * BITCOST_MULTIPLIER as u32);
    }
    let mlCode = ZSTD_MLcode(mlBase);
    price = price.wrapping_add(
        ((*ML_bits.as_ptr().offset(mlCode as isize) as core::ffi::c_int * BITCOST_MULTIPLIER)
            as u32)
            .wrapping_add(
                ((*optPtr).matchLengthSumBasePrice).wrapping_sub(if optLevel != 0 {
                    ZSTD_fracWeight(*((*optPtr).matchLengthFreq).offset(mlCode as isize))
                } else {
                    ZSTD_bitWeight(*((*optPtr).matchLengthFreq).offset(mlCode as isize))
                }),
            ),
    );
    price = price.wrapping_add((BITCOST_MULTIPLIER / 5) as u32);
    price
}
unsafe fn ZSTD_updateStats(
    optPtr: *mut optState_t,
    litLength: u32,
    literals: *const u8,
    offBase: u32,
    matchLength: u32,
) {
    if ZSTD_compressedLiterals(optPtr) != 0 {
        let mut u: u32 = 0;
        u = 0;
        while u < litLength {
            let fresh2 = &mut (*((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize));
            *fresh2 = (*fresh2).wrapping_add(ZSTD_LITFREQ_ADD as core::ffi::c_uint);
            u = u.wrapping_add(1);
        }
        (*optPtr).litSum = ((*optPtr).litSum).wrapping_add(litLength * ZSTD_LITFREQ_ADD as u32);
    }
    let llCode = ZSTD_LLcode(litLength);
    let fresh3 = &mut (*((*optPtr).litLengthFreq).offset(llCode as isize));
    *fresh3 = (*fresh3).wrapping_add(1);
    (*optPtr).litLengthSum = ((*optPtr).litLengthSum).wrapping_add(1);
    let offCode = ZSTD_highbit32(offBase);
    let fresh4 = &mut (*((*optPtr).offCodeFreq).offset(offCode as isize));
    *fresh4 = (*fresh4).wrapping_add(1);
    (*optPtr).offCodeSum = ((*optPtr).offCodeSum).wrapping_add(1);
    let mlBase = matchLength.wrapping_sub(MINMATCH as u32);
    let mlCode = ZSTD_MLcode(mlBase);
    let fresh5 = &mut (*((*optPtr).matchLengthFreq).offset(mlCode as isize));
    *fresh5 = (*fresh5).wrapping_add(1);
    (*optPtr).matchLengthSum = ((*optPtr).matchLengthSum).wrapping_add(1);
}
#[inline]
unsafe fn ZSTD_readMINMATCH(memPtr: *const core::ffi::c_void, length: u32) -> u32 {
    match length {
        3 => {
            if cfg!(target_endian = "little") {
                MEM_read32(memPtr) << 8
            } else {
                MEM_read32(memPtr) >> 8
            }
        }
        4 | _ => MEM_read32(memPtr),
    }
}
unsafe fn ZSTD_insertAndFindFirstIndexHash3(
    ms: *const ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
) -> u32 {
    let hashTable3 = (*ms).hashTable3;
    let hashLog3 = (*ms).hashLog3;
    let base = (*ms).window.base;
    let mut idx = *nextToUpdate3;
    let target = ip.offset_from(base) as core::ffi::c_long as u32;
    let hash3 = ZSTD_hash3Ptr(ip as *const core::ffi::c_void, hashLog3);
    while idx < target {
        *hashTable3.add(ZSTD_hash3Ptr(
            base.offset(idx as isize) as *const core::ffi::c_void,
            hashLog3,
        )) = idx;
        idx = idx.wrapping_add(1);
    }
    *nextToUpdate3 = target;
    *hashTable3.add(hash3)
}
unsafe fn ZSTD_insertBt1(
    ms: *const ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    target: u32,
    mls: u32,
    extDict: core::ffi::c_int,
) -> u32 {
    let cParams: *const ZSTD_compressionParameters = &(*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const core::ffi::c_void, hashLog, mls);
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1);
    let btMask = (((1) << btLog) - 1) as u32;
    let mut matchIndex = *hashTable.add(h);
    let mut commonLengthSmaller = 0;
    let mut commonLengthLarger = 0;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut match_0 = core::ptr::null::<u8>();
    let curr = ip.offset_from(base) as core::ffi::c_long as u32;
    let btLow = if btMask >= curr {
        0
    } else {
        curr.wrapping_sub(btMask)
    };
    let mut smallerPtr = bt.offset((2 * (curr & btMask)) as isize);
    let mut largerPtr = smallerPtr.add(1);
    let mut dummy32: u32 = 0;
    let windowLow = ZSTD_getLowestMatchIndex(ms, target, (*cParams).windowLog);
    let mut matchEndIdx = curr.wrapping_add(8).wrapping_add(1);
    let mut bestLength = 8;
    let mut nbCompares = (1 as core::ffi::c_uint) << (*cParams).searchLog;
    *hashTable.add(h) = curr;
    while nbCompares != 0 && matchIndex >= windowLow {
        let nextPtr = bt.offset((2 * (matchIndex & btMask)) as isize);
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        if extDict == 0 || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
            match_0 = base.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.add(matchLength),
                match_0.add(matchLength),
                iend,
            ));
        } else {
            match_0 = dictBase.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.add(matchLength),
                match_0.add(matchLength),
                iend,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                match_0 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            bestLength = matchLength;
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                matchEndIdx = matchIndex.wrapping_add(matchLength as u32);
            }
        }
        if ip.add(matchLength) == iend {
            break;
        } else {
            if (*match_0.add(matchLength) as core::ffi::c_int)
                < *ip.add(matchLength) as core::ffi::c_int
            {
                *smallerPtr = matchIndex;
                commonLengthSmaller = matchLength;
                if matchIndex <= btLow {
                    smallerPtr = &mut dummy32;
                    break;
                } else {
                    smallerPtr = nextPtr.add(1);
                    matchIndex = *nextPtr.add(1);
                }
            } else {
                *largerPtr = matchIndex;
                commonLengthLarger = matchLength;
                if matchIndex <= btLow {
                    largerPtr = &mut dummy32;
                    break;
                } else {
                    largerPtr = nextPtr;
                    matchIndex = *nextPtr;
                }
            }
            nbCompares = nbCompares.wrapping_sub(1);
        }
    }
    *largerPtr = 0;
    *smallerPtr = *largerPtr;
    let mut positions = 0;
    if bestLength > 384 {
        positions = if (192) < bestLength.wrapping_sub(384) as u32 {
            192
        } else {
            bestLength.wrapping_sub(384) as u32
        };
    }
    if positions > matchEndIdx.wrapping_sub(curr.wrapping_add(8)) {
        positions
    } else {
        matchEndIdx.wrapping_sub(curr.wrapping_add(8))
    }
}
#[inline(always)]
unsafe fn ZSTD_updateTree_internal(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) {
    let base = ms.window.base;
    let target = ip.offset_from(base) as core::ffi::c_long as u32;
    let mut idx = ms.nextToUpdate;
    while idx < target {
        let forward = ZSTD_insertBt1(
            ms,
            base.offset(idx as isize),
            iend,
            target,
            mls,
            (dictMode as core::ffi::c_uint == ZSTD_extDict as core::ffi::c_int as core::ffi::c_uint)
                as core::ffi::c_int,
        );
        idx = idx.wrapping_add(forward);
    }
    ms.nextToUpdate = target;
}
pub unsafe fn ZSTD_updateTree(ms: &mut ZSTD_MatchState_t, ip: *const u8, iend: *const u8) {
    ZSTD_updateTree_internal(ms, ip, iend, ms.cParams.minMatch, ZSTD_noDict);
}
#[inline(always)]
unsafe fn ZSTD_insertBtAndGetAllMatches(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iLimit: *const u8,
    dictMode: ZSTD_dictMode_e,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
    mls: u32,
) -> u32 {
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let sufficient_len = if (*cParams).targetLength < (((1) << 12) - 1) as core::ffi::c_uint {
        (*cParams).targetLength
    } else {
        (((1) << 12) - 1) as core::ffi::c_uint
    };
    let base = ms.window.base;
    let curr = ip.offset_from(base) as core::ffi::c_long as u32;
    let hashLog = (*cParams).hashLog;
    let minMatch = (if mls == 3 { 3 } else { 4 }) as u32;
    let hashTable = ms.hashTable;
    let h = ZSTD_hashPtr(ip as *const core::ffi::c_void, hashLog, mls);
    let mut matchIndex = *hashTable.add(h);
    let bt = ms.chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as core::ffi::c_uint);
    let btMask = ((1 as core::ffi::c_uint) << btLog).wrapping_sub(1);
    let mut commonLengthSmaller = 0;
    let mut commonLengthLarger = 0;
    let dictBase = ms.window.dictBase;
    let dictLimit = ms.window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let btLow = if btMask >= curr {
        0
    } else {
        curr.wrapping_sub(btMask)
    };
    let windowLow = ZSTD_getLowestMatchIndex(ms, curr, (*cParams).windowLog);
    let matchLow = if windowLow != 0 { windowLow } else { 1 };
    let mut smallerPtr = bt.offset((2 * (curr & btMask)) as isize);
    let mut largerPtr = bt.offset((2 * (curr & btMask)) as isize).add(1);
    let mut matchEndIdx = curr.wrapping_add(8).wrapping_add(1);
    let mut dummy32: u32 = 0;
    let mut mnum = 0u32;
    let mut nbCompares = (1 as core::ffi::c_uint) << (*cParams).searchLog;
    let dms = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        ms.dictMatchState
    } else {
        core::ptr::null()
    };
    let dmsCParams = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        &(*dms).cParams
    } else {
        core::ptr::null()
    };
    let dmsBase = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        (*dms).window.base
    } else {
        core::ptr::null()
    };
    let dmsEnd = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        (*dms).window.nextSrc
    } else {
        core::ptr::null()
    };
    let dmsHighLimit = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        dmsEnd.offset_from(dmsBase) as core::ffi::c_long as u32
    } else {
        0
    };
    let dmsLowLimit = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        (*dms).window.lowLimit
    } else {
        0
    };
    let dmsIndexDelta = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        windowLow.wrapping_sub(dmsHighLimit)
    } else {
        0
    };
    let dmsHashLog = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        (*dmsCParams).hashLog
    } else {
        hashLog
    };
    let dmsBtLog = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        ((*dmsCParams).chainLog).wrapping_sub(1)
    } else {
        btLog
    };
    let dmsBtMask = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
    {
        ((1 as core::ffi::c_uint) << dmsBtLog).wrapping_sub(1)
    } else {
        0
    };
    let dmsBtLow = if dictMode as core::ffi::c_uint
        == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
        && dmsBtMask < dmsHighLimit.wrapping_sub(dmsLowLimit)
    {
        dmsHighLimit.wrapping_sub(dmsBtMask)
    } else {
        dmsLowLimit
    };
    let mut bestLength = lengthToBeat.wrapping_sub(1) as size_t;
    let lastR = (ZSTD_REP_NUM as u32).wrapping_add(ll0);
    let mut repCode: u32 = 0;
    repCode = ll0;
    while repCode < lastR {
        let repOffset = if repCode == ZSTD_REP_NUM as u32 {
            (*rep).wrapping_sub(1)
        } else {
            *rep.offset(repCode as isize)
        };
        let repIndex = curr.wrapping_sub(repOffset);
        let mut repLen = 0;
        if repOffset.wrapping_sub(1) < curr.wrapping_sub(dictLimit) {
            if (repIndex >= windowLow) as core::ffi::c_int
                & (ZSTD_readMINMATCH(ip as *const core::ffi::c_void, minMatch)
                    == ZSTD_readMINMATCH(
                        ip.offset(-(repOffset as isize)) as *const core::ffi::c_void,
                        minMatch,
                    )) as core::ffi::c_int
                != 0
            {
                repLen = (ZSTD_count(
                    ip.offset(minMatch as isize),
                    ip.offset(minMatch as isize).offset(-(repOffset as isize)),
                    iLimit,
                ) as u32)
                    .wrapping_add(minMatch);
            }
        } else {
            let repMatch = if dictMode as core::ffi::c_uint
                == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
            {
                dmsBase
                    .offset(repIndex as isize)
                    .offset(-(dmsIndexDelta as isize))
            } else {
                dictBase.offset(repIndex as isize)
            };
            if dictMode as core::ffi::c_uint
                == ZSTD_extDict as core::ffi::c_int as core::ffi::c_uint
                && (repOffset.wrapping_sub(1) < curr.wrapping_sub(windowLow)) as core::ffi::c_int
                    & ZSTD_index_overlap_check(dictLimit, repIndex)
                    != 0
                && ZSTD_readMINMATCH(ip as *const core::ffi::c_void, minMatch)
                    == ZSTD_readMINMATCH(repMatch as *const core::ffi::c_void, minMatch)
            {
                repLen = (ZSTD_count_2segments(
                    ip.offset(minMatch as isize),
                    repMatch.offset(minMatch as isize),
                    iLimit,
                    dictEnd,
                    prefixStart,
                ) as u32)
                    .wrapping_add(minMatch);
            }
            if dictMode as core::ffi::c_uint
                == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
                && (repOffset.wrapping_sub(1)
                    < curr.wrapping_sub(dmsLowLimit.wrapping_add(dmsIndexDelta)))
                    as core::ffi::c_int
                    & ZSTD_index_overlap_check(dictLimit, repIndex)
                    != 0
                && ZSTD_readMINMATCH(ip as *const core::ffi::c_void, minMatch)
                    == ZSTD_readMINMATCH(repMatch as *const core::ffi::c_void, minMatch)
            {
                repLen = (ZSTD_count_2segments(
                    ip.offset(minMatch as isize),
                    repMatch.offset(minMatch as isize),
                    iLimit,
                    dmsEnd,
                    prefixStart,
                ) as u32)
                    .wrapping_add(minMatch);
            }
        }
        if repLen as size_t > bestLength {
            bestLength = repLen as size_t;
            (*matches.offset(mnum as isize)).off = repCode.wrapping_sub(ll0).wrapping_add(1);
            (*matches.offset(mnum as isize)).len = repLen;
            mnum = mnum.wrapping_add(1);
            if (repLen > sufficient_len) as core::ffi::c_int
                | (ip.offset(repLen as isize) == iLimit) as core::ffi::c_int
                != 0
            {
                return mnum;
            }
        }
        repCode = repCode.wrapping_add(1);
    }
    if mls == 3 && bestLength < mls as size_t {
        let matchIndex3 = ZSTD_insertAndFindFirstIndexHash3(ms, nextToUpdate3, ip);
        if (matchIndex3 >= matchLow) as core::ffi::c_int
            & (curr.wrapping_sub(matchIndex3) < ((1) << 18) as u32) as core::ffi::c_int
            != 0
        {
            let mut mlen: size_t = 0;
            if dictMode as core::ffi::c_uint == ZSTD_noDict as core::ffi::c_int as core::ffi::c_uint
                || dictMode as core::ffi::c_uint
                    == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
                || matchIndex3 >= dictLimit
            {
                let match_0 = base.offset(matchIndex3 as isize);
                mlen = ZSTD_count(ip, match_0, iLimit);
            } else {
                let match_1 = dictBase.offset(matchIndex3 as isize);
                mlen = ZSTD_count_2segments(ip, match_1, iLimit, dictEnd, prefixStart);
            }
            if mlen >= mls as size_t {
                bestLength = mlen;
                (*matches).off = curr
                    .wrapping_sub(matchIndex3)
                    .wrapping_add(ZSTD_REP_NUM as u32);
                (*matches).len = mlen as u32;
                mnum = 1;
                if (mlen > sufficient_len as size_t) as core::ffi::c_int
                    | (ip.add(mlen) == iLimit) as core::ffi::c_int
                    != 0
                {
                    ms.nextToUpdate = curr.wrapping_add(1);
                    return 1;
                }
            }
        }
    }
    *hashTable.add(h) = curr;
    while nbCompares != 0 && matchIndex >= matchLow {
        let nextPtr = bt.offset((2 * (matchIndex & btMask)) as isize);
        let mut match_2 = core::ptr::null::<u8>();
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        if dictMode as core::ffi::c_uint == ZSTD_noDict as core::ffi::c_int as core::ffi::c_uint
            || dictMode as core::ffi::c_uint
                == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
            || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
        {
            match_2 = base.offset(matchIndex as isize);
            if matchIndex >= dictLimit {
                assert!(libc::memcmp(match_2.cast(), ip.cast(), matchLength) == 0);
                /* ensure early section of match is equal as expected */
            }
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.add(matchLength),
                match_2.add(matchLength),
                iLimit,
            ));
        } else {
            match_2 = dictBase.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.add(matchLength),
                match_2.add(matchLength),
                iLimit,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                match_2 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                matchEndIdx = matchIndex.wrapping_add(matchLength as u32);
            }
            bestLength = matchLength;
            (*matches.offset(mnum as isize)).off = curr
                .wrapping_sub(matchIndex)
                .wrapping_add(ZSTD_REP_NUM as u32);
            (*matches.offset(mnum as isize)).len = matchLength as u32;
            mnum = mnum.wrapping_add(1);
            if (matchLength > ZSTD_OPT_NUM as size_t) as core::ffi::c_int
                | (ip.add(matchLength) == iLimit) as core::ffi::c_int
                != 0
            {
                if dictMode as core::ffi::c_uint
                    == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
                {
                    nbCompares = 0;
                }
                break;
            }
        }
        if (*match_2.add(matchLength) as core::ffi::c_int)
            < *ip.add(matchLength) as core::ffi::c_int
        {
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32;
                break;
            } else {
                smallerPtr = nextPtr.add(1);
                matchIndex = *nextPtr.add(1);
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32;
                break;
            } else {
                largerPtr = nextPtr;
                matchIndex = *nextPtr;
            }
        }
        nbCompares = nbCompares.wrapping_sub(1);
    }
    *largerPtr = 0;
    *smallerPtr = *largerPtr;
    if dictMode as core::ffi::c_uint == ZSTD_dictMatchState as core::ffi::c_int as core::ffi::c_uint
        && nbCompares != 0
    {
        let dmsH = ZSTD_hashPtr(ip as *const core::ffi::c_void, dmsHashLog, mls);
        let mut dictMatchIndex = *((*dms).hashTable).add(dmsH);
        let dmsBt: *const u32 = (*dms).chainTable;
        commonLengthLarger = 0;
        commonLengthSmaller = commonLengthLarger;
        while nbCompares != 0 && dictMatchIndex > dmsLowLimit {
            let nextPtr_0 = dmsBt.offset((2 * (dictMatchIndex & dmsBtMask)) as isize);
            let mut matchLength_0 = if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else {
                commonLengthLarger
            };
            let mut match_3 = dmsBase.offset(dictMatchIndex as isize);
            matchLength_0 = matchLength_0.wrapping_add(ZSTD_count_2segments(
                ip.add(matchLength_0),
                match_3.add(matchLength_0),
                iLimit,
                dmsEnd,
                prefixStart,
            ));
            if (dictMatchIndex as size_t).wrapping_add(matchLength_0) >= dmsHighLimit as size_t {
                match_3 = base
                    .offset(dictMatchIndex as isize)
                    .offset(dmsIndexDelta as isize);
            }
            if matchLength_0 > bestLength {
                matchIndex = dictMatchIndex.wrapping_add(dmsIndexDelta);
                if matchLength_0 > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                    matchEndIdx = matchIndex.wrapping_add(matchLength_0 as u32);
                }
                bestLength = matchLength_0;
                (*matches.offset(mnum as isize)).off = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as u32);
                (*matches.offset(mnum as isize)).len = matchLength_0 as u32;
                mnum = mnum.wrapping_add(1);
                if (matchLength_0 > ZSTD_OPT_NUM as size_t) as core::ffi::c_int
                    | (ip.add(matchLength_0) == iLimit) as core::ffi::c_int
                    != 0
                {
                    break;
                }
            }
            if dictMatchIndex <= dmsBtLow {
                break;
            }
            if (*match_3.add(matchLength_0) as core::ffi::c_int)
                < *ip.add(matchLength_0) as core::ffi::c_int
            {
                commonLengthSmaller = matchLength_0;
                dictMatchIndex = *nextPtr_0.add(1);
            } else {
                commonLengthLarger = matchLength_0;
                dictMatchIndex = *nextPtr_0;
            }
            nbCompares = nbCompares.wrapping_sub(1);
        }
    }
    ms.nextToUpdate = matchEndIdx.wrapping_sub(8);
    mnum
}
#[inline(always)]
unsafe fn ZSTD_btGetAllMatches_internal(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
    dictMode: ZSTD_dictMode_e,
    mls: u32,
) -> u32 {
    if ip < (ms.window.base).offset(ms.nextToUpdate as isize) {
        return 0;
    }
    ZSTD_updateTree_internal(ms, ip, iHighLimit, mls, dictMode);
    ZSTD_insertBtAndGetAllMatches(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        dictMode,
        rep,
        ll0,
        lengthToBeat,
        mls,
    )
}
unsafe fn ZSTD_btGetAllMatches_noDict_5(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        5,
    )
}
unsafe fn ZSTD_btGetAllMatches_noDict_6(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        6,
    )
}
unsafe fn ZSTD_btGetAllMatches_noDict_4(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        4,
    )
}
unsafe fn ZSTD_btGetAllMatches_noDict_3(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        3,
    )
}
unsafe fn ZSTD_btGetAllMatches_extDict_5(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        5,
    )
}
unsafe fn ZSTD_btGetAllMatches_extDict_6(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        6,
    )
}
unsafe fn ZSTD_btGetAllMatches_extDict_4(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        4,
    )
}
unsafe fn ZSTD_btGetAllMatches_extDict_3(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        3,
    )
}
unsafe fn ZSTD_btGetAllMatches_dictMatchState_5(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        5,
    )
}
unsafe fn ZSTD_btGetAllMatches_dictMatchState_6(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        6,
    )
}
unsafe fn ZSTD_btGetAllMatches_dictMatchState_3(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        3,
    )
}
unsafe fn ZSTD_btGetAllMatches_dictMatchState_4(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: *const u32,
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        4,
    )
}
unsafe fn ZSTD_selectBtGetAllMatches(
    ms: *const ZSTD_MatchState_t,
    dictMode: ZSTD_dictMode_e,
) -> ZSTD_getAllMatchesFn {
    let getAllMatchesFns: [[ZSTD_getAllMatchesFn; 4]; 3] = [
        [
            Some(
                ZSTD_btGetAllMatches_noDict_3
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_4
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_5
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_6
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
        ],
        [
            Some(
                ZSTD_btGetAllMatches_extDict_3
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_4
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_5
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_6
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
        ],
        [
            Some(
                ZSTD_btGetAllMatches_dictMatchState_3
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_4
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_5
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_6
                    as unsafe fn(
                        *mut ZSTD_match_t,
                        &mut ZSTD_MatchState_t,
                        *mut u32,
                        *const u8,
                        *const u8,
                        *const u32,
                        u32,
                        u32,
                    ) -> u32,
            ),
        ],
    ];
    let mls = if 3
        > (if (*ms).cParams.minMatch < 6 {
            (*ms).cParams.minMatch
        } else {
            6
        }) {
        3
    } else if (*ms).cParams.minMatch < 6 {
        (*ms).cParams.minMatch
    } else {
        6
    };
    *(*getAllMatchesFns
        .as_ptr()
        .offset(dictMode as core::ffi::c_int as isize))
    .as_ptr()
    .offset(mls.wrapping_sub(3) as isize)
}
unsafe fn ZSTD_optLdm_skipRawSeqStoreBytes(rawSeqStore: *mut RawSeqStore_t, nbBytes: size_t) {
    let mut currPos = ((*rawSeqStore).posInSequence).wrapping_add(nbBytes) as u32;
    while currPos != 0 && (*rawSeqStore).pos < (*rawSeqStore).size {
        let currSeq = *((*rawSeqStore).seq).add((*rawSeqStore).pos);
        if currPos >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            currPos = currPos.wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength));
            (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
        } else {
            (*rawSeqStore).posInSequence = currPos as size_t;
            break;
        }
    }
    if currPos == 0 || (*rawSeqStore).pos == (*rawSeqStore).size {
        (*rawSeqStore).posInSequence = 0;
    }
}
unsafe fn ZSTD_opt_getNextMatchAndUpdateSeqStore(
    optLdm: *mut ZSTD_optLdm_t,
    currPosInBlock: u32,
    blockBytesRemaining: u32,
) {
    let mut currSeq = rawSeq {
        offset: 0,
        litLength: 0,
        matchLength: 0,
    };
    let mut currBlockEndPos: u32 = 0;
    let mut literalsBytesRemaining: u32 = 0;
    let mut matchBytesRemaining: u32 = 0;
    if (*optLdm).seqStore.size == 0 || (*optLdm).seqStore.pos >= (*optLdm).seqStore.size {
        (*optLdm).startPosInBlock = UINT_MAX;
        (*optLdm).endPosInBlock = UINT_MAX;
        return;
    }
    currSeq = *((*optLdm).seqStore.seq).add((*optLdm).seqStore.pos);
    currBlockEndPos = currPosInBlock.wrapping_add(blockBytesRemaining);
    literalsBytesRemaining = if (*optLdm).seqStore.posInSequence < currSeq.litLength as size_t {
        (currSeq.litLength).wrapping_sub((*optLdm).seqStore.posInSequence as u32)
    } else {
        0
    };
    matchBytesRemaining = if literalsBytesRemaining == 0 {
        (currSeq.matchLength)
            .wrapping_sub(((*optLdm).seqStore.posInSequence as u32).wrapping_sub(currSeq.litLength))
    } else {
        currSeq.matchLength
    };
    if literalsBytesRemaining >= blockBytesRemaining {
        (*optLdm).startPosInBlock = UINT_MAX;
        (*optLdm).endPosInBlock = UINT_MAX;
        ZSTD_optLdm_skipRawSeqStoreBytes(&mut (*optLdm).seqStore, blockBytesRemaining as size_t);
        return;
    }
    (*optLdm).startPosInBlock = currPosInBlock.wrapping_add(literalsBytesRemaining);
    (*optLdm).endPosInBlock = ((*optLdm).startPosInBlock).wrapping_add(matchBytesRemaining);
    (*optLdm).offset = currSeq.offset;
    if (*optLdm).endPosInBlock > currBlockEndPos {
        (*optLdm).endPosInBlock = currBlockEndPos;
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut (*optLdm).seqStore,
            currBlockEndPos.wrapping_sub(currPosInBlock) as size_t,
        );
    } else {
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut (*optLdm).seqStore,
            literalsBytesRemaining.wrapping_add(matchBytesRemaining) as size_t,
        );
    };
}
unsafe fn ZSTD_optLdm_maybeAddMatch(
    matches: *mut ZSTD_match_t,
    nbMatches: *mut u32,
    optLdm: *const ZSTD_optLdm_t,
    currPosInBlock: u32,
    minMatch: u32,
) {
    let posDiff = currPosInBlock.wrapping_sub((*optLdm).startPosInBlock);
    let candidateMatchLength = ((*optLdm).endPosInBlock)
        .wrapping_sub((*optLdm).startPosInBlock)
        .wrapping_sub(posDiff);
    if currPosInBlock < (*optLdm).startPosInBlock
        || currPosInBlock >= (*optLdm).endPosInBlock
        || candidateMatchLength < minMatch
    {
        return;
    }
    if *nbMatches == 0
        || candidateMatchLength > (*matches.offset((*nbMatches).wrapping_sub(1) as isize)).len
            && *nbMatches < ZSTD_OPT_NUM as u32
    {
        let candidateOffBase = ((*optLdm).offset).wrapping_add(ZSTD_REP_NUM as u32);
        (*matches.offset(*nbMatches as isize)).len = candidateMatchLength;
        (*matches.offset(*nbMatches as isize)).off = candidateOffBase;
        *nbMatches = (*nbMatches).wrapping_add(1);
    }
}
unsafe fn ZSTD_optLdm_processMatchCandidate(
    optLdm: *mut ZSTD_optLdm_t,
    matches: *mut ZSTD_match_t,
    nbMatches: *mut u32,
    currPosInBlock: u32,
    remainingBytes: u32,
    minMatch: u32,
) {
    if (*optLdm).seqStore.size == 0 || (*optLdm).seqStore.pos >= (*optLdm).seqStore.size {
        return;
    }
    if currPosInBlock >= (*optLdm).endPosInBlock {
        if currPosInBlock > (*optLdm).endPosInBlock {
            let posOvershoot = currPosInBlock.wrapping_sub((*optLdm).endPosInBlock);
            ZSTD_optLdm_skipRawSeqStoreBytes(&mut (*optLdm).seqStore, posOvershoot as size_t);
        }
        ZSTD_opt_getNextMatchAndUpdateSeqStore(optLdm, currPosInBlock, remainingBytes);
    }
    ZSTD_optLdm_maybeAddMatch(matches, nbMatches, optLdm, currPosInBlock, minMatch);
}
#[inline(always)]
unsafe fn ZSTD_compressBlock_opt_generic(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    optLevel: core::ffi::c_int,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let mut current_block: u64;
    let optStatePtr: *mut optState_t = &mut ms.opt;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.add(srcSize);
    let ilimit = iend.sub(8);
    let base = ms.window.base;
    let prefixStart = base.offset(ms.window.dictLimit as isize);
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let getAllMatches = ZSTD_selectBtGetAllMatches(ms, dictMode);
    let sufficient_len = if (*cParams).targetLength < (((1) << 12) - 1) as core::ffi::c_uint {
        (*cParams).targetLength
    } else {
        (((1) << 12) - 1) as core::ffi::c_uint
    };
    let minMatch = (if (*cParams).minMatch == 3 { 3 } else { 4 }) as u32;
    let mut nextToUpdate3 = ms.nextToUpdate;
    let opt = (*optStatePtr).priceTable;
    let matches = (*optStatePtr).matchTable;
    let mut lastStretch = ZSTD_optimal_t {
        price: 0,
        off: 0,
        mlen: 0,
        litlen: 0,
        rep: [0; 3],
    };
    let mut optLdm = ZSTD_optLdm_t {
        seqStore: RawSeqStore_t {
            seq: core::ptr::null_mut::<rawSeq>(),
            pos: 0,
            posInSequence: 0,
            size: 0,
            capacity: 0,
        },
        startPosInBlock: 0,
        endPosInBlock: 0,
        offset: 0,
    };
    ptr::write_bytes(
        &mut lastStretch as *mut ZSTD_optimal_t as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_optimal_t>(),
    );
    optLdm.seqStore = if !(ms.ldmSeqStore).is_null() {
        *ms.ldmSeqStore
    } else {
        kNullRawSeqStore
    };
    optLdm.offset = 0;
    optLdm.startPosInBlock = optLdm.offset;
    optLdm.endPosInBlock = optLdm.startPosInBlock;
    ZSTD_opt_getNextMatchAndUpdateSeqStore(
        &mut optLdm,
        ip.offset_from(istart) as core::ffi::c_long as u32,
        iend.offset_from(ip) as core::ffi::c_long as u32,
    );
    ZSTD_rescaleFreqs(optStatePtr, src as *const u8, srcSize, optLevel);
    ip = ip.offset((ip == prefixStart) as core::ffi::c_int as isize);
    while ip < ilimit {
        let mut cur: u32 = 0;
        let mut last_pos = 0;
        let litlen = ip.offset_from(anchor) as core::ffi::c_long as u32;
        let ll0 = (litlen == 0) as core::ffi::c_int as u32;
        let mut nbMatches = getAllMatches.unwrap_unchecked()(
            matches,
            ms,
            &mut nextToUpdate3,
            ip,
            iend,
            rep as *const u32,
            ll0,
            minMatch,
        );
        ZSTD_optLdm_processMatchCandidate(
            &mut optLdm,
            matches,
            &mut nbMatches,
            ip.offset_from(istart) as core::ffi::c_long as u32,
            iend.offset_from(ip) as core::ffi::c_long as u32,
            minMatch,
        );
        if nbMatches == 0 {
            ip = ip.add(1);
        } else {
            (*opt).mlen = 0;
            (*opt).litlen = litlen;
            (*opt).price = ZSTD_litLengthPrice(litlen, optStatePtr, optLevel) as core::ffi::c_int;
            libc::memcpy(
                &mut (*opt).rep as *mut [u32; 3] as *mut core::ffi::c_void,
                rep as *const core::ffi::c_void,
                ::core::mem::size_of::<[u32; 3]>() as core::ffi::c_ulong as libc::size_t,
            );
            let maxML = (*matches.offset(nbMatches.wrapping_sub(1) as isize)).len;
            let maxOffBase = (*matches.offset(nbMatches.wrapping_sub(1) as isize)).off;
            if maxML > sufficient_len {
                lastStretch.litlen = 0;
                lastStretch.mlen = maxML;
                lastStretch.off = maxOffBase;
                cur = 0;
                last_pos = maxML;
            } else {
                let mut pos: u32 = 0;
                let mut matchNb: u32 = 0;
                pos = 1;
                while pos < minMatch {
                    (*opt.offset(pos as isize)).price = ZSTD_MAX_PRICE;
                    (*opt.offset(pos as isize)).mlen = 0;
                    (*opt.offset(pos as isize)).litlen = litlen.wrapping_add(pos);
                    pos = pos.wrapping_add(1);
                }
                matchNb = 0;
                while matchNb < nbMatches {
                    let offBase = (*matches.offset(matchNb as isize)).off;
                    let end = (*matches.offset(matchNb as isize)).len;
                    while pos <= end {
                        let matchPrice = ZSTD_getMatchPrice(offBase, pos, optStatePtr, optLevel)
                            as core::ffi::c_int;
                        let sequencePrice = (*opt).price + matchPrice;
                        (*opt.offset(pos as isize)).mlen = pos;
                        (*opt.offset(pos as isize)).off = offBase;
                        (*opt.offset(pos as isize)).litlen = 0;
                        (*opt.offset(pos as isize)).price = sequencePrice
                            + ZSTD_litLengthPrice(0, optStatePtr, optLevel) as core::ffi::c_int;
                        pos = pos.wrapping_add(1);
                    }
                    matchNb = matchNb.wrapping_add(1);
                }
                last_pos = pos.wrapping_sub(1);
                (*opt.offset(pos as isize)).price = ZSTD_MAX_PRICE;
                cur = 1;
                loop {
                    if cur > last_pos {
                        current_block = 10357520176418200368;
                        break;
                    }
                    let inr = ip.offset(cur as isize);
                    let litlen_0 =
                        ((*opt.offset(cur.wrapping_sub(1) as isize)).litlen).wrapping_add(1);
                    let price = (*opt.offset(cur.wrapping_sub(1) as isize)).price
                        + ZSTD_rawLiteralsCost(
                            ip.offset(cur as isize).sub(1),
                            1,
                            optStatePtr,
                            optLevel,
                        ) as core::ffi::c_int
                        + (ZSTD_litLengthPrice(litlen_0, optStatePtr, optLevel)
                            as core::ffi::c_int
                            - ZSTD_litLengthPrice(litlen_0.wrapping_sub(1), optStatePtr, optLevel)
                                as core::ffi::c_int);
                    if price <= (*opt.offset(cur as isize)).price {
                        let prevMatch = *opt.offset(cur as isize);
                        *opt.offset(cur as isize) = *opt.offset(cur.wrapping_sub(1) as isize);
                        (*opt.offset(cur as isize)).litlen = litlen_0;
                        (*opt.offset(cur as isize)).price = price;
                        if optLevel >= 1
                            && prevMatch.litlen == 0
                            && (ZSTD_litLengthPrice(1, optStatePtr, optLevel) as core::ffi::c_int
                                - ZSTD_litLengthPrice((1 - 1) as u32, optStatePtr, optLevel)
                                    as core::ffi::c_int)
                                < 0
                            && (ip.offset(cur as isize) < iend) as core::ffi::c_int
                                as core::ffi::c_long
                                != 0
                        {
                            let with1literal = prevMatch.price
                                + ZSTD_rawLiteralsCost(
                                    ip.offset(cur as isize),
                                    1,
                                    optStatePtr,
                                    optLevel,
                                ) as core::ffi::c_int
                                + (ZSTD_litLengthPrice(1, optStatePtr, optLevel)
                                    as core::ffi::c_int
                                    - ZSTD_litLengthPrice((1 - 1) as u32, optStatePtr, optLevel)
                                        as core::ffi::c_int);
                            let withMoreLiterals = price
                                + ZSTD_rawLiteralsCost(
                                    ip.offset(cur as isize),
                                    1,
                                    optStatePtr,
                                    optLevel,
                                ) as core::ffi::c_int
                                + (ZSTD_litLengthPrice(
                                    litlen_0.wrapping_add(1),
                                    optStatePtr,
                                    optLevel,
                                ) as core::ffi::c_int
                                    - ZSTD_litLengthPrice(
                                        litlen_0.wrapping_add(1).wrapping_sub(1),
                                        optStatePtr,
                                        optLevel,
                                    ) as core::ffi::c_int);
                            if with1literal < withMoreLiterals
                                && with1literal < (*opt.offset(cur.wrapping_add(1) as isize)).price
                            {
                                let prev = cur.wrapping_sub(prevMatch.mlen);
                                let newReps = ZSTD_newRep(
                                    ((*opt.offset(prev as isize)).rep).as_mut_ptr() as *const u32,
                                    prevMatch.off,
                                    ((*opt.offset(prev as isize)).litlen == 0) as core::ffi::c_int
                                        as u32,
                                );
                                *opt.offset(cur.wrapping_add(1) as isize) = prevMatch;
                                libc::memcpy(
                                    ((*opt.offset(cur.wrapping_add(1) as isize)).rep).as_mut_ptr()
                                        as *mut core::ffi::c_void,
                                    &newReps as *const Repcodes_t as *const core::ffi::c_void,
                                    ::core::mem::size_of::<Repcodes_t>() as core::ffi::c_ulong
                                        as libc::size_t,
                                );
                                (*opt.offset(cur.wrapping_add(1) as isize)).litlen = 1;
                                (*opt.offset(cur.wrapping_add(1) as isize)).price = with1literal;
                                if last_pos < cur.wrapping_add(1) {
                                    last_pos = cur.wrapping_add(1);
                                }
                            }
                        }
                    }
                    if (*opt.offset(cur as isize)).litlen == 0 {
                        let prev_0 = cur.wrapping_sub((*opt.offset(cur as isize)).mlen);
                        let newReps_0 = ZSTD_newRep(
                            ((*opt.offset(prev_0 as isize)).rep).as_mut_ptr() as *const u32,
                            (*opt.offset(cur as isize)).off,
                            ((*opt.offset(prev_0 as isize)).litlen == 0) as core::ffi::c_int as u32,
                        );
                        libc::memcpy(
                            ((*opt.offset(cur as isize)).rep).as_mut_ptr()
                                as *mut core::ffi::c_void,
                            &newReps_0 as *const Repcodes_t as *const core::ffi::c_void,
                            ::core::mem::size_of::<Repcodes_t>() as core::ffi::c_ulong
                                as libc::size_t,
                        );
                    }
                    if inr <= ilimit {
                        if cur == last_pos {
                            current_block = 10357520176418200368;
                            break;
                        }
                        if !(optLevel == 0
                            && (*opt.offset(cur.wrapping_add(1) as isize)).price
                                <= (*opt.offset(cur as isize)).price + BITCOST_MULTIPLIER / 2)
                        {
                            let ll0_0 = ((*opt.offset(cur as isize)).litlen == 0)
                                as core::ffi::c_int as u32;
                            let previousPrice = (*opt.offset(cur as isize)).price;
                            let basePrice = previousPrice
                                + ZSTD_litLengthPrice(0, optStatePtr, optLevel) as core::ffi::c_int;
                            let mut nbMatches_0 = getAllMatches.unwrap_unchecked()(
                                matches,
                                ms,
                                &mut nextToUpdate3,
                                inr,
                                iend,
                                ((*opt.offset(cur as isize)).rep).as_mut_ptr() as *const u32,
                                ll0_0,
                                minMatch,
                            );
                            let mut matchNb_0: u32 = 0;
                            ZSTD_optLdm_processMatchCandidate(
                                &mut optLdm,
                                matches,
                                &mut nbMatches_0,
                                inr.offset_from(istart) as core::ffi::c_long as u32,
                                iend.offset_from(inr) as core::ffi::c_long as u32,
                                minMatch,
                            );
                            if nbMatches_0 != 0 {
                                let longestML =
                                    (*matches.offset(nbMatches_0.wrapping_sub(1) as isize)).len;
                                if longestML > sufficient_len
                                    || cur.wrapping_add(longestML) >= ZSTD_OPT_NUM as u32
                                    || ip.offset(cur as isize).offset(longestML as isize) >= iend
                                {
                                    lastStretch.mlen = longestML;
                                    lastStretch.off =
                                        (*matches.offset(nbMatches_0.wrapping_sub(1) as isize)).off;
                                    lastStretch.litlen = 0;
                                    last_pos = cur.wrapping_add(longestML);
                                    current_block = 12608488225262500095;
                                    break;
                                } else {
                                    matchNb_0 = 0;
                                    while matchNb_0 < nbMatches_0 {
                                        let offset = (*matches.offset(matchNb_0 as isize)).off;
                                        let lastML = (*matches.offset(matchNb_0 as isize)).len;
                                        let startML = if matchNb_0 > 0 {
                                            ((*matches.offset(matchNb_0.wrapping_sub(1) as isize))
                                                .len)
                                                .wrapping_add(1)
                                        } else {
                                            minMatch
                                        };
                                        let mut mlen: u32 = 0;
                                        mlen = lastML;
                                        while mlen >= startML {
                                            let pos_0 = cur.wrapping_add(mlen);
                                            let price_0 = basePrice
                                                + ZSTD_getMatchPrice(
                                                    offset,
                                                    mlen,
                                                    optStatePtr,
                                                    optLevel,
                                                )
                                                    as core::ffi::c_int;
                                            if pos_0 > last_pos
                                                || price_0 < (*opt.offset(pos_0 as isize)).price
                                            {
                                                while last_pos < pos_0 {
                                                    last_pos = last_pos.wrapping_add(1);
                                                    (*opt.offset(last_pos as isize)).price =
                                                        ZSTD_MAX_PRICE;
                                                    (*opt.offset(last_pos as isize)).litlen =
                                                        (0 == 0) as core::ffi::c_int as u32;
                                                }
                                                (*opt.offset(pos_0 as isize)).mlen = mlen;
                                                (*opt.offset(pos_0 as isize)).off = offset;
                                                (*opt.offset(pos_0 as isize)).litlen = 0;
                                                (*opt.offset(pos_0 as isize)).price = price_0;
                                            } else if optLevel == 0 {
                                                break;
                                            }
                                            mlen = mlen.wrapping_sub(1);
                                        }
                                        matchNb_0 = matchNb_0.wrapping_add(1);
                                    }
                                    (*opt.offset(last_pos.wrapping_add(1) as isize)).price =
                                        ZSTD_MAX_PRICE;
                                }
                            }
                        }
                    }
                    cur = cur.wrapping_add(1);
                }
                match current_block {
                    12608488225262500095 => {}
                    _ => {
                        lastStretch = *opt.offset(last_pos as isize);
                        cur = last_pos.wrapping_sub(lastStretch.mlen);
                    }
                }
            }
            if lastStretch.mlen == 0 {
                ip = ip.offset(last_pos as isize);
            } else {
                if lastStretch.litlen == 0 {
                    let reps = ZSTD_newRep(
                        ((*opt.offset(cur as isize)).rep).as_mut_ptr() as *const u32,
                        lastStretch.off,
                        ((*opt.offset(cur as isize)).litlen == 0) as core::ffi::c_int as u32,
                    );
                    libc::memcpy(
                        rep as *mut core::ffi::c_void,
                        &reps as *const Repcodes_t as *const core::ffi::c_void,
                        ::core::mem::size_of::<Repcodes_t>() as core::ffi::c_ulong as libc::size_t,
                    );
                } else {
                    libc::memcpy(
                        rep as *mut core::ffi::c_void,
                        (lastStretch.rep).as_mut_ptr() as *const core::ffi::c_void,
                        ::core::mem::size_of::<Repcodes_t>() as core::ffi::c_ulong as libc::size_t,
                    );
                    cur = cur.wrapping_sub(lastStretch.litlen);
                }
                let storeEnd = cur.wrapping_add(2);
                let mut storeStart = storeEnd;
                let mut stretchPos = cur;
                if lastStretch.litlen > 0 {
                    (*opt.offset(storeEnd as isize)).litlen = lastStretch.litlen;
                    (*opt.offset(storeEnd as isize)).mlen = 0;
                    storeStart = storeEnd.wrapping_sub(1);
                    *opt.offset(storeStart as isize) = lastStretch;
                }
                *opt.offset(storeEnd as isize) = lastStretch;
                storeStart = storeEnd;
                loop {
                    let nextStretch = *opt.offset(stretchPos as isize);
                    (*opt.offset(storeStart as isize)).litlen = nextStretch.litlen;
                    if nextStretch.mlen == 0 {
                        break;
                    }
                    storeStart = storeStart.wrapping_sub(1);
                    *opt.offset(storeStart as isize) = nextStretch;
                    stretchPos = stretchPos
                        .wrapping_sub((nextStretch.litlen).wrapping_add(nextStretch.mlen));
                }
                let mut storePos: u32 = 0;
                storePos = storeStart;
                while storePos <= storeEnd {
                    let llen = (*opt.offset(storePos as isize)).litlen;
                    let mlen_0 = (*opt.offset(storePos as isize)).mlen;
                    let offBase_0 = (*opt.offset(storePos as isize)).off;
                    let advance = llen.wrapping_add(mlen_0);
                    if mlen_0 == 0 {
                        ip = anchor.offset(llen as isize);
                    } else {
                        ZSTD_updateStats(optStatePtr, llen, anchor, offBase_0, mlen_0);
                        ZSTD_storeSeq(
                            seqStore,
                            llen as size_t,
                            anchor,
                            iend,
                            offBase_0,
                            mlen_0 as size_t,
                        );
                        anchor = anchor.offset(advance as isize);
                        ip = anchor;
                    }
                    storePos = storePos.wrapping_add(1);
                }
                ZSTD_setBasePrices(optStatePtr, optLevel);
            }
        }
    }
    iend.offset_from_unsigned(anchor)
}
unsafe fn ZSTD_compressBlock_opt0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize, 0, dictMode)
}
unsafe fn ZSTD_compressBlock_opt2(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    ZSTD_compressBlock_opt_generic(ms, seqStore, rep, src, srcSize, 2, dictMode)
}
pub unsafe fn ZSTD_compressBlock_btopt(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_noDict)
}
unsafe fn ZSTD_initStats_ultra(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    let mut tmpRep: [u32; 3] = [0; 3];
    libc::memcpy(
        tmpRep.as_mut_ptr() as *mut core::ffi::c_void,
        rep as *const core::ffi::c_void,
        ::core::mem::size_of::<[u32; 3]>() as core::ffi::c_ulong as libc::size_t,
    );
    ZSTD_compressBlock_opt2(ms, seqStore, tmpRep.as_mut_ptr(), src, srcSize, ZSTD_noDict);
    ZSTD_resetSeqStore(seqStore);
    ms.window.base = (ms.window.base).offset(-(srcSize as isize));
    ms.window.dictLimit = (ms.window.dictLimit).wrapping_add(srcSize as u32);
    ms.window.lowLimit = ms.window.dictLimit;
    ms.nextToUpdate = ms.window.dictLimit;
}
pub unsafe fn ZSTD_compressBlock_btultra(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_noDict)
}
pub unsafe fn ZSTD_compressBlock_btultra2(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let curr = (src as *const u8).offset_from(ms.window.base) as core::ffi::c_long as u32;
    if ms.opt.litLengthSum == 0
        && seqStore.sequences == seqStore.sequencesStart
        && ms.window.dictLimit == ms.window.lowLimit
        && curr == ms.window.dictLimit
        && srcSize > ZSTD_PREDEF_THRESHOLD as size_t
    {
        ZSTD_initStats_ultra(ms, seqStore, rep, src, srcSize);
    }
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_noDict)
}
pub unsafe fn ZSTD_compressBlock_btopt_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_dictMatchState)
}
pub unsafe fn ZSTD_compressBlock_btopt_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_extDict)
}
pub unsafe fn ZSTD_compressBlock_btultra_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_dictMatchState)
}
pub unsafe fn ZSTD_compressBlock_btultra_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_extDict)
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647;
