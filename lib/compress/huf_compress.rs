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
    HUF_repeat_valid, HUF_BLOCKSIZE_MAX, HUF_CTABLEBOUND, HUF_SYMBOLVALUE_MAX,
    HUF_TABLELOG_ABSOLUTEMAX, HUF_TABLELOG_DEFAULT, HUF_TABLELOG_MAX,
};
use crate::lib::common::mem::{MEM_32bits, MEM_writeLE16, MEM_writeLEST};
use crate::lib::compress::fse_compress::{
    FSE_buildCTable_wksp, FSE_compress_usingCTable, FSE_normalizeCount, FSE_optimalTableLog,
    FSE_optimalTableLog_internal, FSE_writeNCount,
};
use crate::lib::compress::hist::{HIST_count_simple, HIST_count_wksp};

pub type nodeElt = nodeElt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeElt_s {
    pub count: u32,
    pub parent: u16,
    pub byte: u8,
    pub nbBits: u8,
}

/* *******************************************************
*  HUF : Huffman block compression
*********************************************************/

const HUF_WORKSPACE_MAX_ALIGNMENT: usize = 8;

unsafe fn HUF_alignUpWorkspace(
    workspace: *mut core::ffi::c_void,
    workspaceSizePtr: *mut size_t,
    align: usize,
) -> *mut core::ffi::c_void {
    let mask = align - (1);
    let rem = workspace as usize & mask;
    let add = (align - rem) & mask;
    let aligned = (workspace as *mut u8).add(add);

    assert!((align & (align - 1)) == 0); /* pow 2 */
    assert!(align <= HUF_WORKSPACE_MAX_ALIGNMENT);

    if *workspaceSizePtr >= add {
        *workspaceSizePtr -= add;
        aligned as *mut core::ffi::c_void
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
    pub count: [core::ffi::c_uint; HUF_TABLELOG_MAX + 1],
    pub norm: [i16; HUF_TABLELOG_MAX + 1],
}

/// Same as [`FSE_compress`], but dedicated to huff0's weights compression.
/// The use case needs much less stack memory.
/// Note : all elements within weightTable are supposed to be <= `HUF_TABLELOG_MAX`.
///
unsafe fn HUF_compressWeights(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    weightTable: *const core::ffi::c_void,
    wtSize: size_t,
    workspace: *mut core::ffi::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(dstSize);
    let mut maxSymbolValue = HUF_TABLELOG_MAX as core::ffi::c_uint;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as u32;
    let wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<u32>(),
    ) as *mut HUF_CompressWeightsWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_CompressWeightsWksp>() {
        return Error::GENERIC.to_error_code();
    }

    /* init conditions */
    if wtSize <= 1 {
        return 0; /* Not compressible */
    }

    /* Scan input and build symbol stats */
    {
        let maxCount = HIST_count_simple(
            ((*wksp).count).as_mut_ptr(),
            &mut maxSymbolValue,
            weightTable,
            wtSize,
        ); /* never fails */
        if maxCount as size_t == wtSize {
            return 1; /* only a single symbol in src : rle */
        }
        if maxCount == 1 {
            return 0; /* each symbol present maximum once => not compressible */
        }
    }

    tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
    let _var_err__ = FSE_normalizeCount(
        ((*wksp).norm).as_mut_ptr(),
        tableLog,
        ((*wksp).count).as_mut_ptr(),
        wtSize,
        maxSymbolValue,
        /* useLowProbCount */ 0,
    );
    if ERR_isError(_var_err__) {
        return _var_err__;
    }

    /* Write table description header */
    {
        let hSize = FSE_writeNCount(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            ((*wksp).norm).as_mut_ptr(),
            maxSymbolValue,
            tableLog,
        );
        if ERR_isError(hSize) {
            return hSize;
        }
        op = op.add(hSize);
    }

    /* Compress */
    let _var_err___0 = FSE_buildCTable_wksp(
        ((*wksp).CTable).as_mut_ptr(),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[u32; 41]>(),
    );
    if ERR_isError(_var_err___0) {
        return _var_err___0;
    }
    {
        let cSize = FSE_compress_usingCTable(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            weightTable,
            wtSize,
            ((*wksp).CTable).as_mut_ptr(),
        );
        if ERR_isError(cSize) {
            return cSize;
        }
        if cSize == 0 {
            return 0; /* not enough space for compressed data */
        }
        op = op.add(cSize);
    }
    op.offset_from_unsigned(ostart)
}

fn HUF_getNbBits(elt: HUF_CElt) -> size_t {
    elt & 0xff as core::ffi::c_int as HUF_CElt
}
fn HUF_getNbBitsFast(elt: HUF_CElt) -> size_t {
    elt
}
fn HUF_getValue(elt: HUF_CElt) -> size_t {
    elt & !(0xff as core::ffi::c_int as size_t)
}
fn HUF_getValueFast(elt: HUF_CElt) -> size_t {
    elt
}

fn HUF_setNbBits(elt: &mut HUF_CElt, nbBits: size_t) {
    *elt = nbBits;
}

fn HUF_setValue(elt: &mut HUF_CElt, value: size_t) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 {
        *elt |= value << (::core::mem::size_of::<HUF_CElt>() * 8 - nbBits);
    }
}
pub(super) unsafe fn HUF_readCTableHeader(ctable: &[HUF_CElt]) -> HUF_CTableHeader {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; _],
    };

    let n = ::core::mem::size_of::<HUF_CTableHeader>();

    core::ptr::copy_nonoverlapping(
        ctable.as_ptr() as *const u8,
        (&mut header) as *mut HUF_CTableHeader as *mut u8,
        n,
    );

    header
}

unsafe fn HUF_writeCTableHeader(ctable: &mut [HUF_CElt], tableLog: u32, maxSymbolValue: u32) {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; _],
    };

    header.tableLog = tableLog as u8;
    header.maxSymbolValue = maxSymbolValue as u8;

    let n = ::core::mem::size_of::<HUF_CTableHeader>();
    core::ptr::copy_nonoverlapping(
        (&header) as *const HUF_CTableHeader as *const u8,
        ctable.as_mut_ptr() as *mut u8,
        n,
    );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_WriteCTableWksp {
    pub wksp: HUF_CompressWeightsWksp,
    pub bitsToWeight: [u8; HUF_TABLELOG_MAX + 1], /* precomputed conversion table */
    pub huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize],
}

pub unsafe fn HUF_writeCTable_wksp(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    CTable: &[HUF_CElt; 257],
    maxSymbolValue: core::ffi::c_uint,
    huffLog: core::ffi::c_uint,
    workspace: *mut core::ffi::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ct = &CTable[1..];
    let op = dst as *mut u8;
    let wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<u32>() as size_t,
    ) as *mut HUF_WriteCTableWksp;

    assert!(HUF_readCTableHeader(CTable).maxSymbolValue as core::ffi::c_uint == maxSymbolValue);
    assert!(HUF_readCTableHeader(CTable).tableLog as core::ffi::c_uint == huffLog);

    /* check conditions */
    if workspaceSize < ::core::mem::size_of::<HUF_WriteCTableWksp>() {
        return Error::GENERIC.to_error_code();
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX {
        return Error::maxSymbolValue_tooLarge.to_error_code();
    }

    /* convert to weight */
    *((*wksp).bitsToWeight).as_mut_ptr() = 0;

    for n in 1..(huffLog as usize + 1) {
        *((*wksp).bitsToWeight).as_mut_ptr().add(n) = (huffLog + 1 - (n as u32)) as u8;
    }

    for n in 0..maxSymbolValue as usize {
        ((*wksp).huffWeight)[n] = ((*wksp).bitsToWeight)[HUF_getNbBits(ct[n]) as usize];
    }

    /* attempt weights compression by FSE */
    if maxDstSize < 1 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    {
        let hSize = HUF_compressWeights(
            op.add(1) as *mut core::ffi::c_void,
            maxDstSize.wrapping_sub(1),
            ((*wksp).huffWeight).as_mut_ptr() as *const core::ffi::c_void,
            maxSymbolValue as size_t,
            &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut core::ffi::c_void,
            ::core::mem::size_of::<HUF_CompressWeightsWksp>(),
        );
        if ERR_isError(hSize) {
            return hSize;
        }
        if (hSize > 1) as core::ffi::c_int
            & (hSize < maxSymbolValue.wrapping_div(2) as size_t) as core::ffi::c_int
            != 0
        {
            /* FSE compressed */
            *op = hSize as u8;
            return hSize.wrapping_add(1);
        }
    }

    /* write raw values as 4-bits (max : 15) */
    if maxSymbolValue > (256 - 128) as core::ffi::c_uint {
        return Error::GENERIC.to_error_code(); /* should not happen : likely means source cannot be compressed */
    }
    if (maxSymbolValue.div_ceil(2) + 1) as size_t > maxDstSize {
        return Error::dstSize_tooSmall.to_error_code(); /* not enough space within dst buffer */
    }
    *op = (128 as core::ffi::c_uint/*special case*/).wrapping_add(maxSymbolValue.wrapping_sub(1))
        as u8;
    *((*wksp).huffWeight)
        .as_mut_ptr()
        .offset(maxSymbolValue as isize) = 0; /* to be sure it doesn't cause msan issue in final combination */

    for n in (0..maxSymbolValue as usize).step_by(2) {
        *op.add((n / 2) + 1) = (((*((*wksp).huffWeight).as_mut_ptr().add(n) as core::ffi::c_int)
            << 4)
            + *((*wksp).huffWeight).as_mut_ptr().add(n + (1)) as core::ffi::c_int)
            as u8;
    }
    (maxSymbolValue as usize).div_ceil(2) + 1
}

pub unsafe fn HUF_readCTable(
    CTable: &mut [HUF_CElt; 257],
    maxSymbolValuePtr: *mut core::ffi::c_uint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    hasZeroWeights: *mut core::ffi::c_uint,
) -> size_t {
    let src = core::slice::from_raw_parts(src.cast(), srcSize);

    let mut huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize + 1] =
        [0; HUF_SYMBOLVALUE_MAX as usize + 1]; /* init not required, even though some static analyzer may complain */
    let mut rankVal: [u32; HUF_TABLELOG_ABSOLUTEMAX + 1] = [0; HUF_TABLELOG_ABSOLUTEMAX + 1]; /* large enough for values from 0 to 16 */
    let mut tableLog = 0;
    let mut nbSymbols = 0;

    /* get symbol weights */
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
    *hasZeroWeights = (*rankVal.as_mut_ptr() > 0) as core::ffi::c_int as core::ffi::c_uint;

    /* check result */
    if tableLog > HUF_TABLELOG_MAX as u32 {
        return Error::tableLog_tooLarge.to_error_code();
    }
    if nbSymbols > (*maxSymbolValuePtr).wrapping_add(1) {
        return Error::maxSymbolValue_tooSmall.to_error_code();
    }

    *maxSymbolValuePtr = nbSymbols.wrapping_sub(1);

    HUF_writeCTableHeader(CTable, tableLog, *maxSymbolValuePtr);

    let ct: &mut [usize] = &mut CTable[1..];

    /* Prepare base value per rank */
    {
        let mut nextRankStart = 0u32;
        for n in 1..=tableLog as usize {
            let curr = nextRankStart;
            nextRankStart += *rankVal.as_mut_ptr().add(n) << (n - 1);
            rankVal[n] = curr;
        }
    }

    /* fill nbBits */
    {
        for (n, w) in huffWeight.iter().enumerate().take(nbSymbols as usize) {
            let w = *w as u32;
            HUF_setNbBits(
                &mut ct[n],
                ((tableLog + 1 - w) as u8 as core::ffi::c_int & -((w != 0) as core::ffi::c_int))
                    as size_t,
            );
        }
    }

    /* fill val */
    {
        let mut nbPerRank: [u16; HUF_TABLELOG_MAX + 2] = [0; HUF_TABLELOG_MAX + 2]; /* support w=0=>n=tableLog+1 */
        let mut valPerRank: [u16; HUF_TABLELOG_MAX + 2] = [0; HUF_TABLELOG_MAX + 2];

        {
            for n in 0..nbSymbols as usize {
                let v = &mut nbPerRank[HUF_getNbBits(ct[n]) as usize];
                *v += 1;
            }
        }

        /* determine stating value per rank */
        valPerRank[tableLog as usize + 1] = 0; /* for w==0 */

        {
            let mut min = 0;
            let mut n_2: u32 = 0;
            n_2 = tableLog;
            while n_2 > 0 {
                /* start at n=tablelog <-> w=1 */
                valPerRank[n_2 as usize] = min; /* get starting value within each rank */
                min += nbPerRank[n_2 as usize];
                min = (min as core::ffi::c_int >> 1) as u16;
                n_2 -= 1;
            }
        }

        /* assign value within rank, symbol order */
        {
            for n in 0..nbSymbols as usize {
                let t = &mut valPerRank[HUF_getNbBits(ct[n])];
                HUF_setValue(&mut ct[n], *t as size_t);
                *t += 1;
            }
        }
    }

    readSize
}

pub unsafe fn HUF_getNbBitsFromCTable(CTable: &[HUF_CElt; 257], symbolValue: u32) -> u32 {
    let ct = &CTable[1..];
    if symbolValue > (HUF_readCTableHeader(CTable)).maxSymbolValue as u32 {
        return 0;
    }
    HUF_getNbBits(ct[symbolValue as usize]) as u32
}

/**
 * HUF_setMaxHeight():
 * Try to enforce @targetNbBits on the Huffman tree described in @huffNode.
 *
 * It attempts to convert all nodes with nbBits > @targetNbBits
 * to employ @targetNbBits instead. Then it adjusts the tree
 * so that it remains a valid canonical Huffman tree.
 *
 * @pre               The sum of the ranks of each symbol == 2^largestBits,
 *                    where largestBits == huffNode[lastNonNull].nbBits.
 * @post              The sum of the ranks of each symbol == 2^largestBits,
 *                    where largestBits is the return value (expected <= targetNbBits).
 *
 * @param huffNode    The Huffman tree modified in place to enforce targetNbBits.
 *                    It's presumed sorted, from most frequent to rarest symbol.
 * @param lastNonNull The symbol with the lowest count in the Huffman tree.
 * @param targetNbBits  The allowed number of bits, which the Huffman tree
 *                    may not respect. After this function the Huffman tree will
 *                    respect targetNbBits.
 * @return            The maximum number of bits of the Huffman tree after adjustment.
 */
unsafe fn HUF_setMaxHeight(huffNode: *mut nodeElt, lastNonNull: u32, targetNbBits: u32) -> u32 {
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
        if *rankLast.as_mut_ptr().add(1) == noSymbol {
            while (*huffNode.offset(n as isize)).nbBits as u32 == targetNbBits {
                n -= 1;
            }
            let fresh5 = &mut (*huffNode.offset((n + 1) as isize)).nbBits;
            *fresh5 = (*fresh5).wrapping_sub(1);
            *rankLast.as_mut_ptr().add(1) = (n + 1) as u32;
            totalCost += 1;
        } else {
            let fresh6 = &mut (*huffNode
                .offset((*rankLast.as_mut_ptr().add(1)).wrapping_add(1) as isize))
            .nbBits;
            *fresh6 = (*fresh6).wrapping_sub(1);
            let fresh7 = &mut (*rankLast.as_mut_ptr().add(1));
            *fresh7 = (*fresh7).wrapping_add(1);
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

/* Number of buckets available for HUF_sort() */
pub const RANK_POSITION_TABLE_SIZE: usize = 192;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_buildCTable_wksp_tables {
    pub huffNodeTbl: huffNodeTable,
    pub rankPosition: [rankPos; RANK_POSITION_TABLE_SIZE],
}

/* RANK_POSITION_DISTINCT_COUNT_CUTOFF == Cutoff point in HUF_sort() buckets for which we use log2 bucketing.
 * Strategy is to use as many buckets as possible for representing distinct
 * counts while using the remainder to represent all "large" counts.
 *
 * To satisfy this requirement for 192 buckets, we can do the following:
 * Let buckets 0-166 represent distinct counts of [0, 166]
 * Let buckets 166 to 192 represent all remaining counts up to RANK_POSITION_MAX_COUNT_LOG using log2 bucketing.
 */
pub const RANK_POSITION_MAX_COUNT_LOG: usize = 32;
pub const RANK_POSITION_LOG_BUCKETS_BEGIN: core::ffi::c_int =
    ((RANK_POSITION_TABLE_SIZE - 1) - RANK_POSITION_MAX_COUNT_LOG - 1) as core::ffi::c_int; /* == 158 */
pub const RANK_POSITION_DISTINCT_COUNT_CUTOFF: core::ffi::c_uint = (RANK_POSITION_LOG_BUCKETS_BEGIN
    as core::ffi::c_uint)
    + (ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN as u32));

/* Return the appropriate bucket index for a given count. See definition of
 * RANK_POSITION_DISTINCT_COUNT_CUTOFF for explanation of bucketing strategy.
 */
unsafe fn HUF_getIndex(count: u32) -> u32 {
    if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        (ZSTD_highbit32(count)).wrapping_add(RANK_POSITION_LOG_BUCKETS_BEGIN as core::ffi::c_uint)
    }
}

/* Helper swap function for HUF_quickSortPartition() */
unsafe fn HUF_swapNodes(a: *mut nodeElt, b: *mut nodeElt) {
    core::ptr::swap(a, b);
}

/// Insertion sort by descending order
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

/* Pivot helper function for quicksort. */
unsafe fn HUF_quickSortPartition(
    arr: *mut nodeElt,
    low: core::ffi::c_int,
    high: core::ffi::c_int,
) -> core::ffi::c_int {
    /* Simply select rightmost element as pivot. "Better" selectors like
     * median-of-three don't experimentally appear to have any benefit.
     */
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

/* Classic quicksort by descending with partially iterative calls
 * to reduce worst case callstack size.
 */
unsafe fn HUF_simpleQuickSort(
    arr: *mut nodeElt,
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

/**
 * HUF_sort():
 * Sorts the symbols [0, maxSymbolValue] by count[symbol] in decreasing order.
 * This is a typical bucket sorting strategy that uses either quicksort or insertion sort to sort each bucket.
 *
 * @param[out] huffNode       Sorted symbols by decreasing count. Only members `.count` and `.byte` are filled.
 *                            Must have (maxSymbolValue + 1) entries.
 * @param[in]  count          Histogram of the symbols.
 * @param[in]  maxSymbolValue Maximum symbol value.
 * @param      rankPosition   This is a scratch workspace. Must have RANK_POSITION_TABLE_SIZE entries.
 */
unsafe fn HUF_sort(
    huffNode: *mut nodeElt,
    count: *const core::ffi::c_uint,
    maxSymbolValue: u32,
    rankPosition: *mut rankPos,
) {
    let mut n: u32 = 0;
    let maxSymbolValue1 = maxSymbolValue.wrapping_add(1);

    /* Compute base and set curr to base.
     * For symbol s let lowerRank = HUF_getIndex(count[n]) and rank = lowerRank + 1.
     * See HUF_getIndex to see bucketing strategy.
     * We attribute each symbol to lowerRank's base value, because we want to know where
     * each rank begins in the output, so for rank R we want to count ranks R+1 and above.
     */

    ptr::write_bytes(
        rankPosition as *mut u8,
        0,
        ::core::mem::size_of::<rankPos>() * 192,
    );
    n = 0;
    while n < maxSymbolValue1 {
        let lowerRank = HUF_getIndex(*count.offset(n as isize));
        let fresh8 = &mut (*rankPosition.offset(lowerRank as isize)).base;
        *fresh8 = (*fresh8).wrapping_add(1);
        n = n.wrapping_add(1);
    }

    // assert!(rankPosition[RANK_POSITION_TABLE_SIZE - 1].base == 0);
    /* Set up the rankPosition table */
    n = (RANK_POSITION_TABLE_SIZE - 1) as u32;
    while n > 0 {
        let fresh9 = &mut (*rankPosition.offset(n.wrapping_sub(1) as isize)).base;
        *fresh9 = (*fresh9 as core::ffi::c_int
            + (*rankPosition.offset(n as isize)).base as core::ffi::c_int) as u16;
        (*rankPosition.offset(n.wrapping_sub(1) as isize)).curr =
            (*rankPosition.offset(n.wrapping_sub(1) as isize)).base;
        n = n.wrapping_sub(1);
    }

    /* Insert each symbol into their appropriate bucket, setting up rankPosition table. */
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

    /* Sort each bucket. */
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

/** HUF_buildCTable_wksp() :
 *  Same as HUF_buildCTable(), but using externally allocated scratch buffer.
 *  `workSpace` must be aligned on 4-bytes boundaries, and be at least as large as sizeof(HUF_buildCTable_wksp_tables).
 */
pub const STARTNODE: core::ffi::c_int = HUF_SYMBOLVALUE_MAX as i32 + 1;

/* HUF_buildTree():
 * Takes the huffNode array sorted by HUF_sort() and builds an unlimited-depth Huffman tree.
 *
 * @param huffNode        The array sorted by HUF_sort(). Builds the Huffman tree in this array.
 * @param maxSymbolValue  The maximum symbol value.
 * @return                The smallest node in the Huffman tree (by count).
 */
unsafe fn HUF_buildTree(huffNode: *mut nodeElt, maxSymbolValue: u32) -> core::ffi::c_int {
    let huffNode0 = huffNode.sub(1);
    let mut nonNullRank: core::ffi::c_int = 0;
    let mut lowS: core::ffi::c_int = 0;
    let mut lowN: core::ffi::c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut n: core::ffi::c_int = 0;
    let mut nodeRoot: core::ffi::c_int = 0;

    /* init for parents */
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
    (*huffNode0).count = (1) << 31; /* fake entry, strong barrier */

    /* create parents */
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

    /* distribute weights (unlimited tree height) */
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

/**
 * HUF_buildCTableFromTree():
 * Build the CTable given the Huffman tree in huffNode.
 *
 * @param[out] CTable         The output Huffman CTable.
 * @param      huffNode       The Huffman tree.
 * @param      nonNullRank    The last and smallest node in the Huffman tree.
 * @param      maxSymbolValue The maximum symbol value.
 * @param      maxNbBits      The exact maximum number of bits used in the Huffman tree.
 */
unsafe fn HUF_buildCTableFromTree(
    CTable: &mut [HUF_CElt; 257],
    huffNode: *const nodeElt,
    nonNullRank: core::ffi::c_int,
    maxSymbolValue: u32,
    maxNbBits: u32,
) {
    /* fill result into ctable (val, nbBits) */
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

    /* determine starting value per rank */
    let mut min = 0;
    n = maxNbBits as core::ffi::c_int;
    while n > 0 {
        *valPerRank.as_mut_ptr().offset(n as isize) = min; /* get starting value within each rank */
        min = (min as core::ffi::c_int
            + *nbPerRank.as_mut_ptr().offset(n as isize) as core::ffi::c_int) as u16;
        min = (min as core::ffi::c_int >> 1) as u16;
        n -= 1;
    }

    let ct = &mut CTable[1..];
    n = 0;
    while n < alphabetSize {
        HUF_setNbBits(
            &mut ct[(*huffNode.offset(n as isize)).byte as usize],
            (*huffNode.offset(n as isize)).nbBits as size_t,
        ); /* push nbBits per symbol, symbol order */
        n += 1;
    }

    n = 0;
    while n < alphabetSize {
        let fresh19 = &mut (*valPerRank.as_mut_ptr().add(HUF_getNbBits(ct[n as usize])));
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).wrapping_add(1);
        HUF_setValue(&mut ct[n as usize], fresh20 as size_t); /* assign value within rank, symbol order */
        n += 1;
    }
    HUF_writeCTableHeader(CTable, maxNbBits, maxSymbolValue);
}

pub unsafe fn HUF_buildCTable_wksp(
    CTable: &mut [HUF_CElt; 257],
    count: *const core::ffi::c_uint,
    maxSymbolValue: u32,
    mut maxNbBits: u32,
    workSpace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let wksp_tables = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<u32>() as size_t,
    ) as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.add(1);
    let mut nonNullRank: core::ffi::c_int = 0;
    if wkspSize < ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() {
        return Error::workSpace_tooSmall.to_error_code();
    }
    if maxNbBits == 0 {
        maxNbBits = HUF_TABLELOG_DEFAULT;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX {
        return Error::maxSymbolValue_tooLarge.to_error_code();
    }
    ptr::write_bytes(
        huffNode0 as *mut u8,
        0,
        ::core::mem::size_of::<huffNodeTable>(),
    );

    /* sort, decreasing order */
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );

    /* build tree */
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);

    /* determine and enforce maxTableLog */
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as u32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as u32 {
        return Error::GENERIC.to_error_code(); /* check fit into table */
    }

    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);

    maxNbBits as size_t
}

pub unsafe fn HUF_estimateCompressedSize(
    CTable: *const HUF_CElt,
    count: *const core::ffi::c_uint,
    maxSymbolValue: core::ffi::c_uint,
) -> size_t {
    let ct = CTable.add(1);
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
    CTable: &[HUF_CElt; 257],
    count: *const core::ffi::c_uint,
    maxSymbolValue: core::ffi::c_uint,
) -> core::ffi::c_int {
    let header = HUF_readCTableHeader(CTable);
    let ct = &CTable[1..];
    let mut bad = 0;
    let mut s: core::ffi::c_int = 0;
    if (header.maxSymbolValue as core::ffi::c_uint) < maxSymbolValue {
        return 0;
    }
    s = 0;
    while s <= maxSymbolValue as core::ffi::c_int {
        bad |= (*count.offset(s as isize) != 0) as core::ffi::c_int
            & (HUF_getNbBits(ct[s as usize]) == 0) as core::ffi::c_int;
        s += 1;
    }
    (bad == 0) as core::ffi::c_int
}

pub fn HUF_compressBound(size: size_t) -> size_t {
    HUF_CTABLEBOUND.wrapping_add(size.wrapping_add(size >> 8).wrapping_add(8))
}

/** HUF_CStream_t:
 * Huffman uses its own BIT_CStream_t implementation.
 * There are three major differences from BIT_CStream_t:
 *   1. HUF_addBits() takes a HUF_CElt (size_t) which is
 *      the pair (nbBits, value) in the format:
 *      format:
 *        - Bits [0, 4)            = nbBits
 *        - Bits [4, 64 - nbBits)  = 0
 *        - Bits [64 - nbBits, 64) = value
 *   2. The bitContainer is built from the upper bits and
 *      right shifted. E.g. to add a new value of N bits
 *      you right shift the bitContainer by N, then or in
 *      the new value into the N upper bits.
 *   3. The bitstream has two bit containers. You can add
 *      bits to the second container and merge them into
 *      the first container.
 */
pub const HUF_BITS_IN_CONTAINER: size_t = (::core::mem::size_of::<size_t>()) * (8);

#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [size_t; 2],
    pub bitPos: [size_t; 2],
    pub startPtr: *mut u8,
    pub ptr: *mut u8,
    pub endPtr: *mut u8,
}

/** HUF_initCStream():
 * Initializes the bitstream.
 * @returns 0 or an error code.
 */
unsafe fn HUF_initCStream(
    bitC: *mut HUF_CStream_t,
    startPtr: *mut core::ffi::c_void,
    dstCapacity: size_t,
) -> size_t {
    ptr::write_bytes(bitC as *mut u8, 0, ::core::mem::size_of::<HUF_CStream_t>());
    (*bitC).startPtr = startPtr as *mut u8;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr)
        .add(dstCapacity)
        .offset(-(::core::mem::size_of::<size_t>() as core::ffi::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<size_t>() {
        return Error::dstSize_tooSmall.to_error_code();
    }
    0
}
#[inline(always)]
unsafe fn HUF_addBits(
    bitC: *mut HUF_CStream_t,
    elt: HUF_CElt,
    idx: core::ffi::c_int,
    kFast: core::ffi::c_int,
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
unsafe fn HUF_zeroIndex1(bitC: *mut HUF_CStream_t) {
    *((*bitC).bitContainer).as_mut_ptr().add(1) = 0;
    *((*bitC).bitPos).as_mut_ptr().add(1) = 0;
}
#[inline(always)]
unsafe fn HUF_mergeIndex1(bitC: *mut HUF_CStream_t) {
    *((*bitC).bitContainer).as_mut_ptr() >>=
        *((*bitC).bitPos).as_mut_ptr().add(1) & 0xff as core::ffi::c_int as size_t;
    *((*bitC).bitContainer).as_mut_ptr() |= *((*bitC).bitContainer).as_mut_ptr().add(1);
    (*bitC).bitPos[0] += (*bitC).bitPos[1];
}
#[inline(always)]
unsafe fn HUF_flushBits(bitC: *mut HUF_CStream_t, kFast: core::ffi::c_int) {
    let nbBits = *((*bitC).bitPos).as_mut_ptr() & 0xff as core::ffi::c_int as size_t;
    let nbBytes = nbBits >> 3;
    let bitContainer =
        *((*bitC).bitContainer).as_mut_ptr() >> HUF_BITS_IN_CONTAINER.wrapping_sub(nbBits);
    *((*bitC).bitPos).as_mut_ptr() &= 7;
    MEM_writeLEST((*bitC).ptr as *mut core::ffi::c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).add(nbBytes);
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
    let nbBits = *((*bitC).bitPos).as_mut_ptr() & 0xff as core::ffi::c_int as size_t;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0;
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as size_t)
        .wrapping_add((nbBits > 0) as core::ffi::c_int as size_t)
}
#[inline(always)]
unsafe fn HUF_encodeSymbol(
    bitCPtr: *mut HUF_CStream_t,
    symbol: u32,
    CTable: *const HUF_CElt,
    idx: core::ffi::c_int,
    fast: core::ffi::c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}
#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body_loop(
    bitC: *mut HUF_CStream_t,
    ip: *const u8,
    srcSize: size_t,
    ct: *const HUF_CElt,
    kUnroll: core::ffi::c_int,
    kFastFlush: core::ffi::c_int,
    kLastFast: core::ffi::c_int,
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

fn HUF_tightCompressBound(srcSize: size_t, tableLog: size_t) -> size_t {
    ((srcSize * tableLog) >> 3).wrapping_add(8)
}

#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    CTable: &[HUF_CElt; 257],
) -> size_t {
    let tableLog = (HUF_readCTableHeader(CTable)).tableLog as u32;
    let ct = &CTable[1..];
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
    let initErr = HUF_initCStream(
        &mut bitC,
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
    );
    if ERR_isError(initErr) {
        return 0;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as size_t) || tableLog > 11 {
        HUF_compress1X_usingCTable_internal_body_loop(
            &mut bitC,
            ip,
            srcSize,
            ct.as_ptr(),
            if MEM_32bits() { 2 } else { 4 },
            0,
            0,
        );
    } else if MEM_32bits() {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    2,
                    1,
                    0,
                );
            }
            8..=10 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    2,
                    1,
                    1,
                );
            }
            7 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    3,
                    1,
                    1,
                );
            }
        }
    } else {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    5,
                    1,
                    0,
                );
            }
            10 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    5,
                    1,
                    1,
                );
            }
            9 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    6,
                    1,
                    0,
                );
            }
            8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    7,
                    1,
                    0,
                );
            }
            7 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    8,
                    1,
                    0,
                );
            }
            6 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct.as_ptr(),
                    9,
                    1,
                    1,
                );
            }
        }
    }
    HUF_closeCStream(&mut bitC)
}

unsafe fn HUF_compress1X_usingCTable_internal_bmi2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    CTable: &[HUF_CElt; 257],
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}

unsafe fn HUF_compress1X_usingCTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    CTable: &[HUF_CElt; 257],
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}

unsafe fn HUF_compress1X_usingCTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    CTable: &[HUF_CElt; 257],
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        return HUF_compress1X_usingCTable_internal_bmi2(dst, dstSize, src, srcSize, CTable);
    }
    HUF_compress1X_usingCTable_internal_default(dst, dstSize, src, srcSize, CTable)
}

pub unsafe fn HUF_compress1X_usingCTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    CTable: &[HUF_CElt; 257],
    flags: core::ffi::c_int,
) -> size_t {
    HUF_compress1X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}

unsafe fn HUF_compress4X_usingCTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    CTable: &[HUF_CElt; 257],
    flags: core::ffi::c_int,
) -> size_t {
    let segmentSize = srcSize.wrapping_add(3) / 4; /* first 3 segments */
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut op = ostart;

    if dstSize < (6 + 1 + 1 + 1 + 8) {
        return 0; /* minimum space to compress successfully */
    }

    if srcSize < 12 {
        return 0; /* no saving possible : too small input */
    }

    op = op.add(6); /* jumpTable */

    {
        assert!(op <= oend);
        let cSize = HUF_compress1X_usingCTable_internal(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            ip as *const core::ffi::c_void,
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
        MEM_writeLE16(ostart as *mut core::ffi::c_void, cSize as u16);
        op = op.add(cSize);
    }

    ip = ip.add(segmentSize);
    assert!(op <= oend);
    {
        let cSize_0 = HUF_compress1X_usingCTable_internal(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            ip as *const core::ffi::c_void,
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
        MEM_writeLE16(ostart.add(2) as *mut core::ffi::c_void, cSize_0 as u16);
        op = op.add(cSize_0);
    }

    ip = ip.add(segmentSize);
    assert!(op <= oend);
    {
        let cSize_1 = HUF_compress1X_usingCTable_internal(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            ip as *const core::ffi::c_void,
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
        MEM_writeLE16(ostart.add(4) as *mut core::ffi::c_void, cSize_1 as u16);
        op = op.add(cSize_1);
    }

    ip = ip.add(segmentSize);
    assert!(op <= oend);
    {
        let cSize_2 = HUF_compress1X_usingCTable_internal(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            ip as *const core::ffi::c_void,
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
    }

    op.offset_from_unsigned(ostart)
}

pub(crate) unsafe fn HUF_compress4X_usingCTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    CTable: &[HUF_CElt; 257],
    flags: core::ffi::c_int,
) -> size_t {
    HUF_compress4X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}

unsafe fn HUF_compressCTable_internal(
    ostart: *mut u8,
    mut op: *mut u8,
    oend: *mut u8,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    nbStreams: HUF_nbStreams_e,
    CTable: &[HUF_CElt; 257],
    flags: core::ffi::c_int,
) -> size_t {
    let cSize = if nbStreams as core::ffi::c_uint
        == HUF_singleStream as core::ffi::c_int as core::ffi::c_uint
    {
        HUF_compress1X_usingCTable_internal(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut core::ffi::c_void,
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
        return 0; /* uncompressible */
    }
    op = op.add(cSize);

    /* check compressibility */
    assert!(op >= ostart);
    if op.offset_from_unsigned(ostart) >= srcSize.wrapping_sub(1) {
        return 0;
    }

    op.offset_from_unsigned(ostart)
}

pub type HUF_nbStreams_e = core::ffi::c_uint;
pub const HUF_fourStreams: HUF_nbStreams_e = 1;
pub const HUF_singleStream: HUF_nbStreams_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct HUF_compress_tables_t {
    pub count: [core::ffi::c_uint; 256],
    pub CTable: [HUF_CElt; 257],
    pub wksps: workspaces_union,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) union workspaces_union {
    pub buildCTable_wksp: HUF_buildCTable_wksp_tables,
    pub writeCTable_wksp: HUF_WriteCTableWksp,
    pub hist_wksp: [u32; 1024],
}

pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: core::ffi::c_int = 4096;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: core::ffi::c_int = 10; /* Must be >= 2 */

pub unsafe fn HUF_cardinality(
    count: *const core::ffi::c_uint,
    maxSymbolValue: core::ffi::c_uint,
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
pub fn HUF_minTableLog(symbolCardinality: core::ffi::c_uint) -> core::ffi::c_uint {
    (ZSTD_highbit32(symbolCardinality)).wrapping_add(1)
}

pub unsafe fn HUF_optimalTableLog(
    maxTableLog: core::ffi::c_uint,
    srcSize: size_t,
    maxSymbolValue: core::ffi::c_uint,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    table: &mut [HUF_CElt; 257],
    count: *const core::ffi::c_uint,
    flags: core::ffi::c_int,
) -> core::ffi::c_uint {
    assert!(srcSize > 1); /* Not supported, RLE should be used instead */
    assert!(wkspSize >= ::core::mem::size_of::<HUF_buildCTable_wksp_tables>());

    if flags & HUF_flags_optimalDepth as core::ffi::c_int == 0 {
        /* cheap evaluation, based on FSE */
        return FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue, 1);
    }
    let dst = (workSpace as *mut u8)
        .offset(::core::mem::size_of::<HUF_WriteCTableWksp>() as core::ffi::c_ulong as isize);
    let dstSize = wkspSize.wrapping_sub(::core::mem::size_of::<HUF_WriteCTableWksp>());
    let mut hSize: size_t = 0;
    let mut newSize: size_t = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = (!(0) as size_t).wrapping_sub(1);
    let mut optLog = maxTableLog;
    let mut optLogGuess: core::ffi::c_uint = 0;

    /* Search until size increases */
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
                dst as *mut core::ffi::c_void,
                dstSize,
                table,
                maxSymbolValue,
                maxBits as u32,
                workSpace,
                wkspSize,
            );
            if !ERR_isError(hSize) {
                newSize = (HUF_estimateCompressedSize(table.as_ptr(), count, maxSymbolValue))
                    .wrapping_add(hSize);
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

/// `workSpace_align4` must be aligned on 4-bytes boundaries,
/// and occupies the same space as a table of HUF_WORKSPACE_SIZE_U64 unsigned
unsafe fn HUF_compress_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    mut maxSymbolValue: core::ffi::c_uint,
    mut huffLog: core::ffi::c_uint,
    nbStreams: HUF_nbStreams_e,
    workSpace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
    oldHufTable: &mut [HUF_CElt; 257],
    repeat: *mut HUF_repeat,
    flags: core::ffi::c_int,
) -> size_t {
    let table = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<size_t>() as size_t,
    ) as *mut HUF_compress_tables_t;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut op = ostart;

    /* checks & inits */
    if wkspSize < ::core::mem::size_of::<HUF_compress_tables_t>() {
        return Error::workSpace_tooSmall.to_error_code();
    }

    if srcSize == 0 {
        return 0; /* Uncompressed */
    }

    if dstSize == 0 {
        return 0; /* cannot fit anything within dst budget */
    }

    if srcSize > HUF_BLOCKSIZE_MAX {
        return Error::srcSize_wrong.to_error_code();
    }

    if huffLog > HUF_TABLELOG_MAX as core::ffi::c_uint {
        return Error::tableLog_tooLarge.to_error_code(); /* current block size limit */
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

    /* Heuristic : If old table is valid, use it for small inputs */
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

    /* If uncompressible data is suspected, do a smaller sampling first */
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
        if ERR_isError(largestBegin) {
            return largestBegin;
        }
        largestTotal = largestTotal.wrapping_add(largestBegin);
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const u8).add(srcSize).sub(4096) as *const core::ffi::c_void,
            4096,
        ) as size_t;
        if ERR_isError(largestEnd) {
            return largestEnd;
        }
        largestTotal = largestTotal.wrapping_add(largestEnd);
        if largestTotal <= (((2 * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE) >> 7) + 4) as size_t {
            return 0; /* heuristic : probably not compressible enough */
        }
    }

    /* Scan input and build symbol stats */
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const u8 as *const core::ffi::c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[u32; 1024]>(),
    );
    if ERR_isError(largest) {
        return largest;
    }
    if largest == srcSize {
        *ostart = *(src as *const u8);
        return 1; /* single symbol, rle */
    }
    if largest <= (srcSize >> 7) + 4 {
        return 0; /* heuristic : probably not compressible enough */
    }

    /* Check validity of previous table */
    if !repeat.is_null()
        && *repeat as core::ffi::c_uint == HUF_repeat_check as core::ffi::c_int as core::ffi::c_uint
        && HUF_validateCTable(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue) == 0
    {
        *repeat = HUF_repeat_none;
    }

    /* Heuristic : use existing table for small inputs */
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

    /* Build Huffman Tree */
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        &mut (*table).wksps as *mut workspaces_union as *mut core::ffi::c_void,
        ::core::mem::size_of::<workspaces_union>(),
        &mut ((*table).CTable),
        ((*table).count).as_mut_ptr(),
        flags,
    );
    let maxBits = HUF_buildCTable_wksp(
        &mut ((*table).CTable),
        ((*table).count).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.buildCTable_wksp as *mut HUF_buildCTable_wksp_tables
            as *mut core::ffi::c_void,
        ::core::mem::size_of::<HUF_buildCTable_wksp_tables>(),
    );
    let _var_err__ = maxBits;
    if ERR_isError(_var_err__) {
        return _var_err__;
    }
    huffLog = maxBits as u32;

    /* Write table description header */
    let hSize = HUF_writeCTable_wksp(
        op as *mut core::ffi::c_void,
        dstSize,
        &((*table).CTable),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp as *mut core::ffi::c_void,
        ::core::mem::size_of::<HUF_WriteCTableWksp>(),
    );
    if ERR_isError(hSize) {
        return hSize;
    }

    /* Check if using previous huffman table is beneficial */
    if !repeat.is_null()
        && *repeat as core::ffi::c_uint != HUF_repeat_none as core::ffi::c_int as core::ffi::c_uint
    {
        let oldSize = HUF_estimateCompressedSize(
            oldHufTable.as_ptr(),
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        );
        let newSize = HUF_estimateCompressedSize(
            ((*table).CTable).as_mut_ptr(),
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        );

        /* Use the new huffman table */
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
    op = op.add(hSize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }

    // TODO: The C implementation checks for null here, but the above body of code implies its not possible / expected. Requires verification.
    // if !oldHufTable.is_null() {
    ::core::ptr::copy_nonoverlapping(
        ((*table).CTable).as_ptr() as *const u8,
        oldHufTable.as_mut_ptr() as *mut u8,
        ::core::mem::size_of::<[HUF_CElt; 257]>(),
    );
    // }

    HUF_compressCTable_internal(
        ostart,
        op,
        oend,
        src,
        srcSize,
        nbStreams,
        &((*table).CTable),
        flags,
    )
}

pub unsafe extern "C" fn HUF_compress1X_repeat(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    maxSymbolValue: core::ffi::c_uint,
    huffLog: core::ffi::c_uint,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    hufTable: *mut [HUF_CElt; 257],
    repeat: *mut HUF_repeat,
    flags: core::ffi::c_int,
) -> size_t {
    let hufTable = hufTable.as_mut().unwrap();

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

/// compress input using 4 streams.
/// consider skipping quickly
/// reuse an existing huffman compression table
pub unsafe extern "C" fn HUF_compress4X_repeat(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    maxSymbolValue: core::ffi::c_uint,
    huffLog: core::ffi::c_uint,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    hufTable: *mut [HUF_CElt; 257],
    repeat: *mut HUF_repeat,
    flags: core::ffi::c_int,
) -> size_t {
    let hufTable = hufTable.as_mut().unwrap();

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
