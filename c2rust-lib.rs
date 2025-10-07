#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(clippy::too_many_arguments)]
#![cfg_attr(all(feature = "nightly", test), feature(test))]
#![cfg_attr(feature = "nightly", feature(likely_unlikely))]
// FIXME
#![allow(clippy::missing_safety_doc)]

pub mod lib {
    pub mod common {
        pub(crate) mod allocations;
        pub(crate) mod bits;
        pub(crate) mod bitstream;
        pub mod debug;
        pub(crate) mod entropy_common;
        pub(crate) mod error_private;
        pub(crate) mod fse;
        pub(crate) mod fse_decompress;
        pub(crate) mod huf;
        pub(crate) mod mem;
        pub(crate) mod pool;
        pub(crate) mod reader;
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
        pub(crate) mod zstd_compress_internal;
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
    ZSTD_ParamSwitch_e, ZSTD_ResetDirective, ZSTD_cParameter, ZSTD_dParameter,
    ZSTD_dictAttachPref_e, ZSTD_format_e, ZSTD_frameProgression, ZSTD_inBuffer, ZSTD_outBuffer,
    ZSTD_strategy, ZSTD_BLOCKSIZELOG_MAX, ZSTD_BLOCKSIZE_MAX, ZSTD_CLEVEL_DEFAULT,
    ZSTD_FRAMEHEADERSIZE_MAX, ZSTD_MAGICNUMBER, ZSTD_MAGIC_DICTIONARY, ZSTD_MAGIC_SKIPPABLE_MASK,
    ZSTD_MAGIC_SKIPPABLE_START, ZSTD_VERSION_MAJOR, ZSTD_VERSION_MINOR, ZSTD_VERSION_NUMBER,
    ZSTD_VERSION_RELEASE, ZSTD_WINDOWLOG_MAX_32, ZSTD_WINDOWLOG_MAX_64,
};

pub use crate::lib::decompress::{
    zstd_ddict::{
        ZSTD_DDict, ZSTD_createDDict, ZSTD_createDDict_byReference, ZSTD_freeDDict,
        ZSTD_getDictID_fromDDict, ZSTD_sizeof_DDict,
    },
    zstd_decompress::{
        ZSTD_DCtx_getParameter, ZSTD_DCtx_loadDictionary, ZSTD_DCtx_refDDict, ZSTD_DCtx_refPrefix,
        ZSTD_DCtx_reset, ZSTD_DCtx_setParameter, ZSTD_DStream, ZSTD_DStreamInSize,
        ZSTD_DStreamOutSize, ZSTD_copyDCtx, ZSTD_createDCtx, ZSTD_createDStream,
        ZSTD_dParam_getBounds, ZSTD_decompress, ZSTD_decompressBegin,
        ZSTD_decompressBegin_usingDDict, ZSTD_decompressBegin_usingDict, ZSTD_decompressBound,
        ZSTD_decompressContinue, ZSTD_decompressDCtx, ZSTD_decompressStream,
        ZSTD_decompressStream_simpleArgs, ZSTD_decompress_usingDDict, ZSTD_decompress_usingDict,
        ZSTD_decompressionMargin, ZSTD_findDecompressedSize, ZSTD_findFrameCompressedSize,
        ZSTD_freeDCtx, ZSTD_getDecompressedSize, ZSTD_getDictID_fromDict, ZSTD_getDictID_fromFrame,
        ZSTD_getFrameContentSize, ZSTD_initDStream, ZSTD_initDStream_usingDDict,
        ZSTD_initDStream_usingDict, ZSTD_insertBlock, ZSTD_isFrame, ZSTD_isSkippableFrame,
        ZSTD_nextSrcSizeToDecompress, ZSTD_sizeof_DCtx, ZSTD_CONTENTSIZE_ERROR,
        ZSTD_CONTENTSIZE_UNKNOWN,
    },
    zstd_decompress_block::ZSTD_decompressBlock,
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
    ZSTD_CCtx, ZSTD_CCtx_getParameter, ZSTD_CCtx_loadDictionary, ZSTD_CCtx_refCDict,
    ZSTD_CCtx_refPrefix, ZSTD_CCtx_reset, ZSTD_CCtx_setParameter, ZSTD_CCtx_setPledgedSrcSize,
    ZSTD_CDict, ZSTD_CStreamInSize, ZSTD_CStreamOutSize, ZSTD_EndDirective, ZSTD_cParam_getBounds,
    ZSTD_compress, ZSTD_compress2, ZSTD_compressBlock, ZSTD_compressBound, ZSTD_compressCCtx,
    ZSTD_compressStream, ZSTD_compressStream2, ZSTD_compress_usingCDict, ZSTD_compress_usingDict,
    ZSTD_copyCCtx, ZSTD_createCCtx, ZSTD_createCDict, ZSTD_createCDict_byReference, ZSTD_endStream,
    ZSTD_flushStream, ZSTD_freeCCtx, ZSTD_freeCDict, ZSTD_getBlockSize, ZSTD_getDictID_fromCDict,
    ZSTD_getFrameProgression, ZSTD_initCStream, ZSTD_initCStream_srcSize,
    ZSTD_initCStream_usingCDict, ZSTD_initCStream_usingDict, ZSTD_maxCLevel, ZSTD_minCLevel,
    ZSTD_sequenceBound, ZSTD_sizeof_CCtx, ZSTD_sizeof_CDict, ZSTD_BLOCKSIZE_MAX_MIN,
    ZSTD_BLOCKSPLITTER_LEVEL_MAX, ZSTD_CHAINLOG_MAX_32, ZSTD_CHAINLOG_MAX_64, ZSTD_CHAINLOG_MIN,
    ZSTD_HASHLOG_MIN, ZSTD_LDM_BUCKETSIZELOG_MAX, ZSTD_LDM_BUCKETSIZELOG_MIN, ZSTD_LDM_HASHLOG_MIN,
    ZSTD_LDM_HASHRATELOG_MIN, ZSTD_LDM_MINMATCH_MAX, ZSTD_LDM_MINMATCH_MIN, ZSTD_MINMATCH_MAX,
    ZSTD_MINMATCH_MIN, ZSTD_OVERLAPLOG_MAX, ZSTD_OVERLAPLOG_MIN, ZSTD_SEARCHLOG_MIN,
    ZSTD_SKIPPABLEHEADERSIZE, ZSTD_SRCSIZEHINT_MIN, ZSTD_TARGETCBLOCKSIZE_MAX,
    ZSTD_TARGETCBLOCKSIZE_MIN, ZSTD_TARGETLENGTH_MAX, ZSTD_TARGETLENGTH_MIN,
    ZSTD_WINDOWLOG_LIMIT_DEFAULT, ZSTD_WINDOWLOG_MIN,
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
