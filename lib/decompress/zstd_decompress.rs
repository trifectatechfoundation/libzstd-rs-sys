use core::ffi::c_char;
use core::mem::MaybeUninit;
use core::ptr::{self, NonNull};

use libc::size_t;

use crate::lib::common::allocations::{ZSTD_customCalloc, ZSTD_customFree, ZSTD_customMalloc};
use crate::lib::common::entropy_common::FSE_readNCount_slice;
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::MEM_readLE32;
use crate::lib::common::reader::Reader;
use crate::lib::common::xxhash::{
    ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_slice, ZSTD_XXH64_update_slice,
};
use crate::lib::common::zstd_internal::{
    repStartValue, LL_bits, ML_bits, MaxLL, MaxML, MaxOff, ZSTD_blockHeaderSize,
    ZSTD_cpuSupportsBmi2, ZSTD_limitCopy, WILDCOPY_OVERLENGTH, ZSTD_FRAMEIDSIZE,
    ZSTD_WORKSPACETOOLARGE_FACTOR, ZSTD_WORKSPACETOOLARGE_MAXDURATION,
};
use crate::lib::decompress::huf_decompress::{
    DTableDesc, HUF_ReadDTableX2_Workspace, HUF_readDTableX2_wksp, Writer,
};
use crate::lib::decompress::zstd_ddict::{MultipleDDicts, ZSTD_DDict, ZSTD_DDictHashSet};
use crate::lib::decompress::zstd_decompress_block::{
    getc_block_size, ZSTD_buildFSETable, ZSTD_checkContinuity, ZSTD_decompressBlock_internal,
    ZSTD_getcBlockSize,
};
use crate::lib::decompress::{
    blockProperties_t, BlockType, DecompressStage, DictUses, LL_base, ML_base, NextInputType,
    OF_base, OF_bits, StreamStage, ZSTD_DCtx, ZSTD_DCtx_s, ZSTD_FrameHeader, ZSTD_entropyDTables_t,
    ZSTD_frame, ZSTD_skippableFrame,
};
use crate::lib::zstd::experimental::ZSTD_FRAMEHEADERSIZE_MIN;
use crate::lib::zstd::*;

use crate::lib::common::zstd_trace::{
    ZSTD_Trace, ZSTD_trace_decompress_begin, ZSTD_trace_decompress_end,
};

use crate::lib::legacy::zstd_v05::{
    ZBUFFv05_DCtx, ZBUFFv05_createDCtx, ZBUFFv05_decompressContinue,
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
type XXH_errorcode = core::ffi::c_uint;
pub const XXH_ERROR: XXH_errorcode = 1;
pub const XXH_OK: XXH_errorcode = 0;
pub type streaming_operation = core::ffi::c_uint;
pub const is_streaming: streaming_operation = 1;
pub const not_streaming: streaming_operation = 0;

#[derive(Default)]
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
pub const ZSTD_BLOCKSIZE_MAX: core::ffi::c_int = 1 << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: core::ffi::c_ulonglong =
    (0 as core::ffi::c_ulonglong).wrapping_sub(1);
pub const ZSTD_CONTENTSIZE_ERROR: core::ffi::c_ulonglong =
    (0 as core::ffi::c_ulonglong).wrapping_sub(2);
pub const ZSTD_SKIPPABLEHEADERSIZE: core::ffi::c_int = 8;
pub const ZSTD_BLOCKSIZE_MAX_MIN: core::ffi::c_int = (1) << 10;
pub const ZSTD_WINDOWLOG_LIMIT_DEFAULT: core::ffi::c_int = 27;
pub const ZSTD_WINDOWLOG_ABSOLUTEMIN: core::ffi::c_int = 10;

pub const ZSTDv01_magicNumberLE: u32 = 0x1EB52FFD;

pub const ZSTDv02_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB522;
pub const ZSTDv03_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB523;
pub const ZSTDv04_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB524;
pub const ZSTDv05_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB525;
pub const ZSTDv06_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB526;
pub const ZSTDv07_MAGICNUMBER: core::ffi::c_uint = 0xFD2FB527;

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
    let ptr = src.as_ptr().cast::<core::ffi::c_void>();

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

            match ZSTDv05_getFrameParams(&mut fParams, src) {
                Ok(0) => Some(fParams.srcSize as core::ffi::c_ulonglong),
                _ => None,
            }
        }
        6 => {
            let mut fParams = ZSTDv06_frameParams_s {
                frameContentSize: 0,
                windowLog: 0,
            };

            match unsafe { ZSTDv06_getFrameParams(&mut fParams, ptr, src.len() as _) } {
                0 => Some(fParams.frameContentSize),
                _ => None,
            }
        }
        7 => {
            let mut fParams = ZSTDv07_frameParams::default();
            match ZSTDv07_getFrameParams(&mut fParams, src) {
                Ok(0) => Some(fParams.frameContentSize),
                _ => None,
            }
        }

        _ => None,
    }
}

#[inline]
unsafe fn ZSTD_decompressLegacy(
    mut dst: Writer<'_>,
    src: Reader<'_>,
    dict: &[u8],
) -> Result<size_t, Error> {
    let version = is_legacy(src.as_slice());
    let dstCapacity = dst.capacity();

    let mut x: core::ffi::c_char = 0;

    let dst = if dst.is_null() {
        &raw mut x as *mut core::ffi::c_void
    } else {
        dst.as_mut_ptr() as *mut core::ffi::c_void
    };

    match version {
        5 => {
            let zd = ZSTDv05_createDCtx();
            if zd.is_null() {
                return Err(Error::memory_allocation);
            }
            let result = ZSTDv05_decompress_usingDict(&mut *zd, dst, dstCapacity, src, dict);
            ZSTDv05_freeDCtx(zd);
            result
        }
        6 => {
            let compressedSize = src.len();
            let src = if src.is_null() {
                &raw mut x as *const core::ffi::c_void
            } else {
                src.as_ptr() as *const core::ffi::c_void
            };

            let mut result: size_t = 0;
            let zd = ZSTDv06_createDCtx();
            if zd.is_null() {
                return Err(Error::memory_allocation);
            }
            result = ZSTDv06_decompress_usingDict(
                zd,
                dst,
                dstCapacity,
                src,
                compressedSize,
                dict.as_ptr().cast(),
                dict.len(),
            );
            ZSTDv06_freeDCtx(zd);
            // TODO: make `ZSTDv06_decompress_usingDict` return a Result
            Error::from_error_code(result).map_or(Ok(result), Err)
        }
        7 => {
            let compressedSize = src.len();
            let src = if src.is_null() {
                &raw mut x as *const core::ffi::c_void
            } else {
                src.as_ptr() as *const core::ffi::c_void
            };

            let zd = ZSTDv07_createDCtx();
            if zd.is_null() {
                return Err(Error::memory_allocation);
            }
            let result = ZSTDv07_decompress_usingDict(
                &mut *zd,
                dst,
                dstCapacity,
                src,
                compressedSize,
                dict.as_ptr().cast(),
                dict.len(),
            );
            ZSTDv07_freeDCtx(zd);
            result
        }
        _ => Err(Error::prefix_unknown),
    }
}

// FIXME: this should be totally safe at this point.
unsafe fn find_frame_size_info_legacy(src: &[u8]) -> Result<ZSTD_frameSizeInfo, Error> {
    let mut frameSizeInfo = ZSTD_frameSizeInfo::default();

    match is_legacy(src) {
        5 => {
            ZSTDv05_findFrameSizeInfoLegacy(
                src,
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
            return Err(Error::prefix_unknown);
        }
    }

    if let Some(err) = Error::from_error_code(frameSizeInfo.compressedSize) {
        return Err(err);
    }

    if frameSizeInfo.compressedSize > src.len() {
        return Err(Error::srcSize_wrong);
    }

    if frameSizeInfo.decompressedBound != ZSTD_CONTENTSIZE_ERROR {
        frameSizeInfo.nbBlocks = (frameSizeInfo.decompressedBound)
            .wrapping_div(ZSTD_BLOCKSIZE_MAX as core::ffi::c_ulonglong)
            as size_t;
    }

    Ok(frameSizeInfo)
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
        _ => Error::version_unsupported.to_error_code(),
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
            let _ = ZBUFFv05_decompressInitDictionary(
                dctx,
                if dictSize != 0 {
                    core::slice::from_raw_parts(dict.cast(), dictSize)
                } else {
                    &[]
                },
            );
            *legacyContext = dctx as *mut core::ffi::c_void;
            0
        }
        6 => {
            let dctx = if prevVersion != newVersion {
                ZBUFFv06_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv06_DCtx
            };
            if dctx.is_null() {
                return Error::memory_allocation.to_error_code();
            }
            ZBUFFv06_decompressInitDictionary(dctx, dict, dictSize);
            *legacyContext = dctx as *mut core::ffi::c_void;
            0
        }
        7 => {
            let dctx = if prevVersion != newVersion {
                ZBUFFv07_createDCtx()
            } else {
                *legacyContext as *mut ZBUFFv07_DCtx
            };
            if dctx.is_null() {
                return Error::memory_allocation.to_error_code();
            }
            let _ = ZBUFFv07_decompressInitDictionary(&mut *dctx, dict, dictSize);
            *legacyContext = dctx as *mut core::ffi::c_void;
            0
        }
        _ => 0,
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
            let hintSize = ZBUFFv05_decompressContinue(
                &mut *dctx,
                dst,
                &mut decodedSize,
                src.cast(),
                &mut readSize,
            );
            output.pos = (output.pos).wrapping_add(decodedSize);
            input.pos = (input.pos).wrapping_add(readSize);
            hintSize.unwrap_or_else(|err| err.to_error_code())
        }
        6 => {
            let dctx = legacyContext as *mut ZBUFFv06_DCtx;
            let src =
                (input.src as *const core::ffi::c_char).add(input.pos) as *const core::ffi::c_void;
            let mut readSize = (input.size).wrapping_sub(input.pos);
            let dst =
                (output.dst as *mut core::ffi::c_char).add(output.pos) as *mut core::ffi::c_void;
            let mut decodedSize = (output.size).wrapping_sub(output.pos);
            let hintSize =
                ZBUFFv06_decompressContinue(dctx, dst, &mut decodedSize, src, &mut readSize);
            output.pos = (output.pos).wrapping_add(decodedSize);
            input.pos = (input.pos).wrapping_add(readSize);
            hintSize
        }
        7 => {
            let dctx = legacyContext as *mut ZBUFFv07_DCtx;
            let src =
                (input.src as *const core::ffi::c_char).add(input.pos) as *const core::ffi::c_void;
            let mut readSize = (input.size).wrapping_sub(input.pos);
            let dst =
                (output.dst as *mut core::ffi::c_char).add(output.pos) as *mut core::ffi::c_void;
            let mut decodedSize = (output.size).wrapping_sub(output.pos);
            let hintSize =
                ZBUFFv07_decompressContinue(&mut *dctx, dst, &mut decodedSize, src, &mut readSize);
            output.pos = (output.pos).wrapping_add(decodedSize);
            input.pos = (input.pos).wrapping_add(readSize);
            hintSize.unwrap_or_else(|err| err.to_error_code())
        }
        _ => Error::version_unsupported.to_error_code(),
    }
}

// These two constants represent SIZE_MULT/COUNT_MULT load factor without using a float.
// Currently, that means a 0.75 load factor.
// So, if count * COUNT_MULT / size * SIZE_MULT != 0, then we've exceeded the load factor of the ddict hash set.
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_COUNT_MULT: core::ffi::c_int = 4;
pub const DDICT_HASHSET_MAX_LOAD_FACTOR_SIZE_MULT: core::ffi::c_int = 3;

pub const DDICT_HASHSET_TABLE_BASE_SIZE: core::ffi::c_int = 64;
pub const DDICT_HASHSET_RESIZE_FACTOR: core::ffi::c_int = 2;

/// Hash function to determine starting position of dict insertion within the table
///
/// # Returns
///
/// - an index between `0..hashSet.ddictPtrTableSize`
fn ZSTD_DDictHashSet_getIndex(hashSet: &ZSTD_DDictHashSet, dictID: u32) -> size_t {
    let hash = ZSTD_XXH64_slice(&dictID.to_ne_bytes(), 0);
    // `ddictPtrTableSize` is a multiple of 2, use `size - 1` as a mask to get an index within `0..hashSet.ddictPtrTableSize`
    hash as size_t & (hashSet.ddictPtrTableSize).wrapping_sub(1)
}

/// Adds [`ZSTD_DDict`] to a hashset without resizing it.
///
/// If the [`ZSTD_DDict`]'s `dictID` already exists in the set, it replaces the one in the set.
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
    0
}

/// Expands hash table by factor of [`DDICT_HASHSET_RESIZE_FACTOR`] and rehashes all values,
/// allocates the new table, and frees the old table.
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
            if ERR_isError(err_code) {
                return err_code;
            }
        }
        i = i.wrapping_add(1);
    }
    ZSTD_customFree(
        oldTable as *mut core::ffi::c_void,
        oldTableSize.wrapping_mul(::core::mem::size_of::<*mut ZSTD_DDict>()),
        customMem,
    );
    0
}

/// Fetches a [`ZSTD_DDict`] with the given `dictID`.
///
/// # Returns
///
/// - the [`ZSTD_DDict`] with the requested `dictID`
/// - `NULL` if no such dictionary exists
unsafe fn ZSTD_DDictHashSet_getDDict(
    hashSet: &mut ZSTD_DDictHashSet,
    dictID: u32,
) -> *const ZSTD_DDict {
    let mut idx = ZSTD_DDictHashSet_getIndex(hashSet, dictID);
    let idxRangeMask = hashSet.ddictPtrTableSize - 1;
    loop {
        let currDictID = match hashSet.as_slice()[idx].as_ref() {
            Some(ddict) => ddict.dictID as size_t,
            None => 0,
        };

        if currDictID == dictID as size_t || currDictID == 0 {
            // currDictID == 0 implies a NULL ddict entry
            break;
        }

        idx &= idxRangeMask; // loop back to the start of the table when we reach the end
        idx += 1;
    }

    hashSet.as_slice()[idx]
}

/// Allocates space for and returns a ddict hash set.
///
/// # Returns
/// - the [`ZSTD_DDictHashSet`] if allocation succeeds. The hash set's `ZSTD_DDict*` table has all
///   values automatically set to `NULL` to begin with.
/// - `NULL` if allocation failed
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
        ZSTD_customFree(
            ret as *mut core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_DDictHashSet>(),
            customMem,
        );
        return core::ptr::null_mut();
    }
    (*ret).ddictPtrTableSize = DDICT_HASHSET_TABLE_BASE_SIZE as size_t;
    (*ret).ddictPtrCount = 0;
    ret
}

/// Frees the table of `ZSTD_DDict*` within a hashset, then frees the hashset itself.
///
/// Note: The `ZSTD_DDict*` within the table are NOT freed.
unsafe fn ZSTD_freeDDictHashSet(hashSet: *mut ZSTD_DDictHashSet, customMem: ZSTD_customMem) {
    if !hashSet.is_null() && !((*hashSet).ddictPtrTable).is_null() {
        ZSTD_customFree(
            (*hashSet).ddictPtrTable as *mut core::ffi::c_void,
            (*hashSet)
                .ddictPtrTableSize
                .wrapping_mul(::core::mem::size_of::<*mut ZSTD_DDict>()),
            customMem,
        );
    }
    if !hashSet.is_null() {
        ZSTD_customFree(
            hashSet as *mut core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_DDictHashSet>(),
            customMem,
        );
    }
}

/// Adds a [`ZSTD_DDict`] into the [`ZSTD_DDictHashSet`], possibly triggering a resize of the hash set.
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
        if ERR_isError(err_code) {
            return err_code;
        }
    }
    let err_code = ZSTD_DDictHashSet_emplaceDDict(hashSet, ddict);
    if ERR_isError(err_code) {
        return err_code;
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
pub const extern "C" fn ZSTD_estimateDCtxSize() -> size_t {
    size_of::<ZSTD_DCtx>()
}

const fn ZSTD_startingInputLength(format: Format) -> size_t {
    match format {
        Format::ZSTD_f_zstd1 => 5,
        Format::ZSTD_f_zstd1_magicless => 1,
    }
}

fn ZSTD_DCtx_resetParameters(dctx: &mut MaybeUninit<ZSTD_DCtx>) {
    unsafe {
        let dctx = dctx.as_mut_ptr();
        debug_assert_eq!((*dctx).streamStage, StreamStage::Init);
        (*dctx).format = Format::ZSTD_f_zstd1;
        (*dctx).maxWindowSize = ZSTD_MAXWINDOWSIZE_DEFAULT as size_t;
        (*dctx).outBufferMode = BufferMode::Buffered;
        (*dctx).forceIgnoreChecksum = ForceIgnoreChecksum::ValidateChecksum;
        (*dctx).refMultipleDDicts = MultipleDDicts::Single;
        (*dctx).disableHufAsm = false;
        (*dctx).maxBlockSizeParam = 0;
    }
}

fn ZSTD_initDCtx_internal(dctx: &mut MaybeUninit<ZSTD_DCtx>) {
    unsafe {
        let dctx = dctx.as_mut_ptr();

        (*dctx).staticSize = 0;
        (*dctx).ddict = core::ptr::null();
        (*dctx).ddictLocal = core::ptr::null_mut();
        (*dctx).dictEnd = core::ptr::null();
        (*dctx).ddictIsCold = false;
        (*dctx).dictUses = DictUses::ZSTD_dont_use;
        (*dctx).inBuff = core::ptr::null_mut();
        (*dctx).inBuffSize = 0;
        (*dctx).outBuffSize = 0;
        (*dctx).streamStage = StreamStage::Init;
        (*dctx).legacyContext = core::ptr::null_mut();
        (*dctx).previousLegacyVersion = 0;
        (*dctx).noForwardProgress = 0;
        (*dctx).oversizedDuration = 0;
        (*dctx).isFrameDecompression = true;
        (*dctx).bmi2 = ZSTD_cpuSupportsBmi2() as _;
        (*dctx).ddictSet = core::ptr::null_mut();

        // Prevent issues with uninitialized memory.
        (*dctx).headerBuffer = [0u8; 18];
    }

    ZSTD_DCtx_resetParameters(dctx);
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initStaticDCtx))]
pub unsafe extern "C" fn ZSTD_initStaticDCtx(
    workspace: *mut core::ffi::c_void,
    workspaceSize: size_t,
) -> *mut ZSTD_DCtx {
    // workspace should be 8-aligned
    if workspace as size_t & 7 != 0 {
        return core::ptr::null_mut();
    }

    // check minimum workspace size
    if workspaceSize < ::core::mem::size_of::<ZSTD_DCtx>() {
        return core::ptr::null_mut();
    }

    let dctx = workspace.cast::<MaybeUninit<ZSTD_DCtx>>().as_mut().unwrap();
    ZSTD_initDCtx_internal(dctx);

    (*dctx.as_mut_ptr()).staticSize = workspaceSize;
    (*dctx.as_mut_ptr()).inBuff = dctx.as_mut_ptr().add(1).cast::<u8>();
    dctx.as_mut_ptr()
}

unsafe fn ZSTD_createDCtx_internal(customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    let alloc = ZSTD_customMalloc(::core::mem::size_of::<ZSTD_DCtx>(), customMem);
    let Some(dctx) = alloc.cast::<MaybeUninit<ZSTD_DCtx>>().as_mut() else {
        return core::ptr::null_mut();
    };

    (*dctx.as_mut_ptr()).customMem = customMem;
    ZSTD_initDCtx_internal(dctx);
    dctx.as_mut_ptr()
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDCtx_advanced))]
pub unsafe extern "C" fn ZSTD_createDCtx_advanced(customMem: ZSTD_customMem) -> *mut ZSTD_DCtx {
    ZSTD_createDCtx_internal(customMem)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDCtx))]
pub unsafe extern "C" fn ZSTD_createDCtx() -> *mut ZSTD_DCtx {
    ZSTD_createDCtx_internal(ZSTD_customMem::default())
}

unsafe fn ZSTD_clearDict(dctx: *mut ZSTD_DCtx) {
    ZSTD_freeDDict((*dctx).ddictLocal);
    (*dctx).ddictLocal = core::ptr::null_mut();
    (*dctx).ddict = core::ptr::null();
    (*dctx).dictUses = DictUses::ZSTD_dont_use;
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
    ZSTD_customFree(
        (*dctx).inBuff as *mut core::ffi::c_void,
        (*dctx).inBuffSize,
        cMem,
    );
    (*dctx).inBuff = core::ptr::null_mut();
    if !((*dctx).legacyContext).is_null() {
        ZSTD_freeLegacyStreamContext((*dctx).legacyContext, (*dctx).previousLegacyVersion);
    }
    if !((*dctx).ddictSet).is_null() {
        ZSTD_freeDDictHashSet((*dctx).ddictSet, cMem);
        (*dctx).ddictSet = core::ptr::null_mut();
    }
    ZSTD_customFree(
        dctx as *mut core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_DCtx>(),
        cMem,
    );
    0
}

/// No longer useful.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_copyDCtx))]
pub unsafe extern "C" fn ZSTD_copyDCtx(dstDCtx: *mut ZSTD_DCtx, srcDCtx: *const ZSTD_DCtx) {
    core::ptr::copy_nonoverlapping(
        srcDCtx.cast::<u8>(),
        dstDCtx.cast::<u8>(),
        // No need to copy workspace.
        core::mem::offset_of!(ZSTD_DCtx, inBuff),
    );
}

/// Given a `dctx` with a digested frame params, re-selects the correct [`ZSTD_DDict`] based on
/// the requested dict ID from the frame. If there exists a reference to the correct [`ZSTD_DDict`],
/// then accordingly sets the ddict to be used to decompress the frame.
///
/// If no [`ZSTD_DDict`] is found, then no action is taken, and the `ZSTD_DCtx::ddict` remains as-is.
///
/// [`ZSTD_d_refMultipleDDicts`] must be enabled for this function to be called.
fn ZSTD_DCtx_selectFrameDDict(dctx: &mut ZSTD_DCtx) {
    debug_assert_eq!(dctx.refMultipleDDicts, MultipleDDicts::Multiple);
    debug_assert!(!dctx.ddictSet.is_null());
    if !dctx.ddict.is_null() {
        // FIXME: make safe
        let frameDDict = unsafe {
            ZSTD_DDictHashSet_getDDict(dctx.ddictSet.as_mut().unwrap(), dctx.fParams.dictID)
        };
        if !frameDDict.is_null() {
            unsafe { ZSTD_clearDict(dctx) };
            dctx.dictID = dctx.fParams.dictID;
            dctx.ddict = frameDDict;
            dctx.dictUses = DictUses::ZSTD_use_indefinitely;
        }
    }
}

/// Tells if the content of `buffer` starts with a valid Frame Identifier.
///
/// Note: Frame Identifier is 4 bytes. If `size < 4`, it will always return 0.
///
/// Note: Legacy Frame Identifiers are considered valid only if Legacy Support is enabled.
///
/// Note: Skippable Frame Identifiers are considered valid.
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

/// Tells if the content of `buffer` starts with a valid Frame Identifier for a skippable frame.
///
/// Note: Frame Identifier is 4 bytes. If `size < 4`, it will always return 0.
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
    let [a, b, c, d, ..] = *src else {
        return false;
    };

    let magic = u32::from_le_bytes([a, b, c, d]);
    if magic & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START as core::ffi::c_uint {
        return true;
    }

    false
}

fn frame_header_size_internal(src: &[u8], format: Format) -> Result<usize, Error> {
    static ZSTD_fcs_fieldSize: [u8; 4] = [0, 2, 4, 8];
    static ZSTD_did_fieldSize: [u8; 4] = [0, 1, 2, 4];

    let minInputSize = ZSTD_startingInputLength(format);
    let Some([.., fhd]) = src.get(..minInputSize as usize) else {
        return Err(Error::srcSize_wrong);
    };

    let dictID = fhd & 0b11;
    let singleSegment = (fhd >> 5 & 1) != 0;
    let fcsId = fhd >> 6;

    Ok(minInputSize
        + usize::from(!singleSegment)
        + usize::from(ZSTD_did_fieldSize[usize::from(dictID)])
        + usize::from(ZSTD_fcs_fieldSize[usize::from(fcsId)])
        + usize::from(singleSegment && fcsId == 0))
}

/// Get the frame header size
///
/// `srcSize` must be >= [`ZSTD_frameHeaderSize_prefix`]
///
/// # Returns
///
/// - the size of the Frame Header on success
/// - an error code (if `srcSize` is too small), which can be tested with [`ZSTD_isError`]
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

    frame_header_size_internal(src, Format::ZSTD_f_zstd1).unwrap_or_else(Error::to_error_code)
}

/// Decode Frame Header, or require larger `srcSize`.
///
/// # Returns
///
/// - 0 if `zfhPtr` is correctly filled
/// - greater than 0 if `srcSize` is too small, the value is the wanted `srcSize` amount
/// - or an error code, which can be tested with [`ZSTD_isError`]
///
/// Note: this function does not consume input, it only reads it.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getFrameHeader))]
pub unsafe extern "C" fn ZSTD_getFrameHeader(
    zfhPtr: *mut ZSTD_FrameHeader,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_getFrameHeader_advanced(zfhPtr, src, srcSize, ZSTD_format_e::ZSTD_f_zstd1)
}

fn get_frame_header(zfhPtr: &mut ZSTD_FrameHeader, src: &[u8]) -> Result<size_t, Error> {
    get_frame_header_advanced(zfhPtr, src, Format::ZSTD_f_zstd1)
}

/// Decode Frame Header, or require larger `srcSize`.
///
/// # Returns
///
/// - 0 if `zfhPtr` is correctly filled
/// - greater than 0 if `srcSize` is too small, the value is the wanted `srcSize` amount
/// - or an error code, which can be tested with [`ZSTD_isError`]
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
    .unwrap_or_else(Error::to_error_code)
}

fn get_frame_header_advanced(
    zfhPtr: &mut ZSTD_FrameHeader,
    src: &[u8],
    format: Format,
) -> Result<size_t, Error> {
    let minInputSize = ZSTD_startingInputLength(format);
    if src.len() < minInputSize as usize {
        // error out early if magic number is invalid
        if !src.is_empty()
            && format != Format::ZSTD_f_zstd1_magicless
            && src != &ZSTD_MAGICNUMBER.to_le_bytes()[..src.len()]
        {
            // not a zstd frame, let's check if it's a skippable frame
            let mut hbuf = ZSTD_MAGIC_SKIPPABLE_START.to_le_bytes();
            hbuf[..src.len()].copy_from_slice(src);
            if !is_skippable_frame(&hbuf) {
                return Err(Error::prefix_unknown);
            }
        }
        return Ok(minInputSize);
    }

    if format != Format::ZSTD_f_zstd1_magicless
        && u32::from_le_bytes(*src.first_chunk().unwrap()) != ZSTD_MAGICNUMBER
    {
        if is_skippable_frame(src) {
            if src.len() < ZSTD_SKIPPABLEHEADERSIZE as usize {
                return Ok(ZSTD_SKIPPABLEHEADERSIZE as size_t);
            }

            let first_word = u32::from_le_bytes(*src.first_chunk().unwrap());
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

            return Ok(0);
        }
        return Err(Error::prefix_unknown);
    }

    // ensure there is enough `src` to fully read/decode frame header
    let fhsize = frame_header_size_internal(src, format)?;
    if src.len() < fhsize {
        return Ok(fhsize);
    }

    let fhdByte = src[minInputSize as usize - 1];
    let dictIDSizeCode = fhdByte & 0b11;
    let checksumFlag = u32::from(fhdByte) >> 2 & 1;
    let singleSegment = (u32::from(fhdByte) >> 5 & 1) != 0;
    let fcsID = (u32::from(fhdByte) >> 6) as u32;

    let mut windowSize = 0;

    if fhdByte & 0x8 != 0 {
        return Err(Error::frameParameter_unsupported);
    }

    let mut pos = minInputSize as usize;
    if !singleSegment {
        let wlByte = src[pos];
        pos += 1;
        let windowLog = ((i32::from(wlByte) / 8) + ZSTD_WINDOWLOG_ABSOLUTEMIN) as u32;

        if windowLog > (if size_of::<usize>() == 4 { 30 } else { 31 }) as u32 {
            return Err(Error::frameParameter_windowTooLarge);
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
        blockSizeMax: Ord::min(windowSize, ZSTD_BLOCKSIZE_MAX as u64) as u32,
        frameType: ZSTD_frame,
        headerSize: fhsize as u32,
        dictID,
        checksumFlag,
        _reserved1: 0,
        _reserved2: 0,
    };

    Ok(0)
}

/// Get frame content size
///
/// # Returns
///
/// - the decompressed size of the single frame pointed to by `src` if known
/// - [`ZSTD_CONTENTSIZE_UNKNOWN`] if the size cannot be determined
/// - [`ZSTD_CONTENTSIZE_ERROR`] if an error occurred (e.g. invalid magic number, `srcSize` too small)
///
/// Compatible with legacy mode
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
    let Ok(0) = get_frame_header_advanced(&mut zfh, src, Format::ZSTD_f_zstd1) else {
        return ZSTD_CONTENTSIZE_ERROR;
    };

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
    .unwrap_or_else(Error::to_error_code)
}

fn read_skippable_frame_size(src: &[u8]) -> Result<size_t, Error> {
    let skippableHeaderSize = ZSTD_SKIPPABLEHEADERSIZE as u32;

    let [_, _, _, _, a, b, c, d, ..] = *src else {
        return Err(Error::srcSize_wrong);
    };

    let size = u32::from_le_bytes([a, b, c, d]);

    let skippableSize = skippableHeaderSize
        .checked_add(size)
        .ok_or(Error::frameParameter_unsupported)? as usize;

    if skippableSize > src.len() {
        return Err(Error::srcSize_wrong);
    }

    Ok(skippableSize)
}

/// Retrieves content of a skippable frame, and writes it to `dst` buffer.
///
/// The parameter `magicVariant` will receive the `magicVariant` that was supplied when the frame was written,
/// i.e. `magicNumber` - [`ZSTD_MAGIC_SKIPPABLE_START`].  This can be NULL if the caller is not interested
/// in the `magicVariant`.
///
/// # Returns
///
/// - the number of bytes written
/// - an error if destination buffer is not large enough or if this is not a valid skippable frame, which can
///   be tested with [`ZSTD_isError`]
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

    // check input validity
    if ZSTD_isSkippableFrame(src, srcSize) == 0 {
        return Error::frameParameter_unsupported.to_error_code();
    }
    if skippableFrameSize < 8 || skippableFrameSize > srcSize {
        return Error::srcSize_wrong.to_error_code();
    }
    if skippableContentSize > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }

    // deliver payload
    if skippableContentSize > 0 && !dst.is_null() {
        core::ptr::copy_nonoverlapping(
            src.cast::<u8>().add(8),
            dst.cast::<u8>(),
            skippableContentSize,
        );
    }
    if !magicVariant.is_null() {
        *magicVariant = magicNumber.wrapping_sub(ZSTD_MAGIC_SKIPPABLE_START as u32);
    }
    skippableContentSize
}

/// Find decompressed size, compatible with legacy mode
///
/// # Returns
///
/// - the decompressed size of the frames contained in `src`
/// - an [`ZSTD_CONTENTSIZE_ERROR`]
///
/// `srcSize` must be the exact length of some number of ZSTD compressed and/or skippable frames
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

    while src.len() >= ZSTD_startingInputLength(Format::ZSTD_f_zstd1) {
        if is_skippable_frame(src) {
            let skippableSize = match read_skippable_frame_size(src) {
                Ok(size) => size,
                Err(_) => return ZSTD_CONTENTSIZE_ERROR,
            };
            src = &src[skippableSize..];
        } else {
            let fcs = get_frame_content_size(src);
            if fcs >= ZSTD_CONTENTSIZE_ERROR {
                return fcs;
            }
            // check for overflow
            match totalDstSize.checked_add(fcs) {
                None => return ZSTD_CONTENTSIZE_ERROR,
                Some(size) => totalDstSize = size,
            };

            // skip to next frame
            let Ok(frameSrcSize) = ZSTD_findFrameCompressedSize_advanced(src, Format::ZSTD_f_zstd1)
            else {
                return ZSTD_CONTENTSIZE_ERROR;
            };
            src = &src[frameSrcSize..];
        }
    }

    if !src.is_empty() {
        return ZSTD_CONTENTSIZE_ERROR;
    }

    totalDstSize
}

/// Get decompressed size, compatible with legacy mode
///
/// # Returns
///
/// - decompressed size if known
/// - 0 otherwise, which means that either:
///   - the frame content is empty
///   - the decompressed size field is not present in frame header
///   - the frame header is unknown or not supported
///   - or the frame header is not complete (`srcSize` is too small)
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

/// Decode frame header.
///
/// If multiple [`ZSTD_DDict`] references are enabled, it will choose the correct [`ZSTD_DDict`] to use.
///
/// `headerSize` must be the size provided by [`ZSTD_frameHeaderSize`]
fn ZSTD_decodeFrameHeader(dctx: &mut ZSTD_DCtx, src: &[u8]) -> Result<size_t, Error> {
    let result = get_frame_header_advanced(&mut dctx.fParams, src, dctx.format)?;
    if result > 0 {
        return Err(Error::srcSize_wrong);
    }

    // reference `DDict` requested by frame if dctx references multiple `DDict`s
    if dctx.refMultipleDDicts == MultipleDDicts::Multiple && !(dctx.ddictSet).is_null() {
        ZSTD_DCtx_selectFrameDDict(dctx);
    }

    if dctx.fParams.dictID != 0 && dctx.dictID != dctx.fParams.dictID {
        return Err(Error::dictionary_wrong);
    }

    dctx.validateChecksum = dctx.fParams.checksumFlag != 0
        && matches!(
            dctx.forceIgnoreChecksum,
            ForceIgnoreChecksum::ValidateChecksum
        );
    if dctx.validateChecksum {
        ZSTD_XXH64_reset(&mut dctx.xxhState, 0);
    }
    dctx.processedCSize = (dctx.processedCSize as size_t).wrapping_add(src.len()) as u64;
    Ok(0)
}

fn find_frame_size_info(src: &[u8], format: Format) -> Result<ZSTD_frameSizeInfo, Error> {
    let mut frameSizeInfo = ZSTD_frameSizeInfo::default();

    if format == Format::ZSTD_f_zstd1 && is_legacy(src) != 0 {
        return unsafe { find_frame_size_info_legacy(src) };
    }

    if format == Format::ZSTD_f_zstd1
        && src.len() >= ZSTD_SKIPPABLEHEADERSIZE as usize
        && is_skippable_frame(src)
    {
        frameSizeInfo.compressedSize = read_skippable_frame_size(src)?;
        debug_assert!(frameSizeInfo.compressedSize <= src.len());
        Ok(frameSizeInfo)
    } else {
        let mut ip = 0;
        let mut remainingSize = src.len();
        let mut nbBlocks = 0usize;
        let mut zfh = ZSTD_FrameHeader::default();

        // extract Frame Header
        let ret = get_frame_header_advanced(&mut zfh, src, format)?;
        if ret > 0 {
            return Err(Error::srcSize_wrong);
        }

        ip += zfh.headerSize as usize;
        remainingSize = remainingSize.wrapping_sub(zfh.headerSize as size_t);

        // iterate over each block
        loop {
            let mut blockProperties = blockProperties_t::default();
            let cBlockSize = ZSTD_getcBlockSize(&src[ip..], &mut blockProperties)?;
            if ZSTD_blockHeaderSize.wrapping_add(cBlockSize) > remainingSize {
                return Err(Error::srcSize_wrong);
            }

            ip += ZSTD_blockHeaderSize.wrapping_add(cBlockSize) as usize;
            remainingSize =
                remainingSize.wrapping_sub(ZSTD_blockHeaderSize.wrapping_add(cBlockSize));
            nbBlocks = nbBlocks.wrapping_add(1);

            if blockProperties.lastBlock {
                break;
            }
        }

        // final frame content checksum
        if zfh.checksumFlag != 0 {
            if remainingSize < 4 {
                return Err(Error::srcSize_wrong);
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
        Ok(frameSizeInfo)
    }
}

fn ZSTD_findFrameCompressedSize_advanced(src: &[u8], format: Format) -> Result<size_t, Error> {
    Ok(find_frame_size_info(src, format)?.compressedSize)
}

/// Find frame compressed size, compatible with legacy mode
///
/// Note 1: this method is called `_find*()` because it's not enough to read the header, it may have
/// to scan through the frame's content to reach its end.
///
/// Note 2: this method also works with Skippable Frames, in which case it returns the size of the
/// complete skippable frame, which is always equal to its content size + 8 bytes for headers.
///
/// # Returns
///
/// - the compressed size of the first frame starting at `src`, suitable to pass as `srcSize`
///   to [`ZSTD_decompress`] or similar
/// - an error code if input is invalid, which can be tested with [`ZSTD_isError`], which can happen
///   if `src` does not point to the start of a ZSTD frame or skippable frame or if `srcSize` is
///   less than the first frame size
///
/// # Safety
///
/// The caller must guarantee that
///
/// * Either
///     - `src` is `NULL`
///     - `src` and `srcSize` satisfy the requirements of [`core::slice::from_raw_parts`]
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
        .unwrap_or_else(|err| err.to_error_code())
}

/// Get an upper-bound on the decompressed size
///
/// - `src` should point to the start of a series of ZSTD encoded and/or skippable frames
/// - `srcSize` must be the _exact_ size of this series (i.e. there should be a frame boundary at
///   `src + srcSize`)
///
/// # Returns
///
/// - an upper-bound on the decompressed size of all data in all successive frames
/// - `ZSTD_CONTENTSIZE_ERROR`, which can occur if `src` contains an invalid or incorrectly formatted frame
///
/// Note 1: the upper-bound is exact when the decompressed size field is available in every ZSTD
///         encoded frame of `src`. In this case, [`ZSTD_findDecompressedSize`] and
///         [`ZSTD_decompressBound`] return the same value.
///
/// Note 2: when the decompressed size field isn't available, the upper-bound for that frame is
///         calculated by: `upper-bound = #blocks * min(128 KB, Window_Size)`
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

    // iterate over each frame
    while !src.is_empty() {
        let Ok(frameSizeInfo) = find_frame_size_info(src, Format::ZSTD_f_zstd1) else {
            return ZSTD_CONTENTSIZE_ERROR;
        };
        src = &src[frameSizeInfo.compressedSize..];
        bound += frameSizeInfo.decompressedBound;
    }

    bound
}

/// Get decompression margin
///
/// Zstd supports in-place decompression, where the input and output buffers overlap.
/// this case, the output buffer must be at least `Margin + Output_Size` bytes large,
/// and the input buffer must be at the end of the output buffer.
///
/// ```md
///  _______________________ Output Buffer ________________________
/// |                                                              |
/// |                                        ____ Input Buffer ____|
/// |                                       |                      |
/// v                                       v                      v
/// |---------------------------------------|-----------|----------|
/// ^                                                   ^          ^
/// |___________________ Output_Size ___________________|_ Margin _|
/// ```
///
/// Note 1: this applies only to single-pass decompression through `ZSTD_decompress()`
/// or `ZSTD_decompressDCtx()`
///
/// Note 2: this function supports multi-frame input
///
/// # Returns
///
/// - the decompression margin in bytes
/// - an error code, which can be tested with [`ZSTD_isError`]
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
    .unwrap_or_else(|err| err.to_error_code())
}

fn decompression_margin(mut src: &[u8]) -> Result<size_t, Error> {
    let mut margin = 0;
    let mut maxBlockSize = 0;

    // iterate over each frame
    while !src.is_empty() {
        let frameSizeInfo = find_frame_size_info(src, Format::ZSTD_f_zstd1);

        let mut zfh = ZSTD_FrameHeader::default();
        get_frame_header(&mut zfh, src)?;

        let frameSizeInfo = frameSizeInfo.map_err(|_| Error::corruption_detected)?;
        let compressedSize = frameSizeInfo.compressedSize;

        if zfh.frameType as core::ffi::c_uint == ZSTD_frame as core::ffi::c_uint {
            // add the frame header to our margin
            margin += zfh.headerSize as size_t;
            margin += if zfh.checksumFlag != 0 { 4 } else { 0 };
            margin += 3 * frameSizeInfo.nbBlocks;
            maxBlockSize = Ord::max(maxBlockSize, zfh.blockSizeMax)
        } else {
            debug_assert!(zfh.frameType == ZSTD_skippableFrame);
            // add the entire skippable frame size to our margin.
            margin += compressedSize;
        }

        src = &src[compressedSize..];
    }

    // add the max block size back to the margin
    margin += maxBlockSize as size_t;

    Ok(margin)
}

/// Insert `src` block into `dctx` history. Useful to track uncompressed blocks.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_insertBlock))]
pub unsafe extern "C" fn ZSTD_insertBlock(
    dctx: *mut ZSTD_DCtx,
    blockStart: *const core::ffi::c_void,
    blockSize: size_t,
) -> size_t {
    let src = Reader::from_raw_parts(blockStart.cast::<u8>(), blockSize);
    ZSTD_checkContinuity(dctx.as_mut().unwrap(), src.as_ptr_range());
    (*dctx).previousDstEnd = (blockStart).byte_add(blockSize);
    blockSize
}

fn copy_raw_block_slice(mut dst: Writer<'_>, src: &[u8]) -> Result<size_t, Error> {
    if src.len() > dst.capacity() {
        return Err(Error::dstSize_tooSmall);
    }

    if dst.is_null() {
        if src.is_empty() {
            return Ok(0);
        }
        return Err(Error::dstBuffer_null);
    }

    unsafe { core::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len()) };

    Ok(src.len())
}

fn copy_raw_block_reader(mut dst: Writer<'_>, src: Reader<'_>) -> Result<size_t, Error> {
    if src.len() > dst.capacity() {
        return Err(Error::dstSize_tooSmall);
    }

    if dst.is_null() {
        if src.is_empty() {
            return Ok(0);
        }
        return Err(Error::dstBuffer_null);
    }

    // src and dst can overlap in this case.
    unsafe { core::ptr::copy(src.as_ptr(), dst.as_mut_ptr(), src.len()) };

    Ok(src.len())
}

fn ZSTD_setRleBlock(mut dst: Writer<'_>, b: u8, regenSize: size_t) -> Result<size_t, Error> {
    if regenSize > dst.capacity() {
        return Err(Error::dstSize_tooSmall);
    }

    if dst.is_null() {
        if regenSize == 0 {
            return Ok(0);
        }
        return Err(Error::dstBuffer_null);
    }

    unsafe { ptr::write_bytes(dst.as_mut_ptr(), b, regenSize) };

    Ok(regenSize)
}

unsafe fn ZSTD_DCtx_trace_end(
    dctx: *const ZSTD_DCtx,
    uncompressedSize: u64,
    compressedSize: u64,
    streaming: core::ffi::c_int,
) {
    if (*dctx).traceCtx != 0 {
        let mut trace = ZSTD_Trace::default();
        trace.version = ZSTD_VERSION_NUMBER as core::ffi::c_uint;
        trace.streaming = streaming;
        if let Some(ddict) = (*dctx).ddict.as_ref() {
            trace.dictionaryID = ZSTD_getDictID_fromDDict(ddict);
            trace.dictionarySize = ZSTD_DDict_dictSize(ddict);
            trace.dictionaryIsCold = (*dctx).ddictIsCold;
        }
        trace.uncompressedSize = uncompressedSize as size_t;
        trace.compressedSize = compressedSize as size_t;
        trace.dctx = dctx;
        ZSTD_trace_decompress_end((*dctx).traceCtx, &trace);
    }
}

/// Decompress a frame.
/// - `dctx` must be properly initialized
/// - will update `*srcPtr` and `*srcSizePtr` to make `*srcPtr` progress by one frame
unsafe fn ZSTD_decompressFrame(
    dctx: &mut ZSTD_DCtx,
    dst: Writer<'_>,
    srcPtr: &mut Reader<'_>,
) -> Result<size_t, Error> {
    let ilen = srcPtr.len();
    let ip = srcPtr;

    let start_capacity = dst.capacity();
    let mut op = dst;
    let oend = op.as_mut_ptr_range().end;

    // check
    if ip.len() < dctx.format.frame_header_size_min() + ZSTD_blockHeaderSize {
        return Err(Error::srcSize_wrong);
    }

    // Frame Header
    let frameHeaderSize = frame_header_size_internal(ip.as_slice(), dctx.format)?;
    if ip.len() < frameHeaderSize.wrapping_add(ZSTD_blockHeaderSize) {
        return Err(Error::srcSize_wrong);
    }
    ZSTD_decodeFrameHeader(dctx, &ip.as_slice()[..frameHeaderSize])?;
    *ip = ip.subslice(frameHeaderSize..);

    // shrink the blockSizeMax if enabled
    if dctx.maxBlockSizeParam != 0 {
        dctx.fParams.blockSizeMax = Ord::min(
            dctx.fParams.blockSizeMax,
            dctx.maxBlockSizeParam as core::ffi::c_uint,
        );
    }

    // loop on each block
    loop {
        let mut oBlockEnd = oend;

        let (blockProperties, cBlockSize) = getc_block_size(ip.as_slice())?;

        *ip = ip.subslice(ZSTD_blockHeaderSize..);
        if cBlockSize > ip.len() {
            return Err(Error::srcSize_wrong);
        }

        if op.as_mut_ptr_range().contains(&ip.as_ptr().cast_mut()) {
            // We are decompressing in-place. Limit the output pointer so that we
            // don't overwrite the block that we are currently reading. This will
            // fail decompression if the input & output pointers aren't spaced
            // far enough apart.
            //
            // This is important to set, even when the pointers are far enough
            // apart, because ZSTD_decompressBlock_internal() can decide to store
            // literals in the output buffer, after the block it is decompressing.
            // Since we don't want anything to overwrite our input, we have to tell
            // ZSTD_decompressBlock_internal to never write past ip.
            //
            // See ZSTD_allocateLiteralsBuffer() for reference.
            oBlockEnd = op
                .as_mut_ptr()
                .add(ip.as_ptr().offset_from_unsigned(op.as_mut_ptr()));
        }

        let decodedSize = match blockProperties.blockType {
            BlockType::Raw => {
                // Use oend instead of oBlockEnd because this function is safe to overlap. It uses memmove.
                copy_raw_block_reader(op.subslice(..), ip.subslice(..cBlockSize))?
            }
            BlockType::Rle => {
                let capacity = oBlockEnd.offset_from(op.as_mut_ptr()) as size_t;
                ZSTD_setRleBlock(
                    op.subslice(..capacity),
                    ip.as_slice()[0],
                    blockProperties.origSize as size_t,
                )?
            }
            BlockType::Compressed => {
                debug_assert!(dctx.isFrameDecompression);
                ZSTD_decompressBlock_internal(
                    dctx,
                    op.as_mut_ptr().cast(),
                    oBlockEnd.offset_from(op.as_mut_ptr()) as size_t,
                    ip.as_ptr().cast(),
                    cBlockSize,
                    not_streaming,
                )?
            }
            BlockType::Reserved => {
                return Err(Error::corruption_detected);
            }
        };

        if dctx.validateChecksum {
            let written = op.subslice(..decodedSize);
            let slice = unsafe { written.as_slice() };
            ZSTD_XXH64_update_slice(&mut dctx.xxhState, slice);
        }

        // Adding 0 to NULL is not UB in rust.
        op = op.subslice(decodedSize..);

        *ip = ip.subslice(cBlockSize..);
        if blockProperties.lastBlock {
            break;
        }
    }
    if dctx.fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
        && (start_capacity - op.capacity()) as core::ffi::c_ulonglong
            != dctx.fParams.frameContentSize
    {
        return Err(Error::corruption_detected);
    }

    // frame content checksum verification
    if dctx.fParams.checksumFlag != 0 {
        let [a, b, c, d, ..] = *ip.as_slice() else {
            return Err(Error::checksum_wrong);
        };

        if dctx.forceIgnoreChecksum == ForceIgnoreChecksum::ValidateChecksum
            && u32::from_le_bytes([a, b, c, d]) != ZSTD_XXH64_digest(&mut dctx.xxhState) as u32
        {
            return Err(Error::checksum_wrong);
        }

        *ip = ip.subslice(4..);
    }

    ZSTD_DCtx_trace_end(
        dctx,
        (start_capacity - op.capacity()) as u64,
        (ilen - ip.len()) as u64,
        0,
    );

    // allow caller to get size read
    Ok(start_capacity - op.capacity())
}

unsafe fn ZSTD_decompressMultiFrame<'a>(
    dctx: *mut ZSTD_DCtx,
    mut dst: Writer<'_>,
    mut src: Reader<'_>,
    mut dict: &'a [u8],
    ddict: Option<&'a ZSTD_DDict>,
) -> Result<size_t, Error> {
    let start_capacity = dst.capacity();
    let mut more_than_one_frame = false;

    if let Some(ddict) = ddict {
        dict = ddict.as_slice();
    }

    while src.len() >= ZSTD_startingInputLength((*dctx).format) {
        if (*dctx).format == Format::ZSTD_f_zstd1 && is_legacy(src.as_slice()) != 0 {
            let frameSizeInfo = find_frame_size_info_legacy(src.as_slice())?;
            let frameSize = frameSizeInfo.compressedSize;

            if (*dctx).staticSize != 0 {
                return Err(Error::memory_allocation);
            }

            let decodedSize =
                ZSTD_decompressLegacy(dst.subslice(..), src.subslice(..frameSize), dict)?;

            let expectedSize = get_frame_content_size(src.as_slice());
            if expectedSize == ZSTD_CONTENTSIZE_ERROR {
                return Err(Error::corruption_detected);
            }

            if expectedSize != ZSTD_CONTENTSIZE_UNKNOWN && expectedSize != decodedSize as u64 {
                return Err(Error::corruption_detected);
            }

            dst = dst.subslice(decodedSize..);
            src = src.subslice(frameSize..);
        } else {
            if (*dctx).format == Format::ZSTD_f_zstd1 && is_skippable_frame(src.as_slice()) {
                // skippable frame detected: skip it
                let skippableSize = read_skippable_frame_size(src.as_slice())?;
                src = src.subslice(skippableSize..);
                continue;
            }

            if let Some(ddict) = ddict {
                Error::from_error_code(ZSTD_decompressBegin_usingDDict(dctx, ddict))
                    .map_or(Ok(()), Err)?
            } else {
                // this will initialize correctly with no dict if `dict == NULL`, so
                // use this in all cases but for ddict
                ZSTD_decompressBegin_usingDict_slice(dctx, dict)?;
            }
            ZSTD_checkContinuity(dctx.as_mut().unwrap(), dst.as_ptr_range());
            let res = ZSTD_decompressFrame(dctx.as_mut().unwrap(), dst.subslice(..), &mut src)
                .map_err(|err| match err {
                    Error::prefix_unknown if more_than_one_frame => Error::srcSize_wrong,
                    _ => err,
                })?;
            dst = dst.subslice(res..);
            more_than_one_frame = true;
        }
    }

    if !src.is_empty() {
        return Err(Error::srcSize_wrong);
    }

    Ok(start_capacity - dst.capacity())
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
    // It is valid for src and dst to overlap.
    let src = Reader::from_raw_parts(src.cast::<u8>(), srcSize);
    let dst = Writer::from_raw_parts(dst.cast::<u8>(), dstCapacity);

    let dict = if dict.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(dict.cast::<u8>(), dictSize)
    };

    ZSTD_decompressMultiFrame(dctx, dst, src, dict, None).unwrap_or_else(Error::to_error_code)
}

unsafe fn ZSTD_getDDict(dctx: *mut ZSTD_DCtx) -> *const ZSTD_DDict {
    match (*dctx).dictUses {
        DictUses::ZSTD_use_indefinitely => (*dctx).ddict,
        DictUses::ZSTD_use_once => {
            (*dctx).dictUses = DictUses::ZSTD_dont_use;
            (*dctx).ddict
        }
        DictUses::ZSTD_dont_use => {
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
    let dctx = ZSTD_createDCtx_internal(ZSTD_customMem::default());
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

/// Similar to `ZSTD_nextSrcSizeToDecompress()`, but when a block input can be streamed, we
/// allow taking a partial block as the input. Currently only raw uncompressed blocks can
/// be streamed.
///
/// For blocks that can be streamed, this allows us to reduce the latency until we produce
/// output, and avoid copying the input.
fn ZSTD_nextSrcSizeToDecompressWithInputSize(dctx: &mut ZSTD_DCtx, inputSize: size_t) -> size_t {
    match dctx.stage {
        DecompressStage::DecompressBlock | DecompressStage::DecompressLastBlock => {
            if dctx.bType != BlockType::Raw {
                return dctx.expected;
            }

            // Apparently it's possible for min > max here, so Ord::clamp would panic.
            Ord::max(1, Ord::min(inputSize, dctx.expected))
        }
        _ => dctx.expected,
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_nextInputType))]
pub unsafe extern "C" fn ZSTD_nextInputType(dctx: *mut ZSTD_DCtx) -> ZSTD_nextInputType_e {
    (*dctx).stage.to_next_input_type() as ZSTD_nextInputType_e
}

/// Continue decompressing
///
/// `srcSize` must be the exact number of bytes expected (see [`ZSTD_nextSrcSizeToDecompress`])
///
/// # Returns
///
/// - the number of bytes generated into `dst` (necessarily <= `dstCapacity`)
/// - an error code, which can be tested with [`ZSTD_isError`]
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressContinue))]
pub unsafe extern "C" fn ZSTD_decompressContinue(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let dctx = dctx.as_mut().unwrap();

    // For `ZSTD_decompressContinue` is is not valid for src and dst to overlap.
    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast::<u8>(), srcSize)
    };

    // NOTE: already handles the `dst.is_null()` case.
    let dst = Writer::from_raw_parts(dst.cast::<u8>(), dstCapacity);

    decompress_continue(dctx, dst, src).unwrap_or_else(Error::to_error_code)
}

unsafe fn decompress_continue(
    dctx: &mut ZSTD_DCtx,
    mut dst: Writer<'_>,
    src: &[u8],
) -> Result<size_t, Error> {
    if src.len() != ZSTD_nextSrcSizeToDecompressWithInputSize(dctx, src.len()) {
        return Err(Error::srcSize_wrong);
    }
    ZSTD_checkContinuity(dctx, dst.as_ptr_range());
    dctx.processedCSize = (dctx.processedCSize as size_t).wrapping_add(src.len()) as u64;

    match dctx.stage {
        DecompressStage::GetFrameHeaderSize => {
            if dctx.format == Format::ZSTD_f_zstd1 && is_skippable_frame(src) {
                dctx.headerBuffer[..src.len()].copy_from_slice(src);

                // remaining data to load to get full skippable frame header
                dctx.expected = (ZSTD_SKIPPABLEHEADERSIZE as size_t).wrapping_sub(src.len());
                dctx.stage = DecompressStage::DecodeSkippableHeader;
                return Ok(0);
            }
            dctx.headerSize = frame_header_size_internal(src, dctx.format)?;
            dctx.headerBuffer[..src.len()].copy_from_slice(src);
            dctx.expected = (dctx.headerSize).wrapping_sub(src.len());
            dctx.stage = DecompressStage::DecodeFrameHeader;
            Ok(0)
        }
        DecompressStage::DecodeFrameHeader => {
            dctx.headerBuffer[(dctx.headerSize) - src.len()..][..src.len()].copy_from_slice(src);
            let header_buffer = dctx.headerBuffer;
            ZSTD_decodeFrameHeader(dctx, &header_buffer[..dctx.headerSize])?;
            dctx.expected = ZSTD_blockHeaderSize;
            dctx.stage = DecompressStage::DecodeBlockHeader;
            Ok(0)
        }
        DecompressStage::DecodeBlockHeader => {
            let (bp, cBlockSize) = getc_block_size(src)?;

            if cBlockSize > dctx.fParams.blockSizeMax as size_t {
                return Err(Error::corruption_detected);
            }
            dctx.expected = cBlockSize;
            dctx.bType = bp.blockType;
            dctx.rleSize = bp.origSize as size_t;
            if cBlockSize != 0 {
                dctx.stage = if bp.lastBlock {
                    DecompressStage::DecompressLastBlock
                } else {
                    DecompressStage::DecompressBlock
                };
                return Ok(0);
            }

            // empty block
            if bp.lastBlock {
                if dctx.fParams.checksumFlag != 0 {
                    dctx.expected = 4;
                    dctx.stage = DecompressStage::CheckChecksum;
                } else {
                    dctx.expected = 0; // end of frame
                    dctx.stage = DecompressStage::GetFrameHeaderSize;
                }
            } else {
                dctx.expected = ZSTD_blockHeaderSize; // jump to next header
                dctx.stage = DecompressStage::DecodeBlockHeader;
            }
            Ok(0)
        }

        DecompressStage::DecompressBlock | DecompressStage::DecompressLastBlock => {
            let mut rSize: size_t = 0;
            match dctx.bType {
                BlockType::Compressed => {
                    debug_assert!(dctx.isFrameDecompression);
                    dctx.expected = 0; // streaming not supported
                    rSize = ZSTD_decompressBlock_internal(
                        dctx,
                        dst.as_mut_ptr().cast(),
                        dst.capacity(),
                        src.as_ptr().cast(),
                        src.len(),
                        is_streaming,
                    )?;
                }
                BlockType::Raw => {
                    debug_assert!(src.len() <= dctx.expected);
                    rSize = copy_raw_block_slice(dst.subslice(..), src)?;
                    debug_assert_eq!(rSize, src.len());
                    dctx.expected = (dctx.expected).wrapping_sub(rSize);
                }
                BlockType::Rle => {
                    dctx.expected = 0; // streaming not supported
                    rSize = ZSTD_setRleBlock(dst.subslice(..), src[0], dctx.rleSize)?;
                }
                BlockType::Reserved => {
                    return Err(Error::corruption_detected);
                }
            }
            if rSize > dctx.fParams.blockSizeMax as size_t {
                return Err(Error::corruption_detected);
            }
            dctx.decodedSize = dctx.decodedSize.wrapping_add(rSize as u64);
            if dctx.validateChecksum {
                let written = dst.subslice(..rSize);
                let slice = unsafe { written.as_slice() };
                ZSTD_XXH64_update_slice(&mut dctx.xxhState, slice);
            }
            dctx.previousDstEnd = dst.as_mut_ptr().byte_add(rSize).cast::<core::ffi::c_void>();

            // stay on the same stage until we are finished streaming the block
            if dctx.expected > 0 {
                return Ok(rSize);
            }

            if dctx.stage == DecompressStage::DecompressLastBlock {
                // end of frame
                if dctx.fParams.frameContentSize != (0 as core::ffi::c_ulonglong).wrapping_sub(1)
                    && dctx.decodedSize as core::ffi::c_ulonglong != dctx.fParams.frameContentSize
                {
                    return Err(Error::corruption_detected);
                }
                if dctx.fParams.checksumFlag != 0 {
                    // another round for frame checksum
                    dctx.expected = 4;
                    dctx.stage = DecompressStage::CheckChecksum;
                } else {
                    ZSTD_DCtx_trace_end(dctx, dctx.decodedSize, dctx.processedCSize, 1);
                    dctx.expected = 0;
                    dctx.stage = DecompressStage::GetFrameHeaderSize;
                }
            } else {
                dctx.stage = DecompressStage::DecodeBlockHeader;
                dctx.expected = ZSTD_blockHeaderSize;
            }
            Ok(rSize)
        }
        DecompressStage::CheckChecksum => {
            debug_assert_eq!(src.len(), 4); // guaranteed by dctx.expected
            if dctx.validateChecksum {
                let h32 = ZSTD_XXH64_digest(&mut dctx.xxhState) as u32;
                let check32 = u32::from_le_bytes(*src.first_chunk().unwrap());
                if check32 != h32 {
                    return Err(Error::checksum_wrong);
                }
            }
            ZSTD_DCtx_trace_end(dctx, dctx.decodedSize, dctx.processedCSize, 1);
            dctx.expected = 0;
            dctx.stage = DecompressStage::GetFrameHeaderSize;
            Ok(0)
        }
        DecompressStage::DecodeSkippableHeader => {
            debug_assert_ne!(dctx.format, Format::ZSTD_f_zstd1_magicless);
            // complete skippable header
            let headerSize = ZSTD_SKIPPABLEHEADERSIZE as usize;
            dctx.headerBuffer[headerSize - src.len()..headerSize].copy_from_slice(src);
            dctx.expected =
                u32::from_le_bytes(*dctx.headerBuffer[ZSTD_FRAMEIDSIZE..].first_chunk().unwrap())
                    as usize;
            dctx.stage = DecompressStage::SkipFrame;
            Ok(0)
        }
        DecompressStage::SkipFrame => {
            dctx.expected = 0;
            dctx.stage = DecompressStage::GetFrameHeaderSize;
            Ok(0)
        }
    }
}

/// Load dictionary entropy
///
/// `dict` must point at beginning of a valid zstd dictionary
///
/// # Returns
///
/// - size of entropy tables read
/// - an error code, which can be tested with [`ZSTD_isError`]
pub fn ZSTD_loadDEntropy(entropy: &mut ZSTD_entropyDTables_t, dict: &[u8]) -> size_t {
    // skip header = magic + dictID
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

    // use fse tables as temporary workspace; implies fse tables are grouped together
    let workspace = &mut entropy.LLTable;
    let wksp: &mut HUF_ReadDTableX2_Workspace = unsafe { core::mem::transmute(workspace) };

    let hSize = HUF_readDTableX2_wksp(&mut entropy.hufTable, dictPtr, wksp, 0);
    if ERR_isError(hSize) {
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

fn ZSTD_refDictContent(dctx: &mut ZSTD_DCtx, dict: &[u8]) {
    dctx.dictEnd = dctx.previousDstEnd;
    let delta = dctx.previousDstEnd.addr() - dctx.prefixStart.addr();
    dctx.virtualStart = dict.as_ptr().wrapping_sub(delta).cast();
    dctx.prefixStart = dict.as_ptr().cast();
    dctx.previousDstEnd = dict.as_ptr_range().end.cast();
}

fn ZSTD_decompress_insertDictionary(dctx: &mut ZSTD_DCtx, dict: &[u8]) -> Result<(), Error> {
    let ([magic, dict_id, ..], _) = dict.as_chunks::<4>() else {
        ZSTD_refDictContent(dctx, dict);
        return Ok(());
    };

    let magic = u32::from_le_bytes(*magic);
    if magic != ZSTD_MAGIC_DICTIONARY {
        ZSTD_refDictContent(dctx, dict); // pure content mode
        return Ok(());
    }
    dctx.dictID = u32::from_le_bytes(*dict_id);

    let eSize = ZSTD_loadDEntropy(&mut dctx.entropy, dict);
    if ERR_isError(eSize) {
        return Err(Error::dictionary_corrupted);
    }

    dctx.fseEntropy = true;
    dctx.litEntropy = dctx.fseEntropy;

    ZSTD_refDictContent(dctx, &dict[eSize..]);
    Ok(())
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBegin))]
pub unsafe extern "C" fn ZSTD_decompressBegin(dctx: *mut ZSTD_DCtx) -> size_t {
    let dctx = dctx.cast::<MaybeUninit<ZSTD_DCtx>>().as_mut().unwrap();
    decompress_begin(dctx);
    0
}

fn decompress_begin(dctx: &mut MaybeUninit<ZSTD_DCtx>) {
    // SAFETY: the type of dctx guarantees that we're allowed to write to this memory.
    unsafe {
        let dctx = dctx.as_mut_ptr();

        (*dctx).traceCtx = ZSTD_trace_decompress_begin(dctx);
        (*dctx).expected = ZSTD_startingInputLength((*dctx).format);
        (*dctx).stage = DecompressStage::GetFrameHeaderSize;
        (*dctx).processedCSize = 0;
        (*dctx).decodedSize = 0;
        (*dctx).previousDstEnd = core::ptr::null();
        (*dctx).prefixStart = core::ptr::null();
        (*dctx).virtualStart = core::ptr::null();
        (*dctx).dictEnd = core::ptr::null();
        (*dctx).entropy.hufTable.description = DTableDesc::default();
        (*dctx).fseEntropy = false;
        (*dctx).litEntropy = (*dctx).fseEntropy;
        (*dctx).dictID = 0;
        (*dctx).bType = BlockType::Reserved;
        (*dctx).isFrameDecompression = true;
        (*dctx).entropy.rep = repStartValue;
        (*dctx).LLTptr = NonNull::new((&raw const (*dctx).entropy.LLTable).cast_mut());
        (*dctx).MLTptr = NonNull::new((&raw const (*dctx).entropy.MLTable).cast_mut());
        (*dctx).OFTptr = NonNull::new((&raw const (*dctx).entropy.OFTable).cast_mut());

        // None encodes dctx.entropy.hufTable.
        (*dctx).HUFptr = None;
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBegin_usingDict))]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDict(
    dctx: *mut ZSTD_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let dctx = dctx.cast::<MaybeUninit<ZSTD_DCtx>>().as_mut().unwrap();
    decompress_begin(dctx);

    if dict.is_null() || dictSize == 0 {
        return 0;
    }

    let dict = core::slice::from_raw_parts(dict.cast::<u8>(), dictSize);
    ZSTD_decompress_insertDictionary(dctx.assume_init_mut(), dict)
        .map(|_| 0)
        .unwrap_or_else(Error::to_error_code)
}

unsafe fn ZSTD_decompressBegin_usingDict_slice(
    dctx: *mut ZSTD_DCtx,
    dict: &[u8],
) -> Result<(), Error> {
    let dctx = dctx.cast::<MaybeUninit<ZSTD_DCtx>>().as_mut().unwrap();
    decompress_begin(dctx);

    if dict.is_empty() {
        return Ok(());
    }

    ZSTD_decompress_insertDictionary(dctx.assume_init_mut(), dict)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBegin_usingDDict))]
pub unsafe extern "C" fn ZSTD_decompressBegin_usingDDict(
    dctx: *mut ZSTD_DCtx,
    ddict: *const ZSTD_DDict,
) -> size_t {
    // The C version does not check for NULL, we panic instead.
    let dctx = dctx.cast::<MaybeUninit<ZSTD_DCtx>>().as_mut().unwrap();

    if let Some(ddict) = ddict.as_ref() {
        let dictStart = ZSTD_DDict_dictContent(ddict) as *const core::ffi::c_char;
        let dictSize = ZSTD_DDict_dictSize(ddict);
        let dictEnd = dictStart.add(dictSize) as *const core::ffi::c_void;
        (*dctx.as_mut_ptr()).ddictIsCold = (*dctx.as_ptr()).dictEnd != dictEnd;
    }

    decompress_begin(dctx);

    // NULL ddict is equivalent to no dictionary
    if let Some(ddict) = ddict.as_ref() {
        ZSTD_copyDDictParameters(dctx, ddict);
    }

    0
}

/// Provides the `dictID` stored within dictionary
///
/// # Returns
///
/// - the `dictID` if available
/// - `0` if the dictionary is not conformant with Zstandard specification,
///   in which case it can still be loaded as a content-only dictionary
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
        (dict as *const core::ffi::c_char).add(ZSTD_FRAMEIDSIZE) as *const core::ffi::c_void
    )
}

/// Provides the `dictID` required to decompress frame stored within `src`
///
/// # Returns
///
/// - the `dictID` if available
/// - `0` if the `dictID` could not be decoded. This could for one of the following reasons:
///   - The frame does not require a dictionary (most common case)
///   - The frame was built with `dictID` intentionally removed, this also happens when using a
///     non-conformant dictionary
///   - `srcSize` is too small, and as a result, frame header could not be decoded, possible if
///     `srcSize < ZSTD_FRAMEHEADERSIZE_MAX`
///   - This is not a Zstandard frame
///
/// When identifying the exact failure cause, it's possible to use [`ZSTD_getFrameHeader`],
/// which will provide a more precise error code
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getDictID_fromFrame))]
pub unsafe extern "C" fn ZSTD_getDictID_fromFrame(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_uint {
    let mut zfp = ZSTD_FrameHeader::default();

    let hError = ZSTD_getFrameHeader(&mut zfp, src, srcSize);
    if ERR_isError(hError) {
        return 0;
    }

    zfp.dictID
}

/// Decompression using a pre-digested Dictionary
///
/// Uses dictionary without significant overhead
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompress_usingDDict))]
pub unsafe extern "C" fn ZSTD_decompress_usingDDict(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    ddict: *const ZSTD_DDict,
) -> size_t {
    // It is valid for src and dst to overlap.
    let src = Reader::from_raw_parts(src.cast::<u8>(), srcSize);
    let dst = Writer::from_raw_parts(dst.cast::<u8>(), dstCapacity);

    ZSTD_decompressMultiFrame(dctx, dst, src, &[], ddict.as_ref())
        .unwrap_or_else(Error::to_error_code)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDStream))]
pub unsafe extern "C" fn ZSTD_createDStream() -> *mut ZSTD_DStream {
    ZSTD_createDCtx_internal(ZSTD_customMem::default())
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
pub const extern "C" fn ZSTD_DStreamInSize() -> size_t {
    (ZSTD_BLOCKSIZE_MAX as size_t).wrapping_add(ZSTD_blockHeaderSize)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DStreamOutSize))]
pub const extern "C" fn ZSTD_DStreamOutSize() -> size_t {
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
        (*dctx).dictUses = DictUses::ZSTD_use_indefinitely;
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
    if ERR_isError(err_code) {
        return err_code;
    }
    (*dctx).dictUses = DictUses::ZSTD_use_once;
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

/// # Returns
///
/// - the expected size, aka [`ZSTD_startingInputLength`]
/// - an error code, which can be tested with [`ZSTD_isError`]
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initDStream_usingDict))]
pub unsafe extern "C" fn ZSTD_initDStream_usingDict(
    zds: *mut ZSTD_DStream,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let err_code = ZSTD_DCtx_reset(zds, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code = ZSTD_DCtx_loadDictionary(zds, dict, dictSize);
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_startingInputLength((*zds).format)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initDStream))]
pub unsafe extern "C" fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t {
    let err_code = ZSTD_DCtx_reset(zds, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code = ZSTD_DCtx_refDDict(zds, core::ptr::null::<ZSTD_DDict>());
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_startingInputLength((*zds).format)
}

/// Note: DDict will just be referenced, and must outlive decompression session
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initDStream_usingDDict))]
pub unsafe extern "C" fn ZSTD_initDStream_usingDDict(
    dctx: *mut ZSTD_DStream,
    ddict: *const ZSTD_DDict,
) -> size_t {
    let err_code = ZSTD_DCtx_reset(dctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
        return err_code;
    }
    let err_code = ZSTD_DCtx_refDDict(dctx, ddict);
    if ERR_isError(err_code) {
        return err_code;
    }
    ZSTD_startingInputLength((*dctx).format)
}

/// # Returns
///
/// - the expected size, aka [`ZSTD_startingInputLength`]
/// - an error code, which can be tested with [`ZSTD_isError`]
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_resetDStream))]
pub unsafe extern "C" fn ZSTD_resetDStream(dctx: *mut ZSTD_DStream) -> size_t {
    let err_code = ZSTD_DCtx_reset(dctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ERR_isError(err_code) {
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
        (*dctx).dictUses = DictUses::ZSTD_use_indefinitely;
        if (*dctx).refMultipleDDicts == MultipleDDicts::Multiple {
            if ((*dctx).ddictSet).is_null() {
                (*dctx).ddictSet = ZSTD_createDDictHashSet((*dctx).customMem);
            }

            let Some(ddictSet) = (*dctx).ddictSet.as_mut() else {
                return Error::memory_allocation.to_error_code();
            };

            debug_assert_eq!((*dctx).staticSize, 0); // ddictSet cannot have been allocated if static dctx
            let err_code = ZSTD_DDictHashSet_addDDict(ddictSet, ddict, (*dctx).customMem);
            if ERR_isError(err_code) {
                return err_code;
            }
        }
    }

    0
}

/// Note: no direct equivalence in [`ZSTD_DCtx_setParameter`], since this version sets `windowSize`,
/// and the other sets `windowLog`
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_setMaxWindowSize))]
pub unsafe extern "C" fn ZSTD_DCtx_setMaxWindowSize(
    dctx: *mut ZSTD_DCtx,
    maxWindowSize: size_t,
) -> size_t {
    let bounds = ZSTD_dParam_getBounds(ZSTD_dParameter::ZSTD_d_windowLogMax);
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
        ZSTD_dParameter::ZSTD_d_format as ZSTD_dParameter,
        u32::from(format) as core::ffi::c_int,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_dParam_getBounds))]
pub extern "C" fn ZSTD_dParam_getBounds(dParam: ZSTD_dParameter) -> ZSTD_bounds {
    let mut bounds = {
        ZSTD_bounds {
            error: 0,
            lowerBound: 0,
            upperBound: 0,
        }
    };
    match dParam {
        ZSTD_dParameter::ZSTD_d_windowLogMax => {
            bounds.lowerBound = ZSTD_WINDOWLOG_ABSOLUTEMIN;
            bounds.upperBound = ZSTD_WINDOWLOG_MAX;
            return bounds;
        }
        ZSTD_dParameter::ZSTD_d_format => {
            bounds.lowerBound = Format::ZSTD_f_zstd1 as core::ffi::c_int;
            bounds.upperBound = Format::ZSTD_f_zstd1_magicless as core::ffi::c_int;
            return bounds;
        }
        ZSTD_dParameter::ZSTD_d_stableOutBuffer => {
            bounds.lowerBound = BufferMode::Buffered as core::ffi::c_int;
            bounds.upperBound = BufferMode::Stable as core::ffi::c_int;
            return bounds;
        }
        ZSTD_dParameter::ZSTD_d_forceIgnoreChecksum => {
            bounds.lowerBound = ForceIgnoreChecksum::ValidateChecksum as core::ffi::c_int;
            bounds.upperBound = ForceIgnoreChecksum::IgnoreChecksum as core::ffi::c_int;
            return bounds;
        }
        ZSTD_dParameter::ZSTD_d_refMultipleDDicts => {
            bounds.lowerBound = MultipleDDicts::Single as core::ffi::c_int;
            bounds.upperBound = MultipleDDicts::Multiple as core::ffi::c_int;
            return bounds;
        }
        ZSTD_dParameter::ZSTD_d_disableHuffmanAssembly => {
            bounds.lowerBound = 0;
            bounds.upperBound = 1;
            return bounds;
        }
        ZSTD_dParameter::ZSTD_d_maxBlockSize => {
            bounds.lowerBound = ZSTD_BLOCKSIZE_MAX_MIN;
            bounds.upperBound = ZSTD_BLOCKSIZE_MAX;
            return bounds;
        }
        _ => {}
    }
    bounds.error = Error::parameter_unsupported.to_error_code();
    bounds
}

/// # Returns
///
/// - `true` if `value` is within `dParam` bounds
/// - `false` otherwise
fn ZSTD_dParam_withinBounds(dParam: ZSTD_dParameter, value: core::ffi::c_int) -> bool {
    let bounds = ZSTD_dParam_getBounds(dParam);
    if ERR_isError(bounds.error) {
        return false;
    }

    (bounds.lowerBound..=bounds.upperBound).contains(&value)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_DCtx_getParameter))]
pub unsafe extern "C" fn ZSTD_DCtx_getParameter(
    dctx: *mut ZSTD_DCtx,
    param: ZSTD_dParameter,
    value: *mut core::ffi::c_int,
) -> size_t {
    *value = match param {
        ZSTD_dParameter::ZSTD_d_windowLogMax => (*dctx).maxWindowSize.ilog2() as i32,
        ZSTD_dParameter::ZSTD_d_format => (*dctx).format as core::ffi::c_int,
        ZSTD_dParameter::ZSTD_d_stableOutBuffer => (*dctx).outBufferMode as core::ffi::c_int,
        ZSTD_dParameter::ZSTD_d_forceIgnoreChecksum => {
            (*dctx).forceIgnoreChecksum as core::ffi::c_int
        }
        ZSTD_dParameter::ZSTD_d_refMultipleDDicts => (*dctx).refMultipleDDicts as core::ffi::c_int,
        ZSTD_dParameter::ZSTD_d_disableHuffmanAssembly => (*dctx).disableHufAsm as core::ffi::c_int,
        ZSTD_dParameter::ZSTD_d_maxBlockSize => (*dctx).maxBlockSizeParam,
        _ => return Error::parameter_unsupported.to_error_code(),
    };

    0
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
    match dParam {
        ZSTD_dParameter::ZSTD_d_windowLogMax => {
            if value == 0 {
                value = ZSTD_WINDOWLOG_LIMIT_DEFAULT;
            }
            if !ZSTD_dParam_withinBounds(ZSTD_dParameter::ZSTD_d_windowLogMax, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*dctx).maxWindowSize = (1) << value;
            return 0;
        }
        ZSTD_dParameter::ZSTD_d_experimentalParam1 => {
            let Ok(format) = Format::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };

            (*dctx).format = format;

            return 0;
        }
        ZSTD_dParameter::ZSTD_d_stableOutBuffer => {
            let Ok(value) = BufferMode::try_from(value as u32) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*dctx).outBufferMode = value;
            return 0;
        }
        ZSTD_dParameter::ZSTD_d_forceIgnoreChecksum => {
            let Ok(value) = ForceIgnoreChecksum::try_from(value) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            (*dctx).forceIgnoreChecksum = value;
            return 0;
        }
        ZSTD_dParameter::ZSTD_d_refMultipleDDicts => {
            let Ok(value) = MultipleDDicts::try_from(value as u32) else {
                return Error::parameter_outOfBound.to_error_code();
            };
            if (*dctx).staticSize != 0 {
                return Error::parameter_unsupported.to_error_code();
            }
            (*dctx).refMultipleDDicts = value;
            return 0;
        }
        ZSTD_dParameter::ZSTD_d_disableHuffmanAssembly => {
            if !ZSTD_dParam_withinBounds(ZSTD_dParameter::ZSTD_d_experimentalParam5, value) {
                return Error::parameter_outOfBound.to_error_code();
            }
            (*dctx).disableHufAsm = value != 0;
            return 0;
        }
        ZSTD_dParameter::ZSTD_d_maxBlockSize => {
            if value != 0
                && !ZSTD_dParam_withinBounds(ZSTD_dParameter::ZSTD_d_experimentalParam6, value)
            {
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
    let dctx = dctx.as_mut().unwrap();

    match dctx.reset(reset) {
        Ok(()) => 0,
        Err(err) => err.to_error_code(),
    }
}

impl ZSTD_DCtx_s {
    fn reset(&mut self, reset: ZSTD_ResetDirective) -> Result<(), Error> {
        if matches!(
            reset,
            ZSTD_ResetDirective::ZSTD_reset_session_only
                | ZSTD_ResetDirective::ZSTD_reset_session_and_parameters
        ) {
            self.streamStage = StreamStage::Init;
            self.noForwardProgress = 0;
            self.isFrameDecompression = true;
        }

        if matches!(
            reset,
            ZSTD_ResetDirective::ZSTD_reset_parameters
                | ZSTD_ResetDirective::ZSTD_reset_session_and_parameters
        ) {
            if self.streamStage != StreamStage::Init {
                return Err(Error::stage_wrong);
            }
            self.clear_dict();
            ZSTD_DCtx_resetParameters(unsafe {
                &mut *(self as *mut _ as *mut MaybeUninit<ZSTD_DCtx>)
            });
        }

        Ok(())
    }

    fn clear_dict(&mut self) {
        unsafe { ZSTD_freeDDict(self.ddictLocal) };

        self.ddictLocal = core::ptr::null_mut();
        self.ddict = core::ptr::null();
        self.dictUses = DictUses::ZSTD_dont_use;
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_DStream))]
pub unsafe extern "C" fn ZSTD_sizeof_DStream(dctx: *const ZSTD_DStream) -> size_t {
    ZSTD_sizeof_DCtx(dctx)
}

fn ZSTD_decodingBufferSize_internal(
    windowSize: core::ffi::c_ulonglong,
    frameContentSize: core::ffi::c_ulonglong,
    blockSizeMax: size_t,
) -> size_t {
    let blockSize = Ord::min(
        Ord::min(windowSize, ZSTD_BLOCKSIZE_MAX as core::ffi::c_ulonglong) as size_t,
        blockSizeMax,
    );

    // We need blockSize + WILDCOPY_OVERLENGTH worth of buffer so that if a block
    // ends at windowSize + WILDCOPY_OVERLENGTH + 1 bytes, we can start writing
    // the block at the beginning of the output buffer, and maintain a full window.
    //
    // We need another blockSize worth of buffer so that we can store split
    // literals at the end of the block without overwriting the extDict window.
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
pub extern "C" fn ZSTD_decodingBufferSize_min(
    windowSize: core::ffi::c_ulonglong,
    frameContentSize: core::ffi::c_ulonglong,
) -> size_t {
    ZSTD_decodingBufferSize_internal(windowSize, frameContentSize, ZSTD_BLOCKSIZE_MAX as size_t)
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateDStreamSize))]
pub extern "C" fn ZSTD_estimateDStreamSize(windowSize: size_t) -> size_t {
    let blockSize = Ord::min(windowSize, ZSTD_BLOCKSIZE_MAX as size_t);
    let inBuffSize = blockSize; // no block can be larger
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
    let windowSizeMax = 1 << ZSTD_WINDOWLOG_MAX; // note: should be user-selectable, but requires an additional parameter (or a dctx)
    let mut zfh = ZSTD_FrameHeader::default();
    let err = ZSTD_getFrameHeader(&mut zfh, src, srcSize);
    if ERR_isError(err) {
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

fn ZSTD_DCtx_isOverflow(
    zds: &ZSTD_DStream,
    neededInBuffSize: size_t,
    neededOutBuffSize: size_t,
) -> bool {
    zds.inBuffSize.wrapping_add(zds.outBuffSize)
        >= neededInBuffSize.wrapping_add(neededOutBuffSize)
            * ZSTD_WORKSPACETOOLARGE_FACTOR as size_t
}

fn ZSTD_DCtx_updateOversizedDuration(
    zds: &mut ZSTD_DStream,
    neededInBuffSize: size_t,
    neededOutBuffSize: size_t,
) {
    if ZSTD_DCtx_isOverflow(zds, neededInBuffSize, neededOutBuffSize) {
        zds.oversizedDuration = zds.oversizedDuration.wrapping_add(1);
    } else {
        zds.oversizedDuration = 0;
    };
}

fn ZSTD_DCtx_isOversizedTooLong(zds: &ZSTD_DStream) -> bool {
    zds.oversizedDuration >= ZSTD_WORKSPACETOOLARGE_MAXDURATION as size_t
}

/// Checks that the output buffer hasn't changed if [`ZSTD_obm_stable`] is used
fn ZSTD_checkOutBuffer(zds: &ZSTD_DStream, output: &ZSTD_outBuffer) -> Result<(), Error> {
    // No requirement when `ZSTD_obm_stable` is not enabled
    if zds.outBufferMode != BufferMode::Stable {
        return Ok(());
    }

    // Any buffer is allowed in `zdss_init`, this must be the same for every other call until the context is reset
    if zds.streamStage == StreamStage::Init {
        return Ok(());
    }

    // The buffer must match our expectation exactly
    let expect = zds.expectedOutBuffer;
    if expect.dst == output.dst && expect.pos == output.pos && expect.size == output.size {
        return Ok(());
    }

    Err(Error::dstBuffer_wrong)
}

/// Calls `ZSTD_decompressContinue()` with the right parameters for `ZSTD_decompressStream()`
/// and updates the stage and the output buffer state. This call is extracted so it can be
/// used both when reading directly from the [`ZSTD_inBuffer`], and in buffered input mode.
///
/// Note: you must break after calling this function since the `streamStage` is modified
unsafe fn ZSTD_decompressContinueStream(
    zds: &mut ZSTD_DStream,
    op: &mut *mut core::ffi::c_char,
    oend: *mut core::ffi::c_char,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    match zds.outBufferMode {
        BufferMode::Buffered => {
            let dstSize = match zds.stage {
                DecompressStage::SkipFrame => 0,
                _ => (zds.outBuffSize).wrapping_sub(zds.outStart),
            };
            let decodedSize = ZSTD_decompressContinue(
                zds,
                (zds.outBuff).add(zds.outStart) as *mut core::ffi::c_void,
                dstSize,
                src,
                srcSize,
            );
            let err_code = decodedSize;
            if ERR_isError(err_code) {
                return err_code;
            }
            if decodedSize == 0 && !matches!(zds.stage, DecompressStage::SkipFrame) {
                zds.streamStage = StreamStage::Read;
            } else {
                zds.outEnd = (zds.outStart).wrapping_add(decodedSize);
                zds.streamStage = StreamStage::Flush;
            }
        }
        BufferMode::Stable => {
            // write directly into the output buffer
            let dstSize = match zds.stage {
                DecompressStage::SkipFrame => 0,
                _ => oend.offset_from_unsigned(*op),
            };
            let decodedSize =
                ZSTD_decompressContinue(zds, *op as *mut core::ffi::c_void, dstSize, src, srcSize);
            let err_code = decodedSize;
            if ERR_isError(err_code) {
                return err_code;
            }
            *op = (*op).add(decodedSize);
            // flushing is not needed
            zds.streamStage = StreamStage::Read;
            debug_assert!(*op <= oend);
            debug_assert_eq!(zds.outBufferMode, BufferMode::Stable);
        }
    }

    0
}

#[allow(clippy::drop_non_drop)]
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressStream))]
pub unsafe extern "C" fn ZSTD_decompressStream(
    zds: *mut ZSTD_DStream,
    output: *mut ZSTD_outBuffer,
    input: *mut ZSTD_inBuffer,
) -> size_t {
    let zds = zds.as_mut().unwrap();
    let output = output.as_mut().unwrap();
    let input = input.as_mut().unwrap();

    let src = input.src as *const c_char;
    let istart = src.add(input.pos);
    let iend = src.add(input.size);
    let mut ip = istart;
    let dst = output.dst as *mut c_char;
    let ostart = dst.add(output.pos);
    let oend = dst.add(output.size);
    let mut op = ostart;
    let mut some_more_work = true;
    if input.pos > input.size {
        return Error::srcSize_wrong.to_error_code();
    }
    if output.pos > output.size {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if let Err(err) = ZSTD_checkOutBuffer(zds, output) {
        return err.to_error_code();
    }

    while some_more_work {
        #[derive(Eq, PartialEq)]
        enum Block {
            LoadHeader,
            Read,
            Load,
        }

        let mut current_block = match zds.streamStage {
            StreamStage::Init => {
                zds.streamStage = StreamStage::LoadHeader;
                zds.outEnd = 0;
                zds.outStart = zds.outEnd;
                zds.inPos = zds.outStart;
                zds.lhSize = zds.inPos;
                zds.legacyVersion = 0;
                zds.hostageByte = 0;
                zds.expectedOutBuffer = *output;

                Block::LoadHeader
            }
            StreamStage::LoadHeader => Block::LoadHeader,
            StreamStage::Read => Block::Read,
            StreamStage::Load => Block::Load,
            StreamStage::Flush => {
                let toFlushSize = (zds.outEnd).wrapping_sub(zds.outStart);
                let flushedSize = ZSTD_limitCopy(
                    op.cast::<u8>(),
                    oend.offset_from_unsigned(op),
                    (zds.outBuff).add(zds.outStart),
                    toFlushSize,
                );

                op = op.add(flushedSize);

                zds.outStart = (zds.outStart).wrapping_add(flushedSize);
                if flushedSize == toFlushSize {
                    // flush completed
                    zds.streamStage = StreamStage::Read;
                    if (zds.outBuffSize as core::ffi::c_ulonglong) < zds.fParams.frameContentSize
                        && (zds.outStart).wrapping_add(zds.fParams.blockSizeMax as size_t)
                            > zds.outBuffSize
                    {
                        zds.outEnd = 0;
                        zds.outStart = 0;
                    }
                    continue;
                }

                // cannot complete flush
                some_more_work = false;
                continue;
            }
        };

        if current_block == Block::LoadHeader {
            drop(current_block);

            if zds.legacyVersion != 0 {
                if zds.staticSize != 0 {
                    return Error::memory_allocation.to_error_code();
                }
                let hint = ZSTD_decompressLegacyStream(
                    zds.legacyContext,
                    zds.legacyVersion,
                    output,
                    input,
                );
                if hint == 0 {
                    zds.streamStage = StreamStage::Init;
                }
                return hint;
            }

            let hSize = get_frame_header_advanced(
                &mut zds.fParams,
                &zds.headerBuffer[..zds.lhSize],
                zds.format,
            );
            if zds.refMultipleDDicts != MultipleDDicts::Single && !zds.ddictSet.is_null() {
                ZSTD_DCtx_selectFrameDDict(zds);
            }
            let hSize = match hSize {
                Ok(size) => size,
                Err(err) => {
                    let legacyVersion = ZSTD_isLegacy(
                        istart as *const core::ffi::c_void,
                        iend.offset_from_unsigned(istart),
                    );
                    if legacyVersion != 0 {
                        let (dict, dictSize) = match ZSTD_getDDict(zds).as_ref() {
                            Some(ddict) => {
                                (ZSTD_DDict_dictContent(ddict), ZSTD_DDict_dictSize(ddict))
                            }
                            None => (core::ptr::null(), 0),
                        };
                        if zds.staticSize != 0 {
                            return Error::memory_allocation.to_error_code();
                        }
                        let err_code = ZSTD_initLegacyStream(
                            &mut zds.legacyContext,
                            zds.previousLegacyVersion,
                            legacyVersion,
                            dict,
                            dictSize,
                        );
                        if ERR_isError(err_code) {
                            return err_code;
                        }
                        zds.previousLegacyVersion = legacyVersion;
                        zds.legacyVersion = zds.previousLegacyVersion;
                        let hint = ZSTD_decompressLegacyStream(
                            zds.legacyContext,
                            legacyVersion,
                            output,
                            input,
                        );
                        if hint == 0 {
                            zds.streamStage = StreamStage::Init;
                        }
                        return hint;
                    }

                    return err.to_error_code();
                }
            };

            if hSize != 0 {
                // need more input
                let toLoad = hSize - zds.lhSize; // if hSize!=0, hSize > zds->lhSize
                let remainingInput = iend.offset_from_unsigned(ip);
                debug_assert!(iend >= ip);
                if toLoad > remainingInput {
                    // not enough input to load full header
                    if remainingInput > 0 {
                        core::ptr::copy_nonoverlapping(
                            ip.cast::<u8>(),
                            zds.headerBuffer.as_mut_ptr().add(zds.lhSize),
                            remainingInput,
                        );
                        zds.lhSize = (zds.lhSize).wrapping_add(remainingInput);
                    }
                    input.pos = input.size;
                    // check first few bytes
                    if let Err(err) = get_frame_header_advanced(
                        &mut zds.fParams,
                        &(&zds.headerBuffer)[..zds.lhSize],
                        zds.format,
                    ) {
                        return err.to_error_code();
                    };
                    // remaining header bytes + next block header
                    return Ord::max(ZSTD_FRAMEHEADERSIZE_MIN(zds.format), hSize)
                        .wrapping_sub(zds.lhSize)
                        .wrapping_add(ZSTD_blockHeaderSize);
                }
                debug_assert!(!ip.is_null());
                core::ptr::copy_nonoverlapping(
                    ip.cast::<u8>(),
                    zds.headerBuffer.as_mut_ptr().add(zds.lhSize),
                    toLoad,
                );
                zds.lhSize = hSize;
                ip = ip.add(toLoad);
                continue;
            } else {
                // check for single-pass mode opportunity
                if zds.fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
                    && zds.fParams.frameType != ZSTD_skippableFrame
                    && oend.offset_from_unsigned(op) as core::ffi::c_ulonglong
                        >= zds.fParams.frameContentSize
                {
                    let cSize = ZSTD_findFrameCompressedSize_advanced(
                        core::slice::from_raw_parts(
                            istart.cast(),
                            iend.offset_from_unsigned(istart),
                        ),
                        zds.format,
                    )
                    .unwrap_or_else(Error::to_error_code);

                    if cSize <= iend.offset_from_unsigned(istart) {
                        let decompressedSize = ZSTD_decompress_usingDDict(
                            zds,
                            op as *mut core::ffi::c_void,
                            oend.offset_from_unsigned(op),
                            istart as *const core::ffi::c_void,
                            cSize,
                            ZSTD_getDDict(zds),
                        );
                        if ERR_isError(decompressedSize) {
                            return decompressedSize;
                        }
                        debug_assert!(!istart.is_null());
                        ip = istart.add(cSize);
                        op = if !op.is_null() {
                            op.add(decompressedSize)
                        } else {
                            // can occur if frameContentSize = 0 (empty frame)
                            op
                        };
                        zds.expected = 0;
                        zds.streamStage = StreamStage::Init;
                        some_more_work = false;
                        continue;
                    }
                }

                // Check output buffer is large enough for ZSTD_odm_stable.
                if zds.outBufferMode == BufferMode::Stable
                    && zds.fParams.frameType != ZSTD_skippableFrame
                    && zds.fParams.frameContentSize != ZSTD_CONTENTSIZE_UNKNOWN
                    && (oend.offset_from_unsigned(op) as core::ffi::c_ulonglong)
                        < zds.fParams.frameContentSize
                {
                    return Error::dstSize_tooSmall.to_error_code();
                }
                let err_code = ZSTD_decompressBegin_usingDDict(zds, ZSTD_getDDict(zds));
                if ERR_isError(err_code) {
                    return err_code;
                }
                if zds.format == Format::ZSTD_f_zstd1 && is_skippable_frame(&(zds.headerBuffer)) {
                    // skippable frame
                    zds.expected = {
                        let [_, _, _, _, a, b, c, d, ..] = zds.headerBuffer;
                        u32::from_le_bytes([a, b, c, d]) as usize
                    };
                    zds.stage = DecompressStage::SkipFrame;
                } else {
                    let header_buffer = zds.headerBuffer;
                    if let Err(err) = ZSTD_decodeFrameHeader(zds, &header_buffer[..zds.lhSize]) {
                        return err.to_error_code();
                    }
                    zds.expected = ZSTD_blockHeaderSize;
                    zds.stage = DecompressStage::DecodeBlockHeader;
                }

                // control buffer memory usage
                zds.fParams.windowSize = Ord::max(
                    zds.fParams.windowSize,
                    (1 << ZSTD_WINDOWLOG_ABSOLUTEMIN) as core::ffi::c_ulonglong,
                );
                if zds.fParams.windowSize > zds.maxWindowSize as core::ffi::c_ulonglong {
                    return Error::frameParameter_windowTooLarge.to_error_code();
                }
                if zds.maxBlockSizeParam != 0 {
                    zds.fParams.blockSizeMax = core::cmp::min(
                        zds.fParams.blockSizeMax,
                        zds.maxBlockSizeParam as core::ffi::c_uint,
                    );
                }

                // Adapt buffer sizes to frame header instructions
                let neededInBuffSize = core::cmp::max(zds.fParams.blockSizeMax, 4) as size_t;
                let neededOutBuffSize = if zds.outBufferMode == BufferMode::Buffered {
                    ZSTD_decodingBufferSize_internal(
                        zds.fParams.windowSize,
                        zds.fParams.frameContentSize,
                        zds.fParams.blockSizeMax as size_t,
                    )
                } else {
                    0
                };

                ZSTD_DCtx_updateOversizedDuration(zds, neededInBuffSize, neededOutBuffSize);

                let tooSmall =
                    zds.inBuffSize < neededInBuffSize || zds.outBuffSize < neededOutBuffSize;
                let tooLarge = ZSTD_DCtx_isOversizedTooLong(zds);

                if tooSmall || tooLarge {
                    let bufferSize = neededInBuffSize.wrapping_add(neededOutBuffSize);
                    if zds.staticSize != 0 {
                        // static DCtx
                        debug_assert!(zds.staticSize >= size_of::<ZSTD_DCtx>()); // controlled at init
                        if bufferSize > zds.staticSize - size_of::<ZSTD_DCtx>() {
                            return Error::dictionary_corrupted.to_error_code();
                        }
                    } else {
                        ZSTD_customFree(
                            zds.inBuff as *mut core::ffi::c_void,
                            zds.inBuffSize,
                            zds.customMem,
                        );
                        zds.inBuffSize = 0;
                        zds.outBuffSize = 0;
                        zds.inBuff = ZSTD_customMalloc(bufferSize, zds.customMem).cast::<u8>();
                        if zds.inBuff.is_null() {
                            return Error::dictionary_corrupted.to_error_code();
                        }
                    }
                    zds.inBuffSize = neededInBuffSize;
                    zds.outBuff = (zds.inBuff).add(zds.inBuffSize);
                    zds.outBuffSize = neededOutBuffSize;
                }
                zds.streamStage = StreamStage::Read;
                current_block = Block::Read;
            }
        }

        if current_block == Block::Read {
            drop(current_block);

            let neededInSize =
                ZSTD_nextSrcSizeToDecompressWithInputSize(zds, iend.offset_from_unsigned(ip));
            if neededInSize == 0 {
                // end of frame
                zds.streamStage = StreamStage::Init;
                some_more_work = false;
                continue;
            } else if iend.offset_from_unsigned(ip) >= neededInSize {
                // decode directly from src
                let err_code = ZSTD_decompressContinueStream(
                    zds,
                    &mut op,
                    oend,
                    ip as *const core::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(err_code) {
                    return err_code;
                }
                debug_assert!(!ip.is_null());
                ip = ip.add(neededInSize);
                // Function modifies the stage so we must break
                continue;
            } else if ip == iend {
                // no more input
                some_more_work = false;
                continue;
            } else {
                zds.streamStage = StreamStage::Load;
                current_block = Block::Load;
            }
        }

        if current_block == Block::Load {
            drop(current_block);

            let neededInSize = zds.expected;
            let toLoad = neededInSize.wrapping_sub(zds.inPos);
            let isSkipFrame = matches!(zds.stage, DecompressStage::SkipFrame);
            // At this point we shouldn't be decompressing a block that we can stream.
            debug_assert_eq!(
                neededInSize,
                ZSTD_nextSrcSizeToDecompressWithInputSize(zds, iend.offset_from_unsigned(ip))
            );

            let loadedSize = if isSkipFrame {
                Ord::min(toLoad, iend.offset_from_unsigned(ip))
            } else {
                if toLoad > zds.inBuffSize.wrapping_sub(zds.inPos) {
                    return Error::corruption_detected.to_error_code();
                }
                ZSTD_limitCopy(
                    zds.inBuff.add(zds.inPos),
                    toLoad,
                    ip.cast::<u8>(),
                    iend.offset_from_unsigned(ip),
                )
            };

            ip = ip.add(loadedSize);
            zds.inPos = zds.inPos.wrapping_add(loadedSize);

            if loadedSize < toLoad {
                // not enough input, wait for more
                some_more_work = false;
            } else {
                // decode loaded input
                zds.inPos = 0; // input is consumed
                let err_code = ZSTD_decompressContinueStream(
                    zds,
                    &mut op,
                    oend,
                    zds.inBuff as *const core::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(err_code) {
                    return err_code;
                }
                // Function modifies the stage so we must break
            }
        }
    }

    // result
    input.pos = ip.byte_offset_from(input.src) as size_t;
    output.pos = op.byte_offset_from(output.dst) as size_t;

    // Update the expected output buffer for ZSTD_obm_stable.
    zds.expectedOutBuffer = *output;

    if ip == istart && op == ostart {
        // no forward progress
        zds.noForwardProgress += 1;
        if zds.noForwardProgress >= ZSTD_NO_FORWARD_PROGRESS_MAX {
            if op == oend {
                return Error::noForwardProgress_destFull.to_error_code();
            }
            if ip == iend {
                return Error::noForwardProgress_inputEmpty.to_error_code();
            }
            unreachable!();
        }
    } else {
        zds.noForwardProgress = 0;
    }

    let mut nextSrcSizeHint = zds.expected;
    if nextSrcSizeHint == 0 {
        // frame fully decoded
        if zds.outEnd == zds.outStart {
            // output fully flushed
            if zds.hostageByte != 0 {
                if input.pos >= input.size {
                    // can't release hostage (not present)
                    zds.streamStage = StreamStage::Read;
                    return 1;
                }
                // release hostage
                input.pos = (input.pos).wrapping_add(1);
            }
            return 0;
        }
        if zds.hostageByte == 0 {
            // output not fully flushed; keep last byte as hostage; will be
            // released when all output is flushed
            // note: pos > 0, otherwise, impossible to finish reading last block
            input.pos = (input.pos).wrapping_sub(1);
            zds.hostageByte = 1;
        }
        return 1;
    }
    // preload header of next block
    nextSrcSizeHint = nextSrcSizeHint.wrapping_add(
        ZSTD_blockHeaderSize * (zds.stage.to_next_input_type() == NextInputType::Block) as size_t,
    );
    debug_assert!(zds.inPos <= nextSrcSizeHint);
    // part already loaded
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub(zds.inPos);
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
        dst,
        size: dstCapacity,
        pos: *dstPos,
    };
    let mut input = ZSTD_inBuffer_s {
        src,
        size: srcSize,
        pos: *srcPos,
    };
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
