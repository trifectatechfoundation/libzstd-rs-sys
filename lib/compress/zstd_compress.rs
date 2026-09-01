use core::ptr;

use crate::lib::polyfill::PointerExt;

pub type ZSTD_CCtx = ZSTD_CCtx_s;

#[repr(C)]
pub struct ZSTD_CCtx_s {
    pub(super) stage: CompressionStage,
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
    pub(super) bufferedPolicy: BufferedPolicy,
    pub(super) inBuff: *mut u8,
    pub(super) inBuffSize: size_t,
    pub(super) inToCompress: size_t,
    pub(super) inBuffPos: size_t,
    pub(super) inBuffTarget: size_t,
    pub(super) outBuff: *mut u8,
    pub(super) outBuffSize: size_t,
    pub(super) outBuffContentSize: size_t,
    pub(super) outBuffFlushedSize: size_t,
    pub(super) streamStage: StreamStage,
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
pub struct ZSTD_entropyCTablesMetadata_t {
    pub hufMetadata: ZSTD_hufCTablesMetadata_t,
    pub fseMetadata: ZSTD_fseCTablesMetadata_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTablesMetadata_t {
    pub llType: SymbolEncodingType,
    pub ofType: SymbolEncodingType,
    pub mlType: SymbolEncodingType,
    pub fseTablesBuffer: [u8; 133],
    pub fseTablesSize: size_t,
    pub lastCountSize: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTablesMetadata_t {
    pub hType: SymbolEncodingType,
    pub hufDesBuffer: [u8; ZSTD_MAX_HUF_HEADER_SIZE],
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
    pub longLengthType: LongLengthType,
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

pub type ZSTD_CDict = ZSTD_CDict_s;

#[repr(C)]
pub struct ZSTD_CDict_s {
    pub dictContent: *const core::ffi::c_void,
    pub dictContentSize: size_t,
    /// The dictContentType the CDict was created with
    pub dictContentType: ZSTD_dictContentType_e,
    /// entropy workspace of HUF_WORKSPACE_SIZE bytes
    pub entropyWorkspace: *mut u32,
    pub workspace: ZSTD_cwksp,
    pub matchState: ZSTD_MatchState_t,
    pub cBlockState: ZSTD_compressedBlockState_t,
    pub customMem: ZSTD_customMem,
    pub dictID: u32,
    /// 0 indicates that advanced API was used to select CDict params
    pub compressionLevel: core::ffi::c_int,
    /// Indicates whether the CDict was created with params that would use
    /// row-based matchfinder. Unless the CDict is reloaded, we will use
    /// the same greedy/lazy matchfinder at compression time.
    pub useRowMatchFinder: ParamSwitch,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressedBlockState_t {
    pub entropy: ZSTD_entropyCTables_t,
    pub rep: [u32; 3],
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

impl RawSeqStore_t {
    pub const fn new() -> Self {
        Self {
            seq: core::ptr::null_mut(),
            pos: 0,
            posInSequence: 0,
            size: 0,
            capacity: 0,
        }
    }
}

impl Default for RawSeqStore_t {
    fn default() -> Self {
        Self::new()
    }
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
pub struct ZSTD_optimal_t {
    pub price: core::ffi::c_int,
    pub off: u32,
    pub mlen: u32,
    pub litlen: u32,
    pub rep: [u32; 3],
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const u8,
    pub base: *const u8,
    pub dictBase: *const u8,
    pub dictLimit: u32,
    pub lowLimit: u32,
    pub nbOverflowCorrections: u32,
}

#[derive(Copy, Clone, Default)]
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
    pub phase: CwkspAllocPhase,
    pub isStatic: CwkspAllocKind,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum CwkspAllocKind {
    #[default]
    Dynamic = 0,
    Static = 1,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum CwkspAllocPhase {
    #[default]
    Objects = 0,
    AlignedInitOnce = 1,
    Aligned = 2,
    Buffers = 3,
}

pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;

#[derive(Copy, Clone, Default)]
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
    pub postBlockSplitter: ParamSwitch,
    pub preBlockSplitter_level: core::ffi::c_int,
    pub maxBlockSize: size_t,
    pub useRowMatchFinder: ParamSwitch,
    pub deterministicRefPrefix: core::ffi::c_int,
    pub customMem: ZSTD_customMem,
    pub prefetchCDictTables: ParamSwitch,
    pub enableMatchFinderFallback: core::ffi::c_int,
    pub extSeqProdState: *mut core::ffi::c_void,
    pub extSeqProdFunc: ZSTD_sequenceProducer_F,
    pub searchForExternalRepcodes: ParamSwitch,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct ZSTD_symbolEncodingTypeStats_t {
    pub LLtype: SymbolEncodingType,
    pub Offtype: SymbolEncodingType,
    pub MLtype: SymbolEncodingType,
    pub size: size_t,
    pub lastCountSize: size_t,
    pub longOffsets: bool,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum DefaultPolicy {
    #[default]
    Disallowed = 0,
    Allowed = 1,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum BuildSeqStore {
    #[default]
    Compress = 0,
    NoCompress = 1,
}

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

pub type ZSTD_compResetPolicy_e = core::ffi::c_uint;
pub const ZSTDcrp_leaveDirty: ZSTD_compResetPolicy_e = 1;
pub const ZSTDcrp_makeClean: ZSTD_compResetPolicy_e = 0;
pub type ZSTD_resetTarget_e = core::ffi::c_uint;
pub const ZSTD_resetTarget_CCtx: ZSTD_resetTarget_e = 1;
pub const ZSTD_resetTarget_CDict: ZSTD_resetTarget_e = 0;
pub type ZSTD_indexResetPolicy_e = core::ffi::c_uint;
pub const ZSTDirp_reset: ZSTD_indexResetPolicy_e = 1;
pub const ZSTDirp_continue: ZSTD_indexResetPolicy_e = 0;
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

pub type ZSTD_CStream = ZSTD_CCtx;

pub type ZSTD_SequenceCopier_f = Option<
    unsafe fn(
        *mut ZSTD_CCtx,
        *mut ZSTD_SequencePosition,
        *const ZSTD_Sequence,
        size_t,
        *const core::ffi::c_void,
        size_t,
        ParamSwitch,
    ) -> size_t,
>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSummary {
    pub nbSequences: size_t,
    pub blockSize: size_t,
    pub litSize: size_t,
}

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

pub const ZSTD_OPT_SIZE: core::ffi::c_int = ZSTD_OPT_NUM + 3;
pub const ZSTD_MAX_NB_BLOCK_SPLITS: usize = 196;

#[inline]
fn ZSTD_LLcode(litLength: u32) -> u32 {
    static LL_Code: [u8; 64] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20,
        20, 20, 20, 21, 21, 21, 21, 22, 22, 22, 22, 22, 22, 22, 22, 23, 23, 23, 23, 23, 23, 23, 23,
        24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    ];
    static LL_deltaCode: u32 = 19;
    if litLength > 63 {
        (ZSTD_highbit32(litLength)).wrapping_add(LL_deltaCode)
    } else {
        LL_Code[litLength as usize] as core::ffi::c_uint
    }
}

/// Note: mlBase = matchLength - MINMATCH;
/// because it's the format it's stored in seqStore->sequences
#[inline]
fn ZSTD_MLcode(mlBase: u32) -> u32 {
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
        ML_Code[mlBase as usize] as core::ffi::c_uint
    }
}

/// # Returns
///
/// `true` if value is within cParam bounds
#[inline]
fn ZSTD_cParam_withinBounds(cParam: ZSTD_cParameter, value: core::ffi::c_int) -> bool {
    let bounds = ZSTD_cParam_getBounds(cParam);
    if ERR_isError(bounds.error) {
        return false;
    }
    if value < bounds.lowerBound {
        return false;
    }
    if value > bounds.upperBound {
        return false;
    }
    true
}

/// Writes uncompressed block to dst buffer from given src.
/// Returns the size of the block
#[inline]
unsafe fn ZSTD_noCompressBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastBlock: u32,
) -> size_t {
    let cBlockHeader24 = lastBlock
        .wrapping_add((BlockType::Raw as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }
    MEM_writeLE24(dst, cBlockHeader24);
    core::ptr::copy_nonoverlapping(
        src.cast::<u8>(),
        dst.byte_add(ZSTD_blockHeaderSize).cast::<u8>(),
        srcSize,
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
        .wrapping_add((BlockType::Rle as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    if dstCapacity < 4 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader);
    *op.add(3) = src;
    4
}

/// Minimum compression required to generate a compress block or a compressed
/// literals section. note: use same formula for both situations
#[inline]
fn ZSTD_minGain(srcSize: size_t, strat: ZSTD_strategy) -> size_t {
    let minlog = if strat >= ZSTD_btultra {
        strat.wrapping_sub(1)
    } else {
        6
    };
    (srcSize >> minlog).wrapping_add(2)
}

#[inline]
unsafe fn ZSTD_literalsCompressionIsDisabled(cctxParams: &ZSTD_CCtx_params) -> bool {
    match cctxParams.literalCompressionMode {
        ParamSwitch::Enable => false,
        ParamSwitch::Disable => true,
        ParamSwitch::Auto => {
            cctxParams.cParams.strategy == ZSTD_fast && cctxParams.cParams.targetLength > 0
        }
    }
}

pub const REPCODE1_TO_OFFBASE: core::ffi::c_int = 1;
pub const REPCODE3_TO_OFFBASE: core::ffi::c_int = 3;

/// Clears the window containing the history by simply setting it to empty.
#[inline]
unsafe fn ZSTD_window_clear(window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).wrapping_offset_from((*window).base) as size_t;
    let end = endT as u32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}

/// Reduces the indices to protect from index overflow.
/// Returns the correction made to the indices, which must be applied to every stored index.
///
/// The least significant cycleLog bits of the indices must remain the same, which may be 0.
/// Every index up to maxDist in the past must be valid.
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
        cycleSize.max(2)
    } else {
        0
    };
    let newCurrent = currentCycle
        .wrapping_add(currentCycleCorrection)
        .wrapping_add(maxDist.max(cycleSize));
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

/// Similar to ZSTD_window_enforceMaxDist(), but only invalidates dictionary when input
/// progresses beyond window size.
/// assumption: loadedDictEndPtr and dictMatchStatePtr are valid (non NULL),
/// loadedDictEnd uses same referential as window->base,
/// maxDist is the window size
#[inline]
fn ZSTD_checkDictValidity(
    window: &ZSTD_window_t,
    blockEnd: *const core::ffi::c_void,
    maxDist: u32,
    loadedDictEndPtr: &mut u32,
    dictMatchStatePtr: &mut *const ZSTD_MatchState_t,
) {
    let blockEndIdx =
        (blockEnd as *const u8).wrapping_offset_from(window.base) as core::ffi::c_long as u32;
    let loadedDictEnd = *loadedDictEndPtr;
    if blockEndIdx > loadedDictEnd.wrapping_add(maxDist) || loadedDictEnd != window.dictLimit {
        *loadedDictEndPtr = 0;
        *dictMatchStatePtr = core::ptr::null();
    } else {
        // FIXME: add log
        // *loadedDictEndPtr != 0;
    }
}

#[inline]
fn ZSTD_window_init(window: &mut ZSTD_window_t) {
    window.base = c" ".as_ptr() as *const u8;
    window.dictBase = c" ".as_ptr() as *const u8;
    window.dictLimit = ZSTD_WINDOW_START_INDEX as u32; // start from >0, so that 1st position is valid
    window.lowLimit = ZSTD_WINDOW_START_INDEX as u32; // it ensures first and later CCtx usages compress the same
    window.nextSrc = (window.base).wrapping_offset(ZSTD_WINDOW_START_INDEX as isize);
    window.nbOverflowCorrections = 0;
}

pub const ZSTD_SHORT_CACHE_TAG_BITS: core::ffi::c_int = 8;

/// Returns `true` if an external sequence producer is registered.
#[inline]
unsafe fn ZSTD_hasExtSeqProd(params: *const ZSTD_CCtx_params) -> bool {
    ((*params).extSeqProdFunc).is_some()
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
    HUF_flags_optimalDepth, HUF_repeat_check, HUF_repeat_none, HUF_repeat_valid,
    HUF_OPTIMAL_DEPTH_THRESHOLD, HUF_SYMBOLVALUE_MAX, HUF_SYMBOLVALUE_MAX_U8, HUF_WORKSPACE_SIZE,
};
use crate::lib::common::mem::{
    MEM_32bits, MEM_64bits, MEM_read64, MEM_readLE32, MEM_readST, MEM_writeLE16, MEM_writeLE24,
    MEM_writeLE32, MEM_writeLE64,
};
use crate::lib::common::pool::ZSTD_threadPool;
use crate::lib::common::xxhash::{
    XXH64_state_t, ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_update_slice,
};
use crate::lib::common::zstd_internal::{
    repStartValue, BlockType, DefaultMaxOff, LLFSELog, LL_bits, LL_defaultNorm, LL_defaultNormLog,
    LitHufLog, Litbits, MLFSELog, ML_bits, ML_defaultNorm, ML_defaultNormLog, MaxLL, MaxML, MaxOff,
    MaxSeq, OF_defaultNorm, OF_defaultNormLog, OffFSELog, SymbolEncodingType, ZSTD_cpuSupportsBmi2,
    ZSTD_limitCopy, MINMATCH, WILDCOPY_OVERLENGTH, ZSTD_MAX_HUF_HEADER_SIZE, ZSTD_OPT_NUM,
    ZSTD_REP_NUM, ZSTD_WORKSPACETOOLARGE_FACTOR, ZSTD_WORKSPACETOOLARGE_MAXDURATION,
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
    optState_t, repcodes_s, BufferedPolicy, CParamMode, CompressionStage, DictMode,
    DictTableLoadMethod, LongLengthType, OptPrice, Repcodes_t, SeqCollector, StreamStage,
    TableFillPurpose, ZSTD_BlockCompressor_f, ZSTD_blockSplitCtx, ZSTD_blockState_t, ZSTD_count,
    ZSTD_entropyCTables_t, ZSTD_fseCTables_t, ZSTD_getSequenceLength, ZSTD_hufCTables_t,
    ZSTD_localDict, ZSTD_matchState_dictMode, ZSTD_match_t, ZSTD_prefixDict, ZSTD_prefixDict_s,
    ZSTD_storeSeq, ZSTD_storeSeqOnly, ZSTD_updateRep, ZSTD_window_enforceMaxDist,
    ZSTD_window_needOverflowCorrection, ZSTD_window_update,
    ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY, ZSTD_WINDOW_START_INDEX,
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
use crate::lib::compress::zstd_preSplit::{ZSTD_splitBlock, ZSTD_SLIPBLOCK_WORKSPACESIZE};
use crate::lib::compress::zstdmt_compress::{
    ZSTDMT_CCtx, ZSTDMT_compressStream_generic, ZSTDMT_createCCtx_advanced, ZSTDMT_freeCCtx,
    ZSTDMT_getFrameProgression, ZSTDMT_initCStream_internal, ZSTDMT_nextInputSizeHint,
    ZSTDMT_sizeof_CCtx, ZSTDMT_toFlushNow, ZSTDMT_updateCParams_whileCompressing,
};
use crate::lib::zstd::{
    Format, ParamSwitch, ZSTD_EndDirective, ZSTD_ResetDirective, ZSTD_Sequence,
    ZSTD_SequenceFormat_e, ZSTD_bm_buffered, ZSTD_bm_stable, ZSTD_btlazy2, ZSTD_btopt,
    ZSTD_btultra, ZSTD_btultra2, ZSTD_bufferMode_e, ZSTD_cParameter, ZSTD_compressionParameters,
    ZSTD_customMem, ZSTD_dct_auto, ZSTD_dct_fullDict, ZSTD_dct_rawContent, ZSTD_dfast,
    ZSTD_dictAttachPref_e, ZSTD_dictContentType_e, ZSTD_dictLoadMethod_e, ZSTD_dlm_byCopy,
    ZSTD_dlm_byRef, ZSTD_e_continue, ZSTD_e_end, ZSTD_e_flush,
    ZSTD_error_stabilityCondition_notRespected, ZSTD_fast, ZSTD_frameParameters,
    ZSTD_frameProgression, ZSTD_greedy, ZSTD_inBuffer, ZSTD_inBuffer_s, ZSTD_lazy, ZSTD_lazy2,
    ZSTD_outBuffer, ZSTD_outBuffer_s, ZSTD_parameters, ZSTD_sequenceProducer_F,
    ZSTD_sf_explicitBlockDelimiters, ZSTD_sf_noBlockDelimiters, ZSTD_strategy, ZSTD_BLOCKSIZE_MAX,
    ZSTD_BLOCKSIZE_MAX_MIN, ZSTD_CLEVEL_DEFAULT, ZSTD_CONTENTSIZE_UNKNOWN, ZSTD_MAGICNUMBER,
    ZSTD_MAGIC_DICTIONARY, ZSTD_MAGIC_SKIPPABLE_START, ZSTD_SKIPPABLEHEADERSIZE,
    ZSTD_VERSION_NUMBER, ZSTD_WINDOWLOG_ABSOLUTEMIN, ZSTD_WINDOWLOG_MAX, ZSTD_WINDOWLOG_MAX_32,
    ZSTD_WINDOWLOG_MAX_64,
};

pub const ZSTD_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub const MIN_CBLOCK_SIZE: core::ffi::c_int = 1 + 1;
pub const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
pub const ZSTD_CWKSP_ALIGNMENT_BYTES: core::ffi::c_int = 64;

#[inline]
fn ZSTD_cwksp_assert_internal_consistency(ws: &mut ZSTD_cwksp) {
    assert!(ws.workspace <= ws.objectEnd);
    assert!(ws.objectEnd <= ws.tableEnd);
    assert!(ws.objectEnd <= ws.tableValidEnd);
    assert!(ws.tableEnd <= ws.allocStart);
    assert!(ws.tableValidEnd <= ws.allocStart);
    assert!(ws.allocStart <= ws.workspaceEnd);
    assert!(ws.initOnceStart <= ZSTD_cwksp_initialAllocStart(ws));
    assert!(ws.workspace <= ws.initOnceStart);
}

/// Align must be a power of 2.
#[inline]
fn ZSTD_cwksp_align(size: size_t, align: size_t) -> size_t {
    let mask = align.wrapping_sub(1);
    size.wrapping_add(mask) & !mask
}

/// Use this to determine how much space in the workspace we will consume to allocate this object.
/// (Normally it should be exactly the size of the object, but under special conditions, like ASAN,
/// where we pad each object, it might be larger.)
///
/// Since tables aren't currently redzoned, you don't need to call through this to figure out how
/// much space you need for the matchState tables. Everything else is though.
///
/// Do not use for sizing aligned buffers. Instead, use ZSTD_cwksp_aligned64_alloc_size().
#[inline]
fn ZSTD_cwksp_alloc_size(size: size_t) -> size_t {
    if size == 0 {
        return 0;
    }
    size
}

#[inline]
fn ZSTD_cwksp_aligned_alloc_size(size: size_t, alignment: size_t) -> size_t {
    ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(size, alignment))
}

/// Returns an adjusted alloc size that is the nearest larger multiple of 64 bytes.
/// Used to determine the number of bytes required for a given "aligned".
#[inline]
fn ZSTD_cwksp_aligned64_alloc_size(size: size_t) -> size_t {
    ZSTD_cwksp_aligned_alloc_size(size, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t)
}

/// Returns the amount of additional space the cwksp must allocate for internal purposes
/// (currently only alignment).
#[inline]
fn ZSTD_cwksp_slack_space_required() -> size_t {
    (ZSTD_CWKSP_ALIGNMENT_BYTES * 2) as size_t
}

/// Return the number of additional bytes required to align a pointer to the given number of bytes.
/// alignBytes must be a power of two.
#[inline]
fn ZSTD_cwksp_bytes_to_align_ptr(ptr: *mut core::ffi::c_void, alignBytes: size_t) -> size_t {
    let alignBytesMask = alignBytes.wrapping_sub(1);

    alignBytes.wrapping_sub(ptr as size_t & alignBytesMask) & alignBytesMask
}

/// Returns the initial value for allocStart which is used to determine the position from which
/// we can allocate from the end of the workspace.
#[inline]
fn ZSTD_cwksp_initialAllocStart(ws: &mut ZSTD_cwksp) -> *mut core::ffi::c_void {
    let mut endPtr = ws.workspaceEnd as *mut core::ffi::c_char;
    endPtr = endPtr.wrapping_sub(endPtr as size_t % ZSTD_CWKSP_ALIGNMENT_BYTES as size_t);
    endPtr as *mut core::ffi::c_void
}

/// Internal function. Do not use directly.
/// Reserves the given number of bytes within the aligned/buffer segment of the wksp,
/// which counts from the end of the wksp (as opposed to the object/table segment).
///
/// # Returns
///
/// A pointer to the beginning of that space.
#[inline]
fn ZSTD_cwksp_reserve_internal_buffer_space(
    ws: &mut ZSTD_cwksp,
    bytes: size_t,
) -> *mut core::ffi::c_void {
    let alloc = ws.allocStart.wrapping_byte_sub(bytes);
    let bottom = ws.tableEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
    if alloc < bottom {
        ws.allocFailed = 1;
        return core::ptr::null_mut();
    }
    if alloc < ws.tableValidEnd {
        ws.tableValidEnd = alloc;
    }
    ws.allocStart = alloc;
    alloc
}

/// Moves the cwksp to the next phase, and does any necessary allocations.
/// cwksp initialization must necessarily go through each phase in order.
///
/// # Returns
///
/// 0 on success, or zstd error
#[inline]
fn ZSTD_cwksp_internal_advance_phase(ws: &mut ZSTD_cwksp, phase: CwkspAllocPhase) -> size_t {
    if phase > ws.phase {
        if ws.phase < CwkspAllocPhase::AlignedInitOnce && phase >= CwkspAllocPhase::AlignedInitOnce
        {
            ws.tableValidEnd = ws.objectEnd;
            ws.initOnceStart = ZSTD_cwksp_initialAllocStart(ws);
            let alloc = ws.objectEnd;
            let bytesToAlign =
                ZSTD_cwksp_bytes_to_align_ptr(alloc, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t);
            let objectEnd = alloc.wrapping_byte_add(bytesToAlign);
            if objectEnd > ws.workspaceEnd {
                return Error::memory_allocation.to_error_code();
            }
            ws.objectEnd = objectEnd;
            ws.tableEnd = objectEnd;
            if ws.tableValidEnd < ws.tableEnd {
                ws.tableValidEnd = ws.tableEnd;
            }
        }
        ws.phase = phase;
        ZSTD_cwksp_assert_internal_consistency(ws);
    }
    0
}

/// Returns whether this object/buffer/etc was allocated in this workspace.
#[inline]
fn ZSTD_cwksp_owns_buffer(ws: &ZSTD_cwksp, ptr: *const core::ffi::c_void) -> bool {
    !ptr.is_null()
        && ws.workspace <= ptr as *mut core::ffi::c_void
        && ptr < ws.workspaceEnd as *const core::ffi::c_void
}

/// Internal function. Do not use directly.
#[inline]
fn ZSTD_cwksp_reserve_internal(
    ws: &mut ZSTD_cwksp,
    bytes: size_t,
    phase: CwkspAllocPhase,
) -> *mut core::ffi::c_void {
    let mut alloc = core::ptr::null_mut::<core::ffi::c_void>();
    if ERR_isError(ZSTD_cwksp_internal_advance_phase(ws, phase)) || bytes == 0 {
        return core::ptr::null_mut();
    }
    alloc = ZSTD_cwksp_reserve_internal_buffer_space(ws, bytes);
    alloc
}

/// Reserves and returns unaligned memory.
#[inline]
fn ZSTD_cwksp_reserve_buffer(ws: &mut ZSTD_cwksp, bytes: size_t) -> *mut u8 {
    ZSTD_cwksp_reserve_internal(ws, bytes, CwkspAllocPhase::Buffers) as *mut u8
}

/// Reserves and returns memory sized on and aligned on ZSTD_CWKSP_ALIGNMENT_BYTES (64 bytes).
/// This memory has been initialized at least once in the past. This doesn't mean it has been
/// initialized this time, and it might contain data from previous operations.
/// The main usage is for algorithms that might need read access into uninitialized memory.
/// The algorithm must maintain safety under these conditions and must make sure it doesn't leak
/// any of the past data (directly or in side channels).
#[inline]
unsafe fn ZSTD_cwksp_reserve_aligned_init_once(
    ws: &mut ZSTD_cwksp,
    bytes: size_t,
) -> *mut core::ffi::c_void {
    let alignedBytes = ZSTD_cwksp_align(bytes, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t);
    let ptr = ZSTD_cwksp_reserve_internal(ws, alignedBytes, CwkspAllocPhase::AlignedInitOnce);
    if !ptr.is_null() && ptr < ws.initOnceStart {
        ptr::write_bytes(
            ptr as *mut u8,
            0,
            (if ((ws.initOnceStart as *mut u8).offset_from(ptr as *mut u8) as core::ffi::c_long
                as size_t)
                < alignedBytes
            {
                (ws.initOnceStart as *mut u8).offset_from(ptr as *mut u8) as core::ffi::c_long
                    as size_t
            } else {
                alignedBytes
            }) as libc::size_t,
        );
        ws.initOnceStart = ptr;
    }
    ptr
}

/// Reserves and returns memory sized on and aligned on ZSTD_CWKSP_ALIGNMENT_BYTES (64 bytes).
#[inline]
unsafe fn ZSTD_cwksp_reserve_aligned64(
    ws: &mut ZSTD_cwksp,
    bytes: size_t,
) -> *mut core::ffi::c_void {
    ZSTD_cwksp_reserve_internal(
        ws,
        ZSTD_cwksp_align(bytes, ZSTD_CWKSP_ALIGNMENT_BYTES as size_t),
        CwkspAllocPhase::Aligned,
    )
}

/// Aligned on 64 bytes. These buffers have the special property that their values remain
/// constrained, allowing us to reuse them without memset()-ing them.
#[inline]
fn ZSTD_cwksp_reserve_table(ws: &mut ZSTD_cwksp, bytes: size_t) -> *mut core::ffi::c_void {
    let phase = CwkspAllocPhase::AlignedInitOnce;
    let mut alloc = core::ptr::null_mut::<core::ffi::c_void>();
    let mut end = core::ptr::null_mut::<core::ffi::c_void>();
    let mut top = core::ptr::null_mut::<core::ffi::c_void>();
    if ws.phase < phase && ERR_isError(ZSTD_cwksp_internal_advance_phase(ws, phase)) {
        return core::ptr::null_mut();
    }
    alloc = ws.tableEnd;
    end = alloc.wrapping_byte_add(bytes);
    top = ws.allocStart;
    ZSTD_cwksp_assert_internal_consistency(ws);
    if end > top {
        ws.allocFailed = 1;
        return core::ptr::null_mut();
    }
    ws.tableEnd = end;
    alloc
}

/// Aligned on sizeof(void*).
/// Note: should happen only once, at workspace first initialization
#[inline]
fn ZSTD_cwksp_reserve_object(ws: &mut ZSTD_cwksp, bytes: size_t) -> *mut core::ffi::c_void {
    let roundedBytes = ZSTD_cwksp_align(bytes, size_of::<*mut core::ffi::c_void>());
    let alloc = ws.objectEnd;
    let end = alloc.wrapping_byte_add(roundedBytes);
    ZSTD_cwksp_assert_internal_consistency(ws);
    if ws.phase != CwkspAllocPhase::Objects || end > ws.workspaceEnd {
        ws.allocFailed = 1;
        return core::ptr::null_mut();
    }
    ws.objectEnd = end;
    ws.tableEnd = end;
    ws.tableValidEnd = end;
    alloc
}

#[inline]
fn ZSTD_cwksp_mark_tables_dirty(ws: &mut ZSTD_cwksp) {
    ws.tableValidEnd = ws.objectEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
}

#[inline]
fn ZSTD_cwksp_mark_tables_clean(ws: &mut ZSTD_cwksp) {
    if ws.tableValidEnd < ws.tableEnd {
        ws.tableValidEnd = ws.tableEnd;
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
}

/// Zero the part of the allocated tables not already marked clean.
#[inline]
unsafe fn ZSTD_cwksp_clean_tables(ws: &mut ZSTD_cwksp) {
    if ws.tableValidEnd < ws.tableEnd {
        ptr::write_bytes(
            ws.tableValidEnd,
            0,
            (ws.tableEnd as *mut u8).offset_from(ws.tableValidEnd as *mut u8) as usize,
        );
    }
    ZSTD_cwksp_mark_tables_clean(ws);
}

/// Invalidates table allocations. All other allocations remain valid.
#[inline]
fn ZSTD_cwksp_clear_tables(ws: &mut ZSTD_cwksp) {
    ws.tableEnd = ws.objectEnd;
    ZSTD_cwksp_assert_internal_consistency(ws);
}

/// Invalidates all buffer, aligned, and table allocations.
/// Object allocations remain valid.
#[inline]
fn ZSTD_cwksp_clear(ws: &mut ZSTD_cwksp) {
    ws.tableEnd = ws.objectEnd;
    ws.allocStart = ZSTD_cwksp_initialAllocStart(ws);
    ws.allocFailed = 0;
    if ws.phase > CwkspAllocPhase::AlignedInitOnce {
        ws.phase = CwkspAllocPhase::AlignedInitOnce;
    }
    ZSTD_cwksp_assert_internal_consistency(ws);
}

#[inline]
fn ZSTD_cwksp_sizeof(ws: &ZSTD_cwksp) -> size_t {
    (ws.workspaceEnd as *mut u8).wrapping_offset_from(ws.workspace as *mut u8) as size_t
}

/// The provided workspace takes ownership of the buffer [start, start+size).
/// Any existing values in the workspace are ignored (the previously managed buffer,
/// if present, must be separately freed).
#[inline]
fn ZSTD_cwksp_init(
    ws: &mut ZSTD_cwksp,
    start: *mut core::ffi::c_void,
    size: size_t,
    isStatic: CwkspAllocKind,
) {
    ws.workspace = start;
    ws.workspaceEnd = start.wrapping_byte_add(size);
    ws.objectEnd = ws.workspace;
    ws.tableValidEnd = ws.objectEnd;
    ws.initOnceStart = ZSTD_cwksp_initialAllocStart(ws);
    ws.phase = CwkspAllocPhase::Objects;
    ws.isStatic = isStatic;
    ZSTD_cwksp_clear(ws);
    ws.workspaceOversizedDuration = 0;
    ZSTD_cwksp_assert_internal_consistency(ws);
}

#[inline]
unsafe fn ZSTD_cwksp_create(
    ws: &mut ZSTD_cwksp,
    size: size_t,
    customMem: ZSTD_customMem,
) -> size_t {
    let workspace = ZSTD_customMalloc(size, customMem);
    if workspace.is_null() {
        return Error::memory_allocation.to_error_code();
    }
    ZSTD_cwksp_init(ws, workspace, size, CwkspAllocKind::Dynamic);
    0
}

#[inline]
unsafe fn ZSTD_cwksp_free(ws: *mut ZSTD_cwksp, customMem: ZSTD_customMem) {
    let ptr = (*ws).workspace;
    let size = (*ws)
        .workspaceEnd
        .byte_offset_from_unsigned((*ws).workspace);
    ptr::write_bytes(ws as *mut u8, 0, size_of::<ZSTD_cwksp>());
    ZSTD_customFree(ptr, size, customMem);
}

/// Moves the management of a workspace from one cwksp to another. The src cwksp is left in an
/// invalid state (src must be re-init()'ed before it's used again).
#[inline]
fn ZSTD_cwksp_move(dst: &mut ZSTD_cwksp, src: &mut ZSTD_cwksp) {
    *dst = core::mem::take(src);
}

#[inline]
fn ZSTD_cwksp_reserve_failed(ws: &ZSTD_cwksp) -> bool {
    ws.allocFailed != 0
}

#[inline]
fn ZSTD_cwksp_available_space(ws: &mut ZSTD_cwksp) -> size_t {
    (ws.allocStart as *mut u8).wrapping_offset_from(ws.tableEnd as *mut u8) as size_t
}

#[inline]
fn ZSTD_cwksp_check_available(ws: &mut ZSTD_cwksp, additionalNeededSpace: size_t) -> bool {
    ZSTD_cwksp_available_space(ws) >= additionalNeededSpace
}

#[inline]
fn ZSTD_cwksp_check_too_large(ws: &mut ZSTD_cwksp, additionalNeededSpace: size_t) -> bool {
    ZSTD_cwksp_check_available(
        ws,
        additionalNeededSpace * ZSTD_WORKSPACETOOLARGE_FACTOR as size_t,
    )
}

#[inline]
fn ZSTD_cwksp_check_wasteful(ws: &mut ZSTD_cwksp, additionalNeededSpace: size_t) -> bool {
    ZSTD_cwksp_check_too_large(ws, additionalNeededSpace)
        && ws.workspaceOversizedDuration > ZSTD_WORKSPACETOOLARGE_MAXDURATION
}

#[inline]
fn ZSTD_cwksp_bump_oversized_duration(ws: &mut ZSTD_cwksp, additionalNeededSpace: size_t) {
    if ZSTD_cwksp_check_too_large(ws, additionalNeededSpace) {
        ws.workspaceOversizedDuration += 1;
    } else {
        ws.workspaceOversizedDuration = 0;
    }
}

pub const ZSTDMT_JOBSIZE_MIN: core::ffi::c_int = 512 * (1 << 10);

pub const STREAM_ACCUMULATOR_MIN_32: core::ffi::c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: core::ffi::c_int = 57;
pub const ZSTD_LAZY_DDSS_BUCKET_LOG: core::ffi::c_int = 2;
pub const ZSTD_ROW_HASH_TAG_BITS: core::ffi::c_int = 8;
pub const ZSTD_LDM_DEFAULT_WINDOW_LOG: core::ffi::c_int = 27;

/// Maximum size of the hash table dedicated to find 3-bytes matches,
/// in log format, aka 17 => 1 << 17 == 128Ki positions.
/// This structure is only used in zstd_opt.
/// Since allocation is centralized for all strategies, it has to be known here.
/// The actual (selected) size of the hash table is then stored in ZSTD_MatchState_t.hashLog3,
/// so that zstd_opt.c doesn't need to know about this constant.
const ZSTD_HASHLOG3_MAX: u32 = 17;

pub const INT_MAX: core::ffi::c_int = __INT_MAX__;

// ------- Helper functions -------

/// Note that the result from this function is only valid for
/// the one-pass compression functions.
/// When employing the streaming mode,
/// if flushes are frequently altering the size of blocks,
/// the overhead from block headers can make the compressed data larger
/// than the return value of ZSTD_compressBound().
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBound))]
pub extern "C" fn ZSTD_compressBound(srcSize: size_t) -> size_t {
    let r = if srcSize as core::ffi::c_ulonglong
        >= (if size_of::<size_t>() == 8 {
            0xff00ff00ff00ff00 as core::ffi::c_ulonglong
        } else {
            0xff00ff00 as core::ffi::c_uint as core::ffi::c_ulonglong
        }) {
        0
    } else {
        srcSize
            .wrapping_add(srcSize >> 8)
            .wrapping_add(if srcSize < (128 << 10) as size_t {
                ((128 << 10) as size_t).wrapping_sub(srcSize) >> 11
            } else {
                0
            })
    };
    if r == 0 {
        return Error::srcSize_wrong.to_error_code();
    }
    r
}

// ------- Context memory management -------

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCCtx))]
pub unsafe extern "C" fn ZSTD_createCCtx() -> *mut ZSTD_CCtx {
    ZSTD_createCCtx_advanced(ZSTD_customMem::default())
}

unsafe fn ZSTD_initCCtx(cctx: *mut ZSTD_CCtx, memManager: ZSTD_customMem) {
    ptr::write_bytes(cctx as *mut u8, 0, size_of::<ZSTD_CCtx>());
    (*cctx).customMem = memManager;
    (*cctx).bmi2 = ZSTD_cpuSupportsBmi2() as _;
    let _err = ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_parameters);
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createCCtx_advanced))]
pub unsafe extern "C" fn ZSTD_createCCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_CCtx {
    let cctx = ZSTD_customMalloc(size_of::<ZSTD_CCtx>(), customMem) as *mut ZSTD_CCtx;
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
    let mut ws = ZSTD_cwksp::default();
    let mut cctx = core::ptr::null_mut::<ZSTD_CCtx>();
    if workspaceSize <= size_of::<ZSTD_CCtx>() {
        // minimum size
        return core::ptr::null_mut();
    }
    if workspace as size_t & 7 != 0 {
        // must be 8-aligned
        return core::ptr::null_mut();
    }
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, CwkspAllocKind::Static);

    cctx = ZSTD_cwksp_reserve_object(&mut ws, size_of::<ZSTD_CCtx>()) as *mut ZSTD_CCtx;
    if cctx.is_null() {
        return core::ptr::null_mut();
    }

    ptr::write_bytes(cctx as *mut u8, 0, size_of::<ZSTD_CCtx>());
    ZSTD_cwksp_move(&mut (*cctx).workspace, &mut ws);
    (*cctx).staticSize = workspaceSize;

    // statically sized space. tmpWorkspace never moves (but prev/next block swap places)
    if !ZSTD_cwksp_check_available(
        &mut (*cctx).workspace,
        (((8 << 10) + 512) as size_t)
            .wrapping_add(size_of::<core::ffi::c_uint>().wrapping_mul((MaxSeq + 2) as size_t))
            .max(ZSTD_SLIPBLOCK_WORKSPACESIZE)
            .wrapping_add((2 as size_t).wrapping_mul(size_of::<ZSTD_compressedBlockState_t>())),
    ) {
        return core::ptr::null_mut();
    }
    (*cctx).blockState.prevCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        size_of::<ZSTD_compressedBlockState_t>(),
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx).blockState.nextCBlock = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        size_of::<ZSTD_compressedBlockState_t>(),
    ) as *mut ZSTD_compressedBlockState_t;
    (*cctx).tmpWorkspace = ZSTD_cwksp_reserve_object(
        &mut (*cctx).workspace,
        (((8 << 10) + 512) as size_t)
            .wrapping_add(size_of::<core::ffi::c_uint>().wrapping_mul(MaxSeq + 2))
            .max(ZSTD_SLIPBLOCK_WORKSPACESIZE),
    );
    (*cctx).tmpWkspSize = (((8 << 10) + 512) as size_t)
        .wrapping_add(size_of::<core::ffi::c_uint>().wrapping_mul(MaxSeq + 2))
        .max(ZSTD_SLIPBLOCK_WORKSPACESIZE);
    (*cctx).bmi2 = ZSTD_cpuSupportsBmi2() as _;
    cctx
}

/// Clears and frees all of the dictionaries in the CCtx.
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
        size_of::<ZSTD_localDict>(),
    );
    ptr::write_bytes(
        &mut (*cctx).prefixDict as *mut ZSTD_prefixDict as *mut u8,
        0,
        size_of::<ZSTD_prefixDict>(),
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
    if !cctxInWorkspace {
        ZSTD_customFree(
            cctx as *mut core::ffi::c_void,
            size_of::<ZSTD_CCtx>(),
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

    // cctx may be in the workspace
    (if (*cctx).workspace.workspace == cctx as *mut core::ffi::c_void {
        0
    } else {
        size_of::<ZSTD_CCtx>()
    })
    .wrapping_add(ZSTD_cwksp_sizeof(&(*cctx).workspace))
    .wrapping_add(ZSTD_sizeof_localDict((*cctx).localDict))
    .wrapping_add(ZSTD_sizeof_mtctx(cctx))
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_CStream))]
pub unsafe extern "C" fn ZSTD_sizeof_CStream(zcs: *const ZSTD_CStream) -> size_t {
    ZSTD_sizeof_CCtx(zcs)
}

/// private API call, for dictBuilder only
pub unsafe fn ZSTD_getSeqStore(ctx: *const ZSTD_CCtx) -> *const SeqStore_t {
    &(*ctx).seqStore
}

/// Returns true if the strategy supports using a row based matchfinder
fn ZSTD_rowMatchFinderSupported(strategy: ZSTD_strategy) -> bool {
    (ZSTD_greedy..=ZSTD_lazy2).contains(&strategy)
}

/// Returns true if the strategy and useRowMatchFinder mode indicate that we will use the row based
/// matchfinder for this compression.
fn ZSTD_rowMatchFinderUsed(strategy: ZSTD_strategy, mode: ParamSwitch) -> bool {
    ZSTD_rowMatchFinderSupported(strategy) && mode == ParamSwitch::Enable
}

/// Returns row matchfinder usage given an initial mode and cParams
fn ZSTD_resolveRowMatchFinderMode(
    mut mode: ParamSwitch,
    cParams: &ZSTD_compressionParameters,
) -> ParamSwitch {
    let kWindowLogLowerBound = 14;
    if mode != ParamSwitch::Auto {
        // if requested enabled, but no SIMD, we still will use row matchfinder
        return mode;
    }
    mode = ParamSwitch::Disable;
    if !ZSTD_rowMatchFinderSupported(cParams.strategy) {
        return mode;
    }
    if cParams.windowLog > kWindowLogLowerBound {
        mode = ParamSwitch::Enable;
    }
    mode
}

/// Returns block splitter usage (generally speaking, when using slower/stronger compression modes)
fn ZSTD_resolveBlockSplitterMode(
    mode: ParamSwitch,
    cParams: &ZSTD_compressionParameters,
) -> ParamSwitch {
    if mode != ParamSwitch::Auto {
        return mode;
    }

    if cParams.strategy >= ZSTD_btopt && cParams.windowLog >= 17 {
        ParamSwitch::Enable
    } else {
        ParamSwitch::Disable
    }
}

/// Returns 1 if the arguments indicate that we should allocate a chainTable, 0 otherwise.
///
/// We always should allocate a chaintable if we are allocating a matchstate for a DDS dictionary
/// matchstate. We do not allocate a chaintable if we are using ZSTD_fast, or are using the
/// row-based matchfinder.
fn ZSTD_allocateChainTable(
    strategy: ZSTD_strategy,
    useRowMatchFinder: ParamSwitch,
    forDDSDict: bool,
) -> bool {
    forDDSDict || strategy != ZSTD_fast && !ZSTD_rowMatchFinderUsed(strategy, useRowMatchFinder)
}

/// Returns Enable if compression parameters are such that we should
/// enable long distance matching (wlog >= 27, strategy >= btopt).
/// Returns Disable otherwise.
fn ZSTD_resolveEnableLdm(mode: ParamSwitch, cParams: &ZSTD_compressionParameters) -> ParamSwitch {
    if mode != ParamSwitch::Auto {
        return mode;
    }

    if cParams.strategy >= ZSTD_btopt && cParams.windowLog >= 27 {
        ParamSwitch::Enable
    } else {
        ParamSwitch::Disable
    }
}

fn ZSTD_resolveExternalSequenceValidation(mode: core::ffi::c_int) -> core::ffi::c_int {
    mode
}

/// Resolves maxBlockSize to the default if no value is present.
fn ZSTD_resolveMaxBlockSize(maxBlockSize: size_t) -> size_t {
    if maxBlockSize == 0 {
        ZSTD_BLOCKSIZE_MAX as size_t
    } else {
        maxBlockSize
    }
}

fn ZSTD_resolveExternalRepcodeSearch(value: ParamSwitch, cLevel: core::ffi::c_int) -> ParamSwitch {
    if value != ParamSwitch::Auto {
        return value;
    }

    if cLevel < 10 {
        ParamSwitch::Disable
    } else {
        ParamSwitch::Enable
    }
}

/// Returns 1 if compression parameters are such that CDict hashtable and chaintable indices are
/// tagged. If so, the tags need to be removed in ZSTD_resetCCtx_byCopyingCDict.
unsafe fn ZSTD_CDictIndicesAreTagged(cParams: *const ZSTD_compressionParameters) -> bool {
    (*cParams).strategy == ZSTD_fast || (*cParams).strategy == ZSTD_dfast
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
            enableLdm: ParamSwitch::Auto,
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
        postBlockSplitter: ParamSwitch::Auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ParamSwitch::Auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ParamSwitch::Auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ParamSwitch::Auto,
    };

    // should not matter, as all cParams are presumed properly defined
    ZSTD_CCtxParams_init(&mut cctxParams, ZSTD_CLEVEL_DEFAULT);
    cctxParams.cParams = cParams;

    // Adjust advanced params according to cParams
    cctxParams.ldmParams.enableLdm =
        ZSTD_resolveEnableLdm(cctxParams.ldmParams.enableLdm, &cParams);
    if cctxParams.ldmParams.enableLdm == ParamSwitch::Enable {
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
    let params =
        ZSTD_customCalloc(size_of::<ZSTD_CCtx_params>(), customMem) as *mut ZSTD_CCtx_params;
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
        size_of::<ZSTD_CCtx_params>(),
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
    ptr::write_bytes(cctxParams as *mut u8, 0, size_of::<ZSTD_CCtx_params>());
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams).fParams.contentSizeFlag = 1;

    0
}

pub const ZSTD_NO_CLEVEL: core::ffi::c_int = 0;

/// Initializes `cctxParams` from `params` and `compressionLevel`.
/// If params are derived from a compression level then that compression
/// level, otherwise ZSTD_NO_CLEVEL.
unsafe fn ZSTD_CCtxParams_init_internal(
    cctxParams: *mut ZSTD_CCtx_params,
    params: &ZSTD_parameters,
    compressionLevel: core::ffi::c_int,
) {
    ptr::write_bytes(cctxParams as *mut u8, 0, size_of::<ZSTD_CCtx_params>());
    (*cctxParams).cParams = params.cParams;
    (*cctxParams).fParams = params.fParams;
    // Should not matter, as all cParams are presumed properly defined.
    // But, set it for tracing anyway.
    (*cctxParams).compressionLevel = compressionLevel;
    (*cctxParams).useRowMatchFinder =
        ZSTD_resolveRowMatchFinderMode((*cctxParams).useRowMatchFinder, &params.cParams);
    (*cctxParams).postBlockSplitter =
        ZSTD_resolveBlockSplitterMode((*cctxParams).postBlockSplitter, &params.cParams);
    (*cctxParams).ldmParams.enableLdm =
        ZSTD_resolveEnableLdm((*cctxParams).ldmParams.enableLdm, &params.cParams);
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

/// Sets cctxParams' cParams and fParams from validated zstd params, but otherwise leaves them alone.
fn ZSTD_CCtxParams_setZstdParams(cctxParams: &mut ZSTD_CCtx_params, params: &ZSTD_parameters) {
    cctxParams.cParams = params.cParams;
    cctxParams.fParams = params.fParams;
    // Should not matter, as all cParams are presumed properly defined.
    // But, set it for tracing anyway.
    cctxParams.compressionLevel = ZSTD_NO_CLEVEL;
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_cParam_getBounds))]
pub extern "C" fn ZSTD_cParam_getBounds(param: ZSTD_cParameter) -> ZSTD_bounds {
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
            bounds.upperBound = if size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            };
            bounds
        }
        102 => {
            bounds.lowerBound = ZSTD_HASHLOG_MIN;
            bounds.upperBound = if (if size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) < 30
            {
                if size_of::<size_t>() == 4 {
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
            bounds.upperBound = if size_of::<size_t>() == 4 {
                ZSTD_CHAINLOG_MAX_32
            } else {
                ZSTD_CHAINLOG_MAX_64
            };
            bounds
        }
        104 => {
            bounds.lowerBound = ZSTD_SEARCHLOG_MIN;
            bounds.upperBound = (if size_of::<size_t>() == 4 {
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
            bounds.upperBound = if size_of::<*mut core::ffi::c_void>() == 4 {
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
            bounds.lowerBound = ParamSwitch::Auto as core::ffi::c_int;
            bounds.upperBound = ParamSwitch::Disable as core::ffi::c_int;
            bounds
        }
        161 => {
            bounds.lowerBound = ZSTD_LDM_HASHLOG_MIN;
            bounds.upperBound = if (if size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) < 30
            {
                if size_of::<size_t>() == 4 {
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
            bounds.upperBound = (if size_of::<size_t>() == 4 {
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
            bounds.lowerBound = ParamSwitch::Auto as core::ffi::c_int;
            bounds.upperBound = ParamSwitch::Disable as core::ffi::c_int;
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
            bounds.lowerBound = ParamSwitch::Auto as core::ffi::c_int;
            bounds.upperBound = ParamSwitch::Disable as core::ffi::c_int;
            bounds
        }
        1017 => {
            bounds.lowerBound = 0;
            bounds.upperBound = ZSTD_BLOCKSPLITTER_LEVEL_MAX;
            bounds
        }
        1011 => {
            bounds.lowerBound = ParamSwitch::Auto as core::ffi::c_int;
            bounds.upperBound = ParamSwitch::Disable as core::ffi::c_int;
            bounds
        }
        1012 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            bounds
        }
        1013 => {
            bounds.lowerBound = ParamSwitch::Auto as core::ffi::c_int;
            bounds.upperBound = ParamSwitch::Disable as core::ffi::c_int;
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
            bounds.lowerBound = ParamSwitch::Auto as core::ffi::c_int;
            bounds.upperBound = ParamSwitch::Disable as core::ffi::c_int;
            bounds
        }
        _ => {
            bounds.error = Error::parameter_unsupported.to_error_code();
            bounds
        }
    }
}

/// Clamps the value into the bounded range.
fn ZSTD_cParam_clampBounds(cParam: ZSTD_cParameter, value: &mut core::ffi::c_int) -> size_t {
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

fn ZSTD_isUpdateAuthorized(param: ZSTD_cParameter) -> bool {
    match param {
        ZSTD_cParameter::ZSTD_c_compressionLevel
        | ZSTD_cParameter::ZSTD_c_hashLog
        | ZSTD_cParameter::ZSTD_c_chainLog
        | ZSTD_cParameter::ZSTD_c_searchLog
        | ZSTD_cParameter::ZSTD_c_minMatch
        | ZSTD_cParameter::ZSTD_c_targetLength
        | ZSTD_cParameter::ZSTD_c_strategy => true,

        _ if param == ZSTD_cParameter::ZSTD_c_blockSplitterLevel => true,

        _ => false,
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setParameter))]
pub unsafe extern "C" fn ZSTD_CCtx_setParameter(
    cctx: *mut ZSTD_CCtx,
    param: ZSTD_cParameter,
    value: core::ffi::c_int,
) -> size_t {
    if (*cctx).streamStage != StreamStage::Init {
        if ZSTD_isUpdateAuthorized(param) {
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
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_windowLog, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.windowLog = value as u32;
            (*CCtxParams).cParams.windowLog as size_t
        }
        102 => {
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_hashLog, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.hashLog = value as u32;
            (*CCtxParams).cParams.hashLog as size_t
        }
        103 => {
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_chainLog, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.chainLog = value as u32;
            (*CCtxParams).cParams.chainLog as size_t
        }
        104 => {
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_searchLog, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.searchLog = value as u32;
            value as size_t
        }
        105 => {
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_minMatch, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.minMatch = value as u32;
            (*CCtxParams).cParams.minMatch as size_t
        }
        106 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_targetLength, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.targetLength = value as u32;
            (*CCtxParams).cParams.targetLength as size_t
        }
        107 => {
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_strategy, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).cParams.strategy = value as ZSTD_strategy;
            (*CCtxParams).cParams.strategy as size_t
        }
        200 => {
            (*CCtxParams).fParams.contentSizeFlag = (value != 0) as core::ffi::c_int;
            (*CCtxParams).fParams.contentSizeFlag as size_t
        }
        201 => {
            (*CCtxParams).fParams.checksumFlag = (value != 0) as core::ffi::c_int;
            (*CCtxParams).fParams.checksumFlag as size_t
        }
        202 => {
            (*CCtxParams).fParams.noDictIDFlag = (value == 0) as core::ffi::c_int;
            ((*CCtxParams).fParams.noDictIDFlag == 0) as core::ffi::c_int as size_t
        }
        1000 => {
            (*CCtxParams).forceWindow = (value != 0) as core::ffi::c_int;
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
            (*CCtxParams).enableDedicatedDictSearch = (value != 0) as core::ffi::c_int;
            (*CCtxParams).enableDedicatedDictSearch as size_t
        }
        160 => {
            let Ok(value) = ParamSwitch::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).ldmParams.enableLdm = value;
            (*CCtxParams).ldmParams.enableLdm as size_t
        }
        161 => {
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmHashLog, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.hashLog = value as u32;
            (*CCtxParams).ldmParams.hashLog as size_t
        }
        162 => {
            if value != 0 && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmMinMatch, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.minMatchLength = value as u32;
            (*CCtxParams).ldmParams.minMatchLength as size_t
        }
        163 => {
            if value != 0
                && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog, value)
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.bucketSizeLog = value as u32;
            (*CCtxParams).ldmParams.bucketSizeLog as size_t
        }
        164 => {
            if value != 0
                && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_ldmHashRateLog, value)
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).ldmParams.hashRateLog = value as u32;
            (*CCtxParams).ldmParams.hashRateLog as size_t
        }
        130 => {
            if value != 0 {
                value = value.max(ZSTD_TARGETCBLOCKSIZE_MIN);
                if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_targetCBlockSize, value) {
                    return Error::parameter_outOfBound.to_error_code();
                }
            }
            (*CCtxParams).targetCBlockSize = value as u32 as size_t;
            (*CCtxParams).targetCBlockSize
        }
        1004 => {
            if value != 0
                && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam7, value)
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).srcSizeHint = value;
            (*CCtxParams).srcSizeHint as size_t
        }
        1006 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam9, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).inBufferMode = value as ZSTD_bufferMode_e;
            (*CCtxParams).inBufferMode as size_t
        }
        1007 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam10, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).outBufferMode = value as ZSTD_bufferMode_e;
            (*CCtxParams).outBufferMode as size_t
        }
        1008 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam11, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).blockDelimiters = value as ZSTD_SequenceFormat_e;
            (*CCtxParams).blockDelimiters as size_t
        }
        1009 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam12, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).validateSequences = value;
            (*CCtxParams).validateSequences as size_t
        }
        1010 => {
            let Ok(value) = ParamSwitch::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).postBlockSplitter = value;
            (*CCtxParams).postBlockSplitter as size_t
        }
        1017 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam20, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).preBlockSplitter_level = value;
            (*CCtxParams).preBlockSplitter_level as size_t
        }
        1011 => {
            let Ok(value) = ParamSwitch::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).useRowMatchFinder = value;
            (*CCtxParams).useRowMatchFinder as size_t
        }
        1012 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam15, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).deterministicRefPrefix = (value != 0) as core::ffi::c_int;
            (*CCtxParams).deterministicRefPrefix as size_t
        }
        1013 => {
            let Ok(value) = ParamSwitch::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*CCtxParams).prefetchCDictTables = value;
            (*CCtxParams).prefetchCDictTables as size_t
        }
        1014 => {
            if !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam17, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).enableMatchFinderFallback = value;
            (*CCtxParams).enableMatchFinderFallback as size_t
        }
        1015 => {
            if value != 0
                && !ZSTD_cParam_withinBounds(ZSTD_cParameter::ZSTD_c_experimentalParam18, value)
            {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*CCtxParams).maxBlockSize = value as size_t;
            (*CCtxParams).maxBlockSize
        }
        1016 => {
            let Ok(value) = ParamSwitch::try_from(value) else {
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
            *value = ((*CCtxParams).fParams.noDictIDFlag == 0) as core::ffi::c_int;
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

/// Just applies `params` into `cctx`. No action is performed, parameters are merely stored.
/// If ZSTDMT is enabled, parameters are pushed to cctx->mtctx.
/// This is possible even if a compression is ongoing.
/// In which case, new parameters will be applied on the fly, starting with next compression job.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_setParametersUsingCCtxParams))]
pub unsafe extern "C" fn ZSTD_CCtx_setParametersUsingCCtxParams(
    cctx: *mut ZSTD_CCtx,
    params: *const ZSTD_CCtx_params,
) -> size_t {
    if (*cctx).streamStage != StreamStage::Init {
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
        (fparams.contentSizeFlag != 0) as core::ffi::c_int,
    );
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code_0 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_checksumFlag,
        (fparams.checksumFlag != 0) as core::ffi::c_int,
    );
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    let err_code_1 = ZSTD_CCtx_setParameter(
        cctx,
        ZSTD_cParameter::ZSTD_c_dictIDFlag,
        (fparams.noDictIDFlag == 0) as core::ffi::c_int,
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
    if (*cctx).streamStage != StreamStage::Init {
        return Error::stage_wrong.to_error_code();
    }

    (*cctx).pledgedSrcSizePlusOne = pledgedSrcSize.wrapping_add(1);

    0
}

/// Initializes the local dictionary using requested parameters.
///
/// NOTE: Initialization does not employ the pledged src size,
/// because the dictionary may be used for multiple compressions.
unsafe fn ZSTD_initLocalDict(cctx: *mut ZSTD_CCtx) -> size_t {
    let dl: *mut ZSTD_localDict = &mut (*cctx).localDict;
    if ((*dl).dict).is_null() {
        // No local dictionary
        return 0;
    }
    if !((*dl).cdict).is_null() {
        // Local dictionary already initialized
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
    if (*cctx).streamStage != StreamStage::Init {
        return Error::stage_wrong.to_error_code();
    }
    ZSTD_clearAllDicts(cctx); // erase any previously set dictionary
    if dict.is_null() || dictSize == 0 {
        return 0; // no dictionary
    }
    if dictLoadMethod == ZSTD_dlm_byRef {
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
        core::ptr::copy_nonoverlapping(dict.cast::<u8>(), dictBuffer.cast::<u8>(), dictSize);
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
    if (*cctx).streamStage != StreamStage::Init {
        return Error::stage_wrong.to_error_code();
    }

    // Free the existing local cdict (if any) to save memory.
    ZSTD_clearAllDicts(cctx);

    (*cctx).cdict = cdict;

    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CCtx_refThreadPool))]
pub unsafe extern "C" fn ZSTD_CCtx_refThreadPool(
    cctx: *mut ZSTD_CCtx,
    pool: *mut ZSTD_threadPool,
) -> size_t {
    if (*cctx).streamStage != StreamStage::Init {
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
    if (*cctx).streamStage != StreamStage::Init {
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

/// Also dumps dictionary
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
        (*cctx).streamStage = StreamStage::Init;
        (*cctx).pledgedSrcSizePlusOne = 0;
    }

    if matches!(
        reset,
        ZSTD_ResetDirective::ZSTD_reset_parameters
            | ZSTD_ResetDirective::ZSTD_reset_session_and_parameters
    ) {
        if (*cctx).streamStage != StreamStage::Init {
            return Error::stage_wrong.to_error_code();
        }
        ZSTD_clearAllDicts(cctx);
        return ZSTD_CCtxParams_reset(&mut (*cctx).requestedParams);
    }

    0
}

/// Control CParam values remain within authorized range.
///
/// # Returns
///
/// 0, or an error code if one value is beyond authorized range.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_checkCParams))]
pub extern "C" fn ZSTD_checkCParams(cParams: ZSTD_compressionParameters) -> size_t {
    if !ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_windowLog,
        cParams.windowLog as core::ffi::c_int,
    ) {
        return Error::parameter_outOfBound.to_error_code();
    }
    if !ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_chainLog,
        cParams.chainLog as core::ffi::c_int,
    ) {
        return Error::parameter_outOfBound.to_error_code();
    }
    if !ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_hashLog,
        cParams.hashLog as core::ffi::c_int,
    ) {
        return Error::parameter_outOfBound.to_error_code();
    }
    if !ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_searchLog,
        cParams.searchLog as core::ffi::c_int,
    ) {
        return Error::parameter_outOfBound.to_error_code();
    }
    if !ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_minMatch,
        cParams.minMatch as core::ffi::c_int,
    ) {
        return Error::parameter_outOfBound.to_error_code();
    }
    if !ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_targetLength,
        cParams.targetLength as core::ffi::c_int,
    ) {
        return Error::parameter_outOfBound.to_error_code();
    }
    if !ZSTD_cParam_withinBounds(
        ZSTD_cParameter::ZSTD_c_strategy,
        cParams.strategy as core::ffi::c_int,
    ) {
        return Error::parameter_outOfBound.to_error_code();
    }

    0
}

/// Make CParam values within valid range.
fn ZSTD_clampCParams(mut cParams: ZSTD_compressionParameters) -> ZSTD_compressionParameters {
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

/// Condition for correct operation: hashLog > 1.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_cycleLog))]
pub extern "C" fn ZSTD_cycleLog(hashLog: u32, strat: ZSTD_strategy) -> u32 {
    let btScale = (strat >= ZSTD_btlazy2) as u32;
    hashLog.wrapping_sub(btScale)
}

/// Returns an adjusted window log that is large enough to fit the source and the dictionary.
/// The zstd format says that the entire dictionary is valid if one byte of the dictionary
/// is within the window. So the hashLog and chainLog should be large enough to reference both
/// the dictionary and the window. So we must use this adjusted dictAndWindowLog when downsizing
/// the hashLog and windowLog.
///
/// NOTE: srcSize must not be ZSTD_CONTENTSIZE_UNKNOWN.
fn ZSTD_dictAndWindowLog(windowLog: u32, srcSize: u64, dictSize: u64) -> u32 {
    let maxWindowSize = 1 << ZSTD_WINDOWLOG_MAX;
    if dictSize == 0 {
        // No dictionary ==> No change
        return windowLog;
    }

    let windowSize = (1 << windowLog) as u64;
    let dictAndWindowSize = dictSize.wrapping_add(windowSize);
    // If the window size is already large enough to fit both the source and the dictionary
    // then just use the window size. Otherwise adjust so that it fits the dictionary and
    // the window.
    if windowSize >= dictSize.wrapping_add(srcSize) {
        // Window size large enough already
        windowLog
    } else if dictAndWindowSize >= maxWindowSize {
        // Larger than max window log
        (if size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) as u32
    } else {
        (ZSTD_highbit32((dictAndWindowSize as u32).wrapping_sub(1))).wrapping_add(1)
    }
}

/// Optimize `cPar` for a specified input (`srcSize` and `dictSize`).
/// Mostly downsize to reduce memory consumption and initialization latency.
/// `srcSize` can be ZSTD_CONTENTSIZE_UNKNOWN when not known.
/// `mode` is the mode for parameter adjustment. See docs for [`CParamMode`].
///
/// Note: `srcSize==0` means 0!
///
/// Condition: cPar is presumed validated (can be checked using ZSTD_checkCParams()).
fn ZSTD_adjustCParams_internal(
    mut cPar: ZSTD_compressionParameters,
    mut srcSize: core::ffi::c_ulonglong,
    mut dictSize: size_t,
    mode: CParamMode,
    mut useRowMatchFinder: ParamSwitch,
) -> ZSTD_compressionParameters {
    let minSrcSize = 513; // (1<<9) + 1
    let maxWindowResize = (1
        << ((if size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) - 1)) as u64;

    match mode as core::ffi::c_uint {
        2 => {
            // Assume a small source size when creating a dictionary
            // with an unknown source size.
            if dictSize != 0 && srcSize == ZSTD_CONTENTSIZE_UNKNOWN {
                srcSize = minSrcSize as core::ffi::c_ulonglong;
            }
        }
        1 => {
            // Dictionary has its own dedicated parameters which have
            // already been selected. We are selecting parameters
            // for only the source.
            dictSize = 0;
        }
        3 | 0 | _ => {
            // If we don't know the source size, don't make any
            // assumptions about it. We will already have selected
            // smaller parameters if a dictionary is in use.
        }
    }

    // resize windowLog if input is small enough, to use less memory
    if srcSize <= maxWindowResize && dictSize as u64 <= maxWindowResize {
        let tSize = srcSize.wrapping_add(dictSize as core::ffi::c_ulonglong) as u32;
        static hashSizeMin: u32 = (1 << ZSTD_HASHLOG_MIN) as u32;
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
        // minimum wlog required for valid frame header
        cPar.windowLog = ZSTD_WINDOWLOG_ABSOLUTEMIN as core::ffi::c_uint;
    }

    // We can't use more than 32 bits of hash in total, so that means that we require:
    // (hashLog + 8) <= 32 && (chainLog + 8) <= 32
    if mode == CParamMode::CreateCDict && unsafe { ZSTD_CDictIndicesAreTagged(&cPar) } {
        let maxShortCacheHashLog = (32 - ZSTD_SHORT_CACHE_TAG_BITS) as u32;
        if cPar.hashLog > maxShortCacheHashLog {
            cPar.hashLog = maxShortCacheHashLog;
        }
        if cPar.chainLog > maxShortCacheHashLog {
            cPar.chainLog = maxShortCacheHashLog;
        }
    }

    // At this point, we aren't 100% sure if we are using the row match finder.
    // Unless it is explicitly disabled, conservatively assume that it is enabled.
    // In this case it will only be disabled for small sources, so shrinking the
    // hash log a little bit shouldn't result in any ratio loss.
    if useRowMatchFinder == ParamSwitch::Auto {
        useRowMatchFinder = ParamSwitch::Enable;
    }

    // We can't hash more than 32-bits in total. So that means that we require:
    // (hashLog - rowLog + 8) <= 32
    if ZSTD_rowMatchFinderUsed(cPar.strategy, useRowMatchFinder) {
        // Switch to 32-entry rows if searchLog is 5 (or more)
        let rowLog = cPar.searchLog.clamp(4, 6);
        let maxRowHashLog = (32 - ZSTD_ROW_HASH_TAG_BITS) as u32;
        let maxHashLog = maxRowHashLog.wrapping_add(rowLog);
        if cPar.hashLog > maxHashLog {
            cPar.hashLog = maxHashLog;
        }
    }

    cPar
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_adjustCParams))]
pub extern "C" fn ZSTD_adjustCParams(
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
        CParamMode::Unknown,
        ParamSwitch::Auto,
    )
}

fn ZSTD_overrideCParams(
    cParams: &mut ZSTD_compressionParameters,
    overrides: &ZSTD_compressionParameters,
) {
    if overrides.windowLog != 0 {
        cParams.windowLog = overrides.windowLog;
    }
    if overrides.hashLog != 0 {
        cParams.hashLog = overrides.hashLog;
    }
    if overrides.chainLog != 0 {
        cParams.chainLog = overrides.chainLog;
    }
    if overrides.searchLog != 0 {
        cParams.searchLog = overrides.searchLog;
    }
    if overrides.minMatch != 0 {
        cParams.minMatch = overrides.minMatch;
    }
    if overrides.targetLength != 0 {
        cParams.targetLength = overrides.targetLength;
    }
    if overrides.strategy as u64 != 0 {
        cParams.strategy = overrides.strategy;
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getCParamsFromCCtxParams))]
pub unsafe extern "C" fn ZSTD_getCParamsFromCCtxParams(
    CCtxParams: *const ZSTD_CCtx_params,
    srcSizeHint: u64,
    dictSize: size_t,
    mode: core::ffi::c_int,
) -> ZSTD_compressionParameters {
    let mode = CParamMode::from(mode);

    ZSTD_getCParamsFromCCtxParams_internal(&*CCtxParams, srcSizeHint, dictSize, mode)
}

pub fn ZSTD_getCParamsFromCCtxParams_internal(
    CCtxParams: &ZSTD_CCtx_params,
    mut srcSizeHint: u64,
    dictSize: size_t,
    mode: CParamMode,
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
        && CCtxParams.srcSizeHint > 0
    {
        srcSizeHint = CCtxParams.srcSizeHint as u64;
    }
    cParams = ZSTD_getCParams_internal(
        CCtxParams.compressionLevel,
        srcSizeHint as core::ffi::c_ulonglong,
        dictSize,
        mode,
    );
    if CCtxParams.ldmParams.enableLdm == ParamSwitch::Enable {
        cParams.windowLog = ZSTD_LDM_DEFAULT_WINDOW_LOG as core::ffi::c_uint;
    }
    ZSTD_overrideCParams(&mut cParams, &CCtxParams.cParams);

    // srcSizeHint == 0 means 0
    ZSTD_adjustCParams_internal(
        cParams,
        srcSizeHint as core::ffi::c_ulonglong,
        dictSize,
        mode,
        CCtxParams.useRowMatchFinder,
    )
}

fn ZSTD_sizeof_matchState(
    cParams: &ZSTD_compressionParameters,
    useRowMatchFinder: ParamSwitch,
    enableDedicatedDictSearch: core::ffi::c_int,
    forCCtx: u32,
) -> size_t {
    // chain table size should be 0 for fast or row-hash strategies
    let chainSize = if ZSTD_allocateChainTable(
        cParams.strategy,
        useRowMatchFinder,
        enableDedicatedDictSearch != 0 && forCCtx == 0,
    ) {
        (1 as size_t) << cParams.chainLog
    } else {
        0
    };
    let hSize = (1 as size_t) << cParams.hashLog;
    let hashLog3 = if forCCtx != 0 && cParams.minMatch == 3 {
        cParams.windowLog.min(ZSTD_HASHLOG3_MAX)
    } else {
        0
    };
    let h3Size = if hashLog3 != 0 {
        (1 as size_t) << hashLog3
    } else {
        0
    };
    // We don't use ZSTD_cwksp_alloc_size() here because the tables aren't
    // surrounded by redzones in ASAN.
    let tableSpace = chainSize
        .wrapping_mul(size_of::<u32>())
        .wrapping_add(hSize.wrapping_mul(size_of::<u32>()))
        .wrapping_add(h3Size.wrapping_mul(size_of::<u32>()));
    let optPotentialSpace =
        (ZSTD_cwksp_aligned64_alloc_size((usize::from(MaxML) + 1).wrapping_mul(size_of::<u32>())))
            .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
                (usize::from(MaxLL) + 1).wrapping_mul(size_of::<u32>()),
            ))
            .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
                ((MaxOff + 1) as size_t).wrapping_mul(size_of::<u32>()),
            ))
            .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
                ((1 << Litbits) as size_t).wrapping_mul(size_of::<u32>()),
            ))
            .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
                (ZSTD_OPT_SIZE as size_t).wrapping_mul(size_of::<ZSTD_match_t>()),
            ))
            .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
                (ZSTD_OPT_SIZE as size_t).wrapping_mul(size_of::<ZSTD_optimal_t>()),
            ));
    let lazyAdditionalSpace = if ZSTD_rowMatchFinderUsed(cParams.strategy, useRowMatchFinder) {
        ZSTD_cwksp_aligned64_alloc_size(hSize)
    } else {
        0
    };
    let optSpace = if forCCtx != 0 && cParams.strategy >= ZSTD_btopt {
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

/// Helper function for calculating memory requirements.
///
/// Gives a tighter bound than ZSTD_sequenceBound() by taking minMatch into account.
fn ZSTD_maxNbSeq(
    blockSize: size_t,
    minMatch: core::ffi::c_uint,
    useSequenceProducer: bool,
) -> size_t {
    let divider = (if minMatch == 3 || useSequenceProducer {
        3
    } else {
        4
    }) as u32;
    blockSize / divider as size_t
}

fn ZSTD_estimateCCtxSize_usingCCtxParams_internal(
    cParams: &ZSTD_compressionParameters,
    ldmParams: &ldmParams_t,
    isStatic: core::ffi::c_int,
    useRowMatchFinder: ParamSwitch,
    buffInSize: size_t,
    buffOutSize: size_t,
    pledgedSrcSize: u64,
    useSequenceProducer: bool,
    maxBlockSize: size_t,
) -> size_t {
    let windowSize = ((1 as core::ffi::c_ulonglong) << cParams.windowLog)
        .min(pledgedSrcSize as core::ffi::c_ulonglong) // pledgedSrcSize can be 0, so .clamp() would panic
        .max(1) as size_t;
    let blockSize = ZSTD_resolveMaxBlockSize(maxBlockSize).min(windowSize);
    let maxNbSeq = ZSTD_maxNbSeq(blockSize, cParams.minMatch, useSequenceProducer);
    let tokenSpace = (ZSTD_cwksp_alloc_size(WILDCOPY_OVERLENGTH.wrapping_add(blockSize)))
        .wrapping_add(ZSTD_cwksp_aligned64_alloc_size(
            maxNbSeq.wrapping_mul(size_of::<SeqDef>()),
        ))
        .wrapping_add(3 * ZSTD_cwksp_alloc_size(maxNbSeq.wrapping_mul(size_of::<u8>())));
    let tmpWorkSpace = ZSTD_cwksp_alloc_size(
        (((8 << 10) + 512) as size_t)
            .wrapping_add(size_of::<core::ffi::c_uint>().wrapping_mul(MaxSeq + 2))
            .max(ZSTD_SLIPBLOCK_WORKSPACESIZE),
    );
    let blockStateSpace = 2 * ZSTD_cwksp_alloc_size(size_of::<ZSTD_compressedBlockState_t>());
    let matchStateSize = ZSTD_sizeof_matchState(cParams, useRowMatchFinder, 0, 1);

    let ldmSpace = ZSTD_ldm_getTableSize(*ldmParams);
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq(*ldmParams, blockSize);
    let ldmSeqSpace = if ldmParams.enableLdm == ParamSwitch::Enable {
        ZSTD_cwksp_aligned64_alloc_size(maxNbLdmSeq.wrapping_mul(size_of::<rawSeq>()))
    } else {
        0
    };

    let bufferSpace =
        (ZSTD_cwksp_alloc_size(buffInSize)).wrapping_add(ZSTD_cwksp_alloc_size(buffOutSize));
    let cctxSpace = if isStatic != 0 {
        ZSTD_cwksp_alloc_size(size_of::<ZSTD_CCtx>())
    } else {
        0
    };

    let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
    let externalSeqSpace = if useSequenceProducer {
        ZSTD_cwksp_aligned64_alloc_size(maxNbExternalSeq.wrapping_mul(size_of::<ZSTD_Sequence>()))
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
    let cParams = ZSTD_getCParamsFromCCtxParams_internal(
        &*params,
        ZSTD_CONTENTSIZE_UNKNOWN,
        0,
        CParamMode::NoAttachDict,
    );
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode((*params).useRowMatchFinder, &cParams);

    if (*params).nbWorkers > 0 {
        return Error::GENERIC.to_error_code();
    }

    // estimateCCtxSize is for one-shot compression. So no buffers should
    // be needed. However, we still allocate two 0-sized buffers, which can
    // take space under ASAN.
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
    if ZSTD_rowMatchFinderSupported(cParams.strategy) {
        // Pick bigger of not using and using row-based matchfinder for greedy and lazy strategies
        let mut noRowCCtxSize: size_t = 0;
        let mut rowCCtxSize: size_t = 0;
        initialParams.useRowMatchFinder = ParamSwitch::Disable;
        noRowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&initialParams);
        initialParams.useRowMatchFinder = ParamSwitch::Enable;
        rowCCtxSize = ZSTD_estimateCCtxSize_usingCCtxParams(&initialParams);
        noRowCCtxSize.max(rowCCtxSize)
    } else {
        ZSTD_estimateCCtxSize_usingCCtxParams(&initialParams)
    }
}

static SRC_SIZE_TIERS: [core::ffi::c_ulonglong; 4] = [
    16 * (1 << 10),
    128 * (1 << 10),
    256 * (1 << 10),
    ZSTD_CONTENTSIZE_UNKNOWN,
];

unsafe extern "C" fn ZSTD_estimateCCtxSize_internal(compressionLevel: core::ffi::c_int) -> size_t {
    let mut largestSize = 0;
    for srcSizeHint in SRC_SIZE_TIERS {
        // Choose the set of cParams for a given level across all srcSizes that give the largest cctxSize
        let cParams =
            ZSTD_getCParams_internal(compressionLevel, srcSizeHint, 0, CParamMode::NoAttachDict);
        largestSize = ZSTD_estimateCCtxSize_usingCParams(cParams).max(largestSize);
    }
    largestSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCCtxSize))]
pub unsafe extern "C" fn ZSTD_estimateCCtxSize(compressionLevel: core::ffi::c_int) -> size_t {
    let mut memBudget = 0;
    // Negative compression levels are used for fast mode, these should still do 1 iteration
    let start = compressionLevel.min(1);
    for level in start..compressionLevel + 1 {
        // Ensure monotonically increasing memory usage as compression level increases
        let newMB = ZSTD_estimateCCtxSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
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
    let cParams = ZSTD_getCParamsFromCCtxParams_internal(
        &*params,
        ZSTD_CONTENTSIZE_UNKNOWN,
        0,
        CParamMode::NoAttachDict,
    );
    let blockSize = ZSTD_resolveMaxBlockSize((*params).maxBlockSize).min(1 << cParams.windowLog);
    let inBuffSize = if (*params).inBufferMode == ZSTD_bm_buffered {
        ((1 as size_t) << cParams.windowLog).wrapping_add(blockSize)
    } else {
        0
    };
    let outBuffSize = if (*params).outBufferMode == ZSTD_bm_buffered {
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
    if ZSTD_rowMatchFinderSupported(cParams.strategy) {
        // Pick bigger of not using and using row-based matchfinder for greedy and lazy strategies
        let mut noRowCCtxSize: size_t = 0;
        let mut rowCCtxSize: size_t = 0;
        initialParams.useRowMatchFinder = ParamSwitch::Disable;
        noRowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&initialParams);
        initialParams.useRowMatchFinder = ParamSwitch::Enable;
        rowCCtxSize = ZSTD_estimateCStreamSize_usingCCtxParams(&initialParams);
        noRowCCtxSize.max(rowCCtxSize)
    } else {
        ZSTD_estimateCStreamSize_usingCCtxParams(&initialParams)
    }
}

unsafe fn ZSTD_estimateCStreamSize_internal(compressionLevel: core::ffi::c_int) -> size_t {
    let cParams = ZSTD_getCParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        0,
        CParamMode::NoAttachDict,
    );
    ZSTD_estimateCStreamSize_usingCParams(cParams)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCStreamSize))]
pub unsafe extern "C" fn ZSTD_estimateCStreamSize(compressionLevel: core::ffi::c_int) -> size_t {
    let mut memBudget = 0;
    let start = compressionLevel.min(1);
    for level in start..compressionLevel + 1 {
        let newMB = ZSTD_estimateCStreamSize_internal(level);
        if newMB > memBudget {
            memBudget = newMB;
        }
    }
    memBudget
}

/// Tells how much data has been consumed (input) and produced (output) for current frame.
///
/// Able to count progression inside worker threads (non-blocking mode).
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
    // simplified; some data might still be left within streaming output buffer
    fp.flushed = (*cctx).producedCSize;
    fp.currentJobID = 0;
    fp.nbActiveWorkers = 0;
    fp
}

/// Only useful for multithreading scenarios currently (nbWorkers >= 1).
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_toFlushNow))]
pub unsafe extern "C" fn ZSTD_toFlushNow(cctx: *mut ZSTD_CCtx) -> size_t {
    if (*cctx).appliedParams.nbWorkers > 0 {
        return ZSTDMT_toFlushNow((*cctx).mtctx);
    }

    // over-simplification; could also check if context is currently running in
    // streaming mode, and in which case, report how many bytes are left to be
    // flushed within output buffer
    0
}

fn ZSTD_assertEqualCParams(
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
    for i in 0..ZSTD_REP_NUM {
        (*bs).rep[i as usize] = repStartValue[i as usize];
    }
    (*bs).entropy.huf.repeatMode = HUF_repeat_none;
    (*bs).entropy.fse.offcode_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.matchlength_repeatMode = FSE_repeat_none;
    (*bs).entropy.fse.litlength_repeatMode = FSE_repeat_none;
}

/// Invalidate all the matches in the match finder tables.
/// Requires nextSrc and base to be set (can be NULL).
unsafe fn ZSTD_invalidateMatchState(ms: &mut ZSTD_MatchState_t) {
    ZSTD_window_clear(&mut ms.window);
    ms.nextToUpdate = ms.window.dictLimit;
    ms.loadedDictEnd = 0;
    ms.opt.litLengthSum = 0; // force reset of btopt stats
    ms.dictMatchState = core::ptr::null();
}

/// Mixes bits in a 64 bits in a value, based on XXH3_rrmxmx
fn ZSTD_bitmix(mut val: u64, len: u64) -> u64 {
    val ^= val.rotate_right(49) ^ val.rotate_right(24);
    val = val.wrapping_mul(0x9fb21c651e98df25u64);
    val ^= (val >> 35).wrapping_add(len);
    val = val.wrapping_mul(0x9fb21c651e98df25u64);
    val ^ val >> 28
}

/// Mixes in the hashSalt and hashSaltEntropy to create a new hashSalt
fn ZSTD_advanceHashSalt(ms: &mut ZSTD_MatchState_t) {
    ms.hashSalt = ZSTD_bitmix(ms.hashSalt, 8) ^ ZSTD_bitmix(ms.hashSaltEntropy as u64, 4);
}

unsafe fn ZSTD_reset_matchState(
    ms: &mut ZSTD_MatchState_t,
    ws: &mut ZSTD_cwksp,
    cParams: &ZSTD_compressionParameters,
    useRowMatchFinder: ParamSwitch,
    crp: ZSTD_compResetPolicy_e,
    forceResetIndex: ZSTD_indexResetPolicy_e,
    forWho: ZSTD_resetTarget_e,
) -> size_t {
    // disable chain table allocation for fast or row-based strategies
    let chainSize = if ZSTD_allocateChainTable(
        cParams.strategy,
        useRowMatchFinder,
        ms.dedicatedDictSearch != 0 && forWho == ZSTD_resetTarget_CDict,
    ) {
        (1 as size_t) << cParams.chainLog
    } else {
        0
    };
    let hSize = (1 as size_t) << cParams.hashLog;
    let hashLog3 = if forWho == ZSTD_resetTarget_CCtx && cParams.minMatch == 3 {
        cParams.windowLog.min(ZSTD_HASHLOG3_MAX)
    } else {
        0
    };
    let h3Size = if hashLog3 != 0 {
        (1 as size_t) << hashLog3
    } else {
        0
    };

    if forceResetIndex == ZSTDirp_reset {
        ZSTD_window_init(&mut ms.window);
        ZSTD_cwksp_mark_tables_dirty(ws);
    }

    ms.hashLog3 = hashLog3;
    ms.lazySkipping = 0;

    ZSTD_invalidateMatchState(ms);

    ZSTD_cwksp_clear_tables(ws);

    // table Space
    ms.hashTable = ZSTD_cwksp_reserve_table(ws, hSize.wrapping_mul(size_of::<u32>())) as *mut u32;
    ms.chainTable =
        ZSTD_cwksp_reserve_table(ws, chainSize.wrapping_mul(size_of::<u32>())) as *mut u32;
    ms.hashTable3 = ZSTD_cwksp_reserve_table(ws, h3Size.wrapping_mul(size_of::<u32>())) as *mut u32;
    if ZSTD_cwksp_reserve_failed(ws) {
        return Error::memory_allocation.to_error_code();
    }

    if crp != ZSTDcrp_leaveDirty {
        // reset tables only
        ZSTD_cwksp_clean_tables(ws);
    }

    if ZSTD_rowMatchFinderUsed(cParams.strategy, useRowMatchFinder) {
        // Row match finder needs an additional table of hashes ("tags")
        let tagTableSize = hSize;
        // We want to generate a new salt in case we reset a Cctx, but we always want to use
        // 0 when we reset a Cdict
        if forWho == ZSTD_resetTarget_CCtx {
            ms.tagTable = ZSTD_cwksp_reserve_aligned_init_once(ws, tagTableSize) as *mut u8;
            ZSTD_advanceHashSalt(ms);
        } else {
            // When we are not salting we want to always memset the memory
            ms.tagTable = ZSTD_cwksp_reserve_aligned64(ws, tagTableSize) as *mut u8;
            ptr::write_bytes(ms.tagTable, 0, tagTableSize);
            ms.hashSalt = 0;
        }

        // Switch to 32-entry rows if searchLog is 5 (or more)
        let rowLog = cParams.searchLog.clamp(4, 6);
        ms.rowHashLog = cParams.hashLog.wrapping_sub(rowLog);
    }

    // opt parser space
    if forWho == ZSTD_resetTarget_CCtx && cParams.strategy >= ZSTD_btopt {
        ms.opt.litFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((1 << Litbits) as size_t).wrapping_mul(size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.litLengthFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            (usize::from(MaxLL) + 1).wrapping_mul(size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.matchLengthFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            (usize::from(MaxML) + 1).wrapping_mul(size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.offCodeFreq = ZSTD_cwksp_reserve_aligned64(
            ws,
            ((MaxOff + 1) as size_t).wrapping_mul(size_of::<core::ffi::c_uint>()),
        ) as *mut core::ffi::c_uint;
        ms.opt.matchTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            (ZSTD_OPT_SIZE as size_t).wrapping_mul(size_of::<ZSTD_match_t>()),
        ) as *mut ZSTD_match_t;
        ms.opt.priceTable = ZSTD_cwksp_reserve_aligned64(
            ws,
            (ZSTD_OPT_SIZE as size_t).wrapping_mul(size_of::<ZSTD_optimal_t>()),
        ) as *mut ZSTD_optimal_t;
    }

    ms.cParams = *cParams;

    if ZSTD_cwksp_reserve_failed(ws) {
        return Error::memory_allocation.to_error_code();
    }

    0
}

pub const ZSTD_INDEXOVERFLOW_MARGIN: core::ffi::c_int = 16 * (1 << 20);

/// Minor optimization: prefer memset() rather than reduceIndex() which is measurably slow in some
/// circumstances (reported for Visual Studio). Works when re-using a context for a lot of smallish
/// inputs: if all inputs are smaller than ZSTD_INDEXOVERFLOW_MARGIN, memset() will be triggered
/// before reduceIndex().
fn ZSTD_indexTooCloseToMax(w: ZSTD_window_t) -> bool {
    (w.nextSrc).wrapping_offset_from(w.base) as size_t
        > (if MEM_64bits() {
            (3500 as core::ffi::c_uint)
                .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
        } else {
            (2000 as core::ffi::c_uint)
                .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
        })
        .wrapping_sub(ZSTD_INDEXOVERFLOW_MARGIN as core::ffi::c_uint) as size_t
}

/// When dictionaries are larger than ZSTD_CHUNKSIZE_MAX they can't be loaded in
/// one go generically. So we ensure that in that case we reset the tables to zero,
/// so that we can load as much of the dictionary as possible.
fn ZSTD_dictTooBig(loadedDictSize: size_t) -> bool {
    loadedDictSize
        > (-(1 as core::ffi::c_int) as u32).wrapping_sub(if MEM_64bits() {
            (3500 as core::ffi::c_uint)
                .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
        } else {
            (2000 as core::ffi::c_uint)
                .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
        }) as size_t
}

/// loadedDictSize is the size of the dictionary to be loaded
/// into the context, if any. If no dictionary is used, or the
/// dictionary is being attached / copied, then pass 0.
///
/// Note: `params` are assumed fully validated at this stage.
unsafe fn ZSTD_resetCCtx_internal(
    zc: *mut ZSTD_CCtx,
    mut params: &ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    loadedDictSize: size_t,
    crp: ZSTD_compResetPolicy_e,
    zbuff: BufferedPolicy,
) -> size_t {
    let ws = &mut (*zc).workspace;

    (*zc).isFirstBlock = 1;

    // Set applied params early so we can modify them for LDM,
    // and point params at the applied params.
    (*zc).appliedParams = *params;
    params = &mut (*zc).appliedParams;

    if params.ldmParams.enableLdm == ParamSwitch::Enable {
        // Adjust long distance matching parameters
        ZSTD_ldm_adjustParameters(&mut (*zc).appliedParams.ldmParams, &params.cParams);
    }

    let windowSize = ((1 as size_t) << params.cParams.windowLog)
        .min(pledgedSrcSize as size_t) // pledgedSrcSize can be 0, so .clamp() would panic
        .max(1);
    let blockSize = params.maxBlockSize.min(windowSize);
    let maxNbSeq = ZSTD_maxNbSeq(
        blockSize,
        params.cParams.minMatch,
        ZSTD_hasExtSeqProd(params),
    );
    let buffOutSize =
        if zbuff == BufferedPolicy::Buffered && params.outBufferMode == ZSTD_bm_buffered {
            (ZSTD_compressBound(blockSize)).wrapping_add(1)
        } else {
            0
        };
    let buffInSize = if zbuff == BufferedPolicy::Buffered && params.inBufferMode == ZSTD_bm_buffered
    {
        windowSize.wrapping_add(blockSize)
    } else {
        0
    };
    let maxNbLdmSeq = ZSTD_ldm_getMaxNbSeq(params.ldmParams, blockSize);

    let indexTooClose = ZSTD_indexTooCloseToMax((*zc).blockState.matchState.window);
    let dictTooBig = ZSTD_dictTooBig(loadedDictSize);
    let mut needsIndexReset = (if indexTooClose || dictTooBig || (*zc).initialized == 0 {
        ZSTDirp_reset as core::ffi::c_int
    } else {
        ZSTDirp_continue as core::ffi::c_int
    }) as ZSTD_indexResetPolicy_e;

    let neededSpace = ZSTD_estimateCCtxSize_usingCCtxParams_internal(
        &params.cParams,
        &params.ldmParams,
        ((*zc).staticSize != 0) as core::ffi::c_int,
        params.useRowMatchFinder,
        buffInSize,
        buffOutSize,
        pledgedSrcSize,
        ZSTD_hasExtSeqProd(params),
        params.maxBlockSize,
    );

    let err_code = neededSpace;
    if ERR_isError(err_code) {
        return err_code;
    }

    if (*zc).staticSize == 0 {
        ZSTD_cwksp_bump_oversized_duration(ws, 0);
    }

    // Check if workspace is large enough, alloc a new one if needed
    let workspaceTooSmall = ZSTD_cwksp_sizeof(ws) < neededSpace;
    let workspaceWasteful = ZSTD_cwksp_check_wasteful(ws, neededSpace);
    let resizeWorkspace = workspaceTooSmall || workspaceWasteful;

    if resizeWorkspace {
        if (*zc).staticSize != 0 {
            return Error::memory_allocation.to_error_code();
        }
        needsIndexReset = ZSTDirp_reset;
        ZSTD_cwksp_free(ws, (*zc).customMem);
        let err_code_0 = ZSTD_cwksp_create(ws, neededSpace, (*zc).customMem);
        if ERR_isError(err_code_0) {
            return err_code_0;
        }

        // Statically sized space.
        // tmpWorkspace never moves,
        // though prev/next block swap places
        (*zc).blockState.prevCBlock =
            ZSTD_cwksp_reserve_object(ws, size_of::<ZSTD_compressedBlockState_t>())
                as *mut ZSTD_compressedBlockState_t;
        if ((*zc).blockState.prevCBlock).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*zc).blockState.nextCBlock =
            ZSTD_cwksp_reserve_object(ws, size_of::<ZSTD_compressedBlockState_t>())
                as *mut ZSTD_compressedBlockState_t;
        if ((*zc).blockState.nextCBlock).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*zc).tmpWorkspace = ZSTD_cwksp_reserve_object(
            ws,
            (((8 << 10) + 512) as size_t)
                .wrapping_add(size_of::<core::ffi::c_uint>().wrapping_mul(MaxSeq + 2))
                .max(ZSTD_SLIPBLOCK_WORKSPACESIZE),
        );
        if ((*zc).tmpWorkspace).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*zc).tmpWkspSize = (((8 << 10) + 512) as size_t)
            .wrapping_add(size_of::<core::ffi::c_uint>().wrapping_mul(MaxSeq + 2))
            .max(ZSTD_SLIPBLOCK_WORKSPACESIZE);
    }

    ZSTD_cwksp_clear(ws);

    // init params
    (*zc).blockState.matchState.cParams = params.cParams;
    (*zc).blockState.matchState.prefetchCDictTables =
        (params.prefetchCDictTables == ParamSwitch::Enable) as core::ffi::c_int;
    (*zc).pledgedSrcSizePlusOne = pledgedSrcSize.wrapping_add(1) as core::ffi::c_ulonglong;
    (*zc).consumedSrcSize = 0;
    (*zc).producedCSize = 0;
    if pledgedSrcSize as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN {
        (*zc).appliedParams.fParams.contentSizeFlag = 0;
    }
    (*zc).blockSizeMax = blockSize;

    ZSTD_XXH64_reset(&mut (*zc).xxhState, 0);
    (*zc).stage = CompressionStage::Init;
    (*zc).dictID = 0;
    (*zc).dictContentSize = 0;

    ZSTD_reset_compressedBlockState((*zc).blockState.prevCBlock);

    let err_code_1 = ZSTD_reset_matchState(
        &mut (*zc).blockState.matchState,
        ws,
        &params.cParams,
        params.useRowMatchFinder,
        crp,
        needsIndexReset,
        ZSTD_resetTarget_CCtx,
    );

    if ERR_isError(err_code_1) {
        return err_code_1;
    }

    (*zc).seqStore.sequencesStart =
        ZSTD_cwksp_reserve_aligned64(ws, maxNbSeq.wrapping_mul(size_of::<SeqDef>())) as *mut SeqDef;

    // ldm hash table
    if params.ldmParams.enableLdm == ParamSwitch::Enable {
        // TODO: avoid memset?
        let ldmHSize = (1 as size_t) << params.ldmParams.hashLog;
        (*zc).ldmState.hashTable =
            ZSTD_cwksp_reserve_aligned64(ws, ldmHSize.wrapping_mul(size_of::<ldmEntry_t>()))
                as *mut ldmEntry_t;
        ptr::write_bytes(
            (*zc).ldmState.hashTable as *mut u8,
            0,
            ldmHSize.wrapping_mul(size_of::<ldmEntry_t>()),
        );
        (*zc).ldmSequences =
            ZSTD_cwksp_reserve_aligned64(ws, maxNbLdmSeq.wrapping_mul(size_of::<rawSeq>()))
                as *mut rawSeq;
        (*zc).maxNbLdmSequences = maxNbLdmSeq;

        ZSTD_window_init(&mut (*zc).ldmState.window);
        (*zc).ldmState.loadedDictEnd = 0;
    }

    // reserve space for block-level external sequences
    if ZSTD_hasExtSeqProd(params) {
        let maxNbExternalSeq = ZSTD_sequenceBound(blockSize);
        (*zc).extSeqBufCapacity = maxNbExternalSeq;
        (*zc).extSeqBuf = ZSTD_cwksp_reserve_aligned64(
            ws,
            maxNbExternalSeq.wrapping_mul(size_of::<ZSTD_Sequence>()),
        ) as *mut ZSTD_Sequence;
    }

    // buffers
    // ZSTD_wildcopy() is used to copy into the literals buffer,
    // so we have to oversize the buffer by WILDCOPY_OVERLENGTH bytes.
    (*zc).seqStore.litStart =
        ZSTD_cwksp_reserve_buffer(ws, blockSize.wrapping_add(WILDCOPY_OVERLENGTH));
    (*zc).seqStore.maxNbLit = blockSize;

    (*zc).bufferedPolicy = zbuff;
    (*zc).inBuffSize = buffInSize;
    (*zc).inBuff = ZSTD_cwksp_reserve_buffer(ws, buffInSize);
    (*zc).outBuffSize = buffOutSize;
    (*zc).outBuff = ZSTD_cwksp_reserve_buffer(ws, buffOutSize);

    // ldm bucketOffsets table
    if params.ldmParams.enableLdm == ParamSwitch::Enable {
        // TODO: avoid memset?
        let numBuckets =
            1 << (params.ldmParams.hashLog).wrapping_sub(params.ldmParams.bucketSizeLog);
        (*zc).ldmState.bucketOffsets = ZSTD_cwksp_reserve_buffer(ws, numBuckets);
        ptr::write_bytes((*zc).ldmState.bucketOffsets, 0, numBuckets);
    }

    // sequences storage
    ZSTD_referenceExternalSequences(zc, core::ptr::null_mut(), 0);
    (*zc).seqStore.maxNbSeq = maxNbSeq;
    (*zc).seqStore.llCode = ZSTD_cwksp_reserve_buffer(ws, maxNbSeq.wrapping_mul(size_of::<u8>()));
    (*zc).seqStore.mlCode = ZSTD_cwksp_reserve_buffer(ws, maxNbSeq.wrapping_mul(size_of::<u8>()));
    (*zc).seqStore.ofCode = ZSTD_cwksp_reserve_buffer(ws, maxNbSeq.wrapping_mul(size_of::<u8>()));
    (*zc).initialized = 1;

    0
}

/// Ensures next compression will not use repcodes from previous block.
///
/// Note: only works with regular variant; do not use with extDict variant!
pub unsafe fn ZSTD_invalidateRepCodes(cctx: *mut ZSTD_CCtx) {
    for i in 0..ZSTD_REP_NUM {
        (*(*cctx).blockState.prevCBlock).rep[i as usize] = 0;
    }
}

/// Approximate sizes for each strategy past which copying the dictionary tables into the working
/// context is faster than using them in-place.
static attachDictSizeCutoffs: [size_t; 10] = [
    (8 * (1 << 10)) as size_t,
    (8 * (1 << 10)) as size_t,
    (16 * (1 << 10)) as size_t,
    (32 * (1 << 10)) as size_t,
    (32 * (1 << 10)) as size_t,
    (32 * (1 << 10)) as size_t,
    (32 * (1 << 10)) as size_t,
    (32 * (1 << 10)) as size_t,
    (8 * (1 << 10)) as size_t,
    (8 * (1 << 10)) as size_t,
];

unsafe fn ZSTD_shouldAttachDict(
    cdict: *const ZSTD_CDict,
    params: &ZSTD_CCtx_params,
    pledgedSrcSize: u64,
) -> bool {
    let cutoff = attachDictSizeCutoffs[(*cdict).matchState.cParams.strategy as usize];
    let dedicatedDictSearch = (*cdict).matchState.dedicatedDictSearch;
    dedicatedDictSearch != 0
        || (pledgedSrcSize <= cutoff as u64
            || pledgedSrcSize as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
            || params.attachDictPref == ZSTD_dictAttachPref_e::ZSTD_dictForceAttach)
            && params.attachDictPref != ZSTD_dictAttachPref_e::ZSTD_dictForceCopy
            && params.forceWindow == 0
}

unsafe fn ZSTD_resetCCtx_byAttachingCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: BufferedPolicy,
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
        CParamMode::AttachDict,
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

        // prep working match state so dict matches never have negative indices
        // when they are translated to the working context's index space.
        if (*cctx).blockState.matchState.window.dictLimit < cdictEnd {
            (*cctx).blockState.matchState.window.nextSrc =
                ((*cctx).blockState.matchState.window.base).wrapping_offset(cdictEnd as isize);
            ZSTD_window_clear(&mut (*cctx).blockState.matchState.window);
        }
        // loadedDictEnd is expressed within the referential of the active context
        (*cctx).blockState.matchState.loadedDictEnd =
            (*cctx).blockState.matchState.window.dictLimit;
    }

    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;

    // copy block state
    core::ptr::copy_nonoverlapping(
        &raw const (*cdict).cBlockState,
        (*cctx).blockState.prevCBlock,
        1,
    );

    0
}

unsafe fn ZSTD_copyCDictTableIntoCCtx(
    dst: *mut u32,
    src: *const u32,
    tableSize: size_t,
    cParams: *const ZSTD_compressionParameters,
) {
    if ZSTD_CDictIndicesAreTagged(cParams) {
        // Remove tags from the CDict table if they are present.
        // See docs on "short cache" in zstd_compress_internal.h for context.
        for i in 0..tableSize {
            let taggedIndex = *src.add(i);
            let index = taggedIndex >> ZSTD_SHORT_CACHE_TAG_BITS;
            *dst.add(i) = index;
        }
    } else {
        core::ptr::copy_nonoverlapping(src, dst, tableSize);
    }
}

unsafe fn ZSTD_resetCCtx_byCopyingCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: BufferedPolicy,
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

    // copy tables
    let chainSize =
        if ZSTD_allocateChainTable((*cdict_cParams).strategy, (*cdict).useRowMatchFinder, false) {
            1 << (*cdict_cParams).chainLog
        } else {
            0
        };
    let hSize = 1 << (*cdict_cParams).hashLog;
    ZSTD_copyCDictTableIntoCCtx(
        (*cctx).blockState.matchState.hashTable,
        (*cdict).matchState.hashTable,
        hSize,
        cdict_cParams,
    );
    // Do not copy cdict's chainTable if cctx has parameters such that it would not use chainTable
    if ZSTD_allocateChainTable(
        (*cctx).appliedParams.cParams.strategy,
        (*cctx).appliedParams.useRowMatchFinder,
        false,
    ) {
        ZSTD_copyCDictTableIntoCCtx(
            (*cctx).blockState.matchState.chainTable,
            (*cdict).matchState.chainTable,
            chainSize,
            cdict_cParams,
        );
    }
    // copy tag table
    if ZSTD_rowMatchFinderUsed((*cdict_cParams).strategy, (*cdict).useRowMatchFinder) {
        let tagTableSize = hSize;
        core::ptr::copy_nonoverlapping(
            (*cdict).matchState.tagTable,
            (*cctx).blockState.matchState.tagTable,
            tagTableSize,
        );
        (*cctx).blockState.matchState.hashSalt = (*cdict).matchState.hashSalt;
    }

    let h3log = (*cctx).blockState.matchState.hashLog3;
    let h3Size = if h3log != 0 {
        (1 as size_t) << h3log
    } else {
        0
    };
    ptr::write_bytes(
        (*cctx).blockState.matchState.hashTable3 as *mut u8,
        0,
        h3Size.wrapping_mul(size_of::<u32>()),
    );

    ZSTD_cwksp_mark_tables_clean(&mut (*cctx).workspace);

    // copy dictionary offsets
    let srcMatchState: *const ZSTD_MatchState_t = &(*cdict).matchState;
    let dstMatchState: &mut ZSTD_MatchState_t = &mut (*cctx).blockState.matchState;
    dstMatchState.window = (*srcMatchState).window;
    dstMatchState.nextToUpdate = (*srcMatchState).nextToUpdate;
    dstMatchState.loadedDictEnd = (*srcMatchState).loadedDictEnd;

    (*cctx).dictID = (*cdict).dictID;
    (*cctx).dictContentSize = (*cdict).dictContentSize;

    // copy block state
    core::ptr::copy_nonoverlapping(
        &raw const (*cdict).cBlockState,
        (*cctx).blockState.prevCBlock,
        1,
    );

    0
}

/// We have a choice between copying the dictionary context into the working context,
/// or referencing the dictionary context from the working context in-place.
/// We decide here which strategy to use.
unsafe fn ZSTD_resetCCtx_usingCDict(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    params: &ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: BufferedPolicy,
) -> size_t {
    if ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) {
        ZSTD_resetCCtx_byAttachingCDict(cctx, cdict, *params, pledgedSrcSize, zbuff)
    } else {
        ZSTD_resetCCtx_byCopyingCDict(cctx, cdict, *params, pledgedSrcSize, zbuff)
    }
}

/// Duplicate an existing context `srcCCtx` into another one `dstCCtx`.
/// Only works during stage `CompressionStage::Init` (i.e. after creation, but
/// before first call to `ZSTD_compressContinue`).
/// The "context", in this case, refers to the hash and chain tables,
/// entropy tables, and dictionary references.
/// `windowLog` value is enforced if != 0, otherwise value is copied from srcCCtx.
///
/// # Returns
///
/// - 0
/// - or an error code
unsafe fn ZSTD_copyCCtx_internal(
    dstCCtx: *mut ZSTD_CCtx,
    srcCCtx: *const ZSTD_CCtx,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: u64,
    zbuff: BufferedPolicy,
) -> size_t {
    if (*srcCCtx).stage != CompressionStage::Init {
        return Error::stage_wrong.to_error_code();
    }
    (*dstCCtx).customMem = (*srcCCtx).customMem;

    let mut params = (*dstCCtx).requestedParams;
    // Copy only compression parameters related to tables.
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

    // copy tables
    let chainSize = if ZSTD_allocateChainTable(
        (*srcCCtx).appliedParams.cParams.strategy,
        (*srcCCtx).appliedParams.useRowMatchFinder,
        false,
    ) {
        (1 as size_t) << (*srcCCtx).appliedParams.cParams.chainLog
    } else {
        0
    };
    let hSize = (1 as size_t) << (*srcCCtx).appliedParams.cParams.hashLog;
    let h3log = (*srcCCtx).blockState.matchState.hashLog3;
    let h3Size = if h3log != 0 {
        (1 as size_t) << h3log
    } else {
        0
    };
    core::ptr::copy_nonoverlapping(
        (*srcCCtx).blockState.matchState.hashTable,
        (*dstCCtx).blockState.matchState.hashTable,
        hSize,
    );
    core::ptr::copy_nonoverlapping(
        (*srcCCtx).blockState.matchState.chainTable,
        (*dstCCtx).blockState.matchState.chainTable,
        chainSize,
    );
    core::ptr::copy_nonoverlapping(
        (*srcCCtx).blockState.matchState.hashTable3,
        (*dstCCtx).blockState.matchState.hashTable3,
        h3Size,
    );

    ZSTD_cwksp_mark_tables_clean(&mut (*dstCCtx).workspace);

    // copy dictionary offsets
    let srcMatchState: *const ZSTD_MatchState_t = &(*srcCCtx).blockState.matchState;
    let dstMatchState: &mut ZSTD_MatchState_t = &mut (*dstCCtx).blockState.matchState;
    dstMatchState.window = (*srcMatchState).window;
    dstMatchState.nextToUpdate = (*srcMatchState).nextToUpdate;
    dstMatchState.loadedDictEnd = (*srcMatchState).loadedDictEnd;

    (*dstCCtx).dictID = (*srcCCtx).dictID;
    (*dstCCtx).dictContentSize = (*srcCCtx).dictContentSize;

    // copy block state
    core::ptr::copy_nonoverlapping(
        (*srcCCtx).blockState.prevCBlock,
        (*dstCCtx).blockState.prevCBlock,
        1,
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
    fParams.contentSizeFlag = (pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN) as core::ffi::c_int;

    ZSTD_copyCCtx_internal(dstCCtx, srcCCtx, fParams, pledgedSrcSize, zbuff)
}

pub const ZSTD_ROWSIZE: core::ffi::c_int = 16;

/// Reduce table indexes by `reducerValue`, or squash to zero.
/// PreserveMark preserves "unsorted mark" for btlazy2 strategy.
/// It must be set to a clear 0/1 value, to remove branch during inlining.
/// Presume table size is a multiple of ZSTD_ROWSIZE to help auto-vectorization.
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
    // Protect special index values < ZSTD_WINDOW_START_INDEX.
    let reducerThreshold = reducerValue.wrapping_add(ZSTD_WINDOW_START_INDEX as u32);

    rowNb = 0;
    while rowNb < nbRows {
        for _column in 0..ZSTD_ROWSIZE {
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

/// Rescale all indexes to avoid future overflow (indexes are U32).
unsafe fn ZSTD_reduceIndex(
    ms: &mut ZSTD_MatchState_t,
    params: &ZSTD_CCtx_params,
    reducerValue: u32,
) {
    let hSize = 1 << params.cParams.hashLog;
    ZSTD_reduceTable(ms.hashTable, hSize, reducerValue);

    if ZSTD_allocateChainTable(
        params.cParams.strategy,
        params.useRowMatchFinder,
        ms.dedicatedDictSearch != 0,
    ) {
        let chainSize = 1 << params.cParams.chainLog;
        if params.cParams.strategy == ZSTD_btlazy2 {
            ZSTD_reduceTable_btlazy2(ms.chainTable, chainSize, reducerValue);
        } else {
            ZSTD_reduceTable(ms.chainTable, chainSize, reducerValue);
        }
    }

    if ms.hashLog3 != 0 {
        let h3Size = 1 << ms.hashLog3;
        ZSTD_reduceTable(ms.hashTable3, h3Size, reducerValue);
    }
}

/// See doc/zstd_compression_format.md for detailed format description
pub unsafe fn ZSTD_seqToCodes(seqStorePtr: *const SeqStore_t) -> bool {
    let sequences: *const SeqDef = (*seqStorePtr).sequencesStart;
    let llCodeTable = (*seqStorePtr).llCode;
    let ofCodeTable = (*seqStorePtr).ofCode;
    let mlCodeTable = (*seqStorePtr).mlCode;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart)
        as core::ffi::c_long as u32;
    let mut longOffsets = false;
    for u in 0..nbSeq {
        let llv = (*sequences.offset(u as isize)).litLength as u32;
        let ofCode = ZSTD_highbit32((*sequences.offset(u as isize)).offBase);
        let mlv = (*sequences.offset(u as isize)).mlBase as u32;
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
            longOffsets = true;
        }
    }
    if (*seqStorePtr).longLengthType == LongLengthType::Literal {
        *llCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxLL;
    }
    if (*seqStorePtr).longLengthType == LongLengthType::Match {
        *mlCodeTable.offset((*seqStorePtr).longLengthPos as isize) = MaxML;
    }
    longOffsets
}

/// Returns whether the target compressed block size param is being used.
/// If used, compression will do best effort to make a compressed block size to be around
/// targetCBlockSize.
fn ZSTD_useTargetCBlockSize(cctxParams: &ZSTD_CCtx_params) -> bool {
    cctxParams.targetCBlockSize != 0
}

/// Returns whether the block splitting param is being used.
/// If used, compression will do best effort to split a block in order to improve compression ratio.
/// At the time this function is called, the parameter must be finalized.
fn ZSTD_blockSplitterEnabled(cctxParams: &ZSTD_CCtx_params) -> bool {
    cctxParams.postBlockSplitter == ParamSwitch::Enable
}

/// Returns a ZSTD_symbolEncodingTypeStats_t, or a zstd error code in the `size` field.
/// Modifies `nextEntropy` to have the appropriate values as a side effect.
/// nbSeq must be greater than 0.
///
/// entropyWkspSize must be of size at least ENTROPY_WORKSPACE_SIZE - (MaxSeq + 1)*sizeof(U32)
unsafe fn ZSTD_buildSequencesStatistics(
    seqStorePtr: *const SeqStore_t,
    nbSeq: size_t,
    prevEntropy: &ZSTD_fseCTables_t,
    nextEntropy: &mut ZSTD_fseCTables_t,
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
    let ofCodeTable: *const u8 = (*seqStorePtr).ofCode;
    let llCodeTable: *const u8 = (*seqStorePtr).llCode;
    let mlCodeTable: *const u8 = (*seqStorePtr).mlCode;

    // convert length/distances into codes
    let mut stats = ZSTD_symbolEncodingTypeStats_t {
        longOffsets: ZSTD_seqToCodes(seqStorePtr),
        ..Default::default()
    };

    // build CTable for Literal Lengths
    let mut max = MaxLL;
    let mostFrequent = HIST_countFast_wksp(
        countWorkspace,
        &mut max,
        llCodeTable as *const core::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    nextEntropy.litlength_repeatMode = prevEntropy.litlength_repeatMode;
    stats.LLtype = ZSTD_selectEncodingType(
        &mut nextEntropy.litlength_repeatMode,
        countWorkspace,
        max,
        mostFrequent,
        nbSeq,
        LLFSELog,
        &prevEntropy.litlengthCTable,
        &LL_defaultNorm,
        LL_defaultNormLog,
        DefaultPolicy::Allowed,
        strategy,
    );
    let countSize = ZSTD_buildCTable(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        &mut nextEntropy.litlengthCTable,
        LLFSELog,
        stats.LLtype,
        countWorkspace,
        max,
        llCodeTable,
        nbSeq,
        &LL_defaultNorm,
        LL_defaultNormLog,
        MaxLL,
        &prevEntropy.litlengthCTable,
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize) {
        stats.size = countSize;
        return stats;
    }
    if stats.LLtype == SymbolEncodingType::Compressed {
        stats.lastCountSize = countSize;
    }
    op = op.add(countSize);

    // build CTable for Offsets
    let mut max_0 = MaxOff;
    let mostFrequent_0 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_0,
        ofCodeTable as *const core::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    // We can only use the basic table if max <= DefaultMaxOff, otherwise the offsets are too large
    let defaultPolicy = if max_0 <= DefaultMaxOff {
        DefaultPolicy::Allowed
    } else {
        DefaultPolicy::Disallowed
    };
    nextEntropy.offcode_repeatMode = prevEntropy.offcode_repeatMode;
    stats.Offtype = ZSTD_selectEncodingType(
        &mut nextEntropy.offcode_repeatMode,
        countWorkspace,
        max_0,
        mostFrequent_0,
        nbSeq,
        OffFSELog,
        &prevEntropy.offcodeCTable,
        &OF_defaultNorm,
        OF_defaultNormLog,
        defaultPolicy,
        strategy,
    );
    let countSize_0 = ZSTD_buildCTable(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        &mut nextEntropy.offcodeCTable,
        OffFSELog,
        stats.Offtype,
        countWorkspace,
        max_0,
        ofCodeTable,
        nbSeq,
        &OF_defaultNorm,
        OF_defaultNormLog,
        DefaultMaxOff,
        &prevEntropy.offcodeCTable,
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_0) {
        stats.size = countSize_0;
        return stats;
    }
    if stats.Offtype == SymbolEncodingType::Compressed {
        stats.lastCountSize = countSize_0;
    }
    op = op.add(countSize_0);

    // build CTable for MatchLengths
    let mut max_1 = MaxML;
    let mostFrequent_1 = HIST_countFast_wksp(
        countWorkspace,
        &mut max_1,
        mlCodeTable as *const core::ffi::c_void,
        nbSeq,
        entropyWorkspace,
        entropyWkspSize,
    );
    nextEntropy.matchlength_repeatMode = prevEntropy.matchlength_repeatMode;
    stats.MLtype = ZSTD_selectEncodingType(
        &mut nextEntropy.matchlength_repeatMode,
        countWorkspace,
        max_1,
        mostFrequent_1,
        nbSeq,
        MLFSELog,
        &prevEntropy.matchlengthCTable,
        &ML_defaultNorm,
        ML_defaultNormLog,
        DefaultPolicy::Allowed,
        strategy,
    );
    let countSize_1 = ZSTD_buildCTable(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        &mut nextEntropy.matchlengthCTable,
        MLFSELog,
        stats.MLtype,
        countWorkspace,
        max_1,
        mlCodeTable,
        nbSeq,
        &ML_defaultNorm,
        ML_defaultNormLog,
        MaxML,
        &prevEntropy.matchlengthCTable,
        entropyWorkspace,
        entropyWkspSize,
    );
    if ERR_isError(countSize_1) {
        stats.size = countSize_1;
        return stats;
    }
    if stats.MLtype == SymbolEncodingType::Compressed {
        stats.lastCountSize = countSize_1;
    }
    op = op.add(countSize_1);

    stats.size = op.offset_from_unsigned(ostart);

    stats
}

pub const SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO: core::ffi::c_int = 20;

/// Compresses both literals and sequences.
///
/// # Returns
///
/// - The compressed size of block
/// - Or a zstd error
#[inline]
unsafe fn ZSTD_entropyCompressSeqStore_internal(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    literals: *const core::ffi::c_void,
    litSize: size_t,
    seqStorePtr: *const SeqStore_t,
    prevEntropy: &ZSTD_entropyCTables_t,
    nextEntropy: &mut ZSTD_entropyCTables_t,
    cctxParams: &ZSTD_CCtx_params,
    mut entropyWorkspace: *mut core::ffi::c_void,
    mut entropyWkspSize: size_t,
    bmi2: core::ffi::c_int,
) -> size_t {
    let strategy = cctxParams.cParams.strategy;
    let count = entropyWorkspace as *mut core::ffi::c_uint;
    let sequences: *const SeqDef = (*seqStorePtr).sequencesStart;
    let nbSeq = ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart) as size_t;
    let ofCodeTable: *const u8 = (*seqStorePtr).ofCode;
    let llCodeTable: *const u8 = (*seqStorePtr).llCode;
    let mlCodeTable: *const u8 = (*seqStorePtr).mlCode;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstCapacity);
    let mut op = ostart;
    let mut lastCountSize: size_t = 0;
    let mut longOffsets = false;

    entropyWorkspace = count.add(MaxSeq + 1) as *mut core::ffi::c_void;
    entropyWkspSize = (entropyWkspSize as size_t)
        .wrapping_sub((MaxSeq + 1).wrapping_mul(size_of::<core::ffi::c_uint>()));

    // Compress literals
    let numSequences =
        ((*seqStorePtr).sequences).offset_from((*seqStorePtr).sequencesStart) as size_t;
    // Base suspicion of uncompressibility on ratio of literals to sequences
    let suspectUncompressible = (numSequences == 0
        || litSize / numSequences >= SUSPECT_UNCOMPRESSIBLE_LITERAL_RATIO as size_t)
        as core::ffi::c_int;

    let cSize = ZSTD_compressLiterals(
        op as *mut core::ffi::c_void,
        dstCapacity,
        literals,
        litSize,
        entropyWorkspace,
        entropyWkspSize,
        &prevEntropy.huf,
        &mut nextEntropy.huf,
        cctxParams.cParams.strategy,
        ZSTD_literalsCompressionIsDisabled(cctxParams),
        suspectUncompressible,
        bmi2,
    );
    let err_code = cSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    op = op.add(cSize);

    // Sequences Header
    if (oend.offset_from(op) as core::ffi::c_long) < (3 + 1) as core::ffi::c_long {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if nbSeq < 128 {
        *op = nbSeq as u8;
        op = op.add(1);
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
        // Copy the old tables over as if we repeated them
        core::ptr::copy_nonoverlapping(&raw const prevEntropy.fse, &raw mut nextEntropy.fse, 1);
        return op.offset_from_unsigned(ostart);
    }

    let seqHead = op;
    op = op.add(1);
    // build stats for sequences
    let stats = ZSTD_buildSequencesStatistics(
        seqStorePtr,
        nbSeq,
        &prevEntropy.fse,
        &mut nextEntropy.fse,
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
    *seqHead = ((stats.LLtype as u32) << 6)
        .wrapping_add((stats.Offtype as u32) << 4)
        .wrapping_add((stats.MLtype as u32) << 2) as u8;
    lastCountSize = stats.lastCountSize;
    op = op.add(stats.size);
    longOffsets = stats.longOffsets;

    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        &nextEntropy.fse.matchlengthCTable,
        mlCodeTable,
        &nextEntropy.fse.offcodeCTable,
        ofCodeTable,
        &nextEntropy.fse.litlengthCTable,
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
    // zstd versions <= 1.3.4 mistakenly report corruption when
    // FSE_readNCount() receives a buffer < 4 bytes.
    // Fixed by https://github.com/facebook/zstd/pull/1146.
    // This can happen when the last SymbolEncodingType::Compressed table present is 2
    // bytes and the bitstream is only one byte.
    // In this exceedingly rare case, we will simply emit an uncompressed
    // block, since it isn't worth optimizing.
    if lastCountSize != 0 && lastCountSize.wrapping_add(bitstreamSize) < 4 {
        // lastCountSize >= 2 && bitstreamSize > 0 ==> lastCountSize == 3
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
    prevEntropy: &ZSTD_entropyCTables_t,
    nextEntropy: &mut ZSTD_entropyCTables_t,
    cctxParams: &ZSTD_CCtx_params,
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
    // When srcSize <= dstCapacity, there is enough space to write a raw uncompressed block.
    // Since we ran out of space, block must be not compressible, so fall back to raw uncompressed block.
    if (cSize == Error::dstSize_tooSmall.to_error_code()) as core::ffi::c_int
        & (blockSize <= dstCapacity) as core::ffi::c_int
        != 0
    {
        return 0; // block not compressed
    }
    let err_code = cSize;
    if ERR_isError(err_code) {
        return err_code;
    }

    // Check compressibility
    let maxCSize = blockSize.wrapping_sub(ZSTD_minGain(blockSize, cctxParams.cParams.strategy));
    if cSize >= maxCSize {
        return 0;
    }
    cSize
}

unsafe fn ZSTD_entropyCompressSeqStore(
    seqStorePtr: *const SeqStore_t,
    prevEntropy: &ZSTD_entropyCTables_t,
    nextEntropy: &mut ZSTD_entropyCTables_t,
    cctxParams: &ZSTD_CCtx_params,
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

/// Not static, but internal use only (used by long distance matcher).
///
/// Assumption: strat is a valid strategy
pub fn ZSTD_selectBlockCompressor(
    strat: ZSTD_strategy,
    useRowMatchFinder: ParamSwitch,
    dictMode: DictMode,
) -> Option<ZSTD_BlockCompressor_f> {
    if ZSTD_rowMatchFinderUsed(strat, useRowMatchFinder) {
        match dictMode {
            DictMode::NoDict => match strat {
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy_row),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy_row),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2_row),
                _ => unreachable!(),
            },
            DictMode::ExtDict => match strat {
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy_extDict_row),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy_extDict_row),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2_extDict_row),
                _ => unreachable!(),
            },
            DictMode::DictMatchState => match strat {
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy_dictMatchState_row),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy_dictMatchState_row),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2_dictMatchState_row),
                _ => unreachable!(),
            },
            DictMode::DedicatedDictSearch => match strat {
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy_dedicatedDictSearch_row),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy_dedicatedDictSearch_row),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2_dedicatedDictSearch_row),
                _ => unreachable!(),
            },
        }
    } else {
        match dictMode {
            DictMode::NoDict => match strat {
                0 => Some(ZSTD_compressBlock_fast),
                ZSTD_fast => Some(ZSTD_compressBlock_fast),
                ZSTD_dfast => Some(ZSTD_compressBlock_doubleFast),
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2),
                ZSTD_btlazy2 => Some(ZSTD_compressBlock_btlazy2),
                ZSTD_btopt => Some(ZSTD_compressBlock_btopt),
                ZSTD_btultra => Some(ZSTD_compressBlock_btultra),
                ZSTD_btultra2 => Some(ZSTD_compressBlock_btultra2),
                _ => unreachable!(),
            },
            DictMode::ExtDict => match strat {
                0 => Some(ZSTD_compressBlock_fast_extDict),
                ZSTD_fast => Some(ZSTD_compressBlock_fast_extDict),
                ZSTD_dfast => Some(ZSTD_compressBlock_doubleFast_extDict),
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy_extDict),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy_extDict),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2_extDict),
                ZSTD_btlazy2 => Some(ZSTD_compressBlock_btlazy2_extDict),
                ZSTD_btopt => Some(ZSTD_compressBlock_btopt_extDict),
                ZSTD_btultra => Some(ZSTD_compressBlock_btultra_extDict),
                ZSTD_btultra2 => Some(ZSTD_compressBlock_btultra_extDict),
                _ => unreachable!(),
            },
            DictMode::DictMatchState => match strat {
                0 => Some(ZSTD_compressBlock_fast_dictMatchState),
                ZSTD_fast => Some(ZSTD_compressBlock_fast_dictMatchState),
                ZSTD_dfast => Some(ZSTD_compressBlock_doubleFast_dictMatchState),
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy_dictMatchState),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy_dictMatchState),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2_dictMatchState),
                ZSTD_btlazy2 => Some(ZSTD_compressBlock_btlazy2_dictMatchState),
                ZSTD_btopt => Some(ZSTD_compressBlock_btopt_dictMatchState),
                ZSTD_btultra => Some(ZSTD_compressBlock_btultra_dictMatchState),
                ZSTD_btultra2 => Some(ZSTD_compressBlock_btultra_dictMatchState),
                _ => unreachable!(),
            },
            DictMode::DedicatedDictSearch => match strat {
                0 => None,
                ZSTD_fast => None,
                ZSTD_dfast => None,
                ZSTD_greedy => Some(ZSTD_compressBlock_greedy_dedicatedDictSearch),
                ZSTD_lazy => Some(ZSTD_compressBlock_lazy_dedicatedDictSearch),
                ZSTD_lazy2 => Some(ZSTD_compressBlock_lazy2_dedicatedDictSearch),
                ZSTD_btlazy2 => None,
                ZSTD_btopt => None,
                ZSTD_btultra => None,
                ZSTD_btultra2 => None,
                _ => unreachable!(),
            },
        }
    }
}

unsafe fn ZSTD_storeLastLiterals(
    seqStorePtr: &mut SeqStore_t,
    anchor: *const u8,
    lastLLSize: size_t,
) {
    core::ptr::copy_nonoverlapping(anchor, seqStorePtr.lit, lastLLSize);
    seqStorePtr.lit = (seqStorePtr.lit).add(lastLLSize);
}

pub fn ZSTD_resetSeqStore(ssPtr: &mut SeqStore_t) {
    ssPtr.lit = ssPtr.litStart;
    ssPtr.sequences = ssPtr.sequencesStart;
    ssPtr.longLengthType = LongLengthType::None;
}

/// Validates and post-processes sequences obtained through the external matchfinder API:
///   - Checks whether nbExternalSeqs represents an error condition.
///   - Appends a block delimiter to outSeqs if one is not already present.
///     See zstd.h for context regarding block delimiters.
///
/// # Returns
///
/// - The number of sequences after post-processing
/// - Or an error code
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
            size_of::<ZSTD_Sequence>(),
        );
        return 1;
    }

    let lastSeq = *outSeqs.add(nbExternalSeqs.wrapping_sub(1));
    // We can return early if lastSeq is already a block delimiter.
    if lastSeq.offset == 0 && lastSeq.matchLength == 0 {
        return nbExternalSeqs;
    }

    // This error condition is only possible if the external matchfinder
    // produced an invalid parse, by definition of ZSTD_sequenceBound().
    if nbExternalSeqs == outSeqsCapacity {
        return Error::sequenceProducer_failed.to_error_code();
    }

    // lastSeq is not a block delimiter, so we need to append one.
    ptr::write_bytes(
        &mut *outSeqs.add(nbExternalSeqs) as *mut ZSTD_Sequence as *mut u8,
        0,
        size_of::<ZSTD_Sequence>(),
    );
    nbExternalSeqs.wrapping_add(1)
}

/// Returns sum(litLen) + sum(matchLen) + lastLits for *seqBuf*.
/// Similar to another function in zstd_compress.c (determine_blockSize),
/// except it doesn't check for a block delimiter to end summation.
/// Removing the early exit allows the compiler to auto-vectorize.
/// This function can be deleted and replaced by determine_blockSize after we resolve issue #3456.
unsafe fn ZSTD_fastSequenceLengthSum(seqBuf: *const ZSTD_Sequence, seqBufSize: size_t) -> size_t {
    let mut matchLenSum: size_t = 0;
    let mut litLenSum: size_t = 0;
    for i in 0..seqBufSize {
        litLenSum = litLenSum.wrapping_add((*seqBuf.add(i)).litLength as size_t);
        matchLenSum = matchLenSum.wrapping_add((*seqBuf.add(i)).matchLength as size_t);
    }
    litLenSum.wrapping_add(matchLenSum)
}

/// Validate sequences produced by a block compressor.
unsafe fn ZSTD_validateSeqStore(seqStore: &SeqStore_t, cParams: &ZSTD_compressionParameters) {
    let matchLenLowerBound = match cParams.minMatch {
        3 => 3,
        _ => 4,
    };

    let start = seqStore.sequences;
    let end = seqStore.sequences;

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
        if (*zc).appliedParams.cParams.strategy >= ZSTD_btopt {
            ZSTD_ldm_skipRawSeqStoreBytes(&mut (*zc).externSeqStore, srcSize);
        } else {
            ZSTD_ldm_skipSequences(
                &mut (*zc).externSeqStore,
                srcSize,
                (*zc).appliedParams.cParams.minMatch,
            );
        }
        // don't even attempt compression below a certain srcSize
        return BuildSeqStore::NoCompress as size_t;
    }
    ZSTD_resetSeqStore(&mut (*zc).seqStore);
    // required for optimal parser to read stats from dictionary
    ms.opt.symbolCosts = &mut (*(*zc).blockState.prevCBlock).entropy;
    // tell the optimal parser how we expect to compress literals
    ms.opt.literalCompressionMode = (*zc).appliedParams.literalCompressionMode;

    // limited update after a very long match
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

    // select and store sequences
    let dictMode = ZSTD_matchState_dictMode(ms);
    let mut lastLLSize: size_t = 0;
    for i in 0..ZSTD_REP_NUM {
        (*(*zc).blockState.nextCBlock).rep[i as usize] =
            (*(*zc).blockState.prevCBlock).rep[i as usize];
    }
    if (*zc).externSeqStore.pos < (*zc).externSeqStore.size {
        if ZSTD_hasExtSeqProd(&(*zc).appliedParams) {
            return Error::parameter_combination_unsupported.to_error_code();
        }
        lastLLSize = ZSTD_ldm_blockCompress(
            &mut (*zc).externSeqStore,
            ms,
            &mut (*zc).seqStore,
            &mut (*(*zc).blockState.nextCBlock).rep,
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
    } else if (*zc).appliedParams.ldmParams.enableLdm == ParamSwitch::Enable {
        let mut ldmSeqStore = RawSeqStore_t::new();
        if ZSTD_hasExtSeqProd(&(*zc).appliedParams) {
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
            &mut (*(*zc).blockState.nextCBlock).rep,
            (*zc).appliedParams.useRowMatchFinder,
            src,
            srcSize,
        );
    } else if ZSTD_hasExtSeqProd(&(*zc).appliedParams) {
        let windowSize = 1 << (*zc).appliedParams.cParams.windowLog;

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

        // Return early if there is no error, since we don't need to worry about last literals
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
            return BuildSeqStore::Compress as size_t;
        }

        // Propagate the error if fallback is disabled
        if (*zc).appliedParams.enableMatchFinderFallback == 0 {
            return nbPostProcessedSeqs;
        }

        // Fallback to software matchfinder
        let blockCompressor = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        ms.ldmSeqStore = core::ptr::null();
        lastLLSize = blockCompressor.unwrap_unchecked()(
            ms,
            &mut (*zc).seqStore,
            &mut (*(*zc).blockState.nextCBlock).rep,
            src,
            srcSize,
        );
    } else {
        // not long range mode and no external matchfinder
        let blockCompressor_0 = ZSTD_selectBlockCompressor(
            (*zc).appliedParams.cParams.strategy,
            (*zc).appliedParams.useRowMatchFinder,
            dictMode,
        );
        ms.ldmSeqStore = core::ptr::null();
        lastLLSize = blockCompressor_0.unwrap_unchecked()(
            ms,
            &mut (*zc).seqStore,
            &mut (*(*zc).blockState.nextCBlock).rep,
            src,
            srcSize,
        );
    }

    let lastLiterals = (src as *const u8).add(srcSize).sub(lastLLSize as usize);
    ZSTD_storeLastLiterals(&mut (*zc).seqStore, lastLiterals, lastLLSize);

    ZSTD_validateSeqStore(&(*zc).seqStore, &(*zc).appliedParams.cParams);
    BuildSeqStore::Compress as size_t
}

unsafe fn ZSTD_copyBlockSequences(
    seqCollector: &mut SeqCollector,
    seqStore: *const SeqStore_t,
    prevRepcodes: &[u32; ZSTD_REP_NUM as usize],
) -> size_t {
    let inSeqs: *const SeqDef = (*seqStore).sequencesStart;
    let nbInSequences = ((*seqStore).sequences).offset_from_unsigned(inSeqs);
    let nbInLiterals = ((*seqStore).lit).offset_from((*seqStore).litStart) as size_t;

    let outSeqs = if seqCollector.seqIndex == 0 {
        seqCollector.seqStart
    } else {
        (seqCollector.seqStart).add(seqCollector.seqIndex)
    };
    let nbOutSequences = nbInSequences.wrapping_add(1);
    let mut nbOutLiterals = 0usize;
    let mut repcodes = repcodes_s { rep: [0; 3] };

    if nbOutSequences > (seqCollector.maxSequences).wrapping_sub(seqCollector.seqIndex) {
        return Error::dstSize_tooSmall.to_error_code();
    }

    repcodes.rep = *prevRepcodes;
    for i in 0..nbInSequences {
        let mut rawOffset: u32 = 0;
        (*outSeqs.add(i)).litLength = (*inSeqs.add(i)).litLength as core::ffi::c_uint;
        (*outSeqs.add(i)).matchLength =
            ((*inSeqs.add(i)).mlBase as core::ffi::c_int + MINMATCH) as core::ffi::c_uint;
        (*outSeqs.add(i)).rep = 0;

        // Handle the possible single length >= 64K
        // There can only be one because we add MINMATCH to every match length,
        // and blocks are at most 128K.
        if i == (*seqStore).longLengthPos as size_t {
            if (*seqStore).longLengthType == LongLengthType::Literal {
                let fresh4 = &mut (*outSeqs.add(i)).litLength;
                *fresh4 = (*fresh4).wrapping_add(0x10000);
            } else if (*seqStore).longLengthType == LongLengthType::Match {
                let fresh5 = &mut (*outSeqs.add(i)).matchLength;
                *fresh5 = (*fresh5).wrapping_add(0x10000);
            }
        }

        // Determine the raw offset given the offBase, which may be a repcode.
        if 1 <= (*inSeqs.add(i)).offBase && (*inSeqs.add(i)).offBase <= ZSTD_REP_NUM as u32 {
            let repcode = (*inSeqs.add(i)).offBase;
            (*outSeqs.add(i)).rep = repcode;
            if (*outSeqs.add(i)).litLength != 0 {
                rawOffset = repcodes.rep[repcode.wrapping_sub(1) as usize];
            } else if repcode == 3 {
                rawOffset = repcodes.rep[0].wrapping_sub(1);
            } else {
                rawOffset = repcodes.rep[repcode as usize];
            }
        } else {
            rawOffset = ((*inSeqs.add(i)).offBase).wrapping_sub(ZSTD_REP_NUM as u32);
        }
        (*outSeqs.add(i)).offset = rawOffset;

        // Update repcode history for the sequence
        ZSTD_updateRep(
            &mut repcodes.rep,
            (*inSeqs.add(i)).offBase,
            ((*inSeqs.add(i)).litLength as core::ffi::c_int == 0) as core::ffi::c_int as u32,
        );
        nbOutLiterals = nbOutLiterals.wrapping_add((*outSeqs.add(i)).litLength as size_t);
    }

    // Insert last literals (if any exist) in the block as a sequence with ml == off == 0.
    // If there are no last literals, then we'll emit (of: 0, ml: 0, ll: 0), which is a marker
    // for the block boundary, according to the API.
    let lastLLSize = nbInLiterals.wrapping_sub(nbOutLiterals);
    (*outSeqs.add(nbInSequences)).litLength = lastLLSize as u32;
    (*outSeqs.add(nbInSequences)).matchLength = 0;
    (*outSeqs.add(nbInSequences)).offset = 0;

    seqCollector.seqIndex = (seqCollector.seqIndex).wrapping_add(nbOutSequences);

    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sequenceBound))]
pub extern "C" fn ZSTD_sequenceBound(srcSize: size_t) -> size_t {
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
    let mut out = 0usize;
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

/// Unrolled loop to read four size_ts of input at a time. Returns `true` if the input is RLE.
unsafe fn ZSTD_isRLE(src: *const u8, length: size_t) -> bool {
    let ip = src;
    let value = *ip;
    let valueST = (value as u64 as core::ffi::c_ulonglong)
        .wrapping_mul(0x101010101010101 as core::ffi::c_ulonglong) as size_t;
    let unrollSize = size_of::<size_t>().wrapping_mul(4);
    let unrollMask = unrollSize.wrapping_sub(1);
    let prefixLength = length & unrollMask;
    let mut i: size_t = 0;

    if length == 1 {
        return true;
    }

    // Check if prefix is RLE first before using unrolled loop
    if prefixLength != 0
        && ZSTD_count(ip.add(1), ip, ip.add(prefixLength)) != prefixLength.wrapping_sub(1)
    {
        return false;
    }

    i = prefixLength;
    while i != length {
        for u in (0..unrollSize).step_by(size_of::<size_t>()) {
            if MEM_readST(ip.add(i).add(u) as *const core::ffi::c_void) != valueST {
                return false;
            }
        }
        i = i.wrapping_add(unrollSize);
    }

    true
}

/// Returns true if the given block may be RLE.
/// This is just a heuristic based on the compressibility.
/// It may return both false positives and false negatives.
unsafe fn ZSTD_maybeRLE(seqStore: &SeqStore_t) -> bool {
    let nbSeqs = (seqStore.sequences).offset_from(seqStore.sequencesStart) as size_t;
    let nbLits = (seqStore.lit).offset_from(seqStore.litStart) as size_t;

    nbSeqs < 4 && nbLits < 10
}

fn ZSTD_blockState_confirmRepcodesAndEntropyTables(bs: &mut ZSTD_blockState_t) {
    core::mem::swap(&mut bs.prevCBlock, &mut bs.nextCBlock);
}

/// Writes the block header
unsafe fn writeBlockHeader(
    op: *mut core::ffi::c_void,
    cSize: size_t,
    blockSize: size_t,
    lastBlock: u32,
) {
    let cBlockHeader = if cSize == 1 {
        lastBlock
            .wrapping_add((BlockType::Rle as u32) << 1)
            .wrapping_add((blockSize << 3) as u32)
    } else {
        lastBlock
            .wrapping_add((BlockType::Compressed as u32) << 1)
            .wrapping_add((cSize << 3) as u32)
    };
    MEM_writeLE24(op, cBlockHeader);
}

/// Builds entropy for the literals.
/// Stores literals block type (raw, rle, compressed, repeat) and
/// huffman description table to hufMetadata.
/// Requires ENTROPY_WORKSPACE_SIZE workspace
///
/// # Returns
///
/// - The size of huffman description table
/// - Or an error code
unsafe fn ZSTD_buildBlockEntropyStats_literals(
    src: *mut core::ffi::c_void,
    srcSize: size_t,
    prevHuf: &ZSTD_hufCTables_t,
    nextHuf: &mut ZSTD_hufCTables_t,
    hufMetadata: &mut ZSTD_hufCTablesMetadata_t,
    literalsCompressionIsDisabled: bool,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    hufFlags: core::ffi::c_int,
) -> size_t {
    let wkspStart = workspace as *mut u8;
    let wkspEnd = wkspStart.add(wkspSize);
    let countWkspStart = wkspStart;
    let countWksp = workspace as *mut core::ffi::c_uint;
    let countWkspSize =
        ((HUF_SYMBOLVALUE_MAX + 1) as size_t).wrapping_mul(size_of::<core::ffi::c_uint>());
    let nodeWksp = countWkspStart.add(countWkspSize);
    let nodeWkspSize = wkspEnd.offset_from_unsigned(nodeWksp);
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX_U8;
    let mut huffLog = LitHufLog;
    let mut repeat = prevHuf.repeatMode;

    // Prepare nextEntropy assuming reusing the existing table
    core::ptr::copy_nonoverlapping(prevHuf, nextHuf, 1);

    if literalsCompressionIsDisabled {
        hufMetadata.hType = SymbolEncodingType::Basic;
        return 0;
    }

    // small ? don't even attempt compression (speed opt)
    let minLitSize = (if prevHuf.repeatMode == HUF_repeat_valid {
        6
    } else {
        COMPRESS_LITERALS_SIZE_MIN
    }) as size_t;
    if srcSize <= minLitSize {
        hufMetadata.hType = SymbolEncodingType::Basic;
        return 0;
    }

    // Scan input and build symbol stats
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
        // only one literal symbol
        hufMetadata.hType = SymbolEncodingType::Rle;
        return 0;
    }
    if largest <= (srcSize >> 7).wrapping_add(4) {
        // heuristic: likely not compressible
        hufMetadata.hType = SymbolEncodingType::Basic;
        return 0;
    }

    // Validate the previous Huffman table
    if repeat == HUF_repeat_check
        && !HUF_validateCTable((prevHuf.CTable).as_ptr(), countWksp, maxSymbolValue)
    {
        repeat = HUF_repeat_none;
    }

    // Build Huffman Tree
    nextHuf.CTable.fill(0);
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        nodeWksp as *mut core::ffi::c_void,
        nodeWkspSize,
        &mut nextHuf.CTable,
        countWksp,
        hufFlags,
    );
    let maxBits = HUF_buildCTable_wksp(
        &mut nextHuf.CTable,
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
    // Build and write the CTable
    let newCSize =
        HUF_estimateCompressedSize((nextHuf.CTable).as_mut_ptr(), countWksp, maxSymbolValue);
    let hSize = HUF_writeCTable_wksp(
        (hufMetadata.hufDesBuffer).as_mut_ptr() as *mut core::ffi::c_void,
        size_of::<[u8; ZSTD_MAX_HUF_HEADER_SIZE]>(),
        &nextHuf.CTable,
        maxSymbolValue,
        huffLog,
        nodeWksp as *mut core::ffi::c_void,
        nodeWkspSize,
    );
    // Check against repeating the previous CTable
    if repeat != HUF_repeat_none {
        let oldCSize =
            HUF_estimateCompressedSize((prevHuf.CTable).as_ptr(), countWksp, maxSymbolValue);
        if oldCSize < srcSize
            && (oldCSize <= hSize.wrapping_add(newCSize) || hSize.wrapping_add(12) >= srcSize)
        {
            core::ptr::copy_nonoverlapping(prevHuf, nextHuf, 1);
            hufMetadata.hType = SymbolEncodingType::Repeat;
            return 0;
        }
    }
    if newCSize.wrapping_add(hSize) >= srcSize {
        core::ptr::copy_nonoverlapping(prevHuf, nextHuf, 1);
        hufMetadata.hType = SymbolEncodingType::Basic;
        return 0;
    }
    hufMetadata.hType = SymbolEncodingType::Compressed;
    nextHuf.repeatMode = HUF_repeat_check;

    hSize
}

pub const COMPRESS_LITERALS_SIZE_MIN: core::ffi::c_int = 63;

/// Returns a [`ZSTD_symbolEncodingTypeStats_t`] with all encoding types as [`SymbolEncodingType::Basic`],
/// and updates nextEntropy to the appropriate repeatMode.
unsafe fn ZSTD_buildDummySequencesStatistics(
    nextEntropy: &mut ZSTD_fseCTables_t,
) -> ZSTD_symbolEncodingTypeStats_t {
    nextEntropy.litlength_repeatMode = FSE_repeat_none;
    nextEntropy.offcode_repeatMode = FSE_repeat_none;
    nextEntropy.matchlength_repeatMode = FSE_repeat_none;

    ZSTD_symbolEncodingTypeStats_t::default()
}

/// Builds entropy for the sequences.
/// Stores symbol compression modes and fse table to fseMetadata.
/// Requires ENTROPY_WORKSPACE_SIZE wksp.
///
/// # Returns
/// - The size of the fse tables
/// - Or an error code
unsafe fn ZSTD_buildBlockEntropyStats_sequences(
    seqStorePtr: &SeqStore_t,
    prevEntropy: &ZSTD_fseCTables_t,
    nextEntropy: &mut ZSTD_fseCTables_t,
    cctxParams: &ZSTD_CCtx_params,
    fseMetadata: &mut ZSTD_fseCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let strategy = cctxParams.cParams.strategy;
    let nbSeq = (seqStorePtr.sequences).offset_from(seqStorePtr.sequencesStart) as size_t;
    let ostart = (fseMetadata.fseTablesBuffer).as_mut_ptr();
    let oend = ostart.add(size_of::<[u8; 133]>());
    let op = ostart;
    let countWorkspace = workspace as *mut core::ffi::c_uint;
    let entropyWorkspace = countWorkspace.add(MaxSeq + 1);
    let entropyWorkspaceSize =
        wkspSize.wrapping_sub((MaxSeq + 1).wrapping_mul(size_of::<core::ffi::c_uint>()));

    let stats = if nbSeq != 0 {
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

    fseMetadata.llType = stats.LLtype;
    fseMetadata.ofType = stats.Offtype;
    fseMetadata.mlType = stats.MLtype;
    fseMetadata.lastCountSize = stats.lastCountSize;

    stats.size
}

/// Builds entropy for the block.
/// Requires workspace size ENTROPY_WORKSPACE_SIZE
///
/// Note: also employed in superblock
///
/// # Returns
///
/// - 0 on success
/// - Or an error code
pub unsafe fn ZSTD_buildBlockEntropyStats(
    seqStorePtr: &SeqStore_t,
    prevEntropy: &ZSTD_entropyCTables_t,
    nextEntropy: &mut ZSTD_entropyCTables_t,
    cctxParams: &ZSTD_CCtx_params,
    entropyMetadata: *mut ZSTD_entropyCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let litSize = (seqStorePtr.lit).offset_from(seqStorePtr.litStart) as size_t;
    let huf_useOptDepth = (cctxParams.cParams.strategy
        >= HUF_OPTIMAL_DEPTH_THRESHOLD as core::ffi::c_uint)
        as core::ffi::c_int;
    let hufFlags = if huf_useOptDepth != 0 {
        HUF_flags_optimalDepth as core::ffi::c_int
    } else {
        0
    };

    (*entropyMetadata).hufMetadata.hufDesSize = ZSTD_buildBlockEntropyStats_literals(
        seqStorePtr.litStart as *mut core::ffi::c_void,
        litSize,
        &prevEntropy.huf,
        &mut nextEntropy.huf,
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
        &prevEntropy.fse,
        &mut nextEntropy.fse,
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

/// Returns the size estimate for the literals section (header + content) of a block
unsafe fn ZSTD_estimateBlockSize_literal(
    literals: *const u8,
    litSize: size_t,
    huf: &ZSTD_hufCTables_t,
    hufMetadata: &ZSTD_hufCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeEntropy: bool,
) -> size_t {
    let countWksp = workspace as *mut core::ffi::c_uint;
    let mut maxSymbolValue = HUF_SYMBOLVALUE_MAX_U8;
    let literalSectionHeaderSize =
        (3 + (litSize >= (1 << 10) as size_t) as core::ffi::c_int
            + (litSize >= (16 * (1 << 10)) as size_t) as core::ffi::c_int) as size_t;
    let singleStream = litSize < 256;

    if hufMetadata.hType == SymbolEncodingType::Basic {
        return litSize;
    } else if hufMetadata.hType == SymbolEncodingType::Rle {
        return 1;
    } else if hufMetadata.hType == SymbolEncodingType::Compressed
        || hufMetadata.hType == SymbolEncodingType::Repeat
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
            HUF_estimateCompressedSize((huf.CTable).as_ptr(), countWksp, maxSymbolValue);
        if writeEntropy {
            cLitSizeEstimate = cLitSizeEstimate.wrapping_add(hufMetadata.hufDesSize);
        }
        if !singleStream {
            // multi-stream huffman uses 6-byte jump table
            cLitSizeEstimate = cLitSizeEstimate.wrapping_add(6);
        }
        return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
    }

    0
}

/// Returns the size estimate for the FSE-compressed symbols (of, ml, ll) of a block
unsafe fn ZSTD_estimateBlockSize_symbolType(
    type_0: SymbolEncodingType,
    codeTable: *const u8,
    nbSeq: size_t,
    maxCode: u8,
    fseCTable: &[FSE_CTable],
    additionalBits: *const u8,
    defaultNorm: &[core::ffi::c_short],
    defaultNormLog: u32,
    defaultMax: u8,
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
    if type_0 == SymbolEncodingType::Basic {
        /* We selected this encoding type, so it must be valid. */
        assert!(max <= defaultMax);

        cSymbolTypeSizeEstimateInBits =
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, countWksp, max);
    } else if type_0 == SymbolEncodingType::Rle {
        cSymbolTypeSizeEstimateInBits = 0;
    } else if type_0 == SymbolEncodingType::Compressed || type_0 == SymbolEncodingType::Repeat {
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

/// Returns the size estimate for the sequences section (header + content) of a block
unsafe fn ZSTD_estimateBlockSize_sequences(
    ofCodeTable: *const u8,
    llCodeTable: *const u8,
    mlCodeTable: *const u8,
    nbSeq: size_t,
    fseTables: &ZSTD_fseCTables_t,
    fseMetadata: &ZSTD_fseCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeEntropy: bool,
) -> size_t {
    let sequencesSectionHeaderSize =
        (1 + 1
            + (nbSeq >= 128) as core::ffi::c_int
            + (nbSeq >= LONGNBSEQ as size_t) as core::ffi::c_int) as size_t;
    let mut cSeqSizeEstimate = 0usize;

    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateBlockSize_symbolType(
        fseMetadata.ofType,
        ofCodeTable,
        nbSeq,
        MaxOff,
        &fseTables.offcodeCTable,
        core::ptr::null(),
        &OF_defaultNorm,
        OF_defaultNormLog,
        DefaultMaxOff,
        workspace,
        wkspSize,
    ));
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateBlockSize_symbolType(
        fseMetadata.llType,
        llCodeTable,
        nbSeq,
        MaxLL,
        &fseTables.litlengthCTable,
        LL_bits.as_ptr(),
        &LL_defaultNorm,
        LL_defaultNormLog,
        MaxLL,
        workspace,
        wkspSize,
    ));
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateBlockSize_symbolType(
        fseMetadata.mlType,
        mlCodeTable,
        nbSeq,
        MaxML,
        &fseTables.matchlengthCTable,
        ML_bits.as_ptr(),
        &ML_defaultNorm,
        ML_defaultNormLog,
        MaxML,
        workspace,
        wkspSize,
    ));

    if writeEntropy {
        cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(fseMetadata.fseTablesSize);
    }

    cSeqSizeEstimate.wrapping_add(sequencesSectionHeaderSize)
}

/// Returns the size estimate for a given stream of literals, of, ll, ml
unsafe fn ZSTD_estimateBlockSize(
    literals: *const u8,
    litSize: size_t,
    ofCodeTable: *const u8,
    llCodeTable: *const u8,
    mlCodeTable: *const u8,
    nbSeq: size_t,
    entropy: &ZSTD_entropyCTables_t,
    entropyMetadata: *const ZSTD_entropyCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeLitEntropy: bool,
    writeSeqEntropy: bool,
) -> size_t {
    let literalsSize = ZSTD_estimateBlockSize_literal(
        literals,
        litSize,
        &entropy.huf,
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
        &entropy.fse,
        &(*entropyMetadata).fseMetadata,
        workspace,
        wkspSize,
        writeSeqEntropy,
    );
    seqSize
        .wrapping_add(literalsSize)
        .wrapping_add(ZSTD_blockHeaderSize)
}

/// Builds entropy statistics and uses them for blocksize estimation.
///
/// # Returns
///
/// The estimated compressed size of the seqStore, or a zstd error.
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
        (*entropyMetadata).hufMetadata.hType == SymbolEncodingType::Compressed,
        true,
    )
}

/// Returns literals bytes represented in a seqStore
unsafe fn ZSTD_countSeqStoreLiteralsBytes(seqStore: *const SeqStore_t) -> size_t {
    let mut literalsBytes = 0usize;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as size_t;
    for i in 0..nbSeqs {
        let seq = *((*seqStore).sequencesStart).add(i);
        literalsBytes = literalsBytes.wrapping_add(seq.litLength as size_t);
        if i == (*seqStore).longLengthPos as size_t
            && (*seqStore).longLengthType == LongLengthType::Literal
        {
            literalsBytes = literalsBytes.wrapping_add(0x10000 as core::ffi::c_int as size_t);
        }
    }
    literalsBytes
}

/// Returns match bytes represented in a seqStore
unsafe fn ZSTD_countSeqStoreMatchBytes(seqStore: *const SeqStore_t) -> size_t {
    let mut matchBytes = 0usize;
    let nbSeqs = ((*seqStore).sequences).offset_from((*seqStore).sequencesStart) as size_t;
    for i in 0..nbSeqs {
        let seq = *((*seqStore).sequencesStart).add(i);
        matchBytes = matchBytes.wrapping_add((seq.mlBase as core::ffi::c_int + MINMATCH) as size_t);
        if i == (*seqStore).longLengthPos as size_t
            && (*seqStore).longLengthType == LongLengthType::Match
        {
            matchBytes = matchBytes.wrapping_add(0x10000 as core::ffi::c_int as size_t);
        }
    }
    matchBytes
}

/// Derives the seqStore that is a chunk of the originalSeqStore from [startIdx, endIdx).
/// Stores the result in resultSeqStore.
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

    // Move longLengthPos into the correct position if necessary
    if (*originalSeqStore).longLengthType != LongLengthType::None {
        if ((*originalSeqStore).longLengthPos as size_t) < startIdx
            || (*originalSeqStore).longLengthPos as size_t > endIdx
        {
            resultSeqStore.longLengthType = LongLengthType::None;
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

/// Returns the raw offset represented by the combination of offBase, ll0, and repcode history.
/// offBase must represent a repcode in the numeric representation of ZSTD_storeSeq().
fn ZSTD_resolveRepcodeToRawOffset(rep: &[u32; 3], offBase: u32, ll0: u32) -> u32 {
    let adjustedRepCode = offBase.wrapping_sub(1).wrapping_add(ll0);
    if adjustedRepCode == ZSTD_REP_NUM as u32 {
        return rep[0].wrapping_sub(1);
    }
    rep[adjustedRepCode as usize]
}

/// ZSTD_seqStore_resolveOffCodes() reconciles any possible divergences in offset history that
/// may arise due to emission of RLE/raw blocks that disturb the offset history, and replaces any
/// repcodes within the seqStore that may be invalid.
///
/// dRepcodes are updated as would be on the decompression side.
/// cRepcodes are updated exactly in accordance with the seqStore.
///
/// Note: this function assumes seq->offBase respects the following numbering scheme:
/// 0: invalid, 1-3: repcode 1-3, 4+: real_offset+3
unsafe fn ZSTD_seqStore_resolveOffCodes(
    dRepcodes: &mut Repcodes_t,
    cRepcodes: &mut Repcodes_t,
    seqStore: *const SeqStore_t,
    nbSeq: u32,
) {
    let longLitLenIdx = if (*seqStore).longLengthType == LongLengthType::Literal {
        (*seqStore).longLengthPos
    } else {
        nbSeq
    };
    for idx in 0..nbSeq {
        let seq = ((*seqStore).sequencesStart).offset(idx as isize);
        let ll0 = ((*seq).litLength as core::ffi::c_int == 0 && idx != longLitLenIdx)
            as core::ffi::c_int as u32;
        let offBase = (*seq).offBase;
        if 1 <= offBase && offBase <= ZSTD_REP_NUM as u32 {
            let dRawOffset = ZSTD_resolveRepcodeToRawOffset(&dRepcodes.rep, offBase, ll0);
            let cRawOffset = ZSTD_resolveRepcodeToRawOffset(&cRepcodes.rep, offBase, ll0);
            // Adjust simulated decompression repcode history if we come across a mismatch. Replace
            // the repcode with the offset it actually references, determined by the compression
            // repcode history.
            if dRawOffset != cRawOffset {
                (*seq).offBase = cRawOffset.wrapping_add(ZSTD_REP_NUM as u32);
            }
        }
        // Compression repcode history is always updated with values directly from the unmodified seqStore.
        // Decompression repcode history may use modified seq->offset value taken from compression repcode history.
        ZSTD_updateRep(&mut dRepcodes.rep, (*seq).offBase, ll0);
        ZSTD_updateRep(&mut cRepcodes.rep, offBase, ll0);
    }
}

/// ZSTD_compressSeqStore_singleBlock(): Compresses a seqStore into a block with a block header,
/// into the buffer dst.
///
/// # Returns
///
/// The total size of that block (including header) or a ZSTD error code.
unsafe fn ZSTD_compressSeqStore_singleBlock(
    zc: *mut ZSTD_CCtx,
    seqStore: *const SeqStore_t,
    dRep: &mut Repcodes_t,
    cRep: &mut Repcodes_t,
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

    // In case of an RLE or raw block, the simulated decompression repcode history must be reset
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
        && ZSTD_isRLE(src as *const u8, srcSize)
    {
        // We don't want to emit our first block as a RLE even if it qualifies because
        // doing so will cause the decoder (cli only) to throw a "should consume all input error."
        // This is only an issue for zstd <= v1.4.3
        cSeqsSize = 1;
    }

    // Sequence collection not supported when block splitting */
    if (*zc).seqCollector.collectSequences != 0 {
        let err_code_0 =
            ZSTD_copyBlockSequences(&mut (*zc).seqCollector, seqStore, &dRepOriginal.rep);
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
        *dRep = dRepOriginal; // reset simulated decompression repcode history
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
        *dRep = dRepOriginal; // reset simulated decompression repcode history
    } else {
        ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*zc).blockState);
        writeBlockHeader(op as *mut core::ffi::c_void, cSeqsSize, srcSize, lastBlock);
        cSize = ZSTD_blockHeaderSize.wrapping_add(cSeqsSize);
    }

    if (*(*zc).blockState.prevCBlock)
        .entropy
        .fse
        .offcode_repeatMode
        == FSE_repeat_valid
    {
        (*(*zc).blockState.prevCBlock)
            .entropy
            .fse
            .offcode_repeatMode = FSE_repeat_check;
    }

    cSize
}

pub const MIN_SEQUENCES_BLOCK_SPLITTING: usize = 300;

/// Helper function to perform the recursive search for block splits.
/// Estimates the cost of seqStore prior to split, and estimates the cost of splitting the
/// sequences in half. If advantageous to split, then we recurse down the two sub-blocks.
/// If not, or if an error occurred in estimation, then we do not recurse.
///
/// Note: The recursion depth is capped by a heuristic minimum number of sequences, defined by
/// MIN_SEQUENCES_BLOCK_SPLITTING. In theory, this means the absolute largest recursion depth is
/// 10 == log2(maxNbSeqInBlock/MIN_SEQUENCES_BLOCK_SPLITTING). In practice, recursion depth
/// usually doesn't go beyond 4.
///
/// Furthermore, the number of splits is capped by ZSTD_MAX_NB_BLOCK_SPLITS.
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

    if endIdx.wrapping_sub(startIdx) < MIN_SEQUENCES_BLOCK_SPLITTING
        || (*splits).idx >= ZSTD_MAX_NB_BLOCK_SPLITS
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

/// Base recursive function. Populates a table with intra-block partition indices that can improve
/// compression ratio.
///
/// # Returns
///
/// The number of splits made (which equals the size of the partition table - 1).
unsafe fn ZSTD_deriveBlockSplits(zc: *mut ZSTD_CCtx, partitions: *mut u32, nbSeq: u32) -> size_t {
    let mut splits = seqStoreSplits {
        splitLocations: core::ptr::null_mut::<u32>(),
        idx: 0,
    };
    splits.splitLocations = partitions;
    splits.idx = 0;
    if nbSeq <= 4 {
        // Refuse to try and split anything with less than 4 sequences
        return 0;
    }
    ZSTD_deriveBlockSplitsHelper(&mut splits, 0, nbSeq as size_t, zc, &(*zc).seqStore);
    *(splits.splitLocations).add(splits.idx) = nbSeq;
    splits.idx
}

/// ZSTD_compressBlock_splitBlock(): Attempts to split a given block into multiple blocks to
/// improve compression ratio.
///
/// # Returns
///
/// THe combined size of all blocks (which includes headers), or a ZSTD error code.
unsafe fn ZSTD_compressBlock_splitBlock_internal(
    zc: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    src: *const core::ffi::c_void,
    blockSize: size_t,
    lastBlock: u32,
    nbSeq: u32,
) -> size_t {
    let mut cSize = 0usize;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let mut srcBytesTotal = 0usize;
    let partitions = ((*zc).blockSplitCtx.partitions).as_mut_ptr();
    let nextSeqStore: &mut SeqStore_t = &mut (*zc).blockSplitCtx.nextSeqStore;
    let currSeqStore: &mut SeqStore_t = &mut (*zc).blockSplitCtx.currSeqStore;
    let numSplits = ZSTD_deriveBlockSplits(zc, partitions, nbSeq);

    let mut dRep = repcodes_s { rep: [0; 3] };
    let mut cRep = repcodes_s { rep: [0; 3] };
    dRep.rep = (*(*zc).blockState.prevCBlock).rep;
    cRep.rep = (*(*zc).blockState.prevCBlock).rep;
    ptr::write_bytes(
        nextSeqStore as *mut SeqStore_t as *mut u8,
        0,
        size_of::<SeqStore_t>(),
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
    for i in 0..numSplits + 1 {
        let mut cSizeChunk: size_t = 0;
        let lastPartition = i == numSplits;
        let mut lastBlockEntireSrc = 0;
        let mut srcBytes = (ZSTD_countSeqStoreLiteralsBytes(currSeqStore))
            .wrapping_add(ZSTD_countSeqStoreMatchBytes(currSeqStore));
        srcBytesTotal = srcBytesTotal.wrapping_add(srcBytes);
        if lastPartition {
            // This is the final partition, need to account for possible last literals
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
    }

    // cRep and dRep may have diverged during the compression.
    // If so, we use the dRep repcodes for the next block.
    (*(*zc).blockState.prevCBlock).rep = dRep.rep;
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

    if bss == BuildSeqStore::NoCompress as size_t {
        if (*(*zc).blockState.prevCBlock)
            .entropy
            .fse
            .offcode_repeatMode
            == FSE_repeat_valid
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
    // This is an estimated upper bound for the length of an rle block.
    // This isn't the actual upper bound.
    // Finding the real threshold needs further investigation.
    let rleMaxLength = 25;
    let mut cSize: size_t = 0;
    let ip = src as *const u8;
    let op = dst as *mut u8;

    let bss = ZSTD_buildSeqStore(zc, src, srcSize);
    let err_code = bss;
    if ERR_isError(err_code) {
        return err_code;
    }
    if bss == BuildSeqStore::NoCompress as size_t {
        if (*zc).seqCollector.collectSequences != 0 {
            return Error::sequenceProducer_failed.to_error_code();
        }
        cSize = 0;
    } else {
        if (*zc).seqCollector.collectSequences != 0 {
            let err_code_0 = ZSTD_copyBlockSequences(
                &mut (*zc).seqCollector,
                ZSTD_getSeqStore(zc),
                &(*(*zc).blockState.prevCBlock).rep,
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
            && ZSTD_isRLE(ip, srcSize)
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
        .offcode_repeatMode
        == FSE_repeat_valid
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
    if bss == BuildSeqStore::Compress as size_t {
        if (*zc).isFirstBlock == 0
            && ZSTD_maybeRLE(&(*zc).seqStore)
            && ZSTD_isRLE(src as *const u8, srcSize)
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

    // Superblock compression failed, attempt to emit a single no compress block.
    // The decoder will be able to stream this block since it is uncompressed.
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
        .offcode_repeatMode
        == FSE_repeat_valid
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
    ws: &mut ZSTD_cwksp,
    params: &ZSTD_CCtx_params,
    ip: *const core::ffi::c_void,
    iend: *const core::ffi::c_void,
) {
    let cycleLog = ZSTD_cycleLog(params.cParams.chainLog, params.cParams.strategy);
    let maxDist = 1 << params.cParams.windowLog;
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
        // invalidate dictionaries on overflow correction
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
    // Split level based on compression strategy, from `fast` to `btultra2`
    static splitLevels: [core::ffi::c_int; 10] = [0, 0, 1, 2, 2, 3, 3, 4, 4, 4];
    // Note: conservatively only split full blocks (128 KB) currently.
    // While it's possible to go lower, let's keep it simple for a first implementation.
    // Besides, benefits of splitting are reduced when blocks are already small.
    if srcSize < (128 * (1 << 10)) as size_t || blockSizeMax < (128 * (1 << 10)) as size_t {
        return srcSize.min(blockSizeMax);
    }
    // Do not split incompressible data though:
    // Require verified savings to allow pre-splitting.
    // Note: as a consequence, the first full block is not split.
    if savings < 3 {
        return (128 * (1 << 10)) as size_t;
    }
    // Apply @splitLevel, or use default value (which depends on @strat).
    // Note that splitting heuristic is still conditioned by @savings >= 3,
    // so the first block will not reach this code path.
    if splitLevel == 1 {
        return (128 * (1 << 10)) as size_t;
    }
    if splitLevel == 0 {
        splitLevel = splitLevels[strat as usize];
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

/// Compress a chunk of data into one or multiple blocks.
/// All blocks will be terminated, all input will be consumed.
/// Function will issue an error if there is not enough `dstCapacity` to hold the compressed content.
/// Frame is supposed already started (header already produced)
///
/// # Returns
///
/// The compressed size, or an error code
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
    let maxDist = 1 << (*cctx).appliedParams.cParams.windowLog;
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
        let lastBlock = lastFrameChunk & (blockSize == remaining) as core::ffi::c_int as u32;

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
            Some(&mut ms.dictMatchState),
        );

        // Ensure hash/chain table insertion resumes no sooner than lowlimit
        if ms.nextToUpdate < ms.window.lowLimit {
            ms.nextToUpdate = ms.window.lowLimit;
        }

        let mut cSize: size_t = 0;
        if ZSTD_useTargetCBlockSize(&(*cctx).appliedParams) {
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
        } else if ZSTD_blockSplitterEnabled(&(*cctx).appliedParams) {
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
                        .wrapping_add((BlockType::Rle as u32) << 1)
                        .wrapping_add((blockSize << 3) as u32)
                } else {
                    lastBlock
                        .wrapping_add((BlockType::Compressed as u32) << 1)
                        .wrapping_add((cSize << 3) as u32)
                };
                MEM_writeLE24(op as *mut core::ffi::c_void, cBlockHeader);
                cSize = cSize.wrapping_add(ZSTD_blockHeaderSize);
            }
        }

        // @savings is employed to ensure that splitting doesn't worsen expansion of incompressible data.
        // Without splitting, the maximum expansion is 3 bytes per full block.
        // An adversarial input could attempt to fudge the split detector,
        // and make it split incompressible data, resulting in more block headers.
        //
        // Note that, since ZSTD_COMPRESSBOUND() assumes a worst case scenario of 1KB per block,
        // and the splitter never creates blocks that small (current lower limit is 8 KB),
        // there is already no risk to expand beyond ZSTD_COMPRESSBOUND() limit.
        // But if the goal is to not expand by more than 3-bytes per 128 KB full block,
        // then yes, it becomes possible to make the block splitter oversplit incompressible data.
        //
        // Using @savings, we enforce an even more conservative condition,
        // requiring the presence of enough savings (at least 3 bytes) to authorize splitting,
        // otherwise only full blocks are used.
        // But being conservative is fine,
        // since splitting barely compressible blocks is not fruitful anyway.
        savings += blockSize as S64 - cSize as S64;
        ip = ip.add(blockSize);
        remaining = remaining.wrapping_sub(blockSize);
        op = op.add(cSize);
        dstCapacity = dstCapacity.wrapping_sub(cSize);
        (*cctx).isFirstBlock = 0;
    }

    if lastFrameChunk != 0 && op > ostart {
        (*cctx).stage = CompressionStage::Ending;
    }
    op.offset_from_unsigned(ostart)
}

unsafe fn ZSTD_writeFrameHeader(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    params: &ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    dictID: u32,
) -> size_t {
    let op = dst as *mut u8;
    let dictIDSizeCodeLength = ((dictID > 0) as core::ffi::c_int
        + (dictID >= 256) as core::ffi::c_int
        + (dictID >= 65536) as core::ffi::c_int) as u32;
    let dictIDSizeCode = if params.fParams.noDictIDFlag != 0 {
        0
    } else {
        dictIDSizeCodeLength
    };
    let checksumFlag = (params.fParams.checksumFlag > 0) as core::ffi::c_int as u32;
    let windowSize = 1 << params.cParams.windowLog;
    let singleSegment = (params.fParams.contentSizeFlag != 0 && windowSize as u64 >= pledgedSrcSize)
        as core::ffi::c_int as u32;
    let windowLogByte = ((params.cParams.windowLog)
        .wrapping_sub(ZSTD_WINDOWLOG_ABSOLUTEMIN as core::ffi::c_uint)
        << 3) as u8;
    let fcsCode = (if params.fParams.contentSizeFlag != 0 {
        (pledgedSrcSize >= 256) as core::ffi::c_int
            + (pledgedSrcSize >= (65536 + 256) as u64) as core::ffi::c_int
            + (pledgedSrcSize >= 0xffffffff as core::ffi::c_uint as u64) as core::ffi::c_int
    } else {
        0
    }) as u32;
    let frameHeaderDescriptionByte = dictIDSizeCode
        .wrapping_add(checksumFlag << 2)
        .wrapping_add(singleSegment << 5)
        .wrapping_add(fcsCode << 6) as u8;
    let mut pos = 0usize;

    if dstCapacity < 18 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if params.format == Format::ZSTD_f_zstd1 {
        MEM_writeLE32(dst, ZSTD_MAGICNUMBER);
        pos = 4;
    }
    *op.add(pos) = frameHeaderDescriptionByte;
    pos = pos.wrapping_add(1);
    if singleSegment == 0 {
        *op.add(pos) = windowLogByte;
        pos = pos.wrapping_add(1);
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
                *op.add(pos) = pledgedSrcSize as u8;
                pos = pos.wrapping_add(1);
            }
        }
    }
    pos
}

/// Writes out a skippable frame with the specified magic number variant (16 are supported),
/// from ZSTD_MAGIC_SKIPPABLE_START to ZSTD_MAGIC_SKIPPABLE_START+15, and the desired source data.
///
/// # Returns
///
/// The total number of bytes written, or a ZSTD error code.
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
    core::ptr::copy_nonoverlapping(src.cast::<u8>(), op.add(8), srcSize);

    srcSize.wrapping_add(ZSTD_SKIPPABLEHEADERSIZE as size_t)
}

/// Output an empty Block with end-of-frame mark to complete a frame.
///
/// # Returns
///
/// Size of data written into `dst` (== ZSTD_blockHeaderSize) or an error code if `dstCapacity`
/// is too small (<ZSTD_blockHeaderSize)
pub unsafe fn ZSTD_writeLastEmptyBlock(dst: *mut core::ffi::c_void, dstCapacity: size_t) -> size_t {
    if dstCapacity < ZSTD_blockHeaderSize {
        return Error::dstSize_tooSmall.to_error_code();
    }
    let cBlockHeader24 = (1u32).wrapping_add((BlockType::Raw as u32) << 1);
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

    if (*cctx).stage == CompressionStage::Created {
        return Error::stage_wrong.to_error_code();
    }

    if frame != 0 && (*cctx).stage == CompressionStage::Init {
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
        (*cctx).stage = CompressionStage::Ongoing;
    }

    if srcSize == 0 {
        // Do not generate an empty block if no input
        return fhSize;
    }

    if !ZSTD_window_update(&mut ms.window, src, srcSize, ms.forceNonContiguous != 0) {
        ms.forceNonContiguous = 0;
        ms.nextToUpdate = ms.window.dictLimit;
    }
    if (*cctx).appliedParams.ldmParams.enableLdm == ParamSwitch::Enable {
        ZSTD_window_update(&mut (*cctx).ldmState.window, src, srcSize, false);
    }

    if frame == 0 {
        // Overflow check and correction for block mode
        ZSTD_overflowCorrectIfNeeded(
            ms,
            &mut (*cctx).workspace,
            &(*cctx).appliedParams,
            src,
            src.byte_add(srcSize),
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
    (*cctx)
        .appliedParams
        .maxBlockSize
        .min(1 << cParams.windowLog)
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

/// # Returns
///
/// 0, or an error code
unsafe fn ZSTD_loadDictionaryContent(
    ms: &mut ZSTD_MatchState_t,
    ls: *mut ldmState_t,
    ws: &mut ZSTD_cwksp,
    params: &ZSTD_CCtx_params,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    dtlm: DictTableLoadMethod,
    tfp: TableFillPurpose,
) -> size_t {
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let loadLdmDict =
        (params.ldmParams.enableLdm == ParamSwitch::Enable && !ls.is_null()) as core::ffi::c_int;

    // Assert that the ms params match the params we're being given
    ZSTD_assertEqualCParams(params.cParams, ms.cParams);

    // Ensure large dictionaries can't cause index overflow
    let mut maxDictSize = (if MEM_64bits() {
        (3500 as core::ffi::c_uint)
            .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
    } else {
        (2000 as core::ffi::c_uint)
            .wrapping_mul(((1 as core::ffi::c_int) << 20) as core::ffi::c_uint)
    })
    .wrapping_sub(ZSTD_WINDOW_START_INDEX as core::ffi::c_uint);

    let CDictTaggedIndices = ZSTD_CDictIndicesAreTagged(&params.cParams);
    if CDictTaggedIndices && tfp == TableFillPurpose::ForCDict {
        let shortCacheMaxDictSize = ((1 as core::ffi::c_uint) << (32 - ZSTD_SHORT_CACHE_TAG_BITS))
            .wrapping_sub(ZSTD_WINDOW_START_INDEX as core::ffi::c_uint);
        maxDictSize = maxDictSize.min(shortCacheMaxDictSize);
    }

    // If the dictionary is too large, only load the suffix of the dictionary.
    if srcSize > maxDictSize as size_t {
        ip = iend.sub(maxDictSize as usize);
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
        // We must have cleared our windows when our source is this large.
        assert!(loadLdmDict != 0);
    }

    ZSTD_window_update(&mut ms.window, src, srcSize, false);

    if loadLdmDict != 0 {
        // Load the entire dict into LDM matchfinders.
        ZSTD_window_update(&mut (*ls).window, src, srcSize, false);
        (*ls).loadedDictEnd = if params.forceWindow != 0 {
            0
        } else {
            iend.offset_from((*ls).window.base) as core::ffi::c_long as u32
        };
        ZSTD_ldm_fillHashTable(ls, ip, iend, &params.ldmParams);
    }

    // If the dict is larger than we can reasonably index in our tables, only load the suffix.
    let maxDictSize_0 = 1
        << (if (if (params.cParams.hashLog).wrapping_add(3)
            > (params.cParams.chainLog).wrapping_add(1)
        {
            (params.cParams.hashLog).wrapping_add(3)
        } else {
            (params.cParams.chainLog).wrapping_add(1)
        }) < 31
        {
            (params.cParams.hashLog)
                .wrapping_add(3)
                .max((params.cParams.chainLog).wrapping_add(1))
        } else {
            31
        });
    if srcSize > maxDictSize_0 as size_t {
        ip = iend.sub(maxDictSize_0 as usize);
        src = ip as *const core::ffi::c_void;
        srcSize = maxDictSize_0 as size_t;
    }

    ms.nextToUpdate = ip.wrapping_offset_from(ms.window.base) as core::ffi::c_long as u32;
    ms.loadedDictEnd = if params.forceWindow != 0 {
        0
    } else {
        iend.wrapping_offset_from(ms.window.base) as core::ffi::c_long as u32
    };
    ms.forceNonContiguous = params.deterministicRefPrefix;

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

    match params.cParams.strategy as core::ffi::c_uint {
        1 => {
            ZSTD_fillHashTable(ms, iend as *const core::ffi::c_void, dtlm, tfp);
        }
        2 => {
            ZSTD_fillDoubleHashTable(ms, iend as *const core::ffi::c_void, dtlm, tfp);
        }
        3..=5 => {
            if ms.dedicatedDictSearch != 0 {
                ZSTD_dedicatedDictSearch_lazy_loadDictionary(ms, iend.sub(HASH_READ_SIZE as usize));
            } else if params.useRowMatchFinder == ParamSwitch::Enable {
                let tagTableSize = 1 << params.cParams.hashLog;
                ptr::write_bytes(ms.tagTable, 0, tagTableSize as usize);
                ZSTD_row_update(ms, iend.sub(HASH_READ_SIZE as usize));
            } else {
                ZSTD_insertAndFindFirstIndex(ms, iend.sub(HASH_READ_SIZE as usize));
            }
        }
        6..=9 => {
            ZSTD_updateTree(ms, iend.sub(HASH_READ_SIZE as usize), iend);
        }
        _ => {}
    }

    ms.nextToUpdate = iend.wrapping_offset_from(ms.window.base) as core::ffi::c_long as u32;

    0
}

/// Dictionaries that assign zero probability to symbols that show up causes problems when FSE
/// encoding. Mark dictionaries with zero probability symbols as FSE_repeat_check and only
/// dictionaries with 100% valid symbols can be assumed valid.
unsafe fn ZSTD_dictNCountRepeat(
    normalizedCounter: &[core::ffi::c_short],
    dictMaxSymbolValue: u8,
    maxSymbolValue: u8,
) -> FSE_repeat {
    if dictMaxSymbolValue < maxSymbolValue {
        return FSE_repeat_check;
    }
    for s in 0..usize::from(maxSymbolValue) + 1 {
        if normalizedCounter[s] as core::ffi::c_int == 0 {
            return FSE_repeat_check;
        }
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

    let mut maxSymbolValue = u8::MAX;
    let mut hasZeroWeights = 1;
    let hufHeaderSize = HUF_readCTable(
        &mut (*bs).entropy.huf.CTable,
        &mut maxSymbolValue,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from_unsigned(dictPtr),
        &mut hasZeroWeights,
    );

    // We only set the loaded table as valid if it contains all non-zero
    // weights. Otherwise, we set it to check
    if hasZeroWeights == 0 && maxSymbolValue == u8::MAX {
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
    // fill all offset symbols to avoid garbage at end of table
    if ERR_isError(FSE_buildCTable_wksp(
        &mut (*bs).entropy.fse.offcodeCTable,
        &offcodeNCount,
        31,
        offcodeLog,
        workspace,
        ((8 << 10) + 512) as size_t,
    )) {
        return Error::dictionary_corrupted.to_error_code();
    }
    // Defer checking offcodeMaxValue because we need to know the size of the dictionary content
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
        &mut (*bs).entropy.fse.matchlengthCTable,
        &matchlengthNCount,
        matchlengthMaxValue,
        matchlengthLog,
        workspace,
        ((8 << 10) + 512) as size_t,
    )) {
        return Error::dictionary_corrupted.to_error_code();
    }
    (*bs).entropy.fse.matchlength_repeatMode =
        ZSTD_dictNCountRepeat(&matchlengthNCount, matchlengthMaxValue, MaxML);
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
        &mut (*bs).entropy.fse.litlengthCTable,
        &litlengthNCount,
        litlengthMaxValue,
        litlengthLog,
        workspace,
        ((8 << 10) + 512) as size_t,
    )) {
        return Error::dictionary_corrupted.to_error_code();
    }
    (*bs).entropy.fse.litlength_repeatMode =
        ZSTD_dictNCountRepeat(&litlengthNCount, litlengthMaxValue, MaxLL);
    dictPtr = dictPtr.add(litlengthHeaderSize);

    if dictPtr.add(12) > dictEnd {
        return Error::dictionary_corrupted.to_error_code();
    }
    (*bs).rep[0] = MEM_readLE32(dictPtr as *const core::ffi::c_void);
    (*bs).rep[1] = MEM_readLE32(dictPtr.add(4) as *const core::ffi::c_void);
    (*bs).rep[2] = MEM_readLE32(dictPtr.add(8) as *const core::ffi::c_void);
    dictPtr = dictPtr.add(12);

    let dictContentSize = dictEnd.offset_from_unsigned(dictPtr);
    let mut offcodeMax = MaxOff;
    if dictContentSize
        <= (-(1 as core::ffi::c_int) as u32)
            .wrapping_sub((128 as core::ffi::c_int * ((1 as core::ffi::c_int) << 10)) as u32)
            as size_t
    {
        let maxOffset = (dictContentSize as u32).wrapping_add((128 * (1 << 10)) as u32);
        // `maxOffset` is a `u32`, so its highest set bit is at most 31
        offcodeMax = ZSTD_highbit32(maxOffset) as u8;
    }
    // All offset values <= dictContentSize + 128 KB must be representable for a valid table
    (*bs).entropy.fse.offcode_repeatMode =
        ZSTD_dictNCountRepeat(&offcodeNCount, offcodeMaxValue, offcodeMax.min(31));

    // All repCodes must be <= dictContentSize and != 0
    for size in (*bs).rep {
        if !(1..=dictContentSize).contains(&(size as usize)) {
            return Error::dictionary_corrupted.to_error_code();
        }
    }

    dictPtr.offset_from_unsigned(dict as *const u8)
}

/// Assumptions: magic number supposed already checked, dictSize supposed >= 8.
///
/// # Returns
///
/// dictID, or an error code.
unsafe fn ZSTD_loadZstdDictionary(
    bs: *mut ZSTD_compressedBlockState_t,
    ms: &mut ZSTD_MatchState_t,
    ws: &mut ZSTD_cwksp,
    params: &ZSTD_CCtx_params,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dtlm: DictTableLoadMethod,
    tfp: TableFillPurpose,
    workspace: *mut core::ffi::c_void,
) -> size_t {
    let mut dictPtr = dict as *const u8;
    let dictEnd = dictPtr.add(dictSize);
    let mut dictID: size_t = 0;
    let mut eSize: size_t = 0;

    dictID = (if params.fParams.noDictIDFlag != 0 {
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

/// # Returns
///
/// dictID, or an error code.
unsafe fn ZSTD_compress_insertDictionary(
    bs: *mut ZSTD_compressedBlockState_t,
    ms: &mut ZSTD_MatchState_t,
    ls: *mut ldmState_t,
    ws: &mut ZSTD_cwksp,
    params: &ZSTD_CCtx_params,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
    dtlm: DictTableLoadMethod,
    tfp: TableFillPurpose,
    workspace: *mut core::ffi::c_void,
) -> size_t {
    if dict.is_null() || dictSize < 8 {
        if dictContentType == ZSTD_dct_fullDict {
            return Error::dictionary_wrong.to_error_code();
        }
        return 0;
    }

    ZSTD_reset_compressedBlockState(bs);

    // dict restricted modes
    if dictContentType == ZSTD_dct_rawContent {
        return ZSTD_loadDictionaryContent(ms, ls, ws, params, dict, dictSize, dtlm, tfp);
    }

    if MEM_readLE32(dict) != ZSTD_MAGIC_DICTIONARY {
        if dictContentType == ZSTD_dct_auto {
            return ZSTD_loadDictionaryContent(ms, ls, ws, params, dict, dictSize, dtlm, tfp);
        }
        if dictContentType == ZSTD_dct_fullDict {
            return Error::dictionary_wrong.to_error_code();
        }
    }

    // dict as full zstd dictionary
    ZSTD_loadZstdDictionary(bs, ms, ws, params, dict, dictSize, dtlm, tfp, workspace)
}

pub const ZSTD_USE_CDICT_PARAMS_SRCSIZE_CUTOFF: core::ffi::c_int = 128 * (1 << 10);

pub const ZSTD_USE_CDICT_PARAMS_DICTSIZE_MULTIPLIER: core::ffi::c_ulonglong = 6;

/// Assumption: either @dict OR @cdict (or none) is non-NULL, never both.
///
/// # Returns
///
/// 0, or an error code.
unsafe fn ZSTD_compressBegin_internal(
    cctx: *mut ZSTD_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
    dtlm: DictTableLoadMethod,
    cdict: *const ZSTD_CDict,
    params: &ZSTD_CCtx_params,
    pledgedSrcSize: u64,
    zbuff: BufferedPolicy,
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
        && params.attachDictPref != ZSTD_dictAttachPref_e::ZSTD_dictForceLoad
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
            TableFillPurpose::ForCCtx,
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
            TableFillPurpose::ForCCtx,
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
    dtlm: DictTableLoadMethod,
    cdict: *const ZSTD_CDict,
    params: &ZSTD_CCtx_params,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    // compression parameters verification and optimization
    let err_code = ZSTD_checkCParams(params.cParams);
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
        BufferedPolicy::NotBuffered,
    )
}

/// # Returns
///
/// 0, or an error code.
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
            enableLdm: ParamSwitch::Auto,
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
        postBlockSplitter: ParamSwitch::Auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ParamSwitch::Auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ParamSwitch::Auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ParamSwitch::Auto,
    };
    ZSTD_CCtxParams_init_internal(&mut cctxParams, &params, ZSTD_NO_CLEVEL);
    ZSTD_compressBegin_advanced_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        DictTableLoadMethod::Fast,
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
            enableLdm: ParamSwitch::Auto,
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
        postBlockSplitter: ParamSwitch::Auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ParamSwitch::Auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ParamSwitch::Auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ParamSwitch::Auto,
    };

    let params = ZSTD_getParams_internal(
        compressionLevel,
        ZSTD_CONTENTSIZE_UNKNOWN,
        dictSize,
        CParamMode::NoAttachDict,
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
        DictTableLoadMethod::Fast,
        core::ptr::null(),
        &cctxParams,
        ZSTD_CONTENTSIZE_UNKNOWN,
        BufferedPolicy::NotBuffered,
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

/// Ends a frame.
///
/// # Returns
///
/// The number of bytes written into dst (or an error code).
unsafe fn ZSTD_writeEpilogue(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;

    if (*cctx).stage == CompressionStage::Created {
        return Error::stage_wrong.to_error_code();
    }

    // special case: empty frame
    if (*cctx).stage == CompressionStage::Init {
        let fhSize = ZSTD_writeFrameHeader(dst, dstCapacity, &(*cctx).appliedParams, 0, 0);
        let err_code = fhSize;
        if ERR_isError(err_code) {
            return err_code;
        }
        dstCapacity = dstCapacity.wrapping_sub(fhSize);
        op = op.add(fhSize);
        (*cctx).stage = CompressionStage::Ongoing;
    }

    if (*cctx).stage != CompressionStage::Ending {
        // write one last empty block, make it the "last" block
        let cBlockHeader24 = 1u32
            .wrapping_add((BlockType::Raw as u32) << 1)
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

    // return to "created but no init" status
    (*cctx).stage = CompressionStage::Created;
    op.offset_from_unsigned(ostart)
}

pub unsafe fn ZSTD_CCtx_trace(cctx: *mut ZSTD_CCtx, extraCSize: size_t) {
    if (*cctx).traceCtx != 0 {
        let streaming = ((*cctx).inBuffSize > 0
            || (*cctx).outBuffSize > 0
            || (*cctx).appliedParams.nbWorkers > 0) as core::ffi::c_int;
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
    // control src size
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
    params: &ZSTD_CCtx_params,
) -> size_t {
    let err_code = ZSTD_compressBegin_internal(
        cctx,
        dict,
        dictSize,
        ZSTD_dct_auto,
        DictTableLoadMethod::Fast,
        core::ptr::null::<ZSTD_CDict>(),
        params,
        srcSize as u64,
        BufferedPolicy::NotBuffered,
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
        CParamMode::NoAttachDict,
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
        stage: CompressionStage::Created,
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
                enableLdm: ParamSwitch::Auto,
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
            postBlockSplitter: ParamSwitch::Auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ParamSwitch::Auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem::default(),
            prefetchCDictTables: ParamSwitch::Auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ParamSwitch::Auto,
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
                enableLdm: ParamSwitch::Auto,
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
            postBlockSplitter: ParamSwitch::Auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ParamSwitch::Auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem::default(),
            prefetchCDictTables: ParamSwitch::Auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ParamSwitch::Auto,
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
                enableLdm: ParamSwitch::Auto,
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
            postBlockSplitter: ParamSwitch::Auto,
            preBlockSplitter_level: 0,
            maxBlockSize: 0,
            useRowMatchFinder: ParamSwitch::Auto,
            deterministicRefPrefix: 0,
            customMem: ZSTD_customMem::default(),
            prefetchCDictTables: ParamSwitch::Auto,
            enableMatchFinderFallback: 0,
            extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
            extSeqProdFunc: None,
            searchForExternalRepcodes: ParamSwitch::Auto,
        },
        dictID: 0,
        dictContentSize: 0,
        workspace: ZSTD_cwksp::default(),
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
            longLengthType: LongLengthType::None,
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
                    priceType: OptPrice::Dynamic,
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
        bufferedPolicy: BufferedPolicy::NotBuffered,
        inBuff: core::ptr::null_mut(),
        inBuffSize: 0,
        inToCompress: 0,
        inBuffPos: 0,
        inBuffTarget: 0,
        outBuff: core::ptr::null_mut(),
        outBuffSize: 0,
        outBuffContentSize: 0,
        outBuffFlushedSize: 0,
        streamStage: StreamStage::Init,
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
                longLengthType: LongLengthType::None,
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
                longLengthType: LongLengthType::None,
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
                longLengthType: LongLengthType::None,
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
                longLengthType: LongLengthType::None,
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
                longLengthType: LongLengthType::None,
                longLengthPos: 0,
            },
            partitions: [0; ZSTD_MAX_NB_BLOCK_SPLITS],
            entropyMetadata: ZSTD_entropyCTablesMetadata_t {
                hufMetadata: ZSTD_hufCTablesMetadata_t {
                    hType: SymbolEncodingType::Basic,
                    hufDesBuffer: [0; ZSTD_MAX_HUF_HEADER_SIZE],
                    hufDesSize: 0,
                },
                fseMetadata: ZSTD_fseCTablesMetadata_t {
                    llType: SymbolEncodingType::Basic,
                    ofType: SymbolEncodingType::Basic,
                    mlType: SymbolEncodingType::Basic,
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
    // can't free ctxBody itself, as it's on stack; free only heap content
    ZSTD_freeCCtxContent(&mut ctxBody);

    result
}

/// Estimate amount of memory that will be needed to create a dictionary with following arguments
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateCDictSize_advanced))]
pub unsafe extern "C" fn ZSTD_estimateCDictSize_advanced(
    dictSize: size_t,
    cParams: ZSTD_compressionParameters,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
) -> size_t {
    (ZSTD_cwksp_alloc_size(size_of::<ZSTD_CDict>()))
        .wrapping_add(ZSTD_cwksp_alloc_size(HUF_WORKSPACE_SIZE))
        // enableDedicatedDictSearch == 1 ensures that CDict estimation will not be too small
        // in case we are using DDS with row-hash.
        .wrapping_add(ZSTD_sizeof_matchState(
            &cParams,
            ZSTD_resolveRowMatchFinderMode(ParamSwitch::Auto, &cParams),
            1,
            0,
        ))
        .wrapping_add(if dictLoadMethod == ZSTD_dlm_byRef {
            0
        } else {
            ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(
                dictSize,
                size_of::<*mut core::ffi::c_void>(),
            ))
        })
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
        CParamMode::CreateCDict,
    );
    ZSTD_estimateCDictSize_advanced(dictSize, cParams, ZSTD_dlm_byCopy)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_CDict))]
pub unsafe extern "C" fn ZSTD_sizeof_CDict(cdict: *const ZSTD_CDict) -> size_t {
    if cdict.is_null() {
        return 0;
    }

    // cdict may be in the workspace
    (if (*cdict).workspace.workspace == cdict as *mut core::ffi::c_void {
        0
    } else {
        size_of::<ZSTD_CDict>()
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
    if dictLoadMethod == ZSTD_dlm_byRef || dictBuffer.is_null() || dictSize == 0 {
        (*cdict).dictContent = dictBuffer;
    } else {
        let internalBuffer = ZSTD_cwksp_reserve_object(
            &mut (*cdict).workspace,
            ZSTD_cwksp_align(dictSize, size_of::<*mut core::ffi::c_void>()),
        );
        if internalBuffer.is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*cdict).dictContent = internalBuffer;
        core::ptr::copy_nonoverlapping(
            dictBuffer.cast::<u8>(),
            internalBuffer.cast::<u8>(),
            dictSize,
        );
    }
    (*cdict).dictContentSize = dictSize;
    (*cdict).dictContentType = dictContentType;

    (*cdict).entropyWorkspace =
        ZSTD_cwksp_reserve_object(&mut (*cdict).workspace, HUF_WORKSPACE_SIZE) as *mut u32;

    // Reset the state to no dictionary
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

    //(Maybe) load the dictionary
    // Skips loading the dictionary if it is < 8 bytes.
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
        DictTableLoadMethod::Full,
        TableFillPurpose::ForCDict,
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
    useRowMatchFinder: ParamSwitch,
    enableDedicatedDictSearch: core::ffi::c_int,
    customMem: ZSTD_customMem,
) -> *mut ZSTD_CDict {
    let workspaceSize = (ZSTD_cwksp_alloc_size(size_of::<ZSTD_CDict>()))
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
                size_of::<*mut core::ffi::c_void>(),
            ))
        });
    let workspace = ZSTD_customMalloc(workspaceSize, customMem);
    let mut ws = ZSTD_cwksp::default();

    if workspace.is_null() {
        return core::ptr::null_mut();
    }

    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, CwkspAllocKind::Dynamic);

    let cdict = ZSTD_cwksp_reserve_object(&mut ws, size_of::<ZSTD_CDict>()) as *mut ZSTD_CDict;
    ZSTD_cwksp_move(&mut (*cdict).workspace, &mut ws);
    (*cdict).customMem = customMem;
    (*cdict).compressionLevel = ZSTD_NO_CLEVEL; // signals advanced API usage
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
            enableLdm: ParamSwitch::Auto,
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
        postBlockSplitter: ParamSwitch::Auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ParamSwitch::Auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ParamSwitch::Auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ParamSwitch::Auto,
    };
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
        cParams = ZSTD_getCParamsFromCCtxParams_internal(
            &cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN,
            dictSize,
            CParamMode::CreateCDict,
        );
    }

    if !ZSTD_dedicatedDictSearch_isSupported(&cParams) {
        // Fall back to non-DDSS params
        cctxParams.enableDedicatedDictSearch = 0;
        cParams = ZSTD_getCParamsFromCCtxParams_internal(
            &cctxParams,
            ZSTD_CONTENTSIZE_UNKNOWN,
            dictSize,
            CParamMode::CreateCDict,
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
        CParamMode::CreateCDict,
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
        CParamMode::CreateCDict,
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
    if !cdictInWorkspace {
        ZSTD_customFree(
            cdict as *mut core::ffi::c_void,
            (*cdict).dictContentSize,
            cMem,
        );
    }
    0
}

/// Generate a digested dictionary in provided memory area.
/// workspace: The memory area to emplace the dictionary into.
///            Provided pointer must 8-bytes aligned.
///            It must outlive dictionary usage.
/// workspaceSize: Use ZSTD_estimateCDictSize() to determine how large workspace must be.
/// cParams: use ZSTD_getCParams() to transform a compression level into its relevant cParams.
///
/// # Returns
///
/// pointer to ZSTD_CDict*, or NULL if error (size too small).
///
/// Note: There is no corresponding "free" function.
/// Since workspace was allocated externally, it must be freed externally.
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
    let useRowMatchFinder = ZSTD_resolveRowMatchFinderMode(ParamSwitch::Auto, &cParams);
    // enableDedicatedDictSearch == 1 ensures matchstate is not too small in case this CDict will be used for DDS + row hash
    let matchStateSize = ZSTD_sizeof_matchState(&cParams, useRowMatchFinder, 1, 0);
    let neededSize = (ZSTD_cwksp_alloc_size(size_of::<ZSTD_CDict>()))
        .wrapping_add(if dictLoadMethod == ZSTD_dlm_byRef {
            0
        } else {
            ZSTD_cwksp_alloc_size(ZSTD_cwksp_align(
                dictSize,
                size_of::<*mut core::ffi::c_void>(),
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
            enableLdm: ParamSwitch::Auto,
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
        postBlockSplitter: ParamSwitch::Auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ParamSwitch::Auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ParamSwitch::Auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ParamSwitch::Auto,
    };

    // 8-aligned
    if workspace as size_t & 7 != 0 {
        return core::ptr::null();
    }

    let mut ws = ZSTD_cwksp::default();
    ZSTD_cwksp_init(&mut ws, workspace, workspaceSize, CwkspAllocKind::Static);
    cdict = ZSTD_cwksp_reserve_object(&mut ws, size_of::<ZSTD_CDict>()) as *mut ZSTD_CDict;
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

/// Provides the dictID of the dictionary loaded into `cdict`.
/// Non-conformant dictionaries can still be loaded, but as content-only dictionaries.
///
/// # Returns
///
/// dictID, or 0 if the dictionary is not conformant to Zstandard specification, or empty.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getDictID_fromCDict))]
pub unsafe extern "C" fn ZSTD_getDictID_fromCDict(cdict: *const ZSTD_CDict) -> core::ffi::c_uint {
    if cdict.is_null() {
        return 0;
    }
    (*cdict).dictID
}

/// Implementation of various ZSTD_compressBegin_usingCDict* functions.
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
            enableLdm: ParamSwitch::Auto,
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
        postBlockSplitter: ParamSwitch::Auto,
        preBlockSplitter_level: 0,
        maxBlockSize: 0,
        useRowMatchFinder: ParamSwitch::Auto,
        deterministicRefPrefix: 0,
        customMem: ZSTD_customMem::default(),
        prefetchCDictTables: ParamSwitch::Auto,
        enableMatchFinderFallback: 0,
        extSeqProdState: core::ptr::null_mut::<core::ffi::c_void>(),
        extSeqProdFunc: None,
        searchForExternalRepcodes: ParamSwitch::Auto,
    };
    if cdict.is_null() {
        return Error::dictionary_wrong.to_error_code();
    }

    // Initialize the cctxParams from the cdict
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

    // Increase window log to fit the entire dictionary and source if the
    // source size is known. Limit the increase to 19, which is the
    // window log for compression level 1 with the largest source size.
    if pledgedSrcSize != ZSTD_CONTENTSIZE_UNKNOWN {
        let limitedSrcSize = (pledgedSrcSize.min((1 << 19) as core::ffi::c_ulonglong)) as u32;
        let limitedSrcLog = if limitedSrcSize > 1 {
            (ZSTD_highbit32(limitedSrcSize.wrapping_sub(1))).wrapping_add(1)
        } else {
            1
        };
        cctxParams.cParams.windowLog = cctxParams.cParams.windowLog.max(limitedSrcLog);
    }

    ZSTD_compressBegin_internal(
        cctx,
        core::ptr::null(),
        0,
        ZSTD_dct_auto,
        DictTableLoadMethod::Fast,
        cdict,
        &cctxParams,
        pledgedSrcSize,
        BufferedPolicy::NotBuffered,
    )
}

/// This function is DEPRECATED.
/// cdict must be != NULL
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_compressBegin_usingCDict_advanced))]
pub unsafe extern "C" fn ZSTD_compressBegin_usingCDict_advanced(
    cctx: *mut ZSTD_CCtx,
    cdict: *const ZSTD_CDict,
    fParams: ZSTD_frameParameters,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    ZSTD_compressBegin_usingCDict_internal(cctx, cdict, fParams, pledgedSrcSize)
}

/// cdict must be != NULL
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

/// Implementation of various ZSTD_compress_usingCDict* functions.
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

/// This function is DEPRECATED.
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

/// Compression using a digested Dictionary.
/// Faster startup than ZSTD_compress_usingDict(), recommended when same dictionary is used multiple times.
/// Note that compression parameters are decided at CDict creation time
/// while frame parameters are hardcoded.
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
pub extern "C" fn ZSTD_CStreamInSize() -> size_t {
    ZSTD_BLOCKSIZE_MAX as size_t
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_CStreamOutSize))]
pub extern "C" fn ZSTD_CStreamOutSize() -> size_t {
    (ZSTD_compressBound(ZSTD_BLOCKSIZE_MAX as size_t))
        .wrapping_add(ZSTD_blockHeaderSize)
        .wrapping_add(4)
}

unsafe fn ZSTD_getCParamMode(
    cdict: *const ZSTD_CDict,
    params: &ZSTD_CCtx_params,
    pledgedSrcSize: u64,
) -> CParamMode {
    if !cdict.is_null() && ZSTD_shouldAttachDict(cdict, params, pledgedSrcSize) {
        CParamMode::AttachDict
    } else {
        CParamMode::NoAttachDict
    }
}

/// pledgedSrcSize == 0 means "unknown"
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_resetCStream))]
pub unsafe extern "C" fn ZSTD_resetCStream(
    zcs: *mut ZSTD_CStream,
    pss: core::ffi::c_ulonglong,
) -> size_t {
    // temporary: 0 interpreted as "unknown" during transition period.
    // Users willing to specify "unknown" **must** use ZSTD_CONTENTSIZE_UNKNOWN.
    // 0 will be interpreted as "empty" in the future.
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

/// For lib/compress only. Used by zstdmt_compress.c.
/// Assumption 1: params are valid.
/// Assumption 2: either dict, or cdict, is defined, not both.
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

/// same as ZSTD_initCStream_usingCDict(), with control over frame parameters
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

/// cdict must outlive compression session
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

/// pledgedSrcSize must be exact.
/// if srcSize is not known at init time, use value ZSTD_CONTENTSIZE_UNKNOWN.
/// dict is loaded with default parameters ZSTD_dct_auto and ZSTD_dlm_byCopy.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initCStream_advanced))]
pub unsafe extern "C" fn ZSTD_initCStream_advanced(
    zcs: *mut ZSTD_CStream,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    params: ZSTD_parameters,
    pss: core::ffi::c_ulonglong,
) -> size_t {
    // for compatibility with older programs relying on this behavior.
    // Users should now specify ZSTD_CONTENTSIZE_UNKNOWN.
    // This line will be removed in the future.
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
    // temporary : 0 interpreted as "unknown" during transition period.
    // Users willing to specify "unknown" **must** use ZSTD_CONTENTSIZE_UNKNOWN.
    // 0 will be interpreted as "empty" in the future.
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
    if (*cctx).appliedParams.inBufferMode == ZSTD_bm_stable {
        return ((*cctx).blockSizeMax).wrapping_sub((*cctx).stableIn_notConsumed);
    }
    let mut hintInSize = ((*cctx).inBuffTarget).wrapping_sub((*cctx).inBuffPos);
    if hintInSize == 0 {
        hintInSize = (*cctx).blockSizeMax;
    }
    hintInSize
}

/// Internal function for all *compressStream*() variants.
///
/// # Returns
///
/// The hint size for next input to complete ongoing block
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
    let mut someMoreWork = true;

    if (*zcs).appliedParams.inBufferMode == ZSTD_bm_stable {
        (*input).pos = ((*input).pos).wrapping_sub((*zcs).stableIn_notConsumed);
        if !ip.is_null() {
            ip = ip.sub((*zcs).stableIn_notConsumed);
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

    while someMoreWork {
        let mut current_block_156: u64;
        match (*zcs).streamStage {
            StreamStage::Init => return Error::init_missing.to_error_code(),
            StreamStage::Load => {
                if flushMode == ZSTD_e_end
                    && (oend.offset_from_unsigned(op)
                        >= ZSTD_compressBound(iend.offset_from_unsigned(ip))
                        || (*zcs).appliedParams.outBufferMode == ZSTD_bm_stable)
                    && (*zcs).inBuffPos == 0
                {
                    // shortcut to compression pass directly into output buffer
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
                    someMoreWork = false;
                    current_block_156 = 16754622181974910496;
                } else {
                    if (*zcs).appliedParams.inBufferMode == ZSTD_bm_buffered {
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
                        if flushMode == ZSTD_e_continue && (*zcs).inBuffPos < (*zcs).inBuffTarget {
                            someMoreWork = false;
                            current_block_156 = 16754622181974910496;
                        } else if flushMode == ZSTD_e_flush
                            && (*zcs).inBuffPos == (*zcs).inToCompress
                        {
                            someMoreWork = false;
                            current_block_156 = 16754622181974910496;
                        } else {
                            current_block_156 = 13910774313357589740;
                        }
                    } else if flushMode == ZSTD_e_continue
                        && (iend.offset_from_unsigned(ip)) < (*zcs).blockSizeMax
                    {
                        (*zcs).stableIn_notConsumed = iend.offset_from_unsigned(ip);
                        ip = iend;
                        someMoreWork = false;
                        current_block_156 = 16754622181974910496;
                    } else if flushMode == ZSTD_e_flush && ip == iend {
                        someMoreWork = false;
                        current_block_156 = 16754622181974910496;
                    } else {
                        current_block_156 = 13910774313357589740;
                    }
                    match current_block_156 {
                        16754622181974910496 => {}
                        _ => {
                            let inputBuffered = ((*zcs).appliedParams.inBufferMode
                                == ZSTD_bm_buffered)
                                as core::ffi::c_int;
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
                                || (*zcs).appliedParams.outBufferMode == ZSTD_bm_stable
                            {
                                cDst = op as *mut core::ffi::c_void; // compress into output buffer, to skip flush stage
                            } else {
                                cDst = (*zcs).outBuff as *mut core::ffi::c_void;
                                oSize = (*zcs).outBuffSize;
                            }
                            if inputBuffered != 0 {
                                let lastBlock = flushMode == ZSTD_e_end && ip == iend;
                                cSize_0 = if lastBlock {
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
                                (*zcs).frameEnded = lastBlock as u32;
                                (*zcs).inBuffTarget =
                                    ((*zcs).inBuffPos).wrapping_add((*zcs).blockSizeMax);
                                if (*zcs).inBuffTarget > (*zcs).inBuffSize {
                                    (*zcs).inBuffPos = 0;
                                    (*zcs).inBuffTarget = (*zcs).blockSizeMax;
                                }
                                if !lastBlock {
                                    assert!((*zcs).inBuffTarget <= (*zcs).inBuffSize);
                                }
                                (*zcs).inToCompress = (*zcs).inBuffPos;
                            } else {
                                let lastBlock_0 = (flushMode == ZSTD_e_end && ip.add(iSize) == iend)
                                    as core::ffi::c_int
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
                                    someMoreWork = false;
                                    ZSTD_CCtx_reset(
                                        zcs,
                                        ZSTD_ResetDirective::ZSTD_reset_session_only,
                                    );
                                }
                                current_block_156 = 16754622181974910496;
                            } else {
                                (*zcs).outBuffContentSize = cSize_0;
                                (*zcs).outBuffFlushedSize = 0;
                                (*zcs).streamStage = StreamStage::Flush; // pass-through to flush stage
                                current_block_156 = 5431927413890720344;
                            }
                        }
                    }
                }
            }
            StreamStage::Flush => {
                current_block_156 = 5431927413890720344;
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
                // flush not fully completed, presumably because dst is too small
                someMoreWork = false;
            } else {
                (*zcs).outBuffFlushedSize = 0;
                (*zcs).outBuffContentSize = (*zcs).outBuffFlushedSize;
                if (*zcs).frameEnded != 0 {
                    someMoreWork = false;
                    ZSTD_CCtx_reset(zcs, ZSTD_ResetDirective::ZSTD_reset_session_only);
                } else {
                    (*zcs).streamStage = StreamStage::Load;
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

/// After a compression call, set the expected input/output buffer.
/// This is validated at the start of the next compression call.
unsafe fn ZSTD_setBufferExpectations(
    cctx: *mut ZSTD_CCtx,
    output: *const ZSTD_outBuffer,
    input: *const ZSTD_inBuffer,
) {
    if (*cctx).appliedParams.inBufferMode == ZSTD_bm_stable {
        (*cctx).expectedInBuffer = *input;
    }

    if (*cctx).appliedParams.outBufferMode == ZSTD_bm_stable {
        (*cctx).expectedOutBufferSize = ((*output).size).wrapping_sub((*output).pos);
    }
}

/// Validate that the input/output buffers match the expectations set by
/// ZSTD_setBufferExpectations.
unsafe fn ZSTD_checkBufferStability(
    cctx: *const ZSTD_CCtx,
    output: *const ZSTD_outBuffer,
    input: *const ZSTD_inBuffer,
    _endOp: ZSTD_EndDirective,
) -> size_t {
    if (*cctx).appliedParams.inBufferMode == ZSTD_bm_stable {
        let expect = (*cctx).expectedInBuffer;
        if expect.src != (*input).src || expect.pos != (*input).pos {
            return Error::stabilityCondition_notRespected.to_error_code();
        }
    }

    if (*cctx).appliedParams.outBufferMode == ZSTD_bm_stable {
        let outBufferSize = ((*output).size).wrapping_sub((*output).pos);
        if (*cctx).expectedOutBufferSize != outBufferSize {
            return Error::stabilityCondition_notRespected.to_error_code();
        }
    }

    0
}

/// If @endOp == ZSTD_e_end, @inSize becomes pledgedSrcSize.
/// Otherwise, it's ignored.
///
/// # Returns
///
/// 0 on success, or a ZSTD_error code otherwise.
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
        size_of::<ZSTD_prefixDict>(),
    );
    if !((*cctx).cdict).is_null() && ((*cctx).localDict.cdict).is_null() {
        // Let the cdict's compression level take priority over the requested params.
        // But do not take the cdict's compression level if the "cdict" is actually a localDict
        // generated from ZSTD_initLocalDict().
        params.compressionLevel = (*(*cctx).cdict).compressionLevel;
    }

    if endOp == ZSTD_e_end {
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
    params.cParams = ZSTD_getCParamsFromCCtxParams_internal(
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

    if ZSTD_hasExtSeqProd(&params) && params.nbWorkers >= 1 {
        return Error::parameter_combination_unsupported.to_error_code();
    }

    if ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1)
        <= ZSTDMT_JOBSIZE_MIN as core::ffi::c_ulonglong
    {
        // do not invoke multi-threading when src size is too small
        params.nbWorkers = 0;
    }
    if params.nbWorkers > 0 {
        (*cctx).traceCtx = ZSTD_trace_compress_begin(cctx);

        // mt context creation
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

        // mt compression
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
        (*cctx).streamStage = StreamStage::Load;
        (*cctx).appliedParams = params;
    } else {
        let pledgedSrcSize = ((*cctx).pledgedSrcSizePlusOne).wrapping_sub(1);
        let err_code_1 = ZSTD_compressBegin_internal(
            cctx,
            prefixDict.dict,
            prefixDict.dictSize,
            prefixDict.dictContentType,
            DictTableLoadMethod::Fast,
            (*cctx).cdict,
            &params,
            pledgedSrcSize,
            BufferedPolicy::Buffered,
        );
        if ERR_isError(err_code_1) {
            return err_code_1;
        }

        (*cctx).inToCompress = 0;
        (*cctx).inBuffPos = 0;

        if (*cctx).appliedParams.inBufferMode == ZSTD_bm_buffered {
            // for small input: avoid automatic flush on reaching end of block, since
            // it would require to add a 3-bytes null block to end frame
            (*cctx).inBuffTarget = ((*cctx).blockSizeMax).wrapping_add(
                ((*cctx).blockSizeMax as u64 == pledgedSrcSize) as core::ffi::c_int as size_t,
            );
        } else {
            (*cctx).inBuffTarget = 0;
        }

        (*cctx).outBuffFlushedSize = 0;
        (*cctx).outBuffContentSize = (*cctx).outBuffFlushedSize;
        (*cctx).streamStage = StreamStage::Load;
        (*cctx).frameEnded = 0;
    }

    0
}

/// # Returns
///
/// The minimum amount of data remaining to be flushed from internal buffers
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

    // transparent initialization stage
    if (*cctx).streamStage == StreamStage::Init {
        // no obligation to start from pos==0
        let inputSize = ((*input).size).wrapping_sub((*input).pos);
        let totalInputSize = inputSize.wrapping_add((*cctx).stableIn_notConsumed);
        if (*cctx).requestedParams.inBufferMode == ZSTD_bm_stable
            && endOp == ZSTD_e_continue
            && totalInputSize < ZSTD_BLOCKSIZE_MAX as size_t
        {
            if (*cctx).stableIn_notConsumed != 0 {
                // check stable source guarantees
                if (*input).src != (*cctx).expectedInBuffer.src {
                    return -(ZSTD_error_stabilityCondition_notRespected as core::ffi::c_int)
                        as size_t;
                }
                if (*input).pos != (*cctx).expectedInBuffer.size {
                    return -(ZSTD_error_stabilityCondition_notRespected as core::ffi::c_int)
                        as size_t;
                }
            }
            // pretend input was consumed, to give a sense forward progress
            (*input).pos = (*input).size;
            // save stable inBuffer, for later control, and flush/end
            (*cctx).expectedInBuffer = *input;
            // but actually input wasn't consumed, so keep track of position from where compression shall resume
            (*cctx).stableIn_notConsumed = ((*cctx).stableIn_notConsumed).wrapping_add(inputSize);
            // don't initialize yet, wait for the first block of flush() order, for better parameters adaptation
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

    // compression stage
    if (*cctx).appliedParams.nbWorkers > 0 {
        let mut flushMin: size_t = 0;
        if (*cctx).cParamsChanged != 0 {
            ZSTDMT_updateCParams_whileCompressing((*cctx).mtctx, &(*cctx).requestedParams);
            (*cctx).cParamsChanged = 0;
        }
        if (*cctx).stableIn_notConsumed != 0 {
            // some early data was skipped - make it available for consumption
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
            if ERR_isError(flushMin) || endOp == ZSTD_e_end && flushMin == 0 {
                if flushMin == 0 {
                    ZSTD_CCtx_trace(cctx, 0);
                }
                ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
            }
            let err_code_1 = flushMin;
            if ERR_isError(err_code_1) {
                return err_code_1;
            }

            if endOp == ZSTD_e_continue {
                // We only require some progress with ZSTD_e_continue, not maximal progress.
                // We're done if we've consumed or produced any bytes, or either buffer is full.
                if (*input).pos != ipos
                    || (*output).pos != opos
                    || (*input).pos == (*input).size
                    || (*output).pos == (*output).size
                {
                    break;
                }
            } else if flushMin == 0 || (*output).pos == (*output).size {
                // We require maximal progress. We're done when the flush is complete or the
                // output buffer is full.
                break;
            }
        }

        // Either we don't require maximum forward progress, we've finished the
        // flush, or we are out of output space.
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

    // ZSTD_compressStream2() will check validity of dstPos and srcPos
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

    // Enable stable input/output buffers.
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

    // Reset to the original values.
    (*cctx).requestedParams.inBufferMode = originalInBufferMode;
    (*cctx).requestedParams.outBufferMode = originalOutBufferMode;

    let err_code = result;
    if ERR_isError(err_code) {
        return err_code;
    }
    if result != 0 {
        // compression not completed, due to lack of output space
        return Error::dstSize_tooSmall.to_error_code();
    }

    oPos
}

/// offBase must use the format required by ZSTD_storeSeq().
///
/// # Returns
///
/// 0, or a ZSTD error code if sequence is not valid
fn ZSTD_validateSequence(
    offBase: u32,
    matchLength: u32,
    minMatch: u32,
    posInSrc: size_t,
    windowLog: u32,
    dictSize: size_t,
    useSequenceProducer: bool,
) -> size_t {
    let windowSize = 1 << windowLog;
    // posInSrc represents the amount of data the decoder would decode up to this point.
    // As long as the amount of data decoded is less than or equal to window size, offsets may be
    // larger than the total length of output decoded in order to reference the dict, even larger than
    // window size. After output surpasses windowSize, we're limited to windowSize offsets again.
    let offsetBound = if posInSrc > windowSize as size_t {
        windowSize as size_t
    } else {
        posInSrc.wrapping_add(dictSize)
    };
    let matchLenLowerBound = (if minMatch == 3 || useSequenceProducer {
        3
    } else {
        4
    }) as size_t;

    if offBase as size_t > offsetBound.wrapping_add(3) {
        return Error::externalSequences_invalid.to_error_code();
    }

    // Validate maxNbSeq is large enough for the given matchLength and minMatch
    if (matchLength as size_t) < matchLenLowerBound {
        return Error::externalSequences_invalid.to_error_code();
    }

    0
}

/// Returns an offset code, given a sequence's raw offset, the ongoing repcode array, and whether
/// litLength == 0
fn ZSTD_finalizeOffBase(rawOffset: u32, rep: &[u32; 3], ll0: u32) -> u32 {
    let mut offBase = rawOffset.wrapping_add(ZSTD_REP_NUM as u32);

    if ll0 == 0 && rawOffset == rep[0] {
        offBase = REPCODE1_TO_OFFBASE as u32;
    } else if rawOffset == rep[1] {
        offBase = 2u32.wrapping_sub(ll0);
    } else if rawOffset == rep[2] {
        offBase = 3u32.wrapping_sub(ll0);
    } else if ll0 != 0 && rawOffset == rep[0].wrapping_sub(1) {
        offBase = REPCODE3_TO_OFFBASE as u32;
    }

    offBase
}

/// This function scans through an array of ZSTD_Sequence,
/// storing the sequences it reads, until it reaches a block delimiter.
/// Note that the block delimiter includes the last literals of the block.
/// `blockSize` must be == sum(sequence_lengths).
///
/// # Returns
///
/// `blockSize` on success, or a ZSTD_error
unsafe fn ZSTD_transferSequences_wBlockDelim(
    cctx: *mut ZSTD_CCtx,
    seqPos: *mut ZSTD_SequencePosition,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    src: *const core::ffi::c_void,
    blockSize: size_t,
    externalRepSearch: ParamSwitch,
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

    updatedRepcodes.rep = (*(*cctx).blockState.prevCBlock).rep;
    while (idx as size_t) < inSeqsSize
        && ((*inSeqs.offset(idx as isize)).matchLength != 0
            || (*inSeqs.offset(idx as isize)).offset != 0)
    {
        let litLength = (*inSeqs.offset(idx as isize)).litLength;
        let matchLength = (*inSeqs.offset(idx as isize)).matchLength;
        let mut offBase: u32 = 0;

        if externalRepSearch == ParamSwitch::Disable {
            offBase = ((*inSeqs.offset(idx as isize)).offset)
                .wrapping_add(ZSTD_REP_NUM as core::ffi::c_uint);
        } else {
            let ll0 = (litLength == 0) as core::ffi::c_int as u32;
            offBase = ZSTD_finalizeOffBase(
                (*inSeqs.offset(idx as isize)).offset,
                &updatedRepcodes.rep,
                ll0,
            );
            ZSTD_updateRep(&mut updatedRepcodes.rep, offBase, ll0);
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

    // If we skipped repcode search while parsing, we need to update repcodes now
    if externalRepSearch == ParamSwitch::Disable && idx != startIdx {
        let rep = &mut updatedRepcodes.rep;
        let lastSeqIdx = idx.wrapping_sub(1); // index of last non-block-delimiter sequence

        if lastSeqIdx >= startIdx.wrapping_add(2) {
            rep[2] = (*inSeqs.offset(lastSeqIdx.wrapping_sub(2) as isize)).offset;
            rep[1] = (*inSeqs.offset(lastSeqIdx.wrapping_sub(1) as isize)).offset;
            rep[0] = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else if lastSeqIdx == startIdx.wrapping_add(1) {
            rep[2] = rep[0];
            rep[1] = (*inSeqs.offset(lastSeqIdx.wrapping_sub(1) as isize)).offset;
            rep[0] = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else {
            rep[2] = rep[1];
            rep[1] = rep[0];
            rep[0] = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        }
    }

    (*(*cctx).blockState.nextCBlock).rep = updatedRepcodes.rep;

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

/// This function attempts to scan through @blockSize bytes in @src
/// represented by the sequences in @inSeqs, storing any (partial) sequences.
///
/// Occasionally, we may want to reduce the actual number of bytes consumed from @src
/// to avoid splitting a match, notably if it would produce a match smaller than MINMATCH.
///
/// # Returns
///
/// The number of bytes consumed from `src`, necessarily <= `blockSize`.
/// Otherwise, it may return a ZSTD error if something went wrong.
unsafe fn ZSTD_transferSequences_noDelim(
    cctx: *mut ZSTD_CCtx,
    seqPos: *mut ZSTD_SequencePosition,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    src: *const core::ffi::c_void,
    blockSize: size_t,
    externalRepSearch: ParamSwitch,
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
    let mut finalMatchSplit = false;

    /* TODO(embg) support fast parsing mode in noBlockDelim mode */
    let _ = externalRepSearch;

    if !((*cctx).cdict).is_null() {
        dictSize = (*(*cctx).cdict).dictContentSize;
    } else if !((*cctx).prefixDict.dict).is_null() {
        dictSize = (*cctx).prefixDict.dictSize;
    } else {
        dictSize = 0;
    }
    updatedRepcodes.rep = (*(*cctx).blockState.prevCBlock).rep;
    while endPosInSequence != 0 && (idx as size_t) < inSeqsSize && !finalMatchSplit {
        let currSeq = *inSeqs.offset(idx as isize);
        let mut litLength = currSeq.litLength;
        let mut matchLength = currSeq.matchLength;
        let rawOffset = currSeq.offset;
        let mut offBase: u32 = 0;

        // Modify the sequence depending on where endPosInSequence lies
        if endPosInSequence >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            if startPosInSequence >= litLength {
                startPosInSequence = startPosInSequence.wrapping_sub(litLength);
                litLength = 0;
                matchLength = matchLength.wrapping_sub(startPosInSequence);
            } else {
                litLength = litLength.wrapping_sub(startPosInSequence);
            }
            // Move to the next sequence
            endPosInSequence = (endPosInSequence as core::ffi::c_uint)
                .wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength));
            startPosInSequence = 0;
        } else {
            // This is the final (partial) sequence we're adding from inSeqs, and endPosInSequence
            // does not reach the end of the match. So, we have to split the sequence
            if endPosInSequence <= litLength {
                // This sequence ends inside the literals, break to store the last literals
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
                // Only ever split the match if it is larger than the block size
                let secondHalfMatchLength = (currSeq.matchLength)
                    .wrapping_add(currSeq.litLength)
                    .wrapping_sub(endPosInSequence);
                if secondHalfMatchLength < (*cctx).appliedParams.cParams.minMatch {
                    // Move the endPosInSequence backward so that it creates match of minMatch length
                    endPosInSequence = (endPosInSequence as core::ffi::c_uint).wrapping_sub(
                        ((*cctx).appliedParams.cParams.minMatch)
                            .wrapping_sub(secondHalfMatchLength),
                    );
                    bytesAdjustment = ((*cctx).appliedParams.cParams.minMatch)
                        .wrapping_sub(secondHalfMatchLength);
                    firstHalfMatchLength = firstHalfMatchLength.wrapping_sub(bytesAdjustment);
                }
                matchLength = firstHalfMatchLength;
                // Flag that we split the last match - after storing the sequence, exit the loop,
                // but keep the value of endPosInSequence
                finalMatchSplit = true;
            } else {
                // Move the position in sequence backwards so that we don't split match, and break to store
                // the last literals. We use the original currSeq.litLength as a marker for where endPosInSequence
                // should go. We prefer to do this whenever it is not necessary to split the match, or if doing so
                // would cause the first half of the match to be too small
                bytesAdjustment = endPosInSequence.wrapping_sub(currSeq.litLength);
                endPosInSequence = currSeq.litLength;
                break;
            }
        }

        // Check if this offset can be represented with a repcode
        let ll0 = (litLength == 0) as core::ffi::c_int as u32;
        offBase = ZSTD_finalizeOffBase(rawOffset, &updatedRepcodes.rep, ll0);
        ZSTD_updateRep(&mut updatedRepcodes.rep, offBase, ll0);

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
        if !finalMatchSplit {
            // Next Sequence
            idx = idx.wrapping_add(1);
        }
    }

    (*seqPos).idx = idx;
    (*seqPos).posInSequence = endPosInSequence;
    (*(*cctx).blockState.nextCBlock).rep = updatedRepcodes.rep;

    iend = iend.sub(bytesAdjustment as usize);
    if ip != iend {
        // Store any last literals
        let lastLLSize = iend.offset_from(ip) as core::ffi::c_long as u32;
        ZSTD_storeLastLiterals(&mut (*cctx).seqStore, ip, lastLLSize as size_t);
        (*seqPos).posInSrc = ((*seqPos).posInSrc).wrapping_add(lastLLSize as size_t);
    }

    iend.offset_from_unsigned(istart)
}

/// @seqPos represents a position within @inSeqs,
/// it is read and updated by this function,
/// once the goal to produce a block of size @blockSize is reached.
///
/// # Returns
///
/// The number of bytes consumed from @src, necessarily <= @blockSize.
fn ZSTD_selectSequenceCopier(mode: ZSTD_SequenceFormat_e) -> ZSTD_SequenceCopier_f {
    if mode == ZSTD_sf_explicitBlockDelimiters {
        return Some(
            ZSTD_transferSequences_wBlockDelim
                as unsafe fn(
                    *mut ZSTD_CCtx,
                    *mut ZSTD_SequencePosition,
                    *const ZSTD_Sequence,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                    ParamSwitch,
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
                ParamSwitch,
            ) -> size_t,
    )
}

/// Discover the size of next block by searching for the delimiter.
/// Note that a block delimiter **must** exist in this mode, otherwise it's an input error.
/// The block size retrieved will be later compared to ensure it remains within bounds
unsafe fn blockSize_explicitDelimiter(
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    seqPos: ZSTD_SequencePosition,
) -> size_t {
    let mut end = 0;
    let mut blockSize = 0usize;
    let mut spos = seqPos.idx as size_t;

    while spos < inSeqsSize {
        end = ((*inSeqs.add(spos)).offset == 0) as core::ffi::c_int;
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
    if mode == ZSTD_sf_noBlockDelimiters {
        // Note: more a "target" block size
        return remaining.min(blockSize);
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

/// Compress all provided sequences, block-by-block.
///
/// # Returns
///
/// The cumulative size of all compressed blocks (including their headers),
/// or a ZSTD error.
unsafe fn ZSTD_compressSequences_internal(
    cctx: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    inSeqs: *const ZSTD_Sequence,
    inSeqsSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut cSize = 0usize;
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

    // Special case: empty frame
    if remaining == 0 {
        let cBlockHeader24 = 1u32.wrapping_add((BlockType::Raw as u32) << 1);
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
        let lastBlock = (blockSize == remaining) as core::ffi::c_int as u32;
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

        // If blocks are too small, emit as a nocompress block
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
                && ZSTD_maybeRLE(&(*cctx).seqStore)
                && ZSTD_isRLE(ip, blockSize)
            {
                // Note: don't emit the first block as RLE even if it qualifies because
                // doing so will cause the decoder (cli <= v1.4.3 only) to throw an (invalid) error
                // "should consume all input error."
                compressedSeqsSize = 1;
            }

            if compressedSeqsSize == 0 {
                // ZSTD_noCompressBlock writes the block header as well
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
                // Error checking and repcodes update
                ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*cctx).blockState);
                if (*(*cctx).blockState.prevCBlock)
                    .entropy
                    .fse
                    .offcode_repeatMode
                    == FSE_repeat_valid
                {
                    (*(*cctx).blockState.prevCBlock)
                        .entropy
                        .fse
                        .offcode_repeatMode = FSE_repeat_check;
                }

                // Write block header into beginning of block
                cBlockHeader = lastBlock
                    .wrapping_add((BlockType::Compressed as u32) << 1)
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
    let mut cSize = 0usize;

    // Transparent initialization stage, same as compressStream2()
    let err_code = ZSTD_CCtx_init_compressStream2(cctx, ZSTD_e_end, srcSize);
    if ERR_isError(err_code) {
        return err_code;
    }

    // Begin writing output, starting with frame header
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

    // Now generate compressed blocks
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

    // Complete with frame checksum, if needed
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

/// Convert sequences from external ZSTD_Sequence format to internal SeqDef format:
///   - offset -> offBase = offset + ZSTD_REP_NUM
///   - litLength -> (U16) litLength
///   - matchLength -> (U16)(matchLength - MINMATCH)
///   - rep is ignored
///
/// Store only 8 bytes per SeqDef (offBase[4], litLength[2], mlBase[2]).
///
/// # Returns
///
/// - 0 on success, with no long length detected.
/// - > 0 if there is one long length (> 65535), indicating the position and type.
pub unsafe fn convertSequences_noRepcodes(
    dstSeqs: *mut SeqDef,
    inSeqs: *const ZSTD_Sequence,
    nbSequences: size_t,
) -> size_t {
    let mut longLen = 0;

    for n in 0..nbSequences {
        (*dstSeqs.add(n)).offBase =
            ((*inSeqs.add(n)).offset).wrapping_add(ZSTD_REP_NUM as core::ffi::c_uint);
        (*dstSeqs.add(n)).litLength = (*inSeqs.add(n)).litLength as u16;
        (*dstSeqs.add(n)).mlBase =
            ((*inSeqs.add(n)).matchLength).wrapping_sub(MINMATCH as core::ffi::c_uint) as u16;
        // Check for long length > 65535
        if (*inSeqs.add(n)).matchLength > 65535 + 3 {
            longLen = n.wrapping_add(1);
        }
        if (*inSeqs.add(n)).litLength > 65535 {
            longLen = n.wrapping_add(nbSequences).wrapping_add(1);
        }
    }

    longLen
}

/// Precondition: Sequences must end on an explicit Block Delimiter.
///
/// # Returns
///
/// 0 on success, or an error code.
///
/// Note: sequence validation functionality has been disabled (removed).
/// This is helpful to generate a lean main pipeline, improving performance.
/// It may be re-inserted later.
pub unsafe fn ZSTD_convertBlockSequences(
    cctx: *mut ZSTD_CCtx,
    inSeqs: *const ZSTD_Sequence,
    nbSequences: size_t,
    repcodeResolution: bool,
) -> size_t {
    let mut updatedRepcodes = repcodes_s { rep: [0; 3] };
    let mut seqNb = 0;

    if nbSequences >= (*cctx).seqStore.maxNbSeq {
        return Error::externalSequences_invalid.to_error_code();
    }

    updatedRepcodes.rep = (*(*cctx).blockState.prevCBlock).rep;

    // Convert Sequences from public format to internal format
    if !repcodeResolution {
        let longl = convertSequences_noRepcodes(
            (*cctx).seqStore.sequencesStart,
            inSeqs,
            nbSequences.wrapping_sub(1),
        );
        (*cctx).seqStore.sequences = ((*cctx).seqStore.sequencesStart).add(nbSequences).sub(1);
        if longl != 0 {
            if longl <= nbSequences.wrapping_sub(1) {
                (*cctx).seqStore.longLengthType = LongLengthType::Match;
                (*cctx).seqStore.longLengthPos = longl.wrapping_sub(1) as u32;
            } else {
                (*cctx).seqStore.longLengthType = LongLengthType::Literal;
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
            let ll0 = (litLength == 0) as core::ffi::c_int as u32;
            let offBase =
                ZSTD_finalizeOffBase((*inSeqs.add(seqNb)).offset, &updatedRepcodes.rep, ll0);
            ZSTD_storeSeqOnly(
                &mut (*cctx).seqStore,
                litLength as size_t,
                offBase,
                matchLength as size_t,
            );
            ZSTD_updateRep(&mut updatedRepcodes.rep, offBase, ll0);
            seqNb = seqNb.wrapping_add(1);
        }
    }

    // If we skipped repcode search while parsing, we need to update repcodes now
    if !repcodeResolution && nbSequences > 1 {
        let rep = &mut updatedRepcodes.rep;

        if nbSequences >= 4 {
            let lastSeqIdx = (nbSequences as u32).wrapping_sub(2); // index of last full sequence
            rep[2] = (*inSeqs.offset(lastSeqIdx.wrapping_sub(2) as isize)).offset;
            rep[1] = (*inSeqs.offset(lastSeqIdx.wrapping_sub(1) as isize)).offset;
            rep[0] = (*inSeqs.offset(lastSeqIdx as isize)).offset;
        } else if nbSequences == 3 {
            rep[2] = rep[0];
            rep[1] = (*inSeqs).offset;
            rep[0] = (*inSeqs.add(1)).offset;
        } else {
            rep[2] = rep[1];
            rep[1] = rep[0];
            rep[0] = (*inSeqs).offset;
        }
    }

    (*(*cctx).blockState.nextCBlock).rep = updatedRepcodes.rep;

    0
}

/// The function assumes `litMatchLength` is a packed 64-bit value where the
/// lower 32 bits represent the match length. The check varies based on the
/// system's endianness:
/// - On little-endian systems, it verifies if the entire 64-bit value is at most
///   0xFFFFFFFF, indicating the match length (lower 32 bits) is zero.
/// - On big-endian systems, it directly checks if the lower 32 bits are zero.
///
/// # Returns
///
/// `true` if the match length is zero
#[inline(always)]
const fn matchLengthHalfIsZero(litMatchLength: u64) -> bool {
    if cfg!(target_endian = "little") {
        litMatchLength <= 0xffffffff
    } else {
        litMatchLength as u32 == 0
    }
}

pub unsafe fn ZSTD_get1BlockSummary(seqs: *const ZSTD_Sequence, nbSeqs: size_t) -> BlockSummary {
    let mut current_block: u64;
    // Use multiple accumulators for efficient use of wide out-of-order machines.
    let mut litMatchSize0 = 0u64;
    let mut litMatchSize1 = 0u64;
    let mut litMatchSize2 = 0u64;
    let mut litMatchSize3 = 0u64;
    let mut n = 0usize;

    if nbSeqs > 3 as size_t {
        // Process the input in 4 independent streams to reach high throughput.
        loop {
            let mut litMatchLength = MEM_read64(
                &(*seqs.add(n)).litLength as *const core::ffi::c_uint as *const core::ffi::c_void,
            );
            litMatchSize0 = litMatchSize0.wrapping_add(litMatchLength);
            if matchLengthHalfIsZero(litMatchLength) {
                current_block = 13744635599856597681;
                break;
            }

            litMatchLength = MEM_read64(
                &(*seqs.add(n.wrapping_add(1))).litLength as *const core::ffi::c_uint
                    as *const core::ffi::c_void,
            );
            litMatchSize1 = litMatchSize1.wrapping_add(litMatchLength);
            if matchLengthHalfIsZero(litMatchLength) {
                n = n.wrapping_add(1);
                current_block = 13744635599856597681;
                break;
            } else {
                litMatchLength = MEM_read64(
                    &(*seqs.add(n.wrapping_add(2))).litLength as *const core::ffi::c_uint
                        as *const core::ffi::c_void,
                );
                litMatchSize2 = litMatchSize2.wrapping_add(litMatchLength);
                if matchLengthHalfIsZero(litMatchLength) {
                    n = n.wrapping_add(2);
                    current_block = 13744635599856597681;
                    break;
                } else {
                    litMatchLength = MEM_read64(
                        &(*seqs.add(n.wrapping_add(3))).litLength as *const core::ffi::c_uint
                            as *const core::ffi::c_void,
                    );
                    litMatchSize3 = litMatchSize3.wrapping_add(litMatchLength);
                    if matchLengthHalfIsZero(litMatchLength) {
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
                if cfg!(target_endian = "little") {
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
                    if matchLengthHalfIsZero(litMatchLength_0) {
                        current_block = 13744635599856597681;
                        continue;
                    }
                    n = n.wrapping_add(1);
                    current_block = 2668756484064249700;
                } else {
                    // At this point n == nbSeqs, so no end terminator.
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
    let mut cSize = 0usize;
    let mut op = dst as *mut u8;
    let repcodeResolution = (*cctx).appliedParams.searchForExternalRepcodes == ParamSwitch::Enable;

    if nbSequences == 0 {
        return Error::externalSequences_invalid.to_error_code();
    }

    // Special case: empty frame
    if nbSequences == 1 && (*inSeqs).litLength == 0 {
        let cBlockHeader24 = 1u32.wrapping_add((BlockType::Raw as u32) << 1);
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
        let lastBlock = (block.nbSequences == nbSequences) as core::ffi::c_int as u32;
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

        // Note: when blockSize is very small, other variant send it uncompressed.
        // Here, we still send the sequences, because we don't have the original source to send it uncompressed.
        // One could imagine in theory reproducing the source from the sequences,
        // but that's complex and costly memory intensive, and goes against the objectives of this variant.

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
        // Note: the spec forbids for any compressed block to be larger than maximum block size
        if compressedSeqsSize > (*cctx).blockSizeMax {
            compressedSeqsSize = 0;
        }
        litSize = litSize.wrapping_sub(block.litSize);
        literals =
            (literals as *const core::ffi::c_char).add(block.litSize) as *const core::ffi::c_void;

        // Note: difficult to check source for RLE block when only Literals are provided,
        // but it could be considered from analyzing the sequence directly

        if compressedSeqsSize == 0 {
            // Sending uncompressed blocks is out of reach, because the source is not provided.
            // In theory, one could use the sequences to regenerate the source, like a decompressor,
            // but it's complex, and memory hungry, killing the purpose of this variant.
            // Current outcome: generate an error code.
            return Error::cannotProduce_uncompressedBlock.to_error_code();
        } else {
            let mut cBlockHeader: u32 = 0;
            // Error checking and repcodes update
            ZSTD_blockState_confirmRepcodesAndEntropyTables(&mut (*cctx).blockState);
            if (*(*cctx).blockState.prevCBlock)
                .entropy
                .fse
                .offcode_repeatMode
                == FSE_repeat_valid
            {
                (*(*cctx).blockState.prevCBlock)
                    .entropy
                    .fse
                    .offcode_repeatMode = FSE_repeat_check;
            }

            // Write block header into beginning of block
            cBlockHeader = lastBlock
                .wrapping_add((BlockType::Compressed as u32) << 1)
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
    let mut cSize = 0usize;

    // Transparent initialization stage, same as compressStream2()
    if litCapacity < litSize {
        return Error::workSpace_tooSmall.to_error_code();
    }
    let err_code = ZSTD_CCtx_init_compressStream2(cctx, ZSTD_e_end, decompressedSize);
    if ERR_isError(err_code) {
        return err_code;
    }

    if (*cctx).appliedParams.blockDelimiters == ZSTD_sf_noBlockDelimiters {
        return Error::frameParameter_unsupported.to_error_code();
    }
    if (*cctx).appliedParams.validateSequences != 0 {
        return Error::parameter_unsupported.to_error_code();
    }
    if (*cctx).appliedParams.fParams.checksumFlag != 0 {
        return Error::frameParameter_unsupported.to_error_code();
    }

    // Begin writing output, starting with frame header
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

    // Now generate compressed blocks
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
    let stableInput = ((*zcs).appliedParams.inBufferMode == ZSTD_bm_stable) as core::ffi::c_int;
    if stableInput != 0 {
        (*zcs).expectedInBuffer
    } else {
        nullInput
    }
}

/// # Returns
///
/// amount of data remaining to flush
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_flushStream))]
pub unsafe extern "C" fn ZSTD_flushStream(
    zcs: *mut ZSTD_CStream,
    output: *mut ZSTD_outBuffer,
) -> size_t {
    let mut input = inBuffer_forEndFlush(zcs);
    input.size = input.pos; // do not ingest more input during flush
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
        return remainingToFlush; // minimal estimation
    }

    // single thread mode: attempt to calculate remaining to flush more precisely
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

fn ZSTD_dedicatedDictSearch_isSupported(cParams: &ZSTD_compressionParameters) -> bool {
    cParams.strategy >= ZSTD_greedy
        && cParams.strategy <= ZSTD_lazy2
        && cParams.hashLog > cParams.chainLog
        && cParams.chainLog <= 24
}

/// Reverses the adjustment applied to cparams when enabling dedicated dict
/// search. This is used to recover the params set to be used in the working
/// context. (Otherwise, those tables would also grow.)
fn ZSTD_dedicatedDictSearch_revertCParams(cParams: &mut ZSTD_compressionParameters) {
    if let 3..=5 = cParams.strategy as core::ffi::c_uint {
        cParams.hashLog =
            (cParams.hashLog).wrapping_sub(ZSTD_LAZY_DDSS_BUCKET_LOG as core::ffi::c_uint);
        if cParams.hashLog < ZSTD_HASHLOG_MIN as core::ffi::c_uint {
            cParams.hashLog = ZSTD_HASHLOG_MIN as core::ffi::c_uint;
        }
    }
}

fn ZSTD_getCParamRowSize(srcSizeHint: u64, mut dictSize: size_t, mode: CParamMode) -> u64 {
    if mode == CParamMode::AttachDict {
        dictSize = 0;
    }

    let unknown = srcSizeHint as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN;
    let addedSize = if unknown && dictSize > 0 { 500 } else { 0 };
    if unknown && dictSize == 0 {
        ZSTD_CONTENTSIZE_UNKNOWN
    } else {
        srcSizeHint
            .wrapping_add(dictSize as u64)
            .wrapping_add(addedSize)
    }
}

/// Size values are optional, provide 0 if not known or unused.
///
/// # Returns
///
/// `ZSTD_compressionParameters` structure for a selected compression level, srcSize and dictSize.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getCParams))]
pub extern "C" fn ZSTD_getCParams(
    compressionLevel: core::ffi::c_int,
    mut srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
) -> ZSTD_compressionParameters {
    if srcSizeHint == 0 {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    ZSTD_getCParams_internal(compressionLevel, srcSizeHint, dictSize, CParamMode::Unknown)
}

/// Same idea as ZSTD_getCParams().
/// Fields of `ZSTD_frameParameters` are set to default values.
///
/// # Returns
///
/// a `ZSTD_parameters` structure (instead of `ZSTD_compressionParameters`).
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getParams))]
pub unsafe extern "C" fn ZSTD_getParams(
    compressionLevel: core::ffi::c_int,
    mut srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
) -> ZSTD_parameters {
    if srcSizeHint == 0 {
        srcSizeHint = ZSTD_CONTENTSIZE_UNKNOWN;
    }
    ZSTD_getParams_internal(compressionLevel, srcSizeHint, dictSize, CParamMode::Unknown)
}

pub const __INT_MAX__: core::ffi::c_int = 2147483647;

pub const ZSTD_MAX_CLEVEL: core::ffi::c_int = 22;

static ZSTD_defaultCParameters: [[ZSTD_compressionParameters; 23]; 4] = [
    // "default" - for any srcSize > 256 KB
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
    // for srcSize <= 256 KB
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
    // for srcSize <= 128 KB
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
    // for srcSize <= 16 KB
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

fn ZSTD_dedicatedDictSearch_getCParams(
    compressionLevel: core::ffi::c_int,
    dictSize: size_t,
) -> ZSTD_compressionParameters {
    let mut cParams =
        ZSTD_getCParams_internal(compressionLevel, 0, dictSize, CParamMode::CreateCDict);
    if let 3..=5 = cParams.strategy as core::ffi::c_uint {
        cParams.hashLog =
            (cParams.hashLog).wrapping_add(ZSTD_LAZY_DDSS_BUCKET_LOG as core::ffi::c_uint);
    }
    cParams
}

/// # Returns
///
/// `ZSTD_compressionParameters` structure for a selected compression level, srcSize and dictSize.
///
/// # Note
///
/// srcSizeHint 0 means 0, use ZSTD_CONTENTSIZE_UNKNOWN for unknown.
/// Use dictSize == 0 for unknown or unused.
/// `mode` controls how we treat the `dictSize`. See docs for [`CParamMode`].
fn ZSTD_getCParams_internal(
    compressionLevel: core::ffi::c_int,
    srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
    mode: CParamMode,
) -> ZSTD_compressionParameters {
    let rSize = ZSTD_getCParamRowSize(srcSizeHint, dictSize, mode);
    let tableID = ((rSize <= (256 * (1 << 10)) as u64) as core::ffi::c_int
        + (rSize <= (128 * (1 << 10)) as u64) as core::ffi::c_int
        + (rSize <= (16 * (1 << 10)) as u64) as core::ffi::c_int) as u32;

    let mut row: core::ffi::c_int = 0;
    if compressionLevel == 0 {
        row = ZSTD_CLEVEL_DEFAULT;
    } else if compressionLevel < 0 {
        row = 0; // entry 0 is baseline for fast mode
    } else if compressionLevel > ZSTD_MAX_CLEVEL {
        row = ZSTD_MAX_CLEVEL;
    } else {
        row = compressionLevel;
    }

    let mut cp = ZSTD_defaultCParameters[tableID as usize][row as usize];
    // acceleration factor
    if compressionLevel < 0 {
        let clampedCompressionLevel = ZSTD_minCLevel().max(compressionLevel);
        cp.targetLength = -clampedCompressionLevel as core::ffi::c_uint;
    }

    // refine parameters based on srcSize & dictSize
    ZSTD_adjustCParams_internal(cp, srcSizeHint, dictSize, mode, ParamSwitch::Auto)
}

/// Same idea as ZSTD_getCParams().
/// Fields of `ZSTD_frameParameters` are set to default values.
///
/// # Returns
///
/// a `ZSTD_parameters` structure (instead of `ZSTD_compressionParameters`).
fn ZSTD_getParams_internal(
    compressionLevel: core::ffi::c_int,
    srcSizeHint: core::ffi::c_ulonglong,
    dictSize: size_t,
    mode: CParamMode,
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
    }
}
