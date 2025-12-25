use std::ffi::CStr;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::{Mutex, OnceLock};

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
static g_traceFile: Mutex<Option<File>> = Mutex::new(None);
static g_enableTime: OnceLock<UTIL_time_t> = OnceLock::new();

pub unsafe fn TRACE_enable(filename: *const core::ffi::c_char) {
    let writeHeader = core::ffi::c_int::from(UTIL_isRegularFile(filename) == 0);
    let mut traceFile = g_traceFile.lock().unwrap();
    assert!(traceFile.is_none());
    let traceFile = traceFile.insert(
        OpenOptions::new()
            .append(true)
            .open(CStr::from_ptr(filename).to_str().unwrap())
            .unwrap(),
    );
    if writeHeader != 0 {
        writeln!(
            traceFile,
            "Algorithm, Version, Method, Mode, Level, Workers, Dictionary Size, Uncompressed Size, Compressed Size, Duration Nanos, Compression Ratio, Speed MB/s"
        ).unwrap();
    }
    g_enableTime.set(UTIL_getTime()).map_err(|_| ()).unwrap();
}
pub unsafe fn TRACE_finish() {
    let _ = g_traceFile.lock().unwrap().take();
}
unsafe fn TRACE_log(method: &str, duration: PTime, trace: &ZSTD_Trace) {
    let mut level = 0;
    let mut workers = 0;
    let ratio =
        trace.uncompressedSize as core::ffi::c_double / trace.compressedSize as core::ffi::c_double;
    let speed =
        trace.uncompressedSize as core::ffi::c_double * 1000.0 / duration as core::ffi::c_double;
    if !(trace.params).is_null() {
        ZSTD_CCtxParams_getParameter(
            trace.params,
            ZSTD_cParameter::ZSTD_c_compressionLevel,
            &mut level,
        );
        ZSTD_CCtxParams_getParameter(
            trace.params,
            ZSTD_cParameter::ZSTD_c_nbWorkers,
            &mut workers,
        );
    }
    writeln!(
        g_traceFile.lock().unwrap().as_mut().unwrap(),
        "zstd, {}, {}, {}, {}, {}, {}, {}, {}, {}, {:.2}, {:.2}",
        trace.version,
        method,
        if trace.streaming != 0 {
            "streaming"
        } else {
            "single-pass"
        },
        level,
        workers,
        trace.dictionarySize as core::ffi::c_ulonglong,
        trace.uncompressedSize as core::ffi::c_ulonglong,
        trace.compressedSize as core::ffi::c_ulonglong,
        duration as core::ffi::c_ulonglong,
        ratio,
        speed,
    )
    .unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_compress_begin(_cctx: *const ZSTD_CCtx) -> ZSTD_TraceCtx {
    g_enableTime
        .get()
        .map_or(0, |&time| UTIL_clockSpanNano(time) as ZSTD_TraceCtx)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_compress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace) {
    let beginNanos = ctx as PTime;
    let endNanos = UTIL_clockSpanNano(*g_enableTime.get().unwrap());
    let durationNanos = if endNanos > beginNanos {
        endNanos.wrapping_sub(beginNanos)
    } else {
        0
    };
    TRACE_log("compress", durationNanos, &*trace);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_decompress_begin(_dctx: *const ZSTD_DCtx) -> ZSTD_TraceCtx {
    g_enableTime
        .get()
        .map_or(0, |&time| UTIL_clockSpanNano(time) as ZSTD_TraceCtx)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_decompress_end(ctx: ZSTD_TraceCtx, trace: *const ZSTD_Trace) {
    let beginNanos = ctx as PTime;
    let endNanos = UTIL_clockSpanNano(*g_enableTime.get().unwrap());
    let durationNanos = if endNanos > beginNanos {
        endNanos.wrapping_sub(beginNanos)
    } else {
        0
    };
    TRACE_log("decompress", durationNanos, &*trace);
}
