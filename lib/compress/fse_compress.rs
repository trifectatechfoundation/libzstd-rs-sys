use libc::size_t;

use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::bitstream::{
    BIT_CStream_t, BIT_closeCStream, BIT_flushBits, BIT_flushBitsFast, BIT_initCStream,
    BitContainerType,
};
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{
    FSE_CState_t, FSE_CTable, FSE_encodeSymbol, FSE_flushCState, FSE_initCState2,
    FSE_symbolCompressionTransform, FSE_symbolTTIndex, FSE_writeU16Pair, FSE_DEFAULT_TABLELOG,
    FSE_MAX_TABLELOG, FSE_MIN_TABLELOG, FSE_NCOUNTBOUND,
};
use crate::lib::common::mem::MEM_write64;

/// Same as FSE_buildCTable(), but using an externally allocated scratch buffer (`workSpace`).
/// wkspSize should be sized to handle worst case situation, which is `1<<max_tableLog * sizeof(FSE_FUNCTION_TYPE)`
/// workSpace must also be properly aligned with FSE_FUNCTION_TYPE requirements
pub(crate) unsafe fn FSE_buildCTable_wksp(
    ct: *mut FSE_CTable,
    normalizedCounter: *const core::ffi::c_short,
    maxSymbolValue: u8,
    tableLog: core::ffi::c_uint,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let tableSize = (1 << tableLog) as u32;
    let tableMask = tableSize.wrapping_sub(1);
    let ptr = ct as *mut core::ffi::c_void;
    let tableU16 = (ptr as *mut u16).add(2);
    let FSCT = (ptr as *mut u32)
        .add(1) // header
        .offset((if tableLog != 0 { tableSize >> 1 } else { 1 }) as isize)
        as *mut core::ffi::c_void;
    let symbolTT = FSCT as *mut FSE_symbolCompressionTransform;
    let step = (tableSize >> 1)
        .wrapping_add(tableSize >> 3)
        .wrapping_add(3);
    let maxSV1 = u32::from(maxSymbolValue) + 1;

    let cumul = workSpace as *mut u16;
    let tableSymbol = cumul.offset(maxSV1.wrapping_add(1) as isize) as *mut u8;

    let mut highThreshold = tableSize.wrapping_sub(1);

    if (size_of::<core::ffi::c_uint>() as core::ffi::c_ulonglong).wrapping_mul(
        ((u32::from(maxSymbolValue) + 2) as core::ffi::c_ulonglong)
            .wrapping_add(1 << tableLog)
            .wrapping_div(2)
            .wrapping_add(
                (size_of::<u64>() as core::ffi::c_ulong)
                    .wrapping_div(size_of::<u32>() as core::ffi::c_ulong)
                    as core::ffi::c_ulonglong,
            ),
    ) > wkspSize as core::ffi::c_ulonglong
    {
        return Error::tableLog_tooLarge.to_error_code();
    }

    // CTable header
    *tableU16.sub(2) = tableLog as u16;
    *tableU16.sub(1) = u16::from(maxSymbolValue);

    // For explanations on how to distribute symbol values over the table :
    // https://fastcompression.blogspot.fr/2014/02/fse-distributing-symbol-values.html

    // symbol start positions
    *cumul = 0;
    for u in 1..maxSV1 + 1 {
        if *normalizedCounter.offset(u.wrapping_sub(1) as isize) as core::ffi::c_int == -1 {
            // Low proba symbol
            *cumul.offset(u as isize) =
                (*cumul.offset(u.wrapping_sub(1) as isize) as core::ffi::c_int + 1) as u16;
            *tableSymbol.offset(highThreshold as isize) = u.wrapping_sub(1) as u8;
            highThreshold = highThreshold.wrapping_sub(1);
        } else {
            *cumul.offset(u as isize) = (*cumul.offset(u.wrapping_sub(1) as isize)
                as core::ffi::c_int
                + *normalizedCounter.offset(u.wrapping_sub(1) as isize) as u16 as core::ffi::c_int)
                as u16;
        }
    }
    *cumul.offset(maxSV1 as isize) = tableSize.wrapping_add(1) as u16;

    // Spread symbols
    if highThreshold == tableSize.wrapping_sub(1) {
        // Case for no low prob count symbols. Lay down 8 bytes at a time
        // to reduce branch misses since we are operating on a small block
        let spread = tableSymbol.offset(tableSize as isize);
        let add = 0x101010101010101u64;
        let mut pos = 0usize;
        let mut sv = 0u64;
        for s in 0..maxSV1 {
            let mut i: core::ffi::c_int = 0;
            let n = *normalizedCounter.offset(s as isize) as core::ffi::c_int;
            MEM_write64(spread.add(pos) as *mut core::ffi::c_void, sv);
            i = 8;
            while i < n {
                MEM_write64(
                    spread.add(pos).offset(i as isize) as *mut core::ffi::c_void,
                    sv,
                );
                i += 8;
            }
            pos = pos.wrapping_add(n as size_t);
            sv = sv.wrapping_add(add);
        }

        // Spread symbols across the table. Lack of lowprob symbols means that
        // we don't need variable sized inner loop, so we can unroll the loop and
        // reduce branch misses.
        let mut position = 0usize;
        let mut s_0: size_t = 0;
        let unroll = 2; // Experimentally determined optimal unroll
        s_0 = 0;
        while s_0 < tableSize as size_t {
            for u_0 in 0..unroll {
                let uPosition = position.wrapping_add(u_0 * step as size_t) & tableMask as size_t;
                *tableSymbol.add(uPosition) = *spread.add(s_0.wrapping_add(u_0));
            }
            position = position.wrapping_add(unroll * step as size_t) & tableMask as size_t;
            s_0 = s_0.wrapping_add(unroll);
        }
    } else {
        let mut position_0 = 0u32;
        for symbol in 0..maxSV1 {
            let mut nbOccurrences: core::ffi::c_int = 0;
            let freq = *normalizedCounter.offset(symbol as isize) as core::ffi::c_int;
            nbOccurrences = 0;
            while nbOccurrences < freq {
                *tableSymbol.offset(position_0 as isize) = symbol as u8;
                position_0 = position_0.wrapping_add(step) & tableMask;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step) & tableMask; // Low proba area
                }
                nbOccurrences += 1;
            }
        }
    }

    // Build table
    let mut u_1: u32 = 0;
    while u_1 < tableSize {
        let s_1 = *tableSymbol.offset(u_1 as isize);
        let fresh1 = &mut (*cumul.offset(s_1 as isize));
        *tableU16.offset(*fresh1 as isize) = tableSize.wrapping_add(u_1) as u16;
        *fresh1 = (*fresh1).wrapping_add(1);
        u_1 = u_1.wrapping_add(1);
    }

    // Build Symbol Transformation Table
    let mut total = 0u32;
    let mut s_2: core::ffi::c_uint = 0;
    while s_2 <= u32::from(maxSymbolValue) {
        match *normalizedCounter.offset(s_2 as isize) as core::ffi::c_int {
            0 => {
                // filling nonetheless, for compatibility with FSE_getMaxNbBits()
                (*symbolTT.offset(s_2 as isize)).deltaNbBits = (tableLog.wrapping_add(1) << 16)
                    .wrapping_sub((1 << tableLog) as core::ffi::c_uint);
            }
            -1 | 1 => {
                (*symbolTT.offset(s_2 as isize)).deltaNbBits =
                    (tableLog << 16).wrapping_sub((1 << tableLog) as core::ffi::c_uint);
                (*symbolTT.offset(s_2 as isize)).deltaFindState =
                    total.wrapping_sub(1) as core::ffi::c_int;
                total = total.wrapping_add(1);
            }
            _ => {
                let maxBitsOut = tableLog.wrapping_sub(ZSTD_highbit32(
                    (*normalizedCounter.offset(s_2 as isize) as u32).wrapping_sub(1),
                ));
                let minStatePlus = (*normalizedCounter.offset(s_2 as isize) as u32) << maxBitsOut;
                (*symbolTT.offset(s_2 as isize)).deltaNbBits =
                    (maxBitsOut << 16).wrapping_sub(minStatePlus);
                (*symbolTT.offset(s_2 as isize)).deltaFindState = total
                    .wrapping_sub(*normalizedCounter.offset(s_2 as isize) as core::ffi::c_uint)
                    as core::ffi::c_int;
                total = total
                    .wrapping_add(*normalizedCounter.offset(s_2 as isize) as core::ffi::c_uint);
            }
        }
        s_2 = s_2.wrapping_add(1);
    }
    0
}

fn FSE_NCountWriteBound(maxSymbolValue: u8, tableLog: core::ffi::c_uint) -> size_t {
    let maxHeaderSize = (u32::from(maxSymbolValue) + 1)
        .wrapping_mul(tableLog)
        .wrapping_add(4) // bitCount initialized at 4
        .wrapping_add(2) // first two symbols may use one additional bit each
        .wrapping_div(8)
        .wrapping_add(1) // round up to whole nb bytes
        .wrapping_add(2) // additional two bytes for bitstream flush
        as size_t;
    if maxSymbolValue != 0 {
        maxHeaderSize
    } else {
        FSE_NCOUNTBOUND as size_t
    }
}

unsafe fn FSE_writeNCount_generic(
    header: *mut core::ffi::c_void,
    headerBufferSize: size_t,
    normalizedCounter: *const core::ffi::c_short,
    maxSymbolValue: u8,
    tableLog: core::ffi::c_uint,
    writeIsSafe: core::ffi::c_uint,
) -> size_t {
    let ostart = header as *mut u8;
    let mut out = ostart;
    let oend = ostart.add(headerBufferSize);
    let mut nbBits: core::ffi::c_int = 0;
    let tableSize = 1 << tableLog;
    let mut remaining: core::ffi::c_int = 0;
    let mut threshold: core::ffi::c_int = 0;
    let mut bitStream = 0;
    let mut bitCount = 0;
    let mut symbol = 0;
    let alphabetSize = u32::from(maxSymbolValue) + 1;
    let mut previousIs0 = false;

    // Table Size
    bitStream = (bitStream as core::ffi::c_uint)
        .wrapping_add(tableLog.wrapping_sub(FSE_MIN_TABLELOG as core::ffi::c_uint) << bitCount);
    bitCount += 4;

    // Init
    remaining = tableSize + 1; // +1 for extra accuracy
    threshold = tableSize;
    nbBits = tableLog as core::ffi::c_int + 1;

    // stops at 1
    while symbol < alphabetSize && remaining > 1 {
        if previousIs0 {
            let mut start = symbol;
            while symbol < alphabetSize && *normalizedCounter.offset(symbol as isize) == 0 {
                symbol = symbol.wrapping_add(1);
            }
            if symbol == alphabetSize {
                break; // incorrect distribution
            }
            while symbol >= start.wrapping_add(24) {
                start = start.wrapping_add(24);
                bitStream = (bitStream as core::ffi::c_uint)
                    .wrapping_add((0xffff as core::ffi::c_uint) << bitCount);
                if writeIsSafe == 0 && out > oend.sub(2) {
                    return Error::dstSize_tooSmall.to_error_code(); // Buffer overflow
                }
                *out = bitStream as u8;
                *out.add(1) = (bitStream >> 8) as u8;
                out = out.add(2);
                bitStream >>= 16;
            }
            while symbol >= start.wrapping_add(3) {
                start = start.wrapping_add(3);
                bitStream = (bitStream as core::ffi::c_uint).wrapping_add(3 << bitCount);
                bitCount += 2;
            }
            bitStream = (bitStream as core::ffi::c_uint)
                .wrapping_add(symbol.wrapping_sub(start) << bitCount);
            bitCount += 2;
            if bitCount > 16 {
                if writeIsSafe == 0 && out > oend.sub(2) {
                    return Error::dstSize_tooSmall.to_error_code(); // Buffer overflow
                }
                *out = bitStream as u8;
                *out.add(1) = (bitStream >> 8) as u8;
                out = out.add(2);
                bitStream >>= 16;
                bitCount -= 16;
            }
        }

        let mut count = *normalizedCounter.offset(symbol as isize) as core::ffi::c_int;
        symbol = symbol.wrapping_add(1);
        let max = 2 * threshold - 1 - remaining;
        remaining -= if count < 0 { -count } else { count };
        count += 1; // +1 for extra accuracy
        if count >= threshold {
            count += max;
        }
        bitStream = bitStream.wrapping_add((count as u32) << bitCount);
        bitCount += nbBits;
        bitCount -= (count < max) as core::ffi::c_int;
        previousIs0 = count == 1;
        if remaining < 1 {
            return Error::GENERIC.to_error_code();
        }
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1;
        }
        if bitCount > 16 {
            if writeIsSafe == 0 && out > oend.sub(2) {
                return Error::dstSize_tooSmall.to_error_code(); // Buffer overflow
            }
            *out = bitStream as u8;
            *out.add(1) = (bitStream >> 8) as u8;
            out = out.add(2);
            bitStream >>= 16;
            bitCount -= 16;
        }
    }

    if remaining != 1 {
        return Error::GENERIC.to_error_code(); // incorrect normalized distribution
    }

    // flush remaining bitStream
    if writeIsSafe == 0 && out > oend.sub(2) {
        return Error::dstSize_tooSmall.to_error_code(); // Buffer overflow
    }
    *out = bitStream as u8;
    *out.add(1) = (bitStream >> 8) as u8;
    out = out.offset(((bitCount + 7) / 8) as isize);

    out.offset_from_unsigned(ostart)
}

pub(crate) unsafe fn FSE_writeNCount(
    buffer: *mut core::ffi::c_void,
    bufferSize: size_t,
    normalizedCounter: *const core::ffi::c_short,
    maxSymbolValue: u8,
    tableLog: core::ffi::c_uint,
) -> size_t {
    if tableLog > FSE_MAX_TABLELOG as core::ffi::c_uint {
        return Error::tableLog_tooLarge.to_error_code(); // Unsupported
    }
    if tableLog < FSE_MIN_TABLELOG as core::ffi::c_uint {
        return Error::GENERIC.to_error_code(); // Unsupported
    }

    if bufferSize < FSE_NCountWriteBound(maxSymbolValue, tableLog) {
        return FSE_writeNCount_generic(
            buffer,
            bufferSize,
            normalizedCounter,
            maxSymbolValue,
            tableLog,
            0,
        );
    }

    FSE_writeNCount_generic(
        buffer,
        bufferSize,
        normalizedCounter,
        maxSymbolValue,
        tableLog,
        1, // write in buffer is safe
    )
}

/// Provides the minimum logSize to safely represent a distribution.
fn FSE_minTableLog(srcSize: size_t, maxSymbolValue: u8) -> core::ffi::c_uint {
    let minBitsSrc = (ZSTD_highbit32(srcSize as u32)).wrapping_add(1);
    let minBitsSymbols = maxSymbolValue.ilog2().wrapping_add(2);
    minBitsSrc.min(minBitsSymbols)
}

pub(crate) fn FSE_optimalTableLog_internal(
    maxTableLog: core::ffi::c_uint,
    srcSize: size_t,
    maxSymbolValue: u8,
    minus: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let maxBitsSrc = (ZSTD_highbit32(srcSize.wrapping_sub(1) as u32)).wrapping_sub(minus);
    let mut tableLog = maxTableLog;
    let minBits = FSE_minTableLog(srcSize, maxSymbolValue);

    if tableLog == 0 {
        tableLog = FSE_DEFAULT_TABLELOG as u32;
    }
    if maxBitsSrc < tableLog {
        tableLog = maxBitsSrc; // Accuracy can be reduced
    }
    if minBits > tableLog {
        tableLog = minBits; // Need a minimum to safely represent all symbol values
    }
    if tableLog < FSE_MIN_TABLELOG as u32 {
        tableLog = FSE_MIN_TABLELOG as u32;
    }
    if tableLog > FSE_MAX_TABLELOG as u32 {
        tableLog = FSE_MAX_TABLELOG as u32;
    }
    tableLog
}

pub(crate) fn FSE_optimalTableLog(
    maxTableLog: core::ffi::c_uint,
    srcSize: size_t,
    maxSymbolValue: u8,
) -> core::ffi::c_uint {
    FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue, 2)
}

/// Secondary normalization method.
/// To be used when primary method fails.
unsafe fn FSE_normalizeM2(
    norm: *mut core::ffi::c_short,
    tableLog: u32,
    count: *const core::ffi::c_uint,
    mut total: size_t,
    maxSymbolValue: u8,
    lowProbCount: core::ffi::c_short,
) -> size_t {
    let maxSV1 = u32::from(maxSymbolValue) + 1;
    let NOT_YET_ASSIGNED = -2 as core::ffi::c_short;
    let mut s: u32 = 0;
    let mut distributed = 0u32;
    let mut ToDistribute: u32 = 0;

    let lowThreshold = (total >> tableLog) as u32;
    let mut lowOne = ((total * 3) >> tableLog.wrapping_add(1)) as u32;

    for s in 0..maxSV1 {
        if *count.offset(s as isize) == 0 {
            *norm.offset(s as isize) = 0;
        } else if *count.offset(s as isize) <= lowThreshold {
            *norm.offset(s as isize) = lowProbCount;
            distributed = distributed.wrapping_add(1);
            total = total.wrapping_sub(*count.offset(s as isize) as size_t);
        } else if *count.offset(s as isize) <= lowOne {
            *norm.offset(s as isize) = 1;
            distributed = distributed.wrapping_add(1);
            total = total.wrapping_sub(*count.offset(s as isize) as size_t);
        } else {
            *norm.offset(s as isize) = NOT_YET_ASSIGNED;
        }
    }
    ToDistribute = ((1 << tableLog) as u32).wrapping_sub(distributed);

    if ToDistribute == 0 {
        return 0;
    }

    if total / ToDistribute as size_t > lowOne as size_t {
        // risk of rounding to zero
        lowOne = (total * 3 / (ToDistribute * 2) as size_t) as u32;
        for s in 0..maxSV1 {
            if *norm.offset(s as isize) as core::ffi::c_int == NOT_YET_ASSIGNED as core::ffi::c_int
                && *count.offset(s as isize) <= lowOne
            {
                *norm.offset(s as isize) = 1;
                distributed = distributed.wrapping_add(1);
                total = total.wrapping_sub(*count.offset(s as isize) as size_t);
            }
        }
        ToDistribute = ((1 << tableLog) as u32).wrapping_sub(distributed);
    }

    if distributed == maxSV1 {
        // all values are pretty poor;
        // probably incompressible data (should have already been detected);
        // find max, then give all remaining points to max
        let mut maxV = 0;
        let mut maxC = 0;
        for s in 0..maxSV1 {
            if *count.offset(s as isize) > maxC {
                maxV = s;
                maxC = *count.offset(s as isize);
            }
        }
        let fresh4 = &mut (*norm.offset(maxV as isize));
        *fresh4 = (*fresh4 as core::ffi::c_int
            + ToDistribute as core::ffi::c_short as core::ffi::c_int)
            as core::ffi::c_short;
        return 0;
    }

    if total == 0 {
        // all of the symbols were low enough for the lowOne or lowThreshold
        s = 0;
        while ToDistribute > 0 {
            if *norm.offset(s as isize) as core::ffi::c_int > 0 {
                ToDistribute = ToDistribute.wrapping_sub(1);
                *norm.offset(s as isize) += 1;
            }
            s = s.wrapping_add(1) % maxSV1;
        }
        return 0;
    }

    let vStepLog = 62u32.wrapping_sub(tableLog) as u64;
    let mid = (1u64 << vStepLog.wrapping_sub(1)).wrapping_sub(1);
    let rStep = ((1 << vStepLog) * ToDistribute as u64).wrapping_add(mid) / total as u32 as u64;
    let mut tmpTotal = mid;
    for s in 0..maxSV1 {
        if *norm.offset(s as isize) as core::ffi::c_int == NOT_YET_ASSIGNED as core::ffi::c_int {
            let end = tmpTotal.wrapping_add(*count.offset(s as isize) as u64 * rStep);
            let sStart = (tmpTotal >> vStepLog) as u32;
            let sEnd = (end >> vStepLog) as u32;
            let weight = sEnd.wrapping_sub(sStart);
            if weight < 1 {
                return Error::GENERIC.to_error_code();
            }
            *norm.offset(s as isize) = weight as core::ffi::c_short;
            tmpTotal = end;
        }
    }

    0
}

pub(crate) unsafe fn FSE_normalizeCount(
    normalizedCounter: *mut core::ffi::c_short,
    mut tableLog: core::ffi::c_uint,
    count: *const core::ffi::c_uint,
    total: size_t,
    maxSymbolValue: u8,
    useLowProbCount: bool,
) -> size_t {
    // Sanity checks
    if tableLog == 0 {
        tableLog = FSE_DEFAULT_TABLELOG as core::ffi::c_uint;
    }
    if tableLog < FSE_MIN_TABLELOG as core::ffi::c_uint {
        return Error::GENERIC.to_error_code(); // Unsupported size
    }
    if tableLog > FSE_MAX_TABLELOG as core::ffi::c_uint {
        return Error::tableLog_tooLarge.to_error_code(); // Unsupported size
    }
    if tableLog < FSE_minTableLog(total, maxSymbolValue) {
        return Error::GENERIC.to_error_code(); // Too small tableLog, compression potentially impossible
    }

    static rtbTable: [u32; 8] = [0, 473195, 504333, 520860, 550000, 700000, 750000, 830000];
    let lowProbCount = (if useLowProbCount { -1 } else { 1 }) as core::ffi::c_short;
    let scale = (62 as core::ffi::c_uint).wrapping_sub(tableLog) as u64;
    let step = (1 << 62) / total as u32 as u64;
    let vStep = 1u64.wrapping_shl(scale.wrapping_sub(20) as u32);
    let mut stillToDistribute = 1 << tableLog;
    let mut largest = 0;
    let mut largestP = 0;
    let lowThreshold = (total >> tableLog) as u32;

    for s in 0..u32::from(maxSymbolValue) + 1 {
        if *count.offset(s as isize) as size_t == total {
            return 0; // rle special case
        }
        if *count.offset(s as isize) == 0 {
            *normalizedCounter.offset(s as isize) = 0;
        } else if *count.offset(s as isize) <= lowThreshold {
            *normalizedCounter.offset(s as isize) = lowProbCount;
            stillToDistribute -= 1;
        } else {
            let mut proba =
                ((*count.offset(s as isize) as u64 * step) >> scale) as core::ffi::c_short;
            if (proba as core::ffi::c_int) < 8 {
                let restToBeat = vStep * rtbTable[proba as usize] as u64;
                proba = (proba as core::ffi::c_int
                    + ((*count.offset(s as isize) as u64 * step)
                        .wrapping_sub((proba as u64) << scale)
                        > restToBeat) as core::ffi::c_int)
                    as core::ffi::c_short;
            }
            if proba as core::ffi::c_int > largestP as core::ffi::c_int {
                largestP = proba;
                largest = s;
            }
            *normalizedCounter.offset(s as isize) = proba;
            stillToDistribute -= proba as core::ffi::c_int;
        }
    }
    if -stillToDistribute >= *normalizedCounter.offset(largest as isize) as core::ffi::c_int >> 1 {
        // corner case, need another normalization method
        let errorCode = FSE_normalizeM2(
            normalizedCounter,
            tableLog,
            count,
            total,
            maxSymbolValue,
            lowProbCount,
        );
        if ERR_isError(errorCode) {
            return errorCode;
        }
    } else {
        let fresh6 = &mut (*normalizedCounter.offset(largest as isize));
        *fresh6 = (*fresh6 as core::ffi::c_int
            + stillToDistribute as core::ffi::c_short as core::ffi::c_int)
            as core::ffi::c_short;
    }

    tableLog as size_t
}

/// Fake FSE_CTable, for rle input (always same symbol).
pub(crate) fn FSE_buildCTable_rle(ct: &mut [FSE_CTable], symbolValue: u8) -> size_t {
    // header: a tableLog of zero, and `symbolValue` as the maximum symbol value
    ct[0] = FSE_writeU16Pair(0, symbolValue as u16);

    // the (two-entry) state table, zeroed just in case
    ct[1] = FSE_writeU16Pair(0, 0);

    // Build Symbol Transformation Table
    let index = FSE_symbolTTIndex(0) + 2 * symbolValue as usize;
    ct[index] = 0; // deltaFindState
    ct[index + 1] = 0; // deltaNbBits

    0
}

unsafe fn FSE_compress_usingCTable_generic(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    mut srcSize: size_t,
    ct: &[FSE_CTable],
    fast: bool,
) -> size_t {
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let mut ip = iend;

    let mut bitC = BIT_CStream_t {
        bitContainer: 0,
        bitPos: 0,
        startPtr: core::ptr::null_mut::<core::ffi::c_char>(),
        ptr: core::ptr::null_mut::<core::ffi::c_char>(),
        endPtr: core::ptr::null_mut::<core::ffi::c_char>(),
    };
    let mut CState1 = FSE_CState_t {
        value: 0,
        stateTable: core::ptr::null::<core::ffi::c_void>(),
        symbolTT: core::ptr::null::<core::ffi::c_void>(),
        stateLog: 0,
    };
    let mut CState2 = FSE_CState_t {
        value: 0,
        stateTable: core::ptr::null::<core::ffi::c_void>(),
        symbolTT: core::ptr::null::<core::ffi::c_void>(),
        stateLog: 0,
    };

    // init
    if srcSize <= 2 {
        return 0;
    }
    let initError = BIT_initCStream(&mut bitC, dst, dstSize);
    if ERR_isError(initError) {
        return 0; // not enough space available to write a bitstream
    }

    if srcSize & 1 != 0 {
        ip = ip.sub(1);
        FSE_initCState2(&mut CState1, ct, *ip as u32);
        ip = ip.sub(1);
        FSE_initCState2(&mut CState2, ct, *ip as u32);
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as core::ffi::c_uint);
        if fast {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        }
    } else {
        ip = ip.sub(1);
        FSE_initCState2(&mut CState2, ct, *ip as u32);
        ip = ip.sub(1);
        FSE_initCState2(&mut CState1, ct, *ip as u32);
    }

    // join to mod 4
    srcSize = srcSize.wrapping_sub(2);
    if (size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
        > (FSE_MAX_TABLELOG * 4 + 7) as core::ffi::c_ulong
        && srcSize & 2 != 0
    {
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as core::ffi::c_uint);
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as core::ffi::c_uint);
        if fast {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        }
    }

    // 2 or 4 encoding per loop
    while ip > istart {
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as core::ffi::c_uint);

        if (size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
            < (FSE_MAX_TABLELOG * 2 + 7) as core::ffi::c_ulong
        {
            // this test must be static
            if fast {
                BIT_flushBitsFast(&mut bitC);
            } else {
                BIT_flushBits(&mut bitC);
            }
        }

        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as core::ffi::c_uint);

        if (size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
            > (FSE_MAX_TABLELOG * 4 + 7) as core::ffi::c_ulong
        {
            // this test must be static
            ip = ip.sub(1);
            FSE_encodeSymbol(&mut bitC, &mut CState2, *ip as core::ffi::c_uint);
            ip = ip.sub(1);
            FSE_encodeSymbol(&mut bitC, &mut CState1, *ip as core::ffi::c_uint);
        }

        if fast {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        }
    }

    FSE_flushCState(&mut bitC, &CState2);
    FSE_flushCState(&mut bitC, &CState1);
    BIT_closeCStream(&mut bitC)
}

pub(crate) unsafe fn FSE_compress_usingCTable(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    ct: &[FSE_CTable],
) -> size_t {
    let fast = dstSize
        >= srcSize
            .wrapping_add(srcSize >> 7)
            .wrapping_add(4)
            .wrapping_add(size_of::<size_t>());

    FSE_compress_usingCTable_generic(dst, dstSize, src, srcSize, ct, fast)
}
