use libc::size_t;

use crate::internal::MEM_readLE32;
use crate::lib::common::fse::{FSE_CTable, FSE_repeat};
use crate::lib::common::huf::{HUF_CElt, HUF_repeat, HUF_CTABLE_SIZE_ST};
use crate::lib::common::mem::{MEM_64bits, MEM_read16, MEM_read32, MEM_readLE64, MEM_readST};
use crate::lib::common::zstd_internal::{
    Overlap, ZSTD_copy16, ZSTD_wildcopy, MINMATCH, WILDCOPY_OVERLENGTH, ZSTD_REP_NUM,
};
use crate::lib::compress::zstd_compress::{
    SeqDef, SeqStore_t, ZSTD_CDict, ZSTD_MatchState_t, ZSTD_compressedBlockState_t,
    ZSTD_entropyCTablesMetadata_t, ZSTD_optimal_t, ZSTD_window_t, HASH_READ_SIZE,
    ZSTD_MAX_NB_BLOCK_SPLITS,
};
use crate::lib::compress::zstd_compress_superblock::ZSTD_SequenceLength;
use crate::lib::polyfill::PointerExt;
use crate::lib::zstd::{ParamSwitch, ZSTD_Sequence, ZSTD_dictContentType_e};

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum CompressionStage {
    #[default]
    Created = 0,
    Init = 1,
    Ongoing = 2,
    Ending = 3,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum StreamStage {
    #[default]
    Init = 0,
    Load = 1,
    Flush = 2,
}

pub type ZSTD_prefixDict = ZSTD_prefixDict_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut core::ffi::c_void,
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; HUF_CTABLE_SIZE_ST(255)],
    pub repeatMode: HUF_repeat,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum LongLengthType {
    #[default]
    None = 0,
    Literal = 1,
    Match = 2,
}

pub(crate) unsafe fn ZSTD_getSequenceLength(
    seqStore: *const SeqStore_t,
    seq: *const SeqDef,
) -> ZSTD_SequenceLength {
    let mut seqLen = ZSTD_SequenceLength {
        litLength: u32::from((*seq).litLength),
        matchLength: u32::from((*seq).mlBase) + MINMATCH as u32,
    };

    if (*seqStore).longLengthPos == (seq as usize - (*seqStore).sequencesStart as usize) as u32 {
        if (*seqStore).longLengthType == LongLengthType::Literal {
            seqLen.litLength += 0x10000;
        }
        if (*seqStore).longLengthType == LongLengthType::Match {
            seqLen.matchLength += 0x10000;
        }
    }

    seqLen
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    /// Offset sumtype code for the match, using `ZSTD_storeSeq()` format
    pub off: u32,
    /// Raw length of match
    pub len: u32,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum OptPrice {
    #[default]
    Dynamic = 0,
    Predef = 1,
}

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
    pub priceType: OptPrice,
    pub symbolCosts: *const ZSTD_entropyCTables_t,
    pub literalCompressionMode: ParamSwitch,
}

#[repr(C)]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_MatchState_t,
}

#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: core::ffi::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: size_t,
    pub maxSequences: size_t,
}

/// Indicates whether this compression proceeds directly from user-provided
/// source buffer to user-provided destination buffer (`BufferedPolicy::NotBuffered`), or
/// whether the context needs to buffer the input/output (`BufferedPolicy::Buffered`).
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum BufferedPolicy {
    #[default]
    NotBuffered = 0,
    Buffered = 1,
}

/// Struct that contains all elements of block splitter that should be allocated
/// in a wksp.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockSplitCtx {
    pub fullSeqStoreChunk: SeqStore_t,
    pub firstHalfSeqStore: SeqStore_t,
    pub secondHalfSeqStore: SeqStore_t,
    pub currSeqStore: SeqStore_t,
    pub nextSeqStore: SeqStore_t,
    pub partitions: [u32; ZSTD_MAX_NB_BLOCK_SPLITS],
    pub entropyMetadata: ZSTD_entropyCTablesMetadata_t,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum DictTableLoadMethod {
    #[default]
    Fast = 0,
    Full = 1,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum TableFillPurpose {
    #[default]
    ForCCtx = 0,
    ForCDict = 1,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum DictMode {
    #[default]
    NoDict = 0,
    ExtDict = 1,
    DictMatchState = 2,
    DedicatedDictSearch = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum CParamMode {
    /// Compression with [`DictMode::NoDict`] or [`DictMode::ExtDict`].
    /// In this mode we use both the srcSize and the dictSize
    /// when selecting and adjusting parameters.
    #[default]
    NoAttachDict = 0,
    /// Compression with [`DictMode::DictMatchState`] or [`DictMode::DedicatedDictSearch`].
    /// In this mode we only take the srcSize into account when selecting
    /// and adjusting parameters.
    AttachDict = 1,
    /// Creating a CDict.
    /// In this mode we take both the source size and the dictionary size
    /// into account when selecting and adjusting the parameters.
    CreateCDict = 2,
    /// `ZSTD_getCParams`, `ZSTD_getParams`, `ZSTD_adjustParams`.
    /// We don't know what these parameters are for. We default to the legacy
    /// behavior of taking both the source size and the dict size into account
    /// when selecting and adjusting parameters.
    Unknown = 3,
}

impl From<i32> for CParamMode {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::NoAttachDict,
            1 => Self::AttachDict,
            2 => Self::CreateCDict,
            _ => Self::Unknown,
        }
    }
}

pub type ZSTD_BlockCompressor_f = unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    &mut [u32; 3],
    *const core::ffi::c_void,
    size_t,
) -> size_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct repcodes_s {
    pub rep: [u32; 3],
}

pub(crate) type Repcodes_t = repcodes_s;

pub(crate) const ZSTD_CURRENT_MAX: usize = if MEM_64bits() {
    3500 * (1 << 20)
} else {
    2000 * (1 << 20)
};

#[inline(always)]
pub(crate) unsafe fn ZSTD_storeSeqOnly(
    seqStorePtr: &mut SeqStore_t,
    litLength: usize,
    offBase: u32,
    matchLength: usize,
) {
    if litLength > 0xffff {
        seqStorePtr.longLengthType = LongLengthType::Literal;
        seqStorePtr.longLengthPos = (seqStorePtr.sequences).offset_from(seqStorePtr.sequencesStart)
            as core::ffi::c_long as u32;
    }
    (*(seqStorePtr.sequences)).litLength = litLength as u16;
    (*(seqStorePtr.sequences)).offBase = offBase;
    let mlBase = matchLength.wrapping_sub(MINMATCH as usize);
    if mlBase > 0xffff {
        seqStorePtr.longLengthType = LongLengthType::Match;
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
pub(crate) fn ZSTD_updateRep(rep: &mut [u32; 3], offBase: u32, ll0: u32) {
    if offBase > ZSTD_REP_NUM as u32 {
        rep[2] = rep[1];
        rep[1] = rep[0];
        rep[0] = offBase.wrapping_sub(ZSTD_REP_NUM as u32);
    } else {
        let repCode = offBase.wrapping_sub(1).wrapping_add(ll0);
        if repCode > 0 {
            let currentOffset = if repCode == ZSTD_REP_NUM as u32 {
                rep[0].wrapping_sub(1)
            } else {
                rep[repCode as usize]
            };
            rep[2] = if repCode >= 2 { rep[1] } else { rep[2] };
            rep[1] = rep[0];
            rep[0] = currentOffset;
        }
    }
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
        *op = *ip;
        ip = ip.add(1);
        op = op.add(1);
    }
}

#[inline]
const fn ZSTD_NbCommonBytes(val: usize) -> u32 {
    if cfg!(target_endian = "little") {
        val.trailing_zeros() >> 3
    } else {
        val.leading_zeros() >> 3
    }
}

#[inline]
pub(crate) unsafe fn ZSTD_count(
    mut pIn: *const u8,
    mut pMatch: *const u8,
    pInLimit: *const u8,
) -> usize {
    let pStart = pIn;
    let pInLoopLimit = pInLimit.sub(size_of::<usize>().wrapping_sub(1) as usize);
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const core::ffi::c_void)
            ^ MEM_readST(pIn as *const core::ffi::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as usize;
        }
        pIn = pIn.add(size_of::<usize>());
        pMatch = pMatch.add(size_of::<usize>());
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const core::ffi::c_void)
                ^ MEM_readST(pIn as *const core::ffi::c_void);
            if diff_0 == 0 {
                pIn = pIn.add(size_of::<usize>());
                pMatch = pMatch.add(size_of::<usize>());
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
        && MEM_read16(pMatch as *const core::ffi::c_void) as core::ffi::c_int
            == MEM_read16(pIn as *const core::ffi::c_void) as core::ffi::c_int
    {
        pIn = pIn.add(2);
        pMatch = pMatch.add(2);
    }
    if pIn < pInLimit && *pMatch as core::ffi::c_int == *pIn as core::ffi::c_int {
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
    ms: &ZSTD_MatchState_t,
    curr: u32,
    windowLog: core::ffi::c_uint,
) -> u32 {
    let maxDistance = 1 << windowLog;
    let lowestValid = ms.window.lowLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = (ms.loadedDictEnd != 0) as core::ffi::c_int as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}

#[inline]
pub(crate) unsafe fn ZSTD_getLowestPrefixIndex(
    ms: &ZSTD_MatchState_t,
    curr: u32,
    windowLog: core::ffi::c_uint,
) -> u32 {
    let maxDistance = 1 << windowLog;
    let lowestValid = ms.window.dictLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary = (ms.loadedDictEnd != 0) as core::ffi::c_int as u32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}

#[inline]
pub(crate) fn ZSTD_index_overlap_check(prefixLowestIndex: u32, repIndex: u32) -> bool {
    prefixLowestIndex.wrapping_sub(1).wrapping_sub(repIndex) >= 3
}

#[inline]
pub(crate) fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> bool {
    window.lowLimit < window.dictLimit
}

/// Inspects the provided matchState and figures out what dictMode
/// should be passed to the compressor.
#[inline]
pub(crate) unsafe fn ZSTD_matchState_dictMode(ms: *const ZSTD_MatchState_t) -> DictMode {
    if ZSTD_window_hasExtDict((*ms).window) {
        DictMode::ExtDict
    } else if !((*ms).dictMatchState).is_null() {
        if (*(*ms).dictMatchState).dedicatedDictSearch != 0 {
            DictMode::DedicatedDictSearch
        } else {
            DictMode::DictMatchState
        }
    } else {
        DictMode::NoDict
    }
}

/// Updates the window by appending [src, src + srcSize) to the window.
///
/// If it is not contiguous, the current prefix becomes the extDict, and we forget about the
/// extDict. Handles overlap of the prefix and extDict.
///
/// Returns `true` if the segment is contiguous.
#[inline]
pub(crate) unsafe fn ZSTD_window_update(
    window: &mut ZSTD_window_t,
    src: *const core::ffi::c_void,
    srcSize: usize,
    forceNonContiguous: bool,
) -> bool {
    let ip = src as *const u8;
    let mut contiguous = true;
    if srcSize == 0 {
        return contiguous;
    }

    // Check if blocks follow each other
    if src != window.nextSrc as *const core::ffi::c_void || forceNonContiguous {
        // not contiguous
        let distanceFromBase = (window.nextSrc).wrapping_offset_from(window.base) as usize;
        window.lowLimit = window.dictLimit;
        window.dictLimit = distanceFromBase as u32;
        window.dictBase = window.base;
        window.base = ip.wrapping_sub(distanceFromBase);
        if (window.dictLimit).wrapping_sub(window.lowLimit) < HASH_READ_SIZE as u32 {
            window.lowLimit = window.dictLimit;
        }
        contiguous = false;
    }
    window.nextSrc = ip.add(srcSize);

    // if input and dictionary overlap: reduce dictionary (area presumed modified by input)
    if (ip.add(srcSize) > (window.dictBase).wrapping_offset(window.lowLimit as isize))
        && (ip < (window.dictBase).wrapping_offset(window.dictLimit as isize))
    {
        let highInputIdx = ip.add(srcSize).offset_from(window.dictBase) as usize;
        let lowLimitMax = (highInputIdx as u32).min(window.dictLimit);
        window.lowLimit = lowLimitMax;
    }

    contiguous
}

/// Updates lowLimit so that:
///    (srcEnd - base) - lowLimit == maxDist + loadedDictEnd
///
/// It ensures index is valid as long as index >= lowLimit.
/// This must be called before a block compression call.
///
/// loadedDictEnd is only defined if a dictionary is in use for current compression.
/// As the name implies, loadedDictEnd represents the index at end of dictionary.
/// The value lies within context's referential, it can be directly compared to blockEndIdx.
///
/// If loadedDictEndPtr is NULL, no dictionary is in use, and we use loadedDictEnd == 0.
/// If loadedDictEndPtr is not NULL, we set it to zero after updating lowLimit.
/// This is because dictionaries are allowed to be referenced fully
/// as long as the last byte of the dictionary is in the window.
/// Once input has progressed beyond window size, dictionary cannot be referenced anymore.
///
/// In normal dict mode, the dictionary lies between lowLimit and dictLimit.
/// In dictMatchState mode, lowLimit and dictLimit are the same,
/// and the dictionary is below them.
/// forceWindow and dictMatchState are therefore incompatible.
#[inline]
pub(crate) unsafe fn ZSTD_window_enforceMaxDist(
    window: &mut ZSTD_window_t,
    blockEnd: *const core::ffi::c_void,
    maxDist: u32,
    loadedDictEndPtr: &mut u32,
    dictMatchStatePtr: Option<&mut *const ZSTD_MatchState_t>,
) {
    let blockEndIdx =
        (blockEnd as *const u8).wrapping_offset_from(window.base) as core::ffi::c_long as u32;
    let loadedDictEnd = *loadedDictEndPtr;

    // - When there is no dictionary: loadedDictEnd == 0.
    //   In which case, the test (blockEndIdx > maxDist) is merely to avoid
    //   overflowing next operation `newLowLimit = blockEndIdx - maxDist`.
    // - When there is a standard dictionary:
    //   Index referential is copied from the dictionary,
    //   which means it starts from 0.
    //   In which case, loadedDictEnd == dictSize,
    //   and it makes sense to compare `blockEndIdx > maxDist + dictSize`
    //   since `blockEndIdx` also starts from zero.
    // - When there is an attached dictionary:
    //   loadedDictEnd is expressed within the referential of the context,
    //   so it can be directly compared against blockEndIdx.
    if blockEndIdx > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit = blockEndIdx.wrapping_sub(maxDist);
        if window.lowLimit < newLowLimit {
            window.lowLimit = newLowLimit;
        }
        if window.dictLimit < window.lowLimit {
            window.dictLimit = window.lowLimit;
        }
        // On reaching window size, dictionaries are invalidated
        *loadedDictEndPtr = 0;
        if let Some(dictMatchStatePtr) = dictMatchStatePtr {
            *dictMatchStatePtr = core::ptr::null();
        }
    }
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
        if ZSTD_window_canOverflowCorrect(window, cycleLog, maxDist, loadedDictEnd, src) {
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
) -> bool {
    let cycleSize = (1 as core::ffi::c_uint) << cycleLog;
    let curr = (src as *const u8).offset_from(window.base) as core::ffi::c_long as u32;
    let minIndexToOverflowCorrect = cycleSize
        .wrapping_add(maxDist.max(cycleSize))
        .wrapping_add(ZSTD_WINDOW_START_INDEX as u32);
    let adjustment = (window.nbOverflowCorrections).wrapping_add(1);
    let adjustedIndex = minIndexToOverflowCorrect
        .wrapping_mul(adjustment)
        .max(minIndexToOverflowCorrect);
    let indexLargeEnough = curr > adjustedIndex;
    let dictionaryInvalidated = curr > maxDist.wrapping_add(loadedDictEnd);
    indexLargeEnough && dictionaryInvalidated
}
