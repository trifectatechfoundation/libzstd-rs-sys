use core::ffi::c_void;

use libc::size_t;

#[inline]
pub(crate) const fn MEM_32bits() -> bool {
    size_of::<usize>() == 4
}
#[inline]
pub(crate) const fn MEM_64bits() -> bool {
    size_of::<usize>() == 8
}

#[inline]
pub(crate) const fn MEM_isLittleEndian() -> bool {
    cfg!(target_endian = "little")
}

#[inline]
pub(crate) unsafe fn MEM_read16(ptr: *const c_void) -> u16 {
    ptr.cast::<u16>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_read32(ptr: *const c_void) -> u32 {
    ptr.cast::<u32>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_read64(ptr: *const c_void) -> u64 {
    ptr.cast::<u64>().read_unaligned()
}
#[inline]
pub(crate) unsafe fn MEM_readST(ptr: *const c_void) -> size_t {
    ptr.cast::<usize>().read_unaligned()
}

#[inline]
pub(crate) unsafe fn MEM_write16(memPtr: *mut c_void, value: u16) {
    memPtr.cast::<u16>().write_unaligned(value)
}
#[inline]
pub(crate) unsafe fn MEM_write32(memPtr: *mut c_void, value: u32) {
    memPtr.cast::<u32>().write_unaligned(value)
}
#[inline]
pub(crate) unsafe fn MEM_write64(memPtr: *mut c_void, value: u64) {
    memPtr.cast::<u64>().write_unaligned(value)
}

#[inline]
pub(crate) fn MEM_swap16(in_0: u16) -> u16 {
    in_0.swap_bytes()
}
#[inline]
pub(crate) fn MEM_swap32(in_0: u32) -> u32 {
    in_0.swap_bytes()
}
#[inline]
pub(crate) fn MEM_swap64(in_0: u64) -> u64 {
    in_0.swap_bytes()
}

#[inline]
pub(crate) unsafe fn MEM_readLE16(memPtr: *const c_void) -> u16 {
    if cfg!(target_endian = "little") {
        MEM_read16(memPtr)
    } else {
        let p = memPtr as *const u8;
        (core::ffi::c_int::from(*p) + (core::ffi::c_int::from(*p.add(1)) << 8)) as u16
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLE16(memPtr: *mut c_void, val32: u16) {
    if cfg!(target_endian = "little") {
        MEM_write16(memPtr, val32);
    } else {
        MEM_write16(memPtr, MEM_swap16(val32));
    };
}

#[inline]
pub unsafe fn MEM_readLE24(memPtr: *const c_void) -> u32 {
    u32::from(MEM_readLE16(memPtr)).wrapping_add(u32::from(*(memPtr as *const u8).add(2)) << 16)
}

#[inline]
pub(crate) unsafe fn MEM_writeLE24(memPtr: *mut c_void, val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8).add(2) = (val >> 16) as u8;
}

#[inline]
pub unsafe fn MEM_readLE32(memPtr: *const c_void) -> u32 {
    if cfg!(target_endian = "little") {
        MEM_read32(memPtr)
    } else {
        MEM_swap32(MEM_read32(memPtr))
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLE32(memPtr: *mut c_void, val32: u32) {
    if cfg!(target_endian = "little") {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}

#[inline]
pub(crate) unsafe fn MEM_readLE64(memPtr: *const c_void) -> u64 {
    if cfg!(target_endian = "little") {
        MEM_read64(memPtr)
    } else {
        MEM_swap64(MEM_read64(memPtr))
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLE64(memPtr: *mut c_void, val64: u64) {
    if cfg!(target_endian = "little") {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}

#[inline]
pub(crate) unsafe fn MEM_readLEST(memPtr: *const c_void) -> size_t {
    match size_of::<size_t>() {
        4 => MEM_readLE32(memPtr) as size_t,
        8 => MEM_readLE64(memPtr) as size_t,
        _ => unreachable!(),
    }
}

#[inline]
pub(crate) unsafe fn MEM_writeLEST(memPtr: *mut c_void, val: size_t) {
    match size_of::<size_t>() {
        4 => MEM_writeLE32(memPtr, val as u32),
        8 => MEM_writeLE64(memPtr, val as u64),
        _ => unreachable!(),
    }
}

const _: () = {
    assert!(size_of::<size_t>() == 4 || size_of::<size_t>() == 8);
};
