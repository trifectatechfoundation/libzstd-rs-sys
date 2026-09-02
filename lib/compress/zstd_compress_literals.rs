use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::huf::{
    HUF_CElt, HUF_flags_bmi2, HUF_flags_optimalDepth, HUF_flags_preferRepeat,
    HUF_flags_suspectUncompressible, HUF_repeat, HUF_repeat_check, HUF_repeat_none,
    HUF_repeat_valid, HUF_OPTIMAL_DEPTH_THRESHOLD, HUF_SYMBOLVALUE_MAX,
};
use crate::lib::common::mem::{MEM_writeLE16, MEM_writeLE24, MEM_writeLE32};
use crate::lib::common::zstd_internal::{LitHufLog, SymbolEncodingType};
use crate::lib::compress::huf_compress::{HUF_compress1X_repeat, HUF_compress4X_repeat};
use crate::lib::compress::zstd_compress_internal::ZSTD_minGain;
use crate::lib::compress::zstd_compress_internal::{CTable, ZSTD_hufCTables_t};
use crate::lib::zstd::{ZSTD_lazy, ZSTD_strategy};

const MIN_LITERALS_FOR_4_STREAMS: usize = 6;

pub type huf_compress_f = unsafe fn(
    *mut core::ffi::c_void,
    size_t,
    *const core::ffi::c_void,
    size_t,
    core::ffi::c_uint,
    core::ffi::c_uint,
    *mut core::ffi::c_void,
    size_t,
    &mut CTable,
    *mut HUF_repeat,
    core::ffi::c_int,
) -> size_t;

pub unsafe fn ZSTD_noCompressLiterals(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let flSize =
        (1 + (srcSize > 31) as core::ffi::c_int + (srcSize > 4095) as core::ffi::c_int) as u32;

    if srcSize.wrapping_add(flSize as size_t) > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }

    match flSize {
        1 => {
            // 2 - 1 - 5
            *ostart = (SymbolEncodingType::Basic as size_t).wrapping_add(srcSize << 3) as u8;
        }
        2 => {
            // 2 - 2 - 12
            MEM_writeLE16(
                ostart as *mut core::ffi::c_void,
                (SymbolEncodingType::Basic as size_t)
                    .wrapping_add(1 << 2)
                    .wrapping_add(srcSize << 4) as u16,
            );
        }
        3 => {
            // 2 - 2 - 20
            MEM_writeLE32(
                ostart as *mut core::ffi::c_void,
                (SymbolEncodingType::Basic as size_t)
                    .wrapping_add(3 << 2)
                    .wrapping_add(srcSize << 4) as u32,
            );
        }
        _ => {} // not necessary : flSize is {1,2,3}
    }

    core::ptr::copy_nonoverlapping(src.cast::<u8>(), ostart.offset(flSize as isize), srcSize);

    srcSize.wrapping_add(flSize as size_t)
}

unsafe fn allBytesIdentical(src: *const core::ffi::c_void, srcSize: size_t) -> bool {
    let b = *(src as *const u8);
    let mut p: size_t = 0;
    p = 1;
    while p < srcSize {
        if *(src as *const u8).add(p) as core::ffi::c_int != b as core::ffi::c_int {
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
        (1 + (srcSize > 31) as core::ffi::c_int + (srcSize > 4095) as core::ffi::c_int) as u32;

    assert!(dstCapacity >= 4);
    assert!(allBytesIdentical(src, srcSize));

    match flSize {
        1 => {
            // 2 - 1 - 5
            *ostart = (SymbolEncodingType::Rle as size_t).wrapping_add(srcSize << 3) as u8;
        }
        2 => {
            // 2 - 2 - 12
            MEM_writeLE16(
                ostart as *mut core::ffi::c_void,
                (SymbolEncodingType::Rle as size_t)
                    .wrapping_add(1 << 2)
                    .wrapping_add(srcSize << 4) as u16,
            );
        }
        3 => {
            // 2 - 2 - 20
            MEM_writeLE32(
                ostart as *mut core::ffi::c_void,
                (SymbolEncodingType::Rle as size_t)
                    .wrapping_add(3 << 2)
                    .wrapping_add(srcSize << 4) as u32,
            );
        }
        _ => {} // not necessary : flSize is {1,2,3}
    }

    *ostart.offset(flSize as isize) = *(src as *const u8);
    flSize.wrapping_add(1) as size_t
}

/// # Returns
/// The minimal amount of literals for literal compression to
/// be attempted.
/// Minimum is made tighter as compression strategy increases.
fn ZSTD_minLiteralsToCompress(strategy: ZSTD_strategy, huf_repeat: HUF_repeat) -> size_t {
    // btultra2 : min 8 bytes;
    // then 2x larger for each successive compression strategy
    // max threshold 64 bytes
    let shift = (9 - strategy as core::ffi::c_int).min(3);

    if huf_repeat == HUF_repeat_valid {
        6
    } else {
        8 << shift
    }
}

pub unsafe fn ZSTD_compressLiterals(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    entropyWorkspace: *mut core::ffi::c_void,
    entropyWorkspaceSize: size_t,
    prevHuf: &ZSTD_hufCTables_t,
    nextHuf: &mut ZSTD_hufCTables_t,
    strategy: ZSTD_strategy,
    disableLiteralCompression: bool,
    suspectUncompressible: core::ffi::c_int,
    bmi2: core::ffi::c_int,
) -> size_t {
    let lhSize = (3
        + (srcSize >= (1 << 10) as size_t) as core::ffi::c_int
        + (srcSize >= (16 * (1 << 10)) as size_t) as core::ffi::c_int) as size_t;
    let ostart = dst as *mut u8;
    let mut singleStream = srcSize < 256;
    let mut hType = SymbolEncodingType::Compressed;
    let mut cLitSize: size_t = 0;

    // Prepare nextEntropy assuming reusing the existing table
    core::ptr::copy_nonoverlapping(prevHuf, nextHuf, 1);

    if disableLiteralCompression {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }

    // if too small, don't even attempt compression (speed opt)
    if srcSize < ZSTD_minLiteralsToCompress(strategy, prevHuf.repeatMode) {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }

    if dstCapacity < lhSize.wrapping_add(1) {
        return Error::dstSize_tooSmall.to_error_code();
    }

    let mut repeat = prevHuf.repeatMode;
    let flags = (if bmi2 != 0 {
        HUF_flags_bmi2 as core::ffi::c_int
    } else {
        0
    }) | (if (strategy as core::ffi::c_uint) < ZSTD_lazy && srcSize <= 1024 {
        HUF_flags_preferRepeat as core::ffi::c_int
    } else {
        0
    }) | (if strategy >= HUF_OPTIMAL_DEPTH_THRESHOLD as core::ffi::c_uint {
        HUF_flags_optimalDepth as core::ffi::c_int
    } else {
        0
    }) | (if suspectUncompressible != 0 {
        HUF_flags_suspectUncompressible as core::ffi::c_int
    } else {
        0
    });
    if repeat == HUF_repeat_valid && lhSize == 3 {
        singleStream = true;
    }
    let huf_compress: huf_compress_f = if singleStream {
        HUF_compress1X_repeat
    } else {
        HUF_compress4X_repeat
    };
    cLitSize = huf_compress(
        ostart.add(lhSize) as *mut core::ffi::c_void,
        dstCapacity.wrapping_sub(lhSize),
        src,
        srcSize,
        HUF_SYMBOLVALUE_MAX,
        LitHufLog,
        entropyWorkspace,
        entropyWorkspaceSize,
        &mut nextHuf.CTable,
        &mut repeat,
        flags,
    );
    if repeat != HUF_repeat_none {
        // reused the existing table
        hType = SymbolEncodingType::Repeat;
    }

    let minGain = ZSTD_minGain(srcSize, strategy);
    if cLitSize == 0 || cLitSize >= srcSize.wrapping_sub(minGain) || ERR_isError(cLitSize) {
        core::ptr::copy_nonoverlapping(prevHuf, nextHuf, 1);
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }

    // A return value of 1 signals that the alphabet consists of a single symbol.
    // However, in some rare circumstances, it could be the compressed size (a single byte).
    // For that outcome to have a chance to happen, it's necessary that `srcSize < 8`.
    // (it's also necessary to not generate statistics).
    // Therefore, in such a case, actively check that all bytes are identical.
    if cLitSize == 1 && (srcSize >= 8 || allBytesIdentical(src, srcSize)) {
        core::ptr::copy_nonoverlapping(prevHuf, nextHuf, 1);
        return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src, srcSize);
    }

    if hType == SymbolEncodingType::Compressed {
        // using a newly constructed table
        nextHuf.repeatMode = HUF_repeat_check;
    }

    // Build header
    match lhSize {
        3 => {
            // 2 - 2 - 10 - 10
            if !singleStream {
                assert!(srcSize >= MIN_LITERALS_FOR_4_STREAMS)
            }

            let lhc = (hType as core::ffi::c_uint)
                .wrapping_add(((!singleStream) as core::ffi::c_int as u32) << 2)
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 14);
            MEM_writeLE24(ostart as *mut core::ffi::c_void, lhc);
        }
        4 => {
            // 2 - 2 - 14 - 14
            let lhc_0 = (hType as core::ffi::c_uint)
                .wrapping_add((2 << 2) as core::ffi::c_uint)
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 18);
            MEM_writeLE32(ostart as *mut core::ffi::c_void, lhc_0);
        }
        5 => {
            // 2 - 2 - 18 - 18
            let lhc_1 = (hType as core::ffi::c_uint)
                .wrapping_add((3 << 2) as core::ffi::c_uint)
                .wrapping_add((srcSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 22);
            MEM_writeLE32(ostart as *mut core::ffi::c_void, lhc_1);
            *ostart.add(4) = (cLitSize >> 10) as u8;
        }
        _ => {} // not possible : lhSize is {3,4,5}
    }

    lhSize.wrapping_add(cLitSize)
}
