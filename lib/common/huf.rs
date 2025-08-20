use libc::size_t;

use crate::lib::zstd::ZSTD_btultra;

pub(crate) const HUF_BLOCKSIZE_MAX: core::ffi::c_int = 128 * 1024;

pub(crate) const HUF_WORKSPACE_SIZE: core::ffi::c_int = ((8) << 10) + 512;

/// Max runtime value of tableLog (due to static allocation); can be modified up to HUF_TABLELOG_ABSOLUTEMAX.
pub(crate) const HUF_TABLELOG_MAX: usize = 12;
/// Default tableLog value when none specified
pub(crate) const HUF_TABLELOG_DEFAULT: core::ffi::c_int = 11;
pub(crate) const HUF_SYMBOLVALUE_MAX: core::ffi::c_int = 255;

/// Absolute limit of HUF_MAX_TABLELOG. Beyond that value, code does not work
const HUF_TABLELOG_ABSOLUTEMAX: usize = 12;
const _: () = assert!(
    HUF_TABLELOG_MAX <= HUF_TABLELOG_ABSOLUTEMAX,
    "HUF_TABLELOG_MAX is too large !"
);

pub(crate) const HUF_CTABLEBOUND: core::ffi::c_int = 129;

pub(crate) type HUF_CElt = size_t;

pub(crate) const HUF_flags_bmi2: core::ffi::c_uint = 1;
pub(crate) const HUF_flags_optimalDepth: core::ffi::c_uint = 2;
pub(crate) const HUF_flags_preferRepeat: core::ffi::c_uint = 4;
pub(crate) const HUF_flags_suspectUncompressible: core::ffi::c_uint = 8;
pub(crate) const HUF_flags_disableAsm: core::ffi::c_uint = 16;
pub(crate) const HUF_flags_disableFast: core::ffi::c_uint = 32;

pub(crate) const HUF_OPTIMAL_DEPTH_THRESHOLD: core::ffi::c_int = ZSTD_btultra as core::ffi::c_int;

pub(crate) type HUF_repeat = core::ffi::c_uint;
/// Cannot use the previous table
pub(crate) const HUF_repeat_none: HUF_repeat = 0;
/// Can use the previous table but it must be checked. Note : The previous table must have been constructed by HUF_compress{1, 4}X_repeat
pub(crate) const HUF_repeat_check: HUF_repeat = 1;
/// Can use the previous table and it is assumed to be valid
pub(crate) const HUF_repeat_valid: HUF_repeat = 2;

// FIXME move the FSE_* functions to fse.rs
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
        + FSE_MAX_SYMBOL_VALUE.div_ceil(2)
        + 1
}

pub(crate) const HUF_READ_STATS_WORKSPACE_SIZE_U32: usize =
    FSE_DECOMPRESS_WKSP_SIZE_U32(6, HUF_TABLELOG_MAX - 1);

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct HUF_CTableHeader {
    pub(crate) tableLog: u8,
    pub(crate) maxSymbolValue: u8,
    pub(crate) unused: [u8; core::mem::size_of::<usize>() - 2],
}
