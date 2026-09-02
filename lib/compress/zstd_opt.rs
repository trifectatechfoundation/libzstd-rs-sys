pub type ZSTD_getAllMatchesFn = unsafe fn(
    *mut ZSTD_match_t,
    &mut ZSTD_MatchState_t,
    *mut u32,
    *const u8,
    *const u8,
    &[u32; 3],
    u32,
    u32,
) -> u32;

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
use crate::lib::common::fse::{FSE_CState_t, FSE_getMaxNbBits, FSE_initCState};
use crate::lib::common::huf::HUF_repeat_valid;
use crate::lib::common::mem::MEM_read32;
use crate::lib::common::zstd_internal::{
    LL_bits, ML_bits, MaxLL, MaxLit, MaxML, MaxOff, MINMATCH, ZSTD_OPT_NUM, ZSTD_REP_NUM,
};
use crate::lib::compress::hist::HIST_count_simple;
use crate::lib::compress::huf_compress::HUF_getNbBitsFromCTable;
use crate::lib::compress::zstd_compress::{
    rawSeq, RawSeqStore_t, SeqStore_t, ZSTD_MatchState_t, ZSTD_optimal_t, ZSTD_resetSeqStore,
};
use crate::lib::compress::zstd_compress_internal::{
    optState_t, repcodes_s, DictMode, OptPrice, Repcodes_t, ZSTD_count, ZSTD_count_2segments,
    ZSTD_getLowestMatchIndex, ZSTD_hash32Ptr, ZSTD_hashPtr, ZSTD_index_overlap_check, ZSTD_match_t,
    ZSTD_storeSeq, ZSTD_updateRep,
};
use crate::lib::polyfill::PointerExt;
use crate::lib::zstd::{ParamSwitch, ZSTD_compressionParameters, ZSTD_BLOCKSIZE_MAX};

#[inline]
fn ZSTD_LLcode(litLength: u32) -> u32 {
    static LL_Code: [u8; 64] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20,
        20, 20, 20, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23,
        24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    ];
    static LL_deltaCode: u32 = 19;
    if litLength > 63 {
        (ZSTD_highbit32(litLength)).wrapping_add(LL_deltaCode)
    } else {
        LL_Code[litLength as usize] as core::ffi::c_uint
    }
}

#[inline]
fn ZSTD_MLcode(mlBase: u32) -> u32 {
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
        ML_Code[mlBase as usize] as core::ffi::c_uint
    }
}

#[inline]
fn ZSTD_newRep(rep: &[u32; 3], offBase: u32, ll0: u32) -> Repcodes_t {
    let mut newReps = repcodes_s { rep: *rep };
    ZSTD_updateRep(&mut newReps.rep, offBase, ll0);
    newReps
}

pub const UINT_MAX: core::ffi::c_uint = (__INT_MAX__ as core::ffi::c_uint)
    .wrapping_mul(2)
    .wrapping_add(1);

pub const ZSTD_LITFREQ_ADD: core::ffi::c_int = 2;
pub const ZSTD_MAX_PRICE: core::ffi::c_int = 1 << 30;
// if srcSize < ZSTD_PREDEF_THRESHOLD, symbols' cost is assumed static, directly determined by pre-defined distributions
pub const ZSTD_PREDEF_THRESHOLD: core::ffi::c_int = 8;
pub const BITCOST_ACCURACY: core::ffi::c_int = 8;
pub const BITCOST_MULTIPLIER: core::ffi::c_int = 1 << BITCOST_ACCURACY;

// provide estimated "cost" of a stat in full bits only
#[inline]
fn ZSTD_bitWeight(stat: u32) -> u32 {
    (ZSTD_highbit32(stat.wrapping_add(1))).wrapping_mul(BITCOST_MULTIPLIER as core::ffi::c_uint)
}

// provide fractional-bit "cost" of a stat,
// using linear interpolation approximation
#[inline]
fn ZSTD_fracWeight(rawStat: u32) -> u32 {
    let stat = rawStat.wrapping_add(1);
    let hb = ZSTD_highbit32(stat);
    let BWeight = hb * BITCOST_MULTIPLIER as u32;
    // FWeight was meant for "Fractional weight"
    // but it's effectively a value between 1 and 2
    // using fixed point arithmetic
    let FWeight = stat << BITCOST_ACCURACY >> hb;

    BWeight.wrapping_add(FWeight)
}

unsafe fn ZSTD_compressedLiterals(optPtr: *const optState_t) -> bool {
    (*optPtr).literalCompressionMode != ParamSwitch::Disable
}

unsafe fn ZSTD_setBasePrices(optPtr: *mut optState_t, optLevel: core::ffi::c_int) {
    if ZSTD_compressedLiterals(optPtr) {
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
    let mut total = 0;
    for n in 0..nbElts {
        total = (total as core::ffi::c_uint).wrapping_add(*table.add(n));
    }
    total
}

unsafe fn ZSTD_downscaleStats(
    table: *mut core::ffi::c_uint,
    lastEltIndex: u32,
    shift: u32,
    base1: base_directive_e,
) -> u32 {
    let mut sum = 0;
    for s in 0..lastEltIndex.wrapping_add(1) {
        let base = (if base1 != 0 {
            1
        } else {
            (*table.offset(s as isize) > 0) as core::ffi::c_int
        }) as core::ffi::c_uint;
        let newStat = base.wrapping_add(*table.offset(s as isize) >> shift);
        sum = (sum as core::ffi::c_uint).wrapping_add(newStat);
        *table.offset(s as isize) = newStat;
    }
    sum
}

/// Reduce all elt frequencies in table if sum too large.
/// Returns the resulting sum of elements.
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

/// if first block (detected by optPtr->litLengthSum == 0): init statistics
///    take hints from dictionary if there is one
///    and init from zero if there is none,
///    using src for literals stats, and baseline stats for sequence symbols
/// otherwise downscale existing stats, to be used as seed for next block.
unsafe fn ZSTD_rescaleFreqs(
    opt_state: &mut optState_t,
    src: *const u8,
    srcSize: size_t,
    optLevel: core::ffi::c_int,
) {
    let compressedLiterals = ZSTD_compressedLiterals(opt_state);
    opt_state.priceType = OptPrice::Dynamic;

    if opt_state.litLengthSum == 0 {
        // no literals stats collected -> first block assumed -> init

        // heuristic: use pre-defined stats for too small inputs
        if srcSize <= ZSTD_PREDEF_THRESHOLD as size_t {
            opt_state.priceType = OptPrice::Predef;
        }

        if (*opt_state.symbolCosts).huf.repeatMode == HUF_repeat_valid {
            // huffman stats covering the full value set: table presumed generated by dictionary
            opt_state.priceType = OptPrice::Dynamic;

            if compressedLiterals {
                // generate literals statistics from huffman table
                opt_state.litSum = 0;
                for lit in 0..=MaxLit {
                    let scaleLog = 11u32; // scale to 2K
                    let bitCost = HUF_getNbBitsFromCTable(
                        &(*opt_state.symbolCosts).huf.CTable,
                        u32::from(lit),
                    );
                    *(opt_state.litFreq).offset(lit as isize) = (if bitCost != 0 {
                        1 << scaleLog.wrapping_sub(bitCost)
                    } else {
                        1 // minimum to calculate cost
                    })
                        as core::ffi::c_uint;
                    opt_state.litSum = (opt_state.litSum as core::ffi::c_uint)
                        .wrapping_add(*(opt_state.litFreq).offset(lit as isize));
                }
            }

            let mut llstate = FSE_CState_t::default();
            FSE_initCState(&mut llstate, &(*opt_state.symbolCosts).fse.litlengthCTable);
            opt_state.litLengthSum = 0;
            for ll in 0..=MaxLL {
                let scaleLog_0 = 10u32; // scale to 1K
                let bitCost_0 = FSE_getMaxNbBits(llstate.symbolTT, u32::from(ll));
                *(opt_state.litLengthFreq).offset(ll as isize) = (if bitCost_0 != 0 {
                    1 << scaleLog_0.wrapping_sub(bitCost_0)
                } else {
                    1 // minimum to calculate cost
                })
                    as core::ffi::c_uint;
                opt_state.litLengthSum = (opt_state.litLengthSum as core::ffi::c_uint)
                    .wrapping_add(*(opt_state.litLengthFreq).offset(ll as isize));
            }

            let mut mlstate = FSE_CState_t::default();
            FSE_initCState(
                &mut mlstate,
                &(*opt_state.symbolCosts).fse.matchlengthCTable,
            );
            opt_state.matchLengthSum = 0;
            for ml in 0..=MaxML {
                let scaleLog_1 = 10u32;
                let bitCost_1 = FSE_getMaxNbBits(mlstate.symbolTT, u32::from(ml));
                *(opt_state.matchLengthFreq).offset(ml as isize) = (if bitCost_1 != 0 {
                    1 << scaleLog_1.wrapping_sub(bitCost_1)
                } else {
                    1 // minimum to calculate cost
                })
                    as core::ffi::c_uint;
                opt_state.matchLengthSum = (opt_state.matchLengthSum as core::ffi::c_uint)
                    .wrapping_add(*(opt_state.matchLengthFreq).offset(ml as isize));
            }

            let mut ofstate = FSE_CState_t::default();
            FSE_initCState(&mut ofstate, &(*opt_state.symbolCosts).fse.offcodeCTable);
            opt_state.offCodeSum = 0;
            for of in 0..=MaxOff {
                let scaleLog_2 = 10u32;
                let bitCost_2 = FSE_getMaxNbBits(ofstate.symbolTT, u32::from(of));
                *(opt_state.offCodeFreq).offset(of as isize) = (if bitCost_2 != 0 {
                    1 << scaleLog_2.wrapping_sub(bitCost_2)
                } else {
                    1 // minimum to calculate cost
                })
                    as core::ffi::c_uint;
                opt_state.offCodeSum = (opt_state.offCodeSum as core::ffi::c_uint)
                    .wrapping_add(*(opt_state.offCodeFreq).offset(of as isize));
            }
        } else {
            // first block, no dictionary
            if compressedLiterals {
                // base initial cost of literals on direct frequency within src
                let mut lit_0 = MaxLit;
                HIST_count_simple(
                    opt_state.litFreq,
                    &mut lit_0,
                    src as *const core::ffi::c_void,
                    srcSize, // use raw first block to init statistics
                );
                opt_state.litSum =
                    ZSTD_downscaleStats(opt_state.litFreq, u32::from(MaxLit), 8, base_0possible);
            }

            let baseLLfreqs: [core::ffi::c_uint; 36] = [
                4, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1,
            ];
            core::ptr::copy_nonoverlapping(
                baseLLfreqs.as_ptr(),
                opt_state.litLengthFreq,
                baseLLfreqs.len(),
            );
            opt_state.litLengthSum = sum_u32(baseLLfreqs.as_ptr(), usize::from(MaxLL) + 1);

            for ml_0 in 0..MaxML + 1 {
                *(opt_state.matchLengthFreq).offset(ml_0 as isize) = 1;
            }
            opt_state.matchLengthSum = u32::from(MaxML) + 1;

            let baseOFCfreqs: [core::ffi::c_uint; 32] = [
                6, 2, 1, 1, 2, 3, 4, 4, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1,
            ];
            core::ptr::copy_nonoverlapping(
                baseOFCfreqs.as_ptr(),
                opt_state.offCodeFreq,
                baseOFCfreqs.len(),
            );
            opt_state.offCodeSum = sum_u32(baseOFCfreqs.as_ptr(), usize::from(MaxOff) + 1);
        }
    } else {
        // new block: scale down accumulated statistics
        if compressedLiterals {
            opt_state.litSum = ZSTD_scaleStats(opt_state.litFreq, u32::from(MaxLit), 12);
        }
        opt_state.litLengthSum = ZSTD_scaleStats(opt_state.litLengthFreq, u32::from(MaxLL), 11);
        opt_state.matchLengthSum = ZSTD_scaleStats(opt_state.matchLengthFreq, u32::from(MaxML), 11);
        opt_state.offCodeSum = ZSTD_scaleStats(opt_state.offCodeFreq, u32::from(MaxOff), 11);
    }

    ZSTD_setBasePrices(opt_state, optLevel);
}

/// price of literals (only) in specified segment (which length can be 0).
/// does not include price of literalLength symbol
unsafe fn ZSTD_rawLiteralsCost(
    literals: *const u8,
    litLength: u32,
    optPtr: &optState_t,
    optLevel: core::ffi::c_int,
) -> u32 {
    if litLength == 0 {
        return 0;
    }

    if !ZSTD_compressedLiterals(optPtr) {
        return (litLength << 3) * BITCOST_MULTIPLIER as u32; // Uncompressed - 8 bytes per literal.
    }

    if optPtr.priceType == OptPrice::Predef {
        return litLength * 6 * BITCOST_MULTIPLIER as u32; // 6 bit per literal - no statistic used
    }

    // dynamic statistics
    let mut price = optPtr.litSumBasePrice * litLength;
    let litPriceMax = (optPtr.litSumBasePrice).wrapping_sub(BITCOST_MULTIPLIER as u32);
    for u in 0..litLength {
        let mut litPrice = if optLevel != 0 {
            ZSTD_fracWeight(*(optPtr.litFreq).offset(*literals.offset(u as isize) as isize))
        } else {
            ZSTD_bitWeight(*(optPtr.litFreq).offset(*literals.offset(u as isize) as isize))
        };
        if litPrice > litPriceMax {
            litPrice = litPriceMax;
        }
        price = price.wrapping_sub(litPrice);
    }

    price
}

/// Cost of literalLength symbol
unsafe fn ZSTD_litLengthPrice(
    litLength: u32,
    optPtr: &optState_t,
    optLevel: core::ffi::c_int,
) -> u32 {
    if optPtr.priceType == OptPrice::Predef {
        return if optLevel != 0 {
            ZSTD_fracWeight(litLength)
        } else {
            ZSTD_bitWeight(litLength)
        };
    }

    // ZSTD_LLcode() can't compute litLength price for sizes >= ZSTD_BLOCKSIZE_MAX
    // because it isn't representable in the zstd format.
    // So instead just pretend it would cost 1 bit more than ZSTD_BLOCKSIZE_MAX - 1.
    // In such a case, the block would be all literals.
    if litLength == ZSTD_BLOCKSIZE_MAX as u32 {
        return (BITCOST_MULTIPLIER as u32).wrapping_add(ZSTD_litLengthPrice(
            (ZSTD_BLOCKSIZE_MAX - 1) as u32,
            optPtr,
            optLevel,
        ));
    }

    // dynamic statistics
    let llCode = ZSTD_LLcode(litLength);
    ((LL_bits[llCode as usize] as core::ffi::c_int * BITCOST_MULTIPLIER) as u32)
        .wrapping_add(optPtr.litLengthSumBasePrice)
        .wrapping_sub(if optLevel != 0 {
            ZSTD_fracWeight(*(optPtr.litLengthFreq).offset(llCode as isize))
        } else {
            ZSTD_bitWeight(*(optPtr.litLengthFreq).offset(llCode as isize))
        })
}

/// Provides the cost of the match part (offset + matchLength) of a sequence.
/// Must be combined with ZSTD_fullLiteralsCost() to get the full cost of a sequence.
/// @offBase: sumtype, representing an offset or a repcode, and using numeric representation of ZSTD_storeSeq()
/// @optLevel: when <2, favors small offset for decompression speed (improved cache efficiency)
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

    if (*optPtr).priceType == OptPrice::Predef {
        // fixed scheme, does not use statistics
        return (if optLevel != 0 {
            ZSTD_fracWeight(mlBase)
        } else {
            ZSTD_bitWeight(mlBase)
        })
        .wrapping_add(16u32.wrapping_add(offCode) * BITCOST_MULTIPLIER as u32); // emulated offset cost
    }

    // dynamic statistics
    price = (offCode * BITCOST_MULTIPLIER as u32).wrapping_add(
        ((*optPtr).offCodeSumBasePrice).wrapping_sub(if optLevel != 0 {
            ZSTD_fracWeight(*((*optPtr).offCodeFreq).offset(offCode as isize))
        } else {
            ZSTD_bitWeight(*((*optPtr).offCodeFreq).offset(offCode as isize))
        }),
    );
    if optLevel < 2 && offCode >= 20 {
        // handicap for long distance offsets, favor decompression speed
        price = price.wrapping_add(offCode.wrapping_sub(19) * 2 * BITCOST_MULTIPLIER as u32);
    }

    // match Length
    let mlCode = ZSTD_MLcode(mlBase);
    price = price.wrapping_add(
        ((ML_bits[mlCode as usize] as core::ffi::c_int * BITCOST_MULTIPLIER) as u32).wrapping_add(
            ((*optPtr).matchLengthSumBasePrice).wrapping_sub(if optLevel != 0 {
                ZSTD_fracWeight(*((*optPtr).matchLengthFreq).offset(mlCode as isize))
            } else {
                ZSTD_bitWeight(*((*optPtr).matchLengthFreq).offset(mlCode as isize))
            }),
        ),
    );

    price = price.wrapping_add((BITCOST_MULTIPLIER / 5) as u32); // heuristic: make matches a bit more costly to favor less sequences -> faster decompression speed

    price
}

/// assumption: literals + litLength <= iend
unsafe fn ZSTD_updateStats(
    optPtr: *mut optState_t,
    litLength: u32,
    literals: *const u8,
    offBase: u32,
    matchLength: u32,
) {
    // literals
    if ZSTD_compressedLiterals(optPtr) {
        for u in 0..litLength {
            let fresh2 = &mut (*((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize));
            *fresh2 = (*fresh2).wrapping_add(ZSTD_LITFREQ_ADD as core::ffi::c_uint);
        }
        (*optPtr).litSum = ((*optPtr).litSum).wrapping_add(litLength * ZSTD_LITFREQ_ADD as u32);
    }

    // literal Length
    let llCode = ZSTD_LLcode(litLength);
    let fresh3 = &mut (*((*optPtr).litLengthFreq).offset(llCode as isize));
    *fresh3 = (*fresh3).wrapping_add(1);
    (*optPtr).litLengthSum = ((*optPtr).litLengthSum).wrapping_add(1);

    // offset code: follows storeSeq() numeric representation
    let offCode = ZSTD_highbit32(offBase);
    let fresh4 = &mut (*((*optPtr).offCodeFreq).offset(offCode as isize));
    *fresh4 = (*fresh4).wrapping_add(1);
    (*optPtr).offCodeSum = ((*optPtr).offCodeSum).wrapping_add(1);

    // match Length
    let mlBase = matchLength.wrapping_sub(MINMATCH as u32);
    let mlCode = ZSTD_MLcode(mlBase);
    let fresh5 = &mut (*((*optPtr).matchLengthFreq).offset(mlCode as isize));
    *fresh5 = (*fresh5).wrapping_add(1);
    (*optPtr).matchLengthSum = ((*optPtr).matchLengthSum).wrapping_add(1);
}

// function safe only for comparisons
// assumption: memPtr must be at least 4 bytes before end of buffer
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

// Update hashTable3 up to ip (excluded)
// Assumption: always within prefix (i.e. not within extDict)
unsafe fn ZSTD_insertAndFindFirstIndexHash3(
    ms: &ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
) -> u32 {
    let hashTable3 = ms.hashTable3;
    let hashLog3 = ms.hashLog3;
    let base = ms.window.base;
    let target = ip.offset_from(base) as core::ffi::c_long as u32;
    let hash3 = ZSTD_hash32Ptr::<3>(ip as *const core::ffi::c_void, hashLog3);

    for idx in *nextToUpdate3..target {
        *hashTable3.add(ZSTD_hash32Ptr::<3>(
            base.offset(idx as isize) as *const core::ffi::c_void,
            hashLog3,
        )) = idx;
    }

    *nextToUpdate3 = target;
    *hashTable3.add(hash3)
}

/// Add one or multiple positions to tree.
///
/// @param ip assumed <= iend-8 .
/// @param target The target of ZSTD_updateTree_internal() - we are filling to this position
///
/// # Returns
///
/// The number of positions added
unsafe fn ZSTD_insertBt1(
    ms: &ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    target: u32,
    mls: u32,
    extDict: bool,
) -> u32 {
    let cParams: *const ZSTD_compressionParameters = &ms.cParams;
    let hashTable = ms.hashTable;
    let hashLog = (*cParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const core::ffi::c_void, hashLog, mls);
    let bt = ms.chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1);
    let btMask = ((1 << btLog) - 1) as u32;
    let mut matchIndex = *hashTable.add(h);
    let mut commonLengthSmaller = 0;
    let mut commonLengthLarger = 0;
    let base = ms.window.base;
    let dictBase = ms.window.dictBase;
    let dictLimit = ms.window.dictLimit;
    let dictEnd = dictBase.wrapping_offset(dictLimit as isize);
    let prefixStart = base.wrapping_offset(dictLimit as isize);
    let mut match_0 = core::ptr::null::<u8>();
    let curr = ip.wrapping_offset_from(base) as core::ffi::c_long as u32;
    let btLow = if btMask >= curr {
        0
    } else {
        curr.wrapping_sub(btMask)
    };
    let mut smallerPtr = bt.offset((2 * (curr & btMask)) as isize);
    let mut largerPtr = smallerPtr.add(1);
    let mut dummy32: u32 = 0;
    // windowLow is based on target because
    // we only need positions that will be in the window at the end of the tree update.
    let windowLow = ZSTD_getLowestMatchIndex(ms, target, (*cParams).windowLog);
    let mut matchEndIdx = curr.wrapping_add(8).wrapping_add(1);
    let mut bestLength = 8;
    let mut nbCompares = (1 as core::ffi::c_uint) << (*cParams).searchLog;

    *hashTable.add(h) = curr; // Update Hash Table

    while nbCompares != 0 && matchIndex >= windowLow {
        let nextPtr = bt.offset((2 * (matchIndex & btMask)) as isize);
        // guaranteed minimum nb of common bytes
        let mut matchLength = commonLengthSmaller.min(commonLengthLarger);

        if !extDict || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
            match_0 = base.wrapping_offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.add(matchLength),
                match_0.add(matchLength),
                iend,
            ));
        } else {
            match_0 = dictBase.wrapping_offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.add(matchLength),
                match_0.add(matchLength),
                iend,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                // to prepare for next usage of match[matchLength]
                match_0 = base.wrapping_offset(matchIndex as isize);
            }
        }

        if matchLength > bestLength {
            bestLength = matchLength;
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                matchEndIdx = matchIndex.wrapping_add(matchLength as u32);
            }
        }

        // equal: no way to know if inf or sup
        if ip.add(matchLength) == iend {
            break; // drop, to guarantee consistency; miss a bit of compression, but other solutions can corrupt tree
        } else {
            if (*match_0.add(matchLength) as core::ffi::c_int)
                < *ip.add(matchLength) as core::ffi::c_int
            {
                // match is smaller than current
                *smallerPtr = matchIndex; // update smaller idx
                commonLengthSmaller = matchLength; // all smaller will now have at least this guaranteed common length
                if matchIndex <= btLow {
                    smallerPtr = &mut dummy32;
                    break; // beyond tree size, stop searching
                } else {
                    smallerPtr = nextPtr.add(1); // new "candidate" => larger than match, which was smaller than target
                    matchIndex = *nextPtr.add(1); // new matchIndex, larger than previous and closer to current
                }
            } else {
                // match is larger than current
                *largerPtr = matchIndex;
                commonLengthLarger = matchLength;
                if matchIndex <= btLow {
                    largerPtr = &mut dummy32;
                    break; // beyond tree size, stop searching
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
        // speed optimization
        positions = (bestLength.wrapping_sub(384) as u32).min(192);
    }
    positions.max(matchEndIdx.wrapping_sub(curr.wrapping_add(8)))
}

#[inline(always)]
unsafe fn ZSTD_updateTree_internal(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    mls: u32,
    dictMode: DictMode,
) {
    let base = ms.window.base;
    let target = ip.wrapping_offset_from(base) as core::ffi::c_long as u32;
    let mut idx = ms.nextToUpdate;

    while idx < target {
        let forward = ZSTD_insertBt1(
            ms,
            base.wrapping_offset(idx as isize),
            iend,
            target,
            mls,
            dictMode == DictMode::ExtDict,
        );
        idx = idx.wrapping_add(forward);
    }

    ms.nextToUpdate = target;
}

pub unsafe fn ZSTD_updateTree(ms: &mut ZSTD_MatchState_t, ip: *const u8, iend: *const u8) {
    ZSTD_updateTree_internal(ms, ip, iend, ms.cParams.minMatch, DictMode::NoDict);
}

#[inline(always)]
unsafe fn ZSTD_insertBtAndGetAllMatches(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iLimit: *const u8,
    dictMode: DictMode,
    rep: &[u32; 3],
    ll0: u32,
    lengthToBeat: u32,
    mls: u32,
) -> u32 {
    let cParams = &ms.cParams;
    let sufficient_len = cParams
        .targetLength
        .min(((1 << 12) - 1) as core::ffi::c_uint);
    let base = ms.window.base;
    let curr = ip.wrapping_offset_from(base) as core::ffi::c_long as u32;
    let hashLog = cParams.hashLog;
    let minMatch = (if mls == 3 { 3 } else { 4 }) as u32;
    let hashTable = ms.hashTable;
    let h = ZSTD_hashPtr(ip as *const core::ffi::c_void, hashLog, mls);
    let mut matchIndex = *hashTable.add(h);
    let bt = ms.chainTable;
    let btLog = (cParams.chainLog).wrapping_sub(1 as core::ffi::c_uint);
    let btMask = ((1 as core::ffi::c_uint) << btLog).wrapping_sub(1);
    let mut commonLengthSmaller = 0;
    let mut commonLengthLarger = 0;
    let dictBase = ms.window.dictBase;
    let dictLimit = ms.window.dictLimit;
    let dictEnd = dictBase.wrapping_offset(dictLimit as isize);
    let prefixStart = base.wrapping_offset(dictLimit as isize);
    let btLow = if btMask >= curr {
        0
    } else {
        curr.wrapping_sub(btMask)
    };
    let windowLow = ZSTD_getLowestMatchIndex(ms, curr, cParams.windowLog);
    let matchLow = if windowLow != 0 { windowLow } else { 1 };
    let mut smallerPtr = bt.offset((2 * (curr & btMask)) as isize);
    let mut largerPtr = bt.offset((2 * (curr & btMask)) as isize).add(1);
    let mut matchEndIdx = curr.wrapping_add(8).wrapping_add(1); // farthest referenced position of any match => detects repetitive patterns
    let mut dummy32: u32 = 0;
    let mut mnum = 0u32;
    let mut nbCompares = (1 as core::ffi::c_uint) << cParams.searchLog;

    let dms = if dictMode == DictMode::DictMatchState {
        ms.dictMatchState
    } else {
        core::ptr::null()
    };
    let dmsCParams = if dictMode == DictMode::DictMatchState {
        &(*dms).cParams
    } else {
        core::ptr::null()
    };
    let dmsBase = if dictMode == DictMode::DictMatchState {
        (*dms).window.base
    } else {
        core::ptr::null()
    };
    let dmsEnd = if dictMode == DictMode::DictMatchState {
        (*dms).window.nextSrc
    } else {
        core::ptr::null()
    };
    let dmsHighLimit = if dictMode == DictMode::DictMatchState {
        dmsEnd.offset_from(dmsBase) as core::ffi::c_long as u32
    } else {
        0
    };
    let dmsLowLimit = if dictMode == DictMode::DictMatchState {
        (*dms).window.lowLimit
    } else {
        0
    };
    let dmsIndexDelta = if dictMode == DictMode::DictMatchState {
        windowLow.wrapping_sub(dmsHighLimit)
    } else {
        0
    };
    let dmsHashLog = if dictMode == DictMode::DictMatchState {
        (*dmsCParams).hashLog
    } else {
        hashLog
    };
    let dmsBtLog = if dictMode == DictMode::DictMatchState {
        ((*dmsCParams).chainLog).wrapping_sub(1)
    } else {
        btLog
    };
    let dmsBtMask = if dictMode == DictMode::DictMatchState {
        ((1 as core::ffi::c_uint) << dmsBtLog).wrapping_sub(1)
    } else {
        0
    };
    let dmsBtLow = if dictMode == DictMode::DictMatchState
        && dmsBtMask < dmsHighLimit.wrapping_sub(dmsLowLimit)
    {
        dmsHighLimit.wrapping_sub(dmsBtMask)
    } else {
        dmsLowLimit
    };

    let mut bestLength = lengthToBeat.wrapping_sub(1) as size_t;

    // check repCode
    let lastR = (ZSTD_REP_NUM as u32).wrapping_add(ll0);
    let mut repCode: u32 = 0;
    repCode = ll0;
    while repCode < lastR {
        let repOffset = if repCode == ZSTD_REP_NUM as u32 {
            rep[0].wrapping_sub(1)
        } else {
            rep[repCode as usize]
        };
        let repIndex = curr.wrapping_sub(repOffset);
        let mut repLen = 0;
        // intentional overflow, discards 0 and -1, equivalent to `curr > repIndex >= dictLimit
        if repOffset.wrapping_sub(1) < curr.wrapping_sub(dictLimit) {
            // We must validate the repcode offset because when we're using a dictionary the
            // valid offset range shrinks when the dictionary goes out of bounds.
            if (repIndex >= windowLow) as core::ffi::c_int
                & (ZSTD_readMINMATCH(ip as *const core::ffi::c_void, minMatch)
                    == ZSTD_readMINMATCH(
                        ip.sub(repOffset as usize) as *const core::ffi::c_void,
                        minMatch,
                    )) as core::ffi::c_int
                != 0
            {
                repLen = (ZSTD_count(
                    ip.offset(minMatch as isize),
                    ip.offset(minMatch as isize).sub(repOffset as usize),
                    iLimit,
                ) as u32)
                    .wrapping_add(minMatch);
            }
        } else {
            let repMatch = if dictMode == DictMode::DictMatchState {
                dmsBase
                    .offset(repIndex as isize)
                    .sub(dmsIndexDelta as usize)
            } else {
                dictBase.wrapping_offset(repIndex as isize)
            };

            // intentional overflow, equivalent to `curr > repIndex >= windowLow`
            if dictMode == DictMode::ExtDict
                && (repOffset.wrapping_sub(1) < curr.wrapping_sub(windowLow))
                    & ZSTD_index_overlap_check(dictLimit, repIndex)
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
            // intentional overflow, equivalent to `curr > repIndex >= dmsLowLimit`
            if dictMode == DictMode::DictMatchState
                && (repOffset.wrapping_sub(1)
                    < curr.wrapping_sub(dmsLowLimit.wrapping_add(dmsIndexDelta)))
                    & ZSTD_index_overlap_check(dictLimit, repIndex)
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

        // save longer solution
        if repLen as size_t > bestLength {
            bestLength = repLen as size_t;
            (*matches.offset(mnum as isize)).off = repCode.wrapping_sub(ll0).wrapping_add(1); // expect value between 1 and 3
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

    // HC3 match finder
    if mls == 3 && bestLength < mls as size_t {
        let matchIndex3 = ZSTD_insertAndFindFirstIndexHash3(ms, nextToUpdate3, ip);
        // heuristic: longer distance likely too expensive
        if (matchIndex3 >= matchLow) as core::ffi::c_int
            & (curr.wrapping_sub(matchIndex3) < (1 << 18) as u32) as core::ffi::c_int
            != 0
        {
            let mut mlen: size_t = 0;
            if dictMode == DictMode::NoDict
                || dictMode == DictMode::DictMatchState
                || matchIndex3 >= dictLimit
            {
                let match_0 = base.offset(matchIndex3 as isize);
                mlen = ZSTD_count(ip, match_0, iLimit);
            } else {
                let match_1 = dictBase.offset(matchIndex3 as isize);
                mlen = ZSTD_count_2segments(ip, match_1, iLimit, dictEnd, prefixStart);
            }

            // save best solution
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
                    ms.nextToUpdate = curr.wrapping_add(1); // skip insertion
                    return 1;
                }
            }
        }

        // no dictMatchState lookup: dicts don't have a populated HC3 table
    }

    *hashTable.add(h) = curr; // Update Hash Table

    while nbCompares != 0 && matchIndex >= matchLow {
        let nextPtr = bt.offset((2 * (matchIndex & btMask)) as isize);
        let mut match_2 = core::ptr::null::<u8>();
        // guaranteed minimum nb of common bytes
        let mut matchLength = commonLengthSmaller.min(commonLengthLarger);

        if dictMode == DictMode::NoDict
            || dictMode == DictMode::DictMatchState
            || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
        {
            match_2 = base.wrapping_offset(matchIndex as isize);
            if matchIndex >= dictLimit {
                // ensure early section of match is equal as expected
                debug_assert!(libc::memcmp(match_2.cast(), ip.cast(), matchLength) == 0);
            }
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.add(matchLength),
                match_2.add(matchLength),
                iLimit,
            ));
        } else {
            match_2 = dictBase.wrapping_offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.add(matchLength),
                match_2.add(matchLength),
                iLimit,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                // prepare for match[matchLength] read
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
            // equal: no way to know if inf or sup
            if (matchLength > ZSTD_OPT_NUM as size_t) as core::ffi::c_int
                | (ip.add(matchLength) == iLimit) as core::ffi::c_int
                != 0
            {
                if dictMode == DictMode::DictMatchState {
                    nbCompares = 0; // break should also skip searching dms
                }
                break; // drop, to preserve bt consistency (miss a little bit of compression)
            }
        }

        if (*match_2.add(matchLength) as core::ffi::c_int)
            < *ip.add(matchLength) as core::ffi::c_int
        {
            // match smaller than current
            *smallerPtr = matchIndex; // update smaller idx
            commonLengthSmaller = matchLength; // all smaller will now have at least this guaranteed common length
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32;
                break; // beyond tree size, stop the search
            } else {
                smallerPtr = nextPtr.add(1); // new candidate => larger than match, which was smaller than current
                matchIndex = *nextPtr.add(1); // new matchIndex, larger than previous, closer to current
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32;
                break; // beyond tree size, stop the search
            } else {
                largerPtr = nextPtr;
                matchIndex = *nextPtr;
            }
        }

        nbCompares = nbCompares.wrapping_sub(1);
    }

    *largerPtr = 0;
    *smallerPtr = *largerPtr;

    if dictMode == DictMode::DictMatchState && nbCompares != 0 {
        let dmsH = ZSTD_hashPtr(ip as *const core::ffi::c_void, dmsHashLog, mls);
        let mut dictMatchIndex = *((*dms).hashTable).add(dmsH);
        let dmsBt: *const u32 = (*dms).chainTable;
        commonLengthLarger = 0;
        commonLengthSmaller = commonLengthLarger;
        while nbCompares != 0 && dictMatchIndex > dmsLowLimit {
            let nextPtr_0 = dmsBt.offset((2 * (dictMatchIndex & dmsBtMask)) as isize);
            let mut matchLength_0 = commonLengthSmaller.min(commonLengthLarger); // guaranteed minimum number of common bytes
            let mut match_3 = dmsBase.offset(dictMatchIndex as isize);
            matchLength_0 = matchLength_0.wrapping_add(ZSTD_count_2segments(
                ip.add(matchLength_0),
                match_3.add(matchLength_0),
                iLimit,
                dmsEnd,
                prefixStart,
            ));
            if (dictMatchIndex as size_t).wrapping_add(matchLength_0) >= dmsHighLimit as size_t {
                // to prepare for next usage of match[matchLength]
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
                // equal: no way to know if inf or sup
                if (matchLength_0 > ZSTD_OPT_NUM as size_t) as core::ffi::c_int
                    | (ip.add(matchLength_0) == iLimit) as core::ffi::c_int
                    != 0
                {
                    break; // drop, to guarantee consistency (miss a little bit of compression)
                }
            }

            if dictMatchIndex <= dmsBtLow {
                break; // beyond tree size, stop the search
            }
            if (*match_3.add(matchLength_0) as core::ffi::c_int)
                < *ip.add(matchLength_0) as core::ffi::c_int
            {
                commonLengthSmaller = matchLength_0; // all smaller will now have at least this guaranteed common length
                dictMatchIndex = *nextPtr_0.add(1); // new matchIndex larger than previous (closer to current)
            } else {
                // match is larger than current
                commonLengthLarger = matchLength_0;
                dictMatchIndex = *nextPtr_0;
            }
            nbCompares = nbCompares.wrapping_sub(1);
        }
    }

    ms.nextToUpdate = matchEndIdx.wrapping_sub(8); // skip repetitive patterns

    mnum
}

#[inline(always)]
unsafe fn ZSTD_btGetAllMatches_internal<const MLS: u32>(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: &[u32; 3],
    ll0: u32,
    lengthToBeat: u32,
    dictMode: DictMode,
) -> u32 {
    if ip < (ms.window.base).wrapping_offset(ms.nextToUpdate as isize) {
        return 0; // skipped area
    }
    ZSTD_updateTree_internal(ms, ip, iHighLimit, MLS, dictMode);
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
        MLS,
    )
}

unsafe fn ZSTD_btGetAllMatches_noDict<const MLS: u32>(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: &[u32; 3],
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal::<MLS>(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        DictMode::NoDict,
    )
}

unsafe fn ZSTD_btGetAllMatches_extDict<const MLS: u32>(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: &[u32; 3],
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal::<MLS>(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        DictMode::ExtDict,
    )
}

unsafe fn ZSTD_btGetAllMatches_dictMatchState<const MLS: u32>(
    matches: *mut ZSTD_match_t,
    ms: &mut ZSTD_MatchState_t,
    nextToUpdate3: *mut u32,
    ip: *const u8,
    iHighLimit: *const u8,
    rep: &[u32; 3],
    ll0: u32,
    lengthToBeat: u32,
) -> u32 {
    ZSTD_btGetAllMatches_internal::<MLS>(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        DictMode::DictMatchState,
    )
}

fn ZSTD_selectBtGetAllMatches(ms: &ZSTD_MatchState_t, dictMode: DictMode) -> ZSTD_getAllMatchesFn {
    let getAllMatchesFns: [[ZSTD_getAllMatchesFn; 4]; 3] = [
        [
            ZSTD_btGetAllMatches_noDict::<3>,
            ZSTD_btGetAllMatches_noDict::<4>,
            ZSTD_btGetAllMatches_noDict::<5>,
            ZSTD_btGetAllMatches_noDict::<6>,
        ],
        [
            ZSTD_btGetAllMatches_extDict::<3>,
            ZSTD_btGetAllMatches_extDict::<4>,
            ZSTD_btGetAllMatches_extDict::<5>,
            ZSTD_btGetAllMatches_extDict::<6>,
        ],
        [
            ZSTD_btGetAllMatches_dictMatchState::<3>,
            ZSTD_btGetAllMatches_dictMatchState::<4>,
            ZSTD_btGetAllMatches_dictMatchState::<5>,
            ZSTD_btGetAllMatches_dictMatchState::<6>,
        ],
    ];
    let mls = ms.cParams.minMatch.clamp(3, 6);
    getAllMatchesFns[dictMode as usize][mls.wrapping_sub(3) as usize]
}

/// Moves forward in @rawSeqStore by @nbBytes,
/// which will update the fields 'pos' and 'posInSequence'.
unsafe fn ZSTD_optLdm_skipRawSeqStoreBytes(rawSeqStore: &mut RawSeqStore_t, nbBytes: size_t) {
    let mut currPos = (rawSeqStore.posInSequence).wrapping_add(nbBytes) as u32;
    while currPos != 0 && rawSeqStore.pos < rawSeqStore.size {
        let currSeq = *(rawSeqStore.seq).add(rawSeqStore.pos);
        if currPos >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            currPos = currPos.wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength));
            rawSeqStore.pos = (rawSeqStore.pos).wrapping_add(1);
        } else {
            rawSeqStore.posInSequence = currPos as size_t;
            break;
        }
    }
    if currPos == 0 || rawSeqStore.pos == rawSeqStore.size {
        rawSeqStore.posInSequence = 0;
    }
}

/// Calculates the beginning and end of the next match in the current block.
/// Updates 'pos' and 'posInSequence' of the ldmSeqStore.
unsafe fn ZSTD_opt_getNextMatchAndUpdateSeqStore(
    optLdm: &mut ZSTD_optLdm_t,
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

    // Setting match end position to MAX to ensure we never use an LDM during this block
    if optLdm.seqStore.size == 0 || optLdm.seqStore.pos >= optLdm.seqStore.size {
        optLdm.startPosInBlock = UINT_MAX;
        optLdm.endPosInBlock = UINT_MAX;
        return;
    }
    // Calculate appropriate bytes left in matchLength and litLength
    // after adjusting based on ldmSeqStore->posInSequence
    currSeq = *(optLdm.seqStore.seq).add(optLdm.seqStore.pos);
    currBlockEndPos = currPosInBlock.wrapping_add(blockBytesRemaining);
    literalsBytesRemaining = if optLdm.seqStore.posInSequence < currSeq.litLength as size_t {
        (currSeq.litLength).wrapping_sub(optLdm.seqStore.posInSequence as u32)
    } else {
        0
    };
    matchBytesRemaining = if literalsBytesRemaining == 0 {
        (currSeq.matchLength)
            .wrapping_sub((optLdm.seqStore.posInSequence as u32).wrapping_sub(currSeq.litLength))
    } else {
        currSeq.matchLength
    };

    // If there are more literal bytes than bytes remaining in block, no ldm is possible
    if literalsBytesRemaining >= blockBytesRemaining {
        optLdm.startPosInBlock = UINT_MAX;
        optLdm.endPosInBlock = UINT_MAX;
        ZSTD_optLdm_skipRawSeqStoreBytes(&mut optLdm.seqStore, blockBytesRemaining as size_t);
        return;
    }

    // Matches may be < minMatch by this process. In that case, we will reject them
    // when we are deciding whether or not to add the ldm
    optLdm.startPosInBlock = currPosInBlock.wrapping_add(literalsBytesRemaining);
    optLdm.endPosInBlock = (optLdm.startPosInBlock).wrapping_add(matchBytesRemaining);
    optLdm.offset = currSeq.offset;

    if optLdm.endPosInBlock > currBlockEndPos {
        // Match ends after the block ends, we can't use the whole match
        optLdm.endPosInBlock = currBlockEndPos;
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut optLdm.seqStore,
            currBlockEndPos.wrapping_sub(currPosInBlock) as size_t,
        );
    } else {
        // Consume number of bytes equal to size of sequence left
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut optLdm.seqStore,
            literalsBytesRemaining.wrapping_add(matchBytesRemaining) as size_t,
        );
    }
}

/// Adds a match if it's long enough,
/// based on it's 'matchStartPosInBlock' and 'matchEndPosInBlock',
/// into 'matches'. Maintains the correct ordering of 'matches'.
unsafe fn ZSTD_optLdm_maybeAddMatch(
    matches: *mut ZSTD_match_t,
    nbMatches: &mut u32,
    optLdm: &ZSTD_optLdm_t,
    currPosInBlock: u32,
    minMatch: u32,
) {
    let posDiff = currPosInBlock.wrapping_sub(optLdm.startPosInBlock);
    // Note: ZSTD_match_t actually contains offBase and matchLength (before subtracting MINMATCH)
    let candidateMatchLength = (optLdm.endPosInBlock)
        .wrapping_sub(optLdm.startPosInBlock)
        .wrapping_sub(posDiff);

    // Ensure that current block position is not outside of the match
    if currPosInBlock < optLdm.startPosInBlock
        || currPosInBlock >= optLdm.endPosInBlock
        || candidateMatchLength < minMatch
    {
        return;
    }

    if *nbMatches == 0
        || candidateMatchLength > (*matches.offset((*nbMatches).wrapping_sub(1) as isize)).len
            && *nbMatches < ZSTD_OPT_NUM as u32
    {
        let candidateOffBase = (optLdm.offset).wrapping_add(ZSTD_REP_NUM as u32);
        (*matches.offset(*nbMatches as isize)).len = candidateMatchLength;
        (*matches.offset(*nbMatches as isize)).off = candidateOffBase;
        *nbMatches = (*nbMatches).wrapping_add(1);
    }
}

/// Wrapper function to update ldm seq store and call ldm functions as necessary.
unsafe fn ZSTD_optLdm_processMatchCandidate(
    optLdm: &mut ZSTD_optLdm_t,
    matches: *mut ZSTD_match_t,
    nbMatches: &mut u32,
    currPosInBlock: u32,
    remainingBytes: u32,
    minMatch: u32,
) {
    if optLdm.seqStore.size == 0 || optLdm.seqStore.pos >= optLdm.seqStore.size {
        return;
    }

    if currPosInBlock >= optLdm.endPosInBlock {
        if currPosInBlock > optLdm.endPosInBlock {
            // The position at which ZSTD_optLdm_processMatchCandidate() is called is not necessarily
            // at the end of a match from the ldm seq store, and will often be some bytes
            // over beyond matchEndPosInBlock. As such, we need to correct for these "overshoots"
            let posOvershoot = currPosInBlock.wrapping_sub(optLdm.endPosInBlock);
            ZSTD_optLdm_skipRawSeqStoreBytes(&mut optLdm.seqStore, posOvershoot as size_t);
        }
        ZSTD_opt_getNextMatchAndUpdateSeqStore(optLdm, currPosInBlock, remainingBytes);
    }
    ZSTD_optLdm_maybeAddMatch(matches, nbMatches, optLdm, currPosInBlock, minMatch);
}

unsafe fn ZSTD_compressBlock_opt_generic<const OPT_LEVEL: core::ffi::c_int>(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dictMode: DictMode,
) -> size_t {
    let mut current_block: u64;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.add(srcSize);
    let ilimit = iend.sub(8);
    let base = ms.window.base;
    let prefixStart = base.wrapping_offset(ms.window.dictLimit as isize);
    let cParams: *const ZSTD_compressionParameters = &ms.cParams;

    let getAllMatches = ZSTD_selectBtGetAllMatches(ms, dictMode);

    let sufficient_len = (*cParams)
        .targetLength
        .min(((1 << 12) - 1) as core::ffi::c_uint);
    let minMatch = (if (*cParams).minMatch == 3 { 3 } else { 4 }) as u32;
    let mut nextToUpdate3 = ms.nextToUpdate;

    let opt = ms.opt.priceTable;
    let matches = ms.opt.matchTable;
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

    optLdm.seqStore = if !(ms.ldmSeqStore).is_null() {
        *ms.ldmSeqStore
    } else {
        RawSeqStore_t::new()
    };
    optLdm.offset = 0;
    optLdm.startPosInBlock = optLdm.offset;
    optLdm.endPosInBlock = optLdm.startPosInBlock;
    ZSTD_opt_getNextMatchAndUpdateSeqStore(
        &mut optLdm,
        ip.offset_from(istart) as core::ffi::c_long as u32,
        iend.offset_from(ip) as core::ffi::c_long as u32,
    );

    // init
    ZSTD_rescaleFreqs(&mut ms.opt, src as *const u8, srcSize, OPT_LEVEL);
    ip = ip.offset((ip == prefixStart) as core::ffi::c_int as isize);

    // Match Loop
    while ip < ilimit {
        let mut cur: u32 = 0;
        let mut last_pos = 0;

        // find first match
        let litlen = ip.offset_from(anchor) as core::ffi::c_long as u32;
        let ll0 = (litlen == 0) as core::ffi::c_int as u32;
        let mut nbMatches = getAllMatches(
            matches,
            ms,
            &mut nextToUpdate3,
            ip,
            iend,
            rep,
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
            // Match found: let's store this solution, and eventually find more candidates.
            // During this forward pass, @opt is used to store stretches,
            // defined as "a match followed by N literals".
            // Note how this is different from a Sequence, which is "N literals followed by a match".
            // Storing stretches allows us to store different match predecessors
            // for each literal position part of a literals run.

            // initialize opt[0]
            (*opt).mlen = 0; // there are only literals so far
            (*opt).litlen = litlen;
            (*opt).off = 0; // initialized to prevent UB

            // No need to include the actual price of the literals before the first match
            // because it is static for the duration of the forward pass, and is included
            // in every subsequent price. But, we include the literal length because
            // the cost variation of litlen depends on the value of litlen.
            (*opt).price = ZSTD_litLengthPrice(litlen, &ms.opt, OPT_LEVEL) as core::ffi::c_int;
            (*opt).rep = *rep;

            // large match -> immediate encoding
            let maxML = (*matches.offset(nbMatches.wrapping_sub(1) as isize)).len;
            let maxOffBase = (*matches.offset(nbMatches.wrapping_sub(1) as isize)).off;
            if maxML > sufficient_len {
                lastStretch.litlen = 0;
                lastStretch.mlen = maxML;
                lastStretch.off = maxOffBase;
                cur = 0;
                last_pos = maxML;
            } else {
                // set prices for first matches starting position == 0
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
                        let matchPrice = ZSTD_getMatchPrice(offBase, pos, &ms.opt, OPT_LEVEL)
                            as core::ffi::c_int;
                        let sequencePrice = (*opt).price + matchPrice;
                        (*opt.offset(pos as isize)).mlen = pos;
                        (*opt.offset(pos as isize)).off = offBase;
                        (*opt.offset(pos as isize)).litlen = 0; // end of match
                        (*opt.offset(pos as isize)).price = sequencePrice
                            + ZSTD_litLengthPrice(0, &ms.opt, OPT_LEVEL) as core::ffi::c_int;
                        pos = pos.wrapping_add(1);
                    }
                    matchNb = matchNb.wrapping_add(1);
                }
                last_pos = pos.wrapping_sub(1);
                (*opt.offset(pos as isize)).price = ZSTD_MAX_PRICE;

                // check further positions
                cur = 1;
                loop {
                    if cur > last_pos {
                        current_block = 10357520176418200368;
                        break;
                    }
                    let inr = ip.offset(cur as isize);

                    // Fix current position with one literal if cheaper
                    let litlen_0 =
                        ((*opt.offset(cur.wrapping_sub(1) as isize)).litlen).wrapping_add(1);
                    let price = (*opt.offset(cur.wrapping_sub(1) as isize)).price
                        + ZSTD_rawLiteralsCost(
                            ip.offset(cur as isize).sub(1),
                            1,
                            &ms.opt,
                            OPT_LEVEL,
                        ) as core::ffi::c_int
                        + (ZSTD_litLengthPrice(litlen_0, &ms.opt, OPT_LEVEL) as core::ffi::c_int
                            - ZSTD_litLengthPrice(litlen_0.wrapping_sub(1), &ms.opt, OPT_LEVEL)
                                as core::ffi::c_int);
                    if price <= (*opt.offset(cur as isize)).price {
                        let prevMatch = *opt.offset(cur as isize);
                        *opt.offset(cur as isize) = *opt.offset(cur.wrapping_sub(1) as isize);
                        (*opt.offset(cur as isize)).litlen = litlen_0;
                        (*opt.offset(cur as isize)).price = price;
                        if OPT_LEVEL >= 1
                            && prevMatch.litlen == 0
                            && (ZSTD_litLengthPrice(1, &ms.opt, OPT_LEVEL) as core::ffi::c_int
                                - ZSTD_litLengthPrice((1 - 1) as u32, &ms.opt, OPT_LEVEL)
                                    as core::ffi::c_int)
                                < 0
                            && (ip.offset(cur as isize) < iend) as core::ffi::c_int
                                as core::ffi::c_long
                                != 0
                        {
                            // check next position, in case it would be cheaper
                            let with1literal = prevMatch.price
                                + ZSTD_rawLiteralsCost(
                                    ip.offset(cur as isize),
                                    1,
                                    &ms.opt,
                                    OPT_LEVEL,
                                ) as core::ffi::c_int
                                + (ZSTD_litLengthPrice(1, &ms.opt, OPT_LEVEL) as core::ffi::c_int
                                    - ZSTD_litLengthPrice((1 - 1) as u32, &ms.opt, OPT_LEVEL)
                                        as core::ffi::c_int);
                            let withMoreLiterals = price
                                + ZSTD_rawLiteralsCost(
                                    ip.offset(cur as isize),
                                    1,
                                    &ms.opt,
                                    OPT_LEVEL,
                                ) as core::ffi::c_int
                                + (ZSTD_litLengthPrice(litlen_0.wrapping_add(1), &ms.opt, OPT_LEVEL)
                                    as core::ffi::c_int
                                    - ZSTD_litLengthPrice(
                                        litlen_0.wrapping_add(1).wrapping_sub(1),
                                        &ms.opt,
                                        OPT_LEVEL,
                                    ) as core::ffi::c_int);
                            if with1literal < withMoreLiterals
                                && with1literal < (*opt.offset(cur.wrapping_add(1) as isize)).price
                            {
                                // update offset history - before it disappears
                                let prev = cur.wrapping_sub(prevMatch.mlen);
                                let newReps = ZSTD_newRep(
                                    &(*opt.offset(prev as isize)).rep,
                                    prevMatch.off,
                                    ((*opt.offset(prev as isize)).litlen == 0) as core::ffi::c_int
                                        as u32,
                                );
                                *opt.offset(cur.wrapping_add(1) as isize) = prevMatch;
                                (*opt.offset(cur.wrapping_add(1) as isize)).rep = newReps.rep;
                                (*opt.offset(cur.wrapping_add(1) as isize)).litlen = 1;
                                (*opt.offset(cur.wrapping_add(1) as isize)).price = with1literal;
                                if last_pos < cur.wrapping_add(1) {
                                    last_pos = cur.wrapping_add(1);
                                }
                            }
                        }
                    }

                    // Offset history is not updated during match comparison.
                    // Do it here, now that the match is selected and confirmed.
                    if (*opt.offset(cur as isize)).litlen == 0 {
                        // just finished a match => alter offset history
                        let prev_0 = cur.wrapping_sub((*opt.offset(cur as isize)).mlen);
                        let newReps_0 = ZSTD_newRep(
                            &(*opt.offset(prev_0 as isize)).rep,
                            (*opt.offset(cur as isize)).off,
                            ((*opt.offset(prev_0 as isize)).litlen == 0) as core::ffi::c_int as u32,
                        );
                        (*opt.offset(cur as isize)).rep = newReps_0.rep;
                    }

                    // last match must start at a minimum distance of 8 from oend
                    if inr <= ilimit {
                        if cur == last_pos {
                            current_block = 10357520176418200368;
                            break;
                        }

                        // skip unpromising positions; about ~+6% speed, -0.01 ratio
                        if !(OPT_LEVEL == 0
                            && (*opt.offset(cur.wrapping_add(1) as isize)).price
                                <= (*opt.offset(cur as isize)).price + BITCOST_MULTIPLIER / 2)
                        {
                            let ll0_0 = ((*opt.offset(cur as isize)).litlen == 0)
                                as core::ffi::c_int as u32;
                            let previousPrice = (*opt.offset(cur as isize)).price;
                            let basePrice = previousPrice
                                + ZSTD_litLengthPrice(0, &ms.opt, OPT_LEVEL) as core::ffi::c_int;
                            let mut nbMatches_0 = getAllMatches(
                                matches,
                                ms,
                                &mut nextToUpdate3,
                                inr,
                                iend,
                                &(*opt.offset(cur as isize)).rep,
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
                                    // set prices using matches found at position == cur
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

                                        // scan downward
                                        for mlen in (startML..lastML + 1).rev() {
                                            let pos_0 = cur.wrapping_add(mlen);
                                            let price_0 = basePrice
                                                + ZSTD_getMatchPrice(
                                                    offset, mlen, &ms.opt, OPT_LEVEL,
                                                )
                                                    as core::ffi::c_int;

                                            if pos_0 > last_pos
                                                || price_0 < (*opt.offset(pos_0 as isize)).price
                                            {
                                                while last_pos < pos_0 {
                                                    // fill empty positions, for future comparisons
                                                    last_pos = last_pos.wrapping_add(1);
                                                    (*opt.offset(last_pos as isize)).price =
                                                        ZSTD_MAX_PRICE;
                                                    // just needs to be != 0, to mean "not an end of match
                                                    (*opt.offset(last_pos as isize)).litlen =
                                                        (0 == 0) as core::ffi::c_int as u32;
                                                }
                                                (*opt.offset(pos_0 as isize)).mlen = mlen;
                                                (*opt.offset(pos_0 as isize)).off = offset;
                                                (*opt.offset(pos_0 as isize)).litlen = 0;
                                                (*opt.offset(pos_0 as isize)).price = price_0;
                                            } else if OPT_LEVEL == 0 {
                                                break; // early update abort; gets ~+10% speed for about -0.01 ratio loss
                                            }
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
                // no solution: all matches have been converted into literals
                ip = ip.offset(last_pos as isize);
            } else {
                // Update offset history
                if lastStretch.litlen == 0 {
                    // finishing on a match: update offset history
                    let reps = ZSTD_newRep(
                        &(*opt.offset(cur as isize)).rep,
                        lastStretch.off,
                        ((*opt.offset(cur as isize)).litlen == 0) as core::ffi::c_int as u32,
                    );
                    *rep = reps.rep;
                } else {
                    *rep = lastStretch.rep;
                    cur = cur.wrapping_sub(lastStretch.litlen);
                }

                // Let's write the shortest path solution.
                // It is stored in @opt in reverse order,
                // starting from @storeEnd (==cur+2),
                // effectively partially @opt overwriting.
                // Content is changed too:
                // - So far, @opt stored stretches, aka a match followed by literals
                // - Now, it will store sequences, aka literals followed by a match
                let storeEnd = cur.wrapping_add(2);
                let mut storeStart = storeEnd;
                let mut stretchPos = cur;

                if lastStretch.litlen > 0 {
                    // last "sequence" is unfinished: just a bunch of literals
                    (*opt.offset(storeEnd as isize)).litlen = lastStretch.litlen;
                    (*opt.offset(storeEnd as isize)).mlen = 0;
                    storeStart = storeEnd.wrapping_sub(1);
                    *opt.offset(storeStart as isize) = lastStretch;
                }
                *opt.offset(storeEnd as isize) = lastStretch; // note: litlen will be fixed
                storeStart = storeEnd;

                loop {
                    let nextStretch = *opt.offset(stretchPos as isize);
                    (*opt.offset(storeStart as isize)).litlen = nextStretch.litlen;
                    if nextStretch.mlen == 0 {
                        // reaching beginning of segment
                        break;
                    }
                    storeStart = storeStart.wrapping_sub(1);
                    *opt.offset(storeStart as isize) = nextStretch; // note: litlen will be fixed
                    stretchPos = stretchPos
                        .wrapping_sub((nextStretch.litlen).wrapping_add(nextStretch.mlen));
                }

                // save sequences
                let mut storePos: u32 = 0;
                storePos = storeStart;
                while storePos <= storeEnd {
                    let llen = (*opt.offset(storePos as isize)).litlen;
                    let mlen_0 = (*opt.offset(storePos as isize)).mlen;
                    let offBase_0 = (*opt.offset(storePos as isize)).off;
                    let advance = llen.wrapping_add(mlen_0);

                    if mlen_0 == 0 {
                        // only literals => must be last "sequence", actually starting a new stream of sequences
                        ip = anchor.offset(llen as isize); // last "sequence" is a bunch of literals => don't progress anchor
                    } else {
                        ZSTD_updateStats(&mut ms.opt, llen, anchor, offBase_0, mlen_0);
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

                // update all costs
                ZSTD_setBasePrices(&mut ms.opt, OPT_LEVEL);
            }
        }
    }

    // Return the last literals size
    iend.offset_from_unsigned(anchor)
}

pub unsafe fn ZSTD_compressBlock_btopt(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt_generic::<0>(ms, seqStore, rep, src, srcSize, DictMode::NoDict)
}

/// Make a first compression pass, just to seed stats with more accurate starting values.
/// Only works on first block, with no dictionary and no ldm.
/// This function cannot error out, its narrow contract must be respected.
unsafe fn ZSTD_initStats_ultra(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    let mut tmpRep = *rep; // updated rep codes will sink here

    // generate stats into ms->opt
    ZSTD_compressBlock_opt_generic::<2>(ms, seqStore, &mut tmpRep, src, srcSize, DictMode::NoDict);

    // invalidate first scan from history, only keep entropy stats
    ZSTD_resetSeqStore(seqStore);
    ms.window.base = (ms.window.base).wrapping_sub(srcSize);
    ms.window.dictLimit = (ms.window.dictLimit).wrapping_add(srcSize as u32);
    ms.window.lowLimit = ms.window.dictLimit;
    ms.nextToUpdate = ms.window.dictLimit;
}

pub unsafe fn ZSTD_compressBlock_btultra(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt_generic::<2>(ms, seqStore, rep, src, srcSize, DictMode::NoDict)
}

pub unsafe fn ZSTD_compressBlock_btultra2(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let curr = (src as *const u8).wrapping_offset_from(ms.window.base) as core::ffi::c_long as u32;

    // 2-passes strategy:
    // this strategy makes a first pass over first block to collect statistics
    // in order to seed next round's statistics with it.
    // After 1st pass, function forgets history, and starts a new block.
    // Consequently, this can only work if no data has been previously loaded in tables,
    // aka, no dictionary, no prefix, no ldm preprocessing.
    // The compression ratio gain is generally small (~0.5% on first block),
    // the cost is 2x cpu time on first block.
    if ms.opt.litLengthSum == 0
        && seqStore.sequences == seqStore.sequencesStart
        && ms.window.dictLimit == ms.window.lowLimit
        && curr == ms.window.dictLimit
        && srcSize > ZSTD_PREDEF_THRESHOLD as size_t
    {
        ZSTD_initStats_ultra(ms, seqStore, rep, src, srcSize);
    }

    ZSTD_compressBlock_opt_generic::<2>(ms, seqStore, rep, src, srcSize, DictMode::NoDict)
}

pub unsafe fn ZSTD_compressBlock_btopt_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt_generic::<0>(ms, seqStore, rep, src, srcSize, DictMode::DictMatchState)
}

pub unsafe fn ZSTD_compressBlock_btopt_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt_generic::<0>(ms, seqStore, rep, src, srcSize, DictMode::ExtDict)
}

pub unsafe fn ZSTD_compressBlock_btultra_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt_generic::<2>(ms, seqStore, rep, src, srcSize, DictMode::DictMatchState)
}

pub unsafe fn ZSTD_compressBlock_btultra_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: &mut [u32; 3],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt_generic::<2>(ms, seqStore, rep, src, srcSize, DictMode::ExtDict)
}

// note: no btultra2 variant for extDict nor dictMatchState,
// because btultra2 is not meant to work with dictionaries
// and is only specific for the first block (no prefix)

pub const __INT_MAX__: core::ffi::c_int = 2147483647;
