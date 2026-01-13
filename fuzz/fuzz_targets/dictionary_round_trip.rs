#![no_main]
#![allow(deprecated)]

use c2rust_out_fuzz::ArbitrarySamples;
use libfuzzer_sys::{arbitrary, fuzz_target};
use libzstd_rs_sys::{
    internal::ZSTD_XXH64,
    lib::{
        compress::zstd_compress::{
            ZSTD_CCtx_loadDictionary_advanced, ZSTD_CCtx_refPrefix_advanced,
        },
        decompress::zstd_decompress::{
            ZSTD_DCtx_loadDictionary_advanced, ZSTD_DCtx_refPrefix_advanced,
        },
        zstd::ZSTD_dct_auto,
    },
};

macro_rules! zassert {
    ($expr:expr, $msg:literal) => {{
        let res = unsafe { $expr };
        if ZSTD_isError(res) == 1 {
            let err_msg = unsafe {
                let ptr = ZSTD_getErrorName(res);
                core::ffi::CStr::from_ptr(ptr)
                    .to_string_lossy()
                    .into_owned()
            };
            panic!("{}: {err_msg}", $msg);
        }
        res
    }};
}

const MIN_CLEVEL: i32 = -3;
const MAX_CLEVEL: i32 = 19;

#[derive(Debug)]
struct ArbitraryRoundTrip {
    pub ref_prefix: bool,
    pub c_buf_size_minus_one: bool,
    pub dctx_load_dict_method: bool,
    pub compression: CompressionStrat,
}

#[derive(Debug)]
enum CompressionStrat {
    CompressUsingDict {
        c_level: i32,
    },
    Compress2 {
        dict_content_type: u8,
        cctx_load_dict_method: u32,
    },
}

impl arbitrary::Arbitrary<'_> for ArbitraryRoundTrip {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        // pick a compression strategy
        let compression = if u8::arbitrary(u)? % 16 == 0 {
            CompressionStrat::CompressUsingDict {
                c_level: u.int_in_range(MIN_CLEVEL..=MAX_CLEVEL)?,
            }
        } else {
            CompressionStrat::Compress2 {
                dict_content_type: u.int_in_range(0..=2)?,
                cctx_load_dict_method: u.int_in_range(0..=1)?,
            }
        };

        Ok(ArbitraryRoundTrip {
            ref_prefix: bool::arbitrary(u)?,
            c_buf_size_minus_one: bool::arbitrary(u)?,
            dctx_load_dict_method: bool::arbitrary(u)?,
            compression,
        })
    }
}

fuzz_target!(|data: (ArbitraryRoundTrip, ArbitrarySamples)| {
    use libzstd_rs_sys::*;

    let (data, samples) = data;

    let src = samples.src;

    let result_capacity = src.len();
    let mut result = vec![0; result_capacity];

    // Half of the time fuzz with a 1 byte smaller output size.
    // This will still succeed because we force the checksum to be disabled, giving us 4 bytes of overhead.
    let compressed_capacity = ZSTD_compressBound(src.len()) - data.c_buf_size_minus_one as usize;
    let mut compressed = vec![0; compressed_capacity];

    let cctx = unsafe { ZSTD_createCCtx() };
    assert!(!cctx.is_null());
    let dctx = unsafe { ZSTD_createDCtx() };
    assert!(!dctx.is_null());

    let dict_content_type = ZSTD_dct_auto;

    // Train dictionary
    let params = ZDICT_fastCover_params_t {
        accel: 5,
        k: 40,
        d: 8,
        f: 14,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.0,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 1,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    let mut dict_size = samples.dict_size;
    let mut dict = vec![0u8; dict_size];
    let res = unsafe {
        ZDICT_trainFromBuffer_fastCover(
            dict.as_mut_ptr().cast(),
            dict_size,
            samples.samples.as_ptr().cast(),
            samples.sample_sizes.as_ptr(),
            samples.nb_samples as u32,
            params,
        )
    };
    if ZSTD_isError(res) == 1 {
        dict = vec![];
        dict_size = 0;
    } else {
        dict_size = res;
    }

    // Compress
    let c_size = match data.compression {
        CompressionStrat::CompressUsingDict { c_level } => {
            let c_size = zassert!(
                ZSTD_compress_usingDict(
                    cctx,
                    compressed.as_mut_ptr().cast(),
                    compressed_capacity,
                    src.as_ptr().cast(),
                    src.len(),
                    dict.as_ptr().cast(),
                    dict_size,
                    c_level,
                ),
                "Compression using dict failed"
            );

            // Compress a second time and check for determinism
            let hash = unsafe { ZSTD_XXH64(compressed.as_ptr().cast(), c_size, 0) };
            let c_size2 = unsafe {
                ZSTD_compress_usingDict(
                    cctx,
                    compressed.as_mut_ptr().cast(),
                    compressed_capacity,
                    src.as_ptr().cast(),
                    src.len(),
                    dict.as_ptr().cast(),
                    dict_size,
                    c_level,
                )
            };
            assert_eq!(c_size, c_size2);
            let hash2 = unsafe { ZSTD_XXH64(compressed.as_ptr().cast(), c_size2, 0) };
            assert_eq!(hash, hash2);

            c_size
        }
        CompressionStrat::Compress2 {
            dict_content_type,
            cctx_load_dict_method,
        } => {
            // TODO: set random parameters?

            // Disable checksum so we can use sizes smaller than compress bound.
            zassert!(
                ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_checksumFlag, 0),
                "Could not set checksum flag"
            );

            if data.ref_prefix {
                zassert!(
                    ZSTD_CCtx_refPrefix_advanced(
                        cctx,
                        dict.as_ptr().cast(),
                        dict_size,
                        dict_content_type.into()
                    ),
                    "Referencing dictionary prefix for compression failed"
                );
            } else {
                zassert!(
                    ZSTD_CCtx_loadDictionary_advanced(
                        cctx,
                        dict.as_ptr().cast(),
                        dict_size,
                        cctx_load_dict_method,
                        dict_content_type.into()
                    ),
                    "Loading dictionary for compression failed"
                );
            }

            let c_size = zassert!(
                ZSTD_compress2(
                    cctx,
                    compressed.as_mut_ptr().cast(),
                    compressed_capacity,
                    src.as_ptr().cast(),
                    src.len(),
                ),
                "Compression 2 failed"
            );

            // Compress a second time and check for determinism
            let hash = unsafe { ZSTD_XXH64(compressed.as_ptr().cast(), c_size, 0) };
            // TODO: reset random parameters?
            zassert!(
                ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_checksumFlag, 0),
                "Could not set checksum flag"
            );
            if data.ref_prefix {
                zassert!(
                    ZSTD_CCtx_refPrefix_advanced(
                        cctx,
                        dict.as_ptr().cast(),
                        dict_size,
                        dict_content_type.into()
                    ),
                    "Referencing dictionary prefix for compression failed"
                );
            }
            let c_size2 = zassert!(
                ZSTD_compress2(
                    cctx,
                    compressed.as_mut_ptr().cast(),
                    compressed_capacity,
                    src.as_ptr().cast(),
                    src.len(),
                ),
                "Compression 2 failed"
            );
            assert_eq!(c_size, c_size2);
            let hash2 = unsafe { ZSTD_XXH64(compressed.as_ptr().cast(), c_size2, 0) };
            assert_eq!(hash, hash2);

            c_size
        }
    };

    // Decompress
    if data.ref_prefix {
        zassert!(
            ZSTD_DCtx_refPrefix_advanced(dctx, dict.as_ptr().cast(), dict_size, dict_content_type,),
            "Referencing dictionary prefix for decompression failed"
        );
    } else {
        zassert!(
            ZSTD_DCtx_loadDictionary_advanced(
                dctx,
                dict.as_ptr().cast(),
                dict_size,
                data.dctx_load_dict_method as u32,
                dict_content_type,
            ),
            "Loading dictionary for decompression failed"
        );
    }

    let res = zassert!(
        ZSTD_decompressDCtx(
            dctx,
            result.as_mut_ptr().cast(),
            result_capacity,
            compressed.as_ptr().cast(),
            c_size,
        ),
        "Decompression failed"
    );
    assert!(res == src.len(), "Incorrect regenerated size");
    assert_eq!(result, src, "Corruption!");

    unsafe {
        ZSTD_freeCCtx(cctx);
        ZSTD_freeDCtx(dctx);
    }
});
