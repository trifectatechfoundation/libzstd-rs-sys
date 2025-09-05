use libc::size_t;

use crate::lib::common::error_private::{
    ERR_getErrorCode, ERR_getErrorName, ERR_getErrorString, ERR_isError,
};
use crate::lib::zstd::*;

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_versionNumber))]
pub const extern "C" fn ZSTD_versionNumber() -> core::ffi::c_uint {
    ZSTD_VERSION_NUMBER as core::ffi::c_uint
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_versionString))]
pub const extern "C" fn ZSTD_versionString() -> *const core::ffi::c_char {
    c"1.5.8".as_ptr()
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_isError))]
pub const extern "C" fn ZSTD_isError(code: size_t) -> core::ffi::c_uint {
    ERR_isError(code) as _
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getErrorName))]
pub extern "C" fn ZSTD_getErrorName(code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getErrorCode))]
pub extern "C" fn ZSTD_getErrorCode(code: size_t) -> ZSTD_ErrorCode {
    ERR_getErrorCode(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_getErrorString))]
pub extern "C" fn ZSTD_getErrorString(code: ZSTD_ErrorCode) -> *const core::ffi::c_char {
    ERR_getErrorString(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_isDeterministicBuild))]
pub extern "C" fn ZSTD_isDeterministicBuild() -> core::ffi::c_int {
    true as core::ffi::c_int
}
