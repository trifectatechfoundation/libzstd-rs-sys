use core::ffi::c_void;
use std::borrow::Cow;

fn main() {
    let mut it = std::env::args();

    let _ = it.next().unwrap();
    let variant = it.next().unwrap();
    let file = it.next();

    let silesia_small_tar_zst = include_bytes!("../../silesia-small.tar.zst");

    let input: Cow<'_, [u8]> = match file {
        None => silesia_small_tar_zst.into(),
        Some(path) => std::fs::read(path).unwrap().into(),
    };

    const N: usize = 20; // do multiple runs for benchmarking

    match variant.as_str() {
        "c" => {
            for _ in 0..N {
                c(&input);
            }
        }
        "rs" => {
            for _ in 0..N {
                rs(&input);
            }
        }
        "both" => {
            for _ in 0..N {
                rs(&input);
                c(&input);
            }
        }
        "rs-chunked" => {
            let chunk_size = it.next().unwrap().parse().unwrap();
            let dict = it.next().map(|path| std::fs::read(path).unwrap());

            for _ in 0..N {
                rs_chunked(&input, chunk_size, dict.as_deref());
            }
        }
        "c-chunked" => {
            let chunk_size = it.next().unwrap().parse().unwrap();
            let dict = it.next().map(|path| std::fs::read(path).unwrap());

            for _ in 0..N {
                c_chunked(&input, chunk_size, dict.as_deref());
            }
        }
        "both-chunked" => {
            let chunk_size = it.next().unwrap().parse().unwrap();
            let dict = it.next().map(|path| std::fs::read(path).unwrap());
            for _ in 0..N {
                rs_chunked(&input, chunk_size, dict.as_deref());
                c_chunked(&input, chunk_size, dict.as_deref());
            }
        }
        bad => panic!("invalid command {bad}"),
    }
}

macro_rules! decompress {
    ($compressed:expr) => {
        let compressed_ptr = $compressed.as_ptr() as *const c_void;
        let compressed_size = $compressed.len();

        // Get decompressed size from frame header
        let decompressed_size =
            unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) };
        if decompressed_size == ZSTD_CONTENTSIZE_ERROR as u64 {
            panic!("Not a valid zstd compressed frame!");
        } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN as u64 {
            panic!("Original size unknown — use streaming decompression.");
        }

        // Allocate buffer for decompressed output
        let mut decompressed = vec![0u8; decompressed_size as usize];
        let result = unsafe {
            ZSTD_decompress(
                decompressed.as_mut_ptr() as *mut c_void,
                decompressed_size as usize,
                compressed_ptr,
                compressed_size,
            )
        };

        // Check for errors
        if ZSTD_isError(result) != 0 {
            let err_msg = unsafe {
                let ptr = ZSTD_getErrorName(result);
                core::ffi::CStr::from_ptr(ptr)
                    .to_string_lossy()
                    .into_owned()
            };
            panic!("Decompression failed: {}", err_msg);
        }
    };
}

fn rs(compressed: &[u8]) {
    use libzstd_rs_sys::*;

    decompress!(compressed);
}

fn c(compressed: &[u8]) {
    use zstd_sys::*;

    // `ZSTD_isError` is safe in the Rust version, but unsafe with the C bindings
    #[allow(unused_unsafe)]
    unsafe {
        decompress!(compressed);
    }
}

macro_rules! decompress_chunks {
    ($compressed:expr, $chunk_size:expr, $dict:expr) => {
        // Allocate and initialize a decompression context
        let dctx = unsafe { ZSTD_createDCtx() };
        if dctx.is_null() {
            panic!("Failed to create DCtx");
        }

        if let Some(dict) = $dict {
            // Initialize decompression with the dictionary
            let ddict = unsafe { ZSTD_createDDict(dict.as_ptr() as *const c_void, dict.len()) };
            if ddict.is_null() {
                panic!("Failed to create DDict");
            }

            // Reference the dictionary in the decompression context
            let res = unsafe { ZSTD_DCtx_refDDict(dctx, ddict) };
            assert_eq!(ZSTD_isError(res), 0, "Failed to reference DDict");
        }

        // Get decompressed size from frame header
        let decompressed_size =
            unsafe { ZSTD_getFrameContentSize($compressed.as_ptr() as *const c_void, $chunk_size) };
        if decompressed_size == ZSTD_CONTENTSIZE_ERROR as u64 {
            panic!("Not a valid zstd compressed frame!");
        } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN as u64 {
            panic!("Original size unknown — use streaming decompression.");
        }
        let mut output_buf = vec![0u8; decompressed_size as usize];
        let mut output = ZSTD_outBuffer {
            dst: output_buf.as_mut_ptr() as *mut c_void,
            size: output_buf.len(),
            pos: 0,
        };

        for chunk in $compressed.chunks($chunk_size) {
            let mut input = ZSTD_inBuffer {
                src: chunk.as_ptr() as *const c_void,
                size: chunk.len(),
                pos: 0,
            };

            // Allocate buffer for decompressed output
            let result = unsafe { ZSTD_decompressStream(dctx, &mut output, &mut input) };

            // Check for errors
            if ZSTD_isError(result) != 0 {
                let err_msg = unsafe {
                    let ptr = ZSTD_getErrorName(result);
                    core::ffi::CStr::from_ptr(ptr)
                        .to_string_lossy()
                        .into_owned()
                };
                panic!("Decompression failed: {}", err_msg);
            }

            if input.pos < input.size {
                panic!("Not all input data consumed");
            }
        }
    };
}

fn rs_chunked(compressed: &[u8], chunk_size: usize, dict: Option<&[u8]>) {
    use libzstd_rs_sys::*;

    decompress_chunks!(compressed, chunk_size, dict);
}

fn c_chunked(compressed: &[u8], chunk_size: usize, dict: Option<&[u8]>) {
    use zstd_sys::*;

    // `ZSTD_isError` is safe in the Rust version, but unsafe with the C bindings
    #[allow(unused_unsafe)]
    unsafe {
        decompress_chunks!(compressed, chunk_size, dict);
    }
}
