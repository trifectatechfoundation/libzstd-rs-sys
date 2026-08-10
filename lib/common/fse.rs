use libc::ptrdiff_t;

use crate::lib::common::bitstream::{BIT_CStream_t, BIT_addBits, BIT_flushBits, BitContainerType};

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

/// Pack two consecutive `u16` values into one `u32`.
#[inline]
pub(crate) fn FSE_writeU16Pair(first: u16, second: u16) -> FSE_CTable {
    let [a, b] = first.to_ne_bytes();
    let [c, d] = second.to_ne_bytes();

    u32::from_ne_bytes([a, b, c, d])
}

/// Read the `index`th `u16` of a `&[u32]`.
#[inline]
fn FSE_readU16(ct: &[FSE_CTable], index: usize) -> u16 {
    let bytes = ct[index / 2].to_ne_bytes();
    if index.is_multiple_of(2) {
        u16::from_ne_bytes([bytes[0], bytes[1]])
    } else {
        u16::from_ne_bytes([bytes[2], bytes[3]])
    }
}

#[inline]
pub(crate) fn FSE_initCState(statePtr: &mut FSE_CState_t, ct: &[FSE_CTable]) {
    // the table header occupies the first two bytes of `ct`
    let tableLog = FSE_readU16(ct, 0) as u32;

    // the state table follows the header
    let stateTable = &ct[1..];
    let symbolTT = &ct[FSE_symbolTTIndex(tableLog)..];

    statePtr.value = 1 << tableLog;
    statePtr.stateTable = stateTable.as_ptr().cast::<core::ffi::c_void>();
    statePtr.symbolTT = symbolTT.as_ptr().cast::<core::ffi::c_void>();
    statePtr.stateLog = tableLog;
}

#[inline]
pub(crate) const fn FSE_symbolTTIndex(tableLog: u32) -> usize {
    let skip_header = 1;

    match tableLog {
        0 => skip_header + 1,
        _ => skip_header + (1 << (tableLog - 1)),
    }
}

/// Read the transform of `symbol` out of the symbol transformation table of `ct`.
#[inline]
fn FSE_readSymbolTT(
    ct: &[FSE_CTable],
    tableLog: u32,
    symbol: u32,
) -> FSE_symbolCompressionTransform {
    let index = FSE_symbolTTIndex(tableLog) + 2 * symbol as usize;

    FSE_symbolCompressionTransform {
        deltaFindState: ct[index] as core::ffi::c_int,
        deltaNbBits: ct[index + 1],
    }
}

#[inline]
pub(crate) fn FSE_initCState2(statePtr: &mut FSE_CState_t, ct: &[FSE_CTable], symbol: u32) {
    FSE_initCState(statePtr, ct);
    let symbolTT = FSE_readSymbolTT(ct, statePtr.stateLog, symbol);
    let nbBitsOut = (symbolTT.deltaNbBits).wrapping_add((1) << 15) >> 16;
    let value = (nbBitsOut << 16).wrapping_sub(symbolTT.deltaNbBits) as ptrdiff_t;

    // the state table starts at the third `u16` of `ct`
    let index = 2 + (value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t;
    statePtr.value = FSE_readU16(ct, index as usize) as ptrdiff_t;
}

#[inline]
pub(crate) unsafe fn FSE_encodeSymbol(
    bitC: &mut BIT_CStream_t,
    statePtr: &mut FSE_CState_t,
    symbol: core::ffi::c_uint,
) {
    let symbolTT =
        *(statePtr.symbolTT as *const FSE_symbolCompressionTransform).offset(symbol as isize);
    let stateTable = statePtr.stateTable as *const u16;
    let nbBitsOut = ((statePtr.value + symbolTT.deltaNbBits as ptrdiff_t) >> 16) as u32;
    BIT_addBits(bitC, statePtr.value as BitContainerType, nbBitsOut);
    statePtr.value = *stateTable
        .offset((statePtr.value >> nbBitsOut) + symbolTT.deltaFindState as ptrdiff_t)
        as ptrdiff_t;
}

#[inline]
pub(crate) unsafe fn FSE_flushCState(bitC: &mut BIT_CStream_t, statePtr: &FSE_CState_t) {
    BIT_addBits(bitC, statePtr.value as BitContainerType, statePtr.stateLog);
    BIT_flushBits(bitC);
}

#[inline]
pub(crate) fn FSE_getMaxNbBits(ct: &[FSE_CTable], tableLog: u32, symbolValue: u32) -> u32 {
    let deltaNbBits = FSE_readSymbolTT(ct, tableLog, symbolValue).deltaNbBits;

    deltaNbBits.wrapping_add((((1) << 16) - 1) as u32) >> 16
}

#[inline]
pub(crate) fn FSE_bitCost(
    ct: &[FSE_CTable],
    tableLog: u32,
    symbolValue: u32,
    accuracyLog: u32,
) -> u32 {
    let deltaNbBits = FSE_readSymbolTT(ct, tableLog, symbolValue).deltaNbBits;
    let minNbBits = deltaNbBits >> 16;
    let threshold = minNbBits.wrapping_add(1) << 16;
    let tableSize = ((1) << tableLog) as u32;
    let deltaFromThreshold = threshold.wrapping_sub(deltaNbBits.wrapping_add(tableSize));
    let normalizedDeltaFromThreshold = deltaFromThreshold << accuracyLog >> tableLog;
    let bitMultiplier = ((1) << accuracyLog) as u32;
    (minNbBits.wrapping_add(1) * bitMultiplier).wrapping_sub(normalizedDeltaFromThreshold)
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[repr(C)]
pub(crate) struct FSE_DTableHeader {
    pub(crate) tableLog: u16,
    pub(crate) fastMode: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
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
