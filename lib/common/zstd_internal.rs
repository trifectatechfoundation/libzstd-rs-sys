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
pub(crate) static repStartValue: [u32; ZSTD_REP_NUM as usize] = [1, 4, 8];

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

pub(crate) unsafe fn ZSTD_copy16(dst: *mut u8, src: *const u8) {
    // We use `copy` instead of `copy_nonoverlapping` here because the literal buffer can now
    // be located within the dst buffer. In circumstances where the op "catches up" to where the
    // literal buffer is, there can be partial overlaps in this call on the final
    // copy if the literal is being shifted by less than 16 bytes.
    core::ptr::copy(src, dst, 16)
}

pub(crate) const WILDCOPY_OVERLENGTH: usize = 32;
pub(crate) const WILDCOPY_VECLEN: core::ffi::c_int = 16;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Overlap {
    NoOverlap,
    OverlapSrcBeforeDst,
}

/// Custom version of ZSTD_memcpy(), can over read/write up to WILDCOPY_OVERLENGTH bytes (if length==0)
///
/// The `ovtype` controls the overlap detection
/// - ZSTD_no_overlap: The source and destination are guaranteed to be at least WILDCOPY_VECLEN bytes apart.
/// - ZSTD_overlap_src_before_dst: The src and dst may overlap, but they MUST be at least 8 bytes apart.
///   The src buffer must be before the dst buffer.
#[inline(always)]
pub(crate) unsafe fn ZSTD_wildcopy(
    mut op: *mut u8,
    mut ip: *const u8,
    length: size_t,
    ovtype: Overlap,
) {
    let diff = op as isize - ip as isize;
    let oend = op.add(length);
    if ovtype == Overlap::OverlapSrcBeforeDst && diff < WILDCOPY_VECLEN as isize {
        debug_assert!(ip <= op.wrapping_sub(8).cast_const());

        // Handle short offset copies.
        loop {
            // SAFETY: ip and op are guaranteed to be at least 8 bytes apart.
            core::ptr::copy_nonoverlapping(ip, op, 8);

            op = op.add(8);
            ip = ip.add(8);

            if op >= oend {
                break;
            }
        }
    } else {
        debug_assert!(diff.abs() >= WILDCOPY_VECLEN as isize);

        // NOTE: ip and op are at least 8 apart, but may be less than 16 apart, so we cannot use
        // `copy_nonoverlapping` when copying more than 8 bytes.

        // Separate out the first 16-byte copy call because the copy length is
        // almost certain to be short, so the branches have different
        // probabilities. Since it is almost certain to be short, only do
        // one 16-byte copy in the first call. Then, do two calls per loop since
        // at that point it is more likely to have a high trip count.
        core::ptr::copy(ip, op, 16);

        if 16 >= length {
            return;
        }

        op = op.add(16);
        ip = ip.add(16);

        loop {
            core::ptr::copy(ip, op, 16);
            op = op.add(16);
            ip = ip.add(16);

            core::ptr::copy(ip, op, 16);
            op = op.add(16);
            ip = ip.add(16);

            if op >= oend {
                break;
            }
        }
    }
}

#[inline]
pub(crate) unsafe fn ZSTD_limitCopy(
    dst: *mut u8,
    dstCapacity: size_t,
    src: *const u8,
    srcSize: size_t,
) -> size_t {
    let length = Ord::min(dstCapacity, srcSize);
    core::ptr::copy_nonoverlapping(src, dst, length);
    length
}

pub(crate) const ZSTD_WORKSPACETOOLARGE_FACTOR: core::ffi::c_int = 3;
pub(crate) const ZSTD_WORKSPACETOOLARGE_MAXDURATION: core::ffi::c_int = 128;

#[inline]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) fn ZSTD_cpuSupportsBmi2() -> bool {
    is_x86_feature_detected!("bmi1") && is_x86_feature_detected!("bmi2")
}

#[inline]
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub(crate) fn ZSTD_cpuSupportsBmi2() -> bool {
    false
}
