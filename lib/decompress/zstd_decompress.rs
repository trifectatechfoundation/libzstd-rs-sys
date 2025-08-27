use core::ptr;

use libc::{calloc, free, malloc, size_t};

use crate::lib::common::entropy_common::FSE_readNCount_slice;
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::MEM_readLE32;
use crate::lib::common::xxhash::{
    ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_slice, ZSTD_XXH64_update,
};
use crate::lib::common::zstd_common::ZSTD_getErrorCode;
use crate::lib::common::zstd_internal::{
    repStartValue, LL_bits, ML_bits, MaxLL, MaxML, MaxOff, ZSTD_blockHeaderSize,
    ZSTD_cpuSupportsBmi2, ZSTD_limitCopy, WILDCOPY_OVERLENGTH, ZSTD_FRAMEIDSIZE,
    ZSTD_WORKSPACETOOLARGE_FACTOR, ZSTD_WORKSPACETOOLARGE_MAXDURATION,
};
use crate::lib::compress::zstd_compress::{ZSTD_CCtx_params_s, ZSTD_CCtx_s};
use crate::lib::decompress::huf_decompress::{
    DTableDesc, HUF_ReadDTableX2_Workspace, HUF_readDTableX2_wksp,
};
use crate::lib::decompress::zstd_ddict::{MultipleDDicts, ZSTD_DDict, ZSTD_DDictHashSet};
use crate::lib::decompress::zstd_decompress_block::{
    getc_block_size, ZSTD_buildFSETable, ZSTD_checkContinuity, ZSTD_decompressBlock_internal,
    ZSTD_getcBlockSize,
};
use crate::lib::decompress::{
    blockProperties_t, BlockType, DecompressStage, LL_base, ML_base, NextInputType, OF_base,
    OF_bits, StreamStage, ZSTD_DCtx, ZSTD_DCtx_s, ZSTD_FrameHeader, ZSTD_d_ignoreChecksum,
    ZSTD_d_validateChecksum, ZSTD_dont_use, ZSTD_entropyDTables_t, ZSTD_forceIgnoreChecksum_e,
    ZSTD_frame, ZSTD_skippableFrame, ZSTD_use_indefinitely, ZSTD_use_once,
};
use crate::lib::zstd::experimental::ZSTD_FRAMEHEADERSIZE_MIN;
use crate::lib::zstd::*;

use crate::lib::common::zstd_trace::{
    ZSTD_Trace, ZSTD_trace_decompress_begin, ZSTD_trace_decompress_end,
};

use crate::lib::legacy::zstd_v05::{
    ZBUFFv05_DCtx_s, ZBUFFv05_createDCtx, ZBUFFv05_decompressContinue,
    ZBUFFv05_decompressInitDictionary, ZBUFFv05_freeDCtx, ZSTDv05_createDCtx,
    ZSTDv05_decompress_usingDict, ZSTDv05_fast, ZSTDv05_findFrameSizeInfoLegacy, ZSTDv05_freeDCtx,
    ZSTDv05_getFrameParams, ZSTDv05_parameters,
};
use crate::lib::legacy::zstd_v06::{
    ZBUFFv06_DCtx_s, ZBUFFv06_createDCtx, ZBUFFv06_decompressContinue,
    ZBUFFv06_decompressInitDictionary, ZBUFFv06_freeDCtx, ZSTDv06_createDCtx,
    ZSTDv06_decompress_usingDict, ZSTDv06_findFrameSizeInfoLegacy, ZSTDv06_frameParams_s,
    ZSTDv06_freeDCtx, ZSTDv06_getFrameParams,
};
use crate::lib::legacy::zstd_v07::{
    ZBUFFv07_DCtx_s, ZBUFFv07_createDCtx, ZBUFFv07_decompressContinue,
    ZBUFFv07_decompressInitDictionary, ZBUFFv07_freeDCtx, ZSTDv07_createDCtx,
    ZSTDv07_decompress_usingDict, ZSTDv07_findFrameSizeInfoLegacy, ZSTDv07_frameParams,
    ZSTDv07_freeDCtx, ZSTDv07_getFrameParams,
};

use crate::lib::decompress::zstd_ddict::{
    ZSTD_DDict_dictContent, ZSTD_DDict_dictSize, ZSTD_copyDDictParameters,
    ZSTD_createDDict_advanced, ZSTD_freeDDict, ZSTD_getDictID_fromDDict, ZSTD_sizeof_DDict,
};

pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[repr(C)]
pub struct ZSTD_cpuid_t {
    pub f1c: u32,
    pub f1d: u32,
    pub f7b: u32,
    pub f7c: u32,
}
type ZBUFFv07_DCtx = ZBUFFv07_DCtx_s;
type ZBUFFv06_DCtx = ZBUFFv06_DCtx_s;
type ZBUFFv05_DCtx = ZBUFFv05_DCtx_s;
type XXH_errorcode = core::ffi::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type streaming_operation = core::ffi::c_uint;
pub const is_streaming: streaming_operation = 1;
pub const not_streaming: streaming_operation = 0;
#[repr(C)]
pub struct ZSTD_frameSizeInfo {
    pub nbBlocks: size_t,
    pub compressedSize: size_t,
    pub decompressedBound: core::ffi::c_ulonglong,
}
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: core::ffi::c_int,
    pub upperBound: core::ffi::c_int,
}
pub type ZSTD_ResetDirective = core::ffi::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
pub type ZSTD_dParameter = core::ffi::c_uint;
pub const ZSTD_d_experimentalParam6: ZSTD_dParameter = 1005;
pub const ZSTD_d_experimentalParam5: ZSTD_dParameter = 1004;
pub const ZSTD_d_experimentalParam4: ZSTD_dParameter = 1003;
pub const ZSTD_d_experimentalParam3: ZSTD_dParameter = 1002;
pub const ZSTD_d_experimentalParam2: ZSTD_dParameter = 1001;
pub const ZSTD_d_experimentalParam1: ZSTD_dParameter = 1000;
pub const ZSTD_d_windowLogMax: ZSTD_dParameter = 100;
pub type ZSTD_DStream = ZSTD_DCtx;
pub type ZSTD_nextInputType_e = core::ffi::c_uint;
pub const ZSTDnit_skippableFrame: ZSTD_nextInputType_e = 5;
pub const ZSTDnit_checksum: ZSTD_nextInputType_e = 4;
pub const ZSTDnit_lastBlock: ZSTD_nextInputType_e = 3;
pub const ZSTDnit_block: ZSTD_nextInputType_e = 2;
pub const ZSTDnit_blockHeader: ZSTD_nextInputType_e = 1;
pub const ZSTDnit_frameHeader: ZSTD_nextInputType_e = 0;
pub type ZSTD_dictContentType_e = core::ffi::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_dictLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub const ZSTD_MAXWINDOWSIZE_DEFAULT: u32 = (1u32 << ZSTD_WINDOWLOG_LIMIT_DEFAULT).wrapping_add(1);
pub const ZSTD_NO_FORWARD_PROGRESS_MAX: core::ffi::c_int = 16;
pub const ZSTD_VERSION_MAJOR: core::ffi::c_int = 1;
pub const ZSTD_VERSION_MINOR: core::ffi::c_int = 5;
pub const ZSTD_VERSION_RELEASE: core::ffi::c_int = 8;
pub const ZSTD_VERSION_NUMBER: core::ffi::c_int =
    ZSTD_VERSION_MAJOR * 100 * 100 + ZSTD_VERSION_MINOR * 100 + ZSTD_VERSION_RELEASE;
pub const ZSTD_MAGICNUMBER: core::ffi::c_uint = 0xfd2fb528;
pub const ZSTD_MAGIC_DICTIONARY: core::ffi::c_uint = 0xec30a437;
pub const ZSTD_MAGIC_SKIPPABLE_START: core::ffi::c_int = 0x184d2a50;
pub const ZSTD_MAGIC_SKIPPABLE_MASK: core::ffi::c_uint = 0xfffffff0;
pub const ZSTD_BLOCKSIZELOG_MAX: core::ffi::c_int = 17;
pub const ZSTD_BLOCKSIZE_MAX: core::ffi::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: core::ffi::c_ulonglong =
    (0 as core::ffi::c_ulonglong).wrapping_sub(1);
pub const ZSTD_CONTENTSIZE_ERROR: core::ffi::c_ulonglong =
    (0 as core::ffi::c_ulonglong).wrapping_sub(2);
pub const ZSTD_SKIPPABLEHEADERSIZE: core::ffi::c_int = 8;
pub const ZSTD_WINDOWLOG_MAX_32: core::ffi::c_int = 30;
pub const ZSTD_WINDOWLOG_MAX_64: core::ffi::c_int = 31;
pub const ZSTD_BLOCKSIZE_MAX_MIN: core::ffi::c_int = (1) << 10;
pub const ZSTD_WINDOWLOG_LIMIT_DEFAULT: core::ffi::c_int = 27;
pub const ZSTD_d_format: core::ffi::c_int = 1000;
pub const ZSTD_d_stableOutBuffer: core::ffi::c_int = 1001;
pub const ZSTD_d_forceIgnoreChecksum: core::ffi::c_int = 1002;
pub const ZSTD_d_refMultipleDDicts: core::ffi::c_int = 1003;
pub const ZSTD_d_disableHuffmanAssembly: core::ffi::c_int = 1004;
pub const ZSTD_d_maxBlockSize: core::ffi::c_int = 1005;
pub const ZSTD_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const ZSTD_WINDOWLOG_ABSOLUTEMIN: core::ffi::c_int = 10;

#[inline]
unsafe fn ZSTD_customMalloc(size: size_t, customMem: ZSTD_customMem) -> *mut core::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
    }
    malloc(size)
}
#[inline]
unsafe fn ZSTD_customCalloc(size: size_t, customMem: ZSTD_customMem) -> *mut core::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        let ptr = (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
        ptr::write_bytes(ptr, 0, size);
        return ptr;
    }
    calloc(1, size)
}
#[inline]
unsafe fn ZSTD_customFree(ptr: *mut core::ffi::c_void, customMem: ZSTD_customMem) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree).unwrap_unchecked()(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}

const ZSTDv01_magicNumberLE: u32 = 0x1EB52FFD;

const ZSTDv02_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB522;
const ZSTDv03_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB523;
const ZSTDv04_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB524;
const ZSTDv05_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB525;
const ZSTDv06_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB526;
const ZSTDv07_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB527;

#[inline]
unsafe fn ZSTD_isLegacy(src: *const core::ffi::c_void, srcSize: size_t) -> u32 {
    is_legacy(unsafe { core::slice::from_raw_parts(src.cast::<u8>(), srcSize) })
}

fn is_legacy(src: &[u8]) -> u32 {
    let Some(chunk) = src.first_chunk() else {
        return 0;
    };

    match u32::from_le_bytes(*chunk) {
        ZSTDv01_magicNumberLE => 1,
        ZSTDv02_MAGICNUMBER => 2,
        ZSTDv03_MAGICNUMBER => 3,
        ZSTDv04_MAGICNUMBER => 4,
        ZSTDv05_MAGICNUMBER => 5,
        ZSTDv06_MAGICNUMBER => 6,
        ZSTDv07_MAGICNUMBER => 7,
        _ => 0,
    }
}

#[inline]
fn get_decompressed_size_legacy(src: &[u8]) -> Option<u64> {
    let ptr = src.as_ptr().cast();

    match is_legacy(src) {
        5 => {
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

            match unsafe { ZSTDv05_getFrameParams(&mut fParams, ptr, src.len() as _) } {
                0 => Some(fParams.srcSize as core::ffi::c_ulonglong),
                _ => None,
            }
        }
        6 => {
            let mut fParams_0 = ZSTDv06_frameParams_s {
                frameContentSize: 0,
                windowLog: 0,
            };

            match unsafe { ZSTDv06_getFrameParams(&mut fParams_0, ptr, src.len() as _) } {
                0 => Some(fParams_0.frameContentSize),
                _ => None,
            }
        }
        7 => {
            let mut fParams_1 = ZSTDv07_frameParams {
                frameContentSize: 0,
                windowSize: 0,
                dictID: 0,
                checksumFlag: 0,
            };

            match unsafe { ZSTDv07_getFrameParams(&mut fParams_1, ptr, src.len() as _) } {
                0 => Some(fParams_1.frameContentSize),
                _ => None,
            }
        }

        _ => None,
    }
}
#[inline]
unsafe fn ZSTD_decompressLegacy(
    mut dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    compressedSize: size_t,
    mut dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let version = ZSTD_isLegacy(src, compressedSize);
    let mut x: core::ffi::c_char = 0;
    if dst.is_null() {
        dst = &mut x as *mut core::ffi::c_char as *mut core::ffi::c_void;
    }
    if src.is_null() {
        src = &mut x as *mut core::ffi::c_char as *const core::ffi::c_void;
    }
    if dict.is_null() {
        dict = &mut x as *mut core::ffi::c_char as *const core::ffi::c_void;
    }
    match version {
        5 => {
            let mut result: size_t = 0;
            let zd = ZSTDv05_createDCtx();
            if zd.is_null() {
                return Error::memory_allocation.to_error_code();
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
                return Error::memory_allocation.to_error_code();
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
                return Error::memory_allocation.to_error_code();
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
        _ => Error::prefix_unknown.to_error_code(),
    }
}

#[inline]
unsafe fn ZSTD_findFrameSizeInfoLegacy(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> ZSTD_frameSizeInfo {
    find_frame_size_info_legacy(if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    })
}

unsafe fn find_frame_size_info_legacy(src: &[u8]) -> ZSTD_frameSizeInfo {
    let mut frameSizeInfo = ZSTD_frameSizeInfo {
        nbBlocks: 0,
        compressedSize: 0,
        decompressedBound: 0,
    };

    match is_legacy(src) {
        5 => {
            ZSTDv05_findFrameSizeInfoLegacy(
                src.as_ptr().cast(),
                src.len(),
                &mut frameSizeInfo.compressedSize,
                &mut frameSizeInfo.decompressedBound,
            );
        }
        6 => {
            ZSTDv06_findFrameSizeInfoLegacy(
                src.as_ptr().cast(),
                src.len(),
                &mut frameSizeInfo.compressedSize,
                &mut frameSizeInfo.decompressedBound,
            );
        }
        7 => {
            ZSTDv07_findFrameSizeInfoLegacy(
                src.as_ptr().cast(),
                src.len(),
                &mut frameSizeInfo.compressedSize,
                &mut frameSizeInfo.decompressedBound,
            );
        }
        _ => {
            frameSizeInfo.compressedSize = Error::prefix_unknown.to_error_code();
            frameSizeInfo.decompressedBound = ZSTD_CONTENTSIZE_ERROR;
        }
    }

    if ERR_isError(frameSizeInfo.compressedSize) == 0 && frameSizeInfo.compressedSize > src.len() {
        frameSizeInfo.compressedSize = Error::srcSize_wrong.to_error_code();
        frameSizeInfo.decompressedBound = ZSTD_CONTENTSIZE_ERROR;
    }

    if frameSizeInfo.decompressedBound != ZSTD_CONTENTSIZE_ERROR {
        frameSizeInfo.nbBlocks = (frameSizeInfo.decompressedBound)
            .wrapping_div(ZSTD_BLOCKSIZE_MAX as core::ffi::c_ulonglong)
            as size_t;
    }

    frameSizeInfo
}

#[inline]
unsafe fn ZSTD_findFrameCompressedSizeLegacy(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let frameSizeInfo = ZSTD_findFrameSizeInfoLegacy(src, srcSize);
    frameSizeInfo.compressedSize
}

#[inline]
unsafe fn ZSTD_freeLegacyStreamContext(
    legacyContext: *mut core::ffi::c_void,
    version: u32,
) -> size_t {
    match version {
        5 => ZBUFFv05_freeDCtx(legacyContext as *mut ZBUFFv05_DCtx),
        6 => ZBUFFv06_freeDCtx(legacyContext as *mut ZBUFFv06_DCtx),
        7 => ZBUFFv07_freeDCtx(legacyContext as *mut ZBUFFv07_DCtx),
        1 | 2 | 3 | _ => Error::version_unsupported.to_error_code(),
    }
}
#[inline]
unsafe fn ZSTD_initLegacyStream(
    legacyContext: *mut *mut core::ffi::c_void,
    prevVersion: u32,
    newVersion: u32,
    mut dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let mut x: core::ffi::c_char = 0;
    if dict.is_null() {
        dict = &mut x as *mut core::ffi::c_char as *const core::ffi::c_void;
    }
    if prevVersion != newVersion {
        ZSTD_freeLegacyStreamContext(*legacyContext, prevVersion);
    }
    match newVersion {
        5 => {
            let dctx = if prevVersion != newVersion {
                ZBUFFv05_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv05_DCtx
            };
            if dctx.is_null() {
                return Error::memory_allocation.to_error_code();
            }
            ZBUFFv05_decompressInitDictionary(dctx, dict, dictSize);
            *legacyContext = dctx as *mut core::ffi::c_void;
            0
        }
        6 => {
            let dctx_0 = if prevVersion != newVersion {
                ZBUFFv06_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv06_DCtx
            };
            if dctx_0.is_null() {
                return Error::memory_allocation.to_error_code();
            }
            ZBUFFv06_decompressInitDictionary(dctx_0, dict, dictSize);
            *legacyContext = dctx_0 as *mut core::ffi::c_void;
            0
        }
        7 => {
            let dctx_1 = if prevVersion != newVersion {
                ZBUFFv07_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv07_DCtx
            };
            if dctx_1.is_null() {
                return Error::memory_allocation.to_error_code();
            }
            ZBUFFv07_decompressInitDictionary(dctx_1, dict, dictSize);
            *legacyContext = dctx_1 as *mut core::ffi::c_void;
            0
        }
        1 | 2 | 3 | _ => 0,
    }
}

#[inline]
unsafe fn ZSTD_decompressLegacyStream(
    legacyContext: *mut core::ffi::c_void,
    version: u32,
    output: &mut ZSTD_outBuffer,
    input: &mut ZSTD_inBuffer,
) -> size_t {
    static mut x: core::ffi::c_char = 0;
    if (output.dst).is_null() {
        output.dst = &raw mut x as *mut core::ffi::c_void;
    }
    if (input.src).is_null() {
        input.src = &raw mut x as *const core::ffi::c_void;
    }
    match version {
        5 => {
            let dctx = legacyContext as *mut ZBUFFv05_DCtx;
            let src =
                (input.src as *const core::ffi::c_char).add(input.pos) as *const core::ffi::c_void;
            let mut readSize = (input.size).wrapping_sub(input.pos);
            let dst =
                (output.dst as *mut core::ffi::c_char).add(output.pos) as *mut core::ffi::c_void;
            let mut decodedSize = (output.size).wrapping_sub(output.pos);
            let hintSize =
                ZBUFFv05_decompressContinue(dctx, dst, &mut decodedSize, src, &mut readSize);
            output.pos = (output.pos).wrapping_add(decodedSize);
            input.pos = (input.pos).wrapping_add(readSize);
            hintSize
        }
        6 => {
            let dctx_0 = legacyContext as *mut ZBUFFv06_DCtx;
            let src_0 =
                (input.src as *const core::ffi::c_char).add(input.pos) as *const core::ffi::c_void;
            let mut readSize_0 = (input.size).wrapping_sub(input.pos);
            let dst_0 =
                (output.dst as *mut core::ffi::c_char).add(output.pos) as *mut core::ffi::c_void;
            let mut decodedSize_0 = (output.size).wrapping_sub(output.pos);
            let hintSize_0 = ZBUFFv06_decompressContinue(
                dctx_0,
                dst_0,
                &mut decodedSize_0,
                src_0,
                &mut readSize_0,
            );
            output.pos = (output.pos).wrapping_add(decodedSize_0);
            input.pos = (input.pos).wrapping_add(readSize_0);
            hintSize_0
        }
        7 => {
            let dctx_1 = legacyContext as *mut ZBUFFv07_DCtx;
            let src_1 =
                (input.src as *const core::ffi::c_char).add(input.pos) as *const core::ffi::c_void;
            let mut readSize_1 = (input.size).wrapping_sub(input.pos);
            let dst_1 =
                (output.dst as *mut core::ffi::c_char).add(output.pos) as *mut core::ffi::c_void;
            let mut decodedSize_1 = (output.size).wrapping_sub(output.pos);
            let hintSize_1 = ZBUFFv07_decompressContinue(
                dctx_1,
                dst_1,
                &mut decodedSize_1,
                src_1,
                &mut readSize_1,
            );
            output.pos = (output.pos).wrapping_add(decodedSize_1);
            input.pos = (input.pos).wrapping_add(readSize_1);
            hintSize_1
        }
        1 | 2 | 3 | _ => Error::version_unsupported.to_error_code(),
    }
}

pub const DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT: core::ffi::c_int = 4;
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_SIZE_MULT: core::ffi::c_int = 3;
pub const DDICT_HASHSET_TABLE_BASE_SIZE: core::ffi::c_int = 64;
pub const DDICT_HASHSET_RESIZE_FACTOR: core::ffi::c_int = 2;

fn ZSTD_DDictHashSet_getIndex(hashSet: &ZSTD_DDictHashSet, dictID: u32) -> size_t {
    let hash = ZSTD_XXH64_slice(&dictID.to_ne_bytes(), 0);
    hash as size_t & (hashSet.ddictPtrTableSize).wrapping_sub(1)
}

unsafe fn ZSTD_DDictHashSet_emplaceDDict(
    hashSet: &mut ZSTD_DDictHashSet,
    ddict: *const ZSTD_DDict,
) -> size_t {
    let dictID = ZSTD_getDictID_fromDDict(ddict);
    let mut idx = ZSTD_DDictHashSet_getIndex(hashSet, dictID);
    let idxRangeMask = (hashSet.ddictPtrTableSize).wrapping_sub(1);
    if hashSet.ddictPtrCount == hashSet.ddictPtrTableSize {
        return Error::GENERIC.to_error_code();
    }
    while !(*(hashSet.ddictPtrTable).add(idx)).is_null() {
        if ZSTD_getDictID_fromDDict(*(hashSet.ddictPtrTable).add(idx)) == dictID {
            let fresh0 = &mut (*(hashSet.ddictPtrTable).add(idx));
            *fresh0 = ddict;
            return 0;
        }
        idx &= idxRangeMask;
        idx = idx.wrapping_add(1);
    }
    let fresh1 = &mut (*(hashSet.ddictPtrTable).add(idx));
    *fresh1 = ddict;
    hashSet.ddictPtrCount = (hashSet.ddictPtrCount).wrapping_add(1);
    hashSet.ddictPtrCount;
    0
}

unsafe fn ZSTD_DDictHashSet_expand(
    hashSet: &mut ZSTD_DDictHashSet,
    customMem: ZSTD_customMem,
) -> size_t {
    let newTableSize = hashSet.ddictPtrTableSize * DDICT_HASHSET_RESIZE_FACTOR as size_t;
    let newTable = ZSTD_customCalloc(
        (::core::mem::size_of::<*mut ZSTD_DDict>()).wrapping_mul(newTableSize),
        customMem,
    ) as *mut *const ZSTD_DDict;
    let oldTable = hashSet.ddictPtrTable;
    let oldTableSize = hashSet.ddictPtrTableSize;
    let mut i: size_t = 0;
    if newTable.is_null() {
        return Error::memory_allocation.to_error_code();
    }
    hashSet.ddictPtrTable = newTable;
    hashSet.ddictPtrTableSize = newTableSize;
    hashSet.ddictPtrCount = 0;
    i = 0;
    while i < oldTableSize {
        if !(*oldTable.add(i)).is_null() {
            let err_code = ZSTD_DDictHashSet_emplaceDDict(hashSet, *oldTable.add(i));
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
        }
        i = i.wrapping_add(1);
    }
    ZSTD_customFree(oldTable as *mut core::ffi::c_void, customMem);
    0
}

unsafe fn ZSTD_DDictHashSet_getDDict(
    hashSet: &mut ZSTD_DDictHashSet,
    dictID: u32,
) -> *const ZSTD_DDict {
    let mut idx = ZSTD_DDictHashSet_getIndex(hashSet, dictID);
    let idxRangeMask = (hashSet.ddictPtrTableSize).wrapping_sub(1);
    loop {
        let currDictID = ZSTD_getDictID_fromDDict(*(hashSet.ddictPtrTable).add(idx)) as size_t;
        if currDictID == dictID as size_t || currDictID == 0 {
            break;
        }
        idx &= idxRangeMask;
        idx = idx.wrapping_add(1);
    }
    *(hashSet.ddictPtrTable).add(idx)
}

unsafe fn ZSTD_createDDictHashSet(customMem: ZSTD_customMem) -> *mut ZSTD_DDictHashSet {
    let ret = ZSTD_customMalloc(::core::mem::size_of::<ZSTD_DDictHashSet>(), customMem)
        as *mut ZSTD_DDictHashSet;
    if ret.is_null() {
        return core::ptr::null_mut();
    }
    (*ret).ddictPtrTable = ZSTD_customCalloc(
        (DDICT_HASHSET_TABLE_BASE_SIZE as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ZSTD_DDict>()),
        customMem,
    ) as *mut *const ZSTD_DDict;
    if ((*ret).ddictPtrTable).is_null() {
        ZSTD_customFree(ret as *mut core::ffi::c_void, customMem);
        return core::ptr::null_mut();
    }
    (*ret).ddictPtrTableSize = DDICT_HASHSET_TABLE_BASE_SIZE as size_t;
    (*ret).ddictPtrCount = 0;
    ret
}

unsafe fn ZSTD_freeDDictHashSet(hashSet: *mut ZSTD_DDictHashSet, customMem: ZSTD_customMem) {
    if !hashSet.is_null() && !((*hashSet).ddictPtrTable).is_null() {
        ZSTD_customFree(
            (*hashSet).ddictPtrTable as *mut core::ffi::c_void,
            customMem,
        );
    }
    if !hashSet.is_null() {
        ZSTD_customFree(hashSet as *mut core::ffi::c_void, customMem);
    }
}
unsafe fn ZSTD_DDictHashSet_addDDict(
    hashSet: &mut ZSTD_DDictHashSet,
    ddict: *const ZSTD_DDict,
    customMem: ZSTD_customMem,
) -> size_t {
    if hashSet.ddictPtrCount * DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT as size_t
        / hashSet.ddictPtrTableSize
        * DDICT_HASHSET_MAX_LOAD_FACTOR_SIZE_MULT as size_t
        != 0
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
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_DCtx))]
pub unsafe extern "C" fn ZSTD_sizeof_DCtx(dctx: *const ZSTD_DCtx) -> size_t {
    if dctx.is_null() {
        return 0;
    }
    (::core::mem::size_of::<ZSTD_DCtx>())
        .wrapping_add(ZSTD_sizeof_DDict((*dctx).ddictLocal))
        .wrapping_add((*dctx).inBuffSize)
        .wrapping_add((*dctx).outBuffSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateDCtxSize))]
pub unsafe extern "C" fn ZSTD_estimateDCtxSize() -> size_t {
    ::core::mem::size_of::<ZSTD_DCtx>()
}

const fn ZSTD_startingInputLength(format: Format) -> size_t {
    match format {
        Format::ZSTD_f_zstd1 => 5,
        Format::ZSTD_f_zstd1_magicless => 1,
    }
}

unsafe fn ZSTD_DCtx_resetParameters(dctx: *mut ZSTD_DCtx) {
    (*dctx).format = Format::ZSTD_f_zstd1;
    (*dctx).maxWindowSize = ZSTD_MAXWINDOWSIZE_DEFAULT as size_t;
    (*dctx).outBufferMode = BufferMode::Buffered;
    (*dctx).forceIgnoreChecksum = ZSTD_d_validateChecksum;
    (*dctx).refMultipleDDicts = MultipleDDicts::Single;
    (*dctx).disableHufAsm = 0;
    (*dctx).maxBlockSizeParam = 0;
}

unsafe fn ZSTD_initDCtx_internal(dctx: *mut ZSTD_DCtx) {
    (*dctx).staticSize = 0;
    (*dctx).ddict = core::ptr::null();
    (*dctx).ddictLocal = core::ptr::null_mut();
    (*dctx).dictEnd = core::ptr::null();
    (*dctx).ddictIsCold = 0;
    (*dctx).dictUses = ZSTD_dont_use;
    (*dctx).inBuff = core::ptr::null_mut();
    (*dctx).inBuffSize = 0;
    (*dctx).outBuffSize = 0;
    (*dctx).streamStage = StreamStage::Init;
    (*dctx).legacyContext = core::ptr::null_mut();
    (*dctx).previousLegacyVersion = 0;
    (*dctx).noForwardProgress = 0;
    (*dctx).oversizedDuration = 0;
    (*dctx).isFrameDecompression = 1;
    (*dctx).bmi2 = ZSTD_cpuSupportsBmi2() as _;
    (*dctx).ddictSet = core::ptr::null_mut();
    ZSTD_DCtx_resetParameters(dctx);
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initStaticDCtx))]
pub unsafe extern "C" fn ZSTD_initStaticDCtx(
    workspace: *mut core::ffi::c_void,
    workspaceSize: size_t,
) -> *mut ZSTD_DCtx {
    let dctx = workspace as *mut ZSTD_DCtx;
    if workspace as size_t & 7 != 0 {
        return core::ptr::null_mut();
    }
    if workspaceSize < ::core::mem::size_of::<ZSTD_DCtx>() {
        return core::ptr::null_mut();
    }
    ZSTD_initDCtx_internal(dctx);
    (*dctx).staticSize = workspaceSize;
    (*dctx).inBuff = dctx.offset(1) as *mut core::ffi::c_char;
    dctx
}

unsafe fn ZSTD_createDCtx_internal(customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    if (customMem.customAlloc).is_none() ^ (customMem.customFree).is_none() {
        return core::ptr::null_mut();
    }

    let dctx = ZSTD_customMalloc(::core::mem::size_of::<ZSTD_DCtx>(), customMem) as *mut ZSTD_DCtx;
    if dctx.is_null() {
        return core::ptr::null_mut();
    }

    (*dctx).customMem = customMem;
    ZSTD_initDCtx_internal(dctx);
    dctx
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDCtx_advanced))]
pub unsafe extern "C" fn ZSTD_createDCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    ZSTD_createDCtx_internal(customMem)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDCtx))]
pub unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx {
    ZSTD_createDCtx_internal(ZSTD_defaultCMem)
}
unsafe fn ZSTD_clearDict(dctx: *mut ZSTD_DCtx) {
    ZSTD_freeDDict((*dctx).ddictLocal);
    (*dctx).ddictLocal = core::ptr::null_mut();
    (*dctx).ddict = core::ptr::null();
    (*dctx).dictUses = ZSTD_dont_use;
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeDCtx))]
pub unsafe extern "C" fn ZSTD_freeDCtx(dctx: *mut ZSTD_DCtx) -> size_t {
    if dctx.is_null() {
        return 0;
    }
    if (*dctx).staticSize != 0 {
        return Error::memory_allocation.to_error_code();
    }
    let cMem = (*dctx).customMem;
    ZSTD_clearDict(dctx);
    ZSTD_customFree((*dctx).inBuff as *mut core::ffi::c_void, cMem);
    (*dctx).inBuff = core::ptr::null_mut();
    if !((*dctx).legacyContext).is_null() {
        ZSTD_freeLegacyStreamContext((*dctx).legacyContext, (*dctx).previousLegacyVersion);
    }
    if !((*dctx).ddictSet).is_null() {
        ZSTD_freeDDictHashSet((*dctx).ddictSet, cMem);
        (*dctx).ddictSet = core::ptr::null_mut();
    }
    ZSTD_customFree(dctx as *mut core::ffi::c_void, cMem);
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_copyDCtx))]
pub unsafe extern "C" fn ZSTD_copyDCtx(dstDCtx: *mut ZSTD_DCtx, srcDCtx: *const ZSTD_DCtx) {
    let toCopy = (&mut (*dstDCtx).inBuff as *mut *mut core::ffi::c_char as *mut core::ffi::c_char)
        .offset_from(dstDCtx as *mut core::ffi::c_char) as core::ffi::c_long
        as size_t;
    libc::memcpy(
        dstDCtx as *mut core::ffi::c_void,
        srcDCtx as *const core::ffi::c_void,
        toCopy as libc::size_t,
    );
}
unsafe fn ZSTD_DCtx_selectFrameDDict(dctx: *mut ZSTD_DCtx) {
    if !((*dctx).ddict).is_null() {
        let frameDDict =
            ZSTD_DDictHashSet_getDDict((*dctx).ddictSet.as_mut().unwrap(), (*dctx).fParams.dictID);
        if !frameDDict.is_null() {
            ZSTD_clearDict(dctx);
            (*dctx).dictID = (*dctx).fParams.dictID;
            (*dctx).ddict = frameDDict;
            (*dctx).dictUses = ZSTD_use_indefinitely;
        }
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_isFrame))]
pub unsafe extern "C" fn ZSTD_isFrame(
    buffer: *const core::ffi::c_void,
    size: size_t,
) -> core::ffi::c_uint {
    let src = if buffer.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(buffer.cast(), size)
    };

    is_frame(src) as core::ffi::c_uint
}

fn is_frame(src: &[u8]) -> bool {
    let [a, b, c, d] = *src else {
        return false;
    };

    let magic = u32::from_le_bytes([a, b, c, d]);
    if magic == ZSTD_MAGICNUMBER {
        return true;
    }

    if magic & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint {
        return true;
    }

    if is_legacy(src) != 0 {
        return true;
    }

    false
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_isSkippableFrame))]
pub unsafe extern "C" fn ZSTD_isSkippableFrame(
    buffer: *const core::ffi::c_void,
    size: size_t,
) -> core::ffi::c_uint {
    let src = if buffer.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(buffer.cast(), size)
    };

    is_skippable_frame(src) as core::ffi::c_uint
}

fn is_skippable_frame(src: &[u8]) -> bool {
    let [a, b, c, d] = *src else {
        return false;
    };

    let magic = u32::from_le_bytes([a, b, c, d]);
    if magic & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint {
        return true;
    }

    false
}

fn frame_header_size_internal(src: &[u8], format: Format) -> usize {
    static ZSTD_fcs_fieldSize: [u8; 4] = [0, 2, 4, 8];
    static ZSTD_did_fieldSize: [u8; 4] = [0, 1, 2, 4];

    let minInputSize = ZSTD_startingInputLength(format);
    let Some([.., fhd]) = src.get(..minInputSize as usize) else {
        return Error::srcSize_wrong.to_error_code();
    };

    let dictID = fhd & 0b11;
    let singleSegment = (fhd >> 5 & 1) != 0;
    let fcsId = fhd >> 6;

    minInputSize
        + usize::from(!singleSegment)
        + usize::from(ZSTD_did_fieldSize[usize::from(dictID)])
        + usize::from(ZSTD_fcs_fieldSize[usize::from(fcsId)])
        + usize::from(singleSegment && fcsId == 0)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_frameHeaderSize))]
pub unsafe extern "C" fn ZSTD_frameHeaderSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    };

    frame_header_size_internal(src, Format::ZSTD_f_zstd1)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getFrameHeader))]
pub unsafe extern "C" fn ZSTD_getFrameHeader(
    zfhPtr: *mut ZSTD_FrameHeader,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_getFrameHeader_advanced(zfhPtr, src, srcSize, Format::ZSTD_f_zstd1 as _)
}

fn get_frame_header(zfhPtr: &mut ZSTD_FrameHeader, src: &[u8]) -> size_t {
    get_frame_header_advanced(zfhPtr, src, Format::ZSTD_f_zstd1)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getFrameHeader_advanced))]
pub unsafe extern "C" fn ZSTD_getFrameHeader_advanced(
    zfhPtr: *mut ZSTD_FrameHeader,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    format: ZSTD_format_e,
) -> size_t {
    // Apparently some sanitizers require this?
    unsafe { zfhPtr.write(ZSTD_FrameHeader::default()) };

    let Some(zfhPtr) = zfhPtr.as_mut() else {
        return Error::GENERIC.to_error_code();
    };

    // Compatibility: this is stricter than zstd.
    let Ok(format) = Format::try_from(format) else {
        return Error::GENERIC.to_error_code();
    };

    if srcSize > 0 && src.is_null() {
        return Error::GENERIC.to_error_code();
    }

    get_frame_header_advanced(
        zfhPtr,
        if src.is_null() {
            &[]
        } else {
            core::slice::from_raw_parts(src as *const u8, srcSize)
        },
        format,
    )
}

fn get_frame_header_advanced(zfhPtr: &mut ZSTD_FrameHeader, src: &[u8], format: Format) -> size_t {
    let minInputSize = ZSTD_startingInputLength(format);
    if src.len() < minInputSize as usize {
        if !src.is_empty()
            && format != Format::ZSTD_f_zstd1_magicless
            && src != &ZSTD_MAGICNUMBER.to_le_bytes()[..src.len()]
        {
            let mut hbuf = ZSTD_MAGIC_SKIPPABLE_START.to_le_bytes();
            hbuf[..src.len()].copy_from_slice(src);
            if u32::from_le_bytes(hbuf) & ZSTD_MAGIC_SKIPPABLE_MASK
                != ZSTD_MAGIC_SKIPPABLE_START as u32
            {
                return Error::prefix_unknown.to_error_code();
            }
        }
        return minInputSize;
    }

    if format != Format::ZSTD_f_zstd1_magicless
        && u32::from_le_bytes(*src.first_chunk().unwrap()) != ZSTD_MAGICNUMBER
    {
        let first_word = u32::from_le_bytes(*src.first_chunk().unwrap());
        if first_word & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
        {
            if src.len() < ZSTD_SKIPPABLEHEADERSIZE as usize {
                return ZSTD_SKIPPABLEHEADERSIZE as size_t;
            }

            let dictID = first_word.wrapping_sub(ZSTD_MAGIC_SKIPPABLE_START as u32);
            let frameContentSize =
                u32::from_le_bytes(*src[ZSTD_FRAMEIDSIZE..].first_chunk().unwrap());

            *zfhPtr = ZSTD_FrameHeader {
                frameContentSize: u64::from(frameContentSize),
                windowSize: 0,
                blockSizeMax: 0,
                frameType: ZSTD_skippableFrame,
                headerSize: ZSTD_SKIPPABLEHEADERSIZE as core::ffi::c_uint,
                dictID,
                checksumFlag: 0,
                _reserved1: 0,
                _reserved2: 0,
            };

            return 0;
        }
        return Error::prefix_unknown.to_error_code();
    }

    let fhsize = frame_header_size_internal(src, format);
    if src.len() < fhsize {
        return fhsize;
    }

    let fhdByte = src[minInputSize as usize - 1];
    let dictIDSizeCode = fhdByte & 0b11;
    let checksumFlag = u32::from(fhdByte) >> 2 & 1;
    let singleSegment = (u32::from(fhdByte) >> 5 & 1) != 0;
    let fcsID = (u32::from(fhdByte) >> 6) as u32;

    let mut windowSize = 0;

    if fhdByte & 0x8 != 0 {
        return Error::frameParameter_unsupported.to_error_code();
    }

    let mut pos = minInputSize as usize;
    if !singleSegment {
        let wlByte = src[pos];
        pos += 1;
        let windowLog = ((i32::from(wlByte) / 8) + ZSTD_WINDOWLOG_ABSOLUTEMIN) as u32;

        if windowLog > (if size_of::<usize>() == 4 { 30 } else { 31 }) as u32 {
            return Error::frameParameter_windowTooLarge.to_error_code();
        }

        windowSize = 1u64 << windowLog;
        windowSize = windowSize.wrapping_add((windowSize / 8) * (wlByte & 7) as u64);
    }

    let dictID;
    match dictIDSizeCode {
        1 => {
            dictID = u32::from(src[pos]);
            pos += 1;
        }
        2 => {
            dictID = u32::from(u16::from_le_bytes(src[pos..][..2].try_into().unwrap()));
            pos += 2;
        }
        3 => {
            dictID = u32::from_le_bytes(src[pos..][..4].try_into().unwrap());
            pos += 4;
        }
        _ => {
            dictID = 0;
        }
    }

    let frameContentSize = match fcsID {
        1 => u64::from(u16::from_le_bytes(src[pos..][..2].try_into().unwrap())) + 256,
        2 => u64::from(u32::from_le_bytes(src[pos..][..4].try_into().unwrap())),
        3 => u64::from_le_bytes(src[pos..][..8].try_into().unwrap()),
        _ if singleSegment => u64::from(src[pos]),
        _ => ZSTD_CONTENTSIZE_UNKNOWN,
    };

    if singleSegment {
        windowSize = frameContentSize;
    }

    *zfhPtr = ZSTD_FrameHeader {
        frameContentSize: frameContentSize as core::ffi::c_ulonglong,
        windowSize: windowSize as core::ffi::c_ulonglong,
        blockSizeMax: Ord::min(windowSize, 1 << 17) as u32,
        frameType: ZSTD_frame,
        headerSize: fhsize as u32,
        dictID,
        checksumFlag,
        _reserved1: 0,
        _reserved2: 0,
    };

    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getFrameContentSize))]
pub unsafe extern "C" fn ZSTD_getFrameContentSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_ulonglong {
    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    };

    get_frame_content_size(src)
}

fn get_frame_content_size(src: &[u8]) -> u64 {
    if is_legacy(src) != 0 {
        return match get_decompressed_size_legacy(src) {
            None | Some(0) => ZSTD_CONTENTSIZE_UNKNOWN,
            Some(decompressed_size) => decompressed_size,
        };
    }

    let mut zfh = ZSTD_FrameHeader::default();
    if get_frame_header_advanced(&mut zfh, src, Format::ZSTD_f_zstd1) != 0 {
        return ZSTD_CONTENTSIZE_ERROR;
    }

    if zfh.frameType == ZSTD_skippableFrame {
        0
    } else {
        zfh.frameContentSize
    }
}

unsafe fn readSkippableFrameSize(src: *const core::ffi::c_void, srcSize: size_t) -> size_t {
    read_skippable_frame_size(if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    })
}

fn read_skippable_frame_size(src: &[u8]) -> size_t {
    let skippableHeaderSize = ZSTD_SKIPPABLEHEADERSIZE as usize;

    let [_, _, _, _, a, b, c, d, ..] = *src else {
        return Error::srcSize_wrong.to_error_code();
    };

    let size = u32::from_le_bytes([a, b, c, d]);

    if size.wrapping_add(8) < size {
        return Error::frameParameter_unsupported.to_error_code();
    }

    let skippableSize = skippableHeaderSize.wrapping_add(size as usize);
    if skippableSize > src.len() {
        return Error::srcSize_wrong.to_error_code();
    }

    skippableSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_readSkippableFrame))]
pub unsafe extern "C" fn ZSTD_readSkippableFrame(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    magicVariant: *mut core::ffi::c_uint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize < 8 {
        return Error::srcSize_wrong.to_error_code();
    }
    let magicNumber = MEM_readLE32(src);
    let skippableFrameSize = readSkippableFrameSize(src, srcSize);
    let skippableContentSize = skippableFrameSize.wrapping_sub(ZSTD_SKIPPABLEHEADERSIZE as size_t);
    if ZSTD_isSkippableFrame(src, srcSize) == 0 {
        return Error::frameParameter_unsupported.to_error_code();
    }
    if skippableFrameSize < 8 || skippableFrameSize > srcSize {
        return Error::srcSize_wrong.to_error_code();
    }
    if skippableContentSize > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if skippableContentSize > 0 && !dst.is_null() {
        libc::memcpy(
            dst,
            (src as *const u8).offset(8) as *const core::ffi::c_void,
            skippableContentSize as libc::size_t,
        );
    }
    if !magicVariant.is_null() {
        *magicVariant = magicNumber.wrapping_sub(ZSTD_MAGIC_SKIPPABLE_START as u32);
    }
    skippableContentSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_findDecompressedSize))]
pub unsafe extern "C" fn ZSTD_findDecompressedSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_ulonglong {
    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    };

    find_decompressed_size(src)
}

fn find_decompressed_size(mut src: &[u8]) -> u64 {
    let mut totalDstSize = 0u64;

    while let [a, b, c, d, _, ..] = *src {
        let magicNumber = u32::from_le_bytes([a, b, c, d]);
        if magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK
            == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint
        {
            let skippableSize = read_skippable_frame_size(src);
            if ERR_isError(skippableSize) != 0 {
                return ZSTD_CONTENTSIZE_ERROR;
            }
            src = &src[skippableSize..];
        } else {
            let fcs = get_frame_content_size(src);
            if fcs >= ZSTD_CONTENTSIZE_ERROR {
                return fcs;
            }
            if totalDstSize.wrapping_add(fcs) < totalDstSize {
                return ZSTD_CONTENTSIZE_ERROR;
            }
            totalDstSize = totalDstSize.wrapping_add(fcs);
            let frameSrcSize = ZSTD_findFrameCompressedSize_advanced(src, Format::ZSTD_f_zstd1);
            if ERR_isError(frameSrcSize) != 0 {
                return ZSTD_CONTENTSIZE_ERROR;
            }
            src = &src[frameSrcSize..];
        }
    }

    if !src.is_empty() {
        return ZSTD_CONTENTSIZE_ERROR;
    }

    totalDstSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getDecompressedSize))]
pub unsafe extern "C" fn ZSTD_getDecompressedSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_ulonglong {
    let ret = ZSTD_getFrameContentSize(src, srcSize);
    if ret >= ZSTD_CONTENTSIZE_ERROR {
        0
    } else {
        ret
    }
}

unsafe fn ZSTD_decodeFrameHeader(dctx: *mut ZSTD_DCtx, src: &[u8]) -> size_t {
    let result = get_frame_header_advanced(&mut (*dctx).fParams, src, (*dctx).format);
    if ERR_isError(result) != 0 {
        return result;
    }
    if result > 0 {
        return Error::srcSize_wrong.to_error_code();
    }
    if (*dctx).refMultipleDDicts == MultipleDDicts::Multiple && !((*dctx).ddictSet).is_null() {
        ZSTD_DCtx_selectFrameDDict(dctx);
    }
    if (*dctx).fParams.dictID != 0 && (*dctx).dictID != (*dctx).fParams.dictID {
        return Error::dictionary_wrong.to_error_code();
    }
    (*dctx).validateChecksum =
        ((*dctx).fParams.checksumFlag != 0 && (*dctx).forceIgnoreChecksum as u64 == 0) as u32;
    if (*dctx).validateChecksum != 0 {
        ZSTD_XXH64_reset(&mut (*dctx).xxhState, 0);
    }
    (*dctx).processedCSize = ((*dctx).processedCSize as size_t).wrapping_add(src.len()) as u64;
    0
}

fn ZSTD_errorFrameSizeInfo(ret: size_t) -> ZSTD_frameSizeInfo {
    ZSTD_frameSizeInfo {
        nbBlocks: 0,
        compressedSize: ret,
        decompressedBound: ZSTD_CONTENTSIZE_ERROR,
    }
}

fn find_frame_size_info(src: &[u8], format: Format) -> ZSTD_frameSizeInfo {
    let mut frameSizeInfo = ZSTD_frameSizeInfo {
        nbBlocks: 0,
        compressedSize: 0,
        decompressedBound: 0,
    };

    if format == Format::ZSTD_f_zstd1 && is_legacy(src) != 0 {
        return unsafe { find_frame_size_info_legacy(src) };
    }

    if format == Format::ZSTD_f_zstd1
        && src.len() >= ZSTD_SKIPPABLEHEADERSIZE as usize
        && u32::from_le_bytes(*src.first_chunk().unwrap()) & ZSTD_MAGIC_SKIPPABLE_MASK
            == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint
    {
        frameSizeInfo.compressedSize = read_skippable_frame_size(src);
        frameSizeInfo
    } else {
        let mut ip = 0;
        let mut remainingSize = src.len();
        let mut nbBlocks = 0usize;
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
        let ret = get_frame_header_advanced(&mut zfh, src, format);
        if ERR_isError(ret) != 0 {
            return ZSTD_errorFrameSizeInfo(ret);
        }
        if ret > 0 {
            return ZSTD_errorFrameSizeInfo(Error::srcSize_wrong.to_error_code());
        }
        ip += zfh.headerSize as usize;
        remainingSize = remainingSize.wrapping_sub(zfh.headerSize as size_t);
        loop {
            let mut blockProperties = blockProperties_t {
                blockType: BlockType::Raw,
                lastBlock: 0,
                origSize: 0,
            };
            let cBlockSize = unsafe {
                ZSTD_getcBlockSize(
                    src[ip..].as_ptr().cast(),
                    remainingSize,
                    &mut blockProperties,
                )
            };
            if ERR_isError(cBlockSize) != 0 {
                return ZSTD_errorFrameSizeInfo(cBlockSize);
            }
            if ZSTD_blockHeaderSize.wrapping_add(cBlockSize) > remainingSize {
                return ZSTD_errorFrameSizeInfo(Error::srcSize_wrong.to_error_code());
            }
            ip += ZSTD_blockHeaderSize.wrapping_add(cBlockSize) as usize;
            remainingSize =
                remainingSize.wrapping_sub(ZSTD_blockHeaderSize.wrapping_add(cBlockSize));
            nbBlocks = nbBlocks.wrapping_add(1);
            if blockProperties.lastBlock != 0 {
                break;
            }
        }
        if zfh.checksumFlag != 0 {
            if remainingSize < 4 {
                return ZSTD_errorFrameSizeInfo(Error::srcSize_wrong.to_error_code());
            }
            ip += 4;
        }
        frameSizeInfo.nbBlocks = nbBlocks;
        frameSizeInfo.compressedSize = ip as size_t;
        frameSizeInfo.decompressedBound = if zfh.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN {
            zfh.frameContentSize
        } else {
            (nbBlocks as core::ffi::c_ulonglong)
                .wrapping_mul(zfh.blockSizeMax as core::ffi::c_ulonglong)
        };
        frameSizeInfo
    }
}

fn ZSTD_findFrameCompressedSize_advanced(src: &[u8], format: Format) -> size_t {
    find_frame_size_info(src, format).compressedSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_findFrameCompressedSize))]
pub unsafe extern "C" fn ZSTD_findFrameCompressedSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    };

    ZSTD_findFrameCompressedSize_advanced(src, Format::ZSTD_f_zstd1)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBound))]
pub unsafe extern "C" fn ZSTD_decompressBound(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_ulonglong {
    decompress_bound(if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    })
}

fn decompress_bound(mut src: &[u8]) -> core::ffi::c_ulonglong {
    let mut bound = 0;

    while !src.is_empty() {
        let frameSizeInfo = find_frame_size_info(src, Format::ZSTD_f_zstd1);
        let compressedSize = frameSizeInfo.compressedSize;
        let decompressedBound = frameSizeInfo.decompressedBound;
        if ERR_isError(compressedSize) != 0 || decompressedBound == ZSTD_CONTENTSIZE_ERROR {
            return ZSTD_CONTENTSIZE_ERROR;
        }
        src = &src[compressedSize as usize..];
        bound += decompressedBound;
    }

    bound
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressionMargin))]
pub unsafe extern "C" fn ZSTD_decompressionMargin(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    decompression_margin(if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast(), srcSize)
    })
}

fn decompression_margin(mut src: &[u8]) -> size_t {
    let mut margin = 0;
    let mut maxBlockSize = 0;

    /* Iterate over each frame */
    while !src.is_empty() {
        let frameSizeInfo = find_frame_size_info(src, Format::ZSTD_f_zstd1);
        let compressedSize = frameSizeInfo.compressedSize;
        let decompressedBound = frameSizeInfo.decompressedBound;

        let mut zfh = ZSTD_FrameHeader::default();
        let err_code = get_frame_header(&mut zfh, src);
        if ERR_isError(err_code) != 0 {
            return err_code;
        }

        if ERR_isError(compressedSize) != 0 || decompressedBound == ZSTD_CONTENTSIZE_ERROR {
            return Error::corruption_detected.to_error_code();
        }

        if zfh.frameType as core::ffi::c_uint == ZSTD_frame as core::ffi::c_uint {
            /* Add the frame header to our margin */
            margin += zfh.headerSize as size_t;
            margin += if zfh.checksumFlag != 0 { 4 } else { 0 };
            margin += 3 * frameSizeInfo.nbBlocks;
            maxBlockSize = Ord::max(maxBlockSize, zfh.blockSizeMax)
        } else {
            assert!(zfh.frameType == ZSTD_skippableFrame);
            /* Add the entire skippable frame size to our margin. */
            margin += compressedSize;
        }

        src = &src[compressedSize as usize..];
    }

    /* Add the max block size back to the margin. */
    margin += maxBlockSize as size_t;

    margin
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_insertBlock))]
pub unsafe extern "C" fn ZSTD_insertBlock(
    dctx: *mut ZSTD_DCtx,
    blockStart: *const core::ffi::c_void,
    blockSize: size_t,
) -> size_t {
    ZSTD_checkContinuity(dctx, blockStart, blockSize);
    (*dctx).previousDstEnd =
        (blockStart as *const core::ffi::c_char).add(blockSize) as *const core::ffi::c_void;
    blockSize
}
unsafe fn ZSTD_copyRawBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if dst.is_null() {
        if srcSize == 0 {
            return 0;
        }
        return Error::dstBuffer_null.to_error_code();
    }
    core::ptr::copy(src.cast::<u8>(), dst.cast::<u8>(), srcSize);
    srcSize
}
unsafe fn ZSTD_setRleBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    b: u8,
    regenSize: size_t,
) -> size_t {
    if regenSize > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if dst.is_null() {
        if regenSize == 0 {
            return 0;
        }
        return Error::dstBuffer_null.to_error_code();
    }
    ptr::write_bytes(dst, b, regenSize);
    regenSize
}
unsafe fn ZSTD_DCtx_trace_end(
    dctx: *const ZSTD_DCtx,
    uncompressedSize: u64,
    compressedSize: u64,
    streaming: core::ffi::c_int,
) {
    if (*dctx).traceCtx != 0 {
        let mut trace = ZSTD_Trace {
            version: 0,
            streaming: 0,
            dictionaryID: 0,
            dictionaryIsCold: 0,
            dictionarySize: 0,
            uncompressedSize: 0,
            compressedSize: 0,
            params: core::ptr::null::<ZSTD_CCtx_params_s>(),
            cctx: core::ptr::null::<ZSTD_CCtx_s>(),
            dctx: core::ptr::null::<ZSTD_DCtx_s>(),
        };
        ptr::write_bytes(
            &mut trace as *mut ZSTD_Trace as *mut u8,
            0,
            ::core::mem::size_of::<ZSTD_Trace>(),
        );
        trace.version = ZSTD_VERSION_NUMBER as core::ffi::c_uint;
        trace.streaming = streaming;
        if !((*dctx).ddict).is_null() {
            trace.dictionaryID = ZSTD_getDictID_fromDDict((*dctx).ddict);
            trace.dictionarySize = ZSTD_DDict_dictSize((*dctx).ddict);
            trace.dictionaryIsCold = (*dctx).ddictIsCold;
        }
        trace.uncompressedSize = uncompressedSize as size_t;
        trace.compressedSize = compressedSize as size_t;
        trace.dctx = dctx;
        ZSTD_trace_decompress_end((*dctx).traceCtx, &mut trace);
    }
}
unsafe fn ZSTD_decompressFrame(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    srcPtr: &mut &[u8],
) -> size_t {
    let ilen = srcPtr.len();
    let ip = srcPtr;
    let ostart = dst as *mut u8;
    let oend = if dstCapacity != 0 {
        ostart.add(dstCapacity)
    } else {
        ostart
    };
    let mut op = ostart;
    if ip.len()
        < (match (*dctx).format {
            Format::ZSTD_f_zstd1 => 6usize,
            Format::ZSTD_f_zstd1_magicless => 2,
        })
        .wrapping_add(ZSTD_blockHeaderSize)
    {
        return Error::srcSize_wrong.to_error_code();
    }
    let frameHeaderSize = frame_header_size_internal(ip, (*dctx).format);
    if ERR_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    if ip.len() < frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        return Error::srcSize_wrong.to_error_code();
    }
    let err_code = ZSTD_decodeFrameHeader(dctx, &ip[..frameHeaderSize]);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    *ip = &ip[frameHeaderSize..];
    if (*dctx).maxBlockSizeParam != 0 {
        (*dctx).fParams.blockSizeMax =
            if (*dctx).fParams.blockSizeMax < (*dctx).maxBlockSizeParam as core::ffi::c_uint {
                (*dctx).fParams.blockSizeMax
            } else {
                (*dctx).maxBlockSizeParam as core::ffi::c_uint
            };
    }
    loop {
        let mut oBlockEnd = oend;
        let mut decodedSize: size_t = 0;

        let (blockProperties, cBlockSize) = match getc_block_size(ip) {
            Ok(ret) => ret,
            Err(e) => return e.to_error_code(),
        };

        *ip = &ip[ZSTD_blockHeaderSize..];
        if cBlockSize > ip.len() {
            return Error::srcSize_wrong.to_error_code();
        }
        if ip.as_ptr() >= op as *const u8 && ip.as_ptr() < oBlockEnd as *const u8 {
            oBlockEnd = op.offset(ip.as_ptr().offset_from(op) as core::ffi::c_long as isize);
        }
        match blockProperties.blockType {
            BlockType::Raw => {
                decodedSize = ZSTD_copyRawBlock(
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as size_t,
                    ip.as_ptr().cast(),
                    cBlockSize,
                );
            }
            BlockType::Rle => {
                decodedSize = ZSTD_setRleBlock(
                    op as *mut core::ffi::c_void,
                    oBlockEnd.offset_from(op) as size_t,
                    ip[0],
                    blockProperties.origSize as size_t,
                );
            }
            BlockType::Compressed => {
                decodedSize = ZSTD_decompressBlock_internal(
                    dctx,
                    op as *mut core::ffi::c_void,
                    oBlockEnd.offset_from(op) as size_t,
                    ip.as_ptr().cast(),
                    cBlockSize,
                    not_streaming,
                );
            }
            BlockType::Reserved => {
                return Error::corruption_detected.to_error_code();
            }
        }
        let err_code_0 = decodedSize;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        if (*dctx).validateChecksum != 0 {
            ZSTD_XXH64_update(
                &mut (*dctx).xxhState,
                op as *const core::ffi::c_void,
                decodedSize,
            );
        }
        if decodedSize != 0 {
            op = op.add(decodedSize);
        }
        *ip = &ip[cBlockSize..];
        if blockProperties.lastBlock != 0 {
            break;
        }
    }
    if (*dctx).fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
        && op.offset_from(ostart) as core::ffi::c_long as u64 as core::ffi::c_ulonglong
            != (*dctx).fParams.frameContentSize
    {
        return Error::corruption_detected.to_error_code();
    }
    if (*dctx).fParams.checksumFlag != 0 {
        let [a, b, c, d, ..] = **ip else {
            return Error::checksum_wrong.to_error_code();
        };

        if (*dctx).forceIgnoreChecksum == 0 {
            if u32::from_le_bytes([a, b, c, d]) != ZSTD_XXH64_digest(&mut (*dctx).xxhState) as u32 {
                return Error::checksum_wrong.to_error_code();
            }
        }

        *ip = &ip[4..];
    }

    ZSTD_DCtx_trace_end(
        dctx,
        op.offset_from(ostart) as core::ffi::c_long as u64,
        (ilen - ip.len()) as u64,
        0,
    );

    op.offset_from(ostart) as size_t
}

unsafe fn ZSTD_decompressMultiFrame(
    dctx: *mut ZSTD_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: &[u8],
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
    ddict: Option<&ZSTD_DDict>,
) -> size_t {
    let dststart = dst;
    let mut more_than_one_frame = false;

    if let Some(ddict) = ddict {
        dict = ZSTD_DDict_dictContent(ddict);
        dictSize = ZSTD_DDict_dictSize(ddict);
    }

    while src.len() >= ZSTD_startingInputLength((*dctx).format) {
        if (*dctx).format == Format::ZSTD_f_zstd1 && is_legacy(src) != 0 {
            let frameSize;
            {
                let srcSize = src.len();
                let src = src.as_ptr().cast();

                let mut decodedSize: size_t = 0;
                frameSize = ZSTD_findFrameCompressedSizeLegacy(src, srcSize);
                if ERR_isError(frameSize) != 0 {
                    return frameSize;
                }
                if (*dctx).staticSize != 0 {
                    return Error::memory_allocation.to_error_code();
                }
                decodedSize =
                    ZSTD_decompressLegacy(dst, dstCapacity, src, frameSize, dict, dictSize);
                if ERR_isError(decodedSize) != 0 {
                    return decodedSize;
                }
                let expectedSize = ZSTD_getFrameContentSize(src, srcSize);
                if expectedSize == ZSTD_CONTENTSIZE_ERROR {
                    return Error::corruption_detected.to_error_code();
                }
                if expectedSize != ZSTD_CONTENTSIZE_UNKNOWN
                    && expectedSize != decodedSize as core::ffi::c_ulonglong
                {
                    return Error::corruption_detected.to_error_code();
                }
                dst = (dst as *mut u8).add(decodedSize) as *mut core::ffi::c_void;
                dstCapacity = dstCapacity.wrapping_sub(decodedSize);
            }
            src = &src[frameSize..];
        } else {
            if (*dctx).format == Format::ZSTD_f_zstd1 && src.len() >= 4 {
                let magicNumber = u32::from_le_bytes(*src.first_chunk().unwrap());
                if magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK
                    == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint
                {
                    let skippableSize = read_skippable_frame_size(src);
                    let err_code = skippableSize;
                    if ERR_isError(err_code) != 0 {
                        return err_code;
                    }
                    src = &src[skippableSize..];
                    continue;
                }
            }
            if let Some(ddict) = ddict {
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
            let res = ZSTD_decompressFrame(dctx, dst, dstCapacity, &mut src);
            if ZSTD_getErrorCode(res) == ZSTD_error_prefix_unknown && more_than_one_frame {
                return Error::srcSize_wrong.to_error_code();
            }
            if ERR_isError(res) != 0 {
                return res;
            }
            if res != 0 {
                dst = (dst as *mut u8).add(res) as *mut core::ffi::c_void;
            }
            dstCapacity = dstCapacity.wrapping_sub(res);
            more_than_one_frame = true;
        }
    }

    if !src.is_empty() {
        return Error::srcSize_wrong.to_error_code();
    }

    (dst as *mut u8).offset_from(dststart as *mut u8) as size_t
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompress_usingDict))]
pub unsafe extern "C" fn ZSTD_decompress_usingDict(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast::<u8>(), srcSize)
    };
    ZSTD_decompressMultiFrame(dctx, dst, dstCapacity, src, dict, dictSize, None)
}

unsafe fn ZSTD_getDDict(dctx: *mut ZSTD_DCtx) -> *const ZSTD_DDict {
    match (*dctx).dictUses as core::ffi::c_int {
        -1 => (*dctx).ddict,
        1 => {
            (*dctx).dictUses = ZSTD_dont_use;
            (*dctx).ddict
        }
        0 | _ => {
            ZSTD_clearDict(dctx);
            core::ptr::null()
        }
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressDCtx))]
pub unsafe extern "C" fn ZSTD_decompressDCtx(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_decompress_usingDDict(dctx, dst, dstCapacity, src, srcSize, ZSTD_getDDict(dctx))
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompress))]
pub unsafe extern "C" fn ZSTD_decompress(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut regenSize: size_t = 0;
    let dctx = ZSTD_createDCtx_internal(ZSTD_defaultCMem);
    if dctx.is_null() {
        return Error::memory_allocation.to_error_code();
    }
    regenSize = ZSTD_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTD_freeDCtx(dctx);
    regenSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_nextSrcSizeToDecompress))]
pub unsafe extern "C" fn ZSTD_nextSrcSizeToDecompress(dctx: *mut ZSTD_DCtx) -> size_t {
    (*dctx).expected
}

unsafe fn ZSTD_nextSrcSizeToDecompressWithInputSize(
    dctx: *mut ZSTD_DCtx,
    inputSize: size_t,
) -> size_t {
    match (*dctx).stage {
        DecompressStage::DecompressBlock | DecompressStage::DecompressLastBlock => {
            if (*dctx).bType != BlockType::Raw {
                return (*dctx).expected;
            }

            // Apparently it's possible for min > max here, so Ord::clamp would panic.
            Ord::max(1, Ord::min(inputSize, (*dctx).expected))
        }
        _ => (*dctx).expected,
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_nextInputType))]
pub unsafe extern "C" fn ZSTD_nextInputType(dctx: *mut ZSTD_DCtx) -> ZSTD_nextInputType_e {
    (*dctx).stage.to_next_input_type() as ZSTD_nextInputType_e
}

unsafe fn ZSTD_isSkipFrame(dctx: *mut ZSTD_DCtx) -> core::ffi::c_int {
    matches!((*dctx).stage, DecompressStage::SkipFrame) as core::ffi::c_int
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressContinue))]
pub unsafe extern "C" fn ZSTD_decompressContinue(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize != ZSTD_nextSrcSizeToDecompressWithInputSize(dctx, srcSize) {
        return Error::srcSize_wrong.to_error_code();
    }
    ZSTD_checkContinuity(dctx, dst, dstCapacity);
    (*dctx).processedCSize = ((*dctx).processedCSize as size_t).wrapping_add(srcSize) as u64;
    match (*dctx).stage {
        DecompressStage::GetFrameHeaderSize => {
            if (*dctx).format == Format::ZSTD_f_zstd1
                && MEM_readLE32(src) & ZSTD_MAGIC_SKIPPABLE_MASK
                    == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint
            {
                libc::memcpy(
                    ((*dctx).headerBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    src,
                    srcSize as libc::size_t,
                );
                (*dctx).expected = (ZSTD_SKIPPABLEHEADERSIZE as size_t).wrapping_sub(srcSize);
                (*dctx).stage = DecompressStage::DecodeSkippableHeader;
                return 0;
            }
            let src_slice = core::slice::from_raw_parts(src.cast(), srcSize);
            (*dctx).headerSize = frame_header_size_internal(src_slice, (*dctx).format);
            if ERR_isError((*dctx).headerSize) != 0 {
                return (*dctx).headerSize;
            }
            libc::memcpy(
                ((*dctx).headerBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                src,
                srcSize as libc::size_t,
            );
            (*dctx).expected = ((*dctx).headerSize).wrapping_sub(srcSize);
            (*dctx).stage = DecompressStage::DecodeFrameHeader;
            0
        }
        DecompressStage::DecodeFrameHeader => {
            libc::memcpy(
                ((*dctx).headerBuffer)
                    .as_mut_ptr()
                    .add(((*dctx).headerSize).wrapping_sub(srcSize))
                    as *mut core::ffi::c_void,
                src,
                srcSize as libc::size_t,
            );
            let err_code =
                ZSTD_decodeFrameHeader(dctx, &(&(*dctx).headerBuffer)[..(*dctx).headerSize]);
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            (*dctx).expected = ZSTD_blockHeaderSize;
            (*dctx).stage = DecompressStage::DecodeBlockHeader;
            0
        }
        DecompressStage::DecodeBlockHeader => {
            let mut bp = blockProperties_t {
                blockType: BlockType::Raw,
                lastBlock: 0,
                origSize: 0,
            };
            let cBlockSize = ZSTD_getcBlockSize(src, ZSTD_blockHeaderSize, &mut bp);
            if ERR_isError(cBlockSize) != 0 {
                return cBlockSize;
            }
            if cBlockSize > (*dctx).fParams.blockSizeMax as size_t {
                return Error::corruption_detected.to_error_code();
            }
            (*dctx).expected = cBlockSize;
            (*dctx).bType = bp.blockType;
            (*dctx).rleSize = bp.origSize as size_t;
            if cBlockSize != 0 {
                (*dctx).stage = match bp.lastBlock {
                    0 => DecompressStage::DecompressBlock,
                    _ => DecompressStage::DecompressLastBlock,
                };
                return 0;
            }
            if bp.lastBlock != 0 {
                if (*dctx).fParams.checksumFlag != 0 {
                    (*dctx).expected = 4;
                    (*dctx).stage = DecompressStage::CheckChecksum;
                } else {
                    (*dctx).expected = 0;
                    (*dctx).stage = DecompressStage::GetFrameHeaderSize;
                }
            } else {
                (*dctx).expected = ZSTD_blockHeaderSize;
                (*dctx).stage = DecompressStage::DecodeBlockHeader;
            }
            0
        }

        DecompressStage::DecompressBlock | DecompressStage::DecompressLastBlock => {
            let mut rSize: size_t = 0;
            match (*dctx).bType {
                BlockType::Compressed => {
                    rSize = ZSTD_decompressBlock_internal(
                        dctx,
                        dst,
                        dstCapacity,
                        src,
                        srcSize,
                        is_streaming,
                    );
                    (*dctx).expected = 0;
                }
                BlockType::Raw => {
                    rSize = ZSTD_copyRawBlock(dst, dstCapacity, src, srcSize);
                    let err_code_0 = rSize;
                    if ERR_isError(err_code_0) != 0 {
                        return err_code_0;
                    }
                    (*dctx).expected = ((*dctx).expected).wrapping_sub(rSize);
                }
                BlockType::Rle => {
                    rSize =
                        ZSTD_setRleBlock(dst, dstCapacity, *(src as *const u8), (*dctx).rleSize);
                    (*dctx).expected = 0;
                }
                BlockType::Reserved => {
                    return Error::corruption_detected.to_error_code();
                }
            }
            let err_code_1 = rSize;
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            if rSize > (*dctx).fParams.blockSizeMax as size_t {
                return Error::corruption_detected.to_error_code();
            }
            (*dctx).decodedSize = ((*dctx).decodedSize as size_t).wrapping_add(rSize) as u64 as u64;
            if (*dctx).validateChecksum != 0 {
                ZSTD_XXH64_update(&mut (*dctx).xxhState, dst, rSize as usize);
            }
            (*dctx).previousDstEnd =
                (dst as *mut core::ffi::c_char).add(rSize) as *const core::ffi::c_void;
            if (*dctx).expected > 0 {
                return rSize;
            }
            if (*dctx).stage == DecompressStage::DecompressLastBlock {
                if (*dctx).fParams.frameContentSize != (0 as core::ffi::c_ulonglong).wrapping_sub(1)
                    && (*dctx).decodedSize as core::ffi::c_ulonglong
                        != (*dctx).fParams.frameContentSize
                {
                    return Error::corruption_detected.to_error_code();
                }
                if (*dctx).fParams.checksumFlag != 0 {
                    (*dctx).expected = 4;
                    (*dctx).stage = DecompressStage::CheckChecksum;
                } else {
                    ZSTD_DCtx_trace_end(dctx, (*dctx).decodedSize, (*dctx).processedCSize, 1);
                    (*dctx).expected = 0;
                    (*dctx).stage = DecompressStage::GetFrameHeaderSize;
                }
            } else {
                (*dctx).stage = DecompressStage::DecodeBlockHeader;
                (*dctx).expected = ZSTD_blockHeaderSize;
            }
            rSize
        }
        DecompressStage::CheckChecksum => {
            if (*dctx).validateChecksum != 0 {
                let h32 = ZSTD_XXH64_digest(&mut (*dctx).xxhState) as u32;
                let check32 = MEM_readLE32(src);
                if check32 != h32 {
                    return Error::checksum_wrong.to_error_code();
                }
            }
            ZSTD_DCtx_trace_end(dctx, (*dctx).decodedSize, (*dctx).processedCSize, 1);
            (*dctx).expected = 0;
            (*dctx).stage = DecompressStage::GetFrameHeaderSize;
            0
        }
        DecompressStage::DecodeSkippableHeader => {
            libc::memcpy(
                ((*dctx).headerBuffer)
                    .as_mut_ptr()
                    .add((8 as size_t).wrapping_sub(srcSize))
                    as *mut core::ffi::c_void,
                src,
                srcSize as libc::size_t,
            );
            (*dctx).expected = MEM_readLE32(
                ((*dctx).headerBuffer)
                    .as_mut_ptr().add(ZSTD_FRAMEIDSIZE) as *const core::ffi::c_void,
            ) as size_t;
            (*dctx).stage = DecompressStage::SkipFrame;
            0
        }
        DecompressStage::SkipFrame => {
            (*dctx).expected = 0;
            (*dctx).stage = DecompressStage::GetFrameHeaderSize;
            0
        }
    }
}

pub unsafe fn ZSTD_loadDEntropy(entropy: &mut ZSTD_entropyDTables_t, dict: &[u8]) -> size_t {
    let Some((_, mut dictPtr)) = dict.split_at_checked(8) else {
        return Error::dictionary_corrupted.to_error_code();
    };

    const _: () = assert!(
        size_of::<crate::lib::decompress::SymbolTable<512>>()
            >= size_of::<HUF_ReadDTableX2_Workspace>()
    );
    const _: () = assert!(
        align_of::<crate::lib::decompress::SymbolTable<512>>()
            >= align_of::<HUF_ReadDTableX2_Workspace>()
    );

    let workspace = &mut entropy.LLTable;
    let wksp: &mut HUF_ReadDTableX2_Workspace = unsafe { core::mem::transmute(workspace) };

    let hSize = HUF_readDTableX2_wksp(&mut entropy.hufTable, dictPtr, wksp, 0);
    if ERR_isError(hSize) != 0 {
        return Error::dictionary_corrupted.to_error_code();
    }

    dictPtr = &dictPtr[hSize..];
    let mut offcodeNCount: [core::ffi::c_short; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff as core::ffi::c_uint;
    let mut offcodeLog: core::ffi::c_uint = 0;
    let offcodeHeaderSize = FSE_readNCount_slice(
        &mut offcodeNCount,
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dictPtr,
    );

    let Ok(offcodeHeaderSize) = offcodeHeaderSize else {
        return Error::dictionary_corrupted.to_error_code();
    };
    if offcodeMaxValue > 31 {
        return Error::dictionary_corrupted.to_error_code();
    }
    if offcodeLog > 8 {
        return Error::dictionary_corrupted.to_error_code();
    }
    ZSTD_buildFSETable(
        &mut entropy.OFTable,
        &offcodeNCount[..=offcodeMaxValue as usize],
        &OF_base,
        &OF_bits,
        offcodeLog,
        &mut entropy.workspace,
        false,
    );
    dictPtr = &dictPtr[offcodeHeaderSize..];
    let mut matchlengthNCount: [core::ffi::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as core::ffi::c_uint;
    let mut matchlengthLog: core::ffi::c_uint = 0;
    let matchlengthHeaderSize = FSE_readNCount_slice(
        &mut matchlengthNCount,
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dictPtr,
    );
    let Ok(matchlengthHeaderSize) = matchlengthHeaderSize else {
        return Error::dictionary_corrupted.to_error_code();
    };
    if matchlengthMaxValue > 52 {
        return Error::dictionary_corrupted.to_error_code();
    }
    if matchlengthLog > 9 {
        return Error::dictionary_corrupted.to_error_code();
    }
    ZSTD_buildFSETable(
        &mut entropy.MLTable,
        &matchlengthNCount[..=matchlengthMaxValue as usize],
        &ML_base,
        &ML_bits,
        matchlengthLog,
        &mut entropy.workspace,
        false,
    );
    dictPtr = &dictPtr[matchlengthHeaderSize..];
    let mut litlengthNCount: [core::ffi::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as core::ffi::c_uint;
    let mut litlengthLog: core::ffi::c_uint = 0;
    let litlengthHeaderSize = FSE_readNCount_slice(
        &mut litlengthNCount,
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dictPtr,
    );
    let Ok(litlengthHeaderSize) = litlengthHeaderSize else {
        return Error::dictionary_corrupted.to_error_code();
    };
    if litlengthMaxValue > 35 {
        return Error::dictionary_corrupted.to_error_code();
    }
    if litlengthLog > 9 {
        return Error::dictionary_corrupted.to_error_code();
    }
    ZSTD_buildFSETable(
        &mut entropy.LLTable,
        &litlengthNCount[..=litlengthMaxValue as usize],
        &LL_base,
        &LL_bits,
        litlengthLog,
        &mut entropy.workspace,
        false,
    );
    dictPtr = &dictPtr[litlengthHeaderSize..];
    let Some((chunk, dict_content)) = dictPtr.split_first_chunk::<12>() else {
        return Error::dictionary_corrupted.to_error_code();
    };

    let dict_content_size = dict_content.len();
    for (i, rep) in chunk.as_chunks::<4>().0.iter().enumerate() {
        let rep = u32::from_le_bytes(*rep);
        if rep == 0 || rep as size_t > dict_content_size {
            return Error::dictionary_corrupted.to_error_code();
        }
        entropy.rep[i] = rep;
    }

    dict.len() - dict_content_size
}

unsafe fn ZSTD_refDictContent(dctx: *mut ZSTD_DCtx, dict: &[u8]) -> size_t {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).virtualStart = dict
        .as_ptr()
        .sub((((*dctx).previousDstEnd).byte_offset_from((*dctx).prefixStart)) as usize)
        .cast();
    (*dctx).prefixStart = dict.as_ptr().cast();
    (*dctx).previousDstEnd = dict.as_ptr_range().end.cast();

    0
}

unsafe fn ZSTD_decompress_insertDictionary(dctx: *mut ZSTD_DCtx, dict: &[u8]) -> size_t {
    let ([magic, dict_id, ..], _) = dict.as_chunks::<4>() else {
        return ZSTD_refDictContent(dctx, dict);
    };

    let magic = u32::from_le_bytes(*magic);
    if magic != ZSTD_MAGIC_DICTIONARY {
        return ZSTD_refDictContent(dctx, dict);
    }
    (*dctx).dictID = u32::from_le_bytes(*dict_id);
    let eSize = ZSTD_loadDEntropy(&mut (*dctx).entropy, dict);
    if ERR_isError(eSize) != 0 {
        return Error::dictionary_corrupted.to_error_code();
    }

    (*dctx).fseEntropy = 1;
    (*dctx).litEntropy = (*dctx).fseEntropy;

    ZSTD_refDictContent(dctx, &dict[eSize..])
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBegin))]
pub unsafe extern "C" fn ZSTD_decompressBegin(dctx: *mut ZSTD_DCtx) -> size_t {
    (*dctx).traceCtx = ZSTD_trace_decompress_begin(dctx);
    (*dctx).expected = ZSTD_startingInputLength((*dctx).format);
    (*dctx).stage = DecompressStage::GetFrameHeaderSize;
    (*dctx).processedCSize = 0;
    (*dctx).decodedSize = 0;
    (*dctx).previousDstEnd = core::ptr::null();
    (*dctx).prefixStart = core::ptr::null();
    (*dctx).virtualStart = core::ptr::null();
    (*dctx).dictEnd = core::ptr::null();
    (*dctx).entropy.hufTable.description = DTableDesc::from_u32(12 * 0x1000001);
    (*dctx).fseEntropy = 0;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    (*dctx).dictID = 0;
    (*dctx).bType = BlockType::Reserved;
    (*dctx).isFrameDecompression = 1;
    libc::memcpy(
        ((*dctx).entropy.rep).as_mut_ptr() as *mut core::ffi::c_void,
        repStartValue.as_ptr() as *const core::ffi::c_void,
        ::core::mem::size_of::<[u32; 3]>() as libc::size_t,
    );
    (*dctx).LLTptr = ((*dctx).entropy.LLTable).as_mut_ptr();
    (*dctx).MLTptr = ((*dctx).entropy.MLTable).as_mut_ptr();
    (*dctx).OFTptr = ((*dctx).entropy.OFTable).as_mut_ptr();
    (*dctx).HUFptr = &raw const (*dctx).entropy.hufTable;
    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBegin_usingDict))]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDict(
    dctx: *mut ZSTD_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let err_code = ZSTD_decompressBegin(dctx);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }

    if dict.is_null() || dictSize == 0 {
        return 0;
    }

    let dict = core::slice::from_raw_parts(dict.cast::<u8>(), dictSize);
    if ERR_isError(ZSTD_decompress_insertDictionary(dctx, dict)) != 0 {
        return Error::dictionary_corrupted.to_error_code();
    }

    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBegin_usingDDict))]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDDict(
    dctx: *mut ZSTD_DCtx,
    ddict: *const ZSTD_DDict,
) -> size_t {
    if !ddict.is_null() {
        let dictStart = ZSTD_DDict_dictContent(ddict) as *const core::ffi::c_char;
        let dictSize = ZSTD_DDict_dictSize(ddict);
        let dictEnd = dictStart.add(dictSize) as *const core::ffi::c_void;
        (*dctx).ddictIsCold = ((*dctx).dictEnd != dictEnd) as core::ffi::c_int;
    }
    let err_code = ZSTD_decompressBegin(dctx);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if !ddict.is_null() {
        ZSTD_copyDDictParameters(dctx, ddict);
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getDictID_fromDict))]
pub unsafe extern "C" fn ZSTD_getDictID_fromDict(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> core::ffi::c_uint {
    if dictSize < 8 {
        return 0;
    }
    if MEM_readLE32(dict) != ZSTD_MAGIC_DICTIONARY {
        return 0;
    }
    MEM_readLE32(
        (dict as *const core::ffi::c_char).add(ZSTD_FRAMEIDSIZE)
            as *const core::ffi::c_void,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getDictID_fromFrame))]
pub unsafe extern "C" fn ZSTD_getDictID_fromFrame(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_uint {
    let mut zfp = {
        ZSTD_FrameHeader {
            frameContentSize: 0,
            windowSize: 0,
            blockSizeMax: 0,
            frameType: ZSTD_frame,
            headerSize: 0,
            dictID: 0,
            checksumFlag: 0,
            _reserved1: 0,
            _reserved2: 0,
        }
    };
    let hError = ZSTD_getFrameHeader(&mut zfp, src, srcSize);
    if ERR_isError(hError) != 0 {
        return 0;
    }
    zfp.dictID
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompress_usingDDict))]
pub unsafe extern "C" fn ZSTD_decompress_usingDDict(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    ddict: *const ZSTD_DDict,
) -> size_t {
    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast::<u8>(), srcSize)
    };

    ZSTD_decompressMultiFrame(
        dctx,
        dst,
        dstCapacity,
        src,
        core::ptr::null(),
        0,
        ddict.as_ref(),
    )
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDStream))]
pub unsafe extern "C" fn ZSTD_createDStream() -> *mut ZSTD_DStream {
    ZSTD_createDCtx_internal(ZSTD_defaultCMem)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initStaticDStream))]
pub unsafe extern "C" fn ZSTD_initStaticDStream(
    workspace: *mut core::ffi::c_void,
    workspaceSize: size_t,
) -> *mut ZSTD_DStream {
    ZSTD_initStaticDCtx(workspace, workspaceSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDStream_advanced))]
pub unsafe extern "C" fn ZSTD_createDStream_advanced(
    customMem: ZSTD_customMem,
) -> *mut ZSTD_DStream {
    ZSTD_createDCtx_internal(customMem)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeDStream))]
pub unsafe extern "C" fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t {
    ZSTD_freeDCtx(zds)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DStreamInSize))]
pub unsafe extern "C" fn ZSTD_DStreamInSize() -> size_t {
    (ZSTD_BLOCKSIZE_MAX as size_t).wrapping_add(ZSTD_blockHeaderSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DStreamOutSize))]
pub unsafe extern "C" fn ZSTD_DStreamOutSize() -> size_t {
    ZSTD_BLOCKSIZE_MAX as size_t
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_loadDictionary_advanced))]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_advanced(
    dctx: *mut ZSTD_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if (*dctx).streamStage != StreamStage::Init {
        return Error::stage_wrong.to_error_code();
    }
    ZSTD_clearDict(dctx);
    if !dict.is_null() && dictSize != 0 {
        (*dctx).ddictLocal = ZSTD_createDDict_advanced(
            dict,
            dictSize,
            dictLoadMethod,
            dictContentType,
            (*dctx).customMem,
        );
        if ((*dctx).ddictLocal).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*dctx).ddict = (*dctx).ddictLocal;
        (*dctx).dictUses = ZSTD_use_indefinitely;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_loadDictionary_byReference))]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary_byReference(
    dctx: *mut ZSTD_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize, ZSTD_dlm_byRef, ZSTD_dct_auto)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_loadDictionary))]
pub unsafe extern "C" fn ZSTD_DCtx_loadDictionary(
    dctx: *mut ZSTD_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    ZSTD_DCtx_loadDictionary_advanced(dctx, dict, dictSize, ZSTD_dlm_byCopy, ZSTD_dct_auto)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_refPrefix_advanced))]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix_advanced(
    dctx: *mut ZSTD_DCtx,
    prefix: *const core::ffi::c_void,
    prefixSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
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
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_refPrefix))]
pub unsafe extern "C" fn ZSTD_DCtx_refPrefix(
    dctx: *mut ZSTD_DCtx,
    prefix: *const core::ffi::c_void,
    prefixSize: size_t,
) -> size_t {
    ZSTD_DCtx_refPrefix_advanced(dctx, prefix, prefixSize, ZSTD_dct_rawContent)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initDStream_usingDict))]
pub unsafe extern "C" fn ZSTD_initDStream_usingDict(
    zds: *mut ZSTD_DStream,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
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
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initDStream))]
pub unsafe extern "C" fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t {
    let err_code = ZSTD_DCtx_reset(zds, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    let err_code_0 = ZSTD_DCtx_refDDict(zds, core::ptr::null::<ZSTD_DDict>());
    if ERR_isError(err_code_0) != 0 {
        return err_code_0;
    }
    ZSTD_startingInputLength((*zds).format)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initDStream_usingDDict))]
pub unsafe extern "C" fn ZSTD_initDStream_usingDDict(
    dctx: *mut ZSTD_DStream,
    ddict: *const ZSTD_DDict,
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
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_resetDStream))]
pub unsafe extern "C" fn ZSTD_resetDStream(dctx: *mut ZSTD_DStream) -> size_t {
    let err_code = ZSTD_DCtx_reset(dctx, ZSTD_reset_session_only);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    ZSTD_startingInputLength((*dctx).format)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_refDDict))]
pub unsafe extern "C" fn ZSTD_DCtx_refDDict(
    dctx: *mut ZSTD_DCtx,
    ddict: *const ZSTD_DDict,
) -> size_t {
    if (*dctx).streamStage != StreamStage::Init {
        return Error::stage_wrong.to_error_code();
    }
    ZSTD_clearDict(dctx);

    if !ddict.is_null() {
        (*dctx).ddict = ddict;
        (*dctx).dictUses = ZSTD_use_indefinitely;
        if (*dctx).refMultipleDDicts == MultipleDDicts::Multiple {
            if ((*dctx).ddictSet).is_null() {
                (*dctx).ddictSet = ZSTD_createDDictHashSet((*dctx).customMem);
            }

            let Some(ddictSet) = (*dctx).ddictSet.as_mut() else {
                return Error::memory_allocation.to_error_code();
            };

            let err_code = ZSTD_DDictHashSet_addDDict(ddictSet, ddict, (*dctx).customMem);
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
        }
    }

    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_setMaxWindowSize))]
pub unsafe extern "C" fn ZSTD_DCtx_setMaxWindowSize(
    dctx: *mut ZSTD_DCtx,
    maxWindowSize: size_t,
) -> size_t {
    let bounds = ZSTD_dParam_getBounds(ZSTD_d_windowLogMax);
    let min = (1) << bounds.lowerBound;
    let max = (1) << bounds.upperBound;
    if (*dctx).streamStage != StreamStage::Init {
        return Error::stage_wrong.to_error_code();
    }
    if maxWindowSize < min {
        return Error::parameter_outOfBound.to_error_code();
    }
    if maxWindowSize > max {
        return Error::parameter_outOfBound.to_error_code();
    }
    (*dctx).maxWindowSize = maxWindowSize;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_setFormat))]
pub unsafe extern "C" fn ZSTD_DCtx_setFormat(
    dctx: *mut ZSTD_DCtx,
    format: ZSTD_format_e,
) -> size_t {
    ZSTD_DCtx_setParameter(
        dctx,
        ZSTD_d_format as ZSTD_dParameter,
        format as core::ffi::c_int,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_dParam_getBounds))]
pub unsafe extern "C" fn ZSTD_dParam_getBounds(dParam: ZSTD_dParameter) -> ZSTD_bounds {
    let mut bounds = {
        ZSTD_bounds {
            error: 0,
            lowerBound: 0,
            upperBound: 0,
        }
    };
    match dParam as core::ffi::c_uint {
        100 => {
            bounds.lowerBound = ZSTD_WINDOWLOG_ABSOLUTEMIN;
            bounds.upperBound = if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            };
            return bounds;
        }
        1000 => {
            bounds.lowerBound = Format::ZSTD_f_zstd1 as core::ffi::c_int;
            bounds.upperBound = Format::ZSTD_f_zstd1_magicless as core::ffi::c_int;
            return bounds;
        }
        1001 => {
            bounds.lowerBound = BufferMode::Buffered as core::ffi::c_int;
            bounds.upperBound = BufferMode::Stable as core::ffi::c_int;
            return bounds;
        }
        1002 => {
            bounds.lowerBound = ZSTD_d_validateChecksum as core::ffi::c_int;
            bounds.upperBound = ZSTD_d_ignoreChecksum as core::ffi::c_int;
            return bounds;
        }
        1003 => {
            bounds.lowerBound = MultipleDDicts::Single as core::ffi::c_int;
            bounds.upperBound = MultipleDDicts::Multiple as core::ffi::c_int;
            return bounds;
        }
        1004 => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            return bounds;
        }
        1005 => {
            bounds.lowerBound = ZSTD_BLOCKSIZE_MAX_MIN;
            bounds.upperBound = ZSTD_BLOCKSIZE_MAX;
            return bounds;
        }
        _ => {}
    }
    bounds.error = Error::parameter_unsupported.to_error_code();
    bounds
}
unsafe fn ZSTD_dParam_withinBounds(
    dParam: ZSTD_dParameter,
    value: core::ffi::c_int,
) -> core::ffi::c_int {
    let bounds = ZSTD_dParam_getBounds(dParam);
    if ERR_isError(bounds.error) != 0 {
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

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_getParameter))]
pub unsafe extern "C" fn ZSTD_DCtx_getParameter(
    dctx: *mut ZSTD_DCtx,
    param: ZSTD_dParameter,
    value: *mut core::ffi::c_int,
) -> size_t {
    match param as core::ffi::c_uint {
        100 => {
            *value = (*dctx).maxWindowSize.ilog2() as i32;
            0
        }
        1000 => {
            *value = (*dctx).format as core::ffi::c_int;
            0
        }
        1001 => {
            *value = (*dctx).outBufferMode as core::ffi::c_int;
            0
        }
        1002 => {
            *value = (*dctx).forceIgnoreChecksum as core::ffi::c_int;
            0
        }
        1003 => {
            *value = (*dctx).refMultipleDDicts as core::ffi::c_int;
            0
        }
        1004 => {
            *value = (*dctx).disableHufAsm;
            0
        }
        1005 => {
            *value = (*dctx).maxBlockSizeParam;
            0
        }
        _ => Error::parameter_unsupported.to_error_code(),
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_setParameter))]
pub unsafe extern "C" fn ZSTD_DCtx_setParameter(
    dctx: *mut ZSTD_DCtx,
    dParam: ZSTD_dParameter,
    mut value: core::ffi::c_int,
) -> size_t {
    if (*dctx).streamStage != StreamStage::Init {
        return Error::stage_wrong.to_error_code();
    }
    match dParam as core::ffi::c_uint {
        100 => {
            if value == 0 {
                value = ZSTD_WINDOWLOG_LIMIT_DEFAULT;
            }
            if ZSTD_dParam_withinBounds(ZSTD_d_windowLogMax, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*dctx).maxWindowSize = (1) << value;
            return 0;
        }
        1000 => {
            let Ok(format) = Format::try_from(value as ZSTD_format_e) else {
                return Error::parameter_outOfBound.to_error_code();
            };

            (*dctx).format = format;

            return 0;
        }
        1001 => {
            let Ok(value) = BufferMode::try_from(value as u32) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*dctx).outBufferMode = value;
            return 0;
        }
        1002 => {
            if ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam3, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*dctx).forceIgnoreChecksum = value as ZSTD_forceIgnoreChecksum_e;
            return 0;
        }
        1003 => {
            let Ok(value) = MultipleDDicts::try_from(value as u32) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            if (*dctx).staticSize != 0 {
                return Error::parameter_unsupported.to_error_code();
            }
            (*dctx).refMultipleDDicts = value;
            return 0;
        }
        1004 => {
            if ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam5, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*dctx).disableHufAsm = (value != 0) as core::ffi::c_int;
            return 0;
        }
        1005 => {
            if value != 0 && ZSTD_dParam_withinBounds(ZSTD_d_experimentalParam6, value) == 0 {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*dctx).maxBlockSizeParam = value;
            return 0;
        }
        _ => {}
    }
    Error::parameter_unsupported.to_error_code()
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_reset))]
pub unsafe extern "C" fn ZSTD_DCtx_reset(
    dctx: *mut ZSTD_DCtx,
    reset: ZSTD_ResetDirective,
) -> size_t {
    if reset as core::ffi::c_uint
        == ZSTD_reset_session_only as core::ffi::c_int as core::ffi::c_uint
        || reset as core::ffi::c_uint
            == ZSTD_reset_session_and_parameters as core::ffi::c_int as core::ffi::c_uint
    {
        (*dctx).streamStage = StreamStage::Init;
        (*dctx).noForwardProgress = 0;
        (*dctx).isFrameDecompression = 1;
    }
    if reset as core::ffi::c_uint == ZSTD_reset_parameters as core::ffi::c_int as core::ffi::c_uint
        || reset as core::ffi::c_uint
            == ZSTD_reset_session_and_parameters as core::ffi::c_int as core::ffi::c_uint
    {
        if (*dctx).streamStage != StreamStage::Init {
            return Error::stage_wrong.to_error_code();
        }
        ZSTD_clearDict(dctx);
        ZSTD_DCtx_resetParameters(dctx);
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_DStream))]
pub unsafe extern "C" fn ZSTD_sizeof_DStream(dctx: *const ZSTD_DStream) -> size_t {
    ZSTD_sizeof_DCtx(dctx)
}
unsafe fn ZSTD_decodingBufferSize_internal(
    windowSize: core::ffi::c_ulonglong,
    frameContentSize: core::ffi::c_ulonglong,
    blockSizeMax: size_t,
) -> size_t {
    let blockSize = if ((if windowSize < ((1) << 17) as core::ffi::c_ulonglong {
        windowSize
    } else {
        ((1) << 17) as core::ffi::c_ulonglong
    }) as size_t)
        < blockSizeMax
    {
        (if windowSize < ((1) << 17) as core::ffi::c_ulonglong {
            windowSize
        } else {
            ((1) << 17) as core::ffi::c_ulonglong
        }) as size_t
    } else {
        blockSizeMax
    };
    let neededRBSize = windowSize
        .wrapping_add((blockSize * 2) as core::ffi::c_ulonglong)
        .wrapping_add((WILDCOPY_OVERLENGTH * 2) as core::ffi::c_ulonglong);
    let neededSize = if frameContentSize < neededRBSize {
        frameContentSize
    } else {
        neededRBSize
    };
    let minRBSize = neededSize as size_t;
    if minRBSize as core::ffi::c_ulonglong != neededSize {
        return Error::frameParameter_windowTooLarge.to_error_code();
    }
    minRBSize
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decodingBufferSize_min))]
pub unsafe extern "C" fn ZSTD_decodingBufferSize_min(
    windowSize: core::ffi::c_ulonglong,
    frameContentSize: core::ffi::c_ulonglong,
) -> size_t {
    ZSTD_decodingBufferSize_internal(windowSize, frameContentSize, ZSTD_BLOCKSIZE_MAX as size_t)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateDStreamSize))]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize(windowSize: size_t) -> size_t {
    let blockSize = if windowSize < ((1) << 17) as size_t {
        windowSize
    } else {
        ((1) << 17) as size_t
    };
    let inBuffSize = blockSize;
    let outBuffSize = ZSTD_decodingBufferSize_min(
        windowSize as core::ffi::c_ulonglong,
        ZSTD_CONTENTSIZE_UNKNOWN,
    );
    (ZSTD_estimateDCtxSize())
        .wrapping_add(inBuffSize)
        .wrapping_add(outBuffSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateDStreamSize_fromFrame))]
pub unsafe extern "C" fn ZSTD_estimateDStreamSize_fromFrame(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let windowSizeMax = (1)
        << (if ::core::mem::size_of::<size_t>() == 4 {
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
    if err > 0 {
        return Error::srcSize_wrong.to_error_code();
    }
    if zfh.windowSize > windowSizeMax as core::ffi::c_ulonglong {
        return Error::frameParameter_windowTooLarge.to_error_code();
    }
    ZSTD_estimateDStreamSize(zfh.windowSize as size_t)
}
unsafe fn ZSTD_DCtx_isOverflow(
    zds: *mut ZSTD_DStream,
    neededInBuffSize: size_t,
    neededOutBuffSize: size_t,
) -> core::ffi::c_int {
    (((*zds).inBuffSize).wrapping_add((*zds).outBuffSize)
        >= neededInBuffSize.wrapping_add(neededOutBuffSize)
            * ZSTD_WORKSPACETOOLARGE_FACTOR as size_t) as core::ffi::c_int
}
unsafe fn ZSTD_DCtx_updateOversizedDuration(
    zds: *mut ZSTD_DStream,
    neededInBuffSize: size_t,
    neededOutBuffSize: size_t,
) {
    if ZSTD_DCtx_isOverflow(zds, neededInBuffSize, neededOutBuffSize) != 0 {
        (*zds).oversizedDuration = ((*zds).oversizedDuration).wrapping_add(1);
        (*zds).oversizedDuration;
    } else {
        (*zds).oversizedDuration = 0;
    };
}
unsafe fn ZSTD_DCtx_isOversizedTooLong(zds: *mut ZSTD_DStream) -> core::ffi::c_int {
    ((*zds).oversizedDuration >= ZSTD_WORKSPACETOOLARGE_MAXDURATION as size_t) as core::ffi::c_int
}
unsafe fn ZSTD_checkOutBuffer(zds: *const ZSTD_DStream, output: *const ZSTD_outBuffer) -> size_t {
    if (*zds).outBufferMode != BufferMode::Stable {
        return 0;
    }
    if (*zds).streamStage == StreamStage::Init {
        return 0;
    }
    let expect = (*zds).expectedOutBuffer;
    if expect.dst == (*output).dst && expect.pos == (*output).pos && expect.size == (*output).size {
        return 0;
    }
    Error::dstBuffer_wrong.to_error_code()
}
unsafe fn ZSTD_decompressContinueStream(
    zds: *mut ZSTD_DStream,
    op: *mut *mut core::ffi::c_char,
    oend: *mut core::ffi::c_char,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let isSkipFrame = ZSTD_isSkipFrame(zds);
    if (*zds).outBufferMode == BufferMode::Buffered {
        let dstSize = if isSkipFrame != 0 {
            0
        } else {
            ((*zds).outBuffSize).wrapping_sub((*zds).outStart)
        };
        let decodedSize = ZSTD_decompressContinue(
            zds,
            ((*zds).outBuff).add((*zds).outStart) as *mut core::ffi::c_void,
            dstSize,
            src,
            srcSize,
        );
        let err_code = decodedSize;
        if ERR_isError(err_code) != 0 {
            return err_code;
        }
        if decodedSize == 0 && isSkipFrame == 0 {
            (*zds).streamStage = StreamStage::Read;
        } else {
            (*zds).outEnd = ((*zds).outStart).wrapping_add(decodedSize);
            (*zds).streamStage = StreamStage::Flush;
        }
    } else {
        let dstSize_0 = if isSkipFrame != 0 {
            0
        } else {
            oend.offset_from(*op) as size_t
        };
        let decodedSize_0 =
            ZSTD_decompressContinue(zds, *op as *mut core::ffi::c_void, dstSize_0, src, srcSize);
        let err_code_0 = decodedSize_0;
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        *op = (*op).add(decodedSize_0);
        (*zds).streamStage = StreamStage::Read;
    }
    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressStream))]
pub unsafe extern "C" fn ZSTD_decompressStream(
    zds: *mut ZSTD_DStream,
    output: *mut ZSTD_outBuffer,
    input: *mut ZSTD_inBuffer,
) -> size_t {
    let output = output.as_mut().unwrap();
    let input = input.as_mut().unwrap();

    let src = input.src as *const core::ffi::c_char;
    let istart = if input.pos != 0 {
        src.add(input.pos)
    } else {
        src
    };
    let iend = if input.size != 0 {
        src.add(input.size)
    } else {
        src
    };
    let mut ip = istart;
    let dst = output.dst as *mut core::ffi::c_char;
    let ostart = if output.pos != 0 {
        dst.add(output.pos)
    } else {
        dst
    };
    let oend = if output.size != 0 {
        dst.add(output.size)
    } else {
        dst
    };
    let mut op = ostart;
    let mut some_more_work = true;
    if input.pos > input.size {
        return Error::srcSize_wrong.to_error_code();
    }
    if output.pos > output.size {
        return Error::dstSize_tooSmall.to_error_code();
    }
    let err_code = ZSTD_checkOutBuffer(zds, output);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }

    while some_more_work {
        #[derive(Eq, PartialEq)]
        enum Block {
            LoadHeader,
            Read,
            Load,
        }
        let mut current_block: Block;
        match (*zds).streamStage {
            StreamStage::Init => {
                (*zds).streamStage = StreamStage::LoadHeader;
                (*zds).outEnd = 0;
                (*zds).outStart = (*zds).outEnd;
                (*zds).inPos = (*zds).outStart;
                (*zds).lhSize = (*zds).inPos;
                (*zds).legacyVersion = 0;
                (*zds).hostageByte = 0;
                (*zds).expectedOutBuffer = *output;
                current_block = Block::LoadHeader;
            }
            StreamStage::LoadHeader => {
                current_block = Block::LoadHeader;
            }
            StreamStage::Read => {
                current_block = Block::Read;
            }
            StreamStage::Load => {
                current_block = Block::Load;
            }
            StreamStage::Flush => {
                let toFlushSize = ((*zds).outEnd).wrapping_sub((*zds).outStart);
                let flushedSize = ZSTD_limitCopy(
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as size_t,
                    ((*zds).outBuff).add((*zds).outStart) as *const core::ffi::c_void,
                    toFlushSize,
                );

                op = if !op.is_null() {
                    op.add(flushedSize)
                } else {
                    op
                };

                (*zds).outStart = ((*zds).outStart).wrapping_add(flushedSize);
                if flushedSize == toFlushSize {
                    // flush completed
                    (*zds).streamStage = StreamStage::Read;
                    if ((*zds).outBuffSize as core::ffi::c_ulonglong)
                        < (*zds).fParams.frameContentSize
                        && ((*zds).outStart).wrapping_add((*zds).fParams.blockSizeMax as size_t)
                            > (*zds).outBuffSize
                    {
                        (*zds).outEnd = 0;
                        (*zds).outStart = 0;
                    }
                    continue;
                }

                // cannot complete flush
                some_more_work = false;
                continue;
            }
        }
        if current_block == Block::LoadHeader {
            drop(current_block);

            if (*zds).legacyVersion != 0 {
                if (*zds).staticSize != 0 {
                    return Error::memory_allocation.to_error_code();
                }
                let hint = ZSTD_decompressLegacyStream(
                    (*zds).legacyContext,
                    (*zds).legacyVersion,
                    output,
                    input,
                );
                if hint == 0 {
                    (*zds).streamStage = StreamStage::Init;
                }
                return hint;
            }

            let hSize = get_frame_header_advanced(
                &mut (*zds).fParams,
                &(&(*zds).headerBuffer)[..(*zds).lhSize],
                (*zds).format,
            );
            if (*zds).refMultipleDDicts != MultipleDDicts::Single && !(*zds).ddictSet.is_null() {
                ZSTD_DCtx_selectFrameDDict(zds);
            }
            if ERR_isError(hSize) != 0 {
                let legacyVersion = ZSTD_isLegacy(
                    istart as *const core::ffi::c_void,
                    iend.offset_from(istart) as size_t,
                );
                if legacyVersion != 0 {
                    let ddict = ZSTD_getDDict(zds);
                    let dict = if !ddict.is_null() {
                        ZSTD_DDict_dictContent(ddict)
                    } else {
                        core::ptr::null()
                    };
                    let dictSize = if !ddict.is_null() {
                        ZSTD_DDict_dictSize(ddict)
                    } else {
                        0
                    };
                    if (*zds).staticSize != 0 {
                        return Error::memory_allocation.to_error_code();
                    }
                    let err_code = ZSTD_initLegacyStream(
                        &mut (*zds).legacyContext,
                        (*zds).previousLegacyVersion,
                        legacyVersion,
                        dict,
                        dictSize,
                    );
                    if ERR_isError(err_code) != 0 {
                        return err_code;
                    }
                    (*zds).previousLegacyVersion = legacyVersion;
                    (*zds).legacyVersion = (*zds).previousLegacyVersion;
                    let hint = ZSTD_decompressLegacyStream(
                        (*zds).legacyContext,
                        legacyVersion,
                        output,
                        input,
                    );
                    if hint == 0 {
                        (*zds).streamStage = StreamStage::Init;
                    }
                    return hint;
                }

                // error
                return hSize;
            }

            if hSize != 0 {
                // need more input
                let toLoad = hSize - (*zds).lhSize; // if hSize!=0, hSize > zds->lhSize
                let remainingInput = iend.offset_from(ip) as size_t;
                if toLoad > remainingInput {
                    // not enough input to load full header
                    if remainingInput > 0 {
                        libc::memcpy(
                            ((*zds).headerBuffer).as_mut_ptr().add((*zds).lhSize)
                                as *mut core::ffi::c_void,
                            ip as *const core::ffi::c_void,
                            remainingInput as libc::size_t,
                        );
                        (*zds).lhSize = ((*zds).lhSize).wrapping_add(remainingInput);
                    }
                    input.pos = input.size;
                    // check first few bytes
                    let err_code = get_frame_header_advanced(
                        &mut (*zds).fParams,
                        &(&(*zds).headerBuffer)[..(*zds).lhSize],
                        (*zds).format,
                    );
                    if ERR_isError(err_code) != 0 {
                        return err_code;
                    }
                    // remaining header bytes + next block header
                    return std::cmp::max(ZSTD_FRAMEHEADERSIZE_MIN((*zds).format), hSize)
                        .wrapping_sub((*zds).lhSize)
                        .wrapping_add(ZSTD_blockHeaderSize);
                }
                assert!(!ip.is_null());
                libc::memcpy(
                    ((*zds).headerBuffer).as_mut_ptr().add((*zds).lhSize) as *mut core::ffi::c_void,
                    ip as *const core::ffi::c_void,
                    toLoad as libc::size_t,
                );
                (*zds).lhSize = hSize;
                ip = ip.add(toLoad);
                continue;
            } else {
                // check for single-pass mode opportunity
                if (*zds).fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
                    && (*zds).fParams.frameType != ZSTD_skippableFrame
                    && oend.offset_from(op) as size_t as core::ffi::c_ulonglong
                        >= (*zds).fParams.frameContentSize
                {
                    let cSize = ZSTD_findFrameCompressedSize_advanced(
                        core::slice::from_raw_parts(
                            istart.cast(),
                            iend.offset_from(istart) as usize,
                        ),
                        (*zds).format,
                    );
                    if cSize <= iend.offset_from(istart) as size_t {
                        let decompressedSize = ZSTD_decompress_usingDDict(
                            zds,
                            op as *mut core::ffi::c_void,
                            oend.offset_from(op) as size_t,
                            istart as *const core::ffi::c_void,
                            cSize,
                            ZSTD_getDDict(zds),
                        );
                        if ERR_isError(decompressedSize) != 0 {
                            return decompressedSize;
                        }
                        assert!(!istart.is_null());
                        ip = istart.add(cSize);
                        op = if !op.is_null() {
                            op.add(decompressedSize)
                        } else {
                            // can occur if frameContentSize = 0 (empty frame)
                            op
                        };
                        (*zds).expected = 0;
                        (*zds).streamStage = StreamStage::Init;
                        some_more_work = false;
                        continue;
                    }
                }

                // Check output buffer is large enough for ZSTD_odm_stable.
                if (*zds).outBufferMode == BufferMode::Stable
                    && (*zds).fParams.frameType != ZSTD_skippableFrame
                    && (*zds).fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
                    && (oend.offset_from(op) as size_t as core::ffi::c_ulonglong)
                        < (*zds).fParams.frameContentSize
                {
                    return Error::dstSize_tooSmall.to_error_code();
                }
                let err_code = ZSTD_decompressBegin_usingDDict(zds, ZSTD_getDDict(zds));
                if ERR_isError(err_code) != 0 {
                    return err_code;
                }
                if (*zds).format == Format::ZSTD_f_zstd1
                    && MEM_readLE32(((*zds).headerBuffer).as_mut_ptr() as *const core::ffi::c_void)
                        & ZSTD_MAGIC_SKIPPABLE_MASK
                        == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint
                {
                    // skippable frame
                    (*zds).expected = MEM_readLE32(
                        ((*zds).headerBuffer)
                            .as_mut_ptr().add(ZSTD_FRAMEIDSIZE)
                            as *const core::ffi::c_void,
                    ) as size_t;
                    (*zds).stage = DecompressStage::SkipFrame;
                } else {
                    let err_code =
                        ZSTD_decodeFrameHeader(zds, &((&(*zds).headerBuffer)[..(*zds).lhSize]));
                    if ERR_isError(err_code) != 0 {
                        return err_code;
                    }
                    (*zds).expected = ZSTD_blockHeaderSize;
                    (*zds).stage = DecompressStage::DecodeBlockHeader;
                }

                // control buffer memory usage
                (*zds).fParams.windowSize = Ord::max(
                    (*zds).fParams.windowSize,
                    (1 << ZSTD_WINDOWLOG_ABSOLUTEMIN) as core::ffi::c_ulonglong,
                );
                if (*zds).fParams.windowSize > (*zds).maxWindowSize as core::ffi::c_ulonglong {
                    return Error::frameParameter_windowTooLarge.to_error_code();
                }
                if (*zds).maxBlockSizeParam != 0 {
                    (*zds).fParams.blockSizeMax = std::cmp::min(
                        (*zds).fParams.blockSizeMax,
                        (*zds).maxBlockSizeParam as core::ffi::c_uint,
                    );
                }

                // Adapt buffer sizes to frame header instructions
                let neededInBuffSize = std::cmp::max((*zds).fParams.blockSizeMax, 4) as size_t;
                let neededOutBuffSize = if (*zds).outBufferMode == BufferMode::Buffered {
                    ZSTD_decodingBufferSize_internal(
                        (*zds).fParams.windowSize,
                        (*zds).fParams.frameContentSize,
                        (*zds).fParams.blockSizeMax as size_t,
                    )
                } else {
                    0
                };

                ZSTD_DCtx_updateOversizedDuration(zds, neededInBuffSize, neededOutBuffSize);

                let tooSmall = ((*zds).inBuffSize < neededInBuffSize
                    || (*zds).outBuffSize < neededOutBuffSize)
                    as core::ffi::c_int;
                let tooLarge = ZSTD_DCtx_isOversizedTooLong(zds);

                if tooSmall != 0 || tooLarge != 0 {
                    let bufferSize = neededInBuffSize.wrapping_add(neededOutBuffSize);
                    if (*zds).staticSize != 0 {
                        // static DCtx
                        assert!((*zds).staticSize >= size_of::<ZSTD_DCtx>()); // controlled at init
                        if bufferSize > (*zds).staticSize - size_of::<ZSTD_DCtx>() {
                            return Error::dictionary_corrupted.to_error_code();
                        }
                    } else {
                        ZSTD_customFree((*zds).inBuff as *mut core::ffi::c_void, (*zds).customMem);
                        (*zds).inBuffSize = 0;
                        (*zds).outBuffSize = 0;
                        (*zds).inBuff = ZSTD_customMalloc(bufferSize, (*zds).customMem)
                            as *mut core::ffi::c_char;
                        if (*zds).inBuff.is_null() {
                            return Error::dictionary_corrupted.to_error_code();
                        }
                    }
                    (*zds).inBuffSize = neededInBuffSize;
                    (*zds).outBuff = ((*zds).inBuff).add((*zds).inBuffSize);
                    (*zds).outBuffSize = neededOutBuffSize;
                }
                (*zds).streamStage = StreamStage::Read;
                current_block = Block::Read;
            }
        }

        if current_block == Block::Read {
            drop(current_block);

            let neededInSize =
                ZSTD_nextSrcSizeToDecompressWithInputSize(zds, iend.offset_from(ip) as size_t);
            if neededInSize == 0 {
                (*zds).streamStage = StreamStage::Init;
                some_more_work = false;
                continue;
            } else if iend.offset_from(ip) as size_t >= neededInSize {
                // decode directly from src
                let err_code_4 = ZSTD_decompressContinueStream(
                    zds,
                    &mut op,
                    oend,
                    ip as *const core::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(err_code_4) != 0 {
                    return err_code_4;
                }
                assert!(!ip.is_null());
                ip = ip.add(neededInSize);
                // Function modifies the stage so we must break
                continue;
            } else if ip == iend {
                // no more input
                some_more_work = false;
                continue;
            } else {
                (*zds).streamStage = StreamStage::Load;
                current_block = Block::Load;
            }
        }

        if current_block == Block::Load {
            drop(current_block);

            let neededInSize = ZSTD_nextSrcSizeToDecompress(zds);
            let toLoad_0 = neededInSize.wrapping_sub((*zds).inPos);
            let isSkipFrame = ZSTD_isSkipFrame(zds);
            let mut loadedSize: size_t = 0;
            // At this point we shouldn't be decompressing a block that we can stream.
            assert!(
                neededInSize
                    == ZSTD_nextSrcSizeToDecompressWithInputSize(
                        zds,
                        iend.offset_from(ip) as usize
                    )
            );
            if isSkipFrame != 0 {
                loadedSize = std::cmp::min(toLoad_0, iend.offset_from(ip) as size_t);
            } else {
                if toLoad_0 > ((*zds).inBuffSize).wrapping_sub((*zds).inPos) {
                    return Error::corruption_detected.to_error_code();
                }
                loadedSize = ZSTD_limitCopy(
                    ((*zds).inBuff).add((*zds).inPos) as *mut core::ffi::c_void,
                    toLoad_0,
                    ip as *const core::ffi::c_void,
                    iend.offset_from(ip) as size_t,
                );
            }
            if loadedSize != 0 {
                // ip may be NULL
                ip = ip.add(loadedSize);
                (*zds).inPos = ((*zds).inPos).wrapping_add(loadedSize);
            }
            if loadedSize < toLoad_0 {
                // not enough input, wait for more
                some_more_work = false;
            } else {
                // decode loaded input
                (*zds).inPos = 0; // input is consumed
                let err_code_5 = ZSTD_decompressContinueStream(
                    zds,
                    &mut op,
                    oend,
                    (*zds).inBuff as *const core::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(err_code_5) != 0 {
                    return err_code_5;
                }
                // Function modifies the stage so we must break
            }
        }
    }

    // result
    input.pos = ip.offset_from(input.src as *const core::ffi::c_char) as size_t;
    output.pos = op.offset_from(output.dst as *mut core::ffi::c_char) as size_t;

    // Update the expected output buffer for ZSTD_obm_stable.
    (*zds).expectedOutBuffer = *output;

    if ip == istart && op == ostart {
        // no forward progress
        (*zds).noForwardProgress += 1;
        (*zds).noForwardProgress;
        if (*zds).noForwardProgress >= ZSTD_NO_FORWARD_PROGRESS_MAX {
            if op == oend {
                return Error::noForwardProgress_destFull.to_error_code();
            }
            if ip == iend {
                return Error::noForwardProgress_inputEmpty.to_error_code();
            }
            unreachable!();
        }
    } else {
        (*zds).noForwardProgress = 0;
    }

    let mut nextSrcSizeHint = ZSTD_nextSrcSizeToDecompress(zds);
    if nextSrcSizeHint == 0 {
        // frame fully decoded
        if (*zds).outEnd == (*zds).outStart {
            // output fully flushed
            if (*zds).hostageByte != 0 {
                if input.pos >= input.size {
                    // can't release hostage (not present)
                    (*zds).streamStage = StreamStage::Read;
                    return 1;
                }
                // release hostage
                input.pos = (input.pos).wrapping_add(1);
            }
            return 0;
        }
        if (*zds).hostageByte == 0 {
            // output not fully flushed; keep last byte as hostage; will be
            // released when all output is flushed
            // note : pos > 0, otherwise, impossible to finish reading last block
            input.pos = (input.pos).wrapping_sub(1);
            (*zds).hostageByte = 1;
        }
        return 1;
    }
    // preload header of next block
    nextSrcSizeHint = nextSrcSizeHint.wrapping_add(
        ZSTD_blockHeaderSize
            * ((*zds).stage.to_next_input_type() == NextInputType::Block) as size_t,
    );
    assert!((*zds).inPos <= nextSrcSizeHint);
    // part already loaded
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub((*zds).inPos);
    nextSrcSizeHint
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressStream_simpleArgs))]
pub unsafe extern "C" fn ZSTD_decompressStream_simpleArgs(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    dstPos: *mut size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    srcPos: *mut size_t,
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
    let cErr = ZSTD_decompressStream(dctx, &mut output, &mut input);
    *dstPos = output.pos;
    *srcPos = input.pos;
    cErr
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    #[test]
    fn decompress_bound_null() {
        assert_eq!(unsafe { ZSTD_decompressBound(core::ptr::null(), 0) }, 0);
    }

    quickcheck! {
        #[cfg(not(miri))]
        fn decompress_bound_quickcheck(input: Vec<u8>) -> bool {
            unsafe {
                let expected = zstd_sys::ZSTD_decompressBound(input.as_ptr().cast(), input.len() );
                let actual = super::ZSTD_decompressBound(input.as_ptr().cast(), input.len());

                assert_eq!(expected, actual);
                expected == actual
            }
        }
    }

    #[test]
    fn decompression_margin_null() {
        assert_eq!(unsafe { ZSTD_decompressionMargin(core::ptr::null(), 0) }, 0);
    }

    quickcheck! {
        #[cfg(not(miri))]
        fn decompression_margin_quickcheck(input: Vec<u8>) -> bool {
            unsafe {
                let expected = zstd_sys::ZSTD_decompressionMargin(input.as_ptr().cast(), input.len() );
                let actual = super::ZSTD_decompressionMargin(input.as_ptr().cast(), input.len());

                assert_eq!(expected, actual);
                expected == actual
            }
        }
    }
}
