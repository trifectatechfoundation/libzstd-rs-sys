use core::arch::asm;

#[repr(C)]
pub struct optState_t {
    pub litFreq: *mut core::ffi::c_uint,
    pub litLengthFreq: *mut core::ffi::c_uint,
    pub matchLengthFreq: *mut core::ffi::c_uint,
    pub offCodeFreq: *mut core::ffi::c_uint,
    pub matchTable: *mut ZSTD_match_t,
    pub priceTable: *mut ZSTD_optimal_t,
    pub litSum: u32,
    pub litLengthSum: u32,
    pub matchLengthSum: u32,
    pub offCodeSum: u32,
    pub litSumBasePrice: u32,
    pub litLengthSumBasePrice: u32,
    pub matchLengthSumBasePrice: u32,
    pub offCodeSumBasePrice: u32,
    pub priceType: ZSTD_OptPrice_e,
    pub symbolCosts: *const ZSTD_entropyCTables_t,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
}

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
    pub CTable: [HUF_CElt; HUF_CTABLE_SIZE_ST(255)],
    pub repeatMode: HUF_repeat,
}

#[repr(C)]
pub struct ZSTD_match_t {
    pub off: u32,
    pub len: u32,
}

pub type ZSTD_dictMode_e = core::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type searchMethod_e = core::ffi::c_uint;
pub const search_rowHash: searchMethod_e = 2;
pub const search_binaryTree: searchMethod_e = 1;
pub const search_hashChain: searchMethod_e = 0;
pub type ZSTD_VecMask = u64;

use libc::size_t;

use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::fse::{FSE_CTable, FSE_repeat};
use crate::lib::common::huf::{HUF_CElt, HUF_repeat, HUF_CTABLE_SIZE_ST};
use crate::lib::common::mem::MEM_read32;
use crate::lib::common::zstd_internal::ZSTD_REP_NUM;
use crate::lib::compress::zstd_compress::{SeqStore_t, ZSTD_MatchState_t, ZSTD_optimal_t};
use crate::lib::compress::zstd_compress_internal::{
    ZSTD_OptPrice_e, ZSTD_count, ZSTD_count_2segments, ZSTD_getLowestMatchIndex,
    ZSTD_getLowestPrefixIndex, ZSTD_hashPtr, ZSTD_hashPtrSalted, ZSTD_index_overlap_check,
    ZSTD_storeSeq,
};
use crate::lib::polyfill::{prefetch_read_data, Locality};
use crate::lib::zstd::{ZSTD_ParamSwitch_e, ZSTD_compressionParameters};

pub const kSearchStrength: core::ffi::c_int = 8;
pub const ZSTD_DUBT_UNSORTED_MARK: core::ffi::c_int = 1;
pub const ZSTD_ROW_HASH_CACHE_SIZE: core::ffi::c_int = 8;

pub const REPCODE1_TO_OFFBASE: core::ffi::c_int = 1;

pub const ZSTD_LAZY_DDSS_BUCKET_LOG: core::ffi::c_int = 2;
pub const ZSTD_ROW_HASH_TAG_BITS: core::ffi::c_int = 8;
pub const kLazySkippingStep: core::ffi::c_int = 8;

unsafe fn ZSTD_updateDUBT(ms: &mut ZSTD_MatchState_t, ip: *const u8, iend: *const u8, mls: u32) {
    let cParams = &ms.cParams;
    let hashTable = ms.hashTable;
    let hashLog = cParams.hashLog;

    let bt = ms.chainTable;
    let btLog = (cParams.chainLog).wrapping_sub(1);
    let btMask = ((1 << btLog) - 1) as u32;

    let base = ms.window.base;
    let target = ip.offset_from(base) as core::ffi::c_long as u32;
    assert!(ip.wrapping_add(8) <= iend); // condition for ZSTD_hashPtr

    for idx in ms.nextToUpdate..target {
        // assumption: ip + 8 <= iend
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const core::ffi::c_void,
            hashLog,
            mls,
        );
        let matchIndex = *hashTable.add(h);
        let nextCandidatePtr = bt.offset((2 * (idx & btMask)) as isize);
        let sortMarkPtr = nextCandidatePtr.add(1);

        *hashTable.add(h) = idx; // Update Hash Table
        *nextCandidatePtr = matchIndex; // update BT like a chain
        *sortMarkPtr = ZSTD_DUBT_UNSORTED_MARK as u32;
    }

    ms.nextToUpdate = target;
}

/// Sort one already inserted but unsorted position
///
/// Assumption: curr >= btlow == (curr - btmask)
///
/// doesn't fail
unsafe fn ZSTD_insertDUBT1(
    ms: &ZSTD_MatchState_t,
    curr: u32,
    inputEnd: *const u8,
    mut nbCompares: u32,
    btLow: u32,
    dictMode: ZSTD_dictMode_e,
) {
    let cParams: *const ZSTD_compressionParameters = &ms.cParams;
    let bt = ms.chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1);
    let btMask = ((1 << btLog) - 1) as u32;
    let mut commonLengthSmaller = 0;
    let mut commonLengthLarger = 0;
    let base = ms.window.base;
    let dictBase = ms.window.dictBase;
    let dictLimit = ms.window.dictLimit;
    let ip = if curr >= dictLimit {
        base.offset(curr as isize)
    } else {
        dictBase.offset(curr as isize)
    };
    let iend = if curr >= dictLimit {
        inputEnd
    } else {
        dictBase.offset(dictLimit as isize)
    };
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut match_0 = core::ptr::null::<u8>();
    let mut smallerPtr = bt.offset((2 * (curr & btMask)) as isize);
    let mut largerPtr = smallerPtr.add(1);
    let mut matchIndex = *smallerPtr; // this candidate is unsorted: next sorted candidate is reached through *smallerPtr, while *largerPtr contains previous unsorted candidate (which is already saved and can be overwritten)
    let mut dummy32: u32 = 0; // to be nullified at the end
    let windowValid = ms.window.lowLimit;
    let maxDistance = 1 << (*cParams).windowLog;
    let windowLow = if curr.wrapping_sub(windowValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        windowValid
    };

    while nbCompares != 0 && matchIndex > windowLow {
        let nextPtr = bt.offset((2 * (matchIndex & btMask)) as isize);
        let mut matchLength = commonLengthSmaller.min(commonLengthLarger); // guaranteed minimum number of common bytes

        // Note: all candidates are now supposed sorted,
        // but it's still possible to have nextPtr[1] == ZSTD_DUBT_UNSORTED_MARK
        // when a real index has the same value as ZSTD_DUBT_UNSORTED_MARK

        if dictMode != ZSTD_extDict
            || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
            || curr < dictLimit
        {
            let mBase = if dictMode != ZSTD_extDict
                || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
            {
                base
            } else {
                dictBase
            };
            match_0 = mBase.offset(matchIndex as isize);
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
                match_0 = base.offset(matchIndex as isize); // preparation for next read of match[matchLength]
            }
        }

        if ip.add(matchLength) == iend {
            // equal: no way to know if inf or sup
            // drop , to guarantee consistency; miss a bit of compression, but other solutions can corrupt tree
            break;
        } else {
            // necessarily within buffer
            if (*match_0.add(matchLength) as core::ffi::c_int)
                < *ip.add(matchLength) as core::ffi::c_int
            {
                // match is smaller than current
                *smallerPtr = matchIndex; // update smaller idx
                commonLengthSmaller = matchLength; // all smaller will now have at least this guaranteed common length
                if matchIndex <= btLow {
                    // beyond tree size, stop searching
                    smallerPtr = &mut dummy32;
                    break;
                } else {
                    smallerPtr = nextPtr.add(1); // new "candidate" => larger than match, which was smaller than target
                    matchIndex = *nextPtr.add(1); // new matchIndex, larger than previous and closer to current
                }
            } else {
                // match is larger than current
                *largerPtr = matchIndex;
                commonLengthLarger = matchLength;
                if matchIndex <= btLow {
                    // beyond tree size, stop searching
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
}

unsafe fn ZSTD_DUBT_findBetterDictMatch(
    ms: &ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    offsetPtr: &mut size_t,
    mut bestLength: size_t,
    mut nbCompares: u32,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let dms = ms.dictMatchState;
    let dmsCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashTable: *const u32 = (*dms).hashTable;
    let hashLog = (*dmsCParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const core::ffi::c_void, hashLog, mls);
    let mut dictMatchIndex = *dictHashTable.add(h);

    let base = ms.window.base;
    let prefixStart = base.offset(ms.window.dictLimit as isize);
    let curr = ip.offset_from(base) as core::ffi::c_long as u32;
    let dictBase = (*dms).window.base;
    let dictEnd = (*dms).window.nextSrc;
    let dictHighLimit =
        ((*dms).window.nextSrc).offset_from((*dms).window.base) as core::ffi::c_long as u32;
    let dictLowLimit = (*dms).window.lowLimit;
    let dictIndexDelta = (ms.window.lowLimit).wrapping_sub(dictHighLimit);

    let dictBt = (*dms).chainTable;
    let btLog = ((*dmsCParams).chainLog).wrapping_sub(1);
    let btMask = ((1 << btLog) - 1) as u32;
    let btLow = if btMask >= dictHighLimit.wrapping_sub(dictLowLimit) {
        dictLowLimit
    } else {
        dictHighLimit.wrapping_sub(btMask)
    };

    let mut commonLengthSmaller = 0usize;
    let mut commonLengthLarger = 0usize;

    assert_eq!(dictMode, ZSTD_dictMatchState);

    while nbCompares != 0 && dictMatchIndex > dictLowLimit {
        let nextPtr = dictBt.offset((2 * (dictMatchIndex & btMask)) as isize);
        let mut matchLength = commonLengthSmaller.min(commonLengthLarger); // guaranteed minimum nb of common bytes
        let mut match_0 = dictBase.offset(dictMatchIndex as isize);
        matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
            ip.add(matchLength),
            match_0.add(matchLength),
            iend,
            dictEnd,
            prefixStart,
        ));
        if (dictMatchIndex as size_t).wrapping_add(matchLength) >= dictHighLimit as size_t {
            // to prepare for next usage of match[matchLength]
            match_0 = base
                .offset(dictMatchIndex as isize)
                .offset(dictIndexDelta as isize);
        }

        if matchLength > bestLength {
            let matchIndex = dictMatchIndex.wrapping_add(dictIndexDelta);
            if 4 * matchLength.wrapping_sub(bestLength) as core::ffi::c_int
                > (ZSTD_highbit32(curr.wrapping_sub(matchIndex).wrapping_add(1)))
                    .wrapping_sub(ZSTD_highbit32((*offsetPtr as u32).wrapping_add(1)))
                    as core::ffi::c_int
            {
                bestLength = matchLength;
                *offsetPtr = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            }
            if ip.add(matchLength) == iend {
                // reached end of input: ip[matchLength] is not valid, no way to know if it's larger or smaller than match
                // drop, to guarantee consistency (miss a little bit of compression)
                break;
            }
        }

        if (*match_0.add(matchLength) as core::ffi::c_int)
            < *ip.add(matchLength) as core::ffi::c_int
        {
            if dictMatchIndex <= btLow {
                break; // beyond tree size, stop the search
            }
            commonLengthSmaller = matchLength; // all smaller will now have at least this guaranteed common length
            dictMatchIndex = *nextPtr.add(1); // new matchIndex larger than previous (closer to current)
        } else {
            // match is larger than current
            if dictMatchIndex <= btLow {
                break; // beyond tree size, stop the search
            }
            commonLengthLarger = matchLength;
            dictMatchIndex = *nextPtr;
        }
        nbCompares = nbCompares.wrapping_sub(1);
    }

    bestLength
}

unsafe fn ZSTD_DUBT_findBestMatch(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    offBasePtr: &mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let cParams = &ms.cParams;
    let hashTable = ms.hashTable;
    let hashLog = cParams.hashLog;
    let h = ZSTD_hashPtr(ip as *const core::ffi::c_void, hashLog, mls);
    let mut matchIndex = *hashTable.add(h);

    let base = ms.window.base;
    let curr = ip.offset_from(base) as core::ffi::c_long as u32;
    let windowLow = ZSTD_getLowestMatchIndex(ms, curr, cParams.windowLog);

    let bt = ms.chainTable;
    let btLog = (cParams.chainLog).wrapping_sub(1);
    let btMask = ((1 << btLog) - 1) as u32;
    let btLow = if btMask >= curr {
        0
    } else {
        curr.wrapping_sub(btMask)
    };
    let unsortLimit = btLow.max(windowLow);

    let mut nextCandidate = bt.offset((2 * (matchIndex & btMask)) as isize);
    let mut unsortedMark = bt.offset((2 * (matchIndex & btMask)) as isize).add(1);
    let mut nbCompares = (1 as core::ffi::c_uint) << cParams.searchLog;
    let mut nbCandidates = nbCompares;
    let mut previousCandidate = 0;

    // reach end of unsorted candidates list
    while matchIndex > unsortLimit
        && *unsortedMark == ZSTD_DUBT_UNSORTED_MARK as u32
        && nbCandidates > 1
    {
        *unsortedMark = previousCandidate; // the unsortedMark becomes a reversed chain, to move up back to original position
        previousCandidate = matchIndex;
        matchIndex = *nextCandidate;
        nextCandidate = bt.offset((2 * (matchIndex & btMask)) as isize);
        unsortedMark = bt.offset((2 * (matchIndex & btMask)) as isize).add(1);
        nbCandidates = nbCandidates.wrapping_sub(1);
    }

    // nullify last candidate if it's still unsorted
    // simplification, detrimental to compression ratio, beneficial for speed
    if matchIndex > unsortLimit && *unsortedMark == ZSTD_DUBT_UNSORTED_MARK as u32 {
        *unsortedMark = 0;
        *nextCandidate = *unsortedMark;
    }

    // batch sort stacked candidates
    matchIndex = previousCandidate;
    while matchIndex != 0 {
        let nextCandidateIdxPtr = bt.offset((2 * (matchIndex & btMask)) as isize).add(1);
        let nextCandidateIdx = *nextCandidateIdxPtr;
        ZSTD_insertDUBT1(ms, matchIndex, iend, nbCandidates, unsortLimit, dictMode);
        matchIndex = nextCandidateIdx;
        nbCandidates = nbCandidates.wrapping_add(1);
    }

    // find longest match
    let mut commonLengthSmaller = 0;
    let mut commonLengthLarger = 0;
    let dictBase = ms.window.dictBase;
    let dictLimit = ms.window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut smallerPtr = bt.offset((2 * (curr & btMask)) as isize);
    let mut largerPtr = bt.offset((2 * (curr & btMask)) as isize).add(1);
    let mut matchEndIdx = curr.wrapping_add(8).wrapping_add(1);
    let mut dummy32: u32 = 0;
    let mut bestLength = 0;

    matchIndex = *hashTable.add(h);
    *hashTable.add(h) = curr;

    while nbCompares != 0 && matchIndex > windowLow {
        let nextPtr = bt.offset((2 * (matchIndex & btMask)) as isize);
        let mut matchLength = commonLengthSmaller.min(commonLengthLarger); // guaranteed minimum nb of common bytes
        let mut match_0 = core::ptr::null::<u8>();

        if dictMode != ZSTD_extDict
            || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
        {
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
                match_0 = base.offset(matchIndex as isize); // to prepare for next usage of match[matchLength]
            }
        }

        if matchLength > bestLength {
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                matchEndIdx = matchIndex.wrapping_add(matchLength as u32);
            }
            if 4 * matchLength.wrapping_sub(bestLength) as core::ffi::c_int
                > (ZSTD_highbit32(curr.wrapping_sub(matchIndex).wrapping_add(1)))
                    .wrapping_sub(ZSTD_highbit32(*offBasePtr as u32))
                    as core::ffi::c_int
            {
                bestLength = matchLength;
                *offBasePtr = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            }
            if ip.add(matchLength) == iend {
                // equal: no way to know if inf or sup
                if dictMode == ZSTD_dictMatchState {
                    // in addition to avoiding checking any further in this loop,
                    // make sure we skip checking in the dictionary
                    nbCompares = 0;
                }
                break; // drop, to guarantee consistency (miss a little bit of compression)
            }
        }

        if (*match_0.add(matchLength) as core::ffi::c_int)
            < *ip.add(matchLength) as core::ffi::c_int
        {
            // match is smaller than current
            *smallerPtr = matchIndex; // update smaller idx
            commonLengthSmaller = matchLength; // all smaller will now have at least this guaranteed common length
            if matchIndex <= btLow {
                // beyond tree size, stop the search
                smallerPtr = &mut dummy32;
                break;
            } else {
                smallerPtr = nextPtr.add(1); // new "smaller" => larger of match
                matchIndex = *nextPtr.add(1); // new matchIndex larger than previous (closer to current)
            }
        } else {
            // match is larger than current
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

    if dictMode == ZSTD_dictMatchState && nbCompares != 0 {
        bestLength = ZSTD_DUBT_findBetterDictMatch(
            ms, ip, iend, offBasePtr, bestLength, nbCompares, mls, dictMode,
        );
    }

    ms.nextToUpdate = matchEndIdx.wrapping_sub(8); // skip repetitive patterns

    bestLength
}

/// Tree updater, providing best match
#[inline(always)]
unsafe fn ZSTD_BtFindBestMatch(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    if ip < (ms.window.base).offset(ms.nextToUpdate as isize) {
        return 0; // skipped area
    }
    ZSTD_updateDUBT(ms, ip, iLimit, mls);
    ZSTD_DUBT_findBestMatch(ms, ip, iLimit, offBasePtr, mls, dictMode)
}

pub unsafe fn ZSTD_dedicatedDictSearch_lazy_loadDictionary(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
) {
    let base = ms.window.base;
    let target = ip.offset_from(base) as core::ffi::c_long as u32;
    let hashTable = ms.hashTable;
    let chainTable = ms.chainTable;
    let chainSize = (1 << ms.cParams.chainLog) as u32;
    let idx = ms.nextToUpdate;
    let minChain = if chainSize < target.wrapping_sub(idx) {
        target.wrapping_sub(chainSize)
    } else {
        idx
    };
    let bucketSize = (1 << ZSTD_LAZY_DDSS_BUCKET_LOG) as u32;
    let cacheSize = bucketSize.wrapping_sub(1);
    let chainAttempts = ((1 << ms.cParams.searchLog) as u32).wrapping_sub(cacheSize);
    let chainLimit = chainAttempts.min(255);

    // We know the hashtable is oversized by a factor of `bucketSize`.
    // We are going to temporarily pretend `bucketSize == 1`, keeping only a
    // single entry. We will use the rest of the space to construct a temporary
    // chaintable.
    let hashLog = (ms.cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as core::ffi::c_uint);
    let tmpHashTable = hashTable;
    let tmpChainTable = hashTable.offset((1 << hashLog) as isize);
    let tmpChainSize = (((1 << ZSTD_LAZY_DDSS_BUCKET_LOG) - 1) as u32) << hashLog;
    let tmpMinChain = if tmpChainSize < target {
        target.wrapping_sub(tmpChainSize)
    } else {
        idx
    };
    let mut hashIdx: u32 = 0;

    // fill conventional hash table and conventional chain table
    for idx in idx..target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const core::ffi::c_void,
            hashLog,
            ms.cParams.minMatch,
        ) as u32;
        if idx >= tmpMinChain {
            *tmpChainTable.offset(idx.wrapping_sub(tmpMinChain) as isize) =
                *hashTable.offset(h as isize);
        }
        *tmpHashTable.offset(h as isize) = idx;
    }

    // sort chains into DDSS chain table
    let mut chainPos = 0u32;
    hashIdx = 0;
    while hashIdx < 1 << hashLog {
        let mut count: u32 = 0;
        let mut countBeyondMinChain = 0u32;
        let mut i = *tmpHashTable.offset(hashIdx as isize);
        count = 0;
        while i >= tmpMinChain && count < cacheSize {
            // skip through the chain to the first position that won't be
            // in the hash cache bucket
            if i < minChain {
                countBeyondMinChain = countBeyondMinChain.wrapping_add(1);
            }
            i = *tmpChainTable.offset(i.wrapping_sub(tmpMinChain) as isize);
            count = count.wrapping_add(1);
        }
        if count == cacheSize {
            count = 0;
            while count < chainLimit {
                if i < minChain
                    && (i == 0 || {
                        countBeyondMinChain = countBeyondMinChain.wrapping_add(1);
                        countBeyondMinChain > cacheSize
                    })
                {
                    // only allow pulling `cacheSize` number of entries
                    // into the cache or chainTable beyond `minChain`,
                    // to replace the entries pulled out of the
                    // chainTable into the cache. This lets us reach
                    // back further without increasing the total number
                    // of entries in the chainTable, guaranteeing the
                    // DDSS chain table will fit into the space
                    // allocated for the regular one.
                    break;
                }
                *chainTable.offset(chainPos as isize) = i;
                chainPos = chainPos.wrapping_add(1);
                count = count.wrapping_add(1);
                if i < tmpMinChain {
                    break;
                }
                i = *tmpChainTable.offset(i.wrapping_sub(tmpMinChain) as isize);
            }
        } else {
            count = 0;
        }
        if count != 0 {
            *tmpHashTable.offset(hashIdx as isize) =
                (chainPos.wrapping_sub(count) << 8).wrapping_add(count);
        } else {
            *tmpHashTable.offset(hashIdx as isize) = 0;
        }
        hashIdx = hashIdx.wrapping_add(1);
    }

    // move chain pointers into the last entry of each hash bucket
    hashIdx = (1 << hashLog) as u32;
    while hashIdx != 0 {
        hashIdx = hashIdx.wrapping_sub(1);
        let bucketIdx = hashIdx << ZSTD_LAZY_DDSS_BUCKET_LOG;
        let chainPackedPointer = *tmpHashTable.offset(hashIdx as isize);
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < cacheSize {
            *hashTable.offset(bucketIdx.wrapping_add(i_0) as isize) = 0;
            i_0 = i_0.wrapping_add(1);
        }
        *hashTable.offset(bucketIdx.wrapping_add(bucketSize).wrapping_sub(1) as isize) =
            chainPackedPointer;
    }

    // fill the buckets of the hash table
    for idx in ms.nextToUpdate..target {
        let h_0 = (ZSTD_hashPtr(
            base.offset(idx as isize) as *const core::ffi::c_void,
            hashLog,
            ms.cParams.minMatch,
        ) as u32)
            << ZSTD_LAZY_DDSS_BUCKET_LOG;
        let mut i_1: u32 = 0;
        i_1 = cacheSize.wrapping_sub(1);
        // Shift hash cache down 1
        while i_1 != 0 {
            *hashTable.offset(h_0.wrapping_add(i_1) as isize) =
                *hashTable.offset(h_0.wrapping_add(i_1).wrapping_sub(1) as isize);
            i_1 = i_1.wrapping_sub(1);
        }
        *hashTable.offset(h_0 as isize) = idx;
    }

    ms.nextToUpdate = target;
}

/// Returns the longest match length found in the dedicated dict search structure.
/// If none are longer than the argument ml, then ml will be returned.
#[inline(always)]
unsafe fn ZSTD_dedicatedDictSearch_lazy_search(
    offsetPtr: &mut size_t,
    mut ml: size_t,
    nbAttempts: u32,
    dms: *const ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    prefixStart: *const u8,
    curr: u32,
    dictLimit: u32,
    ddsIdx: size_t,
) -> size_t {
    let ddsLowestIndex = (*dms).window.dictLimit;
    let ddsBase = (*dms).window.base;
    let ddsEnd = (*dms).window.nextSrc;
    let ddsSize = ddsEnd.offset_from(ddsBase) as core::ffi::c_long as u32;
    let ddsIndexDelta = dictLimit.wrapping_sub(ddsSize);
    let bucketSize = (1 << ZSTD_LAZY_DDSS_BUCKET_LOG) as u32;
    let bucketLimit = nbAttempts.min(bucketSize.wrapping_sub(1));
    let mut ddsAttempt: u32 = 0;
    let mut matchIndex: u32 = 0;

    ddsAttempt = 0;
    while ddsAttempt < bucketSize.wrapping_sub(1) {
        ddsAttempt = ddsAttempt.wrapping_add(1);

        prefetch_read_data(
            ddsBase.add(*((*dms).hashTable).add(ddsIdx + ddsAttempt as usize) as usize),
            Locality::L1,
        );
    }

    {
        let chainPackedPointer =
            *((*dms).hashTable).add(ddsIdx.wrapping_add(bucketSize as size_t).wrapping_sub(1));
        let chainIndex = chainPackedPointer >> 8;

        prefetch_read_data(
            ((*dms).chainTable).offset(chainIndex as isize),
            Locality::L1,
        );
    }

    ddsAttempt = 0;
    while ddsAttempt < bucketLimit {
        let mut currentMl = 0;
        let mut match_0 = core::ptr::null::<u8>();
        matchIndex = *((*dms).hashTable).add(ddsIdx.wrapping_add(ddsAttempt as size_t));
        match_0 = ddsBase.offset(matchIndex as isize);

        if matchIndex == 0 {
            return ml;
        }

        // guaranteed by table construction
        assert!(matchIndex >= ddsLowestIndex);
        assert!(match_0.wrapping_add(4) <= ddsEnd);
        if MEM_read32(match_0 as *const core::ffi::c_void)
            == MEM_read32(ip as *const core::ffi::c_void)
        {
            // assumption: matchIndex <= dictLimit-4 (by table construction)
            currentMl =
                (ZSTD_count_2segments(ip.add(4), match_0.add(4), iLimit, ddsEnd, prefixStart))
                    .wrapping_add(4);
        }

        // save best solution
        if currentMl > ml {
            ml = currentMl;
            *offsetPtr = curr
                .wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.add(currentMl) == iLimit {
                // best possible, avoids read overflow on next attempt
                return ml;
            }
        }

        ddsAttempt = ddsAttempt.wrapping_add(1);
    }

    let chainPackedPointer_0 =
        *((*dms).hashTable).add(ddsIdx.wrapping_add(bucketSize as size_t).wrapping_sub(1));
    let mut chainIndex_0 = chainPackedPointer_0 >> 8;
    let chainLength = chainPackedPointer_0 & 0xff as core::ffi::c_int as u32;
    let chainAttempts = nbAttempts.wrapping_sub(ddsAttempt);
    let chainLimit = chainAttempts.min(chainLength);
    let mut chainAttempt: u32 = 0;

    chainAttempt = 0;
    while chainAttempt < chainLimit {
        chainAttempt = chainAttempt.wrapping_add(1);
    }

    chainAttempt = 0;
    while chainAttempt < chainLimit {
        let mut currentMl_0 = 0;
        let mut match_1 = core::ptr::null::<u8>();
        matchIndex = *((*dms).chainTable).offset(chainIndex_0 as isize);
        match_1 = ddsBase.offset(matchIndex as isize);

        if MEM_read32(match_1 as *const core::ffi::c_void)
            == MEM_read32(ip as *const core::ffi::c_void)
        {
            // assumption: matchIndex <= dictLimit-4 (by table construction)
            currentMl_0 =
                (ZSTD_count_2segments(ip.add(4), match_1.add(4), iLimit, ddsEnd, prefixStart))
                    .wrapping_add(4);
        }

        // save best solution
        if currentMl_0 > ml {
            ml = currentMl_0;
            *offsetPtr = curr
                .wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.add(currentMl_0) == iLimit {
                break; // best possible, avoids read overflow on next attempt
            }
        }
        chainAttempt = chainAttempt.wrapping_add(1);
        chainIndex_0 = chainIndex_0.wrapping_add(1);
    }

    ml
}

/// Update chains up to ip (excluded).
/// Assumption: always within prefix (i.e. not within extDict)
#[inline(always)]
unsafe fn ZSTD_insertAndFindFirstIndex_internal(
    ms: &mut ZSTD_MatchState_t,
    cParams: *const ZSTD_compressionParameters,
    ip: *const u8,
    mls: u32,
    lazySkipping: u32,
) -> u32 {
    let hashTable = ms.hashTable;
    let hashLog = (*cParams).hashLog;
    let chainTable = ms.chainTable;
    let chainMask = ((1 << (*cParams).chainLog) - 1) as u32;
    let base = ms.window.base;
    let target = ip.offset_from(base) as core::ffi::c_long as u32;

    for idx in ms.nextToUpdate..target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const core::ffi::c_void,
            hashLog,
            mls,
        );
        *chainTable.offset((idx & chainMask) as isize) = *hashTable.add(h);
        *hashTable.add(h) = idx;
        // Stop inserting every position when in the lazy skipping mode
        if lazySkipping != 0 {
            break;
        }
    }

    ms.nextToUpdate = target;
    *hashTable.add(ZSTD_hashPtr(ip as *const core::ffi::c_void, hashLog, mls))
}

pub unsafe fn ZSTD_insertAndFindFirstIndex(ms: &mut ZSTD_MatchState_t, ip: *const u8) -> u32 {
    let cParams = &ms.cParams;
    ZSTD_insertAndFindFirstIndex_internal(ms, cParams, ip, ms.cParams.minMatch, 0)
}

/// inlining is important to hardwire a hot branch (template emulation)
#[inline(always)]
unsafe fn ZSTD_HcFindBestMatch(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let cParams = &ms.cParams;
    let chainTable = ms.chainTable;
    let chainSize = (1 << cParams.chainLog) as u32;
    let chainMask = chainSize.wrapping_sub(1);
    let base = ms.window.base;
    let dictBase = ms.window.dictBase;
    let dictLimit = ms.window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictEnd = dictBase.offset(dictLimit as isize);
    let curr = ip.offset_from(base) as core::ffi::c_long as u32;
    let maxDistance = 1 << cParams.windowLog;
    let lowestValid = ms.window.lowLimit;
    let withinMaxDistance = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ms.loadedDictEnd != 0;
    let lowLimit = if isDictionary {
        lowestValid
    } else {
        withinMaxDistance
    };
    let minChain = if curr > chainSize {
        curr.wrapping_sub(chainSize)
    } else {
        0
    };
    let mut nbAttempts = (1 as core::ffi::c_uint) << cParams.searchLog;
    let mut ml = (4 - 1) as size_t;

    let dms = ms.dictMatchState;
    let ddsHashLog = if dictMode == ZSTD_dedicatedDictSearch {
        ((*dms).cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as core::ffi::c_uint)
    } else {
        0
    };
    let ddsIdx = if dictMode == ZSTD_dedicatedDictSearch {
        ZSTD_hashPtr(ip as *const core::ffi::c_void, ddsHashLog, mls) << ZSTD_LAZY_DDSS_BUCKET_LOG
    } else {
        0
    };

    let mut matchIndex: u32 = 0;

    if dictMode == ZSTD_dedicatedDictSearch {
        let entry: *const u32 = &mut *((*dms).hashTable).add(ddsIdx) as *mut u32;
        prefetch_read_data(entry, Locality::L1);
    }

    // HC4 match finder
    matchIndex =
        ZSTD_insertAndFindFirstIndex_internal(ms, cParams, ip, mls, ms.lazySkipping as u32);

    while (matchIndex >= lowLimit) && (nbAttempts > 0) {
        let mut currentMl = 0;
        if dictMode != ZSTD_extDict || matchIndex >= dictLimit {
            let match_0 = base.offset(matchIndex as isize);
            // read 4B starting from (match + ml + 1 - sizeof(U32))
            if MEM_read32(match_0.add(ml).sub(3) as *const core::ffi::c_void)
                == MEM_read32(ip.add(ml).sub(3) as *const core::ffi::c_void)
            {
                currentMl = ZSTD_count(ip, match_0, iLimit);
            }
        } else {
            let match_1 = dictBase.offset(matchIndex as isize);
            // assumption: matchIndex <= dictLimit-4 (by table construction)
            if MEM_read32(match_1 as *const core::ffi::c_void)
                == MEM_read32(ip as *const core::ffi::c_void)
            {
                currentMl =
                    (ZSTD_count_2segments(ip.add(4), match_1.add(4), iLimit, dictEnd, prefixStart))
                        .wrapping_add(4);
            }
        }

        // save best solution
        if currentMl > ml {
            ml = currentMl;
            *offsetPtr = curr
                .wrapping_sub(matchIndex)
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.add(currentMl) == iLimit {
                break; // best possible, avoids read overflow on next attempt
            }
        }

        if matchIndex <= minChain {
            break;
        }

        matchIndex = *chainTable.offset((matchIndex & chainMask) as isize);
        nbAttempts = nbAttempts.wrapping_sub(1);
    }

    if dictMode == ZSTD_dedicatedDictSearch {
        ml = ZSTD_dedicatedDictSearch_lazy_search(
            offsetPtr,
            ml,
            nbAttempts,
            dms,
            ip,
            iLimit,
            prefixStart,
            curr,
            dictLimit,
            ddsIdx,
        );
    } else if dictMode == ZSTD_dictMatchState {
        let dmsChainTable: *const u32 = (*dms).chainTable;
        let dmsChainSize = (1 << (*dms).cParams.chainLog) as u32;
        let dmsChainMask = dmsChainSize.wrapping_sub(1);
        let dmsLowestIndex = (*dms).window.dictLimit;
        let dmsBase = (*dms).window.base;
        let dmsEnd = (*dms).window.nextSrc;
        let dmsSize = dmsEnd.offset_from(dmsBase) as core::ffi::c_long as u32;
        let dmsIndexDelta = dictLimit.wrapping_sub(dmsSize);
        let dmsMinChain = if dmsSize > dmsChainSize {
            dmsSize.wrapping_sub(dmsChainSize)
        } else {
            0
        };

        matchIndex = *((*dms).hashTable).add(ZSTD_hashPtr(
            ip as *const core::ffi::c_void,
            (*dms).cParams.hashLog,
            mls,
        ));

        while (matchIndex >= dmsLowestIndex) && (nbAttempts > 0) {
            let mut currentMl_0 = 0;
            let match_2 = dmsBase.offset(matchIndex as isize);

            // assumption: matchIndex <= dictLimit-4 (by table construction)
            if MEM_read32(match_2 as *const core::ffi::c_void)
                == MEM_read32(ip as *const core::ffi::c_void)
            {
                currentMl_0 =
                    (ZSTD_count_2segments(ip.add(4), match_2.add(4), iLimit, dmsEnd, prefixStart))
                        .wrapping_add(4);
            }

            // save best solution
            if currentMl_0 > ml {
                ml = currentMl_0;
                *offsetPtr = curr
                    .wrapping_sub(matchIndex.wrapping_add(dmsIndexDelta))
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
                if ip.add(currentMl_0) == iLimit {
                    break;
                }
            }

            if matchIndex <= dmsMinChain {
                break;
            }

            matchIndex = *dmsChainTable.offset((matchIndex & dmsChainMask) as isize);
            nbAttempts = nbAttempts.wrapping_sub(1);
        }
    }

    ml
}

pub const ZSTD_ROW_HASH_TAG_MASK: core::ffi::c_uint =
    ((1 as core::ffi::c_uint) << ZSTD_ROW_HASH_TAG_BITS).wrapping_sub(1);
pub const ZSTD_ROW_HASH_CACHE_MASK: core::ffi::c_int = ZSTD_ROW_HASH_CACHE_SIZE - 1;

/// Starting from the LSB, returns the idx of the next non-zero bit.
/// Basically counting the number of trailing zeroes.
#[inline]
fn ZSTD_VecMask_next(val: ZSTD_VecMask) -> u32 {
    val.trailing_zeros()
}

/// Returns the next index to insert at within a tagTable row, and updates the "head"
/// value to reflect the update. Essentially cycles backwards from [1, {entries per row})
#[inline(always)]
unsafe fn ZSTD_row_nextIndex(tagRow: *mut u8, rowMask: u32) -> u32 {
    let mut next = (*tagRow as core::ffi::c_int - 1) as u32 & rowMask;
    next = next.wrapping_add(if next == 0 { rowMask } else { 0 }); // skip first position
    *tagRow = next as u8;
    next
}

/// Performs prefetching for the hashTable and tagTable at a given row.
#[inline(always)]
unsafe fn ZSTD_row_prefetch(hashTable: *const u32, tagTable: *const u8, relRow: u32, rowLog: u32) {
    prefetch_read_data(hashTable.add(relRow as usize), Locality::L1);

    if rowLog >= 5 {
        // Note: prefetching more of the hash table does not appear to be beneficial for 128-entry rows.
        prefetch_read_data(hashTable.add(relRow as usize + 16), Locality::L1);
    }
    prefetch_read_data(tagTable.add(relRow as usize), Locality::L1);
    if rowLog == 6 {
        prefetch_read_data(tagTable.add(relRow as usize + 32), Locality::L1);
    }

    assert!(rowLog == 4 || rowLog == 5 || rowLog == 6);
    // Prefetched hash row always 64-byte aligned.
    assert!((hashTable.wrapping_add(relRow as usize) as usize).is_multiple_of(64));
    // Prefetched tagRow sits on correct multiple of bytes (32,64,128).
    assert!((tagTable.wrapping_add(relRow as usize) as usize).is_multiple_of(1 << rowLog));
}

/// Fill up the hash cache starting at idx, prefetching up to ZSTD_ROW_HASH_CACHE_SIZE entries,
/// but not beyond iLimit.
#[inline(always)]
unsafe fn ZSTD_row_fillHashCache(
    ms: &mut ZSTD_MatchState_t,
    base: *const u8,
    rowLog: u32,
    mls: u32,
    idx: u32,
    iLimit: *const u8,
) {
    let hashTable: *const u32 = ms.hashTable;
    let tagTable: *const u8 = ms.tagTable;
    let hashLog = ms.rowHashLog;
    let maxElemsToPrefetch = if base.offset(idx as isize) > iLimit {
        0
    } else {
        (iLimit.offset_from(base.offset(idx as isize)) as core::ffi::c_long + 1) as u32
    };
    let lim = idx.wrapping_add(if (8) < maxElemsToPrefetch {
        8
    } else {
        maxElemsToPrefetch
    });

    for idx in idx..lim {
        let hash = ZSTD_hashPtrSalted(
            base.offset(idx as isize) as *const core::ffi::c_void,
            hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
            mls,
            ms.hashSalt,
        ) as u32;
        let row = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        ZSTD_row_prefetch(hashTable, tagTable, row, rowLog);
        ms.hashCache[(idx & ZSTD_ROW_HASH_CACHE_MASK as u32) as usize] = hash;
    }
}

/// Returns the hash of base + idx, and replaces the hash in the hash cache with the byte at
/// base + idx + ZSTD_ROW_HASH_CACHE_SIZE. Also prefetches the appropriate rows from hashTable and tagTable.
#[inline(always)]
unsafe fn ZSTD_row_nextCachedHash(
    cache: *mut u32,
    hashTable: *const u32,
    tagTable: *const u8,
    base: *const u8,
    idx: u32,
    hashLog: u32,
    rowLog: u32,
    mls: u32,
    hashSalt: u64,
) -> u32 {
    let newHash = ZSTD_hashPtrSalted(
        base.offset(idx as isize)
            .offset(ZSTD_ROW_HASH_CACHE_SIZE as isize) as *const core::ffi::c_void,
        hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
        mls,
        hashSalt,
    ) as u32;
    let row = newHash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
    ZSTD_row_prefetch(hashTable, tagTable, row, rowLog);

    let hash = *cache.offset((idx & ZSTD_ROW_HASH_CACHE_MASK as u32) as isize);
    *cache.offset((idx & ZSTD_ROW_HASH_CACHE_MASK as u32) as isize) = newHash;
    hash
}

/// Updates the hash table with positions starting from updateStartIdx until updateEndIdx.
#[inline(always)]
unsafe fn ZSTD_row_update_internalImpl(
    ms: &mut ZSTD_MatchState_t,
    mut updateStartIdx: u32,
    updateEndIdx: u32,
    mls: u32,
    rowLog: u32,
    rowMask: u32,
    useCache: bool,
) {
    let hashTable = ms.hashTable;
    let tagTable = ms.tagTable;
    let hashLog = ms.rowHashLog;
    let base = ms.window.base;

    while updateStartIdx < updateEndIdx {
        let hash = if useCache {
            ZSTD_row_nextCachedHash(
                (ms.hashCache).as_mut_ptr(),
                hashTable,
                tagTable,
                base,
                updateStartIdx,
                hashLog,
                rowLog,
                mls,
                ms.hashSalt,
            )
        } else {
            ZSTD_hashPtrSalted(
                base.offset(updateStartIdx as isize) as *const core::ffi::c_void,
                hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
                mls,
                ms.hashSalt,
            ) as u32
        };
        let relRow = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        let row = hashTable.offset(relRow as isize);
        let tagRow = tagTable.offset(relRow as isize);
        let pos = ZSTD_row_nextIndex(tagRow, rowMask);

        *tagRow.offset(pos as isize) = (hash & ZSTD_ROW_HASH_TAG_MASK) as u8;
        *row.offset(pos as isize) = updateStartIdx;
        updateStartIdx = updateStartIdx.wrapping_add(1);
    }
}

/// Inserts the byte at ip into the appropriate position in the hash table, and updates ms->nextToUpdate.
/// Skips sections of long matches as is necessary.
#[inline(always)]
unsafe fn ZSTD_row_update_internal(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    mls: u32,
    rowLog: u32,
    rowMask: u32,
    useCache: bool,
) {
    let mut idx = ms.nextToUpdate;
    let base = ms.window.base;
    let target = ip.offset_from(base) as core::ffi::c_long as u32;
    let kSkipThreshold = 384;
    let kMaxMatchStartPositionsToUpdate = 96;
    let kMaxMatchEndPositionsToUpdate = 32;

    if useCache && target.wrapping_sub(idx) > kSkipThreshold {
        // Only skip positions when using hash cache, i.e.
        // if we are loading a dict, don't skip anything.
        // If we decide to skip, then we only update a set number
        // of positions at the beginning and end of the match.
        let bound = idx.wrapping_add(kMaxMatchStartPositionsToUpdate);
        ZSTD_row_update_internalImpl(ms, idx, bound, mls, rowLog, rowMask, useCache);
        idx = target.wrapping_sub(kMaxMatchEndPositionsToUpdate);
        ZSTD_row_fillHashCache(ms, base, rowLog, mls, idx, ip.add(1));
    }

    ZSTD_row_update_internalImpl(ms, idx, target, mls, rowLog, rowMask, useCache);
    ms.nextToUpdate = target;
}

/// External wrapper for ZSTD_row_update_internal(). Used for filling the hashtable during dictionary
/// processing.
pub unsafe fn ZSTD_row_update(ms: &mut ZSTD_MatchState_t, ip: *const u8) {
    let rowLog = ms.cParams.searchLog.clamp(4, 6);
    let rowMask = ((1 as core::ffi::c_uint) << rowLog).wrapping_sub(1);
    let mls = ms.cParams.minMatch.min(6);

    ZSTD_row_update_internal(ms, ip, mls, rowLog, rowMask, false);
}

/// Returns the mask width of bits group of which will be set to 1. Given not all
/// architectures have easy movemask instruction, this helps to iterate over
/// groups of bits easier and faster.
#[inline(always)]
fn ZSTD_row_matchMaskGroupWidth(_rowEntries: u32) -> u32 {
    // FIXME: add a more optimal implementation for aarch64.
    1
}

#[inline(always)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn ZSTD_row_getSSEMask(
    nbChunks: core::ffi::c_int,
    src: *const u8,
    tag: u8,
    head: u32,
) -> ZSTD_VecMask {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::{
        __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8,
    };
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::{
        __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8,
    };

    let comparisonMask = _mm_set1_epi8(tag as core::ffi::c_char);
    let mut matches: [core::ffi::c_int; 4] = [0; 4];
    for i in 0..nbChunks {
        let chunk = _mm_loadu_si128(
            src.offset((16 * i) as isize) as *const core::ffi::c_void as *const __m128i
        );
        let equalMask = _mm_cmpeq_epi8(chunk, comparisonMask);
        *matches.as_mut_ptr().offset(i as isize) = _mm_movemask_epi8(equalMask);
    }
    if nbChunks == 1 {
        return (matches[0] as u16).rotate_right(head) as ZSTD_VecMask;
    }
    if nbChunks == 2 {
        return ((matches[1] as u32) << 16 | matches[0] as u32).rotate_right(head) as ZSTD_VecMask;
    }
    ((matches[3] as u64) << 48
        | (matches[2] as u64) << 32
        | (matches[1] as u64) << 16
        | matches[0] as u64)
        .rotate_right(head)
}

/// Returns a ZSTD_VecMask (U64) that has the nth group (determined by
/// ZSTD_row_matchMaskGroupWidth) of bits set to 1 if the newly-computed "tag"
/// matches the hash at the nth position in a row of the tagTable.
/// Each row is a circular buffer beginning at the value of "headGrouped". So we
/// must rotate the "matches" bitfield to match up with the actual layout of the
/// entries within the hashTable.
#[inline(always)]
unsafe fn ZSTD_row_getMatchMask(
    tagRow: *const u8,
    tag: u8,
    headGrouped: u32,
    rowEntries: u32,
) -> ZSTD_VecMask {
    let src = tagRow;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if true {
        return ZSTD_row_getSSEMask((rowEntries / 16) as core::ffi::c_int, src, tag, headGrouped);
    }

    // FIXME: Evaluate if the custom SIMD implementation is worth it on x86, and if so also add an
    // aarch64 implementation.

    // Fallback using Simd Within A Register (SWAR).

    let chunkSize = size_of::<usize>();
    let shiftAmount = (chunkSize * 8) - chunkSize;
    let xFF = usize::MAX;
    let x01 = xFF / 0xFF;
    let x80 = x01 << 7;
    let splatChar = usize::from(tag) * x01;

    let mut matches: ZSTD_VecMask = 0;
    let mut i = rowEntries as isize - chunkSize as isize;
    assert!((size_of::<usize>() == 4) || (size_of::<usize>() == 8));

    if cfg!(target_endian = "little") {
        let extractMagic = (xFF / 0x7F) >> chunkSize;

        loop {
            let mut chunk = src.offset(i).cast::<usize>().read_unaligned();
            chunk ^= splatChar;
            chunk = (((chunk | x80) - x01) | chunk) & x80;
            matches <<= chunkSize;
            matches |= ((chunk.wrapping_mul(extractMagic)) >> shiftAmount) as ZSTD_VecMask;
            i -= chunkSize as isize;

            if i < 0 {
                break;
            }
        }
    } else {
        // big endian: reverse bits during extraction.
        let msb = xFF ^ (xFF >> 1);
        let extractMagic = (msb / 0x1FF) | msb;

        loop {
            let mut chunk = src.offset(i).cast::<usize>().read_unaligned();
            chunk ^= splatChar;
            chunk = (((chunk | x80) - x01) | chunk) & x80;
            matches <<= chunkSize;
            matches |= (((chunk >> 7) * extractMagic) >> shiftAmount) as ZSTD_VecMask;
            i -= chunkSize as isize;

            if i < 0 {
                break;
            }
        }
    }

    matches = !matches;
    match rowEntries {
        16 => (matches as u16).rotate_right(headGrouped) as ZSTD_VecMask,
        32 => (matches as u32).rotate_right(headGrouped) as ZSTD_VecMask,
        64 => (matches as u64).rotate_right(headGrouped) as ZSTD_VecMask,
        _ => unreachable!(),
    }
}

// The high-level approach of the SIMD row based match finder is as follows:
// - Figure out where to insert the new entry:
//      - Generate a hash for current input position and split it into a one byte of tag and `rowHashLog` bits of index.
//           - The hash is salted by a value that changes on every context reset, so when the same table is used
//             we will avoid collisions that would otherwise slow us down by introducing phantom matches.
//      - The hashTable is effectively split into groups or "rows" of 15 or 31 entries of U32, and the index determines
//        which row to insert into.
//      - Determine the correct position within the row to insert the entry into. Each row of 15 or 31 can
//        be considered as a circular buffer with a "head" index that resides in the tagTable (overall 16 or 32 bytes
//        per row).
// - Use SIMD to efficiently compare the tags in the tagTable to the 1-byte tag calculated for the position and
//   generate a bitfield that we can cycle through to check the collisions in the hash table.
// - Pick the longest match.
// - Insert the tag into the equivalent row and position in the tagTable.
#[inline(always)]
unsafe fn ZSTD_RowFindBestMatch(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
    rowLog: u32,
) -> size_t {
    let hashTable = ms.hashTable;
    let tagTable = ms.tagTable;
    let hashCache = (ms.hashCache).as_mut_ptr();
    let hashLog = ms.rowHashLog;
    let cParams = &ms.cParams;
    let base = ms.window.base;
    let dictBase = ms.window.dictBase;
    let dictLimit = ms.window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictEnd = dictBase.offset(dictLimit as isize);
    let curr = ip.offset_from(base) as core::ffi::c_long as u32;
    let maxDistance = 1 << cParams.windowLog;
    let lowestValid = ms.window.lowLimit;
    let withinMaxDistance = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ms.loadedDictEnd != 0;
    let lowLimit = if isDictionary {
        lowestValid
    } else {
        withinMaxDistance
    };
    let rowEntries = (1 as core::ffi::c_uint) << rowLog;
    let rowMask = rowEntries.wrapping_sub(1);
    let cappedSearchLog = cParams.searchLog.min(rowLog);
    let groupWidth = ZSTD_row_matchMaskGroupWidth(rowEntries);
    let hashSalt = ms.hashSalt;
    let mut nbAttempts = (1 as core::ffi::c_uint) << cappedSearchLog;
    let mut ml = (4 - 1) as size_t;
    let mut hash: u32 = 0;

    // DMS/DDS variables that may be referenced later
    let dms = ms.dictMatchState;

    // Initialize the following variables to satisfy static analyzer
    let mut ddsIdx = 0;
    let mut ddsExtraAttempts = 0; // cctx hash tables are limited in searches, but allow extra searches into DDS
    let mut dmsTag = 0;
    let mut dmsRow = core::ptr::null_mut();
    let mut dmsTagRow = core::ptr::null_mut();

    if dictMode == ZSTD_dedicatedDictSearch {
        let ddsHashLog =
            ((*dms).cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as core::ffi::c_uint);
        {
            /* Prefetch DDS hashtable entry */
            ddsIdx = ZSTD_hashPtr(ip as *const core::ffi::c_void, ddsHashLog, mls)
                << ZSTD_LAZY_DDSS_BUCKET_LOG;
            prefetch_read_data(((*dms).hashTable).add(ddsIdx), Locality::L1);
        }
        ddsExtraAttempts = if cParams.searchLog > rowLog {
            1 << (cParams.searchLog).wrapping_sub(rowLog)
        } else {
            0
        };
    }

    if dictMode == ZSTD_dictMatchState {
        // Prefetch DMS rows
        let dmsHashTable = (*dms).hashTable;
        let dmsTagTable = (*dms).tagTable;
        let dmsHash = ZSTD_hashPtr(
            ip as *const core::ffi::c_void,
            ((*dms).rowHashLog).wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
            mls,
        ) as u32;
        let dmsRelRow = dmsHash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        dmsTag = dmsHash & ZSTD_ROW_HASH_TAG_MASK;
        dmsTagRow = dmsTagTable.offset(dmsRelRow as isize);
        dmsRow = dmsHashTable.offset(dmsRelRow as isize);
        ZSTD_row_prefetch(dmsHashTable, dmsTagTable, dmsRelRow, rowLog);
    }

    // Update the hashTable and tagTable up to (but not including) ip
    if ms.lazySkipping == 0 {
        ZSTD_row_update_internal(ms, ip, mls, rowLog, rowMask, true);
        hash = ZSTD_row_nextCachedHash(
            hashCache, hashTable, tagTable, base, curr, hashLog, rowLog, mls, hashSalt,
        );
    } else {
        // Stop inserting every position when in the lazy skipping mode.
        // The hash cache is also not kept up to date in this mode.
        hash = ZSTD_hashPtrSalted(
            ip as *const core::ffi::c_void,
            hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
            mls,
            hashSalt,
        ) as u32;
        ms.nextToUpdate = curr;
    }
    ms.hashSaltEntropy = (ms.hashSaltEntropy).wrapping_add(hash); // collect salt entropy

    // Get the hash for ip, compute the appropriate row
    let relRow = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
    let tag = hash & ZSTD_ROW_HASH_TAG_MASK;
    let row = hashTable.offset(relRow as isize);
    let tagRow = tagTable.offset(relRow as isize);
    let headGrouped = (*tagRow as u32 & rowMask) * groupWidth;
    let mut matchBuffer: [u32; 64] = [0; 64];
    let mut numMatches = 0usize;
    let mut currMatch = 0;
    let mut matches = ZSTD_row_getMatchMask(tagRow, tag as u8, headGrouped, rowEntries);

    // Cycle through the matches and prefetch
    while matches > 0 && nbAttempts > 0 {
        let matchPos =
            (headGrouped.wrapping_add(ZSTD_VecMask_next(matches)) / groupWidth) & rowMask;
        let matchIndex = *row.offset(matchPos as isize);
        if matchPos != 0 {
            if matchIndex < lowLimit {
                break;
            }

            if dictMode != ZSTD_extDict || matchIndex >= dictLimit {
                prefetch_read_data(base.add(matchIndex as usize), Locality::L1);
            } else {
                prefetch_read_data(dictBase.add(matchIndex as usize), Locality::L1);
            }

            *matchBuffer.as_mut_ptr().add(numMatches) = matchIndex;
            numMatches = numMatches.wrapping_add(1);
            nbAttempts = nbAttempts.wrapping_sub(1);
        }
        matches &= matches.wrapping_sub(1);
    }

    // Speed opt: insert current byte into hashtable too. This allows us to avoid one iteration of the loop
    // in ZSTD_row_update_internal() at the next search.
    let pos = ZSTD_row_nextIndex(tagRow, rowMask);
    *tagRow.offset(pos as isize) = tag as u8;
    *row.offset(pos as isize) = ms.nextToUpdate;
    ms.nextToUpdate = (ms.nextToUpdate).wrapping_add(1);

    // Return the longest match
    while currMatch < numMatches {
        let matchIndex_0 = *matchBuffer.as_mut_ptr().add(currMatch);
        let mut currentMl = 0;

        if dictMode != ZSTD_extDict || matchIndex_0 >= dictLimit {
            let match_0 = base.offset(matchIndex_0 as isize);
            // read 4B starting from (match + ml + 1 - sizeof(U32))
            if MEM_read32(match_0.add(ml).sub(3) as *const core::ffi::c_void)
                == MEM_read32(ip.add(ml).sub(3) as *const core::ffi::c_void)
            {
                currentMl = ZSTD_count(ip, match_0, iLimit);
            }
        } else {
            let match_1 = dictBase.offset(matchIndex_0 as isize);
            if MEM_read32(match_1 as *const core::ffi::c_void)
                == MEM_read32(ip as *const core::ffi::c_void)
            {
                // assumption: matchIndex <= dictLimit-4 (by table construction)
                currentMl =
                    (ZSTD_count_2segments(ip.add(4), match_1.add(4), iLimit, dictEnd, prefixStart))
                        .wrapping_add(4);
            }
        }

        // Save best solution
        if currentMl > ml {
            ml = currentMl;
            *offsetPtr = curr
                .wrapping_sub(matchIndex_0)
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.add(currentMl) == iLimit {
                break; // best possible, avoids read overflow on next attempt
            }
        }
        currMatch = currMatch.wrapping_add(1);
    }

    if dictMode == ZSTD_dedicatedDictSearch {
        ml = ZSTD_dedicatedDictSearch_lazy_search(
            offsetPtr,
            ml,
            nbAttempts.wrapping_add(ddsExtraAttempts),
            dms,
            ip,
            iLimit,
            prefixStart,
            curr,
            dictLimit,
            ddsIdx,
        );
    } else if dictMode == ZSTD_dictMatchState {
        // TODO: Measure and potentially add prefetching to DMS
        let dmsLowestIndex = (*dms).window.dictLimit;
        let dmsBase = (*dms).window.base;
        let dmsEnd = (*dms).window.nextSrc;
        let dmsSize = dmsEnd.offset_from(dmsBase) as core::ffi::c_long as u32;
        let dmsIndexDelta = dictLimit.wrapping_sub(dmsSize);

        let headGrouped_0 = (*dmsTagRow as u32 & rowMask) * groupWidth;
        let mut matchBuffer_0: [u32; 64] = [0; 64];
        let mut numMatches_0 = 0usize;
        let mut currMatch_0 = 0;
        let mut matches_0 =
            ZSTD_row_getMatchMask(dmsTagRow, dmsTag as u8, headGrouped_0, rowEntries);

        while matches_0 > 0 && nbAttempts > 0 {
            let matchPos_0 =
                (headGrouped_0.wrapping_add(ZSTD_VecMask_next(matches_0)) / groupWidth) & rowMask;
            let matchIndex_1 = *dmsRow.offset(matchPos_0 as isize);
            if matchPos_0 != 0 {
                if matchIndex_1 < dmsLowestIndex {
                    break;
                }
                *matchBuffer_0.as_mut_ptr().add(numMatches_0) = matchIndex_1;
                numMatches_0 = numMatches_0.wrapping_add(1);
                nbAttempts = nbAttempts.wrapping_sub(1);
            }
            matches_0 &= matches_0.wrapping_sub(1);
        }

        // Return the longest match
        while currMatch_0 < numMatches_0 {
            let matchIndex_2 = *matchBuffer_0.as_mut_ptr().add(currMatch_0);
            let mut currentMl_0 = 0;

            let match_2 = dmsBase.offset(matchIndex_2 as isize);
            if MEM_read32(match_2 as *const core::ffi::c_void)
                == MEM_read32(ip as *const core::ffi::c_void)
            {
                currentMl_0 =
                    (ZSTD_count_2segments(ip.add(4), match_2.add(4), iLimit, dmsEnd, prefixStart))
                        .wrapping_add(4);
            }

            if currentMl_0 > ml {
                ml = currentMl_0;
                *offsetPtr = curr
                    .wrapping_sub(matchIndex_2.wrapping_add(dmsIndexDelta))
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
                if ip.add(currentMl_0) == iLimit {
                    break;
                }
            }

            currMatch_0 = currMatch_0.wrapping_add(1);
        }
    }

    ml
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_6_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dictMatchState, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_4_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_extDict, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_4_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dictMatchState, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dedicatedDictSearch, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_6_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_extDict, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_6_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_extDict, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_6_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_extDict, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_5_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_extDict, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_5_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_extDict, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_5_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_extDict, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_4_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dictMatchState, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_4_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_extDict, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_extDict_4_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_extDict, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dedicatedDictSearch, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dedicatedDictSearch, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dedicatedDictSearch, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dedicatedDictSearch, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dedicatedDictSearch, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dedicatedDictSearch, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_6_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_noDict, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dedicatedDictSearch, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_6_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_noDict, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_5_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_noDict, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_5_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_noDict, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_5_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_noDict, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_4_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_noDict, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_4_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_noDict, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_4_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_noDict, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dedicatedDictSearch, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_4_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dictMatchState, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_6_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dictMatchState, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_6_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dictMatchState, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_5_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dictMatchState, 6)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_5_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dictMatchState, 5)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_dictMatchState_5_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dictMatchState, 4)
}

#[inline(never)]
unsafe fn ZSTD_RowFindBestMatch_noDict_6_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_noDict, 5)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_noDict_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 6, ZSTD_noDict)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_dictMatchState_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 6, ZSTD_dictMatchState)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_noDict_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 5, ZSTD_noDict)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_dedicatedDictSearch_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 5, ZSTD_dedicatedDictSearch)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_dedicatedDictSearch_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 6, ZSTD_dedicatedDictSearch)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_dedicatedDictSearch_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 4, ZSTD_dedicatedDictSearch)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_extDict_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 4, ZSTD_extDict)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_dictMatchState_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 4, ZSTD_dictMatchState)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_extDict_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 6, ZSTD_extDict)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_noDict_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 4, ZSTD_noDict)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_extDict_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 5, ZSTD_extDict)
}

#[inline(never)]
unsafe fn ZSTD_BtFindBestMatch_dictMatchState_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offBasePtr: &mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(ms, ip, iLimit, offBasePtr, 5, ZSTD_dictMatchState)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_noDict_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_noDict)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_dictMatchState_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dictMatchState)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_dedicatedDictSearch_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dedicatedDictSearch)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_dictMatchState_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dictMatchState)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_dedicatedDictSearch_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_dedicatedDictSearch)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_dedicatedDictSearch_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_dedicatedDictSearch)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_noDict_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_noDict)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_dictMatchState_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_dictMatchState)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_noDict_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_noDict)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_extDict_6(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 6, ZSTD_extDict)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_extDict_5(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 5, ZSTD_extDict)
}

#[inline(never)]
unsafe fn ZSTD_HcFindBestMatch_extDict_4(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    offsetPtr: &mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(ms, ip, iLimit, offsetPtr, 4, ZSTD_extDict)
}

/// Searches for the longest match at @p ip.
/// Dispatches to the correct implementation function based on the
/// (searchMethod, dictMode, mls, rowLog). We use switch statements
/// here instead of using an indirect function call through a function
/// pointer because after Spectre and Meltdown mitigations, indirect
/// function calls can be very costly, especially in the kernel.
///
/// NOTE: dictMode and searchMethod should be templated, so those switch
/// statements should be optimized out. Only the mls & rowLog switches
/// should be left.
///
/// @param ms The match state.
/// @param ip The position to search at.
/// @param iend The end of the input data.
/// @param[out] offsetPtr Stores the match offset into this pointer.
/// @param mls The minimum search length, in the range [4, 6].
/// @param rowLog The row log (if applicable), in the range [4, 6].
/// @param searchMethod The search method to use (templated).
/// @param dictMode The dictMode (templated).
///
/// # Returns
///
/// The length of the longest match found, or < mls if no match is found.
/// If a match is found its offset is stored in @p offsetPtr.
#[inline(always)]
unsafe fn ZSTD_searchMax(
    ms: &mut ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    offsetPtr: &mut size_t,
    mls: u32,
    rowLog: u32,
    searchMethod: searchMethod_e,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    if dictMode == ZSTD_noDict {
        match searchMethod as core::ffi::c_uint {
            0 => match mls {
                4 => return ZSTD_HcFindBestMatch_noDict_4(ms, ip, iend, offsetPtr),
                5 => return ZSTD_HcFindBestMatch_noDict_5(ms, ip, iend, offsetPtr),
                6 => return ZSTD_HcFindBestMatch_noDict_6(ms, ip, iend, offsetPtr),
                _ => {}
            },
            1 => match mls {
                4 => return ZSTD_BtFindBestMatch_noDict_4(ms, ip, iend, offsetPtr),
                5 => return ZSTD_BtFindBestMatch_noDict_5(ms, ip, iend, offsetPtr),
                6 => return ZSTD_BtFindBestMatch_noDict_6(ms, ip, iend, offsetPtr),
                _ => {}
            },
            2 => match mls {
                4 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_noDict_4_4(ms, ip, iend, offsetPtr);
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_noDict_4_5(ms, ip, iend, offsetPtr);
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_noDict_4_6(ms, ip, iend, offsetPtr);
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                5 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_noDict_5_4(ms, ip, iend, offsetPtr);
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_noDict_5_5(ms, ip, iend, offsetPtr);
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_noDict_5_6(ms, ip, iend, offsetPtr);
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                6 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_noDict_6_4(ms, ip, iend, offsetPtr);
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_noDict_6_5(ms, ip, iend, offsetPtr);
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_noDict_6_6(ms, ip, iend, offsetPtr);
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                _ => {}
            },
            _ => {}
        }
        unreachable!();
    } else if dictMode == ZSTD_extDict {
        match searchMethod as core::ffi::c_uint {
            0 => match mls {
                4 => return ZSTD_HcFindBestMatch_extDict_4(ms, ip, iend, offsetPtr),
                5 => return ZSTD_HcFindBestMatch_extDict_5(ms, ip, iend, offsetPtr),
                6 => return ZSTD_HcFindBestMatch_extDict_6(ms, ip, iend, offsetPtr),
                _ => {}
            },
            1 => match mls {
                4 => return ZSTD_BtFindBestMatch_extDict_4(ms, ip, iend, offsetPtr),
                5 => return ZSTD_BtFindBestMatch_extDict_5(ms, ip, iend, offsetPtr),
                6 => return ZSTD_BtFindBestMatch_extDict_6(ms, ip, iend, offsetPtr),
                _ => {}
            },
            2 => match mls {
                4 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_extDict_4_4(ms, ip, iend, offsetPtr);
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_extDict_4_5(ms, ip, iend, offsetPtr);
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_extDict_4_6(ms, ip, iend, offsetPtr);
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                5 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_extDict_5_4(ms, ip, iend, offsetPtr);
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_extDict_5_5(ms, ip, iend, offsetPtr);
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_extDict_5_6(ms, ip, iend, offsetPtr);
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                6 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_extDict_6_4(ms, ip, iend, offsetPtr);
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_extDict_6_5(ms, ip, iend, offsetPtr);
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_extDict_6_6(ms, ip, iend, offsetPtr);
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                _ => {}
            },
            _ => {}
        }
        unreachable!();
    } else if dictMode == ZSTD_dictMatchState {
        match searchMethod as core::ffi::c_uint {
            0 => match mls {
                4 => {
                    return ZSTD_HcFindBestMatch_dictMatchState_4(ms, ip, iend, offsetPtr);
                }
                5 => {
                    return ZSTD_HcFindBestMatch_dictMatchState_5(ms, ip, iend, offsetPtr);
                }
                6 => {
                    return ZSTD_HcFindBestMatch_dictMatchState_6(ms, ip, iend, offsetPtr);
                }
                _ => {}
            },
            1 => match mls {
                4 => {
                    return ZSTD_BtFindBestMatch_dictMatchState_4(ms, ip, iend, offsetPtr);
                }
                5 => {
                    return ZSTD_BtFindBestMatch_dictMatchState_5(ms, ip, iend, offsetPtr);
                }
                6 => {
                    return ZSTD_BtFindBestMatch_dictMatchState_6(ms, ip, iend, offsetPtr);
                }
                _ => {}
            },
            2 => match mls {
                4 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_4_4(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_4_5(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_4_6(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                5 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_5_4(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_5_5(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_5_6(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                6 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_6_4(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_6_5(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_dictMatchState_6_6(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                _ => {}
            },
            _ => {}
        }
        unreachable!();
    } else if dictMode == ZSTD_dedicatedDictSearch {
        match searchMethod as core::ffi::c_uint {
            0 => match mls {
                4 => {
                    return ZSTD_HcFindBestMatch_dedicatedDictSearch_4(ms, ip, iend, offsetPtr);
                }
                5 => {
                    return ZSTD_HcFindBestMatch_dedicatedDictSearch_5(ms, ip, iend, offsetPtr);
                }
                6 => {
                    return ZSTD_HcFindBestMatch_dedicatedDictSearch_6(ms, ip, iend, offsetPtr);
                }
                _ => {}
            },
            1 => match mls {
                4 => {
                    return ZSTD_BtFindBestMatch_dedicatedDictSearch_4(ms, ip, iend, offsetPtr);
                }
                5 => {
                    return ZSTD_BtFindBestMatch_dedicatedDictSearch_5(ms, ip, iend, offsetPtr);
                }
                6 => {
                    return ZSTD_BtFindBestMatch_dedicatedDictSearch_6(ms, ip, iend, offsetPtr);
                }
                _ => {}
            },
            2 => match mls {
                4 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_4_4(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_4_5(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_4_6(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                5 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_5_4(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_5_5(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_5_6(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                6 => {
                    match rowLog {
                        4 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_6_4(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        5 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_6_5(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        6 => {
                            return ZSTD_RowFindBestMatch_dedicatedDictSearch_6_6(
                                ms, ip, iend, offsetPtr,
                            );
                        }
                        _ => {}
                    }
                    unreachable!();
                }
                _ => {}
            },
            _ => {}
        }
        unreachable!();
    }
    unreachable!();
}

#[inline(always)]
unsafe fn ZSTD_compressBlock_lazy_generic(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    searchMethod: searchMethod_e,
    depth: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let mut current_block: u64;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.add(srcSize);
    let ilimit = if searchMethod == search_rowHash {
        iend.sub(8).sub(ZSTD_ROW_HASH_CACHE_SIZE as usize)
    } else {
        iend.sub(8)
    };
    let base = ms.window.base;
    let prefixLowestIndex = ms.window.dictLimit;
    let prefixLowest = base.offset(prefixLowestIndex as isize);
    let mls = ms.cParams.minMatch.clamp(4, 6);
    let rowLog = ms.cParams.searchLog.clamp(4, 6);

    let mut offset_1 = *rep;
    let mut offset_2 = *rep.add(1);
    let mut offsetSaved1 = 0;
    let mut offsetSaved2 = 0;

    let isDMS = dictMode == ZSTD_dictMatchState;
    let isDDS = dictMode == ZSTD_dedicatedDictSearch;
    let isDxS = isDMS || isDDS;
    let dms = ms.dictMatchState;
    let dictLowestIndex = if isDxS { (*dms).window.dictLimit } else { 0 };
    let dictBase = if isDxS {
        (*dms).window.base
    } else {
        core::ptr::null()
    };
    let dictLowest = if isDxS {
        dictBase.offset(dictLowestIndex as isize)
    } else {
        core::ptr::null()
    };
    let dictEnd = if isDxS {
        (*dms).window.nextSrc
    } else {
        core::ptr::null()
    };
    let dictIndexDelta = if isDxS {
        prefixLowestIndex.wrapping_sub(dictEnd.offset_from(dictBase) as core::ffi::c_long as u32)
    } else {
        0
    };
    let dictAndPrefixLength = (ip.offset_from(prefixLowest) as core::ffi::c_long
        + dictEnd.offset_from(dictLowest) as core::ffi::c_long)
        as u32;

    ip = ip.offset((dictAndPrefixLength == 0) as core::ffi::c_int as isize);
    if dictMode == ZSTD_noDict {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let windowLow = ZSTD_getLowestPrefixIndex(ms, curr, ms.cParams.windowLog);
        let maxRep = curr.wrapping_sub(windowLow);
        if offset_2 > maxRep {
            offsetSaved2 = offset_2;
            offset_2 = 0;
        }
        if offset_1 > maxRep {
            offsetSaved1 = offset_1;
            offset_1 = 0;
        }
    }

    if isDxS {
        // dictMatchState repCode checks don't currently handle repCode == 0 disabling.
        assert!(offset_1 <= dictAndPrefixLength);
        assert!(offset_2 <= dictAndPrefixLength);
    }

    // Reset the lazy skipping state
    ms.lazySkipping = 0;

    if searchMethod == search_rowHash {
        ZSTD_row_fillHashCache(ms, base, rowLog, mls, ms.nextToUpdate, ilimit);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    asm!(".p2align 5", options(preserves_flags));

    // Match Loop
    while ip < ilimit {
        let mut matchLength = 0;
        let mut offBase = REPCODE1_TO_OFFBASE as size_t;
        let mut start = ip.add(1);

        // check repCode
        if isDxS {
            let repIndex = (ip.offset_from(base) as core::ffi::c_long as u32)
                .wrapping_add(1)
                .wrapping_sub(offset_1);
            let repMatch = if (dictMode == ZSTD_dictMatchState
                || dictMode == ZSTD_dedicatedDictSearch)
                && repIndex < prefixLowestIndex
            {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
            } else {
                base.offset(repIndex as isize)
            };
            if ZSTD_index_overlap_check(prefixLowestIndex, repIndex)
                && MEM_read32(repMatch as *const core::ffi::c_void)
                    == MEM_read32(ip.add(1) as *const core::ffi::c_void)
            {
                let repMatchEnd = if repIndex < prefixLowestIndex {
                    dictEnd
                } else {
                    iend
                };
                matchLength = (ZSTD_count_2segments(
                    ip.add(1).add(4),
                    repMatch.add(4),
                    iend,
                    repMatchEnd,
                    prefixLowest,
                ))
                .wrapping_add(4);
                if depth == 0 {
                    current_block = 9173645608424642017;
                } else {
                    current_block = 14136749492126903395;
                }
            } else {
                current_block = 14136749492126903395;
            }
        } else {
            current_block = 14136749492126903395;
        }
        if current_block == 14136749492126903395 {
            if dictMode == ZSTD_noDict
                && (offset_1 > 0) as core::ffi::c_int
                    & (MEM_read32(ip.add(1).sub(offset_1 as usize) as *const core::ffi::c_void)
                        == MEM_read32(ip.add(1) as *const core::ffi::c_void))
                        as core::ffi::c_int
                    != 0
            {
                matchLength = (ZSTD_count(
                    ip.add(1).add(4),
                    ip.add(1).add(4).sub(offset_1 as usize),
                    iend,
                ))
                .wrapping_add(4);
                if depth == 0 {
                    current_block = 9173645608424642017;
                } else {
                    current_block = 6450636197030046351;
                }
            } else {
                current_block = 6450636197030046351;
            }
            match current_block {
                9173645608424642017 => {}
                _ => {
                    // first search (depth 0)
                    let mut offbaseFound = 999999999;
                    let ml2 = ZSTD_searchMax(
                        ms,
                        ip,
                        iend,
                        &mut offbaseFound,
                        mls,
                        rowLog,
                        searchMethod,
                        dictMode,
                    );
                    if ml2 > matchLength {
                        matchLength = ml2;
                        start = ip;
                        offBase = offbaseFound;
                    }

                    if matchLength < 4 {
                        let step =
                            (ip.offset_from_unsigned(anchor) >> kSearchStrength).wrapping_add(1); // jump faster over incompressible sections
                        ip = ip.add(step);
                        // Enter the lazy skipping mode once we are skipping more than 8 bytes at a time.
                        // In this mode we stop inserting every position into our tables, and only insert
                        // positions that we search, which is one in step positions.
                        // The exact cutoff is flexible, I've just chosen a number that is reasonably high,
                        // so we minimize the compression ratio loss in "normal" scenarios. This mode gets
                        // triggered once we've gone 2KB without finding any matches.
                        ms.lazySkipping = (step > kLazySkippingStep as size_t) as core::ffi::c_int;
                        continue;
                    } else {
                        // let's try to find a better solution
                        if depth >= 1 {
                            while ip < ilimit {
                                ip = ip.add(1);
                                if dictMode == ZSTD_noDict
                                    && offBase != 0
                                    && (offset_1 > 0) as core::ffi::c_int
                                        & (MEM_read32(ip as *const core::ffi::c_void)
                                            == MEM_read32(ip.sub(offset_1 as usize)
                                                as *const core::ffi::c_void))
                                            as core::ffi::c_int
                                        != 0
                                {
                                    let mlRep = (ZSTD_count(
                                        ip.add(4),
                                        ip.add(4).sub(offset_1 as usize),
                                        iend,
                                    ))
                                    .wrapping_add(4);
                                    let gain2 = (mlRep * 3) as core::ffi::c_int;
                                    let gain1 = (matchLength * 3)
                                        .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                        .wrapping_add(1)
                                        as core::ffi::c_int;
                                    if mlRep >= 4 && gain2 > gain1 {
                                        matchLength = mlRep;
                                        offBase = REPCODE1_TO_OFFBASE as size_t;
                                        start = ip;
                                    }
                                }

                                if isDxS {
                                    let repIndex_0 = (ip.offset_from(base) as core::ffi::c_long
                                        as u32)
                                        .wrapping_sub(offset_1);
                                    let repMatch_0 = if repIndex_0 < prefixLowestIndex {
                                        dictBase
                                            .offset(repIndex_0.wrapping_sub(dictIndexDelta)
                                                as isize)
                                    } else {
                                        base.offset(repIndex_0 as isize)
                                    };
                                    if ZSTD_index_overlap_check(prefixLowestIndex, repIndex_0)
                                        && MEM_read32(repMatch_0 as *const core::ffi::c_void)
                                            == MEM_read32(ip as *const core::ffi::c_void)
                                    {
                                        let repMatchEnd_0 = if repIndex_0 < prefixLowestIndex {
                                            dictEnd
                                        } else {
                                            iend
                                        };
                                        let mlRep_0 = (ZSTD_count_2segments(
                                            ip.add(4),
                                            repMatch_0.add(4),
                                            iend,
                                            repMatchEnd_0,
                                            prefixLowest,
                                        ))
                                        .wrapping_add(4);
                                        let gain2_0 = (mlRep_0 * 3) as core::ffi::c_int;
                                        let gain1_0 = (matchLength * 3)
                                            .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                            .wrapping_add(1)
                                            as core::ffi::c_int;
                                        if mlRep_0 >= 4 && gain2_0 > gain1_0 {
                                            matchLength = mlRep_0;
                                            offBase = REPCODE1_TO_OFFBASE as size_t;
                                            start = ip;
                                        }
                                    }
                                }

                                let mut ofbCandidate = 999999999;
                                let ml2_0 = ZSTD_searchMax(
                                    ms,
                                    ip,
                                    iend,
                                    &mut ofbCandidate,
                                    mls,
                                    rowLog,
                                    searchMethod,
                                    dictMode,
                                );
                                let gain2_1 = (ml2_0 * 4)
                                    .wrapping_sub(ZSTD_highbit32(ofbCandidate as u32) as size_t)
                                    as core::ffi::c_int;
                                let gain1_1 = (matchLength * 4)
                                    .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                    .wrapping_add(4)
                                    as core::ffi::c_int;
                                if ml2_0 >= 4 && gain2_1 > gain1_1 {
                                    matchLength = ml2_0;
                                    offBase = ofbCandidate;
                                    start = ip;
                                    // search a better one
                                } else {
                                    if !(depth == 2 && ip < ilimit) {
                                        break;
                                    }
                                    // let's find an even better one
                                    ip = ip.add(1);
                                    if dictMode == ZSTD_noDict
                                        && offBase != 0
                                        && (offset_1 > 0) as core::ffi::c_int
                                            & (MEM_read32(ip as *const core::ffi::c_void)
                                                == MEM_read32(ip.sub(offset_1 as usize)
                                                    as *const core::ffi::c_void))
                                                as core::ffi::c_int
                                            != 0
                                    {
                                        let mlRep_1 = (ZSTD_count(
                                            ip.add(4),
                                            ip.add(4).sub(offset_1 as usize),
                                            iend,
                                        ))
                                        .wrapping_add(4);
                                        let gain2_2 = (mlRep_1 * 4) as core::ffi::c_int;
                                        let gain1_2 = (matchLength * 4)
                                            .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                            .wrapping_add(1)
                                            as core::ffi::c_int;
                                        if mlRep_1 >= 4 && gain2_2 > gain1_2 {
                                            matchLength = mlRep_1;
                                            offBase = REPCODE1_TO_OFFBASE as size_t;
                                            start = ip;
                                        }
                                    }

                                    if isDxS {
                                        let repIndex_1 = (ip.offset_from(base) as core::ffi::c_long
                                            as u32)
                                            .wrapping_sub(offset_1);
                                        let repMatch_1 = if repIndex_1 < prefixLowestIndex {
                                            dictBase
                                                .offset(repIndex_1.wrapping_sub(dictIndexDelta)
                                                    as isize)
                                        } else {
                                            base.offset(repIndex_1 as isize)
                                        };
                                        if ZSTD_index_overlap_check(prefixLowestIndex, repIndex_1)
                                            && MEM_read32(repMatch_1 as *const core::ffi::c_void)
                                                == MEM_read32(ip as *const core::ffi::c_void)
                                        {
                                            let repMatchEnd_1 = if repIndex_1 < prefixLowestIndex {
                                                dictEnd
                                            } else {
                                                iend
                                            };
                                            let mlRep_2 = (ZSTD_count_2segments(
                                                ip.add(4),
                                                repMatch_1.add(4),
                                                iend,
                                                repMatchEnd_1,
                                                prefixLowest,
                                            ))
                                            .wrapping_add(4);
                                            let gain2_3 = (mlRep_2 * 4) as core::ffi::c_int;
                                            let gain1_3 = (matchLength * 4)
                                                .wrapping_sub(
                                                    ZSTD_highbit32(offBase as u32) as size_t
                                                )
                                                .wrapping_add(1)
                                                as core::ffi::c_int;
                                            if mlRep_2 >= 4 && gain2_3 > gain1_3 {
                                                matchLength = mlRep_2;
                                                offBase = REPCODE1_TO_OFFBASE as size_t;
                                                start = ip;
                                            }
                                        }
                                    }

                                    let mut ofbCandidate_0 = 999999999;
                                    let ml2_1 = ZSTD_searchMax(
                                        ms,
                                        ip,
                                        iend,
                                        &mut ofbCandidate_0,
                                        mls,
                                        rowLog,
                                        searchMethod,
                                        dictMode,
                                    );
                                    let gain2_4 = (ml2_1 * 4)
                                        .wrapping_sub(
                                            ZSTD_highbit32(ofbCandidate_0 as u32) as size_t
                                        )
                                        as core::ffi::c_int;
                                    let gain1_4 = (matchLength * 4)
                                        .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                        .wrapping_add(7)
                                        as core::ffi::c_int;
                                    if !(ml2_1 >= 4 && gain2_4 > gain1_4) {
                                        break;
                                    }
                                    matchLength = ml2_1;
                                    offBase = ofbCandidate_0;
                                    start = ip;
                                }
                            }
                        }
                        // NOTE:
                        // Pay attention that `start[-value]` can lead to strange undefined behavior
                        // notably if `value` is unsigned, resulting in a large positive `-value`.
                        // catch up
                        if offBase > ZSTD_REP_NUM as size_t {
                            if dictMode == ZSTD_noDict {
                                while (start > anchor) as core::ffi::c_int
                                    & (start.offset(
                                        -(offBase.wrapping_sub(ZSTD_REP_NUM as size_t) as isize),
                                    ) > prefixLowest)
                                        as core::ffi::c_int
                                    != 0
                                    && *start.sub(1) as core::ffi::c_int
                                        == *start
                                            .offset(
                                                -(offBase.wrapping_sub(ZSTD_REP_NUM as size_t)
                                                    as isize),
                                            )
                                            .sub(1)
                                            as core::ffi::c_int
                                {
                                    start = start.sub(1);
                                    matchLength = matchLength.wrapping_add(1);
                                }
                            }

                            if isDxS {
                                let matchIndex = (start.offset_from(base) as core::ffi::c_long
                                    as size_t)
                                    .wrapping_sub(offBase.wrapping_sub(ZSTD_REP_NUM as size_t))
                                    as u32;
                                let mut match_0 = if matchIndex < prefixLowestIndex {
                                    dictBase
                                        .offset(matchIndex as isize)
                                        .sub(dictIndexDelta as usize)
                                } else {
                                    base.offset(matchIndex as isize)
                                };
                                let mStart = if matchIndex < prefixLowestIndex {
                                    dictLowest
                                } else {
                                    prefixLowest
                                };
                                while start > anchor
                                    && match_0 > mStart
                                    && *start.sub(1) as core::ffi::c_int
                                        == *match_0.sub(1) as core::ffi::c_int
                                {
                                    start = start.sub(1);
                                    match_0 = match_0.sub(1);
                                    matchLength = matchLength.wrapping_add(1);
                                }
                            }

                            offset_2 = offset_1;
                            offset_1 = offBase.wrapping_sub(ZSTD_REP_NUM as size_t) as u32;
                        }
                    }
                }
            }
        }

        // store sequence
        let litLength = start.offset_from_unsigned(anchor);
        ZSTD_storeSeq(
            seqStore,
            litLength,
            anchor,
            iend,
            offBase as u32,
            matchLength,
        );
        ip = start.add(matchLength);
        anchor = ip;

        if ms.lazySkipping != 0 {
            // We've found a match, disable lazy skipping mode, and refill the hash cache.
            if searchMethod == search_rowHash {
                ZSTD_row_fillHashCache(ms, base, rowLog, mls, ms.nextToUpdate, ilimit);
            }
            ms.lazySkipping = 0;
        }

        // check immediate repcode
        if isDxS {
            while ip <= ilimit {
                let current2 = ip.offset_from(base) as core::ffi::c_long as u32;
                let repIndex_2 = current2.wrapping_sub(offset_2);
                let repMatch_2 = if repIndex_2 < prefixLowestIndex {
                    dictBase
                        .sub(dictIndexDelta as usize)
                        .offset(repIndex_2 as isize)
                } else {
                    base.offset(repIndex_2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixLowestIndex, repIndex_2)
                    && MEM_read32(repMatch_2 as *const core::ffi::c_void)
                        == MEM_read32(ip as *const core::ffi::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex_2 < prefixLowestIndex {
                    dictEnd
                } else {
                    iend
                };
                matchLength = (ZSTD_count_2segments(
                    ip.add(4),
                    repMatch_2.add(4),
                    iend,
                    repEnd2,
                    prefixLowest,
                ))
                .wrapping_add(4);
                offBase = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offBase as u32; // swap offset_2 <=> offset_1
                ZSTD_storeSeq(
                    seqStore,
                    0,
                    anchor,
                    iend,
                    REPCODE1_TO_OFFBASE as u32,
                    matchLength,
                );
                ip = ip.add(matchLength);
                anchor = ip;
            }
        }

        if dictMode == ZSTD_noDict {
            while (ip <= ilimit) as core::ffi::c_int & (offset_2 > 0) as core::ffi::c_int != 0
                && MEM_read32(ip as *const core::ffi::c_void)
                    == MEM_read32(ip.sub(offset_2 as usize) as *const core::ffi::c_void)
            {
                // store sequence
                matchLength =
                    (ZSTD_count(ip.add(4), ip.add(4).sub(offset_2 as usize), iend)).wrapping_add(4);
                offBase = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offBase as u32; // swap repcodes
                ZSTD_storeSeq(
                    seqStore,
                    0,
                    anchor,
                    iend,
                    REPCODE1_TO_OFFBASE as u32,
                    matchLength,
                );
                ip = ip.add(matchLength);
                anchor = ip;
            }
        }
    }

    // If offset_1 started invalid (offsetSaved1 != 0) and became valid (offset_1 != 0),
    // rotate saved offsets. See comment in ZSTD_compressBlock_fast_noDict for more context.
    offsetSaved2 = if offsetSaved1 != 0 && offset_1 != 0 {
        offsetSaved1
    } else {
        offsetSaved2
    };

    // save reps for next block
    *rep = if offset_1 != 0 {
        offset_1
    } else {
        offsetSaved1
    };
    *rep.add(1) = if offset_2 != 0 {
        offset_2
    } else {
        offsetSaved2
    };

    // Return the last literals size
    iend.offset_from_unsigned(anchor)
}

pub unsafe fn ZSTD_compressBlock_greedy(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0,
        ZSTD_noDict,
    )
}

pub unsafe fn ZSTD_compressBlock_greedy_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0,
        ZSTD_dictMatchState,
    )
}

pub unsafe fn ZSTD_compressBlock_greedy_dedicatedDictSearch(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0,
        ZSTD_dedicatedDictSearch,
    )
}

pub unsafe fn ZSTD_compressBlock_greedy_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0,
        ZSTD_noDict,
    )
}

pub unsafe fn ZSTD_compressBlock_greedy_dictMatchState_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0,
        ZSTD_dictMatchState,
    )
}

pub unsafe fn ZSTD_compressBlock_greedy_dedicatedDictSearch_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0,
        ZSTD_dedicatedDictSearch,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1,
        ZSTD_noDict,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1,
        ZSTD_dictMatchState,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy_dedicatedDictSearch(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1,
        ZSTD_dedicatedDictSearch,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1,
        ZSTD_noDict,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy_dictMatchState_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1,
        ZSTD_dictMatchState,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy_dedicatedDictSearch_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1,
        ZSTD_dedicatedDictSearch,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy2(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2,
        ZSTD_noDict,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy2_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2,
        ZSTD_dictMatchState,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy2_dedicatedDictSearch(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2,
        ZSTD_dedicatedDictSearch,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy2_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2,
        ZSTD_noDict,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy2_dictMatchState_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2,
        ZSTD_dictMatchState,
    )
}

pub unsafe fn ZSTD_compressBlock_lazy2_dedicatedDictSearch_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2,
        ZSTD_dedicatedDictSearch,
    )
}

pub unsafe fn ZSTD_compressBlock_btlazy2(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2,
        ZSTD_noDict,
    )
}

pub unsafe fn ZSTD_compressBlock_btlazy2_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2,
        ZSTD_dictMatchState,
    )
}

#[inline(always)]
unsafe fn ZSTD_compressBlock_lazy_extDict_generic(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    searchMethod: searchMethod_e,
    depth: u32,
) -> size_t {
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.add(srcSize);
    let ilimit = if searchMethod == search_rowHash {
        iend.sub(8).sub(ZSTD_ROW_HASH_CACHE_SIZE as usize)
    } else {
        iend.sub(8)
    };
    let base = ms.window.base;
    let dictLimit = ms.window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictBase = ms.window.dictBase;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let dictStart = dictBase.offset(ms.window.lowLimit as isize);
    let windowLog = ms.cParams.windowLog;
    let mls = ms.cParams.minMatch.clamp(4, 6);
    let rowLog = ms.cParams.searchLog.clamp(4, 6);

    let mut offset_1 = *rep;
    let mut offset_2 = *rep.add(1);

    // Reset the lazy skipping state
    ms.lazySkipping = 0;

    // init
    ip = ip.offset((ip == prefixStart) as core::ffi::c_int as isize);
    if searchMethod == search_rowHash {
        ZSTD_row_fillHashCache(ms, base, rowLog, mls, ms.nextToUpdate, ilimit);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    asm!(".p2align 5", options(preserves_flags, att_syntax));

    // Match Loop
    let mut current_block_61: u64;
    while ip < ilimit {
        let mut matchLength = 0;
        let mut offBase = REPCODE1_TO_OFFBASE as size_t;
        let mut start = ip.add(1);
        let mut curr = ip.offset_from(base) as core::ffi::c_long as u32;

        // check repCode
        let windowLow = ZSTD_getLowestMatchIndex(ms, curr.wrapping_add(1), windowLog);
        let repIndex = curr.wrapping_add(1).wrapping_sub(offset_1);
        let repBase = if repIndex < dictLimit { dictBase } else { base };
        let repMatch = repBase.offset(repIndex as isize);
        if ZSTD_index_overlap_check(dictLimit, repIndex)
            & (offset_1 <= curr.wrapping_add(1).wrapping_sub(windowLow))
        {
            if MEM_read32(ip.add(1) as *const core::ffi::c_void)
                == MEM_read32(repMatch as *const core::ffi::c_void)
            {
                // repcode detected we should take it
                let repEnd = if repIndex < dictLimit { dictEnd } else { iend };
                matchLength = (ZSTD_count_2segments(
                    ip.add(1).add(4),
                    repMatch.add(4),
                    iend,
                    repEnd,
                    prefixStart,
                ))
                .wrapping_add(4);
                if depth == 0 {
                    current_block_61 = 10962704168502628720;
                } else {
                    current_block_61 = 12147880666119273379;
                }
            } else {
                current_block_61 = 12147880666119273379;
            }
        } else {
            current_block_61 = 12147880666119273379;
        }
        if current_block_61 == 12147880666119273379 {
            // first search (depth 0)
            let mut ofbCandidate = 999999999;
            let ml2 = ZSTD_searchMax(
                ms,
                ip,
                iend,
                &mut ofbCandidate,
                mls,
                rowLog,
                searchMethod,
                ZSTD_extDict,
            );
            if ml2 > matchLength {
                matchLength = ml2;
                start = ip;
                offBase = ofbCandidate;
            }

            if matchLength < 4 {
                let step = ip.offset_from_unsigned(anchor) >> kSearchStrength;
                // Jump faster over incompressible sections
                ip = ip.add(step.wrapping_add(1));
                // Enter the lazy skipping mode once we are skipping more than 8 bytes at a time.
                // In this mode we stop inserting every position into our tables, and only insert
                // positions that we search, which is one in step positions.
                // The exact cutoff is flexible, I've just chosen a number that is reasonably high,
                // so we minimize the compression ratio loss in "normal" scenarios. This mode gets
                // triggered once we've gone 2KB without finding any matches.
                ms.lazySkipping = (step > kLazySkippingStep as size_t) as core::ffi::c_int;
                continue;
            } else {
                // let's try to find a better solution
                if depth >= 1 {
                    while ip < ilimit {
                        ip = ip.add(1);
                        curr = curr.wrapping_add(1);
                        // check repCode
                        if offBase != 0 {
                            let windowLow_0 = ZSTD_getLowestMatchIndex(ms, curr, windowLog);
                            let repIndex_0 = curr.wrapping_sub(offset_1);
                            let repBase_0 = if repIndex_0 < dictLimit {
                                dictBase
                            } else {
                                base
                            };
                            let repMatch_0 = repBase_0.offset(repIndex_0 as isize);
                            if ZSTD_index_overlap_check(dictLimit, repIndex_0)
                                & (offset_1 <= curr.wrapping_sub(windowLow_0))
                                && MEM_read32(ip as *const core::ffi::c_void)
                                    == MEM_read32(repMatch_0 as *const core::ffi::c_void)
                            {
                                // repcode detected
                                let repEnd_0 = if repIndex_0 < dictLimit {
                                    dictEnd
                                } else {
                                    iend
                                };
                                let repLength = (ZSTD_count_2segments(
                                    ip.add(4),
                                    repMatch_0.add(4),
                                    iend,
                                    repEnd_0,
                                    prefixStart,
                                ))
                                .wrapping_add(4);
                                let gain2 = (repLength * 3) as core::ffi::c_int;
                                let gain1 = (matchLength * 3)
                                    .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                    .wrapping_add(1)
                                    as core::ffi::c_int;
                                if repLength >= 4 && gain2 > gain1 {
                                    matchLength = repLength;
                                    offBase = REPCODE1_TO_OFFBASE as size_t;
                                    start = ip;
                                }
                            }
                        }

                        // search match, depth 1
                        let mut ofbCandidate_0 = 999999999;
                        let ml2_0 = ZSTD_searchMax(
                            ms,
                            ip,
                            iend,
                            &mut ofbCandidate_0,
                            mls,
                            rowLog,
                            searchMethod,
                            ZSTD_extDict,
                        );
                        let gain2_0 = (ml2_0 * 4)
                            .wrapping_sub(ZSTD_highbit32(ofbCandidate_0 as u32) as size_t)
                            as core::ffi::c_int; // raw approx
                        let gain1_0 = (matchLength * 4)
                            .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                            .wrapping_add(4)
                            as core::ffi::c_int;
                        if ml2_0 >= 4 && gain2_0 > gain1_0 {
                            matchLength = ml2_0;
                            offBase = ofbCandidate_0;
                            start = ip;
                            // search a better one
                        } else {
                            // let's find an even better one
                            if !(depth == 2 && ip < ilimit) {
                                break;
                            }
                            ip = ip.add(1);
                            curr = curr.wrapping_add(1);
                            // check repCode
                            if offBase != 0 {
                                let windowLow_1 = ZSTD_getLowestMatchIndex(ms, curr, windowLog);
                                let repIndex_1 = curr.wrapping_sub(offset_1);
                                let repBase_1 = if repIndex_1 < dictLimit {
                                    dictBase
                                } else {
                                    base
                                };
                                let repMatch_1 = repBase_1.offset(repIndex_1 as isize);
                                if ZSTD_index_overlap_check(dictLimit, repIndex_1)
                                    & (offset_1 <= curr.wrapping_sub(windowLow_1))
                                    && MEM_read32(ip as *const core::ffi::c_void)
                                        == MEM_read32(repMatch_1 as *const core::ffi::c_void)
                                {
                                    // repcode detected
                                    let repEnd_1 = if repIndex_1 < dictLimit {
                                        dictEnd
                                    } else {
                                        iend
                                    };
                                    let repLength_0 = (ZSTD_count_2segments(
                                        ip.add(4),
                                        repMatch_1.add(4),
                                        iend,
                                        repEnd_1,
                                        prefixStart,
                                    ))
                                    .wrapping_add(4);
                                    let gain2_1 = (repLength_0 * 4) as core::ffi::c_int;
                                    let gain1_1 = (matchLength * 4)
                                        .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                        .wrapping_add(1)
                                        as core::ffi::c_int;
                                    if repLength_0 >= 4 && gain2_1 > gain1_1 {
                                        matchLength = repLength_0;
                                        offBase = REPCODE1_TO_OFFBASE as size_t;
                                        start = ip;
                                    }
                                }
                            }

                            // search match, depth 2
                            let mut ofbCandidate_1 = 999999999;
                            let ml2_1 = ZSTD_searchMax(
                                ms,
                                ip,
                                iend,
                                &mut ofbCandidate_1,
                                mls,
                                rowLog,
                                searchMethod,
                                ZSTD_extDict,
                            );
                            let gain2_2 = (ml2_1 * 4)
                                .wrapping_sub(ZSTD_highbit32(ofbCandidate_1 as u32) as size_t)
                                as core::ffi::c_int;
                            let gain1_2 = (matchLength * 4)
                                .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                .wrapping_add(7)
                                as core::ffi::c_int;
                            if !(ml2_1 >= 4 && gain2_2 > gain1_2) {
                                break;
                            }
                            matchLength = ml2_1;
                            offBase = ofbCandidate_1;
                            start = ip;
                        }
                    }
                }

                // catch up
                if offBase > ZSTD_REP_NUM as size_t {
                    let matchIndex = (start.offset_from_unsigned(base))
                        .wrapping_sub(offBase.wrapping_sub(ZSTD_REP_NUM as size_t))
                        as u32;
                    let mut match_0 = if matchIndex < dictLimit {
                        dictBase.offset(matchIndex as isize)
                    } else {
                        base.offset(matchIndex as isize)
                    };
                    let mStart = if matchIndex < dictLimit {
                        dictStart
                    } else {
                        prefixStart
                    };
                    while start > anchor
                        && match_0 > mStart
                        && *start.sub(1) as core::ffi::c_int == *match_0.sub(1) as core::ffi::c_int
                    {
                        start = start.sub(1);
                        match_0 = match_0.sub(1);
                        matchLength = matchLength.wrapping_add(1);
                    }
                    offset_2 = offset_1;
                    offset_1 = offBase.wrapping_sub(ZSTD_REP_NUM as size_t) as u32;
                }
            }
        }

        // store sequence
        let litLength = start.offset_from_unsigned(anchor);
        ZSTD_storeSeq(
            seqStore,
            litLength,
            anchor,
            iend,
            offBase as u32,
            matchLength,
        );
        ip = start.add(matchLength);
        anchor = ip;

        if ms.lazySkipping != 0 {
            // We've found a match, disable lazy skipping mode, and refill the hash cache.
            if searchMethod == search_rowHash {
                ZSTD_row_fillHashCache(ms, base, rowLog, mls, ms.nextToUpdate, ilimit);
            }
            ms.lazySkipping = 0;
        }

        // check immediate repcode
        while ip <= ilimit {
            let repCurrent = ip.offset_from(base) as core::ffi::c_long as u32;
            let windowLow_2 = ZSTD_getLowestMatchIndex(ms, repCurrent, windowLog);
            let repIndex_2 = repCurrent.wrapping_sub(offset_2);
            let repBase_2 = if repIndex_2 < dictLimit {
                dictBase
            } else {
                base
            };
            let repMatch_2 = repBase_2.offset(repIndex_2 as isize);
            if !(ZSTD_index_overlap_check(dictLimit, repIndex_2)
                & (offset_2 <= repCurrent.wrapping_sub(windowLow_2)))
            {
                break;
            }
            if MEM_read32(ip as *const core::ffi::c_void)
                != MEM_read32(repMatch_2 as *const core::ffi::c_void)
            {
                break;
            }
            // repcode detected we should take it
            let repEnd_2 = if repIndex_2 < dictLimit {
                dictEnd
            } else {
                iend
            };
            matchLength =
                (ZSTD_count_2segments(ip.add(4), repMatch_2.add(4), iend, repEnd_2, prefixStart))
                    .wrapping_add(4);
            offBase = offset_2 as size_t;
            offset_2 = offset_1;
            offset_1 = offBase as u32; // swap offset history
            ZSTD_storeSeq(
                seqStore,
                0,
                anchor,
                iend,
                REPCODE1_TO_OFFBASE as u32,
                matchLength,
            );
            ip = ip.add(matchLength);
            anchor = ip;
        }
    }

    // Save reps for next block
    *rep = offset_1;
    *rep.add(1) = offset_2;

    // Return the last literals size
    iend.offset_from_unsigned(anchor)
}

pub unsafe fn ZSTD_compressBlock_greedy_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src, srcSize, search_hashChain, 0)
}

pub unsafe fn ZSTD_compressBlock_greedy_extDict_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src, srcSize, search_rowHash, 0)
}

pub unsafe fn ZSTD_compressBlock_lazy_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src, srcSize, search_hashChain, 1)
}

pub unsafe fn ZSTD_compressBlock_lazy_extDict_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src, srcSize, search_rowHash, 1)
}

pub unsafe fn ZSTD_compressBlock_lazy2_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src, srcSize, search_hashChain, 2)
}
pub unsafe fn ZSTD_compressBlock_lazy2_extDict_row(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src, srcSize, search_rowHash, 2)
}

pub unsafe fn ZSTD_compressBlock_btlazy2_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(ms, seqStore, rep, src, srcSize, search_binaryTree, 2)
}
