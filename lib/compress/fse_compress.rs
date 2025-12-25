use libc::{ptrdiff_t, size_t};

use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::bitstream::{
    BIT_CStream_t, BIT_addBits, BIT_closeCStream, BIT_flushBits, BIT_flushBitsFast,
    BIT_initCStream, BitContainerType,
};
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{
    FSE_CState_t, FSE_CTable, FSE_encodeSymbol, FSE_symbolCompressionTransform,
    FSE_DEFAULT_TABLELOG, FSE_MAX_TABLELOG, FSE_MIN_TABLELOG, FSE_NCOUNTBOUND,
};
use crate::lib::common::mem::{MEM_read16, MEM_write64};

#[inline]
unsafe fn FSE_initCState(statePtr: *mut FSE_CState_t, ct: *const FSE_CTable) {
    let ptr = ct as *const core::ffi::c_void;
    let u16ptr = ptr as *const u16;
    let tableLog = u32::from(MEM_read16(ptr));
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
unsafe fn FSE_initCState2(statePtr: *mut FSE_CState_t, ct: *const FSE_CTable, symbol: u32) {
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
unsafe fn FSE_flushCState(bitC: *mut BIT_CStream_t, statePtr: *const FSE_CState_t) {
    BIT_addBits(
        bitC,
        (*statePtr).value as BitContainerType,
        (*statePtr).stateLog,
    );
    BIT_flushBits(bitC);
}
pub(crate) unsafe fn FSE_buildCTable_wksp(
    ct: *mut FSE_CTable,
    normalizedCounter: *const core::ffi::c_short,
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let tableSize = ((1) << tableLog) as u32;
    let tableMask = tableSize.wrapping_sub(1);
    let ptr = ct as *mut core::ffi::c_void;
    let tableU16 = (ptr as *mut u16).add(2);
    let FSCT = (ptr as *mut u32)
        .add(1)
        .offset((if tableLog != 0 { tableSize >> 1 } else { 1 }) as isize)
        as *mut core::ffi::c_void;
    let symbolTT = FSCT as *mut FSE_symbolCompressionTransform;
    let step = (tableSize >> 1)
        .wrapping_add(tableSize >> 3)
        .wrapping_add(3);
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let cumul = workSpace as *mut u16;
    let tableSymbol = cumul.offset(maxSV1.wrapping_add(1) as isize) as *mut u8;
    let mut highThreshold = tableSize.wrapping_sub(1);
    if (::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong as core::ffi::c_ulonglong)
        .wrapping_mul(
            core::ffi::c_ulonglong::from(maxSymbolValue.wrapping_add(2))
                .wrapping_add((1) << tableLog)
                .wrapping_div(2)
                .wrapping_add(
                    (::core::mem::size_of::<u64>() as core::ffi::c_ulong)
                        .wrapping_div(::core::mem::size_of::<u32>() as core::ffi::c_ulong)
                        as core::ffi::c_ulonglong,
                ),
        )
        > wkspSize as core::ffi::c_ulonglong
    {
        return Error::tableLog_tooLarge.to_error_code();
    }
    *tableU16.sub(2) = tableLog as u16;
    *tableU16.sub(1) = maxSymbolValue as u16;
    let mut u: u32 = 0;
    *cumul = 0;
    u = 1;
    while u <= maxSV1 {
        if core::ffi::c_int::from(*normalizedCounter.offset(u.wrapping_sub(1) as isize)) == -(1) {
            *cumul.offset(u as isize) =
                (core::ffi::c_int::from(*cumul.offset(u.wrapping_sub(1) as isize)) + 1) as u16;
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            *tableSymbol.offset(fresh0 as isize) = u.wrapping_sub(1) as u8;
        } else {
            *cumul.offset(u as isize) =
                (core::ffi::c_int::from(*cumul.offset(u.wrapping_sub(1) as isize))
                    + core::ffi::c_int::from(
                        *normalizedCounter.offset(u.wrapping_sub(1) as isize) as u16
                    )) as u16;
        }
        u = u.wrapping_add(1);
    }
    *cumul.offset(maxSV1 as isize) = tableSize.wrapping_add(1) as u16;
    if highThreshold == tableSize.wrapping_sub(1) {
        let spread = tableSymbol.offset(tableSize as isize);
        let add = 0x101010101010101u64;
        let mut pos = 0 as size_t;
        let mut sv = 0u64;
        let mut s: u32 = 0;
        s = 0;
        while s < maxSV1 {
            let mut i: core::ffi::c_int = 0;
            let n = core::ffi::c_int::from(*normalizedCounter.offset(s as isize));
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
            s = s.wrapping_add(1);
            sv = sv.wrapping_add(add);
        }
        let mut position = 0 as size_t;
        let mut s_0: size_t = 0;
        let unroll = 2;
        s_0 = 0;
        while s_0 < tableSize as size_t {
            let mut u_0: size_t = 0;
            u_0 = 0;
            while u_0 < unroll {
                let uPosition = position.wrapping_add(u_0 * step as size_t) & tableMask as size_t;
                *tableSymbol.add(uPosition) = *spread.add(s_0.wrapping_add(u_0));
                u_0 = u_0.wrapping_add(1);
            }
            position = position.wrapping_add(unroll * step as size_t) & tableMask as size_t;
            s_0 = s_0.wrapping_add(unroll);
        }
    } else {
        let mut position_0 = 0u32;
        let mut symbol: u32 = 0;
        symbol = 0;
        while symbol < maxSV1 {
            let mut nbOccurrences: core::ffi::c_int = 0;
            let freq = core::ffi::c_int::from(*normalizedCounter.offset(symbol as isize));
            nbOccurrences = 0;
            while nbOccurrences < freq {
                *tableSymbol.offset(position_0 as isize) = symbol as u8;
                position_0 = position_0.wrapping_add(step) & tableMask;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step) & tableMask;
                }
                nbOccurrences += 1;
            }
            symbol = symbol.wrapping_add(1);
        }
    }
    let mut u_1: u32 = 0;
    u_1 = 0;
    while u_1 < tableSize {
        let s_1 = *tableSymbol.offset(u_1 as isize);
        let fresh1 = &mut (*cumul.offset(s_1 as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        *tableU16.offset(fresh2 as isize) = tableSize.wrapping_add(u_1) as u16;
        u_1 = u_1.wrapping_add(1);
    }
    let mut total = 0 as core::ffi::c_uint;
    let mut s_2: core::ffi::c_uint = 0;
    s_2 = 0;
    while s_2 <= maxSymbolValue {
        match core::ffi::c_int::from(*normalizedCounter.offset(s_2 as isize)) {
            0 => {
                (*symbolTT.offset(s_2 as isize)).deltaNbBits = (tableLog.wrapping_add(1) << 16)
                    .wrapping_sub(((1) << tableLog) as core::ffi::c_uint);
            }
            -1 | 1 => {
                (*symbolTT.offset(s_2 as isize)).deltaNbBits =
                    (tableLog << 16).wrapping_sub(((1) << tableLog) as core::ffi::c_uint);
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
unsafe fn FSE_NCountWriteBound(
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
) -> size_t {
    let maxHeaderSize = maxSymbolValue
        .wrapping_add(1)
        .wrapping_mul(tableLog)
        .wrapping_add(4)
        .wrapping_add(2)
        .wrapping_div(8)
        .wrapping_add(1)
        .wrapping_add(2) as size_t;
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
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
    writeIsSafe: core::ffi::c_uint,
) -> size_t {
    let ostart = header as *mut u8;
    let mut out = ostart;
    let oend = ostart.add(headerBufferSize);
    let mut nbBits: core::ffi::c_int = 0;
    let tableSize = (1) << tableLog;
    let mut remaining: core::ffi::c_int = 0;
    let mut threshold: core::ffi::c_int = 0;
    let mut bitStream = 0;
    let mut bitCount = 0;
    let mut symbol = 0;
    let alphabetSize = maxSymbolValue.wrapping_add(1);
    let mut previousIs0 = 0;
    bitStream = (bitStream as core::ffi::c_uint)
        .wrapping_add(tableLog.wrapping_sub(FSE_MIN_TABLELOG as core::ffi::c_uint) << bitCount);
    bitCount += 4;
    remaining = tableSize + 1;
    threshold = tableSize;
    nbBits = tableLog as core::ffi::c_int + 1;
    while symbol < alphabetSize && remaining > 1 {
        if previousIs0 != 0 {
            let mut start = symbol;
            while symbol < alphabetSize && *normalizedCounter.offset(symbol as isize) == 0 {
                symbol = symbol.wrapping_add(1);
            }
            if symbol == alphabetSize {
                break;
            }
            while symbol >= start.wrapping_add(24) {
                start = start.wrapping_add(24);
                bitStream = (bitStream as core::ffi::c_uint)
                    .wrapping_add((0xffff as core::ffi::c_uint) << bitCount);
                if writeIsSafe == 0 && out > oend.sub(2) {
                    return Error::dstSize_tooSmall.to_error_code();
                }
                *out = bitStream as u8;
                *out.add(1) = (bitStream >> 8) as u8;
                out = out.add(2);
                bitStream >>= 16;
            }
            while symbol >= start.wrapping_add(3) {
                start = start.wrapping_add(3);
                bitStream = (bitStream as core::ffi::c_uint).wrapping_add((3) << bitCount);
                bitCount += 2;
            }
            bitStream = (bitStream as core::ffi::c_uint)
                .wrapping_add(symbol.wrapping_sub(start) << bitCount);
            bitCount += 2;
            if bitCount > 16 {
                if writeIsSafe == 0 && out > oend.sub(2) {
                    return Error::dstSize_tooSmall.to_error_code();
                }
                *out = bitStream as u8;
                *out.add(1) = (bitStream >> 8) as u8;
                out = out.add(2);
                bitStream >>= 16;
                bitCount -= 16;
            }
        }
        let fresh3 = symbol;
        symbol = symbol.wrapping_add(1);
        let mut count = core::ffi::c_int::from(*normalizedCounter.offset(fresh3 as isize));
        let max = 2 * threshold - 1 - remaining;
        remaining -= if count < 0 { -count } else { count };
        count += 1;
        if count >= threshold {
            count += max;
        }
        bitStream = bitStream.wrapping_add((count as u32) << bitCount);
        bitCount += nbBits;
        bitCount -= core::ffi::c_int::from(count < max);
        previousIs0 = core::ffi::c_int::from(count == 1);
        if remaining < 1 {
            return Error::GENERIC.to_error_code();
        }
        while remaining < threshold {
            nbBits -= 1;
            threshold >>= 1;
        }
        if bitCount > 16 {
            if writeIsSafe == 0 && out > oend.sub(2) {
                return Error::dstSize_tooSmall.to_error_code();
            }
            *out = bitStream as u8;
            *out.add(1) = (bitStream >> 8) as u8;
            out = out.add(2);
            bitStream >>= 16;
            bitCount -= 16;
        }
    }
    if remaining != 1 {
        return Error::GENERIC.to_error_code();
    }
    if writeIsSafe == 0 && out > oend.sub(2) {
        return Error::dstSize_tooSmall.to_error_code();
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
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
) -> size_t {
    if tableLog > FSE_MAX_TABLELOG as core::ffi::c_uint {
        return Error::tableLog_tooLarge.to_error_code();
    }
    if tableLog < FSE_MIN_TABLELOG as core::ffi::c_uint {
        return Error::GENERIC.to_error_code();
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
        1,
    )
}
unsafe fn FSE_minTableLog(srcSize: size_t, maxSymbolValue: core::ffi::c_uint) -> core::ffi::c_uint {
    let minBitsSrc = (ZSTD_highbit32(srcSize as u32)).wrapping_add(1);
    let minBitsSymbols = (ZSTD_highbit32(maxSymbolValue)).wrapping_add(2);
    if minBitsSrc < minBitsSymbols {
        minBitsSrc
    } else {
        minBitsSymbols
    }
}
pub(crate) unsafe fn FSE_optimalTableLog_internal(
    maxTableLog: core::ffi::c_uint,
    srcSize: size_t,
    maxSymbolValue: core::ffi::c_uint,
    minus: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let maxBitsSrc = (ZSTD_highbit32(srcSize.wrapping_sub(1) as u32)).wrapping_sub(minus);
    let mut tableLog = maxTableLog;
    let minBits = FSE_minTableLog(srcSize, maxSymbolValue);
    if tableLog == 0 {
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
pub(crate) unsafe fn FSE_optimalTableLog(
    maxTableLog: core::ffi::c_uint,
    srcSize: size_t,
    maxSymbolValue: core::ffi::c_uint,
) -> core::ffi::c_uint {
    FSE_optimalTableLog_internal(maxTableLog, srcSize, maxSymbolValue, 2)
}
unsafe fn FSE_normalizeM2(
    norm: *mut core::ffi::c_short,
    tableLog: u32,
    count: *const core::ffi::c_uint,
    mut total: size_t,
    maxSymbolValue: u32,
    lowProbCount: core::ffi::c_short,
) -> size_t {
    let NOT_YET_ASSIGNED = -(2) as core::ffi::c_short;
    let mut s: u32 = 0;
    let mut distributed = 0u32;
    let mut ToDistribute: u32 = 0;
    let lowThreshold = (total >> tableLog) as u32;
    let mut lowOne = ((total * 3) >> tableLog.wrapping_add(1)) as u32;
    s = 0;
    while s <= maxSymbolValue {
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
        s = s.wrapping_add(1);
    }
    ToDistribute = ((1 << tableLog) as u32).wrapping_sub(distributed);
    if ToDistribute == 0 {
        return 0;
    }
    if total / ToDistribute as size_t > lowOne as size_t {
        lowOne = (total * 3 / (ToDistribute * 2) as size_t) as u32;
        s = 0;
        while s <= maxSymbolValue {
            if core::ffi::c_int::from(*norm.offset(s as isize))
                == core::ffi::c_int::from(NOT_YET_ASSIGNED)
                && *count.offset(s as isize) <= lowOne
            {
                *norm.offset(s as isize) = 1;
                distributed = distributed.wrapping_add(1);
                total = total.wrapping_sub(*count.offset(s as isize) as size_t);
            }
            s = s.wrapping_add(1);
        }
        ToDistribute = ((1 << tableLog) as u32).wrapping_sub(distributed);
    }
    if distributed == maxSymbolValue.wrapping_add(1) {
        let mut maxV = 0;
        let mut maxC = 0;
        s = 0;
        while s <= maxSymbolValue {
            if *count.offset(s as isize) > maxC {
                maxV = s;
                maxC = *count.offset(s as isize);
            }
            s = s.wrapping_add(1);
        }
        let fresh4 = &mut (*norm.offset(maxV as isize));
        *fresh4 = (core::ffi::c_int::from(*fresh4)
            + core::ffi::c_int::from(ToDistribute as core::ffi::c_short))
            as core::ffi::c_short;
        return 0;
    }
    if total == 0 {
        s = 0;
        while ToDistribute > 0 {
            if core::ffi::c_int::from(*norm.offset(s as isize)) > 0 {
                ToDistribute = ToDistribute.wrapping_sub(1);
                let fresh5 = &mut (*norm.offset(s as isize));
                *fresh5 += 1;
            }
            s = s.wrapping_add(1) % maxSymbolValue.wrapping_add(1);
        }
        return 0;
    }
    let vStepLog = u64::from(62u32.wrapping_sub(tableLog));
    let mid = (1u64 << vStepLog.wrapping_sub(1)).wrapping_sub(1);
    let rStep =
        ((1 << vStepLog) * u64::from(ToDistribute)).wrapping_add(mid) / u64::from(total as u32);
    let mut tmpTotal = mid;
    s = 0;
    while s <= maxSymbolValue {
        if core::ffi::c_int::from(*norm.offset(s as isize))
            == core::ffi::c_int::from(NOT_YET_ASSIGNED)
        {
            let end = tmpTotal.wrapping_add(u64::from(*count.offset(s as isize)) * rStep);
            let sStart = (tmpTotal >> vStepLog) as u32;
            let sEnd = (end >> vStepLog) as u32;
            let weight = sEnd.wrapping_sub(sStart);
            if weight < 1 {
                return Error::GENERIC.to_error_code();
            }
            *norm.offset(s as isize) = weight as core::ffi::c_short;
            tmpTotal = end;
        }
        s = s.wrapping_add(1);
    }
    0
}
pub(crate) unsafe fn FSE_normalizeCount(
    normalizedCounter: *mut core::ffi::c_short,
    mut tableLog: core::ffi::c_uint,
    count: *const core::ffi::c_uint,
    total: size_t,
    maxSymbolValue: core::ffi::c_uint,
    useLowProbCount: core::ffi::c_uint,
) -> size_t {
    if tableLog == 0 {
        tableLog = FSE_DEFAULT_TABLELOG as core::ffi::c_uint;
    }
    if tableLog < FSE_MIN_TABLELOG as core::ffi::c_uint {
        return Error::GENERIC.to_error_code();
    }
    if tableLog > FSE_MAX_TABLELOG as core::ffi::c_uint {
        return Error::tableLog_tooLarge.to_error_code();
    }
    if tableLog < FSE_minTableLog(total, maxSymbolValue) {
        return Error::GENERIC.to_error_code();
    }
    static rtbTable: [u32; 8] = [0, 473195, 504333, 520860, 550000, 700000, 750000, 830000];
    let lowProbCount = (if useLowProbCount != 0 { -(1) } else { 1 }) as core::ffi::c_short;
    let scale = u64::from((62 as core::ffi::c_uint).wrapping_sub(tableLog));
    let step = (1 << 62) / u64::from(total as u32);
    let vStep = 1u64.wrapping_shl(scale.wrapping_sub(20) as u32);
    let mut stillToDistribute = (1) << tableLog;
    let mut s: core::ffi::c_uint = 0;
    let mut largest = 0;
    let mut largestP = 0;
    let lowThreshold = (total >> tableLog) as u32;
    s = 0;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) as size_t == total {
            return 0;
        }
        if *count.offset(s as isize) == 0 {
            *normalizedCounter.offset(s as isize) = 0;
        } else if *count.offset(s as isize) <= lowThreshold {
            *normalizedCounter.offset(s as isize) = lowProbCount;
            stillToDistribute -= 1;
        } else {
            let mut proba =
                ((u64::from(*count.offset(s as isize)) * step) >> scale) as core::ffi::c_short;
            if core::ffi::c_int::from(proba) < 8 {
                let restToBeat = vStep * u64::from(*rtbTable.as_ptr().offset(proba as isize));
                proba = (core::ffi::c_int::from(proba)
                    + core::ffi::c_int::from(
                        (u64::from(*count.offset(s as isize)) * step)
                            .wrapping_sub((proba as u64) << scale)
                            > restToBeat,
                    )) as core::ffi::c_short;
            }
            if core::ffi::c_int::from(proba) > core::ffi::c_int::from(largestP) {
                largestP = proba;
                largest = s;
            }
            *normalizedCounter.offset(s as isize) = proba;
            stillToDistribute -= core::ffi::c_int::from(proba);
        }
        s = s.wrapping_add(1);
    }
    if -stillToDistribute
        >= core::ffi::c_int::from(*normalizedCounter.offset(largest as isize)) >> 1
    {
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
        *fresh6 = (core::ffi::c_int::from(*fresh6)
            + core::ffi::c_int::from(stillToDistribute as core::ffi::c_short))
            as core::ffi::c_short;
    }
    tableLog as size_t
}
pub(crate) unsafe fn FSE_buildCTable_rle(ct: *mut FSE_CTable, symbolValue: u8) -> size_t {
    let ptr = ct as *mut core::ffi::c_void;
    let tableU16 = (ptr as *mut u16).add(2);
    let FSCTptr = (ptr as *mut u32).add(2) as *mut core::ffi::c_void;
    let symbolTT = FSCTptr as *mut FSE_symbolCompressionTransform;
    *tableU16.sub(2) = 0;
    *tableU16.sub(1) = u16::from(symbolValue);
    *tableU16 = 0;
    *tableU16.add(1) = 0;
    (*symbolTT.offset(symbolValue as isize)).deltaNbBits = 0;
    (*symbolTT.offset(symbolValue as isize)).deltaFindState = 0;
    0
}
unsafe fn FSE_compress_usingCTable_generic(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    mut srcSize: size_t,
    ct: *const FSE_CTable,
    fast: core::ffi::c_uint,
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
    if srcSize <= 2 {
        return 0;
    }
    let initError = BIT_initCStream(&mut bitC, dst, dstSize);
    if ERR_isError(initError) {
        return 0;
    }
    if srcSize & 1 != 0 {
        ip = ip.sub(1);
        FSE_initCState2(&mut CState1, ct, u32::from(*ip));
        ip = ip.sub(1);
        FSE_initCState2(&mut CState2, ct, u32::from(*ip));
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, core::ffi::c_uint::from(*ip));
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    } else {
        ip = ip.sub(1);
        FSE_initCState2(&mut CState2, ct, u32::from(*ip));
        ip = ip.sub(1);
        FSE_initCState2(&mut CState1, ct, u32::from(*ip));
    }
    srcSize = srcSize.wrapping_sub(2);
    if (::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
        > (FSE_MAX_TABLELOG * 4 + 7) as core::ffi::c_ulong
        && srcSize & 2 != 0
    {
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, core::ffi::c_uint::from(*ip));
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, core::ffi::c_uint::from(*ip));
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
    }
    while ip > istart {
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState2, core::ffi::c_uint::from(*ip));
        if (::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
            < (FSE_MAX_TABLELOG * 2 + 7) as core::ffi::c_ulong
        {
            if fast != 0 {
                BIT_flushBitsFast(&mut bitC);
            } else {
                BIT_flushBits(&mut bitC);
            };
        }
        ip = ip.sub(1);
        FSE_encodeSymbol(&mut bitC, &mut CState1, core::ffi::c_uint::from(*ip));
        if (::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
            > (FSE_MAX_TABLELOG * 4 + 7) as core::ffi::c_ulong
        {
            ip = ip.sub(1);
            FSE_encodeSymbol(&mut bitC, &mut CState2, core::ffi::c_uint::from(*ip));
            ip = ip.sub(1);
            FSE_encodeSymbol(&mut bitC, &mut CState1, core::ffi::c_uint::from(*ip));
        }
        if fast != 0 {
            BIT_flushBitsFast(&mut bitC);
        } else {
            BIT_flushBits(&mut bitC);
        };
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
    ct: *const FSE_CTable,
) -> size_t {
    let fast = core::ffi::c_int::from(
        dstSize
            >= srcSize
                .wrapping_add(srcSize >> 7)
                .wrapping_add(4)
                .wrapping_add(::core::mem::size_of::<size_t>()),
    ) as core::ffi::c_uint;
    if fast != 0 {
        FSE_compress_usingCTable_generic(dst, dstSize, src, srcSize, ct, 1)
    } else {
        FSE_compress_usingCTable_generic(dst, dstSize, src, srcSize, ct, 0)
    }
}
