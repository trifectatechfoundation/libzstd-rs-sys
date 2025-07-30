#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
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
pub type unalignArch = size_t;
pub type ZSTD_longLengthType_e = std::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
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
pub struct ZSTD_match_t {
    pub off: u32,
    pub len: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub loadedDictEnd: u32,
    pub bucketOffsets: *mut u8,
    pub splitIndices: [size_t; 64],
    pub matchCandidates: [ldmMatchCandidate_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmMatchCandidate_t {
    pub split: *const u8,
    pub hash: u32,
    pub checksum: u32,
    pub bucket: *mut ldmEntry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmEntry_t {
    pub offset: u32,
    pub checksum: u32,
}
pub type XXH64_hash_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmParams_t {
    pub enableLdm: ZSTD_ParamSwitch_e,
    pub hashLog: u32,
    pub bucketSizeLog: u32,
    pub minMatchLength: u32,
    pub hashRateLog: u32,
    pub windowLog: u32,
}
pub type ZSTD_overlap_e = std::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
pub type ZSTD_dictTableLoadMethod_e = std::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = std::ffi::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
pub type ZSTD_dictMode_e = std::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type ZSTD_BlockCompressor_f = Option<
    unsafe extern "C" fn(
        *mut ZSTD_MatchState_t,
        *mut SeqStore_t,
        *mut u32,
        *const std::ffi::c_void,
        size_t,
    ) -> size_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmRollingHashState_t {
    pub rolling: u64,
    pub stopMask: u64,
}
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 8 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
use crate::lib::common::xxhash::ZSTD_XXH64;
use crate::lib::compress::zstd_compress::{
    rawSeq, RawSeqStore_t, SeqStore_t, ZSTD_MatchState_t, ZSTD_optimal_t,
    ZSTD_selectBlockCompressor, ZSTD_window_t,
};
use crate::lib::compress::zstd_double_fast::ZSTD_fillDoubleHashTable;
use crate::lib::compress::zstd_fast::ZSTD_fillHashTable;
use crate::lib::zstd::*;
use crate::{MEM_isLittleEndian, MEM_read16, MEM_read32, MEM_readST};
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
pub const HASH_READ_SIZE: std::ffi::c_int = 8 as std::ffi::c_int;
pub const ZSTD_WINDOW_START_INDEX: std::ffi::c_int = 2 as std::ffi::c_int;
pub const LDM_BATCH_SIZE: std::ffi::c_int = 64 as std::ffi::c_int;
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
#[inline]
unsafe extern "C" fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> u32 {
    (window.lowLimit < window.dictLimit) as std::ffi::c_int as u32
}
#[inline]
unsafe extern "C" fn ZSTD_matchState_dictMode(mut ms: *const ZSTD_MatchState_t) -> ZSTD_dictMode_e {
    (if ZSTD_window_hasExtDict((*ms).window) != 0 {
        ZSTD_extDict as std::ffi::c_int
    } else if !((*ms).dictMatchState).is_null() {
        if (*(*ms).dictMatchState).dedicatedDictSearch != 0 {
            ZSTD_dedicatedDictSearch as std::ffi::c_int
        } else {
            ZSTD_dictMatchState as std::ffi::c_int
        }
    } else {
        ZSTD_noDict as std::ffi::c_int
    }) as ZSTD_dictMode_e
}
pub const ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY: std::ffi::c_int = 0 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn ZSTD_window_canOverflowCorrect(
    window: ZSTD_window_t,
    mut cycleLog: u32,
    mut maxDist: u32,
    mut loadedDictEnd: u32,
    mut src: *const std::ffi::c_void,
) -> u32 {
    let cycleSize = (1 as std::ffi::c_uint) << cycleLog;
    let curr = (src as *const u8).offset_from(window.base) as std::ffi::c_long as u32;
    let minIndexToOverflowCorrect = cycleSize
        .wrapping_add(if maxDist > cycleSize {
            maxDist
        } else {
            cycleSize
        })
        .wrapping_add(ZSTD_WINDOW_START_INDEX as u32);
    let adjustment = (window.nbOverflowCorrections).wrapping_add(1 as std::ffi::c_int as u32);
    let adjustedIndex = if minIndexToOverflowCorrect * adjustment > minIndexToOverflowCorrect {
        minIndexToOverflowCorrect * adjustment
    } else {
        minIndexToOverflowCorrect
    };
    let indexLargeEnough = (curr > adjustedIndex) as std::ffi::c_int as u32;
    let dictionaryInvalidated =
        (curr > maxDist.wrapping_add(loadedDictEnd)) as std::ffi::c_int as u32;
    (indexLargeEnough != 0 && dictionaryInvalidated != 0) as std::ffi::c_int as u32
}
#[inline]
unsafe extern "C" fn ZSTD_window_needOverflowCorrection(
    window: ZSTD_window_t,
    mut cycleLog: u32,
    mut maxDist: u32,
    mut loadedDictEnd: u32,
    mut src: *const std::ffi::c_void,
    mut srcEnd: *const std::ffi::c_void,
) -> u32 {
    let curr = (srcEnd as *const u8).offset_from(window.base) as std::ffi::c_long as u32;
    (curr
        > (if MEM_64bits() != 0 {
            (3500 as std::ffi::c_uint)
                .wrapping_mul(((1 as std::ffi::c_int) << 20 as std::ffi::c_int) as std::ffi::c_uint)
        } else {
            (2000 as std::ffi::c_uint)
                .wrapping_mul(((1 as std::ffi::c_int) << 20 as std::ffi::c_int) as std::ffi::c_uint)
        })) as std::ffi::c_int as u32
}
#[inline]
unsafe extern "C" fn ZSTD_window_correctOverflow(
    mut window: *mut ZSTD_window_t,
    mut cycleLog: u32,
    mut maxDist: u32,
    mut src: *const std::ffi::c_void,
) -> u32 {
    let cycleSize = (1 as std::ffi::c_uint) << cycleLog;
    let cycleMask = cycleSize.wrapping_sub(1 as std::ffi::c_int as u32);
    let curr = (src as *const u8).offset_from((*window).base) as std::ffi::c_long as u32;
    let currentCycle = curr & cycleMask;
    let currentCycleCorrection = if currentCycle < ZSTD_WINDOW_START_INDEX as u32 {
        if cycleSize > 2 as std::ffi::c_int as u32 {
            cycleSize
        } else {
            2 as std::ffi::c_int as u32
        }
    } else {
        0 as std::ffi::c_int as u32
    };
    let newCurrent = currentCycle
        .wrapping_add(currentCycleCorrection)
        .wrapping_add(if maxDist > cycleSize {
            maxDist
        } else {
            cycleSize
        });
    let correction = curr.wrapping_sub(newCurrent);
    ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY == 0;
    (*window).base = ((*window).base).offset(correction as isize);
    (*window).dictBase = ((*window).dictBase).offset(correction as isize);
    if (*window).lowLimit < correction.wrapping_add(ZSTD_WINDOW_START_INDEX as u32) {
        (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32;
    } else {
        (*window).lowLimit = ((*window).lowLimit).wrapping_sub(correction);
    }
    if (*window).dictLimit < correction.wrapping_add(ZSTD_WINDOW_START_INDEX as u32) {
        (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32;
    } else {
        (*window).dictLimit = ((*window).dictLimit).wrapping_sub(correction);
    }
    (*window).nbOverflowCorrections = ((*window).nbOverflowCorrections).wrapping_add(1);
    (*window).nbOverflowCorrections;
    correction
}
#[inline]
unsafe extern "C" fn ZSTD_window_enforceMaxDist(
    mut window: *mut ZSTD_window_t,
    mut blockEnd: *const std::ffi::c_void,
    mut maxDist: u32,
    mut loadedDictEndPtr: *mut u32,
    mut dictMatchStatePtr: *mut *const ZSTD_MatchState_t,
) {
    let blockEndIdx =
        (blockEnd as *const u8).offset_from((*window).base) as std::ffi::c_long as u32;
    let loadedDictEnd = if !loadedDictEndPtr.is_null() {
        *loadedDictEndPtr
    } else {
        0 as std::ffi::c_int as u32
    };
    if blockEndIdx > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit = blockEndIdx.wrapping_sub(maxDist);
        if (*window).lowLimit < newLowLimit {
            (*window).lowLimit = newLowLimit;
        }
        if (*window).dictLimit < (*window).lowLimit {
            (*window).dictLimit = (*window).lowLimit;
        }
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr = 0 as std::ffi::c_int as u32;
        }
        if !dictMatchStatePtr.is_null() {
            *dictMatchStatePtr = NULL as *const ZSTD_MatchState_t;
        }
    }
}
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
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
unsafe extern "C" fn ZSTD_cwksp_alloc_size(mut size: size_t) -> size_t {
    if size == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    size
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
static mut ZSTD_ldm_gearTab: [u64; 256] = [
    0xf5b8f72c5f77775c as std::ffi::c_ulong,
    0x84935f266b7ac412 as std::ffi::c_ulong,
    0xb647ada9ca730ccc as std::ffi::c_ulong,
    0xb065bb4b114fb1de as std::ffi::c_ulong,
    0x34584e7e8c3a9fd0 as std::ffi::c_long as u64,
    0x4e97e17c6ae26b05 as std::ffi::c_long as u64,
    0x3a03d743bc99a604 as std::ffi::c_long as u64,
    0xcecd042422c4044f as std::ffi::c_ulong,
    0x76de76c58524259e as std::ffi::c_long as u64,
    0x9c8528f65badeaca as std::ffi::c_ulong,
    0x86563706e2097529 as std::ffi::c_ulong,
    0x2902475fa375d889 as std::ffi::c_long as u64,
    0xafb32a9739a5ebe6 as std::ffi::c_ulong,
    0xce2714da3883e639 as std::ffi::c_ulong,
    0x21eaf821722e69e as std::ffi::c_long as u64,
    0x37b628620b628 as std::ffi::c_long as u64,
    0x49a8d455d88caf5 as std::ffi::c_long as u64,
    0x8556d711e6958140 as std::ffi::c_ulong,
    0x4f7ae74fc605c1f as std::ffi::c_long as u64,
    0x829f0c3468bd3a20 as std::ffi::c_ulong,
    0x4ffdc885c625179e as std::ffi::c_long as u64,
    0x8473de048a3daf1b as std::ffi::c_ulong,
    0x51008822b05646b2 as std::ffi::c_long as u64,
    0x69d75d12b2d1cc5f as std::ffi::c_long as u64,
    0x8c9d4a19159154bc as std::ffi::c_ulong,
    0xc3cc10f4abbd4003 as std::ffi::c_ulong,
    0xd06ddc1cecb97391 as std::ffi::c_ulong,
    0xbe48e6e7ed80302e as std::ffi::c_ulong,
    0x3481db31cee03547 as std::ffi::c_long as u64,
    0xacc3f67cdaa1d210 as std::ffi::c_ulong,
    0x65cb771d8c7f96cc as std::ffi::c_long as u64,
    0x8eb27177055723dd as std::ffi::c_ulong,
    0xc789950d44cd94be as std::ffi::c_ulong,
    0x934feadc3700b12b as std::ffi::c_ulong,
    0x5e485f11edbdf182 as std::ffi::c_long as u64,
    0x1e2e2a46fd64767a as std::ffi::c_long as u64,
    0x2969ca71d82efa7c as std::ffi::c_long as u64,
    0x9d46e9935ebbba2e as std::ffi::c_ulong,
    0xe056b67e05e6822b as std::ffi::c_ulong,
    0x94d73f55739d03a0 as std::ffi::c_ulong,
    0xcd7010bdb69b5a03 as std::ffi::c_ulong,
    0x455ef9fcd79b82f4 as std::ffi::c_long as u64,
    0x869cb54a8749c161 as std::ffi::c_ulong,
    0x38d1a4fa6185d225 as std::ffi::c_long as u64,
    0xb475166f94bbe9bb as std::ffi::c_ulong,
    0xa4143548720959f1 as std::ffi::c_ulong,
    0x7aed4780ba6b26ba as std::ffi::c_long as u64,
    0xd0ce264439e02312 as std::ffi::c_ulong,
    0x84366d746078d508 as std::ffi::c_ulong,
    0xa8ce973c72ed17be as std::ffi::c_ulong,
    0x21c323a29a430b01 as std::ffi::c_long as u64,
    0x9962d617e3af80ee as std::ffi::c_ulong,
    0xab0ce91d9c8cf75b as std::ffi::c_ulong,
    0x530e8ee6d19a4dbc as std::ffi::c_long as u64,
    0x2ef68c0cf53f5d72 as std::ffi::c_long as u64,
    0xc03a681640a85506 as std::ffi::c_ulong,
    0x496e4e9f9c310967 as std::ffi::c_long as u64,
    0x78580472b59b14a0 as std::ffi::c_long as u64,
    0x273824c23b388577 as std::ffi::c_long as u64,
    0x66bf923ad45cb553 as std::ffi::c_long as u64,
    0x47ae1a5a2492ba86 as std::ffi::c_long as u64,
    0x35e304569e229659 as std::ffi::c_long as u64,
    0x4765182a46870b6f as std::ffi::c_long as u64,
    0x6cbab625e9099412 as std::ffi::c_long as u64,
    0xddac9a2e598522c1 as std::ffi::c_ulong,
    0x7172086e666624f2 as std::ffi::c_long as u64,
    0xdf5003ca503b7837 as std::ffi::c_ulong,
    0x88c0c1db78563d09 as std::ffi::c_ulong,
    0x58d51865acfc289d as std::ffi::c_long as u64,
    0x177671aec65224f1 as std::ffi::c_long as u64,
    0xfb79d8a241e967d7 as std::ffi::c_ulong,
    0x2be1e101cad9a49a as std::ffi::c_long as u64,
    0x6625682f6e29186b as std::ffi::c_long as u64,
    0x399553457ac06e50 as std::ffi::c_long as u64,
    0x35dffb4c23abb74 as std::ffi::c_long as u64,
    0x429db2591f54aade as std::ffi::c_long as u64,
    0xc52802a8037d1009 as std::ffi::c_ulong,
    0x6acb27381f0b25f3 as std::ffi::c_long as u64,
    0xf45e2551ee4f823b as std::ffi::c_ulong,
    0x8b0ea2d99580c2f7 as std::ffi::c_ulong,
    0x3bed519cbcb4e1e1 as std::ffi::c_long as u64,
    0xff452823dbb010a as std::ffi::c_long as u64,
    0x9d42ed614f3dd267 as std::ffi::c_ulong,
    0x5b9313c06257c57b as std::ffi::c_long as u64,
    0xa114b8008b5e1442 as std::ffi::c_ulong,
    0xc1fe311c11c13d4b as std::ffi::c_ulong,
    0x66e8763ea34c5568 as std::ffi::c_long as u64,
    0x8b982af1c262f05d as std::ffi::c_ulong,
    0xee8876faaa75fbb7 as std::ffi::c_ulong,
    0x8a62a4d0d172bb2a as std::ffi::c_ulong,
    0xc13d94a3b7449a97 as std::ffi::c_ulong,
    0x6dbbba9dc15d037c as std::ffi::c_long as u64,
    0xc786101f1d92e0f1 as std::ffi::c_ulong,
    0xd78681a907a0b79b as std::ffi::c_ulong,
    0xf61aaf2962c9abb9 as std::ffi::c_ulong,
    0x2cfd16fcd3cb7ad9 as std::ffi::c_long as u64,
    0x868c5b6744624d21 as std::ffi::c_ulong,
    0x25e650899c74ddd7 as std::ffi::c_long as u64,
    0xba042af4a7c37463 as std::ffi::c_ulong,
    0x4eb1a539465a3eca as std::ffi::c_long as u64,
    0xbe09dbf03b05d5ca as std::ffi::c_ulong,
    0x774e5a362b5472ba as std::ffi::c_long as u64,
    0x47a1221229d183cd as std::ffi::c_long as u64,
    0x504b0ca18ef5a2df as std::ffi::c_long as u64,
    0xdffbdfbde2456eb9 as std::ffi::c_ulong,
    0x46cd2b2fbee34634 as std::ffi::c_long as u64,
    0xf2aef8fe819d98c3 as std::ffi::c_ulong,
    0x357f5276d4599d61 as std::ffi::c_long as u64,
    0x24a5483879c453e3 as std::ffi::c_long as u64,
    0x88026889192b4b9 as std::ffi::c_long as u64,
    0x28da96671782dbec as std::ffi::c_long as u64,
    0x4ef37c40588e9aaa as std::ffi::c_long as u64,
    0x8837b90651bc9fb3 as std::ffi::c_ulong,
    0xc164f741d3f0e5d6 as std::ffi::c_ulong,
    0xbc135a0a704b70ba as std::ffi::c_ulong,
    0x69cd868f7622ada as std::ffi::c_long as u64,
    0xbc37ba89e0b9c0ab as std::ffi::c_ulong,
    0x47c14a01323552f6 as std::ffi::c_long as u64,
    0x4f00794bacee98bb as std::ffi::c_long as u64,
    0x7107de7d637a69d5 as std::ffi::c_long as u64,
    0x88af793bb6f2255e as std::ffi::c_ulong,
    0xf3c6466b8799b598 as std::ffi::c_ulong,
    0xc288c616aa7f3b59 as std::ffi::c_ulong,
    0x81ca63cf42fca3fd as std::ffi::c_ulong,
    0x88d85ace36a2674b as std::ffi::c_ulong,
    0xd056bd3792389e7 as std::ffi::c_long as u64,
    0xe55c396c4e9dd32d as std::ffi::c_ulong,
    0xbefb504571e6c0a6 as std::ffi::c_ulong,
    0x96ab32115e91e8cc as std::ffi::c_ulong,
    0xbf8acb18de8f38d1 as std::ffi::c_ulong,
    0x66dae58801672606 as std::ffi::c_long as u64,
    0x833b6017872317fb as std::ffi::c_ulong,
    0xb87c16f2d1c92864 as std::ffi::c_ulong,
    0xdb766a74e58b669c as std::ffi::c_ulong,
    0x89659f85c61417be as std::ffi::c_ulong,
    0xc8daad856011ea0c as std::ffi::c_ulong,
    0x76a4b565b6fe7eae as std::ffi::c_long as u64,
    0xa469d085f6237312 as std::ffi::c_ulong,
    0xaaf0365683a3e96c as std::ffi::c_ulong,
    0x4dbb746f8424f7b8 as std::ffi::c_long as u64,
    0x638755af4e4acc1 as std::ffi::c_long as u64,
    0x3d7807f5bde64486 as std::ffi::c_long as u64,
    0x17be6d8f5bbb7639 as std::ffi::c_long as u64,
    0x903f0cd44dc35dc as std::ffi::c_long as u64,
    0x67b672eafdf1196c as std::ffi::c_long as u64,
    0xa676ff93ed4c82f1 as std::ffi::c_ulong,
    0x521d1004c5053d9d as std::ffi::c_long as u64,
    0x37ba9ad09ccc9202 as std::ffi::c_long as u64,
    0x84e54d297aacfb51 as std::ffi::c_ulong,
    0xa0b4b776a143445 as std::ffi::c_long as u64,
    0x820d471e20b348e as std::ffi::c_long as u64,
    0x1874383cb83d46dc as std::ffi::c_long as u64,
    0x97edeec7a1efe11c as std::ffi::c_ulong,
    0xb330e50b1bdc42aa as std::ffi::c_ulong,
    0x1dd91955ce70e032 as std::ffi::c_long as u64,
    0xa514cdb88f2939d5 as std::ffi::c_ulong,
    0x2791233fd90db9d3 as std::ffi::c_long as u64,
    0x7b670a4cc50f7a9b as std::ffi::c_long as u64,
    0x77c07d2a05c6dfa5 as std::ffi::c_long as u64,
    0xe3778b6646d0a6fa as std::ffi::c_ulong,
    0xb39c8eda47b56749 as std::ffi::c_ulong,
    0x933ed448addbef28 as std::ffi::c_ulong,
    0xaf846af6ab7d0bf4 as std::ffi::c_ulong,
    0xe5af208eb666e49 as std::ffi::c_long as u64,
    0x5e6622f73534cd6a as std::ffi::c_long as u64,
    0x297daeca42ef5b6e as std::ffi::c_long as u64,
    0x862daef3d35539a6 as std::ffi::c_ulong,
    0xe68722498f8e1ea9 as std::ffi::c_ulong,
    0x981c53093dc0d572 as std::ffi::c_ulong,
    0xfa09b0bfbf86fbf5 as std::ffi::c_ulong,
    0x30b1e96166219f15 as std::ffi::c_long as u64,
    0x70e7d466bdc4fb83 as std::ffi::c_long as u64,
    0x5a66736e35f2a8e9 as std::ffi::c_long as u64,
    0xcddb59d2b7c1baef as std::ffi::c_ulong,
    0xd6c7d247d26d8996 as std::ffi::c_ulong,
    0xea4e39eac8de1ba3 as std::ffi::c_ulong,
    0x539c8bb19fa3aff2 as std::ffi::c_long as u64,
    0x9f90e4c5fd508d8 as std::ffi::c_long as u64,
    0xa34e5956fbaf3385 as std::ffi::c_ulong,
    0x2e2f8e151d3ef375 as std::ffi::c_long as u64,
    0x173691e9b83faec1 as std::ffi::c_long as u64,
    0xb85a8d56bf016379 as std::ffi::c_ulong,
    0x8382381267408ae3 as std::ffi::c_ulong,
    0xb90f901bbdc0096d as std::ffi::c_ulong,
    0x7c6ad32933bcec65 as std::ffi::c_long as u64,
    0x76bb5e2f2c8ad595 as std::ffi::c_long as u64,
    0x390f851a6cf46d28 as std::ffi::c_long as u64,
    0xc3e6064da1c2da72 as std::ffi::c_ulong,
    0xc52a0c101cfa5389 as std::ffi::c_ulong,
    0xd78eaf84a3fbc530 as std::ffi::c_ulong,
    0x3781b9e2288b997e as std::ffi::c_long as u64,
    0x73c2f6dea83d05c4 as std::ffi::c_long as u64,
    0x4228e364c5b5ed7 as std::ffi::c_long as u64,
    0x9d7a3edf0da43911 as std::ffi::c_ulong,
    0x8edcfeda24686756 as std::ffi::c_ulong,
    0x5e7667a7b7a9b3a1 as std::ffi::c_long as u64,
    0x4c4f389fa143791d as std::ffi::c_long as u64,
    0xb08bc1023da7cddc as std::ffi::c_ulong,
    0x7ab4be3ae529b1cc as std::ffi::c_long as u64,
    0x754e6132dbe74ff9 as std::ffi::c_long as u64,
    0x71635442a839df45 as std::ffi::c_long as u64,
    0x2f6fb1643fbe52de as std::ffi::c_long as u64,
    0x961e0a42cf7a8177 as std::ffi::c_ulong,
    0xf3b45d83d89ef2ea as std::ffi::c_ulong,
    0xee3de4cf4a6e3e9b as std::ffi::c_ulong,
    0xcd6848542c3295e7 as std::ffi::c_ulong,
    0xe4cee1664c78662f as std::ffi::c_ulong,
    0x9947548b474c68c4 as std::ffi::c_ulong,
    0x25d73777a5ed8b0b as std::ffi::c_long as u64,
    0xc915b1d636b7fc as std::ffi::c_long as u64,
    0x21c2ba75d9b0d2da as std::ffi::c_long as u64,
    0x5f6b5dcf608a64a1 as std::ffi::c_long as u64,
    0xdcf333255ff9570c as std::ffi::c_ulong,
    0x633b922418ced4ee as std::ffi::c_long as u64,
    0xc136dde0b004b34a as std::ffi::c_ulong,
    0x58cc83b05d4b2f5a as std::ffi::c_long as u64,
    0x5eb424dda28e42d2 as std::ffi::c_long as u64,
    0x62df47369739cd98 as std::ffi::c_long as u64,
    0xb4e0b42485e4ce17 as std::ffi::c_ulong,
    0x16e1f0c1f9a8d1e7 as std::ffi::c_long as u64,
    0x8ec3916707560ebf as std::ffi::c_ulong,
    0x62ba6e2df2cc9db3 as std::ffi::c_long as u64,
    0xcbf9f4ff77d83a16 as std::ffi::c_ulong,
    0x78d9d7d07d2bbcc4 as std::ffi::c_long as u64,
    0xef554ce1e02c41f4 as std::ffi::c_ulong,
    0x8d7581127eccf94d as std::ffi::c_ulong,
    0xa9b53336cb3c8a05 as std::ffi::c_ulong,
    0x38c42c0bf45c4f91 as std::ffi::c_long as u64,
    0x640893cdf4488863 as std::ffi::c_long as u64,
    0x80ec34bc575ea568 as std::ffi::c_ulong,
    0x39f324f5b48eaa40 as std::ffi::c_long as u64,
    0xe9d9ed1f8eff527f as std::ffi::c_ulong,
    0x9224fc058cc5a214 as std::ffi::c_ulong,
    0xbaba00b04cfe7741 as std::ffi::c_ulong,
    0x309a9f120fcf52af as std::ffi::c_long as u64,
    0xa558f3ec65626212 as std::ffi::c_ulong,
    0x424bec8b7adabe2f as std::ffi::c_long as u64,
    0x41622513a6aea433 as std::ffi::c_long as u64,
    0xb88da2d5324ca798 as std::ffi::c_ulong,
    0xd287733b245528a4 as std::ffi::c_ulong,
    0x9a44697e6d68aec3 as std::ffi::c_ulong,
    0x7b1093be2f49bb28 as std::ffi::c_long as u64,
    0x50bbec632e3d8aad as std::ffi::c_long as u64,
    0x6cd90723e1ea8283 as std::ffi::c_long as u64,
    0x897b9e7431b02bf3 as std::ffi::c_ulong,
    0x219efdcb338a7047 as std::ffi::c_long as u64,
    0x3b0311f0a27c0656 as std::ffi::c_long as u64,
    0xdb17bf91c0db96e7 as std::ffi::c_ulong,
    0x8cd4fd6b4e85a5b2 as std::ffi::c_ulong,
    0xfab071054ba6409d as std::ffi::c_ulong,
    0x40d6fe831fa9dfd9 as std::ffi::c_long as u64,
    0xaf358debad7d791e as std::ffi::c_ulong,
    0xeb8d0e25a65e3e58 as std::ffi::c_ulong,
    0xbbcbd3df14e08580 as std::ffi::c_ulong,
    0xcf751f27ecdab2b as std::ffi::c_long as u64,
    0x2b4da14f2613d8f4 as std::ffi::c_long as u64,
];
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const LDM_MIN_MATCH_LENGTH: std::ffi::c_int = 64 as std::ffi::c_int;
unsafe extern "C" fn ZSTD_ldm_gear_init(
    mut state: *mut ldmRollingHashState_t,
    mut params: *const ldmParams_t,
) {
    let mut maxBitsInMask = if (*params).minMatchLength < 64 as std::ffi::c_int as u32 {
        (*params).minMatchLength
    } else {
        64 as std::ffi::c_int as u32
    };
    let mut hashRateLog = (*params).hashRateLog;
    (*state).rolling = !(0 as std::ffi::c_int as u32) as u64;
    if hashRateLog > 0 as std::ffi::c_int as std::ffi::c_uint && hashRateLog <= maxBitsInMask {
        (*state).stopMask = ((1 as std::ffi::c_int as u64) << hashRateLog)
            .wrapping_sub(1 as std::ffi::c_int as u64)
            << maxBitsInMask.wrapping_sub(hashRateLog);
    } else {
        (*state).stopMask = ((1 as std::ffi::c_int as u64) << hashRateLog)
            .wrapping_sub(1 as std::ffi::c_int as u64);
    };
}
unsafe extern "C" fn ZSTD_ldm_gear_reset(
    mut state: *mut ldmRollingHashState_t,
    mut data: *const u8,
    mut minMatchLength: size_t,
) {
    let mut hash = (*state).rolling;
    let mut n = 0 as std::ffi::c_int as size_t;
    while n.wrapping_add(3 as std::ffi::c_int as size_t) < minMatchLength {
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
    }
    while n < minMatchLength {
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
    }
}
unsafe extern "C" fn ZSTD_ldm_gear_feed(
    mut state: *mut ldmRollingHashState_t,
    mut data: *const u8,
    mut size: size_t,
    mut splits: *mut size_t,
    mut numSplits: *mut std::ffi::c_uint,
) -> size_t {
    let mut current_block: u64;
    let mut n: size_t = 0;
    let mut hash: u64 = 0;
    let mut mask: u64 = 0;
    hash = (*state).rolling;
    mask = (*state).stopMask;
    n = 0 as std::ffi::c_int as size_t;
    loop {
        if n.wrapping_add(3 as std::ffi::c_int as size_t) >= size {
            current_block = 5689316957504528238;
            break;
        }
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
        if (hash & mask == 0 as std::ffi::c_int as u64) as std::ffi::c_int as std::ffi::c_long != 0
        {
            *splits.offset(*numSplits as isize) = n;
            *numSplits = (*numSplits).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
            if *numSplits == LDM_BATCH_SIZE as std::ffi::c_uint {
                current_block = 12351618399163395313;
                break;
            }
        }
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
        if (hash & mask == 0 as std::ffi::c_int as u64) as std::ffi::c_int as std::ffi::c_long != 0
        {
            *splits.offset(*numSplits as isize) = n;
            *numSplits = (*numSplits).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
            if *numSplits == LDM_BATCH_SIZE as std::ffi::c_uint {
                current_block = 12351618399163395313;
                break;
            }
        }
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
        if (hash & mask == 0 as std::ffi::c_int as u64) as std::ffi::c_int as std::ffi::c_long != 0
        {
            *splits.offset(*numSplits as isize) = n;
            *numSplits = (*numSplits).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
            if *numSplits == LDM_BATCH_SIZE as std::ffi::c_uint {
                current_block = 12351618399163395313;
                break;
            }
        }
        hash = (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
            (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int) as isize,
        ));
        n = n.wrapping_add(1 as std::ffi::c_int as size_t);
        if (hash & mask == 0 as std::ffi::c_int as u64) as std::ffi::c_int as std::ffi::c_long == 0
        {
            continue;
        }
        *splits.offset(*numSplits as isize) = n;
        *numSplits = (*numSplits).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
        if *numSplits == LDM_BATCH_SIZE as std::ffi::c_uint {
            current_block = 12351618399163395313;
            break;
        }
    }
    loop {
        match current_block {
            12351618399163395313 => {
                (*state).rolling = hash;
                break;
            }
            _ => {
                if n >= size {
                    current_block = 12351618399163395313;
                    continue;
                }
                hash =
                    (hash << 1 as std::ffi::c_int).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                        (*data.offset(n as isize) as std::ffi::c_int & 0xff as std::ffi::c_int)
                            as isize,
                    ));
                n = n.wrapping_add(1 as std::ffi::c_int as size_t);
                if (hash & mask == 0 as std::ffi::c_int as u64) as std::ffi::c_int
                    as std::ffi::c_long
                    == 0
                {
                    current_block = 5689316957504528238;
                    continue;
                }
                *splits.offset(*numSplits as isize) = n;
                *numSplits = (*numSplits).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
                if *numSplits == LDM_BATCH_SIZE as std::ffi::c_uint {
                    current_block = 12351618399163395313;
                } else {
                    current_block = 5689316957504528238;
                }
            }
        }
    }
    n
}
#[export_name = crate::prefix!(ZSTD_ldm_adjustParameters)]
pub unsafe extern "C" fn ZSTD_ldm_adjustParameters(
    mut params: *mut ldmParams_t,
    mut cParams: *const ZSTD_compressionParameters,
) {
    (*params).windowLog = (*cParams).windowLog;
    if (*params).hashRateLog == 0 as std::ffi::c_int as u32 {
        if (*params).hashLog > 0 as std::ffi::c_int as u32 {
            if (*params).windowLog > (*params).hashLog {
                (*params).hashRateLog = ((*params).windowLog).wrapping_sub((*params).hashLog);
            }
        } else {
            (*params).hashRateLog = (7 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(
                ((*cParams).strategy as std::ffi::c_uint)
                    .wrapping_div(3 as std::ffi::c_int as std::ffi::c_uint),
            );
        }
    }
    if (*params).hashLog == 0 as std::ffi::c_int as u32 {
        (*params).hashLog = if 6 as std::ffi::c_int as u32
            > (if ((*params).windowLog).wrapping_sub((*params).hashRateLog)
                < (if (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                    == 4 as std::ffi::c_int as std::ffi::c_ulong
                {
                    30 as std::ffi::c_int
                } else {
                    31 as std::ffi::c_int
                }) < 30 as std::ffi::c_int
                {
                    if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                        == 4 as std::ffi::c_int as std::ffi::c_ulong
                    {
                        30 as std::ffi::c_int
                    } else {
                        31 as std::ffi::c_int
                    }
                } else {
                    30 as std::ffi::c_int
                }) as u32
            {
                ((*params).windowLog).wrapping_sub((*params).hashRateLog)
            } else {
                (if (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                    == 4 as std::ffi::c_int as std::ffi::c_ulong
                {
                    30 as std::ffi::c_int
                } else {
                    31 as std::ffi::c_int
                }) < 30 as std::ffi::c_int
                {
                    if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                        == 4 as std::ffi::c_int as std::ffi::c_ulong
                    {
                        30 as std::ffi::c_int
                    } else {
                        31 as std::ffi::c_int
                    }
                } else {
                    30 as std::ffi::c_int
                }) as u32
            }) {
            6 as std::ffi::c_int as u32
        } else if ((*params).windowLog).wrapping_sub((*params).hashRateLog)
            < (if (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 4 as std::ffi::c_int as std::ffi::c_ulong
            {
                30 as std::ffi::c_int
            } else {
                31 as std::ffi::c_int
            }) < 30 as std::ffi::c_int
            {
                if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                    == 4 as std::ffi::c_int as std::ffi::c_ulong
                {
                    30 as std::ffi::c_int
                } else {
                    31 as std::ffi::c_int
                }
            } else {
                30 as std::ffi::c_int
            }) as u32
        {
            ((*params).windowLog).wrapping_sub((*params).hashRateLog)
        } else {
            (if (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 4 as std::ffi::c_int as std::ffi::c_ulong
            {
                30 as std::ffi::c_int
            } else {
                31 as std::ffi::c_int
            }) < 30 as std::ffi::c_int
            {
                if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                    == 4 as std::ffi::c_int as std::ffi::c_ulong
                {
                    30 as std::ffi::c_int
                } else {
                    31 as std::ffi::c_int
                }
            } else {
                30 as std::ffi::c_int
            }) as u32
        };
    }
    if (*params).minMatchLength == 0 as std::ffi::c_int as u32 {
        (*params).minMatchLength = LDM_MIN_MATCH_LENGTH as u32;
        if (*cParams).strategy as std::ffi::c_uint
            >= ZSTD_btultra as std::ffi::c_int as std::ffi::c_uint
        {
            (*params).minMatchLength /= 2 as std::ffi::c_int as u32;
        }
    }
    if (*params).bucketSizeLog == 0 as std::ffi::c_int as u32 {
        (*params).bucketSizeLog = if 4 as std::ffi::c_int as u32
            > (if ((*cParams).strategy as u32) < 8 as std::ffi::c_int as u32 {
                (*cParams).strategy as u32
            } else {
                8 as std::ffi::c_int as u32
            }) {
            4 as std::ffi::c_int as u32
        } else if ((*cParams).strategy as u32) < 8 as std::ffi::c_int as u32 {
            (*cParams).strategy as u32
        } else {
            8 as std::ffi::c_int as u32
        };
    }
    (*params).bucketSizeLog = if (*params).bucketSizeLog < (*params).hashLog {
        (*params).bucketSizeLog
    } else {
        (*params).hashLog
    };
}
#[export_name = crate::prefix!(ZSTD_ldm_getTableSize)]
pub unsafe extern "C" fn ZSTD_ldm_getTableSize(mut params: ldmParams_t) -> size_t {
    let ldmHSize = (1 as std::ffi::c_int as size_t) << params.hashLog;
    let ldmBucketSizeLog = (if params.bucketSizeLog < params.hashLog {
        params.bucketSizeLog
    } else {
        params.hashLog
    }) as size_t;
    let ldmBucketSize = (1 as std::ffi::c_int as size_t)
        << (params.hashLog as size_t).wrapping_sub(ldmBucketSizeLog);
    let totalSize = (ZSTD_cwksp_alloc_size(ldmBucketSize)).wrapping_add(ZSTD_cwksp_alloc_size(
        ldmHSize.wrapping_mul(::core::mem::size_of::<ldmEntry_t>() as std::ffi::c_ulong),
    ));
    if params.enableLdm as std::ffi::c_uint == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        totalSize
    } else {
        0 as std::ffi::c_int as size_t
    }
}
#[export_name = crate::prefix!(ZSTD_ldm_getMaxNbSeq)]
pub unsafe extern "C" fn ZSTD_ldm_getMaxNbSeq(
    mut params: ldmParams_t,
    mut maxChunkSize: size_t,
) -> size_t {
    if params.enableLdm as std::ffi::c_uint == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        maxChunkSize / params.minMatchLength as size_t
    } else {
        0 as std::ffi::c_int as size_t
    }
}
unsafe extern "C" fn ZSTD_ldm_getBucket(
    mut ldmState: *const ldmState_t,
    mut hash: size_t,
    bucketSizeLog: u32,
) -> *mut ldmEntry_t {
    ((*ldmState).hashTable).offset((hash << bucketSizeLog) as isize)
}
unsafe extern "C" fn ZSTD_ldm_insertEntry(
    mut ldmState: *mut ldmState_t,
    hash: size_t,
    entry: ldmEntry_t,
    bucketSizeLog: u32,
) {
    let pOffset = ((*ldmState).bucketOffsets).offset(hash as isize);
    let offset = *pOffset as std::ffi::c_uint;
    *(ZSTD_ldm_getBucket(ldmState, hash, bucketSizeLog)).offset(offset as isize) = entry;
    *pOffset = (offset.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
        & ((1 as std::ffi::c_uint) << bucketSizeLog)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)) as u8;
}
unsafe extern "C" fn ZSTD_ldm_countBackwardsMatch(
    mut pIn: *const u8,
    mut pAnchor: *const u8,
    mut pMatch: *const u8,
    mut pMatchBase: *const u8,
) -> size_t {
    let mut matchLength = 0 as std::ffi::c_int as size_t;
    while pIn > pAnchor
        && pMatch > pMatchBase
        && *pIn.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int
            == *pMatch.offset(-(1 as std::ffi::c_int) as isize) as std::ffi::c_int
    {
        pIn = pIn.offset(-1);
        pIn;
        pMatch = pMatch.offset(-1);
        pMatch;
        matchLength = matchLength.wrapping_add(1);
        matchLength;
    }
    matchLength
}
unsafe extern "C" fn ZSTD_ldm_countBackwardsMatch_2segments(
    mut pIn: *const u8,
    mut pAnchor: *const u8,
    mut pMatch: *const u8,
    mut pMatchBase: *const u8,
    mut pExtDictStart: *const u8,
    mut pExtDictEnd: *const u8,
) -> size_t {
    let mut matchLength = ZSTD_ldm_countBackwardsMatch(pIn, pAnchor, pMatch, pMatchBase);
    if pMatch.offset(-(matchLength as isize)) != pMatchBase || pMatchBase == pExtDictStart {
        return matchLength;
    }
    matchLength = matchLength.wrapping_add(ZSTD_ldm_countBackwardsMatch(
        pIn.offset(-(matchLength as isize)),
        pAnchor,
        pExtDictEnd,
        pExtDictStart,
    ));
    matchLength
}
unsafe extern "C" fn ZSTD_ldm_fillFastTables(
    mut ms: *mut ZSTD_MatchState_t,
    mut end: *const std::ffi::c_void,
) -> size_t {
    let iend = end as *const u8;
    match (*ms).cParams.strategy as std::ffi::c_uint {
        1 => {
            ZSTD_fillHashTable(
                ms,
                iend as *const std::ffi::c_void,
                ZSTD_dtlm_fast,
                ZSTD_tfp_forCCtx,
            );
        }
        2 => {
            ZSTD_fillDoubleHashTable(
                ms,
                iend as *const std::ffi::c_void,
                ZSTD_dtlm_fast,
                ZSTD_tfp_forCCtx,
            );
        }
        3 | 4 | 5 | 6 | 7 | 8 | 9 | _ => {}
    }
    0 as std::ffi::c_int as size_t
}
#[export_name = crate::prefix!(ZSTD_ldm_fillHashTable)]
pub unsafe extern "C" fn ZSTD_ldm_fillHashTable(
    mut ldmState: *mut ldmState_t,
    mut ip: *const u8,
    mut iend: *const u8,
    mut params: *const ldmParams_t,
) {
    let minMatchLength = (*params).minMatchLength;
    let bucketSizeLog = (*params).bucketSizeLog;
    let hBits = ((*params).hashLog).wrapping_sub(bucketSizeLog);
    let base = (*ldmState).window.base;
    let istart = ip;
    let mut hashState = ldmRollingHashState_t {
        rolling: 0,
        stopMask: 0,
    };
    let splits = ((*ldmState).splitIndices).as_mut_ptr();
    let mut numSplits: std::ffi::c_uint = 0;
    ZSTD_ldm_gear_init(&mut hashState, params);
    while ip < iend {
        let mut hashed: size_t = 0;
        let mut n: std::ffi::c_uint = 0;
        numSplits = 0 as std::ffi::c_int as std::ffi::c_uint;
        hashed = ZSTD_ldm_gear_feed(
            &mut hashState,
            ip,
            iend.offset_from(ip) as std::ffi::c_long as size_t,
            splits,
            &mut numSplits,
        );
        n = 0 as std::ffi::c_int as std::ffi::c_uint;
        while n < numSplits {
            if ip.offset(*splits.offset(n as isize) as isize)
                >= istart.offset(minMatchLength as isize)
            {
                let split = ip
                    .offset(*splits.offset(n as isize) as isize)
                    .offset(-(minMatchLength as isize));
                let xxhash = ZSTD_XXH64(
                    split as *const std::ffi::c_void,
                    minMatchLength as usize,
                    0 as std::ffi::c_int as XXH64_hash_t,
                );
                let hash = (xxhash
                    & ((1 as std::ffi::c_int as u32) << hBits)
                        .wrapping_sub(1 as std::ffi::c_int as u32) as u64)
                    as u32;
                let mut entry = ldmEntry_t {
                    offset: 0,
                    checksum: 0,
                };
                entry.offset = split.offset_from(base) as std::ffi::c_long as u32;
                entry.checksum = (xxhash >> 32 as std::ffi::c_int) as u32;
                ZSTD_ldm_insertEntry(ldmState, hash as size_t, entry, (*params).bucketSizeLog);
            }
            n = n.wrapping_add(1);
            n;
        }
        ip = ip.offset(hashed as isize);
    }
}
unsafe extern "C" fn ZSTD_ldm_limitTableUpdate(
    mut ms: *mut ZSTD_MatchState_t,
    mut anchor: *const u8,
) {
    let curr = anchor.offset_from((*ms).window.base) as std::ffi::c_long as u32;
    if curr > ((*ms).nextToUpdate).wrapping_add(1024 as std::ffi::c_int as u32) {
        (*ms).nextToUpdate = curr.wrapping_sub(
            if (512 as std::ffi::c_int as u32)
                < curr
                    .wrapping_sub((*ms).nextToUpdate)
                    .wrapping_sub(1024 as std::ffi::c_int as u32)
            {
                512 as std::ffi::c_int as u32
            } else {
                curr.wrapping_sub((*ms).nextToUpdate)
                    .wrapping_sub(1024 as std::ffi::c_int as u32)
            },
        );
    }
}
unsafe extern "C" fn ZSTD_ldm_generateSequences_internal(
    mut ldmState: *mut ldmState_t,
    mut rawSeqStore: *mut RawSeqStore_t,
    mut params: *const ldmParams_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let extDict = ZSTD_window_hasExtDict((*ldmState).window) as std::ffi::c_int;
    let minMatchLength = (*params).minMatchLength;
    let entsPerBucket = (1 as std::ffi::c_uint) << (*params).bucketSizeLog;
    let hBits = ((*params).hashLog).wrapping_sub((*params).bucketSizeLog);
    let dictLimit = (*ldmState).window.dictLimit;
    let lowestIndex = if extDict != 0 {
        (*ldmState).window.lowLimit
    } else {
        dictLimit
    };
    let base = (*ldmState).window.base;
    let dictBase = if extDict != 0 {
        (*ldmState).window.dictBase
    } else {
        NULL as *const u8
    };
    let dictStart = if extDict != 0 {
        dictBase.offset(lowestIndex as isize)
    } else {
        NULL as *const u8
    };
    let dictEnd = if extDict != 0 {
        dictBase.offset(dictLimit as isize)
    } else {
        NULL as *const u8
    };
    let lowPrefixPtr = base.offset(dictLimit as isize);
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut anchor = istart;
    let mut ip = istart;
    let mut hashState = ldmRollingHashState_t {
        rolling: 0,
        stopMask: 0,
    };
    let splits = ((*ldmState).splitIndices).as_mut_ptr();
    let candidates = ((*ldmState).matchCandidates).as_mut_ptr();
    let mut numSplits: std::ffi::c_uint = 0;
    if srcSize < minMatchLength as size_t {
        return iend.offset_from(anchor) as std::ffi::c_long as size_t;
    }
    ZSTD_ldm_gear_init(&mut hashState, params);
    ZSTD_ldm_gear_reset(&mut hashState, ip, minMatchLength as size_t);
    ip = ip.offset(minMatchLength as isize);
    while ip < ilimit {
        let mut hashed: size_t = 0;
        let mut n: std::ffi::c_uint = 0;
        numSplits = 0 as std::ffi::c_int as std::ffi::c_uint;
        hashed = ZSTD_ldm_gear_feed(
            &mut hashState,
            ip,
            ilimit.offset_from(ip) as std::ffi::c_long as size_t,
            splits,
            &mut numSplits,
        );
        n = 0 as std::ffi::c_int as std::ffi::c_uint;
        while n < numSplits {
            let split = ip
                .offset(*splits.offset(n as isize) as isize)
                .offset(-(minMatchLength as isize));
            let xxhash = ZSTD_XXH64(
                split as *const std::ffi::c_void,
                minMatchLength as usize,
                0 as std::ffi::c_int as XXH64_hash_t,
            );
            let hash = (xxhash
                & ((1 as std::ffi::c_int as u32) << hBits).wrapping_sub(1 as std::ffi::c_int as u32)
                    as u64) as u32;
            let fresh2 = &mut (*candidates.offset(n as isize)).split;
            *fresh2 = split;
            (*candidates.offset(n as isize)).hash = hash;
            (*candidates.offset(n as isize)).checksum = (xxhash >> 32 as std::ffi::c_int) as u32;
            let fresh3 = &mut (*candidates.offset(n as isize)).bucket;
            *fresh3 = ZSTD_ldm_getBucket(ldmState, hash as size_t, (*params).bucketSizeLog);
            n = n.wrapping_add(1);
            n;
        }
        n = 0 as std::ffi::c_int as std::ffi::c_uint;
        while n < numSplits {
            let mut forwardMatchLength = 0 as std::ffi::c_int as size_t;
            let mut backwardMatchLength = 0 as std::ffi::c_int as size_t;
            let mut bestMatchLength = 0 as std::ffi::c_int as size_t;
            let mut mLength: size_t = 0;
            let mut offset: u32 = 0;
            let split_0 = (*candidates.offset(n as isize)).split;
            let checksum = (*candidates.offset(n as isize)).checksum;
            let hash_0 = (*candidates.offset(n as isize)).hash;
            let bucket = (*candidates.offset(n as isize)).bucket;
            let mut cur = std::ptr::null::<ldmEntry_t>();
            let mut bestEntry = NULL as *const ldmEntry_t;
            let mut newEntry = ldmEntry_t {
                offset: 0,
                checksum: 0,
            };
            newEntry.offset = split_0.offset_from(base) as std::ffi::c_long as u32;
            newEntry.checksum = checksum;
            if split_0 < anchor {
                ZSTD_ldm_insertEntry(
                    ldmState,
                    hash_0 as size_t,
                    newEntry,
                    (*params).bucketSizeLog,
                );
            } else {
                let mut current_block_30: u64;
                cur = bucket;
                while cur < bucket.offset(entsPerBucket as isize) as *const ldmEntry_t {
                    let mut curForwardMatchLength: size_t = 0;
                    let mut curBackwardMatchLength: size_t = 0;
                    let mut curTotalMatchLength: size_t = 0;
                    if !((*cur).checksum != checksum || (*cur).offset <= lowestIndex) {
                        if extDict != 0 {
                            let curMatchBase = if (*cur).offset < dictLimit {
                                dictBase
                            } else {
                                base
                            };
                            let pMatch = curMatchBase.offset((*cur).offset as isize);
                            let matchEnd = if (*cur).offset < dictLimit {
                                dictEnd
                            } else {
                                iend
                            };
                            let lowMatchPtr = if (*cur).offset < dictLimit {
                                dictStart
                            } else {
                                lowPrefixPtr
                            };
                            curForwardMatchLength =
                                ZSTD_count_2segments(split_0, pMatch, iend, matchEnd, lowPrefixPtr);
                            if curForwardMatchLength < minMatchLength as size_t {
                                current_block_30 = 17788412896529399552;
                            } else {
                                curBackwardMatchLength = ZSTD_ldm_countBackwardsMatch_2segments(
                                    split_0,
                                    anchor,
                                    pMatch,
                                    lowMatchPtr,
                                    dictStart,
                                    dictEnd,
                                );
                                current_block_30 = 15512526488502093901;
                            }
                        } else {
                            let pMatch_0 = base.offset((*cur).offset as isize);
                            curForwardMatchLength = ZSTD_count(split_0, pMatch_0, iend);
                            if curForwardMatchLength < minMatchLength as size_t {
                                current_block_30 = 17788412896529399552;
                            } else {
                                curBackwardMatchLength = ZSTD_ldm_countBackwardsMatch(
                                    split_0,
                                    anchor,
                                    pMatch_0,
                                    lowPrefixPtr,
                                );
                                current_block_30 = 15512526488502093901;
                            }
                        }
                        match current_block_30 {
                            17788412896529399552 => {}
                            _ => {
                                curTotalMatchLength =
                                    curForwardMatchLength.wrapping_add(curBackwardMatchLength);
                                if curTotalMatchLength > bestMatchLength {
                                    bestMatchLength = curTotalMatchLength;
                                    forwardMatchLength = curForwardMatchLength;
                                    backwardMatchLength = curBackwardMatchLength;
                                    bestEntry = cur;
                                }
                            }
                        }
                    }
                    cur = cur.offset(1);
                    cur;
                }
                if bestEntry.is_null() {
                    ZSTD_ldm_insertEntry(
                        ldmState,
                        hash_0 as size_t,
                        newEntry,
                        (*params).bucketSizeLog,
                    );
                } else {
                    offset = (split_0.offset_from(base) as std::ffi::c_long as u32)
                        .wrapping_sub((*bestEntry).offset);
                    mLength = forwardMatchLength.wrapping_add(backwardMatchLength);
                    let seq = ((*rawSeqStore).seq).offset((*rawSeqStore).size as isize);
                    if (*rawSeqStore).size == (*rawSeqStore).capacity {
                        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
                    }
                    (*seq).litLength = split_0
                        .offset(-(backwardMatchLength as isize))
                        .offset_from(anchor)
                        as std::ffi::c_long as u32;
                    (*seq).matchLength = mLength as u32;
                    (*seq).offset = offset;
                    (*rawSeqStore).size = ((*rawSeqStore).size).wrapping_add(1);
                    (*rawSeqStore).size;
                    ZSTD_ldm_insertEntry(
                        ldmState,
                        hash_0 as size_t,
                        newEntry,
                        (*params).bucketSizeLog,
                    );
                    anchor = split_0.offset(forwardMatchLength as isize);
                    if anchor > ip.offset(hashed as isize) {
                        ZSTD_ldm_gear_reset(
                            &mut hashState,
                            anchor.offset(-(minMatchLength as isize)),
                            minMatchLength as size_t,
                        );
                        ip = anchor.offset(-(hashed as isize));
                        break;
                    }
                }
            }
            n = n.wrapping_add(1);
            n;
        }
        ip = ip.offset(hashed as isize);
    }
    iend.offset_from(anchor) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTD_ldm_reduceTable(table: *mut ldmEntry_t, size: u32, reducerValue: u32) {
    let mut u: u32 = 0;
    u = 0 as std::ffi::c_int as u32;
    while u < size {
        if (*table.offset(u as isize)).offset < reducerValue {
            (*table.offset(u as isize)).offset = 0 as std::ffi::c_int as u32;
        } else {
            let fresh4 = &mut (*table.offset(u as isize)).offset;
            *fresh4 = (*fresh4).wrapping_sub(reducerValue);
        }
        u = u.wrapping_add(1);
        u;
    }
}
#[export_name = crate::prefix!(ZSTD_ldm_generateSequences)]
pub unsafe extern "C" fn ZSTD_ldm_generateSequences(
    mut ldmState: *mut ldmState_t,
    mut sequences: *mut RawSeqStore_t,
    mut params: *const ldmParams_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let maxDist = (1 as std::ffi::c_uint) << (*params).windowLog;
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let kMaxChunkSize = ((1 as std::ffi::c_int) << 20 as std::ffi::c_int) as size_t;
    let nbChunks = (srcSize / kMaxChunkSize).wrapping_add(
        (srcSize % kMaxChunkSize != 0 as std::ffi::c_int as size_t) as std::ffi::c_int as size_t,
    );
    let mut chunk: size_t = 0;
    let mut leftoverSize = 0 as std::ffi::c_int as size_t;
    chunk = 0 as std::ffi::c_int as size_t;
    while chunk < nbChunks && (*sequences).size < (*sequences).capacity {
        let chunkStart = istart.offset((chunk * kMaxChunkSize) as isize);
        let remaining = iend.offset_from(chunkStart) as std::ffi::c_long as size_t;
        let chunkEnd = if remaining < kMaxChunkSize {
            iend
        } else {
            chunkStart.offset(kMaxChunkSize as isize)
        };
        let chunkSize = chunkEnd.offset_from(chunkStart) as std::ffi::c_long as size_t;
        let mut newLeftoverSize: size_t = 0;
        let prevSize = (*sequences).size;
        if ZSTD_window_needOverflowCorrection(
            (*ldmState).window,
            0 as std::ffi::c_int as u32,
            maxDist,
            (*ldmState).loadedDictEnd,
            chunkStart as *const std::ffi::c_void,
            chunkEnd as *const std::ffi::c_void,
        ) != 0
        {
            let ldmHSize = (1 as std::ffi::c_uint) << (*params).hashLog;
            let correction = ZSTD_window_correctOverflow(
                &mut (*ldmState).window,
                0 as std::ffi::c_int as u32,
                maxDist,
                chunkStart as *const std::ffi::c_void,
            );
            ZSTD_ldm_reduceTable((*ldmState).hashTable, ldmHSize, correction);
            (*ldmState).loadedDictEnd = 0 as std::ffi::c_int as u32;
        }
        ZSTD_window_enforceMaxDist(
            &mut (*ldmState).window,
            chunkEnd as *const std::ffi::c_void,
            maxDist,
            &mut (*ldmState).loadedDictEnd,
            NULL as *mut *const ZSTD_MatchState_t,
        );
        newLeftoverSize = ZSTD_ldm_generateSequences_internal(
            ldmState,
            sequences,
            params,
            chunkStart as *const std::ffi::c_void,
            chunkSize,
        );
        if ERR_isError(newLeftoverSize) != 0 {
            return newLeftoverSize;
        }
        if prevSize < (*sequences).size {
            let fresh5 = &mut (*((*sequences).seq).offset(prevSize as isize)).litLength;
            *fresh5 = (*fresh5).wrapping_add(leftoverSize as u32);
            leftoverSize = newLeftoverSize;
        } else {
            leftoverSize = leftoverSize.wrapping_add(chunkSize);
        }
        chunk = chunk.wrapping_add(1);
        chunk;
    }
    0 as std::ffi::c_int as size_t
}
#[export_name = crate::prefix!(ZSTD_ldm_skipSequences)]
pub unsafe extern "C" fn ZSTD_ldm_skipSequences(
    mut rawSeqStore: *mut RawSeqStore_t,
    mut srcSize: size_t,
    minMatch: u32,
) {
    while srcSize > 0 as std::ffi::c_int as size_t && (*rawSeqStore).pos < (*rawSeqStore).size {
        let mut seq = ((*rawSeqStore).seq).offset((*rawSeqStore).pos as isize);
        if srcSize <= (*seq).litLength as size_t {
            (*seq).litLength = ((*seq).litLength).wrapping_sub(srcSize as u32);
            return;
        }
        srcSize = srcSize.wrapping_sub((*seq).litLength as size_t);
        (*seq).litLength = 0 as std::ffi::c_int as u32;
        if srcSize < (*seq).matchLength as size_t {
            (*seq).matchLength = ((*seq).matchLength).wrapping_sub(srcSize as u32);
            if (*seq).matchLength < minMatch {
                if ((*rawSeqStore).pos).wrapping_add(1 as std::ffi::c_int as size_t)
                    < (*rawSeqStore).size
                {
                    let fresh6 = &mut (*seq.offset(1 as std::ffi::c_int as isize)).litLength;
                    *fresh6 = (*fresh6)
                        .wrapping_add((*seq.offset(0 as std::ffi::c_int as isize)).matchLength);
                }
                (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
                (*rawSeqStore).pos;
            }
            return;
        }
        srcSize = srcSize.wrapping_sub((*seq).matchLength as size_t);
        (*seq).matchLength = 0 as std::ffi::c_int as u32;
        (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
        (*rawSeqStore).pos;
    }
}
unsafe extern "C" fn maybeSplitSequence(
    mut rawSeqStore: *mut RawSeqStore_t,
    remaining: u32,
    minMatch: u32,
) -> rawSeq {
    let mut sequence = *((*rawSeqStore).seq).offset((*rawSeqStore).pos as isize);
    if remaining >= (sequence.litLength).wrapping_add(sequence.matchLength) {
        (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
        (*rawSeqStore).pos;
        return sequence;
    }
    if remaining <= sequence.litLength {
        sequence.offset = 0 as std::ffi::c_int as u32;
    } else if remaining < (sequence.litLength).wrapping_add(sequence.matchLength) {
        sequence.matchLength = remaining.wrapping_sub(sequence.litLength);
        if sequence.matchLength < minMatch {
            sequence.offset = 0 as std::ffi::c_int as u32;
        }
    }
    ZSTD_ldm_skipSequences(rawSeqStore, remaining as size_t, minMatch);
    sequence
}
#[export_name = crate::prefix!(ZSTD_ldm_skipRawSeqStoreBytes)]
pub unsafe extern "C" fn ZSTD_ldm_skipRawSeqStoreBytes(
    mut rawSeqStore: *mut RawSeqStore_t,
    mut nbBytes: size_t,
) {
    let mut currPos = ((*rawSeqStore).posInSequence).wrapping_add(nbBytes) as u32;
    while currPos != 0 && (*rawSeqStore).pos < (*rawSeqStore).size {
        let mut currSeq = *((*rawSeqStore).seq).offset((*rawSeqStore).pos as isize);
        if currPos >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            currPos = currPos.wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength));
            (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
            (*rawSeqStore).pos;
        } else {
            (*rawSeqStore).posInSequence = currPos as size_t;
            break;
        }
    }
    if currPos == 0 as std::ffi::c_int as u32 || (*rawSeqStore).pos == (*rawSeqStore).size {
        (*rawSeqStore).posInSequence = 0 as std::ffi::c_int as size_t;
    }
}
#[export_name = crate::prefix!(ZSTD_ldm_blockCompress)]
pub unsafe extern "C" fn ZSTD_ldm_blockCompress(
    mut rawSeqStore: *mut RawSeqStore_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut u32,
    mut useRowMatchFinder: ZSTD_ParamSwitch_e,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let minMatch = (*cParams).minMatch;
    let blockCompressor = ZSTD_selectBlockCompressor(
        (*cParams).strategy,
        useRowMatchFinder,
        ZSTD_matchState_dictMode(ms),
    );
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let mut ip = istart;
    if (*cParams).strategy as std::ffi::c_uint >= ZSTD_btopt as std::ffi::c_int as std::ffi::c_uint
    {
        let mut lastLLSize: size_t = 0;
        (*ms).ldmSeqStore = rawSeqStore;
        lastLLSize = blockCompressor.unwrap_unchecked()(ms, seqStore, rep, src, srcSize);
        ZSTD_ldm_skipRawSeqStoreBytes(rawSeqStore, srcSize);
        return lastLLSize;
    }
    while (*rawSeqStore).pos < (*rawSeqStore).size && ip < iend {
        let sequence = maybeSplitSequence(
            rawSeqStore,
            iend.offset_from(ip) as std::ffi::c_long as u32,
            minMatch,
        );
        if sequence.offset == 0 as std::ffi::c_int as u32 {
            break;
        }
        ZSTD_ldm_limitTableUpdate(ms, ip);
        ZSTD_ldm_fillFastTables(ms, ip as *const std::ffi::c_void);
        let mut i: std::ffi::c_int = 0;
        let newLitLength = blockCompressor.unwrap_unchecked()(
            ms,
            seqStore,
            rep,
            ip as *const std::ffi::c_void,
            sequence.litLength as size_t,
        );
        ip = ip.offset(sequence.litLength as isize);
        i = ZSTD_REP_NUM - 1 as std::ffi::c_int;
        while i > 0 as std::ffi::c_int {
            *rep.offset(i as isize) = *rep.offset((i - 1 as std::ffi::c_int) as isize);
            i -= 1;
            i;
        }
        *rep.offset(0 as std::ffi::c_int as isize) = sequence.offset;
        ZSTD_storeSeq(
            seqStore,
            newLitLength,
            ip.offset(-(newLitLength as isize)),
            iend,
            (sequence.offset).wrapping_add(ZSTD_REP_NUM as u32),
            sequence.matchLength as size_t,
        );
        ip = ip.offset(sequence.matchLength as isize);
    }
    ZSTD_ldm_limitTableUpdate(ms, ip);
    ZSTD_ldm_fillFastTables(ms, ip as *const std::ffi::c_void);
    blockCompressor.unwrap_unchecked()(
        ms,
        seqStore,
        rep,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
    )
}
