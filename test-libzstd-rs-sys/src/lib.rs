#[cfg(test)]
mod decompress;

#[macro_export]
macro_rules! assert_eq_rs_c {
    ($tt:tt) => {{
        #[cfg(not(miri))]
        #[allow(clippy::macro_metavars_in_unsafe)]
        let _c = unsafe {
            use zstd_sys::*;

            #[allow(unused_braces)]
            $tt
        };

        #[allow(clippy::macro_metavars_in_unsafe)]
        let _rs = unsafe {
            use libzstd_rs::*;

            #[allow(unused_braces)]
            $tt
        };

        #[cfg(not(miri))]
        assert_eq!(_rs, _c);

        _c
    }};
}
