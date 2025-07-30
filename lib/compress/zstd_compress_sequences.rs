use crate::lib::compress::fse_compress::{
    FSE_normalizeCount, FSE_optimalTableLog, FSE_writeNCount,
};
use crate::lib::compress::zstd_compress::SeqDef;
use crate::lib::zstd::*;

extern "C" {
    fn FSE_buildCTable_rle(ct: *mut FSE_CTable, symbolValue: std::ffi::c_uchar) -> size_t;
    fn FSE_buildCTable_wksp(
        ct: *mut FSE_CTable,
        normalizedCounter: *const std::ffi::c_short,
        maxSymbolValue: std::ffi::c_uint,
        tableLog: std::ffi::c_uint,
        workSpace: *mut std::ffi::c_void,
        wkspSize: size_t,
    ) -> size_t;
}
pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type unalign64 = u64;
pub type SymbolEncodingType_e = std::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type FSE_repeat = std::ffi::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = std::ffi::c_uint;
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
pub type ZSTD_DefaultPolicy_e = std::ffi::c_uint;
pub const ZSTD_defaultAllowed: ZSTD_DefaultPolicy_e = 1;
pub const ZSTD_defaultDisallowed: ZSTD_DefaultPolicy_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_BuildCTableWksp {
    pub norm: [i16; 53],
    pub wksp: [u32; 285],
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
unsafe extern "C" fn _force_has_format_string(mut format: *const std::ffi::c_char, mut args: ...) {}
pub const MLFSELog: std::ffi::c_int = 9 as std::ffi::c_int;
pub const LLFSELog: std::ffi::c_int = 9 as std::ffi::c_int;
pub const OffFSELog: std::ffi::c_int = 8 as std::ffi::c_int;
static mut LL_bits: [u8; 36] = [
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    4 as std::ffi::c_int as u8,
    6 as std::ffi::c_int as u8,
    7 as std::ffi::c_int as u8,
    8 as std::ffi::c_int as u8,
    9 as std::ffi::c_int as u8,
    10 as std::ffi::c_int as u8,
    11 as std::ffi::c_int as u8,
    12 as std::ffi::c_int as u8,
    13 as std::ffi::c_int as u8,
    14 as std::ffi::c_int as u8,
    15 as std::ffi::c_int as u8,
    16 as std::ffi::c_int as u8,
];
static mut ML_bits: [u8; 53] = [
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    0 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    1 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    2 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    3 as std::ffi::c_int as u8,
    4 as std::ffi::c_int as u8,
    4 as std::ffi::c_int as u8,
    5 as std::ffi::c_int as u8,
    7 as std::ffi::c_int as u8,
    8 as std::ffi::c_int as u8,
    9 as std::ffi::c_int as u8,
    10 as std::ffi::c_int as u8,
    11 as std::ffi::c_int as u8,
    12 as std::ffi::c_int as u8,
    13 as std::ffi::c_int as u8,
    14 as std::ffi::c_int as u8,
    15 as std::ffi::c_int as u8,
    16 as std::ffi::c_int as u8,
];
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
#[inline]
unsafe extern "C" fn FSE_bitCost(
    mut symbolTTPtr: *const std::ffi::c_void,
    mut tableLog: u32,
    mut symbolValue: u32,
    mut accuracyLog: u32,
) -> u32 {
    let mut symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    let minNbBits = (*symbolTT.offset(symbolValue as isize)).deltaNbBits >> 16 as std::ffi::c_int;
    let threshold = minNbBits.wrapping_add(1 as std::ffi::c_int as u32) << 16 as std::ffi::c_int;
    let tableSize = ((1 as std::ffi::c_int) << tableLog) as u32;
    let deltaFromThreshold = threshold.wrapping_sub(
        ((*symbolTT.offset(symbolValue as isize)).deltaNbBits).wrapping_add(tableSize),
    );
    let normalizedDeltaFromThreshold = deltaFromThreshold << accuracyLog >> tableLog;
    let bitMultiplier = ((1 as std::ffi::c_int) << accuracyLog) as u32;
    (minNbBits.wrapping_add(1 as std::ffi::c_int as u32) * bitMultiplier)
        .wrapping_sub(normalizedDeltaFromThreshold)
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
static mut kInverseProbabilityLog256: [std::ffi::c_uint; 256] = [
    0 as std::ffi::c_int as std::ffi::c_uint,
    2048 as std::ffi::c_int as std::ffi::c_uint,
    1792 as std::ffi::c_int as std::ffi::c_uint,
    1642 as std::ffi::c_int as std::ffi::c_uint,
    1536 as std::ffi::c_int as std::ffi::c_uint,
    1453 as std::ffi::c_int as std::ffi::c_uint,
    1386 as std::ffi::c_int as std::ffi::c_uint,
    1329 as std::ffi::c_int as std::ffi::c_uint,
    1280 as std::ffi::c_int as std::ffi::c_uint,
    1236 as std::ffi::c_int as std::ffi::c_uint,
    1197 as std::ffi::c_int as std::ffi::c_uint,
    1162 as std::ffi::c_int as std::ffi::c_uint,
    1130 as std::ffi::c_int as std::ffi::c_uint,
    1100 as std::ffi::c_int as std::ffi::c_uint,
    1073 as std::ffi::c_int as std::ffi::c_uint,
    1047 as std::ffi::c_int as std::ffi::c_uint,
    1024 as std::ffi::c_int as std::ffi::c_uint,
    1001 as std::ffi::c_int as std::ffi::c_uint,
    980 as std::ffi::c_int as std::ffi::c_uint,
    960 as std::ffi::c_int as std::ffi::c_uint,
    941 as std::ffi::c_int as std::ffi::c_uint,
    923 as std::ffi::c_int as std::ffi::c_uint,
    906 as std::ffi::c_int as std::ffi::c_uint,
    889 as std::ffi::c_int as std::ffi::c_uint,
    874 as std::ffi::c_int as std::ffi::c_uint,
    859 as std::ffi::c_int as std::ffi::c_uint,
    844 as std::ffi::c_int as std::ffi::c_uint,
    830 as std::ffi::c_int as std::ffi::c_uint,
    817 as std::ffi::c_int as std::ffi::c_uint,
    804 as std::ffi::c_int as std::ffi::c_uint,
    791 as std::ffi::c_int as std::ffi::c_uint,
    779 as std::ffi::c_int as std::ffi::c_uint,
    768 as std::ffi::c_int as std::ffi::c_uint,
    756 as std::ffi::c_int as std::ffi::c_uint,
    745 as std::ffi::c_int as std::ffi::c_uint,
    734 as std::ffi::c_int as std::ffi::c_uint,
    724 as std::ffi::c_int as std::ffi::c_uint,
    714 as std::ffi::c_int as std::ffi::c_uint,
    704 as std::ffi::c_int as std::ffi::c_uint,
    694 as std::ffi::c_int as std::ffi::c_uint,
    685 as std::ffi::c_int as std::ffi::c_uint,
    676 as std::ffi::c_int as std::ffi::c_uint,
    667 as std::ffi::c_int as std::ffi::c_uint,
    658 as std::ffi::c_int as std::ffi::c_uint,
    650 as std::ffi::c_int as std::ffi::c_uint,
    642 as std::ffi::c_int as std::ffi::c_uint,
    633 as std::ffi::c_int as std::ffi::c_uint,
    626 as std::ffi::c_int as std::ffi::c_uint,
    618 as std::ffi::c_int as std::ffi::c_uint,
    610 as std::ffi::c_int as std::ffi::c_uint,
    603 as std::ffi::c_int as std::ffi::c_uint,
    595 as std::ffi::c_int as std::ffi::c_uint,
    588 as std::ffi::c_int as std::ffi::c_uint,
    581 as std::ffi::c_int as std::ffi::c_uint,
    574 as std::ffi::c_int as std::ffi::c_uint,
    567 as std::ffi::c_int as std::ffi::c_uint,
    561 as std::ffi::c_int as std::ffi::c_uint,
    554 as std::ffi::c_int as std::ffi::c_uint,
    548 as std::ffi::c_int as std::ffi::c_uint,
    542 as std::ffi::c_int as std::ffi::c_uint,
    535 as std::ffi::c_int as std::ffi::c_uint,
    529 as std::ffi::c_int as std::ffi::c_uint,
    523 as std::ffi::c_int as std::ffi::c_uint,
    517 as std::ffi::c_int as std::ffi::c_uint,
    512 as std::ffi::c_int as std::ffi::c_uint,
    506 as std::ffi::c_int as std::ffi::c_uint,
    500 as std::ffi::c_int as std::ffi::c_uint,
    495 as std::ffi::c_int as std::ffi::c_uint,
    489 as std::ffi::c_int as std::ffi::c_uint,
    484 as std::ffi::c_int as std::ffi::c_uint,
    478 as std::ffi::c_int as std::ffi::c_uint,
    473 as std::ffi::c_int as std::ffi::c_uint,
    468 as std::ffi::c_int as std::ffi::c_uint,
    463 as std::ffi::c_int as std::ffi::c_uint,
    458 as std::ffi::c_int as std::ffi::c_uint,
    453 as std::ffi::c_int as std::ffi::c_uint,
    448 as std::ffi::c_int as std::ffi::c_uint,
    443 as std::ffi::c_int as std::ffi::c_uint,
    438 as std::ffi::c_int as std::ffi::c_uint,
    434 as std::ffi::c_int as std::ffi::c_uint,
    429 as std::ffi::c_int as std::ffi::c_uint,
    424 as std::ffi::c_int as std::ffi::c_uint,
    420 as std::ffi::c_int as std::ffi::c_uint,
    415 as std::ffi::c_int as std::ffi::c_uint,
    411 as std::ffi::c_int as std::ffi::c_uint,
    407 as std::ffi::c_int as std::ffi::c_uint,
    402 as std::ffi::c_int as std::ffi::c_uint,
    398 as std::ffi::c_int as std::ffi::c_uint,
    394 as std::ffi::c_int as std::ffi::c_uint,
    390 as std::ffi::c_int as std::ffi::c_uint,
    386 as std::ffi::c_int as std::ffi::c_uint,
    382 as std::ffi::c_int as std::ffi::c_uint,
    377 as std::ffi::c_int as std::ffi::c_uint,
    373 as std::ffi::c_int as std::ffi::c_uint,
    370 as std::ffi::c_int as std::ffi::c_uint,
    366 as std::ffi::c_int as std::ffi::c_uint,
    362 as std::ffi::c_int as std::ffi::c_uint,
    358 as std::ffi::c_int as std::ffi::c_uint,
    354 as std::ffi::c_int as std::ffi::c_uint,
    350 as std::ffi::c_int as std::ffi::c_uint,
    347 as std::ffi::c_int as std::ffi::c_uint,
    343 as std::ffi::c_int as std::ffi::c_uint,
    339 as std::ffi::c_int as std::ffi::c_uint,
    336 as std::ffi::c_int as std::ffi::c_uint,
    332 as std::ffi::c_int as std::ffi::c_uint,
    329 as std::ffi::c_int as std::ffi::c_uint,
    325 as std::ffi::c_int as std::ffi::c_uint,
    322 as std::ffi::c_int as std::ffi::c_uint,
    318 as std::ffi::c_int as std::ffi::c_uint,
    315 as std::ffi::c_int as std::ffi::c_uint,
    311 as std::ffi::c_int as std::ffi::c_uint,
    308 as std::ffi::c_int as std::ffi::c_uint,
    305 as std::ffi::c_int as std::ffi::c_uint,
    302 as std::ffi::c_int as std::ffi::c_uint,
    298 as std::ffi::c_int as std::ffi::c_uint,
    295 as std::ffi::c_int as std::ffi::c_uint,
    292 as std::ffi::c_int as std::ffi::c_uint,
    289 as std::ffi::c_int as std::ffi::c_uint,
    286 as std::ffi::c_int as std::ffi::c_uint,
    282 as std::ffi::c_int as std::ffi::c_uint,
    279 as std::ffi::c_int as std::ffi::c_uint,
    276 as std::ffi::c_int as std::ffi::c_uint,
    273 as std::ffi::c_int as std::ffi::c_uint,
    270 as std::ffi::c_int as std::ffi::c_uint,
    267 as std::ffi::c_int as std::ffi::c_uint,
    264 as std::ffi::c_int as std::ffi::c_uint,
    261 as std::ffi::c_int as std::ffi::c_uint,
    258 as std::ffi::c_int as std::ffi::c_uint,
    256 as std::ffi::c_int as std::ffi::c_uint,
    253 as std::ffi::c_int as std::ffi::c_uint,
    250 as std::ffi::c_int as std::ffi::c_uint,
    247 as std::ffi::c_int as std::ffi::c_uint,
    244 as std::ffi::c_int as std::ffi::c_uint,
    241 as std::ffi::c_int as std::ffi::c_uint,
    239 as std::ffi::c_int as std::ffi::c_uint,
    236 as std::ffi::c_int as std::ffi::c_uint,
    233 as std::ffi::c_int as std::ffi::c_uint,
    230 as std::ffi::c_int as std::ffi::c_uint,
    228 as std::ffi::c_int as std::ffi::c_uint,
    225 as std::ffi::c_int as std::ffi::c_uint,
    222 as std::ffi::c_int as std::ffi::c_uint,
    220 as std::ffi::c_int as std::ffi::c_uint,
    217 as std::ffi::c_int as std::ffi::c_uint,
    215 as std::ffi::c_int as std::ffi::c_uint,
    212 as std::ffi::c_int as std::ffi::c_uint,
    209 as std::ffi::c_int as std::ffi::c_uint,
    207 as std::ffi::c_int as std::ffi::c_uint,
    204 as std::ffi::c_int as std::ffi::c_uint,
    202 as std::ffi::c_int as std::ffi::c_uint,
    199 as std::ffi::c_int as std::ffi::c_uint,
    197 as std::ffi::c_int as std::ffi::c_uint,
    194 as std::ffi::c_int as std::ffi::c_uint,
    192 as std::ffi::c_int as std::ffi::c_uint,
    190 as std::ffi::c_int as std::ffi::c_uint,
    187 as std::ffi::c_int as std::ffi::c_uint,
    185 as std::ffi::c_int as std::ffi::c_uint,
    182 as std::ffi::c_int as std::ffi::c_uint,
    180 as std::ffi::c_int as std::ffi::c_uint,
    178 as std::ffi::c_int as std::ffi::c_uint,
    175 as std::ffi::c_int as std::ffi::c_uint,
    173 as std::ffi::c_int as std::ffi::c_uint,
    171 as std::ffi::c_int as std::ffi::c_uint,
    168 as std::ffi::c_int as std::ffi::c_uint,
    166 as std::ffi::c_int as std::ffi::c_uint,
    164 as std::ffi::c_int as std::ffi::c_uint,
    162 as std::ffi::c_int as std::ffi::c_uint,
    159 as std::ffi::c_int as std::ffi::c_uint,
    157 as std::ffi::c_int as std::ffi::c_uint,
    155 as std::ffi::c_int as std::ffi::c_uint,
    153 as std::ffi::c_int as std::ffi::c_uint,
    151 as std::ffi::c_int as std::ffi::c_uint,
    149 as std::ffi::c_int as std::ffi::c_uint,
    146 as std::ffi::c_int as std::ffi::c_uint,
    144 as std::ffi::c_int as std::ffi::c_uint,
    142 as std::ffi::c_int as std::ffi::c_uint,
    140 as std::ffi::c_int as std::ffi::c_uint,
    138 as std::ffi::c_int as std::ffi::c_uint,
    136 as std::ffi::c_int as std::ffi::c_uint,
    134 as std::ffi::c_int as std::ffi::c_uint,
    132 as std::ffi::c_int as std::ffi::c_uint,
    130 as std::ffi::c_int as std::ffi::c_uint,
    128 as std::ffi::c_int as std::ffi::c_uint,
    126 as std::ffi::c_int as std::ffi::c_uint,
    123 as std::ffi::c_int as std::ffi::c_uint,
    121 as std::ffi::c_int as std::ffi::c_uint,
    119 as std::ffi::c_int as std::ffi::c_uint,
    117 as std::ffi::c_int as std::ffi::c_uint,
    115 as std::ffi::c_int as std::ffi::c_uint,
    114 as std::ffi::c_int as std::ffi::c_uint,
    112 as std::ffi::c_int as std::ffi::c_uint,
    110 as std::ffi::c_int as std::ffi::c_uint,
    108 as std::ffi::c_int as std::ffi::c_uint,
    106 as std::ffi::c_int as std::ffi::c_uint,
    104 as std::ffi::c_int as std::ffi::c_uint,
    102 as std::ffi::c_int as std::ffi::c_uint,
    100 as std::ffi::c_int as std::ffi::c_uint,
    98 as std::ffi::c_int as std::ffi::c_uint,
    96 as std::ffi::c_int as std::ffi::c_uint,
    94 as std::ffi::c_int as std::ffi::c_uint,
    93 as std::ffi::c_int as std::ffi::c_uint,
    91 as std::ffi::c_int as std::ffi::c_uint,
    89 as std::ffi::c_int as std::ffi::c_uint,
    87 as std::ffi::c_int as std::ffi::c_uint,
    85 as std::ffi::c_int as std::ffi::c_uint,
    83 as std::ffi::c_int as std::ffi::c_uint,
    82 as std::ffi::c_int as std::ffi::c_uint,
    80 as std::ffi::c_int as std::ffi::c_uint,
    78 as std::ffi::c_int as std::ffi::c_uint,
    76 as std::ffi::c_int as std::ffi::c_uint,
    74 as std::ffi::c_int as std::ffi::c_uint,
    73 as std::ffi::c_int as std::ffi::c_uint,
    71 as std::ffi::c_int as std::ffi::c_uint,
    69 as std::ffi::c_int as std::ffi::c_uint,
    67 as std::ffi::c_int as std::ffi::c_uint,
    66 as std::ffi::c_int as std::ffi::c_uint,
    64 as std::ffi::c_int as std::ffi::c_uint,
    62 as std::ffi::c_int as std::ffi::c_uint,
    61 as std::ffi::c_int as std::ffi::c_uint,
    59 as std::ffi::c_int as std::ffi::c_uint,
    57 as std::ffi::c_int as std::ffi::c_uint,
    55 as std::ffi::c_int as std::ffi::c_uint,
    54 as std::ffi::c_int as std::ffi::c_uint,
    52 as std::ffi::c_int as std::ffi::c_uint,
    50 as std::ffi::c_int as std::ffi::c_uint,
    49 as std::ffi::c_int as std::ffi::c_uint,
    47 as std::ffi::c_int as std::ffi::c_uint,
    46 as std::ffi::c_int as std::ffi::c_uint,
    44 as std::ffi::c_int as std::ffi::c_uint,
    42 as std::ffi::c_int as std::ffi::c_uint,
    41 as std::ffi::c_int as std::ffi::c_uint,
    39 as std::ffi::c_int as std::ffi::c_uint,
    37 as std::ffi::c_int as std::ffi::c_uint,
    36 as std::ffi::c_int as std::ffi::c_uint,
    34 as std::ffi::c_int as std::ffi::c_uint,
    33 as std::ffi::c_int as std::ffi::c_uint,
    31 as std::ffi::c_int as std::ffi::c_uint,
    30 as std::ffi::c_int as std::ffi::c_uint,
    28 as std::ffi::c_int as std::ffi::c_uint,
    26 as std::ffi::c_int as std::ffi::c_uint,
    25 as std::ffi::c_int as std::ffi::c_uint,
    23 as std::ffi::c_int as std::ffi::c_uint,
    22 as std::ffi::c_int as std::ffi::c_uint,
    20 as std::ffi::c_int as std::ffi::c_uint,
    19 as std::ffi::c_int as std::ffi::c_uint,
    17 as std::ffi::c_int as std::ffi::c_uint,
    16 as std::ffi::c_int as std::ffi::c_uint,
    14 as std::ffi::c_int as std::ffi::c_uint,
    13 as std::ffi::c_int as std::ffi::c_uint,
    11 as std::ffi::c_int as std::ffi::c_uint,
    10 as std::ffi::c_int as std::ffi::c_uint,
    8 as std::ffi::c_int as std::ffi::c_uint,
    7 as std::ffi::c_int as std::ffi::c_uint,
    5 as std::ffi::c_int as std::ffi::c_uint,
    4 as std::ffi::c_int as std::ffi::c_uint,
    2 as std::ffi::c_int as std::ffi::c_uint,
    1 as std::ffi::c_int as std::ffi::c_uint,
];
unsafe extern "C" fn ZSTD_getFSEMaxSymbolValue(mut ctable: *const FSE_CTable) -> std::ffi::c_uint {
    let mut ptr = ctable as *const std::ffi::c_void;
    let mut u16ptr = ptr as *const u16;

    MEM_read16(u16ptr.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void) as u32
}
unsafe extern "C" fn ZSTD_useLowProbCount(nbSeq: size_t) -> std::ffi::c_uint {
    (nbSeq >= 2048 as std::ffi::c_int as size_t) as std::ffi::c_int as std::ffi::c_uint
}
unsafe extern "C" fn ZSTD_NCountCost(
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
    nbSeq: size_t,
    FSELog: std::ffi::c_uint,
) -> size_t {
    let mut wksp: [u8; 512] = [0; 512];
    let mut norm: [i16; 53] = [0; 53];
    let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
    let err_code = FSE_normalizeCount(
        norm.as_mut_ptr(),
        tableLog,
        count,
        nbSeq,
        max,
        ZSTD_useLowProbCount(nbSeq),
    );
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    FSE_writeNCount(
        wksp.as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[u8; 512]>() as std::ffi::c_ulong,
        norm.as_mut_ptr(),
        max,
        tableLog,
    )
}
unsafe extern "C" fn ZSTD_entropyCost(
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
    total: size_t,
) -> size_t {
    let mut cost = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut s: std::ffi::c_uint = 0;
    s = 0 as std::ffi::c_int as std::ffi::c_uint;
    while s <= max {
        let mut norm = ((256 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_mul(*count.offset(s as isize)) as size_t
            / total) as std::ffi::c_uint;
        if *count.offset(s as isize) != 0 as std::ffi::c_int as std::ffi::c_uint
            && norm == 0 as std::ffi::c_int as std::ffi::c_uint
        {
            norm = 1 as std::ffi::c_int as std::ffi::c_uint;
        }
        cost = cost.wrapping_add(
            (*count.offset(s as isize))
                .wrapping_mul(*kInverseProbabilityLog256.as_ptr().offset(norm as isize)),
        );
        s = s.wrapping_add(1);
        s;
    }
    (cost >> 8 as std::ffi::c_int) as size_t
}
#[export_name = crate::prefix!(ZSTD_fseBitCost)]
pub unsafe extern "C" fn ZSTD_fseBitCost(
    mut ctable: *const FSE_CTable,
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
) -> size_t {
    let kAccuracyLog = 8 as std::ffi::c_int as std::ffi::c_uint;
    let mut cost = 0 as std::ffi::c_int as size_t;
    let mut s: std::ffi::c_uint = 0;
    let mut cstate = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null::<std::ffi::c_void>(),
        symbolTT: std::ptr::null::<std::ffi::c_void>(),
        stateLog: 0,
    };
    FSE_initCState(&mut cstate, ctable);
    if ZSTD_getFSEMaxSymbolValue(ctable) < max {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    s = 0 as std::ffi::c_int as std::ffi::c_uint;
    while s <= max {
        let tableLog = cstate.stateLog;
        let badCost =
            tableLog.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) << kAccuracyLog;
        let bitCost = FSE_bitCost(cstate.symbolTT, tableLog, s, kAccuracyLog);
        if *count.offset(s as isize) != 0 as std::ffi::c_int as std::ffi::c_uint {
            if bitCost >= badCost {
                return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
            }
            cost = cost.wrapping_add(*count.offset(s as isize) as size_t * bitCost as size_t);
        }
        s = s.wrapping_add(1);
        s;
    }
    cost >> kAccuracyLog
}
#[export_name = crate::prefix!(ZSTD_crossEntropyCost)]
pub unsafe extern "C" fn ZSTD_crossEntropyCost(
    mut norm: *const std::ffi::c_short,
    mut accuracyLog: std::ffi::c_uint,
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
) -> size_t {
    let shift = (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(accuracyLog);
    let mut cost = 0 as std::ffi::c_int as size_t;
    let mut s: std::ffi::c_uint = 0;
    s = 0 as std::ffi::c_int as std::ffi::c_uint;
    while s <= max {
        let normAcc = if *norm.offset(s as isize) as std::ffi::c_int != -(1 as std::ffi::c_int) {
            *norm.offset(s as isize) as std::ffi::c_uint
        } else {
            1 as std::ffi::c_int as std::ffi::c_uint
        };
        let norm256 = normAcc << shift;
        cost = cost.wrapping_add(
            (*count.offset(s as isize))
                .wrapping_mul(*kInverseProbabilityLog256.as_ptr().offset(norm256 as isize))
                as size_t,
        );
        s = s.wrapping_add(1);
        s;
    }
    cost >> 8 as std::ffi::c_int
}
#[export_name = crate::prefix!(ZSTD_selectEncodingType)]
pub unsafe extern "C" fn ZSTD_selectEncodingType(
    mut repeatMode: *mut FSE_repeat,
    mut count: *const std::ffi::c_uint,
    max: std::ffi::c_uint,
    mostFrequent: size_t,
    mut nbSeq: size_t,
    FSELog: std::ffi::c_uint,
    mut prevCTable: *const FSE_CTable,
    mut defaultNorm: *const std::ffi::c_short,
    mut defaultNormLog: u32,
    isDefaultAllowed: ZSTD_DefaultPolicy_e,
    strategy: ZSTD_strategy,
) -> SymbolEncodingType_e {
    if mostFrequent == nbSeq {
        *repeatMode = FSE_repeat_none;
        if isDefaultAllowed as std::ffi::c_uint != 0 && nbSeq <= 2 as std::ffi::c_int as size_t {
            return set_basic;
        }
        return set_rle;
    }
    if (strategy as std::ffi::c_uint) < ZSTD_lazy as std::ffi::c_int as std::ffi::c_uint {
        if isDefaultAllowed as u64 != 0 {
            let staticFse_nbSeq_max = 1000 as std::ffi::c_int as size_t;
            let mult = (10 as std::ffi::c_int as std::ffi::c_uint)
                .wrapping_sub(strategy as std::ffi::c_uint) as size_t;
            let baseLog = 3 as std::ffi::c_int as size_t;
            let dynamicFse_nbSeq_min =
                (((1 as std::ffi::c_int as size_t) << defaultNormLog) * mult) >> baseLog;
            if *repeatMode as std::ffi::c_uint
                == FSE_repeat_valid as std::ffi::c_int as std::ffi::c_uint
                && nbSeq < staticFse_nbSeq_max
            {
                return set_repeat;
            }
            if nbSeq < dynamicFse_nbSeq_min
                || mostFrequent < nbSeq >> defaultNormLog.wrapping_sub(1 as std::ffi::c_int as u32)
            {
                *repeatMode = FSE_repeat_none;
                return set_basic;
            }
        }
    } else {
        let basicCost = if isDefaultAllowed as std::ffi::c_uint != 0 {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, count, max)
        } else {
            -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t
        };
        let repeatCost = if *repeatMode as std::ffi::c_uint
            != FSE_repeat_none as std::ffi::c_int as std::ffi::c_uint
        {
            ZSTD_fseBitCost(prevCTable, count, max)
        } else {
            -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t
        };
        let NCountCost = ZSTD_NCountCost(count, max, nbSeq, FSELog);
        let compressedCost =
            (NCountCost << 3 as std::ffi::c_int).wrapping_add(ZSTD_entropyCost(count, max, nbSeq));
        isDefaultAllowed as u64 != 0;
        if basicCost <= repeatCost && basicCost <= compressedCost {
            *repeatMode = FSE_repeat_none;
            return set_basic;
        }
        if repeatCost <= compressedCost {
            return set_repeat;
        }
    }
    *repeatMode = FSE_repeat_check;
    set_compressed
}
#[export_name = crate::prefix!(ZSTD_buildCTable)]
pub unsafe extern "C" fn ZSTD_buildCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut nextCTable: *mut FSE_CTable,
    mut FSELog: u32,
    mut type_0: SymbolEncodingType_e,
    mut count: *mut std::ffi::c_uint,
    mut max: u32,
    mut codeTable: *const u8,
    mut nbSeq: size_t,
    mut defaultNorm: *const i16,
    mut defaultNormLog: u32,
    mut defaultMax: u32,
    mut prevCTable: *const FSE_CTable,
    mut prevCTableSize: size_t,
    mut entropyWorkspace: *mut std::ffi::c_void,
    mut entropyWorkspaceSize: size_t,
) -> size_t {
    let mut op = dst as *mut u8;
    let oend: *const u8 = op.offset(dstCapacity as isize);
    match type_0 as std::ffi::c_uint {
        1 => {
            let err_code = FSE_buildCTable_rle(nextCTable, max as u8);
            if ERR_isError(err_code) != 0 {
                return err_code;
            }
            if dstCapacity == 0 as std::ffi::c_int as size_t {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            *op = *codeTable.offset(0 as std::ffi::c_int as isize);
            1 as std::ffi::c_int as size_t
        }
        3 => {
            libc::memcpy(
                nextCTable as *mut std::ffi::c_void,
                prevCTable as *const std::ffi::c_void,
                prevCTableSize as libc::size_t,
            );
            0 as std::ffi::c_int as size_t
        }
        0 => {
            let err_code_0 = FSE_buildCTable_wksp(
                nextCTable,
                defaultNorm,
                defaultMax,
                defaultNormLog,
                entropyWorkspace,
                entropyWorkspaceSize,
            );
            if ERR_isError(err_code_0) != 0 {
                return err_code_0;
            }
            0 as std::ffi::c_int as size_t
        }
        2 => {
            let mut wksp = entropyWorkspace as *mut ZSTD_BuildCTableWksp;
            let mut nbSeq_1 = nbSeq;
            let tableLog = FSE_optimalTableLog(FSELog, nbSeq, max);
            if *count.offset(
                *codeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                    as isize,
            ) > 1 as std::ffi::c_int as std::ffi::c_uint
            {
                let fresh0 = &mut (*count.offset(
                    *codeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                        as isize,
                ));
                *fresh0 = (*fresh0).wrapping_sub(1);
                *fresh0;
                nbSeq_1 = nbSeq_1.wrapping_sub(1);
                nbSeq_1;
            }
            let err_code_1 = FSE_normalizeCount(
                ((*wksp).norm).as_mut_ptr(),
                tableLog,
                count,
                nbSeq_1,
                max,
                ZSTD_useLowProbCount(nbSeq_1),
            );
            if ERR_isError(err_code_1) != 0 {
                return err_code_1;
            }
            let NCountSize = FSE_writeNCount(
                op as *mut std::ffi::c_void,
                oend.offset_from(op) as std::ffi::c_long as size_t,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
            );
            let err_code_2 = NCountSize;
            if ERR_isError(err_code_2) != 0 {
                return err_code_2;
            }
            let err_code_3 = FSE_buildCTable_wksp(
                nextCTable,
                ((*wksp).norm).as_mut_ptr(),
                max,
                tableLog,
                ((*wksp).wksp).as_mut_ptr() as *mut std::ffi::c_void,
                ::core::mem::size_of::<[u32; 285]>() as std::ffi::c_ulong,
            );
            if ERR_isError(err_code_3) != 0 {
                return err_code_3;
            }
            NCountSize
        }
        _ => -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_encodeSequences_body(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: std::ffi::c_int,
) -> size_t {
    let mut blockStream = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: std::ptr::null_mut::<std::ffi::c_char>(),
        ptr: std::ptr::null_mut::<std::ffi::c_char>(),
        endPtr: std::ptr::null_mut::<std::ffi::c_char>(),
    };
    let mut stateMatchLength = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null::<std::ffi::c_void>(),
        symbolTT: std::ptr::null::<std::ffi::c_void>(),
        stateLog: 0,
    };
    let mut stateOffsetBits = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null::<std::ffi::c_void>(),
        symbolTT: std::ptr::null::<std::ffi::c_void>(),
        stateLog: 0,
    };
    let mut stateLitLength = FSE_CState_t {
        value: 0,
        stateTable: std::ptr::null::<std::ffi::c_void>(),
        symbolTT: std::ptr::null::<std::ffi::c_void>(),
        stateLog: 0,
    };
    if ERR_isError(BIT_initCStream(&mut blockStream, dst, dstCapacity)) != 0 {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    FSE_initCState2(
        &mut stateMatchLength,
        CTable_MatchLength,
        *mlCodeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) as u32,
    );
    FSE_initCState2(
        &mut stateOffsetBits,
        CTable_OffsetBits,
        *ofCodeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) as u32,
    );
    FSE_initCState2(
        &mut stateLitLength,
        CTable_LitLength,
        *llCodeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) as u32,
    );
    BIT_addBits(
        &mut blockStream,
        (*sequences.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)).litLength
            as BitContainerType,
        *LL_bits.as_ptr().offset(
            *llCodeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                as isize,
        ) as std::ffi::c_uint,
    );
    if MEM_32bits() != 0 {
        BIT_flushBits(&mut blockStream);
    }
    BIT_addBits(
        &mut blockStream,
        (*sequences.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)).mlBase
            as BitContainerType,
        *ML_bits.as_ptr().offset(
            *mlCodeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                as isize,
        ) as std::ffi::c_uint,
    );
    if MEM_32bits() != 0 {
        BIT_flushBits(&mut blockStream);
    }
    if longOffsets != 0 {
        let ofBits =
            *ofCodeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) as u32;
        let extraBits = ofBits.wrapping_sub(
            if ofBits
                < ((if MEM_32bits() != 0 {
                    25 as std::ffi::c_int
                } else {
                    57 as std::ffi::c_int
                }) as u32)
                    .wrapping_sub(1 as std::ffi::c_int as u32)
            {
                ofBits
            } else {
                ((if MEM_32bits() != 0 {
                    25 as std::ffi::c_int
                } else {
                    57 as std::ffi::c_int
                }) as u32)
                    .wrapping_sub(1 as std::ffi::c_int as u32)
            },
        );
        if extraBits != 0 {
            BIT_addBits(
                &mut blockStream,
                (*sequences.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize))
                    .offBase as BitContainerType,
                extraBits,
            );
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            ((*sequences.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize))
                .offBase
                >> extraBits) as BitContainerType,
            ofBits.wrapping_sub(extraBits),
        );
    } else {
        BIT_addBits(
            &mut blockStream,
            (*sequences.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)).offBase
                as BitContainerType,
            *ofCodeTable.offset(nbSeq.wrapping_sub(1 as std::ffi::c_int as size_t) as isize)
                as std::ffi::c_uint,
        );
    }
    BIT_flushBits(&mut blockStream);
    let mut n: size_t = 0;
    n = nbSeq.wrapping_sub(2 as std::ffi::c_int as size_t);
    while n < nbSeq {
        let llCode = *llCodeTable.offset(n as isize);
        let ofCode = *ofCodeTable.offset(n as isize);
        let mlCode = *mlCodeTable.offset(n as isize);
        let llBits = *LL_bits.as_ptr().offset(llCode as isize) as u32;
        let ofBits_0 = ofCode as u32;
        let mlBits = *ML_bits.as_ptr().offset(mlCode as isize) as u32;
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateOffsetBits,
            ofCode as std::ffi::c_uint,
        );
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateMatchLength,
            mlCode as std::ffi::c_uint,
        );
        if MEM_32bits() != 0 {
            BIT_flushBits(&mut blockStream);
        }
        FSE_encodeSymbol(
            &mut blockStream,
            &mut stateLitLength,
            llCode as std::ffi::c_uint,
        );
        if MEM_32bits() != 0
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits)
                >= (64 as std::ffi::c_int
                    - 7 as std::ffi::c_int
                    - (LLFSELog + MLFSELog + OffFSELog)) as u32
        {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.offset(n as isize)).litLength as BitContainerType,
            llBits,
        );
        if MEM_32bits() != 0 && llBits.wrapping_add(mlBits) > 24 as std::ffi::c_int as u32 {
            BIT_flushBits(&mut blockStream);
        }
        BIT_addBits(
            &mut blockStream,
            (*sequences.offset(n as isize)).mlBase as BitContainerType,
            mlBits,
        );
        if MEM_32bits() != 0
            || ofBits_0.wrapping_add(mlBits).wrapping_add(llBits) > 56 as std::ffi::c_int as u32
        {
            BIT_flushBits(&mut blockStream);
        }
        if longOffsets != 0 {
            let extraBits_0 = ofBits_0.wrapping_sub(
                if ofBits_0
                    < ((if MEM_32bits() != 0 {
                        25 as std::ffi::c_int
                    } else {
                        57 as std::ffi::c_int
                    }) as u32)
                        .wrapping_sub(1 as std::ffi::c_int as u32)
                {
                    ofBits_0
                } else {
                    ((if MEM_32bits() != 0 {
                        25 as std::ffi::c_int
                    } else {
                        57 as std::ffi::c_int
                    }) as u32)
                        .wrapping_sub(1 as std::ffi::c_int as u32)
                },
            );
            if extraBits_0 != 0 {
                BIT_addBits(
                    &mut blockStream,
                    (*sequences.offset(n as isize)).offBase as BitContainerType,
                    extraBits_0,
                );
                BIT_flushBits(&mut blockStream);
            }
            BIT_addBits(
                &mut blockStream,
                ((*sequences.offset(n as isize)).offBase >> extraBits_0) as BitContainerType,
                ofBits_0.wrapping_sub(extraBits_0),
            );
        } else {
            BIT_addBits(
                &mut blockStream,
                (*sequences.offset(n as isize)).offBase as BitContainerType,
                ofBits_0,
            );
        }
        BIT_flushBits(&mut blockStream);
        n = n.wrapping_sub(1);
        n;
    }
    FSE_flushCState(&mut blockStream, &mut stateMatchLength);
    FSE_flushCState(&mut blockStream, &mut stateOffsetBits);
    FSE_flushCState(&mut blockStream, &mut stateLitLength);
    let streamSize = BIT_closeCStream(&mut blockStream);
    if streamSize == 0 as std::ffi::c_int as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    streamSize
}
unsafe extern "C" fn ZSTD_encodeSequences_default(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: std::ffi::c_int,
) -> size_t {
    ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    )
}
unsafe extern "C" fn ZSTD_encodeSequences_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: std::ffi::c_int,
) -> size_t {
    ZSTD_encodeSequences_body(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    )
}
#[export_name = crate::prefix!(ZSTD_encodeSequences)]
pub unsafe extern "C" fn ZSTD_encodeSequences(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut CTable_MatchLength: *const FSE_CTable,
    mut mlCodeTable: *const u8,
    mut CTable_OffsetBits: *const FSE_CTable,
    mut ofCodeTable: *const u8,
    mut CTable_LitLength: *const FSE_CTable,
    mut llCodeTable: *const u8,
    mut sequences: *const SeqDef,
    mut nbSeq: size_t,
    mut longOffsets: std::ffi::c_int,
    mut bmi2: std::ffi::c_int,
) -> size_t {
    if bmi2 != 0 {
        return ZSTD_encodeSequences_bmi2(
            dst,
            dstCapacity,
            CTable_MatchLength,
            mlCodeTable,
            CTable_OffsetBits,
            ofCodeTable,
            CTable_LitLength,
            llCodeTable,
            sequences,
            nbSeq,
            longOffsets,
        );
    }
    ZSTD_encodeSequences_default(
        dst,
        dstCapacity,
        CTable_MatchLength,
        mlCodeTable,
        CTable_OffsetBits,
        ofCodeTable,
        CTable_LitLength,
        llCodeTable,
        sequences,
        nbSeq,
        longOffsets,
    )
}
