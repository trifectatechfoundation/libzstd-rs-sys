macro_rules! cfg_select {
    ({ $($tt:tt)* }) => {{
        $crate::cfg_select! { $($tt)* }
    }};
    (_ => { $($output:tt)* }) => {
        $($output)*
    };
    (
        $cfg:meta => $output:tt
        $($( $rest:tt )+)?
    ) => {
        #[cfg($cfg)]
        $crate::lib::polyfill::cfg_select! { _ => $output }
        $(
            #[cfg(not($cfg))]
            $crate::lib::polyfill::cfg_select! { $($rest)+ }
        )?
    }
}
pub(crate) use cfg_select;

/// Keep the branches on either side of this point in source order.
///
/// Once inlined, `a && b` on two cheap, side-effect-free tests becomes two branches, and LLVM
/// may evaluate `b` first. That matters when `a` is the predictable test and `b` the
/// unpredictable one: e.g. in the match finders, the data comparison almost never succeeds,
/// while the match-index validity check is close to random.
macro_rules! branch_barrier {
    () => {
        #[cfg(not(any(target_family = "wasm", miri)))]
        // SAFETY: an empty asm block has no effect.
        unsafe {
            core::arch::asm!("", options(preserves_flags));
        }
    };
}
pub(crate) use branch_barrier;

pub trait PointerExt {
    fn wrapping_offset_from(self, other: Self) -> isize;
}

impl<T> PointerExt for *const T {
    fn wrapping_offset_from(self, base: Self) -> isize {
        ((self as isize) - (base as isize)) / size_of::<T>() as isize
    }
}

impl<T> PointerExt for *mut T {
    /// Like `offset_from`, but without the UB.
    fn wrapping_offset_from(self, base: Self) -> isize {
        ((self as isize) - (base as isize)) / size_of::<T>() as isize
    }
}

cfg_select! {
    feature = "nightly" => {
        pub use core::hint::{cold_path, likely, unlikely};
    }
    _ => {
        #[inline(always)]
        pub fn likely(b: bool) -> bool {
            if !b {
                cold_path()
            }

            b
        }
        #[inline(always)]
        pub fn unlikely(b: bool) -> bool {
            if b {
                cold_path();
            }

            b
        }

        #[cold]
        pub fn cold_path() {}
    }
}

/// The C library items this crate uses.
///
/// `libc` provides nothing on `wasm32-unknown-unknown`, so that target gets Rust implementations.
#[allow(non_camel_case_types)]
pub(crate) mod libc {
    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    pub(crate) use ::libc::{calloc, free, malloc, memcmp, memcpy, ptrdiff_t, size_t};

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    pub(crate) use self::wasm::*;

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    mod wasm {
        use core::alloc::Layout;
        use core::ffi::{c_int, c_void};

        pub(crate) type size_t = usize;
        pub(crate) type ptrdiff_t = isize;

        // `free` receives no size, so each block starts with a header that records its layout.
        const HEADER: usize = 16;

        unsafe fn alloc_block(size: size_t, zeroed: bool) -> *mut c_void {
            let Some(total) = size.checked_add(HEADER) else {
                return core::ptr::null_mut();
            };
            let Ok(layout) = Layout::from_size_align(total, HEADER) else {
                return core::ptr::null_mut();
            };
            let base = if zeroed {
                std::alloc::alloc_zeroed(layout)
            } else {
                std::alloc::alloc(layout)
            };
            if base.is_null() {
                return core::ptr::null_mut();
            }
            base.cast::<usize>().write(total);
            base.add(HEADER).cast()
        }

        pub(crate) unsafe fn malloc(size: size_t) -> *mut c_void {
            alloc_block(size, false)
        }

        pub(crate) unsafe fn calloc(nobj: size_t, size: size_t) -> *mut c_void {
            match nobj.checked_mul(size) {
                Some(total) => alloc_block(total, true),
                None => core::ptr::null_mut(),
            }
        }

        pub(crate) unsafe fn free(ptr: *mut c_void) {
            if ptr.is_null() {
                return;
            }
            let base = ptr.cast::<u8>().sub(HEADER);
            let total = base.cast::<usize>().read();
            std::alloc::dealloc(base, Layout::from_size_align_unchecked(total, HEADER));
        }

        pub(crate) unsafe fn memcpy(
            dest: *mut c_void,
            src: *const c_void,
            n: size_t,
        ) -> *mut c_void {
            core::ptr::copy_nonoverlapping(src.cast::<u8>(), dest.cast::<u8>(), n);
            dest
        }

        pub(crate) unsafe fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int {
            let a = core::slice::from_raw_parts(cx.cast::<u8>(), n);
            let b = core::slice::from_raw_parts(ct.cast::<u8>(), n);
            a.cmp(b) as c_int
        }
    }
}
