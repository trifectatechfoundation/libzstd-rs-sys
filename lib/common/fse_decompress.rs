type size_t = std::ffi::c_ulong;
type unalign32 = u32;
type unalign64 = u64;
type C2RustUnnamed = std::ffi::c_uint;

const ZSTD_error_maxCode: C2RustUnnamed = 120;
const ZSTD_error_externalSequences_invalid: C2RustUnnamed = 107;
const ZSTD_error_sequenceProducer_failed: C2RustUnnamed = 106;
const ZSTD_error_srcBuffer_wrong: C2RustUnnamed = 105;
const ZSTD_error_dstBuffer_wrong: C2RustUnnamed = 104;
const ZSTD_error_seekableIO: C2RustUnnamed = 102;
const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed = 100;
const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed = 82;
const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed = 80;
const ZSTD_error_dstBuffer_null: C2RustUnnamed = 74;
const ZSTD_error_srcSize_wrong: C2RustUnnamed = 72;
const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
const ZSTD_error_init_missing: C2RustUnnamed = 62;
const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed = 49;
const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed = 48;
const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed = 46;
const ZSTD_error_tableLog_tooLarge: C2RustUnnamed = 44;
const ZSTD_error_parameter_outOfBound: C2RustUnnamed = 42;
const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed = 41;
const ZSTD_error_parameter_unsupported: C2RustUnnamed = 40;
const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed = 34;
const ZSTD_error_dictionary_wrong: C2RustUnnamed = 32;
const ZSTD_error_dictionary_corrupted: C2RustUnnamed = 30;
const ZSTD_error_literals_headerWrong: C2RustUnnamed = 24;
const ZSTD_error_checksum_wrong: C2RustUnnamed = 22;
const ZSTD_error_corruption_detected: C2RustUnnamed = 20;
const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed = 16;
const ZSTD_error_frameParameter_unsupported: C2RustUnnamed = 14;
const ZSTD_error_version_unsupported: C2RustUnnamed = 12;
const ZSTD_error_prefix_unknown: C2RustUnnamed = 10;
const ZSTD_error_GENERIC: C2RustUnnamed = 1;
const ZSTD_error_no_error: C2RustUnnamed = 0;

enum Error {
    GENERIC = 1,
    prefix_unknown = 10,
    version_unsupported = 12,
    frameParameter_unsupported = 14,
    frameParameter_windowTooLarge = 16,
    corruption_detected = 20,
    checksum_wrong = 22,
    literals_headerWrong = 24,
    dictionary_corrupted = 30,
    dictionary_wrong = 32,
    dictionaryCreation_failed = 34,
    parameter_unsupported = 40,
    parameter_combination_unsupported = 41,
    parameter_outOfBound = 42,
    tableLog_tooLarge = 44,
    maxSymbolValue_tooLarge = 46,
    maxSymbolValue_tooSmall = 48,
    cannotProduce_uncompressedBlock = 49,
    stabilityCondition_notRespected = 50,
    stage_wrong = 60,
    init_missing = 62,
    memory_allocation = 64,
    workSpace_tooSmall = 66,
    dstSize_tooSmall = 70,
    srcSize_wrong = 72,
    dstBuffer_null = 74,
    noForwardProgress_destFull = 80,
    noForwardProgress_inputEmpty = 82,
    frameIndex_tooLarge = 100,
    seekableIO = 102,
    dstBuffer_wrong = 104,
    srcBuffer_wrong = 105,
    sequenceProducer_failed = 106,
    externalSequences_invalid = 107,
    maxCode = 120,
}

type BitContainerType = size_t;

#[derive(Copy, Clone)]
#[repr(C)]
struct BIT_DStream_t {
    bitContainer: BitContainerType,
    bitsConsumed: std::ffi::c_uint,
    ptr: *const std::ffi::c_char,
    start: *const std::ffi::c_char,
    limitPtr: *const std::ffi::c_char,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StreamStatus {
    Unfinished = 0,
    EndOfBuffer = 1,
    Completed = 2,
    Overflow = 3,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct FSE_decode_t {
    pub newState: std::ffi::c_ushort,
    pub symbol: std::ffi::c_uchar,
    pub nbBits: std::ffi::c_uchar,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, align(4))]
pub(crate) struct FSE_DTable {
    pub header: FSE_DTableHeader,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub(crate) struct FSE_DTableHeader {
    pub tableLog: u16,
    pub fastMode: u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub(crate) struct FSE_DecompressWksp {
    pub ncount: [std::ffi::c_short; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
struct FSE_DState_t<'a> {
    state: size_t,
    table: &'a [FSE_decode_t; 90],
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
use crate::{
    lib::common::entropy_common::{DTable, FSE_readNCount_bmi2, Workspace},
    MEM_readLEST, MEM_write64,
};
const fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}

impl BIT_DStream_t {
    unsafe fn new(mut srcBuffer: &[u8]) -> Result<Self, size_t> {
        let mut bitD = Self {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: std::ptr::null::<std::ffi::c_char>(),
            start: std::ptr::null::<std::ffi::c_char>(),
            limitPtr: std::ptr::null::<std::ffi::c_char>(),
        };

        if srcBuffer.is_empty() {
            return Err(-(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t);
        }

        const USIZE_BYTES: usize = size_of::<BitContainerType>();

        if let Some(chunk) = srcBuffer.last_chunk() {
            bitD.start = srcBuffer.as_ptr() as *const std::ffi::c_char;
            bitD.limitPtr = (bitD.start).add(USIZE_BYTES);

            bitD.ptr = (srcBuffer.as_ptr() as *const std::ffi::c_char)
                .add(srcBuffer.len())
                .offset(-(USIZE_BYTES as isize));
            bitD.bitContainer = usize::from_le_bytes(*chunk) as size_t;

            match srcBuffer.last().and_then(|v| v.checked_ilog2()) {
                None => {
                    /* endMark not present */
                    return Err(-(ZSTD_error_GENERIC as std::ffi::c_int) as size_t);
                }
                Some(v) => {
                    bitD.bitsConsumed = 8 - v;
                }
            }
        } else {
            bitD.start = srcBuffer.as_ptr() as *const std::ffi::c_char;
            bitD.limitPtr = (bitD.start)
                .offset(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize);
            bitD.ptr = bitD.start;

            bitD.bitContainer = u64::from(srcBuffer[0]);

            if srcBuffer.len() >= 7 {
                bitD.bitContainer += u64::from(srcBuffer[6]) << (USIZE_BYTES * 8 - 16);
            }
            if srcBuffer.len() >= 6 {
                bitD.bitContainer += u64::from(srcBuffer[5]) << (USIZE_BYTES * 8 - 24);
            }
            if srcBuffer.len() >= 5 {
                bitD.bitContainer += u64::from(srcBuffer[4]) << (USIZE_BYTES * 8 - 32);
            }
            if srcBuffer.len() >= 4 {
                bitD.bitContainer += u64::from(srcBuffer[3]) << 24;
            }
            if srcBuffer.len() >= 3 {
                bitD.bitContainer += u64::from(srcBuffer[2]) << 16;
            }
            if srcBuffer.len() >= 2 {
                bitD.bitContainer += u64::from(srcBuffer[1]) << 8;
            }

            match srcBuffer.last().and_then(|v| v.checked_ilog2()) {
                None => {
                    /* endMark not present */
                    return Err(-(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t);
                }
                Some(v) => {
                    bitD.bitsConsumed = 8 - v;
                }
            }

            bitD.bitsConsumed += ((USIZE_BYTES - srcBuffer.len()) * 8) as u32;
        }

        Ok(bitD)
    }
}

#[inline(always)]
const fn BIT_getMiddleBits(
    bitContainer: BitContainerType,
    start: u32,
    nbBits: u32,
) -> BitContainerType {
    const MASK: usize = size_of::<BitContainerType>() * 8 - 1;
    bitContainer >> (start & MASK as u32) & (1u64 << nbBits).wrapping_sub(1)
}

#[inline(always)]
const fn BIT_lookBits(bitD: &BIT_DStream_t, nbBits: u32) -> BitContainerType {
    BIT_getMiddleBits(
        bitD.bitContainer,
        (size_of::<BitContainerType>() as u64)
            .wrapping_mul(8)
            .wrapping_sub(bitD.bitsConsumed as std::ffi::c_ulong)
            .wrapping_sub(nbBits as std::ffi::c_ulong) as u32,
        nbBits,
    )
}

#[inline]
const fn BIT_lookBitsFast(bitD: &BIT_DStream_t, nbBits: u32) -> BitContainerType {
    const MASK: u32 = (size_of::<BitContainerType>() * 8 - 1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & MASK)
        >> (MASK.wrapping_add(1).wrapping_sub(nbBits) & MASK)
}

#[inline(always)]
const fn BIT_skipBits(bitD: &mut BIT_DStream_t, mut nbBits: u32) {
    bitD.bitsConsumed += nbBits;
}

#[inline(always)]
const fn BIT_readBits(bitD: &mut BIT_DStream_t, nbBits: std::ffi::c_uint) -> BitContainerType {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    value
}

#[inline]
const fn BIT_readBitsFast(bitD: &mut BIT_DStream_t, nbBits: std::ffi::c_uint) -> size_t {
    let value = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    value
}

impl BIT_DStream_t {
    fn reload_internal(&mut self) {
        self.ptr = unsafe { (self.ptr).sub(self.bitsConsumed as usize / 8) };
        self.bitsConsumed &= 7;
        self.bitContainer = unsafe { MEM_readLEST(self.ptr as *const std::ffi::c_void) };
    }
}

#[inline(always)]
unsafe fn BIT_reloadDStream(bitD: &mut BIT_DStream_t) -> StreamStatus {
    if bitD.bitsConsumed > (size_of::<BitContainerType>() as u32) * 8 {
        static zeroFilled: BitContainerType = 0 as std::ffi::c_int as BitContainerType;
        bitD.ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char;

        return StreamStatus::Overflow;
    }

    if bitD.ptr >= bitD.limitPtr {
        bitD.reload_internal();

        return StreamStatus::Unfinished;
    }

    if bitD.ptr == bitD.start {
        return if bitD.bitsConsumed < size_of::<BitContainerType>() as u32 * 8 {
            StreamStatus::EndOfBuffer
        } else {
            StreamStatus::Completed
        };
    }

    let mut nbBytes = bitD.bitsConsumed / 8;
    let result = if (bitD.ptr).sub(nbBytes as usize) < bitD.start {
        nbBytes = (bitD.ptr).offset_from(bitD.start) as u32;

        StreamStatus::EndOfBuffer
    } else {
        StreamStatus::Unfinished
    };

    bitD.ptr = bitD.ptr.sub(nbBytes as usize);
    bitD.bitsConsumed = (bitD.bitsConsumed).wrapping_sub(nbBytes * 8);
    bitD.bitContainer = MEM_readLEST(bitD.ptr as *const std::ffi::c_void);

    result
}

impl<'a> FSE_DState_t<'a> {
    unsafe fn new(mut bitD: &mut BIT_DStream_t, mut dt: &'a DTable) -> Self {
        let state = BIT_readBits(bitD, dt.header.tableLog as std::ffi::c_uint);
        let _ = BIT_reloadDStream(bitD);
        let table = &dt.elements;

        Self { state, table }
    }
}

#[inline]
fn FSE_decodeSymbol(DStatePtr: &mut FSE_DState_t, bitD: &mut BIT_DStream_t) -> u8 {
    let FSE_decode_t {
        nbBits,
        symbol,
        newState,
    } = DStatePtr.table[DStatePtr.state as usize];

    let lowBits = BIT_readBits(bitD, u32::from(nbBits));
    DStatePtr.state = size_t::from(newState) + lowBits;

    symbol
}

#[inline]
fn FSE_decodeSymbolFast(DStatePtr: &mut FSE_DState_t, bitD: &mut BIT_DStream_t) -> u8 {
    let FSE_decode_t {
        nbBits,
        symbol,
        newState,
    } = DStatePtr.table[DStatePtr.state as usize];

    let lowBits = BIT_readBitsFast(bitD, u32::from(nbBits));
    DStatePtr.state = size_t::from(newState) + lowBits;

    symbol
}

pub const FSE_MAX_MEMORY_USAGE: std::ffi::c_int = 14 as std::ffi::c_int;
pub const FSE_MAX_SYMBOL_VALUE: std::ffi::c_int = 255 as std::ffi::c_int;
pub const FSE_MAX_TABLELOG: std::ffi::c_int = FSE_MAX_MEMORY_USAGE - 2 as std::ffi::c_int;
pub const FSE_isError: fn(size_t) -> std::ffi::c_uint = ERR_isError;

fn FSE_buildDTable_internal(
    mut dt: &mut DTable,
    mut normalizedCounter: &[std::ffi::c_short; 256],
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
) -> size_t {
    let wkspSize = dt.elements[(1 << tableLog)..].len() * 4;
    let (header, elements, symbols, spread) = dt.destructure_mut(maxSymbolValue, tableLog);
    let maxSV1 = maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let tableSize = ((1 as std::ffi::c_int) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1 as std::ffi::c_int as u32);

    if ((::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong).wrapping_mul(
        maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_ulong,
    ) as std::ffi::c_ulonglong)
        .wrapping_add((1 as std::ffi::c_ulonglong) << tableLog)
        .wrapping_add(8 as std::ffi::c_int as std::ffi::c_ulonglong)
        > wkspSize as std::ffi::c_ulonglong
    {
        return -(ZSTD_error_maxSymbolValue_tooLarge as std::ffi::c_int) as size_t;
    }

    if maxSymbolValue > FSE_MAX_SYMBOL_VALUE as std::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as std::ffi::c_int) as size_t;
    }

    if tableLog > FSE_MAX_TABLELOG as std::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }

    let mut DTableH = FSE_DTableHeader {
        tableLog: tableLog as u16,
        fastMode: 1,
    };

    let largeLimit = ((1 as std::ffi::c_int)
        << tableLog.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint))
        as i16;
    let mut s: u32 = 0;
    s = 0 as std::ffi::c_int as u32;
    while s < maxSV1 {
        if normalizedCounter[s as usize] as std::ffi::c_int == -(1 as std::ffi::c_int) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            elements[fresh0 as usize].symbol = s as u8;
            symbols[s as usize] = 1;
        } else {
            if normalizedCounter[s as usize] as std::ffi::c_int >= largeLimit as std::ffi::c_int {
                DTableH.fastMode = 0 as std::ffi::c_int as u16;
            }
            symbols[s as usize] = normalizedCounter[s as usize] as u16;
        }
        s = s.wrapping_add(1);
    }

    *header = DTableH;

    if highThreshold == tableSize.wrapping_sub(1 as std::ffi::c_int as u32) {
        let tableMask = tableSize.wrapping_sub(1 as std::ffi::c_int as u32) as size_t;
        let step = (tableSize >> 1 as std::ffi::c_int)
            .wrapping_add(tableSize >> 3 as std::ffi::c_int)
            .wrapping_add(3 as std::ffi::c_int as u32) as size_t;
        let add = 0x101010101010101 as std::ffi::c_ulonglong as u64;
        let mut pos = 0 as std::ffi::c_int as size_t;
        let mut sv = 0 as std::ffi::c_int as u64;

        for s_0 in 0..maxSV1 {
            let mut i: std::ffi::c_int = 0;
            let n = normalizedCounter[s_0 as usize] as std::ffi::c_int;
            spread[pos as usize..][..8].copy_from_slice(&sv.to_le_bytes());
            i = 8 as std::ffi::c_int;
            while i < n {
                spread[pos as usize..][i as usize..][..8].copy_from_slice(&sv.to_le_bytes());
                i += 8 as std::ffi::c_int;
            }
            pos = pos.wrapping_add(n as size_t);
            sv = sv.wrapping_add(add);
        }

        let mut position = 0 as std::ffi::c_int as size_t;
        let mut s_1: size_t = 0;
        let unroll = 2 as std::ffi::c_int as size_t;
        s_1 = 0 as std::ffi::c_int as size_t;
        while s_1 < tableSize as size_t {
            let mut u: size_t = 0;
            u = 0 as std::ffi::c_int as size_t;
            while u < unroll {
                let uPosition = position.wrapping_add(u * step) & tableMask;
                elements[uPosition as usize].symbol = spread[(s_1 + u) as usize];
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll * step) & tableMask;
            s_1 = s_1.wrapping_add(unroll);
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1);
        let step_0 = (tableSize >> 1)
            .wrapping_add(tableSize >> 3)
            .wrapping_add(3);

        let mut position_0 = 0 as std::ffi::c_int as u32;
        for s_2 in 0..maxSV1 {
            for _ in 0..normalizedCounter[s_2 as usize] {
                elements[position_0 as usize].symbol = s_2 as u8;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
            }
        }

        if position_0 != 0 as std::ffi::c_int as u32 {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
    }

    for u in 0..tableSize {
        let symbol = usize::from((elements[u as usize]).symbol);
        let nextState = u32::from(symbols[symbol]);
        symbols[symbol] += 1;
        (elements[u as usize]).nbBits = tableLog.wrapping_sub(nextState.ilog2()) as u8;
        (elements[u as usize]).newState = (nextState
            << (elements[u as usize]).nbBits as std::ffi::c_int)
            .wrapping_sub(tableSize) as u16;
    }

    0 as std::ffi::c_int as size_t
}

#[inline(always)]
unsafe fn FSE_decompress_usingDTable_generic(
    dst: &mut [u8],
    mut cSrc: &[u8],
    mut dt: &DTable,
    fast: bool,
) -> size_t {
    enum Mode {
        Slow,
        Fast,
    }

    let mode = if fast { Mode::Fast } else { Mode::Slow };

    let mut op = 0;
    let omax = dst.len();
    let olimit = omax - 3;

    let mut bitD = match BIT_DStream_t::new(cSrc) {
        Err(e) => return e,
        Ok(bitD) => bitD,
    };

    let mut state1 = FSE_DState_t::new(&mut bitD, dt);
    let mut state2 = FSE_DState_t::new(&mut bitD, dt);

    if let StreamStatus::Overflow = BIT_reloadDStream(&mut bitD) {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }

    while BIT_reloadDStream(&mut bitD) == StreamStatus::Unfinished && op < olimit {
        dst[op] = match mode {
            Mode::Fast => FSE_decodeSymbolFast(&mut state1, &mut bitD),
            Mode::Slow => FSE_decodeSymbol(&mut state1, &mut bitD),
        };

        if (FSE_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            let _ = BIT_reloadDStream(&mut bitD);
        }

        dst[op + 1] = match mode {
            Mode::Fast => FSE_decodeSymbolFast(&mut state2, &mut bitD),
            Mode::Slow => FSE_decodeSymbol(&mut state2, &mut bitD),
        };

        if (FSE_MAX_TABLELOG * 4 + 7) as std::ffi::c_ulong
            > (size_of::<BitContainerType>() as std::ffi::c_ulong).wrapping_mul(8)
            && BIT_reloadDStream(&mut bitD) != StreamStatus::Unfinished
        {
            op += 2;
            break;
        }

        dst[op + 2] = match mode {
            Mode::Fast => FSE_decodeSymbolFast(&mut state1, &mut bitD),
            Mode::Slow => FSE_decodeSymbol(&mut state1, &mut bitD),
        };

        if (FSE_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            let _ = BIT_reloadDStream(&mut bitD);
        }

        dst[op + 3] = match mode {
            Mode::Fast => FSE_decodeSymbolFast(&mut state2, &mut bitD),
            Mode::Slow => FSE_decodeSymbol(&mut state2, &mut bitD),
        };

        op += 4;
    }

    loop {
        if op > omax - 2 {
            return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
        }

        dst[op] = match mode {
            Mode::Fast => FSE_decodeSymbolFast(&mut state1, &mut bitD),
            Mode::Slow => FSE_decodeSymbol(&mut state1, &mut bitD),
        };
        op += 1;

        if let StreamStatus::Overflow = BIT_reloadDStream(&mut bitD) {
            dst[op] = match mode {
                Mode::Fast => FSE_decodeSymbolFast(&mut state2, &mut bitD),
                Mode::Slow => FSE_decodeSymbol(&mut state2, &mut bitD),
            };
            op += 1;
            break;
        } else {
            if op > omax - 2 {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }

            dst[op] = match mode {
                Mode::Fast => FSE_decodeSymbolFast(&mut state2, &mut bitD),
                Mode::Slow => FSE_decodeSymbol(&mut state2, &mut bitD),
            };
            op += 1;

            match BIT_reloadDStream(&mut bitD) {
                StreamStatus::Overflow => { /* fall through */ }
                _ => continue,
            }

            dst[op] = match mode {
                Mode::Fast => FSE_decodeSymbolFast(&mut state1, &mut bitD),
                Mode::Slow => FSE_decodeSymbol(&mut state1, &mut bitD),
            };
            op += 1;

            break;
        }
    }

    op as size_t
}

#[inline(always)]
unsafe fn FSE_decompress_wksp_body(
    mut dst: &mut [u8],
    mut cSrc: &[u8],
    mut maxLog: std::ffi::c_uint,
    workspace: &mut Workspace,
    mut bmi2: std::ffi::c_int,
) -> size_t {
    let mut wkspSize = size_of::<Workspace>() as size_t;

    let mut tableLog: std::ffi::c_uint = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as std::ffi::c_uint;
    if wkspSize < ::core::mem::size_of::<FSE_DecompressWksp>() as std::ffi::c_ulong {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    let NCountLength = FSE_readNCount_bmi2(
        &mut workspace.a.ncount,
        &mut maxSymbolValue,
        &mut tableLog,
        cSrc,
        bmi2,
    );
    if ERR_isError(NCountLength) != 0 {
        return NCountLength;
    }
    if tableLog > maxLog {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    let ip = &cSrc[NCountLength as usize..];
    if ((1 as std::ffi::c_int + ((1 as std::ffi::c_int) << tableLog) + 1 as std::ffi::c_int)
        as std::ffi::c_ulonglong)
        .wrapping_add(
            ((::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong).wrapping_mul(
                maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_ulong,
            ) as std::ffi::c_ulonglong)
                .wrapping_add((1 as std::ffi::c_ulonglong) << tableLog)
                .wrapping_add(8 as std::ffi::c_int as std::ffi::c_ulonglong)
                .wrapping_add(
                    ::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong
                        as std::ffi::c_ulonglong,
                )
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong)
                .wrapping_div(
                    ::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong
                        as std::ffi::c_ulonglong,
                ),
        )
        .wrapping_add(
            ((FSE_MAX_SYMBOL_VALUE + 1 as std::ffi::c_int) / 2 as std::ffi::c_int)
                as std::ffi::c_ulonglong,
        )
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulonglong)
        .wrapping_mul(
            ::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong
                as std::ffi::c_ulonglong,
        )
        > wkspSize as std::ffi::c_ulonglong
    {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    wkspSize = (wkspSize as std::ffi::c_ulong).wrapping_sub(
        (::core::mem::size_of::<FSE_DecompressWksp>() as std::ffi::c_ulong).wrapping_add(
            ((1 as std::ffi::c_int + ((1 as std::ffi::c_int) << tableLog)) as std::ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<FSE_DTable>() as std::ffi::c_ulong),
        ),
    ) as size_t as size_t;

    let _var_err__ = FSE_buildDTable_internal(
        &mut workspace.dtable,
        &workspace.a.ncount,
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }

    FSE_decompress_usingDTable_generic(
        dst,
        ip,
        &workspace.dtable,
        workspace.dtable.header.fastMode != 0,
    )
}
unsafe fn FSE_decompress_wksp_body_default(
    dst: &mut [u8],
    cSrc: &[u8],
    maxLog: std::ffi::c_uint,
    workSpace: &mut Workspace,
) -> size_t {
    FSE_decompress_wksp_body(dst, cSrc, maxLog, workSpace, 0 as std::ffi::c_int)
}
unsafe fn FSE_decompress_wksp_body_bmi2(
    dst: &mut [u8],
    cSrc: &[u8],
    maxLog: std::ffi::c_uint,
    workSpace: &mut Workspace,
) -> size_t {
    FSE_decompress_wksp_body(dst, cSrc, maxLog, workSpace, 1 as std::ffi::c_int)
}

pub unsafe fn FSE_decompress_wksp_bmi2(
    dst: &mut [u8],
    cSrc: &[u8],
    maxLog: std::ffi::c_uint,
    workSpace: &mut Workspace,
    bmi2: bool,
) -> size_t {
    if bmi2 {
        FSE_decompress_wksp_body_bmi2(dst, cSrc, maxLog, workSpace)
    } else {
        FSE_decompress_wksp_body_default(dst, cSrc, maxLog, workSpace)
    }
}
