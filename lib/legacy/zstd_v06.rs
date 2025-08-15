use core::ptr;

use libc::{free, malloc, memcpy, memmove, memset, size_t};

use crate::lib::common::error_private::{ERR_getErrorName, ERR_isError};
use crate::lib::common::mem::{
    MEM_32bits, MEM_64bits, MEM_readLE16, MEM_readLE32, MEM_readLE64, MEM_readLEST, MEM_writeLE16,
};
use crate::lib::zstd::*;

pub type ptrdiff_t = core::ffi::c_long;
pub type ZSTDv06_DCtx = ZSTDv06_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv06_DCtx_s {
    pub LLTable: [FSEv06_DTable; 513],
    pub OffTable: [FSEv06_DTable; 257],
    pub MLTable: [FSEv06_DTable; 513],
    pub hufTableX4: [core::ffi::c_uint; 4097],
    pub previousDstEnd: *const core::ffi::c_void,
    pub base: *const core::ffi::c_void,
    pub vBase: *const core::ffi::c_void,
    pub dictEnd: *const core::ffi::c_void,
    pub expected: size_t,
    pub headerSize: size_t,
    pub fParams: ZSTDv06_frameParams,
    pub bType: blockType_t,
    pub stage: ZSTDv06_dStage,
    pub flagRepeatTable: u32,
    pub litPtr: *const u8,
    pub litSize: size_t,
    pub litBuffer: [u8; 131080],
    pub headerBuffer: [u8; 13],
}
pub type ZSTDv06_dStage = core::ffi::c_uint;
pub const ZSTDds_decompressBlock: ZSTDv06_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTDv06_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTDv06_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTDv06_dStage = 0;
pub type blockType_t = core::ffi::c_uint;
pub const bt_end: blockType_t = 3;
pub const bt_rle: blockType_t = 2;
pub const bt_raw: blockType_t = 1;
pub const bt_compressed: blockType_t = 0;
pub type ZSTDv06_frameParams = ZSTDv06_frameParams_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv06_frameParams_s {
    pub frameContentSize: core::ffi::c_ulonglong,
    pub windowLog: core::ffi::c_uint,
}
pub type FSEv06_DTable = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_t,
    pub origSize: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: size_t,
    pub matchLength: size_t,
    pub offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BITv06_DStream_t,
    pub stateLL: FSEv06_DState_t,
    pub stateOffb: FSEv06_DState_t,
    pub stateML: FSEv06_DState_t,
    pub prevOffset: [size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv06_DState_t {
    pub state: size_t,
    pub table: *const core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITv06_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: core::ffi::c_uint,
    pub ptr: *const core::ffi::c_char,
    pub start: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv06_decode_t {
    pub newState: core::ffi::c_ushort,
    pub symbol: core::ffi::c_uchar,
    pub nbBits: core::ffi::c_uchar,
}
pub type BITv06_DStream_status = core::ffi::c_uint;
pub const BITv06_DStream_overflow: BITv06_DStream_status = 3;
pub const BITv06_DStream_completed: BITv06_DStream_status = 2;
pub const BITv06_DStream_endOfBuffer: BITv06_DStream_status = 1;
pub const BITv06_DStream_unfinished: BITv06_DStream_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: u32,
    pub c: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv06_DTableHeader {
    pub tableLog: u16,
    pub fastMode: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv06_DEltX4 {
    pub sequence: u16,
    pub nbBits: u8,
    pub length: u8,
}
pub type decompressionAlgo =
    Option<unsafe fn(*mut core::ffi::c_void, size_t, *const core::ffi::c_void, size_t) -> size_t>;
pub type rankVal_t = [[u32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: u8,
    pub weight: u8,
}
pub type DTable_max_t = [u32; 4097];
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HUFv06_static_assert: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv06_DEltX2 {
    pub byte: u8,
    pub nbBits: u8,
}
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const HUFv06_static_assert_0: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: u32,
    pub decode256Time: u32,
}
pub type ERR_enum = ZSTD_ErrorCode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZBUFFv06_DCtx_s {
    pub zd: *mut ZSTDv06_DCtx,
    pub fParams: ZSTDv06_frameParams,
    pub stage: ZBUFFv06_dStage,
    pub inBuff: *mut core::ffi::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub outBuff: *mut core::ffi::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub blockSize: size_t,
    pub headerBuffer: [u8; 13],
    pub lhSize: size_t,
}
pub type ZBUFFv06_dStage = core::ffi::c_uint;
pub const ZBUFFds_flush: ZBUFFv06_dStage = 4;
pub const ZBUFFds_load: ZBUFFv06_dStage = 3;
pub const ZBUFFds_read: ZBUFFv06_dStage = 2;
pub const ZBUFFds_loadHeader: ZBUFFv06_dStage = 1;
pub const ZBUFFds_init: ZBUFFv06_dStage = 0;
pub type ZBUFFv06_DCtx = ZBUFFv06_DCtx_s;
pub const ZSTDv06_MAGICNUMBER: core::ffi::c_uint = 0xfd2fb526 as core::ffi::c_uint;
pub const NULL: core::ffi::c_int = 0;
pub const ZSTDv06_FRAMEHEADERSIZE_MAX: core::ffi::c_int = 13;
static ZSTDv06_frameHeaderSize_min: size_t = 5;
static ZSTDv06_frameHeaderSize_max: size_t = ZSTDv06_FRAMEHEADERSIZE_MAX as size_t;
pub const ZSTDv06_BLOCKSIZE_MAX: core::ffi::c_int = 128 * 1024;
pub const ZSTDv06_DICT_MAGIC: core::ffi::c_uint = 0xec30a436 as core::ffi::c_uint;
pub const ZSTDv06_REP_NUM: core::ffi::c_int = 3;
pub const ZSTDv06_REP_INIT: core::ffi::c_int = 3;
pub const ZSTDv06_REP_MOVE: core::ffi::c_int = ZSTDv06_REP_NUM - 1;
pub const ZSTDv06_WINDOWLOG_ABSOLUTEMIN: core::ffi::c_int = 12;
static ZSTDv06_fcs_fieldSize: [size_t; 4] = [0, 1, 2, 8];
pub const ZSTDv06_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTDv06_blockHeaderSize: size_t = ZSTDv06_BLOCKHEADERSIZE as size_t;
pub const MIN_SEQUENCES_SIZE: core::ffi::c_int = 1;
pub const MIN_CBLOCK_SIZE: core::ffi::c_int = 1 + 1 + MIN_SEQUENCES_SIZE;
pub const ZSTD_HUFFDTABLE_CAPACITY_LOG: core::ffi::c_int = 12;
pub const IS_HUF: core::ffi::c_int = 0;
pub const IS_PCH: core::ffi::c_int = 1;
pub const IS_RAW: core::ffi::c_int = 2;
pub const IS_RLE: core::ffi::c_int = 3;
pub const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
pub const MINMATCH: core::ffi::c_int = 3;
pub const REPCODE_STARTVALUE: core::ffi::c_int = 1;
pub const MaxML: core::ffi::c_int = 52;
pub const MaxLL: core::ffi::c_int = 35;
pub const MaxOff: core::ffi::c_int = 28;
pub const MLFSELog: core::ffi::c_int = 9;
pub const LLFSELog: core::ffi::c_int = 9;
pub const OffFSELog: core::ffi::c_int = 8;
pub const FSEv06_ENCODING_RAW: core::ffi::c_int = 0;
pub const FSEv06_ENCODING_RLE: core::ffi::c_int = 1;
pub const FSEv06_ENCODING_STATIC: core::ffi::c_int = 2;
pub const FSEv06_ENCODING_DYNAMIC: core::ffi::c_int = 3;
pub const ZSTD_CONTENTSIZE_ERROR: core::ffi::c_ulonglong =
    (0 as core::ffi::c_ulonglong).wrapping_sub(2);
static LL_bits: [u32; 36] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16,
];
static LL_defaultNorm: [i16; 36] = [
    4, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 1, 1, 1, 1, 1,
    -1, -1, -1, -1,
];
static LL_defaultNormLog: u32 = 6;
static ML_bits: [u32; 53] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
];
static ML_defaultNorm: [i16; 53] = [
    1, 4, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1,
];
static ML_defaultNormLog: u32 = 6;
static OF_defaultNorm: [i16; 29] = [
    1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1,
];
static OF_defaultNormLog: u32 = 5;
unsafe fn ZSTDv06_copy8(mut dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void) {
    memcpy(dst, src, 8);
}
pub const WILDCOPY_OVERLENGTH: core::ffi::c_int = 8;
#[inline]
unsafe fn ZSTDv06_wildcopy(
    mut dst: *mut core::ffi::c_void,
    mut src: *const core::ffi::c_void,
    mut length: ptrdiff_t,
) {
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    loop {
        ZSTDv06_copy8(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
        op = op.offset(8);
        ip = ip.offset(8);
        if op >= oend {
            break;
        }
    }
}
#[inline]
unsafe fn BITv06_highbit32(mut val: u32) -> core::ffi::c_uint {
    (val.leading_zeros() as i32 ^ 31) as core::ffi::c_uint
}
#[inline]
unsafe fn BITv06_initDStream(
    mut bitD: *mut BITv06_DStream_t,
    mut srcBuffer: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 {
        ptr::write_bytes(
            bitD as *mut u8,
            0,
            ::core::mem::size_of::<BITv06_DStream_t>(),
        );
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if srcSize >= ::core::mem::size_of::<size_t>() as size_t {
        (*bitD).start = srcBuffer as *const core::ffi::c_char;
        (*bitD).ptr = (srcBuffer as *const core::ffi::c_char)
            .add(srcSize)
            .offset(-(::core::mem::size_of::<size_t>() as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const core::ffi::c_void);
        let lastByte = *(srcBuffer as *const u8).add(srcSize.wrapping_sub(1));
        if lastByte as core::ffi::c_int == 0 {
            return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed =
            (8 as core::ffi::c_uint).wrapping_sub(BITv06_highbit32(lastByte as u32));
    } else {
        (*bitD).start = srcBuffer as *const core::ffi::c_char;
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as size_t;
        let mut current_block_20: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                    (*(srcBuffer as *const u8).offset(6) as size_t)
                        << (::core::mem::size_of::<size_t>() as size_t)
                            .wrapping_mul(8)
                            .wrapping_sub(16),
                );
                current_block_20 = 11220331375136032509;
            }
            6 => {
                current_block_20 = 11220331375136032509;
            }
            5 => {
                current_block_20 = 10901957826175510184;
            }
            4 => {
                current_block_20 = 3201895511516222412;
            }
            3 => {
                current_block_20 = 12760952191649157579;
            }
            2 => {
                current_block_20 = 13935781298497728377;
            }
            _ => {
                current_block_20 = 5689001924483802034;
            }
        }
        if current_block_20 == 11220331375136032509 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(5) as size_t)
                    << (::core::mem::size_of::<size_t>())
                        .wrapping_mul(8)
                        .wrapping_sub(24),
            );
            current_block_20 = 10901957826175510184;
        }
        if current_block_20 == 10901957826175510184 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(4) as size_t)
                    << (::core::mem::size_of::<size_t>())
                        .wrapping_mul(8)
                        .wrapping_sub(32),
            );
            current_block_20 = 3201895511516222412;
        }
        if current_block_20 == 3201895511516222412 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(3) as size_t) << 24);
            current_block_20 = 12760952191649157579;
        }
        if current_block_20 == 12760952191649157579 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(2) as size_t) << 16);
            current_block_20 = 13935781298497728377;
        }
        if current_block_20 == 13935781298497728377 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(1) as size_t) << 8);
        }
        let lastByte_0 = *(srcBuffer as *const u8).add(srcSize.wrapping_sub(1));
        if lastByte_0 as core::ffi::c_int == 0 {
            return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed =
            (8 as core::ffi::c_uint).wrapping_sub(BITv06_highbit32(lastByte_0 as u32));
        (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(
            (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(srcSize) as u32 * 8,
        );
    }
    srcSize
}
#[inline]
unsafe fn BITv06_lookBits(mut bitD: *const BITv06_DStream_t, mut nbBits: u32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as size_t)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> 1
        >> (bitMask.wrapping_sub(nbBits) & bitMask)
}
#[inline]
unsafe fn BITv06_lookBitsFast(mut bitD: *const BITv06_DStream_t, mut nbBits: u32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as size_t)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> (bitMask.wrapping_add(1).wrapping_sub(nbBits) & bitMask)
}
#[inline]
unsafe fn BITv06_skipBits(mut bitD: *mut BITv06_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline]
unsafe fn BITv06_readBits(mut bitD: *mut BITv06_DStream_t, mut nbBits: u32) -> size_t {
    let value = BITv06_lookBits(bitD, nbBits);
    BITv06_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe fn BITv06_readBitsFast(mut bitD: *mut BITv06_DStream_t, mut nbBits: u32) -> size_t {
    let value = BITv06_lookBitsFast(bitD, nbBits);
    BITv06_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe fn BITv06_reloadDStream(mut bitD: *mut BITv06_DStream_t) -> BITv06_DStream_status {
    if (*bitD).bitsConsumed as size_t > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
    {
        return BITv06_DStream_overflow;
    }
    if (*bitD).ptr >= ((*bitD).start).add(::core::mem::size_of::<size_t>()) {
        (*bitD).ptr = ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3) as isize));
        (*bitD).bitsConsumed &= 7;
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const core::ffi::c_void);
        return BITv06_DStream_unfinished;
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as size_t)
            < (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
        {
            return BITv06_DStream_endOfBuffer;
        }
        return BITv06_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3;
    let mut result = BITv06_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as core::ffi::c_long as u32;
        result = BITv06_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const core::ffi::c_void);
    result
}
#[inline]
unsafe fn BITv06_endOfDStream(mut DStream: *const BITv06_DStream_t) -> core::ffi::c_uint {
    ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as size_t
            == (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)) as core::ffi::c_int
        as core::ffi::c_uint
}
#[inline]
unsafe fn FSEv06_initDState(
    mut DStatePtr: *mut FSEv06_DState_t,
    mut bitD: *mut BITv06_DStream_t,
    mut dt: *const FSEv06_DTable,
) {
    let mut ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const FSEv06_DTableHeader;
    (*DStatePtr).state = BITv06_readBits(bitD, (*DTableH).tableLog as core::ffi::c_uint);
    BITv06_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1) as *const core::ffi::c_void;
}
#[inline]
unsafe fn FSEv06_peekSymbol(mut DStatePtr: *const FSEv06_DState_t) -> u8 {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).add((*DStatePtr).state);
    DInfo.symbol
}
#[inline]
unsafe fn FSEv06_updateState(mut DStatePtr: *mut FSEv06_DState_t, mut bitD: *mut BITv06_DStream_t) {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let lowBits = BITv06_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
}
#[inline]
unsafe fn FSEv06_decodeSymbol(
    mut DStatePtr: *mut FSEv06_DState_t,
    mut bitD: *mut BITv06_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv06_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe fn FSEv06_decodeSymbolFast(
    mut DStatePtr: *mut FSEv06_DState_t,
    mut bitD: *mut BITv06_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv06_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
pub const FSEv06_MAX_MEMORY_USAGE: core::ffi::c_int = 14;
pub const FSEv06_MAX_SYMBOL_VALUE: core::ffi::c_int = 255;
pub const FSEv06_MAX_TABLELOG: core::ffi::c_int = FSEv06_MAX_MEMORY_USAGE - 2;
pub const FSEv06_MIN_TABLELOG: core::ffi::c_int = 5;
pub const FSEv06_TABLELOG_ABSOLUTE_MAX: core::ffi::c_int = 15;
pub unsafe fn FSEv06_isError_1(mut code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
pub unsafe fn FSEv06_getErrorName(mut code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
unsafe fn HUFv06_isError(mut code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
unsafe fn FSEv06_abs(mut a: core::ffi::c_short) -> core::ffi::c_short {
    (if (a as core::ffi::c_int) < 0 {
        -(a as core::ffi::c_int)
    } else {
        a as core::ffi::c_int
    }) as core::ffi::c_short
}
pub unsafe fn FSEv06_readNCount(
    mut normalizedCounter: *mut core::ffi::c_short,
    mut maxSVPtr: *mut core::ffi::c_uint,
    mut tableLogPtr: *mut core::ffi::c_uint,
    mut headerBuffer: *const core::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart = headerBuffer as *const u8;
    let iend = istart.add(hbSize);
    let mut ip = istart;
    let mut nbBits: core::ffi::c_int = 0;
    let mut remaining: core::ffi::c_int = 0;
    let mut threshold: core::ffi::c_int = 0;
    let mut bitStream: u32 = 0;
    let mut bitCount: core::ffi::c_int = 0;
    let mut charnum = 0;
    let mut previous0 = 0;
    if hbSize < 4 {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    bitStream = MEM_readLE32(ip as *const core::ffi::c_void);
    nbBits = (bitStream & 0xf as core::ffi::c_int as u32).wrapping_add(FSEv06_MIN_TABLELOG as u32)
        as core::ffi::c_int;
    if nbBits > FSEv06_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    bitStream >>= 4;
    bitCount = 4;
    *tableLogPtr = nbBits as core::ffi::c_uint;
    remaining = ((1) << nbBits) + 1;
    threshold = (1) << nbBits;
    nbBits += 1;
    while remaining > 1 && charnum <= *maxSVPtr {
        if previous0 != 0 {
            let mut n0 = charnum;
            while bitStream & 0xffff as core::ffi::c_int as u32 == 0xffff as core::ffi::c_int as u32
            {
                n0 = n0.wrapping_add(24);
                if ip < iend.offset(-(5)) {
                    ip = ip.offset(2);
                    bitStream = MEM_readLE32(ip as *const core::ffi::c_void) >> bitCount;
                } else {
                    bitStream >>= 16;
                    bitCount += 16;
                }
            }
            while bitStream & 3 == 3 {
                n0 = n0.wrapping_add(3);
                bitStream >>= 2;
                bitCount += 2;
            }
            n0 = n0.wrapping_add(bitStream & 3);
            bitCount += 2;
            if n0 > *maxSVPtr {
                return -(ZSTD_error_maxSymbolValue_tooSmall as core::ffi::c_int) as size_t;
            }
            while charnum < n0 {
                let fresh0 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter.offset(fresh0 as isize) = 0;
            }
            if ip <= iend.offset(-(7)) || ip.offset((bitCount >> 3) as isize) <= iend.offset(-(4)) {
                ip = ip.offset((bitCount >> 3) as isize);
                bitCount &= 7;
                bitStream = MEM_readLE32(ip as *const core::ffi::c_void) >> bitCount;
            } else {
                bitStream >>= 2;
            }
        }
        let max = (2 * threshold - 1 - remaining) as core::ffi::c_short;
        let mut count: core::ffi::c_short = 0;
        if (bitStream & (threshold - 1) as u32) < max as u32 {
            count = (bitStream & (threshold - 1) as u32) as core::ffi::c_short;
            bitCount += nbBits - 1;
        } else {
            count = (bitStream & (2 * threshold - 1) as u32) as core::ffi::c_short;
            if count as core::ffi::c_int >= threshold {
                count = (count as core::ffi::c_int - max as core::ffi::c_int) as core::ffi::c_short;
            }
            bitCount += nbBits;
        }
        count -= 1;
        remaining -= FSEv06_abs(count) as core::ffi::c_int;
        let fresh1 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh1 as isize) = count;
        previous0 = (count == 0) as core::ffi::c_int;
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1;
        }
        if ip <= iend.offset(-(7)) || ip.offset((bitCount >> 3) as isize) <= iend.offset(-(4)) {
            ip = ip.offset((bitCount >> 3) as isize);
            bitCount &= 7;
        } else {
            bitCount -=
                (8 * iend.offset(-(4)).offset_from(ip) as core::ffi::c_long) as core::ffi::c_int;
            ip = iend.offset(-(4));
        }
        bitStream = MEM_readLE32(ip as *const core::ffi::c_void) >> (bitCount & 31);
    }
    if remaining != 1 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1);
    ip = ip.offset(((bitCount + 7) >> 3) as isize);
    if ip.offset_from(istart) as core::ffi::c_long as size_t > hbSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip.offset_from(istart) as core::ffi::c_long as size_t
}
pub const FSEv06_isError_0: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub unsafe fn FSEv06_createDTable(mut tableLog: core::ffi::c_uint) -> *mut FSEv06_DTable {
    if tableLog > FSEv06_TABLELOG_ABSOLUTE_MAX as core::ffi::c_uint {
        tableLog = FSEv06_TABLELOG_ABSOLUTE_MAX as core::ffi::c_uint;
    }
    malloc(
        ((1 + ((1) << tableLog)) as size_t).wrapping_mul(::core::mem::size_of::<u32>() as size_t),
    ) as *mut FSEv06_DTable
}
pub unsafe fn FSEv06_freeDTable(mut dt: *mut FSEv06_DTable) {
    free(dt as *mut core::ffi::c_void);
}
pub unsafe fn FSEv06_buildDTable(
    mut dt: *mut FSEv06_DTable,
    mut normalizedCounter: *const core::ffi::c_short,
    mut maxSymbolValue: core::ffi::c_uint,
    mut tableLog: core::ffi::c_uint,
) -> size_t {
    let tdPtr = dt.offset(1) as *mut core::ffi::c_void;
    let tableDecode = tdPtr as *mut FSEv06_decode_t;
    let mut symbolNext: [u16; 256] = [0; 256];
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    if maxSymbolValue > FSEv06_MAX_SYMBOL_VALUE as core::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as core::ffi::c_int) as size_t;
    }
    if tableLog > FSEv06_MAX_TABLELOG as core::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    let mut DTableH = FSEv06_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as u16;
    DTableH.fastMode = 1;
    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;
    let mut s: u32 = 0;
    s = 0;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as core::ffi::c_int == -(1) {
            let fresh2 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh2 as isize)).symbol = s as u8;
            *symbolNext.as_mut_ptr().offset(s as isize) = 1;
        } else {
            if *normalizedCounter.offset(s as isize) as core::ffi::c_int
                >= largeLimit as core::ffi::c_int
            {
                DTableH.fastMode = 0;
            }
            *symbolNext.as_mut_ptr().offset(s as isize) =
                *normalizedCounter.offset(s as isize) as u16;
        }
        s = s.wrapping_add(1);
    }
    memcpy(
        dt as *mut core::ffi::c_void,
        &mut DTableH as *mut FSEv06_DTableHeader as *const core::ffi::c_void,
        ::core::mem::size_of::<FSEv06_DTableHeader>() as size_t,
    );
    let tableMask = tableSize.wrapping_sub(1);
    let step = (tableSize >> 1)
        .wrapping_add(tableSize >> 3)
        .wrapping_add(3);
    let mut s_0: u32 = 0;
    let mut position = 0u32;
    s_0 = 0;
    while s_0 < maxSV1 {
        let mut i: core::ffi::c_int = 0;
        i = 0;
        while i < *normalizedCounter.offset(s_0 as isize) as core::ffi::c_int {
            (*tableDecode.offset(position as isize)).symbol = s_0 as u8;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
        }
        s_0 = s_0.wrapping_add(1);
    }
    if position != 0 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    let mut u: u32 = 0;
    u = 0;
    while u < tableSize {
        let symbol = (*tableDecode.offset(u as isize)).symbol;
        let fresh3 = &mut (*symbolNext.as_mut_ptr().offset(symbol as isize));
        let fresh4 = *fresh3;
        *fresh3 = (*fresh3).wrapping_add(1);
        let mut nextState = fresh4;
        (*tableDecode.offset(u as isize)).nbBits =
            tableLog.wrapping_sub(BITv06_highbit32(nextState as u32)) as u8;
        (*tableDecode.offset(u as isize)).newState = (((nextState as core::ffi::c_int)
            << (*tableDecode.offset(u as isize)).nbBits as core::ffi::c_int)
            as u32)
            .wrapping_sub(tableSize) as u16;
        u = u.wrapping_add(1);
    }
    0
}
pub unsafe fn FSEv06_buildDTable_rle(mut dt: *mut FSEv06_DTable, mut symbolValue: u8) -> size_t {
    let mut ptr = dt as *mut core::ffi::c_void;
    let DTableH = ptr as *mut FSEv06_DTableHeader;
    let mut dPtr = dt.offset(1) as *mut core::ffi::c_void;
    let cell = dPtr as *mut FSEv06_decode_t;
    (*DTableH).tableLog = 0;
    (*DTableH).fastMode = 0;
    (*cell).newState = 0;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0;
    0
}
pub unsafe fn FSEv06_buildDTable_raw(
    mut dt: *mut FSEv06_DTable,
    mut nbBits: core::ffi::c_uint,
) -> size_t {
    let mut ptr = dt as *mut core::ffi::c_void;
    let DTableH = ptr as *mut FSEv06_DTableHeader;
    let mut dPtr = dt.offset(1) as *mut core::ffi::c_void;
    let dinfo = dPtr as *mut FSEv06_decode_t;
    let tableSize = ((1) << nbBits) as core::ffi::c_uint;
    let tableMask = tableSize.wrapping_sub(1);
    let maxSV1 = tableMask.wrapping_add(1);
    let mut s: core::ffi::c_uint = 0;
    if nbBits < 1 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    (*DTableH).tableLog = nbBits as u16;
    (*DTableH).fastMode = 1;
    s = 0;
    while s < maxSV1 {
        (*dinfo.offset(s as isize)).newState = 0;
        (*dinfo.offset(s as isize)).symbol = s as u8;
        (*dinfo.offset(s as isize)).nbBits = nbBits as u8;
        s = s.wrapping_add(1);
    }
    0
}
#[inline(always)]
unsafe fn FSEv06_decompress_usingDTable_generic(
    mut dst: *mut core::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSEv06_DTable,
    fast: core::ffi::c_uint,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.add(maxDstSize);
    let olimit = omax.offset(-(3));
    let mut bitD = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut state1 = FSEv06_DState_t {
        state: 0,
        table: core::ptr::null::<core::ffi::c_void>(),
    };
    let mut state2 = FSEv06_DState_t {
        state: 0,
        table: core::ptr::null::<core::ffi::c_void>(),
    };
    let errorCode = BITv06_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv06_initDState(&mut state1, &mut bitD, dt);
    FSEv06_initDState(&mut state2, &mut bitD, dt);
    while BITv06_reloadDStream(&mut bitD) as core::ffi::c_uint
        == BITv06_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && op < olimit
    {
        *op.offset(0) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv06_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
        {
            BITv06_reloadDStream(&mut bitD);
        }
        *op.offset(1) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv06_MAX_TABLELOG * 4 + 7) as size_t
            > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
            && BITv06_reloadDStream(&mut bitD) as core::ffi::c_uint
                > BITv06_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        {
            op = op.offset(2);
            break;
        }
        *op.offset(2) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv06_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
        {
            BITv06_reloadDStream(&mut bitD);
        }
        *op.offset(3) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        op = op.offset(4);
    }
    loop {
        if op > omax.offset(-(2)) {
            return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
        }
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if BITv06_reloadDStream(&mut bitD) as core::ffi::c_uint
            == BITv06_DStream_overflow as core::ffi::c_int as core::ffi::c_uint
        {
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = (if fast != 0 {
                FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
            } else {
                FSEv06_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
            }) as u8;
            break;
        } else {
            if op > omax.offset(-(2)) {
                return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
            }
            let fresh7 = op;
            op = op.offset(1);
            *fresh7 = (if fast != 0 {
                FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
            } else {
                FSEv06_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
            }) as u8;
            if BITv06_reloadDStream(&mut bitD) as core::ffi::c_uint
                != BITv06_DStream_overflow as core::ffi::c_int as core::ffi::c_uint
            {
                continue;
            }
            let fresh8 = op;
            op = op.offset(1);
            *fresh8 = (if fast != 0 {
                FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
            } else {
                FSEv06_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
            }) as u8;
            break;
        }
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
pub unsafe fn FSEv06_decompress_usingDTable(
    mut dst: *mut core::ffi::c_void,
    mut originalSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSEv06_DTable,
) -> size_t {
    let mut ptr = dt as *const core::ffi::c_void;
    let mut DTableH = ptr as *const FSEv06_DTableHeader;
    let fastMode = (*DTableH).fastMode as u32;
    if fastMode != 0 {
        return FSEv06_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 1);
    }
    FSEv06_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 0)
}
pub unsafe fn FSEv06_decompress(
    mut dst: *mut core::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let istart = cSrc as *const u8;
    let mut ip = istart;
    let mut counting: [core::ffi::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: core::ffi::c_uint = 0;
    let mut maxSymbolValue = FSEv06_MAX_SYMBOL_VALUE as core::ffi::c_uint;
    if cSrcSize < 2 {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let NCountLength = FSEv06_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const core::ffi::c_void,
        cSrcSize,
    );
    if ERR_isError(NCountLength) != 0 {
        return NCountLength;
    }
    if NCountLength >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(NCountLength);
    cSrcSize = cSrcSize.wrapping_sub(NCountLength);
    let errorCode = FSEv06_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv06_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    )
}
pub const HUFv06_ABSOLUTEMAX_TABLELOG: core::ffi::c_int = 16;
pub const HUFv06_MAX_TABLELOG: core::ffi::c_int = 12;
pub const HUFv06_MAX_SYMBOL_VALUE: core::ffi::c_int = 255;
#[inline]
unsafe fn HUFv06_readStats(
    mut huffWeight: *mut u8,
    mut hwSize: size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightTotal: u32 = 0;
    let mut ip = src as *const u8;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    iSize = *ip.offset(0) as size_t;
    if iSize >= 128 {
        if iSize >= 242 {
            static l: [u32; 14] = [1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128];
            oSize = l[iSize.wrapping_sub(242)] as size_t;
            memset(huffWeight as *mut core::ffi::c_void, 1, hwSize);
            iSize = 0;
        } else {
            oSize = iSize.wrapping_sub(127);
            iSize = oSize.wrapping_add(1) / 2;
            if iSize.wrapping_add(1) > srcSize {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            if oSize >= hwSize {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            ip = ip.offset(1);
            let mut n: u32 = 0;
            n = 0;
            while (n as size_t) < oSize {
                *huffWeight.offset(n as isize) =
                    (*ip.offset((n / 2) as isize) as core::ffi::c_int >> 4) as u8;
                *huffWeight.offset(n.wrapping_add(1) as isize) =
                    (*ip.offset((n / 2) as isize) as core::ffi::c_int & 15) as u8;
                n = n.wrapping_add(2);
            }
        }
    } else {
        if iSize.wrapping_add(1) > srcSize {
            return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
        }
        oSize = FSEv06_decompress(
            huffWeight as *mut core::ffi::c_void,
            hwSize.wrapping_sub(1),
            ip.offset(1) as *const core::ffi::c_void,
            iSize,
        );
        if ERR_isError(oSize) != 0 {
            return oSize;
        }
    }
    memset(
        rankStats as *mut core::ffi::c_void,
        0,
        ((HUFv06_ABSOLUTEMAX_TABLELOG + 1) as size_t)
            .wrapping_mul(::core::mem::size_of::<u32>() as size_t),
    );
    weightTotal = 0;
    let mut n_0: u32 = 0;
    n_0 = 0;
    while (n_0 as size_t) < oSize {
        if *huffWeight.offset(n_0 as isize) as core::ffi::c_int >= HUFv06_ABSOLUTEMAX_TABLELOG {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        let fresh9 = &mut (*rankStats.offset(*huffWeight.offset(n_0 as isize) as isize));
        *fresh9 = (*fresh9).wrapping_add(1);
        weightTotal = weightTotal.wrapping_add(
            ((1) << *huffWeight.offset(n_0 as isize) as core::ffi::c_int >> 1) as u32,
        );
        n_0 = n_0.wrapping_add(1);
    }
    if weightTotal == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let tableLog = (BITv06_highbit32(weightTotal)).wrapping_add(1);
    if tableLog > HUFv06_ABSOLUTEMAX_TABLELOG as u32 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    *tableLogPtr = tableLog;
    let total = ((1) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1) << BITv06_highbit32(rest)) as u32;
    let lastWeight = (BITv06_highbit32(rest)).wrapping_add(1);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    *huffWeight.add(oSize) = lastWeight as u8;
    let fresh10 = &mut (*rankStats.offset(lastWeight as isize));
    *fresh10 = (*fresh10).wrapping_add(1);
    if *rankStats.offset(1) < 2 || *rankStats.offset(1) & 1 != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    iSize.wrapping_add(1)
}
pub unsafe fn HUFv06_readDTableX2(
    mut DTable: *mut u16,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 17] = [0; 17];
    let mut tableLog = 0;
    let mut iSize: size_t = 0;
    let mut nbSymbols = 0;
    let mut n: u32 = 0;
    let mut nextRankStart: u32 = 0;
    let dtPtr = DTable.offset(1) as *mut core::ffi::c_void;
    let dt = dtPtr as *mut HUFv06_DEltX2;
    iSize = HUFv06_readStats(
        huffWeight.as_mut_ptr(),
        (HUFv06_MAX_SYMBOL_VALUE + 1) as size_t,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv06_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > *DTable.offset(0) as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    *DTable.offset(0) = tableLog as u16;
    nextRankStart = 0;
    n = 1;
    while n < tableLog.wrapping_add(1) {
        let mut current = nextRankStart;
        nextRankStart = nextRankStart
            .wrapping_add(*rankVal.as_mut_ptr().offset(n as isize) << n.wrapping_sub(1));
        *rankVal.as_mut_ptr().offset(n as isize) = current;
        n = n.wrapping_add(1);
    }
    n = 0;
    while n < nbSymbols {
        let w = *huffWeight.as_mut_ptr().offset(n as isize) as u32;
        let length = ((1) << w >> 1) as u32;
        let mut i: u32 = 0;
        let mut D = HUFv06_DEltX2 { byte: 0, nbBits: 0 };
        D.byte = n as u8;
        D.nbBits = tableLog.wrapping_add(1).wrapping_sub(w) as u8;
        i = *rankVal.as_mut_ptr().offset(w as isize);
        while i < (*rankVal.as_mut_ptr().offset(w as isize)).wrapping_add(length) {
            *dt.offset(i as isize) = D;
            i = i.wrapping_add(1);
        }
        let fresh11 = &mut (*rankVal.as_mut_ptr().offset(w as isize));
        *fresh11 = (*fresh11).wrapping_add(length);
        n = n.wrapping_add(1);
    }
    iSize
}
unsafe fn HUFv06_decodeSymbolX2(
    mut Dstream: *mut BITv06_DStream_t,
    mut dt: *const HUFv06_DEltX2,
    dtLog: u32,
) -> u8 {
    let val = BITv06_lookBitsFast(Dstream, dtLog);
    let c = (*dt.add(val)).byte;
    BITv06_skipBits(Dstream, (*dt.add(val)).nbBits as u32);
    c
}
#[inline]
unsafe fn HUFv06_decodeStreamX2(
    mut p: *mut u8,
    bitDPtr: *mut BITv06_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv06_DEltX2,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv06_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv06_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p <= pEnd.offset(-(4))
    {
        if MEM_64bits() != 0 {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh14 = p;
            p = p.offset(1);
            *fresh14 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while BITv06_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv06_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p < pEnd
    {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    pEnd.offset_from(pStart) as core::ffi::c_long as size_t
}
pub unsafe fn HUFv06_decompress1X2_usingDTable(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const u16,
) -> size_t {
    let mut op = dst as *mut u8;
    let oend = op.add(dstSize);
    let dtLog = *DTable.offset(0) as u32;
    let mut dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX2).offset(1);
    let mut bitD = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let errorCode = BITv06_initDStream(&mut bitD, cSrc, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv06_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if BITv06_endOfDStream(&bitD) == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv06_decompress1X2(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [core::ffi::c_ushort; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let errorCode = HUFv06_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(errorCode);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv06_decompress1X2_usingDTable(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
pub unsafe fn HUFv06_decompress4X2_usingDTable(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const u16,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX2).offset(1);
    let dtLog = *DTable.offset(0) as u32;
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD2 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD3 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD4 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const core::ffi::c_void) as size_t;
    let mut length4: size_t = 0;
    let istart1 = istart.offset(6);
    let istart2 = istart1.add(length1);
    let istart3 = istart2.add(length2);
    let istart4 = istart3.add(length3);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let opStart2 = ostart.add(segmentSize);
    let opStart3 = opStart2.add(segmentSize);
    let opStart4 = opStart3.add(segmentSize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal: u32 = 0;
    length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    errorCode = BITv06_initDStream(&mut bitD1, istart1 as *const core::ffi::c_void, length1);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD2, istart2 as *const core::ffi::c_void, length2);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD3, istart3 as *const core::ffi::c_void, length3);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD4, istart4 as *const core::ffi::c_void, length4);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BITv06_reloadDStream(&mut bitD1) as core::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD2) as core::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD3) as core::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    while endSignal == BITv06_DStream_unfinished as core::ffi::c_int as u32
        && op4 < oend.offset(-(7))
    {
        if MEM_64bits() != 0 {
            let fresh18 = op1;
            op1 = op1.offset(1);
            *fresh18 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh19 = op2;
            op2 = op2.offset(1);
            *fresh19 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh20 = op3;
            op3 = op3.offset(1);
            *fresh20 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh21 = op4;
            op4 = op4.offset(1);
            *fresh21 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            let fresh22 = op1;
            op1 = op1.offset(1);
            *fresh22 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            let fresh23 = op2;
            op2 = op2.offset(1);
            *fresh23 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            let fresh24 = op3;
            op3 = op3.offset(1);
            *fresh24 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            let fresh25 = op4;
            op4 = op4.offset(1);
            *fresh25 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh26 = op1;
            op1 = op1.offset(1);
            *fresh26 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh27 = op2;
            op2 = op2.offset(1);
            *fresh27 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh28 = op3;
            op3 = op3.offset(1);
            *fresh28 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh29 = op4;
            op4 = op4.offset(1);
            *fresh29 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh30 = op1;
        op1 = op1.offset(1);
        *fresh30 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh31 = op2;
        op2 = op2.offset(1);
        *fresh31 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh32 = op3;
        op3 = op3.offset(1);
        *fresh32 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh33 = op4;
        op4 = op4.offset(1);
        *fresh33 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        endSignal = BITv06_reloadDStream(&mut bitD1) as core::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD2) as core::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD3) as core::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    HUFv06_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv06_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv06_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv06_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv06_endOfDStream(&bitD1)
        & BITv06_endOfDStream(&bitD2)
        & BITv06_endOfDStream(&bitD3)
        & BITv06_endOfDStream(&bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv06_decompress4X2(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [core::ffi::c_ushort; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let errorCode = HUFv06_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(errorCode);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv06_decompress4X2_usingDTable(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
unsafe fn HUFv06_fillDTableX4Level2(
    mut DTable: *mut HUFv06_DEltX4,
    mut sizeLog: u32,
    consumed: u32,
    mut rankValOrigin: *const u32,
    minWeight: core::ffi::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    sortedListSize: u32,
    mut nbBitsBaseline: u32,
    mut baseSeq: u16,
) {
    let mut DElt = HUFv06_DEltX4 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let mut rankVal: [u32; 17] = [0; 17];
    memcpy(
        rankVal.as_mut_ptr() as *mut core::ffi::c_void,
        rankValOrigin as *const core::ffi::c_void,
        ::core::mem::size_of::<[u32; 17]>() as size_t,
    );
    if minWeight > 1 {
        let mut i: u32 = 0;
        let mut skipSize = *rankVal.as_mut_ptr().offset(minWeight as isize);
        MEM_writeLE16(
            &mut DElt.sequence as *mut u16 as *mut core::ffi::c_void,
            baseSeq,
        );
        DElt.nbBits = consumed as u8;
        DElt.length = 1;
        i = 0;
        while i < skipSize {
            *DTable.offset(i as isize) = DElt;
            i = i.wrapping_add(1);
        }
    }
    let mut s: u32 = 0;
    s = 0;
    while s < sortedListSize {
        let symbol = (*sortedSymbols.offset(s as isize)).symbol as u32;
        let weight = (*sortedSymbols.offset(s as isize)).weight as u32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let length = ((1) << sizeLog.wrapping_sub(nbBits)) as u32;
        let start = *rankVal.as_mut_ptr().offset(weight as isize);
        let mut i_0 = start;
        let end = start.wrapping_add(length);
        MEM_writeLE16(
            &mut DElt.sequence as *mut u16 as *mut core::ffi::c_void,
            (baseSeq as u32).wrapping_add(symbol << 8) as u16,
        );
        DElt.nbBits = nbBits.wrapping_add(consumed) as u8;
        DElt.length = 2;
        loop {
            let fresh34 = i_0;
            i_0 = i_0.wrapping_add(1);
            *DTable.offset(fresh34 as isize) = DElt;
            if i_0 >= end {
                break;
            }
        }
        let fresh35 = &mut (*rankVal.as_mut_ptr().offset(weight as isize));
        *fresh35 = (*fresh35).wrapping_add(length);
        s = s.wrapping_add(1);
    }
}
unsafe fn HUFv06_fillDTableX4(
    mut DTable: *mut HUFv06_DEltX4,
    targetLog: u32,
    mut sortedList: *const sortedSymbol_t,
    sortedListSize: u32,
    mut rankStart: *const u32,
    mut rankValOrigin: *mut [u32; 17],
    maxWeight: u32,
    nbBitsBaseline: u32,
) {
    let mut rankVal: [u32; 17] = [0; 17];
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as core::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s: u32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut core::ffi::c_void,
        rankValOrigin as *const core::ffi::c_void,
        ::core::mem::size_of::<[u32; 17]>() as size_t,
    );
    s = 0;
    while s < sortedListSize {
        let symbol = (*sortedList.offset(s as isize)).symbol as u16;
        let weight = (*sortedList.offset(s as isize)).weight as u32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let start = *rankVal.as_mut_ptr().offset(weight as isize);
        let length = ((1) << targetLog.wrapping_sub(nbBits)) as u32;
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut sortedRank: u32 = 0;
            let mut minWeight = nbBits.wrapping_add(scaleLog as u32) as core::ffi::c_int;
            if minWeight < 1 {
                minWeight = 1;
            }
            sortedRank = *rankStart.offset(minWeight as isize);
            HUFv06_fillDTableX4Level2(
                DTable.offset(start as isize),
                targetLog.wrapping_sub(nbBits),
                nbBits,
                (*rankValOrigin.offset(nbBits as isize)).as_mut_ptr(),
                minWeight,
                sortedList.offset(sortedRank as isize),
                sortedListSize.wrapping_sub(sortedRank),
                nbBitsBaseline,
                symbol,
            );
        } else {
            let mut DElt = HUFv06_DEltX4 {
                sequence: 0,
                nbBits: 0,
                length: 0,
            };
            MEM_writeLE16(
                &mut DElt.sequence as *mut u16 as *mut core::ffi::c_void,
                symbol,
            );
            DElt.nbBits = nbBits as u8;
            DElt.length = 1;
            let mut u: u32 = 0;
            let end = start.wrapping_add(length);
            u = start;
            while u < end {
                *DTable.offset(u as isize) = DElt;
                u = u.wrapping_add(1);
            }
        }
        let fresh36 = &mut (*rankVal.as_mut_ptr().offset(weight as isize));
        *fresh36 = (*fresh36).wrapping_add(length);
        s = s.wrapping_add(1);
    }
}
pub unsafe fn HUFv06_readDTableX4(
    mut DTable: *mut u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightList: [u8; 256] = [0; 256];
    let mut sortedSymbol: [sortedSymbol_t; 256] = [sortedSymbol_t {
        symbol: 0,
        weight: 0,
    }; 256];
    let mut rankStats: [u32; 17] = [0; 17];
    let mut rankStart0: [u32; 18] = [0; 18];
    let rankStart = rankStart0.as_mut_ptr().offset(1);
    let mut rankVal: rankVal_t = [[0; 17]; 16];
    let mut tableLog: u32 = 0;
    let mut maxW: u32 = 0;
    let mut sizeOfSort: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let memLog = *DTable.offset(0);
    let mut iSize: size_t = 0;
    let mut dtPtr = DTable as *mut core::ffi::c_void;
    let dt = (dtPtr as *mut HUFv06_DEltX4).offset(1);
    if memLog > HUFv06_ABSOLUTEMAX_TABLELOG as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    iSize = HUFv06_readStats(
        weightList.as_mut_ptr(),
        (HUFv06_MAX_SYMBOL_VALUE + 1) as size_t,
        rankStats.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv06_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > memLog {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    maxW = tableLog;
    while *rankStats.as_mut_ptr().offset(maxW as isize) == 0 {
        maxW = maxW.wrapping_sub(1);
    }
    let mut w: u32 = 0;
    let mut nextRankStart = 0u32;
    w = 1;
    while w < maxW.wrapping_add(1) {
        let mut current = nextRankStart;
        nextRankStart = nextRankStart.wrapping_add(*rankStats.as_mut_ptr().offset(w as isize));
        *rankStart.offset(w as isize) = current;
        w = w.wrapping_add(1);
    }
    *rankStart.offset(0) = nextRankStart;
    sizeOfSort = nextRankStart;
    let mut s: u32 = 0;
    s = 0;
    while s < nbSymbols {
        let w_0 = *weightList.as_mut_ptr().offset(s as isize) as u32;
        let fresh37 = &mut (*rankStart.offset(w_0 as isize));
        let fresh38 = *fresh37;
        *fresh37 = (*fresh37).wrapping_add(1);
        let r = fresh38;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).symbol = s as u8;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).weight = w_0 as u8;
        s = s.wrapping_add(1);
    }
    *rankStart.offset(0) = 0;
    let rankVal0 = (*rankVal.as_mut_ptr().offset(0)).as_mut_ptr();
    let rescale = memLog.wrapping_sub(tableLog).wrapping_sub(1) as core::ffi::c_int;
    let mut nextRankVal = 0u32;
    let mut w_1: u32 = 0;
    w_1 = 1;
    while w_1 < maxW.wrapping_add(1) {
        let mut current_0 = nextRankVal;
        nextRankVal = nextRankVal.wrapping_add(
            *rankStats.as_mut_ptr().offset(w_1 as isize) << w_1.wrapping_add(rescale as u32),
        );
        *rankVal0.offset(w_1 as isize) = current_0;
        w_1 = w_1.wrapping_add(1);
    }
    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut consumed: u32 = 0;
    consumed = minBits;
    while consumed < memLog.wrapping_sub(minBits).wrapping_add(1) {
        let rankValPtr = (*rankVal.as_mut_ptr().offset(consumed as isize)).as_mut_ptr();
        let mut w_2: u32 = 0;
        w_2 = 1;
        while w_2 < maxW.wrapping_add(1) {
            *rankValPtr.offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
        }
        consumed = consumed.wrapping_add(1);
    }
    HUFv06_fillDTableX4(
        dt,
        memLog,
        sortedSymbol.as_mut_ptr(),
        sizeOfSort,
        rankStart0.as_mut_ptr(),
        rankVal.as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1),
    );
    iSize
}
unsafe fn HUFv06_decodeSymbolX4(
    mut op: *mut core::ffi::c_void,
    mut DStream: *mut BITv06_DStream_t,
    mut dt: *const HUFv06_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv06_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.add(val) as *const core::ffi::c_void, 2);
    BITv06_skipBits(DStream, (*dt.add(val)).nbBits as u32);
    (*dt.add(val)).length as u32
}
unsafe fn HUFv06_decodeLastSymbolX4(
    mut op: *mut core::ffi::c_void,
    mut DStream: *mut BITv06_DStream_t,
    mut dt: *const HUFv06_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv06_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.add(val) as *const core::ffi::c_void, 1);
    if (*dt.add(val)).length as core::ffi::c_int == 1 {
        BITv06_skipBits(DStream, (*dt.add(val)).nbBits as u32);
    } else if ((*DStream).bitsConsumed as size_t)
        < (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
    {
        BITv06_skipBits(DStream, (*dt.add(val)).nbBits as u32);
        if (*DStream).bitsConsumed as size_t
            > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
        {
            (*DStream).bitsConsumed =
                (::core::mem::size_of::<size_t>()).wrapping_mul(8) as core::ffi::c_uint;
        }
    }
    1
}
#[inline]
unsafe fn HUFv06_decodeStreamX4(
    mut p: *mut u8,
    mut bitDPtr: *mut BITv06_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv06_DEltX4,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv06_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv06_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p < pEnd.offset(-(7))
    {
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv06_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            p = p.offset(
                HUFv06_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv06_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        p = p.offset(
            HUFv06_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while BITv06_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv06_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p <= pEnd.offset(-(2))
    {
        p = p.offset(
            HUFv06_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while p <= pEnd.offset(-(2)) {
        p = p.offset(
            HUFv06_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    if p < pEnd {
        p = p.offset(
            HUFv06_decodeLastSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as core::ffi::c_long as size_t
}
pub unsafe fn HUFv06_decompress1X4_usingDTable(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const u32,
) -> size_t {
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtLog = *DTable.offset(0);
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX4).offset(1);
    let mut bitD = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let errorCode = BITv06_initDStream(&mut bitD, istart as *const core::ffi::c_void, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv06_decodeStreamX4(ostart, &mut bitD, oend, dt, dtLog);
    if BITv06_endOfDStream(&bitD) == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv06_decompress1X4(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [core::ffi::c_uint; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let hSize = HUFv06_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(hSize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv06_decompress1X4_usingDTable(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
pub unsafe fn HUFv06_decompress4X4_usingDTable(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const u32,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX4).offset(1);
    let dtLog = *DTable.offset(0);
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD2 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD3 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD4 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const core::ffi::c_void) as size_t;
    let mut length4: size_t = 0;
    let istart1 = istart.offset(6);
    let istart2 = istart1.add(length1);
    let istart3 = istart2.add(length2);
    let istart4 = istart3.add(length3);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let opStart2 = ostart.add(segmentSize);
    let opStart3 = opStart2.add(segmentSize);
    let opStart4 = opStart3.add(segmentSize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal: u32 = 0;
    length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    errorCode = BITv06_initDStream(&mut bitD1, istart1 as *const core::ffi::c_void, length1);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD2, istart2 as *const core::ffi::c_void, length2);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD3, istart3 as *const core::ffi::c_void, length3);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD4, istart4 as *const core::ffi::c_void, length4);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BITv06_reloadDStream(&mut bitD1) as core::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD2) as core::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD3) as core::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    while endSignal == BITv06_DStream_unfinished as core::ffi::c_int as u32
        && op4 < oend.offset(-(7))
    {
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv06_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv06_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv06_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv06_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            op1 = op1.offset(HUFv06_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            op2 = op2.offset(HUFv06_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            op3 = op3.offset(HUFv06_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 {
            op4 = op4.offset(HUFv06_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv06_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv06_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv06_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv06_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        op1 = op1.offset(
            HUFv06_decodeSymbolX4(op1 as *mut core::ffi::c_void, &mut bitD1, dt, dtLog) as isize,
        );
        op2 = op2.offset(
            HUFv06_decodeSymbolX4(op2 as *mut core::ffi::c_void, &mut bitD2, dt, dtLog) as isize,
        );
        op3 = op3.offset(
            HUFv06_decodeSymbolX4(op3 as *mut core::ffi::c_void, &mut bitD3, dt, dtLog) as isize,
        );
        op4 = op4.offset(
            HUFv06_decodeSymbolX4(op4 as *mut core::ffi::c_void, &mut bitD4, dt, dtLog) as isize,
        );
        endSignal = BITv06_reloadDStream(&mut bitD1) as core::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD2) as core::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD3) as core::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    HUFv06_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv06_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv06_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv06_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv06_endOfDStream(&bitD1)
        & BITv06_endOfDStream(&bitD2)
        & BITv06_endOfDStream(&bitD3)
        & BITv06_endOfDStream(&bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv06_decompress4X4(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [core::ffi::c_uint; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let mut hSize = HUFv06_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(hSize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv06_decompress4X4_usingDTable(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
static algoTime: [[algo_time_t; 3]; 16] = [
    [
        {
            algo_time_t {
                tableTime: 0,
                decode256Time: 0,
            }
        },
        {
            algo_time_t {
                tableTime: 1,
                decode256Time: 1,
            }
        },
        {
            algo_time_t {
                tableTime: 2,
                decode256Time: 2,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 0,
                decode256Time: 0,
            }
        },
        {
            algo_time_t {
                tableTime: 1,
                decode256Time: 1,
            }
        },
        {
            algo_time_t {
                tableTime: 2,
                decode256Time: 2,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 38,
                decode256Time: 130,
            }
        },
        {
            algo_time_t {
                tableTime: 1313,
                decode256Time: 74,
            }
        },
        {
            algo_time_t {
                tableTime: 2151,
                decode256Time: 38,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 448,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1353,
                decode256Time: 74,
            }
        },
        {
            algo_time_t {
                tableTime: 2238,
                decode256Time: 41,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 556,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1353,
                decode256Time: 74,
            }
        },
        {
            algo_time_t {
                tableTime: 2238,
                decode256Time: 47,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 714,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1418,
                decode256Time: 74,
            }
        },
        {
            algo_time_t {
                tableTime: 2436,
                decode256Time: 53,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 883,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1437,
                decode256Time: 74,
            }
        },
        {
            algo_time_t {
                tableTime: 2464,
                decode256Time: 61,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 897,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1515,
                decode256Time: 75,
            }
        },
        {
            algo_time_t {
                tableTime: 2622,
                decode256Time: 68,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 926,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1613,
                decode256Time: 75,
            }
        },
        {
            algo_time_t {
                tableTime: 2730,
                decode256Time: 75,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 947,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1729,
                decode256Time: 77,
            }
        },
        {
            algo_time_t {
                tableTime: 3359,
                decode256Time: 77,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1107,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 2083,
                decode256Time: 81,
            }
        },
        {
            algo_time_t {
                tableTime: 4006,
                decode256Time: 84,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1177,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 2379,
                decode256Time: 87,
            }
        },
        {
            algo_time_t {
                tableTime: 4785,
                decode256Time: 88,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1242,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 2415,
                decode256Time: 93,
            }
        },
        {
            algo_time_t {
                tableTime: 5155,
                decode256Time: 84,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1349,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 2644,
                decode256Time: 106,
            }
        },
        {
            algo_time_t {
                tableTime: 5260,
                decode256Time: 106,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1455,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 2422,
                decode256Time: 124,
            }
        },
        {
            algo_time_t {
                tableTime: 4174,
                decode256Time: 124,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 722,
                decode256Time: 128,
            }
        },
        {
            algo_time_t {
                tableTime: 1891,
                decode256Time: 145,
            }
        },
        {
            algo_time_t {
                tableTime: 1936,
                decode256Time: 146,
            }
        },
    ],
];
pub unsafe fn HUFv06_decompress(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    static decompress: [decompressionAlgo; 3] = unsafe {
        [
            Some(
                HUFv06_decompress4X2
                    as unsafe fn(
                        *mut core::ffi::c_void,
                        size_t,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(
                HUFv06_decompress4X4
                    as unsafe fn(
                        *mut core::ffi::c_void,
                        size_t,
                        *const core::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            ::core::mem::transmute::<libc::intptr_t, decompressionAlgo>(NULL as libc::intptr_t),
        ]
    };
    let mut Dtime: [u32; 3] = [0; 3];
    if dstSize == 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize;
    }
    if cSrcSize == 1 {
        memset(dst, *(cSrc as *const u8) as core::ffi::c_int, dstSize);
        return dstSize;
    }
    let Q = (cSrcSize * 16 / dstSize) as u32;
    let D256 = (dstSize >> 8) as u32;
    let mut n: u32 = 0;
    n = 0;
    while n < 3 {
        *Dtime.as_mut_ptr().offset(n as isize) = ((*(*algoTime.as_ptr().offset(Q as isize))
            .as_ptr()
            .offset(n as isize))
        .tableTime)
            .wrapping_add(
                (*(*algoTime.as_ptr().offset(Q as isize))
                    .as_ptr()
                    .offset(n as isize))
                .decode256Time
                    * D256,
            );
        n = n.wrapping_add(1);
    }
    let fresh39 = &mut (*Dtime.as_mut_ptr().offset(1));
    *fresh39 = (*fresh39).wrapping_add(*Dtime.as_mut_ptr().offset(1) >> 4);
    let fresh40 = &mut (*Dtime.as_mut_ptr().offset(2));
    *fresh40 = (*fresh40).wrapping_add(*Dtime.as_mut_ptr().offset(2) >> 3);
    let mut algoNb = 0;
    if *Dtime.as_mut_ptr().offset(1) < *Dtime.as_mut_ptr().offset(0) {
        algoNb = 1;
    }
    (*decompress.as_ptr().offset(algoNb as isize)).unwrap_unchecked()(dst, dstSize, cSrc, cSrcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_isError))]
pub unsafe extern "C" fn ZSTDv06_isError_0(mut code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_getErrorName))]
pub unsafe extern "C" fn ZSTDv06_getErrorName(mut code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZBUFFv06_isError))]
pub unsafe extern "C" fn ZBUFFv06_isError(mut errorCode: size_t) -> core::ffi::c_uint {
    ERR_isError(errorCode)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZBUFFv06_getErrorName))]
pub unsafe extern "C" fn ZBUFFv06_getErrorName(mut errorCode: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(errorCode)
}
pub const ZSTDv06_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const FSEv06_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const HUFv06_isError_0: fn(size_t) -> core::ffi::c_uint = ERR_isError;
unsafe extern "C" fn ZSTDv06_copy4(
    mut dst: *mut core::ffi::c_void,
    mut src: *const core::ffi::c_void,
) {
    memcpy(dst, src, 4);
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_sizeofDCtx))]
pub unsafe extern "C" fn ZSTDv06_sizeofDCtx() -> size_t {
    ::core::mem::size_of::<ZSTDv06_DCtx>() as size_t
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompressBegin))]
pub unsafe extern "C" fn ZSTDv06_decompressBegin(mut dctx: *mut ZSTDv06_DCtx) -> size_t {
    (*dctx).expected = ZSTDv06_frameHeaderSize_min;
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).previousDstEnd = NULL as *const core::ffi::c_void;
    (*dctx).base = NULL as *const core::ffi::c_void;
    (*dctx).vBase = NULL as *const core::ffi::c_void;
    (*dctx).dictEnd = NULL as *const core::ffi::c_void;
    *((*dctx).hufTableX4).as_mut_ptr().offset(0) =
        ZSTD_HUFFDTABLE_CAPACITY_LOG as core::ffi::c_uint;
    (*dctx).flagRepeatTable = 0;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_createDCtx))]
pub unsafe extern "C" fn ZSTDv06_createDCtx() -> *mut ZSTDv06_DCtx {
    let mut dctx = malloc(::core::mem::size_of::<ZSTDv06_DCtx>() as size_t) as *mut ZSTDv06_DCtx;
    if dctx.is_null() {
        return NULL as *mut ZSTDv06_DCtx;
    }
    ZSTDv06_decompressBegin(dctx);
    dctx
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_freeDCtx))]
pub unsafe extern "C" fn ZSTDv06_freeDCtx(mut dctx: *mut ZSTDv06_DCtx) -> size_t {
    free(dctx as *mut core::ffi::c_void);
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_copyDCtx))]
pub unsafe extern "C" fn ZSTDv06_copyDCtx(
    mut dstDCtx: *mut ZSTDv06_DCtx,
    mut srcDCtx: *const ZSTDv06_DCtx,
) {
    memcpy(
        dstDCtx as *mut core::ffi::c_void,
        srcDCtx as *const core::ffi::c_void,
        (::core::mem::size_of::<ZSTDv06_DCtx>() as size_t).wrapping_sub(
            ((ZSTDv06_BLOCKSIZE_MAX + WILDCOPY_OVERLENGTH) as size_t)
                .wrapping_add(ZSTDv06_frameHeaderSize_max),
        ),
    );
}
unsafe extern "C" fn ZSTDv06_frameHeaderSize(
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < ZSTDv06_frameHeaderSize_min {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let fcsId = (*(src as *const u8).offset(4) as core::ffi::c_int >> 6) as u32;
    ZSTDv06_frameHeaderSize_min.wrapping_add(*ZSTDv06_fcs_fieldSize.as_ptr().offset(fcsId as isize))
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_getFrameParams))]
pub unsafe extern "C" fn ZSTDv06_getFrameParams(
    mut fparamsPtr: *mut ZSTDv06_frameParams,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    if srcSize < ZSTDv06_frameHeaderSize_min {
        return ZSTDv06_frameHeaderSize_min;
    }
    if MEM_readLE32(src) != ZSTDv06_MAGICNUMBER {
        return -(ZSTD_error_prefix_unknown as core::ffi::c_int) as size_t;
    }
    let fhsize = ZSTDv06_frameHeaderSize(src, srcSize);
    if srcSize < fhsize {
        return fhsize;
    }
    ptr::write_bytes(
        fparamsPtr as *mut u8,
        0,
        ::core::mem::size_of::<ZSTDv06_frameParams>(),
    );
    let frameDesc = *ip.offset(4);
    (*fparamsPtr).windowLog = ((frameDesc as core::ffi::c_int & 0xf as core::ffi::c_int)
        + ZSTDv06_WINDOWLOG_ABSOLUTEMIN) as core::ffi::c_uint;
    if frameDesc as core::ffi::c_int & 0x20 as core::ffi::c_int != 0 {
        return -(ZSTD_error_frameParameter_unsupported as core::ffi::c_int) as size_t;
    }
    let mut current_block_14: u64;
    match frameDesc as core::ffi::c_int >> 6 {
        0 => {
            current_block_14 = 2128447534733725941;
        }
        1 => {
            (*fparamsPtr).frameContentSize = *ip.offset(5) as core::ffi::c_ulonglong;
            current_block_14 = 17407779659766490442;
        }
        2 => {
            (*fparamsPtr).frameContentSize = (MEM_readLE16(ip.offset(5) as *const core::ffi::c_void)
                as core::ffi::c_int
                + 256) as core::ffi::c_ulonglong;
            current_block_14 = 17407779659766490442;
        }
        3 => {
            (*fparamsPtr).frameContentSize =
                MEM_readLE64(ip.offset(5) as *const core::ffi::c_void) as core::ffi::c_ulonglong;
            current_block_14 = 17407779659766490442;
        }
        _ => {
            current_block_14 = 2128447534733725941;
        }
    }
    if current_block_14 == 2128447534733725941 {
        (*fparamsPtr).frameContentSize = 0;
    }
    0
}
unsafe fn ZSTDv06_decodeFrameHeader(
    mut zc: *mut ZSTDv06_DCtx,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let result = ZSTDv06_getFrameParams(&mut (*zc).fParams, src, srcSize);
    if MEM_32bits() != 0 && (*zc).fParams.windowLog > 25 {
        return -(ZSTD_error_frameParameter_unsupported as core::ffi::c_int) as size_t;
    }
    result
}
unsafe fn ZSTDv06_getcBlockSize(
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut bpPtr: *mut blockProperties_t,
) -> size_t {
    let in_0 = src as *const u8;
    let mut cSize: u32 = 0;
    if srcSize < ZSTDv06_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    (*bpPtr).blockType = (*in_0 as core::ffi::c_int >> 6) as blockType_t;
    cSize = (*in_0.offset(2) as core::ffi::c_int
        + ((*in_0.offset(1) as core::ffi::c_int) << 8)
        + ((*in_0.offset(0) as core::ffi::c_int & 7) << 16)) as u32;
    (*bpPtr).origSize = if (*bpPtr).blockType as core::ffi::c_uint
        == bt_rle as core::ffi::c_int as core::ffi::c_uint
    {
        cSize
    } else {
        0
    };
    if (*bpPtr).blockType as core::ffi::c_uint == bt_end as core::ffi::c_int as core::ffi::c_uint {
        return 0;
    }
    if (*bpPtr).blockType as core::ffi::c_uint == bt_rle as core::ffi::c_int as core::ffi::c_uint {
        return 1;
    }
    cSize as size_t
}
unsafe fn ZSTDv06_copyRawBlock(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if dst.is_null() {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if srcSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    memcpy(dst, src, srcSize);
    srcSize
}
unsafe fn ZSTDv06_decodeLiteralsBlock(
    mut dctx: *mut ZSTDv06_DCtx,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const u8;
    if srcSize < MIN_CBLOCK_SIZE as size_t {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    match *istart.offset(0) as core::ffi::c_int >> 6 {
        IS_HUF => {
            let mut litSize: size_t = 0;
            let mut litCSize: size_t = 0;
            let mut singleStream = 0;
            let mut lhSize = (*istart.offset(0) as core::ffi::c_int >> 4 & 3) as u32;
            if srcSize < 5 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            match lhSize {
                2 => {
                    lhSize = 4;
                    litSize = (((*istart.offset(0) as core::ffi::c_int & 15) << 10)
                        + ((*istart.offset(1) as core::ffi::c_int) << 2)
                        + (*istart.offset(2) as core::ffi::c_int >> 6))
                        as size_t;
                    litCSize = (((*istart.offset(2) as core::ffi::c_int & 63) << 8)
                        + *istart.offset(3) as core::ffi::c_int)
                        as size_t;
                }
                3 => {
                    lhSize = 5;
                    litSize = (((*istart.offset(0) as core::ffi::c_int & 15) << 14)
                        + ((*istart.offset(1) as core::ffi::c_int) << 6)
                        + (*istart.offset(2) as core::ffi::c_int >> 2))
                        as size_t;
                    litCSize = (((*istart.offset(2) as core::ffi::c_int & 3) << 16)
                        + ((*istart.offset(3) as core::ffi::c_int) << 8)
                        + *istart.offset(4) as core::ffi::c_int)
                        as size_t;
                }
                0 | 1 | _ => {
                    lhSize = 3;
                    singleStream = (*istart.offset(0) as core::ffi::c_int & 16) as size_t;
                    litSize = (((*istart.offset(0) as core::ffi::c_int & 15) << 6)
                        + (*istart.offset(1) as core::ffi::c_int >> 2))
                        as size_t;
                    litCSize = (((*istart.offset(1) as core::ffi::c_int & 3) << 8)
                        + *istart.offset(2) as core::ffi::c_int)
                        as size_t;
                }
            }
            if litSize > ZSTDv06_BLOCKSIZE_MAX as size_t {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if litCSize.wrapping_add(lhSize as size_t) > srcSize {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if ERR_isError(if singleStream != 0 {
                HUFv06_decompress1X2(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const core::ffi::c_void,
                    litCSize,
                )
            } else {
                HUFv06_decompress(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const core::ffi::c_void,
                    litCSize,
                )
            }) != 0
            {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize;
            ptr::write_bytes(
                ((*dctx).litBuffer).as_mut_ptr().add((*dctx).litSize) as *mut u8,
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            litCSize.wrapping_add(lhSize as size_t)
        }
        IS_PCH => {
            let mut litSize_0: size_t = 0;
            let mut litCSize_0: size_t = 0;
            let mut lhSize_0 = (*istart.offset(0) as core::ffi::c_int >> 4 & 3) as u32;
            if lhSize_0 != 1 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if (*dctx).flagRepeatTable == 0 {
                return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
            }
            lhSize_0 = 3;
            litSize_0 = (((*istart.offset(0) as core::ffi::c_int & 15) << 6)
                + (*istart.offset(1) as core::ffi::c_int >> 2)) as size_t;
            litCSize_0 = (((*istart.offset(1) as core::ffi::c_int & 3) << 8)
                + *istart.offset(2) as core::ffi::c_int) as size_t;
            if litCSize_0.wrapping_add(lhSize_0 as size_t) > srcSize {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            let errorCode = HUFv06_decompress1X4_usingDTable(
                ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                litSize_0,
                istart.offset(lhSize_0 as isize) as *const core::ffi::c_void,
                litCSize_0,
                ((*dctx).hufTableX4).as_mut_ptr(),
            );
            if ERR_isError(errorCode) != 0 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_0;
            ptr::write_bytes(
                ((*dctx).litBuffer).as_mut_ptr().add((*dctx).litSize) as *mut u8,
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            litCSize_0.wrapping_add(lhSize_0 as size_t)
        }
        IS_RAW => {
            let mut litSize_1: size_t = 0;
            let mut lhSize_1 = (*istart.offset(0) as core::ffi::c_int >> 4 & 3) as u32;
            match lhSize_1 {
                2 => {
                    litSize_1 = (((*istart.offset(0) as core::ffi::c_int & 15) << 8)
                        + *istart.offset(1) as core::ffi::c_int)
                        as size_t;
                }
                3 => {
                    litSize_1 = (((*istart.offset(0) as core::ffi::c_int & 15) << 16)
                        + ((*istart.offset(1) as core::ffi::c_int) << 8)
                        + *istart.offset(2) as core::ffi::c_int)
                        as size_t;
                }
                0 | 1 | _ => {
                    lhSize_1 = 1;
                    litSize_1 = (*istart.offset(0) as core::ffi::c_int & 31) as size_t;
                }
            }
            if (lhSize_1 as size_t)
                .wrapping_add(litSize_1)
                .wrapping_add(WILDCOPY_OVERLENGTH as size_t)
                > srcSize
            {
                if litSize_1.wrapping_add(lhSize_1 as size_t) > srcSize {
                    return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
                }
                memcpy(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    istart.offset(lhSize_1 as isize) as *const core::ffi::c_void,
                    litSize_1,
                );
                (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
                (*dctx).litSize = litSize_1;
                ptr::write_bytes(
                    ((*dctx).litBuffer).as_mut_ptr().add((*dctx).litSize) as *mut u8,
                    0,
                    WILDCOPY_OVERLENGTH as usize,
                );
                return (lhSize_1 as size_t).wrapping_add(litSize_1);
            }
            (*dctx).litPtr = istart.offset(lhSize_1 as isize);
            (*dctx).litSize = litSize_1;
            (lhSize_1 as size_t).wrapping_add(litSize_1)
        }
        IS_RLE => {
            let mut litSize_2: size_t = 0;
            let mut lhSize_2 = (*istart.offset(0) as core::ffi::c_int >> 4 & 3) as u32;
            match lhSize_2 {
                2 => {
                    litSize_2 = (((*istart.offset(0) as core::ffi::c_int & 15) << 8)
                        + *istart.offset(1) as core::ffi::c_int)
                        as size_t;
                }
                3 => {
                    litSize_2 = (((*istart.offset(0) as core::ffi::c_int & 15) << 16)
                        + ((*istart.offset(1) as core::ffi::c_int) << 8)
                        + *istart.offset(2) as core::ffi::c_int)
                        as size_t;
                    if srcSize < 4 {
                        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
                    }
                }
                0 | 1 | _ => {
                    lhSize_2 = 1;
                    litSize_2 = (*istart.offset(0) as core::ffi::c_int & 31) as size_t;
                }
            }
            if litSize_2 > ZSTDv06_BLOCKSIZE_MAX as size_t {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            memset(
                ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                *istart.offset(lhSize_2 as isize) as core::ffi::c_int,
                litSize_2.wrapping_add(WILDCOPY_OVERLENGTH as size_t),
            );
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_2;
            lhSize_2.wrapping_add(1) as size_t
        }
        _ => -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t,
    }
}
unsafe fn ZSTDv06_buildSeqTable(
    mut DTable: *mut FSEv06_DTable,
    mut type_0: u32,
    mut max: u32,
    mut maxLog: u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut defaultNorm: *const i16,
    mut defaultLog: u32,
    mut flagRepeatTable: u32,
) -> size_t {
    match type_0 {
        1 => {
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            if *(src as *const u8) as u32 > max {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            FSEv06_buildDTable_rle(DTable, *(src as *const u8));
            1
        }
        0 => {
            FSEv06_buildDTable(DTable, defaultNorm, max, defaultLog);
            0
        }
        2 => {
            if flagRepeatTable == 0 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            0
        }
        3 | _ => {
            let mut tableLog: u32 = 0;
            let mut norm: [i16; 53] = [0; 53];
            let headerSize =
                FSEv06_readNCount(norm.as_mut_ptr(), &mut max, &mut tableLog, src, srcSize);
            if ERR_isError(headerSize) != 0 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            FSEv06_buildDTable(DTable, norm.as_mut_ptr(), max, tableLog);
            headerSize
        }
    }
}
unsafe fn ZSTDv06_decodeSeqHeaders(
    mut nbSeqPtr: *mut core::ffi::c_int,
    mut DTableLL: *mut FSEv06_DTable,
    mut DTableML: *mut FSEv06_DTable,
    mut DTableOffb: *mut FSEv06_DTable,
    mut flagRepeatTable: u32,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let mut ip = istart;
    if srcSize < MIN_SEQUENCES_SIZE as size_t {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let fresh41 = ip;
    ip = ip.offset(1);
    let mut nbSeq = *fresh41 as core::ffi::c_int;
    if nbSeq == 0 {
        *nbSeqPtr = 0;
        return 1;
    }
    if nbSeq > 0x7f as core::ffi::c_int {
        if nbSeq == 0xff as core::ffi::c_int {
            if ip.offset(2) > iend {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            nbSeq = MEM_readLE16(ip as *const core::ffi::c_void) as core::ffi::c_int + LONGNBSEQ;
            ip = ip.offset(2);
        } else {
            if ip >= iend {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            let fresh42 = ip;
            ip = ip.offset(1);
            nbSeq = ((nbSeq - 0x80 as core::ffi::c_int) << 8) + *fresh42 as core::ffi::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.offset(4) > iend {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let LLtype = (*ip as core::ffi::c_int >> 6) as u32;
    let Offtype = (*ip as core::ffi::c_int >> 4 & 3) as u32;
    let MLtype = (*ip as core::ffi::c_int >> 2 & 3) as u32;
    ip = ip.offset(1);
    let bhSize = ZSTDv06_buildSeqTable(
        DTableLL,
        LLtype,
        MaxLL as u32,
        LLFSELog as u32,
        ip as *const core::ffi::c_void,
        iend.offset_from(ip) as core::ffi::c_long as size_t,
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(bhSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(bhSize);
    let bhSize_0 = ZSTDv06_buildSeqTable(
        DTableOffb,
        Offtype,
        MaxOff as u32,
        OffFSELog as u32,
        ip as *const core::ffi::c_void,
        iend.offset_from(ip) as core::ffi::c_long as size_t,
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(bhSize_0) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(bhSize_0);
    let bhSize_1 = ZSTDv06_buildSeqTable(
        DTableML,
        MLtype,
        MaxML as u32,
        MLFSELog as u32,
        ip as *const core::ffi::c_void,
        iend.offset_from(ip) as core::ffi::c_long as size_t,
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(bhSize_1) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(bhSize_1);
    ip.offset_from(istart) as core::ffi::c_long as size_t
}
unsafe fn ZSTDv06_decodeSequence(mut seq: *mut seq_t, mut seqState: *mut seqState_t) {
    let llCode = FSEv06_peekSymbol(&(*seqState).stateLL) as u32;
    let mlCode = FSEv06_peekSymbol(&(*seqState).stateML) as u32;
    let ofCode = FSEv06_peekSymbol(&(*seqState).stateOffb) as u32;
    let llBits = *LL_bits.as_ptr().offset(llCode as isize);
    let mlBits = *ML_bits.as_ptr().offset(mlCode as isize);
    let ofBits = ofCode;
    let totalBits = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    static LL_base: [u32; 36] = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        18,
        20,
        22,
        24,
        28,
        32,
        40,
        48,
        64,
        0x80 as core::ffi::c_int as u32,
        0x100 as core::ffi::c_int as u32,
        0x200 as core::ffi::c_int as u32,
        0x400 as core::ffi::c_int as u32,
        0x800 as core::ffi::c_int as u32,
        0x1000 as core::ffi::c_int as u32,
        0x2000 as core::ffi::c_int as u32,
        0x4000 as core::ffi::c_int as u32,
        0x8000 as core::ffi::c_int as u32,
        0x10000 as core::ffi::c_int as u32,
    ];
    static ML_base: [u32; 53] = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        24,
        25,
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        34,
        36,
        38,
        40,
        44,
        48,
        56,
        64,
        80,
        96,
        0x80 as core::ffi::c_int as u32,
        0x100 as core::ffi::c_int as u32,
        0x200 as core::ffi::c_int as u32,
        0x400 as core::ffi::c_int as u32,
        0x800 as core::ffi::c_int as u32,
        0x1000 as core::ffi::c_int as u32,
        0x2000 as core::ffi::c_int as u32,
        0x4000 as core::ffi::c_int as u32,
        0x8000 as core::ffi::c_int as u32,
        0x10000 as core::ffi::c_int as u32,
    ];
    static OF_base: [u32; 29] = [
        0,
        1,
        3,
        7,
        0xf as core::ffi::c_int as u32,
        0x1f as core::ffi::c_int as u32,
        0x3f as core::ffi::c_int as u32,
        0x7f as core::ffi::c_int as u32,
        0xff as core::ffi::c_int as u32,
        0x1ff as core::ffi::c_int as u32,
        0x3ff as core::ffi::c_int as u32,
        0x7ff as core::ffi::c_int as u32,
        0xfff as core::ffi::c_int as u32,
        0x1fff as core::ffi::c_int as u32,
        0x3fff as core::ffi::c_int as u32,
        0x7fff as core::ffi::c_int as u32,
        0xffff as core::ffi::c_int as u32,
        0x1ffff as core::ffi::c_int as u32,
        0x3ffff as core::ffi::c_int as u32,
        0x7ffff as core::ffi::c_int as u32,
        0xfffff as core::ffi::c_int as u32,
        0x1fffff as core::ffi::c_int as u32,
        0x3fffff as core::ffi::c_int as u32,
        0x7fffff as core::ffi::c_int as u32,
        0xffffff as core::ffi::c_int as u32,
        0x1ffffff as core::ffi::c_int as u32,
        0x3ffffff as core::ffi::c_int as u32,
        1,
        1,
    ];
    let mut offset: size_t = 0;
    if ofCode == 0 {
        offset = 0;
    } else {
        offset = (*OF_base.as_ptr().offset(ofCode as isize) as size_t)
            .wrapping_add(BITv06_readBits(&mut (*seqState).DStream, ofBits));
        if MEM_32bits() != 0 {
            BITv06_reloadDStream(&mut (*seqState).DStream);
        }
    }
    if offset < ZSTDv06_REP_NUM as size_t {
        if llCode == 0 && offset <= 1 {
            offset = (1 as size_t).wrapping_sub(offset);
        }
        if offset != 0 {
            let mut temp = *((*seqState).prevOffset).as_mut_ptr().add(offset);
            if offset != 1 {
                *((*seqState).prevOffset).as_mut_ptr().offset(2) =
                    *((*seqState).prevOffset).as_mut_ptr().offset(1);
            }
            *((*seqState).prevOffset).as_mut_ptr().offset(1) =
                *((*seqState).prevOffset).as_mut_ptr().offset(0);
            offset = temp;
            *((*seqState).prevOffset).as_mut_ptr().offset(0) = offset;
        } else {
            offset = *((*seqState).prevOffset).as_mut_ptr().offset(0);
        }
    } else {
        offset = offset.wrapping_sub(ZSTDv06_REP_MOVE as size_t);
        *((*seqState).prevOffset).as_mut_ptr().offset(2) =
            *((*seqState).prevOffset).as_mut_ptr().offset(1);
        *((*seqState).prevOffset).as_mut_ptr().offset(1) =
            *((*seqState).prevOffset).as_mut_ptr().offset(0);
        *((*seqState).prevOffset).as_mut_ptr().offset(0) = offset;
    }
    (*seq).offset = offset;
    (*seq).matchLength = ((*ML_base.as_ptr().offset(mlCode as isize)).wrapping_add(MINMATCH as u32)
        as size_t)
        .wrapping_add(if mlCode > 31 {
            BITv06_readBits(&mut (*seqState).DStream, mlBits)
        } else {
            0
        });
    if MEM_32bits() != 0 && mlBits.wrapping_add(llBits) > 24 {
        BITv06_reloadDStream(&mut (*seqState).DStream);
    }
    (*seq).litLength =
        (*LL_base.as_ptr().offset(llCode as isize) as size_t).wrapping_add(if llCode > 15 {
            BITv06_readBits(&mut (*seqState).DStream, llBits)
        } else {
            0
        });
    if MEM_32bits() != 0 || totalBits > (64 - 7 - (LLFSELog + MLFSELog + OffFSELog)) as u32 {
        BITv06_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv06_updateState(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    FSEv06_updateState(&mut (*seqState).stateML, &mut (*seqState).DStream);
    if MEM_32bits() != 0 {
        BITv06_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv06_updateState(&mut (*seqState).stateOffb, &mut (*seqState).DStream);
}
unsafe fn ZSTDv06_execSequence(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    base: *const u8,
    vBase: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.add(sequenceLength);
    let oend_8 = oend.wrapping_sub(8);
    let iLitEnd = (*litPtr).add(sequence.litLength);
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset);
    let seqLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    if seqLength > oend.offset_from(op) as core::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as core::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if oLitEnd > oend_8 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ZSTDv06_wildcopy(
        op as *mut core::ffi::c_void,
        *litPtr as *const core::ffi::c_void,
        sequence.litLength as ptrdiff_t,
    );
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(base) as core::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(vBase) as core::ffi::c_long as size_t {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(-(base.offset_from(match_0) as core::ffi::c_long as isize));
        if match_0.add(sequence.matchLength) <= dictEnd {
            memmove(
                oLitEnd as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                sequence.matchLength,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as core::ffi::c_long as size_t;
        memmove(
            oLitEnd as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            length1,
        );
        op = oLitEnd.add(length1);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = base;
        if op > oend_8 || sequence.matchLength < MINMATCH as size_t {
            while op < oMatchEnd {
                let fresh43 = match_0;
                match_0 = match_0.offset(1);
                let fresh44 = op;
                op = op.offset(1);
                *fresh44 = *fresh43;
            }
            return sequenceLength;
        }
    }
    if sequence.offset < 8 {
        static dec32table: [u32; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
        static dec64table: [core::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
        let sub2 = *dec64table.as_ptr().add(sequence.offset);
        *op.offset(0) = *match_0.offset(0);
        *op.offset(1) = *match_0.offset(1);
        *op.offset(2) = *match_0.offset(2);
        *op.offset(3) = *match_0.offset(3);
        match_0 = match_0.offset(*dec32table.as_ptr().add(sequence.offset) as isize);
        ZSTDv06_copy4(
            op.offset(4) as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
        );
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ZSTDv06_copy8(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
        );
    }
    op = op.offset(8);
    match_0 = match_0.offset(8);
    if oMatchEnd > oend.offset(-((16 - MINMATCH) as isize)) {
        if op < oend_8 {
            ZSTDv06_wildcopy(
                op as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                oend_8.offset_from(op) as core::ffi::c_long,
            );
            match_0 = match_0.offset(oend_8.offset_from(op) as core::ffi::c_long as isize);
            op = oend_8;
        }
        while op < oMatchEnd {
            let fresh45 = match_0;
            match_0 = match_0.offset(1);
            let fresh46 = op;
            op = op.offset(1);
            *fresh46 = *fresh45;
        }
    } else {
        ZSTDv06_wildcopy(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            sequence.matchLength as ptrdiff_t - 8,
        );
    }
    sequenceLength
}
unsafe fn ZSTDv06_decompressSequences(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const core::ffi::c_void,
    mut seqSize: size_t,
) -> size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.add(seqSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(maxDstSize);
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.add((*dctx).litSize);
    let mut DTableLL = ((*dctx).LLTable).as_mut_ptr();
    let mut DTableML = ((*dctx).MLTable).as_mut_ptr();
    let mut DTableOffb = ((*dctx).OffTable).as_mut_ptr();
    let base = (*dctx).base as *const u8;
    let vBase = (*dctx).vBase as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    let mut nbSeq: core::ffi::c_int = 0;
    let seqHSize = ZSTDv06_decodeSeqHeaders(
        &mut nbSeq,
        DTableLL,
        DTableML,
        DTableOffb,
        (*dctx).flagRepeatTable,
        ip as *const core::ffi::c_void,
        seqSize,
    );
    if ERR_isError(seqHSize) != 0 {
        return seqHSize;
    }
    ip = ip.add(seqHSize);
    (*dctx).flagRepeatTable = 0;
    if nbSeq != 0 {
        let mut sequence = seq_t {
            litLength: 0,
            matchLength: 0,
            offset: 0,
        };
        let mut seqState = seqState_t {
            DStream: BITv06_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: core::ptr::null::<core::ffi::c_char>(),
                start: core::ptr::null::<core::ffi::c_char>(),
            },
            stateLL: FSEv06_DState_t {
                state: 0,
                table: core::ptr::null::<core::ffi::c_void>(),
            },
            stateOffb: FSEv06_DState_t {
                state: 0,
                table: core::ptr::null::<core::ffi::c_void>(),
            },
            stateML: FSEv06_DState_t {
                state: 0,
                table: core::ptr::null::<core::ffi::c_void>(),
            },
            prevOffset: [0; 3],
        };
        ptr::write_bytes(
            &mut sequence as *mut seq_t as *mut u8,
            0,
            ::core::mem::size_of::<seq_t>(),
        );
        sequence.offset = REPCODE_STARTVALUE as size_t;
        let mut i: u32 = 0;
        i = 0;
        while i < ZSTDv06_REP_INIT as u32 {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) = REPCODE_STARTVALUE as size_t;
            i = i.wrapping_add(1);
        }
        let errorCode = BITv06_initDStream(
            &mut seqState.DStream,
            ip as *const core::ffi::c_void,
            iend.offset_from(ip) as core::ffi::c_long as size_t,
        );
        if ERR_isError(errorCode) != 0 {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        FSEv06_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
        FSEv06_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
        FSEv06_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
        while BITv06_reloadDStream(&mut seqState.DStream) as core::ffi::c_uint
            <= BITv06_DStream_completed as core::ffi::c_int as core::ffi::c_uint
            && nbSeq != 0
        {
            nbSeq -= 1;
            ZSTDv06_decodeSequence(&mut sequence, &mut seqState);
            let oneSeqSize = ZSTDv06_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litEnd,
                base,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize) != 0 {
                return oneSeqSize;
            }
            op = op.add(oneSeqSize);
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if litPtr > litEnd {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op.add(lastLLSize) > oend {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if lastLLSize > 0 {
        memcpy(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize,
        );
        op = op.add(lastLLSize);
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
unsafe fn ZSTDv06_checkContinuity(mut dctx: *mut ZSTDv06_DCtx, mut dst: *const core::ffi::c_void) {
    if dst != (*dctx).previousDstEnd {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx).vBase = (dst as *const core::ffi::c_char).offset(
            -(((*dctx).previousDstEnd as *const core::ffi::c_char)
                .offset_from((*dctx).base as *const core::ffi::c_char)
                as core::ffi::c_long as isize),
        ) as *const core::ffi::c_void;
        (*dctx).base = dst;
        (*dctx).previousDstEnd = dst;
    }
}
unsafe fn ZSTDv06_decompressBlock_internal(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    if srcSize >= ZSTDv06_BLOCKSIZE_MAX as size_t {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let litCSize = ZSTDv06_decodeLiteralsBlock(dctx, src, srcSize);
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.add(litCSize);
    srcSize = srcSize.wrapping_sub(litCSize);
    ZSTDv06_decompressSequences(
        dctx,
        dst,
        dstCapacity,
        ip as *const core::ffi::c_void,
        srcSize,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompressBlock))]
pub unsafe extern "C" fn ZSTDv06_decompressBlock(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv06_checkContinuity(dctx, dst);
    ZSTDv06_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize)
}
unsafe fn ZSTDv06_decompressFrame(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.wrapping_add(dstCapacity);
    let mut remainingSize = srcSize;
    let mut blockProperties = {
        blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        }
    };
    if srcSize < ZSTDv06_frameHeaderSize_min.wrapping_add(ZSTDv06_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let frameHeaderSize = ZSTDv06_frameHeaderSize(src, ZSTDv06_frameHeaderSize_min);
    if ERR_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv06_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if ZSTDv06_decodeFrameHeader(dctx, src, frameHeaderSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(frameHeaderSize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    loop {
        let mut decodedSize = 0;
        let cBlockSize = ZSTDv06_getcBlockSize(
            ip as *const core::ffi::c_void,
            iend.offset_from(ip) as core::ffi::c_long as size_t,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            return cBlockSize;
        }
        ip = ip.add(ZSTDv06_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv06_blockHeaderSize);
        if cBlockSize > remainingSize {
            return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
        }
        match blockProperties.blockType as core::ffi::c_uint {
            0 => {
                decodedSize = ZSTDv06_decompressBlock_internal(
                    dctx,
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as core::ffi::c_long as size_t,
                    ip as *const core::ffi::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTDv06_copyRawBlock(
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as core::ffi::c_long as size_t,
                    ip as *const core::ffi::c_void,
                    cBlockSize,
                );
            }
            2 => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
            3 => {
                if remainingSize != 0 {
                    return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
                }
            }
            _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
        }
        if cBlockSize == 0 {
            break;
        }
        if ERR_isError(decodedSize) != 0 {
            return decodedSize;
        }
        op = op.add(decodedSize);
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompress_usingPreparedDCtx))]
pub unsafe extern "C" fn ZSTDv06_decompress_usingPreparedDCtx(
    mut dctx: *mut ZSTDv06_DCtx,
    mut refDCtx: *const ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv06_copyDCtx(dctx, refDCtx);
    ZSTDv06_checkContinuity(dctx, dst);
    ZSTDv06_decompressFrame(dctx, dst, dstCapacity, src, srcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompress_usingDict))]
pub unsafe extern "C" fn ZSTDv06_decompress_usingDict(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    ZSTDv06_decompressBegin_usingDict(dctx, dict, dictSize);
    ZSTDv06_checkContinuity(dctx, dst);
    ZSTDv06_decompressFrame(dctx, dst, dstCapacity, src, srcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompressDCtx))]
pub unsafe extern "C" fn ZSTDv06_decompressDCtx(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv06_decompress_usingDict(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        NULL as *const core::ffi::c_void,
        0,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompress))]
pub unsafe extern "C" fn ZSTDv06_decompress(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut regenSize: size_t = 0;
    let mut dctx = ZSTDv06_createDCtx();
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    }
    regenSize = ZSTDv06_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTDv06_freeDCtx(dctx);
    regenSize
}
unsafe fn ZSTD_errorFrameSizeInfoLegacy(
    mut cSize: *mut size_t,
    mut dBound: *mut core::ffi::c_ulonglong,
    mut ret: size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_findFrameSizeInfoLegacy))]
pub unsafe extern "C" fn ZSTDv06_findFrameSizeInfoLegacy(
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut cSize: *mut size_t,
    mut dBound: *mut core::ffi::c_ulonglong,
) {
    let mut ip = src as *const u8;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as size_t;
    let mut blockProperties = {
        blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        }
    };
    let frameHeaderSize = ZSTDv06_frameHeaderSize(src, srcSize);
    if ERR_isError(frameHeaderSize) != 0 {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, frameHeaderSize);
        return;
    }
    if MEM_readLE32(src) != ZSTDv06_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_prefix_unknown as core::ffi::c_int) as size_t,
        );
        return;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv06_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t,
        );
        return;
    }
    ip = ip.add(frameHeaderSize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    loop {
        let cBlockSize = ZSTDv06_getcBlockSize(
            ip as *const core::ffi::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, cBlockSize);
            return;
        }
        ip = ip.add(ZSTDv06_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv06_blockHeaderSize);
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(
                cSize,
                dBound,
                -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t,
            );
            return;
        }
        if cBlockSize == 0 {
            break;
        }
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const u8) as core::ffi::c_long as size_t;
    *dBound = (nbBlocks * ZSTDv06_BLOCKSIZE_MAX as size_t) as core::ffi::c_ulonglong;
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_nextSrcSizeToDecompress))]
pub unsafe extern "C" fn ZSTDv06_nextSrcSizeToDecompress(mut dctx: *mut ZSTDv06_DCtx) -> size_t {
    (*dctx).expected
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompressContinue))]
pub unsafe extern "C" fn ZSTDv06_decompressContinue(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize != (*dctx).expected {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if dstCapacity != 0 {
        ZSTDv06_checkContinuity(dctx, dst);
    }
    match (*dctx).stage as core::ffi::c_uint {
        0 => {
            if srcSize != ZSTDv06_frameHeaderSize_min {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            (*dctx).headerSize = ZSTDv06_frameHeaderSize(src, ZSTDv06_frameHeaderSize_min);
            if ERR_isError((*dctx).headerSize) != 0 {
                return (*dctx).headerSize;
            }
            memcpy(
                ((*dctx).headerBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                src,
                ZSTDv06_frameHeaderSize_min,
            );
            if (*dctx).headerSize > ZSTDv06_frameHeaderSize_min {
                (*dctx).expected = ((*dctx).headerSize).wrapping_sub(ZSTDv06_frameHeaderSize_min);
                (*dctx).stage = ZSTDds_decodeFrameHeader;
                return 0;
            }
            (*dctx).expected = 0;
        }
        1 => {}
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_compressed,
                origSize: 0,
            };
            let cBlockSize = ZSTDv06_getcBlockSize(src, ZSTDv06_blockHeaderSize, &mut bp);
            if ERR_isError(cBlockSize) != 0 {
                return cBlockSize;
            }
            if bp.blockType as core::ffi::c_uint == bt_end as core::ffi::c_int as core::ffi::c_uint
            {
                (*dctx).expected = 0;
                (*dctx).stage = ZSTDds_getFrameHeaderSize;
            } else {
                (*dctx).expected = cBlockSize;
                (*dctx).bType = bp.blockType;
                (*dctx).stage = ZSTDds_decompressBlock;
            }
            return 0;
        }
        3 => {
            let mut rSize: size_t = 0;
            match (*dctx).bType as core::ffi::c_uint {
                0 => {
                    rSize = ZSTDv06_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize);
                }
                1 => {
                    rSize = ZSTDv06_copyRawBlock(dst, dstCapacity, src, srcSize);
                }
                2 => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
                3 => {
                    rSize = 0;
                }
                _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
            }
            (*dctx).stage = ZSTDds_decodeBlockHeader;
            (*dctx).expected = ZSTDv06_blockHeaderSize;
            if ERR_isError(rSize) != 0 {
                return rSize;
            }
            (*dctx).previousDstEnd =
                (dst as *mut core::ffi::c_char).add(rSize) as *const core::ffi::c_void;
            return rSize;
        }
        _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
    }
    let mut result: size_t = 0;
    memcpy(
        ((*dctx).headerBuffer)
            .as_mut_ptr()
            .add(ZSTDv06_frameHeaderSize_min) as *mut core::ffi::c_void,
        src,
        (*dctx).expected,
    );
    result = ZSTDv06_decodeFrameHeader(
        dctx,
        ((*dctx).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
        (*dctx).headerSize,
    );
    if ERR_isError(result) != 0 {
        return result;
    }
    (*dctx).expected = ZSTDv06_blockHeaderSize;
    (*dctx).stage = ZSTDds_decodeBlockHeader;
    0
}
unsafe fn ZSTDv06_refDictContent(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).vBase = (dict as *const core::ffi::c_char).offset(
        -(((*dctx).previousDstEnd as *const core::ffi::c_char)
            .offset_from((*dctx).base as *const core::ffi::c_char) as core::ffi::c_long
            as isize),
    ) as *const core::ffi::c_void;
    (*dctx).base = dict;
    (*dctx).previousDstEnd =
        (dict as *const core::ffi::c_char).add(dictSize) as *const core::ffi::c_void;
}
unsafe fn ZSTDv06_loadEntropy(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut hSize: size_t = 0;
    let mut offcodeHeaderSize: size_t = 0;
    let mut matchlengthHeaderSize: size_t = 0;
    let mut litlengthHeaderSize: size_t = 0;
    hSize = HUFv06_readDTableX4(((*dctx).hufTableX4).as_mut_ptr(), dict, dictSize);
    if ERR_isError(hSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dict = (dict as *const core::ffi::c_char).add(hSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(hSize);
    let mut offcodeNCount: [core::ffi::c_short; 29] = [0; 29];
    let mut offcodeMaxValue = MaxOff as u32;
    let mut offcodeLog: u32 = 0;
    offcodeHeaderSize = FSEv06_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dict,
        dictSize,
    );
    if ERR_isError(offcodeHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    if offcodeLog > OffFSELog as u32 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    let errorCode = FSEv06_buildDTable(
        ((*dctx).OffTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        offcodeLog,
    );
    if ERR_isError(errorCode) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dict = (dict as *const core::ffi::c_char).add(offcodeHeaderSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(offcodeHeaderSize);
    let mut matchlengthNCount: [core::ffi::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as core::ffi::c_uint;
    let mut matchlengthLog: core::ffi::c_uint = 0;
    matchlengthHeaderSize = FSEv06_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dict,
        dictSize,
    );
    if ERR_isError(matchlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    if matchlengthLog > MLFSELog as core::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    let errorCode_0 = FSEv06_buildDTable(
        ((*dctx).MLTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
    );
    if ERR_isError(errorCode_0) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dict =
        (dict as *const core::ffi::c_char).add(matchlengthHeaderSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(matchlengthHeaderSize);
    let mut litlengthNCount: [core::ffi::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as core::ffi::c_uint;
    let mut litlengthLog: core::ffi::c_uint = 0;
    litlengthHeaderSize = FSEv06_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dict,
        dictSize,
    );
    if ERR_isError(litlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    if litlengthLog > LLFSELog as core::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    let errorCode_1 = FSEv06_buildDTable(
        ((*dctx).LLTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
    );
    if ERR_isError(errorCode_1) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    (*dctx).flagRepeatTable = 1;
    hSize
        .wrapping_add(offcodeHeaderSize)
        .wrapping_add(matchlengthHeaderSize)
        .wrapping_add(litlengthHeaderSize)
}
unsafe fn ZSTDv06_decompress_insertDictionary(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut eSize: size_t = 0;
    let magic = MEM_readLE32(dict);
    if magic != ZSTDv06_DICT_MAGIC {
        ZSTDv06_refDictContent(dctx, dict, dictSize);
        return 0;
    }
    dict = (dict as *const core::ffi::c_char).offset(4) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(4);
    eSize = ZSTDv06_loadEntropy(dctx, dict, dictSize);
    if ERR_isError(eSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dict = (dict as *const core::ffi::c_char).add(eSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(eSize);
    ZSTDv06_refDictContent(dctx, dict, dictSize);
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv06_decompressBegin_usingDict))]
pub unsafe extern "C" fn ZSTDv06_decompressBegin_usingDict(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let errorCode = ZSTDv06_decompressBegin(dctx);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    if !dict.is_null() && dictSize != 0 {
        let errorCode_0 = ZSTDv06_decompress_insertDictionary(dctx, dict, dictSize);
        if ERR_isError(errorCode_0) != 0 {
            return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
        }
    }
    0
}
pub unsafe fn ZBUFFv06_createDCtx() -> *mut ZBUFFv06_DCtx {
    let mut zbd = malloc(::core::mem::size_of::<ZBUFFv06_DCtx>() as size_t) as *mut ZBUFFv06_DCtx;
    if zbd.is_null() {
        return NULL as *mut ZBUFFv06_DCtx;
    }
    ptr::write_bytes(zbd as *mut u8, 0, ::core::mem::size_of::<ZBUFFv06_DCtx>());
    (*zbd).zd = ZSTDv06_createDCtx();
    if ((*zbd).zd).is_null() {
        ZBUFFv06_freeDCtx(zbd);
        return NULL as *mut ZBUFFv06_DCtx;
    }
    (*zbd).stage = ZBUFFds_init;
    zbd
}
pub unsafe fn ZBUFFv06_freeDCtx(mut zbd: *mut ZBUFFv06_DCtx) -> size_t {
    if zbd.is_null() {
        return 0;
    }
    ZSTDv06_freeDCtx((*zbd).zd);
    free((*zbd).inBuff as *mut core::ffi::c_void);
    free((*zbd).outBuff as *mut core::ffi::c_void);
    free(zbd as *mut core::ffi::c_void);
    0
}
pub unsafe fn ZBUFFv06_decompressInitDictionary(
    mut zbd: *mut ZBUFFv06_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    (*zbd).stage = ZBUFFds_loadHeader;
    (*zbd).outEnd = 0;
    (*zbd).outStart = (*zbd).outEnd;
    (*zbd).inPos = (*zbd).outStart;
    (*zbd).lhSize = (*zbd).inPos;
    ZSTDv06_decompressBegin_usingDict((*zbd).zd, dict, dictSize)
}
pub unsafe fn ZBUFFv06_decompressInit(mut zbd: *mut ZBUFFv06_DCtx) -> size_t {
    ZBUFFv06_decompressInitDictionary(zbd, NULL as *const core::ffi::c_void, 0)
}
#[inline]
unsafe fn ZBUFFv06_limitCopy(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut length = if dstCapacity < srcSize {
        dstCapacity
    } else {
        srcSize
    };
    if length > 0 {
        memcpy(dst, src, length);
    }
    length
}
pub unsafe fn ZBUFFv06_decompressContinue(
    mut zbd: *mut ZBUFFv06_DCtx,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacityPtr: *mut size_t,
    mut src: *const core::ffi::c_void,
    mut srcSizePtr: *mut size_t,
) -> size_t {
    let istart = src as *const core::ffi::c_char;
    let iend = istart.add(*srcSizePtr);
    let mut ip = istart;
    let ostart = dst as *mut core::ffi::c_char;
    let oend = ostart.add(*dstCapacityPtr);
    let mut op = ostart;
    let mut notDone = 1;
    while notDone != 0 {
        let mut current_block_65: u64;
        match (*zbd).stage as core::ffi::c_uint {
            0 => return -(ZSTD_error_init_missing as core::ffi::c_int) as size_t,
            1 => {
                let hSize = ZSTDv06_getFrameParams(
                    &mut (*zbd).fParams,
                    ((*zbd).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
                    (*zbd).lhSize,
                );
                if hSize != 0 {
                    let toLoad = hSize.wrapping_sub((*zbd).lhSize);
                    if ERR_isError(hSize) != 0 {
                        return hSize;
                    }
                    if toLoad > iend.offset_from(ip) as core::ffi::c_long as size_t {
                        if !ip.is_null() {
                            memcpy(
                                ((*zbd).headerBuffer).as_mut_ptr().add((*zbd).lhSize)
                                    as *mut core::ffi::c_void,
                                ip as *const core::ffi::c_void,
                                iend.offset_from(ip) as size_t,
                            );
                        }
                        (*zbd).lhSize = ((*zbd).lhSize)
                            .wrapping_add(iend.offset_from(ip) as core::ffi::c_long as size_t);
                        *dstCapacityPtr = 0;
                        return hSize
                            .wrapping_sub((*zbd).lhSize)
                            .wrapping_add(ZSTDv06_blockHeaderSize);
                    }
                    memcpy(
                        ((*zbd).headerBuffer).as_mut_ptr().add((*zbd).lhSize)
                            as *mut core::ffi::c_void,
                        ip as *const core::ffi::c_void,
                        toLoad,
                    );
                    (*zbd).lhSize = hSize;
                    ip = ip.add(toLoad);
                    current_block_65 = 13853033528615664019;
                } else {
                    let h1Size = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
                    let h1Result = ZSTDv06_decompressContinue(
                        (*zbd).zd,
                        NULL as *mut core::ffi::c_void,
                        0,
                        ((*zbd).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
                        h1Size,
                    );
                    if ERR_isError(h1Result) != 0 {
                        return h1Result;
                    }
                    if h1Size < (*zbd).lhSize {
                        let h2Size = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
                        let h2Result = ZSTDv06_decompressContinue(
                            (*zbd).zd,
                            NULL as *mut core::ffi::c_void,
                            0,
                            ((*zbd).headerBuffer).as_mut_ptr().add(h1Size)
                                as *const core::ffi::c_void,
                            h2Size,
                        );
                        if ERR_isError(h2Result) != 0 {
                            return h2Result;
                        }
                    }
                    let blockSize = (if (1) << (*zbd).fParams.windowLog < 128 * 1024 {
                        (1) << (*zbd).fParams.windowLog
                    } else {
                        128 * 1024
                    }) as size_t;
                    (*zbd).blockSize = blockSize;
                    if (*zbd).inBuffSize < blockSize {
                        free((*zbd).inBuff as *mut core::ffi::c_void);
                        (*zbd).inBuffSize = blockSize;
                        (*zbd).inBuff = malloc(blockSize) as *mut core::ffi::c_char;
                        if ((*zbd).inBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
                        }
                    }
                    let neededOutSize = ((1 as size_t) << (*zbd).fParams.windowLog)
                        .wrapping_add(blockSize)
                        .wrapping_add((WILDCOPY_OVERLENGTH * 2) as size_t);
                    if (*zbd).outBuffSize < neededOutSize {
                        free((*zbd).outBuff as *mut core::ffi::c_void);
                        (*zbd).outBuffSize = neededOutSize;
                        (*zbd).outBuff = malloc(neededOutSize) as *mut core::ffi::c_char;
                        if ((*zbd).outBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
                        }
                    }
                    (*zbd).stage = ZBUFFds_read;
                    current_block_65 = 11048769245176032998;
                }
            }
            2 => {
                current_block_65 = 11048769245176032998;
            }
            3 => {
                current_block_65 = 14220266465818359136;
            }
            4 => {
                current_block_65 = 15594603006322722090;
            }
            _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
        }
        if current_block_65 == 11048769245176032998 {
            let neededInSize = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
            if neededInSize == 0 {
                (*zbd).stage = ZBUFFds_init;
                notDone = 0;
                current_block_65 = 13853033528615664019;
            } else if iend.offset_from(ip) as core::ffi::c_long as size_t >= neededInSize {
                let decodedSize = ZSTDv06_decompressContinue(
                    (*zbd).zd,
                    ((*zbd).outBuff).add((*zbd).outStart) as *mut core::ffi::c_void,
                    ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart),
                    ip as *const core::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(decodedSize) != 0 {
                    return decodedSize;
                }
                ip = ip.add(neededInSize);
                if decodedSize == 0 {
                    current_block_65 = 13853033528615664019;
                } else {
                    (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize);
                    (*zbd).stage = ZBUFFds_flush;
                    current_block_65 = 13853033528615664019;
                }
            } else if ip == iend {
                notDone = 0;
                current_block_65 = 13853033528615664019;
            } else {
                (*zbd).stage = ZBUFFds_load;
                current_block_65 = 14220266465818359136;
            }
        }
        if current_block_65 == 14220266465818359136 {
            let neededInSize_0 = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
            let toLoad_0 = neededInSize_0.wrapping_sub((*zbd).inPos);
            let mut loadedSize: size_t = 0;
            if toLoad_0 > ((*zbd).inBuffSize).wrapping_sub((*zbd).inPos) {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            loadedSize = ZBUFFv06_limitCopy(
                ((*zbd).inBuff).add((*zbd).inPos) as *mut core::ffi::c_void,
                toLoad_0,
                ip as *const core::ffi::c_void,
                iend.offset_from(ip) as core::ffi::c_long as size_t,
            );
            ip = ip.add(loadedSize);
            (*zbd).inPos = ((*zbd).inPos).wrapping_add(loadedSize);
            if loadedSize < toLoad_0 {
                notDone = 0;
                current_block_65 = 13853033528615664019;
            } else {
                let decodedSize_0 = ZSTDv06_decompressContinue(
                    (*zbd).zd,
                    ((*zbd).outBuff).add((*zbd).outStart) as *mut core::ffi::c_void,
                    ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart),
                    (*zbd).inBuff as *const core::ffi::c_void,
                    neededInSize_0,
                );
                if ERR_isError(decodedSize_0) != 0 {
                    return decodedSize_0;
                }
                (*zbd).inPos = 0;
                if decodedSize_0 == 0 {
                    (*zbd).stage = ZBUFFds_read;
                    current_block_65 = 13853033528615664019;
                } else {
                    (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize_0);
                    (*zbd).stage = ZBUFFds_flush;
                    current_block_65 = 15594603006322722090;
                }
            }
        }
        if current_block_65 == 15594603006322722090 {
            let toFlushSize = ((*zbd).outEnd).wrapping_sub((*zbd).outStart);
            let flushedSize = ZBUFFv06_limitCopy(
                op as *mut core::ffi::c_void,
                oend.offset_from(op) as core::ffi::c_long as size_t,
                ((*zbd).outBuff).add((*zbd).outStart) as *const core::ffi::c_void,
                toFlushSize,
            );
            op = op.add(flushedSize);
            (*zbd).outStart = ((*zbd).outStart).wrapping_add(flushedSize);
            if flushedSize == toFlushSize {
                (*zbd).stage = ZBUFFds_read;
                if ((*zbd).outStart).wrapping_add((*zbd).blockSize) > (*zbd).outBuffSize {
                    (*zbd).outEnd = 0;
                    (*zbd).outStart = (*zbd).outEnd;
                }
            } else {
                notDone = 0;
            }
        }
    }
    *srcSizePtr = ip.offset_from(istart) as core::ffi::c_long as size_t;
    *dstCapacityPtr = op.offset_from(ostart) as core::ffi::c_long as size_t;
    let mut nextSrcSizeHint = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
    if nextSrcSizeHint > ZSTDv06_blockHeaderSize {
        nextSrcSizeHint = nextSrcSizeHint.wrapping_add(ZSTDv06_blockHeaderSize);
    }
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub((*zbd).inPos);
    nextSrcSizeHint
}
pub unsafe fn ZBUFFv06_recommendedDInSize() -> size_t {
    (ZSTDv06_BLOCKSIZE_MAX as size_t).wrapping_add(ZSTDv06_blockHeaderSize)
}
pub unsafe fn ZBUFFv06_recommendedDOutSize() -> size_t {
    ZSTDv06_BLOCKSIZE_MAX as size_t
}
