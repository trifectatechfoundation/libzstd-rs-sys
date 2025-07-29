use crate::lib::common::error_private::ERR_getErrorString;
use crate::lib::zstd::*;

pub type ERR_enum = ZSTD_ErrorCode;
pub type size_t = std::ffi::c_ulong;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    (0 as std::ffi::c_int as size_t).wrapping_sub(code) as ERR_enum
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorString(ERR_getErrorCode(code))
}
pub const ZSTD_VERSION_MAJOR: std::ffi::c_int = 1 as std::ffi::c_int;
pub const ZSTD_VERSION_MINOR: std::ffi::c_int = 5 as std::ffi::c_int;
pub const ZSTD_VERSION_RELEASE: std::ffi::c_int = 8 as std::ffi::c_int;
pub const ZSTD_VERSION_NUMBER: std::ffi::c_int =
    ZSTD_VERSION_MAJOR * 100 as std::ffi::c_int * 100 as std::ffi::c_int
        + ZSTD_VERSION_MINOR * 100 as std::ffi::c_int
        + ZSTD_VERSION_RELEASE;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_versionNumber() -> std::ffi::c_uint {
    ZSTD_VERSION_NUMBER as std::ffi::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_versionString() -> *const std::ffi::c_char {
    b"1.5.8\0" as *const u8 as *const std::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isError(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(code)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getErrorCode(mut code: size_t) -> ZSTD_ErrorCode {
    ERR_getErrorCode(code)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getErrorString(mut code: ZSTD_ErrorCode) -> *const std::ffi::c_char {
    ERR_getErrorString(code)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_isDeterministicBuild() -> std::ffi::c_int {
    1 as std::ffi::c_int
}
