use core::ptr;

use crate::lib::common::entropy_common::HUF_readStats;
use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::mem::{MEM_32bits, MEM_writeLE16, MEM_writeLEST};
use crate::lib::compress::fse_compress::{
    FSE_buildCTable_wksp, FSE_compress_usingCTable, FSE_normalizeCount, FSE_optimalTableLog,
    FSE_optimalTableLog_internal, FSE_writeNCount,
};
use crate::lib::compress::hist::{HIST_count_simple, HIST_count_wksp};
use crate::lib::zstd::*;
pub type size_t = core::ffi::c_ulong;
pub type FSE_CTable = core::ffi::c_uint;
pub type HUF_CElt = size_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type nodeElt = nodeElt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeElt_s {
    pub count: u32,
    pub parent: u16,
    pub byte: u8,
    pub nbBits: u8,
}
pub type huffNodeTable = [nodeElt; 512];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_buildCTable_wksp_tables {
    pub huffNodeTbl: huffNodeTable,
    pub rankPosition: [rankPos; 192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rankPos {
    pub base: u16,
    pub curr: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CTableHeader {
    pub tableLog: u8,
    pub maxSymbolValue: u8,
    pub unused: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_WriteCTableWksp {
    pub wksp: HUF_CompressWeightsWksp,
    pub bitsToWeight: [u8; 13],
    pub huffWeight: [u8; 255],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CompressWeightsWksp {
    pub CTable: [FSE_CTable; 59],
    pub scratchBuffer: [u32; 41],
    pub count: [core::ffi::c_uint; 13],
    pub norm: [i16; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [size_t; 2],
    pub bitPos: [size_t; 2],
    pub startPtr: *mut u8,
    pub ptr: *mut u8,
    pub endPtr: *mut u8,
}
pub type HUF_repeat = core::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_nbStreams_e = core::ffi::c_uint;
pub const HUF_fourStreams: HUF_nbStreams_e = 1;
pub const HUF_singleStream: HUF_nbStreams_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [core::ffi::c_uint; 256],
    pub CTable: [HUF_CElt; 257],
    pub wksps: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub buildCTable_wksp: HUF_buildCTable_wksp_tables,
    pub writeCTable_wksp: HUF_WriteCTableWksp,
    pub hist_wksp: [u32; 1024],
}
#[inline]
const fn ZSTD_countLeadingZeros32(mut val: u32) -> core::ffi::c_uint {
    val.leading_zeros() as i32 as core::ffi::c_uint
}
#[inline]
const fn ZSTD_highbit32(mut val: u32) -> core::ffi::c_uint {
    (31 as core::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const HUF_BLOCKSIZE_MAX: core::ffi::c_int = 128 * 1024;
pub const HUF_TABLELOG_MAX: core::ffi::c_int = 12;
pub const HUF_TABLELOG_DEFAULT: core::ffi::c_int = 11;
pub const HUF_SYMBOLVALUE_MAX: core::ffi::c_int = 255;
pub const HUF_CTABLEBOUND: core::ffi::c_int = 129;
pub const NULL: core::ffi::c_int = 0;
pub const HUF_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
unsafe fn HUF_alignUpWorkspace(
    mut workspace: *mut core::ffi::c_void,
    mut workspaceSizePtr: *mut size_t,
    mut align: size_t,
) -> *mut core::ffi::c_void {
    let mask = align.wrapping_sub(1);
    let rem = workspace as size_t & mask;
    let add = align.wrapping_sub(rem) & mask;
    let aligned = (workspace as *mut u8).offset(add as isize);
    if *workspaceSizePtr >= add {
        *workspaceSizePtr = (*workspaceSizePtr).wrapping_sub(add);
        aligned as *mut core::ffi::c_void
    } else {
        *workspaceSizePtr = 0;
        NULL as *mut core::ffi::c_void
    }
}
pub const MAX_FSE_TABLELOG_FOR_HUFF_HEADER: core::ffi::c_int = 6;
unsafe fn HUF_compressWeights(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut weightTable: *const core::ffi::c_void,
    mut wtSize: size_t,
    mut workspace: *mut core::ffi::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.offset(dstSize as isize);
    let mut maxSymbolValue = HUF_TABLELOG_MAX as core::ffi::c_uint;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as u32;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<u32>() as core::ffi::c_ulong,
    ) as *mut HUF_CompressWeightsWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_CompressWeightsWksp>() as core::ffi::c_ulong {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    if wtSize <= 1 {
        return 0;
    }
    let maxCount = HIST_count_simple(
        ((*wksp).count).as_mut_ptr(),
        &mut maxSymbolValue,
        weightTable,
        wtSize,
    );
    if maxCount as size_t == wtSize {
        return 1;
    }
    if maxCount == 1 {
        return 0;
    }
    tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
    let _var_err__ = FSE_normalizeCount(
        ((*wksp).norm).as_mut_ptr(),
        tableLog,
        ((*wksp).count).as_mut_ptr(),
        wtSize,
        maxSymbolValue,
        0,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let hSize = FSE_writeNCount(
        op as *mut core::ffi::c_void,
        oend.offset_from(op) as core::ffi::c_long as size_t,
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    op = op.offset(hSize as isize);
    let _var_err___0 = FSE_buildCTable_wksp(
        ((*wksp).CTable).as_mut_ptr(),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[u32; 41]>() as core::ffi::c_ulong,
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let cSize = FSE_compress_usingCTable(
        op as *mut core::ffi::c_void,
        oend.offset_from(op) as core::ffi::c_long as size_t,
        weightTable,
        wtSize,
        ((*wksp).CTable).as_mut_ptr(),
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 {
        return 0;
    }
    op = op.offset(cSize as isize);
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
fn HUF_getNbBits(mut elt: HUF_CElt) -> size_t {
    elt & 0xff as core::ffi::c_int as HUF_CElt
}
fn HUF_getNbBitsFast(mut elt: HUF_CElt) -> size_t {
    elt
}
fn HUF_getValue(mut elt: HUF_CElt) -> size_t {
    elt & !(0xff as core::ffi::c_int as size_t)
}
fn HUF_getValueFast(mut elt: HUF_CElt) -> size_t {
    elt
}
unsafe fn HUF_setNbBits(mut elt: *mut HUF_CElt, mut nbBits: size_t) {
    *elt = nbBits;
}
unsafe fn HUF_setValue(mut elt: *mut HUF_CElt, mut value: size_t) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 {
        *elt |= value
            << (::core::mem::size_of::<HUF_CElt>() as core::ffi::c_ulong)
                .wrapping_mul(8)
                .wrapping_sub(nbBits);
    }
}
pub unsafe fn HUF_readCTableHeader(mut ctable: *const HUF_CElt) -> HUF_CTableHeader {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    libc::memcpy(
        &mut header as *mut HUF_CTableHeader as *mut core::ffi::c_void,
        ctable as *const core::ffi::c_void,
        ::core::mem::size_of::<HUF_CTableHeader>() as core::ffi::c_ulong as libc::size_t,
    );
    header
}
unsafe fn HUF_writeCTableHeader(
    mut ctable: *mut HUF_CElt,
    mut tableLog: u32,
    mut maxSymbolValue: u32,
) {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    ptr::write_bytes(
        &mut header as *mut HUF_CTableHeader as *mut u8,
        0,
        ::core::mem::size_of::<HUF_CTableHeader>(),
    );
    header.tableLog = tableLog as u8;
    header.maxSymbolValue = maxSymbolValue as u8;
    libc::memcpy(
        ctable as *mut core::ffi::c_void,
        &mut header as *mut HUF_CTableHeader as *const core::ffi::c_void,
        ::core::mem::size_of::<HUF_CTableHeader>() as core::ffi::c_ulong as libc::size_t,
    );
}
pub unsafe fn HUF_writeCTable_wksp(
    mut dst: *mut core::ffi::c_void,
    mut maxDstSize: size_t,
    mut CTable: *const HUF_CElt,
    mut maxSymbolValue: core::ffi::c_uint,
    mut huffLog: core::ffi::c_uint,
    mut workspace: *mut core::ffi::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ct = CTable.offset(1);
    let mut op = dst as *mut u8;
    let mut n: u32 = 0;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<u32>() as core::ffi::c_ulong,
    ) as *mut HUF_WriteCTableWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_WriteCTableWksp>() as core::ffi::c_ulong {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as core::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as core::ffi::c_int) as size_t;
    }
    *((*wksp).bitsToWeight).as_mut_ptr().offset(0) = 0;
    n = 1;
    while n < huffLog.wrapping_add(1) {
        *((*wksp).bitsToWeight).as_mut_ptr().offset(n as isize) =
            huffLog.wrapping_add(1).wrapping_sub(n) as u8;
        n = n.wrapping_add(1);
    }
    n = 0;
    while n < maxSymbolValue {
        *((*wksp).huffWeight).as_mut_ptr().offset(n as isize) = *((*wksp).bitsToWeight)
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n as isize)) as isize);
        n = n.wrapping_add(1);
    }
    if maxDstSize < 1 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    let hSize = HUF_compressWeights(
        op.offset(1) as *mut core::ffi::c_void,
        maxDstSize.wrapping_sub(1),
        ((*wksp).huffWeight).as_mut_ptr() as *const core::ffi::c_void,
        maxSymbolValue as size_t,
        &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut core::ffi::c_void,
        ::core::mem::size_of::<HUF_CompressWeightsWksp>() as core::ffi::c_ulong,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if (hSize > 1) as core::ffi::c_int
        & (hSize < maxSymbolValue.wrapping_div(2) as size_t) as core::ffi::c_int
        != 0
    {
        *op.offset(0) = hSize as u8;
        return hSize.wrapping_add(1);
    }
    if maxSymbolValue > (256 - 128) as core::ffi::c_uint {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    if maxSymbolValue
        .wrapping_add(1)
        .wrapping_div(2)
        .wrapping_add(1) as size_t
        > maxDstSize
    {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    *op.offset(0) = (128 as core::ffi::c_uint).wrapping_add(maxSymbolValue.wrapping_sub(1)) as u8;
    *((*wksp).huffWeight)
        .as_mut_ptr()
        .offset(maxSymbolValue as isize) = 0;
    n = 0;
    while n < maxSymbolValue {
        *op.offset((n / 2).wrapping_add(1) as isize) =
            (((*((*wksp).huffWeight).as_mut_ptr().offset(n as isize) as core::ffi::c_int) << 4)
                + *((*wksp).huffWeight)
                    .as_mut_ptr()
                    .offset(n.wrapping_add(1) as isize) as core::ffi::c_int) as u8;
        n = n.wrapping_add(2);
    }
    maxSymbolValue
        .wrapping_add(1)
        .wrapping_div(2)
        .wrapping_add(1) as size_t
}

pub unsafe fn HUF_readCTable(
    mut CTable: *mut HUF_CElt,
    mut maxSymbolValuePtr: *mut core::ffi::c_uint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut hasZeroWeights: *mut core::ffi::c_uint,
) -> size_t {
    let src = core::slice::from_raw_parts(src.cast(), srcSize as usize);

    let mut huffWeight: [u8; 256] = [0; 256];
    let mut rankVal: [u32; 13] = [0; 13];
    let mut tableLog = 0;
    let mut nbSymbols = 0;
    let ct = CTable.offset(1);
    let readSize = HUF_readStats(
        &mut huffWeight,
        (255 + 1) as size_t,
        &mut rankVal,
        &mut nbSymbols,
        &mut tableLog,
        src,
    );
    if ERR_isError(readSize) != 0 {
        return readSize;
    }
    *hasZeroWeights =
        (*rankVal.as_mut_ptr().offset(0) > 0) as core::ffi::c_int as core::ffi::c_uint;
    if tableLog > HUF_TABLELOG_MAX as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    if nbSymbols > (*maxSymbolValuePtr).wrapping_add(1) {
        return -(ZSTD_error_maxSymbolValue_tooSmall as core::ffi::c_int) as size_t;
    }
    *maxSymbolValuePtr = nbSymbols.wrapping_sub(1);
    HUF_writeCTableHeader(CTable, tableLog, *maxSymbolValuePtr);
    let mut n: u32 = 0;
    let mut nextRankStart = 0u32;
    n = 1;
    while n <= tableLog {
        let mut curr = nextRankStart;
        nextRankStart = nextRankStart
            .wrapping_add(*rankVal.as_mut_ptr().offset(n as isize) << n.wrapping_sub(1));
        *rankVal.as_mut_ptr().offset(n as isize) = curr;
        n = n.wrapping_add(1);
    }
    let mut n_0: u32 = 0;
    n_0 = 0;
    while n_0 < nbSymbols {
        let w = *huffWeight.as_mut_ptr().offset(n_0 as isize) as u32;
        HUF_setNbBits(
            ct.offset(n_0 as isize),
            (tableLog.wrapping_add(1).wrapping_sub(w) as u8 as core::ffi::c_int
                & -((w != 0) as core::ffi::c_int)) as size_t,
        );
        n_0 = n_0.wrapping_add(1);
    }
    let mut nbPerRank: [u16; 14] = [0; 14];
    let mut valPerRank: [u16; 14] = [0; 14];
    let mut n_1: u32 = 0;
    n_1 = 0;
    while n_1 < nbSymbols {
        let fresh0 = &mut (*nbPerRank
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n_1 as isize)) as isize));
        *fresh0 = (*fresh0).wrapping_add(1);
        n_1 = n_1.wrapping_add(1);
    }
    *valPerRank
        .as_mut_ptr()
        .offset(tableLog.wrapping_add(1) as isize) = 0;
    let mut min = 0;
    let mut n_2: u32 = 0;
    n_2 = tableLog;
    while n_2 > 0 {
        *valPerRank.as_mut_ptr().offset(n_2 as isize) = min;
        min = (min as core::ffi::c_int
            + *nbPerRank.as_mut_ptr().offset(n_2 as isize) as core::ffi::c_int)
            as u16;
        min = (min as core::ffi::c_int >> 1) as u16;
        n_2 = n_2.wrapping_sub(1);
    }
    let mut n_3: u32 = 0;
    n_3 = 0;
    while n_3 < nbSymbols {
        let fresh1 = &mut (*valPerRank
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n_3 as isize)) as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        HUF_setValue(ct.offset(n_3 as isize), fresh2 as size_t);
        n_3 = n_3.wrapping_add(1);
    }
    readSize
}
pub unsafe fn HUF_getNbBitsFromCTable(mut CTable: *const HUF_CElt, mut symbolValue: u32) -> u32 {
    let ct = CTable.offset(1);
    if symbolValue > (HUF_readCTableHeader(CTable)).maxSymbolValue as u32 {
        return 0;
    }
    HUF_getNbBits(*ct.offset(symbolValue as isize)) as u32
}
unsafe fn HUF_setMaxHeight(
    mut huffNode: *mut nodeElt,
    mut lastNonNull: u32,
    mut targetNbBits: u32,
) -> u32 {
    let largestBits = (*huffNode.offset(lastNonNull as isize)).nbBits as u32;
    if largestBits <= targetNbBits {
        return largestBits;
    }
    let mut totalCost = 0;
    let baseCost = ((1) << largestBits.wrapping_sub(targetNbBits)) as u32;
    let mut n = lastNonNull as core::ffi::c_int;
    while (*huffNode.offset(n as isize)).nbBits as u32 > targetNbBits {
        totalCost = (totalCost as u32).wrapping_add(baseCost.wrapping_sub(
            ((1) << largestBits.wrapping_sub((*huffNode.offset(n as isize)).nbBits as u32)) as u32,
        )) as core::ffi::c_int as core::ffi::c_int;
        (*huffNode.offset(n as isize)).nbBits = targetNbBits as u8;
        n -= 1;
    }
    while (*huffNode.offset(n as isize)).nbBits as u32 == targetNbBits {
        n -= 1;
    }
    totalCost >>= largestBits.wrapping_sub(targetNbBits);
    let noSymbol = 0xf0f0f0f0 as core::ffi::c_uint;
    let mut rankLast: [u32; 14] = [0; 14];
    ptr::write_bytes(
        rankLast.as_mut_ptr() as *mut u8,
        0xf0,
        ::core::mem::size_of::<[u32; 14]>(),
    );
    let mut currentNbBits = targetNbBits;
    let mut pos: core::ffi::c_int = 0;
    pos = n;
    while pos >= 0 {
        if ((*huffNode.offset(pos as isize)).nbBits as u32) < currentNbBits {
            currentNbBits = (*huffNode.offset(pos as isize)).nbBits as u32;
            *rankLast
                .as_mut_ptr()
                .offset(targetNbBits.wrapping_sub(currentNbBits) as isize) = pos as u32;
        }
        pos -= 1;
    }
    while totalCost > 0 {
        let mut nBitsToDecrease = (ZSTD_highbit32(totalCost as u32)).wrapping_add(1);
        while nBitsToDecrease > 1 {
            let highPos = *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize);
            let lowPos = *rankLast
                .as_mut_ptr()
                .offset(nBitsToDecrease.wrapping_sub(1) as isize);
            if highPos != noSymbol {
                if lowPos == noSymbol {
                    break;
                }
                let highTotal = (*huffNode.offset(highPos as isize)).count;
                let lowTotal = 2 * (*huffNode.offset(lowPos as isize)).count;
                if highTotal <= lowTotal {
                    break;
                }
            }
            nBitsToDecrease = nBitsToDecrease.wrapping_sub(1);
        }
        while nBitsToDecrease <= HUF_TABLELOG_MAX as u32
            && *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) == noSymbol
        {
            nBitsToDecrease = nBitsToDecrease.wrapping_add(1);
        }
        totalCost -= (1) << nBitsToDecrease.wrapping_sub(1);
        let fresh3 = &mut (*huffNode
            .offset(*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) as isize))
        .nbBits;
        *fresh3 = (*fresh3).wrapping_add(1);
        if *rankLast
            .as_mut_ptr()
            .offset(nBitsToDecrease.wrapping_sub(1) as isize)
            == noSymbol
        {
            *rankLast
                .as_mut_ptr()
                .offset(nBitsToDecrease.wrapping_sub(1) as isize) =
                *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize);
        }
        if *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) == 0 {
            *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) = noSymbol;
        } else {
            let fresh4 = &mut (*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize));
            *fresh4 = (*fresh4).wrapping_sub(1);
            if (*huffNode.offset(*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) as isize))
                .nbBits as u32
                != targetNbBits.wrapping_sub(nBitsToDecrease)
            {
                *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) = noSymbol;
            }
        }
    }
    while totalCost < 0 {
        if *rankLast.as_mut_ptr().offset(1) == noSymbol {
            while (*huffNode.offset(n as isize)).nbBits as u32 == targetNbBits {
                n -= 1;
            }
            let fresh5 = &mut (*huffNode.offset((n + 1) as isize)).nbBits;
            *fresh5 = (*fresh5).wrapping_sub(1);
            *rankLast.as_mut_ptr().offset(1) = (n + 1) as u32;
            totalCost += 1;
        } else {
            let fresh6 = &mut (*huffNode
                .offset((*rankLast.as_mut_ptr().offset(1)).wrapping_add(1) as isize))
            .nbBits;
            *fresh6 = (*fresh6).wrapping_sub(1);
            let fresh7 = &mut (*rankLast.as_mut_ptr().offset(1));
            *fresh7 = (*fresh7).wrapping_add(1);
            totalCost += 1;
        }
    }
    targetNbBits
}
pub const RANK_POSITION_TABLE_SIZE: core::ffi::c_int = 192;
pub const RANK_POSITION_MAX_COUNT_LOG: core::ffi::c_int = 32;
pub const RANK_POSITION_LOG_BUCKETS_BEGIN: core::ffi::c_int =
    RANK_POSITION_TABLE_SIZE - 1 - RANK_POSITION_MAX_COUNT_LOG - 1;
pub const RANK_POSITION_DISTINCT_COUNT_CUTOFF: core::ffi::c_uint = (RANK_POSITION_LOG_BUCKETS_BEGIN
    as core::ffi::c_uint)
    .wrapping_add(ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN as u32));
unsafe fn HUF_getIndex(count: u32) -> u32 {
    if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        (ZSTD_highbit32(count)).wrapping_add(RANK_POSITION_LOG_BUCKETS_BEGIN as core::ffi::c_uint)
    }
}
unsafe fn HUF_swapNodes(mut a: *mut nodeElt, mut b: *mut nodeElt) {
    core::ptr::swap(a, b);
}
#[inline(always)]
unsafe fn HUF_insertionSort(
    mut huffNode: *mut nodeElt,
    low: core::ffi::c_int,
    high: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let size = high - low + 1;
    huffNode = huffNode.offset(low as isize);
    i = 1;
    while i < size {
        let key = *huffNode.offset(i as isize);
        let mut j = i - 1;
        while j >= 0 && (*huffNode.offset(j as isize)).count < key.count {
            *huffNode.offset((j + 1) as isize) = *huffNode.offset(j as isize);
            j -= 1;
        }
        *huffNode.offset((j + 1) as isize) = key;
        i += 1;
    }
}
unsafe fn HUF_quickSortPartition(
    mut arr: *mut nodeElt,
    low: core::ffi::c_int,
    high: core::ffi::c_int,
) -> core::ffi::c_int {
    let pivot = (*arr.offset(high as isize)).count;
    let mut i = low - 1;
    let mut j = low;
    while j < high {
        if (*arr.offset(j as isize)).count > pivot {
            i += 1;
            HUF_swapNodes(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1;
    }
    HUF_swapNodes(
        &mut *arr.offset((i + 1) as isize),
        &mut *arr.offset(high as isize),
    );
    i + 1
}
unsafe fn HUF_simpleQuickSort(
    mut arr: *mut nodeElt,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) {
    let kInsertionSortThreshold = 8;
    if high - low < kInsertionSortThreshold {
        HUF_insertionSort(arr, low, high);
        return;
    }
    while low < high {
        let idx = HUF_quickSortPartition(arr, low, high);
        if idx - low < high - idx {
            HUF_simpleQuickSort(arr, low, idx - 1);
            low = idx + 1;
        } else {
            HUF_simpleQuickSort(arr, idx + 1, high);
            high = idx - 1;
        }
    }
}
unsafe fn HUF_sort(
    mut huffNode: *mut nodeElt,
    mut count: *const core::ffi::c_uint,
    maxSymbolValue: u32,
    mut rankPosition: *mut rankPos,
) {
    let mut n: u32 = 0;
    let maxSymbolValue1 = maxSymbolValue.wrapping_add(1);
    ptr::write_bytes(
        rankPosition as *mut u8,
        0,
        ::core::mem::size_of::<rankPos>() * 192,
    );
    n = 0;
    while n < maxSymbolValue1 {
        let mut lowerRank = HUF_getIndex(*count.offset(n as isize));
        let fresh8 = &mut (*rankPosition.offset(lowerRank as isize)).base;
        *fresh8 = (*fresh8).wrapping_add(1);
        n = n.wrapping_add(1);
    }
    n = (RANK_POSITION_TABLE_SIZE - 1) as u32;
    while n > 0 {
        let fresh9 = &mut (*rankPosition.offset(n.wrapping_sub(1) as isize)).base;
        *fresh9 = (*fresh9 as core::ffi::c_int
            + (*rankPosition.offset(n as isize)).base as core::ffi::c_int) as u16;
        (*rankPosition.offset(n.wrapping_sub(1) as isize)).curr =
            (*rankPosition.offset(n.wrapping_sub(1) as isize)).base;
        n = n.wrapping_sub(1);
    }
    n = 0;
    while n < maxSymbolValue1 {
        let c = *count.offset(n as isize);
        let r = (HUF_getIndex(c)).wrapping_add(1);
        let fresh10 = &mut (*rankPosition.offset(r as isize)).curr;
        let fresh11 = *fresh10;
        *fresh10 = (*fresh10).wrapping_add(1);
        let pos = fresh11 as u32;
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as u8;
        n = n.wrapping_add(1);
    }
    n = RANK_POSITION_DISTINCT_COUNT_CUTOFF;
    while n < (RANK_POSITION_TABLE_SIZE - 1) as u32 {
        let bucketSize = (*rankPosition.offset(n as isize)).curr as core::ffi::c_int
            - (*rankPosition.offset(n as isize)).base as core::ffi::c_int;
        let bucketStartIdx = (*rankPosition.offset(n as isize)).base as u32;
        if bucketSize > 1 {
            HUF_simpleQuickSort(huffNode.offset(bucketStartIdx as isize), 0, bucketSize - 1);
        }
        n = n.wrapping_add(1);
    }
}
pub const STARTNODE: core::ffi::c_int = HUF_SYMBOLVALUE_MAX + 1;
unsafe fn HUF_buildTree(mut huffNode: *mut nodeElt, mut maxSymbolValue: u32) -> core::ffi::c_int {
    let huffNode0 = huffNode.offset(-(1));
    let mut nonNullRank: core::ffi::c_int = 0;
    let mut lowS: core::ffi::c_int = 0;
    let mut lowN: core::ffi::c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut n: core::ffi::c_int = 0;
    let mut nodeRoot: core::ffi::c_int = 0;
    nonNullRank = maxSymbolValue as core::ffi::c_int;
    while (*huffNode.offset(nonNullRank as isize)).count == 0 {
        nonNullRank -= 1;
    }
    lowS = nonNullRank;
    nodeRoot = nodeNb + lowS - 1;
    lowN = nodeNb;
    (*huffNode.offset(nodeNb as isize)).count = ((*huffNode.offset(lowS as isize)).count)
        .wrapping_add((*huffNode.offset((lowS - 1) as isize)).count);
    let fresh12 = &mut (*huffNode.offset((lowS - 1) as isize)).parent;
    *fresh12 = nodeNb as u16;
    (*huffNode.offset(lowS as isize)).parent = *fresh12;
    nodeNb += 1;
    lowS -= 2;
    n = nodeNb;
    while n <= nodeRoot {
        (*huffNode.offset(n as isize)).count = (1) << 30;
        n += 1;
    }
    (*huffNode0.offset(0)).count = (1) << 31;
    while nodeNb <= nodeRoot {
        let n1 =
            if (*huffNode.offset(lowS as isize)).count < (*huffNode.offset(lowN as isize)).count {
                let fresh13 = lowS;
                lowS -= 1;
                fresh13
            } else {
                let fresh14 = lowN;
                lowN += 1;
                fresh14
            };
        let n2 =
            if (*huffNode.offset(lowS as isize)).count < (*huffNode.offset(lowN as isize)).count {
                let fresh15 = lowS;
                lowS -= 1;
                fresh15
            } else {
                let fresh16 = lowN;
                lowN += 1;
                fresh16
            };
        (*huffNode.offset(nodeNb as isize)).count = ((*huffNode.offset(n1 as isize)).count)
            .wrapping_add((*huffNode.offset(n2 as isize)).count);
        let fresh17 = &mut (*huffNode.offset(n2 as isize)).parent;
        *fresh17 = nodeNb as u16;
        (*huffNode.offset(n1 as isize)).parent = *fresh17;
        nodeNb += 1;
    }
    (*huffNode.offset(nodeRoot as isize)).nbBits = 0;
    n = nodeRoot - 1;
    while n >= STARTNODE {
        (*huffNode.offset(n as isize)).nbBits = ((*huffNode
            .offset((*huffNode.offset(n as isize)).parent as isize))
        .nbBits as core::ffi::c_int
            + 1) as u8;
        n -= 1;
    }
    n = 0;
    while n <= nonNullRank {
        (*huffNode.offset(n as isize)).nbBits = ((*huffNode
            .offset((*huffNode.offset(n as isize)).parent as isize))
        .nbBits as core::ffi::c_int
            + 1) as u8;
        n += 1;
    }
    nonNullRank
}
unsafe fn HUF_buildCTableFromTree(
    mut CTable: *mut HUF_CElt,
    mut huffNode: *const nodeElt,
    mut nonNullRank: core::ffi::c_int,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
) {
    let ct = CTable.offset(1);
    let mut n: core::ffi::c_int = 0;
    let mut nbPerRank: [u16; 13] = [0; 13];
    let mut valPerRank: [u16; 13] = [0; 13];
    let alphabetSize = maxSymbolValue.wrapping_add(1) as core::ffi::c_int;
    n = 0;
    while n <= nonNullRank {
        let fresh18 = &mut (*nbPerRank
            .as_mut_ptr()
            .offset((*huffNode.offset(n as isize)).nbBits as isize));
        *fresh18 = (*fresh18).wrapping_add(1);
        n += 1;
    }
    let mut min = 0;
    n = maxNbBits as core::ffi::c_int;
    while n > 0 {
        *valPerRank.as_mut_ptr().offset(n as isize) = min;
        min = (min as core::ffi::c_int
            + *nbPerRank.as_mut_ptr().offset(n as isize) as core::ffi::c_int) as u16;
        min = (min as core::ffi::c_int >> 1) as u16;
        n -= 1;
    }
    n = 0;
    while n < alphabetSize {
        HUF_setNbBits(
            ct.offset((*huffNode.offset(n as isize)).byte as core::ffi::c_int as isize),
            (*huffNode.offset(n as isize)).nbBits as size_t,
        );
        n += 1;
    }
    n = 0;
    while n < alphabetSize {
        let fresh19 = &mut (*valPerRank
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n as isize)) as isize));
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).wrapping_add(1);
        HUF_setValue(ct.offset(n as isize), fresh20 as size_t);
        n += 1;
    }
    HUF_writeCTableHeader(CTable, maxNbBits, maxSymbolValue);
}
pub unsafe fn HUF_buildCTable_wksp(
    mut CTable: *mut HUF_CElt,
    mut count: *const core::ffi::c_uint,
    mut maxSymbolValue: u32,
    mut maxNbBits: u32,
    mut workSpace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let wksp_tables = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<u32>() as core::ffi::c_ulong,
    ) as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.offset(1);
    let mut nonNullRank: core::ffi::c_int = 0;
    if wkspSize < ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() as core::ffi::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as core::ffi::c_int) as size_t;
    }
    if maxNbBits == 0 {
        maxNbBits = HUF_TABLELOG_DEFAULT as u32;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as u32 {
        return -(ZSTD_error_maxSymbolValue_tooLarge as core::ffi::c_int) as size_t;
    }
    ptr::write_bytes(
        huffNode0 as *mut u8,
        0,
        ::core::mem::size_of::<huffNodeTable>(),
    );
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as u32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as u32 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);
    maxNbBits as size_t
}
pub unsafe fn HUF_estimateCompressedSize(
    mut CTable: *const HUF_CElt,
    mut count: *const core::ffi::c_uint,
    mut maxSymbolValue: core::ffi::c_uint,
) -> size_t {
    let mut ct = CTable.offset(1);
    let mut nbBits = 0 as size_t;
    let mut s: core::ffi::c_int = 0;
    s = 0;
    while s <= maxSymbolValue as core::ffi::c_int {
        nbBits = nbBits.wrapping_add(
            HUF_getNbBits(*ct.offset(s as isize)) * *count.offset(s as isize) as size_t,
        );
        s += 1;
    }
    nbBits >> 3
}
pub unsafe fn HUF_validateCTable(
    mut CTable: *const HUF_CElt,
    mut count: *const core::ffi::c_uint,
    mut maxSymbolValue: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut header = HUF_readCTableHeader(CTable);
    let mut ct = CTable.offset(1);
    let mut bad = 0;
    let mut s: core::ffi::c_int = 0;
    if (header.maxSymbolValue as core::ffi::c_uint) < maxSymbolValue {
        return 0;
    }
    s = 0;
    while s <= maxSymbolValue as core::ffi::c_int {
        bad |= (*count.offset(s as isize) != 0) as core::ffi::c_int
            & (HUF_getNbBits(*ct.offset(s as isize)) == 0) as core::ffi::c_int;
        s += 1;
    }
    (bad == 0) as core::ffi::c_int
}
pub fn HUF_compressBound(mut size: size_t) -> size_t {
    (HUF_CTABLEBOUND as size_t).wrapping_add(size.wrapping_add(size >> 8).wrapping_add(8))
}
pub const HUF_BITS_IN_CONTAINER: core::ffi::c_ulong =
    (::core::mem::size_of::<size_t>() as core::ffi::c_ulong).wrapping_mul(8);
unsafe fn HUF_initCStream(
    mut bitC: *mut HUF_CStream_t,
    mut startPtr: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    ptr::write_bytes(bitC as *mut u8, 0, ::core::mem::size_of::<HUF_CStream_t>());
    (*bitC).startPtr = startPtr as *mut u8;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr)
        .offset(dstCapacity as isize)
        .offset(-(::core::mem::size_of::<size_t>() as core::ffi::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<size_t>() as core::ffi::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    0
}
#[inline(always)]
unsafe fn HUF_addBits(
    mut bitC: *mut HUF_CStream_t,
    mut elt: HUF_CElt,
    mut idx: core::ffi::c_int,
    mut kFast: core::ffi::c_int,
) {
    *((*bitC).bitContainer).as_mut_ptr().offset(idx as isize) >>= HUF_getNbBits(elt);
    *((*bitC).bitContainer).as_mut_ptr().offset(idx as isize) |= if kFast != 0 {
        HUF_getValueFast(elt)
    } else {
        HUF_getValue(elt)
    };
    let fresh21 = &mut (*((*bitC).bitPos).as_mut_ptr().offset(idx as isize));
    *fresh21 = (*fresh21).wrapping_add(HUF_getNbBitsFast(elt));
}
#[inline(always)]
unsafe fn HUF_zeroIndex1(mut bitC: *mut HUF_CStream_t) {
    *((*bitC).bitContainer).as_mut_ptr().offset(1) = 0;
    *((*bitC).bitPos).as_mut_ptr().offset(1) = 0;
}
#[inline(always)]
unsafe fn HUF_mergeIndex1(mut bitC: *mut HUF_CStream_t) {
    *((*bitC).bitContainer).as_mut_ptr().offset(0) >>=
        *((*bitC).bitPos).as_mut_ptr().offset(1) & 0xff as core::ffi::c_int as size_t;
    *((*bitC).bitContainer).as_mut_ptr().offset(0) |=
        *((*bitC).bitContainer).as_mut_ptr().offset(1);
    let fresh22 = &mut (*((*bitC).bitPos).as_mut_ptr().offset(0));
    *fresh22 = (*fresh22).wrapping_add(*((*bitC).bitPos).as_mut_ptr().offset(1));
}
#[inline(always)]
unsafe fn HUF_flushBits(mut bitC: *mut HUF_CStream_t, mut kFast: core::ffi::c_int) {
    let nbBits = *((*bitC).bitPos).as_mut_ptr().offset(0) & 0xff as core::ffi::c_int as size_t;
    let nbBytes = nbBits >> 3;
    let bitContainer = *((*bitC).bitContainer).as_mut_ptr().offset(0)
        >> HUF_BITS_IN_CONTAINER.wrapping_sub(nbBits);
    *((*bitC).bitPos).as_mut_ptr().offset(0) &= 7;
    MEM_writeLEST((*bitC).ptr as *mut core::ffi::c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    if kFast == 0 && (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
}
unsafe fn HUF_endMark() -> HUF_CElt {
    let mut endMark: HUF_CElt = 0;
    HUF_setNbBits(&mut endMark, 1);
    HUF_setValue(&mut endMark, 1);
    endMark
}
unsafe fn HUF_closeCStream(mut bitC: *mut HUF_CStream_t) -> size_t {
    HUF_addBits(bitC, HUF_endMark(), 0, 0);
    HUF_flushBits(bitC, 0);
    let nbBits = *((*bitC).bitPos).as_mut_ptr().offset(0) & 0xff as core::ffi::c_int as size_t;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0;
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as core::ffi::c_long as size_t)
        .wrapping_add((nbBits > 0) as core::ffi::c_int as size_t)
}
#[inline(always)]
unsafe fn HUF_encodeSymbol(
    mut bitCPtr: *mut HUF_CStream_t,
    mut symbol: u32,
    mut CTable: *const HUF_CElt,
    mut idx: core::ffi::c_int,
    mut fast: core::ffi::c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}
#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body_loop(
    mut bitC: *mut HUF_CStream_t,
    mut ip: *const u8,
    mut srcSize: size_t,
    mut ct: *const HUF_CElt,
    mut kUnroll: core::ffi::c_int,
    mut kFastFlush: core::ffi::c_int,
    mut kLastFast: core::ffi::c_int,
) {
    let mut n = srcSize as core::ffi::c_int;
    let mut rem = n % kUnroll;
    if rem > 0 {
        while rem > 0 {
            n -= 1;
            HUF_encodeSymbol(bitC, *ip.offset(n as isize) as u32, ct, 0, 0);
            rem -= 1;
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    if n % (2 * kUnroll) != 0 {
        let mut u: core::ffi::c_int = 0;
        u = 1;
        while u < kUnroll {
            HUF_encodeSymbol(bitC, *ip.offset((n - u) as isize) as u32, ct, 0, 1);
            u += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as u32,
            ct,
            0,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        n -= kUnroll;
    }
    while n > 0 {
        let mut u_0: core::ffi::c_int = 0;
        u_0 = 1;
        while u_0 < kUnroll {
            HUF_encodeSymbol(bitC, *ip.offset((n - u_0) as isize) as u32, ct, 0, 1);
            u_0 += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as u32,
            ct,
            0,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        HUF_zeroIndex1(bitC);
        u_0 = 1;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - kUnroll - u_0) as isize) as u32,
                ct,
                1,
                1,
            );
            u_0 += 1;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll - kUnroll) as isize) as u32,
            ct,
            1,
            kLastFast,
        );
        HUF_mergeIndex1(bitC);
        HUF_flushBits(bitC, kFastFlush);
        n -= 2 * kUnroll;
    }
}
fn HUF_tightCompressBound(mut srcSize: size_t, mut tableLog: size_t) -> size_t {
    ((srcSize * tableLog) >> 3).wrapping_add(8)
}
#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    let tableLog = (HUF_readCTableHeader(CTable)).tableLog as u32;
    let mut ct = CTable.offset(1);
    let mut ip = src as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut bitC = HUF_CStream_t {
        bitContainer: [0; 2],
        bitPos: [0; 2],
        startPtr: core::ptr::null_mut::<u8>(),
        ptr: core::ptr::null_mut::<u8>(),
        endPtr: core::ptr::null_mut::<u8>(),
    };
    if dstSize < 8 {
        return 0;
    }
    let mut op = ostart;
    let initErr = HUF_initCStream(
        &mut bitC,
        op as *mut core::ffi::c_void,
        oend.offset_from(op) as core::ffi::c_long as size_t,
    );
    if ERR_isError(initErr) != 0 {
        return 0;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as size_t) || tableLog > 11 {
        HUF_compress1X_usingCTable_internal_body_loop(
            &mut bitC,
            ip,
            srcSize,
            ct,
            if MEM_32bits() != 0 { 2 } else { 4 },
            0,
            0,
        );
    } else if MEM_32bits() != 0 {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 2, 1, 0);
            }
            8..=10 => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 2, 1, 1);
            }
            7 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 3, 1, 1);
            }
        }
    } else {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 5, 1, 0);
            }
            10 => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 5, 1, 1);
            }
            9 => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 6, 1, 0);
            }
            8 => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 7, 1, 0);
            }
            7 => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 8, 1, 0);
            }
            6 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(&mut bitC, ip, srcSize, ct, 9, 1, 1);
            }
        }
    }
    HUF_closeCStream(&mut bitC)
}
unsafe fn HUF_compress1X_usingCTable_internal_bmi2(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}
unsafe fn HUF_compress1X_usingCTable_internal_default(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}
unsafe fn HUF_compress1X_usingCTable_internal(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        return HUF_compress1X_usingCTable_internal_bmi2(dst, dstSize, src, srcSize, CTable);
    }
    HUF_compress1X_usingCTable_internal_default(dst, dstSize, src, srcSize, CTable)
}
pub unsafe fn HUF_compress1X_usingCTable(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: core::ffi::c_int,
) -> size_t {
    HUF_compress1X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}
unsafe fn HUF_compress4X_usingCTable_internal(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: core::ffi::c_int,
) -> size_t {
    let segmentSize = srcSize.wrapping_add(3) / 4;
    let mut ip = src as *const u8;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if dstSize < (6 + 1 + 1 + 1 + 8) as size_t {
        return 0;
    }
    if srcSize < 12 {
        return 0;
    }
    op = op.offset(6);
    let cSize = HUF_compress1X_usingCTable_internal(
        op as *mut core::ffi::c_void,
        oend.offset_from(op) as core::ffi::c_long as size_t,
        ip as *const core::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 || cSize > 65535 {
        return 0;
    }
    MEM_writeLE16(ostart as *mut core::ffi::c_void, cSize as u16);
    op = op.offset(cSize as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_0 = HUF_compress1X_usingCTable_internal(
        op as *mut core::ffi::c_void,
        oend.offset_from(op) as core::ffi::c_long as size_t,
        ip as *const core::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_0) != 0 {
        return cSize_0;
    }
    if cSize_0 == 0 || cSize_0 > 65535 {
        return 0;
    }
    MEM_writeLE16(ostart.offset(2) as *mut core::ffi::c_void, cSize_0 as u16);
    op = op.offset(cSize_0 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_1 = HUF_compress1X_usingCTable_internal(
        op as *mut core::ffi::c_void,
        oend.offset_from(op) as core::ffi::c_long as size_t,
        ip as *const core::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_1) != 0 {
        return cSize_1;
    }
    if cSize_1 == 0 || cSize_1 > 65535 {
        return 0;
    }
    MEM_writeLE16(ostart.offset(4) as *mut core::ffi::c_void, cSize_1 as u16);
    op = op.offset(cSize_1 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_2 = HUF_compress1X_usingCTable_internal(
        op as *mut core::ffi::c_void,
        oend.offset_from(op) as core::ffi::c_long as size_t,
        ip as *const core::ffi::c_void,
        iend.offset_from(ip) as core::ffi::c_long as size_t,
        CTable,
        flags,
    );
    if ERR_isError(cSize_2) != 0 {
        return cSize_2;
    }
    if cSize_2 == 0 || cSize_2 > 65535 {
        return 0;
    }
    op = op.offset(cSize_2 as isize);
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
pub unsafe fn HUF_compress4X_usingCTable(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: core::ffi::c_int,
) -> size_t {
    HUF_compress4X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}
unsafe fn HUF_compressCTable_internal(
    ostart: *mut u8,
    mut op: *mut u8,
    oend: *mut u8,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut nbStreams: HUF_nbStreams_e,
    mut CTable: *const HUF_CElt,
    flags: core::ffi::c_int,
) -> size_t {
    let cSize = if nbStreams as core::ffi::c_uint
        == HUF_singleStream as core::ffi::c_int as core::ffi::c_uint
    {
        HUF_compress1X_usingCTable_internal(
            op as *mut core::ffi::c_void,
            oend.offset_from(op) as core::ffi::c_long as size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut core::ffi::c_void,
            oend.offset_from(op) as core::ffi::c_long as size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    };
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 {
        return 0;
    }
    op = op.offset(cSize as isize);
    if op.offset_from(ostart) as core::ffi::c_long as size_t >= srcSize.wrapping_sub(1) {
        return 0;
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: core::ffi::c_int = 4096;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: core::ffi::c_int = 10;
pub unsafe fn HUF_cardinality(
    mut count: *const core::ffi::c_uint,
    mut maxSymbolValue: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let mut cardinality = 0 as core::ffi::c_uint;
    let mut i: core::ffi::c_uint = 0;
    i = 0;
    while i < maxSymbolValue.wrapping_add(1) {
        if *count.offset(i as isize) != 0 {
            cardinality = cardinality.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    cardinality
}
pub fn HUF_minTableLog(mut symbolCardinality: core::ffi::c_uint) -> core::ffi::c_uint {
    (ZSTD_highbit32(symbolCardinality)).wrapping_add(1)
}
pub unsafe fn HUF_optimalTableLog(
    mut maxTableLog: core::ffi::c_uint,
    mut srcSize: size_t,
    mut maxSymbolValue: core::ffi::c_uint,
    mut workSpace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
    mut table: *mut HUF_CElt,
    mut count: *const core::ffi::c_uint,
    mut flags: core::ffi::c_int,
) -> core::ffi::c_uint {
    if flags & HUF_flags_optimalDepth as core::ffi::c_int == 0 {
        return FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue, 1);
    }
    let mut dst = (workSpace as *mut u8)
        .offset(::core::mem::size_of::<HUF_WriteCTableWksp>() as core::ffi::c_ulong as isize);
    let mut dstSize =
        wkspSize.wrapping_sub(::core::mem::size_of::<HUF_WriteCTableWksp>() as core::ffi::c_ulong);
    let mut hSize: size_t = 0;
    let mut newSize: size_t = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = (!(0) as size_t).wrapping_sub(1);
    let mut optLog = maxTableLog;
    let mut optLogGuess: core::ffi::c_uint = 0;
    optLogGuess = minTableLog;
    while optLogGuess <= maxTableLog {
        let mut maxBits = HUF_buildCTable_wksp(
            table,
            count,
            maxSymbolValue,
            optLogGuess,
            workSpace,
            wkspSize,
        );
        if ERR_isError(maxBits) == 0 {
            if maxBits < optLogGuess as size_t && optLogGuess > minTableLog {
                break;
            }
            hSize = HUF_writeCTable_wksp(
                dst as *mut core::ffi::c_void,
                dstSize,
                table,
                maxSymbolValue,
                maxBits as u32,
                workSpace,
                wkspSize,
            );
            if ERR_isError(hSize) == 0 {
                newSize =
                    (HUF_estimateCompressedSize(table, count, maxSymbolValue)).wrapping_add(hSize);
                if newSize > optSize.wrapping_add(1) {
                    break;
                }
                if newSize < optSize {
                    optSize = newSize;
                    optLog = optLogGuess;
                }
            }
        }
        optLogGuess = optLogGuess.wrapping_add(1);
    }
    optLog
}
unsafe fn HUF_compress_internal(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: core::ffi::c_uint,
    mut huffLog: core::ffi::c_uint,
    mut nbStreams: HUF_nbStreams_e,
    mut workSpace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
    mut oldHufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: core::ffi::c_int,
) -> size_t {
    let table = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<size_t>() as core::ffi::c_ulong,
    ) as *mut HUF_compress_tables_t;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if wkspSize < ::core::mem::size_of::<HUF_compress_tables_t>() as core::ffi::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as core::ffi::c_int) as size_t;
    }
    if srcSize == 0 {
        return 0;
    }
    if dstSize == 0 {
        return 0;
    }
    if srcSize > HUF_BLOCKSIZE_MAX as size_t {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if huffLog > HUF_TABLELOG_MAX as core::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as core::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as core::ffi::c_int) as size_t;
    }
    if maxSymbolValue == 0 {
        maxSymbolValue = HUF_SYMBOLVALUE_MAX as core::ffi::c_uint;
    }
    if huffLog == 0 {
        huffLog = HUF_TABLELOG_DEFAULT as core::ffi::c_uint;
    }
    if flags & HUF_flags_preferRepeat as core::ffi::c_int != 0
        && !repeat.is_null()
        && *repeat as core::ffi::c_uint == HUF_repeat_valid as core::ffi::c_int as core::ffi::c_uint
    {
        return HUF_compressCTable_internal(
            ostart,
            op,
            oend,
            src,
            srcSize,
            nbStreams,
            oldHufTable,
            flags,
        );
    }
    if flags & HUF_flags_suspectUncompressible as core::ffi::c_int != 0
        && srcSize
            >= (SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE * SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO) as size_t
    {
        let mut largestTotal = 0 as size_t;
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const u8 as *const core::ffi::c_void,
            4096,
        ) as size_t;
        if ERR_isError(largestBegin) != 0 {
            return largestBegin;
        }
        largestTotal = largestTotal.wrapping_add(largestBegin);
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const u8).offset(srcSize as isize).offset(-(4096)) as *const core::ffi::c_void,
            4096,
        ) as size_t;
        if ERR_isError(largestEnd) != 0 {
            return largestEnd;
        }
        largestTotal = largestTotal.wrapping_add(largestEnd);
        if largestTotal <= (((2 * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE) >> 7) + 4) as size_t {
            return 0;
        }
    }
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const u8 as *const core::ffi::c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[u32; 1024]>() as core::ffi::c_ulong,
    );
    if ERR_isError(largest) != 0 {
        return largest;
    }
    if largest == srcSize {
        *ostart = *(src as *const u8).offset(0);
        return 1;
    }
    if largest <= (srcSize >> 7).wrapping_add(4) {
        return 0;
    }
    if !repeat.is_null()
        && *repeat as core::ffi::c_uint == HUF_repeat_check as core::ffi::c_int as core::ffi::c_uint
        && HUF_validateCTable(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue) == 0
    {
        *repeat = HUF_repeat_none;
    }
    if flags & HUF_flags_preferRepeat as core::ffi::c_int != 0
        && !repeat.is_null()
        && *repeat as core::ffi::c_uint != HUF_repeat_none as core::ffi::c_int as core::ffi::c_uint
    {
        return HUF_compressCTable_internal(
            ostart,
            op,
            oend,
            src,
            srcSize,
            nbStreams,
            oldHufTable,
            flags,
        );
    }
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        &mut (*table).wksps as *mut C2RustUnnamed_1 as *mut core::ffi::c_void,
        ::core::mem::size_of::<C2RustUnnamed_1>() as core::ffi::c_ulong,
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        flags,
    );
    let maxBits = HUF_buildCTable_wksp(
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.buildCTable_wksp as *mut HUF_buildCTable_wksp_tables
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() as core::ffi::c_ulong,
    );
    let _var_err__ = maxBits;
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    huffLog = maxBits as u32;
    let hSize = HUF_writeCTable_wksp(
        op as *mut core::ffi::c_void,
        dstSize,
        ((*table).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp as *mut core::ffi::c_void,
        ::core::mem::size_of::<HUF_WriteCTableWksp>() as core::ffi::c_ulong,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if !repeat.is_null()
        && *repeat as core::ffi::c_uint != HUF_repeat_none as core::ffi::c_int as core::ffi::c_uint
    {
        let oldSize =
            HUF_estimateCompressedSize(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue);
        let newSize = HUF_estimateCompressedSize(
            ((*table).CTable).as_mut_ptr(),
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        );
        if oldSize <= hSize.wrapping_add(newSize) || hSize.wrapping_add(12) >= srcSize {
            return HUF_compressCTable_internal(
                ostart,
                op,
                oend,
                src,
                srcSize,
                nbStreams,
                oldHufTable,
                flags,
            );
        }
    }
    if hSize.wrapping_add(12) >= srcSize {
        return 0;
    }
    op = op.offset(hSize as isize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }
    if !oldHufTable.is_null() {
        libc::memcpy(
            oldHufTable as *mut core::ffi::c_void,
            ((*table).CTable).as_mut_ptr() as *const core::ffi::c_void,
            ::core::mem::size_of::<[HUF_CElt; 257]>() as core::ffi::c_ulong as libc::size_t,
        );
    }
    HUF_compressCTable_internal(
        ostart,
        op,
        oend,
        src,
        srcSize,
        nbStreams,
        ((*table).CTable).as_mut_ptr(),
        flags,
    )
}
pub unsafe extern "C" fn HUF_compress1X_repeat(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: core::ffi::c_uint,
    mut huffLog: core::ffi::c_uint,
    mut workSpace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: core::ffi::c_int,
) -> size_t {
    HUF_compress_internal(
        dst,
        dstSize,
        src,
        srcSize,
        maxSymbolValue,
        huffLog,
        HUF_singleStream,
        workSpace,
        wkspSize,
        hufTable,
        repeat,
        flags,
    )
}
pub unsafe extern "C" fn HUF_compress4X_repeat(
    mut dst: *mut core::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: core::ffi::c_uint,
    mut huffLog: core::ffi::c_uint,
    mut workSpace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: core::ffi::c_int,
) -> size_t {
    HUF_compress_internal(
        dst,
        dstSize,
        src,
        srcSize,
        maxSymbolValue,
        huffLog,
        HUF_fourStreams,
        workSpace,
        wkspSize,
        hufTable,
        repeat,
        flags,
    )
}
