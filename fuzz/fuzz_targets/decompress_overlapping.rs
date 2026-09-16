#![no_main]

use libfuzzer_sys::fuzz_target;
use libzstd_rs_sys_fuzz::{to_result, Error};

use std::ffi::*;

fn rs(compressed: &[u8]) -> (usize, Vec<u8>) {
    use libzstd_rs_sys::*;

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let mut decompressed_size: usize =
        unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) } as usize;

    if decompressed_size == 0 {
        return (decompressed_size, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_ERROR as usize {
        return (decompressed_size, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN as usize {
        return (decompressed_size, vec![]);
    }

    // Allocate buffer for decompressed output
    let margin =
        match to_result(unsafe { ZSTD_decompressionMargin(compressed_ptr, compressed_size) }) {
            Err(e) => return (e.to_error_code(), vec![]),
            Ok(v) => v,
        };
    decompressed_size = match decompressed_size.checked_add(margin) {
        Some(v) => v,
        None => return (0, vec![]),
    };
    decompressed_size = Ord::min(decompressed_size as usize, 1 << 20);
    decompressed_size = Ord::max(decompressed_size, compressed.len());
    let mut decompressed = vec![0u8; decompressed_size];

    decompressed[decompressed_size - compressed.len()..].copy_from_slice(compressed);

    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            decompressed
                .as_mut_ptr()
                .add(decompressed_size)
                .sub(compressed_size)
                .cast(),
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
    let mut decompressed_size: usize =
        unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) } as usize;

    if decompressed_size == 0 {
        return (decompressed_size, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_ERROR as usize {
        return (decompressed_size, vec![]);
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN as usize {
        return (decompressed_size, vec![]);
    }

    // Allocate buffer for decompressed output
    let margin =
        match to_result(unsafe { ZSTD_decompressionMargin(compressed_ptr, compressed_size) }) {
            Err(e) => return (e.to_error_code(), vec![]),
            Ok(v) => v,
        };
    decompressed_size = match decompressed_size.checked_add(margin) {
        Some(v) => v,
        None => return (0, vec![]),
    };
    decompressed_size = Ord::min(decompressed_size, 1 << 20);
    decompressed_size = Ord::max(decompressed_size, compressed.len());
    let mut decompressed = vec![0u8; decompressed_size];

    decompressed[decompressed_size - compressed.len()..].copy_from_slice(compressed);

    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            decompressed
                .as_mut_ptr()
                .add(decompressed_size)
                .sub(compressed_size)
                .cast(),
            compressed_size,
        )
    };

    (result as usize, decompressed)
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
