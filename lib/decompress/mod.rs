use crate::lib::common::xxhash::XXH64_state_t;

use crate::lib::common::zstd_trace::ZSTD_TraceCtx;
use crate::lib::decompress::huf_decompress::DTable;
use crate::lib::decompress::zstd_ddict::{ZSTD_DDict, ZSTD_DDictHashSet, ZSTD_refMultipleDDicts_e};
use crate::lib::zstd::{Format, ZSTD_bufferMode_e, ZSTD_customMem, ZSTD_outBuffer};

pub mod huf_decompress;
pub mod zstd_ddict;
pub mod zstd_decompress;
pub mod zstd_decompress_block;

static LL_base: [u32; 36] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 20, 22, 24, 28, 32, 40, 48, 64,
    0x80, 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000,
];
static OF_base: [u32; 32] = [
    0, 1, 1, 5, 0xd, 0x1d, 0x3d, 0x7d, 0xfd, 0x1fd, 0x3fd, 0x7fd, 0xffd, 0x1ffd, 0x3ffd, 0x7ffd,
    0xfffd, 0x1fffd, 0x3fffd, 0x7fffd, 0xffffd, 0x1ffffd, 0x3ffffd, 0x7ffffd, 0xfffffd, 0x1fffffd,
    0x3fffffd, 0x7fffffd, 0xffffffd, 0x1ffffffd, 0x3ffffffd, 0x7ffffffd,
];
static OF_bits: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
static ML_base: [u32; 53] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 37, 39, 41, 43, 47, 51, 59, 67, 83, 99, 0x83, 0x103, 0x203,
    0x403, 0x803, 0x1003, 0x2003, 0x4003, 0x8003, 0x10003,
];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol_header {
    pub fastMode: u32,
    pub tableLog: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol {
    pub nextState: u16,
    pub nbAdditionalBits: u8,
    pub nbBits: u8,
    pub baseValue: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyDTables_t {
    pub LLTable: [ZSTD_seqSymbol; 513],
    pub OFTable: [ZSTD_seqSymbol; 257],
    pub MLTable: [ZSTD_seqSymbol; 513],
    pub hufTable: DTable,
    pub rep: [u32; 3],
    pub workspace: [u32; 157],
}

pub type HUF_DTable = u32;

pub type ZSTD_dStage = core::ffi::c_uint;
pub const ZSTDds_skipFrame: ZSTD_dStage = 7;
pub const ZSTDds_decodeSkippableHeader: ZSTD_dStage = 6;
pub const ZSTDds_checkChecksum: ZSTD_dStage = 5;
pub const ZSTDds_decompressLastBlock: ZSTD_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTD_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTD_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTD_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTD_dStage = 0;

pub type ZSTD_dStreamStage = core::ffi::c_uint;
pub const zdss_flush: ZSTD_dStreamStage = 4;
pub const zdss_load: ZSTD_dStreamStage = 3;
pub const zdss_read: ZSTD_dStreamStage = 2;
pub const zdss_loadHeader: ZSTD_dStreamStage = 1;
pub const zdss_init: ZSTD_dStreamStage = 0;

pub type ZSTD_dictUses_e = core::ffi::c_int;
pub const ZSTD_use_once: ZSTD_dictUses_e = 1;
pub const ZSTD_dont_use: ZSTD_dictUses_e = 0;
pub const ZSTD_use_indefinitely: ZSTD_dictUses_e = -1;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LitLocation {
    ZSTD_split = 2,
    ZSTD_in_dst = 1,
    ZSTD_not_in_dst = 0,
}

pub type ZSTD_forceIgnoreChecksum_e = core::ffi::c_uint;
pub const ZSTD_d_ignoreChecksum: ZSTD_forceIgnoreChecksum_e = 1;
pub const ZSTD_d_validateChecksum: ZSTD_forceIgnoreChecksum_e = 0;

pub type ZSTD_FrameType_e = core::ffi::c_uint;
pub const ZSTD_skippableFrame: ZSTD_FrameType_e = 1;
pub const ZSTD_frame: ZSTD_FrameType_e = 0;

pub type blockType_e = core::ffi::c_uint;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct ZSTD_FrameHeader {
    pub frameContentSize: core::ffi::c_ulonglong,
    pub windowSize: core::ffi::c_ulonglong,
    pub blockSizeMax: core::ffi::c_uint,
    pub frameType: ZSTD_FrameType_e,
    pub headerSize: core::ffi::c_uint,
    pub dictID: core::ffi::c_uint,
    pub checksumFlag: core::ffi::c_uint,
    pub _reserved1: core::ffi::c_uint,
    pub _reserved2: core::ffi::c_uint,
}

// FIXME: make usize
type size_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Workspace {
    data: [u32; 640],
}

impl Workspace {
    fn as_mut_ptr(&mut self) -> *mut u32 {
        self.data.as_mut_ptr()
    }

    fn as_x1_mut(&mut self) -> &mut huf_decompress::HUF_ReadDTableX1_Workspace {
        const { assert!(size_of::<Self>() >= size_of::<huf_decompress::HUF_ReadDTableX1_Workspace>()) }
        const { assert!(align_of::<Self>() >= align_of::<huf_decompress::HUF_ReadDTableX1_Workspace>()) }

        unsafe { core::mem::transmute(&mut self.data) }
    }

    fn as_x2_mut(&mut self) -> &mut huf_decompress::HUF_ReadDTableX2_Workspace {
        const { assert!(size_of::<Self>() >= size_of::<huf_decompress::HUF_ReadDTableX2_Workspace>()) }
        const { assert!(align_of::<Self>() >= align_of::<huf_decompress::HUF_ReadDTableX2_Workspace>()) }

        unsafe { core::mem::transmute(&mut self.data) }
    }

    fn as_symbols_mut(&mut self) -> &mut [u16; 2 * 640] {
        unsafe { core::mem::transmute(&mut self.data) }
    }
}

pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: Workspace,
    pub previousDstEnd: *const core::ffi::c_void,
    pub prefixStart: *const core::ffi::c_void,
    pub virtualStart: *const core::ffi::c_void,
    pub dictEnd: *const core::ffi::c_void,
    pub expected: size_t,
    pub fParams: ZSTD_FrameHeader,
    pub processedCSize: u64,
    pub decodedSize: u64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: size_t,
    pub format: Format,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    pub validateChecksum: u32,
    pub litPtr: *const u8,
    pub customMem: ZSTD_customMem,
    pub litSize: size_t,
    pub rleSize: size_t,
    pub staticSize: size_t,
    pub isFrameDecompression: core::ffi::c_int,
    pub bmi2: core::ffi::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: u32,
    pub ddictIsCold: core::ffi::c_int,
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e,
    pub disableHufAsm: core::ffi::c_int,
    pub maxBlockSizeParam: core::ffi::c_int,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut core::ffi::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub maxWindowSize: size_t,
    pub outBuff: *mut core::ffi::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub lhSize: size_t,
    pub legacyContext: *mut core::ffi::c_void,
    pub previousLegacyVersion: u32,
    pub legacyVersion: u32,
    pub hostageByte: u32,
    pub noForwardProgress: core::ffi::c_int,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut u8,
    pub litBufferEnd: *const u8,
    pub litBufferLocation: LitLocation,
    pub litExtraBuffer: [u8; 65568],
    pub headerBuffer: [u8; 18],
    pub oversizedDuration: size_t,
    pub traceCtx: ZSTD_TraceCtx,
}
