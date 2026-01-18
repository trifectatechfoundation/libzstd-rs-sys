use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::ptr;

use libc::size_t;

use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::entropy_common::HUF_readStats;
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{
    FSE_CTable, FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32, FSE_CTABLE_SIZE_U32,
};
use crate::lib::common::huf::{
    HUF_CElt, HUF_CTableHeader, HUF_flags_bmi2, HUF_flags_optimalDepth, HUF_flags_preferRepeat,
    HUF_flags_suspectUncompressible, HUF_repeat, HUF_repeat_check, HUF_repeat_none,
    HUF_repeat_valid, HUF_BLOCKSIZE_MAX, HUF_CTABLEBOUND, HUF_CTABLE_SIZE_ST,
    HUF_CTABLE_WORKSPACE_SIZE, HUF_SYMBOLVALUE_MAX, HUF_TABLELOG_ABSOLUTEMAX, HUF_TABLELOG_DEFAULT,
    HUF_TABLELOG_MAX, HUF_WORKSPACE_SIZE,
};
use crate::lib::common::mem::{MEM_32bits, MEM_writeLE16, MEM_writeLEST};
use crate::lib::compress::fse_compress::{
    FSE_buildCTable_wksp, FSE_compress_usingCTable, FSE_normalizeCount, FSE_optimalTableLog,
    FSE_optimalTableLog_internal, FSE_writeNCount,
};
use crate::lib::compress::hist::{HIST_count_simple, HIST_count_wksp, HIST_WKSP_SIZE_U32};

pub type nodeElt = nodeElt_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeElt_s {
    pub count: u32,
    pub parent: u16,
    pub byte: u8,
    pub nbBits: u8,
}

pub const HUF_WORKSPACE_MAX_ALIGNMENT: usize = 8;

unsafe fn HUF_alignUpWorkspace(
    workspace: *mut c_void,
    workspaceSizePtr: *mut size_t,
    align: size_t,
) -> *mut c_void {
    let mask = align - 1;
    let rem = workspace as size_t & mask;
    let add = (align - (rem)) & mask;
    let aligned = (workspace as *mut u8).add(add);

    debug_assert!((align & (align - 1)) == 0); /* pow 2 */
    debug_assert!(align <= HUF_WORKSPACE_MAX_ALIGNMENT);

    if *workspaceSizePtr >= add {
        debug_assert!(add < align);
        debug_assert!(((aligned as size_t) & mask) == 0);
        *workspaceSizePtr -= add;
        aligned as *mut c_void
    } else {
        *workspaceSizePtr = 0;
        core::ptr::null_mut()
    }
}

pub const MAX_FSE_TABLELOG_FOR_HUFF_HEADER: usize = 6;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CompressWeightsWksp {
    pub CTable:
        [FSE_CTable; FSE_CTABLE_SIZE_U32(MAX_FSE_TABLELOG_FOR_HUFF_HEADER, HUF_TABLELOG_MAX)],
    pub scratchBuffer: [u32; FSE_BUILD_CTABLE_WORKSPACE_SIZE_U32(
        HUF_TABLELOG_MAX,
        MAX_FSE_TABLELOG_FOR_HUFF_HEADER,
    )],
    pub count: [c_uint; HUF_TABLELOG_MAX + 1],
    pub norm: [i16; HUF_TABLELOG_MAX + 1],
}

unsafe fn HUF_compressWeights(
    dst: *mut c_void,
    dstSize: size_t,
    weightTable: *const c_void,
    wtSize: size_t,
    workspace: *mut c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(dstSize);
    let mut maxSymbolValue = HUF_TABLELOG_MAX as c_uint;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as u32;
    let wksp = HUF_alignUpWorkspace(workspace, &mut workspaceSize, align_of::<u32>())
        as *mut HUF_CompressWeightsWksp;
    if workspaceSize < size_of::<HUF_CompressWeightsWksp>() {
        return Error::GENERIC.to_error_code();
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
    if ERR_isError(_var_err__) {
        return _var_err__;
    }
    let hSize = FSE_writeNCount(
        op as *mut c_void,
        oend.offset_from_unsigned(op),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(hSize) {
        return hSize;
    }
    op = op.add(hSize);
    let _var_err___0 = FSE_buildCTable_wksp(
        ((*wksp).CTable).as_mut_ptr(),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut c_void,
        size_of::<[u32; 41]>(),
    );
    if ERR_isError(_var_err___0) {
        return _var_err___0;
    }
    let cSize = FSE_compress_usingCTable(
        op as *mut c_void,
        oend.offset_from_unsigned(op),
        weightTable,
        wtSize,
        ((*wksp).CTable).as_mut_ptr(),
    );
    if ERR_isError(cSize) {
        return cSize;
    }
    if cSize == 0 {
        return 0;
    }
    op = op.add(cSize);
    op.offset_from_unsigned(ostart)
}

fn HUF_getNbBits(elt: HUF_CElt) -> size_t {
    elt & 0xff as c_int as HUF_CElt
}

fn HUF_getNbBitsFast(elt: HUF_CElt) -> size_t {
    elt
}

fn HUF_getValue(elt: HUF_CElt) -> size_t {
    elt & !(0xff as c_int as size_t)
}

fn HUF_getValueFast(elt: HUF_CElt) -> size_t {
    elt
}

unsafe fn HUF_setNbBits(elt: *mut HUF_CElt, nbBits: size_t) {
    debug_assert!(nbBits <= HUF_TABLELOG_ABSOLUTEMAX);
    *elt = nbBits;
}

unsafe fn HUF_setValue(elt: *mut HUF_CElt, value: size_t) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 {
        debug_assert!((value >> nbBits) == 0);
        *elt |= value << (core::mem::size_of::<HUF_CElt>() * 8 - nbBits);
    }
}

pub(super) unsafe fn HUF_readCTableHeader(ctable: *const HUF_CElt) -> HUF_CTableHeader {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; _],
    };
    libc::memcpy(
        &mut header as *mut HUF_CTableHeader as *mut c_void,
        ctable as *const c_void,
        size_of::<HUF_CTableHeader>() as c_ulong as libc::size_t,
    );
    header
}

unsafe fn HUF_writeCTableHeader(ctable: *mut HUF_CElt, tableLog: u32, maxSymbolValue: u32) {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; _],
    };
    const {
        assert!(core::mem::size_of::<HUF_CElt>() == core::mem::size_of::<HUF_CTableHeader>());
    }
    ptr::write_bytes(
        &mut header as *mut HUF_CTableHeader as *mut u8,
        0,
        size_of::<HUF_CTableHeader>(),
    );
    debug_assert!(tableLog < 256);
    header.tableLog = tableLog as u8;
    debug_assert!(maxSymbolValue < 256);
    header.maxSymbolValue = maxSymbolValue as u8;
    libc::memcpy(
        ctable as *mut c_void,
        &mut header as *mut HUF_CTableHeader as *const c_void,
        size_of::<HUF_CTableHeader>() as c_ulong as libc::size_t,
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_WriteCTableWksp {
    pub wksp: HUF_CompressWeightsWksp,
    pub bitsToWeight: [u8; HUF_TABLELOG_MAX + 1],
    pub huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize],
}

pub unsafe fn HUF_writeCTable_wksp(
    dst: *mut c_void,
    maxDstSize: size_t,
    CTable: *const HUF_CElt,
    maxSymbolValue: c_uint,
    huffLog: c_uint,
    workspace: *mut c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ct = CTable.add(1);
    let op = dst as *mut u8;
    let mut n: u32 = 0;
    let wksp = HUF_alignUpWorkspace(workspace, &mut workspaceSize, align_of::<u32>() as size_t)
        as *mut HUF_WriteCTableWksp;

    const {
        assert!(HUF_CTABLE_WORKSPACE_SIZE >= core::mem::size_of::<HUF_WriteCTableWksp>());
    }

    debug_assert!(HUF_readCTableHeader(CTable).maxSymbolValue as c_uint == maxSymbolValue);
    debug_assert!(HUF_readCTableHeader(CTable).tableLog as c_uint == huffLog);

    if workspaceSize < size_of::<HUF_WriteCTableWksp>() {
        return Error::GENERIC.to_error_code();
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX {
        return Error::maxSymbolValue_tooLarge.to_error_code();
    }
    *((*wksp).bitsToWeight).as_mut_ptr() = 0;
    n = 1;
    while n < huffLog + 1 {
        *((*wksp).bitsToWeight).as_mut_ptr().offset(n as isize) = (huffLog + 1 - n) as u8;
        n += 1;
    }
    n = 0;
    while n < maxSymbolValue {
        *((*wksp).huffWeight).as_mut_ptr().offset(n as isize) = *((*wksp).bitsToWeight)
            .as_mut_ptr()
            .add(HUF_getNbBits(*ct.offset(n as isize)));
        n += 1;
    }
    if maxDstSize < 1 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    let hSize = HUF_compressWeights(
        op.add(1) as *mut c_void,
        maxDstSize - 1,
        ((*wksp).huffWeight).as_mut_ptr() as *const c_void,
        maxSymbolValue as size_t,
        &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut c_void,
        size_of::<HUF_CompressWeightsWksp>(),
    );
    if ERR_isError(hSize) {
        return hSize;
    }
    if (hSize > 1) as c_int & (hSize < (maxSymbolValue / 2) as size_t) as c_int != 0 {
        *op = hSize as u8;
        return hSize + 1;
    }
    if maxSymbolValue > (256 - 128) as c_uint {
        return Error::GENERIC.to_error_code();
    }
    if (maxSymbolValue.div_ceil(2) + 1) as size_t > maxDstSize {
        return Error::dstSize_tooSmall.to_error_code();
    }
    *op = ((128 as c_uint) + (maxSymbolValue - 1)) as u8;
    *((*wksp).huffWeight)
        .as_mut_ptr()
        .offset(maxSymbolValue as isize) = 0;
    n = 0;
    while n < maxSymbolValue {
        *op.offset(((n / 2) + 1) as isize) =
            (((*((*wksp).huffWeight).as_mut_ptr().offset(n as isize) as c_int) << 4)
                + *((*wksp).huffWeight).as_mut_ptr().offset((n + 1) as isize) as c_int)
                as u8;
        n += 2;
    }
    (maxSymbolValue.div_ceil(2) + 1) as size_t
}

pub unsafe fn HUF_readCTable(
    CTable: *mut HUF_CElt,
    maxSymbolValuePtr: *mut c_uint,
    src: *const c_void,
    srcSize: size_t,
    hasZeroWeights: *mut c_uint,
) -> size_t {
    let src = core::slice::from_raw_parts(src.cast(), srcSize);

    let mut huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize + 1] =
        [0; HUF_SYMBOLVALUE_MAX as usize + 1];
    let mut rankVal: [u32; HUF_TABLELOG_ABSOLUTEMAX + 1] = [0; HUF_TABLELOG_ABSOLUTEMAX + 1];
    let mut tableLog = 0;
    let mut nbSymbols = 0;
    let ct = CTable.add(1);
    let readSize = HUF_readStats(
        &mut huffWeight,
        (255 + 1) as size_t,
        &mut rankVal,
        &mut nbSymbols,
        &mut tableLog,
        src,
    );
    if ERR_isError(readSize) {
        return readSize;
    }
    *hasZeroWeights = (*rankVal.as_mut_ptr() > 0) as c_int as c_uint;
    if tableLog > HUF_TABLELOG_MAX as u32 {
        return Error::tableLog_tooLarge.to_error_code();
    }
    if nbSymbols > (*maxSymbolValuePtr) + 1 {
        return Error::maxSymbolValue_tooSmall.to_error_code();
    }
    *maxSymbolValuePtr = nbSymbols - 1;
    HUF_writeCTableHeader(CTable, tableLog, *maxSymbolValuePtr);
    let mut n: u32 = 0;
    let mut nextRankStart = 0u32;
    n = 1;
    while n <= tableLog {
        let curr = nextRankStart;
        nextRankStart += *rankVal.as_mut_ptr().offset(n as isize) << (n - 1);
        *rankVal.as_mut_ptr().offset(n as isize) = curr;
        n += 1;
    }
    let mut n_0: u32 = 0;
    n_0 = 0;
    while n_0 < nbSymbols {
        let w = *huffWeight.as_mut_ptr().offset(n_0 as isize) as u32;
        HUF_setNbBits(
            ct.offset(n_0 as isize),
            ((tableLog + 1 - w) as u8 as c_int & -((w != 0) as c_int) as c_int) as size_t,
        );
        n_0 += 1;
    }
    let mut nbPerRank: [u16; HUF_TABLELOG_MAX + 2] = [0; HUF_TABLELOG_MAX + 2];
    let mut valPerRank: [u16; HUF_TABLELOG_MAX + 2] = [0; HUF_TABLELOG_MAX + 2];
    let mut n_1: u32 = 0;
    n_1 = 0;
    while n_1 < nbSymbols {
        let fresh0 = &mut (*nbPerRank
            .as_mut_ptr()
            .add(HUF_getNbBits(*ct.offset(n_1 as isize))));
        *fresh0 += 1;
        n_1 += 1;
    }
    *valPerRank.as_mut_ptr().offset((tableLog + 1) as isize) = 0;
    let mut min = 0;
    let mut n_2: u32 = 0;
    n_2 = tableLog;
    while n_2 > 0 {
        *valPerRank.as_mut_ptr().offset(n_2 as isize) = min;
        min = (min as c_int + *nbPerRank.as_mut_ptr().offset(n_2 as isize) as c_int) as u16;
        min = (min as c_int >> 1) as u16;
        n_2 -= 1;
    }
    let mut n_3: u32 = 0;
    n_3 = 0;
    while n_3 < nbSymbols {
        let fresh1 = &mut (*valPerRank
            .as_mut_ptr()
            .add(HUF_getNbBits(*ct.offset(n_3 as isize))));
        let fresh2 = *fresh1;
        *fresh1 += 1;
        HUF_setValue(ct.offset(n_3 as isize), fresh2 as size_t);
        n_3 += 1;
    }
    readSize
}

pub unsafe fn HUF_getNbBitsFromCTable(CTable: *const HUF_CElt, symbolValue: u32) -> u32 {
    let ct = CTable.add(1);
    debug_assert!(symbolValue <= HUF_SYMBOLVALUE_MAX);
    if symbolValue > (HUF_readCTableHeader(CTable)).maxSymbolValue as u32 {
        return 0;
    }
    HUF_getNbBits(*ct.offset(symbolValue as isize)) as u32
}

unsafe fn HUF_setMaxHeight(huffNode: *mut nodeElt, lastNonNull: u32, targetNbBits: u32) -> u32 {
    let largestBits = (*huffNode.offset(lastNonNull as isize)).nbBits as u32;
    if largestBits <= targetNbBits {
        return largestBits;
    }
    let mut totalCost: c_int = 0;
    let baseCost = (1 << (largestBits - (targetNbBits))) as u32;
    let mut n = lastNonNull as c_int;
    while (*huffNode.offset(n as isize)).nbBits as u32 > targetNbBits {
        totalCost += (baseCost
            - (1 << (largestBits - (*huffNode.offset(n as isize)).nbBits as u32)))
            as c_int;
        (*huffNode.offset(n as isize)).nbBits = targetNbBits as u8;
        n -= 1;
    }
    debug_assert!((*huffNode.offset(n as isize)).nbBits as u32 <= targetNbBits);
    while (*huffNode.offset(n as isize)).nbBits as u32 == targetNbBits {
        n -= 1;
    }
    debug_assert!(((totalCost as u32) & (baseCost - 1)) == 0);
    totalCost >>= largestBits - (targetNbBits);
    debug_assert!(totalCost > 0);
    let noSymbol = 0xf0f0f0f0 as c_uint;
    let mut rankLast: [u32; HUF_TABLELOG_MAX + 2] = [0; HUF_TABLELOG_MAX + 2];
    ptr::write_bytes(
        rankLast.as_mut_ptr() as *mut u8,
        0xf0,
        size_of::<[u32; HUF_TABLELOG_MAX + 2]>(),
    );
    let mut currentNbBits = targetNbBits;
    let mut pos: c_int = 0;
    pos = n;
    while pos >= 0 {
        if ((*huffNode.offset(pos as isize)).nbBits as u32) < currentNbBits {
            currentNbBits = (*huffNode.offset(pos as isize)).nbBits as u32;
            *rankLast
                .as_mut_ptr()
                .offset((targetNbBits - (currentNbBits)) as isize) = pos as u32;
        }
        pos -= 1;
    }
    while totalCost > 0 {
        let mut nBitsToDecrease = (ZSTD_highbit32(totalCost as u32)) + 1;
        debug_assert!(nBitsToDecrease as usize <= (HUF_TABLELOG_MAX + 1));
        while nBitsToDecrease > 1 {
            let highPos = *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize);
            let lowPos = *rankLast.as_mut_ptr().offset((nBitsToDecrease - 1) as isize);
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
            nBitsToDecrease -= 1;
        }
        debug_assert!(
            *rankLast.as_ptr().offset(nBitsToDecrease as isize) != noSymbol || nBitsToDecrease == 1
        );

        while nBitsToDecrease <= HUF_TABLELOG_MAX as u32
            && *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) == noSymbol
        {
            nBitsToDecrease += 1;
        }
        debug_assert!(*rankLast.as_ptr().offset(nBitsToDecrease as isize) != noSymbol);
        totalCost -= 1 << (nBitsToDecrease - 1);
        let fresh3 = &mut (*huffNode
            .offset(*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) as isize))
        .nbBits;
        *fresh3 += 1;
        if *rankLast.as_mut_ptr().offset((nBitsToDecrease - 1) as isize) == noSymbol {
            *rankLast.as_mut_ptr().offset((nBitsToDecrease - 1) as isize) =
                *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize);
        }
        if *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) == 0 {
            *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) = noSymbol;
        } else {
            let fresh4 = &mut (*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize));
            *fresh4 -= 1;
            if (*huffNode.offset(*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) as isize))
                .nbBits as u32
                != targetNbBits - (nBitsToDecrease)
            {
                *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) = noSymbol;
            }
        }
    }
    while totalCost < 0 {
        if *rankLast.as_mut_ptr().add(1) == noSymbol {
            while (*huffNode.offset(n as isize)).nbBits as u32 == targetNbBits {
                n -= 1;
            }
            let fresh5 = &mut (*huffNode.offset((n + 1) as isize)).nbBits;
            *fresh5 -= 1;
            debug_assert!(n >= 0);
            *rankLast.as_mut_ptr().add(1) = (n + 1) as u32;
            totalCost += 1;
        } else {
            let fresh6 =
                &mut (*huffNode.offset(((*rankLast.as_mut_ptr().add(1)) + 1) as isize)).nbBits;
            *fresh6 -= 1;
            let fresh7 = &mut (*rankLast.as_mut_ptr().add(1));
            *fresh7 += 1;
            totalCost += 1;
        }
    }
    targetNbBits
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rankPos {
    pub base: u16,
    pub curr: u16,
}

pub type huffNodeTable = [nodeElt; 2 * (HUF_SYMBOLVALUE_MAX as usize + 1)];

pub const RANK_POSITION_TABLE_SIZE: usize = 192;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_buildCTable_wksp_tables {
    pub huffNodeTbl: huffNodeTable,
    pub rankPosition: [rankPos; RANK_POSITION_TABLE_SIZE],
}

pub const RANK_POSITION_MAX_COUNT_LOG: c_int = 32;
pub const RANK_POSITION_LOG_BUCKETS_BEGIN: c_int =
    (RANK_POSITION_TABLE_SIZE as c_int) - 1 - RANK_POSITION_MAX_COUNT_LOG - 1;
pub const RANK_POSITION_DISTINCT_COUNT_CUTOFF: c_uint = (RANK_POSITION_LOG_BUCKETS_BEGIN as c_uint)
    + (ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN as u32));

unsafe fn HUF_getIndex(count: u32) -> u32 {
    if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        (ZSTD_highbit32(count)) + (RANK_POSITION_LOG_BUCKETS_BEGIN as c_uint)
    }
}

unsafe fn HUF_swapNodes(a: *mut nodeElt, b: *mut nodeElt) {
    core::ptr::swap(a, b);
}

unsafe fn HUF_isSorted(huffNode: *mut nodeElt, maxSymbolValue1: u32) -> c_int {
    let mut i = 1;
    while i < maxSymbolValue1 {
        if (*huffNode.offset(i as isize)).count > (*huffNode.offset((i - 1) as isize)).count {
            return 0;
        }
        i += 1;
    }
    1
}

#[inline(always)]
unsafe fn HUF_insertionSort(mut huffNode: *mut nodeElt, low: c_int, high: c_int) {
    let mut i: c_int = 0;
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

unsafe fn HUF_quickSortPartition(arr: *mut nodeElt, low: c_int, high: c_int) -> c_int {
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

unsafe fn HUF_simpleQuickSort(arr: *mut nodeElt, mut low: c_int, mut high: c_int) {
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
    huffNode: *mut nodeElt,
    count: *const c_uint,
    maxSymbolValue: u32,
    rankPosition: *mut rankPos,
) {
    let mut n: u32 = 0;
    let maxSymbolValue1 = maxSymbolValue + 1;
    ptr::write_bytes(rankPosition as *mut u8, 0, size_of::<rankPos>() * 192);
    n = 0;
    while n < maxSymbolValue1 {
        let lowerRank = HUF_getIndex(*count.offset(n as isize));
        let fresh8 = &mut (*rankPosition.offset(lowerRank as isize)).base;
        debug_assert!((lowerRank as usize) < (RANK_POSITION_TABLE_SIZE - 1));
        *fresh8 += 1;
        n += 1;
    }
    debug_assert!((*rankPosition.add(RANK_POSITION_TABLE_SIZE - 1)).base == 0);

    n = (RANK_POSITION_TABLE_SIZE - 1) as u32;
    while n > 0 {
        let fresh9 = &mut (*rankPosition.offset((n - 1) as isize)).base;
        *fresh9 = (*fresh9 as c_int + (*rankPosition.offset(n as isize)).base as c_int) as u16;
        (*rankPosition.offset((n - 1) as isize)).curr =
            (*rankPosition.offset((n - 1) as isize)).base;
        n -= 1;
    }
    n = 0;
    while n < maxSymbolValue1 {
        let c = *count.offset(n as isize);
        let r = (HUF_getIndex(c)) + 1;
        let fresh10 = &mut (*rankPosition.offset(r as isize)).curr;
        let fresh11 = *fresh10;
        *fresh10 += 1;
        let pos = fresh11 as u32;
        debug_assert!(pos < maxSymbolValue1);
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as u8;
        n += 1;
    }
    n = RANK_POSITION_DISTINCT_COUNT_CUTOFF;
    while n < (RANK_POSITION_TABLE_SIZE - 1) as u32 {
        let bucketSize = (*rankPosition.offset(n as isize)).curr as c_int
            - (*rankPosition.offset(n as isize)).base as c_int;
        let bucketStartIdx = (*rankPosition.offset(n as isize)).base as u32;
        if bucketSize > 1 {
            debug_assert!(bucketStartIdx < maxSymbolValue1);
            HUF_simpleQuickSort(huffNode.offset(bucketStartIdx as isize), 0, bucketSize - 1);
        }
        n += 1;
    }
    debug_assert!(HUF_isSorted(huffNode, maxSymbolValue1) != 0);
}

pub const STARTNODE: c_int = HUF_SYMBOLVALUE_MAX as i32 + 1;

unsafe fn HUF_buildTree(huffNode: *mut nodeElt, maxSymbolValue: u32) -> c_int {
    let huffNode0 = huffNode.sub(1);
    let mut nonNullRank: c_int = 0;
    let mut lowS: c_int = 0;
    let mut lowN: c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut n: c_int = 0;
    let mut nodeRoot: c_int = 0;
    nonNullRank = maxSymbolValue as c_int;
    while (*huffNode.offset(nonNullRank as isize)).count == 0 {
        nonNullRank -= 1;
    }
    lowS = nonNullRank;
    nodeRoot = nodeNb + lowS - 1;
    lowN = nodeNb;
    (*huffNode.offset(nodeNb as isize)).count =
        ((*huffNode.offset(lowS as isize)).count) + ((*huffNode.offset((lowS - 1) as isize)).count);
    let fresh12 = &mut (*huffNode.offset((lowS - 1) as isize)).parent;
    *fresh12 = nodeNb as u16;
    (*huffNode.offset(lowS as isize)).parent = *fresh12;
    nodeNb += 1;
    lowS -= 2;
    n = nodeNb;
    while n <= nodeRoot {
        (*huffNode.offset(n as isize)).count = 1 << 30;
        n += 1;
    }
    (*huffNode0).count = 1 << 31;
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
        (*huffNode.offset(nodeNb as isize)).count =
            ((*huffNode.offset(n1 as isize)).count) + ((*huffNode.offset(n2 as isize)).count);
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
        .nbBits as c_int
            + 1) as u8;
        n -= 1;
    }
    n = 0;
    while n <= nonNullRank {
        (*huffNode.offset(n as isize)).nbBits = ((*huffNode
            .offset((*huffNode.offset(n as isize)).parent as isize))
        .nbBits as c_int
            + 1) as u8;
        n += 1;
    }
    nonNullRank
}

unsafe fn HUF_buildCTableFromTree(
    CTable: *mut HUF_CElt,
    huffNode: *const nodeElt,
    nonNullRank: c_int,
    maxSymbolValue: u32,
    maxNbBits: u32,
) {
    let ct = CTable.add(1);
    let mut n: c_int = 0;
    let mut nbPerRank: [u16; HUF_TABLELOG_MAX + 1] = [0; HUF_TABLELOG_MAX + 1];
    let mut valPerRank: [u16; HUF_TABLELOG_MAX + 1] = [0; HUF_TABLELOG_MAX + 1];
    let alphabetSize = (maxSymbolValue + 1) as c_int;
    n = 0;
    while n <= nonNullRank {
        let fresh18 = &mut (*nbPerRank
            .as_mut_ptr()
            .offset((*huffNode.offset(n as isize)).nbBits as isize));
        *fresh18 += 1;
        n += 1;
    }
    let mut min = 0;
    n = maxNbBits as c_int;
    while n > 0 {
        *valPerRank.as_mut_ptr().offset(n as isize) = min;
        min = (min as c_int + *nbPerRank.as_mut_ptr().offset(n as isize) as c_int) as u16;
        min = (min as c_int >> 1) as u16;
        n -= 1;
    }
    n = 0;
    while n < alphabetSize {
        HUF_setNbBits(
            ct.offset((*huffNode.offset(n as isize)).byte as c_int as isize),
            (*huffNode.offset(n as isize)).nbBits as size_t,
        );
        n += 1;
    }
    n = 0;
    while n < alphabetSize {
        let fresh19 = &mut (*valPerRank
            .as_mut_ptr()
            .add(HUF_getNbBits(*ct.offset(n as isize))));
        let fresh20 = *fresh19;
        *fresh19 += 1;
        HUF_setValue(ct.offset(n as isize), fresh20 as size_t);
        n += 1;
    }
    HUF_writeCTableHeader(CTable, maxNbBits, maxSymbolValue);
}

pub unsafe fn HUF_buildCTable_wksp(
    CTable: *mut HUF_CElt,
    count: *const c_uint,
    maxSymbolValue: u32,
    mut maxNbBits: u32,
    workSpace: *mut c_void,
    mut wkspSize: size_t,
) -> size_t {
    let wksp_tables = HUF_alignUpWorkspace(workSpace, &mut wkspSize, align_of::<u32>() as size_t)
        as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.add(1);
    let mut nonNullRank: c_int = 0;

    const {
        assert!(HUF_CTABLE_WORKSPACE_SIZE == size_of::<HUF_buildCTable_wksp_tables>());
    }

    if wkspSize < size_of::<HUF_buildCTable_wksp_tables>() {
        return Error::workSpace_tooSmall.to_error_code();
    }
    if maxNbBits == 0 {
        maxNbBits = HUF_TABLELOG_DEFAULT;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX {
        return Error::maxSymbolValue_tooLarge.to_error_code();
    }
    ptr::write_bytes(huffNode0 as *mut u8, 0, size_of::<huffNodeTable>());
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as u32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as u32 {
        return Error::GENERIC.to_error_code();
    }
    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);
    maxNbBits as size_t
}

pub unsafe fn HUF_estimateCompressedSize(
    CTable: *const HUF_CElt,
    count: *const c_uint,
    maxSymbolValue: c_uint,
) -> size_t {
    let ct = CTable.add(1);
    let mut nbBits = 0 as size_t;
    let mut s: c_int = 0;
    s = 0;
    while s <= maxSymbolValue as c_int {
        nbBits += HUF_getNbBits(*ct.offset(s as isize)) * *count.offset(s as isize) as size_t;
        s += 1;
    }
    nbBits >> 3
}

pub unsafe fn HUF_validateCTable(
    CTable: *const HUF_CElt,
    count: *const c_uint,
    maxSymbolValue: c_uint,
) -> c_int {
    let header = HUF_readCTableHeader(CTable);
    let ct = CTable.add(1);
    let mut bad = 0;
    let mut s: c_int = 0;

    debug_assert!(header.tableLog as usize <= HUF_TABLELOG_ABSOLUTEMAX);

    if (header.maxSymbolValue as c_uint) < maxSymbolValue {
        return 0;
    }
    s = 0;
    while s <= maxSymbolValue as c_int {
        bad |= (*count.offset(s as isize) != 0) as c_int
            & (HUF_getNbBits(*ct.offset(s as isize)) == 0) as c_int;
        s += 1;
    }
    (bad == 0) as c_int
}

pub fn HUF_compressBound(size: size_t) -> size_t {
    HUF_CTABLEBOUND + (size + (size >> 8) + (8))
}

pub const HUF_BITS_IN_CONTAINER: size_t = size_t::BITS as usize;

#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [size_t; 2],
    pub bitPos: [size_t; 2],
    pub startPtr: *mut u8,
    pub ptr: *mut u8,
    pub endPtr: *mut u8,
}

unsafe fn HUF_initCStream(
    bitC: *mut HUF_CStream_t,
    startPtr: *mut c_void,
    dstCapacity: size_t,
) -> size_t {
    ptr::write_bytes(bitC as *mut u8, 0, size_of::<HUF_CStream_t>());
    (*bitC).startPtr = startPtr as *mut u8;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr)
        .add(dstCapacity)
        .offset(-(size_of::<size_t>() as c_ulong as isize));
    if dstCapacity <= size_of::<size_t>() {
        return Error::dstSize_tooSmall.to_error_code();
    }
    0
}

#[inline(always)]
unsafe fn HUF_addBits(bitC: *mut HUF_CStream_t, elt: HUF_CElt, idx: c_int, kFast: c_int) {
    debug_assert!(idx <= 1);
    debug_assert!(HUF_getNbBits(elt) <= HUF_TABLELOG_ABSOLUTEMAX);
    *((*bitC).bitContainer).as_mut_ptr().offset(idx as isize) >>= HUF_getNbBits(elt);
    *((*bitC).bitContainer).as_mut_ptr().offset(idx as isize) |= if kFast != 0 {
        HUF_getValueFast(elt)
    } else {
        HUF_getValue(elt)
    };
    let fresh21 = &mut (*((*bitC).bitPos).as_mut_ptr().offset(idx as isize));
    *fresh21 = (*fresh21).wrapping_add(HUF_getNbBitsFast(elt));
    debug_assert!(
        (*((*bitC).bitPos).as_ptr().offset(idx as isize) & 0xFF) <= HUF_BITS_IN_CONTAINER
    );
}

#[inline(always)]
unsafe fn HUF_zeroIndex1(bitC: *mut HUF_CStream_t) {
    *((*bitC).bitContainer).as_mut_ptr().add(1) = 0;
    *((*bitC).bitPos).as_mut_ptr().add(1) = 0;
}

#[inline(always)]
unsafe fn HUF_mergeIndex1(bitC: *mut HUF_CStream_t) {
    debug_assert!((*((*bitC).bitPos).as_ptr().add(1) & 0xFF) < HUF_BITS_IN_CONTAINER);
    *((*bitC).bitContainer).as_mut_ptr() >>=
        *((*bitC).bitPos).as_mut_ptr().add(1) & 0xff as c_int as size_t;
    *((*bitC).bitContainer).as_mut_ptr() |= *((*bitC).bitContainer).as_mut_ptr().add(1);
    *((*bitC).bitPos).as_mut_ptr() += *((*bitC).bitPos).as_mut_ptr().add(1);
    debug_assert!((*((*bitC).bitPos).as_ptr() & 0xFF) <= HUF_BITS_IN_CONTAINER);
}

#[inline(always)]
unsafe fn HUF_flushBits(bitC: *mut HUF_CStream_t, kFast: c_int) {
    let nbBits = *((*bitC).bitPos).as_mut_ptr() & 0xff as c_int as size_t;
    let nbBytes = nbBits >> 3;
    let bitContainer = *((*bitC).bitContainer).as_mut_ptr() >> (HUF_BITS_IN_CONTAINER - (nbBits));
    *((*bitC).bitPos).as_mut_ptr() &= 7;
    debug_assert!(nbBits > 0);
    debug_assert!(nbBits <= core::mem::size_of::<size_t>() * 8);
    debug_assert!((*bitC).ptr <= (*bitC).endPtr);
    MEM_writeLEST((*bitC).ptr as *mut c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
    debug_assert!(kFast == 0 || (*bitC).ptr <= (*bitC).endPtr);
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

unsafe fn HUF_closeCStream(bitC: *mut HUF_CStream_t) -> size_t {
    HUF_addBits(bitC, HUF_endMark(), 0, 0);
    HUF_flushBits(bitC, 0);
    let nbBits = *((*bitC).bitPos).as_mut_ptr() & 0xff as c_int as size_t;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0;
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as size_t) + ((nbBits > 0) as c_int as size_t)
}

#[inline(always)]
unsafe fn HUF_encodeSymbol(
    bitCPtr: *mut HUF_CStream_t,
    symbol: u32,
    CTable: *const HUF_CElt,
    idx: c_int,
    fast: c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}

#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body_loop(
    bitC: *mut HUF_CStream_t,
    ip: *const u8,
    srcSize: size_t,
    ct: *const HUF_CElt,
    kUnroll: c_int,
    kFastFlush: c_int,
    kLastFast: c_int,
) {
    let mut n = srcSize as c_int;
    let mut rem = n % kUnroll;
    if rem > 0 {
        while rem > 0 {
            n -= 1;
            HUF_encodeSymbol(bitC, *ip.offset(n as isize) as u32, ct, 0, 0);
            rem -= 1;
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    debug_assert!(n % kUnroll == 0);

    if n % (2 * kUnroll) != 0 {
        let mut u: c_int = 0;
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
    debug_assert!(n % (2 * kUnroll) == 0);

    while n > 0 {
        let mut u_0: c_int = 0;
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
    debug_assert!(n == 0);
}

fn HUF_tightCompressBound(srcSize: size_t, tableLog: size_t) -> size_t {
    ((srcSize * tableLog) >> 3) + (8)
}

#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: *const HUF_CElt,
) -> size_t {
    let tableLog = (HUF_readCTableHeader(CTable)).tableLog as u32;
    let ct = CTable.add(1);
    let ip = src as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
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
    let op = ostart;
    let initErr = HUF_initCStream(&mut bitC, op as *mut c_void, oend.offset_from_unsigned(op));
    if ERR_isError(initErr) {
        return 0;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as size_t) || tableLog > 11 {
        HUF_compress1X_usingCTable_internal_body_loop(
            &mut bitC,
            ip,
            srcSize,
            ct,
            if MEM_32bits() { 2 } else { 4 },
            0,
            0,
        );
    } else if MEM_32bits() {
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
    debug_assert!(bitC.ptr <= bitC.endPtr);
    HUF_closeCStream(&mut bitC)
}

unsafe fn HUF_compress1X_usingCTable_internal_bmi2(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: *const HUF_CElt,
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}

unsafe fn HUF_compress1X_usingCTable_internal_default(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: *const HUF_CElt,
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}

unsafe fn HUF_compress1X_usingCTable_internal(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: *const HUF_CElt,
    flags: c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as c_int != 0 {
        return HUF_compress1X_usingCTable_internal_bmi2(dst, dstSize, src, srcSize, CTable);
    }
    HUF_compress1X_usingCTable_internal_default(dst, dstSize, src, srcSize, CTable)
}

pub unsafe fn HUF_compress1X_usingCTable(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: *const HUF_CElt,
    flags: c_int,
) -> size_t {
    HUF_compress1X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}

unsafe fn HUF_compress4X_usingCTable_internal(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: *const HUF_CElt,
    flags: c_int,
) -> size_t {
    let segmentSize = srcSize.div_ceil(4);
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut op = ostart;
    if dstSize < (6 + 1 + 1 + 1 + 8) as size_t {
        return 0;
    }
    if srcSize < 12 {
        return 0;
    }
    op = op.add(6);

    debug_assert!(op <= oend);

    let cSize = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from_unsigned(op),
        ip as *const c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize) {
        return cSize;
    }
    if cSize == 0 || cSize > 65535 {
        return 0;
    }
    MEM_writeLE16(ostart as *mut c_void, cSize as u16);
    op = op.add(cSize);
    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    let cSize_0 = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from_unsigned(op),
        ip as *const c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_0) {
        return cSize_0;
    }
    if cSize_0 == 0 || cSize_0 > 65535 {
        return 0;
    }
    MEM_writeLE16(ostart.add(2) as *mut c_void, cSize_0 as u16);
    op = op.add(cSize_0);
    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    let cSize_1 = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from_unsigned(op),
        ip as *const c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_1) {
        return cSize_1;
    }
    if cSize_1 == 0 || cSize_1 > 65535 {
        return 0;
    }
    MEM_writeLE16(ostart.add(4) as *mut c_void, cSize_1 as u16);
    op = op.add(cSize_1);
    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    debug_assert!(ip <= iend);
    let cSize_2 = HUF_compress1X_usingCTable_internal(
        op as *mut c_void,
        oend.offset_from_unsigned(op),
        ip as *const c_void,
        iend.offset_from_unsigned(ip),
        CTable,
        flags,
    );
    if ERR_isError(cSize_2) {
        return cSize_2;
    }
    if cSize_2 == 0 || cSize_2 > 65535 {
        return 0;
    }
    op = op.add(cSize_2);
    op.offset_from_unsigned(ostart)
}

pub unsafe fn HUF_compress4X_usingCTable(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: *const HUF_CElt,
    flags: c_int,
) -> size_t {
    HUF_compress4X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}

pub type HUF_nbStreams_e = c_uint;

pub const HUF_fourStreams: HUF_nbStreams_e = 1;

pub const HUF_singleStream: HUF_nbStreams_e = 0;

unsafe fn HUF_compressCTable_internal(
    ostart: *mut u8,
    mut op: *mut u8,
    oend: *mut u8,
    src: *const c_void,
    srcSize: size_t,
    nbStreams: HUF_nbStreams_e,
    CTable: *const HUF_CElt,
    flags: c_int,
) -> size_t {
    let cSize = if nbStreams as c_uint == HUF_singleStream as c_int as c_uint {
        HUF_compress1X_usingCTable_internal(
            op as *mut c_void,
            oend.offset_from_unsigned(op),
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut c_void,
            oend.offset_from_unsigned(op),
            src,
            srcSize,
            CTable,
            flags,
        )
    };
    if ERR_isError(cSize) {
        return cSize;
    }
    if cSize == 0 {
        return 0;
    }
    op = op.add(cSize);
    debug_assert!(op >= ostart);
    if op.offset_from_unsigned(ostart) >= srcSize - 1 {
        return 0;
    }
    op.offset_from_unsigned(ostart)
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union workspace_union {
    pub buildCTable_wksp: HUF_buildCTable_wksp_tables,
    pub writeCTable_wksp: HUF_WriteCTableWksp,
    pub hist_wksp: [u32; HIST_WKSP_SIZE_U32 as usize],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [c_uint; HUF_SYMBOLVALUE_MAX as usize + 1],
    pub CTable: [HUF_CElt; HUF_CTABLE_SIZE_ST(HUF_SYMBOLVALUE_MAX as usize)],
    pub wksps: workspace_union,
}

pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: c_int = 4096;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: c_int = 10;

pub unsafe fn HUF_cardinality(count: *const c_uint, maxSymbolValue: c_uint) -> c_uint {
    let mut cardinality = 0 as c_uint;
    let mut i: c_uint = 0;
    i = 0;
    while i < maxSymbolValue + 1 {
        if *count.offset(i as isize) != 0 {
            cardinality += 1;
        }
        i += 1;
    }
    cardinality
}

pub fn HUF_minTableLog(symbolCardinality: c_uint) -> c_uint {
    (ZSTD_highbit32(symbolCardinality)) + 1
}

pub unsafe fn HUF_optimalTableLog(
    maxTableLog: c_uint,
    srcSize: size_t,
    maxSymbolValue: c_uint,
    workSpace: *mut c_void,
    wkspSize: size_t,
    table: *mut HUF_CElt,
    count: *const c_uint,
    flags: c_int,
) -> c_uint {
    debug_assert!(srcSize > 1); /* Not supported, RLE should be used instead */
    debug_assert!(wkspSize >= size_of::<HUF_buildCTable_wksp_tables>());

    if flags & HUF_flags_optimalDepth as c_int == 0 {
        return FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue, 1);
    }
    let dst = (workSpace as *mut u8).offset(size_of::<HUF_WriteCTableWksp>() as c_ulong as isize);
    let dstSize = wkspSize - (size_of::<HUF_WriteCTableWksp>());
    let mut hSize: size_t = 0;
    let mut newSize: size_t = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = (!(0) as size_t) - 1;
    let mut optLog = maxTableLog;
    let mut optLogGuess: c_uint = 0;
    optLogGuess = minTableLog;
    while optLogGuess <= maxTableLog {
        let maxBits = HUF_buildCTable_wksp(
            table,
            count,
            maxSymbolValue,
            optLogGuess,
            workSpace,
            wkspSize,
        );
        if !ERR_isError(maxBits) {
            if maxBits < optLogGuess as size_t && optLogGuess > minTableLog {
                break;
            }
            hSize = HUF_writeCTable_wksp(
                dst as *mut c_void,
                dstSize,
                table,
                maxSymbolValue,
                maxBits as u32,
                workSpace,
                wkspSize,
            );
            if !ERR_isError(hSize) {
                newSize = (HUF_estimateCompressedSize(table, count, maxSymbolValue)) + (hSize);
                if newSize > optSize + 1 {
                    break;
                }
                if newSize < optSize {
                    optSize = newSize;
                    optLog = optLogGuess;
                }
            }
        }
        optLogGuess += 1;
    }
    debug_assert!(optLog as usize <= HUF_TABLELOG_MAX);
    optLog
}

unsafe fn HUF_compress_internal(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    mut maxSymbolValue: c_uint,
    mut huffLog: c_uint,
    nbStreams: HUF_nbStreams_e,
    workSpace: *mut c_void,
    mut wkspSize: size_t,
    oldHufTable: *mut HUF_CElt,
    repeat: *mut HUF_repeat,
    flags: c_int,
) -> size_t {
    let table = HUF_alignUpWorkspace(workSpace, &mut wkspSize, align_of::<size_t>() as size_t)
        as *mut HUF_compress_tables_t;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut op = ostart;

    const {
        assert!(
            size_of::<HUF_compress_tables_t>() + HUF_WORKSPACE_MAX_ALIGNMENT <= HUF_WORKSPACE_SIZE
        );
    }

    if wkspSize < size_of::<HUF_compress_tables_t>() {
        return Error::workSpace_tooSmall.to_error_code();
    }
    if srcSize == 0 {
        return 0;
    }
    if dstSize == 0 {
        return 0;
    }
    if srcSize > HUF_BLOCKSIZE_MAX {
        return Error::srcSize_wrong.to_error_code();
    }
    if huffLog > HUF_TABLELOG_MAX as c_uint {
        return Error::tableLog_tooLarge.to_error_code();
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX {
        return Error::maxSymbolValue_tooLarge.to_error_code();
    }
    if maxSymbolValue == 0 {
        maxSymbolValue = HUF_SYMBOLVALUE_MAX;
    }
    if huffLog == 0 {
        huffLog = HUF_TABLELOG_DEFAULT;
    }
    if flags & HUF_flags_preferRepeat as c_int != 0
        && !repeat.is_null()
        && *repeat as c_uint == HUF_repeat_valid as c_int as c_uint
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
    if flags & HUF_flags_suspectUncompressible as c_int != 0
        && srcSize
            >= (SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE * SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO) as size_t
    {
        let mut largestTotal = 0 as size_t;
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const u8 as *const c_void,
            4096,
        ) as size_t;
        if ERR_isError(largestBegin) {
            return largestBegin;
        }
        largestTotal += largestBegin;
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const u8).add(srcSize).sub(4096) as *const c_void,
            4096,
        ) as size_t;
        if ERR_isError(largestEnd) {
            return largestEnd;
        }
        largestTotal += largestEnd;
        if largestTotal <= (((2 * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE) >> 7) + 4) as size_t {
            return 0;
        }
    }
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const u8 as *const c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut c_void,
        size_of::<[u32; 1024]>(),
    );
    if ERR_isError(largest) {
        return largest;
    }
    if largest == srcSize {
        *ostart = *(src as *const u8);
        return 1;
    }
    if largest <= (srcSize >> 7) + (4) {
        return 0;
    }
    if !repeat.is_null()
        && *repeat as c_uint == HUF_repeat_check as c_int as c_uint
        && HUF_validateCTable(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue) == 0
    {
        *repeat = HUF_repeat_none;
    }
    if flags & HUF_flags_preferRepeat as c_int != 0
        && !repeat.is_null()
        && *repeat as c_uint != HUF_repeat_none as c_int as c_uint
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
        &mut (*table).wksps as *mut workspace_union as *mut c_void,
        size_of::<workspace_union>(),
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        flags,
    );
    let maxBits = HUF_buildCTable_wksp(
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.buildCTable_wksp as *mut HUF_buildCTable_wksp_tables as *mut c_void,
        size_of::<HUF_buildCTable_wksp_tables>(),
    );
    let _var_err__ = maxBits;
    if ERR_isError(_var_err__) {
        return _var_err__;
    }
    huffLog = maxBits as u32;
    let hSize = HUF_writeCTable_wksp(
        op as *mut c_void,
        dstSize,
        ((*table).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp as *mut c_void,
        size_of::<HUF_WriteCTableWksp>(),
    );
    if ERR_isError(hSize) {
        return hSize;
    }
    if !repeat.is_null() && *repeat as c_uint != HUF_repeat_none as c_int as c_uint {
        let oldSize =
            HUF_estimateCompressedSize(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue);
        let newSize = HUF_estimateCompressedSize(
            ((*table).CTable).as_mut_ptr(),
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        );
        if oldSize <= hSize + (newSize) || hSize + (12) >= srcSize {
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
    if hSize + (12) >= srcSize {
        return 0;
    }
    op = op.add(hSize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }
    if !oldHufTable.is_null() {
        libc::memcpy(
            oldHufTable as *mut c_void,
            ((*table).CTable).as_mut_ptr() as *const c_void,
            size_of::<[HUF_CElt; 257]>() as c_ulong as libc::size_t,
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
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    maxSymbolValue: c_uint,
    huffLog: c_uint,
    workSpace: *mut c_void,
    wkspSize: size_t,
    hufTable: *mut HUF_CElt,
    repeat: *mut HUF_repeat,
    flags: c_int,
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
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    maxSymbolValue: c_uint,
    huffLog: c_uint,
    workSpace: *mut c_void,
    wkspSize: size_t,
    hufTable: *mut HUF_CElt,
    repeat: *mut HUF_repeat,
    flags: c_int,
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
