use ::libc;

type size_t = std::ffi::c_ulong;
type __uint8_t = std::ffi::c_uchar;
type __int16_t = std::ffi::c_short;
type __uint16_t = std::ffi::c_ushort;
type __uint32_t = std::ffi::c_uint;
type __uint64_t = std::ffi::c_ulong;
type int16_t = __int16_t;
type uint8_t = __uint8_t;
type uint16_t = __uint16_t;
type uint32_t = __uint32_t;
type uint64_t = __uint64_t;
type BYTE = uint8_t;
type U16 = uint16_t;
type S16 = int16_t;
type U32 = uint32_t;
type U64 = uint64_t;
type unalign32 = U32;
type unalign64 = U64;
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
type FSE_DTable = std::ffi::c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
struct FSE_decode_t {
    newState: std::ffi::c_ushort,
    symbol: std::ffi::c_uchar,
    nbBits: std::ffi::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct FSE_DTableHeader {
    tableLog: U16,
    fastMode: U16,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct FSE_DecompressWksp {
    ncount: [std::ffi::c_short; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
struct FSE_DState_t {
    state: size_t,
    table: *const std::ffi::c_void,
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
use crate::{lib::common::entropy_common::FSE_readNCount_bmi2, MEM_readLEST, MEM_write64};
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
#[inline]
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 as std::ffi::c_int as size_t {
        libc::memset(
            bitD as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            ::core::mem::size_of::<BIT_DStream_t>() as std::ffi::c_ulong as libc::size_t,
        );
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    (*bitD).start = srcBuffer as *const std::ffi::c_char;
    (*bitD).limitPtr = ((*bitD).start)
        .offset(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize);
    if srcSize >= ::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong {
        (*bitD).ptr = (srcBuffer as *const std::ffi::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        let lastByte = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
        (*bitD).bitsConsumed = if lastByte as std::ffi::c_int != 0 {
            (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_highbit32(lastByte as U32))
        } else {
            0 as std::ffi::c_int as std::ffi::c_uint
        };
        if lastByte as std::ffi::c_int == 0 as std::ffi::c_int {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const BYTE) as BitContainerType;
        let mut current_block_32: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                    (*(srcBuffer as *const BYTE).offset(6 as std::ffi::c_int as isize)
                        as BitContainerType)
                        << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                            .wrapping_sub(16 as std::ffi::c_int as std::ffi::c_ulong),
                );
                current_block_32 = 1526876707632491634;
            }
            6 => {
                current_block_32 = 1526876707632491634;
            }
            5 => {
                current_block_32 = 3132044029094175486;
            }
            4 => {
                current_block_32 = 5336772990438301456;
            }
            3 => {
                current_block_32 = 11143942153966811798;
            }
            2 => {
                current_block_32 = 2397150252174490634;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        if current_block_32 == 1526876707632491634 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(5 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                        .wrapping_sub(24 as std::ffi::c_int as std::ffi::c_ulong),
            );
            current_block_32 = 3132044029094175486;
        }
        if current_block_32 == 3132044029094175486 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(4 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                        .wrapping_sub(32 as std::ffi::c_int as std::ffi::c_ulong),
            );
            current_block_32 = 5336772990438301456;
        }
        if current_block_32 == 5336772990438301456 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(3 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << 24 as std::ffi::c_int,
            );
            current_block_32 = 11143942153966811798;
        }
        if current_block_32 == 11143942153966811798 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(2 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << 16 as std::ffi::c_int,
            );
            current_block_32 = 2397150252174490634;
        }
        if current_block_32 == 2397150252174490634 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(1 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << 8 as std::ffi::c_int,
            );
        }
        let lastByte_0 = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
        (*bitD).bitsConsumed = if lastByte_0 as std::ffi::c_int != 0 {
            (8 as std::ffi::c_int as std::ffi::c_uint)
                .wrapping_sub(ZSTD_highbit32(lastByte_0 as U32))
        } else {
            0 as std::ffi::c_int as std::ffi::c_uint
        };
        if lastByte_0 as std::ffi::c_int == 0 as std::ffi::c_int {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(
            (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong).wrapping_sub(srcSize)
                as U32
                * 8 as std::ffi::c_int as U32,
        );
    }
    srcSize
}
#[inline(always)]
unsafe extern "C" fn BIT_getMiddleBits(
    mut bitContainer: BitContainerType,
    start: U32,
    nbBits: U32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as U32;
    bitContainer >> (start & regMask)
        & ((1 as std::ffi::c_int as U64) << nbBits).wrapping_sub(1 as std::ffi::c_int as U64)
}
#[inline(always)]
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: U32,
) -> BitContainerType {
    BIT_getMiddleBits(
        (*bitD).bitContainer,
        (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
            .wrapping_sub((*bitD).bitsConsumed as std::ffi::c_ulong)
            .wrapping_sub(nbBits as std::ffi::c_ulong) as U32,
        nbBits,
    )
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: U32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as U32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask
            .wrapping_add(1 as std::ffi::c_int as U32)
            .wrapping_sub(nbBits)
            & regMask)
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: U32) {
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
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as U32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed =
        ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8 as std::ffi::c_int as U32);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    result
}
#[inline]
unsafe extern "C" fn FSE_initDState(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const FSE_DTable,
) {
    let mut ptr = dt as *const std::ffi::c_void;
    let DTableH = ptr as *const FSE_DTableHeader;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog as std::ffi::c_uint);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
}
#[inline]
unsafe extern "C" fn FSE_decodeSymbol(
    mut DStatePtr: *mut FSE_DState_t,
    mut bitD: *mut BIT_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSE_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
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
    let nbBits = DInfo.nbBits as U32;
    let symbol = DInfo.symbol;
    let lowBits = BIT_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
pub const FSE_MAX_MEMORY_USAGE: std::ffi::c_int = 14 as std::ffi::c_int;
pub const FSE_MAX_SYMBOL_VALUE: std::ffi::c_int = 255 as std::ffi::c_int;
pub const FSE_MAX_TABLELOG: std::ffi::c_int = FSE_MAX_MEMORY_USAGE - 2 as std::ffi::c_int;
pub const FSE_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
unsafe extern "C" fn FSE_buildDTable_internal(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let tdPtr = dt.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let tableDecode = tdPtr as *mut FSE_decode_t;
    let mut symbolNext = workSpace as *mut U16;
    let mut spread = symbolNext
        .offset(maxSymbolValue as isize)
        .offset(1 as std::ffi::c_int as isize) as *mut BYTE;
    let maxSV1 = maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let tableSize = ((1 as std::ffi::c_int) << tableLog) as U32;
    let mut highThreshold = tableSize.wrapping_sub(1 as std::ffi::c_int as U32);
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
    DTableH.tableLog = tableLog as U16;
    DTableH.fastMode = 1 as std::ffi::c_int as U16;
    let largeLimit = ((1 as std::ffi::c_int)
        << tableLog.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint))
        as S16;
    let mut s: U32 = 0;
    s = 0 as std::ffi::c_int as U32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as std::ffi::c_int == -(1 as std::ffi::c_int) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).symbol = s as BYTE;
            *symbolNext.offset(s as isize) = 1 as std::ffi::c_int as U16;
        } else {
            if *normalizedCounter.offset(s as isize) as std::ffi::c_int
                >= largeLimit as std::ffi::c_int
            {
                DTableH.fastMode = 0 as std::ffi::c_int as U16;
            }
            *symbolNext.offset(s as isize) = *normalizedCounter.offset(s as isize) as U16;
        }
        s = s.wrapping_add(1);
        s;
    }
    libc::memcpy(
        dt as *mut std::ffi::c_void,
        &mut DTableH as *mut FSE_DTableHeader as *const std::ffi::c_void,
        ::core::mem::size_of::<FSE_DTableHeader>() as std::ffi::c_ulong as libc::size_t,
    );
    if highThreshold == tableSize.wrapping_sub(1 as std::ffi::c_int as U32) {
        let tableMask = tableSize.wrapping_sub(1 as std::ffi::c_int as U32) as size_t;
        let step = (tableSize >> 1 as std::ffi::c_int)
            .wrapping_add(tableSize >> 3 as std::ffi::c_int)
            .wrapping_add(3 as std::ffi::c_int as U32) as size_t;
        let add = 0x101010101010101 as std::ffi::c_ulonglong as U64;
        let mut pos = 0 as std::ffi::c_int as size_t;
        let mut sv = 0 as std::ffi::c_int as U64;
        let mut s_0: U32 = 0;
        s_0 = 0 as std::ffi::c_int as U32;
        while s_0 < maxSV1 {
            let mut i: std::ffi::c_int = 0;
            let n = *normalizedCounter.offset(s_0 as isize) as std::ffi::c_int;
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
            s_0;
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
                (*tableDecode.offset(uPosition as isize)).symbol =
                    *spread.offset(s_1.wrapping_add(u) as isize);
                u = u.wrapping_add(1);
                u;
            }
            position = position.wrapping_add(unroll * step) & tableMask;
            s_1 = s_1.wrapping_add(unroll);
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1 as std::ffi::c_int as U32);
        let step_0 = (tableSize >> 1 as std::ffi::c_int)
            .wrapping_add(tableSize >> 3 as std::ffi::c_int)
            .wrapping_add(3 as std::ffi::c_int as U32);
        let mut s_2: U32 = 0;
        let mut position_0 = 0 as std::ffi::c_int as U32;
        s_2 = 0 as std::ffi::c_int as U32;
        while s_2 < maxSV1 {
            let mut i_0: std::ffi::c_int = 0;
            i_0 = 0 as std::ffi::c_int;
            while i_0 < *normalizedCounter.offset(s_2 as isize) as std::ffi::c_int {
                (*tableDecode.offset(position_0 as isize)).symbol = s_2 as BYTE;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
                i_0 += 1;
                i_0;
            }
            s_2 = s_2.wrapping_add(1);
            s_2;
        }
        if position_0 != 0 as std::ffi::c_int as U32 {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
    }
    let mut u_0: U32 = 0;
    u_0 = 0 as std::ffi::c_int as U32;
    while u_0 < tableSize {
        let symbol = (*tableDecode.offset(u_0 as isize)).symbol;
        let fresh1 = &mut (*symbolNext.offset(symbol as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as U32;
        (*tableDecode.offset(u_0 as isize)).nbBits =
            tableLog.wrapping_sub(ZSTD_highbit32(nextState)) as BYTE;
        (*tableDecode.offset(u_0 as isize)).newState = (nextState
            << (*tableDecode.offset(u_0 as isize)).nbBits as std::ffi::c_int)
            .wrapping_sub(tableSize) as U16;
        u_0 = u_0.wrapping_add(1);
        u_0;
    }
    0 as std::ffi::c_int as size_t
}

unsafe fn FSE_buildDTable_wksp(
    mut dt: *mut FSE_DTable,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    FSE_buildDTable_internal(
        dt,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        workSpace,
        wkspSize,
    )
}

#[inline(always)]
unsafe extern "C" fn FSE_decompress_usingDTable_generic(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSE_DTable,
    fast: std::ffi::c_uint,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-(3 as std::ffi::c_int as isize));
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut state1 = FSE_DState_t {
        state: 0,
        table: std::ptr::null::<std::ffi::c_void>(),
    };
    let mut state2 = FSE_DState_t {
        state: 0,
        table: std::ptr::null::<std::ffi::c_void>(),
    };
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    FSE_initDState(&mut state1, &mut bitD, dt);
    FSE_initDState(&mut state2, &mut bitD, dt);
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
        *op.offset(0 as std::ffi::c_int as isize) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(1 as std::ffi::c_int as isize) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 4 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
            && BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
                > BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        {
            op = op.offset(2 as std::ffi::c_int as isize);
            break;
        }
        *op.offset(2 as std::ffi::c_int as isize) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if (FSE_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            BIT_reloadDStream(&mut bitD);
        }
        *op.offset(3 as std::ffi::c_int as isize) = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        op = op.offset(4 as std::ffi::c_int as isize);
    }
    loop {
        if op > omax.offset(-(2 as std::ffi::c_int as isize)) {
            return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
        }
        let fresh3 = op;
        op = op.offset(1);
        *fresh3 = (if fast != 0 {
            FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
            == BIT_DStream_overflow as std::ffi::c_int as std::ffi::c_uint
        {
            let fresh4 = op;
            op = op.offset(1);
            *fresh4 = (if fast != 0 {
                FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
            } else {
                FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
            }) as BYTE;
            break;
        } else {
            if op > omax.offset(-(2 as std::ffi::c_int as isize)) {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            let fresh5 = op;
            op = op.offset(1);
            *fresh5 = (if fast != 0 {
                FSE_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
            } else {
                FSE_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
            }) as BYTE;
            if BIT_reloadDStream(&mut bitD) as std::ffi::c_uint
                != BIT_DStream_overflow as std::ffi::c_int as std::ffi::c_uint
            {
                continue;
            }
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = (if fast != 0 {
                FSE_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
            } else {
                FSE_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
            }) as BYTE;
            break;
        }
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
#[inline(always)]
unsafe extern "C" fn FSE_decompress_wksp_body(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut maxLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut bmi2: std::ffi::c_int,
) -> size_t {
    let istart = cSrc as *const BYTE;
    let mut ip = istart;
    let mut tableLog: std::ffi::c_uint = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as std::ffi::c_uint;
    let wksp = workSpace as *mut FSE_DecompressWksp;
    let dtablePos = (::core::mem::size_of::<FSE_DecompressWksp>() as std::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<FSE_DTable>() as std::ffi::c_ulong);
    let dtable = (workSpace as *mut FSE_DTable).offset(dtablePos as isize);
    if wkspSize < ::core::mem::size_of::<FSE_DecompressWksp>() as std::ffi::c_ulong {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    let NCountLength = FSE_readNCount_bmi2(
        ((*wksp).ncount).as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const std::ffi::c_void,
        cSrcSize,
        bmi2,
    );
    if ERR_isError(NCountLength) != 0 {
        return NCountLength;
    }
    if tableLog > maxLog {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(NCountLength as isize);
    cSrcSize = cSrcSize.wrapping_sub(NCountLength);
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
    workSpace = (workSpace as *mut BYTE)
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
        dtable,
        ((*wksp).ncount).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        workSpace,
        wkspSize,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let mut ptr = dtable as *const std::ffi::c_void;
    let mut DTableH = ptr as *const FSE_DTableHeader;
    let fastMode = (*DTableH).fastMode as U32;
    if fastMode != 0 {
        return FSE_decompress_usingDTable_generic(
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            cSrcSize,
            dtable,
            1 as std::ffi::c_int as std::ffi::c_uint,
        );
    }
    FSE_decompress_usingDTable_generic(
        dst,
        dstCapacity,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dtable,
        0 as std::ffi::c_int as std::ffi::c_uint,
    )
}
unsafe extern "C" fn FSE_decompress_wksp_body_default(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut maxLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    FSE_decompress_wksp_body(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
        0 as std::ffi::c_int,
    )
}
unsafe extern "C" fn FSE_decompress_wksp_body_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut maxLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    FSE_decompress_wksp_body(
        dst,
        dstCapacity,
        cSrc,
        cSrcSize,
        maxLog,
        workSpace,
        wkspSize,
        1 as std::ffi::c_int,
    )
}

pub unsafe fn FSE_decompress_wksp_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut maxLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut bmi2: bool,
) -> size_t {
    if bmi2 {
        FSE_decompress_wksp_body_bmi2(
            dst,
            dstCapacity,
            cSrc,
            cSrcSize,
            maxLog,
            workSpace,
            wkspSize,
        )
    } else {
        FSE_decompress_wksp_body_default(
            dst,
            dstCapacity,
            cSrc,
            cSrcSize,
            maxLog,
            workSpace,
            wkspSize,
        )
    }
}
