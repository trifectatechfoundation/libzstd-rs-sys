use core::arch::asm;

use crate::lib::common::entropy_common::FSE_readNCount;
use crate::lib::common::zstd_common::ZSTD_getErrorCode;
use crate::lib::decompress::zstd_ddict::ZSTD_DDict;
use crate::lib::decompress::zstd_decompress_block::ZSTD_buildFSETable;
use crate::lib::decompress::{
    zdss_flush, zdss_init, zdss_load, zdss_loadHeader, zdss_read, HUF_DTable, LL_base, ML_base,
    OF_base, OF_bits, ZSTD_dStage, ZSTD_dStreamStage, ZSTD_dictUses_e, ZSTD_dont_use,
    ZSTD_entropyDTables_t, ZSTD_litLocation_e, ZSTD_seqSymbol, ZSTD_use_indefinitely,
    ZSTD_use_once, ZSTDds_checkChecksum, ZSTDds_decodeBlockHeader, ZSTDds_decodeFrameHeader,
    ZSTDds_decodeSkippableHeader, ZSTDds_decompressBlock, ZSTDds_decompressLastBlock,
    ZSTDds_getFrameHeaderSize, ZSTDds_skipFrame,
};
use crate::lib::zstd::*;
use crate::{MEM_readLE16, MEM_readLE32, MEM_readLE64, MEM_writeLE32};
extern "C" {
    pub type ZBUFFv07_DCtx_s;
    pub type ZBUFFv06_DCtx_s;
    pub type ZBUFFv05_DCtx_s;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_CCtx_params_s;
    pub type ZSTDv07_DCtx_s;
    pub type ZSTDv06_DCtx_s;
    pub type ZSTDv05_DCtx_s;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> size_t;
    fn ZSTD_getDictID_fromDDict(ddict: *const ZSTD_DDict) -> std::ffi::c_uint;
    fn ZSTD_sizeof_DDict(ddict: *const ZSTD_DDict) -> size_t;
    fn ZSTD_createDDict_advanced(
        dict: *const std::ffi::c_void,
        dictSize: size_t,
        dictLoadMethod: ZSTD_dictLoadMethod_e,
        dictContentType: ZSTD_dictContentType_e,
        customMem: ZSTD_customMem,
    ) -> *mut ZSTD_DDict;
    fn ZSTD_checkContinuity(dctx: *mut ZSTD_DCtx, dst: *const std::ffi::c_void, dstSize: size_t);
    fn HUF_readDTableX2_wksp(
        DTable: *mut HUF_DTable,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        workSpace: *mut std::ffi::c_void,
        wkspSize: size_t,
        flags: std::ffi::c_int,
    ) -> size_t;
    fn ZSTD_getcBlockSize(
        src: *const std::ffi::c_void,
        srcSize: size_t,
        bpPtr: *mut blockProperties_t,
    ) -> size_t;
    fn ZSTD_XXH64(
        input: *const std::ffi::c_void,
        length: size_t,
        seed: XXH64_hash_t,
    ) -> XXH64_hash_t;
    fn ZSTD_XXH64_reset(statePtr: *mut XXH64_state_t, seed: XXH64_hash_t) -> XXH_errorcode;
    fn ZSTD_XXH64_update(
        statePtr: *mut XXH64_state_t,
        input: *const std::ffi::c_void,
        length: size_t,
    ) -> XXH_errorcode;
    fn ZSTD_XXH64_digest(statePtr: *const XXH64_state_t) -> XXH64_hash_t;
    fn ZSTD_trace_decompress_begin(dctx: *const ZSTD_DCtx_s) -> ZSTD_TraceCtx;
    fn ZSTD_trace_decompress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace);
    fn ZSTD_DDict_dictContent(ddict: *const ZSTD_DDict) -> *const std::ffi::c_void;
    fn ZSTD_DDict_dictSize(ddict: *const ZSTD_DDict) -> size_t;
    fn ZSTD_copyDDictParameters(dctx: *mut ZSTD_DCtx, ddict: *const ZSTD_DDict);
    fn ZSTD_decompressBlock_internal(
        dctx: *mut ZSTD_DCtx,
        dst: *mut std::ffi::c_void,
        dstCapacity: size_t,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        streaming: streaming_operation,
    ) -> size_t;
    fn ZSTDv05_findFrameSizeInfoLegacy(
        src: *const std::ffi::c_void,
        srcSize: size_t,
        cSize: *mut size_t,
        dBound: *mut std::ffi::c_ulonglong,
    );
    fn ZSTDv05_createDCtx() -> *mut ZSTDv05_DCtx;
    fn ZSTDv05_freeDCtx(dctx: *mut ZSTDv05_DCtx) -> size_t;
    fn ZSTDv05_decompress_usingDict(
        dctx: *mut ZSTDv05_DCtx,
        dst: *mut std::ffi::c_void,
        dstCapacity: size_t,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTDv05_getFrameParams(
        params: *mut ZSTDv05_parameters,
        src: *const std::ffi::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZBUFFv05_createDCtx() -> *mut ZBUFFv05_DCtx;
    fn ZBUFFv05_freeDCtx(dctx: *mut ZBUFFv05_DCtx) -> size_t;
    fn ZBUFFv05_decompressInitDictionary(
        dctx: *mut ZBUFFv05_DCtx,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZBUFFv05_decompressContinue(
        dctx: *mut ZBUFFv05_DCtx,
        dst: *mut std::ffi::c_void,
        dstCapacityPtr: *mut size_t,
        src: *const std::ffi::c_void,
        srcSizePtr: *mut size_t,
    ) -> size_t;
    fn ZSTDv06_findFrameSizeInfoLegacy(
        src: *const std::ffi::c_void,
        srcSize: size_t,
        cSize: *mut size_t,
        dBound: *mut std::ffi::c_ulonglong,
    );
    fn ZSTDv06_createDCtx() -> *mut ZSTDv06_DCtx;
    fn ZSTDv06_freeDCtx(dctx: *mut ZSTDv06_DCtx) -> size_t;
    fn ZSTDv06_decompress_usingDict(
        dctx: *mut ZSTDv06_DCtx,
        dst: *mut std::ffi::c_void,
        dstCapacity: size_t,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTDv06_getFrameParams(
        fparamsPtr: *mut ZSTDv06_frameParams,
        src: *const std::ffi::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZBUFFv06_createDCtx() -> *mut ZBUFFv06_DCtx;
    fn ZBUFFv06_freeDCtx(dctx: *mut ZBUFFv06_DCtx) -> size_t;
    fn ZBUFFv06_decompressInitDictionary(
        dctx: *mut ZBUFFv06_DCtx,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZBUFFv06_decompressContinue(
        dctx: *mut ZBUFFv06_DCtx,
        dst: *mut std::ffi::c_void,
        dstCapacityPtr: *mut size_t,
        src: *const std::ffi::c_void,
        srcSizePtr: *mut size_t,
    ) -> size_t;
    fn ZSTDv07_findFrameSizeInfoLegacy(
        src: *const std::ffi::c_void,
        srcSize: size_t,
        cSize: *mut size_t,
        dBound: *mut std::ffi::c_ulonglong,
    );
    fn ZSTDv07_createDCtx() -> *mut ZSTDv07_DCtx;
    fn ZSTDv07_freeDCtx(dctx: *mut ZSTDv07_DCtx) -> size_t;
    fn ZSTDv07_decompress_usingDict(
        dctx: *mut ZSTDv07_DCtx,
        dst: *mut std::ffi::c_void,
        dstCapacity: size_t,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTDv07_getFrameParams(
        fparamsPtr: *mut ZSTDv07_frameParams,
        src: *const std::ffi::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZBUFFv07_createDCtx() -> *mut ZBUFFv07_DCtx;
    fn ZBUFFv07_freeDCtx(dctx: *mut ZBUFFv07_DCtx) -> size_t;
    fn ZBUFFv07_decompressInitDictionary(
        dctx: *mut ZBUFFv07_DCtx,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZBUFFv07_decompressContinue(
        dctx: *mut ZBUFFv07_DCtx,
        dst: *mut std::ffi::c_void,
        dstCapacityPtr: *mut size_t,
        src: *const std::ffi::c_void,
        srcSizePtr: *mut size_t,
    ) -> size_t;
}
pub type size_t = std::ffi::c_ulong;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [u32; 640],
    pub previousDstEnd: *const std::ffi::c_void,
    pub prefixStart: *const std::ffi::c_void,
    pub virtualStart: *const std::ffi::c_void,
    pub dictEnd: *const std::ffi::c_void,
    pub expected: size_t,
    pub fParams: ZSTD_FrameHeader,
    pub processedCSize: U64,
    pub decodedSize: U64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: size_t,
    pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    pub validateChecksum: u32,
    pub litPtr: *const u8,
    pub customMem: ZSTD_customMem,
    pub litSize: size_t,
    pub rleSize: size_t,
    pub staticSize: size_t,
    pub isFrameDecompression: std::ffi::c_int,
    pub bmi2: std::ffi::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: u32,
    pub ddictIsCold: std::ffi::c_int,
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e,
    pub disableHufAsm: std::ffi::c_int,
    pub maxBlockSizeParam: std::ffi::c_int,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub maxWindowSize: size_t,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub lhSize: size_t,
    pub legacyContext: *mut std::ffi::c_void,
    pub previousLegacyVersion: u32,
    pub legacyVersion: u32,
    pub hostageByte: u32,
    pub noForwardProgress: std::ffi::c_int,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut u8,
    pub litBufferEnd: *const u8,
    pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [u8; 65568],
    pub headerBuffer: [u8; 18],
    pub oversizedDuration: size_t,
    pub traceCtx: ZSTD_TraceCtx,
}
pub type ZSTD_TraceCtx = std::ffi::c_ulonglong;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut std::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_bufferMode_e = std::ffi::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;
pub type ZSTD_refMultipleDDicts_e = std::ffi::c_uint;
pub const ZSTD_rmd_refMultipleDDicts: ZSTD_refMultipleDDicts_e = 1;
pub const ZSTD_rmd_refSingleDDict: ZSTD_refMultipleDDicts_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DDictHashSet {
    pub ddictPtrTable: *mut *const ZSTD_DDict,
    pub ddictPtrTableSize: size_t,
    pub ddictPtrCount: size_t,
}
pub type ZSTD_forceIgnoreChecksum_e = std::ffi::c_uint;
pub const ZSTD_d_ignoreChecksum: ZSTD_forceIgnoreChecksum_e = 1;
pub const ZSTD_d_validateChecksum: ZSTD_forceIgnoreChecksum_e = 0;
pub type ZSTD_format_e = std::ffi::c_uint;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
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
pub type blockType_e = std::ffi::c_uint;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;
pub type U64 = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_FrameHeader {
    pub frameContentSize: std::ffi::c_ulonglong,
    pub windowSize: std::ffi::c_ulonglong,
    pub blockSizeMax: std::ffi::c_uint,
    pub frameType: ZSTD_FrameType_e,
    pub headerSize: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
    pub checksumFlag: std::ffi::c_uint,
    pub _reserved1: std::ffi::c_uint,
    pub _reserved2: std::ffi::c_uint,
}
pub type ZSTD_FrameType_e = std::ffi::c_uint;
pub const ZSTD_skippableFrame: ZSTD_FrameType_e = 1;
pub const ZSTD_frame: ZSTD_FrameType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_cpuid_t {
    pub f1c: u32,
    pub f1d: u32,
    pub f7b: u32,
    pub f7c: u32,
}
pub type ZBUFFv07_DCtx = ZBUFFv07_DCtx_s;
pub type ZBUFFv06_DCtx = ZBUFFv06_DCtx_s;
pub type ZBUFFv05_DCtx = ZBUFFv05_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Trace {
    pub version: std::ffi::c_uint,
    pub streaming: std::ffi::c_int,
    pub dictionaryID: std::ffi::c_uint,
    pub dictionaryIsCold: std::ffi::c_int,
    pub dictionarySize: size_t,
    pub uncompressedSize: size_t,
    pub compressedSize: size_t,
    pub params: *const ZSTD_CCtx_params_s,
    pub cctx: *const ZSTD_CCtx_s,
    pub dctx: *const ZSTD_DCtx_s,
}
pub type unalign32 = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: u32,
    pub origSize: u32,
}
pub type XXH_errorcode = std::ffi::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type streaming_operation = std::ffi::c_uint;
pub const is_streaming: streaming_operation = 1;
pub const not_streaming: streaming_operation = 0;
pub type unalign64 = U64;
pub type unalign16 = u16;
pub type U8 = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameSizeInfo {
    pub nbBlocks: size_t,
    pub compressedSize: size_t,
    pub decompressedBound: std::ffi::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_frameParams {
    pub frameContentSize: std::ffi::c_ulonglong,
    pub windowSize: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
    pub checksumFlag: std::ffi::c_uint,
}
pub type ZSTDv06_frameParams = ZSTDv06_frameParams_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv06_frameParams_s {
    pub frameContentSize: std::ffi::c_ulonglong,
    pub windowLog: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv05_parameters {
    pub srcSize: U64,
    pub windowLog: u32,
    pub contentLog: u32,
    pub hashLog: u32,
    pub searchLog: u32,
    pub searchLength: u32,
    pub targetLength: u32,
    pub strategy: ZSTDv05_strategy,
}
pub type ZSTDv05_strategy = std::ffi::c_uint;
pub const ZSTDv05_btopt: ZSTDv05_strategy = 6;
pub const ZSTDv05_opt: ZSTDv05_strategy = 5;
pub const ZSTDv05_btlazy2: ZSTDv05_strategy = 4;
pub const ZSTDv05_lazy2: ZSTDv05_strategy = 3;
pub const ZSTDv05_lazy: ZSTDv05_strategy = 2;
pub const ZSTDv05_greedy: ZSTDv05_strategy = 1;
pub const ZSTDv05_fast: ZSTDv05_strategy = 0;
pub type ZSTDv07_DCtx = ZSTDv07_DCtx_s;
pub type ZSTDv06_DCtx = ZSTDv06_DCtx_s;
pub type ZSTDv05_DCtx = ZSTDv05_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: std::ffi::c_int,
    pub upperBound: std::ffi::c_int,
}
pub type ZSTD_ResetDirective = std::ffi::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
pub type ZSTD_dParameter = std::ffi::c_uint;
pub const ZSTD_d_experimentalParam6: ZSTD_dParameter = 1005;
pub const ZSTD_d_experimentalParam5: ZSTD_dParameter = 1004;
pub const ZSTD_d_experimentalParam4: ZSTD_dParameter = 1003;
pub const ZSTD_d_experimentalParam3: ZSTD_dParameter = 1002;
pub const ZSTD_d_experimentalParam2: ZSTD_dParameter = 1001;
pub const ZSTD_d_experimentalParam1: ZSTD_dParameter = 1000;
pub const ZSTD_d_windowLogMax: ZSTD_dParameter = 100;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const std::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
pub type ZSTD_DStream = ZSTD_DCtx;
pub const ZSTDnit_block: ZSTD_nextInputType_e = 2;
pub type ZSTD_nextInputType_e = std::ffi::c_uint;
pub const ZSTDnit_skippableFrame: ZSTD_nextInputType_e = 5;
pub const ZSTDnit_checksum: ZSTD_nextInputType_e = 4;
pub const ZSTDnit_lastBlock: ZSTD_nextInputType_e = 3;
pub const ZSTDnit_blockHeader: ZSTD_nextInputType_e = 1;
pub const ZSTDnit_frameHeader: ZSTD_nextInputType_e = 0;
pub type ZSTD_dictContentType_e = std::ffi::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_dictLoadMethod_e = std::ffi::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub const ZSTD_MAXWINDOWSIZE_DEFAULT: u32 = ((1 as std::ffi::c_int as u32)
    << ZSTD_WINDOWLOG_LIMIT_DEFAULT)
    .wrapping_add(1 as std::ffi::c_int as u32);
pub const ZSTD_NO_FORWARD_PROGRESS_MAX: std::ffi::c_int = 16 as std::ffi::c_int;
pub const ZSTD_VERSION_MAJOR: std::ffi::c_int = 1 as std::ffi::c_int;
pub const ZSTD_VERSION_MINOR: std::ffi::c_int = 5 as std::ffi::c_int;
pub const ZSTD_VERSION_RELEASE: std::ffi::c_int = 8 as std::ffi::c_int;
pub const ZSTD_VERSION_NUMBER: std::ffi::c_int =
    ZSTD_VERSION_MAJOR * 100 as std::ffi::c_int * 100 as std::ffi::c_int
        + ZSTD_VERSION_MINOR * 100 as std::ffi::c_int
        + ZSTD_VERSION_RELEASE;
pub const ZSTD_MAGICNUMBER: std::ffi::c_uint = 0xfd2fb528 as std::ffi::c_uint;
pub const ZSTD_MAGIC_DICTIONARY: std::ffi::c_uint = 0xec30a437 as std::ffi::c_uint;
pub const ZSTD_MAGIC_SKIPPABLE_START: std::ffi::c_int = 0x184d2a50 as std::ffi::c_int;
pub const ZSTD_MAGIC_SKIPPABLE_MASK: std::ffi::c_uint = 0xfffffff0 as std::ffi::c_uint;
pub const ZSTD_BLOCKSIZELOG_MAX: std::ffi::c_int = 17 as std::ffi::c_int;
pub const ZSTD_BLOCKSIZE_MAX: std::ffi::c_int = (1 as std::ffi::c_int) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: std::ffi::c_ulonglong =
    (0 as std::ffi::c_ulonglong).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong);
pub const ZSTD_CONTENTSIZE_ERROR: std::ffi::c_ulonglong =
    (0 as std::ffi::c_ulonglong).wrapping_sub(2 as std::ffi::c_int as std::ffi::c_ulonglong);
pub const ZSTD_SKIPPABLEHEADERSIZE: std::ffi::c_int = 8 as std::ffi::c_int;
pub const ZSTD_WINDOWLOG_MAX_32: std::ffi::c_int = 30 as std::ffi::c_int;
pub const ZSTD_WINDOWLOG_MAX_64: std::ffi::c_int = 31 as std::ffi::c_int;
pub const ZSTD_BLOCKSIZE_MAX_MIN: std::ffi::c_int = (1 as std::ffi::c_int) << 10 as std::ffi::c_int;
pub const ZSTD_WINDOWLOG_LIMIT_DEFAULT: std::ffi::c_int = 27 as std::ffi::c_int;
static mut ZSTD_defaultCMem: ZSTD_customMem = unsafe {
    {
        ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<libc::intptr_t, ZSTD_allocFunction>(
                NULL as libc::intptr_t,
            ),
            customFree: ::core::mem::transmute::<libc::intptr_t, ZSTD_freeFunction>(
                NULL as libc::intptr_t,
            ),
            opaque: NULL as *mut std::ffi::c_void,
        }
    }
};
pub const ZSTD_d_format: std::ffi::c_int = 1000;
pub const ZSTD_d_stableOutBuffer: std::ffi::c_int = 1001;
pub const ZSTD_d_forceIgnoreChecksum: std::ffi::c_int = 1002;
pub const ZSTD_d_refMultipleDDicts: std::ffi::c_int = 1003;
pub const ZSTD_d_disableHuffmanAssembly: std::ffi::c_int = 1004;
pub const ZSTD_d_maxBlockSize: std::ffi::c_int = 1005;
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
static mut repStartValue: [u32; 3] = [
    1 as std::ffi::c_int as u32,
    4 as std::ffi::c_int as u32,
    8 as std::ffi::c_int as u32,
];
pub const ZSTD_WINDOWLOG_ABSOLUTEMIN: std::ffi::c_int = 10 as std::ffi::c_int;
static mut ZSTD_fcs_fieldSize: [size_t; 4] = [
    0 as std::ffi::c_int as size_t,
    2 as std::ffi::c_int as size_t,
    4 as std::ffi::c_int as size_t,
    8 as std::ffi::c_int as size_t,
];
static mut ZSTD_did_fieldSize: [size_t; 4] = [
    0 as std::ffi::c_int as size_t,
    1 as std::ffi::c_int as size_t,
    2 as std::ffi::c_int as size_t,
    4 as std::ffi::c_int as size_t,
];
pub const ZSTD_FRAMEIDSIZE: std::ffi::c_int = 4 as std::ffi::c_int;
pub const ZSTD_BLOCKHEADERSIZE: std::ffi::c_int = 3 as std::ffi::c_int;
static mut ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub const MaxML: std::ffi::c_int = 52 as std::ffi::c_int;
pub const MaxLL: std::ffi::c_int = 35 as std::ffi::c_int;
pub const MaxOff: std::ffi::c_int = 31 as std::ffi::c_int;
static mut LL_bits: [U8; 36] = [
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    4 as std::ffi::c_int as U8,
    6 as std::ffi::c_int as U8,
    7 as std::ffi::c_int as U8,
    8 as std::ffi::c_int as U8,
    9 as std::ffi::c_int as U8,
    10 as std::ffi::c_int as U8,
    11 as std::ffi::c_int as U8,
    12 as std::ffi::c_int as U8,
    13 as std::ffi::c_int as U8,
    14 as std::ffi::c_int as U8,
    15 as std::ffi::c_int as U8,
    16 as std::ffi::c_int as U8,
];
static mut ML_bits: [U8; 53] = [
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    4 as std::ffi::c_int as U8,
    4 as std::ffi::c_int as U8,
    5 as std::ffi::c_int as U8,
    7 as std::ffi::c_int as U8,
    8 as std::ffi::c_int as U8,
    9 as std::ffi::c_int as U8,
    10 as std::ffi::c_int as U8,
    11 as std::ffi::c_int as U8,
    12 as std::ffi::c_int as U8,
    13 as std::ffi::c_int as U8,
    14 as std::ffi::c_int as U8,
    15 as std::ffi::c_int as U8,
    16 as std::ffi::c_int as U8,
];
pub const WILDCOPY_OVERLENGTH: std::ffi::c_int = 32 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn ZSTD_limitCopy(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let length = if dstCapacity < srcSize {
        dstCapacity
    } else {
        srcSize
    };
    if length > 0 as std::ffi::c_int as size_t {
        libc::memcpy(dst, src, length as libc::size_t);
    }
    length
}
pub const ZSTD_WORKSPACETOOLARGE_FACTOR: std::ffi::c_int = 3 as std::ffi::c_int;
pub const ZSTD_WORKSPACETOOLARGE_MAXDURATION: std::ffi::c_int = 128 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn ZSTD_cpuSupportsBmi2() -> std::ffi::c_int {
    let mut cpuid = ZSTD_cpuid();
    (ZSTD_cpuid_bmi1(cpuid) != 0 && ZSTD_cpuid_bmi2(cpuid) != 0) as std::ffi::c_int
}
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
        libc::memset(ptr, 0 as std::ffi::c_int, size as libc::size_t);
        return ptr;
    }
    calloc(1 as std::ffi::c_int as std::ffi::c_ulong, size)
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
unsafe extern "C" fn ZSTD_cpuid() -> ZSTD_cpuid_t {
    let mut f1c = 0 as std::ffi::c_int as u32;
    let mut f1d = 0 as std::ffi::c_int as u32;
    let mut f7b = 0 as std::ffi::c_int as u32;
    let mut f7c = 0 as std::ffi::c_int as u32;
    let mut n: u32 = 0;
    asm!(
        "cpuid", inlateout("ax") 0 as std::ffi::c_int => n, out("ecx") _, out("edx") _,
        options(preserves_flags, pure, readonly, att_syntax)
    );
    if n >= 1 as std::ffi::c_int as u32 {
        let mut f1a: u32 = 0;
        asm!(
            "cpuid", inlateout("ax") 1 as std::ffi::c_int => f1a, lateout("cx") f1c,
            lateout("dx") f1d, options(preserves_flags, pure, readonly, att_syntax)
        );
    }
    if n >= 7 as std::ffi::c_int as u32 {
        let mut f7a: u32 = 0;
        asm!(
            "cpuid\nmov {restmp0:x}, %bx", restmp0 = lateout(reg) f7b, inlateout("ax") 7
            as std::ffi::c_int => f7a, inlateout("cx") 0 as std::ffi::c_int => f7c,
            out("edx") _, options(preserves_flags, pure, readonly, att_syntax)
        );
    }
    let mut cpuid = ZSTD_cpuid_t {
        f1c: 0,
        f1d: 0,
        f7b: 0,
        f7c: 0,
    };
    cpuid.f1c = f1c;
    cpuid.f1d = f1d;
    cpuid.f7b = f7b;
    cpuid.f7c = f7c;
    cpuid
}
#[inline]
unsafe extern "C" fn ZSTD_cpuid_bmi1(cpuid: ZSTD_cpuid_t) -> std::ffi::c_int {
    (cpuid.f7b & (1 as std::ffi::c_uint) << 3 as std::ffi::c_int
        != 0 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int
}
#[inline]
unsafe extern "C" fn ZSTD_cpuid_bmi2(cpuid: ZSTD_cpuid_t) -> std::ffi::c_int {
    (cpuid.f7b & (1 as std::ffi::c_uint) << 8 as std::ffi::c_int
        != 0 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const ZSTDv05_MAGICNUMBER: std::ffi::c_uint = 4247762213;
pub const ZSTDv06_MAGICNUMBER: std::ffi::c_uint = 4247762214;
pub const ZSTDv07_MAGICNUMBER: std::ffi::c_uint = 4247762215;
#[inline]
unsafe extern "C" fn ZSTD_isLegacy(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_uint {
    let mut magicNumberLE: u32 = 0;
    if srcSize < 4 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    magicNumberLE = MEM_readLE32(src);
    match magicNumberLE {
        ZSTDv05_MAGICNUMBER => 5 as std::ffi::c_int as std::ffi::c_uint,
        ZSTDv06_MAGICNUMBER => 6 as std::ffi::c_int as std::ffi::c_uint,
        ZSTDv07_MAGICNUMBER => 7 as std::ffi::c_int as std::ffi::c_uint,
        _ => 0 as std::ffi::c_int as std::ffi::c_uint,
    }
}
#[inline]
unsafe extern "C" fn ZSTD_getDecompressedSize_legacy(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_ulonglong {
    let version = ZSTD_isLegacy(src, srcSize);
    if version < 5 as std::ffi::c_int as u32 {
        return 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    }
    if version == 5 as std::ffi::c_int as u32 {
        let mut fParams = ZSTDv05_parameters {
            srcSize: 0,
            windowLog: 0,
            contentLog: 0,
            hashLog: 0,
            searchLog: 0,
            searchLength: 0,
            targetLength: 0,
            strategy: ZSTDv05_fast,
        };
        let frResult = ZSTDv05_getFrameParams(&mut fParams, src, srcSize);
        if frResult != 0 as std::ffi::c_int as size_t {
            return 0 as std::ffi::c_int as std::ffi::c_ulonglong;
        }
        return fParams.srcSize as std::ffi::c_ulonglong;
    }
    if version == 6 as std::ffi::c_int as u32 {
        let mut fParams_0 = ZSTDv06_frameParams_s {
            frameContentSize: 0,
            windowLog: 0,
        };
        let frResult_0 = ZSTDv06_getFrameParams(&mut fParams_0, src, srcSize);
        if frResult_0 != 0 as std::ffi::c_int as size_t {
            return 0 as std::ffi::c_int as std::ffi::c_ulonglong;
        }
        return fParams_0.frameContentSize;
    }
    if version == 7 as std::ffi::c_int as u32 {
        let mut fParams_1 = ZSTDv07_frameParams {
            frameContentSize: 0,
            windowSize: 0,
            dictID: 0,
            checksumFlag: 0,
        };
        let frResult_1 = ZSTDv07_getFrameParams(&mut fParams_1, src, srcSize);
        if frResult_1 != 0 as std::ffi::c_int as size_t {
            return 0 as std::ffi::c_int as std::ffi::c_ulonglong;
        }
        return fParams_1.frameContentSize;
    }
    0 as std::ffi::c_int as std::ffi::c_ulonglong
}
#[inline]
unsafe extern "C" fn ZSTD_decompressLegacy(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut compressedSize: size_t,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let version = ZSTD_isLegacy(src, compressedSize);
    let mut x: std::ffi::c_char = 0;
    if dst.is_null() {
        dst = &mut x as *mut std::ffi::c_char as *mut std::ffi::c_void;
    }
    if src.is_null() {
        src = &mut x as *mut std::ffi::c_char as *const std::ffi::c_void;
    }
    if dict.is_null() {
        dict = &mut x as *mut std::ffi::c_char as *const std::ffi::c_void;
    }
    match version {
        5 => {
            let mut result: size_t = 0;
            let zd = ZSTDv05_createDCtx();
            if zd.is_null() {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            result = ZSTDv05_decompress_usingDict(
                zd,
                dst,
                dstCapacity,
                src,
                compressedSize,
                dict,
                dictSize,
            );
            ZSTDv05_freeDCtx(zd);
            result
        }
        6 => {
            let mut result_0: size_t = 0;
            let zd_0 = ZSTDv06_createDCtx();
            if zd_0.is_null() {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            result_0 = ZSTDv06_decompress_usingDict(
                zd_0,
                dst,
                dstCapacity,
                src,
                compressedSize,
                dict,
                dictSize,
            );
            ZSTDv06_freeDCtx(zd_0);
            result_0
        }
        7 => {
            let mut result_1: size_t = 0;
            let zd_1 = ZSTDv07_createDCtx();
            if zd_1.is_null() {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            result_1 = ZSTDv07_decompress_usingDict(
                zd_1,
                dst,
                dstCapacity,
                src,
                compressedSize,
                dict,
                dictSize,
            );
            ZSTDv07_freeDCtx(zd_1);
            result_1
        }
        _ => -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t,
    }
}
#[inline]
unsafe extern "C" fn ZSTD_findFrameSizeInfoLegacy(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> ZSTD_frameSizeInfo {
    let mut frameSizeInfo = ZSTD_frameSizeInfo {
        nbBlocks: 0,
        compressedSize: 0,
        decompressedBound: 0,
    };
    let version = ZSTD_isLegacy(src, srcSize);
    match version {
        5 => {
            ZSTDv05_findFrameSizeInfoLegacy(
                src,
                srcSize,
                &mut frameSizeInfo.compressedSize,
                &mut frameSizeInfo.decompressedBound,
            );
        }
        6 => {
            ZSTDv06_findFrameSizeInfoLegacy(
                src,
                srcSize,
                &mut frameSizeInfo.compressedSize,
                &mut frameSizeInfo.decompressedBound,
            );
        }
        7 => {
            ZSTDv07_findFrameSizeInfoLegacy(
                src,
                srcSize,
                &mut frameSizeInfo.compressedSize,
                &mut frameSizeInfo.decompressedBound,
            );
        }
        _ => {
            frameSizeInfo.compressedSize =
                -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t;
            frameSizeInfo.decompressedBound = ZSTD_CONTENTSIZE_ERROR;
        }
    }
    if ERR_isError(frameSizeInfo.compressedSize) == 0 && frameSizeInfo.compressedSize > srcSize {
        frameSizeInfo.compressedSize = -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        frameSizeInfo.decompressedBound = ZSTD_CONTENTSIZE_ERROR;
    }
    if frameSizeInfo.decompressedBound != ZSTD_CONTENTSIZE_ERROR {
        frameSizeInfo.nbBlocks = (frameSizeInfo.decompressedBound)
            .wrapping_div(ZSTD_BLOCKSIZE_MAX as std::ffi::c_ulonglong)
            as size_t;
    }
    frameSizeInfo
}
#[inline]
unsafe extern "C" fn ZSTD_findFrameCompressedSizeLegacy(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut frameSizeInfo = ZSTD_findFrameSizeInfoLegacy(src, srcSize);
    frameSizeInfo.compressedSize
}
#[inline]
unsafe extern "C" fn ZSTD_freeLegacyStreamContext(
    mut legacyContext: *mut std::ffi::c_void,
    mut version: u32,
) -> size_t {
    match version {
        5 => ZBUFFv05_freeDCtx(legacyContext as *mut ZBUFFv05_DCtx),
        6 => ZBUFFv06_freeDCtx(legacyContext as *mut ZBUFFv06_DCtx),
        7 => ZBUFFv07_freeDCtx(legacyContext as *mut ZBUFFv07_DCtx),
        1 | 2 | 3 | _ => -(ZSTD_error_version_unsupported as std::ffi::c_int) as size_t,
    }
}
#[inline]
unsafe extern "C" fn ZSTD_initLegacyStream(
    mut legacyContext: *mut *mut std::ffi::c_void,
    mut prevVersion: u32,
    mut newVersion: u32,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut x: std::ffi::c_char = 0;
    if dict.is_null() {
        dict = &mut x as *mut std::ffi::c_char as *const std::ffi::c_void;
    }
    if prevVersion != newVersion {
        ZSTD_freeLegacyStreamContext(*legacyContext, prevVersion);
    }
    match newVersion {
        5 => {
            let mut dctx = if prevVersion != newVersion {
                ZBUFFv05_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv05_DCtx
            };
            if dctx.is_null() {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            ZBUFFv05_decompressInitDictionary(dctx, dict, dictSize);
            *legacyContext = dctx as *mut std::ffi::c_void;
            0 as std::ffi::c_int as size_t
        }
        6 => {
            let mut dctx_0 = if prevVersion != newVersion {
                ZBUFFv06_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv06_DCtx
            };
            if dctx_0.is_null() {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            ZBUFFv06_decompressInitDictionary(dctx_0, dict, dictSize);
            *legacyContext = dctx_0 as *mut std::ffi::c_void;
            0 as std::ffi::c_int as size_t
        }
        7 => {
            let mut dctx_1 = if prevVersion != newVersion {
                ZBUFFv07_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv07_DCtx
            };
            if dctx_1.is_null() {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            ZBUFFv07_decompressInitDictionary(dctx_1, dict, dictSize);
            *legacyContext = dctx_1 as *mut std::ffi::c_void;
            0 as std::ffi::c_int as size_t
        }
        1 | 2 | 3 | _ => 0 as std::ffi::c_int as size_t,
    }
}
#[inline]
unsafe extern "C" fn ZSTD_decompressLegacyStream(
    mut legacyContext: *mut std::ffi::c_void,
    mut version: u32,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
) -> size_t {
    static mut x: std::ffi::c_char = 0;
    if ((*output).dst).is_null() {
        (*output).dst = &mut x as *mut std::ffi::c_char as *mut std::ffi::c_void;
    }
    if ((*input).src).is_null() {
        (*input).src = &mut x as *mut std::ffi::c_char as *const std::ffi::c_void;
    }
    match version {
        5 => {
            let mut dctx = legacyContext as *mut ZBUFFv05_DCtx;
            let mut src = ((*input).src as *const std::ffi::c_char).offset((*input).pos as isize)
                as *const std::ffi::c_void;
            let mut readSize = ((*input).size).wrapping_sub((*input).pos);
            let mut dst = ((*output).dst as *mut std::ffi::c_char).offset((*output).pos as isize)
                as *mut std::ffi::c_void;
            let mut decodedSize = ((*output).size).wrapping_sub((*output).pos);
            let hintSize =
                ZBUFFv05_decompressContinue(dctx, dst, &mut decodedSize, src, &mut readSize);
            (*output).pos = ((*output).pos).wrapping_add(decodedSize);
            (*input).pos = ((*input).pos).wrapping_add(readSize);
            hintSize
        }
        6 => {
            let mut dctx_0 = legacyContext as *mut ZBUFFv06_DCtx;
            let mut src_0 = ((*input).src as *const std::ffi::c_char).offset((*input).pos as isize)
                as *const std::ffi::c_void;
            let mut readSize_0 = ((*input).size).wrapping_sub((*input).pos);
            let mut dst_0 = ((*output).dst as *mut std::ffi::c_char).offset((*output).pos as isize)
                as *mut std::ffi::c_void;
            let mut decodedSize_0 = ((*output).size).wrapping_sub((*output).pos);
            let hintSize_0 = ZBUFFv06_decompressContinue(
                dctx_0,
                dst_0,
                &mut decodedSize_0,
                src_0,
                &mut readSize_0,
            );
            (*output).pos = ((*output).pos).wrapping_add(decodedSize_0);
            (*input).pos = ((*input).pos).wrapping_add(readSize_0);
            hintSize_0
        }
        7 => {
            let mut dctx_1 = legacyContext as *mut ZBUFFv07_DCtx;
            let mut src_1 = ((*input).src as *const std::ffi::c_char).offset((*input).pos as isize)
                as *const std::ffi::c_void;
            let mut readSize_1 = ((*input).size).wrapping_sub((*input).pos);
            let mut dst_1 = ((*output).dst as *mut std::ffi::c_char).offset((*output).pos as isize)
                as *mut std::ffi::c_void;
            let mut decodedSize_1 = ((*output).size).wrapping_sub((*output).pos);
            let hintSize_1 = ZBUFFv07_decompressContinue(
                dctx_1,
                dst_1,
                &mut decodedSize_1,
                src_1,
                &mut readSize_1,
            );
            (*output).pos = ((*output).pos).wrapping_add(decodedSize_1);
            (*input).pos = ((*input).pos).wrapping_add(readSize_1);
            hintSize_1
        }
        1 | 2 | 3 | _ => -(ZSTD_error_version_unsupported as std::ffi::c_int) as size_t,
    }
}
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT: std::ffi::c_int = 4 as std::ffi::c_int;
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_SIZE_MULT: std::ffi::c_int = 3 as std::ffi::c_int;
pub const DDICT_HASHSET_TABLE_BASE_SIZE: std::ffi::c_int = 64 as std::ffi::c_int;
pub const DDICT_HASHSET_RESIZE_FACTOR: std::ffi::c_int = 2 as std::ffi::c_int;
unsafe extern "C" fn ZSTD_DDictHashSet_getIndex(
    mut hashSet: *const ZSTD_DDictHashSet,
    mut dictID: u32,
) -> size_t {
    let hash = ZSTD_XXH64(
        &mut dictID as *mut u32 as *const std::ffi::c_void,
        ::core::mem::size_of::<u32>() as std::ffi::c_ulong,
        0 as std::ffi::c_int as XXH64_hash_t,
    );
    hash & ((*hashSet).ddictPtrTableSize).wrapping_sub(1 as std::ffi::c_int as size_t)
}
unsafe extern "C" fn ZSTD_DDictHashSet_emplaceDDict(
    mut hashSet: *mut ZSTD_DDictHashSet,
    mut ddict: *const ZSTD_DDict,
) -> size_t {
    let dictID = ZSTD_getDictID_fromDDict(ddict);
    let mut idx = ZSTD_DDictHashSet_getIndex(hashSet, dictID);
    let idxRangeMask = ((*hashSet).ddictPtrTableSize).wrapping_sub(1 as std::ffi::c_int as size_t);
    if (*hashSet).ddictPtrCount == (*hashSet).ddictPtrTableSize {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    while !(*((*hashSet).ddictPtrTable).offset(idx as isize)).is_null() {
        if ZSTD_getDictID_fromDDict(*((*hashSet).ddictPtrTable).offset(idx as isize)) == dictID {
            let fresh0 = &mut (*((*hashSet).ddictPtrTable).offset(idx as isize));
            *fresh0 = ddict;
            return 0 as std::ffi::c_int as size_t;
        }
        idx &= idxRangeMask;
        idx = idx.wrapping_add(1);
        idx;
    }
    let fresh1 = &mut (*((*hashSet).ddictPtrTable).offset(idx as isize));
    *fresh1 = ddict;
    (*hashSet).ddictPtrCount = ((*hashSet).ddictPtrCount).wrapping_add(1);
    (*hashSet).ddictPtrCount;
    0 as std::ffi::c_int as size_t
}
unsafe extern "C" fn ZSTD_DDictHashSet_expand(
    mut hashSet: *mut ZSTD_DDictHashSet,
    mut customMem: ZSTD_customMem,
) -> size_t {
    let mut newTableSize = (*hashSet).ddictPtrTableSize * DDICT_HASHSET_RESIZE_FACTOR as size_t;
    let mut newTable = ZSTD_customCalloc(
        (::core::mem::size_of::<*mut ZSTD_DDict>() as std::ffi::c_ulong).wrapping_mul(newTableSize),
        customMem,
    ) as *mut *const ZSTD_DDict;
    let mut oldTable = (*hashSet).ddictPtrTable;
    let mut oldTableSize = (*hashSet).ddictPtrTableSize;
    let mut i: size_t = 0;
    if newTable.is_null() {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    (*hashSet).ddictPtrTable = newTable;
    (*hashSet).ddictPtrTableSize = newTableSize;
    (*hashSet).ddictPtrCount = 0 as std::ffi::c_int as size_t;
    i = 0 as std::ffi::c_int as size_t;
    while i < oldTableSize {
        if !(*oldTable.offset(i as isize)).is_null() {
            let err_code = ZSTD_DDictHashSet_emplaceDDict(hashSet, *oldTable.offset(i as isize));
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    ZSTD_customFree(oldTable as *mut std::ffi::c_void, customMem);
    0 as std::ffi::c_int as size_t
}
unsafe extern "C" fn ZSTD_DDictHashSet_getDDict(
    mut hashSet: *mut ZSTD_DDictHashSet,
    mut dictID: u32,
) -> *const ZSTD_DDict {
    let mut idx = ZSTD_DDictHashSet_getIndex(hashSet, dictID);
    let idxRangeMask = ((*hashSet).ddictPtrTableSize).wrapping_sub(1 as std::ffi::c_int as size_t);
    loop {
        let mut currDictID =
            ZSTD_getDictID_fromDDict(*((*hashSet).ddictPtrTable).offset(idx as isize)) as size_t;
        if currDictID == dictID as size_t || currDictID == 0 as std::ffi::c_int as size_t {
            break;
        }
        idx &= idxRangeMask;
        idx = idx.wrapping_add(1);
        idx;
    }
    *((*hashSet).ddictPtrTable).offset(idx as isize)
}
unsafe extern "C" fn ZSTD_createDDictHashSet(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_DDictHashSet {
    let mut ret = ZSTD_customMalloc(
        ::core::mem::size_of::<ZSTD_DDictHashSet>() as std::ffi::c_ulong,
        customMem,
    ) as *mut ZSTD_DDictHashSet;
    if ret.is_null() {
        return NULL as *mut ZSTD_DDictHashSet;
    }
    (*ret).ddictPtrTable = ZSTD_customCalloc(
        (DDICT_HASHSET_TABLE_BASE_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut ZSTD_DDict>() as std::ffi::c_ulong),
        customMem,
    ) as *mut *const ZSTD_DDict;
    if ((*ret).ddictPtrTable).is_null() {
        ZSTD_customFree(ret as *mut std::ffi::c_void, customMem);
        return NULL as *mut ZSTD_DDictHashSet;
    }
    (*ret).ddictPtrTableSize = DDICT_HASHSET_TABLE_BASE_SIZE as size_t;
    (*ret).ddictPtrCount = 0 as std::ffi::c_int as size_t;
    ret
}
unsafe extern "C" fn ZSTD_freeDDictHashSet(
    mut hashSet: *mut ZSTD_DDictHashSet,
    mut customMem: ZSTD_customMem,
) {
    if !hashSet.is_null() && !((*hashSet).ddictPtrTable).is_null() {
        ZSTD_customFree((*hashSet).ddictPtrTable as *mut std::ffi::c_void, customMem);
    }
    if !hashSet.is_null() {
        ZSTD_customFree(hashSet as *mut std::ffi::c_void, customMem);
    }
}
unsafe extern "C" fn ZSTD_DDictHashSet_addDDict(
    mut hashSet: *mut ZSTD_DDictHashSet,
    mut ddict: *const ZSTD_DDict,
    mut customMem: ZSTD_customMem,
) -> size_t {
    if (*hashSet).ddictPtrCount * DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT as size_t
        / (*hashSet).ddictPtrTableSize
        * DDICT_HASHSET_MAX_LOAD_FACTOR_SIZE_MULT as size_t
        != 0 as std::ffi::c_int as size_t
    {
        let err_code = ZSTD_DDictHashSet_expand(hashSet, customMem);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
    }
    let err_code_0 = ZSTD_DDictHashSet_emplaceDDict(hashSet, ddict);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DCtx(mut dctx: *const ZSTD_DCtx) -> size_t {
    if dctx.is_null() {
        return 0 as std::ffi::c_int as size_t;
    }
    (::core::mem::size_of::<ZSTD_DCtx>() as std::ffi::c_ulong)
        .wrapping_add(ZSTD_sizeof_DDict((*dctx).ddictLocal))
        .wrapping_add((*dctx).inBuffSize)
        .wrapping_add((*dctx).outBuffSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDCtxSize() -> size_t {
    ::core::mem::size_of::<ZSTD_DCtx>() as std::ffi::c_ulong
}
unsafe extern "C" fn ZSTD_startingInputLength(mut format: ZSTD_format_e) -> size_t {
    (if format as std::ffi::c_uint == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint {
        5 as std::ffi::c_int
    } else {
        1 as std::ffi::c_int
    }) as size_t
}
unsafe extern "C" fn ZSTD_DCtx_resetParameters(mut dctx: *mut ZSTD_DCtx) {
    (*dctx).format = ZSTD_f_zstd1;
    (*dctx).maxWindowSize = ZSTD_MAXWINDOWSIZE_DEFAULT as size_t;
    (*dctx).outBufferMode = ZSTD_bm_buffered;
    (*dctx).forceIgnoreChecksum = ZSTD_d_validateChecksum;
    (*dctx).refMultipleDDicts = ZSTD_rmd_refSingleDDict;
    (*dctx).disableHufAsm = 0 as std::ffi::c_int;
    (*dctx).maxBlockSizeParam = 0 as std::ffi::c_int;
}
unsafe extern "C" fn ZSTD_initDCtx_internal(mut dctx: *mut ZSTD_DCtx) {
    (*dctx).staticSize = 0 as std::ffi::c_int as size_t;
    (*dctx).ddict = NULL as *const ZSTD_DDict;
    (*dctx).ddictLocal = NULL as *mut ZSTD_DDict;
    (*dctx).dictEnd = NULL as *const std::ffi::c_void;
    (*dctx).ddictIsCold = 0 as std::ffi::c_int;
    (*dctx).dictUses = ZSTD_dont_use;
    (*dctx).inBuff = NULL as *mut std::ffi::c_char;
    (*dctx).inBuffSize = 0 as std::ffi::c_int as size_t;
    (*dctx).outBuffSize = 0 as std::ffi::c_int as size_t;
    (*dctx).streamStage = zdss_init;
    (*dctx).legacyContext = NULL as *mut std::ffi::c_void;
    (*dctx).previousLegacyVersion = 0 as std::ffi::c_int as u32;
    (*dctx).noForwardProgress = 0 as std::ffi::c_int;
    (*dctx).oversizedDuration = 0 as std::ffi::c_int as size_t;
    (*dctx).isFrameDecompression = 1 as std::ffi::c_int;
    (*dctx).bmi2 = ZSTD_cpuSupportsBmi2();
    (*dctx).ddictSet = NULL as *mut ZSTD_DDictHashSet;
    ZSTD_DCtx_resetParameters(dctx);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDCtx(
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: size_t,
) -> *mut ZSTD_DCtx {
    let dctx = workspace as *mut ZSTD_DCtx;
    if workspace as size_t & 7 as std::ffi::c_int as size_t != 0 {
        return NULL as *mut ZSTD_DCtx;
    }
    if workspaceSize < ::core::mem::size_of::<ZSTD_DCtx>() as std::ffi::c_ulong {
        return NULL as *mut ZSTD_DCtx;
    }
    ZSTD_initDCtx_internal(dctx);
    (*dctx).staticSize = workspaceSize;
    (*dctx).inBuff = dctx.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_char;
    dctx
}
unsafe extern "C" fn ZSTD_createDCtx_internal(mut customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    if (customMem.customAlloc).is_none() as std::ffi::c_int
        ^ (customMem.customFree).is_none() as std::ffi::c_int
        != 0
    {
        return NULL as *mut ZSTD_DCtx;
    }
    let dctx = ZSTD_customMalloc(
        ::core::mem::size_of::<ZSTD_DCtx>() as std::ffi::c_ulong,
        customMem,
    ) as *mut ZSTD_DCtx;
    if dctx.is_null() {
        return NULL as *mut ZSTD_DCtx;
    }
    (*dctx).customMem = customMem;
    ZSTD_initDCtx_internal(dctx);
    dctx
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx_advanced(mut customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    ZSTD_createDCtx_internal(customMem)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx {
    ZSTD_createDCtx_internal(ZSTD_defaultCMem)
}
unsafe extern "C" fn ZSTD_clearDict(mut dctx: *mut ZSTD_DCtx) {
    ZSTD_freeDDict((*dctx).ddictLocal);
    (*dctx).ddictLocal = NULL as *mut ZSTD_DDict;
    (*dctx).ddict = NULL as *const ZSTD_DDict;
    (*dctx).dictUses = ZSTD_dont_use;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDCtx(mut dctx: *mut ZSTD_DCtx) -> size_t {
    if dctx.is_null() {
        return 0 as std::ffi::c_int as size_t;
    }
    if (*dctx).staticSize != 0 {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    let cMem = (*dctx).customMem;
    ZSTD_clearDict(dctx);
    ZSTD_customFree((*dctx).inBuff as *mut std::ffi::c_void, cMem);
    (*dctx).inBuff = NULL as *mut std::ffi::c_char;
    if !((*dctx).legacyContext).is_null() {
        ZSTD_freeLegacyStreamContext((*dctx).legacyContext, (*dctx).previousLegacyVersion);
    }
    if !((*dctx).ddictSet).is_null() {
        ZSTD_freeDDictHashSet((*dctx).ddictSet, cMem);
        (*dctx).ddictSet = NULL as *mut ZSTD_DDictHashSet;
    }
    ZSTD_customFree(dctx as *mut std::ffi::c_void, cMem);
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyDCtx(mut dstDCtx: *mut ZSTD_DCtx, mut srcDCtx: *const ZSTD_DCtx) {
    let toCopy = (&mut (*dstDCtx).inBuff as *mut *mut std::ffi::c_char as *mut std::ffi::c_char)
        .offset_from(dstDCtx as *mut std::ffi::c_char) as std::ffi::c_long
        as size_t;
    libc::memcpy(
        dstDCtx as *mut std::ffi::c_void,
        srcDCtx as *const std::ffi::c_void,
        toCopy as libc::size_t,
    );
}
unsafe extern "C" fn ZSTD_DCtx_selectFrameDDict(mut dctx: *mut ZSTD_DCtx) {
    if !((*dctx).ddict).is_null() {
        let mut frameDDict = ZSTD_DDictHashSet_getDDict((*dctx).ddictSet, (*dctx).fParams.dictID);
        if !frameDDict.is_null() {
            ZSTD_clearDict(dctx);
            (*dctx).dictID = (*dctx).fParams.dictID;
            (*dctx).ddict = frameDDict;
            (*dctx).dictUses = ZSTD_use_indefinitely;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isFrame(
    mut buffer: *const std::ffi::c_void,
    mut size: size_t,
) -> std::ffi::c_uint {
    if size < ZSTD_FRAMEIDSIZE as size_t {
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    let magic = MEM_readLE32(buffer);
    if magic == ZSTD_MAGICNUMBER {
        return 1 as std::ffi::c_int as std::ffi::c_uint;
    }
    if magic & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint {
        return 1 as std::ffi::c_int as std::ffi::c_uint;
    }
    if ZSTD_isLegacy(buffer, size) != 0 {
        return 1 as std::ffi::c_int as std::ffi::c_uint;
    }
    0 as std::ffi::c_int as std::ffi::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isSkippableFrame(
    mut buffer: *const std::ffi::c_void,
    mut size: size_t,
) -> std::ffi::c_uint {
    if size < ZSTD_FRAMEIDSIZE as size_t {
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    let magic = MEM_readLE32(buffer);
    if magic & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint {
        return 1 as std::ffi::c_int as std::ffi::c_uint;
    }
    0 as std::ffi::c_int as std::ffi::c_uint
}
unsafe extern "C" fn ZSTD_frameHeaderSize_internal(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut format: ZSTD_format_e,
) -> size_t {
    let minInputSize = ZSTD_startingInputLength(format);
    if srcSize < minInputSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let fhd = *(src as *const u8)
        .offset(minInputSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
    let dictID = (fhd as std::ffi::c_int & 3 as std::ffi::c_int) as u32;
    let singleSegment =
        (fhd as std::ffi::c_int >> 5 as std::ffi::c_int & 1 as std::ffi::c_int) as u32;
    let fcsId = (fhd as std::ffi::c_int >> 6 as std::ffi::c_int) as u32;
    minInputSize
        .wrapping_add((singleSegment == 0) as std::ffi::c_int as size_t)
        .wrapping_add(*ZSTD_did_fieldSize.as_ptr().offset(dictID as isize))
        .wrapping_add(*ZSTD_fcs_fieldSize.as_ptr().offset(fcsId as isize))
        .wrapping_add((singleSegment != 0 && fcsId == 0) as std::ffi::c_int as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_frameHeaderSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_frameHeaderSize_internal(src, srcSize, ZSTD_f_zstd1)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader_advanced(
    mut zfhPtr: *mut ZSTD_FrameHeader,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut format: ZSTD_format_e,
) -> size_t {
    let mut ip = src as *const u8;
    let minInputSize = ZSTD_startingInputLength(format);
    if srcSize > 0 as std::ffi::c_int as size_t && src.is_null() {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if srcSize < minInputSize {
        if srcSize > 0 as std::ffi::c_int as size_t
            && format as std::ffi::c_uint
                != ZSTD_f_zstd1_magicless as std::ffi::c_int as std::ffi::c_uint
        {
            let toCopy = if (4 as std::ffi::c_int as size_t) < srcSize {
                4 as std::ffi::c_int as size_t
            } else {
                srcSize
            };
            let mut hbuf: [std::ffi::c_uchar; 4] = [0; 4];
            MEM_writeLE32(hbuf.as_mut_ptr() as *mut std::ffi::c_void, ZSTD_MAGICNUMBER);
            libc::memcpy(
                hbuf.as_mut_ptr() as *mut std::ffi::c_void,
                src,
                toCopy as libc::size_t,
            );
            if MEM_readLE32(hbuf.as_mut_ptr() as *const std::ffi::c_void) != ZSTD_MAGICNUMBER {
                MEM_writeLE32(
                    hbuf.as_mut_ptr() as *mut std::ffi::c_void,
                    ZSTD_MAGIC_SKIPPABLE_START as u32,
                );
                libc::memcpy(
                    hbuf.as_mut_ptr() as *mut std::ffi::c_void,
                    src,
                    toCopy as libc::size_t,
                );
                if MEM_readLE32(hbuf.as_mut_ptr() as *const std::ffi::c_void)
                    & ZSTD_MAGIC_SKIPPABLE_MASK
                    != ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
                {
                    return -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t;
                }
            }
        }
        return minInputSize;
    }
    libc::memset(
        zfhPtr as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<ZSTD_FrameHeader>() as std::ffi::c_ulong as libc::size_t,
    );
    if format as std::ffi::c_uint != ZSTD_f_zstd1_magicless as std::ffi::c_int as std::ffi::c_uint
        && MEM_readLE32(src) != ZSTD_MAGICNUMBER
    {
        if MEM_readLE32(src) & ZSTD_MAGIC_SKIPPABLE_MASK
            == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
        {
            if srcSize < ZSTD_SKIPPABLEHEADERSIZE as size_t {
                return ZSTD_SKIPPABLEHEADERSIZE as size_t;
            }
            libc::memset(
                zfhPtr as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                ::core::mem::size_of::<ZSTD_FrameHeader>() as std::ffi::c_ulong as libc::size_t,
            );
            (*zfhPtr).frameType = ZSTD_skippableFrame;
            (*zfhPtr).dictID = (MEM_readLE32(src)).wrapping_sub(ZSTD_MAGIC_SKIPPABLE_START as u32);
            (*zfhPtr).headerSize = ZSTD_SKIPPABLEHEADERSIZE as std::ffi::c_uint;
            (*zfhPtr).frameContentSize = MEM_readLE32(
                (src as *const std::ffi::c_char).offset(ZSTD_FRAMEIDSIZE as isize)
                    as *const std::ffi::c_void,
            ) as std::ffi::c_ulonglong;
            return 0 as std::ffi::c_int as size_t;
        }
        return -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t;
    }
    let fhsize = ZSTD_frameHeaderSize_internal(src, srcSize, format);
    if srcSize < fhsize {
        return fhsize;
    }
    (*zfhPtr).headerSize = fhsize as u32;
    let fhdByte = *ip.offset(minInputSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
    let mut pos = minInputSize;
    let dictIDSizeCode = (fhdByte as std::ffi::c_int & 3 as std::ffi::c_int) as u32;
    let checksumFlag =
        (fhdByte as std::ffi::c_int >> 2 as std::ffi::c_int & 1 as std::ffi::c_int) as u32;
    let singleSegment =
        (fhdByte as std::ffi::c_int >> 5 as std::ffi::c_int & 1 as std::ffi::c_int) as u32;
    let fcsID = (fhdByte as std::ffi::c_int >> 6 as std::ffi::c_int) as u32;
    let mut windowSize = 0 as std::ffi::c_int as U64;
    let mut dictID = 0 as std::ffi::c_int as u32;
    let mut frameContentSize = ZSTD_CONTENTSIZE_UNKNOWN as U64;
    if fhdByte as std::ffi::c_int & 0x8 as std::ffi::c_int != 0 as std::ffi::c_int {
        return -(ZSTD_error_frameParameter_unsupported as std::ffi::c_int) as size_t;
    }
    if singleSegment == 0 {
        let fresh2 = pos;
        pos = pos.wrapping_add(1);
        let wlByte = *ip.offset(fresh2 as isize);
        let windowLog = ((wlByte as std::ffi::c_int >> 3 as std::ffi::c_int)
            + ZSTD_WINDOWLOG_ABSOLUTEMIN) as u32;
        if windowLog
            > (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 4 as std::ffi::c_int as std::ffi::c_ulong
            {
                30 as std::ffi::c_int
            } else {
                31 as std::ffi::c_int
            }) as u32
        {
            return -(ZSTD_error_frameParameter_windowTooLarge as std::ffi::c_int) as size_t;
        }
        windowSize = ((1 as std::ffi::c_ulonglong) << windowLog) as U64;
        windowSize = windowSize.wrapping_add(
            (windowSize >> 3 as std::ffi::c_int)
                * (wlByte as std::ffi::c_int & 7 as std::ffi::c_int) as U64,
        );
    }
    match dictIDSizeCode {
        1 => {
            dictID = *ip.offset(pos as isize) as u32;
            pos = pos.wrapping_add(1);
            pos;
        }
        2 => {
            dictID = MEM_readLE16(ip.offset(pos as isize) as *const std::ffi::c_void) as u32;
            pos = pos.wrapping_add(2 as std::ffi::c_int as size_t);
        }
        3 => {
            dictID = MEM_readLE32(ip.offset(pos as isize) as *const std::ffi::c_void);
            pos = pos.wrapping_add(4 as std::ffi::c_int as size_t);
        }
        0 | _ => {}
    }
    match fcsID {
        1 => {
            frameContentSize = (MEM_readLE16(ip.offset(pos as isize) as *const std::ffi::c_void)
                as std::ffi::c_int
                + 256 as std::ffi::c_int) as U64;
        }
        2 => {
            frameContentSize =
                MEM_readLE32(ip.offset(pos as isize) as *const std::ffi::c_void) as U64;
        }
        3 => {
            frameContentSize = MEM_readLE64(ip.offset(pos as isize) as *const std::ffi::c_void);
        }
        0 | _ => {
            if singleSegment != 0 {
                frameContentSize = *ip.offset(pos as isize) as U64;
            }
        }
    }
    if singleSegment != 0 {
        windowSize = frameContentSize;
    }
    (*zfhPtr).frameType = ZSTD_frame;
    (*zfhPtr).frameContentSize = frameContentSize as std::ffi::c_ulonglong;
    (*zfhPtr).windowSize = windowSize as std::ffi::c_ulonglong;
    (*zfhPtr).blockSizeMax =
        (if windowSize < ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as U64 {
            windowSize
        } else {
            ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as U64
        }) as std::ffi::c_uint;
    (*zfhPtr).dictID = dictID;
    (*zfhPtr).checksumFlag = checksumFlag;
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameHeader(
    mut zfhPtr: *mut ZSTD_FrameHeader,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_getFrameHeader_advanced(zfhPtr, src, srcSize, ZSTD_f_zstd1)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getFrameContentSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_ulonglong {
    if ZSTD_isLegacy(src, srcSize) != 0 {
        let ret = ZSTD_getDecompressedSize_legacy(src, srcSize);
        return if ret == 0 as std::ffi::c_int as std::ffi::c_ulonglong {
            ZSTD_CONTENTSIZE_UNKNOWN
        } else {
            ret
        };
    }
    let mut zfh = ZSTD_FrameHeader {
        frameContentSize: 0,
        windowSize: 0,
        blockSizeMax: 0,
        frameType: ZSTD_frame,
        headerSize: 0,
        dictID: 0,
        checksumFlag: 0,
        _reserved1: 0,
        _reserved2: 0,
    };
    if ZSTD_getFrameHeader(&mut zfh, src, srcSize) != 0 as std::ffi::c_int as size_t {
        return ZSTD_CONTENTSIZE_ERROR;
    }
    if zfh.frameType as std::ffi::c_uint
        == ZSTD_skippableFrame as std::ffi::c_int as std::ffi::c_uint
    {
        0 as std::ffi::c_int as std::ffi::c_ulonglong
    } else {
        zfh.frameContentSize
    }
}
unsafe extern "C" fn readSkippableFrameSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let skippableHeaderSize = ZSTD_SKIPPABLEHEADERSIZE as size_t;
    let mut sizeU32: u32 = 0;
    if srcSize < 8 as std::ffi::c_int as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    sizeU32 = MEM_readLE32(
        (src as *const u8).offset(ZSTD_FRAMEIDSIZE as isize) as *const std::ffi::c_void
    );
    if sizeU32.wrapping_add(8 as std::ffi::c_int as u32) < sizeU32 {
        return -(ZSTD_error_frameParameter_unsupported as std::ffi::c_int) as size_t;
    }
    let skippableSize = skippableHeaderSize.wrapping_add(sizeU32 as size_t);
    if skippableSize > srcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    skippableSize
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_readSkippableFrame(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut magicVariant: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 8 as std::ffi::c_int as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let magicNumber = MEM_readLE32(src);
    let mut skippableFrameSize = readSkippableFrameSize(src, srcSize);
    let mut skippableContentSize =
        skippableFrameSize.wrapping_sub(ZSTD_SKIPPABLEHEADERSIZE as size_t);
    if ZSTD_isSkippableFrame(src, srcSize) == 0 {
        return -(ZSTD_error_frameParameter_unsupported as std::ffi::c_int) as size_t;
    }
    if skippableFrameSize < 8 as std::ffi::c_int as size_t || skippableFrameSize > srcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if skippableContentSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if skippableContentSize > 0 as std::ffi::c_int as size_t && !dst.is_null() {
        libc::memcpy(
            dst,
            (src as *const u8).offset(8 as std::ffi::c_int as isize) as *const std::ffi::c_void,
            skippableContentSize as libc::size_t,
        );
    }
    if !magicVariant.is_null() {
        *magicVariant = magicNumber.wrapping_sub(ZSTD_MAGIC_SKIPPABLE_START as u32);
    }
    skippableContentSize
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findDecompressedSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_ulonglong {
    let mut totalDstSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    while srcSize >= ZSTD_startingInputLength(ZSTD_f_zstd1) {
        let magicNumber = MEM_readLE32(src);
        if magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
        {
            let skippableSize = readSkippableFrameSize(src, srcSize);
            if ERR_isError(skippableSize) != 0 {
                return ZSTD_CONTENTSIZE_ERROR;
            }
            src = (src as *const u8).offset(skippableSize as isize) as *const std::ffi::c_void;
            srcSize = srcSize.wrapping_sub(skippableSize);
        } else {
            let fcs = ZSTD_getFrameContentSize(src, srcSize);
            if fcs >= ZSTD_CONTENTSIZE_ERROR {
                return fcs;
            }
            if totalDstSize.wrapping_add(fcs) < totalDstSize {
                return ZSTD_CONTENTSIZE_ERROR;
            }
            totalDstSize = totalDstSize.wrapping_add(fcs);
            let frameSrcSize = ZSTD_findFrameCompressedSize(src, srcSize);
            if ERR_isError(frameSrcSize) != 0 {
                return ZSTD_CONTENTSIZE_ERROR;
            }
            src = (src as *const u8).offset(frameSrcSize as isize) as *const std::ffi::c_void;
            srcSize = srcSize.wrapping_sub(frameSrcSize);
        }
    }
    if srcSize != 0 {
        return ZSTD_CONTENTSIZE_ERROR;
    }
    totalDstSize
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDecompressedSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_ulonglong {
    let ret = ZSTD_getFrameContentSize(src, srcSize);
    if ret >= ZSTD_CONTENTSIZE_ERROR {
        0 as std::ffi::c_int as std::ffi::c_ulonglong
    } else {
        ret
    }
}
unsafe extern "C" fn ZSTD_decodeFrameHeader(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const std::ffi::c_void,
    mut headerSize: size_t,
) -> size_t {
    let result =
        ZSTD_getFrameHeader_advanced(&mut (*dctx).fParams, src, headerSize, (*dctx).format);
    if ERR_isError(result) != 0 {
        return result;
    }
    if result > 0 as std::ffi::c_int as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if (*dctx).refMultipleDDicts as std::ffi::c_uint
        == ZSTD_rmd_refMultipleDDicts as std::ffi::c_int as std::ffi::c_uint
        && !((*dctx).ddictSet).is_null()
    {
        ZSTD_DCtx_selectFrameDDict(dctx);
    }
    if (*dctx).fParams.dictID != 0 && (*dctx).dictID != (*dctx).fParams.dictID {
        return -(ZSTD_error_dictionary_wrong as std::ffi::c_int) as size_t;
    }
    (*dctx).validateChecksum =
        (if (*dctx).fParams.checksumFlag != 0 && (*dctx).forceIgnoreChecksum as u64 == 0 {
            1 as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        }) as u32;
    if (*dctx).validateChecksum != 0 {
        ZSTD_XXH64_reset(&mut (*dctx).xxhState, 0 as std::ffi::c_int as XXH64_hash_t);
    }
    (*dctx).processedCSize =
        ((*dctx).processedCSize as std::ffi::c_ulong).wrapping_add(headerSize) as U64 as U64;
    0 as std::ffi::c_int as size_t
}
unsafe extern "C" fn ZSTD_errorFrameSizeInfo(mut ret: size_t) -> ZSTD_frameSizeInfo {
    let mut frameSizeInfo = ZSTD_frameSizeInfo {
        nbBlocks: 0,
        compressedSize: 0,
        decompressedBound: 0,
    };
    frameSizeInfo.compressedSize = ret;
    frameSizeInfo.decompressedBound = ZSTD_CONTENTSIZE_ERROR;
    frameSizeInfo
}
unsafe extern "C" fn ZSTD_findFrameSizeInfo(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut format: ZSTD_format_e,
) -> ZSTD_frameSizeInfo {
    let mut frameSizeInfo = ZSTD_frameSizeInfo {
        nbBlocks: 0,
        compressedSize: 0,
        decompressedBound: 0,
    };
    libc::memset(
        &mut frameSizeInfo as *mut ZSTD_frameSizeInfo as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<ZSTD_frameSizeInfo>() as std::ffi::c_ulong as libc::size_t,
    );
    if format as std::ffi::c_uint == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
        && ZSTD_isLegacy(src, srcSize) != 0
    {
        return ZSTD_findFrameSizeInfoLegacy(src, srcSize);
    }
    if format as std::ffi::c_uint == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
        && srcSize >= ZSTD_SKIPPABLEHEADERSIZE as size_t
        && MEM_readLE32(src) & ZSTD_MAGIC_SKIPPABLE_MASK
            == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
    {
        frameSizeInfo.compressedSize = readSkippableFrameSize(src, srcSize);
        frameSizeInfo
    } else {
        let mut ip = src as *const u8;
        let ipstart = ip;
        let mut remainingSize = srcSize;
        let mut nbBlocks = 0 as std::ffi::c_int as size_t;
        let mut zfh = ZSTD_FrameHeader {
            frameContentSize: 0,
            windowSize: 0,
            blockSizeMax: 0,
            frameType: ZSTD_frame,
            headerSize: 0,
            dictID: 0,
            checksumFlag: 0,
            _reserved1: 0,
            _reserved2: 0,
        };
        let ret = ZSTD_getFrameHeader_advanced(&mut zfh, src, srcSize, format);
        if ERR_isError(ret) != 0 {
            return ZSTD_errorFrameSizeInfo(ret);
        }
        if ret > 0 as std::ffi::c_int as size_t {
            return ZSTD_errorFrameSizeInfo(
                -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t,
            );
        }
        ip = ip.offset(zfh.headerSize as isize);
        remainingSize = remainingSize.wrapping_sub(zfh.headerSize as size_t);
        loop {
            let mut blockProperties = blockProperties_t {
                blockType: bt_raw,
                lastBlock: 0,
                origSize: 0,
            };
            let cBlockSize = ZSTD_getcBlockSize(
                ip as *const std::ffi::c_void,
                remainingSize,
                &mut blockProperties,
            );
            if ERR_isError(cBlockSize) != 0 {
                return ZSTD_errorFrameSizeInfo(cBlockSize);
            }
            if ZSTD_blockHeaderSize.wrapping_add(cBlockSize) > remainingSize {
                return ZSTD_errorFrameSizeInfo(
                    -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t,
                );
            }
            ip = ip.offset(ZSTD_blockHeaderSize.wrapping_add(cBlockSize) as isize);
            remainingSize =
                remainingSize.wrapping_sub(ZSTD_blockHeaderSize.wrapping_add(cBlockSize));
            nbBlocks = nbBlocks.wrapping_add(1);
            nbBlocks;
            if blockProperties.lastBlock != 0 {
                break;
            }
        }
        if zfh.checksumFlag != 0 {
            if remainingSize < 4 as std::ffi::c_int as size_t {
                return ZSTD_errorFrameSizeInfo(
                    -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t,
                );
            }
            ip = ip.offset(4 as std::ffi::c_int as isize);
        }
        frameSizeInfo.nbBlocks = nbBlocks;
        frameSizeInfo.compressedSize = ip.offset_from(ipstart) as std::ffi::c_long as size_t;
        frameSizeInfo.decompressedBound = if zfh.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN {
            zfh.frameContentSize
        } else {
            (nbBlocks as std::ffi::c_ulonglong)
                .wrapping_mul(zfh.blockSizeMax as std::ffi::c_ulonglong)
        };
        frameSizeInfo
    }
}
unsafe extern "C" fn ZSTD_findFrameCompressedSize_advanced(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut format: ZSTD_format_e,
) -> size_t {
    let frameSizeInfo = ZSTD_findFrameSizeInfo(src, srcSize, format);
    frameSizeInfo.compressedSize
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_findFrameCompressedSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_findFrameCompressedSize_advanced(src, srcSize, ZSTD_f_zstd1)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBound(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_ulonglong {
    let mut bound = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    while srcSize > 0 as std::ffi::c_int as size_t {
        let frameSizeInfo = ZSTD_findFrameSizeInfo(src, srcSize, ZSTD_f_zstd1);
        let compressedSize = frameSizeInfo.compressedSize;
        let decompressedBound = frameSizeInfo.decompressedBound;
        if ERR_isError(compressedSize) != 0 || decompressedBound == ZSTD_CONTENTSIZE_ERROR {
            return ZSTD_CONTENTSIZE_ERROR;
        }
        src = (src as *const u8).offset(compressedSize as isize) as *const std::ffi::c_void;
        srcSize = srcSize.wrapping_sub(compressedSize);
        bound = bound.wrapping_add(decompressedBound);
    }
    bound
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressionMargin(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut margin = 0 as std::ffi::c_int as size_t;
    let mut maxBlockSize = 0 as std::ffi::c_int as std::ffi::c_uint;
    while srcSize > 0 as std::ffi::c_int as size_t {
        let frameSizeInfo = ZSTD_findFrameSizeInfo(src, srcSize, ZSTD_f_zstd1);
        let compressedSize = frameSizeInfo.compressedSize;
        let decompressedBound = frameSizeInfo.decompressedBound;
        let mut zfh = ZSTD_FrameHeader {
            frameContentSize: 0,
            windowSize: 0,
            blockSizeMax: 0,
            frameType: ZSTD_frame,
            headerSize: 0,
            dictID: 0,
            checksumFlag: 0,
            _reserved1: 0,
            _reserved2: 0,
        };
        let err_code = ZSTD_getFrameHeader(&mut zfh, src, srcSize);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        if ERR_isError(compressedSize) != 0 || decompressedBound == ZSTD_CONTENTSIZE_ERROR {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        if zfh.frameType as std::ffi::c_uint == ZSTD_frame as std::ffi::c_int as std::ffi::c_uint {
            margin = margin.wrapping_add(zfh.headerSize as size_t);
            margin = margin.wrapping_add(
                (if zfh.checksumFlag != 0 {
                    4 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                }) as size_t,
            );
            margin = margin.wrapping_add(3 as std::ffi::c_int as size_t * frameSizeInfo.nbBlocks);
            maxBlockSize = if maxBlockSize > zfh.blockSizeMax {
                maxBlockSize
            } else {
                zfh.blockSizeMax
            };
        } else {
            margin = margin.wrapping_add(compressedSize);
        }
        src = (src as *const u8).offset(compressedSize as isize) as *const std::ffi::c_void;
        srcSize = srcSize.wrapping_sub(compressedSize);
    }
    margin = margin.wrapping_add(maxBlockSize as size_t);
    margin
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_insertBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut blockStart: *const std::ffi::c_void,
    mut blockSize: size_t,
) -> size_t {
    ZSTD_checkContinuity(dctx, blockStart, blockSize);
    (*dctx).previousDstEnd = (blockStart as *const std::ffi::c_char).offset(blockSize as isize)
        as *const std::ffi::c_void;
    blockSize
}
unsafe extern "C" fn ZSTD_copyRawBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if dst.is_null() {
        if srcSize == 0 as std::ffi::c_int as size_t {
            return 0 as std::ffi::c_int as size_t;
        }
        return -(ZSTD_error_dstBuffer_null as std::ffi::c_int) as size_t;
    }
    libc::memmove(dst, src, srcSize as libc::size_t);
    srcSize
}
unsafe extern "C" fn ZSTD_setRleBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut b: u8,
    mut regenSize: size_t,
) -> size_t {
    if regenSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if dst.is_null() {
        if regenSize == 0 as std::ffi::c_int as size_t {
            return 0 as std::ffi::c_int as size_t;
        }
        return -(ZSTD_error_dstBuffer_null as std::ffi::c_int) as size_t;
    }
    libc::memset(dst, b as std::ffi::c_int, regenSize as libc::size_t);
    regenSize
}
unsafe extern "C" fn ZSTD_DCtx_trace_end(
    mut dctx: *const ZSTD_DCtx,
    mut uncompressedSize: U64,
    mut compressedSize: U64,
    mut streaming: std::ffi::c_int,
) {
    if (*dctx).traceCtx != 0
        && (Some(
            ZSTD_trace_decompress_end
                as unsafe extern "C" fn(ZSTD_TraceCtx, *const ZSTD_Trace) -> (),
        ))
        .is_some()
    {
        let mut trace = ZSTD_Trace {
            version: 0,
            streaming: 0,
            dictionaryID: 0,
            dictionaryIsCold: 0,
            dictionarySize: 0,
            uncompressedSize: 0,
            compressedSize: 0,
            params: std::ptr::null::<ZSTD_CCtx_params_s>(),
            cctx: std::ptr::null::<ZSTD_CCtx_s>(),
            dctx: std::ptr::null::<ZSTD_DCtx_s>(),
        };
        libc::memset(
            &mut trace as *mut ZSTD_Trace as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            ::core::mem::size_of::<ZSTD_Trace>() as std::ffi::c_ulong as libc::size_t,
        );
        trace.version = ZSTD_VERSION_NUMBER as std::ffi::c_uint;
        trace.streaming = streaming;
        if !((*dctx).ddict).is_null() {
            trace.dictionaryID = ZSTD_getDictID_fromDDict((*dctx).ddict);
            trace.dictionarySize = ZSTD_DDict_dictSize((*dctx).ddict);
            trace.dictionaryIsCold = (*dctx).ddictIsCold;
        }
        trace.uncompressedSize = uncompressedSize;
        trace.compressedSize = compressedSize;
        trace.dctx = dctx;
        ZSTD_trace_decompress_end((*dctx).traceCtx, &mut trace);
    }
}
unsafe extern "C" fn ZSTD_decompressFrame(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut srcPtr: *mut *const std::ffi::c_void,
    mut srcSizePtr: *mut size_t,
) -> size_t {
    let istart = *srcPtr as *const u8;
    let mut ip = istart;
    let ostart = dst as *mut u8;
    let oend = if dstCapacity != 0 as std::ffi::c_int as size_t {
        ostart.offset(dstCapacity as isize)
    } else {
        ostart
    };
    let mut op = ostart;
    let mut remainingSrcSize = *srcSizePtr;
    if remainingSrcSize
        < ((if (*dctx).format as std::ffi::c_uint
            == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
        {
            6 as std::ffi::c_int
        } else {
            2 as std::ffi::c_int
        }) as size_t)
            .wrapping_add(ZSTD_blockHeaderSize)
    {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let frameHeaderSize = ZSTD_frameHeaderSize_internal(
        ip as *const std::ffi::c_void,
        (if (*dctx).format as std::ffi::c_uint
            == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
        {
            5 as std::ffi::c_int
        } else {
            1 as std::ffi::c_int
        }) as size_t,
        (*dctx).format,
    );
    if ERR_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    if remainingSrcSize < frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let err_code = ZSTD_decodeFrameHeader(dctx, ip as *const std::ffi::c_void, frameHeaderSize);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    ip = ip.offset(frameHeaderSize as isize);
    remainingSrcSize = remainingSrcSize.wrapping_sub(frameHeaderSize);
    if (*dctx).maxBlockSizeParam != 0 as std::ffi::c_int {
        (*dctx).fParams.blockSizeMax =
            if (*dctx).fParams.blockSizeMax < (*dctx).maxBlockSizeParam as std::ffi::c_uint {
                (*dctx).fParams.blockSizeMax
            } else {
                (*dctx).maxBlockSizeParam as std::ffi::c_uint
            };
    }
    loop {
        let mut oBlockEnd = oend;
        let mut decodedSize: size_t = 0;
        let mut blockProperties = blockProperties_t {
            blockType: bt_raw,
            lastBlock: 0,
            origSize: 0,
        };
        let cBlockSize = ZSTD_getcBlockSize(
            ip as *const std::ffi::c_void,
            remainingSrcSize,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            return cBlockSize;
        }
        ip = ip.offset(ZSTD_blockHeaderSize as isize);
        remainingSrcSize = remainingSrcSize.wrapping_sub(ZSTD_blockHeaderSize);
        if cBlockSize > remainingSrcSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        if ip >= op as *const u8 && ip < oBlockEnd as *const u8 {
            oBlockEnd = op.offset(ip.offset_from(op) as std::ffi::c_long as isize);
        }
        match blockProperties.blockType as std::ffi::c_uint {
            2 => {
                decodedSize = ZSTD_decompressBlock_internal(
                    dctx,
                    op as *mut std::ffi::c_void,
                    oBlockEnd.offset_from(op) as std::ffi::c_long as size_t,
                    ip as *const std::ffi::c_void,
                    cBlockSize,
                    not_streaming,
                );
            }
            0 => {
                decodedSize = ZSTD_copyRawBlock(
                    op as *mut std::ffi::c_void,
                    oend.offset_from(op) as std::ffi::c_long as size_t,
                    ip as *const std::ffi::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTD_setRleBlock(
                    op as *mut std::ffi::c_void,
                    oBlockEnd.offset_from(op) as std::ffi::c_long as size_t,
                    *ip,
                    blockProperties.origSize as size_t,
                );
            }
            3 | _ => {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
        }
        let err_code_0 = decodedSize;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        if (*dctx).validateChecksum != 0 {
            ZSTD_XXH64_update(
                &mut (*dctx).xxhState,
                op as *const std::ffi::c_void,
                decodedSize,
            );
        }
        if decodedSize != 0 {
            op = op.offset(decodedSize as isize);
        }
        ip = ip.offset(cBlockSize as isize);
        remainingSrcSize = remainingSrcSize.wrapping_sub(cBlockSize);
        if blockProperties.lastBlock != 0 {
            break;
        }
    }
    if (*dctx).fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
        && op.offset_from(ostart) as std::ffi::c_long as U64 as std::ffi::c_ulonglong
            != (*dctx).fParams.frameContentSize
    {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if (*dctx).fParams.checksumFlag != 0 {
        if remainingSrcSize < 4 as std::ffi::c_int as size_t {
            return -(ZSTD_error_checksum_wrong as std::ffi::c_int) as size_t;
        }
        if (*dctx).forceIgnoreChecksum as u64 == 0 {
            let checkCalc = ZSTD_XXH64_digest(&mut (*dctx).xxhState) as u32;
            let mut checkRead: u32 = 0;
            checkRead = MEM_readLE32(ip as *const std::ffi::c_void);
            if checkRead != checkCalc {
                return -(ZSTD_error_checksum_wrong as std::ffi::c_int) as size_t;
            }
        }
        ip = ip.offset(4 as std::ffi::c_int as isize);
        remainingSrcSize = remainingSrcSize.wrapping_sub(4 as std::ffi::c_int as size_t);
    }
    ZSTD_DCtx_trace_end(
        dctx,
        op.offset_from(ostart) as std::ffi::c_long as U64,
        ip.offset_from(istart) as std::ffi::c_long as U64,
        0 as std::ffi::c_int,
    );
    *srcPtr = ip as *const std::ffi::c_void;
    *srcSizePtr = remainingSrcSize;
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTD_decompressMultiFrame(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
    mut ddict: *const ZSTD_DDict,
) -> size_t {
    let dststart = dst;
    let mut moreThan1Frame = 0 as std::ffi::c_int;
    if !ddict.is_null() {
        dict = ZSTD_DDict_dictContent(ddict);
        dictSize = ZSTD_DDict_dictSize(ddict);
    }
    while srcSize >= ZSTD_startingInputLength((*dctx).format) {
        if (*dctx).format as std::ffi::c_uint == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
            && ZSTD_isLegacy(src, srcSize) != 0
        {
            let mut decodedSize: size_t = 0;
            let frameSize = ZSTD_findFrameCompressedSizeLegacy(src, srcSize);
            if ERR_isError(frameSize) != 0 {
                return frameSize;
            }
            if (*dctx).staticSize != 0 {
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            decodedSize = ZSTD_decompressLegacy(dst, dstCapacity, src, frameSize, dict, dictSize);
            if ERR_isError(decodedSize) != 0 {
                return decodedSize;
            }
            let expectedSize = ZSTD_getFrameContentSize(src, srcSize);
            if expectedSize
                == (0 as std::ffi::c_ulonglong)
                    .wrapping_sub(2 as std::ffi::c_int as std::ffi::c_ulonglong)
            {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if expectedSize != ZSTD_CONTENTSIZE_UNKNOWN
                && expectedSize != decodedSize as std::ffi::c_ulonglong
            {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            dst = (dst as *mut u8).offset(decodedSize as isize) as *mut std::ffi::c_void;
            dstCapacity = dstCapacity.wrapping_sub(decodedSize);
            src = (src as *const u8).offset(frameSize as isize) as *const std::ffi::c_void;
            srcSize = srcSize.wrapping_sub(frameSize);
        } else {
            if (*dctx).format as std::ffi::c_uint
                == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
                && srcSize >= 4 as std::ffi::c_int as size_t
            {
                let magicNumber = MEM_readLE32(src);
                if magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK
                    == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
                {
                    let skippableSize = readSkippableFrameSize(src, srcSize);
                    let err_code = skippableSize;
                    if ERR_isError(err_code) != 0 {
                        return err_code;
                    }
                    src = (src as *const u8).offset(skippableSize as isize)
                        as *const std::ffi::c_void;
                    srcSize = srcSize.wrapping_sub(skippableSize);
                    continue;
                }
            }
            if !ddict.is_null() {
                let err_code_0 = ZSTD_decompressBegin_usingDDict(dctx, ddict);
                if ERR_isError(err_code_0) != 0 {
                    return err_code_0;
                }
            } else {
                let err_code_1 = ZSTD_decompressBegin_usingDict(dctx, dict, dictSize);
                if ERR_isError(err_code_1) != 0 {
                    return err_code_1;
                }
            }
            ZSTD_checkContinuity(dctx, dst, dstCapacity);
            let res = ZSTD_decompressFrame(dctx, dst, dstCapacity, &mut src, &mut srcSize);
            if ZSTD_getErrorCode(res) as std::ffi::c_uint
                == ZSTD_error_prefix_unknown as std::ffi::c_int as std::ffi::c_uint
                && moreThan1Frame == 1 as std::ffi::c_int
            {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            if ERR_isError(res) != 0 {
                return res;
            }
            if res != 0 as std::ffi::c_int as size_t {
                dst = (dst as *mut u8).offset(res as isize) as *mut std::ffi::c_void;
            }
            dstCapacity = dstCapacity.wrapping_sub(res);
            moreThan1Frame = 1 as std::ffi::c_int;
        }
    }
    if srcSize != 0 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    (dst as *mut u8).offset_from(dststart as *mut u8) as std::ffi::c_long as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_usingDict(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    ZSTD_decompressMultiFrame(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        dict,
        dictSize,
        NULL as *const ZSTD_DDict,
    )
}
unsafe extern "C" fn ZSTD_getDDict(mut dctx: *mut ZSTD_DCtx) -> *const ZSTD_DDict {
    match (*dctx).dictUses as std::ffi::c_int {
        -1 => (*dctx).ddict,
        1 => {
            (*dctx).dictUses = ZSTD_dont_use;
            (*dctx).ddict
        }
        0 | _ => {
            ZSTD_clearDict(dctx);
            NULL as *const ZSTD_DDict
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressDCtx(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_decompress_usingDDict(dctx, dst, dstCapacity, src, srcSize, ZSTD_getDDict(dctx))
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut regenSize: size_t = 0;
    let dctx = ZSTD_createDCtx_internal(ZSTD_defaultCMem);
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    regenSize = ZSTD_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTD_freeDCtx(dctx);
    regenSize
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_nextSrcSizeToDecompress(mut dctx: *mut ZSTD_DCtx) -> size_t {
    (*dctx).expected
}
unsafe extern "C" fn ZSTD_nextSrcSizeToDecompressWithInputSize(
    mut dctx: *mut ZSTD_DCtx,
    mut inputSize: size_t,
) -> size_t {
    if !((*dctx).stage as std::ffi::c_uint
        == ZSTDds_decompressBlock as std::ffi::c_int as std::ffi::c_uint
        || (*dctx).stage as std::ffi::c_uint
            == ZSTDds_decompressLastBlock as std::ffi::c_int as std::ffi::c_uint)
    {
        return (*dctx).expected;
    }
    if (*dctx).bType as std::ffi::c_uint != bt_raw as std::ffi::c_int as std::ffi::c_uint {
        return (*dctx).expected;
    }
    if 1 as std::ffi::c_int as size_t
        > (if inputSize < (*dctx).expected {
            inputSize
        } else {
            (*dctx).expected
        })
    {
        1 as std::ffi::c_int as size_t
    } else if inputSize < (*dctx).expected {
        inputSize
    } else {
        (*dctx).expected
    }
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_nextInputType(mut dctx: *mut ZSTD_DCtx) -> ZSTD_nextInputType_e {
    match (*dctx).stage as std::ffi::c_uint {
        2 => ZSTDnit_blockHeader,
        3 => ZSTDnit_block,
        4 => ZSTDnit_lastBlock,
        5 => ZSTDnit_checksum,
        6 | 7 => ZSTDnit_skippableFrame,
        0 | 1 | _ => ZSTDnit_frameHeader,
    }
}
unsafe extern "C" fn ZSTD_isSkipFrame(mut dctx: *mut ZSTD_DCtx) -> std::ffi::c_int {
    ((*dctx).stage as std::ffi::c_uint == ZSTDds_skipFrame as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressContinue(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize != ZSTD_nextSrcSizeToDecompressWithInputSize(dctx, srcSize) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ZSTD_checkContinuity(dctx, dst, dstCapacity);
    (*dctx).processedCSize =
        ((*dctx).processedCSize as std::ffi::c_ulong).wrapping_add(srcSize) as U64 as U64;
    match (*dctx).stage as std::ffi::c_uint {
        0 => {
            if (*dctx).format as std::ffi::c_uint
                == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
                && MEM_readLE32(src) & ZSTD_MAGIC_SKIPPABLE_MASK
                    == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
            {
                libc::memcpy(
                    ((*dctx).headerBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                    src,
                    srcSize as libc::size_t,
                );
                (*dctx).expected = (ZSTD_SKIPPABLEHEADERSIZE as size_t).wrapping_sub(srcSize);
                (*dctx).stage = ZSTDds_decodeSkippableHeader;
                return 0 as std::ffi::c_int as size_t;
            }
            (*dctx).headerSize = ZSTD_frameHeaderSize_internal(src, srcSize, (*dctx).format);
            if ERR_isError((*dctx).headerSize) != 0 {
                return (*dctx).headerSize;
            }
            libc::memcpy(
                ((*dctx).headerBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                src,
                srcSize as libc::size_t,
            );
            (*dctx).expected = ((*dctx).headerSize).wrapping_sub(srcSize);
            (*dctx).stage = ZSTDds_decodeFrameHeader;
            0 as std::ffi::c_int as size_t
        }
        1 => {
            libc::memcpy(
                ((*dctx).headerBuffer)
                    .as_mut_ptr()
                    .offset(((*dctx).headerSize).wrapping_sub(srcSize) as isize)
                    as *mut std::ffi::c_void,
                src,
                srcSize as libc::size_t,
            );
            let err_code = ZSTD_decodeFrameHeader(
                dctx,
                ((*dctx).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                (*dctx).headerSize,
            );
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            (*dctx).expected = ZSTD_blockHeaderSize;
            (*dctx).stage = ZSTDds_decodeBlockHeader;
            0 as std::ffi::c_int as size_t
        }
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_raw,
                lastBlock: 0,
                origSize: 0,
            };
            let cBlockSize = ZSTD_getcBlockSize(src, ZSTD_blockHeaderSize, &mut bp);
            if ERR_isError(cBlockSize) != 0 {
                return cBlockSize;
            }
            if cBlockSize > (*dctx).fParams.blockSizeMax as size_t {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            (*dctx).expected = cBlockSize;
            (*dctx).bType = bp.blockType;
            (*dctx).rleSize = bp.origSize as size_t;
            if cBlockSize != 0 {
                (*dctx).stage = (if bp.lastBlock != 0 {
                    ZSTDds_decompressLastBlock as std::ffi::c_int
                } else {
                    ZSTDds_decompressBlock as std::ffi::c_int
                }) as ZSTD_dStage;
                return 0 as std::ffi::c_int as size_t;
            }
            if bp.lastBlock != 0 {
                if (*dctx).fParams.checksumFlag != 0 {
                    (*dctx).expected = 4 as std::ffi::c_int as size_t;
                    (*dctx).stage = ZSTDds_checkChecksum;
                } else {
                    (*dctx).expected = 0 as std::ffi::c_int as size_t;
                    (*dctx).stage = ZSTDds_getFrameHeaderSize;
                }
            } else {
                (*dctx).expected = ZSTD_blockHeaderSize;
                (*dctx).stage = ZSTDds_decodeBlockHeader;
            }
            0 as std::ffi::c_int as size_t
        }
        4 | 3 => {
            let mut rSize: size_t = 0;
            match (*dctx).bType as std::ffi::c_uint {
                2 => {
                    rSize = ZSTD_decompressBlock_internal(
                        dctx,
                        dst,
                        dstCapacity,
                        src,
                        srcSize,
                        is_streaming,
                    );
                    (*dctx).expected = 0 as std::ffi::c_int as size_t;
                }
                0 => {
                    rSize = ZSTD_copyRawBlock(dst, dstCapacity, src, srcSize);
                    let err_code_0 = rSize;
                    if ERR_isError(err_code_0) != 0 {
                        return err_code_0;
                    }
                    (*dctx).expected = ((*dctx).expected).wrapping_sub(rSize);
                }
                1 => {
                    rSize =
                        ZSTD_setRleBlock(dst, dstCapacity, *(src as *const u8), (*dctx).rleSize);
                    (*dctx).expected = 0 as std::ffi::c_int as size_t;
                }
                3 | _ => {
                    return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                }
            }
            let err_code_1 = rSize;
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            if rSize > (*dctx).fParams.blockSizeMax as size_t {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            (*dctx).decodedSize =
                ((*dctx).decodedSize as std::ffi::c_ulong).wrapping_add(rSize) as U64 as U64;
            if (*dctx).validateChecksum != 0 {
                ZSTD_XXH64_update(&mut (*dctx).xxhState, dst, rSize);
            }
            (*dctx).previousDstEnd =
                (dst as *mut std::ffi::c_char).offset(rSize as isize) as *const std::ffi::c_void;
            if (*dctx).expected > 0 as std::ffi::c_int as size_t {
                return rSize;
            }
            if (*dctx).stage as std::ffi::c_uint
                == ZSTDds_decompressLastBlock as std::ffi::c_int as std::ffi::c_uint
            {
                if (*dctx).fParams.frameContentSize
                    != (0 as std::ffi::c_ulonglong)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong)
                    && (*dctx).decodedSize as std::ffi::c_ulonglong
                        != (*dctx).fParams.frameContentSize
                {
                    return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                }
                if (*dctx).fParams.checksumFlag != 0 {
                    (*dctx).expected = 4 as std::ffi::c_int as size_t;
                    (*dctx).stage = ZSTDds_checkChecksum;
                } else {
                    ZSTD_DCtx_trace_end(
                        dctx,
                        (*dctx).decodedSize,
                        (*dctx).processedCSize,
                        1 as std::ffi::c_int,
                    );
                    (*dctx).expected = 0 as std::ffi::c_int as size_t;
                    (*dctx).stage = ZSTDds_getFrameHeaderSize;
                }
            } else {
                (*dctx).stage = ZSTDds_decodeBlockHeader;
                (*dctx).expected = ZSTD_blockHeaderSize;
            }
            rSize
        }
        5 => {
            if (*dctx).validateChecksum != 0 {
                let h32 = ZSTD_XXH64_digest(&mut (*dctx).xxhState) as u32;
                let check32 = MEM_readLE32(src);
                if check32 != h32 {
                    return -(ZSTD_error_checksum_wrong as std::ffi::c_int) as size_t;
                }
            }
            ZSTD_DCtx_trace_end(
                dctx,
                (*dctx).decodedSize,
                (*dctx).processedCSize,
                1 as std::ffi::c_int,
            );
            (*dctx).expected = 0 as std::ffi::c_int as size_t;
            (*dctx).stage = ZSTDds_getFrameHeaderSize;
            0 as std::ffi::c_int as size_t
        }
        6 => {
            libc::memcpy(
                ((*dctx).headerBuffer)
                    .as_mut_ptr()
                    .offset((8 as std::ffi::c_int as size_t).wrapping_sub(srcSize) as isize)
                    as *mut std::ffi::c_void,
                src,
                srcSize as libc::size_t,
            );
            (*dctx).expected = MEM_readLE32(
                ((*dctx).headerBuffer)
                    .as_mut_ptr()
                    .offset(ZSTD_FRAMEIDSIZE as isize) as *const std::ffi::c_void,
            ) as size_t;
            (*dctx).stage = ZSTDds_skipFrame;
            0 as std::ffi::c_int as size_t
        }
        7 => {
            (*dctx).expected = 0 as std::ffi::c_int as size_t;
            (*dctx).stage = ZSTDds_getFrameHeaderSize;
            0 as std::ffi::c_int as size_t
        }
        _ => -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
    }
}
unsafe extern "C" fn ZSTD_refDictContent(
    mut dctx: *mut ZSTD_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).virtualStart = (dict as *const std::ffi::c_char).offset(
        -(((*dctx).previousDstEnd as *const std::ffi::c_char)
            .offset_from((*dctx).prefixStart as *const std::ffi::c_char)
            as std::ffi::c_long as isize),
    ) as *const std::ffi::c_void;
    (*dctx).prefixStart = dict;
    (*dctx).previousDstEnd =
        (dict as *const std::ffi::c_char).offset(dictSize as isize) as *const std::ffi::c_void;
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_loadDEntropy(
    mut entropy: *mut ZSTD_entropyDTables_t,
    dict: *const std::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let mut dictPtr = dict as *const u8;
    let dictEnd = dictPtr.offset(dictSize as isize);
    if dictSize <= 8 as std::ffi::c_int as size_t {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dictPtr = dictPtr.offset(8 as std::ffi::c_int as isize);
    let workspace = &mut (*entropy).LLTable as *mut [ZSTD_seqSymbol; 513] as *mut std::ffi::c_void;
    let workspaceSize = (::core::mem::size_of::<[ZSTD_seqSymbol; 513]>() as std::ffi::c_ulong)
        .wrapping_add(::core::mem::size_of::<[ZSTD_seqSymbol; 257]>() as std::ffi::c_ulong)
        .wrapping_add(::core::mem::size_of::<[ZSTD_seqSymbol; 513]>() as std::ffi::c_ulong);
    let hSize = HUF_readDTableX2_wksp(
        ((*entropy).hufTable).as_mut_ptr(),
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as std::ffi::c_long as size_t,
        workspace,
        workspaceSize,
        0 as std::ffi::c_int,
    );
    if ERR_isError(hSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dictPtr = dictPtr.offset(hSize as isize);
    let mut offcodeNCount: [std::ffi::c_short; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff as std::ffi::c_uint;
    let mut offcodeLog: std::ffi::c_uint = 0;
    let offcodeHeaderSize = FSE_readNCount(
        &mut offcodeNCount,
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as std::ffi::c_long as size_t,
    );
    if ERR_isError(offcodeHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if offcodeMaxValue > 31 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if offcodeLog > 8 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    ZSTD_buildFSETable(
        ((*entropy).OFTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        OF_base.as_ptr(),
        OF_bits.as_ptr(),
        offcodeLog,
        ((*entropy).workspace).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 157]>() as std::ffi::c_ulong,
        0 as std::ffi::c_int,
    );
    dictPtr = dictPtr.offset(offcodeHeaderSize as isize);
    let mut matchlengthNCount: [std::ffi::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as std::ffi::c_uint;
    let mut matchlengthLog: std::ffi::c_uint = 0;
    let matchlengthHeaderSize = FSE_readNCount(
        &mut matchlengthNCount,
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as std::ffi::c_long as size_t,
    );
    if ERR_isError(matchlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if matchlengthMaxValue > 52 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if matchlengthLog > 9 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    ZSTD_buildFSETable(
        ((*entropy).MLTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        ML_base.as_ptr(),
        ML_bits.as_ptr(),
        matchlengthLog,
        ((*entropy).workspace).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 157]>() as std::ffi::c_ulong,
        0 as std::ffi::c_int,
    );
    dictPtr = dictPtr.offset(matchlengthHeaderSize as isize);
    let mut litlengthNCount: [std::ffi::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as std::ffi::c_uint;
    let mut litlengthLog: std::ffi::c_uint = 0;
    let litlengthHeaderSize = FSE_readNCount(
        &mut litlengthNCount,
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dictPtr as *const std::ffi::c_void,
        dictEnd.offset_from(dictPtr) as std::ffi::c_long as size_t,
    );
    if ERR_isError(litlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if litlengthMaxValue > 35 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if litlengthLog > 9 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    ZSTD_buildFSETable(
        ((*entropy).LLTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        LL_base.as_ptr(),
        LL_bits.as_ptr(),
        litlengthLog,
        ((*entropy).workspace).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u32; 157]>() as std::ffi::c_ulong,
        0 as std::ffi::c_int,
    );
    dictPtr = dictPtr.offset(litlengthHeaderSize as isize);
    if dictPtr.offset(12 as std::ffi::c_int as isize) > dictEnd {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    let mut i: std::ffi::c_int = 0;
    let dictContentSize = dictEnd.offset_from(dictPtr.offset(12 as std::ffi::c_int as isize))
        as std::ffi::c_long as size_t;
    i = 0 as std::ffi::c_int;
    while i < 3 as std::ffi::c_int {
        let rep = MEM_readLE32(dictPtr as *const std::ffi::c_void);
        dictPtr = dictPtr.offset(4 as std::ffi::c_int as isize);
        if rep == 0 as std::ffi::c_int as u32 || rep as size_t > dictContentSize {
            return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
        }
        *((*entropy).rep).as_mut_ptr().offset(i as isize) = rep;
        i += 1;
        i;
    }
    dictPtr.offset_from(dict as *const u8) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTD_decompress_insertDictionary(
    mut dctx: *mut ZSTD_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    if dictSize < 8 as std::ffi::c_int as size_t {
        return ZSTD_refDictContent(dctx, dict, dictSize);
    }
    let magic = MEM_readLE32(dict);
    if magic != ZSTD_MAGIC_DICTIONARY {
        return ZSTD_refDictContent(dctx, dict, dictSize);
    }
    (*dctx).dictID = MEM_readLE32(
        (dict as *const std::ffi::c_char).offset(ZSTD_FRAMEIDSIZE as isize)
            as *const std::ffi::c_void,
    );
    let eSize = ZSTD_loadDEntropy(&mut (*dctx).entropy, dict, dictSize);
    if ERR_isError(eSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(eSize as isize) as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(eSize);
    (*dctx).fseEntropy = 1 as std::ffi::c_int as u32;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    ZSTD_refDictContent(dctx, dict, dictSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin(mut dctx: *mut ZSTD_DCtx) -> size_t {
    (*dctx).traceCtx = if (Some(
        ZSTD_trace_decompress_begin as unsafe extern "C" fn(*const ZSTD_DCtx_s) -> ZSTD_TraceCtx,
    ))
    .is_some()
    {
        ZSTD_trace_decompress_begin(dctx)
    } else {
        0 as std::ffi::c_int as ZSTD_TraceCtx
    };
    (*dctx).expected = ZSTD_startingInputLength((*dctx).format);
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).processedCSize = 0 as std::ffi::c_int as U64;
    (*dctx).decodedSize = 0 as std::ffi::c_int as U64;
    (*dctx).previousDstEnd = NULL as *const std::ffi::c_void;
    (*dctx).prefixStart = NULL as *const std::ffi::c_void;
    (*dctx).virtualStart = NULL as *const std::ffi::c_void;
    (*dctx).dictEnd = NULL as *const std::ffi::c_void;
    *((*dctx).entropy.hufTable)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) =
        (12 as std::ffi::c_int * 0x1000001 as std::ffi::c_int) as HUF_DTable;
    (*dctx).fseEntropy = 0 as std::ffi::c_int as u32;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    (*dctx).dictID = 0 as std::ffi::c_int as u32;
    (*dctx).bType = bt_reserved;
    (*dctx).isFrameDecompression = 1 as std::ffi::c_int;
    libc::memcpy(
        ((*dctx).entropy.rep).as_mut_ptr() as *mut std::ffi::c_void,
        repStartValue.as_ptr() as *const std::ffi::c_void,
        ::core::mem::size_of::<[u32; 3]>() as std::ffi::c_ulong as libc::size_t,
    );
    (*dctx).LLTptr = ((*dctx).entropy.LLTable).as_mut_ptr();
    (*dctx).MLTptr = ((*dctx).entropy.MLTable).as_mut_ptr();
    (*dctx).OFTptr = ((*dctx).entropy.OFTable).as_mut_ptr();
    (*dctx).HUFptr = ((*dctx).entropy.hufTable).as_mut_ptr();
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDict(
    mut dctx: *mut ZSTD_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let err_code = ZSTD_decompressBegin(dctx);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if !dict.is_null()
        && dictSize != 0
        && ERR_isError(ZSTD_decompress_insertDictionary(dctx, dict, dictSize)) != 0
    {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDDict(
    mut dctx: *mut ZSTD_DCtx,
    mut ddict: *const ZSTD_DDict,
) -> size_t {
    if !ddict.is_null() {
        let dictStart = ZSTD_DDict_dictContent(ddict) as *const std::ffi::c_char;
        let dictSize = ZSTD_DDict_dictSize(ddict);
        let dictEnd = dictStart.offset(dictSize as isize) as *const std::ffi::c_void;
        (*dctx).ddictIsCold = ((*dctx).dictEnd != dictEnd) as std::ffi::c_int;
    }
    let err_code = ZSTD_decompressBegin(dctx);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if !ddict.is_null() {
        ZSTD_copyDDictParameters(dctx, ddict);
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDict(
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> std::ffi::c_uint {
    if dictSize < 8 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    if MEM_readLE32(dict) != ZSTD_MAGIC_DICTIONARY {
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    MEM_readLE32(
        (dict as *const std::ffi::c_char).offset(ZSTD_FRAMEIDSIZE as isize)
            as *const std::ffi::c_void,
    )
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromFrame(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_uint {
    let mut zfp = {
        ZSTD_FrameHeader {
            frameContentSize: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            windowSize: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            blockSizeMax: 0 as std::ffi::c_int as std::ffi::c_uint,
            frameType: ZSTD_frame,
            headerSize: 0 as std::ffi::c_int as std::ffi::c_uint,
            dictID: 0 as std::ffi::c_int as std::ffi::c_uint,
            checksumFlag: 0 as std::ffi::c_int as std::ffi::c_uint,
            _reserved1: 0 as std::ffi::c_int as std::ffi::c_uint,
            _reserved2: 0 as std::ffi::c_int as std::ffi::c_uint,
        }
    };
    let hError = ZSTD_getFrameHeader(&mut zfp, src, srcSize);
    if ERR_isError(hError) != 0 {
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    zfp.dictID
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompress_usingDDict(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut ddict: *const ZSTD_DDict,
) -> size_t {
    ZSTD_decompressMultiFrame(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        NULL as *const std::ffi::c_void,
        0 as std::ffi::c_int as size_t,
        ddict,
    )
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream() -> *mut ZSTD_DStream {
    ZSTD_createDCtx_internal(ZSTD_defaultCMem)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDStream(
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: size_t,
) -> *mut ZSTD_DStream {
    ZSTD_initStaticDCtx(workspace, workspaceSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDStream_advanced(
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_DStream {
    ZSTD_createDCtx_internal(customMem)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDStream(mut zds: *mut ZSTD_DStream) -> size_t {
    ZSTD_freeDCtx(zds)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DStreamInSize() -> size_t {
    (ZSTD_BLOCKSIZE_MAX as size_t).wrapping_add(ZSTD_blockHeaderSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DStreamOutSize() -> size_t {
    ZSTD_BLOCKSIZE_MAX as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_advanced(
    mut dctx: *mut ZSTD_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if (*dctx).streamStage as std::ffi::c_uint != zdss_init as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_stage_wrong as std::ffi::c_int) as size_t;
    }
    ZSTD_clearDict(dctx);
    if !dict.is_null() && dictSize != 0 as std::ffi::c_int as size_t {
        (*dctx).ddictLocal = ZSTD_createDDict_advanced(
            dict,
            dictSize,
            dictLoadMethod,
            dictContentType,
            (*dctx).customMem,
        );
        if ((*dctx).ddictLocal).is_null() {
            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        }
        (*dctx).ddict = (*dctx).ddictLocal;
        (*dctx).dictUses = ZSTD_use_indefinitely;
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_byReference(
    mut dctx: *mut ZSTD_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize, ZSTD_dlm_byRef, ZSTD_dct_auto)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary(
    mut dctx: *mut ZSTD_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize, ZSTD_dlm_byCopy, ZSTD_dct_auto)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix_advanced(
    mut dctx: *mut ZSTD_DCtx,
    mut prefix: *const std::ffi::c_void,
    mut prefixSize: size_t,
    mut dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    let err_code = ZSTD_DCtx_loadDictionary_advanced(
        dctx,
        prefix,
        prefixSize,
        ZSTD_dlm_byRef,
        dictContentType,
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    (*dctx).dictUses = ZSTD_use_once;
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix(
    mut dctx: *mut ZSTD_DCtx,
    mut prefix: *const std::ffi::c_void,
    mut prefixSize: size_t,
) -> size_t {
    ZSTD_DCtx_refPrefix_advanced(dctx, prefix, prefixSize, ZSTD_dct_rawContent)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDict(
    mut zds: *mut ZSTD_DStream,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let err_code = ZSTD_DCtx_reset(zds, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_DCtx_loadDictionary(zds, dict, dictSize);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    ZSTD_startingInputLength((*zds).format)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream(mut zds: *mut ZSTD_DStream) -> size_t {
    let err_code = ZSTD_DCtx_reset(zds, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_DCtx_refDDict(zds, std::ptr::null::<ZSTD_DDict>());
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    ZSTD_startingInputLength((*zds).format)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initDStream_usingDDict(
    mut dctx: *mut ZSTD_DStream,
    mut ddict: *const ZSTD_DDict,
) -> size_t {
    let err_code = ZSTD_DCtx_reset(dctx, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_DCtx_refDDict(dctx, ddict);
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    ZSTD_startingInputLength((*dctx).format)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_resetDStream(mut dctx: *mut ZSTD_DStream) -> size_t {
    let err_code = ZSTD_DCtx_reset(dctx, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    ZSTD_startingInputLength((*dctx).format)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_refDDict(
    mut dctx: *mut ZSTD_DCtx,
    mut ddict: *const ZSTD_DDict,
) -> size_t {
    if (*dctx).streamStage as std::ffi::c_uint != zdss_init as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_stage_wrong as std::ffi::c_int) as size_t;
    }
    ZSTD_clearDict(dctx);
    if !ddict.is_null() {
        (*dctx).ddict = ddict;
        (*dctx).dictUses = ZSTD_use_indefinitely;
        if (*dctx).refMultipleDDicts as std::ffi::c_uint
            == ZSTD_rmd_refMultipleDDicts as std::ffi::c_int as std::ffi::c_uint
        {
            if ((*dctx).ddictSet).is_null() {
                (*dctx).ddictSet = ZSTD_createDDictHashSet((*dctx).customMem);
                if ((*dctx).ddictSet).is_null() {
                    return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                }
            }
            let err_code = ZSTD_DDictHashSet_addDDict((*dctx).ddictSet, ddict, (*dctx).customMem);
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
        }
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setMaxWindowSize(
    mut dctx: *mut ZSTD_DCtx,
    mut maxWindowSize: size_t,
) -> size_t {
    let bounds = ZSTD_dParam_getBounds(ZSTD_d_windowLogMax);
    let min = (1 as std::ffi::c_int as size_t) << bounds.lowerBound;
    let max = (1 as std::ffi::c_int as size_t) << bounds.upperBound;
    if (*dctx).streamStage as std::ffi::c_uint != zdss_init as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_stage_wrong as std::ffi::c_int) as size_t;
    }
    if maxWindowSize < min {
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
    }
    if maxWindowSize > max {
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
    }
    (*dctx).maxWindowSize = maxWindowSize;
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setFormat(
    mut dctx: *mut ZSTD_DCtx,
    mut format: ZSTD_format_e,
) -> size_t {
    ZSTD_DCtx_setParameter(
        dctx,
        ZSTD_d_format as ZSTD_dParameter,
        format as std::ffi::c_int,
    )
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_dParam_getBounds(mut dParam: ZSTD_dParameter) -> ZSTD_bounds {
    let mut bounds = {
        ZSTD_bounds {
            error: 0 as std::ffi::c_int as size_t,
            lowerBound: 0 as std::ffi::c_int,
            upperBound: 0 as std::ffi::c_int,
        }
    };
    match dParam as std::ffi::c_uint {
        100 => {
            bounds.lowerBound = ZSTD_WINDOWLOG_ABSOLUTEMIN;
            bounds.upperBound = if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 4 as std::ffi::c_int as std::ffi::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            };
            return bounds;
        }
        1000 => {
            bounds.lowerBound = ZSTD_f_zstd1 as std::ffi::c_int;
            bounds.upperBound = ZSTD_f_zstd1_magicless as std::ffi::c_int;
            return bounds;
        }
        1001 => {
            bounds.lowerBound = ZSTD_bm_buffered as std::ffi::c_int;
            bounds.upperBound = ZSTD_bm_stable as std::ffi::c_int;
            return bounds;
        }
        1002 => {
            bounds.lowerBound = ZSTD_d_validateChecksum as std::ffi::c_int;
            bounds.upperBound = ZSTD_d_ignoreChecksum as std::ffi::c_int;
            return bounds;
        }
        1003 => {
            bounds.lowerBound = ZSTD_rmd_refSingleDDict as std::ffi::c_int;
            bounds.upperBound = ZSTD_rmd_refMultipleDDicts as std::ffi::c_int;
            return bounds;
        }
        1004 => {
            bounds.lowerBound = 0 as std::ffi::c_int;
            bounds.upperBound = 1 as std::ffi::c_int;
            return bounds;
        }
        1005 => {
            bounds.lowerBound = ZSTD_BLOCKSIZE_MAX_MIN;
            bounds.upperBound = ZSTD_BLOCKSIZE_MAX;
            return bounds;
        }
        _ => {}
    }
    bounds.error = -(ZSTD_error_parameter_unsupported as std::ffi::c_int) as size_t;
    bounds
}
unsafe extern "C" fn ZSTD_dParam_withinBounds(
    mut dParam: ZSTD_dParameter,
    mut value: std::ffi::c_int,
) -> std::ffi::c_int {
    let bounds = ZSTD_dParam_getBounds(dParam);
    if ERR_isError(bounds.error) != 0 {
        return 0 as std::ffi::c_int;
    }
    if value < bounds.lowerBound {
        return 0 as std::ffi::c_int;
    }
    if value > bounds.upperBound {
        return 0 as std::ffi::c_int;
    }
    1 as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_getParameter(
    mut dctx: *mut ZSTD_DCtx,
    mut param: ZSTD_dParameter,
    mut value: *mut std::ffi::c_int,
) -> size_t {
    match param as std::ffi::c_uint {
        100 => {
            *value = ZSTD_highbit32((*dctx).maxWindowSize as u32) as std::ffi::c_int;
            return 0 as std::ffi::c_int as size_t;
        }
        1000 => {
            *value = (*dctx).format as std::ffi::c_int;
            return 0 as std::ffi::c_int as size_t;
        }
        1001 => {
            *value = (*dctx).outBufferMode as std::ffi::c_int;
            return 0 as std::ffi::c_int as size_t;
        }
        1002 => {
            *value = (*dctx).forceIgnoreChecksum as std::ffi::c_int;
            return 0 as std::ffi::c_int as size_t;
        }
        1003 => {
            *value = (*dctx).refMultipleDDicts as std::ffi::c_int;
            return 0 as std::ffi::c_int as size_t;
        }
        1004 => {
            *value = (*dctx).disableHufAsm;
            return 0 as std::ffi::c_int as size_t;
        }
        1005 => {
            *value = (*dctx).maxBlockSizeParam;
            return 0 as std::ffi::c_int as size_t;
        }
        _ => {}
    }
    -(ZSTD_error_parameter_unsupported as std::ffi::c_int) as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_setParameter(
    mut dctx: *mut ZSTD_DCtx,
    mut dParam: ZSTD_dParameter,
    mut value: std::ffi::c_int,
) -> size_t {
    if (*dctx).streamStage as std::ffi::c_uint != zdss_init as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_stage_wrong as std::ffi::c_int) as size_t;
    }
    match dParam as std::ffi::c_uint {
        100 => {
            if value == 0 as std::ffi::c_int {
                value = ZSTD_WINDOWLOG_LIMIT_DEFAULT;
            }
            if ZSTD_dParam_withinBounds(ZSTD_d_windowLogMax, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
            }
            (*dctx).maxWindowSize = (1 as std::ffi::c_int as size_t) << value;
            return 0 as std::ffi::c_int as size_t;
        }
        1000 => {
            if ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam1, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
            }
            (*dctx).format = value as ZSTD_format_e;
            return 0 as std::ffi::c_int as size_t;
        }
        1001 => {
            if ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam2, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
            }
            (*dctx).outBufferMode = value as ZSTD_bufferMode_e;
            return 0 as std::ffi::c_int as size_t;
        }
        1002 => {
            if ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam3, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
            }
            (*dctx).forceIgnoreChecksum = value as ZSTD_forceIgnoreChecksum_e;
            return 0 as std::ffi::c_int as size_t;
        }
        1003 => {
            if ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam4, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
            }
            if (*dctx).staticSize != 0 as std::ffi::c_int as size_t {
                return -(ZSTD_error_parameter_unsupported as std::ffi::c_int) as size_t;
            }
            (*dctx).refMultipleDDicts = value as ZSTD_refMultipleDDicts_e;
            return 0 as std::ffi::c_int as size_t;
        }
        1004 => {
            if ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam5, value) == 0 {
                return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
            }
            (*dctx).disableHufAsm = (value != 0 as std::ffi::c_int) as std::ffi::c_int;
            return 0 as std::ffi::c_int as size_t;
        }
        1005 => {
            if value != 0 as std::ffi::c_int
                && ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam6, value) == 0
            {
                return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
            }
            (*dctx).maxBlockSizeParam = value;
            return 0 as std::ffi::c_int as size_t;
        }
        _ => {}
    }
    -(ZSTD_error_parameter_unsupported as std::ffi::c_int) as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DCtx_reset(
    mut dctx: *mut ZSTD_DCtx,
    mut reset: ZSTD_ResetDirective,
) -> size_t {
    if reset as std::ffi::c_uint == ZSTD_reset_session_only as std::ffi::c_int as std::ffi::c_uint
        || reset as std::ffi::c_uint
            == ZSTD_reset_session_and_parameters as std::ffi::c_int as std::ffi::c_uint
    {
        (*dctx).streamStage = zdss_init;
        (*dctx).noForwardProgress = 0 as std::ffi::c_int;
        (*dctx).isFrameDecompression = 1 as std::ffi::c_int;
    }
    if reset as std::ffi::c_uint == ZSTD_reset_parameters as std::ffi::c_int as std::ffi::c_uint
        || reset as std::ffi::c_uint
            == ZSTD_reset_session_and_parameters as std::ffi::c_int as std::ffi::c_uint
    {
        if (*dctx).streamStage as std::ffi::c_uint
            != zdss_init as std::ffi::c_int as std::ffi::c_uint
        {
            return -(ZSTD_error_stage_wrong as std::ffi::c_int) as size_t;
        }
        ZSTD_clearDict(dctx);
        ZSTD_DCtx_resetParameters(dctx);
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DStream(mut dctx: *const ZSTD_DStream) -> size_t {
    ZSTD_sizeof_DCtx(dctx)
}
unsafe extern "C" fn ZSTD_decodingBufferSize_internal(
    mut windowSize: std::ffi::c_ulonglong,
    mut frameContentSize: std::ffi::c_ulonglong,
    mut blockSizeMax: size_t,
) -> size_t {
    let blockSize = if ((if windowSize
        < ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as std::ffi::c_ulonglong
    {
        windowSize
    } else {
        ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as std::ffi::c_ulonglong
    }) as size_t)
        < blockSizeMax
    {
        (if windowSize < ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as std::ffi::c_ulonglong
        {
            windowSize
        } else {
            ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as std::ffi::c_ulonglong
        }) as size_t
    } else {
        blockSizeMax
    };
    let neededRBSize = windowSize
        .wrapping_add((blockSize * 2 as std::ffi::c_int as size_t) as std::ffi::c_ulonglong)
        .wrapping_add((WILDCOPY_OVERLENGTH * 2 as std::ffi::c_int) as std::ffi::c_ulonglong);
    let neededSize = if frameContentSize < neededRBSize {
        frameContentSize
    } else {
        neededRBSize
    };
    let minRBSize = neededSize as size_t;
    if minRBSize as std::ffi::c_ulonglong != neededSize {
        return -(ZSTD_error_frameParameter_windowTooLarge as std::ffi::c_int) as size_t;
    }
    minRBSize
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decodingBufferSize_min(
    mut windowSize: std::ffi::c_ulonglong,
    mut frameContentSize: std::ffi::c_ulonglong,
) -> size_t {
    ZSTD_decodingBufferSize_internal(windowSize, frameContentSize, ZSTD_BLOCKSIZE_MAX as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize(mut windowSize: size_t) -> size_t {
    let blockSize = if windowSize < ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as size_t {
        windowSize
    } else {
        ((1 as std::ffi::c_int) << 17 as std::ffi::c_int) as size_t
    };
    let inBuffSize = blockSize;
    let outBuffSize = ZSTD_decodingBufferSize_min(
        windowSize as std::ffi::c_ulonglong,
        ZSTD_CONTENTSIZE_UNKNOWN,
    );
    (ZSTD_estimateDCtxSize())
        .wrapping_add(inBuffSize)
        .wrapping_add(outBuffSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize_fromFrame(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let windowSizeMax = (1 as std::ffi::c_uint)
        << (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
            == 4 as std::ffi::c_int as std::ffi::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        });
    let mut zfh = ZSTD_FrameHeader {
        frameContentSize: 0,
        windowSize: 0,
        blockSizeMax: 0,
        frameType: ZSTD_frame,
        headerSize: 0,
        dictID: 0,
        checksumFlag: 0,
        _reserved1: 0,
        _reserved2: 0,
    };
    let err = ZSTD_getFrameHeader(&mut zfh, src, srcSize);
    if ERR_isError(err) != 0 {
        return err;
    }
    if err > 0 as std::ffi::c_int as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if zfh.windowSize > windowSizeMax as std::ffi::c_ulonglong {
        return -(ZSTD_error_frameParameter_windowTooLarge as std::ffi::c_int) as size_t;
    }
    ZSTD_estimateDStreamSize(zfh.windowSize as size_t)
}
unsafe extern "C" fn ZSTD_DCtx_isOverflow(
    mut zds: *mut ZSTD_DStream,
    neededInBuffSize: size_t,
    neededOutBuffSize: size_t,
) -> std::ffi::c_int {
    (((*zds).inBuffSize).wrapping_add((*zds).outBuffSize)
        >= neededInBuffSize.wrapping_add(neededOutBuffSize)
            * ZSTD_WORKSPACETOOLARGE_FACTOR as size_t) as std::ffi::c_int
}
unsafe extern "C" fn ZSTD_DCtx_updateOversizedDuration(
    mut zds: *mut ZSTD_DStream,
    neededInBuffSize: size_t,
    neededOutBuffSize: size_t,
) {
    if ZSTD_DCtx_isOverflow(zds, neededInBuffSize, neededOutBuffSize) != 0 {
        (*zds).oversizedDuration = ((*zds).oversizedDuration).wrapping_add(1);
        (*zds).oversizedDuration;
    } else {
        (*zds).oversizedDuration = 0 as std::ffi::c_int as size_t;
    };
}
unsafe extern "C" fn ZSTD_DCtx_isOversizedTooLong(mut zds: *mut ZSTD_DStream) -> std::ffi::c_int {
    ((*zds).oversizedDuration >= ZSTD_WORKSPACETOOLARGE_MAXDURATION as size_t) as std::ffi::c_int
}
unsafe extern "C" fn ZSTD_checkOutBuffer(
    mut zds: *const ZSTD_DStream,
    mut output: *const ZSTD_outBuffer,
) -> size_t {
    let expect = (*zds).expectedOutBuffer;
    if (*zds).outBufferMode as std::ffi::c_uint
        != ZSTD_bm_stable as std::ffi::c_int as std::ffi::c_uint
    {
        return 0 as std::ffi::c_int as size_t;
    }
    if (*zds).streamStage as std::ffi::c_uint == zdss_init as std::ffi::c_int as std::ffi::c_uint {
        return 0 as std::ffi::c_int as size_t;
    }
    if expect.dst == (*output).dst && expect.pos == (*output).pos && expect.size == (*output).size {
        return 0 as std::ffi::c_int as size_t;
    }
    -(ZSTD_error_dstBuffer_wrong as std::ffi::c_int) as size_t
}
unsafe extern "C" fn ZSTD_decompressContinueStream(
    mut zds: *mut ZSTD_DStream,
    mut op: *mut *mut std::ffi::c_char,
    mut oend: *mut std::ffi::c_char,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let isSkipFrame = ZSTD_isSkipFrame(zds);
    if (*zds).outBufferMode as std::ffi::c_uint
        == ZSTD_bm_buffered as std::ffi::c_int as std::ffi::c_uint
    {
        let dstSize = if isSkipFrame != 0 {
            0 as std::ffi::c_int as size_t
        } else {
            ((*zds).outBuffSize).wrapping_sub((*zds).outStart)
        };
        let decodedSize = ZSTD_decompressContinue(
            zds,
            ((*zds).outBuff).offset((*zds).outStart as isize) as *mut std::ffi::c_void,
            dstSize,
            src,
            srcSize,
        );
        let err_code = decodedSize;
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        if decodedSize == 0 && isSkipFrame == 0 {
            (*zds).streamStage = zdss_read;
        } else {
            (*zds).outEnd = ((*zds).outStart).wrapping_add(decodedSize);
            (*zds).streamStage = zdss_flush;
        }
    } else {
        let dstSize_0 = if isSkipFrame != 0 {
            0 as std::ffi::c_int as size_t
        } else {
            oend.offset_from(*op) as std::ffi::c_long as size_t
        };
        let decodedSize_0 =
            ZSTD_decompressContinue(zds, *op as *mut std::ffi::c_void, dstSize_0, src, srcSize);
        let err_code_0 = decodedSize_0;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        *op = (*op).offset(decodedSize_0 as isize);
        (*zds).streamStage = zdss_read;
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressStream(
    mut zds: *mut ZSTD_DStream,
    mut output: *mut ZSTD_outBuffer,
    mut input: *mut ZSTD_inBuffer,
) -> size_t {
    let src = (*input).src as *const std::ffi::c_char;
    let istart = if (*input).pos != 0 as std::ffi::c_int as size_t {
        src.offset((*input).pos as isize)
    } else {
        src
    };
    let iend = if (*input).size != 0 as std::ffi::c_int as size_t {
        src.offset((*input).size as isize)
    } else {
        src
    };
    let mut ip = istart;
    let dst = (*output).dst as *mut std::ffi::c_char;
    let ostart = if (*output).pos != 0 as std::ffi::c_int as size_t {
        dst.offset((*output).pos as isize)
    } else {
        dst
    };
    let oend = if (*output).size != 0 as std::ffi::c_int as size_t {
        dst.offset((*output).size as isize)
    } else {
        dst
    };
    let mut op = ostart;
    let mut someMoreWork = 1 as std::ffi::c_int as u32;
    if (*input).pos > (*input).size {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if (*output).pos > (*output).size {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    let err_code = ZSTD_checkOutBuffer(zds, output);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    while someMoreWork != 0 {
        let mut current_block_402: u64;
        match (*zds).streamStage as std::ffi::c_uint {
            0 => {
                (*zds).streamStage = zdss_loadHeader;
                (*zds).outEnd = 0 as std::ffi::c_int as size_t;
                (*zds).outStart = (*zds).outEnd;
                (*zds).inPos = (*zds).outStart;
                (*zds).lhSize = (*zds).inPos;
                (*zds).legacyVersion = 0 as std::ffi::c_int as u32;
                (*zds).hostageByte = 0 as std::ffi::c_int as u32;
                (*zds).expectedOutBuffer = *output;
                current_block_402 = 1623252117315916725;
            }
            1 => {
                current_block_402 = 1623252117315916725;
            }
            2 => {
                current_block_402 = 7991679940794782184;
            }
            3 => {
                current_block_402 = 18111739650402451604;
            }
            4 => {
                let toFlushSize = ((*zds).outEnd).wrapping_sub((*zds).outStart);
                let flushedSize = ZSTD_limitCopy(
                    op as *mut std::ffi::c_void,
                    oend.offset_from(op) as std::ffi::c_long as size_t,
                    ((*zds).outBuff).offset((*zds).outStart as isize) as *const std::ffi::c_void,
                    toFlushSize,
                );
                op = if !op.is_null() {
                    op.offset(flushedSize as isize)
                } else {
                    op
                };
                (*zds).outStart = ((*zds).outStart).wrapping_add(flushedSize);
                if flushedSize == toFlushSize {
                    (*zds).streamStage = zdss_read;
                    if ((*zds).outBuffSize as std::ffi::c_ulonglong)
                        < (*zds).fParams.frameContentSize
                        && ((*zds).outStart).wrapping_add((*zds).fParams.blockSizeMax as size_t)
                            > (*zds).outBuffSize
                    {
                        (*zds).outEnd = 0 as std::ffi::c_int as size_t;
                        (*zds).outStart = (*zds).outEnd;
                    }
                } else {
                    someMoreWork = 0 as std::ffi::c_int as u32;
                }
                current_block_402 = 7792909578691485565;
            }
            _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
        }
        if current_block_402 == 1623252117315916725 {
            if (*zds).legacyVersion != 0 {
                if (*zds).staticSize != 0 {
                    return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                }
                let hint = ZSTD_decompressLegacyStream(
                    (*zds).legacyContext,
                    (*zds).legacyVersion,
                    output,
                    input,
                );
                if hint == 0 as std::ffi::c_int as size_t {
                    (*zds).streamStage = zdss_init;
                }
                return hint;
            }
            let hSize = ZSTD_getFrameHeader_advanced(
                &mut (*zds).fParams,
                ((*zds).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                (*zds).lhSize,
                (*zds).format,
            );
            if (*zds).refMultipleDDicts as std::ffi::c_uint != 0 && !((*zds).ddictSet).is_null() {
                ZSTD_DCtx_selectFrameDDict(zds);
            }
            if ERR_isError(hSize) != 0 {
                let legacyVersion = ZSTD_isLegacy(
                    istart as *const std::ffi::c_void,
                    iend.offset_from(istart) as std::ffi::c_long as size_t,
                );
                if legacyVersion != 0 {
                    let ddict = ZSTD_getDDict(zds);
                    let dict = if !ddict.is_null() {
                        ZSTD_DDict_dictContent(ddict)
                    } else {
                        NULL as *const std::ffi::c_void
                    };
                    let dictSize = if !ddict.is_null() {
                        ZSTD_DDict_dictSize(ddict)
                    } else {
                        0 as std::ffi::c_int as size_t
                    };
                    if (*zds).staticSize != 0 {
                        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                    }
                    let err_code_0 = ZSTD_initLegacyStream(
                        &mut (*zds).legacyContext,
                        (*zds).previousLegacyVersion,
                        legacyVersion,
                        dict,
                        dictSize,
                    );
                    if ERR_isError(err_code_0) != 0 {
                        return err_code_0;
                    }
                    (*zds).previousLegacyVersion = legacyVersion;
                    (*zds).legacyVersion = (*zds).previousLegacyVersion;
                    let hint_0 = ZSTD_decompressLegacyStream(
                        (*zds).legacyContext,
                        legacyVersion,
                        output,
                        input,
                    );
                    if hint_0 == 0 as std::ffi::c_int as size_t {
                        (*zds).streamStage = zdss_init;
                    }
                    return hint_0;
                }
                return hSize;
            }
            if hSize != 0 as std::ffi::c_int as size_t {
                let toLoad = hSize.wrapping_sub((*zds).lhSize);
                let remainingInput = iend.offset_from(ip) as std::ffi::c_long as size_t;
                if toLoad > remainingInput {
                    if remainingInput > 0 as std::ffi::c_int as size_t {
                        libc::memcpy(
                            ((*zds).headerBuffer)
                                .as_mut_ptr()
                                .offset((*zds).lhSize as isize)
                                as *mut std::ffi::c_void,
                            ip as *const std::ffi::c_void,
                            remainingInput as libc::size_t,
                        );
                        (*zds).lhSize = ((*zds).lhSize).wrapping_add(remainingInput);
                    }
                    (*input).pos = (*input).size;
                    let err_code_1 = ZSTD_getFrameHeader_advanced(
                        &mut (*zds).fParams,
                        ((*zds).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                        (*zds).lhSize,
                        (*zds).format,
                    );
                    if ERR_isError(err_code_1) != 0 {
                        return err_code_1;
                    }
                    return (if (if (*zds).format as std::ffi::c_uint
                        == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
                    {
                        6 as std::ffi::c_int
                    } else {
                        2 as std::ffi::c_int
                    }) as size_t
                        > hSize
                    {
                        (if (*zds).format as std::ffi::c_uint
                            == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
                        {
                            6 as std::ffi::c_int
                        } else {
                            2 as std::ffi::c_int
                        }) as size_t
                    } else {
                        hSize
                    })
                    .wrapping_sub((*zds).lhSize)
                    .wrapping_add(ZSTD_blockHeaderSize);
                }
                libc::memcpy(
                    ((*zds).headerBuffer)
                        .as_mut_ptr()
                        .offset((*zds).lhSize as isize)
                        as *mut std::ffi::c_void,
                    ip as *const std::ffi::c_void,
                    toLoad as libc::size_t,
                );
                (*zds).lhSize = hSize;
                ip = ip.offset(toLoad as isize);
                current_block_402 = 7792909578691485565;
            } else {
                if (*zds).fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
                    && (*zds).fParams.frameType as std::ffi::c_uint
                        != ZSTD_skippableFrame as std::ffi::c_int as std::ffi::c_uint
                    && oend.offset_from(op) as std::ffi::c_long as size_t as std::ffi::c_ulonglong
                        >= (*zds).fParams.frameContentSize
                {
                    let cSize = ZSTD_findFrameCompressedSize_advanced(
                        istart as *const std::ffi::c_void,
                        iend.offset_from(istart) as std::ffi::c_long as size_t,
                        (*zds).format,
                    );
                    if cSize <= iend.offset_from(istart) as std::ffi::c_long as size_t {
                        let decompressedSize = ZSTD_decompress_usingDDict(
                            zds,
                            op as *mut std::ffi::c_void,
                            oend.offset_from(op) as std::ffi::c_long as size_t,
                            istart as *const std::ffi::c_void,
                            cSize,
                            ZSTD_getDDict(zds),
                        );
                        if ERR_isError(decompressedSize) != 0 {
                            return decompressedSize;
                        }
                        ip = istart.offset(cSize as isize);
                        op = if !op.is_null() {
                            op.offset(decompressedSize as isize)
                        } else {
                            op
                        };
                        (*zds).expected = 0 as std::ffi::c_int as size_t;
                        (*zds).streamStage = zdss_init;
                        someMoreWork = 0 as std::ffi::c_int as u32;
                        current_block_402 = 7792909578691485565;
                    } else {
                        current_block_402 = 8968043056769084000;
                    }
                } else {
                    current_block_402 = 8968043056769084000;
                }
                match current_block_402 {
                    7792909578691485565 => {}
                    _ => {
                        if (*zds).outBufferMode as std::ffi::c_uint
                            == ZSTD_bm_stable as std::ffi::c_int as std::ffi::c_uint
                            && (*zds).fParams.frameType as std::ffi::c_uint
                                != ZSTD_skippableFrame as std::ffi::c_int as std::ffi::c_uint
                            && (*zds).fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
                            && (oend.offset_from(op) as std::ffi::c_long as size_t
                                as std::ffi::c_ulonglong)
                                < (*zds).fParams.frameContentSize
                        {
                            return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
                        }
                        let err_code_2 = ZSTD_decompressBegin_usingDDict(zds, ZSTD_getDDict(zds));
                        if ERR_isError(err_code_2) != 0 {
                            return err_code_2;
                        }
                        if (*zds).format as std::ffi::c_uint
                            == ZSTD_f_zstd1 as std::ffi::c_int as std::ffi::c_uint
                            && MEM_readLE32(
                                ((*zds).headerBuffer).as_mut_ptr() as *const std::ffi::c_void
                            ) & ZSTD_MAGIC_SKIPPABLE_MASK
                                == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
                        {
                            (*zds).expected = MEM_readLE32(
                                ((*zds).headerBuffer)
                                    .as_mut_ptr()
                                    .offset(ZSTD_FRAMEIDSIZE as isize)
                                    as *const std::ffi::c_void,
                            ) as size_t;
                            (*zds).stage = ZSTDds_skipFrame;
                        } else {
                            let err_code_3 = ZSTD_decodeFrameHeader(
                                zds,
                                ((*zds).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                                (*zds).lhSize,
                            );
                            if ERR_isError(err_code_3) != 0 {
                                return err_code_3;
                            }
                            (*zds).expected = ZSTD_blockHeaderSize;
                            (*zds).stage = ZSTDds_decodeBlockHeader;
                        }
                        (*zds).fParams.windowSize = if (*zds).fParams.windowSize
                            > ((1 as std::ffi::c_uint) << 10 as std::ffi::c_int)
                                as std::ffi::c_ulonglong
                        {
                            (*zds).fParams.windowSize
                        } else {
                            ((1 as std::ffi::c_uint) << 10 as std::ffi::c_int)
                                as std::ffi::c_ulonglong
                        };
                        if (*zds).fParams.windowSize > (*zds).maxWindowSize as std::ffi::c_ulonglong
                        {
                            return -(ZSTD_error_frameParameter_windowTooLarge as std::ffi::c_int)
                                as size_t;
                        }
                        if (*zds).maxBlockSizeParam != 0 as std::ffi::c_int {
                            (*zds).fParams.blockSizeMax = if (*zds).fParams.blockSizeMax
                                < (*zds).maxBlockSizeParam as std::ffi::c_uint
                            {
                                (*zds).fParams.blockSizeMax
                            } else {
                                (*zds).maxBlockSizeParam as std::ffi::c_uint
                            };
                        }
                        let neededInBuffSize = (if (*zds).fParams.blockSizeMax
                            > 4 as std::ffi::c_int as std::ffi::c_uint
                        {
                            (*zds).fParams.blockSizeMax
                        } else {
                            4 as std::ffi::c_int as std::ffi::c_uint
                        }) as size_t;
                        let neededOutBuffSize = if (*zds).outBufferMode as std::ffi::c_uint
                            == ZSTD_bm_buffered as std::ffi::c_int as std::ffi::c_uint
                        {
                            ZSTD_decodingBufferSize_internal(
                                (*zds).fParams.windowSize,
                                (*zds).fParams.frameContentSize,
                                (*zds).fParams.blockSizeMax as size_t,
                            )
                        } else {
                            0 as std::ffi::c_int as size_t
                        };
                        ZSTD_DCtx_updateOversizedDuration(zds, neededInBuffSize, neededOutBuffSize);
                        let tooSmall = ((*zds).inBuffSize < neededInBuffSize
                            || (*zds).outBuffSize < neededOutBuffSize)
                            as std::ffi::c_int;
                        let tooLarge = ZSTD_DCtx_isOversizedTooLong(zds);
                        if tooSmall != 0 || tooLarge != 0 {
                            let bufferSize = neededInBuffSize.wrapping_add(neededOutBuffSize);
                            if (*zds).staticSize != 0 {
                                if bufferSize
                                    > ((*zds).staticSize).wrapping_sub(::core::mem::size_of::<
                                        ZSTD_DCtx,
                                    >(
                                    )
                                        as std::ffi::c_ulong)
                                {
                                    return -(ZSTD_error_memory_allocation as std::ffi::c_int)
                                        as size_t;
                                }
                            } else {
                                ZSTD_customFree(
                                    (*zds).inBuff as *mut std::ffi::c_void,
                                    (*zds).customMem,
                                );
                                (*zds).inBuffSize = 0 as std::ffi::c_int as size_t;
                                (*zds).outBuffSize = 0 as std::ffi::c_int as size_t;
                                (*zds).inBuff = ZSTD_customMalloc(bufferSize, (*zds).customMem)
                                    as *mut std::ffi::c_char;
                                if ((*zds).inBuff).is_null() {
                                    return -(ZSTD_error_memory_allocation as std::ffi::c_int)
                                        as size_t;
                                }
                            }
                            (*zds).inBuffSize = neededInBuffSize;
                            (*zds).outBuff = ((*zds).inBuff).offset((*zds).inBuffSize as isize);
                            (*zds).outBuffSize = neededOutBuffSize;
                        }
                        (*zds).streamStage = zdss_read;
                        current_block_402 = 7991679940794782184;
                    }
                }
            }
        }
        if current_block_402 == 7991679940794782184 {
            let neededInSize = ZSTD_nextSrcSizeToDecompressWithInputSize(
                zds,
                iend.offset_from(ip) as std::ffi::c_long as size_t,
            );
            if neededInSize == 0 as std::ffi::c_int as size_t {
                (*zds).streamStage = zdss_init;
                someMoreWork = 0 as std::ffi::c_int as u32;
                current_block_402 = 7792909578691485565;
            } else if iend.offset_from(ip) as std::ffi::c_long as size_t >= neededInSize {
                let err_code_4 = ZSTD_decompressContinueStream(
                    zds,
                    &mut op,
                    oend,
                    ip as *const std::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(err_code_4) != 0 {
                    return err_code_4;
                }
                ip = ip.offset(neededInSize as isize);
                current_block_402 = 7792909578691485565;
            } else if ip == iend {
                someMoreWork = 0 as std::ffi::c_int as u32;
                current_block_402 = 7792909578691485565;
            } else {
                (*zds).streamStage = zdss_load;
                current_block_402 = 18111739650402451604;
            }
        }
        if current_block_402 == 18111739650402451604 {
            let neededInSize_0 = ZSTD_nextSrcSizeToDecompress(zds);
            let toLoad_0 = neededInSize_0.wrapping_sub((*zds).inPos);
            let isSkipFrame = ZSTD_isSkipFrame(zds);
            let mut loadedSize: size_t = 0;
            if isSkipFrame != 0 {
                loadedSize = if toLoad_0 < iend.offset_from(ip) as std::ffi::c_long as size_t {
                    toLoad_0
                } else {
                    iend.offset_from(ip) as std::ffi::c_long as size_t
                };
            } else {
                if toLoad_0 > ((*zds).inBuffSize).wrapping_sub((*zds).inPos) {
                    return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                }
                loadedSize = ZSTD_limitCopy(
                    ((*zds).inBuff).offset((*zds).inPos as isize) as *mut std::ffi::c_void,
                    toLoad_0,
                    ip as *const std::ffi::c_void,
                    iend.offset_from(ip) as std::ffi::c_long as size_t,
                );
            }
            if loadedSize != 0 as std::ffi::c_int as size_t {
                ip = ip.offset(loadedSize as isize);
                (*zds).inPos = ((*zds).inPos).wrapping_add(loadedSize);
            }
            if loadedSize < toLoad_0 {
                someMoreWork = 0 as std::ffi::c_int as u32;
            } else {
                (*zds).inPos = 0 as std::ffi::c_int as size_t;
                let err_code_5 = ZSTD_decompressContinueStream(
                    zds,
                    &mut op,
                    oend,
                    (*zds).inBuff as *const std::ffi::c_void,
                    neededInSize_0,
                );
                if ERR_isError(err_code_5) != 0 {
                    return err_code_5;
                }
            }
        }
    }
    (*input).pos =
        ip.offset_from((*input).src as *const std::ffi::c_char) as std::ffi::c_long as size_t;
    (*output).pos =
        op.offset_from((*output).dst as *mut std::ffi::c_char) as std::ffi::c_long as size_t;
    (*zds).expectedOutBuffer = *output;
    if ip == istart && op == ostart {
        (*zds).noForwardProgress += 1;
        (*zds).noForwardProgress;
        if (*zds).noForwardProgress >= ZSTD_NO_FORWARD_PROGRESS_MAX {
            if op == oend {
                return -(ZSTD_error_noForwardProgress_destFull as std::ffi::c_int) as size_t;
            }
            if ip == iend {
                return -(ZSTD_error_noForwardProgress_inputEmpty as std::ffi::c_int) as size_t;
            }
        }
    } else {
        (*zds).noForwardProgress = 0 as std::ffi::c_int;
    }
    let mut nextSrcSizeHint = ZSTD_nextSrcSizeToDecompress(zds);
    if nextSrcSizeHint == 0 {
        if (*zds).outEnd == (*zds).outStart {
            if (*zds).hostageByte != 0 {
                if (*input).pos >= (*input).size {
                    (*zds).streamStage = zdss_read;
                    return 1 as std::ffi::c_int as size_t;
                }
                (*input).pos = ((*input).pos).wrapping_add(1);
                (*input).pos;
            }
            return 0 as std::ffi::c_int as size_t;
        }
        if (*zds).hostageByte == 0 {
            (*input).pos = ((*input).pos).wrapping_sub(1);
            (*input).pos;
            (*zds).hostageByte = 1 as std::ffi::c_int as u32;
        }
        return 1 as std::ffi::c_int as size_t;
    }
    nextSrcSizeHint = nextSrcSizeHint.wrapping_add(
        ZSTD_blockHeaderSize
            * (ZSTD_nextInputType(zds) as std::ffi::c_uint
                == ZSTDnit_block as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as size_t,
    );
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub((*zds).inPos);
    nextSrcSizeHint
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_decompressStream_simpleArgs(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut dstPos: *mut size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut srcPos: *mut size_t,
) -> size_t {
    let mut output = ZSTD_outBuffer_s {
        dst: std::ptr::null_mut::<std::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    let mut input = ZSTD_inBuffer_s {
        src: std::ptr::null::<std::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    output.dst = dst;
    output.size = dstCapacity;
    output.pos = *dstPos;
    input.src = src;
    input.size = srcSize;
    input.pos = *srcPos;
    let cErr = ZSTD_decompressStream(dctx, &mut output, &mut input);
    *dstPos = output.pos;
    *srcPos = input.pos;
    cErr
}
