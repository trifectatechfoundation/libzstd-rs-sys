use core::ptr;

use libc::{free, malloc, memcpy, memmove, memset, ptrdiff_t, size_t};

use crate::lib::common::error_private::{ERR_getErrorName, ERR_isError};
use crate::lib::common::mem::{
    MEM_32bits, MEM_64bits, MEM_readLE16, MEM_readLE32, MEM_readLE64, MEM_readLEST, MEM_writeLE16,
};
use crate::lib::common::xxhash::{
    XXH64_state_t, ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_update,
};
use crate::lib::decompress::huf_decompress::DTableDesc;
use crate::lib::zstd::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_frameParams {
    pub frameContentSize: core::ffi::c_ulonglong,
    pub windowSize: core::ffi::c_uint,
    pub dictID: core::ffi::c_uint,
    pub checksumFlag: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: u32,
    pub c: [u8; 4],
}
pub type ZSTDv07_DCtx = ZSTDv07_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_DCtx_s {
    pub LLTable: [FSEv07_DTable; 513],
    pub OffTable: [FSEv07_DTable; 257],
    pub MLTable: [FSEv07_DTable; 513],
    pub hufTable: [HUFv07_DTable; 4097],
    pub previousDstEnd: *const core::ffi::c_void,
    pub base: *const core::ffi::c_void,
    pub vBase: *const core::ffi::c_void,
    pub dictEnd: *const core::ffi::c_void,
    pub expected: size_t,
    pub rep: [u32; 3],
    pub fParams: ZSTDv07_frameParams,
    pub bType: blockType_t,
    pub stage: ZSTDv07_dStage,
    pub litEntropy: u32,
    pub fseEntropy: u32,
    pub xxhState: XXH64_state_t,
    pub headerSize: size_t,
    pub dictID: u32,
    pub litPtr: *const u8,
    pub customMem: ZSTDv07_customMem,
    pub litSize: size_t,
    pub litBuffer: [u8; 131080],
    pub headerBuffer: [u8; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_customMem {
    pub customAlloc: ZSTDv07_allocFunction,
    pub customFree: ZSTDv07_freeFunction,
    pub opaque: *mut core::ffi::c_void,
}
pub type ZSTDv07_freeFunction =
    Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> ()>;
pub type ZSTDv07_allocFunction =
    Option<unsafe extern "C" fn(*mut core::ffi::c_void, size_t) -> *mut core::ffi::c_void>;
pub type ZSTDv07_dStage = core::ffi::c_uint;
pub const ZSTDds_skipFrame: ZSTDv07_dStage = 5;
pub const ZSTDds_decodeSkippableHeader: ZSTDv07_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTDv07_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTDv07_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTDv07_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTDv07_dStage = 0;
pub type blockType_t = core::ffi::c_uint;
pub const bt_end: blockType_t = 3;
pub const bt_rle: blockType_t = 2;
pub const bt_raw: blockType_t = 1;
pub const bt_compressed: blockType_t = 0;
pub type HUFv07_DTable = u32;
pub type FSEv07_DTable = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_t,
    pub origSize: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BITv07_DStream_t,
    pub stateLL: FSEv07_DState_t,
    pub stateOffb: FSEv07_DState_t,
    pub stateML: FSEv07_DState_t,
    pub prevOffset: [size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv07_DState_t {
    pub state: size_t,
    pub table: *const core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITv07_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: core::ffi::c_uint,
    pub ptr: *const core::ffi::c_char,
    pub start: *const core::ffi::c_char,
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
pub struct FSEv07_decode_t {
    pub newState: core::ffi::c_ushort,
    pub symbol: core::ffi::c_uchar,
    pub nbBits: core::ffi::c_uchar,
}
pub type BITv07_DStream_status = core::ffi::c_uint;
pub const BITv07_DStream_overflow: BITv07_DStream_status = 3;
pub const BITv07_DStream_completed: BITv07_DStream_status = 2;
pub const BITv07_DStream_endOfBuffer: BITv07_DStream_status = 1;
pub const BITv07_DStream_unfinished: BITv07_DStream_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv07_DTableHeader {
    pub tableLog: u16,
    pub fastMode: u16,
}
pub const lbt_rle: litBlockType_t = 3;
pub const lbt_raw: litBlockType_t = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv07_DEltX4 {
    pub sequence: u16,
    pub nbBits: u8,
    pub length: u8,
}
pub const lbt_repeat: litBlockType_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv07_DEltX2 {
    pub byte: u8,
    pub nbBits: u8,
}
pub type DTable_max_t = [u32; 4097];
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HUFv07_static_assert: C2RustUnnamed_0 = 1;
pub type rankVal_t = [[u32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: u8,
    pub weight: u8,
}
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const HUFv07_static_assert_0: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: u32,
    pub decode256Time: u32,
}
pub const lbt_huffman: litBlockType_t = 0;
pub type litBlockType_t = core::ffi::c_uint;
pub type ERR_enum = ZSTD_ErrorCode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv07_DDict_s {
    pub dict: *mut core::ffi::c_void,
    pub dictSize: size_t,
    pub refContext: *mut ZSTDv07_DCtx,
}
pub type ZSTDv07_DDict = ZSTDv07_DDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZBUFFv07_DCtx_s {
    pub zd: *mut ZSTDv07_DCtx,
    pub fParams: ZSTDv07_frameParams,
    pub stage: ZBUFFv07_dStage,
    pub inBuff: *mut core::ffi::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub outBuff: *mut core::ffi::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub blockSize: size_t,
    pub headerBuffer: [u8; 18],
    pub lhSize: size_t,
    pub customMem: ZSTDv07_customMem,
}
pub type ZBUFFv07_dStage = core::ffi::c_uint;
pub const ZBUFFds_flush: ZBUFFv07_dStage = 4;
pub const ZBUFFds_load: ZBUFFv07_dStage = 3;
pub const ZBUFFds_read: ZBUFFv07_dStage = 2;
pub const ZBUFFds_loadHeader: ZBUFFv07_dStage = 1;
pub const ZBUFFds_init: ZBUFFv07_dStage = 0;
pub type ZBUFFv07_DCtx = ZBUFFv07_DCtx_s;
pub type decompressionAlgo =
    Option<unsafe fn(*mut core::ffi::c_void, size_t, *const core::ffi::c_void, size_t) -> size_t>;
pub const ZSTDv07_MAGICNUMBER: core::ffi::c_uint = 0xfd2fb527 as core::ffi::c_uint;
pub const NULL: core::ffi::c_int = 0;
pub const ZSTDv07_MAGIC_SKIPPABLE_START: core::ffi::c_uint = 0x184d2a50 as core::ffi::c_uint;
pub const ZSTDv07_WINDOWLOG_MAX_32: core::ffi::c_int = 25;
pub const ZSTDv07_WINDOWLOG_MAX_64: core::ffi::c_int = 27;
pub const ZSTDv07_FRAMEHEADERSIZE_MAX: core::ffi::c_int = 18;
static ZSTDv07_frameHeaderSize_min: size_t = 5;
static ZSTDv07_frameHeaderSize_max: size_t = ZSTDv07_FRAMEHEADERSIZE_MAX as size_t;
static ZSTDv07_skippableHeaderSize: size_t = 8;
pub const ZSTDv07_BLOCKSIZE_ABSOLUTEMAX: core::ffi::c_int = 128 * 1024;
#[inline]
unsafe fn BITv07_highbit32(val: u32) -> core::ffi::c_uint {
    (val.leading_zeros() as i32 ^ 31) as core::ffi::c_uint
}
#[inline]
unsafe fn BITv07_initDStream(
    bitD: *mut BITv07_DStream_t,
    srcBuffer: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize < 1 {
        ptr::write_bytes(
            bitD as *mut u8,
            0,
            ::core::mem::size_of::<BITv07_DStream_t>(),
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
        (*bitD).bitsConsumed = if lastByte as core::ffi::c_int != 0 {
            (8 as core::ffi::c_uint).wrapping_sub(BITv07_highbit32(lastByte as u32))
        } else {
            0
        };
        if lastByte as core::ffi::c_int == 0 {
            return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
        }
    } else {
        (*bitD).start = srcBuffer as *const core::ffi::c_char;
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as size_t;
        let mut current_block_20: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                    (*(srcBuffer as *const u8).offset(6) as size_t)
                        << (::core::mem::size_of::<size_t>())
                            .wrapping_mul(8)
                            .wrapping_sub(16),
                );
                current_block_20 = 5278524126910058039;
            }
            6 => {
                current_block_20 = 5278524126910058039;
            }
            5 => {
                current_block_20 = 12979234177089312207;
            }
            4 => {
                current_block_20 = 147996480457998713;
            }
            3 => {
                current_block_20 = 17438867486747911763;
            }
            2 => {
                current_block_20 = 6492410612197318440;
            }
            _ => {
                current_block_20 = 5689001924483802034;
            }
        }
        if current_block_20 == 5278524126910058039 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(5) as size_t)
                    << (::core::mem::size_of::<size_t>())
                        .wrapping_mul(8)
                        .wrapping_sub(24),
            );
            current_block_20 = 12979234177089312207;
        }
        if current_block_20 == 12979234177089312207 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(4) as size_t)
                    << (::core::mem::size_of::<size_t>())
                        .wrapping_mul(8)
                        .wrapping_sub(32),
            );
            current_block_20 = 147996480457998713;
        }
        if current_block_20 == 147996480457998713 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(3) as size_t) << 24);
            current_block_20 = 17438867486747911763;
        }
        if current_block_20 == 17438867486747911763 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(2) as size_t) << 16);
            current_block_20 = 6492410612197318440;
        }
        if current_block_20 == 6492410612197318440 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(1) as size_t) << 8);
        }
        let lastByte_0 = *(srcBuffer as *const u8).add(srcSize.wrapping_sub(1));
        (*bitD).bitsConsumed = if lastByte_0 as core::ffi::c_int != 0 {
            (8 as core::ffi::c_uint).wrapping_sub(BITv07_highbit32(lastByte_0 as u32))
        } else {
            0
        };
        if lastByte_0 as core::ffi::c_int == 0 {
            return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(
            (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(srcSize) as u32 * 8,
        );
    }
    srcSize
}
#[inline]
unsafe fn BITv07_lookBits(bitD: *const BITv07_DStream_t, nbBits: u32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> 1
        >> (bitMask.wrapping_sub(nbBits) & bitMask)
}
#[inline]
unsafe fn BITv07_lookBitsFast(bitD: *const BITv07_DStream_t, nbBits: u32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> (bitMask.wrapping_add(1).wrapping_sub(nbBits) & bitMask)
}
#[inline]
unsafe fn BITv07_skipBits(bitD: *mut BITv07_DStream_t, nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline]
unsafe fn BITv07_readBits(bitD: *mut BITv07_DStream_t, nbBits: u32) -> size_t {
    let value = BITv07_lookBits(bitD, nbBits);
    BITv07_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe fn BITv07_readBitsFast(bitD: *mut BITv07_DStream_t, nbBits: u32) -> size_t {
    let value = BITv07_lookBitsFast(bitD, nbBits);
    BITv07_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe fn BITv07_reloadDStream(bitD: *mut BITv07_DStream_t) -> BITv07_DStream_status {
    if (*bitD).bitsConsumed as size_t > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
    {
        return BITv07_DStream_overflow;
    }
    if (*bitD).ptr >= ((*bitD).start).add(::core::mem::size_of::<size_t>()) {
        (*bitD).ptr = ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3) as isize));
        (*bitD).bitsConsumed &= 7;
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const core::ffi::c_void);
        return BITv07_DStream_unfinished;
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as size_t)
            < (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
        {
            return BITv07_DStream_endOfBuffer;
        }
        return BITv07_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3;
    let mut result = BITv07_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as core::ffi::c_long as u32;
        result = BITv07_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const core::ffi::c_void);
    result
}
#[inline]
unsafe fn BITv07_endOfDStream(DStream: *const BITv07_DStream_t) -> core::ffi::c_uint {
    ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as size_t
            == (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)) as core::ffi::c_int
        as core::ffi::c_uint
}
#[inline]
unsafe fn FSEv07_initDState(
    DStatePtr: *mut FSEv07_DState_t,
    bitD: *mut BITv07_DStream_t,
    dt: *const FSEv07_DTable,
) {
    let ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const FSEv07_DTableHeader;
    (*DStatePtr).state = BITv07_readBits(bitD, (*DTableH).tableLog as core::ffi::c_uint);
    BITv07_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1) as *const core::ffi::c_void;
}
#[inline]
unsafe fn FSEv07_peekSymbol(DStatePtr: *const FSEv07_DState_t) -> u8 {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    DInfo.symbol
}
#[inline]
unsafe fn FSEv07_updateState(DStatePtr: *mut FSEv07_DState_t, bitD: *mut BITv07_DStream_t) {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let lowBits = BITv07_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
}
#[inline]
unsafe fn FSEv07_decodeSymbol(
    DStatePtr: *mut FSEv07_DState_t,
    bitD: *mut BITv07_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv07_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe fn FSEv07_decodeSymbolFast(
    DStatePtr: *mut FSEv07_DState_t,
    bitD: *mut BITv07_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv07_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
pub const FSEv07_MAX_MEMORY_USAGE: core::ffi::c_int = 14;
pub const FSEv07_MAX_SYMBOL_VALUE: core::ffi::c_int = 255;
pub const FSEv07_MAX_TABLELOG: core::ffi::c_int = FSEv07_MAX_MEMORY_USAGE - 2;
pub const FSEv07_MIN_TABLELOG: core::ffi::c_int = 5;
pub const FSEv07_TABLELOG_ABSOLUTE_MAX: core::ffi::c_int = 15;
pub const HUFv07_TABLELOG_ABSOLUTEMAX: core::ffi::c_int = 16;
pub const HUFv07_TABLELOG_MAX: core::ffi::c_int = 12;
pub const HUFv07_SYMBOLVALUE_MAX: core::ffi::c_int = 255;
pub unsafe fn FSEv07_isError_0(code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
pub unsafe fn FSEv07_getErrorName(code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
pub unsafe fn HUFv07_isError(code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
pub unsafe fn HUFv07_getErrorName(code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
unsafe fn FSEv07_abs(a: core::ffi::c_short) -> core::ffi::c_short {
    (if (a as core::ffi::c_int) < 0 {
        -(a as core::ffi::c_int)
    } else {
        a as core::ffi::c_int
    }) as core::ffi::c_short
}
pub unsafe fn FSEv07_readNCount(
    normalizedCounter: *mut core::ffi::c_short,
    maxSVPtr: *mut core::ffi::c_uint,
    tableLogPtr: *mut core::ffi::c_uint,
    headerBuffer: *const core::ffi::c_void,
    hbSize: size_t,
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
    nbBits = (bitStream & 0xf as core::ffi::c_int as u32).wrapping_add(FSEv07_MIN_TABLELOG as u32)
        as core::ffi::c_int;
    if nbBits > FSEv07_TABLELOG_ABSOLUTE_MAX {
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
        remaining -= FSEv07_abs(count) as core::ffi::c_int;
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
pub unsafe fn HUFv07_readStats(
    huffWeight: *mut u8,
    hwSize: size_t,
    rankStats: *mut u32,
    nbSymbolsPtr: *mut u32,
    tableLogPtr: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
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
        oSize = FSEv07_decompress(
            huffWeight as *mut core::ffi::c_void,
            hwSize.wrapping_sub(1),
            ip.offset(1) as *const core::ffi::c_void,
            iSize,
        );
        if FSEv07_isError_0(oSize) != 0 {
            return oSize;
        }
    }
    memset(
        rankStats as *mut core::ffi::c_void,
        0,
        ((HUFv07_TABLELOG_ABSOLUTEMAX + 1) as size_t)
            .wrapping_mul(::core::mem::size_of::<u32>() as size_t),
    );
    weightTotal = 0;
    let mut n_0: u32 = 0;
    n_0 = 0;
    while (n_0 as size_t) < oSize {
        if *huffWeight.offset(n_0 as isize) as core::ffi::c_int >= HUFv07_TABLELOG_ABSOLUTEMAX {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        let fresh2 = &mut (*rankStats.offset(*huffWeight.offset(n_0 as isize) as isize));
        *fresh2 = (*fresh2).wrapping_add(1);
        weightTotal = weightTotal.wrapping_add(
            ((1) << *huffWeight.offset(n_0 as isize) as core::ffi::c_int >> 1) as u32,
        );
        n_0 = n_0.wrapping_add(1);
    }
    if weightTotal == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let tableLog = (BITv07_highbit32(weightTotal)).wrapping_add(1);
    if tableLog > HUFv07_TABLELOG_ABSOLUTEMAX as u32 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    *tableLogPtr = tableLog;
    let total = ((1) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1) << BITv07_highbit32(rest)) as u32;
    let lastWeight = (BITv07_highbit32(rest)).wrapping_add(1);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    *huffWeight.add(oSize) = lastWeight as u8;
    let fresh3 = &mut (*rankStats.offset(lastWeight as isize));
    *fresh3 = (*fresh3).wrapping_add(1);
    if *rankStats.offset(1) < 2 || *rankStats.offset(1) & 1 != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    iSize.wrapping_add(1)
}
pub const FSEv07_isError_1: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub unsafe fn FSEv07_createDTable(mut tableLog: core::ffi::c_uint) -> *mut FSEv07_DTable {
    if tableLog > FSEv07_TABLELOG_ABSOLUTE_MAX as core::ffi::c_uint {
        tableLog = FSEv07_TABLELOG_ABSOLUTE_MAX as core::ffi::c_uint;
    }
    malloc(
        ((1 + ((1) << tableLog)) as size_t).wrapping_mul(::core::mem::size_of::<u32>() as size_t),
    ) as *mut FSEv07_DTable
}
pub unsafe fn FSEv07_freeDTable(dt: *mut FSEv07_DTable) {
    free(dt as *mut core::ffi::c_void);
}
pub unsafe fn FSEv07_buildDTable(
    dt: *mut FSEv07_DTable,
    normalizedCounter: *const core::ffi::c_short,
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
) -> size_t {
    let tdPtr = dt.offset(1) as *mut core::ffi::c_void;
    let tableDecode = tdPtr as *mut FSEv07_decode_t;
    let mut symbolNext: [u16; 256] = [0; 256];
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    if maxSymbolValue > FSEv07_MAX_SYMBOL_VALUE as core::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as core::ffi::c_int) as size_t;
    }
    if tableLog > FSEv07_MAX_TABLELOG as core::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    let mut DTableH = FSEv07_DTableHeader {
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
            let fresh4 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh4 as isize)).symbol = s as u8;
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
        &mut DTableH as *mut FSEv07_DTableHeader as *const core::ffi::c_void,
        ::core::mem::size_of::<FSEv07_DTableHeader>() as size_t,
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
        let fresh5 = &mut (*symbolNext.as_mut_ptr().offset(symbol as isize));
        let fresh6 = *fresh5;
        *fresh5 = (*fresh5).wrapping_add(1);
        let nextState = fresh6;
        (*tableDecode.offset(u as isize)).nbBits =
            tableLog.wrapping_sub(BITv07_highbit32(nextState as u32)) as u8;
        (*tableDecode.offset(u as isize)).newState = (((nextState as core::ffi::c_int)
            << (*tableDecode.offset(u as isize)).nbBits as core::ffi::c_int)
            as u32)
            .wrapping_sub(tableSize) as u16;
        u = u.wrapping_add(1);
    }
    0
}
pub unsafe fn FSEv07_buildDTable_rle(dt: *mut FSEv07_DTable, symbolValue: u8) -> size_t {
    let ptr = dt as *mut core::ffi::c_void;
    let DTableH = ptr as *mut FSEv07_DTableHeader;
    let dPtr = dt.offset(1) as *mut core::ffi::c_void;
    let cell = dPtr as *mut FSEv07_decode_t;
    (*DTableH).tableLog = 0;
    (*DTableH).fastMode = 0;
    (*cell).newState = 0;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0;
    0
}
pub unsafe fn FSEv07_buildDTable_raw(dt: *mut FSEv07_DTable, nbBits: core::ffi::c_uint) -> size_t {
    let ptr = dt as *mut core::ffi::c_void;
    let DTableH = ptr as *mut FSEv07_DTableHeader;
    let dPtr = dt.offset(1) as *mut core::ffi::c_void;
    let dinfo = dPtr as *mut FSEv07_decode_t;
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
unsafe fn FSEv07_decompress_usingDTable_generic(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    dt: *const FSEv07_DTable,
    fast: core::ffi::c_uint,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.add(maxDstSize);
    let olimit = omax.offset(-(3));
    let mut bitD = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut state1 = FSEv07_DState_t {
        state: 0,
        table: core::ptr::null::<core::ffi::c_void>(),
    };
    let mut state2 = FSEv07_DState_t {
        state: 0,
        table: core::ptr::null::<core::ffi::c_void>(),
    };
    let errorCode = BITv07_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv07_initDState(&mut state1, &mut bitD, dt);
    FSEv07_initDState(&mut state2, &mut bitD, dt);
    while BITv07_reloadDStream(&mut bitD) as core::ffi::c_uint
        == BITv07_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && op < olimit
    {
        *op.offset(0) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
        {
            BITv07_reloadDStream(&mut bitD);
        }
        *op.offset(1) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 4 + 7) as size_t
            > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
            && BITv07_reloadDStream(&mut bitD) as core::ffi::c_uint
                > BITv07_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        {
            op = op.offset(2);
            break;
        }
        *op.offset(2) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
        {
            BITv07_reloadDStream(&mut bitD);
        }
        *op.offset(3) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        op = op.offset(4);
    }
    loop {
        if op > omax.offset(-(2)) {
            return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
        }
        let fresh7 = op;
        op = op.offset(1);
        *fresh7 = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if BITv07_reloadDStream(&mut bitD) as core::ffi::c_uint
            == BITv07_DStream_overflow as core::ffi::c_int as core::ffi::c_uint
        {
            let fresh8 = op;
            op = op.offset(1);
            *fresh8 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
            } else {
                FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
            }) as u8;
            break;
        } else {
            if op > omax.offset(-(2)) {
                return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
            }
            let fresh9 = op;
            op = op.offset(1);
            *fresh9 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
            } else {
                FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
            }) as u8;
            if BITv07_reloadDStream(&mut bitD) as core::ffi::c_uint
                != BITv07_DStream_overflow as core::ffi::c_int as core::ffi::c_uint
            {
                continue;
            }
            let fresh10 = op;
            op = op.offset(1);
            *fresh10 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
            } else {
                FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
            }) as u8;
            break;
        }
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
pub unsafe fn FSEv07_decompress_usingDTable(
    dst: *mut core::ffi::c_void,
    originalSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    dt: *const FSEv07_DTable,
) -> size_t {
    let ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const FSEv07_DTableHeader;
    let fastMode = (*DTableH).fastMode as u32;
    if fastMode != 0 {
        return FSEv07_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 1);
    }
    FSEv07_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 0)
}
pub unsafe fn FSEv07_decompress(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let istart = cSrc as *const u8;
    let mut ip = istart;
    let mut counting: [core::ffi::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: core::ffi::c_uint = 0;
    let mut maxSymbolValue = FSEv07_MAX_SYMBOL_VALUE as core::ffi::c_uint;
    if cSrcSize < 2 {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let NCountLength = FSEv07_readNCount(
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
    let errorCode = FSEv07_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv07_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    )
}
unsafe fn HUFv07_getDTableDesc(table: *const HUFv07_DTable) -> DTableDesc {
    let mut dtd = DTableDesc {
        maxTableLog: 0,
        tableType: 0,
        tableLog: 0,
        reserved: 0,
    };
    memcpy(
        &mut dtd as *mut DTableDesc as *mut core::ffi::c_void,
        table as *const core::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as size_t,
    );
    dtd
}
pub unsafe fn HUFv07_readDTableX2(
    DTable: *mut HUFv07_DTable,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 17] = [0; 17];
    let mut tableLog = 0;
    let mut nbSymbols = 0;
    let mut iSize: size_t = 0;
    let dtPtr = DTable.offset(1) as *mut core::ffi::c_void;
    let dt = dtPtr as *mut HUFv07_DEltX2;
    iSize = HUFv07_readStats(
        huffWeight.as_mut_ptr(),
        (HUFv07_SYMBOLVALUE_MAX + 1) as size_t,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv07_isError(iSize) != 0 {
        return iSize;
    }
    let mut dtd = HUFv07_getDTableDesc(DTable);
    if tableLog > (dtd.maxTableLog as core::ffi::c_int + 1) as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    dtd.tableType = 0;
    dtd.tableLog = tableLog as u8;
    memcpy(
        DTable as *mut core::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const core::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as size_t,
    );
    let mut n: u32 = 0;
    let mut nextRankStart = 0u32;
    n = 1;
    while n < tableLog.wrapping_add(1) {
        let current = nextRankStart;
        nextRankStart = nextRankStart
            .wrapping_add(*rankVal.as_mut_ptr().offset(n as isize) << n.wrapping_sub(1));
        *rankVal.as_mut_ptr().offset(n as isize) = current;
        n = n.wrapping_add(1);
    }
    let mut n_0: u32 = 0;
    n_0 = 0;
    while n_0 < nbSymbols {
        let w = *huffWeight.as_mut_ptr().offset(n_0 as isize) as u32;
        let length = ((1) << w >> 1) as u32;
        let mut i: u32 = 0;
        let mut D = HUFv07_DEltX2 { byte: 0, nbBits: 0 };
        D.byte = n_0 as u8;
        D.nbBits = tableLog.wrapping_add(1).wrapping_sub(w) as u8;
        i = *rankVal.as_mut_ptr().offset(w as isize);
        while i < (*rankVal.as_mut_ptr().offset(w as isize)).wrapping_add(length) {
            *dt.offset(i as isize) = D;
            i = i.wrapping_add(1);
        }
        let fresh11 = &mut (*rankVal.as_mut_ptr().offset(w as isize));
        *fresh11 = (*fresh11).wrapping_add(length);
        n_0 = n_0.wrapping_add(1);
    }
    iSize
}
unsafe fn HUFv07_decodeSymbolX2(
    Dstream: *mut BITv07_DStream_t,
    dt: *const HUFv07_DEltX2,
    dtLog: u32,
) -> u8 {
    let val = BITv07_lookBitsFast(Dstream, dtLog);
    let c = (*dt.add(val)).byte;
    BITv07_skipBits(Dstream, (*dt.add(val)).nbBits as u32);
    c
}
#[inline]
unsafe fn HUFv07_decodeStreamX2(
    mut p: *mut u8,
    bitDPtr: *mut BITv07_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv07_DEltX2,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv07_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv07_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p <= pEnd.offset(-(4))
    {
        if MEM_64bits() != 0 {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh14 = p;
            p = p.offset(1);
            *fresh14 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while BITv07_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv07_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p < pEnd
    {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    pEnd.offset_from(pStart) as core::ffi::c_long as size_t
}
unsafe fn HUFv07_decompress1X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let op = dst as *mut u8;
    let oend = op.add(dstSize);
    let dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX2;
    let mut bitD = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let dtd = HUFv07_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let errorCode = BITv07_initDStream(&mut bitD, cSrc, cSrcSize);
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv07_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if BITv07_endOfDStream(&bitD) == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv07_decompress1X2_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 0 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    HUFv07_decompress1X2_usingDTable_internal(dst, dstSize, cSrc, cSrcSize, DTable)
}
pub unsafe fn HUFv07_decompress1X2_DCtx(
    DCtx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUFv07_readDTableX2(DCtx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(hSize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv07_decompress1X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DCtx,
    )
}
pub unsafe fn HUFv07_decompress1X2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
    let mut DTable: [HUFv07_DTable; 2049] =
        [(12 - 1) as u32 * 0x1000001 as core::ffi::c_int as u32; 2049];
    HUFv07_decompress1X2_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize)
}
unsafe fn HUFv07_decompress4X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX2;
    let mut bitD1 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD2 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD3 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD4 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const core::ffi::c_void) as size_t;
    let length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
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
    let dtd = HUFv07_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let errorCode = BITv07_initDStream(&mut bitD1, istart1 as *const core::ffi::c_void, length1);
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    let errorCode_0 = BITv07_initDStream(&mut bitD2, istart2 as *const core::ffi::c_void, length2);
    if HUFv07_isError(errorCode_0) != 0 {
        return errorCode_0;
    }
    let errorCode_1 = BITv07_initDStream(&mut bitD3, istart3 as *const core::ffi::c_void, length3);
    if HUFv07_isError(errorCode_1) != 0 {
        return errorCode_1;
    }
    let errorCode_2 = BITv07_initDStream(&mut bitD4, istart4 as *const core::ffi::c_void, length4);
    if HUFv07_isError(errorCode_2) != 0 {
        return errorCode_2;
    }
    endSignal = BITv07_reloadDStream(&mut bitD1) as core::ffi::c_uint
        | BITv07_reloadDStream(&mut bitD2) as core::ffi::c_uint
        | BITv07_reloadDStream(&mut bitD3) as core::ffi::c_uint
        | BITv07_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    while endSignal == BITv07_DStream_unfinished as core::ffi::c_int as u32
        && op4 < oend.offset(-(7))
    {
        if MEM_64bits() != 0 {
            let fresh18 = op1;
            op1 = op1.offset(1);
            *fresh18 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh19 = op2;
            op2 = op2.offset(1);
            *fresh19 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh20 = op3;
            op3 = op3.offset(1);
            *fresh20 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh21 = op4;
            op4 = op4.offset(1);
            *fresh21 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            let fresh22 = op1;
            op1 = op1.offset(1);
            *fresh22 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            let fresh23 = op2;
            op2 = op2.offset(1);
            *fresh23 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            let fresh24 = op3;
            op3 = op3.offset(1);
            *fresh24 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            let fresh25 = op4;
            op4 = op4.offset(1);
            *fresh25 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh26 = op1;
            op1 = op1.offset(1);
            *fresh26 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh27 = op2;
            op2 = op2.offset(1);
            *fresh27 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh28 = op3;
            op3 = op3.offset(1);
            *fresh28 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh29 = op4;
            op4 = op4.offset(1);
            *fresh29 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh30 = op1;
        op1 = op1.offset(1);
        *fresh30 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh31 = op2;
        op2 = op2.offset(1);
        *fresh31 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh32 = op3;
        op3 = op3.offset(1);
        *fresh32 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh33 = op4;
        op4 = op4.offset(1);
        *fresh33 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        endSignal = BITv07_reloadDStream(&mut bitD1) as core::ffi::c_uint
            | BITv07_reloadDStream(&mut bitD2) as core::ffi::c_uint
            | BITv07_reloadDStream(&mut bitD3) as core::ffi::c_uint
            | BITv07_reloadDStream(&mut bitD4) as core::ffi::c_uint;
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
    HUFv07_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv07_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv07_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv07_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv07_endOfDStream(&bitD1)
        & BITv07_endOfDStream(&bitD2)
        & BITv07_endOfDStream(&bitD3)
        & BITv07_endOfDStream(&bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv07_decompress4X2_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 0 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    HUFv07_decompress4X2_usingDTable_internal(dst, dstSize, cSrc, cSrcSize, DTable)
}
pub unsafe fn HUFv07_decompress4X2_DCtx(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUFv07_readDTableX2(dctx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(hSize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv07_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dctx,
    )
}
pub unsafe fn HUFv07_decompress4X2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
    let mut DTable: [HUFv07_DTable; 2049] =
        [(12 - 1) as u32 * 0x1000001 as core::ffi::c_int as u32; 2049];
    HUFv07_decompress4X2_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize)
}
unsafe fn HUFv07_fillDTableX4Level2(
    DTable: *mut HUFv07_DEltX4,
    sizeLog: u32,
    consumed: u32,
    rankValOrigin: *const u32,
    minWeight: core::ffi::c_int,
    sortedSymbols: *const sortedSymbol_t,
    sortedListSize: u32,
    nbBitsBaseline: u32,
    baseSeq: u16,
) {
    let mut DElt = HUFv07_DEltX4 {
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
        let skipSize = *rankVal.as_mut_ptr().offset(minWeight as isize);
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
unsafe fn HUFv07_fillDTableX4(
    DTable: *mut HUFv07_DEltX4,
    targetLog: u32,
    sortedList: *const sortedSymbol_t,
    sortedListSize: u32,
    rankStart: *const u32,
    rankValOrigin: *mut [u32; 17],
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
            HUFv07_fillDTableX4Level2(
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
            let mut DElt = HUFv07_DEltX4 {
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
pub unsafe fn HUFv07_readDTableX4(
    DTable: *mut HUFv07_DTable,
    src: *const core::ffi::c_void,
    srcSize: size_t,
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
    let mut dtd = HUFv07_getDTableDesc(DTable);
    let maxTableLog = dtd.maxTableLog as u32;
    let mut iSize: size_t = 0;
    let dtPtr = DTable.offset(1) as *mut core::ffi::c_void;
    let dt = dtPtr as *mut HUFv07_DEltX4;
    if maxTableLog > HUFv07_TABLELOG_ABSOLUTEMAX as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    iSize = HUFv07_readStats(
        weightList.as_mut_ptr(),
        (HUFv07_SYMBOLVALUE_MAX + 1) as size_t,
        rankStats.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv07_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > maxTableLog {
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
        let current = nextRankStart;
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
    let rescale = maxTableLog.wrapping_sub(tableLog).wrapping_sub(1) as core::ffi::c_int;
    let mut nextRankVal = 0u32;
    let mut w_1: u32 = 0;
    w_1 = 1;
    while w_1 < maxW.wrapping_add(1) {
        let current_0 = nextRankVal;
        nextRankVal = nextRankVal.wrapping_add(
            *rankStats.as_mut_ptr().offset(w_1 as isize) << w_1.wrapping_add(rescale as u32),
        );
        *rankVal0.offset(w_1 as isize) = current_0;
        w_1 = w_1.wrapping_add(1);
    }
    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut consumed: u32 = 0;
    consumed = minBits;
    while consumed < maxTableLog.wrapping_sub(minBits).wrapping_add(1) {
        let rankValPtr = (*rankVal.as_mut_ptr().offset(consumed as isize)).as_mut_ptr();
        let mut w_2: u32 = 0;
        w_2 = 1;
        while w_2 < maxW.wrapping_add(1) {
            *rankValPtr.offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
        }
        consumed = consumed.wrapping_add(1);
    }
    HUFv07_fillDTableX4(
        dt,
        maxTableLog,
        sortedSymbol.as_mut_ptr(),
        sizeOfSort,
        rankStart0.as_mut_ptr(),
        rankVal.as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1),
    );
    dtd.tableLog = maxTableLog as u8;
    dtd.tableType = 1;
    memcpy(
        DTable as *mut core::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const core::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as size_t,
    );
    iSize
}
unsafe fn HUFv07_decodeSymbolX4(
    op: *mut core::ffi::c_void,
    DStream: *mut BITv07_DStream_t,
    dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv07_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.add(val) as *const core::ffi::c_void, 2);
    BITv07_skipBits(DStream, (*dt.add(val)).nbBits as u32);
    (*dt.add(val)).length as u32
}
unsafe fn HUFv07_decodeLastSymbolX4(
    op: *mut core::ffi::c_void,
    DStream: *mut BITv07_DStream_t,
    dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv07_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.add(val) as *const core::ffi::c_void, 1);
    if (*dt.add(val)).length as core::ffi::c_int == 1 {
        BITv07_skipBits(DStream, (*dt.add(val)).nbBits as u32);
    } else if ((*DStream).bitsConsumed as size_t)
        < (::core::mem::size_of::<size_t>() as size_t).wrapping_mul(8)
    {
        BITv07_skipBits(DStream, (*dt.add(val)).nbBits as u32);
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
unsafe fn HUFv07_decodeStreamX4(
    mut p: *mut u8,
    bitDPtr: *mut BITv07_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv07_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv07_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p < pEnd.offset(-(7))
    {
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            p = p.offset(
                HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        p = p.offset(
            HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while BITv07_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv07_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p <= pEnd.offset(-(2))
    {
        p = p.offset(
            HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while p <= pEnd.offset(-(2)) {
        p = p.offset(
            HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    if p < pEnd {
        p = p.offset(
            HUFv07_decodeLastSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as core::ffi::c_long as size_t
}
unsafe fn HUFv07_decompress1X4_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let mut bitD = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let errorCode = BITv07_initDStream(&mut bitD, cSrc, cSrcSize);
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX4;
    let dtd = HUFv07_getDTableDesc(DTable);
    HUFv07_decodeStreamX4(ostart, &mut bitD, oend, dt, dtd.tableLog as u32);
    if BITv07_endOfDStream(&bitD) == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv07_decompress1X4_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 1 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    HUFv07_decompress1X4_usingDTable_internal(dst, dstSize, cSrc, cSrcSize, DTable)
}
pub unsafe fn HUFv07_decompress1X4_DCtx(
    DCtx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUFv07_readDTableX4(DCtx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(hSize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv07_decompress1X4_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DCtx,
    )
}
pub unsafe fn HUFv07_decompress1X4(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
    let mut DTable: [HUFv07_DTable; 4097] = [12 * 0x1000001 as core::ffi::c_int as u32; 4097];
    HUFv07_decompress1X4_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize)
}
unsafe fn HUFv07_decompress4X4_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX4;
    let mut bitD1 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD2 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD3 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut bitD4 = BITv07_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: core::ptr::null::<core::ffi::c_char>(),
        start: core::ptr::null::<core::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const core::ffi::c_void) as size_t;
    let length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
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
    let dtd = HUFv07_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let errorCode = BITv07_initDStream(&mut bitD1, istart1 as *const core::ffi::c_void, length1);
    if HUFv07_isError(errorCode) != 0 {
        return errorCode;
    }
    let errorCode_0 = BITv07_initDStream(&mut bitD2, istart2 as *const core::ffi::c_void, length2);
    if HUFv07_isError(errorCode_0) != 0 {
        return errorCode_0;
    }
    let errorCode_1 = BITv07_initDStream(&mut bitD3, istart3 as *const core::ffi::c_void, length3);
    if HUFv07_isError(errorCode_1) != 0 {
        return errorCode_1;
    }
    let errorCode_2 = BITv07_initDStream(&mut bitD4, istart4 as *const core::ffi::c_void, length4);
    if HUFv07_isError(errorCode_2) != 0 {
        return errorCode_2;
    }
    endSignal = BITv07_reloadDStream(&mut bitD1) as core::ffi::c_uint
        | BITv07_reloadDStream(&mut bitD2) as core::ffi::c_uint
        | BITv07_reloadDStream(&mut bitD3) as core::ffi::c_uint
        | BITv07_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    while endSignal == BITv07_DStream_unfinished as core::ffi::c_int as u32
        && op4 < oend.offset(-(7))
    {
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv07_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv07_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv07_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv07_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            op1 = op1.offset(HUFv07_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            op2 = op2.offset(HUFv07_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            op3 = op3.offset(HUFv07_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv07_TABLELOG_MAX <= 12 {
            op4 = op4.offset(HUFv07_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv07_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv07_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv07_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv07_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        op1 = op1.offset(
            HUFv07_decodeSymbolX4(op1 as *mut core::ffi::c_void, &mut bitD1, dt, dtLog) as isize,
        );
        op2 = op2.offset(
            HUFv07_decodeSymbolX4(op2 as *mut core::ffi::c_void, &mut bitD2, dt, dtLog) as isize,
        );
        op3 = op3.offset(
            HUFv07_decodeSymbolX4(op3 as *mut core::ffi::c_void, &mut bitD3, dt, dtLog) as isize,
        );
        op4 = op4.offset(
            HUFv07_decodeSymbolX4(op4 as *mut core::ffi::c_void, &mut bitD4, dt, dtLog) as isize,
        );
        endSignal = BITv07_reloadDStream(&mut bitD1) as core::ffi::c_uint
            | BITv07_reloadDStream(&mut bitD2) as core::ffi::c_uint
            | BITv07_reloadDStream(&mut bitD3) as core::ffi::c_uint
            | BITv07_reloadDStream(&mut bitD4) as core::ffi::c_uint;
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
    HUFv07_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv07_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv07_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv07_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BITv07_endOfDStream(&bitD1)
        & BITv07_endOfDStream(&bitD2)
        & BITv07_endOfDStream(&bitD3)
        & BITv07_endOfDStream(&bitD4);
    if endCheck == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}
pub unsafe fn HUFv07_decompress4X4_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 1 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    HUFv07_decompress4X4_usingDTable_internal(dst, dstSize, cSrc, cSrcSize, DTable)
}
pub unsafe fn HUFv07_decompress4X4_DCtx(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUFv07_readDTableX4(dctx, cSrc, cSrcSize);
    if HUFv07_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.add(hSize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv07_decompress4X4_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dctx,
    )
}
pub unsafe fn HUFv07_decompress4X4(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
    let mut DTable: [HUFv07_DTable; 4097] = [12 * 0x1000001 as core::ffi::c_int as u32; 4097];
    HUFv07_decompress4X4_DCtx(DTable.as_mut_ptr(), dst, dstSize, cSrc, cSrcSize)
}
pub unsafe fn HUFv07_decompress1X_usingDTable(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 0 {
        HUFv07_decompress1X4_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable)
    } else {
        HUFv07_decompress1X2_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable)
    }
}
pub unsafe fn HUFv07_decompress4X_usingDTable(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUFv07_DTable,
) -> size_t {
    let dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 0 {
        HUFv07_decompress4X4_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable)
    } else {
        HUFv07_decompress4X2_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable)
    }
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
pub unsafe fn HUFv07_selectDecoder(dstSize: size_t, cSrcSize: size_t) -> u32 {
    let Q = (cSrcSize * 16 / dstSize) as u32;
    let D256 = (dstSize >> 8) as u32;
    let DTime0 = ((*(*algoTime.as_ptr().offset(Q as isize)).as_ptr().offset(0)).tableTime)
        .wrapping_add(
            (*(*algoTime.as_ptr().offset(Q as isize)).as_ptr().offset(0)).decode256Time * D256,
        );
    let mut DTime1 = ((*(*algoTime.as_ptr().offset(Q as isize)).as_ptr().offset(1)).tableTime)
        .wrapping_add(
            (*(*algoTime.as_ptr().offset(Q as isize)).as_ptr().offset(1)).decode256Time * D256,
        );
    DTime1 = DTime1.wrapping_add(DTime1 >> 3);
    (DTime1 < DTime0) as core::ffi::c_int as u32
}
pub unsafe fn HUFv07_decompress(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
    static decompress: [decompressionAlgo; 2] = [
        Some(
            HUFv07_decompress4X2
                as unsafe fn(
                    *mut core::ffi::c_void,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                ) -> size_t,
        ),
        Some(
            HUFv07_decompress4X4
                as unsafe fn(
                    *mut core::ffi::c_void,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                ) -> size_t,
        ),
    ];
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
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    (*decompress.as_ptr().offset(algoNb as isize)).unwrap_unchecked()(dst, dstSize, cSrc, cSrcSize)
}
pub unsafe fn HUFv07_decompress4X_DCtx(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
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
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    if algoNb != 0 {
        HUFv07_decompress4X4_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    } else {
        HUFv07_decompress4X2_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    }
}
pub unsafe fn HUFv07_decompress4X_hufOnly(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
    if dstSize == 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if cSrcSize >= dstSize || cSrcSize <= 1 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    if algoNb != 0 {
        HUFv07_decompress4X4_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    } else {
        HUFv07_decompress4X2_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    }
}
pub unsafe fn HUFv07_decompress1X_DCtx(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
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
    let algoNb = HUFv07_selectDecoder(dstSize, cSrcSize);
    if algoNb != 0 {
        HUFv07_decompress1X4_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    } else {
        HUFv07_decompress1X2_DCtx(dctx, dst, dstSize, cSrc, cSrcSize)
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_isError))]
pub unsafe extern "C" fn ZSTDv07_isError_0(code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_getErrorName))]
pub unsafe extern "C" fn ZSTDv07_getErrorName(code: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(code)
}
pub unsafe fn ZBUFFv07_isError(errorCode: size_t) -> core::ffi::c_uint {
    ERR_isError(errorCode)
}
pub unsafe fn ZBUFFv07_getErrorName(errorCode: size_t) -> *const core::ffi::c_char {
    ERR_getErrorName(errorCode)
}
unsafe extern "C" fn ZSTDv07_defaultAllocFunction(
    _opaque: *mut core::ffi::c_void,
    size: size_t,
) -> *mut core::ffi::c_void {
    malloc(size)
}
unsafe extern "C" fn ZSTDv07_defaultFreeFunction(
    _opaque: *mut core::ffi::c_void,
    address: *mut core::ffi::c_void,
) {
    free(address);
}
pub const ZSTDv07_DICT_MAGIC: core::ffi::c_uint = 0xec30a437 as core::ffi::c_uint;
pub const ZSTDv07_REP_NUM: core::ffi::c_int = 3;
pub const ZSTDv07_REP_INIT: core::ffi::c_int = 3;
static repStartValue: [u32; 3] = [1, 4, 8];
pub const ZSTDv07_WINDOWLOG_ABSOLUTEMIN: core::ffi::c_int = 10;
static ZSTDv07_fcs_fieldSize: [size_t; 4] = [0, 2, 4, 8];
static ZSTDv07_did_fieldSize: [size_t; 4] = [0, 1, 2, 4];
pub const ZSTDv07_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTDv07_blockHeaderSize: size_t = ZSTDv07_BLOCKHEADERSIZE as size_t;
pub const MIN_SEQUENCES_SIZE: core::ffi::c_int = 1;
pub const MIN_CBLOCK_SIZE: core::ffi::c_int = 1 + 1 + MIN_SEQUENCES_SIZE;
pub const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
pub const MINMATCH: core::ffi::c_int = 3;
pub const MaxML: core::ffi::c_int = 52;
pub const MaxLL: core::ffi::c_int = 35;
pub const MaxOff: core::ffi::c_int = 28;
pub const MLFSELog: core::ffi::c_int = 9;
pub const LLFSELog: core::ffi::c_int = 9;
pub const OffFSELog: core::ffi::c_int = 8;
pub const FSEv07_ENCODING_RAW: core::ffi::c_int = 0;
pub const FSEv07_ENCODING_RLE: core::ffi::c_int = 1;
pub const FSEv07_ENCODING_STATIC: core::ffi::c_int = 2;
pub const FSEv07_ENCODING_DYNAMIC: core::ffi::c_int = 3;
pub const ZSTD_CONTENTSIZE_ERROR: core::ffi::c_ulonglong =
    (0 as core::ffi::c_ulonglong).wrapping_sub(2);
static LL_bits: [u32; 36] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16,
];
static LL_defaultNorm: [i16; 36] = [
    4,
    3,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    1,
    1,
    1,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    3,
    2,
    1,
    1,
    1,
    1,
    1,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
];
static LL_defaultNormLog: u32 = 6;
static ML_bits: [u32; 53] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
];
static ML_defaultNorm: [i16; 53] = [
    1,
    4,
    3,
    2,
    2,
    2,
    2,
    2,
    2,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
];
static ML_defaultNormLog: u32 = 6;
static OF_defaultNorm: [i16; 29] = [
    1,
    1,
    1,
    1,
    1,
    1,
    2,
    2,
    2,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
    -(1) as i16,
];
static OF_defaultNormLog: u32 = 5;
unsafe fn ZSTDv07_copy8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    memcpy(dst, src, 8);
}
pub const WILDCOPY_OVERLENGTH: core::ffi::c_int = 8;
#[inline]
unsafe fn ZSTDv07_wildcopy(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    length: ptrdiff_t,
) {
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    loop {
        ZSTDv07_copy8(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
        op = op.offset(8);
        ip = ip.offset(8);
        if op >= oend {
            break;
        }
    }
}
static mut defaultCustomMem: ZSTDv07_customMem = ZSTDv07_customMem {
    customAlloc: Some(
        ZSTDv07_defaultAllocFunction
            as unsafe extern "C" fn(*mut core::ffi::c_void, size_t) -> *mut core::ffi::c_void,
    ),
    customFree: Some(
        ZSTDv07_defaultFreeFunction
            as unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> (),
    ),
    opaque: NULL as *mut core::ffi::c_void,
};
pub const ZSTDv07_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const FSEv07_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const HUFv07_isError_0: fn(size_t) -> core::ffi::c_uint = ERR_isError;
unsafe extern "C" fn ZSTDv07_copy4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    memcpy(dst, src, 4);
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_sizeofDCtx))]
pub unsafe extern "C" fn ZSTDv07_sizeofDCtx(_dctx: *const ZSTDv07_DCtx) -> size_t {
    ::core::mem::size_of::<ZSTDv07_DCtx>()
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_estimateDCtxSize))]
pub unsafe extern "C" fn ZSTDv07_estimateDCtxSize() -> size_t {
    ::core::mem::size_of::<ZSTDv07_DCtx>()
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompressBegin))]
pub unsafe extern "C" fn ZSTDv07_decompressBegin(dctx: *mut ZSTDv07_DCtx) -> size_t {
    (*dctx).expected = ZSTDv07_frameHeaderSize_min;
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).previousDstEnd = NULL as *const core::ffi::c_void;
    (*dctx).base = NULL as *const core::ffi::c_void;
    (*dctx).vBase = NULL as *const core::ffi::c_void;
    (*dctx).dictEnd = NULL as *const core::ffi::c_void;
    *((*dctx).hufTable).as_mut_ptr().offset(0) =
        (12 * 0x1000001 as core::ffi::c_int) as HUFv07_DTable;
    (*dctx).fseEntropy = 0;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    (*dctx).dictID = 0;
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < ZSTDv07_REP_NUM {
        *((*dctx).rep).as_mut_ptr().offset(i as isize) = *repStartValue.as_ptr().offset(i as isize);
        i += 1;
    }
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_createDCtx_advanced))]
pub unsafe extern "C" fn ZSTDv07_createDCtx_advanced(
    mut customMem: ZSTDv07_customMem,
) -> *mut ZSTDv07_DCtx {
    let mut dctx = core::ptr::null_mut::<ZSTDv07_DCtx>();
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return NULL as *mut ZSTDv07_DCtx;
    }
    dctx = (customMem.customAlloc).unwrap_unchecked()(
        customMem.opaque,
        ::core::mem::size_of::<ZSTDv07_DCtx>() as size_t,
    ) as *mut ZSTDv07_DCtx;
    if dctx.is_null() {
        return NULL as *mut ZSTDv07_DCtx;
    }
    memcpy(
        &mut (*dctx).customMem as *mut ZSTDv07_customMem as *mut core::ffi::c_void,
        &mut customMem as *mut ZSTDv07_customMem as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTDv07_customMem>() as size_t,
    );
    ZSTDv07_decompressBegin(dctx);
    dctx
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_createDCtx))]
pub unsafe extern "C" fn ZSTDv07_createDCtx() -> *mut ZSTDv07_DCtx {
    ZSTDv07_createDCtx_advanced(defaultCustomMem)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_freeDCtx))]
pub unsafe extern "C" fn ZSTDv07_freeDCtx(dctx: *mut ZSTDv07_DCtx) -> size_t {
    if dctx.is_null() {
        return 0;
    }
    ((*dctx).customMem.customFree).unwrap_unchecked()(
        (*dctx).customMem.opaque,
        dctx as *mut core::ffi::c_void,
    );
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_copyDCtx))]
pub unsafe extern "C" fn ZSTDv07_copyDCtx(
    dstDCtx: *mut ZSTDv07_DCtx,
    srcDCtx: *const ZSTDv07_DCtx,
) {
    memcpy(
        dstDCtx as *mut core::ffi::c_void,
        srcDCtx as *const core::ffi::c_void,
        (::core::mem::size_of::<ZSTDv07_DCtx>() as size_t).wrapping_sub(
            ((ZSTDv07_BLOCKSIZE_ABSOLUTEMAX + WILDCOPY_OVERLENGTH) as size_t)
                .wrapping_add(ZSTDv07_frameHeaderSize_max),
        ),
    );
}
unsafe fn ZSTDv07_frameHeaderSize(src: *const core::ffi::c_void, srcSize: size_t) -> size_t {
    if srcSize < ZSTDv07_frameHeaderSize_min {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let fhd = *(src as *const u8).offset(4);
    let dictID = (fhd as core::ffi::c_int & 3) as u32;
    let directMode = (fhd as core::ffi::c_int >> 5 & 1) as u32;
    let fcsId = (fhd as core::ffi::c_int >> 6) as u32;
    ZSTDv07_frameHeaderSize_min
        .wrapping_add((directMode == 0) as core::ffi::c_int as size_t)
        .wrapping_add(*ZSTDv07_did_fieldSize.as_ptr().offset(dictID as isize))
        .wrapping_add(*ZSTDv07_fcs_fieldSize.as_ptr().offset(fcsId as isize))
        .wrapping_add(
            (directMode != 0 && *ZSTDv07_fcs_fieldSize.as_ptr().offset(fcsId as isize) == 0)
                as core::ffi::c_int as size_t,
        )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_getFrameParams))]
pub unsafe extern "C" fn ZSTDv07_getFrameParams(
    fparamsPtr: *mut ZSTDv07_frameParams,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let ip = src as *const u8;
    if srcSize < ZSTDv07_frameHeaderSize_min {
        return ZSTDv07_frameHeaderSize_min;
    }
    ptr::write_bytes(
        fparamsPtr as *mut u8,
        0,
        ::core::mem::size_of::<ZSTDv07_frameParams>(),
    );
    if MEM_readLE32(src) != ZSTDv07_MAGICNUMBER {
        if MEM_readLE32(src) & 0xfffffff0 as core::ffi::c_uint == ZSTDv07_MAGIC_SKIPPABLE_START {
            if srcSize < ZSTDv07_skippableHeaderSize {
                return ZSTDv07_skippableHeaderSize;
            }
            (*fparamsPtr).frameContentSize = MEM_readLE32(
                (src as *const core::ffi::c_char).offset(4) as *const core::ffi::c_void,
            ) as core::ffi::c_ulonglong;
            (*fparamsPtr).windowSize = 0;
            return 0;
        }
        return -(ZSTD_error_prefix_unknown as core::ffi::c_int) as size_t;
    }
    let fhsize = ZSTDv07_frameHeaderSize(src, srcSize);
    if srcSize < fhsize {
        return fhsize;
    }
    let fhdByte = *ip.offset(4);
    let mut pos = 5 as size_t;
    let dictIDSizeCode = (fhdByte as core::ffi::c_int & 3) as u32;
    let checksumFlag = (fhdByte as core::ffi::c_int >> 2 & 1) as u32;
    let directMode = (fhdByte as core::ffi::c_int >> 5 & 1) as u32;
    let fcsID = (fhdByte as core::ffi::c_int >> 6) as u32;
    let windowSizeMax = (1)
        << (if MEM_32bits() != 0 {
            ZSTDv07_WINDOWLOG_MAX_32
        } else {
            ZSTDv07_WINDOWLOG_MAX_64
        }) as u32;
    let mut windowSize = 0u32;
    let mut dictID = 0;
    let mut frameContentSize = 0;
    if fhdByte as core::ffi::c_int & 0x8 as core::ffi::c_int != 0 {
        return -(ZSTD_error_frameParameter_unsupported as core::ffi::c_int) as size_t;
    }
    if directMode == 0 {
        let fresh39 = pos;
        pos = pos.wrapping_add(1);
        let wlByte = *ip.add(fresh39);
        let windowLog = ((wlByte as core::ffi::c_int >> 3) + ZSTDv07_WINDOWLOG_ABSOLUTEMIN) as u32;
        if windowLog
            > (if MEM_32bits() != 0 {
                ZSTDv07_WINDOWLOG_MAX_32
            } else {
                ZSTDv07_WINDOWLOG_MAX_64
            }) as u32
        {
            return -(ZSTD_error_frameParameter_unsupported as core::ffi::c_int) as size_t;
        }
        windowSize = (1) << windowLog;
        windowSize =
            windowSize.wrapping_add((windowSize >> 3) * (wlByte as core::ffi::c_int & 7) as u32);
    }
    match dictIDSizeCode {
        1 => {
            dictID = *ip.add(pos) as u32;
            pos = pos.wrapping_add(1);
        }
        2 => {
            dictID = MEM_readLE16(ip.add(pos) as *const core::ffi::c_void) as u32;
            pos = pos.wrapping_add(2);
        }
        3 => {
            dictID = MEM_readLE32(ip.add(pos) as *const core::ffi::c_void);
            pos = pos.wrapping_add(4);
        }
        0 | _ => {}
    }
    match fcsID {
        1 => {
            frameContentSize = (MEM_readLE16(ip.add(pos) as *const core::ffi::c_void)
                as core::ffi::c_int
                + 256) as u64;
        }
        2 => {
            frameContentSize = MEM_readLE32(ip.add(pos) as *const core::ffi::c_void) as u64;
        }
        3 => {
            frameContentSize = MEM_readLE64(ip.add(pos) as *const core::ffi::c_void);
        }
        0 | _ => {
            if directMode != 0 {
                frameContentSize = *ip.add(pos) as u64;
            }
        }
    }
    if windowSize == 0 {
        windowSize = frameContentSize as u32;
    }
    if windowSize > windowSizeMax {
        return -(ZSTD_error_frameParameter_unsupported as core::ffi::c_int) as size_t;
    }
    (*fparamsPtr).frameContentSize = frameContentSize as core::ffi::c_ulonglong;
    (*fparamsPtr).windowSize = windowSize;
    (*fparamsPtr).dictID = dictID;
    (*fparamsPtr).checksumFlag = checksumFlag;
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_getDecompressedSize))]
pub unsafe extern "C" fn ZSTDv07_getDecompressedSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_ulonglong {
    let mut fparams = ZSTDv07_frameParams {
        frameContentSize: 0,
        windowSize: 0,
        dictID: 0,
        checksumFlag: 0,
    };
    let frResult = ZSTDv07_getFrameParams(&mut fparams, src, srcSize);
    if frResult != 0 {
        return 0;
    }
    fparams.frameContentSize
}
unsafe fn ZSTDv07_decodeFrameHeader(
    dctx: *mut ZSTDv07_DCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let result = ZSTDv07_getFrameParams(&mut (*dctx).fParams, src, srcSize);
    if (*dctx).fParams.dictID != 0 && (*dctx).dictID != (*dctx).fParams.dictID {
        return -(ZSTD_error_dictionary_wrong as core::ffi::c_int) as size_t;
    }
    if (*dctx).fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(&mut (*dctx).xxhState, 0);
    }
    result
}
unsafe fn ZSTDv07_getcBlockSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
    bpPtr: *mut blockProperties_t,
) -> size_t {
    let in_0 = src as *const u8;
    let mut cSize: u32 = 0;
    if srcSize < ZSTDv07_blockHeaderSize {
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
unsafe fn ZSTDv07_copyRawBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if srcSize > 0 {
        memcpy(dst, src, srcSize);
    }
    srcSize
}
unsafe fn ZSTDv07_decodeLiteralsBlock(
    dctx: *mut ZSTDv07_DCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let istart = src as *const u8;
    if srcSize < MIN_CBLOCK_SIZE as size_t {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    match (*istart.offset(0) as core::ffi::c_int >> 6) as litBlockType_t as core::ffi::c_uint {
        0 => {
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
            if litSize > ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as size_t {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if litCSize.wrapping_add(lhSize as size_t) > srcSize {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if ERR_isError(if singleStream != 0 {
                HUFv07_decompress1X2_DCtx(
                    ((*dctx).hufTable).as_mut_ptr(),
                    ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const core::ffi::c_void,
                    litCSize,
                )
            } else {
                HUFv07_decompress4X_hufOnly(
                    ((*dctx).hufTable).as_mut_ptr(),
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
            (*dctx).litEntropy = 1;
            ptr::write_bytes(
                ((*dctx).litBuffer).as_mut_ptr().add((*dctx).litSize) as *mut u8,
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            litCSize.wrapping_add(lhSize as size_t)
        }
        1 => {
            let mut litSize_0: size_t = 0;
            let mut litCSize_0: size_t = 0;
            let mut lhSize_0 = (*istart.offset(0) as core::ffi::c_int >> 4 & 3) as u32;
            if lhSize_0 != 1 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if (*dctx).litEntropy == 0 {
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
            let errorCode = HUFv07_decompress1X4_usingDTable(
                ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                litSize_0,
                istart.offset(lhSize_0 as isize) as *const core::ffi::c_void,
                litCSize_0,
                ((*dctx).hufTable).as_mut_ptr(),
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
        2 => {
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
        3 => {
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
            if litSize_2 > ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as size_t {
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
unsafe extern "C" fn ZSTDv07_buildSeqTable(
    DTable: *mut FSEv07_DTable,
    type_0: u32,
    mut max: u32,
    maxLog: u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    defaultNorm: *const i16,
    defaultLog: u32,
    flagRepeatTable: u32,
) -> size_t {
    match type_0 {
        1 => {
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            if *(src as *const u8) as u32 > max {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            FSEv07_buildDTable_rle(DTable, *(src as *const u8));
            1
        }
        0 => {
            FSEv07_buildDTable(DTable, defaultNorm, max, defaultLog);
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
                FSEv07_readNCount(norm.as_mut_ptr(), &mut max, &mut tableLog, src, srcSize);
            if ERR_isError(headerSize) != 0 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            FSEv07_buildDTable(DTable, norm.as_mut_ptr(), max, tableLog);
            headerSize
        }
    }
}
unsafe fn ZSTDv07_decodeSeqHeaders(
    nbSeqPtr: *mut core::ffi::c_int,
    DTableLL: *mut FSEv07_DTable,
    DTableML: *mut FSEv07_DTable,
    DTableOffb: *mut FSEv07_DTable,
    flagRepeatTable: u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let mut ip = istart;
    if srcSize < MIN_SEQUENCES_SIZE as size_t {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let fresh40 = ip;
    ip = ip.offset(1);
    let mut nbSeq = *fresh40 as core::ffi::c_int;
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
            let fresh41 = ip;
            ip = ip.offset(1);
            nbSeq = ((nbSeq - 0x80 as core::ffi::c_int) << 8) + *fresh41 as core::ffi::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.offset(4) > iend {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let LLtype = (*ip as core::ffi::c_int >> 6) as u32;
    let OFtype = (*ip as core::ffi::c_int >> 4 & 3) as u32;
    let MLtype = (*ip as core::ffi::c_int >> 2 & 3) as u32;
    ip = ip.offset(1);
    let llhSize = ZSTDv07_buildSeqTable(
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
    if ERR_isError(llhSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(llhSize);
    let ofhSize = ZSTDv07_buildSeqTable(
        DTableOffb,
        OFtype,
        MaxOff as u32,
        OffFSELog as u32,
        ip as *const core::ffi::c_void,
        iend.offset_from(ip) as core::ffi::c_long as size_t,
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(ofhSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(ofhSize);
    let mlhSize = ZSTDv07_buildSeqTable(
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
    if ERR_isError(mlhSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(mlhSize);
    ip.offset_from(istart) as core::ffi::c_long as size_t
}
unsafe fn ZSTDv07_decodeSequence(seqState: *mut seqState_t) -> seq_t {
    let mut seq = seq_t {
        litLength: 0,
        matchLength: 0,
        offset: 0,
    };
    let llCode = FSEv07_peekSymbol(&(*seqState).stateLL) as u32;
    let mlCode = FSEv07_peekSymbol(&(*seqState).stateML) as u32;
    let ofCode = FSEv07_peekSymbol(&(*seqState).stateOffb) as u32;
    let llBits = *LL_bits.as_ptr().offset(llCode as isize);
    let mlBits = *ML_bits.as_ptr().offset(mlCode as isize);
    let ofBits = ofCode;
    let totalBits = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    static LL_base: [u32; 36] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 20, 22, 24, 28, 32, 40, 48,
        64, 0x80, 0x100, 0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000,
    ];
    static ML_base: [u32; 53] = [
        3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29, 30, 31, 32, 33, 34, 35, 37, 39, 41, 43, 47, 51, 59, 67, 83, 99, 0x83, 0x103,
        0x203, 0x403, 0x803, 0x1003, 0x2003, 0x4003, 0x8003, 0x10003,
    ];
    static OF_base: [u32; 29] = [
        0, 1, 1, 5, 0xd, 0x1d, 0x3d, 0x7d, 0xfd, 0x1fd, 0x3fd, 0x7fd, 0xffd, 0x1ffd, 0x3ffd,
        0x7ffd, 0xfffd, 0x1fffd, 0x3fffd, 0x7fffd, 0xffffd, 0x1ffffd, 0x3ffffd, 0x7ffffd, 0xfffffd,
        0x1fffffd, 0x3fffffd, 0x7fffffd, 0xffffffd,
    ];
    let mut offset: size_t = 0;
    if ofCode == 0 {
        offset = 0;
    } else {
        offset = (*OF_base.as_ptr().offset(ofCode as isize) as size_t)
            .wrapping_add(BITv07_readBits(&mut (*seqState).DStream, ofBits));
        if MEM_32bits() != 0 {
            BITv07_reloadDStream(&mut (*seqState).DStream);
        }
    }
    if ofCode <= 1 {
        if (llCode == 0) as core::ffi::c_int & (offset <= 1) as core::ffi::c_int != 0 {
            offset = (1 as size_t).wrapping_sub(offset);
        }
        if offset != 0 {
            let temp = *((*seqState).prevOffset).as_mut_ptr().add(offset);
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
        *((*seqState).prevOffset).as_mut_ptr().offset(2) =
            *((*seqState).prevOffset).as_mut_ptr().offset(1);
        *((*seqState).prevOffset).as_mut_ptr().offset(1) =
            *((*seqState).prevOffset).as_mut_ptr().offset(0);
        *((*seqState).prevOffset).as_mut_ptr().offset(0) = offset;
    }
    seq.offset = offset;
    seq.matchLength =
        (*ML_base.as_ptr().offset(mlCode as isize) as size_t).wrapping_add(if mlCode > 31 {
            BITv07_readBits(&mut (*seqState).DStream, mlBits)
        } else {
            0
        });
    if MEM_32bits() != 0 && mlBits.wrapping_add(llBits) > 24 {
        BITv07_reloadDStream(&mut (*seqState).DStream);
    }
    seq.litLength =
        (*LL_base.as_ptr().offset(llCode as isize) as size_t).wrapping_add(if llCode > 15 {
            BITv07_readBits(&mut (*seqState).DStream, llBits)
        } else {
            0
        });
    if MEM_32bits() != 0 || totalBits > (64 - 7 - (LLFSELog + MLFSELog + OffFSELog)) as u32 {
        BITv07_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv07_updateState(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    FSEv07_updateState(&mut (*seqState).stateML, &mut (*seqState).DStream);
    if MEM_32bits() != 0 {
        BITv07_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv07_updateState(&mut (*seqState).stateOffb, &mut (*seqState).DStream);
    seq
}
unsafe fn ZSTDv07_execSequence(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    litPtr: *mut *const u8,
    litLimit: *const u8,
    base: *const u8,
    vBase: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.add(sequenceLength);
    let oend_w = oend.wrapping_sub(WILDCOPY_OVERLENGTH as usize);
    let iLitEnd = (*litPtr).add(sequence.litLength);
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset);
    if (sequence.litLength).wrapping_add(WILDCOPY_OVERLENGTH as size_t)
        > oend.offset_from(op) as core::ffi::c_long as size_t
    {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if sequenceLength > oend.offset_from(op) as core::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as core::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ZSTDv07_wildcopy(
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
        if op > oend_w || sequence.matchLength < MINMATCH as size_t {
            while op < oMatchEnd {
                let fresh42 = match_0;
                match_0 = match_0.offset(1);
                let fresh43 = op;
                op = op.offset(1);
                *fresh43 = *fresh42;
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
        ZSTDv07_copy4(
            op.offset(4) as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
        );
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ZSTDv07_copy8(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
        );
    }
    op = op.offset(8);
    match_0 = match_0.offset(8);
    if oMatchEnd > oend.offset(-((16 - MINMATCH) as isize)) {
        if op < oend_w {
            ZSTDv07_wildcopy(
                op as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                oend_w.offset_from(op) as ptrdiff_t,
            );
            match_0 = match_0.offset(oend_w.offset_from(op) as core::ffi::c_long as isize);
            op = oend_w;
        }
        while op < oMatchEnd {
            let fresh44 = match_0;
            match_0 = match_0.offset(1);
            let fresh45 = op;
            op = op.offset(1);
            *fresh45 = *fresh44;
        }
    } else {
        ZSTDv07_wildcopy(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            sequence.matchLength as ptrdiff_t - 8,
        );
    }
    sequenceLength
}
unsafe fn ZSTDv07_decompressSequences(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: *const core::ffi::c_void,
    seqSize: size_t,
) -> size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.add(seqSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(maxDstSize);
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.add((*dctx).litSize);
    let DTableLL = ((*dctx).LLTable).as_mut_ptr();
    let DTableML = ((*dctx).MLTable).as_mut_ptr();
    let DTableOffb = ((*dctx).OffTable).as_mut_ptr();
    let base = (*dctx).base as *const u8;
    let vBase = (*dctx).vBase as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    let mut nbSeq: core::ffi::c_int = 0;
    let seqHSize = ZSTDv07_decodeSeqHeaders(
        &mut nbSeq,
        DTableLL,
        DTableML,
        DTableOffb,
        (*dctx).fseEntropy,
        ip as *const core::ffi::c_void,
        seqSize,
    );
    if ERR_isError(seqHSize) != 0 {
        return seqHSize;
    }
    ip = ip.add(seqHSize);
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BITv07_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: core::ptr::null::<core::ffi::c_char>(),
                start: core::ptr::null::<core::ffi::c_char>(),
            },
            stateLL: FSEv07_DState_t {
                state: 0,
                table: core::ptr::null::<core::ffi::c_void>(),
            },
            stateOffb: FSEv07_DState_t {
                state: 0,
                table: core::ptr::null::<core::ffi::c_void>(),
            },
            stateML: FSEv07_DState_t {
                state: 0,
                table: core::ptr::null::<core::ffi::c_void>(),
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1;
        let mut i: u32 = 0;
        i = 0;
        while i < ZSTDv07_REP_INIT as u32 {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) =
                *((*dctx).rep).as_mut_ptr().offset(i as isize) as size_t;
            i = i.wrapping_add(1);
        }
        let errorCode = BITv07_initDStream(
            &mut seqState.DStream,
            ip as *const core::ffi::c_void,
            iend.offset_from(ip) as core::ffi::c_long as size_t,
        );
        if ERR_isError(errorCode) != 0 {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        FSEv07_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
        FSEv07_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
        FSEv07_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
        while BITv07_reloadDStream(&mut seqState.DStream) as core::ffi::c_uint
            <= BITv07_DStream_completed as core::ffi::c_int as core::ffi::c_uint
            && nbSeq != 0
        {
            nbSeq -= 1;
            let sequence = ZSTDv07_decodeSequence(&mut seqState);
            let oneSeqSize = ZSTDv07_execSequence(
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
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTDv07_REP_INIT as u32 {
            *((*dctx).rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if lastLLSize > oend.offset_from(op) as core::ffi::c_long as size_t {
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
unsafe fn ZSTDv07_checkContinuity(dctx: *mut ZSTDv07_DCtx, dst: *const core::ffi::c_void) {
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
unsafe fn ZSTDv07_decompressBlock_internal(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    if srcSize >= ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as size_t {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let litCSize = ZSTDv07_decodeLiteralsBlock(dctx, src, srcSize);
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.add(litCSize);
    srcSize = srcSize.wrapping_sub(litCSize);
    ZSTDv07_decompressSequences(
        dctx,
        dst,
        dstCapacity,
        ip as *const core::ffi::c_void,
        srcSize,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompressBlock))]
pub unsafe extern "C" fn ZSTDv07_decompressBlock(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut dSize: size_t = 0;
    ZSTDv07_checkContinuity(dctx, dst);
    dSize = ZSTDv07_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize);
    (*dctx).previousDstEnd = (dst as *mut core::ffi::c_char).add(dSize) as *const core::ffi::c_void;
    dSize
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_insertBlock))]
pub unsafe extern "C" fn ZSTDv07_insertBlock(
    dctx: *mut ZSTDv07_DCtx,
    blockStart: *const core::ffi::c_void,
    blockSize: size_t,
) -> size_t {
    ZSTDv07_checkContinuity(dctx, blockStart);
    (*dctx).previousDstEnd =
        (blockStart as *const core::ffi::c_char).add(blockSize) as *const core::ffi::c_void;
    blockSize
}
unsafe fn ZSTDv07_generateNxBytes(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    byte: u8,
    length: size_t,
) -> size_t {
    if length > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if length > 0 {
        memset(dst, byte as core::ffi::c_int, length);
    }
    length
}
unsafe fn ZSTDv07_decompressFrame(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstCapacity);
    let mut op = ostart;
    let mut remainingSize = srcSize;
    if srcSize < ZSTDv07_frameHeaderSize_min.wrapping_add(ZSTDv07_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let frameHeaderSize = ZSTDv07_frameHeaderSize(src, ZSTDv07_frameHeaderSize_min);
    if ERR_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv07_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if ZSTDv07_decodeFrameHeader(dctx, src, frameHeaderSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ip = ip.add(frameHeaderSize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    loop {
        let mut decodedSize: size_t = 0;
        let mut blockProperties = blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        };
        let cBlockSize = ZSTDv07_getcBlockSize(
            ip as *const core::ffi::c_void,
            iend.offset_from(ip) as core::ffi::c_long as size_t,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            return cBlockSize;
        }
        ip = ip.add(ZSTDv07_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv07_blockHeaderSize);
        if cBlockSize > remainingSize {
            return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
        }
        match blockProperties.blockType as core::ffi::c_uint {
            0 => {
                decodedSize = ZSTDv07_decompressBlock_internal(
                    dctx,
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as core::ffi::c_long as size_t,
                    ip as *const core::ffi::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTDv07_copyRawBlock(
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as core::ffi::c_long as size_t,
                    ip as *const core::ffi::c_void,
                    cBlockSize,
                );
            }
            2 => {
                decodedSize = ZSTDv07_generateNxBytes(
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as core::ffi::c_long as size_t,
                    *ip,
                    blockProperties.origSize as size_t,
                );
            }
            3 => {
                if remainingSize != 0 {
                    return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
                }
                decodedSize = 0;
            }
            _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
        }
        if blockProperties.blockType as core::ffi::c_uint
            == bt_end as core::ffi::c_int as core::ffi::c_uint
        {
            break;
        }
        if ERR_isError(decodedSize) != 0 {
            return decodedSize;
        }
        if (*dctx).fParams.checksumFlag != 0 {
            ZSTD_XXH64_update(
                &mut (*dctx).xxhState,
                op as *const core::ffi::c_void,
                decodedSize as usize,
            );
        }
        op = op.add(decodedSize);
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
unsafe fn ZSTDv07_decompress_usingPreparedDCtx(
    dctx: *mut ZSTDv07_DCtx,
    refDCtx: *const ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTDv07_copyDCtx(dctx, refDCtx);
    ZSTDv07_checkContinuity(dctx, dst);
    ZSTDv07_decompressFrame(dctx, dst, dstCapacity, src, srcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompress_usingDict))]
pub unsafe extern "C" fn ZSTDv07_decompress_usingDict(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    ZSTDv07_decompressBegin_usingDict(dctx, dict, dictSize);
    ZSTDv07_checkContinuity(dctx, dst);
    ZSTDv07_decompressFrame(dctx, dst, dstCapacity, src, srcSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompressDCtx))]
pub unsafe extern "C" fn ZSTDv07_decompressDCtx(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTDv07_decompress_usingDict(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        NULL as *const core::ffi::c_void,
        0,
    )
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompress))]
pub unsafe extern "C" fn ZSTDv07_decompress(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut regenSize: size_t = 0;
    let dctx = ZSTDv07_createDCtx();
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    }
    regenSize = ZSTDv07_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTDv07_freeDCtx(dctx);
    regenSize
}
unsafe fn ZSTD_errorFrameSizeInfoLegacy(
    cSize: *mut size_t,
    dBound: *mut core::ffi::c_ulonglong,
    ret: size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_findFrameSizeInfoLegacy))]
pub unsafe extern "C" fn ZSTDv07_findFrameSizeInfoLegacy(
    src: *const core::ffi::c_void,
    srcSize: size_t,
    cSize: *mut size_t,
    dBound: *mut core::ffi::c_ulonglong,
) {
    let mut ip = src as *const u8;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as size_t;
    if srcSize < ZSTDv07_frameHeaderSize_min.wrapping_add(ZSTDv07_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t,
        );
        return;
    }
    let frameHeaderSize = ZSTDv07_frameHeaderSize(src, srcSize);
    if ERR_isError(frameHeaderSize) != 0 {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, frameHeaderSize);
        return;
    }
    if MEM_readLE32(src) != ZSTDv07_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_prefix_unknown as core::ffi::c_int) as size_t,
        );
        return;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv07_blockHeaderSize) {
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
        let mut blockProperties = blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        };
        let cBlockSize = ZSTDv07_getcBlockSize(
            ip as *const core::ffi::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, cBlockSize);
            return;
        }
        ip = ip.add(ZSTDv07_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv07_blockHeaderSize);
        if blockProperties.blockType as core::ffi::c_uint
            == bt_end as core::ffi::c_int as core::ffi::c_uint
        {
            break;
        }
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(
                cSize,
                dBound,
                -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t,
            );
            return;
        }
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const u8) as core::ffi::c_long as size_t;
    *dBound = (nbBlocks * ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as size_t) as core::ffi::c_ulonglong;
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_nextSrcSizeToDecompress))]
pub unsafe extern "C" fn ZSTDv07_nextSrcSizeToDecompress(dctx: *mut ZSTDv07_DCtx) -> size_t {
    (*dctx).expected
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_isSkipFrame))]
pub unsafe extern "C" fn ZSTDv07_isSkipFrame(dctx: *mut ZSTDv07_DCtx) -> core::ffi::c_int {
    ((*dctx).stage as core::ffi::c_uint
        == ZSTDds_skipFrame as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompressContinue))]
pub unsafe extern "C" fn ZSTDv07_decompressContinue(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize != (*dctx).expected {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if dstCapacity != 0 {
        ZSTDv07_checkContinuity(dctx, dst);
    }
    match (*dctx).stage as core::ffi::c_uint {
        0 => {
            if srcSize != ZSTDv07_frameHeaderSize_min {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            if MEM_readLE32(src) & 0xfffffff0 as core::ffi::c_uint == ZSTDv07_MAGIC_SKIPPABLE_START
            {
                memcpy(
                    ((*dctx).headerBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    src,
                    ZSTDv07_frameHeaderSize_min,
                );
                (*dctx).expected =
                    ZSTDv07_skippableHeaderSize.wrapping_sub(ZSTDv07_frameHeaderSize_min);
                (*dctx).stage = ZSTDds_decodeSkippableHeader;
                return 0;
            }
            (*dctx).headerSize = ZSTDv07_frameHeaderSize(src, ZSTDv07_frameHeaderSize_min);
            if ERR_isError((*dctx).headerSize) != 0 {
                return (*dctx).headerSize;
            }
            memcpy(
                ((*dctx).headerBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                src,
                ZSTDv07_frameHeaderSize_min,
            );
            if (*dctx).headerSize > ZSTDv07_frameHeaderSize_min {
                (*dctx).expected = ((*dctx).headerSize).wrapping_sub(ZSTDv07_frameHeaderSize_min);
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
            let cBlockSize = ZSTDv07_getcBlockSize(src, ZSTDv07_blockHeaderSize, &mut bp);
            if ERR_isError(cBlockSize) != 0 {
                return cBlockSize;
            }
            if bp.blockType as core::ffi::c_uint == bt_end as core::ffi::c_int as core::ffi::c_uint
            {
                if (*dctx).fParams.checksumFlag != 0 {
                    let h64 = ZSTD_XXH64_digest(&mut (*dctx).xxhState);
                    let h32 = (h64 >> 11) as u32 & (((1) << 22) - 1) as u32;
                    let ip = src as *const u8;
                    let check32 = (*ip.offset(2) as core::ffi::c_int
                        + ((*ip.offset(1) as core::ffi::c_int) << 8)
                        + ((*ip.offset(0) as core::ffi::c_int & 0x3f as core::ffi::c_int) << 16))
                        as u32;
                    if check32 != h32 {
                        return -(ZSTD_error_checksum_wrong as core::ffi::c_int) as size_t;
                    }
                }
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
                    rSize = ZSTDv07_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize);
                }
                1 => {
                    rSize = ZSTDv07_copyRawBlock(dst, dstCapacity, src, srcSize);
                }
                2 => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
                3 => {
                    rSize = 0;
                }
                _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
            }
            (*dctx).stage = ZSTDds_decodeBlockHeader;
            (*dctx).expected = ZSTDv07_blockHeaderSize;
            if ERR_isError(rSize) != 0 {
                return rSize;
            }
            (*dctx).previousDstEnd =
                (dst as *mut core::ffi::c_char).add(rSize) as *const core::ffi::c_void;
            if (*dctx).fParams.checksumFlag != 0 {
                ZSTD_XXH64_update(&mut (*dctx).xxhState, dst, rSize as usize);
            }
            return rSize;
        }
        4 => {
            memcpy(
                ((*dctx).headerBuffer)
                    .as_mut_ptr()
                    .add(ZSTDv07_frameHeaderSize_min) as *mut core::ffi::c_void,
                src,
                (*dctx).expected,
            );
            (*dctx).expected = MEM_readLE32(
                ((*dctx).headerBuffer).as_mut_ptr().offset(4) as *const core::ffi::c_void
            ) as size_t;
            (*dctx).stage = ZSTDds_skipFrame;
            return 0;
        }
        5 => {
            (*dctx).expected = 0;
            (*dctx).stage = ZSTDds_getFrameHeaderSize;
            return 0;
        }
        _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
    }
    let mut result: size_t = 0;
    memcpy(
        ((*dctx).headerBuffer)
            .as_mut_ptr()
            .add(ZSTDv07_frameHeaderSize_min) as *mut core::ffi::c_void,
        src,
        (*dctx).expected,
    );
    result = ZSTDv07_decodeFrameHeader(
        dctx,
        ((*dctx).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
        (*dctx).headerSize,
    );
    if ERR_isError(result) != 0 {
        return result;
    }
    (*dctx).expected = ZSTDv07_blockHeaderSize;
    (*dctx).stage = ZSTDds_decodeBlockHeader;
    0
}
unsafe fn ZSTDv07_refDictContent(
    dctx: *mut ZSTDv07_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).vBase = (dict as *const core::ffi::c_char).offset(
        -(((*dctx).previousDstEnd as *const core::ffi::c_char)
            .offset_from((*dctx).base as *const core::ffi::c_char) as core::ffi::c_long
            as isize),
    ) as *const core::ffi::c_void;
    (*dctx).base = dict;
    (*dctx).previousDstEnd =
        (dict as *const core::ffi::c_char).add(dictSize) as *const core::ffi::c_void;
    0
}
unsafe fn ZSTDv07_loadEntropy(
    dctx: *mut ZSTDv07_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let mut dictPtr = dict as *const u8;
    let dictEnd = dictPtr.add(dictSize);
    let hSize = HUFv07_readDTableX4(((*dctx).hufTable).as_mut_ptr(), dict, dictSize);
    if ERR_isError(hSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dictPtr = dictPtr.add(hSize);
    let mut offcodeNCount: [core::ffi::c_short; 29] = [0; 29];
    let mut offcodeMaxValue = MaxOff as u32;
    let mut offcodeLog: u32 = 0;
    let offcodeHeaderSize = FSEv07_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from(dictPtr) as core::ffi::c_long as size_t,
    );
    if ERR_isError(offcodeHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    if offcodeLog > OffFSELog as u32 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    let errorCode = FSEv07_buildDTable(
        ((*dctx).OffTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        offcodeLog,
    );
    if ERR_isError(errorCode) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dictPtr = dictPtr.add(offcodeHeaderSize);
    let mut matchlengthNCount: [core::ffi::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as core::ffi::c_uint;
    let mut matchlengthLog: core::ffi::c_uint = 0;
    let matchlengthHeaderSize = FSEv07_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from(dictPtr) as core::ffi::c_long as size_t,
    );
    if ERR_isError(matchlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    if matchlengthLog > MLFSELog as core::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    let errorCode_0 = FSEv07_buildDTable(
        ((*dctx).MLTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
    );
    if ERR_isError(errorCode_0) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dictPtr = dictPtr.add(matchlengthHeaderSize);
    let mut litlengthNCount: [core::ffi::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as core::ffi::c_uint;
    let mut litlengthLog: core::ffi::c_uint = 0;
    let litlengthHeaderSize = FSEv07_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dictPtr as *const core::ffi::c_void,
        dictEnd.offset_from(dictPtr) as core::ffi::c_long as size_t,
    );
    if ERR_isError(litlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    if litlengthLog > LLFSELog as core::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    let errorCode_1 = FSEv07_buildDTable(
        ((*dctx).LLTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
    );
    if ERR_isError(errorCode_1) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dictPtr = dictPtr.add(litlengthHeaderSize);
    if dictPtr.offset(12) > dictEnd {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    *((*dctx).rep).as_mut_ptr().offset(0) =
        MEM_readLE32(dictPtr.offset(0) as *const core::ffi::c_void);
    if *((*dctx).rep).as_mut_ptr().offset(0) == 0
        || *((*dctx).rep).as_mut_ptr().offset(0) as size_t >= dictSize
    {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    *((*dctx).rep).as_mut_ptr().offset(1) =
        MEM_readLE32(dictPtr.offset(4) as *const core::ffi::c_void);
    if *((*dctx).rep).as_mut_ptr().offset(1) == 0
        || *((*dctx).rep).as_mut_ptr().offset(1) as size_t >= dictSize
    {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    *((*dctx).rep).as_mut_ptr().offset(2) =
        MEM_readLE32(dictPtr.offset(8) as *const core::ffi::c_void);
    if *((*dctx).rep).as_mut_ptr().offset(2) == 0
        || *((*dctx).rep).as_mut_ptr().offset(2) as size_t >= dictSize
    {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dictPtr = dictPtr.offset(12);
    (*dctx).fseEntropy = 1;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    dictPtr.offset_from(dict as *const u8) as core::ffi::c_long as size_t
}
unsafe fn ZSTDv07_decompress_insertDictionary(
    dctx: *mut ZSTDv07_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    if dictSize < 8 {
        return ZSTDv07_refDictContent(dctx, dict, dictSize);
    }
    let magic = MEM_readLE32(dict);
    if magic != ZSTDv07_DICT_MAGIC {
        return ZSTDv07_refDictContent(dctx, dict, dictSize);
    }
    (*dctx).dictID =
        MEM_readLE32((dict as *const core::ffi::c_char).offset(4) as *const core::ffi::c_void);
    dict = (dict as *const core::ffi::c_char).offset(8) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(8);
    let eSize = ZSTDv07_loadEntropy(dctx, dict, dictSize);
    if ERR_isError(eSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
    }
    dict = (dict as *const core::ffi::c_char).add(eSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(eSize);
    ZSTDv07_refDictContent(dctx, dict, dictSize)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompressBegin_usingDict))]
pub unsafe extern "C" fn ZSTDv07_decompressBegin_usingDict(
    dctx: *mut ZSTDv07_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let errorCode = ZSTDv07_decompressBegin(dctx);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    if !dict.is_null() && dictSize != 0 {
        let errorCode_0 = ZSTDv07_decompress_insertDictionary(dctx, dict, dictSize);
        if ERR_isError(errorCode_0) != 0 {
            return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
        }
    }
    0
}
unsafe fn ZSTDv07_createDDict_advanced(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    mut customMem: ZSTDv07_customMem,
) -> *mut ZSTDv07_DDict {
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return NULL as *mut ZSTDv07_DDict;
    }
    let ddict = (customMem.customAlloc).unwrap_unchecked()(
        customMem.opaque,
        ::core::mem::size_of::<ZSTDv07_DDict>() as size_t,
    ) as *mut ZSTDv07_DDict;
    let dictContent = (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, dictSize);
    let dctx = ZSTDv07_createDCtx_advanced(customMem);
    if dictContent.is_null() || ddict.is_null() || dctx.is_null() {
        (customMem.customFree).unwrap_unchecked()(customMem.opaque, dictContent);
        (customMem.customFree).unwrap_unchecked()(
            customMem.opaque,
            ddict as *mut core::ffi::c_void,
        );
        (customMem.customFree).unwrap_unchecked()(customMem.opaque, dctx as *mut core::ffi::c_void);
        return NULL as *mut ZSTDv07_DDict;
    }
    memcpy(dictContent, dict, dictSize);
    let errorCode = ZSTDv07_decompressBegin_usingDict(dctx, dictContent, dictSize);
    if ERR_isError(errorCode) != 0 {
        (customMem.customFree).unwrap_unchecked()(customMem.opaque, dictContent);
        (customMem.customFree).unwrap_unchecked()(
            customMem.opaque,
            ddict as *mut core::ffi::c_void,
        );
        (customMem.customFree).unwrap_unchecked()(customMem.opaque, dctx as *mut core::ffi::c_void);
        return NULL as *mut ZSTDv07_DDict;
    }
    (*ddict).dict = dictContent;
    (*ddict).dictSize = dictSize;
    (*ddict).refContext = dctx;
    ddict
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_createDDict))]
pub unsafe extern "C" fn ZSTDv07_createDDict(
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> *mut ZSTDv07_DDict {
    let allocator = {
        ZSTDv07_customMem {
            customAlloc: ::core::mem::transmute::<libc::intptr_t, ZSTDv07_allocFunction>(
                NULL as libc::intptr_t,
            ),
            customFree: ::core::mem::transmute::<libc::intptr_t, ZSTDv07_freeFunction>(
                NULL as libc::intptr_t,
            ),
            opaque: NULL as *mut core::ffi::c_void,
        }
    };
    ZSTDv07_createDDict_advanced(dict, dictSize, allocator)
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_freeDDict))]
pub unsafe extern "C" fn ZSTDv07_freeDDict(ddict: *mut ZSTDv07_DDict) -> size_t {
    let cFree: ZSTDv07_freeFunction = (*(*ddict).refContext).customMem.customFree;
    let opaque = (*(*ddict).refContext).customMem.opaque;
    ZSTDv07_freeDCtx((*ddict).refContext);
    cFree.unwrap_unchecked()(opaque, (*ddict).dict);
    cFree.unwrap_unchecked()(opaque, ddict as *mut core::ffi::c_void);
    0
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTDv07_decompress_usingDDict))]
pub unsafe extern "C" fn ZSTDv07_decompress_usingDDict(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    ddict: *const ZSTDv07_DDict,
) -> size_t {
    ZSTDv07_decompress_usingPreparedDCtx(dctx, (*ddict).refContext, dst, dstCapacity, src, srcSize)
}
pub unsafe fn ZBUFFv07_createDCtx() -> *mut ZBUFFv07_DCtx {
    ZBUFFv07_createDCtx_advanced(defaultCustomMem)
}
pub unsafe fn ZBUFFv07_createDCtx_advanced(mut customMem: ZSTDv07_customMem) -> *mut ZBUFFv07_DCtx {
    let mut zbd = core::ptr::null_mut::<ZBUFFv07_DCtx>();
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return NULL as *mut ZBUFFv07_DCtx;
    }
    zbd = (customMem.customAlloc).unwrap_unchecked()(
        customMem.opaque,
        ::core::mem::size_of::<ZBUFFv07_DCtx>() as size_t,
    ) as *mut ZBUFFv07_DCtx;
    if zbd.is_null() {
        return NULL as *mut ZBUFFv07_DCtx;
    }
    ptr::write_bytes(zbd as *mut u8, 0, ::core::mem::size_of::<ZBUFFv07_DCtx>());
    memcpy(
        &mut (*zbd).customMem as *mut ZSTDv07_customMem as *mut core::ffi::c_void,
        &mut customMem as *mut ZSTDv07_customMem as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTDv07_customMem>() as size_t,
    );
    (*zbd).zd = ZSTDv07_createDCtx_advanced(customMem);
    if ((*zbd).zd).is_null() {
        ZBUFFv07_freeDCtx(zbd);
        return NULL as *mut ZBUFFv07_DCtx;
    }
    (*zbd).stage = ZBUFFds_init;
    zbd
}
pub unsafe fn ZBUFFv07_freeDCtx(zbd: *mut ZBUFFv07_DCtx) -> size_t {
    if zbd.is_null() {
        return 0;
    }
    ZSTDv07_freeDCtx((*zbd).zd);
    if !((*zbd).inBuff).is_null() {
        ((*zbd).customMem.customFree).unwrap_unchecked()(
            (*zbd).customMem.opaque,
            (*zbd).inBuff as *mut core::ffi::c_void,
        );
    }
    if !((*zbd).outBuff).is_null() {
        ((*zbd).customMem.customFree).unwrap_unchecked()(
            (*zbd).customMem.opaque,
            (*zbd).outBuff as *mut core::ffi::c_void,
        );
    }
    ((*zbd).customMem.customFree).unwrap_unchecked()(
        (*zbd).customMem.opaque,
        zbd as *mut core::ffi::c_void,
    );
    0
}
pub unsafe fn ZBUFFv07_decompressInitDictionary(
    zbd: *mut ZBUFFv07_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    (*zbd).stage = ZBUFFds_loadHeader;
    (*zbd).outEnd = 0;
    (*zbd).outStart = (*zbd).outEnd;
    (*zbd).inPos = (*zbd).outStart;
    (*zbd).lhSize = (*zbd).inPos;
    ZSTDv07_decompressBegin_usingDict((*zbd).zd, dict, dictSize)
}
pub unsafe fn ZBUFFv07_decompressInit(zbd: *mut ZBUFFv07_DCtx) -> size_t {
    ZBUFFv07_decompressInitDictionary(zbd, NULL as *const core::ffi::c_void, 0)
}
#[inline]
unsafe fn ZBUFFv07_limitCopy(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let length = if dstCapacity < srcSize {
        dstCapacity
    } else {
        srcSize
    };
    if length > 0 {
        memcpy(dst, src, length);
    }
    length
}
pub unsafe fn ZBUFFv07_decompressContinue(
    zbd: *mut ZBUFFv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacityPtr: *mut size_t,
    src: *const core::ffi::c_void,
    srcSizePtr: *mut size_t,
) -> size_t {
    let istart = src as *const core::ffi::c_char;
    let iend = istart.add(*srcSizePtr);
    let mut ip = istart;
    let ostart = dst as *mut core::ffi::c_char;
    let oend = ostart.add(*dstCapacityPtr);
    let mut op = ostart;
    let mut notDone = 1;
    while notDone != 0 {
        let mut current_block_66: u64;
        match (*zbd).stage as core::ffi::c_uint {
            0 => return -(ZSTD_error_init_missing as core::ffi::c_int) as size_t,
            1 => {
                let hSize = ZSTDv07_getFrameParams(
                    &mut (*zbd).fParams,
                    ((*zbd).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
                    (*zbd).lhSize,
                );
                if ERR_isError(hSize) != 0 {
                    return hSize;
                }
                if hSize != 0 {
                    let toLoad = hSize.wrapping_sub((*zbd).lhSize);
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
                            .wrapping_add(ZSTDv07_blockHeaderSize);
                    }
                    memcpy(
                        ((*zbd).headerBuffer).as_mut_ptr().add((*zbd).lhSize)
                            as *mut core::ffi::c_void,
                        ip as *const core::ffi::c_void,
                        toLoad,
                    );
                    (*zbd).lhSize = hSize;
                    ip = ip.add(toLoad);
                    current_block_66 = 12961834331865314435;
                } else {
                    let h1Size = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
                    let h1Result = ZSTDv07_decompressContinue(
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
                        let h2Size = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
                        let h2Result = ZSTDv07_decompressContinue(
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
                    (*zbd).fParams.windowSize = if (*zbd).fParams.windowSize > (1) << 10 {
                        (*zbd).fParams.windowSize
                    } else {
                        (1) << 10
                    };
                    let blockSize =
                        (if (*zbd).fParams.windowSize < (128 * 1024) as core::ffi::c_uint {
                            (*zbd).fParams.windowSize
                        } else {
                            (128 * 1024) as core::ffi::c_uint
                        }) as size_t;
                    (*zbd).blockSize = blockSize;
                    if (*zbd).inBuffSize < blockSize {
                        ((*zbd).customMem.customFree).unwrap_unchecked()(
                            (*zbd).customMem.opaque,
                            (*zbd).inBuff as *mut core::ffi::c_void,
                        );
                        (*zbd).inBuffSize = blockSize;
                        (*zbd).inBuff = ((*zbd).customMem.customAlloc).unwrap_unchecked()(
                            (*zbd).customMem.opaque,
                            blockSize,
                        ) as *mut core::ffi::c_char;
                        if ((*zbd).inBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
                        }
                    }
                    let neededOutSize = ((*zbd).fParams.windowSize as size_t)
                        .wrapping_add(blockSize)
                        .wrapping_add((WILDCOPY_OVERLENGTH * 2) as size_t);
                    if (*zbd).outBuffSize < neededOutSize {
                        ((*zbd).customMem.customFree).unwrap_unchecked()(
                            (*zbd).customMem.opaque,
                            (*zbd).outBuff as *mut core::ffi::c_void,
                        );
                        (*zbd).outBuffSize = neededOutSize;
                        (*zbd).outBuff = ((*zbd).customMem.customAlloc).unwrap_unchecked()(
                            (*zbd).customMem.opaque,
                            neededOutSize,
                        ) as *mut core::ffi::c_char;
                        if ((*zbd).outBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
                        }
                    }
                    (*zbd).stage = ZBUFFds_read;
                    current_block_66 = 8845338526596852646;
                }
            }
            2 => {
                current_block_66 = 8845338526596852646;
            }
            3 => {
                current_block_66 = 14945149239039849694;
            }
            4 => {
                current_block_66 = 5181772461570869434;
            }
            _ => return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t,
        }
        if current_block_66 == 8845338526596852646 {
            let neededInSize = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
            if neededInSize == 0 {
                (*zbd).stage = ZBUFFds_init;
                notDone = 0;
                current_block_66 = 12961834331865314435;
            } else if iend.offset_from(ip) as core::ffi::c_long as size_t >= neededInSize {
                let isSkipFrame = ZSTDv07_isSkipFrame((*zbd).zd);
                let decodedSize = ZSTDv07_decompressContinue(
                    (*zbd).zd,
                    ((*zbd).outBuff).add((*zbd).outStart) as *mut core::ffi::c_void,
                    if isSkipFrame != 0 {
                        0
                    } else {
                        ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart)
                    },
                    ip as *const core::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(decodedSize) != 0 {
                    return decodedSize;
                }
                ip = ip.add(neededInSize);
                if decodedSize == 0 && isSkipFrame == 0 {
                    current_block_66 = 12961834331865314435;
                } else {
                    (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize);
                    (*zbd).stage = ZBUFFds_flush;
                    current_block_66 = 12961834331865314435;
                }
            } else if ip == iend {
                notDone = 0;
                current_block_66 = 12961834331865314435;
            } else {
                (*zbd).stage = ZBUFFds_load;
                current_block_66 = 14945149239039849694;
            }
        }
        if current_block_66 == 14945149239039849694 {
            let neededInSize_0 = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
            let toLoad_0 = neededInSize_0.wrapping_sub((*zbd).inPos);
            let mut loadedSize: size_t = 0;
            if toLoad_0 > ((*zbd).inBuffSize).wrapping_sub((*zbd).inPos) {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            loadedSize = ZBUFFv07_limitCopy(
                ((*zbd).inBuff).add((*zbd).inPos) as *mut core::ffi::c_void,
                toLoad_0,
                ip as *const core::ffi::c_void,
                iend.offset_from(ip) as core::ffi::c_long as size_t,
            );
            ip = ip.add(loadedSize);
            (*zbd).inPos = ((*zbd).inPos).wrapping_add(loadedSize);
            if loadedSize < toLoad_0 {
                notDone = 0;
                current_block_66 = 12961834331865314435;
            } else {
                let isSkipFrame_0 = ZSTDv07_isSkipFrame((*zbd).zd);
                let decodedSize_0 = ZSTDv07_decompressContinue(
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
                if decodedSize_0 == 0 && isSkipFrame_0 == 0 {
                    (*zbd).stage = ZBUFFds_read;
                    current_block_66 = 12961834331865314435;
                } else {
                    (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize_0);
                    (*zbd).stage = ZBUFFds_flush;
                    current_block_66 = 5181772461570869434;
                }
            }
        }
        if current_block_66 == 5181772461570869434 {
            let toFlushSize = ((*zbd).outEnd).wrapping_sub((*zbd).outStart);
            let flushedSize = ZBUFFv07_limitCopy(
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
    let mut nextSrcSizeHint = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub((*zbd).inPos);
    nextSrcSizeHint
}
pub unsafe fn ZBUFFv07_recommendedDInSize() -> size_t {
    (ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as size_t).wrapping_add(ZSTDv07_blockHeaderSize)
}
pub unsafe fn ZBUFFv07_recommendedDOutSize() -> size_t {
    ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as size_t
}
