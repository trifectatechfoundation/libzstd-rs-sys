use core::mem::MaybeUninit;
use core::ptr::NonNull;
use libc::size_t;

use crate::lib::common::allocations::{ZSTD_customFree, ZSTD_customMalloc};
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

impl ZSTD_DDictHashSet {
    pub fn as_slice(&mut self) -> &[*const ZSTD_DDict] {
        unsafe { core::slice::from_raw_parts(self.ddictPtrTable, self.ddictPtrCount) }
    }
}

#[repr(C)]
pub struct ZSTD_DDict {
    dictBuffer: *mut core::ffi::c_void,
    dictContent: *const core::ffi::c_void,
    dictSize: size_t,
    entropy: ZSTD_entropyDTables_t,
    pub(crate) dictID: u32,
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
/// Refuses to load a dictionary if it does not respect Zstandard's specification, starting with
/// [`ZSTD_MAGIC_DICTIONARY`]
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
/// Ensures dictionary is always loaded as `rawContent`, even if it starts with [`ZSTD_MAGIC_DICTIONARY`]
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
/// Dictionary is "full" when starting with [`ZSTD_MAGIC_DICTIONARY`], otherwise it is "rawContent"
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DictLoadMethod {
    ByCopy = 0,
    ByRef = 1,
}

pub type ZSTD_dictLoadMethod_e = core::ffi::c_uint;
/// Copy dictionary content internally
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = DictLoadMethod::ByCopy as _;
/// Reference dictionary content -- the dictionary buffer must outlive its users
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = DictLoadMethod::ByRef as _;

pub const ZSTD_MAGIC_DICTIONARY: core::ffi::c_uint = 0xec30a437 as core::ffi::c_uint;

pub fn ZSTD_DDict_dictContent(ddict: &ZSTD_DDict) -> *const core::ffi::c_void {
    ddict.dictContent
}

pub fn ZSTD_DDict_dictSize(ddict: &ZSTD_DDict) -> size_t {
    ddict.dictSize
}

pub fn ZSTD_copyDDictParameters(dctx: &mut MaybeUninit<ZSTD_DCtx>, ddict: &ZSTD_DDict) {
    let dctx = dctx.as_mut_ptr();

    // SAFETY: we only write to the raw pointer, never read from it. The types guarantee that the
    // writes are in-bounds and that we are allowed to write to this memory.
    unsafe {
        (*dctx).dictID = ddict.dictID;
        (*dctx).prefixStart = ddict.dictContent;
        (*dctx).virtualStart = ddict.dictContent;
        (*dctx).dictEnd = (ddict.dictContent).wrapping_byte_add(ddict.dictSize);
        (*dctx).previousDstEnd = (*dctx).dictEnd;

        if ddict.entropyPresent != 0 {
            (*dctx).litEntropy = true;
            (*dctx).fseEntropy = true;
            (*dctx).LLTptr = NonNull::new((&raw const ddict.entropy.LLTable).cast_mut());
            (*dctx).MLTptr = NonNull::new((&raw const ddict.entropy.MLTable).cast_mut());
            (*dctx).OFTptr = NonNull::new((&raw const ddict.entropy.OFTable).cast_mut());
            (*dctx).HUFptr = NonNull::new((&raw const ddict.entropy.hufTable).cast_mut());
            (*dctx).entropy.rep = ddict.entropy.rep;
        } else {
            (*dctx).litEntropy = false;
            (*dctx).fseEntropy = false;
        };
    }
}

fn ZSTD_loadEntropy_intoDDict(
    ddict: &mut ZSTD_DDict,
    dictContentType: ZSTD_dictContentType_e,
) -> Result<(), Error> {
    ddict.dictID = 0;
    ddict.entropyPresent = 0;

    if dictContentType == ZSTD_dct_rawContent as ZSTD_dictContentType_e {
        return Ok(());
    }

    let dict = if ddict.dictContent.is_null() {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(ddict.dictContent.cast::<u8>(), ddict.dictSize) }
    };

    let ([magic, dict_id, ..], _) = dict.as_chunks::<4>() else {
        if dictContentType == ZSTD_dct_fullDict as ZSTD_dictContentType_e {
            return Err(Error::dictionary_corrupted);
        }

        return Ok(()); // pure content mode
    };

    let magic = u32::from_le_bytes(*magic);
    if magic != ZSTD_MAGIC_DICTIONARY {
        if dictContentType == ZSTD_dct_fullDict as ZSTD_dictContentType_e {
            return Err(Error::dictionary_corrupted);
        }

        return Ok(()); // pure content mode
    }

    ddict.dictID = u32::from_le_bytes(*dict_id);

    let ret = ZSTD_loadDEntropy(&mut ddict.entropy, dict);

    if ERR_isError(ret) {
        return Err(Error::dictionary_corrupted);
    }

    ddict.entropyPresent = 1;

    Ok(())
}

fn ZSTD_initDDict_internal(
    ddict: &mut ZSTD_DDict,
    dict: *const core::ffi::c_void,
    mut dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
) -> Result<(), Error> {
    if dictLoadMethod == DictLoadMethod::ByRef as ZSTD_dictLoadMethod_e
        || dict.is_null()
        || dictSize == 0
    {
        ddict.dictBuffer = core::ptr::null_mut();
        ddict.dictContent = dict;
        if dict.is_null() {
            dictSize = 0;
        }
    } else {
        unsafe {
            let internalBuffer = ZSTD_customMalloc(dictSize, ddict.cMem);
            ddict.dictBuffer = internalBuffer;
            ddict.dictContent = internalBuffer;
            if internalBuffer.is_null() {
                return Err(Error::dictionary_corrupted);
            }
            core::ptr::copy_nonoverlapping(dict, internalBuffer, dictSize);
        }
    }

    ddict.dictSize = dictSize;
    ddict.entropy.hufTable.description = DTableDesc::default();

    ZSTD_loadEntropy_intoDDict(ddict, dictContentType)?;

    Ok(())
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDDict_advanced))]
pub unsafe extern "C" fn ZSTD_createDDict_advanced(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictLoadMethod: ZSTD_dictLoadMethod_e,
    dictContentType: ZSTD_dictContentType_e,
    customMem: ZSTD_customMem,
) -> *mut ZSTD_DDict {
    let ddict = ZSTD_customMalloc(size_of::<ZSTD_DDict>(), customMem) as *mut ZSTD_DDict;

    if ddict.is_null() {
        return core::ptr::null_mut();
    }

    (*ddict).cMem = customMem;
    if ZSTD_initDDict_internal(
        ddict.as_mut().unwrap(),
        dict,
        dictSize,
        dictLoadMethod,
        dictContentType,
    )
    .is_err()
    {
        ZSTD_freeDDict(ddict);
        return core::ptr::null_mut();
    }

    ddict
}

/// Create a digested dictionary, to start decompression without startup delay.
///
/// The `dict`'s content is copied inside the [`ZSTD_DDict`], so `dict` can be released after
/// [`ZSTD_DDict`] creation.
///
/// The DDict can be freed using [`ZSTD_freeDDict`].
///
/// # Returns
///
/// - a [`ZSTD_DDict`] if it was successfully created
/// - NULL if there was an error creating the dictionary
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDDict))]
pub unsafe extern "C" fn ZSTD_createDDict(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> *mut ZSTD_DDict {
    ZSTD_createDDict_advanced(
        dict,
        dictSize,
        ZSTD_dlm_byCopy,
        ZSTD_dct_auto,
        ZSTD_customMem::default(),
    )
}

/// Create a digested dictionary, to start decompression without startup delay.
///
/// Dictionary content is simply referenced, it will be accessed during decompression.
/// `dictBuffer` must outlive [`ZSTD_DDict`] ([`ZSTD_DDict`] must be freed before `dictBuffer`)
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createDDict_byReference))]
pub unsafe extern "C" fn ZSTD_createDDict_byReference(
    dictBuffer: *const core::ffi::c_void,
    dictSize: size_t,
) -> *mut ZSTD_DDict {
    ZSTD_createDDict_advanced(
        dictBuffer,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
        ZSTD_customMem::default(),
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
    debug_assert!(!sBuffer.is_null());
    debug_assert!(!dict.is_null());

    // sBuffer should be 8-aligned
    if sBuffer as usize & 0b111 != 0 {
        return core::ptr::null_mut();
    }

    if sBufferSize < ZSTD_estimateDDictSize(dictSize, dictLoadMethod) {
        return core::ptr::null_mut();
    }

    let ddict = sBuffer as *mut ZSTD_DDict;
    if dictLoadMethod == DictLoadMethod::ByCopy as ZSTD_dictLoadMethod_e {
        core::ptr::copy_nonoverlapping(dict.cast::<u8>(), ddict.add(1).cast::<u8>(), dictSize); // local copy
        dict = ddict.add(1) as *const core::ffi::c_void;
    }

    if ZSTD_initDDict_internal(
        ddict.as_mut().unwrap(),
        dict,
        dictSize,
        DictLoadMethod::ByRef as _,
        dictContentType,
    )
    .is_err()
    {
        return core::ptr::null_mut();
    }

    ddict
}

/// Free the memory allocated with [`ZSTD_createDDict`].
///
/// If a NULL pointer is passed, no operation is performed.
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeDDict))]
pub unsafe extern "C" fn ZSTD_freeDDict(ddict: *mut ZSTD_DDict) -> size_t {
    if ddict.is_null() {
        return 0;
    }
    let cMem = (*ddict).cMem;
    ZSTD_customFree((*ddict).dictBuffer, (*ddict).dictSize, cMem);
    ZSTD_customFree(
        ddict as *mut core::ffi::c_void,
        size_of::<ZSTD_DDict>(),
        cMem,
    );
    0
}

/// Estimate amount of memory that will be needed to create a dictionary for decompression.
///
/// Note: dictionary created by reference using [`ZSTD_dlm_byRef`] are smaller
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

/// Get the _current_ memory usage of the [`ZSTD_DDict`]
///
/// # Returns
///
/// - the size of the DDict, including the size of the DDict's `dictBuffer` if present
/// - 0 if the `ddict` is NULL
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

/// Provides the `dictID` of the dictionary loaded into [`ZSTD_DDict`].
///
/// If it returns 0, the dictionary is not conformant to Zstandard specification, or empty.
/// Non-conformant dictionaries can still be loaded, but as content-only dictionaries.
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
