use core::ffi::c_uint;
use libc::size_t;

pub const ZSTD_FRAMEHEADERSIZE_MAX: core::ffi::c_int = 18;

pub const ZSTD_WINDOWLOG_MAX_32: core::ffi::c_int = 30;
pub const ZSTD_WINDOWLOG_MAX_64: core::ffi::c_int = 31;
pub const ZSTD_WINDOWLOG_MAX: core::ffi::c_int = match size_of::<usize>() {
    4 => ZSTD_WINDOWLOG_MAX_32,
    8 => ZSTD_WINDOWLOG_MAX_64,
    _ => panic!(),
};

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

#[derive(Copy, Clone, Default)]
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

impl Format {
    pub(crate) const fn frame_header_size_min(self) -> usize {
        match self {
            Format::ZSTD_f_zstd1 => 6,
            Format::ZSTD_f_zstd1_magicless => 2,
        }
    }
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZSTD_dParameter(u32);

impl ZSTD_dParameter {
    pub const ZSTD_d_experimentalParam1: Self = Self(1000);
    pub const ZSTD_d_experimentalParam2: Self = Self(1001);
    pub const ZSTD_d_experimentalParam3: Self = Self(1002);
    pub const ZSTD_d_experimentalParam4: Self = Self(1003);
    pub const ZSTD_d_experimentalParam5: Self = Self(1004);
    pub const ZSTD_d_experimentalParam6: Self = Self(1005);

    pub const ZSTD_d_format: Self = Self::ZSTD_d_experimentalParam1;
    pub const ZSTD_d_stableOutBuffer: Self = Self::ZSTD_d_experimentalParam2;
    pub const ZSTD_d_forceIgnoreChecksum: Self = Self::ZSTD_d_experimentalParam3;
    pub const ZSTD_d_refMultipleDDicts: Self = Self::ZSTD_d_experimentalParam4;
    pub const ZSTD_d_disableHuffmanAssembly: Self = Self::ZSTD_d_experimentalParam5;
    pub const ZSTD_d_maxBlockSize: Self = Self::ZSTD_d_experimentalParam6;

    pub const ZSTD_d_windowLogMax: Self = Self(100);
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ForceIgnoreChecksum {
    ValidateChecksum = 0,
    IgnoreChecksum = 1,
}

impl TryFrom<i32> for ForceIgnoreChecksum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ValidateChecksum),
            1 => Ok(Self::IgnoreChecksum),
            _ => Err(()),
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZSTD_cParameter(pub(crate) u32);

impl ZSTD_cParameter {
    pub const ZSTD_c_experimentalParam1: Self = Self(500);
    pub const ZSTD_c_experimentalParam2: Self = Self(10);
    pub const ZSTD_c_experimentalParam3: Self = Self(1000);
    pub const ZSTD_c_experimentalParam4: Self = Self(1001);
    pub const ZSTD_c_experimentalParam5: Self = Self(1002);
    pub const ZSTD_c_experimentalParam7: Self = Self(1004);
    pub const ZSTD_c_experimentalParam8: Self = Self(1005);
    pub const ZSTD_c_experimentalParam9: Self = Self(1006);
    pub const ZSTD_c_experimentalParam10: Self = Self(1007);
    pub const ZSTD_c_experimentalParam11: Self = Self(1008);
    pub const ZSTD_c_experimentalParam12: Self = Self(1009);
    pub const ZSTD_c_experimentalParam13: Self = Self(1010);
    pub const ZSTD_c_experimentalParam14: Self = Self(1011);
    pub const ZSTD_c_experimentalParam15: Self = Self(1012);
    pub const ZSTD_c_experimentalParam16: Self = Self(1013);
    pub const ZSTD_c_experimentalParam17: Self = Self(1014);
    pub const ZSTD_c_experimentalParam18: Self = Self(1015);
    pub const ZSTD_c_experimentalParam19: Self = Self(1016);
    pub const ZSTD_c_experimentalParam20: Self = Self(1017);

    pub const ZSTD_c_rsyncable: Self = Self::ZSTD_c_experimentalParam1;
    pub const ZSTD_c_format: Self = Self::ZSTD_c_experimentalParam2;
    pub const ZSTD_c_forceMaxWindow: Self = Self::ZSTD_c_experimentalParam3;
    pub const ZSTD_c_forceAttachDict: Self = Self::ZSTD_c_experimentalParam4;
    pub const ZSTD_c_literalCompressionMode: Self = Self::ZSTD_c_experimentalParam5;
    pub const ZSTD_c_srcSizeHint: Self = Self::ZSTD_c_experimentalParam7;
    pub const ZSTD_c_enableDedicatedDictSearch: Self = Self::ZSTD_c_experimentalParam8;
    pub const ZSTD_c_stableInBuffer: Self = Self::ZSTD_c_experimentalParam9;
    pub const ZSTD_c_stableOutBuffer: Self = Self::ZSTD_c_experimentalParam10;
    pub const ZSTD_c_blockDelimiters: Self = Self::ZSTD_c_experimentalParam11;
    pub const ZSTD_c_validateSequences: Self = Self::ZSTD_c_experimentalParam12;
    pub const ZSTD_c_blockSplitterLevel: Self = Self::ZSTD_c_experimentalParam20;
    pub const ZSTD_c_splitAfterSequences: Self = Self::ZSTD_c_experimentalParam13;
    pub const ZSTD_c_useRowMatchFinder: Self = Self::ZSTD_c_experimentalParam14;
    pub const ZSTD_c_deterministicRefPrefix: Self = Self::ZSTD_c_experimentalParam15;
    pub const ZSTD_c_prefetchCDictTables: Self = Self::ZSTD_c_experimentalParam16;
    pub const ZSTD_c_enableSeqProducerFallback: Self = Self::ZSTD_c_experimentalParam17;
    pub const ZSTD_c_maxBlockSize: Self = Self::ZSTD_c_experimentalParam18;
    pub const ZSTD_c_repcodeResolution: Self = Self::ZSTD_c_experimentalParam19;
    pub const ZSTD_c_searchForExternalRepcodes: Self = Self::ZSTD_c_experimentalParam19;

    pub const ZSTD_c_compressionLevel: Self = Self(100);
    pub const ZSTD_c_windowLog: Self = Self(101);
    pub const ZSTD_c_hashLog: Self = Self(102);
    pub const ZSTD_c_chainLog: Self = Self(103);
    pub const ZSTD_c_searchLog: Self = Self(104);
    pub const ZSTD_c_minMatch: Self = Self(105);
    pub const ZSTD_c_targetLength: Self = Self(106);
    pub const ZSTD_c_strategy: Self = Self(107);
    pub const ZSTD_c_targetCBlockSize: Self = Self(130);
    pub const ZSTD_c_enableLongDistanceMatching: Self = Self(160);
    pub const ZSTD_c_ldmHashLog: Self = Self(161);
    pub const ZSTD_c_ldmMinMatch: Self = Self(162);
    pub const ZSTD_c_ldmBucketSizeLog: Self = Self(163);
    pub const ZSTD_c_ldmHashRateLog: Self = Self(164);
    pub const ZSTD_c_contentSizeFlag: Self = Self(200);
    pub const ZSTD_c_checksumFlag: Self = Self(201);
    pub const ZSTD_c_dictIDFlag: Self = Self(202);
    pub const ZSTD_c_nbWorkers: Self = Self(400);
    pub const ZSTD_c_jobSize: Self = Self(401);
    pub const ZSTD_c_overlapLog: Self = Self(402);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZSTD_ResetDirective(pub(crate) u32);
impl ZSTD_ResetDirective {
    pub const ZSTD_reset_session_only: Self = Self(1);
    pub const ZSTD_reset_parameters: Self = Self(2);
    pub const ZSTD_reset_session_and_parameters: Self = Self(3);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZSTD_dictAttachPref_e(pub(crate) u32);
impl ZSTD_dictAttachPref_e {
    pub const ZSTD_dictDefaultAttach: Self = Self(0);
    pub const ZSTD_dictForceAttach: Self = Self(1);
    pub const ZSTD_dictForceCopy: Self = Self(2);
    pub const ZSTD_dictForceLoad: Self = Self(3);
}

impl TryFrom<i32> for ZSTD_dictAttachPref_e {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ZSTD_dictDefaultAttach),
            1 => Ok(Self::ZSTD_dictForceAttach),
            2 => Ok(Self::ZSTD_dictForceCopy),
            3 => Ok(Self::ZSTD_dictForceLoad),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZSTD_ParamSwitch_e {
    ZSTD_ps_auto = 0,
    ZSTD_ps_enable = 1,
    ZSTD_ps_disable = 2,
}

impl ZSTD_ParamSwitch_e {
    pub fn to_i32(self) -> i32 {
        self as i32
    }
}

impl TryFrom<i32> for ZSTD_ParamSwitch_e {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ZSTD_ps_auto),
            1 => Ok(Self::ZSTD_ps_enable),
            2 => Ok(Self::ZSTD_ps_disable),
            _ => Err(()),
        }
    }
}
