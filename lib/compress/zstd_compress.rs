use core::ptr;

use crate::lib::polyfill::PointerExt;

pub type ZSTD_CCtx = ZSTD_CCtx_s;
#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub(super) stage: ZSTD_compressionStage_e,
    pub(super) cParamsChanged: core::ffi::c_int,
    pub(super) bmi2: core::ffi::c_int,
    pub(super) requestedParams: ZSTD_CCtx_params,
    pub(super) appliedParams: ZSTD_CCtx_params,
    pub(super) simpleApiParams: ZSTD_CCtx_params,
    pub(super) dictID: u32,
    pub(super) dictContentSize: size_t,
    pub(super) workspace: ZSTD_cwksp,
    pub(super) blockSizeMax: size_t,
    pub(super) pledgedSrcSizePlusOne: core::ffi::c_ulonglong,
    pub(super) consumedSrcSize: core::ffi::c_ulonglong,
    pub(super) producedCSize: core::ffi::c_ulonglong,
    pub(super) xxhState: XXH64_state_t,
    pub(super) customMem: ZSTD_customMem,
    pub(super) pool: *mut ZSTD_threadPool,
    pub(super) staticSize: size_t,
    pub(super) seqCollector: SeqCollector,
    pub(super) isFirstBlock: core::ffi::c_int,
    pub(super) initialized: core::ffi::c_int,
    pub(super) seqStore: SeqStore_t,
    pub(super) ldmState: ldmState_t,
    pub(super) ldmSequences: *mut rawSeq,
    pub(super) maxNbLdmSequences: size_t,
    pub(super) externSeqStore: RawSeqStore_t,
    pub(super) blockState: ZSTD_blockState_t,
    pub(super) tmpWorkspace: *mut core::ffi::c_void,
    pub(super) tmpWkspSize: size_t,
    pub(super) bufferedPolicy: ZSTD_buffered_policy_e,
    pub(super) inBuff: *mut u8,
    pub(super) inBuffSize: size_t,
    pub(super) inToCompress: size_t,
    pub(super) inBuffPos: size_t,
    pub(super) inBuffTarget: size_t,
    pub(super) outBuff: *mut u8,
    pub(super) outBuffSize: size_t,
    pub(super) outBuffContentSize: size_t,
    pub(super) outBuffFlushedSize: size_t,
    pub(super) streamStage: ZSTD_cStreamStage,
    pub(super) frameEnded: u32,
    pub(super) expectedInBuffer: ZSTD_inBuffer,
    pub(super) stableIn_notConsumed: size_t,
    pub(super) expectedOutBufferSize: size_t,
    pub(super) localDict: ZSTD_localDict,
    pub(super) cdict: *const ZSTD_CDict,
    pub(super) prefixDict: ZSTD_prefixDict,
    pub(super) mtctx: *mut ZSTDMT_CCtx,
    pub(super) traceCtx: ZSTD_TraceCtx,
    pub(super) blockSplitCtx: ZSTD_blockSplitCtx,
    pub(super) extSeqBuf: *mut ZSTD_Sequence,
    pub(super) extSeqBufCapacity: size_t,
}
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
pub type SeqDef = SeqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqDef_s {
    pub offBase: u32,
    pub litLength: u16,
    pub mlBase: u16,
}
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}
pub type ZSTD_CDict = ZSTD_CDict_s;
#[repr(C)]
pub struct ZSTD_CDict_s {
    pub dictContent: *const core::ffi::c_void,
    pub dictContentSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub entropyWorkspace: *mut u32,
    pub workspace: ZSTD_cwksp,
    pub matchState: ZSTD_MatchState_t,
    pub cBlockState: ZSTD_compressedBlockState_t,
    pub customMem: ZSTD_customMem,
    pub dictID: u32,
    pub compressionLevel: core::ffi::c_int,
    pub useRowMatchFinder: ZSTD_ParamSwitch_e,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ParamSwitch {
    Auto = 0,
    Enable = 1,
    Disable = 2,
}

impl TryFrom<i32> for ParamSwitch {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Auto),
            1 => Ok(Self::Enable),
            2 => Ok(Self::Disable),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [u32; 3],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
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
    pub forceNonContiguous: core::ffi::c_int,
    pub dedicatedDictSearch: core::ffi::c_int,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_MatchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const RawSeqStore_t,
    pub prefetchCDictTables: core::ffi::c_int,
    pub lazySkipping: core::ffi::c_int,
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
    pub literalCompressionMode: ParamSwitch,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut core::ffi::c_void,
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}
pub type ZSTD_cStreamStage = core::ffi::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub type ZSTD_buffered_policy_e = core::ffi::c_uint;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;
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
    pub literalCompressionMode: ParamSwitch,
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
pub type ZSTD_compressionStage_e = core::ffi::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;
pub type unalignArch = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_symbolEncodingTypeStats_t {
    pub LLtype: u32,
    pub Offtype: u32,
    pub MLtype: u32,
    pub size: size_t,
    pub lastCountSize: size_t,
    pub longOffsets: core::ffi::c_int,
}
pub type S16 = i16;
pub type ZSTD_DefaultPolicy_e = core::ffi::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_DefaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_DefaultPolicy_e = 0;
pub type Repcodes_t = repcodes_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [u32; 3],
}
pub const ZSTDbss_noCompress: C2RustUnnamed_2 = 1;
pub const ZSTDbss_compress: C2RustUnnamed_2 = 0;
pub type ZSTD_BlockCompressor_f = Option<
    unsafe fn(
        &mut ZSTD_MatchState_t,
        &mut SeqStore_t,
        *mut u32,
        *const core::ffi::c_void,
        size_t,
    ) -> size_t,
>;
pub type ZSTD_dictMode_e = core::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_SequencePosition {
    pub idx: u32,
    pub posInSequence: u32,
    pub posInSrc: size_t,
}
pub type S64 = i64;
#[repr(C)]
pub struct seqStoreSplits {
    pub splitLocations: *mut u32,
    pub idx: size_t,
}

pub type ZSTD_dictTableLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = core::ffi::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
pub type ZSTD_compResetPolicy_e = core::ffi::c_uint;
pub const ZSTDcrp_leaveDirty: ZSTD_compResetPolicy_e = 1;
pub const ZSTDcrp_makeClean: ZSTD_compResetPolicy_e = 0;
pub type ZSTD_resetTarget_e = core::ffi::c_uint;
pub const ZSTD_resetTarget_CCtx: ZSTD_resetTarget_e = 1;
pub const ZSTD_resetTarget_CDict: ZSTD_resetTarget_e = 0;
pub type ZSTD_indexResetPolicy_e = core::ffi::c_uint;
pub const ZSTDirp_reset: ZSTD_indexResetPolicy_e = 1;
pub const ZSTDirp_continue: ZSTD_indexResetPolicy_e = 0;
pub type ZSTD_CParamMode_e = core::ffi::c_uint;
pub const ZSTD_cpm_unknown: ZSTD_CParamMode_e = 3;
pub const ZSTD_cpm_createCDict: ZSTD_CParamMode_e = 2;
pub const ZSTD_cpm_attachDict: ZSTD_CParamMode_e = 1;
pub const ZSTD_cpm_noAttachDict: ZSTD_CParamMode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cpuid_t {
    pub f1c: u32,
    pub f1d: u32,
    pub f7b: u32,
    pub f7c: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: core::ffi::c_int,
    pub upperBound: core::ffi::c_int,
}
pub type ZSTD_EndDirective = core::ffi::c_uint;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type ZSTD_CStream = ZSTD_CCtx;
pub type ZSTD_SequenceCopier_f = Option<
    unsafe fn(
        *mut ZSTD_CCtx,
        *mut ZSTD_SequencePosition,
        *const ZSTD_Sequence,
        size_t,
        *const core::ffi::c_void,
        size_t,
        ZSTD_ParamSwitch_e,
    ) -> size_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSummary {
    pub nbSequences: size_t,
    pub blockSize: size_t,
    pub litSize: size_t,
}
pub type C2RustUnnamed_2 = core::ffi::c_uint;
pub const ZSTD_WINDOWLOG_MIN: core::ffi::c_int = 10;
pub const ZSTD_HASHLOG_MIN: core::ffi::c_int = 6;
pub const ZSTD_CHAINLOG_MAX_32: core::ffi::c_int = 29;
pub const ZSTD_CHAINLOG_MAX_64: core::ffi::c_int = 30;
pub const ZSTD_CHAINLOG_MIN: core::ffi::c_int = ZSTD_HASHLOG_MIN;
pub const ZSTD_SEARCHLOG_MIN: core::ffi::c_int = 1;
pub const ZSTD_MINMATCH_MAX: core::ffi::c_int = 7;
pub const ZSTD_MINMATCH_MIN: core::ffi::c_int = 3;
pub const ZSTD_TARGETLENGTH_MAX: core::ffi::c_int = ZSTD_BLOCKSIZE_MAX;
pub const ZSTD_TARGETLENGTH_MIN: core::ffi::c_int = 0;
pub const ZSTD_STRATEGY_MIN: core::ffi::c_int = ZSTD_fast as core::ffi::c_int;
pub const ZSTD_STRATEGY_MAX: core::ffi::c_int = ZSTD_btultra2 as core::ffi::c_int;
pub const ZSTD_OVERLAPLOG_MIN: core::ffi::c_int = 0;
pub const ZSTD_OVERLAPLOG_MAX: core::ffi::c_int = 9;
pub const ZSTD_LDM_HASHLOG_MIN: core::ffi::c_int = ZSTD_HASHLOG_MIN;
pub const ZSTD_LDM_MINMATCH_MIN: core::ffi::c_int = 4;
pub const ZSTD_LDM_MINMATCH_MAX: core::ffi::c_int = 4096;
pub const ZSTD_LDM_BUCKETSIZELOG_MIN: core::ffi::c_int = 1;
pub const ZSTD_LDM_BUCKETSIZELOG_MAX: core::ffi::c_int = 8;
pub const ZSTD_LDM_HASHRATELOG_MIN: core::ffi::c_int = 0;
pub const ZSTD_TARGETCBLOCKSIZE_MIN: core::ffi::c_int = 1340;
pub const ZSTD_TARGETCBLOCKSIZE_MAX: core::ffi::c_int = ZSTD_BLOCKSIZE_MAX;
pub const ZSTD_SRCSIZEHINT_MIN: core::ffi::c_int = 0;
pub const ZSTD_SRCSIZEHINT_MAX: core::ffi::c_int = INT_MAX;
pub const ZSTD_c_rsyncable: core::ffi::c_int = 500;
pub const ZSTD_c_format: core::ffi::c_int = 10;
pub const ZSTD_c_forceMaxWindow: core::ffi::c_int = 1000;
pub const ZSTD_c_forceAttachDict: core::ffi::c_int = 1001;
pub const ZSTD_c_literalCompressionMode: core::ffi::c_int = 1002;
pub const ZSTD_c_srcSizeHint: core::ffi::c_int = 1004;
pub const ZSTD_c_enableDedicatedDictSearch: core::ffi::c_int = 1005;
pub const ZSTD_c_stableInBuffer: core::ffi::c_int = 1006;
pub const ZSTD_c_stableOutBuffer: core::ffi::c_int = 1007;
pub const ZSTD_c_blockDelimiters: core::ffi::c_int = 1008;
pub const ZSTD_c_validateSequences: core::ffi::c_int = 1009;
pub const ZSTD_BLOCKSPLITTER_LEVEL_MAX: core::ffi::c_int = 6;
pub const ZSTD_c_blockSplitterLevel: core::ffi::c_int = 1017;
pub const ZSTD_c_splitAfterSequences: core::ffi::c_int = 1010;
pub const ZSTD_c_useRowMatchFinder: core::ffi::c_int = 1011;
pub const ZSTD_c_deterministicRefPrefix: core::ffi::c_int = 1012;
pub const ZSTD_c_prefetchCDictTables: core::ffi::c_int = 1013;
pub const ZSTD_c_enableSeqProducerFallback: core::ffi::c_int = 1014;
pub const ZSTD_c_maxBlockSize: core::ffi::c_int = 1015;
pub const ZSTD_c_repcodeResolution: core::ffi::c_int = 1016;
pub const HASH_READ_SIZE: core::ffi::c_int = 8;
pub const ZSTD_DUBT_UNSORTED_MARK: core::ffi::c_int = 1;
static mut kNullRawSeqStore: RawSeqStore_t = RawSeqStore_t {
    seq: core::ptr::null_mut(),
    pos: 0,
    posInSequence: 0,
    size: 0,
    capacity: 0,
};
pub const ZSTD_OPT_SIZE: core::ffi::c_int = ZSTD_OPT_NUM + 3;
pub const ZSTD_MAX_NB_BLOCK_SPLITS: core::ffi::c_int = 196;
#[inline]
unsafe fn ZSTD_LLcode(litLength: u32) -> u32 {
    static LL_Code: [u8; 64] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20,
        20, 20, 20, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23,
        24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    ];
    static LL_deltaCode: u32 = 19;
    if litLength > 63 {
        (ZSTD_highbit32(litLength)).wrapping_add(LL_deltaCode)
    } else {
        core::ffi::c_uint::from(*LL_Code.as_ptr().offset(litLength as isize))
    }
}
#[inline]
unsafe fn ZSTD_MLcode(mlBase: u32) -> u32 {
    static ML_Code: [u8; 128] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 32, 33, 33, 34, 34, 35, 35, 36, 36, 36, 36, 37, 37, 37, 37,
        38, 38, 38, 38, 38, 38, 38, 38, 39, 39, 39, 39, 39, 39, 39, 39, 40, 40, 40, 40, 40, 40, 40,
        40, 40, 40, 40, 40, 40, 40, 40, 40, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41,
        41, 41, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
        42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
    ];
    static ML_deltaCode: u32 = 36;
    if mlBase > 127 {
        (ZSTD_highbit32(mlBase)).wrapping_add(ML_deltaCode)
    } else {
        core::ffi::c_uint::from(*ML_Code.as_ptr().offset(mlBase as isize))
    }
}
#[inline]
unsafe fn ZSTD_cParam_withinBounds(
    cParam: ZSTD_cParameter,
    value: core::ffi::c_int,
) -> core::ffi::c_int {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) {
        return 0;
    }
    if value < bounds.lowerBound {
        return 0;
    }
    if value > bounds.upperBound {
        return 0;
    }
    1
}
#[inline]
unsafe fn ZSTD_noCompressBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastBlock: u32,
) -> size_t {
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_raw as core::ffi::c_int as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }
    MEM_writeLE24(dst, cBlockHeader24);
    libc::memcpy(
        (dst as *mut u8).add(ZSTD_blockHeaderSize) as *mut core::ffi::c_void,
        src,
        srcSize as libc::size_t,
    );
    ZSTD_blockHeaderSize.wrapping_add(srcSize)
}
#[inline]
unsafe fn ZSTD_rleCompressBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: u8,
    srcSize: size_t,
    lastBlock: u32,
) -> size_t {
    let op = dst as *mut u8;
    let cBlockHeader = lastBlock
        .wrapping_add((bt_rle as core::ffi::c_int as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    if dstCapacity < 4 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader);
    *op.add(3) = src;
    4
}
#[inline]
unsafe fn ZSTD_minGain(srcSize: size_t, strat: ZSTD_strategy) -> size_t {
    let minlog =
        if strat as core::ffi::c_uint >= ZSTD_btultra as core::ffi::c_int as core::ffi::c_uint {
            strat.wrapping_sub(1)
        } else {
            6
        };
    (srcSize >> minlog).wrapping_add(2)
}
#[inline]
unsafe fn ZSTD_literalsCompressionIsDisabled(
    cctxParams: *const ZSTD_CCtx_params,
) -> core::ffi::c_int {
    match (*cctxParams).literalCompressionMode {
        ParamSwitch::Enable => 0,
        ParamSwitch::Disable => 1,
        ParamSwitch::Auto => core::ffi::c_int::from(
            (*cctxParams).cParams.strategy as core::ffi::c_uint
                == ZSTD_fast as core::ffi::c_int as core::ffi::c_uint
                && (*cctxParams).cParams.targetLength > 0,
        ),
    }
}
pub const REPCODE1_TO_OFFBASE: core::ffi::c_int = 1;
pub const REPCODE3_TO_OFFBASE: core::ffi::c_int = 3;

#[inline]
unsafe fn ZSTD_window_clear(window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).wrapping_offset_from((*window).base) as size_t;
    let end = endT as u32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
#[inline]
pub(crate) unsafe fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> u32 {
    core::ffi::c_int::from(window.lowLimit < window.dictLimit) as u32
}
#[inline]
unsafe fn ZSTD_matchState_dictMode(ms: *const ZSTD_MatchState_t) -> ZSTD_dictMode_e {
    (if ZSTD_window_hasExtDict((*ms).window) != 0 {
        ZSTD_extDict as core::ffi::c_int
    } else if !((*ms).dictMatchState).is_null() {
        if (*(*ms).dictMatchState).dedicatedDictSearch != 0 {
            ZSTD_dedicatedDictSearch as core::ffi::c_int
        } else {
            ZSTD_dictMatchState as core::ffi::c_int
        }
    } else {
        ZSTD_noDict as core::ffi::c_int
    }) as ZSTD_dictMode_e
}

#[inline]
unsafe fn ZSTD_window_correctOverflow(
    window: *mut ZSTD_window_t,
    cycleLog: u32,
    maxDist: u32,
    src: *const core::ffi::c_void,
) -> u32 {
    let cycleSize = (1 as core::ffi::c_uint) << cycleLog;
    let cycleMask = cycleSize.wrapping_sub(1);
    let curr = (src as *const u8).offset_from((*window).base) as core::ffi::c_long as u32;
    let currentCycle = curr & cycleMask;
    let currentCycleCorrection = if currentCycle < ZSTD_WINDOW_START_INDEX as u32 {
        if cycleSize > 2 {
            cycleSize
        } else {
            2
        }
    } else {
        0
    };
    let newCurrent = currentCycle
        .wrapping_add(currentCycleCorrection)
        .wrapping_add(if maxDist > cycleSize {
            maxDist
        } else {
            cycleSize
        });
    let correction = curr.wrapping_sub(newCurrent);
    if ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY == 0 {
        // Loose bound, should be around 1<<29 (see above)
        assert!(correction > 1 << 28);
    }
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
    correction
}
#[inline]
unsafe fn ZSTD_window_enforceMaxDist(
    window: *mut ZSTD_window_t,
    blockEnd: *const core::ffi::c_void,
    maxDist: u32,
    loadedDictEndPtr: *mut u32,
    dictMatchStatePtr: *mut *const ZSTD_MatchState_t,
) {
    let blockEndIdx =
        (blockEnd as *const u8).wrapping_offset_from((*window).base) as core::ffi::c_long as u32;
    let loadedDictEnd = if !loadedDictEndPtr.is_null() {
        *loadedDictEndPtr
    } else {
        0
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
            *loadedDictEndPtr = 0;
        }
        if !dictMatchStatePtr.is_null() {
            *dictMatchStatePtr = core::ptr::null();
        }
    }
}
#[inline]
unsafe fn ZSTD_checkDictValidity(
    window: *const ZSTD_window_t,
    blockEnd: *const core::ffi::c_void,
    maxDist: u32,
    loadedDictEndPtr: *mut u32,
    dictMatchStatePtr: *mut *const ZSTD_MatchState_t,
) {
    let blockEndIdx =
        (blockEnd as *const u8).wrapping_offset_from((*window).base) as core::ffi::c_long as u32;
    let loadedDictEnd = *loadedDictEndPtr;
    if blockEndIdx > loadedDictEnd.wrapping_add(maxDist) || loadedDictEnd != (*window).dictLimit {
        *loadedDictEndPtr = 0;
        *dictMatchStatePtr = core::ptr::null();
    } else {
        // FIXME: add log
        // *loadedDictEndPtr != 0;
    };
}
#[inline]
unsafe fn ZSTD_window_init(window: *mut ZSTD_window_t) {
    ptr::write_bytes(
        window as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_window_t>(),
    );
    (*window).base = b" \0" as *const u8 as *const core::ffi::c_char as *const u8;
    (*window).dictBase = b" \0" as *const u8 as *const core::ffi::c_char as *const u8;
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize);
    (*window).nbOverflowCorrections = 0;
}
#[inline]
unsafe fn ZSTD_window_update(
    window: *mut ZSTD_window_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    forceNonContiguous: core::ffi::c_int,
) -> u32 {
    let ip = src as *const u8;
    let mut contiguous = 1;
    if srcSize == 0 {
        return contiguous;
    }
    if src != (*window).nextSrc as *const core::ffi::c_void || forceNonContiguous != 0 {
        let distanceFromBase = ((*window).nextSrc).wrapping_offset_from((*window).base) as size_t;
        (*window).lowLimit = (*window).dictLimit;
        (*window).dictLimit = distanceFromBase as u32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.wrapping_sub(distanceFromBase);
        if ((*window).dictLimit).wrapping_sub((*window).lowLimit) < HASH_READ_SIZE as u32 {
            (*window).lowLimit = (*window).dictLimit;
        }
        contiguous = 0;
    }
    (*window).nextSrc = ip.add(srcSize);
    if (ip.add(srcSize) > ((*window).dictBase).wrapping_offset((*window).lowLimit as isize))
        && (ip < ((*window).dictBase).wrapping_offset((*window).dictLimit as isize))
    {
        let highInputIdx = ip.add(srcSize).offset_from((*window).dictBase) as size_t;
        let lowLimitMax = if highInputIdx > (*window).dictLimit as size_t {
            (*window).dictLimit
        } else {
            highInputIdx as u32
        };
        (*window).lowLimit = lowLimitMax;
    }
    contiguous
}
pub const ZSTD_SHORT_CACHE_TAG_BITS: core::ffi::c_int = 8;
#[inline]
unsafe fn ZSTD_hasExtSeqProd(params: *const ZSTD_CCtx_params) -> core::ffi::c_int {
    core::ffi::c_int::from(((*params).extSeqProdFunc).is_some())
}

use libc::{ptrdiff_t, size_t};

use crate::lib::common::allocations::{ZSTD_customCalloc, ZSTD_customFree, ZSTD_customMalloc};
use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::entropy_common::FSE_readNCount;
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{
    FSE_CTable, FSE_repeat, FSE_repeat_check, FSE_repeat_none, FSE_repeat_valid,
};
use crate::lib::common::huf::{
    HUF_CElt, HUF_flags_optimalDepth, HUF_repeat, HUF_repeat_check, HUF_repeat_none,
    HUF_repeat_valid, HUF_OPTIMAL_DEPTH_THRESHOLD, HUF_SYMBOLVALUE_MAX, HUF_WORKSPACE_SIZE,
};
use crate::lib::common::mem::{
    MEM_32bits, MEM_64bits, MEM_isLittleEndian, MEM_read64, MEM_readLE32, MEM_readST,
    MEM_writeLE16, MEM_writeLE24, MEM_writeLE32, MEM_writeLE64,
};
use crate::lib::common::pool::ZSTD_threadPool;
use crate::lib::common::xxhash::{
    XXH64_state_t, ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_update_slice,
};
use crate::lib::common::zstd_internal::{
    bt_compressed, bt_raw, bt_rle, repStartValue, DefaultMaxOff, LLFSELog, LL_bits, LL_defaultNorm,
    LL_defaultNormLog, LitHufLog, Litbits, MLFSELog, ML_bits, ML_defaultNorm, ML_defaultNormLog,
    MaxLL, MaxML, MaxOff, OF_defaultNorm, OF_defaultNormLog, OffFSELog, ZSTD_cpuSupportsBmi2,
    ZSTD_limitCopy, MINMATCH, WILDCOPY_OVERLENGTH, ZSTD_OPT_NUM, ZSTD_REP_NUM,
    ZSTD_WORKSPACETOOLARGE_FACTOR, ZSTD_WORKSPACETOOLARGE_MAXDURATION,
};
use crate::lib::common::zstd_trace::{
    ZSTD_Trace, ZSTD_TraceCtx, ZSTD_trace_compress_begin, ZSTD_trace_compress_end,
};
use crate::lib::compress::fse_compress::FSE_buildCTable_wksp;
use crate::lib::compress::hist::{HIST_countFast_wksp, HIST_count_wksp};
use crate::lib::compress::huf_compress::{
    HUF_buildCTable_wksp, HUF_estimateCompressedSize, HUF_optimalTableLog, HUF_readCTable,
    HUF_validateCTable, HUF_writeCTable_wksp,
};
use crate::lib::compress::zstd_compress_internal::{
    zop_dynamic, ZSTD_OptPrice_e, ZSTD_count, ZSTD_getSequenceLength, ZSTD_llt_literalLength,
    ZSTD_llt_matchLength, ZSTD_llt_none, ZSTD_longLengthType_e, ZSTD_storeSeq, ZSTD_storeSeqOnly,
    ZSTD_updateRep, ZSTD_window_needOverflowCorrection, ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY,
    ZSTD_WINDOW_START_INDEX,
};
use crate::lib::compress::zstd_compress_literals::ZSTD_compressLiterals;
use crate::lib::compress::zstd_compress_sequences::{
    ZSTD_buildCTable, ZSTD_crossEntropyCost, ZSTD_encodeSequences, ZSTD_fseBitCost,
    ZSTD_selectEncodingType,
};
use crate::lib::compress::zstd_compress_superblock::ZSTD_compressSuperBlock;
use crate::lib::compress::zstd_double_fast::{
    ZSTD_compressBlock_doubleFast, ZSTD_compressBlock_doubleFast_dictMatchState,
    ZSTD_compressBlock_doubleFast_extDict, ZSTD_fillDoubleHashTable,
};
use crate::lib::compress::zstd_fast::{
    ZSTD_compressBlock_fast, ZSTD_compressBlock_fast_dictMatchState,
    ZSTD_compressBlock_fast_extDict, ZSTD_fillHashTable,
};
use crate::lib::compress::zstd_lazy::{
    ZSTD_compressBlock_btlazy2, ZSTD_compressBlock_btlazy2_dictMatchState,
    ZSTD_compressBlock_btlazy2_extDict, ZSTD_compressBlock_greedy,
    ZSTD_compressBlock_greedy_dedicatedDictSearch,
    ZSTD_compressBlock_greedy_dedicatedDictSearch_row, ZSTD_compressBlock_greedy_dictMatchState,
    ZSTD_compressBlock_greedy_dictMatchState_row, ZSTD_compressBlock_greedy_extDict,
    ZSTD_compressBlock_greedy_extDict_row, ZSTD_compressBlock_greedy_row, ZSTD_compressBlock_lazy,
    ZSTD_compressBlock_lazy2, ZSTD_compressBlock_lazy2_dedicatedDictSearch,
    ZSTD_compressBlock_lazy2_dedicatedDictSearch_row, ZSTD_compressBlock_lazy2_dictMatchState,
    ZSTD_compressBlock_lazy2_dictMatchState_row, ZSTD_compressBlock_lazy2_extDict,
    ZSTD_compressBlock_lazy2_extDict_row, ZSTD_compressBlock_lazy2_row,
    ZSTD_compressBlock_lazy_dedicatedDictSearch, ZSTD_compressBlock_lazy_dedicatedDictSearch_row,
    ZSTD_compressBlock_lazy_dictMatchState, ZSTD_compressBlock_lazy_dictMatchState_row,
    ZSTD_compressBlock_lazy_extDict, ZSTD_compressBlock_lazy_extDict_row,
    ZSTD_compressBlock_lazy_row, ZSTD_dedicatedDictSearch_lazy_loadDictionary,
    ZSTD_insertAndFindFirstIndex, ZSTD_row_update,
};
use crate::lib::compress::zstd_ldm::{
    ldmEntry_t, ldmMatchCandidate_t, ldmParams_t, ldmState_t, ZSTD_ldm_adjustParameters,
    ZSTD_ldm_blockCompress, ZSTD_ldm_fillHashTable, ZSTD_ldm_generateSequences,
    ZSTD_ldm_getMaxNbSeq, ZSTD_ldm_getTableSize, ZSTD_ldm_skipRawSeqStoreBytes,
    ZSTD_ldm_skipSequences,
};
use crate::lib::compress::zstd_opt::{
    ZSTD_compressBlock_btopt, ZSTD_compressBlock_btopt_dictMatchState,
    ZSTD_compressBlock_btopt_extDict, ZSTD_compressBlock_btultra, ZSTD_compressBlock_btultra2,
    ZSTD_compressBlock_btultra_dictMatchState, ZSTD_compressBlock_btultra_extDict, ZSTD_updateTree,
};
use crate::lib::compress::zstd_preSplit::ZSTD_splitBlock;
use crate::lib::compress::zstdmt_compress::{
    ZSTDMT_CCtx, ZSTDMT_compressStream_generic, ZSTDMT_createCCtx_advanced, ZSTDMT_freeCCtx,
    ZSTDMT_getFrameProgression, ZSTDMT_initCStream_internal, ZSTDMT_nextInputSizeHint,
    ZSTDMT_sizeof_CCtx, ZSTDMT_toFlushNow, ZSTDMT_updateCParams_whileCompressing,
};
use crate::lib::zstd::*;
pub const ZSTD_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub const MIN_CBLOCK_SIZE: core::ffi::c_int = 1 + 1;
pub const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
pub const ZSTD_CWKSP_ALIGNMENT_BYTES: core::ffi::c_int = 64;
#[inline]
unsafe fn ZSTD_cwksp_assert_internal_consistency(ws: *mut ZSTD_cwksp) {
    assert!((*ws).workspace <= (*ws).objectEnd);
    assert!((*ws).objectEnd <= (*ws).tableEnd);
    assert!((*ws).objectEnd <= (*ws).tableValidEnd);
    assert!((*ws).tableEnd <= (*ws).allocStart);
    assert!((*ws).tableValidEnd <= (*ws).allocStart);
    assert!((*ws).allocStart <= (*ws).workspaceEnd);
    assert!((*ws).initOnceStart <= ZSTD_cwksp_initialAllocStart(ws));
    assert!((*ws).workspace <= (*ws).initOnceStart);
}
#[inline]
unsafe fn ZSTD_cwksp_align(size: size_t, align: size_t) -> size_t {
    let mask = align.wrapping_sub(1);
    size.wrapping_add(mask) & !mask
}
#[inline]
unsafe fn ZSTD_cwksp_alloc_size(size: size_t) -> size_t {
    if size == 0 {
        return 0;
    }
    size
}
#[inline]
unsafe fn ZSTD_cwksp_aligned_alloc_size(size: size_t, alignment: size_t) -> size_t {
    ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(size, alignment))
}
#[inline]
unsafe fn ZSTD_cwksp_aligned64_alloc_size(size: size_t) -> size_t {
    ZSTD_cwksp_aligned_alloc_size(size, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t)
}
#[inline]
unsafe fn ZSTD_cwksp_slack_space_required() -> size_t {
    (ZSTD_CWKSP_ALIGNMENT_BYTES * 2) as size_t
}
#[inline]
unsafe fn ZSTD_cwksp_bytes_to_align_ptr(ptr: *mut core::ffi::c_void, alignBytes: size_t) -> size_t {
    let alignBytesMask = alignBytes.wrapping_sub(1);

    alignBytes.wrapping_sub(ptr as size_t & alignBytesMask) & alignBytesMask
}
#[inline]
unsafe fn ZSTD_cwksp_initialAllocStart(ws: *mut ZSTD_cwksp) -> *mut core::ffi::c_void {
    let mut endPtr = (*ws).workspaceEnd as *mut core::ffi::c_char;
    endPtr = endPtr.offset(-((endPtr as size_t % ZSTD_CWKSP_ALIGNMENT_BYTES as size_t) as isize));
    endPtr as *mut core::ffi::c_void
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_internal_buffer_space(
    ws: *mut ZSTD_cwksp,
    bytes: size_t,
) -> *mut core::ffi::c_void {
    let alloc = ((*ws).allocStart as *mut u8).offset(-(bytes as isize)) as *mut core::ffi::c_void;
    let bottom = (*ws).tableEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
    if alloc < bottom {
        (*ws).allocFailed = 1;
        return core::ptr::null_mut();
    }
    if alloc < (*ws).tableValidEnd {
        (*ws).tableValidEnd = alloc;
    }
    (*ws).allocStart = alloc;
    alloc
}
#[inline]
unsafe fn ZSTD_cwksp_internal_advance_phase(
    ws: *mut ZSTD_cwksp,
    phase: ZSTD_cwksp_alloc_phase_e,
) -> size_t {
    if phase as core::ffi::c_uint > (*ws).phase as core::ffi::c_uint {
        if ((*ws).phase as core::ffi::c_uint)
            < ZSTD_cwksp_alloc_aligned_init_once as core::ffi::c_int as core::ffi::c_uint
            && phase as core::ffi::c_uint
                >= ZSTD_cwksp_alloc_aligned_init_once as core::ffi::c_int as core::ffi::c_uint
        {
            (*ws).tableValidEnd = (*ws).objectEnd;
            (*ws).initOnceStart = ZSTD_cwksp_initialAllocStart(ws);
            let alloc = (*ws).objectEnd;
            let bytesToAlign =
                ZSTD_cwksp_bytes_to_align_ptr(alloc, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t);
            let objectEnd = (alloc as *mut u8).add(bytesToAlign) as *mut core::ffi::c_void;
            if objectEnd > (*ws).workspaceEnd {
                return Error::memory_allocation.to_error_code();
            }
            (*ws).objectEnd = objectEnd;
            (*ws).tableEnd = objectEnd;
            if (*ws).tableValidEnd < (*ws).tableEnd {
                (*ws).tableValidEnd = (*ws).tableEnd;
            }
        }
        (*ws).phase = phase;
        ZSTD_cwksp_assert_internal_consistency(ws);
    }
    0
}
#[inline]
unsafe fn ZSTD_cwksp_owns_buffer(
    ws: *const ZSTD_cwksp,
    ptr: *const core::ffi::c_void,
) -> core::ffi::c_int {
    core::ffi::c_int::from(
        !ptr.is_null()
            && (*ws).workspace <= ptr as *mut core::ffi::c_void
            && ptr < (*ws).workspaceEnd as *const core::ffi::c_void,
    )
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_internal(
    ws: *mut ZSTD_cwksp,
    bytes: size_t,
    phase: ZSTD_cwksp_alloc_phase_e,
) -> *mut core::ffi::c_void {
    let mut alloc = core::ptr::null_mut::<core::ffi::c_void>();
    if ERR_isError(ZSTD_cwksp_internal_advance_phase(ws, phase)) || bytes == 0 {
        return core::ptr::null_mut();
    }
    alloc = ZSTD_cwksp_reserve_internal_buffer_space(ws, bytes);
    alloc
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_buffer(ws: *mut ZSTD_cwksp, bytes: size_t) -> *mut u8 {
    ZSTD_cwksp_reserve_internal(ws, bytes, ZSTD_cwksp_alloc_buffers) as *mut u8
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_aligned_init_once(
    ws: *mut ZSTD_cwksp,
    bytes: size_t,
) -> *mut core::ffi::c_void {
    let alignedBytes = ZSTD_cwksp_align(bytes, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t);
    let ptr = ZSTD_cwksp_reserve_internal(ws, alignedBytes, ZSTD_cwksp_alloc_aligned_init_once);
    if !ptr.is_null() && ptr < (*ws).initOnceStart {
        ptr::write_bytes(
            ptr as *mut u8,
            0,
            (if (((*ws).initOnceStart as *mut u8).offset_from(ptr as *mut u8) as core::ffi::c_long
                as size_t)
                < alignedBytes
            {
                ((*ws).initOnceStart as *mut u8).offset_from(ptr as *mut u8) as core::ffi::c_long
                    as size_t
            } else {
                alignedBytes
            }) as libc::size_t,
        );
        (*ws).initOnceStart = ptr;
    }
    ptr
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_aligned64(
    ws: *mut ZSTD_cwksp,
    bytes: size_t,
) -> *mut core::ffi::c_void {
    ZSTD_cwksp_reserve_internal(
        ws,
        ZSTD_cwksp_align(bytes, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t),
        ZSTD_cwksp_alloc_aligned,
    )
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_table(ws: *mut ZSTD_cwksp, bytes: size_t) -> *mut core::ffi::c_void {
    let phase = ZSTD_cwksp_alloc_aligned_init_once;
    let mut alloc = core::ptr::null_mut::<core::ffi::c_void>();
    let mut end = core::ptr::null_mut::<core::ffi::c_void>();
    let mut top = core::ptr::null_mut::<core::ffi::c_void>();
    if ((*ws).phase as core::ffi::c_uint) < phase as core::ffi::c_uint
        && ERR_isError(ZSTD_cwksp_internal_advance_phase(ws, phase))
    {
        return core::ptr::null_mut();
    }
    alloc = (*ws).tableEnd;
    end = (alloc as *mut u8).add(bytes) as *mut core::ffi::c_void;
    top = (*ws).allocStart;
    ZSTD_cwksp_assert_internal_consistency(ws);
    if end > top {
        (*ws).allocFailed = 1;
        return core::ptr::null_mut();
    }
    (*ws).tableEnd = end;
    alloc
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_object(ws: *mut ZSTD_cwksp, bytes: size_t) -> *mut core::ffi::c_void {
    let roundedBytes = ZSTD_cwksp_align(bytes, ::core::mem::size_of::<*mut core::ffi::c_void>());
    let alloc = (*ws).objectEnd;
    let end = (alloc as *mut u8).add(roundedBytes) as *mut core::ffi::c_void;
    ZSTD_cwksp_assert_internal_consistency(ws);
    if (*ws).phase as core::ffi::c_uint
        != ZSTD_cwksp_alloc_objects as core::ffi::c_int as core::ffi::c_uint
        || end > (*ws).workspaceEnd
    {
        (*ws).allocFailed = 1;
        return core::ptr::null_mut();
    }
    (*ws).objectEnd = end;
    (*ws).tableEnd = end;
    (*ws).tableValidEnd = end;
    alloc
}
#[inline]
unsafe fn ZSTD_cwksp_mark_tables_dirty(ws: *mut ZSTD_cwksp) {
    (*ws).tableValidEnd = (*ws).objectEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe fn ZSTD_cwksp_mark_tables_clean(ws: *mut ZSTD_cwksp) {
    if (*ws).tableValidEnd < (*ws).tableEnd {
        (*ws).tableValidEnd = (*ws).tableEnd;
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe fn ZSTD_cwksp_clean_tables(ws: *mut ZSTD_cwksp) {
    if (*ws).tableValidEnd < (*ws).tableEnd {
        ptr::write_bytes(
            (*ws).tableValidEnd,
            0,
            ((*ws).tableEnd as *mut u8).offset_from((*ws).tableValidEnd as *mut u8) as usize,
        );
    }
    ZSTD_cwksp_mark_tables_clean(ws);
}
#[inline]
unsafe fn ZSTD_cwksp_clear_tables(ws: *mut ZSTD_cwksp) {
    (*ws).tableEnd = (*ws).objectEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe fn ZSTD_cwksp_clear(ws: *mut ZSTD_cwksp) {
    (*ws).tableEnd = (*ws).objectEnd;
    (*ws).allocStart = ZSTD_cwksp_initialAllocStart(ws);
    (*ws).allocFailed = 0;
    if (*ws).phase as core::ffi::c_uint
        > ZSTD_cwksp_alloc_aligned_init_once as core::ffi::c_int as core::ffi::c_uint
    {
        (*ws).phase = ZSTD_cwksp_alloc_aligned_init_once;
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe fn ZSTD_cwksp_sizeof(ws: *const ZSTD_cwksp) -> size_t {
    ((*ws).workspaceEnd as *mut u8).offset_from((*ws).workspace as *mut u8) as core::ffi::c_long
        as size_t
}
#[inline]
unsafe fn ZSTD_cwksp_init(
    ws: *mut ZSTD_cwksp,
    start: *mut core::ffi::c_void,
    size: size_t,
    isStatic: ZSTD_cwksp_static_alloc_e,
) {
    (*ws).workspace = start;
    (*ws).workspaceEnd = (start as *mut u8).add(size) as *mut core::ffi::c_void;
    (*ws).objectEnd = (*ws).workspace;
    (*ws).tableValidEnd = (*ws).objectEnd;
    (*ws).initOnceStart = ZSTD_cwksp_initialAllocStart(ws);
    (*ws).phase = ZSTD_cwksp_alloc_objects;
    (*ws).isStatic = isStatic;
    ZSTD_cwksp_clear(ws);
    (*ws).workspaceOversizedDuration = 0;
    ZSTD_cwksp_assert_internal_consistency(ws);
}
#[inline]
unsafe fn ZSTD_cwksp_create(
    ws: *mut ZSTD_cwksp,
    size: size_t,
    customMem: ZSTD_customMem,
) -> size_t {
    let workspace = ZSTD_customMalloc(size, customMem);
    if workspace.is_null() {
        return Error::memory_allocation.to_error_code();
    }
    ZSTD_cwksp_init(ws, workspace, size, ZSTD_cwksp_dynamic_alloc);
    0
}
#[inline]
unsafe fn ZSTD_cwksp_free(ws: *mut ZSTD_cwksp, customMem: ZSTD_customMem) {
    let ptr = (*ws).workspace;
    let size = (*ws)
        .workspaceEnd
        .byte_offset_from_unsigned((*ws).workspace);
    ptr::write_bytes(ws as *mut u8, 0, ::core::mem::size_of::<ZSTD_cwksp>());
    ZSTD_customFree(ptr, size, customMem);
}
#[inline]
unsafe fn ZSTD_cwksp_move(dst: *mut ZSTD_cwksp, src: *mut ZSTD_cwksp) {
    *dst = *src;
    ptr::write_bytes(src as *mut u8, 0, ::core::mem::size_of::<ZSTD_cwksp>());
}
#[inline]
unsafe fn ZSTD_cwksp_reserve_failed(ws: *const ZSTD_cwksp) -> core::ffi::c_int {
    core::ffi::c_int::from((*ws).allocFailed)
}
#[inline]
unsafe fn ZSTD_cwksp_available_space(ws: *mut ZSTD_cwksp) -> size_t {
    ((*ws).allocStart as *mut u8).offset_from((*ws).tableEnd as *mut u8) as core::ffi::c_long
        as size_t
}
#[inline]
unsafe fn ZSTD_cwksp_check_available(
    ws: *mut ZSTD_cwksp,
    additionalNeededSpace: size_t,
) -> core::ffi::c_int {
    core::ffi::c_int::from(ZSTD_cwksp_available_space(ws) >= additionalNeededSpace)
}
#[inline]
unsafe fn ZSTD_cwksp_check_too_large(
    ws: *mut ZSTD_cwksp,
    additionalNeededSpace: size_t,
) -> core::ffi::c_int {
    ZSTD_cwksp_check_available(
        ws,
        additionalNeededSpace * ZSTD_WORKSPACETOOLARGE_FACTOR as size_t,
    )
}
#[inline]
unsafe fn ZSTD_cwksp_check_wasteful(
    ws: *mut ZSTD_cwksp,
    additionalNeededSpace: size_t,
) -> core::ffi::c_int {
    core::ffi::c_int::from(
        ZSTD_cwksp_check_too_large(ws, additionalNeededSpace) != 0
            && (*ws).workspaceOversizedDuration > ZSTD_WORKSPACETOOLARGE_MAXDURATION,
    )
}
#[inline]
unsafe fn ZSTD_cwksp_bump_oversized_duration(ws: *mut ZSTD_cwksp, additionalNeededSpace: size_t) {
    if ZSTD_cwksp_check_too_large(ws, additionalNeededSpace) != 0 {
        (*ws).workspaceOversizedDuration += 1;
    } else {
        (*ws).workspaceOversizedDuration = 0;
    };
}
pub const ZSTDMT_JOBSIZE_MIN: core::ffi::c_int = 512 * ((1) << 10);

pub const STREAM_ACCUMULATOR_MIN_32: core::ffi::c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: core::ffi::c_int = 57;
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_doubleFast;
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_DICTMATCHSTATE: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_doubleFast_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_DOUBLEFAST_EXTDICT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_doubleFast_extDict;
pub const ZSTD_LAZY_DDSS_BUCKET_LOG: core::ffi::c_int = 2;
pub const ZSTD_ROW_HASH_TAG_BITS: core::ffi::c_int = 8;
pub const ZSTD_COMPRESSBLOCK_GREEDY: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy;
pub const ZSTD_COMPRESSBLOCK_GREEDY_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy_row;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy_dictMatchState_row;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy_dedicatedDictSearch;
pub const ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy_dedicatedDictSearch_row;
pub const ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy_extDict;
pub const ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_greedy_extDict_row;
pub const ZSTD_COMPRESSBLOCK_LAZY: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy;
pub const ZSTD_COMPRESSBLOCK_LAZY_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy_row;
pub const ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy_dictMatchState_row;
pub const ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy_dedicatedDictSearch;
pub const ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy_dedicatedDictSearch_row;
pub const ZSTD_COMPRESSBLOCK_LAZY_EXTDICT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy_extDict;
pub const ZSTD_COMPRESSBLOCK_LAZY_EXTDICT_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy_extDict_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2;
pub const ZSTD_COMPRESSBLOCK_LAZY2_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2_dictMatchState_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2_dedicatedDictSearch;
pub const ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2_dedicatedDictSearch_row;
pub const ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2_extDict;
pub const ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT_ROW: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_lazy2_extDict_row;
pub const ZSTD_COMPRESSBLOCK_BTLAZY2: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btlazy2;
pub const ZSTD_COMPRESSBLOCK_BTLAZY2_DICTMATCHSTATE: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btlazy2_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_BTLAZY2_EXTDICT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btlazy2_extDict;
pub const ZSTD_COMPRESSBLOCK_BTOPT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btopt;
pub const ZSTD_COMPRESSBLOCK_BTOPT_DICTMATCHSTATE: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btopt_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_BTOPT_EXTDICT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btopt_extDict;
pub const ZSTD_COMPRESSBLOCK_BTULTRA: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btultra;
pub const ZSTD_COMPRESSBLOCK_BTULTRA_DICTMATCHSTATE: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btultra_dictMatchState;
pub const ZSTD_COMPRESSBLOCK_BTULTRA_EXTDICT: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btultra_extDict;
pub const ZSTD_COMPRESSBLOCK_BTULTRA2: unsafe fn(
    &mut ZSTD_MatchState_t,
    &mut SeqStore_t,
    *mut u32,
    *const core::ffi::c_void,
    size_t,
) -> size_t = ZSTD_compressBlock_btultra2;
pub const ZSTD_LDM_DEFAULT_WINDOW_LOG: core::ffi::c_int = 27;
pub const INT_MAX: core::ffi::c_int = __INT_MAX__;
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBound))]
pub extern "C" fn ZSTD_compressBound(srcSize: size_t) -> size_t {
    let r = if srcSize as core::ffi::c_ulonglong
        >= (if ::core::mem::size_of::<size_t>() == 8 {
            0xff00ff00ff00ff00 as core::ffi::c_ulonglong
        } else {
            core::ffi::c_ulonglong::from(0xff00ff00 as core::ffi::c_uint)
        }) {
        0
    } else {
        srcSize
            .wrapping_add(srcSize >> 8)
            .wrapping_add(if srcSize < ((128) << 10) as size_t {
                (((128) << 10) as size_t).wrapping_sub(srcSize) >> 11
            } else {
                0
            })
    };
    if r == 0 {
        return Error::srcSize_wrong.to_error_code();
    }
    r
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCCtx))]
pub unsafe extern "C" fn ZSTD_createCCtx() -> *mut ZSTD_CCtx {
    ZSTD_createCCtx_advanced(ZSTD_customMem::default())
}
unsafe fn ZSTD_initCCtx(cctx: *mut ZSTD_CCtx, memManager: ZSTD_customMem) {
    ptr::write_bytes(cctx as *mut u8, 0, ::core::mem::size_of::<ZSTD_CCtx>());
    (*cctx).customMem = memManager;
    (*cctx).bmi2 = ZSTD_cpuSupportsBmi2().into();
    let _err = ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_parameters);
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCCtx_advanced))]
pub unsafe extern "C" fn ZSTD_createCCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_CCtx {
    let cctx = ZSTD_customMalloc(::core::mem::size_of::<ZSTD_CCtx>(), customMem) as *mut ZSTD_CCtx;
    if cctx.is_null() {
        return core::ptr::null_mut();
    }
    ZSTD_initCCtx(cctx, customMem);
    cctx
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initStaticCCtx))]
pub unsafe extern "C" fn ZSTD_initStaticCCtx(
    workspace: *mut core::ffi::c_void,
    workspaceSize: size_t,
) -> *mut ZSTD_CCtx {
    let mut ws = ZSTD_cwksp {
        workspace: core::ptr::null_mut::<core::ffi::c_void>(),
        workspaceEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        objectEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        tableEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        tableValidEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        allocStart: core::ptr::null_mut::<core::ffi::c_void>(),
        initOnceStart: core::ptr::null_mut::<core::ffi::c_void>(),
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    let mut cctx = core::ptr::null_mut::<ZSTD_CCtx>();
    if workspaceSize <= ::core::mem::size_of::<ZSTD_CCtx>() {
        return core::ptr::null_mut();
    }
    if workspace as size_t & 7 != 0 {
        return core::ptr::null_mut();
    }
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_static_alloc);
    cctx =
        ZSTD_cwksp_reserve_object(&mut ws, ::core::mem::size_of::<ZSTD_CCtx>()) as *mut ZSTD_CCtx;
    if cctx.is_null() {
        return core::ptr::null_mut();
    }
    ptr::write_bytes(cctx as *mut u8, 0, ::core::mem::size_of::<ZSTD_CCtx>());
    ZSTD_cwksp_move(&mut (*cctx).workspace, &mut ws);
    (*cctx).staticSize = workspaceSize;
    if ZSTD_cwksp_check_available(
        &mut (*cctx).workspace,
        (if ((((8) << 10) + 512) as size_t).wrapping_add(
            (::core::mem::size_of::<core::ffi::c_uint>())
                .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
        ) > 8208
        {
            ((((8) << 10) + 512) as size_t).wrapping_add(
                (::core::mem::size_of::<core::ffi::c_uint>())
                    .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
            )
        } else {
            8208
        })
        .wrapping_add(
            (2 as size_t).wrapping_mul(::core::mem::size_of::<ZSTD_compressedBlockState_t>()),
        ),
    ) == 0
    {
        return core::ptr::null_mut();
    }
    (*cctx).blockState.prevCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>(),
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx).blockState.nextCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>(),
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx).tmpWorkspace = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        if ((((8) << 10) + 512) as size_t).wrapping_add(
            (::core::mem::size_of::<core::ffi::c_uint>())
                .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
        ) > 8208
        {
            ((((8) << 10) + 512) as size_t).wrapping_add(
                (::core::mem::size_of::<core::ffi::c_uint>())
                    .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
            )
        } else {
            8208
        },
    );
    (*cctx).tmpWkspSize = if ((((8) << 10) + 512) as size_t).wrapping_add(
        (::core::mem::size_of::<core::ffi::c_uint>())
            .wrapping_mul(((if 35 > 52 as core::ffi::c_int { 35 } else { 52 }) + 2) as size_t),
    ) > 8208 as size_t
    {
        ((((8) << 10) + 512) as size_t).wrapping_add(
            (::core::mem::size_of::<core::ffi::c_uint>())
                .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
        )
    } else {
        8208
    };
    (*cctx).bmi2 = ZSTD_cpuSupportsBmi2().into();
    cctx
}
unsafe fn ZSTD_clearAllDicts(cctx: *mut ZSTD_CCtx) {
    ZSTD_customFree(
        (*cctx).localDict.dictBuffer,
        (*cctx).localDict.dictSize,
        (*cctx).customMem,
    );
    ZSTD_freeCDict((*cctx).localDict.cdict);
    ptr::write_bytes(
        &mut (*cctx).localDict as *mut ZSTD_localDict as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_localDict>(),
    );
    ptr::write_bytes(
        &mut (*cctx).prefixDict as *mut ZSTD_prefixDict as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_prefixDict>(),
    );
    (*cctx).cdict = core::ptr::null();
}
unsafe fn ZSTD_sizeof_localDict(dict: ZSTD_localDict) -> size_t {
    let bufferSize = if !(dict.dictBuffer).is_null() {
        dict.dictSize
    } else {
        0
    };
    let cdictSize = ZSTD_sizeof_CDict(dict.cdict);
    bufferSize.wrapping_add(cdictSize)
}
unsafe fn ZSTD_freeCCtxContent(cctx: *mut ZSTD_CCtx) {
    ZSTD_clearAllDicts(cctx);
    ZSTDMT_freeCCtx((*cctx).mtctx);
    (*cctx).mtctx = core::ptr::null_mut();
    ZSTD_cwksp_free(&mut (*cctx).workspace, (*cctx).customMem);
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeCCtx))]
pub unsafe extern "C" fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t {
    if cctx.is_null() {
        return 0;
    }
    if (*cctx).staticSize != 0 {
        return Error::memory_allocation.to_error_code();
    }
    let cctxInWorkspace =
        ZSTD_cwksp_owns_buffer(&(*cctx).workspace, cctx as *const core::ffi::c_void);
    ZSTD_freeCCtxContent(cctx);
    if cctxInWorkspace == 0 {
        ZSTD_customFree(
            cctx as *mut core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_CCtx>(),
            (*cctx).customMem,
        );
    }
    0
}
unsafe fn ZSTD_sizeof_mtctx(cctx: *const ZSTD_CCtx) -> size_t {
    ZSTDMT_sizeof_CCtx((*cctx).mtctx)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_CCtx))]
pub unsafe extern "C" fn ZSTD_sizeof_CCtx(cctx: *const ZSTD_CCtx) -> size_t {
    if cctx.is_null() {
        return 0;
    }
    (if (*cctx).workspace.workspace == cctx as *mut core::ffi::c_void {
        0
    } else {
        ::core::mem::size_of::<ZSTD_CCtx>()
    })
    .wrapping_add(ZSTD_cwksp_sizeof(&(*cctx).workspace))
    .wrapping_add(ZSTD_sizeof_localDict((*cctx).localDict))
    .wrapping_add(ZSTD_sizeof_mtctx(cctx))
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_CStream))]
pub unsafe extern "C" fn ZSTD_sizeof_CStream(zcs: *const ZSTD_CStream) -> size_t {
    ZSTD_sizeof_CCtx(zcs)
}
pub unsafe fn ZSTD_getSeqStore(ctx: *const ZSTD_CCtx) -> *const SeqStore_t {
    &(*ctx).seqStore
}
unsafe fn ZSTD_rowMatchFinderSupported(strategy: ZSTD_strategy) -> core::ffi::c_int {
    core::ffi::c_int::from(
        strategy as core::ffi::c_uint >= ZSTD_greedy as core::ffi::c_int as core::ffi::c_uint
            && strategy as core::ffi::c_uint <= ZSTD_lazy2 as core::ffi::c_int as core::ffi::c_uint,
    )
}
unsafe fn ZSTD_rowMatchFinderUsed(
    strategy: ZSTD_strategy,
    mode: ZSTD_ParamSwitch_e,
) -> core::ffi::c_int {
    core::ffi::c_int::from(
        ZSTD_rowMatchFinderSupported(strategy) != 0 && mode == ZSTD_ParamSwitch_e::ZSTD_ps_enable,
    )
}
unsafe fn ZSTD_resolveRowMatchFinderMode(
    mut mode: ZSTD_ParamSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_ParamSwitch_e {
    let kWindowLogLowerBound = 14;
    if mode != ZSTD_ParamSwitch_e::ZSTD_ps_auto {
        return mode;
    }
    mode = ZSTD_ParamSwitch_e::ZSTD_ps_disable;
    if ZSTD_rowMatchFinderSupported((*cParams).strategy) == 0 {
        return mode;
    }
    if (*cParams).windowLog > kWindowLogLowerBound {
        mode = ZSTD_ParamSwitch_e::ZSTD_ps_enable;
    }
    mode
}
unsafe fn ZSTD_resolveBlockSplitterMode(
    mode: ZSTD_ParamSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_ParamSwitch_e {
    if mode != ZSTD_ParamSwitch_e::ZSTD_ps_auto {
        return mode;
    }
    if (*cParams).strategy as core::ffi::c_uint
        >= ZSTD_btopt as core::ffi::c_int as core::ffi::c_uint
        && (*cParams).windowLog >= 17
    {
        ZSTD_ParamSwitch_e::ZSTD_ps_enable
    } else {
        ZSTD_ParamSwitch_e::ZSTD_ps_disable
    }
}
unsafe fn ZSTD_allocateChainTable(
    strategy: ZSTD_strategy,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    forDDSDict: u32,
) -> core::ffi::c_int {
    core::ffi::c_int::from(
        forDDSDict != 0
            || strategy as core::ffi::c_uint != ZSTD_fast as core::ffi::c_int as core::ffi::c_uint
                && ZSTD_rowMatchFinderUsed(strategy, useRowMatchFinder) == 0,
    )
}
unsafe fn ZSTD_resolveEnableLdm(
    mode: ZSTD_ParamSwitch_e,
    cParams: *const ZSTD_compressionParameters,
) -> ZSTD_ParamSwitch_e {
    if mode != ZSTD_ParamSwitch_e::ZSTD_ps_auto {
        return mode;
    }
    if (*cParams).strategy as core::ffi::c_uint
        >= ZSTD_btopt as core::ffi::c_int as core::ffi::c_uint
        && (*cParams).windowLog >= 27
    {
        ZSTD_ParamSwitch_e::ZSTD_ps_enable
    } else {
        ZSTD_ParamSwitch_e::ZSTD_ps_disable
    }
}
unsafe fn ZSTD_resolveExternalSequenceValidation(mode: core::ffi::c_int) -> core::ffi::c_int {
    mode
}
unsafe fn ZSTD_resolveMaxBlockSize(maxBlockSize: size_t) -> size_t {
    if maxBlockSize == 0 {
        ZSTD_BLOCKSIZE_MAX as size_t
    } else {
        maxBlockSize
    }
}
unsafe fn ZSTD_resolveExternalRepcodeSearch(
    value: ZSTD_ParamSwitch_e,
    cLevel: core::ffi::c_int,
) -> ZSTD_ParamSwitch_e {
    if value != ZSTD_ParamSwitch_e::ZSTD_ps_auto {
        return value;
    }
    if cLevel < 10 {
        ZSTD_ParamSwitch_e::ZSTD_ps_disable
    } else {
        ZSTD_ParamSwitch_e::ZSTD_ps_enable
    }
}
unsafe fn ZSTD_CDictIndicesAreTagged(
    cParams: *const ZSTD_compressionParameters,
) -> core::ffi::c_int {
    core::ffi::c_int::from(
        (*cParams).strategy as core::ffi::c_uint
            == ZSTD_fast as core::ffi::c_int as core::ffi::c_uint
            || (*cParams).strategy as core::ffi::c_uint
                == ZSTD_dfast as core::ffi::c_int as core::ffi::c_uint,
    )
}
unsafe fn ZSTD_makeCCtxParamsFromCParams(cParams: ZSTD_compressionParameters) -> ZSTD_CCtx_params {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: Format::ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
        literalCompressionMode: ParamSwitch::Auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    };
    ZSTD_CCtxParams_init(&mut cctxParams, ZSTD_CLEVEL_DEFAULT);
    cctxParams.cParams = cParams;
    cctxParams.ldmParams.enableLdm =
        ZSTD_resolveEnableLdm(cctxParams.ldmParams.enableLdm, &cParams);
    if cctxParams.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        ZSTD_ldm_adjustParameters(&mut cctxParams.ldmParams, &cParams);
    }
    cctxParams.postBlockSplitter =
        ZSTD_resolveBlockSplitterMode(cctxParams.postBlockSplitter, &cParams);
    cctxParams.useRowMatchFinder =
        ZSTD_resolveRowMatchFinderMode(cctxParams.useRowMatchFinder, &cParams);
    cctxParams.validateSequences =
        ZSTD_resolveExternalSequenceValidation(cctxParams.validateSequences);
    cctxParams.maxBlockSize = ZSTD_resolveMaxBlockSize(cctxParams.maxBlockSize);
    cctxParams.searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        cctxParams.searchForExternalRepcodes,
        cctxParams.compressionLevel,
    );
    cctxParams
}
unsafe fn ZSTD_createCCtxParams_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_CCtx_params {
    let params = ZSTD_customCalloc(::core::mem::size_of::<ZSTD_CCtx_params>(), customMem)
        as *mut ZSTD_CCtx_params;
    if params.is_null() {
        return core::ptr::null_mut();
    }
    ZSTD_CCtxParams_init(params, ZSTD_CLEVEL_DEFAULT);
    (*params).customMem = customMem;
    params
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCCtxParams))]
pub unsafe extern "C" fn ZSTD_createCCtxParams() -> *mut ZSTD_CCtx_params {
    ZSTD_createCCtxParams_advanced(ZSTD_customMem::default())
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeCCtxParams))]
pub unsafe extern "C" fn ZSTD_freeCCtxParams(params: *mut ZSTD_CCtx_params) -> size_t {
    if params.is_null() {
        return 0;
    }
    ZSTD_customFree(
        params as *mut core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_CCtx_params>(),
        (*params).customMem,
    );
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtxParams_reset))]
pub unsafe extern "C" fn ZSTD_CCtxParams_reset(params: *mut ZSTD_CCtx_params) -> size_t {
    ZSTD_CCtxParams_init(params, ZSTD_CLEVEL_DEFAULT)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtxParams_init))]
pub unsafe extern "C" fn ZSTD_CCtxParams_init(
    cctxParams: *mut ZSTD_CCtx_params,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    if cctxParams.is_null() {
        return Error::GENERIC.to_error_code();
    }
    ptr::write_bytes(
        cctxParams as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_CCtx_params>(),
    );
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams).fParams.contentSizeFlag = 1;
    0
}
pub const ZSTD_NO_CLEVEL: core::ffi::c_int = 0;
unsafe fn ZSTD_CCtxParams_init_internal(
    cctxParams: *mut ZSTD_CCtx_params,
    params: *const ZSTD_parameters,
    compressionLevel: core::ffi::c_int,
) {
    ptr::write_bytes(
        cctxParams as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_CCtx_params>(),
    );
    (*cctxParams).cParams = (*params).cParams;
    (*cctxParams).fParams = (*params).fParams;
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams).useRowMatchFinder =
        ZSTD_resolveRowMatchFinderMode((*cctxParams).useRowMatchFinder, &(*params).cParams);
    (*cctxParams).postBlockSplitter =
        ZSTD_resolveBlockSplitterMode((*cctxParams).postBlockSplitter, &(*params).cParams);
    (*cctxParams).ldmParams.enableLdm =
        ZSTD_resolveEnableLdm((*cctxParams).ldmParams.enableLdm, &(*params).cParams);
    (*cctxParams).validateSequences =
        ZSTD_resolveExternalSequenceValidation((*cctxParams).validateSequences);
    (*cctxParams).maxBlockSize = ZSTD_resolveMaxBlockSize((*cctxParams).maxBlockSize);
    (*cctxParams).searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        (*cctxParams).searchForExternalRepcodes,
        compressionLevel,
    );
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtxParams_init_advanced))]
pub unsafe extern "C" fn ZSTD_CCtxParams_init_advanced(
    cctxParams: *mut ZSTD_CCtx_params,
    params: ZSTD_parameters,
) -> size_t {
    if cctxParams.is_null() {
        return Error::GENERIC.to_error_code();
    }
    let err_code = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_CCtxParams_init_internal(cctxParams, &params, ZSTD_NO_CLEVEL);
    0
}
unsafe fn ZSTD_CCtxParams_setZstdParams(
    cctxParams: *mut ZSTD_CCtx_params,
    params: *const ZSTD_parameters,
) {
    (*cctxParams).cParams = (*params).cParams;
    (*cctxParams).fParams = (*params).fParams;
    (*cctxParams).compressionLevel = ZSTD_NO_CLEVEL;
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_cParam_getBounds))]
pub unsafe extern "C" fn ZSTD_cParam_getBounds(param: ZSTD_cParameter) -> ZSTD_bounds {
    let mut bounds = {
        ZSTD_bounds {
            error: 0,
            lowerBound: 0,
            upperBound: 0,
        }
    };
    match param.0 {
        100 => {
            bounds.lowerBound = ZSTD_minCLevel();
            bounds.upperBound = ZSTD_maxCLevel();
            bounds
        }
        101 => {
            bounds.lowerBound = ZSTD_WINDOWLOG_MIN;
            bounds.upperBound = if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            };
            bounds
        }
        102 => {
            bounds.lowerBound = ZSTD_HASHLOG_MIN;
            bounds.upperBound = if (if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) < 30
            {
                if ::core::mem::size_of::<size_t>() == 4 {
                    ZSTD_WINDOWLOG_MAX_32
                } else {
                    ZSTD_WINDOWLOG_MAX_64
                }
            } else {
                30
            };
            bounds
        }
        103 => {
            bounds.lowerBound = ZSTD_CHAINLOG_MIN;
            bounds.upperBound = if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_CHAINLOG_MAX_32
            } else {
                ZSTD_CHAINLOG_MAX_64
            };
            bounds
        }
        104 => {
            bounds.lowerBound = ZSTD_SEARCHLOG_MIN;
            bounds.upperBound = (if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) - 1;
            bounds
        }
        105 => {
            bounds.lowerBound = ZSTD_MINMATCH_MIN;
            bounds.upperBound = ZSTD_MINMATCH_MAX;
            bounds
        }
        106 => {
            bounds.lowerBound = ZSTD_TARGETLENGTH_MIN;
            bounds.upperBound = ZSTD_TARGETLENGTH_MAX;
            bounds
        }
        107 => {
            bounds.lowerBound = ZSTD_STRATEGY_MIN;
            bounds.upperBound = ZSTD_STRATEGY_MAX;
            bounds
        }
        200 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        201 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        202 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        400 => {
            bounds.lowerBound = 0;
            bounds.upperBound = if ::core::mem::size_of::<*mut core::ffi::c_void>() == 4 {
                64
            } else {
                256
            };
            bounds
        }
        401 => {
            bounds.lowerBound = 0;
            bounds.upperBound = if MEM_32bits() {
                512 * (1 << 20)
            } else {
                1024 * (1 << 20)
            };
            bounds
        }
        402 => {
            bounds.lowerBound = ZSTD_OVERLAPLOG_MIN;
            bounds.upperBound = ZSTD_OVERLAPLOG_MAX;
            bounds
        }
        1005 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        160 => {
            bounds.lowerBound = ZSTD_ParamSwitch_e::ZSTD_ps_auto as core::ffi::c_int;
            bounds.upperBound = ZSTD_ParamSwitch_e::ZSTD_ps_disable as core::ffi::c_int;
            bounds
        }
        161 => {
            bounds.lowerBound = ZSTD_LDM_HASHLOG_MIN;
            bounds.upperBound = if (if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) < 30
            {
                if ::core::mem::size_of::<size_t>() == 4 {
                    ZSTD_WINDOWLOG_MAX_32
                } else {
                    ZSTD_WINDOWLOG_MAX_64
                }
            } else {
                30
            };
            bounds
        }
        162 => {
            bounds.lowerBound = ZSTD_LDM_MINMATCH_MIN;
            bounds.upperBound = ZSTD_LDM_MINMATCH_MAX;
            bounds
        }
        163 => {
            bounds.lowerBound = ZSTD_LDM_BUCKETSIZELOG_MIN;
            bounds.upperBound = ZSTD_LDM_BUCKETSIZELOG_MAX;
            bounds
        }
        164 => {
            bounds.lowerBound = ZSTD_LDM_HASHRATELOG_MIN;
            bounds.upperBound = (if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) - ZSTD_HASHLOG_MIN;
            bounds
        }
        500 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        1000 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        10 => {
            bounds.lowerBound = Format::ZSTD_f_zstd1 as core::ffi::c_int;
            bounds.upperBound = Format::ZSTD_f_zstd1_magicless as core::ffi::c_int;
            bounds
        }
        1001 => {
            bounds.lowerBound = ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach.0 as core::ffi::c_int;
            bounds.upperBound = ZSTD_dictAttachPref_e::ZSTD_dictForceLoad.0 as core::ffi::c_int;
            bounds
        }
        1002 => {
            bounds.lowerBound = ZSTD_ParamSwitch_e::ZSTD_ps_auto as core::ffi::c_int;
            bounds.upperBound = ZSTD_ParamSwitch_e::ZSTD_ps_disable as core::ffi::c_int;
            bounds
        }
        130 => {
            bounds.lowerBound = ZSTD_TARGETCBLOCKSIZE_MIN;
            bounds.upperBound = ZSTD_TARGETCBLOCKSIZE_MAX;
            bounds
        }
        1004 => {
            bounds.lowerBound = ZSTD_SRCSIZEHINT_MIN;
            bounds.upperBound = ZSTD_SRCSIZEHINT_MAX;
            bounds
        }
        1006 | 1007 => {
            bounds.lowerBound = ZSTD_bm_buffered as core::ffi::c_int;
            bounds.upperBound = ZSTD_bm_stable as core::ffi::c_int;
            bounds
        }
        1008 => {
            bounds.lowerBound = ZSTD_sf_noBlockDelimiters as core::ffi::c_int;
            bounds.upperBound = ZSTD_sf_explicitBlockDelimiters as core::ffi::c_int;
            bounds
        }
        1009 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        1010 => {
            bounds.lowerBound = ZSTD_ParamSwitch_e::ZSTD_ps_auto as core::ffi::c_int;
            bounds.upperBound = ZSTD_ParamSwitch_e::ZSTD_ps_disable as core::ffi::c_int;
            bounds
        }
        1017 => {
            bounds.lowerBound = 0;
            bounds.upperBound = ZSTD_BLOCKSPLITTER_LEVEL_MAX;
            bounds
        }
        1011 => {
            bounds.lowerBound = ZSTD_ParamSwitch_e::ZSTD_ps_auto as core::ffi::c_int;
            bounds.upperBound = ZSTD_ParamSwitch_e::ZSTD_ps_disable as core::ffi::c_int;
            bounds
        }
        1012 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        1013 => {
            bounds.lowerBound = ZSTD_ParamSwitch_e::ZSTD_ps_auto as core::ffi::c_int;
            bounds.upperBound = ZSTD_ParamSwitch_e::ZSTD_ps_disable as core::ffi::c_int;
            bounds
        }
        1014 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        1015 => {
            bounds.lowerBound = ZSTD_BLOCKSIZE_MAX_MIN;
            bounds.upperBound = ZSTD_BLOCKSIZE_MAX;
            bounds
        }
        1016 => {
            bounds.lowerBound = ZSTD_ParamSwitch_e::ZSTD_ps_auto as core::ffi::c_int;
            bounds.upperBound = ZSTD_ParamSwitch_e::ZSTD_ps_disable as core::ffi::c_int;
            bounds
        }
        _ => {
            bounds.error = Error::parameter_unsupported.to_error_code();
            bounds
        }
    }
}
unsafe fn ZSTD_cParam_clampBounds(cParam: ZSTD_cParameter, value: *mut core::ffi::c_int) -> size_t {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) {
        return bounds.error;
    }
    if *value < bounds.lowerBound {
        *value = bounds.lowerBound;
    }
    if *value > bounds.upperBound {
        *value = bounds.upperBound;
    }
    0
}
unsafe fn ZSTD_isUpdateAuthorized(param: ZSTD_cParameter) -> core::ffi::c_int {
    match param {
        ZSTD_cParameter::ZSTD_c_compressionLevel
        | ZSTD_cParameter::ZSTD_c_hashLog
        | ZSTD_cParameter::ZSTD_c_chainLog
        | ZSTD_cParameter::ZSTD_c_searchLog
        | ZSTD_cParameter::ZSTD_c_minMatch
        | ZSTD_cParameter::ZSTD_c_targetLength
        | ZSTD_cParameter::ZSTD_c_strategy => 1,

        _ if param == ZSTD_cParameter::ZSTD_c_blockSplitterLevel => 1,

        _ => 0,
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setParameter))]
pub unsafe extern "C" fn ZSTD_CCtx_setParameter(
    cctx: *mut ZSTD_CCtx,
    param: ZSTD_cParameter,
    value: core::ffi::c_int,
) -> size_t {
    if (*cctx).streamStage != zcss_init {
        if ZSTD_isUpdateAuthorized(param) != 0 {
            (*cctx).cParamsChanged = 1;
        } else {
            return Error::stage_wrong.to_error_code();
        }
    }

    match param {
        ZSTD_cParameter::ZSTD_c_nbWorkers => {
            if value != 0 && (*cctx).staticSize != 0 {
                Error::parameter_unsupported.to_error_code()
            } else {
                ZSTD_CCtxParams_setParameter(&mut (*cctx).requestedParams, param, value)
            }
        }

        ZSTD_cParameter::ZSTD_c_compressionLevel
        | ZSTD_cParameter::ZSTD_c_windowLog
        | ZSTD_cParameter::ZSTD_c_hashLog
        | ZSTD_cParameter::ZSTD_c_chainLog
        | ZSTD_cParameter::ZSTD_c_searchLog
        | ZSTD_cParameter::ZSTD_c_minMatch
        | ZSTD_cParameter::ZSTD_c_targetLength
        | ZSTD_cParameter::ZSTD_c_strategy
        | ZSTD_cParameter::ZSTD_c_ldmHashRateLog
        | ZSTD_cParameter::ZSTD_c_format
        | ZSTD_cParameter::ZSTD_c_contentSizeFlag
        | ZSTD_cParameter::ZSTD_c_checksumFlag
        | ZSTD_cParameter::ZSTD_c_dictIDFlag
        | ZSTD_cParameter::ZSTD_c_forceMaxWindow
        | ZSTD_cParameter::ZSTD_c_forceAttachDict
        | ZSTD_cParameter::ZSTD_c_literalCompressionMode
        | ZSTD_cParameter::ZSTD_c_jobSize
        | ZSTD_cParameter::ZSTD_c_overlapLog
        | ZSTD_cParameter::ZSTD_c_rsyncable
        | ZSTD_cParameter::ZSTD_c_enableDedicatedDictSearch
        | ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching
        | ZSTD_cParameter::ZSTD_c_ldmHashLog
        | ZSTD_cParameter::ZSTD_c_ldmMinMatch
        | ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog
        | ZSTD_cParameter::ZSTD_c_targetCBlockSize
        | ZSTD_cParameter::ZSTD_c_srcSizeHint
        | ZSTD_cParameter::ZSTD_c_stableInBuffer
        | ZSTD_cParameter::ZSTD_c_stableOutBuffer
        | ZSTD_cParameter::ZSTD_c_blockDelimiters
        | ZSTD_cParameter::ZSTD_c_validateSequences
        | ZSTD_cParameter::ZSTD_c_splitAfterSequences
        | ZSTD_cParameter::ZSTD_c_blockSplitterLevel
        | ZSTD_cParameter::ZSTD_c_useRowMatchFinder
        | ZSTD_cParameter::ZSTD_c_deterministicRefPrefix
        | ZSTD_cParameter::ZSTD_c_prefetchCDictTables
        | ZSTD_cParameter::ZSTD_c_enableSeqProducerFallback
        | ZSTD_cParameter::ZSTD_c_maxBlockSize
        | ZSTD_cParameter::ZSTD_c_repcodeResolution => {
            ZSTD_CCtxParams_setParameter(&mut (*cctx).requestedParams, param, value)
        }

        _ => Error::parameter_unsupported.to_error_code(),
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtxParams_setParameter))]
pub unsafe extern "C" fn ZSTD_CCtxParams_setParameter(
    CCtxParams: *mut ZSTD_CCtx_params,
    param: ZSTD_cParameter,
    mut value: core::ffi::c_int,
) -> size_t {
    match param.0 {
        10 => {
            let Ok(format) = Format::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };

            (*CCtxParams).format = format;
            (*CCtxParams).format as size_t
        }
        100 => {
            let err_code = ZSTD_cParam_clampBounds(param, &mut value);
            if ERR_isError(err_code) {
                return err_code;
            }
            if value == 0 {
                (*CCtxParams).compressionLevel = ZSTD_CLEVEL_DEFAULT;
            } else {
                (*CCtxParams).compressionLevel = value;
            }
            if (*CCtxParams).compressionLevel >= 0 {
                return (*CCtxParams).compressionLevel as size_t;
            }
            0
        }
        101 => {
            if value != 0 && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_windowLog, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.windowLog = value as u32;
            (*CCtxParams).cParams.windowLog as size_t
        }
        102 => {
            if value != 0 && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_hashLog, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.hashLog = value as u32;
            (*CCtxParams).cParams.hashLog as size_t
        }
        103 => {
            if value != 0 && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_chainLog, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.chainLog = value as u32;
            (*CCtxParams).cParams.chainLog as size_t
        }
        104 => {
            if value != 0 && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_searchLog, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.searchLog = value as u32;
            value as size_t
        }
        105 => {
            if value != 0 && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_minMatch, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.minMatch = value as u32;
            (*CCtxParams).cParams.minMatch as size_t
        }
        106 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_targetLength, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.targetLength = value as u32;
            (*CCtxParams).cParams.targetLength as size_t
        }
        107 => {
            if value != 0 && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_strategy, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.strategy = value as ZSTD_strategy;
            (*CCtxParams).cParams.strategy as size_t
        }
        200 => {
            (*CCtxParams).fParams.contentSizeFlag = core::ffi::c_int::from(value != 0);
            (*CCtxParams).fParams.contentSizeFlag as size_t
        }
        201 => {
            (*CCtxParams).fParams.checksumFlag = core::ffi::c_int::from(value != 0);
            (*CCtxParams).fParams.checksumFlag as size_t
        }
        202 => {
            (*CCtxParams).fParams.noDictIDFlag = core::ffi::c_int::from(value == 0);
            core::ffi::c_int::from((*CCtxParams).fParams.noDictIDFlag == 0) as size_t
        }
        1000 => {
            (*CCtxParams).forceWindow = core::ffi::c_int::from(value != 0);
            (*CCtxParams).forceWindow as size_t
        }
        1001 => {
            let Ok(pref) = ZSTD_dictAttachPref_e::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).attachDictPref = pref;
            (*CCtxParams).attachDictPref.0 as size_t
        }
        1002 => {
            let Ok(lcm) = ParamSwitch::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).literalCompressionMode = lcm;
            (*CCtxParams).literalCompressionMode as size_t
        }
        400 => {
            let err_code_0 = ZSTD_cParam_clampBounds(param, &mut value);
            if ERR_isError(err_code_0) {
                return err_code_0;
            }
            (*CCtxParams).nbWorkers = value;
            (*CCtxParams).nbWorkers as size_t
        }
        401 => {
            if value != 0 && value < ZSTDMT_JOBSIZE_MIN {
                value = ZSTDMT_JOBSIZE_MIN;
            }
            let err_code_1 = ZSTD_cParam_clampBounds(param, &mut value);
            if ERR_isError(err_code_1) {
                return err_code_1;
            }
            (*CCtxParams).jobSize = value as size_t;
            (*CCtxParams).jobSize
        }
        402 => {
            let err_code_2 =
                ZSTD_cParam_clampBounds(ZSTD_cParameter::ZSTD_c_overlapLog, &mut value);
            if ERR_isError(err_code_2) {
                return err_code_2;
            }
            (*CCtxParams).overlapLog = value;
            (*CCtxParams).overlapLog as size_t
        }
        500 => {
            let err_code_3 =
                ZSTD_cParam_clampBounds(ZSTD_cParameter::ZSTD_c_overlapLog, &mut value);
            if ERR_isError(err_code_3) {
                return err_code_3;
            }
            (*CCtxParams).rsyncable = value;
            (*CCtxParams).rsyncable as size_t
        }
        1005 => {
            (*CCtxParams).enableDedicatedDictSearch = core::ffi::c_int::from(value != 0);
            (*CCtxParams).enableDedicatedDictSearch as size_t
        }
        160 => {
            let Ok(value) = ZSTD_ParamSwitch_e::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).ldmParams.enableLdm = value as ZSTD_ParamSwitch_e;
            (*CCtxParams).ldmParams.enableLdm as size_t
        }
        161 => {
            if value != 0
                && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmHashLog, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.hashLog = value as u32;
            (*CCtxParams).ldmParams.hashLog as size_t
        }
        162 => {
            if value != 0
                && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmMinMatch, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.minMatchLength = value as u32;
            (*CCtxParams).ldmParams.minMatchLength as size_t
        }
        163 => {
            if value != 0
                && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.bucketSizeLog = value as u32;
            (*CCtxParams).ldmParams.bucketSizeLog as size_t
        }
        164 => {
            if value != 0
                && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmHashRateLog, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.hashRateLog = value as u32;
            (*CCtxParams).ldmParams.hashRateLog as size_t
        }
        130 => {
            if value != 0 {
                value = if value > 1340 { value } else { 1340 };
                if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_targetCBlockSize, value) == 0 {
                    return Error::parameter_outOfBound.to_error_code();
                }
            }
            (*CCtxParams).targetCBlockSize = value as u32 as size_t;
            (*CCtxParams).targetCBlockSize
        }
        1004 => {
            if value != 0
                && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam7, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).srcSizeHint = value;
            (*CCtxParams).srcSizeHint as size_t
        }
        1006 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam9, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).inBufferMode = value as ZSTD_bufferMode_e;
            (*CCtxParams).inBufferMode as size_t
        }
        1007 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam10, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).outBufferMode = value as ZSTD_bufferMode_e;
            (*CCtxParams).outBufferMode as size_t
        }
        1008 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam11, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).blockDelimiters = value as ZSTD_SequenceFormat_e;
            (*CCtxParams).blockDelimiters as size_t
        }
        1009 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam12, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).validateSequences = value;
            (*CCtxParams).validateSequences as size_t
        }
        1010 => {
            let Ok(value) = ZSTD_ParamSwitch_e::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).postBlockSplitter = value as ZSTD_ParamSwitch_e;
            (*CCtxParams).postBlockSplitter as size_t
        }
        1017 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam20, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).preBlockSplitter_level = value;
            (*CCtxParams).preBlockSplitter_level as size_t
        }
        1011 => {
            let Ok(value) = ZSTD_ParamSwitch_e::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).useRowMatchFinder = value as ZSTD_ParamSwitch_e;
            (*CCtxParams).useRowMatchFinder as size_t
        }
        1012 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam15, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).deterministicRefPrefix = core::ffi::c_int::from(value != 0);
            (*CCtxParams).deterministicRefPrefix as size_t
        }
        1013 => {
            let Ok(value) = ZSTD_ParamSwitch_e::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).prefetchCDictTables = value as ZSTD_ParamSwitch_e;
            (*CCtxParams).prefetchCDictTables as size_t
        }
        1014 => {
            if ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam17, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).enableMatchFinderFallback = value;
            (*CCtxParams).enableMatchFinderFallback as size_t
        }
        1015 => {
            if value != 0
                && ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam18, value) == 0
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).maxBlockSize = value as size_t;
            (*CCtxParams).maxBlockSize
        }
        1016 => {
            let Ok(value) = ZSTD_ParamSwitch_e::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).searchForExternalRepcodes = value;
            (*CCtxParams).searchForExternalRepcodes as size_t
        }
        _ => Error::parameter_unsupported.to_error_code(),
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_getParameter))]
pub unsafe extern "C" fn ZSTD_CCtx_getParameter(
    cctx: *const ZSTD_CCtx,
    param: ZSTD_cParameter,
    value: *mut core::ffi::c_int,
) -> size_t {
    ZSTD_CCtxParams_getParameter(&(*cctx).requestedParams, param, value)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtxParams_getParameter))]
pub unsafe extern "C" fn ZSTD_CCtxParams_getParameter(
    CCtxParams: *const ZSTD_CCtx_params,
    param: ZSTD_cParameter,
    value: *mut core::ffi::c_int,
) -> size_t {
    match param.0 {
        10 => {
            *value = (*CCtxParams).format as core::ffi::c_int;
        }
        100 => {
            *value = (*CCtxParams).compressionLevel;
        }
        101 => {
            *value = (*CCtxParams).cParams.windowLog as core::ffi::c_int;
        }
        102 => {
            *value = (*CCtxParams).cParams.hashLog as core::ffi::c_int;
        }
        103 => {
            *value = (*CCtxParams).cParams.chainLog as core::ffi::c_int;
        }
        104 => {
            *value = (*CCtxParams).cParams.searchLog as core::ffi::c_int;
        }
        105 => {
            *value = (*CCtxParams).cParams.minMatch as core::ffi::c_int;
        }
        106 => {
            *value = (*CCtxParams).cParams.targetLength as core::ffi::c_int;
        }
        107 => {
            *value = (*CCtxParams).cParams.strategy as core::ffi::c_int;
        }
        200 => {
            *value = (*CCtxParams).fParams.contentSizeFlag;
        }
        201 => {
            *value = (*CCtxParams).fParams.checksumFlag;
        }
        202 => {
            *value = core::ffi::c_int::from((*CCtxParams).fParams.noDictIDFlag == 0);
        }
        1000 => {
            *value = (*CCtxParams).forceWindow;
        }
        1001 => {
            *value = (*CCtxParams).attachDictPref.0 as core::ffi::c_int;
        }
        1002 => {
            *value = (*CCtxParams).literalCompressionMode as core::ffi::c_int;
        }
        400 => {
            *value = (*CCtxParams).nbWorkers;
        }
        401 => {
            *value = (*CCtxParams).jobSize as core::ffi::c_int;
        }
        402 => {
            *value = (*CCtxParams).overlapLog;
        }
        500 => {
            *value = (*CCtxParams).rsyncable;
        }
        1005 => {
            *value = (*CCtxParams).enableDedicatedDictSearch;
        }
        160 => {
            *value = (*CCtxParams).ldmParams.enableLdm as core::ffi::c_int;
        }
        161 => {
            *value = (*CCtxParams).ldmParams.hashLog as core::ffi::c_int;
        }
        162 => {
            *value = (*CCtxParams).ldmParams.minMatchLength as core::ffi::c_int;
        }
        163 => {
            *value = (*CCtxParams).ldmParams.bucketSizeLog as core::ffi::c_int;
        }
        164 => {
            *value = (*CCtxParams).ldmParams.hashRateLog as core::ffi::c_int;
        }
        130 => {
            *value = (*CCtxParams).targetCBlockSize as core::ffi::c_int;
        }
        1004 => {
            *value = (*CCtxParams).srcSizeHint;
        }
        1006 => {
            *value = (*CCtxParams).inBufferMode as core::ffi::c_int;
        }
        1007 => {
            *value = (*CCtxParams).outBufferMode as core::ffi::c_int;
        }
        1008 => {
            *value = (*CCtxParams).blockDelimiters as core::ffi::c_int;
        }
        1009 => {
            *value = (*CCtxParams).validateSequences;
        }
        1010 => {
            *value = (*CCtxParams).postBlockSplitter as core::ffi::c_int;
        }
        1017 => {
            *value = (*CCtxParams).preBlockSplitter_level;
        }
        1011 => {
            *value = (*CCtxParams).useRowMatchFinder as core::ffi::c_int;
        }
        1012 => {
            *value = (*CCtxParams).deterministicRefPrefix;
        }
        1013 => {
            *value = (*CCtxParams).prefetchCDictTables as core::ffi::c_int;
        }
        1014 => {
            *value = (*CCtxParams).enableMatchFinderFallback;
        }
        1015 => {
            *value = (*CCtxParams).maxBlockSize as core::ffi::c_int;
        }
        1016 => {
            *value = (*CCtxParams).searchForExternalRepcodes as core::ffi::c_int;
        }
        _ => return Error::parameter_unsupported.to_error_code(),
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setParametersUsingCCtxParams))]
pub unsafe extern "C" fn ZSTD_CCtx_setParametersUsingCCtxParams(
    cctx: *mut ZSTD_CCtx,
    params: *const ZSTD_CCtx_params,
) -> size_t {
    if (*cctx).streamStage as core::ffi::c_uint
        != zcss_init as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    if !((*cctx).cdict).is_null() {
        return Error::stage_wrong.to_error_code();
    }
    (*cctx).requestedParams = *params;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setCParams))]
pub unsafe extern "C" fn ZSTD_CCtx_setCParams(
    cctx: *mut ZSTD_CCtx,
    cparams: ZSTD_compressionParameters,
) -> size_t {
    let err_code = ZSTD_checkCParams(cparams);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_windowLog,
        cparams.windowLog as core::ffi::c_int,
    );
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_chainLog,
        cparams.chainLog as core::ffi::c_int,
    );
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    let err_code_2 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_hashLog,
        cparams.hashLog as core::ffi::c_int,
    );
    if ERR_isError(err_code_2) {
        return err_code_2;
    }
    let err_code_3 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_searchLog,
        cparams.searchLog as core::ffi::c_int,
    );
    if ERR_isError(err_code_3) {
        return err_code_3;
    }
    let err_code_4 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_minMatch,
        cparams.minMatch as core::ffi::c_int,
    );
    if ERR_isError(err_code_4) {
        return err_code_4;
    }
    let err_code_5 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_targetLength,
        cparams.targetLength as core::ffi::c_int,
    );
    if ERR_isError(err_code_5) {
        return err_code_5;
    }
    let err_code_6 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_strategy,
        cparams.strategy as core::ffi::c_int,
    );
    if ERR_isError(err_code_6) {
        return err_code_6;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setFParams))]
pub unsafe extern "C" fn ZSTD_CCtx_setFParams(
    cctx: *mut ZSTD_CCtx,
    fparams: ZSTD_frameParameters,
) -> size_t {
    let err_code = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_contentSizeFlag,
        core::ffi::c_int::from(fparams.contentSizeFlag != 0),
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_checksumFlag,
        core::ffi::c_int::from(fparams.checksumFlag != 0),
    );
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_dictIDFlag,
        core::ffi::c_int::from(fparams.noDictIDFlag == 0),
    );
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setParams))]
pub unsafe extern "C" fn ZSTD_CCtx_setParams(
    cctx: *mut ZSTD_CCtx,
    params: ZSTD_parameters,
) -> size_t {
    let err_code = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setFParams(cctx, params.fParams);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setCParams(cctx, params.cParams);
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setPledgedSrcSize))]
pub unsafe extern "C" fn ZSTD_CCtx_setPledgedSrcSize(
    cctx: *mut ZSTD_CCtx,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    if (*cctx).streamStage as core::ffi::c_uint
        != zcss_init as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    (*cctx).pledgedSrcSizePlusOne = pledgedSrcSize.wrapping_add(1);
    0
}
unsafe fn ZSTD_initLocalDict(cctx: *mut ZSTD_CCtx) -> size_t {
    let dl: *mut ZSTD_localDict = &mut (*cctx).localDict;
    if ((*dl).dict).is_null() {
        return 0;
    }
    if !((*dl).cdict).is_null() {
        return 0;
    }
    (*dl).cdict = ZSTD_createCDict_advanced2(
        (*dl).dict,
        (*dl).dictSize,
        ZSTD_dlm_byRef,
        (*dl).dictContentType,
        &(*cctx).requestedParams,
        (*cctx).customMem,
    );
    if ((*dl).cdict).is_null() {
        return Error::memory_allocation.to_error_code();
    }
    (*cctx).cdict = (*dl).cdict;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_loadDictionary_advanced))]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary_advanced(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if (*cctx).streamStage as core::ffi::c_uint
        != zcss_init as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    ZSTD_clearAllDicts(cctx);
    if dict.is_null() || dictSize == 0 {
        return 0;
    }
    if dictLoadMethod as core::ffi::c_uint
        == ZSTD_dlm_byRef as core::ffi::c_int as core::ffi::c_uint
    {
        (*cctx).localDict.dict = dict;
    } else {
        let mut dictBuffer = core::ptr::null_mut::<core::ffi::c_void>();
        if (*cctx).staticSize != 0 {
            return Error::memory_allocation.to_error_code();
        }
        dictBuffer = ZSTD_customMalloc(dictSize, (*cctx).customMem);
        if dictBuffer.is_null() {
            return Error::memory_allocation.to_error_code();
        }
        libc::memcpy(dictBuffer, dict, dictSize as libc::size_t);
        (*cctx).localDict.dictBuffer = dictBuffer;
        (*cctx).localDict.dict = dictBuffer;
    }
    (*cctx).localDict.dictSize = dictSize;
    (*cctx).localDict.dictContentType = dictContentType;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_loadDictionary_byReference))]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary_byReference(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    ZSTD_CCtx_loadDictionary_advanced(cctx, dict, dictSize, ZSTD_dlm_byRef, ZSTD_dct_auto)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_loadDictionary))]
pub unsafe extern "C" fn ZSTD_CCtx_loadDictionary(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    ZSTD_CCtx_loadDictionary_advanced(cctx, dict, dictSize, ZSTD_dlm_byCopy, ZSTD_dct_auto)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_refCDict))]
pub unsafe extern "C" fn ZSTD_CCtx_refCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
) -> size_t {
    if (*cctx).streamStage as core::ffi::c_uint
        != zcss_init as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    ZSTD_clearAllDicts(cctx);
    (*cctx).cdict = cdict;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_refThreadPool))]
pub unsafe extern "C" fn ZSTD_CCtx_refThreadPool(
    cctx: *mut ZSTD_CCtx,
    pool: *mut ZSTD_threadPool,
) -> size_t {
    if (*cctx).streamStage as core::ffi::c_uint
        != zcss_init as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    (*cctx).pool = pool;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_refPrefix))]
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix(
    cctx: *mut ZSTD_CCtx,
    prefix: *const core::ffi::c_void,
    prefixSize: size_t,
) -> size_t {
    ZSTD_CCtx_refPrefix_advanced(cctx, prefix, prefixSize, ZSTD_dct_rawContent)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_refPrefix_advanced))]
pub unsafe extern "C" fn ZSTD_CCtx_refPrefix_advanced(
    cctx: *mut ZSTD_CCtx,
    prefix: *const core::ffi::c_void,
    prefixSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if (*cctx).streamStage as core::ffi::c_uint
        != zcss_init as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    ZSTD_clearAllDicts(cctx);
    if !prefix.is_null() && prefixSize > 0 {
        (*cctx).prefixDict.dict = prefix;
        (*cctx).prefixDict.dictSize = prefixSize;
        (*cctx).prefixDict.dictContentType = dictContentType;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_reset))]
pub unsafe extern "C" fn ZSTD_CCtx_reset(
    cctx: *mut ZSTD_CCtx,
    reset: ZSTD_ResetDirective,
) -> size_t {
    if matches!(
        reset,
        ZSTD_ResetDirective::ZSTD_reset_session_only
            | ZSTD_ResetDirective::ZSTD_reset_session_and_parameters
    ) {
        (*cctx).streamStage = zcss_init;
        (*cctx).pledgedSrcSizePlusOne = 0;
    }

    if matches!(
        reset,
        ZSTD_ResetDirective::ZSTD_reset_parameters
            | ZSTD_ResetDirective::ZSTD_reset_session_and_parameters
    ) {
        if (*cctx).streamStage as core::ffi::c_uint
            != zcss_init as core::ffi::c_int as core::ffi::c_uint
        {
            return Error::stage_wrong.to_error_code();
        }
        ZSTD_clearAllDicts(cctx);
        return ZSTD_CCtxParams_reset(&mut (*cctx).requestedParams);
    }

    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_checkCParams))]
pub unsafe extern "C" fn ZSTD_checkCParams(cParams: ZSTD_compressionParameters) -> size_t {
    if ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_windowLog,
        cParams.windowLog as core::ffi::c_int,
    ) == 0
    {
        return Error::parameter_outOfBound.to_error_code();
    }
    if ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_chainLog,
        cParams.chainLog as core::ffi::c_int,
    ) == 0
    {
        return Error::parameter_outOfBound.to_error_code();
    }
    if ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_hashLog,
        cParams.hashLog as core::ffi::c_int,
    ) == 0
    {
        return Error::parameter_outOfBound.to_error_code();
    }
    if ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_searchLog,
        cParams.searchLog as core::ffi::c_int,
    ) == 0
    {
        return Error::parameter_outOfBound.to_error_code();
    }
    if ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_minMatch,
        cParams.minMatch as core::ffi::c_int,
    ) == 0
    {
        return Error::parameter_outOfBound.to_error_code();
    }
    if ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_targetLength,
        cParams.targetLength as core::ffi::c_int,
    ) == 0
    {
        return Error::parameter_outOfBound.to_error_code();
    }
    if ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_strategy,
        cParams.strategy as core::ffi::c_int,
    ) == 0
    {
        return Error::parameter_outOfBound.to_error_code();
    }
    0
}
unsafe fn ZSTD_clampCParams(mut cParams: ZSTD_compressionParameters) -> ZSTD_compressionParameters {
    let bounds = ZSTD_cParam_getBounds(ZSTD_cParameter::ZSTD_c_windowLog);
    if (cParams.windowLog as core::ffi::c_int) < bounds.lowerBound {
        cParams.windowLog = bounds.lowerBound as core::ffi::c_uint;
    } else if cParams.windowLog as core::ffi::c_int > bounds.upperBound {
        cParams.windowLog = bounds.upperBound as core::ffi::c_uint;
    }
    let bounds_0 = ZSTD_cParam_getBounds(ZSTD_cParameter::ZSTD_c_chainLog);
    if (cParams.chainLog as core::ffi::c_int) < bounds_0.lowerBound {
        cParams.chainLog = bounds_0.lowerBound as core::ffi::c_uint;
    } else if cParams.chainLog as core::ffi::c_int > bounds_0.upperBound {
        cParams.chainLog = bounds_0.upperBound as core::ffi::c_uint;
    }
    let bounds_1 = ZSTD_cParam_getBounds(ZSTD_cParameter::ZSTD_c_hashLog);
    if (cParams.hashLog as core::ffi::c_int) < bounds_1.lowerBound {
        cParams.hashLog = bounds_1.lowerBound as core::ffi::c_uint;
    } else if cParams.hashLog as core::ffi::c_int > bounds_1.upperBound {
        cParams.hashLog = bounds_1.upperBound as core::ffi::c_uint;
    }
    let bounds_2 = ZSTD_cParam_getBounds(ZSTD_cParameter::ZSTD_c_searchLog);
    if (cParams.searchLog as core::ffi::c_int) < bounds_2.lowerBound {
        cParams.searchLog = bounds_2.lowerBound as core::ffi::c_uint;
    } else if cParams.searchLog as core::ffi::c_int > bounds_2.upperBound {
        cParams.searchLog = bounds_2.upperBound as core::ffi::c_uint;
    }
    let bounds_3 = ZSTD_cParam_getBounds(ZSTD_cParameter::ZSTD_c_minMatch);
    if (cParams.minMatch as core::ffi::c_int) < bounds_3.lowerBound {
        cParams.minMatch = bounds_3.lowerBound as core::ffi::c_uint;
    } else if cParams.minMatch as core::ffi::c_int > bounds_3.upperBound {
        cParams.minMatch = bounds_3.upperBound as core::ffi::c_uint;
    }
    let bounds_4 = ZSTD_cParam_getBounds(ZSTD_cParameter::ZSTD_c_targetLength);
    if (cParams.targetLength as core::ffi::c_int) < bounds_4.lowerBound {
        cParams.targetLength = bounds_4.lowerBound as core::ffi::c_uint;
    } else if cParams.targetLength as core::ffi::c_int > bounds_4.upperBound {
        cParams.targetLength = bounds_4.upperBound as core::ffi::c_uint;
    }
    let bounds_5 = ZSTD_cParam_getBounds(ZSTD_cParameter::ZSTD_c_strategy);
    if (cParams.strategy as core::ffi::c_int) < bounds_5.lowerBound {
        cParams.strategy = bounds_5.lowerBound as ZSTD_strategy;
    } else if cParams.strategy as core::ffi::c_int > bounds_5.upperBound {
        cParams.strategy = bounds_5.upperBound as ZSTD_strategy;
    }
    cParams
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_cycleLog))]
pub unsafe extern "C" fn ZSTD_cycleLog(hashLog: u32, strat: ZSTD_strategy) -> u32 {
    let btScale = u32::from(strat >= ZSTD_btlazy2);
    hashLog.wrapping_sub(btScale)
}
unsafe fn ZSTD_dictAndWindowLog(windowLog: u32, srcSize: u64, dictSize: u64) -> u32 {
    let maxWindowSize = 1 << ZSTD_WINDOWLOG_MAX;
    if dictSize == 0 {
        return windowLog;
    }
    let windowSize = ((1) << windowLog) as u64;
    let dictAndWindowSize = dictSize.wrapping_add(windowSize);
    if windowSize >= dictSize.wrapping_add(srcSize) {
        windowLog
    } else if dictAndWindowSize >= maxWindowSize {
        (if ::core::mem::size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) as u32
    } else {
        (ZSTD_highbit32((dictAndWindowSize as u32).wrapping_sub(1))).wrapping_add(1)
    }
}
unsafe fn ZSTD_adjustCParams_internal(
    mut cPar: ZSTD_compressionParameters,
    mut srcSize: core::ffi::c_ulonglong,
    mut dictSize: size_t,
    mode: ZSTD_CParamMode_e,
    mut useRowMatchFinder: ZSTD_ParamSwitch_e,
) -> ZSTD_compressionParameters {
    let minSrcSize = 513;
    let maxWindowResize = ((1)
        << ((if ::core::mem::size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) - 1)) as u64;
    match mode as core::ffi::c_uint {
        2 => {
            if dictSize != 0 && srcSize == ZSTD_CONTENTSIZE_UNKNOWN {
                srcSize = minSrcSize as core::ffi::c_ulonglong;
            }
        }
        1 => {
            dictSize = 0;
        }
        3 | 0 | _ => {}
    }
    if srcSize <= maxWindowResize && dictSize as u64 <= maxWindowResize {
        let tSize = srcSize.wrapping_add(dictSize as core::ffi::c_ulonglong) as u32;
        static hashSizeMin: u32 = ((1) << ZSTD_HASHLOG_MIN) as u32;
        let srcLog = if tSize < hashSizeMin {
            ZSTD_HASHLOG_MIN as core::ffi::c_uint
        } else {
            (ZSTD_highbit32(tSize.wrapping_sub(1))).wrapping_add(1)
        };
        if cPar.windowLog > srcLog {
            cPar.windowLog = srcLog;
        }
    }
    if srcSize != ZSTD_CONTENTSIZE_UNKNOWN {
        let dictAndWindowLog = ZSTD_dictAndWindowLog(cPar.windowLog, srcSize, dictSize as u64);
        let cycleLog = ZSTD_cycleLog(cPar.chainLog, cPar.strategy);
        if cPar.hashLog > dictAndWindowLog.wrapping_add(1) {
            cPar.hashLog = dictAndWindowLog.wrapping_add(1);
        }
        if cycleLog > dictAndWindowLog {
            cPar.chainLog = (cPar.chainLog).wrapping_sub(cycleLog.wrapping_sub(dictAndWindowLog));
        }
    }
    if cPar.windowLog < ZSTD_WINDOWLOG_ABSOLUTEMIN as core::ffi::c_uint {
        cPar.windowLog = ZSTD_WINDOWLOG_ABSOLUTEMIN as core::ffi::c_uint;
    }
    if mode as core::ffi::c_uint == ZSTD_cpm_createCDict as core::ffi::c_int as core::ffi::c_uint
        && ZSTD_CDictIndicesAreTagged(&cPar) != 0
    {
        let maxShortCacheHashLog = (32 - ZSTD_SHORT_CACHE_TAG_BITS) as u32;
        if cPar.hashLog > maxShortCacheHashLog {
            cPar.hashLog = maxShortCacheHashLog;
        }
        if cPar.chainLog > maxShortCacheHashLog {
            cPar.chainLog = maxShortCacheHashLog;
        }
    }
    if useRowMatchFinder == ZSTD_ParamSwitch_e::ZSTD_ps_auto {
        useRowMatchFinder = ZSTD_ParamSwitch_e::ZSTD_ps_enable;
    }
    if ZSTD_rowMatchFinderUsed(cPar.strategy, useRowMatchFinder) != 0 {
        let rowLog = if 4
            > (if cPar.searchLog < 6 {
                cPar.searchLog
            } else {
                6
            }) {
            4
        } else if cPar.searchLog < 6 {
            cPar.searchLog
        } else {
            6
        };
        let maxRowHashLog = (32 - ZSTD_ROW_HASH_TAG_BITS) as u32;
        let maxHashLog = maxRowHashLog.wrapping_add(rowLog);
        if cPar.hashLog > maxHashLog {
            cPar.hashLog = maxHashLog;
        }
    }
    cPar
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_adjustCParams))]
pub unsafe extern "C" fn ZSTD_adjustCParams(
    mut cPar: ZSTD_compressionParameters,
    mut srcSize: core::ffi::c_ulonglong,
    dictSize: size_t,
) -> ZSTD_compressionParameters {
    cPar = ZSTD_clampCParams(cPar);
    if srcSize == 0 {
        srcSize = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    ZSTD_adjustCParams_internal(
        cPar,
        srcSize,
        dictSize,
        ZSTD_cpm_unknown,
        ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    )
}
unsafe fn ZSTD_overrideCParams(
    cParams: *mut ZSTD_compressionParameters,
    overrides: *const ZSTD_compressionParameters,
) {
    if (*overrides).windowLog != 0 {
        (*cParams).windowLog = (*overrides).windowLog;
    }
    if (*overrides).hashLog != 0 {
        (*cParams).hashLog = (*overrides).hashLog;
    }
    if (*overrides).chainLog != 0 {
        (*cParams).chainLog = (*overrides).chainLog;
    }
    if (*overrides).searchLog != 0 {
        (*cParams).searchLog = (*overrides).searchLog;
    }
    if (*overrides).minMatch != 0 {
        (*cParams).minMatch = (*overrides).minMatch;
    }
    if (*overrides).targetLength != 0 {
        (*cParams).targetLength = (*overrides).targetLength;
    }
    if u64::from((*overrides).strategy) != 0 {
        (*cParams).strategy = (*overrides).strategy;
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getCParamsFromCCtxParams))]
pub unsafe extern "C" fn ZSTD_getCParamsFromCCtxParams(
    CCtxParams: *const ZSTD_CCtx_params,
    mut srcSizeHint: u64,
    dictSize: size_t,
    mode: ZSTD_CParamMode_e,
) -> ZSTD_compressionParameters {
    let mut cParams = ZSTD_compressionParameters {
        windowLog: 0,
        chainLog: 0,
        hashLog: 0,
        searchLog: 0,
        minMatch: 0,
        targetLength: 0,
        strategy: 0,
    };
    if srcSizeHint as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
        && (*CCtxParams).srcSizeHint > 0
    {
        srcSizeHint = (*CCtxParams).srcSizeHint as u64;
    }
    cParams = ZSTD_getCParams_internal(
        (*CCtxParams).compressionLevel,
        srcSizeHint as core::ffi::c_ulonglong,
        dictSize,
        mode,
    );
    if (*CCtxParams).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        cParams.windowLog = ZSTD_LDM_DEFAULT_WINDOW_LOG as core::ffi::c_uint;
    }
    ZSTD_overrideCParams(&mut cParams, &(*CCtxParams).cParams);
    ZSTD_adjustCParams_internal(
        cParams,
        srcSizeHint as core::ffi::c_ulonglong,
        dictSize,
        mode,
        (*CCtxParams).useRowMatchFinder,
    )
}
unsafe fn ZSTD_sizeof_matchState(
    cParams: *const ZSTD_compressionParameters,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    enableDedicatedDictSearch: core::ffi::c_int,
    forCCtx: u32,
) -> size_t {
    let chainSize = if ZSTD_allocateChainTable(
        (*cParams).strategy,
        useRowMatchFinder,
        core::ffi::c_int::from(enableDedicatedDictSearch != 0 && forCCtx == 0) as u32,
    ) != 0
    {
        (1 as size_t) << (*cParams).chainLog
    } else {
        0 as size_t
    };
    let hSize = (1 as size_t) << (*cParams).hashLog;
    let hashLog3 = if forCCtx != 0 && (*cParams).minMatch == 3 {
        if (17) < (*cParams).windowLog {
            17
        } else {
            (*cParams).windowLog
        }
    } else {
        0
    };
    let h3Size = if hashLog3 != 0 {
        (1 as size_t) << hashLog3
    } else {
        0 as size_t
    };
    let tableSpace = chainSize
        .wrapping_mul(::core::mem::size_of::<u32>())
        .wrapping_add(hSize.wrapping_mul(::core::mem::size_of::<u32>()))
        .wrapping_add(h3Size.wrapping_mul(::core::mem::size_of::<u32>()));
    let optPotentialSpace = (ZSTD_cwksp_aligned64_alloc_size(
        ((MaxML + 1) as size_t).wrapping_mul(::core::mem::size_of::<u32>()),
    ))
    .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
        ((MaxLL + 1) as size_t).wrapping_mul(::core::mem::size_of::<u32>()),
    ))
    .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
        ((MaxOff + 1) as size_t).wrapping_mul(::core::mem::size_of::<u32>()),
    ))
    .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
        (((1) << Litbits) as size_t).wrapping_mul(::core::mem::size_of::<u32>()),
    ))
    .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
        (ZSTD_OPT_SIZE as size_t).wrapping_mul(::core::mem::size_of::<ZSTD_match_t>()),
    ))
    .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
        (ZSTD_OPT_SIZE as size_t).wrapping_mul(::core::mem::size_of::<ZSTD_optimal_t>()),
    ));
    let lazyAdditionalSpace =
        if ZSTD_rowMatchFinderUsed((*cParams).strategy, useRowMatchFinder) != 0 {
            ZSTD_cwksp_aligned64_alloc_size(hSize)
        } else {
            0
        };
    let optSpace = if forCCtx != 0
        && (*cParams).strategy as core::ffi::c_uint
            >= ZSTD_btopt as core::ffi::c_int as core::ffi::c_uint
    {
        optPotentialSpace
    } else {
        0
    };
    let slackSpace = ZSTD_cwksp_slack_space_required();
    tableSpace
        .wrapping_add(optSpace)
        .wrapping_add(slackSpace)
        .wrapping_add(lazyAdditionalSpace)
}
unsafe fn ZSTD_maxNbSeq(
    blockSize: size_t,
    minMatch: core::ffi::c_uint,
    useSequenceProducer: core::ffi::c_int,
) -> size_t {
    let divider = (if minMatch == 3 || useSequenceProducer != 0 {
        3
    } else {
        4
    }) as u32;
    blockSize / divider as size_t
}
unsafe fn ZSTD_estimateCCtxSize_usingCCtxParams_internal(
    cParams: *const ZSTD_compressionParameters,
    ldmParams: *const ldmParams_t,
    isStatic: core::ffi::c_int,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    buffInSize: size_t,
    buffOutSize: size_t,
    pledgedSrcSize: u64,
    useSequenceProducer: core::ffi::c_int,
    maxBlockSize: size_t,
) -> size_t {
    let windowSize = (if 1
        > (if (1) << (*cParams).windowLog < pledgedSrcSize as core::ffi::c_ulonglong {
            (1) << (*cParams).windowLog
        } else {
            pledgedSrcSize as core::ffi::c_ulonglong
        }) {
        1
    } else if (1) << (*cParams).windowLog < pledgedSrcSize as core::ffi::c_ulonglong {
        (1) << (*cParams).windowLog
    } else {
        pledgedSrcSize as core::ffi::c_ulonglong
    }) as size_t;
    let blockSize = if ZSTD_resolveMaxBlockSize(maxBlockSize) < windowSize {
        ZSTD_resolveMaxBlockSize(maxBlockSize)
    } else {
        windowSize
    };
    let maxNbSeq = ZSTD_maxNbSeq(blockSize, (*cParams).minMatch, useSequenceProducer);
    let tokenSpace = (ZSTD_cwksp_alloc_size(WILDCOPY_OVERLENGTH.wrapping_add(blockSize)))
        .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
            maxNbSeq.wrapping_mul(::core::mem::size_of::<SeqDef>()),
        ))
        .wrapping_add(
            3 * ZSTD_cwksp_alloc_size(maxNbSeq.wrapping_mul(::core::mem::size_of::<u8>())),
        );
    let tmpWorkSpace = ZSTD_cwksp_alloc_size(
        if ((((8) << 10) + 512) as size_t).wrapping_add(
            (::core::mem::size_of::<core::ffi::c_uint>())
                .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
        ) > 8208
        {
            ((((8) << 10) + 512) as size_t).wrapping_add(
                (::core::mem::size_of::<core::ffi::c_uint>())
                    .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
            )
        } else {
            8208
        },
    );
    let blockStateSpace =
        2 * ZSTD_cwksp_alloc_size(::core::mem::size_of::<ZSTD_compressedBlockState_t>());
    let matchStateSize = ZSTD_sizeof_matchState(cParams, useRowMatchFinder, 0, 1);
    let ldmSpace = ZSTD_ldm_getTableSize(*ldmParams);
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq(*ldmParams, blockSize);
    let ldmSeqSpace = if (*ldmParams).enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        ZSTD_cwksp_aligned64_alloc_size(maxNbLdmSeq.wrapping_mul(::core::mem::size_of::<rawSeq>()))
    } else {
        0
    };
    let bufferSpace =
        (ZSTD_cwksp_alloc_size(buffInSize)).wrapping_add(ZSTD_cwksp_alloc_size(buffOutSize));
    let cctxSpace = if isStatic != 0 {
        ZSTD_cwksp_alloc_size(::core::mem::size_of::<ZSTD_CCtx>())
    } else {
        0
    };
    let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
    let externalSeqSpace = if useSequenceProducer != 0 {
        ZSTD_cwksp_aligned64_alloc_size(
            maxNbExternalSeq.wrapping_mul(::core::mem::size_of::<ZSTD_Sequence>()),
        )
    } else {
        0
    };

    cctxSpace
        .wrapping_add(tmpWorkSpace)
        .wrapping_add(blockStateSpace)
        .wrapping_add(ldmSpace)
        .wrapping_add(ldmSeqSpace)
        .wrapping_add(matchStateSize)
        .wrapping_add(tokenSpace)
        .wrapping_add(bufferSpace)
        .wrapping_add(externalSeqSpace)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCCtxSize_usingCCtxParams))]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCCtxParams(
    params: *const ZSTD_CCtx_params,
) -> size_t {
    let cParams =
        ZSTD_getCParamsFromCCtxParams(params, ZSTD_CONTENTSIZE_UNKNOWN, 0, ZSTD_cpm_noAttachDict);
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode((*params).useRowMatchFinder, &cParams);
    if (*params).nbWorkers > 0 {
        return Error::GENERIC.to_error_code();
    }
    ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &cParams,
        &(*params).ldmParams,
        1,
        useRowMatchFinder,
        0,
        0,
        ZSTD_CONTENTSIZE_UNKNOWN,
        ZSTD_hasExtSeqProd(params),
        (*params).maxBlockSize,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCCtxSize_usingCParams))]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize_usingCParams(
    cParams: ZSTD_compressionParameters,
) -> size_t {
    let mut initialParams = ZSTD_makeCCtxParamsFromCParams(cParams);
    if ZSTD_rowMatchFinderSupported(cParams.strategy) != 0 {
        let mut noRowCCtxSize: size_t = 0;
        let mut rowCCtxSize: size_t = 0;
        initialParams.useRowMatchFinder = ZSTD_ParamSwitch_e::ZSTD_ps_disable;
        noRowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&initialParams);
        initialParams.useRowMatchFinder = ZSTD_ParamSwitch_e::ZSTD_ps_enable;
        rowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&initialParams);
        if noRowCCtxSize > rowCCtxSize {
            noRowCCtxSize
        } else {
            rowCCtxSize
        }
    } else {
        ZSTD_estimateCCtxSize_usingCCtxParams(&initialParams)
    }
}
static srcSizeTiers: [core::ffi::c_ulonglong; 4] = [
    16 * (1 << 10),
    128 * (1 << 10),
    256 * (1 << 10),
    ZSTD_CONTENTSIZE_UNKNOWN,
];
unsafe extern "C" fn ZSTD_estimateCCtxSize_internal(compressionLevel: core::ffi::c_int) -> size_t {
    let mut tier = 0;
    let mut largestSize = 0;
    while tier < 4 {
        let cParams = ZSTD_getCParams_internal(
            compressionLevel,
            srcSizeTiers[tier],
            0,
            ZSTD_cpm_noAttachDict,
        );
        largestSize = if ZSTD_estimateCCtxSize_usingCParams(cParams) > largestSize {
            ZSTD_estimateCCtxSize_usingCParams(cParams)
        } else {
            largestSize
        };
        tier += 1;
    }
    largestSize
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCCtxSize))]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize(compressionLevel: core::ffi::c_int) -> size_t {
    let mut level: core::ffi::c_int = 0;
    let mut memBudget = 0;
    level = if compressionLevel < 1 {
        compressionLevel
    } else {
        1
    };
    while level <= compressionLevel {
        let newMB = ZSTD_estimateCCtxSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
        level += 1;
    }
    memBudget
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCStreamSize_usingCCtxParams))]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize_usingCCtxParams(
    params: *const ZSTD_CCtx_params,
) -> size_t {
    if (*params).nbWorkers > 0 {
        return Error::GENERIC.to_error_code();
    }
    let cParams =
        ZSTD_getCParamsFromCCtxParams(params, ZSTD_CONTENTSIZE_UNKNOWN, 0, ZSTD_cpm_noAttachDict);
    let blockSize = if ZSTD_resolveMaxBlockSize((*params).maxBlockSize) < (1) << cParams.windowLog {
        ZSTD_resolveMaxBlockSize((*params).maxBlockSize)
    } else {
        (1) << cParams.windowLog
    };
    let inBuffSize = if (*params).inBufferMode as core::ffi::c_uint
        == ZSTD_bm_buffered as core::ffi::c_int as core::ffi::c_uint
    {
        ((1 as size_t) << cParams.windowLog).wrapping_add(blockSize)
    } else {
        0
    };
    let outBuffSize = if (*params).outBufferMode as core::ffi::c_uint
        == ZSTD_bm_buffered as core::ffi::c_int as core::ffi::c_uint
    {
        (ZSTD_compressBound(blockSize)).wrapping_add(1)
    } else {
        0
    };
    let useRowMatchFinder =
        ZSTD_resolveRowMatchFinderMode((*params).useRowMatchFinder, &(*params).cParams);
    ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &cParams,
        &(*params).ldmParams,
        1,
        useRowMatchFinder,
        inBuffSize,
        outBuffSize,
        ZSTD_CONTENTSIZE_UNKNOWN,
        ZSTD_hasExtSeqProd(params),
        (*params).maxBlockSize,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCStreamSize_usingCParams))]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize_usingCParams(
    cParams: ZSTD_compressionParameters,
) -> size_t {
    let mut initialParams = ZSTD_makeCCtxParamsFromCParams(cParams);
    if ZSTD_rowMatchFinderSupported(cParams.strategy) != 0 {
        let mut noRowCCtxSize: size_t = 0;
        let mut rowCCtxSize: size_t = 0;
        initialParams.useRowMatchFinder = ZSTD_ParamSwitch_e::ZSTD_ps_disable;
        noRowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&initialParams);
        initialParams.useRowMatchFinder = ZSTD_ParamSwitch_e::ZSTD_ps_enable;
        rowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&initialParams);
        if noRowCCtxSize > rowCCtxSize {
            noRowCCtxSize
        } else {
            rowCCtxSize
        }
    } else {
        ZSTD_estimateCStreamSize_usingCCtxParams(&initialParams)
    }
}
unsafe fn ZSTD_estimateCStreamSize_internal(compressionLevel: core::ffi::c_int) -> size_t {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        0,
        ZSTD_cpm_noAttachDict,
    );
    ZSTD_estimateCStreamSize_usingCParams(cParams)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCStreamSize))]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize(compressionLevel: core::ffi::c_int) -> size_t {
    let mut level: core::ffi::c_int = 0;
    let mut memBudget = 0;
    level = if compressionLevel < 1 {
        compressionLevel
    } else {
        1
    };
    while level <= compressionLevel {
        let newMB = ZSTD_estimateCStreamSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
        level += 1;
    }
    memBudget
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getFrameProgression))]
pub unsafe extern "C" fn ZSTD_getFrameProgression(cctx: *const ZSTD_CCtx) -> ZSTD_frameProgression {
    if (*cctx).appliedParams.nbWorkers > 0 {
        return ZSTDMT_getFrameProgression((*cctx).mtctx);
    }
    let mut fp = ZSTD_frameProgression {
        ingested: 0,
        consumed: 0,
        produced: 0,
        flushed: 0,
        currentJobID: 0,
        nbActiveWorkers: 0,
    };
    let buffered = if ((*cctx).inBuff).is_null() {
        0
    } else {
        ((*cctx).inBuffPos).wrapping_sub((*cctx).inToCompress)
    };

    if buffered != 0 {
        assert!((*cctx).inBuffPos >= (*cctx).inToCompress)
    }
    assert!(buffered <= ZSTD_BLOCKSIZE_MAX as usize);

    fp.ingested = ((*cctx).consumedSrcSize).wrapping_add(buffered as core::ffi::c_ulonglong);
    fp.consumed = (*cctx).consumedSrcSize;
    fp.produced = (*cctx).producedCSize;
    fp.flushed = (*cctx).producedCSize;
    fp.currentJobID = 0;
    fp.nbActiveWorkers = 0;
    fp
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_toFlushNow))]
pub unsafe extern "C" fn ZSTD_toFlushNow(cctx: *mut ZSTD_CCtx) -> size_t {
    if (*cctx).appliedParams.nbWorkers > 0 {
        return ZSTDMT_toFlushNow((*cctx).mtctx);
    }
    0
}
unsafe fn ZSTD_assertEqualCParams(
    cParams1: ZSTD_compressionParameters,
    cParams2: ZSTD_compressionParameters,
) {
    assert_eq!(cParams1.windowLog, cParams2.windowLog);
    assert_eq!(cParams1.chainLog, cParams2.chainLog);
    assert_eq!(cParams1.hashLog, cParams2.hashLog);
    assert_eq!(cParams1.searchLog, cParams2.searchLog);
    assert_eq!(cParams1.minMatch, cParams2.minMatch);
    assert_eq!(cParams1.targetLength, cParams2.targetLength);
    assert_eq!(cParams1.strategy, cParams2.strategy);
}
pub unsafe fn ZSTD_reset_compressedBlockState(bs: *mut ZSTD_compressedBlockState_t) {
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < ZSTD_REP_NUM {
        *((*bs).rep).as_mut_ptr().offset(i as isize) = *repStartValue.as_ptr().offset(i as isize);
        i += 1;
    }
    (*bs).entropy.huf.repeatMode = HUF_repeat_none;
    (*bs).entropy.fse.offcode_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.matchlength_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.litlength_repeatMode = FSE_repeat_none;
}
unsafe fn ZSTD_invalidateMatchState(ms: &mut ZSTD_MatchState_t) {
    ZSTD_window_clear(&mut ms.window);
    ms.nextToUpdate = ms.window.dictLimit;
    ms.loadedDictEnd = 0;
    ms.opt.litLengthSum = 0;
    ms.dictMatchState = core::ptr::null();
}
unsafe fn ZSTD_bitmix(mut val: u64, len: u64) -> u64 {
    val ^= val.rotate_right(49) ^ val.rotate_right(24);
    val = val.wrapping_mul(0x9fb21c651e98df25u64);
    val ^= (val >> 35).wrapping_add(len);
    val = val.wrapping_mul(0x9fb21c651e98df25u64);
    val ^ val >> 28
}
unsafe fn ZSTD_advanceHashSalt(ms: &mut ZSTD_MatchState_t) {
    ms.hashSalt = ZSTD_bitmix(ms.hashSalt, 8) ^ ZSTD_bitmix(u64::from(ms.hashSaltEntropy), 4);
}
unsafe fn ZSTD_reset_matchState(
    ms: &mut ZSTD_MatchState_t,
    ws: *mut ZSTD_cwksp,
    cParams: *const ZSTD_compressionParameters,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    crp: ZSTD_compResetPolicy_e,
    forceResetIndex: ZSTD_indexResetPolicy_e,
    forWho: ZSTD_resetTarget_e,
) -> size_t {
    let chainSize = if ZSTD_allocateChainTable(
        (*cParams).strategy,
        useRowMatchFinder,
        core::ffi::c_int::from(
            ms.dedicatedDictSearch != 0
                && forWho as core::ffi::c_uint
                    == ZSTD_resetTarget_CDict as core::ffi::c_int as core::ffi::c_uint,
        ) as u32,
    ) != 0
    {
        (1 as size_t) << (*cParams).chainLog
    } else {
        0 as size_t
    };
    let hSize = (1 as size_t) << (*cParams).hashLog;
    let hashLog3 = if forWho as core::ffi::c_uint
        == ZSTD_resetTarget_CCtx as core::ffi::c_int as core::ffi::c_uint
        && (*cParams).minMatch == 3
    {
        if (17) < (*cParams).windowLog {
            17
        } else {
            (*cParams).windowLog
        }
    } else {
        0
    };
    let h3Size = if hashLog3 != 0 {
        (1 as size_t) << hashLog3
    } else {
        0 as size_t
    };
    if forceResetIndex as core::ffi::c_uint
        == ZSTDirp_reset as core::ffi::c_int as core::ffi::c_uint
    {
        ZSTD_window_init(&mut ms.window);
        ZSTD_cwksp_mark_tables_dirty(ws);
    }
    ms.hashLog3 = hashLog3;
    ms.lazySkipping = 0;
    ZSTD_invalidateMatchState(ms);
    ZSTD_cwksp_clear_tables(ws);
    ms.hashTable =
        ZSTD_cwksp_reserve_table(ws, hSize.wrapping_mul(::core::mem::size_of::<u32>())) as *mut u32;
    ms.chainTable =
        ZSTD_cwksp_reserve_table(ws, chainSize.wrapping_mul(::core::mem::size_of::<u32>()))
            as *mut u32;
    ms.hashTable3 = ZSTD_cwksp_reserve_table(ws, h3Size.wrapping_mul(::core::mem::size_of::<u32>()))
        as *mut u32;
    if ZSTD_cwksp_reserve_failed(ws) != 0 {
        return Error::memory_allocation.to_error_code();
    }
    if crp as core::ffi::c_uint != ZSTDcrp_leaveDirty as core::ffi::c_int as core::ffi::c_uint {
        ZSTD_cwksp_clean_tables(ws);
    }
    if ZSTD_rowMatchFinderUsed((*cParams).strategy, useRowMatchFinder) != 0 {
        let tagTableSize = hSize;
        if forWho as core::ffi::c_uint
            == ZSTD_resetTarget_CCtx as core::ffi::c_int as core::ffi::c_uint
        {
            ms.tagTable = ZSTD_cwksp_reserve_aligned_init_once(ws, tagTableSize) as *mut u8;
            ZSTD_advanceHashSalt(ms);
        } else {
            ms.tagTable = ZSTD_cwksp_reserve_aligned64(ws, tagTableSize) as *mut u8;
            ptr::write_bytes(ms.tagTable, 0, tagTableSize);
            ms.hashSalt = 0;
        }
        let rowLog = if 4
            > (if (*cParams).searchLog < 6 {
                (*cParams).searchLog
            } else {
                6
            }) {
            4
        } else if (*cParams).searchLog < 6 {
            (*cParams).searchLog
        } else {
            6
        };
        ms.rowHashLog = ((*cParams).hashLog).wrapping_sub(rowLog);
    }
    if forWho as core::ffi::c_uint == ZSTD_resetTarget_CCtx as core::ffi::c_int as core::ffi::c_uint
        && (*cParams).strategy as core::ffi::c_uint
            >= ZSTD_btopt as core::ffi::c_int as core::ffi::c_uint
    {
        ms.opt.litFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            (((1) << Litbits) as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.litLengthFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((MaxLL + 1) as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.matchLengthFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((MaxML + 1) as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.offCodeFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((MaxOff + 1) as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.matchTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            (ZSTD_OPT_SIZE as size_t).wrapping_mul(::core::mem::size_of::<ZSTD_match_t>()),
        ) as *mut ZSTD_match_t;
        ms.opt.priceTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            (ZSTD_OPT_SIZE as size_t).wrapping_mul(::core::mem::size_of::<ZSTD_optimal_t>()),
        ) as *mut ZSTD_optimal_t;
    }
    ms.cParams = *cParams;
    if ZSTD_cwksp_reserve_failed(ws) != 0 {
        return Error::memory_allocation.to_error_code();
    }
    0
}
pub const ZSTD_INDEXOVERFLOW_MARGIN: core::ffi::c_int = 16 * ((1) << 20);
unsafe fn ZSTD_indexTooCloseToMax(w: ZSTD_window_t) -> core::ffi::c_int {
    core::ffi::c_int::from(
        (w.nextSrc).wrapping_offset_from(w.base) as size_t
            > (if MEM_64bits() {
                (3500 as core::ffi::c_uint)
                    .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
            } else {
                (2000 as core::ffi::c_uint)
                    .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
            })
            .wrapping_sub(ZSTD_INDEXOVERFLOW_MARGIN as core::ffi::c_uint) as size_t,
    )
}
unsafe fn ZSTD_dictTooBig(loadedDictSize: size_t) -> core::ffi::c_int {
    core::ffi::c_int::from(
        loadedDictSize
            > (-(1 as core::ffi::c_int) as u32).wrapping_sub(if MEM_64bits() {
                (3500 as core::ffi::c_uint)
                    .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
            } else {
                (2000 as core::ffi::c_uint)
                    .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
            }) as size_t,
    )
}
unsafe fn ZSTD_resetCCtx_internal(
    zc: *mut ZSTD_CCtx,
    mut params: *const ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    loadedDictSize: size_t,
    crp: ZSTD_compResetPolicy_e,
    zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let ws: *mut ZSTD_cwksp = &mut (*zc).workspace;
    (*zc).isFirstBlock = 1;
    (*zc).appliedParams = *params;
    params = &mut (*zc).appliedParams;
    if (*params).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        ZSTD_ldm_adjustParameters(&mut (*zc).appliedParams.ldmParams, &(*params).cParams);
    }
    let windowSize = if 1
        > (if (1 as size_t) << (*params).cParams.windowLog < pledgedSrcSize as size_t {
            (1 as size_t) << (*params).cParams.windowLog
        } else {
            pledgedSrcSize as size_t
        }) {
        1
    } else if (1 as size_t) << (*params).cParams.windowLog < pledgedSrcSize as size_t {
        (1 as size_t) << (*params).cParams.windowLog
    } else {
        pledgedSrcSize as size_t
    };
    let blockSize = if (*params).maxBlockSize < windowSize {
        (*params).maxBlockSize
    } else {
        windowSize
    };
    let maxNbSeq = ZSTD_maxNbSeq(
        blockSize,
        (*params).cParams.minMatch,
        ZSTD_hasExtSeqProd(params),
    );
    let buffOutSize = if zbuff as core::ffi::c_uint
        == ZSTDb_buffered as core::ffi::c_int as core::ffi::c_uint
        && (*params).outBufferMode as core::ffi::c_uint
            == ZSTD_bm_buffered as core::ffi::c_int as core::ffi::c_uint
    {
        (ZSTD_compressBound(blockSize)).wrapping_add(1)
    } else {
        0
    };
    let buffInSize = if zbuff as core::ffi::c_uint
        == ZSTDb_buffered as core::ffi::c_int as core::ffi::c_uint
        && (*params).inBufferMode as core::ffi::c_uint
            == ZSTD_bm_buffered as core::ffi::c_int as core::ffi::c_uint
    {
        windowSize.wrapping_add(blockSize)
    } else {
        0
    };
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq((*params).ldmParams, blockSize);
    let indexTooClose = ZSTD_indexTooCloseToMax((*zc).blockState.matchState.window);
    let dictTooBig = ZSTD_dictTooBig(loadedDictSize);
    let mut needsIndexReset = (if indexTooClose != 0 || dictTooBig != 0 || (*zc).initialized == 0 {
        ZSTDirp_reset as core::ffi::c_int
    } else {
        ZSTDirp_continue as core::ffi::c_int
    }) as ZSTD_indexResetPolicy_e;
    let neededSpace = ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &(*params).cParams,
        &(*params).ldmParams,
        core::ffi::c_int::from((*zc).staticSize != 0),
        (*params).useRowMatchFinder,
        buffInSize,
        buffOutSize,
        pledgedSrcSize,
        ZSTD_hasExtSeqProd(params),
        (*params).maxBlockSize,
    );
    let err_code = neededSpace;
    if ERR_isError(err_code) {
        return err_code;
    }
    if (*zc).staticSize == 0 {
        ZSTD_cwksp_bump_oversized_duration(ws, 0);
    }
    let workspaceTooSmall = core::ffi::c_int::from(ZSTD_cwksp_sizeof(ws) < neededSpace);
    let workspaceWasteful = ZSTD_cwksp_check_wasteful(ws, neededSpace);
    let resizeWorkspace = core::ffi::c_int::from(workspaceTooSmall != 0 || workspaceWasteful != 0);
    if resizeWorkspace != 0 {
        if (*zc).staticSize != 0 {
            return Error::memory_allocation.to_error_code();
        }
        needsIndexReset = ZSTDirp_reset;
        ZSTD_cwksp_free(ws, (*zc).customMem);
        let err_code_0 = ZSTD_cwksp_create(ws, neededSpace, (*zc).customMem);
        if ERR_isError(err_code_0) {
            return err_code_0;
        }
        (*zc).blockState.prevCBlock =
            ZSTD_cwksp_reserve_object(ws, ::core::mem::size_of::<ZSTD_compressedBlockState_t>())
                as *mut ZSTD_compressedBlockState_t;
        if ((*zc).blockState.prevCBlock).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*zc).blockState.nextCBlock =
            ZSTD_cwksp_reserve_object(ws, ::core::mem::size_of::<ZSTD_compressedBlockState_t>())
                as *mut ZSTD_compressedBlockState_t;
        if ((*zc).blockState.nextCBlock).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*zc).tmpWorkspace = ZSTD_cwksp_reserve_object(
            ws,
            if ((((8) << 10) + 512) as size_t).wrapping_add(
                (::core::mem::size_of::<core::ffi::c_uint>())
                    .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
            ) > 8208
            {
                ((((8) << 10) + 512) as size_t).wrapping_add(
                    (::core::mem::size_of::<core::ffi::c_uint>())
                        .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
                )
            } else {
                8208
            },
        );
        if ((*zc).tmpWorkspace).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*zc).tmpWkspSize = if ((((8) << 10) + 512) as size_t).wrapping_add(
            (::core::mem::size_of::<core::ffi::c_uint>())
                .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
        ) > 8208
        {
            ((((8) << 10) + 512) as size_t).wrapping_add(
                (::core::mem::size_of::<core::ffi::c_uint>())
                    .wrapping_mul(((if 35 > 52 { 35 } else { 52 }) + 2) as size_t),
            )
        } else {
            8208
        };
    }
    ZSTD_cwksp_clear(ws);
    (*zc).blockState.matchState.cParams = (*params).cParams;
    (*zc).blockState.matchState.prefetchCDictTables =
        core::ffi::c_int::from((*params).prefetchCDictTables == ZSTD_ParamSwitch_e::ZSTD_ps_enable);
    (*zc).pledgedSrcSizePlusOne = pledgedSrcSize.wrapping_add(1) as core::ffi::c_ulonglong;
    (*zc).consumedSrcSize = 0;
    (*zc).producedCSize = 0;
    if pledgedSrcSize as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN {
        (*zc).appliedParams.fParams.contentSizeFlag = 0;
    }
    (*zc).blockSizeMax = blockSize;
    ZSTD_XXH64_reset(&mut (*zc).xxhState, 0);
    (*zc).stage = ZSTDcs_init;
    (*zc).dictID = 0;
    (*zc).dictContentSize = 0;
    ZSTD_reset_compressedBlockState((*zc).blockState.prevCBlock);
    let err_code_1 = ZSTD_reset_matchState(
        &mut (*zc).blockState.matchState,
        ws,
        &(*params).cParams,
        (*params).useRowMatchFinder,
        crp,
        needsIndexReset,
        ZSTD_resetTarget_CCtx,
    );
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    (*zc).seqStore.sequencesStart =
        ZSTD_cwksp_reserve_aligned64(ws, maxNbSeq.wrapping_mul(::core::mem::size_of::<SeqDef>()))
            as *mut SeqDef;
    if (*params).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        let ldmHSize = (1 as size_t) << (*params).ldmParams.hashLog;
        (*zc).ldmState.hashTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            ldmHSize.wrapping_mul(::core::mem::size_of::<ldmEntry_t>()),
        ) as *mut ldmEntry_t;
        ptr::write_bytes(
            (*zc).ldmState.hashTable as *mut u8,
            0,
            ldmHSize.wrapping_mul(::core::mem::size_of::<ldmEntry_t>()),
        );
        (*zc).ldmSequences = ZSTD_cwksp_reserve_aligned64(
            ws,
            maxNbLdmSeq.wrapping_mul(::core::mem::size_of::<rawSeq>()),
        ) as *mut rawSeq;
        (*zc).maxNbLdmSequences = maxNbLdmSeq;
        ZSTD_window_init(&mut (*zc).ldmState.window);
        (*zc).ldmState.loadedDictEnd = 0;
    }
    if ZSTD_hasExtSeqProd(params) != 0 {
        let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
        (*zc).extSeqBufCapacity = maxNbExternalSeq;
        (*zc).extSeqBuf = ZSTD_cwksp_reserve_aligned64(
            ws,
            maxNbExternalSeq.wrapping_mul(::core::mem::size_of::<ZSTD_Sequence>()),
        ) as *mut ZSTD_Sequence;
    }
    (*zc).seqStore.litStart =
        ZSTD_cwksp_reserve_buffer(ws, blockSize.wrapping_add(WILDCOPY_OVERLENGTH));
    (*zc).seqStore.maxNbLit = blockSize;
    (*zc).bufferedPolicy = zbuff;
    (*zc).inBuffSize = buffInSize;
    (*zc).inBuff = ZSTD_cwksp_reserve_buffer(ws, buffInSize);
    (*zc).outBuffSize = buffOutSize;
    (*zc).outBuff = ZSTD_cwksp_reserve_buffer(ws, buffOutSize);
    if (*params).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        let numBuckets =
            (1) << ((*params).ldmParams.hashLog).wrapping_sub((*params).ldmParams.bucketSizeLog);
        (*zc).ldmState.bucketOffsets = ZSTD_cwksp_reserve_buffer(ws, numBuckets);
        ptr::write_bytes((*zc).ldmState.bucketOffsets, 0, numBuckets);
    }
    ZSTD_referenceExternalSequences(zc, core::ptr::null_mut(), 0);
    (*zc).seqStore.maxNbSeq = maxNbSeq;
    (*zc).seqStore.llCode =
        ZSTD_cwksp_reserve_buffer(ws, maxNbSeq.wrapping_mul(::core::mem::size_of::<u8>()));
    (*zc).seqStore.mlCode =
        ZSTD_cwksp_reserve_buffer(ws, maxNbSeq.wrapping_mul(::core::mem::size_of::<u8>()));
    (*zc).seqStore.ofCode =
        ZSTD_cwksp_reserve_buffer(ws, maxNbSeq.wrapping_mul(::core::mem::size_of::<u8>()));
    (*zc).initialized = 1;
    0
}
pub unsafe fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx) {
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < ZSTD_REP_NUM {
        *((*(*cctx).blockState.prevCBlock).rep)
            .as_mut_ptr()
            .offset(i as isize) = 0;
        i += 1;
    }
}
static attachDictSizeCutoffs: [size_t; 10] = [
    (8 * ((1) << 10)) as size_t,
    (8 * ((1) << 10)) as size_t,
    (16 * ((1) << 10)) as size_t,
    (32 * ((1) << 10)) as size_t,
    (32 * ((1) << 10)) as size_t,
    (32 * ((1) << 10)) as size_t,
    (32 * ((1) << 10)) as size_t,
    (32 * ((1) << 10)) as size_t,
    (8 * ((1) << 10)) as size_t,
    (8 * ((1) << 10)) as size_t,
];
unsafe fn ZSTD_shouldAttachDict(
    cdict: *const ZSTD_CDict,
    params: *const ZSTD_CCtx_params,
    pledgedSrcSize: u64,
) -> core::ffi::c_int {
    let cutoff = *attachDictSizeCutoffs
        .as_ptr()
        .offset((*cdict).matchState.cParams.strategy as isize);
    let dedicatedDictSearch = (*cdict).matchState.dedicatedDictSearch;
    core::ffi::c_int::from(
        dedicatedDictSearch != 0
            || (pledgedSrcSize <= cutoff as u64
                || pledgedSrcSize as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
                || (*params).attachDictPref == ZSTD_dictAttachPref_e::ZSTD_dictForceAttach)
                && (*params).attachDictPref != ZSTD_dictAttachPref_e::ZSTD_dictForceCopy
                && (*params).forceWindow == 0,
    )
}
unsafe fn ZSTD_resetCCtx_byAttachingCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let mut adjusted_cdict_cParams = (*cdict).matchState.cParams;
    let windowLog = params.cParams.windowLog;
    if (*cdict).matchState.dedicatedDictSearch != 0 {
        ZSTD_dedicatedDictSearch_revertCParams(&mut adjusted_cdict_cParams);
    }
    params.cParams = ZSTD_adjustCParams_internal(
        adjusted_cdict_cParams,
        pledgedSrcSize as core::ffi::c_ulonglong,
        (*cdict).dictContentSize,
        ZSTD_cpm_attachDict,
        params.useRowMatchFinder,
    );
    params.cParams.windowLog = windowLog;
    params.useRowMatchFinder = (*cdict).useRowMatchFinder;
    let err_code =
        ZSTD_resetCCtx_internal(cctx, &params, pledgedSrcSize, 0, ZSTDcrp_makeClean, zbuff);
    if ERR_isError(err_code) {
        return err_code;
    }
    let cdictEnd = ((*cdict).matchState.window.nextSrc).offset_from((*cdict).matchState.window.base)
        as core::ffi::c_long as u32;
    let cdictLen = cdictEnd.wrapping_sub((*cdict).matchState.window.dictLimit);
    if cdictLen != 0 {
        (*cctx).blockState.matchState.dictMatchState = &(*cdict).matchState;
        if (*cctx).blockState.matchState.window.dictLimit < cdictEnd {
            (*cctx).blockState.matchState.window.nextSrc =
                ((*cctx).blockState.matchState.window.base).wrapping_offset(cdictEnd as isize);
            ZSTD_window_clear(&mut (*cctx).blockState.matchState.window);
        }
        (*cctx).blockState.matchState.loadedDictEnd =
            (*cctx).blockState.matchState.window.dictLimit;
    }
    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;
    libc::memcpy(
        (*cctx).blockState.prevCBlock as *mut core::ffi::c_void,
        &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>(),
    );
    0
}
unsafe fn ZSTD_copyCDictTableIntoCCtx(
    dst: *mut u32,
    src: *const u32,
    tableSize: size_t,
    cParams: *const ZSTD_compressionParameters,
) {
    if ZSTD_CDictIndicesAreTagged(cParams) != 0 {
        let mut i: size_t = 0;
        i = 0;
        while i < tableSize {
            let taggedIndex = *src.add(i);
            let index = taggedIndex >> ZSTD_SHORT_CACHE_TAG_BITS;
            *dst.add(i) = index;
            i = i.wrapping_add(1);
        }
    } else {
        libc::memcpy(
            dst as *mut core::ffi::c_void,
            src as *const core::ffi::c_void,
            tableSize.wrapping_mul(::core::mem::size_of::<u32>()),
        );
    };
}
unsafe fn ZSTD_resetCCtx_byCopyingCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let cdict_cParams: *const ZSTD_compressionParameters = &(*cdict).matchState.cParams;
    let windowLog = params.cParams.windowLog;
    params.cParams = *cdict_cParams;
    params.cParams.windowLog = windowLog;
    params.useRowMatchFinder = (*cdict).useRowMatchFinder;
    let err_code =
        ZSTD_resetCCtx_internal(cctx, &params, pledgedSrcSize, 0, ZSTDcrp_leaveDirty, zbuff);
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_cwksp_mark_tables_dirty(&mut (*cctx).workspace);
    let chainSize =
        if ZSTD_allocateChainTable((*cdict_cParams).strategy, (*cdict).useRowMatchFinder, 0) != 0 {
            (1) << (*cdict_cParams).chainLog
        } else {
            0
        };
    let hSize = (1) << (*cdict_cParams).hashLog;
    ZSTD_copyCDictTableIntoCCtx(
        (*cctx).blockState.matchState.hashTable,
        (*cdict).matchState.hashTable,
        hSize,
        cdict_cParams,
    );
    if ZSTD_allocateChainTable(
        (*cctx).appliedParams.cParams.strategy,
        (*cctx).appliedParams.useRowMatchFinder,
        0,
    ) != 0
    {
        ZSTD_copyCDictTableIntoCCtx(
            (*cctx).blockState.matchState.chainTable,
            (*cdict).matchState.chainTable,
            chainSize,
            cdict_cParams,
        );
    }
    if ZSTD_rowMatchFinderUsed((*cdict_cParams).strategy, (*cdict).useRowMatchFinder) != 0 {
        let tagTableSize = hSize;
        libc::memcpy(
            (*cctx).blockState.matchState.tagTable as *mut core::ffi::c_void,
            (*cdict).matchState.tagTable as *const core::ffi::c_void,
            tagTableSize as libc::size_t,
        );
        (*cctx).blockState.matchState.hashSalt = (*cdict).matchState.hashSalt;
    }
    let h3log = (*cctx).blockState.matchState.hashLog3;
    let h3Size = if h3log != 0 {
        (1 as size_t) << h3log
    } else {
        0 as size_t
    };
    ptr::write_bytes(
        (*cctx).blockState.matchState.hashTable3 as *mut u8,
        0,
        h3Size.wrapping_mul(::core::mem::size_of::<u32>()),
    );
    ZSTD_cwksp_mark_tables_clean(&mut (*cctx).workspace);
    let srcMatchState: *const ZSTD_MatchState_t = &(*cdict).matchState;
    let dstMatchState: &mut ZSTD_MatchState_t = &mut (*cctx).blockState.matchState;
    dstMatchState.window = (*srcMatchState).window;
    dstMatchState.nextToUpdate = (*srcMatchState).nextToUpdate;
    dstMatchState.loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;
    libc::memcpy(
        (*cctx).blockState.prevCBlock as *mut core::ffi::c_void,
        &(*cdict).cBlockState as *const ZSTD_compressedBlockState_t as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>(),
    );
    0
}
unsafe fn ZSTD_resetCCtx_usingCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    params: *const ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    if ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) != 0 {
        ZSTD_resetCCtx_byAttachingCDict(cctx, cdict, *params, pledgedSrcSize, zbuff)
    } else {
        ZSTD_resetCCtx_byCopyingCDict(cctx, cdict, *params, pledgedSrcSize, zbuff)
    }
}
unsafe fn ZSTD_copyCCtx_internal(
    dstCCtx: *mut ZSTD_CCtx,
    srcCCtx: *const ZSTD_CCtx,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: u64,
    zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    if (*srcCCtx).stage as core::ffi::c_uint != ZSTDcs_init as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    libc::memcpy(
        &mut (*dstCCtx).customMem as *mut ZSTD_customMem as *mut core::ffi::c_void,
        &(*srcCCtx).customMem as *const ZSTD_customMem as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_customMem>(),
    );
    let mut params = (*dstCCtx).requestedParams;
    params.cParams = (*srcCCtx).appliedParams.cParams;
    params.useRowMatchFinder = (*srcCCtx).appliedParams.useRowMatchFinder;
    params.postBlockSplitter = (*srcCCtx).appliedParams.postBlockSplitter;
    params.ldmParams = (*srcCCtx).appliedParams.ldmParams;
    params.fParams = fParams;
    params.maxBlockSize = (*srcCCtx).appliedParams.maxBlockSize;
    ZSTD_resetCCtx_internal(
        dstCCtx,
        &params,
        pledgedSrcSize,
        0,
        ZSTDcrp_leaveDirty,
        zbuff,
    );
    ZSTD_cwksp_mark_tables_dirty(&mut (*dstCCtx).workspace);
    let chainSize = if ZSTD_allocateChainTable(
        (*srcCCtx).appliedParams.cParams.strategy,
        (*srcCCtx).appliedParams.useRowMatchFinder,
        0,
    ) != 0
    {
        (1 as size_t) << (*srcCCtx).appliedParams.cParams.chainLog
    } else {
        0 as size_t
    };
    let hSize = (1 as size_t) << (*srcCCtx).appliedParams.cParams.hashLog;
    let h3log = (*srcCCtx).blockState.matchState.hashLog3;
    let h3Size = if h3log != 0 {
        (1 as size_t) << h3log
    } else {
        0 as size_t
    };
    libc::memcpy(
        (*dstCCtx).blockState.matchState.hashTable as *mut core::ffi::c_void,
        (*srcCCtx).blockState.matchState.hashTable as *const core::ffi::c_void,
        hSize.wrapping_mul(::core::mem::size_of::<u32>()) as libc::size_t,
    );
    libc::memcpy(
        (*dstCCtx).blockState.matchState.chainTable as *mut core::ffi::c_void,
        (*srcCCtx).blockState.matchState.chainTable as *const core::ffi::c_void,
        chainSize.wrapping_mul(::core::mem::size_of::<u32>()) as libc::size_t,
    );
    libc::memcpy(
        (*dstCCtx).blockState.matchState.hashTable3 as *mut core::ffi::c_void,
        (*srcCCtx).blockState.matchState.hashTable3 as *const core::ffi::c_void,
        h3Size.wrapping_mul(::core::mem::size_of::<u32>()) as libc::size_t,
    );
    ZSTD_cwksp_mark_tables_clean(&mut (*dstCCtx).workspace);
    let srcMatchState: *const ZSTD_MatchState_t = &(*srcCCtx).blockState.matchState;
    let dstMatchState: &mut ZSTD_MatchState_t = &mut (*dstCCtx).blockState.matchState;
    dstMatchState.window = (*srcMatchState).window;
    dstMatchState.nextToUpdate = (*srcMatchState).nextToUpdate;
    dstMatchState.loadedDictEnd = (*srcMatchState).loadedDictEnd;
    (*dstCCtx).dictID = (*srcCCtx).dictID;
    (*dstCCtx).dictContentSize = (*srcCCtx).dictContentSize;
    libc::memcpy(
        (*dstCCtx).blockState.prevCBlock as *mut core::ffi::c_void,
        (*srcCCtx).blockState.prevCBlock as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_compressedBlockState_t>() as libc::size_t,
    );
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_copyCCtx))]
pub unsafe extern "C" fn ZSTD_copyCCtx(
    dstCCtx: *mut ZSTD_CCtx,
    srcCCtx: *const ZSTD_CCtx,
    mut pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    let mut fParams = {
        ZSTD_frameParameters {
            contentSizeFlag: 1,
            checksumFlag: 0,
            noDictIDFlag: 0,
        }
    };
    let zbuff = (*srcCCtx).bufferedPolicy;
    if pledgedSrcSize == 0 {
        pledgedSrcSize = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    fParams.contentSizeFlag = core::ffi::c_int::from(pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN);
    ZSTD_copyCCtx_internal(dstCCtx, srcCCtx, fParams, pledgedSrcSize, zbuff)
}
pub const ZSTD_ROWSIZE: core::ffi::c_int = 16;
#[inline(always)]
unsafe fn ZSTD_reduceTable_internal(
    table: *mut u32,
    size: u32,
    reducerValue: u32,
    preserveMark: core::ffi::c_int,
) {
    let nbRows = size as core::ffi::c_int / ZSTD_ROWSIZE;
    let mut cellNb = 0;
    let mut rowNb: core::ffi::c_int = 0;
    let reducerThreshold = reducerValue.wrapping_add(ZSTD_WINDOW_START_INDEX as u32);
    rowNb = 0;
    while rowNb < nbRows {
        let mut column: core::ffi::c_int = 0;
        column = 0;
        while column < ZSTD_ROWSIZE {
            let mut newVal: u32 = 0;
            if preserveMark != 0 && *table.offset(cellNb as isize) == ZSTD_DUBT_UNSORTED_MARK as u32
            {
                newVal = ZSTD_DUBT_UNSORTED_MARK as u32;
            } else if *table.offset(cellNb as isize) < reducerThreshold {
                newVal = 0;
            } else {
                newVal = (*table.offset(cellNb as isize)).wrapping_sub(reducerValue);
            }
            *table.offset(cellNb as isize) = newVal;
            cellNb += 1;
            column += 1;
        }
        rowNb += 1;
    }
}
unsafe fn ZSTD_reduceTable(table: *mut u32, size: u32, reducerValue: u32) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 0);
}
unsafe fn ZSTD_reduceTable_btlazy2(table: *mut u32, size: u32, reducerValue: u32) {
    ZSTD_reduceTable_internal(table, size, reducerValue, 1);
}
unsafe fn ZSTD_reduceIndex(
    ms: &mut ZSTD_MatchState_t,
    params: *const ZSTD_CCtx_params,
    reducerValue: u32,
) {
    let hSize = (1) << (*params).cParams.hashLog;
    ZSTD_reduceTable(ms.hashTable, hSize, reducerValue);
    if ZSTD_allocateChainTable(
        (*params).cParams.strategy,
        (*params).useRowMatchFinder,
        ms.dedicatedDictSearch as u32,
    ) != 0
    {
        let chainSize = (1) << (*params).cParams.chainLog;
        if (*params).cParams.strategy as core::ffi::c_uint
            == ZSTD_btlazy2 as core::ffi::c_int as core::ffi::c_uint
        {
            ZSTD_reduceTable_btlazy2(ms.chainTable, chainSize, reducerValue);
        } else {
            ZSTD_reduceTable(ms.chainTable, chainSize, reducerValue);
        }
    }
    if ms.hashLog3 != 0 {
        let h3Size = (1) << ms.hashLog3;
        ZSTD_reduceTable(ms.hashTable3, h3Size, reducerValue);
    }
}
pub unsafe fn ZSTD_seqToCodes(seqStorePtr: *const SeqStore_t) -> core::ffi::c_int {
    let sequences: *const SeqDef = (*seqStorePtr).sequencesStart;
    let llCodeTable = (*seqStorePtr).llCode;
    let ofCodeTable = (*seqStorePtr).ofCode;
    let mlCodeTable = (*seqStorePtr).mlCode;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as core::ffi::c_long as u32;
    let mut u: u32 = 0;
    let mut longOffsets = 0;
    u = 0;
    while u < nbSeq {
        let llv = u32::from((*sequences.offset(u as isize)).litLength);
        let ofCode = ZSTD_highbit32((*sequences.offset(u as isize)).offBase);
        let mlv = u32::from((*sequences.offset(u as isize)).mlBase);
        *llCodeTable.offset(u as isize) = ZSTD_LLcode(llv) as u8;
        *ofCodeTable.offset(u as isize) = ofCode as u8;
        *mlCodeTable.offset(u as isize) = ZSTD_MLcode(mlv) as u8;
        if MEM_32bits()
            && ofCode
                >= (if MEM_32bits() {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as u32
        {
            longOffsets = 1;
        }
        u = u.wrapping_add(1);
    }
    if (*seqStorePtr).longLengthType == ZSTD_llt_literalLength {
        *llCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxLL as u8;
    }
    if (*seqStorePtr).longLengthType == ZSTD_llt_matchLength {
        *mlCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxML as u8;
    }
    longOffsets
}
unsafe fn ZSTD_useTargetCBlockSize(cctxParams: *const ZSTD_CCtx_params) -> core::ffi::c_int {
    core::ffi::c_int::from((*cctxParams).targetCBlockSize != 0)
}
unsafe fn ZSTD_blockSplitterEnabled(cctxParams: *mut ZSTD_CCtx_params) -> core::ffi::c_int {
    core::ffi::c_int::from((*cctxParams).postBlockSplitter == ZSTD_ParamSwitch_e::ZSTD_ps_enable)
}
unsafe fn ZSTD_buildSequencesStatistics(
    seqStorePtr: *const SeqStore_t,
    nbSeq: size_t,
    prevEntropy: *const ZSTD_fseCTables_t,
    nextEntropy: *mut ZSTD_fseCTables_t,
    dst: *mut u8,
    dstEnd: *const u8,
    strategy: ZSTD_strategy,
    countWorkspace: *mut core::ffi::c_uint,
    entropyWorkspace: *mut core::ffi::c_void,
    entropyWkspSize: size_t,
) -> ZSTD_symbolEncodingTypeStats_t {
    let ostart = dst;
    let oend = dstEnd;
    let mut op = ostart;
    let CTable_LitLength = ((*nextEntropy).litlengthCTable).as_mut_ptr();
    let CTable_OffsetBits = ((*nextEntropy).offcodeCTable).as_mut_ptr();
    let CTable_MatchLength = ((*nextEntropy).matchlengthCTable).as_mut_ptr();
    let ofCodeTable: *const u8 = (*seqStorePtr).ofCode;
    let llCodeTable: *const u8 = (*seqStorePtr).llCode;
    let mlCodeTable: *const u8 = (*seqStorePtr).mlCode;
    let mut stats = ZSTD_symbolEncodingTypeStats_t {
        LLtype: 0,
        Offtype: 0,
        MLtype: 0,
        size: 0,
        lastCountSize: 0,
        longOffsets: 0,
    };
    stats.lastCountSize = 0;
    stats.longOffsets = ZSTD_seqToCodes(seqStorePtr);
    let mut max = MaxLL;
    let mostFrequent = HIST_countFast_wksp(
        countWorkspace,
        &mut max,
        llCodeTable as *const core::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    (*nextEntropy).litlength_repeatMode = (*prevEntropy).litlength_repeatMode;
    stats.LLtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).litlength_repeatMode,
        countWorkspace,
        max,
        mostFrequent,
        nbSeq,
        LLFSELog,
        ((*prevEntropy).litlengthCTable).as_ptr(),
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        ZSTD_defaultAllowed,
        strategy,
    ) as u32;
    let countSize = ZSTD_buildCTable(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        CTable_LitLength,
        LLFSELog,
        stats.LLtype as SymbolEncodingType_e,
        countWorkspace,
        max,
        llCodeTable,
        nbSeq,
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        MaxLL,
        ((*prevEntropy).litlengthCTable).as_ptr(),
        ::core::mem::size_of::<[FSE_CTable; 329]>(),
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize) {
        stats.size = countSize;
        return stats;
    }
    if stats.LLtype == set_compressed as core::ffi::c_int as u32 {
        stats.lastCountSize = countSize;
    }
    op = op.add(countSize);
    let mut max_0 = MaxOff;
    let mostFrequent_0 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_0,
        ofCodeTable as *const core::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    let defaultPolicy = (if max_0 <= DefaultMaxOff {
        ZSTD_defaultAllowed as core::ffi::c_int
    } else {
        ZSTD_defaultDisallowed as core::ffi::c_int
    }) as ZSTD_DefaultPolicy_e;
    (*nextEntropy).offcode_repeatMode = (*prevEntropy).offcode_repeatMode;
    stats.Offtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).offcode_repeatMode,
        countWorkspace,
        max_0,
        mostFrequent_0,
        nbSeq,
        OffFSELog,
        ((*prevEntropy).offcodeCTable).as_ptr(),
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        defaultPolicy,
        strategy,
    ) as u32;
    let countSize_0 = ZSTD_buildCTable(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        CTable_OffsetBits,
        OffFSELog,
        stats.Offtype as SymbolEncodingType_e,
        countWorkspace,
        max_0,
        ofCodeTable,
        nbSeq,
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        DefaultMaxOff,
        ((*prevEntropy).offcodeCTable).as_ptr(),
        ::core::mem::size_of::<[FSE_CTable; 193]>(),
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_0) {
        stats.size = countSize_0;
        return stats;
    }
    if stats.Offtype == set_compressed as core::ffi::c_int as u32 {
        stats.lastCountSize = countSize_0;
    }
    op = op.add(countSize_0);
    let mut max_1 = MaxML;
    let mostFrequent_1 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_1,
        mlCodeTable as *const core::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    (*nextEntropy).matchlength_repeatMode = (*prevEntropy).matchlength_repeatMode;
    stats.MLtype = ZSTD_selectEncodingType(
        &mut (*nextEntropy).matchlength_repeatMode,
        countWorkspace,
        max_1,
        mostFrequent_1,
        nbSeq,
        MLFSELog,
        ((*prevEntropy).matchlengthCTable).as_ptr(),
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        ZSTD_defaultAllowed,
        strategy,
    ) as u32;
    let countSize_1 = ZSTD_buildCTable(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        CTable_MatchLength,
        MLFSELog,
        stats.MLtype as SymbolEncodingType_e,
        countWorkspace,
        max_1,
        mlCodeTable,
        nbSeq,
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        MaxML,
        ((*prevEntropy).matchlengthCTable).as_ptr(),
        ::core::mem::size_of::<[FSE_CTable; 363]>(),
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_1) {
        stats.size = countSize_1;
        return stats;
    }
    if stats.MLtype == set_compressed as core::ffi::c_int as u32 {
        stats.lastCountSize = countSize_1;
    }
    op = op.add(countSize_1);
    stats.size = op.offset_from_unsigned(ostart);
    stats
}
pub const SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO: core::ffi::c_int = 20;
#[inline]
unsafe fn ZSTD_entropyCompressSeqStore_internal(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    literals: *const core::ffi::c_void,
    litSize: size_t,
    seqStorePtr: *const SeqStore_t,
    prevEntropy: *const ZSTD_entropyCTables_t,
    nextEntropy: *mut ZSTD_entropyCTables_t,
    cctxParams: *const ZSTD_CCtx_params,
    mut entropyWorkspace: *mut core::ffi::c_void,
    mut entropyWkspSize: size_t,
    bmi2: core::ffi::c_int,
) -> size_t {
    let strategy = (*cctxParams).cParams.strategy;
    let count = entropyWorkspace as *mut core::ffi::c_uint;
    let CTable_LitLength = (&raw const ((*nextEntropy).fse.litlengthCTable)).cast::<u32>();
    let CTable_OffsetBits = (&raw const ((*nextEntropy).fse.offcodeCTable)).cast::<u32>();
    let CTable_MatchLength = (&raw const ((*nextEntropy).fse.matchlengthCTable)).cast::<u32>();
    let sequences: *const SeqDef = (*seqStorePtr).sequencesStart;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart) as size_t;
    let ofCodeTable: *const u8 = (*seqStorePtr).ofCode;
    let llCodeTable: *const u8 = (*seqStorePtr).llCode;
    let mlCodeTable: *const u8 = (*seqStorePtr).mlCode;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstCapacity);
    let mut op = ostart;
    let mut lastCountSize: size_t = 0;
    let mut longOffsets = 0;
    entropyWorkspace =
        count.offset(((if 35 > 52 { 35 } else { 52 }) + 1) as isize) as *mut core::ffi::c_void;
    entropyWkspSize = (entropyWkspSize as size_t).wrapping_sub(
        ((if 35 > 52 { 35 as size_t } else { 52 }) + 1)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>()),
    );
    let numSequences =
        ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart) as size_t;
    let suspectUncompressible = core::ffi::c_int::from(
        numSequences == 0
            || litSize / numSequences >= SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO as size_t,
    );
    let cSize = ZSTD_compressLiterals(
        op as *mut core::ffi::c_void,
        dstCapacity,
        literals,
        litSize,
        entropyWorkspace,
        entropyWkspSize,
        &(*prevEntropy).huf,
        &mut (*nextEntropy).huf,
        (*cctxParams).cParams.strategy,
        ZSTD_literalsCompressionIsDisabled(cctxParams),
        suspectUncompressible,
        bmi2,
    );
    let err_code = cSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    op = op.add(cSize);
    if (oend.offset_from(op) as core::ffi::c_long) < core::ffi::c_long::from(3 + 1) {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if nbSeq < 128 {
        let fresh2 = op;
        op = op.add(1);
        *fresh2 = nbSeq as u8;
    } else if nbSeq < LONGNBSEQ as size_t {
        *op = (nbSeq >> 8).wrapping_add(0x80 as core::ffi::c_int as size_t) as u8;
        *op.add(1) = nbSeq as u8;
        op = op.add(2);
    } else {
        *op = 0xff as core::ffi::c_int as u8;
        MEM_writeLE16(
            op.add(1) as *mut core::ffi::c_void,
            nbSeq.wrapping_sub(LONGNBSEQ as size_t) as u16,
        );
        op = op.add(3);
    }
    if nbSeq == 0 {
        libc::memcpy(
            &mut (*nextEntropy).fse as *mut ZSTD_fseCTables_t as *mut core::ffi::c_void,
            &(*prevEntropy).fse as *const ZSTD_fseCTables_t as *const core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_fseCTables_t>(),
        );
        return op.offset_from_unsigned(ostart);
    }
    let fresh3 = op;
    op = op.add(1);
    let seqHead = fresh3;
    let stats = ZSTD_buildSequencesStatistics(
        seqStorePtr,
        nbSeq,
        &(*prevEntropy).fse,
        &mut (*nextEntropy).fse,
        op,
        oend,
        strategy,
        count,
        entropyWorkspace,
        entropyWkspSize,
    );
    let err_code_0 = stats.size;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    *seqHead = (stats.LLtype << 6)
        .wrapping_add(stats.Offtype << 4)
        .wrapping_add(stats.MLtype << 2) as u8;
    lastCountSize = stats.lastCountSize;
    op = op.add(stats.size);
    longOffsets = stats.longOffsets;
    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
        bmi2,
    );
    let err_code_1 = bitstreamSize;
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    op = op.add(bitstreamSize);
    if lastCountSize != 0 && lastCountSize.wrapping_add(bitstreamSize) < 4 {
        return 0;
    }
    op.offset_from_unsigned(ostart)
}
unsafe fn ZSTD_entropyCompressSeqStore_wExtLitBuffer(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    literals: *const core::ffi::c_void,
    litSize: size_t,
    blockSize: size_t,
    seqStorePtr: *const SeqStore_t,
    prevEntropy: *const ZSTD_entropyCTables_t,
    nextEntropy: *mut ZSTD_entropyCTables_t,
    cctxParams: *const ZSTD_CCtx_params,
    entropyWorkspace: *mut core::ffi::c_void,
    entropyWkspSize: size_t,
    bmi2: core::ffi::c_int,
) -> size_t {
    let cSize = ZSTD_entropyCompressSeqStore_internal(
        dst,
        dstCapacity,
        literals,
        litSize,
        seqStorePtr,
        prevEntropy,
        nextEntropy,
        cctxParams,
        entropyWorkspace,
        entropyWkspSize,
        bmi2,
    );
    if cSize == 0 {
        return 0;
    }
    if core::ffi::c_int::from(cSize == Error::dstSize_tooSmall.to_error_code())
        & core::ffi::c_int::from(blockSize <= dstCapacity)
        != 0
    {
        return 0;
    }
    let err_code = cSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    let maxCSize = blockSize.wrapping_sub(ZSTD_minGain(blockSize, (*cctxParams).cParams.strategy));
    if cSize >= maxCSize {
        return 0;
    }
    cSize
}
unsafe fn ZSTD_entropyCompressSeqStore(
    seqStorePtr: *const SeqStore_t,
    prevEntropy: *const ZSTD_entropyCTables_t,
    nextEntropy: *mut ZSTD_entropyCTables_t,
    cctxParams: *const ZSTD_CCtx_params,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    srcSize: size_t,
    entropyWorkspace: *mut core::ffi::c_void,
    entropyWkspSize: size_t,
    bmi2: core::ffi::c_int,
) -> size_t {
    ZSTD_entropyCompressSeqStore_wExtLitBuffer(
        dst,
        dstCapacity,
        (*seqStorePtr).litStart as *const core::ffi::c_void,
        ((*seqStorePtr).lit).offset_from((*seqStorePtr).litStart) as size_t,
        srcSize,
        seqStorePtr,
        prevEntropy,
        nextEntropy,
        cctxParams,
        entropyWorkspace,
        entropyWkspSize,
        bmi2,
    )
}
pub unsafe fn ZSTD_selectBlockCompressor(
    strat: ZSTD_strategy,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    dictMode: ZSTD_dictMode_e,
) -> ZSTD_BlockCompressor_f {
    static blockCompressor: [[ZSTD_BlockCompressor_f; 10]; 4] = [
        [
            Some(
                ZSTD_compressBlock_fast
                    as unsafe fn(
                        &mut ZSTD_MatchState_t,
                        &mut SeqStore_t,
                        *mut u32,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(
                ZSTD_compressBlock_fast
                    as unsafe fn(
                        &mut ZSTD_MatchState_t,
                        &mut SeqStore_t,
                        *mut u32,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(ZSTD_COMPRESSBLOCK_DOUBLEFAST),
            Some(ZSTD_COMPRESSBLOCK_GREEDY),
            Some(ZSTD_COMPRESSBLOCK_LAZY),
            Some(ZSTD_COMPRESSBLOCK_LAZY2),
            Some(ZSTD_COMPRESSBLOCK_BTLAZY2),
            Some(ZSTD_COMPRESSBLOCK_BTOPT),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA2),
        ],
        [
            Some(
                ZSTD_compressBlock_fast_extDict
                    as unsafe fn(
                        &mut ZSTD_MatchState_t,
                        &mut SeqStore_t,
                        *mut u32,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(
                ZSTD_compressBlock_fast_extDict
                    as unsafe fn(
                        &mut ZSTD_MatchState_t,
                        &mut SeqStore_t,
                        *mut u32,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(ZSTD_COMPRESSBLOCK_DOUBLEFAST_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_LAZY_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTLAZY2_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTOPT_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_EXTDICT),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_EXTDICT),
        ],
        [
            Some(
                ZSTD_compressBlock_fast_dictMatchState
                    as unsafe fn(
                        &mut ZSTD_MatchState_t,
                        &mut SeqStore_t,
                        *mut u32,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(
                ZSTD_compressBlock_fast_dictMatchState
                    as unsafe fn(
                        &mut ZSTD_MatchState_t,
                        &mut SeqStore_t,
                        *mut u32,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(ZSTD_COMPRESSBLOCK_DOUBLEFAST_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTLAZY2_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTOPT_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_DICTMATCHSTATE),
            Some(ZSTD_COMPRESSBLOCK_BTULTRA_DICTMATCHSTATE),
        ],
        [
            None,
            None,
            None,
            Some(ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH),
            Some(ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH),
            Some(ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH),
            None,
            None,
            None,
            None,
        ],
    ];
    let mut selectedCompressor: ZSTD_BlockCompressor_f = None;
    if ZSTD_rowMatchFinderUsed(strat, useRowMatchFinder) != 0 {
        static rowBasedBlockCompressors: [[ZSTD_BlockCompressor_f; 3]; 4] = [
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_ROW),
            ],
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_EXTDICT_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_EXTDICT_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_EXTDICT_ROW),
            ],
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_DICTMATCHSTATE_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_DICTMATCHSTATE_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_DICTMATCHSTATE_ROW),
            ],
            [
                Some(ZSTD_COMPRESSBLOCK_GREEDY_DEDICATEDDICTSEARCH_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY_DEDICATEDDICTSEARCH_ROW),
                Some(ZSTD_COMPRESSBLOCK_LAZY2_DEDICATEDDICTSEARCH_ROW),
            ],
        ];
        selectedCompressor = *(*rowBasedBlockCompressors
            .as_ptr()
            .offset(dictMode as core::ffi::c_int as isize))
        .as_ptr()
        .offset((strat as core::ffi::c_int - ZSTD_greedy as core::ffi::c_int) as isize);
    } else {
        selectedCompressor = *(*blockCompressor
            .as_ptr()
            .offset(dictMode as core::ffi::c_int as isize))
        .as_ptr()
        .offset(strat as core::ffi::c_int as isize);
    }
    selectedCompressor
}
unsafe fn ZSTD_storeLastLiterals(
    seqStorePtr: &mut SeqStore_t,
    anchor: *const u8,
    lastLLSize: size_t,
) {
    libc::memcpy(
        seqStorePtr.lit as *mut core::ffi::c_void,
        anchor as *const core::ffi::c_void,
        lastLLSize as libc::size_t,
    );
    seqStorePtr.lit = (seqStorePtr.lit).add(lastLLSize);
}
pub unsafe fn ZSTD_resetSeqStore(ssPtr: &mut SeqStore_t) {
    ssPtr.lit = ssPtr.litStart;
    ssPtr.sequences = ssPtr.sequencesStart;
    ssPtr.longLengthType = ZSTD_llt_none;
}
unsafe fn ZSTD_postProcessSequenceProducerResult(
    outSeqs: *mut ZSTD_Sequence,
    nbExternalSeqs: size_t,
    outSeqsCapacity: size_t,
    srcSize: size_t,
) -> size_t {
    if nbExternalSeqs > outSeqsCapacity {
        return Error::sequenceProducer_failed.to_error_code();
    }
    if nbExternalSeqs == 0 && srcSize > 0 {
        return Error::sequenceProducer_failed.to_error_code();
    }
    if srcSize == 0 {
        ptr::write_bytes(
            &mut *outSeqs as *mut ZSTD_Sequence as *mut u8,
            0,
            ::core::mem::size_of::<ZSTD_Sequence>(),
        );
        return 1;
    }
    let lastSeq = *outSeqs.add(nbExternalSeqs.wrapping_sub(1));
    if lastSeq.offset == 0 && lastSeq.matchLength == 0 {
        return nbExternalSeqs;
    }
    if nbExternalSeqs == outSeqsCapacity {
        return Error::sequenceProducer_failed.to_error_code();
    }
    ptr::write_bytes(
        &mut *outSeqs.add(nbExternalSeqs) as *mut ZSTD_Sequence as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_Sequence>(),
    );
    nbExternalSeqs.wrapping_add(1)
}
unsafe fn ZSTD_fastSequenceLengthSum(seqBuf: *const ZSTD_Sequence, seqBufSize: size_t) -> size_t {
    let mut matchLenSum: size_t = 0;
    let mut litLenSum: size_t = 0;
    let mut i: size_t = 0;
    matchLenSum = 0;
    litLenSum = 0;
    i = 0;
    while i < seqBufSize {
        litLenSum = litLenSum.wrapping_add((*seqBuf.add(i)).litLength as size_t);
        matchLenSum = matchLenSum.wrapping_add((*seqBuf.add(i)).matchLength as size_t);
        i = i.wrapping_add(1);
    }
    litLenSum.wrapping_add(matchLenSum)
}

unsafe fn ZSTD_validateSeqStore(
    seqStore: *const SeqStore_t,
    cParams: *const ZSTD_compressionParameters,
) {
    let matchLenLowerBound = match (*cParams).minMatch {
        3 => 3,
        _ => 4,
    };

    let start = (*seqStore).sequences;
    let end = (*seqStore).sequences;

    if cfg!(debug_assertions) {
        for n in 0..end as usize - start as usize {
            let seqLength = ZSTD_getSequenceLength(seqStore, start.add(n));
            debug_assert!(seqLength.matchLength >= matchLenLowerBound);
        }
    }
}
unsafe fn ZSTD_buildSeqStore(
    zc: *mut ZSTD_CCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let ms: &mut ZSTD_MatchState_t = &mut (*zc).blockState.matchState;
    ZSTD_assertEqualCParams((*zc).appliedParams.cParams, ms.cParams);
    if srcSize
        < (MIN_CBLOCK_SIZE as size_t)
            .wrapping_add(ZSTD_blockHeaderSize)
            .wrapping_add(1)
            .wrapping_add(1)
    {
        if (*zc).appliedParams.cParams.strategy as core::ffi::c_uint
            >= ZSTD_btopt as core::ffi::c_int as core::ffi::c_uint
        {
            ZSTD_ldm_skipRawSeqStoreBytes(&mut (*zc).externSeqStore, srcSize);
        } else {
            ZSTD_ldm_skipSequences(
                &mut (*zc).externSeqStore,
                srcSize,
                (*zc).appliedParams.cParams.minMatch,
            );
        }
        return ZSTDbss_noCompress as core::ffi::c_int as size_t;
    }
    ZSTD_resetSeqStore(&mut (*zc).seqStore);
    ms.opt.symbolCosts = &mut (*(*zc).blockState.prevCBlock).entropy;
    ms.opt.literalCompressionMode = (*zc).appliedParams.literalCompressionMode;
    let base = ms.window.base;
    let istart = src as *const u8;
    let curr = istart.wrapping_offset_from(base) as core::ffi::c_long as u32;

    if size_of::<ptrdiff_t>() == 8 {
        assert!(istart.wrapping_offset_from(base) < u32::MAX as ptrdiff_t); /* ensure no overflow */
    }

    if curr > (ms.nextToUpdate).wrapping_add(384) {
        ms.nextToUpdate = curr.wrapping_sub(Ord::min(
            192,
            curr.wrapping_sub(ms.nextToUpdate).wrapping_sub(384),
        ));
    }

    /* select and store sequences */
    let dictMode = ZSTD_matchState_dictMode(ms);
    let mut lastLLSize: size_t = 0;
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < ZSTD_REP_NUM {
        *((*(*zc).blockState.nextCBlock).rep)
            .as_mut_ptr()
            .offset(i as isize) = *((*(*zc).blockState.prevCBlock).rep)
            .as_mut_ptr()
            .offset(i as isize);
        i += 1;
    }
    if (*zc).externSeqStore.pos < (*zc).externSeqStore.size {
        if ZSTD_hasExtSeqProd(&(*zc).appliedParams) != 0 {
            return Error::parameter_combination_unsupported.to_error_code();
        }
        lastLLSize = ZSTD_ldm_blockCompress(
            &mut (*zc).externSeqStore,
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
    } else if (*zc).appliedParams.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        let mut ldmSeqStore = kNullRawSeqStore;
        if ZSTD_hasExtSeqProd(&(*zc).appliedParams) != 0 {
            return Error::parameter_combination_unsupported.to_error_code();
        }
        ldmSeqStore.seq = (*zc).ldmSequences;
        ldmSeqStore.capacity = (*zc).maxNbLdmSequences;
        let err_code = ZSTD_ldm_generateSequences(
            &mut (*zc).ldmState,
            &mut ldmSeqStore,
            &(*zc).appliedParams.ldmParams,
            src,
            srcSize,
        );
        if ERR_isError(err_code) {
            return err_code;
        }
        lastLLSize = ZSTD_ldm_blockCompress(
            &mut ldmSeqStore,
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
    } else if ZSTD_hasExtSeqProd(&(*zc).appliedParams) != 0 {
        let windowSize = (1) << (*zc).appliedParams.cParams.windowLog;
        let nbExternalSeqs = ((*zc).appliedParams.extSeqProdFunc).unwrap_unchecked()(
            (*zc).appliedParams.extSeqProdState,
            (*zc).extSeqBuf,
            (*zc).extSeqBufCapacity,
            src,
            srcSize,
            core::ptr::null(),
            0,
            (*zc).appliedParams.compressionLevel,
            windowSize as size_t,
        );
        let nbPostProcessedSeqs = ZSTD_postProcessSequenceProducerResult(
            (*zc).extSeqBuf,
            nbExternalSeqs,
            (*zc).extSeqBufCapacity,
            srcSize,
        );
        if !ERR_isError(nbPostProcessedSeqs) {
            let mut seqPos = {
                ZSTD_SequencePosition {
                    idx: 0,
                    posInSequence: 0,
                    posInSrc: 0,
                }
            };
            let seqLenSum = ZSTD_fastSequenceLengthSum((*zc).extSeqBuf, nbPostProcessedSeqs);
            if seqLenSum > srcSize {
                return Error::externalSequences_invalid.to_error_code();
            }
            let err_code_0 = ZSTD_transferSequences_wBlockDelim(
                zc,
                &mut seqPos,
                (*zc).extSeqBuf,
                nbPostProcessedSeqs,
                src,
                srcSize,
                (*zc).appliedParams.searchForExternalRepcodes,
            );
            if ERR_isError(err_code_0) {
                return err_code_0;
            }
            ms.ldmSeqStore = core::ptr::null();
            return ZSTDbss_compress as core::ffi::c_int as size_t;
        }
        if (*zc).appliedParams.enableMatchFinderFallback == 0 {
            return nbPostProcessedSeqs;
        }
        let blockCompressor = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        ms.ldmSeqStore = core::ptr::null();
        lastLLSize = blockCompressor.unwrap_unchecked()(
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            src,
            srcSize,
        );
    } else {
        let blockCompressor_0 = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        ms.ldmSeqStore = core::ptr::null();
        lastLLSize = blockCompressor_0.unwrap_unchecked()(
            ms,
            &mut (*zc).seqStore,
            ((*(*zc).blockState.nextCBlock).rep).as_mut_ptr(),
            src,
            srcSize,
        );
    }
    let lastLiterals = (src as *const u8)
        .add(srcSize)
        .offset(-(lastLLSize as isize));
    ZSTD_storeLastLiterals(&mut (*zc).seqStore, lastLiterals, lastLLSize);
    ZSTD_validateSeqStore(&(*zc).seqStore, &(*zc).appliedParams.cParams);
    ZSTDbss_compress as core::ffi::c_int as size_t
}
unsafe fn ZSTD_copyBlockSequences(
    seqCollector: *mut SeqCollector,
    seqStore: *const SeqStore_t,
    prevRepcodes: *const u32,
) -> size_t {
    let inSeqs: *const SeqDef = (*seqStore).sequencesStart;
    let nbInSequences = ((*seqStore).sequences).offset_from_unsigned(inSeqs);
    let nbInLiterals = ((*seqStore).lit).offset_from((*seqStore).litStart) as size_t;
    let outSeqs = if (*seqCollector).seqIndex == 0 {
        (*seqCollector).seqStart
    } else {
        ((*seqCollector).seqStart).add((*seqCollector).seqIndex)
    };
    let nbOutSequences = nbInSequences.wrapping_add(1);
    let mut nbOutLiterals = 0 as size_t;
    let mut repcodes = repcodes_s { rep: [0; 3] };
    let mut i: size_t = 0;
    if nbOutSequences > ((*seqCollector).maxSequences).wrapping_sub((*seqCollector).seqIndex) {
        return Error::dstSize_tooSmall.to_error_code();
    }
    libc::memcpy(
        &mut repcodes as *mut Repcodes_t as *mut core::ffi::c_void,
        prevRepcodes as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    i = 0;
    while i < nbInSequences {
        let mut rawOffset: u32 = 0;
        (*outSeqs.add(i)).litLength = core::ffi::c_uint::from((*inSeqs.add(i)).litLength);
        (*outSeqs.add(i)).matchLength =
            (core::ffi::c_int::from((*inSeqs.add(i)).mlBase) + MINMATCH) as core::ffi::c_uint;
        (*outSeqs.add(i)).rep = 0;
        if i == (*seqStore).longLengthPos as size_t {
            if (*seqStore).longLengthType == ZSTD_llt_literalLength {
                let fresh4 = &mut (*outSeqs.add(i)).litLength;
                *fresh4 = (*fresh4).wrapping_add(0x10000 as core::ffi::c_int as core::ffi::c_uint);
            } else if (*seqStore).longLengthType == ZSTD_llt_matchLength {
                let fresh5 = &mut (*outSeqs.add(i)).matchLength;
                *fresh5 = (*fresh5).wrapping_add(0x10000 as core::ffi::c_int as core::ffi::c_uint);
            }
        }
        if 1 <= (*inSeqs.add(i)).offBase && (*inSeqs.add(i)).offBase <= ZSTD_REP_NUM as u32 {
            let repcode = (*inSeqs.add(i)).offBase;
            (*outSeqs.add(i)).rep = repcode;
            if (*outSeqs.add(i)).litLength != 0 {
                rawOffset = *(repcodes.rep)
                    .as_mut_ptr()
                    .offset(repcode.wrapping_sub(1) as isize);
            } else if repcode == 3 {
                rawOffset = (*(repcodes.rep).as_mut_ptr()).wrapping_sub(1);
            } else {
                rawOffset = *(repcodes.rep).as_mut_ptr().offset(repcode as isize);
            }
        } else {
            rawOffset = ((*inSeqs.add(i)).offBase).wrapping_sub(ZSTD_REP_NUM as u32);
        }
        (*outSeqs.add(i)).offset = rawOffset;
        ZSTD_updateRep(
            (repcodes.rep).as_mut_ptr(),
            (*inSeqs.add(i)).offBase,
            core::ffi::c_int::from(core::ffi::c_int::from((*inSeqs.add(i)).litLength) == 0) as u32,
        );
        nbOutLiterals = nbOutLiterals.wrapping_add((*outSeqs.add(i)).litLength as size_t);
        i = i.wrapping_add(1);
    }
    let lastLLSize = nbInLiterals.wrapping_sub(nbOutLiterals);
    (*outSeqs.add(nbInSequences)).litLength = lastLLSize as u32;
    (*outSeqs.add(nbInSequences)).matchLength = 0;
    (*outSeqs.add(nbInSequences)).offset = 0;
    (*seqCollector).seqIndex = ((*seqCollector).seqIndex).wrapping_add(nbOutSequences);
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sequenceBound))]
pub unsafe extern "C" fn ZSTD_sequenceBound(srcSize: size_t) -> size_t {
    let maxNbSeq = (srcSize / ZSTD_MINMATCH_MIN as size_t).wrapping_add(1);
    let maxNbDelims = (srcSize / ZSTD_BLOCKSIZE_MAX_MIN as size_t).wrapping_add(1);
    maxNbSeq.wrapping_add(maxNbDelims)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_generateSequences))]
pub unsafe extern "C" fn ZSTD_generateSequences(
    zc: *mut ZSTD_CCtx,
    outSeqs: *mut ZSTD_Sequence,
    outSeqsSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let dstCapacity = ZSTD_compressBound(srcSize);
    let mut dst = core::ptr::null_mut::<core::ffi::c_void>();
    let mut seqCollector = SeqCollector {
        collectSequences: 0,
        seqStart: core::ptr::null_mut::<ZSTD_Sequence>(),
        seqIndex: 0,
        maxSequences: 0,
    };
    let mut targetCBlockSize: core::ffi::c_int = 0;
    let err_code = ZSTD_CCtx_getParameter(
        zc,
        ZSTD_cParameter::ZSTD_c_targetCBlockSize,
        &mut targetCBlockSize,
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    if targetCBlockSize != 0 {
        return Error::parameter_unsupported.to_error_code();
    }
    let mut nbWorkers: core::ffi::c_int = 0;
    let err_code_0 = ZSTD_CCtx_getParameter(zc, ZSTD_cParameter::ZSTD_c_nbWorkers, &mut nbWorkers);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    if nbWorkers != 0 {
        return Error::parameter_unsupported.to_error_code();
    }
    dst = ZSTD_customMalloc(dstCapacity, ZSTD_customMem::default());
    if dst.is_null() {
        return Error::memory_allocation.to_error_code();
    }
    seqCollector.collectSequences = 1;
    seqCollector.seqStart = outSeqs;
    seqCollector.seqIndex = 0;
    seqCollector.maxSequences = outSeqsSize;
    (*zc).seqCollector = seqCollector;
    let ret = ZSTD_compress2(zc, dst, dstCapacity, src, srcSize);
    ZSTD_customFree(dst, dstCapacity, ZSTD_customMem::default());
    let err_code_1 = ret;
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    (*zc).seqCollector.seqIndex
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_mergeBlockDelimiters))]
pub unsafe extern "C" fn ZSTD_mergeBlockDelimiters(
    sequences: *mut ZSTD_Sequence,
    seqsSize: size_t,
) -> size_t {
    let mut in_0 = 0;
    let mut out = 0 as size_t;
    while in_0 < seqsSize {
        if (*sequences.add(in_0)).offset == 0 && (*sequences.add(in_0)).matchLength == 0 {
            if in_0 != seqsSize.wrapping_sub(1) {
                let fresh6 = &mut (*sequences.add(in_0.wrapping_add(1))).litLength;
                *fresh6 = (*fresh6).wrapping_add((*sequences.add(in_0)).litLength);
            }
        } else {
            *sequences.add(out) = *sequences.add(in_0);
            out = out.wrapping_add(1);
        }
        in_0 = in_0.wrapping_add(1);
    }
    out
}
unsafe fn ZSTD_isRLE(src: *const u8, length: size_t) -> core::ffi::c_int {
    let ip = src;
    let value = *ip;
    let valueST = (u64::from(value) as core::ffi::c_ulonglong)
        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong) as size_t;
    let unrollSize = ::core::mem::size_of::<size_t>().wrapping_mul(4);
    let unrollMask = unrollSize.wrapping_sub(1);
    let prefixLength = length & unrollMask;
    let mut i: size_t = 0;
    if length == 1 {
        return 1;
    }
    if prefixLength != 0
        && ZSTD_count(ip.add(1), ip, ip.add(prefixLength)) != prefixLength.wrapping_sub(1)
    {
        return 0;
    }
    i = prefixLength;
    while i != length {
        let mut u: size_t = 0;
        u = 0;
        while u < unrollSize {
            if MEM_readST(ip.add(i).add(u) as *const core::ffi::c_void) != valueST {
                return 0;
            }
            u = (u as size_t).wrapping_add(::core::mem::size_of::<size_t>());
        }
        i = i.wrapping_add(unrollSize);
    }
    1
}
unsafe fn ZSTD_maybeRLE(seqStore: *const SeqStore_t) -> core::ffi::c_int {
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as size_t;
    let nbLits = ((*seqStore).lit).offset_from((*seqStore).litStart) as size_t;
    core::ffi::c_int::from(nbSeqs < 4 && nbLits < 10)
}
unsafe fn ZSTD_blockState_confirmRepcodesAndEntropyTables(bs: *mut ZSTD_blockState_t) {
    core::mem::swap(&mut (*bs).prevCBlock, &mut (*bs).nextCBlock);
}
unsafe fn writeBlockHeader(
    op: *mut core::ffi::c_void,
    cSize: size_t,
    blockSize: size_t,
    lastBlock: u32,
) {
    let cBlockHeader = if cSize == 1 {
        lastBlock
            .wrapping_add((bt_rle as core::ffi::c_int as u32) << 1)
            .wrapping_add((blockSize << 3) as u32)
    } else {
        lastBlock
            .wrapping_add((bt_compressed as core::ffi::c_int as u32) << 1)
            .wrapping_add((cSize << 3) as u32)
    };
    MEM_writeLE24(op, cBlockHeader);
}
unsafe fn ZSTD_buildBlockEntropyStats_literals(
    src: *mut core::ffi::c_void,
    srcSize: size_t,
    prevHuf: *const ZSTD_hufCTables_t,
    nextHuf: *mut ZSTD_hufCTables_t,
    hufMetadata: *mut ZSTD_hufCTablesMetadata_t,
    literalsCompressionIsDisabled: core::ffi::c_int,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    hufFlags: core::ffi::c_int,
) -> size_t {
    let wkspStart = workspace as *mut u8;
    let wkspEnd = wkspStart.add(wkspSize);
    let countWkspStart = wkspStart;
    let countWksp = workspace as *mut core::ffi::c_uint;
    let countWkspSize = ((HUF_SYMBOLVALUE_MAX + 1) as size_t)
        .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>());
    let nodeWksp = countWkspStart.add(countWkspSize);
    let nodeWkspSize = wkspEnd.offset_from_unsigned(nodeWksp);
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX;
    let mut huffLog = LitHufLog;
    let mut repeat = (*prevHuf).repeatMode;
    libc::memcpy(
        nextHuf as *mut core::ffi::c_void,
        prevHuf as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_hufCTables_t>(),
    );
    if literalsCompressionIsDisabled != 0 {
        (*hufMetadata).hType = set_basic;
        return 0;
    }
    let minLitSize = (if (*prevHuf).repeatMode as core::ffi::c_uint
        == HUF_repeat_valid as core::ffi::c_int as core::ffi::c_uint
    {
        6
    } else {
        COMPRESS_LITERALS_SIZE_MIN
    }) as size_t;
    if srcSize <= minLitSize {
        (*hufMetadata).hType = set_basic;
        return 0;
    }
    let largest = HIST_count_wksp(
        countWksp,
        &mut maxSymbolValue,
        src as *const u8 as *const core::ffi::c_void,
        srcSize,
        workspace,
        wkspSize,
    );
    let err_code = largest;
    if ERR_isError(err_code) {
        return err_code;
    }
    if largest == srcSize {
        (*hufMetadata).hType = set_rle;
        return 0;
    }
    if largest <= (srcSize >> 7).wrapping_add(4) {
        (*hufMetadata).hType = set_basic;
        return 0;
    }
    if repeat as core::ffi::c_uint == HUF_repeat_check as core::ffi::c_int as core::ffi::c_uint
        && HUF_validateCTable(((*prevHuf).CTable).as_ptr(), countWksp, maxSymbolValue) == 0
    {
        repeat = HUF_repeat_none;
    }
    ptr::write_bytes(
        ((*nextHuf).CTable).as_mut_ptr() as *mut u8,
        0,
        ::core::mem::size_of::<[HUF_CElt; 257]>(),
    );
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        nodeWksp as *mut core::ffi::c_void,
        nodeWkspSize,
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        hufFlags,
    );
    let maxBits = HUF_buildCTable_wksp(
        ((*nextHuf).CTable).as_mut_ptr(),
        countWksp,
        maxSymbolValue,
        huffLog,
        nodeWksp as *mut core::ffi::c_void,
        nodeWkspSize,
    );
    let err_code_0 = maxBits;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    huffLog = maxBits as u32;
    let newCSize =
        HUF_estimateCompressedSize(((*nextHuf).CTable).as_mut_ptr(), countWksp, maxSymbolValue);
    let hSize = HUF_writeCTable_wksp(
        ((*hufMetadata).hufDesBuffer).as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[u8; 128]>(),
        ((*nextHuf).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        nodeWksp as *mut core::ffi::c_void,
        nodeWkspSize,
    );
    if repeat as core::ffi::c_uint != HUF_repeat_none as core::ffi::c_int as core::ffi::c_uint {
        let oldCSize =
            HUF_estimateCompressedSize(((*prevHuf).CTable).as_ptr(), countWksp, maxSymbolValue);
        if oldCSize < srcSize
            && (oldCSize <= hSize.wrapping_add(newCSize) || hSize.wrapping_add(12) >= srcSize)
        {
            libc::memcpy(
                nextHuf as *mut core::ffi::c_void,
                prevHuf as *const core::ffi::c_void,
                ::core::mem::size_of::<ZSTD_hufCTables_t>(),
            );
            (*hufMetadata).hType = set_repeat;
            return 0;
        }
    }
    if newCSize.wrapping_add(hSize) >= srcSize {
        libc::memcpy(
            nextHuf as *mut core::ffi::c_void,
            prevHuf as *const core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>(),
        );
        (*hufMetadata).hType = set_basic;
        return 0;
    }
    (*hufMetadata).hType = set_compressed;
    (*nextHuf).repeatMode = HUF_repeat_check;
    hSize
}
pub const COMPRESS_LITERALS_SIZE_MIN: core::ffi::c_int = 63;
unsafe fn ZSTD_buildDummySequencesStatistics(
    nextEntropy: *mut ZSTD_fseCTables_t,
) -> ZSTD_symbolEncodingTypeStats_t {
    let stats = {
        ZSTD_symbolEncodingTypeStats_t {
            LLtype: set_basic as core::ffi::c_int as u32,
            Offtype: set_basic as core::ffi::c_int as u32,
            MLtype: set_basic as core::ffi::c_int as u32,
            size: 0,
            lastCountSize: 0,
            longOffsets: 0,
        }
    };
    (*nextEntropy).litlength_repeatMode = FSE_repeat_none;
    (*nextEntropy).offcode_repeatMode = FSE_repeat_none;
    (*nextEntropy).matchlength_repeatMode = FSE_repeat_none;
    stats
}
unsafe fn ZSTD_buildBlockEntropyStats_sequences(
    seqStorePtr: *const SeqStore_t,
    prevEntropy: *const ZSTD_fseCTables_t,
    nextEntropy: *mut ZSTD_fseCTables_t,
    cctxParams: *const ZSTD_CCtx_params,
    fseMetadata: *mut ZSTD_fseCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let strategy = (*cctxParams).cParams.strategy;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart) as size_t;
    let ostart = ((*fseMetadata).fseTablesBuffer).as_mut_ptr();
    let oend = ostart.add(::core::mem::size_of::<[u8; 133]>());
    let op = ostart;
    let countWorkspace = workspace as *mut core::ffi::c_uint;
    let entropyWorkspace = countWorkspace.offset(((if 35 > 52 { 35 } else { 52 }) + 1) as isize);
    let entropyWorkspaceSize = wkspSize.wrapping_sub(
        ((if 35 > 52 { 35 as size_t } else { 52 }) + 1)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>()),
    );
    let mut stats = ZSTD_symbolEncodingTypeStats_t {
        LLtype: 0,
        Offtype: 0,
        MLtype: 0,
        size: 0,
        lastCountSize: 0,
        longOffsets: 0,
    };
    stats = if nbSeq != 0 {
        ZSTD_buildSequencesStatistics(
            seqStorePtr,
            nbSeq,
            prevEntropy,
            nextEntropy,
            op,
            oend,
            strategy,
            countWorkspace,
            entropyWorkspace as *mut core::ffi::c_void,
            entropyWorkspaceSize,
        )
    } else {
        ZSTD_buildDummySequencesStatistics(nextEntropy)
    };
    let err_code = stats.size;
    if ERR_isError(err_code) {
        return err_code;
    }
    (*fseMetadata).llType = stats.LLtype as SymbolEncodingType_e;
    (*fseMetadata).ofType = stats.Offtype as SymbolEncodingType_e;
    (*fseMetadata).mlType = stats.MLtype as SymbolEncodingType_e;
    (*fseMetadata).lastCountSize = stats.lastCountSize;
    stats.size
}
pub unsafe fn ZSTD_buildBlockEntropyStats(
    seqStorePtr: *const SeqStore_t,
    prevEntropy: *const ZSTD_entropyCTables_t,
    nextEntropy: *mut ZSTD_entropyCTables_t,
    cctxParams: *const ZSTD_CCtx_params,
    entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let litSize = ((*seqStorePtr).lit).offset_from((*seqStorePtr).litStart) as size_t;
    let huf_useOptDepth = core::ffi::c_int::from(
        (*cctxParams).cParams.strategy as core::ffi::c_uint
            >= HUF_OPTIMAL_DEPTH_THRESHOLD as core::ffi::c_uint,
    );
    let hufFlags = if huf_useOptDepth != 0 {
        HUF_flags_optimalDepth as core::ffi::c_int
    } else {
        0
    };
    (*entropyMetadata).hufMetadata.hufDesSize = ZSTD_buildBlockEntropyStats_literals(
        (*seqStorePtr).litStart as *mut core::ffi::c_void,
        litSize,
        &(*prevEntropy).huf,
        &mut (*nextEntropy).huf,
        &mut (*entropyMetadata).hufMetadata,
        ZSTD_literalsCompressionIsDisabled(cctxParams),
        workspace,
        wkspSize,
        hufFlags,
    );
    let err_code = (*entropyMetadata).hufMetadata.hufDesSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    (*entropyMetadata).fseMetadata.fseTablesSize = ZSTD_buildBlockEntropyStats_sequences(
        seqStorePtr,
        &(*prevEntropy).fse,
        &mut (*nextEntropy).fse,
        cctxParams,
        &mut (*entropyMetadata).fseMetadata,
        workspace,
        wkspSize,
    );
    let err_code_0 = (*entropyMetadata).fseMetadata.fseTablesSize;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    0
}
unsafe fn ZSTD_estimateBlockSize_literal(
    literals: *const u8,
    litSize: size_t,
    huf: *const ZSTD_hufCTables_t,
    hufMetadata: *const ZSTD_hufCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeEntropy: core::ffi::c_int,
) -> size_t {
    let countWksp = workspace as *mut core::ffi::c_uint;
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX;
    let literalSectionHeaderSize =
        (3 + core::ffi::c_int::from(litSize >= ((1) << 10) as size_t)
            + core::ffi::c_int::from(litSize >= (16 * ((1) << 10)) as size_t)) as size_t;
    let singleStream = core::ffi::c_int::from(litSize < 256) as u32;
    if (*hufMetadata).hType as core::ffi::c_uint
        == set_basic as core::ffi::c_int as core::ffi::c_uint
    {
        return litSize;
    } else if (*hufMetadata).hType as core::ffi::c_uint
        == set_rle as core::ffi::c_int as core::ffi::c_uint
    {
        return 1;
    } else if (*hufMetadata).hType as core::ffi::c_uint
        == set_compressed as core::ffi::c_int as core::ffi::c_uint
        || (*hufMetadata).hType as core::ffi::c_uint
            == set_repeat as core::ffi::c_int as core::ffi::c_uint
    {
        let largest = HIST_count_wksp(
            countWksp,
            &mut maxSymbolValue,
            literals as *const core::ffi::c_void,
            litSize,
            workspace,
            wkspSize,
        );
        if ERR_isError(largest) {
            return litSize;
        }
        let mut cLitSizeEstimate =
            HUF_estimateCompressedSize(((*huf).CTable).as_ptr(), countWksp, maxSymbolValue);
        if writeEntropy != 0 {
            cLitSizeEstimate = cLitSizeEstimate.wrapping_add((*hufMetadata).hufDesSize);
        }
        if singleStream == 0 {
            cLitSizeEstimate = cLitSizeEstimate.wrapping_add(6);
        }
        return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
    }
    0
}
unsafe fn ZSTD_estimateBlockSize_symbolType(
    type_0: SymbolEncodingType_e,
    codeTable: *const u8,
    nbSeq: size_t,
    maxCode: core::ffi::c_uint,
    fseCTable: *const FSE_CTable,
    additionalBits: *const u8,
    defaultNorm: *const core::ffi::c_short,
    defaultNormLog: u32,
    defaultMax: u32,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let countWksp = workspace as *mut core::ffi::c_uint;
    let mut ctp = codeTable;
    let ctStart = ctp;
    let ctEnd = ctStart.add(nbSeq);
    let mut cSymbolTypeSizeEstimateInBits = 0;
    let mut max = maxCode;
    HIST_countFast_wksp(
        countWksp,
        &mut max,
        codeTable as *const core::ffi::c_void,
        nbSeq,
        workspace,
        wkspSize,
    );
    if type_0 as core::ffi::c_uint == set_basic as core::ffi::c_int as core::ffi::c_uint {
        /* We selected this encoding type, so it must be valid. */
        assert!(max <= defaultMax);

        cSymbolTypeSizeEstimateInBits =
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, countWksp, max);
    } else if type_0 as core::ffi::c_uint == set_rle as core::ffi::c_int as core::ffi::c_uint {
        cSymbolTypeSizeEstimateInBits = 0;
    } else if type_0 as core::ffi::c_uint == set_compressed as core::ffi::c_int as core::ffi::c_uint
        || type_0 as core::ffi::c_uint == set_repeat as core::ffi::c_int as core::ffi::c_uint
    {
        cSymbolTypeSizeEstimateInBits = ZSTD_fseBitCost(fseCTable, countWksp, max);
    }
    if ERR_isError(cSymbolTypeSizeEstimateInBits) {
        return nbSeq * 10;
    }
    while ctp < ctEnd {
        if !additionalBits.is_null() {
            cSymbolTypeSizeEstimateInBits = cSymbolTypeSizeEstimateInBits
                .wrapping_add(*additionalBits.offset(*ctp as isize) as size_t);
        } else {
            cSymbolTypeSizeEstimateInBits =
                cSymbolTypeSizeEstimateInBits.wrapping_add(*ctp as size_t);
        }
        ctp = ctp.add(1);
    }
    cSymbolTypeSizeEstimateInBits >> 3
}
unsafe fn ZSTD_estimateBlockSize_sequences(
    ofCodeTable: *const u8,
    llCodeTable: *const u8,
    mlCodeTable: *const u8,
    nbSeq: size_t,
    fseTables: *const ZSTD_fseCTables_t,
    fseMetadata: *const ZSTD_fseCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeEntropy: core::ffi::c_int,
) -> size_t {
    let sequencesSectionHeaderSize =
        (1 + 1
            + core::ffi::c_int::from(nbSeq >= 128)
            + core::ffi::c_int::from(nbSeq >= LONGNBSEQ as size_t)) as size_t;
    let mut cSeqSizeEstimate = 0 as size_t;
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateBlockSize_symbolType(
        (*fseMetadata).ofType,
        ofCodeTable,
        nbSeq,
        MaxOff,
        ((*fseTables).offcodeCTable).as_ptr(),
        core::ptr::null(),
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        DefaultMaxOff,
        workspace,
        wkspSize,
    ));
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateBlockSize_symbolType(
        (*fseMetadata).llType,
        llCodeTable,
        nbSeq,
        MaxLL,
        ((*fseTables).litlengthCTable).as_ptr(),
        LL_bits.as_ptr(),
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        MaxLL,
        workspace,
        wkspSize,
    ));
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateBlockSize_symbolType(
        (*fseMetadata).mlType,
        mlCodeTable,
        nbSeq,
        MaxML,
        ((*fseTables).matchlengthCTable).as_ptr(),
        ML_bits.as_ptr(),
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        MaxML,
        workspace,
        wkspSize,
    ));
    if writeEntropy != 0 {
        cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add((*fseMetadata).fseTablesSize);
    }
    cSeqSizeEstimate.wrapping_add(sequencesSectionHeaderSize)
}
unsafe fn ZSTD_estimateBlockSize(
    literals: *const u8,
    litSize: size_t,
    ofCodeTable: *const u8,
    llCodeTable: *const u8,
    mlCodeTable: *const u8,
    nbSeq: size_t,
    entropy: *const ZSTD_entropyCTables_t,
    entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeLitEntropy: core::ffi::c_int,
    writeSeqEntropy: core::ffi::c_int,
) -> size_t {
    let literalsSize = ZSTD_estimateBlockSize_literal(
        literals,
        litSize,
        &(*entropy).huf,
        &(*entropyMetadata).hufMetadata,
        workspace,
        wkspSize,
        writeLitEntropy,
    );
    let seqSize = ZSTD_estimateBlockSize_sequences(
        ofCodeTable,
        llCodeTable,
        mlCodeTable,
        nbSeq,
        &(*entropy).fse,
        &(*entropyMetadata).fseMetadata,
        workspace,
        wkspSize,
        writeSeqEntropy,
    );
    seqSize
        .wrapping_add(literalsSize)
        .wrapping_add(ZSTD_blockHeaderSize)
}
unsafe fn ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(
    seqStore: &mut SeqStore_t,
    zc: *mut ZSTD_CCtx,
) -> size_t {
    let entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t =
        &mut (*zc).blockSplitCtx.entropyMetadata;
    let err_code = ZSTD_buildBlockEntropyStats(
        seqStore,
        &(*(*zc).blockState.prevCBlock).entropy,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        &(*zc).appliedParams,
        entropyMetadata,
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_estimateBlockSize(
        seqStore.litStart,
        (seqStore.lit).offset_from_unsigned(seqStore.litStart),
        seqStore.ofCode,
        seqStore.llCode,
        seqStore.mlCode,
        (seqStore.sequences).offset_from(seqStore.sequencesStart) as core::ffi::c_long as size_t,
        &(*(*zc).blockState.nextCBlock).entropy,
        entropyMetadata,
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
        core::ffi::c_int::from(
            (*entropyMetadata).hufMetadata.hType as core::ffi::c_uint
                == set_compressed as core::ffi::c_int as core::ffi::c_uint,
        ),
        1,
    )
}
unsafe fn ZSTD_countSeqStoreLiteralsBytes(seqStore: *const SeqStore_t) -> size_t {
    let mut literalsBytes = 0 as size_t;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as size_t;
    let mut i: size_t = 0;
    i = 0;
    while i < nbSeqs {
        let seq = *((*seqStore).sequencesStart).add(i);
        literalsBytes = literalsBytes.wrapping_add(seq.litLength as size_t);
        if i == (*seqStore).longLengthPos as size_t
            && (*seqStore).longLengthType == ZSTD_llt_literalLength
        {
            literalsBytes = literalsBytes.wrapping_add(0x10000 as core::ffi::c_int as size_t);
        }
        i = i.wrapping_add(1);
    }
    literalsBytes
}
unsafe fn ZSTD_countSeqStoreMatchBytes(seqStore: *const SeqStore_t) -> size_t {
    let mut matchBytes = 0 as size_t;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as size_t;
    let mut i: size_t = 0;
    i = 0;
    while i < nbSeqs {
        let seq = *((*seqStore).sequencesStart).add(i);
        matchBytes =
            matchBytes.wrapping_add((core::ffi::c_int::from(seq.mlBase) + MINMATCH) as size_t);
        if i == (*seqStore).longLengthPos as size_t
            && (*seqStore).longLengthType == ZSTD_llt_matchLength
        {
            matchBytes = matchBytes.wrapping_add(0x10000 as core::ffi::c_int as size_t);
        }
        i = i.wrapping_add(1);
    }
    matchBytes
}
unsafe fn ZSTD_deriveSeqStoreChunk(
    resultSeqStore: &mut SeqStore_t,
    originalSeqStore: *const SeqStore_t,
    startIdx: size_t,
    endIdx: size_t,
) {
    *resultSeqStore = *originalSeqStore;
    if startIdx > 0 {
        resultSeqStore.sequences = ((*originalSeqStore).sequencesStart).add(startIdx);
        resultSeqStore.litStart =
            (resultSeqStore.litStart).add(ZSTD_countSeqStoreLiteralsBytes(resultSeqStore));
    }
    if (*originalSeqStore).longLengthType != ZSTD_llt_none {
        if ((*originalSeqStore).longLengthPos as size_t) < startIdx
            || (*originalSeqStore).longLengthPos as size_t > endIdx
        {
            resultSeqStore.longLengthType = ZSTD_llt_none;
        } else {
            resultSeqStore.longLengthPos =
                (resultSeqStore.longLengthPos).wrapping_sub(startIdx as u32);
        }
    }
    resultSeqStore.sequencesStart = ((*originalSeqStore).sequencesStart).add(startIdx);
    resultSeqStore.sequences = ((*originalSeqStore).sequencesStart).add(endIdx);
    if endIdx
        != ((*originalSeqStore).sequences).offset_from((*originalSeqStore).sequencesStart) as size_t
    {
        let literalsBytes = ZSTD_countSeqStoreLiteralsBytes(resultSeqStore);
        resultSeqStore.lit = (resultSeqStore.litStart).add(literalsBytes);
    }
    resultSeqStore.llCode = (resultSeqStore.llCode).add(startIdx);
    resultSeqStore.mlCode = (resultSeqStore.mlCode).add(startIdx);
    resultSeqStore.ofCode = (resultSeqStore.ofCode).add(startIdx);
}
unsafe fn ZSTD_resolveRepcodeToRawOffset(rep: *const u32, offBase: u32, ll0: u32) -> u32 {
    let adjustedRepCode = offBase.wrapping_sub(1).wrapping_add(ll0);
    if adjustedRepCode == ZSTD_REP_NUM as u32 {
        return (*rep).wrapping_sub(1);
    }
    *rep.offset(adjustedRepCode as isize)
}
unsafe fn ZSTD_seqStore_resolveOffCodes(
    dRepcodes: *mut Repcodes_t,
    cRepcodes: *mut Repcodes_t,
    seqStore: *const SeqStore_t,
    nbSeq: u32,
) {
    let mut idx = 0;
    let longLitLenIdx = if (*seqStore).longLengthType == ZSTD_llt_literalLength {
        (*seqStore).longLengthPos
    } else {
        nbSeq
    };
    while idx < nbSeq {
        let seq = ((*seqStore).sequencesStart).offset(idx as isize);
        let ll0 = core::ffi::c_int::from(
            core::ffi::c_int::from((*seq).litLength) == 0 && idx != longLitLenIdx,
        ) as u32;
        let offBase = (*seq).offBase;
        if 1 <= offBase && offBase <= ZSTD_REP_NUM as u32 {
            let dRawOffset = ZSTD_resolveRepcodeToRawOffset(
                ((*dRepcodes).rep).as_mut_ptr() as *const u32,
                offBase,
                ll0,
            );
            let cRawOffset = ZSTD_resolveRepcodeToRawOffset(
                ((*cRepcodes).rep).as_mut_ptr() as *const u32,
                offBase,
                ll0,
            );
            if dRawOffset != cRawOffset {
                (*seq).offBase = cRawOffset.wrapping_add(ZSTD_REP_NUM as u32);
            }
        }
        ZSTD_updateRep(((*dRepcodes).rep).as_mut_ptr(), (*seq).offBase, ll0);
        ZSTD_updateRep(((*cRepcodes).rep).as_mut_ptr(), offBase, ll0);
        idx = idx.wrapping_add(1);
    }
}
unsafe fn ZSTD_compressSeqStore_singleBlock(
    zc: *mut ZSTD_CCtx,
    seqStore: *const SeqStore_t,
    dRep: *mut Repcodes_t,
    cRep: *mut Repcodes_t,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastBlock: u32,
    isPartition: u32,
) -> size_t {
    let rleMaxLength = 25;
    let op = dst as *mut u8;
    let ip = src as *const u8;
    let mut cSize: size_t = 0;
    let mut cSeqsSize: size_t = 0;
    let dRepOriginal = *dRep;
    if isPartition != 0 {
        ZSTD_seqStore_resolveOffCodes(
            dRep,
            cRep,
            seqStore,
            ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as core::ffi::c_long
                as u32,
        );
    }
    if dstCapacity < ZSTD_blockHeaderSize {
        return Error::dstSize_tooSmall.to_error_code();
    }
    cSeqsSize = ZSTD_entropyCompressSeqStore(
        seqStore,
        &(*(*zc).blockState.prevCBlock).entropy,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        &(*zc).appliedParams,
        op.add(ZSTD_blockHeaderSize) as *mut core::ffi::c_void,
        dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
        srcSize,
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
        (*zc).bmi2,
    );
    let err_code = cSeqsSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    if (*zc).isFirstBlock == 0
        && cSeqsSize < rleMaxLength as size_t
        && ZSTD_isRLE(src as *const u8, srcSize) != 0
    {
        cSeqsSize = 1;
    }
    if (*zc).seqCollector.collectSequences != 0 {
        let err_code_0 = ZSTD_copyBlockSequences(
            &mut (*zc).seqCollector,
            seqStore,
            (dRepOriginal.rep).as_ptr(),
        );
        if ERR_isError(err_code_0) {
            return err_code_0;
        }
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
        return 0;
    }
    if cSeqsSize == 0 {
        cSize = ZSTD_noCompressBlock(
            op as *mut core::ffi::c_void,
            dstCapacity,
            ip as *const core::ffi::c_void,
            srcSize,
            lastBlock,
        );
        let err_code_1 = cSize;
        if ERR_isError(err_code_1) {
            return err_code_1;
        }
        *dRep = dRepOriginal;
    } else if cSeqsSize == 1 {
        cSize = ZSTD_rleCompressBlock(
            op as *mut core::ffi::c_void,
            dstCapacity,
            *ip,
            srcSize,
            lastBlock,
        );
        let err_code_2 = cSize;
        if ERR_isError(err_code_2) {
            return err_code_2;
        }
        *dRep = dRepOriginal;
    } else {
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
        writeBlockHeader(op as *mut core::ffi::c_void, cSeqsSize, srcSize, lastBlock);
        cSize = ZSTD_blockHeaderSize.wrapping_add(cSeqsSize);
    }
    if (*(*zc).blockState.prevCBlock)
        .entropy
        .fse
        .offcode_repeatMode as core::ffi::c_uint
        == FSE_repeat_valid as core::ffi::c_int as core::ffi::c_uint
    {
        (*(*zc).blockState.prevCBlock)
            .entropy
            .fse
            .offcode_repeatMode = FSE_repeat_check;
    }
    cSize
}
pub const MIN_SEQUENCES_BLOCK_SPLITTING: core::ffi::c_int = 300;
unsafe fn ZSTD_deriveBlockSplitsHelper(
    splits: *mut seqStoreSplits,
    startIdx: size_t,
    endIdx: size_t,
    zc: *mut ZSTD_CCtx,
    origSeqStore: *const SeqStore_t,
) {
    let fullSeqStoreChunk: &mut SeqStore_t = &mut (*zc).blockSplitCtx.fullSeqStoreChunk;
    let firstHalfSeqStore: &mut SeqStore_t = &mut (*zc).blockSplitCtx.firstHalfSeqStore;
    let secondHalfSeqStore: &mut SeqStore_t = &mut (*zc).blockSplitCtx.secondHalfSeqStore;
    let mut estimatedOriginalSize: size_t = 0;
    let mut estimatedFirstHalfSize: size_t = 0;
    let mut estimatedSecondHalfSize: size_t = 0;
    let midIdx = startIdx.wrapping_add(endIdx) / 2;
    if endIdx.wrapping_sub(startIdx) < MIN_SEQUENCES_BLOCK_SPLITTING as size_t
        || (*splits).idx >= ZSTD_MAX_NB_BLOCK_SPLITS as size_t
    {
        return;
    }
    ZSTD_deriveSeqStoreChunk(fullSeqStoreChunk, origSeqStore, startIdx, endIdx);
    ZSTD_deriveSeqStoreChunk(firstHalfSeqStore, origSeqStore, startIdx, midIdx);
    ZSTD_deriveSeqStoreChunk(secondHalfSeqStore, origSeqStore, midIdx, endIdx);
    estimatedOriginalSize =
        ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(fullSeqStoreChunk, zc);
    estimatedFirstHalfSize =
        ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(firstHalfSeqStore, zc);
    estimatedSecondHalfSize =
        ZSTD_buildEntropyStatisticsAndEstimateSubBlockSize(secondHalfSeqStore, zc);
    if ERR_isError(estimatedOriginalSize)
        || ERR_isError(estimatedFirstHalfSize)
        || ERR_isError(estimatedSecondHalfSize)
    {
        return;
    }
    if estimatedFirstHalfSize.wrapping_add(estimatedSecondHalfSize) < estimatedOriginalSize {
        ZSTD_deriveBlockSplitsHelper(splits, startIdx, midIdx, zc, origSeqStore);
        *((*splits).splitLocations).add((*splits).idx) = midIdx as u32;
        (*splits).idx = ((*splits).idx).wrapping_add(1);
        ZSTD_deriveBlockSplitsHelper(splits, midIdx, endIdx, zc, origSeqStore);
    }
}
unsafe fn ZSTD_deriveBlockSplits(zc: *mut ZSTD_CCtx, partitions: *mut u32, nbSeq: u32) -> size_t {
    let mut splits = seqStoreSplits {
        splitLocations: core::ptr::null_mut::<u32>(),
        idx: 0,
    };
    splits.splitLocations = partitions;
    splits.idx = 0;
    if nbSeq <= 4 {
        return 0;
    }
    ZSTD_deriveBlockSplitsHelper(&mut splits, 0, nbSeq as size_t, zc, &(*zc).seqStore);
    *(splits.splitLocations).add(splits.idx) = nbSeq;
    splits.idx
}
unsafe fn ZSTD_compressBlock_splitBlock_internal(
    zc: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    src: *const core::ffi::c_void,
    blockSize: size_t,
    lastBlock: u32,
    nbSeq: u32,
) -> size_t {
    let mut cSize = 0 as size_t;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let mut i = 0;
    let mut srcBytesTotal = 0 as size_t;
    let partitions = ((*zc).blockSplitCtx.partitions).as_mut_ptr();
    let nextSeqStore: &mut SeqStore_t = &mut (*zc).blockSplitCtx.nextSeqStore;
    let currSeqStore: &mut SeqStore_t = &mut (*zc).blockSplitCtx.currSeqStore;
    let numSplits = ZSTD_deriveBlockSplits(zc, partitions, nbSeq);
    let mut dRep = repcodes_s { rep: [0; 3] };
    let mut cRep = repcodes_s { rep: [0; 3] };
    libc::memcpy(
        (dRep.rep).as_mut_ptr() as *mut core::ffi::c_void,
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    libc::memcpy(
        (cRep.rep).as_mut_ptr() as *mut core::ffi::c_void,
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    ptr::write_bytes(
        nextSeqStore as *mut SeqStore_t as *mut u8,
        0,
        ::core::mem::size_of::<SeqStore_t>(),
    );
    if numSplits == 0 {
        let cSizeSingleBlock = ZSTD_compressSeqStore_singleBlock(
            zc,
            &(*zc).seqStore,
            &mut dRep,
            &mut cRep,
            op as *mut core::ffi::c_void,
            dstCapacity,
            ip as *const core::ffi::c_void,
            blockSize,
            lastBlock,
            0,
        );
        let err_code = cSizeSingleBlock;
        if ERR_isError(err_code) {
            return err_code;
        }
        return cSizeSingleBlock;
    }
    ZSTD_deriveSeqStoreChunk(currSeqStore, &(*zc).seqStore, 0, *partitions as size_t);
    i = 0;
    while i <= numSplits {
        let mut cSizeChunk: size_t = 0;
        let lastPartition = core::ffi::c_int::from(i == numSplits) as u32;
        let mut lastBlockEntireSrc = 0;
        let mut srcBytes = (ZSTD_countSeqStoreLiteralsBytes(currSeqStore))
            .wrapping_add(ZSTD_countSeqStoreMatchBytes(currSeqStore));
        srcBytesTotal = srcBytesTotal.wrapping_add(srcBytes);
        if lastPartition != 0 {
            srcBytes = srcBytes.wrapping_add(blockSize.wrapping_sub(srcBytesTotal));
            lastBlockEntireSrc = lastBlock;
        } else {
            ZSTD_deriveSeqStoreChunk(
                nextSeqStore,
                &(*zc).seqStore,
                *partitions.add(i) as size_t,
                *partitions.add(i.wrapping_add(1)) as size_t,
            );
        }
        cSizeChunk = ZSTD_compressSeqStore_singleBlock(
            zc,
            currSeqStore,
            &mut dRep,
            &mut cRep,
            op as *mut core::ffi::c_void,
            dstCapacity,
            ip as *const core::ffi::c_void,
            srcBytes,
            lastBlockEntireSrc,
            1,
        );
        let err_code_0 = cSizeChunk;
        if ERR_isError(err_code_0) {
            return err_code_0;
        }
        ip = ip.add(srcBytes);
        op = op.add(cSizeChunk);
        dstCapacity = dstCapacity.wrapping_sub(cSizeChunk);
        cSize = cSize.wrapping_add(cSizeChunk);
        *currSeqStore = *nextSeqStore;
        i = i.wrapping_add(1);
    }
    libc::memcpy(
        ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *mut core::ffi::c_void,
        (dRep.rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    cSize
}
unsafe fn ZSTD_compressBlock_splitBlock(
    zc: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastBlock: u32,
) -> size_t {
    let mut nbSeq: u32 = 0;
    let mut cSize: size_t = 0;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    let err_code = bss;
    if ERR_isError(err_code) {
        return err_code;
    }
    if bss == ZSTDbss_noCompress as core::ffi::c_int as size_t {
        if (*(*zc).blockState.prevCBlock)
            .entropy
            .fse
            .offcode_repeatMode as core::ffi::c_uint
            == FSE_repeat_valid as core::ffi::c_int as core::ffi::c_uint
        {
            (*(*zc).blockState.prevCBlock)
                .entropy
                .fse
                .offcode_repeatMode = FSE_repeat_check;
        }
        if (*zc).seqCollector.collectSequences != 0 {
            return Error::sequenceProducer_failed.to_error_code();
        }
        cSize = ZSTD_noCompressBlock(dst, dstCapacity, src, srcSize, lastBlock);
        let err_code_0 = cSize;
        if ERR_isError(err_code_0) {
            return err_code_0;
        }
        return cSize;
    }
    nbSeq = ((*zc).seqStore.sequences).offset_from((*zc).seqStore.sequencesStart)
        as core::ffi::c_long as u32;
    cSize = ZSTD_compressBlock_splitBlock_internal(
        zc,
        dst,
        dstCapacity,
        src,
        srcSize,
        lastBlock,
        nbSeq,
    );
    let err_code_1 = cSize;
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    cSize
}
unsafe fn ZSTD_compressBlock_internal(
    zc: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    frame: u32,
) -> size_t {
    let rleMaxLength = 25;
    let mut cSize: size_t = 0;
    let ip = src as *const u8;
    let op = dst as *mut u8;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    let err_code = bss;
    if ERR_isError(err_code) {
        return err_code;
    }
    if bss == ZSTDbss_noCompress as core::ffi::c_int as size_t {
        if (*zc).seqCollector.collectSequences != 0 {
            return Error::sequenceProducer_failed.to_error_code();
        }
        cSize = 0;
    } else {
        if (*zc).seqCollector.collectSequences != 0 {
            let err_code_0 = ZSTD_copyBlockSequences(
                &mut (*zc).seqCollector,
                ZSTD_getSeqStore(zc),
                ((*(*zc).blockState.prevCBlock).rep).as_mut_ptr() as *const u32,
            );
            if ERR_isError(err_code_0) {
                return err_code_0;
            }
            ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
            return 0;
        }
        cSize = ZSTD_entropyCompressSeqStore(
            &(*zc).seqStore,
            &(*(*zc).blockState.prevCBlock).entropy,
            &mut (*(*zc).blockState.nextCBlock).entropy,
            &(*zc).appliedParams,
            dst,
            dstCapacity,
            srcSize,
            (*zc).tmpWorkspace,
            (*zc).tmpWkspSize,
            (*zc).bmi2,
        );
        if frame != 0
            && (*zc).isFirstBlock == 0
            && cSize < rleMaxLength as size_t
            && ZSTD_isRLE(ip, srcSize) != 0
        {
            cSize = 1;
            *op = *ip;
        }
    }
    if !ERR_isError(cSize) && cSize > 1 {
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
    }
    if (*(*zc).blockState.prevCBlock)
        .entropy
        .fse
        .offcode_repeatMode as core::ffi::c_uint
        == FSE_repeat_valid as core::ffi::c_int as core::ffi::c_uint
    {
        (*(*zc).blockState.prevCBlock)
            .entropy
            .fse
            .offcode_repeatMode = FSE_repeat_check;
    }
    cSize
}
unsafe fn ZSTD_compressBlock_targetCBlockSize_body(
    zc: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    bss: size_t,
    lastBlock: u32,
) -> size_t {
    if bss == ZSTDbss_compress as core::ffi::c_int as size_t {
        if (*zc).isFirstBlock == 0
            && ZSTD_maybeRLE(&(*zc).seqStore) != 0
            && ZSTD_isRLE(src as *const u8, srcSize) != 0
        {
            return ZSTD_rleCompressBlock(
                dst,
                dstCapacity,
                *(src as *const u8),
                srcSize,
                lastBlock,
            );
        }
        let cSize = ZSTD_compressSuperBlock(zc, dst, dstCapacity, src, srcSize, lastBlock);
        if cSize != Error::dstSize_tooSmall.to_error_code() {
            let maxCSize =
                srcSize.wrapping_sub(ZSTD_minGain(srcSize, (*zc).appliedParams.cParams.strategy));
            let err_code = cSize;
            if ERR_isError(err_code) {
                return err_code;
            }
            if cSize != 0 && cSize < maxCSize.wrapping_add(ZSTD_blockHeaderSize) {
                ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
                return cSize;
            }
        }
    }
    ZSTD_noCompressBlock(dst, dstCapacity, src, srcSize, lastBlock)
}
unsafe fn ZSTD_compressBlock_targetCBlockSize(
    zc: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastBlock: u32,
) -> size_t {
    let mut cSize = 0;
    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    let err_code = bss;
    if ERR_isError(err_code) {
        return err_code;
    }
    cSize = ZSTD_compressBlock_targetCBlockSize_body(
        zc,
        dst,
        dstCapacity,
        src,
        srcSize,
        bss,
        lastBlock,
    );
    let err_code_0 = cSize;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    if (*(*zc).blockState.prevCBlock)
        .entropy
        .fse
        .offcode_repeatMode as core::ffi::c_uint
        == FSE_repeat_valid as core::ffi::c_int as core::ffi::c_uint
    {
        (*(*zc).blockState.prevCBlock)
            .entropy
            .fse
            .offcode_repeatMode = FSE_repeat_check;
    }
    cSize
}
unsafe fn ZSTD_overflowCorrectIfNeeded(
    ms: &mut ZSTD_MatchState_t,
    ws: *mut ZSTD_cwksp,
    params: *const ZSTD_CCtx_params,
    ip: *const core::ffi::c_void,
    iend: *const core::ffi::c_void,
) {
    let cycleLog = ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy);
    let maxDist = (1) << (*params).cParams.windowLog;
    if ZSTD_window_needOverflowCorrection(ms.window, cycleLog, maxDist, ms.loadedDictEnd, ip, iend)
    {
        let correction = ZSTD_window_correctOverflow(&mut ms.window, cycleLog, maxDist, ip);
        ZSTD_cwksp_mark_tables_dirty(ws);
        ZSTD_reduceIndex(ms, params, correction);
        ZSTD_cwksp_mark_tables_clean(ws);
        if ms.nextToUpdate < correction {
            ms.nextToUpdate = 0;
        } else {
            ms.nextToUpdate = (ms.nextToUpdate).wrapping_sub(correction);
        }
        ms.loadedDictEnd = 0;
        ms.dictMatchState = core::ptr::null();
    }
}
unsafe fn ZSTD_optimalBlockSize(
    cctx: *mut ZSTD_CCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    blockSizeMax: size_t,
    mut splitLevel: core::ffi::c_int,
    strat: ZSTD_strategy,
    savings: S64,
) -> size_t {
    static splitLevels: [core::ffi::c_int; 10] = [0, 0, 1, 2, 2, 3, 3, 4, 4, 4];
    if srcSize < (128 * ((1) << 10)) as size_t || blockSizeMax < (128 * ((1) << 10)) as size_t {
        return if srcSize < blockSizeMax {
            srcSize
        } else {
            blockSizeMax
        };
    }
    if savings < 3 {
        return (128 * ((1) << 10)) as size_t;
    }
    if splitLevel == 1 {
        return (128 * ((1) << 10)) as size_t;
    }
    if splitLevel == 0 {
        splitLevel = *splitLevels.as_ptr().offset(strat as isize);
    } else {
        splitLevel -= 2;
    }
    ZSTD_splitBlock(
        src,
        blockSizeMax,
        splitLevel,
        (*cctx).tmpWorkspace,
        (*cctx).tmpWkspSize,
    )
}
unsafe fn ZSTD_compress_frameChunk(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastFrameChunk: u32,
) -> size_t {
    let blockSizeMax = (*cctx).blockSizeMax;
    let mut remaining = srcSize;
    let mut ip = src as *const u8;
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let maxDist = (1) << (*cctx).appliedParams.cParams.windowLog;
    let mut savings = (*cctx).consumedSrcSize as S64 - (*cctx).producedCSize as S64;
    if (*cctx).appliedParams.fParams.checksumFlag != 0 && srcSize != 0 {
        ZSTD_XXH64_update_slice(
            &mut (*cctx).xxhState,
            core::slice::from_raw_parts(src as *const u8, srcSize),
        );
    }
    while remaining != 0 {
        let ms: &mut ZSTD_MatchState_t = &mut (*cctx).blockState.matchState;
        let blockSize = ZSTD_optimalBlockSize(
            cctx,
            ip as *const core::ffi::c_void,
            remaining,
            blockSizeMax,
            (*cctx).appliedParams.preBlockSplitter_level,
            (*cctx).appliedParams.cParams.strategy,
            savings,
        );
        let lastBlock = lastFrameChunk & core::ffi::c_int::from(blockSize == remaining) as u32;
        if dstCapacity
            < ZSTD_blockHeaderSize
                .wrapping_add((1 + 1) as size_t)
                .wrapping_add(1)
        {
            return Error::dstSize_tooSmall.to_error_code();
        }
        ZSTD_overflowCorrectIfNeeded(
            ms,
            &mut (*cctx).workspace,
            &(*cctx).appliedParams,
            ip as *const core::ffi::c_void,
            ip.add(blockSize) as *const core::ffi::c_void,
        );
        ZSTD_checkDictValidity(
            &ms.window,
            ip.add(blockSize) as *const core::ffi::c_void,
            maxDist,
            &mut ms.loadedDictEnd,
            &mut ms.dictMatchState,
        );
        ZSTD_window_enforceMaxDist(
            &mut ms.window,
            ip as *const core::ffi::c_void,
            maxDist,
            &mut ms.loadedDictEnd,
            &mut ms.dictMatchState,
        );
        if ms.nextToUpdate < ms.window.lowLimit {
            ms.nextToUpdate = ms.window.lowLimit;
        }
        let mut cSize: size_t = 0;
        if ZSTD_useTargetCBlockSize(&(*cctx).appliedParams) != 0 {
            cSize = ZSTD_compressBlock_targetCBlockSize(
                cctx,
                op as *mut core::ffi::c_void,
                dstCapacity,
                ip as *const core::ffi::c_void,
                blockSize,
                lastBlock,
            );
            let err_code = cSize;
            if ERR_isError(err_code) {
                return err_code;
            }
        } else if ZSTD_blockSplitterEnabled(&mut (*cctx).appliedParams) != 0 {
            cSize = ZSTD_compressBlock_splitBlock(
                cctx,
                op as *mut core::ffi::c_void,
                dstCapacity,
                ip as *const core::ffi::c_void,
                blockSize,
                lastBlock,
            );
            let err_code_0 = cSize;
            if ERR_isError(err_code_0) {
                return err_code_0;
            }
        } else {
            cSize = ZSTD_compressBlock_internal(
                cctx,
                op.add(ZSTD_blockHeaderSize) as *mut core::ffi::c_void,
                dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                ip as *const core::ffi::c_void,
                blockSize,
                1,
            );
            let err_code_1 = cSize;
            if ERR_isError(err_code_1) {
                return err_code_1;
            }
            if cSize == 0 {
                cSize = ZSTD_noCompressBlock(
                    op as *mut core::ffi::c_void,
                    dstCapacity,
                    ip as *const core::ffi::c_void,
                    blockSize,
                    lastBlock,
                );
                let err_code_2 = cSize;
                if ERR_isError(err_code_2) {
                    return err_code_2;
                }
            } else {
                let cBlockHeader = if cSize == 1 {
                    lastBlock
                        .wrapping_add((bt_rle as core::ffi::c_int as u32) << 1)
                        .wrapping_add((blockSize << 3) as u32)
                } else {
                    lastBlock
                        .wrapping_add((bt_compressed as core::ffi::c_int as u32) << 1)
                        .wrapping_add((cSize << 3) as u32)
                };
                MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader);
                cSize = cSize.wrapping_add(ZSTD_blockHeaderSize);
            }
        }
        savings += blockSize as S64 - cSize as S64;
        ip = ip.add(blockSize);
        remaining = remaining.wrapping_sub(blockSize);
        op = op.add(cSize);
        dstCapacity = dstCapacity.wrapping_sub(cSize);
        (*cctx).isFirstBlock = 0;
    }
    if lastFrameChunk != 0 && op > ostart {
        (*cctx).stage = ZSTDcs_ending;
    }
    op.offset_from_unsigned(ostart)
}
unsafe fn ZSTD_writeFrameHeader(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    params: *const ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    dictID: u32,
) -> size_t {
    let op = dst as *mut u8;
    let dictIDSizeCodeLength = (core::ffi::c_int::from(dictID > 0)
        + core::ffi::c_int::from(dictID >= 256)
        + core::ffi::c_int::from(dictID >= 65536)) as u32;
    let dictIDSizeCode = if (*params).fParams.noDictIDFlag != 0 {
        0
    } else {
        dictIDSizeCodeLength
    };
    let checksumFlag = core::ffi::c_int::from((*params).fParams.checksumFlag > 0) as u32;
    let windowSize = (1) << (*params).cParams.windowLog;
    let singleSegment = core::ffi::c_int::from(
        (*params).fParams.contentSizeFlag != 0 && windowSize as u64 >= pledgedSrcSize,
    ) as u32;
    let windowLogByte = (((*params).cParams.windowLog)
        .wrapping_sub(ZSTD_WINDOWLOG_ABSOLUTEMIN as core::ffi::c_uint)
        << 3) as u8;
    let fcsCode = (if (*params).fParams.contentSizeFlag != 0 {
        core::ffi::c_int::from(pledgedSrcSize >= 256)
            + core::ffi::c_int::from(pledgedSrcSize >= (65536 + 256) as u64)
            + core::ffi::c_int::from(pledgedSrcSize >= u64::from(0xffffffff as core::ffi::c_uint))
    } else {
        0
    }) as u32;
    let frameHeaderDescriptionByte = dictIDSizeCode
        .wrapping_add(checksumFlag << 2)
        .wrapping_add(singleSegment << 5)
        .wrapping_add(fcsCode << 6) as u8;
    let mut pos = 0 as size_t;
    if dstCapacity < 18 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if (*params).format == Format::ZSTD_f_zstd1 {
        MEM_writeLE32(dst, ZSTD_MAGICNUMBER);
        pos = 4;
    }
    let fresh7 = pos;
    pos = pos.wrapping_add(1);
    *op.add(fresh7) = frameHeaderDescriptionByte;
    if singleSegment == 0 {
        let fresh8 = pos;
        pos = pos.wrapping_add(1);
        *op.add(fresh8) = windowLogByte;
    }
    match dictIDSizeCode {
        1 => {
            *op.add(pos) = dictID as u8;
            pos = pos.wrapping_add(1);
        }
        2 => {
            MEM_writeLE16(op.add(pos) as *mut core::ffi::c_void, dictID as u16);
            pos = pos.wrapping_add(2);
        }
        3 => {
            MEM_writeLE32(op.add(pos) as *mut core::ffi::c_void, dictID);
            pos = pos.wrapping_add(4);
        }
        0 | _ => {}
    }
    match fcsCode {
        1 => {
            MEM_writeLE16(
                op.add(pos) as *mut core::ffi::c_void,
                pledgedSrcSize.wrapping_sub(256) as u16,
            );
            pos = pos.wrapping_add(2);
        }
        2 => {
            MEM_writeLE32(op.add(pos) as *mut core::ffi::c_void, pledgedSrcSize as u32);
            pos = pos.wrapping_add(4);
        }
        3 => {
            MEM_writeLE64(op.add(pos) as *mut core::ffi::c_void, pledgedSrcSize);
            pos = pos.wrapping_add(8);
        }
        0 | _ => {
            if singleSegment != 0 {
                let fresh9 = pos;
                pos = pos.wrapping_add(1);
                *op.add(fresh9) = pledgedSrcSize as u8;
            }
        }
    }
    pos
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_writeSkippableFrame))]
pub unsafe extern "C" fn ZSTD_writeSkippableFrame(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    magicVariant: core::ffi::c_uint,
) -> size_t {
    let op = dst as *mut u8;
    if dstCapacity < srcSize.wrapping_add(8) {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if srcSize > 0xffffffff as core::ffi::c_uint as size_t {
        return Error::srcSize_wrong.to_error_code();
    }
    if magicVariant > 15 {
        return Error::parameter_outOfBound.to_error_code();
    }
    MEM_writeLE32(
        op as *mut core::ffi::c_void,
        ZSTD_MAGIC_SKIPPABLE_START.wrapping_add(magicVariant),
    );
    MEM_writeLE32(op.add(4) as *mut core::ffi::c_void, srcSize as u32);
    libc::memcpy(
        op.add(8) as *mut core::ffi::c_void,
        src,
        srcSize as libc::size_t,
    );
    srcSize.wrapping_add(ZSTD_SKIPPABLEHEADERSIZE as size_t)
}
pub unsafe fn ZSTD_writeLastEmptyBlock(dst: *mut core::ffi::c_void, dstCapacity: size_t) -> size_t {
    if dstCapacity < ZSTD_blockHeaderSize {
        return Error::dstSize_tooSmall.to_error_code();
    }
    let cBlockHeader24 = (1u32).wrapping_add((bt_raw as core::ffi::c_int as u32) << 1);
    MEM_writeLE24(dst, cBlockHeader24);
    ZSTD_blockHeaderSize
}
pub unsafe fn ZSTD_referenceExternalSequences(
    cctx: *mut ZSTD_CCtx,
    seq: *mut rawSeq,
    nbSeq: size_t,
) {
    (*cctx).externSeqStore.seq = seq;
    (*cctx).externSeqStore.size = nbSeq;
    (*cctx).externSeqStore.capacity = nbSeq;
    (*cctx).externSeqStore.pos = 0;
    (*cctx).externSeqStore.posInSequence = 0;
}
unsafe extern "C" fn ZSTD_compressContinue_internal(
    cctx: *mut ZSTD_CCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    frame: u32,
    lastFrameChunk: u32,
) -> size_t {
    let ms: &mut ZSTD_MatchState_t = &mut (*cctx).blockState.matchState;
    let mut fhSize = 0;
    if (*cctx).stage as core::ffi::c_uint == ZSTDcs_created as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    if frame != 0
        && (*cctx).stage as core::ffi::c_uint
            == ZSTDcs_init as core::ffi::c_int as core::ffi::c_uint
    {
        fhSize = ZSTD_writeFrameHeader(
            dst,
            dstCapacity,
            &(*cctx).appliedParams,
            ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1),
            (*cctx).dictID,
        );
        let err_code = fhSize;
        if ERR_isError(err_code) {
            return err_code;
        }
        dstCapacity = dstCapacity.wrapping_sub(fhSize);
        dst = (dst as *mut core::ffi::c_char).add(fhSize) as *mut core::ffi::c_void;
        (*cctx).stage = ZSTDcs_ongoing;
    }
    if srcSize == 0 {
        return fhSize;
    }
    if ZSTD_window_update(&mut ms.window, src, srcSize, ms.forceNonContiguous) == 0 {
        ms.forceNonContiguous = 0;
        ms.nextToUpdate = ms.window.dictLimit;
    }
    if (*cctx).appliedParams.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        ZSTD_window_update(&mut (*cctx).ldmState.window, src, srcSize, 0);
    }
    if frame == 0 {
        ZSTD_overflowCorrectIfNeeded(
            ms,
            &mut (*cctx).workspace,
            &(*cctx).appliedParams,
            src,
            (src as *const u8).add(srcSize) as *const core::ffi::c_void,
        );
    }
    let cSize = if frame != 0 {
        ZSTD_compress_frameChunk(cctx, dst, dstCapacity, src, srcSize, lastFrameChunk)
    } else {
        ZSTD_compressBlock_internal(cctx, dst, dstCapacity, src, srcSize, 0)
    };
    let err_code_0 = cSize;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    (*cctx).consumedSrcSize =
        ((*cctx).consumedSrcSize).wrapping_add(srcSize as core::ffi::c_ulonglong);
    (*cctx).producedCSize =
        ((*cctx).producedCSize).wrapping_add(cSize.wrapping_add(fhSize) as core::ffi::c_ulonglong);
    if (*cctx).pledgedSrcSizePlusOne != 0
        && ((*cctx).consumedSrcSize).wrapping_add(1) > (*cctx).pledgedSrcSizePlusOne
    {
        return Error::srcSize_wrong.to_error_code();
    }
    cSize.wrapping_add(fhSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressContinue_public))]
pub unsafe extern "C" fn ZSTD_compressContinue_public(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressContinue_internal(cctx, dst, dstCapacity, src, srcSize, 1, 0)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressContinue))]
pub unsafe extern "C" fn ZSTD_compressContinue(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressContinue_public(cctx, dst, dstCapacity, src, srcSize)
}
unsafe fn ZSTD_getBlockSize_deprecated(cctx: *const ZSTD_CCtx) -> size_t {
    let cParams = (*cctx).appliedParams.cParams;
    if (*cctx).appliedParams.maxBlockSize < (1) << cParams.windowLog {
        (*cctx).appliedParams.maxBlockSize
    } else {
        (1) << cParams.windowLog
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getBlockSize))]
pub unsafe extern "C" fn ZSTD_getBlockSize(cctx: *const ZSTD_CCtx) -> size_t {
    ZSTD_getBlockSize_deprecated(cctx)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBlock_deprecated))]
pub unsafe extern "C" fn ZSTD_compressBlock_deprecated(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let blockSizeMax = ZSTD_getBlockSize_deprecated(cctx);
    if srcSize > blockSizeMax {
        return Error::srcSize_wrong.to_error_code();
    }
    ZSTD_compressContinue_internal(cctx, dst, dstCapacity, src, srcSize, 0, 0)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBlock))]
pub unsafe extern "C" fn ZSTD_compressBlock(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_deprecated(cctx, dst, dstCapacity, src, srcSize)
}
unsafe fn ZSTD_loadDictionaryContent(
    ms: &mut ZSTD_MatchState_t,
    ls: *mut ldmState_t,
    ws: *mut ZSTD_cwksp,
    params: *const ZSTD_CCtx_params,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    dtlm: ZSTD_dictTableLoadMethod_e,
    tfp: ZSTD_tableFillPurpose_e,
) -> size_t {
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let loadLdmDict = core::ffi::c_int::from(
        (*params).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable && !ls.is_null(),
    );
    ZSTD_assertEqualCParams((*params).cParams, ms.cParams);
    let mut maxDictSize = (if MEM_64bits() {
        (3500 as core::ffi::c_uint)
            .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
    } else {
        (2000 as core::ffi::c_uint)
            .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
    })
    .wrapping_sub(ZSTD_WINDOW_START_INDEX as core::ffi::c_uint);
    let CDictTaggedIndices = ZSTD_CDictIndicesAreTagged(&(*params).cParams);
    if CDictTaggedIndices != 0
        && tfp as core::ffi::c_uint == ZSTD_tfp_forCDict as core::ffi::c_int as core::ffi::c_uint
    {
        let shortCacheMaxDictSize = ((1 as core::ffi::c_uint) << (32 - ZSTD_SHORT_CACHE_TAG_BITS))
            .wrapping_sub(ZSTD_WINDOW_START_INDEX as core::ffi::c_uint);
        maxDictSize = if maxDictSize < shortCacheMaxDictSize {
            maxDictSize
        } else {
            shortCacheMaxDictSize
        };
    }
    if srcSize > maxDictSize as size_t {
        ip = iend.offset(-(maxDictSize as isize));
        src = ip as *const core::ffi::c_void;
        srcSize = maxDictSize as size_t;
    }
    if srcSize
        > (-(1 as core::ffi::c_int) as u32).wrapping_sub(if MEM_64bits() {
            (3500 as core::ffi::c_uint)
                .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
        } else {
            (2000 as core::ffi::c_uint)
                .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
        }) as size_t
    {
        assert!(loadLdmDict != 0);
    }
    ZSTD_window_update(&mut ms.window, src, srcSize, 0);
    if loadLdmDict != 0 {
        ZSTD_window_update(&mut (*ls).window, src, srcSize, 0);
        (*ls).loadedDictEnd = if (*params).forceWindow != 0 {
            0
        } else {
            iend.offset_from((*ls).window.base) as core::ffi::c_long as u32
        };
        ZSTD_ldm_fillHashTable(ls, ip, iend, &(*params).ldmParams);
    }
    let maxDictSize_0 = (1)
        << (if (if ((*params).cParams.hashLog).wrapping_add(3)
            > ((*params).cParams.chainLog).wrapping_add(1)
        {
            ((*params).cParams.hashLog).wrapping_add(3)
        } else {
            ((*params).cParams.chainLog).wrapping_add(1)
        }) < 31
        {
            if ((*params).cParams.hashLog).wrapping_add(3)
                > ((*params).cParams.chainLog).wrapping_add(1)
            {
                ((*params).cParams.hashLog).wrapping_add(3)
            } else {
                ((*params).cParams.chainLog).wrapping_add(1)
            }
        } else {
            31
        });
    if srcSize > maxDictSize_0 as size_t {
        ip = iend.offset(-(maxDictSize_0 as isize));
        src = ip as *const core::ffi::c_void;
        srcSize = maxDictSize_0 as size_t;
    }
    ms.nextToUpdate = ip.offset_from(ms.window.base) as core::ffi::c_long as u32;
    ms.loadedDictEnd = if (*params).forceWindow != 0 {
        0
    } else {
        iend.offset_from(ms.window.base) as core::ffi::c_long as u32
    };
    ms.forceNonContiguous = (*params).deterministicRefPrefix;
    if srcSize <= HASH_READ_SIZE as size_t {
        return 0;
    }
    ZSTD_overflowCorrectIfNeeded(
        ms,
        ws,
        params,
        ip as *const core::ffi::c_void,
        iend as *const core::ffi::c_void,
    );
    match (*params).cParams.strategy as core::ffi::c_uint {
        1 => {
            ZSTD_fillHashTable(ms, iend as *const core::ffi::c_void, dtlm, tfp);
        }
        2 => {
            ZSTD_fillDoubleHashTable(ms, iend as *const core::ffi::c_void, dtlm, tfp);
        }
        3..=5 => {
            if ms.dedicatedDictSearch != 0 {
                ZSTD_dedicatedDictSearch_lazy_loadDictionary(
                    ms,
                    iend.offset(-(HASH_READ_SIZE as isize)),
                );
            } else if (*params).useRowMatchFinder == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
                let tagTableSize = (1) << (*params).cParams.hashLog;
                ptr::write_bytes(ms.tagTable, 0, tagTableSize as usize);
                ZSTD_row_update(ms, iend.offset(-(HASH_READ_SIZE as isize)));
            } else {
                ZSTD_insertAndFindFirstIndex(ms, iend.offset(-(HASH_READ_SIZE as isize)));
            }
        }
        6..=9 => {
            ZSTD_updateTree(ms, iend.offset(-(HASH_READ_SIZE as isize)), iend);
        }
        _ => {}
    }
    ms.nextToUpdate = iend.offset_from(ms.window.base) as core::ffi::c_long as u32;
    0
}
unsafe fn ZSTD_dictNCountRepeat(
    normalizedCounter: *mut core::ffi::c_short,
    dictMaxSymbolValue: core::ffi::c_uint,
    maxSymbolValue: core::ffi::c_uint,
) -> FSE_repeat {
    let mut s: u32 = 0;
    if dictMaxSymbolValue < maxSymbolValue {
        return FSE_repeat_check;
    }
    s = 0;
    while s <= maxSymbolValue {
        if core::ffi::c_int::from(*normalizedCounter.offset(s as isize)) == 0 {
            return FSE_repeat_check;
        }
        s = s.wrapping_add(1);
    }
    FSE_repeat_valid
}
pub unsafe fn ZSTD_loadCEntropy(
    bs: *mut ZSTD_compressedBlockState_t,
    workspace: *mut core::ffi::c_void,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let mut offcodeNCount: [core::ffi::c_short; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff;
    let mut dictPtr = dict as *const u8;
    let dictEnd = dictPtr.add(dictSize);
    dictPtr = dictPtr.add(8);
    (*bs).entropy.huf.repeatMode = HUF_repeat_check;
    let mut maxSymbolValue = 255;
    let mut hasZeroWeights = 1;
    let hufHeaderSize = HUF_readCTable(
        ((*bs).entropy.huf.CTable).as_mut_ptr(),
        &mut maxSymbolValue,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from_unsigned(dictPtr),
        &mut hasZeroWeights,
    );
    if hasZeroWeights == 0 && maxSymbolValue == 255 {
        (*bs).entropy.huf.repeatMode = HUF_repeat_valid;
    }
    if ERR_isError(hufHeaderSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    dictPtr = dictPtr.add(hufHeaderSize);
    let mut offcodeLog: core::ffi::c_uint = 0;
    let offcodeHeaderSize = FSE_readNCount(
        &mut offcodeNCount,
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from_unsigned(dictPtr),
    );
    if ERR_isError(offcodeHeaderSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    if offcodeLog > 8 {
        return Error::dictionary_corrupted.to_error_code();
    }
    if ERR_isError(FSE_buildCTable_wksp(
        ((*bs).entropy.fse.offcodeCTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        31,
        offcodeLog,
        workspace,
        (((8) << 10) + 512) as size_t,
    )) {
        return Error::dictionary_corrupted.to_error_code();
    }
    dictPtr = dictPtr.add(offcodeHeaderSize);
    let mut matchlengthNCount: [core::ffi::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML;
    let mut matchlengthLog: core::ffi::c_uint = 0;
    let matchlengthHeaderSize = FSE_readNCount(
        &mut matchlengthNCount,
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from_unsigned(dictPtr),
    );
    if ERR_isError(matchlengthHeaderSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    if matchlengthLog > 9 {
        return Error::dictionary_corrupted.to_error_code();
    }
    if ERR_isError(FSE_buildCTable_wksp(
        ((*bs).entropy.fse.matchlengthCTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
        workspace,
        (((8) << 10) + 512) as size_t,
    )) {
        return Error::dictionary_corrupted.to_error_code();
    }
    (*bs).entropy.fse.matchlength_repeatMode =
        ZSTD_dictNCountRepeat(matchlengthNCount.as_mut_ptr(), matchlengthMaxValue, MaxML);
    dictPtr = dictPtr.add(matchlengthHeaderSize);
    let mut litlengthNCount: [core::ffi::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL;
    let mut litlengthLog: core::ffi::c_uint = 0;
    let litlengthHeaderSize = FSE_readNCount(
        &mut litlengthNCount,
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from_unsigned(dictPtr),
    );
    if ERR_isError(litlengthHeaderSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    if litlengthLog > 9 {
        return Error::dictionary_corrupted.to_error_code();
    }
    if ERR_isError(FSE_buildCTable_wksp(
        ((*bs).entropy.fse.litlengthCTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
        workspace,
        (((8) << 10) + 512) as size_t,
    )) {
        return Error::dictionary_corrupted.to_error_code();
    }
    (*bs).entropy.fse.litlength_repeatMode =
        ZSTD_dictNCountRepeat(litlengthNCount.as_mut_ptr(), litlengthMaxValue, MaxLL);
    dictPtr = dictPtr.add(litlengthHeaderSize);
    if dictPtr.add(12) > dictEnd {
        return Error::dictionary_corrupted.to_error_code();
    }
    *((*bs).rep).as_mut_ptr() = MEM_readLE32(dictPtr as *const core::ffi::c_void);
    *((*bs).rep).as_mut_ptr().add(1) = MEM_readLE32(dictPtr.add(4) as *const core::ffi::c_void);
    *((*bs).rep).as_mut_ptr().add(2) = MEM_readLE32(dictPtr.add(8) as *const core::ffi::c_void);
    dictPtr = dictPtr.add(12);
    let dictContentSize = dictEnd.offset_from_unsigned(dictPtr);
    let mut offcodeMax = MaxOff;
    if dictContentSize
        <= (-(1 as core::ffi::c_int) as u32)
            .wrapping_sub((128 as core::ffi::c_int * ((1 as core::ffi::c_int) << 10)) as u32)
            as size_t
    {
        let maxOffset = (dictContentSize as u32).wrapping_add((128 * ((1) << 10)) as u32);
        offcodeMax = ZSTD_highbit32(maxOffset);
    }
    (*bs).entropy.fse.offcode_repeatMode = ZSTD_dictNCountRepeat(
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        if offcodeMax < 31 { offcodeMax } else { 31 },
    );
    let mut u: u32 = 0;
    u = 0;
    while u < 3 {
        if *((*bs).rep).as_mut_ptr().offset(u as isize) == 0 {
            return Error::dictionary_corrupted.to_error_code();
        }
        if *((*bs).rep).as_mut_ptr().offset(u as isize) as size_t > dictContentSize {
            return Error::dictionary_corrupted.to_error_code();
        }
        u = u.wrapping_add(1);
    }
    dictPtr.offset_from_unsigned(dict as *const u8)
}
unsafe fn ZSTD_loadZstdDictionary(
    bs: *mut ZSTD_compressedBlockState_t,
    ms: &mut ZSTD_MatchState_t,
    ws: *mut ZSTD_cwksp,
    params: *const ZSTD_CCtx_params,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dtlm: ZSTD_dictTableLoadMethod_e,
    tfp: ZSTD_tableFillPurpose_e,
    workspace: *mut core::ffi::c_void,
) -> size_t {
    let mut dictPtr = dict as *const u8;
    let dictEnd = dictPtr.add(dictSize);
    let mut dictID: size_t = 0;
    let mut eSize: size_t = 0;
    dictID = (if (*params).fParams.noDictIDFlag != 0 {
        0
    } else {
        MEM_readLE32(dictPtr.add(4) as *const core::ffi::c_void)
    }) as size_t;
    eSize = ZSTD_loadCEntropy(bs, workspace, dict, dictSize);
    let err_code = eSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    dictPtr = dictPtr.add(eSize);
    let dictContentSize = dictEnd.offset_from_unsigned(dictPtr);
    let err_code_0 = ZSTD_loadDictionaryContent(
        ms,
        core::ptr::null_mut::<ldmState_t>(),
        ws,
        params,
        dictPtr as *const core::ffi::c_void,
        dictContentSize,
        dtlm,
        tfp,
    );
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    dictID
}
unsafe fn ZSTD_compress_insertDictionary(
    bs: *mut ZSTD_compressedBlockState_t,
    ms: &mut ZSTD_MatchState_t,
    ls: *mut ldmState_t,
    ws: *mut ZSTD_cwksp,
    params: *const ZSTD_CCtx_params,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
    dtlm: ZSTD_dictTableLoadMethod_e,
    tfp: ZSTD_tableFillPurpose_e,
    workspace: *mut core::ffi::c_void,
) -> size_t {
    if dict.is_null() || dictSize < 8 {
        if dictContentType as core::ffi::c_uint
            == ZSTD_dct_fullDict as core::ffi::c_int as core::ffi::c_uint
        {
            return Error::dictionary_wrong.to_error_code();
        }
        return 0;
    }
    ZSTD_reset_compressedBlockState(bs);
    if dictContentType as core::ffi::c_uint
        == ZSTD_dct_rawContent as core::ffi::c_int as core::ffi::c_uint
    {
        return ZSTD_loadDictionaryContent(ms, ls, ws, params, dict, dictSize, dtlm, tfp);
    }
    if MEM_readLE32(dict) != ZSTD_MAGIC_DICTIONARY {
        if dictContentType as core::ffi::c_uint
            == ZSTD_dct_auto as core::ffi::c_int as core::ffi::c_uint
        {
            return ZSTD_loadDictionaryContent(ms, ls, ws, params, dict, dictSize, dtlm, tfp);
        }
        if dictContentType as core::ffi::c_uint
            == ZSTD_dct_fullDict as core::ffi::c_int as core::ffi::c_uint
        {
            return Error::dictionary_wrong.to_error_code();
        }
    }
    ZSTD_loadZstdDictionary(bs, ms, ws, params, dict, dictSize, dtlm, tfp, workspace)
}
pub const ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF: core::ffi::c_int = 128 * ((1) << 10);
pub const ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER: core::ffi::c_ulonglong = 6;
unsafe fn ZSTD_compressBegin_internal(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
    dtlm: ZSTD_dictTableLoadMethod_e,
    cdict: *const ZSTD_CDict,
    params: *const ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: ZSTD_buffered_policy_e,
) -> size_t {
    let dictContentSize = if !cdict.is_null() {
        (*cdict).dictContentSize
    } else {
        dictSize
    };
    (*cctx).traceCtx = ZSTD_trace_compress_begin(cctx);
    if !cdict.is_null()
        && (*cdict).dictContentSize > 0
        && (pledgedSrcSize < ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF as u64
            || (pledgedSrcSize as core::ffi::c_ulonglong)
                < ((*cdict).dictContentSize as core::ffi::c_ulonglong)
                    .wrapping_mul(ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER)
            || pledgedSrcSize as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
            || (*cdict).compressionLevel == 0)
        && (*params).attachDictPref != ZSTD_dictAttachPref_e::ZSTD_dictForceLoad
    {
        return ZSTD_resetCCtx_usingCDict(cctx, cdict, params, pledgedSrcSize, zbuff);
    }
    let err_code = ZSTD_resetCCtx_internal(
        cctx,
        params,
        pledgedSrcSize,
        dictContentSize,
        ZSTDcrp_makeClean,
        zbuff,
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    let dictID = if !cdict.is_null() {
        ZSTD_compress_insertDictionary(
            (*cctx).blockState.prevCBlock,
            &mut (*cctx).blockState.matchState,
            &mut (*cctx).ldmState,
            &mut (*cctx).workspace,
            &(*cctx).appliedParams,
            (*cdict).dictContent,
            (*cdict).dictContentSize,
            (*cdict).dictContentType,
            dtlm,
            ZSTD_tfp_forCCtx,
            (*cctx).tmpWorkspace,
        )
    } else {
        ZSTD_compress_insertDictionary(
            (*cctx).blockState.prevCBlock,
            &mut (*cctx).blockState.matchState,
            &mut (*cctx).ldmState,
            &mut (*cctx).workspace,
            &(*cctx).appliedParams,
            dict,
            dictSize,
            dictContentType,
            dtlm,
            ZSTD_tfp_forCCtx,
            (*cctx).tmpWorkspace,
        )
    };
    let err_code_0 = dictID;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    (*cctx).dictID = dictID as u32;
    (*cctx).dictContentSize = dictContentSize;
    0
}
pub unsafe fn ZSTD_compressBegin_advanced_internal(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
    dtlm: ZSTD_dictTableLoadMethod_e,
    cdict: *const ZSTD_CDict,
    params: *const ZSTD_CCtx_params,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    let err_code = ZSTD_checkCParams((*params).cParams);
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        dictContentType,
        dtlm,
        cdict,
        params,
        pledgedSrcSize,
        ZSTDb_not_buffered,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBegin_advanced))]
pub unsafe extern "C" fn ZSTD_compressBegin_advanced(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    params: ZSTD_parameters,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: Format::ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
        literalCompressionMode: ParamSwitch::Auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    };
    ZSTD_CCtxParams_init_internal(&mut cctxParams, &params, ZSTD_NO_CLEVEL);
    ZSTD_compressBegin_advanced_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        core::ptr::null(),
        &cctxParams,
        pledgedSrcSize,
    )
}
unsafe fn ZSTD_compressBegin_usingDict_deprecated(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: Format::ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
        literalCompressionMode: ParamSwitch::Auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    };
    let params = ZSTD_getParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_noAttachDict,
    );
    ZSTD_CCtxParams_init_internal(
        &mut cctxParams,
        &params,
        if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        },
    );
    ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        core::ptr::null(),
        &cctxParams,
        ZSTD_CONTENTSIZE_UNKNOWN,
        ZSTDb_not_buffered,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBegin_usingDict))]
pub unsafe extern "C" fn ZSTD_compressBegin_usingDict(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    ZSTD_compressBegin_usingDict_deprecated(cctx, dict, dictSize, compressionLevel)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBegin))]
pub unsafe extern "C" fn ZSTD_compressBegin(
    cctx: *mut ZSTD_CCtx,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    ZSTD_compressBegin_usingDict_deprecated(cctx, core::ptr::null(), 0, compressionLevel)
}
unsafe fn ZSTD_writeEpilogue(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    if (*cctx).stage as core::ffi::c_uint == ZSTDcs_created as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::stage_wrong.to_error_code();
    }
    if (*cctx).stage as core::ffi::c_uint == ZSTDcs_init as core::ffi::c_int as core::ffi::c_uint {
        let fhSize = ZSTD_writeFrameHeader(dst, dstCapacity, &(*cctx).appliedParams, 0, 0);
        let err_code = fhSize;
        if ERR_isError(err_code) {
            return err_code;
        }
        dstCapacity = dstCapacity.wrapping_sub(fhSize);
        op = op.add(fhSize);
        (*cctx).stage = ZSTDcs_ongoing;
    }
    if (*cctx).stage as core::ffi::c_uint != ZSTDcs_ending as core::ffi::c_int as core::ffi::c_uint
    {
        let cBlockHeader24 = 1u32
            .wrapping_add((bt_raw as core::ffi::c_int as u32) << 1)
            .wrapping_add(0);
        if dstCapacity < 3 as size_t {
            return Error::dstSize_tooSmall.to_error_code();
        }
        MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader24);
        op = op.add(ZSTD_blockHeaderSize);
        dstCapacity = dstCapacity.wrapping_sub(ZSTD_blockHeaderSize);
    }
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        let checksum = ZSTD_XXH64_digest(&mut (*cctx).xxhState) as u32;
        if dstCapacity < 4 {
            return Error::dstSize_tooSmall.to_error_code();
        }
        MEM_writeLE32(op as *mut core::ffi::c_void, checksum);
        op = op.add(4);
    }
    (*cctx).stage = ZSTDcs_created;
    op.offset_from_unsigned(ostart)
}
pub unsafe fn ZSTD_CCtx_trace(cctx: *mut ZSTD_CCtx, extraCSize: size_t) {
    if (*cctx).traceCtx != 0 {
        let streaming = core::ffi::c_int::from(
            (*cctx).inBuffSize > 0
                || (*cctx).outBuffSize > 0
                || (*cctx).appliedParams.nbWorkers > 0,
        );
        let mut trace = ZSTD_Trace::default();
        trace.version = ZSTD_VERSION_NUMBER as core::ffi::c_uint;
        trace.streaming = streaming;
        trace.dictionaryID = (*cctx).dictID;
        trace.dictionarySize = (*cctx).dictContentSize;
        trace.uncompressedSize = (*cctx).consumedSrcSize as size_t;
        trace.compressedSize =
            ((*cctx).producedCSize).wrapping_add(extraCSize as core::ffi::c_ulonglong) as size_t;
        trace.params = &mut (*cctx).appliedParams;
        trace.cctx = cctx;
        ZSTD_trace_compress_end((*cctx).traceCtx, &trace);
    }
    (*cctx).traceCtx = 0;
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressEnd_public))]
pub unsafe extern "C" fn ZSTD_compressEnd_public(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut endResult: size_t = 0;
    let cSize = ZSTD_compressContinue_internal(cctx, dst, dstCapacity, src, srcSize, 1, 1);
    let err_code = cSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    endResult = ZSTD_writeEpilogue(
        cctx,
        (dst as *mut core::ffi::c_char).add(cSize) as *mut core::ffi::c_void,
        dstCapacity.wrapping_sub(cSize),
    );
    let err_code_0 = endResult;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    if (*cctx).pledgedSrcSizePlusOne != 0
        && (*cctx).pledgedSrcSizePlusOne != ((*cctx).consumedSrcSize).wrapping_add(1)
    {
        return Error::srcSize_wrong.to_error_code();
    }
    ZSTD_CCtx_trace(cctx, endResult);
    cSize.wrapping_add(endResult)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressEnd))]
pub unsafe extern "C" fn ZSTD_compressEnd(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compress_advanced))]
pub unsafe extern "C" fn ZSTD_compress_advanced(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    params: ZSTD_parameters,
) -> size_t {
    let err_code = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_CCtxParams_init_internal(&mut (*cctx).simpleApiParams, &params, ZSTD_NO_CLEVEL);
    ZSTD_compress_advanced_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        dict,
        dictSize,
        &(*cctx).simpleApiParams,
    )
}
pub unsafe fn ZSTD_compress_advanced_internal(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    params: *const ZSTD_CCtx_params,
) -> size_t {
    let err_code = ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        core::ptr::null::<ZSTD_CDict>(),
        params,
        srcSize as u64,
        ZSTDb_not_buffered,
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compress_usingDict))]
pub unsafe extern "C" fn ZSTD_compress_usingDict(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    let params = ZSTD_getParams_internal(
        compressionLevel,
        srcSize as core::ffi::c_ulonglong,
        if !dict.is_null() { dictSize } else { 0 },
        ZSTD_cpm_noAttachDict,
    );
    ZSTD_CCtxParams_init_internal(
        &mut (*cctx).simpleApiParams,
        &params,
        if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        },
    );
    ZSTD_compress_advanced_internal(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        dict,
        dictSize,
        &(*cctx).simpleApiParams,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressCCtx))]
pub unsafe extern "C" fn ZSTD_compressCCtx(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    ZSTD_compress_usingDict(
        cctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        core::ptr::null(),
        0,
        compressionLevel,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compress))]
pub unsafe extern "C" fn ZSTD_compress(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    let mut result: size_t = 0;
    let mut ctxBody = ZSTD_CCtx_s {
        stage: ZSTDcs_created,
        cParamsChanged: 0,
        bmi2: 0,
        requestedParams: ZSTD_CCtx_params_s {
            format: Format::ZSTD_f_zstd1,
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
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
            literalCompressionMode: ParamSwitch::Auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem::default(),
            prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        },
        appliedParams: ZSTD_CCtx_params_s {
            format: Format::ZSTD_f_zstd1,
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
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
            literalCompressionMode: ParamSwitch::Auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem::default(),
            prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        },
        simpleApiParams: ZSTD_CCtx_params_s {
            format: Format::ZSTD_f_zstd1,
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
            compressionLevel: 0,
            forceWindow: 0,
            targetCBlockSize: 0,
            srcSizeHint: 0,
            attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
            literalCompressionMode: ParamSwitch::Auto,
            nbWorkers: 0,
            jobSize: 0,
            overlapLog: 0,
            rsyncable: 0,
            ldmParams: ldmParams_t {
                enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
                hashLog: 0,
                bucketSizeLog: 0,
                minMatchLength: 0,
                hashRateLog: 0,
                windowLog: 0,
            },
            enableDedicatedDictSearch: 0,
            inBufferMode: ZSTD_bm_buffered,
            outBufferMode: ZSTD_bm_buffered,
            blockDelimiters: ZSTD_sf_noBlockDelimiters,
            validateSequences: 0,
            postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem::default(),
            prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        },
        dictID: 0,
        dictContentSize: 0,
        workspace: ZSTD_cwksp {
            workspace: core::ptr::null_mut::<core::ffi::c_void>(),
            workspaceEnd: core::ptr::null_mut::<core::ffi::c_void>(),
            objectEnd: core::ptr::null_mut::<core::ffi::c_void>(),
            tableEnd: core::ptr::null_mut::<core::ffi::c_void>(),
            tableValidEnd: core::ptr::null_mut::<core::ffi::c_void>(),
            allocStart: core::ptr::null_mut::<core::ffi::c_void>(),
            initOnceStart: core::ptr::null_mut::<core::ffi::c_void>(),
            allocFailed: 0,
            workspaceOversizedDuration: 0,
            phase: ZSTD_cwksp_alloc_objects,
            isStatic: ZSTD_cwksp_dynamic_alloc,
        },
        blockSizeMax: 0,
        pledgedSrcSizePlusOne: 0,
        consumedSrcSize: 0,
        producedCSize: 0,
        xxhState: XXH64_state_t::default(),
        customMem: ZSTD_customMem::default(),
        pool: core::ptr::null_mut::<ZSTD_threadPool>(),
        staticSize: 0,
        seqCollector: SeqCollector {
            collectSequences: 0,
            seqStart: core::ptr::null_mut::<ZSTD_Sequence>(),
            seqIndex: 0,
            maxSequences: 0,
        },
        isFirstBlock: 0,
        initialized: 0,
        seqStore: SeqStore_t {
            sequencesStart: core::ptr::null_mut::<SeqDef>(),
            sequences: core::ptr::null_mut::<SeqDef>(),
            litStart: core::ptr::null_mut::<u8>(),
            lit: core::ptr::null_mut::<u8>(),
            llCode: core::ptr::null_mut::<u8>(),
            mlCode: core::ptr::null_mut::<u8>(),
            ofCode: core::ptr::null_mut::<u8>(),
            maxNbSeq: 0,
            maxNbLit: 0,
            longLengthType: ZSTD_llt_none,
            longLengthPos: 0,
        },
        ldmState: ldmState_t {
            window: ZSTD_window_t {
                nextSrc: core::ptr::null::<u8>(),
                base: core::ptr::null::<u8>(),
                dictBase: core::ptr::null::<u8>(),
                dictLimit: 0,
                lowLimit: 0,
                nbOverflowCorrections: 0,
            },
            hashTable: core::ptr::null_mut::<ldmEntry_t>(),
            loadedDictEnd: 0,
            bucketOffsets: core::ptr::null_mut::<u8>(),
            splitIndices: [0; 64],
            matchCandidates: [ldmMatchCandidate_t {
                split: core::ptr::null::<u8>(),
                hash: 0,
                checksum: 0,
                bucket: core::ptr::null_mut::<ldmEntry_t>(),
            }; 64],
        },
        ldmSequences: core::ptr::null_mut::<rawSeq>(),
        maxNbLdmSequences: 0,
        externSeqStore: RawSeqStore_t {
            seq: core::ptr::null_mut::<rawSeq>(),
            pos: 0,
            posInSequence: 0,
            size: 0,
            capacity: 0,
        },
        blockState: ZSTD_blockState_t {
            prevCBlock: core::ptr::null_mut::<ZSTD_compressedBlockState_t>(),
            nextCBlock: core::ptr::null_mut::<ZSTD_compressedBlockState_t>(),
            matchState: ZSTD_MatchState_t {
                window: ZSTD_window_t {
                    nextSrc: core::ptr::null::<u8>(),
                    base: core::ptr::null::<u8>(),
                    dictBase: core::ptr::null::<u8>(),
                    dictLimit: 0,
                    lowLimit: 0,
                    nbOverflowCorrections: 0,
                },
                loadedDictEnd: 0,
                nextToUpdate: 0,
                hashLog3: 0,
                rowHashLog: 0,
                tagTable: core::ptr::null_mut::<u8>(),
                hashCache: [0; 8],
                hashSalt: 0,
                hashSaltEntropy: 0,
                hashTable: core::ptr::null_mut::<u32>(),
                hashTable3: core::ptr::null_mut::<u32>(),
                chainTable: core::ptr::null_mut::<u32>(),
                forceNonContiguous: 0,
                dedicatedDictSearch: 0,
                opt: optState_t {
                    litFreq: core::ptr::null_mut::<core::ffi::c_uint>(),
                    litLengthFreq: core::ptr::null_mut::<core::ffi::c_uint>(),
                    matchLengthFreq: core::ptr::null_mut::<core::ffi::c_uint>(),
                    offCodeFreq: core::ptr::null_mut::<core::ffi::c_uint>(),
                    matchTable: core::ptr::null_mut::<ZSTD_match_t>(),
                    priceTable: core::ptr::null_mut::<ZSTD_optimal_t>(),
                    litSum: 0,
                    litLengthSum: 0,
                    matchLengthSum: 0,
                    offCodeSum: 0,
                    litSumBasePrice: 0,
                    litLengthSumBasePrice: 0,
                    matchLengthSumBasePrice: 0,
                    offCodeSumBasePrice: 0,
                    priceType: zop_dynamic,
                    symbolCosts: core::ptr::null::<ZSTD_entropyCTables_t>(),
                    literalCompressionMode: ParamSwitch::Auto,
                },
                dictMatchState: core::ptr::null::<ZSTD_MatchState_t>(),
                cParams: ZSTD_compressionParameters {
                    windowLog: 0,
                    chainLog: 0,
                    hashLog: 0,
                    searchLog: 0,
                    minMatch: 0,
                    targetLength: 0,
                    strategy: 0,
                },
                ldmSeqStore: core::ptr::null::<RawSeqStore_t>(),
                prefetchCDictTables: 0,
                lazySkipping: 0,
            },
        },
        tmpWorkspace: core::ptr::null_mut::<core::ffi::c_void>(),
        tmpWkspSize: 0,
        bufferedPolicy: ZSTDb_not_buffered,
        inBuff: core::ptr::null_mut(),
        inBuffSize: 0,
        inToCompress: 0,
        inBuffPos: 0,
        inBuffTarget: 0,
        outBuff: core::ptr::null_mut(),
        outBuffSize: 0,
        outBuffContentSize: 0,
        outBuffFlushedSize: 0,
        streamStage: zcss_init,
        frameEnded: 0,
        expectedInBuffer: ZSTD_inBuffer_s {
            src: core::ptr::null::<core::ffi::c_void>(),
            size: 0,
            pos: 0,
        },
        stableIn_notConsumed: 0,
        expectedOutBufferSize: 0,
        localDict: ZSTD_localDict {
            dictBuffer: core::ptr::null_mut::<core::ffi::c_void>(),
            dict: core::ptr::null::<core::ffi::c_void>(),
            dictSize: 0,
            dictContentType: ZSTD_dct_auto,
            cdict: core::ptr::null_mut::<ZSTD_CDict>(),
        },
        cdict: core::ptr::null::<ZSTD_CDict>(),
        prefixDict: ZSTD_prefixDict_s {
            dict: core::ptr::null::<core::ffi::c_void>(),
            dictSize: 0,
            dictContentType: ZSTD_dct_auto,
        },
        mtctx: core::ptr::null_mut::<ZSTDMT_CCtx>(),
        traceCtx: 0,
        blockSplitCtx: ZSTD_blockSplitCtx {
            fullSeqStoreChunk: SeqStore_t {
                sequencesStart: core::ptr::null_mut::<SeqDef>(),
                sequences: core::ptr::null_mut::<SeqDef>(),
                litStart: core::ptr::null_mut::<u8>(),
                lit: core::ptr::null_mut::<u8>(),
                llCode: core::ptr::null_mut::<u8>(),
                mlCode: core::ptr::null_mut::<u8>(),
                ofCode: core::ptr::null_mut::<u8>(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            firstHalfSeqStore: SeqStore_t {
                sequencesStart: core::ptr::null_mut::<SeqDef>(),
                sequences: core::ptr::null_mut::<SeqDef>(),
                litStart: core::ptr::null_mut::<u8>(),
                lit: core::ptr::null_mut::<u8>(),
                llCode: core::ptr::null_mut::<u8>(),
                mlCode: core::ptr::null_mut::<u8>(),
                ofCode: core::ptr::null_mut::<u8>(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            secondHalfSeqStore: SeqStore_t {
                sequencesStart: core::ptr::null_mut::<SeqDef>(),
                sequences: core::ptr::null_mut::<SeqDef>(),
                litStart: core::ptr::null_mut::<u8>(),
                lit: core::ptr::null_mut::<u8>(),
                llCode: core::ptr::null_mut::<u8>(),
                mlCode: core::ptr::null_mut::<u8>(),
                ofCode: core::ptr::null_mut::<u8>(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            currSeqStore: SeqStore_t {
                sequencesStart: core::ptr::null_mut::<SeqDef>(),
                sequences: core::ptr::null_mut::<SeqDef>(),
                litStart: core::ptr::null_mut::<u8>(),
                lit: core::ptr::null_mut::<u8>(),
                llCode: core::ptr::null_mut::<u8>(),
                mlCode: core::ptr::null_mut::<u8>(),
                ofCode: core::ptr::null_mut::<u8>(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            nextSeqStore: SeqStore_t {
                sequencesStart: core::ptr::null_mut::<SeqDef>(),
                sequences: core::ptr::null_mut::<SeqDef>(),
                litStart: core::ptr::null_mut::<u8>(),
                lit: core::ptr::null_mut::<u8>(),
                llCode: core::ptr::null_mut::<u8>(),
                mlCode: core::ptr::null_mut::<u8>(),
                ofCode: core::ptr::null_mut::<u8>(),
                maxNbSeq: 0,
                maxNbLit: 0,
                longLengthType: ZSTD_llt_none,
                longLengthPos: 0,
            },
            partitions: [0; 196],
            entropyMetadata: ZSTD_entropyCTablesMetadata_t {
                hufMetadata: ZSTD_hufCTablesMetadata_t {
                    hType: set_basic,
                    hufDesBuffer: [0; 128],
                    hufDesSize: 0,
                },
                fseMetadata: ZSTD_fseCTablesMetadata_t {
                    llType: set_basic,
                    ofType: set_basic,
                    mlType: set_basic,
                    fseTablesBuffer: [0; 133],
                    fseTablesSize: 0,
                    lastCountSize: 0,
                },
            },
        },
        extSeqBuf: core::ptr::null_mut::<ZSTD_Sequence>(),
        extSeqBufCapacity: 0,
    };
    ZSTD_initCCtx(&mut ctxBody, ZSTD_customMem::default());
    result = ZSTD_compressCCtx(
        &mut ctxBody,
        dst,
        dstCapacity,
        src,
        srcSize,
        compressionLevel,
    );
    ZSTD_freeCCtxContent(&mut ctxBody);
    result
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCDictSize_advanced))]
pub unsafe extern "C" fn ZSTD_estimateCDictSize_advanced(
    dictSize: size_t,
    cParams: ZSTD_compressionParameters,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
) -> size_t {
    (ZSTD_cwksp_alloc_size(::core::mem::size_of::<ZSTD_CDict>()))
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE))
        .wrapping_add(ZSTD_sizeof_matchState(
            &cParams,
            ZSTD_resolveRowMatchFinderMode(ZSTD_ParamSwitch_e::ZSTD_ps_auto, &cParams),
            1,
            0,
        ))
        .wrapping_add(
            if dictLoadMethod as core::ffi::c_uint
                == ZSTD_dlm_byRef as core::ffi::c_int as core::ffi::c_uint
            {
                0
            } else {
                ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(
                    dictSize,
                    ::core::mem::size_of::<*mut core::ffi::c_void>(),
                ))
            },
        )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCDictSize))]
pub unsafe extern "C" fn ZSTD_estimateCDictSize(
    dictSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    ZSTD_estimateCDictSize_advanced(dictSize, cParams, ZSTD_dlm_byCopy)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_CDict))]
pub unsafe extern "C" fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> size_t {
    if cdict.is_null() {
        return 0;
    }
    (if (*cdict).workspace.workspace == cdict as *mut core::ffi::c_void {
        0
    } else {
        ::core::mem::size_of::<ZSTD_CDict>()
    })
    .wrapping_add(ZSTD_cwksp_sizeof(&(*cdict).workspace))
}
unsafe fn ZSTD_initCDict_internal(
    cdict: *mut ZSTD_CDict,
    dictBuffer: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
    mut params: ZSTD_CCtx_params,
) -> size_t {
    (*cdict).matchState.cParams = params.cParams;
    (*cdict).matchState.dedicatedDictSearch = params.enableDedicatedDictSearch;
    if dictLoadMethod as core::ffi::c_uint
        == ZSTD_dlm_byRef as core::ffi::c_int as core::ffi::c_uint
        || dictBuffer.is_null()
        || dictSize == 0
    {
        (*cdict).dictContent = dictBuffer;
    } else {
        let internalBuffer = ZSTD_cwksp_reserve_object(
            &mut (*cdict).workspace,
            ZSTD_cwksp_align(dictSize, ::core::mem::size_of::<*mut core::ffi::c_void>()),
        );
        if internalBuffer.is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*cdict).dictContent = internalBuffer;
        libc::memcpy(internalBuffer, dictBuffer, dictSize as libc::size_t);
    }
    (*cdict).dictContentSize = dictSize;
    (*cdict).dictContentType = dictContentType;
    (*cdict).entropyWorkspace =
        ZSTD_cwksp_reserve_object(&mut (*cdict).workspace, HUF_WORKSPACE_SIZE) as *mut u32;
    ZSTD_reset_compressedBlockState(&mut (*cdict).cBlockState);
    let err_code = ZSTD_reset_matchState(
        &mut (*cdict).matchState,
        &mut (*cdict).workspace,
        &params.cParams,
        params.useRowMatchFinder,
        ZSTDcrp_makeClean,
        ZSTDirp_reset,
        ZSTD_resetTarget_CDict,
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    params.compressionLevel = ZSTD_CLEVEL_DEFAULT;
    params.fParams.contentSizeFlag = 1;
    let dictID = ZSTD_compress_insertDictionary(
        &mut (*cdict).cBlockState,
        &mut (*cdict).matchState,
        core::ptr::null_mut(),
        &mut (*cdict).workspace,
        &params,
        (*cdict).dictContent,
        (*cdict).dictContentSize,
        dictContentType,
        ZSTD_dtlm_full,
        ZSTD_tfp_forCDict,
        (*cdict).entropyWorkspace as *mut core::ffi::c_void,
    );
    let err_code_0 = dictID;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    (*cdict).dictID = dictID as u32;
    0
}
unsafe fn ZSTD_createCDict_advanced_internal(
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    cParams: ZSTD_compressionParameters,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    enableDedicatedDictSearch: core::ffi::c_int,
    customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let workspaceSize = (ZSTD_cwksp_alloc_size(::core::mem::size_of::<ZSTD_CDict>()))
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE))
        .wrapping_add(ZSTD_sizeof_matchState(
            &cParams,
            useRowMatchFinder,
            enableDedicatedDictSearch,
            0,
        ))
        .wrapping_add(if dictLoadMethod == ZSTD_dlm_byRef {
            0
        } else {
            ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(
                dictSize,
                ::core::mem::size_of::<*mut core::ffi::c_void>(),
            ))
        });
    let workspace = ZSTD_customMalloc(workspaceSize, customMem);
    let mut ws = ZSTD_cwksp {
        workspace: core::ptr::null_mut::<core::ffi::c_void>(),
        workspaceEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        objectEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        tableEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        tableValidEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        allocStart: core::ptr::null_mut::<core::ffi::c_void>(),
        initOnceStart: core::ptr::null_mut::<core::ffi::c_void>(),
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    if workspace.is_null() {
        return core::ptr::null_mut();
    }
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_dynamic_alloc);
    let cdict =
        ZSTD_cwksp_reserve_object(&mut ws, ::core::mem::size_of::<ZSTD_CDict>()) as *mut ZSTD_CDict;
    ZSTD_cwksp_move(&mut (*cdict).workspace, &mut ws);
    (*cdict).customMem = customMem;
    (*cdict).compressionLevel = ZSTD_NO_CLEVEL;
    (*cdict).useRowMatchFinder = useRowMatchFinder;
    cdict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCDict_advanced))]
pub unsafe extern "C" fn ZSTD_createCDict_advanced(
    dictBuffer: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
    cParams: ZSTD_compressionParameters,
    customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: Format::ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
        literalCompressionMode: ParamSwitch::Auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    };
    ptr::write_bytes(
        &mut cctxParams as *mut ZSTD_CCtx_params as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_CCtx_params>(),
    );
    ZSTD_CCtxParams_init(&mut cctxParams, 0);
    cctxParams.cParams = cParams;
    cctxParams.customMem = customMem;
    ZSTD_createCDict_advanced2(
        dictBuffer,
        dictSize,
        dictLoadMethod,
        dictContentType,
        &cctxParams,
        customMem,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCDict_advanced2))]
pub unsafe extern "C" fn ZSTD_createCDict_advanced2(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
    originalCctxParams: *const ZSTD_CCtx_params,
    customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let mut cctxParams = *originalCctxParams;
    let mut cParams = ZSTD_compressionParameters {
        windowLog: 0,
        chainLog: 0,
        hashLog: 0,
        searchLog: 0,
        minMatch: 0,
        targetLength: 0,
        strategy: 0,
    };
    if cctxParams.enableDedicatedDictSearch != 0 {
        cParams = ZSTD_dedicatedDictSearch_getCParams(cctxParams.compressionLevel, dictSize);
        ZSTD_overrideCParams(&mut cParams, &cctxParams.cParams);
    } else {
        cParams = ZSTD_getCParamsFromCCtxParams(
            &cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN,
            dictSize,
            ZSTD_cpm_createCDict,
        );
    }
    if ZSTD_dedicatedDictSearch_isSupported(&cParams) == 0 {
        cctxParams.enableDedicatedDictSearch = 0;
        cParams = ZSTD_getCParamsFromCCtxParams(
            &cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN,
            dictSize,
            ZSTD_cpm_createCDict,
        );
    }
    cctxParams.cParams = cParams;
    cctxParams.useRowMatchFinder =
        ZSTD_resolveRowMatchFinderMode(cctxParams.useRowMatchFinder, &cParams);
    let cdict = ZSTD_createCDict_advanced_internal(
        dictSize,
        dictLoadMethod,
        cctxParams.cParams,
        cctxParams.useRowMatchFinder,
        cctxParams.enableDedicatedDictSearch,
        customMem,
    );
    if cdict.is_null()
        || ERR_isError(ZSTD_initCDict_internal(
            cdict,
            dict,
            dictSize,
            dictLoadMethod,
            dictContentType,
            cctxParams,
        ))
    {
        ZSTD_freeCDict(cdict);
        return core::ptr::null_mut();
    }
    cdict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCDict))]
pub unsafe extern "C" fn ZSTD_createCDict(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> *mut ZSTD_CDict {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    let cdict = ZSTD_createCDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byCopy,
        ZSTD_dct_auto,
        cParams,
        ZSTD_customMem::default(),
    );
    if !cdict.is_null() {
        (*cdict).compressionLevel = if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        };
    }
    cdict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCDict_byReference))]
pub unsafe extern "C" fn ZSTD_createCDict_byReference(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> *mut ZSTD_CDict {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        ZSTD_cpm_createCDict,
    );
    let cdict = ZSTD_createCDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
        cParams,
        ZSTD_customMem::default(),
    );
    if !cdict.is_null() {
        (*cdict).compressionLevel = if compressionLevel == 0 {
            ZSTD_CLEVEL_DEFAULT
        } else {
            compressionLevel
        };
    }
    cdict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeCDict))]
pub unsafe extern "C" fn ZSTD_freeCDict(cdict: *mut ZSTD_CDict) -> size_t {
    if cdict.is_null() {
        return 0;
    }
    let cMem = (*cdict).customMem;
    let cdictInWorkspace =
        ZSTD_cwksp_owns_buffer(&(*cdict).workspace, cdict as *const core::ffi::c_void);
    ZSTD_cwksp_free(&mut (*cdict).workspace, cMem);
    if cdictInWorkspace == 0 {
        ZSTD_customFree(
            cdict as *mut core::ffi::c_void,
            (*cdict).dictContentSize,
            cMem,
        );
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initStaticCDict))]
pub unsafe extern "C" fn ZSTD_initStaticCDict(
    workspace: *mut core::ffi::c_void,
    workspaceSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
    cParams: ZSTD_compressionParameters,
) -> *const ZSTD_CDict {
    let useRowMatchFinder =
        ZSTD_resolveRowMatchFinderMode(ZSTD_ParamSwitch_e::ZSTD_ps_auto, &cParams);
    let matchStateSize = ZSTD_sizeof_matchState(&cParams, useRowMatchFinder, 1, 0);
    let neededSize = (ZSTD_cwksp_alloc_size(::core::mem::size_of::<ZSTD_CDict>()))
        .wrapping_add(if dictLoadMethod == ZSTD_dlm_byRef {
            0
        } else {
            ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(
                dictSize,
                ::core::mem::size_of::<*mut core::ffi::c_void>(),
            ))
        })
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE))
        .wrapping_add(matchStateSize);
    let mut cdict = core::ptr::null_mut::<ZSTD_CDict>();
    let mut params = ZSTD_CCtx_params_s {
        format: Format::ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
        literalCompressionMode: ParamSwitch::Auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    };
    if workspace as size_t & 7 != 0 {
        return core::ptr::null();
    }
    let mut ws = ZSTD_cwksp {
        workspace: core::ptr::null_mut::<core::ffi::c_void>(),
        workspaceEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        objectEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        tableEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        tableValidEnd: core::ptr::null_mut::<core::ffi::c_void>(),
        allocStart: core::ptr::null_mut::<core::ffi::c_void>(),
        initOnceStart: core::ptr::null_mut::<core::ffi::c_void>(),
        allocFailed: 0,
        workspaceOversizedDuration: 0,
        phase: ZSTD_cwksp_alloc_objects,
        isStatic: ZSTD_cwksp_dynamic_alloc,
    };
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, ZSTD_cwksp_static_alloc);
    cdict =
        ZSTD_cwksp_reserve_object(&mut ws, ::core::mem::size_of::<ZSTD_CDict>()) as *mut ZSTD_CDict;
    if cdict.is_null() {
        return core::ptr::null();
    }
    ZSTD_cwksp_move(&mut (*cdict).workspace, &mut ws);
    if workspaceSize < neededSize {
        return core::ptr::null();
    }
    ZSTD_CCtxParams_init(&mut params, 0);
    params.cParams = cParams;
    params.useRowMatchFinder = useRowMatchFinder;
    (*cdict).useRowMatchFinder = useRowMatchFinder;
    (*cdict).compressionLevel = ZSTD_NO_CLEVEL;
    if ERR_isError(ZSTD_initCDict_internal(
        cdict,
        dict,
        dictSize,
        dictLoadMethod,
        dictContentType,
        params,
    )) {
        return core::ptr::null();
    }
    cdict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getCParamsFromCDict))]
pub unsafe extern "C" fn ZSTD_getCParamsFromCDict(
    cdict: *const ZSTD_CDict,
) -> ZSTD_compressionParameters {
    (*cdict).matchState.cParams
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getDictID_fromCDict))]
pub unsafe extern "C" fn ZSTD_getDictID_fromCDict(cdict: *const ZSTD_CDict) -> core::ffi::c_uint {
    if cdict.is_null() {
        return 0;
    }
    (*cdict).dictID
}
unsafe fn ZSTD_compressBegin_usingCDict_internal(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    let mut cctxParams = ZSTD_CCtx_params_s {
        format: Format::ZSTD_f_zstd1,
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
        compressionLevel: 0,
        forceWindow: 0,
        targetCBlockSize: 0,
        srcSizeHint: 0,
        attachDictPref: ZSTD_dictAttachPref_e::ZSTD_dictDefaultAttach,
        literalCompressionMode: ParamSwitch::Auto,
        nbWorkers: 0,
        jobSize: 0,
        overlapLog: 0,
        rsyncable: 0,
        ldmParams: ldmParams_t {
            enableLdm: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            hashLog: 0,
            bucketSizeLog: 0,
            minMatchLength: 0,
            hashRateLog: 0,
            windowLog: 0,
        },
        enableDedicatedDictSearch: 0,
        inBufferMode: ZSTD_bm_buffered,
        outBufferMode: ZSTD_bm_buffered,
        blockDelimiters: ZSTD_sf_noBlockDelimiters,
        validateSequences: 0,
        postBlockSplitter: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    };
    if cdict.is_null() {
        return Error::dictionary_wrong.to_error_code();
    }
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
    params.fParams = fParams;
    params.cParams = if pledgedSrcSize
        < ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF as core::ffi::c_ulonglong
        || pledgedSrcSize
            < ((*cdict).dictContentSize as core::ffi::c_ulonglong)
                .wrapping_mul(ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER)
        || pledgedSrcSize == ZSTD_CONTENTSIZE_UNKNOWN
        || (*cdict).compressionLevel == 0
    {
        ZSTD_getCParamsFromCDict(cdict)
    } else {
        ZSTD_getCParams(
            (*cdict).compressionLevel,
            pledgedSrcSize,
            (*cdict).dictContentSize,
        )
    };
    ZSTD_CCtxParams_init_internal(&mut cctxParams, &params, (*cdict).compressionLevel);
    if pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN {
        let limitedSrcSize = (if pledgedSrcSize < ((1) << 19) as core::ffi::c_ulonglong {
            pledgedSrcSize
        } else {
            ((1) << 19) as core::ffi::c_ulonglong
        }) as u32;
        let limitedSrcLog = if limitedSrcSize > 1 {
            (ZSTD_highbit32(limitedSrcSize.wrapping_sub(1))).wrapping_add(1)
        } else {
            1
        };
        cctxParams.cParams.windowLog = if cctxParams.cParams.windowLog > limitedSrcLog {
            cctxParams.cParams.windowLog
        } else {
            limitedSrcLog
        };
    }
    ZSTD_compressBegin_internal(
        cctx,
        core::ptr::null(),
        0,
        ZSTD_dct_auto,
        ZSTD_dtlm_fast,
        cdict,
        &cctxParams,
        pledgedSrcSize,
        ZSTDb_not_buffered,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBegin_usingCDict_advanced))]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict_advanced(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    ZSTD_compressBegin_usingCDict_internal(cctx, cdict, fParams, pledgedSrcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBegin_usingCDict_deprecated))]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict_deprecated(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
) -> size_t {
    let fParams = {
        ZSTD_frameParameters {
            contentSizeFlag: 0,
            checksumFlag: 0,
            noDictIDFlag: 0,
        }
    };
    ZSTD_compressBegin_usingCDict_internal(cctx, cdict, fParams, ZSTD_CONTENTSIZE_UNKNOWN)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBegin_usingCDict))]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
) -> size_t {
    ZSTD_compressBegin_usingCDict_deprecated(cctx, cdict)
}
unsafe fn ZSTD_compress_usingCDict_internal(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
) -> size_t {
    let err_code = ZSTD_compressBegin_usingCDict_internal(
        cctx,
        cdict,
        fParams,
        srcSize as core::ffi::c_ulonglong,
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_compressEnd_public(cctx, dst, dstCapacity, src, srcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compress_usingCDict_advanced))]
pub unsafe extern "C" fn ZSTD_compress_usingCDict_advanced(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
) -> size_t {
    ZSTD_compress_usingCDict_internal(cctx, dst, dstCapacity, src, srcSize, cdict, fParams)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compress_usingCDict))]
pub unsafe extern "C" fn ZSTD_compress_usingCDict(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    cdict: *const ZSTD_CDict,
) -> size_t {
    let fParams = {
        ZSTD_frameParameters {
            contentSizeFlag: 1,
            checksumFlag: 0,
            noDictIDFlag: 0,
        }
    };
    ZSTD_compress_usingCDict_internal(cctx, dst, dstCapacity, src, srcSize, cdict, fParams)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCStream))]
pub unsafe extern "C" fn ZSTD_createCStream() -> *mut ZSTD_CStream {
    ZSTD_createCStream_advanced(ZSTD_customMem::default())
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initStaticCStream))]
pub unsafe extern "C" fn ZSTD_initStaticCStream(
    workspace: *mut core::ffi::c_void,
    workspaceSize: size_t,
) -> *mut ZSTD_CStream {
    ZSTD_initStaticCCtx(workspace, workspaceSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCStream_advanced))]
pub unsafe extern "C" fn ZSTD_createCStream_advanced(
    customMem: ZSTD_customMem,
) -> *mut ZSTD_CStream {
    ZSTD_createCCtx_advanced(customMem)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeCStream))]
pub unsafe extern "C" fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t {
    ZSTD_freeCCtx(zcs)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CStreamInSize))]
pub unsafe extern "C" fn ZSTD_CStreamInSize() -> size_t {
    ZSTD_BLOCKSIZE_MAX as size_t
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CStreamOutSize))]
pub unsafe extern "C" fn ZSTD_CStreamOutSize() -> size_t {
    (ZSTD_compressBound(ZSTD_BLOCKSIZE_MAX as size_t))
        .wrapping_add(ZSTD_blockHeaderSize)
        .wrapping_add(4)
}
unsafe fn ZSTD_getCParamMode(
    cdict: *const ZSTD_CDict,
    params: *const ZSTD_CCtx_params,
    pledgedSrcSize: u64,
) -> ZSTD_CParamMode_e {
    if !cdict.is_null() && ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) != 0 {
        ZSTD_cpm_attachDict
    } else {
        ZSTD_cpm_noAttachDict
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_resetCStream))]
pub unsafe extern "C" fn ZSTD_resetCStream(
    zcs: *mut ZSTD_CStream,
    pss: core::ffi::c_ulonglong,
) -> size_t {
    let pledgedSrcSize = if pss == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    };
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize as core::ffi::c_ulonglong);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    0
}
pub unsafe fn ZSTD_initCStream_internal(
    zcs: *mut ZSTD_CStream,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    cdict: *const ZSTD_CDict,
    params: *const ZSTD_CCtx_params,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    (*zcs).requestedParams = *params;
    if !dict.is_null() {
        let err_code_1 = ZSTD_CCtx_loadDictionary(zcs, dict, dictSize);
        if ERR_isError(err_code_1) {
            return err_code_1;
        }
    } else {
        let err_code_2 = ZSTD_CCtx_refCDict(zcs, cdict);
        if ERR_isError(err_code_2) {
            return err_code_2;
        }
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initCStream_usingCDict_advanced))]
pub unsafe extern "C" fn ZSTD_initCStream_usingCDict_advanced(
    zcs: *mut ZSTD_CStream,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    (*zcs).requestedParams.fParams = fParams;
    let err_code_1 = ZSTD_CCtx_refCDict(zcs, cdict);
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initCStream_usingCDict))]
pub unsafe extern "C" fn ZSTD_initCStream_usingCDict(
    zcs: *mut ZSTD_CStream,
    cdict: *const ZSTD_CDict,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_refCDict(zcs, cdict);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initCStream_advanced))]
pub unsafe extern "C" fn ZSTD_initCStream_advanced(
    zcs: *mut ZSTD_CStream,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    params: ZSTD_parameters,
    pss: core::ffi::c_ulonglong,
) -> size_t {
    let pledgedSrcSize = if pss == 0 && params.fParams.contentSizeFlag == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    };
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize as core::ffi::c_ulonglong);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_checkCParams(params.cParams);
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    ZSTD_CCtxParams_setZstdParams(&mut (*zcs).requestedParams, &params);
    let err_code_2 = ZSTD_CCtx_loadDictionary(zcs, dict, dictSize);
    if ERR_isError(err_code_2) {
        return err_code_2;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initCStream_usingDict))]
pub unsafe extern "C" fn ZSTD_initCStream_usingDict(
    zcs: *mut ZSTD_CStream,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setParameter(
        zcs,
        ZSTD_cParameter::ZSTD_c_compressionLevel,
        compressionLevel,
    );
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_loadDictionary(zcs, dict, dictSize);
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initCStream_srcSize))]
pub unsafe extern "C" fn ZSTD_initCStream_srcSize(
    zcs: *mut ZSTD_CStream,
    compressionLevel: core::ffi::c_int,
    pss: core::ffi::c_ulonglong,
) -> size_t {
    let pledgedSrcSize = if pss == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        pss
    };
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_refCDict(zcs, core::ptr::null::<ZSTD_CDict>());
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        zcs,
        ZSTD_cParameter::ZSTD_c_compressionLevel,
        compressionLevel,
    );
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    let err_code_2 = ZSTD_CCtx_setPledgedSrcSize(zcs, pledgedSrcSize as core::ffi::c_ulonglong);
    if ERR_isError(err_code_2) {
        return err_code_2;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initCStream))]
pub unsafe extern "C" fn ZSTD_initCStream(
    zcs: *mut ZSTD_CStream,
    compressionLevel: core::ffi::c_int,
) -> size_t {
    let err_code = ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_refCDict(zcs, core::ptr::null::<ZSTD_CDict>());
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        zcs,
        ZSTD_cParameter::ZSTD_c_compressionLevel,
        compressionLevel,
    );
    if ERR_isError(err_code_1) {
        return err_code_1;
    }
    0
}
unsafe fn ZSTD_nextInputSizeHint(cctx: *const ZSTD_CCtx) -> size_t {
    if (*cctx).appliedParams.inBufferMode as core::ffi::c_uint
        == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
    {
        return ((*cctx).blockSizeMax).wrapping_sub((*cctx).stableIn_notConsumed);
    }
    let mut hintInSize = ((*cctx).inBuffTarget).wrapping_sub((*cctx).inBuffPos);
    if hintInSize == 0 {
        hintInSize = (*cctx).blockSizeMax;
    }
    hintInSize
}
unsafe fn ZSTD_compressStream_generic(
    zcs: *mut ZSTD_CStream,
    output: *mut ZSTD_outBuffer,
    input: *mut ZSTD_inBuffer,
    flushMode: ZSTD_EndDirective,
) -> size_t {
    let istart = (*input).src as *const u8;
    let iend = if !istart.is_null() {
        istart.add((*input).size)
    } else {
        istart
    };
    let mut ip = if !istart.is_null() {
        istart.add((*input).pos)
    } else {
        istart
    };
    let ostart = (*output).dst as *mut u8;
    let oend = if !ostart.is_null() {
        ostart.add((*output).size)
    } else {
        ostart
    };
    let mut op = if !ostart.is_null() {
        ostart.add((*output).pos)
    } else {
        ostart
    };
    let mut someMoreWork = 1;
    if (*zcs).appliedParams.inBufferMode as core::ffi::c_uint
        == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
    {
        (*input).pos = ((*input).pos).wrapping_sub((*zcs).stableIn_notConsumed);
        if !ip.is_null() {
            ip = ip.offset(-((*zcs).stableIn_notConsumed as isize));
        }
        (*zcs).stableIn_notConsumed = 0;
    }
    if (*zcs).appliedParams.inBufferMode == ZSTD_bm_buffered {
        assert!(!(*zcs).inBuff.is_null());
        assert!((*zcs).inBuffSize > 0);
    }
    if (*zcs).appliedParams.outBufferMode == ZSTD_bm_buffered {
        assert!(!(*zcs).outBuff.is_null());
        assert!((*zcs).outBuffSize > 0);
    }
    ((*input).src).is_null();
    ((*output).dst).is_null();
    while someMoreWork != 0 {
        let mut current_block_156: u64;
        match (*zcs).streamStage as core::ffi::c_uint {
            0 => return Error::init_missing.to_error_code(),
            1 => {
                if flushMode as core::ffi::c_uint
                    == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
                    && (oend.offset_from_unsigned(op)
                        >= ZSTD_compressBound(iend.offset_from_unsigned(ip))
                        || (*zcs).appliedParams.outBufferMode as core::ffi::c_uint
                            == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint)
                    && (*zcs).inBuffPos == 0
                {
                    let cSize = ZSTD_compressEnd_public(
                        zcs,
                        op as *mut core::ffi::c_void,
                        oend.offset_from_unsigned(op),
                        ip as *const core::ffi::c_void,
                        iend.offset_from_unsigned(ip),
                    );
                    let err_code = cSize;
                    if ERR_isError(err_code) {
                        return err_code;
                    }
                    ip = iend;
                    op = op.add(cSize);
                    (*zcs).frameEnded = 1;
                    ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
                    someMoreWork = 0;
                    current_block_156 = 16754622181974910496;
                } else {
                    if (*zcs).appliedParams.inBufferMode as core::ffi::c_uint
                        == ZSTD_bm_buffered as core::ffi::c_int as core::ffi::c_uint
                    {
                        let toLoad = ((*zcs).inBuffTarget).wrapping_sub((*zcs).inBuffPos);
                        let loaded = ZSTD_limitCopy(
                            ((*zcs).inBuff).add((*zcs).inBuffPos),
                            toLoad,
                            ip,
                            iend.offset_from_unsigned(ip),
                        );
                        (*zcs).inBuffPos = ((*zcs).inBuffPos).wrapping_add(loaded);
                        if !ip.is_null() {
                            ip = ip.add(loaded);
                        }
                        if flushMode as core::ffi::c_uint
                            == ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
                            && (*zcs).inBuffPos < (*zcs).inBuffTarget
                        {
                            someMoreWork = 0;
                            current_block_156 = 16754622181974910496;
                        } else if flushMode as core::ffi::c_uint
                            == ZSTD_e_flush as core::ffi::c_int as core::ffi::c_uint
                            && (*zcs).inBuffPos == (*zcs).inToCompress
                        {
                            someMoreWork = 0;
                            current_block_156 = 16754622181974910496;
                        } else {
                            current_block_156 = 13910774313357589740;
                        }
                    } else if flushMode as core::ffi::c_uint
                        == ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
                        && (iend.offset_from_unsigned(ip)) < (*zcs).blockSizeMax
                    {
                        (*zcs).stableIn_notConsumed = iend.offset_from_unsigned(ip);
                        ip = iend;
                        someMoreWork = 0;
                        current_block_156 = 16754622181974910496;
                    } else if flushMode as core::ffi::c_uint
                        == ZSTD_e_flush as core::ffi::c_int as core::ffi::c_uint
                        && ip == iend
                    {
                        someMoreWork = 0;
                        current_block_156 = 16754622181974910496;
                    } else {
                        current_block_156 = 13910774313357589740;
                    }
                    match current_block_156 {
                        16754622181974910496 => {}
                        _ => {
                            let inputBuffered = core::ffi::c_int::from(
                                (*zcs).appliedParams.inBufferMode as core::ffi::c_uint
                                    == ZSTD_bm_buffered as core::ffi::c_int as core::ffi::c_uint,
                            );
                            let mut cDst = core::ptr::null_mut::<core::ffi::c_void>();
                            let mut cSize_0: size_t = 0;
                            let mut oSize = oend.offset_from_unsigned(op);
                            let iSize = if inputBuffered != 0 {
                                ((*zcs).inBuffPos).wrapping_sub((*zcs).inToCompress)
                            } else if (iend.offset_from_unsigned(ip)) < (*zcs).blockSizeMax {
                                iend.offset_from_unsigned(ip)
                            } else {
                                (*zcs).blockSizeMax
                            };
                            if oSize >= ZSTD_compressBound(iSize)
                                || (*zcs).appliedParams.outBufferMode as core::ffi::c_uint
                                    == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
                            {
                                cDst = op as *mut core::ffi::c_void;
                            } else {
                                cDst = (*zcs).outBuff as *mut core::ffi::c_void;
                                oSize = (*zcs).outBuffSize;
                            }
                            if inputBuffered != 0 {
                                let lastBlock = core::ffi::c_int::from(
                                    flushMode as core::ffi::c_uint
                                        == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
                                        && ip == iend,
                                )
                                    as core::ffi::c_uint;
                                cSize_0 = if lastBlock != 0 {
                                    ZSTD_compressEnd_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ((*zcs).inBuff).add((*zcs).inToCompress)
                                            as *const core::ffi::c_void,
                                        iSize,
                                    )
                                } else {
                                    ZSTD_compressContinue_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ((*zcs).inBuff).add((*zcs).inToCompress)
                                            as *const core::ffi::c_void,
                                        iSize,
                                    )
                                };
                                let err_code_0 = cSize_0;
                                if ERR_isError(err_code_0) {
                                    return err_code_0;
                                }
                                (*zcs).frameEnded = lastBlock;
                                (*zcs).inBuffTarget =
                                    ((*zcs).inBuffPos).wrapping_add((*zcs).blockSizeMax);
                                if (*zcs).inBuffTarget > (*zcs).inBuffSize {
                                    (*zcs).inBuffPos = 0;
                                    (*zcs).inBuffTarget = (*zcs).blockSizeMax;
                                }
                                if lastBlock == 0 {
                                    assert!((*zcs).inBuffTarget <= (*zcs).inBuffSize);
                                }
                                (*zcs).inToCompress = (*zcs).inBuffPos;
                            } else {
                                let lastBlock_0 = core::ffi::c_int::from(
                                    flushMode as core::ffi::c_uint
                                        == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
                                        && ip.add(iSize) == iend,
                                )
                                    as core::ffi::c_uint;
                                cSize_0 = if lastBlock_0 != 0 {
                                    ZSTD_compressEnd_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ip as *const core::ffi::c_void,
                                        iSize,
                                    )
                                } else {
                                    ZSTD_compressContinue_public(
                                        zcs,
                                        cDst,
                                        oSize,
                                        ip as *const core::ffi::c_void,
                                        iSize,
                                    )
                                };
                                if !ip.is_null() {
                                    ip = ip.add(iSize);
                                }
                                let err_code_1 = cSize_0;
                                if ERR_isError(err_code_1) {
                                    return err_code_1;
                                }
                                (*zcs).frameEnded = lastBlock_0;
                                if lastBlock_0 != 0 {
                                    assert_eq!(ip, iend);
                                }
                            }
                            if cDst == op as *mut core::ffi::c_void {
                                op = op.add(cSize_0);
                                if (*zcs).frameEnded != 0 {
                                    someMoreWork = 0;
                                    ZSTD_CCtx_reset(
                                        zcs,
                                        ZSTD_ResetDirective::ZSTD_reset_session_only,
                                    );
                                }
                                current_block_156 = 16754622181974910496;
                            } else {
                                (*zcs).outBuffContentSize = cSize_0;
                                (*zcs).outBuffFlushedSize = 0;
                                (*zcs).streamStage = zcss_flush;
                                current_block_156 = 5431927413890720344;
                            }
                        }
                    }
                }
            }
            2 => {
                current_block_156 = 5431927413890720344;
            }
            _ => {
                current_block_156 = 16754622181974910496;
            }
        }
        if current_block_156 == 5431927413890720344 {
            let toFlush = ((*zcs).outBuffContentSize).wrapping_sub((*zcs).outBuffFlushedSize);
            let flushed = ZSTD_limitCopy(
                op,
                oend.offset_from_unsigned(op),
                ((*zcs).outBuff).add((*zcs).outBuffFlushedSize),
                toFlush,
            );
            if flushed != 0 {
                op = op.add(flushed);
            }
            (*zcs).outBuffFlushedSize = ((*zcs).outBuffFlushedSize).wrapping_add(flushed);
            if toFlush != flushed {
                someMoreWork = 0;
            } else {
                (*zcs).outBuffFlushedSize = 0;
                (*zcs).outBuffContentSize = (*zcs).outBuffFlushedSize;
                if (*zcs).frameEnded != 0 {
                    someMoreWork = 0;
                    ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
                } else {
                    (*zcs).streamStage = zcss_load;
                }
            }
        }
    }
    (*input).pos = ip.offset_from_unsigned(istart);
    (*output).pos = op.offset_from_unsigned(ostart);
    if (*zcs).frameEnded != 0 {
        return 0;
    }
    ZSTD_nextInputSizeHint(zcs)
}
unsafe fn ZSTD_nextInputSizeHint_MTorST(cctx: *const ZSTD_CCtx) -> size_t {
    if (*cctx).appliedParams.nbWorkers >= 1 {
        return ZSTDMT_nextInputSizeHint((*cctx).mtctx);
    }
    ZSTD_nextInputSizeHint(cctx)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressStream))]
pub unsafe extern "C" fn ZSTD_compressStream(
    zcs: *mut ZSTD_CStream,
    output: *mut ZSTD_outBuffer,
    input: *mut ZSTD_inBuffer,
) -> size_t {
    let err_code = ZSTD_compressStream2(zcs, output, input, ZSTD_e_continue);
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_nextInputSizeHint_MTorST(zcs)
}
unsafe fn ZSTD_setBufferExpectations(
    cctx: *mut ZSTD_CCtx,
    output: *const ZSTD_outBuffer,
    input: *const ZSTD_inBuffer,
) {
    if (*cctx).appliedParams.inBufferMode as core::ffi::c_uint
        == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
    {
        (*cctx).expectedInBuffer = *input;
    }
    if (*cctx).appliedParams.outBufferMode as core::ffi::c_uint
        == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
    {
        (*cctx).expectedOutBufferSize = ((*output).size).wrapping_sub((*output).pos);
    }
}
unsafe fn ZSTD_checkBufferStability(
    cctx: *const ZSTD_CCtx,
    output: *const ZSTD_outBuffer,
    input: *const ZSTD_inBuffer,
    _endOp: ZSTD_EndDirective,
) -> size_t {
    if (*cctx).appliedParams.inBufferMode as core::ffi::c_uint
        == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
    {
        let expect = (*cctx).expectedInBuffer;
        if expect.src != (*input).src || expect.pos != (*input).pos {
            return Error::stabilityCondition_notRespected.to_error_code();
        }
    }
    if (*cctx).appliedParams.outBufferMode as core::ffi::c_uint
        == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
    {
        let outBufferSize = ((*output).size).wrapping_sub((*output).pos);
        if (*cctx).expectedOutBufferSize != outBufferSize {
            return Error::stabilityCondition_notRespected.to_error_code();
        }
    }
    0
}
unsafe fn ZSTD_CCtx_init_compressStream2(
    cctx: *mut ZSTD_CCtx,
    endOp: ZSTD_EndDirective,
    inSize: size_t,
) -> size_t {
    let mut params = (*cctx).requestedParams;
    let prefixDict = (*cctx).prefixDict;
    let err_code = ZSTD_initLocalDict(cctx);
    if ERR_isError(err_code) {
        return err_code;
    }
    ptr::write_bytes(
        &mut (*cctx).prefixDict as *mut ZSTD_prefixDict as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_prefixDict>(),
    );
    if !((*cctx).cdict).is_null() && ((*cctx).localDict.cdict).is_null() {
        params.compressionLevel = (*(*cctx).cdict).compressionLevel;
    }
    if endOp as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint {
        (*cctx).pledgedSrcSizePlusOne = inSize.wrapping_add(1) as core::ffi::c_ulonglong;
    }
    let dictSize = if !(prefixDict.dict).is_null() {
        prefixDict.dictSize
    } else if !((*cctx).cdict).is_null() {
        (*(*cctx).cdict).dictContentSize
    } else {
        0
    };
    let mode = ZSTD_getCParamMode(
        (*cctx).cdict,
        &params,
        ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1),
    );
    params.cParams = ZSTD_getCParamsFromCCtxParams(
        &params,
        ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1),
        dictSize,
        mode,
    );
    params.postBlockSplitter =
        ZSTD_resolveBlockSplitterMode(params.postBlockSplitter, &params.cParams);
    params.ldmParams.enableLdm = ZSTD_resolveEnableLdm(params.ldmParams.enableLdm, &params.cParams);
    params.useRowMatchFinder =
        ZSTD_resolveRowMatchFinderMode(params.useRowMatchFinder, &params.cParams);
    params.validateSequences = ZSTD_resolveExternalSequenceValidation(params.validateSequences);
    params.maxBlockSize = ZSTD_resolveMaxBlockSize(params.maxBlockSize);
    params.searchForExternalRepcodes = ZSTD_resolveExternalRepcodeSearch(
        params.searchForExternalRepcodes,
        params.compressionLevel,
    );
    if ZSTD_hasExtSeqProd(&params) != 0 && params.nbWorkers >= 1 {
        return Error::parameter_combination_unsupported.to_error_code();
    }
    if ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1)
        <= ZSTDMT_JOBSIZE_MIN as core::ffi::c_ulonglong
    {
        params.nbWorkers = 0;
    }
    if params.nbWorkers > 0 {
        (*cctx).traceCtx = ZSTD_trace_compress_begin(cctx);
        if ((*cctx).mtctx).is_null() {
            (*cctx).mtctx = ZSTDMT_createCCtx_advanced(
                params.nbWorkers as u32,
                (*cctx).customMem,
                (*cctx).pool,
            );
            if ((*cctx).mtctx).is_null() {
                return Error::memory_allocation.to_error_code();
            }
        }
        let err_code_0 = ZSTDMT_initCStream_internal(
            (*cctx).mtctx,
            prefixDict.dict,
            prefixDict.dictSize,
            prefixDict.dictContentType,
            (*cctx).cdict,
            params,
            ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1),
        );
        if ERR_isError(err_code_0) {
            return err_code_0;
        }
        (*cctx).dictID = if !((*cctx).cdict).is_null() {
            (*(*cctx).cdict).dictID
        } else {
            0
        };
        (*cctx).dictContentSize = if !((*cctx).cdict).is_null() {
            (*(*cctx).cdict).dictContentSize
        } else {
            prefixDict.dictSize
        };
        (*cctx).consumedSrcSize = 0;
        (*cctx).producedCSize = 0;
        (*cctx).streamStage = zcss_load;
        (*cctx).appliedParams = params;
    } else {
        let pledgedSrcSize = ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1);
        let err_code_1 = ZSTD_compressBegin_internal(
            cctx,
            prefixDict.dict,
            prefixDict.dictSize,
            prefixDict.dictContentType,
            ZSTD_dtlm_fast,
            (*cctx).cdict,
            &params,
            pledgedSrcSize,
            ZSTDb_buffered,
        );
        if ERR_isError(err_code_1) {
            return err_code_1;
        }
        (*cctx).inToCompress = 0;
        (*cctx).inBuffPos = 0;
        if (*cctx).appliedParams.inBufferMode as core::ffi::c_uint
            == ZSTD_bm_buffered as core::ffi::c_int as core::ffi::c_uint
        {
            (*cctx).inBuffTarget = ((*cctx).blockSizeMax).wrapping_add(core::ffi::c_int::from(
                (*cctx).blockSizeMax as u64 == pledgedSrcSize,
            ) as size_t);
        } else {
            (*cctx).inBuffTarget = 0;
        }
        (*cctx).outBuffFlushedSize = 0;
        (*cctx).outBuffContentSize = (*cctx).outBuffFlushedSize;
        (*cctx).streamStage = zcss_load;
        (*cctx).frameEnded = 0;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressStream2))]
pub unsafe extern "C" fn ZSTD_compressStream2(
    cctx: *mut ZSTD_CCtx,
    output: *mut ZSTD_outBuffer,
    input: *mut ZSTD_inBuffer,
    endOp: ZSTD_EndDirective,
) -> size_t {
    if (*output).pos > (*output).size {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if (*input).pos > (*input).size {
        return Error::srcSize_wrong.to_error_code();
    }
    if endOp > ZSTD_e_end as core::ffi::c_int as u32 {
        return Error::parameter_outOfBound.to_error_code();
    }
    if (*cctx).streamStage as core::ffi::c_uint
        == zcss_init as core::ffi::c_int as core::ffi::c_uint
    {
        let inputSize = ((*input).size).wrapping_sub((*input).pos);
        let totalInputSize = inputSize.wrapping_add((*cctx).stableIn_notConsumed);
        if (*cctx).requestedParams.inBufferMode as core::ffi::c_uint
            == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint
            && endOp as core::ffi::c_uint
                == ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
            && totalInputSize < ZSTD_BLOCKSIZE_MAX as size_t
        {
            if (*cctx).stableIn_notConsumed != 0 {
                if (*input).src != (*cctx).expectedInBuffer.src {
                    return -(ZSTD_error_stabilityCondition_notRespected as core::ffi::c_int)
                        as size_t;
                }
                if (*input).pos != (*cctx).expectedInBuffer.size {
                    return -(ZSTD_error_stabilityCondition_notRespected as core::ffi::c_int)
                        as size_t;
                }
            }
            (*input).pos = (*input).size;
            (*cctx).expectedInBuffer = *input;
            (*cctx).stableIn_notConsumed = ((*cctx).stableIn_notConsumed).wrapping_add(inputSize);
            return (if (*cctx).requestedParams.format == Format::ZSTD_f_zstd1 {
                6
            } else {
                2
            }) as size_t;
        }
        let err_code = ZSTD_CCtx_init_compressStream2(cctx, endOp, totalInputSize);
        if ERR_isError(err_code) {
            return err_code;
        }
        ZSTD_setBufferExpectations(cctx, output, input);
    }
    let err_code_0 = ZSTD_checkBufferStability(cctx, output, input, endOp);
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    if (*cctx).appliedParams.nbWorkers > 0 {
        let mut flushMin: size_t = 0;
        if (*cctx).cParamsChanged != 0 {
            ZSTDMT_updateCParams_whileCompressing((*cctx).mtctx, &(*cctx).requestedParams);
            (*cctx).cParamsChanged = 0;
        }
        if (*cctx).stableIn_notConsumed != 0 {
            (*input).pos = ((*input).pos).wrapping_sub((*cctx).stableIn_notConsumed);
            (*cctx).stableIn_notConsumed = 0;
        }
        loop {
            let ipos = (*input).pos;
            let opos = (*output).pos;
            flushMin = ZSTDMT_compressStream_generic((*cctx).mtctx, output, input, endOp);
            (*cctx).consumedSrcSize = ((*cctx).consumedSrcSize)
                .wrapping_add(((*input).pos).wrapping_sub(ipos) as core::ffi::c_ulonglong);
            (*cctx).producedCSize = ((*cctx).producedCSize)
                .wrapping_add(((*output).pos).wrapping_sub(opos) as core::ffi::c_ulonglong);
            if ERR_isError(flushMin)
                || endOp as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
                    && flushMin == 0
            {
                if flushMin == 0 {
                    ZSTD_CCtx_trace(cctx, 0);
                }
                ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
            }
            let err_code_1 = flushMin;
            if ERR_isError(err_code_1) {
                return err_code_1;
            }
            if endOp as core::ffi::c_uint
                == ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
            {
                if (*input).pos != ipos
                    || (*output).pos != opos
                    || (*input).pos == (*input).size
                    || (*output).pos == (*output).size
                {
                    break;
                }
            } else if flushMin == 0 || (*output).pos == (*output).size {
                break;
            }
        }
        ZSTD_setBufferExpectations(cctx, output, input);
        return flushMin;
    }
    let err_code_2 = ZSTD_compressStream_generic(cctx, output, input, endOp);
    if ERR_isError(err_code_2) {
        return err_code_2;
    }
    ZSTD_setBufferExpectations(cctx, output, input);
    ((*cctx).outBuffContentSize).wrapping_sub((*cctx).outBuffFlushedSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressStream2_simpleArgs))]
pub unsafe extern "C" fn ZSTD_compressStream2_simpleArgs(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    dstPos: *mut size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    srcPos: *mut size_t,
    endOp: ZSTD_EndDirective,
) -> size_t {
    let mut output = ZSTD_outBuffer_s {
        dst: core::ptr::null_mut::<core::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    let mut input = ZSTD_inBuffer_s {
        src: core::ptr::null::<core::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    output.dst = dst;
    output.size = dstCapacity;
    output.pos = *dstPos;
    input.src = src;
    input.size = srcSize;
    input.pos = *srcPos;
    let cErr = ZSTD_compressStream2(cctx, &mut output, &mut input, endOp);
    *dstPos = output.pos;
    *srcPos = input.pos;
    cErr
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compress2))]
pub unsafe extern "C" fn ZSTD_compress2(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let originalInBufferMode = (*cctx).requestedParams.inBufferMode;
    let originalOutBufferMode = (*cctx).requestedParams.outBufferMode;
    ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
    (*cctx).requestedParams.inBufferMode = ZSTD_bm_stable;
    (*cctx).requestedParams.outBufferMode = ZSTD_bm_stable;
    let mut oPos = 0;
    let mut iPos = 0;
    let result = ZSTD_compressStream2_simpleArgs(
        cctx,
        dst,
        dstCapacity,
        &mut oPos,
        src,
        srcSize,
        &mut iPos,
        ZSTD_e_end,
    );
    (*cctx).requestedParams.inBufferMode = originalInBufferMode;
    (*cctx).requestedParams.outBufferMode = originalOutBufferMode;
    let err_code = result;
    if ERR_isError(err_code) {
        return err_code;
    }
    if result != 0 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    oPos
}
unsafe fn ZSTD_validateSequence(
    offBase: u32,
    matchLength: u32,
    minMatch: u32,
    posInSrc: size_t,
    windowLog: u32,
    dictSize: size_t,
    useSequenceProducer: core::ffi::c_int,
) -> size_t {
    let windowSize = (1) << windowLog;
    let offsetBound = if posInSrc > windowSize as size_t {
        windowSize as size_t
    } else {
        posInSrc.wrapping_add(dictSize)
    };
    let matchLenLowerBound = (if minMatch == 3 || useSequenceProducer != 0 {
        3
    } else {
        4
    }) as size_t;
    if offBase as size_t > offsetBound.wrapping_add(3) {
        return Error::externalSequences_invalid.to_error_code();
    }
    if (matchLength as size_t) < matchLenLowerBound {
        return Error::externalSequences_invalid.to_error_code();
    }
    0
}
unsafe fn ZSTD_finalizeOffBase(rawOffset: u32, rep: *const u32, ll0: u32) -> u32 {
    let mut offBase = rawOffset.wrapping_add(ZSTD_REP_NUM as u32);
    if ll0 == 0 && rawOffset == *rep {
        offBase = REPCODE1_TO_OFFBASE as u32;
    } else if rawOffset == *rep.add(1) {
        offBase = 2u32.wrapping_sub(ll0);
    } else if rawOffset == *rep.add(2) {
        offBase = 3u32.wrapping_sub(ll0);
    } else if ll0 != 0 && rawOffset == (*rep).wrapping_sub(1) {
        offBase = REPCODE3_TO_OFFBASE as u32;
    }
    offBase
}
unsafe fn ZSTD_transferSequences_wBlockDelim(
    cctx: *mut ZSTD_CCtx,
    seqPos: *mut ZSTD_SequencePosition,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    src: *const core::ffi::c_void,
    blockSize: size_t,
    externalRepSearch: ZSTD_ParamSwitch_e,
) -> size_t {
    let mut idx = (*seqPos).idx;
    let startIdx = idx;
    let mut ip = src as *const u8;
    let iend = ip.add(blockSize);
    let mut updatedRepcodes = repcodes_s { rep: [0; 3] };
    let mut dictSize: u32 = 0;
    if !((*cctx).cdict).is_null() {
        dictSize = (*(*cctx).cdict).dictContentSize as u32;
    } else if !((*cctx).prefixDict.dict).is_null() {
        dictSize = (*cctx).prefixDict.dictSize as u32;
    } else {
        dictSize = 0;
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut core::ffi::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    while (idx as size_t) < inSeqsSize
        && ((*inSeqs.offset(idx as isize)).matchLength != 0
            || (*inSeqs.offset(idx as isize)).offset != 0)
    {
        let litLength = (*inSeqs.offset(idx as isize)).litLength;
        let matchLength = (*inSeqs.offset(idx as isize)).matchLength;
        let mut offBase: u32 = 0;
        if externalRepSearch == ZSTD_ParamSwitch_e::ZSTD_ps_disable {
            offBase = ((*inSeqs.offset(idx as isize)).offset)
                .wrapping_add(ZSTD_REP_NUM as core::ffi::c_uint);
        } else {
            let ll0 = core::ffi::c_int::from(litLength == 0) as u32;
            offBase = ZSTD_finalizeOffBase(
                (*inSeqs.offset(idx as isize)).offset,
                (updatedRepcodes.rep).as_mut_ptr() as *const u32,
                ll0,
            );
            ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
        }
        if (*cctx).appliedParams.validateSequences != 0 {
            (*seqPos).posInSrc =
                ((*seqPos).posInSrc).wrapping_add(litLength.wrapping_add(matchLength) as size_t);
            let err_code = ZSTD_validateSequence(
                offBase,
                matchLength,
                (*cctx).appliedParams.cParams.minMatch,
                (*seqPos).posInSrc,
                (*cctx).appliedParams.cParams.windowLog,
                dictSize as size_t,
                ZSTD_hasExtSeqProd(&(*cctx).appliedParams),
            );
            if ERR_isError(err_code) {
                return err_code;
            }
        }
        if idx.wrapping_sub((*seqPos).idx) as size_t >= (*cctx).seqStore.maxNbSeq {
            return Error::externalSequences_invalid.to_error_code();
        }
        ZSTD_storeSeq(
            &mut (*cctx).seqStore,
            litLength as size_t,
            ip,
            iend,
            offBase,
            matchLength as size_t,
        );
        ip = ip.offset(matchLength.wrapping_add(litLength) as isize);
        idx = idx.wrapping_add(1);
    }
    if idx as size_t == inSeqsSize {
        return Error::externalSequences_invalid.to_error_code();
    }
    if externalRepSearch == ZSTD_ParamSwitch_e::ZSTD_ps_disable && idx != startIdx {
        let rep = (updatedRepcodes.rep).as_mut_ptr();
        let lastSeqIdx = idx.wrapping_sub(1);
        if lastSeqIdx >= startIdx.wrapping_add(2) {
            *rep.add(2) = (*inSeqs.offset(lastSeqIdx.wrapping_sub(2) as isize)).offset;
            *rep.add(1) = (*inSeqs.offset(lastSeqIdx.wrapping_sub(1) as isize)).offset;
            *rep = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else if lastSeqIdx == startIdx.wrapping_add(1) {
            *rep.add(2) = *rep;
            *rep.add(1) = (*inSeqs.offset(lastSeqIdx.wrapping_sub(1) as isize)).offset;
            *rep = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else {
            *rep.add(2) = *rep.add(1);
            *rep.add(1) = *rep;
            *rep = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        }
    }
    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut core::ffi::c_void,
        (updatedRepcodes.rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    if (*inSeqs.offset(idx as isize)).litLength != 0 {
        ZSTD_storeLastLiterals(
            &mut (*cctx).seqStore,
            ip,
            (*inSeqs.offset(idx as isize)).litLength as size_t,
        );
        ip = ip.offset((*inSeqs.offset(idx as isize)).litLength as isize);
        (*seqPos).posInSrc =
            ((*seqPos).posInSrc).wrapping_add((*inSeqs.offset(idx as isize)).litLength as size_t);
    }
    if ip != iend {
        return Error::externalSequences_invalid.to_error_code();
    }
    (*seqPos).idx = idx.wrapping_add(1);
    blockSize
}
unsafe fn ZSTD_transferSequences_noDelim(
    cctx: *mut ZSTD_CCtx,
    seqPos: *mut ZSTD_SequencePosition,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    src: *const core::ffi::c_void,
    blockSize: size_t,
    externalRepSearch: ZSTD_ParamSwitch_e,
) -> size_t {
    let mut idx = (*seqPos).idx;
    let mut startPosInSequence = (*seqPos).posInSequence;
    let mut endPosInSequence = ((*seqPos).posInSequence).wrapping_add(blockSize as u32);
    let mut dictSize: size_t = 0;
    let istart = src as *const u8;
    let mut ip = istart;
    let mut iend = istart.add(blockSize);
    let mut updatedRepcodes = repcodes_s { rep: [0; 3] };
    let mut bytesAdjustment = 0;
    let mut finalMatchSplit = 0;

    /* TODO(embg) support fast parsing mode in noBlockDelim mode */
    let _ = externalRepSearch;

    if !((*cctx).cdict).is_null() {
        dictSize = (*(*cctx).cdict).dictContentSize;
    } else if !((*cctx).prefixDict.dict).is_null() {
        dictSize = (*cctx).prefixDict.dictSize;
    } else {
        dictSize = 0;
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut core::ffi::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    while endPosInSequence != 0 && (idx as size_t) < inSeqsSize && finalMatchSplit == 0 {
        let currSeq = *inSeqs.offset(idx as isize);
        let mut litLength = currSeq.litLength;
        let mut matchLength = currSeq.matchLength;
        let rawOffset = currSeq.offset;
        let mut offBase: u32 = 0;
        if endPosInSequence >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            if startPosInSequence >= litLength {
                startPosInSequence = startPosInSequence.wrapping_sub(litLength);
                litLength = 0;
                matchLength = matchLength.wrapping_sub(startPosInSequence);
            } else {
                litLength = litLength.wrapping_sub(startPosInSequence);
            }
            endPosInSequence = (endPosInSequence as core::ffi::c_uint)
                .wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength));
            startPosInSequence = 0;
        } else {
            if endPosInSequence <= litLength {
                break;
            }
            let mut firstHalfMatchLength: u32 = 0;
            litLength = if startPosInSequence >= litLength {
                0
            } else {
                litLength.wrapping_sub(startPosInSequence)
            };
            firstHalfMatchLength = endPosInSequence
                .wrapping_sub(startPosInSequence)
                .wrapping_sub(litLength);
            if matchLength as size_t > blockSize
                && firstHalfMatchLength >= (*cctx).appliedParams.cParams.minMatch
            {
                let secondHalfMatchLength = (currSeq.matchLength)
                    .wrapping_add(currSeq.litLength)
                    .wrapping_sub(endPosInSequence);
                if secondHalfMatchLength < (*cctx).appliedParams.cParams.minMatch {
                    endPosInSequence = (endPosInSequence as core::ffi::c_uint).wrapping_sub(
                        ((*cctx).appliedParams.cParams.minMatch)
                            .wrapping_sub(secondHalfMatchLength),
                    );
                    bytesAdjustment = ((*cctx).appliedParams.cParams.minMatch)
                        .wrapping_sub(secondHalfMatchLength);
                    firstHalfMatchLength = firstHalfMatchLength.wrapping_sub(bytesAdjustment);
                }
                matchLength = firstHalfMatchLength;
                finalMatchSplit = 1;
            } else {
                bytesAdjustment = endPosInSequence.wrapping_sub(currSeq.litLength);
                endPosInSequence = currSeq.litLength;
                break;
            }
        }
        let ll0 = core::ffi::c_int::from(litLength == 0) as u32;
        offBase = ZSTD_finalizeOffBase(
            rawOffset,
            (updatedRepcodes.rep).as_mut_ptr() as *const u32,
            ll0,
        );
        ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
        if (*cctx).appliedParams.validateSequences != 0 {
            (*seqPos).posInSrc =
                ((*seqPos).posInSrc).wrapping_add(litLength.wrapping_add(matchLength) as size_t);
            let err_code = ZSTD_validateSequence(
                offBase,
                matchLength,
                (*cctx).appliedParams.cParams.minMatch,
                (*seqPos).posInSrc,
                (*cctx).appliedParams.cParams.windowLog,
                dictSize,
                ZSTD_hasExtSeqProd(&(*cctx).appliedParams),
            );
            if ERR_isError(err_code) {
                return err_code;
            }
        }
        if idx.wrapping_sub((*seqPos).idx) as size_t >= (*cctx).seqStore.maxNbSeq {
            return Error::externalSequences_invalid.to_error_code();
        }
        ZSTD_storeSeq(
            &mut (*cctx).seqStore,
            litLength as size_t,
            ip,
            iend,
            offBase,
            matchLength as size_t,
        );
        ip = ip.offset(matchLength.wrapping_add(litLength) as isize);
        if finalMatchSplit == 0 {
            idx = idx.wrapping_add(1);
        }
    }
    (*seqPos).idx = idx;
    (*seqPos).posInSequence = endPosInSequence;
    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut core::ffi::c_void,
        (updatedRepcodes.rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    iend = iend.offset(-(bytesAdjustment as isize));
    if ip != iend {
        let lastLLSize = iend.offset_from(ip) as core::ffi::c_long as u32;
        ZSTD_storeLastLiterals(&mut (*cctx).seqStore, ip, lastLLSize as size_t);
        (*seqPos).posInSrc = ((*seqPos).posInSrc).wrapping_add(lastLLSize as size_t);
    }
    iend.offset_from_unsigned(istart)
}
unsafe fn ZSTD_selectSequenceCopier(mode: ZSTD_SequenceFormat_e) -> ZSTD_SequenceCopier_f {
    if mode as core::ffi::c_uint
        == ZSTD_sf_explicitBlockDelimiters as core::ffi::c_int as core::ffi::c_uint
    {
        return Some(
            ZSTD_transferSequences_wBlockDelim
                as unsafe fn(
                    *mut ZSTD_CCtx,
                    *mut ZSTD_SequencePosition,
                    *const ZSTD_Sequence,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                    ZSTD_ParamSwitch_e,
                ) -> size_t,
        );
    }
    Some(
        ZSTD_transferSequences_noDelim
            as unsafe fn(
                *mut ZSTD_CCtx,
                *mut ZSTD_SequencePosition,
                *const ZSTD_Sequence,
                size_t,
                *const core::ffi::c_void,
                size_t,
                ZSTD_ParamSwitch_e,
            ) -> size_t,
    )
}
unsafe fn blockSize_explicitDelimiter(
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    seqPos: ZSTD_SequencePosition,
) -> size_t {
    let mut end = 0;
    let mut blockSize = 0 as size_t;
    let mut spos = seqPos.idx as size_t;
    while spos < inSeqsSize {
        end = core::ffi::c_int::from((*inSeqs.add(spos)).offset == 0);
        blockSize = blockSize.wrapping_add(
            ((*inSeqs.add(spos)).litLength).wrapping_add((*inSeqs.add(spos)).matchLength) as size_t,
        );
        if end != 0 {
            if (*inSeqs.add(spos)).matchLength != 0 {
                return Error::externalSequences_invalid.to_error_code();
            }
            break;
        } else {
            spos = spos.wrapping_add(1);
        }
    }
    if end == 0 {
        return Error::externalSequences_invalid.to_error_code();
    }
    blockSize
}
unsafe fn determine_blockSize(
    mode: ZSTD_SequenceFormat_e,
    blockSize: size_t,
    remaining: size_t,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    seqPos: ZSTD_SequencePosition,
) -> size_t {
    if mode as core::ffi::c_uint
        == ZSTD_sf_noBlockDelimiters as core::ffi::c_int as core::ffi::c_uint
    {
        return if remaining < blockSize {
            remaining
        } else {
            blockSize
        };
    }
    let explicitBlockSize = blockSize_explicitDelimiter(inSeqs, inSeqsSize, seqPos);
    let err_code = explicitBlockSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    if explicitBlockSize > blockSize {
        return Error::externalSequences_invalid.to_error_code();
    }
    if explicitBlockSize > remaining {
        return Error::externalSequences_invalid.to_error_code();
    }
    explicitBlockSize
}
unsafe fn ZSTD_compressSequences_internal(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut cSize = 0 as size_t;
    let mut remaining = srcSize;
    let mut seqPos = {
        ZSTD_SequencePosition {
            idx: 0,
            posInSequence: 0,
            posInSrc: 0,
        }
    };
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let sequenceCopier = ZSTD_selectSequenceCopier((*cctx).appliedParams.blockDelimiters);
    if remaining == 0 {
        let cBlockHeader24 = 1u32.wrapping_add((bt_raw as core::ffi::c_int as u32) << 1);
        if dstCapacity < 4 {
            return Error::dstSize_tooSmall.to_error_code();
        }
        MEM_writeLE32(op as *mut core::ffi::c_void, cBlockHeader24);
        op = op.add(ZSTD_blockHeaderSize);
        dstCapacity = dstCapacity.wrapping_sub(ZSTD_blockHeaderSize);
        cSize = cSize.wrapping_add(ZSTD_blockHeaderSize);
    }
    while remaining != 0 {
        let mut compressedSeqsSize: size_t = 0;
        let mut cBlockSize: size_t = 0;
        let mut blockSize = determine_blockSize(
            (*cctx).appliedParams.blockDelimiters,
            (*cctx).blockSizeMax,
            remaining,
            inSeqs,
            inSeqsSize,
            seqPos,
        );
        let lastBlock = core::ffi::c_int::from(blockSize == remaining) as u32;
        let err_code = blockSize;
        if ERR_isError(err_code) {
            return err_code;
        }
        ZSTD_resetSeqStore(&mut (*cctx).seqStore);
        blockSize = sequenceCopier.unwrap_unchecked()(
            cctx,
            &mut seqPos,
            inSeqs,
            inSeqsSize,
            ip as *const core::ffi::c_void,
            blockSize,
            (*cctx).appliedParams.searchForExternalRepcodes,
        );
        let err_code_0 = blockSize;
        if ERR_isError(err_code_0) {
            return err_code_0;
        }
        if blockSize
            < (MIN_CBLOCK_SIZE as size_t)
                .wrapping_add(ZSTD_blockHeaderSize)
                .wrapping_add(1)
                .wrapping_add(1)
        {
            cBlockSize = ZSTD_noCompressBlock(
                op as *mut core::ffi::c_void,
                dstCapacity,
                ip as *const core::ffi::c_void,
                blockSize,
                lastBlock,
            );
            let err_code_1 = cBlockSize;
            if ERR_isError(err_code_1) {
                return err_code_1;
            }
            cSize = cSize.wrapping_add(cBlockSize);
            ip = ip.add(blockSize);
            op = op.add(cBlockSize);
            remaining = remaining.wrapping_sub(blockSize);
            dstCapacity = dstCapacity.wrapping_sub(cBlockSize);
        } else {
            if dstCapacity < ZSTD_blockHeaderSize {
                return Error::dstSize_tooSmall.to_error_code();
            }
            compressedSeqsSize = ZSTD_entropyCompressSeqStore(
                &(*cctx).seqStore,
                &(*(*cctx).blockState.prevCBlock).entropy,
                &mut (*(*cctx).blockState.nextCBlock).entropy,
                &(*cctx).appliedParams,
                op.add(ZSTD_blockHeaderSize) as *mut core::ffi::c_void,
                dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
                blockSize,
                (*cctx).tmpWorkspace,
                (*cctx).tmpWkspSize,
                (*cctx).bmi2,
            );
            let err_code_2 = compressedSeqsSize;
            if ERR_isError(err_code_2) {
                return err_code_2;
            }
            if (*cctx).isFirstBlock == 0
                && ZSTD_maybeRLE(&(*cctx).seqStore) != 0
                && ZSTD_isRLE(ip, blockSize) != 0
            {
                compressedSeqsSize = 1;
            }
            if compressedSeqsSize == 0 {
                cBlockSize = ZSTD_noCompressBlock(
                    op as *mut core::ffi::c_void,
                    dstCapacity,
                    ip as *const core::ffi::c_void,
                    blockSize,
                    lastBlock,
                );
                let err_code_3 = cBlockSize;
                if ERR_isError(err_code_3) {
                    return err_code_3;
                }
            } else if compressedSeqsSize == 1 {
                cBlockSize = ZSTD_rleCompressBlock(
                    op as *mut core::ffi::c_void,
                    dstCapacity,
                    *ip,
                    blockSize,
                    lastBlock,
                );
                let err_code_4 = cBlockSize;
                if ERR_isError(err_code_4) {
                    return err_code_4;
                }
            } else {
                let mut cBlockHeader: u32 = 0;
                ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*cctx).blockState);
                if (*(*cctx).blockState.prevCBlock)
                    .entropy
                    .fse
                    .offcode_repeatMode as core::ffi::c_uint
                    == FSE_repeat_valid as core::ffi::c_int as core::ffi::c_uint
                {
                    (*(*cctx).blockState.prevCBlock)
                        .entropy
                        .fse
                        .offcode_repeatMode = FSE_repeat_check;
                }
                cBlockHeader = lastBlock
                    .wrapping_add((bt_compressed as core::ffi::c_int as u32) << 1)
                    .wrapping_add((compressedSeqsSize << 3) as u32);
                MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader);
                cBlockSize = ZSTD_blockHeaderSize.wrapping_add(compressedSeqsSize);
            }
            cSize = cSize.wrapping_add(cBlockSize);
            if lastBlock != 0 {
                break;
            }
            ip = ip.add(blockSize);
            op = op.add(cBlockSize);
            remaining = remaining.wrapping_sub(blockSize);
            dstCapacity = dstCapacity.wrapping_sub(cBlockSize);
            (*cctx).isFirstBlock = 0;
        }
    }
    cSize
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressSequences))]
pub unsafe extern "C" fn ZSTD_compressSequences(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut op = dst as *mut u8;
    let mut cSize = 0 as size_t;
    let err_code = ZSTD_CCtx_init_compressStream2(cctx, ZSTD_e_end, srcSize);
    if ERR_isError(err_code) {
        return err_code;
    }
    let frameHeaderSize = ZSTD_writeFrameHeader(
        op as *mut core::ffi::c_void,
        dstCapacity,
        &(*cctx).appliedParams,
        srcSize as u64,
        (*cctx).dictID,
    );
    op = op.add(frameHeaderSize);
    dstCapacity = dstCapacity.wrapping_sub(frameHeaderSize);
    cSize = cSize.wrapping_add(frameHeaderSize);
    if (*cctx).appliedParams.fParams.checksumFlag != 0 && srcSize != 0 {
        ZSTD_XXH64_update_slice(
            &mut (*cctx).xxhState,
            core::slice::from_raw_parts(src as *const u8, srcSize),
        );
    }
    let cBlocksSize = ZSTD_compressSequences_internal(
        cctx,
        op as *mut core::ffi::c_void,
        dstCapacity,
        inSeqs,
        inSeqsSize,
        src,
        srcSize,
    );
    let err_code_0 = cBlocksSize;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    cSize = cSize.wrapping_add(cBlocksSize);
    dstCapacity = dstCapacity.wrapping_sub(cBlocksSize);
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        let checksum = ZSTD_XXH64_digest(&mut (*cctx).xxhState) as u32;
        if dstCapacity < 4 {
            return Error::dstSize_tooSmall.to_error_code();
        }
        MEM_writeLE32(
            (dst as *mut core::ffi::c_char).add(cSize) as *mut core::ffi::c_void,
            checksum,
        );
        cSize = cSize.wrapping_add(4);
    }
    cSize
}
pub unsafe fn convertSequences_noRepcodes(
    dstSeqs: *mut SeqDef,
    inSeqs: *const ZSTD_Sequence,
    nbSequences: size_t,
) -> size_t {
    let mut longLen = 0;
    let mut n: size_t = 0;
    n = 0;
    while n < nbSequences {
        (*dstSeqs.add(n)).offBase =
            ((*inSeqs.add(n)).offset).wrapping_add(ZSTD_REP_NUM as core::ffi::c_uint);
        (*dstSeqs.add(n)).litLength = (*inSeqs.add(n)).litLength as u16;
        (*dstSeqs.add(n)).mlBase =
            ((*inSeqs.add(n)).matchLength).wrapping_sub(MINMATCH as core::ffi::c_uint) as u16;
        if (*inSeqs.add(n)).matchLength > 65535 + 3 {
            longLen = n.wrapping_add(1);
        }
        if (*inSeqs.add(n)).litLength > 65535 {
            longLen = n.wrapping_add(nbSequences).wrapping_add(1);
        }
        n = n.wrapping_add(1);
    }
    longLen
}
pub unsafe fn ZSTD_convertBlockSequences(
    cctx: *mut ZSTD_CCtx,
    inSeqs: *const ZSTD_Sequence,
    nbSequences: size_t,
    repcodeResolution: core::ffi::c_int,
) -> size_t {
    let mut updatedRepcodes = repcodes_s { rep: [0; 3] };
    let mut seqNb = 0;
    if nbSequences >= (*cctx).seqStore.maxNbSeq {
        return Error::externalSequences_invalid.to_error_code();
    }
    libc::memcpy(
        (updatedRepcodes.rep).as_mut_ptr() as *mut core::ffi::c_void,
        ((*(*cctx).blockState.prevCBlock).rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    if repcodeResolution == 0 {
        let longl = convertSequences_noRepcodes(
            (*cctx).seqStore.sequencesStart,
            inSeqs,
            nbSequences.wrapping_sub(1),
        );
        (*cctx).seqStore.sequences = ((*cctx).seqStore.sequencesStart).add(nbSequences).sub(1);
        if longl != 0 {
            if longl <= nbSequences.wrapping_sub(1) {
                (*cctx).seqStore.longLengthType = ZSTD_llt_matchLength;
                (*cctx).seqStore.longLengthPos = longl.wrapping_sub(1) as u32;
            } else {
                (*cctx).seqStore.longLengthType = ZSTD_llt_literalLength;
                (*cctx).seqStore.longLengthPos = longl
                    .wrapping_sub(nbSequences.wrapping_sub(1))
                    .wrapping_sub(1) as u32;
            }
        }
    } else {
        seqNb = 0;
        while seqNb < nbSequences.wrapping_sub(1) {
            let litLength = (*inSeqs.add(seqNb)).litLength;
            let matchLength = (*inSeqs.add(seqNb)).matchLength;
            let ll0 = core::ffi::c_int::from(litLength == 0) as u32;
            let offBase = ZSTD_finalizeOffBase(
                (*inSeqs.add(seqNb)).offset,
                (updatedRepcodes.rep).as_mut_ptr() as *const u32,
                ll0,
            );
            ZSTD_storeSeqOnly(
                &mut (*cctx).seqStore,
                litLength as size_t,
                offBase,
                matchLength as size_t,
            );
            ZSTD_updateRep((updatedRepcodes.rep).as_mut_ptr(), offBase, ll0);
            seqNb = seqNb.wrapping_add(1);
        }
    }
    if repcodeResolution == 0 && nbSequences > 1 {
        let rep = (updatedRepcodes.rep).as_mut_ptr();
        if nbSequences >= 4 {
            let lastSeqIdx = (nbSequences as u32).wrapping_sub(2);
            *rep.add(2) = (*inSeqs.offset(lastSeqIdx.wrapping_sub(2) as isize)).offset;
            *rep.add(1) = (*inSeqs.offset(lastSeqIdx.wrapping_sub(1) as isize)).offset;
            *rep = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else if nbSequences == 3 {
            *rep.add(2) = *rep;
            *rep.add(1) = (*inSeqs).offset;
            *rep = (*inSeqs.add(1)).offset;
        } else {
            *rep.add(2) = *rep.add(1);
            *rep.add(1) = *rep;
            *rep = (*inSeqs).offset;
        }
    }
    libc::memcpy(
        ((*(*cctx).blockState.nextCBlock).rep).as_mut_ptr() as *mut core::ffi::c_void,
        (updatedRepcodes.rep).as_mut_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>(),
    );
    0
}
#[inline(always)]
unsafe fn matchLengthHalfIsZero(litMatchLength: u64) -> core::ffi::c_int {
    if MEM_isLittleEndian() {
        core::ffi::c_int::from(litMatchLength <= 0xffffffff)
    } else {
        core::ffi::c_int::from(litMatchLength as u32 == 0)
    }
}
pub unsafe fn ZSTD_get1BlockSummary(seqs: *const ZSTD_Sequence, nbSeqs: size_t) -> BlockSummary {
    let mut current_block: u64;
    let mut litMatchSize0 = 0u64;
    let mut litMatchSize1 = 0u64;
    let mut litMatchSize2 = 0u64;
    let mut litMatchSize3 = 0u64;
    let mut n = 0 as size_t;
    if nbSeqs > 3 as size_t {
        loop {
            let mut litMatchLength = MEM_read64(
                &(*seqs.add(n)).litLength as *const core::ffi::c_uint as *const core::ffi::c_void,
            );
            litMatchSize0 = litMatchSize0.wrapping_add(litMatchLength);
            if matchLengthHalfIsZero(litMatchLength) != 0 {
                current_block = 13744635599856597681;
                break;
            }
            litMatchLength = MEM_read64(
                &(*seqs.add(n.wrapping_add(1))).litLength as *const core::ffi::c_uint
                    as *const core::ffi::c_void,
            );
            litMatchSize1 = litMatchSize1.wrapping_add(litMatchLength);
            if matchLengthHalfIsZero(litMatchLength) != 0 {
                n = n.wrapping_add(1);
                current_block = 13744635599856597681;
                break;
            } else {
                litMatchLength = MEM_read64(
                    &(*seqs.add(n.wrapping_add(2))).litLength as *const core::ffi::c_uint
                        as *const core::ffi::c_void,
                );
                litMatchSize2 = litMatchSize2.wrapping_add(litMatchLength);
                if matchLengthHalfIsZero(litMatchLength) != 0 {
                    n = n.wrapping_add(2);
                    current_block = 13744635599856597681;
                    break;
                } else {
                    litMatchLength = MEM_read64(
                        &(*seqs.add(n.wrapping_add(3))).litLength as *const core::ffi::c_uint
                            as *const core::ffi::c_void,
                    );
                    litMatchSize3 = litMatchSize3.wrapping_add(litMatchLength);
                    if matchLengthHalfIsZero(litMatchLength) != 0 {
                        n = n.wrapping_add(3);
                        current_block = 13744635599856597681;
                        break;
                    } else {
                        n = n.wrapping_add(4);
                        if n >= nbSeqs.wrapping_sub(3) {
                            current_block = 2668756484064249700;
                            break;
                        }
                    }
                }
            }
        }
    } else {
        current_block = 2668756484064249700;
    }
    loop {
        match current_block {
            13744635599856597681 => {
                litMatchSize0 = litMatchSize0.wrapping_add(
                    litMatchSize1
                        .wrapping_add(litMatchSize2)
                        .wrapping_add(litMatchSize3),
                );
                let mut bs_0 = BlockSummary {
                    nbSequences: 0,
                    blockSize: 0,
                    litSize: 0,
                };
                bs_0.nbSequences = n.wrapping_add(1);
                if MEM_isLittleEndian() {
                    bs_0.litSize = litMatchSize0 as u32 as size_t;
                    bs_0.blockSize =
                        (bs_0.litSize as u64).wrapping_add(litMatchSize0 >> 32) as usize;
                } else {
                    bs_0.litSize = (litMatchSize0 >> 32) as usize;
                    bs_0.blockSize = (bs_0.litSize).wrapping_add(litMatchSize0 as u32 as size_t);
                }
                return bs_0;
            }
            _ => {
                if n < nbSeqs {
                    let litMatchLength_0 = MEM_read64(
                        &(*seqs.add(n)).litLength as *const core::ffi::c_uint
                            as *const core::ffi::c_void,
                    );
                    litMatchSize0 = litMatchSize0.wrapping_add(litMatchLength_0);
                    if matchLengthHalfIsZero(litMatchLength_0) != 0 {
                        current_block = 13744635599856597681;
                        continue;
                    }
                    n = n.wrapping_add(1);
                    current_block = 2668756484064249700;
                } else {
                    let mut bs = BlockSummary {
                        nbSequences: 0,
                        blockSize: 0,
                        litSize: 0,
                    };
                    bs.nbSequences = Error::externalSequences_invalid.to_error_code();
                    return bs;
                }
            }
        }
    }
}
unsafe fn ZSTD_compressSequencesAndLiterals_internal(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut inSeqs: *const ZSTD_Sequence,
    mut nbSequences: size_t,
    mut literals: *const core::ffi::c_void,
    mut litSize: size_t,
    srcSize: size_t,
) -> size_t {
    let mut remaining = srcSize;
    let mut cSize = 0 as size_t;
    let mut op = dst as *mut u8;
    let repcodeResolution = core::ffi::c_int::from(
        (*cctx).appliedParams.searchForExternalRepcodes == ZSTD_ParamSwitch_e::ZSTD_ps_enable,
    );
    if nbSequences == 0 {
        return Error::externalSequences_invalid.to_error_code();
    }
    if nbSequences == 1 && (*inSeqs).litLength == 0 {
        let cBlockHeader24 = 1u32.wrapping_add((bt_raw as core::ffi::c_int as u32) << 1);
        if dstCapacity < 3 {
            return Error::dstSize_tooSmall.to_error_code();
        }
        MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader24);
        op = op.add(ZSTD_blockHeaderSize);
        dstCapacity = dstCapacity.wrapping_sub(ZSTD_blockHeaderSize);
        cSize = cSize.wrapping_add(ZSTD_blockHeaderSize);
    }
    while nbSequences != 0 {
        let mut compressedSeqsSize: size_t = 0;
        let mut cBlockSize: size_t = 0;
        let mut conversionStatus: size_t = 0;
        let block = ZSTD_get1BlockSummary(inSeqs, nbSequences);
        let lastBlock = core::ffi::c_int::from(block.nbSequences == nbSequences) as u32;
        let err_code = block.nbSequences;
        if ERR_isError(err_code) {
            return err_code;
        }
        if block.litSize > litSize {
            return Error::externalSequences_invalid.to_error_code();
        }
        ZSTD_resetSeqStore(&mut (*cctx).seqStore);
        conversionStatus =
            ZSTD_convertBlockSequences(cctx, inSeqs, block.nbSequences, repcodeResolution);
        let err_code_0 = conversionStatus;
        if ERR_isError(err_code_0) {
            return err_code_0;
        }
        inSeqs = inSeqs.add(block.nbSequences);
        nbSequences = nbSequences.wrapping_sub(block.nbSequences);
        remaining = remaining.wrapping_sub(block.blockSize);
        if dstCapacity < ZSTD_blockHeaderSize {
            return Error::dstSize_tooSmall.to_error_code();
        }
        compressedSeqsSize = ZSTD_entropyCompressSeqStore_internal(
            op.add(ZSTD_blockHeaderSize) as *mut core::ffi::c_void,
            dstCapacity.wrapping_sub(ZSTD_blockHeaderSize),
            literals,
            block.litSize,
            &(*cctx).seqStore,
            &(*(*cctx).blockState.prevCBlock).entropy,
            &mut (*(*cctx).blockState.nextCBlock).entropy,
            &(*cctx).appliedParams,
            (*cctx).tmpWorkspace,
            (*cctx).tmpWkspSize,
            (*cctx).bmi2,
        );
        let err_code_1 = compressedSeqsSize;
        if ERR_isError(err_code_1) {
            return err_code_1;
        }
        if compressedSeqsSize > (*cctx).blockSizeMax {
            compressedSeqsSize = 0;
        }
        litSize = litSize.wrapping_sub(block.litSize);
        literals =
            (literals as *const core::ffi::c_char).add(block.litSize) as *const core::ffi::c_void;
        if compressedSeqsSize == 0 {
            return Error::cannotProduce_uncompressedBlock.to_error_code();
        } else {
            let mut cBlockHeader: u32 = 0;
            ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*cctx).blockState);
            if (*(*cctx).blockState.prevCBlock)
                .entropy
                .fse
                .offcode_repeatMode as core::ffi::c_uint
                == FSE_repeat_valid as core::ffi::c_int as core::ffi::c_uint
            {
                (*(*cctx).blockState.prevCBlock)
                    .entropy
                    .fse
                    .offcode_repeatMode = FSE_repeat_check;
            }
            cBlockHeader = lastBlock
                .wrapping_add((bt_compressed as core::ffi::c_int as u32) << 1)
                .wrapping_add((compressedSeqsSize << 3) as u32);
            MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader);
            cBlockSize = ZSTD_blockHeaderSize.wrapping_add(compressedSeqsSize);
        }
        cSize = cSize.wrapping_add(cBlockSize);
        op = op.add(cBlockSize);
        dstCapacity = dstCapacity.wrapping_sub(cBlockSize);
        (*cctx).isFirstBlock = 0;
        if lastBlock != 0 {
            break;
        }
    }
    if litSize != 0 {
        return Error::externalSequences_invalid.to_error_code();
    }
    if remaining != 0 {
        return Error::externalSequences_invalid.to_error_code();
    }
    cSize
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressSequencesAndLiterals))]
pub unsafe extern "C" fn ZSTD_compressSequencesAndLiterals(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    literals: *const core::ffi::c_void,
    litSize: size_t,
    litCapacity: size_t,
    decompressedSize: size_t,
) -> size_t {
    let mut op = dst as *mut u8;
    let mut cSize = 0 as size_t;
    if litCapacity < litSize {
        return Error::workSpace_tooSmall.to_error_code();
    }
    let err_code = ZSTD_CCtx_init_compressStream2(cctx, ZSTD_e_end, decompressedSize);
    if ERR_isError(err_code) {
        return err_code;
    }
    if (*cctx).appliedParams.blockDelimiters as core::ffi::c_uint
        == ZSTD_sf_noBlockDelimiters as core::ffi::c_int as core::ffi::c_uint
    {
        return Error::frameParameter_unsupported.to_error_code();
    }
    if (*cctx).appliedParams.validateSequences != 0 {
        return Error::parameter_unsupported.to_error_code();
    }
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        return Error::frameParameter_unsupported.to_error_code();
    }
    let frameHeaderSize = ZSTD_writeFrameHeader(
        op as *mut core::ffi::c_void,
        dstCapacity,
        &(*cctx).appliedParams,
        decompressedSize as u64,
        (*cctx).dictID,
    );
    op = op.add(frameHeaderSize);
    dstCapacity = dstCapacity.wrapping_sub(frameHeaderSize);
    cSize = cSize.wrapping_add(frameHeaderSize);
    let cBlocksSize = ZSTD_compressSequencesAndLiterals_internal(
        cctx,
        op as *mut core::ffi::c_void,
        dstCapacity,
        inSeqs,
        inSeqsSize,
        literals,
        litSize,
        decompressedSize,
    );
    let err_code_0 = cBlocksSize;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    cSize = cSize.wrapping_add(cBlocksSize);
    dstCapacity = dstCapacity.wrapping_sub(cBlocksSize);
    cSize
}
unsafe fn inBuffer_forEndFlush(zcs: *const ZSTD_CStream) -> ZSTD_inBuffer {
    let nullInput = {
        ZSTD_inBuffer_s {
            src: core::ptr::null(),
            size: 0,
            pos: 0,
        }
    };
    let stableInput = core::ffi::c_int::from(
        (*zcs).appliedParams.inBufferMode as core::ffi::c_uint
            == ZSTD_bm_stable as core::ffi::c_int as core::ffi::c_uint,
    );
    if stableInput != 0 {
        (*zcs).expectedInBuffer
    } else {
        nullInput
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_flushStream))]
pub unsafe extern "C" fn ZSTD_flushStream(
    zcs: *mut ZSTD_CStream,
    output: *mut ZSTD_outBuffer,
) -> size_t {
    let mut input = inBuffer_forEndFlush(zcs);
    input.size = input.pos;
    ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_flush)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_endStream))]
pub unsafe extern "C" fn ZSTD_endStream(
    zcs: *mut ZSTD_CStream,
    output: *mut ZSTD_outBuffer,
) -> size_t {
    let mut input = inBuffer_forEndFlush(zcs);
    let remainingToFlush = ZSTD_compressStream2(zcs, output, &mut input, ZSTD_e_end);
    let err_code = remainingToFlush;
    if ERR_isError(err_code) {
        return err_code;
    }
    if (*zcs).appliedParams.nbWorkers > 0 {
        return remainingToFlush;
    }
    let lastBlockSize = (if (*zcs).frameEnded != 0 {
        0
    } else {
        ZSTD_BLOCKHEADERSIZE
    }) as size_t;
    let checksumSize = (if (*zcs).frameEnded != 0 {
        0
    } else {
        (*zcs).appliedParams.fParams.checksumFlag * 4
    }) as size_t;

    remainingToFlush
        .wrapping_add(lastBlockSize)
        .wrapping_add(checksumSize)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_maxCLevel))]
pub const extern "C" fn ZSTD_maxCLevel() -> core::ffi::c_int {
    ZSTD_MAX_CLEVEL
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_minCLevel))]
pub const extern "C" fn ZSTD_minCLevel() -> core::ffi::c_int {
    -ZSTD_TARGETLENGTH_MAX
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_defaultCLevel))]
pub const extern "C" fn ZSTD_defaultCLevel() -> core::ffi::c_int {
    ZSTD_CLEVEL_DEFAULT
}

unsafe fn ZSTD_dedicatedDictSearch_isSupported(
    cParams: *const ZSTD_compressionParameters,
) -> core::ffi::c_int {
    core::ffi::c_int::from(
        (*cParams).strategy as core::ffi::c_uint
            >= ZSTD_greedy as core::ffi::c_int as core::ffi::c_uint
            && (*cParams).strategy as core::ffi::c_uint
                <= ZSTD_lazy2 as core::ffi::c_int as core::ffi::c_uint
            && (*cParams).hashLog > (*cParams).chainLog
            && (*cParams).chainLog <= 24,
    )
}
unsafe fn ZSTD_dedicatedDictSearch_revertCParams(cParams: *mut ZSTD_compressionParameters) {
    if let 3..=5 = (*cParams).strategy as core::ffi::c_uint {
        (*cParams).hashLog =
            ((*cParams).hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as core::ffi::c_uint);
        if (*cParams).hashLog < ZSTD_HASHLOG_MIN as core::ffi::c_uint {
            (*cParams).hashLog = ZSTD_HASHLOG_MIN as core::ffi::c_uint;
        }
    };
}
unsafe fn ZSTD_getCParamRowSize(
    srcSizeHint: u64,
    mut dictSize: size_t,
    mode: ZSTD_CParamMode_e,
) -> u64 {
    if mode as core::ffi::c_uint == 1 {
        dictSize = 0;
    }
    let unknown =
        core::ffi::c_int::from(srcSizeHint as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN);
    let addedSize = if unknown != 0 && dictSize > 0 { 500 } else { 0 };
    if unknown != 0 && dictSize == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        srcSizeHint
            .wrapping_add(dictSize as u64)
            .wrapping_add(addedSize)
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getCParams))]
pub unsafe extern "C" fn ZSTD_getCParams(
    compressionLevel: core::ffi::c_int,
    mut srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
) -> ZSTD_compressionParameters {
    if srcSizeHint == 0 {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    ZSTD_getCParams_internal(compressionLevel, srcSizeHint, dictSize, ZSTD_cpm_unknown)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getParams))]
pub unsafe extern "C" fn ZSTD_getParams(
    compressionLevel: core::ffi::c_int,
    mut srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
) -> ZSTD_parameters {
    if srcSizeHint == 0 {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    ZSTD_getParams_internal(compressionLevel, srcSizeHint, dictSize, ZSTD_cpm_unknown)
}
pub const __INT_MAX__: core::ffi::c_int = 2147483647;
pub const ZSTD_MAX_CLEVEL: core::ffi::c_int = 22;
static ZSTD_defaultCParameters: [[ZSTD_compressionParameters; 23]; 4] = [
    [
        {
            ZSTD_compressionParameters {
                windowLog: 19,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 6,
                targetLength: 1,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 19,
                chainLog: 13,
                hashLog: 14,
                searchLog: 1,
                minMatch: 7,
                targetLength: 0,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 20,
                chainLog: 15,
                hashLog: 16,
                searchLog: 1,
                minMatch: 6,
                targetLength: 0,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 16,
                hashLog: 17,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 18,
                hashLog: 18,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 18,
                hashLog: 19,
                searchLog: 3,
                minMatch: 5,
                targetLength: 2,
                strategy: ZSTD_greedy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 18,
                hashLog: 19,
                searchLog: 3,
                minMatch: 5,
                targetLength: 4,
                strategy: ZSTD_lazy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 19,
                hashLog: 20,
                searchLog: 4,
                minMatch: 5,
                targetLength: 8,
                strategy: ZSTD_lazy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 21,
                chainLog: 19,
                hashLog: 20,
                searchLog: 4,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 20,
                hashLog: 21,
                searchLog: 4,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 21,
                hashLog: 22,
                searchLog: 5,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 21,
                hashLog: 22,
                searchLog: 6,
                minMatch: 5,
                targetLength: 16,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 23,
                searchLog: 6,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 22,
                searchLog: 4,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 23,
                searchLog: 5,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 23,
                hashLog: 23,
                searchLog: 6,
                minMatch: 5,
                targetLength: 32,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 22,
                chainLog: 22,
                hashLog: 22,
                searchLog: 5,
                minMatch: 5,
                targetLength: 48,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 23,
                chainLog: 23,
                hashLog: 22,
                searchLog: 5,
                minMatch: 4,
                targetLength: 64,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 23,
                chainLog: 23,
                hashLog: 22,
                searchLog: 6,
                minMatch: 3,
                targetLength: 64,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 23,
                chainLog: 24,
                hashLog: 22,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 25,
                chainLog: 25,
                hashLog: 23,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 26,
                chainLog: 26,
                hashLog: 24,
                searchLog: 7,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 27,
                chainLog: 27,
                hashLog: 25,
                searchLog: 9,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            }
        },
    ],
    [
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 5,
                targetLength: 1,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 13,
                hashLog: 14,
                searchLog: 1,
                minMatch: 6,
                targetLength: 0,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 14,
                hashLog: 14,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 16,
                hashLog: 16,
                searchLog: 1,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_dfast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 5,
                targetLength: 2,
                strategy: ZSTD_greedy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 17,
                hashLog: 18,
                searchLog: 5,
                minMatch: 5,
                targetLength: 2,
                strategy: ZSTD_greedy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 3,
                minMatch: 5,
                targetLength: 4,
                strategy: ZSTD_lazy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 4,
                targetLength: 4,
                strategy: ZSTD_lazy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 6,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 5,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 7,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 4,
                targetLength: 16,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 4,
                minMatch: 3,
                targetLength: 32,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 18,
                hashLog: 19,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 10,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 12,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 18,
                chainLog: 19,
                hashLog: 19,
                searchLog: 13,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            }
        },
    ],
    [
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 12,
                hashLog: 12,
                searchLog: 1,
                minMatch: 5,
                targetLength: 1,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 6,
                targetLength: 0,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 13,
                hashLog: 15,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 15,
                hashLog: 16,
                searchLog: 2,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_dfast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 17,
                hashLog: 17,
                searchLog: 2,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_dfast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 2,
                strategy: ZSTD_greedy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 4,
                strategy: ZSTD_lazy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 4,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 16,
                hashLog: 17,
                searchLog: 6,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 17,
                hashLog: 17,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 7,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 3,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 4,
                minMatch: 3,
                targetLength: 32,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 6,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 10,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 5,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 7,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 9,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 17,
                chainLog: 18,
                hashLog: 17,
                searchLog: 11,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            }
        },
    ],
    [
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 12,
                hashLog: 13,
                searchLog: 1,
                minMatch: 5,
                targetLength: 1,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 15,
                searchLog: 1,
                minMatch: 5,
                targetLength: 0,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 15,
                searchLog: 1,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_fast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 15,
                searchLog: 2,
                minMatch: 4,
                targetLength: 0,
                strategy: ZSTD_dfast,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 4,
                minMatch: 4,
                targetLength: 2,
                strategy: ZSTD_greedy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 3,
                minMatch: 4,
                targetLength: 4,
                strategy: ZSTD_lazy,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 4,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 6,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 14,
                hashLog: 14,
                searchLog: 8,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_lazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 5,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 9,
                minMatch: 4,
                targetLength: 8,
                strategy: ZSTD_btlazy2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 3,
                minMatch: 4,
                targetLength: 12,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 4,
                minMatch: 3,
                targetLength: 24,
                strategy: ZSTD_btopt,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 14,
                searchLog: 5,
                minMatch: 3,
                targetLength: 32,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 6,
                minMatch: 3,
                targetLength: 64,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 5,
                minMatch: 3,
                targetLength: 48,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 6,
                minMatch: 3,
                targetLength: 128,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 7,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 8,
                minMatch: 3,
                targetLength: 256,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 8,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 9,
                minMatch: 3,
                targetLength: 512,
                strategy: ZSTD_btultra2,
            }
        },
        {
            ZSTD_compressionParameters {
                windowLog: 14,
                chainLog: 15,
                hashLog: 15,
                searchLog: 10,
                minMatch: 3,
                targetLength: 999,
                strategy: ZSTD_btultra2,
            }
        },
    ],
];

unsafe fn ZSTD_dedicatedDictSearch_getCParams(
    compressionLevel: core::ffi::c_int,
    dictSize: size_t,
) -> ZSTD_compressionParameters {
    let mut cParams = ZSTD_getCParams_internal(compressionLevel, 0, dictSize, ZSTD_cpm_createCDict);
    if let 3..=5 = cParams.strategy as core::ffi::c_uint {
        cParams.hashLog =
            (cParams.hashLog).wrapping_add(ZSTD_LAZY_DDSS_BUCKET_LOG as core::ffi::c_uint);
    }
    cParams
}
unsafe fn ZSTD_getCParams_internal(
    compressionLevel: core::ffi::c_int,
    srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
    mode: ZSTD_CParamMode_e,
) -> ZSTD_compressionParameters {
    let rSize = ZSTD_getCParamRowSize(srcSizeHint, dictSize, mode);
    let tableID = (core::ffi::c_int::from(rSize <= (256 * ((1) << 10)) as u64)
        + core::ffi::c_int::from(rSize <= (128 * ((1) << 10)) as u64)
        + core::ffi::c_int::from(rSize <= (16 * ((1) << 10)) as u64)) as u32;
    let mut row: core::ffi::c_int = 0;
    if compressionLevel == 0 {
        row = ZSTD_CLEVEL_DEFAULT;
    } else if compressionLevel < 0 {
        row = 0;
    } else if compressionLevel > ZSTD_MAX_CLEVEL {
        row = ZSTD_MAX_CLEVEL;
    } else {
        row = compressionLevel;
    }
    let mut cp = *(*ZSTD_defaultCParameters.as_ptr().offset(tableID as isize))
        .as_ptr()
        .offset(row as isize);
    if compressionLevel < 0 {
        let clampedCompressionLevel = if ZSTD_minCLevel() > compressionLevel {
            ZSTD_minCLevel()
        } else {
            compressionLevel
        };
        cp.targetLength = -clampedCompressionLevel as core::ffi::c_uint;
    }
    ZSTD_adjustCParams_internal(
        cp,
        srcSizeHint,
        dictSize,
        mode,
        ZSTD_ParamSwitch_e::ZSTD_ps_auto,
    )
}
unsafe fn ZSTD_getParams_internal(
    compressionLevel: core::ffi::c_int,
    srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
    mode: ZSTD_CParamMode_e,
) -> ZSTD_parameters {
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
    let cParams = ZSTD_getCParams_internal(compressionLevel, srcSizeHint, dictSize, mode);
    ptr::write_bytes(
        &mut params as *mut ZSTD_parameters as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_parameters>(),
    );
    params.cParams = cParams;
    params.fParams.contentSizeFlag = 1;
    params
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_registerSequenceProducer))]
pub unsafe extern "C" fn ZSTD_registerSequenceProducer(
    zc: *mut ZSTD_CCtx,
    extSeqProdState: *mut core::ffi::c_void,
    extSeqProdFunc: ZSTD_sequenceProducer_F,
) {
    ZSTD_CCtxParams_registerSequenceProducer(
        &mut (*zc).requestedParams,
        extSeqProdState,
        extSeqProdFunc,
    );
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtxParams_registerSequenceProducer))]
pub unsafe extern "C" fn ZSTD_CCtxParams_registerSequenceProducer(
    params: *mut ZSTD_CCtx_params,
    extSeqProdState: *mut core::ffi::c_void,
    extSeqProdFunc: ZSTD_sequenceProducer_F,
) {
    if extSeqProdFunc.is_some() {
        (*params).extSeqProdFunc = extSeqProdFunc;
        (*params).extSeqProdState = extSeqProdState;
    } else {
        (*params).extSeqProdFunc = None;
        (*params).extSeqProdState = core::ptr::null_mut();
    };
}
