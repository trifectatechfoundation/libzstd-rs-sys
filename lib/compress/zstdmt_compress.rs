use std::ptr;

use ::c2rust_bitfields;
use libc::{
    free, pthread_cond_broadcast, pthread_cond_destroy, pthread_cond_init, pthread_cond_signal,
    pthread_cond_t, pthread_cond_wait, pthread_condattr_t, pthread_mutex_destroy,
    pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock,
    pthread_mutexattr_t,
};

use crate::lib::common::pool::{
    POOL_create_advanced, POOL_ctx, POOL_free, POOL_resize, POOL_sizeof, POOL_tryAdd,
};
use crate::lib::common::xxhash::{
    XXH64_state_t, ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_update,
};
use crate::lib::common::zstd_trace::ZSTD_TraceCtx;
use crate::lib::compress::zstd_compress::{
    rawSeq, RawSeqStore_t, SeqStore_t, ZSTD_CCtx, ZSTD_CCtxParams_setParameter, ZSTD_CCtx_params,
    ZSTD_CCtx_trace, ZSTD_CDict, ZSTD_MatchState_t, ZSTD_compressBegin_advanced_internal,
    ZSTD_compressBound, ZSTD_compressContinue_public, ZSTD_compressEnd_public,
    ZSTD_createCCtx_advanced, ZSTD_createCDict_advanced, ZSTD_cycleLog, ZSTD_freeCCtx,
    ZSTD_freeCDict, ZSTD_getCParamsFromCCtxParams, ZSTD_invalidateRepCodes, ZSTD_optimal_t,
    ZSTD_referenceExternalSequences, ZSTD_sizeof_CCtx, ZSTD_sizeof_CDict, ZSTD_window_t,
    ZSTD_writeLastEmptyBlock,
};
use crate::lib::compress::zstd_ldm::{
    ldmEntry_t, ldmParams_t, ldmState_t, ZSTD_ldm_adjustParameters, ZSTD_ldm_fillHashTable,
    ZSTD_ldm_generateSequences, ZSTD_ldm_getMaxNbSeq,
};
use crate::lib::zstd::*;
extern "C" {
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
}
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: std::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed {
    pub __low: std::ffi::c_uint,
    pub __high: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub stage: ZSTD_compressionStage_e,
    pub cParamsChanged: std::ffi::c_int,
    pub bmi2: std::ffi::c_int,
    pub requestedParams: ZSTD_CCtx_params,
    pub appliedParams: ZSTD_CCtx_params,
    pub simpleApiParams: ZSTD_CCtx_params,
    pub dictID: u32,
    pub dictContentSize: size_t,
    pub workspace: ZSTD_cwksp,
    pub blockSizeMax: size_t,
    pub pledgedSrcSizePlusOne: std::ffi::c_ulonglong,
    pub consumedSrcSize: std::ffi::c_ulonglong,
    pub producedCSize: std::ffi::c_ulonglong,
    pub xxhState: XXH64_state_t,
    pub customMem: ZSTD_customMem,
    pub pool: *mut ZSTD_threadPool,
    pub staticSize: size_t,
    pub seqCollector: SeqCollector,
    pub isFirstBlock: std::ffi::c_int,
    pub initialized: std::ffi::c_int,
    pub seqStore: SeqStore_t,
    pub ldmState: ldmState_t,
    pub ldmSequences: *mut rawSeq,
    pub maxNbLdmSequences: size_t,
    pub externSeqStore: RawSeqStore_t,
    pub blockState: ZSTD_blockState_t,
    pub tmpWorkspace: *mut std::ffi::c_void,
    pub tmpWkspSize: size_t,
    pub bufferedPolicy: ZSTD_buffered_policy_e,
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: size_t,
    pub inToCompress: size_t,
    pub inBuffPos: size_t,
    pub inBuffTarget: size_t,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: size_t,
    pub outBuffContentSize: size_t,
    pub outBuffFlushedSize: size_t,
    pub streamStage: ZSTD_cStreamStage,
    pub frameEnded: u32,
    pub expectedInBuffer: ZSTD_inBuffer,
    pub stableIn_notConsumed: size_t,
    pub expectedOutBufferSize: size_t,
    pub localDict: ZSTD_localDict,
    pub cdict: *const ZSTD_CDict,
    pub prefixDict: ZSTD_prefixDict,
    pub mtctx: *mut ZSTDMT_CCtx,
    pub traceCtx: ZSTD_TraceCtx,
    pub blockSplitCtx: ZSTD_blockSplitCtx,
    pub extSeqBuf: *mut ZSTD_Sequence,
    pub extSeqBufCapacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Sequence {
    pub offset: std::ffi::c_uint,
    pub litLength: std::ffi::c_uint,
    pub matchLength: std::ffi::c_uint,
    pub rep: std::ffi::c_uint,
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
pub type SymbolEncodingType_e = std::ffi::c_uint;
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
pub type ZSTD_longLengthType_e = std::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type ZSTDMT_CCtx = ZSTDMT_CCtx_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ZSTDMT_CCtx_s {
    pub factory: *mut POOL_ctx,
    pub jobs: *mut ZSTDMT_jobDescription,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub params: ZSTD_CCtx_params,
    pub targetSectionSize: size_t,
    pub targetPrefixSize: size_t,
    pub jobReady: std::ffi::c_int,
    pub inBuff: InBuff_t,
    pub roundBuff: RoundBuff_t,
    pub serial: SerialState,
    pub rsync: RSyncState_t,
    pub jobIDMask: std::ffi::c_uint,
    pub doneJobID: std::ffi::c_uint,
    pub nextJobID: std::ffi::c_uint,
    pub frameEnded: std::ffi::c_uint,
    pub allJobsCompleted: std::ffi::c_uint,
    pub frameContentSize: std::ffi::c_ulonglong,
    pub consumed: std::ffi::c_ulonglong,
    pub produced: std::ffi::c_ulonglong,
    pub cMem: ZSTD_customMem,
    pub cdictLocal: *mut ZSTD_CDict,
    pub cdict: *const ZSTD_CDict,
    #[bitfield(name = "providedFactory", ty = "std::ffi::c_uint", bits = "0..=0")]
    pub providedFactory: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RSyncState_t {
    pub hash: u64,
    pub hitMask: u64,
    pub primePower: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SerialState {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub params: ZSTD_CCtx_params,
    pub ldmState: ldmState_t,
    pub xxhState: XXH64_state_t,
    pub nextJobID: std::ffi::c_uint,
    pub ldmWindowMutex: pthread_mutex_t,
    pub ldmWindowCond: pthread_cond_t,
    pub ldmWindow: ZSTD_window_t,
}
pub type XXH64_hash_t = u64;
pub type XXH32_hash_t = u32;
pub type ZSTD_ParamSwitch_e = std::ffi::c_uint;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
pub type ZSTD_sequenceProducer_F = Option<
    unsafe extern "C" fn(
        *mut std::ffi::c_void,
        *mut ZSTD_Sequence,
        size_t,
        *const std::ffi::c_void,
        size_t,
        *const std::ffi::c_void,
        size_t,
        std::ffi::c_int,
        size_t,
    ) -> size_t,
>;
pub type ZSTD_SequenceFormat_e = std::ffi::c_uint;
pub const ZSTD_sf_explicitBlockDelimiters: ZSTD_SequenceFormat_e = 1;
pub const ZSTD_sf_noBlockDelimiters: ZSTD_SequenceFormat_e = 0;
pub type ZSTD_dictAttachPref_e = std::ffi::c_uint;
pub const ZSTD_dictForceLoad: ZSTD_dictAttachPref_e = 3;
pub const ZSTD_dictForceCopy: ZSTD_dictAttachPref_e = 2;
pub const ZSTD_dictForceAttach: ZSTD_dictAttachPref_e = 1;
pub const ZSTD_dictDefaultAttach: ZSTD_dictAttachPref_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: std::ffi::c_int,
    pub checksumFlag: std::ffi::c_int,
    pub noDictIDFlag: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RoundBuff_t {
    pub buffer: *mut u8,
    pub capacity: size_t,
    pub pos: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InBuff_t {
    pub prefix: Range,
    pub buffer: Buffer,
    pub filled: size_t,
}
pub type Buffer = buffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_s {
    pub start: *mut std::ffi::c_void,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub start: *const std::ffi::c_void,
    pub size: size_t,
}
pub type ZSTDMT_seqPool = ZSTDMT_bufferPool;
pub type ZSTDMT_bufferPool = ZSTDMT_bufferPool_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_bufferPool_s {
    pub poolMutex: pthread_mutex_t,
    pub bufferSize: size_t,
    pub totalBuffers: std::ffi::c_uint,
    pub nbBuffers: std::ffi::c_uint,
    pub cMem: ZSTD_customMem,
    pub buffers: *mut Buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_CCtxPool {
    pub poolMutex: pthread_mutex_t,
    pub totalCCtx: std::ffi::c_int,
    pub availCCtx: std::ffi::c_int,
    pub cMem: ZSTD_customMem,
    pub cctxs: *mut *mut ZSTD_CCtx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDMT_jobDescription {
    pub consumed: size_t,
    pub cSize: size_t,
    pub job_mutex: pthread_mutex_t,
    pub job_cond: pthread_cond_t,
    pub cctxPool: *mut ZSTDMT_CCtxPool,
    pub bufPool: *mut ZSTDMT_bufferPool,
    pub seqPool: *mut ZSTDMT_seqPool,
    pub serial: *mut SerialState,
    pub dstBuff: Buffer,
    pub prefix: Range,
    pub src: Range,
    pub jobID: std::ffi::c_uint,
    pub firstJob: std::ffi::c_uint,
    pub lastJob: std::ffi::c_uint,
    pub params: ZSTD_CCtx_params,
    pub cdict: *const ZSTD_CDict,
    pub fullFrameSize: std::ffi::c_ulonglong,
    pub dstFlushed: size_t,
    pub frameChecksumNeeded: std::ffi::c_uint,
}
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const std::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub type ZSTD_dictContentType_e = std::ffi::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut std::ffi::c_void,
    pub dict: *const std::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}
pub type ZSTD_cStreamStage = std::ffi::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub type ZSTD_buffered_policy_e = std::ffi::c_uint;
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
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [u32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: std::ffi::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: size_t,
    pub maxSequences: size_t,
}
pub type ZSTD_threadPool = POOL_ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cwksp {
    pub workspace: *mut std::ffi::c_void,
    pub workspaceEnd: *mut std::ffi::c_void,
    pub objectEnd: *mut std::ffi::c_void,
    pub tableEnd: *mut std::ffi::c_void,
    pub tableValidEnd: *mut std::ffi::c_void,
    pub allocStart: *mut std::ffi::c_void,
    pub initOnceStart: *mut std::ffi::c_void,
    pub allocFailed: u8,
    pub workspaceOversizedDuration: std::ffi::c_int,
    pub phase: ZSTD_cwksp_alloc_phase_e,
    pub isStatic: ZSTD_cwksp_static_alloc_e,
}
pub type ZSTD_cwksp_static_alloc_e = std::ffi::c_uint;
pub const ZSTD_cwksp_static_alloc: ZSTD_cwksp_static_alloc_e = 1;
pub const ZSTD_cwksp_dynamic_alloc: ZSTD_cwksp_static_alloc_e = 0;
pub type ZSTD_cwksp_alloc_phase_e = std::ffi::c_uint;
pub const ZSTD_cwksp_alloc_buffers: ZSTD_cwksp_alloc_phase_e = 3;
pub const ZSTD_cwksp_alloc_aligned: ZSTD_cwksp_alloc_phase_e = 2;
pub const ZSTD_cwksp_alloc_aligned_init_once: ZSTD_cwksp_alloc_phase_e = 1;
pub const ZSTD_cwksp_alloc_objects: ZSTD_cwksp_alloc_phase_e = 0;
pub type ZSTD_compressionStage_e = std::ffi::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub type ZSTD_cParameter = std::ffi::c_uint;
pub const ZSTD_c_experimentalParam20: ZSTD_cParameter = 1017;
pub const ZSTD_c_experimentalParam19: ZSTD_cParameter = 1016;
pub const ZSTD_c_experimentalParam18: ZSTD_cParameter = 1015;
pub const ZSTD_c_experimentalParam17: ZSTD_cParameter = 1014;
pub const ZSTD_c_experimentalParam16: ZSTD_cParameter = 1013;
pub const ZSTD_c_experimentalParam15: ZSTD_cParameter = 1012;
pub const ZSTD_c_experimentalParam14: ZSTD_cParameter = 1011;
pub const ZSTD_c_experimentalParam13: ZSTD_cParameter = 1010;
pub const ZSTD_c_experimentalParam12: ZSTD_cParameter = 1009;
pub const ZSTD_c_experimentalParam11: ZSTD_cParameter = 1008;
pub const ZSTD_c_experimentalParam10: ZSTD_cParameter = 1007;
pub const ZSTD_c_experimentalParam9: ZSTD_cParameter = 1006;
pub const ZSTD_c_experimentalParam8: ZSTD_cParameter = 1005;
pub const ZSTD_c_experimentalParam7: ZSTD_cParameter = 1004;
pub const ZSTD_c_experimentalParam5: ZSTD_cParameter = 1002;
pub const ZSTD_c_experimentalParam4: ZSTD_cParameter = 1001;
pub const ZSTD_c_experimentalParam3: ZSTD_cParameter = 1000;
pub const ZSTD_c_experimentalParam2: ZSTD_cParameter = 10;
pub const ZSTD_c_experimentalParam1: ZSTD_cParameter = 500;
pub const ZSTD_c_overlapLog: ZSTD_cParameter = 402;
pub const ZSTD_c_jobSize: ZSTD_cParameter = 401;
pub const ZSTD_c_nbWorkers: ZSTD_cParameter = 400;
pub const ZSTD_c_dictIDFlag: ZSTD_cParameter = 202;
pub const ZSTD_c_checksumFlag: ZSTD_cParameter = 201;
pub const ZSTD_c_contentSizeFlag: ZSTD_cParameter = 200;
pub const ZSTD_c_ldmHashRateLog: ZSTD_cParameter = 164;
pub const ZSTD_c_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub const ZSTD_c_ldmMinMatch: ZSTD_cParameter = 162;
pub const ZSTD_c_ldmHashLog: ZSTD_cParameter = 161;
pub const ZSTD_c_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const ZSTD_c_targetCBlockSize: ZSTD_cParameter = 130;
pub const ZSTD_c_strategy: ZSTD_cParameter = 107;
pub const ZSTD_c_targetLength: ZSTD_cParameter = 106;
pub const ZSTD_c_minMatch: ZSTD_cParameter = 105;
pub const ZSTD_c_searchLog: ZSTD_cParameter = 104;
pub const ZSTD_c_chainLog: ZSTD_cParameter = 103;
pub const ZSTD_c_hashLog: ZSTD_cParameter = 102;
pub const ZSTD_c_windowLog: ZSTD_cParameter = 101;
pub const ZSTD_c_compressionLevel: ZSTD_cParameter = 100;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_EndDirective = std::ffi::c_uint;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type ZSTD_dictLoadMethod_e = std::ffi::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub type unalign32 = u32;
pub type XXH_errorcode = std::ffi::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type ZSTD_dictTableLoadMethod_e = std::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SyncPoint {
    pub toLoad: size_t,
    pub flush: std::ffi::c_int,
}
pub type ZSTD_CParamMode_e = std::ffi::c_uint;
pub const ZSTD_cpm_unknown: ZSTD_CParamMode_e = 3;
pub const ZSTD_cpm_createCDict: ZSTD_CParamMode_e = 2;
pub const ZSTD_cpm_attachDict: ZSTD_CParamMode_e = 1;
pub const ZSTD_cpm_noAttachDict: ZSTD_CParamMode_e = 0;
pub const ZSTD_BLOCKSIZELOG_MAX: std::ffi::c_int = 17;
pub const ZSTD_BLOCKSIZE_MAX: std::ffi::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: std::ffi::c_ulonglong =
    (0 as std::ffi::c_ulonglong).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong);
pub const ZSTD_c_forceMaxWindow: std::ffi::c_int = ZSTD_c_experimentalParam3 as std::ffi::c_int;
pub const ZSTD_c_deterministicRefPrefix: std::ffi::c_int =
    ZSTD_c_experimentalParam15 as std::ffi::c_int;
pub const HASH_READ_SIZE: std::ffi::c_int = 8;
static mut kNullRawSeqStore: RawSeqStore_t = {
    RawSeqStore_t {
        seq: NULL_0 as *mut rawSeq,
        pos: 0,
        posInSequence: 0,
        size: 0,
        capacity: 0,
    }
};
pub const ZSTD_WINDOW_START_INDEX: std::ffi::c_int = 2;
static prime8bytes: u64 = 0xcf1bbcdcb7a56463 as std::ffi::c_ulonglong as u64;
unsafe extern "C" fn ZSTD_ipow(mut base: u64, mut exponent: u64) -> u64 {
    let mut power = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            power *= base;
        }
        exponent >>= 1;
        base = base * base;
    }
    power
}
pub const ZSTD_ROLL_HASH_CHAR_OFFSET: std::ffi::c_int = 10;
unsafe extern "C" fn ZSTD_rollingHash_append(
    mut hash: u64,
    mut buf: *const std::ffi::c_void,
    mut size: size_t,
) -> u64 {
    let mut istart = buf as *const u8;
    let mut pos: size_t = 0;
    pos = 0;
    while pos < size {
        hash *= prime8bytes;
        hash = hash.wrapping_add(
            (*istart.offset(pos as isize) as std::ffi::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64,
        );
        pos = pos.wrapping_add(1);
    }
    hash
}
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_compute(
    mut buf: *const std::ffi::c_void,
    mut size: size_t,
) -> u64 {
    ZSTD_rollingHash_append(0, buf, size)
}
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_primePower(mut length: u32) -> u64 {
    ZSTD_ipow(prime8bytes, length.wrapping_sub(1) as u64)
}
#[inline]
unsafe extern "C" fn ZSTD_rollingHash_rotate(
    mut hash: u64,
    mut toRemove: u8,
    mut toAdd: u8,
    mut primePower: u64,
) -> u64 {
    hash = hash.wrapping_sub(
        (toRemove as std::ffi::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64 * primePower,
    );
    hash *= prime8bytes;
    hash = hash.wrapping_add((toAdd as std::ffi::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64);
    hash
}
#[inline]
unsafe extern "C" fn ZSTD_window_clear(mut window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).offset_from((*window).base) as std::ffi::c_long as size_t;
    let end = endT as u32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
#[inline]
unsafe extern "C" fn ZSTD_window_init(mut window: *mut ZSTD_window_t) {
    ptr::write_bytes(
        window as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_window_t>(),
    );
    (*window).base = b" \0" as *const u8 as *const std::ffi::c_char as *const u8;
    (*window).dictBase = b" \0" as *const u8 as *const std::ffi::c_char as *const u8;
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize);
    (*window).nbOverflowCorrections = 0;
}
#[inline]
unsafe extern "C" fn ZSTD_window_update(
    mut window: *mut ZSTD_window_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut forceNonContiguous: std::ffi::c_int,
) -> u32 {
    let ip = src as *const u8;
    let mut contiguous = 1;
    if srcSize == 0 {
        return contiguous;
    }
    if src != (*window).nextSrc as *const std::ffi::c_void || forceNonContiguous != 0 {
        let distanceFromBase =
            ((*window).nextSrc).offset_from((*window).base) as std::ffi::c_long as size_t;
        (*window).lowLimit = (*window).dictLimit;
        (*window).dictLimit = distanceFromBase as u32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-(distanceFromBase as isize));
        if ((*window).dictLimit).wrapping_sub((*window).lowLimit) < HASH_READ_SIZE as u32 {
            (*window).lowLimit = (*window).dictLimit;
        }
        contiguous = 0;
    }
    (*window).nextSrc = ip.offset(srcSize as isize);
    if (ip.offset(srcSize as isize) > ((*window).dictBase).offset((*window).lowLimit as isize))
        as std::ffi::c_int
        & (ip < ((*window).dictBase).offset((*window).dictLimit as isize)) as std::ffi::c_int
        != 0
    {
        let highInputIdx = ip.offset(srcSize as isize).offset_from((*window).dictBase)
            as std::ffi::c_long as size_t;
        let lowLimitMax = if highInputIdx > (*window).dictLimit as size_t {
            (*window).dictLimit
        } else {
            highInputIdx as u32
        };
        (*window).lowLimit = lowLimitMax;
    }
    contiguous
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong == 4) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    1
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut std::ffi::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    in_0.swap_bytes()
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut std::ffi::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
pub const ZSTDMT_JOBSIZE_MIN: std::ffi::c_int = 512 * ((1) << 10);
#[inline]
unsafe extern "C" fn ZSTD_customMalloc(
    mut size: size_t,
    mut customMem: ZSTD_customMem,
) -> *mut std::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
    }
    malloc(size)
}
#[inline]
unsafe extern "C" fn ZSTD_customCalloc(
    mut size: size_t,
    mut customMem: ZSTD_customMem,
) -> *mut std::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        let ptr = (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
        ptr::write_bytes(ptr, 0, size as usize);
        return ptr;
    }
    calloc(1, size)
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(
    mut ptr: *mut std::ffi::c_void,
    mut customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree).unwrap_unchecked()(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn _force_has_format_string(mut format: *const std::ffi::c_char, mut args: ...) {}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const NULL: std::ffi::c_int = 0;
pub const NULL_0: std::ffi::c_int = 0;
static mut g_nullBuffer: Buffer = {
    buffer_s {
        start: NULL_0 as *mut std::ffi::c_void,
        capacity: 0,
    }
};
unsafe extern "C" fn ZSTDMT_freeBufferPool(mut bufPool: *mut ZSTDMT_bufferPool) {
    if bufPool.is_null() {
        return;
    }
    if !((*bufPool).buffers).is_null() {
        let mut u: std::ffi::c_uint = 0;
        u = 0;
        while u < (*bufPool).totalBuffers {
            ZSTD_customFree(
                (*((*bufPool).buffers).offset(u as isize)).start,
                (*bufPool).cMem,
            );
            u = u.wrapping_add(1);
        }
        ZSTD_customFree((*bufPool).buffers as *mut std::ffi::c_void, (*bufPool).cMem);
    }
    pthread_mutex_destroy(&mut (*bufPool).poolMutex);
    ZSTD_customFree(bufPool as *mut std::ffi::c_void, (*bufPool).cMem);
}
unsafe extern "C" fn ZSTDMT_createBufferPool(
    mut maxNbBuffers: std::ffi::c_uint,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_bufferPool {
    let bufPool = ZSTD_customCalloc(
        ::core::mem::size_of::<ZSTDMT_bufferPool>() as std::ffi::c_ulong,
        cMem,
    ) as *mut ZSTDMT_bufferPool;
    if bufPool.is_null() {
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    if pthread_mutex_init(
        &mut (*bufPool).poolMutex,
        std::ptr::null::<pthread_mutexattr_t>(),
    ) != 0
    {
        ZSTD_customFree(bufPool as *mut std::ffi::c_void, cMem);
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    (*bufPool).buffers = ZSTD_customCalloc(
        (maxNbBuffers as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Buffer>() as std::ffi::c_ulong),
        cMem,
    ) as *mut Buffer;
    if ((*bufPool).buffers).is_null() {
        ZSTDMT_freeBufferPool(bufPool);
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    (*bufPool).bufferSize = (64 * ((1) << 10)) as size_t;
    (*bufPool).totalBuffers = maxNbBuffers;
    (*bufPool).nbBuffers = 0;
    (*bufPool).cMem = cMem;
    bufPool
}
unsafe extern "C" fn ZSTDMT_sizeof_bufferPool(mut bufPool: *mut ZSTDMT_bufferPool) -> size_t {
    let poolSize = ::core::mem::size_of::<ZSTDMT_bufferPool>() as std::ffi::c_ulong;
    let arraySize = ((*bufPool).totalBuffers as std::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Buffer>() as std::ffi::c_ulong);
    let mut u: std::ffi::c_uint = 0;
    let mut totalBufferSize = 0 as std::ffi::c_int as size_t;
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    u = 0;
    while u < (*bufPool).totalBuffers {
        totalBufferSize =
            totalBufferSize.wrapping_add((*((*bufPool).buffers).offset(u as isize)).capacity);
        u = u.wrapping_add(1);
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
    poolSize
        .wrapping_add(arraySize)
        .wrapping_add(totalBufferSize)
}
unsafe extern "C" fn ZSTDMT_setBufferSize(bufPool: *mut ZSTDMT_bufferPool, bSize: size_t) {
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    (*bufPool).bufferSize = bSize;
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
}
unsafe extern "C" fn ZSTDMT_expandBufferPool(
    mut srcBufPool: *mut ZSTDMT_bufferPool,
    mut maxNbBuffers: std::ffi::c_uint,
) -> *mut ZSTDMT_bufferPool {
    if srcBufPool.is_null() {
        return NULL_0 as *mut ZSTDMT_bufferPool;
    }
    if (*srcBufPool).totalBuffers >= maxNbBuffers {
        return srcBufPool;
    }
    let cMem = (*srcBufPool).cMem;
    let bSize = (*srcBufPool).bufferSize;
    let mut newBufPool = std::ptr::null_mut::<ZSTDMT_bufferPool>();
    ZSTDMT_freeBufferPool(srcBufPool);
    newBufPool = ZSTDMT_createBufferPool(maxNbBuffers, cMem);
    if newBufPool.is_null() {
        return newBufPool;
    }
    ZSTDMT_setBufferSize(newBufPool, bSize);
    newBufPool
}
unsafe extern "C" fn ZSTDMT_getBuffer(mut bufPool: *mut ZSTDMT_bufferPool) -> Buffer {
    let bSize = (*bufPool).bufferSize;
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    if (*bufPool).nbBuffers != 0 {
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_sub(1);
        let buf = *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize);
        let availBufferSize = buf.capacity;
        *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize) = g_nullBuffer;
        if (availBufferSize >= bSize) as std::ffi::c_int
            & (availBufferSize >> 3 <= bSize) as std::ffi::c_int
            != 0
        {
            pthread_mutex_unlock(&mut (*bufPool).poolMutex);
            return buf;
        }
        ZSTD_customFree(buf.start, (*bufPool).cMem);
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
    let mut buffer = buffer_s {
        start: std::ptr::null_mut::<std::ffi::c_void>(),
        capacity: 0,
    };
    let start = ZSTD_customMalloc(bSize, (*bufPool).cMem);
    buffer.start = start;
    buffer.capacity = if start.is_null() { 0 } else { bSize };
    start.is_null();
    buffer
}
unsafe extern "C" fn ZSTDMT_releaseBuffer(mut bufPool: *mut ZSTDMT_bufferPool, mut buf: Buffer) {
    if (buf.start).is_null() {
        return;
    }
    pthread_mutex_lock(&mut (*bufPool).poolMutex);
    if (*bufPool).nbBuffers < (*bufPool).totalBuffers {
        let fresh0 = (*bufPool).nbBuffers;
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_add(1);
        *((*bufPool).buffers).offset(fresh0 as isize) = buf;
        pthread_mutex_unlock(&mut (*bufPool).poolMutex);
        return;
    }
    pthread_mutex_unlock(&mut (*bufPool).poolMutex);
    ZSTD_customFree(buf.start, (*bufPool).cMem);
}
unsafe extern "C" fn ZSTDMT_sizeof_seqPool(mut seqPool: *mut ZSTDMT_seqPool) -> size_t {
    ZSTDMT_sizeof_bufferPool(seqPool)
}
unsafe extern "C" fn bufferToSeq(mut buffer: Buffer) -> RawSeqStore_t {
    let mut seq = kNullRawSeqStore;
    seq.seq = buffer.start as *mut rawSeq;
    seq.capacity =
        (buffer.capacity).wrapping_div(::core::mem::size_of::<rawSeq>() as std::ffi::c_ulong);
    seq
}
unsafe extern "C" fn seqToBuffer(mut seq: RawSeqStore_t) -> Buffer {
    let mut buffer = buffer_s {
        start: std::ptr::null_mut::<std::ffi::c_void>(),
        capacity: 0,
    };
    buffer.start = seq.seq as *mut std::ffi::c_void;
    buffer.capacity =
        (seq.capacity).wrapping_mul(::core::mem::size_of::<rawSeq>() as std::ffi::c_ulong);
    buffer
}
unsafe extern "C" fn ZSTDMT_getSeq(mut seqPool: *mut ZSTDMT_seqPool) -> RawSeqStore_t {
    if (*seqPool).bufferSize == 0 {
        return kNullRawSeqStore;
    }
    bufferToSeq(ZSTDMT_getBuffer(seqPool))
}
unsafe extern "C" fn ZSTDMT_releaseSeq(mut seqPool: *mut ZSTDMT_seqPool, mut seq: RawSeqStore_t) {
    ZSTDMT_releaseBuffer(seqPool, seqToBuffer(seq));
}
unsafe extern "C" fn ZSTDMT_setNbSeq(seqPool: *mut ZSTDMT_seqPool, nbSeq: size_t) {
    ZSTDMT_setBufferSize(
        seqPool,
        nbSeq.wrapping_mul(::core::mem::size_of::<rawSeq>() as std::ffi::c_ulong),
    );
}
unsafe extern "C" fn ZSTDMT_createSeqPool(
    mut nbWorkers: std::ffi::c_uint,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_seqPool {
    let seqPool = ZSTDMT_createBufferPool(nbWorkers, cMem);
    if seqPool.is_null() {
        return NULL_0 as *mut ZSTDMT_seqPool;
    }
    ZSTDMT_setNbSeq(seqPool, 0);
    seqPool
}
unsafe extern "C" fn ZSTDMT_freeSeqPool(mut seqPool: *mut ZSTDMT_seqPool) {
    ZSTDMT_freeBufferPool(seqPool);
}
unsafe extern "C" fn ZSTDMT_expandSeqPool(
    mut pool: *mut ZSTDMT_seqPool,
    mut nbWorkers: u32,
) -> *mut ZSTDMT_seqPool {
    ZSTDMT_expandBufferPool(pool, nbWorkers)
}
unsafe extern "C" fn ZSTDMT_freeCCtxPool(mut pool: *mut ZSTDMT_CCtxPool) {
    if pool.is_null() {
        return;
    }
    pthread_mutex_destroy(&mut (*pool).poolMutex);
    if !((*pool).cctxs).is_null() {
        let mut cid: std::ffi::c_int = 0;
        cid = 0;
        while cid < (*pool).totalCCtx {
            ZSTD_freeCCtx(*((*pool).cctxs).offset(cid as isize));
            cid += 1;
        }
        ZSTD_customFree((*pool).cctxs as *mut std::ffi::c_void, (*pool).cMem);
    }
    ZSTD_customFree(pool as *mut std::ffi::c_void, (*pool).cMem);
}
unsafe extern "C" fn ZSTDMT_createCCtxPool(
    mut nbWorkers: std::ffi::c_int,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_CCtxPool {
    let cctxPool = ZSTD_customCalloc(
        ::core::mem::size_of::<ZSTDMT_CCtxPool>() as std::ffi::c_ulong,
        cMem,
    ) as *mut ZSTDMT_CCtxPool;
    if cctxPool.is_null() {
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    if pthread_mutex_init(
        &mut (*cctxPool).poolMutex,
        std::ptr::null::<pthread_mutexattr_t>(),
    ) != 0
    {
        ZSTD_customFree(cctxPool as *mut std::ffi::c_void, cMem);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    (*cctxPool).totalCCtx = nbWorkers;
    (*cctxPool).cctxs = ZSTD_customCalloc(
        (nbWorkers as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>() as std::ffi::c_ulong),
        cMem,
    ) as *mut *mut ZSTD_CCtx;
    if ((*cctxPool).cctxs).is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    (*cctxPool).cMem = cMem;
    let fresh1 = &mut (*((*cctxPool).cctxs).offset(0));
    *fresh1 = ZSTD_createCCtx_advanced(cMem);
    if (*((*cctxPool).cctxs).offset(0)).is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    (*cctxPool).availCCtx = 1;
    cctxPool
}
unsafe extern "C" fn ZSTDMT_expandCCtxPool(
    mut srcPool: *mut ZSTDMT_CCtxPool,
    mut nbWorkers: std::ffi::c_int,
) -> *mut ZSTDMT_CCtxPool {
    if srcPool.is_null() {
        return NULL_0 as *mut ZSTDMT_CCtxPool;
    }
    if nbWorkers <= (*srcPool).totalCCtx {
        return srcPool;
    }
    let cMem = (*srcPool).cMem;
    ZSTDMT_freeCCtxPool(srcPool);
    ZSTDMT_createCCtxPool(nbWorkers, cMem)
}
unsafe extern "C" fn ZSTDMT_sizeof_CCtxPool(mut cctxPool: *mut ZSTDMT_CCtxPool) -> size_t {
    pthread_mutex_lock(&mut (*cctxPool).poolMutex);
    let nbWorkers = (*cctxPool).totalCCtx as std::ffi::c_uint;
    let poolSize = ::core::mem::size_of::<ZSTDMT_CCtxPool>() as std::ffi::c_ulong;
    let arraySize = ((*cctxPool).totalCCtx as std::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>() as std::ffi::c_ulong);
    let mut totalCCtxSize = 0 as std::ffi::c_int as size_t;
    let mut u: std::ffi::c_uint = 0;
    u = 0;
    while u < nbWorkers {
        totalCCtxSize =
            totalCCtxSize.wrapping_add(ZSTD_sizeof_CCtx(*((*cctxPool).cctxs).offset(u as isize)));
        u = u.wrapping_add(1);
    }
    pthread_mutex_unlock(&mut (*cctxPool).poolMutex);
    poolSize.wrapping_add(arraySize).wrapping_add(totalCCtxSize)
}
unsafe extern "C" fn ZSTDMT_getCCtx(mut cctxPool: *mut ZSTDMT_CCtxPool) -> *mut ZSTD_CCtx {
    pthread_mutex_lock(&mut (*cctxPool).poolMutex);
    if (*cctxPool).availCCtx != 0 {
        (*cctxPool).availCCtx -= 1;
        (*cctxPool).availCCtx;
        let cctx = *((*cctxPool).cctxs).offset((*cctxPool).availCCtx as isize);
        pthread_mutex_unlock(&mut (*cctxPool).poolMutex);
        return cctx;
    }
    pthread_mutex_unlock(&mut (*cctxPool).poolMutex);
    ZSTD_createCCtx_advanced((*cctxPool).cMem)
}
unsafe extern "C" fn ZSTDMT_releaseCCtx(mut pool: *mut ZSTDMT_CCtxPool, mut cctx: *mut ZSTD_CCtx) {
    if cctx.is_null() {
        return;
    }
    pthread_mutex_lock(&mut (*pool).poolMutex);
    if (*pool).availCCtx < (*pool).totalCCtx {
        let fresh2 = (*pool).availCCtx;
        (*pool).availCCtx += 1;
        let fresh3 = &mut (*((*pool).cctxs).offset(fresh2 as isize));
        *fresh3 = cctx;
    } else {
        ZSTD_freeCCtx(cctx);
    }
    pthread_mutex_unlock(&mut (*pool).poolMutex);
}
unsafe extern "C" fn ZSTDMT_serialState_reset(
    mut serialState: *mut SerialState,
    mut seqPool: *mut ZSTDMT_seqPool,
    mut params: ZSTD_CCtx_params,
    mut jobSize: size_t,
    mut dict: *const std::ffi::c_void,
    dictSize: size_t,
    mut dictContentType: ZSTD_dictContentType_e,
) -> std::ffi::c_int {
    if params.ldmParams.enableLdm as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams, &mut params.cParams);
    } else {
        ptr::write_bytes(
            &mut params.ldmParams as *mut ldmParams_t as *mut u8,
            0,
            ::core::mem::size_of::<ldmParams_t>(),
        );
    }
    (*serialState).nextJobID = 0;
    if params.fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(&mut (*serialState).xxhState, 0);
    }
    if params.ldmParams.enableLdm as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        let mut cMem = params.customMem;
        let hashLog = params.ldmParams.hashLog;
        let hashSize = ((1 as std::ffi::c_int as size_t) << hashLog)
            .wrapping_mul(::core::mem::size_of::<ldmEntry_t>() as std::ffi::c_ulong);
        let bucketLog = (params.ldmParams.hashLog).wrapping_sub(params.ldmParams.bucketSizeLog);
        let prevBucketLog = ((*serialState).params.ldmParams.hashLog)
            .wrapping_sub((*serialState).params.ldmParams.bucketSizeLog);
        let numBuckets = (1) << bucketLog;
        ZSTDMT_setNbSeq(seqPool, ZSTD_ldm_getMaxNbSeq(params.ldmParams, jobSize));
        ZSTD_window_init(&mut (*serialState).ldmState.window);
        if ((*serialState).ldmState.hashTable).is_null()
            || (*serialState).params.ldmParams.hashLog < hashLog
        {
            ZSTD_customFree(
                (*serialState).ldmState.hashTable as *mut std::ffi::c_void,
                cMem,
            );
            (*serialState).ldmState.hashTable =
                ZSTD_customMalloc(hashSize, cMem) as *mut ldmEntry_t;
        }
        if ((*serialState).ldmState.bucketOffsets).is_null() || prevBucketLog < bucketLog {
            ZSTD_customFree(
                (*serialState).ldmState.bucketOffsets as *mut std::ffi::c_void,
                cMem,
            );
            (*serialState).ldmState.bucketOffsets = ZSTD_customMalloc(numBuckets, cMem) as *mut u8;
        }
        if ((*serialState).ldmState.hashTable).is_null()
            || ((*serialState).ldmState.bucketOffsets).is_null()
        {
            return 1;
        }
        ptr::write_bytes(
            (*serialState).ldmState.hashTable as *mut u8,
            0,
            hashSize as usize,
        );
        ptr::write_bytes(
            (*serialState).ldmState.bucketOffsets as *mut u8,
            0,
            numBuckets as usize,
        );
        (*serialState).ldmState.loadedDictEnd = 0;
        if dictSize > 0
            && dictContentType as std::ffi::c_uint
                == ZSTD_dct_rawContent as std::ffi::c_int as std::ffi::c_uint
        {
            let dictEnd = (dict as *const u8).offset(dictSize as isize);
            ZSTD_window_update(&mut (*serialState).ldmState.window, dict, dictSize, 0);
            ZSTD_ldm_fillHashTable(
                &mut (*serialState).ldmState,
                dict as *const u8,
                dictEnd,
                &mut params.ldmParams,
            );
            (*serialState).ldmState.loadedDictEnd = if params.forceWindow != 0 {
                0
            } else {
                dictEnd.offset_from((*serialState).ldmState.window.base) as std::ffi::c_long as u32
            };
        }
        (*serialState).ldmWindow = (*serialState).ldmState.window;
    }
    (*serialState).params = params;
    (*serialState).params.jobSize = jobSize as u32 as size_t;
    0
}
unsafe extern "C" fn ZSTDMT_serialState_init(mut serialState: *mut SerialState) -> std::ffi::c_int {
    let mut initError = 0;
    ptr::write_bytes(
        serialState as *mut u8,
        0,
        ::core::mem::size_of::<SerialState>(),
    );
    initError |= pthread_mutex_init(
        &mut (*serialState).mutex,
        std::ptr::null::<pthread_mutexattr_t>(),
    );
    initError |= pthread_cond_init(
        &mut (*serialState).cond,
        std::ptr::null::<pthread_condattr_t>(),
    );
    initError |= pthread_mutex_init(
        &mut (*serialState).ldmWindowMutex,
        std::ptr::null::<pthread_mutexattr_t>(),
    );
    initError |= pthread_cond_init(
        &mut (*serialState).ldmWindowCond,
        std::ptr::null::<pthread_condattr_t>(),
    );
    initError
}
unsafe extern "C" fn ZSTDMT_serialState_free(mut serialState: *mut SerialState) {
    let mut cMem = (*serialState).params.customMem;
    pthread_mutex_destroy(&mut (*serialState).mutex);
    pthread_cond_destroy(&mut (*serialState).cond);
    pthread_mutex_destroy(&mut (*serialState).ldmWindowMutex);
    pthread_cond_destroy(&mut (*serialState).ldmWindowCond);
    ZSTD_customFree(
        (*serialState).ldmState.hashTable as *mut std::ffi::c_void,
        cMem,
    );
    ZSTD_customFree(
        (*serialState).ldmState.bucketOffsets as *mut std::ffi::c_void,
        cMem,
    );
}
unsafe extern "C" fn ZSTDMT_serialState_genSequences(
    mut serialState: *mut SerialState,
    mut seqStore: *mut RawSeqStore_t,
    mut src: Range,
    mut jobID: std::ffi::c_uint,
) {
    pthread_mutex_lock(&mut (*serialState).mutex);
    while (*serialState).nextJobID < jobID {
        pthread_cond_wait(&mut (*serialState).cond, &mut (*serialState).mutex);
    }
    if (*serialState).nextJobID == jobID {
        if (*serialState).params.ldmParams.enableLdm as std::ffi::c_uint
            == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
        {
            let mut error: size_t = 0;
            ZSTD_window_update(&mut (*serialState).ldmState.window, src.start, src.size, 0);
            error = ZSTD_ldm_generateSequences(
                &mut (*serialState).ldmState,
                seqStore,
                &mut (*serialState).params.ldmParams,
                src.start,
                src.size,
            );
            pthread_mutex_lock(&mut (*serialState).ldmWindowMutex);
            (*serialState).ldmWindow = (*serialState).ldmState.window;
            pthread_cond_signal(&mut (*serialState).ldmWindowCond);
            pthread_mutex_unlock(&mut (*serialState).ldmWindowMutex);
        }
        if (*serialState).params.fParams.checksumFlag != 0 && src.size > 0 {
            ZSTD_XXH64_update(&mut (*serialState).xxhState, src.start, src.size as usize);
        }
    }
    (*serialState).nextJobID = ((*serialState).nextJobID).wrapping_add(1);
    (*serialState).nextJobID;
    pthread_cond_broadcast(&mut (*serialState).cond);
    pthread_mutex_unlock(&mut (*serialState).mutex);
}
unsafe extern "C" fn ZSTDMT_serialState_applySequences(
    mut serialState: *const SerialState,
    mut jobCCtx: *mut ZSTD_CCtx,
    mut seqStore: *const RawSeqStore_t,
) {
    if (*seqStore).size > 0 {
        ZSTD_referenceExternalSequences(jobCCtx, (*seqStore).seq, (*seqStore).size);
    }
}
unsafe extern "C" fn ZSTDMT_serialState_ensureFinished(
    mut serialState: *mut SerialState,
    mut jobID: std::ffi::c_uint,
    mut cSize: size_t,
) {
    pthread_mutex_lock(&mut (*serialState).mutex);
    if (*serialState).nextJobID <= jobID {
        (*serialState).nextJobID = jobID.wrapping_add(1);
        pthread_cond_broadcast(&mut (*serialState).cond);
        pthread_mutex_lock(&mut (*serialState).ldmWindowMutex);
        ZSTD_window_clear(&mut (*serialState).ldmWindow);
        pthread_cond_signal(&mut (*serialState).ldmWindowCond);
        pthread_mutex_unlock(&mut (*serialState).ldmWindowMutex);
    }
    pthread_mutex_unlock(&mut (*serialState).mutex);
}
static mut kNullRange: Range = {
    Range {
        start: NULL_0 as *const std::ffi::c_void,
        size: 0,
    }
};
unsafe extern "C" fn ZSTDMT_compressionJob(mut jobDescription: *mut std::ffi::c_void) {
    let mut current_block: u64;
    let job = jobDescription as *mut ZSTDMT_jobDescription;
    let mut jobParams = (*job).params;
    let cctx = ZSTDMT_getCCtx((*job).cctxPool);
    let mut rawSeqStore = ZSTDMT_getSeq((*job).seqPool);
    let mut dstBuff = (*job).dstBuff;
    let mut lastCBlockSize = 0;
    if cctx.is_null() {
        pthread_mutex_lock(&mut (*job).job_mutex);
        (*job).cSize = -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        pthread_mutex_unlock(&mut (*job).job_mutex);
    } else {
        if (dstBuff.start).is_null() {
            dstBuff = ZSTDMT_getBuffer((*job).bufPool);
            if (dstBuff.start).is_null() {
                pthread_mutex_lock(&mut (*job).job_mutex);
                (*job).cSize = -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                pthread_mutex_unlock(&mut (*job).job_mutex);
                current_block = 17100290475540901977;
            } else {
                (*job).dstBuff = dstBuff;
                current_block = 7976072742316086414;
            }
        } else {
            current_block = 7976072742316086414;
        }
        match current_block {
            17100290475540901977 => {}
            _ => {
                if jobParams.ldmParams.enableLdm as std::ffi::c_uint
                    == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
                    && (rawSeqStore.seq).is_null()
                {
                    pthread_mutex_lock(&mut (*job).job_mutex);
                    (*job).cSize = -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                    pthread_mutex_unlock(&mut (*job).job_mutex);
                } else {
                    if (*job).jobID != 0 {
                        jobParams.fParams.checksumFlag = 0;
                    }
                    jobParams.ldmParams.enableLdm = ZSTD_ps_disable;
                    jobParams.nbWorkers = 0;
                    ZSTDMT_serialState_genSequences(
                        (*job).serial,
                        &mut rawSeqStore,
                        (*job).src,
                        (*job).jobID,
                    );
                    if !((*job).cdict).is_null() {
                        let initError = ZSTD_compressBegin_advanced_internal(
                            cctx,
                            NULL_0 as *const std::ffi::c_void,
                            0,
                            ZSTD_dct_auto,
                            ZSTD_dtlm_fast,
                            (*job).cdict,
                            &mut jobParams,
                            (*job).fullFrameSize,
                        );
                        if ERR_isError(initError) != 0 {
                            pthread_mutex_lock(&mut (*job).job_mutex);
                            (*job).cSize = initError;
                            pthread_mutex_unlock(&mut (*job).job_mutex);
                            current_block = 17100290475540901977;
                        } else {
                            current_block = 16738040538446813684;
                        }
                    } else {
                        let pledgedSrcSize = (if (*job).firstJob != 0 {
                            (*job).fullFrameSize
                        } else {
                            (*job).src.size as std::ffi::c_ulonglong
                        }) as u64;
                        let forceWindowError = ZSTD_CCtxParams_setParameter(
                            &mut jobParams,
                            ZSTD_c_forceMaxWindow as ZSTD_cParameter,
                            ((*job).firstJob == 0) as std::ffi::c_int,
                        );
                        if ERR_isError(forceWindowError) != 0 {
                            pthread_mutex_lock(&mut (*job).job_mutex);
                            (*job).cSize = forceWindowError;
                            pthread_mutex_unlock(&mut (*job).job_mutex);
                            current_block = 17100290475540901977;
                        } else {
                            if (*job).firstJob == 0 {
                                let err = ZSTD_CCtxParams_setParameter(
                                    &mut jobParams,
                                    ZSTD_c_deterministicRefPrefix as ZSTD_cParameter,
                                    0,
                                );
                                if ERR_isError(err) != 0 {
                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                    (*job).cSize = err;
                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                    current_block = 17100290475540901977;
                                } else {
                                    current_block = 2543120759711851213;
                                }
                            } else {
                                current_block = 2543120759711851213;
                            }
                            match current_block {
                                17100290475540901977 => {}
                                _ => {
                                    let initError_0 = ZSTD_compressBegin_advanced_internal(
                                        cctx,
                                        (*job).prefix.start,
                                        (*job).prefix.size,
                                        ZSTD_dct_rawContent,
                                        ZSTD_dtlm_fast,
                                        NULL_0 as *const ZSTD_CDict,
                                        &mut jobParams,
                                        pledgedSrcSize as std::ffi::c_ulonglong,
                                    );
                                    if ERR_isError(initError_0) != 0 {
                                        pthread_mutex_lock(&mut (*job).job_mutex);
                                        (*job).cSize = initError_0;
                                        pthread_mutex_unlock(&mut (*job).job_mutex);
                                        current_block = 17100290475540901977;
                                    } else {
                                        current_block = 16738040538446813684;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        17100290475540901977 => {}
                        _ => {
                            ZSTDMT_serialState_applySequences(
                                (*job).serial,
                                cctx,
                                &mut rawSeqStore,
                            );
                            if (*job).firstJob == 0 {
                                let hSize = ZSTD_compressContinue_public(
                                    cctx,
                                    dstBuff.start,
                                    dstBuff.capacity,
                                    (*job).src.start,
                                    0,
                                );
                                if ERR_isError(hSize) != 0 {
                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                    (*job).cSize = hSize;
                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                    current_block = 17100290475540901977;
                                } else {
                                    ZSTD_invalidateRepCodes(cctx);
                                    current_block = 6560072651652764009;
                                }
                            } else {
                                current_block = 6560072651652764009;
                            }
                            match current_block {
                                17100290475540901977 => {}
                                _ => {
                                    let chunkSize = (4 * ZSTD_BLOCKSIZE_MAX) as size_t;
                                    let nbChunks = (((*job).src.size)
                                        .wrapping_add(chunkSize.wrapping_sub(1))
                                        / chunkSize)
                                        as std::ffi::c_int;
                                    let mut ip = (*job).src.start as *const u8;
                                    let ostart = dstBuff.start as *mut u8;
                                    let mut op = ostart;
                                    let mut oend = op.offset(dstBuff.capacity as isize);
                                    let mut chunkNb: std::ffi::c_int = 0;
                                    ::core::mem::size_of::<size_t>();
                                    ::core::mem::size_of::<std::ffi::c_int>();
                                    chunkNb = 1;
                                    loop {
                                        if chunkNb >= nbChunks {
                                            current_block = 851619935621435220;
                                            break;
                                        }
                                        let cSize = ZSTD_compressContinue_public(
                                            cctx,
                                            op as *mut std::ffi::c_void,
                                            oend.offset_from(op) as std::ffi::c_long as size_t,
                                            ip as *const std::ffi::c_void,
                                            chunkSize,
                                        );
                                        if ERR_isError(cSize) != 0 {
                                            pthread_mutex_lock(&mut (*job).job_mutex);
                                            (*job).cSize = cSize;
                                            pthread_mutex_unlock(&mut (*job).job_mutex);
                                            current_block = 17100290475540901977;
                                            break;
                                        } else {
                                            ip = ip.offset(chunkSize as isize);
                                            op = op.offset(cSize as isize);
                                            pthread_mutex_lock(&mut (*job).job_mutex);
                                            (*job).cSize = ((*job).cSize).wrapping_add(cSize);
                                            (*job).consumed = chunkSize * chunkNb as size_t;
                                            pthread_cond_signal(&mut (*job).job_cond);
                                            pthread_mutex_unlock(&mut (*job).job_mutex);
                                            chunkNb += 1;
                                        }
                                    }
                                    match current_block {
                                        17100290475540901977 => {}
                                        _ => {
                                            if (nbChunks > 0) as std::ffi::c_int as std::ffi::c_uint
                                                | (*job).lastJob
                                                != 0
                                            {
                                                let lastBlockSize1 =
                                                    (*job).src.size & chunkSize.wrapping_sub(1);
                                                let lastBlockSize = if (lastBlockSize1 == 0)
                                                    as std::ffi::c_int
                                                    & ((*job).src.size >= chunkSize)
                                                        as std::ffi::c_int
                                                    != 0
                                                {
                                                    chunkSize
                                                } else {
                                                    lastBlockSize1
                                                };
                                                let cSize_0 = if (*job).lastJob != 0 {
                                                    ZSTD_compressEnd_public(
                                                        cctx,
                                                        op as *mut std::ffi::c_void,
                                                        oend.offset_from(op) as std::ffi::c_long
                                                            as size_t,
                                                        ip as *const std::ffi::c_void,
                                                        lastBlockSize,
                                                    )
                                                } else {
                                                    ZSTD_compressContinue_public(
                                                        cctx,
                                                        op as *mut std::ffi::c_void,
                                                        oend.offset_from(op) as std::ffi::c_long
                                                            as size_t,
                                                        ip as *const std::ffi::c_void,
                                                        lastBlockSize,
                                                    )
                                                };
                                                if ERR_isError(cSize_0) != 0 {
                                                    pthread_mutex_lock(&mut (*job).job_mutex);
                                                    (*job).cSize = cSize_0;
                                                    pthread_mutex_unlock(&mut (*job).job_mutex);
                                                    current_block = 17100290475540901977;
                                                } else {
                                                    lastCBlockSize = cSize_0;
                                                    current_block = 200744462051969938;
                                                }
                                            } else {
                                                current_block = 200744462051969938;
                                            }
                                            match current_block {
                                                17100290475540901977 => {}
                                                _ => {
                                                    (*job).firstJob == 0;
                                                    ZSTD_CCtx_trace(cctx, 0);
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
    }
    ZSTDMT_serialState_ensureFinished((*job).serial, (*job).jobID, (*job).cSize);
    (*job).prefix.size > 0;
    ZSTDMT_releaseSeq((*job).seqPool, rawSeqStore);
    ZSTDMT_releaseCCtx((*job).cctxPool, cctx);
    pthread_mutex_lock(&mut (*job).job_mutex);
    ERR_isError((*job).cSize);
    0;
    (*job).cSize = ((*job).cSize).wrapping_add(lastCBlockSize);
    (*job).consumed = (*job).src.size;
    pthread_cond_signal(&mut (*job).job_cond);
    pthread_mutex_unlock(&mut (*job).job_mutex);
}
static mut kNullRoundBuff: RoundBuff_t = {
    RoundBuff_t {
        buffer: NULL_0 as *mut u8,
        capacity: 0,
        pos: 0,
    }
};
pub const RSYNC_LENGTH: std::ffi::c_int = 32;
pub const RSYNC_MIN_BLOCK_LOG: std::ffi::c_int = ZSTD_BLOCKSIZELOG_MAX;
pub const RSYNC_MIN_BLOCK_SIZE: std::ffi::c_int = (1) << RSYNC_MIN_BLOCK_LOG;
unsafe extern "C" fn ZSTDMT_freeJobsTable(
    mut jobTable: *mut ZSTDMT_jobDescription,
    mut nbJobs: u32,
    mut cMem: ZSTD_customMem,
) {
    let mut jobNb: u32 = 0;
    if jobTable.is_null() {
        return;
    }
    jobNb = 0;
    while jobNb < nbJobs {
        pthread_mutex_destroy(&mut (*jobTable.offset(jobNb as isize)).job_mutex);
        pthread_cond_destroy(&mut (*jobTable.offset(jobNb as isize)).job_cond);
        jobNb = jobNb.wrapping_add(1);
    }
    ZSTD_customFree(jobTable as *mut std::ffi::c_void, cMem);
}
unsafe extern "C" fn ZSTDMT_createJobsTable(
    mut nbJobsPtr: *mut u32,
    mut cMem: ZSTD_customMem,
) -> *mut ZSTDMT_jobDescription {
    let nbJobsLog2 = (ZSTD_highbit32(*nbJobsPtr)).wrapping_add(1);
    let nbJobs = ((1) << nbJobsLog2) as u32;
    let mut jobNb: u32 = 0;
    let jobTable = ZSTD_customCalloc(
        (nbJobs as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ZSTDMT_jobDescription>() as std::ffi::c_ulong),
        cMem,
    ) as *mut ZSTDMT_jobDescription;
    let mut initError = 0;
    if jobTable.is_null() {
        return NULL_0 as *mut ZSTDMT_jobDescription;
    }
    *nbJobsPtr = nbJobs;
    jobNb = 0;
    while jobNb < nbJobs {
        initError |= pthread_mutex_init(
            &mut (*jobTable.offset(jobNb as isize)).job_mutex,
            std::ptr::null::<pthread_mutexattr_t>(),
        );
        initError |= pthread_cond_init(
            &mut (*jobTable.offset(jobNb as isize)).job_cond,
            std::ptr::null::<pthread_condattr_t>(),
        );
        jobNb = jobNb.wrapping_add(1);
    }
    if initError != 0 {
        ZSTDMT_freeJobsTable(jobTable, nbJobs, cMem);
        return NULL_0 as *mut ZSTDMT_jobDescription;
    }
    jobTable
}
unsafe extern "C" fn ZSTDMT_expandJobsTable(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut nbWorkers: u32,
) -> size_t {
    let mut nbJobs = nbWorkers.wrapping_add(2);
    if nbJobs > ((*mtctx).jobIDMask).wrapping_add(1) {
        ZSTDMT_freeJobsTable(
            (*mtctx).jobs,
            ((*mtctx).jobIDMask).wrapping_add(1),
            (*mtctx).cMem,
        );
        (*mtctx).jobIDMask = 0;
        (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, (*mtctx).cMem);
        if ((*mtctx).jobs).is_null() {
            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        }
        (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    }
    0
}
unsafe extern "C" fn ZSTDMT_CCtxParam_setNbWorkers(
    mut params: *mut ZSTD_CCtx_params,
    mut nbWorkers: std::ffi::c_uint,
) -> size_t {
    ZSTD_CCtxParams_setParameter(params, ZSTD_c_nbWorkers, nbWorkers as std::ffi::c_int)
}
#[inline]
unsafe extern "C" fn ZSTDMT_createCCtx_advanced_internal(
    mut nbWorkers: std::ffi::c_uint,
    mut cMem: ZSTD_customMem,
    mut pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    let mut mtctx = std::ptr::null_mut::<ZSTDMT_CCtx>();
    let mut nbJobs = nbWorkers.wrapping_add(2);
    let mut initError: std::ffi::c_int = 0;
    if nbWorkers < 1 {
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    nbWorkers = if nbWorkers
        < (if ::core::mem::size_of::<*mut std::ffi::c_void>() as std::ffi::c_ulong == 4 {
            64
        } else {
            256
        }) as std::ffi::c_uint
    {
        nbWorkers
    } else {
        (if ::core::mem::size_of::<*mut std::ffi::c_void>() as std::ffi::c_ulong == 4 {
            64
        } else {
            256
        }) as std::ffi::c_uint
    };
    if (cMem.customAlloc).is_some() as std::ffi::c_int
        ^ (cMem.customFree).is_some() as std::ffi::c_int
        != 0
    {
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    mtctx = ZSTD_customCalloc(
        ::core::mem::size_of::<ZSTDMT_CCtx>() as std::ffi::c_ulong,
        cMem,
    ) as *mut ZSTDMT_CCtx;
    if mtctx.is_null() {
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    (*mtctx).cMem = cMem;
    (*mtctx).allJobsCompleted = 1;
    if !pool.is_null() {
        (*mtctx).factory = pool;
        (*mtctx).set_providedFactory(1);
    } else {
        (*mtctx).factory = POOL_create_advanced(nbWorkers as size_t, 0, cMem);
        (*mtctx).set_providedFactory(0);
    }
    (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, cMem);
    (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    (*mtctx).bufPool = ZSTDMT_createBufferPool(
        (2 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_mul(nbWorkers)
            .wrapping_add(3),
        cMem,
    );
    (*mtctx).cctxPool = ZSTDMT_createCCtxPool(nbWorkers as std::ffi::c_int, cMem);
    (*mtctx).seqPool = ZSTDMT_createSeqPool(nbWorkers, cMem);
    initError = ZSTDMT_serialState_init(&mut (*mtctx).serial);
    (*mtctx).roundBuff = kNullRoundBuff;
    if ((*mtctx).factory).is_null() as std::ffi::c_int
        | ((*mtctx).jobs).is_null() as std::ffi::c_int
        | ((*mtctx).bufPool).is_null() as std::ffi::c_int
        | ((*mtctx).cctxPool).is_null() as std::ffi::c_int
        | ((*mtctx).seqPool).is_null() as std::ffi::c_int
        | initError
        != 0
    {
        ZSTDMT_freeCCtx(mtctx);
        return NULL_0 as *mut ZSTDMT_CCtx;
    }
    mtctx
}
#[export_name = crate::prefix!(ZSTDMT_createCCtx_advanced)]
pub unsafe extern "C" fn ZSTDMT_createCCtx_advanced(
    mut nbWorkers: std::ffi::c_uint,
    mut cMem: ZSTD_customMem,
    mut pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    ZSTDMT_createCCtx_advanced_internal(nbWorkers, cMem, pool)
}
unsafe extern "C" fn ZSTDMT_releaseAllJobResources(mut mtctx: *mut ZSTDMT_CCtx) {
    let mut jobID: std::ffi::c_uint = 0;
    jobID = 0;
    while jobID <= (*mtctx).jobIDMask {
        let mutex = (*((*mtctx).jobs).offset(jobID as isize)).job_mutex;
        let cond = (*((*mtctx).jobs).offset(jobID as isize)).job_cond;
        ZSTDMT_releaseBuffer(
            (*mtctx).bufPool,
            (*((*mtctx).jobs).offset(jobID as isize)).dstBuff,
        );
        ptr::write_bytes(
            &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription as *mut u8,
            0,
            ::core::mem::size_of::<ZSTDMT_jobDescription>(),
        );
        (*((*mtctx).jobs).offset(jobID as isize)).job_mutex = mutex;
        (*((*mtctx).jobs).offset(jobID as isize)).job_cond = cond;
        jobID = jobID.wrapping_add(1);
    }
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0;
    (*mtctx).allJobsCompleted = 1;
}
unsafe extern "C" fn ZSTDMT_waitForAllJobsCompleted(mut mtctx: *mut ZSTDMT_CCtx) {
    while (*mtctx).doneJobID < (*mtctx).nextJobID {
        let jobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;
        pthread_mutex_lock(&mut (*((*mtctx).jobs).offset(jobID as isize)).job_mutex);
        while (*((*mtctx).jobs).offset(jobID as isize)).consumed
            < (*((*mtctx).jobs).offset(jobID as isize)).src.size
        {
            pthread_cond_wait(
                &mut (*((*mtctx).jobs).offset(jobID as isize)).job_cond,
                &mut (*((*mtctx).jobs).offset(jobID as isize)).job_mutex,
            );
        }
        pthread_mutex_unlock(&mut (*((*mtctx).jobs).offset(jobID as isize)).job_mutex);
        (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
        (*mtctx).doneJobID;
    }
}
#[export_name = crate::prefix!(ZSTDMT_freeCCtx)]
pub unsafe extern "C" fn ZSTDMT_freeCCtx(mut mtctx: *mut ZSTDMT_CCtx) -> size_t {
    if mtctx.is_null() {
        return 0;
    }
    if (*mtctx).providedFactory() == 0 {
        POOL_free((*mtctx).factory);
    }
    ZSTDMT_releaseAllJobResources(mtctx);
    ZSTDMT_freeJobsTable(
        (*mtctx).jobs,
        ((*mtctx).jobIDMask).wrapping_add(1),
        (*mtctx).cMem,
    );
    ZSTDMT_freeBufferPool((*mtctx).bufPool);
    ZSTDMT_freeCCtxPool((*mtctx).cctxPool);
    ZSTDMT_freeSeqPool((*mtctx).seqPool);
    ZSTDMT_serialState_free(&mut (*mtctx).serial);
    ZSTD_freeCDict((*mtctx).cdictLocal);
    if !((*mtctx).roundBuff.buffer).is_null() {
        ZSTD_customFree(
            (*mtctx).roundBuff.buffer as *mut std::ffi::c_void,
            (*mtctx).cMem,
        );
    }
    ZSTD_customFree(mtctx as *mut std::ffi::c_void, (*mtctx).cMem);
    0
}
#[export_name = crate::prefix!(ZSTDMT_sizeof_CCtx)]
pub unsafe extern "C" fn ZSTDMT_sizeof_CCtx(mut mtctx: *mut ZSTDMT_CCtx) -> size_t {
    if mtctx.is_null() {
        return 0;
    }
    (::core::mem::size_of::<ZSTDMT_CCtx>() as std::ffi::c_ulong)
        .wrapping_add(POOL_sizeof((*mtctx).factory))
        .wrapping_add(ZSTDMT_sizeof_bufferPool((*mtctx).bufPool))
        .wrapping_add(
            (((*mtctx).jobIDMask).wrapping_add(1) as std::ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<ZSTDMT_jobDescription>() as std::ffi::c_ulong),
        )
        .wrapping_add(ZSTDMT_sizeof_CCtxPool((*mtctx).cctxPool))
        .wrapping_add(ZSTDMT_sizeof_seqPool((*mtctx).seqPool))
        .wrapping_add(ZSTD_sizeof_CDict((*mtctx).cdictLocal))
        .wrapping_add((*mtctx).roundBuff.capacity)
}
unsafe extern "C" fn ZSTDMT_resize(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut nbWorkers: std::ffi::c_uint,
) -> size_t {
    if POOL_resize((*mtctx).factory, nbWorkers as size_t) != 0 {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    let err_code = ZSTDMT_expandJobsTable(mtctx, nbWorkers);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    (*mtctx).bufPool = ZSTDMT_expandBufferPool(
        (*mtctx).bufPool,
        (2 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_mul(nbWorkers)
            .wrapping_add(3),
    );
    if ((*mtctx).bufPool).is_null() {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    (*mtctx).cctxPool = ZSTDMT_expandCCtxPool((*mtctx).cctxPool, nbWorkers as std::ffi::c_int);
    if ((*mtctx).cctxPool).is_null() {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    (*mtctx).seqPool = ZSTDMT_expandSeqPool((*mtctx).seqPool, nbWorkers);
    if ((*mtctx).seqPool).is_null() {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    0
}
#[export_name = crate::prefix!(ZSTDMT_updateCParams_whileCompressing)]
pub unsafe extern "C" fn ZSTDMT_updateCParams_whileCompressing(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut cctxParams: *const ZSTD_CCtx_params,
) {
    let saved_wlog = (*mtctx).params.cParams.windowLog;
    let compressionLevel = (*cctxParams).compressionLevel;
    (*mtctx).params.compressionLevel = compressionLevel;
    let mut cParams = ZSTD_getCParamsFromCCtxParams(
        cctxParams,
        ZSTD_CONTENTSIZE_UNKNOWN as u64,
        0,
        ZSTD_cpm_noAttachDict,
    );
    cParams.windowLog = saved_wlog;
    (*mtctx).params.cParams = cParams;
}
#[export_name = crate::prefix!(ZSTDMT_getFrameProgression)]
pub unsafe extern "C" fn ZSTDMT_getFrameProgression(
    mut mtctx: *mut ZSTDMT_CCtx,
) -> ZSTD_frameProgression {
    let mut fps = ZSTD_frameProgression {
        ingested: 0,
        consumed: 0,
        produced: 0,
        flushed: 0,
        currentJobID: 0,
        nbActiveWorkers: 0,
    };
    fps.ingested =
        ((*mtctx).consumed).wrapping_add((*mtctx).inBuff.filled as std::ffi::c_ulonglong);
    fps.consumed = (*mtctx).consumed;
    fps.flushed = (*mtctx).produced;
    fps.produced = fps.flushed;
    fps.currentJobID = (*mtctx).nextJobID;
    fps.nbActiveWorkers = 0;
    let mut jobNb: std::ffi::c_uint = 0;
    let mut lastJobNb = ((*mtctx).nextJobID).wrapping_add((*mtctx).jobReady as std::ffi::c_uint);
    jobNb = (*mtctx).doneJobID;
    while jobNb < lastJobNb {
        let wJobID = jobNb & (*mtctx).jobIDMask;
        let mut jobPtr: *mut ZSTDMT_jobDescription =
            &mut *((*mtctx).jobs).offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
        pthread_mutex_lock(&mut (*jobPtr).job_mutex);
        let cResult = (*jobPtr).cSize;
        let produced = if ERR_isError(cResult) != 0 {
            0
        } else {
            cResult
        };
        let flushed = if ERR_isError(cResult) != 0 {
            0
        } else {
            (*jobPtr).dstFlushed
        };
        fps.ingested = (fps.ingested).wrapping_add((*jobPtr).src.size as std::ffi::c_ulonglong);
        fps.consumed = (fps.consumed).wrapping_add((*jobPtr).consumed as std::ffi::c_ulonglong);
        fps.produced = (fps.produced).wrapping_add(produced as std::ffi::c_ulonglong);
        fps.flushed = (fps.flushed).wrapping_add(flushed as std::ffi::c_ulonglong);
        fps.nbActiveWorkers = (fps.nbActiveWorkers).wrapping_add(
            ((*jobPtr).consumed < (*jobPtr).src.size) as std::ffi::c_int as std::ffi::c_uint,
        );
        pthread_mutex_unlock(&mut (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
        jobNb = jobNb.wrapping_add(1);
    }
    fps
}
#[export_name = crate::prefix!(ZSTDMT_toFlushNow)]
pub unsafe extern "C" fn ZSTDMT_toFlushNow(mut mtctx: *mut ZSTDMT_CCtx) -> size_t {
    let mut toFlush: size_t = 0;
    let jobID = (*mtctx).doneJobID;
    if jobID == (*mtctx).nextJobID {
        return 0;
    }
    let wJobID = jobID & (*mtctx).jobIDMask;
    let jobPtr: *mut ZSTDMT_jobDescription =
        &mut *((*mtctx).jobs).offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
    pthread_mutex_lock(&mut (*jobPtr).job_mutex);
    let cResult = (*jobPtr).cSize;
    let produced = if ERR_isError(cResult) != 0 {
        0
    } else {
        cResult
    };
    let flushed = if ERR_isError(cResult) != 0 {
        0
    } else {
        (*jobPtr).dstFlushed
    };
    toFlush = produced.wrapping_sub(flushed);
    toFlush == 0;
    pthread_mutex_unlock(&mut (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
    toFlush
}
unsafe extern "C" fn ZSTDMT_computeTargetJobLog(
    mut params: *const ZSTD_CCtx_params,
) -> std::ffi::c_uint {
    let mut jobLog: std::ffi::c_uint = 0;
    if (*params).ldmParams.enableLdm as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        jobLog = if 21
            > (ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy))
                .wrapping_add(3)
        {
            21
        } else {
            (ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy)).wrapping_add(3)
        };
    } else {
        jobLog = if 20 > ((*params).cParams.windowLog).wrapping_add(2) {
            20
        } else {
            ((*params).cParams.windowLog).wrapping_add(2)
        };
    }
    if jobLog < (if MEM_32bits() != 0 { 29 } else { 30 }) as std::ffi::c_uint {
        jobLog
    } else {
        (if MEM_32bits() != 0 { 29 } else { 30 }) as std::ffi::c_uint
    }
}
unsafe extern "C" fn ZSTDMT_overlapLog_default(mut strat: ZSTD_strategy) -> std::ffi::c_int {
    match strat as std::ffi::c_uint {
        9 => return 9,
        8 | 7 => return 8,
        6 | 5 => return 7,
        4 | 3 | 2 | 1 | _ => {}
    }
    6
}
unsafe extern "C" fn ZSTDMT_overlapLog(
    mut ovlog: std::ffi::c_int,
    mut strat: ZSTD_strategy,
) -> std::ffi::c_int {
    if ovlog == 0 {
        return ZSTDMT_overlapLog_default(strat);
    }
    ovlog
}
unsafe extern "C" fn ZSTDMT_computeOverlapSize(mut params: *const ZSTD_CCtx_params) -> size_t {
    let overlapRLog = 9 - ZSTDMT_overlapLog((*params).overlapLog, (*params).cParams.strategy);
    let mut ovLog = (if overlapRLog >= 8 {
        0
    } else {
        ((*params).cParams.windowLog).wrapping_sub(overlapRLog as std::ffi::c_uint)
    }) as std::ffi::c_int;
    if (*params).ldmParams.enableLdm as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        ovLog = (if (*params).cParams.windowLog
            < (ZSTDMT_computeTargetJobLog(params)).wrapping_sub(2)
        {
            (*params).cParams.windowLog
        } else {
            (ZSTDMT_computeTargetJobLog(params)).wrapping_sub(2)
        })
        .wrapping_sub(overlapRLog as std::ffi::c_uint) as std::ffi::c_int;
    }
    if ovLog == 0 {
        0
    } else {
        (1) << ovLog
    }
}
#[export_name = crate::prefix!(ZSTDMT_initCStream_internal)]
pub unsafe extern "C" fn ZSTDMT_initCStream_internal(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
    mut dictContentType: ZSTD_dictContentType_e,
    mut cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    mut pledgedSrcSize: std::ffi::c_ulonglong,
) -> size_t {
    if params.nbWorkers != (*mtctx).params.nbWorkers {
        let err_code = ZSTDMT_resize(mtctx, params.nbWorkers as std::ffi::c_uint);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
    }
    if params.jobSize != 0 && params.jobSize < ZSTDMT_JOBSIZE_MIN as size_t {
        params.jobSize = ZSTDMT_JOBSIZE_MIN as size_t;
    }
    if params.jobSize
        > (if MEM_32bits() != 0 {
            512 * ((1) << 20)
        } else {
            1024 * ((1) << 20)
        }) as size_t
    {
        params.jobSize = (if MEM_32bits() != 0 {
            512 * ((1) << 20)
        } else {
            1024 * ((1) << 20)
        }) as size_t;
    }
    if (*mtctx).allJobsCompleted == 0 {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        (*mtctx).allJobsCompleted = 1;
    }
    (*mtctx).params = params;
    (*mtctx).frameContentSize = pledgedSrcSize;
    ZSTD_freeCDict((*mtctx).cdictLocal);
    if !dict.is_null() {
        (*mtctx).cdictLocal = ZSTD_createCDict_advanced(
            dict,
            dictSize,
            ZSTD_dlm_byCopy,
            dictContentType,
            params.cParams,
            (*mtctx).cMem,
        );
        (*mtctx).cdict = (*mtctx).cdictLocal;
        if ((*mtctx).cdictLocal).is_null() {
            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        }
    } else {
        (*mtctx).cdictLocal = NULL_0 as *mut ZSTD_CDict;
        (*mtctx).cdict = cdict;
    }
    (*mtctx).targetPrefixSize = ZSTDMT_computeOverlapSize(&mut params);
    (*mtctx).targetSectionSize = params.jobSize;
    if (*mtctx).targetSectionSize == 0 {
        (*mtctx).targetSectionSize = ((1) << ZSTDMT_computeTargetJobLog(&mut params)) as size_t;
    }
    if params.rsyncable != 0 {
        let jobSizeKB = ((*mtctx).targetSectionSize >> 10) as u32;
        let rsyncBits = (ZSTD_highbit32(jobSizeKB)).wrapping_add(10);
        (*mtctx).rsync.hash = 0;
        (*mtctx).rsync.hitMask = ((1 as std::ffi::c_ulonglong) << rsyncBits)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong)
            as u64;
        (*mtctx).rsync.primePower = ZSTD_rollingHash_primePower(RSYNC_LENGTH as u32);
    }
    if (*mtctx).targetSectionSize < (*mtctx).targetPrefixSize {
        (*mtctx).targetSectionSize = (*mtctx).targetPrefixSize;
    }
    ZSTDMT_setBufferSize(
        (*mtctx).bufPool,
        ZSTD_compressBound((*mtctx).targetSectionSize),
    );
    let windowSize = (if (*mtctx).params.ldmParams.enableLdm as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        (1) << (*mtctx).params.cParams.windowLog
    } else {
        0
    }) as size_t;
    let nbSlackBuffers = (2 + ((*mtctx).targetPrefixSize > 0) as std::ffi::c_int) as size_t;
    let slackSize = (*mtctx).targetSectionSize * nbSlackBuffers;
    let nbWorkers = (if (*mtctx).params.nbWorkers > 1 {
        (*mtctx).params.nbWorkers
    } else {
        1
    }) as size_t;
    let sectionsSize = (*mtctx).targetSectionSize * nbWorkers;
    let capacity = (if windowSize > sectionsSize {
        windowSize
    } else {
        sectionsSize
    })
    .wrapping_add(slackSize);
    if (*mtctx).roundBuff.capacity < capacity {
        if !((*mtctx).roundBuff.buffer).is_null() {
            ZSTD_customFree(
                (*mtctx).roundBuff.buffer as *mut std::ffi::c_void,
                (*mtctx).cMem,
            );
        }
        (*mtctx).roundBuff.buffer = ZSTD_customMalloc(capacity, (*mtctx).cMem) as *mut u8;
        if ((*mtctx).roundBuff.buffer).is_null() {
            (*mtctx).roundBuff.capacity = 0;
            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        }
        (*mtctx).roundBuff.capacity = capacity;
    }
    (*mtctx).roundBuff.pos = 0;
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0;
    (*mtctx).inBuff.prefix = kNullRange;
    (*mtctx).doneJobID = 0;
    (*mtctx).nextJobID = 0;
    (*mtctx).frameEnded = 0;
    (*mtctx).allJobsCompleted = 0;
    (*mtctx).consumed = 0;
    (*mtctx).produced = 0;
    ZSTD_freeCDict((*mtctx).cdictLocal);
    (*mtctx).cdictLocal = NULL_0 as *mut ZSTD_CDict;
    (*mtctx).cdict = NULL_0 as *const ZSTD_CDict;
    if !dict.is_null() {
        if dictContentType as std::ffi::c_uint
            == ZSTD_dct_rawContent as std::ffi::c_int as std::ffi::c_uint
        {
            (*mtctx).inBuff.prefix.start = dict as *const u8 as *const std::ffi::c_void;
            (*mtctx).inBuff.prefix.size = dictSize;
        } else {
            (*mtctx).cdictLocal = ZSTD_createCDict_advanced(
                dict,
                dictSize,
                ZSTD_dlm_byRef,
                dictContentType,
                params.cParams,
                (*mtctx).cMem,
            );
            (*mtctx).cdict = (*mtctx).cdictLocal;
            if ((*mtctx).cdictLocal).is_null() {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
        }
    } else {
        (*mtctx).cdict = cdict;
    }
    if ZSTDMT_serialState_reset(
        &mut (*mtctx).serial,
        (*mtctx).seqPool,
        params,
        (*mtctx).targetSectionSize,
        dict,
        dictSize,
        dictContentType,
    ) != 0
    {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    0
}
unsafe extern "C" fn ZSTDMT_writeLastEmptyBlock(mut job: *mut ZSTDMT_jobDescription) {
    (*job).dstBuff = ZSTDMT_getBuffer((*job).bufPool);
    if ((*job).dstBuff.start).is_null() {
        (*job).cSize = -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        return;
    }
    (*job).src = kNullRange;
    (*job).cSize = ZSTD_writeLastEmptyBlock((*job).dstBuff.start, (*job).dstBuff.capacity);
}
unsafe extern "C" fn ZSTDMT_createCompressionJob(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut srcSize: size_t,
    mut endOp: ZSTD_EndDirective,
) -> size_t {
    let jobID = (*mtctx).nextJobID & (*mtctx).jobIDMask;
    let endFrame = (endOp as std::ffi::c_uint == ZSTD_e_end as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int;
    if (*mtctx).nextJobID > ((*mtctx).doneJobID).wrapping_add((*mtctx).jobIDMask) {
        return 0;
    }
    if (*mtctx).jobReady == 0 {
        let mut src = (*mtctx).inBuff.buffer.start as *const u8;
        let fresh4 = &mut (*((*mtctx).jobs).offset(jobID as isize)).src.start;
        *fresh4 = src as *const std::ffi::c_void;
        (*((*mtctx).jobs).offset(jobID as isize)).src.size = srcSize;
        (*((*mtctx).jobs).offset(jobID as isize)).prefix = (*mtctx).inBuff.prefix;
        (*((*mtctx).jobs).offset(jobID as isize)).consumed = 0;
        (*((*mtctx).jobs).offset(jobID as isize)).cSize = 0;
        (*((*mtctx).jobs).offset(jobID as isize)).params = (*mtctx).params;
        let fresh5 = &mut (*((*mtctx).jobs).offset(jobID as isize)).cdict;
        *fresh5 = if (*mtctx).nextJobID == 0 {
            (*mtctx).cdict
        } else {
            NULL_0 as *const ZSTD_CDict
        };
        (*((*mtctx).jobs).offset(jobID as isize)).fullFrameSize = (*mtctx).frameContentSize;
        (*((*mtctx).jobs).offset(jobID as isize)).dstBuff = g_nullBuffer;
        let fresh6 = &mut (*((*mtctx).jobs).offset(jobID as isize)).cctxPool;
        *fresh6 = (*mtctx).cctxPool;
        let fresh7 = &mut (*((*mtctx).jobs).offset(jobID as isize)).bufPool;
        *fresh7 = (*mtctx).bufPool;
        let fresh8 = &mut (*((*mtctx).jobs).offset(jobID as isize)).seqPool;
        *fresh8 = (*mtctx).seqPool;
        let fresh9 = &mut (*((*mtctx).jobs).offset(jobID as isize)).serial;
        *fresh9 = &mut (*mtctx).serial;
        (*((*mtctx).jobs).offset(jobID as isize)).jobID = (*mtctx).nextJobID;
        (*((*mtctx).jobs).offset(jobID as isize)).firstJob =
            ((*mtctx).nextJobID == 0) as std::ffi::c_int as std::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).lastJob = endFrame as std::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).frameChecksumNeeded =
            ((*mtctx).params.fParams.checksumFlag != 0 && endFrame != 0 && (*mtctx).nextJobID > 0)
                as std::ffi::c_int as std::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).dstFlushed = 0;
        (*mtctx).roundBuff.pos = ((*mtctx).roundBuff.pos).wrapping_add(srcSize);
        (*mtctx).inBuff.buffer = g_nullBuffer;
        (*mtctx).inBuff.filled = 0;
        if endFrame == 0 {
            let newPrefixSize = if srcSize < (*mtctx).targetPrefixSize {
                srcSize
            } else {
                (*mtctx).targetPrefixSize
            };
            (*mtctx).inBuff.prefix.start =
                src.offset(srcSize as isize)
                    .offset(-(newPrefixSize as isize)) as *const std::ffi::c_void;
            (*mtctx).inBuff.prefix.size = newPrefixSize;
        } else {
            (*mtctx).inBuff.prefix = kNullRange;
            (*mtctx).frameEnded = endFrame as std::ffi::c_uint;
            if (*mtctx).nextJobID == 0 {
                (*mtctx).params.fParams.checksumFlag = 0;
            }
        }
        if srcSize == 0 && (*mtctx).nextJobID > 0 {
            ZSTDMT_writeLastEmptyBlock(((*mtctx).jobs).offset(jobID as isize));
            (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
            (*mtctx).nextJobID;
            return 0;
        }
    }
    if POOL_tryAdd(
        (*mtctx).factory,
        Some(ZSTDMT_compressionJob as unsafe extern "C" fn(*mut std::ffi::c_void) -> ()),
        &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription
            as *mut std::ffi::c_void,
    ) != 0
    {
        (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
        (*mtctx).nextJobID;
        (*mtctx).jobReady = 0;
    } else {
        (*mtctx).jobReady = 1;
    }
    0
}
unsafe extern "C" fn ZSTDMT_flushProduced(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut output: *mut ZSTD_outBuffer,
    mut blockToFlush: std::ffi::c_uint,
    mut end: ZSTD_EndDirective,
) -> size_t {
    let wJobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;
    pthread_mutex_lock(&mut (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
    if blockToFlush != 0 && (*mtctx).doneJobID < (*mtctx).nextJobID {
        while (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed
            == (*((*mtctx).jobs).offset(wJobID as isize)).cSize
        {
            if (*((*mtctx).jobs).offset(wJobID as isize)).consumed
                == (*((*mtctx).jobs).offset(wJobID as isize)).src.size
            {
                break;
            }
            pthread_cond_wait(
                &mut (*((*mtctx).jobs).offset(wJobID as isize)).job_cond,
                &mut (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex,
            );
        }
    }
    let mut cSize = (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
    let srcConsumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed;
    let srcSize = (*((*mtctx).jobs).offset(wJobID as isize)).src.size;
    pthread_mutex_unlock(&mut (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
    if ERR_isError(cSize) != 0 {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        return cSize;
    }
    if srcConsumed == srcSize && (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded != 0
    {
        let checksum = ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState) as u32;
        MEM_writeLE32(
            ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start as *mut std::ffi::c_char)
                .offset((*((*mtctx).jobs).offset(wJobID as isize)).cSize as isize)
                as *mut std::ffi::c_void,
            checksum,
        );
        cSize = cSize.wrapping_add(4);
        let fresh10 = &mut (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
        *fresh10 = (*fresh10).wrapping_add(4);
        (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded = 0;
    }
    if cSize > 0 {
        let toFlush = if cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
            < ((*output).size).wrapping_sub((*output).pos)
        {
            cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
        } else {
            ((*output).size).wrapping_sub((*output).pos)
        };
        if toFlush > 0 {
            libc::memcpy(
                ((*output).dst as *mut std::ffi::c_char).offset((*output).pos as isize)
                    as *mut std::ffi::c_void,
                ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start
                    as *const std::ffi::c_char)
                    .offset((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed as isize)
                    as *const std::ffi::c_void,
                toFlush as libc::size_t,
            );
        }
        (*output).pos = ((*output).pos).wrapping_add(toFlush);
        let fresh11 = &mut (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed;
        *fresh11 = (*fresh11).wrapping_add(toFlush);
        if srcConsumed == srcSize && (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed == cSize
        {
            ZSTDMT_releaseBuffer(
                (*mtctx).bufPool,
                (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff,
            );
            (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff = g_nullBuffer;
            (*((*mtctx).jobs).offset(wJobID as isize)).cSize = 0;
            (*mtctx).consumed = ((*mtctx).consumed).wrapping_add(srcSize as std::ffi::c_ulonglong);
            (*mtctx).produced = ((*mtctx).produced).wrapping_add(cSize as std::ffi::c_ulonglong);
            (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
            (*mtctx).doneJobID;
        }
    }
    if cSize > (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed {
        return cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed);
    }
    if srcSize > srcConsumed {
        return 1;
    }
    if (*mtctx).doneJobID < (*mtctx).nextJobID {
        return 1;
    }
    if (*mtctx).jobReady != 0 {
        return 1;
    }
    if (*mtctx).inBuff.filled > 0 {
        return 1;
    }
    (*mtctx).allJobsCompleted = (*mtctx).frameEnded;
    if end as std::ffi::c_uint == ZSTD_e_end as std::ffi::c_int as std::ffi::c_uint {
        return ((*mtctx).frameEnded == 0) as std::ffi::c_int as size_t;
    }
    0
}
unsafe extern "C" fn ZSTDMT_getInputDataInUse(mut mtctx: *mut ZSTDMT_CCtx) -> Range {
    let firstJobID = (*mtctx).doneJobID;
    let lastJobID = (*mtctx).nextJobID;
    let mut jobID: std::ffi::c_uint = 0;
    let mut roundBuffCapacity = (*mtctx).roundBuff.capacity;
    let mut nbJobs1stRoundMin = roundBuffCapacity / (*mtctx).targetSectionSize;
    if (lastJobID as size_t) < nbJobs1stRoundMin {
        return kNullRange;
    }
    jobID = firstJobID;
    while jobID < lastJobID {
        let wJobID = jobID & (*mtctx).jobIDMask;
        let mut consumed: size_t = 0;
        pthread_mutex_lock(&mut (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
        consumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed;
        pthread_mutex_unlock(&mut (*((*mtctx).jobs).offset(wJobID as isize)).job_mutex);
        if consumed < (*((*mtctx).jobs).offset(wJobID as isize)).src.size {
            let mut range = (*((*mtctx).jobs).offset(wJobID as isize)).prefix;
            if range.size == 0 {
                range = (*((*mtctx).jobs).offset(wJobID as isize)).src;
            }
            return range;
        }
        jobID = jobID.wrapping_add(1);
    }
    kNullRange
}
unsafe extern "C" fn ZSTDMT_isOverlapped(mut buffer: Buffer, mut range: Range) -> std::ffi::c_int {
    let bufferStart = buffer.start as *const u8;
    let rangeStart = range.start as *const u8;
    if rangeStart.is_null() || bufferStart.is_null() {
        return 0;
    }
    let bufferEnd = bufferStart.offset(buffer.capacity as isize);
    let rangeEnd = rangeStart.offset(range.size as isize);
    if bufferStart == bufferEnd || rangeStart == rangeEnd {
        return 0;
    }
    (bufferStart < rangeEnd && rangeStart < bufferEnd) as std::ffi::c_int
}
unsafe extern "C" fn ZSTDMT_doesOverlapWindow(
    mut buffer: Buffer,
    mut window: ZSTD_window_t,
) -> std::ffi::c_int {
    let mut extDict = Range {
        start: std::ptr::null::<std::ffi::c_void>(),
        size: 0,
    };
    let mut prefix = Range {
        start: std::ptr::null::<std::ffi::c_void>(),
        size: 0,
    };
    extDict.start = (window.dictBase).offset(window.lowLimit as isize) as *const std::ffi::c_void;
    extDict.size = (window.dictLimit).wrapping_sub(window.lowLimit) as size_t;
    prefix.start = (window.base).offset(window.dictLimit as isize) as *const std::ffi::c_void;
    prefix.size = (window.nextSrc).offset_from((window.base).offset(window.dictLimit as isize))
        as std::ffi::c_long as size_t;
    (ZSTDMT_isOverlapped(buffer, extDict) != 0 || ZSTDMT_isOverlapped(buffer, prefix) != 0)
        as std::ffi::c_int
}
unsafe extern "C" fn ZSTDMT_waitForLdmComplete(mut mtctx: *mut ZSTDMT_CCtx, mut buffer: Buffer) {
    if (*mtctx).params.ldmParams.enableLdm as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint
    {
        let mut mutex: *mut pthread_mutex_t = &mut (*mtctx).serial.ldmWindowMutex;
        pthread_mutex_lock(mutex);
        while ZSTDMT_doesOverlapWindow(buffer, (*mtctx).serial.ldmWindow) != 0 {
            pthread_cond_wait(&mut (*mtctx).serial.ldmWindowCond, mutex);
        }
        pthread_mutex_unlock(mutex);
    }
}
unsafe extern "C" fn ZSTDMT_tryGetInputRange(mut mtctx: *mut ZSTDMT_CCtx) -> std::ffi::c_int {
    let inUse = ZSTDMT_getInputDataInUse(mtctx);
    let spaceLeft = ((*mtctx).roundBuff.capacity).wrapping_sub((*mtctx).roundBuff.pos);
    let spaceNeeded = (*mtctx).targetSectionSize;
    let mut buffer = buffer_s {
        start: std::ptr::null_mut::<std::ffi::c_void>(),
        capacity: 0,
    };
    if spaceLeft < spaceNeeded {
        let start = (*mtctx).roundBuff.buffer;
        let prefixSize = (*mtctx).inBuff.prefix.size;
        buffer.start = start as *mut std::ffi::c_void;
        buffer.capacity = prefixSize;
        if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
            return 0;
        }
        ZSTDMT_waitForLdmComplete(mtctx, buffer);
        libc::memmove(
            start as *mut std::ffi::c_void,
            (*mtctx).inBuff.prefix.start,
            prefixSize as libc::size_t,
        );
        (*mtctx).inBuff.prefix.start = start as *const std::ffi::c_void;
        (*mtctx).roundBuff.pos = prefixSize;
    }
    buffer.start = ((*mtctx).roundBuff.buffer).offset((*mtctx).roundBuff.pos as isize)
        as *mut std::ffi::c_void;
    buffer.capacity = spaceNeeded;
    if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
        return 0;
    }
    ZSTDMT_waitForLdmComplete(mtctx, buffer);
    (*mtctx).inBuff.buffer = buffer;
    (*mtctx).inBuff.filled = 0;
    1
}
unsafe extern "C" fn findSynchronizationPoint(
    mut mtctx: *const ZSTDMT_CCtx,
    input: ZSTD_inBuffer,
) -> SyncPoint {
    let istart = (input.src as *const u8).offset(input.pos as isize);
    let primePower = (*mtctx).rsync.primePower;
    let hitMask = (*mtctx).rsync.hitMask;
    let mut syncPoint = SyncPoint {
        toLoad: 0,
        flush: 0,
    };
    let mut hash: u64 = 0;
    let mut prev = std::ptr::null::<u8>();
    let mut pos: size_t = 0;
    syncPoint.toLoad = if (input.size).wrapping_sub(input.pos)
        < ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled)
    {
        (input.size).wrapping_sub(input.pos)
    } else {
        ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled)
    };
    syncPoint.flush = 0;
    if (*mtctx).params.rsyncable == 0 {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled)
        .wrapping_add(input.size)
        .wrapping_sub(input.pos)
        < RSYNC_MIN_BLOCK_SIZE as size_t
    {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad) < RSYNC_LENGTH as size_t {
        return syncPoint;
    }
    if (*mtctx).inBuff.filled < RSYNC_MIN_BLOCK_SIZE as size_t {
        pos = (RSYNC_MIN_BLOCK_SIZE as size_t).wrapping_sub((*mtctx).inBuff.filled);
        if pos >= RSYNC_LENGTH as size_t {
            prev = istart.offset(pos as isize).offset(-(RSYNC_LENGTH as isize));
            hash =
                ZSTD_rollingHash_compute(prev as *const std::ffi::c_void, RSYNC_LENGTH as size_t);
        } else {
            prev = ((*mtctx).inBuff.buffer.start as *const u8)
                .offset((*mtctx).inBuff.filled as isize)
                .offset(-(RSYNC_LENGTH as isize));
            hash = ZSTD_rollingHash_compute(
                prev.offset(pos as isize) as *const std::ffi::c_void,
                (RSYNC_LENGTH as size_t).wrapping_sub(pos),
            );
            hash = ZSTD_rollingHash_append(hash, istart as *const std::ffi::c_void, pos);
        }
    } else {
        pos = 0;
        prev = ((*mtctx).inBuff.buffer.start as *const u8)
            .offset((*mtctx).inBuff.filled as isize)
            .offset(-(RSYNC_LENGTH as isize));
        hash = ZSTD_rollingHash_compute(prev as *const std::ffi::c_void, RSYNC_LENGTH as size_t);
        if hash & hitMask == hitMask {
            syncPoint.toLoad = 0;
            syncPoint.flush = 1;
            return syncPoint;
        }
    }
    while pos < syncPoint.toLoad {
        let toRemove = (if pos < RSYNC_LENGTH as size_t {
            *prev.offset(pos as isize) as std::ffi::c_int
        } else {
            *istart.offset(pos.wrapping_sub(RSYNC_LENGTH as size_t) as isize) as std::ffi::c_int
        }) as u8;
        hash = ZSTD_rollingHash_rotate(hash, toRemove, *istart.offset(pos as isize), primePower);
        if hash & hitMask == hitMask {
            syncPoint.toLoad = pos.wrapping_add(1);
            syncPoint.flush = 1;
            pos = pos.wrapping_add(1);
            break;
        } else {
            pos = pos.wrapping_add(1);
        }
    }
    syncPoint
}
#[export_name = crate::prefix!(ZSTDMT_nextInputSizeHint)]
pub unsafe extern "C" fn ZSTDMT_nextInputSizeHint(mut mtctx: *const ZSTDMT_CCtx) -> size_t {
    let mut hintInSize = ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled);
    if hintInSize == 0 {
        hintInSize = (*mtctx).targetSectionSize;
    }
    hintInSize
}
#[export_name = crate::prefix!(ZSTDMT_compressStream_generic)]
pub unsafe extern "C" fn ZSTDMT_compressStream_generic(
    mut mtctx: *mut ZSTDMT_CCtx,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
    mut endOp: ZSTD_EndDirective,
) -> size_t {
    let mut forwardInputProgress = 0;
    if (*mtctx).frameEnded != 0
        && endOp as std::ffi::c_uint == ZSTD_e_continue as std::ffi::c_int as std::ffi::c_uint
    {
        return -(ZSTD_error_stage_wrong as std::ffi::c_int) as size_t;
    }
    if (*mtctx).jobReady == 0 && (*input).size > (*input).pos {
        if ((*mtctx).inBuff.buffer.start).is_null() {
            ZSTDMT_tryGetInputRange(mtctx);
            0;
        }
        if !((*mtctx).inBuff.buffer.start).is_null() {
            let syncPoint = findSynchronizationPoint(mtctx, *input);
            if syncPoint.flush != 0
                && endOp as std::ffi::c_uint
                    == ZSTD_e_continue as std::ffi::c_int as std::ffi::c_uint
            {
                endOp = ZSTD_e_flush;
            }
            libc::memcpy(
                ((*mtctx).inBuff.buffer.start as *mut std::ffi::c_char)
                    .offset((*mtctx).inBuff.filled as isize)
                    as *mut std::ffi::c_void,
                ((*input).src as *const std::ffi::c_char).offset((*input).pos as isize)
                    as *const std::ffi::c_void,
                syncPoint.toLoad as libc::size_t,
            );
            (*input).pos = ((*input).pos).wrapping_add(syncPoint.toLoad);
            (*mtctx).inBuff.filled = ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad);
            forwardInputProgress = (syncPoint.toLoad > 0) as std::ffi::c_int as std::ffi::c_uint;
        }
    }
    if (*input).pos < (*input).size
        && endOp as std::ffi::c_uint == ZSTD_e_end as std::ffi::c_int as std::ffi::c_uint
    {
        endOp = ZSTD_e_flush;
    }
    if (*mtctx).jobReady != 0
        || (*mtctx).inBuff.filled >= (*mtctx).targetSectionSize
        || endOp as std::ffi::c_uint != ZSTD_e_continue as std::ffi::c_int as std::ffi::c_uint
            && (*mtctx).inBuff.filled > 0
        || endOp as std::ffi::c_uint == ZSTD_e_end as std::ffi::c_int as std::ffi::c_uint
            && (*mtctx).frameEnded == 0
    {
        let jobSize = (*mtctx).inBuff.filled;
        let err_code = ZSTDMT_createCompressionJob(mtctx, jobSize, endOp);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
    }
    let remainingToFlush = ZSTDMT_flushProduced(
        mtctx,
        output,
        (forwardInputProgress == 0) as std::ffi::c_int as std::ffi::c_uint,
        endOp,
    );
    if (*input).pos < (*input).size {
        return if remainingToFlush > 1 {
            remainingToFlush
        } else {
            1
        };
    }
    remainingToFlush
}
