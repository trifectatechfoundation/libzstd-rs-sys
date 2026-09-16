#![no_main]

use libfuzzer_sys::fuzz_target;
use libzstd_rs_sys_fuzz::{to_result, Error};

use std::ffi::*;

fn rs(compressed: &[u8]) -> (usize, Vec<u8>) {
    use libzstd_rs_sys::*;

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size = unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) };
    if decompressed_size == ZSTD_CONTENTSIZE_ERROR {
        return (decompressed_size as usize, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN {
        return (decompressed_size as usize, vec![]);
    }

    // Allocate buffer for decompressed output
    let mut decompressed = vec![0u8; Ord::min(decompressed_size as usize, 1 << 20)];
    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            compressed_ptr,
            compressed_size,
        )
    };

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

    // Allocate buffer for decompressed output
    let mut decompressed = vec![0u8; Ord::min(decompressed_size as usize, 1 << 20)];
    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            compressed_ptr,
            compressed_size,
        )
    };

    (result, decompressed)
}

fuzz_target!(|data: &[u8]| {
    let (c_err, c_out) = c(data);
    let (rs_err, rs_out) = rs(data);

    let rs_err = to_result(rs_err);
    let c_err = to_result(c_err);

    // The zstd version that we're testing against supports much older legacy versions.
    if rs_err == Err(Error::prefix_unknown) {
        return;
    }

    assert_eq!(rs_err, c_err);

    if rs_err.is_ok() {
        assert_eq!(rs_out, c_out);
    }
});
