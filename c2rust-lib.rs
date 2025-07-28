#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(test)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(likely_unlikely)]
#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod lib {
    pub mod common {
        pub mod debug;
        pub mod entropy_common;
        pub mod entropy_common_old;
        pub mod error_private;
        pub mod fse_decompress;
        pub mod fse_decompress_old;
        pub mod pool;
        pub mod threading;
        pub mod xxhash;
        pub mod zstd_common;
    } // mod common
    pub mod compress {
        pub mod fse_compress;
        pub mod hist;
        pub mod huf_compress;
        pub mod zstd_compress;
        pub mod zstd_compress_literals;
        pub mod zstd_compress_sequences;
        pub mod zstd_compress_superblock;
        pub mod zstd_double_fast;
        pub mod zstd_fast;
        pub mod zstd_lazy;
        pub mod zstd_ldm;
        pub mod zstd_opt;
        pub mod zstd_preSplit;
        pub mod zstdmt_compress;
    } // mod compress
    pub mod decompress;
    pub mod dictBuilder {
        pub mod cover;
        pub mod divsufsort;
        pub mod fastcover;
        pub mod zdict;
    } // mod dictBuilder
    pub mod legacy {
        pub mod zstd_v05;
        pub mod zstd_v06;
        pub mod zstd_v07;
    } // mod legacy
    pub mod zstd;
} // mod lib
pub mod programs {
    pub mod benchfn;
    pub mod benchzstd;
    pub mod datagen;
    pub mod dibio;
    pub mod fileio;
    pub mod fileio_asyncio;
    pub mod lorem;
    pub mod timefn;
    pub mod util;
    pub mod zstdcli;
    pub mod zstdcli_trace;
} // mod programs

#[inline]
pub(crate) fn MEM_isLittleEndian() -> std::ffi::c_uint {
    cfg!(target_endian = "little") as _
}
#[inline]
pub(crate) unsafe fn MEM_read16(mut ptr: *const std::ffi::c_void) -> u16 {
    ptr.cast::<u16>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_read32(mut ptr: *const std::ffi::c_void) -> u32 {
    ptr.cast::<u32>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_read64(mut ptr: *const std::ffi::c_void) -> u64 {
    ptr.cast::<u64>().read_unaligned()
}
pub type size_t = std::ffi::c_ulong;
#[inline]
pub(crate) unsafe extern "C" fn MEM_readST(mut ptr: *const std::ffi::c_void) -> size_t {
    ptr.cast::<size_t>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_write16(mut memPtr: *mut std::ffi::c_void, mut value: u16) {
    memPtr.cast::<u16>().write_unaligned(value)
}
#[inline]
pub(crate) unsafe fn MEM_write32(mut memPtr: *mut std::ffi::c_void, mut value: u32) {
    memPtr.cast::<u32>().write_unaligned(value)
}
#[inline]
pub(crate) unsafe fn MEM_write64(mut memPtr: *mut std::ffi::c_void, mut value: u64) {
    memPtr.cast::<u64>().write_unaligned(value)
}
#[inline]
pub(crate) fn MEM_swap16(mut in_0: u16) -> u16 {
    in_0.swap_bytes()
}
#[inline]
pub(crate) fn MEM_swap32(mut in_0: u32) -> u32 {
    in_0.swap_bytes()
}
#[inline]
pub(crate) fn MEM_swap64(mut in_0: u64) -> u64 {
    in_0.swap_bytes()
}

#[inline]
pub(crate) unsafe fn MEM_readLE16(mut memPtr: *const std::ffi::c_void) -> u16 {
    if MEM_isLittleEndian() != 0 {
        MEM_read16(memPtr)
    } else {
        let mut p = memPtr as *const u8;
        (*p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
            + ((*p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                << 8 as std::ffi::c_int)) as u16
    }
}
#[inline]
pub(crate) unsafe fn MEM_readLE24(mut memPtr: *const std::ffi::c_void) -> u32 {
    (MEM_readLE16(memPtr) as u32).wrapping_add(
        (*(memPtr as *const u8).offset(2 as std::ffi::c_int as isize) as u32)
            << 16 as std::ffi::c_int,
    )
}
#[inline]
pub(crate) unsafe fn MEM_readLE32(mut memPtr: *const std::ffi::c_void) -> u32 {
    if MEM_isLittleEndian() != 0 {
        MEM_read32(memPtr)
    } else {
        MEM_swap32(MEM_read32(memPtr))
    }
}
#[inline]
pub(crate) unsafe fn MEM_writeLE16(mut memPtr: *mut std::ffi::c_void, mut val32: u16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val32);
    } else {
        MEM_write16(memPtr, MEM_swap16(val32));
    };
}
#[inline]
pub(crate) unsafe fn MEM_writeLE24(mut memPtr: *mut std::ffi::c_void, mut val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8).offset(2 as std::ffi::c_int as isize) =
        (val >> 16 as std::ffi::c_int) as u8;
}
#[inline]
pub(crate) unsafe fn MEM_writeLE32(mut memPtr: *mut std::ffi::c_void, mut val32: u32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
#[inline]
pub(crate) unsafe fn MEM_writeLE64(mut memPtr: *mut std::ffi::c_void, mut val64: u64) {
    if MEM_isLittleEndian() != 0 {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
#[inline]
pub(crate) unsafe fn MEM_readLE64(mut memPtr: *const std::ffi::c_void) -> u64 {
    if MEM_isLittleEndian() != 0 {
        MEM_read64(memPtr)
    } else {
        MEM_swap64(MEM_read64(memPtr))
    }
}

#[inline]
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const std::ffi::c_void) -> size_t {
    match core::mem::size_of::<size_t>() {
        4 => MEM_readLE32(memPtr) as size_t,
        8 => MEM_readLE64(memPtr) as size_t,
        _ => unreachable!(),
    }
}
