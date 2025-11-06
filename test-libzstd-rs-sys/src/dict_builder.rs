use crate::assert_eq_rs_c;
use std::ffi::{c_void, CStr};

const SAMPLES: [&str; 16] = [
    "The quick brown fox jumps over the lazy dog",
    "The quick brown fox jumps high",
    "The slow turtle crawls under the energetic cat",
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit",
    "Pack my box with five dozen liquor jugs",
    "Bright vixens jump; dozy fowl quack",
    "Sphinx of black quartz, judge my vow",
    "How razorback-jumping frogs can level six piqued gymnasts",
    "Crazy Fredrick bought many very exquisite opal jewels",
    "Five quacking zephyrs jolt my wax bed",
    "Jackdaws love my big sphinx of quartz",
    "Two driven jocks help fax my big quiz",
    "The wizard quickly jinxed the gnomes before they vaporized",
    "Quick zephyrs blow, vexing daft Jim",
    "Heavy boxes perform quick waltzes and jigs",
    "Jovial harpooned sharks quizzed exotic men drinking water",
];

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_train_from_buffer_cover() {
    let input_data = "The quick brown fox jumps high";

    assert_eq_rs_c!({
        let mut sample_data = Vec::new();

        let mut sample_sizes = Vec::new();

        for &s in &SAMPLES {
            sample_data.extend_from_slice(s.as_bytes());

            sample_sizes.push(s.len());
        }

        let dict_capacity = 16 * 1024;

        let mut dict_buffer = vec![0u8; dict_capacity];

        let params = ZDICT_cover_params_t {
            k: 200,
            d: 8,
            steps: 4,
            nbThreads: 1,
            splitPoint: 75.0,
            shrinkDict: 0,
            shrinkDictMaxRegression: 1,
            zParams: ZDICT_params_t {
                compressionLevel: 3,
                notificationLevel: 0,
                dictID: 0,
            },
        };

        let dict_size = ZDICT_trainFromBuffer_cover(
            dict_buffer.as_mut_ptr() as *mut c_void,
            dict_buffer.len(),
            sample_data.as_ptr() as *const c_void,
            sample_sizes.as_ptr(),
            sample_sizes.len() as u32,
            params,
        );

        assert_eq!(
            ZDICT_isError(dict_size),
            0,
            "Dict training failed {:?}",
            CStr::from_ptr(ZDICT_getErrorName(dict_size)).to_str(),
        );

        dict_buffer.truncate(dict_size);

        println!("Dictionary size: {}", dict_size);

        let cctx = ZSTD_createCCtx();

        assert!(!cctx.is_null());

        let max_compressed_size = ZSTD_compressBound(input_data.len());

        let mut compressed = vec![0u8; max_compressed_size];

        let compressed_size = ZSTD_compress_usingDict(
            cctx,
            compressed.as_mut_ptr() as *mut c_void,
            compressed.len(),
            input_data.as_bytes().as_ptr() as *const c_void,
            input_data.len(),
            dict_buffer.as_ptr() as *const c_void,
            dict_buffer.len(),
            3, // compression level
        );

        assert_eq!(ZSTD_isError(compressed_size), 0, "Compression failed");

        compressed.truncate(compressed_size);

        println!("Compressed size: {}", compressed_size);

        ZSTD_freeCCtx(cctx);

        (compressed, dict_buffer)
    });
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_train_from_buffer_fastcover() {
    let input_data = "The quick brown fox jumps high";

    assert_eq_rs_c!({
        let mut sample_data = Vec::new();

        let mut sample_sizes = Vec::new();

        for &s in &SAMPLES {
            sample_data.extend_from_slice(s.as_bytes());

            sample_sizes.push(s.len());
        }

        let dict_capacity = 16 * 1024;

        let mut dict_buffer = vec![0u8; dict_capacity];

        let params = ZDICT_fastCover_params_t {
            k: 200,
            d: 8,
            f: 20,
            steps: 4,
            nbThreads: 1,
            splitPoint: 75.0,
            shrinkDict: 0,
            shrinkDictMaxRegression: 1,
            zParams: ZDICT_params_t {
                compressionLevel: 3,
                notificationLevel: 0,
                dictID: 0,
            },
            accel: 1,
        };

        let dict_size = ZDICT_trainFromBuffer_fastCover(
            dict_buffer.as_mut_ptr() as *mut c_void,
            dict_buffer.len(),
            sample_data.as_ptr() as *const c_void,
            sample_sizes.as_ptr(),
            sample_sizes.len() as u32,
            params,
        );

        assert_eq!(
            ZDICT_isError(dict_size),
            0,
            "Dict training failed {:?}",
            CStr::from_ptr(ZDICT_getErrorName(dict_size)).to_str(),
        );

        dict_buffer.truncate(dict_size);

        println!("Dictionary size: {}", dict_size);

        let cctx = ZSTD_createCCtx();

        assert!(!cctx.is_null());

        let max_compressed_size = ZSTD_compressBound(input_data.len());

        let mut compressed = vec![0u8; max_compressed_size];

        let compressed_size = ZSTD_compress_usingDict(
            cctx,
            compressed.as_mut_ptr() as *mut c_void,
            compressed.len(),
            input_data.as_bytes().as_ptr() as *const c_void,
            input_data.len(),
            dict_buffer.as_ptr() as *const c_void,
            dict_buffer.len(),
            3, // compression level
        );

        assert_eq!(ZSTD_isError(compressed_size), 0, "Compression failed");

        compressed.truncate(compressed_size);

        println!("Compressed size: {}", compressed_size);

        ZSTD_freeCCtx(cctx);

        (compressed, dict_buffer)
    });
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_optimize_train_from_buffer_cover_single_threaded() {
    test_optimize_train_from_buffer_cover_pool(1);
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_optimize_train_from_buffer_cover_multi_threaded() {
    test_optimize_train_from_buffer_cover_pool(4);
}

fn test_optimize_train_from_buffer_cover_pool(thread_count: u32) {
    let input_data = "The quick brown fox jumps high";

    assert_eq_rs_c!({
        let mut sample_data = Vec::new();

        let mut sample_sizes = Vec::new();

        for &s in &SAMPLES {
            sample_data.extend_from_slice(s.as_bytes());

            sample_sizes.push(s.len());
        }

        let dict_capacity = 16 * 1024;

        let mut dict_buffer = vec![0u8; dict_capacity];

        let mut params = ZDICT_cover_params_t {
            k: 200,
            d: 8,
            steps: 4,
            nbThreads: thread_count,
            splitPoint: 0.5,
            shrinkDict: 0,
            shrinkDictMaxRegression: 1,
            zParams: ZDICT_params_t {
                compressionLevel: 3,
                notificationLevel: 1,
                dictID: 0,
            },
        };

        let dict_size = ZDICT_optimizeTrainFromBuffer_cover(
            dict_buffer.as_mut_ptr() as *mut c_void,
            dict_buffer.len(),
            sample_data.as_ptr() as *const c_void,
            sample_sizes.as_ptr(),
            sample_sizes.len() as u32,
            &mut params,
        );

        assert_eq!(
            ZDICT_isError(dict_size),
            0,
            "Dict training failed {:?}",
            CStr::from_ptr(ZDICT_getErrorName(dict_size)).to_str(),
        );

        dict_buffer.truncate(dict_size);

        println!("Dictionary size: {}", dict_size);

        let cctx = ZSTD_createCCtx();

        assert!(!cctx.is_null());

        let max_compressed_size = ZSTD_compressBound(input_data.len());

        let mut compressed = vec![0u8; max_compressed_size];

        let compressed_size = ZSTD_compress_usingDict(
            cctx,
            compressed.as_mut_ptr() as *mut c_void,
            compressed.len(),
            input_data.as_bytes().as_ptr() as *const c_void,
            input_data.len(),
            dict_buffer.as_ptr() as *const c_void,
            dict_buffer.len(),
            3, // compression level
        );

        assert_eq!(ZSTD_isError(compressed_size), 0, "Compression failed");

        compressed.truncate(compressed_size);

        println!("Compressed size: {}", compressed_size);

        ZSTD_freeCCtx(cctx);

        (compressed, dict_buffer)
    });
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_optimize_train_from_buffer_fastcover_single_threaded() {
    test_optimize_train_from_buffer_fastcover_pool(1);
}

#[test]
#[ignore = "let's fix the single-threaded case first"]
#[cfg(not(target_family = "wasm"))]
fn test_optimize_train_from_buffer_fastcover_multi_threaded() {
    test_optimize_train_from_buffer_fastcover_pool(4);
}

fn test_optimize_train_from_buffer_fastcover_pool(thread_count: u32) {
    let input_data = "The quick brown fox jumps high";

    assert_eq_rs_c!({
        let mut sample_data = Vec::new();

        let mut sample_sizes = Vec::new();

        for &s in &SAMPLES {
            sample_data.extend_from_slice(s.as_bytes());

            sample_sizes.push(s.len());
        }

        let dict_capacity = 16 * 1024;

        let mut dict_buffer = vec![0u8; dict_capacity];

        let mut params = ZDICT_fastCover_params_t {
            k: 200,
            d: 8,
            steps: 4,
            nbThreads: thread_count,
            splitPoint: 0.5,
            shrinkDict: 0,
            shrinkDictMaxRegression: 1,
            zParams: ZDICT_params_t {
                compressionLevel: 3,
                notificationLevel: 1,
                dictID: 0,
            },
            f: 20,
            accel: 1,
        };

        let dict_size = ZDICT_optimizeTrainFromBuffer_fastCover(
            dict_buffer.as_mut_ptr() as *mut c_void,
            dict_buffer.len(),
            sample_data.as_ptr() as *const c_void,
            sample_sizes.as_ptr(),
            sample_sizes.len() as u32,
            &mut params,
        );

        assert_eq!(
            ZDICT_isError(dict_size),
            0,
            "Dict training failed {:?}",
            CStr::from_ptr(ZDICT_getErrorName(dict_size)).to_str(),
        );

        dict_buffer.truncate(dict_size);

        println!("Dictionary size: {}", dict_size);

        let cctx = ZSTD_createCCtx();

        assert!(!cctx.is_null());

        let max_compressed_size = ZSTD_compressBound(input_data.len());

        let mut compressed = vec![0u8; max_compressed_size];

        let compressed_size = ZSTD_compress_usingDict(
            cctx,
            compressed.as_mut_ptr() as *mut c_void,
            compressed.len(),
            input_data.as_bytes().as_ptr() as *const c_void,
            input_data.len(),
            dict_buffer.as_ptr() as *const c_void,
            dict_buffer.len(),
            3, // compression level
        );

        assert_eq!(ZSTD_isError(compressed_size), 0, "Compression failed");

        compressed.truncate(compressed_size);

        println!("Compressed size: {}", compressed_size);

        ZSTD_freeCCtx(cctx);

        (compressed, dict_buffer)
    });
}
