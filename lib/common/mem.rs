use core::ffi::c_void;

use libc::size_t;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type BYTE = u8;
// pub(crate) type U8 = u8;
// pub(crate) type S8 = i8;
// pub(crate) type U16 = u16;
// pub(crate) type S16 = i16;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type U32 = u32;
// pub(crate) type S32 = i32;

#[allow(clippy::upper_case_acronyms)]
pub(crate) type U64 = u64;
// pub(crate) type S64 = i64;

#[inline]
pub(crate) const fn MEM_32bits() -> bool {
    size_of::<usize>() == 4
}
#[inline]
pub(crate) const fn MEM_64bits() -> bool {
    size_of::<usize>() == 8
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
pub(crate) unsafe fn MEM_readLE16(memPtr: *const c_void) -> u16 {
    MEM_read16(memPtr).to_le()
}

#[inline]
pub(crate) unsafe fn MEM_writeLE16(memPtr: *mut c_void, val32: u16) {
    MEM_write16(memPtr, val32.to_le());
}

#[inline]
pub unsafe fn MEM_readLE24(memPtr: *const c_void) -> u32 {
    (MEM_readLE16(memPtr) as u32).wrapping_add((*(memPtr as *const u8).add(2) as u32) << 16)
}

#[inline]
pub(crate) unsafe fn MEM_writeLE24(memPtr: *mut c_void, val: u32) {
    MEM_writeLE16(memPtr, val as u16);
    *(memPtr as *mut u8).add(2) = (val >> 16) as u8;
}

#[inline]
pub unsafe fn MEM_readLE32(memPtr: *const c_void) -> u32 {
    MEM_read32(memPtr).to_le()
}

#[inline]
pub(crate) unsafe fn MEM_writeLE32(memPtr: *mut c_void, val32: u32) {
    MEM_write32(memPtr, val32.to_le());
}

#[inline]
pub(crate) unsafe fn MEM_readLE64(memPtr: *const c_void) -> u64 {
    MEM_read64(memPtr).to_le()
}

#[inline]
pub(crate) unsafe fn MEM_writeLE64(memPtr: *mut c_void, val64: u64) {
    MEM_write64(memPtr, val64.to_le());
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
