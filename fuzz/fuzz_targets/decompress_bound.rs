#![no_main]

use libfuzzer_sys::fuzz_target;
use libzstd_rs_sys::*;
use libzstd_rs_sys_fuzz::to_result;

use std::ffi::*;

/// Whether `src` is a series of valid zstd and skippable frames.
///
/// The legacy decoders (v0.5 to v0.7) are frozen and their size queries do not bound what they
/// decode, so inputs with legacy frames are not checked.
fn is_current_format(mut src: &[u8]) -> bool {
    while let Some(magic) = src.first_chunk::<4>().map(|m| u32::from_le_bytes(*m)) {
        if magic != ZSTD_MAGICNUMBER
            && magic & ZSTD_MAGIC_SKIPPABLE_MASK != ZSTD_MAGIC_SKIPPABLE_START
        {
            return false;
        }
        let frame_size = unsafe { ZSTD_findFrameCompressedSize(src.as_ptr().cast(), src.len()) };
        if to_result(frame_size).is_err() {
            return false;
        }
        src = &src[frame_size..];
    }
    src.is_empty()
}

fuzz_target!(|data: &[u8]| {
    if !is_current_format(data) {
        return;
    }

    let ptr = data.as_ptr() as *const c_void;
    let size = unsafe { ZSTD_findDecompressedSize(ptr, data.len()) };
    let bound = unsafe { ZSTD_decompressBound(ptr, data.len()) };

    // When every frame declares its size the bound is that total, and a total that does not fit
    // is an error for both.
    if size != ZSTD_CONTENTSIZE_UNKNOWN {
        assert_eq!(bound, size);
    }

    // What decodes successfully must fit in the bound. Inputs that need more than the buffer
    // fail with `dstSize_tooSmall` and are skipped.
    let mut decompressed = vec![0u8; 1 << 20];
    let result = unsafe {
        ZSTD_decompress(
            decompressed.as_mut_ptr() as *mut c_void,
            decompressed.len(),
            ptr,
            data.len(),
        )
    };
    if let Ok(decompressed_size) = to_result(result) {
        assert!(decompressed_size as u64 <= bound);
    }
});
