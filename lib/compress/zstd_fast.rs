use core::arch::asm;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
pub type ptrdiff_t = core::ffi::c_long;
pub type ZSTD_longLengthType_e = core::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
pub type FSE_repeat = core::ffi::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = core::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = size_t;
pub type ZSTD_OptPrice_e = core::ffi::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const u8,
    pub base: *const u8,
    pub dictBase: *const u8,
    pub dictLimit: u32,
    pub lowLimit: u32,
    pub nbOverflowCorrections: u32,
}
pub type ZSTD_overlap_e = core::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
pub type ZSTD_dictTableLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = core::ffi::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
pub type ZSTD_match4Found = Option<unsafe fn(*const u8, *const u8, u32, u32) -> core::ffi::c_int>;
pub const CACHELINE_SIZE: core::ffi::c_int = 64;

use libc::size_t;

use crate::lib::common::mem::{
    MEM_64bits, MEM_isLittleEndian, MEM_read16, MEM_read32, MEM_readLE32, MEM_readLE64, MEM_readST,
};
use crate::lib::compress::zstd_compress::{
    SeqStore_t, ZSTD_MatchState_t, ZSTD_match_t, ZSTD_optimal_t,
};
use crate::lib::zstd::*;
pub const kSearchStrength: core::ffi::c_int = 8;
pub const HASH_READ_SIZE: core::ffi::c_int = 8;
#[inline]
unsafe fn ZSTD_selectAddr(
    mut index: u32,
    mut lowLimit: u32,
    mut candidate: *const u8,
    mut backup: *const u8,
) -> *const u8 {
    asm!(
        "cmp {1}, {2}\ncmova {3}, {0}\n", inlateout(reg) candidate, inlateout(reg) index
        => _, inlateout(reg) lowLimit => _, inlateout(reg) backup => _,
        options(preserves_flags, pure, readonly, att_syntax)
    );
    candidate
}
unsafe fn ZSTD_safecopyLiterals(
    mut op: *mut u8,
    mut ip: *const u8,
    iend: *const u8,
    mut ilimit_w: *const u8,
) {
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            ip as *const core::ffi::c_void,
            ilimit_w.offset_from(ip) as core::ffi::c_long as size_t,
            ZSTD_no_overlap,
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
    mut seqStorePtr: *mut SeqStore_t,
    mut litLength: size_t,
    mut offBase: u32,
    mut matchLength: size_t,
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
    mut seqStorePtr: *mut SeqStore_t,
    mut litLength: size_t,
    mut literals: *const u8,
    mut litLimit: *const u8,
    mut offBase: u32,
    mut matchLength: size_t,
) {
    let litLimit_w = litLimit.offset(-(WILDCOPY_OVERLENGTH as isize));
    let litEnd = literals.offset(litLength as isize);
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
                ZSTD_no_overlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals((*seqStorePtr).lit, literals, litEnd, litLimit_w);
    }
    (*seqStorePtr).lit = ((*seqStorePtr).lit).offset(litLength as isize);
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
                return pIn.offset_from(pStart) as core::ffi::c_long as size_t;
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
    pIn.offset_from(pStart) as core::ffi::c_long as size_t
}
#[inline]
unsafe fn ZSTD_count_2segments(
    mut ip: *const u8,
    mut match_0: *const u8,
    mut iEnd: *const u8,
    mut mEnd: *const u8,
    mut iStart: *const u8,
) -> size_t {
    let vEnd = if ip.offset(mEnd.offset_from(match_0) as core::ffi::c_long as isize) < iEnd {
        ip.offset(mEnd.offset_from(match_0) as core::ffi::c_long as isize)
    } else {
        iEnd
    };
    let matchLength = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd {
        return matchLength;
    }
    matchLength.wrapping_add(ZSTD_count(ip.offset(matchLength as isize), iStart, iEnd))
}
static prime4bytes: u32 = 2654435761;
unsafe fn ZSTD_hash4(mut u: u32, mut h: u32, mut s: u32) -> u32 {
    ((u * prime4bytes) ^ s) >> 32u32.wrapping_sub(h)
}
unsafe fn ZSTD_hash4Ptr(mut ptr: *const core::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0) as size_t
}
static prime5bytes: u64 = 889523592379;
unsafe fn ZSTD_hash5(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    ((((u << (64 - 40)) * prime5bytes) ^ s) >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash5Ptr(mut p: *const core::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash5(MEM_readLE64(p), h, 0)
}
static prime6bytes: u64 = 227718039650203;
unsafe fn ZSTD_hash6(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    ((((u << (64 - 48)) * prime6bytes) ^ s) >> (64u32).wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash6Ptr(mut p: *const core::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash6(MEM_readLE64(p), h, 0)
}
static prime7bytes: u64 = 58295818150454627;
unsafe fn ZSTD_hash7(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    ((((u << (64 - 56)) * prime7bytes) ^ s) >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash7Ptr(mut p: *const core::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash7(MEM_readLE64(p), h, 0)
}
static prime8bytes: u64 = 0xcf1bbcdcb7a56463 as core::ffi::c_ulonglong as u64;
unsafe fn ZSTD_hash8(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    (((u * prime8bytes) ^ s) >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash8Ptr(mut p: *const core::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash8(MEM_readLE64(p), h, 0)
}
#[inline(always)]
unsafe fn ZSTD_hashPtr(mut p: *const core::ffi::c_void, mut hBits: u32, mut mls: u32) -> size_t {
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
    mut ms: *const ZSTD_MatchState_t,
    mut curr: u32,
    mut windowLog: core::ffi::c_uint,
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
    mut ms: *const ZSTD_MatchState_t,
    mut curr: u32,
    mut windowLog: core::ffi::c_uint,
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
unsafe fn ZSTD_writeTaggedIndex(hashTable: *mut u32, mut hashAndTag: size_t, mut index: u32) {
    let hash = hashAndTag >> ZSTD_SHORT_CACHE_TAG_BITS;
    let tag = (hashAndTag & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    *hashTable.offset(hash as isize) = index << ZSTD_SHORT_CACHE_TAG_BITS | tag;
}
#[inline]
unsafe fn ZSTD_comparePackedTags(
    mut packedTag1: size_t,
    mut packedTag2: size_t,
) -> core::ffi::c_int {
    let tag1 = (packedTag1 & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    let tag2 = (packedTag2 & ZSTD_SHORT_CACHE_TAG_MASK as size_t) as u32;
    (tag1 == tag2) as core::ffi::c_int
}
pub const ZSTD_REP_NUM: core::ffi::c_int = 3;
pub const MINMATCH: core::ffi::c_int = 3;
unsafe fn ZSTD_copy8(mut dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void) {
    libc::memcpy(dst, src, 8);
}
unsafe fn ZSTD_copy16(mut dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub const WILDCOPY_OVERLENGTH: core::ffi::c_int = 32;
pub const WILDCOPY_VECLEN: core::ffi::c_int = 16;
#[inline(always)]
unsafe fn ZSTD_wildcopy(
    mut dst: *mut core::ffi::c_void,
    mut src: *const core::ffi::c_void,
    mut length: size_t,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut u8).offset_from(src as *const u8) as core::ffi::c_long;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    if ovtype as core::ffi::c_uint
        == ZSTD_overlap_src_before_dst as core::ffi::c_int as core::ffi::c_uint
        && diff < WILDCOPY_VECLEN as ptrdiff_t
    {
        loop {
            ZSTD_copy8(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(8);
            ip = ip.offset(8);
            if op >= oend {
                break;
            }
        }
    } else {
        ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
        if 16 >= length {
            return;
        }
        op = op.offset(16);
        ip = ip.offset(16);
        loop {
            ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            if op >= oend {
                break;
            }
        }
    };
}
#[inline]
unsafe fn ZSTD_countTrailingZeros32(mut val: u32) -> core::ffi::c_uint {
    val.trailing_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_countLeadingZeros32(mut val: u32) -> core::ffi::c_uint {
    val.leading_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_countTrailingZeros64(mut val: u64) -> core::ffi::c_uint {
    (val as core::ffi::c_ulonglong).trailing_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_countLeadingZeros64(mut val: u64) -> core::ffi::c_uint {
    (val as core::ffi::c_ulonglong).leading_zeros() as i32 as core::ffi::c_uint
}
#[inline]
unsafe fn ZSTD_NbCommonBytes(mut val: size_t) -> core::ffi::c_uint {
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
unsafe fn ZSTD_fillHashTableForCDict(
    mut ms: *mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hBits = ((*cParams).hashLog).wrapping_add(ZSTD_SHORT_CACHE_TAG_BITS as core::ffi::c_uint);
    let mls = (*cParams).minMatch;
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3;
    while ip.offset(fastHashFillStep as isize) < iend.offset(2) {
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
                if *hashTable.offset((hashAndTag_0 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize) == 0 {
                    ZSTD_writeTaggedIndex(hashTable, hashAndTag_0, curr.wrapping_add(p));
                }
                p = p.wrapping_add(1);
            }
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
unsafe fn ZSTD_fillHashTableForCCtx(
    mut ms: *mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hBits = (*cParams).hashLog;
    let mls = (*cParams).minMatch;
    let base = (*ms).window.base;
    let mut ip = base.offset((*ms).nextToUpdate as isize);
    let iend = (end as *const u8).offset(-(HASH_READ_SIZE as isize));
    let fastHashFillStep = 3;
    while ip.offset(fastHashFillStep as isize) < iend.offset(2) {
        let curr = ip.offset_from(base) as core::ffi::c_long as u32;
        let hash0 = ZSTD_hashPtr(ip as *const core::ffi::c_void, hBits, mls);
        *hashTable.offset(hash0 as isize) = curr;
        if dtlm as core::ffi::c_uint != ZSTD_dtlm_fast as core::ffi::c_int as core::ffi::c_uint {
            let mut p: u32 = 0;
            p = 1;
            while p < fastHashFillStep {
                let hash = ZSTD_hashPtr(
                    ip.offset(p as isize) as *const core::ffi::c_void,
                    hBits,
                    mls,
                );
                if *hashTable.offset(hash as isize) == 0 {
                    *hashTable.offset(hash as isize) = curr.wrapping_add(p);
                }
                p = p.wrapping_add(1);
            }
        }
        ip = ip.offset(fastHashFillStep as isize);
    }
}
pub unsafe fn ZSTD_fillHashTable(
    mut ms: *mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
    mut dtlm: ZSTD_dictTableLoadMethod_e,
    mut tfp: ZSTD_tableFillPurpose_e,
) {
    if tfp as core::ffi::c_uint == ZSTD_tfp_forCDict as core::ffi::c_int as core::ffi::c_uint {
        ZSTD_fillHashTableForCDict(ms, end, dtlm);
    } else {
        ZSTD_fillHashTableForCCtx(ms, end, dtlm);
    };
}
unsafe fn ZSTD_match4Found_cmov(
    mut currentPtr: *const u8,
    mut matchAddress: *const u8,
    mut matchIdx: u32,
    mut idxLowLimit: u32,
) -> core::ffi::c_int {
    static dummy: [u8; 4] = [
        0x12 as core::ffi::c_int as u8,
        0x34 as core::ffi::c_int as u8,
        0x56 as core::ffi::c_int as u8,
        0x78 as core::ffi::c_int as u8,
    ];
    let mut mvalAddr = ZSTD_selectAddr(matchIdx, idxLowLimit, matchAddress, dummy.as_ptr());
    if MEM_read32(currentPtr as *const core::ffi::c_void)
        != MEM_read32(mvalAddr as *const core::ffi::c_void)
    {
        return 0;
    }
    asm!("", options(preserves_flags, att_syntax));
    (matchIdx >= idxLowLimit) as core::ffi::c_int
}
unsafe fn ZSTD_match4Found_branch(
    mut currentPtr: *const u8,
    mut matchAddress: *const u8,
    mut matchIdx: u32,
    mut idxLowLimit: u32,
) -> core::ffi::c_int {
    let mut mval: u32 = 0;
    if matchIdx >= idxLowLimit {
        mval = MEM_read32(matchAddress as *const core::ffi::c_void);
    } else {
        mval = MEM_read32(currentPtr as *const core::ffi::c_void) ^ 1;
    }
    (MEM_read32(currentPtr as *const core::ffi::c_void) == mval) as core::ffi::c_int
}
#[inline(always)]
unsafe fn ZSTD_compressBlock_fast_noDict_generic(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mls: u32,
    mut useCmov: core::ffi::c_int,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as core::ffi::c_int as core::ffi::c_uint)
        .wrapping_add(1) as size_t;
    let base = (*ms).window.base;
    let istart = src as *const u8;
    let endIndex =
        (istart.offset_from(base) as core::ffi::c_long as size_t).wrapping_add(srcSize) as u32;
    let prefixStartIndex = ZSTD_getLowestPrefixIndex(ms, endIndex, (*cParams).windowLog);
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut anchor = istart;
    let mut ip0 = istart;
    let mut ip1 = core::ptr::null::<u8>();
    let mut ip2 = core::ptr::null::<u8>();
    let mut ip3 = core::ptr::null::<u8>();
    let mut current0: u32 = 0;
    let mut rep_offset1 = *rep.offset(0);
    let mut rep_offset2 = *rep.offset(1);
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
    '__start: loop {
        step = stepSize;
        nextStep = ip0.offset(kStepIncr as isize);
        ip1 = ip0.offset(1);
        ip2 = ip0.offset(step as isize);
        ip3 = ip2.offset(1);
        if ip3 >= ilimit {
            break;
        }
        hash0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls);
        hash1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, hlog, mls);
        matchIdx = *hashTable.offset(hash0 as isize);
        loop {
            let rval = MEM_read32(ip2.offset(-(rep_offset1 as isize)) as *const core::ffi::c_void);
            current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
            *hashTable.offset(hash0 as isize) = current0;
            if (MEM_read32(ip2 as *const core::ffi::c_void) == rval) as core::ffi::c_int
                & (rep_offset1 > 0) as core::ffi::c_int
                != 0
            {
                ip0 = ip2;
                match0 = ip0.offset(-(rep_offset1 as isize));
                mLength = (*ip0.offset(-(1) as isize) as core::ffi::c_int
                    == *match0.offset(-(1) as isize) as core::ffi::c_int)
                    as core::ffi::c_int as size_t;
                ip0 = ip0.offset(-(mLength as isize));
                match0 = match0.offset(-(mLength as isize));
                offcode = REPCODE1_TO_OFFBASE as u32;
                mLength = mLength.wrapping_add(4);
                *hashTable.offset(hash1 as isize) =
                    ip1.offset_from(base) as core::ffi::c_long as u32;
                current_block = 4391991184774404966;
                break;
            } else if matchFound.unwrap_unchecked()(
                ip0,
                base.offset(matchIdx as isize),
                matchIdx,
                prefixStartIndex,
            ) != 0
            {
                *hashTable.offset(hash1 as isize) =
                    ip1.offset_from(base) as core::ffi::c_long as u32;
                current_block = 11113405673187116881;
                break;
            } else {
                matchIdx = *hashTable.offset(hash1 as isize);
                hash0 = hash1;
                hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);
                ip0 = ip1;
                ip1 = ip2;
                ip2 = ip3;
                current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
                *hashTable.offset(hash0 as isize) = current0;
                if matchFound.unwrap_unchecked()(
                    ip0,
                    base.offset(matchIdx as isize),
                    matchIdx,
                    prefixStartIndex,
                ) != 0
                {
                    if step <= 4 {
                        *hashTable.offset(hash1 as isize) =
                            ip1.offset_from(base) as core::ffi::c_long as u32;
                    }
                    current_block = 11113405673187116881;
                    break;
                } else {
                    matchIdx = *hashTable.offset(hash1 as isize);
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip0.offset(step as isize);
                    ip3 = ip1.offset(step as isize);
                    if ip2 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.offset(kStepIncr as isize);
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
            while (ip0 > anchor) as core::ffi::c_int & (match0 > prefixStart) as core::ffi::c_int
                != 0
                && *ip0.offset(-(1) as isize) as core::ffi::c_int
                    == *match0.offset(-(1) as isize) as core::ffi::c_int
            {
                ip0 = ip0.offset(-1);
                match0 = match0.offset(-1);
                mLength = mLength.wrapping_add(1);
            }
        }
        mLength = mLength.wrapping_add(ZSTD_count(
            ip0.offset(mLength as isize),
            match0.offset(mLength as isize),
            iend,
        ));
        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from(anchor) as core::ffi::c_long as size_t,
            anchor,
            iend,
            offcode,
            mLength,
        );
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip0 <= ilimit {
            *hashTable.offset(ZSTD_hashPtr(
                base.offset(current0 as isize).offset(2) as *const core::ffi::c_void,
                hlog,
                mls,
            ) as isize) = current0.wrapping_add(2);
            *hashTable.offset(
                ZSTD_hashPtr(ip0.offset(-(2)) as *const core::ffi::c_void, hlog, mls) as isize,
            ) = ip0.offset(-(2)).offset_from(base) as core::ffi::c_long as u32;
            if rep_offset2 > 0 {
                while ip0 <= ilimit
                    && MEM_read32(ip0 as *const core::ffi::c_void)
                        == MEM_read32(
                            ip0.offset(-(rep_offset2 as isize)) as *const core::ffi::c_void
                        )
                {
                    let rLength = (ZSTD_count(
                        ip0.offset(4),
                        ip0.offset(4).offset(-(rep_offset2 as isize)),
                        iend,
                    ))
                    .wrapping_add(4);
                    core::mem::swap(&mut rep_offset2, &mut rep_offset1);
                    *hashTable
                        .offset(ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls) as isize) =
                        ip0.offset_from(base) as core::ffi::c_long as u32;
                    ip0 = ip0.offset(rLength as isize);
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
    *rep.offset(0) = if rep_offset1 != 0 {
        rep_offset1
    } else {
        offsetSaved1
    };
    *rep.offset(1) = if rep_offset2 != 0 {
        rep_offset2
    } else {
        offsetSaved2
    };
    iend.offset_from(anchor) as core::ffi::c_long as size_t
}
unsafe fn ZSTD_compressBlock_fast_noDict_4_1(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 4, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_5_1(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 5, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_6_1(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 6, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_7_1(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 7, 1)
}
unsafe fn ZSTD_compressBlock_fast_noDict_4_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 4, 0)
}
unsafe fn ZSTD_compressBlock_fast_noDict_5_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 5, 0)
}
unsafe fn ZSTD_compressBlock_fast_noDict_6_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 6, 0)
}
unsafe fn ZSTD_compressBlock_fast_noDict_7_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_noDict_generic(ms, seqStore, rep, src, srcSize, 7, 0)
}
pub unsafe fn ZSTD_compressBlock_fast(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mml = (*ms).cParams.minMatch;
    let useCmov = ((*ms).cParams.windowLog < 19) as core::ffi::c_int;
    if useCmov != 0 {
        match mml {
            5 => ZSTD_compressBlock_fast_noDict_5_1(ms, seqStore, rep, src, srcSize),
            6 => ZSTD_compressBlock_fast_noDict_6_1(ms, seqStore, rep, src, srcSize),
            7 => ZSTD_compressBlock_fast_noDict_7_1(ms, seqStore, rep, src, srcSize),
            4 | _ => ZSTD_compressBlock_fast_noDict_4_1(ms, seqStore, rep, src, srcSize),
        }
    } else {
        match mml {
            5 => ZSTD_compressBlock_fast_noDict_5_0(ms, seqStore, rep, src, srcSize),
            6 => ZSTD_compressBlock_fast_noDict_6_0(ms, seqStore, rep, src, srcSize),
            7 => ZSTD_compressBlock_fast_noDict_7_0(ms, seqStore, rep, src, srcSize),
            4 | _ => ZSTD_compressBlock_fast_noDict_4_0(ms, seqStore, rep, src, srcSize),
        }
    }
}
#[inline(always)]
unsafe fn ZSTD_compressBlock_fast_dictMatchState_generic(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mls: u32,
    hasStep: u32,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as core::ffi::c_int as core::ffi::c_uint);
    let base = (*ms).window.base;
    let istart = src as *const u8;
    let mut ip0 = istart;
    let mut ip1 = ip0.offset(stepSize as isize);
    let mut anchor = istart;
    let prefixStartIndex = (*ms).window.dictLimit;
    let prefixStart = base.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut offset_1 = *rep.offset(0);
    let mut offset_2 = *rep.offset(1);
    let dms = (*ms).dictMatchState;
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
    let endIndex =
        (istart.offset_from(base) as core::ffi::c_long as size_t).wrapping_add(srcSize) as u32;
    if (*ms).prefetchCDictTables != 0 {
        let hashTableBytes = ((1 as core::ffi::c_int as size_t) << (*dictCParams).hashLog)
            .wrapping_mul(::core::mem::size_of::<u32>() as size_t);
        let _ptr = dictHashTable as *const core::ffi::c_char;
        let _size = hashTableBytes;
        let mut _pos: size_t = 0;
        _pos = 0;
        while _pos < _size {
            _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
        }
    }
    ip0 = ip0.offset((dictAndPrefixLength == 0) as core::ffi::c_int as isize);
    's_135: while ip1 <= ilimit {
        let mut mLength: size_t = 0;
        let mut hash0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls);
        let dictHashAndTag0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, dictHBits, mls);
        let mut dictMatchIndexAndTag =
            *dictHashTable.offset((dictHashAndTag0 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize);
        let mut dictTagsMatch =
            ZSTD_comparePackedTags(dictMatchIndexAndTag as size_t, dictHashAndTag0);
        let mut matchIndex = *hashTable.offset(hash0 as isize);
        let mut curr = ip0.offset_from(base) as core::ffi::c_long as u32;
        let mut step = stepSize as size_t;
        let kStepIncr = ((1) << kSearchStrength) as size_t;
        let mut nextStep = ip0.offset(kStepIncr as isize);
        loop {
            let mut match_0 = base.offset(matchIndex as isize);
            let repIndex = curr.wrapping_add(1).wrapping_sub(offset_1);
            let mut repMatch = if repIndex < prefixStartIndex {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
            } else {
                base.offset(repIndex as isize)
            };
            let hash1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, hlog, mls);
            let dictHashAndTag1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, dictHBits, mls);
            *hashTable.offset(hash0 as isize) = curr;
            if ZSTD_index_overlap_check(prefixStartIndex, repIndex) != 0
                && MEM_read32(repMatch as *const core::ffi::c_void)
                    == MEM_read32(ip0.offset(1) as *const core::ffi::c_void)
            {
                let repMatchEnd = if repIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                mLength = (ZSTD_count_2segments(
                    ip0.offset(1).offset(4),
                    repMatch.offset(4),
                    iend,
                    repMatchEnd,
                    prefixStart,
                ))
                .wrapping_add(4);
                ip0 = ip0.offset(1);
                ZSTD_storeSeq(
                    seqStore,
                    ip0.offset_from(anchor) as core::ffi::c_long as size_t,
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
                            ip0.offset(4),
                            dictMatch.offset(4),
                            iend,
                            dictEnd,
                            prefixStart,
                        ))
                        .wrapping_add(4);
                        while (ip0 > anchor) as core::ffi::c_int
                            & (dictMatch > dictStart) as core::ffi::c_int
                            != 0
                            && *ip0.offset(-(1) as isize) as core::ffi::c_int
                                == *dictMatch.offset(-(1) as isize) as core::ffi::c_int
                        {
                            ip0 = ip0.offset(-1);
                            dictMatch = dictMatch.offset(-1);
                            mLength = mLength.wrapping_add(1);
                        }
                        offset_2 = offset_1;
                        offset_1 = offset;
                        ZSTD_storeSeq(
                            seqStore,
                            ip0.offset_from(anchor) as core::ffi::c_long as size_t,
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
                    mLength = (ZSTD_count(ip0.offset(4), match_0.offset(4), iend)).wrapping_add(4);
                    while (ip0 > anchor) as core::ffi::c_int
                        & (match_0 > prefixStart) as core::ffi::c_int
                        != 0
                        && *ip0.offset(-(1) as isize) as core::ffi::c_int
                            == *match_0.offset(-(1) as isize) as core::ffi::c_int
                    {
                        ip0 = ip0.offset(-1);
                        match_0 = match_0.offset(-1);
                        mLength = mLength.wrapping_add(1);
                    }
                    offset_2 = offset_1;
                    offset_1 = offset_0;
                    ZSTD_storeSeq(
                        seqStore,
                        ip0.offset_from(anchor) as core::ffi::c_long as size_t,
                        anchor,
                        iend,
                        offset_0.wrapping_add(ZSTD_REP_NUM as u32),
                        mLength,
                    );
                    break;
                } else {
                    dictMatchIndexAndTag = *dictHashTable
                        .offset((dictHashAndTag1 >> ZSTD_SHORT_CACHE_TAG_BITS) as isize);
                    dictTagsMatch =
                        ZSTD_comparePackedTags(dictMatchIndexAndTag as size_t, dictHashAndTag1);
                    matchIndex = *hashTable.offset(hash1 as isize);
                    if ip1 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.offset(kStepIncr as isize);
                    }
                    ip0 = ip1;
                    ip1 = ip1.offset(step as isize);
                    if ip1 > ilimit {
                        break 's_135;
                    }
                    curr = ip0.offset_from(base) as core::ffi::c_long as u32;
                    hash0 = hash1;
                }
            }
        }
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip0 <= ilimit {
            *hashTable.offset(ZSTD_hashPtr(
                base.offset(curr as isize).offset(2) as *const core::ffi::c_void,
                hlog,
                mls,
            ) as isize) = curr.wrapping_add(2);
            *hashTable.offset(
                ZSTD_hashPtr(ip0.offset(-(2)) as *const core::ffi::c_void, hlog, mls) as isize,
            ) = ip0.offset(-(2)).offset_from(base) as core::ffi::c_long as u32;
            while ip0 <= ilimit {
                let current2 = ip0.offset_from(base) as core::ffi::c_long as u32;
                let repIndex2 = current2.wrapping_sub(offset_2);
                let mut repMatch2 = if repIndex2 < prefixStartIndex {
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
                    ip0.offset(4),
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
                *hashTable
                    .offset(ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls) as isize) =
                    current2;
                ip0 = ip0.offset(repLength2 as isize);
                anchor = ip0;
            }
        }
        ip1 = ip0.offset(stepSize as isize);
    }
    *rep.offset(0) = offset_1;
    *rep.offset(1) = offset_2;
    iend.offset_from(anchor) as core::ffi::c_long as size_t
}
unsafe fn ZSTD_compressBlock_fast_dictMatchState_4_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 4, 0)
}
unsafe fn ZSTD_compressBlock_fast_dictMatchState_5_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 5, 0)
}
unsafe fn ZSTD_compressBlock_fast_dictMatchState_6_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 6, 0)
}
unsafe fn ZSTD_compressBlock_fast_dictMatchState_7_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_dictMatchState_generic(ms, seqStore, rep, src, srcSize, 7, 0)
}
pub unsafe fn ZSTD_compressBlock_fast_dictMatchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => ZSTD_compressBlock_fast_dictMatchState_5_0(ms, seqStore, rep, src, srcSize),
        6 => ZSTD_compressBlock_fast_dictMatchState_6_0(ms, seqStore, rep, src, srcSize),
        7 => ZSTD_compressBlock_fast_dictMatchState_7_0(ms, seqStore, rep, src, srcSize),
        4 | _ => ZSTD_compressBlock_fast_dictMatchState_4_0(ms, seqStore, rep, src, srcSize),
    }
}
unsafe fn ZSTD_compressBlock_fast_extDict_generic(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mls: u32,
    hasStep: u32,
) -> size_t {
    let mut current_block: u64;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hlog = (*cParams).hashLog;
    let stepSize = ((*cParams).targetLength)
        .wrapping_add(((*cParams).targetLength == 0) as core::ffi::c_int as core::ffi::c_uint)
        .wrapping_add(1) as size_t;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let istart = src as *const u8;
    let mut anchor = istart;
    let endIndex =
        (istart.offset_from(base) as core::ffi::c_long as size_t).wrapping_add(srcSize) as u32;
    let lowLimit = ZSTD_getLowestMatchIndex(ms, endIndex, (*cParams).windowLog);
    let dictStartIndex = lowLimit;
    let dictStart = dictBase.offset(dictStartIndex as isize);
    let dictLimit = (*ms).window.dictLimit;
    let prefixStartIndex = if dictLimit < lowLimit {
        lowLimit
    } else {
        dictLimit
    };
    let prefixStart = base.offset(prefixStartIndex as isize);
    let dictEnd = dictBase.offset(prefixStartIndex as isize);
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(8));
    let mut offset_1 = *rep.offset(0);
    let mut offset_2 = *rep.offset(1);
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
        nextStep = ip0.offset(kStepIncr as isize);
        ip1 = ip0.offset(1);
        ip2 = ip0.offset(step as isize);
        ip3 = ip2.offset(1);
        if ip3 >= ilimit {
            break;
        }
        hash0 = ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls);
        hash1 = ZSTD_hashPtr(ip1 as *const core::ffi::c_void, hlog, mls);
        idx = *hashTable.offset(hash0 as isize);
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
            if (prefixStartIndex.wrapping_sub(repIndex) >= 4) as core::ffi::c_int
                & (offset_1 > 0) as core::ffi::c_int
                != 0
            {
                rval = MEM_read32(repBase.offset(repIndex as isize) as *const core::ffi::c_void);
            } else {
                rval = MEM_read32(ip2 as *const core::ffi::c_void) ^ 1;
            }
            current0 = ip0.offset_from(base) as core::ffi::c_long as u32;
            *hashTable.offset(hash0 as isize) = current0;
            if MEM_read32(ip2 as *const core::ffi::c_void) == rval {
                ip0 = ip2;
                match0 = repBase.offset(repIndex as isize);
                matchEnd = if repIndex < prefixStartIndex {
                    dictEnd
                } else {
                    iend
                };
                mLength = (*ip0.offset(-(1) as isize) as core::ffi::c_int
                    == *match0.offset(-(1) as isize) as core::ffi::c_int)
                    as core::ffi::c_int as size_t;
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
                    idx = *hashTable.offset(hash1 as isize);
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
                    *hashTable.offset(hash0 as isize) = current0;
                    let mval_0 = if idx >= dictStartIndex {
                        MEM_read32(idxBase.offset(idx as isize) as *const core::ffi::c_void)
                    } else {
                        MEM_read32(ip0 as *const core::ffi::c_void) ^ 1
                    };
                    if MEM_read32(ip0 as *const core::ffi::c_void) == mval_0 {
                        current_block = 934346911184053177;
                        break;
                    }
                    idx = *hashTable.offset(hash1 as isize);
                    idxBase = if idx < prefixStartIndex {
                        dictBase
                    } else {
                        base
                    };
                    hash0 = hash1;
                    hash1 = ZSTD_hashPtr(ip2 as *const core::ffi::c_void, hlog, mls);
                    ip0 = ip1;
                    ip1 = ip2;
                    ip2 = ip0.offset(step as isize);
                    ip3 = ip1.offset(step as isize);
                    if ip2 >= nextStep {
                        step = step.wrapping_add(1);
                        nextStep = nextStep.offset(kStepIncr as isize);
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
            while (ip0 > anchor) as core::ffi::c_int & (match0 > lowMatchPtr) as core::ffi::c_int
                != 0
                && *ip0.offset(-(1) as isize) as core::ffi::c_int
                    == *match0.offset(-(1) as isize) as core::ffi::c_int
            {
                ip0 = ip0.offset(-1);
                match0 = match0.offset(-1);
                mLength = mLength.wrapping_add(1);
            }
        }
        mLength = mLength.wrapping_add(ZSTD_count_2segments(
            ip0.offset(mLength as isize),
            match0.offset(mLength as isize),
            iend,
            matchEnd,
            prefixStart,
        ));
        ZSTD_storeSeq(
            seqStore,
            ip0.offset_from(anchor) as core::ffi::c_long as size_t,
            anchor,
            iend,
            offcode,
            mLength,
        );
        ip0 = ip0.offset(mLength as isize);
        anchor = ip0;
        if ip1 < ip0 {
            *hashTable.offset(hash1 as isize) = ip1.offset_from(base) as core::ffi::c_long as u32;
        }
        if ip0 <= ilimit {
            *hashTable.offset(ZSTD_hashPtr(
                base.offset(current0 as isize).offset(2) as *const core::ffi::c_void,
                hlog,
                mls,
            ) as isize) = current0.wrapping_add(2);
            *hashTable.offset(
                ZSTD_hashPtr(ip0.offset(-(2)) as *const core::ffi::c_void, hlog, mls) as isize,
            ) = ip0.offset(-(2)).offset_from(base) as core::ffi::c_long as u32;
            while ip0 <= ilimit {
                let repIndex2 =
                    (ip0.offset_from(base) as core::ffi::c_long as u32).wrapping_sub(offset_2);
                let repMatch2 = if repIndex2 < prefixStartIndex {
                    dictBase.offset(repIndex2 as isize)
                } else {
                    base.offset(repIndex2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixStartIndex, repIndex2)
                    & (offset_2 > 0) as core::ffi::c_int
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
                    ip0.offset(4),
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
                *hashTable
                    .offset(ZSTD_hashPtr(ip0 as *const core::ffi::c_void, hlog, mls) as isize) =
                    ip0.offset_from(base) as core::ffi::c_long as u32;
                ip0 = ip0.offset(repLength2 as isize);
                anchor = ip0;
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
    iend.offset_from(anchor) as core::ffi::c_long as size_t
}
unsafe fn ZSTD_compressBlock_fast_extDict_4_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 4, 0)
}
unsafe fn ZSTD_compressBlock_fast_extDict_5_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 5, 0)
}
unsafe fn ZSTD_compressBlock_fast_extDict_6_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 6, 0)
}
unsafe fn ZSTD_compressBlock_fast_extDict_7_0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_fast_extDict_generic(ms, seqStore, rep, src, srcSize, 7, 0)
}
pub unsafe fn ZSTD_compressBlock_fast_extDict(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mls = (*ms).cParams.minMatch;
    match mls {
        5 => ZSTD_compressBlock_fast_extDict_5_0(ms, seqStore, rep, src, srcSize),
        6 => ZSTD_compressBlock_fast_extDict_6_0(ms, seqStore, rep, src, srcSize),
        7 => ZSTD_compressBlock_fast_extDict_7_0(ms, seqStore, rep, src, srcSize),
        4 | _ => ZSTD_compressBlock_fast_extDict_4_0(ms, seqStore, rep, src, srcSize),
    }
}
