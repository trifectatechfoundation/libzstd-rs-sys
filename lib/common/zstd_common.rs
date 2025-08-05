use crate::lib::common::error_private::ERR_getErrorString;
use crate::lib::zstd::*;

pub type ERR_enum = ZSTD_ErrorCode;
pub type size_t = core::ffi::c_ulong;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> core::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as core::ffi::c_int) as size_t) as core::ffi::c_int
        as core::ffi::c_uint
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    (0 as core::ffi::c_int as size_t).wrapping_sub(code) as ERR_enum
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorString(ERR_getErrorCode(code))
}
pub const ZSTD_VERSION_MAJOR: core::ffi::c_int = 1;
pub const ZSTD_VERSION_MINOR: core::ffi::c_int = 5;
pub const ZSTD_VERSION_RELEASE: core::ffi::c_int = 8;
pub const ZSTD_VERSION_NUMBER: core::ffi::c_int =
    ZSTD_VERSION_MAJOR * 100 * 100 + ZSTD_VERSION_MINOR * 100 + ZSTD_VERSION_RELEASE;
#[export_name = crate::prefix!(ZSTD_versionNumber)]
pub unsafe extern "C" fn ZSTD_versionNumber() -> core::ffi::c_uint {
    ZSTD_VERSION_NUMBER as core::ffi::c_uint
}
#[export_name = crate::prefix!(ZSTD_versionString)]
pub unsafe extern "C" fn ZSTD_versionString() -> *const core::ffi::c_char {
    b"1.5.8\0" as *const u8 as *const core::ffi::c_char
}
#[export_name = crate::prefix!(ZSTD_isError)]
pub unsafe extern "C" fn ZSTD_isError(mut code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
#[export_name = crate::prefix!(ZSTD_getErrorName)]
pub unsafe extern "C" fn ZSTD_getErrorName(mut code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
#[export_name = crate::prefix!(ZSTD_getErrorCode)]
pub unsafe extern "C" fn ZSTD_getErrorCode(mut code: size_t) -> ZSTD_ErrorCode {
    ERR_getErrorCode(code)
}
#[export_name = crate::prefix!(ZSTD_getErrorString)]
pub unsafe extern "C" fn ZSTD_getErrorString(mut code: ZSTD_ErrorCode) -> *const core::ffi::c_char {
    ERR_getErrorString(code)
}
#[export_name = crate::prefix!(ZSTD_isDeterministicBuild)]
pub unsafe extern "C" fn ZSTD_isDeterministicBuild() -> core::ffi::c_int {
    1
}
