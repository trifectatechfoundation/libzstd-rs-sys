use core::ffi::c_void;
use std::borrow::Cow;

fn main() {
    let mut it = std::env::args();

    let _ = it.next().unwrap(); // exe
    let variant = it.next().unwrap();
    let level: i32 = it
        .next()
        .unwrap_or_else(|| panic!("missing compression level"))
        .parse()
        .unwrap_or_else(|_| panic!("compression level must be an integer"));
    let file = it.next();

    let silesia_small_tar = include_bytes!("../../silesia-small.tar");

    let input: Cow<'_, [u8]> = match file {
        None => silesia_small_tar.into(),
        Some(path) => std::fs::read(path).unwrap().into(),
    };

    const N: usize = 1;

    match variant.as_str() {
        "c" => {
            for _ in 0..N {
                let err = c(&input, level);
                assert_eq!(err, 0);
            }
        }
        "rs" => {
            for _ in 0..N {
                let err = rs(&input, level);
                assert_eq!(err, 0);
            }
        }
        bad => panic!("invalid command {bad}"),
    }
}

fn rs(input: &[u8], level: i32) -> i32 {
    use libzstd_rs_sys::{ZSTD_compress, ZSTD_compressBound, ZSTD_getErrorName, ZSTD_isError};

    let input_ptr = input.as_ptr() as *const c_void;
    let input_size = input.len();

    // Allocate output buffer using compress bound
    let bound = ZSTD_compressBound(input_size);
    let mut compressed = vec![0u8; bound];

    let result = unsafe {
        ZSTD_compress(
            compressed.as_mut_ptr() as *mut c_void,
            bound,
            input_ptr,
            input_size,
            level,
        )
    };

    if ZSTD_isError(result) != 0 {
        let err_msg = unsafe {
            let ptr = ZSTD_getErrorName(result);
            core::ffi::CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        };
        panic!("Compression failed: {}", err_msg);
    }

    0
}

fn c(input: &[u8], level: i32) -> i32 {
    use zstd_sys::{ZSTD_compress, ZSTD_compressBound, ZSTD_getErrorName, ZSTD_isError};

    let input_ptr = input.as_ptr() as *const c_void;
    let input_size = input.len();

    // Allocate output buffer using compress bound
    let bound = unsafe { ZSTD_compressBound(input_size) };
    let mut compressed = vec![0u8; bound];

    let result = unsafe {
        ZSTD_compress(
            compressed.as_mut_ptr() as *mut c_void,
            bound,
            input_ptr,
            input_size,
            level,
        )
    };

    if unsafe { ZSTD_isError(result) } != 0 {
        let err_msg = unsafe {
            let ptr = ZSTD_getErrorName(result);
            core::ffi::CStr::from_ptr(ptr)
                .to_string_lossy()
                .into_owned()
        };
        panic!("Compression failed: {}", err_msg);
    }

    0
}
