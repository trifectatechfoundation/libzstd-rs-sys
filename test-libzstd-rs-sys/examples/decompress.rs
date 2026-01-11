#![allow(deprecated)]
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

    match variant.as_str() {
        "c" => {
            let err = c(&input);
            assert_eq!(err, 0);
        }
        "rs" => {
            let err = rs(&input);
            assert_eq!(err, 0);
        }
        bad => panic!("invalid command {bad}"),
    }
}

fn rs(compressed: &[u8]) -> i32 {
    use libzstd_rs_sys::lib::{
        common::zstd_common::{ZSTD_getErrorName, ZSTD_isError},
        decompress::zstd_decompress::{ZSTD_decompress, ZSTD_getFrameContentSize},
        zstd::*,
    };

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size = unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) };
    if decompressed_size == ZSTD_CONTENTSIZE_ERROR {
        panic!("Not a valid zstd compressed frame!");
    } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN {
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

    0
}

fn c(compressed: &[u8]) -> i32 {
    use zstd_sys::*;

    let compressed_ptr = compressed.as_ptr() as *const c_void;
    let compressed_size = compressed.len();

    // Get decompressed size from frame header
    let decompressed_size = unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size) };
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
    if unsafe { ZSTD_isError(result) } != 0 {
        let err_msg = unsafe {
            let ptr = ZSTD_getErrorName(result);
            core::ffi::CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        };
        panic!("Decompression failed: {}", err_msg);
    }

    0
}
