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
pub type ZSTD_match4Found = Option<unsafe fn(*const u8, *const u8, u32, u32) -> core::ffi::c_int>;
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
use crate::lib::zstd::*;
pub const kSearchStrength: core::ffi::c_int = 8;
pub const HASH_READ_SIZE: core::ffi::c_int = 8;

pub const REPCODE1_TO_OFFBASE: core::ffi::c_int = 1;

pub const ZSTD_SHORT_CACHE_TAG_BITS: core::ffi::c_int = 8;
pub const ZSTD_SHORT_CACHE_TAG_MASK: core::ffi::c_uint =
    ((1 as core::ffi::c_uint) << ZSTD_SHORT_CACHE_TAG_BITS).wrapping_sub(1);
#[inline]
unsafe fn ZSTD_writeTaggedIndex(hashTable: *mut u32, hashAndTag: size_t, index: u32) {
    let hash = hashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
    let tag = (hashAndTag & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    *hashTable.add(hash) = index << ZSTD_SHORT_CACHE_TAG_BITS | tag;
}
#[inline]
unsafe fn ZSTD_comparePackedTags(packedTag1: size_t, packedTag2: size_t) -> core::ffi::c_int {
    let tag1 = (packedTag1 & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    let tag2 = (packedTag2 & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    core::ffi::c_int::from(tag1 == tag2)
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
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3;
    while ip.offset(fastHashFillStep as isize) < iend.add(2) {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let hashAndTag = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBits, mls);
        ZSTD_writeTaggedIndex(hashTable, hashAndTag, curr);
        if dtlm as core::ffi::c_uint != ZSTD_dtlm_fast as core::ffi::c_int as core::ffi::c_uint {
            let mut p: u32 = 0;
            p = 1;
            while p < fastHashFillStep {
                let hashAndTag_0 = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const core::ffi::c_void,
                    hBits,
                    mls,
                );
                if *hashTable.add(hashAndTag_0 >> ZSTD_SHORT_CACHE_TAG_BITS) == 0 {
                    ZSTD_writeTaggedIndex(hashTable, hashAndTag_0, curr.wrapping_add(p));
                }
                p = p.wrapping_add(1);
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
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3;
    while ip.offset(fastHashFillStep as isize) < iend.add(2) {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let hash0 = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBits, mls);
        *hashTable.add(hash0) = curr;
        if dtlm as core::ffi::c_uint != ZSTD_dtlm_fast as core::ffi::c_int as core::ffi::c_uint {
            let mut p: u32 = 0;
            p = 1;
            while p < fastHashFillStep {
                let hash = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const core::ffi::c_void,
                    hBits,
                    mls,
                );
                if *hashTable.add(hash) == 0 {
                    *hashTable.add(hash) = curr.wrapping_add(p);
                }
                p = p.wrapping_add(1);
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
    if tfp as core::ffi::c_uint == ZSTD_tfp_forCDict as core::ffi::c_int as core::ffi::c_uint {
        ZSTD_fillHashTableForCDict(ms, end, dtlm);
    } else {
        ZSTD_fillHashTableForCCtx(ms, end, dtlm);
    };
}

unsafe fn ZSTD_match4Found_cmov(
    currentPtr: *const u8,
    matchAddress: *const u8,
    matchIdx: u32,
    idxLowLimit: u32,
) -> core::ffi::c_int {
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
        return 0;
    }

    // force ordering of these tests, which matters once the function is inlined, as they become branches.
    #[cfg(not(target_family = "wasm"))]
    asm!("", options(preserves_flags));

    core::ffi::c_int::from(matchIdx >= idxLowLimit)
}

unsafe fn ZSTD_match4Found_branch(
    currentPtr: *const u8,
    matchAddress: *const u8,
    matchIdx: u32,
    idxLowLimit: u32,
) -> core::ffi::c_int {
    let mut mval: u32 = 0;
    if matchIdx >= idxLowLimit {
        mval = MEM_read32(matchAddress as *const core::ffi::c_void);
    } else {
        mval = MEM_read32(currentPtr as *const core::ffi::c_void) ^ 1;
    }
    core::ffi::c_int::from(MEM_read32(currentPtr as *const core::ffi::c_void) == mval)
}
#[inline(always)]
unsafe fn ZSTD_compressBlock_fast_noDict_generic(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mls: u32,
    useCmov: core::ffi::c_int,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let hashTable = ms.hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(core::ffi::c_int::from((*cParams).targetLength == 0) as core::ffi::c_uint)
        .wrapping_add(1) as size_t;
    let base = ms.window.base;
    let istart = src as *const u8;
    let endIndex = (istart.offset_from_unsigned(base)).wrapping_add(srcSize) as u32;
    let prefixStartIndex = ZSTD_getLowestPrefixIndex(ms, endIndex, (*cParams).windowLog);
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.add(srcSize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
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
    let mut hash0: size_t = 0;
    let mut hash1: size_t = 0;
    let mut matchIdx: u32 = 0;
    let mut offcode: u32 = 0;
    let mut match0 = core::ptr::null::<u8>();
    let mut mLength: size_t = 0;
    let mut step: size_t = 0;
    let mut nextStep = core::ptr::null::<u8>();
    let kStepIncr = ((1) << (kSearchStrength - 1)) as size_t;
    let matchFound: ZSTD_match4Found = if useCmov != 0 {
        Some(ZSTD_match4Found_cmov as unsafe fn(*const u8, *const u8, u32, u32) -> core::ffi::c_int)
    } else {
        Some(
            ZSTD_match4Found_branch
                as unsafe fn(*const u8, *const u8, u32, u32) -> core::ffi::c_int,
        )
    };
    ip0 = ip0.offset(core::ffi::c_int::from(ip0 == prefixStart) as isize);
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
    '__start: loop {
        step = stepSize;
        nextStep = ip0.add(kStepIncr);
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
            let rval = MEM_read32(ip2.offset(-(rep_offset1 as isize)) as *const core::ffi::c_void);
            current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
            *hashTable.add(hash0) = current0;
            if core::ffi::c_int::from(MEM_read32(ip2 as *const core::ffi::c_void) == rval)
                & core::ffi::c_int::from(rep_offset1 > 0)
                != 0
            {
                ip0 = ip2;
                match0 = ip0.offset(-(rep_offset1 as isize));
                mLength = core::ffi::c_int::from(
                    core::ffi::c_int::from(*ip0.sub(1)) == core::ffi::c_int::from(*match0.sub(1)),
                ) as size_t;
                ip0 = ip0.offset(-(mLength as isize));
                match0 = match0.offset(-(mLength as isize));
                offcode = REPCODE1_TO_OFFBASE as u32;
                mLength = mLength.wrapping_add(4);
                *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;
                current_block = 4391991184774404966;
                break;
            } else if matchFound.unwrap_unchecked()(
                ip0,
                base.offset(matchIdx as isize),
                matchIdx,
                prefixStartIndex,
            ) != 0
            {
                *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;
                current_block = 11113405673187116881;
                break;
            } else {
                matchIdx = *hashTable.add(hash1);
                hash0 = hash1;
                hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);
                ip0 = ip1;
                ip1 = ip2;
                ip2 = ip3;
                current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
                *hashTable.add(hash0) = current0;
                if matchFound.unwrap_unchecked()(
                    ip0,
                    base.offset(matchIdx as isize),
                    matchIdx,
                    prefixStartIndex,
                ) != 0
                {
                    if step <= 4 {
                        *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;
                    }
                    current_block = 11113405673187116881;
                    break;
                } else {
                    matchIdx = *hashTable.add(hash1);
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip0.add(step);
                    ip3 = ip1.add(step);
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
        if current_block == 11113405673187116881 {
            match0 = base.offset(matchIdx as isize);
            rep_offset2 = rep_offset1;
            rep_offset1 = ip0.offset_from(match0) as core::ffi::c_long as u32;
            offcode = rep_offset1.wrapping_add(ZSTD_REP_NUM as u32);
            mLength = 4;
            while core::ffi::c_int::from(ip0 > anchor)
                & core::ffi::c_int::from(match0 > prefixStart)
                != 0
                && core::ffi::c_int::from(*ip0.sub(1)) == core::ffi::c_int::from(*match0.sub(1))
            {
                ip0 = ip0.sub(1);
                match0 = match0.sub(1);
                mLength = mLength.wrapping_add(1);
            }
        }
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
        if ip0 <= ilimit {
            *hashTable.add(ZSTD_hashPtr(
                base.offset(current0 as isize).add(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = current0.wrapping_add(2);
            *hashTable.add(ZSTD_hashPtr(
                ip0.sub(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = ip0.sub(2).offset_from(base) as core::ffi::c_long as u32;
            if rep_offset2 > 0 {
                while ip0 <= ilimit
                    && MEM_read32(ip0 as *const core::ffi::c_void)
                        == MEM_read32(
                            ip0.offset(-(rep_offset2 as isize)) as *const core::ffi::c_void
                        )
                {
                    let rLength =
                        (ZSTD_count(ip0.add(4), ip0.add(4).offset(-(rep_offset2 as isize)), iend))
                            .wrapping_add(4);
                    core::mem::swap(&mut rep_offset2, &mut rep_offset1);
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
    offsetSaved2 = if offsetSaved1 != 0 && rep_offset1 != 0 {
        offsetSaved1
    } else {
        offsetSaved2
    };
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
    iend.offset_from_unsigned(anchor)
}
unsafe fn ZSTD_compressBlock_fast_noDict_4_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 4, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_5_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 5, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_6_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 6, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_7_1(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 7, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_4_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 4, 0)
}
unsafe fn ZSTD_compressBlock_fast_noDict_5_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 5, 0)
}
unsafe fn ZSTD_compressBlock_fast_noDict_6_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 6, 0)
}
unsafe fn ZSTD_compressBlock_fast_noDict_7_0(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 7, 0)
}
pub unsafe fn ZSTD_compressBlock_fast(
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mml = ms.cParams.minMatch;
    let useCmov = core::ffi::c_int::from(ms.cParams.windowLog < 19);
    if useCmov != 0 {
        match mml {
            5 => ZSTD_compressBlock_fast_noDict_5_1(ms, seqStore, rep, src, srcSize),
            6 => ZSTD_compressBlock_fast_noDict_6_1(ms, seqStore, rep, src, srcSize),
            7 => ZSTD_compressBlock_fast_noDict_7_1(ms, seqStore, rep, src, srcSize),
            _ => ZSTD_compressBlock_fast_noDict_4_1(ms, seqStore, rep, src, srcSize),
        }
    } else {
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
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(core::ffi::c_int::from((*cParams).targetLength == 0) as core::ffi::c_uint);
    let base = ms.window.base;
    let istart = src as *const u8;
    let mut ip0 = istart;
    let mut ip1 = ip0.offset(stepSize as isize);
    let mut anchor = istart;
    let prefixStartIndex = ms.window.dictLimit;
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.add(srcSize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
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
    let maxDistance = (1) << (*cParams).windowLog;
    let endIndex = (istart.offset_from_unsigned(base)).wrapping_add(srcSize) as u32;
    assert!(endIndex - prefixStartIndex <= maxDistance);

    let _ = hasStep; /* not currently specialized on whether it's accelerated */

    /* ensure there will be no underflow
     * when translating a dict index into a local index */
    assert!(prefixStartIndex as usize >= dictEnd as usize - dictBase as usize);

    if ms.prefetchCDictTables != 0 {
        let hashTableBytes = ((1 as core::ffi::c_int as size_t) << (*dictCParams).hashLog)
            .wrapping_mul(::core::mem::size_of::<u32>());
        let _ptr = dictHashTable as *const core::ffi::c_char;
        let _size = hashTableBytes;
        let mut _pos: size_t = 0;
        _pos = 0;
        while _pos < _size {
            _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
        }
    }
    ip0 = ip0.offset(core::ffi::c_int::from(dictAndPrefixLength == 0) as isize);
    's_135: while ip1 <= ilimit {
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
        let kStepIncr = ((1) << kSearchStrength) as size_t;
        let mut nextStep = ip0.add(kStepIncr);
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
            *hashTable.add(hash0) = curr;
            if ZSTD_index_overlap_check(prefixStartIndex, repIndex) != 0
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
                if dictTagsMatch != 0 {
                    let dictMatchIndex = dictMatchIndexAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
                    let mut dictMatch = dictBase.offset(dictMatchIndex as isize);
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
                        while core::ffi::c_int::from(ip0 > anchor)
                            & core::ffi::c_int::from(dictMatch > dictStart)
                            != 0
                            && core::ffi::c_int::from(*ip0.sub(1))
                                == core::ffi::c_int::from(*dictMatch.sub(1))
                        {
                            ip0 = ip0.sub(1);
                            dictMatch = dictMatch.sub(1);
                            mLength = mLength.wrapping_add(1);
                        }
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
                if ZSTD_match4Found_cmov(ip0, match_0, matchIndex, prefixStartIndex) != 0 {
                    let offset_0 = ip0.offset_from(match_0) as core::ffi::c_long as u32;
                    mLength = (ZSTD_count(ip0.add(4), match_0.add(4), iend)).wrapping_add(4);
                    while core::ffi::c_int::from(ip0 > anchor)
                        & core::ffi::c_int::from(match_0 > prefixStart)
                        != 0
                        && core::ffi::c_int::from(*ip0.sub(1))
                            == core::ffi::c_int::from(*match_0.sub(1))
                    {
                        ip0 = ip0.sub(1);
                        match_0 = match_0.sub(1);
                        mLength = mLength.wrapping_add(1);
                    }
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
        ip0 = ip0.add(mLength);
        anchor = ip0;
        if ip0 <= ilimit {
            *hashTable.add(ZSTD_hashPtr(
                base.offset(curr as isize).add(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = curr.wrapping_add(2);
            *hashTable.add(ZSTD_hashPtr(
                ip0.sub(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = ip0.sub(2).offset_from(base) as core::ffi::c_long as u32;
            while ip0 <= ilimit {
                let current2 = ip0.offset_from(base) as core::ffi::c_long as u32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase
                        .offset(-(dictIndexDelta as isize))
                        .offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixStartIndex, repIndex2) != 0
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
        ip1 = ip0.offset(stepSize as isize);
    }
    *rep = offset_1;
    *rep.add(1) = offset_2;
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
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(core::ffi::c_int::from((*cParams).targetLength == 0) as core::ffi::c_uint)
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
    let prefixStartIndex = if dictLimit < lowLimit {
        lowLimit
    } else {
        dictLimit
    };
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
    let mut hash0: size_t = 0;
    let mut hash1: size_t = 0;
    let mut idx: u32 = 0;
    let mut idxBase = core::ptr::null::<u8>();
    let mut offcode: u32 = 0;
    let mut match0 = core::ptr::null::<u8>();
    let mut mLength: size_t = 0;
    let mut matchEnd = core::ptr::null::<u8>();
    let mut step: size_t = 0;
    let mut nextStep = core::ptr::null::<u8>();
    let kStepIncr = ((1) << (kSearchStrength - 1)) as size_t;

    let _ = hasStep; /* not currently specialized on whether it's accelerated */

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
    '__start: loop {
        step = stepSize;
        nextStep = ip0.add(kStepIncr);
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
            let current2 = ip2.offset_from(base) as core::ffi::c_long as u32;
            let repIndex = current2.wrapping_sub(offset_1);
            let repBase = if repIndex < prefixStartIndex {
                dictBase
            } else {
                base
            };
            let mut rval: u32 = 0;
            if core::ffi::c_int::from(prefixStartIndex.wrapping_sub(repIndex) >= 4)
                & core::ffi::c_int::from(offset_1 > 0)
                != 0
            {
                rval = MEM_read32(repBase.offset(repIndex as isize) as *const core::ffi::c_void);
            } else {
                rval = MEM_read32(ip2 as *const core::ffi::c_void) ^ 1;
            }
            current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
            *hashTable.add(hash0) = current0;
            if MEM_read32(ip2 as *const core::ffi::c_void) == rval {
                ip0 = ip2;
                match0 = repBase.offset(repIndex as isize);
                matchEnd = if repIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                mLength = core::ffi::c_int::from(
                    core::ffi::c_int::from(*ip0.sub(1)) == core::ffi::c_int::from(*match0.sub(1)),
                ) as size_t;
                ip0 = ip0.offset(-(mLength as isize));
                match0 = match0.offset(-(mLength as isize));
                offcode = REPCODE1_TO_OFFBASE as u32;
                mLength = mLength.wrapping_add(4);
                current_block = 1352918242886884122;
                break;
            } else {
                let mval = if idx >= dictStartIndex {
                    MEM_read32(idxBase.offset(idx as isize) as *const core::ffi::c_void)
                } else {
                    MEM_read32(ip0 as *const core::ffi::c_void) ^ 1
                };
                if MEM_read32(ip0 as *const core::ffi::c_void) == mval {
                    current_block = 934346911184053177;
                    break;
                } else {
                    idx = *hashTable.add(hash1);
                    idxBase = if idx < prefixStartIndex {
                        dictBase
                    } else {
                        base
                    };
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip3;
                    current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
                    *hashTable.add(hash0) = current0;
                    let mval_0 = if idx >= dictStartIndex {
                        MEM_read32(idxBase.offset(idx as isize) as *const core::ffi::c_void)
                    } else {
                        MEM_read32(ip0 as *const core::ffi::c_void) ^ 1
                    };
                    if MEM_read32(ip0 as *const core::ffi::c_void) == mval_0 {
                        current_block = 934346911184053177;
                        break;
                    }
                    idx = *hashTable.add(hash1);
                    idxBase = if idx < prefixStartIndex {
                        dictBase
                    } else {
                        base
                    };
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip0.add(step);
                    ip3 = ip1.add(step);
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
        if current_block == 934346911184053177 {
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
            while core::ffi::c_int::from(ip0 > anchor)
                & core::ffi::c_int::from(match0 > lowMatchPtr)
                != 0
                && core::ffi::c_int::from(*ip0.sub(1)) == core::ffi::c_int::from(*match0.sub(1))
            {
                ip0 = ip0.sub(1);
                match0 = match0.sub(1);
                mLength = mLength.wrapping_add(1);
            }
        }
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
        if ip1 < ip0 {
            *hashTable.add(hash1) = ip1.offset_from(base) as core::ffi::c_long as u32;
        }
        if ip0 <= ilimit {
            *hashTable.add(ZSTD_hashPtr(
                base.offset(current0 as isize).add(2) as *const core::ffi::c_void,
                hlog,
                mls,
            )) = current0.wrapping_add(2);
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
                if !(ZSTD_index_overlap_check(prefixStartIndex, repIndex2)
                    & core::ffi::c_int::from(offset_2 > 0)
                    != 0
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
    offsetSaved2 = if offsetSaved1 != 0 && offset_1 != 0 {
        offsetSaved1
    } else {
        offsetSaved2
    };
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
