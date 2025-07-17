use ::libc;
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
#[no_mangle]
pub unsafe extern "C" fn ERR_getErrorString(mut code: ERR_enum) -> *const std::ffi::c_char {
    static mut notErrorCode: *const std::ffi::c_char =
        b"Unspecified error code\0" as *const u8 as *const std::ffi::c_char;
    match code as std::ffi::c_uint {
        0 => return b"No error detected\0" as *const u8 as *const std::ffi::c_char,
        1 => return b"Error (generic)\0" as *const u8 as *const std::ffi::c_char,
        10 => {
            return b"Unknown frame descriptor\0" as *const u8 as *const std::ffi::c_char;
        }
        12 => return b"Version not supported\0" as *const u8 as *const std::ffi::c_char,
        14 => {
            return b"Unsupported frame parameter\0" as *const u8 as *const std::ffi::c_char;
        }
        16 => {
            return b"Frame requires too much memory for decoding\0" as *const u8
                as *const std::ffi::c_char;
        }
        20 => {
            return b"Data corruption detected\0" as *const u8 as *const std::ffi::c_char;
        }
        22 => {
            return b"Restored data doesn't match checksum\0" as *const u8
                as *const std::ffi::c_char;
        }
        24 => {
            return b"Header of Literals' block doesn't respect format specification\0" as *const u8
                as *const std::ffi::c_char;
        }
        40 => return b"Unsupported parameter\0" as *const u8 as *const std::ffi::c_char,
        41 => {
            return b"Unsupported combination of parameters\0" as *const u8
                as *const std::ffi::c_char;
        }
        42 => {
            return b"Parameter is out of bound\0" as *const u8 as *const std::ffi::c_char;
        }
        62 => {
            return b"Context should be init first\0" as *const u8 as *const std::ffi::c_char;
        }
        64 => {
            return b"Allocation error : not enough memory\0" as *const u8
                as *const std::ffi::c_char;
        }
        66 => {
            return b"workSpace buffer is not large enough\0" as *const u8
                as *const std::ffi::c_char;
        }
        60 => {
            return b"Operation not authorized at current processing stage\0" as *const u8
                as *const std::ffi::c_char;
        }
        44 => {
            return b"tableLog requires too much memory : unsupported\0" as *const u8
                as *const std::ffi::c_char;
        }
        46 => {
            return b"Unsupported max Symbol Value : too large\0" as *const u8
                as *const std::ffi::c_char;
        }
        48 => {
            return b"Specified maxSymbolValue is too small\0" as *const u8
                as *const std::ffi::c_char;
        }
        49 => {
            return b"This mode cannot generate an uncompressed block\0" as *const u8
                as *const std::ffi::c_char;
        }
        50 => {
            return b"pledged buffer stability condition is not respected\0" as *const u8
                as *const std::ffi::c_char;
        }
        30 => return b"Dictionary is corrupted\0" as *const u8 as *const std::ffi::c_char,
        32 => return b"Dictionary mismatch\0" as *const u8 as *const std::ffi::c_char,
        34 => {
            return b"Cannot create Dictionary from provided samples\0" as *const u8
                as *const std::ffi::c_char;
        }
        70 => {
            return b"Destination buffer is too small\0" as *const u8 as *const std::ffi::c_char;
        }
        72 => return b"Src size is incorrect\0" as *const u8 as *const std::ffi::c_char,
        74 => {
            return b"Operation on NULL destination buffer\0" as *const u8
                as *const std::ffi::c_char;
        }
        80 => {
            return b"Operation made no progress over multiple calls, due to output buffer being full\0"
                as *const u8 as *const std::ffi::c_char;
        }
        82 => {
            return b"Operation made no progress over multiple calls, due to input being empty\0"
                as *const u8 as *const std::ffi::c_char;
        }
        100 => {
            return b"Frame index is too large\0" as *const u8 as *const std::ffi::c_char;
        }
        102 => {
            return b"An I/O error occurred when reading/seeking\0" as *const u8
                as *const std::ffi::c_char;
        }
        104 => {
            return b"Destination buffer is wrong\0" as *const u8 as *const std::ffi::c_char;
        }
        105 => return b"Source buffer is wrong\0" as *const u8 as *const std::ffi::c_char,
        106 => {
            return b"Block-level external sequence producer returned an error code\0" as *const u8
                as *const std::ffi::c_char;
        }
        107 => {
            return b"External sequences are not valid\0" as *const u8 as *const std::ffi::c_char;
        }
        120 | _ => return notErrorCode,
    };
}
