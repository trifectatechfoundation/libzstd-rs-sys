use core::ffi::c_uint;
use libc::size_t;

pub const ZSTD_BLOCKSIZELOG_MAX: c_uint = 17;
pub const ZSTD_BLOCKSIZE_MAX: c_uint = (1) << ZSTD_BLOCKSIZELOG_MAX;

pub const ZSTD_CLEVEL_DEFAULT: c_uint = 3;
pub const ZSTD_MAGICNUMBER: c_uint = 0xfd2fb528;
pub const ZSTD_MAGIC_DICTIONARY: c_uint = 0xec30a437;
pub const ZSTD_MAGIC_SKIPPABLE_START: c_uint = 0x184d2a50;
pub const ZSTD_MAGIC_SKIPPABLE_MASK: c_uint = 0xfffffff0;

pub const ZSTD_VERSION_MAJOR: c_uint = 1;
pub const ZSTD_VERSION_MINOR: c_uint = 5;
pub const ZSTD_VERSION_RELEASE: c_uint = 8;
pub const ZSTD_VERSION_NUMBER: c_uint =
    ZSTD_VERSION_MAJOR * 100 * 100 + ZSTD_VERSION_MINOR * 100 + ZSTD_VERSION_RELEASE;

pub type ZSTD_ErrorCode = core::ffi::c_uint;

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

pub type ZSTD_strategy = core::ffi::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: core::ffi::c_uint,
    pub chainLog: core::ffi::c_uint,
    pub hashLog: core::ffi::c_uint,
    pub searchLog: core::ffi::c_uint,
    pub minMatch: core::ffi::c_uint,
    pub targetLength: core::ffi::c_uint,
    pub strategy: ZSTD_strategy,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: core::ffi::c_int,
    pub checksumFlag: core::ffi::c_int,
    pub noDictIDFlag: core::ffi::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_parameters {
    pub cParams: ZSTD_compressionParameters,
    pub fParams: ZSTD_frameParameters,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut core::ffi::c_void,
}
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> ()>;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(*mut core::ffi::c_void, size_t) -> *mut core::ffi::c_void>;

pub type ZSTD_format_e = core::ffi::c_uint;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Format {
    ZSTD_f_zstd1 = 0,
    ZSTD_f_zstd1_magicless = 1,
}

impl TryFrom<ZSTD_format_e> for Format {
    type Error = ();

    fn try_from(value: ZSTD_format_e) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ZSTD_f_zstd1),
            1 => Ok(Self::ZSTD_f_zstd1_magicless),
            _ => Err(()),
        }
    }
}

pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const core::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}

pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut core::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}

pub type ZSTD_bufferMode_e = core::ffi::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BufferMode {
    Buffered,
    Stable,
}

impl TryFrom<u32> for BufferMode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Buffered),
            1 => Ok(Self::Stable),
            _ => Err(()),
        }
    }
}

pub static mut ZSTD_defaultCMem: ZSTD_customMem = ZSTD_customMem {
    customAlloc: None,
    customFree: None,
    opaque: core::ptr::null_mut(),
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameProgression {
    pub ingested: core::ffi::c_ulonglong,
    pub consumed: core::ffi::c_ulonglong,
    pub produced: core::ffi::c_ulonglong,
    pub flushed: core::ffi::c_ulonglong,
    pub currentJobID: core::ffi::c_uint,
    pub nbActiveWorkers: core::ffi::c_uint,
}

pub mod experimental {
    use crate::lib::zstd::Format;

    pub const fn ZSTD_FRAMEHEADERSIZE_MIN(format: Format) -> usize {
        if let Format::ZSTD_f_zstd1 = format {
            6
        } else {
            2
        }
    }

    pub use crate::lib::common::pool::{
        ZSTD_createThreadPool, ZSTD_freeThreadPool, ZSTD_threadPool,
    };
}
