use core::ptr::NonNull;
use libc::size_t;

use crate::lib::common::xxhash::XXH64_state_t;

use crate::lib::common::zstd_internal::WILDCOPY_OVERLENGTH;
use crate::lib::common::zstd_trace::ZSTD_TraceCtx;
use crate::lib::decompress::huf_decompress::DTable;
use crate::lib::decompress::zstd_ddict::{MultipleDDicts, ZSTD_DDict, ZSTD_DDictHashSet};
use crate::lib::decompress::zstd_decompress_block::{FseWorkspace, ZSTD_LITBUFFEREXTRASIZE};
use crate::lib::zstd::{
    BufferMode, Format, ZSTD_customMem, ZSTD_forceIgnoreChecksum_e, ZSTD_outBuffer,
};

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
    pub LLTable: SymbolTable<512>,
    pub OFTable: SymbolTable<256>,
    pub MLTable: SymbolTable<512>,
    pub hufTable: DTable,
    pub rep: [u32; 3],
    pub workspace: FseWorkspace,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolTable<const N: usize> {
    header: ZSTD_seqSymbol_header,
    symbols: [ZSTD_seqSymbol; N],
}

pub type ZSTD_dStage = core::ffi::c_uint;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecompressStage {
    GetFrameHeaderSize = 0,
    DecodeFrameHeader = 1,
    DecodeBlockHeader = 2,
    DecompressBlock = 3,
    DecompressLastBlock = 4,
    CheckChecksum = 5,
    DecodeSkippableHeader = 6,
    SkipFrame = 7,
}

impl DecompressStage {
    pub const fn to_next_input_type(self) -> NextInputType {
        match self {
            Self::DecodeBlockHeader => NextInputType::BlockHeader,
            Self::DecompressBlock => NextInputType::Block,
            Self::DecompressLastBlock => NextInputType::LastBlock,
            Self::CheckChecksum => NextInputType::Checksum,
            Self::DecodeSkippableHeader | Self::SkipFrame => NextInputType::SkippableFrame,
            Self::GetFrameHeaderSize | Self::DecodeFrameHeader => NextInputType::FrameHeader,
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextInputType {
    FrameHeader = 0,
    BlockHeader = 1,
    Block = 2,
    LastBlock = 3,
    Checksum = 4,
    SkippableFrame = 5,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamStage {
    Init,
    LoadHeader,
    Read,
    Load,
    Flush,
}

#[repr(i32)]
enum DictUses {
    ZSTD_use_once = 1,
    ZSTD_dont_use = 0,
    ZSTD_use_indefinitely = -1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LitLocation {
    /// Split between litExtraBuffer and dst.
    ZSTD_split = 2,
    /// Stored entirely within dst (in memory after current output write).
    ZSTD_in_dst = 1,
    /// Stored entirely within litExtraBuffer.
    ZSTD_not_in_dst = 0,
}

pub type ZSTD_FrameType_e = core::ffi::c_uint;
pub const ZSTD_skippableFrame: ZSTD_FrameType_e = 1;
pub const ZSTD_frame: ZSTD_FrameType_e = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: BlockType,
    pub lastBlock: u32,
    pub origSize: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Raw = 0,
    Rle = 1,
    Compressed = 2,
    Reserved = 3,
}

impl From<u32> for BlockType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Raw,
            1 => Self::Rle,
            2 => Self::Compressed,
            3 => Self::Reserved,
            _ => panic!("invalid `BlockType`: {value}"),
        }
    }
}

#[derive(Default)]
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

#[repr(C)]
pub struct Workspace {
    data: [u32; 640],
}

impl Workspace {
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

    fn as_fse_workspace(&mut self) -> &mut FseWorkspace {
        const { assert!(size_of::<Self>() >= size_of::<FseWorkspace>()) }
        const { assert!(align_of::<Self>() >= align_of::<FseWorkspace>()) }
        unsafe { core::mem::transmute(&mut self.data) }
    }
}

pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[repr(C)]
pub struct ZSTD_DCtx_s {
    LLTptr: Option<NonNull<SymbolTable<512>>>,
    MLTptr: Option<NonNull<SymbolTable<512>>>,
    OFTptr: Option<NonNull<SymbolTable<256>>>,
    HUFptr: Option<NonNull<DTable>>, // None encodes dctx.entropy.hufTable
    entropy: ZSTD_entropyDTables_t,
    workspace: Workspace,
    previousDstEnd: *const core::ffi::c_void,
    prefixStart: *const core::ffi::c_void,
    virtualStart: *const core::ffi::c_void,
    dictEnd: *const core::ffi::c_void,
    expected: size_t,
    fParams: ZSTD_FrameHeader,
    processedCSize: u64,
    decodedSize: u64,
    bType: BlockType,
    stage: DecompressStage,
    litEntropy: bool,
    _padding0: [u8; 3],
    fseEntropy: bool,
    _padding1: [u8; 3],
    xxhState: XXH64_state_t,
    headerSize: size_t,
    format: Format,
    forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    validateChecksum: u32,
    litPtr: *const u8,
    customMem: ZSTD_customMem,
    litSize: size_t,
    rleSize: size_t,
    staticSize: size_t,
    isFrameDecompression: core::ffi::c_int,
    bmi2: bool,
    _padding2: [u8; 3],
    ddictLocal: *mut ZSTD_DDict,
    ddict: *const ZSTD_DDict,
    dictID: u32,
    ddictIsCold: core::ffi::c_int,
    dictUses: DictUses,
    ddictSet: *mut ZSTD_DDictHashSet,
    refMultipleDDicts: MultipleDDicts,
    disableHufAsm: core::ffi::c_int,
    maxBlockSizeParam: core::ffi::c_int,
    streamStage: StreamStage,
    inBuff: *mut core::ffi::c_char,
    inBuffSize: size_t,
    inPos: size_t,
    maxWindowSize: size_t,
    outBuff: *mut core::ffi::c_char,
    outBuffSize: size_t,
    outStart: size_t,
    outEnd: size_t,
    lhSize: size_t,
    legacyContext: *mut core::ffi::c_void,
    previousLegacyVersion: u32,
    legacyVersion: u32,
    hostageByte: u32,
    noForwardProgress: core::ffi::c_int,
    outBufferMode: BufferMode,
    expectedOutBuffer: ZSTD_outBuffer,
    litBuffer: *mut u8,
    litBufferEnd: *const u8,
    litBufferLocation: LitLocation,
    // literal buffer can be split between storage within dst and within this scratch buffer.
    litExtraBuffer: [u8; ZSTD_LITBUFFEREXTRASIZE + WILDCOPY_OVERLENGTH],
    headerBuffer: [u8; 18],
    oversizedDuration: size_t,
    traceCtx: ZSTD_TraceCtx,
}
