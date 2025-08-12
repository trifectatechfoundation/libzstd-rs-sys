#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(test)]
#![feature(likely_unlikely)]
#![feature(linkage)]
#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod lib {
    pub mod common {
        pub mod bitstream;
        pub mod debug;
        pub mod entropy_common;
        pub mod error_private;
        pub mod fse_decompress;
        pub mod mem;
        pub mod pool;
        pub mod xxhash;
        pub mod zstd_common;
        pub mod zstd_trace;
    } // mod common
    pub mod compress {
        pub mod fse_compress;
        pub mod hist;
        pub mod huf_compress;
        pub mod zstd_compress;
        pub mod zstd_compress_literals;
        pub mod zstd_compress_sequences;
        pub mod zstd_compress_superblock;
        pub mod zstd_double_fast;
        pub mod zstd_fast;
        pub mod zstd_lazy;
        pub mod zstd_ldm;
        pub mod zstd_opt;
        pub mod zstd_preSplit;
        pub mod zstdmt_compress;
    } // mod compress
    pub mod decompress;
    pub mod dictBuilder {
        pub mod cover;
        pub mod divsufsort;
        pub mod fastcover;
        pub mod zdict;
    } // mod dictBuilder
    pub mod legacy {
        pub mod zstd_v05;
        pub mod zstd_v06;
        pub mod zstd_v07;
    } // mod legacy
    pub mod zstd;
} // mod lib

#[cfg(feature = "semver-prefix")]
macro_rules! prefix {
    ($name:expr) => {
        concat!(
            "LIBZSTD_RS_SYS_v",
            env!("CARGO_PKG_VERSION_MAJOR"),
            "_",
            env!("CARGO_PKG_VERSION_MINOR"),
            "_x_",
            stringify!($name)
        )
    };
}

#[cfg(not(feature = "semver-prefix"))]
macro_rules! prefix {
    ($name:expr) => {
        stringify!($name)
    };
}

pub(crate) use prefix;

pub type size_t = core::ffi::c_ulong;
