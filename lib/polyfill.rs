use std::{
    alloc::{AllocError, Allocator, Layout},
    mem::MaybeUninit,
    ptr::NonNull,
};

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
        (self.addr() as isize - base.addr() as isize) / size_of::<T>() as isize
    }
}

impl<T> PointerExt for *mut T {
    /// Like `offset_from`, but without the UB.
    fn wrapping_offset_from(self, base: Self) -> isize {
        (self.addr() as isize - base.addr() as isize) / size_of::<T>() as isize
    }
}

cfg_select! {
    feature = "nightly" => {
        pub use core::hint::{likely, unlikely};
    }
    _ => {
        #[inline(always)]
        pub fn likely(b: bool) -> bool {
            if !b {
                core::hint::cold_path();
            }

            b
        }
        #[inline(always)]
        pub fn unlikely(b: bool) -> bool {
            if b {
                core::hint::cold_path();
            }

            b
        }
    }
}

pub(crate) fn box_try_new_zeroed<T>() -> Result<Box<MaybeUninit<T>>, AllocError> {
    box_try_new_zeroed_in(std::alloc::Global)
}

pub(crate) fn box_try_new_zeroed_in<T, A: Allocator>(
    alloc: A,
) -> Result<Box<MaybeUninit<T>, A>, AllocError> {
    let ptr = if size_of::<T>() == 0 {
        NonNull::dangling()
    } else {
        let layout = Layout::new::<MaybeUninit<T>>();
        alloc.allocate_zeroed(layout)?.cast()
    };
    // SAFETY: Pointer is nonnull and matches the allocator.
    unsafe { Ok(Box::from_raw_in(ptr.as_ptr(), alloc)) }
}
