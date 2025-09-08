use core::ptr::NonNull;
use libc::{free, malloc, size_t};

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::decompress::huf_decompress::DTableDesc;
use crate::lib::decompress::zstd_decompress::ZSTD_loadDEntropy;
use crate::lib::decompress::{ZSTD_DCtx, ZSTD_entropyDTables_t};
use crate::lib::zstd::*;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultipleDDicts {
    Single = 0,
    Multiple = 1,
}

impl TryFrom<u32> for MultipleDDicts {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Single),
            1 => Ok(Self::Multiple),
            _ => Err(()),
        }
    }
}

#[repr(C)]
pub struct ZSTD_DDictHashSet {
    pub ddictPtrTable: *mut *const ZSTD_DDict,
    pub ddictPtrTableSize: size_t,
    pub ddictPtrCount: size_t,
}
#[repr(C)]
pub struct ZSTD_DDict {
    dictBuffer: *mut core::ffi::c_void,
    dictContent: *const core::ffi::c_void,
    dictSize: size_t,
    entropy: ZSTD_entropyDTables_t,
    dictID: u32,
    entropyPresent: u32,
    cMem: ZSTD_customMem,
}

impl ZSTD_DDict {
    pub fn as_slice(&self) -> &[u8] {
        if self.dictContent.is_null() {
            debug_assert_eq!(self.dictSize, 0);
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.dictContent.cast::<u8>(), self.dictSize) }
        }
    }
}

pub type ZSTD_dictContentType_e = core::ffi::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DictLoadMethod {
    ByCopy = 0,
    ByRef = 1,
}

pub type ZSTD_dictLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = DictLoadMethod::ByCopy as _;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = DictLoadMethod::ByRef as _;

pub const ZSTD_MAGIC_DICTIONARY: core::ffi::c_uint = 0xec30a437 as core::ffi::c_uint;

#[inline]
unsafe extern "C" fn ZSTD_customMalloc(
    size: size_t,
    customMem: ZSTD_customMem,
) -> *mut core::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
    }
    malloc(size)
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(ptr: *mut core::ffi::c_void, customMem: ZSTD_customMem) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree).unwrap_unchecked()(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}
pub unsafe fn ZSTD_DDict_dictContent(ddict: *const ZSTD_DDict) -> *const core::ffi::c_void {
    (*ddict).dictContent
}
pub unsafe fn ZSTD_DDict_dictSize(ddict: *const ZSTD_DDict) -> size_t {
    (*ddict).dictSize
}
pub unsafe fn ZSTD_copyDDictParameters(dctx: *mut ZSTD_DCtx, ddict: *const ZSTD_DDict) {
    (*dctx).dictID = (*ddict).dictID;
    (*dctx).prefixStart = (*ddict).dictContent;
    (*dctx).virtualStart = (*ddict).dictContent;
    (*dctx).dictEnd =
        ((*ddict).dictContent as *const u8).add((*ddict).dictSize) as *const core::ffi::c_void;
    (*dctx).previousDstEnd = (*dctx).dictEnd;
    if (*ddict).entropyPresent != 0 {
        (*dctx).litEntropy = 1;
        (*dctx).fseEntropy = 1;
        (*dctx).LLTptr = NonNull::new((&raw const (*ddict).entropy.LLTable).cast_mut());
        (*dctx).MLTptr = NonNull::new((&raw const (*ddict).entropy.MLTable).cast_mut());
        (*dctx).OFTptr = NonNull::new((&raw const (*ddict).entropy.OFTable).cast_mut());
        (*dctx).HUFptr = &raw const (*ddict).entropy.hufTable;
        *((*dctx).entropy.rep).as_mut_ptr().offset(0) = *((*ddict).entropy.rep).as_ptr().offset(0);
        *((*dctx).entropy.rep).as_mut_ptr().offset(1) = *((*ddict).entropy.rep).as_ptr().offset(1);
        *((*dctx).entropy.rep).as_mut_ptr().offset(2) = *((*ddict).entropy.rep).as_ptr().offset(2);
    } else {
        (*dctx).litEntropy = 0;
        (*dctx).fseEntropy = 0;
    };
}
unsafe fn ZSTD_loadEntropy_intoDDict(
    ddict: *mut ZSTD_DDict,
    dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    (*ddict).dictID = 0;
    (*ddict).entropyPresent = 0;

    if dictContentType == ZSTD_dct_rawContent as ZSTD_dictContentType_e {
        return 0;
    }

    let dict = if (*ddict).dictContent.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts((*ddict).dictContent.cast::<u8>(), (*ddict).dictSize)
    };

    let ([magic, dict_id, ..], _) = dict.as_chunks::<4>() else {
        if dictContentType == ZSTD_dct_fullDict as ZSTD_dictContentType_e {
            return Error::dictionary_corrupted.to_error_code();
        }

        return 0;
    };

    let magic = u32::from_le_bytes(*magic);
    if magic != ZSTD_MAGIC_DICTIONARY {
        if dictContentType == ZSTD_dct_fullDict as ZSTD_dictContentType_e {
            return Error::dictionary_corrupted.to_error_code();
        }

        return 0;
    }

    (*ddict).dictID = u32::from_le_bytes(*dict_id);

    let ret = ZSTD_loadDEntropy(&mut (*ddict).entropy, dict);

    if ERR_isError(ret) {
        return Error::dictionary_corrupted.to_error_code();
    }

    (*ddict).entropyPresent = 1;

    0
}

unsafe fn ZSTD_initDDict_internal(
    ddict: *mut ZSTD_DDict,
    dict: *const core::ffi::c_void,
    mut dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if dictLoadMethod == DictLoadMethod::ByRef as ZSTD_dictLoadMethod_e
        || dict.is_null()
        || dictSize == 0
    {
        (*ddict).dictBuffer = core::ptr::null_mut();
        (*ddict).dictContent = dict;
        if dict.is_null() {
            dictSize = 0;
        }
    } else {
        let internalBuffer = ZSTD_customMalloc(dictSize, (*ddict).cMem);
        (*ddict).dictBuffer = internalBuffer;
        (*ddict).dictContent = internalBuffer;
        if internalBuffer.is_null() {
            return Error::dictionary_corrupted.to_error_code();
        }
        libc::memcpy(internalBuffer, dict, dictSize as libc::size_t);
    }

    (*ddict).dictSize = dictSize;
    (*ddict).entropy.hufTable.description = DTableDesc::from_u32(12 * 0x1000001);

    let err_code = ZSTD_loadEntropy_intoDDict(ddict, dictContentType);
    if ERR_isError(err_code) {
        return err_code;
    }

    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDDict_advanced))]
pub unsafe extern "C" fn ZSTD_createDDict_advanced(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
    customMem: ZSTD_customMem,
) -> *mut ZSTD_DDict {
    if customMem.customAlloc.is_none() ^ customMem.customFree.is_none() {
        return core::ptr::null_mut();
    }

    let ddict = ZSTD_customMalloc(size_of::<ZSTD_DDict>(), customMem) as *mut ZSTD_DDict;

    if ddict.is_null() {
        return core::ptr::null_mut();
    }

    (*ddict).cMem = customMem;
    let initResult =
        ZSTD_initDDict_internal(ddict, dict, dictSize, dictLoadMethod, dictContentType);

    if ERR_isError(initResult) {
        ZSTD_freeDDict(ddict);
        return core::ptr::null_mut();
    }

    ddict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDDict))]
pub unsafe extern "C" fn ZSTD_createDDict(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> *mut ZSTD_DDict {
    let allocator = ZSTD_customMem {
        customAlloc: None,
        customFree: None,
        opaque: core::ptr::null_mut(),
    };
    ZSTD_createDDict_advanced(dict, dictSize, ZSTD_dlm_byCopy, ZSTD_dct_auto, allocator)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDDict_byReference))]
pub unsafe extern "C" fn ZSTD_createDDict_byReference(
    dictBuffer: *const core::ffi::c_void,
    dictSize: size_t,
) -> *mut ZSTD_DDict {
    let allocator = {
        ZSTD_customMem {
            customAlloc: None,
            customFree: None,
            opaque: core::ptr::null_mut(),
        }
    };
    ZSTD_createDDict_advanced(
        dictBuffer,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
        allocator,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_initStaticDDict))]
pub unsafe extern "C" fn ZSTD_initStaticDDict(
    sBuffer: *mut core::ffi::c_void,
    sBufferSize: size_t,
    mut dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
) -> *const ZSTD_DDict {
    if sBuffer as usize & 0b111 != 0 {
        return core::ptr::null_mut();
    }

    if sBufferSize < ZSTD_estimateDDictSize(dictSize, dictLoadMethod) {
        return core::ptr::null_mut();
    }

    let ddict = sBuffer as *mut ZSTD_DDict;
    if dictLoadMethod == DictLoadMethod::ByCopy as ZSTD_dictLoadMethod_e {
        libc::memcpy(ddict.add(1) as *mut core::ffi::c_void, dict, dictSize);
        dict = ddict.add(1) as *const core::ffi::c_void;
    }

    let ret = ZSTD_initDDict_internal(
        ddict,
        dict,
        dictSize,
        DictLoadMethod::ByRef as _,
        dictContentType,
    );

    if ERR_isError(ret) {
        return core::ptr::null_mut();
    }

    ddict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeDDict))]
pub unsafe extern "C" fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> size_t {
    if ddict.is_null() {
        return 0;
    }
    let cMem = (*ddict).cMem;
    ZSTD_customFree((*ddict).dictBuffer, cMem);
    ZSTD_customFree(ddict as *mut core::ffi::c_void, cMem);
    0
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_estimateDDictSize))]
pub const extern "C" fn ZSTD_estimateDDictSize(
    dict_size: size_t,
    dict_load_method: ZSTD_dictLoadMethod_e,
) -> size_t {
    if dict_load_method == ZSTD_dlm_byRef as ZSTD_dictLoadMethod_e {
        size_of::<ZSTD_DDict>()
    } else {
        size_of::<ZSTD_DDict>() + dict_size
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_sizeof_DDict))]
pub unsafe extern "C" fn ZSTD_sizeof_DDict(ddict: *const ZSTD_DDict) -> size_t {
    if ddict.is_null() {
        return 0;
    }
    (::core::mem::size_of::<ZSTD_DDict>()).wrapping_add(if !((*ddict).dictBuffer).is_null() {
        (*ddict).dictSize
    } else {
        0
    })
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getDictID_fromDDict))]
pub unsafe extern "C" fn ZSTD_getDictID_fromDDict(ddict: *const ZSTD_DDict) -> core::ffi::c_uint {
    if ddict.is_null() {
        return 0;
    }
    (*ddict).dictID
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_ddict_size() {
        assert_eq!(
            ZSTD_estimateDDictSize(1234, ZSTD_dlm_byCopy),
            size_of::<ZSTD_DDict>() + 1234
        );
        assert_eq!(
            ZSTD_estimateDDictSize(1234, ZSTD_dlm_byRef),
            size_of::<ZSTD_DDict>()
        );
    }
}
