#![cfg(test)]
#![allow(deprecated)]
mod compress;
mod decompress;
mod dict_builder;
mod parameters;

#[macro_export]
macro_rules! assert_eq_rs_c {
    ($tt:tt) => {{
        #[cfg(not(miri))]
        #[allow(clippy::macro_metavars_in_unsafe)]
        #[allow(unused_unsafe)]
        let _c = unsafe {
            use zstd_sys::*;

            #[allow(unused_braces)]
            $tt
        };

        #[allow(clippy::macro_metavars_in_unsafe)]
        #[allow(unused_unsafe)]
        let _rs = unsafe {
            use libzstd_rs_sys::*;

            #[allow(unused_braces)]
            $tt
        };

        #[cfg(not(miri))]
        assert_eq!(_rs, _c);

        _rs
    }};
}
