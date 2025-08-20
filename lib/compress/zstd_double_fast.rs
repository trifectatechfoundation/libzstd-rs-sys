use core::arch::asm;
pub type ZSTD_longLengthType_e = core::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
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
pub type ZSTD_ParamSwitch_e = core::ffi::c_uint;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
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
pub type ZSTD_OptPrice_e = core::ffi::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const u8,
    pub base: *const u8,
    pub dictBase: *const u8,
    pub dictLimit: u32,
    pub lowLimit: u32,
    pub nbOverflowCorrections: u32,
}
pub type ZSTD_dictTableLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = core::ffi::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
pub const CACHELINE_SIZE: core::ffi::c_int = 64;

use libc::size_t;

use crate::lib::common::fse::{FSE_CTable, FSE_repeat};
use crate::lib::common::huf::{HUF_CElt, HUF_repeat};
use crate::lib::common::mem::{
    MEM_64bits, MEM_isLittleEndian, MEM_read16, MEM_read32, MEM_read64, MEM_readLE32, MEM_readLE64,
    MEM_readST,
};
use crate::lib::common::zstd_internal::{
    Overlap, ZSTD_copy16, ZSTD_wildcopy, MINMATCH, WILDCOPY_OVERLENGTH, ZSTD_REP_NUM,
};
use crate::lib::compress::zstd_compress::{
    SeqStore_t, ZSTD_MatchState_t, ZSTD_match_t, ZSTD_optimal_t,
};
use crate::lib::zstd::*;
pub const kSearchStrength: core::ffi::c_int = 8;
pub const HASH_READ_SIZE: core::ffi::c_int = 8;
#[inline]
unsafe fn ZSTD_selectAddr(
    index: u32,
    lowLimit: u32,
    mut candidate: *const u8,
    backup: *const u8,
) -> *const u8 {
    asm!(
        "cmp {1:e}, {2:e}
        cmova {3}, {0}",
        inlateout(reg) candidate,
        inlateout(reg) index => _,
        inlateout(reg) lowLimit => _,
        inlateout(reg) backup => _,
        options(preserves_flags, pure, readonly, att_syntax)
    );
    candidate
}
unsafe fn ZSTD_safecopyLiterals(
    mut op: *mut u8,
    mut ip: *const u8,
    iend: *const u8,
    ilimit_w: *const u8,
) {
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            ip as *const core::ffi::c_void,
            ilimit_w.offset_from(ip) as size_t,
            Overlap::NoOverlap,
        );
        op = op.offset(ilimit_w.offset_from(ip) as core::ffi::c_long as isize);
        ip = ilimit_w;
    }
    while ip < iend {
        let fresh0 = ip;
        ip = ip.offset(1);
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 = *fresh0;
    }
}
pub const REPCODE1_TO_OFFBASE: core::ffi::c_int = 1;
#[inline(always)]
unsafe fn ZSTD_storeSeqOnly(
    seqStorePtr: *mut SeqStore_t,
    litLength: size_t,
    offBase: u32,
    matchLength: size_t,
) {
    if (litLength > 0xffff as core::ffi::c_int as size_t) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength;
        (*seqStorePtr).longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart)
            as core::ffi::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0)).litLength = litLength as u16;
    (*((*seqStorePtr).sequences).offset(0)).offBase = offBase;
    let mlBase = matchLength.wrapping_sub(MINMATCH as size_t);
    if (mlBase > 0xffff as core::ffi::c_int as size_t) as core::ffi::c_int as core::ffi::c_long != 0
    {
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr).longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart)
            as core::ffi::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0)).mlBase = mlBase as u16;
    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
    (*seqStorePtr).sequences;
}
#[inline(always)]
unsafe fn ZSTD_storeSeq(
    seqStorePtr: *mut SeqStore_t,
    litLength: size_t,
    literals: *const u8,
    litLimit: *const u8,
    offBase: u32,
    matchLength: size_t,
) {
    let litLimit_w = litLimit.sub(WILDCOPY_OVERLENGTH);
    let litEnd = literals.add(litLength);
    if litEnd <= litLimit_w {
        ZSTD_copy16(
            (*seqStorePtr).lit as *mut core::ffi::c_void,
            literals as *const core::ffi::c_void,
        );
        if litLength > 16 {
            ZSTD_wildcopy(
                ((*seqStorePtr).lit).offset(16) as *mut core::ffi::c_void,
                literals.offset(16) as *const core::ffi::c_void,
                litLength.wrapping_sub(16),
                Overlap::NoOverlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals((*seqStorePtr).lit, literals, litEnd, litLimit_w);
    }
    (*seqStorePtr).lit = ((*seqStorePtr).lit).add(litLength);
    ZSTD_storeSeqOnly(seqStorePtr, litLength, offBase, matchLength);
}
#[inline]
unsafe fn ZSTD_count(mut pIn: *const u8, mut pMatch: *const u8, pInLimit: *const u8) -> size_t {
    let pStart = pIn;
    let pInLoopLimit = pInLimit.offset(
        -((::core::mem::size_of::<size_t>() as core::ffi::c_ulong).wrapping_sub(1) as isize),
    );
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const core::ffi::c_void)
            ^ MEM_readST(pIn as *const core::ffi::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as size_t;
        }
        pIn = pIn.offset(::core::mem::size_of::<size_t>() as core::ffi::c_ulong as isize);
        pMatch = pMatch.offset(::core::mem::size_of::<size_t>() as core::ffi::c_ulong as isize);
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const core::ffi::c_void)
                ^ MEM_readST(pIn as *const core::ffi::c_void);
            if diff_0 == 0 {
                pIn = pIn.offset(::core::mem::size_of::<size_t>() as core::ffi::c_ulong as isize);
                pMatch =
                    pMatch.offset(::core::mem::size_of::<size_t>() as core::ffi::c_ulong as isize);
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as size_t;
            }
        }
    }
    if MEM_64bits() != 0
        && pIn < pInLimit.offset(-(3))
        && MEM_read32(pMatch as *const core::ffi::c_void)
            == MEM_read32(pIn as *const core::ffi::c_void)
    {
        pIn = pIn.offset(4);
        pMatch = pMatch.offset(4);
    }
    if pIn < pInLimit.offset(-(1))
        && MEM_read16(pMatch as *const core::ffi::c_void) as core::ffi::c_int
            == MEM_read16(pIn as *const core::ffi::c_void) as core::ffi::c_int
    {
        pIn = pIn.offset(2);
        pMatch = pMatch.offset(2);
    }
    if pIn < pInLimit && *pMatch as core::ffi::c_int == *pIn as core::ffi::c_int {
        pIn = pIn.offset(1);
    }
    pIn.offset_from(pStart) as size_t
}
#[inline]
unsafe fn ZSTD_count_2segments(
    ip: *const u8,
    match_0: *const u8,
    iEnd: *const u8,
    mEnd: *const u8,
    iStart: *const u8,
) -> size_t {
    let vEnd = if ip.offset(mEnd.offset_from(match_0) as core::ffi::c_long as isize) < iEnd {
        ip.offset(mEnd.offset_from(match_0) as core::ffi::c_long as isize)
    } else {
        iEnd
    };
    let matchLength = ZSTD_count(ip, match_0, vEnd);
    if match_0.add(matchLength) != mEnd {
        return matchLength;
    }
    matchLength.wrapping_add(ZSTD_count(ip.add(matchLength), iStart, iEnd))
}
static prime4bytes: u32 = 2654435761;
unsafe fn ZSTD_hash4(u: u32, h: u32, s: u32) -> u32 {
    ((u.wrapping_mul(prime4bytes)) ^ s) >> 32u32.wrapping_sub(h)
}
unsafe fn ZSTD_hash4Ptr(ptr: *const core::ffi::c_void, h: u32) -> size_t {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0) as size_t
}
static prime5bytes: u64 = 889523592379;
unsafe fn ZSTD_hash5(u: u64, h: u32, s: u64) -> size_t {
    ((((u << (64 - 40)) * prime5bytes) ^ s) >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash5Ptr(p: *const core::ffi::c_void, h: u32) -> size_t {
    ZSTD_hash5(MEM_readLE64(p), h, 0)
}
static prime6bytes: u64 = 227718039650203;
unsafe fn ZSTD_hash6(u: u64, h: u32, s: u64) -> size_t {
    ((((u << (64 - 48)) * prime6bytes) ^ s) >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash6Ptr(p: *const core::ffi::c_void, h: u32) -> size_t {
    ZSTD_hash6(MEM_readLE64(p), h, 0)
}
static prime7bytes: u64 = 58295818150454627;
unsafe fn ZSTD_hash7(u: u64, h: u32, s: u64) -> size_t {
    ((((u << (64 - 56)) * prime7bytes) ^ s) >> (64u32).wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash7Ptr(p: *const core::ffi::c_void, h: u32) -> size_t {
    ZSTD_hash7(MEM_readLE64(p), h, 0)
}
static prime8bytes: u64 = 0xcf1bbcdcb7a56463 as core::ffi::c_ulonglong;
unsafe fn ZSTD_hash8(u: u64, h: u32, s: u64) -> size_t {
    (((u.wrapping_mul(prime8bytes)) ^ s) >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash8Ptr(p: *const core::ffi::c_void, h: u32) -> size_t {
    ZSTD_hash8(MEM_readLE64(p), h, 0)
}
#[inline(always)]
unsafe fn ZSTD_hashPtr(p: *const core::ffi::c_void, hBits: u32, mls: u32) -> size_t {
    match mls {
        5 => ZSTD_hash5Ptr(p, hBits),
        6 => ZSTD_hash6Ptr(p, hBits),
        7 => ZSTD_hash7Ptr(p, hBits),
        8 => ZSTD_hash8Ptr(p, hBits),
        4 | _ => ZSTD_hash4Ptr(p, hBits),
    }
}
#[inline]
unsafe fn ZSTD_getLowestMatchIndex(
    ms: *const ZSTD_MatchState_t,
    curr: u32,
    windowLog: core::ffi::c_uint,
) -> u32 {
    let maxDistance = (1) << windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0) as core::ffi::c_int as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}
#[inline]
unsafe fn ZSTD_getLowestPrefixIndex(
    ms: *const ZSTD_MatchState_t,
    curr: u32,
    windowLog: core::ffi::c_uint,
) -> u32 {
    let maxDistance = (1) << windowLog;
    let lowestValid = (*ms).window.dictLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = ((*ms).loadedDictEnd != 0) as core::ffi::c_int as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}
#[inline]
unsafe fn ZSTD_index_overlap_check(prefixLowestIndex: u32, repIndex: u32) -> core::ffi::c_int {
    (prefixLowestIndex.wrapping_sub(1).wrapping_sub(repIndex) >= 3) as core::ffi::c_int
}
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
    (tag1 == tag2) as core::ffi::c_int
}
#[inline]
unsafe fn ZSTD_countTrailingZeros32(val: u32) -> core::ffi::c_uint {
    val.trailing_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_countLeadingZeros32(val: u32) -> core::ffi::c_uint {
    val.leading_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_countTrailingZeros64(val: u64) -> core::ffi::c_uint {
    (val as core::ffi::c_ulonglong).trailing_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_countLeadingZeros64(val: u64) -> core::ffi::c_uint {
    (val as core::ffi::c_ulonglong).leading_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_NbCommonBytes(val: size_t) -> core::ffi::c_uint {
    if MEM_isLittleEndian() != 0 {
        if MEM_64bits() != 0 {
            ZSTD_countTrailingZeros64(val as u64) >> 3
        } else {
            ZSTD_countTrailingZeros32(val as u32) >> 3
        }
    } else if MEM_64bits() != 0 {
        ZSTD_countLeadingZeros64(val as u64) >> 3
    } else {
        ZSTD_countLeadingZeros32(val as u32) >> 3
    }
}
unsafe fn ZSTD_fillDoubleHashTableForCDict(
    ms: *mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLarge = (*ms).hashTable;
    let hBitsL = ((*cParams).hashLog).wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as core::ffi::c_uint);
    let mls = (*cParams).minMatch;
    let hashSmall = (*ms).chainTable;
    let hBitsS = ((*cParams).chainLog).wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as core::ffi::c_uint);
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3;
    while ip.offset(fastHashFillStep as isize).offset(-(1)) <= iend {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let mut i: u32 = 0;
        i = 0;
        while i < fastHashFillStep {
            let smHashAndTag = ZSTD_hashPtr(
                ip.offset(i as isize) as *const core::ffi::c_void,
                hBitsS,
                mls,
            );
            let lgHashAndTag =
                ZSTD_hashPtr(ip.offset(i as isize) as *const core::ffi::c_void, hBitsL, 8);
            if i == 0 {
                ZSTD_writeTaggedIndex(hashSmall, smHashAndTag, curr.wrapping_add(i));
            }
            if i == 0 || *hashLarge.add(lgHashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS) == 0 {
                ZSTD_writeTaggedIndex(hashLarge, lgHashAndTag, curr.wrapping_add(i));
            }
            if dtlm as core::ffi::c_uint == ZSTD_dtlm_fast as core::ffi::c_int as core::ffi::c_uint
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
unsafe fn ZSTD_fillDoubleHashTableForCCtx(
    ms: *mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLarge = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let mls = (*cParams).minMatch;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3;
    while ip.offset(fastHashFillStep as isize).offset(-(1)) <= iend {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let mut i: u32 = 0;
        i = 0;
        while i < fastHashFillStep {
            let smHash = ZSTD_hashPtr(
                ip.offset(i as isize) as *const core::ffi::c_void,
                hBitsS,
                mls,
            );
            let lgHash = ZSTD_hashPtr(ip.offset(i as isize) as *const core::ffi::c_void, hBitsL, 8);
            if i == 0 {
                *hashSmall.add(smHash) = curr.wrapping_add(i);
            }
            if i == 0 || *hashLarge.add(lgHash) == 0 {
                *hashLarge.add(lgHash) = curr.wrapping_add(i);
            }
            if dtlm as core::ffi::c_uint == ZSTD_dtlm_fast as core::ffi::c_int as core::ffi::c_uint
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
pub unsafe fn ZSTD_fillDoubleHashTable(
    ms: *mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    dtlm: ZSTD_dictTableLoadMethod_e,
    tfp: ZSTD_tableFillPurpose_e,
) {
    if tfp as core::ffi::c_uint == ZSTD_tfp_forCDict as core::ffi::c_int as core::ffi::c_uint {
        ZSTD_fillDoubleHashTableForCDict(ms, end, dtlm);
    } else {
        ZSTD_fillDoubleHashTableForCCtx(ms, end, dtlm);
    };
}
#[inline(always)]
unsafe fn ZSTD_compressBlock_doubleFast_noDict_generic(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mls: u32,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLong = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let base = (*ms).window.base;
    let istart = src as *const u8;
    let mut anchor = istart;
    let endIndex = (istart.offset_from(base) as size_t).wrapping_add(srcSize) as u32;
    let prefixLowestIndex = ZSTD_getLowestPrefixIndex(ms, endIndex, (*cParams).windowLog);
    let prefixLowest = base.offset(prefixLowestIndex as isize);
    let iend = istart.add(srcSize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut offset_1 = *rep.offset(0);
    let mut offset_2 = *rep.offset(1);
    let mut offsetSaved1 = 0;
    let mut offsetSaved2 = 0;
    let mut mLength: size_t = 0;
    let mut offset: u32 = 0;
    let mut curr: u32 = 0;
    let kStepIncr = ((1) << kSearchStrength) as size_t;
    let mut nextStep = core::ptr::null::<u8>();
    let mut step: size_t = 0;
    let mut hl0: size_t = 0;
    let mut hl1: size_t = 0;
    let mut idxl0: u32 = 0;
    let mut idxl1: u32 = 0;
    let mut matchl0 = core::ptr::null::<u8>();
    let mut matchs0 = core::ptr::null::<u8>();
    let mut matchl1 = core::ptr::null::<u8>();
    let mut matchs0_safe = core::ptr::null::<u8>();
    let mut ip = istart;
    let mut ip1 = core::ptr::null::<u8>();
    let dummy: [u8; 10] = [
        0x12 as core::ffi::c_int as u8,
        0x34 as core::ffi::c_int as u8,
        0x56 as core::ffi::c_int as u8,
        0x78 as core::ffi::c_int as u8,
        0x9a as core::ffi::c_int as u8,
        0xbc as core::ffi::c_int as u8,
        0xde as core::ffi::c_int as u8,
        0xf0 as core::ffi::c_int as u8,
        0xe2 as core::ffi::c_int as u8,
        0xb4 as core::ffi::c_int as u8,
    ];
    ip = ip.offset(
        (ip.offset_from(prefixLowest) as core::ffi::c_long == 0) as core::ffi::c_int as isize,
    );
    let current = ip.offset_from(base) as core::ffi::c_long as u32;
    let windowLow = ZSTD_getLowestPrefixIndex(ms, current, (*cParams).windowLog);
    let maxRep = current.wrapping_sub(windowLow);
    if offset_2 > maxRep {
        offsetSaved2 = offset_2;
        offset_2 = 0;
    }
    if offset_1 > maxRep {
        offsetSaved1 = offset_1;
        offset_1 = 0;
    }
    loop {
        's_428: {
            let mut current_block_83: u64;
            step = 1;
            nextStep = ip.add(kStepIncr);
            ip1 = ip.add(step);
            if ip1 <= ilimit {
                hl0 = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsL, 8);
                idxl0 = *hashLong.add(hl0);
                matchl0 = base.offset(idxl0 as isize);
                loop {
                    let hs0 = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsS, mls);
                    let idxs0 = *hashSmall.add(hs0);
                    curr = ip.offset_from(base) as core::ffi::c_long as u32;
                    matchs0 = base.offset(idxs0 as isize);
                    let fresh2 = &mut (*hashSmall.add(hs0));
                    *fresh2 = curr;
                    *hashLong.add(hl0) = *fresh2;
                    if (offset_1 > 0) as core::ffi::c_int
                        & (MEM_read32(
                            ip.offset(1).offset(-(offset_1 as isize)) as *const core::ffi::c_void
                        ) == MEM_read32(ip.offset(1) as *const core::ffi::c_void))
                            as core::ffi::c_int
                        != 0
                    {
                        mLength = (ZSTD_count(
                            ip.offset(1).offset(4),
                            ip.offset(1).offset(4).offset(-(offset_1 as isize)),
                            iend,
                        ))
                        .wrapping_add(4);
                        ip = ip.offset(1);
                        ZSTD_storeSeq(
                            seqStore,
                            ip.offset_from(anchor) as size_t,
                            anchor,
                            iend,
                            REPCODE1_TO_OFFBASE as u32,
                            mLength,
                        );
                        current_block_83 = 18341544293284149774;
                        break;
                    } else {
                        hl1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, hBitsL, 8);
                        let matchl0_safe = ZSTD_selectAddr(
                            idxl0,
                            prefixLowestIndex,
                            matchl0,
                            &*dummy.as_ptr().offset(0),
                        );
                        if MEM_read64(matchl0_safe as *const core::ffi::c_void)
                            == MEM_read64(ip as *const core::ffi::c_void)
                            && matchl0_safe == matchl0
                        {
                            mLength =
                                (ZSTD_count(ip.offset(8), matchl0.offset(8), iend)).wrapping_add(8);
                            offset = ip.offset_from(matchl0) as core::ffi::c_long as u32;
                            while (ip > anchor) as core::ffi::c_int
                                & (matchl0 > prefixLowest) as core::ffi::c_int
                                != 0
                                && *ip.offset(-1_isize) as core::ffi::c_int
                                    == *matchl0.offset(-1_isize) as core::ffi::c_int
                            {
                                ip = ip.offset(-1);
                                matchl0 = matchl0.offset(-1);
                                mLength = mLength.wrapping_add(1);
                            }
                            current_block_83 = 14716613436827065636;
                            break;
                        } else {
                            idxl1 = *hashLong.add(hl1);
                            matchl1 = base.offset(idxl1 as isize);
                            matchs0_safe = ZSTD_selectAddr(
                                idxs0,
                                prefixLowestIndex,
                                matchs0,
                                &*dummy.as_ptr().offset(0),
                            );
                            if MEM_read32(matchs0_safe as *const core::ffi::c_void)
                                == MEM_read32(ip as *const core::ffi::c_void)
                                && matchs0_safe == matchs0
                            {
                                current_block_83 = 6142208486753608565;
                                break;
                            }
                            if ip1 >= nextStep {
                                step = step.wrapping_add(1);
                                nextStep = nextStep.add(kStepIncr);
                            }
                            ip = ip1;
                            ip1 = ip1.add(step);
                            hl0 = hl1;
                            idxl0 = idxl1;
                            matchl0 = matchl1;
                            if ip1 > ilimit {
                                current_block_83 = 14575735148454673654;
                                break;
                            }
                        }
                    }
                }
                match current_block_83 {
                    14575735148454673654 => {}
                    _ => {
                        if current_block_83 == 6142208486753608565 {
                            mLength =
                                (ZSTD_count(ip.offset(4), matchs0.offset(4), iend)).wrapping_add(4);
                            offset = ip.offset_from(matchs0) as core::ffi::c_long as u32;
                            if idxl1 > prefixLowestIndex
                                && MEM_read64(matchl1 as *const core::ffi::c_void)
                                    == MEM_read64(ip1 as *const core::ffi::c_void)
                            {
                                let l1len = (ZSTD_count(ip1.offset(8), matchl1.offset(8), iend))
                                    .wrapping_add(8);
                                if l1len > mLength {
                                    ip = ip1;
                                    mLength = l1len;
                                    offset = ip.offset_from(matchl1) as core::ffi::c_long as u32;
                                    matchs0 = matchl1;
                                }
                            }
                            while (ip > anchor) as core::ffi::c_int
                                & (matchs0 > prefixLowest) as core::ffi::c_int
                                != 0
                                && *ip.offset(-1_isize) as core::ffi::c_int
                                    == *matchs0.offset(-1_isize) as core::ffi::c_int
                            {
                                ip = ip.offset(-1);
                                matchs0 = matchs0.offset(-1);
                                mLength = mLength.wrapping_add(1);
                            }
                            current_block_83 = 14716613436827065636;
                        }
                        if current_block_83 == 14716613436827065636 {
                            offset_2 = offset_1;
                            offset_1 = offset;
                            if step < 4 {
                                *hashLong.add(hl1) =
                                    ip1.offset_from(base) as core::ffi::c_long as u32;
                            }
                            ZSTD_storeSeq(
                                seqStore,
                                ip.offset_from(anchor) as size_t,
                                anchor,
                                iend,
                                offset.wrapping_add(ZSTD_REP_NUM as u32),
                                mLength,
                            );
                        }
                        ip = ip.add(mLength);
                        anchor = ip;
                        if ip <= ilimit {
                            let indexToInsert = curr.wrapping_add(2);
                            *hashLong.add(ZSTD_hashPtr(
                                base.offset(indexToInsert as isize) as *const core::ffi::c_void,
                                hBitsL,
                                8,
                            )) = indexToInsert;
                            *hashLong.add(ZSTD_hashPtr(
                                ip.offset(-(2)) as *const core::ffi::c_void,
                                hBitsL,
                                8,
                            )) = ip.offset(-(2)).offset_from(base) as core::ffi::c_long as u32;
                            *hashSmall.add(ZSTD_hashPtr(
                                base.offset(indexToInsert as isize) as *const core::ffi::c_void,
                                hBitsS,
                                mls,
                            )) = indexToInsert;
                            *hashSmall.add(ZSTD_hashPtr(
                                ip.offset(-(1)) as *const core::ffi::c_void,
                                hBitsS,
                                mls,
                            )) = ip.offset(-(1)).offset_from(base) as core::ffi::c_long as u32;
                            while ip <= ilimit
                                && (offset_2 > 0) as core::ffi::c_int
                                    & (MEM_read32(ip as *const core::ffi::c_void)
                                        == MEM_read32(ip.offset(-(offset_2 as isize))
                                            as *const core::ffi::c_void))
                                        as core::ffi::c_int
                                    != 0
                            {
                                let rLength = (ZSTD_count(
                                    ip.offset(4),
                                    ip.offset(4).offset(-(offset_2 as isize)),
                                    iend,
                                ))
                                .wrapping_add(4);
                                core::mem::swap(&mut offset_2, &mut offset_1);
                                *hashSmall.add(ZSTD_hashPtr(
                                    ip as *const core::ffi::c_void,
                                    hBitsS,
                                    mls,
                                )) = ip.offset_from(base) as core::ffi::c_long as u32;
                                *hashLong.add(ZSTD_hashPtr(
                                    ip as *const core::ffi::c_void,
                                    hBitsL,
                                    8,
                                )) = ip.offset_from(base) as core::ffi::c_long as u32;
                                ZSTD_storeSeq(
                                    seqStore,
                                    0,
                                    anchor,
                                    iend,
                                    REPCODE1_TO_OFFBASE as u32,
                                    rLength,
                                );
                                ip = ip.add(rLength);
                                anchor = ip;
                            }
                        }
                        break 's_428;
                    }
                }
            }
            offsetSaved2 = if offsetSaved1 != 0 && offset_1 != 0 {
                offsetSaved1
            } else {
                offsetSaved2
            };
            *rep.offset(0) = if offset_1 != 0 {
                offset_1
            } else {
                offsetSaved1
            };
            *rep.offset(1) = if offset_2 != 0 {
                offset_2
            } else {
                offsetSaved2
            };
            return iend.offset_from(anchor) as size_t;
        }
    }
}
#[inline(always)]
unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_generic(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mls: u32,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLong = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let base = (*ms).window.base;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let endIndex = (istart.offset_from(base) as size_t).wrapping_add(srcSize) as u32;
    let prefixLowestIndex = ZSTD_getLowestPrefixIndex(ms, endIndex, (*cParams).windowLog);
    let prefixLowest = base.offset(prefixLowestIndex as isize);
    let iend = istart.add(srcSize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut offset_1 = *rep.offset(0);
    let mut offset_2 = *rep.offset(1);
    let dms = (*ms).dictMatchState;
    let dictCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashLong: *const u32 = (*dms).hashTable;
    let dictHashSmall: *const u32 = (*dms).chainTable;
    let dictStartIndex = (*dms).window.dictLimit;
    let dictBase = (*dms).window.base;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictEnd = (*dms).window.nextSrc;
    let dictIndexDelta =
        prefixLowestIndex.wrapping_sub(dictEnd.offset_from(dictBase) as core::ffi::c_long as u32);
    let dictHBitsL =
        ((*dictCParams).hashLog).wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as core::ffi::c_uint);
    let dictHBitsS =
        ((*dictCParams).chainLog).wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as core::ffi::c_uint);
    let dictAndPrefixLength = (ip.offset_from(prefixLowest) as core::ffi::c_long
        + dictEnd.offset_from(dictStart) as core::ffi::c_long) as u32;
    if (*ms).prefetchCDictTables != 0 {
        let hashTableBytes =
            ((1 as size_t) << (*dictCParams).hashLog).wrapping_mul(::core::mem::size_of::<u32>());
        let chainTableBytes =
            ((1 as size_t) << (*dictCParams).chainLog).wrapping_mul(::core::mem::size_of::<u32>());
        let _ptr = dictHashLong as *const core::ffi::c_char;
        let _size = hashTableBytes;
        let mut _pos: size_t = 0;
        _pos = 0;
        while _pos < _size {
            _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
        }
        let _ptr_0 = dictHashSmall as *const core::ffi::c_char;
        let _size_0 = chainTableBytes;
        let mut _pos_0: size_t = 0;
        _pos_0 = 0;
        while _pos_0 < _size_0 {
            _pos_0 = _pos_0.wrapping_add(CACHELINE_SIZE as size_t);
        }
    }
    ip = ip.offset((dictAndPrefixLength == 0) as core::ffi::c_int as isize);
    while ip < ilimit {
        let mut mLength: size_t = 0;
        let mut offset: u32 = 0;
        let h2 = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsL, 8);
        let h = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsS, mls);
        let dictHashAndTagL = ZSTD_hashPtr(ip as *const core::ffi::c_void, dictHBitsL, 8);
        let dictHashAndTagS = ZSTD_hashPtr(ip as *const core::ffi::c_void, dictHBitsS, mls);
        let dictMatchIndexAndTagL = *dictHashLong.add(dictHashAndTagL >> ZSTD_SHORT_CACHE_TAG_BITS);
        let dictMatchIndexAndTagS =
            *dictHashSmall.add(dictHashAndTagS >> ZSTD_SHORT_CACHE_TAG_BITS);
        let dictTagsMatchL =
            ZSTD_comparePackedTags(dictMatchIndexAndTagL as size_t, dictHashAndTagL);
        let dictTagsMatchS =
            ZSTD_comparePackedTags(dictMatchIndexAndTagS as size_t, dictHashAndTagS);
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let matchIndexL = *hashLong.add(h2);
        let mut matchIndexS = *hashSmall.add(h);
        let mut matchLong = base.offset(matchIndexL as isize);
        let mut match_0 = base.offset(matchIndexS as isize);
        let repIndex = curr.wrapping_add(1).wrapping_sub(offset_1);
        let repMatch = if repIndex < prefixLowestIndex {
            dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
        } else {
            base.offset(repIndex as isize)
        };
        let fresh3 = &mut (*hashSmall.add(h));
        *fresh3 = curr;
        *hashLong.add(h2) = *fresh3;
        if ZSTD_index_overlap_check(prefixLowestIndex, repIndex) != 0
            && MEM_read32(repMatch as *const core::ffi::c_void)
                == MEM_read32(ip.offset(1) as *const core::ffi::c_void)
        {
            let repMatchEnd = if repIndex < prefixLowestIndex {
                dictEnd
            } else {
                iend
            };
            mLength = (ZSTD_count_2segments(
                ip.offset(1).offset(4),
                repMatch.offset(4),
                iend,
                repMatchEnd,
                prefixLowest,
            ))
            .wrapping_add(4);
            ip = ip.offset(1);
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as size_t,
                anchor,
                iend,
                REPCODE1_TO_OFFBASE as u32,
                mLength,
            );
        } else {
            if matchIndexL >= prefixLowestIndex
                && MEM_read64(matchLong as *const core::ffi::c_void)
                    == MEM_read64(ip as *const core::ffi::c_void)
            {
                mLength = (ZSTD_count(ip.offset(8), matchLong.offset(8), iend)).wrapping_add(8);
                offset = ip.offset_from(matchLong) as core::ffi::c_long as u32;
                while (ip > anchor) as core::ffi::c_int
                    & (matchLong > prefixLowest) as core::ffi::c_int
                    != 0
                    && *ip.offset(-1_isize) as core::ffi::c_int
                        == *matchLong.offset(-1_isize) as core::ffi::c_int
                {
                    ip = ip.offset(-1);
                    matchLong = matchLong.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            } else {
                if dictTagsMatchL != 0 {
                    let dictMatchIndexL = dictMatchIndexAndTagL >> ZSTD_SHORT_CACHE_TAG_BITS;
                    let mut dictMatchL = dictBase.offset(dictMatchIndexL as isize);
                    if dictMatchL > dictStart
                        && MEM_read64(dictMatchL as *const core::ffi::c_void)
                            == MEM_read64(ip as *const core::ffi::c_void)
                    {
                        mLength = (ZSTD_count_2segments(
                            ip.offset(8),
                            dictMatchL.offset(8),
                            iend,
                            dictEnd,
                            prefixLowest,
                        ))
                        .wrapping_add(8);
                        offset = curr
                            .wrapping_sub(dictMatchIndexL)
                            .wrapping_sub(dictIndexDelta);
                        while (ip > anchor) as core::ffi::c_int
                            & (dictMatchL > dictStart) as core::ffi::c_int
                            != 0
                            && *ip.offset(-1_isize) as core::ffi::c_int
                                == *dictMatchL.offset(-1_isize) as core::ffi::c_int
                        {
                            ip = ip.offset(-1);
                            dictMatchL = dictMatchL.offset(-1);
                            mLength = mLength.wrapping_add(1);
                        }
                        current_block = 17830677668754335218;
                    } else {
                        current_block = 6721012065216013753;
                    }
                } else {
                    current_block = 6721012065216013753;
                }
                match current_block {
                    17830677668754335218 => {}
                    _ => {
                        if matchIndexS > prefixLowestIndex {
                            if MEM_read32(match_0 as *const core::ffi::c_void)
                                == MEM_read32(ip as *const core::ffi::c_void)
                            {
                                current_block = 2631791190359682872;
                            } else {
                                current_block = 5372832139739605200;
                            }
                        } else if dictTagsMatchS != 0 {
                            let dictMatchIndexS =
                                dictMatchIndexAndTagS >> ZSTD_SHORT_CACHE_TAG_BITS;
                            match_0 = dictBase.offset(dictMatchIndexS as isize);
                            matchIndexS = dictMatchIndexS.wrapping_add(dictIndexDelta);
                            if match_0 > dictStart
                                && MEM_read32(match_0 as *const core::ffi::c_void)
                                    == MEM_read32(ip as *const core::ffi::c_void)
                            {
                                current_block = 2631791190359682872;
                            } else {
                                current_block = 5372832139739605200;
                            }
                        } else {
                            current_block = 5372832139739605200;
                        }
                        match current_block {
                            5372832139739605200 => {
                                ip = ip.offset(
                                    ((ip.offset_from(anchor) as core::ffi::c_long
                                        >> kSearchStrength)
                                        + 1) as isize,
                                );
                                continue;
                            }
                            _ => {
                                let hl3 = ZSTD_hashPtr(
                                    ip.offset(1) as *const core::ffi::c_void,
                                    hBitsL,
                                    8,
                                );
                                let dictHashAndTagL3 = ZSTD_hashPtr(
                                    ip.offset(1) as *const core::ffi::c_void,
                                    dictHBitsL,
                                    8,
                                );
                                let matchIndexL3 = *hashLong.add(hl3);
                                let dictMatchIndexAndTagL3 = *dictHashLong
                                    .add(dictHashAndTagL3 >> ZSTD_SHORT_CACHE_TAG_BITS);
                                let dictTagsMatchL3 = ZSTD_comparePackedTags(
                                    dictMatchIndexAndTagL3 as size_t,
                                    dictHashAndTagL3,
                                );
                                let mut matchL3 = base.offset(matchIndexL3 as isize);
                                *hashLong.add(hl3) = curr.wrapping_add(1);
                                if matchIndexL3 >= prefixLowestIndex
                                    && MEM_read64(matchL3 as *const core::ffi::c_void)
                                        == MEM_read64(ip.offset(1) as *const core::ffi::c_void)
                                {
                                    mLength = (ZSTD_count(ip.offset(9), matchL3.offset(8), iend))
                                        .wrapping_add(8);
                                    ip = ip.offset(1);
                                    offset = ip.offset_from(matchL3) as core::ffi::c_long as u32;
                                    while (ip > anchor) as core::ffi::c_int
                                        & (matchL3 > prefixLowest) as core::ffi::c_int
                                        != 0
                                        && *ip.offset(-1_isize) as core::ffi::c_int
                                            == *matchL3.offset(-1_isize) as core::ffi::c_int
                                    {
                                        ip = ip.offset(-1);
                                        matchL3 = matchL3.offset(-1);
                                        mLength = mLength.wrapping_add(1);
                                    }
                                } else {
                                    if dictTagsMatchL3 != 0 {
                                        let dictMatchIndexL3 =
                                            dictMatchIndexAndTagL3 >> ZSTD_SHORT_CACHE_TAG_BITS;
                                        let mut dictMatchL3 =
                                            dictBase.offset(dictMatchIndexL3 as isize);
                                        if dictMatchL3 > dictStart
                                            && MEM_read64(dictMatchL3 as *const core::ffi::c_void)
                                                == MEM_read64(
                                                    ip.offset(1) as *const core::ffi::c_void
                                                )
                                        {
                                            mLength = (ZSTD_count_2segments(
                                                ip.offset(1).offset(8),
                                                dictMatchL3.offset(8),
                                                iend,
                                                dictEnd,
                                                prefixLowest,
                                            ))
                                            .wrapping_add(8);
                                            ip = ip.offset(1);
                                            offset = curr
                                                .wrapping_add(1)
                                                .wrapping_sub(dictMatchIndexL3)
                                                .wrapping_sub(dictIndexDelta);
                                            while (ip > anchor) as core::ffi::c_int
                                                & (dictMatchL3 > dictStart) as core::ffi::c_int
                                                != 0
                                                && *ip.offset(-1_isize) as core::ffi::c_int
                                                    == *dictMatchL3.offset(-1_isize)
                                                        as core::ffi::c_int
                                            {
                                                ip = ip.offset(-1);
                                                dictMatchL3 = dictMatchL3.offset(-1);
                                                mLength = mLength.wrapping_add(1);
                                            }
                                            current_block = 17830677668754335218;
                                        } else {
                                            current_block = 1209030638129645089;
                                        }
                                    } else {
                                        current_block = 1209030638129645089;
                                    }
                                    match current_block {
                                        17830677668754335218 => {}
                                        _ => {
                                            if matchIndexS < prefixLowestIndex {
                                                mLength = (ZSTD_count_2segments(
                                                    ip.offset(4),
                                                    match_0.offset(4),
                                                    iend,
                                                    dictEnd,
                                                    prefixLowest,
                                                ))
                                                .wrapping_add(4);
                                                offset = curr.wrapping_sub(matchIndexS);
                                                while (ip > anchor) as core::ffi::c_int
                                                    & (match_0 > dictStart) as core::ffi::c_int
                                                    != 0
                                                    && *ip.offset(-1_isize) as core::ffi::c_int
                                                        == *match_0.offset(-1_isize)
                                                            as core::ffi::c_int
                                                {
                                                    ip = ip.offset(-1);
                                                    match_0 = match_0.offset(-1);
                                                    mLength = mLength.wrapping_add(1);
                                                }
                                            } else {
                                                mLength = (ZSTD_count(
                                                    ip.offset(4),
                                                    match_0.offset(4),
                                                    iend,
                                                ))
                                                .wrapping_add(4);
                                                offset = ip.offset_from(match_0)
                                                    as core::ffi::c_long
                                                    as u32;
                                                while (ip > anchor) as core::ffi::c_int
                                                    & (match_0 > prefixLowest) as core::ffi::c_int
                                                    != 0
                                                    && *ip.offset(-1_isize) as core::ffi::c_int
                                                        == *match_0.offset(-1_isize)
                                                            as core::ffi::c_int
                                                {
                                                    ip = ip.offset(-1);
                                                    match_0 = match_0.offset(-1);
                                                    mLength = mLength.wrapping_add(1);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            offset_2 = offset_1;
            offset_1 = offset;
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as size_t,
                anchor,
                iend,
                offset.wrapping_add(ZSTD_REP_NUM as u32),
                mLength,
            );
        }
        ip = ip.add(mLength);
        anchor = ip;
        if ip <= ilimit {
            let indexToInsert = curr.wrapping_add(2);
            *hashLong.add(ZSTD_hashPtr(
                base.offset(indexToInsert as isize) as *const core::ffi::c_void,
                hBitsL,
                8,
            )) = indexToInsert;
            *hashLong.add(ZSTD_hashPtr(
                ip.offset(-(2)) as *const core::ffi::c_void,
                hBitsL,
                8,
            )) = ip.offset(-(2)).offset_from(base) as core::ffi::c_long as u32;
            *hashSmall.add(ZSTD_hashPtr(
                base.offset(indexToInsert as isize) as *const core::ffi::c_void,
                hBitsS,
                mls,
            )) = indexToInsert;
            *hashSmall.add(ZSTD_hashPtr(
                ip.offset(-(1)) as *const core::ffi::c_void,
                hBitsS,
                mls,
            )) = ip.offset(-(1)).offset_from(base) as core::ffi::c_long as u32;
            while ip <= ilimit {
                let current2 = ip.offset_from(base) as core::ffi::c_long as u32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixLowestIndex {
                    dictBase
                        .offset(repIndex2 as isize)
                        .offset(-(dictIndexDelta as isize))
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixLowestIndex, repIndex2) != 0
                    && MEM_read32(repMatch2 as *const core::ffi::c_void)
                        == MEM_read32(ip as *const core::ffi::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixLowestIndex {
                    dictEnd
                } else {
                    iend
                };
                let repLength2 = (ZSTD_count_2segments(
                    ip.offset(4),
                    repMatch2.offset(4),
                    iend,
                    repEnd2,
                    prefixLowest,
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
                *hashSmall.add(ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsS, mls)) =
                    current2;
                *hashLong.add(ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsL, 8)) = current2;
                ip = ip.add(repLength2);
                anchor = ip;
            }
        }
    }
    *rep.offset(0) = offset_1;
    *rep.offset(1) = offset_2;
    iend.offset_from(anchor) as size_t
}
unsafe fn ZSTD_compressBlock_doubleFast_noDict_4(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_noDict_generic(ms, seqStore, rep, src, srcSize, 4)
}
unsafe fn ZSTD_compressBlock_doubleFast_noDict_5(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_noDict_generic(ms, seqStore, rep, src, srcSize, 5)
}
unsafe fn ZSTD_compressBlock_doubleFast_noDict_6(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_noDict_generic(ms, seqStore, rep, src, srcSize, 6)
}
unsafe fn ZSTD_compressBlock_doubleFast_noDict_7(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_noDict_generic(ms, seqStore, rep, src, srcSize, 7)
}
unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_4(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 4)
}
unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_5(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 5)
}
unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_6(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 6)
}
unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState_7(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 7)
}
pub unsafe fn ZSTD_compressBlock_doubleFast(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => ZSTD_compressBlock_doubleFast_noDict_5(ms, seqStore, rep, src, srcSize),
        6 => ZSTD_compressBlock_doubleFast_noDict_6(ms, seqStore, rep, src, srcSize),
        7 => ZSTD_compressBlock_doubleFast_noDict_7(ms, seqStore, rep, src, srcSize),
        4 | _ => ZSTD_compressBlock_doubleFast_noDict_4(ms, seqStore, rep, src, srcSize),
    }
}
pub unsafe fn ZSTD_compressBlock_doubleFast_dictMatchState(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => ZSTD_compressBlock_doubleFast_dictMatchState_5(ms, seqStore, rep, src, srcSize),
        6 => ZSTD_compressBlock_doubleFast_dictMatchState_6(ms, seqStore, rep, src, srcSize),
        7 => ZSTD_compressBlock_doubleFast_dictMatchState_7(ms, seqStore, rep, src, srcSize),
        4 | _ => ZSTD_compressBlock_doubleFast_dictMatchState_4(ms, seqStore, rep, src, srcSize),
    }
}
unsafe fn ZSTD_compressBlock_doubleFast_extDict_generic(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mls: u32,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashLong = (*ms).hashTable;
    let hBitsL = (*cParams).hashLog;
    let hashSmall = (*ms).chainTable;
    let hBitsS = (*cParams).chainLog;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.add(srcSize);
    let ilimit = iend.offset(-(8));
    let base = (*ms).window.base;
    let endIndex = (istart.offset_from(base) as size_t).wrapping_add(srcSize) as u32;
    let lowLimit = ZSTD_getLowestMatchIndex(ms, endIndex, (*cParams).windowLog);
    let dictStartIndex = lowLimit;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStartIndex = if dictLimit > lowLimit {
        dictLimit
    } else {
        lowLimit
    };
    let prefixStart = base.offset(prefixStartIndex as isize);
    let dictBase = (*ms).window.dictBase;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictEnd = dictBase.offset(prefixStartIndex as isize);
    let mut offset_1 = *rep.offset(0);
    let mut offset_2 = *rep.offset(1);
    if prefixStartIndex == dictStartIndex {
        return ZSTD_compressBlock_doubleFast(ms, seqStore, rep, src, srcSize);
    }
    while ip < ilimit {
        let hSmall = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsS, mls);
        let matchIndex = *hashSmall.add(hSmall);
        let matchBase = if matchIndex < prefixStartIndex {
            dictBase
        } else {
            base
        };
        let mut match_0 = matchBase.offset(matchIndex as isize);
        let hLong = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsL, 8);
        let matchLongIndex = *hashLong.add(hLong);
        let matchLongBase = if matchLongIndex < prefixStartIndex {
            dictBase
        } else {
            base
        };
        let mut matchLong = matchLongBase.offset(matchLongIndex as isize);
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let repIndex = curr.wrapping_add(1).wrapping_sub(offset_1);
        let repBase = if repIndex < prefixStartIndex {
            dictBase
        } else {
            base
        };
        let repMatch = repBase.offset(repIndex as isize);
        let mut mLength: size_t = 0;
        let fresh4 = &mut (*hashLong.add(hLong));
        *fresh4 = curr;
        *hashSmall.add(hSmall) = *fresh4;
        if ZSTD_index_overlap_check(prefixStartIndex, repIndex)
            & (offset_1 <= curr.wrapping_add(1).wrapping_sub(dictStartIndex)) as core::ffi::c_int
            != 0
            && MEM_read32(repMatch as *const core::ffi::c_void)
                == MEM_read32(ip.offset(1) as *const core::ffi::c_void)
        {
            let repMatchEnd = if repIndex < prefixStartIndex {
                dictEnd
            } else {
                iend
            };
            mLength = (ZSTD_count_2segments(
                ip.offset(1).offset(4),
                repMatch.offset(4),
                iend,
                repMatchEnd,
                prefixStart,
            ))
            .wrapping_add(4);
            ip = ip.offset(1);
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as size_t,
                anchor,
                iend,
                REPCODE1_TO_OFFBASE as u32,
                mLength,
            );
        } else if matchLongIndex > dictStartIndex
            && MEM_read64(matchLong as *const core::ffi::c_void)
                == MEM_read64(ip as *const core::ffi::c_void)
        {
            let matchEnd = if matchLongIndex < prefixStartIndex {
                dictEnd
            } else {
                iend
            };
            let lowMatchPtr = if matchLongIndex < prefixStartIndex {
                dictStart
            } else {
                prefixStart
            };
            let mut offset: u32 = 0;
            mLength = (ZSTD_count_2segments(
                ip.offset(8),
                matchLong.offset(8),
                iend,
                matchEnd,
                prefixStart,
            ))
            .wrapping_add(8);
            offset = curr.wrapping_sub(matchLongIndex);
            while (ip > anchor) as core::ffi::c_int & (matchLong > lowMatchPtr) as core::ffi::c_int
                != 0
                && *ip.offset(-1_isize) as core::ffi::c_int
                    == *matchLong.offset(-1_isize) as core::ffi::c_int
            {
                ip = ip.offset(-1);
                matchLong = matchLong.offset(-1);
                mLength = mLength.wrapping_add(1);
            }
            offset_2 = offset_1;
            offset_1 = offset;
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as size_t,
                anchor,
                iend,
                offset.wrapping_add(ZSTD_REP_NUM as u32),
                mLength,
            );
        } else if matchIndex > dictStartIndex
            && MEM_read32(match_0 as *const core::ffi::c_void)
                == MEM_read32(ip as *const core::ffi::c_void)
        {
            let h3 = ZSTD_hashPtr(ip.offset(1) as *const core::ffi::c_void, hBitsL, 8);
            let matchIndex3 = *hashLong.add(h3);
            let match3Base = if matchIndex3 < prefixStartIndex {
                dictBase
            } else {
                base
            };
            let mut match3 = match3Base.offset(matchIndex3 as isize);
            let mut offset_0: u32 = 0;
            *hashLong.add(h3) = curr.wrapping_add(1);
            if matchIndex3 > dictStartIndex
                && MEM_read64(match3 as *const core::ffi::c_void)
                    == MEM_read64(ip.offset(1) as *const core::ffi::c_void)
            {
                let matchEnd_0 = if matchIndex3 < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                let lowMatchPtr_0 = if matchIndex3 < prefixStartIndex {
                    dictStart
                } else {
                    prefixStart
                };
                mLength = (ZSTD_count_2segments(
                    ip.offset(9),
                    match3.offset(8),
                    iend,
                    matchEnd_0,
                    prefixStart,
                ))
                .wrapping_add(8);
                ip = ip.offset(1);
                offset_0 = curr.wrapping_add(1).wrapping_sub(matchIndex3);
                while (ip > anchor) as core::ffi::c_int
                    & (match3 > lowMatchPtr_0) as core::ffi::c_int
                    != 0
                    && *ip.offset(-1_isize) as core::ffi::c_int
                        == *match3.offset(-1_isize) as core::ffi::c_int
                {
                    ip = ip.offset(-1);
                    match3 = match3.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            } else {
                let matchEnd_1 = if matchIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                let lowMatchPtr_1 = if matchIndex < prefixStartIndex {
                    dictStart
                } else {
                    prefixStart
                };
                mLength = (ZSTD_count_2segments(
                    ip.offset(4),
                    match_0.offset(4),
                    iend,
                    matchEnd_1,
                    prefixStart,
                ))
                .wrapping_add(4);
                offset_0 = curr.wrapping_sub(matchIndex);
                while (ip > anchor) as core::ffi::c_int
                    & (match_0 > lowMatchPtr_1) as core::ffi::c_int
                    != 0
                    && *ip.offset(-1_isize) as core::ffi::c_int
                        == *match_0.offset(-1_isize) as core::ffi::c_int
                {
                    ip = ip.offset(-1);
                    match_0 = match_0.offset(-1);
                    mLength = mLength.wrapping_add(1);
                }
            }
            offset_2 = offset_1;
            offset_1 = offset_0;
            ZSTD_storeSeq(
                seqStore,
                ip.offset_from(anchor) as size_t,
                anchor,
                iend,
                offset_0.wrapping_add(ZSTD_REP_NUM as u32),
                mLength,
            );
        } else {
            ip = ip.offset(
                ((ip.offset_from(anchor) as core::ffi::c_long >> kSearchStrength) + 1) as isize,
            );
            continue;
        }
        ip = ip.add(mLength);
        anchor = ip;
        if ip <= ilimit {
            let indexToInsert = curr.wrapping_add(2);
            *hashLong.add(ZSTD_hashPtr(
                base.offset(indexToInsert as isize) as *const core::ffi::c_void,
                hBitsL,
                8,
            )) = indexToInsert;
            *hashLong.add(ZSTD_hashPtr(
                ip.offset(-(2)) as *const core::ffi::c_void,
                hBitsL,
                8,
            )) = ip.offset(-(2)).offset_from(base) as core::ffi::c_long as u32;
            *hashSmall.add(ZSTD_hashPtr(
                base.offset(indexToInsert as isize) as *const core::ffi::c_void,
                hBitsS,
                mls,
            )) = indexToInsert;
            *hashSmall.add(ZSTD_hashPtr(
                ip.offset(-(1)) as *const core::ffi::c_void,
                hBitsS,
                mls,
            )) = ip.offset(-(1)).offset_from(base) as core::ffi::c_long as u32;
            while ip <= ilimit {
                let current2 = ip.offset_from(base) as core::ffi::c_long as u32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase.offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixStartIndex, repIndex2)
                    & (offset_2 <= current2.wrapping_sub(dictStartIndex)) as core::ffi::c_int
                    != 0
                    && MEM_read32(repMatch2 as *const core::ffi::c_void)
                        == MEM_read32(ip as *const core::ffi::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex2 < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                let repLength2 = (ZSTD_count_2segments(
                    ip.offset(4),
                    repMatch2.offset(4),
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
                *hashSmall.add(ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsS, mls)) =
                    current2;
                *hashLong.add(ZSTD_hashPtr(ip as *const core::ffi::c_void, hBitsL, 8)) = current2;
                ip = ip.add(repLength2);
                anchor = ip;
            }
        }
    }
    *rep.offset(0) = offset_1;
    *rep.offset(1) = offset_2;
    iend.offset_from(anchor) as size_t
}
unsafe fn ZSTD_compressBlock_doubleFast_extDict_4(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore, rep, src, srcSize, 4)
}
unsafe fn ZSTD_compressBlock_doubleFast_extDict_5(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore, rep, src, srcSize, 5)
}
unsafe fn ZSTD_compressBlock_doubleFast_extDict_6(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore, rep, src, srcSize, 6)
}
unsafe fn ZSTD_compressBlock_doubleFast_extDict_7(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_doubleFast_extDict_generic(ms, seqStore, rep, src, srcSize, 7)
}
pub unsafe fn ZSTD_compressBlock_doubleFast_extDict(
    ms: *mut ZSTD_MatchState_t,
    seqStore: *mut SeqStore_t,
    rep: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => ZSTD_compressBlock_doubleFast_extDict_5(ms, seqStore, rep, src, srcSize),
        6 => ZSTD_compressBlock_doubleFast_extDict_6(ms, seqStore, rep, src, srcSize),
        7 => ZSTD_compressBlock_doubleFast_extDict_7(ms, seqStore, rep, src, srcSize),
        4 | _ => ZSTD_compressBlock_doubleFast_extDict_4(ms, seqStore, rep, src, srcSize),
    }
}
