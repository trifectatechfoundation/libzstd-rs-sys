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
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}

pub type ZSTD_dictTableLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = core::ffi::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
pub type ZSTD_match4Found = Option<unsafe fn(*const u8, *const u8, u32, u32) -> bool>;
pub const CACHELINE_SIZE: core::ffi::c_int = 64;

use libc::size_t;

use crate::lib::common::fse::{FSE_CTable, FSE_repeat};
use crate::lib::common::huf::{HUF_CElt, HUF_repeat};
use crate::lib::common::mem::MEM_read32;
use crate::lib::common::zstd_internal::ZSTD_REP_NUM;
use crate::lib::compress::zstd_compress::{
    SeqStore_t, ZSTD_MatchState_t, ZSTD_match_t, ZSTD_optimal_t,
};
use crate::lib::compress::zstd_compress_internal::{
    ZSTD_OptPrice_e, ZSTD_count, ZSTD_count_2segments, ZSTD_getLowestMatchIndex,
    ZSTD_getLowestPrefixIndex, ZSTD_hashPtr, ZSTD_index_overlap_check, ZSTD_storeSeq,
};
use crate::lib::zstd::{ZSTD_ParamSwitch_e, ZSTD_compressionParameters};

pub const kSearchStrength: core::ffi::c_int = 8;
pub const HASH_READ_SIZE: core::ffi::c_int = 8;

pub const REPCODE1_TO_OFFBASE: core::ffi::c_int = 1;

pub const ZSTD_SHORT_CACHE_TAG_BITS: core::ffi::c_int = 8;
pub const ZSTD_SHORT_CACHE_TAG_MASK: core::ffi::c_uint =
    ((1 as core::ffi::c_uint) << ZSTD_SHORT_CACHE_TAG_BITS).wrapping_sub(1);

/// Helper function for ZSTD_fillHashTable and ZSTD_fillDoubleHashTable.
/// Unpacks hashAndTag into (hash, tag), then packs (index, tag) into hashTable[hash].
#[inline]
unsafe fn ZSTD_writeTaggedIndex(hashTable: *mut u32, hashAndTag: size_t, index: u32) {
    let hash = hashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
    let tag = (hashAndTag & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    *hashTable.add(hash) = index << ZSTD_SHORT_CACHE_TAG_BITS | tag;
}

/// Helper function for short cache matchfinders.
/// Unpacks tag1 and tag2 from lower bits of packedTag1 and packedTag2, then checks if the tags match.
#[inline]
fn ZSTD_comparePackedTags(packedTag1: size_t, packedTag2: size_t) -> bool {
    let tag1 = (packedTag1 & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    let tag2 = (packedTag2 & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    tag1 == tag2
}

unsafe fn ZSTD_fillHashTableForCDict(
    ms: &mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let hashTable = ms.hashTable;
    let hBits = ((*cParams).hashLog).wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as core::ffi::c_uint);
    let mls = (*cParams).minMatch;
    let base = ms.window.base;
    let mut ip = base.offset(ms.nextToUpdate as isize);
    let iend = (end as *const u8).sub(HASH_READ_SIZE as usize);
    let fastHashFillStep = 3;

    // Always insert every fastHashFillStep position into the hash table.
    // Insert the other positions if their hash entry is empty.
    while ip.offset(fastHashFillStep as isize) < iend.add(2) {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let hashAndTag = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBits, mls);
        ZSTD_writeTaggedIndex(hashTable, hashAndTag, curr);
        if dtlm != ZSTD_dtlm_fast {
            // Only load extra positions for ZSTD_dtlm_full
            for p in 1..fastHashFillStep {
                let hashAndTag_0 = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const core::ffi::c_void,
                    hBits,
                    mls,
                );
                if *hashTable.add(hashAndTag_0 >> ZSTD_SHORT_CACHE_TAG_BITS) == 0 {
                    // not yet filled
                    ZSTD_writeTaggedIndex(hashTable, hashAndTag_0, curr.wrapping_add(p));
                }
            }
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}

unsafe fn ZSTD_fillHashTableForCCtx(
    ms: &mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let hashTable = ms.hashTable;
    let hBits = (*cParams).hashLog;
    let mls = (*cParams).minMatch;
    let base = ms.window.base;
    let mut ip = base.offset(ms.nextToUpdate as isize);
    let iend = (end as *const u8).sub(HASH_READ_SIZE as usize);
    let fastHashFillStep = 3;

    // Always insert every fastHashFillStep position into the hash table.
    // Insert the other positions if their hash entry is empty.
    while ip.offset(fastHashFillStep as isize) < iend.add(2) {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let hash0 = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBits, mls);
        *hashTable.add(hash0) = curr;
        if dtlm != ZSTD_dtlm_fast {
            // Only load extra positions for ZSTD_dtlm_full
            for p in 1..fastHashFillStep {
                let hash = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const core::ffi::c_void,
                    hBits,
                    mls,
                );
                if *hashTable.add(hash) == 0 {
                    // not yet filled
                    *hashTable.add(hash) = curr.wrapping_add(p);
                }
            }
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}

pub unsafe fn ZSTD_fillHashTable(
    ms: &mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    dtlm: ZSTD_dictTableLoadMethod_e,
    tfp: ZSTD_tableFillPurpose_e,
) {
    if tfp == ZSTD_tfp_forCDict {
        ZSTD_fillHashTableForCDict(ms, end, dtlm);
    } else {
        ZSTD_fillHashTableForCCtx(ms, end, dtlm);
    }
}

unsafe fn ZSTD_match4Found_cmov(
    currentPtr: *const u8,
    matchAddress: *const u8,
    matchIdx: u32,
    idxLowLimit: u32,
) -> bool {
    // Array of ~random data, should have low probability of matching data.
    // Load from here if the index is invalid.
    // Used to avoid unpredictable branches.
    static dummy: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

    // currentIdx >= lowLimit is a (somewhat) unpredictable branch.
    // However expression below compiles into conditional move.
    let mvalAddr =
        core::hint::select_unpredictable(matchIdx >= idxLowLimit, matchAddress, dummy.as_ptr());

    // Note: this used to be written as : return test1 && test2;
    // Unfortunately, once inlined, these tests become branches,
    // in which case it becomes critical that they are executed in the right order (test1 then test2).
    // So we have to write these tests in a specific manner to ensure their ordering.
    if MEM_read32(currentPtr as *const core::ffi::c_void)
        != MEM_read32(mvalAddr as *const core::ffi::c_void)
    {
        return false;
    }

    // force ordering of these tests, which matters once the function is inlined, as they become branches.
    #[cfg(not(target_family = "wasm"))]
    asm!("", options(preserves_flags));

    matchIdx >= idxLowLimit
}

unsafe fn ZSTD_match4Found_branch(
    currentPtr: *const u8,
    matchAddress: *const u8,
    matchIdx: u32,
    idxLowLimit: u32,
) -> bool {
    // using a branch instead of a cmov,
    // because it's faster in scenarios where matchIdx >= idxLowLimit is generally true,
    // aka almost all candidates are within range
    let mut mval: u32 = 0;
    if matchIdx >= idxLowLimit {
        mval = MEM_read32(matchAddress as *const core::ffi::c_void);
    } else {
        mval = MEM_read32(currentPtr as *const core::ffi::c_void) ^ 1;
    }
    MEM_read32(currentPtr as *const core::ffi::c_void) == mval
}

/// If you squint hard enough (and ignore repcodes), the search operation at any
/// given position is broken into 4 stages:
///
/// 1. Hash   (map position to hash value via input read)
/// 2. Lookup (map hash val to index via hashtable read)
/// 3. Load   (map index to value at that position via input read)
/// 4. Compare
///
/// Each of these steps involves a memory read at an address which is computed
/// from the previous step. This means these steps must be sequenced and their
/// latencies are cumulative.
///
/// Rather than do 1->2->3->4 sequentially for a single position before moving
/// onto the next, this implementation interleaves these operations across the
/// next few positions:
///
/// R = Repcode Read & Compare
/// H = Hash
/// T = Table Lookup
/// M = Match Read & Compare
///
/// Pos | Time -->
/// ----+-------------------
/// N   | ... M
/// N+1 | ...   TM
/// N+2 |    R H   T M
/// N+3 |         H    TM
/// N+4 |           R H   T M
/// N+5 |                H   ...
/// N+6 |                  R ...
///
/// This is very much analogous to the pipelining of execution in a CPU. And just
/// like a CPU, we have to dump the pipeline when we find a match (i.e., take a
/// branch).
///
/// When this happens, we throw away our current state, and do the following prep
/// to re-enter the loop:
///
/// Pos | Time -->
/// ----+-------------------
/// N   | H T
/// N+1 |  H
///
/// This is also the work we do at the beginning to enter the loop initially.
#[inline(always)]
unsafe fn ZSTD_compressBlock_fast_noDict_generic(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mls: u32,
    useCmov: bool,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let hashTable = ms.hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as core::ffi::c_uint)
        .wrapping_add(1) as size_t; // min 2
    let base = ms.window.base;
    let istart = src as *const u8;
    let endIndex = (istart.offset_from_unsigned(base)).wrapping_add(srcSize) as u32;
    let prefixStartIndex = ZSTD_getLowestPrefixIndex(ms, endIndex, (*cParams).windowLog);
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.add(srcSize);
    let ilimit = iend.sub(HASH_READ_SIZE as usize);

    let mut anchor = istart;
    let mut ip0 = istart;
    let mut ip1 = core::ptr::null::<u8>();
    let mut ip2 = core::ptr::null::<u8>();
    let mut ip3 = core::ptr::null::<u8>();
    let mut current0: u32 = 0;

    let mut rep_offset1 = *rep;
    let mut rep_offset2 = *rep.add(1);
    let mut offsetSaved1 = 0;
    let mut offsetSaved2 = 0;

    let mut hash0: size_t = 0; // hash for ip0
    let mut hash1: size_t = 0; // hash for ip1
    let mut matchIdx: u32 = 0; // match idx for ip0

    let mut offcode: u32 = 0;
    let mut match0 = core::ptr::null::<u8>();
    let mut mLength: size_t = 0;

    // ip0 and ip1 are always adjacent. The targetLength skipping and
    // uncompressibility acceleration is applied to every other position,
    // matching the behavior of #1562. step therefore represents the gap
    // between pairs of positions, from ip0 to ip2 or ip1 to ip3.
    let mut step: size_t = 0;
    let mut nextStep = core::ptr::null::<u8>();
    let kStepIncr = (1 << (kSearchStrength - 1)) as size_t;
    let matchFound: ZSTD_match4Found = if useCmov {
        Some(ZSTD_match4Found_cmov as unsafe fn(*const u8, *const u8, u32, u32) -> bool)
    } else {
        Some(ZSTD_match4Found_branch as unsafe fn(*const u8, *const u8, u32, u32) -> bool)
    };

    ip0 = ip0.offset((ip0 == prefixStart) as core::ffi::c_int as isize);
    let curr = ip0.offset_from(base) as core::ffi::c_long as u32;
    let windowLow = ZSTD_getLowestPrefixIndex(ms, curr, (*cParams).windowLog);
    let maxRep = curr.wrapping_sub(windowLow);
    if rep_offset2 > maxRep {
        offsetSaved2 = rep_offset2;
        rep_offset2 = 0;
    }
    if rep_offset1 > maxRep {
        offsetSaved1 = rep_offset1;
        rep_offset1 = 0;
    }

    // start each op
    '__start: loop {
        step = stepSize;
        nextStep = ip0.add(kStepIncr);

        // calculate positions, ip0 - anchor == 0, so we skip step calc
        ip1 = ip0.add(1);
        ip2 = ip0.add(step);
        ip3 = ip2.add(1);

        if ip3 >= ilimit {
            break;
        }

        hash0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls);
        hash1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, hlog, mls);

        matchIdx = *hashTable.add(hash0);

        loop {
            // load repcode match for ip[2]
            let rval = MEM_read32(ip2.sub(rep_offset1 as usize) as *const core::ffi::c_void);

            // write back hash table entry
            current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
            *hashTable.add(hash0) = current0;

            // check repcode at ip[2]
            if (MEM_read32(ip2 as *const core::ffi::c_void) == rval) as core::ffi::c_int
                & (rep_offset1 > 0) as core::ffi::c_int
                != 0
            {
                ip0 = ip2;
                match0 = ip0.sub(rep_offset1 as usize);
                mLength = (*ip0.sub(1) as core::ffi::c_int == *match0.sub(1) as core::ffi::c_int)
                    as core::ffi::c_int as size_t;
                ip0 = ip0.sub(mLength as usize);
                match0 = match0.sub(mLength as usize);
                offcode = REPCODE1_TO_OFFBASE as u32;
                mLength = mLength.wrapping_add(4);

                // Write next hash table entry: it's already calculated.
                // This write is known to be safe because ip1 is before the repcode (ip2).
                *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;

                current_block = 4391991184774404966;
                break;
            } else if matchFound.unwrap_unchecked()(
                ip0,
                base.offset(matchIdx as isize),
                matchIdx,
                prefixStartIndex,
            ) {
                // Write next hash table entry (it's already calculated).
                // This write is known to be safe because ip1 == ip0 + 1,
                // so searching will resume after ip1.
                *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;

                current_block = 11113405673187116881;
                break;
            } else {
                // lookup ip[1]
                matchIdx = *hashTable.add(hash1);

                // hash ip[2]
                hash0 = hash1;
                hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);

                // advance to next positions
                ip0 = ip1;
                ip1 = ip2;
                ip2 = ip3;

                // write back hash table entry
                current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
                *hashTable.add(hash0) = current0;

                if matchFound.unwrap_unchecked()(
                    ip0,
                    base.offset(matchIdx as isize),
                    matchIdx,
                    prefixStartIndex,
                ) {
                    // Write next hash table entry, since it's already calculated
                    if step <= 4 {
                        // Avoid writing an index if it's >= position where search will resume.
                        // The minimum possible match has length 4, so search can resume at ip0 + 4.
                        *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;
                    }
                    current_block = 11113405673187116881;
                    break;
                } else {
                    // lookup ip[1]
                    matchIdx = *hashTable.add(hash1);

                    // hash ip[2]
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);

                    // advance to next positions
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip0.add(step);
                    ip3 = ip1.add(step);

                    // calculate step
                    if ip2 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.add(kStepIncr);
                    }

                    if ip3 >= ilimit {
                        break '__start;
                    }
                }
            }
        }

        // _offset: Requires ip0, idx
        if current_block == 11113405673187116881 {
            // Compute the offset code.
            match0 = base.offset(matchIdx as isize);
            rep_offset2 = rep_offset1;
            rep_offset1 = ip0.offset_from(match0) as core::ffi::c_long as u32;
            offcode = rep_offset1.wrapping_add(ZSTD_REP_NUM as u32);
            mLength = 4;

            // Count the backwards match length.
            while (ip0 > anchor) as core::ffi::c_int & (match0 > prefixStart) as core::ffi::c_int
                != 0
                && *ip0.sub(1) as core::ffi::c_int == *match0.sub(1) as core::ffi::c_int
            {
                ip0 = ip0.sub(1);
                match0 = match0.sub(1);
                mLength = mLength.wrapping_add(1);
            }
        }

        // _match: Requires ip0, match0, offcode
        // Count the forward length.
        mLength = mLength.wrapping_add(ZSTD_count(ip0.add(mLength), match0.add(mLength), iend));

        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from_unsigned(anchor),
            anchor,
            iend,
            offcode,
            mLength,
        );

        ip0 = ip0.add(mLength);
        anchor = ip0;

        // Fill table and check for immediate repcode.
        if ip0 <= ilimit {
            // Fill Table
            *hashTable.add(ZSTD_hashPtr(
                base.offset(current0 as isize).add(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = current0.wrapping_add(2); // here because current+2 could be > iend-8
            *hashTable.add(ZSTD_hashPtr(
                ip0.sub(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = ip0.sub(2).offset_from(base) as core::ffi::c_long as u32;

            if rep_offset2 > 0 {
                // rep_offset2==0 means rep_offset2 is invalidated
                while ip0 <= ilimit
                    && MEM_read32(ip0 as *const core::ffi::c_void)
                        == MEM_read32(ip0.sub(rep_offset2 as usize) as *const core::ffi::c_void)
                {
                    // store sequence
                    let rLength =
                        (ZSTD_count(ip0.add(4), ip0.add(4).sub(rep_offset2 as usize), iend))
                            .wrapping_add(4);
                    core::mem::swap(&mut rep_offset2, &mut rep_offset1); // swap rep_offset2 <=> rep_offset1
                    *hashTable.add(ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls)) =
                        ip0.offset_from(base) as core::ffi::c_long as u32;
                    ip0 = ip0.add(rLength);
                    ZSTD_storeSeq(
                        seqStore,
                        0,
                        anchor,
                        iend,
                        REPCODE1_TO_OFFBASE as u32,
                        rLength,
                    );
                    anchor = ip0;
                }
            }
        }
    }

    // Note that there are probably still a couple positions one could search.
    // However, it seems to be a meaningful performance hit to try to search
    // them. So let's not.

    // When the repcodes are outside of the prefix, we set them to zero before the loop.
    // When the offsets are still zero, we need to restore them after the block to have a correct
    // repcode history. If only one offset was invalid, it is easy. The tricky case is when both
    // offsets were invalid. We need to figure out which offset to refill with.
    //     - If both offsets are zero they are in the same order.
    //     - If both offsets are non-zero, we won't restore the offsets from `offsetSaved[12]`.
    //     - If only one is zero, we need to decide which offset to restore.
    //         - If rep_offset1 is non-zero, then rep_offset2 must be offsetSaved1.
    //         - It is impossible for rep_offset2 to be non-zero.
    //
    // So if rep_offset1 started invalid (offsetSaved1 != 0) and became valid (rep_offset1 != 0), then
    // set rep[0] = rep_offset1 and rep[1] = offsetSaved1.
    offsetSaved2 = if offsetSaved1 != 0 && rep_offset1 != 0 {
        offsetSaved1
    } else {
        offsetSaved2
    };

    // save reps for next block
    *rep = if rep_offset1 != 0 {
        rep_offset1
    } else {
        offsetSaved1
    };
    *rep.add(1) = if rep_offset2 != 0 {
        rep_offset2
    } else {
        offsetSaved2
    };

    // Return the last literals size
    iend.offset_from_unsigned(anchor)
}

unsafe fn ZSTD_compressBlock_fast_noDict_4_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 4, true)
}

unsafe fn ZSTD_compressBlock_fast_noDict_5_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 5, true)
}

unsafe fn ZSTD_compressBlock_fast_noDict_6_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 6, true)
}

unsafe fn ZSTD_compressBlock_fast_noDict_7_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 7, true)
}

unsafe fn ZSTD_compressBlock_fast_noDict_4_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 4, false)
}

unsafe fn ZSTD_compressBlock_fast_noDict_5_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 5, false)
}

unsafe fn ZSTD_compressBlock_fast_noDict_6_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 6, false)
}

unsafe fn ZSTD_compressBlock_fast_noDict_7_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 7, false)
}

pub unsafe fn ZSTD_compressBlock_fast(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mml = ms.cParams.minMatch;
    // use cmov when "candidate in range" branch is likely unpredictable
    let useCmov = ms.cParams.windowLog < 19;
    if useCmov {
        match mml {
            5 => ZSTD_compressBlock_fast_noDict_5_1(ms, seqStore, rep, src, srcSize),
            6 => ZSTD_compressBlock_fast_noDict_6_1(ms, seqStore, rep, src, srcSize),
            7 => ZSTD_compressBlock_fast_noDict_7_1(ms, seqStore, rep, src, srcSize),
            _ => ZSTD_compressBlock_fast_noDict_4_1(ms, seqStore, rep, src, srcSize),
        }
    } else {
        // use a branch instead
        match mml {
            5 => ZSTD_compressBlock_fast_noDict_5_0(ms, seqStore, rep, src, srcSize),
            6 => ZSTD_compressBlock_fast_noDict_6_0(ms, seqStore, rep, src, srcSize),
            7 => ZSTD_compressBlock_fast_noDict_7_0(ms, seqStore, rep, src, srcSize),
            _ => ZSTD_compressBlock_fast_noDict_4_0(ms, seqStore, rep, src, srcSize),
        }
    }
}

#[inline(always)]
unsafe fn ZSTD_compressBlock_fast_dictMatchState_generic(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mls: u32,
    hasStep: u32,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let hashTable = ms.hashTable;
    let hlog = (*cParams).hashLog;
    // support stepSize of 0
    let stepSize =
        ((*cParams).targetLength).wrapping_add(((*cParams).targetLength == 0) as core::ffi::c_uint);
    let base = ms.window.base;
    let istart = src as *const u8;
    let mut ip0 = istart;
    let mut ip1 = ip0.offset(stepSize as isize); // we assert below that stepSize >= 1
    let mut anchor = istart;
    let prefixStartIndex = ms.window.dictLimit;
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.add(srcSize);
    let ilimit = iend.sub(HASH_READ_SIZE as usize);
    let mut offset_1 = *rep;
    let mut offset_2 = *rep.add(1);

    let dms = ms.dictMatchState;
    let dictCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashTable: *const u32 = (*dms).hashTable;
    let dictStartIndex = (*dms).window.dictLimit;
    let dictBase = (*dms).window.base;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictEnd = (*dms).window.nextSrc;
    let dictIndexDelta =
        prefixStartIndex.wrapping_sub(dictEnd.offset_from(dictBase) as core::ffi::c_long as u32);
    let dictAndPrefixLength = dictEnd
        .offset(istart.offset_from(prefixStart) as core::ffi::c_long as isize)
        .offset_from(dictStart) as core::ffi::c_long as u32;
    let dictHBits =
        ((*dictCParams).hashLog).wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as core::ffi::c_uint);

    // if a dictionary is still attached, it necessarily means that
    // it is within window size. So we just check it.
    let maxDistance = 1 << (*cParams).windowLog;
    let endIndex = (istart.offset_from_unsigned(base)).wrapping_add(srcSize) as u32;
    assert!(endIndex - prefixStartIndex <= maxDistance);

    let _ = hasStep; // not currently specialized on whether it's accelerated

    // ensure there will be no underflow
    // when translating a dict index into a local index
    assert!(prefixStartIndex as usize >= dictEnd as usize - dictBase as usize);

    if ms.prefetchCDictTables != 0 {
        let hashTableBytes = ((1 as core::ffi::c_int as size_t) << (*dictCParams).hashLog)
            .wrapping_mul(size_of::<u32>());
        let _ptr = dictHashTable as *const core::ffi::c_char;
        let _size = hashTableBytes;
        let mut _pos: size_t = 0;
        _pos = 0;
        while _pos < _size {
            _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
        }
    }

    // init
    ip0 = ip0.offset((dictAndPrefixLength == 0) as core::ffi::c_int as isize);

    // Outer search loop
    's_135: while ip1 <= ilimit {
        // repcode check at (ip0 + 1) is safe because ip0 < ip1
        let mut mLength: size_t = 0;
        let mut hash0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls);

        let dictHashAndTag0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, dictHBits, mls);
        let mut dictMatchIndexAndTag =
            *dictHashTable.add(dictHashAndTag0 >> ZSTD_SHORT_CACHE_TAG_BITS);
        let mut dictTagsMatch =
            ZSTD_comparePackedTags(dictMatchIndexAndTag as size_t, dictHashAndTag0);

        let mut matchIndex = *hashTable.add(hash0);
        let mut curr = ip0.offset_from(base) as core::ffi::c_long as u32;
        let mut step = stepSize as size_t;
        let kStepIncr = (1 << kSearchStrength) as size_t;
        let mut nextStep = ip0.add(kStepIncr);

        // Inner search loop
        loop {
            let mut match_0 = base.offset(matchIndex as isize);
            let repIndex = curr.wrapping_add(1).wrapping_sub(offset_1);
            let repMatch = if repIndex < prefixStartIndex {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
            } else {
                base.offset(repIndex as isize)
            };
            let hash1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, hlog, mls);
            let dictHashAndTag1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, dictHBits, mls);
            *hashTable.add(hash0) = curr; // update hash table

            if ZSTD_index_overlap_check(prefixStartIndex, repIndex)
                && MEM_read32(repMatch as *const core::ffi::c_void)
                    == MEM_read32(ip0.add(1) as *const core::ffi::c_void)
            {
                let repMatchEnd = if repIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                mLength = (ZSTD_count_2segments(
                    ip0.add(1).add(4),
                    repMatch.add(4),
                    iend,
                    repMatchEnd,
                    prefixStart,
                ))
                .wrapping_add(4);
                ip0 = ip0.add(1);
                ZSTD_storeSeq(
                    seqStore,
                    ip0.offset_from_unsigned(anchor),
                    anchor,
                    iend,
                    REPCODE1_TO_OFFBASE as u32,
                    mLength,
                );
                break;
            } else {
                if dictTagsMatch {
                    // Found a possible dict match
                    let dictMatchIndex = dictMatchIndexAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
                    let mut dictMatch = dictBase.offset(dictMatchIndex as isize);
                    // To replicate extDict parse behavior, we only use dict matches when the normal matchIndex is invalid
                    if dictMatchIndex > dictStartIndex
                        && MEM_read32(dictMatch as *const core::ffi::c_void)
                            == MEM_read32(ip0 as *const core::ffi::c_void)
                        && matchIndex <= prefixStartIndex
                    {
                        let offset = curr
                            .wrapping_sub(dictMatchIndex)
                            .wrapping_sub(dictIndexDelta);
                        mLength = (ZSTD_count_2segments(
                            ip0.add(4),
                            dictMatch.add(4),
                            iend,
                            dictEnd,
                            prefixStart,
                        ))
                        .wrapping_add(4);
                        while (ip0 > anchor) as core::ffi::c_int
                            & (dictMatch > dictStart) as core::ffi::c_int
                            != 0
                            && *ip0.sub(1) as core::ffi::c_int
                                == *dictMatch.sub(1) as core::ffi::c_int
                        {
                            ip0 = ip0.sub(1);
                            dictMatch = dictMatch.sub(1);
                            mLength = mLength.wrapping_add(1);
                        } // catch up
                        offset_2 = offset_1;
                        offset_1 = offset;
                        ZSTD_storeSeq(
                            seqStore,
                            ip0.offset_from_unsigned(anchor),
                            anchor,
                            iend,
                            offset.wrapping_add(ZSTD_REP_NUM as u32),
                            mLength,
                        );
                        break;
                    }
                }

                if ZSTD_match4Found_cmov(ip0, match_0, matchIndex, prefixStartIndex) {
                    // found a regular match of size >= 4
                    let offset_0 = ip0.offset_from(match_0) as core::ffi::c_long as u32;
                    mLength = (ZSTD_count(ip0.add(4), match_0.add(4), iend)).wrapping_add(4);
                    while (ip0 > anchor) as core::ffi::c_int
                        & (match_0 > prefixStart) as core::ffi::c_int
                        != 0
                        && *ip0.sub(1) as core::ffi::c_int == *match_0.sub(1) as core::ffi::c_int
                    {
                        ip0 = ip0.sub(1);
                        match_0 = match_0.sub(1);
                        mLength = mLength.wrapping_add(1);
                    } // catch up
                    offset_2 = offset_1;
                    offset_1 = offset_0;
                    ZSTD_storeSeq(
                        seqStore,
                        ip0.offset_from_unsigned(anchor),
                        anchor,
                        iend,
                        offset_0.wrapping_add(ZSTD_REP_NUM as u32),
                        mLength,
                    );
                    break;
                } else {
                    // Prepare for next iteration
                    dictMatchIndexAndTag =
                        *dictHashTable.add(dictHashAndTag1 >> ZSTD_SHORT_CACHE_TAG_BITS);
                    dictTagsMatch =
                        ZSTD_comparePackedTags(dictMatchIndexAndTag as size_t, dictHashAndTag1);
                    matchIndex = *hashTable.add(hash1);

                    if ip1 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.add(kStepIncr);
                    }
                    ip0 = ip1;
                    ip1 = ip1.add(step);
                    if ip1 > ilimit {
                        break 's_135;
                    }

                    curr = ip0.offset_from(base) as core::ffi::c_long as u32;
                    hash0 = hash1;
                }
            }
        }

        // match found
        ip0 = ip0.add(mLength);
        anchor = ip0;

        if ip0 <= ilimit {
            // Fill Table
            *hashTable.add(ZSTD_hashPtr(
                base.offset(curr as isize).add(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = curr.wrapping_add(2); // here because curr+2 could be > iend-8
            *hashTable.add(ZSTD_hashPtr(
                ip0.sub(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = ip0.sub(2).offset_from(base) as core::ffi::c_long as u32;

            // check immediate repcode
            while ip0 <= ilimit {
                let current2 = ip0.offset_from(base) as core::ffi::c_long as u32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase
                        .sub(dictIndexDelta as usize)
                        .offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixStartIndex, repIndex2)
                    && MEM_read32(repMatch2 as *const core::ffi::c_void)
                        == MEM_read32(ip0 as *const core::ffi::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                let repLength2 = (ZSTD_count_2segments(
                    ip0.add(4),
                    repMatch2.add(4),
                    iend,
                    repEnd2,
                    prefixStart,
                ))
                .wrapping_add(4);
                core::mem::swap(&mut offset_2, &mut offset_1);
                ZSTD_storeSeq(
                    seqStore,
                    0,
                    anchor,
                    iend,
                    REPCODE1_TO_OFFBASE as u32,
                    repLength2,
                );
                *hashTable.add(ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls)) = current2;
                ip0 = ip0.add(repLength2);
                anchor = ip0;
            }
        }

        // Prepare for next iteration
        ip1 = ip0.offset(stepSize as isize);
    }

    // save reps for next block
    *rep = offset_1;
    *rep.add(1) = offset_2;

    // Return the last literals size
    iend.offset_from_unsigned(anchor)
}

unsafe fn ZSTD_compressBlock_fast_dictMatchState_4_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 4, 0)
}

unsafe fn ZSTD_compressBlock_fast_dictMatchState_5_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 5, 0)
}

unsafe fn ZSTD_compressBlock_fast_dictMatchState_6_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 6, 0)
}

unsafe fn ZSTD_compressBlock_fast_dictMatchState_7_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 7, 0)
}

pub unsafe fn ZSTD_compressBlock_fast_dictMatchState(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mls = ms.cParams.minMatch;
    match mls {
        5 => ZSTD_compressBlock_fast_dictMatchState_5_0(ms, seqStore, rep, src, srcSize),
        6 => ZSTD_compressBlock_fast_dictMatchState_6_0(ms, seqStore, rep, src, srcSize),
        7 => ZSTD_compressBlock_fast_dictMatchState_7_0(ms, seqStore, rep, src, srcSize),
        _ => ZSTD_compressBlock_fast_dictMatchState_4_0(ms, seqStore, rep, src, srcSize),
    }
}

unsafe fn ZSTD_compressBlock_fast_extDict_generic(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mls: u32,
    hasStep: u32,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let hashTable = ms.hashTable;
    let hlog = (*cParams).hashLog;
    // support stepSize of 0
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as core::ffi::c_uint)
        .wrapping_add(1) as size_t;
    let base = ms.window.base;
    let dictBase = ms.window.dictBase;
    let istart = src as *const u8;
    let mut anchor = istart;
    let endIndex = (istart.offset_from_unsigned(base)).wrapping_add(srcSize) as u32;
    let lowLimit = ZSTD_getLowestMatchIndex(ms, endIndex, (*cParams).windowLog);
    let dictStartIndex = lowLimit;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictLimit = ms.window.dictLimit;
    let prefixStartIndex = dictLimit.max(lowLimit);
    let prefixStart = base.offset(prefixStartIndex as isize);
    let dictEnd = dictBase.offset(prefixStartIndex as isize);
    let iend = istart.add(srcSize);
    let ilimit = iend.sub(8);
    let mut offset_1 = *rep;
    let mut offset_2 = *rep.add(1);
    let mut offsetSaved1 = 0;
    let mut offsetSaved2 = 0;

    let mut ip0 = istart;
    let mut ip1 = core::ptr::null::<u8>();
    let mut ip2 = core::ptr::null::<u8>();
    let mut ip3 = core::ptr::null::<u8>();
    let mut current0: u32 = 0;

    let mut hash0: size_t = 0; // hash for ip0
    let mut hash1: size_t = 0; // hash for ip1
    let mut idx: u32 = 0; // match idx for ip0
    let mut idxBase = core::ptr::null::<u8>(); // base pointer for idx

    let mut offcode: u32 = 0;
    let mut match0 = core::ptr::null::<u8>();
    let mut mLength: size_t = 0;
    let mut matchEnd = core::ptr::null::<u8>(); // initialize to avoid warning, assert != 0 later

    let mut step: size_t = 0;
    let mut nextStep = core::ptr::null::<u8>();
    let kStepIncr = (1 << (kSearchStrength - 1)) as size_t;

    let _ = hasStep; // not currently specialized on whether it's accelerated

    // switch to "regular" variant if extDict is invalidated due to maxDistance
    if prefixStartIndex == dictStartIndex {
        return ZSTD_compressBlock_fast(ms, seqStore, rep, src, srcSize);
    }

    let curr = ip0.offset_from(base) as core::ffi::c_long as u32;
    let maxRep = curr.wrapping_sub(dictStartIndex);
    if offset_2 >= maxRep {
        offsetSaved2 = offset_2;
        offset_2 = 0;
    }
    if offset_1 >= maxRep {
        offsetSaved1 = offset_1;
        offset_1 = 0;
    }

    // start each op
    '__start: loop {
        step = stepSize;
        nextStep = ip0.add(kStepIncr);

        // calculate positions, ip0 - anchor == 0, so we skip step calc
        ip1 = ip0.add(1);
        ip2 = ip0.add(step);
        ip3 = ip2.add(1);

        if ip3 >= ilimit {
            break;
        }

        hash0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls);
        hash1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, hlog, mls);

        idx = *hashTable.add(hash0);
        idxBase = if idx < prefixStartIndex {
            dictBase
        } else {
            base
        };

        loop {
            // load repcode match for ip[2]
            let current2 = ip2.offset_from(base) as core::ffi::c_long as u32;
            let repIndex = current2.wrapping_sub(offset_1);
            let repBase = if repIndex < prefixStartIndex {
                dictBase
            } else {
                base
            };
            let mut rval: u32 = 0;
            if (prefixStartIndex.wrapping_sub(repIndex) >= 4) as core::ffi::c_int // intentional underflow
                    & (offset_1 > 0) as core::ffi::c_int
                != 0
            {
                rval = MEM_read32(repBase.offset(repIndex as isize) as *const core::ffi::c_void);
            } else {
                rval = MEM_read32(ip2 as *const core::ffi::c_void) ^ 1; // guaranteed to not match.
            }

            // write back hash table entry
            current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
            *hashTable.add(hash0) = current0;

            // check repcode at ip[2]
            if MEM_read32(ip2 as *const core::ffi::c_void) == rval {
                ip0 = ip2;
                match0 = repBase.offset(repIndex as isize);
                matchEnd = if repIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                mLength = (*ip0.sub(1) as core::ffi::c_int == *match0.sub(1) as core::ffi::c_int)
                    as core::ffi::c_int as size_t;
                ip0 = ip0.sub(mLength as usize);
                match0 = match0.sub(mLength as usize);
                offcode = REPCODE1_TO_OFFBASE as u32;
                mLength = mLength.wrapping_add(4);
                current_block = 1352918242886884122;
                break;
            } else {
                // load match for ip[0]
                let mval = if idx >= dictStartIndex {
                    MEM_read32(idxBase.offset(idx as isize) as *const core::ffi::c_void)
                } else {
                    MEM_read32(ip0 as *const core::ffi::c_void) ^ 1 // guaranteed not to match
                };

                // check match at ip[0]
                if MEM_read32(ip0 as *const core::ffi::c_void) == mval {
                    // found a match!
                    current_block = 934346911184053177;
                    break;
                } else {
                    // lookup ip[1]
                    idx = *hashTable.add(hash1);
                    idxBase = if idx < prefixStartIndex {
                        dictBase
                    } else {
                        base
                    };

                    // hash ip[2]
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);

                    // advance to next positions
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip3;

                    // write back hash table entry
                    current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
                    *hashTable.add(hash0) = current0;

                    // load match for ip[0]
                    let mval_0 = if idx >= dictStartIndex {
                        MEM_read32(idxBase.offset(idx as isize) as *const core::ffi::c_void)
                    } else {
                        MEM_read32(ip0 as *const core::ffi::c_void) ^ 1 // guaranteed not to match
                    };
                    // check match at ip[0]
                    if MEM_read32(ip0 as *const core::ffi::c_void) == mval_0 {
                        // found a match!
                        current_block = 934346911184053177;
                        break;
                    }

                    // lookup ip[1]
                    idx = *hashTable.add(hash1);
                    idxBase = if idx < prefixStartIndex {
                        dictBase
                    } else {
                        base
                    };

                    // hash ip[2]
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);

                    // advance to next positions
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip0.add(step);
                    ip3 = ip1.add(step);

                    // calculate step
                    if ip2 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.add(kStepIncr);
                    }
                    if ip3 >= ilimit {
                        break '__start;
                    }
                }
            }
        }

        // _offset: Requires ip0, idx, idxBase
        if current_block == 934346911184053177 {
            // Compute the offset code.
            let offset = current0.wrapping_sub(idx);
            let lowMatchPtr = if idx < prefixStartIndex {
                dictStart
            } else {
                prefixStart
            };
            matchEnd = if idx < prefixStartIndex {
                dictEnd
            } else {
                iend
            };
            match0 = idxBase.offset(idx as isize);
            offset_2 = offset_1;
            offset_1 = offset;
            offcode = offset.wrapping_add(ZSTD_REP_NUM as u32);
            mLength = 4;

            // Count the backwards match length.
            while (ip0 > anchor) as core::ffi::c_int & (match0 > lowMatchPtr) as core::ffi::c_int
                != 0
                && *ip0.sub(1) as core::ffi::c_int == *match0.sub(1) as core::ffi::c_int
            {
                ip0 = ip0.sub(1);
                match0 = match0.sub(1);
                mLength = mLength.wrapping_add(1);
            }
        }

        // _match: Requires ip0, match0, offcode, matchEnd
        // Count the forward length.
        mLength = mLength.wrapping_add(ZSTD_count_2segments(
            ip0.add(mLength),
            match0.add(mLength),
            iend,
            matchEnd,
            prefixStart,
        ));

        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from_unsigned(anchor),
            anchor,
            iend,
            offcode,
            mLength,
        );

        ip0 = ip0.add(mLength);
        anchor = ip0;

        // write next hash table entry
        if ip1 < ip0 {
            *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;
        }

        // Fill table and check for immediate repcode.
        if ip0 <= ilimit {
            // Fill Table
            *hashTable.add(ZSTD_hashPtr(
                base.offset(current0 as isize).add(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = current0.wrapping_add(2); // here because current+2 could be > iend-8
            *hashTable.add(ZSTD_hashPtr(
                ip0.sub(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = ip0.sub(2).offset_from(base) as core::ffi::c_long as u32;

            while ip0 <= ilimit {
                let repIndex2 =
                    (ip0.offset_from(base) as core::ffi::c_long as u32).wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase.offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixStartIndex, repIndex2) & (offset_2 > 0)
                    && MEM_read32(repMatch2 as *const core::ffi::c_void)
                        == MEM_read32(ip0 as *const core::ffi::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                let repLength2 = (ZSTD_count_2segments(
                    ip0.add(4),
                    repMatch2.add(4),
                    iend,
                    repEnd2,
                    prefixStart,
                ))
                .wrapping_add(4);
                core::mem::swap(&mut offset_2, &mut offset_1);
                ZSTD_storeSeq(
                    seqStore,
                    0,
                    anchor,
                    iend,
                    REPCODE1_TO_OFFBASE as u32,
                    repLength2,
                );
                *hashTable.add(ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls)) =
                    ip0.offset_from(base) as core::ffi::c_long as u32;
                ip0 = ip0.add(repLength2);
                anchor = ip0;
            }
        }
    }

    // Note that there are probably still a couple positions we could search.
    // However, it seems to be a meaningful performance hit to try to search
    // them. So let's not.

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

unsafe fn ZSTD_compressBlock_fast_extDict_4_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 4, 0)
}

unsafe fn ZSTD_compressBlock_fast_extDict_5_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 5, 0)
}

unsafe fn ZSTD_compressBlock_fast_extDict_6_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 6, 0)
}

unsafe fn ZSTD_compressBlock_fast_extDict_7_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 7, 0)
}

pub unsafe fn ZSTD_compressBlock_fast_extDict(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mls = ms.cParams.minMatch;
    match mls {
        5 => ZSTD_compressBlock_fast_extDict_5_0(ms, seqStore, rep, src, srcSize),
        6 => ZSTD_compressBlock_fast_extDict_6_0(ms, seqStore, rep, src, srcSize),
        7 => ZSTD_compressBlock_fast_extDict_7_0(ms, seqStore, rep, src, srcSize),
        _ => ZSTD_compressBlock_fast_extDict_4_0(ms, seqStore, rep, src, srcSize),
    }
}
