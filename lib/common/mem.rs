use core::ffi::{c_uint, c_void};
use core::mem;

pub type size_t = core::ffi::c_ulong;

#[inline]
pub(crate) fn MEM_32bits() -> c_uint {
    (::core::mem::size_of::<usize>() == 4) as _
}
#[inline]
pub(crate) fn MEM_64bits() -> c_uint {
    (::core::mem::size_of::<usize>() == 8) as _
}

#[inline]
pub(crate) fn MEM_isLittleEndian() -> c_uint {
    cfg!(target_endian = "little") as _
}

#[inline]
pub(crate) unsafe fn MEM_read16(mut ptr: *const c_void) -> u16 {
    ptr.cast::<u16>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_read32(mut ptr: *const c_void) -> u32 {
    ptr.cast::<u32>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_read64(mut ptr: *const c_void) -> u64 {
    ptr.cast::<u64>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_readST(mut ptr: *const c_void) -> size_t {
    ptr.cast::<size_t>().read_unaligned()
}

#[inline]
pub(crate) unsafe fn MEM_write16(mut memPtr: *mut c_void, mut value: u16) {
    memPtr.cast::<u16>().write_unaligned(value)
}
#[inline]
pub(crate) unsafe fn MEM_write32(mut memPtr: *mut c_void, mut value: u32) {
    memPtr.cast::<u32>().write_unaligned(value)
}
#[inline]
pub(crate) unsafe fn MEM_write64(mut memPtr: *mut c_void, mut value: u64) {
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
pub(crate) unsafe fn MEM_readLE16(mut memPtr: *const c_void) -> u16 {
    if cfg!(target_endian = "little") {
        MEM_read16(memPtr)
    } else {
        let mut p = memPtr as *const u8;
        (*p.offset(0) as core::ffi::c_int + ((*p.offset(1) as core::ffi::c_int) << 8)) as u16
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLE16(mut memPtr: *mut c_void, mut val32: u16) {
    if cfg!(target_endian = "little") {
        MEM_write16(memPtr, val32);
    } else {
        MEM_write16(memPtr, MEM_swap16(val32));
    };
}

#[inline]
pub unsafe fn MEM_readLE24(mut memPtr: *const c_void) -> u32 {
    (MEM_readLE16(memPtr) as u32).wrapping_add((*(memPtr as *const u8).offset(2) as u32) << 16)
}

#[inline]
pub(crate) unsafe fn MEM_writeLE24(mut memPtr: *mut c_void, mut val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8).offset(2) = (val >> 16) as u8;
}

#[inline]
pub unsafe fn MEM_readLE32(mut memPtr: *const c_void) -> u32 {
    if cfg!(target_endian = "little") {
        MEM_read32(memPtr)
    } else {
        MEM_swap32(MEM_read32(memPtr))
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLE32(mut memPtr: *mut c_void, mut val32: u32) {
    if cfg!(target_endian = "little") {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}

#[inline]
pub(crate) unsafe fn MEM_readLE64(mut memPtr: *const c_void) -> u64 {
    if cfg!(target_endian = "little") {
        MEM_read64(memPtr)
    } else {
        MEM_swap64(MEM_read64(memPtr))
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLE64(mut memPtr: *mut c_void, mut val64: u64) {
    if cfg!(target_endian = "little") {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}

#[inline]
pub(crate) unsafe fn MEM_readLEST(mut memPtr: *const c_void) -> size_t {
    match mem::size_of::<size_t>() {
        4 => MEM_readLE32(memPtr) as size_t,
        8 => MEM_readLE64(memPtr) as size_t,
        _ => unreachable!(),
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLEST(mut memPtr: *mut c_void, mut val: size_t) {
    match mem::size_of::<size_t>() {
        4 => MEM_writeLE32(memPtr, val as u32),
        8 => MEM_writeLE64(memPtr, val as u64),
        _ => unreachable!(),
    }
}

const _: () = {
    assert!(mem::size_of::<size_t>() == 4 || mem::size_of::<size_t>() == 8);
};
