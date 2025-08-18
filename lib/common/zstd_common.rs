use libc::size_t;

use crate::lib::common::error_private::{
    ERR_getErrorCode, ERR_getErrorName, ERR_getErrorString, ERR_isError,
};
use crate::lib::zstd::*;

pub type ERR_enum = ZSTD_ErrorCode;
pub const ZSTD_VERSION_MAJOR: core::ffi::c_int = 1;
pub const ZSTD_VERSION_MINOR: core::ffi::c_int = 5;
pub const ZSTD_VERSION_RELEASE: core::ffi::c_int = 8;
pub const ZSTD_VERSION_NUMBER: core::ffi::c_int =
    ZSTD_VERSION_MAJOR * 100 * 100 + ZSTD_VERSION_MINOR * 100 + ZSTD_VERSION_RELEASE;
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_versionNumber))]
pub unsafe extern "C" fn ZSTD_versionNumber() -> core::ffi::c_uint {
    ZSTD_VERSION_NUMBER as core::ffi::c_uint
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_versionString))]
pub unsafe extern "C" fn ZSTD_versionString() -> *const core::ffi::c_char {
    b"1.5.8\0" as *const u8 as *const core::ffi::c_char
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_isError))]
pub unsafe extern "C" fn ZSTD_isError(code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getErrorName))]
pub unsafe extern "C" fn ZSTD_getErrorName(code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getErrorCode))]
pub unsafe extern "C" fn ZSTD_getErrorCode(code: size_t) -> ZSTD_ErrorCode {
    ERR_getErrorCode(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getErrorString))]
pub unsafe extern "C" fn ZSTD_getErrorString(code: ZSTD_ErrorCode) -> *const core::ffi::c_char {
    ERR_getErrorString(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_isDeterministicBuild))]
pub unsafe extern "C" fn ZSTD_isDeterministicBuild() -> core::ffi::c_int {
    1
}
