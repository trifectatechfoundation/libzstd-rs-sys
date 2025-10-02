use std::ptr;

use crate::lib::zstd::ZSTD_customMem;

#[inline]
pub(crate) unsafe fn ZSTD_customMalloc(
    size: usize,
    customMem: ZSTD_customMem,
) -> *mut core::ffi::c_void {
    if customMem.customAlloc.is_some() ^ customMem.customFree.is_some() {
        return core::ptr::null_mut();
    }

    if let Some(f) = customMem.customAlloc {
        return f(customMem.opaque, size);
    }

    #[cfg(feature = "rust-allocator")]
    return std::alloc::alloc(core::alloc::Layout::from_size_align_unchecked(size, 16)).cast();

    #[cfg(feature = "c-allocator")]
    return libc::malloc(size);
}
#[inline]
pub(crate) unsafe fn ZSTD_customCalloc(
    size: usize,
    customMem: ZSTD_customMem,
) -> *mut core::ffi::c_void {
    if customMem.customAlloc.is_some() ^ customMem.customFree.is_some() {
        return core::ptr::null_mut();
    }

    if let Some(f) = customMem.customAlloc {
        let ptr = f(customMem.opaque, size);
        ptr::write_bytes(ptr, 0, size);
        return ptr;
    }

    #[cfg(feature = "rust-allocator")]
    return std::alloc::alloc_zeroed(core::alloc::Layout::from_size_align_unchecked(size, 16))
        .cast();

    #[cfg(feature = "c-allocator")]
    return libc::calloc(1, size);
}
#[inline]
pub(crate) unsafe fn ZSTD_customFree(
    ptr: *mut core::ffi::c_void,
    _size: usize,
    customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if let Some(f) = customMem.customFree {
            f(customMem.opaque, ptr);
        } else {
            #[cfg(feature = "rust-allocator")]
            return std::alloc::dealloc(
                ptr.cast(),
                core::alloc::Layout::from_size_align_unchecked(_size, 16),
            );

            #[cfg(feature = "c-allocator")]
            return libc::free(ptr);
        }
    }
}
