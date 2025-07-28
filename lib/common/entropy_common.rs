use std::hint::likely;

pub type size_t = std::ffi::c_ulong;
pub type unalign32 = u32;
pub type ERR_enum = ZSTD_ErrorCode;
pub type C2RustUnnamed = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed = 2;
pub const HUF_flags_bmi2: C2RustUnnamed = 1;
use crate::lib::common::{
    error_private::ERR_getErrorString,
    fse_decompress::{
        FSE_DTableHeader, FSE_DecompressWksp, FSE_decode_t, FSE_decompress_wksp_bmi2,
    },
};
use crate::lib::zstd::*;
const fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
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
const fn ZSTD_countTrailingZeros32(mut val: u32) -> u32 {
    val.trailing_zeros() as i32 as u32
}

#[inline]
const fn ZSTD_highbit32(mut val: u32) -> u32 {
    val.ilog2()
}

pub fn FSE_isError(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}

#[inline(always)]
fn FSE_readNCount_body(
    mut normalizedCounter: &mut [i16],
    mut maxSVPtr: &mut std::ffi::c_uint,
    mut tableLogPtr: &mut std::ffi::c_uint,
    headerBuffer: &[u8],
) -> size_t {
    let hbSize = headerBuffer.len();

    let iend = hbSize;
    let mut ip = 0usize;
    let mut nbBits: std::ffi::c_int = 0;
    let mut remaining: std::ffi::c_int = 0;
    let mut threshold: std::ffi::c_int = 0;
    let mut bitStream: u32 = 0;
    let mut bitCount: std::ffi::c_int = 0;
    let mut charnum = 0 as std::ffi::c_int as std::ffi::c_uint;
    let maxSV1 = (*maxSVPtr).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let mut previous_was_0 = false;
    if hbSize < 8 {
        let mut buffer = [0u8; 8];
        buffer[..hbSize].copy_from_slice(headerBuffer);

        let countSize = FSE_readNCount_slice(normalizedCounter, maxSVPtr, tableLogPtr, &mut buffer);

        if FSE_isError(countSize) != 0 {
            return countSize;
        }
        if countSize > hbSize as size_t {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        return countSize;
    }

    normalizedCounter.fill(0);

    let read_u32_le = |offset| u32::from_le_bytes(headerBuffer[offset..][..4].try_into().unwrap());

    bitStream = read_u32_le(ip);
    nbBits = (bitStream & 0xf as std::ffi::c_int as u32).wrapping_add(FSE_MIN_TABLELOG as u32)
        as std::ffi::c_int;
    if nbBits > FSE_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    bitStream >>= 4;
    bitCount = 4;
    *tableLogPtr = nbBits as std::ffi::c_uint;
    remaining = (1 << nbBits) + 1;
    threshold = 1 << nbBits;
    nbBits += 1;
    loop {
        if previous_was_0 {
            // Count the number of repeats. Each time the 2-bit repeat code is 0b11 there is another repeat.
            let mut repeats = bitStream.trailing_ones() >> 1;
            while repeats >= 12 {
                charnum = charnum.wrapping_add(3 * 12);
                if likely(ip <= iend - 7) {
                    ip += 3;
                } else {
                    bitCount -= 8 * ((iend - 7) - ip) as i32;
                    bitCount &= 31;
                    ip = iend - 4;
                }
                bitStream = read_u32_le(ip) >> bitCount;
                repeats = bitStream.trailing_ones() >> 1;
            }

            charnum = charnum.wrapping_add(3 * repeats);
            bitStream >>= 2 * repeats;
            bitCount += 2 * repeats as i32;

            /* Add the final repeat which isn't 0b11. */
            assert!((bitStream & 0b11) < 3);
            charnum = charnum.wrapping_add(bitStream & 0b11);
            bitCount += 2;

            /* This is an error, but break and return an error
             * at the end, because returning out of a loop makes
             * it harder for the compiler to optimize.
             */
            if charnum >= maxSV1 {
                break;
            }

            /* We don't need to set the normalized count to 0
             * because we already memset the whole buffer to 0.
             */

            if (ip <= iend - 7) || (ip + (bitCount as usize >> 3)) <= iend - 4 {
                ip += bitCount as usize >> 3;
                bitCount &= 0b111;
            } else {
                bitCount -= 8 * ((iend - 4) - ip) as i32;
                bitCount &= 31;
                ip = iend - 4;
            }

            bitStream = read_u32_le(ip) >> bitCount;
        }

        let max = 2 as std::ffi::c_int * threshold - 1 as std::ffi::c_int - remaining;
        let mut count: std::ffi::c_int = 0;
        if (bitStream & (threshold - 1) as u32) < max as u32 {
            count = (bitStream & (threshold - 1) as u32) as std::ffi::c_int;
            bitCount += nbBits - 1;
        } else {
            count = (bitStream & (2 * threshold - 1 as std::ffi::c_int) as u32) as std::ffi::c_int;
            if count >= threshold {
                count -= max;
            }
            bitCount += nbBits;
        }
        count -= 1;

        if count >= 0 {
            remaining -= count;
        } else {
            remaining += count;
        }

        normalizedCounter[charnum as usize] = count as std::ffi::c_short;
        charnum += 1;
        previous_was_0 = count == 0;
        if remaining < threshold {
            if remaining <= 1 as std::ffi::c_int {
                break;
            }
            nbBits = remaining.ilog2() as i32 + 1;
            threshold = (1 as std::ffi::c_int) << (nbBits - 1 as std::ffi::c_int);
        }
        if charnum >= maxSV1 {
            break;
        }
        if (ip <= iend - 7) || (ip + (bitCount as usize >> 3)) <= iend - 4 {
            ip += bitCount as usize >> 3;
            bitCount &= 7 as std::ffi::c_int;
        } else {
            bitCount -= 8 * ((iend - 4) - ip) as i32;
            bitCount &= 31;
            ip = iend - 4;
        }
        bitStream = read_u32_le(ip) >> bitCount;
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
    ip += ((bitCount + 7 as std::ffi::c_int) >> 3) as usize;

    ip as size_t
}

fn FSE_readNCount_body_default(
    mut normalizedCounter: &mut [i16],
    mut maxSVPtr: &mut std::ffi::c_uint,
    mut tableLogPtr: &mut std::ffi::c_uint,
    headerBuffer: &[u8],
) -> size_t {
    FSE_readNCount_body(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer)
}
fn FSE_readNCount_body_bmi2(
    mut normalizedCounter: &mut [i16],
    mut maxSVPtr: &mut std::ffi::c_uint,
    mut tableLogPtr: &mut std::ffi::c_uint,
    headerBuffer: &[u8],
) -> size_t {
    FSE_readNCount_body(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer)
}

pub fn FSE_readNCount_bmi2(
    mut normalizedCounter: &mut [i16],
    mut maxSVPtr: &mut std::ffi::c_uint,
    mut tableLogPtr: &mut std::ffi::c_uint,
    headerBuffer: &[u8],
    mut bmi2: std::ffi::c_int,
) -> size_t {
    if bmi2 != 0 {
        return FSE_readNCount_body_bmi2(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer);
    }
    FSE_readNCount_body_default(normalizedCounter, maxSVPtr, tableLogPtr, headerBuffer)
}

pub unsafe fn FSE_readNCount(
    mut normalizedCounter: &mut [i16],
    mut maxSVPtr: &mut std::ffi::c_uint,
    mut tableLogPtr: &mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    FSE_readNCount_slice(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        core::slice::from_raw_parts(headerBuffer.cast(), hbSize as usize),
    )
}

pub fn FSE_readNCount_slice(
    mut normalizedCounter: &mut [i16],
    mut maxSVPtr: &mut std::ffi::c_uint,
    mut tableLogPtr: &mut std::ffi::c_uint,
    headerBuffer: &[u8],
) -> size_t {
    FSE_readNCount_bmi2(
        normalizedCounter,
        maxSVPtr,
        tableLogPtr,
        headerBuffer,
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
    mut nbSymbolsPtr: &mut u32,
    mut tableLogPtr: &mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    // We can remove this at some point, it's just a check that the constants are correct.
    const _: () = assert!(HUF_READ_STATS_WORKSPACE_SIZE_U32 == 219);

    const _: () = assert!(HUF_READ_STATS_WORKSPACE_SIZE_U32 == size_of::<Workspace>() / 4);

    let mut wksp = Workspace::new();

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
    mut huffWeight: &mut [u8; 256],
    hwSize: size_t,
    mut rankStats: &mut [u32; 13],
    mut nbSymbolsPtr: &mut u32,
    mut tableLogPtr: &mut u32,
    mut ip: &[u8],
    workspace: &mut Workspace,
    mut bmi2: bool,
) -> size_t {
    let srcSize = ip.len() as size_t;

    let mut weightTotal: u32 = 0;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    iSize = ip[0] as size_t;
    if iSize >= 128 {
        // Special header case.
        oSize = iSize.wrapping_sub(127);
        iSize = oSize.wrapping_add(1) / 2;
        if iSize.wrapping_add(1) > srcSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        if oSize >= hwSize {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        ip = &ip[1..];
        for n in (0..oSize as usize).step_by(2) {
            huffWeight[n] = ip[n / 2] >> 4;
            huffWeight[n + 1] = ip[n / 2] & 0b1111;
        }
    } else {
        // Normal case: header compressed with FSE.
        if iSize.wrapping_add(1) > srcSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        oSize = FSE_decompress_wksp_bmi2(
            // At most (hwSize-1) values decoded, the last one is implied.
            &mut huffWeight[..hwSize as usize - 1],
            &ip[1..][..iSize as usize],
            6,
            // TODO this should probably be a (4-byte aligned) byte slice from the start.
            workspace,
            bmi2,
        );
        if FSE_isError(oSize) != 0 {
            return oSize;
        }
    }

    // Collect weight stats.
    rankStats[..HUF_TABLELOG_MAX + 1].fill(0);
    weightTotal = 0;
    for n in 0..oSize as usize {
        let Some(rank_stat) = rankStats.get_mut(usize::from(huffWeight[n])) else {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        };
        *rank_stat += 1;
        weightTotal += (1 << huffWeight[n] >> 1) as u32;
    }
    if weightTotal == 0 as std::ffi::c_int as u32 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }

    // Get last non-null symbol weight (implied, total must be 2^n).
    let tableLog = weightTotal.ilog2() + 1;
    if tableLog > HUF_TABLELOG_MAX as u32 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *tableLogPtr = tableLog;

    // Determine last weight.
    let total = 1u32 << tableLog;
    let rest = total.wrapping_sub(weightTotal);
    let verif = 1u32 << rest.ilog2();
    let lastWeight = rest.ilog2() + 1;
    if verif != rest {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    huffWeight[oSize as usize] = lastWeight as u8;
    rankStats[lastWeight as usize] += 1;

    // Check tree construction validity.
    if rankStats[1] < 2 || rankStats[1] & 1 != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }

    // Store results.
    *nbSymbolsPtr = oSize.wrapping_add(1) as u32;
    iSize.wrapping_add(1)
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(4))]
pub struct DTable {
    pub header: FSE_DTableHeader,
    pub v: [FSE_decode_t; 90],
}

impl DTable {
    const fn new() -> Self {
        Self {
            header: FSE_DTableHeader {
                tableLog: 0,
                fastMode: 0,
            },
            v: [FSE_decode_t {
                newState: 0,
                symbol: 0,
                nbBits: 0,
            }; 90],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Workspace {
    a: FSE_DecompressWksp,
    dtable: DTable,
}

impl Workspace {
    const _SIZE: () = assert!(size_of::<Self>() == 4 * 219);

    const fn new() -> Self {
        Self {
            a: FSE_DecompressWksp {
                ncount: [0i16; 256],
            },
            dtable: DTable::new(),
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Workspace {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            a: {
                let mut arr = [0i16; 256];
                for elem in &mut arr {
                    *elem = i16::arbitrary(g);
                }
                FSE_DecompressWksp { ncount: arr }
            },
            dtable: DTable::arbitrary(g),
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for DTable {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            header: FSE_DTableHeader {
                tableLog: u16::arbitrary(g),
                fastMode: u16::arbitrary(g),
            },
            v: {
                let mut arr = [FSE_decode_t {
                    newState: 0,
                    symbol: 0,
                    nbBits: 0,
                }; 90];
                for elem in &mut arr {
                    *elem = FSE_decode_t {
                        newState: quickcheck::Arbitrary::arbitrary(g),
                        symbol: quickcheck::Arbitrary::arbitrary(g),
                        nbBits: quickcheck::Arbitrary::arbitrary(g),
                    }
                }

                arr
            },
        }
    }
}

pub unsafe fn HUF_readStats_wksp(
    mut huffWeight: &mut [u8; 256],
    mut hwSize: size_t,
    mut rankStats: &mut [u32; 13],
    mut nbSymbolsPtr: &mut u32,
    mut tableLogPtr: &mut u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    workspace: &mut Workspace,
    mut flags: std::ffi::c_int,
) -> size_t {
    let use_bmi2 = flags & HUF_flags_bmi2 as std::ffi::c_int != 0;

    HUF_readStats_body(
        huffWeight,
        hwSize,
        rankStats,
        nbSymbolsPtr,
        tableLogPtr,
        core::slice::from_raw_parts(src.cast(), srcSize as usize),
        workspace,
        use_bmi2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::quickcheck;

    extern crate test;

    #[derive(Debug, Clone, PartialEq)]
    struct Input {
        huffWeight: [u8; 256],
        rankStats: [u32; 13],
        nbSymbolsPtr: u32,
        tableLogPtr: u32,
        src: Vec<u8>,
        workspace: Workspace,
        bmi2: bool,
    }

    impl quickcheck::Arbitrary for Input {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Input {
                huffWeight: {
                    let mut arr = [0u8; 256];
                    for elem in &mut arr {
                        *elem = u8::arbitrary(g);
                    }
                    arr
                },
                rankStats: {
                    let mut arr = [0u32; 13];
                    for elem in &mut arr {
                        *elem = u32::arbitrary(g);
                    }
                    arr
                },
                nbSymbolsPtr: u32::arbitrary(g),
                tableLogPtr: u32::arbitrary(g),
                src: Vec::<u8>::arbitrary(g),
                workspace: Workspace::arbitrary(g),
                bmi2: bool::arbitrary(g),
            }
        }
    }

    quickcheck! {
        fn new_matches_old(input: Input) -> bool {
            unsafe {
                let expected = {
                    let Input {
                        mut huffWeight,
                        mut rankStats,
                        mut nbSymbolsPtr,
                        mut tableLogPtr,
                        src,
                        workspace,
                        bmi2,
                    } = input.clone();

                    let mut workspace = unsafe { core::mem::transmute(workspace) } ;

                    let v =crate::lib::common::entropy_common_old::HUF_readStats_body(
                        &mut huffWeight,
                        256,
                        &mut rankStats,
                        &mut nbSymbolsPtr,
                        &mut tableLogPtr,
                        &src,
                        &mut workspace,
                        bmi2,
                    );
                    (v, huffWeight, rankStats, nbSymbolsPtr, tableLogPtr, workspace)
                };
                let actual = {
                    let Input {
                        mut huffWeight,
                        mut rankStats,
                        mut nbSymbolsPtr,
                        mut tableLogPtr,
                        src,
                        mut workspace,
                        bmi2,
                    } = input.clone();
                    let v = HUF_readStats_body(
                        &mut huffWeight,
                        256,
                        &mut rankStats,
                        &mut nbSymbolsPtr,
                        &mut tableLogPtr,
                        &src,
                        &mut workspace,
                        bmi2,
                    );

                    (v, huffWeight, rankStats, nbSymbolsPtr, tableLogPtr, std::mem::transmute::<_, [u32; 219]>(workspace))
                };
                assert_eq!(expected, actual);
                expected == actual
            }
        }
    }
}
