use core::ptr;
use std::marker::PhantomData;

use libc::{free, malloc};

use crate::lib::common::error_private::Error;
use crate::lib::common::mem::{
    MEM_32bits, MEM_64bits, MEM_readLE16, MEM_readLE32, MEM_readLE64, MEM_readLEST, MEM_writeLE16,
};
use crate::lib::common::xxhash::{
    XXH64_state_t, ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_update,
};
use crate::lib::decompress::huf_decompress::DTableDesc;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ZSTDv07_frameParams {
    pub(crate) frameContentSize: core::ffi::c_ulonglong,
    pub(crate) windowSize: core::ffi::c_uint,
    pub(crate) dictID: core::ffi::c_uint,
    pub(crate) checksumFlag: core::ffi::c_uint,
}

#[repr(C)]
pub(crate) struct ZSTDv07_DCtx {
    LLTable: [FSEv07_DTable; 513],
    OffTable: [FSEv07_DTable; 257],
    MLTable: [FSEv07_DTable; 513],
    hufTable: [HUFv07_DTable; 4097],
    previousDstEnd: *const core::ffi::c_void,
    base: *const core::ffi::c_void,
    vBase: *const core::ffi::c_void,
    dictEnd: *const core::ffi::c_void,
    expected: usize,
    rep: [u32; 3],
    fParams: ZSTDv07_frameParams,
    bType: blockType_t,
    stage: ZSTDv07_dStage,
    litEntropy: u32,
    fseEntropy: u32,
    xxhState: XXH64_state_t,
    headerSize: usize,
    dictID: u32,
    litPtr: *const u8,
    customMem: ZSTDv07_customMem,
    litSize: usize,
    litBuffer: [u8; 131080],
    headerBuffer: [u8; 18],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ZSTDv07_customMem {
    customAlloc: ZSTDv07_allocFunction,
    customFree: ZSTDv07_freeFunction,
    opaque: *mut core::ffi::c_void,
}
type ZSTDv07_freeFunction = Option<unsafe fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> ()>;
type ZSTDv07_allocFunction =
    Option<unsafe fn(*mut core::ffi::c_void, usize) -> *mut core::ffi::c_void>;
type ZSTDv07_dStage = core::ffi::c_uint;
const ZSTDds_skipFrame: ZSTDv07_dStage = 5;
const ZSTDds_decodeSkippableHeader: ZSTDv07_dStage = 4;
const ZSTDds_decompressBlock: ZSTDv07_dStage = 3;
const ZSTDds_decodeBlockHeader: ZSTDv07_dStage = 2;
const ZSTDds_decodeFrameHeader: ZSTDv07_dStage = 1;
const ZSTDds_getFrameHeaderSize: ZSTDv07_dStage = 0;
type blockType_t = core::ffi::c_uint;
const bt_end: blockType_t = 3;
const bt_rle: blockType_t = 2;
const bt_raw: blockType_t = 1;
const bt_compressed: blockType_t = 0;
type HUFv07_DTable = u32;
type FSEv07_DTable = core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
struct blockProperties_t {
    blockType: blockType_t,
    origSize: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct seqState_t<'a> {
    DStream: BITv07_DStream_t<'a>,
    stateLL: FSEv07_DState_t,
    stateOffb: FSEv07_DState_t,
    stateML: FSEv07_DState_t,
    prevOffset: [usize; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv07_DState_t {
    state: usize,
    table: *const core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct seq_t {
    litLength: usize,
    matchLength: usize,
    offset: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv07_decode_t {
    newState: core::ffi::c_ushort,
    symbol: core::ffi::c_uchar,
    nbBits: core::ffi::c_uchar,
}
type BITv07_DStream_status = core::ffi::c_uint;
const BITv07_DStream_overflow: BITv07_DStream_status = 3;
const BITv07_DStream_completed: BITv07_DStream_status = 2;
const BITv07_DStream_endOfBuffer: BITv07_DStream_status = 1;
const BITv07_DStream_unfinished: BITv07_DStream_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
struct FSEv07_DTableHeader {
    tableLog: u16,
    fastMode: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct HUFv07_DEltX4 {
    sequence: u16,
    nbBits: u8,
    length: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct HUFv07_DEltX2 {
    byte: u8,
    nbBits: u8,
}
type DTable_max_t = [u32; 4097];
type rankVal_t = [[u32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
struct sortedSymbol_t {
    symbol: u8,
    weight: u8,
}
#[repr(C)]
struct algo_time_t {
    tableTime: u32,
    decode256Time: u32,
}
type litBlockType_t = core::ffi::c_uint;
#[repr(C)]
pub(crate) struct ZBUFFv07_DCtx_s {
    zd: *mut ZSTDv07_DCtx,
    fParams: ZSTDv07_frameParams,
    stage: ZBUFFv07_dStage,
    inBuff: *mut core::ffi::c_char,
    inBuffSize: usize,
    inPos: usize,
    outBuff: *mut core::ffi::c_char,
    outBuffSize: usize,
    outStart: usize,
    outEnd: usize,
    blockSize: usize,
    headerBuffer: [u8; 18],
    lhSize: usize,
    customMem: ZSTDv07_customMem,
}
type ZBUFFv07_dStage = core::ffi::c_uint;
const ZBUFFds_flush: ZBUFFv07_dStage = 4;
const ZBUFFds_load: ZBUFFv07_dStage = 3;
const ZBUFFds_read: ZBUFFv07_dStage = 2;
const ZBUFFds_loadHeader: ZBUFFv07_dStage = 1;
const ZBUFFds_init: ZBUFFv07_dStage = 0;
type ZBUFFv07_DCtx = ZBUFFv07_DCtx_s;
const ZSTDv07_MAGICNUMBER: core::ffi::c_uint = 0xfd2fb527 as core::ffi::c_uint;
const ZSTDv07_MAGIC_SKIPPABLE_START: core::ffi::c_uint = 0x184d2a50 as core::ffi::c_uint;
const ZSTDv07_WINDOWLOG_MAX_32: core::ffi::c_int = 25;
const ZSTDv07_WINDOWLOG_MAX_64: core::ffi::c_int = 27;
static ZSTDv07_frameHeaderSize_min: usize = 5;
static ZSTDv07_skippableHeaderSize: usize = 8;
const ZSTDv07_BLOCKSIZE_ABSOLUTEMAX: core::ffi::c_int = 128 * 1024;
#[inline]
fn BITv07_highbit32(val: u32) -> core::ffi::c_uint {
    (val.leading_zeros() as i32 ^ 31) as core::ffi::c_uint
}

#[derive(Copy, Clone)]
#[repr(C)]
struct BITv07_DStream_t<'a> {
    bitContainer: usize,
    bitsConsumed: core::ffi::c_uint,
    ptr: *const u8,
    start: *const u8,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> BITv07_DStream_t<'a> {
    #[inline]
    fn new(src: &'a [u8]) -> Result<Self, Error> {
        if src.is_empty() {
            return Err(Error::srcSize_wrong);
        }

        if src.len() >= ::core::mem::size_of::<usize>() {
            // normal case
            let mut bitD = BITv07_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: unsafe {
                    src.as_ptr()
                        .add(src.len())
                        .sub(::core::mem::size_of::<usize>())
                },
                start: src.as_ptr(),
                _marker: PhantomData,
            };

            bitD.bitContainer = unsafe { MEM_readLEST(bitD.ptr as *const core::ffi::c_void) };
            let lastByte = src[src.len() - 1];
            bitD.bitsConsumed = if lastByte != 0 {
                8 - BITv07_highbit32(lastByte as u32)
            } else {
                0
            };
            if lastByte == 0 {
                return Err(Error::GENERIC); // endMark not present
            }

            Ok(bitD)
        } else {
            let mut bitD = BITv07_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: src.as_ptr(),
                start: src.as_ptr(),
                _marker: PhantomData,
            };

            bitD.bitContainer = usize::from(src[0]);

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

            let lastByte = src[src.len() - 1];
            bitD.bitsConsumed = if lastByte != 0 {
                8 - BITv07_highbit32(lastByte as u32)
            } else {
                0
            };
            if lastByte == 0 {
                // endMark not present
                return Err(Error::GENERIC);
            }
            bitD.bitsConsumed += (::core::mem::size_of::<usize>() - src.len()) as u32 * 8;

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
        self.bitsConsumed = self.bitsConsumed.wrapping_add(nbBits);
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
    fn reload(&mut self) -> BITv07_DStream_status {
        if self.bitsConsumed > usize::BITS {
            return BITv07_DStream_overflow;
        }
        if self.ptr >= unsafe { (self.start).add(::core::mem::size_of::<usize>()) } {
            self.ptr = unsafe { (self.ptr).offset(-((self.bitsConsumed >> 3) as isize)) };
            self.bitsConsumed &= 7;
            self.bitContainer = unsafe { MEM_readLEST(self.ptr as *const core::ffi::c_void) };
            return BITv07_DStream_unfinished;
        }
        if self.ptr == self.start {
            if self.bitsConsumed < usize::BITS {
                return BITv07_DStream_endOfBuffer;
            }
            return BITv07_DStream_completed;
        }
        let mut nbBytes = self.bitsConsumed >> 3;
        let mut result = BITv07_DStream_unfinished;
        if unsafe { (self.ptr).offset(-(nbBytes as isize)) } < self.start {
            nbBytes = unsafe { (self.ptr).offset_from(self.start) } as core::ffi::c_long as u32;
            result = BITv07_DStream_endOfBuffer;
        }
        self.ptr = unsafe { (self.ptr).offset(-(nbBytes as isize)) };
        self.bitsConsumed = (self.bitsConsumed).wrapping_sub(nbBytes * 8);
        self.bitContainer = unsafe { MEM_readLEST(self.ptr as *const core::ffi::c_void) };
        result
    }
    #[inline]
    fn is_empty(&self) -> core::ffi::c_uint {
        (self.ptr == self.start && self.bitsConsumed == usize::BITS) as core::ffi::c_uint
    }
}

#[inline]
unsafe fn FSEv07_initDState(
    DStatePtr: *mut FSEv07_DState_t,
    bitD: &mut BITv07_DStream_t,
    dt: *const FSEv07_DTable,
) {
    let ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const FSEv07_DTableHeader;
    (*DStatePtr).state = bitD.read_bits((*DTableH).tableLog as core::ffi::c_uint);
    bitD.reload();
    (*DStatePtr).table = dt.add(1) as *const core::ffi::c_void;
}
#[inline]
unsafe fn FSEv07_peekSymbol(DStatePtr: *const FSEv07_DState_t) -> u8 {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    DInfo.symbol
}
#[inline]
unsafe fn FSEv07_updateState(DStatePtr: *mut FSEv07_DState_t, bitD: &mut BITv07_DStream_t) {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let lowBits = bitD.read_bits(nbBits);
    (*DStatePtr).state = (DInfo.newState as usize).wrapping_add(lowBits);
}
#[inline]
unsafe fn FSEv07_decodeSymbol(
    DStatePtr: *mut FSEv07_DState_t,
    bitD: &mut BITv07_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = bitD.read_bits(nbBits);
    (*DStatePtr).state = (DInfo.newState as usize).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe fn FSEv07_decodeSymbolFast(
    DStatePtr: *mut FSEv07_DState_t,
    bitD: &mut BITv07_DStream_t,
) -> core::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv07_decode_t).add((*DStatePtr).state);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = bitD.read_bits_fast(nbBits);
    (*DStatePtr).state = (DInfo.newState as usize).wrapping_add(lowBits);
    symbol
}
const FSEv07_MAX_MEMORY_USAGE: core::ffi::c_int = 14;
const FSEv07_MAX_SYMBOL_VALUE: core::ffi::c_int = 255;
const FSEv07_MAX_TABLELOG: core::ffi::c_int = FSEv07_MAX_MEMORY_USAGE - 2;
const FSEv07_MIN_TABLELOG: core::ffi::c_int = 5;
const FSEv07_TABLELOG_ABSOLUTE_MAX: core::ffi::c_int = 15;
const HUFv07_TABLELOG_ABSOLUTEMAX: core::ffi::c_int = 16;
const HUFv07_TABLELOG_MAX: core::ffi::c_int = 12;
const HUFv07_SYMBOLVALUE_MAX: core::ffi::c_int = 255;

unsafe fn FSEv07_readNCount(
    normalizedCounter: *mut core::ffi::c_short,
    maxSVPtr: *mut core::ffi::c_uint,
    tableLogPtr: *mut core::ffi::c_uint,
    headerBuffer: &[u8],
) -> Result<usize, Error> {
    let istart = headerBuffer.as_ptr();
    let iend = istart.add(headerBuffer.len());
    let mut ip = istart;
    let mut nbBits: core::ffi::c_int = 0;
    let mut remaining: core::ffi::c_int = 0;
    let mut threshold: core::ffi::c_int = 0;
    let mut bitStream: u32 = 0;
    let mut bitCount: core::ffi::c_int = 0;
    let mut charnum = 0;
    let mut previous0 = 0;
    if headerBuffer.len() < 4 {
        return Err(Error::srcSize_wrong);
    }
    bitStream = MEM_readLE32(ip as *const core::ffi::c_void);
    nbBits = (bitStream & 0xf as core::ffi::c_int as u32).wrapping_add(FSEv07_MIN_TABLELOG as u32)
        as core::ffi::c_int;
    if nbBits > FSEv07_TABLELOG_ABSOLUTE_MAX {
        return Err(Error::tableLog_tooLarge);
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
                let fresh0 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter.offset(fresh0 as isize) = 0;
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
            if count as core::ffi::c_int >= threshold {
                count = (count as core::ffi::c_int - max as core::ffi::c_int) as core::ffi::c_short;
            }
            bitCount += nbBits;
        }
        count -= 1;
        remaining -= count.abs() as core::ffi::c_int;
        let fresh1 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh1 as isize) = count;
        previous0 = (count == 0) as core::ffi::c_int;
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
    if ip.offset_from(istart) as usize > headerBuffer.len() {
        return Err(Error::srcSize_wrong);
    }
    Ok(ip.offset_from(istart) as usize)
}
unsafe fn HUFv07_readStats(
    huffWeight: *mut u8,
    hwSize: usize,
    rankStats: *mut u32,
    nbSymbolsPtr: *mut u32,
    tableLogPtr: *mut u32,
    src: &[u8],
) -> Result<usize, Error> {
    let mut weightTotal: u32 = 0;
    let mut ip = src;
    let mut iSize: usize = 0;
    let mut oSize: usize = 0;
    if src.is_empty() {
        return Err(Error::srcSize_wrong);
    }
    iSize = ip[0] as usize;
    if iSize >= 128 {
        if iSize >= 242 {
            static l: [u32; 14] = [1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128];
            oSize = l[iSize.wrapping_sub(242)] as usize;
            core::ptr::write_bytes(huffWeight, 1, hwSize);
            iSize = 0;
        } else {
            oSize = iSize.wrapping_sub(127);
            iSize = oSize.wrapping_add(1) / 2;
            if iSize.wrapping_add(1) > src.len() {
                return Err(Error::srcSize_wrong);
            }
            if oSize >= hwSize {
                return Err(Error::corruption_detected);
            }
            ip = &ip[1..];
            let mut n: u32 = 0;
            n = 0;
            while (n as usize) < oSize {
                *huffWeight.offset(n as isize) =
                    (ip[n as usize / 2] as core::ffi::c_int >> 4) as u8;
                *huffWeight.offset(n.wrapping_add(1) as isize) =
                    (ip[n as usize / 2] as core::ffi::c_int & 15) as u8;
                n = n.wrapping_add(2);
            }
        }
    } else {
        if iSize.wrapping_add(1) > src.len() {
            return Err(Error::srcSize_wrong);
        }
        oSize = FSEv07_decompress(
            huffWeight as *mut core::ffi::c_void,
            hwSize.wrapping_sub(1),
            &ip[1..iSize + 1],
        )?;
    }
    core::ptr::write_bytes(rankStats, 0, (HUFv07_TABLELOG_ABSOLUTEMAX + 1) as usize);
    weightTotal = 0;
    let mut n_0: u32 = 0;
    n_0 = 0;
    while (n_0 as usize) < oSize {
        if *huffWeight.offset(n_0 as isize) as core::ffi::c_int >= HUFv07_TABLELOG_ABSOLUTEMAX {
            return Err(Error::corruption_detected);
        }
        let fresh2 = &mut (*rankStats.offset(*huffWeight.offset(n_0 as isize) as isize));
        *fresh2 = (*fresh2).wrapping_add(1);
        weightTotal = weightTotal.wrapping_add(
            ((1) << *huffWeight.offset(n_0 as isize) as core::ffi::c_int >> 1) as u32,
        );
        n_0 = n_0.wrapping_add(1);
    }
    if weightTotal == 0 {
        return Err(Error::corruption_detected);
    }
    let tableLog = (BITv07_highbit32(weightTotal)).wrapping_add(1);
    if tableLog > HUFv07_TABLELOG_ABSOLUTEMAX as u32 {
        return Err(Error::corruption_detected);
    }
    *tableLogPtr = tableLog;
    let total = ((1) << tableLog) as u32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1) << BITv07_highbit32(rest)) as u32;
    let lastWeight = (BITv07_highbit32(rest)).wrapping_add(1);
    if verif != rest {
        return Err(Error::corruption_detected);
    }
    *huffWeight.add(oSize) = lastWeight as u8;
    let fresh3 = &mut (*rankStats.offset(lastWeight as isize));
    *fresh3 = (*fresh3).wrapping_add(1);
    if *rankStats.add(1) < 2 || *rankStats.add(1) & 1 != 0 {
        return Err(Error::corruption_detected);
    }
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    Ok(iSize.wrapping_add(1))
}
unsafe fn FSEv07_buildDTable(
    dt: *mut FSEv07_DTable,
    normalizedCounter: *const core::ffi::c_short,
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
) -> Result<(), Error> {
    let tdPtr = dt.add(1) as *mut core::ffi::c_void;
    let tableDecode = tdPtr as *mut FSEv07_decode_t;
    let mut symbolNext: [u16; 256] = [0; 256];
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    if maxSymbolValue > FSEv07_MAX_SYMBOL_VALUE as core::ffi::c_uint {
        return Err(Error::maxSymbolValue_tooLarge);
    }
    if tableLog > FSEv07_MAX_TABLELOG as core::ffi::c_uint {
        return Err(Error::tableLog_tooLarge);
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
    ptr::write(dt as *mut FSEv07_DTableHeader, DTableH);
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
        return Err(Error::GENERIC);
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
    Ok(())
}
unsafe fn FSEv07_buildDTable_rle(dt: *mut FSEv07_DTable, symbolValue: u8) -> usize {
    let ptr = dt as *mut core::ffi::c_void;
    let DTableH = ptr as *mut FSEv07_DTableHeader;
    let dPtr = dt.add(1) as *mut core::ffi::c_void;
    let cell = dPtr as *mut FSEv07_decode_t;
    (*DTableH).tableLog = 0;
    (*DTableH).fastMode = 0;
    (*cell).newState = 0;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0;
    0
}
#[inline(always)]
unsafe fn FSEv07_decompress_usingDTable_generic(
    dst: *mut core::ffi::c_void,
    maxDstSize: usize,
    cSrc: &[u8],
    dt: *const FSEv07_DTable,
    fast: core::ffi::c_uint,
) -> Result<usize, Error> {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.add(maxDstSize);
    let olimit = omax.sub(3);
    let mut state1 = FSEv07_DState_t {
        state: 0,
        table: core::ptr::null::<core::ffi::c_void>(),
    };
    let mut state2 = FSEv07_DState_t {
        state: 0,
        table: core::ptr::null::<core::ffi::c_void>(),
    };
    let mut bitD = BITv07_DStream_t::new(cSrc)?;
    FSEv07_initDState(&mut state1, &mut bitD, dt);
    FSEv07_initDState(&mut state2, &mut bitD, dt);
    while bitD.reload() == BITv07_DStream_unfinished && op < olimit {
        *op = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 2 + 7) as u32 > usize::BITS {
            bitD.reload();
        }
        *op.add(1) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 4 + 7) as u32 > usize::BITS
            && bitD.reload() as core::ffi::c_uint
                > BITv07_DStream_unfinished as core::ffi::c_int as core::ffi::c_uint
        {
            op = op.add(2);
            break;
        }
        *op.add(2) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if (FSEv07_MAX_TABLELOG * 2 + 7) as u32 > usize::BITS {
            bitD.reload();
        }
        *op.add(3) = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
        }) as u8;
        op = op.add(4);
    }
    loop {
        if op > omax.sub(2) {
            return Err(Error::dstSize_tooSmall);
        }
        let fresh7 = op;
        op = op.add(1);
        *fresh7 = (if fast != 0 {
            FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
        } else {
            FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
        }) as u8;
        if bitD.reload() == BITv07_DStream_overflow {
            let fresh8 = op;
            op = op.add(1);
            *fresh8 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
            } else {
                FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
            }) as u8;
            break;
        } else {
            if op > omax.sub(2) {
                return Err(Error::dstSize_tooSmall);
            }
            let fresh9 = op;
            op = op.add(1);
            *fresh9 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state2, &mut bitD) as core::ffi::c_int
            } else {
                FSEv07_decodeSymbol(&mut state2, &mut bitD) as core::ffi::c_int
            }) as u8;
            if bitD.reload() != BITv07_DStream_overflow {
                continue;
            }
            let fresh10 = op;
            op = op.add(1);
            *fresh10 = (if fast != 0 {
                FSEv07_decodeSymbolFast(&mut state1, &mut bitD) as core::ffi::c_int
            } else {
                FSEv07_decodeSymbol(&mut state1, &mut bitD) as core::ffi::c_int
            }) as u8;
            break;
        }
    }
    Ok(op.offset_from(ostart) as usize)
}
unsafe fn FSEv07_decompress_usingDTable(
    dst: *mut core::ffi::c_void,
    originalSize: usize,
    cSrc: &[u8],
    dt: *const FSEv07_DTable,
) -> Result<usize, Error> {
    let ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const FSEv07_DTableHeader;
    let fastMode = (*DTableH).fastMode as u32;
    if fastMode != 0 {
        return FSEv07_decompress_usingDTable_generic(dst, originalSize, cSrc, dt, 1);
    }
    FSEv07_decompress_usingDTable_generic(dst, originalSize, cSrc, dt, 0)
}
unsafe fn FSEv07_decompress(
    dst: *mut core::ffi::c_void,
    maxDstSize: usize,
    cSrc: &[u8],
) -> Result<usize, Error> {
    let istart = cSrc;
    let mut ip = istart;
    let mut counting: [core::ffi::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: core::ffi::c_uint = 0;
    let mut maxSymbolValue = FSEv07_MAX_SYMBOL_VALUE as core::ffi::c_uint;
    if cSrc.len() < 2 {
        return Err(Error::srcSize_wrong);
    }
    let NCountLength = FSEv07_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart,
    )?;
    if NCountLength >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    ip = &ip[NCountLength..];
    FSEv07_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    )?;
    FSEv07_decompress_usingDTable(dst, maxDstSize, ip, dt.as_mut_ptr())
}
unsafe fn HUFv07_getDTableDesc(table: *const HUFv07_DTable) -> DTableDesc {
    ptr::read::<DTableDesc>(table as *const DTableDesc)
}
unsafe fn HUFv07_readDTableX2(DTable: *mut HUFv07_DTable, src: &[u8]) -> Result<usize, Error> {
    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 17] = [0; 17];
    let mut tableLog = 0;
    let mut nbSymbols = 0;
    let mut iSize: usize = 0;
    let dtPtr = DTable.add(1) as *mut core::ffi::c_void;
    let dt = dtPtr as *mut HUFv07_DEltX2;
    iSize = HUFv07_readStats(
        huffWeight.as_mut_ptr(),
        (HUFv07_SYMBOLVALUE_MAX + 1) as usize,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
    )?;
    let mut dtd = HUFv07_getDTableDesc(DTable);
    if tableLog > (dtd.maxTableLog as core::ffi::c_int + 1) as u32 {
        return Err(Error::tableLog_tooLarge);
    }
    dtd.tableType = 0;
    dtd.tableLog = tableLog as u8;
    ptr::write(DTable as *mut DTableDesc, dtd);
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
    Ok(iSize)
}
unsafe fn HUFv07_decodeSymbolX2(
    Dstream: &mut BITv07_DStream_t,
    dt: *const HUFv07_DEltX2,
    dtLog: u32,
) -> u8 {
    let val = Dstream.look_bits_fast(dtLog);
    let c = (*dt.add(val)).byte;
    Dstream.skip_bits((*dt.add(val)).nbBits as u32);
    c
}
#[inline]
unsafe fn HUFv07_decodeStreamX2(
    mut p: *mut u8,
    bitDPtr: &mut BITv07_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv07_DEltX2,
    dtLog: u32,
) -> usize {
    let pStart = p;
    while bitDPtr.reload() == BITv07_DStream_unfinished && p <= pEnd.sub(4) {
        if MEM_64bits() {
            let fresh12 = p;
            p = p.add(1);
            *fresh12 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            let fresh13 = p;
            p = p.add(1);
            *fresh13 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh14 = p;
            p = p.add(1);
            *fresh14 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh15 = p;
        p = p.add(1);
        *fresh15 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while bitDPtr.reload() == BITv07_DStream_unfinished && p < pEnd {
        let fresh16 = p;
        p = p.add(1);
        *fresh16 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh17 = p;
        p = p.add(1);
        *fresh17 = HUFv07_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    pEnd.offset_from(pStart) as usize
}
unsafe fn HUFv07_decompress1X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
    DTable: *const HUFv07_DTable,
) -> Result<(), Error> {
    let op = dst as *mut u8;
    let oend = op.add(dstSize);
    let dtPtr = DTable.add(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX2;
    let dtd = HUFv07_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let mut bitD = BITv07_DStream_t::new(cSrc)?;
    HUFv07_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if bitD.is_empty() == 0 {
        return Err(Error::corruption_detected);
    }
    Ok(())
}
unsafe fn HUFv07_decompress1X2_DCtx(
    DCtx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
) -> Result<(), Error> {
    let mut ip = cSrc;
    let hSize = HUFv07_readDTableX2(DCtx, ip)?;
    if hSize >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    ip = &ip[hSize..];
    HUFv07_decompress1X2_usingDTable_internal(dst, dstSize, ip, DCtx)
}
unsafe fn HUFv07_decompress4X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: *const core::ffi::c_void,
    cSrcSize: usize,
    DTable: *const HUFv07_DTable,
) -> Result<usize, Error> {
    if cSrcSize < 10 {
        return Err(Error::corruption_detected);
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable.add(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX2;
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as usize;
    let length2 = MEM_readLE16(istart.add(2) as *const core::ffi::c_void) as usize;
    let length3 = MEM_readLE16(istart.add(4) as *const core::ffi::c_void) as usize;
    let length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    let istart1 = istart.add(6);
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
        return Err(Error::corruption_detected);
    }
    let mut bitD1 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart1, length1))?;
    let mut bitD2 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart2, length2))?;
    let mut bitD3 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart3, length3))?;
    let mut bitD4 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart4, length4))?;
    endSignal = bitD1.reload() as core::ffi::c_uint
        | bitD2.reload() as core::ffi::c_uint
        | bitD3.reload() as core::ffi::c_uint
        | bitD4.reload() as core::ffi::c_uint;
    while endSignal == BITv07_DStream_unfinished as core::ffi::c_int as u32 && op4 < oend.sub(7) {
        if MEM_64bits() {
            let fresh18 = op1;
            op1 = op1.add(1);
            *fresh18 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh19 = op2;
            op2 = op2.add(1);
            *fresh19 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh20 = op3;
            op3 = op3.add(1);
            *fresh20 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh21 = op4;
            op4 = op4.add(1);
            *fresh21 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            let fresh22 = op1;
            op1 = op1.add(1);
            *fresh22 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            let fresh23 = op2;
            op2 = op2.add(1);
            *fresh23 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            let fresh24 = op3;
            op3 = op3.add(1);
            *fresh24 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            let fresh25 = op4;
            op4 = op4.add(1);
            *fresh25 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh26 = op1;
            op1 = op1.add(1);
            *fresh26 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh27 = op2;
            op2 = op2.add(1);
            *fresh27 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh28 = op3;
            op3 = op3.add(1);
            *fresh28 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() {
            let fresh29 = op4;
            op4 = op4.add(1);
            *fresh29 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh30 = op1;
        op1 = op1.add(1);
        *fresh30 = HUFv07_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh31 = op2;
        op2 = op2.add(1);
        *fresh31 = HUFv07_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh32 = op3;
        op3 = op3.add(1);
        *fresh32 = HUFv07_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh33 = op4;
        op4 = op4.add(1);
        *fresh33 = HUFv07_decodeSymbolX2(&mut bitD4, dt, dtLog);
        endSignal = bitD1.reload() as core::ffi::c_uint
            | bitD2.reload() as core::ffi::c_uint
            | bitD3.reload() as core::ffi::c_uint
            | bitD4.reload() as core::ffi::c_uint;
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
    HUFv07_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv07_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv07_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv07_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = bitD1.is_empty() & bitD2.is_empty() & bitD3.is_empty() & bitD4.is_empty();
    if endSignal == 0 {
        return Err(Error::corruption_detected);
    }
    Ok(dstSize)
}
unsafe fn HUFv07_decompress4X2_DCtx(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
) -> Result<usize, Error> {
    let mut ip = cSrc;
    let hSize = HUFv07_readDTableX2(dctx, ip)?;
    if hSize >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    ip = &ip[hSize..];
    HUFv07_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        ip.as_ptr() as *const core::ffi::c_void,
        ip.len(),
        dctx,
    )
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
    let mut rankVal: [u32; 17] = ptr::read::<[u32; 17]>(rankValOrigin as *const [u32; 17]);
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
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as core::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s: u32 = 0;
    let mut rankVal: [u32; 17] = ptr::read::<[u32; 17]>(rankValOrigin);
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
unsafe fn HUFv07_readDTableX4(DTable: *mut HUFv07_DTable, src: &[u8]) -> Result<usize, Error> {
    let mut weightList: [u8; 256] = [0; 256];
    let mut sortedSymbol: [sortedSymbol_t; 256] = [sortedSymbol_t {
        symbol: 0,
        weight: 0,
    }; 256];
    let mut rankStats: [u32; 17] = [0; 17];
    let mut rankStart0: [u32; 18] = [0; 18];
    let rankStart = rankStart0.as_mut_ptr().add(1);
    let mut rankVal: rankVal_t = [[0; 17]; 16];
    let mut tableLog: u32 = 0;
    let mut maxW: u32 = 0;
    let mut sizeOfSort: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let mut dtd = HUFv07_getDTableDesc(DTable);
    let maxTableLog = dtd.maxTableLog as u32;
    let mut iSize: usize = 0;
    let dtPtr = DTable.add(1) as *mut core::ffi::c_void;
    let dt = dtPtr as *mut HUFv07_DEltX4;
    if maxTableLog > HUFv07_TABLELOG_ABSOLUTEMAX as u32 {
        return Err(Error::tableLog_tooLarge);
    }
    iSize = HUFv07_readStats(
        weightList.as_mut_ptr(),
        (HUFv07_SYMBOLVALUE_MAX + 1) as usize,
        rankStats.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
    )?;
    if tableLog > maxTableLog {
        return Err(Error::tableLog_tooLarge);
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
    *rankStart = nextRankStart;
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
    *rankStart = 0;
    let rankVal0 = (*rankVal.as_mut_ptr()).as_mut_ptr();
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
    ptr::write(DTable as *mut DTableDesc, dtd);
    Ok(iSize)
}
unsafe fn HUFv07_decodeSymbolX4(
    op: *mut core::ffi::c_void,
    DStream: &mut BITv07_DStream_t,
    dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = DStream.look_bits_fast(dtLog);
    ptr::copy_nonoverlapping(dt.add(val) as *const [u8; 2], op as *mut [u8; 2], 1);
    DStream.skip_bits((*dt.add(val)).nbBits as u32);
    (*dt.add(val)).length as u32
}
unsafe fn HUFv07_decodeLastSymbolX4(
    op: *mut core::ffi::c_void,
    DStream: &mut BITv07_DStream_t,
    dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> u32 {
    let val = DStream.look_bits_fast(dtLog);
    ptr::copy_nonoverlapping(dt.add(val) as *const u8, op as *mut u8, 1);
    if (*dt.add(val)).length as core::ffi::c_int == 1 {
        DStream.skip_bits((*dt.add(val)).nbBits as u32);
    } else if DStream.bitsConsumed < usize::BITS {
        DStream.skip_bits((*dt.add(val)).nbBits as u32);
        if DStream.bitsConsumed > usize::BITS {
            DStream.bitsConsumed = usize::BITS;
        }
    }
    1
}
#[inline]
unsafe fn HUFv07_decodeStreamX4(
    mut p: *mut u8,
    bitDPtr: &mut BITv07_DStream_t,
    pEnd: *mut u8,
    dt: *const HUFv07_DEltX4,
    dtLog: u32,
) -> usize {
    let pStart = p;
    while bitDPtr.reload() == BITv07_DStream_unfinished && p < pEnd.sub(7) {
        if MEM_64bits() {
            p = p.offset(
                HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            p = p.offset(
                HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() {
            p = p.offset(
                HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        p = p.offset(
            HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while bitDPtr.reload() == BITv07_DStream_unfinished && p <= pEnd.sub(2) {
        p = p.offset(
            HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    while p <= pEnd.sub(2) {
        p = p.offset(
            HUFv07_decodeSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    if p < pEnd {
        p = p.offset(
            HUFv07_decodeLastSymbolX4(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as usize
}
unsafe fn HUFv07_decompress1X4_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
    DTable: *const HUFv07_DTable,
) -> Result<(), Error> {
    let mut bitD = BITv07_DStream_t::new(cSrc)?;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable.add(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX4;
    let dtd = HUFv07_getDTableDesc(DTable);
    HUFv07_decodeStreamX4(ostart, &mut bitD, oend, dt, dtd.tableLog as u32);
    if bitD.is_empty() == 0 {
        return Err(Error::corruption_detected);
    }
    Ok(())
}
unsafe fn HUFv07_decompress1X4_usingDTable(
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
    DTable: *const HUFv07_DTable,
) -> Result<(), Error> {
    let dtd = HUFv07_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 1 {
        return Err(Error::GENERIC);
    }
    HUFv07_decompress1X4_usingDTable_internal(dst, dstSize, cSrc, DTable)
}
unsafe fn HUFv07_decompress4X4_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
    DTable: *const HUFv07_DTable,
) -> Result<usize, Error> {
    if cSrc.len() < 10 {
        return Err(Error::corruption_detected);
    }
    let istart = cSrc.as_ptr();
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let dtPtr = DTable.add(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUFv07_DEltX4;
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as usize;
    let length2 = MEM_readLE16(istart.add(2) as *const core::ffi::c_void) as usize;
    let length3 = MEM_readLE16(istart.add(4) as *const core::ffi::c_void) as usize;
    let length4 = cSrc.len().wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    let istart1 = istart.add(6);
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
    if length4 > cSrc.len() {
        return Err(Error::corruption_detected);
    }
    let mut bitD1 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart1, length1))?;
    let mut bitD2 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart2, length2))?;
    let mut bitD3 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart3, length3))?;
    let mut bitD4 = BITv07_DStream_t::new(core::slice::from_raw_parts(istart4, length4))?;
    endSignal = bitD1.reload() as core::ffi::c_uint
        | bitD2.reload() as core::ffi::c_uint
        | bitD3.reload() as core::ffi::c_uint
        | bitD4.reload() as core::ffi::c_uint;
    while endSignal == BITv07_DStream_unfinished as core::ffi::c_int as u32 && op4 < oend.sub(7) {
        if MEM_64bits() {
            op1 = op1.offset(HUFv07_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() {
            op2 = op2.offset(HUFv07_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() {
            op3 = op3.offset(HUFv07_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() {
            op4 = op4.offset(HUFv07_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            op1 = op1.offset(HUFv07_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            op2 = op2.offset(HUFv07_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            op3 = op3.offset(HUFv07_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() || HUFv07_TABLELOG_MAX <= 12 {
            op4 = op4.offset(HUFv07_decodeSymbolX4(
                op4 as *mut core::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() {
            op1 = op1.offset(HUFv07_decodeSymbolX4(
                op1 as *mut core::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() {
            op2 = op2.offset(HUFv07_decodeSymbolX4(
                op2 as *mut core::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() {
            op3 = op3.offset(HUFv07_decodeSymbolX4(
                op3 as *mut core::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() {
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
        endSignal = bitD1.reload() as core::ffi::c_uint
            | bitD2.reload() as core::ffi::c_uint
            | bitD3.reload() as core::ffi::c_uint
            | bitD4.reload() as core::ffi::c_uint;
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
    HUFv07_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv07_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv07_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv07_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = bitD1.is_empty() & bitD2.is_empty() & bitD3.is_empty() & bitD4.is_empty();
    if endCheck == 0 {
        return Err(Error::corruption_detected);
    }
    Ok(dstSize)
}
unsafe fn HUFv07_decompress4X4_DCtx(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
) -> Result<usize, Error> {
    let mut ip = cSrc;
    let hSize = HUFv07_readDTableX4(dctx, ip)?;
    if hSize >= cSrc.len() {
        return Err(Error::srcSize_wrong);
    }
    ip = &ip[hSize..];
    HUFv07_decompress4X4_usingDTable_internal(dst, dstSize, ip, dctx)
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
unsafe fn HUFv07_selectDecoder(dstSize: usize, cSrcSize: usize) -> u32 {
    let Q = (cSrcSize * 16 / dstSize) as u32;
    let D256 = (dstSize >> 8) as u32;
    let DTime0 = ((*(*algoTime.as_ptr().offset(Q as isize)).as_ptr()).tableTime)
        .wrapping_add((*(*algoTime.as_ptr().offset(Q as isize)).as_ptr()).decode256Time * D256);
    let mut DTime1 = ((*(*algoTime.as_ptr().offset(Q as isize)).as_ptr().add(1)).tableTime)
        .wrapping_add(
            (*(*algoTime.as_ptr().offset(Q as isize)).as_ptr().add(1)).decode256Time * D256,
        );
    DTime1 = DTime1.wrapping_add(DTime1 >> 3);
    (DTime1 < DTime0) as core::ffi::c_int as u32
}
unsafe fn HUFv07_decompress4X_hufOnly(
    dctx: *mut HUFv07_DTable,
    dst: *mut core::ffi::c_void,
    dstSize: usize,
    cSrc: &[u8],
) -> Result<usize, Error> {
    if dstSize == 0 {
        return Err(Error::dstSize_tooSmall);
    }
    if cSrc.len() >= dstSize || cSrc.len() <= 1 {
        return Err(Error::corruption_detected);
    }
    let algoNb = HUFv07_selectDecoder(dstSize, cSrc.len());
    if algoNb != 0 {
        HUFv07_decompress4X4_DCtx(dctx, dst, dstSize, cSrc)
    } else {
        HUFv07_decompress4X2_DCtx(dctx, dst, dstSize, cSrc)
    }
}
unsafe fn ZSTDv07_defaultAllocFunction(
    _opaque: *mut core::ffi::c_void,
    size: usize,
) -> *mut core::ffi::c_void {
    malloc(size)
}
unsafe fn ZSTDv07_defaultFreeFunction(
    _opaque: *mut core::ffi::c_void,
    address: *mut core::ffi::c_void,
) {
    free(address);
}
const ZSTDv07_DICT_MAGIC: core::ffi::c_uint = 0xec30a437 as core::ffi::c_uint;
const ZSTDv07_REP_NUM: core::ffi::c_int = 3;
const ZSTDv07_REP_INIT: core::ffi::c_int = 3;
static repStartValue: [u32; 3] = [1, 4, 8];
const ZSTDv07_WINDOWLOG_ABSOLUTEMIN: core::ffi::c_int = 10;
static ZSTDv07_fcs_fieldSize: [usize; 4] = [0, 2, 4, 8];
static ZSTDv07_did_fieldSize: [usize; 4] = [0, 1, 2, 4];
const ZSTDv07_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTDv07_blockHeaderSize: usize = ZSTDv07_BLOCKHEADERSIZE as usize;
const MIN_SEQUENCES_SIZE: core::ffi::c_int = 1;
const MIN_CBLOCK_SIZE: core::ffi::c_int = 1 + 1 + MIN_SEQUENCES_SIZE;
const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
const MINMATCH: core::ffi::c_int = 3;
const MaxML: core::ffi::c_int = 52;
const MaxLL: core::ffi::c_int = 35;
const MaxOff: core::ffi::c_int = 28;
const MLFSELog: core::ffi::c_int = 9;
const LLFSELog: core::ffi::c_int = 9;
const OffFSELog: core::ffi::c_int = 8;
const ZSTD_CONTENTSIZE_ERROR: core::ffi::c_ulonglong =
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
const WILDCOPY_OVERLENGTH: core::ffi::c_int = 8;
#[inline]
unsafe fn ZSTDv07_wildcopy(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    length: isize,
) {
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length);
    loop {
        core::ptr::copy_nonoverlapping(ip, op, 8);
        op = op.add(8);
        ip = ip.add(8);
        if op >= oend {
            break;
        }
    }
}
static mut defaultCustomMem: ZSTDv07_customMem = ZSTDv07_customMem {
    customAlloc: Some(
        ZSTDv07_defaultAllocFunction
            as unsafe fn(*mut core::ffi::c_void, usize) -> *mut core::ffi::c_void,
    ),
    customFree: Some(
        ZSTDv07_defaultFreeFunction
            as unsafe fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> (),
    ),
    opaque: core::ptr::null_mut(),
};
unsafe fn ZSTDv07_decompressBegin(dctx: *mut ZSTDv07_DCtx) {
    (*dctx).expected = ZSTDv07_frameHeaderSize_min;
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).previousDstEnd = core::ptr::null();
    (*dctx).base = core::ptr::null();
    (*dctx).vBase = core::ptr::null();
    (*dctx).dictEnd = core::ptr::null();
    *((*dctx).hufTable).as_mut_ptr() = (12 * 0x1000001 as core::ffi::c_int) as HUFv07_DTable;
    (*dctx).fseEntropy = 0;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    (*dctx).dictID = 0;
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < ZSTDv07_REP_NUM {
        *((*dctx).rep).as_mut_ptr().offset(i as isize) = *repStartValue.as_ptr().offset(i as isize);
        i += 1;
    }
}
unsafe fn ZSTDv07_createDCtx_advanced(mut customMem: ZSTDv07_customMem) -> *mut ZSTDv07_DCtx {
    let mut dctx = core::ptr::null_mut::<ZSTDv07_DCtx>();
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return core::ptr::null_mut();
    }
    dctx = (customMem.customAlloc).unwrap_unchecked()(
        customMem.opaque,
        ::core::mem::size_of::<ZSTDv07_DCtx>(),
    ) as *mut ZSTDv07_DCtx;
    if dctx.is_null() {
        return core::ptr::null_mut();
    }
    ptr::copy_nonoverlapping(&customMem, &mut (*dctx).customMem, 1);
    ZSTDv07_decompressBegin(dctx);
    dctx
}
pub(crate) unsafe fn ZSTDv07_createDCtx() -> *mut ZSTDv07_DCtx {
    ZSTDv07_createDCtx_advanced(defaultCustomMem)
}
pub(crate) unsafe fn ZSTDv07_freeDCtx(dctx: *mut ZSTDv07_DCtx) -> usize {
    if dctx.is_null() {
        return 0;
    }
    ((*dctx).customMem.customFree).unwrap_unchecked()(
        (*dctx).customMem.opaque,
        dctx as *mut core::ffi::c_void,
    );
    0
}
unsafe fn ZSTDv07_frameHeaderSize(
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    if srcSize < ZSTDv07_frameHeaderSize_min {
        return Err(Error::srcSize_wrong);
    }
    let fhd = *(src as *const u8).add(4);
    let dictID = (fhd as core::ffi::c_int & 3) as u32;
    let directMode = (fhd as core::ffi::c_int >> 5 & 1) as u32;
    let fcsId = (fhd as core::ffi::c_int >> 6) as u32;
    Ok(ZSTDv07_frameHeaderSize_min
        .wrapping_add((directMode == 0) as core::ffi::c_int as usize)
        .wrapping_add(*ZSTDv07_did_fieldSize.as_ptr().offset(dictID as isize))
        .wrapping_add(*ZSTDv07_fcs_fieldSize.as_ptr().offset(fcsId as isize))
        .wrapping_add(
            (directMode != 0 && *ZSTDv07_fcs_fieldSize.as_ptr().offset(fcsId as isize) == 0)
                as core::ffi::c_int as usize,
        ))
}
pub(crate) unsafe fn ZSTDv07_getFrameParams(
    fparamsPtr: *mut ZSTDv07_frameParams,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    let ip = src as *const u8;
    if srcSize < ZSTDv07_frameHeaderSize_min {
        return Ok(ZSTDv07_frameHeaderSize_min);
    }
    ptr::write_bytes(
        fparamsPtr as *mut u8,
        0,
        ::core::mem::size_of::<ZSTDv07_frameParams>(),
    );
    if MEM_readLE32(src) != ZSTDv07_MAGICNUMBER {
        if MEM_readLE32(src) & 0xfffffff0 as core::ffi::c_uint == ZSTDv07_MAGIC_SKIPPABLE_START {
            if srcSize < ZSTDv07_skippableHeaderSize {
                return Ok(ZSTDv07_skippableHeaderSize);
            }
            (*fparamsPtr).frameContentSize =
                MEM_readLE32((src as *const core::ffi::c_char).add(4) as *const core::ffi::c_void)
                    as core::ffi::c_ulonglong;
            (*fparamsPtr).windowSize = 0;
            return Ok(0);
        }
        return Err(Error::prefix_unknown);
    }
    let fhsize = ZSTDv07_frameHeaderSize(src, srcSize)?;
    if srcSize < fhsize {
        return Ok(fhsize);
    }
    let fhdByte = *ip.add(4);
    let mut pos = 5 as usize;
    let dictIDSizeCode = (fhdByte as core::ffi::c_int & 3) as u32;
    let checksumFlag = (fhdByte as core::ffi::c_int >> 2 & 1) as u32;
    let directMode = (fhdByte as core::ffi::c_int >> 5 & 1) as u32;
    let fcsID = (fhdByte as core::ffi::c_int >> 6) as u32;
    let windowSizeMax = (1)
        << (if MEM_32bits() {
            ZSTDv07_WINDOWLOG_MAX_32
        } else {
            ZSTDv07_WINDOWLOG_MAX_64
        }) as u32;
    let mut windowSize = 0u32;
    let mut dictID = 0;
    let mut frameContentSize = 0;
    if fhdByte as core::ffi::c_int & 0x8 as core::ffi::c_int != 0 {
        return Err(Error::frameParameter_unsupported);
    }
    if directMode == 0 {
        let fresh39 = pos;
        pos = pos.wrapping_add(1);
        let wlByte = *ip.add(fresh39);
        let windowLog = ((wlByte as core::ffi::c_int >> 3) + ZSTDv07_WINDOWLOG_ABSOLUTEMIN) as u32;
        if windowLog
            > (if MEM_32bits() {
                ZSTDv07_WINDOWLOG_MAX_32
            } else {
                ZSTDv07_WINDOWLOG_MAX_64
            }) as u32
        {
            return Err(Error::frameParameter_unsupported);
        }
        windowSize = (1) << windowLog;
        windowSize =
            windowSize.wrapping_add((windowSize >> 3) * (wlByte as core::ffi::c_int & 7) as u32);
    }
    match dictIDSizeCode {
        0 => {}
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
        _ => unreachable!(),
    }
    match fcsID {
        0 => {
            if directMode != 0 {
                frameContentSize = *ip.add(pos) as u64;
            }
        }
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
        _ => unreachable!(),
    }
    if windowSize == 0 {
        windowSize = frameContentSize as u32;
    }
    if windowSize > windowSizeMax {
        return Err(Error::frameParameter_unsupported);
    }
    (*fparamsPtr).frameContentSize = frameContentSize as core::ffi::c_ulonglong;
    (*fparamsPtr).windowSize = windowSize;
    (*fparamsPtr).dictID = dictID;
    (*fparamsPtr).checksumFlag = checksumFlag;
    Ok(0)
}
unsafe fn ZSTDv07_decodeFrameHeader(
    dctx: *mut ZSTDv07_DCtx,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    let result = ZSTDv07_getFrameParams(&mut (*dctx).fParams, src, srcSize);
    if (*dctx).fParams.dictID != 0 && (*dctx).dictID != (*dctx).fParams.dictID {
        return Err(Error::dictionary_wrong);
    }
    if (*dctx).fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(&mut (*dctx).xxhState, 0);
    }
    result
}
unsafe fn ZSTDv07_getcBlockSize(
    src: *const core::ffi::c_void,
    srcSize: usize,
    bpPtr: *mut blockProperties_t,
) -> Result<usize, Error> {
    let in_0 = src as *const u8;
    let mut cSize: u32 = 0;
    if srcSize < ZSTDv07_blockHeaderSize {
        return Err(Error::srcSize_wrong);
    }
    (*bpPtr).blockType = (*in_0 as core::ffi::c_int >> 6) as blockType_t;
    cSize = (*in_0.add(2) as core::ffi::c_int
        + ((*in_0.add(1) as core::ffi::c_int) << 8)
        + ((*in_0 as core::ffi::c_int & 7) << 16)) as u32;
    (*bpPtr).origSize = if (*bpPtr).blockType as core::ffi::c_uint
        == bt_rle as core::ffi::c_int as core::ffi::c_uint
    {
        cSize
    } else {
        0
    };
    if (*bpPtr).blockType as core::ffi::c_uint == bt_end as core::ffi::c_int as core::ffi::c_uint {
        return Ok(0);
    }
    if (*bpPtr).blockType as core::ffi::c_uint == bt_rle as core::ffi::c_int as core::ffi::c_uint {
        return Ok(1);
    }
    Ok(cSize as usize)
}
unsafe fn ZSTDv07_copyRawBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    if srcSize > dstCapacity {
        return Err(Error::dstSize_tooSmall);
    }
    if srcSize > 0 {
        ptr::copy_nonoverlapping(src, dst, srcSize);
    }
    Ok(srcSize)
}
unsafe fn ZSTDv07_decodeLiteralsBlock(
    dctx: *mut ZSTDv07_DCtx,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    let istart = src as *const u8;
    if srcSize < MIN_CBLOCK_SIZE as usize {
        return Err(Error::corruption_detected);
    }
    match (*istart as core::ffi::c_int >> 6) as litBlockType_t as core::ffi::c_uint {
        0 => {
            let mut litSize: usize = 0;
            let mut litCSize: usize = 0;
            let mut singleStream = 0;
            let mut lhSize = (*istart as core::ffi::c_int >> 4 & 3) as u32;
            if srcSize < 5 {
                return Err(Error::corruption_detected);
            }
            match lhSize {
                2 => {
                    lhSize = 4;
                    litSize = (((*istart as core::ffi::c_int & 15) << 10)
                        + ((*istart.add(1) as core::ffi::c_int) << 2)
                        + (*istart.add(2) as core::ffi::c_int >> 6))
                        as usize;
                    litCSize = (((*istart.add(2) as core::ffi::c_int & 63) << 8)
                        + *istart.add(3) as core::ffi::c_int)
                        as usize;
                }
                3 => {
                    lhSize = 5;
                    litSize = (((*istart as core::ffi::c_int & 15) << 14)
                        + ((*istart.add(1) as core::ffi::c_int) << 6)
                        + (*istart.add(2) as core::ffi::c_int >> 2))
                        as usize;
                    litCSize = (((*istart.add(2) as core::ffi::c_int & 3) << 16)
                        + ((*istart.add(3) as core::ffi::c_int) << 8)
                        + *istart.add(4) as core::ffi::c_int)
                        as usize;
                }
                0 | 1 => {
                    lhSize = 3;
                    singleStream = (*istart as core::ffi::c_int & 16) as usize;
                    litSize = (((*istart as core::ffi::c_int & 15) << 6)
                        + (*istart.add(1) as core::ffi::c_int >> 2))
                        as usize;
                    litCSize = (((*istart.add(1) as core::ffi::c_int & 3) << 8)
                        + *istart.add(2) as core::ffi::c_int)
                        as usize;
                }
                _ => unreachable!(),
            }
            if litSize > ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as usize {
                return Err(Error::corruption_detected);
            }
            if litCSize.wrapping_add(lhSize as usize) > srcSize {
                return Err(Error::corruption_detected);
            }
            if singleStream != 0 {
                HUFv07_decompress1X2_DCtx(
                    ((*dctx).hufTable).as_mut_ptr(),
                    ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    litSize,
                    core::slice::from_raw_parts(istart.offset(lhSize as isize), litCSize),
                )
                .map_err(|_| Error::corruption_detected)?;
            } else {
                HUFv07_decompress4X_hufOnly(
                    ((*dctx).hufTable).as_mut_ptr(),
                    ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                    litSize,
                    core::slice::from_raw_parts(istart.offset(lhSize as isize), litCSize),
                )
                .map_err(|_| Error::corruption_detected)?;
            }
            (*dctx).litPtr = (&raw mut (*dctx).litBuffer).cast();
            (*dctx).litSize = litSize;
            (*dctx).litEntropy = 1;
            ptr::write_bytes(
                ((*dctx).litBuffer).as_mut_ptr().add((*dctx).litSize),
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            Ok(litCSize.wrapping_add(lhSize as usize))
        }
        1 => {
            let mut litSize_0: usize = 0;
            let mut litCSize_0: usize = 0;
            let mut lhSize_0 = (*istart as core::ffi::c_int >> 4 & 3) as u32;
            if lhSize_0 != 1 {
                return Err(Error::corruption_detected);
            }
            if (*dctx).litEntropy == 0 {
                return Err(Error::dictionary_corrupted);
            }
            lhSize_0 = 3;
            litSize_0 = (((*istart as core::ffi::c_int & 15) << 6)
                + (*istart.add(1) as core::ffi::c_int >> 2)) as usize;
            litCSize_0 = (((*istart.add(1) as core::ffi::c_int & 3) << 8)
                + *istart.add(2) as core::ffi::c_int) as usize;
            if litCSize_0.wrapping_add(lhSize_0 as usize) > srcSize {
                return Err(Error::corruption_detected);
            }
            HUFv07_decompress1X4_usingDTable(
                ((*dctx).litBuffer).as_mut_ptr() as *mut core::ffi::c_void,
                litSize_0,
                core::slice::from_raw_parts(istart.offset(lhSize_0 as isize), litCSize_0),
                ((*dctx).hufTable).as_mut_ptr(),
            )
            .map_err(|_| Error::corruption_detected)?;
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_0;
            ptr::write_bytes(
                ((*dctx).litBuffer).as_mut_ptr().add((*dctx).litSize),
                0,
                WILDCOPY_OVERLENGTH as usize,
            );
            Ok(litCSize_0.wrapping_add(lhSize_0 as usize))
        }
        2 => {
            let mut litSize_1: usize = 0;
            let mut lhSize_1 = (*istart as core::ffi::c_int >> 4 & 3) as u32;
            match lhSize_1 {
                2 => {
                    litSize_1 = (((*istart as core::ffi::c_int & 15) << 8)
                        + *istart.add(1) as core::ffi::c_int)
                        as usize;
                }
                3 => {
                    litSize_1 = (((*istart as core::ffi::c_int & 15) << 16)
                        + ((*istart.add(1) as core::ffi::c_int) << 8)
                        + *istart.add(2) as core::ffi::c_int)
                        as usize;
                }
                0 | 1 => {
                    lhSize_1 = 1;
                    litSize_1 = (*istart as core::ffi::c_int & 31) as usize;
                }
                _ => unreachable!(),
            }
            if (lhSize_1 as usize)
                .wrapping_add(litSize_1)
                .wrapping_add(WILDCOPY_OVERLENGTH as usize)
                > srcSize
            {
                if litSize_1.wrapping_add(lhSize_1 as usize) > srcSize {
                    return Err(Error::corruption_detected);
                }
                ptr::copy_nonoverlapping(
                    istart.offset(lhSize_1 as isize),
                    (*dctx).litBuffer.as_mut_ptr(),
                    litSize_1,
                );
                (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
                (*dctx).litSize = litSize_1;
                ptr::write_bytes(
                    ((*dctx).litBuffer).as_mut_ptr().add((*dctx).litSize),
                    0,
                    WILDCOPY_OVERLENGTH as usize,
                );
                return Ok((lhSize_1 as usize).wrapping_add(litSize_1));
            }
            (*dctx).litPtr = istart.offset(lhSize_1 as isize);
            (*dctx).litSize = litSize_1;
            Ok((lhSize_1 as usize).wrapping_add(litSize_1))
        }
        3 => {
            let mut litSize_2: usize = 0;
            let mut lhSize_2 = (*istart as core::ffi::c_int >> 4 & 3) as u32;
            match lhSize_2 {
                2 => {
                    litSize_2 = (((*istart as core::ffi::c_int & 15) << 8)
                        + *istart.add(1) as core::ffi::c_int)
                        as usize;
                }
                3 => {
                    litSize_2 = (((*istart as core::ffi::c_int & 15) << 16)
                        + ((*istart.add(1) as core::ffi::c_int) << 8)
                        + *istart.add(2) as core::ffi::c_int)
                        as usize;
                    if srcSize < 4 {
                        return Err(Error::corruption_detected);
                    }
                }
                0 | 1 => {
                    lhSize_2 = 1;
                    litSize_2 = (*istart as core::ffi::c_int & 31) as usize;
                }
                _ => unreachable!(),
            }
            if litSize_2 > ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as usize {
                return Err(Error::corruption_detected);
            }
            core::ptr::write_bytes(
                ((*dctx).litBuffer).as_mut_ptr(),
                *istart.offset(lhSize_2 as isize),
                litSize_2.wrapping_add(WILDCOPY_OVERLENGTH as usize),
            );
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_2;
            Ok(lhSize_2.wrapping_add(1) as usize)
        }
        _ => Err(Error::corruption_detected),
    }
}
unsafe fn ZSTDv07_buildSeqTable(
    DTable: *mut FSEv07_DTable,
    type_0: u32,
    mut max: u32,
    maxLog: u32,
    src: &[u8],
    defaultNorm: *const i16,
    defaultLog: u32,
    flagRepeatTable: u32,
) -> Result<usize, Error> {
    match type_0 {
        1 => {
            if src.is_empty() {
                return Err(Error::srcSize_wrong);
            }
            if src[0] as u32 > max {
                return Err(Error::corruption_detected);
            }
            FSEv07_buildDTable_rle(DTable, src[0]);
            Ok(1)
        }
        0 => {
            let _ = FSEv07_buildDTable(DTable, defaultNorm, max, defaultLog);
            Ok(0)
        }
        2 => {
            if flagRepeatTable == 0 {
                return Err(Error::corruption_detected);
            }
            Ok(0)
        }
        3 => {
            let mut tableLog: u32 = 0;
            let mut norm: [i16; 53] = [0; 53];
            let headerSize = FSEv07_readNCount(norm.as_mut_ptr(), &mut max, &mut tableLog, src)
                .map_err(|_| Error::corruption_detected)?;

            if tableLog > maxLog {
                return Err(Error::corruption_detected);
            }
            let _ = FSEv07_buildDTable(DTable, norm.as_mut_ptr(), max, tableLog);
            Ok(headerSize)
        }
        _ => unreachable!(),
    }
}
unsafe fn ZSTDv07_decodeSeqHeaders(
    nbSeqPtr: *mut core::ffi::c_int,
    DTableLL: *mut FSEv07_DTable,
    DTableML: *mut FSEv07_DTable,
    DTableOffb: *mut FSEv07_DTable,
    flagRepeatTable: u32,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let mut ip = istart;
    if srcSize < MIN_SEQUENCES_SIZE as usize {
        return Err(Error::srcSize_wrong);
    }
    let fresh40 = ip;
    ip = ip.add(1);
    let mut nbSeq = *fresh40 as core::ffi::c_int;
    if nbSeq == 0 {
        *nbSeqPtr = 0;
        return Ok(1);
    }
    if nbSeq > 0x7f as core::ffi::c_int {
        if nbSeq == 0xff as core::ffi::c_int {
            if ip.add(2) > iend {
                return Err(Error::srcSize_wrong);
            }
            nbSeq = MEM_readLE16(ip as *const core::ffi::c_void) as core::ffi::c_int + LONGNBSEQ;
            ip = ip.add(2);
        } else {
            if ip >= iend {
                return Err(Error::srcSize_wrong);
            }
            let fresh41 = ip;
            ip = ip.add(1);
            nbSeq = ((nbSeq - 0x80 as core::ffi::c_int) << 8) + *fresh41 as core::ffi::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.add(4) > iend {
        return Err(Error::srcSize_wrong);
    }
    let LLtype = (*ip as core::ffi::c_int >> 6) as u32;
    let OFtype = (*ip as core::ffi::c_int >> 4 & 3) as u32;
    let MLtype = (*ip as core::ffi::c_int >> 2 & 3) as u32;
    ip = ip.add(1);
    let llhSize = ZSTDv07_buildSeqTable(
        DTableLL,
        LLtype,
        MaxLL as u32,
        LLFSELog as u32,
        core::slice::from_raw_parts(ip, iend.offset_from(ip) as usize),
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        flagRepeatTable,
    )
    .map_err(|_| Error::corruption_detected)?;
    ip = ip.add(llhSize);
    let ofhSize = ZSTDv07_buildSeqTable(
        DTableOffb,
        OFtype,
        MaxOff as u32,
        OffFSELog as u32,
        core::slice::from_raw_parts(ip, iend.offset_from(ip) as usize),
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        flagRepeatTable,
    )
    .map_err(|_| Error::corruption_detected)?;
    ip = ip.add(ofhSize);
    let mlhSize = ZSTDv07_buildSeqTable(
        DTableML,
        MLtype,
        MaxML as u32,
        MLFSELog as u32,
        core::slice::from_raw_parts(ip, iend.offset_from(ip) as usize),
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        flagRepeatTable,
    )
    .map_err(|_| Error::corruption_detected)?;
    ip = ip.add(mlhSize);
    Ok(ip.offset_from(istart) as usize)
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
    let mut offset: usize = 0;
    if ofCode == 0 {
        offset = 0;
    } else {
        offset = (*OF_base.as_ptr().offset(ofCode as isize) as usize)
            .wrapping_add((*seqState).DStream.read_bits(ofBits));
        if MEM_32bits() {
            (*seqState).DStream.reload();
        }
    }
    if ofCode <= 1 {
        if (llCode == 0) as core::ffi::c_int & (offset <= 1) as core::ffi::c_int != 0 {
            offset = (1 as usize).wrapping_sub(offset);
        }
        if offset != 0 {
            let temp = *((*seqState).prevOffset).as_mut_ptr().add(offset);
            if offset != 1 {
                *((*seqState).prevOffset).as_mut_ptr().add(2) =
                    *((*seqState).prevOffset).as_mut_ptr().add(1);
            }
            *((*seqState).prevOffset).as_mut_ptr().add(1) = *((*seqState).prevOffset).as_mut_ptr();
            offset = temp;
            *((*seqState).prevOffset).as_mut_ptr() = offset;
        } else {
            offset = *((*seqState).prevOffset).as_mut_ptr();
        }
    } else {
        *((*seqState).prevOffset).as_mut_ptr().add(2) =
            *((*seqState).prevOffset).as_mut_ptr().add(1);
        *((*seqState).prevOffset).as_mut_ptr().add(1) = *((*seqState).prevOffset).as_mut_ptr();
        *((*seqState).prevOffset).as_mut_ptr() = offset;
    }
    seq.offset = offset;
    seq.matchLength =
        (*ML_base.as_ptr().offset(mlCode as isize) as usize).wrapping_add(if mlCode > 31 {
            (*seqState).DStream.read_bits(mlBits)
        } else {
            0
        });
    if MEM_32bits() && mlBits.wrapping_add(llBits) > 24 {
        (*seqState).DStream.reload();
    }
    seq.litLength =
        (*LL_base.as_ptr().offset(llCode as isize) as usize).wrapping_add(if llCode > 15 {
            (*seqState).DStream.read_bits(llBits)
        } else {
            0
        });
    if MEM_32bits() || totalBits > (64 - 7 - (LLFSELog + MLFSELog + OffFSELog)) as u32 {
        (*seqState).DStream.reload();
    }
    FSEv07_updateState(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    FSEv07_updateState(&mut (*seqState).stateML, &mut (*seqState).DStream);
    if MEM_32bits() {
        (*seqState).DStream.reload();
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
) -> Result<usize, Error> {
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.add(sequenceLength);
    let oend_w = oend.wrapping_sub(WILDCOPY_OVERLENGTH as usize);
    let iLitEnd = (*litPtr).add(sequence.litLength);
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset);
    if (sequence.litLength).wrapping_add(WILDCOPY_OVERLENGTH as usize)
        > oend.offset_from(op) as usize
    {
        return Err(Error::dstSize_tooSmall);
    }
    if sequenceLength > oend.offset_from(op) as usize {
        return Err(Error::dstSize_tooSmall);
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as usize {
        return Err(Error::corruption_detected);
    }
    ZSTDv07_wildcopy(
        op as *mut core::ffi::c_void,
        *litPtr as *const core::ffi::c_void,
        sequence.litLength as isize,
    );
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(base) as usize {
        if sequence.offset > oLitEnd.offset_from(vBase) as usize {
            return Err(Error::corruption_detected);
        }
        match_0 = dictEnd.offset(-(base.offset_from(match_0)));
        if match_0.add(sequence.matchLength) <= dictEnd {
            core::ptr::copy(match_0, oLitEnd, sequence.matchLength);
            return Ok(sequenceLength);
        }
        let length1 = dictEnd.offset_from(match_0) as usize;
        core::ptr::copy(match_0, oLitEnd, length1);
        op = oLitEnd.add(length1);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = base;
        if op > oend_w || sequence.matchLength < MINMATCH as usize {
            while op < oMatchEnd {
                let fresh42 = match_0;
                match_0 = match_0.add(1);
                let fresh43 = op;
                op = op.add(1);
                *fresh43 = *fresh42;
            }
            return Ok(sequenceLength);
        }
    }
    if sequence.offset < 8 {
        static dec32table: [u32; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
        static dec64table: [core::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
        let sub2 = *dec64table.as_ptr().add(sequence.offset);
        *op = *match_0;
        *op.add(1) = *match_0.add(1);
        *op.add(2) = *match_0.add(2);
        *op.add(3) = *match_0.add(3);
        match_0 = match_0.offset(*dec32table.as_ptr().add(sequence.offset) as isize);
        ptr::copy_nonoverlapping(match_0, op.add(4), 4);
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ptr::copy_nonoverlapping(match_0, op, 8);
    }
    op = op.add(8);
    match_0 = match_0.add(8);
    if oMatchEnd > oend.offset(-((16 - MINMATCH) as isize)) {
        if op < oend_w {
            ZSTDv07_wildcopy(
                op as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                oend_w.offset_from(op) as isize,
            );
            match_0 = match_0.offset(oend_w.offset_from(op) as core::ffi::c_long as isize);
            op = oend_w;
        }
        while op < oMatchEnd {
            let fresh44 = match_0;
            match_0 = match_0.add(1);
            let fresh45 = op;
            op = op.add(1);
            *fresh45 = *fresh44;
        }
    } else {
        ZSTDv07_wildcopy(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            sequence.matchLength as isize - 8,
        );
    }
    Ok(sequenceLength)
}
unsafe fn ZSTDv07_decompressSequences(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: usize,
    seqStart: *const core::ffi::c_void,
    seqSize: usize,
) -> Result<usize, Error> {
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
    )?;
    ip = ip.add(seqHSize);
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BITv07_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: core::ptr::null(),
                start: core::ptr::null(),
                _marker: PhantomData,
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
                *((*dctx).rep).as_mut_ptr().offset(i as isize) as usize;
            i = i.wrapping_add(1);
        }
        match BITv07_DStream_t::new(core::slice::from_raw_parts(
            ip,
            iend.offset_from(ip) as usize,
        )) {
            Ok(bitD) => seqState.DStream = bitD,
            Err(_) => return Err(Error::corruption_detected),
        };
        FSEv07_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
        FSEv07_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
        FSEv07_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
        while seqState.DStream.reload() as core::ffi::c_uint
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
            )?;
            op = op.add(oneSeqSize);
        }
        if nbSeq != 0 {
            return Err(Error::corruption_detected);
        }
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTDv07_REP_INIT as u32 {
            *((*dctx).rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as usize;
    if lastLLSize > oend.offset_from(op) as usize {
        return Err(Error::dstSize_tooSmall);
    }
    if lastLLSize > 0 {
        ptr::copy_nonoverlapping(litPtr, op, lastLLSize);
        op = op.add(lastLLSize);
    }
    Ok(op.offset_from(ostart) as usize)
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
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    mut srcSize: usize,
) -> Result<usize, Error> {
    let mut ip = src as *const u8;
    if srcSize >= ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as usize {
        return Err(Error::srcSize_wrong);
    }
    let litCSize = ZSTDv07_decodeLiteralsBlock(dctx, src, srcSize)?;
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
unsafe fn ZSTDv07_generateNxBytes(
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    byte: u8,
    length: usize,
) -> Result<usize, Error> {
    if length > dstCapacity {
        return Err(Error::dstSize_tooSmall);
    }
    if length > 0 {
        core::ptr::write_bytes(dst.cast::<u8>(), byte, length);
    }
    Ok(length)
}
unsafe fn ZSTDv07_decompressFrame(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstCapacity);
    let mut op = ostart;
    let mut remainingSize = srcSize;
    if srcSize < ZSTDv07_frameHeaderSize_min.wrapping_add(ZSTDv07_blockHeaderSize) {
        return Err(Error::srcSize_wrong);
    }
    let frameHeaderSize = ZSTDv07_frameHeaderSize(src, ZSTDv07_frameHeaderSize_min)?;
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv07_blockHeaderSize) {
        return Err(Error::srcSize_wrong);
    }
    if ZSTDv07_decodeFrameHeader(dctx, src, frameHeaderSize) != Ok(0) {
        return Err(Error::corruption_detected);
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
            iend.offset_from(ip) as usize,
            &mut blockProperties,
        )?;
        ip = ip.add(ZSTDv07_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv07_blockHeaderSize);
        if cBlockSize > remainingSize {
            return Err(Error::srcSize_wrong);
        }
        let decodedSize = match blockProperties.blockType as core::ffi::c_uint {
            bt_compressed => ZSTDv07_decompressBlock_internal(
                dctx,
                op as *mut core::ffi::c_void,
                oend.offset_from(op) as usize,
                ip as *const core::ffi::c_void,
                cBlockSize,
            )?,
            bt_raw => ZSTDv07_copyRawBlock(
                op as *mut core::ffi::c_void,
                oend.offset_from(op) as usize,
                ip as *const core::ffi::c_void,
                cBlockSize,
            )?,
            bt_rle => ZSTDv07_generateNxBytes(
                op as *mut core::ffi::c_void,
                oend.offset_from(op) as usize,
                *ip,
                blockProperties.origSize as usize,
            )?,
            bt_end => {
                if remainingSize != 0 {
                    return Err(Error::srcSize_wrong);
                }
                break;
            }
            _ => return Err(Error::GENERIC),
        };
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
    Ok(op.offset_from(ostart) as usize)
}
pub(crate) unsafe fn ZSTDv07_decompress_usingDict(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    srcSize: usize,
    dict: *const core::ffi::c_void,
    dictSize: usize,
) -> Result<usize, Error> {
    let _ = ZSTDv07_decompressBegin_usingDict(dctx, dict, dictSize);
    ZSTDv07_checkContinuity(dctx, dst);
    ZSTDv07_decompressFrame(dctx, dst, dstCapacity, src, srcSize)
}
unsafe fn ZSTD_errorFrameSizeInfoLegacy(
    cSize: *mut usize,
    dBound: *mut core::ffi::c_ulonglong,
    ret: Error,
) {
    *cSize = ret.to_error_code();
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
pub(crate) unsafe fn ZSTDv07_findFrameSizeInfoLegacy(
    src: *const core::ffi::c_void,
    srcSize: usize,
    cSize: *mut usize,
    dBound: *mut core::ffi::c_ulonglong,
) {
    let mut ip = src as *const u8;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as usize;
    if srcSize < ZSTDv07_frameHeaderSize_min.wrapping_add(ZSTDv07_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::srcSize_wrong);
        return;
    }
    let frameHeaderSize = match ZSTDv07_frameHeaderSize(src, srcSize) {
        Ok(frameHeaderSize) => frameHeaderSize,
        Err(err) => {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, err);
            return;
        }
    };
    if MEM_readLE32(src) != ZSTDv07_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::prefix_unknown);
        return;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv07_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::srcSize_wrong);
        return;
    }
    ip = ip.add(frameHeaderSize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    loop {
        let mut blockProperties = blockProperties_t {
            blockType: bt_compressed,
            origSize: 0,
        };
        let cBlockSize = match ZSTDv07_getcBlockSize(
            ip as *const core::ffi::c_void,
            remainingSize,
            &mut blockProperties,
        ) {
            Ok(cBlockSize) => cBlockSize,
            Err(err) => {
                ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, err);
                return;
            }
        };
        ip = ip.add(ZSTDv07_blockHeaderSize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv07_blockHeaderSize);
        if blockProperties.blockType as core::ffi::c_uint
            == bt_end as core::ffi::c_int as core::ffi::c_uint
        {
            break;
        }
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, Error::srcSize_wrong);
            return;
        }
        ip = ip.add(cBlockSize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
        nbBlocks = nbBlocks.wrapping_add(1);
    }
    *cSize = ip.offset_from(src as *const u8) as usize;
    *dBound = (nbBlocks * ZSTDv07_BLOCKSIZE_ABSOLUTEMAX as usize) as core::ffi::c_ulonglong;
}
unsafe fn ZSTDv07_nextSrcSizeToDecompress(dctx: *mut ZSTDv07_DCtx) -> usize {
    (*dctx).expected
}
unsafe fn ZSTDv07_isSkipFrame(dctx: *mut ZSTDv07_DCtx) -> core::ffi::c_int {
    ((*dctx).stage as core::ffi::c_uint
        == ZSTDds_skipFrame as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int
}
unsafe fn ZSTDv07_decompressContinue(
    dctx: *mut ZSTDv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> Result<usize, Error> {
    if srcSize != (*dctx).expected {
        return Err(Error::srcSize_wrong);
    }
    if dstCapacity != 0 {
        ZSTDv07_checkContinuity(dctx, dst);
    }
    match (*dctx).stage as core::ffi::c_uint {
        0 => {
            if srcSize != ZSTDv07_frameHeaderSize_min {
                return Err(Error::srcSize_wrong);
            }
            if MEM_readLE32(src) & 0xfffffff0 as core::ffi::c_uint == ZSTDv07_MAGIC_SKIPPABLE_START
            {
                ptr::copy_nonoverlapping(
                    src as *const u8,
                    (*dctx).headerBuffer.as_mut_ptr(),
                    ZSTDv07_frameHeaderSize_min,
                );
                (*dctx).expected =
                    ZSTDv07_skippableHeaderSize.wrapping_sub(ZSTDv07_frameHeaderSize_min);
                (*dctx).stage = ZSTDds_decodeSkippableHeader;
                return Ok(0);
            }
            (*dctx).headerSize = ZSTDv07_frameHeaderSize(src, ZSTDv07_frameHeaderSize_min)?;
            ptr::copy_nonoverlapping(
                src as *const u8,
                (*dctx).headerBuffer.as_mut_ptr(),
                ZSTDv07_frameHeaderSize_min,
            );
            if (*dctx).headerSize > ZSTDv07_frameHeaderSize_min {
                (*dctx).expected = ((*dctx).headerSize).wrapping_sub(ZSTDv07_frameHeaderSize_min);
                (*dctx).stage = ZSTDds_decodeFrameHeader;
                return Ok(0);
            }
            (*dctx).expected = 0;
        }
        1 => {}
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_compressed,
                origSize: 0,
            };
            let cBlockSize = ZSTDv07_getcBlockSize(src, ZSTDv07_blockHeaderSize, &mut bp)?;
            if bp.blockType as core::ffi::c_uint == bt_end as core::ffi::c_int as core::ffi::c_uint
            {
                if (*dctx).fParams.checksumFlag != 0 {
                    let h64 = ZSTD_XXH64_digest(&mut (*dctx).xxhState);
                    let h32 = (h64 >> 11) as u32 & (((1) << 22) - 1) as u32;
                    let ip = src as *const u8;
                    let check32 = (*ip.add(2) as core::ffi::c_int
                        + ((*ip.add(1) as core::ffi::c_int) << 8)
                        + ((*ip as core::ffi::c_int & 0x3f as core::ffi::c_int) << 16))
                        as u32;
                    if check32 != h32 {
                        return Err(Error::checksum_wrong);
                    }
                }
                (*dctx).expected = 0;
                (*dctx).stage = ZSTDds_getFrameHeaderSize;
            } else {
                (*dctx).expected = cBlockSize;
                (*dctx).bType = bp.blockType;
                (*dctx).stage = ZSTDds_decompressBlock;
            }
            return Ok(0);
        }
        3 => {
            let rSize = match (*dctx).bType as core::ffi::c_uint {
                0 => ZSTDv07_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize),
                1 => ZSTDv07_copyRawBlock(dst, dstCapacity, src, srcSize),
                2 => return Err(Error::GENERIC),
                3 => Ok(0),
                _ => return Err(Error::GENERIC),
            };
            (*dctx).stage = ZSTDds_decodeBlockHeader;
            (*dctx).expected = ZSTDv07_blockHeaderSize;
            let rSize = rSize?;
            (*dctx).previousDstEnd =
                (dst as *mut core::ffi::c_char).add(rSize) as *const core::ffi::c_void;
            if (*dctx).fParams.checksumFlag != 0 {
                ZSTD_XXH64_update(&mut (*dctx).xxhState, dst, rSize as usize);
            }
            return Ok(rSize);
        }
        4 => {
            ptr::copy_nonoverlapping(
                src as *const u8,
                (*dctx)
                    .headerBuffer
                    .as_mut_ptr()
                    .add(ZSTDv07_frameHeaderSize_min),
                (*dctx).expected,
            );
            (*dctx).expected =
                MEM_readLE32((*dctx).headerBuffer.as_mut_ptr().add(4) as *const core::ffi::c_void)
                    as usize;
            (*dctx).stage = ZSTDds_skipFrame;
            return Ok(0);
        }
        5 => {
            (*dctx).expected = 0;
            (*dctx).stage = ZSTDds_getFrameHeaderSize;
            return Ok(0);
        }
        _ => return Err(Error::GENERIC),
    }
    ptr::copy_nonoverlapping(
        src as *const u8,
        (*dctx)
            .headerBuffer
            .as_mut_ptr()
            .add(ZSTDv07_frameHeaderSize_min),
        (*dctx).expected,
    );
    ZSTDv07_decodeFrameHeader(
        dctx,
        ((*dctx).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
        (*dctx).headerSize,
    )?;
    (*dctx).expected = ZSTDv07_blockHeaderSize;
    (*dctx).stage = ZSTDds_decodeBlockHeader;
    Ok(0)
}
unsafe fn ZSTDv07_refDictContent(dctx: *mut ZSTDv07_DCtx, dict: &[u8]) {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).vBase = dict.as_ptr().offset(
        -(((*dctx).previousDstEnd as *const core::ffi::c_char)
            .offset_from((*dctx).base as *const core::ffi::c_char) as core::ffi::c_long
            as isize),
    ) as *const core::ffi::c_void;
    (*dctx).base = dict.as_ptr().cast();
    (*dctx).previousDstEnd = dict.as_ptr().add(dict.len()) as *const core::ffi::c_void;
}
unsafe fn ZSTDv07_loadEntropy(dctx: *mut ZSTDv07_DCtx, mut dict: &[u8]) -> Result<usize, Error> {
    let dictSize = dict.len();
    let hSize = HUFv07_readDTableX4(((*dctx).hufTable).as_mut_ptr(), dict)
        .map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[hSize..];
    let mut offcodeNCount: [core::ffi::c_short; 29] = [0; 29];
    let mut offcodeMaxValue = MaxOff as u32;
    let mut offcodeLog: u32 = 0;
    let offcodeHeaderSize = FSEv07_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dict,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    if offcodeLog > OffFSELog as u32 {
        return Err(Error::dictionary_corrupted);
    }
    FSEv07_buildDTable(
        ((*dctx).OffTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        offcodeLog,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[offcodeHeaderSize..];
    let mut matchlengthNCount: [core::ffi::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as core::ffi::c_uint;
    let mut matchlengthLog: core::ffi::c_uint = 0;
    let matchlengthHeaderSize = FSEv07_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dict,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    if matchlengthLog > MLFSELog as core::ffi::c_uint {
        return Err(Error::dictionary_corrupted);
    }
    FSEv07_buildDTable(
        ((*dctx).MLTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[matchlengthHeaderSize..];
    let mut litlengthNCount: [core::ffi::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as core::ffi::c_uint;
    let mut litlengthLog: core::ffi::c_uint = 0;
    let litlengthHeaderSize = FSEv07_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dict,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    if litlengthLog > LLFSELog as core::ffi::c_uint {
        return Err(Error::dictionary_corrupted);
    }
    FSEv07_buildDTable(
        ((*dctx).LLTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
    )
    .map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[litlengthHeaderSize..];
    if dict.len() < 12 {
        return Err(Error::dictionary_corrupted);
    }
    *((*dctx).rep).as_mut_ptr() = MEM_readLE32(dict.as_ptr().cast());
    if *((*dctx).rep).as_mut_ptr() == 0 || *((*dctx).rep).as_mut_ptr() as usize >= dict.len() {
        return Err(Error::dictionary_corrupted);
    }
    *((*dctx).rep).as_mut_ptr().add(1) = MEM_readLE32(dict[4..].as_ptr().cast());
    if *((*dctx).rep).as_mut_ptr().add(1) == 0
        || *((*dctx).rep).as_mut_ptr().add(1) as usize >= dict.len()
    {
        return Err(Error::dictionary_corrupted);
    }
    *((*dctx).rep).as_mut_ptr().add(2) = MEM_readLE32(dict[8..].as_ptr().cast());
    if *((*dctx).rep).as_mut_ptr().add(2) == 0
        || *((*dctx).rep).as_mut_ptr().add(2) as usize >= dict.len()
    {
        return Err(Error::dictionary_corrupted);
    }
    dict = &dict[12..];
    (*dctx).fseEntropy = 1;
    (*dctx).litEntropy = (*dctx).fseEntropy;
    Ok(dictSize - dict.len())
}
unsafe fn ZSTDv07_decompress_insertDictionary(
    dctx: *mut ZSTDv07_DCtx,
    mut dict: &[u8],
) -> Result<(), Error> {
    if dict.len() < 8 {
        ZSTDv07_refDictContent(dctx, dict);
        return Ok(());
    }
    let magic = MEM_readLE32(dict.as_ptr().cast());
    if magic != ZSTDv07_DICT_MAGIC {
        ZSTDv07_refDictContent(dctx, dict);
        return Ok(());
    }
    (*dctx).dictID = MEM_readLE32(dict[4..].as_ptr() as *const core::ffi::c_void);
    dict = &dict[8..];
    let eSize = ZSTDv07_loadEntropy(dctx, dict).map_err(|_| Error::dictionary_corrupted)?;
    dict = &dict[eSize..];
    ZSTDv07_refDictContent(dctx, dict);
    Ok(())
}
unsafe fn ZSTDv07_decompressBegin_usingDict(
    dctx: *mut ZSTDv07_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: usize,
) -> Result<(), Error> {
    ZSTDv07_decompressBegin(dctx);
    if !dict.is_null() && dictSize != 0 {
        ZSTDv07_decompress_insertDictionary(
            dctx,
            core::slice::from_raw_parts(dict.cast::<u8>(), dictSize),
        )
        .map_err(|_| Error::dictionary_corrupted)?;
    }
    Ok(())
}
pub(crate) unsafe fn ZBUFFv07_createDCtx() -> *mut ZBUFFv07_DCtx {
    ZBUFFv07_createDCtx_advanced(defaultCustomMem)
}
unsafe fn ZBUFFv07_createDCtx_advanced(mut customMem: ZSTDv07_customMem) -> *mut ZBUFFv07_DCtx {
    let mut zbd = core::ptr::null_mut::<ZBUFFv07_DCtx>();
    if (customMem.customAlloc).is_none() && (customMem.customFree).is_none() {
        customMem = defaultCustomMem;
    }
    if (customMem.customAlloc).is_none() || (customMem.customFree).is_none() {
        return core::ptr::null_mut();
    }
    zbd = (customMem.customAlloc).unwrap_unchecked()(
        customMem.opaque,
        ::core::mem::size_of::<ZBUFFv07_DCtx>(),
    ) as *mut ZBUFFv07_DCtx;
    if zbd.is_null() {
        return core::ptr::null_mut();
    }
    ptr::write_bytes(zbd as *mut u8, 0, ::core::mem::size_of::<ZBUFFv07_DCtx>());
    ptr::write(&mut (*zbd).customMem, customMem);
    (*zbd).zd = ZSTDv07_createDCtx_advanced(customMem);
    if ((*zbd).zd).is_null() {
        ZBUFFv07_freeDCtx(zbd);
        return core::ptr::null_mut();
    }
    (*zbd).stage = ZBUFFds_init;
    zbd
}
pub(crate) unsafe fn ZBUFFv07_freeDCtx(zbd: *mut ZBUFFv07_DCtx) -> usize {
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
pub(crate) unsafe fn ZBUFFv07_decompressInitDictionary(
    zbd: *mut ZBUFFv07_DCtx,
    dict: *const core::ffi::c_void,
    dictSize: usize,
) -> Result<(), Error> {
    (*zbd).stage = ZBUFFds_loadHeader;
    (*zbd).outEnd = 0;
    (*zbd).outStart = (*zbd).outEnd;
    (*zbd).inPos = (*zbd).outStart;
    (*zbd).lhSize = (*zbd).inPos;
    ZSTDv07_decompressBegin_usingDict((*zbd).zd, dict, dictSize)
}
#[inline]
unsafe fn ZBUFFv07_limitCopy(
    dst: *mut core::ffi::c_void,
    dstCapacity: usize,
    src: *const core::ffi::c_void,
    srcSize: usize,
) -> usize {
    let length = if dstCapacity < srcSize {
        dstCapacity
    } else {
        srcSize
    };
    if length > 0 {
        ptr::copy_nonoverlapping(src, dst, length);
    }
    length
}

#[allow(clippy::drop_non_drop)]
pub(crate) unsafe fn ZBUFFv07_decompressContinue(
    zbd: *mut ZBUFFv07_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacityPtr: *mut usize,
    src: *const core::ffi::c_void,
    srcSizePtr: *mut usize,
) -> Result<usize, Error> {
    let istart = src as *const u8;
    let iend = istart.add(*srcSizePtr);
    let mut ip = istart;
    let ostart = dst as *mut u8;
    let oend = ostart.add(*dstCapacityPtr);
    let mut op = ostart;
    let mut notDone = 1;

    while notDone != 0 {
        #[derive(Eq, PartialEq)]
        enum Block {
            Read,
            Load,
            Flush,
        }
        let mut current_block: Block;
        match (*zbd).stage {
            ZBUFFds_init => return Err(Error::init_missing),
            ZBUFFds_loadHeader => {
                let hSize = ZSTDv07_getFrameParams(
                    &mut (*zbd).fParams,
                    ((*zbd).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
                    (*zbd).lhSize,
                )?;
                if hSize != 0 {
                    // if hSize!=0, hSize > zbd->lhSize
                    let toLoad = hSize - (*zbd).lhSize;
                    if toLoad > iend.offset_from(ip) as usize {
                        // not enough input to load full header
                        if !ip.is_null() {
                            ptr::copy_nonoverlapping(
                                ip,
                                (*zbd).headerBuffer.as_mut_ptr().add((*zbd).lhSize),
                                iend.offset_from(ip) as usize,
                            );
                        }
                        (*zbd).lhSize = ((*zbd).lhSize).wrapping_add(iend.offset_from(ip) as usize);
                        *dstCapacityPtr = 0;
                        return Ok(hSize
                            .wrapping_sub((*zbd).lhSize)
                            .wrapping_add(ZSTDv07_blockHeaderSize));
                    }
                    ptr::copy_nonoverlapping(
                        ip,
                        (*zbd).headerBuffer.as_mut_ptr().add((*zbd).lhSize),
                        toLoad,
                    );
                    (*zbd).lhSize = hSize;
                    ip = ip.add(toLoad);
                    continue;
                }

                // Consume header
                let h1Size = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd); // == ZSTDv07_frameHeaderSize_min
                ZSTDv07_decompressContinue(
                    (*zbd).zd,
                    core::ptr::null_mut(),
                    0,
                    ((*zbd).headerBuffer).as_mut_ptr() as *const core::ffi::c_void,
                    h1Size,
                )?;
                if h1Size < (*zbd).lhSize {
                    // long header
                    let h2Size = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
                    ZSTDv07_decompressContinue(
                        (*zbd).zd,
                        core::ptr::null_mut(),
                        0,
                        ((*zbd).headerBuffer).as_mut_ptr().add(h1Size) as *const core::ffi::c_void,
                        h2Size,
                    )?;
                }
                (*zbd).fParams.windowSize = std::cmp::max((*zbd).fParams.windowSize, 1 << 10);

                // Frame header instruct buffer sizes
                let blockSize = std::cmp::min((*zbd).fParams.windowSize, 128 * 1024) as usize;
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
                        return Err(Error::memory_allocation);
                    }
                }
                let neededOutSize = ((*zbd).fParams.windowSize as usize)
                    .wrapping_add(blockSize)
                    .wrapping_add((WILDCOPY_OVERLENGTH * 2) as usize);
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
                        return Err(Error::memory_allocation);
                    }
                }
                (*zbd).stage = ZBUFFds_read;
                current_block = Block::Read;
            }
            ZBUFFds_read => {
                current_block = Block::Read;
            }
            ZBUFFds_load => {
                current_block = Block::Load;
            }
            ZBUFFds_flush => {
                current_block = Block::Flush;
            }
            _ => return Err(Error::GENERIC),
        }
        if current_block == Block::Read {
            drop(current_block);

            let neededInSize = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
            if neededInSize == 0 {
                // end of frame
                (*zbd).stage = ZBUFFds_init;
                notDone = 0;
                continue;
            }
            if iend.offset_from(ip) as usize >= neededInSize {
                // decode directly from src
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
                )?;
                ip = ip.add(neededInSize);
                if decodedSize == 0 && isSkipFrame == 0 {
                    // this was just a header
                    continue;
                }
                (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize);
                (*zbd).stage = ZBUFFds_flush;
                continue;
            } else if ip == iend {
                // no more input
                notDone = 0;
                continue;
            }
            (*zbd).stage = ZBUFFds_load;
            current_block = Block::Load;
        }

        if current_block == Block::Load {
            drop(current_block);
            let neededInSize_0 = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
            // should always be <= remaining space within inBuff
            let toLoad = neededInSize_0.wrapping_sub((*zbd).inPos);
            let mut loadedSize: usize = 0;
            if toLoad > ((*zbd).inBuffSize).wrapping_sub((*zbd).inPos) {
                return Err(Error::corruption_detected); // should never happen
            }
            loadedSize = ZBUFFv07_limitCopy(
                ((*zbd).inBuff).add((*zbd).inPos) as *mut core::ffi::c_void,
                toLoad,
                ip as *const core::ffi::c_void,
                iend.offset_from(ip) as usize,
            );
            ip = ip.add(loadedSize);
            (*zbd).inPos = ((*zbd).inPos).wrapping_add(loadedSize);
            if loadedSize < toLoad {
                // not enough input, wait for more
                notDone = 0;
                continue;
            }

            // decode loaded input
            let isSkipFrame_0 = ZSTDv07_isSkipFrame((*zbd).zd);
            let decodedSize_0 = ZSTDv07_decompressContinue(
                (*zbd).zd,
                ((*zbd).outBuff).add((*zbd).outStart) as *mut core::ffi::c_void,
                ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart),
                (*zbd).inBuff as *const core::ffi::c_void,
                neededInSize_0,
            )?;
            (*zbd).inPos = 0; // input is consumed
            if decodedSize_0 == 0 && isSkipFrame_0 == 0 {
                (*zbd).stage = ZBUFFds_read;
                continue;
            }
            (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize_0);
            (*zbd).stage = ZBUFFds_flush;
            current_block = Block::Flush;
        }

        if current_block == Block::Flush {
            drop(current_block);

            let toFlushSize = ((*zbd).outEnd).wrapping_sub((*zbd).outStart);
            let flushedSize = ZBUFFv07_limitCopy(
                op as *mut core::ffi::c_void,
                oend.offset_from(op) as usize,
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
            }
            // cannot flush everything
            notDone = 0;
        }
    }

    // result
    *srcSizePtr = ip.offset_from(istart) as usize;
    *dstCapacityPtr = op.offset_from(ostart) as usize;
    let mut nextSrcSizeHint = ZSTDv07_nextSrcSizeToDecompress((*zbd).zd);
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub((*zbd).inPos); // already loaded
    Ok(nextSrcSizeHint)
}
