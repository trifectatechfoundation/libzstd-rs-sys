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

#[inline]
pub(crate) fn box_try_new_zeroed<T>() -> Result<Box<MaybeUninit<T>>, AllocError> {
    box_try_new_zeroed_in(std::alloc::Global)
}

#[inline]
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

#[inline]
pub(crate) fn box_try_new_zeroed_slice<T>(len: usize) -> Result<Box<[MaybeUninit<T>]>, AllocError> {
    box_try_new_zeroed_slice_in(len, std::alloc::Global)
}

#[inline]
pub(crate) fn box_try_new_zeroed_slice_in<T, A: Allocator>(
    len: usize,
    alloc: A,
) -> Result<Box<[MaybeUninit<T>], A>, AllocError> {
    let ptr = if size_of::<T>() == 0 || len == 0 {
        NonNull::dangling()
    } else {
        let layout = match Layout::array::<MaybeUninit<T>>(len) {
            Ok(l) => l,
            Err(_) => return Err(AllocError),
        };
        alloc.allocate_zeroed(layout)?.cast()
    };
    // SAFETY: `ptr` was just allocated with `alloc` with the layout for an array of length
    // `len`, and the layout creation would have failed if `len` overflowed an isize.
    // `into_box` is sound to call since `len` corresponds to the length of the just-created
    // `RawVec`.
    unsafe { Ok(Vec::from_raw_parts_in(ptr.as_ptr(), len, len, alloc).into_boxed_slice()) }
}
