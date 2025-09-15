use crate::internal::MEM_readLE32;
use crate::lib::common::mem::MEM_readLE64;

const prime3bytes: u32 = 506832829;
const fn ZSTD_hash3(u: u32, h: u32, s: u32) -> u32 {
    (((u << (32 as core::ffi::c_int - 24 as core::ffi::c_int)) * prime3bytes) ^ s)
        >> 32u32.wrapping_sub(h)
}
#[inline]
pub(crate) unsafe fn ZSTD_hash3Ptr(ptr: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash3(MEM_readLE32(ptr), h, 0) as usize
}

const prime4bytes: u32 = 2654435761;
const fn ZSTD_hash4(u: u32, h: u32, s: u32) -> u32 {
    ((u.wrapping_mul(prime4bytes)) ^ s) >> 32u32.wrapping_sub(h)
}
unsafe fn ZSTD_hash4Ptr(ptr: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0) as usize
}
unsafe fn ZSTD_hash4PtrS(ptr: *const core::ffi::c_void, h: u32, s: u32) -> usize {
    ZSTD_hash4(MEM_readLE32(ptr), h, s) as usize
}

const prime5bytes: u64 = 889523592379;
const fn ZSTD_hash5(u: u64, h: u32, s: u64) -> usize {
    ((((u << (64 - 40)).wrapping_mul(prime5bytes)) ^ s) >> 64u32.wrapping_sub(h)) as usize
}
unsafe fn ZSTD_hash5Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash5(MEM_readLE64(p), h, 0)
}
unsafe fn ZSTD_hash5PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash5(MEM_readLE64(p), h, s)
}

const prime6bytes: u64 = 227718039650203;
const fn ZSTD_hash6(u: u64, h: u32, s: u64) -> usize {
    ((((u << (64 - 48)).wrapping_mul(prime6bytes)) ^ s) >> 64u32.wrapping_sub(h)) as usize
}
pub(crate) unsafe fn ZSTD_hash6Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash6(MEM_readLE64(p), h, 0)
}
unsafe fn ZSTD_hash6PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash6(MEM_readLE64(p), h, s)
}

const prime7bytes: u64 = 58295818150454627;
const fn ZSTD_hash7(u: u64, h: u32, s: u64) -> usize {
    ((((u << (64 - 56)).wrapping_mul(prime7bytes)) ^ s) >> (64u32).wrapping_sub(h)) as usize
}
unsafe fn ZSTD_hash7Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash7(MEM_readLE64(p), h, 0)
}
unsafe fn ZSTD_hash7PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash7(MEM_readLE64(p), h, s)
}

const prime8bytes: u64 = 0xcf1bbcdcb7a56463 as core::ffi::c_ulonglong;
const fn ZSTD_hash8(u: u64, h: u32, s: u64) -> usize {
    (((u.wrapping_mul(prime8bytes)) ^ s) >> 64u32.wrapping_sub(h)) as usize
}
pub(crate) unsafe fn ZSTD_hash8Ptr(p: *const core::ffi::c_void, h: u32) -> usize {
    ZSTD_hash8(MEM_readLE64(p), h, 0)
}
unsafe fn ZSTD_hash8PtrS(p: *const core::ffi::c_void, h: u32, s: u64) -> usize {
    ZSTD_hash8(MEM_readLE64(p), h, s)
}

#[inline(always)]
pub(crate) unsafe fn ZSTD_hashPtr(p: *const core::ffi::c_void, hBits: u32, mls: u32) -> usize {
    match mls {
        5 => ZSTD_hash5Ptr(p, hBits),
        6 => ZSTD_hash6Ptr(p, hBits),
        7 => ZSTD_hash7Ptr(p, hBits),
        8 => ZSTD_hash8Ptr(p, hBits),
        _ => ZSTD_hash4Ptr(p, hBits),
    }
}

#[inline(always)]
pub(crate) unsafe fn ZSTD_hashPtrSalted(
    p: *const core::ffi::c_void,
    hBits: u32,
    mls: u32,
    hashSalt: u64,
) -> usize {
    match mls {
        5 => ZSTD_hash5PtrS(p, hBits, hashSalt),
        6 => ZSTD_hash6PtrS(p, hBits, hashSalt),
        7 => ZSTD_hash7PtrS(p, hBits, hashSalt),
        8 => ZSTD_hash8PtrS(p, hBits, hashSalt),
        4 | _ => ZSTD_hash4PtrS(p, hBits, hashSalt as u32),
    }
}
