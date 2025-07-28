use c2rust_out::lib::common::zstd_common::ZSTD_getErrorName;
use c2rust_out::lib::decompress::zstd_decompress::*;
use std::ffi::*;

fn main() {
    // Your compressed data goes here
    // let compressed: &[u8] = include_bytes!("/tmp/foo.txt.zst");
    let compressed: &[u8] = include_bytes!("../silesia-small.tar.zst");
    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size =
        unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size as u64) };
    if decompressed_size == ZSTD_CONTENTSIZE_ERROR {
        eprintln!("Not a valid zstd compressed frame!");
        return;
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN {
        eprintln!("Original size unknown — use streaming decompression.");
        return;
    }

    // Allocate buffer for decompressed output
    let mut decompressed = vec![0u8; decompressed_size as usize];
    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed_size,
            compressed_ptr,
            compressed_size as u64,
        )
    };

    // Check for errors
    if unsafe { ZSTD_isError(result) } != 0 {
        let err_msg = unsafe {
            let ptr = ZSTD_getErrorName(result);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        };
        eprintln!("Decompression failed: {}", err_msg);
        return;
    }

    println!("Decompressed {} bytes successfully.", result);

    // Use `decompressed[..result as usize]` as your output data
}
