use core::ffi::c_char;

use libc::size_t;

use crate::lib::zstd::{ZSTD_ErrorCode, ZSTD_error_maxCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[allow(clippy::upper_case_acronyms)]
    GENERIC = 1,
    prefix_unknown = 10,
    version_unsupported = 12,
    frameParameter_unsupported = 14,
    frameParameter_windowTooLarge = 16,
    corruption_detected = 20,
    checksum_wrong = 22,
    literals_headerWrong = 24,
    dictionary_corrupted = 30,
    dictionary_wrong = 32,
    dictionaryCreation_failed = 34,
    parameter_unsupported = 40,
    parameter_combination_unsupported = 41,
    parameter_outOfBound = 42,
    tableLog_tooLarge = 44,
    maxSymbolValue_tooLarge = 46,
    maxSymbolValue_tooSmall = 48,
    cannotProduce_uncompressedBlock = 49,
    stabilityCondition_notRespected = 50,
    stage_wrong = 60,
    init_missing = 62,
    memory_allocation = 64,
    workSpace_tooSmall = 66,
    dstSize_tooSmall = 70,
    srcSize_wrong = 72,
    dstBuffer_null = 74,
    noForwardProgress_destFull = 80,
    noForwardProgress_inputEmpty = 82,
    frameIndex_tooLarge = 100,
    seekableIO = 102,
    dstBuffer_wrong = 104,
    srcBuffer_wrong = 105,
    sequenceProducer_failed = 106,
    externalSequences_invalid = 107,
    maxCode = 120,
}

impl Error {
    pub fn to_error_code(self) -> size_t {
        -(self as core::ffi::c_int) as size_t
    }

    pub fn from_error_code(code: size_t) -> Option<Self> {
        if !ERR_isError(code) {
            return None;
        }

        Self::try_from(code.wrapping_neg() as u32).ok()
    }
}

impl TryFrom<u32> for Error {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use Error::*;

        Ok(match value {
            1 => GENERIC,
            10 => prefix_unknown,
            12 => version_unsupported,
            14 => frameParameter_unsupported,
            16 => frameParameter_windowTooLarge,
            20 => corruption_detected,
            22 => checksum_wrong,
            24 => literals_headerWrong,
            30 => dictionary_corrupted,
            32 => dictionary_wrong,
            34 => dictionaryCreation_failed,
            40 => parameter_unsupported,
            41 => parameter_combination_unsupported,
            42 => parameter_outOfBound,
            44 => tableLog_tooLarge,
            46 => maxSymbolValue_tooLarge,
            48 => maxSymbolValue_tooSmall,
            49 => cannotProduce_uncompressedBlock,
            50 => stabilityCondition_notRespected,
            60 => stage_wrong,
            62 => init_missing,
            64 => memory_allocation,
            66 => workSpace_tooSmall,
            70 => dstSize_tooSmall,
            72 => srcSize_wrong,
            74 => dstBuffer_null,
            80 => noForwardProgress_destFull,
            82 => noForwardProgress_inputEmpty,
            100 => frameIndex_tooLarge,
            102 => seekableIO,
            104 => dstBuffer_wrong,
            105 => srcBuffer_wrong,
            106 => sequenceProducer_failed,
            107 => externalSequences_invalid,
            120 => maxCode,
            _ => return Err(()),
        })
    }
}

type ERR_enum = ZSTD_ErrorCode;

pub(crate) const fn ERR_isError(code: size_t) -> bool {
    code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t
}

pub(crate) const fn ERR_getErrorCode(code: size_t) -> ZSTD_ErrorCode {
    if !ERR_isError(code) {
        return 0;
    }

    code.wrapping_neg() as _
}

pub(crate) fn ERR_getErrorString(code: ERR_enum) -> *const c_char {
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
        120 => b"Unspecified error code\0" as *const u8 as *const c_char,
        _ => b"Unspecified error code\0" as *const u8 as *const c_char,
    }
}

pub(crate) fn ERR_getErrorName(code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorString(ERR_getErrorCode(code))
}
