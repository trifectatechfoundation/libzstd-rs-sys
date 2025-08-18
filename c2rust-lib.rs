#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(test)]
#![feature(likely_unlikely)]
#![feature(linkage)]
#[macro_use]
extern crate c2rust_bitfields;
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

pub use crate::lib::zstd::{ZSTD_inBuffer, ZSTD_outBuffer};

pub use crate::lib::decompress::{
    zstd_ddict::{ZSTD_DDict, ZSTD_createDDict, ZSTD_getDictID_fromDDict},
    zstd_decompress::{
        ZSTD_DCtx_refDDict, ZSTD_createDCtx, ZSTD_decompressBegin_usingDDict,
        ZSTD_decompressBegin_usingDict, ZSTD_decompressStream, ZSTD_decompress_usingDict,
        ZSTD_freeDCtx, ZSTD_getDictID_fromDict, ZSTD_getFrameContentSize, ZSTD_CONTENTSIZE_ERROR,
        ZSTD_CONTENTSIZE_UNKNOWN,
    },
};

pub use crate::lib::common::zstd_common::{ZSTD_getErrorName, ZSTD_isError};

pub use crate::lib::dictBuilder::zdict::{
    ZDICT_getErrorName, ZDICT_isError, ZDICT_trainFromBuffer,
};

pub use crate::lib::compress::zstd_compress::{
    ZSTD_compressBound, ZSTD_compress_usingDict, ZSTD_createCCtx, ZSTD_freeCCtx,
};

#[cfg(all(feature = "export-symbols", feature = "semver-prefix"))]
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

#[cfg(all(feature = "export-symbols", not(feature = "semver-prefix")))]
macro_rules! prefix {
    ($name:expr) => {
        stringify!($name)
    };
}

use libc::size_t;

#[cfg(feature = "export-symbols")]
pub(crate) use prefix;
