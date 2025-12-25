use crate::internal::MEM_readLE32;
use crate::lib::common::bits::ZSTD_NbCommonBytes;
use crate::lib::common::mem::{MEM_64bits, MEM_read16, MEM_read32, MEM_readLE64, MEM_readST};
use crate::lib::common::zstd_internal::{
    Overlap, ZSTD_copy16, ZSTD_wildcopy, MINMATCH, WILDCOPY_OVERLENGTH, ZSTD_REP_NUM,
};
use crate::lib::compress::zstd_compress::{SeqDef, SeqStore_t, ZSTD_MatchState_t, ZSTD_window_t};
use crate::lib::compress::zstd_compress_superblock::ZSTD_SequenceLength;

pub(crate) type ZSTD_longLengthType_e = core::ffi::c_uint;
pub(crate) const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub(crate) const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub(crate) const ZSTD_llt_none: ZSTD_longLengthType_e = 0;

pub(crate) const ZSTD_CURRENT_MAX: usize = if MEM_64bits() {
    3500 * (1 << 20)
} else {
    2000 * (1 << 20)
};

pub(crate) unsafe fn ZSTD_getSequenceLength(
    seqStore: *const SeqStore_t,
    seq: *const SeqDef,
) -> ZSTD_SequenceLength {
    let mut seqLen = ZSTD_SequenceLength {
        litLength: u32::from((*seq).litLength),
        matchLength: u32::from((*seq).mlBase) + MINMATCH as u32,
    };

    if (*seqStore).longLengthPos == (seq as usize - (*seqStore).sequencesStart as usize) as u32 {
        if (*seqStore).longLengthType == ZSTD_llt_literalLength {
            seqLen.litLength += 0x10000;
        }
        if (*seqStore).longLengthType == ZSTD_llt_matchLength {
            seqLen.matchLength += 0x10000;
        }
    }

    seqLen
}

pub type ZSTD_OptPrice_e = core::ffi::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;

#[inline(always)]
pub(crate) unsafe fn ZSTD_storeSeqOnly(
    seqStorePtr: &mut SeqStore_t,
    litLength: usize,
    offBase: u32,
    matchLength: usize,
) {
    if core::ffi::c_long::from(core::ffi::c_int::from(
        litLength > 0xffff as core::ffi::c_int as usize,
    )) != 0
    {
        seqStorePtr.longLengthType = ZSTD_llt_literalLength;
        seqStorePtr.longLengthPos = (seqStorePtr.sequences).offset_from(seqStorePtr.sequencesStart)
            as core::ffi::c_long as u32;
    }
    (*(seqStorePtr.sequences)).litLength = litLength as u16;
    (*(seqStorePtr.sequences)).offBase = offBase;
    let mlBase = matchLength.wrapping_sub(MINMATCH as usize);
    if mlBase > 0xffff {
        seqStorePtr.longLengthType = ZSTD_llt_matchLength;
        seqStorePtr.longLengthPos = (seqStorePtr.sequences).offset_from(seqStorePtr.sequencesStart)
            as core::ffi::c_long as u32;
    }
    (*(seqStorePtr.sequences)).mlBase = mlBase as u16;
    seqStorePtr.sequences = (seqStorePtr.sequences).add(1);
}
#[inline(always)]
pub(crate) unsafe fn ZSTD_storeSeq(
    seqStorePtr: &mut SeqStore_t,
    litLength: usize,
    literals: *const u8,
    litLimit: *const u8,
    offBase: u32,
    matchLength: usize,
) {
    let litLimit_w = litLimit.wrapping_sub(WILDCOPY_OVERLENGTH);
    let litEnd = literals.add(litLength);
    if litEnd <= litLimit_w {
        ZSTD_copy16(seqStorePtr.lit, literals);
        if litLength > 16 {
            ZSTD_wildcopy(
                seqStorePtr.lit.add(16),
                literals.add(16),
                litLength.wrapping_sub(16),
                Overlap::NoOverlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals(seqStorePtr.lit, literals, litEnd, litLimit_w);
    }
    seqStorePtr.lit = (seqStorePtr.lit).add(litLength);
    ZSTD_storeSeqOnly(seqStorePtr, litLength, offBase, matchLength);
}

#[inline]
pub(crate) unsafe fn ZSTD_updateRep(rep: *mut u32, offBase: u32, ll0: u32) {
    if offBase > ZSTD_REP_NUM as u32 {
        *rep.add(2) = *rep.add(1);
        *rep.add(1) = *rep;
        *rep = offBase.wrapping_sub(ZSTD_REP_NUM as u32);
    } else {
        let repCode = offBase.wrapping_sub(1).wrapping_add(ll0);
        if repCode > 0 {
            let currentOffset = if repCode == ZSTD_REP_NUM as u32 {
                (*rep).wrapping_sub(1)
            } else {
                *rep.offset(repCode as isize)
            };
            *rep.add(2) = if repCode >= 2 {
                *rep.add(1)
            } else {
                *rep.add(2)
            };
            *rep.add(1) = *rep;
            *rep = currentOffset;
        }
    };
}

pub(crate) unsafe fn ZSTD_safecopyLiterals(
    mut op: *mut u8,
    mut ip: *const u8,
    iend: *const u8,
    ilimit_w: *const u8,
) {
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op,
            ip,
            ilimit_w.offset_from_unsigned(ip),
            Overlap::NoOverlap,
        );
        op = op.offset(ilimit_w.offset_from(ip));
        ip = ilimit_w;
    }
    while ip < iend {
        let fresh0 = ip;
        ip = ip.add(1);
        let fresh1 = op;
        op = op.add(1);
        *fresh1 = *fresh0;
    }
}

#[inline]
pub(crate) unsafe fn ZSTD_count(
    mut pIn: *const u8,
    mut pMatch: *const u8,
    pInLimit: *const u8,
) -> usize {
    let pStart = pIn;
    let pInLoopLimit =
        pInLimit.offset(-((::core::mem::size_of::<usize>()).wrapping_sub(1) as isize));
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const core::ffi::c_void)
            ^ MEM_readST(pIn as *const core::ffi::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as usize;
        }
        pIn = pIn.add(::core::mem::size_of::<usize>());
        pMatch = pMatch.add(::core::mem::size_of::<usize>());
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const core::ffi::c_void)
                ^ MEM_readST(pIn as *const core::ffi::c_void);
            if diff_0 == 0 {
                pIn = pIn.add(::core::mem::size_of::<usize>());
                pMatch = pMatch.add(::core::mem::size_of::<usize>());
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from_unsigned(pStart);
            }
        }
    }
    if MEM_64bits()
        && pIn < pInLimit.sub(3)
        && MEM_read32(pMatch as *const core::ffi::c_void)
            == MEM_read32(pIn as *const core::ffi::c_void)
    {
        pIn = pIn.add(4);
        pMatch = pMatch.add(4);
    }
    if pIn < pInLimit.sub(1)
        && core::ffi::c_int::from(MEM_read16(pMatch as *const core::ffi::c_void))
            == core::ffi::c_int::from(MEM_read16(pIn as *const core::ffi::c_void))
    {
        pIn = pIn.add(2);
        pMatch = pMatch.add(2);
    }
    if pIn < pInLimit && core::ffi::c_int::from(*pMatch) == core::ffi::c_int::from(*pIn) {
        pIn = pIn.add(1);
    }
    pIn.offset_from_unsigned(pStart)
}

#[inline]
pub(crate) unsafe fn ZSTD_count_2segments(
    ip: *const u8,
    match_0: *const u8,
    iEnd: *const u8,
    mEnd: *const u8,
    iStart: *const u8,
) -> usize {
    let vEnd = if ip.wrapping_offset(mEnd.offset_from(match_0) as core::ffi::c_long as isize) < iEnd
    {
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

const prime3bytes: u32 = 506832829;
const fn ZSTD_hash3(u: u32, h: u32, s: u32) -> u32 {
    (((u << (32 as core::ffi::c_int - 24 as core::ffi::c_int)).wrapping_mul(prime3bytes)) ^ s)
        >> 32u32.wrapping_sub(h)
}
#[inline]
pub(crate) unsafe fn ZSTD_hash3Ptr(ptr: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash3(MEM_readLE32(ptr), h, 0) as usize
}

const prime4bytes: u32 = 2654435761;
const fn ZSTD_hash4(u: u32, h: u32, s: u32) -> u32 {
    ((u.wrapping_mul(prime4bytes)) ^ s) >> 32u32.wrapping_sub(h)
}
unsafe fn ZSTD_hash4Ptr(ptr: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0) as usize
}
unsafe fn ZSTD_hash4PtrS(ptr: *const core::ffi::c_void, h: u32, s: u32) -> usize {
    ZSTD_hash4(MEM_readLE32(ptr), h, s) as usize
}

const prime5bytes: u64 = 889523592379;
const fn ZSTD_hash5(u: u64, h: u32, s: u64) -> usize {
    ((((u << (64 - 40)).wrapping_mul(prime5bytes)) ^ s) >> 64u32.wrapping_sub(h)) as usize
}
unsafe fn ZSTD_hash5Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash5(MEM_readLE64(p), h, 0)
}
unsafe fn ZSTD_hash5PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash5(MEM_readLE64(p), h, s)
}

const prime6bytes: u64 = 227718039650203;
const fn ZSTD_hash6(u: u64, h: u32, s: u64) -> usize {
    ((((u << (64 - 48)).wrapping_mul(prime6bytes)) ^ s) >> 64u32.wrapping_sub(h)) as usize
}
pub(crate) unsafe fn ZSTD_hash6Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash6(MEM_readLE64(p), h, 0)
}
pub(crate) fn ZSTD_hash6Ptr_array(p: &[u8; 8], h: u32) -> usize {
    ZSTD_hash6(u64::from_le_bytes(*p), h, 0)
}
unsafe fn ZSTD_hash6PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash6(MEM_readLE64(p), h, s)
}

const prime7bytes: u64 = 58295818150454627;
const fn ZSTD_hash7(u: u64, h: u32, s: u64) -> usize {
    ((((u << (64 - 56)).wrapping_mul(prime7bytes)) ^ s) >> (64u32).wrapping_sub(h)) as usize
}
unsafe fn ZSTD_hash7Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash7(MEM_readLE64(p), h, 0)
}
unsafe fn ZSTD_hash7PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash7(MEM_readLE64(p), h, s)
}

const prime8bytes: u64 = 0xcf1bbcdcb7a56463 as core::ffi::c_ulonglong;
const fn ZSTD_hash8(u: u64, h: u32, s: u64) -> usize {
    (((u.wrapping_mul(prime8bytes)) ^ s) >> 64u32.wrapping_sub(h)) as usize
}
pub(crate) unsafe fn ZSTD_hash8Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash8(MEM_readLE64(p), h, 0)
}
pub(crate) fn ZSTD_hash8Ptr_array(p: &[u8; 8], h: u32) -> usize {
    ZSTD_hash8(u64::from_le_bytes(*p), h, 0)
}
unsafe fn ZSTD_hash8PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash8(MEM_readLE64(p), h, s)
}

#[inline(always)]
pub(crate) unsafe fn ZSTD_hashPtr(p: *const core::ffi::c_void, hBits: u32, mls: u32) -> usize {
    match mls {
        5 => ZSTD_hash5Ptr(p, hBits),
        6 => ZSTD_hash6Ptr(p, hBits),
        7 => ZSTD_hash7Ptr(p, hBits),
        8 => ZSTD_hash8Ptr(p, hBits),
        _ => ZSTD_hash4Ptr(p, hBits),
    }
}

#[inline(always)]
pub(crate) unsafe fn ZSTD_hashPtrSalted(
    p: *const core::ffi::c_void,
    hBits: u32,
    mls: u32,
    hashSalt: u64,
) -> usize {
    match mls {
        5 => ZSTD_hash5PtrS(p, hBits, hashSalt),
        6 => ZSTD_hash6PtrS(p, hBits, hashSalt),
        7 => ZSTD_hash7PtrS(p, hBits, hashSalt),
        8 => ZSTD_hash8PtrS(p, hBits, hashSalt),
        4 | _ => ZSTD_hash4PtrS(p, hBits, hashSalt as u32),
    }
}

#[inline]
pub(crate) unsafe fn ZSTD_getLowestMatchIndex(
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
    let isDictionary = core::ffi::c_int::from((*ms).loadedDictEnd != 0) as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}

#[inline]
pub(crate) unsafe fn ZSTD_getLowestPrefixIndex(
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
    let isDictionary = core::ffi::c_int::from((*ms).loadedDictEnd != 0) as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}

#[inline]
pub(crate) fn ZSTD_index_overlap_check(prefixLowestIndex: u32, repIndex: u32) -> core::ffi::c_int {
    core::ffi::c_int::from(prefixLowestIndex.wrapping_sub(1).wrapping_sub(repIndex) >= 3)
}

pub const ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY: core::ffi::c_int = 0;

#[inline]
pub(crate) unsafe fn ZSTD_window_needOverflowCorrection(
    window: ZSTD_window_t,
    cycleLog: u32,
    maxDist: u32,
    loadedDictEnd: u32,
    src: *const core::ffi::c_void,
    srcEnd: *const core::ffi::c_void,
) -> bool {
    if ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY != 0 {
        if ZSTD_window_canOverflowCorrect(window, cycleLog, maxDist, loadedDictEnd, src) != 0 {
            return true;
        }
    }

    let curr = srcEnd.addr() - window.base.addr();
    curr > ZSTD_CURRENT_MAX
}

pub const ZSTD_WINDOW_START_INDEX: core::ffi::c_int = 2;

#[inline]
unsafe fn ZSTD_window_canOverflowCorrect(
    window: ZSTD_window_t,
    cycleLog: u32,
    maxDist: u32,
    loadedDictEnd: u32,
    src: *const core::ffi::c_void,
) -> u32 {
    let cycleSize = (1 as core::ffi::c_uint) << cycleLog;
    let curr = (src as *const u8).offset_from(window.base) as core::ffi::c_long as u32;
    let minIndexToOverflowCorrect = cycleSize
        .wrapping_add(if maxDist > cycleSize {
            maxDist
        } else {
            cycleSize
        })
        .wrapping_add(ZSTD_WINDOW_START_INDEX as u32);
    let adjustment = (window.nbOverflowCorrections).wrapping_add(1);
    let adjustedIndex = if minIndexToOverflowCorrect * adjustment > minIndexToOverflowCorrect {
        minIndexToOverflowCorrect * adjustment
    } else {
        minIndexToOverflowCorrect
    };
    let indexLargeEnough = core::ffi::c_int::from(curr > adjustedIndex) as u32;
    let dictionaryInvalidated =
        core::ffi::c_int::from(curr > maxDist.wrapping_add(loadedDictEnd)) as u32;
    core::ffi::c_int::from(indexLargeEnough != 0 && dictionaryInvalidated != 0) as u32
}
