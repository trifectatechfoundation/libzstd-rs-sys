extern "C" {
    fn ERR_getErrorString(code: ERR_enum) -> *const std::ffi::c_char;
}
pub type ZSTD_ErrorCode = std::ffi::c_uint;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_externalSequences_invalid: ZSTD_ErrorCode = 107;
pub const ZSTD_error_sequenceProducer_failed: ZSTD_ErrorCode = 106;
pub const ZSTD_error_srcBuffer_wrong: ZSTD_ErrorCode = 105;
pub const ZSTD_error_dstBuffer_wrong: ZSTD_ErrorCode = 104;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ZSTD_ErrorCode = 82;
pub const ZSTD_error_noForwardProgress_destFull: ZSTD_ErrorCode = 80;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ZSTD_ErrorCode = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: ZSTD_ErrorCode = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_combination_unsupported: ZSTD_ErrorCode = 41;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_literals_headerWrong: ZSTD_ErrorCode = 24;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
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
