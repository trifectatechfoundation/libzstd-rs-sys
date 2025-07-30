use crate::lib::compress::zstd_compress::{ZSTD_CCtx, ZSTD_CCtx_params_s, ZSTD_CCtx_s};
use crate::lib::decompress::{ZSTD_DCtx, ZSTD_DCtx_s};

type size_t = std::ffi::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Trace {
    pub version: std::ffi::c_uint,
    pub streaming: std::ffi::c_int,
    pub dictionaryID: std::ffi::c_uint,
    pub dictionaryIsCold: std::ffi::c_int,
    pub dictionarySize: size_t,
    pub uncompressedSize: size_t,
    pub compressedSize: size_t,
    pub params: *const ZSTD_CCtx_params_s,
    pub cctx: *const ZSTD_CCtx_s,
    pub dctx: *const ZSTD_DCtx_s,
}

pub type ZSTD_TraceCtx = std::ffi::c_ulonglong;

extern "C" {
    #[linkage = "weak"]
    pub fn ZSTD_trace_compress_begin(cctx: *const ZSTD_CCtx) -> ZSTD_TraceCtx;
    #[linkage = "weak"]
    pub fn ZSTD_trace_compress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace);
    #[linkage = "weak"]
    pub fn ZSTD_trace_decompress_begin(dctx: *const ZSTD_DCtx) -> ZSTD_TraceCtx;
    #[linkage = "weak"]
    pub fn ZSTD_trace_decompress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace);
}
