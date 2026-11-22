use core::ffi::c_void;
use core::ptr::{self, NonNull};

use crate::lib::zstd::ZSTD_customMem;

#[repr(C)]
pub(crate) struct CustomMem {
    allocate: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void,
    deallocate: unsafe extern "C" fn(*mut c_void, *mut c_void) -> (),
    opaque: *mut c_void,
}

unsafe impl std::alloc::Allocator for CustomMem {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        if layout.align() > 16 {
            return Err(std::alloc::AllocError);
        }

        if layout.size() == 0 {
            return Ok(NonNull::slice_from_raw_parts(layout.dangling_ptr(), 0));
        }

        let raw_ptr = unsafe { (self.allocate)(self.opaque, layout.size()) };

        if !raw_ptr.addr().is_multiple_of(layout.align()) {
            return Err(std::alloc::AllocError);
        }

        let ptr = NonNull::new(raw_ptr.cast()).ok_or(std::alloc::AllocError)?;

        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: std::alloc::Layout) {
        if layout.size() != 0 {
            // SAFETY: `layout` is non-zero in size
            // other conditions must be upheld by the caller
            unsafe { (self.deallocate)(self.opaque, ptr.as_ptr().cast()) };
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct ZstdAllocator(ZSTD_customMem);

enum CurrentAllocator {
    Custom(CustomMem),
    Rust,
}

impl ZstdAllocator {
    pub(crate) fn new(customMem: ZSTD_customMem) -> Self {
        Self(customMem)
    }

    fn current_allocator(&self) -> Option<CurrentAllocator> {
        let Self(customMem) = self;

        match (customMem.customAlloc, customMem.customFree) {
            (Some(allocate), Some(deallocate)) => {
                let custom_mem = CustomMem {
                    allocate,
                    deallocate,
                    opaque: customMem.opaque,
                };

                Some(CurrentAllocator::Custom(custom_mem))
            }
            (None, None) => {
                cfg_select! {
                    feature = "rust-allocator" => Some(CurrentAllocator::Rust),
                    feature = "c-allocator" => {
                        unsafe extern "C" fn custom_malloc(
                            _opaque: *mut c_void,
                            n: usize,
                        ) -> *mut c_void {
                            libc::malloc(n)
                        }

                        unsafe extern "C" fn custom_free(_opaque: *mut c_void, ptr: *mut c_void) {
                            libc::free(ptr)
                        }

                        let custom_mem = CustomMem {
                            allocate: custom_malloc,
                            deallocate: custom_free,
                            opaque: customMem.opaque,
                        };

                        Some(CurrentAllocator::Custom(custom_mem))
                    }
                    _ => None,
                }
            }

            _ => None,
        }
    }
}

unsafe impl std::alloc::Allocator for ZstdAllocator {
    fn allocate(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        match self.current_allocator().ok_or(std::alloc::AllocError)? {
            CurrentAllocator::Custom(custom_mem) => custom_mem.allocate(layout),
            CurrentAllocator::Rust => std::alloc::Global.allocate(layout),
        }
    }

    fn allocate_zeroed(
        &self,
        layout: std::alloc::Layout,
    ) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        match self.current_allocator().ok_or(std::alloc::AllocError)? {
            CurrentAllocator::Custom(custom_mem) => custom_mem.allocate_zeroed(layout),
            CurrentAllocator::Rust => std::alloc::Global.allocate_zeroed(layout),
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: std::alloc::Layout) {
        let Some(current) = self.current_allocator() else {
            panic!("deallocation without a current allocator");
        };

        match current {
            CurrentAllocator::Custom(custom_mem) => custom_mem.deallocate(ptr, layout),
            CurrentAllocator::Rust => std::alloc::Global.deallocate(ptr, layout),
        }
    }
}

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

    cfg_select! {
        feature = "rust-allocator" => {
            let layout = core::alloc::Layout::from_size_align_unchecked(size, 16);
            std::alloc::alloc(layout).cast()
        }
        feature = "c-allocator" => libc::malloc(size),
        _ => {
            panic!("no allocator specified");
        }
    }
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

    cfg_select! {
        feature = "rust-allocator" => {
            let layout = core::alloc::Layout::from_size_align_unchecked(size, 16);
            std::alloc::alloc_zeroed(layout).cast()
        }
        feature = "c-allocator" => libc::calloc(1, size),
        _ => {
            panic!("no allocator specified");
        }
    }
}

#[inline]
pub(crate) unsafe fn ZSTD_customFree(
    ptr: *mut core::ffi::c_void,
    _size: usize,
    customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if let Some(f) = customMem.customFree {
            return f(customMem.opaque, ptr);
        }

        cfg_select! {
            feature = "rust-allocator" => {
                let layout = core::alloc::Layout::from_size_align_unchecked(_size, 16);
                std::alloc::dealloc(ptr.cast(), layout)
            }
            feature = "c-allocator" => {
                libc::free(ptr);
            }
            _ => {
                panic!("no allocator specified");
            }
        }
    }
}
