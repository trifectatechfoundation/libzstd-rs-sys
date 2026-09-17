#![no_main]

use libfuzzer_sys::{arbitrary, fuzz_target};
use libzstd_rs_sys_fuzz::to_result;

use std::ffi::*;

const MIN_CLEVEL: i32 = -5;
const MAX_CLEVEL: i32 = 19;

// ZSTD_fast..=ZSTD_btultra2
const MIN_STRATEGY: i32 = 1;
const MAX_STRATEGY: i32 = 9;

#[derive(Debug)]
struct ArbitraryLevel(i32);

impl arbitrary::Arbitrary<'_> for ArbitraryLevel {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(ArbitraryLevel(u.int_in_range(MIN_CLEVEL..=MAX_CLEVEL)?))
    }
}

#[derive(Debug)]
struct ArbitraryStrategy(i32);

impl arbitrary::Arbitrary<'_> for ArbitraryStrategy {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(ArbitraryStrategy(
            u.int_in_range(MIN_STRATEGY..=MAX_STRATEGY)?,
        ))
    }
}

fn rs(src: &[u8], level: i32, strategy: i32) -> (usize, Vec<u8>) {
    use libzstd_rs_sys::*;

    let bound = ZSTD_compressBound(src.len());
    let mut compressed = vec![0u8; bound];

    let cctx = unsafe { ZSTD_createCCtx() };
    assert!(!cctx.is_null());

    to_result(unsafe {
        ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_compressionLevel, level)
    })
    .unwrap();
    to_result(unsafe { ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_strategy, strategy) })
        .unwrap();

    let result = unsafe {
        ZSTD_compress2(
            cctx,
            compressed.as_mut_ptr() as *mut c_void,
            compressed.len(),
            src.as_ptr() as *const c_void,
            src.len(),
        )
    };
    unsafe { ZSTD_freeCCtx(cctx) };

    (result as usize, compressed)
}

fn c(src: &[u8], level: i32, strategy: i32) -> (usize, Vec<u8>) {
    use zstd_sys::*;

    let bound = unsafe { ZSTD_compressBound(src.len()) };
    let mut compressed = vec![0u8; bound];

    let cctx = unsafe { ZSTD_createCCtx() };
    assert!(!cctx.is_null());

    to_result(unsafe {
        ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_compressionLevel, level)
    })
    .unwrap();
    to_result(unsafe { ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_strategy, strategy) })
        .unwrap();

    let result = unsafe {
        ZSTD_compress2(
            cctx,
            compressed.as_mut_ptr() as *mut c_void,
            compressed.len(),
            src.as_ptr() as *const c_void,
            src.len(),
        )
    };
    unsafe { ZSTD_freeCCtx(cctx) };

    (result, compressed)
}

fuzz_target!(|input: (ArbitraryLevel, ArbitraryStrategy, Vec<u8>)| {
    let (ArbitraryLevel(level), ArbitraryStrategy(strategy), src) = input;

    let (c_result, mut c_out) = c(&src, level, strategy);
    let (rs_result, mut rs_out) = rs(&src, level, strategy);

    let rs_err = to_result(rs_result);
    let c_err = to_result(c_result);

    assert_eq!(rs_err, c_err);

    if let Ok(size) = rs_err {
        c_out.truncate(size);
        rs_out.truncate(size);

        assert_eq!(rs_out, c_out);
    };
});
