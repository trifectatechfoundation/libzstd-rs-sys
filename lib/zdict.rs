#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: core::ffi::c_int,
    pub notificationLevel: core::ffi::c_uint,
    pub dictID: core::ffi::c_uint,
}

pub use crate::lib::dictBuilder::zdict::{
    ZDICT_finalizeDictionary, ZDICT_getDictHeaderSize, ZDICT_getDictID, ZDICT_getErrorName,
    ZDICT_isError, ZDICT_trainFromBuffer,
};

pub mod experimental {
    use super::*;

    pub const ZDICT_DICTSIZE_MIN: core::ffi::c_int = 256;

    #[deprecated = "will be removed in v1.6.0"]
    pub const ZDICT_CONTENTSIZE_MIN: core::ffi::c_int = 128;

    #[derive(Debug, Copy, Clone, Default)]
    #[repr(C)]
    pub struct ZDICT_cover_params_t {
        pub k: core::ffi::c_uint,
        pub d: core::ffi::c_uint,
        pub steps: core::ffi::c_uint,
        pub nbThreads: core::ffi::c_uint,
        pub splitPoint: core::ffi::c_double,
        pub shrinkDict: core::ffi::c_uint,
        pub shrinkDictMaxRegression: core::ffi::c_uint,
        pub zParams: ZDICT_params_t,
    }

    #[derive(Debug, Copy, Clone, Default)]
    #[repr(C)]
    pub struct ZDICT_fastCover_params_t {
        pub k: core::ffi::c_uint,
        pub d: core::ffi::c_uint,
        pub f: core::ffi::c_uint,
        pub steps: core::ffi::c_uint,
        pub nbThreads: core::ffi::c_uint,
        pub splitPoint: core::ffi::c_double,
        pub accel: core::ffi::c_uint,
        pub shrinkDict: core::ffi::c_uint,
        pub shrinkDictMaxRegression: core::ffi::c_uint,
        pub zParams: ZDICT_params_t,
    }

    pub use crate::lib::dictBuilder::cover::{
        ZDICT_optimizeTrainFromBuffer_cover, ZDICT_trainFromBuffer_cover,
    };
    pub use crate::lib::dictBuilder::fastcover::{
        ZDICT_optimizeTrainFromBuffer_fastCover, ZDICT_trainFromBuffer_fastCover,
    };

    #[derive(Debug, Copy, Clone, Default)]
    #[repr(C)]
    pub struct ZDICT_legacy_params_t {
        pub selectivityLevel: core::ffi::c_uint,
        pub zParams: ZDICT_params_t,
    }

    #[expect(deprecated)]
    pub use crate::lib::dictBuilder::zdict::{
        ZDICT_addEntropyTablesFromBuffer, ZDICT_trainFromBuffer_legacy,
    };
}
