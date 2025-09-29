use std::ptr;

use libc::{calloc, free, malloc, size_t};

use crate::lib::zstd::ZSTD_customMem;

#[inline]
pub(crate) unsafe fn ZSTD_customMalloc(
    size: size_t,
    customMem: ZSTD_customMem,
) -> *mut core::ffi::c_void {
    if let Some(f) = customMem.customAlloc {
        return f(customMem.opaque, size);
    }

    malloc(size)
}
#[inline]
pub(crate) unsafe fn ZSTD_customCalloc(
    size: size_t,
    customMem: ZSTD_customMem,
) -> *mut core::ffi::c_void {
    if let Some(f) = customMem.customAlloc {
        let ptr = f(customMem.opaque, size);
        ptr::write_bytes(ptr, 0, size);
        return ptr;
    }
    calloc(1, size)
}
#[inline]
pub(crate) unsafe fn ZSTD_customFree(ptr: *mut core::ffi::c_void, customMem: ZSTD_customMem) {
    if !ptr.is_null() {
        if let Some(f) = customMem.customFree {
            f(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}
