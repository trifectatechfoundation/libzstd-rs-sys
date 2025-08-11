use std::sync::Mutex;

use libc::{fclose, fopen, fprintf, FILE};
use libzstd_rs::lib::common::zstd_trace::{ZSTD_Trace, ZSTD_TraceCtx};
use libzstd_rs::lib::compress::zstd_compress::{
    ZSTD_CCtxParams_getParameter, ZSTD_CCtx_params_s, ZSTD_CCtx_s,
};
use libzstd_rs::lib::decompress::ZSTD_DCtx;

use crate::timefn::{PTime, UTIL_clockSpanNano, UTIL_getTime, UTIL_time_t};
use crate::util::UTIL_isRegularFile;

pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_cParameter = core::ffi::c_uint;
pub const ZSTD_c_experimentalParam20: ZSTD_cParameter = 1017;
pub const ZSTD_c_experimentalParam19: ZSTD_cParameter = 1016;
pub const ZSTD_c_experimentalParam18: ZSTD_cParameter = 1015;
pub const ZSTD_c_experimentalParam17: ZSTD_cParameter = 1014;
pub const ZSTD_c_experimentalParam16: ZSTD_cParameter = 1013;
pub const ZSTD_c_experimentalParam15: ZSTD_cParameter = 1012;
pub const ZSTD_c_experimentalParam14: ZSTD_cParameter = 1011;
pub const ZSTD_c_experimentalParam13: ZSTD_cParameter = 1010;
pub const ZSTD_c_experimentalParam12: ZSTD_cParameter = 1009;
pub const ZSTD_c_experimentalParam11: ZSTD_cParameter = 1008;
pub const ZSTD_c_experimentalParam10: ZSTD_cParameter = 1007;
pub const ZSTD_c_experimentalParam9: ZSTD_cParameter = 1006;
pub const ZSTD_c_experimentalParam8: ZSTD_cParameter = 1005;
pub const ZSTD_c_experimentalParam7: ZSTD_cParameter = 1004;
pub const ZSTD_c_experimentalParam5: ZSTD_cParameter = 1002;
pub const ZSTD_c_experimentalParam4: ZSTD_cParameter = 1001;
pub const ZSTD_c_experimentalParam3: ZSTD_cParameter = 1000;
pub const ZSTD_c_experimentalParam2: ZSTD_cParameter = 10;
pub const ZSTD_c_experimentalParam1: ZSTD_cParameter = 500;
pub const ZSTD_c_overlapLog: ZSTD_cParameter = 402;
pub const ZSTD_c_jobSize: ZSTD_cParameter = 401;
pub const ZSTD_c_nbWorkers: ZSTD_cParameter = 400;
pub const ZSTD_c_dictIDFlag: ZSTD_cParameter = 202;
pub const ZSTD_c_checksumFlag: ZSTD_cParameter = 201;
pub const ZSTD_c_contentSizeFlag: ZSTD_cParameter = 200;
pub const ZSTD_c_ldmHashRateLog: ZSTD_cParameter = 164;
pub const ZSTD_c_ldmBucketSizeLog: ZSTD_cParameter = 163;
pub const ZSTD_c_ldmMinMatch: ZSTD_cParameter = 162;
pub const ZSTD_c_ldmHashLog: ZSTD_cParameter = 161;
pub const ZSTD_c_enableLongDistanceMatching: ZSTD_cParameter = 160;
pub const ZSTD_c_targetCBlockSize: ZSTD_cParameter = 130;
pub const ZSTD_c_strategy: ZSTD_cParameter = 107;
pub const ZSTD_c_targetLength: ZSTD_cParameter = 106;
pub const ZSTD_c_minMatch: ZSTD_cParameter = 105;
pub const ZSTD_c_searchLog: ZSTD_cParameter = 104;
pub const ZSTD_c_chainLog: ZSTD_cParameter = 103;
pub const ZSTD_c_hashLog: ZSTD_cParameter = 102;
pub const ZSTD_c_windowLog: ZSTD_cParameter = 101;
pub const ZSTD_c_compressionLevel: ZSTD_cParameter = 100;
pub type ZSTD_CCtx_params = ZSTD_CCtx_params_s;
pub const NULL: core::ffi::c_int = 0;
static mut g_traceFile: *mut FILE = NULL as *mut FILE;
static mut g_enableTime: UTIL_time_t = UTIL_time_t { t: 0 };

static WRITE_LOCK: Mutex<()> = Mutex::new(());

#[no_mangle]
pub unsafe extern "C" fn TRACE_enable(mut filename: *const core::ffi::c_char) {
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
#[no_mangle]
pub unsafe extern "C" fn TRACE_finish() {
    if !g_traceFile.is_null() {
        fclose(g_traceFile);
    }
    g_traceFile = NULL as *mut FILE;
}
unsafe extern "C" fn TRACE_log(
    mut method: *const core::ffi::c_char,
    mut duration: PTime,
    mut trace: *const ZSTD_Trace,
) {
    let mut level = 0;
    let mut workers = 0;
    let ratio = (*trace).uncompressedSize as core::ffi::c_double
        / (*trace).compressedSize as core::ffi::c_double;
    let speed = (*trace).uncompressedSize as core::ffi::c_double
        * 1000 as core::ffi::c_int as core::ffi::c_double
        / duration as core::ffi::c_double;
    if !((*trace).params).is_null() {
        ZSTD_CCtxParams_getParameter((*trace).params, ZSTD_c_compressionLevel, &mut level);
        ZSTD_CCtxParams_getParameter((*trace).params, ZSTD_c_nbWorkers, &mut workers);
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
pub unsafe extern "C" fn ZSTD_trace_compress_end(
    mut ctx: ZSTD_TraceCtx,
    mut trace: *const ZSTD_Trace,
) {
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
pub unsafe extern "C" fn ZSTD_trace_decompress_end(
    mut ctx: ZSTD_TraceCtx,
    mut trace: *const ZSTD_Trace,
) {
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
