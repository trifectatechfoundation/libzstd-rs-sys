use core::{mem, ptr};
use std::marker::PhantomData;

use libc::{free, malloc, ptrdiff_t, size_t};

use crate::lib::common::error_private::Error;
use crate::lib::common::mem::{MEM_32bits, MEM_64bits, MEM_readLE16, MEM_readLE32, MEM_readLEST};
use crate::lib::common::reader::Reader;
use crate::lib::decompress::huf_decompress::Writer;
use crate::ZSTD_CONTENTSIZE_ERROR;

#[repr(C)]
pub(crate) struct ZSTDv05_DCtx {
    LLTable: FSEv05_DTable<1024>,
    OffTable: FSEv05_DTable<512>,
    MLTable: FSEv05_DTable<1024>,
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
    headerBuffer: [u8; ZSTDv05_frameHeaderSize_min],
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
struct seqState_t<'a> {
    DStream: BITv05_DStream_t<'a>,
    stateLL: FSEv05_DState_t<'a, 1024>,
    stateOffb: FSEv05_DState_t<'a, 512>,
    stateML: FSEv05_DState_t<'a, 1024>,
    prevOffset: size_t,
    dumps: &'a [u8],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv05_DState_t<'a, const N: usize> {
    state: size_t,
    table: &'a [FSEv05_decode_t; N],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv05_decode_t {
    newState: core::ffi::c_ushort,
    symbol: core::ffi::c_uchar,
    nbBits: core::ffi::c_uchar,
}

#[derive(Copy, Clone, Default)]
#[repr(transparent)]
struct LE16([u8; 2]);

#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv05_DTableHeader {
    tableLog: u16,
    fastMode: u16,
}
#[repr(C)]
struct FSEv05_DTable<const N: usize> {
    header: FSEv05_DTableHeader,
    data: [FSEv05_decode_t; N],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct HUFv05_DEltX4 {
    sequence: LE16,
    nbBits: u8,
    length: u8,
}
type rankVal_t = [[u32; HUFv05_ABSOLUTEMAX_TABLELOG + 1]; HUFv05_ABSOLUTEMAX_TABLELOG];
#[derive(Copy, Clone)]
#[repr(C)]
struct sortedSymbol_t {
    symbol: u8,
    weight: u8,
}
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
const ZSTDv05_WINDOWLOG_ABSOLUTEMIN: u8 = 11;
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

#[inline]
unsafe fn ZSTDv05_wildcopy(dst: *mut u8, src: *const u8, length: ptrdiff_t) {
    let mut ip = src;
    let mut op = dst;
    let oend = op.offset(length);
    loop {
        ptr::copy_nonoverlapping(ip, op, 8);
        op = op.add(8);
        ip = ip.add(8);
        if op >= oend {
            break;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum StreamStatus {
    /// Fully refilled.
    Unfinished = 0,
    /// Still some bits left in the bitstream.
    EndOfBuffer = 1,
    /// Bitstream entirely consumed, bit-exact.
    Completed = 2,
    /// User requested more bits than present in bitstream.
    Overflow = 3,
}

#[inline]
fn BITv05_highbit32(val: u32) -> u32 {
    val.leading_zeros() ^ 31
}

#[derive(Copy, Clone)]
#[repr(C)]
struct BITv05_DStream_t<'a> {
    bitContainer: usize,
    bitsConsumed: u32,
    ptr: *const u8,
    start: *const u8,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> BITv05_DStream_t<'a> {
    #[inline]
    fn new(src: &'a [u8]) -> Result<Self, Error> {
        if src.is_empty() {
            return Err(Error::srcSize_wrong);
        }

        if src.len() >= size_of::<usize>() {
            // normal case
            let mut bitD = BITv05_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: unsafe { src.as_ptr().add(src.len()).sub(size_of::<usize>()) },
                start: src.as_ptr(),
                _marker: PhantomData,
            };

            bitD.bitContainer = usize::from_le_bytes(*src.last_chunk().unwrap());
            let contain32 = u32::from(src[src.len() - 1]);
            if contain32 == 0 {
                return Err(Error::GENERIC); // endMark not present
            }
            bitD.bitsConsumed = 8 - BITv05_highbit32(contain32);

            Ok(bitD)
        } else {
            let mut bitD = BITv05_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: src.as_ptr(),
                start: src.as_ptr(),
                _marker: PhantomData,
            };

            if src.len() == 7 {
                bitD.bitContainer += usize::from(src[6]) << (usize::BITS - 16);
            }

            if src.len() >= 6 {
                bitD.bitContainer += usize::from(src[5]) << (usize::BITS - 24);
            }

            if src.len() >= 5 {
                bitD.bitContainer += usize::from(src[4]) << (usize::BITS - 32);
            }

            if src.len() >= 4 {
                bitD.bitContainer += usize::from(src[3]) << 24;
            }

            if src.len() >= 3 {
                bitD.bitContainer += usize::from(src[2]) << 16;
            }

            if src.len() >= 2 {
                bitD.bitContainer += usize::from(src[1]) << 8;
            }

            bitD.bitContainer += usize::from(src[0]);

            let contain32 = u32::from(src[src.len() - 1]);
            if contain32 == 0 {
                // endMark not present
                return Err(Error::GENERIC);
            }
            bitD.bitsConsumed = 8 - BITv05_highbit32(contain32);
            bitD.bitsConsumed += (size_of::<usize>() - src.len()) as u32 * 8;

            Ok(bitD)
        }
    }

    #[inline]
    fn look_bits(&self, nbBits: u32) -> usize {
        let bitMask = usize::BITS - 1;
        self.bitContainer << (self.bitsConsumed & bitMask)
            >> 1
            >> (bitMask.wrapping_sub(nbBits) & bitMask)
    }
    #[inline]
    fn look_bits_fast(&self, nbBits: u32) -> usize {
        let bitMask = usize::BITS - 1;
        self.bitContainer << (self.bitsConsumed & bitMask)
            >> (bitMask.wrapping_add(1).wrapping_sub(nbBits) & bitMask)
    }
    #[inline]
    fn skip_bits(&mut self, nbBits: u32) {
        self.bitsConsumed += nbBits;
    }
    #[inline]
    fn read_bits(&mut self, nbBits: u32) -> usize {
        let value = self.look_bits(nbBits);
        self.skip_bits(nbBits);
        value
    }
    #[inline]
    fn read_bits_fast(&mut self, nbBits: u32) -> usize {
        let value = self.look_bits_fast(nbBits);
        self.skip_bits(nbBits);
        value
    }
    #[inline]
    fn reload(&mut self) -> StreamStatus {
        if self.bitsConsumed > usize::BITS {
            // should never happen
            return StreamStatus::Overflow;
        }

        if self.ptr >= unsafe { self.start.add(size_of::<usize>()) } {
            self.ptr = unsafe { self.ptr.sub((self.bitsConsumed >> 3) as usize) };
            self.bitsConsumed &= 7;
            self.bitContainer = unsafe { MEM_readLEST(self.ptr.cast()) };
            return StreamStatus::Unfinished;
        }
        if self.ptr == self.start {
            if self.bitsConsumed < usize::BITS {
                return StreamStatus::EndOfBuffer;
            }
            return StreamStatus::Completed;
        }
        let mut nbBytes = self.bitsConsumed >> 3;
        let mut result = StreamStatus::Unfinished;
        if unsafe { self.ptr.sub(nbBytes as usize) < self.start } {
            nbBytes = unsafe { self.ptr.offset_from(self.start) } as u32;
            result = StreamStatus::EndOfBuffer;
        }
        self.ptr = unsafe { self.ptr.sub(nbBytes as usize) };
        self.bitsConsumed = (self.bitsConsumed).wrapping_sub(nbBytes * 8);
        self.bitContainer = unsafe { MEM_readLEST(self.ptr.cast()) };
        result
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.ptr == self.start && self.bitsConsumed == usize::BITS
    }
}

#[inline]
fn FSEv05_initDState<'a, const N: usize>(
    bitD: &mut BITv05_DStream_t,
    dt: &'a FSEv05_DTable<N>,
) -> FSEv05_DState_t<'a, N> {
    let state = bitD.read_bits(core::ffi::c_uint::from(dt.header.tableLog));
    bitD.reload();

    FSEv05_DState_t {
        state,
        table: &dt.data,
    }
}
#[inline]
fn FSEv05_peakSymbol<const N: usize>(DStatePtr: &mut FSEv05_DState_t<N>) -> u8 {
    let DInfo = DStatePtr.table[DStatePtr.state];
    DInfo.symbol
}
#[inline]
fn FSEv05_decodeSymbol<const N: usize>(
    DStatePtr: &mut FSEv05_DState_t<N>,
    bitD: &mut BITv05_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = DStatePtr.table[DStatePtr.state];
    let nbBits = u32::from(DInfo.nbBits);
    let symbol = DInfo.symbol;
    let lowBits = bitD.read_bits(nbBits);
    DStatePtr.state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
fn FSEv05_decodeSymbolFast<const N: usize>(
    DStatePtr: &mut FSEv05_DState_t<N>,
    bitD: &mut BITv05_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = DStatePtr.table[DStatePtr.state];
    let nbBits = u32::from(DInfo.nbBits);
    let symbol = DInfo.symbol;
    let lowBits = bitD.read_bits_fast(nbBits);
    DStatePtr.state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
fn FSEv05_endOfDState<const N: usize>(DStatePtr: &FSEv05_DState_t<N>) -> core::ffi::c_uint {
    core::ffi::c_int::from(DStatePtr.state == 0) as core::ffi::c_uint
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
fn FSEv05_buildDTable<const N: usize>(
    dt: &mut FSEv05_DTable<N>,
    normalizedCounter: &[core::ffi::c_short],
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
) -> Result<(), Error> {
    let mut DTableH = FSEv05_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    let tableDecode = &mut dt.data;
    let tableSize = ((1) << tableLog) as u32;
    let tableMask = tableSize.wrapping_sub(1);
    let step = FSEv05_tableStep(tableSize);
    let mut symbolNext: [u16; 256] = [0; 256];
    let mut position = 0u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;
    let mut noLarge = 1;
    if maxSymbolValue > FSEv05_MAX_SYMBOL_VALUE as core::ffi::c_uint {
        return Err(Error::maxSymbolValue_tooLarge);
    }
    if tableLog > FSEv05_MAX_TABLELOG as core::ffi::c_uint {
        return Err(Error::tableLog_tooLarge);
    }
    tableDecode[..maxSymbolValue as usize + 1].fill(FSEv05_decode_t {
        newState: 0,
        symbol: 0,
        nbBits: 0,
    });
    DTableH.tableLog = tableLog as u16;
    for s in 0..maxSymbolValue + 1 {
        if core::ffi::c_int::from(normalizedCounter[s as usize]) == -(1) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            tableDecode[fresh0 as usize].symbol = s as u8;
            symbolNext[s as usize] = 1;
        } else {
            if normalizedCounter[s as usize] >= largeLimit {
                noLarge = 0;
            }
            symbolNext[s as usize] = normalizedCounter[s as usize] as u16;
        }
    }
    for s in 0..maxSymbolValue + 1 {
        let mut i: core::ffi::c_int = 0;
        i = 0;
        while i < core::ffi::c_int::from(normalizedCounter[s as usize]) {
            tableDecode[position as usize].symbol = s as u8;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
        }
    }
    if position != 0 {
        return Err(Error::GENERIC);
    }
    let mut i_0: u32 = 0;
    while i_0 < tableSize {
        let symbol = tableDecode[i_0 as usize].symbol;
        let fresh1 = &mut symbolNext[symbol as usize];
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2;
        tableDecode[i_0 as usize].nbBits =
            tableLog.wrapping_sub(BITv05_highbit32(u32::from(nextState))) as u8;
        tableDecode[i_0 as usize].newState = ((core::ffi::c_int::from(nextState)
            << core::ffi::c_int::from((tableDecode[i_0 as usize]).nbBits))
            as u32)
            .wrapping_sub(tableSize) as u16;
        i_0 = i_0.wrapping_add(1);
    }
    DTableH.fastMode = noLarge as u16;
    dt.header = DTableH;
    Ok(())
}

unsafe fn FSEv05_readNCount(
    normalizedCounter: &mut [core::ffi::c_short],
    maxSVPtr: &mut core::ffi::c_uint,
    tableLogPtr: &mut core::ffi::c_uint,
    headerBuffer: &[u8],
) -> Result<size_t, Error> {
    let istart = headerBuffer.as_ptr();
    let iend = istart.add(headerBuffer.len());
    let mut ip = istart;
    let mut charnum = 0;
    let mut previous0 = 0;
    if headerBuffer.len() < 4 {
        return Err(Error::srcSize_wrong);
    }
    let mut bitStream = MEM_readLE32(ip as *const core::ffi::c_void);
    let mut nbBits = (bitStream & 0xf as core::ffi::c_int as u32)
        .wrapping_add(FSEv05_MIN_TABLELOG as u32) as core::ffi::c_int;
    if nbBits > FSEv05_TABLELOG_ABSOLUTE_MAX {
        return Err(Error::tableLog_tooLarge);
    }
    bitStream >>= 4;
    let mut bitCount = 4;
    *tableLogPtr = nbBits as core::ffi::c_uint;
    let mut remaining = ((1) << nbBits) + 1;
    let mut threshold = (1) << nbBits;
    nbBits += 1;
    while remaining > 1 && charnum <= *maxSVPtr {
        if previous0 != 0 {
            let mut n0 = charnum;
            while bitStream & 0xffff as core::ffi::c_int as u32 == 0xffff as core::ffi::c_int as u32
            {
                n0 = n0.wrapping_add(24);
                if ip < iend.sub(5) {
                    ip = ip.add(2);
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
                return Err(Error::maxSymbolValue_tooSmall);
            }
            while charnum < n0 {
                let fresh3 = charnum;
                charnum = charnum.wrapping_add(1);
                normalizedCounter[fresh3 as usize] = 0;
            }
            if ip <= iend.sub(7) || ip.offset((bitCount >> 3) as isize) <= iend.sub(4) {
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
            if core::ffi::c_int::from(count) >= threshold {
                count = (core::ffi::c_int::from(count) - core::ffi::c_int::from(max))
                    as core::ffi::c_short;
            }
            bitCount += nbBits;
        }
        count -= 1;
        remaining -= core::ffi::c_int::from(count.abs());
        let fresh4 = charnum;
        charnum = charnum.wrapping_add(1);
        normalizedCounter[fresh4 as usize] = count;
        previous0 = core::ffi::c_int::from(count == 0);
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1;
        }
        if ip <= iend.sub(7) || ip.offset((bitCount >> 3) as isize) <= iend.sub(4) {
            ip = ip.offset((bitCount >> 3) as isize);
            bitCount &= 7;
        } else {
            bitCount -= (8 * iend.sub(4).offset_from(ip) as core::ffi::c_long) as core::ffi::c_int;
            ip = iend.sub(4);
        }
        bitStream = MEM_readLE32(ip as *const core::ffi::c_void) >> (bitCount & 31);
    }
    if remaining != 1 {
        return Err(Error::GENERIC);
    }
    *maxSVPtr = charnum.wrapping_sub(1);
    ip = ip.offset(((bitCount + 7) >> 3) as isize);
    if ip.offset_from_unsigned(istart) > headerBuffer.len() {
        return Err(Error::srcSize_wrong);
    }
    Ok(ip.offset_from_unsigned(istart))
}
fn FSEv05_buildDTable_rle<const N: usize>(dt: &mut FSEv05_DTable<N>, symbolValue: u8) -> size_t {
    dt.header.tableLog = 0;
    dt.header.fastMode = 0;
    dt.data[0].newState = 0;
    dt.data[0].symbol = symbolValue;
    dt.data[0].nbBits = 0;
    0
}
fn FSEv05_buildDTable_raw<const N: usize>(
    dt: &mut FSEv05_DTable<N>,
    nbBits: core::ffi::c_uint,
) -> Result<(), Error> {
    let tableSize = (1 << nbBits) as core::ffi::c_uint;
    let tableMask = tableSize.wrapping_sub(1);
    let maxSymbolValue = tableMask;
    if nbBits < 1 {
        return Err(Error::GENERIC);
    }
    dt.header.tableLog = nbBits as u16;
    dt.header.fastMode = 1;
    let mut s = 0;
    while s <= maxSymbolValue {
        dt.data[s as usize].newState = 0;
        dt.data[s as usize].symbol = s as u8;
        dt.data[s as usize].nbBits = nbBits as u8;
        s += 1;
    }
    Ok(())
}
#[inline(always)]
fn FSEv05_decompress_usingDTable_generic<const N: usize>(
    dst: &mut [u8],
    cSrc: &[u8],
    dt: &FSEv05_DTable<N>,
    fast: core::ffi::c_uint,
) -> Result<size_t, Error> {
    let dst_len = dst.len();
    let mut op = dst;
    let mut bitD = BITv05_DStream_t::new(cSrc)?;
    let mut state1 = FSEv05_initDState(&mut bitD, dt);
    let mut state2 = FSEv05_initDState(&mut bitD, dt);
    while bitD.reload() == StreamStatus::Unfinished && op.len() < 3 {
        op[0] = (if fast != 0 {
            core::ffi::c_int::from(FSEv05_decodeSymbolFast(&mut state1, &mut bitD))
        } else {
            core::ffi::c_int::from(FSEv05_decodeSymbol(&mut state1, &mut bitD))
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>()).wrapping_mul(8)
        {
            bitD.reload();
        }
        op[1] = (if fast != 0 {
            core::ffi::c_int::from(FSEv05_decodeSymbolFast(&mut state2, &mut bitD))
        } else {
            core::ffi::c_int::from(FSEv05_decodeSymbol(&mut state2, &mut bitD))
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 4 + 7) as size_t
            > (::core::mem::size_of::<size_t>()).wrapping_mul(8)
            && bitD.reload() > StreamStatus::Unfinished
        {
            op = &mut op[2..];
            break;
        }
        op[2] = (if fast != 0 {
            core::ffi::c_int::from(FSEv05_decodeSymbolFast(&mut state1, &mut bitD))
        } else {
            core::ffi::c_int::from(FSEv05_decodeSymbol(&mut state1, &mut bitD))
        }) as u8;
        if (FSEv05_MAX_TABLELOG * 2 + 7) as size_t
            > (::core::mem::size_of::<size_t>()).wrapping_mul(8)
        {
            bitD.reload();
        }
        op[3] = (if fast != 0 {
            core::ffi::c_int::from(FSEv05_decodeSymbolFast(&mut state2, &mut bitD))
        } else {
            core::ffi::c_int::from(FSEv05_decodeSymbol(&mut state2, &mut bitD))
        }) as u8;
        op = &mut op[4..];
    }
    while !(bitD.reload() > StreamStatus::Completed
        || op.is_empty()
        || bitD.is_empty() && (fast != 0 || FSEv05_endOfDState(&state1) != 0))
    {
        let fresh5: &mut [u8; 1];
        (fresh5, op) = op.split_first_chunk_mut::<1>().unwrap();
        fresh5[0] = (if fast != 0 {
            core::ffi::c_int::from(FSEv05_decodeSymbolFast(&mut state1, &mut bitD))
        } else {
            core::ffi::c_int::from(FSEv05_decodeSymbol(&mut state1, &mut bitD))
        }) as u8;
        if bitD.reload() > StreamStatus::Completed
            || op.is_empty()
            || bitD.is_empty() && (fast != 0 || FSEv05_endOfDState(&state2) != 0)
        {
            break;
        }
        let fresh6: &mut [u8; 1];
        (fresh6, op) = op.split_first_chunk_mut::<1>().unwrap();
        fresh6[0] = (if fast != 0 {
            core::ffi::c_int::from(FSEv05_decodeSymbolFast(&mut state2, &mut bitD))
        } else {
            core::ffi::c_int::from(FSEv05_decodeSymbol(&mut state2, &mut bitD))
        }) as u8;
    }
    if bitD.is_empty() && FSEv05_endOfDState(&state1) != 0 && FSEv05_endOfDState(&state2) != 0 {
        return Ok(dst_len - op.len());
    }
    if op.is_empty() {
        return Err(Error::dstSize_tooSmall);
    }
    Err(Error::corruption_detected)
}
fn FSEv05_decompress_usingDTable(
    dst: &mut [u8],
    cSrc: &[u8],
    dt: &FSEv05_DTable<4096>,
) -> Result<size_t, Error> {
    let fastMode = u32::from(dt.header.fastMode);
    if fastMode != 0 {
        return FSEv05_decompress_usingDTable_generic(dst, cSrc, dt, 1);
    }
    FSEv05_decompress_usingDTable_generic(dst, cSrc, dt, 0)
}
fn FSEv05_decompress(dst: &mut [u8], cSrc: &[u8]) -> Result<size_t, Error> {
    let mut counting: [core::ffi::c_short; 256] = [0; 256];
    let mut dt = FSEv05_DTable {
        header: FSEv05_DTableHeader {
            tableLog: 0,
            fastMode: 0,
        },
        data: [FSEv05_decode_t {
            newState: 0,
            symbol: 0,
            nbBits: 0,
        }; 4096],
    };
    let mut tableLog: core::ffi::c_uint = 0;
    let mut maxSymbolValue = FSEv05_MAX_SYMBOL_VALUE as core::ffi::c_uint;
    if cSrc.len() < 2 {
        return Err(Error::srcSize_wrong);
    }
    let headerSize =
        unsafe { FSEv05_readNCount(&mut counting, &mut maxSymbolValue, &mut tableLog, cSrc)? };
    if headerSize >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    FSEv05_buildDTable(&mut dt, &counting, maxSymbolValue, tableLog)?;
    FSEv05_decompress_usingDTable(dst, &cSrc[headerSize..], &dt)
}
const HUFv05_ABSOLUTEMAX_TABLELOG: usize = 16;
const HUFv05_MAX_TABLELOG: core::ffi::c_int = 12;
const HUFv05_MAX_SYMBOL_VALUE: core::ffi::c_int = 255;
fn HUFv05_readStats(
    huffWeight: &mut [u8],
    rankStats: &mut [u32; HUFv05_ABSOLUTEMAX_TABLELOG + 1],
    nbSymbolsPtr: &mut u32,
    tableLogPtr: &mut u32,
    src: &[u8],
) -> Result<size_t, Error> {
    let mut ip = src;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    if src.is_empty() {
        return Err(Error::srcSize_wrong);
    }
    iSize = ip[0] as size_t;
    if iSize >= 128 {
        if iSize >= 242 {
            static l: [core::ffi::c_int; 14] = [1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128];
            oSize = l[iSize.wrapping_sub(242)] as size_t;
            huffWeight.fill(1);
            iSize = 0;
        } else {
            oSize = iSize.wrapping_sub(127);
            iSize = oSize.wrapping_add(1) / 2;
            if iSize.wrapping_add(1) > src.len() {
                return Err(Error::srcSize_wrong);
            }
            if oSize >= huffWeight.len() {
                return Err(Error::corruption_detected);
            }
            ip = &ip[1..];
            let mut n = 0;
            while n < oSize {
                huffWeight[n] = (core::ffi::c_int::from(ip[n / 2]) >> 4) as u8;
                huffWeight[n + 1] = (core::ffi::c_int::from(ip[n / 2]) & 15) as u8;
                n = n.wrapping_add(2);
            }
        }
    } else {
        if iSize.wrapping_add(1) > src.len() {
            return Err(Error::srcSize_wrong);
        }
        let l = huffWeight.len();
        oSize = FSEv05_decompress(&mut huffWeight[..l - 1], &ip[1..iSize + 1])?;
    }
    rankStats.fill(0);
    let mut weightTotal: u32 = 0;
    for &w in &huffWeight[..oSize] {
        if usize::from(w) >= HUFv05_ABSOLUTEMAX_TABLELOG {
            return Err(Error::corruption_detected);
        }
        let fresh7 = &mut rankStats[w as usize];
        *fresh7 = (*fresh7).wrapping_add(1);
        weightTotal = weightTotal.wrapping_add(((1) << core::ffi::c_int::from(w) >> 1) as u32);
    }
    if weightTotal == 0 {
        return Err(Error::corruption_detected);
    }
    let tableLog = (BITv05_highbit32(weightTotal)).wrapping_add(1);
    if tableLog > HUFv05_ABSOLUTEMAX_TABLELOG as u32 {
        return Err(Error::corruption_detected);
    }
    let total = ((1) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1) << BITv05_highbit32(rest)) as u32;
    let lastWeight = (BITv05_highbit32(rest)).wrapping_add(1);
    if verif != rest {
        return Err(Error::corruption_detected);
    }
    huffWeight[oSize] = lastWeight as u8;
    let fresh8 = &mut rankStats[lastWeight as usize];
    *fresh8 = (*fresh8).wrapping_add(1);
    if rankStats[1] < 2 || rankStats[1] & 1 != 0 {
        return Err(Error::corruption_detected);
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    *tableLogPtr = tableLog;
    Ok(iSize.wrapping_add(1))
}
unsafe fn HUFv05_readDTableX2(DTable: *mut u16, src: &[u8]) -> Result<size_t, Error> {
    let mut huffWeight = [0; HUFv05_MAX_SYMBOL_VALUE as usize + 1];
    let mut rankVal: [u32; 17] = [0; 17];
    let mut tableLog = 0;
    let mut nbSymbols = 0;
    let dt = DTable.add(1) as *mut HUFv05_DEltX2;
    let iSize = HUFv05_readStats(
        &mut huffWeight,
        &mut rankVal,
        &mut nbSymbols,
        &mut tableLog,
        src,
    )?;
    if tableLog > u32::from(*DTable) {
        return Err(Error::tableLog_tooLarge);
    }
    *DTable = tableLog as u16;
    let mut nextRankStart: u32 = 0;
    let mut n = 1;
    while n <= tableLog {
        let current = nextRankStart;
        nextRankStart = nextRankStart
            .wrapping_add(*rankVal.as_mut_ptr().offset(n as isize) << n.wrapping_sub(1));
        *rankVal.as_mut_ptr().offset(n as isize) = current;
        n = n.wrapping_add(1);
    }
    n = 0;
    while n < nbSymbols {
        let w = u32::from(*huffWeight.as_mut_ptr().offset(n as isize));
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
    Ok(iSize)
}
unsafe fn HUFv05_decodeSymbolX2(
    Dstream: &mut BITv05_DStream_t,
    dt: *const HUFv05_DEltX2,
    dtLog: u32,
) -> u8 {
    let val = Dstream.look_bits_fast(dtLog);
    let c = (*dt.add(val)).byte;
    Dstream.skip_bits(u32::from((*dt.add(val)).nbBits));
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
    while bitDPtr.reload() == StreamStatus::Unfinished && p <= pEnd.sub(4) {
        if MEM_64bits() {
            let fresh10 = p;
            p = p.add(1);
            *fresh10 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            let fresh11 = p;
            p = p.add(1);
            *fresh11 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh12 = p;
            p = p.add(1);
            *fresh12 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh13 = p;
        p = p.add(1);
        *fresh13 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while bitDPtr.reload() == StreamStatus::Unfinished && p < pEnd {
        let fresh14 = p;
        p = p.add(1);
        *fresh14 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh15 = p;
        p = p.add(1);
        *fresh15 = HUFv05_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    pEnd.offset_from_unsigned(pStart)
}
unsafe fn HUFv05_decompress1X2_usingDTable(
    mut dst: Writer<'_>,
    cSrc: &[u8],
    DTable: *const u16,
) -> Result<size_t, Error> {
    let op = dst.as_mut_ptr();
    let oend = op.add(dst.capacity());
    let dtLog = u32::from(*DTable.add(0));
    let dt = (DTable as *const HUFv05_DEltX2).add(1);
    if dst.capacity() <= cSrc.len() {
        return Err(Error::dstSize_tooSmall);
    }
    let mut bitD = BITv05_DStream_t::new(cSrc)?;
    HUFv05_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if !bitD.is_empty() {
        return Err(Error::corruption_detected);
    }
    Ok(dst.capacity())
}
unsafe fn HUFv05_decompress1X2(dst: Writer<'_>, cSrc: &[u8]) -> Result<size_t, Error> {
    let mut DTable: [core::ffi::c_ushort; 4097] = [12; 4097];
    let mut ip = cSrc;
    let amount = HUFv05_readDTableX2(DTable.as_mut_ptr(), cSrc)?;
    if amount >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    ip = &ip[amount..];
    HUFv05_decompress1X2_usingDTable(dst, ip, DTable.as_mut_ptr())
}
unsafe fn HUFv05_decompress4X2_usingDTable(
    mut dst: Writer<'_>,
    cSrc: &[u8],
    DTable: *const u16,
) -> Result<size_t, Error> {
    if cSrc.len() < 10 {
        return Err(Error::corruption_detected);
    }
    let istart = cSrc.as_ptr();
    let ostart = dst.as_mut_ptr();
    let oend = ostart.add(dst.capacity());
    let dt = (DTable as *const HUFv05_DEltX2).add(1);
    let dtLog = u32::from(*DTable);
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.add(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.add(4) as *const core::ffi::c_void) as size_t;
    let istart1 = istart.add(6);
    let istart2 = istart1.add(length1);
    let istart3 = istart2.add(length2);
    let istart4 = istart3.add(length3);
    let segmentSize = dst.capacity().wrapping_add(3) / 4;
    let opStart2 = ostart.add(segmentSize);
    let opStart3 = opStart2.add(segmentSize);
    let opStart4 = opStart3.add(segmentSize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let length4 = cSrc.len().wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    if length4 > cSrc.len() {
        return Err(Error::corruption_detected);
    }
    let mut bitD1 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart1, length1))?;
    let mut bitD2 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart2, length2))?;
    let mut bitD3 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart3, length3))?;
    let mut bitD4 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart4, length4))?;
    let mut end_signal = true;
    end_signal &= bitD1.reload() == StreamStatus::Unfinished;
    end_signal &= bitD2.reload() == StreamStatus::Unfinished;
    end_signal &= bitD3.reload() == StreamStatus::Unfinished;
    end_signal &= bitD4.reload() == StreamStatus::Unfinished;
    while end_signal && op4 < oend.sub(7) {
        if MEM_64bits() {
            let fresh16 = op1;
            op1 = op1.add(1);
            *fresh16 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh17 = op2;
            op2 = op2.add(1);
            *fresh17 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh18 = op3;
            op3 = op3.add(1);
            *fresh18 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh19 = op4;
            op4 = op4.add(1);
            *fresh19 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            let fresh20 = op1;
            op1 = op1.add(1);
            *fresh20 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            let fresh21 = op2;
            op2 = op2.add(1);
            *fresh21 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            let fresh22 = op3;
            op3 = op3.add(1);
            *fresh22 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            let fresh23 = op4;
            op4 = op4.add(1);
            *fresh23 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh24 = op1;
            op1 = op1.add(1);
            *fresh24 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh25 = op2;
            op2 = op2.add(1);
            *fresh25 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh26 = op3;
            op3 = op3.add(1);
            *fresh26 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh27 = op4;
            op4 = op4.add(1);
            *fresh27 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh28 = op1;
        op1 = op1.add(1);
        *fresh28 = HUFv05_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh29 = op2;
        op2 = op2.add(1);
        *fresh29 = HUFv05_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh30 = op3;
        op3 = op3.add(1);
        *fresh30 = HUFv05_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh31 = op4;
        op4 = op4.add(1);
        *fresh31 = HUFv05_decodeSymbolX2(&mut bitD4, dt, dtLog);
        end_signal &= bitD1.reload() == StreamStatus::Unfinished;
        end_signal &= bitD2.reload() == StreamStatus::Unfinished;
        end_signal &= bitD3.reload() == StreamStatus::Unfinished;
        end_signal &= bitD4.reload() == StreamStatus::Unfinished;
    }
    if op1 > opStart2 {
        return Err(Error::corruption_detected);
    }
    if op2 > opStart3 {
        return Err(Error::corruption_detected);
    }
    if op3 > opStart4 {
        return Err(Error::corruption_detected);
    }
    HUFv05_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv05_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv05_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv05_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    if !(bitD1.is_empty() && bitD2.is_empty() && bitD3.is_empty() && bitD4.is_empty()) {
        return Err(Error::corruption_detected);
    }
    Ok(dst.capacity())
}
fn HUFv05_decompress4X2(dst: Writer<'_>, cSrc: &[u8]) -> Result<size_t, Error> {
    let mut DTable: [core::ffi::c_ushort; 4097] = [12; 4097];
    let amount = unsafe { HUFv05_readDTableX2(DTable.as_mut_ptr(), cSrc)? };
    if amount >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    unsafe { HUFv05_decompress4X2_usingDTable(dst, &cSrc[amount..], DTable.as_mut_ptr()) }
}
fn HUFv05_fillDTableX4Level2(
    DTable: &mut [HUFv05_DEltX4],
    sizeLog: u32,
    consumed: u32,
    rankValOrigin: &[u32; 17],
    minWeight: core::ffi::c_int,
    sortedSymbols: &[sortedSymbol_t],
    nbBitsBaseline: u32,
    baseSeq: u16,
) {
    let mut DElt = HUFv05_DEltX4 {
        sequence: LE16::default(),
        nbBits: 0,
        length: 0,
    };
    let mut rankVal: [u32; 17] = *rankValOrigin;
    if minWeight > 1 {
        let mut i: u32 = 0;
        let skipSize = rankVal[minWeight as usize];
        DElt.sequence.0 = u16::to_le_bytes(baseSeq);
        DElt.nbBits = consumed as u8;
        DElt.length = 1;
        i = 0;
        while i < skipSize {
            DTable[i as usize] = DElt;
            i = i.wrapping_add(1);
        }
    }
    let mut s: usize = 0;
    while s < sortedSymbols.len() {
        let symbol = u32::from((sortedSymbols[s]).symbol);
        let weight = u32::from((sortedSymbols[s]).weight);
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let length = ((1) << sizeLog.wrapping_sub(nbBits)) as u32;
        let start = rankVal[weight as usize];
        let mut i_0 = start;
        let end = start.wrapping_add(length);
        DElt.sequence.0 = u16::to_le_bytes(u32::from(baseSeq).wrapping_add(symbol << 8) as u16);
        DElt.nbBits = nbBits.wrapping_add(consumed) as u8;
        DElt.length = 2;
        loop {
            let fresh32 = i_0;
            i_0 = i_0.wrapping_add(1);
            DTable[fresh32 as usize] = DElt;
            if i_0 >= end {
                break;
            }
        }
        let fresh33 = &mut rankVal[weight as usize];
        *fresh33 = (*fresh33).wrapping_add(length);
        s = s.wrapping_add(1);
    }
}
fn HUFv05_fillDTableX4(
    DTable: &mut [HUFv05_DEltX4; 4096],
    targetLog: u32,
    sortedList: &[sortedSymbol_t],
    rankStart: &[u32; 18],
    rankValOrigin: &mut [[u32; HUFv05_ABSOLUTEMAX_TABLELOG + 1]; HUFv05_ABSOLUTEMAX_TABLELOG],
    maxWeight: u32,
    nbBitsBaseline: u32,
) {
    let mut rankVal = rankValOrigin[0];
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as core::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s = 0;
    while s < sortedList.len() {
        let symbol = u16::from((sortedList[s]).symbol);
        let weight = u32::from((sortedList[s]).weight);
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let start: u32 = rankVal[weight as usize];
        let length = ((1) << targetLog.wrapping_sub(nbBits)) as u32;
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut minWeight = nbBits.wrapping_add(scaleLog as u32) as core::ffi::c_int;
            if minWeight < 1 {
                minWeight = 1;
            }
            let sortedRank = rankStart[minWeight as usize];
            HUFv05_fillDTableX4Level2(
                &mut DTable[start as usize..],
                targetLog.wrapping_sub(nbBits),
                nbBits,
                &rankValOrigin[nbBits as usize],
                minWeight,
                &sortedList[sortedRank as usize..],
                nbBitsBaseline,
                symbol,
            );
        } else {
            let end = start.wrapping_add(length);
            let mut DElt = HUFv05_DEltX4 {
                sequence: LE16::default(),
                nbBits: 0,
                length: 0,
            };
            DElt.sequence.0 = u16::to_le_bytes(symbol);
            DElt.nbBits = nbBits as u8;
            DElt.length = 1;
            for i in start..end {
                DTable[i as usize] = DElt;
            }
        }
        let fresh34 = &mut rankVal[weight as usize];
        *fresh34 = (*fresh34).wrapping_add(length);
        s = s.wrapping_add(1);
    }
}
unsafe fn HUFv05_readDTableX4(DTable: &mut [u32; 4097], src: &[u8]) -> Result<size_t, Error> {
    let mut weightList = [0; HUFv05_MAX_SYMBOL_VALUE as usize + 1];
    let mut sortedSymbol: [sortedSymbol_t; 256] = [sortedSymbol_t {
        symbol: 0,
        weight: 0,
    }; 256];
    let mut rankStats: [u32; 17] = [0; 17];
    let mut rankStart0: [u32; 18] = [0; 18];
    let rankStart = rankStart0.as_mut_ptr().add(1);
    let mut rankVal: rankVal_t =
        [[0; HUFv05_ABSOLUTEMAX_TABLELOG + 1]; HUFv05_ABSOLUTEMAX_TABLELOG];
    let mut tableLog: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let memLog = DTable[0];
    let dt = (&mut mem::transmute::<&mut [u32; 4097], &mut [HUFv05_DEltX4; 4097]>(DTable)[1..])
        .try_into()
        .unwrap();
    if memLog > HUFv05_ABSOLUTEMAX_TABLELOG as u32 {
        return Err(Error::tableLog_tooLarge);
    }
    let iSize = HUFv05_readStats(
        &mut weightList,
        &mut rankStats,
        &mut nbSymbols,
        &mut tableLog,
        src,
    )?;
    if tableLog > memLog {
        return Err(Error::tableLog_tooLarge);
    }
    let mut maxW = tableLog;
    while rankStats[maxW as usize] == 0 {
        maxW = maxW.wrapping_sub(1);
    }
    let mut nextRankStart = 0u32;
    let mut w: u32 = 1;
    while w <= maxW {
        let current = nextRankStart;
        nextRankStart = nextRankStart.wrapping_add(rankStats[w as usize]);
        *rankStart.offset(w as isize) = current;
        w = w.wrapping_add(1);
    }
    *rankStart = nextRankStart;
    let sizeOfSort = nextRankStart;
    let mut s: u32 = 0;
    while s < nbSymbols {
        let w_0 = u32::from(weightList[s as usize]);
        let fresh35 = &mut (*rankStart.offset(w_0 as isize));
        let fresh36 = *fresh35;
        *fresh35 = (*fresh35).wrapping_add(1);
        let r = fresh36;
        sortedSymbol[r as usize].symbol = s as u8;
        sortedSymbol[r as usize].weight = w_0 as u8;
        s = s.wrapping_add(1);
    }
    *rankStart = 0;
    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut nextRankVal = 0u32;
    let mut consumed: u32 = 0;
    let rescale = memLog.wrapping_sub(tableLog).wrapping_sub(1) as core::ffi::c_int;
    let rankVal0 = (*rankVal.as_mut_ptr()).as_mut_ptr();
    let mut w_1 = 1;
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
        &sortedSymbol[..sizeOfSort as usize],
        &rankStart0,
        &mut rankVal,
        maxW,
        tableLog.wrapping_add(1),
    );
    Ok(iSize)
}
unsafe fn HUFv05_decodeSymbolX4(
    op: *mut [u8; 2],
    DStream: &mut BITv05_DStream_t,
    dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = DStream.look_bits_fast(dtLog);
    op.cast::<[u8; 2]>().write((*dt.add(val)).sequence.0);
    DStream.skip_bits(u32::from((*dt.add(val)).nbBits));
    u32::from((*dt.add(val)).length)
}
unsafe fn HUFv05_decodeLastSymbolX4(
    op: *mut u8,
    DStream: &mut BITv05_DStream_t,
    dt: *const HUFv05_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = DStream.look_bits_fast(dtLog);
    ptr::copy_nonoverlapping(dt.add(val).cast::<u8>(), op.cast::<u8>(), 1);
    if (*dt.add(val)).length == 1 {
        DStream.skip_bits(u32::from((*dt.add(val)).nbBits));
    } else if DStream.bitsConsumed < usize::BITS {
        DStream.skip_bits(u32::from((*dt.add(val)).nbBits));
        if DStream.bitsConsumed > usize::BITS {
            DStream.bitsConsumed = usize::BITS;
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
    while bitDPtr.reload() == StreamStatus::Unfinished && p < pEnd.sub(7) {
        if MEM_64bits() {
            p = p.offset(HUFv05_decodeSymbolX4(p as *mut [u8; 2], bitDPtr, dt, dtLog) as isize);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            p = p.offset(HUFv05_decodeSymbolX4(p as *mut [u8; 2], bitDPtr, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            p = p.offset(HUFv05_decodeSymbolX4(p as *mut [u8; 2], bitDPtr, dt, dtLog) as isize);
        }
        p = p.offset(HUFv05_decodeSymbolX4(p as *mut [u8; 2], bitDPtr, dt, dtLog) as isize);
    }
    while bitDPtr.reload() == StreamStatus::Unfinished && p <= pEnd.sub(2) {
        p = p.offset(HUFv05_decodeSymbolX4(p as *mut [u8; 2], bitDPtr, dt, dtLog) as isize);
    }
    while p <= pEnd.sub(2) {
        p = p.offset(HUFv05_decodeSymbolX4(p as *mut [u8; 2], bitDPtr, dt, dtLog) as isize);
    }
    if p < pEnd {
        p = p.offset(HUFv05_decodeLastSymbolX4(p, bitDPtr, dt, dtLog) as isize);
    }
    p.offset_from_unsigned(pStart)
}
unsafe fn HUFv05_decompress1X4_usingDTable(
    mut dst: Writer<'_>,
    cSrc: &[u8],
    DTable: *const core::ffi::c_uint,
) -> Result<(), Error> {
    let ostart = dst.as_mut_ptr();
    let oend = ostart.add(dst.capacity());
    let dtLog = *DTable.add(0);
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX4).add(1);
    let mut bitD = BITv05_DStream_t::new(cSrc)?;
    HUFv05_decodeStreamX4(ostart, &mut bitD, oend, dt, dtLog);
    if !bitD.is_empty() {
        return Err(Error::corruption_detected);
    }
    Ok(())
}
unsafe fn HUFv05_decompress4X4_usingDTable(
    mut dst: Writer<'_>,
    cSrc: &[u8],
    DTable: *const core::ffi::c_uint,
) -> Result<size_t, Error> {
    if cSrc.len() < 10 {
        return Err(Error::corruption_detected);
    }
    let istart = cSrc.as_ptr();
    let ostart = dst.as_mut_ptr();
    let oend = ostart.add(dst.capacity());
    let dtPtr = DTable as *const core::ffi::c_void;
    let dt = (dtPtr as *const HUFv05_DEltX4).add(1);
    let dtLog = *DTable;
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.add(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.add(4) as *const core::ffi::c_void) as size_t;
    let istart1 = istart.add(6);
    let istart2 = istart1.add(length1);
    let istart3 = istart2.add(length2);
    let istart4 = istart3.add(length3);
    let segmentSize = dst.capacity().div_ceil(4);
    let opStart2 = ostart.add(segmentSize);
    let opStart3 = opStart2.add(segmentSize);
    let opStart4 = opStart3.add(segmentSize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let length4 = cSrc.len().wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    if length4 > cSrc.len() {
        return Err(Error::corruption_detected);
    }
    let mut bitD1 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart1, length1))?;
    let mut bitD2 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart2, length2))?;
    let mut bitD3 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart3, length3))?;
    let mut bitD4 = BITv05_DStream_t::new(core::slice::from_raw_parts(istart4, length4))?;
    let mut endSignal = true;
    endSignal &= bitD1.reload() == StreamStatus::Unfinished;
    endSignal &= bitD2.reload() == StreamStatus::Unfinished;
    endSignal &= bitD3.reload() == StreamStatus::Unfinished;
    endSignal &= bitD4.reload() == StreamStatus::Unfinished;
    while endSignal && op4 < oend.sub(7) {
        if MEM_64bits() {
            op1 = op1
                .offset(HUFv05_decodeSymbolX4(op1 as *mut [u8; 2], &mut bitD1, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            op2 = op2
                .offset(HUFv05_decodeSymbolX4(op2 as *mut [u8; 2], &mut bitD2, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            op3 = op3
                .offset(HUFv05_decodeSymbolX4(op3 as *mut [u8; 2], &mut bitD3, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            op4 = op4
                .offset(HUFv05_decodeSymbolX4(op4 as *mut [u8; 2], &mut bitD4, dt, dtLog) as isize);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            op1 = op1
                .offset(HUFv05_decodeSymbolX4(op1 as *mut [u8; 2], &mut bitD1, dt, dtLog) as isize);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            op2 = op2
                .offset(HUFv05_decodeSymbolX4(op2 as *mut [u8; 2], &mut bitD2, dt, dtLog) as isize);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            op3 = op3
                .offset(HUFv05_decodeSymbolX4(op3 as *mut [u8; 2], &mut bitD3, dt, dtLog) as isize);
        }
        if MEM_64bits() || HUFv05_MAX_TABLELOG <= 12 {
            op4 = op4
                .offset(HUFv05_decodeSymbolX4(op4 as *mut [u8; 2], &mut bitD4, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            op1 = op1
                .offset(HUFv05_decodeSymbolX4(op1 as *mut [u8; 2], &mut bitD1, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            op2 = op2
                .offset(HUFv05_decodeSymbolX4(op2 as *mut [u8; 2], &mut bitD2, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            op3 = op3
                .offset(HUFv05_decodeSymbolX4(op3 as *mut [u8; 2], &mut bitD3, dt, dtLog) as isize);
        }
        if MEM_64bits() {
            op4 = op4
                .offset(HUFv05_decodeSymbolX4(op4 as *mut [u8; 2], &mut bitD4, dt, dtLog) as isize);
        }
        op1 =
            op1.offset(HUFv05_decodeSymbolX4(op1 as *mut [u8; 2], &mut bitD1, dt, dtLog) as isize);
        op2 =
            op2.offset(HUFv05_decodeSymbolX4(op2 as *mut [u8; 2], &mut bitD2, dt, dtLog) as isize);
        op3 =
            op3.offset(HUFv05_decodeSymbolX4(op3 as *mut [u8; 2], &mut bitD3, dt, dtLog) as isize);
        op4 =
            op4.offset(HUFv05_decodeSymbolX4(op4 as *mut [u8; 2], &mut bitD4, dt, dtLog) as isize);
        endSignal &= bitD1.reload() == StreamStatus::Unfinished;
        endSignal &= bitD2.reload() == StreamStatus::Unfinished;
        endSignal &= bitD3.reload() == StreamStatus::Unfinished;
        endSignal &= bitD4.reload() == StreamStatus::Unfinished;
    }
    if op1 > opStart2 {
        return Err(Error::corruption_detected);
    }
    if op2 > opStart3 {
        return Err(Error::corruption_detected);
    }
    if op3 > opStart4 {
        return Err(Error::corruption_detected);
    }
    HUFv05_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv05_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv05_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv05_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    if !(bitD1.is_empty() && bitD2.is_empty() && bitD3.is_empty() && bitD4.is_empty()) {
        return Err(Error::corruption_detected);
    }
    Ok(dst.capacity())
}
fn HUFv05_decompress4X4(dst: Writer<'_>, cSrc: &[u8]) -> Result<size_t, Error> {
    let mut DTable: [core::ffi::c_uint; 4097] = [12; 4097];
    let hSize = unsafe { HUFv05_readDTableX4(&mut DTable, cSrc)? };
    if hSize >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    unsafe { HUFv05_decompress4X4_usingDTable(dst, &cSrc[hSize..], DTable.as_mut_ptr()) }
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
unsafe fn HUFv05_decompress(mut dst: Writer<'_>, cSrc: &[u8]) -> Result<size_t, Error> {
    #[allow(clippy::type_complexity)]
    static decompress: [fn(Writer<'_>, &[u8]) -> Result<size_t, Error>; 2] = [
        HUFv05_decompress4X2 as fn(Writer<'_>, &[u8]) -> Result<size_t, Error>,
        HUFv05_decompress4X4 as fn(Writer<'_>, &[u8]) -> Result<size_t, Error>,
    ];
    let D256 = (dst.capacity() >> 8) as u32;
    if dst.capacity() == 0 {
        return Err(Error::dstSize_tooSmall);
    }
    if cSrc.len() >= dst.capacity() {
        return Err(Error::corruption_detected);
    }
    if cSrc.len() == 1 {
        core::ptr::write_bytes(dst.as_mut_ptr().cast::<u8>(), cSrc[0], dst.capacity());
        return Ok(dst.capacity());
    }
    let Q = (cSrc.len() * 16 / dst.capacity()) as u32;
    let mut n = 0;
    let mut Dtime: [u32; 3] = [0; 3];
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
    let fresh37 = &mut (*Dtime.as_mut_ptr().add(1));
    *fresh37 = (*fresh37).wrapping_add(*Dtime.as_mut_ptr().add(1) >> 4);
    let fresh38 = &mut (*Dtime.as_mut_ptr().add(2));
    *fresh38 = (*fresh38).wrapping_add(*Dtime.as_mut_ptr().add(2) >> 3);
    let mut algoNb = 0;
    if *Dtime.as_mut_ptr().add(1) < *Dtime.as_mut_ptr() {
        algoNb = 1;
    }
    decompress[algoNb](dst, cSrc)
}
unsafe fn ZSTDv05_decompressBegin(dctx: *mut ZSTDv05_DCtx) {
    (*dctx).expected = ZSTDv05_frameHeaderSize_min;
    (*dctx).stage = ZSTDv05ds_getFrameHeaderSize;
    (*dctx).previousDstEnd = core::ptr::null();
    (*dctx).base = core::ptr::null();
    (*dctx).vBase = core::ptr::null();
    (*dctx).dictEnd = core::ptr::null();
    *((*dctx).hufTableX4).as_mut_ptr() = ZSTD_HUFFDTABLE_CAPACITY_LOG as core::ffi::c_uint;
    (*dctx).flagStaticTables = 0;
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
fn ZSTDv05_decodeFrameHeader_Part1(
    zc: &mut ZSTDv05_DCtx,
    src: Reader<'_>,
) -> Result<size_t, Error> {
    if src.len() != ZSTDv05_frameHeaderSize_min {
        return Err(Error::srcSize_wrong);
    }
    let magicNumber = unsafe { MEM_readLE32(src.as_ptr().cast()) };
    if magicNumber != ZSTDv05_MAGICNUMBER {
        return Err(Error::prefix_unknown);
    }
    zc.headerSize = ZSTDv05_frameHeaderSize_min;
    Ok(zc.headerSize)
}
pub(crate) fn ZSTDv05_getFrameParams(
    params: &mut ZSTDv05_parameters,
    src: &[u8],
) -> Result<size_t, Error> {
    if src.len() < ZSTDv05_frameHeaderSize_min {
        return Ok(ZSTDv05_frameHeaderSize_max as size_t);
    }
    let magicNumber = u32::from_le_bytes(src[..4].try_into().unwrap());
    if magicNumber != ZSTDv05_MAGICNUMBER {
        return Err(Error::prefix_unknown);
    }
    unsafe {
        ptr::write_bytes(
            params as *mut ZSTDv05_parameters as *mut u8,
            0,
            ::core::mem::size_of::<ZSTDv05_parameters>(),
        );
    }
    params.windowLog = u32::from((src[4] & 15) + ZSTDv05_WINDOWLOG_ABSOLUTEMIN);
    if src[4] >> 4 != 0 {
        return Err(Error::frameParameter_unsupported);
    }
    Ok(0)
}
unsafe fn ZSTDv05_decodeFrameHeader_Part2(
    zc: *mut ZSTDv05_DCtx,
    src: &[u8],
) -> Result<size_t, Error> {
    if src.len() != (*zc).headerSize {
        return Err(Error::srcSize_wrong);
    }
    let result = ZSTDv05_getFrameParams(&mut (*zc).params, src);
    if MEM_32bits() && (*zc).params.windowLog > 25 {
        return Err(Error::frameParameter_unsupported);
    }
    result
}
fn ZSTDv05_getcBlockSize(src: &[u8], bpPtr: &mut blockProperties_t) -> Result<size_t, Error> {
    if src.len() < 3 {
        return Err(Error::srcSize_wrong);
    }
    let headerFlags = src[0];
    let cSize = (core::ffi::c_int::from(src[2])
        + (core::ffi::c_int::from(src[1]) << 8)
        + ((core::ffi::c_int::from(src[0]) & 7) << 16)) as u32;
    bpPtr.blockType = (core::ffi::c_int::from(headerFlags) >> 6) as blockType_t;
    bpPtr.origSize = if bpPtr.blockType == bt_rle { cSize } else { 0 };
    if bpPtr.blockType == bt_end {
        return Ok(0);
    }
    if bpPtr.blockType == bt_rle {
        return Ok(1);
    }
    Ok(cSize as size_t)
}
unsafe fn ZSTDv05_copyRawBlock(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: Reader<'_>,
) -> Result<size_t, Error> {
    if dst.is_null() {
        return Err(Error::dstSize_tooSmall);
    }
    if src.len() > maxDstSize {
        return Err(Error::dstSize_tooSmall);
    }
    ptr::copy_nonoverlapping(src.as_ptr().cast(), dst, src.len());
    Ok(src.len())
}
unsafe fn ZSTDv05_decodeLiteralsBlock(
    dctx: &mut ZSTDv05_DCtx,
    src: Reader<'_>,
) -> Result<size_t, Error> {
    let istart = src.as_ptr();
    if src.len() < MIN_CBLOCK_SIZE as size_t {
        return Err(Error::corruption_detected);
    }
    match core::ffi::c_int::from(*istart) >> 6 {
        IS_HUFv05 => {
            let mut litSize: size_t = 0;
            let mut litCSize: size_t = 0;
            let mut singleStream = 0;
            let mut lhSize = (core::ffi::c_int::from(*istart.add(0)) >> 4 & 3) as u32;
            if src.len() < 5 {
                return Err(Error::corruption_detected);
            }
            match lhSize {
                2 => {
                    lhSize = 4;
                    litSize = (((core::ffi::c_int::from(*istart) & 15) << 10)
                        + (core::ffi::c_int::from(*istart.add(1)) << 2)
                        + (core::ffi::c_int::from(*istart.add(2)) >> 6))
                        as size_t;
                    litCSize = (((core::ffi::c_int::from(*istart.add(2)) & 63) << 8)
                        + core::ffi::c_int::from(*istart.add(3)))
                        as size_t;
                }
                3 => {
                    lhSize = 5;
                    litSize = (((core::ffi::c_int::from(*istart) & 15) << 14)
                        + (core::ffi::c_int::from(*istart.add(1)) << 6)
                        + (core::ffi::c_int::from(*istart.add(2)) >> 2))
                        as size_t;
                    litCSize = (((core::ffi::c_int::from(*istart.add(2)) & 3) << 16)
                        + (core::ffi::c_int::from(*istart.add(3)) << 8)
                        + core::ffi::c_int::from(*istart.add(4)))
                        as size_t;
                }
                0 | 1 => {
                    lhSize = 3;
                    singleStream = (core::ffi::c_int::from(*istart) & 16) as size_t;
                    litSize = (((core::ffi::c_int::from(*istart) & 15) << 6)
                        + (core::ffi::c_int::from(*istart.add(1)) >> 2))
                        as size_t;
                    litCSize = (((core::ffi::c_int::from(*istart.add(1)) & 3) << 8)
                        + core::ffi::c_int::from(*istart.add(2)))
                        as size_t;
                }
                _ => unreachable!(),
            }
            if litSize > BLOCKSIZE as size_t {
                return Err(Error::corruption_detected);
            }
            if litCSize.wrapping_add(lhSize as size_t) > src.len() {
                return Err(Error::corruption_detected);
            }
            if singleStream != 0 {
                HUFv05_decompress1X2(
                    Writer::from_raw_parts(dctx.litBuffer.as_mut_ptr(), litSize),
                    core::slice::from_raw_parts(istart.add(lhSize as usize), litCSize),
                )
                .map_err(|_| Error::corruption_detected)?;
            } else {
                HUFv05_decompress(
                    Writer::from_raw_parts(dctx.litBuffer.as_mut_ptr(), litSize),
                    core::slice::from_raw_parts(istart.add(lhSize as usize), litCSize),
                )
                .map_err(|_| Error::corruption_detected)?;
            }
            dctx.litPtr = (&raw mut dctx.litBuffer).cast();
            dctx.litSize = litSize;
            ptr::write_bytes(
                dctx.litBuffer.as_mut_ptr().add(dctx.litSize),
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            Ok(litCSize.wrapping_add(lhSize as size_t))
        }
        IS_PCH => {
            let mut litSize_0: size_t = 0;
            let mut litCSize_0: size_t = 0;
            let mut lhSize = (core::ffi::c_int::from(*istart) >> 4 & 3) as u32;
            if lhSize != 1 {
                return Err(Error::corruption_detected);
            }
            if dctx.flagStaticTables == 0 {
                return Err(Error::dictionary_corrupted);
            }
            lhSize = 3;
            litSize_0 = (((core::ffi::c_int::from(*istart.add(0)) & 15) << 6)
                + (core::ffi::c_int::from(*istart.add(1)) >> 2)) as size_t;
            litCSize_0 = (((core::ffi::c_int::from(*istart.add(1)) & 3) << 8)
                + core::ffi::c_int::from(*istart.add(2))) as size_t;
            if litCSize_0.wrapping_add(lhSize as size_t) > src.len() {
                return Err(Error::corruption_detected);
            }
            HUFv05_decompress1X4_usingDTable(
                Writer::from_raw_parts(dctx.litBuffer.as_mut_ptr(), litSize_0),
                core::slice::from_raw_parts(istart.add(lhSize as usize), litCSize_0),
                dctx.hufTableX4.as_mut_ptr(),
            )
            .map_err(|_| Error::corruption_detected)?;
            dctx.litPtr = dctx.litBuffer.as_mut_ptr();
            dctx.litSize = litSize_0;
            ptr::write_bytes(
                (dctx.litBuffer).as_mut_ptr().add(dctx.litSize),
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            Ok(litCSize_0.wrapping_add(lhSize as size_t))
        }
        IS_RAW => {
            let mut litSize_1: size_t = 0;
            let mut lhSize_1 = (core::ffi::c_int::from(*istart) >> 4 & 3) as u32;
            match lhSize_1 {
                2 => {
                    litSize_1 = (((core::ffi::c_int::from(*istart) & 15) << 8)
                        + core::ffi::c_int::from(*istart.add(1)))
                        as size_t;
                }
                3 => {
                    litSize_1 = (((core::ffi::c_int::from(*istart) & 15) << 16)
                        + (core::ffi::c_int::from(*istart.add(1)) << 8)
                        + core::ffi::c_int::from(*istart.add(2)))
                        as size_t;
                }
                0 | 1 => {
                    lhSize_1 = 1;
                    litSize_1 = (core::ffi::c_int::from(*istart) & 31) as size_t;
                }
                _ => unreachable!(),
            }
            if (lhSize_1 as size_t)
                .wrapping_add(litSize_1)
                .wrapping_add(WILDCOPY_OVERLENGTH as size_t)
                > src.len()
            {
                if litSize_1.wrapping_add(lhSize_1 as size_t) > src.len() {
                    return Err(Error::corruption_detected);
                }
                ptr::copy_nonoverlapping(
                    istart.add(lhSize_1 as usize),
                    dctx.litBuffer.as_mut_ptr(),
                    litSize_1,
                );
                dctx.litPtr = dctx.litBuffer.as_mut_ptr();
                dctx.litSize = litSize_1;
                ptr::write_bytes(
                    dctx.litBuffer.as_mut_ptr().add(dctx.litSize),
                    0,
                    WILDCOPY_OVERLENGTH as usize,
                );
                return Ok((lhSize_1 as size_t).wrapping_add(litSize_1));
            }
            dctx.litPtr = istart.offset(lhSize_1 as isize);
            dctx.litSize = litSize_1;
            Ok((lhSize_1 as size_t).wrapping_add(litSize_1))
        }
        IS_RLE => {
            let mut litSize_2: size_t = 0;
            let mut lhSize_2 = (core::ffi::c_int::from(*istart) >> 4 & 3) as u32;
            match lhSize_2 {
                2 => {
                    litSize_2 = (((core::ffi::c_int::from(*istart) & 15) << 8)
                        + core::ffi::c_int::from(*istart.add(1)))
                        as size_t;
                }
                3 => {
                    litSize_2 = (((core::ffi::c_int::from(*istart) & 15) << 16)
                        + (core::ffi::c_int::from(*istart.add(1)) << 8)
                        + core::ffi::c_int::from(*istart.add(2)))
                        as size_t;
                    if src.len() < 4 {
                        return Err(Error::corruption_detected);
                    }
                }
                0 | 1 => {
                    lhSize_2 = 1;
                    litSize_2 = (core::ffi::c_int::from(*istart) & 31) as size_t;
                }
                _ => unreachable!(),
            }
            if litSize_2 > BLOCKSIZE as size_t {
                return Err(Error::corruption_detected);
            }
            core::ptr::write_bytes(
                dctx.litBuffer.as_mut_ptr(),
                *istart.offset(lhSize_2 as isize),
                litSize_2.wrapping_add(WILDCOPY_OVERLENGTH as size_t),
            );
            dctx.litPtr = (&raw mut dctx.litBuffer).cast();
            dctx.litSize = litSize_2;
            Ok(lhSize_2.wrapping_add(1) as size_t)
        }
        _ => Err(Error::corruption_detected),
    }
}
unsafe fn ZSTDv05_decodeSeqHeaders(
    nbSeq: *mut core::ffi::c_int,
    dumpsPtr: &mut &[u8],
    DTableLL: &mut FSEv05_DTable<1024>,
    DTableML: &mut FSEv05_DTable<1024>,
    DTableOffb: &mut FSEv05_DTable<512>,
    src: Reader<'_>,
    flagStaticTable: u32,
) -> Result<size_t, Error> {
    let istart = src.as_ptr();
    let mut ip = istart;
    let iend = istart.add(src.len());
    let mut LLtype: u32 = 0;
    let mut Offtype: u32 = 0;
    let mut MLtype: u32 = 0;
    let mut LLlog: core::ffi::c_uint = 0;
    let mut Offlog: core::ffi::c_uint = 0;
    let mut MLlog: core::ffi::c_uint = 0;
    let mut dumpsLength: size_t = 0;
    if src.len() < MIN_SEQUENCES_SIZE as size_t {
        return Err(Error::srcSize_wrong);
    }
    let fresh39 = ip;
    ip = ip.add(1);
    *nbSeq = core::ffi::c_int::from(*fresh39);
    if *nbSeq == 0 {
        return Ok(1);
    }
    if *nbSeq >= 128 {
        if ip >= iend {
            return Err(Error::srcSize_wrong);
        }
        let fresh40 = ip;
        ip = ip.add(1);
        *nbSeq = ((*nbSeq - 128) << 8) + core::ffi::c_int::from(*fresh40);
    }
    if ip >= iend {
        return Err(Error::srcSize_wrong);
    }
    LLtype = (core::ffi::c_int::from(*ip) >> 6) as u32;
    Offtype = (core::ffi::c_int::from(*ip) >> 4 & 3) as u32;
    MLtype = (core::ffi::c_int::from(*ip) >> 2 & 3) as u32;
    if core::ffi::c_int::from(*ip) & 2 != 0 {
        if ip.add(3) > iend {
            return Err(Error::srcSize_wrong);
        }
        dumpsLength = *ip.add(2) as size_t;
        dumpsLength = dumpsLength.wrapping_add((core::ffi::c_int::from(*ip.add(1)) << 8) as size_t);
        ip = ip.add(3);
    } else {
        if ip.add(2) > iend {
            return Err(Error::srcSize_wrong);
        }
        dumpsLength = *ip.add(1) as size_t;
        dumpsLength = dumpsLength.wrapping_add(((core::ffi::c_int::from(*ip) & 1) << 8) as size_t);
        ip = ip.add(2);
    }
    *dumpsPtr = core::slice::from_raw_parts(ip, dumpsLength);
    ip = ip.add(dumpsLength);
    if ip > iend.sub(3) {
        return Err(Error::srcSize_wrong);
    }
    let mut norm: [i16; 128] = [0; 128];
    let mut headerSize: size_t = 0;
    match LLtype {
        1 => {
            LLlog = 0;
            let fresh41 = ip;
            ip = ip.add(1);
            FSEv05_buildDTable_rle(DTableLL, *fresh41);
        }
        0 => {
            LLlog = LLbits as core::ffi::c_uint;
            let _ = FSEv05_buildDTable_raw(DTableLL, LLbits as core::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return Err(Error::corruption_detected);
            }
        }
        3 => {
            let mut max = MaxLL as core::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                &mut norm,
                &mut max,
                &mut LLlog,
                core::slice::from_raw_parts(ip, iend.offset_from_unsigned(ip)),
            )
            .map_err(|_| Error::GENERIC)?;
            if LLlog > LLFSEv05Log as core::ffi::c_uint {
                return Err(Error::corruption_detected);
            }
            ip = ip.add(headerSize);
            let _ = FSEv05_buildDTable(DTableLL, &norm, max, LLlog);
        }
        _ => unreachable!(),
    }
    match Offtype {
        1 => {
            Offlog = 0;
            if ip > iend.sub(2) {
                return Err(Error::srcSize_wrong);
            }
            let fresh42 = ip;
            ip = ip.add(1);
            FSEv05_buildDTable_rle(
                DTableOffb,
                *fresh42 as core::ffi::c_uchar & (MaxOff as core::ffi::c_uchar),
            );
        }
        0 => {
            Offlog = Offbits as core::ffi::c_uint;
            let _ = FSEv05_buildDTable_raw(DTableOffb, Offbits as core::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return Err(Error::corruption_detected);
            }
        }
        3 => {
            let mut max = MaxOff as core::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                &mut norm,
                &mut max,
                &mut Offlog,
                core::slice::from_raw_parts(ip, iend.offset_from_unsigned(ip)),
            )
            .map_err(|_| Error::GENERIC)?;
            if Offlog > OffFSEv05Log as core::ffi::c_uint {
                return Err(Error::corruption_detected);
            }
            ip = ip.add(headerSize);
            let _ = FSEv05_buildDTable(DTableOffb, &norm, max, Offlog);
        }
        _ => unreachable!(),
    }
    match MLtype {
        1 => {
            MLlog = 0;
            if ip > iend.sub(2) {
                return Err(Error::srcSize_wrong);
            }
            let fresh43 = ip;
            ip = ip.add(1);
            FSEv05_buildDTable_rle(DTableML, *fresh43);
        }
        0 => {
            MLlog = MLbits as core::ffi::c_uint;
            let _ = FSEv05_buildDTable_raw(DTableML, MLbits as core::ffi::c_uint);
        }
        2 => {
            if flagStaticTable == 0 {
                return Err(Error::corruption_detected);
            }
        }
        3 => {
            let mut max = MaxML as core::ffi::c_uint;
            headerSize = FSEv05_readNCount(
                &mut norm,
                &mut max,
                &mut MLlog,
                core::slice::from_raw_parts(ip, iend.offset_from_unsigned(ip)),
            )
            .map_err(|_| Error::GENERIC)?;
            if MLlog > MLFSEv05Log as core::ffi::c_uint {
                return Err(Error::corruption_detected);
            }
            ip = ip.add(headerSize);
            let _ = FSEv05_buildDTable(DTableML, &norm, max, MLlog);
        }
        _ => unreachable!(),
    }
    Ok(ip.offset_from_unsigned(istart))
}
unsafe fn ZSTDv05_decodeSequence(seq: &mut seq_t, seqState: &mut seqState_t) {
    let mut litLength: size_t = 0;
    let mut prevOffset: size_t = 0;
    let mut offset: size_t = 0;
    let mut matchLength: size_t = 0;
    let mut dumps = seqState.dumps.as_ptr();
    let de = dumps.add(seqState.dumps.len());
    litLength = FSEv05_peakSymbol(&mut seqState.stateLL) as size_t;
    prevOffset = if litLength != 0 {
        seq.offset
    } else {
        seqState.prevOffset
    };
    if litLength == MaxLL as size_t {
        let fresh44 = dumps;
        dumps = dumps.add(1);
        let add = u32::from(*fresh44);
        if add < 255 {
            litLength = litLength.wrapping_add(add as size_t);
        } else if dumps.add(2) <= de {
            litLength = MEM_readLE16(dumps as *const core::ffi::c_void) as size_t;
            dumps = dumps.add(2);
            if litLength & 1 != 0 && dumps < de {
                litLength =
                    litLength.wrapping_add((core::ffi::c_int::from(*dumps) << 16) as size_t);
                dumps = dumps.add(1);
            }
            litLength >>= 1;
        }
        if dumps >= de {
            dumps = de.sub(1);
        }
    }
    static offsetPrefix: [u32; 32] = [
        1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
        131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 1, 1, 1, 1,
        1,
    ];
    let offsetCode = u32::from(FSEv05_peakSymbol(&mut seqState.stateOffb));
    let mut nbBits = offsetCode.wrapping_sub(1);
    if offsetCode == 0 {
        nbBits = 0;
    }
    offset = (*offsetPrefix.as_ptr().offset(offsetCode as isize) as size_t)
        .wrapping_add(seqState.DStream.read_bits(nbBits));
    if MEM_32bits() {
        seqState.DStream.reload();
    }
    if offsetCode == 0 {
        offset = prevOffset;
    }
    if offsetCode | core::ffi::c_int::from(litLength == 0) as u32 != 0 {
        seqState.prevOffset = seq.offset;
    }
    FSEv05_decodeSymbol(&mut seqState.stateOffb, &mut seqState.DStream);
    FSEv05_decodeSymbol(&mut seqState.stateLL, &mut seqState.DStream);
    if MEM_32bits() {
        seqState.DStream.reload();
    }
    matchLength = FSEv05_decodeSymbol(&mut seqState.stateML, &mut seqState.DStream) as size_t;
    if matchLength == MaxML as size_t {
        let add_0 = (if dumps < de {
            let fresh45 = dumps;
            dumps = dumps.add(1);
            core::ffi::c_int::from(*fresh45)
        } else {
            0
        }) as u32;
        if add_0 < 255 {
            matchLength = matchLength.wrapping_add(add_0 as size_t);
        } else if dumps.add(2) <= de {
            matchLength = MEM_readLE16(dumps as *const core::ffi::c_void) as size_t;
            dumps = dumps.add(2);
            if matchLength & 1 != 0 && dumps < de {
                matchLength =
                    matchLength.wrapping_add((core::ffi::c_int::from(*dumps) << 16) as size_t);
                dumps = dumps.add(1);
            }
            matchLength >>= 1;
        }
        if dumps >= de {
            dumps = de.sub(1);
        }
    }
    matchLength = matchLength.wrapping_add(MINMATCH as size_t);
    seq.litLength = litLength;
    seq.offset = offset;
    seq.matchLength = matchLength;
    seqState.dumps = core::slice::from_raw_parts(dumps, de.offset_from_unsigned(dumps));
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
) -> Result<size_t, Error> {
    static dec32table: [core::ffi::c_int; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
    static dec64table: [core::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.add(sequenceLength);
    let oend_8 = oend.wrapping_sub(8);
    let litEnd = (*litPtr).add(sequence.litLength);
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset);
    let seqLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    if seqLength > oend.offset_from_unsigned(op) {
        return Err(Error::dstSize_tooSmall);
    }
    if sequence.litLength > litLimit.offset_from_unsigned(*litPtr) {
        return Err(Error::corruption_detected);
    }
    if oLitEnd > oend_8 {
        return Err(Error::dstSize_tooSmall);
    }
    if oMatchEnd > oend {
        return Err(Error::dstSize_tooSmall);
    }
    if litEnd > litLimit {
        return Err(Error::corruption_detected);
    }
    ZSTDv05_wildcopy(op, *litPtr, sequence.litLength as ptrdiff_t);
    op = oLitEnd;
    *litPtr = litEnd;
    if sequence.offset > oLitEnd.offset_from_unsigned(base) {
        if sequence.offset > oLitEnd.offset_from_unsigned(vBase) {
            return Err(Error::corruption_detected);
        }
        match_0 = dictEnd.offset(-(base.offset_from(match_0) as core::ffi::c_long as isize));
        if match_0.add(sequence.matchLength) <= dictEnd {
            core::ptr::copy(match_0, oLitEnd, sequence.matchLength);
            return Ok(sequenceLength);
        }
        let length1 = dictEnd.offset_from_unsigned(match_0);
        core::ptr::copy(match_0, oLitEnd, length1);
        op = oLitEnd.add(length1);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = base;
        if op > oend_8 || sequence.matchLength < MINMATCH as size_t {
            while op < oMatchEnd {
                let fresh46 = match_0;
                match_0 = match_0.add(1);
                let fresh47 = op;
                op = op.add(1);
                *fresh47 = *fresh46;
            }
            return Ok(sequenceLength);
        }
    }
    if sequence.offset < 8 {
        let sub2 = *dec64table.as_ptr().add(sequence.offset);
        *op = *match_0;
        *op.add(1) = *match_0.add(1);
        *op.add(2) = *match_0.add(2);
        *op.add(3) = *match_0.add(3);
        match_0 = match_0.offset(*dec32table.as_ptr().add(sequence.offset) as isize);
        {
            ptr::copy_nonoverlapping(match_0, op.add(4), 4);
        };
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ptr::copy_nonoverlapping(match_0, op, 8);
    }
    op = op.add(8);
    match_0 = match_0.add(8);
    if oMatchEnd > oend.sub((16 - MINMATCH) as usize) {
        if op < oend_8 {
            ZSTDv05_wildcopy(op, match_0, oend_8.offset_from(op) as ptrdiff_t);
            match_0 = match_0.offset(oend_8.offset_from(op) as core::ffi::c_long as isize);
            op = oend_8;
        }
        while op < oMatchEnd {
            let fresh48 = match_0;
            match_0 = match_0.add(1);
            let fresh49 = op;
            op = op.add(1);
            *fresh49 = *fresh48;
        }
    } else {
        ZSTDv05_wildcopy(op, match_0, sequence.matchLength as ptrdiff_t - 8);
    }
    Ok(sequenceLength)
}
unsafe fn ZSTDv05_decompressSequences(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    mut seq: Reader<'_>,
) -> Result<size_t, Error> {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(maxDstSize);
    let mut litPtr = dctx.litPtr;
    let litEnd = litPtr.add(dctx.litSize);
    let mut nbSeq = 0;
    let mut dumps = &[][..];
    let DTableLL = &mut dctx.LLTable;
    let DTableML = &mut dctx.MLTable;
    let DTableOffb = &mut dctx.OffTable;
    let base = dctx.base as *const u8;
    let vBase = dctx.vBase as *const u8;
    let dictEnd = dctx.dictEnd as *const u8;
    let headerSize = ZSTDv05_decodeSeqHeaders(
        &mut nbSeq,
        &mut dumps,
        DTableLL,
        DTableML,
        DTableOffb,
        seq.subslice(..),
        dctx.flagStaticTables,
    )?;
    seq = seq.subslice(headerSize..);
    if nbSeq != 0 {
        let mut DStream = match BITv05_DStream_t::new(seq.as_slice()) {
            Ok(bitD) => bitD,
            Err(_) => return Err(Error::corruption_detected),
        };
        let mut seqState = seqState_t {
            stateLL: FSEv05_initDState(&mut DStream, DTableLL),
            stateOffb: FSEv05_initDState(&mut DStream, DTableOffb),
            stateML: FSEv05_initDState(&mut DStream, DTableML),
            DStream,
            prevOffset: REPCODE_STARTVALUE as size_t,
            dumps,
        };

        let mut sequence = seq_t {
            litLength: 0,
            matchLength: 0,
            offset: REPCODE_STARTVALUE as size_t,
        };
        while seqState.DStream.reload() <= StreamStatus::Completed && nbSeq != 0 {
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
            )?;
            op = op.add(oneSeqSize);
        }
        if nbSeq != 0 {
            return Err(Error::corruption_detected);
        }
    }
    let lastLLSize = litEnd.offset_from_unsigned(litPtr);
    if litPtr > litEnd {
        return Err(Error::corruption_detected);
    }
    if op.add(lastLLSize) > oend {
        return Err(Error::dstSize_tooSmall);
    }
    if lastLLSize > 0 {
        ptr::copy_nonoverlapping(litPtr, op, lastLLSize);
        op = op.add(lastLLSize);
    }
    Ok(op.offset_from_unsigned(ostart))
}
unsafe fn ZSTDv05_checkContinuity(dctx: &mut ZSTDv05_DCtx, dst: *const core::ffi::c_void) {
    if dst != dctx.previousDstEnd {
        dctx.dictEnd = dctx.previousDstEnd;
        dctx.vBase = (dst as *const core::ffi::c_char).wrapping_sub(
            (dctx.previousDstEnd as *const core::ffi::c_char)
                .offset_from_unsigned(dctx.base as *const core::ffi::c_char),
        ) as *const core::ffi::c_void;
        dctx.base = dst;
        dctx.previousDstEnd = dst;
    }
}
unsafe fn ZSTDv05_decompressBlock_internal(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: Reader<'_>,
) -> Result<size_t, Error> {
    let mut litCSize: size_t = 0;
    if src.len() >= BLOCKSIZE as size_t {
        return Err(Error::srcSize_wrong);
    }
    litCSize = ZSTDv05_decodeLiteralsBlock(dctx, src.subslice(..))?;
    ZSTDv05_decompressSequences(dctx, dst, dstCapacity, src.subslice(litCSize..))
}
unsafe fn ZSTDv05_decompress_continueDCtx(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: Reader<'_>,
) -> Result<size_t, Error> {
    let mut ip = src.as_ptr();
    let iend = ip.add(src.len());
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(maxDstSize);
    let mut remainingSize = src.len();
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
    if src.len() < ZSTDv05_frameHeaderSize_min.wrapping_add(ZSTDv05_blockHeaderSize) {
        return Err(Error::srcSize_wrong);
    }
    frameHeaderSize =
        ZSTDv05_decodeFrameHeader_Part1(dctx, src.subslice(..ZSTDv05_frameHeaderSize_min))?;
    if src.len() < frameHeaderSize.wrapping_add(ZSTDv05_blockHeaderSize) {
        return Err(Error::srcSize_wrong);
    }
    ip = ip.add(frameHeaderSize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    frameHeaderSize =
        ZSTDv05_decodeFrameHeader_Part2(dctx, src.subslice(..frameHeaderSize).as_slice())?;
    loop {
        let cBlockSize = ZSTDv05_getcBlockSize(
            core::slice::from_raw_parts(ip, iend.offset_from_unsigned(ip)),
            &mut blockProperties,
        )?;
        ip = ip.add(ZSTDv05_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv05_blockHeaderSize);
        if cBlockSize > remainingSize {
            return Err(Error::srcSize_wrong);
        }
        let mut decodedSize = Ok(0);
        match blockProperties.blockType as core::ffi::c_uint {
            0 => {
                decodedSize = ZSTDv05_decompressBlock_internal(
                    dctx,
                    op as *mut core::ffi::c_void,
                    oend.offset_from_unsigned(op),
                    Reader::from_raw_parts(ip, cBlockSize),
                );
            }
            1 => {
                decodedSize = ZSTDv05_copyRawBlock(
                    op as *mut core::ffi::c_void,
                    oend.offset_from_unsigned(op),
                    Reader::from_raw_parts(ip, cBlockSize),
                );
            }
            2 => return Err(Error::GENERIC),
            3 => {
                if remainingSize != 0 {
                    return Err(Error::srcSize_wrong);
                }
            }
            _ => return Err(Error::GENERIC),
        }
        if cBlockSize == 0 {
            break;
        }
        let decodedSize = decodedSize?;
        op = op.add(decodedSize);
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
    }
    Ok(op.offset_from_unsigned(ostart))
}
pub(crate) unsafe fn ZSTDv05_decompress_usingDict(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: Reader<'_>,
    dict: &[u8],
) -> Result<size_t, Error> {
    let _ = ZSTDv05_decompressBegin_usingDict(dctx, dict);
    ZSTDv05_checkContinuity(dctx, dst);
    ZSTDv05_decompress_continueDCtx(dctx, dst, maxDstSize, src)
}
fn ZSTD_errorFrameSizeInfoLegacy(
    cSize: &mut size_t,
    dBound: &mut core::ffi::c_ulonglong,
    ret: Error,
) {
    *cSize = ret.to_error_code();
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
pub(crate) fn ZSTDv05_findFrameSizeInfoLegacy(
    src: &[u8],
    cSize: &mut size_t,
    dBound: &mut core::ffi::c_ulonglong,
) {
    let mut ip = src;
    let mut nbBlocks = 0 as size_t;
    let mut blockProperties = blockProperties_t {
        blockType: bt_compressed,
        origSize: 0,
    };
    if src.len() < ZSTDv05_frameHeaderSize_min {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::srcSize_wrong);
        return;
    }
    if u32::from_le_bytes(src[..4].try_into().unwrap()) != ZSTDv05_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::prefix_unknown);
        return;
    }
    ip = &ip[ZSTDv05_frameHeaderSize_min..];
    loop {
        let cBlockSize = match ZSTDv05_getcBlockSize(ip, &mut blockProperties) {
            Ok(cBlockSize) => cBlockSize,
            Err(err) => {
                ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, err);
                return;
            }
        };
        ip = &ip[ZSTDv05_blockHeaderSize..];
        if cBlockSize > ip.len() {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::srcSize_wrong);
            return;
        }
        if cBlockSize == 0 {
            break;
        }
        ip = &ip[cBlockSize..];
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = src.len() - ip.len();
    *dBound = (nbBlocks * BLOCKSIZE as size_t) as core::ffi::c_ulonglong;
}
fn ZSTDv05_nextSrcSizeToDecompress(dctx: &mut ZSTDv05_DCtx) -> size_t {
    dctx.expected
}
unsafe fn ZSTDv05_decompressContinue(
    dctx: &mut ZSTDv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: Reader<'_>,
) -> Result<size_t, Error> {
    if src.len() != dctx.expected {
        return Err(Error::srcSize_wrong);
    }
    ZSTDv05_checkContinuity(dctx, dst);
    match dctx.stage as core::ffi::c_uint {
        0 => {
            if src.len() != ZSTDv05_frameHeaderSize_min {
                return Err(Error::srcSize_wrong);
            }
            dctx.headerSize =
                ZSTDv05_decodeFrameHeader_Part1(dctx, src.subslice(..ZSTDv05_frameHeaderSize_min))?;
            dctx.headerBuffer = src.as_slice()[..ZSTDv05_frameHeaderSize_min]
                .try_into()
                .unwrap();
            if dctx.headerSize > ZSTDv05_frameHeaderSize_min {
                return Err(Error::GENERIC);
            }
            dctx.expected = 0;
        }
        1 => {}
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_compressed,
                origSize: 0,
            };
            let blockSize =
                ZSTDv05_getcBlockSize(src.subslice(..ZSTDv05_blockHeaderSize).as_slice(), &mut bp)?;
            if bp.blockType as core::ffi::c_uint == bt_end as core::ffi::c_int as core::ffi::c_uint
            {
                dctx.expected = 0;
                dctx.stage = ZSTDv05ds_getFrameHeaderSize;
            } else {
                dctx.expected = blockSize;
                dctx.bType = bp.blockType;
                dctx.stage = ZSTDv05ds_decompressBlock;
            }
            return Ok(0);
        }
        3 => {
            let rSize = match dctx.bType as core::ffi::c_uint {
                0 => ZSTDv05_decompressBlock_internal(dctx, dst, maxDstSize, src),
                1 => ZSTDv05_copyRawBlock(dst, maxDstSize, src),
                2 => return Err(Error::GENERIC),
                3 => Ok(0),
                _ => return Err(Error::GENERIC),
            };
            dctx.stage = ZSTDv05ds_decodeBlockHeader;
            dctx.expected = ZSTDv05_blockHeaderSize;
            let rSize = rSize?;
            dctx.previousDstEnd =
                (dst as *mut core::ffi::c_char).add(rSize) as *const core::ffi::c_void;
            return Ok(rSize);
        }
        _ => return Err(Error::GENERIC),
    }
    ZSTDv05_decodeFrameHeader_Part2(dctx, &dctx.headerBuffer[..dctx.headerSize])?;
    dctx.expected = ZSTDv05_blockHeaderSize;
    dctx.stage = ZSTDv05ds_decodeBlockHeader;
    Ok(0)
}
unsafe fn ZSTDv05_refDictContent(dctx: &mut ZSTDv05_DCtx, dict: &[u8]) {
    dctx.dictEnd = dctx.previousDstEnd;
    dctx.vBase = dict.as_ptr().offset(
        -((dctx.previousDstEnd as *const core::ffi::c_char)
            .offset_from(dctx.base as *const core::ffi::c_char) as core::ffi::c_long
            as isize),
    ) as *const core::ffi::c_void;
    dctx.base = dict.as_ptr().cast();
    dctx.previousDstEnd = dict.as_ptr().add(dict.len()) as *const core::ffi::c_void;
}
unsafe fn ZSTDv05_loadEntropy(dctx: &mut ZSTDv05_DCtx, mut dict: &[u8]) -> Result<size_t, Error> {
    let hSize =
        HUFv05_readDTableX4(&mut dctx.hufTableX4, dict).map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[hSize..];

    let mut offcodeNCount: [core::ffi::c_short; 32] = [0; 32];
    let mut offcodeMaxValue = MaxOff as core::ffi::c_uint;
    let mut offcodeLog: core::ffi::c_uint = 0;
    let offcodeHeaderSize = FSEv05_readNCount(
        &mut offcodeNCount,
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dict,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    if offcodeLog > OffFSEv05Log as core::ffi::c_uint {
        return Err(Error::dictionary_corrupted);
    }

    let mut litlengthNCount: [core::ffi::c_short; 64] = [0; 64];
    let mut litlengthMaxValue = MaxLL as core::ffi::c_uint;
    let mut litlengthLog: core::ffi::c_uint = 0;
    FSEv05_buildDTable(
        &mut dctx.OffTable,
        &offcodeNCount,
        offcodeMaxValue,
        offcodeLog,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[offcodeHeaderSize..];

    let mut matchlengthNCount: [core::ffi::c_short; 128] = [0; 128];
    let mut matchlengthMaxValue = MaxML as core::ffi::c_uint;
    let mut matchlengthLog: core::ffi::c_uint = 0;
    let matchlengthHeaderSize = FSEv05_readNCount(
        &mut matchlengthNCount,
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dict,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    if matchlengthLog > MLFSEv05Log as core::ffi::c_uint {
        return Err(Error::dictionary_corrupted);
    }

    FSEv05_buildDTable(
        &mut dctx.MLTable,
        &matchlengthNCount,
        matchlengthMaxValue,
        matchlengthLog,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[matchlengthHeaderSize..];

    let litlengthHeaderSize = FSEv05_readNCount(
        &mut litlengthNCount,
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dict,
    )?;
    if litlengthLog > LLFSEv05Log as core::ffi::c_uint {
        return Err(Error::dictionary_corrupted);
    }

    FSEv05_buildDTable(
        &mut dctx.LLTable,
        &litlengthNCount,
        litlengthMaxValue,
        litlengthLog,
    )
    .map_err(|_| Error::dictionary_corrupted)?;

    dctx.flagStaticTables = 1;
    Ok(hSize
        .wrapping_add(offcodeHeaderSize)
        .wrapping_add(matchlengthHeaderSize)
        .wrapping_add(litlengthHeaderSize))
}
unsafe fn ZSTDv05_decompress_insertDictionary(
    dctx: &mut ZSTDv05_DCtx,
    mut dict: &[u8],
) -> Result<(), Error> {
    let magic = MEM_readLE32(dict.as_ptr().cast());
    if magic != ZSTDv05_DICT_MAGIC {
        ZSTDv05_refDictContent(dctx, dict);
        return Ok(());
    }
    dict = &dict[4..];
    let eSize = ZSTDv05_loadEntropy(dctx, dict).map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[eSize..];
    ZSTDv05_refDictContent(dctx, dict);
    Ok(())
}
unsafe fn ZSTDv05_decompressBegin_usingDict(
    dctx: &mut ZSTDv05_DCtx,
    dict: &[u8],
) -> Result<(), Error> {
    ZSTDv05_decompressBegin(dctx);
    if !dict.is_empty() {
        ZSTDv05_decompress_insertDictionary(dctx, dict).map_err(|_| Error::dictionary_corrupted)?;
    }
    Ok(())
}
static ZBUFFv05_blockHeaderSize: size_t = 3;
unsafe fn ZBUFFv05_limitCopy(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: &[u8],
) -> size_t {
    let length = Ord::min(maxDstSize, src.len());
    ptr::copy_nonoverlapping(src.as_ptr(), dst.cast::<u8>(), length);
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
    dict: &[u8],
) -> Result<(), Error> {
    (*zbc).stage = ZBUFFv05ds_readHeader;
    (*zbc).outEnd = 0;
    (*zbc).outStart = (*zbc).outEnd;
    (*zbc).inPos = (*zbc).outStart;
    (*zbc).hPos = (*zbc).inPos;
    ZSTDv05_decompressBegin_usingDict(&mut *(*zbc).zc, dict)
}

#[allow(clippy::drop_non_drop)]
pub(crate) unsafe fn ZBUFFv05_decompressContinue(
    zbc: &mut ZBUFFv05_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSizePtr: *mut size_t,
    src: *const u8,
    srcSizePtr: *mut size_t,
) -> Result<size_t, Error> {
    let istart = src;
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
            ZBUFFv05ds_init => return Err(Error::init_missing),
            ZBUFFv05ds_readHeader => {
                // read header from src
                let headerSize = ZSTDv05_getFrameParams(
                    &mut zbc.params,
                    core::slice::from_raw_parts(src, *srcSizePtr),
                )?;
                if headerSize != 0 {
                    // not enough input to decode header : tell how many bytes would be necessary
                    ptr::copy_nonoverlapping(
                        src,
                        (zbc.headerBuffer).as_mut_ptr().add(zbc.hPos),
                        *srcSizePtr,
                    );
                    zbc.hPos = (zbc.hPos).wrapping_add(*srcSizePtr);
                    *maxDstSizePtr = 0;
                    zbc.stage = ZBUFFv05ds_loadHeader;
                    return Ok(headerSize.wrapping_sub(zbc.hPos));
                }
                zbc.stage = ZBUFFv05ds_decodeHeader;
                continue;
            }
            ZBUFFv05ds_loadHeader => {
                // complete header from src
                let mut headerSize_0 = ZBUFFv05_limitCopy(
                    (zbc.headerBuffer).as_mut_ptr().add(zbc.hPos) as *mut core::ffi::c_void,
                    (ZSTDv05_frameHeaderSize_max_0 as size_t).wrapping_sub(zbc.hPos),
                    core::slice::from_raw_parts(src, *srcSizePtr),
                );
                zbc.hPos = (zbc.hPos).wrapping_add(headerSize_0);
                ip = ip.add(headerSize_0);
                headerSize_0 =
                    ZSTDv05_getFrameParams(&mut zbc.params, &zbc.headerBuffer[..zbc.hPos])?;
                if headerSize_0 != 0 {
                    // not enough input to decode header : tell how many bytes would be necessary
                    *maxDstSizePtr = 0;
                    return Ok(headerSize_0.wrapping_sub(zbc.hPos));
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
            _ => return Err(Error::GENERIC),
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
                    return Err(Error::memory_allocation);
                }
            }
            if zbc.outBuffSize < neededOutSize {
                free(zbc.outBuff as *mut core::ffi::c_void);
                zbc.outBuffSize = neededOutSize;
                zbc.outBuff = malloc(neededOutSize) as *mut core::ffi::c_char;
                if zbc.outBuff.is_null() {
                    return Err(Error::memory_allocation);
                }
            }
            if zbc.hPos != 0 {
                // some data already loaded into headerBuffer : transfer into inBuff
                ptr::copy_nonoverlapping(
                    (zbc.headerBuffer).as_ptr(),
                    zbc.inBuff.cast::<u8>(),
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

            let neededInSize = ZSTDv05_nextSrcSizeToDecompress(&mut *zbc.zc);
            if neededInSize == 0 {
                // end of frame
                zbc.stage = ZBUFFv05ds_init;
                notDone = 0;
                continue;
            }
            if iend.offset_from_unsigned(ip) >= neededInSize {
                // directly decode from src
                let decodedSize = ZSTDv05_decompressContinue(
                    &mut *zbc.zc,
                    (zbc.outBuff).add(zbc.outStart) as *mut core::ffi::c_void,
                    (zbc.outBuffSize).wrapping_sub(zbc.outStart),
                    Reader::from_raw_parts(ip, neededInSize),
                )?;
                ip = ip.add(neededInSize);
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

            let neededInSize = ZSTDv05_nextSrcSizeToDecompress(&mut *zbc.zc);
            // should always be <= remaining space within inBuff
            let toLoad = neededInSize.wrapping_sub(zbc.inPos);
            let mut loadedSize: size_t = 0;
            if toLoad > (zbc.inBuffSize).wrapping_sub(zbc.inPos) {
                return Err(Error::corruption_detected); // should never happen
            }
            loadedSize = ZBUFFv05_limitCopy(
                (zbc.inBuff).add(zbc.inPos) as *mut core::ffi::c_void,
                toLoad,
                core::slice::from_raw_parts(ip, iend.offset_from_unsigned(ip)),
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
                    Reader::from_raw_parts(zbc.inBuff as *const u8, neededInSize),
                )?;
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
                oend.offset_from_unsigned(op),
                core::slice::from_raw_parts(zbc.outBuff.add(zbc.outStart).cast(), toFlushSize),
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
    *srcSizePtr = ip.offset_from_unsigned(istart);
    *maxDstSizePtr = op.offset_from_unsigned(ostart);
    let mut nextSrcSizeHint = ZSTDv05_nextSrcSizeToDecompress(&mut *zbc.zc);
    if nextSrcSizeHint > ZBUFFv05_blockHeaderSize {
        nextSrcSizeHint = nextSrcSizeHint.wrapping_add(ZBUFFv05_blockHeaderSize);
    }
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub(zbc.inPos); // already loaded
    Ok(nextSrcSizeHint)
}
