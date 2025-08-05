use core::ffi::c_char;
use std::ffi::c_uint;

use crate::lib::zstd::{ZSTD_ErrorCode, ZSTD_error_maxCode};
use crate::size_t;

pub type ERR_enum = ZSTD_ErrorCode;

pub const fn ERR_isError(mut code: size_t) -> c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as c_uint
}

pub const fn ERR_getErrorCode(mut code: size_t) -> ZSTD_ErrorCode {
    if ERR_isError(code) == 0 {
        return 0;
    }

    (0 as size_t).wrapping_sub(code) as _
}

#[export_name = crate::prefix!(ERR_getErrorString)]
pub unsafe extern "C" fn ERR_getErrorString(mut code: ERR_enum) -> *const c_char {
    static mut notErrorCode: *const c_char =
        b"Unspecified error code\0" as *const u8 as *const c_char;
    match code as core::ffi::c_uint {
        0 => b"No error detected\0" as *const u8 as *const c_char,
        1 => b"Error (generic)\0" as *const u8 as *const c_char,
        10 => b"Unknown frame descriptor\0" as *const u8 as *const c_char,
        12 => b"Version not supported\0" as *const u8 as *const c_char,
        14 => b"Unsupported frame parameter\0" as *const u8 as *const c_char,
        16 => b"Frame requires too much memory for decoding\0" as *const u8 as *const c_char,
        20 => b"Data corruption detected\0" as *const u8 as *const c_char,
        22 => b"Restored data doesn't match checksum\0" as *const u8 as *const c_char,
        24 => {
            b"Header of Literals' block doesn't respect format specification\0" as *const u8
                as *const c_char
        }
        40 => b"Unsupported parameter\0" as *const u8 as *const c_char,
        41 => b"Unsupported combination of parameters\0" as *const u8 as *const c_char,
        42 => b"Parameter is out of bound\0" as *const u8 as *const c_char,
        62 => b"Context should be init first\0" as *const u8 as *const c_char,
        64 => b"Allocation error : not enough memory\0" as *const u8 as *const c_char,
        66 => b"workSpace buffer is not large enough\0" as *const u8 as *const c_char,
        60 => {
            b"Operation not authorized at current processing stage\0" as *const u8 as *const c_char
        }
        44 => b"tableLog requires too much memory : unsupported\0" as *const u8 as *const c_char,
        46 => b"Unsupported max Symbol Value : too large\0" as *const u8 as *const c_char,
        48 => b"Specified maxSymbolValue is too small\0" as *const u8 as *const c_char,
        49 => b"This mode cannot generate an uncompressed block\0" as *const u8 as *const c_char,
        50 => {
            b"pledged buffer stability condition is not respected\0" as *const u8 as *const c_char
        }
        30 => b"Dictionary is corrupted\0" as *const u8 as *const c_char,
        32 => b"Dictionary mismatch\0" as *const u8 as *const c_char,
        34 => b"Cannot create Dictionary from provided samples\0" as *const u8 as *const c_char,
        70 => b"Destination buffer is too small\0" as *const u8 as *const c_char,
        72 => b"Src size is incorrect\0" as *const u8 as *const c_char,
        74 => b"Operation on NULL destination buffer\0" as *const u8 as *const c_char,
        80 => {
            b"Operation made no progress over multiple calls, due to output buffer being full\0"
                as *const u8 as *const c_char
        }
        82 => {
            b"Operation made no progress over multiple calls, due to input being empty\0"
                as *const u8 as *const c_char
        }
        100 => b"Frame index is too large\0" as *const u8 as *const c_char,
        102 => b"An I/O error occurred when reading/seeking\0" as *const u8 as *const c_char,
        104 => b"Destination buffer is wrong\0" as *const u8 as *const c_char,
        105 => b"Source buffer is wrong\0" as *const u8 as *const c_char,
        106 => {
            b"Block-level external sequence producer returned an error code\0" as *const u8
                as *const c_char
        }
        107 => b"External sequences are not valid\0" as *const u8 as *const c_char,
        120 | _ => notErrorCode,
    }
}

pub unsafe fn ERR_getErrorName(mut code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorString(ERR_getErrorCode(code))
}
