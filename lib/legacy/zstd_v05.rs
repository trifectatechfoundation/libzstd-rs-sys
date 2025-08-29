use core::ptr;

use libc::{free, malloc, memcpy, ptrdiff_t, size_t};

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::{
    MEM_32bits, MEM_64bits, MEM_readLE16, MEM_readLE32, MEM_readLEST, MEM_writeLE16,
};

#[repr(C)]
pub(crate) struct ZSTDv05_DCtx {
    LLTable: [FSEv05_DTable; 1025],
    OffTable: [FSEv05_DTable; 513],
    MLTable: [FSEv05_DTable; 1025],
    hufTableX4: [core::ffi::c_uint; 4097],
    previousDstEnd: *const core::ffi::c_void,
    base: *const core::ffi::c_void,
    vBase: *const core::ffi::c_void,
    dictEnd: *const core::ffi::c_void,
    expected: size_t,
    headerSize: size_t,
    params: ZSTDv05_parameters,
    bType: blockType_t,
    stage: ZSTDv05_dStage,
    flagStaticTables: u32,
    litPtr: *const u8,
    litSize: size_t,
    litBuffer: [u8; 131080],
    headerBuffer: [u8; 5],
}
type ZSTDv05_dStage = core::ffi::c_uint;
const ZSTDv05ds_decompressBlock: ZSTDv05_dStage = 3;
const ZSTDv05ds_decodeBlockHeader: ZSTDv05_dStage = 2;
const ZSTDv05ds_getFrameHeaderSize: ZSTDv05_dStage = 0;
type blockType_t = core::ffi::c_uint;
const bt_end: blockType_t = 3;
const bt_rle: blockType_t = 2;
const bt_compressed: blockType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ZSTDv05_parameters {
    pub(crate) srcSize: u64,
    pub(crate) windowLog: u32,
    pub(crate) contentLog: u32,
    pub(crate) hashLog: u32,
    pub(crate) searchLog: u32,
    pub(crate) searchLength: u32,
    pub(crate) targetLength: u32,
    pub(crate) strategy: ZSTDv05_strategy,
}
pub(crate) type ZSTDv05_strategy = core::ffi::c_uint;
pub(crate) const ZSTDv05_fast: ZSTDv05_strategy = 0;
type FSEv05_DTable = core::ffi::c_uint;
#[repr(C)]
struct blockProperties_t {
    blockType: blockType_t,
    origSize: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct seq_t {
    litLength: size_t,
    matchLength: size_t,
    offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct seqState_t {
    DStream: BITv05_DStream_t,
    stateLL: FSEv05_DState_t,
    stateOffb: FSEv05_DState_t,
    stateML: FSEv05_DState_t,
    prevOffset: size_t,
    dumps: *const u8,
    dumpsEnd: *const u8,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
struct FSEv05_DState_t {
    state: size_t,
    table: *const core::ffi::c_void,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
struct BITv05_DStream_t {
    bitContainer: size_t,
    bitsConsumed: core::ffi::c_uint,
    ptr: *const core::ffi::c_char,
    start: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv05_decode_t {
    newState: core::ffi::c_ushort,
    symbol: core::ffi::c_uchar,
    nbBits: core::ffi::c_uchar,
}
type BITv05_DStream_status = core::ffi::c_uint;
const BITv05_DStream_overflow: BITv05_DStream_status = 3;
const BITv05_DStream_completed: BITv05_DStream_status = 2;
const BITv05_DStream_endOfBuffer: BITv05_DStream_status = 1;
const BITv05_DStream_unfinished: BITv05_DStream_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv05_DTableHeader {
    tableLog: u16,
    fastMode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct HUFv05_DEltX4 {
    sequence: u16,
    nbBits: u8,
    length: u8,
}
type decompressionAlgo =
    Option<unsafe fn(*mut core::ffi::c_void, size_t, *const core::ffi::c_void, size_t) -> size_t>;
type rankVal_t = [[u32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
struct sortedSymbol_t {
    symbol: u8,
    weight: u8,
}
type DTable_max_t = [core::ffi::c_uint; 4097];
#[derive(Copy, Clone)]
#[repr(C)]
struct HUFv05_DEltX2 {
    byte: u8,
    nbBits: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct algo_time_t {
    tableTime: u32,
    decode256Time: u32,
}
#[repr(C)]
pub(crate) struct ZBUFFv05_DCtx {
    zc: *mut ZSTDv05_DCtx,
    params: ZSTDv05_parameters,
    inBuff: *mut core::ffi::c_char,
    inBuffSize: size_t,
    inPos: size_t,
    outBuff: *mut core::ffi::c_char,
    outBuffSize: size_t,
    outStart: size_t,
    outEnd: size_t,
    hPos: size_t,
    stage: ZBUFFv05_dStage,
    headerBuffer: [core::ffi::c_uchar; 5],
}
type ZBUFFv05_dStage = core::ffi::c_uint;
const ZBUFFv05ds_flush: ZBUFFv05_dStage = 6;
const ZBUFFv05ds_load: ZBUFFv05_dStage = 5;
const ZBUFFv05ds_read: ZBUFFv05_dStage = 4;
const ZBUFFv05ds_decodeHeader: ZBUFFv05_dStage = 3;
const ZBUFFv05ds_loadHeader: ZBUFFv05_dStage = 2;
const ZBUFFv05ds_readHeader: ZBUFFv05_dStage = 1;
const ZBUFFv05ds_init: ZBUFFv05_dStage = 0;
const ZSTDv05_MAGICNUMBER: core::ffi::c_uint = 0xfd2fb525 as core::ffi::c_uint;
const ZSTDv05_WINDOWLOG_ABSOLUTEMIN: core::ffi::c_int = 11;
const ZSTDv05_DICT_MAGIC: core::ffi::c_uint = 0xec30a435 as core::ffi::c_uint;
const BLOCKSIZE: core::ffi::c_int = 128 * ((1) << 10);
static ZSTDv05_blockHeaderSize: size_t = 3;
static ZSTDv05_frameHeaderSize_min: size_t = 5;
const ZSTDv05_frameHeaderSize_max: core::ffi::c_int = 5;
const IS_HUFv05: core::ffi::c_int = 0;
const IS_PCH: core::ffi::c_int = 1;
const IS_RAW: core::ffi::c_int = 2;
const IS_RLE: core::ffi::c_int = 3;
const MINMATCH: core::ffi::c_int = 4;
const REPCODE_STARTVALUE: core::ffi::c_int = 1;
const MLbits: core::ffi::c_int = 7;
const LLbits: core::ffi::c_int = 6;
const Offbits: core::ffi::c_int = 5;
const MaxML: core::ffi::c_int = (1 << MLbits) - 1;
const MaxLL: core::ffi::c_int = (1 << LLbits) - 1;
const MaxOff: core::ffi::c_int = (1 << Offbits) - 1;
const MLFSEv05Log: core::ffi::c_int = 10;
const LLFSEv05Log: core::ffi::c_int = 10;
const OffFSEv05Log: core::ffi::c_int = 9;
const ZSTD_HUFFDTABLE_CAPACITY_LOG: core::ffi::c_int = 12;
const MIN_SEQUENCES_SIZE: core::ffi::c_int = 1;
const MIN_CBLOCK_SIZE: core::ffi::c_int = 1 + 1 + MIN_SEQUENCES_SIZE;
const WILDCOPY_OVERLENGTH: core::ffi::c_int = 8;
const ZSTD_CONTENTSIZE_ERROR: core::ffi::c_ulonglong =
    (0 as core::ffi::c_ulonglong).wrapping_sub(2);
unsafe fn ZSTDv05_copy8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    memcpy(dst, src, 8);
}
#[inline]
unsafe fn ZSTDv05_wildcopy(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    length: ptrdiff_t,
) {
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length);
    loop {
        ZSTDv05_copy8(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
        op = op.offset(8);
        ip = ip.offset(8);
        if op >= oend {
            break;
        }
    }
}
#[inline]
fn BITv05_highbit32(val: u32) -> core::ffi::c_uint {
    (val.leading_zeros() as i32 ^ 31) as core::ffi::c_uint
}

#[inline]
unsafe fn BITv05_initDStream(
    bitD: &mut BITv05_DStream_t,
    srcBuffer: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize < 1 {
        *bitD = BITv05_DStream_t::default();
        return Error::srcSize_wrong.to_error_code();
    }

    if srcSize >= ::core::mem::size_of::<size_t>() {
        // normal case
        bitD.start = srcBuffer as *const core::ffi::c_char;
        bitD.ptr = (srcBuffer as *const core::ffi::c_char)
            .add(srcSize)
            .sub(::core::mem::size_of::<size_t>());
        bitD.bitContainer = MEM_readLEST(bitD.ptr as *const core::ffi::c_void);
        let contain32 = *(srcBuffer as *const u8).add(srcSize.wrapping_sub(1)) as u32;
        if contain32 == 0 {
            return Error::GENERIC.to_error_code(); // endMark not present
        }
        bitD.bitsConsumed = 8 - BITv05_highbit32(contain32);
    } else {
        bitD.start = srcBuffer as *const core::ffi::c_char;
        bitD.ptr = bitD.start;
        bitD.bitContainer = *(bitD.start as *const u8) as size_t;

        if srcSize == 7 {
            bitD.bitContainer += (*(bitD.start as *const u8).offset(6) as size_t)
                << (::core::mem::size_of::<size_t>() * 8 - 16);
        }

        if srcSize >= 6 {
            bitD.bitContainer += (*(bitD.start as *const u8).offset(5) as size_t)
                << (::core::mem::size_of::<size_t>() * 8 - 24);
        }

        if srcSize >= 5 {
            bitD.bitContainer += (*(bitD.start as *const u8).offset(4) as size_t)
                << (::core::mem::size_of::<size_t>() * 8 - 32);
        }

        if srcSize >= 4 {
            bitD.bitContainer += (*(bitD.start as *const u8).offset(3) as size_t) << 24;
        }

        if srcSize >= 3 {
            bitD.bitContainer += (*(bitD.start as *const u8).offset(2) as size_t) << 16;
        }

        if srcSize >= 2 {
            bitD.bitContainer += (*(bitD.start as *const u8).offset(1) as size_t) << 8;
        }

        let contain32 = *(srcBuffer as *const u8).add(srcSize - 1) as u32;
        if contain32 == 0 {
            // endMark not present
            return Error::GENERIC.to_error_code();
        }
        bitD.bitsConsumed = (8 as core::ffi::c_uint) - BITv05_highbit32(contain32);
        bitD.bitsConsumed += (::core::mem::size_of::<size_t>() - srcSize) as u32 * 8;
    }
    srcSize
}

#[inline]
fn BITv05_lookBits(bitD: &mut BITv05_DStream_t, nbBits: u32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    bitD.bitContainer << (bitD.bitsConsumed & bitMask)
        >> 1
        >> (bitMask.wrapping_sub(nbBits) & bitMask)
}
#[inline]
fn BITv05_lookBitsFast(bitD: &mut BITv05_DStream_t, nbBits: u32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>())
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    bitD.bitContainer << (bitD.bitsConsumed & bitMask)
        >> (bitMask.wrapping_add(1).wrapping_sub(nbBits) & bitMask)
}
#[inline]
fn BITv05_skipBits(bitD: &mut BITv05_DStream_t, nbBits: u32) {
    bitD.bitsConsumed += nbBits;
}
#[inline]
fn BITv05_readBits(bitD: &mut BITv05_DStream_t, nbBits: core::ffi::c_uint) -> size_t {
    let value = BITv05_lookBits(bitD, nbBits);
    BITv05_skipBits(bitD, nbBits);
    value
}
#[inline]
fn BITv05_readBitsFast(bitD: &mut BITv05_DStream_t, nbBits: core::ffi::c_uint) -> size_t {
    let value = BITv05_lookBitsFast(bitD, nbBits);
    BITv05_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe fn BITv05_reloadDStream(bitD: &mut BITv05_DStream_t) -> BITv05_DStream_status {
    if bitD.bitsConsumed as size_t > (::core::mem::size_of::<size_t>()).wrapping_mul(8) {
        return BITv05_DStream_overflow;
    }
    if bitD.ptr >= (bitD.start).add(::core::mem::size_of::<size_t>()) {
        bitD.ptr = (bitD.ptr).offset(-((bitD.bitsConsumed >> 3) as isize));
        bitD.bitsConsumed &= 7;
        bitD.bitContainer = MEM_readLEST(bitD.ptr as *const core::ffi::c_void);
        return BITv05_DStream_unfinished;
    }
    if bitD.ptr == bitD.start {
        if (bitD.bitsConsumed as size_t) < (::core::mem::size_of::<size_t>()).wrapping_mul(8) {
            return BITv05_DStream_endOfBuffer;
        }
        return BITv05_DStream_completed;
    }
    let mut nbBytes = bitD.bitsConsumed >> 3;
    let mut result = BITv05_DStream_unfinished;
    if (bitD.ptr).offset(-(nbBytes as isize)) < bitD.start {
        nbBytes = (bitD.ptr).offset_from(bitD.start) as core::ffi::c_long as u32;
        result = BITv05_DStream_endOfBuffer;
    }
    bitD.ptr = (bitD.ptr).offset(-(nbBytes as isize));
    bitD.bitsConsumed = (bitD.bitsConsumed).wrapping_sub(nbBytes * 8);
    bitD.bitContainer = MEM_readLEST(bitD.ptr as *const core::ffi::c_void);
    result
}
#[inline]
fn BITv05_endOfDStream(DStream: &BITv05_DStream_t) -> core::ffi::c_uint {
    (DStream.ptr == DStream.start
        && DStream.bitsConsumed as size_t == (::core::mem::size_of::<size_t>()).wrapping_mul(8))
        as core::ffi::c_int as core::ffi::c_uint
}
#[inline]
unsafe fn FSEv05_initDState(
    DStatePtr: &mut FSEv05_DState_t,
    bitD: &mut BITv05_DStream_t,
    dt: *const FSEv05_DTable,
) {
    let ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const FSEv05_DTableHeader;
    DStatePtr.state = BITv05_readBits(bitD, (*DTableH).tableLog as core::ffi::c_uint);
    BITv05_reloadDStream(bitD);
    DStatePtr.table = dt.offset(1) as *const core::ffi::c_void;
}
#[inline]
unsafe fn FSEv05_peakSymbol(DStatePtr: &mut FSEv05_DState_t) -> u8 {
    let DInfo = *(DStatePtr.table as *const FSEv05_decode_t).add(DStatePtr.state);
    DInfo.symbol
}
#[inline]
unsafe fn FSEv05_decodeSymbol(
    DStatePtr: &mut FSEv05_DState_t,
    bitD: &mut BITv05_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *(DStatePtr.table as *const FSEv05_decode_t).add(DStatePtr.state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv05_readBits(bitD, nbBits);
    DStatePtr.state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe fn FSEv05_decodeSymbolFast(
    DStatePtr: &mut FSEv05_DState_t,
    bitD: &mut BITv05_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *(DStatePtr.table as *const FSEv05_decode_t).add(DStatePtr.state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BITv05_readBitsFast(bitD, nbBits);
    DStatePtr.state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
fn FSEv05_endOfDState(DStatePtr: &FSEv05_DState_t) -> core::ffi::c_uint {
    (DStatePtr.state == 0) as core::ffi::c_int as core::ffi::c_uint
}
const FSEv05_MAX_MEMORY_USAGE: core::ffi::c_int = 14;
const FSEv05_MAX_SYMBOL_VALUE: core::ffi::c_int = 255;
const FSEv05_MAX_TABLELOG: core::ffi::c_int = FSEv05_MAX_MEMORY_USAGE - 2;
const FSEv05_MIN_TABLELOG: core::ffi::c_int = 5;
const FSEv05_TABLELOG_ABSOLUTE_MAX: core::ffi::c_int = 15;
fn FSEv05_tableStep(tableSize: u32) -> u32 {
    (tableSize >> 1)
        .wrapping_add(tableSize >> 3)
        .wrapping_add(3)
}
unsafe fn FSEv05_buildDTable(
    dt: *mut FSEv05_DTable,
    normalizedCounter: *const core::ffi::c_short,
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
) -> size_t {
    let mut DTableH = FSEv05_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    let tdPtr = dt.offset(1) as *mut core::ffi::c_void;
    let tableDecode = tdPtr as *mut FSEv05_decode_t;
    let tableSize = ((1) << tableLog) as u32;
    let tableMask = tableSize.wrapping_sub(1);
    let step = FSEv05_tableStep(tableSize);
    let mut symbolNext: [u16; 256] = [0; 256];
    let mut position = 0u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;
    let mut noLarge = 1;
    if maxSymbolValue > FSEv05_MAX_SYMBOL_VALUE as core::ffi::c_uint {
        return Error::maxSymbolValue_tooLarge.to_error_code();
    }
    if tableLog > FSEv05_MAX_TABLELOG as core::ffi::c_uint {
        return Error::tableLog_tooLarge.to_error_code();
    }
    ptr::write_bytes(tableDecode as *mut u8, 0, maxSymbolValue as size_t + 1);
    DTableH.tableLog = tableLog as u16;
    for s in 0..maxSymbolValue + 1 {
        if *normalizedCounter.offset(s as isize) as core::ffi::c_int == -(1) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).symbol = s as u8;
            symbolNext[s as usize] = 1;
        } else {
            if *normalizedCounter.add(s as usize) >= largeLimit {
                noLarge = 0;
            }
            symbolNext[s as usize] = *normalizedCounter.add(s as usize) as u16;
        }
    }
    for s in 0..maxSymbolValue + 1 {
        let mut i: core::ffi::c_int = 0;
        i = 0;
        while i < *normalizedCounter.add(s as usize) as core::ffi::c_int {
            (*tableDecode.offset(position as isize)).symbol = s as u8;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
        }
    }
    if position != 0 {
        return Error::GENERIC.to_error_code();
    }
    let mut i_0: u32 = 0;
    i_0 = 0;
    while i_0 < tableSize {
        let symbol = (*tableDecode.offset(i_0 as isize)).symbol;
        let fresh1 = &mut (*symbolNext.as_mut_ptr().offset(symbol as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2;
        (*tableDecode.offset(i_0 as isize)).nbBits =
            tableLog.wrapping_sub(BITv05_highbit32(nextState as u32)) as u8;
        (*tableDecode.offset(i_0 as isize)).newState = (((nextState as core::ffi::c_int)
            << (*tableDecode.offset(i_0 as isize)).nbBits as core::ffi::c_int)
            as u32)
            .wrapping_sub(tableSize) as u16;
        i_0 = i_0.wrapping_add(1);
    }
    DTableH.fastMode = noLarge as u16;
    memcpy(
        dt as *mut core::ffi::c_void,
        &mut DTableH as *mut FSEv05_DTableHeader as *const core::ffi::c_void,
        ::core::mem::size_of::<FSEv05_DTableHeader>(),
    );
    0
}
fn FSEv05_abs(a: core::ffi::c_short) -> core::ffi::c_short {
    a.abs()
}
unsafe fn FSEv05_readNCount(
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
        return Error::srcSize_wrong.to_error_code();
    }
    bitStream = MEM_readLE32(ip as *const core::ffi::c_void);
    nbBits = (bitStream & 0xf as core::ffi::c_int as u32).wrapping_add(FSEv05_MIN_TABLELOG as u32)
        as core::ffi::c_int;
    if nbBits > FSEv05_TABLELOG_ABSOLUTE_MAX {
        return Error::tableLog_tooLarge.to_error_code();
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
                return Error::maxSymbolValue_tooSmall.to_error_code();
            }
            while charnum < n0 {
                let fresh3 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter.offset(fresh3 as isize) = 0;
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
        remaining -= FSEv05_abs(count) as core::ffi::c_int;
        let fresh4 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh4 as isize) = count;
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
        return Error::GENERIC.to_error_code();
    }
    *maxSVPtr = charnum.wrapping_sub(1);
    ip = ip.offset(((bitCount + 7) >> 3) as isize);
    if ip.offset_from(istart) as size_t > hbSize {
        return Error::srcSize_wrong.to_error_code();
    }
    ip.offset_from(istart) as size_t
}
unsafe fn FSEv05_buildDTable_rle(dt: *mut FSEv05_DTable, symbolValue: u8) -> size_t {
    let ptr = dt as *mut core::ffi::c_void;
    let DTableH = ptr as *mut FSEv05_DTableHeader;
    let dPtr = dt.offset(1) as *mut core::ffi::c_void;
    let cell = dPtr as *mut FSEv05_decode_t;
    (*DTableH).tableLog = 0;
    (*DTableH).fastMode = 0;
    (*cell).newState = 0;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0;
    0
}
unsafe fn FSEv05_buildDTable_raw(dt: *mut FSEv05_DTable, nbBits: core::ffi::c_uint) -> size_t {
    let ptr = dt as *mut core::ffi::c_void;
    let DTableH = ptr as *mut FSEv05_DTableHeader;
    let dPtr = dt.offset(1) as *mut core::ffi::c_void;
    let dinfo = dPtr as *mut FSEv05_decode_t;
    let tableSize = ((1) << nbBits) as core::ffi::c_uint;
    let tableMask = tableSize.wrapping_sub(1);
    let maxSymbolValue = tableMask;
    let mut s: core::ffi::c_uint = 0;
    if nbBits < 1 {
        return Error::GENERIC.to_error_code();
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
unsafe fn FSEv05_decompress_usingDTable_generic(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    dt: *const FSEv05_DTable,
    fast: core::ffi::c_uint,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.add(maxDstSize);
    let olimit = omax.offset(-(3));
    let mut bitD = BITv05_DStream_t::default();
    let mut state1 = FSEv05_DState_t::default();
    let mut state2 = FSEv05_DState_t::default();
    let mut errorCode: size_t = 0;
    errorCode = BITv05_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    FSEv05_initDState(&mut state1, &mut bitD, dt);
    FSEv05_initDState(&mut state2, &mut bitD, dt);
    while BITv05_reloadDStream(&mut bitD) as core::ffi::c_uint
        == BITv05_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && op < olimit
    {
        *op.offset(0) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>()).wrapping_mul(8)
        {
            BITv05_reloadDStream(&mut bitD);
        }
        *op.offset(1) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 4 + 7) as size_t
            > (::core::mem::size_of::<size_t>()).wrapping_mul(8)
            && BITv05_reloadDStream(&mut bitD) as core::ffi::c_uint
                > BITv05_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        {
            op = op.offset(2);
            break;
        }
        *op.offset(2) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>()).wrapping_mul(8)
        {
            BITv05_reloadDStream(&mut bitD);
        }
        *op.offset(3) = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        op = op.offset(4);
    }
    while !(BITv05_reloadDStream(&mut bitD) as core::ffi::c_uint
        > BITv05_DStream_completed as core::ffi::c_int as core::ffi::c_uint
        || op == omax
        || BITv05_endOfDStream(&bitD) != 0 && (fast != 0 || FSEv05_endOfDState(&state1) != 0))
    {
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if BITv05_reloadDStream(&mut bitD) as core::ffi::c_uint
            > BITv05_DStream_completed as core::ffi::c_int as core::ffi::c_uint
            || op == omax
            || BITv05_endOfDStream(&bitD) != 0 && (fast != 0 || FSEv05_endOfDState(&state2) != 0)
        {
            break;
        }
        let fresh6 = op;
        op = op.offset(1);
        *fresh6 = (if fast != 0 {
            FSEv05_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv05_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
    }
    if BITv05_endOfDStream(&bitD) != 0
        && FSEv05_endOfDState(&state1) != 0
        && FSEv05_endOfDState(&state2) != 0
    {
        return op.offset_from(ostart) as size_t;
    }
    if op == omax {
        return Error::dstSize_tooSmall.to_error_code();
    }
    Error::corruption_detected.to_error_code()
}
unsafe fn FSEv05_decompress_usingDTable(
    dst: *mut core::ffi::c_void,
    originalSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    dt: *const FSEv05_DTable,
) -> size_t {
    let ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const FSEv05_DTableHeader;
    let fastMode = (*DTableH).fastMode as u32;
    if fastMode != 0 {
        return FSEv05_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 1);
    }
    FSEv05_decompress_usingDTable_generic(dst, originalSize, cSrc, cSrcSize, dt, 0)
}
unsafe fn FSEv05_decompress(
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
    let mut maxSymbolValue = FSEv05_MAX_SYMBOL_VALUE as core::ffi::c_uint;
    let mut errorCode: size_t = 0;
    if cSrcSize < 2 {
        return Error::srcSize_wrong.to_error_code();
    }
    errorCode = FSEv05_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const core::ffi::c_void,
        cSrcSize,
    );
    if ERR_isError(errorCode) {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return Error::srcSize_wrong.to_error_code();
    }
    ip = ip.add(errorCode);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    errorCode = FSEv05_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(errorCode) {
        return errorCode;
    }
    FSEv05_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    )
}
const HUFv05_ABSOLUTEMAX_TABLELOG: core::ffi::c_int = 16;
const HUFv05_MAX_TABLELOG: core::ffi::c_int = 12;
const HUFv05_MAX_SYMBOL_VALUE: core::ffi::c_int = 255;
unsafe fn HUFv05_readStats(
    huffWeight: *mut u8,
    hwSize: size_t,
    rankStats: *mut u32,
    nbSymbolsPtr: *mut u32,
    tableLogPtr: *mut u32,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut weightTotal: u32 = 0;
    let mut tableLog: u32 = 0;
    let mut ip = src as *const u8;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    let mut n: u32 = 0;
    if srcSize == 0 {
        return Error::srcSize_wrong.to_error_code();
    }
    iSize = *ip.offset(0) as size_t;
    if iSize >= 128 {
        if iSize >= 242 {
            static l: [core::ffi::c_int; 14] = [1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128];
            oSize = l[iSize.wrapping_sub(242)] as size_t;
            core::ptr::write_bytes(huffWeight, 1, hwSize);
            iSize = 0;
        } else {
            oSize = iSize.wrapping_sub(127);
            iSize = oSize.wrapping_add(1) / 2;
            if iSize.wrapping_add(1) > srcSize {
                return Error::srcSize_wrong.to_error_code();
            }
            if oSize >= hwSize {
                return Error::corruption_detected.to_error_code();
            }
            ip = ip.offset(1);
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
            return Error::srcSize_wrong.to_error_code();
        }
        oSize = FSEv05_decompress(
            huffWeight as *mut core::ffi::c_void,
            hwSize.wrapping_sub(1),
            ip.offset(1) as *const core::ffi::c_void,
            iSize,
        );
        if ERR_isError(oSize) {
            return oSize;
        }
    }
    core::ptr::write_bytes(rankStats, 0, (HUFv05_ABSOLUTEMAX_TABLELOG + 1) as size_t);
    weightTotal = 0;
    n = 0;
    while (n as size_t) < oSize {
        if *huffWeight.offset(n as isize) as core::ffi::c_int >= HUFv05_ABSOLUTEMAX_TABLELOG {
            return Error::corruption_detected.to_error_code();
        }
        let fresh7 = &mut (*rankStats.offset(*huffWeight.offset(n as isize) as isize));
        *fresh7 = (*fresh7).wrapping_add(1);
        weightTotal = weightTotal
            .wrapping_add(((1) << *huffWeight.offset(n as isize) as core::ffi::c_int >> 1) as u32);
        n = n.wrapping_add(1);
    }
    if weightTotal == 0 {
        return Error::corruption_detected.to_error_code();
    }
    tableLog = (BITv05_highbit32(weightTotal)).wrapping_add(1);
    if tableLog > HUFv05_ABSOLUTEMAX_TABLELOG as u32 {
        return Error::corruption_detected.to_error_code();
    }
    let total = ((1) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1) << BITv05_highbit32(rest)) as u32;
    let lastWeight = (BITv05_highbit32(rest)).wrapping_add(1);
    if verif != rest {
        return Error::corruption_detected.to_error_code();
    }
    *huffWeight.add(oSize) = lastWeight as u8;
    let fresh8 = &mut (*rankStats.offset(lastWeight as isize));
    *fresh8 = (*fresh8).wrapping_add(1);
    if *rankStats.offset(1) < 2 || *rankStats.offset(1) & 1 != 0 {
        return Error::corruption_detected.to_error_code();
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    *tableLogPtr = tableLog;
    iSize.wrapping_add(1)
}
unsafe fn HUFv05_readDTableX2(
    DTable: *mut u16,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 17] = [0; 17];
    let mut tableLog = 0;
    let mut iSize: size_t = 0;
    let mut nbSymbols = 0;
    let mut n: u32 = 0;
    let mut nextRankStart: u32 = 0;
    let dtPtr = DTable.offset(1) as *mut core::ffi::c_void;
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
    if ERR_isError(iSize) {
        return iSize;
    }
    if tableLog > *DTable.offset(0) as u32 {
        return Error::tableLog_tooLarge.to_error_code();
    }
    *DTable.offset(0) = tableLog as u16;
    nextRankStart = 0;
    n = 1;
    while n <= tableLog {
        let current = nextRankStart;
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
unsafe fn HUFv05_decodeSymbolX2(
    Dstream: &mut BITv05_DStream_t,
    dt: *const HUFv05_DEltX2,
    dtLog: u32,
) -> u8 {
    let val = BITv05_lookBitsFast(Dstream, dtLog);
    let c = (*dt.add(val)).byte;
    BITv05_skipBits(Dstream, (*dt.add(val)).nbBits as u32);
    c
}
#[inline]
unsafe fn HUFv05_decodeStreamX2(
    mut p: *mut u8,
    bitDPtr: &mut BITv05_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv05_DEltX2,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv05_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv05_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
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
    while BITv05_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv05_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
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
    pEnd.offset_from(pStart) as size_t
}
unsafe fn HUFv05_decompress1X2_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const u16,
) -> size_t {
    let op = dst as *mut u8;
    let oend = op.add(dstSize);
    let dtLog = *DTable.offset(0) as u32;
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX2).offset(1);
    let mut bitD = BITv05_DStream_t::default();
    if dstSize <= cSrcSize {
        return Error::dstSize_tooSmall.to_error_code();
    }
    let errorCode = BITv05_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    HUFv05_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if BITv05_endOfDStream(&bitD) == 0 {
        return Error::corruption_detected.to_error_code();
    }
    dstSize
}
unsafe fn HUFv05_decompress1X2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [core::ffi::c_ushort; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let mut errorCode: size_t = 0;
    errorCode = HUFv05_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return Error::srcSize_wrong.to_error_code();
    }
    ip = ip.add(errorCode);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv05_decompress1X2_usingDTable(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
unsafe fn HUFv05_decompress4X2_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const u16,
) -> size_t {
    if cSrcSize < 10 {
        return Error::corruption_detected.to_error_code();
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX2).offset(1);
    let dtLog = *DTable.offset(0) as u32;
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv05_DStream_t::default();
    let mut bitD2 = BITv05_DStream_t::default();
    let mut bitD3 = BITv05_DStream_t::default();
    let mut bitD4 = BITv05_DStream_t::default();
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
        return Error::corruption_detected.to_error_code();
    }
    errorCode = BITv05_initDStream(&mut bitD1, istart1 as *const core::ffi::c_void, length1);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD2, istart2 as *const core::ffi::c_void, length2);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD3, istart3 as *const core::ffi::c_void, length3);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD4, istart4 as *const core::ffi::c_void, length4);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    endSignal = BITv05_reloadDStream(&mut bitD1) as core::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD2) as core::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD3) as core::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    while endSignal == BITv05_DStream_unfinished as core::ffi::c_int as u32
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
        endSignal = BITv05_reloadDStream(&mut bitD1) as core::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD2) as core::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD3) as core::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    }
    if op1 > opStart2 {
        return Error::corruption_detected.to_error_code();
    }
    if op2 > opStart3 {
        return Error::corruption_detected.to_error_code();
    }
    if op3 > opStart4 {
        return Error::corruption_detected.to_error_code();
    }
    HUFv05_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv05_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv05_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv05_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv05_endOfDStream(&bitD1)
        & BITv05_endOfDStream(&bitD2)
        & BITv05_endOfDStream(&bitD3)
        & BITv05_endOfDStream(&bitD4);
    if endSignal == 0 {
        return Error::corruption_detected.to_error_code();
    }
    dstSize
}
unsafe fn HUFv05_decompress4X2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [core::ffi::c_ushort; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let mut errorCode: size_t = 0;
    errorCode = HUFv05_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return Error::srcSize_wrong.to_error_code();
    }
    ip = ip.add(errorCode);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv05_decompress4X2_usingDTable(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
unsafe fn HUFv05_fillDTableX4Level2(
    DTable: *mut HUFv05_DEltX4,
    sizeLog: u32,
    consumed: u32,
    rankValOrigin: *const u32,
    minWeight: core::ffi::c_int,
    sortedSymbols: *const sortedSymbol_t,
    sortedListSize: u32,
    nbBitsBaseline: u32,
    baseSeq: u16,
) {
    let mut DElt = HUFv05_DEltX4 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let mut rankVal: [u32; 17] = [0; 17];
    let mut s: u32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut core::ffi::c_void,
        rankValOrigin as *const core::ffi::c_void,
        ::core::mem::size_of::<[u32; 17]>(),
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
unsafe fn HUFv05_fillDTableX4(
    DTable: *mut HUFv05_DEltX4,
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
        ::core::mem::size_of::<[u32; 17]>(),
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
                &mut DElt.sequence as *mut u16 as *mut core::ffi::c_void,
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
unsafe fn HUFv05_readDTableX4(
    DTable: *mut core::ffi::c_uint,
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
    let memLog = *DTable.offset(0);
    let mut iSize: size_t = 0;
    let dtPtr = DTable as *mut core::ffi::c_void;
    let dt = (dtPtr as *mut HUFv05_DEltX4).offset(1);
    if memLog > HUFv05_ABSOLUTEMAX_TABLELOG as u32 {
        return Error::tableLog_tooLarge.to_error_code();
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
    if ERR_isError(iSize) {
        return iSize;
    }
    if tableLog > memLog {
        return Error::tableLog_tooLarge.to_error_code();
    }
    maxW = tableLog;
    while *rankStats.as_mut_ptr().offset(maxW as isize) == 0 {
        maxW = maxW.wrapping_sub(1);
    }
    let mut w: u32 = 0;
    let mut nextRankStart = 0u32;
    w = 1;
    while w <= maxW {
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
        let fresh35 = &mut (*rankStart.offset(w_0 as isize));
        let fresh36 = *fresh35;
        *fresh35 = (*fresh35).wrapping_add(1);
        let r = fresh36;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).symbol = s as u8;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).weight = w_0 as u8;
        s = s.wrapping_add(1);
    }
    *rankStart.offset(0) = 0;
    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut nextRankVal = 0u32;
    let mut w_1: u32 = 0;
    let mut consumed: u32 = 0;
    let rescale = memLog.wrapping_sub(tableLog).wrapping_sub(1) as core::ffi::c_int;
    let rankVal0 = (*rankVal.as_mut_ptr().offset(0)).as_mut_ptr();
    w_1 = 1;
    while w_1 <= maxW {
        let current_0 = nextRankVal;
        nextRankVal = nextRankVal.wrapping_add(
            *rankStats.as_mut_ptr().offset(w_1 as isize) << w_1.wrapping_add(rescale as u32),
        );
        *rankVal0.offset(w_1 as isize) = current_0;
        w_1 = w_1.wrapping_add(1);
    }
    consumed = minBits;
    while consumed <= memLog.wrapping_sub(minBits) {
        let rankValPtr = (*rankVal.as_mut_ptr().offset(consumed as isize)).as_mut_ptr();
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
unsafe fn HUFv05_decodeSymbolX4(
    op: *mut core::ffi::c_void,
    DStream: &mut BITv05_DStream_t,
    dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv05_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.add(val) as *const core::ffi::c_void, 2);
    BITv05_skipBits(DStream, (*dt.add(val)).nbBits as u32);
    (*dt.add(val)).length as u32
}
unsafe fn HUFv05_decodeLastSymbolX4(
    op: *mut core::ffi::c_void,
    DStream: &mut BITv05_DStream_t,
    dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = BITv05_lookBitsFast(DStream, dtLog);
    memcpy(op, dt.add(val) as *const core::ffi::c_void, 1);
    if (*dt.add(val)).length as core::ffi::c_int == 1 {
        BITv05_skipBits(DStream, (*dt.add(val)).nbBits as u32);
    } else if (DStream.bitsConsumed as size_t) < (::core::mem::size_of::<size_t>()).wrapping_mul(8)
    {
        BITv05_skipBits(DStream, (*dt.add(val)).nbBits as u32);
        if DStream.bitsConsumed as size_t > (::core::mem::size_of::<size_t>()).wrapping_mul(8) {
            DStream.bitsConsumed =
                (::core::mem::size_of::<size_t>()).wrapping_mul(8) as core::ffi::c_uint;
        }
    }
    1
}
#[inline]
unsafe fn HUFv05_decodeStreamX4(
    mut p: *mut u8,
    bitDPtr: &mut BITv05_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    while BITv05_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv05_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p < pEnd.offset(-(7))
    {
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv05_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            p = p.offset(
                HUFv05_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv05_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        p = p.offset(
            HUFv05_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while BITv05_reloadDStream(bitDPtr) as core::ffi::c_uint
        == BITv05_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        && p <= pEnd.offset(-(2))
    {
        p = p.offset(
            HUFv05_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while p <= pEnd.offset(-(2)) {
        p = p.offset(
            HUFv05_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    if p < pEnd {
        p = p.offset(
            HUFv05_decodeLastSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as size_t
}
unsafe fn HUFv05_decompress1X4_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const core::ffi::c_uint,
) -> size_t {
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtLog = *DTable.offset(0);
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX4).offset(1);
    let mut errorCode: size_t = 0;
    let mut bitD = BITv05_DStream_t::default();
    errorCode = BITv05_initDStream(&mut bitD, istart as *const core::ffi::c_void, cSrcSize);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    HUFv05_decodeStreamX4(ostart, &mut bitD, oend, dt, dtLog);
    if BITv05_endOfDStream(&bitD) == 0 {
        return Error::corruption_detected.to_error_code();
    }
    dstSize
}
unsafe fn HUFv05_decompress4X4_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const core::ffi::c_uint,
) -> size_t {
    if cSrcSize < 10 {
        return Error::corruption_detected.to_error_code();
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX4).offset(1);
    let dtLog = *DTable.offset(0);
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv05_DStream_t::default();
    let mut bitD2 = BITv05_DStream_t::default();
    let mut bitD3 = BITv05_DStream_t::default();
    let mut bitD4 = BITv05_DStream_t::default();
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
        return Error::corruption_detected.to_error_code();
    }
    errorCode = BITv05_initDStream(&mut bitD1, istart1 as *const core::ffi::c_void, length1);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD2, istart2 as *const core::ffi::c_void, length2);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD3, istart3 as *const core::ffi::c_void, length3);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    errorCode = BITv05_initDStream(&mut bitD4, istart4 as *const core::ffi::c_void, length4);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    endSignal = BITv05_reloadDStream(&mut bitD1) as core::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD2) as core::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD3) as core::ffi::c_uint
        | BITv05_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    while endSignal == BITv05_DStream_unfinished as core::ffi::c_int as u32
        && op4 < oend.offset(-(7))
    {
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv05_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv05_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv05_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv05_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op1 = op1.offset(HUFv05_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op2 = op2.offset(HUFv05_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op3 = op3.offset(HUFv05_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv05_MAX_TABLELOG <= 12 {
            op4 = op4.offset(HUFv05_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv05_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv05_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv05_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv05_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        op1 = op1.offset(
            HUFv05_decodeSymbolX4(op1 as *mut core::ffi::c_void, &mut bitD1, dt, dtLog) as isize,
        );
        op2 = op2.offset(
            HUFv05_decodeSymbolX4(op2 as *mut core::ffi::c_void, &mut bitD2, dt, dtLog) as isize,
        );
        op3 = op3.offset(
            HUFv05_decodeSymbolX4(op3 as *mut core::ffi::c_void, &mut bitD3, dt, dtLog) as isize,
        );
        op4 = op4.offset(
            HUFv05_decodeSymbolX4(op4 as *mut core::ffi::c_void, &mut bitD4, dt, dtLog) as isize,
        );
        endSignal = BITv05_reloadDStream(&mut bitD1) as core::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD2) as core::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD3) as core::ffi::c_uint
            | BITv05_reloadDStream(&mut bitD4) as core::ffi::c_uint;
    }
    if op1 > opStart2 {
        return Error::corruption_detected.to_error_code();
    }
    if op2 > opStart3 {
        return Error::corruption_detected.to_error_code();
    }
    if op3 > opStart4 {
        return Error::corruption_detected.to_error_code();
    }
    HUFv05_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv05_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv05_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv05_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv05_endOfDStream(&bitD1)
        & BITv05_endOfDStream(&bitD2)
        & BITv05_endOfDStream(&bitD3)
        & BITv05_endOfDStream(&bitD4);
    if endSignal == 0 {
        return Error::corruption_detected.to_error_code();
    }
    dstSize
}
unsafe fn HUFv05_decompress4X4(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [core::ffi::c_uint; 4097] = [12; 4097];
    let mut ip = cSrc as *const u8;
    let hSize = HUFv05_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if ERR_isError(hSize) {
        return hSize;
    }
    if hSize >= cSrcSize {
        return Error::srcSize_wrong.to_error_code();
    }
    ip = ip.add(hSize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv05_decompress4X4_usingDTable(
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
unsafe fn HUFv05_decompress(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
) -> size_t {
    static decompress: [decompressionAlgo; 3] = [
        Some(
            HUFv05_decompress4X2
                as unsafe fn(
                    *mut core::ffi::c_void,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                ) -> size_t,
        ),
        Some(
            HUFv05_decompress4X4
                as unsafe fn(
                    *mut core::ffi::c_void,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                ) -> size_t,
        ),
        None,
    ];
    let mut Q: u32 = 0;
    let D256 = (dstSize >> 8) as u32;
    let mut Dtime: [u32; 3] = [0; 3];
    let mut algoNb = 0;
    let mut n: core::ffi::c_int = 0;
    if dstSize == 0 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if cSrcSize >= dstSize {
        return Error::corruption_detected.to_error_code();
    }
    if cSrcSize == 1 {
        core::ptr::write_bytes(dst.cast::<u8>(), *(cSrc as *const u8), dstSize);
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
unsafe fn ZSTDv05_copy4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    memcpy(dst, src, 4);
}
unsafe fn ZSTDv05_decompressBegin(dctx: *mut ZSTDv05_DCtx) -> size_t {
    (*dctx).expected = ZSTDv05_frameHeaderSize_min;
    (*dctx).stage = ZSTDv05ds_getFrameHeaderSize;
    (*dctx).previousDstEnd = core::ptr::null();
    (*dctx).base = core::ptr::null();
    (*dctx).vBase = core::ptr::null();
    (*dctx).dictEnd = core::ptr::null();
    *((*dctx).hufTableX4).as_mut_ptr().offset(0) =
        ZSTD_HUFFDTABLE_CAPACITY_LOG as core::ffi::c_uint;
    (*dctx).flagStaticTables = 0;
    0
}
pub(crate) unsafe fn ZSTDv05_createDCtx() -> *mut ZSTDv05_DCtx {
    let dctx = malloc(::core::mem::size_of::<ZSTDv05_DCtx>()) as *mut ZSTDv05_DCtx;
    if dctx.is_null() {
        return core::ptr::null_mut();
    }
    ZSTDv05_decompressBegin(dctx);
    dctx
}
pub(crate) unsafe fn ZSTDv05_freeDCtx(dctx: *mut ZSTDv05_DCtx) -> size_t {
    free(dctx as *mut core::ffi::c_void);
    0
}
unsafe fn ZSTDv05_decodeFrameHeader_Part1(
    zc: &mut ZSTDv05_DCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut magicNumber: u32 = 0;
    if srcSize != ZSTDv05_frameHeaderSize_min {
        return Error::srcSize_wrong.to_error_code();
    }
    magicNumber = MEM_readLE32(src);
    if magicNumber != ZSTDv05_MAGICNUMBER {
        return Error::prefix_unknown.to_error_code();
    }
    zc.headerSize = ZSTDv05_frameHeaderSize_min;
    zc.headerSize
}
pub(crate) unsafe fn ZSTDv05_getFrameParams(
    params: *mut ZSTDv05_parameters,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut magicNumber: u32 = 0;
    if srcSize < ZSTDv05_frameHeaderSize_min {
        return ZSTDv05_frameHeaderSize_max as size_t;
    }
    magicNumber = MEM_readLE32(src);
    if magicNumber != ZSTDv05_MAGICNUMBER {
        return Error::prefix_unknown.to_error_code();
    }
    ptr::write_bytes(
        params as *mut u8,
        0,
        ::core::mem::size_of::<ZSTDv05_parameters>(),
    );
    (*params).windowLog = ((*(src as *const u8).offset(4) as core::ffi::c_int & 15)
        + ZSTDv05_WINDOWLOG_ABSOLUTEMIN) as u32;
    if *(src as *const u8).offset(4) as core::ffi::c_int >> 4 != 0 {
        return Error::frameParameter_unsupported.to_error_code();
    }
    0
}
unsafe fn ZSTDv05_decodeFrameHeader_Part2(
    zc: *mut ZSTDv05_DCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut result: size_t = 0;
    if srcSize != (*zc).headerSize {
        return Error::srcSize_wrong.to_error_code();
    }
    result = ZSTDv05_getFrameParams(&mut (*zc).params, src, srcSize);
    if MEM_32bits() != 0 && (*zc).params.windowLog > 25 {
        return Error::frameParameter_unsupported.to_error_code();
    }
    result
}
unsafe fn ZSTDv05_getcBlockSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
    bpPtr: *mut blockProperties_t,
) -> size_t {
    let in_0 = src as *const u8;
    let mut headerFlags: u8 = 0;
    let mut cSize: u32 = 0;
    if srcSize < 3 {
        return Error::srcSize_wrong.to_error_code();
    }
    headerFlags = *in_0;
    cSize = (*in_0.offset(2) as core::ffi::c_int
        + ((*in_0.offset(1) as core::ffi::c_int) << 8)
        + ((*in_0.offset(0) as core::ffi::c_int & 7) << 16)) as u32;
    (*bpPtr).blockType = (headerFlags as core::ffi::c_int >> 6) as blockType_t;
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
unsafe fn ZSTDv05_copyRawBlock(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if dst.is_null() {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if srcSize > maxDstSize {
        return Error::dstSize_tooSmall.to_error_code();
    }
    memcpy(dst, src, srcSize);
    srcSize
}
unsafe fn ZSTDv05_decodeLiteralsBlock(
    dctx: &mut ZSTDv05_DCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let istart = src as *const u8;
    if srcSize < MIN_CBLOCK_SIZE as size_t {
        return Error::corruption_detected.to_error_code();
    }
    match *istart.offset(0) as core::ffi::c_int >> 6 {
        IS_HUFv05 => {
            let mut litSize: size_t = 0;
            let mut litCSize: size_t = 0;
            let mut singleStream = 0;
            let mut lhSize = (*istart.offset(0) as core::ffi::c_int >> 4 & 3) as u32;
            if srcSize < 5 {
                return Error::corruption_detected.to_error_code();
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
            if litSize > BLOCKSIZE as size_t {
                return Error::corruption_detected.to_error_code();
            }
            if litCSize.wrapping_add(lhSize as size_t) > srcSize {
                return Error::corruption_detected.to_error_code();
            }
            if ERR_isError(if singleStream != 0 {
                HUFv05_decompress1X2(
                    dctx.litBuffer.as_mut_ptr() as *mut core::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const core::ffi::c_void,
                    litCSize,
                )
            } else {
                HUFv05_decompress(
                    dctx.litBuffer.as_mut_ptr() as *mut core::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const core::ffi::c_void,
                    litCSize,
                )
            }) {
                return Error::corruption_detected.to_error_code();
            }
            dctx.litPtr = dctx.litBuffer.as_mut_ptr();
            dctx.litSize = litSize;
            ptr::write_bytes(
                dctx.litBuffer.as_mut_ptr().add(dctx.litSize) as *mut u8,
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            litCSize.wrapping_add(lhSize as size_t)
        }
        IS_PCH => {
            let mut errorCode: size_t = 0;
            let mut litSize_0: size_t = 0;
            let mut litCSize_0: size_t = 0;
            let mut lhSize_0 = (*istart.offset(0) as core::ffi::c_int >> 4 & 3) as u32;
            if lhSize_0 != 1 {
                return Error::corruption_detected.to_error_code();
            }
            if dctx.flagStaticTables == 0 {
                return Error::dictionary_corrupted.to_error_code();
            }
            lhSize_0 = 3;
            litSize_0 = (((*istart.offset(0) as core::ffi::c_int & 15) << 6)
                + (*istart.offset(1) as core::ffi::c_int >> 2)) as size_t;
            litCSize_0 = (((*istart.offset(1) as core::ffi::c_int & 3) << 8)
                + *istart.offset(2) as core::ffi::c_int) as size_t;
            if litCSize_0.wrapping_add(lhSize_0 as size_t) > srcSize {
                return Error::corruption_detected.to_error_code();
            }
            errorCode = HUFv05_decompress1X4_usingDTable(
                dctx.litBuffer.as_mut_ptr() as *mut core::ffi::c_void,
                litSize_0,
                istart.offset(lhSize_0 as isize) as *const core::ffi::c_void,
                litCSize_0,
                dctx.hufTableX4.as_mut_ptr(),
            );
            if ERR_isError(errorCode) {
                return Error::corruption_detected.to_error_code();
            }
            dctx.litPtr = dctx.litBuffer.as_mut_ptr();
            dctx.litSize = litSize_0;
            ptr::write_bytes(
                (dctx.litBuffer).as_mut_ptr().add(dctx.litSize) as *mut u8,
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
                    return Error::corruption_detected.to_error_code();
                }
                memcpy(
                    dctx.litBuffer.as_mut_ptr() as *mut core::ffi::c_void,
                    istart.offset(lhSize_1 as isize) as *const core::ffi::c_void,
                    litSize_1,
                );
                dctx.litPtr = dctx.litBuffer.as_mut_ptr();
                dctx.litSize = litSize_1;
                ptr::write_bytes(
                    dctx.litBuffer.as_mut_ptr().add(dctx.litSize) as *mut u8,
                    0,
                    WILDCOPY_OVERLENGTH as usize,
                );
                return (lhSize_1 as size_t).wrapping_add(litSize_1);
            }
            dctx.litPtr = istart.offset(lhSize_1 as isize);
            dctx.litSize = litSize_1;
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
                        return Error::corruption_detected.to_error_code();
                    }
                }
                0 | 1 | _ => {
                    lhSize_2 = 1;
                    litSize_2 = (*istart.offset(0) as core::ffi::c_int & 31) as size_t;
                }
            }
            if litSize_2 > BLOCKSIZE as size_t {
                return Error::corruption_detected.to_error_code();
            }
            core::ptr::write_bytes(
                dctx.litBuffer.as_mut_ptr(),
                *istart.offset(lhSize_2 as isize),
                litSize_2.wrapping_add(WILDCOPY_OVERLENGTH as size_t),
            );
            dctx.litPtr = dctx.litBuffer.as_mut_ptr();
            dctx.litSize = litSize_2;
            lhSize_2.wrapping_add(1) as size_t
        }
        _ => Error::corruption_detected.to_error_code(),
    }
}
unsafe fn ZSTDv05_decodeSeqHeaders(
    nbSeq: *mut core::ffi::c_int,
    dumpsPtr: *mut *const u8,
    dumpsLengthPtr: *mut size_t,
    DTableLL: *mut FSEv05_DTable,
    DTableML: *mut FSEv05_DTable,
    DTableOffb: *mut FSEv05_DTable,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    flagStaticTable: u32,
) -> size_t {
    let istart = src as *const u8;
    let mut ip = istart;
    let iend = istart.add(srcSize);
    let mut LLtype: u32 = 0;
    let mut Offtype: u32 = 0;
    let mut MLtype: u32 = 0;
    let mut LLlog: core::ffi::c_uint = 0;
    let mut Offlog: core::ffi::c_uint = 0;
    let mut MLlog: core::ffi::c_uint = 0;
    let mut dumpsLength: size_t = 0;
    if srcSize < MIN_SEQUENCES_SIZE as size_t {
        return Error::srcSize_wrong.to_error_code();
    }
    let fresh39 = ip;
    ip = ip.offset(1);
    *nbSeq = *fresh39 as core::ffi::c_int;
    if *nbSeq == 0 {
        return 1;
    }
    if *nbSeq >= 128 {
        if ip >= iend {
            return Error::srcSize_wrong.to_error_code();
        }
        let fresh40 = ip;
        ip = ip.offset(1);
        *nbSeq = ((*nbSeq.offset(0) - 128) << 8) + *fresh40 as core::ffi::c_int;
    }
    if ip >= iend {
        return Error::srcSize_wrong.to_error_code();
    }
    LLtype = (*ip as core::ffi::c_int >> 6) as u32;
    Offtype = (*ip as core::ffi::c_int >> 4 & 3) as u32;
    MLtype = (*ip as core::ffi::c_int >> 2 & 3) as u32;
    if *ip as core::ffi::c_int & 2 != 0 {
        if ip.offset(3) > iend {
            return Error::srcSize_wrong.to_error_code();
        }
        dumpsLength = *ip.offset(2) as size_t;
        dumpsLength =
            dumpsLength.wrapping_add(((*ip.offset(1) as core::ffi::c_int) << 8) as size_t);
        ip = ip.offset(3);
    } else {
        if ip.offset(2) > iend {
            return Error::srcSize_wrong.to_error_code();
        }
        dumpsLength = *ip.offset(1) as size_t;
        dumpsLength =
            dumpsLength.wrapping_add(((*ip.offset(0) as core::ffi::c_int & 1) << 8) as size_t);
        ip = ip.offset(2);
    }
    *dumpsPtr = ip;
    ip = ip.add(dumpsLength);
    *dumpsLengthPtr = dumpsLength;
    if ip > iend.offset(-(3)) {
        return Error::srcSize_wrong.to_error_code();
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
            LLlog = LLbits as core::ffi::c_uint;
            FSEv05_buildDTable_raw(DTableLL, LLbits as core::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return Error::corruption_detected.to_error_code();
            }
        }
        3 | _ => {
            let mut max = MaxLL as core::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut LLlog,
                ip as *const core::ffi::c_void,
                iend.offset_from(ip) as size_t,
            );
            if ERR_isError(headerSize) {
                return Error::GENERIC.to_error_code();
            }
            if LLlog > LLFSEv05Log as core::ffi::c_uint {
                return Error::corruption_detected.to_error_code();
            }
            ip = ip.add(headerSize);
            FSEv05_buildDTable(DTableLL, norm.as_mut_ptr(), max, LLlog);
        }
    }
    match Offtype {
        1 => {
            Offlog = 0;
            if ip > iend.offset(-(2)) {
                return Error::srcSize_wrong.to_error_code();
            }
            let fresh42 = ip;
            ip = ip.offset(1);
            FSEv05_buildDTable_rle(
                DTableOffb,
                *fresh42 as core::ffi::c_uchar & (MaxOff as core::ffi::c_uchar),
            );
        }
        0 => {
            Offlog = Offbits as core::ffi::c_uint;
            FSEv05_buildDTable_raw(DTableOffb, Offbits as core::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return Error::corruption_detected.to_error_code();
            }
        }
        3 | _ => {
            let mut max = MaxOff as core::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut Offlog,
                ip as *const core::ffi::c_void,
                iend.offset_from(ip) as size_t,
            );
            if ERR_isError(headerSize) {
                return Error::GENERIC.to_error_code();
            }
            if Offlog > OffFSEv05Log as core::ffi::c_uint {
                return Error::corruption_detected.to_error_code();
            }
            ip = ip.add(headerSize);
            FSEv05_buildDTable(DTableOffb, norm.as_mut_ptr(), max, Offlog);
        }
    }
    match MLtype {
        1 => {
            MLlog = 0;
            if ip > iend.offset(-(2)) {
                return Error::srcSize_wrong.to_error_code();
            }
            let fresh43 = ip;
            ip = ip.offset(1);
            FSEv05_buildDTable_rle(DTableML, *fresh43);
        }
        0 => {
            MLlog = MLbits as core::ffi::c_uint;
            FSEv05_buildDTable_raw(DTableML, MLbits as core::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return Error::corruption_detected.to_error_code();
            }
        }
        3 | _ => {
            let mut max = MaxML as core::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                norm.as_mut_ptr(),
                &mut max,
                &mut MLlog,
                ip as *const core::ffi::c_void,
                iend.offset_from(ip) as size_t,
            );
            if ERR_isError(headerSize) {
                return Error::GENERIC.to_error_code();
            }
            if MLlog > MLFSEv05Log as core::ffi::c_uint {
                return Error::corruption_detected.to_error_code();
            }
            ip = ip.add(headerSize);
            FSEv05_buildDTable(DTableML, norm.as_mut_ptr(), max, MLlog);
        }
    }
    ip.offset_from(istart) as size_t
}
unsafe fn ZSTDv05_decodeSequence(seq: &mut seq_t, seqState: &mut seqState_t) {
    let mut litLength: size_t = 0;
    let mut prevOffset: size_t = 0;
    let mut offset: size_t = 0;
    let mut matchLength: size_t = 0;
    let mut dumps = seqState.dumps;
    let de = seqState.dumpsEnd;
    litLength = FSEv05_peakSymbol(&mut seqState.stateLL) as size_t;
    prevOffset = if litLength != 0 {
        seq.offset
    } else {
        seqState.prevOffset
    };
    if litLength == MaxLL as size_t {
        let fresh44 = dumps;
        dumps = dumps.offset(1);
        let add = *fresh44 as u32;
        if add < 255 {
            litLength = litLength.wrapping_add(add as size_t);
        } else if dumps.offset(2) <= de {
            litLength = MEM_readLE16(dumps as *const core::ffi::c_void) as size_t;
            dumps = dumps.offset(2);
            if litLength & 1 != 0 && dumps < de {
                litLength = litLength.wrapping_add(((*dumps as core::ffi::c_int) << 16) as size_t);
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
    let offsetCode = FSEv05_peakSymbol(&mut seqState.stateOffb) as u32;
    let mut nbBits = offsetCode.wrapping_sub(1);
    if offsetCode == 0 {
        nbBits = 0;
    }
    offset = (*offsetPrefix.as_ptr().offset(offsetCode as isize) as size_t)
        .wrapping_add(BITv05_readBits(&mut seqState.DStream, nbBits));
    if MEM_32bits() != 0 {
        BITv05_reloadDStream(&mut seqState.DStream);
    }
    if offsetCode == 0 {
        offset = prevOffset;
    }
    if offsetCode | (litLength == 0) as core::ffi::c_int as u32 != 0 {
        seqState.prevOffset = seq.offset;
    }
    FSEv05_decodeSymbol(&mut seqState.stateOffb, &mut seqState.DStream);
    FSEv05_decodeSymbol(&mut seqState.stateLL, &mut seqState.DStream);
    if MEM_32bits() != 0 {
        BITv05_reloadDStream(&mut seqState.DStream);
    }
    matchLength = FSEv05_decodeSymbol(&mut seqState.stateML, &mut seqState.DStream) as size_t;
    if matchLength == MaxML as size_t {
        let add_0 = (if dumps < de {
            let fresh45 = dumps;
            dumps = dumps.offset(1);
            *fresh45 as core::ffi::c_int
        } else {
            0
        }) as u32;
        if add_0 < 255 {
            matchLength = matchLength.wrapping_add(add_0 as size_t);
        } else if dumps.offset(2) <= de {
            matchLength = MEM_readLE16(dumps as *const core::ffi::c_void) as size_t;
            dumps = dumps.offset(2);
            if matchLength & 1 != 0 && dumps < de {
                matchLength =
                    matchLength.wrapping_add(((*dumps as core::ffi::c_int) << 16) as size_t);
                dumps = dumps.offset(1);
            }
            matchLength >>= 1;
        }
        if dumps >= de {
            dumps = de.offset(-(1));
        }
    }
    matchLength = matchLength.wrapping_add(MINMATCH as size_t);
    seq.litLength = litLength;
    seq.offset = offset;
    seq.matchLength = matchLength;
    seqState.dumps = dumps;
}
unsafe fn ZSTDv05_execSequence(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    litPtr: *mut *const u8,
    litLimit: *const u8,
    base: *const u8,
    vBase: *const u8,
    dictEnd: *const u8,
) -> size_t {
    static dec32table: [core::ffi::c_int; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
    static dec64table: [core::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.add(sequenceLength);
    let oend_8 = oend.wrapping_sub(8);
    let litEnd = (*litPtr).add(sequence.litLength);
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset);
    let seqLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    if seqLength > oend.offset_from(op) as size_t {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as size_t {
        return Error::corruption_detected.to_error_code();
    }
    if oLitEnd > oend_8 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if oMatchEnd > oend {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if litEnd > litLimit {
        return Error::corruption_detected.to_error_code();
    }
    ZSTDv05_wildcopy(
        op as *mut core::ffi::c_void,
        *litPtr as *const core::ffi::c_void,
        sequence.litLength as ptrdiff_t,
    );
    op = oLitEnd;
    *litPtr = litEnd;
    if sequence.offset > oLitEnd.offset_from(base) as size_t {
        if sequence.offset > oLitEnd.offset_from(vBase) as size_t {
            return Error::corruption_detected.to_error_code();
        }
        match_0 = dictEnd.offset(-(base.offset_from(match_0) as core::ffi::c_long as isize));
        if match_0.add(sequence.matchLength) <= dictEnd {
            core::ptr::copy(match_0, oLitEnd, sequence.matchLength);
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as size_t;
        core::ptr::copy(match_0, oLitEnd, length1);
        op = oLitEnd.add(length1);
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
        let sub2 = *dec64table.as_ptr().add(sequence.offset);
        *op.offset(0) = *match_0.offset(0);
        *op.offset(1) = *match_0.offset(1);
        *op.offset(2) = *match_0.offset(2);
        *op.offset(3) = *match_0.offset(3);
        match_0 = match_0.offset(*dec32table.as_ptr().add(sequence.offset) as isize);
        ZSTDv05_copy4(
            op.offset(4) as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
        );
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ZSTDv05_copy8(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
        );
    }
    op = op.offset(8);
    match_0 = match_0.offset(8);
    if oMatchEnd > oend.offset(-((16 - MINMATCH) as isize)) {
        if op < oend_8 {
            ZSTDv05_wildcopy(
                op as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                oend_8.offset_from(op) as ptrdiff_t,
            );
            match_0 = match_0.offset(oend_8.offset_from(op) as core::ffi::c_long as isize);
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
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            sequence.matchLength as ptrdiff_t - 8,
        );
    }
    sequenceLength
}
unsafe fn ZSTDv05_decompressSequences(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: *const core::ffi::c_void,
    seqSize: size_t,
) -> size_t {
    let mut ip = seqStart as *const u8;
    let iend = ip.add(seqSize);
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(maxDstSize);
    let mut errorCode: size_t = 0;
    let mut dumpsLength = 0;
    let mut litPtr = dctx.litPtr;
    let litEnd = litPtr.add(dctx.litSize);
    let mut nbSeq = 0;
    let mut dumps = core::ptr::null();
    let DTableLL = dctx.LLTable.as_mut_ptr();
    let DTableML = dctx.MLTable.as_mut_ptr();
    let DTableOffb = dctx.OffTable.as_mut_ptr();
    let base = dctx.base as *const u8;
    let vBase = dctx.vBase as *const u8;
    let dictEnd = dctx.dictEnd as *const u8;
    errorCode = ZSTDv05_decodeSeqHeaders(
        &mut nbSeq,
        &mut dumps,
        &mut dumpsLength,
        DTableLL,
        DTableML,
        DTableOffb,
        ip as *const core::ffi::c_void,
        seqSize,
        dctx.flagStaticTables,
    );
    if ERR_isError(errorCode) {
        return errorCode;
    }
    ip = ip.add(errorCode);
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BITv05_DStream_t::default(),
            stateLL: FSEv05_DState_t::default(),
            stateOffb: FSEv05_DState_t::default(),
            stateML: FSEv05_DState_t::default(),
            prevOffset: REPCODE_STARTVALUE as size_t,
            dumps,
            dumpsEnd: dumps.add(dumpsLength),
        };

        let mut sequence = seq_t {
            litLength: 0,
            matchLength: 0,
            offset: REPCODE_STARTVALUE as size_t,
        };
        errorCode = BITv05_initDStream(
            &mut seqState.DStream,
            ip as *const core::ffi::c_void,
            iend.offset_from(ip) as size_t,
        );
        if ERR_isError(errorCode) {
            return Error::corruption_detected.to_error_code();
        }
        FSEv05_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
        FSEv05_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
        FSEv05_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
        while BITv05_reloadDStream(&mut seqState.DStream) as core::ffi::c_uint
            <= BITv05_DStream_completed as core::ffi::c_int as core::ffi::c_uint
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
            if ERR_isError(oneSeqSize) {
                return oneSeqSize;
            }
            op = op.add(oneSeqSize);
        }
        if nbSeq != 0 {
            return Error::corruption_detected.to_error_code();
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as size_t;
    if litPtr > litEnd {
        return Error::corruption_detected.to_error_code();
    }
    if op.add(lastLLSize) > oend {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if lastLLSize > 0 {
        memcpy(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize,
        );
        op = op.add(lastLLSize);
    }
    op.offset_from(ostart) as size_t
}
unsafe fn ZSTDv05_checkContinuity(dctx: &mut ZSTDv05_DCtx, dst: *const core::ffi::c_void) {
    if dst != dctx.previousDstEnd {
        dctx.dictEnd = dctx.previousDstEnd;
        dctx.vBase = (dst as *const core::ffi::c_char).offset(
            -((dctx.previousDstEnd as *const core::ffi::c_char)
                .offset_from(dctx.base as *const core::ffi::c_char)
                as core::ffi::c_long as isize),
        ) as *const core::ffi::c_void;
        dctx.base = dst;
        dctx.previousDstEnd = dst;
    }
}
unsafe fn ZSTDv05_decompressBlock_internal(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    let mut litCSize: size_t = 0;
    if srcSize >= BLOCKSIZE as size_t {
        return Error::srcSize_wrong.to_error_code();
    }
    litCSize = ZSTDv05_decodeLiteralsBlock(dctx, src, srcSize);
    if ERR_isError(litCSize) {
        return litCSize;
    }
    ip = ip.add(litCSize);
    srcSize = srcSize.wrapping_sub(litCSize);
    ZSTDv05_decompressSequences(
        dctx,
        dst,
        dstCapacity,
        ip as *const core::ffi::c_void,
        srcSize,
    )
}
unsafe fn ZSTDv05_decompress_continueDCtx(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(maxDstSize);
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
        return Error::srcSize_wrong.to_error_code();
    }
    frameHeaderSize = ZSTDv05_decodeFrameHeader_Part1(dctx, src, ZSTDv05_frameHeaderSize_min);
    if ERR_isError(frameHeaderSize) {
        return frameHeaderSize;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv05_blockHeaderSize) {
        return Error::srcSize_wrong.to_error_code();
    }
    ip = ip.add(frameHeaderSize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    frameHeaderSize = ZSTDv05_decodeFrameHeader_Part2(dctx, src, frameHeaderSize);
    if ERR_isError(frameHeaderSize) {
        return frameHeaderSize;
    }
    loop {
        let mut decodedSize = 0;
        let cBlockSize = ZSTDv05_getcBlockSize(
            ip as *const core::ffi::c_void,
            iend.offset_from(ip) as size_t,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) {
            return cBlockSize;
        }
        ip = ip.add(ZSTDv05_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv05_blockHeaderSize);
        if cBlockSize > remainingSize {
            return Error::srcSize_wrong.to_error_code();
        }
        match blockProperties.blockType as core::ffi::c_uint {
            0 => {
                decodedSize = ZSTDv05_decompressBlock_internal(
                    dctx,
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as size_t,
                    ip as *const core::ffi::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTDv05_copyRawBlock(
                    op as *mut core::ffi::c_void,
                    oend.offset_from(op) as size_t,
                    ip as *const core::ffi::c_void,
                    cBlockSize,
                );
            }
            2 => return Error::GENERIC.to_error_code(),
            3 => {
                if remainingSize != 0 {
                    return Error::srcSize_wrong.to_error_code();
                }
            }
            _ => return Error::GENERIC.to_error_code(),
        }
        if cBlockSize == 0 {
            break;
        }
        if ERR_isError(decodedSize) {
            return decodedSize;
        }
        op = op.add(decodedSize);
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
    }
    op.offset_from(ostart) as size_t
}
pub(crate) unsafe fn ZSTDv05_decompress_usingDict(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    ZSTDv05_decompressBegin_usingDict(dctx, dict, dictSize);
    ZSTDv05_checkContinuity(dctx, dst);
    ZSTDv05_decompress_continueDCtx(dctx, dst, maxDstSize, src, srcSize)
}
fn ZSTD_errorFrameSizeInfoLegacy(
    cSize: &mut size_t,
    dBound: &mut core::ffi::c_ulonglong,
    ret: size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
pub(crate) unsafe fn ZSTDv05_findFrameSizeInfoLegacy(
    src: *const core::ffi::c_void,
    srcSize: size_t,
    cSize: &mut size_t,
    dBound: &mut core::ffi::c_ulonglong,
) {
    let mut ip = src as *const u8;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as size_t;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    if srcSize < ZSTDv05_frameHeaderSize_min {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::srcSize_wrong.to_error_code());
        return;
    }
    if MEM_readLE32(src) != ZSTDv05_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::prefix_unknown.to_error_code());
        return;
    }
    ip = ip.add(ZSTDv05_frameHeaderSize_min);
    remainingSize = remainingSize.wrapping_sub(ZSTDv05_frameHeaderSize_min);
    loop {
        let cBlockSize = ZSTDv05_getcBlockSize(
            ip as *const core::ffi::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, cBlockSize);
            return;
        }
        ip = ip.add(ZSTDv05_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv05_blockHeaderSize);
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::srcSize_wrong.to_error_code());
            return;
        }
        if cBlockSize == 0 {
            break;
        }
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const u8) as size_t;
    *dBound = (nbBlocks * BLOCKSIZE as size_t) as core::ffi::c_ulonglong;
}
fn ZSTDv05_nextSrcSizeToDecompress(dctx: &mut ZSTDv05_DCtx) -> size_t {
    dctx.expected
}
unsafe fn ZSTDv05_decompressContinue(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    if srcSize != dctx.expected {
        return Error::srcSize_wrong.to_error_code();
    }
    ZSTDv05_checkContinuity(dctx, dst);
    match dctx.stage as core::ffi::c_uint {
        0 => {
            if srcSize != ZSTDv05_frameHeaderSize_min {
                return Error::srcSize_wrong.to_error_code();
            }
            dctx.headerSize =
                ZSTDv05_decodeFrameHeader_Part1(dctx, src, ZSTDv05_frameHeaderSize_min);
            if ERR_isError(dctx.headerSize) {
                return dctx.headerSize;
            }
            memcpy(
                (dctx.headerBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                src,
                ZSTDv05_frameHeaderSize_min,
            );
            if dctx.headerSize > ZSTDv05_frameHeaderSize_min {
                return Error::GENERIC.to_error_code();
            }
            dctx.expected = 0;
        }
        1 => {}
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_compressed,
                origSize: 0,
            };
            let blockSize = ZSTDv05_getcBlockSize(src, ZSTDv05_blockHeaderSize, &mut bp);
            if ERR_isError(blockSize) {
                return blockSize;
            }
            if bp.blockType as core::ffi::c_uint == bt_end as core::ffi::c_int as core::ffi::c_uint
            {
                dctx.expected = 0;
                dctx.stage = ZSTDv05ds_getFrameHeaderSize;
            } else {
                dctx.expected = blockSize;
                dctx.bType = bp.blockType;
                dctx.stage = ZSTDv05ds_decompressBlock;
            }
            return 0;
        }
        3 => {
            let mut rSize: size_t = 0;
            match dctx.bType as core::ffi::c_uint {
                0 => {
                    rSize = ZSTDv05_decompressBlock_internal(dctx, dst, maxDstSize, src, srcSize);
                }
                1 => {
                    rSize = ZSTDv05_copyRawBlock(dst, maxDstSize, src, srcSize);
                }
                2 => return Error::GENERIC.to_error_code(),
                3 => {
                    rSize = 0;
                }
                _ => return Error::GENERIC.to_error_code(),
            }
            dctx.stage = ZSTDv05ds_decodeBlockHeader;
            dctx.expected = ZSTDv05_blockHeaderSize;
            if ERR_isError(rSize) {
                return rSize;
            }
            dctx.previousDstEnd =
                (dst as *mut core::ffi::c_char).add(rSize) as *const core::ffi::c_void;
            return rSize;
        }
        _ => return Error::GENERIC.to_error_code(),
    }
    let result = ZSTDv05_decodeFrameHeader_Part2(
        dctx,
        (dctx.headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
        dctx.headerSize,
    );
    if ERR_isError(result) {
        return result;
    }
    dctx.expected = ZSTDv05_blockHeaderSize;
    dctx.stage = ZSTDv05ds_decodeBlockHeader;
    0
}
unsafe fn ZSTDv05_refDictContent(
    dctx: &mut ZSTDv05_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) {
    dctx.dictEnd = dctx.previousDstEnd;
    dctx.vBase = (dict as *const core::ffi::c_char).offset(
        -((dctx.previousDstEnd as *const core::ffi::c_char)
            .offset_from(dctx.base as *const core::ffi::c_char) as core::ffi::c_long
            as isize),
    ) as *const core::ffi::c_void;
    dctx.base = dict;
    dctx.previousDstEnd =
        (dict as *const core::ffi::c_char).add(dictSize) as *const core::ffi::c_void;
}
unsafe fn ZSTDv05_loadEntropy(
    dctx: &mut ZSTDv05_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut hSize: size_t = 0;
    let mut offcodeHeaderSize: size_t = 0;
    let mut matchlengthHeaderSize: size_t = 0;
    let mut errorCode: size_t = 0;
    let mut litlengthHeaderSize: size_t = 0;
    let mut offcodeNCount: [core::ffi::c_short; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff as core::ffi::c_uint;
    let mut offcodeLog: core::ffi::c_uint = 0;
    let mut matchlengthNCount: [core::ffi::c_short; 128] = [0; 128];
    let mut matchlengthMaxValue = MaxML as core::ffi::c_uint;
    let mut matchlengthLog: core::ffi::c_uint = 0;
    let mut litlengthNCount: [core::ffi::c_short; 64] = [0; 64];
    let mut litlengthMaxValue = MaxLL as core::ffi::c_uint;
    let mut litlengthLog: core::ffi::c_uint = 0;
    hSize = HUFv05_readDTableX4(dctx.hufTableX4.as_mut_ptr(), dict, dictSize);
    if ERR_isError(hSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    dict = (dict as *const core::ffi::c_char).add(hSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(hSize);
    offcodeHeaderSize = FSEv05_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dict,
        dictSize,
    );
    if ERR_isError(offcodeHeaderSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    if offcodeLog > OffFSEv05Log as core::ffi::c_uint {
        return Error::dictionary_corrupted.to_error_code();
    }
    errorCode = FSEv05_buildDTable(
        dctx.OffTable.as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        offcodeLog,
    );
    if ERR_isError(errorCode) {
        return Error::dictionary_corrupted.to_error_code();
    }
    dict = (dict as *const core::ffi::c_char).add(offcodeHeaderSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(offcodeHeaderSize);
    matchlengthHeaderSize = FSEv05_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dict,
        dictSize,
    );
    if ERR_isError(matchlengthHeaderSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    if matchlengthLog > MLFSEv05Log as core::ffi::c_uint {
        return Error::dictionary_corrupted.to_error_code();
    }
    errorCode = FSEv05_buildDTable(
        dctx.MLTable.as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
    );
    if ERR_isError(errorCode) {
        return Error::dictionary_corrupted.to_error_code();
    }
    dict =
        (dict as *const core::ffi::c_char).add(matchlengthHeaderSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(matchlengthHeaderSize);
    litlengthHeaderSize = FSEv05_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dict,
        dictSize,
    );
    if litlengthLog > LLFSEv05Log as core::ffi::c_uint {
        return Error::dictionary_corrupted.to_error_code();
    }
    if ERR_isError(litlengthHeaderSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    errorCode = FSEv05_buildDTable(
        dctx.LLTable.as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
    );
    if ERR_isError(errorCode) {
        return Error::dictionary_corrupted.to_error_code();
    }
    dctx.flagStaticTables = 1;
    hSize
        .wrapping_add(offcodeHeaderSize)
        .wrapping_add(matchlengthHeaderSize)
        .wrapping_add(litlengthHeaderSize)
}
unsafe fn ZSTDv05_decompress_insertDictionary(
    dctx: &mut ZSTDv05_DCtx,
    mut dict: *const core::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut eSize: size_t = 0;
    let magic = MEM_readLE32(dict);
    if magic != ZSTDv05_DICT_MAGIC {
        ZSTDv05_refDictContent(dctx, dict, dictSize);
        return 0;
    }
    dict = (dict as *const core::ffi::c_char).offset(4) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(4);
    eSize = ZSTDv05_loadEntropy(dctx, dict, dictSize);
    if ERR_isError(eSize) {
        return Error::dictionary_corrupted.to_error_code();
    }
    dict = (dict as *const core::ffi::c_char).add(eSize) as *const core::ffi::c_void;
    dictSize = dictSize.wrapping_sub(eSize);
    ZSTDv05_refDictContent(dctx, dict, dictSize);
    0
}
unsafe fn ZSTDv05_decompressBegin_usingDict(
    dctx: &mut ZSTDv05_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    let mut errorCode: size_t = 0;
    errorCode = ZSTDv05_decompressBegin(dctx);
    if ERR_isError(errorCode) {
        return errorCode;
    }
    if !dict.is_null() && dictSize != 0 {
        errorCode = ZSTDv05_decompress_insertDictionary(dctx, dict, dictSize);
        if ERR_isError(errorCode) {
            return Error::dictionary_corrupted.to_error_code();
        }
    }
    0
}
static ZBUFFv05_blockHeaderSize: size_t = 3;
unsafe fn ZBUFFv05_limitCopy(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let length = if maxDstSize < srcSize {
        maxDstSize
    } else {
        srcSize
    };
    if length > 0 {
        memcpy(dst, src, length);
    }
    length
}
const ZSTDv05_frameHeaderSize_max_0: core::ffi::c_int = 5;
pub(crate) unsafe fn ZBUFFv05_createDCtx() -> *mut ZBUFFv05_DCtx {
    let zbc = malloc(::core::mem::size_of::<ZBUFFv05_DCtx>()) as *mut ZBUFFv05_DCtx;
    if zbc.is_null() {
        return core::ptr::null_mut();
    }
    ptr::write_bytes(zbc as *mut u8, 0, ::core::mem::size_of::<ZBUFFv05_DCtx>());
    (*zbc).zc = ZSTDv05_createDCtx();
    (*zbc).stage = ZBUFFv05ds_init;
    zbc
}
pub(crate) unsafe fn ZBUFFv05_freeDCtx(zbc: *mut ZBUFFv05_DCtx) -> size_t {
    if zbc.is_null() {
        return 0;
    }
    ZSTDv05_freeDCtx((*zbc).zc);
    free((*zbc).inBuff as *mut core::ffi::c_void);
    free((*zbc).outBuff as *mut core::ffi::c_void);
    free(zbc as *mut core::ffi::c_void);
    0
}
pub(crate) unsafe fn ZBUFFv05_decompressInitDictionary(
    zbc: *mut ZBUFFv05_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
) -> size_t {
    (*zbc).stage = ZBUFFv05ds_readHeader;
    (*zbc).outEnd = 0;
    (*zbc).outStart = (*zbc).outEnd;
    (*zbc).inPos = (*zbc).outStart;
    (*zbc).hPos = (*zbc).inPos;
    ZSTDv05_decompressBegin_usingDict(&mut *(*zbc).zc, dict, dictSize)
}
pub(crate) unsafe fn ZBUFFv05_decompressContinue(
    zbc: &mut ZBUFFv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSizePtr: *mut size_t,
    src: *const core::ffi::c_void,
    srcSizePtr: *mut size_t,
) -> size_t {
    let istart = src as *const core::ffi::c_char;
    let mut ip = istart;
    let iend = istart.add(*srcSizePtr);
    let ostart = dst as *mut core::ffi::c_char;
    let mut op = ostart;
    let oend = ostart.add(*maxDstSizePtr);
    let mut notDone = 1;

    while notDone != 0 {
        #[derive(Eq, PartialEq)]
        enum Block {
            DecodeHeader,
            Read,
            Load,
            Flush,
        }
        let mut current_block;
        match zbc.stage {
            ZBUFFv05ds_init => return Error::init_missing.to_error_code(),
            ZBUFFv05ds_readHeader => {
                // read header from src
                let headerSize = ZSTDv05_getFrameParams(&mut zbc.params, src, *srcSizePtr);
                if ERR_isError(headerSize) {
                    return headerSize;
                }
                if headerSize != 0 {
                    // not enough input to decode header : tell how many bytes would be necessary
                    memcpy(
                        (zbc.headerBuffer).as_mut_ptr().add(zbc.hPos) as *mut core::ffi::c_void,
                        src,
                        *srcSizePtr,
                    );
                    zbc.hPos = (zbc.hPos).wrapping_add(*srcSizePtr);
                    *maxDstSizePtr = 0;
                    zbc.stage = ZBUFFv05ds_loadHeader;
                    return headerSize.wrapping_sub(zbc.hPos);
                }
                zbc.stage = ZBUFFv05ds_decodeHeader;
                continue;
            }
            ZBUFFv05ds_loadHeader => {
                // complete header from src
                let mut headerSize_0 = ZBUFFv05_limitCopy(
                    (zbc.headerBuffer).as_mut_ptr().add(zbc.hPos) as *mut core::ffi::c_void,
                    (ZSTDv05_frameHeaderSize_max_0 as size_t).wrapping_sub(zbc.hPos),
                    src,
                    *srcSizePtr,
                );
                zbc.hPos = (zbc.hPos).wrapping_add(headerSize_0);
                ip = ip.add(headerSize_0);
                headerSize_0 = ZSTDv05_getFrameParams(
                    &mut zbc.params,
                    (zbc.headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
                    zbc.hPos,
                );
                if ERR_isError(headerSize_0) {
                    return headerSize_0;
                }
                if headerSize_0 != 0 {
                    // not enough input to decode header : tell how many bytes would be necessary
                    *maxDstSizePtr = 0;
                    return headerSize_0.wrapping_sub(zbc.hPos);
                }
                current_block = Block::DecodeHeader;
            }
            ZBUFFv05ds_decodeHeader => {
                current_block = Block::DecodeHeader;
            }
            ZBUFFv05ds_read => {
                current_block = Block::Read;
            }
            ZBUFFv05ds_load => {
                current_block = Block::Load;
            }
            ZBUFFv05ds_flush => {
                current_block = Block::Flush;
            }
            _ => return Error::GENERIC.to_error_code(),
        }
        if current_block == Block::DecodeHeader {
            drop(current_block);

            // apply header to create / resize buffers
            let neededOutSize = 1 << zbc.params.windowLog;
            let neededInSize = BLOCKSIZE as size_t; // a block is never > BLOCKSIZE
            if zbc.inBuffSize < neededInSize {
                free(zbc.inBuff as *mut core::ffi::c_void);
                zbc.inBuffSize = neededInSize;
                zbc.inBuff = malloc(neededInSize) as *mut core::ffi::c_char;
                if zbc.inBuff.is_null() {
                    return Error::memory_allocation.to_error_code();
                }
            }
            if zbc.outBuffSize < neededOutSize {
                free(zbc.outBuff as *mut core::ffi::c_void);
                zbc.outBuffSize = neededOutSize;
                zbc.outBuff = malloc(neededOutSize) as *mut core::ffi::c_char;
                if zbc.outBuff.is_null() {
                    return Error::memory_allocation.to_error_code();
                }
            }
            if zbc.hPos != 0 {
                // some data already loaded into headerBuffer : transfer into inBuff
                memcpy(
                    zbc.inBuff as *mut core::ffi::c_void,
                    (zbc.headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
                    zbc.hPos,
                );
                zbc.inPos = zbc.hPos;
                zbc.hPos = 0;
                zbc.stage = ZBUFFv05ds_load;
                continue;
            }

            zbc.stage = ZBUFFv05ds_read;
            current_block = Block::Read;
        }
        if current_block == Block::Read {
            drop(current_block);

            let neededInSize_0 = ZSTDv05_nextSrcSizeToDecompress(&mut *zbc.zc);
            if neededInSize_0 == 0 {
                // end of frame
                zbc.stage = ZBUFFv05ds_init;
                notDone = 0;
                continue;
            }
            if iend.offset_from(ip) as size_t >= neededInSize_0 {
                // directly decode from src
                let decodedSize = ZSTDv05_decompressContinue(
                    &mut *zbc.zc,
                    (zbc.outBuff).add(zbc.outStart) as *mut core::ffi::c_void,
                    (zbc.outBuffSize).wrapping_sub(zbc.outStart),
                    ip as *const core::ffi::c_void,
                    neededInSize_0,
                );
                if ERR_isError(decodedSize) {
                    return decodedSize;
                }
                ip = ip.add(neededInSize_0);
                if decodedSize == 0 {
                    continue; // this was just a header
                }
                zbc.outEnd = (zbc.outStart).wrapping_add(decodedSize);
                zbc.stage = ZBUFFv05ds_flush;
                continue;
            } else if ip == iend {
                // no more input
                notDone = 0;
                continue;
            } else {
                zbc.stage = ZBUFFv05ds_load;
                current_block = Block::Load;
            }
        }
        if current_block == Block::Load {
            drop(current_block);

            let neededInSize_1 = ZSTDv05_nextSrcSizeToDecompress(&mut *zbc.zc);
            // should always be <= remaining space within inBuff
            let toLoad = neededInSize_1.wrapping_sub(zbc.inPos);
            let mut loadedSize: size_t = 0;
            if toLoad > (zbc.inBuffSize).wrapping_sub(zbc.inPos) {
                return Error::corruption_detected.to_error_code(); // should never happen
            }
            loadedSize = ZBUFFv05_limitCopy(
                (zbc.inBuff).add(zbc.inPos) as *mut core::ffi::c_void,
                toLoad,
                ip as *const core::ffi::c_void,
                iend.offset_from(ip) as size_t,
            );
            ip = ip.add(loadedSize);
            zbc.inPos = (zbc.inPos).wrapping_add(loadedSize);
            if loadedSize < toLoad {
                // not enough input, wait for more
                notDone = 0;
                continue;
            } else {
                let decodedSize_0 = ZSTDv05_decompressContinue(
                    &mut *zbc.zc,
                    (zbc.outBuff).add(zbc.outStart) as *mut core::ffi::c_void,
                    (zbc.outBuffSize).wrapping_sub(zbc.outStart),
                    zbc.inBuff as *const core::ffi::c_void,
                    neededInSize_1,
                );
                if ERR_isError(decodedSize_0) {
                    return decodedSize_0;
                }
                zbc.inPos = 0; // input is consumed
                if decodedSize_0 == 0 {
                    // this was just a header
                    zbc.stage = ZBUFFv05ds_read;
                    continue;
                }

                zbc.outEnd = (zbc.outStart).wrapping_add(decodedSize_0);
                zbc.stage = ZBUFFv05ds_flush;
                current_block = Block::Flush;
            }
        }
        if current_block == Block::Flush {
            drop(current_block);

            let toFlushSize = (zbc.outEnd).wrapping_sub(zbc.outStart);
            let flushedSize = ZBUFFv05_limitCopy(
                op as *mut core::ffi::c_void,
                oend.offset_from(op) as size_t,
                (zbc.outBuff).add(zbc.outStart) as *const core::ffi::c_void,
                toFlushSize,
            );
            op = op.add(flushedSize);
            zbc.outStart = (zbc.outStart).wrapping_add(flushedSize);
            if flushedSize == toFlushSize {
                zbc.stage = ZBUFFv05ds_read;
                if (zbc.outStart).wrapping_add(BLOCKSIZE as size_t) > zbc.outBuffSize {
                    zbc.outEnd = 0;
                    zbc.outStart = zbc.outEnd;
                }
                continue;
            } else {
                // cannot flush everything
                notDone = 0;
            }
        }
    }
    *srcSizePtr = ip.offset_from(istart) as size_t;
    *maxDstSizePtr = op.offset_from(ostart) as size_t;
    let mut nextSrcSizeHint = ZSTDv05_nextSrcSizeToDecompress(&mut *zbc.zc);
    if nextSrcSizeHint > ZBUFFv05_blockHeaderSize {
        nextSrcSizeHint = nextSrcSizeHint.wrapping_add(ZBUFFv05_blockHeaderSize);
    }
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub(zbc.inPos); // already loaded
    nextSrcSizeHint
}
