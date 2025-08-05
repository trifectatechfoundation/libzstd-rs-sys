use std::ptr;

use crate::lib::common::error_private::ERR_getErrorString;
use crate::lib::zstd::*;
use crate::{MEM_readLE16, MEM_readLE32, MEM_readLEST, MEM_writeLE16};

extern "C" {
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memmove(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
}

#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong == 4) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong == 8) as std::ffi::c_int
        as std::ffi::c_uint
}

pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
pub type ZSTDv05_DCtx = ZSTDv05_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv05_DCtx_s {
    pub LLTable: [FSEv05_DTable; 1025],
    pub OffTable: [FSEv05_DTable; 513],
    pub MLTable: [FSEv05_DTable; 1025],
    pub hufTableX4: [std::ffi::c_uint; 4097],
    pub previousDstEnd: *const std::ffi::c_void,
    pub base: *const std::ffi::c_void,
    pub vBase: *const std::ffi::c_void,
    pub dictEnd: *const std::ffi::c_void,
    pub expected: size_t,
    pub headerSize: size_t,
    pub params: ZSTDv05_parameters,
    pub bType: blockType_t,
    pub stage: ZSTDv05_dStage,
    pub flagStaticTables: u32,
    pub litPtr: *const u8,
    pub litSize: size_t,
    pub litBuffer: [u8; 131080],
    pub headerBuffer: [u8; 5],
}
pub type ZSTDv05_dStage = std::ffi::c_uint;
pub const ZSTDv05ds_decompressBlock: ZSTDv05_dStage = 3;
pub const ZSTDv05ds_decodeBlockHeader: ZSTDv05_dStage = 2;
pub const ZSTDv05ds_decodeFrameHeader: ZSTDv05_dStage = 1;
pub const ZSTDv05ds_getFrameHeaderSize: ZSTDv05_dStage = 0;
pub type blockType_t = std::ffi::c_uint;
pub const bt_end: blockType_t = 3;
pub const bt_rle: blockType_t = 2;
pub const bt_raw: blockType_t = 1;
pub const bt_compressed: blockType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv05_parameters {
    pub srcSize: u64,
    pub windowLog: u32,
    pub contentLog: u32,
    pub hashLog: u32,
    pub searchLog: u32,
    pub searchLength: u32,
    pub targetLength: u32,
    pub strategy: ZSTDv05_strategy,
}
pub type ZSTDv05_strategy = std::ffi::c_uint;
pub const ZSTDv05_btopt: ZSTDv05_strategy = 6;
pub const ZSTDv05_opt: ZSTDv05_strategy = 5;
pub const ZSTDv05_btlazy2: ZSTDv05_strategy = 4;
pub const ZSTDv05_lazy2: ZSTDv05_strategy = 3;
pub const ZSTDv05_lazy: ZSTDv05_strategy = 2;
pub const ZSTDv05_greedy: ZSTDv05_strategy = 1;
pub const ZSTDv05_fast: ZSTDv05_strategy = 0;
pub type FSEv05_DTable = std::ffi::c_uint;
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
    pub DStream: BITv05_DStream_t,
    pub stateLL: FSEv05_DState_t,
    pub stateOffb: FSEv05_DState_t,
    pub stateML: FSEv05_DState_t,
    pub prevOffset: size_t,
    pub dumps: *const u8,
    pub dumpsEnd: *const u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv05_DState_t {
    pub state: size_t,
    pub table: *const std::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITv05_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: std::ffi::c_uint,
    pub ptr: *const std::ffi::c_char,
    pub start: *const std::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv05_decode_t {
    pub newState: std::ffi::c_ushort,
    pub symbol: std::ffi::c_uchar,
    pub nbBits: std::ffi::c_uchar,
}
pub type BITv05_DStream_status = std::ffi::c_uint;
pub const BITv05_DStream_overflow: BITv05_DStream_status = 3;
pub const BITv05_DStream_completed: BITv05_DStream_status = 2;
pub const BITv05_DStream_endOfBuffer: BITv05_DStream_status = 1;
pub const BITv05_DStream_unfinished: BITv05_DStream_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv05_DTableHeader {
    pub tableLog: u16,
    pub fastMode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv05_DEltX4 {
    pub sequence: u16,
    pub nbBits: u8,
    pub length: u8,
}
pub type decompressionAlgo = Option<
    unsafe extern "C" fn(*mut std::ffi::c_void, size_t, *const std::ffi::c_void, size_t) -> size_t,
>;
pub type rankVal_t = [[u32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: u8,
    pub weight: u8,
}
pub type DTable_max_t = [std::ffi::c_uint; 4097];
pub type C2RustUnnamed = std::ffi::c_uint;
pub const HUFv05_static_assert: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv05_DEltX2 {
    pub byte: u8,
    pub nbBits: u8,
}
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUFv05_static_assert_0: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: u32,
    pub decode256Time: u32,
}
pub type ERR_enum = ZSTD_ErrorCode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZBUFFv05_DCtx_s {
    pub zc: *mut ZSTDv05_DCtx,
    pub params: ZSTDv05_parameters,
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub hPos: size_t,
    pub stage: ZBUFFv05_dStage,
    pub headerBuffer: [std::ffi::c_uchar; 5],
}
pub type ZBUFFv05_dStage = std::ffi::c_uint;
pub const ZBUFFv05ds_flush: ZBUFFv05_dStage = 6;
pub const ZBUFFv05ds_load: ZBUFFv05_dStage = 5;
pub const ZBUFFv05ds_read: ZBUFFv05_dStage = 4;
pub const ZBUFFv05ds_decodeHeader: ZBUFFv05_dStage = 3;
pub const ZBUFFv05ds_loadHeader: ZBUFFv05_dStage = 2;
pub const ZBUFFv05ds_readHeader: ZBUFFv05_dStage = 1;
pub const ZBUFFv05ds_init: ZBUFFv05_dStage = 0;
pub type ZBUFFv05_DCtx = ZBUFFv05_DCtx_s;
pub const ZSTDv05_MAGICNUMBER: std::ffi::c_uint = 0xfd2fb525 as std::ffi::c_uint;
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
pub const ZSTDv05_WINDOWLOG_ABSOLUTEMIN: std::ffi::c_int = 11;
pub const ZSTDv05_DICT_MAGIC: std::ffi::c_uint = 0xec30a435 as std::ffi::c_uint;
pub const BLOCKSIZE: std::ffi::c_int = 128 * ((1) << 10);
static ZSTDv05_blockHeaderSize: size_t = 3;
static ZSTDv05_frameHeaderSize_min: size_t = 5;
pub const ZSTDv05_frameHeaderSize_max: std::ffi::c_int = 5;
pub const IS_HUFv05: std::ffi::c_int = 0;
pub const IS_PCH: std::ffi::c_int = 1;
pub const IS_RAW: std::ffi::c_int = 2;
pub const IS_RLE: std::ffi::c_int = 3;
pub const MINMATCH: std::ffi::c_int = 4;
pub const REPCODE_STARTVALUE: std::ffi::c_int = 1;
pub const MLbits: std::ffi::c_int = 7;
pub const LLbits: std::ffi::c_int = 6;
pub const Offbits: std::ffi::c_int = 5;
pub const MaxML: std::ffi::c_int = ((1) << MLbits) - 1;
pub const MaxLL: std::ffi::c_int = ((1) << LLbits) - 1;
pub const MaxOff: std::ffi::c_int = ((1) << Offbits) - 1;
pub const MLFSEv05Log: std::ffi::c_int = 10;
pub const LLFSEv05Log: std::ffi::c_int = 10;
pub const OffFSEv05Log: std::ffi::c_int = 9;
pub const FSEv05_ENCODING_RAW: std::ffi::c_int = 0;
pub const FSEv05_ENCODING_RLE: std::ffi::c_int = 1;
pub const FSEv05_ENCODING_STATIC: std::ffi::c_int = 2;
pub const FSEv05_ENCODING_DYNAMIC: std::ffi::c_int = 3;
pub const ZSTD_HUFFDTABLE_CAPACITY_LOG: std::ffi::c_int = 12;
pub const MIN_SEQUENCES_SIZE: std::ffi::c_int = 1;
pub const MIN_CBLOCK_SIZE: std::ffi::c_int = 1 + 1 + MIN_SEQUENCES_SIZE;
pub const WILDCOPY_OVERLENGTH: std::ffi::c_int = 8;
pub const ZSTD_CONTENTSIZE_ERROR: std::ffi::c_ulonglong =
    (0 as std::ffi::c_ulonglong).wrapping_sub(2 as std::ffi::c_int as std::ffi::c_ulonglong);
unsafe extern "C" fn ZSTDv05_copy8(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
) {
    memcpy(dst, src, 8);
}
#[inline]
unsafe extern "C" fn ZSTDv05_wildcopy(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut length: ptrdiff_t,
) {
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    loop {
        ZSTDv05_copy8(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
        op = op.offset(8);
        ip = ip.offset(8);
        if op >= oend {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn BITv05_highbit32(mut val: u32) -> std::ffi::c_uint {
    (val.leading_zeros() as i32 ^ 31) as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn BITv05_initDStream(
    mut bitD: *mut BITv05_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 {
        ptr::write_bytes(
            bitD as *mut u8,
            0,
            ::core::mem::size_of::<BITv05_DStream_t>(),
        );
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if srcSize >= ::core::mem::size_of::<size_t>() as std::ffi::c_ulong {
        let mut contain32: u32 = 0;
        (*bitD).start = srcBuffer as *const std::ffi::c_char;
        (*bitD).ptr = (srcBuffer as *const std::ffi::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        contain32 = *(srcBuffer as *const u8).offset(srcSize.wrapping_sub(1) as isize) as u32;
        if contain32 == 0 {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed =
            (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(BITv05_highbit32(contain32));
    } else {
        let mut contain32_0: u32 = 0;
        (*bitD).start = srcBuffer as *const std::ffi::c_char;
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as size_t;
        let mut current_block_20: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                    (*((*bitD).start as *const u8).offset(6) as size_t)
                        << (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                            .wrapping_mul(8)
                            .wrapping_sub(16),
                );
                current_block_20 = 2201636535984292624;
            }
            6 => {
                current_block_20 = 2201636535984292624;
            }
            5 => {
                current_block_20 = 8593878353892082780;
            }
            4 => {
                current_block_20 = 9473878909555864502;
            }
            3 => {
                current_block_20 = 16790621803724920579;
            }
            2 => {
                current_block_20 = 9166003750766676026;
            }
            _ => {
                current_block_20 = 5948590327928692120;
            }
        }
        if current_block_20 == 2201636535984292624 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*((*bitD).start as *const u8).offset(5) as size_t)
                    << (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                        .wrapping_mul(8)
                        .wrapping_sub(24),
            );
            current_block_20 = 8593878353892082780;
        }
        if current_block_20 == 8593878353892082780 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*((*bitD).start as *const u8).offset(4) as size_t)
                    << (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                        .wrapping_mul(8)
                        .wrapping_sub(32),
            );
            current_block_20 = 9473878909555864502;
        }
        if current_block_20 == 9473878909555864502 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*((*bitD).start as *const u8).offset(3) as size_t) << 24);
            current_block_20 = 16790621803724920579;
        }
        if current_block_20 == 16790621803724920579 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*((*bitD).start as *const u8).offset(2) as size_t) << 16);
            current_block_20 = 9166003750766676026;
        }
        if current_block_20 == 9166003750766676026 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*((*bitD).start as *const u8).offset(1) as size_t) << 8);
        }
        contain32_0 = *(srcBuffer as *const u8).offset(srcSize.wrapping_sub(1) as isize) as u32;
        if contain32_0 == 0 {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed =
            (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(BITv05_highbit32(contain32_0));
        (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(
            (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_sub(srcSize) as u32
                * 8,
        );
    }
    srcSize
}
#[inline]
unsafe extern "C" fn BITv05_lookBits(mut bitD: *mut BITv05_DStream_t, mut nbBits: u32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> 1
        >> (bitMask.wrapping_sub(nbBits) & bitMask)
}
#[inline]
unsafe extern "C" fn BITv05_lookBitsFast(
    mut bitD: *mut BITv05_DStream_t,
    mut nbBits: u32,
) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> (bitMask.wrapping_add(1).wrapping_sub(nbBits) & bitMask)
}
#[inline]
unsafe extern "C" fn BITv05_skipBits(mut bitD: *mut BITv05_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline]
unsafe extern "C" fn BITv05_readBits(
    mut bitD: *mut BITv05_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> size_t {
    let mut value = BITv05_lookBits(bitD, nbBits);
    BITv05_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BITv05_readBitsFast(
    mut bitD: *mut BITv05_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> size_t {
    let mut value = BITv05_lookBitsFast(bitD, nbBits);
    BITv05_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BITv05_reloadDStream(
    mut bitD: *mut BITv05_DStream_t,
) -> BITv05_DStream_status {
    if (*bitD).bitsConsumed as std::ffi::c_ulong
        > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8)
    {
        return BITv05_DStream_overflow;
    }
    if (*bitD).ptr
        >= ((*bitD).start).offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize)
    {
        (*bitD).ptr = ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3) as isize));
        (*bitD).bitsConsumed &= 7;
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        return BITv05_DStream_unfinished;
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as std::ffi::c_ulong)
            < (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8)
        {
            return BITv05_DStream_endOfBuffer;
        }
        return BITv05_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3;
    let mut result = BITv05_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as u32;
        result = BITv05_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    result
}
#[inline]
unsafe extern "C" fn BITv05_endOfDStream(mut DStream: *const BITv05_DStream_t) -> std::ffi::c_uint {
    ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as std::ffi::c_ulong
            == (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8))
        as std::ffi::c_int as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn FSEv05_initDState(
    mut DStatePtr: *mut FSEv05_DState_t,
    mut bitD: *mut BITv05_DStream_t,
    mut dt: *const FSEv05_DTable,
) {
    let mut ptr = dt as *const std::ffi::c_void;
    let DTableH = ptr as *const FSEv05_DTableHeader;
    (*DStatePtr).state = BITv05_readBits(bitD, (*DTableH).tableLog as std::ffi::c_uint);
    BITv05_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1) as *const std::ffi::c_void;
}
#[inline]
unsafe extern "C" fn FSEv05_peakSymbol(mut DStatePtr: *mut FSEv05_DState_t) -> u8 {
    let DInfo = *((*DStatePtr).table as *const FSEv05_decode_t).offset((*DStatePtr).state as isize);
    DInfo.symbol
}
#[inline]
unsafe extern "C" fn FSEv05_decodeSymbol(
    mut DStatePtr: *mut FSEv05_DState_t,
    mut bitD: *mut BITv05_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv05_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let mut symbol = DInfo.symbol;
    let mut lowBits = BITv05_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe extern "C" fn FSEv05_decodeSymbolFast(
    mut DStatePtr: *mut FSEv05_DState_t,
    mut bitD: *mut BITv05_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv05_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let mut symbol = DInfo.symbol;
    let mut lowBits = BITv05_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe extern "C" fn FSEv05_endOfDState(mut DStatePtr: *const FSEv05_DState_t) -> std::ffi::c_uint {
    ((*DStatePtr).state == 0) as std::ffi::c_int as std::ffi::c_uint
}
pub const FSEv05_MAX_MEMORY_USAGE: std::ffi::c_int = 14;
pub const FSEv05_MAX_SYMBOL_VALUE: std::ffi::c_int = 255;
pub const NULL: std::ffi::c_int = 0;
pub const FSEv05_MAX_TABLELOG: std::ffi::c_int = FSEv05_MAX_MEMORY_USAGE - 2;
pub const FSEv05_MIN_TABLELOG: std::ffi::c_int = 5;
pub const FSEv05_TABLELOG_ABSOLUTE_MAX: std::ffi::c_int = 15;
unsafe extern "C" fn FSEv05_tableStep(mut tableSize: u32) -> u32 {
    (tableSize >> 1)
        .wrapping_add(tableSize >> 3)
        .wrapping_add(3)
}
#[export_name = crate::prefix!(FSEv05_createDTable)]
pub unsafe extern "C" fn FSEv05_createDTable(mut tableLog: std::ffi::c_uint) -> *mut FSEv05_DTable {
    if tableLog > FSEv05_TABLELOG_ABSOLUTE_MAX as std::ffi::c_uint {
        tableLog = FSEv05_TABLELOG_ABSOLUTE_MAX as std::ffi::c_uint;
    }
    malloc(
        ((1 + ((1) << tableLog)) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u32>() as std::ffi::c_ulong),
    ) as *mut FSEv05_DTable
}
#[export_name = crate::prefix!(FSEv05_freeDTable)]
pub unsafe extern "C" fn FSEv05_freeDTable(mut dt: *mut FSEv05_DTable) {
    free(dt as *mut std::ffi::c_void);
}
#[export_name = crate::prefix!(FSEv05_buildDTable)]
pub unsafe extern "C" fn FSEv05_buildDTable(
    mut dt: *mut FSEv05_DTable,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
) -> size_t {
    let mut DTableH = FSEv05_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    let tdPtr = dt.offset(1) as *mut std::ffi::c_void;
    let tableDecode = tdPtr as *mut FSEv05_decode_t;
    let tableSize = ((1) << tableLog) as u32;
    let tableMask = tableSize.wrapping_sub(1);
    let step = FSEv05_tableStep(tableSize);
    let mut symbolNext: [u16; 256] = [0; 256];
    let mut position = 0 as std::ffi::c_int as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;
    let mut noLarge = 1;
    let mut s: u32 = 0;
    if maxSymbolValue > FSEv05_MAX_SYMBOL_VALUE as std::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as std::ffi::c_int) as size_t;
    }
    if tableLog > FSEv05_MAX_TABLELOG as std::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    memset(
        tableDecode as *mut std::ffi::c_void,
        0,
        (::core::mem::size_of::<u8>() as std::ffi::c_ulong)
            .wrapping_mul(maxSymbolValue.wrapping_add(1) as std::ffi::c_ulong),
    );
    DTableH.tableLog = tableLog as u16;
    s = 0;
    while s <= maxSymbolValue {
        if *normalizedCounter.offset(s as isize) as std::ffi::c_int == -(1) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).symbol = s as u8;
            *symbolNext.as_mut_ptr().offset(s as isize) = 1;
        } else {
            if *normalizedCounter.offset(s as isize) as std::ffi::c_int
                >= largeLimit as std::ffi::c_int
            {
                noLarge = 0;
            }
            *symbolNext.as_mut_ptr().offset(s as isize) =
                *normalizedCounter.offset(s as isize) as u16;
        }
        s = s.wrapping_add(1);
    }
    s = 0;
    while s <= maxSymbolValue {
        let mut i: std::ffi::c_int = 0;
        i = 0;
        while i < *normalizedCounter.offset(s as isize) as std::ffi::c_int {
            (*tableDecode.offset(position as isize)).symbol = s as u8;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
        }
        s = s.wrapping_add(1);
    }
    if position != 0 {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    let mut i_0: u32 = 0;
    i_0 = 0;
    while i_0 < tableSize {
        let mut symbol = (*tableDecode.offset(i_0 as isize)).symbol;
        let fresh1 = &mut (*symbolNext.as_mut_ptr().offset(symbol as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let mut nextState = fresh2;
        (*tableDecode.offset(i_0 as isize)).nbBits =
            tableLog.wrapping_sub(BITv05_highbit32(nextState as u32)) as u8;
        (*tableDecode.offset(i_0 as isize)).newState = (((nextState as std::ffi::c_int)
            << (*tableDecode.offset(i_0 as isize)).nbBits as std::ffi::c_int)
            as u32)
            .wrapping_sub(tableSize) as u16;
        i_0 = i_0.wrapping_add(1);
    }
    DTableH.fastMode = noLarge as u16;
    memcpy(
        dt as *mut std::ffi::c_void,
        &mut DTableH as *mut FSEv05_DTableHeader as *const std::ffi::c_void,
        ::core::mem::size_of::<FSEv05_DTableHeader>() as std::ffi::c_ulong,
    );
    0
}
#[export_name = crate::prefix!(FSEv05_isError)]
pub unsafe extern "C" fn FSEv05_isError(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}
#[export_name = crate::prefix!(FSEv05_getErrorName)]
pub unsafe extern "C" fn FSEv05_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(code)
}
unsafe extern "C" fn FSEv05_abs(mut a: std::ffi::c_short) -> std::ffi::c_short {
    (if (a as std::ffi::c_int) < 0 {
        -(a as std::ffi::c_int)
    } else {
        a as std::ffi::c_int
    }) as std::ffi::c_short
}
#[export_name = crate::prefix!(FSEv05_readNCount)]
pub unsafe extern "C" fn FSEv05_readNCount(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut maxSVPtr: *mut std::ffi::c_uint,
    mut tableLogPtr: *mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart = headerBuffer as *const u8;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: std::ffi::c_int = 0;
    let mut remaining: std::ffi::c_int = 0;
    let mut threshold: std::ffi::c_int = 0;
    let mut bitStream: u32 = 0;
    let mut bitCount: std::ffi::c_int = 0;
    let mut charnum = 0;
    let mut previous0 = 0;
    if hbSize < 4 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    bitStream = MEM_readLE32(ip as *const std::ffi::c_void);
    nbBits = (bitStream & 0xf as std::ffi::c_int as u32).wrapping_add(FSEv05_MIN_TABLELOG as u32)
        as std::ffi::c_int;
    if nbBits > FSEv05_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    bitStream >>= 4;
    bitCount = 4;
    *tableLogPtr = nbBits as std::ffi::c_uint;
    remaining = ((1) << nbBits) + 1;
    threshold = (1) << nbBits;
    nbBits += 1;
    while remaining > 1 && charnum <= *maxSVPtr {
        if previous0 != 0 {
            let mut n0 = charnum;
            while bitStream & 0xffff as std::ffi::c_int as u32 == 0xffff as std::ffi::c_int as u32 {
                n0 = n0.wrapping_add(24);
                if ip < iend.offset(-(5)) {
                    ip = ip.offset(2);
                    bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
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
                return -(ZSTD_error_maxSymbolValue_tooSmall as std::ffi::c_int) as size_t;
            }
            while charnum < n0 {
                let fresh3 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter.offset(fresh3 as isize) = 0;
            }
            if ip <= iend.offset(-(7)) || ip.offset((bitCount >> 3) as isize) <= iend.offset(-(4)) {
                ip = ip.offset((bitCount >> 3) as isize);
                bitCount &= 7;
                bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
            } else {
                bitStream >>= 2;
            }
        }
        let max = (2 * threshold - 1 - remaining) as std::ffi::c_short;
        let mut count: std::ffi::c_short = 0;
        if (bitStream & (threshold - 1) as u32) < max as u32 {
            count = (bitStream & (threshold - 1) as u32) as std::ffi::c_short;
            bitCount += nbBits - 1;
        } else {
            count = (bitStream & (2 * threshold - 1) as u32) as std::ffi::c_short;
            if count as std::ffi::c_int >= threshold {
                count = (count as std::ffi::c_int - max as std::ffi::c_int) as std::ffi::c_short;
            }
            bitCount += nbBits;
        }
        count -= 1;
        remaining -= FSEv05_abs(count) as std::ffi::c_int;
        let fresh4 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh4 as isize) = count;
        previous0 = (count == 0) as std::ffi::c_int;
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1;
        }
        if ip <= iend.offset(-(7)) || ip.offset((bitCount >> 3) as isize) <= iend.offset(-(4)) {
            ip = ip.offset((bitCount >> 3) as isize);
            bitCount &= 7;
        } else {
            bitCount -=
                (8 * iend.offset(-(4)).offset_from(ip) as std::ffi::c_long) as std::ffi::c_int;
            ip = iend.offset(-(4));
        }
        bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> (bitCount & 31);
    }
    if remaining != 1 {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1);
    ip = ip.offset(((bitCount + 7) >> 3) as isize);
    if ip.offset_from(istart) as std::ffi::c_long as size_t > hbSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip.offset_from(istart) as std::ffi::c_long as size_t
}
#[export_name = crate::prefix!(FSEv05_buildDTable_rle)]
pub unsafe extern "C" fn FSEv05_buildDTable_rle(
    mut dt: *mut FSEv05_DTable,
    mut symbolValue: u8,
) -> size_t {
    let mut ptr = dt as *mut std::ffi::c_void;
    let DTableH = ptr as *mut FSEv05_DTableHeader;
    let mut dPtr = dt.offset(1) as *mut std::ffi::c_void;
    let cell = dPtr as *mut FSEv05_decode_t;
    (*DTableH).tableLog = 0;
    (*DTableH).fastMode = 0;
    (*cell).newState = 0;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0;
    0
}
#[export_name = crate::prefix!(FSEv05_buildDTable_raw)]
pub unsafe extern "C" fn FSEv05_buildDTable_raw(
    mut dt: *mut FSEv05_DTable,
    mut nbBits: std::ffi::c_uint,
) -> size_t {
    let mut ptr = dt as *mut std::ffi::c_void;
    let DTableH = ptr as *mut FSEv05_DTableHeader;
    let mut dPtr = dt.offset(1) as *mut std::ffi::c_void;
    let dinfo = dPtr as *mut FSEv05_decode_t;
    let tableSize = ((1) << nbBits) as std::ffi::c_uint;
    let tableMask = tableSize.wrapping_sub(1);
    let maxSymbolValue = tableMask;
    let mut s: std::ffi::c_uint = 0;
    if nbBits < 1 {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    (*DTableH).tableLog = nbBits as u16;
    (*DTableH).fastMode = 1;
    s = 0;
    while s <= maxSymbolValue {
        (*dinfo.offset(s as isize)).newState = 0;
        (*dinfo.offset(s as isize)).symbol = s as u8;
        (*dinfo.offset(s as isize)).nbBits = nbBits as u8;
        s = s.wrapping_add(1);
    }
    0
}
#[inline(always)]
unsafe extern "C" fn FSEv05_decompress_usingDTable_generic(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSEv05_DTable,
    fast: std::ffi::c_uint,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-(3));
    let mut bitD = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut state1 = FSEv05_DState_t {
        state: 0,
        table: std::ptr::null::<std::ffi::c_void>(),
    };
    let mut state2 = FSEv05_DState_t {
        state: 0,
        table: std::ptr::null::<std::ffi::c_void>(),
    };
    let mut errorCode: size_t = 0;
    errorCode = BITv05_initDStream(&mut bitD, cSrc, cSrcSize);
    if FSEv05_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv05_initDState(&mut state1, &mut bitD, dt);
    FSEv05_initDState(&mut state2, &mut bitD, dt);
    while BITv05_reloadDStream(&mut bitD) as std::ffi::c_uint
        == BITv05_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && op < olimit
    {
        *op.offset(0) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 2 + 7) as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8)
        {
            BITv05_reloadDStream(&mut bitD);
        }
        *op.offset(1) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 4 + 7) as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8)
            && BITv05_reloadDStream(&mut bitD) as std::ffi::c_uint
                > BITv05_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        {
            op = op.offset(2);
            break;
        }
        *op.offset(2) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 2 + 7) as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8)
        {
            BITv05_reloadDStream(&mut bitD);
        }
        *op.offset(3) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as u8;
        op = op.offset(4);
    }
    while !(BITv05_reloadDStream(&mut bitD) as std::ffi::c_uint
        > BITv05_DStream_completed as std::ffi::c_int as std::ffi::c_uint
        || op == omax
        || BITv05_endOfDStream(&mut bitD) != 0
            && (fast != 0 || FSEv05_endOfDState(&mut state1) != 0))
    {
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as u8;
        if BITv05_reloadDStream(&mut bitD) as std::ffi::c_uint
            > BITv05_DStream_completed as std::ffi::c_int as std::ffi::c_uint
            || op == omax
            || BITv05_endOfDStream(&mut bitD) != 0
                && (fast != 0 || FSEv05_endOfDState(&mut state2) != 0)
        {
            break;
        }
        let fresh6 = op;
        op = op.offset(1);
        *fresh6 = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as u8;
    }
    if BITv05_endOfDStream(&mut bitD) != 0
        && FSEv05_endOfDState(&mut state1) != 0
        && FSEv05_endOfDState(&mut state2) != 0
    {
        return op.offset_from(ostart) as std::ffi::c_long as size_t;
    }
    if op == omax {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t
}
#[export_name = crate::prefix!(FSEv05_decompress_usingDTable)]
pub unsafe extern "C" fn FSEv05_decompress_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut originalSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSEv05_DTable,
) -> size_t {
    let mut ptr = dt as *const std::ffi::c_void;
    let mut DTableH = ptr as *const FSEv05_DTableHeader;
    let fastMode = (*DTableH).fastMode as u32;
    if fastMode != 0 {
        return FSEv05_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 1);
    }
    FSEv05_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 0)
}
#[export_name = crate::prefix!(FSEv05_decompress)]
pub unsafe extern "C" fn FSEv05_decompress(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let istart = cSrc as *const u8;
    let mut ip = istart;
    let mut counting: [std::ffi::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: std::ffi::c_uint = 0;
    let mut maxSymbolValue = FSEv05_MAX_SYMBOL_VALUE as std::ffi::c_uint;
    let mut errorCode: size_t = 0;
    if cSrcSize < 2 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    errorCode = FSEv05_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const std::ffi::c_void,
        cSrcSize,
    );
    if FSEv05_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    errorCode = FSEv05_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if FSEv05_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv05_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    )
}
pub const HUFv05_ABSOLUTEMAX_TABLELOG: std::ffi::c_int = 16;
pub const HUFv05_MAX_TABLELOG: std::ffi::c_int = 12;
pub const HUFv05_MAX_SYMBOL_VALUE: std::ffi::c_int = 255;
#[export_name = crate::prefix!(HUFv05_isError)]
pub unsafe extern "C" fn HUFv05_isError(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}
#[export_name = crate::prefix!(HUFv05_getErrorName)]
pub unsafe extern "C" fn HUFv05_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(code)
}
unsafe extern "C" fn HUFv05_readStats(
    mut huffWeight: *mut u8,
    mut hwSize: size_t,
    mut rankStats: *mut u32,
    mut nbSymbolsPtr: *mut u32,
    mut tableLogPtr: *mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightTotal: u32 = 0;
    let mut tableLog: u32 = 0;
    let mut ip = src as *const u8;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    let mut n: u32 = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    iSize = *ip.offset(0) as size_t;
    if iSize >= 128 {
        if iSize >= 242 {
            static mut l: [std::ffi::c_int; 14] =
                [1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128];
            oSize = *l.as_mut_ptr().offset(iSize.wrapping_sub(242) as isize) as size_t;
            memset(huffWeight as *mut std::ffi::c_void, 1, hwSize);
            iSize = 0;
        } else {
            oSize = iSize.wrapping_sub(127);
            iSize = oSize.wrapping_add(1) / 2;
            if iSize.wrapping_add(1) > srcSize {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            if oSize >= hwSize {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            ip = ip.offset(1);
            n = 0;
            while (n as size_t) < oSize {
                *huffWeight.offset(n as isize) =
                    (*ip.offset((n / 2) as isize) as std::ffi::c_int >> 4) as u8;
                *huffWeight.offset(n.wrapping_add(1) as isize) =
                    (*ip.offset((n / 2) as isize) as std::ffi::c_int & 15) as u8;
                n = n.wrapping_add(2);
            }
        }
    } else {
        if iSize.wrapping_add(1) > srcSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        oSize = FSEv05_decompress(
            huffWeight as *mut std::ffi::c_void,
            hwSize.wrapping_sub(1),
            ip.offset(1) as *const std::ffi::c_void,
            iSize,
        );
        if FSEv05_isError(oSize) != 0 {
            return oSize;
        }
    }
    memset(
        rankStats as *mut std::ffi::c_void,
        0,
        ((HUFv05_ABSOLUTEMAX_TABLELOG + 1) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<u32>() as std::ffi::c_ulong),
    );
    weightTotal = 0;
    n = 0;
    while (n as size_t) < oSize {
        if *huffWeight.offset(n as isize) as std::ffi::c_int >= HUFv05_ABSOLUTEMAX_TABLELOG {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        let fresh7 = &mut (*rankStats.offset(*huffWeight.offset(n as isize) as isize));
        *fresh7 = (*fresh7).wrapping_add(1);
        weightTotal = weightTotal
            .wrapping_add(((1) << *huffWeight.offset(n as isize) as std::ffi::c_int >> 1) as u32);
        n = n.wrapping_add(1);
    }
    if weightTotal == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    tableLog = (BITv05_highbit32(weightTotal)).wrapping_add(1);
    if tableLog > HUFv05_ABSOLUTEMAX_TABLELOG as u32 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let mut total = ((1) << tableLog) as u32;
    let mut rest = total.wrapping_sub(weightTotal);
    let mut verif = ((1) << BITv05_highbit32(rest)) as u32;
    let mut lastWeight = (BITv05_highbit32(rest)).wrapping_add(1);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as u8;
    let fresh8 = &mut (*rankStats.offset(lastWeight as isize));
    *fresh8 = (*fresh8).wrapping_add(1);
    if *rankStats.offset(1) < 2 || *rankStats.offset(1) & 1 != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    *tableLogPtr = tableLog;
    iSize.wrapping_add(1)
}
#[export_name = crate::prefix!(HUFv05_readDTableX2)]
pub unsafe extern "C" fn HUFv05_readDTableX2(
    mut DTable: *mut u16,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 17] = [0; 17];
    let mut tableLog = 0;
    let mut iSize: size_t = 0;
    let mut nbSymbols = 0;
    let mut n: u32 = 0;
    let mut nextRankStart: u32 = 0;
    let dtPtr = DTable.offset(1) as *mut std::ffi::c_void;
    let dt = dtPtr as *mut HUFv05_DEltX2;
    iSize = HUFv05_readStats(
        huffWeight.as_mut_ptr(),
        (HUFv05_MAX_SYMBOL_VALUE + 1) as size_t,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv05_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > *DTable.offset(0) as u32 {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    *DTable.offset(0) = tableLog as u16;
    nextRankStart = 0;
    n = 1;
    while n <= tableLog {
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
        let mut D = HUFv05_DEltX2 { byte: 0, nbBits: 0 };
        D.byte = n as u8;
        D.nbBits = tableLog.wrapping_add(1).wrapping_sub(w) as u8;
        i = *rankVal.as_mut_ptr().offset(w as isize);
        while i < (*rankVal.as_mut_ptr().offset(w as isize)).wrapping_add(length) {
            *dt.offset(i as isize) = D;
            i = i.wrapping_add(1);
        }
        let fresh9 = &mut (*rankVal.as_mut_ptr().offset(w as isize));
        *fresh9 = (*fresh9).wrapping_add(length);
        n = n.wrapping_add(1);
    }
    iSize
}
unsafe extern "C" fn HUFv05_decodeSymbolX2(
    mut Dstream: *mut BITv05_DStream_t,
    mut dt: *const HUFv05_DEltX2,
    dtLog: u32,
) -> u8 {
    let val = BITv05_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    BITv05_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as u32);
    c
}
#[inline]
unsafe extern "C" fn HUFv05_decodeStreamX2(
    mut p: *mut u8,
    bitDPtr: *mut BITv05_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv05_DEltX2,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv05_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv05_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p <= pEnd.offset(-(4))
    {
        if MEM_64bits() != 0 {
            let fresh10 = p;
            p = p.offset(1);
            *fresh10 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while BITv05_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv05_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p < pEnd
    {
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    pEnd.offset_from(pStart) as std::ffi::c_long as size_t
}
#[export_name = crate::prefix!(HUFv05_decompress1X2_usingDTable)]
pub unsafe extern "C" fn HUFv05_decompress1X2_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const u16,
) -> size_t {
    let mut op = dst as *mut u8;
    let oend = op.offset(dstSize as isize);
    let dtLog = *DTable.offset(0) as u32;
    let mut dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX2).offset(1);
    let mut bitD = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    if dstSize <= cSrcSize {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    let errorCode = BITv05_initDStream(&mut bitD, cSrc, cSrcSize);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv05_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if BITv05_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[export_name = crate::prefix!(HUFv05_decompress1X2)]
pub unsafe extern "C" fn HUFv05_decompress1X2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_ushort; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let mut errorCode: size_t = 0;
    errorCode = HUFv05_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv05_decompress1X2_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
#[export_name = crate::prefix!(HUFv05_decompress4X2_usingDTable)]
pub unsafe extern "C" fn HUFv05_decompress4X2_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const u16,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX2).offset(1);
    let dtLog = *DTable.offset(0) as u32;
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD2 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD3 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD4 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const std::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const std::ffi::c_void) as size_t;
    let mut length4: size_t = 0;
    let istart1 = istart.offset(6);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
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
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    errorCode = BITv05_initDStream(&mut bitD1, istart1 as *const std::ffi::c_void, length1);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD2, istart2 as *const std::ffi::c_void, length2);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD3, istart3 as *const std::ffi::c_void, length3);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD4, istart4 as *const std::ffi::c_void, length4);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BITv05_reloadDStream(&mut bitD1) as std::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD2) as std::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD3) as std::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    while endSignal == BITv05_DStream_unfinished as std::ffi::c_int as u32
        && op4 < oend.offset(-(7))
    {
        if MEM_64bits() != 0 {
            let fresh16 = op1;
            op1 = op1.offset(1);
            *fresh16 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh17 = op2;
            op2 = op2.offset(1);
            *fresh17 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh18 = op3;
            op3 = op3.offset(1);
            *fresh18 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh19 = op4;
            op4 = op4.offset(1);
            *fresh19 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            let fresh20 = op1;
            op1 = op1.offset(1);
            *fresh20 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            let fresh21 = op2;
            op2 = op2.offset(1);
            *fresh21 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            let fresh22 = op3;
            op3 = op3.offset(1);
            *fresh22 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            let fresh23 = op4;
            op4 = op4.offset(1);
            *fresh23 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh24 = op1;
            op1 = op1.offset(1);
            *fresh24 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh25 = op2;
            op2 = op2.offset(1);
            *fresh25 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh26 = op3;
            op3 = op3.offset(1);
            *fresh26 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh27 = op4;
            op4 = op4.offset(1);
            *fresh27 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh28 = op1;
        op1 = op1.offset(1);
        *fresh28 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh29 = op2;
        op2 = op2.offset(1);
        *fresh29 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh30 = op3;
        op3 = op3.offset(1);
        *fresh30 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh31 = op4;
        op4 = op4.offset(1);
        *fresh31 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        endSignal = BITv05_reloadDStream(&mut bitD1) as std::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD2) as std::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD3) as std::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    HUFv05_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv05_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv05_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv05_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv05_endOfDStream(&mut bitD1)
        & BITv05_endOfDStream(&mut bitD2)
        & BITv05_endOfDStream(&mut bitD3)
        & BITv05_endOfDStream(&mut bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[export_name = crate::prefix!(HUFv05_decompress4X2)]
pub unsafe extern "C" fn HUFv05_decompress4X2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_ushort; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let mut errorCode: size_t = 0;
    errorCode = HUFv05_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv05_decompress4X2_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
unsafe extern "C" fn HUFv05_fillDTableX4Level2(
    mut DTable: *mut HUFv05_DEltX4,
    mut sizeLog: u32,
    consumed: u32,
    mut rankValOrigin: *const u32,
    minWeight: std::ffi::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    sortedListSize: u32,
    mut nbBitsBaseline: u32,
    mut baseSeq: u16,
) {
    let mut DElt = HUFv05_DEltX4 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let mut rankVal: [u32; 17] = [0; 17];
    let mut s: u32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut std::ffi::c_void,
        rankValOrigin as *const std::ffi::c_void,
        ::core::mem::size_of::<[u32; 17]>() as std::ffi::c_ulong,
    );
    if minWeight > 1 {
        let mut i: u32 = 0;
        let mut skipSize = *rankVal.as_mut_ptr().offset(minWeight as isize);
        MEM_writeLE16(
            &mut DElt.sequence as *mut u16 as *mut std::ffi::c_void,
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
            &mut DElt.sequence as *mut u16 as *mut std::ffi::c_void,
            (baseSeq as u32).wrapping_add(symbol << 8) as u16,
        );
        DElt.nbBits = nbBits.wrapping_add(consumed) as u8;
        DElt.length = 2;
        loop {
            let fresh32 = i_0;
            i_0 = i_0.wrapping_add(1);
            *DTable.offset(fresh32 as isize) = DElt;
            if i_0 >= end {
                break;
            }
        }
        let fresh33 = &mut (*rankVal.as_mut_ptr().offset(weight as isize));
        *fresh33 = (*fresh33).wrapping_add(length);
        s = s.wrapping_add(1);
    }
}
unsafe extern "C" fn HUFv05_fillDTableX4(
    mut DTable: *mut HUFv05_DEltX4,
    targetLog: u32,
    mut sortedList: *const sortedSymbol_t,
    sortedListSize: u32,
    mut rankStart: *const u32,
    mut rankValOrigin: *mut [u32; 17],
    maxWeight: u32,
    nbBitsBaseline: u32,
) {
    let mut rankVal: [u32; 17] = [0; 17];
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as std::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s: u32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut std::ffi::c_void,
        rankValOrigin as *const std::ffi::c_void,
        ::core::mem::size_of::<[u32; 17]>() as std::ffi::c_ulong,
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
            let mut minWeight = nbBits.wrapping_add(scaleLog as u32) as std::ffi::c_int;
            if minWeight < 1 {
                minWeight = 1;
            }
            sortedRank = *rankStart.offset(minWeight as isize);
            HUFv05_fillDTableX4Level2(
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
            let mut i: u32 = 0;
            let end = start.wrapping_add(length);
            let mut DElt = HUFv05_DEltX4 {
                sequence: 0,
                nbBits: 0,
                length: 0,
            };
            MEM_writeLE16(
                &mut DElt.sequence as *mut u16 as *mut std::ffi::c_void,
                symbol,
            );
            DElt.nbBits = nbBits as u8;
            DElt.length = 1;
            i = start;
            while i < end {
                *DTable.offset(i as isize) = DElt;
                i = i.wrapping_add(1);
            }
        }
        let fresh34 = &mut (*rankVal.as_mut_ptr().offset(weight as isize));
        *fresh34 = (*fresh34).wrapping_add(length);
        s = s.wrapping_add(1);
    }
}
#[export_name = crate::prefix!(HUFv05_readDTableX4)]
pub unsafe extern "C" fn HUFv05_readDTableX4(
    mut DTable: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
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
    let mut dtPtr = DTable as *mut std::ffi::c_void;
    let dt = (dtPtr as *mut HUFv05_DEltX4).offset(1);
    if memLog > HUFv05_ABSOLUTEMAX_TABLELOG as u32 {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    iSize = HUFv05_readStats(
        weightList.as_mut_ptr(),
        (HUFv05_MAX_SYMBOL_VALUE + 1) as size_t,
        rankStats.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv05_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > memLog {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    maxW = tableLog;
    while *rankStats.as_mut_ptr().offset(maxW as isize) == 0 {
        maxW = maxW.wrapping_sub(1);
    }
    let mut w: u32 = 0;
    let mut nextRankStart = 0 as std::ffi::c_int as u32;
    w = 1;
    while w <= maxW {
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
        let mut w_0 = *weightList.as_mut_ptr().offset(s as isize) as u32;
        let fresh35 = &mut (*rankStart.offset(w_0 as isize));
        let fresh36 = *fresh35;
        *fresh35 = (*fresh35).wrapping_add(1);
        let mut r = fresh36;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).symbol = s as u8;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).weight = w_0 as u8;
        s = s.wrapping_add(1);
    }
    *rankStart.offset(0) = 0;
    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut nextRankVal = 0 as std::ffi::c_int as u32;
    let mut w_1: u32 = 0;
    let mut consumed: u32 = 0;
    let rescale = memLog.wrapping_sub(tableLog).wrapping_sub(1) as std::ffi::c_int;
    let mut rankVal0 = (*rankVal.as_mut_ptr().offset(0)).as_mut_ptr();
    w_1 = 1;
    while w_1 <= maxW {
        let mut current_0 = nextRankVal;
        nextRankVal = nextRankVal.wrapping_add(
            *rankStats.as_mut_ptr().offset(w_1 as isize) << w_1.wrapping_add(rescale as u32),
        );
        *rankVal0.offset(w_1 as isize) = current_0;
        w_1 = w_1.wrapping_add(1);
    }
    consumed = minBits;
    while consumed <= memLog.wrapping_sub(minBits) {
        let mut rankValPtr = (*rankVal.as_mut_ptr().offset(consumed as isize)).as_mut_ptr();
        w_1 = 1;
        while w_1 <= maxW {
            *rankValPtr.offset(w_1 as isize) = *rankVal0.offset(w_1 as isize) >> consumed;
            w_1 = w_1.wrapping_add(1);
        }
        consumed = consumed.wrapping_add(1);
    }
    HUFv05_fillDTableX4(
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
unsafe extern "C" fn HUFv05_decodeSymbolX4(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BITv05_DStream_t,
    mut dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv05_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.offset(val as isize) as *const std::ffi::c_void, 2);
    BITv05_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    (*dt.offset(val as isize)).length as u32
}
unsafe extern "C" fn HUFv05_decodeLastSymbolX4(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BITv05_DStream_t,
    mut dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv05_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.offset(val as isize) as *const std::ffi::c_void, 1);
    if (*dt.offset(val as isize)).length as std::ffi::c_int == 1 {
        BITv05_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    } else if ((*DStream).bitsConsumed as std::ffi::c_ulong)
        < (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8)
    {
        BITv05_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
        if (*DStream).bitsConsumed as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_mul(8)
        {
            (*DStream).bitsConsumed = (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8) as std::ffi::c_uint;
        }
    }
    1
}
#[inline]
unsafe extern "C" fn HUFv05_decodeStreamX4(
    mut p: *mut u8,
    mut bitDPtr: *mut BITv05_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv05_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv05_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p < pEnd.offset(-(7))
    {
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv05_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            p = p.offset(
                HUFv05_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv05_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        p = p
            .offset(HUFv05_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize);
    }
    while BITv05_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv05_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p <= pEnd.offset(-(2))
    {
        p = p
            .offset(HUFv05_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize);
    }
    while p <= pEnd.offset(-(2)) {
        p = p
            .offset(HUFv05_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize);
    }
    if p < pEnd {
        p = p.offset(
            HUFv05_decodeLastSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as std::ffi::c_long as size_t
}
#[export_name = crate::prefix!(HUFv05_decompress1X4_usingDTable)]
pub unsafe extern "C" fn HUFv05_decompress1X4_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const std::ffi::c_uint,
) -> size_t {
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let dtLog = *DTable.offset(0);
    let dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX4).offset(1);
    let mut errorCode: size_t = 0;
    let mut bitD = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    errorCode = BITv05_initDStream(&mut bitD, istart as *const std::ffi::c_void, cSrcSize);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv05_decodeStreamX4(ostart, &mut bitD, oend, dt, dtLog);
    if BITv05_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[export_name = crate::prefix!(HUFv05_decompress1X4)]
pub unsafe extern "C" fn HUFv05_decompress1X4(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_uint; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let mut hSize = HUFv05_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv05_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv05_decompress1X4_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
#[export_name = crate::prefix!(HUFv05_decompress4X4_usingDTable)]
pub unsafe extern "C" fn HUFv05_decompress4X4_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const std::ffi::c_uint,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX4).offset(1);
    let dtLog = *DTable.offset(0);
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD2 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD3 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD4 = BITv05_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const std::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const std::ffi::c_void) as size_t;
    let mut length4: size_t = 0;
    let istart1 = istart.offset(6);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
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
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    errorCode = BITv05_initDStream(&mut bitD1, istart1 as *const std::ffi::c_void, length1);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD2, istart2 as *const std::ffi::c_void, length2);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD3, istart3 as *const std::ffi::c_void, length3);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD4, istart4 as *const std::ffi::c_void, length4);
    if HUFv05_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BITv05_reloadDStream(&mut bitD1) as std::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD2) as std::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD3) as std::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    while endSignal == BITv05_DStream_unfinished as std::ffi::c_int as u32
        && op4 < oend.offset(-(7))
    {
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv05_decodeSymbolX4(
                op1 as *mut std::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv05_decodeSymbolX4(
                op2 as *mut std::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv05_decodeSymbolX4(
                op3 as *mut std::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv05_decodeSymbolX4(
                op4 as *mut std::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op1 = op1.offset(HUFv05_decodeSymbolX4(
                op1 as *mut std::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op2 = op2.offset(HUFv05_decodeSymbolX4(
                op2 as *mut std::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op3 = op3.offset(HUFv05_decodeSymbolX4(
                op3 as *mut std::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op4 = op4.offset(HUFv05_decodeSymbolX4(
                op4 as *mut std::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv05_decodeSymbolX4(
                op1 as *mut std::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv05_decodeSymbolX4(
                op2 as *mut std::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv05_decodeSymbolX4(
                op3 as *mut std::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv05_decodeSymbolX4(
                op4 as *mut std::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        op1 = op1.offset(
            HUFv05_decodeSymbolX4(op1 as *mut std::ffi::c_void, &mut bitD1, dt, dtLog) as isize,
        );
        op2 = op2.offset(
            HUFv05_decodeSymbolX4(op2 as *mut std::ffi::c_void, &mut bitD2, dt, dtLog) as isize,
        );
        op3 = op3.offset(
            HUFv05_decodeSymbolX4(op3 as *mut std::ffi::c_void, &mut bitD3, dt, dtLog) as isize,
        );
        op4 = op4.offset(
            HUFv05_decodeSymbolX4(op4 as *mut std::ffi::c_void, &mut bitD4, dt, dtLog) as isize,
        );
        endSignal = BITv05_reloadDStream(&mut bitD1) as std::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD2) as std::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD3) as std::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    HUFv05_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv05_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv05_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv05_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv05_endOfDStream(&mut bitD1)
        & BITv05_endOfDStream(&mut bitD2)
        & BITv05_endOfDStream(&mut bitD3)
        & BITv05_endOfDStream(&mut bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[export_name = crate::prefix!(HUFv05_decompress4X4)]
pub unsafe extern "C" fn HUFv05_decompress4X4(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_uint; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let mut hSize = HUFv05_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv05_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv05_decompress4X4_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
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
#[export_name = crate::prefix!(HUFv05_decompress)]
pub unsafe extern "C" fn HUFv05_decompress(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    static decompress: [decompressionAlgo; 3] = unsafe {
        [
            Some(
                HUFv05_decompress4X2
                    as unsafe extern "C" fn(
                        *mut std::ffi::c_void,
                        size_t,
                        *const std::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(
                HUFv05_decompress4X4
                    as unsafe extern "C" fn(
                        *mut std::ffi::c_void,
                        size_t,
                        *const std::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            ::core::mem::transmute::<libc::intptr_t, decompressionAlgo>(NULL as libc::intptr_t),
        ]
    };
    let mut Q: u32 = 0;
    let D256 = (dstSize >> 8) as u32;
    let mut Dtime: [u32; 3] = [0; 3];
    let mut algoNb = 0;
    let mut n: std::ffi::c_int = 0;
    if dstSize == 0 {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if cSrcSize >= dstSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if cSrcSize == 1 {
        memset(dst, *(cSrc as *const u8) as std::ffi::c_int, dstSize);
        return dstSize;
    }
    Q = (cSrcSize * 16 / dstSize) as u32;
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
        n += 1;
    }
    let fresh37 = &mut (*Dtime.as_mut_ptr().offset(1));
    *fresh37 = (*fresh37).wrapping_add(*Dtime.as_mut_ptr().offset(1) >> 4);
    let fresh38 = &mut (*Dtime.as_mut_ptr().offset(2));
    *fresh38 = (*fresh38).wrapping_add(*Dtime.as_mut_ptr().offset(2) >> 3);
    if *Dtime.as_mut_ptr().offset(1) < *Dtime.as_mut_ptr().offset(0) {
        algoNb = 1;
    }
    (*decompress.as_ptr().offset(algoNb as isize)).unwrap_unchecked()(dst, dstSize, cSrc, cSrcSize)
}
unsafe extern "C" fn ZSTDv05_copy4(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
) {
    memcpy(dst, src, 4);
}
#[export_name = crate::prefix!(ZSTDv05_isError)]
pub unsafe extern "C" fn ZSTDv05_isError(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}
#[export_name = crate::prefix!(ZSTDv05_getErrorName)]
pub unsafe extern "C" fn ZSTDv05_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(code)
}
#[export_name = crate::prefix!(ZSTDv05_sizeofDCtx)]
pub unsafe extern "C" fn ZSTDv05_sizeofDCtx() -> size_t {
    ::core::mem::size_of::<ZSTDv05_DCtx>() as std::ffi::c_ulong
}
#[export_name = crate::prefix!(ZSTDv05_decompressBegin)]
pub unsafe extern "C" fn ZSTDv05_decompressBegin(mut dctx: *mut ZSTDv05_DCtx) -> size_t {
    (*dctx).expected = ZSTDv05_frameHeaderSize_min;
    (*dctx).stage = ZSTDv05ds_getFrameHeaderSize;
    (*dctx).previousDstEnd = NULL as *const std::ffi::c_void;
    (*dctx).base = NULL as *const std::ffi::c_void;
    (*dctx).vBase = NULL as *const std::ffi::c_void;
    (*dctx).dictEnd = NULL as *const std::ffi::c_void;
    *((*dctx).hufTableX4).as_mut_ptr().offset(0) = ZSTD_HUFFDTABLE_CAPACITY_LOG as std::ffi::c_uint;
    (*dctx).flagStaticTables = 0;
    0
}
#[export_name = crate::prefix!(ZSTDv05_createDCtx)]
pub unsafe extern "C" fn ZSTDv05_createDCtx() -> *mut ZSTDv05_DCtx {
    let mut dctx =
        malloc(::core::mem::size_of::<ZSTDv05_DCtx>() as std::ffi::c_ulong) as *mut ZSTDv05_DCtx;
    if dctx.is_null() {
        return NULL as *mut ZSTDv05_DCtx;
    }
    ZSTDv05_decompressBegin(dctx);
    dctx
}
#[export_name = crate::prefix!(ZSTDv05_freeDCtx)]
pub unsafe extern "C" fn ZSTDv05_freeDCtx(mut dctx: *mut ZSTDv05_DCtx) -> size_t {
    free(dctx as *mut std::ffi::c_void);
    0
}
#[export_name = crate::prefix!(ZSTDv05_copyDCtx)]
pub unsafe extern "C" fn ZSTDv05_copyDCtx(
    mut dstDCtx: *mut ZSTDv05_DCtx,
    mut srcDCtx: *const ZSTDv05_DCtx,
) {
    memcpy(
        dstDCtx as *mut std::ffi::c_void,
        srcDCtx as *const std::ffi::c_void,
        (::core::mem::size_of::<ZSTDv05_DCtx>() as std::ffi::c_ulong).wrapping_sub(
            (BLOCKSIZE + WILDCOPY_OVERLENGTH + ZSTDv05_frameHeaderSize_max) as std::ffi::c_ulong,
        ),
    );
}
unsafe extern "C" fn ZSTDv05_decodeFrameHeader_Part1(
    mut zc: *mut ZSTDv05_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut magicNumber: u32 = 0;
    if srcSize != ZSTDv05_frameHeaderSize_min {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    magicNumber = MEM_readLE32(src);
    if magicNumber != ZSTDv05_MAGICNUMBER {
        return -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t;
    }
    (*zc).headerSize = ZSTDv05_frameHeaderSize_min;
    (*zc).headerSize
}
#[export_name = crate::prefix!(ZSTDv05_getFrameParams)]
pub unsafe extern "C" fn ZSTDv05_getFrameParams(
    mut params: *mut ZSTDv05_parameters,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut magicNumber: u32 = 0;
    if srcSize < ZSTDv05_frameHeaderSize_min {
        return ZSTDv05_frameHeaderSize_max as size_t;
    }
    magicNumber = MEM_readLE32(src);
    if magicNumber != ZSTDv05_MAGICNUMBER {
        return -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t;
    }
    ptr::write_bytes(
        params as *mut u8,
        0,
        ::core::mem::size_of::<ZSTDv05_parameters>(),
    );
    (*params).windowLog = ((*(src as *const u8).offset(4) as std::ffi::c_int & 15)
        + ZSTDv05_WINDOWLOG_ABSOLUTEMIN) as u32;
    if *(src as *const u8).offset(4) as std::ffi::c_int >> 4 != 0 {
        return -(ZSTD_error_frameParameter_unsupported as std::ffi::c_int) as size_t;
    }
    0
}
unsafe extern "C" fn ZSTDv05_decodeFrameHeader_Part2(
    mut zc: *mut ZSTDv05_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut result: size_t = 0;
    if srcSize != (*zc).headerSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    result = ZSTDv05_getFrameParams(&mut (*zc).params, src, srcSize);
    if MEM_32bits() != 0 && (*zc).params.windowLog > 25 {
        return -(ZSTD_error_frameParameter_unsupported as std::ffi::c_int) as size_t;
    }
    result
}
unsafe extern "C" fn ZSTDv05_getcBlockSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut bpPtr: *mut blockProperties_t,
) -> size_t {
    let in_0 = src as *const u8;
    let mut headerFlags: u8 = 0;
    let mut cSize: u32 = 0;
    if srcSize < 3 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    headerFlags = *in_0;
    cSize = (*in_0.offset(2) as std::ffi::c_int
        + ((*in_0.offset(1) as std::ffi::c_int) << 8)
        + ((*in_0.offset(0) as std::ffi::c_int & 7) << 16)) as u32;
    (*bpPtr).blockType = (headerFlags as std::ffi::c_int >> 6) as blockType_t;
    (*bpPtr).origSize = if (*bpPtr).blockType as std::ffi::c_uint
        == bt_rle as std::ffi::c_int as std::ffi::c_uint
    {
        cSize
    } else {
        0
    };
    if (*bpPtr).blockType as std::ffi::c_uint == bt_end as std::ffi::c_int as std::ffi::c_uint {
        return 0;
    }
    if (*bpPtr).blockType as std::ffi::c_uint == bt_rle as std::ffi::c_int as std::ffi::c_uint {
        return 1;
    }
    cSize as size_t
}
unsafe extern "C" fn ZSTDv05_copyRawBlock(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if dst.is_null() {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if srcSize > maxDstSize {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    memcpy(dst, src, srcSize);
    srcSize
}
unsafe extern "C" fn ZSTDv05_decodeLiteralsBlock(
    mut dctx: *mut ZSTDv05_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const u8;
    if srcSize < MIN_CBLOCK_SIZE as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    match *istart.offset(0) as std::ffi::c_int >> 6 {
        IS_HUFv05 => {
            let mut litSize: size_t = 0;
            let mut litCSize: size_t = 0;
            let mut singleStream = 0;
            let mut lhSize = (*istart.offset(0) as std::ffi::c_int >> 4 & 3) as u32;
            if srcSize < 5 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            match lhSize {
                2 => {
                    lhSize = 4;
                    litSize = (((*istart.offset(0) as std::ffi::c_int & 15) << 10)
                        + ((*istart.offset(1) as std::ffi::c_int) << 2)
                        + (*istart.offset(2) as std::ffi::c_int >> 6))
                        as size_t;
                    litCSize = (((*istart.offset(2) as std::ffi::c_int & 63) << 8)
                        + *istart.offset(3) as std::ffi::c_int)
                        as size_t;
                }
                3 => {
                    lhSize = 5;
                    litSize = (((*istart.offset(0) as std::ffi::c_int & 15) << 14)
                        + ((*istart.offset(1) as std::ffi::c_int) << 6)
                        + (*istart.offset(2) as std::ffi::c_int >> 2))
                        as size_t;
                    litCSize = (((*istart.offset(2) as std::ffi::c_int & 3) << 16)
                        + ((*istart.offset(3) as std::ffi::c_int) << 8)
                        + *istart.offset(4) as std::ffi::c_int)
                        as size_t;
                }
                0 | 1 | _ => {
                    lhSize = 3;
                    singleStream = (*istart.offset(0) as std::ffi::c_int & 16) as size_t;
                    litSize = (((*istart.offset(0) as std::ffi::c_int & 15) << 6)
                        + (*istart.offset(1) as std::ffi::c_int >> 2))
                        as size_t;
                    litCSize = (((*istart.offset(1) as std::ffi::c_int & 3) << 8)
                        + *istart.offset(2) as std::ffi::c_int)
                        as size_t;
                }
            }
            if litSize > BLOCKSIZE as size_t {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if litCSize.wrapping_add(lhSize as size_t) > srcSize {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if HUFv05_isError(if singleStream != 0 {
                HUFv05_decompress1X2(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const std::ffi::c_void,
                    litCSize,
                )
            } else {
                HUFv05_decompress(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const std::ffi::c_void,
                    litCSize,
                )
            }) != 0
            {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize;
            ptr::write_bytes(
                ((*dctx).litBuffer)
                    .as_mut_ptr()
                    .offset((*dctx).litSize as isize) as *mut u8,
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            litCSize.wrapping_add(lhSize as size_t)
        }
        IS_PCH => {
            let mut errorCode: size_t = 0;
            let mut litSize_0: size_t = 0;
            let mut litCSize_0: size_t = 0;
            let mut lhSize_0 = (*istart.offset(0) as std::ffi::c_int >> 4 & 3) as u32;
            if lhSize_0 != 1 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if (*dctx).flagStaticTables == 0 {
                return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
            }
            lhSize_0 = 3;
            litSize_0 = (((*istart.offset(0) as std::ffi::c_int & 15) << 6)
                + (*istart.offset(1) as std::ffi::c_int >> 2)) as size_t;
            litCSize_0 = (((*istart.offset(1) as std::ffi::c_int & 3) << 8)
                + *istart.offset(2) as std::ffi::c_int) as size_t;
            if litCSize_0.wrapping_add(lhSize_0 as size_t) > srcSize {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            errorCode = HUFv05_decompress1X4_usingDTable(
                ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                litSize_0,
                istart.offset(lhSize_0 as isize) as *const std::ffi::c_void,
                litCSize_0,
                ((*dctx).hufTableX4).as_mut_ptr(),
            );
            if HUFv05_isError(errorCode) != 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_0;
            ptr::write_bytes(
                ((*dctx).litBuffer)
                    .as_mut_ptr()
                    .offset((*dctx).litSize as isize) as *mut u8,
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            litCSize_0.wrapping_add(lhSize_0 as size_t)
        }
        IS_RAW => {
            let mut litSize_1: size_t = 0;
            let mut lhSize_1 = (*istart.offset(0) as std::ffi::c_int >> 4 & 3) as u32;
            match lhSize_1 {
                2 => {
                    litSize_1 = (((*istart.offset(0) as std::ffi::c_int & 15) << 8)
                        + *istart.offset(1) as std::ffi::c_int)
                        as size_t;
                }
                3 => {
                    litSize_1 = (((*istart.offset(0) as std::ffi::c_int & 15) << 16)
                        + ((*istart.offset(1) as std::ffi::c_int) << 8)
                        + *istart.offset(2) as std::ffi::c_int)
                        as size_t;
                }
                0 | 1 | _ => {
                    lhSize_1 = 1;
                    litSize_1 = (*istart.offset(0) as std::ffi::c_int & 31) as size_t;
                }
            }
            if (lhSize_1 as size_t)
                .wrapping_add(litSize_1)
                .wrapping_add(WILDCOPY_OVERLENGTH as size_t)
                > srcSize
            {
                if litSize_1.wrapping_add(lhSize_1 as size_t) > srcSize {
                    return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                }
                memcpy(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                    istart.offset(lhSize_1 as isize) as *const std::ffi::c_void,
                    litSize_1,
                );
                (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
                (*dctx).litSize = litSize_1;
                ptr::write_bytes(
                    ((*dctx).litBuffer)
                        .as_mut_ptr()
                        .offset((*dctx).litSize as isize) as *mut u8,
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
            let mut lhSize_2 = (*istart.offset(0) as std::ffi::c_int >> 4 & 3) as u32;
            match lhSize_2 {
                2 => {
                    litSize_2 = (((*istart.offset(0) as std::ffi::c_int & 15) << 8)
                        + *istart.offset(1) as std::ffi::c_int)
                        as size_t;
                }
                3 => {
                    litSize_2 = (((*istart.offset(0) as std::ffi::c_int & 15) << 16)
                        + ((*istart.offset(1) as std::ffi::c_int) << 8)
                        + *istart.offset(2) as std::ffi::c_int)
                        as size_t;
                    if srcSize < 4 {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                    }
                }
                0 | 1 | _ => {
                    lhSize_2 = 1;
                    litSize_2 = (*istart.offset(0) as std::ffi::c_int & 31) as size_t;
                }
            }
            if litSize_2 > BLOCKSIZE as size_t {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            memset(
                ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                *istart.offset(lhSize_2 as isize) as std::ffi::c_int,
                litSize_2.wrapping_add(WILDCOPY_OVERLENGTH as size_t),
            );
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_2;
            lhSize_2.wrapping_add(1) as size_t
        }
        _ => -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t,
    }
}
unsafe extern "C" fn ZSTDv05_decodeSeqHeaders(
    mut nbSeq: *mut std::ffi::c_int,
    mut dumpsPtr: *mut *const u8,
    mut dumpsLengthPtr: *mut size_t,
    mut DTableLL: *mut FSEv05_DTable,
    mut DTableML: *mut FSEv05_DTable,
    mut DTableOffb: *mut FSEv05_DTable,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut flagStaticTable: u32,
) -> size_t {
    let istart = src as *const u8;
    let mut ip = istart;
    let iend = istart.offset(srcSize as isize);
    let mut LLtype: u32 = 0;
    let mut Offtype: u32 = 0;
    let mut MLtype: u32 = 0;
    let mut LLlog: std::ffi::c_uint = 0;
    let mut Offlog: std::ffi::c_uint = 0;
    let mut MLlog: std::ffi::c_uint = 0;
    let mut dumpsLength: size_t = 0;
    if srcSize < MIN_SEQUENCES_SIZE as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let fresh39 = ip;
    ip = ip.offset(1);
    *nbSeq = *fresh39 as std::ffi::c_int;
    if *nbSeq == 0 {
        return 1;
    }
    if *nbSeq >= 128 {
        if ip >= iend {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        let fresh40 = ip;
        ip = ip.offset(1);
        *nbSeq = ((*nbSeq.offset(0) - 128) << 8) + *fresh40 as std::ffi::c_int;
    }
    if ip >= iend {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    LLtype = (*ip as std::ffi::c_int >> 6) as u32;
    Offtype = (*ip as std::ffi::c_int >> 4 & 3) as u32;
    MLtype = (*ip as std::ffi::c_int >> 2 & 3) as u32;
    if *ip as std::ffi::c_int & 2 != 0 {
        if ip.offset(3) > iend {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        dumpsLength = *ip.offset(2) as size_t;
        dumpsLength = dumpsLength.wrapping_add(((*ip.offset(1) as std::ffi::c_int) << 8) as size_t);
        ip = ip.offset(3);
    } else {
        if ip.offset(2) > iend {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        dumpsLength = *ip.offset(1) as size_t;
        dumpsLength =
            dumpsLength.wrapping_add(((*ip.offset(0) as std::ffi::c_int & 1) << 8) as size_t);
        ip = ip.offset(2);
    }
    *dumpsPtr = ip;
    ip = ip.offset(dumpsLength as isize);
    *dumpsLengthPtr = dumpsLength;
    if ip > iend.offset(-(3)) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let mut norm: [i16; 128] = [0; 128];
    let mut headerSize: size_t = 0;
    match LLtype {
        1 => {
            LLlog = 0;
            let fresh41 = ip;
            ip = ip.offset(1);
            FSEv05_buildDTable_rle(DTableLL, *fresh41);
        }
        0 => {
            LLlog = LLbits as std::ffi::c_uint;
            FSEv05_buildDTable_raw(DTableLL, LLbits as std::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
        }
        3 | _ => {
            let mut max = MaxLL as std::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut LLlog,
                ip as *const std::ffi::c_void,
                iend.offset_from(ip) as std::ffi::c_long as size_t,
            );
            if FSEv05_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
            }
            if LLlog > LLFSEv05Log as std::ffi::c_uint {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSEv05_buildDTable(DTableLL, norm.as_mut_ptr(), max, LLlog);
        }
    }
    match Offtype {
        1 => {
            Offlog = 0;
            if ip > iend.offset(-(2)) {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            let fresh42 = ip;
            ip = ip.offset(1);
            FSEv05_buildDTable_rle(
                DTableOffb,
                (*fresh42 as std::ffi::c_int & MaxOff) as std::ffi::c_uchar,
            );
        }
        0 => {
            Offlog = Offbits as std::ffi::c_uint;
            FSEv05_buildDTable_raw(DTableOffb, Offbits as std::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
        }
        3 | _ => {
            let mut max_0 = MaxOff as std::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                norm.as_mut_ptr(),
                &mut max_0,
                &mut Offlog,
                ip as *const std::ffi::c_void,
                iend.offset_from(ip) as std::ffi::c_long as size_t,
            );
            if FSEv05_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
            }
            if Offlog > OffFSEv05Log as std::ffi::c_uint {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSEv05_buildDTable(DTableOffb, norm.as_mut_ptr(), max_0, Offlog);
        }
    }
    match MLtype {
        1 => {
            MLlog = 0;
            if ip > iend.offset(-(2)) {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            let fresh43 = ip;
            ip = ip.offset(1);
            FSEv05_buildDTable_rle(DTableML, *fresh43);
        }
        0 => {
            MLlog = MLbits as std::ffi::c_uint;
            FSEv05_buildDTable_raw(DTableML, MLbits as std::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
        }
        3 | _ => {
            let mut max_1 = MaxML as std::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                norm.as_mut_ptr(),
                &mut max_1,
                &mut MLlog,
                ip as *const std::ffi::c_void,
                iend.offset_from(ip) as std::ffi::c_long as size_t,
            );
            if FSEv05_isError(headerSize) != 0 {
                return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
            }
            if MLlog > MLFSEv05Log as std::ffi::c_uint {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            ip = ip.offset(headerSize as isize);
            FSEv05_buildDTable(DTableML, norm.as_mut_ptr(), max_1, MLlog);
        }
    }
    ip.offset_from(istart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTDv05_decodeSequence(mut seq: *mut seq_t, mut seqState: *mut seqState_t) {
    let mut litLength: size_t = 0;
    let mut prevOffset: size_t = 0;
    let mut offset: size_t = 0;
    let mut matchLength: size_t = 0;
    let mut dumps = (*seqState).dumps;
    let de = (*seqState).dumpsEnd;
    litLength = FSEv05_peakSymbol(&mut (*seqState).stateLL) as size_t;
    prevOffset = if litLength != 0 {
        (*seq).offset
    } else {
        (*seqState).prevOffset
    };
    if litLength == MaxLL as size_t {
        let fresh44 = dumps;
        dumps = dumps.offset(1);
        let add = *fresh44 as u32;
        if add < 255 {
            litLength = litLength.wrapping_add(add as size_t);
        } else if dumps.offset(2) <= de {
            litLength = MEM_readLE16(dumps as *const std::ffi::c_void) as size_t;
            dumps = dumps.offset(2);
            if litLength & 1 != 0 && dumps < de {
                litLength = litLength.wrapping_add(((*dumps as std::ffi::c_int) << 16) as size_t);
                dumps = dumps.offset(1);
            }
            litLength >>= 1;
        }
        if dumps >= de {
            dumps = de.offset(-(1));
        }
    }
    static offsetPrefix: [u32; 32] = [
        1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
        131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 1, 1, 1, 1,
        1,
    ];
    let mut offsetCode = FSEv05_peakSymbol(&mut (*seqState).stateOffb) as u32;
    let mut nbBits = offsetCode.wrapping_sub(1);
    if offsetCode == 0 {
        nbBits = 0;
    }
    offset = (*offsetPrefix.as_ptr().offset(offsetCode as isize) as size_t)
        .wrapping_add(BITv05_readBits(&mut (*seqState).DStream, nbBits));
    if MEM_32bits() != 0 {
        BITv05_reloadDStream(&mut (*seqState).DStream);
    }
    if offsetCode == 0 {
        offset = prevOffset;
    }
    if offsetCode | (litLength == 0) as std::ffi::c_int as u32 != 0 {
        (*seqState).prevOffset = (*seq).offset;
    }
    FSEv05_decodeSymbol(&mut (*seqState).stateOffb, &mut (*seqState).DStream);
    FSEv05_decodeSymbol(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    if MEM_32bits() != 0 {
        BITv05_reloadDStream(&mut (*seqState).DStream);
    }
    matchLength = FSEv05_decodeSymbol(&mut (*seqState).stateML, &mut (*seqState).DStream) as size_t;
    if matchLength == MaxML as size_t {
        let add_0 = (if dumps < de {
            let fresh45 = dumps;
            dumps = dumps.offset(1);
            *fresh45 as std::ffi::c_int
        } else {
            0
        }) as u32;
        if add_0 < 255 {
            matchLength = matchLength.wrapping_add(add_0 as size_t);
        } else if dumps.offset(2) <= de {
            matchLength = MEM_readLE16(dumps as *const std::ffi::c_void) as size_t;
            dumps = dumps.offset(2);
            if matchLength & 1 != 0 && dumps < de {
                matchLength =
                    matchLength.wrapping_add(((*dumps as std::ffi::c_int) << 16) as size_t);
                dumps = dumps.offset(1);
            }
            matchLength >>= 1;
        }
        if dumps >= de {
            dumps = de.offset(-(1));
        }
    }
    matchLength = matchLength.wrapping_add(MINMATCH as size_t);
    (*seq).litLength = litLength;
    (*seq).offset = offset;
    (*seq).matchLength = matchLength;
    (*seqState).dumps = dumps;
}
unsafe extern "C" fn ZSTDv05_execSequence(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    base: *const u8,
    vBase: *const u8,
    dictEnd: *const u8,
) -> size_t {
    static dec32table: [std::ffi::c_int; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
    static dec64table: [std::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_8 = oend.wrapping_sub(8);
    let litEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset as usize);
    let seqLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    if seqLength > oend.offset_from(op) as std::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as std::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if oLitEnd > oend_8 {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if litEnd > litLimit {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ZSTDv05_wildcopy(
        op as *mut std::ffi::c_void,
        *litPtr as *const std::ffi::c_void,
        sequence.litLength as ptrdiff_t,
    );
    op = oLitEnd;
    *litPtr = litEnd;
    if sequence.offset > oLitEnd.offset_from(base) as std::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(vBase) as std::ffi::c_long as size_t {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(-(base.offset_from(match_0) as std::ffi::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            memmove(
                oLitEnd as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                sequence.matchLength,
            );
            return sequenceLength;
        }
        let mut length1 = dictEnd.offset_from(match_0) as std::ffi::c_long as size_t;
        memmove(
            oLitEnd as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            length1,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = base;
        if op > oend_8 || sequence.matchLength < MINMATCH as size_t {
            while op < oMatchEnd {
                let fresh46 = match_0;
                match_0 = match_0.offset(1);
                let fresh47 = op;
                op = op.offset(1);
                *fresh47 = *fresh46;
            }
            return sequenceLength;
        }
    }
    if sequence.offset < 8 {
        let sub2 = *dec64table.as_ptr().offset(sequence.offset as isize);
        *op.offset(0) = *match_0.offset(0);
        *op.offset(1) = *match_0.offset(1);
        *op.offset(2) = *match_0.offset(2);
        *op.offset(3) = *match_0.offset(3);
        match_0 = match_0.offset(*dec32table.as_ptr().offset(sequence.offset as isize) as isize);
        ZSTDv05_copy4(
            op.offset(4) as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
        );
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ZSTDv05_copy8(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
        );
    }
    op = op.offset(8);
    match_0 = match_0.offset(8);
    if oMatchEnd > oend.offset(-((16 - MINMATCH) as isize)) {
        if op < oend_8 {
            ZSTDv05_wildcopy(
                op as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                oend_8.offset_from(op) as std::ffi::c_long,
            );
            match_0 = match_0.offset(oend_8.offset_from(op) as std::ffi::c_long as isize);
            op = oend_8;
        }
        while op < oMatchEnd {
            let fresh48 = match_0;
            match_0 = match_0.offset(1);
            let fresh49 = op;
            op = op.offset(1);
            *fresh49 = *fresh48;
        }
    } else {
        ZSTDv05_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            sequence.matchLength as ptrdiff_t - 8,
        );
    }
    sequenceLength
}
unsafe extern "C" fn ZSTDv05_decompressSequences(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
) -> size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.offset(maxDstSize as isize);
    let mut errorCode: size_t = 0;
    let mut dumpsLength = 0;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.offset((*dctx).litSize as isize);
    let mut nbSeq = 0;
    let mut dumps = NULL as *const u8;
    let mut DTableLL = ((*dctx).LLTable).as_mut_ptr();
    let mut DTableML = ((*dctx).MLTable).as_mut_ptr();
    let mut DTableOffb = ((*dctx).OffTable).as_mut_ptr();
    let base = (*dctx).base as *const u8;
    let vBase = (*dctx).vBase as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    errorCode = ZSTDv05_decodeSeqHeaders(
        &mut nbSeq,
        &mut dumps,
        &mut dumpsLength,
        DTableLL,
        DTableML,
        DTableOffb,
        ip as *const std::ffi::c_void,
        seqSize,
        (*dctx).flagStaticTables,
    );
    if ZSTDv05_isError(errorCode) != 0 {
        return errorCode;
    }
    ip = ip.offset(errorCode as isize);
    if nbSeq != 0 {
        let mut sequence = seq_t {
            litLength: 0,
            matchLength: 0,
            offset: 0,
        };
        let mut seqState = seqState_t {
            DStream: BITv05_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: std::ptr::null::<std::ffi::c_char>(),
                start: std::ptr::null::<std::ffi::c_char>(),
            },
            stateLL: FSEv05_DState_t {
                state: 0,
                table: std::ptr::null::<std::ffi::c_void>(),
            },
            stateOffb: FSEv05_DState_t {
                state: 0,
                table: std::ptr::null::<std::ffi::c_void>(),
            },
            stateML: FSEv05_DState_t {
                state: 0,
                table: std::ptr::null::<std::ffi::c_void>(),
            },
            prevOffset: 0,
            dumps: std::ptr::null::<u8>(),
            dumpsEnd: std::ptr::null::<u8>(),
        };
        ptr::write_bytes(
            &mut sequence as *mut seq_t as *mut u8,
            0,
            ::core::mem::size_of::<seq_t>(),
        );
        sequence.offset = REPCODE_STARTVALUE as size_t;
        seqState.dumps = dumps;
        seqState.dumpsEnd = dumps.offset(dumpsLength as isize);
        seqState.prevOffset = REPCODE_STARTVALUE as size_t;
        errorCode = BITv05_initDStream(
            &mut seqState.DStream,
            ip as *const std::ffi::c_void,
            iend.offset_from(ip) as std::ffi::c_long as size_t,
        );
        if ERR_isError(errorCode) != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        FSEv05_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
        FSEv05_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
        FSEv05_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
        while BITv05_reloadDStream(&mut seqState.DStream) as std::ffi::c_uint
            <= BITv05_DStream_completed as std::ffi::c_int as std::ffi::c_uint
            && nbSeq != 0
        {
            let mut oneSeqSize: size_t = 0;
            nbSeq -= 1;
            ZSTDv05_decodeSequence(&mut sequence, &mut seqState);
            oneSeqSize = ZSTDv05_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litEnd,
                base,
                vBase,
                dictEnd,
            );
            if ZSTDv05_isError(oneSeqSize) != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
    }
    let mut lastLLSize = litEnd.offset_from(litPtr) as std::ffi::c_long as size_t;
    if litPtr > litEnd {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op.offset(lastLLSize as isize) > oend {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if lastLLSize > 0 {
        memcpy(
            op as *mut std::ffi::c_void,
            litPtr as *const std::ffi::c_void,
            lastLLSize,
        );
        op = op.offset(lastLLSize as isize);
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTDv05_checkContinuity(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *const std::ffi::c_void,
) {
    if dst != (*dctx).previousDstEnd {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx).vBase = (dst as *const std::ffi::c_char).offset(
            -(((*dctx).previousDstEnd as *const std::ffi::c_char)
                .offset_from((*dctx).base as *const std::ffi::c_char)
                as std::ffi::c_long as isize),
        ) as *const std::ffi::c_void;
        (*dctx).base = dst;
        (*dctx).previousDstEnd = dst;
    }
}
unsafe extern "C" fn ZSTDv05_decompressBlock_internal(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    let mut litCSize: size_t = 0;
    if srcSize >= BLOCKSIZE as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    litCSize = ZSTDv05_decodeLiteralsBlock(dctx, src, srcSize);
    if ZSTDv05_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.offset(litCSize as isize);
    srcSize = srcSize.wrapping_sub(litCSize);
    ZSTDv05_decompressSequences(
        dctx,
        dst,
        dstCapacity,
        ip as *const std::ffi::c_void,
        srcSize,
    )
}
#[export_name = crate::prefix!(ZSTDv05_decompressBlock)]
pub unsafe extern "C" fn ZSTDv05_decompressBlock(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv05_checkContinuity(dctx, dst);
    ZSTDv05_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize)
}
unsafe extern "C" fn ZSTDv05_decompress_continueDCtx(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    let mut iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.offset(maxDstSize as isize);
    let mut remainingSize = srcSize;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    ptr::write_bytes(
        &mut blockProperties as *mut blockProperties_t as *mut u8,
        0,
        ::core::mem::size_of::<blockProperties_t>(),
    );
    let mut frameHeaderSize: size_t = 0;
    if srcSize < ZSTDv05_frameHeaderSize_min.wrapping_add(ZSTDv05_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    frameHeaderSize = ZSTDv05_decodeFrameHeader_Part1(dctx, src, ZSTDv05_frameHeaderSize_min);
    if ZSTDv05_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv05_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(frameHeaderSize as isize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    frameHeaderSize = ZSTDv05_decodeFrameHeader_Part2(dctx, src, frameHeaderSize);
    if ZSTDv05_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    loop {
        let mut decodedSize = 0;
        let mut cBlockSize = ZSTDv05_getcBlockSize(
            ip as *const std::ffi::c_void,
            iend.offset_from(ip) as std::ffi::c_long as size_t,
            &mut blockProperties,
        );
        if ZSTDv05_isError(cBlockSize) != 0 {
            return cBlockSize;
        }
        ip = ip.offset(ZSTDv05_blockHeaderSize as isize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv05_blockHeaderSize);
        if cBlockSize > remainingSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        match blockProperties.blockType as std::ffi::c_uint {
            0 => {
                decodedSize = ZSTDv05_decompressBlock_internal(
                    dctx,
                    op as *mut std::ffi::c_void,
                    oend.offset_from(op) as std::ffi::c_long as size_t,
                    ip as *const std::ffi::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTDv05_copyRawBlock(
                    op as *mut std::ffi::c_void,
                    oend.offset_from(op) as std::ffi::c_long as size_t,
                    ip as *const std::ffi::c_void,
                    cBlockSize,
                );
            }
            2 => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
            3 => {
                if remainingSize != 0 {
                    return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
                }
            }
            _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
        }
        if cBlockSize == 0 {
            break;
        }
        if ZSTDv05_isError(decodedSize) != 0 {
            return decodedSize;
        }
        op = op.offset(decodedSize as isize);
        ip = ip.offset(cBlockSize as isize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
#[export_name = crate::prefix!(ZSTDv05_decompress_usingPreparedDCtx)]
pub unsafe extern "C" fn ZSTDv05_decompress_usingPreparedDCtx(
    mut dctx: *mut ZSTDv05_DCtx,
    mut refDCtx: *const ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv05_copyDCtx(dctx, refDCtx);
    ZSTDv05_checkContinuity(dctx, dst);
    ZSTDv05_decompress_continueDCtx(dctx, dst, maxDstSize, src, srcSize)
}
#[export_name = crate::prefix!(ZSTDv05_decompress_usingDict)]
pub unsafe extern "C" fn ZSTDv05_decompress_usingDict(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    ZSTDv05_decompressBegin_usingDict(dctx, dict, dictSize);
    ZSTDv05_checkContinuity(dctx, dst);
    ZSTDv05_decompress_continueDCtx(dctx, dst, maxDstSize, src, srcSize)
}
#[export_name = crate::prefix!(ZSTDv05_decompressDCtx)]
pub unsafe extern "C" fn ZSTDv05_decompressDCtx(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv05_decompress_usingDict(
        dctx,
        dst,
        maxDstSize,
        src,
        srcSize,
        NULL as *const std::ffi::c_void,
        0,
    )
}
#[export_name = crate::prefix!(ZSTDv05_decompress)]
pub unsafe extern "C" fn ZSTDv05_decompress(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut regenSize: size_t = 0;
    let mut dctx = ZSTDv05_createDCtx();
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    regenSize = ZSTDv05_decompressDCtx(dctx, dst, maxDstSize, src, srcSize);
    ZSTDv05_freeDCtx(dctx);
    regenSize
}
unsafe extern "C" fn ZSTD_errorFrameSizeInfoLegacy(
    mut cSize: *mut size_t,
    mut dBound: *mut std::ffi::c_ulonglong,
    mut ret: size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
#[export_name = crate::prefix!(ZSTDv05_findFrameSizeInfoLegacy)]
pub unsafe extern "C" fn ZSTDv05_findFrameSizeInfoLegacy(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut cSize: *mut size_t,
    mut dBound: *mut std::ffi::c_ulonglong,
) {
    let mut ip = src as *const u8;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as std::ffi::c_int as size_t;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    if srcSize < ZSTDv05_frameHeaderSize_min {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t,
        );
        return;
    }
    if MEM_readLE32(src) != ZSTDv05_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t,
        );
        return;
    }
    ip = ip.offset(ZSTDv05_frameHeaderSize_min as isize);
    remainingSize = remainingSize.wrapping_sub(ZSTDv05_frameHeaderSize_min);
    loop {
        let mut cBlockSize = ZSTDv05_getcBlockSize(
            ip as *const std::ffi::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ZSTDv05_isError(cBlockSize) != 0 {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, cBlockSize);
            return;
        }
        ip = ip.offset(ZSTDv05_blockHeaderSize as isize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv05_blockHeaderSize);
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(
                cSize,
                dBound,
                -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t,
            );
            return;
        }
        if cBlockSize == 0 {
            break;
        }
        ip = ip.offset(cBlockSize as isize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const u8) as std::ffi::c_long as size_t;
    *dBound = (nbBlocks * BLOCKSIZE as size_t) as std::ffi::c_ulonglong;
}
#[export_name = crate::prefix!(ZSTDv05_nextSrcSizeToDecompress)]
pub unsafe extern "C" fn ZSTDv05_nextSrcSizeToDecompress(mut dctx: *mut ZSTDv05_DCtx) -> size_t {
    (*dctx).expected
}
#[export_name = crate::prefix!(ZSTDv05_decompressContinue)]
pub unsafe extern "C" fn ZSTDv05_decompressContinue(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize != (*dctx).expected {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ZSTDv05_checkContinuity(dctx, dst);
    match (*dctx).stage as std::ffi::c_uint {
        0 => {
            if srcSize != ZSTDv05_frameHeaderSize_min {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            (*dctx).headerSize =
                ZSTDv05_decodeFrameHeader_Part1(dctx, src, ZSTDv05_frameHeaderSize_min);
            if ZSTDv05_isError((*dctx).headerSize) != 0 {
                return (*dctx).headerSize;
            }
            memcpy(
                ((*dctx).headerBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                src,
                ZSTDv05_frameHeaderSize_min,
            );
            if (*dctx).headerSize > ZSTDv05_frameHeaderSize_min {
                return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
            }
            (*dctx).expected = 0;
        }
        1 => {}
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_compressed,
                origSize: 0,
            };
            let mut blockSize = ZSTDv05_getcBlockSize(src, ZSTDv05_blockHeaderSize, &mut bp);
            if ZSTDv05_isError(blockSize) != 0 {
                return blockSize;
            }
            if bp.blockType as std::ffi::c_uint == bt_end as std::ffi::c_int as std::ffi::c_uint {
                (*dctx).expected = 0;
                (*dctx).stage = ZSTDv05ds_getFrameHeaderSize;
            } else {
                (*dctx).expected = blockSize;
                (*dctx).bType = bp.blockType;
                (*dctx).stage = ZSTDv05ds_decompressBlock;
            }
            return 0;
        }
        3 => {
            let mut rSize: size_t = 0;
            match (*dctx).bType as std::ffi::c_uint {
                0 => {
                    rSize = ZSTDv05_decompressBlock_internal(dctx, dst, maxDstSize, src, srcSize);
                }
                1 => {
                    rSize = ZSTDv05_copyRawBlock(dst, maxDstSize, src, srcSize);
                }
                2 => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
                3 => {
                    rSize = 0;
                }
                _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
            }
            (*dctx).stage = ZSTDv05ds_decodeBlockHeader;
            (*dctx).expected = ZSTDv05_blockHeaderSize;
            if ZSTDv05_isError(rSize) != 0 {
                return rSize;
            }
            (*dctx).previousDstEnd =
                (dst as *mut std::ffi::c_char).offset(rSize as isize) as *const std::ffi::c_void;
            return rSize;
        }
        _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
    }
    let result = ZSTDv05_decodeFrameHeader_Part2(
        dctx,
        ((*dctx).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
        (*dctx).headerSize,
    );
    if ZSTDv05_isError(result) != 0 {
        return result;
    }
    (*dctx).expected = ZSTDv05_blockHeaderSize;
    (*dctx).stage = ZSTDv05ds_decodeBlockHeader;
    0
}
unsafe extern "C" fn ZSTDv05_refDictContent(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).vBase = (dict as *const std::ffi::c_char).offset(
        -(((*dctx).previousDstEnd as *const std::ffi::c_char)
            .offset_from((*dctx).base as *const std::ffi::c_char) as std::ffi::c_long
            as isize),
    ) as *const std::ffi::c_void;
    (*dctx).base = dict;
    (*dctx).previousDstEnd =
        (dict as *const std::ffi::c_char).offset(dictSize as isize) as *const std::ffi::c_void;
}
unsafe extern "C" fn ZSTDv05_loadEntropy(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut hSize: size_t = 0;
    let mut offcodeHeaderSize: size_t = 0;
    let mut matchlengthHeaderSize: size_t = 0;
    let mut errorCode: size_t = 0;
    let mut litlengthHeaderSize: size_t = 0;
    let mut offcodeNCount: [std::ffi::c_short; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff as std::ffi::c_uint;
    let mut offcodeLog: std::ffi::c_uint = 0;
    let mut matchlengthNCount: [std::ffi::c_short; 128] = [0; 128];
    let mut matchlengthMaxValue = MaxML as std::ffi::c_uint;
    let mut matchlengthLog: std::ffi::c_uint = 0;
    let mut litlengthNCount: [std::ffi::c_short; 64] = [0; 64];
    let mut litlengthMaxValue = MaxLL as std::ffi::c_uint;
    let mut litlengthLog: std::ffi::c_uint = 0;
    hSize = HUFv05_readDTableX4(((*dctx).hufTableX4).as_mut_ptr(), dict, dictSize);
    if HUFv05_isError(hSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(hSize as isize) as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(hSize);
    offcodeHeaderSize = FSEv05_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dict,
        dictSize,
    );
    if FSEv05_isError(offcodeHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if offcodeLog > OffFSEv05Log as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    errorCode = FSEv05_buildDTable(
        ((*dctx).OffTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        offcodeLog,
    );
    if FSEv05_isError(errorCode) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(offcodeHeaderSize as isize)
        as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(offcodeHeaderSize);
    matchlengthHeaderSize = FSEv05_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dict,
        dictSize,
    );
    if FSEv05_isError(matchlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if matchlengthLog > MLFSEv05Log as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    errorCode = FSEv05_buildDTable(
        ((*dctx).MLTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
    );
    if FSEv05_isError(errorCode) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(matchlengthHeaderSize as isize)
        as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(matchlengthHeaderSize);
    litlengthHeaderSize = FSEv05_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dict,
        dictSize,
    );
    if litlengthLog > LLFSEv05Log as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if FSEv05_isError(litlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    errorCode = FSEv05_buildDTable(
        ((*dctx).LLTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
    );
    if FSEv05_isError(errorCode) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    (*dctx).flagStaticTables = 1;
    hSize
        .wrapping_add(offcodeHeaderSize)
        .wrapping_add(matchlengthHeaderSize)
        .wrapping_add(litlengthHeaderSize)
}
unsafe extern "C" fn ZSTDv05_decompress_insertDictionary(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut eSize: size_t = 0;
    let mut magic = MEM_readLE32(dict);
    if magic != ZSTDv05_DICT_MAGIC {
        ZSTDv05_refDictContent(dctx, dict, dictSize);
        return 0;
    }
    dict = (dict as *const std::ffi::c_char).offset(4) as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(4);
    eSize = ZSTDv05_loadEntropy(dctx, dict, dictSize);
    if ZSTDv05_isError(eSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(eSize as isize) as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(eSize);
    ZSTDv05_refDictContent(dctx, dict, dictSize);
    0
}
#[export_name = crate::prefix!(ZSTDv05_decompressBegin_usingDict)]
pub unsafe extern "C" fn ZSTDv05_decompressBegin_usingDict(
    mut dctx: *mut ZSTDv05_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut errorCode: size_t = 0;
    errorCode = ZSTDv05_decompressBegin(dctx);
    if ZSTDv05_isError(errorCode) != 0 {
        return errorCode;
    }
    if !dict.is_null() && dictSize != 0 {
        errorCode = ZSTDv05_decompress_insertDictionary(dctx, dict, dictSize);
        if ZSTDv05_isError(errorCode) != 0 {
            return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
        }
    }
    0
}
static ZBUFFv05_blockHeaderSize: size_t = 3;
unsafe extern "C" fn ZBUFFv05_limitCopy(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut length = if maxDstSize < srcSize {
        maxDstSize
    } else {
        srcSize
    };
    if length > 0 {
        memcpy(dst, src, length);
    }
    length
}
pub const ZSTDv05_frameHeaderSize_max_0: std::ffi::c_int = 5;
#[export_name = crate::prefix!(ZBUFFv05_createDCtx)]
pub unsafe extern "C" fn ZBUFFv05_createDCtx() -> *mut ZBUFFv05_DCtx {
    let mut zbc =
        malloc(::core::mem::size_of::<ZBUFFv05_DCtx>() as std::ffi::c_ulong) as *mut ZBUFFv05_DCtx;
    if zbc.is_null() {
        return NULL as *mut ZBUFFv05_DCtx;
    }
    ptr::write_bytes(zbc as *mut u8, 0, ::core::mem::size_of::<ZBUFFv05_DCtx>());
    (*zbc).zc = ZSTDv05_createDCtx();
    (*zbc).stage = ZBUFFv05ds_init;
    zbc
}
#[export_name = crate::prefix!(ZBUFFv05_freeDCtx)]
pub unsafe extern "C" fn ZBUFFv05_freeDCtx(mut zbc: *mut ZBUFFv05_DCtx) -> size_t {
    if zbc.is_null() {
        return 0;
    }
    ZSTDv05_freeDCtx((*zbc).zc);
    free((*zbc).inBuff as *mut std::ffi::c_void);
    free((*zbc).outBuff as *mut std::ffi::c_void);
    free(zbc as *mut std::ffi::c_void);
    0
}
#[export_name = crate::prefix!(ZBUFFv05_decompressInitDictionary)]
pub unsafe extern "C" fn ZBUFFv05_decompressInitDictionary(
    mut zbc: *mut ZBUFFv05_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    (*zbc).stage = ZBUFFv05ds_readHeader;
    (*zbc).outEnd = 0;
    (*zbc).outStart = (*zbc).outEnd;
    (*zbc).inPos = (*zbc).outStart;
    (*zbc).hPos = (*zbc).inPos;
    ZSTDv05_decompressBegin_usingDict((*zbc).zc, dict, dictSize)
}
#[export_name = crate::prefix!(ZBUFFv05_decompressInit)]
pub unsafe extern "C" fn ZBUFFv05_decompressInit(mut zbc: *mut ZBUFFv05_DCtx) -> size_t {
    ZBUFFv05_decompressInitDictionary(zbc, NULL as *const std::ffi::c_void, 0)
}
#[export_name = crate::prefix!(ZBUFFv05_decompressContinue)]
pub unsafe extern "C" fn ZBUFFv05_decompressContinue(
    mut zbc: *mut ZBUFFv05_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSizePtr: *mut size_t,
    mut src: *const std::ffi::c_void,
    mut srcSizePtr: *mut size_t,
) -> size_t {
    let istart = src as *const std::ffi::c_char;
    let mut ip = istart;
    let iend = istart.offset(*srcSizePtr as isize);
    let ostart = dst as *mut std::ffi::c_char;
    let mut op = ostart;
    let oend = ostart.offset(*maxDstSizePtr as isize);
    let mut notDone = 1;
    while notDone != 0 {
        let mut current_block_68: u64;
        match (*zbc).stage as std::ffi::c_uint {
            0 => return -(ZSTD_error_init_missing as std::ffi::c_int) as size_t,
            1 => {
                let mut headerSize = ZSTDv05_getFrameParams(&mut (*zbc).params, src, *srcSizePtr);
                if ZSTDv05_isError(headerSize) != 0 {
                    return headerSize;
                }
                if headerSize != 0 {
                    memcpy(
                        ((*zbc).headerBuffer)
                            .as_mut_ptr()
                            .offset((*zbc).hPos as isize)
                            as *mut std::ffi::c_void,
                        src,
                        *srcSizePtr,
                    );
                    (*zbc).hPos = ((*zbc).hPos).wrapping_add(*srcSizePtr);
                    *maxDstSizePtr = 0;
                    (*zbc).stage = ZBUFFv05ds_loadHeader;
                    return headerSize.wrapping_sub((*zbc).hPos);
                }
                (*zbc).stage = ZBUFFv05ds_decodeHeader;
                current_block_68 = 10512632378975961025;
            }
            2 => {
                let mut headerSize_0 = ZBUFFv05_limitCopy(
                    ((*zbc).headerBuffer)
                        .as_mut_ptr()
                        .offset((*zbc).hPos as isize) as *mut std::ffi::c_void,
                    (ZSTDv05_frameHeaderSize_max_0 as size_t).wrapping_sub((*zbc).hPos),
                    src,
                    *srcSizePtr,
                );
                (*zbc).hPos = ((*zbc).hPos).wrapping_add(headerSize_0);
                ip = ip.offset(headerSize_0 as isize);
                headerSize_0 = ZSTDv05_getFrameParams(
                    &mut (*zbc).params,
                    ((*zbc).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                    (*zbc).hPos,
                );
                if ZSTDv05_isError(headerSize_0) != 0 {
                    return headerSize_0;
                }
                if headerSize_0 != 0 {
                    *maxDstSizePtr = 0;
                    return headerSize_0.wrapping_sub((*zbc).hPos);
                }
                current_block_68 = 10092070165720998011;
            }
            3 => {
                current_block_68 = 10092070165720998011;
            }
            4 => {
                current_block_68 = 6450636197030046351;
            }
            5 => {
                current_block_68 = 13460095289871124136;
            }
            6 => {
                current_block_68 = 12930649117290160518;
            }
            _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
        }
        if current_block_68 == 10092070165720998011 {
            let mut neededOutSize = (1) << (*zbc).params.windowLog;
            let mut neededInSize = BLOCKSIZE as size_t;
            if (*zbc).inBuffSize < neededInSize {
                free((*zbc).inBuff as *mut std::ffi::c_void);
                (*zbc).inBuffSize = neededInSize;
                (*zbc).inBuff = malloc(neededInSize) as *mut std::ffi::c_char;
                if ((*zbc).inBuff).is_null() {
                    return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                }
            }
            if (*zbc).outBuffSize < neededOutSize {
                free((*zbc).outBuff as *mut std::ffi::c_void);
                (*zbc).outBuffSize = neededOutSize;
                (*zbc).outBuff = malloc(neededOutSize) as *mut std::ffi::c_char;
                if ((*zbc).outBuff).is_null() {
                    return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                }
            }
            if (*zbc).hPos != 0 {
                memcpy(
                    (*zbc).inBuff as *mut std::ffi::c_void,
                    ((*zbc).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                    (*zbc).hPos,
                );
                (*zbc).inPos = (*zbc).hPos;
                (*zbc).hPos = 0;
                (*zbc).stage = ZBUFFv05ds_load;
                current_block_68 = 10512632378975961025;
            } else {
                (*zbc).stage = ZBUFFv05ds_read;
                current_block_68 = 6450636197030046351;
            }
        }
        if current_block_68 == 6450636197030046351 {
            let mut neededInSize_0 = ZSTDv05_nextSrcSizeToDecompress((*zbc).zc);
            if neededInSize_0 == 0 {
                (*zbc).stage = ZBUFFv05ds_init;
                notDone = 0;
                current_block_68 = 10512632378975961025;
            } else if iend.offset_from(ip) as std::ffi::c_long as size_t >= neededInSize_0 {
                let mut decodedSize = ZSTDv05_decompressContinue(
                    (*zbc).zc,
                    ((*zbc).outBuff).offset((*zbc).outStart as isize) as *mut std::ffi::c_void,
                    ((*zbc).outBuffSize).wrapping_sub((*zbc).outStart),
                    ip as *const std::ffi::c_void,
                    neededInSize_0,
                );
                if ZSTDv05_isError(decodedSize) != 0 {
                    return decodedSize;
                }
                ip = ip.offset(neededInSize_0 as isize);
                if decodedSize == 0 {
                    current_block_68 = 10512632378975961025;
                } else {
                    (*zbc).outEnd = ((*zbc).outStart).wrapping_add(decodedSize);
                    (*zbc).stage = ZBUFFv05ds_flush;
                    current_block_68 = 10512632378975961025;
                }
            } else if ip == iend {
                notDone = 0;
                current_block_68 = 10512632378975961025;
            } else {
                (*zbc).stage = ZBUFFv05ds_load;
                current_block_68 = 13460095289871124136;
            }
        }
        if current_block_68 == 13460095289871124136 {
            let mut neededInSize_1 = ZSTDv05_nextSrcSizeToDecompress((*zbc).zc);
            let mut toLoad = neededInSize_1.wrapping_sub((*zbc).inPos);
            let mut loadedSize: size_t = 0;
            if toLoad > ((*zbc).inBuffSize).wrapping_sub((*zbc).inPos) {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            loadedSize = ZBUFFv05_limitCopy(
                ((*zbc).inBuff).offset((*zbc).inPos as isize) as *mut std::ffi::c_void,
                toLoad,
                ip as *const std::ffi::c_void,
                iend.offset_from(ip) as std::ffi::c_long as size_t,
            );
            ip = ip.offset(loadedSize as isize);
            (*zbc).inPos = ((*zbc).inPos).wrapping_add(loadedSize);
            if loadedSize < toLoad {
                notDone = 0;
                current_block_68 = 10512632378975961025;
            } else {
                let mut decodedSize_0 = ZSTDv05_decompressContinue(
                    (*zbc).zc,
                    ((*zbc).outBuff).offset((*zbc).outStart as isize) as *mut std::ffi::c_void,
                    ((*zbc).outBuffSize).wrapping_sub((*zbc).outStart),
                    (*zbc).inBuff as *const std::ffi::c_void,
                    neededInSize_1,
                );
                if ZSTDv05_isError(decodedSize_0) != 0 {
                    return decodedSize_0;
                }
                (*zbc).inPos = 0;
                if decodedSize_0 == 0 {
                    (*zbc).stage = ZBUFFv05ds_read;
                    current_block_68 = 10512632378975961025;
                } else {
                    (*zbc).outEnd = ((*zbc).outStart).wrapping_add(decodedSize_0);
                    (*zbc).stage = ZBUFFv05ds_flush;
                    current_block_68 = 12930649117290160518;
                }
            }
        }
        if current_block_68 == 12930649117290160518 {
            let mut toFlushSize = ((*zbc).outEnd).wrapping_sub((*zbc).outStart);
            let mut flushedSize = ZBUFFv05_limitCopy(
                op as *mut std::ffi::c_void,
                oend.offset_from(op) as std::ffi::c_long as size_t,
                ((*zbc).outBuff).offset((*zbc).outStart as isize) as *const std::ffi::c_void,
                toFlushSize,
            );
            op = op.offset(flushedSize as isize);
            (*zbc).outStart = ((*zbc).outStart).wrapping_add(flushedSize);
            if flushedSize == toFlushSize {
                (*zbc).stage = ZBUFFv05ds_read;
                if ((*zbc).outStart).wrapping_add(BLOCKSIZE as size_t) > (*zbc).outBuffSize {
                    (*zbc).outEnd = 0;
                    (*zbc).outStart = (*zbc).outEnd;
                }
            } else {
                notDone = 0;
            }
        }
    }
    *srcSizePtr = ip.offset_from(istart) as std::ffi::c_long as size_t;
    *maxDstSizePtr = op.offset_from(ostart) as std::ffi::c_long as size_t;
    let mut nextSrcSizeHint = ZSTDv05_nextSrcSizeToDecompress((*zbc).zc);
    if nextSrcSizeHint > ZBUFFv05_blockHeaderSize {
        nextSrcSizeHint = nextSrcSizeHint.wrapping_add(ZBUFFv05_blockHeaderSize);
    }
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub((*zbc).inPos);
    nextSrcSizeHint
}
#[export_name = crate::prefix!(ZBUFFv05_isError)]
pub unsafe extern "C" fn ZBUFFv05_isError(mut errorCode: size_t) -> std::ffi::c_uint {
    ERR_isError(errorCode)
}
#[export_name = crate::prefix!(ZBUFFv05_getErrorName)]
pub unsafe extern "C" fn ZBUFFv05_getErrorName(mut errorCode: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(errorCode)
}
#[export_name = crate::prefix!(ZBUFFv05_recommendedDInSize)]
pub unsafe extern "C" fn ZBUFFv05_recommendedDInSize() -> size_t {
    (BLOCKSIZE as size_t).wrapping_add(ZBUFFv05_blockHeaderSize)
}
#[export_name = crate::prefix!(ZBUFFv05_recommendedDOutSize)]
pub unsafe extern "C" fn ZBUFFv05_recommendedDOutSize() -> size_t {
    BLOCKSIZE as size_t
}
