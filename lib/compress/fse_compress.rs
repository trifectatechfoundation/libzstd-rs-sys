pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
pub type C2RustUnnamed = std::ffi::c_uint;
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed = 74;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed = 72;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed = 22;
pub const ZSTD_error_corruption_detected: C2RustUnnamed = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed = 12;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed = 10;
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
pub const ZSTD_error_no_error: C2RustUnnamed = 0;
pub type BitContainerType = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_CStream_t {
    pub bitContainer: BitContainerType,
    pub bitPos: std::ffi::c_uint,
    pub startPtr: *mut std::ffi::c_char,
    pub ptr: *mut std::ffi::c_char,
    pub endPtr: *mut std::ffi::c_char,
}
pub type FSE_CTable = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const std::ffi::c_void,
    pub symbolTT: *const std::ffi::c_void,
    pub stateLog: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: std::ffi::c_int,
    pub deltaNbBits: u32,
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    1 as std::ffi::c_int as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_read16(mut ptr: *const std::ffi::c_void) -> u16 {
    *(ptr as *const unalign16)
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut std::ffi::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_write64(mut memPtr: *mut std::ffi::c_void, mut value: u64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    in_0.swap_bytes()
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: u64) -> u64 {
    in_0.swap_bytes()
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut std::ffi::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut std::ffi::c_void, mut val64: u64) {
    if MEM_isLittleEndian() != 0 {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut std::ffi::c_void, mut val: size_t) {
    if MEM_32bits() != 0 {
        MEM_writeLE32(memPtr, val as u32);
    } else {
        MEM_writeLE64(memPtr, val);
    };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
static mut BIT_mask: [std::ffi::c_uint; 32] = [
    0 as std::ffi::c_int as std::ffi::c_uint,
    1 as std::ffi::c_int as std::ffi::c_uint,
    3 as std::ffi::c_int as std::ffi::c_uint,
    7 as std::ffi::c_int as std::ffi::c_uint,
    0xf as std::ffi::c_int as std::ffi::c_uint,
    0x1f as std::ffi::c_int as std::ffi::c_uint,
    0x3f as std::ffi::c_int as std::ffi::c_uint,
    0x7f as std::ffi::c_int as std::ffi::c_uint,
    0xff as std::ffi::c_int as std::ffi::c_uint,
    0x1ff as std::ffi::c_int as std::ffi::c_uint,
    0x3ff as std::ffi::c_int as std::ffi::c_uint,
    0x7ff as std::ffi::c_int as std::ffi::c_uint,
    0xfff as std::ffi::c_int as std::ffi::c_uint,
    0x1fff as std::ffi::c_int as std::ffi::c_uint,
    0x3fff as std::ffi::c_int as std::ffi::c_uint,
    0x7fff as std::ffi::c_int as std::ffi::c_uint,
    0xffff as std::ffi::c_int as std::ffi::c_uint,
    0x1ffff as std::ffi::c_int as std::ffi::c_uint,
    0x3ffff as std::ffi::c_int as std::ffi::c_uint,
    0x7ffff as std::ffi::c_int as std::ffi::c_uint,
    0xfffff as std::ffi::c_int as std::ffi::c_uint,
    0x1fffff as std::ffi::c_int as std::ffi::c_uint,
    0x3fffff as std::ffi::c_int as std::ffi::c_uint,
    0x7fffff as std::ffi::c_int as std::ffi::c_uint,
    0xffffff as std::ffi::c_int as std::ffi::c_uint,
    0x1ffffff as std::ffi::c_int as std::ffi::c_uint,
    0x3ffffff as std::ffi::c_int as std::ffi::c_uint,
    0x7ffffff as std::ffi::c_int as std::ffi::c_uint,
    0xfffffff as std::ffi::c_int as std::ffi::c_uint,
    0x1fffffff as std::ffi::c_int as std::ffi::c_uint,
    0x3fffffff as std::ffi::c_int as std::ffi::c_uint,
    0x7fffffff as std::ffi::c_int as std::ffi::c_uint,
];
#[inline]
unsafe extern "C" fn BIT_initCStream(
    mut bitC: *mut BIT_CStream_t,
    mut startPtr: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    (*bitC).bitContainer = 0 as std::ffi::c_int as BitContainerType;
    (*bitC).bitPos = 0 as std::ffi::c_int as std::ffi::c_uint;
    (*bitC).startPtr = startPtr as *mut std::ffi::c_char;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr)
        .offset(dstCapacity as isize)
        .offset(-(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    0 as std::ffi::c_int as size_t
}
#[inline(always)]
unsafe extern "C" fn BIT_getLowerBits(
    mut bitContainer: BitContainerType,
    nbBits: u32,
) -> BitContainerType {
    bitContainer & *BIT_mask.as_ptr().offset(nbBits as isize) as BitContainerType
}
#[inline]
unsafe extern "C" fn BIT_addBits(
    mut bitC: *mut BIT_CStream_t,
    mut value: BitContainerType,
    mut nbBits: std::ffi::c_uint,
) {
    (*bitC).bitContainer |= BIT_getLowerBits(value, nbBits) << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}
#[inline]
unsafe extern "C" fn BIT_addBitsFast(
    mut bitC: *mut BIT_CStream_t,
    mut value: BitContainerType,
    mut nbBits: std::ffi::c_uint,
) {
    (*bitC).bitContainer |= value << (*bitC).bitPos;
    (*bitC).bitPos = ((*bitC).bitPos).wrapping_add(nbBits);
}
#[inline]
unsafe extern "C" fn BIT_flushBitsFast(mut bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3 as std::ffi::c_int) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut std::ffi::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    (*bitC).bitPos &= 7 as std::ffi::c_int as std::ffi::c_uint;
    (*bitC).bitContainer >>= nbBytes * 8 as std::ffi::c_int as size_t;
}
#[inline]
unsafe extern "C" fn BIT_flushBits(mut bitC: *mut BIT_CStream_t) {
    let nbBytes = ((*bitC).bitPos >> 3 as std::ffi::c_int) as size_t;
    MEM_writeLEST((*bitC).ptr as *mut std::ffi::c_void, (*bitC).bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    if (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
    (*bitC).bitPos &= 7 as std::ffi::c_int as std::ffi::c_uint;
    (*bitC).bitContainer >>= nbBytes * 8 as std::ffi::c_int as size_t;
}
#[inline]
unsafe extern "C" fn BIT_closeCStream(mut bitC: *mut BIT_CStream_t) -> size_t {
    BIT_addBitsFast(
        bitC,
        1 as std::ffi::c_int as BitContainerType,
        1 as std::ffi::c_int as std::ffi::c_uint,
    );
    BIT_flushBits(bitC);
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0 as std::ffi::c_int as size_t;
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as std::ffi::c_long as size_t).wrapping_add(
        ((*bitC).bitPos > 0 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int as size_t,
    )
}
pub const FSE_NCOUNTBOUND: std::ffi::c_int = 512 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn FSE_initCState(mut statePtr: *mut FSE_CState_t, mut ct: *const FSE_CTable) {
    let mut ptr = ct as *const std::ffi::c_void;
    let mut u16ptr = ptr as *const u16;
    let tableLog = MEM_read16(ptr) as u32;
    (*statePtr).value = (1 as std::ffi::c_int as ptrdiff_t) << tableLog;
    (*statePtr).stateTable =
        u16ptr.offset(2 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    (*statePtr).symbolTT = ct.offset(1 as std::ffi::c_int as isize).offset(
        (if tableLog != 0 {
            (1 as std::ffi::c_int) << tableLog.wrapping_sub(1 as std::ffi::c_int as u32)
        } else {
            1 as std::ffi::c_int
        }) as isize,
    ) as *const std::ffi::c_void;
    (*statePtr).stateLog = tableLog;
}
#[inline]
unsafe extern "C" fn FSE_initCState2(
    mut statePtr: *mut FSE_CState_t,
    mut ct: *const FSE_CTable,
    mut symbol: u32,
) {
    FSE_initCState(statePtr, ct);
    let symbolTT =
        *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let mut stateTable = (*statePtr).stateTable as *const u16;
    let mut nbBitsOut = (symbolTT.deltaNbBits)
        .wrapping_add(((1 as std::ffi::c_int) << 15 as std::ffi::c_int) as u32)
        >> 16 as std::ffi::c_int;
    (*statePtr).value =
        (nbBitsOut << 16 as std::ffi::c_int).wrapping_sub(symbolTT.deltaNbBits) as ptrdiff_t;
    (*statePtr).value = *stateTable
        .offset((((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t) as isize)
        as ptrdiff_t;
}
#[inline]
unsafe extern "C" fn FSE_encodeSymbol(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *mut FSE_CState_t,
    mut symbol: std::ffi::c_uint,
) {
    let symbolTT =
        *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable = (*statePtr).stateTable as *const u16;
    let nbBitsOut =
        (((*statePtr).value + symbolTT.deltaNbBits as ptrdiff_t) >> 16 as std::ffi::c_int) as u32;
    BIT_addBits(bitC, (*statePtr).value as BitContainerType, nbBitsOut);
    (*statePtr).value = *stateTable
        .offset((((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t) as isize)
        as ptrdiff_t;
}
#[inline]
unsafe extern "C" fn FSE_flushCState(
    mut bitC: *mut BIT_CStream_t,
    mut statePtr: *const FSE_CState_t,
) {
    BIT_addBits(
        bitC,
        (*statePtr).value as BitContainerType,
        (*statePtr).stateLog,
    );
    BIT_flushBits(bitC);
}
pub const FSE_MAX_MEMORY_USAGE: std::ffi::c_int = 14 as std::ffi::c_int;
pub const FSE_DEFAULT_MEMORY_USAGE: std::ffi::c_int = 13 as std::ffi::c_int;
pub const FSE_MAX_TABLELOG: std::ffi::c_int = FSE_MAX_MEMORY_USAGE - 2 as std::ffi::c_int;
pub const FSE_DEFAULT_TABLELOG: std::ffi::c_int = FSE_DEFAULT_MEMORY_USAGE - 2 as std::ffi::c_int;
pub const FSE_MIN_TABLELOG: std::ffi::c_int = 5 as std::ffi::c_int;
pub const FSE_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_wksp(
    mut ct: *mut FSE_CTable,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let tableSize = ((1 as std::ffi::c_int) << tableLog) as u32;
    let tableMask = tableSize.wrapping_sub(1 as std::ffi::c_int as u32);
    let ptr = ct as *mut std::ffi::c_void;
    let tableU16 = (ptr as *mut u16).offset(2 as std::ffi::c_int as isize);
    let FSCT = (ptr as *mut u32)
        .offset(1 as std::ffi::c_int as isize)
        .offset(
            (if tableLog != 0 {
                tableSize >> 1 as std::ffi::c_int
            } else {
                1 as std::ffi::c_int as u32
            }) as isize,
        ) as *mut std::ffi::c_void;
    let symbolTT = FSCT as *mut FSE_symbolCompressionTransform;
    let step = (tableSize >> 1 as std::ffi::c_int)
        .wrapping_add(tableSize >> 3 as std::ffi::c_int)
        .wrapping_add(3 as std::ffi::c_int as u32);
    let maxSV1 = maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let mut cumul = workSpace as *mut u16;
    let tableSymbol =
        cumul.offset(maxSV1.wrapping_add(1 as std::ffi::c_int as u32) as isize) as *mut u8;
    let mut highThreshold = tableSize.wrapping_sub(1 as std::ffi::c_int as u32);
    if (::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong as std::ffi::c_ulonglong)
        .wrapping_mul(
            (maxSymbolValue.wrapping_add(2 as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_ulonglong)
                .wrapping_add((1 as std::ffi::c_ulonglong) << tableLog)
                .wrapping_div(2 as std::ffi::c_int as std::ffi::c_ulonglong)
                .wrapping_add(
                    (::core::mem::size_of::<u64>() as std::ffi::c_ulong)
                        .wrapping_div(::core::mem::size_of::<u32>() as std::ffi::c_ulong)
                        as std::ffi::c_ulonglong,
                ),
        )
        > wkspSize as std::ffi::c_ulonglong
    {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    *tableU16.offset(-(2 as std::ffi::c_int) as isize) = tableLog as u16;
    *tableU16.offset(-(1 as std::ffi::c_int) as isize) = maxSymbolValue as u16;
    let mut u: u32 = 0;
    *cumul.offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as u16;
    u = 1 as std::ffi::c_int as u32;
    while u <= maxSV1 {
        if *normalizedCounter.offset(u.wrapping_sub(1 as std::ffi::c_int as u32) as isize)
            as std::ffi::c_int
            == -(1 as std::ffi::c_int)
        {
            *cumul.offset(u as isize) = (*cumul
                .offset(u.wrapping_sub(1 as std::ffi::c_int as u32) as isize)
                as std::ffi::c_int
                + 1 as std::ffi::c_int) as u16;
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            *tableSymbol.offset(fresh0 as isize) =
                u.wrapping_sub(1 as std::ffi::c_int as u32) as u8;
        } else {
            *cumul.offset(u as isize) = (*cumul
                .offset(u.wrapping_sub(1 as std::ffi::c_int as u32) as isize)
                as std::ffi::c_int
                + *normalizedCounter.offset(u.wrapping_sub(1 as std::ffi::c_int as u32) as isize)
                    as u16 as std::ffi::c_int) as u16;
        }
        u = u.wrapping_add(1);
        u;
    }
    *cumul.offset(maxSV1 as isize) = tableSize.wrapping_add(1 as std::ffi::c_int as u32) as u16;
    if highThreshold == tableSize.wrapping_sub(1 as std::ffi::c_int as u32) {
        let spread = tableSymbol.offset(tableSize as isize);
        let add = 0x101010101010101 as std::ffi::c_ulonglong as u64;
        let mut pos = 0 as std::ffi::c_int as size_t;
        let mut sv = 0 as std::ffi::c_int as u64;
        let mut s: u32 = 0;
        s = 0 as std::ffi::c_int as u32;
        while s < maxSV1 {
            let mut i: std::ffi::c_int = 0;
            let n = *normalizedCounter.offset(s as isize) as std::ffi::c_int;
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
            s = s.wrapping_add(1);
            s;
            sv = sv.wrapping_add(add);
        }
        let mut position = 0 as std::ffi::c_int as size_t;
        let mut s_0: size_t = 0;
        let unroll = 2 as std::ffi::c_int as size_t;
        s_0 = 0 as std::ffi::c_int as size_t;
        while s_0 < tableSize as size_t {
            let mut u_0: size_t = 0;
            u_0 = 0 as std::ffi::c_int as size_t;
            while u_0 < unroll {
                let uPosition = position.wrapping_add(u_0 * step as size_t) & tableMask as size_t;
                *tableSymbol.offset(uPosition as isize) =
                    *spread.offset(s_0.wrapping_add(u_0) as isize);
                u_0 = u_0.wrapping_add(1);
                u_0;
            }
            position = position.wrapping_add(unroll * step as size_t) & tableMask as size_t;
            s_0 = s_0.wrapping_add(unroll);
        }
    } else {
        let mut position_0 = 0 as std::ffi::c_int as u32;
        let mut symbol: u32 = 0;
        symbol = 0 as std::ffi::c_int as u32;
        while symbol < maxSV1 {
            let mut nbOccurrences: std::ffi::c_int = 0;
            let freq = *normalizedCounter.offset(symbol as isize) as std::ffi::c_int;
            nbOccurrences = 0 as std::ffi::c_int;
            while nbOccurrences < freq {
                *tableSymbol.offset(position_0 as isize) = symbol as u8;
                position_0 = position_0.wrapping_add(step) & tableMask;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step) & tableMask;
                }
                nbOccurrences += 1;
                nbOccurrences;
            }
            symbol = symbol.wrapping_add(1);
            symbol;
        }
    }
    let mut u_1: u32 = 0;
    u_1 = 0 as std::ffi::c_int as u32;
    while u_1 < tableSize {
        let mut s_1 = *tableSymbol.offset(u_1 as isize);
        let fresh1 = &mut (*cumul.offset(s_1 as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        *tableU16.offset(fresh2 as isize) = tableSize.wrapping_add(u_1) as u16;
        u_1 = u_1.wrapping_add(1);
        u_1;
    }
    let mut total = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut s_2: std::ffi::c_uint = 0;
    s_2 = 0 as std::ffi::c_int as std::ffi::c_uint;
    while s_2 <= maxSymbolValue {
        match *normalizedCounter.offset(s_2 as isize) as std::ffi::c_int {
            0 => {
                (*symbolTT.offset(s_2 as isize)).deltaNbBits = (tableLog
                    .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
                    << 16 as std::ffi::c_int)
                    .wrapping_sub(((1 as std::ffi::c_int) << tableLog) as std::ffi::c_uint);
            }
            -1 | 1 => {
                (*symbolTT.offset(s_2 as isize)).deltaNbBits = (tableLog << 16 as std::ffi::c_int)
                    .wrapping_sub(((1 as std::ffi::c_int) << tableLog) as std::ffi::c_uint);
                (*symbolTT.offset(s_2 as isize)).deltaFindState =
                    total.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
                total = total.wrapping_add(1);
                total;
            }
            _ => {
                let maxBitsOut = tableLog.wrapping_sub(ZSTD_highbit32(
                    (*normalizedCounter.offset(s_2 as isize) as u32)
                        .wrapping_sub(1 as std::ffi::c_int as u32),
                ));
                let minStatePlus = (*normalizedCounter.offset(s_2 as isize) as u32) << maxBitsOut;
                (*symbolTT.offset(s_2 as isize)).deltaNbBits =
                    (maxBitsOut << 16 as std::ffi::c_int).wrapping_sub(minStatePlus);
                (*symbolTT.offset(s_2 as isize)).deltaFindState = total
                    .wrapping_sub(*normalizedCounter.offset(s_2 as isize) as std::ffi::c_uint)
                    as std::ffi::c_int;
                total =
                    total.wrapping_add(*normalizedCounter.offset(s_2 as isize) as std::ffi::c_uint);
            }
        }
        s_2 = s_2.wrapping_add(1);
        s_2;
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn FSE_NCountWriteBound(
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
) -> size_t {
    let maxHeaderSize = maxSymbolValue
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_mul(tableLog)
        .wrapping_add(4 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_add(2 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_div(8 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_add(2 as std::ffi::c_int as std::ffi::c_uint) as size_t;
    if maxSymbolValue != 0 {
        maxHeaderSize
    } else {
        FSE_NCOUNTBOUND as size_t
    }
}
unsafe extern "C" fn FSE_writeNCount_generic(
    mut header: *mut std::ffi::c_void,
    mut headerBufferSize: size_t,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
    mut writeIsSafe: std::ffi::c_uint,
) -> size_t {
    let ostart = header as *mut u8;
    let mut out = ostart;
    let oend = ostart.offset(headerBufferSize as isize);
    let mut nbBits: std::ffi::c_int = 0;
    let tableSize = (1 as std::ffi::c_int) << tableLog;
    let mut remaining: std::ffi::c_int = 0;
    let mut threshold: std::ffi::c_int = 0;
    let mut bitStream = 0 as std::ffi::c_int as u32;
    let mut bitCount = 0 as std::ffi::c_int;
    let mut symbol = 0 as std::ffi::c_int as std::ffi::c_uint;
    let alphabetSize = maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let mut previousIs0 = 0 as std::ffi::c_int;
    bitStream = (bitStream as std::ffi::c_uint)
        .wrapping_add(tableLog.wrapping_sub(FSE_MIN_TABLELOG as std::ffi::c_uint) << bitCount)
        as u32 as u32;
    bitCount += 4 as std::ffi::c_int;
    remaining = tableSize + 1 as std::ffi::c_int;
    threshold = tableSize;
    nbBits = tableLog as std::ffi::c_int + 1 as std::ffi::c_int;
    while symbol < alphabetSize && remaining > 1 as std::ffi::c_int {
        if previousIs0 != 0 {
            let mut start = symbol;
            while symbol < alphabetSize && *normalizedCounter.offset(symbol as isize) == 0 {
                symbol = symbol.wrapping_add(1);
                symbol;
            }
            if symbol == alphabetSize {
                break;
            }
            while symbol >= start.wrapping_add(24 as std::ffi::c_int as std::ffi::c_uint) {
                start = start.wrapping_add(24 as std::ffi::c_int as std::ffi::c_uint);
                bitStream = (bitStream as std::ffi::c_uint)
                    .wrapping_add((0xffff as std::ffi::c_uint) << bitCount)
                    as u32 as u32;
                if writeIsSafe == 0 && out > oend.offset(-(2 as std::ffi::c_int as isize)) {
                    return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
                }
                *out.offset(0 as std::ffi::c_int as isize) = bitStream as u8;
                *out.offset(1 as std::ffi::c_int as isize) =
                    (bitStream >> 8 as std::ffi::c_int) as u8;
                out = out.offset(2 as std::ffi::c_int as isize);
                bitStream >>= 16 as std::ffi::c_int;
            }
            while symbol >= start.wrapping_add(3 as std::ffi::c_int as std::ffi::c_uint) {
                start = start.wrapping_add(3 as std::ffi::c_int as std::ffi::c_uint);
                bitStream = (bitStream as std::ffi::c_uint)
                    .wrapping_add((3 as std::ffi::c_uint) << bitCount)
                    as u32 as u32;
                bitCount += 2 as std::ffi::c_int;
            }
            bitStream = (bitStream as std::ffi::c_uint)
                .wrapping_add(symbol.wrapping_sub(start) << bitCount) as u32
                as u32;
            bitCount += 2 as std::ffi::c_int;
            if bitCount > 16 as std::ffi::c_int {
                if writeIsSafe == 0 && out > oend.offset(-(2 as std::ffi::c_int as isize)) {
                    return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
                }
                *out.offset(0 as std::ffi::c_int as isize) = bitStream as u8;
                *out.offset(1 as std::ffi::c_int as isize) =
                    (bitStream >> 8 as std::ffi::c_int) as u8;
                out = out.offset(2 as std::ffi::c_int as isize);
                bitStream >>= 16 as std::ffi::c_int;
                bitCount -= 16 as std::ffi::c_int;
            }
        }
        let fresh3 = symbol;
        symbol = symbol.wrapping_add(1);
        let mut count = *normalizedCounter.offset(fresh3 as isize) as std::ffi::c_int;
        let max = 2 as std::ffi::c_int * threshold - 1 as std::ffi::c_int - remaining;
        remaining -= if count < 0 as std::ffi::c_int {
            -count
        } else {
            count
        };
        count += 1;
        count;
        if count >= threshold {
            count += max;
        }
        bitStream = bitStream.wrapping_add((count as u32) << bitCount);
        bitCount += nbBits;
        bitCount -= (count < max) as std::ffi::c_int;
        previousIs0 = (count == 1 as std::ffi::c_int) as std::ffi::c_int;
        if remaining < 1 as std::ffi::c_int {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
        while remaining < threshold {
            nbBits -= 1;
            nbBits;
            threshold >>= 1 as std::ffi::c_int;
        }
        if bitCount > 16 as std::ffi::c_int {
            if writeIsSafe == 0 && out > oend.offset(-(2 as std::ffi::c_int as isize)) {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            *out.offset(0 as std::ffi::c_int as isize) = bitStream as u8;
            *out.offset(1 as std::ffi::c_int as isize) = (bitStream >> 8 as std::ffi::c_int) as u8;
            out = out.offset(2 as std::ffi::c_int as isize);
            bitStream >>= 16 as std::ffi::c_int;
            bitCount -= 16 as std::ffi::c_int;
        }
    }
    if remaining != 1 as std::ffi::c_int {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if writeIsSafe == 0 && out > oend.offset(-(2 as std::ffi::c_int as isize)) {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    *out.offset(0 as std::ffi::c_int as isize) = bitStream as u8;
    *out.offset(1 as std::ffi::c_int as isize) = (bitStream >> 8 as std::ffi::c_int) as u8;
    out = out.offset(((bitCount + 7 as std::ffi::c_int) / 8 as std::ffi::c_int) as isize);
    out.offset_from(ostart) as std::ffi::c_long as size_t
}
#[no_mangle]
pub unsafe extern "C" fn FSE_writeNCount(
    mut buffer: *mut std::ffi::c_void,
    mut bufferSize: size_t,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
) -> size_t {
    if tableLog > FSE_MAX_TABLELOG as std::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    if tableLog < FSE_MIN_TABLELOG as std::ffi::c_uint {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if bufferSize < FSE_NCountWriteBound(maxSymbolValue, tableLog) {
        return FSE_writeNCount_generic(
            buffer,
            bufferSize,
            normalizedCounter,
            maxSymbolValue,
            tableLog,
            0 as std::ffi::c_int as std::ffi::c_uint,
        );
    }
    FSE_writeNCount_generic(
        buffer,
        bufferSize,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        1 as std::ffi::c_int as std::ffi::c_uint,
    )
}
unsafe extern "C" fn FSE_minTableLog(
    mut srcSize: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
) -> std::ffi::c_uint {
    let mut minBitsSrc =
        (ZSTD_highbit32(srcSize as u32)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let mut minBitsSymbols =
        (ZSTD_highbit32(maxSymbolValue)).wrapping_add(2 as std::ffi::c_int as std::ffi::c_uint);

    if minBitsSrc < minBitsSymbols {
        minBitsSrc
    } else {
        minBitsSymbols
    }
}
#[no_mangle]
pub unsafe extern "C" fn FSE_optimalTableLog_internal(
    mut maxTableLog: std::ffi::c_uint,
    mut srcSize: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
    mut minus: std::ffi::c_uint,
) -> std::ffi::c_uint {
    let mut maxBitsSrc =
        (ZSTD_highbit32(srcSize.wrapping_sub(1 as std::ffi::c_int as size_t) as u32))
            .wrapping_sub(minus);
    let mut tableLog = maxTableLog;
    let mut minBits = FSE_minTableLog(srcSize, maxSymbolValue);
    if tableLog == 0 as std::ffi::c_int as u32 {
        tableLog = FSE_DEFAULT_TABLELOG as u32;
    }
    if maxBitsSrc < tableLog {
        tableLog = maxBitsSrc;
    }
    if minBits > tableLog {
        tableLog = minBits;
    }
    if tableLog < FSE_MIN_TABLELOG as u32 {
        tableLog = FSE_MIN_TABLELOG as u32;
    }
    if tableLog > FSE_MAX_TABLELOG as u32 {
        tableLog = FSE_MAX_TABLELOG as u32;
    }
    tableLog
}
#[no_mangle]
pub unsafe extern "C" fn FSE_optimalTableLog(
    mut maxTableLog: std::ffi::c_uint,
    mut srcSize: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
) -> std::ffi::c_uint {
    FSE_optimalTableLog_internal(
        maxTableLog,
        srcSize,
        maxSymbolValue,
        2 as std::ffi::c_int as std::ffi::c_uint,
    )
}
unsafe extern "C" fn FSE_normalizeM2(
    mut norm: *mut std::ffi::c_short,
    mut tableLog: u32,
    mut count: *const std::ffi::c_uint,
    mut total: size_t,
    mut maxSymbolValue: u32,
    mut lowProbCount: std::ffi::c_short,
) -> size_t {
    let NOT_YET_ASSIGNED = -(2 as std::ffi::c_int) as std::ffi::c_short;
    let mut s: u32 = 0;
    let mut distributed = 0 as std::ffi::c_int as u32;
    let mut ToDistribute: u32 = 0;
    let lowThreshold = (total >> tableLog) as u32;
    let mut lowOne = ((total * 3 as std::ffi::c_int as size_t)
        >> tableLog.wrapping_add(1 as std::ffi::c_int as u32)) as u32;
    s = 0 as std::ffi::c_int as u32;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) == 0 as std::ffi::c_int as std::ffi::c_uint {
            *norm.offset(s as isize) = 0 as std::ffi::c_int as std::ffi::c_short;
        } else if *count.offset(s as isize) <= lowThreshold {
            *norm.offset(s as isize) = lowProbCount;
            distributed = distributed.wrapping_add(1);
            distributed;
            total = total.wrapping_sub(*count.offset(s as isize) as size_t);
        } else if *count.offset(s as isize) <= lowOne {
            *norm.offset(s as isize) = 1 as std::ffi::c_int as std::ffi::c_short;
            distributed = distributed.wrapping_add(1);
            distributed;
            total = total.wrapping_sub(*count.offset(s as isize) as size_t);
        } else {
            *norm.offset(s as isize) = NOT_YET_ASSIGNED;
        }
        s = s.wrapping_add(1);
        s;
    }
    ToDistribute = (((1 as std::ffi::c_int) << tableLog) as u32).wrapping_sub(distributed);
    if ToDistribute == 0 as std::ffi::c_int as u32 {
        return 0 as std::ffi::c_int as size_t;
    }
    if total / ToDistribute as size_t > lowOne as size_t {
        lowOne = (total * 3 as std::ffi::c_int as size_t
            / (ToDistribute * 2 as std::ffi::c_int as u32) as size_t) as u32;
        s = 0 as std::ffi::c_int as u32;
        while s <= maxSymbolValue {
            if *norm.offset(s as isize) as std::ffi::c_int == NOT_YET_ASSIGNED as std::ffi::c_int
                && *count.offset(s as isize) <= lowOne
            {
                *norm.offset(s as isize) = 1 as std::ffi::c_int as std::ffi::c_short;
                distributed = distributed.wrapping_add(1);
                distributed;
                total = total.wrapping_sub(*count.offset(s as isize) as size_t);
            }
            s = s.wrapping_add(1);
            s;
        }
        ToDistribute = (((1 as std::ffi::c_int) << tableLog) as u32).wrapping_sub(distributed);
    }
    if distributed == maxSymbolValue.wrapping_add(1 as std::ffi::c_int as u32) {
        let mut maxV = 0 as std::ffi::c_int as u32;
        let mut maxC = 0 as std::ffi::c_int as u32;
        s = 0 as std::ffi::c_int as u32;
        while s <= maxSymbolValue {
            if *count.offset(s as isize) > maxC {
                maxV = s;
                maxC = *count.offset(s as isize);
            }
            s = s.wrapping_add(1);
            s;
        }
        let fresh4 = &mut (*norm.offset(maxV as isize));
        *fresh4 = (*fresh4 as std::ffi::c_int
            + ToDistribute as std::ffi::c_short as std::ffi::c_int)
            as std::ffi::c_short;
        return 0 as std::ffi::c_int as size_t;
    }
    if total == 0 as std::ffi::c_int as size_t {
        s = 0 as std::ffi::c_int as u32;
        while ToDistribute > 0 as std::ffi::c_int as u32 {
            if *norm.offset(s as isize) as std::ffi::c_int > 0 as std::ffi::c_int {
                ToDistribute = ToDistribute.wrapping_sub(1);
                ToDistribute;
                let fresh5 = &mut (*norm.offset(s as isize));
                *fresh5 += 1;
                *fresh5;
            }
            s = s.wrapping_add(1 as std::ffi::c_int as u32)
                % maxSymbolValue.wrapping_add(1 as std::ffi::c_int as u32);
        }
        return 0 as std::ffi::c_int as size_t;
    }
    let vStepLog = (62 as std::ffi::c_int as u32).wrapping_sub(tableLog) as u64;
    let mid = ((1 as std::ffi::c_ulonglong) << vStepLog.wrapping_sub(1 as std::ffi::c_int as u64))
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong) as u64;
    let rStep = (((1 as std::ffi::c_int as u64) << vStepLog) * ToDistribute as u64)
        .wrapping_add(mid)
        / total as u32 as u64;
    let mut tmpTotal = mid;
    s = 0 as std::ffi::c_int as u32;
    while s <= maxSymbolValue {
        if *norm.offset(s as isize) as std::ffi::c_int == NOT_YET_ASSIGNED as std::ffi::c_int {
            let end = tmpTotal.wrapping_add(*count.offset(s as isize) as u64 * rStep);
            let sStart = (tmpTotal >> vStepLog) as u32;
            let sEnd = (end >> vStepLog) as u32;
            let weight = sEnd.wrapping_sub(sStart);
            if weight < 1 as std::ffi::c_int as u32 {
                return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
            }
            *norm.offset(s as isize) = weight as std::ffi::c_short;
            tmpTotal = end;
        }
        s = s.wrapping_add(1);
        s;
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn FSE_normalizeCount(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut tableLog: std::ffi::c_uint,
    mut count: *const std::ffi::c_uint,
    mut total: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
    mut useLowProbCount: std::ffi::c_uint,
) -> size_t {
    if tableLog == 0 as std::ffi::c_int as std::ffi::c_uint {
        tableLog = FSE_DEFAULT_TABLELOG as std::ffi::c_uint;
    }
    if tableLog < FSE_MIN_TABLELOG as std::ffi::c_uint {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if tableLog > FSE_MAX_TABLELOG as std::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    if tableLog < FSE_minTableLog(total, maxSymbolValue) {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    static mut rtbTable: [u32; 8] = [
        0 as std::ffi::c_int as u32,
        473195 as std::ffi::c_int as u32,
        504333 as std::ffi::c_int as u32,
        520860 as std::ffi::c_int as u32,
        550000 as std::ffi::c_int as u32,
        700000 as std::ffi::c_int as u32,
        750000 as std::ffi::c_int as u32,
        830000 as std::ffi::c_int as u32,
    ];
    let lowProbCount = (if useLowProbCount != 0 {
        -(1 as std::ffi::c_int)
    } else {
        1 as std::ffi::c_int
    }) as std::ffi::c_short;
    let scale = (62 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(tableLog) as u64;
    let step = ((1 as std::ffi::c_int as u64) << 62 as std::ffi::c_int) / total as u32 as u64;
    let vStep =
        ((1 as std::ffi::c_ulonglong) << scale.wrapping_sub(20 as std::ffi::c_int as u64)) as u64;
    let mut stillToDistribute = (1 as std::ffi::c_int) << tableLog;
    let mut s: std::ffi::c_uint = 0;
    let mut largest = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut largestP = 0 as std::ffi::c_int as std::ffi::c_short;
    let mut lowThreshold = (total >> tableLog) as u32;
    s = 0 as std::ffi::c_int as std::ffi::c_uint;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) as size_t == total {
            return 0 as std::ffi::c_int as size_t;
        }
        if *count.offset(s as isize) == 0 as std::ffi::c_int as std::ffi::c_uint {
            *normalizedCounter.offset(s as isize) = 0 as std::ffi::c_int as std::ffi::c_short;
        } else if *count.offset(s as isize) <= lowThreshold {
            *normalizedCounter.offset(s as isize) = lowProbCount;
            stillToDistribute -= 1;
            stillToDistribute;
        } else {
            let mut proba =
                ((*count.offset(s as isize) as u64 * step) >> scale) as std::ffi::c_short;
            if (proba as std::ffi::c_int) < 8 as std::ffi::c_int {
                let mut restToBeat = vStep * *rtbTable.as_ptr().offset(proba as isize) as u64;
                proba = (proba as std::ffi::c_int
                    + ((*count.offset(s as isize) as u64 * step)
                        .wrapping_sub((proba as u64) << scale)
                        > restToBeat) as std::ffi::c_int)
                    as std::ffi::c_short;
            }
            if proba as std::ffi::c_int > largestP as std::ffi::c_int {
                largestP = proba;
                largest = s;
            }
            *normalizedCounter.offset(s as isize) = proba;
            stillToDistribute -= proba as std::ffi::c_int;
        }
        s = s.wrapping_add(1);
        s;
    }
    if -stillToDistribute
        >= *normalizedCounter.offset(largest as isize) as std::ffi::c_int >> 1 as std::ffi::c_int
    {
        let errorCode = FSE_normalizeM2(
            normalizedCounter,
            tableLog,
            count,
            total,
            maxSymbolValue,
            lowProbCount,
        );
        if ERR_isError(errorCode) != 0 {
            return errorCode;
        }
    } else {
        let fresh6 = &mut (*normalizedCounter.offset(largest as isize));
        *fresh6 = (*fresh6 as std::ffi::c_int
            + stillToDistribute as std::ffi::c_short as std::ffi::c_int)
            as std::ffi::c_short;
    }
    tableLog as size_t
}
#[no_mangle]
pub unsafe extern "C" fn FSE_buildCTable_rle(
    mut ct: *mut FSE_CTable,
    mut symbolValue: u8,
) -> size_t {
    let mut ptr = ct as *mut std::ffi::c_void;
    let mut tableU16 = (ptr as *mut u16).offset(2 as std::ffi::c_int as isize);
    let mut FSCTptr =
        (ptr as *mut u32).offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let mut symbolTT = FSCTptr as *mut FSE_symbolCompressionTransform;
    *tableU16.offset(-(2 as std::ffi::c_int) as isize) = 0 as std::ffi::c_int as u16;
    *tableU16.offset(-(1 as std::ffi::c_int) as isize) = symbolValue as u16;
    *tableU16.offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as u16;
    *tableU16.offset(1 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as u16;
    (*symbolTT.offset(symbolValue as isize)).deltaNbBits = 0 as std::ffi::c_int as u32;
    (*symbolTT.offset(symbolValue as isize)).deltaFindState = 0 as std::ffi::c_int;
    0 as std::ffi::c_int as size_t
}
unsafe extern "C" fn FSE_compress_usingCTable_generic(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut ct: *const FSE_CTable,
    fast: std::ffi::c_uint,
) -> size_t {
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let mut ip = iend;
    let mut bitC = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: std::ptr::null_mut::<std::ffi::c_char>(),
        ptr: std::ptr::null_mut::<std::ffi::c_char>(),
        endPtr: std::ptr::null_mut::<std::ffi::c_char>(),
    };
    let mut CState1 = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null::<std::ffi::c_void>(),
        symbolTT: std::ptr::null::<std::ffi::c_void>(),
        stateLog: 0,
    };
    let mut CState2 = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null::<std::ffi::c_void>(),
        symbolTT: std::ptr::null::<std::ffi::c_void>(),
        stateLog: 0,
    };
    if srcSize <= 2 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    let initError = BIT_initCStream(&mut bitC, dst, dstSize);
    if ERR_isError(initError) != 0 {
        return 0 as std::ffi::c_int as size_t;
    }
    if srcSize & 1 as std::ffi::c_int as size_t != 0 {
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState1, ct, *ip as u32);
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState2, ct, *ip as u32);
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as std::ffi::c_uint);
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    } else {
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState2, ct, *ip as u32);
        ip = ip.offset(-1);
        FSE_initCState2(&mut CState1, ct, *ip as u32);
    }
    srcSize = srcSize.wrapping_sub(2 as std::ffi::c_int as size_t);
    if (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        > (FSE_MAX_TABLELOG * 4 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
        && srcSize & 2 as std::ffi::c_int as size_t != 0
    {
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as std::ffi::c_uint);
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as std::ffi::c_uint);
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    }
    while ip > istart {
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as std::ffi::c_uint);
        if (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
            < (FSE_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
        {
            if fast != 0 {
                BIT_flushBitsFast(&mut bitC);
            } else {
                BIT_flushBits(&mut bitC);
            };
        }
        ip = ip.offset(-1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as std::ffi::c_uint);
        if (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
            > (FSE_MAX_TABLELOG * 4 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
        {
            ip = ip.offset(-1);
            FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as std::ffi::c_uint);
            ip = ip.offset(-1);
            FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as std::ffi::c_uint);
        }
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    }
    FSE_flushCState(&mut bitC, &mut CState2);
    FSE_flushCState(&mut bitC, &mut CState1);
    BIT_closeCStream(&mut bitC)
}
#[no_mangle]
pub unsafe extern "C" fn FSE_compress_usingCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut ct: *const FSE_CTable,
) -> size_t {
    let fast = (dstSize
        >= srcSize
            .wrapping_add(srcSize >> 7 as std::ffi::c_int)
            .wrapping_add(4 as std::ffi::c_int as size_t)
            .wrapping_add(::core::mem::size_of::<size_t>() as std::ffi::c_ulong))
        as std::ffi::c_int as std::ffi::c_uint;
    if fast != 0 {
        FSE_compress_usingCTable_generic(
            dst,
            dstSize,
            src,
            srcSize,
            ct,
            1 as std::ffi::c_int as std::ffi::c_uint,
        )
    } else {
        FSE_compress_usingCTable_generic(
            dst,
            dstSize,
            src,
            srcSize,
            ct,
            0 as std::ffi::c_int as std::ffi::c_uint,
        )
    }
}
#[no_mangle]
pub unsafe extern "C" fn FSE_compressBound(mut size: size_t) -> size_t {
    (FSE_NCOUNTBOUND as std::ffi::c_ulong).wrapping_add(
        size.wrapping_add(size >> 7 as std::ffi::c_int)
            .wrapping_add(4 as std::ffi::c_int as size_t)
            .wrapping_add(::core::mem::size_of::<size_t>() as std::ffi::c_ulong),
    )
}
