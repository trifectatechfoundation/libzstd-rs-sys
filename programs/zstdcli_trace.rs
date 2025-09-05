use std::sync::Mutex;

use libc::{fclose, fopen, fprintf, FILE};
use libzstd_rs_sys::lib::common::zstd_trace::{ZSTD_Trace, ZSTD_TraceCtx};
use libzstd_rs_sys::lib::compress::zstd_compress::{
    ZSTD_CCtxParams_getParameter, ZSTD_CCtx_params_s, ZSTD_CCtx_s,
};
use libzstd_rs_sys::lib::decompress::ZSTD_DCtx;
use libzstd_rs_sys::lib::zstd::ZSTD_cParameter;

use crate::timefn::{PTime, UTIL_clockSpanNano, UTIL_getTime, UTIL_time_t};
use crate::util::UTIL_isRegularFile;

pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
static mut g_traceFile: *mut FILE = core::ptr::null_mut();
static mut g_enableTime: UTIL_time_t = UTIL_time_t { t: 0 };

static WRITE_LOCK: Mutex<()> = Mutex::new(());

pub unsafe fn TRACE_enable(filename: *const core::ffi::c_char) {
    let writeHeader = (UTIL_isRegularFile(filename) == 0) as core::ffi::c_int;
    if !g_traceFile.is_null() {
        fclose(g_traceFile);
    }
    g_traceFile = fopen(filename, b"a\0" as *const u8 as *const core::ffi::c_char);
    if !g_traceFile.is_null() && writeHeader != 0 {
        fprintf(
            g_traceFile,
            b"Algorithm, Version, Method, Mode, Level, Workers, Dictionary Size, Uncompressed Size, Compressed Size, Duration Nanos, Compression Ratio, Speed MB/s\n\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    g_enableTime = UTIL_getTime();
}
pub unsafe fn TRACE_finish() {
    if !g_traceFile.is_null() {
        fclose(g_traceFile);
    }
    g_traceFile = core::ptr::null_mut();
}
unsafe fn TRACE_log(method: *const core::ffi::c_char, duration: PTime, trace: *const ZSTD_Trace) {
    let mut level = 0;
    let mut workers = 0;
    let ratio = (*trace).uncompressedSize as core::ffi::c_double
        / (*trace).compressedSize as core::ffi::c_double;
    let speed =
        (*trace).uncompressedSize as core::ffi::c_double * 1000.0 / duration as core::ffi::c_double;
    if !((*trace).params).is_null() {
        ZSTD_CCtxParams_getParameter(
            (*trace).params,
            ZSTD_cParameter::ZSTD_c_compressionLevel,
            &mut level,
        );
        ZSTD_CCtxParams_getParameter(
            (*trace).params,
            ZSTD_cParameter::ZSTD_c_nbWorkers,
            &mut workers,
        );
    }
    let _guard = WRITE_LOCK.lock().unwrap();
    fprintf(
        g_traceFile,
        b"zstd, %u, %s, %s, %d, %d, %llu, %llu, %llu, %llu, %.2f, %.2f\n\0" as *const u8
            as *const core::ffi::c_char,
        (*trace).version,
        method,
        if (*trace).streaming != 0 {
            b"streaming\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"single-pass\0" as *const u8 as *const core::ffi::c_char
        },
        level,
        workers,
        (*trace).dictionarySize as core::ffi::c_ulonglong,
        (*trace).uncompressedSize as core::ffi::c_ulonglong,
        (*trace).compressedSize as core::ffi::c_ulonglong,
        duration as core::ffi::c_ulonglong,
        ratio,
        speed,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_compress_begin(_cctx: *const ZSTD_CCtx) -> ZSTD_TraceCtx {
    if g_traceFile.is_null() {
        return 0;
    }
    UTIL_clockSpanNano(g_enableTime) as ZSTD_TraceCtx
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_compress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace) {
    let beginNanos = ctx as PTime;
    let endNanos = UTIL_clockSpanNano(g_enableTime);
    let durationNanos = if endNanos > beginNanos {
        endNanos.wrapping_sub(beginNanos)
    } else {
        0
    };
    TRACE_log(
        b"compress\0" as *const u8 as *const core::ffi::c_char,
        durationNanos,
        trace,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_decompress_begin(_dctx: *const ZSTD_DCtx) -> ZSTD_TraceCtx {
    if g_traceFile.is_null() {
        return 0;
    }
    UTIL_clockSpanNano(g_enableTime) as ZSTD_TraceCtx
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_decompress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace) {
    let beginNanos = ctx as PTime;
    let endNanos = UTIL_clockSpanNano(g_enableTime);
    let durationNanos = if endNanos > beginNanos {
        endNanos.wrapping_sub(beginNanos)
    } else {
        0
    };
    TRACE_log(
        b"decompress\0" as *const u8 as *const core::ffi::c_char,
        durationNanos,
        trace,
    );
}
