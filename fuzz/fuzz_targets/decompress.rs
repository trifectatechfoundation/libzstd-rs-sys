#![no_main]
#![allow(deprecated)]

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

/// Whether the C library's streaming decoder reports an error for `compressed`.
///
/// The zstd 1.5.7 behind `zstd-sys` rejects a block that decodes to more than the block size
/// maximum in streaming mode only. The Rust port rejects it in both modes, as zstd's dev branch does.
fn c_stream_errors(compressed: &[u8]) -> bool {
    use zstd_sys::*;

    unsafe {
        let dstream = ZSTD_createDStream();
        let mut input = ZSTD_inBuffer {
            src: compressed.as_ptr() as *const c_void,
            size: compressed.len(),
            pos: 0,
        };
        let mut out = vec![0u8; 1 << 17];
        let mut errored = false;
        loop {
            let mut output = ZSTD_outBuffer {
                dst: out.as_mut_ptr() as *mut c_void,
                size: out.len(),
                pos: 0,
            };
            let ret = ZSTD_decompressStream(dstream, &mut output, &mut input);
            if ZSTD_isError(ret) != 0 {
                errored = true;
                break;
            }
            if ret == 0 || (input.pos == input.size && output.pos == 0) {
                break;
            }
        }
        ZSTD_freeDStream(dstream);
        errored
    }
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

    if rs_err == Err(Error::corruption_detected) && c_err != rs_err && c_stream_errors(data) {
        return;
    }

    assert_eq!(rs_err, c_err);

    if rs_err.is_ok() {
        assert_eq!(rs_out, c_out);
    }
});
