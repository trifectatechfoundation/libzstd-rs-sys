#[cfg(target_arch = "x86")]
use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

use libc::size_t;

pub unsafe fn ZSTD_copy8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    libc::memcpy(dst, src, 8);
}
pub unsafe fn ZSTD_copy16(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub const WILDCOPY_OVERLENGTH: usize = 32;
pub const WILDCOPY_VECLEN: core::ffi::c_int = 16;

pub type ZSTD_overlap_e = core::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;

#[inline(always)]
pub unsafe fn ZSTD_wildcopy(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    length: size_t,
    ovtype: ZSTD_overlap_e,
) {
    let diff = (dst as *mut u8).offset_from(src as *const u8);
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.add(length);
    if ovtype as core::ffi::c_uint
        == ZSTD_overlap_src_before_dst as core::ffi::c_int as core::ffi::c_uint
        && diff < WILDCOPY_VECLEN as isize
    {
        loop {
            ZSTD_copy8(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(8);
            ip = ip.offset(8);
            if op >= oend {
                break;
            }
        }
    } else {
        ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
        if 16 >= length {
            return;
        }
        op = op.offset(16);
        ip = ip.offset(16);
        loop {
            ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            if op >= oend {
                break;
            }
        }
    };
}

#[inline]
pub unsafe fn ZSTD_limitCopy(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let length = if dstCapacity < srcSize {
        dstCapacity
    } else {
        srcSize
    };
    if length > 0 {
        libc::memcpy(dst, src, length as libc::size_t);
    }
    length
}

pub const ZSTD_WORKSPACETOOLARGE_FACTOR: core::ffi::c_int = 3;
pub const ZSTD_WORKSPACETOOLARGE_MAXDURATION: core::ffi::c_int = 128;
