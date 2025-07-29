use core::ptr;

use libc::{fflush, fprintf, free, malloc, memcpy, memmove, memset, size_t, FILE};

use crate::lib::common::error_private::{ERR_getErrorName, ERR_isError};
use crate::lib::common::mem::{
    MEM_64bits, MEM_isLittleEndian, MEM_read16, MEM_read64, MEM_readLE32, MEM_readST, MEM_writeLE32,
};
use crate::lib::common::pool::POOL_ctx;
use crate::lib::common::xxhash::ZSTD_XXH64;
use crate::lib::compress::fse_compress::{FSE_normalizeCount, FSE_writeNCount};
use crate::lib::compress::huf_compress::{HUF_buildCTable_wksp, HUF_writeCTable_wksp};
use crate::lib::compress::zstd_compress::{
    SeqDef, ZSTD_CCtx, ZSTD_CDict, ZSTD_MatchState_t, ZSTD_compressBegin_usingCDict_deprecated,
    ZSTD_compressBlock_deprecated, ZSTD_compressedBlockState_t, ZSTD_createCCtx,
    ZSTD_createCDict_advanced, ZSTD_freeCCtx, ZSTD_freeCDict, ZSTD_getParams, ZSTD_getSeqStore,
    ZSTD_loadCEntropy, ZSTD_optimal_t, ZSTD_reset_compressedBlockState, ZSTD_seqToCodes,
};
use crate::lib::dictBuilder::fastcover::{
    ZDICT_fastCover_params_t, ZDICT_optimizeTrainFromBuffer_fastCover,
};
use crate::lib::zstd::*;

extern "C" {
    static mut stderr: *mut FILE;
    fn clock() -> clock_t;
    fn divsufsort(
        T: *const core::ffi::c_uchar,
        SA: *mut core::ffi::c_int,
        n: core::ffi::c_int,
        openMP: core::ffi::c_int,
    ) -> core::ffi::c_int;
}
pub type __clock_t = core::ffi::c_long;
pub type clock_t = __clock_t;
pub type FSE_CTable = core::ffi::c_uint;
pub type ERR_enum = ZSTD_ErrorCode;
pub type FSE_repeat = core::ffi::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type HUF_CElt = size_t;
pub type HUF_repeat = core::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Sequence {
    pub offset: core::ffi::c_uint,
    pub litLength: core::ffi::c_uint,
    pub matchLength: core::ffi::c_uint,
    pub rep: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockSplitCtx {
    pub fullSeqStoreChunk: SeqStore_t,
    pub firstHalfSeqStore: SeqStore_t,
    pub secondHalfSeqStore: SeqStore_t,
    pub currSeqStore: SeqStore_t,
    pub nextSeqStore: SeqStore_t,
    pub partitions: [u32; 196],
    pub entropyMetadata: ZSTD_entropyCTablesMetadata_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTablesMetadata_t {
    pub hufMetadata: ZSTD_hufCTablesMetadata_t,
    pub fseMetadata: ZSTD_fseCTablesMetadata_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTablesMetadata_t {
    pub llType: SymbolEncodingType_e,
    pub ofType: SymbolEncodingType_e,
    pub mlType: SymbolEncodingType_e,
    pub fseTablesBuffer: [u8; 133],
    pub fseTablesSize: size_t,
    pub lastCountSize: size_t,
}
pub type SymbolEncodingType_e = core::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTablesMetadata_t {
    pub hType: SymbolEncodingType_e,
    pub hufDesBuffer: [u8; 128],
    pub hufDesSize: size_t,
}
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
pub type ZSTD_longLengthType_e = core::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub type ZSTD_dictContentType_e = core::ffi::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut core::ffi::c_void,
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const core::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_cStreamStage = core::ffi::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub type ZSTD_buffered_policy_e = core::ffi::c_uint;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_MatchState_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type ZSTD_OptPrice_e = core::ffi::c_uint;
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
pub struct ZSTD_window_t {
    pub nextSrc: *const u8,
    pub base: *const u8,
    pub dictBase: *const u8,
    pub dictLimit: u32,
    pub lowLimit: u32,
    pub nbOverflowCorrections: u32,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: core::ffi::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: size_t,
    pub maxSequences: size_t,
}
pub type ZSTD_threadPool = POOL_ctx;
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_hash_t = u64;
pub type XXH32_hash_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cwksp {
    pub workspace: *mut core::ffi::c_void,
    pub workspaceEnd: *mut core::ffi::c_void,
    pub objectEnd: *mut core::ffi::c_void,
    pub tableEnd: *mut core::ffi::c_void,
    pub tableValidEnd: *mut core::ffi::c_void,
    pub allocStart: *mut core::ffi::c_void,
    pub initOnceStart: *mut core::ffi::c_void,
    pub allocFailed: u8,
    pub workspaceOversizedDuration: core::ffi::c_int,
    pub phase: ZSTD_cwksp_alloc_phase_e,
    pub isStatic: ZSTD_cwksp_static_alloc_e,
}
pub type ZSTD_cwksp_static_alloc_e = core::ffi::c_uint;
pub const ZSTD_cwksp_static_alloc: ZSTD_cwksp_static_alloc_e = 1;
pub const ZSTD_cwksp_dynamic_alloc: ZSTD_cwksp_static_alloc_e = 0;
pub type ZSTD_cwksp_alloc_phase_e = core::ffi::c_uint;
pub const ZSTD_cwksp_alloc_buffers: ZSTD_cwksp_alloc_phase_e = 3;
pub const ZSTD_cwksp_alloc_aligned: ZSTD_cwksp_alloc_phase_e = 2;
pub const ZSTD_cwksp_alloc_aligned_init_once: ZSTD_cwksp_alloc_phase_e = 1;
pub const ZSTD_cwksp_alloc_objects: ZSTD_cwksp_alloc_phase_e = 0;
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_params_s {
    pub format: Format,
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
    pub compressionLevel: core::ffi::c_int,
    pub forceWindow: core::ffi::c_int,
    pub targetCBlockSize: size_t,
    pub srcSizeHint: core::ffi::c_int,
    pub attachDictPref: ZSTD_dictAttachPref_e,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
    pub nbWorkers: core::ffi::c_int,
    pub jobSize: size_t,
    pub overlapLog: core::ffi::c_int,
    pub rsyncable: core::ffi::c_int,
    pub ldmParams: ldmParams_t,
    pub enableDedicatedDictSearch: core::ffi::c_int,
    pub inBufferMode: ZSTD_bufferMode_e,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub blockDelimiters: ZSTD_SequenceFormat_e,
    pub validateSequences: core::ffi::c_int,
    pub postBlockSplitter: ZSTD_ParamSwitch_e,
    pub preBlockSplitter_level: core::ffi::c_int,
    pub maxBlockSize: size_t,
    pub useRowMatchFinder: ZSTD_ParamSwitch_e,
    pub deterministicRefPrefix: core::ffi::c_int,
    pub customMem: ZSTD_customMem,
    pub prefetchCDictTables: ZSTD_ParamSwitch_e,
    pub enableMatchFinderFallback: core::ffi::c_int,
    pub extSeqProdState: *mut core::ffi::c_void,
    pub extSeqProdFunc: ZSTD_sequenceProducer_F,
    pub searchForExternalRepcodes: ZSTD_ParamSwitch_e,
}
pub type ZSTD_sequenceProducer_F = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *mut ZSTD_Sequence,
        size_t,
        *const core::ffi::c_void,
        size_t,
        *const core::ffi::c_void,
        size_t,
        core::ffi::c_int,
        size_t,
    ) -> size_t,
>;
pub type ZSTD_SequenceFormat_e = core::ffi::c_uint;
pub const ZSTD_sf_explicitBlockDelimiters: ZSTD_SequenceFormat_e = 1;
pub const ZSTD_sf_noBlockDelimiters: ZSTD_SequenceFormat_e = 0;
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
pub type ZSTD_dictAttachPref_e = core::ffi::c_uint;
pub const ZSTD_dictForceLoad: ZSTD_dictAttachPref_e = 3;
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = 2;
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
pub type ZSTD_compressionStage_e = core::ffi::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub type ZSTD_dictLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: core::ffi::c_int,
    pub notificationLevel: core::ffi::c_uint,
    pub dictID: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EStats_ress_t {
    pub dict: *mut ZSTD_CDict,
    pub zc: *mut ZSTD_CCtx,
    pub workPlace: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct offsetCount_t {
    pub offset: u32,
    pub count: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: core::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictItem {
    pub pos: u32,
    pub length: u32,
    pub savings: u32,
}
pub const MINRATIO: core::ffi::c_int = 4;
pub const ZDICT_MAX_SAMPLES_SIZE: core::ffi::c_uint = (2000) << 20;
pub const ZDICT_MIN_SAMPLES_SIZE: core::ffi::c_int = ZDICT_CONTENTSIZE_MIN * MINRATIO;

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
#[inline]
unsafe fn ZSTD_highbit32(mut val: u32) -> core::ffi::c_uint {
    (31 as core::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const HUF_WORKSPACE_SIZE: core::ffi::c_int = ((8) << 10) + 512;
pub const ZSTD_CLEVEL_DEFAULT: core::ffi::c_int = 3;
pub const ZSTD_MAGIC_DICTIONARY: core::ffi::c_uint = 0xec30a437 as core::ffi::c_uint;
pub const ZSTD_BLOCKSIZELOG_MAX: core::ffi::c_int = 17;
pub const ZSTD_BLOCKSIZE_MAX: core::ffi::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
static mut ZSTD_defaultCMem: ZSTD_customMem = unsafe {
    {
        ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<libc::intptr_t, ZSTD_allocFunction>(
                NULL as libc::intptr_t,
            ),
            customFree: ::core::mem::transmute::<libc::intptr_t, ZSTD_freeFunction>(
                NULL as libc::intptr_t,
            ),
            opaque: NULL as *mut core::ffi::c_void,
        }
    }
};
pub const ZSTD_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const FSE_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const HUF_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const ZSTD_REP_NUM: core::ffi::c_int = 3;
static repStartValue: [u32; 3] = [1, 4, 8];
pub const MaxML: core::ffi::c_int = 52;
pub const MaxLL: core::ffi::c_int = 35;
pub const MLFSELog: core::ffi::c_int = 9;
pub const LLFSELog: core::ffi::c_int = 9;
pub const OffFSELog: core::ffi::c_int = 8;
pub const ZDICT_DICTSIZE_MIN: core::ffi::c_int = 256;
pub const ZDICT_CONTENTSIZE_MIN: core::ffi::c_int = 128;
pub const NULL: core::ffi::c_int = 0;
pub const CLOCKS_PER_SEC: core::ffi::c_int = 1000000;
pub const NOISELENGTH: core::ffi::c_int = 32;
static g_selectivity_default: u32 = 9;
unsafe fn ZDICT_clockSpan(mut nPrevious: clock_t) -> clock_t {
    clock() - nPrevious
}
unsafe fn ZDICT_printHex(mut ptr: *const core::ffi::c_void, mut length: size_t) {
    let b = ptr as *const u8;
    let mut u: size_t = 0;
    u = 0;
    while u < length {
        let mut c = *b.offset(u as isize);
        if (c as core::ffi::c_int) < 32 || c as core::ffi::c_int > 126 {
            c = '.' as i32 as u8;
        }
        fprintf(
            stderr,
            b"%c\0" as *const u8 as *const core::ffi::c_char,
            c as core::ffi::c_int,
        );
        fflush(stderr);
        u = u.wrapping_add(1);
    }
}
#[export_name = crate::prefix!(ZDICT_isError)]
pub unsafe extern "C" fn ZDICT_isError(mut errorCode: size_t) -> core::ffi::c_uint {
    ERR_isError(errorCode)
}
#[export_name = crate::prefix!(ZDICT_getErrorName)]
pub unsafe extern "C" fn ZDICT_getErrorName(mut errorCode: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(errorCode)
}
#[export_name = crate::prefix!(ZDICT_getDictID)]
pub unsafe extern "C" fn ZDICT_getDictID(
    mut dictBuffer: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> core::ffi::c_uint {
    if dictSize < 8 {
        return 0;
    }
    if MEM_readLE32(dictBuffer) != ZSTD_MAGIC_DICTIONARY {
        return 0;
    }
    MEM_readLE32((dictBuffer as *const core::ffi::c_char).offset(4) as *const core::ffi::c_void)
}
#[export_name = crate::prefix!(ZDICT_getDictHeaderSize)]
pub unsafe extern "C" fn ZDICT_getDictHeaderSize(
    mut dictBuffer: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut headerSize: size_t = 0;
    if dictSize <= 8 || MEM_readLE32(dictBuffer) != ZSTD_MAGIC_DICTIONARY {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    let mut bs = malloc(::core::mem::size_of::<ZSTD_compressedBlockState_t>() as size_t)
        as *mut ZSTD_compressedBlockState_t;
    let mut wksp = malloc(HUF_WORKSPACE_SIZE as size_t) as *mut u32;
    if bs.is_null() || wksp.is_null() {
        headerSize = -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    } else {
        ZSTD_reset_compressedBlockState(bs);
        headerSize = ZSTD_loadCEntropy(bs, wksp as *mut core::ffi::c_void, dictBuffer, dictSize);
    }
    free(bs as *mut core::ffi::c_void);
    free(wksp as *mut core::ffi::c_void);
    headerSize
}
unsafe fn ZDICT_count(
    mut pIn: *const core::ffi::c_void,
    mut pMatch: *const core::ffi::c_void,
) -> size_t {
    let pStart = pIn as *const core::ffi::c_char;
    loop {
        let diff = MEM_readST(pMatch) ^ MEM_readST(pIn);
        if diff == 0 {
            pIn = (pIn as *const core::ffi::c_char)
                .offset(::core::mem::size_of::<size_t>() as isize)
                as *const core::ffi::c_void;
            pMatch = (pMatch as *const core::ffi::c_char)
                .offset(::core::mem::size_of::<size_t>() as isize)
                as *const core::ffi::c_void;
        } else {
            pIn = (pIn as *const core::ffi::c_char).offset(ZSTD_NbCommonBytes(diff) as isize)
                as *const core::ffi::c_void;
            return (pIn as *const core::ffi::c_char).offset_from(pStart) as core::ffi::c_long
                as size_t;
        }
    }
}
unsafe fn ZDICT_initDictItem(mut d: *mut dictItem) {
    (*d).pos = 1;
    (*d).length = 0;
    (*d).savings = -(1 as core::ffi::c_int) as u32;
}
pub const LLIMIT: core::ffi::c_int = 64;
pub const MINMATCHLENGTH: core::ffi::c_int = 7;
unsafe fn ZDICT_analyzePos(
    mut doneMarks: *mut u8,
    mut suffix: *const core::ffi::c_uint,
    mut start: u32,
    mut buffer: *const core::ffi::c_void,
    mut minRatio: u32,
    mut notificationLevel: u32,
) -> dictItem {
    let mut lengthList: [u32; 64] = [0; 64];
    let mut cumulLength: [u32; 64] = [0; 64];
    let mut savings: [u32; 64] = [0; 64];
    let mut b = buffer as *const u8;
    let mut maxLength = LLIMIT as size_t;
    let mut pos = *suffix.offset(start as isize) as size_t;
    let mut end = start;
    let mut solution = dictItem {
        pos: 0,
        length: 0,
        savings: 0,
    };
    ptr::write_bytes(
        &mut solution as *mut dictItem as *mut u8,
        0,
        ::core::mem::size_of::<dictItem>(),
    );
    *doneMarks.offset(pos as isize) = 1;
    if MEM_read16(b.offset(pos as isize).offset(0) as *const core::ffi::c_void) as core::ffi::c_int
        == MEM_read16(b.offset(pos as isize).offset(2) as *const core::ffi::c_void)
            as core::ffi::c_int
        || MEM_read16(b.offset(pos as isize).offset(1) as *const core::ffi::c_void)
            as core::ffi::c_int
            == MEM_read16(b.offset(pos as isize).offset(3) as *const core::ffi::c_void)
                as core::ffi::c_int
        || MEM_read16(b.offset(pos as isize).offset(2) as *const core::ffi::c_void)
            as core::ffi::c_int
            == MEM_read16(b.offset(pos as isize).offset(4) as *const core::ffi::c_void)
                as core::ffi::c_int
    {
        let pattern16 = MEM_read16(b.offset(pos as isize).offset(4) as *const core::ffi::c_void);
        let mut u: u32 = 0;
        let mut patternEnd = 6u32;
        while MEM_read16(
            b.offset(pos as isize).offset(patternEnd as isize) as *const core::ffi::c_void
        ) as core::ffi::c_int
            == pattern16 as core::ffi::c_int
        {
            patternEnd = patternEnd.wrapping_add(2);
        }
        if *b.offset(pos.wrapping_add(patternEnd as size_t) as isize) as core::ffi::c_int
            == *b.offset(pos.wrapping_add(patternEnd as size_t).wrapping_sub(1) as isize)
                as core::ffi::c_int
        {
            patternEnd = patternEnd.wrapping_add(1);
        }
        u = 1;
        while u < patternEnd {
            *doneMarks.offset(pos.wrapping_add(u as size_t) as isize) = 1;
            u = u.wrapping_add(1);
        }
        return solution;
    }
    let mut length: size_t = 0;
    loop {
        end = end.wrapping_add(1);
        length = ZDICT_count(
            b.offset(pos as isize) as *const core::ffi::c_void,
            b.offset(*suffix.offset(end as isize) as isize) as *const core::ffi::c_void,
        );
        if length < MINMATCHLENGTH as size_t {
            break;
        }
    }
    let mut length_0: size_t = 0;
    loop {
        length_0 = ZDICT_count(
            b.offset(pos as isize) as *const core::ffi::c_void,
            b.offset(*suffix.offset(start as isize).offset(-(1)) as isize)
                as *const core::ffi::c_void,
        );
        if length_0 >= MINMATCHLENGTH as size_t {
            start = start.wrapping_sub(1);
        }
        if length_0 < MINMATCHLENGTH as size_t {
            break;
        }
    }
    if end.wrapping_sub(start) < minRatio {
        let mut idx: u32 = 0;
        idx = start;
        while idx < end {
            *doneMarks.offset(*suffix.offset(idx as isize) as isize) = 1;
            idx = idx.wrapping_add(1);
        }
        return solution;
    }
    let mut i: core::ffi::c_int = 0;
    let mut mml: u32 = 0;
    let mut refinedStart = start;
    let mut refinedEnd = end;
    if notificationLevel >= 4 {
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        fflush(stderr);
    }
    if notificationLevel >= 4 {
        fprintf(
            stderr,
            b"found %3u matches of length >= %i at pos %7u  \0" as *const u8
                as *const core::ffi::c_char,
            end.wrapping_sub(start),
            7,
            pos as core::ffi::c_uint,
        );
        fflush(stderr);
    }
    if notificationLevel >= 4 {
        fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
        fflush(stderr);
    }
    mml = MINMATCHLENGTH as u32;
    loop {
        let mut currentChar = 0;
        let mut currentCount = 0u32;
        let mut currentID = refinedStart;
        let mut id: u32 = 0;
        let mut selectedCount = 0;
        let mut selectedID = currentID;
        id = refinedStart;
        while id < refinedEnd {
            if *b.offset((*suffix.offset(id as isize)).wrapping_add(mml) as isize)
                as core::ffi::c_int
                != currentChar as core::ffi::c_int
            {
                if currentCount > selectedCount {
                    selectedCount = currentCount;
                    selectedID = currentID;
                }
                currentID = id;
                currentChar = *b.offset((*suffix.offset(id as isize)).wrapping_add(mml) as isize);
                currentCount = 0;
            }
            currentCount = currentCount.wrapping_add(1);
            id = id.wrapping_add(1);
        }
        if currentCount > selectedCount {
            selectedCount = currentCount;
            selectedID = currentID;
        }
        if selectedCount < minRatio {
            break;
        }
        refinedStart = selectedID;
        refinedEnd = refinedStart.wrapping_add(selectedCount);
        mml = mml.wrapping_add(1);
    }
    start = refinedStart;
    pos = *suffix.offset(refinedStart as isize) as size_t;
    end = start;
    ptr::write_bytes(
        lengthList.as_mut_ptr() as *mut u8,
        0,
        ::core::mem::size_of::<[u32; 64]>(),
    );
    let mut length_1: size_t = 0;
    loop {
        end = end.wrapping_add(1);
        length_1 = ZDICT_count(
            b.offset(pos as isize) as *const core::ffi::c_void,
            b.offset(*suffix.offset(end as isize) as isize) as *const core::ffi::c_void,
        );
        if length_1 >= LLIMIT as size_t {
            length_1 = (LLIMIT - 1) as size_t;
        }
        let fresh0 = &mut (*lengthList.as_mut_ptr().offset(length_1 as isize));
        *fresh0 = (*fresh0).wrapping_add(1);
        if length_1 < MINMATCHLENGTH as size_t {
            break;
        }
    }
    let mut length_2 = MINMATCHLENGTH as size_t;
    while (length_2 >= MINMATCHLENGTH as size_t) as core::ffi::c_int
        & (start > 0) as core::ffi::c_int
        != 0
    {
        length_2 = ZDICT_count(
            b.offset(pos as isize) as *const core::ffi::c_void,
            b.offset(*suffix.offset(start.wrapping_sub(1) as isize) as isize)
                as *const core::ffi::c_void,
        );
        if length_2 >= LLIMIT as size_t {
            length_2 = (LLIMIT - 1) as size_t;
        }
        let fresh1 = &mut (*lengthList.as_mut_ptr().offset(length_2 as isize));
        *fresh1 = (*fresh1).wrapping_add(1);
        if length_2 >= MINMATCHLENGTH as size_t {
            start = start.wrapping_sub(1);
        }
    }
    ptr::write_bytes(
        cumulLength.as_mut_ptr() as *mut u8,
        0,
        ::core::mem::size_of::<[u32; 64]>(),
    );
    *cumulLength
        .as_mut_ptr()
        .offset(maxLength.wrapping_sub(1) as isize) = *lengthList
        .as_mut_ptr()
        .offset(maxLength.wrapping_sub(1) as isize);
    i = maxLength.wrapping_sub(2) as core::ffi::c_int;
    while i >= 0 {
        *cumulLength.as_mut_ptr().offset(i as isize) =
            (*cumulLength.as_mut_ptr().offset((i + 1) as isize))
                .wrapping_add(*lengthList.as_mut_ptr().offset(i as isize));
        i -= 1;
    }
    let mut u_0: core::ffi::c_uint = 0;
    u_0 = (LLIMIT - 1) as core::ffi::c_uint;
    while u_0 >= MINMATCHLENGTH as core::ffi::c_uint {
        if *cumulLength.as_mut_ptr().offset(u_0 as isize) >= minRatio {
            break;
        }
        u_0 = u_0.wrapping_sub(1);
    }
    maxLength = u_0 as size_t;
    let mut l = maxLength as u32;
    let c = *b.offset(pos.wrapping_add(maxLength).wrapping_sub(1) as isize);
    while *b.offset(pos.wrapping_add(l as size_t).wrapping_sub(2) as isize) as core::ffi::c_int
        == c as core::ffi::c_int
    {
        l = l.wrapping_sub(1);
    }
    maxLength = l as size_t;
    if maxLength < MINMATCHLENGTH as size_t {
        return solution;
    }
    *savings.as_mut_ptr().offset(5) = 0;
    let mut u_1: core::ffi::c_uint = 0;
    u_1 = MINMATCHLENGTH as core::ffi::c_uint;
    while u_1 as size_t <= maxLength {
        *savings.as_mut_ptr().offset(u_1 as isize) =
            (*savings.as_mut_ptr().offset(u_1.wrapping_sub(1) as isize)).wrapping_add(
                (*lengthList.as_mut_ptr().offset(u_1 as isize)).wrapping_mul(u_1.wrapping_sub(3)),
            );
        u_1 = u_1.wrapping_add(1);
    }
    if notificationLevel >= 4 {
        fprintf(
            stderr,
            b"Selected dict at position %u, of length %u : saves %u (ratio: %.2f)  \n\0"
                as *const u8 as *const core::ffi::c_char,
            pos as core::ffi::c_uint,
            maxLength as core::ffi::c_uint,
            *savings.as_mut_ptr().offset(maxLength as isize),
            *savings.as_mut_ptr().offset(maxLength as isize) as core::ffi::c_double
                / maxLength as core::ffi::c_double,
        );
        fflush(stderr);
    }
    solution.pos = pos as u32;
    solution.length = maxLength as u32;
    solution.savings = *savings.as_mut_ptr().offset(maxLength as isize);
    let mut id_0: u32 = 0;
    id_0 = start;
    while id_0 < end {
        let mut p: u32 = 0;
        let mut pEnd: u32 = 0;
        let mut length_3: u32 = 0;
        let testedPos = *suffix.offset(id_0 as isize);
        if testedPos as size_t == pos {
            length_3 = solution.length;
        } else {
            length_3 = ZDICT_count(
                b.offset(pos as isize) as *const core::ffi::c_void,
                b.offset(testedPos as isize) as *const core::ffi::c_void,
            ) as u32;
            if length_3 > solution.length {
                length_3 = solution.length;
            }
        }
        pEnd = testedPos.wrapping_add(length_3);
        p = testedPos;
        while p < pEnd {
            *doneMarks.offset(p as isize) = 1;
            p = p.wrapping_add(1);
        }
        id_0 = id_0.wrapping_add(1);
    }
    solution
}
unsafe fn isIncluded(
    mut in_0: *const core::ffi::c_void,
    mut container: *const core::ffi::c_void,
    mut length: size_t,
) -> core::ffi::c_int {
    let ip = in_0 as *const core::ffi::c_char;
    let into = container as *const core::ffi::c_char;
    let mut u: size_t = 0;
    u = 0;
    while u < length {
        if *ip.offset(u as isize) as core::ffi::c_int
            != *into.offset(u as isize) as core::ffi::c_int
        {
            break;
        }
        u = u.wrapping_add(1);
    }
    (u == length) as core::ffi::c_int
}
unsafe fn ZDICT_tryMerge(
    mut table: *mut dictItem,
    mut elt: dictItem,
    mut eltNbToSkip: u32,
    mut buffer: *const core::ffi::c_void,
) -> u32 {
    let tableSize = (*table).pos;
    let eltEnd = (elt.pos).wrapping_add(elt.length);
    let buf = buffer as *const core::ffi::c_char;
    let mut u: u32 = 0;
    u = 1;
    while u < tableSize {
        if (u != eltNbToSkip)
            && (*table.offset(u as isize)).pos > elt.pos
            && (*table.offset(u as isize)).pos <= eltEnd
        {
            let addedLength = ((*table.offset(u as isize)).pos).wrapping_sub(elt.pos);
            let fresh2 = &mut (*table.offset(u as isize)).length;
            *fresh2 = (*fresh2).wrapping_add(addedLength);
            (*table.offset(u as isize)).pos = elt.pos;
            let fresh3 = &mut (*table.offset(u as isize)).savings;
            *fresh3 = (*fresh3).wrapping_add(elt.savings * addedLength / elt.length);
            let fresh4 = &mut (*table.offset(u as isize)).savings;
            *fresh4 = (*fresh4).wrapping_add(elt.length / 8);
            elt = *table.offset(u as isize);
            while u > 1 && (*table.offset(u.wrapping_sub(1) as isize)).savings < elt.savings {
                *table.offset(u as isize) = *table.offset(u.wrapping_sub(1) as isize);
                u = u.wrapping_sub(1);
            }
            *table.offset(u as isize) = elt;
            return u;
        }
        u = u.wrapping_add(1);
    }
    u = 1;
    while u < tableSize {
        if u != eltNbToSkip {
            if ((*table.offset(u as isize)).pos).wrapping_add((*table.offset(u as isize)).length)
                >= elt.pos
                && (*table.offset(u as isize)).pos < elt.pos
            {
                let addedLength_0 = eltEnd as core::ffi::c_int
                    - ((*table.offset(u as isize)).pos)
                        .wrapping_add((*table.offset(u as isize)).length)
                        as core::ffi::c_int;
                let fresh5 = &mut (*table.offset(u as isize)).savings;
                *fresh5 = (*fresh5).wrapping_add(elt.length / 8);
                if addedLength_0 > 0 {
                    let fresh6 = &mut (*table.offset(u as isize)).length;
                    *fresh6 = (*fresh6 as core::ffi::c_uint)
                        .wrapping_add(addedLength_0 as core::ffi::c_uint)
                        as u32 as u32;
                    let fresh7 = &mut (*table.offset(u as isize)).savings;
                    *fresh7 = (*fresh7 as core::ffi::c_uint).wrapping_add(
                        (elt.savings)
                            .wrapping_mul(addedLength_0 as core::ffi::c_uint)
                            .wrapping_div(elt.length),
                    ) as u32 as u32;
                }
                elt = *table.offset(u as isize);
                while u > 1 && (*table.offset(u.wrapping_sub(1) as isize)).savings < elt.savings {
                    *table.offset(u as isize) = *table.offset(u.wrapping_sub(1) as isize);
                    u = u.wrapping_sub(1);
                }
                *table.offset(u as isize) = elt;
                return u;
            }
            if MEM_read64(
                buf.offset((*table.offset(u as isize)).pos as isize) as *const core::ffi::c_void
            ) == MEM_read64(buf.offset(elt.pos as isize).offset(1) as *const core::ffi::c_void)
                && isIncluded(
                    buf.offset((*table.offset(u as isize)).pos as isize)
                        as *const core::ffi::c_void,
                    buf.offset(elt.pos as isize).offset(1) as *const core::ffi::c_void,
                    (*table.offset(u as isize)).length as size_t,
                ) != 0
            {
                let addedLength_1 =
                    (if (elt.length).wrapping_sub((*table.offset(u as isize)).length) > 1 {
                        (elt.length).wrapping_sub((*table.offset(u as isize)).length)
                    } else {
                        1
                    }) as size_t;
                (*table.offset(u as isize)).pos = elt.pos;
                let fresh8 = &mut (*table.offset(u as isize)).savings;
                *fresh8 = (*fresh8).wrapping_add(
                    (elt.savings as size_t * addedLength_1 / elt.length as size_t) as u32,
                );
                (*table.offset(u as isize)).length =
                    if elt.length < ((*table.offset(u as isize)).length).wrapping_add(1) {
                        elt.length
                    } else {
                        ((*table.offset(u as isize)).length).wrapping_add(1)
                    };
                return u;
            }
        }
        u = u.wrapping_add(1);
    }
    0
}
unsafe fn ZDICT_removeDictItem(mut table: *mut dictItem, mut id: u32) {
    let max = (*table.offset(0)).pos;
    let mut u: u32 = 0;
    if id == 0 {
        return;
    }
    u = id;
    while u < max.wrapping_sub(1) {
        *table.offset(u as isize) = *table.offset(u.wrapping_add(1) as isize);
        u = u.wrapping_add(1);
    }
    (*table).pos = ((*table).pos).wrapping_sub(1);
    (*table).pos;
}
unsafe fn ZDICT_insertDictItem(
    mut table: *mut dictItem,
    mut maxSize: u32,
    mut elt: dictItem,
    mut buffer: *const core::ffi::c_void,
) {
    let mut mergeId = ZDICT_tryMerge(table, elt, 0, buffer);
    if mergeId != 0 {
        let mut newMerge = 1;
        while newMerge != 0 {
            newMerge = ZDICT_tryMerge(table, *table.offset(mergeId as isize), mergeId, buffer);
            if newMerge != 0 {
                ZDICT_removeDictItem(table, mergeId);
            }
            mergeId = newMerge;
        }
        return;
    }
    let mut current: u32 = 0;
    let mut nextElt = (*table).pos;
    if nextElt >= maxSize {
        nextElt = maxSize.wrapping_sub(1);
    }
    current = nextElt.wrapping_sub(1);
    while (*table.offset(current as isize)).savings < elt.savings {
        *table.offset(current.wrapping_add(1) as isize) = *table.offset(current as isize);
        current = current.wrapping_sub(1);
    }
    *table.offset(current.wrapping_add(1) as isize) = elt;
    (*table).pos = nextElt.wrapping_add(1);
}
unsafe fn ZDICT_dictSize(mut dictList: *const dictItem) -> u32 {
    let mut u: u32 = 0;
    let mut dictSize = 0u32;
    u = 1;
    while u < (*dictList.offset(0)).pos {
        dictSize = dictSize.wrapping_add((*dictList.offset(u as isize)).length);
        u = u.wrapping_add(1);
    }
    dictSize
}
unsafe fn ZDICT_trainBuffer_legacy(
    mut dictList: *mut dictItem,
    mut dictListSize: u32,
    buffer: *const core::ffi::c_void,
    mut bufferSize: size_t,
    mut fileSizes: *const size_t,
    mut nbFiles: core::ffi::c_uint,
    mut minRatio: core::ffi::c_uint,
    mut notificationLevel: u32,
) -> size_t {
    let suffix0 = malloc(
        bufferSize
            .wrapping_add(2)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as size_t),
    ) as *mut core::ffi::c_uint;
    let suffix = suffix0.offset(1);
    let mut reverseSuffix =
        malloc(bufferSize.wrapping_mul(::core::mem::size_of::<u32>() as size_t)) as *mut u32;
    let mut doneMarks = malloc(
        bufferSize
            .wrapping_add(16)
            .wrapping_mul(::core::mem::size_of::<u8>() as size_t),
    ) as *mut u8;
    let mut filePos =
        malloc((nbFiles as size_t).wrapping_mul(::core::mem::size_of::<u32>() as size_t))
            as *mut u32;
    let mut result = 0;
    let mut displayClock = 0;
    let refreshRate = CLOCKS_PER_SEC as __clock_t * 3 / 10;
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"\r%70s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    if suffix0.is_null() || reverseSuffix.is_null() || doneMarks.is_null() || filePos.is_null() {
        result = -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    } else {
        if minRatio < MINRATIO as core::ffi::c_uint {
            minRatio = MINRATIO as core::ffi::c_uint;
        }
        memset(
            doneMarks as *mut core::ffi::c_void,
            0,
            bufferSize.wrapping_add(16),
        );
        if bufferSize > ZDICT_MAX_SAMPLES_SIZE as size_t && notificationLevel >= 3 {
            fprintf(
                stderr,
                b"sample set too large : reduced to %u MB ...\n\0" as *const u8
                    as *const core::ffi::c_char,
                (2000) << 20 >> 20,
            );
            fflush(stderr);
        }
        while bufferSize > ZDICT_MAX_SAMPLES_SIZE as size_t {
            nbFiles = nbFiles.wrapping_sub(1);
            bufferSize = bufferSize.wrapping_sub(*fileSizes.offset(nbFiles as isize));
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"sorting %u files of total size %u MB ...\n\0" as *const u8
                    as *const core::ffi::c_char,
                nbFiles,
                (bufferSize >> 20) as core::ffi::c_uint,
            );
            fflush(stderr);
        }
        let divSuftSortResult = divsufsort(
            buffer as *const core::ffi::c_uchar,
            suffix as *mut core::ffi::c_int,
            bufferSize as core::ffi::c_int,
            0,
        );
        if divSuftSortResult != 0 {
            result = -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
        } else {
            *suffix.offset(bufferSize as isize) = bufferSize as core::ffi::c_uint;
            *suffix0.offset(0) = bufferSize as core::ffi::c_uint;
            let mut pos: size_t = 0;
            pos = 0;
            while pos < bufferSize {
                *reverseSuffix.offset(*suffix.offset(pos as isize) as isize) = pos as u32;
                pos = pos.wrapping_add(1);
            }
            *filePos.offset(0) = 0;
            pos = 1;
            while pos < nbFiles as size_t {
                *filePos.offset(pos as isize) = (*filePos.offset(pos.wrapping_sub(1) as isize)
                    as size_t)
                    .wrapping_add(*fileSizes.offset(pos.wrapping_sub(1) as isize))
                    as u32;
                pos = pos.wrapping_add(1);
            }
            if notificationLevel >= 2 {
                fprintf(
                    stderr,
                    b"finding patterns ... \n\0" as *const u8 as *const core::ffi::c_char,
                );
                fflush(stderr);
            }
            if notificationLevel >= 3 {
                fprintf(
                    stderr,
                    b"minimum ratio : %u \n\0" as *const u8 as *const core::ffi::c_char,
                    minRatio,
                );
                fflush(stderr);
            }
            let mut cursor: u32 = 0;
            cursor = 0;
            while (cursor as size_t) < bufferSize {
                let mut solution = dictItem {
                    pos: 0,
                    length: 0,
                    savings: 0,
                };
                if *doneMarks.offset(cursor as isize) != 0 {
                    cursor = cursor.wrapping_add(1);
                } else {
                    solution = ZDICT_analyzePos(
                        doneMarks,
                        suffix,
                        *reverseSuffix.offset(cursor as isize),
                        buffer,
                        minRatio,
                        notificationLevel,
                    );
                    if solution.length == 0 {
                        cursor = cursor.wrapping_add(1);
                    } else {
                        ZDICT_insertDictItem(dictList, dictListSize, solution, buffer);
                        cursor = cursor.wrapping_add(solution.length);
                        if notificationLevel >= 2 {
                            if ZDICT_clockSpan(displayClock) > refreshRate {
                                displayClock = clock();
                                fprintf(
                                    stderr,
                                    b"\r%4.2f %% \r\0" as *const u8 as *const core::ffi::c_char,
                                    cursor as core::ffi::c_double
                                        / bufferSize as core::ffi::c_double
                                        * 100.0f64,
                                );
                                fflush(stderr);
                            }
                            if notificationLevel >= 4 {
                                fflush(stderr);
                            }
                        }
                    }
                }
            }
        }
    }
    free(suffix0 as *mut core::ffi::c_void);
    free(reverseSuffix as *mut core::ffi::c_void);
    free(doneMarks as *mut core::ffi::c_void);
    free(filePos as *mut core::ffi::c_void);
    result
}
unsafe fn ZDICT_fillNoise(mut buffer: *mut core::ffi::c_void, mut length: size_t) {
    let prime1 = 2654435761 as core::ffi::c_uint;
    let prime2 = 2246822519 as core::ffi::c_uint;
    let mut acc = prime1;
    let mut p = 0;
    p = 0;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut core::ffi::c_uchar).offset(p as isize) = (acc >> 21) as core::ffi::c_uchar;
        p = p.wrapping_add(1);
    }
}
pub const MAXREPOFFSET: core::ffi::c_int = 1024;
unsafe fn ZDICT_countEStats(
    mut esr: EStats_ress_t,
    mut params: *const ZSTD_parameters,
    mut countLit: *mut core::ffi::c_uint,
    mut offsetcodeCount: *mut core::ffi::c_uint,
    mut matchlengthCount: *mut core::ffi::c_uint,
    mut litlengthCount: *mut core::ffi::c_uint,
    mut repOffsets: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut notificationLevel: u32,
) {
    let blockSizeMax = (if ((1) << 17) < (1) << (*params).cParams.windowLog {
        (1) << 17
    } else {
        (1) << (*params).cParams.windowLog
    }) as size_t;
    let mut cSize: size_t = 0;
    if srcSize > blockSizeMax {
        srcSize = blockSizeMax;
    }
    let errorCode = ZSTD_compressBegin_usingCDict_deprecated(esr.zc, esr.dict);
    if ERR_isError(errorCode) != 0 {
        if notificationLevel >= 1 {
            fprintf(
                stderr,
                b"warning : ZSTD_compressBegin_usingCDict failed \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return;
    }
    cSize = ZSTD_compressBlock_deprecated(
        esr.zc,
        esr.workPlace,
        ZSTD_BLOCKSIZE_MAX as size_t,
        src,
        srcSize,
    );
    if ERR_isError(cSize) != 0 {
        if notificationLevel >= 3 {
            fprintf(
                stderr,
                b"warning : could not compress sample size %u \n\0" as *const u8
                    as *const core::ffi::c_char,
                srcSize as core::ffi::c_uint,
            );
            fflush(stderr);
        }
        return;
    }
    if cSize != 0 {
        let seqStorePtr = ZSTD_getSeqStore(esr.zc);
        let mut bytePtr = core::ptr::null::<u8>();
        bytePtr = (*seqStorePtr).litStart;
        while bytePtr < (*seqStorePtr).lit as *const u8 {
            let fresh9 = &mut (*countLit.offset(*bytePtr as isize));
            *fresh9 = (*fresh9).wrapping_add(1);
            bytePtr = bytePtr.offset(1);
        }
        let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
            as core::ffi::c_long as u32;
        ZSTD_seqToCodes(seqStorePtr);
        let mut codePtr: *const u8 = (*seqStorePtr).ofCode;
        let mut u: u32 = 0;
        u = 0;
        while u < nbSeq {
            let fresh10 = &mut (*offsetcodeCount.offset(*codePtr.offset(u as isize) as isize));
            *fresh10 = (*fresh10).wrapping_add(1);
            u = u.wrapping_add(1);
        }
        let mut codePtr_0: *const u8 = (*seqStorePtr).mlCode;
        let mut u_0: u32 = 0;
        u_0 = 0;
        while u_0 < nbSeq {
            let fresh11 = &mut (*matchlengthCount.offset(*codePtr_0.offset(u_0 as isize) as isize));
            *fresh11 = (*fresh11).wrapping_add(1);
            u_0 = u_0.wrapping_add(1);
        }
        let mut codePtr_1: *const u8 = (*seqStorePtr).llCode;
        let mut u_1: u32 = 0;
        u_1 = 0;
        while u_1 < nbSeq {
            let fresh12 = &mut (*litlengthCount.offset(*codePtr_1.offset(u_1 as isize) as isize));
            *fresh12 = (*fresh12).wrapping_add(1);
            u_1 = u_1.wrapping_add(1);
        }
        if nbSeq >= 2 {
            let seq: *const SeqDef = (*seqStorePtr).sequencesStart;
            let mut offset1 = ((*seq.offset(0)).offBase).wrapping_sub(ZSTD_REP_NUM as u32);
            let mut offset2 = ((*seq.offset(1)).offBase).wrapping_sub(ZSTD_REP_NUM as u32);
            if offset1 >= MAXREPOFFSET as u32 {
                offset1 = 0;
            }
            if offset2 >= MAXREPOFFSET as u32 {
                offset2 = 0;
            }
            let fresh13 = &mut (*repOffsets.offset(offset1 as isize));
            *fresh13 = (*fresh13).wrapping_add(3);
            let fresh14 = &mut (*repOffsets.offset(offset2 as isize));
            *fresh14 = (*fresh14).wrapping_add(1);
        }
    }
}
unsafe fn ZDICT_totalSampleSize(
    mut fileSizes: *const size_t,
    mut nbFiles: core::ffi::c_uint,
) -> size_t {
    let mut total = 0 as size_t;
    let mut u: core::ffi::c_uint = 0;
    u = 0;
    while u < nbFiles {
        total = total.wrapping_add(*fileSizes.offset(u as isize));
        u = u.wrapping_add(1);
    }
    total
}
unsafe fn ZDICT_insertSortCount(mut table: *mut offsetCount_t, mut val: u32, mut count: u32) {
    let mut u: u32 = 0;
    (*table.offset(ZSTD_REP_NUM as isize)).offset = val;
    (*table.offset(ZSTD_REP_NUM as isize)).count = count;
    u = ZSTD_REP_NUM as u32;
    while u > 0 {
        let mut tmp = offsetCount_t {
            offset: 0,
            count: 0,
        };
        if (*table.offset(u.wrapping_sub(1) as isize)).count >= (*table.offset(u as isize)).count {
            break;
        }
        tmp = *table.offset(u.wrapping_sub(1) as isize);
        *table.offset(u.wrapping_sub(1) as isize) = *table.offset(u as isize);
        *table.offset(u as isize) = tmp;
        u = u.wrapping_sub(1);
    }
}
unsafe fn ZDICT_flatLit(mut countLit: *mut core::ffi::c_uint) {
    let mut u: core::ffi::c_int = 0;
    u = 1;
    while u < 256 {
        *countLit.offset(u as isize) = 2;
        u += 1;
    }
    *countLit.offset(0) = 4;
    *countLit.offset(253) = 1;
    *countLit.offset(254) = 1;
}
pub const OFFCODE_MAX: core::ffi::c_int = 30;
unsafe fn ZDICT_analyzeEntropy(
    mut dstBuffer: *mut core::ffi::c_void,
    mut maxDstSize: size_t,
    mut compressionLevel: core::ffi::c_int,
    mut srcBuffer: *const core::ffi::c_void,
    mut fileSizes: *const size_t,
    mut nbFiles: core::ffi::c_uint,
    mut dictBuffer: *const core::ffi::c_void,
    mut dictBufferSize: size_t,
    mut notificationLevel: core::ffi::c_uint,
) -> size_t {
    let mut countLit: [core::ffi::c_uint; 256] = [0; 256];
    let mut hufTable: [HUF_CElt; 257] = [0; 257];
    let mut offcodeCount: [core::ffi::c_uint; 31] = [0; 31];
    let mut offcodeNCount: [core::ffi::c_short; 31] = [0; 31];
    let mut offcodeMax =
        ZSTD_highbit32(dictBufferSize.wrapping_add((128 * ((1) << 10)) as size_t) as u32);
    let mut matchLengthCount: [core::ffi::c_uint; 53] = [0; 53];
    let mut matchLengthNCount: [core::ffi::c_short; 53] = [0; 53];
    let mut litLengthCount: [core::ffi::c_uint; 36] = [0; 36];
    let mut litLengthNCount: [core::ffi::c_short; 36] = [0; 36];
    let mut repOffset: [u32; 1024] = [0; 1024];
    let mut bestRepOffset: [offsetCount_t; 4] = [offsetCount_t {
        offset: 0,
        count: 0,
    }; 4];
    let mut esr = {
        EStats_ress_t {
            dict: NULL as *mut ZSTD_CDict,
            zc: NULL as *mut ZSTD_CCtx,
            workPlace: NULL as *mut core::ffi::c_void,
        }
    };
    let mut params = ZSTD_parameters {
        cParams: ZSTD_compressionParameters {
            windowLog: 0,
            chainLog: 0,
            hashLog: 0,
            searchLog: 0,
            minMatch: 0,
            targetLength: 0,
            strategy: 0,
        },
        fParams: ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        },
    };
    let mut u: u32 = 0;
    let mut huffLog = 11;
    let mut Offlog = OffFSELog as u32;
    let mut mlLog = MLFSELog as u32;
    let mut llLog = LLFSELog as u32;
    let mut total: u32 = 0;
    let mut pos = 0 as size_t;
    let mut errorCode: size_t = 0;
    let mut eSize = 0;
    let totalSrcSize = ZDICT_totalSampleSize(fileSizes, nbFiles);
    let averageSampleSize = totalSrcSize
        / nbFiles.wrapping_add((nbFiles == 0) as core::ffi::c_int as core::ffi::c_uint) as size_t;
    let mut dstPtr = dstBuffer as *mut u8;
    let mut wksp: [u32; 1216] = [0; 1216];
    if offcodeMax > OFFCODE_MAX as u32 {
        eSize = -(ZSTD_error_dictionaryCreation_failed as core::ffi::c_int) as size_t;
    } else {
        u = 0;
        while u < 256 {
            *countLit.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }
        u = 0;
        while u <= offcodeMax {
            *offcodeCount.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }
        u = 0;
        while u <= MaxML as u32 {
            *matchLengthCount.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }
        u = 0;
        while u <= MaxLL as u32 {
            *litLengthCount.as_mut_ptr().offset(u as isize) = 1;
            u = u.wrapping_add(1);
        }
        ptr::write_bytes(
            repOffset.as_mut_ptr() as *mut u8,
            0,
            ::core::mem::size_of::<[u32; 1024]>(),
        );
        let fresh15 = &mut (*repOffset.as_mut_ptr().offset(8));
        *fresh15 = 1;
        let fresh16 = &mut (*repOffset.as_mut_ptr().offset(4));
        *fresh16 = *fresh15;
        *repOffset.as_mut_ptr().offset(1) = *fresh16;
        ptr::write_bytes(
            bestRepOffset.as_mut_ptr() as *mut u8,
            0,
            ::core::mem::size_of::<[offsetCount_t; 4]>(),
        );
        if compressionLevel == 0 {
            compressionLevel = ZSTD_CLEVEL_DEFAULT;
        }
        params = ZSTD_getParams(
            compressionLevel,
            averageSampleSize as core::ffi::c_ulonglong,
            dictBufferSize,
        );
        esr.dict = ZSTD_createCDict_advanced(
            dictBuffer,
            dictBufferSize,
            ZSTD_dlm_byRef,
            ZSTD_dct_rawContent,
            params.cParams,
            ZSTD_defaultCMem,
        );
        esr.zc = ZSTD_createCCtx();
        esr.workPlace = malloc(ZSTD_BLOCKSIZE_MAX as size_t);
        if (esr.dict).is_null() || (esr.zc).is_null() || (esr.workPlace).is_null() {
            eSize = -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
            if notificationLevel >= 1 {
                fprintf(
                    stderr,
                    b"Not enough memory \n\0" as *const u8 as *const core::ffi::c_char,
                );
                fflush(stderr);
            }
        } else {
            u = 0;
            while u < nbFiles {
                ZDICT_countEStats(
                    esr,
                    &mut params,
                    countLit.as_mut_ptr(),
                    offcodeCount.as_mut_ptr(),
                    matchLengthCount.as_mut_ptr(),
                    litLengthCount.as_mut_ptr(),
                    repOffset.as_mut_ptr(),
                    (srcBuffer as *const core::ffi::c_char).offset(pos as isize)
                        as *const core::ffi::c_void,
                    *fileSizes.offset(u as isize),
                    notificationLevel,
                );
                pos = pos.wrapping_add(*fileSizes.offset(u as isize));
                u = u.wrapping_add(1);
            }
            if notificationLevel >= 4 {
                if notificationLevel >= 4 {
                    fprintf(
                        stderr,
                        b"Offset Code Frequencies : \n\0" as *const u8 as *const core::ffi::c_char,
                    );
                    fflush(stderr);
                }
                u = 0;
                while u <= offcodeMax {
                    if notificationLevel >= 4 {
                        fprintf(
                            stderr,
                            b"%2u :%7u \n\0" as *const u8 as *const core::ffi::c_char,
                            u,
                            *offcodeCount.as_mut_ptr().offset(u as isize),
                        );
                        fflush(stderr);
                    }
                    u = u.wrapping_add(1);
                }
            }
            let mut maxNbBits = HUF_buildCTable_wksp(
                hufTable.as_mut_ptr(),
                countLit.as_mut_ptr(),
                255,
                huffLog,
                wksp.as_mut_ptr() as *mut core::ffi::c_void,
                ::core::mem::size_of::<[u32; 1216]>() as size_t,
            );
            if ERR_isError(maxNbBits) != 0 {
                eSize = maxNbBits;
                if notificationLevel >= 1 {
                    fprintf(
                        stderr,
                        b" HUF_buildCTable error \n\0" as *const u8 as *const core::ffi::c_char,
                    );
                    fflush(stderr);
                }
            } else {
                if maxNbBits == 8 {
                    if notificationLevel >= 2 {
                        fprintf(
                            stderr,
                            b"warning : pathological dataset : literals are not compressible : samples are noisy or too regular \n\0"
                                as *const u8 as *const core::ffi::c_char,
                        );
                        fflush(stderr);
                    }
                    ZDICT_flatLit(countLit.as_mut_ptr());
                    maxNbBits = HUF_buildCTable_wksp(
                        hufTable.as_mut_ptr(),
                        countLit.as_mut_ptr(),
                        255,
                        huffLog,
                        wksp.as_mut_ptr() as *mut core::ffi::c_void,
                        ::core::mem::size_of::<[u32; 1216]>() as size_t,
                    );
                }
                huffLog = maxNbBits as u32;
                let mut offset: u32 = 0;
                offset = 1;
                while offset < MAXREPOFFSET as u32 {
                    ZDICT_insertSortCount(
                        bestRepOffset.as_mut_ptr(),
                        offset,
                        *repOffset.as_mut_ptr().offset(offset as isize),
                    );
                    offset = offset.wrapping_add(1);
                }
                total = 0;
                u = 0;
                while u <= offcodeMax {
                    total = (total as core::ffi::c_uint)
                        .wrapping_add(*offcodeCount.as_mut_ptr().offset(u as isize))
                        as u32 as u32;
                    u = u.wrapping_add(1);
                }
                errorCode = FSE_normalizeCount(
                    offcodeNCount.as_mut_ptr(),
                    Offlog,
                    offcodeCount.as_mut_ptr(),
                    total as size_t,
                    offcodeMax,
                    1,
                );
                if ERR_isError(errorCode) != 0 {
                    eSize = errorCode;
                    if notificationLevel >= 1 {
                        fprintf(
                            stderr,
                            b"FSE_normalizeCount error with offcodeCount \n\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                        fflush(stderr);
                    }
                } else {
                    Offlog = errorCode as u32;
                    total = 0;
                    u = 0;
                    while u <= MaxML as u32 {
                        total = (total as core::ffi::c_uint)
                            .wrapping_add(*matchLengthCount.as_mut_ptr().offset(u as isize))
                            as u32 as u32;
                        u = u.wrapping_add(1);
                    }
                    errorCode = FSE_normalizeCount(
                        matchLengthNCount.as_mut_ptr(),
                        mlLog,
                        matchLengthCount.as_mut_ptr(),
                        total as size_t,
                        MaxML as core::ffi::c_uint,
                        1,
                    );
                    if ERR_isError(errorCode) != 0 {
                        eSize = errorCode;
                        if notificationLevel >= 1 {
                            fprintf(
                                stderr,
                                b"FSE_normalizeCount error with matchLengthCount \n\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                            fflush(stderr);
                        }
                    } else {
                        mlLog = errorCode as u32;
                        total = 0;
                        u = 0;
                        while u <= MaxLL as u32 {
                            total = (total as core::ffi::c_uint)
                                .wrapping_add(*litLengthCount.as_mut_ptr().offset(u as isize))
                                as u32 as u32;
                            u = u.wrapping_add(1);
                        }
                        errorCode = FSE_normalizeCount(
                            litLengthNCount.as_mut_ptr(),
                            llLog,
                            litLengthCount.as_mut_ptr(),
                            total as size_t,
                            MaxLL as core::ffi::c_uint,
                            1,
                        );
                        if ERR_isError(errorCode) != 0 {
                            eSize = errorCode;
                            if notificationLevel >= 1 {
                                fprintf(
                                    stderr,
                                    b"FSE_normalizeCount error with litLengthCount \n\0"
                                        as *const u8
                                        as *const core::ffi::c_char,
                                );
                                fflush(stderr);
                            }
                        } else {
                            llLog = errorCode as u32;
                            let hhSize = HUF_writeCTable_wksp(
                                dstPtr as *mut core::ffi::c_void,
                                maxDstSize,
                                hufTable.as_mut_ptr(),
                                255,
                                huffLog,
                                wksp.as_mut_ptr() as *mut core::ffi::c_void,
                                ::core::mem::size_of::<[u32; 1216]>() as size_t,
                            );
                            if ERR_isError(hhSize) != 0 {
                                eSize = hhSize;
                                if notificationLevel >= 1 {
                                    fprintf(
                                        stderr,
                                        b"HUF_writeCTable error \n\0" as *const u8
                                            as *const core::ffi::c_char,
                                    );
                                    fflush(stderr);
                                }
                            } else {
                                dstPtr = dstPtr.offset(hhSize as isize);
                                maxDstSize = maxDstSize.wrapping_sub(hhSize);
                                eSize = eSize.wrapping_add(hhSize);
                                let ohSize = FSE_writeNCount(
                                    dstPtr as *mut core::ffi::c_void,
                                    maxDstSize,
                                    offcodeNCount.as_mut_ptr(),
                                    OFFCODE_MAX as core::ffi::c_uint,
                                    Offlog,
                                );
                                if ERR_isError(ohSize) != 0 {
                                    eSize = ohSize;
                                    if notificationLevel >= 1 {
                                        fprintf(
                                            stderr,
                                            b"FSE_writeNCount error with offcodeNCount \n\0"
                                                as *const u8
                                                as *const core::ffi::c_char,
                                        );
                                        fflush(stderr);
                                    }
                                } else {
                                    dstPtr = dstPtr.offset(ohSize as isize);
                                    maxDstSize = maxDstSize.wrapping_sub(ohSize);
                                    eSize = eSize.wrapping_add(ohSize);
                                    let mhSize = FSE_writeNCount(
                                        dstPtr as *mut core::ffi::c_void,
                                        maxDstSize,
                                        matchLengthNCount.as_mut_ptr(),
                                        MaxML as core::ffi::c_uint,
                                        mlLog,
                                    );
                                    if ERR_isError(mhSize) != 0 {
                                        eSize = mhSize;
                                        if notificationLevel >= 1 {
                                            fprintf(
                                                stderr,
                                                b"FSE_writeNCount error with matchLengthNCount \n\0"
                                                    as *const u8
                                                    as *const core::ffi::c_char,
                                            );
                                            fflush(stderr);
                                        }
                                    } else {
                                        dstPtr = dstPtr.offset(mhSize as isize);
                                        maxDstSize = maxDstSize.wrapping_sub(mhSize);
                                        eSize = eSize.wrapping_add(mhSize);
                                        let lhSize = FSE_writeNCount(
                                            dstPtr as *mut core::ffi::c_void,
                                            maxDstSize,
                                            litLengthNCount.as_mut_ptr(),
                                            MaxLL as core::ffi::c_uint,
                                            llLog,
                                        );
                                        if ERR_isError(lhSize) != 0 {
                                            eSize = lhSize;
                                            if notificationLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"FSE_writeNCount error with litlengthNCount \n\0"
                                                        as *const u8 as *const core::ffi::c_char,
                                                );
                                                fflush(stderr);
                                            }
                                        } else {
                                            dstPtr = dstPtr.offset(lhSize as isize);
                                            maxDstSize = maxDstSize.wrapping_sub(lhSize);
                                            eSize = eSize.wrapping_add(lhSize);
                                            if maxDstSize < 12 {
                                                eSize = -(ZSTD_error_dstSize_tooSmall
                                                    as core::ffi::c_int)
                                                    as size_t;
                                                if notificationLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"not enough space to write RepOffsets \n\0"
                                                            as *const u8
                                                            as *const core::ffi::c_char,
                                                    );
                                                    fflush(stderr);
                                                }
                                            } else {
                                                MEM_writeLE32(
                                                    dstPtr.offset(0) as *mut core::ffi::c_void,
                                                    *repStartValue.as_ptr().offset(0),
                                                );
                                                MEM_writeLE32(
                                                    dstPtr.offset(4) as *mut core::ffi::c_void,
                                                    *repStartValue.as_ptr().offset(1),
                                                );
                                                MEM_writeLE32(
                                                    dstPtr.offset(8) as *mut core::ffi::c_void,
                                                    *repStartValue.as_ptr().offset(2),
                                                );
                                                eSize = eSize.wrapping_add(12);
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
    }
    ZSTD_freeCDict(esr.dict);
    ZSTD_freeCCtx(esr.zc);
    free(esr.workPlace);
    eSize
}
unsafe fn ZDICT_maxRep(mut reps: *const u32) -> u32 {
    let mut maxRep = *reps.offset(0);
    let mut r: core::ffi::c_int = 0;
    r = 1;
    while r < ZSTD_REP_NUM {
        maxRep = if maxRep > *reps.offset(r as isize) {
            maxRep
        } else {
            *reps.offset(r as isize)
        };
        r += 1;
    }
    maxRep
}
#[export_name = crate::prefix!(ZDICT_finalizeDictionary)]
pub unsafe extern "C" fn ZDICT_finalizeDictionary(
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut customDictContent: *const core::ffi::c_void,
    mut dictContentSize: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
    mut params: ZDICT_params_t,
) -> size_t {
    let mut hSize: size_t = 0;
    let mut header: [u8; 256] = [0; 256];
    let compressionLevel = if params.compressionLevel == 0 {
        ZSTD_CLEVEL_DEFAULT
    } else {
        params.compressionLevel
    };
    let notificationLevel = params.notificationLevel;
    let minContentSize = ZDICT_maxRep(repStartValue.as_ptr()) as size_t;
    let mut paddingSize: size_t = 0;
    if dictBufferCapacity < dictContentSize {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    MEM_writeLE32(
        header.as_mut_ptr() as *mut core::ffi::c_void,
        ZSTD_MAGIC_DICTIONARY,
    );
    let randomID = ZSTD_XXH64(customDictContent, dictContentSize as usize, 0);
    let compliantID = (randomID % ((1 as core::ffi::c_uint) << 31).wrapping_sub(32768) as u64)
        .wrapping_add(32768) as u32;
    let dictID = if params.dictID != 0 {
        params.dictID
    } else {
        compliantID
    };
    MEM_writeLE32(
        header.as_mut_ptr().offset(4) as *mut core::ffi::c_void,
        dictID,
    );
    hSize = 8;
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"\r%70s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"statistics ... \n\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    let eSize = ZDICT_analyzeEntropy(
        header.as_mut_ptr().offset(hSize as isize) as *mut core::ffi::c_void,
        (HBUFFSIZE as size_t).wrapping_sub(hSize),
        compressionLevel,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        customDictContent,
        dictContentSize,
        notificationLevel,
    );
    if ZDICT_isError(eSize) != 0 {
        return eSize;
    }
    hSize = hSize.wrapping_add(eSize);
    if hSize.wrapping_add(dictContentSize) > dictBufferCapacity {
        dictContentSize = dictBufferCapacity.wrapping_sub(hSize);
    }
    if dictContentSize < minContentSize {
        if hSize.wrapping_add(minContentSize) > dictBufferCapacity {
            return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
        }
        paddingSize = minContentSize.wrapping_sub(dictContentSize);
    } else {
        paddingSize = 0;
    }
    let dictSize = hSize
        .wrapping_add(paddingSize)
        .wrapping_add(dictContentSize);
    let outDictHeader = dictBuffer as *mut u8;
    let outDictPadding = outDictHeader.offset(hSize as isize);
    let outDictContent = outDictPadding.offset(paddingSize as isize);
    memmove(
        outDictContent as *mut core::ffi::c_void,
        customDictContent,
        dictContentSize,
    );
    memcpy(
        outDictHeader as *mut core::ffi::c_void,
        header.as_mut_ptr() as *const core::ffi::c_void,
        hSize,
    );
    memset(outDictPadding as *mut core::ffi::c_void, 0, paddingSize);
    dictSize
}
pub const HBUFFSIZE: core::ffi::c_int = 256;
unsafe fn ZDICT_addEntropyTablesFromBuffer_advanced(
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictContentSize: size_t,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
    mut params: ZDICT_params_t,
) -> size_t {
    let compressionLevel = if params.compressionLevel == 0 {
        ZSTD_CLEVEL_DEFAULT
    } else {
        params.compressionLevel
    };
    let notificationLevel = params.notificationLevel;
    let mut hSize = 8;
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"\r%70s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    if notificationLevel >= 2 {
        fprintf(
            stderr,
            b"statistics ... \n\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    let eSize = ZDICT_analyzeEntropy(
        (dictBuffer as *mut core::ffi::c_char).offset(hSize as isize) as *mut core::ffi::c_void,
        dictBufferCapacity.wrapping_sub(hSize),
        compressionLevel,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        (dictBuffer as *mut core::ffi::c_char)
            .offset(dictBufferCapacity as isize)
            .offset(-(dictContentSize as isize)) as *const core::ffi::c_void,
        dictContentSize,
        notificationLevel,
    );
    if ZDICT_isError(eSize) != 0 {
        return eSize;
    }
    hSize = hSize.wrapping_add(eSize);
    MEM_writeLE32(dictBuffer, ZSTD_MAGIC_DICTIONARY);
    let randomID = ZSTD_XXH64(
        (dictBuffer as *mut core::ffi::c_char)
            .offset(dictBufferCapacity as isize)
            .offset(-(dictContentSize as isize)) as *const core::ffi::c_void,
        dictContentSize as usize,
        0,
    );
    let compliantID = (randomID % ((1 as core::ffi::c_uint) << 31).wrapping_sub(32768) as u64)
        .wrapping_add(32768) as u32;
    let dictID = if params.dictID != 0 {
        params.dictID
    } else {
        compliantID
    };
    MEM_writeLE32(
        (dictBuffer as *mut core::ffi::c_char).offset(4) as *mut core::ffi::c_void,
        dictID,
    );
    if hSize.wrapping_add(dictContentSize) < dictBufferCapacity {
        memmove(
            (dictBuffer as *mut core::ffi::c_char).offset(hSize as isize) as *mut core::ffi::c_void,
            (dictBuffer as *mut core::ffi::c_char)
                .offset(dictBufferCapacity as isize)
                .offset(-(dictContentSize as isize)) as *const core::ffi::c_void,
            dictContentSize,
        );
    }
    if dictBufferCapacity < hSize.wrapping_add(dictContentSize) {
        dictBufferCapacity
    } else {
        hSize.wrapping_add(dictContentSize)
    }
}
unsafe fn ZDICT_trainFromBuffer_unsafe_legacy(
    mut dictBuffer: *mut core::ffi::c_void,
    mut maxDictSize: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
    mut params: ZDICT_legacy_params_t,
) -> size_t {
    let dictListSize =
        if (if 10000 > nbSamples { 10000 } else { nbSamples }) > (maxDictSize / 16) as u32 {
            if 10000 > nbSamples {
                10000
            } else {
                nbSamples
            }
        } else {
            (maxDictSize / 16) as u32
        };
    let dictList =
        malloc((dictListSize as size_t).wrapping_mul(::core::mem::size_of::<dictItem>() as size_t))
            as *mut dictItem;
    let selectivity = if params.selectivityLevel == 0 {
        g_selectivity_default
    } else {
        params.selectivityLevel
    };
    let minRep = if selectivity > 30 {
        MINRATIO as core::ffi::c_uint
    } else {
        nbSamples >> selectivity
    };
    let targetDictSize = maxDictSize;
    let samplesBuffSize = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    let mut dictSize = 0;
    let notificationLevel = params.zParams.notificationLevel;
    if dictList.is_null() {
        return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    }
    if maxDictSize < ZDICT_DICTSIZE_MIN as size_t {
        free(dictList as *mut core::ffi::c_void);
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if samplesBuffSize < ZDICT_MIN_SAMPLES_SIZE as size_t {
        free(dictList as *mut core::ffi::c_void);
        return -(ZSTD_error_dictionaryCreation_failed as core::ffi::c_int) as size_t;
    }
    ZDICT_initDictItem(dictList);
    ZDICT_trainBuffer_legacy(
        dictList,
        dictListSize,
        samplesBuffer,
        samplesBuffSize,
        samplesSizes,
        nbSamples,
        minRep,
        notificationLevel,
    );
    if params.zParams.notificationLevel >= 3 {
        let nb = if (25) < (*dictList.offset(0)).pos {
            25
        } else {
            (*dictList.offset(0)).pos
        };
        let dictContentSize = ZDICT_dictSize(dictList);
        let mut u: core::ffi::c_uint = 0;
        if notificationLevel >= 3 {
            fprintf(
                stderr,
                b"\n %u segments found, of total size %u \n\0" as *const u8
                    as *const core::ffi::c_char,
                ((*dictList.offset(0)).pos).wrapping_sub(1),
                dictContentSize,
            );
            fflush(stderr);
        }
        if notificationLevel >= 3 {
            fprintf(
                stderr,
                b"list %u best segments \n\0" as *const u8 as *const core::ffi::c_char,
                nb.wrapping_sub(1),
            );
            fflush(stderr);
        }
        u = 1;
        while u < nb {
            let pos = (*dictList.offset(u as isize)).pos;
            let length = (*dictList.offset(u as isize)).length;
            let printedLength = if (40) < length { 40 } else { length };
            if pos as size_t > samplesBuffSize
                || pos.wrapping_add(length) as size_t > samplesBuffSize
            {
                free(dictList as *mut core::ffi::c_void);
                return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
            }
            if notificationLevel >= 3 {
                fprintf(
                    stderr,
                    b"%3u:%3u bytes at pos %8u, savings %7u bytes |\0" as *const u8
                        as *const core::ffi::c_char,
                    u,
                    length,
                    pos,
                    (*dictList.offset(u as isize)).savings,
                );
                fflush(stderr);
            }
            ZDICT_printHex(
                (samplesBuffer as *const core::ffi::c_char).offset(pos as isize)
                    as *const core::ffi::c_void,
                printedLength as size_t,
            );
            if notificationLevel >= 3 {
                fprintf(stderr, b"| \n\0" as *const u8 as *const core::ffi::c_char);
                fflush(stderr);
            }
            u = u.wrapping_add(1);
        }
    }
    let mut dictContentSize_0 = ZDICT_dictSize(dictList);
    if dictContentSize_0 < ZDICT_CONTENTSIZE_MIN as core::ffi::c_uint {
        free(dictList as *mut core::ffi::c_void);
        return -(ZSTD_error_dictionaryCreation_failed as core::ffi::c_int) as size_t;
    }
    if (dictContentSize_0 as size_t) < targetDictSize / 4 {
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  warning : selected content significantly smaller than requested (%u < %u) \n\0"
                    as *const u8 as *const core::ffi::c_char,
                dictContentSize_0,
                maxDictSize as core::ffi::c_uint,
            );
            fflush(stderr);
        }
        if samplesBuffSize < 10 * targetDictSize && notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  consider increasing the number of samples (total size : %u MB)\n\0"
                    as *const u8 as *const core::ffi::c_char,
                (samplesBuffSize >> 20) as core::ffi::c_uint,
            );
            fflush(stderr);
        }
        if minRep > MINRATIO as core::ffi::c_uint {
            if notificationLevel >= 2 {
                fprintf(
                    stderr,
                    b"!  consider increasing selectivity to produce larger dictionary (-s%u) \n\0"
                        as *const u8 as *const core::ffi::c_char,
                    selectivity.wrapping_add(1),
                );
                fflush(stderr);
            }
            if notificationLevel >= 2 {
                fprintf(
                    stderr,
                    b"!  note : larger dictionaries are not necessarily better, test its efficiency on samples \n\0"
                        as *const u8 as *const core::ffi::c_char,
                );
                fflush(stderr);
            }
        }
    }
    if dictContentSize_0 as size_t > targetDictSize * 3
        && nbSamples > (2 * MINRATIO) as core::ffi::c_uint
        && selectivity > 1
    {
        let mut proposedSelectivity = selectivity.wrapping_sub(1);
        while nbSamples >> proposedSelectivity <= MINRATIO as core::ffi::c_uint {
            proposedSelectivity = proposedSelectivity.wrapping_sub(1);
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  note : calculated dictionary significantly larger than requested (%u > %u) \n\0"
                    as *const u8 as *const core::ffi::c_char,
                dictContentSize_0,
                maxDictSize as core::ffi::c_uint,
            );
            fflush(stderr);
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  consider increasing dictionary size, or produce denser dictionary (-s%u) \n\0"
                    as *const u8 as *const core::ffi::c_char,
                proposedSelectivity,
            );
            fflush(stderr);
        }
        if notificationLevel >= 2 {
            fprintf(
                stderr,
                b"!  always test dictionary efficiency on real samples \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
    }
    let max = (*dictList).pos;
    let mut currentSize = 0u32;
    let mut n: u32 = 0;
    n = 1;
    while n < max {
        currentSize = currentSize.wrapping_add((*dictList.offset(n as isize)).length);
        if currentSize as size_t > targetDictSize {
            currentSize = currentSize.wrapping_sub((*dictList.offset(n as isize)).length);
            break;
        } else {
            n = n.wrapping_add(1);
        }
    }
    (*dictList).pos = n;
    dictContentSize_0 = currentSize;
    let mut u_0: u32 = 0;
    let mut ptr = (dictBuffer as *mut u8).offset(maxDictSize as isize);
    u_0 = 1;
    while u_0 < (*dictList).pos {
        let mut l = (*dictList.offset(u_0 as isize)).length;
        ptr = ptr.offset(-(l as isize));
        if ptr < dictBuffer as *mut u8 {
            free(dictList as *mut core::ffi::c_void);
            return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
        }
        memcpy(
            ptr as *mut core::ffi::c_void,
            (samplesBuffer as *const core::ffi::c_char)
                .offset((*dictList.offset(u_0 as isize)).pos as isize)
                as *const core::ffi::c_void,
            l as size_t,
        );
        u_0 = u_0.wrapping_add(1);
    }
    dictSize = ZDICT_addEntropyTablesFromBuffer_advanced(
        dictBuffer,
        dictContentSize_0 as size_t,
        maxDictSize,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        params.zParams,
    );
    free(dictList as *mut core::ffi::c_void);
    dictSize
}
#[export_name = crate::prefix!(ZDICT_trainFromBuffer_legacy)]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_legacy(
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
    mut params: ZDICT_legacy_params_t,
) -> size_t {
    let mut result: size_t = 0;
    let mut newBuff = core::ptr::null_mut::<core::ffi::c_void>();
    let sBuffSize = ZDICT_totalSampleSize(samplesSizes, nbSamples);
    if sBuffSize < ZDICT_MIN_SAMPLES_SIZE as size_t {
        return 0;
    }
    newBuff = malloc(sBuffSize.wrapping_add(NOISELENGTH as size_t));
    if newBuff.is_null() {
        return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    }
    memcpy(newBuff, samplesBuffer, sBuffSize);
    ZDICT_fillNoise(
        (newBuff as *mut core::ffi::c_char).offset(sBuffSize as isize) as *mut core::ffi::c_void,
        NOISELENGTH as size_t,
    );
    result = ZDICT_trainFromBuffer_unsafe_legacy(
        dictBuffer,
        dictBufferCapacity,
        newBuff,
        samplesSizes,
        nbSamples,
        params,
    );
    free(newBuff);
    result
}
#[export_name = crate::prefix!(ZDICT_trainFromBuffer)]
pub unsafe extern "C" fn ZDICT_trainFromBuffer(
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
) -> size_t {
    let mut params = ZDICT_fastCover_params_t {
        k: 0,
        d: 0,
        f: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        accel: 0,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    ptr::write_bytes(
        &mut params as *mut ZDICT_fastCover_params_t as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_fastCover_params_t>(),
    );
    params.d = 8;
    params.steps = 4;
    params.zParams.compressionLevel = ZSTD_CLEVEL_DEFAULT;
    ZDICT_optimizeTrainFromBuffer_fastCover(
        dictBuffer,
        dictBufferCapacity,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        &mut params,
    )
}
#[export_name = crate::prefix!(ZDICT_addEntropyTablesFromBuffer)]
pub unsafe extern "C" fn ZDICT_addEntropyTablesFromBuffer(
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictContentSize: size_t,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
) -> size_t {
    let mut params = ZDICT_params_t {
        compressionLevel: 0,
        notificationLevel: 0,
        dictID: 0,
    };
    ptr::write_bytes(
        &mut params as *mut ZDICT_params_t as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_params_t>(),
    );
    ZDICT_addEntropyTablesFromBuffer_advanced(
        dictBuffer,
        dictContentSize,
        dictBufferCapacity,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        params,
    )
}
