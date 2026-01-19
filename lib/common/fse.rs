use libc::ptrdiff_t;

use crate::lib::common::bitstream::{BIT_CStream_t, BIT_addBits, BIT_flushBits, BitContainerType};
use crate::lib::common::mem::MEM_read16;

pub(crate) type FSE_CTable = core::ffi::c_uint;

pub(crate) const FSE_NCOUNTBOUND: core::ffi::c_int = 512;

pub(crate) const fn FSE_CTABLE_SIZE_U32(maxTableLog: usize, maxSymbolValue: usize) -> usize {
    1 + (1 << ((maxTableLog) - 1)) + (((maxSymbolValue) + 1) * 2)
}

pub(crate) const fn FSE_DTABLE_SIZE_U32(maxTableLog: usize) -> usize {
    1 + (1 << (maxTableLog))
}

pub(crate) const fn FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(
    maxSymbolValue: usize,
    tableLog: usize,
) -> usize {
    ((maxSymbolValue + 2) + (1 << (tableLog))) / 2 + size_of::<u64>() / size_of::<u32>()
    /* additional 8 bytes for potential table overwrite */
}

#[expect(dead_code)] // TODO: Remove when used
pub(crate) const fn FSE_BUILD_CTABLE_WORKSPACE_SIZE(
    maxSymbolValue: usize,
    tableLog: usize,
) -> usize {
    size_of::<core::ffi::c_uint>() * FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(maxSymbolValue, tableLog)
}

pub(crate) const fn FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog: usize, maxSymbolValue: usize) -> usize {
    size_of::<u16>() * (maxSymbolValue + 1) + (1 << maxTableLog) + 8
}

pub(crate) const fn FSE_DECOMPRESS_WKSP_SIZE_U32(
    maxTableLog: usize,
    maxSymbolValue: usize,
) -> usize {
    FSE_DTABLE_SIZE_U32(maxTableLog)
        + 1
        + FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog, maxSymbolValue).div_ceil(size_of::<u32>())
        + FSE_MAX_SYMBOL_VALUE.div_ceil(2)
        + 1
}

pub(crate) type FSE_repeat = core::ffi::c_uint;
/// Cannot use the previous table
pub(crate) const FSE_repeat_none: FSE_repeat = 0;
/// Can use the previous table but it must be checked
pub(crate) const FSE_repeat_check: FSE_repeat = 1;
/// Can use the previous table and it is assumed to be valid
pub(crate) const FSE_repeat_valid: FSE_repeat = 2;

#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const core::ffi::c_void,
    pub symbolTT: *const core::ffi::c_void,
    pub stateLog: core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct FSE_symbolCompressionTransform {
    pub(crate) deltaFindState: core::ffi::c_int,
    pub(crate) deltaNbBits: u32,
}

#[inline]
pub(crate) unsafe fn FSE_initCState(statePtr: *mut FSE_CState_t, ct: *const FSE_CTable) {
    let ptr = ct as *const core::ffi::c_void;
    let u16ptr = ptr as *const u16;
    let tableLog = MEM_read16(ptr) as u32;
    (*statePtr).value = (1) << tableLog;
    (*statePtr).stateTable = u16ptr.add(2) as *const core::ffi::c_void;
    (*statePtr).symbolTT = ct.add(1).offset(
        (if tableLog != 0 {
            (1) << tableLog.wrapping_sub(1)
        } else {
            1
        }) as isize,
    ) as *const core::ffi::c_void;
    (*statePtr).stateLog = tableLog;
}

#[inline]
pub(crate) unsafe fn FSE_initCState2(
    statePtr: *mut FSE_CState_t,
    ct: *const FSE_CTable,
    symbol: u32,
) {
    FSE_initCState(statePtr, ct);
    let symbolTT =
        *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable = (*statePtr).stateTable as *const u16;
    let nbBitsOut = (symbolTT.deltaNbBits).wrapping_add(((1) << 15) as u32) >> 16;
    (*statePtr).value = (nbBitsOut << 16).wrapping_sub(symbolTT.deltaNbBits) as ptrdiff_t;
    (*statePtr).value = *stateTable
        .offset(((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t)
        as ptrdiff_t;
}

#[inline]
pub(crate) unsafe fn FSE_encodeSymbol(
    bitC: *mut BIT_CStream_t,
    statePtr: *mut FSE_CState_t,
    symbol: core::ffi::c_uint,
) {
    let symbolTT =
        *((*statePtr).symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable = (*statePtr).stateTable as *const u16;
    let nbBitsOut = (((*statePtr).value + symbolTT.deltaNbBits as ptrdiff_t) >> 16) as u32;
    BIT_addBits(bitC, (*statePtr).value as BitContainerType, nbBitsOut);
    (*statePtr).value = *stateTable
        .offset(((*statePtr).value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t)
        as ptrdiff_t;
}

#[inline]
pub(crate) unsafe fn FSE_flushCState(bitC: *mut BIT_CStream_t, statePtr: *const FSE_CState_t) {
    BIT_addBits(
        bitC,
        (*statePtr).value as BitContainerType,
        (*statePtr).stateLog,
    );
    BIT_flushBits(bitC);
}

#[inline]
pub(crate) unsafe fn FSE_getMaxNbBits(
    symbolTTPtr: *const core::ffi::c_void,
    symbolValue: u32,
) -> u32 {
    let symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    ((*symbolTT.offset(symbolValue as isize)).deltaNbBits).wrapping_add((((1) << 16) - 1) as u32)
        >> 16
}

#[inline]
pub(crate) unsafe fn FSE_bitCost(
    symbolTTPtr: *const core::ffi::c_void,
    tableLog: u32,
    symbolValue: u32,
    accuracyLog: u32,
) -> u32 {
    let symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    let minNbBits = (*symbolTT.offset(symbolValue as isize)).deltaNbBits >> 16;
    let threshold = minNbBits.wrapping_add(1) << 16;
    let tableSize = ((1) << tableLog) as u32;
    let deltaFromThreshold = threshold.wrapping_sub(
        ((*symbolTT.offset(symbolValue as isize)).deltaNbBits).wrapping_add(tableSize),
    );
    let normalizedDeltaFromThreshold = deltaFromThreshold << accuracyLog >> tableLog;
    let bitMultiplier = ((1) << accuracyLog) as u32;
    (minNbBits.wrapping_add(1) * bitMultiplier).wrapping_sub(normalizedDeltaFromThreshold)
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub(crate) struct FSE_DTableHeader {
    pub(crate) tableLog: u16,
    pub(crate) fastMode: u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub(crate) struct FSE_decode_t {
    pub(crate) newState: u16,
    pub(crate) symbol: u8,
    pub(crate) nbBits: u8,
}

pub const FSE_MAX_MEMORY_USAGE: core::ffi::c_int = 14;
pub const FSE_DEFAULT_MEMORY_USAGE: core::ffi::c_int = 13;
const _: () = assert!(
    FSE_DEFAULT_MEMORY_USAGE <= FSE_MAX_MEMORY_USAGE,
    "FSE_DEFAULT_MEMORY_USAGE must be <= FSE_MAX_MEMORY_USAGE"
);

/// Maximum symbol value authorized.
pub(crate) const FSE_MAX_SYMBOL_VALUE: usize = 255;

pub(crate) const FSE_MAX_TABLELOG: core::ffi::c_int = FSE_MAX_MEMORY_USAGE - 2;
pub(crate) const FSE_DEFAULT_TABLELOG: core::ffi::c_int = FSE_DEFAULT_MEMORY_USAGE - 2;
pub(crate) const FSE_MIN_TABLELOG: core::ffi::c_int = 5;

pub(crate) const FSE_TABLELOG_ABSOLUTE_MAX: core::ffi::c_int = 15;
const _: () = assert!(
    FSE_MAX_TABLELOG <= FSE_TABLELOG_ABSOLUTE_MAX,
    "FSE_MAX_TABLELOG > FSE_TABLELOG_ABSOLUTE_MAX is not supported"
);
