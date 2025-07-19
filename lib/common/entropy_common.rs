use ::libc;

pub type size_t = std::ffi::c_ulong;
pub type __uint8_t = std::ffi::c_uchar;
pub type __uint32_t = std::ffi::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type unalign32 = U32;
pub type ZSTD_ErrorCode = std::ffi::c_uint;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_externalSequences_invalid: ZSTD_ErrorCode = 107;
pub const ZSTD_error_sequenceProducer_failed: ZSTD_ErrorCode = 106;
pub const ZSTD_error_srcBuffer_wrong: ZSTD_ErrorCode = 105;
pub const ZSTD_error_dstBuffer_wrong: ZSTD_ErrorCode = 104;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ZSTD_ErrorCode = 82;
pub const ZSTD_error_noForwardProgress_destFull: ZSTD_ErrorCode = 80;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ZSTD_ErrorCode = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: ZSTD_ErrorCode = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_combination_unsupported: ZSTD_ErrorCode = 41;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_literals_headerWrong: ZSTD_ErrorCode = 24;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type ERR_enum = ZSTD_ErrorCode;
pub type C2RustUnnamed = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed = 2;
pub const HUF_flags_bmi2: C2RustUnnamed = 1;
use crate::{
    lib::common::{error_private::ERR_getErrorString, fse_decompress::FSE_decompress_wksp_bmi2},
    MEM_readLE32,
};
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    (0 as std::ffi::c_int as size_t).wrapping_sub(code) as ERR_enum
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorString(ERR_getErrorCode(code))
}
pub const FSE_VERSION_MAJOR: std::ffi::c_int = 0 as std::ffi::c_int;
pub const FSE_VERSION_MINOR: std::ffi::c_int = 9 as std::ffi::c_int;
pub const FSE_VERSION_RELEASE: std::ffi::c_int = 0 as std::ffi::c_int;
pub const FSE_VERSION_NUMBER: std::ffi::c_int =
    FSE_VERSION_MAJOR * 100 as std::ffi::c_int * 100 as std::ffi::c_int
        + FSE_VERSION_MINOR * 100 as std::ffi::c_int
        + FSE_VERSION_RELEASE;
pub const FSE_MIN_TABLELOG: std::ffi::c_int = 5 as std::ffi::c_int;
pub const FSE_TABLELOG_ABSOLUTE_MAX: std::ffi::c_int = 15 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: U32) -> std::ffi::c_uint {
    val.trailing_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}

pub unsafe fn FSE_isError(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}

#[inline(always)]
unsafe extern "C" fn FSE_readNCount_body(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut maxSVPtr: *mut std::ffi::c_uint,
    mut tableLogPtr: *mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart = headerBuffer as *const BYTE;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: std::ffi::c_int = 0;
    let mut remaining: std::ffi::c_int = 0;
    let mut threshold: std::ffi::c_int = 0;
    let mut bitStream: U32 = 0;
    let mut bitCount: std::ffi::c_int = 0;
    let mut charnum = 0 as std::ffi::c_int as std::ffi::c_uint;
    let maxSV1 = (*maxSVPtr).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let mut previous0 = 0 as std::ffi::c_int;
    if hbSize < 8 as std::ffi::c_int as size_t {
        let mut buffer: [std::ffi::c_char; 8] = [0 as std::ffi::c_int as std::ffi::c_char; 8];
        libc::memcpy(
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            headerBuffer,
            hbSize as libc::size_t,
        );
        let countSize = FSE_readNCount(
            normalizedCounter,
            maxSVPtr,
            tableLogPtr,
            buffer.as_mut_ptr() as *const std::ffi::c_void,
            ::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong,
        );
        if FSE_isError(countSize) != 0 {
            return countSize;
        }
        if countSize > hbSize {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        return countSize;
    }
    libc::memset(
        normalizedCounter as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ((*maxSVPtr).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_short>() as std::ffi::c_ulong)
            as libc::size_t,
    );
    bitStream = MEM_readLE32(ip as *const std::ffi::c_void);
    nbBits = (bitStream & 0xf as std::ffi::c_int as U32).wrapping_add(FSE_MIN_TABLELOG as U32)
        as std::ffi::c_int;
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    bitStream >>= 4 as std::ffi::c_int;
    bitCount = 4 as std::ffi::c_int;
    *tableLogPtr = nbBits as std::ffi::c_uint;
    remaining = ((1 as std::ffi::c_int) << nbBits) + 1 as std::ffi::c_int;
    threshold = (1 as std::ffi::c_int) << nbBits;
    nbBits += 1;
    nbBits;
    loop {
        if previous0 != 0 {
            let mut repeats =
                (ZSTD_countTrailingZeros32(!bitStream | 0x80000000 as std::ffi::c_uint)
                    >> 1 as std::ffi::c_int) as std::ffi::c_int;
            while repeats >= 12 as std::ffi::c_int {
                charnum = charnum.wrapping_add(
                    (3 as std::ffi::c_int * 12 as std::ffi::c_int) as std::ffi::c_uint,
                );
                if (ip <= iend.offset(-(7 as std::ffi::c_int as isize))) as std::ffi::c_int
                    as std::ffi::c_long
                    != 0
                {
                    ip = ip.offset(3 as std::ffi::c_int as isize);
                } else {
                    bitCount -= (8 as std::ffi::c_int as std::ffi::c_long
                        * iend
                            .offset(-(7 as std::ffi::c_int as isize))
                            .offset_from(ip) as std::ffi::c_long)
                        as std::ffi::c_int;
                    bitCount &= 31 as std::ffi::c_int;
                    ip = iend.offset(-(4 as std::ffi::c_int as isize));
                }
                bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
                repeats = (ZSTD_countTrailingZeros32(!bitStream | 0x80000000 as std::ffi::c_uint)
                    >> 1 as std::ffi::c_int) as std::ffi::c_int;
            }
            charnum = charnum.wrapping_add((3 as std::ffi::c_int * repeats) as std::ffi::c_uint);
            bitStream >>= 2 as std::ffi::c_int * repeats;
            bitCount += 2 as std::ffi::c_int * repeats;
            charnum = charnum.wrapping_add(bitStream & 3 as std::ffi::c_int as U32);
            bitCount += 2 as std::ffi::c_int;
            if charnum >= maxSV1 {
                break;
            }
            if (ip <= iend.offset(-(7 as std::ffi::c_int as isize))) as std::ffi::c_int
                as std::ffi::c_long
                != 0
                || ip.offset((bitCount >> 3 as std::ffi::c_int) as isize)
                    <= iend.offset(-(4 as std::ffi::c_int as isize))
            {
                ip = ip.offset((bitCount >> 3 as std::ffi::c_int) as isize);
                bitCount &= 7 as std::ffi::c_int;
            } else {
                bitCount -= (8 as std::ffi::c_int as std::ffi::c_long
                    * iend
                        .offset(-(4 as std::ffi::c_int as isize))
                        .offset_from(ip) as std::ffi::c_long)
                    as std::ffi::c_int;
                bitCount &= 31 as std::ffi::c_int;
                ip = iend.offset(-(4 as std::ffi::c_int as isize));
            }
            bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
        }
        let max = 2 as std::ffi::c_int * threshold - 1 as std::ffi::c_int - remaining;
        let mut count: std::ffi::c_int = 0;
        if (bitStream & (threshold - 1 as std::ffi::c_int) as U32) < max as U32 {
            count = (bitStream & (threshold - 1 as std::ffi::c_int) as U32) as std::ffi::c_int;
            bitCount += nbBits - 1 as std::ffi::c_int;
        } else {
            count = (bitStream & (2 as std::ffi::c_int * threshold - 1 as std::ffi::c_int) as U32)
                as std::ffi::c_int;
            if count >= threshold {
                count -= max;
            }
            bitCount += nbBits;
        }
        count -= 1;
        count;
        if count >= 0 as std::ffi::c_int {
            remaining -= count;
        } else {
            remaining += count;
        }
        let fresh0 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh0 as isize) = count as std::ffi::c_short;
        previous0 = (count == 0) as std::ffi::c_int;
        if remaining < threshold {
            if remaining <= 1 as std::ffi::c_int {
                break;
            }
            nbBits = (ZSTD_highbit32(remaining as U32))
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int;
            threshold = (1 as std::ffi::c_int) << (nbBits - 1 as std::ffi::c_int);
        }
        if charnum >= maxSV1 {
            break;
        }
        if (ip <= iend.offset(-(7 as std::ffi::c_int as isize))) as std::ffi::c_int
            as std::ffi::c_long
            != 0
            || ip.offset((bitCount >> 3 as std::ffi::c_int) as isize)
                <= iend.offset(-(4 as std::ffi::c_int as isize))
        {
            ip = ip.offset((bitCount >> 3 as std::ffi::c_int) as isize);
            bitCount &= 7 as std::ffi::c_int;
        } else {
            bitCount -= (8 as std::ffi::c_int as std::ffi::c_long
                * iend
                    .offset(-(4 as std::ffi::c_int as isize))
                    .offset_from(ip) as std::ffi::c_long)
                as std::ffi::c_int;
            bitCount &= 31 as std::ffi::c_int;
            ip = iend.offset(-(4 as std::ffi::c_int as isize));
        }
        bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
    }
    if remaining != 1 as std::ffi::c_int {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if charnum > maxSV1 {
        return -(ZSTD_error_maxSymbolValue_tooSmall as std::ffi::c_int) as size_t;
    }
    if bitCount > 32 as std::ffi::c_int {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    ip = ip.offset(((bitCount + 7 as std::ffi::c_int) >> 3 as std::ffi::c_int) as isize);
    ip.offset_from(istart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn FSE_readNCount_body_default(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut maxSVPtr: *mut std::ffi::c_uint,
    mut tableLogPtr: *mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    FSE_readNCount_body(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
    )
}
unsafe extern "C" fn FSE_readNCount_body_bmi2(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut maxSVPtr: *mut std::ffi::c_uint,
    mut tableLogPtr: *mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    FSE_readNCount_body(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
    )
}

pub unsafe fn FSE_readNCount_bmi2(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut maxSVPtr: *mut std::ffi::c_uint,
    mut tableLogPtr: *mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
    mut bmi2: std::ffi::c_int,
) -> size_t {
    if bmi2 != 0 {
        return FSE_readNCount_body_bmi2(
            normalizedCounter,
            maxSVPtr,
            tableLogPtr,
            headerBuffer,
            hbSize,
        );
    }
    FSE_readNCount_body_default(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
    )
}

pub unsafe fn FSE_readNCount(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut maxSVPtr: *mut std::ffi::c_uint,
    mut tableLogPtr: *mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    FSE_readNCount_bmi2(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
        hbSize,
        0 as std::ffi::c_int,
    )
}

/// Max runtime value of tableLog (due to static allocation); can be modified up to HUF_TABLELOG_ABSOLUTEMAX.
const HUF_TABLELOG_MAX: usize = 12;

const fn FSE_DTABLE_SIZE_U32(maxTableLog: usize) -> usize {
    1 + (1 << (maxTableLog))
}

const fn FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog: usize, maxSymbolValue: usize) -> usize {
    size_of::<u16>() * (maxSymbolValue + 1) + (1 << maxTableLog) + 8
}

/// Maximum symbol value authorized.
const FSE_MAX_SYMBOL_VALUE: usize = 255;

const fn FSE_DECOMPRESS_WKSP_SIZE_U32(maxTableLog: usize, maxSymbolValue: usize) -> usize {
    FSE_DTABLE_SIZE_U32(maxTableLog)
        + 1
        + FSE_BUILD_DTABLE_WKSP_SIZE(maxTableLog, maxSymbolValue).div_ceil(size_of::<u32>())
        + (FSE_MAX_SYMBOL_VALUE + 1) / 2
        + 1
}
const HUF_READ_STATS_WORKSPACE_SIZE_U32: usize =
    FSE_DECOMPRESS_WKSP_SIZE_U32(6, HUF_TABLELOG_MAX - 1);

pub unsafe fn HUF_readStats(
    mut huffWeight: &mut [u8; 256],
    mut hwSize: size_t,
    mut rankStats: &mut [u32; 13],
    mut nbSymbolsPtr: &mut U32,
    mut tableLogPtr: &mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    // We can remove this at some point, it's just a check that the constants are correct.
    const _: () = assert!(HUF_READ_STATS_WORKSPACE_SIZE_U32 == 219);

    let mut wksp: [u32; HUF_READ_STATS_WORKSPACE_SIZE_U32] = [0; HUF_READ_STATS_WORKSPACE_SIZE_U32];

    HUF_readStats_wksp(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        &mut wksp,
        0 as std::ffi::c_int,
    )
}
#[inline(always)]
unsafe fn HUF_readStats_body(
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut bmi2: bool,
) -> size_t {
    let mut weightTotal: U32 = 0;
    let mut ip = src as *const BYTE;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    iSize = *ip.offset(0 as std::ffi::c_int as isize) as size_t;
    if iSize >= 128 as std::ffi::c_int as size_t {
        oSize = iSize.wrapping_sub(127 as std::ffi::c_int as size_t);
        iSize = oSize.wrapping_add(1 as std::ffi::c_int as size_t) / 2 as std::ffi::c_int as size_t;
        if iSize.wrapping_add(1 as std::ffi::c_int as size_t) > srcSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        if oSize >= hwSize {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        ip = ip.offset(1 as std::ffi::c_int as isize);
        let mut n: U32 = 0;
        n = 0 as std::ffi::c_int as U32;
        while (n as size_t) < oSize {
            *huffWeight.offset(n as isize) = (*ip.offset((n / 2 as std::ffi::c_int as U32) as isize)
                as std::ffi::c_int
                >> 4 as std::ffi::c_int) as BYTE;
            *huffWeight.offset(n.wrapping_add(1 as std::ffi::c_int as U32) as isize) =
                (*ip.offset((n / 2 as std::ffi::c_int as U32) as isize) as std::ffi::c_int
                    & 15 as std::ffi::c_int) as BYTE;
            n = n.wrapping_add(2 as std::ffi::c_int as U32);
        }
    } else {
        if iSize.wrapping_add(1 as std::ffi::c_int as size_t) > srcSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        oSize = FSE_decompress_wksp_bmi2(
            huffWeight as *mut std::ffi::c_void,
            hwSize.wrapping_sub(1 as std::ffi::c_int as size_t),
            ip.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void,
            iSize,
            6 as std::ffi::c_int as std::ffi::c_uint,
            workSpace,
            wkspSize,
            bmi2,
        );
        if FSE_isError(oSize) != 0 {
            return oSize;
        }
    }
    libc::memset(
        rankStats as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ((12 as std::ffi::c_int + 1 as std::ffi::c_int) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<U32>() as std::ffi::c_ulong)
            as libc::size_t,
    );
    weightTotal = 0 as std::ffi::c_int as U32;
    let mut n_0: U32 = 0;
    n_0 = 0 as std::ffi::c_int as U32;
    while (n_0 as size_t) < oSize {
        if usize::from(*huffWeight.offset(n_0 as isize)) > HUF_TABLELOG_MAX {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        let fresh1 = &mut (*rankStats.offset(*huffWeight.offset(n_0 as isize) as isize));
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
        weightTotal = weightTotal.wrapping_add(
            ((1 as std::ffi::c_int) << *huffWeight.offset(n_0 as isize) as std::ffi::c_int
                >> 1 as std::ffi::c_int) as U32,
        );
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    if weightTotal == 0 as std::ffi::c_int as U32 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let tableLog =
        (ZSTD_highbit32(weightTotal)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    if tableLog > HUF_TABLELOG_MAX as U32 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *tableLogPtr = tableLog;
    let total = ((1 as std::ffi::c_int) << tableLog) as U32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1 as std::ffi::c_int) << ZSTD_highbit32(rest)) as U32;
    let lastWeight = (ZSTD_highbit32(rest)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as BYTE;
    let fresh2 = &mut (*rankStats.offset(lastWeight as isize));
    *fresh2 = (*fresh2).wrapping_add(1);
    *fresh2;
    if *rankStats.offset(1 as std::ffi::c_int as isize) < 2 as std::ffi::c_int as U32
        || *rankStats.offset(1 as std::ffi::c_int as isize) & 1 as std::ffi::c_int as U32 != 0
    {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1 as std::ffi::c_int as size_t) as U32;
    iSize.wrapping_add(1 as std::ffi::c_int as size_t)
}

pub unsafe fn HUF_readStats_wksp(
    mut huffWeight: &mut [u8; 256],
    mut hwSize: size_t,
    mut rankStats: &mut [U32; 13],
    mut nbSymbolsPtr: &mut U32,
    mut tableLogPtr: &mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    workspace: &mut [u32; 219],
    mut flags: std::ffi::c_int,
) -> size_t {
    let use_bmi2 = flags & HUF_flags_bmi2 as std::ffi::c_int != 0;

    HUF_readStats_body(
        huffWeight.as_mut_ptr(),
        hwSize,
        rankStats.as_mut_ptr(),
        nbSymbolsPtr,
        tableLogPtr,
        src,
        srcSize,
        workspace.as_mut_ptr().cast(),
        (4 * workspace.len()) as size_t,
        use_bmi2,
    )
}
