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
