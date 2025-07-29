use core::arch::asm;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{
    __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8, _mm_set_epi8,
    _mm_setzero_si128, _mm_storeu_si128,
};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{
    __m128i, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_movemask_epi8, _mm_set1_epi8, _mm_set_epi8,
    _mm_setzero_si128, _mm_storeu_si128,
};
pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __loadu_si128 {
    pub __v: __m128i,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __storeu_si128 {
    pub __v: __m128i,
}
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
pub type unalignArch = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqStore_t {
    pub sequencesStart: *mut SeqDef,
    pub sequences: *mut SeqDef,
    pub litStart: *mut u8,
    pub lit: *mut u8,
    pub llCode: *mut u8,
    pub mlCode: *mut u8,
    pub ofCode: *mut u8,
    pub maxNbSeq: size_t,
    pub maxNbLit: size_t,
    pub longLengthType: ZSTD_longLengthType_e,
    pub longLengthPos: u32,
}
pub type ZSTD_longLengthType_e = std::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type SeqDef = SeqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqDef_s {
    pub offBase: u32,
    pub litLength: u16,
    pub mlBase: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_MatchState_t {
    pub window: ZSTD_window_t,
    pub loadedDictEnd: u32,
    pub nextToUpdate: u32,
    pub hashLog3: u32,
    pub rowHashLog: u32,
    pub tagTable: *mut u8,
    pub hashCache: [u32; 8],
    pub hashSalt: u64,
    pub hashSaltEntropy: u32,
    pub hashTable: *mut u32,
    pub hashTable3: *mut u32,
    pub chainTable: *mut u32,
    pub forceNonContiguous: std::ffi::c_int,
    pub dedicatedDictSearch: std::ffi::c_int,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_MatchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const RawSeqStore_t,
    pub prefetchCDictTables: std::ffi::c_int,
    pub lazySkipping: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: size_t,
    pub posInSequence: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeq {
    pub offset: u32,
    pub litLength: u32,
    pub matchLength: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optState_t {
    pub litFreq: *mut std::ffi::c_uint,
    pub litLengthFreq: *mut std::ffi::c_uint,
    pub matchLengthFreq: *mut std::ffi::c_uint,
    pub offCodeFreq: *mut std::ffi::c_uint,
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
pub type ZSTD_ParamSwitch_e = std::ffi::c_uint;
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
pub type FSE_repeat = std::ffi::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = std::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = size_t;
pub type ZSTD_OptPrice_e = std::ffi::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: std::ffi::c_int,
    pub off: u32,
    pub mlen: u32,
    pub litlen: u32,
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: u32,
    pub len: u32,
}
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
pub type ZSTD_overlap_e = std::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
pub type ZSTD_dictMode_e = std::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type searchMethod_e = std::ffi::c_uint;
pub const search_rowHash: searchMethod_e = 2;
pub const search_binaryTree: searchMethod_e = 1;
pub const search_hashChain: searchMethod_e = 0;
pub type ZSTD_VecMask = u64;
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 8 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
use crate::lib::zstd::*;
use crate::{MEM_isLittleEndian, MEM_read16, MEM_read32, MEM_readLE32, MEM_readLE64, MEM_readST};
pub const kSearchStrength: std::ffi::c_int = 8 as std::ffi::c_int;
pub const ZSTD_DUBT_UNSORTED_MARK: std::ffi::c_int = 1 as std::ffi::c_int;
pub const ZSTD_ROW_HASH_CACHE_SIZE: std::ffi::c_int = 8 as std::ffi::c_int;
unsafe extern "C" fn ZSTD_safecopyLiterals(
    mut op: *mut u8,
    mut ip: *const u8,
    iend: *const u8,
    mut ilimit_w: *const u8,
) {
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            ilimit_w.offset_from(ip) as std::ffi::c_long as size_t,
            ZSTD_no_overlap,
        );
        op = op.offset(ilimit_w.offset_from(ip) as std::ffi::c_long as isize);
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
pub const REPCODE1_TO_OFFBASE: std::ffi::c_int = 1 as std::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn ZSTD_storeSeqOnly(
    mut seqStorePtr: *mut SeqStore_t,
    mut litLength: size_t,
    mut offBase: u32,
    mut matchLength: size_t,
) {
    if (litLength > 0xffff as std::ffi::c_int as size_t) as std::ffi::c_int as std::ffi::c_long != 0
    {
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength;
        (*seqStorePtr).longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart)
            as std::ffi::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0 as std::ffi::c_int as isize)).litLength =
        litLength as u16;
    (*((*seqStorePtr).sequences).offset(0 as std::ffi::c_int as isize)).offBase = offBase;
    let mlBase = matchLength.wrapping_sub(MINMATCH as size_t);
    if (mlBase > 0xffff as std::ffi::c_int as size_t) as std::ffi::c_int as std::ffi::c_long != 0 {
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr).longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart)
            as std::ffi::c_long as u32;
    }
    (*((*seqStorePtr).sequences).offset(0 as std::ffi::c_int as isize)).mlBase = mlBase as u16;
    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
    (*seqStorePtr).sequences;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_storeSeq(
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
            (*seqStorePtr).lit as *mut std::ffi::c_void,
            literals as *const std::ffi::c_void,
        );
        if litLength > 16 as std::ffi::c_int as size_t {
            ZSTD_wildcopy(
                ((*seqStorePtr).lit).offset(16 as std::ffi::c_int as isize)
                    as *mut std::ffi::c_void,
                literals.offset(16 as std::ffi::c_int as isize) as *const std::ffi::c_void,
                litLength.wrapping_sub(16 as std::ffi::c_int as size_t),
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
unsafe extern "C" fn ZSTD_count(
    mut pIn: *const u8,
    mut pMatch: *const u8,
    pInLimit: *const u8,
) -> size_t {
    let pStart = pIn;
    let pInLoopLimit = pInLimit.offset(
        -((::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as isize),
    );
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const std::ffi::c_void)
            ^ MEM_readST(pIn as *const std::ffi::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as size_t;
        }
        pIn = pIn.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
        pMatch = pMatch.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const std::ffi::c_void)
                ^ MEM_readST(pIn as *const std::ffi::c_void);
            if diff_0 == 0 {
                pIn = pIn.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
                pMatch =
                    pMatch.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as std::ffi::c_long as size_t;
            }
        }
    }
    if MEM_64bits() != 0
        && pIn < pInLimit.offset(-(3 as std::ffi::c_int as isize))
        && MEM_read32(pMatch as *const std::ffi::c_void)
            == MEM_read32(pIn as *const std::ffi::c_void)
    {
        pIn = pIn.offset(4 as std::ffi::c_int as isize);
        pMatch = pMatch.offset(4 as std::ffi::c_int as isize);
    }
    if pIn < pInLimit.offset(-(1 as std::ffi::c_int as isize))
        && MEM_read16(pMatch as *const std::ffi::c_void) as std::ffi::c_int
            == MEM_read16(pIn as *const std::ffi::c_void) as std::ffi::c_int
    {
        pIn = pIn.offset(2 as std::ffi::c_int as isize);
        pMatch = pMatch.offset(2 as std::ffi::c_int as isize);
    }
    if pIn < pInLimit && *pMatch as std::ffi::c_int == *pIn as std::ffi::c_int {
        pIn = pIn.offset(1);
        pIn;
    }
    pIn.offset_from(pStart) as std::ffi::c_long as size_t
}
#[inline]
unsafe extern "C" fn ZSTD_count_2segments(
    mut ip: *const u8,
    mut match_0: *const u8,
    mut iEnd: *const u8,
    mut mEnd: *const u8,
    mut iStart: *const u8,
) -> size_t {
    let vEnd = if ip.offset(mEnd.offset_from(match_0) as std::ffi::c_long as isize) < iEnd {
        ip.offset(mEnd.offset_from(match_0) as std::ffi::c_long as isize)
    } else {
        iEnd
    };
    let matchLength = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd {
        return matchLength;
    }
    matchLength.wrapping_add(ZSTD_count(ip.offset(matchLength as isize), iStart, iEnd))
}
static mut prime4bytes: u32 = 2654435761 as std::ffi::c_uint;
unsafe extern "C" fn ZSTD_hash4(mut u: u32, mut h: u32, mut s: u32) -> u32 {
    ((u * prime4bytes) ^ s) >> (32 as std::ffi::c_int as u32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const std::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0 as std::ffi::c_int as u32) as size_t
}
unsafe extern "C" fn ZSTD_hash4PtrS(
    mut ptr: *const std::ffi::c_void,
    mut h: u32,
    mut s: u32,
) -> size_t {
    ZSTD_hash4(MEM_readLE32(ptr), h, s) as size_t
}
static mut prime5bytes: u64 = 889523592379 as std::ffi::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash5(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    (((u << (64 as std::ffi::c_int - 40 as std::ffi::c_int)) * prime5bytes) ^ s)
        >> (64 as std::ffi::c_int as u32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const std::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash5(MEM_readLE64(p), h, 0 as std::ffi::c_int as u64)
}
unsafe extern "C" fn ZSTD_hash5PtrS(
    mut p: *const std::ffi::c_void,
    mut h: u32,
    mut s: u64,
) -> size_t {
    ZSTD_hash5(MEM_readLE64(p), h, s)
}
static mut prime6bytes: u64 = 227718039650203 as std::ffi::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash6(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    (((u << (64 as std::ffi::c_int - 48 as std::ffi::c_int)) * prime6bytes) ^ s)
        >> (64 as std::ffi::c_int as u32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const std::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash6(MEM_readLE64(p), h, 0 as std::ffi::c_int as u64)
}
unsafe extern "C" fn ZSTD_hash6PtrS(
    mut p: *const std::ffi::c_void,
    mut h: u32,
    mut s: u64,
) -> size_t {
    ZSTD_hash6(MEM_readLE64(p), h, s)
}
static mut prime7bytes: u64 = 58295818150454627 as std::ffi::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash7(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    (((u << (64 as std::ffi::c_int - 56 as std::ffi::c_int)) * prime7bytes) ^ s)
        >> (64 as std::ffi::c_int as u32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const std::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash7(MEM_readLE64(p), h, 0 as std::ffi::c_int as u64)
}
unsafe extern "C" fn ZSTD_hash7PtrS(
    mut p: *const std::ffi::c_void,
    mut h: u32,
    mut s: u64,
) -> size_t {
    ZSTD_hash7(MEM_readLE64(p), h, s)
}
static mut prime8bytes: u64 = 0xcf1bbcdcb7a56463 as std::ffi::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_hash8(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    ((u * prime8bytes) ^ s) >> (64 as std::ffi::c_int as u32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash8Ptr(mut p: *const std::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash8(MEM_readLE64(p), h, 0 as std::ffi::c_int as u64)
}
unsafe extern "C" fn ZSTD_hash8PtrS(
    mut p: *const std::ffi::c_void,
    mut h: u32,
    mut s: u64,
) -> size_t {
    ZSTD_hash8(MEM_readLE64(p), h, s)
}
#[inline(always)]
unsafe extern "C" fn ZSTD_hashPtr(
    mut p: *const std::ffi::c_void,
    mut hBits: u32,
    mut mls: u32,
) -> size_t {
    match mls {
        5 => ZSTD_hash5Ptr(p, hBits),
        6 => ZSTD_hash6Ptr(p, hBits),
        7 => ZSTD_hash7Ptr(p, hBits),
        8 => ZSTD_hash8Ptr(p, hBits),
        4 | _ => ZSTD_hash4Ptr(p, hBits),
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_hashPtrSalted(
    mut p: *const std::ffi::c_void,
    mut hBits: u32,
    mut mls: u32,
    hashSalt: u64,
) -> size_t {
    match mls {
        5 => ZSTD_hash5PtrS(p, hBits, hashSalt),
        6 => ZSTD_hash6PtrS(p, hBits, hashSalt),
        7 => ZSTD_hash7PtrS(p, hBits, hashSalt),
        8 => ZSTD_hash8PtrS(p, hBits, hashSalt),
        4 | _ => ZSTD_hash4PtrS(p, hBits, hashSalt as u32),
    }
}
#[inline]
unsafe extern "C" fn ZSTD_getLowestMatchIndex(
    mut ms: *const ZSTD_MatchState_t,
    mut curr: u32,
    mut windowLog: std::ffi::c_uint,
) -> u32 {
    let maxDistance = (1 as std::ffi::c_uint) << windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary =
        ((*ms).loadedDictEnd != 0 as std::ffi::c_int as u32) as std::ffi::c_int as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}
#[inline]
unsafe extern "C" fn ZSTD_getLowestPrefixIndex(
    mut ms: *const ZSTD_MatchState_t,
    mut curr: u32,
    mut windowLog: std::ffi::c_uint,
) -> u32 {
    let maxDistance = (1 as std::ffi::c_uint) << windowLog;
    let lowestValid = (*ms).window.dictLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary =
        ((*ms).loadedDictEnd != 0 as std::ffi::c_int as u32) as std::ffi::c_int as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}
#[inline]
unsafe extern "C" fn ZSTD_index_overlap_check(
    prefixLowestIndex: u32,
    repIndex: u32,
) -> std::ffi::c_int {
    (prefixLowestIndex
        .wrapping_sub(1 as std::ffi::c_int as u32)
        .wrapping_sub(repIndex)
        >= 3 as std::ffi::c_int as u32) as std::ffi::c_int
}
pub const ZSTD_REP_NUM: std::ffi::c_int = 3 as std::ffi::c_int;
pub const MINMATCH: std::ffi::c_int = 3 as std::ffi::c_int;
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut std::ffi::c_void, mut src: *const std::ffi::c_void) {
    libc::memcpy(
        dst,
        src,
        8 as std::ffi::c_int as std::ffi::c_ulong as libc::size_t,
    );
}
unsafe extern "C" fn ZSTD_copy16(mut dst: *mut std::ffi::c_void, mut src: *const std::ffi::c_void) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub const WILDCOPY_OVERLENGTH: std::ffi::c_int = 32 as std::ffi::c_int;
pub const WILDCOPY_VECLEN: std::ffi::c_int = 16 as std::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut length: size_t,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut u8).offset_from(src as *const u8) as std::ffi::c_long;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    if ovtype as std::ffi::c_uint
        == ZSTD_overlap_src_before_dst as std::ffi::c_int as std::ffi::c_uint
        && diff < WILDCOPY_VECLEN as ptrdiff_t
    {
        loop {
            ZSTD_copy8(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(8 as std::ffi::c_int as isize);
            ip = ip.offset(8 as std::ffi::c_int as isize);
            if op >= oend {
                break;
            }
        }
    } else {
        ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
        if 16 as std::ffi::c_int as size_t >= length {
            return;
        }
        op = op.offset(16 as std::ffi::c_int as isize);
        ip = ip.offset(16 as std::ffi::c_int as isize);
        loop {
            ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(16 as std::ffi::c_int as isize);
            ip = ip.offset(16 as std::ffi::c_int as isize);
            ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(16 as std::ffi::c_int as isize);
            ip = ip.offset(16 as std::ffi::c_int as isize);
            if op >= oend {
                break;
            }
        }
    };
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: u32) -> std::ffi::c_uint {
    val.trailing_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: u64) -> std::ffi::c_uint {
    (val as std::ffi::c_ulonglong).trailing_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros64(mut val: u64) -> std::ffi::c_uint {
    (val as std::ffi::c_ulonglong).leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_NbCommonBytes(mut val: size_t) -> std::ffi::c_uint {
    if MEM_isLittleEndian() != 0 {
        if MEM_64bits() != 0 {
            ZSTD_countTrailingZeros64(val) >> 3 as std::ffi::c_int
        } else {
            ZSTD_countTrailingZeros32(val as u32) >> 3 as std::ffi::c_int
        }
    } else if MEM_64bits() != 0 {
        ZSTD_countLeadingZeros64(val) >> 3 as std::ffi::c_int
    } else {
        ZSTD_countLeadingZeros32(val as u32) >> 3 as std::ffi::c_int
    }
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
#[inline]
unsafe extern "C" fn ZSTD_rotateRight_U64(value: u64, mut count: u32) -> u64 {
    count &= 0x3f as std::ffi::c_int as u32;
    value >> count
        | value
            << ((0 as std::ffi::c_uint).wrapping_sub(count)
                & 0x3f as std::ffi::c_int as std::ffi::c_uint)
}
#[inline]
unsafe extern "C" fn ZSTD_rotateRight_U32(value: u32, mut count: u32) -> u32 {
    count &= 0x1f as std::ffi::c_int as u32;
    value >> count
        | value
            << ((0 as std::ffi::c_uint).wrapping_sub(count)
                & 0x1f as std::ffi::c_int as std::ffi::c_uint)
}
#[inline]
unsafe extern "C" fn ZSTD_rotateRight_U16(value: u16, mut count: u32) -> u16 {
    count &= 0xf as std::ffi::c_int as u32;
    (value as std::ffi::c_int >> count
        | ((value as std::ffi::c_int)
            << ((0 as std::ffi::c_uint).wrapping_sub(count)
                & 0xf as std::ffi::c_int as std::ffi::c_uint)) as u16 as std::ffi::c_int) as u16
}
pub const ZSTD_LAZY_DDSS_BUCKET_LOG: std::ffi::c_int = 2 as std::ffi::c_int;
pub const ZSTD_ROW_HASH_TAG_BITS: std::ffi::c_int = 8 as std::ffi::c_int;
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const kLazySkippingStep: std::ffi::c_int = 8 as std::ffi::c_int;
unsafe extern "C" fn ZSTD_updateDUBT(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    mut iend: *const u8,
    mut mls: u32,
) {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let btMask = (((1 as std::ffi::c_int) << btLog) - 1 as std::ffi::c_int) as u32;
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as std::ffi::c_long as u32;
    let mut idx = (*ms).nextToUpdate;
    idx != target;
    while idx < target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const std::ffi::c_void,
            hashLog,
            mls,
        );
        let matchIndex = *hashTable.offset(h as isize);
        let nextCandidatePtr = bt.offset((2 as std::ffi::c_int as u32 * (idx & btMask)) as isize);
        let sortMarkPtr = nextCandidatePtr.offset(1 as std::ffi::c_int as isize);
        *hashTable.offset(h as isize) = idx;
        *nextCandidatePtr = matchIndex;
        *sortMarkPtr = ZSTD_DUBT_UNSORTED_MARK as u32;
        idx = idx.wrapping_add(1);
        idx;
    }
    (*ms).nextToUpdate = target;
}
unsafe extern "C" fn ZSTD_insertDUBT1(
    mut ms: *const ZSTD_MatchState_t,
    mut curr: u32,
    mut inputEnd: *const u8,
    mut nbCompares: u32,
    mut btLow: u32,
    dictMode: ZSTD_dictMode_e,
) {
    let cParams: *const ZSTD_compressionParameters = &(*ms).cParams;
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let btMask = (((1 as std::ffi::c_int) << btLog) - 1 as std::ffi::c_int) as u32;
    let mut commonLengthSmaller = 0 as std::ffi::c_int as size_t;
    let mut commonLengthLarger = 0 as std::ffi::c_int as size_t;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
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
    let mut match_0 = std::ptr::null::<u8>();
    let mut smallerPtr = bt.offset((2 as std::ffi::c_int as u32 * (curr & btMask)) as isize);
    let mut largerPtr = smallerPtr.offset(1 as std::ffi::c_int as isize);
    let mut matchIndex = *smallerPtr;
    let mut dummy32: u32 = 0;
    let windowValid = (*ms).window.lowLimit;
    let maxDistance = (1 as std::ffi::c_uint) << (*cParams).windowLog;
    let windowLow = if curr.wrapping_sub(windowValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        windowValid
    };
    while nbCompares != 0 && matchIndex > windowLow {
        let nextPtr = bt.offset((2 as std::ffi::c_int as u32 * (matchIndex & btMask)) as isize);
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        if dictMode as std::ffi::c_uint != ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint
            || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
            || curr < dictLimit
        {
            let mBase = if dictMode as std::ffi::c_uint
                != ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint
                || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
            {
                base
            } else {
                dictBase
            };
            match_0 = mBase.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.offset(matchLength as isize),
                match_0.offset(matchLength as isize),
                iend,
            ));
        } else {
            match_0 = dictBase.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.offset(matchLength as isize),
                match_0.offset(matchLength as isize),
                iend,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                match_0 = base.offset(matchIndex as isize);
            }
        }
        if ip.offset(matchLength as isize) == iend {
            break;
        } else {
            if (*match_0.offset(matchLength as isize) as std::ffi::c_int)
                < *ip.offset(matchLength as isize) as std::ffi::c_int
            {
                *smallerPtr = matchIndex;
                commonLengthSmaller = matchLength;
                if matchIndex <= btLow {
                    smallerPtr = &mut dummy32;
                    break;
                } else {
                    smallerPtr = nextPtr.offset(1 as std::ffi::c_int as isize);
                    matchIndex = *nextPtr.offset(1 as std::ffi::c_int as isize);
                }
            } else {
                *largerPtr = matchIndex;
                commonLengthLarger = matchLength;
                if matchIndex <= btLow {
                    largerPtr = &mut dummy32;
                    break;
                } else {
                    largerPtr = nextPtr;
                    matchIndex = *nextPtr.offset(0 as std::ffi::c_int as isize);
                }
            }
            nbCompares = nbCompares.wrapping_sub(1);
            nbCompares;
        }
    }
    *largerPtr = 0 as std::ffi::c_int as u32;
    *smallerPtr = *largerPtr;
}
unsafe extern "C" fn ZSTD_DUBT_findBetterDictMatch(
    mut ms: *const ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    mut offsetPtr: *mut size_t,
    mut bestLength: size_t,
    mut nbCompares: u32,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let dms = (*ms).dictMatchState;
    let dmsCParams: *const ZSTD_compressionParameters = &(*dms).cParams;
    let dictHashTable: *const u32 = (*dms).hashTable;
    let hashLog = (*dmsCParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const std::ffi::c_void, hashLog, mls);
    let mut dictMatchIndex = *dictHashTable.offset(h as isize);
    let base = (*ms).window.base;
    let prefixStart = base.offset((*ms).window.dictLimit as isize);
    let curr = ip.offset_from(base) as std::ffi::c_long as u32;
    let dictBase = (*dms).window.base;
    let dictEnd = (*dms).window.nextSrc;
    let dictHighLimit =
        ((*dms).window.nextSrc).offset_from((*dms).window.base) as std::ffi::c_long as u32;
    let dictLowLimit = (*dms).window.lowLimit;
    let dictIndexDelta = ((*ms).window.lowLimit).wrapping_sub(dictHighLimit);
    let dictBt = (*dms).chainTable;
    let btLog = ((*dmsCParams).chainLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let btMask = (((1 as std::ffi::c_int) << btLog) - 1 as std::ffi::c_int) as u32;
    let btLow = if btMask >= dictHighLimit.wrapping_sub(dictLowLimit) {
        dictLowLimit
    } else {
        dictHighLimit.wrapping_sub(btMask)
    };
    let mut commonLengthSmaller = 0 as std::ffi::c_int as size_t;
    let mut commonLengthLarger = 0 as std::ffi::c_int as size_t;
    while nbCompares != 0 && dictMatchIndex > dictLowLimit {
        let nextPtr =
            dictBt.offset((2 as std::ffi::c_int as u32 * (dictMatchIndex & btMask)) as isize);
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        let mut match_0 = dictBase.offset(dictMatchIndex as isize);
        matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
            ip.offset(matchLength as isize),
            match_0.offset(matchLength as isize),
            iend,
            dictEnd,
            prefixStart,
        ));
        if (dictMatchIndex as size_t).wrapping_add(matchLength) >= dictHighLimit as size_t {
            match_0 = base
                .offset(dictMatchIndex as isize)
                .offset(dictIndexDelta as isize);
        }
        if matchLength > bestLength {
            let mut matchIndex = dictMatchIndex.wrapping_add(dictIndexDelta);
            if 4 as std::ffi::c_int * matchLength.wrapping_sub(bestLength) as std::ffi::c_int
                > (ZSTD_highbit32(
                    curr.wrapping_sub(matchIndex)
                        .wrapping_add(1 as std::ffi::c_int as u32),
                ))
                .wrapping_sub(ZSTD_highbit32(
                    (*offsetPtr.offset(0 as std::ffi::c_int as isize) as u32)
                        .wrapping_add(1 as std::ffi::c_int as u32),
                )) as std::ffi::c_int
            {
                bestLength = matchLength;
                *offsetPtr = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            }
            if ip.offset(matchLength as isize) == iend {
                break;
            }
        }
        if (*match_0.offset(matchLength as isize) as std::ffi::c_int)
            < *ip.offset(matchLength as isize) as std::ffi::c_int
        {
            if dictMatchIndex <= btLow {
                break;
            }
            commonLengthSmaller = matchLength;
            dictMatchIndex = *nextPtr.offset(1 as std::ffi::c_int as isize);
        } else {
            if dictMatchIndex <= btLow {
                break;
            }
            commonLengthLarger = matchLength;
            dictMatchIndex = *nextPtr.offset(0 as std::ffi::c_int as isize);
        }
        nbCompares = nbCompares.wrapping_sub(1);
        nbCompares;
    }
    if bestLength >= MINMATCH as size_t {
        let mIndex = curr.wrapping_sub((*offsetPtr).wrapping_sub(ZSTD_REP_NUM as size_t) as u32);
    }
    bestLength
}
unsafe extern "C" fn ZSTD_DUBT_findBestMatch(
    mut ms: *mut ZSTD_MatchState_t,
    ip: *const u8,
    iend: *const u8,
    mut offBasePtr: *mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const std::ffi::c_void, hashLog, mls);
    let mut matchIndex = *hashTable.offset(h as isize);
    let base = (*ms).window.base;
    let curr = ip.offset_from(base) as std::ffi::c_long as u32;
    let windowLow = ZSTD_getLowestMatchIndex(ms, curr, (*cParams).windowLog);
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let btMask = (((1 as std::ffi::c_int) << btLog) - 1 as std::ffi::c_int) as u32;
    let btLow = if btMask >= curr {
        0 as std::ffi::c_int as u32
    } else {
        curr.wrapping_sub(btMask)
    };
    let unsortLimit = if btLow > windowLow { btLow } else { windowLow };
    let mut nextCandidate =
        bt.offset((2 as std::ffi::c_int as u32 * (matchIndex & btMask)) as isize);
    let mut unsortedMark = bt
        .offset((2 as std::ffi::c_int as u32 * (matchIndex & btMask)) as isize)
        .offset(1 as std::ffi::c_int as isize);
    let mut nbCompares = (1 as std::ffi::c_uint) << (*cParams).searchLog;
    let mut nbCandidates = nbCompares;
    let mut previousCandidate = 0 as std::ffi::c_int as u32;
    while matchIndex > unsortLimit
        && *unsortedMark == ZSTD_DUBT_UNSORTED_MARK as u32
        && nbCandidates > 1 as std::ffi::c_int as u32
    {
        *unsortedMark = previousCandidate;
        previousCandidate = matchIndex;
        matchIndex = *nextCandidate;
        nextCandidate = bt.offset((2 as std::ffi::c_int as u32 * (matchIndex & btMask)) as isize);
        unsortedMark = bt
            .offset((2 as std::ffi::c_int as u32 * (matchIndex & btMask)) as isize)
            .offset(1 as std::ffi::c_int as isize);
        nbCandidates = nbCandidates.wrapping_sub(1);
        nbCandidates;
    }
    if matchIndex > unsortLimit && *unsortedMark == ZSTD_DUBT_UNSORTED_MARK as u32 {
        *unsortedMark = 0 as std::ffi::c_int as u32;
        *nextCandidate = *unsortedMark;
    }
    matchIndex = previousCandidate;
    while matchIndex != 0 {
        let nextCandidateIdxPtr = bt
            .offset((2 as std::ffi::c_int as u32 * (matchIndex & btMask)) as isize)
            .offset(1 as std::ffi::c_int as isize);
        let nextCandidateIdx = *nextCandidateIdxPtr;
        ZSTD_insertDUBT1(ms, matchIndex, iend, nbCandidates, unsortLimit, dictMode);
        matchIndex = nextCandidateIdx;
        nbCandidates = nbCandidates.wrapping_add(1);
        nbCandidates;
    }
    let mut commonLengthSmaller = 0 as std::ffi::c_int as size_t;
    let mut commonLengthLarger = 0 as std::ffi::c_int as size_t;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut smallerPtr = bt.offset((2 as std::ffi::c_int as u32 * (curr & btMask)) as isize);
    let mut largerPtr = bt
        .offset((2 as std::ffi::c_int as u32 * (curr & btMask)) as isize)
        .offset(1 as std::ffi::c_int as isize);
    let mut matchEndIdx = curr
        .wrapping_add(8 as std::ffi::c_int as u32)
        .wrapping_add(1 as std::ffi::c_int as u32);
    let mut dummy32: u32 = 0;
    let mut bestLength = 0 as std::ffi::c_int as size_t;
    matchIndex = *hashTable.offset(h as isize);
    *hashTable.offset(h as isize) = curr;
    while nbCompares != 0 && matchIndex > windowLow {
        let nextPtr = bt.offset((2 as std::ffi::c_int as u32 * (matchIndex & btMask)) as isize);
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        let mut match_0 = std::ptr::null::<u8>();
        if dictMode as std::ffi::c_uint != ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint
            || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
        {
            match_0 = base.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.offset(matchLength as isize),
                match_0.offset(matchLength as isize),
                iend,
            ));
        } else {
            match_0 = dictBase.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.offset(matchLength as isize),
                match_0.offset(matchLength as isize),
                iend,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                match_0 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                matchEndIdx = matchIndex.wrapping_add(matchLength as u32);
            }
            if 4 as std::ffi::c_int * matchLength.wrapping_sub(bestLength) as std::ffi::c_int
                > (ZSTD_highbit32(
                    curr.wrapping_sub(matchIndex)
                        .wrapping_add(1 as std::ffi::c_int as u32),
                ))
                .wrapping_sub(ZSTD_highbit32(*offBasePtr as u32))
                    as std::ffi::c_int
            {
                bestLength = matchLength;
                *offBasePtr = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            }
            if ip.offset(matchLength as isize) == iend {
                if dictMode as std::ffi::c_uint
                    == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
                {
                    nbCompares = 0 as std::ffi::c_int as u32;
                }
                break;
            }
        }
        if (*match_0.offset(matchLength as isize) as std::ffi::c_int)
            < *ip.offset(matchLength as isize) as std::ffi::c_int
        {
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32;
                break;
            } else {
                smallerPtr = nextPtr.offset(1 as std::ffi::c_int as isize);
                matchIndex = *nextPtr.offset(1 as std::ffi::c_int as isize);
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32;
                break;
            } else {
                largerPtr = nextPtr;
                matchIndex = *nextPtr.offset(0 as std::ffi::c_int as isize);
            }
        }
        nbCompares = nbCompares.wrapping_sub(1);
        nbCompares;
    }
    *largerPtr = 0 as std::ffi::c_int as u32;
    *smallerPtr = *largerPtr;
    if dictMode as std::ffi::c_uint == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
        && nbCompares != 0
    {
        bestLength = ZSTD_DUBT_findBetterDictMatch(
            ms, ip, iend, offBasePtr, bestLength, nbCompares, mls, dictMode,
        );
    }
    (*ms).nextToUpdate = matchEndIdx.wrapping_sub(8 as std::ffi::c_int as u32);
    if bestLength >= MINMATCH as size_t {
        let mIndex = curr.wrapping_sub((*offBasePtr).wrapping_sub(ZSTD_REP_NUM as size_t) as u32);
    }
    bestLength
}
#[inline(always)]
unsafe extern "C" fn ZSTD_BtFindBestMatch(
    mut ms: *mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    if ip < ((*ms).window.base).offset((*ms).nextToUpdate as isize) {
        return 0 as std::ffi::c_int as size_t;
    }
    ZSTD_updateDUBT(ms, ip, iLimit, mls);
    ZSTD_DUBT_findBestMatch(ms, ip, iLimit, offBasePtr, mls, dictMode)
}
#[export_name = crate::prefix!(ZSTD_dedicatedDictSearch_lazy_loadDictionary)]
pub unsafe extern "C" fn ZSTD_dedicatedDictSearch_lazy_loadDictionary(
    mut ms: *mut ZSTD_MatchState_t,
    ip: *const u8,
) {
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as std::ffi::c_long as u32;
    let hashTable = (*ms).hashTable;
    let chainTable = (*ms).chainTable;
    let chainSize = ((1 as std::ffi::c_int) << (*ms).cParams.chainLog) as u32;
    let mut idx = (*ms).nextToUpdate;
    let minChain = if chainSize < target.wrapping_sub(idx) {
        target.wrapping_sub(chainSize)
    } else {
        idx
    };
    let bucketSize = ((1 as std::ffi::c_int) << ZSTD_LAZY_DDSS_BUCKET_LOG) as u32;
    let cacheSize = bucketSize.wrapping_sub(1 as std::ffi::c_int as u32);
    let chainAttempts =
        (((1 as std::ffi::c_int) << (*ms).cParams.searchLog) as u32).wrapping_sub(cacheSize);
    let chainLimit = if chainAttempts > 255 as std::ffi::c_int as u32 {
        255 as std::ffi::c_int as u32
    } else {
        chainAttempts
    };
    let hashLog =
        ((*ms).cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as std::ffi::c_uint);
    let tmpHashTable = hashTable;
    let tmpChainTable = hashTable.offset(((1 as std::ffi::c_int as size_t) << hashLog) as isize);
    let tmpChainSize = ((((1 as std::ffi::c_int) << ZSTD_LAZY_DDSS_BUCKET_LOG)
        - 1 as std::ffi::c_int) as u32)
        << hashLog;
    let tmpMinChain = if tmpChainSize < target {
        target.wrapping_sub(tmpChainSize)
    } else {
        idx
    };
    let mut hashIdx: u32 = 0;
    while idx < target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const std::ffi::c_void,
            hashLog,
            (*ms).cParams.minMatch,
        ) as u32;
        if idx >= tmpMinChain {
            *tmpChainTable.offset(idx.wrapping_sub(tmpMinChain) as isize) =
                *hashTable.offset(h as isize);
        }
        *tmpHashTable.offset(h as isize) = idx;
        idx = idx.wrapping_add(1);
        idx;
    }
    let mut chainPos = 0 as std::ffi::c_int as u32;
    hashIdx = 0 as std::ffi::c_int as u32;
    while hashIdx < (1 as std::ffi::c_uint) << hashLog {
        let mut count: u32 = 0;
        let mut countBeyondMinChain = 0 as std::ffi::c_int as u32;
        let mut i = *tmpHashTable.offset(hashIdx as isize);
        count = 0 as std::ffi::c_int as u32;
        while i >= tmpMinChain && count < cacheSize {
            if i < minChain {
                countBeyondMinChain = countBeyondMinChain.wrapping_add(1);
                countBeyondMinChain;
            }
            i = *tmpChainTable.offset(i.wrapping_sub(tmpMinChain) as isize);
            count = count.wrapping_add(1);
            count;
        }
        if count == cacheSize {
            count = 0 as std::ffi::c_int as u32;
            while count < chainLimit {
                if i < minChain
                    && (i == 0 || {
                        countBeyondMinChain = countBeyondMinChain.wrapping_add(1);
                        countBeyondMinChain > cacheSize
                    })
                {
                    break;
                }
                let fresh2 = chainPos;
                chainPos = chainPos.wrapping_add(1);
                *chainTable.offset(fresh2 as isize) = i;
                count = count.wrapping_add(1);
                count;
                if i < tmpMinChain {
                    break;
                }
                i = *tmpChainTable.offset(i.wrapping_sub(tmpMinChain) as isize);
            }
        } else {
            count = 0 as std::ffi::c_int as u32;
        }
        if count != 0 {
            *tmpHashTable.offset(hashIdx as isize) =
                (chainPos.wrapping_sub(count) << 8 as std::ffi::c_int).wrapping_add(count);
        } else {
            *tmpHashTable.offset(hashIdx as isize) = 0 as std::ffi::c_int as u32;
        }
        hashIdx = hashIdx.wrapping_add(1);
        hashIdx;
    }
    hashIdx = ((1 as std::ffi::c_int) << hashLog) as u32;
    while hashIdx != 0 {
        hashIdx = hashIdx.wrapping_sub(1);
        let bucketIdx = hashIdx << ZSTD_LAZY_DDSS_BUCKET_LOG;
        let chainPackedPointer = *tmpHashTable.offset(hashIdx as isize);
        let mut i_0: u32 = 0;
        i_0 = 0 as std::ffi::c_int as u32;
        while i_0 < cacheSize {
            *hashTable.offset(bucketIdx.wrapping_add(i_0) as isize) = 0 as std::ffi::c_int as u32;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        *hashTable.offset(
            bucketIdx
                .wrapping_add(bucketSize)
                .wrapping_sub(1 as std::ffi::c_int as u32) as isize,
        ) = chainPackedPointer;
    }
    idx = (*ms).nextToUpdate;
    while idx < target {
        let h_0 = (ZSTD_hashPtr(
            base.offset(idx as isize) as *const std::ffi::c_void,
            hashLog,
            (*ms).cParams.minMatch,
        ) as u32)
            << ZSTD_LAZY_DDSS_BUCKET_LOG;
        let mut i_1: u32 = 0;
        i_1 = cacheSize.wrapping_sub(1 as std::ffi::c_int as u32);
        while i_1 != 0 {
            *hashTable.offset(h_0.wrapping_add(i_1) as isize) = *hashTable.offset(
                h_0.wrapping_add(i_1)
                    .wrapping_sub(1 as std::ffi::c_int as u32) as isize,
            );
            i_1 = i_1.wrapping_sub(1);
            i_1;
        }
        *hashTable.offset(h_0 as isize) = idx;
        idx = idx.wrapping_add(1);
        idx;
    }
    (*ms).nextToUpdate = target;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_dedicatedDictSearch_lazy_search(
    mut offsetPtr: *mut size_t,
    mut ml: size_t,
    mut nbAttempts: u32,
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
    let ddsSize = ddsEnd.offset_from(ddsBase) as std::ffi::c_long as u32;
    let ddsIndexDelta = dictLimit.wrapping_sub(ddsSize);
    let bucketSize = ((1 as std::ffi::c_int) << ZSTD_LAZY_DDSS_BUCKET_LOG) as u32;
    let bucketLimit = if nbAttempts < bucketSize.wrapping_sub(1 as std::ffi::c_int as u32) {
        nbAttempts
    } else {
        bucketSize.wrapping_sub(1 as std::ffi::c_int as u32)
    };
    let mut ddsAttempt: u32 = 0;
    let mut matchIndex: u32 = 0;
    ddsAttempt = 0 as std::ffi::c_int as u32;
    while ddsAttempt < bucketSize.wrapping_sub(1 as std::ffi::c_int as u32) {
        ddsAttempt = ddsAttempt.wrapping_add(1);
        ddsAttempt;
    }
    let chainPackedPointer = *((*dms).hashTable).offset(
        ddsIdx
            .wrapping_add(bucketSize as size_t)
            .wrapping_sub(1 as std::ffi::c_int as size_t) as isize,
    );
    let chainIndex = chainPackedPointer >> 8 as std::ffi::c_int;
    ((*dms).chainTable).offset(chainIndex as isize);
    ddsAttempt = 0 as std::ffi::c_int as u32;
    while ddsAttempt < bucketLimit {
        let mut currentMl = 0 as std::ffi::c_int as size_t;
        let mut match_0 = std::ptr::null::<u8>();
        matchIndex = *((*dms).hashTable).offset(ddsIdx.wrapping_add(ddsAttempt as size_t) as isize);
        match_0 = ddsBase.offset(matchIndex as isize);
        if matchIndex == 0 {
            return ml;
        }
        if MEM_read32(match_0 as *const std::ffi::c_void)
            == MEM_read32(ip as *const std::ffi::c_void)
        {
            currentMl = (ZSTD_count_2segments(
                ip.offset(4 as std::ffi::c_int as isize),
                match_0.offset(4 as std::ffi::c_int as isize),
                iLimit,
                ddsEnd,
                prefixStart,
            ))
            .wrapping_add(4 as std::ffi::c_int as size_t);
        }
        if currentMl > ml {
            ml = currentMl;
            *offsetPtr = curr
                .wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.offset(currentMl as isize) == iLimit {
                return ml;
            }
        }
        ddsAttempt = ddsAttempt.wrapping_add(1);
        ddsAttempt;
    }
    let chainPackedPointer_0 = *((*dms).hashTable).offset(
        ddsIdx
            .wrapping_add(bucketSize as size_t)
            .wrapping_sub(1 as std::ffi::c_int as size_t) as isize,
    );
    let mut chainIndex_0 = chainPackedPointer_0 >> 8 as std::ffi::c_int;
    let chainLength = chainPackedPointer_0 & 0xff as std::ffi::c_int as u32;
    let chainAttempts = nbAttempts.wrapping_sub(ddsAttempt);
    let chainLimit = if chainAttempts > chainLength {
        chainLength
    } else {
        chainAttempts
    };
    let mut chainAttempt: u32 = 0;
    chainAttempt = 0 as std::ffi::c_int as u32;
    while chainAttempt < chainLimit {
        chainAttempt = chainAttempt.wrapping_add(1);
        chainAttempt;
    }
    chainAttempt = 0 as std::ffi::c_int as u32;
    while chainAttempt < chainLimit {
        let mut currentMl_0 = 0 as std::ffi::c_int as size_t;
        let mut match_1 = std::ptr::null::<u8>();
        matchIndex = *((*dms).chainTable).offset(chainIndex_0 as isize);
        match_1 = ddsBase.offset(matchIndex as isize);
        if MEM_read32(match_1 as *const std::ffi::c_void)
            == MEM_read32(ip as *const std::ffi::c_void)
        {
            currentMl_0 = (ZSTD_count_2segments(
                ip.offset(4 as std::ffi::c_int as isize),
                match_1.offset(4 as std::ffi::c_int as isize),
                iLimit,
                ddsEnd,
                prefixStart,
            ))
            .wrapping_add(4 as std::ffi::c_int as size_t);
        }
        if currentMl_0 > ml {
            ml = currentMl_0;
            *offsetPtr = curr
                .wrapping_sub(matchIndex.wrapping_add(ddsIndexDelta))
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.offset(currentMl_0 as isize) == iLimit {
                break;
            }
        }
        chainAttempt = chainAttempt.wrapping_add(1);
        chainAttempt;
        chainIndex_0 = chainIndex_0.wrapping_add(1);
        chainIndex_0;
    }
    ml
}
#[inline(always)]
unsafe extern "C" fn ZSTD_insertAndFindFirstIndex_internal(
    mut ms: *mut ZSTD_MatchState_t,
    cParams: *const ZSTD_compressionParameters,
    mut ip: *const u8,
    mls: u32,
    lazySkipping: u32,
) -> u32 {
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let chainTable = (*ms).chainTable;
    let chainMask = (((1 as std::ffi::c_int) << (*cParams).chainLog) - 1 as std::ffi::c_int) as u32;
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as std::ffi::c_long as u32;
    let mut idx = (*ms).nextToUpdate;
    while idx < target {
        let h = ZSTD_hashPtr(
            base.offset(idx as isize) as *const std::ffi::c_void,
            hashLog,
            mls,
        );
        *chainTable.offset((idx & chainMask) as isize) = *hashTable.offset(h as isize);
        *hashTable.offset(h as isize) = idx;
        idx = idx.wrapping_add(1);
        idx;
        if lazySkipping != 0 {
            break;
        }
    }
    (*ms).nextToUpdate = target;
    *hashTable.offset(ZSTD_hashPtr(ip as *const std::ffi::c_void, hashLog, mls) as isize)
}
#[export_name = crate::prefix!(ZSTD_insertAndFindFirstIndex)]
pub unsafe extern "C" fn ZSTD_insertAndFindFirstIndex(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
) -> u32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    ZSTD_insertAndFindFirstIndex_internal(
        ms,
        cParams,
        ip,
        (*ms).cParams.minMatch,
        0 as std::ffi::c_int as u32,
    )
}
#[inline(always)]
unsafe extern "C" fn ZSTD_HcFindBestMatch(
    mut ms: *mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let chainTable = (*ms).chainTable;
    let chainSize = ((1 as std::ffi::c_int) << (*cParams).chainLog) as u32;
    let chainMask = chainSize.wrapping_sub(1 as std::ffi::c_int as u32);
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictEnd = dictBase.offset(dictLimit as isize);
    let curr = ip.offset_from(base) as std::ffi::c_long as u32;
    let maxDistance = (1 as std::ffi::c_uint) << (*cParams).windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinMaxDistance = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary =
        ((*ms).loadedDictEnd != 0 as std::ffi::c_int as u32) as std::ffi::c_int as u32;
    let lowLimit = if isDictionary != 0 {
        lowestValid
    } else {
        withinMaxDistance
    };
    let minChain = if curr > chainSize {
        curr.wrapping_sub(chainSize)
    } else {
        0 as std::ffi::c_int as u32
    };
    let mut nbAttempts = (1 as std::ffi::c_uint) << (*cParams).searchLog;
    let mut ml = (4 as std::ffi::c_int - 1 as std::ffi::c_int) as size_t;
    let dms = (*ms).dictMatchState;
    let ddsHashLog = if dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint
    {
        ((*dms).cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as std::ffi::c_uint)
    } else {
        0 as std::ffi::c_int as std::ffi::c_uint
    };
    let ddsIdx = if dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint
    {
        ZSTD_hashPtr(ip as *const std::ffi::c_void, ddsHashLog, mls) << ZSTD_LAZY_DDSS_BUCKET_LOG
    } else {
        0 as std::ffi::c_int as size_t
    };
    let mut matchIndex: u32 = 0;
    if dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint
    {
        let mut entry: *const u32 = &mut *((*dms).hashTable).offset(ddsIdx as isize) as *mut u32;
    }
    matchIndex =
        ZSTD_insertAndFindFirstIndex_internal(ms, cParams, ip, mls, (*ms).lazySkipping as u32);
    while (matchIndex >= lowLimit) as std::ffi::c_int
        & (nbAttempts > 0 as std::ffi::c_int as u32) as std::ffi::c_int
        != 0
    {
        let mut currentMl = 0 as std::ffi::c_int as size_t;
        if dictMode as std::ffi::c_uint != ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint
            || matchIndex >= dictLimit
        {
            let match_0 = base.offset(matchIndex as isize);
            if MEM_read32(
                match_0
                    .offset(ml as isize)
                    .offset(-(3 as std::ffi::c_int as isize))
                    as *const std::ffi::c_void,
            ) == MEM_read32(
                ip.offset(ml as isize)
                    .offset(-(3 as std::ffi::c_int as isize))
                    as *const std::ffi::c_void,
            ) {
                currentMl = ZSTD_count(ip, match_0, iLimit);
            }
        } else {
            let match_1 = dictBase.offset(matchIndex as isize);
            if MEM_read32(match_1 as *const std::ffi::c_void)
                == MEM_read32(ip as *const std::ffi::c_void)
            {
                currentMl = (ZSTD_count_2segments(
                    ip.offset(4 as std::ffi::c_int as isize),
                    match_1.offset(4 as std::ffi::c_int as isize),
                    iLimit,
                    dictEnd,
                    prefixStart,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
            }
        }
        if currentMl > ml {
            ml = currentMl;
            *offsetPtr = curr
                .wrapping_sub(matchIndex)
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.offset(currentMl as isize) == iLimit {
                break;
            }
        }
        if matchIndex <= minChain {
            break;
        }
        matchIndex = *chainTable.offset((matchIndex & chainMask) as isize);
        nbAttempts = nbAttempts.wrapping_sub(1);
        nbAttempts;
    }
    if dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint
    {
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
    } else if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        let dmsChainTable: *const u32 = (*dms).chainTable;
        let dmsChainSize = ((1 as std::ffi::c_int) << (*dms).cParams.chainLog) as u32;
        let dmsChainMask = dmsChainSize.wrapping_sub(1 as std::ffi::c_int as u32);
        let dmsLowestIndex = (*dms).window.dictLimit;
        let dmsBase = (*dms).window.base;
        let dmsEnd = (*dms).window.nextSrc;
        let dmsSize = dmsEnd.offset_from(dmsBase) as std::ffi::c_long as u32;
        let dmsIndexDelta = dictLimit.wrapping_sub(dmsSize);
        let dmsMinChain = if dmsSize > dmsChainSize {
            dmsSize.wrapping_sub(dmsChainSize)
        } else {
            0 as std::ffi::c_int as u32
        };
        matchIndex = *((*dms).hashTable).offset(ZSTD_hashPtr(
            ip as *const std::ffi::c_void,
            (*dms).cParams.hashLog,
            mls,
        ) as isize);
        while (matchIndex >= dmsLowestIndex) as std::ffi::c_int
            & (nbAttempts > 0 as std::ffi::c_int as u32) as std::ffi::c_int
            != 0
        {
            let mut currentMl_0 = 0 as std::ffi::c_int as size_t;
            let match_2 = dmsBase.offset(matchIndex as isize);
            if MEM_read32(match_2 as *const std::ffi::c_void)
                == MEM_read32(ip as *const std::ffi::c_void)
            {
                currentMl_0 = (ZSTD_count_2segments(
                    ip.offset(4 as std::ffi::c_int as isize),
                    match_2.offset(4 as std::ffi::c_int as isize),
                    iLimit,
                    dmsEnd,
                    prefixStart,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
            }
            if currentMl_0 > ml {
                ml = currentMl_0;
                *offsetPtr = curr
                    .wrapping_sub(matchIndex.wrapping_add(dmsIndexDelta))
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
                if ip.offset(currentMl_0 as isize) == iLimit {
                    break;
                }
            }
            if matchIndex <= dmsMinChain {
                break;
            }
            matchIndex = *dmsChainTable.offset((matchIndex & dmsChainMask) as isize);
            nbAttempts = nbAttempts.wrapping_sub(1);
            nbAttempts;
        }
    }
    ml
}
pub const ZSTD_ROW_HASH_TAG_MASK: std::ffi::c_uint = ((1 as std::ffi::c_uint)
    << ZSTD_ROW_HASH_TAG_BITS)
    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
pub const ZSTD_ROW_HASH_CACHE_MASK: std::ffi::c_int =
    ZSTD_ROW_HASH_CACHE_SIZE - 1 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn ZSTD_VecMask_next(mut val: ZSTD_VecMask) -> u32 {
    ZSTD_countTrailingZeros64(val)
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_nextIndex(tagRow: *mut u8, rowMask: u32) -> u32 {
    let mut next = (*tagRow as std::ffi::c_int - 1 as std::ffi::c_int) as u32 & rowMask;
    next = next.wrapping_add(if next == 0 as std::ffi::c_int as u32 {
        rowMask
    } else {
        0 as std::ffi::c_int as u32
    });
    *tagRow = next as u8;
    next
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_prefetch(
    mut hashTable: *const u32,
    mut tagTable: *const u8,
    relRow: u32,
    rowLog: u32,
) {
    rowLog >= 5 as std::ffi::c_int as u32;
    rowLog == 6 as std::ffi::c_int as u32;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_fillHashCache(
    mut ms: *mut ZSTD_MatchState_t,
    mut base: *const u8,
    rowLog: u32,
    mls: u32,
    mut idx: u32,
    iLimit: *const u8,
) {
    let hashTable: *const u32 = (*ms).hashTable;
    let tagTable: *const u8 = (*ms).tagTable;
    let hashLog = (*ms).rowHashLog;
    let maxElemsToPrefetch = if base.offset(idx as isize) > iLimit {
        0 as std::ffi::c_int as u32
    } else {
        (iLimit.offset_from(base.offset(idx as isize)) as std::ffi::c_long
            + 1 as std::ffi::c_int as std::ffi::c_long) as u32
    };
    let lim = idx.wrapping_add(if (8 as std::ffi::c_int as u32) < maxElemsToPrefetch {
        8 as std::ffi::c_int as u32
    } else {
        maxElemsToPrefetch
    });
    while idx < lim {
        let hash = ZSTD_hashPtrSalted(
            base.offset(idx as isize) as *const std::ffi::c_void,
            hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
            mls,
            (*ms).hashSalt,
        ) as u32;
        let row = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        ZSTD_row_prefetch(hashTable, tagTable, row, rowLog);
        *((*ms).hashCache)
            .as_mut_ptr()
            .offset((idx & ZSTD_ROW_HASH_CACHE_MASK as u32) as isize) = hash;
        idx = idx.wrapping_add(1);
        idx;
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_nextCachedHash(
    mut cache: *mut u32,
    mut hashTable: *const u32,
    mut tagTable: *const u8,
    mut base: *const u8,
    mut idx: u32,
    hashLog: u32,
    rowLog: u32,
    mls: u32,
    hashSalt: u64,
) -> u32 {
    let newHash = ZSTD_hashPtrSalted(
        base.offset(idx as isize)
            .offset(ZSTD_ROW_HASH_CACHE_SIZE as isize) as *const std::ffi::c_void,
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
#[inline(always)]
unsafe extern "C" fn ZSTD_row_update_internalImpl(
    mut ms: *mut ZSTD_MatchState_t,
    mut updateStartIdx: u32,
    updateEndIdx: u32,
    mls: u32,
    rowLog: u32,
    rowMask: u32,
    useCache: u32,
) {
    let hashTable = (*ms).hashTable;
    let tagTable = (*ms).tagTable;
    let hashLog = (*ms).rowHashLog;
    let base = (*ms).window.base;
    while updateStartIdx < updateEndIdx {
        let hash = if useCache != 0 {
            ZSTD_row_nextCachedHash(
                ((*ms).hashCache).as_mut_ptr(),
                hashTable,
                tagTable,
                base,
                updateStartIdx,
                hashLog,
                rowLog,
                mls,
                (*ms).hashSalt,
            )
        } else {
            ZSTD_hashPtrSalted(
                base.offset(updateStartIdx as isize) as *const std::ffi::c_void,
                hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
                mls,
                (*ms).hashSalt,
            ) as u32
        };
        let relRow = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        let row = hashTable.offset(relRow as isize);
        let mut tagRow = tagTable.offset(relRow as isize);
        let pos = ZSTD_row_nextIndex(tagRow, rowMask);
        *tagRow.offset(pos as isize) = (hash & ZSTD_ROW_HASH_TAG_MASK) as u8;
        *row.offset(pos as isize) = updateStartIdx;
        updateStartIdx = updateStartIdx.wrapping_add(1);
        updateStartIdx;
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_update_internal(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    mls: u32,
    rowLog: u32,
    rowMask: u32,
    useCache: u32,
) {
    let mut idx = (*ms).nextToUpdate;
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as std::ffi::c_long as u32;
    let kSkipThreshold = 384 as std::ffi::c_int as u32;
    let kMaxMatchStartPositionsToUpdate = 96 as std::ffi::c_int as u32;
    let kMaxMatchEndPositionsToUpdate = 32 as std::ffi::c_int as u32;
    if useCache != 0
        && (target.wrapping_sub(idx) > kSkipThreshold) as std::ffi::c_int as std::ffi::c_long != 0
    {
        let bound = idx.wrapping_add(kMaxMatchStartPositionsToUpdate);
        ZSTD_row_update_internalImpl(ms, idx, bound, mls, rowLog, rowMask, useCache);
        idx = target.wrapping_sub(kMaxMatchEndPositionsToUpdate);
        ZSTD_row_fillHashCache(
            ms,
            base,
            rowLog,
            mls,
            idx,
            ip.offset(1 as std::ffi::c_int as isize),
        );
    }
    ZSTD_row_update_internalImpl(ms, idx, target, mls, rowLog, rowMask, useCache);
    (*ms).nextToUpdate = target;
}
#[export_name = crate::prefix!(ZSTD_row_update)]
pub unsafe extern "C" fn ZSTD_row_update(ms: *mut ZSTD_MatchState_t, mut ip: *const u8) {
    let rowLog = if 4 as std::ffi::c_int as std::ffi::c_uint
        > (if (*ms).cParams.searchLog < 6 as std::ffi::c_int as std::ffi::c_uint {
            (*ms).cParams.searchLog
        } else {
            6 as std::ffi::c_int as std::ffi::c_uint
        }) {
        4 as std::ffi::c_int as std::ffi::c_uint
    } else if (*ms).cParams.searchLog < 6 as std::ffi::c_int as std::ffi::c_uint {
        (*ms).cParams.searchLog
    } else {
        6 as std::ffi::c_int as std::ffi::c_uint
    };
    let rowMask =
        ((1 as std::ffi::c_uint) << rowLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let mls = if (*ms).cParams.minMatch < 6 as std::ffi::c_int as std::ffi::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as std::ffi::c_int as std::ffi::c_uint
    };
    ZSTD_row_update_internal(ms, ip, mls, rowLog, rowMask, 0 as std::ffi::c_int as u32);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_matchMaskGroupWidth(rowEntries: u32) -> u32 {
    1 as std::ffi::c_int as u32
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_getSSEMask(
    mut nbChunks: std::ffi::c_int,
    src: *const u8,
    tag: u8,
    head: u32,
) -> ZSTD_VecMask {
    let comparisonMask = _mm_set1_epi8(tag as std::ffi::c_char);
    let mut matches: [std::ffi::c_int; 4] = [0 as std::ffi::c_int; 4];
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < nbChunks {
        let chunk = _mm_loadu_si128(src.offset((16 as std::ffi::c_int * i) as isize)
            as *const std::ffi::c_void as *const __m128i);
        let equalMask = _mm_cmpeq_epi8(chunk, comparisonMask);
        *matches.as_mut_ptr().offset(i as isize) = _mm_movemask_epi8(equalMask);
        i += 1;
        i;
    }
    if nbChunks == 1 as std::ffi::c_int {
        return ZSTD_rotateRight_U16(
            *matches.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as u16,
            head,
        ) as ZSTD_VecMask;
    }
    if nbChunks == 2 as std::ffi::c_int {
        return ZSTD_rotateRight_U32(
            (*matches.as_mut_ptr().offset(1 as std::ffi::c_int as isize) as u32)
                << 16 as std::ffi::c_int
                | *matches.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as u32,
            head,
        ) as ZSTD_VecMask;
    }
    ZSTD_rotateRight_U64(
        (*matches.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as u64)
            << 48 as std::ffi::c_int
            | (*matches.as_mut_ptr().offset(2 as std::ffi::c_int as isize) as u64)
                << 32 as std::ffi::c_int
            | (*matches.as_mut_ptr().offset(1 as std::ffi::c_int as isize) as u64)
                << 16 as std::ffi::c_int
            | *matches.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as u64,
        head,
    )
}
#[inline(always)]
unsafe extern "C" fn ZSTD_row_getMatchMask(
    tagRow: *const u8,
    tag: u8,
    headGrouped: u32,
    rowEntries: u32,
) -> ZSTD_VecMask {
    let src = tagRow;
    ZSTD_row_getSSEMask(
        (rowEntries / 16 as std::ffi::c_int as u32) as std::ffi::c_int,
        src,
        tag,
        headGrouped,
    )
}
#[inline(always)]
unsafe extern "C" fn ZSTD_RowFindBestMatch(
    mut ms: *mut ZSTD_MatchState_t,
    ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
    mls: u32,
    dictMode: ZSTD_dictMode_e,
    rowLog: u32,
) -> size_t {
    let hashTable = (*ms).hashTable;
    let tagTable = (*ms).tagTable;
    let hashCache = ((*ms).hashCache).as_mut_ptr();
    let hashLog = (*ms).rowHashLog;
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictEnd = dictBase.offset(dictLimit as isize);
    let curr = ip.offset_from(base) as std::ffi::c_long as u32;
    let maxDistance = (1 as std::ffi::c_uint) << (*cParams).windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinMaxDistance = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary =
        ((*ms).loadedDictEnd != 0 as std::ffi::c_int as u32) as std::ffi::c_int as u32;
    let lowLimit = if isDictionary != 0 {
        lowestValid
    } else {
        withinMaxDistance
    };
    let rowEntries = (1 as std::ffi::c_uint) << rowLog;
    let rowMask = rowEntries.wrapping_sub(1 as std::ffi::c_int as u32);
    let cappedSearchLog = if (*cParams).searchLog < rowLog {
        (*cParams).searchLog
    } else {
        rowLog
    };
    let groupWidth = ZSTD_row_matchMaskGroupWidth(rowEntries);
    let hashSalt = (*ms).hashSalt;
    let mut nbAttempts = (1 as std::ffi::c_uint) << cappedSearchLog;
    let mut ml = (4 as std::ffi::c_int - 1 as std::ffi::c_int) as size_t;
    let mut hash: u32 = 0;
    let dms = (*ms).dictMatchState;
    let mut ddsIdx = 0 as std::ffi::c_int as size_t;
    let mut ddsExtraAttempts = 0 as std::ffi::c_int as u32;
    let mut dmsTag = 0 as std::ffi::c_int as u32;
    let mut dmsRow = NULL as *mut u32;
    let mut dmsTagRow = NULL as *mut u8;
    if dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint
    {
        let ddsHashLog =
            ((*dms).cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as std::ffi::c_uint);
        ddsIdx = ZSTD_hashPtr(ip as *const std::ffi::c_void, ddsHashLog, mls)
            << ZSTD_LAZY_DDSS_BUCKET_LOG;
        ((*dms).hashTable).offset(ddsIdx as isize);
        ddsExtraAttempts = if (*cParams).searchLog > rowLog {
            (1 as std::ffi::c_uint) << ((*cParams).searchLog).wrapping_sub(rowLog)
        } else {
            0 as std::ffi::c_int as std::ffi::c_uint
        };
    }
    if dictMode as std::ffi::c_uint == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint {
        let dmsHashTable = (*dms).hashTable;
        let dmsTagTable = (*dms).tagTable;
        let dmsHash = ZSTD_hashPtr(
            ip as *const std::ffi::c_void,
            ((*dms).rowHashLog).wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
            mls,
        ) as u32;
        let dmsRelRow = dmsHash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
        dmsTag = dmsHash & ZSTD_ROW_HASH_TAG_MASK;
        dmsTagRow = dmsTagTable.offset(dmsRelRow as isize);
        dmsRow = dmsHashTable.offset(dmsRelRow as isize);
        ZSTD_row_prefetch(dmsHashTable, dmsTagTable, dmsRelRow, rowLog);
    }
    if (*ms).lazySkipping == 0 {
        ZSTD_row_update_internal(ms, ip, mls, rowLog, rowMask, 1 as std::ffi::c_int as u32);
        hash = ZSTD_row_nextCachedHash(
            hashCache, hashTable, tagTable, base, curr, hashLog, rowLog, mls, hashSalt,
        );
    } else {
        hash = ZSTD_hashPtrSalted(
            ip as *const std::ffi::c_void,
            hashLog.wrapping_add(ZSTD_ROW_HASH_TAG_BITS as u32),
            mls,
            hashSalt,
        ) as u32;
        (*ms).nextToUpdate = curr;
    }
    (*ms).hashSaltEntropy = ((*ms).hashSaltEntropy).wrapping_add(hash);
    let relRow = hash >> ZSTD_ROW_HASH_TAG_BITS << rowLog;
    let tag = hash & ZSTD_ROW_HASH_TAG_MASK;
    let row = hashTable.offset(relRow as isize);
    let mut tagRow = tagTable.offset(relRow as isize);
    let headGrouped = (*tagRow as u32 & rowMask) * groupWidth;
    let mut matchBuffer: [u32; 64] = [0; 64];
    let mut numMatches = 0 as std::ffi::c_int as size_t;
    let mut currMatch = 0 as std::ffi::c_int as size_t;
    let mut matches = ZSTD_row_getMatchMask(tagRow, tag as u8, headGrouped, rowEntries);
    while matches > 0 as std::ffi::c_int as ZSTD_VecMask && nbAttempts > 0 as std::ffi::c_int as u32
    {
        let matchPos =
            (headGrouped.wrapping_add(ZSTD_VecMask_next(matches)) / groupWidth) & rowMask;
        let matchIndex = *row.offset(matchPos as isize);
        if matchPos != 0 as std::ffi::c_int as u32 {
            if matchIndex < lowLimit {
                break;
            }
            if dictMode as std::ffi::c_uint == ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint {
                matchIndex >= dictLimit;
            }
            let fresh3 = numMatches;
            numMatches = numMatches.wrapping_add(1);
            *matchBuffer.as_mut_ptr().offset(fresh3 as isize) = matchIndex;
            nbAttempts = nbAttempts.wrapping_sub(1);
            nbAttempts;
        }
        matches &= matches.wrapping_sub(1 as std::ffi::c_int as ZSTD_VecMask);
    }
    let pos = ZSTD_row_nextIndex(tagRow, rowMask);
    *tagRow.offset(pos as isize) = tag as u8;
    let fresh4 = (*ms).nextToUpdate;
    (*ms).nextToUpdate = ((*ms).nextToUpdate).wrapping_add(1);
    *row.offset(pos as isize) = fresh4;
    while currMatch < numMatches {
        let matchIndex_0 = *matchBuffer.as_mut_ptr().offset(currMatch as isize);
        let mut currentMl = 0 as std::ffi::c_int as size_t;
        if dictMode as std::ffi::c_uint != ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint
            || matchIndex_0 >= dictLimit
        {
            let match_0 = base.offset(matchIndex_0 as isize);
            if MEM_read32(
                match_0
                    .offset(ml as isize)
                    .offset(-(3 as std::ffi::c_int as isize))
                    as *const std::ffi::c_void,
            ) == MEM_read32(
                ip.offset(ml as isize)
                    .offset(-(3 as std::ffi::c_int as isize))
                    as *const std::ffi::c_void,
            ) {
                currentMl = ZSTD_count(ip, match_0, iLimit);
            }
        } else {
            let match_1 = dictBase.offset(matchIndex_0 as isize);
            if MEM_read32(match_1 as *const std::ffi::c_void)
                == MEM_read32(ip as *const std::ffi::c_void)
            {
                currentMl = (ZSTD_count_2segments(
                    ip.offset(4 as std::ffi::c_int as isize),
                    match_1.offset(4 as std::ffi::c_int as isize),
                    iLimit,
                    dictEnd,
                    prefixStart,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
            }
        }
        if currentMl > ml {
            ml = currentMl;
            *offsetPtr = curr
                .wrapping_sub(matchIndex_0)
                .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
            if ip.offset(currentMl as isize) == iLimit {
                break;
            }
        }
        currMatch = currMatch.wrapping_add(1);
        currMatch;
    }
    if dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint
    {
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
    } else if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        let dmsLowestIndex = (*dms).window.dictLimit;
        let dmsBase = (*dms).window.base;
        let dmsEnd = (*dms).window.nextSrc;
        let dmsSize = dmsEnd.offset_from(dmsBase) as std::ffi::c_long as u32;
        let dmsIndexDelta = dictLimit.wrapping_sub(dmsSize);
        let headGrouped_0 = (*dmsTagRow as u32 & rowMask) * groupWidth;
        let mut matchBuffer_0: [u32; 64] = [0; 64];
        let mut numMatches_0 = 0 as std::ffi::c_int as size_t;
        let mut currMatch_0 = 0 as std::ffi::c_int as size_t;
        let mut matches_0 =
            ZSTD_row_getMatchMask(dmsTagRow, dmsTag as u8, headGrouped_0, rowEntries);
        while matches_0 > 0 as std::ffi::c_int as ZSTD_VecMask
            && nbAttempts > 0 as std::ffi::c_int as u32
        {
            let matchPos_0 =
                (headGrouped_0.wrapping_add(ZSTD_VecMask_next(matches_0)) / groupWidth) & rowMask;
            let matchIndex_1 = *dmsRow.offset(matchPos_0 as isize);
            if matchPos_0 != 0 as std::ffi::c_int as u32 {
                if matchIndex_1 < dmsLowestIndex {
                    break;
                }
                let fresh5 = numMatches_0;
                numMatches_0 = numMatches_0.wrapping_add(1);
                *matchBuffer_0.as_mut_ptr().offset(fresh5 as isize) = matchIndex_1;
                nbAttempts = nbAttempts.wrapping_sub(1);
                nbAttempts;
            }
            matches_0 &= matches_0.wrapping_sub(1 as std::ffi::c_int as ZSTD_VecMask);
        }
        while currMatch_0 < numMatches_0 {
            let matchIndex_2 = *matchBuffer_0.as_mut_ptr().offset(currMatch_0 as isize);
            let mut currentMl_0 = 0 as std::ffi::c_int as size_t;
            let match_2 = dmsBase.offset(matchIndex_2 as isize);
            if MEM_read32(match_2 as *const std::ffi::c_void)
                == MEM_read32(ip as *const std::ffi::c_void)
            {
                currentMl_0 = (ZSTD_count_2segments(
                    ip.offset(4 as std::ffi::c_int as isize),
                    match_2.offset(4 as std::ffi::c_int as isize),
                    iLimit,
                    dmsEnd,
                    prefixStart,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
            }
            if currentMl_0 > ml {
                ml = currentMl_0;
                *offsetPtr = curr
                    .wrapping_sub(matchIndex_2.wrapping_add(dmsIndexDelta))
                    .wrapping_add(ZSTD_REP_NUM as u32) as size_t;
                if ip.offset(currentMl_0 as isize) == iLimit {
                    break;
                }
            }
            currMatch_0 = currMatch_0.wrapping_add(1);
            currMatch_0;
        }
    }
    ml
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_6_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_4_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_extDict,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_4_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_6_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_extDict,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_6_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_extDict,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_6_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_extDict,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_5_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_extDict,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_5_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_extDict,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_5_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_extDict,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_4_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_4_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_extDict,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_extDict_4_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_extDict,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_6_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_5_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_6_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_noDict,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_6_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_noDict,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_5_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_noDict,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_5_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_noDict,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_5_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_noDict,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_4_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_noDict,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_4_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_noDict,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_4_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_noDict,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dedicatedDictSearch_4_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_4_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_6_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_6_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_5_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        6 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_5_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_dictMatchState_5_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
        4 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_RowFindBestMatch_noDict_6_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_RowFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_noDict,
        5 as std::ffi::c_int as u32,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_noDict_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dictMatchState_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_noDict_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dedicatedDictSearch_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dedicatedDictSearch_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dedicatedDictSearch_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_extDict_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as std::ffi::c_int as u32,
        ZSTD_extDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dictMatchState_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_extDict_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        6 as std::ffi::c_int as u32,
        ZSTD_extDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_noDict_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        4 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_extDict_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as std::ffi::c_int as u32,
        ZSTD_extDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_BtFindBestMatch_dictMatchState_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offBasePtr: *mut size_t,
) -> size_t {
    ZSTD_BtFindBestMatch(
        ms,
        ip,
        iLimit,
        offBasePtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_noDict_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dictMatchState_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dedicatedDictSearch_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dictMatchState_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dedicatedDictSearch_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dedicatedDictSearch_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_noDict_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_dictMatchState_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_noDict_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_extDict_6(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        6 as std::ffi::c_int as u32,
        ZSTD_extDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_extDict_5(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        5 as std::ffi::c_int as u32,
        ZSTD_extDict,
    )
}
#[inline(never)]
unsafe extern "C" fn ZSTD_HcFindBestMatch_extDict_4(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    iLimit: *const u8,
    mut offsetPtr: *mut size_t,
) -> size_t {
    ZSTD_HcFindBestMatch(
        ms,
        ip,
        iLimit,
        offsetPtr,
        4 as std::ffi::c_int as u32,
        ZSTD_extDict,
    )
}
#[inline(always)]
unsafe extern "C" fn ZSTD_searchMax(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const u8,
    mut iend: *const u8,
    mut offsetPtr: *mut size_t,
    mls: u32,
    rowLog: u32,
    searchMethod: searchMethod_e,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    if dictMode as std::ffi::c_uint == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint {
        match searchMethod as std::ffi::c_uint {
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
    } else if dictMode as std::ffi::c_uint == ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint {
        match searchMethod as std::ffi::c_uint {
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
    } else if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        match searchMethod as std::ffi::c_uint {
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
    } else if dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint
    {
        match searchMethod as std::ffi::c_uint {
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
    0 as std::ffi::c_int as size_t
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_lazy_generic(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    searchMethod: searchMethod_e,
    depth: u32,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let mut current_block: u64;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.offset(srcSize as isize);
    let ilimit = if searchMethod as std::ffi::c_uint
        == search_rowHash as std::ffi::c_int as std::ffi::c_uint
    {
        iend.offset(-(8 as std::ffi::c_int as isize))
            .offset(-(ZSTD_ROW_HASH_CACHE_SIZE as isize))
    } else {
        iend.offset(-(8 as std::ffi::c_int as isize))
    };
    let base = (*ms).window.base;
    let prefixLowestIndex = (*ms).window.dictLimit;
    let prefixLowest = base.offset(prefixLowestIndex as isize);
    let mls = if 4 as std::ffi::c_int as std::ffi::c_uint
        > (if (*ms).cParams.minMatch < 6 as std::ffi::c_int as std::ffi::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as std::ffi::c_int as std::ffi::c_uint
        }) {
        4 as std::ffi::c_int as std::ffi::c_uint
    } else if (*ms).cParams.minMatch < 6 as std::ffi::c_int as std::ffi::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as std::ffi::c_int as std::ffi::c_uint
    };
    let rowLog = if 4 as std::ffi::c_int as std::ffi::c_uint
        > (if (*ms).cParams.searchLog < 6 as std::ffi::c_int as std::ffi::c_uint {
            (*ms).cParams.searchLog
        } else {
            6 as std::ffi::c_int as std::ffi::c_uint
        }) {
        4 as std::ffi::c_int as std::ffi::c_uint
    } else if (*ms).cParams.searchLog < 6 as std::ffi::c_int as std::ffi::c_uint {
        (*ms).cParams.searchLog
    } else {
        6 as std::ffi::c_int as std::ffi::c_uint
    };
    let mut offset_1 = *rep.offset(0 as std::ffi::c_int as isize);
    let mut offset_2 = *rep.offset(1 as std::ffi::c_int as isize);
    let mut offsetSaved1 = 0 as std::ffi::c_int as u32;
    let mut offsetSaved2 = 0 as std::ffi::c_int as u32;
    let isDMS = (dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int;
    let isDDS = (dictMode as std::ffi::c_uint
        == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int;
    let isDxS = (isDMS != 0 || isDDS != 0) as std::ffi::c_int;
    let dms = (*ms).dictMatchState;
    let dictLowestIndex = if isDxS != 0 {
        (*dms).window.dictLimit
    } else {
        0 as std::ffi::c_int as u32
    };
    let dictBase = if isDxS != 0 {
        (*dms).window.base
    } else {
        NULL as *const u8
    };
    let dictLowest = if isDxS != 0 {
        dictBase.offset(dictLowestIndex as isize)
    } else {
        NULL as *const u8
    };
    let dictEnd = if isDxS != 0 {
        (*dms).window.nextSrc
    } else {
        NULL as *const u8
    };
    let dictIndexDelta = if isDxS != 0 {
        prefixLowestIndex.wrapping_sub(dictEnd.offset_from(dictBase) as std::ffi::c_long as u32)
    } else {
        0 as std::ffi::c_int as u32
    };
    let dictAndPrefixLength = (ip.offset_from(prefixLowest) as std::ffi::c_long
        + dictEnd.offset_from(dictLowest) as std::ffi::c_long) as u32;
    ip =
        ip.offset((dictAndPrefixLength == 0 as std::ffi::c_int as u32) as std::ffi::c_int as isize);
    if dictMode as std::ffi::c_uint == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint {
        let curr = ip.offset_from(base) as std::ffi::c_long as u32;
        let windowLow = ZSTD_getLowestPrefixIndex(ms, curr, (*ms).cParams.windowLog);
        let maxRep = curr.wrapping_sub(windowLow);
        if offset_2 > maxRep {
            offsetSaved2 = offset_2;
            offset_2 = 0 as std::ffi::c_int as u32;
        }
        if offset_1 > maxRep {
            offsetSaved1 = offset_1;
            offset_1 = 0 as std::ffi::c_int as u32;
        }
    }
    isDxS != 0;
    (*ms).lazySkipping = 0 as std::ffi::c_int;
    if searchMethod as std::ffi::c_uint == search_rowHash as std::ffi::c_int as std::ffi::c_uint {
        ZSTD_row_fillHashCache(ms, base, rowLog, mls, (*ms).nextToUpdate, ilimit);
    }
    asm!(".p2align 5", options(preserves_flags, att_syntax));
    while ip < ilimit {
        let mut matchLength = 0 as std::ffi::c_int as size_t;
        let mut offBase = REPCODE1_TO_OFFBASE as size_t;
        let mut start = ip.offset(1 as std::ffi::c_int as isize);
        if isDxS != 0 {
            let repIndex = (ip.offset_from(base) as std::ffi::c_long as u32)
                .wrapping_add(1 as std::ffi::c_int as u32)
                .wrapping_sub(offset_1);
            let mut repMatch = if (dictMode as std::ffi::c_uint
                == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
                || dictMode as std::ffi::c_uint
                    == ZSTD_dedicatedDictSearch as std::ffi::c_int as std::ffi::c_uint)
                && repIndex < prefixLowestIndex
            {
                dictBase.offset(repIndex.wrapping_sub(dictIndexDelta) as isize)
            } else {
                base.offset(repIndex as isize)
            };
            if ZSTD_index_overlap_check(prefixLowestIndex, repIndex) != 0
                && MEM_read32(repMatch as *const std::ffi::c_void)
                    == MEM_read32(
                        ip.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void
                    )
            {
                let mut repMatchEnd = if repIndex < prefixLowestIndex {
                    dictEnd
                } else {
                    iend
                };
                matchLength = (ZSTD_count_2segments(
                    ip.offset(1 as std::ffi::c_int as isize)
                        .offset(4 as std::ffi::c_int as isize),
                    repMatch.offset(4 as std::ffi::c_int as isize),
                    iend,
                    repMatchEnd,
                    prefixLowest,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
                if depth == 0 as std::ffi::c_int as u32 {
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
            if dictMode as std::ffi::c_uint == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint
                && (offset_1 > 0 as std::ffi::c_int as u32) as std::ffi::c_int
                    & (MEM_read32(
                        ip.offset(1 as std::ffi::c_int as isize)
                            .offset(-(offset_1 as isize))
                            as *const std::ffi::c_void,
                    ) == MEM_read32(
                        ip.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void
                    )) as std::ffi::c_int
                    != 0
            {
                matchLength = (ZSTD_count(
                    ip.offset(1 as std::ffi::c_int as isize)
                        .offset(4 as std::ffi::c_int as isize),
                    ip.offset(1 as std::ffi::c_int as isize)
                        .offset(4 as std::ffi::c_int as isize)
                        .offset(-(offset_1 as isize)),
                    iend,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
                if depth == 0 as std::ffi::c_int as u32 {
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
                    let mut offbaseFound = 999999999 as std::ffi::c_int as size_t;
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
                    if matchLength < 4 as std::ffi::c_int as size_t {
                        let step = (ip.offset_from(anchor) as std::ffi::c_long as size_t
                            >> kSearchStrength)
                            .wrapping_add(1 as std::ffi::c_int as size_t);
                        ip = ip.offset(step as isize);
                        (*ms).lazySkipping =
                            (step > kLazySkippingStep as size_t) as std::ffi::c_int;
                        continue;
                    } else {
                        if depth >= 1 as std::ffi::c_int as u32 {
                            while ip < ilimit {
                                ip = ip.offset(1);
                                ip;
                                if dictMode as std::ffi::c_uint
                                    == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint
                                    && offBase != 0
                                    && (offset_1 > 0 as std::ffi::c_int as u32) as std::ffi::c_int
                                        & (MEM_read32(ip as *const std::ffi::c_void)
                                            == MEM_read32(ip.offset(-(offset_1 as isize))
                                                as *const std::ffi::c_void))
                                            as std::ffi::c_int
                                        != 0
                                {
                                    let mlRep = (ZSTD_count(
                                        ip.offset(4 as std::ffi::c_int as isize),
                                        ip.offset(4 as std::ffi::c_int as isize)
                                            .offset(-(offset_1 as isize)),
                                        iend,
                                    ))
                                    .wrapping_add(4 as std::ffi::c_int as size_t);
                                    let gain2 =
                                        (mlRep * 3 as std::ffi::c_int as size_t) as std::ffi::c_int;
                                    let gain1 = (matchLength * 3 as std::ffi::c_int as size_t)
                                        .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                        .wrapping_add(1 as std::ffi::c_int as size_t)
                                        as std::ffi::c_int;
                                    if mlRep >= 4 as std::ffi::c_int as size_t && gain2 > gain1 {
                                        matchLength = mlRep;
                                        offBase = REPCODE1_TO_OFFBASE as size_t;
                                        start = ip;
                                    }
                                }
                                if isDxS != 0 {
                                    let repIndex_0 = (ip.offset_from(base) as std::ffi::c_long
                                        as u32)
                                        .wrapping_sub(offset_1);
                                    let mut repMatch_0 = if repIndex_0 < prefixLowestIndex {
                                        dictBase
                                            .offset(repIndex_0.wrapping_sub(dictIndexDelta)
                                                as isize)
                                    } else {
                                        base.offset(repIndex_0 as isize)
                                    };
                                    if ZSTD_index_overlap_check(prefixLowestIndex, repIndex_0) != 0
                                        && MEM_read32(repMatch_0 as *const std::ffi::c_void)
                                            == MEM_read32(ip as *const std::ffi::c_void)
                                    {
                                        let mut repMatchEnd_0 = if repIndex_0 < prefixLowestIndex {
                                            dictEnd
                                        } else {
                                            iend
                                        };
                                        let mlRep_0 = (ZSTD_count_2segments(
                                            ip.offset(4 as std::ffi::c_int as isize),
                                            repMatch_0.offset(4 as std::ffi::c_int as isize),
                                            iend,
                                            repMatchEnd_0,
                                            prefixLowest,
                                        ))
                                        .wrapping_add(4 as std::ffi::c_int as size_t);
                                        let gain2_0 = (mlRep_0 * 3 as std::ffi::c_int as size_t)
                                            as std::ffi::c_int;
                                        let gain1_0 = (matchLength * 3 as std::ffi::c_int as size_t)
                                            .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                            .wrapping_add(1 as std::ffi::c_int as size_t)
                                            as std::ffi::c_int;
                                        if mlRep_0 >= 4 as std::ffi::c_int as size_t
                                            && gain2_0 > gain1_0
                                        {
                                            matchLength = mlRep_0;
                                            offBase = REPCODE1_TO_OFFBASE as size_t;
                                            start = ip;
                                        }
                                    }
                                }
                                let mut ofbCandidate = 999999999 as std::ffi::c_int as size_t;
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
                                let gain2_1 = (ml2_0 * 4 as std::ffi::c_int as size_t)
                                    .wrapping_sub(ZSTD_highbit32(ofbCandidate as u32) as size_t)
                                    as std::ffi::c_int;
                                let gain1_1 = (matchLength * 4 as std::ffi::c_int as size_t)
                                    .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                    .wrapping_add(4 as std::ffi::c_int as size_t)
                                    as std::ffi::c_int;
                                if ml2_0 >= 4 as std::ffi::c_int as size_t && gain2_1 > gain1_1 {
                                    matchLength = ml2_0;
                                    offBase = ofbCandidate;
                                    start = ip;
                                } else {
                                    if !(depth == 2 as std::ffi::c_int as u32 && ip < ilimit) {
                                        break;
                                    }
                                    ip = ip.offset(1);
                                    ip;
                                    if dictMode as std::ffi::c_uint
                                        == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint
                                        && offBase != 0
                                        && (offset_1 > 0 as std::ffi::c_int as u32)
                                            as std::ffi::c_int
                                            & (MEM_read32(ip as *const std::ffi::c_void)
                                                == MEM_read32(ip.offset(-(offset_1 as isize))
                                                    as *const std::ffi::c_void))
                                                as std::ffi::c_int
                                            != 0
                                    {
                                        let mlRep_1 = (ZSTD_count(
                                            ip.offset(4 as std::ffi::c_int as isize),
                                            ip.offset(4 as std::ffi::c_int as isize)
                                                .offset(-(offset_1 as isize)),
                                            iend,
                                        ))
                                        .wrapping_add(4 as std::ffi::c_int as size_t);
                                        let gain2_2 = (mlRep_1 * 4 as std::ffi::c_int as size_t)
                                            as std::ffi::c_int;
                                        let gain1_2 = (matchLength * 4 as std::ffi::c_int as size_t)
                                            .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                            .wrapping_add(1 as std::ffi::c_int as size_t)
                                            as std::ffi::c_int;
                                        if mlRep_1 >= 4 as std::ffi::c_int as size_t
                                            && gain2_2 > gain1_2
                                        {
                                            matchLength = mlRep_1;
                                            offBase = REPCODE1_TO_OFFBASE as size_t;
                                            start = ip;
                                        }
                                    }
                                    if isDxS != 0 {
                                        let repIndex_1 = (ip.offset_from(base) as std::ffi::c_long
                                            as u32)
                                            .wrapping_sub(offset_1);
                                        let mut repMatch_1 = if repIndex_1 < prefixLowestIndex {
                                            dictBase
                                                .offset(repIndex_1.wrapping_sub(dictIndexDelta)
                                                    as isize)
                                        } else {
                                            base.offset(repIndex_1 as isize)
                                        };
                                        if ZSTD_index_overlap_check(prefixLowestIndex, repIndex_1)
                                            != 0
                                            && MEM_read32(repMatch_1 as *const std::ffi::c_void)
                                                == MEM_read32(ip as *const std::ffi::c_void)
                                        {
                                            let mut repMatchEnd_1 =
                                                if repIndex_1 < prefixLowestIndex {
                                                    dictEnd
                                                } else {
                                                    iend
                                                };
                                            let mlRep_2 = (ZSTD_count_2segments(
                                                ip.offset(4 as std::ffi::c_int as isize),
                                                repMatch_1.offset(4 as std::ffi::c_int as isize),
                                                iend,
                                                repMatchEnd_1,
                                                prefixLowest,
                                            ))
                                            .wrapping_add(4 as std::ffi::c_int as size_t);
                                            let gain2_3 = (mlRep_2 * 4 as std::ffi::c_int as size_t)
                                                as std::ffi::c_int;
                                            let gain1_3 = (matchLength
                                                * 4 as std::ffi::c_int as size_t)
                                                .wrapping_sub(
                                                    ZSTD_highbit32(offBase as u32) as size_t
                                                )
                                                .wrapping_add(1 as std::ffi::c_int as size_t)
                                                as std::ffi::c_int;
                                            if mlRep_2 >= 4 as std::ffi::c_int as size_t
                                                && gain2_3 > gain1_3
                                            {
                                                matchLength = mlRep_2;
                                                offBase = REPCODE1_TO_OFFBASE as size_t;
                                                start = ip;
                                            }
                                        }
                                    }
                                    let mut ofbCandidate_0 = 999999999 as std::ffi::c_int as size_t;
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
                                    let gain2_4 = (ml2_1 * 4 as std::ffi::c_int as size_t)
                                        .wrapping_sub(
                                            ZSTD_highbit32(ofbCandidate_0 as u32) as size_t
                                        )
                                        as std::ffi::c_int;
                                    let gain1_4 = (matchLength * 4 as std::ffi::c_int as size_t)
                                        .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                        .wrapping_add(7 as std::ffi::c_int as size_t)
                                        as std::ffi::c_int;
                                    if !(ml2_1 >= 4 as std::ffi::c_int as size_t
                                        && gain2_4 > gain1_4)
                                    {
                                        break;
                                    }
                                    matchLength = ml2_1;
                                    offBase = ofbCandidate_0;
                                    start = ip;
                                }
                            }
                        }
                        if offBase > ZSTD_REP_NUM as size_t {
                            if dictMode as std::ffi::c_uint
                                == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint
                            {
                                while (start > anchor) as std::ffi::c_int
                                    & (start.offset(
                                        -(offBase.wrapping_sub(ZSTD_REP_NUM as size_t) as isize),
                                    ) > prefixLowest)
                                        as std::ffi::c_int
                                    != 0
                                    && *start.offset(-(1 as std::ffi::c_int) as isize)
                                        as std::ffi::c_int
                                        == *start
                                            .offset(
                                                -(offBase.wrapping_sub(ZSTD_REP_NUM as size_t)
                                                    as isize),
                                            )
                                            .offset(-(1 as std::ffi::c_int) as isize)
                                            as std::ffi::c_int
                                {
                                    start = start.offset(-1);
                                    start;
                                    matchLength = matchLength.wrapping_add(1);
                                    matchLength;
                                }
                            }
                            if isDxS != 0 {
                                let matchIndex = (start.offset_from(base) as std::ffi::c_long
                                    as size_t)
                                    .wrapping_sub(offBase.wrapping_sub(ZSTD_REP_NUM as size_t))
                                    as u32;
                                let mut match_0 = if matchIndex < prefixLowestIndex {
                                    dictBase
                                        .offset(matchIndex as isize)
                                        .offset(-(dictIndexDelta as isize))
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
                                    && *start.offset(-(1 as std::ffi::c_int) as isize)
                                        as std::ffi::c_int
                                        == *match_0.offset(-(1 as std::ffi::c_int) as isize)
                                            as std::ffi::c_int
                                {
                                    start = start.offset(-1);
                                    start;
                                    match_0 = match_0.offset(-1);
                                    match_0;
                                    matchLength = matchLength.wrapping_add(1);
                                    matchLength;
                                }
                            }
                            offset_2 = offset_1;
                            offset_1 = offBase.wrapping_sub(ZSTD_REP_NUM as size_t) as u32;
                        }
                    }
                }
            }
        }
        let litLength = start.offset_from(anchor) as std::ffi::c_long as size_t;
        ZSTD_storeSeq(
            seqStore,
            litLength,
            anchor,
            iend,
            offBase as u32,
            matchLength,
        );
        ip = start.offset(matchLength as isize);
        anchor = ip;
        if (*ms).lazySkipping != 0 {
            if searchMethod as std::ffi::c_uint
                == search_rowHash as std::ffi::c_int as std::ffi::c_uint
            {
                ZSTD_row_fillHashCache(ms, base, rowLog, mls, (*ms).nextToUpdate, ilimit);
            }
            (*ms).lazySkipping = 0 as std::ffi::c_int;
        }
        if isDxS != 0 {
            while ip <= ilimit {
                let current2 = ip.offset_from(base) as std::ffi::c_long as u32;
                let repIndex_2 = current2.wrapping_sub(offset_2);
                let mut repMatch_2 = if repIndex_2 < prefixLowestIndex {
                    dictBase
                        .offset(-(dictIndexDelta as isize))
                        .offset(repIndex_2 as isize)
                } else {
                    base.offset(repIndex_2 as isize)
                };
                if !(ZSTD_index_overlap_check(prefixLowestIndex, repIndex_2) != 0
                    && MEM_read32(repMatch_2 as *const std::ffi::c_void)
                        == MEM_read32(ip as *const std::ffi::c_void))
                {
                    break;
                }
                let repEnd2 = if repIndex_2 < prefixLowestIndex {
                    dictEnd
                } else {
                    iend
                };
                matchLength = (ZSTD_count_2segments(
                    ip.offset(4 as std::ffi::c_int as isize),
                    repMatch_2.offset(4 as std::ffi::c_int as isize),
                    iend,
                    repEnd2,
                    prefixLowest,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
                offBase = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offBase as u32;
                ZSTD_storeSeq(
                    seqStore,
                    0 as std::ffi::c_int as size_t,
                    anchor,
                    iend,
                    REPCODE1_TO_OFFBASE as u32,
                    matchLength,
                );
                ip = ip.offset(matchLength as isize);
                anchor = ip;
            }
        }
        if dictMode as std::ffi::c_uint == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint {
            while (ip <= ilimit) as std::ffi::c_int
                & (offset_2 > 0 as std::ffi::c_int as u32) as std::ffi::c_int
                != 0
                && MEM_read32(ip as *const std::ffi::c_void)
                    == MEM_read32(ip.offset(-(offset_2 as isize)) as *const std::ffi::c_void)
            {
                matchLength = (ZSTD_count(
                    ip.offset(4 as std::ffi::c_int as isize),
                    ip.offset(4 as std::ffi::c_int as isize)
                        .offset(-(offset_2 as isize)),
                    iend,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
                offBase = offset_2 as size_t;
                offset_2 = offset_1;
                offset_1 = offBase as u32;
                ZSTD_storeSeq(
                    seqStore,
                    0 as std::ffi::c_int as size_t,
                    anchor,
                    iend,
                    REPCODE1_TO_OFFBASE as u32,
                    matchLength,
                );
                ip = ip.offset(matchLength as isize);
                anchor = ip;
            }
        }
    }
    offsetSaved2 =
        if offsetSaved1 != 0 as std::ffi::c_int as u32 && offset_1 != 0 as std::ffi::c_int as u32 {
            offsetSaved1
        } else {
            offsetSaved2
        };
    *rep.offset(0 as std::ffi::c_int as isize) = if offset_1 != 0 {
        offset_1
    } else {
        offsetSaved1
    };
    *rep.offset(1 as std::ffi::c_int as isize) = if offset_2 != 0 {
        offset_2
    } else {
        offsetSaved2
    };
    iend.offset_from(anchor) as std::ffi::c_long as size_t
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy_dictMatchState)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dictMatchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy_dedicatedDictSearch)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dedicatedDictSearch(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy_dictMatchState_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dictMatchState_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy_dedicatedDictSearch_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_dedicatedDictSearch_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy_dictMatchState)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dictMatchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy_dedicatedDictSearch)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dedicatedDictSearch(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy_dictMatchState_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dictMatchState_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy_dedicatedDictSearch_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_dedicatedDictSearch_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2_dictMatchState)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dictMatchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2_dedicatedDictSearch)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dedicatedDictSearch(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2_dictMatchState_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dictMatchState_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2_dedicatedDictSearch_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_dedicatedDictSearch_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as std::ffi::c_int as u32,
        ZSTD_dedicatedDictSearch,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_btlazy2)]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2 as std::ffi::c_int as u32,
        ZSTD_noDict,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_btlazy2_dictMatchState)]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2_dictMatchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2 as std::ffi::c_int as u32,
        ZSTD_dictMatchState,
    )
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict_generic(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    searchMethod: searchMethod_e,
    depth: u32,
) -> size_t {
    let istart = src as *const u8;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.offset(srcSize as isize);
    let ilimit = if searchMethod as std::ffi::c_uint
        == search_rowHash as std::ffi::c_int as std::ffi::c_uint
    {
        iend.offset(-(8 as std::ffi::c_int as isize))
            .offset(-(ZSTD_ROW_HASH_CACHE_SIZE as isize))
    } else {
        iend.offset(-(8 as std::ffi::c_int as isize))
    };
    let base = (*ms).window.base;
    let dictLimit = (*ms).window.dictLimit;
    let prefixStart = base.offset(dictLimit as isize);
    let dictBase = (*ms).window.dictBase;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let dictStart = dictBase.offset((*ms).window.lowLimit as isize);
    let windowLog = (*ms).cParams.windowLog;
    let mls = if 4 as std::ffi::c_int as std::ffi::c_uint
        > (if (*ms).cParams.minMatch < 6 as std::ffi::c_int as std::ffi::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as std::ffi::c_int as std::ffi::c_uint
        }) {
        4 as std::ffi::c_int as std::ffi::c_uint
    } else if (*ms).cParams.minMatch < 6 as std::ffi::c_int as std::ffi::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as std::ffi::c_int as std::ffi::c_uint
    };
    let rowLog = if 4 as std::ffi::c_int as std::ffi::c_uint
        > (if (*ms).cParams.searchLog < 6 as std::ffi::c_int as std::ffi::c_uint {
            (*ms).cParams.searchLog
        } else {
            6 as std::ffi::c_int as std::ffi::c_uint
        }) {
        4 as std::ffi::c_int as std::ffi::c_uint
    } else if (*ms).cParams.searchLog < 6 as std::ffi::c_int as std::ffi::c_uint {
        (*ms).cParams.searchLog
    } else {
        6 as std::ffi::c_int as std::ffi::c_uint
    };
    let mut offset_1 = *rep.offset(0 as std::ffi::c_int as isize);
    let mut offset_2 = *rep.offset(1 as std::ffi::c_int as isize);
    (*ms).lazySkipping = 0 as std::ffi::c_int;
    ip = ip.offset((ip == prefixStart) as std::ffi::c_int as isize);
    if searchMethod as std::ffi::c_uint == search_rowHash as std::ffi::c_int as std::ffi::c_uint {
        ZSTD_row_fillHashCache(ms, base, rowLog, mls, (*ms).nextToUpdate, ilimit);
    }
    asm!(".p2align 5", options(preserves_flags, att_syntax));
    let mut current_block_61: u64;
    while ip < ilimit {
        let mut matchLength = 0 as std::ffi::c_int as size_t;
        let mut offBase = REPCODE1_TO_OFFBASE as size_t;
        let mut start = ip.offset(1 as std::ffi::c_int as isize);
        let mut curr = ip.offset_from(base) as std::ffi::c_long as u32;
        let windowLow = ZSTD_getLowestMatchIndex(
            ms,
            curr.wrapping_add(1 as std::ffi::c_int as u32),
            windowLog,
        );
        let repIndex = curr
            .wrapping_add(1 as std::ffi::c_int as u32)
            .wrapping_sub(offset_1);
        let repBase = if repIndex < dictLimit { dictBase } else { base };
        let repMatch = repBase.offset(repIndex as isize);
        if ZSTD_index_overlap_check(dictLimit, repIndex)
            & (offset_1
                <= curr
                    .wrapping_add(1 as std::ffi::c_int as u32)
                    .wrapping_sub(windowLow)) as std::ffi::c_int
            != 0
        {
            if MEM_read32(ip.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void)
                == MEM_read32(repMatch as *const std::ffi::c_void)
            {
                let repEnd = if repIndex < dictLimit { dictEnd } else { iend };
                matchLength = (ZSTD_count_2segments(
                    ip.offset(1 as std::ffi::c_int as isize)
                        .offset(4 as std::ffi::c_int as isize),
                    repMatch.offset(4 as std::ffi::c_int as isize),
                    iend,
                    repEnd,
                    prefixStart,
                ))
                .wrapping_add(4 as std::ffi::c_int as size_t);
                if depth == 0 as std::ffi::c_int as u32 {
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
            let mut ofbCandidate = 999999999 as std::ffi::c_int as size_t;
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
            if matchLength < 4 as std::ffi::c_int as size_t {
                let step = ip.offset_from(anchor) as std::ffi::c_long as size_t >> kSearchStrength;
                ip = ip.offset(step.wrapping_add(1 as std::ffi::c_int as size_t) as isize);
                (*ms).lazySkipping = (step > kLazySkippingStep as size_t) as std::ffi::c_int;
                continue;
            } else {
                if depth >= 1 as std::ffi::c_int as u32 {
                    while ip < ilimit {
                        ip = ip.offset(1);
                        ip;
                        curr = curr.wrapping_add(1);
                        curr;
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
                                & (offset_1 <= curr.wrapping_sub(windowLow_0)) as std::ffi::c_int
                                != 0
                                && MEM_read32(ip as *const std::ffi::c_void)
                                    == MEM_read32(repMatch_0 as *const std::ffi::c_void)
                            {
                                let repEnd_0 = if repIndex_0 < dictLimit {
                                    dictEnd
                                } else {
                                    iend
                                };
                                let repLength = (ZSTD_count_2segments(
                                    ip.offset(4 as std::ffi::c_int as isize),
                                    repMatch_0.offset(4 as std::ffi::c_int as isize),
                                    iend,
                                    repEnd_0,
                                    prefixStart,
                                ))
                                .wrapping_add(4 as std::ffi::c_int as size_t);
                                let gain2 =
                                    (repLength * 3 as std::ffi::c_int as size_t) as std::ffi::c_int;
                                let gain1 = (matchLength * 3 as std::ffi::c_int as size_t)
                                    .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                    .wrapping_add(1 as std::ffi::c_int as size_t)
                                    as std::ffi::c_int;
                                if repLength >= 4 as std::ffi::c_int as size_t && gain2 > gain1 {
                                    matchLength = repLength;
                                    offBase = REPCODE1_TO_OFFBASE as size_t;
                                    start = ip;
                                }
                            }
                        }
                        let mut ofbCandidate_0 = 999999999 as std::ffi::c_int as size_t;
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
                        let gain2_0 = (ml2_0 * 4 as std::ffi::c_int as size_t)
                            .wrapping_sub(ZSTD_highbit32(ofbCandidate_0 as u32) as size_t)
                            as std::ffi::c_int;
                        let gain1_0 = (matchLength * 4 as std::ffi::c_int as size_t)
                            .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                            .wrapping_add(4 as std::ffi::c_int as size_t)
                            as std::ffi::c_int;
                        if ml2_0 >= 4 as std::ffi::c_int as size_t && gain2_0 > gain1_0 {
                            matchLength = ml2_0;
                            offBase = ofbCandidate_0;
                            start = ip;
                        } else {
                            if !(depth == 2 as std::ffi::c_int as u32 && ip < ilimit) {
                                break;
                            }
                            ip = ip.offset(1);
                            ip;
                            curr = curr.wrapping_add(1);
                            curr;
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
                                        as std::ffi::c_int
                                    != 0
                                    && MEM_read32(ip as *const std::ffi::c_void)
                                        == MEM_read32(repMatch_1 as *const std::ffi::c_void)
                                {
                                    let repEnd_1 = if repIndex_1 < dictLimit {
                                        dictEnd
                                    } else {
                                        iend
                                    };
                                    let repLength_0 = (ZSTD_count_2segments(
                                        ip.offset(4 as std::ffi::c_int as isize),
                                        repMatch_1.offset(4 as std::ffi::c_int as isize),
                                        iend,
                                        repEnd_1,
                                        prefixStart,
                                    ))
                                    .wrapping_add(4 as std::ffi::c_int as size_t);
                                    let gain2_1 = (repLength_0 * 4 as std::ffi::c_int as size_t)
                                        as std::ffi::c_int;
                                    let gain1_1 = (matchLength * 4 as std::ffi::c_int as size_t)
                                        .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                        .wrapping_add(1 as std::ffi::c_int as size_t)
                                        as std::ffi::c_int;
                                    if repLength_0 >= 4 as std::ffi::c_int as size_t
                                        && gain2_1 > gain1_1
                                    {
                                        matchLength = repLength_0;
                                        offBase = REPCODE1_TO_OFFBASE as size_t;
                                        start = ip;
                                    }
                                }
                            }
                            let mut ofbCandidate_1 = 999999999 as std::ffi::c_int as size_t;
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
                            let gain2_2 = (ml2_1 * 4 as std::ffi::c_int as size_t)
                                .wrapping_sub(ZSTD_highbit32(ofbCandidate_1 as u32) as size_t)
                                as std::ffi::c_int;
                            let gain1_2 = (matchLength * 4 as std::ffi::c_int as size_t)
                                .wrapping_sub(ZSTD_highbit32(offBase as u32) as size_t)
                                .wrapping_add(7 as std::ffi::c_int as size_t)
                                as std::ffi::c_int;
                            if !(ml2_1 >= 4 as std::ffi::c_int as size_t && gain2_2 > gain1_2) {
                                break;
                            }
                            matchLength = ml2_1;
                            offBase = ofbCandidate_1;
                            start = ip;
                        }
                    }
                }
                if offBase > ZSTD_REP_NUM as size_t {
                    let matchIndex = (start.offset_from(base) as std::ffi::c_long as size_t)
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
                        && *start.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int
                            == *match_0.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int
                    {
                        start = start.offset(-1);
                        start;
                        match_0 = match_0.offset(-1);
                        match_0;
                        matchLength = matchLength.wrapping_add(1);
                        matchLength;
                    }
                    offset_2 = offset_1;
                    offset_1 = offBase.wrapping_sub(ZSTD_REP_NUM as size_t) as u32;
                }
            }
        }
        let litLength = start.offset_from(anchor) as std::ffi::c_long as size_t;
        ZSTD_storeSeq(
            seqStore,
            litLength,
            anchor,
            iend,
            offBase as u32,
            matchLength,
        );
        ip = start.offset(matchLength as isize);
        anchor = ip;
        if (*ms).lazySkipping != 0 {
            if searchMethod as std::ffi::c_uint
                == search_rowHash as std::ffi::c_int as std::ffi::c_uint
            {
                ZSTD_row_fillHashCache(ms, base, rowLog, mls, (*ms).nextToUpdate, ilimit);
            }
            (*ms).lazySkipping = 0 as std::ffi::c_int;
        }
        while ip <= ilimit {
            let repCurrent = ip.offset_from(base) as std::ffi::c_long as u32;
            let windowLow_2 = ZSTD_getLowestMatchIndex(ms, repCurrent, windowLog);
            let repIndex_2 = repCurrent.wrapping_sub(offset_2);
            let repBase_2 = if repIndex_2 < dictLimit {
                dictBase
            } else {
                base
            };
            let repMatch_2 = repBase_2.offset(repIndex_2 as isize);
            if ZSTD_index_overlap_check(dictLimit, repIndex_2)
                & (offset_2 <= repCurrent.wrapping_sub(windowLow_2)) as std::ffi::c_int
                == 0
            {
                break;
            }
            if MEM_read32(ip as *const std::ffi::c_void)
                != MEM_read32(repMatch_2 as *const std::ffi::c_void)
            {
                break;
            }
            let repEnd_2 = if repIndex_2 < dictLimit {
                dictEnd
            } else {
                iend
            };
            matchLength = (ZSTD_count_2segments(
                ip.offset(4 as std::ffi::c_int as isize),
                repMatch_2.offset(4 as std::ffi::c_int as isize),
                iend,
                repEnd_2,
                prefixStart,
            ))
            .wrapping_add(4 as std::ffi::c_int as size_t);
            offBase = offset_2 as size_t;
            offset_2 = offset_1;
            offset_1 = offBase as u32;
            ZSTD_storeSeq(
                seqStore,
                0 as std::ffi::c_int as size_t,
                anchor,
                iend,
                REPCODE1_TO_OFFBASE as u32,
                matchLength,
            );
            ip = ip.offset(matchLength as isize);
            anchor = ip;
        }
    }
    *rep.offset(0 as std::ffi::c_int as isize) = offset_1;
    *rep.offset(1 as std::ffi::c_int as isize) = offset_2;
    iend.offset_from(anchor) as std::ffi::c_long as size_t
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy_extDict)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_extDict(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        0 as std::ffi::c_int as u32,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_greedy_extDict_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_greedy_extDict_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        0 as std::ffi::c_int as u32,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy_extDict)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        1 as std::ffi::c_int as u32,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy_extDict_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy_extDict_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        1 as std::ffi::c_int as u32,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2_extDict)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_extDict(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_hashChain,
        2 as std::ffi::c_int as u32,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_lazy2_extDict_row)]
pub unsafe extern "C" fn ZSTD_compressBlock_lazy2_extDict_row(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_rowHash,
        2 as std::ffi::c_int as u32,
    )
}
#[export_name = crate::prefix!(ZSTD_compressBlock_btlazy2_extDict)]
pub unsafe extern "C" fn ZSTD_compressBlock_btlazy2_extDict(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_lazy_extDict_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        search_binaryTree,
        2 as std::ffi::c_int as u32,
    )
}
