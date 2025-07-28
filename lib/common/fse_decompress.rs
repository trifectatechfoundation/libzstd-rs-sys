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

type BIT_DStream_status = std::ffi::c_uint;
const BIT_DStream_overflow: BIT_DStream_status = 3;
const BIT_DStream_completed: BIT_DStream_status = 2;
const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
const BIT_DStream_unfinished: BIT_DStream_status = 0;

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
unsafe extern "C" fn BIT_getMiddleBits(
    mut bitContainer: BitContainerType,
    start: u32,
    nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as u32;
    bitContainer >> (start & regMask)
        & ((1 as std::ffi::c_int as u64) << nbBits).wrapping_sub(1 as std::ffi::c_int as u64)
}
#[inline(always)]
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    BIT_getMiddleBits(
        (*bitD).bitContainer,
        (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
            .wrapping_sub((*bitD).bitsConsumed as std::ffi::c_ulong)
            .wrapping_sub(nbBits as std::ffi::c_ulong) as u32,
        nbBits,
    )
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask
            .wrapping_add(1 as std::ffi::c_int as u32)
            .wrapping_sub(nbBits)
            & regMask)
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline(always)]
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> BitContainerType {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BIT_readBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> size_t {
    let value = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BIT_reloadDStream_internal(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    (*bitD).ptr = ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3 as std::ffi::c_int) as isize));
    (*bitD).bitsConsumed &= 7 as std::ffi::c_int as std::ffi::c_uint;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    BIT_DStream_unfinished
}
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t) -> BIT_DStream_status {
    if ((*bitD).bitsConsumed as std::ffi::c_ulong
        > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)) as std::ffi::c_int
        as std::ffi::c_long
        != 0
    {
        static mut zeroFilled: BitContainerType = 0 as std::ffi::c_int as BitContainerType;
        (*bitD).ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char;
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStream_internal(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as std::ffi::c_ulong)
            < (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3 as std::ffi::c_int;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as u32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed =
        ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8 as std::ffi::c_int as u32);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    result
}

impl<'a> FSE_DState_t<'a> {
    unsafe fn new(mut bitD: &mut BIT_DStream_t, mut dt: &'a DTable) -> Self {
        let state = BIT_readBits(bitD, dt.header.tableLog as std::ffi::c_uint);
        BIT_reloadDStream(bitD);
        let table = &dt.elements;

        Self { state, table }
    }
}

#[inline]
unsafe extern "C" fn FSE_decodeSymbol(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe extern "C" fn FSE_decodeSymbolFast(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as u32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
pub const FSE_MAX_MEMORY_USAGE: std::ffi::c_int = 14 as std::ffi::c_int;
pub const FSE_MAX_SYMBOL_VALUE: std::ffi::c_int = 255 as std::ffi::c_int;
pub const FSE_MAX_TABLELOG: std::ffi::c_int = FSE_MAX_MEMORY_USAGE - 2 as std::ffi::c_int;
pub const FSE_isError: fn(size_t) -> std::ffi::c_uint = ERR_isError;
unsafe extern "C" fn FSE_buildDTable_internal(
    mut dt: &mut DTable,
    mut normalizedCounter: &[std::ffi::c_short; 256],
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let mut symbolNext = workSpace as *mut u16;
    let mut spread = symbolNext
        .offset(maxSymbolValue as isize)
        .offset(1 as std::ffi::c_int as isize) as *mut u8;
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
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as u16;
    DTableH.fastMode = 1 as std::ffi::c_int as u16;
    let largeLimit = ((1 as std::ffi::c_int)
        << tableLog.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint))
        as i16;
    let mut s: u32 = 0;
    s = 0 as std::ffi::c_int as u32;
    while s < maxSV1 {
        if normalizedCounter[s as usize] as std::ffi::c_int == -(1 as std::ffi::c_int) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            dt.elements[fresh0 as usize].symbol = s as u8;
            *symbolNext.offset(s as isize) = 1 as std::ffi::c_int as u16;
        } else {
            if normalizedCounter[s as usize] as std::ffi::c_int >= largeLimit as std::ffi::c_int {
                DTableH.fastMode = 0 as std::ffi::c_int as u16;
            }
            *symbolNext.offset(s as isize) = normalizedCounter[s as usize] as u16;
        }
        s = s.wrapping_add(1);
    }
    dt.header = DTableH;
    if highThreshold == tableSize.wrapping_sub(1 as std::ffi::c_int as u32) {
        let tableMask = tableSize.wrapping_sub(1 as std::ffi::c_int as u32) as size_t;
        let step = (tableSize >> 1 as std::ffi::c_int)
            .wrapping_add(tableSize >> 3 as std::ffi::c_int)
            .wrapping_add(3 as std::ffi::c_int as u32) as size_t;
        let add = 0x101010101010101 as std::ffi::c_ulonglong as u64;
        let mut pos = 0 as std::ffi::c_int as size_t;
        let mut sv = 0 as std::ffi::c_int as u64;
        let mut s_0: u32 = 0;
        s_0 = 0 as std::ffi::c_int as u32;
        while s_0 < maxSV1 {
            let mut i: std::ffi::c_int = 0;
            let n = normalizedCounter[s_0 as usize] as std::ffi::c_int;
            MEM_write64(spread.offset(pos as isize) as *mut std::ffi::c_void, sv);
            i = 8 as std::ffi::c_int;
            while i < n {
                MEM_write64(
                    spread.offset(pos as isize).offset(i as isize) as *mut std::ffi::c_void,
                    sv,
                );
                i += 8 as std::ffi::c_int;
            }
            pos = pos.wrapping_add(n as size_t);
            s_0 = s_0.wrapping_add(1);
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
                dt.elements[uPosition as usize].symbol =
                    *spread.offset(s_1.wrapping_add(u) as isize);
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll * step) & tableMask;
            s_1 = s_1.wrapping_add(unroll);
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1 as std::ffi::c_int as u32);
        let step_0 = (tableSize >> 1 as std::ffi::c_int)
            .wrapping_add(tableSize >> 3 as std::ffi::c_int)
            .wrapping_add(3 as std::ffi::c_int as u32);
        let mut s_2: u32 = 0;
        let mut position_0 = 0 as std::ffi::c_int as u32;
        s_2 = 0 as std::ffi::c_int as u32;
        while s_2 < maxSV1 {
            let mut i_0: std::ffi::c_int = 0;
            i_0 = 0 as std::ffi::c_int;
            while i_0 < normalizedCounter[s_2 as usize] as std::ffi::c_int {
                dt.elements[position_0 as usize].symbol = s_2 as u8;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
                i_0 += 1;
            }
            s_2 = s_2.wrapping_add(1);
        }
        if position_0 != 0 as std::ffi::c_int as u32 {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
    }
    let mut u_0: u32 = 0;
    u_0 = 0 as std::ffi::c_int as u32;
    while u_0 < tableSize {
        let symbol = (dt.elements[u_0 as usize]).symbol;
        let fresh1 = &mut (*symbolNext.offset(symbol as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as u32;
        (dt.elements[u_0 as usize]).nbBits = tableLog.wrapping_sub(nextState.ilog2()) as u8;
        (dt.elements[u_0 as usize]).newState = (nextState
            << (dt.elements[u_0 as usize]).nbBits as std::ffi::c_int)
            .wrapping_sub(tableSize) as u16;
        u_0 = u_0.wrapping_add(1);
    }
    0 as std::ffi::c_int as size_t
}

#[inline(always)]
unsafe fn FSE_decompress_usingDTable_generic(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: &[u8],
    mut dt: &DTable,
    fast: bool,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-(3 as std::ffi::c_int as isize));
    let mut bitD = match BIT_DStream_t::new(cSrc) {
        Err(e) => return e,
        Ok(bitD) => bitD,
    };

    let mut state1 = FSE_DState_t::new(&mut bitD, dt);
    let mut state2 = FSE_DState_t::new(&mut bitD, dt);
    if BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
        == BIT_DStream_overflow as std::ffi::c_int as std::ffi::c_uint
    {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    while (BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
        == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int
        & (op < olimit) as std::ffi::c_int
        != 0
    {
        *op.offset(0 as std::ffi::c_int as isize) = (if fast {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as u8;
        if (FSE_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(1 as std::ffi::c_int as isize) = (if fast {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as u8;
        if (FSE_MAX_TABLELOG * 4 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
            && BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
                > BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        {
            op = op.offset(2 as std::ffi::c_int as isize);
            break;
        }
        *op.offset(2 as std::ffi::c_int as isize) = (if fast {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as u8;
        if (FSE_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(3 as std::ffi::c_int as isize) = (if fast {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as u8;
        op = op.offset(4 as std::ffi::c_int as isize);
    }
    loop {
        if op > omax.offset(-(2 as std::ffi::c_int as isize)) {
            return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
        }
        let fresh3 = op;
        op = op.offset(1);
        *fresh3 = (if fast {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as u8;
        if BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
            == BIT_DStream_overflow as std::ffi::c_int as std::ffi::c_uint
        {
            let fresh4 = op;
            op = op.offset(1);
            *fresh4 = (if fast {
                FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
            } else {
                FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
            }) as u8;
            break;
        } else {
            if op > omax.offset(-(2 as std::ffi::c_int as isize)) {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            let fresh5 = op;
            op = op.offset(1);
            *fresh5 = (if fast {
                FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
            } else {
                FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
            }) as u8;
            if BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
                != BIT_DStream_overflow as std::ffi::c_int as std::ffi::c_uint
            {
                continue;
            }
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = (if fast {
                FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
            } else {
                FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
            }) as u8;
            break;
        }
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
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
    let mut workSpace = (workspace as *mut Workspace).cast();

    let mut dstCapacity = dst.len() as size_t;
    let mut dst = dst.as_mut_ptr().cast();

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
    workSpace = (workSpace as *mut u8)
        .offset(::core::mem::size_of::<FSE_DecompressWksp>() as std::ffi::c_ulong as isize)
        .offset(
            ((1 as std::ffi::c_int + ((1 as std::ffi::c_int) << tableLog)) as std::ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<FSE_DTable>() as std::ffi::c_ulong)
                as isize,
        ) as *mut std::ffi::c_void;
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
        workSpace,
        wkspSize,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }

    FSE_decompress_usingDTable_generic(
        dst,
        dstCapacity,
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
