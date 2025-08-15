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
