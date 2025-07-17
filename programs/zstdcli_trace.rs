extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_DCtx_s;
    pub type ZSTD_CCtx_params_s;
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn UTIL_getTime() -> UTIL_time_t;
    fn UTIL_clockSpanNano(clockStart: UTIL_time_t) -> PTime;
    fn UTIL_isRegularFile(infilename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn ZSTD_CCtxParams_getParameter(
        params: *const ZSTD_CCtx_params,
        param: ZSTD_cParameter,
        value: *mut std::ffi::c_int,
    ) -> size_t;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> std::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [std::ffi::c_char; 40],
    pub __align: std::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: std::ffi::c_int,
    pub __count: std::ffi::c_uint,
    pub __owner: std::ffi::c_int,
    pub __nusers: std::ffi::c_uint,
    pub __kind: std::ffi::c_int,
    pub __spins: std::ffi::c_short,
    pub __elision: std::ffi::c_short,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    pub _flags2: std::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type size_t = std::ffi::c_ulong;
pub type __off64_t = std::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = std::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [std::ffi::c_char; 4],
    pub __align: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
pub type PTime = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = std::ffi::c_ulong;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub type ZSTD_cParameter = std::ffi::c_uint;
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
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
static mut g_traceFile: *mut FILE = NULL as *mut FILE;
static mut g_mutexInit: std::ffi::c_int = 0 as std::ffi::c_int;
static mut g_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_internal_list {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
static mut g_enableTime: UTIL_time_t = {
    
    UTIL_time_t {
        t: 0 as std::ffi::c_int as PTime,
    }
};
#[no_mangle]
pub unsafe extern "C" fn TRACE_enable(mut filename: *const std::ffi::c_char) {
    let writeHeader = (UTIL_isRegularFile(filename) == 0) as std::ffi::c_int;
    if !g_traceFile.is_null() {
        fclose(g_traceFile);
    }
    g_traceFile = fopen(filename, b"a\0" as *const u8 as *const std::ffi::c_char);
    if !g_traceFile.is_null() && writeHeader != 0 {
        fprintf(
            g_traceFile,
            b"Algorithm, Version, Method, Mode, Level, Workers, Dictionary Size, Uncompressed Size, Compressed Size, Duration Nanos, Compression Ratio, Speed MB/s\n\0"
                as *const u8 as *const std::ffi::c_char,
        );
    }
    g_enableTime = UTIL_getTime();
    if g_mutexInit == 0 {
        if pthread_mutex_init(&mut g_mutex, std::ptr::null::<pthread_mutexattr_t>()) == 0 {
            g_mutexInit = 1 as std::ffi::c_int;
        } else {
            TRACE_finish();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn TRACE_finish() {
    if !g_traceFile.is_null() {
        fclose(g_traceFile);
    }
    g_traceFile = NULL as *mut FILE;
    if g_mutexInit != 0 {
        pthread_mutex_destroy(&mut g_mutex);
        g_mutexInit = 0 as std::ffi::c_int;
    }
}
unsafe extern "C" fn TRACE_log(
    mut method: *const std::ffi::c_char,
    mut duration: PTime,
    mut trace: *const ZSTD_Trace,
) {
    let mut level = 0 as std::ffi::c_int;
    let mut workers = 0 as std::ffi::c_int;
    let ratio = (*trace).uncompressedSize as std::ffi::c_double
        / (*trace).compressedSize as std::ffi::c_double;
    let speed = (*trace).uncompressedSize as std::ffi::c_double
        * 1000 as std::ffi::c_int as std::ffi::c_double
        / duration as std::ffi::c_double;
    if !((*trace).params).is_null() {
        ZSTD_CCtxParams_getParameter((*trace).params, ZSTD_c_compressionLevel, &mut level);
        ZSTD_CCtxParams_getParameter((*trace).params, ZSTD_c_nbWorkers, &mut workers);
    }
    pthread_mutex_lock(&mut g_mutex);
    fprintf(
        g_traceFile,
        b"zstd, %u, %s, %s, %d, %d, %llu, %llu, %llu, %llu, %.2f, %.2f\n\0" as *const u8
            as *const std::ffi::c_char,
        (*trace).version,
        method,
        if (*trace).streaming != 0 {
            b"streaming\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"single-pass\0" as *const u8 as *const std::ffi::c_char
        },
        level,
        workers,
        (*trace).dictionarySize as std::ffi::c_ulonglong,
        (*trace).uncompressedSize as std::ffi::c_ulonglong,
        (*trace).compressedSize as std::ffi::c_ulonglong,
        duration as std::ffi::c_ulonglong,
        ratio,
        speed,
    );
    pthread_mutex_unlock(&mut g_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_compress_begin(mut cctx: *const ZSTD_CCtx) -> ZSTD_TraceCtx {
    if g_traceFile.is_null() {
        return 0 as std::ffi::c_int as ZSTD_TraceCtx;
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
        0 as std::ffi::c_int as PTime
    };
    TRACE_log(
        b"compress\0" as *const u8 as *const std::ffi::c_char,
        durationNanos,
        trace,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_trace_decompress_begin(mut dctx: *const ZSTD_DCtx) -> ZSTD_TraceCtx {
    if g_traceFile.is_null() {
        return 0 as std::ffi::c_int as ZSTD_TraceCtx;
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
        0 as std::ffi::c_int as PTime
    };
    TRACE_log(
        b"decompress\0" as *const u8 as *const std::ffi::c_char,
        durationNanos,
        trace,
    );
}
