use crate::lib::compress::huf_compress::{HUF_compress1X_repeat, HUF_compress4X_repeat};
use crate::lib::compress::zstd_compress::ZSTD_hufCTables_t;
use crate::lib::zstd::*;

pub type size_t = core::ffi::c_ulong;
pub type unalign16 = u16;
pub type unalign32 = u32;
pub type SymbolEncodingType_e = core::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type HUF_repeat = core::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = size_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
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
unsafe extern "C" fn MEM_isLittleEndian() -> core::ffi::c_uint {
    1
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut core::ffi::c_void, mut value: u16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut core::ffi::c_void, mut value: u32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: u32) -> u32 {
    in_0.swap_bytes()
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut core::ffi::c_void, mut val: u16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut u8;
        *p.offset(0) = val as u8;
        *p.offset(1) = (val as core::ffi::c_int >> 8) as u8;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut core::ffi::c_void, mut val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8).offset(2) = (val >> 16) as u8;
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut core::ffi::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> core::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as core::ffi::c_int) as size_t) as core::ffi::c_int
        as core::ffi::c_uint
}
#[inline]
unsafe extern "C" fn _force_has_format_string(mut format: *const core::ffi::c_char, mut args: ...) {
}
#[inline]
unsafe extern "C" fn ZSTD_minGain(mut srcSize: size_t, mut strat: ZSTD_strategy) -> size_t {
    let minlog =
        if strat as core::ffi::c_uint >= ZSTD_btultra as core::ffi::c_int as core::ffi::c_uint {
            (strat as u32).wrapping_sub(1)
        } else {
            6
        };
    (srcSize >> minlog).wrapping_add(2)
}
pub const LitHufLog: core::ffi::c_int = 11;
pub const HUF_SYMBOLVALUE_MAX: core::ffi::c_int = 255;
pub const HUF_OPTIMAL_DEPTH_THRESHOLD: core::ffi::c_int = ZSTD_btultra as core::ffi::c_int;
#[export_name = crate::prefix!(ZSTD_noCompressLiterals)]
pub unsafe extern "C" fn ZSTD_noCompressLiterals(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let flSize =
        (1 + (srcSize > 31) as core::ffi::c_int + (srcSize > 4095) as core::ffi::c_int) as u32;
    if srcSize.wrapping_add(flSize as size_t) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    match flSize {
        1 => {
            *ostart.offset(0) =
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
unsafe extern "C" fn allBytesIdentical(
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> core::ffi::c_int {
    let b = *(src as *const u8).offset(0);
    let mut p: size_t = 0;
    p = 1;
    while p < srcSize {
        if *(src as *const u8).offset(p as isize) as core::ffi::c_int != b as core::ffi::c_int {
            return 0;
        }
        p = p.wrapping_add(1);
    }
    1
}
#[export_name = crate::prefix!(ZSTD_compressRleLiteralsBlock)]
pub unsafe extern "C" fn ZSTD_compressRleLiteralsBlock(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let ostart = dst as *mut u8;
    let flSize =
        (1 + (srcSize > 31) as core::ffi::c_int + (srcSize > 4095) as core::ffi::c_int) as u32;
    match flSize {
        1 => {
            *ostart.offset(0) =
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
unsafe extern "C" fn ZSTD_minLiteralsToCompress(
    mut strategy: ZSTD_strategy,
    mut huf_repeat: HUF_repeat,
) -> size_t {
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
#[export_name = crate::prefix!(ZSTD_compressLiterals)]
pub unsafe extern "C" fn ZSTD_compressLiterals(
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut entropyWorkspace: *mut core::ffi::c_void,
    mut entropyWorkspaceSize: size_t,
    mut prevHuf: *const ZSTD_hufCTables_t,
    mut nextHuf: *mut ZSTD_hufCTables_t,
    mut strategy: ZSTD_strategy,
    mut disableLiteralCompression: core::ffi::c_int,
    mut suspectUncompressible: core::ffi::c_int,
    mut bmi2: core::ffi::c_int,
) -> size_t {
    let lhSize = (3
        + (srcSize >= (1 * ((1) << 10)) as size_t) as core::ffi::c_int
        + (srcSize >= (16 * ((1) << 10)) as size_t) as core::ffi::c_int) as size_t;
    let ostart = dst as *mut u8;
    let mut singleStream = (srcSize < 256) as core::ffi::c_int as u32;
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
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    let mut repeat = (*prevHuf).repeatMode;
    let flags =
        0 | (if bmi2 != 0 {
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
        ostart.offset(lhSize as isize) as *mut core::ffi::c_void,
        dstCapacity.wrapping_sub(lhSize),
        src,
        srcSize,
        HUF_SYMBOLVALUE_MAX as core::ffi::c_uint,
        LitHufLog as core::ffi::c_uint,
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
    if cLitSize == 0 || cLitSize >= srcSize.wrapping_sub(minGain) || ERR_isError(cLitSize) != 0 {
        libc::memcpy(
            nextHuf as *mut core::ffi::c_void,
            prevHuf as *const core::ffi::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as core::ffi::c_ulong as libc::size_t,
        );
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if cLitSize == 1 && (srcSize >= 8 || allBytesIdentical(src, srcSize) != 0) {
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
            singleStream == 0;
            let lhc = (hType as core::ffi::c_uint)
                .wrapping_add(((singleStream == 0) as core::ffi::c_int as u32) << 2)
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
            *ostart.offset(4) = (cLitSize >> 10) as u8;
        }
        _ => {}
    }
    lhSize.wrapping_add(cLitSize)
}
