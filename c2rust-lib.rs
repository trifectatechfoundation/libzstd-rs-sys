#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(clippy::too_many_arguments)]
#![cfg_attr(test, feature(test))]
#![feature(likely_unlikely)]
// FIXME
#![allow(clippy::missing_safety_doc)]

pub mod lib {
    pub mod common {
        pub(crate) mod bitstream;
        pub mod debug;
        pub(crate) mod entropy_common;
        pub(crate) mod error_private;
        pub(crate) mod fse;
        pub(crate) mod fse_decompress;
        pub(crate) mod huf;
        pub(crate) mod mem;
        pub(crate) mod pool;
        pub(crate) mod xxhash;
        pub mod zstd_common;
        pub(crate) mod zstd_internal;
        pub mod zstd_trace;
    } // mod common
    pub mod compress {
        // FIXME
        #![allow(clippy::wildcard_in_or_patterns)]
        #![allow(clippy::if_same_then_else)]
        #![allow(clippy::collapsible_if)]
        #![allow(clippy::eq_op)]
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
        mod zstdmt_compress;
    } // mod compress
    pub mod decompress;
    pub(crate) mod polyfill;
    pub(crate) mod dictBuilder {
        #![allow(clippy::collapsible_if)]
        pub(crate) mod cover;
        pub(crate) mod divsufsort;
        pub(crate) mod fastcover;
        pub(crate) mod zdict;
    } // mod dictBuilder
    pub(crate) mod legacy {
        pub(crate) mod zstd_v05;
        pub(crate) mod zstd_v06;
        pub(crate) mod zstd_v07;
    } // mod legacy
    pub mod zdict;
    pub mod zstd;
} // mod lib

pub use crate::lib::zstd::{
    ZSTD_cParameter, ZSTD_dParameter, ZSTD_inBuffer, ZSTD_outBuffer, ZSTD_strategy,
    ZSTD_BLOCKSIZELOG_MAX, ZSTD_BLOCKSIZE_MAX, ZSTD_CLEVEL_DEFAULT, ZSTD_MAGICNUMBER,
    ZSTD_MAGIC_DICTIONARY, ZSTD_MAGIC_SKIPPABLE_MASK, ZSTD_MAGIC_SKIPPABLE_START,
    ZSTD_VERSION_MAJOR, ZSTD_VERSION_MINOR, ZSTD_VERSION_NUMBER, ZSTD_VERSION_RELEASE,
};

pub use crate::lib::decompress::{
    zstd_ddict::{
        ZSTD_DDict, ZSTD_createDDict, ZSTD_freeDDict, ZSTD_getDictID_fromDDict, ZSTD_sizeof_DDict,
    },
    zstd_decompress::{
        ZSTD_DCtx_loadDictionary, ZSTD_DCtx_refDDict, ZSTD_DCtx_refPrefix, ZSTD_DCtx_reset,
        ZSTD_DCtx_setParameter, ZSTD_DStream, ZSTD_DStreamInSize, ZSTD_DStreamOutSize,
        ZSTD_createDCtx, ZSTD_createDStream, ZSTD_decompress, ZSTD_decompressBegin,
        ZSTD_decompressBegin_usingDDict, ZSTD_decompressBegin_usingDict, ZSTD_decompressContinue,
        ZSTD_decompressDCtx, ZSTD_decompressStream, ZSTD_decompress_usingDDict,
        ZSTD_decompress_usingDict, ZSTD_findDecompressedSize, ZSTD_findFrameCompressedSize,
        ZSTD_freeDCtx, ZSTD_getDecompressedSize, ZSTD_getDictID_fromDict, ZSTD_getDictID_fromFrame,
        ZSTD_getFrameContentSize, ZSTD_initDStream, ZSTD_initDStream_usingDict,
        ZSTD_nextSrcSizeToDecompress, ZSTD_sizeof_DCtx, ZSTD_CONTENTSIZE_ERROR,
        ZSTD_CONTENTSIZE_UNKNOWN,
    },
    ZSTD_DCtx,
};

pub use crate::lib::common::zstd_common::{
    ZSTD_getErrorName, ZSTD_isError, ZSTD_versionNumber, ZSTD_versionString,
};

pub use crate::lib::dictBuilder::cover::ZDICT_trainFromBuffer_cover;
pub use crate::lib::zdict::{
    experimental::ZDICT_cover_params_t, ZDICT_getDictID, ZDICT_getErrorName, ZDICT_isError,
    ZDICT_params_t, ZDICT_trainFromBuffer,
};

pub use crate::lib::compress::zstd_compress::{
    ZSTD_CCtx, ZSTD_CCtx_loadDictionary, ZSTD_CCtx_refCDict, ZSTD_CCtx_refPrefix, ZSTD_CCtx_reset,
    ZSTD_CCtx_setParameter, ZSTD_CCtx_setPledgedSrcSize, ZSTD_CDict, ZSTD_CStreamInSize,
    ZSTD_CStreamOutSize, ZSTD_EndDirective, ZSTD_compress, ZSTD_compress2, ZSTD_compressBound,
    ZSTD_compressCCtx, ZSTD_compressStream, ZSTD_compressStream2, ZSTD_compress_usingCDict,
    ZSTD_compress_usingDict, ZSTD_createCCtx, ZSTD_createCDict, ZSTD_endStream, ZSTD_flushStream,
    ZSTD_freeCCtx, ZSTD_freeCDict, ZSTD_getDictID_fromCDict, ZSTD_initCStream, ZSTD_maxCLevel,
    ZSTD_minCLevel, ZSTD_sizeof_CCtx, ZSTD_sizeof_CDict,
};

pub mod internal {
    // Needed by benchzstd
    pub use crate::lib::common::xxhash::ZSTD_XXH64;

    // Needed by fileio
    pub use crate::lib::common::mem::{MEM_readLE24, MEM_readLE32};

    // Needed by fileio_asyncio
    pub use crate::lib::common::pool::{
        POOL_add, POOL_create, POOL_ctx, POOL_free, POOL_function, POOL_joinJobs,
    };
}

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

#[cfg(feature = "export-symbols")]
pub(crate) use prefix;
