#![no_main]
#![allow(deprecated)]

use libfuzzer_sys::fuzz_target;

use std::ffi::*;

fn rs(compressed: &[u8]) -> (usize, Vec<u8>) {
    use libzstd_rs_sys::*;

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size = unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) };
    if decompressed_size == ZSTD_CONTENTSIZE_ERROR {
        return (decompressed_size as usize, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN {
        return (decompressed_size as usize, vec![]);
    }

    // Allocate buffer for decompressed output
    let mut decompressed = vec![0u8; Ord::min(decompressed_size as usize, 1 << 20)];
    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            compressed_ptr,
            compressed_size,
        )
    };

    (result as usize, decompressed)
}

fn c(compressed: &[u8]) -> (usize, Vec<u8>) {
    use zstd_sys::*;

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size = unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) };
    if decompressed_size == ZSTD_CONTENTSIZE_ERROR as u64 {
        return (decompressed_size as usize, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN as u64 {
        return (decompressed_size as usize, vec![]);
    }

    // Allocate buffer for decompressed output
    let mut decompressed = vec![0u8; Ord::min(decompressed_size as usize, 1 << 20)];
    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            compressed_ptr,
            compressed_size,
        )
    };

    (result, decompressed)
}

fuzz_target!(|data: &[u8]| {
    let (c_err, c_out) = c(data);
    let (rs_err, rs_out) = rs(data);

    let rs_err = match ERR_getErrorCode(rs_err) {
        0 => Ok(rs_err),
        code => match Error::try_from(code) {
            Ok(v) => Err(v),
            Err(()) => Ok(code),
        },
    };
    let c_err = match ERR_getErrorCode(c_err) {
        0 => Ok(c_err),
        code => match Error::try_from(code) {
            Ok(v) => Err(v),
            Err(()) => Ok(code),
        },
    };

    // The zstd version that we're testing against supports much older legacy versions.
    if rs_err == Err(Error::prefix_unknown) {
        return;
    }

    assert_eq!(rs_err, c_err);

    if rs_err.is_ok() {
        assert_eq!(rs_out, c_out);
    }
});

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
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

impl TryFrom<usize> for Error {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
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

#[allow(non_snake_case)]
const fn ERR_isError(code: usize) -> std::ffi::c_uint {
    use zstd_sys::ZSTD_ErrorCode::ZSTD_error_maxCode;

    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as usize) as std::ffi::c_int
        as std::ffi::c_uint
}
#[allow(non_snake_case)]
const fn ERR_getErrorCode(code: usize) -> usize {
    if ERR_isError(code) == 0 {
        return 0;
    }

    (0 as std::ffi::c_int as usize).wrapping_sub(code) as usize
}
