#[cfg(target_arch = "x86")]
use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

use libc::size_t;

const fn const_max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

pub(crate) const ZSTD_OPT_NUM: core::ffi::c_int = 1 << 12;

pub(crate) const ZSTD_REP_NUM: core::ffi::c_int = 3;
pub(crate) static repStartValue: [u32; 3] = [1, 4, 8];

pub(crate) const ZSTD_FRAMEIDSIZE: usize = 4;

const ZSTD_BLOCKHEADERSIZE: core::ffi::c_int = 3;
pub(crate) static ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub(crate) type blockType_e = core::ffi::c_uint;
pub(crate) const bt_raw: blockType_e = 0;
pub(crate) const bt_rle: blockType_e = 1;
pub(crate) const bt_compressed: blockType_e = 2;

pub(crate) const MINMATCH: core::ffi::c_int = 3;

pub(crate) const Litbits: core::ffi::c_int = 8;
pub(crate) const LitHufLog: core::ffi::c_int = 11;
pub(crate) const MaxLit: core::ffi::c_int = ((1) << Litbits) - 1;
pub(crate) const MaxML: core::ffi::c_int = 52;
pub(crate) const MaxLL: core::ffi::c_int = 35;
pub(crate) const DefaultMaxOff: core::ffi::c_int = 28;
pub(crate) const MaxOff: core::ffi::c_int = 31;
pub(crate) const MaxSeq: usize = const_max(MaxLL as usize, MaxML as usize); /* Assumption : MaxOff < MaxLL,MaxML */
pub(crate) const MLFSELog: core::ffi::c_int = 9;
pub(crate) const LLFSELog: core::ffi::c_int = 9;
pub(crate) const OffFSELog: core::ffi::c_int = 8;
pub(crate) const MaxFSELog: usize = const_max(
    const_max(MLFSELog as usize, LLFSELog as usize),
    OffFSELog as usize,
);
pub(crate) const MaxMLBits: u8 = 16;
pub(crate) const MaxLLBits: u8 = 16;

pub(crate) static LL_bits: [u8; 36] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16,
];
pub(crate) static LL_defaultNorm: [i16; 36] = [
    4, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 1, 1, 1, 1, 1,
    -1, -1, -1, -1,
];
pub(crate) const LL_DEFAULTNORMLOG: u32 = 6;
pub(crate) static LL_defaultNormLog: u32 = LL_DEFAULTNORMLOG;
pub(crate) static ML_bits: [u8; 53] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
];
pub(crate) static ML_defaultNorm: [i16; 53] = [
    1, 4, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1, -1, -1,
];
pub(crate) const ML_DEFAULTNORMLOG: u32 = 6;
pub(crate) static ML_defaultNormLog: u32 = ML_DEFAULTNORMLOG;
pub(crate) static OF_defaultNorm: [i16; 29] = [
    1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1,
];
pub(crate) const OF_DEFAULTNORMLOG: u32 = 5;
pub(crate) static OF_defaultNormLog: u32 = OF_DEFAULTNORMLOG;

pub(crate) unsafe fn ZSTD_copy8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    libc::memcpy(dst, src, 8);
}
pub(crate) unsafe fn ZSTD_copy16(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub(crate) const WILDCOPY_OVERLENGTH: usize = 32;
pub(crate) const WILDCOPY_VECLEN: core::ffi::c_int = 16;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Overlap {
    NoOverlap,
    OverlapSrcBeforeDst,
}

#[inline(always)]
pub(crate) unsafe fn ZSTD_wildcopy(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    length: size_t,
    ovtype: Overlap,
) {
    let diff = (dst as *mut u8).offset_from(src as *const u8);
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.add(length);
    if ovtype == Overlap::OverlapSrcBeforeDst && diff < WILDCOPY_VECLEN as isize {
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
pub(crate) unsafe fn ZSTD_limitCopy(
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

pub(crate) const ZSTD_WORKSPACETOOLARGE_FACTOR: core::ffi::c_int = 3;
pub(crate) const ZSTD_WORKSPACETOOLARGE_MAXDURATION: core::ffi::c_int = 128;
