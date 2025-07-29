#![no_main]

use libfuzzer_sys::fuzz_target;

use std::ffi::*;

fn rs(compressed: &[u8]) -> (usize, Vec<u8>) {
    use c2rust_out::lib::common::zstd_common::ZSTD_getErrorName;
    use c2rust_out::lib::decompress::zstd_decompress::*;

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size =
        unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size as u64) };
    if decompressed_size == ZSTD_CONTENTSIZE_ERROR {
        return (decompressed_size as usize, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN {
        return (decompressed_size as usize, vec![]);
    }

    return (0, vec![]);

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

    decompressed.truncate(result as usize);

    (result as usize, decompressed)
}

fn c(compressed: &[u8]) -> (usize, Vec<u8>) {
    use zstd_sys::*;

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size = unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) };
    if decompressed_size == ZSTD_CONTENTSIZE_ERROR as u64 {
        return (decompressed_size as usize, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN as u64 {
        return (decompressed_size as usize, vec![]);
    }

    return (0, vec![]);

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

    decompressed.truncate(result as usize);

    (result, decompressed)
}

fuzz_target!(|data: &[u8]| {
    let (rs_err, rs_out) = rs(data);
    let (c_err, c_out) = c(data);

    assert_eq!(rs_err, c_err, "{:?}", data);
    assert_eq!(rs_out, c_out);
});
