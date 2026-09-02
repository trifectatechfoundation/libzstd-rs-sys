use core::ffi::{c_int, c_uint, c_void};
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
    HUF_repeat_valid, HUF_BLOCKSIZE_MAX, HUF_CTABLEBOUND, HUF_CTABLE_WORKSPACE_SIZE,
    HUF_SYMBOLVALUE_MAX, HUF_SYMBOLVALUE_MAX_U8, HUF_TABLELOG_ABSOLUTEMAX, HUF_TABLELOG_DEFAULT,
    HUF_TABLELOG_MAX, HUF_WORKSPACE_SIZE,
};
use crate::lib::common::mem::{MEM_32bits, MEM_writeLE16, MEM_writeLEST};
use crate::lib::compress::fse_compress::{
    FSE_buildCTable_wksp, FSE_compress_usingCTable, FSE_normalizeCount, FSE_optimalTableLog,
    FSE_optimalTableLog_internal, FSE_writeNCount,
};
use crate::lib::compress::hist::{HIST_count_simple, HIST_count_wksp_array, HIST_WKSP_SIZE_U32};
use crate::lib::compress::zstd_compress_internal::CTable;

#[cfg(doc)]
use crate::lib::common::bitstream::BIT_CStream_t;

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
    workspaceSizePtr: &mut size_t,
    align: size_t,
) -> *mut c_void {
    let mask = align - 1;
    let rem = workspace as size_t & mask;
    let add = (align - (rem)) & mask;
    let aligned = workspace.byte_add(add);

    debug_assert!((align & (align - 1)) == 0); /* pow 2 */
    debug_assert!(align <= HUF_WORKSPACE_MAX_ALIGNMENT);

    if *workspaceSizePtr >= add {
        debug_assert!(add < align);
        debug_assert!(((aligned as size_t) & mask) == 0);
        *workspaceSizePtr -= add;
        aligned
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

/// Same as [`FSE_compress`], but dedicated to huff0's weights compression.
/// The use case needs much less stack memory.
/// Note : all elements within weightTable are supposed to be <= [`HUF_TABLELOG_MAX`].
unsafe fn HUF_compressWeights(
    dst: *mut c_void,
    dstSize: size_t,
    weightTable: &[u8; HUF_SYMBOLVALUE_MAX as usize],
    wtSize: size_t,
    workspace: *mut c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let mut op = ostart;
    let oend = ostart.add(dstSize);

    let mut maxSymbolValue = HUF_TABLELOG_MAX as u8;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as u32;
    let wksp = HUF_alignUpWorkspace(workspace, &mut workspaceSize, align_of::<u32>())
        as *mut HUF_CompressWeightsWksp;

    if workspaceSize < size_of::<HUF_CompressWeightsWksp>() {
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
            weightTable.as_ptr().cast::<c_void>(),
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
        &mut (*wksp).norm,
        tableLog,
        ((*wksp).count).as_mut_ptr(),
        wtSize,
        maxSymbolValue,
        /* useLowProbCount */ false,
    );
    if ERR_isError(_var_err__) {
        return _var_err__;
    }

    /* Write table description header */
    {
        let hSize = FSE_writeNCount(
            op as *mut c_void,
            oend.offset_from_unsigned(op),
            &(*wksp).norm,
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
        &mut (*wksp).CTable,
        &(*wksp).norm,
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut c_void,
        size_of::<[u32; 41]>(),
    );

    if ERR_isError(_var_err___0) {
        return _var_err___0;
    }
    {
        let cSize = FSE_compress_usingCTable(
            op as *mut c_void,
            oend.offset_from_unsigned(op),
            weightTable.as_ptr().cast::<c_void>(),
            wtSize,
            &(*wksp).CTable,
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

fn HUF_setNbBits(elt: &mut HUF_CElt, nbBits: size_t) {
    debug_assert!(nbBits <= HUF_TABLELOG_ABSOLUTEMAX);
    *elt = nbBits;
}

fn HUF_setValue(elt: &mut HUF_CElt, value: size_t) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 {
        debug_assert!((value >> nbBits) == 0);
        *elt |= value << (HUF_CElt::BITS as usize - nbBits);
    }
}

pub(super) unsafe fn HUF_readCTableHeader(ctable: *const HUF_CElt) -> HUF_CTableHeader {
    // the header is stored in the first `HUF_CElt` slot of the table
    ctable.cast::<HUF_CTableHeader>().read()
}

unsafe fn HUF_writeCTableHeader(ctable: &mut CTable, tableLog: u32, maxSymbolValue: u8) {
    const {
        assert!(size_of::<HUF_CElt>() == size_of::<HUF_CTableHeader>());
    }
    debug_assert!(tableLog < 256);
    let header = HUF_CTableHeader {
        tableLog: tableLog as u8,
        maxSymbolValue,
        unused: [0; _],
    };
    // the header is stored in the first `HUF_CElt` slot of the table
    ctable.as_mut_ptr().cast::<HUF_CTableHeader>().write(header);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_WriteCTableWksp {
    pub wksp: HUF_CompressWeightsWksp,
    pub bitsToWeight: [u8; HUF_TABLELOG_MAX + 1], /* precomputed conversion table */
    pub huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize],
}

pub unsafe fn HUF_writeCTable_wksp(
    dst: *mut c_void,
    maxDstSize: size_t,
    CTable: &CTable,
    maxSymbolValue: u8,
    huffLog: c_uint,
    workspace: *mut c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ct = &CTable[1..];
    let op = dst as *mut u8;
    let wksp = HUF_alignUpWorkspace(workspace, &mut workspaceSize, align_of::<u32>())
        as *mut HUF_WriteCTableWksp;

    const {
        assert!(HUF_CTABLE_WORKSPACE_SIZE >= size_of::<HUF_WriteCTableWksp>());
    }

    debug_assert!(HUF_readCTableHeader(CTable.as_ptr()).maxSymbolValue == maxSymbolValue);
    debug_assert!(HUF_readCTableHeader(CTable.as_ptr()).tableLog as c_uint == huffLog);

    /* check conditions */
    if workspaceSize < size_of::<HUF_WriteCTableWksp>() {
        return Error::GENERIC.to_error_code();
    }

    let maxSymbolValue = usize::from(maxSymbolValue);

    /* convert to weight */
    (*wksp).bitsToWeight[0] = 0;
    for n in 1..huffLog + 1 {
        (*wksp).bitsToWeight[n as usize] = (huffLog + 1 - n) as u8;
    }
    let bitsToWeight = &(*wksp).bitsToWeight;
    let huffWeight = &mut (*wksp).huffWeight;
    for (weight, &elt) in huffWeight[..maxSymbolValue]
        .iter_mut()
        .zip(&ct[..maxSymbolValue])
    {
        *weight = bitsToWeight[HUF_getNbBits(elt)];
    }

    /* attempt weights compression by FSE */
    if maxDstSize < 1 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    {
        let hSize = HUF_compressWeights(
            op.add(1) as *mut c_void,
            maxDstSize - 1,
            &(*wksp).huffWeight,
            maxSymbolValue,
            &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut c_void,
            size_of::<HUF_CompressWeightsWksp>(),
        );
        if ERR_isError(hSize) {
            return hSize;
        }
        if (hSize > 1) && (hSize < maxSymbolValue / 2) {
            /* FSE compressed */
            *op = hSize as u8;
            return hSize + 1;
        }
    }

    /* write raw values as 4-bits (max : 15) */
    if maxSymbolValue > 256 - 128 {
        return Error::GENERIC.to_error_code(); /* should not happen : likely means source cannot be compressed */
    }
    if maxSymbolValue.div_ceil(2) + 1 > maxDstSize {
        return Error::dstSize_tooSmall.to_error_code(); /* not enough space within dst buffer */
    }
    // 128 is the special-case marker; `maxSymbolValue <= 128` was just checked, so this fits a byte
    *op = (128 + maxSymbolValue - 1) as u8;
    (*wksp).huffWeight[maxSymbolValue] = 0;
    let mut n = 0;
    while n < maxSymbolValue {
        *op.add((n / 2) + 1) = ((*wksp).huffWeight[n] << 4) + (*wksp).huffWeight[n + 1];
        n += 2;
    }
    maxSymbolValue.div_ceil(2) + 1
}

pub unsafe fn HUF_readCTable(
    CTable: &mut CTable,
    maxSymbolValuePtr: &mut u8,
    src: *const c_void,
    srcSize: size_t,
    hasZeroWeights: &mut c_uint,
) -> size_t {
    let src = core::slice::from_raw_parts(src.cast(), srcSize);

    let mut huffWeight: [u8; HUF_SYMBOLVALUE_MAX as usize + 1] =
        [0; HUF_SYMBOLVALUE_MAX as usize + 1];
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
    *hasZeroWeights = (rankVal[0] > 0) as c_int as c_uint;

    /* check result */
    if tableLog > HUF_TABLELOG_MAX as u32 {
        return Error::tableLog_tooLarge.to_error_code();
    }
    if nbSymbols > c_uint::from(*maxSymbolValuePtr) + 1 {
        return Error::maxSymbolValue_tooSmall.to_error_code();
    }

    // the check above bounds `nbSymbols - 1` by `*maxSymbolValuePtr`
    match u8::try_from(nbSymbols - 1) {
        Ok(v) => *maxSymbolValuePtr = v,
        Err(_) => return Error::maxSymbolValue_tooSmall.to_error_code(),
    };

    HUF_writeCTableHeader(CTable, tableLog, *maxSymbolValuePtr);

    let ct = &mut CTable[1..];

    /* Prepare base value per rank */
    {
        let mut nextRankStart = 0u32;
        for n in 1..tableLog + 1 {
            let curr = nextRankStart;
            nextRankStart += rankVal[n as usize] << (n - 1);
            rankVal[n as usize] = curr;
        }
    }

    /* fill nbBits */
    {
        for n_0 in 0..nbSymbols {
            let w = huffWeight[n_0 as usize] as u32;
            HUF_setNbBits(
                &mut ct[n_0 as usize],
                ((tableLog + 1 - w) as u8 as c_int & -((w != 0) as c_int) as c_int) as size_t,
            );
        }
    }

    /* fill val */
    {
        let mut nbPerRank: [u16; HUF_TABLELOG_MAX + 2] = [0; HUF_TABLELOG_MAX + 2]; /* support w=0=>n=tableLog+1 */
        let mut valPerRank: [u16; HUF_TABLELOG_MAX + 2] = [0; HUF_TABLELOG_MAX + 2];
        let mut n_1: u32 = 0;
        {
            n_1 = 0;
            while n_1 < nbSymbols {
                nbPerRank[HUF_getNbBits(ct[n_1 as usize])] += 1;
                n_1 += 1;
            }
        }
        /* determine stating value per rank */
        valPerRank[(tableLog + 1) as usize] = 0; /* for w==0 */

        {
            let mut min = 0;
            let mut n_2: u32 = 0;
            n_2 = tableLog;
            while n_2 > 0 {
                /* start at n=tablelog <-> w=1 */
                valPerRank[n_2 as usize] = min; /* get starting value within each rank */
                min = (min as c_int + nbPerRank[n_2 as usize] as c_int) as u16;
                min = (min as c_int >> 1) as u16;
                n_2 -= 1;
            }
        }

        /* assign value within rank, symbol order */
        {
            for n_3 in 0..nbSymbols {
                let fresh1 = &mut valPerRank[HUF_getNbBits(ct[n_3 as usize])];
                HUF_setValue(&mut ct[n_3 as usize], *fresh1 as size_t);
                *fresh1 += 1;
            }
        }
    }
    readSize
}

pub unsafe fn HUF_getNbBitsFromCTable(CTable: &CTable, symbolValue: u32) -> u32 {
    debug_assert!(symbolValue <= HUF_SYMBOLVALUE_MAX);
    if symbolValue > (HUF_readCTableHeader(CTable.as_ptr())).maxSymbolValue as u32 {
        return 0;
    }
    // the first slot holds the header, so symbol `s` lives at index `s + 1`
    HUF_getNbBits(CTable[symbolValue as usize + 1]) as u32
}

/// Try to enforce `targetNbBits` on the Huffman tree described in `huffNode`.
///
/// It attempts to convert all nodes with nbBits > `targetNbBits`
/// to employ `targetNbBits` instead. Then it adjusts the tree
/// so that it remains a valid canonical Huffman tree.
///
/// # Preconditions
///
/// The sum of the ranks of each symbol == 2^largestBits,
/// where largestBits == huffNode[lastNonNull].nbBits.
///
/// # Postconditions
///
/// The sum of the ranks of each symbol == 2^largestBits,
/// where largestBits is the return value (expected <= targetNbBits).
///
/// # Parameters
///
/// * `huffNode` - The Huffman tree modified in place to enforce targetNbBits.
///   It's presumed sorted, from most frequent to rarest symbol.
/// * `lastNonNull` - The symbol with the lowest count in the Huffman tree.
/// * `targetNbBits` - The allowed number of bits, which the Huffman tree
///   may not respect. After this function the Huffman tree will
///   respect targetNbBits.
///
/// # Returns
///
/// The maximum number of bits of the Huffman tree after adjustment.
fn HUF_setMaxHeight(huffNode: &mut [nodeElt], lastNonNull: u32, targetNbBits: u32) -> u32 {
    let largestBits = huffNode[lastNonNull as usize].nbBits as u32;
    /* early exit : no elt > targetNbBits, so the tree is already valid. */
    if largestBits <= targetNbBits {
        return largestBits;
    }

    /* there are several too large elements (at least >= 2) */
    {
        let mut totalCost: c_int = 0;
        let baseCost = (1 << (largestBits - (targetNbBits))) as u32;
        let mut n = lastNonNull as c_int;

        /* Adjust any ranks > targetNbBits to targetNbBits.
         * Compute totalCost, which is how far the sum of the ranks is
         * we are over 2^largestBits after adjust the offending ranks.
         */
        while huffNode[n as usize].nbBits as u32 > targetNbBits {
            totalCost +=
                (baseCost - (1 << (largestBits - huffNode[n as usize].nbBits as u32))) as c_int;
            huffNode[n as usize].nbBits = targetNbBits as u8;
            n -= 1;
        }
        /* n stops at huffNode[n].nbBits <= targetNbBits */
        debug_assert!(huffNode[n as usize].nbBits as u32 <= targetNbBits);
        /* n end at index of smallest symbol using < targetNbBits */
        while huffNode[n as usize].nbBits as u32 == targetNbBits {
            n -= 1;
        }

        /* renorm totalCost from 2^largestBits to 2^targetNbBits
         * note : totalCost is necessarily a multiple of baseCost */
        debug_assert!(((totalCost as u32) & (baseCost - 1)) == 0);
        totalCost >>= largestBits - (targetNbBits);
        debug_assert!(totalCost > 0);

        /* repay normalized cost */
        {
            let noSymbol = 0xf0f0f0f0 as c_uint;
            /* Get pos of last (smallest = lowest cum. count) symbol per rank */
            let mut rankLast: [u32; HUF_TABLELOG_MAX + 2] = [noSymbol; HUF_TABLELOG_MAX + 2];
            {
                let mut currentNbBits = targetNbBits;
                for pos in (0..n + 1).rev() {
                    if (huffNode[pos as usize].nbBits as u32) < currentNbBits {
                        currentNbBits = huffNode[pos as usize].nbBits as u32; /* < targetNbBits */
                        rankLast[(targetNbBits - (currentNbBits)) as usize] = pos as u32;
                    }
                }
            }

            while totalCost > 0 {
                /* Try to reduce the next power of 2 above totalCost because we
                 * gain back half the rank.
                 */
                let mut nBitsToDecrease = (ZSTD_highbit32(totalCost as u32)) + 1;
                debug_assert!(nBitsToDecrease as usize <= (HUF_TABLELOG_MAX + 1));
                while nBitsToDecrease > 1 {
                    let highPos = rankLast[nBitsToDecrease as usize];
                    let lowPos = rankLast[(nBitsToDecrease - 1) as usize];
                    /* Decrease highPos if no symbols of lowPos or if it is
                     * not cheaper to remove 2 lowPos than highPos.
                     */
                    if highPos != noSymbol {
                        if lowPos == noSymbol {
                            break;
                        }
                        let highTotal = huffNode[highPos as usize].count;
                        let lowTotal = 2 * huffNode[lowPos as usize].count;
                        if highTotal <= lowTotal {
                            break;
                        }
                    }
                    nBitsToDecrease -= 1;
                }
                /* only triggered when no more rank 1 symbol left => find closest one (note : there is necessarily at least one !) */
                debug_assert!(
                    rankLast[nBitsToDecrease as usize] != noSymbol || nBitsToDecrease == 1
                );

                /* HUF_MAX_TABLELOG test just to please gcc 5+; but it should not be necessary */
                while nBitsToDecrease <= HUF_TABLELOG_MAX as u32
                    && rankLast[nBitsToDecrease as usize] == noSymbol
                {
                    nBitsToDecrease += 1;
                }
                debug_assert!(rankLast[nBitsToDecrease as usize] != noSymbol);

                /* Increase the number of bits to gain back half the rank cost. */
                totalCost -= 1 << (nBitsToDecrease - 1);
                let fresh3 = &mut huffNode[rankLast[nBitsToDecrease as usize] as usize].nbBits;
                *fresh3 += 1;

                /* Fix up the new rank.
                 * If the new rank was empty, this symbol is now its smallest.
                 * Otherwise, this symbol will be the largest in the new rank so no adjustment.
                 */
                if rankLast[(nBitsToDecrease - 1) as usize] == noSymbol {
                    rankLast[(nBitsToDecrease - 1) as usize] = rankLast[nBitsToDecrease as usize];
                }

                /* Fix up the old rank.
                 * If the symbol was at position 0, meaning it was the highest weight symbol in the tree,
                 * it must be the only symbol in its rank, so the old rank now has no symbols.
                 * Otherwise, since the Huffman nodes are sorted by count, the previous position is now
                 * the smallest node in the rank. If the previous position belongs to a different rank,
                 * then the rank is now empty.
                 */
                if rankLast[nBitsToDecrease as usize] == 0 {
                    /* special case, reached largest symbol */
                    rankLast[nBitsToDecrease as usize] = noSymbol;
                } else {
                    rankLast[nBitsToDecrease as usize] -= 1;
                    if huffNode[rankLast[nBitsToDecrease as usize] as usize].nbBits as u32
                        != targetNbBits - (nBitsToDecrease)
                    {
                        rankLast[nBitsToDecrease as usize] = noSymbol;
                        /* this rank is now empty */
                    }
                }
            } /* while (totalCost > 0) */

            /* If we've removed too much weight, then we have to add it back.
             * To avoid overshooting again, we only adjust the smallest rank.
             * We take the largest nodes from the lowest rank 0 and move them
             * to rank 1. There's guaranteed to be enough rank 0 symbols because
             * TODO.
             */
            while totalCost < 0 {
                /* Sometimes, cost correction overshoot */
                /* special case : no rank 1 symbol (using targetNbBits-1);
                 * let's create one from largest rank 0 (using targetNbBits).
                 */
                if rankLast[1] == noSymbol {
                    while huffNode[n as usize].nbBits as u32 == targetNbBits {
                        n -= 1;
                    }
                    huffNode[(n + 1) as usize].nbBits -= 1;
                    debug_assert!(n >= 0);
                    rankLast[1] = (n + 1) as u32;
                    totalCost += 1;
                } else {
                    huffNode[(rankLast[1] + 1) as usize].nbBits -= 1;
                    rankLast[1] += 1;
                    totalCost += 1;
                }
            }
        } /* repay normalized cost */
    } /* there are several too large elements (at least >= 2) */
    targetNbBits
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rankPos {
    pub base: u16,
    pub curr: u16,
}

pub type huffNodeTable = [nodeElt; 2 * (HUF_SYMBOLVALUE_MAX as usize + 1)];

/// Number of buckets available for `HUF_sort`
pub const RANK_POSITION_TABLE_SIZE: usize = 192;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_buildCTable_wksp_tables {
    pub huffNodeTbl: huffNodeTable,
    pub rankPosition: [rankPos; RANK_POSITION_TABLE_SIZE],
}

/// [`RANK_POSITION_DISTINCT_COUNT_CUTOFF`] == Cutoff point in `HUF_sort` buckets for which we use log2 bucketing.
/// Strategy is to use as many buckets as possible for representing distinct
/// counts while using the remainder to represent all "large" counts.
///
/// To satisfy this requirement for 192 buckets, we can do the following:
/// Let buckets 0-166 represent distinct counts of [0, 166]
/// Let buckets 166 to 192 represent all remaining counts up to [`RANK_POSITION_MAX_COUNT_LOG`] using log2 bucketing.
pub const RANK_POSITION_MAX_COUNT_LOG: usize = 32;
pub const RANK_POSITION_LOG_BUCKETS_BEGIN: c_int =
    ((RANK_POSITION_TABLE_SIZE - 1) - RANK_POSITION_MAX_COUNT_LOG - 1) as c_int; /* == 158 */
pub const RANK_POSITION_DISTINCT_COUNT_CUTOFF: c_uint = (RANK_POSITION_LOG_BUCKETS_BEGIN as c_uint)
    + (ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN as u32)/* == 166 */);

/// Return the appropriate bucket index for a given count. See definition of
/// [`RANK_POSITION_DISTINCT_COUNT_CUTOFF`] for explanation of bucketing strategy.
fn HUF_getIndex(count: u32) -> u32 {
    if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        (ZSTD_highbit32(count)) + (RANK_POSITION_LOG_BUCKETS_BEGIN as c_uint)
    }
}

/// Insertion sort by descending order
#[inline(always)]
fn HUF_insertionSort(huffNode: &mut [nodeElt], low: c_int, high: c_int) {
    let size = high - low + 1;
    let huffNode = &mut huffNode[low as usize..];
    for i in 1..size {
        let key = huffNode[i as usize];
        let mut j = i - 1;
        while j >= 0 && huffNode[j as usize].count < key.count {
            huffNode[(j + 1) as usize] = huffNode[j as usize];
            j -= 1;
        }
        huffNode[(j + 1) as usize] = key;
    }
}

/// Pivot helper function for quicksort.
fn HUF_quickSortPartition(arr: &mut [nodeElt], low: c_int, high: c_int) -> c_int {
    /* Simply select rightmost element as pivot. "Better" selectors like
     * median-of-three don't experimentally appear to have any benefit.
     */
    let pivot = arr[high as usize].count;
    let mut i = low - 1;
    for j in low..high {
        if arr[j as usize].count > pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize);
    i + 1
}

/// Classic quicksort by descending with partially iterative calls
/// to reduce worst case callstack size.
fn HUF_simpleQuickSort(arr: &mut [nodeElt], mut low: c_int, mut high: c_int) {
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

/// Sorts the symbols [0, maxSymbolValue] by count[symbol] in decreasing order.
/// This is a typical bucket sorting strategy that uses either quicksort or insertion sort to sort each bucket.
///
/// # Parameters
///
/// * `huffNode` - Output: Sorted symbols by decreasing count. Only members `.count` and `.byte` are filled.
///   Must have (maxSymbolValue + 1) entries.
/// * `count` - Histogram of the symbols.
/// * `maxSymbolValue` - Maximum symbol value.
/// * `rankPosition` - This is a scratch workspace. Must have RANK_POSITION_TABLE_SIZE entries.
unsafe fn HUF_sort(
    huffNode: &mut [nodeElt],
    count: *const c_uint,
    maxSymbolValue: u8,
    rankPosition: &mut [rankPos; RANK_POSITION_TABLE_SIZE],
) {
    let maxSymbolValue1 = u32::from(maxSymbolValue) + 1;
    /* Compute base and set curr to base.
     * For symbol s let lowerRank = HUF_getIndex(count[n]) and rank = lowerRank + 1.
     * See HUF_getIndex to see bucketing strategy.
     * We attribute each symbol to lowerRank's base value, because we want to know where
     * each rank begins in the output, so for rank R we want to count ranks R+1 and above.
     */
    rankPosition.fill(rankPos { base: 0, curr: 0 });
    for n in 0..maxSymbolValue1 {
        let lowerRank = HUF_getIndex(*count.offset(n as isize));
        debug_assert!((lowerRank as usize) < (RANK_POSITION_TABLE_SIZE - 1));
        rankPosition[lowerRank as usize].base += 1;
    }
    debug_assert!(rankPosition[RANK_POSITION_TABLE_SIZE - 1].base == 0);

    /* Set up the rankPosition table */
    for n in (1..RANK_POSITION_TABLE_SIZE as u32).rev() {
        rankPosition[(n - 1) as usize].base = (rankPosition[(n - 1) as usize].base as c_int
            + rankPosition[n as usize].base as c_int)
            as u16;
        rankPosition[(n - 1) as usize].curr = rankPosition[(n - 1) as usize].base;
    }

    /* Insert each symbol into their appropriate bucket, setting up rankPosition table. */
    for n in 0..maxSymbolValue1 {
        let c = *count.offset(n as isize);
        let r = (HUF_getIndex(c)) + 1;
        let pos = rankPosition[r as usize].curr as u32;
        rankPosition[r as usize].curr += 1;
        debug_assert!(pos < maxSymbolValue1);
        huffNode[pos as usize].count = c;
        huffNode[pos as usize].byte = n as u8;
    }

    /* Sort each bucket. */
    for n in RANK_POSITION_DISTINCT_COUNT_CUTOFF..(RANK_POSITION_TABLE_SIZE - 1) as u32 {
        let bucketSize =
            rankPosition[n as usize].curr as c_int - rankPosition[n as usize].base as c_int;
        let bucketStartIdx = rankPosition[n as usize].base as u32;
        if bucketSize > 1 {
            debug_assert!(bucketStartIdx < maxSymbolValue1);
            HUF_simpleQuickSort(&mut huffNode[bucketStartIdx as usize..], 0, bucketSize - 1);
        }
    }

    debug_assert!(huffNode[..maxSymbolValue1 as usize].is_sorted_by(|a, b| a.count >= b.count));
}

pub const STARTNODE: c_int = HUF_SYMBOLVALUE_MAX as i32 + 1;

/// Takes the huffNode array sorted by HUF_sort() and builds an unlimited-depth Huffman tree.
///
/// # Parameters
///
/// * `huffNode` - The array sorted by HUF_sort(). Builds the Huffman tree in this array.
/// * `maxSymbolValue` - The maximum symbol value.
///
/// # Returns
///
/// The smallest node in the Huffman tree (by count).
unsafe fn HUF_buildTree(huffNode: *mut nodeElt, maxSymbolValue: u8) -> c_int {
    let huffNode0 = huffNode.sub(1);
    let mut nonNullRank: c_int = 0;
    let mut lowS: c_int = 0;
    let mut lowN: c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut nodeRoot: c_int = 0;

    /* init for parents */
    nonNullRank = c_int::from(maxSymbolValue);
    while (*huffNode.offset(nonNullRank as isize)).count == 0 {
        nonNullRank -= 1;
    }
    lowS = nonNullRank;
    nodeRoot = nodeNb + lowS - 1;
    lowN = nodeNb;
    (*huffNode.offset(nodeNb as isize)).count =
        ((*huffNode.offset(lowS as isize)).count) + ((*huffNode.offset((lowS - 1) as isize)).count);
    (*huffNode.offset((lowS - 1) as isize)).parent = nodeNb as u16;
    (*huffNode.offset(lowS as isize)).parent = nodeNb as u16;
    nodeNb += 1;
    lowS -= 2;
    for n in nodeNb..nodeRoot + 1 {
        (*huffNode.offset(n as isize)).count = 1 << 30;
    }
    (*huffNode0).count = 1 << 31; /* fake entry, strong barrier */

    /* create parents */
    while nodeNb <= nodeRoot {
        let n1 =
            if (*huffNode.offset(lowS as isize)).count < (*huffNode.offset(lowN as isize)).count {
                let val = lowS;
                lowS -= 1;
                val
            } else {
                let val = lowN;
                lowN += 1;
                val
            };
        let n2 =
            if (*huffNode.offset(lowS as isize)).count < (*huffNode.offset(lowN as isize)).count {
                let val = lowS;
                lowS -= 1;
                val
            } else {
                let val = lowN;
                lowN += 1;
                val
            };
        (*huffNode.offset(nodeNb as isize)).count =
            ((*huffNode.offset(n1 as isize)).count) + ((*huffNode.offset(n2 as isize)).count);
        (*huffNode.offset(n2 as isize)).parent = nodeNb as u16;
        (*huffNode.offset(n1 as isize)).parent = nodeNb as u16;
        nodeNb += 1;
    }

    /* distribute weights (unlimited tree height) */
    (*huffNode.offset(nodeRoot as isize)).nbBits = 0;
    for n in (STARTNODE..nodeRoot).rev() {
        (*huffNode.offset(n as isize)).nbBits = ((*huffNode
            .offset((*huffNode.offset(n as isize)).parent as isize))
        .nbBits as c_int
            + 1) as u8;
    }
    for n in 0..nonNullRank + 1 {
        (*huffNode.offset(n as isize)).nbBits = ((*huffNode
            .offset((*huffNode.offset(n as isize)).parent as isize))
        .nbBits as c_int
            + 1) as u8;
    }
    nonNullRank
}

/// Build the CTable given the Huffman tree in huffNode.
///
/// # Parameters
///
/// * `CTable` - Output: The output Huffman CTable.
/// * `huffNode` - The Huffman tree.
/// * `nonNullRank` - The last and smallest node in the Huffman tree.
/// * `maxSymbolValue` - The maximum symbol value.
/// * `maxNbBits` - The exact maximum number of bits used in the Huffman tree.
unsafe fn HUF_buildCTableFromTree(
    CTable: &mut CTable,
    huffNode: &[nodeElt],
    nonNullRank: c_int,
    maxSymbolValue: u8,
    maxNbBits: u32,
) {
    /* fill result into ctable (val, nbBits) */
    let ct = &mut CTable[1..];
    let mut nbPerRank: [u16; HUF_TABLELOG_MAX + 1] = [0; HUF_TABLELOG_MAX + 1];
    let mut valPerRank: [u16; HUF_TABLELOG_MAX + 1] = [0; HUF_TABLELOG_MAX + 1];
    let alphabetSize = c_int::from(maxSymbolValue) + 1;
    for n in 0..nonNullRank + 1 {
        nbPerRank[huffNode[n as usize].nbBits as usize] += 1;
    }

    /* determine starting value per rank */
    let mut min = 0;
    for n in (1..maxNbBits as c_int + 1).rev() {
        valPerRank[n as usize] = min; /* get starting value within each rank */
        min = (min as c_int + nbPerRank[n as usize] as c_int) as u16;
        min = (min as c_int >> 1) as u16;
    }

    for n in 0..alphabetSize {
        HUF_setNbBits(
            &mut ct[huffNode[n as usize].byte as usize],
            huffNode[n as usize].nbBits as size_t,
        ); /* push nbBits per symbol, symbol order */
    }
    for n in 0..alphabetSize {
        let fresh19 = &mut valPerRank[HUF_getNbBits(ct[n as usize])];
        HUF_setValue(&mut ct[n as usize], *fresh19 as size_t); /* assign value within rank, symbol order */
        *fresh19 += 1;
    }

    HUF_writeCTableHeader(CTable, maxNbBits, maxSymbolValue);
}

/// Same as `HUF_buildCTable`, but using externally allocated scratch buffer.
/// `workSpace` must be aligned on 4-bytes boundaries, and be at least as large as sizeof([`HUF_buildCTable_wksp_tables`]).
pub unsafe fn HUF_buildCTable_wksp(
    CTable: &mut CTable,
    count: *const c_uint,
    maxSymbolValue: u8,
    mut maxNbBits: u32,
    workSpace: *mut c_void,
    mut wkspSize: size_t,
) -> size_t {
    let wksp_tables = HUF_alignUpWorkspace(workSpace, &mut wkspSize, align_of::<u32>())
        as *mut HUF_buildCTable_wksp_tables;
    let huffNodeTbl = &mut (*wksp_tables).huffNodeTbl;
    let mut nonNullRank: c_int = 0;

    const {
        assert!(HUF_CTABLE_WORKSPACE_SIZE == size_of::<HUF_buildCTable_wksp_tables>());
    }

    /* safety checks */
    if wkspSize < size_of::<HUF_buildCTable_wksp_tables>() {
        return Error::workSpace_tooSmall.to_error_code();
    }
    if maxNbBits == 0 {
        maxNbBits = HUF_TABLELOG_DEFAULT;
    }
    huffNodeTbl.fill(nodeElt {
        count: 0,
        parent: 0,
        byte: 0,
        nbBits: 0,
    });
    /* sort, decreasing order */
    HUF_sort(
        &mut huffNodeTbl[1..],
        count,
        maxSymbolValue,
        &mut (*wksp_tables).rankPosition,
    );

    // HUF_buildTree reaches back to the slot before `huffNode` for its barrier entry,
    // so this pointer must keep provenance over the whole table.
    let huffNode = huffNodeTbl.as_mut_ptr().add(1);

    /* build tree */
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);

    /* determine and enforce maxTableLog */
    maxNbBits = HUF_setMaxHeight(&mut huffNodeTbl[1..], nonNullRank as u32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as u32 {
        return Error::GENERIC.to_error_code(); /* check fit into table */
    }
    HUF_buildCTableFromTree(
        CTable,
        &huffNodeTbl[1..],
        nonNullRank,
        maxSymbolValue,
        maxNbBits,
    );
    maxNbBits as size_t
}

pub unsafe fn HUF_estimateCompressedSize(
    CTable: *const HUF_CElt,
    count: *const c_uint,
    maxSymbolValue: u8,
) -> size_t {
    let ct = CTable.add(1);
    let mut nbBits = 0usize;
    for s in 0..usize::from(maxSymbolValue) + 1 {
        nbBits += HUF_getNbBits(*ct.add(s)) * *count.add(s) as size_t;
    }
    nbBits >> 3
}

pub unsafe fn HUF_validateCTable(
    CTable: *const HUF_CElt,
    count: *const c_uint,
    maxSymbolValue: u8,
) -> bool {
    let header = HUF_readCTableHeader(CTable);
    let ct = CTable.add(1);
    let mut bad = false;

    debug_assert!(header.tableLog as usize <= HUF_TABLELOG_ABSOLUTEMAX);

    if header.maxSymbolValue < maxSymbolValue {
        return false;
    }
    for s in 0..usize::from(maxSymbolValue) + 1 {
        // NOTE: use `&` rather than `&&` to keep the loop branch-free
        bad |= (*count.add(s) != 0) & (HUF_getNbBits(*ct.add(s)) == 0);
    }
    !bad
}

pub fn HUF_compressBound(size: size_t) -> size_t {
    HUF_CTABLEBOUND + (size + (size >> 8) + 8)
}

pub const HUF_BITS_IN_CONTAINER: size_t = size_t::BITS as usize;

/// Huffman uses its own `BIT_CStream_t` implementation.
/// There are three major differences from `BIT_CStream_t`:
///   1. `HUF_addBits` takes a `HUF_CElt` (size_t) which is
///      the pair (nbBits, value) in the format:
///      format:
///        - Bits [0, 4)            = nbBits
///        - Bits [4, 64 - nbBits)  = 0
///        - Bits [64 - nbBits, 64) = value
///   2. The bitContainer is built from the upper bits and
///      right shifted. E.g. to add a new value of N bits
///      you right shift the bitContainer by N, then or in
///      the new value into the N upper bits.
///   3. The bitstream has two bit containers. You can add
///      bits to the second container and merge them into
///      the first container.
#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [size_t; 2],
    pub bitPos: [size_t; 2],
    pub startPtr: *mut u8,
    pub ptr: *mut u8,
    pub endPtr: *mut u8,
}

/// Initializes the bitstream.
///
/// # Returns
///
/// 0 or an error code.
unsafe fn HUF_initCStream(
    bitC: &mut HUF_CStream_t,
    startPtr: *mut c_void,
    dstCapacity: size_t,
) -> size_t {
    ptr::write_bytes(
        ptr::from_mut(bitC).cast::<u8>(),
        0,
        size_of::<HUF_CStream_t>(),
    );
    bitC.startPtr = startPtr as *mut u8;
    bitC.ptr = bitC.startPtr;
    bitC.endPtr = (bitC.startPtr).add(dstCapacity).sub(size_of::<size_t>());
    if dstCapacity <= size_of::<size_t>() {
        return Error::dstSize_tooSmall.to_error_code();
    }
    0
}

/// Adds the symbol stored in HUF_CElt elt to the bitstream.
///
/// # Parameters
///
/// * `elt` - The element we're adding. This is a (nbBits, value) pair.
///   See the HUF_CStream_t docs for the format.
/// * `idx` - Insert into the bitstream at this idx.
/// * `kFast` - This is a template parameter. If the bitstream is guaranteed
///   to have at least 4 unused bits after this call it may be 1,
///   otherwise it must be 0. HUF_addBits() is faster when fast is set.
#[inline(always)]
fn HUF_addBits(bitC: &mut HUF_CStream_t, elt: HUF_CElt, idx: c_int, kFast: c_int) {
    debug_assert!(idx <= 1);
    debug_assert!(HUF_getNbBits(elt) <= HUF_TABLELOG_ABSOLUTEMAX);
    /* This is efficient on x86-64 with BMI2 because shrx
     * only reads the low 6 bits of the register. The compiler
     * knows this and elides the mask. When fast is set,
     * every operation can use the same value loaded from elt.
     */
    bitC.bitContainer[idx as usize] >>= HUF_getNbBits(elt);
    bitC.bitContainer[idx as usize] |= if kFast != 0 {
        HUF_getValueFast(elt)
    } else {
        HUF_getValue(elt)
    };
    /* We only read the low 8 bits of bitC->bitPos[idx] so it
     * doesn't matter that the high bits have noise from the value.
     */
    let fresh21 = &mut bitC.bitPos[idx as usize];
    *fresh21 = (*fresh21).wrapping_add(HUF_getNbBitsFast(elt));
    debug_assert!((bitC.bitPos[idx as usize] & 0xFF) <= HUF_BITS_IN_CONTAINER);
    /* The last 4-bits of elt are dirty if fast is set,
     * so we must not be overwriting bits that have already been
     * inserted into the bit container.
     */
}

#[inline(always)]
fn HUF_zeroIndex1(bitC: &mut HUF_CStream_t) {
    bitC.bitContainer[1] = 0;
    bitC.bitPos[1] = 0;
}

/// Merges the bit container @ index 1 into the bit container @ index 0
/// and zeros the bit container @ index 1.
#[inline(always)]
fn HUF_mergeIndex1(bitC: &mut HUF_CStream_t) {
    debug_assert!((bitC.bitPos[1] & 0xFF) < HUF_BITS_IN_CONTAINER);
    bitC.bitContainer[0] >>= bitC.bitPos[1] & 0xff as c_int as size_t;
    bitC.bitContainer[0] |= bitC.bitContainer[1];
    bitC.bitPos[0] += bitC.bitPos[1];
    debug_assert!((bitC.bitPos[0] & 0xFF) <= HUF_BITS_IN_CONTAINER);
}

/// Flushes the bits in the bit container @ index 0.
///
/// # Parameters
///
/// * `kFast` - If kFast is set then we must know a-priori that
///   the bit container will not overflow.
///
/// # Postconditions
///
/// bitPos will be < 8.
#[inline(always)]
unsafe fn HUF_flushBits(bitC: &mut HUF_CStream_t, kFast: c_int) {
    /* The upper bits of bitPos are noisy, so we must mask by 0xFF. */
    let nbBits = bitC.bitPos[0] & 0xff as c_int as size_t;
    let nbBytes = nbBits >> 3;
    /* The top nbBits bits of bitContainer are the ones we need. */
    let bitContainer = bitC.bitContainer[0] >> (HUF_BITS_IN_CONTAINER - (nbBits));
    /* Mask bitPos to account for the bytes we consumed. */
    bitC.bitPos[0] &= 7;
    debug_assert!(nbBits > 0);
    debug_assert!(nbBits <= size_t::BITS as usize);
    debug_assert!(bitC.ptr <= bitC.endPtr);
    MEM_writeLEST(bitC.ptr as *mut c_void, bitContainer);
    bitC.ptr = (bitC.ptr).add(nbBytes);
    debug_assert!(kFast == 0 || bitC.ptr <= bitC.endPtr);
    if kFast == 0 && bitC.ptr > bitC.endPtr {
        bitC.ptr = bitC.endPtr;
    }
    /* bitContainer doesn't need to be modified because the leftover
     * bits are already the top bitPos bits. And we don't care about
     * noise in the lower values.
     */
}

/// # Returns
///
/// The Huffman stream end mark: A 1-bit value = 1.
fn HUF_endMark() -> HUF_CElt {
    let mut endMark: HUF_CElt = 0;
    HUF_setNbBits(&mut endMark, 1);
    HUF_setValue(&mut endMark, 1);
    endMark
}

/// # Returns
///
/// Size of CStream, in bytes, or 0 if it could not fit into dstBuffer
unsafe fn HUF_closeCStream(bitC: &mut HUF_CStream_t) -> size_t {
    HUF_addBits(bitC, HUF_endMark(), 0, 0);
    HUF_flushBits(bitC, 0);
    let nbBits = bitC.bitPos[0] & 0xff as c_int as size_t;
    if bitC.ptr >= bitC.endPtr {
        return 0; /* overflow detected */
    }
    ((bitC.ptr).offset_from(bitC.startPtr) as size_t) + ((nbBits > 0) as c_int as size_t)
}

#[inline(always)]
unsafe fn HUF_encodeSymbol(
    bitCPtr: &mut HUF_CStream_t,
    symbol: u32,
    CTable: *const HUF_CElt,
    idx: c_int,
    fast: c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}

#[inline(always)]
unsafe fn HUF_compress1X_usingCTable_internal_body_loop(
    bitC: &mut HUF_CStream_t,
    ip: *const u8,
    srcSize: size_t,
    ct: *const HUF_CElt,
    kUnroll: c_int,
    kFastFlush: c_int,
    kLastFast: c_int,
) {
    /* Join to kUnroll */
    let mut n = srcSize as c_int;
    let rem = n % kUnroll;
    if rem > 0 {
        for _ in (1..rem + 1).rev() {
            n -= 1;
            HUF_encodeSymbol(bitC, *ip.offset(n as isize) as u32, ct, 0, 0);
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    debug_assert!(n % kUnroll == 0);

    /* Join to 2 * kUnroll */
    if n % (2 * kUnroll) != 0 {
        for u in 1..kUnroll {
            HUF_encodeSymbol(bitC, *ip.offset((n - u) as isize) as u32, ct, 0, 1);
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
        /* Encode kUnroll symbols into the bitstream @ index 0. */
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
        /* Encode kUnroll symbols into the bitstream @ index 1.
         * This allows us to start filling the bit container
         * without any data dependencies.
         */
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
        /* Merge bitstream @ index 1 into the bitstream @ index 0 */
        HUF_mergeIndex1(bitC);
        HUF_flushBits(bitC, kFastFlush);
        n -= 2 * kUnroll;
    }
    debug_assert!(n == 0);
}

/// Returns a tight upper bound on the output space needed by Huffman
/// with 8 bytes buffer to handle over-writes. If the output is at least
/// this large we don't need to do bounds checks during Huffman encoding.
fn HUF_tightCompressBound(srcSize: size_t, tableLog: size_t) -> size_t {
    ((srcSize * tableLog) >> 3) + 8
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

    /* init */
    if dstSize < 8 {
        return 0; /* not enough space to compress */
    }
    {
        let op = ostart;
        let initErr = HUF_initCStream(&mut bitC, op as *mut c_void, oend.offset_from_unsigned(op));
        if ERR_isError(initErr) {
            return 0;
        }
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
    CTable: &CTable,
    flags: c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as c_int != 0 {
        return HUF_compress1X_usingCTable_internal_bmi2(
            dst,
            dstSize,
            src,
            srcSize,
            CTable.as_ptr(),
        );
    }
    HUF_compress1X_usingCTable_internal_default(dst, dstSize, src, srcSize, CTable.as_ptr())
}

pub unsafe fn HUF_compress1X_usingCTable(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: &CTable,
    flags: c_int,
) -> size_t {
    HUF_compress1X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}

unsafe fn HUF_compress4X_usingCTable_internal(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: &CTable,
    flags: c_int,
) -> size_t {
    let segmentSize = srcSize.div_ceil(4); /* first 3 segments */
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

    debug_assert!(op <= oend);

    {
        assert!(op <= oend);
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
    }

    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    {
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
    }

    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    {
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
    }

    ip = ip.add(segmentSize);
    debug_assert!(op <= oend);
    debug_assert!(ip <= iend);
    {
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
    }

    op.offset_from_unsigned(ostart)
}

pub unsafe fn HUF_compress4X_usingCTable(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    CTable: &CTable,
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
    CTable: &CTable,
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
        return 0; /* uncompressible */
    }
    op = op.add(cSize);

    /* check compressibility */
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
    pub hist_wksp: [u32; HIST_WKSP_SIZE_U32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [c_uint; HUF_SYMBOLVALUE_MAX as usize + 1],
    pub CTable: CTable,
    pub wksps: workspace_union,
}

pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: usize = 4096;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: usize = 10; /* Must be >= 2 */

pub unsafe fn HUF_cardinality(count: *const c_uint, maxSymbolValue: u8) -> c_uint {
    let mut cardinality = 0 as c_uint;
    for i in 0..usize::from(maxSymbolValue) + 1 {
        if *count.add(i) != 0 {
            cardinality += 1;
        }
    }
    cardinality
}

pub fn HUF_minTableLog(symbolCardinality: c_uint) -> c_uint {
    (ZSTD_highbit32(symbolCardinality)) + 1
}

pub unsafe fn HUF_optimalTableLog(
    maxTableLog: c_uint,
    srcSize: size_t,
    maxSymbolValue: u8,
    workSpace: *mut c_void,
    wkspSize: size_t,
    table: &mut CTable,
    count: *const c_uint,
    flags: c_int,
) -> c_uint {
    debug_assert!(srcSize > 1); /* Not supported, RLE should be used instead */
    debug_assert!(wkspSize >= size_of::<HUF_buildCTable_wksp_tables>());

    if flags & HUF_flags_optimalDepth as c_int == 0 {
        /* cheap evaluation, based on FSE */
        return FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue, 1);
    }
    let dst = workSpace.byte_offset(size_of::<HUF_WriteCTableWksp>() as isize);
    let dstSize = wkspSize - size_of::<HUF_WriteCTableWksp>();
    let mut hSize: size_t = 0;
    let mut newSize: size_t = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize = (!(0) as size_t) - 1;
    let mut optLog = maxTableLog;
    let mut optLogGuess: c_uint = 0;

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
                dst,
                dstSize,
                table,
                maxSymbolValue,
                maxBits as u32,
                workSpace,
                wkspSize,
            );
            if !ERR_isError(hSize) {
                newSize =
                    (HUF_estimateCompressedSize(table.as_ptr(), count, maxSymbolValue)) + (hSize);
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

/// `workSpace_align4` must be aligned on 4-bytes boundaries,
/// and occupies the same space as a table of HUF_WORKSPACE_SIZE_U64 unsigned
unsafe fn HUF_compress_internal(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    maxSymbolValue: c_uint,
    mut huffLog: c_uint,
    nbStreams: HUF_nbStreams_e,
    workSpace: *mut c_void,
    mut wkspSize: size_t,
    oldHufTable: &mut CTable,
    repeat: *mut HUF_repeat,
    flags: c_int,
) -> size_t {
    let table = HUF_alignUpWorkspace(workSpace, &mut wkspSize, align_of::<size_t>())
        as *mut HUF_compress_tables_t;

    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut op = ostart;

    /* checks & inits */
    const {
        assert!(
            size_of::<HUF_compress_tables_t>() + HUF_WORKSPACE_MAX_ALIGNMENT <= HUF_WORKSPACE_SIZE
        );
    }

    if wkspSize < size_of::<HUF_compress_tables_t>() {
        return Error::workSpace_tooSmall.to_error_code();
    }

    // Initialize the CTable so we can take a (mutable) reference to its contents.
    core::ptr::write_bytes(&raw mut (*table).CTable, 0, 1);

    if srcSize == 0 {
        return 0; /* Uncompressed */
    }

    if dstSize == 0 {
        return 0; /* cannot fit anything within dst budget */
    }

    if srcSize > HUF_BLOCKSIZE_MAX {
        return Error::srcSize_wrong.to_error_code();
    }

    if huffLog > HUF_TABLELOG_MAX as c_uint {
        return Error::tableLog_tooLarge.to_error_code(); /* current block size limit */
    }

    // The value is restricted to the u8 range, so let's just use a u8.
    const _: () = assert!(HUF_SYMBOLVALUE_MAX == 255);

    let Ok(mut maxSymbolValue) = u8::try_from(maxSymbolValue) else {
        return Error::maxSymbolValue_tooLarge.to_error_code();
    };

    if maxSymbolValue == 0 {
        maxSymbolValue = HUF_SYMBOLVALUE_MAX_U8;
    }

    if huffLog == 0 {
        huffLog = HUF_TABLELOG_DEFAULT;
    }

    /* Heuristic : If old table is valid, use it for small inputs */
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

    /* If uncompressible data is suspected, do a smaller sampling first */
    if flags & HUF_flags_suspectUncompressible as c_int != 0
        && srcSize >= SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE * SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO
    {
        let mut largestTotal = 0usize;
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const u8 as *const c_void,
            SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE,
        ) as size_t;
        if ERR_isError(largestBegin) {
            return largestBegin;
        }
        largestTotal += largestBegin;
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            src.byte_add(srcSize)
                .byte_sub(SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE),
            SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE,
        ) as size_t;
        if ERR_isError(largestEnd) {
            return largestEnd;
        }
        largestTotal += largestEnd;
        if largestTotal <= ((2 * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE) >> 7) + 4 {
            return 0; /* heuristic : probably not compressible enough */
        }
    }

    /* Scan input and build symbol stats */
    (*table).wksps.hist_wksp.fill(0);
    let largest = HIST_count_wksp_array(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const u8 as *const c_void,
        srcSize,
        &mut (*table).wksps.hist_wksp,
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
        && *repeat as c_uint == HUF_repeat_check as c_int as c_uint
        && !HUF_validateCTable(
            oldHufTable.as_ptr(),
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        )
    {
        *repeat = HUF_repeat_none;
    }

    /* Heuristic : use existing table for small inputs */
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

    /* Build Huffman Tree */
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        &mut (*table).wksps as *mut workspace_union as *mut c_void,
        size_of::<workspace_union>(),
        &mut (*table).CTable,
        ((*table).count).as_mut_ptr(),
        flags,
    );
    let maxBits = HUF_buildCTable_wksp(
        &mut (*table).CTable,
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

    /* Write table description header */
    {
        let hSize = HUF_writeCTable_wksp(
            op as *mut c_void,
            dstSize,
            &(*table).CTable,
            maxSymbolValue,
            huffLog,
            &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp as *mut c_void,
            size_of::<HUF_WriteCTableWksp>(),
        );
        if ERR_isError(hSize) {
            return hSize;
        }

        /* Check if using previous huffman table is beneficial */
        if !repeat.is_null() && *repeat as c_uint != HUF_repeat_none as c_int as c_uint {
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

            if oldSize <= hSize + (newSize) || hSize + 12 >= srcSize {
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

        /* Use the new huffman table */
        if hSize + 12 >= srcSize {
            return 0;
        }
        op = op.add(hSize);
        if !repeat.is_null() {
            *repeat = HUF_repeat_none;
        }
        *oldHufTable = (*table).CTable;
    }
    HUF_compressCTable_internal(
        ostart,
        op,
        oend,
        src,
        srcSize,
        nbStreams,
        &(*table).CTable,
        flags,
    )
}

pub unsafe fn HUF_compress1X_repeat(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    maxSymbolValue: c_uint,
    huffLog: c_uint,
    workSpace: *mut c_void,
    wkspSize: size_t,
    hufTable: &mut CTable,
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

/// compress input using 4 streams.
/// consider skipping quickly
/// reuse an existing huffman compression table
pub unsafe fn HUF_compress4X_repeat(
    dst: *mut c_void,
    dstSize: size_t,
    src: *const c_void,
    srcSize: size_t,
    maxSymbolValue: c_uint,
    huffLog: c_uint,
    workSpace: *mut c_void,
    wkspSize: size_t,
    hufTable: &mut CTable,
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
