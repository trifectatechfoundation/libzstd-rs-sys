use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::huf::{
    HUF_CElt, HUF_flags_bmi2, HUF_flags_optimalDepth, HUF_flags_preferRepeat,
    HUF_flags_suspectUncompressible, HUF_repeat, HUF_repeat_check, HUF_repeat_none,
    HUF_repeat_valid, HUF_OPTIMAL_DEPTH_THRESHOLD, HUF_SYMBOLVALUE_MAX,
};
use crate::lib::common::mem::{MEM_writeLE16, MEM_writeLE24, MEM_writeLE32};
use crate::lib::common::zstd_internal::LitHufLog;
use crate::lib::compress::huf_compress::{HUF_compress1X_repeat, HUF_compress4X_repeat};
use crate::lib::compress::zstd_compress::ZSTD_hufCTables_t;
use crate::lib::zstd::*;

const MIN_LITERALS_FOR_4_STREAMS: usize = 6;

pub type SymbolEncodingType_e = core::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type huf_compress_f = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        size_t,
        *const core::ffi::c_void,
        size_t,
        core::ffi::c_uint,
        core::ffi::c_uint,
        *mut core::ffi::c_void,
        size_t,
        *mut HUF_CElt,
        *mut HUF_repeat,
        core::ffi::c_int,
    ) -> size_t,
>;
#[inline]
unsafe fn ZSTD_minGain(srcSize: size_t, strat: ZSTD_strategy) -> size_t {
    let minlog =
        if strat as core::ffi::c_uint >= ZSTD_btultra as core::ffi::c_int as core::ffi::c_uint {
            strat.wrapping_sub(1)
        } else {
            6
        };
    (srcSize >> minlog).wrapping_add(2)
}
pub unsafe fn ZSTD_noCompressLiterals(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let flSize =
        (1 + core::ffi::c_int::from(srcSize > 31) + core::ffi::c_int::from(srcSize > 4095)) as u32;
    if srcSize.wrapping_add(flSize as size_t) > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }
    match flSize {
        1 => {
            *ostart =
                (set_basic as core::ffi::c_int as u32 as size_t).wrapping_add(srcSize << 3) as u8;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut core::ffi::c_void,
                ((set_basic as core::ffi::c_int as u32).wrapping_add(((1) << 2) as u32) as size_t)
                    .wrapping_add(srcSize << 4) as u16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut core::ffi::c_void,
                ((set_basic as core::ffi::c_int as u32).wrapping_add(((3) << 2) as u32) as size_t)
                    .wrapping_add(srcSize << 4) as u32,
            );
        }
        _ => {}
    }
    libc::memcpy(
        ostart.offset(flSize as isize) as *mut core::ffi::c_void,
        src,
        srcSize as libc::size_t,
    );
    srcSize.wrapping_add(flSize as size_t)
}
unsafe fn allBytesIdentical(src: *const core::ffi::c_void, srcSize: size_t) -> bool {
    let b = *(src as *const u8);
    let mut p: size_t = 0;
    p = 1;
    while p < srcSize {
        if core::ffi::c_int::from(*(src as *const u8).add(p)) != core::ffi::c_int::from(b) {
            return false;
        }
        p = p.wrapping_add(1);
    }
    true
}
pub unsafe fn ZSTD_compressRleLiteralsBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let flSize =
        (1 + core::ffi::c_int::from(srcSize > 31) + core::ffi::c_int::from(srcSize > 4095)) as u32;

    assert!(dstCapacity >= 4);
    assert!(allBytesIdentical(src, srcSize));

    match flSize {
        1 => {
            *ostart =
                (set_rle as core::ffi::c_int as u32 as size_t).wrapping_add(srcSize << 3) as u8;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut core::ffi::c_void,
                ((set_rle as core::ffi::c_int as u32).wrapping_add(((1) << 2) as u32) as size_t)
                    .wrapping_add(srcSize << 4) as u16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut core::ffi::c_void,
                ((set_rle as core::ffi::c_int as u32).wrapping_add(((3) << 2) as u32) as size_t)
                    .wrapping_add(srcSize << 4) as u32,
            );
        }
        _ => {}
    }
    *ostart.offset(flSize as isize) = *(src as *const u8);
    flSize.wrapping_add(1) as size_t
}
unsafe fn ZSTD_minLiteralsToCompress(strategy: ZSTD_strategy, huf_repeat: HUF_repeat) -> size_t {
    let shift = if (9 - strategy as core::ffi::c_int) < 3 {
        9 - strategy as core::ffi::c_int
    } else {
        3
    };

    if huf_repeat as core::ffi::c_uint == HUF_repeat_valid as core::ffi::c_int as core::ffi::c_uint
    {
        6
    } else {
        (8) << shift
    }
}
pub unsafe fn ZSTD_compressLiterals(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    entropyWorkspace: *mut core::ffi::c_void,
    entropyWorkspaceSize: size_t,
    prevHuf: *const ZSTD_hufCTables_t,
    nextHuf: *mut ZSTD_hufCTables_t,
    strategy: ZSTD_strategy,
    disableLiteralCompression: core::ffi::c_int,
    suspectUncompressible: core::ffi::c_int,
    bmi2: core::ffi::c_int,
) -> size_t {
    let lhSize =
        (3 + core::ffi::c_int::from(srcSize >= ((1) << 10) as size_t)
            + core::ffi::c_int::from(srcSize >= (16 * ((1) << 10)) as size_t)) as size_t;
    let ostart = dst as *mut u8;
    let mut singleStream = core::ffi::c_int::from(srcSize < 256) as u32;
    let mut hType = set_compressed;
    let mut cLitSize: size_t = 0;
    libc::memcpy(
        nextHuf as *mut core::ffi::c_void,
        prevHuf as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_hufCTables_t>() as core::ffi::c_ulong as libc::size_t,
    );
    if disableLiteralCompression != 0 {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if srcSize < ZSTD_minLiteralsToCompress(strategy, (*prevHuf).repeatMode) {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if dstCapacity < lhSize.wrapping_add(1) {
        return Error::dstSize_tooSmall.to_error_code();
    }
    let mut repeat = (*prevHuf).repeatMode;
    let flags =
        (if bmi2 != 0 {
            HUF_flags_bmi2 as core::ffi::c_int
        } else {
            0
        }) | (if (strategy as core::ffi::c_uint)
            < ZSTD_lazy as core::ffi::c_int as core::ffi::c_uint
            && srcSize <= 1024
        {
            HUF_flags_preferRepeat as core::ffi::c_int
        } else {
            0
        }) | (if strategy as core::ffi::c_uint >= HUF_OPTIMAL_DEPTH_THRESHOLD as core::ffi::c_uint {
            HUF_flags_optimalDepth as core::ffi::c_int
        } else {
            0
        }) | (if suspectUncompressible != 0 {
            HUF_flags_suspectUncompressible as core::ffi::c_int
        } else {
            0
        });
    let mut huf_compress: huf_compress_f = None;
    if repeat as core::ffi::c_uint == HUF_repeat_valid as core::ffi::c_int as core::ffi::c_uint
        && lhSize == 3
    {
        singleStream = 1;
    }
    huf_compress = if singleStream != 0 {
        Some(
            HUF_compress1X_repeat
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                    core::ffi::c_uint,
                    core::ffi::c_uint,
                    *mut core::ffi::c_void,
                    size_t,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    core::ffi::c_int,
                ) -> size_t,
        )
    } else {
        Some(
            HUF_compress4X_repeat
                as unsafe extern "C" fn(
                    *mut core::ffi::c_void,
                    size_t,
                    *const core::ffi::c_void,
                    size_t,
                    core::ffi::c_uint,
                    core::ffi::c_uint,
                    *mut core::ffi::c_void,
                    size_t,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    core::ffi::c_int,
                ) -> size_t,
        )
    };
    cLitSize = huf_compress.unwrap_unchecked()(
        ostart.add(lhSize) as *mut core::ffi::c_void,
        dstCapacity.wrapping_sub(lhSize),
        src,
        srcSize,
        HUF_SYMBOLVALUE_MAX,
        LitHufLog,
        entropyWorkspace,
        entropyWorkspaceSize,
        ((*nextHuf).CTable).as_mut_ptr(),
        &mut repeat,
        flags,
    );
    if repeat as core::ffi::c_uint != HUF_repeat_none as core::ffi::c_int as core::ffi::c_uint {
        hType = set_repeat;
    }
    let minGain = ZSTD_minGain(srcSize, strategy);
    if cLitSize == 0 || cLitSize >= srcSize.wrapping_sub(minGain) || ERR_isError(cLitSize) {
        libc::memcpy(
            nextHuf as *mut core::ffi::c_void,
            prevHuf as *const core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as core::ffi::c_ulong as libc::size_t,
        );
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if cLitSize == 1 && (srcSize >= 8 || allBytesIdentical(src, srcSize)) {
        libc::memcpy(
            nextHuf as *mut core::ffi::c_void,
            prevHuf as *const core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as core::ffi::c_ulong as libc::size_t,
        );
        return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src, srcSize);
    }
    if hType as core::ffi::c_uint == set_compressed as core::ffi::c_int as core::ffi::c_uint {
        (*nextHuf).repeatMode = HUF_repeat_check;
    }
    match lhSize {
        3 => {
            if singleStream == 0 {
                assert!(srcSize >= MIN_LITERALS_FOR_4_STREAMS)
            }

            let lhc = (hType as core::ffi::c_uint)
                .wrapping_add((core::ffi::c_int::from(singleStream == 0) as u32) << 2)
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 14);
            MEM_writeLE24(ostart as *mut core::ffi::c_void, lhc);
        }
        4 => {
            let lhc_0 = (hType as core::ffi::c_uint)
                .wrapping_add(((2) << 2) as core::ffi::c_uint)
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 18);
            MEM_writeLE32(ostart as *mut core::ffi::c_void, lhc_0);
        }
        5 => {
            let lhc_1 = (hType as core::ffi::c_uint)
                .wrapping_add(((3) << 2) as core::ffi::c_uint)
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 22);
            MEM_writeLE32(ostart as *mut core::ffi::c_void, lhc_1);
            *ostart.add(4) = (cLitSize >> 10) as u8;
        }
        _ => {}
    }
    lhSize.wrapping_add(cLitSize)
}
