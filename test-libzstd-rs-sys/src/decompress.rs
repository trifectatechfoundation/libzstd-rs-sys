use std::ffi::CStr;

macro_rules! decompress_stream {
    ($compressed:expr, $dict:expr) => {
        unsafe {
            use core::ffi::c_void;

            // Allocate and initialize a decompression context
            let dctx = ZSTD_createDCtx();
            if dctx.is_null() {
                panic!("Failed to create DCtx");
            }

            if let Some(dict) = $dict {
                // Initialize decompression with the dictionary
                let ddict = ZSTD_createDDict(dict.as_ptr() as *const c_void, dict.len());
                if ddict.is_null() {
                    panic!("Failed to create DDict");
                }

                // Reference the dictionary in the decompression context
                let res = ZSTD_DCtx_refDDict(dctx, ddict);
                assert_eq!(ZSTD_isError(res), 0, "Failed to reference DDict");
            }

            // Prepare input/output buffers for streaming
            let mut input = ZSTD_inBuffer {
                src: $compressed.as_ptr() as *const c_void,
                size: $compressed.len(),
                pos: 0,
            };

            let size =
                ZSTD_getFrameContentSize($compressed.as_ptr() as *const c_void, $compressed.len());

            if size == ZSTD_CONTENTSIZE_UNKNOWN as _ {
                panic!("ZSTD_CONTENTSIZE_UNKNOWN")
            }

            if size == ZSTD_CONTENTSIZE_ERROR as _ {
                panic!("ZSTD_CONTENTSIZE_ERROR")
            };

            let mut output_buf = vec![0u8; size as usize];
            let mut output = ZSTD_outBuffer {
                dst: output_buf.as_mut_ptr() as *mut c_void,
                size: output_buf.len(),
                pos: 0,
            };

            loop {
                let remaining = ZSTD_decompressStream(dctx, &mut output, &mut input);
                if ZSTD_isError(remaining) != 0 {
                    let err = ZSTD_getErrorName(remaining);
                    panic!(
                        "ZSTD_decompressStream failed: {}",
                        std::ffi::CStr::from_ptr(err).to_string_lossy()
                    );
                }

                if output.pos > 0 {
                    // print!("{}", String::from_utf8_lossy(&output_buf[..output.pos]));
                    output.pos = 0; // reset for next chunk
                }

                if remaining == 0 {
                    break; // finished
                }
            }

            ZSTD_freeDCtx(dctx);

            output_buf.truncate(output.size);

            output_buf
        }
    };
}

fn decompress_stream_c(compressed: &[u8], dict: Option<&[u8]>) -> Vec<u8> {
    use zstd_sys::*;

    decompress_stream!(compressed, dict)
}

fn decompress_stream_rs(compressed: &[u8], dict: Option<&[u8]>) -> Vec<u8> {
    use libzstd_rs::*;

    decompress_stream!(compressed, dict)
}

mod fastest_wasm_zlib {
    use super::*;

    const DECOMPRESSED: &[u8] = include_bytes!("../test-data/The fastest WASM zlib.md");

    #[track_caller]
    fn helper(compressed: &[u8]) {
        let c = decompress_stream_c(compressed, None);
        assert_eq!(DECOMPRESSED, c);

        let rs = decompress_stream_rs(compressed, None);

        assert_eq!(c, rs);
    }

    #[test]
    fn zstd_1() {
        helper(include_bytes!(
            "../test-data/The fastest WASM zlib.md.zstd-1.zst"
        ));
    }

    #[test]
    fn zstd_19() {
        helper(include_bytes!(
            "../test-data/The fastest WASM zlib.md.zstd-19.zst"
        ));
    }

    #[test]
    fn zstd_long27_19() {
        helper(include_bytes!(
            "../test-data/The fastest WASM zlib.md.zstd-long27-19.zst"
        ));
    }

    #[test]
    fn zstd_long_ultra_22() {
        helper(include_bytes!(
            "../test-data/The fastest WASM zlib.md.zstd-ultra-22.zst"
        ));
    }

    #[test]
    fn zstd_custom_dict() {
        const DICT: &[u8] = include_bytes!("../test-data/compression-corpus.zstd");
        const COMPRESSED: &[u8] =
            include_bytes!("../test-data/The fastest WASM zlib.md.zstd-custom-dict.zst");

        let c = decompress_stream_c(COMPRESSED, Some(DICT));
        assert_eq!(DECOMPRESSED, c);

        let rs = decompress_stream_rs(COMPRESSED, Some(DICT));
        assert_eq!(c, rs);
    }
}

#[test]

fn decompress_using_dict() {
    use std::ffi::c_void;

    use libzstd_rs::*;

    let input_data = "The quick brown fox jumps high";

    let (compressed, dict_buffer) = unsafe {
        use zstd_sys::*;

        let samples: [&str; 16] = [
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

        let mut sample_data = Vec::new();

        let mut sample_sizes = Vec::new();

        for &s in &samples {
            sample_data.extend_from_slice(s.as_bytes());

            sample_sizes.push(s.len());
        }

        let dict_capacity = 16 * 1024;

        let mut dict_buffer = vec![0u8; dict_capacity];

        let dict_size = ZDICT_trainFromBuffer(
            dict_buffer.as_mut_ptr() as *mut c_void,
            dict_buffer.len(),
            sample_data.as_ptr() as *const c_void,
            sample_sizes.as_ptr(),
            dbg!(sample_sizes.len() as u32),
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
    };

    unsafe {
        let dctx = ZSTD_createDCtx();

        assert!(!dctx.is_null());

        // Initialize decompression with the raw dictionary buffer

        let res = ZSTD_decompressBegin_usingDict(
            dctx,
            dict_buffer.as_ptr() as *const c_void,
            dict_buffer.len(),
        );

        assert_eq!(ZSTD_isError(res), 0, "Init decompression failed");

        let mut decompressed = vec![0u8; input_data.len()];

        let decompressed_size = ZSTD_decompress_usingDict(
            dctx,
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            compressed.as_ptr() as *const c_void,
            compressed.len(),
            dict_buffer.as_ptr() as *const c_void,
            dict_buffer.len(),
        );

        assert_eq!(ZSTD_isError(decompressed_size), 0, "Decompression failed");

        ZSTD_freeDCtx(dctx);

        assert_eq!(input_data, String::from_utf8(decompressed).unwrap());
    }
}
