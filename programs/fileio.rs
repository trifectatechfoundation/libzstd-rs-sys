use crate::lib::compress::zstd_compress::{ZSTD_maxCLevel, ZSTD_minCLevel};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_DCtx_s;
    pub type POOL_ctx_s;
    pub type lzma_internal_s;
    pub type internal_state;
    fn close(__fd: std::ffi::c_int) -> std::ffi::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn fdopen(__fd: std::ffi::c_int, __modes: *const std::ffi::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut std::ffi::c_char,
        __modes: std::ffi::c_int,
        __n: size_t,
    ) -> std::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn fread(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: std::ffi::c_long,
        __whence: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> std::ffi::c_long;
    fn feof(__stream: *mut FILE) -> std::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> std::ffi::c_int;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn exit(_: std::ffi::c_int) -> !;
    fn qsort(
        __base: *mut std::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strcmp(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> std::ffi::c_int;
    fn strrchr(_: *const std::ffi::c_char, _: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn strerror(_: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn UTIL_requireUserConfirmation(
        prompt: *const std::ffi::c_char,
        abortMsg: *const std::ffi::c_char,
        acceptableLetters: *const std::ffi::c_char,
        hasStdinInput: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn UTIL_stat(filename: *const std::ffi::c_char, statbuf: *mut stat_t) -> std::ffi::c_int;
    fn UTIL_setFDStat(
        fd: std::ffi::c_int,
        filename: *const std::ffi::c_char,
        statbuf: *const stat_t,
    ) -> std::ffi::c_int;
    fn UTIL_utime(filename: *const std::ffi::c_char, statbuf: *const stat_t) -> std::ffi::c_int;
    fn UTIL_isRegularFileStat(statbuf: *const stat_t) -> std::ffi::c_int;
    fn UTIL_isDirectoryStat(statbuf: *const stat_t) -> std::ffi::c_int;
    fn UTIL_isFIFOStat(statbuf: *const stat_t) -> std::ffi::c_int;
    fn UTIL_isBlockDevStat(statbuf: *const stat_t) -> std::ffi::c_int;
    fn UTIL_getFileSizeStat(statbuf: *const stat_t) -> U64;
    fn UTIL_isFdRegularFile(fd: std::ffi::c_int) -> std::ffi::c_int;
    fn UTIL_isRegularFile(infilename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn UTIL_isDirectory(infilename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn UTIL_isSameFile(
        file1: *const std::ffi::c_char,
        file2: *const std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn UTIL_isSameFileStat(
        file1: *const std::ffi::c_char,
        file2: *const std::ffi::c_char,
        file1Stat: *const stat_t,
        file2Stat: *const stat_t,
    ) -> std::ffi::c_int;
    fn UTIL_isCompressedFile(
        infilename: *const std::ffi::c_char,
        extensionList: *mut *const std::ffi::c_char,
    ) -> std::ffi::c_int;
    fn UTIL_isFileDescriptorPipe(filename: *const std::ffi::c_char) -> std::ffi::c_int;
    fn UTIL_isConsole(file: *mut FILE) -> std::ffi::c_int;
    fn UTIL_getFileSize(infilename: *const std::ffi::c_char) -> U64;
    fn UTIL_makeHumanReadableSize(size: U64) -> UTIL_HumanReadableSize_t;
    fn UTIL_compareStr(p1: *const std::ffi::c_void, p2: *const std::ffi::c_void)
        -> std::ffi::c_int;
    fn UTIL_mirrorSourceFilesDirectories(
        fileNamesTable: *mut *const std::ffi::c_char,
        nbFiles: std::ffi::c_uint,
        outDirName: *const std::ffi::c_char,
    );
    fn UTIL_createMirroredDestDirName(
        srcFileName: *const std::ffi::c_char,
        outDirRootName: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn clock() -> clock_t;
    fn open(__file: *const std::ffi::c_char, __oflag: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn __assert_fail(
        __assertion: *const std::ffi::c_char,
        __file: *const std::ffi::c_char,
        __line: std::ffi::c_uint,
        __function: *const std::ffi::c_char,
    ) -> !;
    fn __errno_location() -> *mut std::ffi::c_int;
    fn signal(__sig: std::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn UTIL_getTime() -> UTIL_time_t;
    fn UTIL_clockSpanNano(clockStart: UTIL_time_t) -> PTime;
    fn UTIL_clockSpanMicro(clockStart: UTIL_time_t) -> PTime;
    fn ZSTD_getFrameContentSize(
        src: *const std::ffi::c_void,
        srcSize: size_t,
    ) -> std::ffi::c_ulonglong;
    fn ZSTD_isError(result: size_t) -> std::ffi::c_uint;
    fn ZSTD_getErrorCode(functionResult: size_t) -> ZSTD_ErrorCode;
    fn ZSTD_getErrorName(result: size_t) -> *const std::ffi::c_char;
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    fn ZSTD_CCtx_setParameter(
        cctx: *mut ZSTD_CCtx,
        param: ZSTD_cParameter,
        value: std::ffi::c_int,
    ) -> size_t;
    fn ZSTD_CCtx_setPledgedSrcSize(
        cctx: *mut ZSTD_CCtx,
        pledgedSrcSize: std::ffi::c_ulonglong,
    ) -> size_t;
    fn ZSTD_DCtx_setParameter(
        dctx: *mut ZSTD_DCtx,
        param: ZSTD_dParameter,
        value: std::ffi::c_int,
    ) -> size_t;
    fn ZSTD_DCtx_reset(dctx: *mut ZSTD_DCtx, reset: ZSTD_ResetDirective) -> size_t;
    fn ZSTD_freeCStream(zcs: *mut ZSTD_CStream) -> size_t;
    fn ZSTD_compressStream2(
        cctx: *mut ZSTD_CCtx,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
        endOp: ZSTD_EndDirective,
    ) -> size_t;
    fn ZSTD_CStreamInSize() -> size_t;
    fn ZSTD_CStreamOutSize() -> size_t;
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    fn ZSTD_decompressStream(
        zds: *mut ZSTD_DStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    fn ZSTD_DStreamInSize() -> size_t;
    fn ZSTD_DStreamOutSize() -> size_t;
    fn ZSTD_CCtx_refPrefix(
        cctx: *mut ZSTD_CCtx,
        prefix: *const std::ffi::c_void,
        prefixSize: size_t,
    ) -> size_t;
    fn ZSTD_DCtx_refPrefix(
        dctx: *mut ZSTD_DCtx,
        prefix: *const std::ffi::c_void,
        prefixSize: size_t,
    ) -> size_t;
    fn ZSTD_frameHeaderSize(src: *const std::ffi::c_void, srcSize: size_t) -> size_t;
    fn ZSTD_getFrameHeader(
        zfhPtr: *mut ZSTD_FrameHeader,
        src: *const std::ffi::c_void,
        srcSize: size_t,
    ) -> size_t;
    fn ZSTD_getCParams(
        compressionLevel: std::ffi::c_int,
        estimatedSrcSize: std::ffi::c_ulonglong,
        dictSize: size_t,
    ) -> ZSTD_compressionParameters;
    fn ZSTD_CCtx_loadDictionary_byReference(
        cctx: *mut ZSTD_CCtx,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTD_CCtx_getParameter(
        cctx: *const ZSTD_CCtx,
        param: ZSTD_cParameter,
        value: *mut std::ffi::c_int,
    ) -> size_t;
    fn ZSTD_isFrame(buffer: *const std::ffi::c_void, size: size_t) -> std::ffi::c_uint;
    fn ZSTD_DCtx_loadDictionary_byReference(
        dctx: *mut ZSTD_DCtx,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
    fn ZSTD_DCtx_setMaxWindowSize(dctx: *mut ZSTD_DCtx, maxWindowSize: size_t) -> size_t;
    fn ZSTD_getFrameProgression(cctx: *const ZSTD_CCtx) -> ZSTD_frameProgression;
    fn ZSTD_toFlushNow(cctx: *mut ZSTD_CCtx) -> size_t;
    fn AIO_supported() -> std::ffi::c_int;
    fn AIO_WritePool_releaseIoJob(job: *mut IOJob_t);
    fn AIO_WritePool_acquireJob(ctx: *mut WritePoolCtx_t) -> *mut IOJob_t;
    fn AIO_WritePool_enqueueAndReacquireWriteJob(job: *mut *mut IOJob_t);
    fn AIO_WritePool_sparseWriteEnd(ctx: *mut WritePoolCtx_t);
    fn AIO_WritePool_setFile(ctx: *mut WritePoolCtx_t, file: *mut FILE);
    fn AIO_WritePool_getFile(ctx: *const WritePoolCtx_t) -> *mut FILE;
    fn AIO_WritePool_closeFile(ctx: *mut WritePoolCtx_t) -> std::ffi::c_int;
    fn AIO_WritePool_create(prefs: *const FIO_prefs_t, bufferSize: size_t) -> *mut WritePoolCtx_t;
    fn AIO_WritePool_free(ctx: *mut WritePoolCtx_t);
    fn AIO_WritePool_setAsync(ctx: *mut WritePoolCtx_t, async_0: std::ffi::c_int);
    fn AIO_ReadPool_create(prefs: *const FIO_prefs_t, bufferSize: size_t) -> *mut ReadPoolCtx_t;
    fn AIO_ReadPool_free(ctx: *mut ReadPoolCtx_t);
    fn AIO_ReadPool_setAsync(ctx: *mut ReadPoolCtx_t, async_0: std::ffi::c_int);
    fn AIO_ReadPool_consumeBytes(ctx: *mut ReadPoolCtx_t, n: size_t);
    fn AIO_ReadPool_fillBuffer(ctx: *mut ReadPoolCtx_t, n: size_t) -> size_t;
    fn AIO_ReadPool_consumeAndRefill(ctx: *mut ReadPoolCtx_t) -> size_t;
    fn AIO_ReadPool_setFile(ctx: *mut ReadPoolCtx_t, file: *mut FILE);
    fn AIO_ReadPool_getFile(ctx: *const ReadPoolCtx_t) -> *mut FILE;
    fn AIO_ReadPool_closeFile(ctx: *mut ReadPoolCtx_t) -> std::ffi::c_int;
    fn zlibVersion() -> *const std::ffi::c_char;
    fn deflate(strm: z_streamp, flush: std::ffi::c_int) -> std::ffi::c_int;
    fn deflateEnd(strm: z_streamp) -> std::ffi::c_int;
    fn inflate(strm: z_streamp, flush: std::ffi::c_int) -> std::ffi::c_int;
    fn inflateEnd(strm: z_streamp) -> std::ffi::c_int;
    fn deflateInit2_(
        strm: z_streamp,
        level: std::ffi::c_int,
        method: std::ffi::c_int,
        windowBits: std::ffi::c_int,
        memLevel: std::ffi::c_int,
        strategy: std::ffi::c_int,
        version: *const std::ffi::c_char,
        stream_size: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: std::ffi::c_int,
        version: *const std::ffi::c_char,
        stream_size: std::ffi::c_int,
    ) -> std::ffi::c_int;
    fn lzma_version_string() -> *const std::ffi::c_char;
    fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_lzma_preset(options: *mut lzma_options_lzma, preset: uint32_t) -> lzma_bool;
    fn lzma_easy_encoder(strm: *mut lzma_stream, preset: uint32_t, check: lzma_check) -> lzma_ret;
    fn lzma_alone_encoder(strm: *mut lzma_stream, options: *const lzma_options_lzma) -> lzma_ret;
    fn lzma_stream_decoder(strm: *mut lzma_stream, memlimit: uint64_t, flags: uint32_t)
        -> lzma_ret;
    fn lzma_alone_decoder(strm: *mut lzma_stream, memlimit: uint64_t) -> lzma_ret;
    fn mmap(
        __addr: *mut std::ffi::c_void,
        __len: size_t,
        __prot: std::ffi::c_int,
        __flags: std::ffi::c_int,
        __fd: std::ffi::c_int,
        __offset: __off_t,
    ) -> *mut std::ffi::c_void;
    fn munmap(__addr: *mut std::ffi::c_void, __len: size_t) -> std::ffi::c_int;
}
pub type __uint8_t = std::ffi::c_uchar;
pub type __uint16_t = std::ffi::c_ushort;
pub type __uint32_t = std::ffi::c_uint;
pub type __uint64_t = std::ffi::c_ulong;
pub type __dev_t = std::ffi::c_ulong;
pub type __uid_t = std::ffi::c_uint;
pub type __gid_t = std::ffi::c_uint;
pub type __ino_t = std::ffi::c_ulong;
pub type __mode_t = std::ffi::c_uint;
pub type __nlink_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __clock_t = std::ffi::c_long;
pub type __time_t = std::ffi::c_long;
pub type __blksize_t = std::ffi::c_long;
pub type __blkcnt_t = std::ffi::c_long;
pub type __syscall_slong_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
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
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: std::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: std::ffi::c_uint,
    pub __high: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [std::ffi::c_uint; 2],
    pub __g_size: [std::ffi::c_uint; 2],
    pub __g1_orig_size: std::ffi::c_uint,
    pub __wrefs: std::ffi::c_uint,
    pub __g_signals: [std::ffi::c_uint; 2],
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
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [std::ffi::c_char; 48],
    pub __align: std::ffi::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: std::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const std::ffi::c_void, *const std::ffi::c_void) -> std::ffi::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U8 = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type stat_t = stat;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_HumanReadableSize_t {
    pub value: std::ffi::c_double,
    pub precision: std::ffi::c_int,
    pub suffix: *const std::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileNamesTable {
    pub fileNames: *mut *const std::ffi::c_char,
    pub buf: *mut std::ffi::c_char,
    pub tableSize: size_t,
    pub tableCapacity: size_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
pub type PTime = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
pub type ZSTD_ErrorCode = std::ffi::c_uint;
pub const ZSTD_error_maxCode: ZSTD_ErrorCode = 120;
pub const ZSTD_error_externalSequences_invalid: ZSTD_ErrorCode = 107;
pub const ZSTD_error_sequenceProducer_failed: ZSTD_ErrorCode = 106;
pub const ZSTD_error_srcBuffer_wrong: ZSTD_ErrorCode = 105;
pub const ZSTD_error_dstBuffer_wrong: ZSTD_ErrorCode = 104;
pub const ZSTD_error_seekableIO: ZSTD_ErrorCode = 102;
pub const ZSTD_error_frameIndex_tooLarge: ZSTD_ErrorCode = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ZSTD_ErrorCode = 82;
pub const ZSTD_error_noForwardProgress_destFull: ZSTD_ErrorCode = 80;
pub const ZSTD_error_dstBuffer_null: ZSTD_ErrorCode = 74;
pub const ZSTD_error_srcSize_wrong: ZSTD_ErrorCode = 72;
pub const ZSTD_error_dstSize_tooSmall: ZSTD_ErrorCode = 70;
pub const ZSTD_error_workSpace_tooSmall: ZSTD_ErrorCode = 66;
pub const ZSTD_error_memory_allocation: ZSTD_ErrorCode = 64;
pub const ZSTD_error_init_missing: ZSTD_ErrorCode = 62;
pub const ZSTD_error_stage_wrong: ZSTD_ErrorCode = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ZSTD_ErrorCode = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: ZSTD_ErrorCode = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: ZSTD_ErrorCode = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: ZSTD_ErrorCode = 46;
pub const ZSTD_error_tableLog_tooLarge: ZSTD_ErrorCode = 44;
pub const ZSTD_error_parameter_outOfBound: ZSTD_ErrorCode = 42;
pub const ZSTD_error_parameter_combination_unsupported: ZSTD_ErrorCode = 41;
pub const ZSTD_error_parameter_unsupported: ZSTD_ErrorCode = 40;
pub const ZSTD_error_dictionaryCreation_failed: ZSTD_ErrorCode = 34;
pub const ZSTD_error_dictionary_wrong: ZSTD_ErrorCode = 32;
pub const ZSTD_error_dictionary_corrupted: ZSTD_ErrorCode = 30;
pub const ZSTD_error_literals_headerWrong: ZSTD_ErrorCode = 24;
pub const ZSTD_error_checksum_wrong: ZSTD_ErrorCode = 22;
pub const ZSTD_error_corruption_detected: ZSTD_ErrorCode = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: ZSTD_ErrorCode = 16;
pub const ZSTD_error_frameParameter_unsupported: ZSTD_ErrorCode = 14;
pub const ZSTD_error_version_unsupported: ZSTD_ErrorCode = 12;
pub const ZSTD_error_prefix_unknown: ZSTD_ErrorCode = 10;
pub const ZSTD_error_GENERIC: ZSTD_ErrorCode = 1;
pub const ZSTD_error_no_error: ZSTD_ErrorCode = 0;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub type ZSTD_strategy = std::ffi::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
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
pub type ZSTD_ResetDirective = std::ffi::c_uint;
pub const ZSTD_reset_session_and_parameters: ZSTD_ResetDirective = 3;
pub const ZSTD_reset_parameters: ZSTD_ResetDirective = 2;
pub const ZSTD_reset_session_only: ZSTD_ResetDirective = 1;
pub type ZSTD_dParameter = std::ffi::c_uint;
pub const ZSTD_d_experimentalParam6: ZSTD_dParameter = 1005;
pub const ZSTD_d_experimentalParam5: ZSTD_dParameter = 1004;
pub const ZSTD_d_experimentalParam4: ZSTD_dParameter = 1003;
pub const ZSTD_d_experimentalParam3: ZSTD_dParameter = 1002;
pub const ZSTD_d_experimentalParam2: ZSTD_dParameter = 1001;
pub const ZSTD_d_experimentalParam1: ZSTD_dParameter = 1000;
pub const ZSTD_d_windowLogMax: ZSTD_dParameter = 100;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const std::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut std::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_CStream = ZSTD_CCtx;
pub type ZSTD_EndDirective = std::ffi::c_uint;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type ZSTD_DStream = ZSTD_DCtx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: std::ffi::c_uint,
    pub chainLog: std::ffi::c_uint,
    pub hashLog: std::ffi::c_uint,
    pub searchLog: std::ffi::c_uint,
    pub minMatch: std::ffi::c_uint,
    pub targetLength: std::ffi::c_uint,
    pub strategy: ZSTD_strategy,
}
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const ZSTD_f_zstd1_magicless: C2RustUnnamed_0 = 1;
pub const ZSTD_f_zstd1: C2RustUnnamed_0 = 0;
pub type ZSTD_ParamSwitch_e = std::ffi::c_uint;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
pub type ZSTD_FrameType_e = std::ffi::c_uint;
pub const ZSTD_skippableFrame: ZSTD_FrameType_e = 1;
pub const ZSTD_frame: ZSTD_FrameType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_FrameHeader {
    pub frameContentSize: std::ffi::c_ulonglong,
    pub windowSize: std::ffi::c_ulonglong,
    pub blockSizeMax: std::ffi::c_uint,
    pub frameType: ZSTD_FrameType_e,
    pub headerSize: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
    pub checksumFlag: std::ffi::c_uint,
    pub _reserved1: std::ffi::c_uint,
    pub _reserved2: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameProgression {
    pub ingested: std::ffi::c_ulonglong,
    pub consumed: std::ffi::c_ulonglong,
    pub produced: std::ffi::c_ulonglong,
    pub flushed: std::ffi::c_ulonglong,
    pub currentJobID: std::ffi::c_uint,
    pub nbActiveWorkers: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_display_prefs_s {
    pub displayLevel: std::ffi::c_int,
    pub progressSetting: FIO_progressSetting_e,
}
pub type FIO_progressSetting_e = std::ffi::c_uint;
pub const FIO_ps_always: FIO_progressSetting_e = 2;
pub const FIO_ps_never: FIO_progressSetting_e = 1;
pub const FIO_ps_auto: FIO_progressSetting_e = 0;
pub type FIO_display_prefs_t = FIO_display_prefs_s;
pub type FIO_compressionType_t = std::ffi::c_uint;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_prefs_s {
    pub compressionType: FIO_compressionType_t,
    pub sparseFileSupport: std::ffi::c_int,
    pub dictIDFlag: std::ffi::c_int,
    pub checksumFlag: std::ffi::c_int,
    pub jobSize: std::ffi::c_int,
    pub overlapLog: std::ffi::c_int,
    pub adaptiveMode: std::ffi::c_int,
    pub useRowMatchFinder: std::ffi::c_int,
    pub rsyncable: std::ffi::c_int,
    pub minAdaptLevel: std::ffi::c_int,
    pub maxAdaptLevel: std::ffi::c_int,
    pub ldmFlag: std::ffi::c_int,
    pub ldmHashLog: std::ffi::c_int,
    pub ldmMinMatch: std::ffi::c_int,
    pub ldmBucketSizeLog: std::ffi::c_int,
    pub ldmHashRateLog: std::ffi::c_int,
    pub streamSrcSize: size_t,
    pub targetCBlockSize: size_t,
    pub srcSizeHint: std::ffi::c_int,
    pub testMode: std::ffi::c_int,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
    pub removeSrcFile: std::ffi::c_int,
    pub overwrite: std::ffi::c_int,
    pub asyncIO: std::ffi::c_int,
    pub memLimit: std::ffi::c_uint,
    pub nbWorkers: std::ffi::c_int,
    pub excludeCompressedFiles: std::ffi::c_int,
    pub patchFromMode: std::ffi::c_int,
    pub contentSize: std::ffi::c_int,
    pub allowBlockDevices: std::ffi::c_int,
    pub passThrough: std::ffi::c_int,
    pub mmapDict: ZSTD_ParamSwitch_e,
}
pub type FIO_prefs_t = FIO_prefs_s;
pub type FIO_dictBufferType_t = std::ffi::c_uint;
pub const FIO_mmapDict: FIO_dictBufferType_t = 1;
pub const FIO_mallocDict: FIO_dictBufferType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_Dict_t {
    pub dictBuffer: *mut std::ffi::c_void,
    pub dictBufferSize: size_t,
    pub dictBufferType: FIO_dictBufferType_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_ctx_s {
    pub nbFilesTotal: std::ffi::c_int,
    pub hasStdinInput: std::ffi::c_int,
    pub hasStdoutOutput: std::ffi::c_int,
    pub currFileIdx: std::ffi::c_int,
    pub nbFilesProcessed: std::ffi::c_int,
    pub totalBytesInput: size_t,
    pub totalBytesOutput: size_t,
}
pub type FIO_ctx_t = FIO_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cRess_t {
    pub dict: FIO_Dict_t,
    pub dictFileName: *const std::ffi::c_char,
    pub dictFileStat: stat_t,
    pub cctx: *mut ZSTD_CStream,
    pub writeCtx: *mut WritePoolCtx_t,
    pub readCtx: *mut ReadPoolCtx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReadPoolCtx_t {
    pub base: IOPoolCtx_t,
    pub reachedEof: std::ffi::c_int,
    pub nextReadOffset: U64,
    pub waitingOnOffset: U64,
    pub currentJobHeld: *mut std::ffi::c_void,
    pub coalesceBuffer: *mut U8,
    pub srcBuffer: *mut U8,
    pub srcBufferLoaded: size_t,
    pub completedJobs: [*mut std::ffi::c_void; 10],
    pub completedJobsCount: std::ffi::c_int,
    pub jobCompletedCond: pthread_cond_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOPoolCtx_t {
    pub threadPool: *mut POOL_ctx,
    pub threadPoolActive: std::ffi::c_int,
    pub totalIoJobs: std::ffi::c_int,
    pub prefs: *const FIO_prefs_t,
    pub poolFunction: POOL_function,
    pub file: *mut FILE,
    pub ioJobsMutex: pthread_mutex_t,
    pub availableJobs: [*mut std::ffi::c_void; 10],
    pub availableJobsCount: std::ffi::c_int,
    pub jobBufferSize: size_t,
}
pub type POOL_function = Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>;
pub type POOL_ctx = POOL_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WritePoolCtx_t {
    pub base: IOPoolCtx_t,
    pub storedSkips: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOJob_t {
    pub ctx: *mut std::ffi::c_void,
    pub file: *mut FILE,
    pub buffer: *mut std::ffi::c_void,
    pub bufferSize: size_t,
    pub usedBufferSize: size_t,
    pub offset: U64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
    pub total_in: uint64_t,
    pub next_out: *mut uint8_t,
    pub avail_out: size_t,
    pub total_out: uint64_t,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut std::ffi::c_void,
    pub reserved_ptr2: *mut std::ffi::c_void,
    pub reserved_ptr3: *mut std::ffi::c_void,
    pub reserved_ptr4: *mut std::ffi::c_void,
    pub seek_pos: uint64_t,
    pub reserved_int2: uint64_t,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
}
pub type lzma_reserved_enum = std::ffi::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut std::ffi::c_void, size_t, size_t) -> *mut std::ffi::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> ()>,
    pub opaque: *mut std::ffi::c_void,
}
pub const LZMA_STREAM_END: lzma_ret = 1;
pub type lzma_ret = std::ffi::c_uint;
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_OK: lzma_ret = 0;
pub type lzma_action = std::ffi::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
pub type lzma_check = std::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
    pub lc: uint32_t,
    pub lp: uint32_t,
    pub pb: uint32_t,
    pub mode: lzma_mode,
    pub nice_len: uint32_t,
    pub mf: lzma_match_finder,
    pub depth: uint32_t,
    pub ext_flags: uint32_t,
    pub ext_size_low: uint32_t,
    pub ext_size_high: uint32_t,
    pub reserved_int4: uint32_t,
    pub reserved_int5: uint32_t,
    pub reserved_int6: uint32_t,
    pub reserved_int7: uint32_t,
    pub reserved_int8: uint32_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut std::ffi::c_void,
    pub reserved_ptr2: *mut std::ffi::c_void,
}
pub type lzma_match_finder = std::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub type lzma_mode = std::ffi::c_uint;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
pub type lzma_bool = std::ffi::c_uchar;
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut std::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: std::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type uLong = std::ffi::c_ulong;
pub type voidpf = *mut std::ffi::c_void;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type uInt = std::ffi::c_uint;
pub type Bytef = Byte;
pub type Byte = std::ffi::c_uchar;
pub type z_streamp = *mut z_stream;
pub type speedChange_e = std::ffi::c_uint;
pub const faster: speedChange_e = 2;
pub const slower: speedChange_e = 1;
pub const noChange: speedChange_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dRess_t {
    pub dict: FIO_Dict_t,
    pub dctx: *mut ZSTD_DStream,
    pub writeCtx: *mut WritePoolCtx_t,
    pub readCtx: *mut ReadPoolCtx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileInfo_t {
    pub decompressedSize: U64,
    pub compressedSize: U64,
    pub windowSize: U64,
    pub numActualFrames: std::ffi::c_int,
    pub numSkippableFrames: std::ffi::c_int,
    pub decompUnavailable: std::ffi::c_int,
    pub usesCheck: std::ffi::c_int,
    pub checksum: [BYTE; 4],
    pub nbFiles: U32,
    pub dictID: std::ffi::c_uint,
}
pub type InfoError = std::ffi::c_uint;
pub const info_truncated_input: InfoError = 4;
pub const info_file_error: InfoError = 3;
pub const info_not_zstd: InfoError = 2;
pub const info_frame_error: InfoError = 1;
pub const info_success: InfoError = 0;
pub const _IOFBF: std::ffi::c_int = 0 as std::ffi::c_int;
pub const UINT64_MAX: std::ffi::c_ulong = 18446744073709551615 as std::ffi::c_ulong;
use crate::{MEM_readLE24, MEM_readLE32};
pub const PATH_SEP: std::ffi::c_int = '/' as i32;
pub const UTIL_FILESIZE_UNKNOWN: std::ffi::c_int = -(1 as std::ffi::c_int);
pub const S_IRUSR: std::ffi::c_int = __S_IREAD;
pub const S_IWUSR: std::ffi::c_int = __S_IWRITE;
pub const S_IRGRP: std::ffi::c_int = S_IRUSR >> 3 as std::ffi::c_int;
pub const S_IWGRP: std::ffi::c_int = S_IWUSR >> 3 as std::ffi::c_int;
pub const S_IROTH: std::ffi::c_int = S_IRGRP >> 3 as std::ffi::c_int;
pub const S_IWOTH: std::ffi::c_int = S_IWGRP >> 3 as std::ffi::c_int;
pub const SEC_TO_MICRO: std::ffi::c_int = 1000000 as std::ffi::c_int;
pub const ZSTD_MAGICNUMBER: std::ffi::c_uint = 0xfd2fb528 as std::ffi::c_uint;
pub const ZSTD_MAGIC_SKIPPABLE_START: std::ffi::c_int = 0x184d2a50 as std::ffi::c_int;
pub const ZSTD_MAGIC_SKIPPABLE_MASK: std::ffi::c_uint = 0xfffffff0 as std::ffi::c_uint;
pub const ZSTD_BLOCKSIZELOG_MAX: std::ffi::c_int = 17 as std::ffi::c_int;
pub const ZSTD_BLOCKSIZE_MAX: std::ffi::c_int = (1 as std::ffi::c_int) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_CONTENTSIZE_UNKNOWN: std::ffi::c_ulonglong =
    (0 as std::ffi::c_ulonglong).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong);
pub const ZSTD_CONTENTSIZE_ERROR: std::ffi::c_ulonglong =
    (0 as std::ffi::c_ulonglong).wrapping_sub(2 as std::ffi::c_int as std::ffi::c_ulonglong);
pub const ZSTD_FRAMEHEADERSIZE_MAX: std::ffi::c_int = 18 as std::ffi::c_int;
pub const ZSTD_WINDOWLOG_MAX_32: std::ffi::c_int = 30 as std::ffi::c_int;
pub const ZSTD_WINDOWLOG_MAX_64: std::ffi::c_int = 31 as std::ffi::c_int;
pub const ZSTD_WINDOWLOG_LIMIT_DEFAULT: std::ffi::c_int = 27 as std::ffi::c_int;
pub const stdinmark: [std::ffi::c_char; 10] =
    unsafe { *::core::mem::transmute::<&[u8; 10], &[std::ffi::c_char; 10]>(b"/*stdin*\\\0") };
pub const stdoutmark: [std::ffi::c_char; 11] =
    unsafe { *::core::mem::transmute::<&[u8; 11], &[std::ffi::c_char; 11]>(b"/*stdout*\\\0") };
pub const nulmark: [std::ffi::c_char; 10] =
    unsafe { *::core::mem::transmute::<&[u8; 10], &[std::ffi::c_char; 10]>(b"/dev/null\0") };
pub const LZMA_EXTENSION: [std::ffi::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[std::ffi::c_char; 6]>(b".lzma\0") };
pub const XZ_EXTENSION: [std::ffi::c_char; 4] =
    unsafe { *::core::mem::transmute::<&[u8; 4], &[std::ffi::c_char; 4]>(b".xz\0") };
pub const TXZ_EXTENSION: [std::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[std::ffi::c_char; 5]>(b".txz\0") };
pub const GZ_EXTENSION: [std::ffi::c_char; 4] =
    unsafe { *::core::mem::transmute::<&[u8; 4], &[std::ffi::c_char; 4]>(b".gz\0") };
pub const TGZ_EXTENSION: [std::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[std::ffi::c_char; 5]>(b".tgz\0") };
pub const ZSTD_EXTENSION: [std::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[std::ffi::c_char; 5]>(b".zst\0") };
pub const TZSTD_EXTENSION: [std::ffi::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[std::ffi::c_char; 6]>(b".tzst\0") };
pub const ZSTD_ALT_EXTENSION: [std::ffi::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[std::ffi::c_char; 6]>(b".zstd\0") };
pub const LZ4_EXTENSION: [std::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[std::ffi::c_char; 5]>(b".lz4\0") };
pub const TLZ4_EXTENSION: [std::ffi::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[std::ffi::c_char; 6]>(b".tlz4\0") };
#[no_mangle]
pub static mut g_display_prefs: FIO_display_prefs_t = {
    FIO_display_prefs_s {
        displayLevel: 2 as std::ffi::c_int,
        progressSetting: FIO_ps_auto,
    }
};
#[no_mangle]
pub static mut g_displayClock: UTIL_time_t = {
    UTIL_time_t {
        t: 0 as std::ffi::c_int as PTime,
    }
};
pub const ZLIB_VERSION: [std::ffi::c_char; 4] =
    unsafe { *::core::mem::transmute::<&[u8; 4], &[std::ffi::c_char; 4]>(b"1.3\0") };
pub const Z_NO_FLUSH: std::ffi::c_int = 0 as std::ffi::c_int;
pub const Z_FINISH: std::ffi::c_int = 4 as std::ffi::c_int;
pub const Z_OK: std::ffi::c_int = 0 as std::ffi::c_int;
pub const Z_STREAM_END: std::ffi::c_int = 1 as std::ffi::c_int;
pub const Z_BUF_ERROR: std::ffi::c_int = -(5 as std::ffi::c_int);
pub const Z_BEST_COMPRESSION: std::ffi::c_int = 9 as std::ffi::c_int;
pub const Z_NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const ZSTD_SPARSE_DEFAULT: std::ffi::c_int = 1 as std::ffi::c_int;
pub const __S_IREAD: std::ffi::c_int = 0o400 as std::ffi::c_int;
pub const __S_IWRITE: std::ffi::c_int = 0o200 as std::ffi::c_int;
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const CLOCKS_PER_SEC: std::ffi::c_int = 1000000 as std::ffi::c_int;
pub const O_RDONLY: std::ffi::c_int = 0 as std::ffi::c_int;
pub const O_WRONLY: std::ffi::c_int = 0o1 as std::ffi::c_int;
pub const O_CREAT: std::ffi::c_int = 0o100 as std::ffi::c_int;
pub const O_TRUNC: std::ffi::c_int = 0o1000 as std::ffi::c_int;
pub const SIG_DFL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const SIG_IGN: std::ffi::c_int = 1 as std::ffi::c_int;
pub const SIGINT: std::ffi::c_int = 2 as std::ffi::c_int;
pub const REFRESH_RATE: PTime = SEC_TO_MICRO as PTime / 6 as std::ffi::c_int as PTime;
pub const LONG_TELL: unsafe extern "C" fn(*mut FILE) -> std::ffi::c_long = ftell;
pub const LZ4_MAGICNUMBER: std::ffi::c_int = 0x184d2204 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn FIO_zlibVersion() -> *const std::ffi::c_char {
    zlibVersion()
}
#[no_mangle]
pub unsafe extern "C" fn FIO_lz4Version() -> *const std::ffi::c_char {
    b"Unsupported\0" as *const u8 as *const std::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn FIO_lzmaVersion() -> *const std::ffi::c_char {
    lzma_version_string()
}
pub const ADAPT_WINDOWLOG_DEFAULT: std::ffi::c_int = 23 as std::ffi::c_int;
pub const DICTSIZE_MAX: std::ffi::c_int =
    32 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int);
pub const DEFAULT_FILE_PERMISSIONS: std::ffi::c_int =
    S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
pub const TEMPORARY_FILE_PERMISSIONS: std::ffi::c_int = S_IRUSR | S_IWUSR;
static mut g_artefact: *const std::ffi::c_char = NULL as *const std::ffi::c_char;
unsafe extern "C" fn INThandler(mut sig: std::ffi::c_int) {
    if sig == 2 as std::ffi::c_int {
    } else {
        __assert_fail(
            b"sig==SIGINT\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            134 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                b"void INThandler(int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_28249: {
        if sig == 2 as std::ffi::c_int {
        } else {
            __assert_fail(
                b"sig==SIGINT\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                134 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                    b"void INThandler(int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    signal(
        sig,
        ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(SIG_IGN as libc::intptr_t),
    );
    if !g_artefact.is_null() {
        if UTIL_isRegularFile(g_artefact) != 0 {
        } else {
            __assert_fail(
                b"UTIL_isRegularFile(g_artefact)\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                139 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                    b"void INThandler(int)\0",
                ))
                .as_ptr(),
            );
        }
        'c_28198: {
            if UTIL_isRegularFile(g_artefact) != 0 {
            } else {
                __assert_fail(
                    b"UTIL_isRegularFile(g_artefact)\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    139 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                        b"void INThandler(int)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        remove(g_artefact);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
    exit(2 as std::ffi::c_int);
}
unsafe extern "C" fn addHandler(mut dstFileName: *const std::ffi::c_char) {
    if UTIL_isRegularFile(dstFileName) != 0 {
        g_artefact = dstFileName;
        signal(
            SIGINT,
            Some(INThandler as unsafe extern "C" fn(std::ffi::c_int) -> ()),
        );
    } else {
        g_artefact = NULL as *const std::ffi::c_char;
    };
}
unsafe extern "C" fn clearHandler() {
    if !g_artefact.is_null() {
        signal(
            SIGINT,
            ::core::mem::transmute::<libc::intptr_t, __sighandler_t>(SIG_DFL as libc::intptr_t),
        );
    }
    g_artefact = NULL as *const std::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_addAbortHandler() {}
unsafe extern "C" fn FIO_shouldDisplayFileSummary(mut fCtx: *const FIO_ctx_t) -> std::ffi::c_int {
    ((*fCtx).nbFilesTotal <= 1 as std::ffi::c_int
        || g_display_prefs.displayLevel >= 3 as std::ffi::c_int) as std::ffi::c_int
}
unsafe extern "C" fn FIO_shouldDisplayMultipleFileSummary(
    mut fCtx: *const FIO_ctx_t,
) -> std::ffi::c_int {
    let shouldDisplay = ((*fCtx).nbFilesProcessed >= 1 as std::ffi::c_int
        && (*fCtx).nbFilesTotal > 1 as std::ffi::c_int) as std::ffi::c_int;
    if shouldDisplay != 0
        || FIO_shouldDisplayFileSummary(fCtx) != 0
        || (*fCtx).nbFilesProcessed == 0 as std::ffi::c_int
    {
    } else {
        __assert_fail(
            b"shouldDisplay || FIO_shouldDisplayFileSummary(fCtx) || fCtx->nbFilesProcessed == 0\0"
                as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            265 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 60], &[std::ffi::c_char; 60]>(
                b"int FIO_shouldDisplayMultipleFileSummary(const FIO_ctx_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_37124: {
        if shouldDisplay != 0
            || FIO_shouldDisplayFileSummary(fCtx) != 0
            || (*fCtx).nbFilesProcessed == 0 as std::ffi::c_int
        {
        } else {
            __assert_fail(
                b"shouldDisplay || FIO_shouldDisplayFileSummary(fCtx) || fCtx->nbFilesProcessed == 0\0"
                    as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                265 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[std::ffi::c_char; 60],
                >(b"int FIO_shouldDisplayMultipleFileSummary(const FIO_ctx_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    shouldDisplay
}
pub const FIO_OVERLAP_LOG_NOTSET: std::ffi::c_int = 9999 as std::ffi::c_int;
pub const FIO_LDM_PARAM_NOTSET: std::ffi::c_int = 9999 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn FIO_createPreferences() -> *mut FIO_prefs_t {
    let ret =
        malloc(::core::mem::size_of::<FIO_prefs_t>() as std::ffi::c_ulong) as *mut FIO_prefs_t;
    if ret.is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                281 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                21 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Allocation error : not enough memory\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(21 as std::ffi::c_int);
    }
    (*ret).compressionType = FIO_zstdCompression;
    (*ret).overwrite = 0 as std::ffi::c_int;
    (*ret).sparseFileSupport = ZSTD_SPARSE_DEFAULT;
    (*ret).dictIDFlag = 1 as std::ffi::c_int;
    (*ret).checksumFlag = 1 as std::ffi::c_int;
    (*ret).removeSrcFile = 0 as std::ffi::c_int;
    (*ret).memLimit = 0 as std::ffi::c_int as std::ffi::c_uint;
    (*ret).nbWorkers = 1 as std::ffi::c_int;
    (*ret).jobSize = 0 as std::ffi::c_int;
    (*ret).overlapLog = FIO_OVERLAP_LOG_NOTSET;
    (*ret).adaptiveMode = 0 as std::ffi::c_int;
    (*ret).rsyncable = 0 as std::ffi::c_int;
    (*ret).minAdaptLevel = -(50 as std::ffi::c_int);
    (*ret).maxAdaptLevel = 22 as std::ffi::c_int;
    (*ret).ldmFlag = 0 as std::ffi::c_int;
    (*ret).ldmHashLog = 0 as std::ffi::c_int;
    (*ret).ldmMinMatch = 0 as std::ffi::c_int;
    (*ret).ldmBucketSizeLog = FIO_LDM_PARAM_NOTSET;
    (*ret).ldmHashRateLog = FIO_LDM_PARAM_NOTSET;
    (*ret).streamSrcSize = 0 as std::ffi::c_int as size_t;
    (*ret).targetCBlockSize = 0 as std::ffi::c_int as size_t;
    (*ret).srcSizeHint = 0 as std::ffi::c_int;
    (*ret).testMode = 0 as std::ffi::c_int;
    (*ret).literalCompressionMode = ZSTD_ps_auto;
    (*ret).excludeCompressedFiles = 0 as std::ffi::c_int;
    (*ret).allowBlockDevices = 0 as std::ffi::c_int;
    (*ret).asyncIO = AIO_supported();
    (*ret).passThrough = -(1 as std::ffi::c_int);
    ret
}
#[no_mangle]
pub unsafe extern "C" fn FIO_createContext() -> *mut FIO_ctx_t {
    let ret = malloc(::core::mem::size_of::<FIO_ctx_t>() as std::ffi::c_ulong) as *mut FIO_ctx_t;
    if ret.is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                317 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                21 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Allocation error : not enough memory\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(21 as std::ffi::c_int);
    }
    (*ret).currFileIdx = 0 as std::ffi::c_int;
    (*ret).hasStdinInput = 0 as std::ffi::c_int;
    (*ret).hasStdoutOutput = 0 as std::ffi::c_int;
    (*ret).nbFilesTotal = 1 as std::ffi::c_int;
    (*ret).nbFilesProcessed = 0 as std::ffi::c_int;
    (*ret).totalBytesInput = 0 as std::ffi::c_int as size_t;
    (*ret).totalBytesOutput = 0 as std::ffi::c_int as size_t;
    ret
}
#[no_mangle]
pub unsafe extern "C" fn FIO_freePreferences(prefs: *mut FIO_prefs_t) {
    free(prefs as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_freeContext(fCtx: *mut FIO_ctx_t) {
    free(fCtx as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNotificationLevel(mut level: std::ffi::c_int) {
    g_display_prefs.displayLevel = level;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setProgressSetting(mut setting: FIO_progressSetting_e) {
    g_display_prefs.progressSetting = setting;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setCompressionType(
    prefs: *mut FIO_prefs_t,
    mut compressionType: FIO_compressionType_t,
) {
    (*prefs).compressionType = compressionType;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_overwriteMode(prefs: *mut FIO_prefs_t) {
    (*prefs).overwrite = 1 as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setSparseWrite(prefs: *mut FIO_prefs_t, mut sparse: std::ffi::c_int) {
    (*prefs).sparseFileSupport = sparse;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setDictIDFlag(
    prefs: *mut FIO_prefs_t,
    mut dictIDFlag: std::ffi::c_int,
) {
    (*prefs).dictIDFlag = dictIDFlag;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setChecksumFlag(
    prefs: *mut FIO_prefs_t,
    mut checksumFlag: std::ffi::c_int,
) {
    (*prefs).checksumFlag = checksumFlag;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setRemoveSrcFile(prefs: *mut FIO_prefs_t, mut flag: std::ffi::c_int) {
    (*prefs).removeSrcFile = (flag != 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setMemLimit(prefs: *mut FIO_prefs_t, mut memLimit: std::ffi::c_uint) {
    (*prefs).memLimit = memLimit;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNbWorkers(prefs: *mut FIO_prefs_t, mut nbWorkers: std::ffi::c_int) {
    (*prefs).nbWorkers = nbWorkers;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setExcludeCompressedFile(
    prefs: *mut FIO_prefs_t,
    mut excludeCompressedFiles: std::ffi::c_int,
) {
    (*prefs).excludeCompressedFiles = excludeCompressedFiles;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAllowBlockDevices(
    prefs: *mut FIO_prefs_t,
    mut allowBlockDevices: std::ffi::c_int,
) {
    (*prefs).allowBlockDevices = allowBlockDevices;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setJobSize(prefs: *mut FIO_prefs_t, mut jobSize: std::ffi::c_int) {
    if jobSize != 0
        && (*prefs).nbWorkers == 0 as std::ffi::c_int
        && g_display_prefs.displayLevel >= 2 as std::ffi::c_int
    {
        fprintf(
            stderr,
            b"Setting block size is useless in single-thread mode \n\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    (*prefs).jobSize = jobSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setOverlapLog(
    prefs: *mut FIO_prefs_t,
    mut overlapLog: std::ffi::c_int,
) {
    if overlapLog != 0
        && (*prefs).nbWorkers == 0 as std::ffi::c_int
        && g_display_prefs.displayLevel >= 2 as std::ffi::c_int
    {
        fprintf(
            stderr,
            b"Setting overlapLog is useless in single-thread mode \n\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    (*prefs).overlapLog = overlapLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptiveMode(prefs: *mut FIO_prefs_t, mut adapt: std::ffi::c_int) {
    if adapt > 0 as std::ffi::c_int && (*prefs).nbWorkers == 0 as std::ffi::c_int {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                394 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                1 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Adaptive mode is not compatible with single thread mode \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(1 as std::ffi::c_int);
    }
    (*prefs).adaptiveMode = adapt;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setUseRowMatchFinder(
    prefs: *mut FIO_prefs_t,
    mut useRowMatchFinder: std::ffi::c_int,
) {
    (*prefs).useRowMatchFinder = useRowMatchFinder;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setRsyncable(prefs: *mut FIO_prefs_t, mut rsyncable: std::ffi::c_int) {
    if rsyncable > 0 as std::ffi::c_int && (*prefs).nbWorkers == 0 as std::ffi::c_int {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                404 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                1 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Rsyncable mode is not compatible with single thread mode \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(1 as std::ffi::c_int);
    }
    (*prefs).rsyncable = rsyncable;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setStreamSrcSize(prefs: *mut FIO_prefs_t, mut streamSrcSize: size_t) {
    (*prefs).streamSrcSize = streamSrcSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setTargetCBlockSize(
    prefs: *mut FIO_prefs_t,
    mut targetCBlockSize: size_t,
) {
    (*prefs).targetCBlockSize = targetCBlockSize;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setSrcSizeHint(prefs: *mut FIO_prefs_t, mut srcSizeHint: size_t) {
    (*prefs).srcSizeHint = (if (2147483647 as std::ffi::c_int as size_t) < srcSizeHint {
        2147483647 as std::ffi::c_int as size_t
    } else {
        srcSizeHint
    }) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setTestMode(prefs: *mut FIO_prefs_t, mut testMode: std::ffi::c_int) {
    (*prefs).testMode = (testMode != 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLiteralCompressionMode(
    prefs: *mut FIO_prefs_t,
    mut mode: ZSTD_ParamSwitch_e,
) {
    (*prefs).literalCompressionMode = mode;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptMin(prefs: *mut FIO_prefs_t, mut minCLevel: std::ffi::c_int) {
    if minCLevel >= ZSTD_minCLevel() {
    } else {
        __assert_fail(
            b"minCLevel >= ZSTD_minCLevel()\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            433 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 46], &[std::ffi::c_char; 46]>(
                b"void FIO_setAdaptMin(FIO_prefs_t *const, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_13520: {
        if minCLevel >= ZSTD_minCLevel() {
        } else {
            __assert_fail(
                b"minCLevel >= ZSTD_minCLevel()\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                433 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 46], &[std::ffi::c_char; 46]>(
                    b"void FIO_setAdaptMin(FIO_prefs_t *const, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    (*prefs).minAdaptLevel = minCLevel;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAdaptMax(prefs: *mut FIO_prefs_t, mut maxCLevel: std::ffi::c_int) {
    (*prefs).maxAdaptLevel = maxCLevel;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmFlag(prefs: *mut FIO_prefs_t, mut ldmFlag: std::ffi::c_uint) {
    (*prefs).ldmFlag = (ldmFlag > 0 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashLog(
    prefs: *mut FIO_prefs_t,
    mut ldmHashLog: std::ffi::c_int,
) {
    (*prefs).ldmHashLog = ldmHashLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmMinMatch(
    prefs: *mut FIO_prefs_t,
    mut ldmMinMatch: std::ffi::c_int,
) {
    (*prefs).ldmMinMatch = ldmMinMatch;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmBucketSizeLog(
    prefs: *mut FIO_prefs_t,
    mut ldmBucketSizeLog: std::ffi::c_int,
) {
    (*prefs).ldmBucketSizeLog = ldmBucketSizeLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setLdmHashRateLog(
    prefs: *mut FIO_prefs_t,
    mut ldmHashRateLog: std::ffi::c_int,
) {
    (*prefs).ldmHashRateLog = ldmHashRateLog;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setPatchFromMode(prefs: *mut FIO_prefs_t, mut value: std::ffi::c_int) {
    (*prefs).patchFromMode = (value != 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setContentSize(prefs: *mut FIO_prefs_t, mut value: std::ffi::c_int) {
    (*prefs).contentSize = (value != 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setAsyncIOFlag(prefs: *mut FIO_prefs_t, mut value: std::ffi::c_int) {
    (*prefs).asyncIO = value;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setPassThroughFlag(
    prefs: *mut FIO_prefs_t,
    mut value: std::ffi::c_int,
) {
    (*prefs).passThrough = (value != 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setMMapDict(prefs: *mut FIO_prefs_t, mut value: ZSTD_ParamSwitch_e) {
    (*prefs).mmapDict = value;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setHasStdoutOutput(fCtx: *mut FIO_ctx_t, mut value: std::ffi::c_int) {
    (*fCtx).hasStdoutOutput = value;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_setNbFilesTotal(fCtx: *mut FIO_ctx_t, mut value: std::ffi::c_int) {
    (*fCtx).nbFilesTotal = value;
}
#[no_mangle]
pub unsafe extern "C" fn FIO_determineHasStdinInput(
    fCtx: *mut FIO_ctx_t,
    filenames: *const FileNamesTable,
) {
    let mut i = 0 as std::ffi::c_int as size_t;
    while i < (*filenames).tableSize {
        if strcmp(
            stdinmark.as_ptr(),
            *((*filenames).fileNames).offset(i as isize),
        ) == 0
        {
            (*fCtx).hasStdinInput = 1 as std::ffi::c_int;
            return;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn FIO_removeFile(mut path: *const std::ffi::c_char) -> std::ffi::c_int {
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if UTIL_stat(path, &mut statbuf) == 0 {
        if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: Failed to stat %s while trying to remove it\n\0" as *const u8
                    as *const std::ffi::c_char,
                path,
            );
        }
        return 0 as std::ffi::c_int;
    }
    if UTIL_isRegularFileStat(&mut statbuf) == 0 {
        if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: Refusing to remove non-regular file %s\n\0" as *const u8
                    as *const std::ffi::c_char,
                path,
            );
        }
        return 0 as std::ffi::c_int;
    }
    remove(path)
}
unsafe extern "C" fn FIO_openSrcFile(
    prefs: *const FIO_prefs_t,
    mut srcFileName: *const std::ffi::c_char,
    mut statbuf: *mut stat_t,
) -> *mut FILE {
    let mut allowBlockDevices = if !prefs.is_null() {
        (*prefs).allowBlockDevices
    } else {
        0 as std::ffi::c_int
    };
    if !srcFileName.is_null() {
    } else {
        __assert_fail(
            b"srcFileName != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            547 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 72], &[std::ffi::c_char; 72]>(
                b"FILE *FIO_openSrcFile(const FIO_prefs_t *const, const char *, stat_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_29459: {
        if !srcFileName.is_null() {
        } else {
            __assert_fail(
                b"srcFileName != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                547 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 72], &[std::ffi::c_char; 72]>(
                    b"FILE *FIO_openSrcFile(const FIO_prefs_t *const, const char *, stat_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !statbuf.is_null() {
    } else {
        __assert_fail(
            b"statbuf != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            548 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 72], &[std::ffi::c_char; 72]>(
                b"FILE *FIO_openSrcFile(const FIO_prefs_t *const, const char *, stat_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_29417: {
        if !statbuf.is_null() {
        } else {
            __assert_fail(
                b"statbuf != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                548 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 72], &[std::ffi::c_char; 72]>(
                    b"FILE *FIO_openSrcFile(const FIO_prefs_t *const, const char *, stat_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Using stdin for input \n\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        return stdin;
    }
    if UTIL_stat(srcFileName, statbuf) == 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: can't stat %s : %s -- ignored \n\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
                strerror(*__errno_location()),
            );
        }
        return NULL as *mut FILE;
    }
    if UTIL_isRegularFileStat(statbuf) == 0
        && UTIL_isFIFOStat(statbuf) == 0
        && UTIL_isFileDescriptorPipe(srcFileName) == 0
        && !(allowBlockDevices != 0 && UTIL_isBlockDevStat(statbuf) != 0)
    {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s is not a regular file -- ignored \n\0" as *const u8
                    as *const std::ffi::c_char,
                srcFileName,
            );
        }
        return NULL as *mut FILE;
    }
    let f = fopen(srcFileName, b"rb\0" as *const u8 as *const std::ffi::c_char);
    if f.is_null() && g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
        fprintf(
            stderr,
            b"zstd: %s: %s \n\0" as *const u8 as *const std::ffi::c_char,
            srcFileName,
            strerror(*__errno_location()),
        );
    }
    f
}
unsafe extern "C" fn FIO_openDstFile(
    mut fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut srcFileName: *const std::ffi::c_char,
    mut dstFileName: *const std::ffi::c_char,
    mode: std::ffi::c_int,
) -> *mut FILE {
    if (*prefs).testMode != 0 {
        return NULL as *mut FILE;
    }
    if !dstFileName.is_null() {
    } else {
        __assert_fail(
            b"dstFileName != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            590 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 94],
                &[std::ffi::c_char; 94],
            >(
                b"FILE *FIO_openDstFile(FIO_ctx_t *, FIO_prefs_t *const, const char *, const char *, const int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_28879: {
        if !dstFileName.is_null() {
        } else {
            __assert_fail(
                b"dstFileName != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                590 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 94],
                    &[std::ffi::c_char; 94],
                >(
                    b"FILE *FIO_openDstFile(FIO_ctx_t *, FIO_prefs_t *const, const char *, const char *, const int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if strcmp(dstFileName, stdoutmark.as_ptr()) == 0 {
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Using stdout for output \n\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if (*prefs).sparseFileSupport == 1 as std::ffi::c_int {
            (*prefs).sparseFileSupport = 0 as std::ffi::c_int;
            if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Sparse File Support is automatically disabled on stdout ; try --sparse \n\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
        }
        return stdout;
    }
    if !srcFileName.is_null() && UTIL_isSameFile(srcFileName, dstFileName) != 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: Refusing to open an output file which will overwrite the input file \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        return NULL as *mut FILE;
    }
    if UTIL_isRegularFile(dstFileName) != 0 {
        if strcmp(dstFileName, nulmark.as_ptr()) == 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    614 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    40 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s is unexpectedly categorized as a regular file\0" as *const u8
                        as *const std::ffi::c_char,
                    dstFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(40 as std::ffi::c_int);
        }
        if (*prefs).overwrite == 0 {
            if g_display_prefs.displayLevel <= 1 as std::ffi::c_int {
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s already exists; not overwritten  \n\0" as *const u8
                            as *const std::ffi::c_char,
                        dstFileName,
                    );
                }
                return NULL as *mut FILE;
            }
            fprintf(
                stderr,
                b"zstd: %s already exists; \0" as *const u8 as *const std::ffi::c_char,
                dstFileName,
            );
            if UTIL_requireUserConfirmation(
                b"overwrite (y/n) ? \0" as *const u8 as *const std::ffi::c_char,
                b"Not overwritten  \n\0" as *const u8 as *const std::ffi::c_char,
                b"yY\0" as *const u8 as *const std::ffi::c_char,
                (*fCtx).hasStdinInput,
            ) != 0
            {
                return NULL as *mut FILE;
            }
        }
        FIO_removeFile(dstFileName);
    }
    let mut isDstRegFile: std::ffi::c_int = 0;
    let openflags = O_WRONLY | O_CREAT | O_TRUNC;
    let fd = open(dstFileName, openflags, mode);
    let mut f = NULL as *mut FILE;
    if fd != -(1 as std::ffi::c_int) {
        f = fdopen(fd, b"wb\0" as *const u8 as *const std::ffi::c_char);
    }
    isDstRegFile = UTIL_isFdRegularFile(fd);
    if (*prefs).sparseFileSupport == 1 as std::ffi::c_int {
        (*prefs).sparseFileSupport = ZSTD_SPARSE_DEFAULT;
        if isDstRegFile == 0 {
            (*prefs).sparseFileSupport = 0 as std::ffi::c_int;
            if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Sparse File Support is disabled when output is not a file \n\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
        }
    }
    if f.is_null() {
        if UTIL_isFileDescriptorPipe(dstFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: error: no output specified (use -o or -c). \n\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
        } else if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                dstFileName,
                strerror(*__errno_location()),
            );
        }
    } else if setvbuf(
        f,
        NULL as *mut std::ffi::c_char,
        _IOFBF,
        (1 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int)) as size_t,
    ) != 0
        && g_display_prefs.displayLevel >= 2 as std::ffi::c_int
    {
        fprintf(
            stderr,
            b"Warning: setvbuf failed for %s\n\0" as *const u8 as *const std::ffi::c_char,
            dstFileName,
        );
    }
    f
}
unsafe extern "C" fn FIO_getDictFileStat(
    mut fileName: *const std::ffi::c_char,
    mut dictFileStat: *mut stat_t,
) {
    if !dictFileStat.is_null() {
    } else {
        __assert_fail(
            b"dictFileStat != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            694 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 49], &[std::ffi::c_char; 49]>(
                b"void FIO_getDictFileStat(const char *, stat_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_21844: {
        if !dictFileStat.is_null() {
        } else {
            __assert_fail(
                b"dictFileStat != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                694 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 49], &[std::ffi::c_char; 49]>(
                    b"void FIO_getDictFileStat(const char *, stat_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if fileName.is_null() {
        return;
    }
    if UTIL_stat(fileName, dictFileStat) == 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                698 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                31 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Stat failed on dictionary file %s: %s\0" as *const u8 as *const std::ffi::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(31 as std::ffi::c_int);
    }
    if UTIL_isRegularFileStat(dictFileStat) == 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                702 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                32 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Dictionary %s must be a regular file.\0" as *const u8 as *const std::ffi::c_char,
                fileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(32 as std::ffi::c_int);
    }
}
unsafe extern "C" fn FIO_setDictBufferMalloc(
    mut dict: *mut FIO_Dict_t,
    mut fileName: *const std::ffi::c_char,
    prefs: *mut FIO_prefs_t,
    mut dictFileStat: *mut stat_t,
) -> size_t {
    let mut fileHandle = std::ptr::null_mut::<FILE>();
    let mut fileSize: U64 = 0;
    let mut bufferPtr: *mut *mut std::ffi::c_void = &mut (*dict).dictBuffer;
    if !bufferPtr.is_null() {
    } else {
        __assert_fail(
            b"bufferPtr != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            718 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 89],
                &[std::ffi::c_char; 89],
            >(
                b"size_t FIO_setDictBufferMalloc(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20583: {
        if !bufferPtr.is_null() {
        } else {
            __assert_fail(
                b"bufferPtr != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                718 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 89],
                    &[std::ffi::c_char; 89],
                >(
                    b"size_t FIO_setDictBufferMalloc(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !dictFileStat.is_null() {
    } else {
        __assert_fail(
            b"dictFileStat != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            719 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 89],
                &[std::ffi::c_char; 89],
            >(
                b"size_t FIO_setDictBufferMalloc(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20541: {
        if !dictFileStat.is_null() {
        } else {
            __assert_fail(
                b"dictFileStat != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                719 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 89],
                    &[std::ffi::c_char; 89],
                >(
                    b"size_t FIO_setDictBufferMalloc(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *bufferPtr = NULL as *mut std::ffi::c_void;
    if fileName.is_null() {
        return 0 as std::ffi::c_int as size_t;
    }
    if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Loading %s as dictionary \n\0" as *const u8 as *const std::ffi::c_char,
            fileName,
        );
    }
    fileHandle = fopen(fileName, b"rb\0" as *const u8 as *const std::ffi::c_char);
    if fileHandle.is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                728 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                33 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Couldn't open dictionary %s: %s\0" as *const u8 as *const std::ffi::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(33 as std::ffi::c_int);
    }
    fileSize = UTIL_getFileSizeStat(dictFileStat);
    let dictSizeMax = (if (*prefs).patchFromMode != 0 {
        (*prefs).memLimit
    } else {
        DICTSIZE_MAX as std::ffi::c_uint
    }) as size_t;
    if fileSize > dictSizeMax {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                736 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                34 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Dictionary file %s is too large (> %u bytes)\0" as *const u8
                    as *const std::ffi::c_char,
                fileName,
                dictSizeMax as std::ffi::c_uint,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(34 as std::ffi::c_int);
    }
    *bufferPtr = malloc(fileSize);
    if (*bufferPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                740 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                34 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(34 as std::ffi::c_int);
    }
    let readSize = fread(
        *bufferPtr,
        1 as std::ffi::c_int as std::ffi::c_ulong,
        fileSize,
        fileHandle,
    );
    if readSize != fileSize {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                744 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                35 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error reading dictionary file %s : %s\0" as *const u8 as *const std::ffi::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(35 as std::ffi::c_int);
    }
    fclose(fileHandle);
    fileSize
}
pub const PROT_READ: std::ffi::c_int = 0x1 as std::ffi::c_int;
pub const MAP_PRIVATE: std::ffi::c_int = 0x2 as std::ffi::c_int;
unsafe extern "C" fn FIO_munmap(mut dict: *mut FIO_Dict_t) {
    munmap((*dict).dictBuffer, (*dict).dictBufferSize);
    (*dict).dictBuffer = NULL as *mut std::ffi::c_void;
    (*dict).dictBufferSize = 0 as std::ffi::c_int as size_t;
}
unsafe extern "C" fn FIO_setDictBufferMMap(
    mut dict: *mut FIO_Dict_t,
    mut fileName: *const std::ffi::c_char,
    prefs: *mut FIO_prefs_t,
    mut dictFileStat: *mut stat_t,
) -> size_t {
    let mut fileHandle: std::ffi::c_int = 0;
    let mut fileSize: U64 = 0;
    let mut bufferPtr: *mut *mut std::ffi::c_void = &mut (*dict).dictBuffer;
    if !bufferPtr.is_null() {
    } else {
        __assert_fail(
            b"bufferPtr != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            765 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 87],
                &[std::ffi::c_char; 87],
            >(
                b"size_t FIO_setDictBufferMMap(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19866: {
        if !bufferPtr.is_null() {
        } else {
            __assert_fail(
                b"bufferPtr != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                765 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[std::ffi::c_char; 87],
                >(
                    b"size_t FIO_setDictBufferMMap(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !dictFileStat.is_null() {
    } else {
        __assert_fail(
            b"dictFileStat != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            766 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 87],
                &[std::ffi::c_char; 87],
            >(
                b"size_t FIO_setDictBufferMMap(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19823: {
        if !dictFileStat.is_null() {
        } else {
            __assert_fail(
                b"dictFileStat != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                766 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[std::ffi::c_char; 87],
                >(
                    b"size_t FIO_setDictBufferMMap(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *bufferPtr = NULL as *mut std::ffi::c_void;
    if fileName.is_null() {
        return 0 as std::ffi::c_int as size_t;
    }
    if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Loading %s as dictionary \n\0" as *const u8 as *const std::ffi::c_char,
            fileName,
        );
    }
    fileHandle = open(fileName, O_RDONLY);
    if fileHandle == -(1 as std::ffi::c_int) {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                775 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                33 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Couldn't open dictionary %s: %s\0" as *const u8 as *const std::ffi::c_char,
                fileName,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(33 as std::ffi::c_int);
    }
    fileSize = UTIL_getFileSizeStat(dictFileStat);
    let dictSizeMax = (if (*prefs).patchFromMode != 0 {
        (*prefs).memLimit
    } else {
        DICTSIZE_MAX as std::ffi::c_uint
    }) as size_t;
    if fileSize > dictSizeMax {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                783 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                34 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Dictionary file %s is too large (> %u bytes)\0" as *const u8
                    as *const std::ffi::c_char,
                fileName,
                dictSizeMax as std::ffi::c_uint,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(34 as std::ffi::c_int);
    }
    *bufferPtr = mmap(
        NULL as *mut std::ffi::c_void,
        fileSize,
        PROT_READ,
        MAP_PRIVATE,
        fileHandle,
        0 as std::ffi::c_int as __off_t,
    );
    if (*bufferPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                788 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                34 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(34 as std::ffi::c_int);
    }
    close(fileHandle);
    fileSize
}
unsafe extern "C" fn FIO_freeDict(mut dict: *mut FIO_Dict_t) {
    if (*dict).dictBufferType as std::ffi::c_uint
        == FIO_mallocDict as std::ffi::c_int as std::ffi::c_uint
    {
        free((*dict).dictBuffer);
        (*dict).dictBuffer = NULL as *mut std::ffi::c_void;
        (*dict).dictBufferSize = 0 as std::ffi::c_int as size_t;
    } else if (*dict).dictBufferType as std::ffi::c_uint
        == FIO_mmapDict as std::ffi::c_int as std::ffi::c_uint
    {
        FIO_munmap(dict);
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            861 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 32], &[std::ffi::c_char; 32]>(
                b"void FIO_freeDict(FIO_Dict_t *)\0",
            ))
            .as_ptr(),
        );
        'c_29954: {
            __assert_fail(
                b"0\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                861 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[std::ffi::c_char; 32]>(
                    b"void FIO_freeDict(FIO_Dict_t *)\0",
                ))
                .as_ptr(),
            );
        };
    };
}
unsafe extern "C" fn FIO_initDict(
    mut dict: *mut FIO_Dict_t,
    mut fileName: *const std::ffi::c_char,
    prefs: *mut FIO_prefs_t,
    mut dictFileStat: *mut stat_t,
    mut dictBufferType: FIO_dictBufferType_t,
) {
    (*dict).dictBufferType = dictBufferType;
    if (*dict).dictBufferType as std::ffi::c_uint
        == FIO_mallocDict as std::ffi::c_int as std::ffi::c_uint
    {
        (*dict).dictBufferSize = FIO_setDictBufferMalloc(dict, fileName, prefs, dictFileStat);
    } else if (*dict).dictBufferType as std::ffi::c_uint
        == FIO_mmapDict as std::ffi::c_int as std::ffi::c_uint
    {
        (*dict).dictBufferSize = FIO_setDictBufferMMap(dict, fileName, prefs, dictFileStat);
    } else {
        __assert_fail(
            b"0\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            872 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[std::ffi::c_char; 98],
            >(
                b"void FIO_initDict(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *, FIO_dictBufferType_t)\0",
            ))
                .as_ptr(),
        );
        'c_19278: {
            __assert_fail(
                b"0\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                872 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[std::ffi::c_char; 98],
                >(
                    b"void FIO_initDict(FIO_Dict_t *, const char *, FIO_prefs_t *const, stat_t *, FIO_dictBufferType_t)\0",
                ))
                    .as_ptr(),
            );
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn FIO_checkFilenameCollisions(
    mut filenameTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_uint,
) -> std::ffi::c_int {
    let mut filenameTableSorted = std::ptr::null_mut::<*const std::ffi::c_char>();
    let mut prevElem = std::ptr::null::<std::ffi::c_char>();
    let mut filename = std::ptr::null::<std::ffi::c_char>();
    let mut u: std::ffi::c_uint = 0;
    filenameTableSorted = malloc(
        (::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong)
            .wrapping_mul(nbFiles as std::ffi::c_ulong),
    ) as *mut *const std::ffi::c_char;
    if filenameTableSorted.is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Allocation error during filename collision checking \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        return 1 as std::ffi::c_int;
    }
    u = 0 as std::ffi::c_int as std::ffi::c_uint;
    while u < nbFiles {
        filename = strrchr(*filenameTable.offset(u as isize), PATH_SEP);
        if filename.is_null() {
            let fresh0 = &mut (*filenameTableSorted.offset(u as isize));
            *fresh0 = *filenameTable.offset(u as isize);
        } else {
            let fresh1 = &mut (*filenameTableSorted.offset(u as isize));
            *fresh1 = filename.offset(1 as std::ffi::c_int as isize);
        }
        u = u.wrapping_add(1);
        u;
    }
    qsort(
        filenameTableSorted as *mut std::ffi::c_void,
        nbFiles as size_t,
        ::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong,
        Some(
            UTIL_compareStr
                as unsafe extern "C" fn(
                    *const std::ffi::c_void,
                    *const std::ffi::c_void,
                ) -> std::ffi::c_int,
        ),
    );
    prevElem = *filenameTableSorted.offset(0 as std::ffi::c_int as isize);
    u = 1 as std::ffi::c_int as std::ffi::c_uint;
    while u < nbFiles {
        if strcmp(prevElem, *filenameTableSorted.offset(u as isize)) == 0 as std::ffi::c_int
            && g_display_prefs.displayLevel >= 2 as std::ffi::c_int
        {
            fprintf(
                stderr,
                b"WARNING: Two files have same filename: %s\n\0" as *const u8
                    as *const std::ffi::c_char,
                prevElem,
            );
        }
        prevElem = *filenameTableSorted.offset(u as isize);
        u = u.wrapping_add(1);
        u;
    }
    free(filenameTableSorted as *mut std::ffi::c_void);
    0 as std::ffi::c_int
}
unsafe extern "C" fn extractFilename(
    mut path: *const std::ffi::c_char,
    mut separator: std::ffi::c_char,
) -> *const std::ffi::c_char {
    let mut search: *const std::ffi::c_char = strrchr(path, separator as std::ffi::c_int);
    if search.is_null() {
        return path;
    }
    search.offset(1 as std::ffi::c_int as isize)
}
unsafe extern "C" fn FIO_createFilename_fromOutDir(
    mut path: *const std::ffi::c_char,
    mut outDirName: *const std::ffi::c_char,
    suffixLen: size_t,
) -> *mut std::ffi::c_char {
    let mut filenameStart = std::ptr::null::<std::ffi::c_char>();
    let mut separator: std::ffi::c_char = 0;
    let mut result = std::ptr::null_mut::<std::ffi::c_char>();
    separator = '/' as i32 as std::ffi::c_char;
    filenameStart = extractFilename(path, separator);
    result = calloc(
        1 as std::ffi::c_int as std::ffi::c_ulong,
        (strlen(outDirName))
            .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong)
            .wrapping_add(strlen(filenameStart))
            .wrapping_add(suffixLen)
            .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
    ) as *mut std::ffi::c_char;
    if result.is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                945 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                30 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: FIO_createFilename_fromOutDir: %s\0" as *const u8
                    as *const std::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(30 as std::ffi::c_int);
    }
    memcpy(
        result as *mut std::ffi::c_void,
        outDirName as *const std::ffi::c_void,
        strlen(outDirName),
    );
    if *outDirName.offset(
        (strlen(outDirName)).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as isize,
    ) as std::ffi::c_int
        == separator as std::ffi::c_int
    {
        memcpy(
            result.offset(strlen(outDirName) as isize) as *mut std::ffi::c_void,
            filenameStart as *const std::ffi::c_void,
            strlen(filenameStart),
        );
    } else {
        memcpy(
            result.offset(strlen(outDirName) as isize) as *mut std::ffi::c_void,
            &mut separator as *mut std::ffi::c_char as *const std::ffi::c_void,
            1 as std::ffi::c_int as std::ffi::c_ulong,
        );
        memcpy(
            result
                .offset(strlen(outDirName) as isize)
                .offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
            filenameStart as *const std::ffi::c_void,
            strlen(filenameStart),
        );
    }
    result
}
unsafe extern "C" fn FIO_highbit64(mut v: std::ffi::c_ulonglong) -> std::ffi::c_uint {
    let mut count = 0 as std::ffi::c_int as std::ffi::c_uint;
    if v != 0 as std::ffi::c_int as std::ffi::c_ulonglong {
    } else {
        __assert_fail(
            b"v != 0\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            966 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 47], &[std::ffi::c_char; 47]>(
                b"unsigned int FIO_highbit64(unsigned long long)\0",
            ))
            .as_ptr(),
        );
    }
    'c_21046: {
        if v != 0 as std::ffi::c_int as std::ffi::c_ulonglong {
        } else {
            __assert_fail(
                b"v != 0\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                966 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 47], &[std::ffi::c_char; 47]>(
                    b"unsigned int FIO_highbit64(unsigned long long)\0",
                ))
                .as_ptr(),
            );
        }
    };
    v >>= 1 as std::ffi::c_int;
    while v != 0 {
        v >>= 1 as std::ffi::c_int;
        count = count.wrapping_add(1);
        count;
    }
    count
}
unsafe extern "C" fn FIO_adjustMemLimitForPatchFromMode(
    prefs: *mut FIO_prefs_t,
    dictSize: std::ffi::c_ulonglong,
    maxSrcFileSize: std::ffi::c_ulonglong,
) {
    let mut maxSize = if (*prefs).memLimit as std::ffi::c_ulonglong
        > (if dictSize > maxSrcFileSize {
            dictSize
        } else {
            maxSrcFileSize
        }) {
        (*prefs).memLimit as std::ffi::c_ulonglong
    } else if dictSize > maxSrcFileSize {
        dictSize
    } else {
        maxSrcFileSize
    };
    let maxWindowSize = (1 as std::ffi::c_uint)
        << (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
            == 4 as std::ffi::c_int as std::ffi::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        });
    if maxSize == UTIL_FILESIZE_UNKNOWN as std::ffi::c_ulonglong {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                979 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                42 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Using --patch-from with stdin requires --stream-size\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(42 as std::ffi::c_int);
    }
    if maxSize != -(1 as std::ffi::c_int) as U64 as std::ffi::c_ulonglong {
    } else {
        __assert_fail(
            b"maxSize != UTIL_FILESIZE_UNKNOWN\0" as *const u8
                as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            980 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[std::ffi::c_char; 112],
            >(
                b"void FIO_adjustMemLimitForPatchFromMode(FIO_prefs_t *const, const unsigned long long, const unsigned long long)\0",
            ))
                .as_ptr(),
        );
    }
    'c_21413: {
        if maxSize != -(1 as std::ffi::c_int) as U64 as std::ffi::c_ulonglong {
        } else {
            __assert_fail(
                b"maxSize != UTIL_FILESIZE_UNKNOWN\0" as *const u8
                    as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                980 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 112],
                    &[std::ffi::c_char; 112],
                >(
                    b"void FIO_adjustMemLimitForPatchFromMode(FIO_prefs_t *const, const unsigned long long, const unsigned long long)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if maxSize > maxWindowSize as std::ffi::c_ulonglong {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                982 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                42 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Can't handle files larger than %u GB\n\0" as *const u8 as *const std::ffi::c_char,
                maxWindowSize.wrapping_div(
                    (1 as std::ffi::c_int as std::ffi::c_uint)
                        .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int),
                ),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(42 as std::ffi::c_int);
    }
    FIO_setMemLimit(prefs, maxSize as std::ffi::c_uint);
}
unsafe extern "C" fn FIO_multiFilesConcatWarning(
    mut fCtx: *const FIO_ctx_t,
    mut prefs: *mut FIO_prefs_t,
    mut outFileName: *const std::ffi::c_char,
    mut displayLevelCutoff: std::ffi::c_int,
) -> std::ffi::c_int {
    if (*fCtx).hasStdoutOutput != 0 && (*prefs).removeSrcFile != 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1009 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                43 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                    stderr,
                    b"It's not allowed to remove input files when processed output is piped to stdout. This scenario is not supposed to be possible. This is a programming error. File an issue for it to be fixed.\0"
                        as *const u8 as *const std::ffi::c_char,
                );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(43 as std::ffi::c_int);
    }
    if (*prefs).testMode != 0 {
        if (*prefs).removeSrcFile != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1017 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    43 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Test mode shall not remove input files! This scenario is not supposed to be possible. This is a programming error. File an issue for it to be fixed.\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(43 as std::ffi::c_int);
        }
        return 0 as std::ffi::c_int;
    }
    if (*fCtx).nbFilesTotal == 1 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    }
    if (*fCtx).nbFilesTotal > 1 as std::ffi::c_int {
    } else {
        __assert_fail(
            b"fCtx->nbFilesTotal > 1\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            1022 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 85],
                &[std::ffi::c_char; 85],
            >(
                b"int FIO_multiFilesConcatWarning(const FIO_ctx_t *, FIO_prefs_t *, const char *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_38693: {
        if (*fCtx).nbFilesTotal > 1 as std::ffi::c_int {
        } else {
            __assert_fail(
                b"fCtx->nbFilesTotal > 1\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1022 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 85],
                    &[std::ffi::c_char; 85],
                >(
                    b"int FIO_multiFilesConcatWarning(const FIO_ctx_t *, FIO_prefs_t *, const char *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if outFileName.is_null() {
        return 0 as std::ffi::c_int;
    }
    if (*fCtx).hasStdoutOutput != 0 {
        if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: WARNING: all input files will be processed and concatenated into stdout. \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
    } else if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"zstd: WARNING: all input files will be processed and concatenated into a single output file: %s \n\0"
                as *const u8 as *const std::ffi::c_char,
            outFileName,
        );
    }
    if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"The concatenated output CANNOT regenerate original file names nor directory structure. \n\0"
                as *const u8 as *const std::ffi::c_char,
        );
    }
    if (*prefs).removeSrcFile != 0 {
        if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Since it's a destructive operation, input files will not be removed. \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        (*prefs).removeSrcFile = 0 as std::ffi::c_int;
    }
    if (*fCtx).hasStdoutOutput != 0 {
        return 0 as std::ffi::c_int;
    }
    if (*prefs).overwrite != 0 {
        return 0 as std::ffi::c_int;
    }
    if g_display_prefs.displayLevel <= displayLevelCutoff {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Concatenating multiple processed inputs into a single output loses file metadata. \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Aborting. \n\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        return 1 as std::ffi::c_int;
    }
    UTIL_requireUserConfirmation(
        b"Proceed? (y/n): \0" as *const u8 as *const std::ffi::c_char,
        b"Aborting...\0" as *const u8 as *const std::ffi::c_char,
        b"yY\0" as *const u8 as *const std::ffi::c_char,
        (*fCtx).hasStdinInput,
    )
}
unsafe extern "C" fn setInBuffer(
    mut buf: *const std::ffi::c_void,
    mut s: size_t,
    mut pos: size_t,
) -> ZSTD_inBuffer {
    let mut i = ZSTD_inBuffer_s {
        src: std::ptr::null::<std::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    i.src = buf;
    i.size = s;
    i.pos = pos;
    i
}
unsafe extern "C" fn setOutBuffer(
    mut buf: *mut std::ffi::c_void,
    mut s: size_t,
    mut pos: size_t,
) -> ZSTD_outBuffer {
    let mut o = ZSTD_outBuffer_s {
        dst: std::ptr::null_mut::<std::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    o.dst = buf;
    o.size = s;
    o.pos = pos;
    o
}
unsafe extern "C" fn ZSTD_cycleLog(mut hashLog: U32, mut strat: ZSTD_strategy) -> U32 {
    let btScale =
        (strat as U32 >= ZSTD_btlazy2 as std::ffi::c_int as U32) as std::ffi::c_int as U32;
    if hashLog > 1 as std::ffi::c_int as U32 {
    } else {
        __assert_fail(
            b"hashLog > 1\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            1090 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 38], &[std::ffi::c_char; 38]>(
                b"U32 ZSTD_cycleLog(U32, ZSTD_strategy)\0",
            ))
            .as_ptr(),
        );
    }
    'c_20970: {
        if hashLog > 1 as std::ffi::c_int as U32 {
        } else {
            __assert_fail(
                b"hashLog > 1\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1090 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 38], &[std::ffi::c_char; 38]>(
                    b"U32 ZSTD_cycleLog(U32, ZSTD_strategy)\0",
                ))
                .as_ptr(),
            );
        }
    };
    hashLog.wrapping_sub(btScale)
}
unsafe extern "C" fn FIO_adjustParamsForPatchFromMode(
    prefs: *mut FIO_prefs_t,
    mut comprParams: *mut ZSTD_compressionParameters,
    dictSize: std::ffi::c_ulonglong,
    maxSrcFileSize: std::ffi::c_ulonglong,
    mut cLevel: std::ffi::c_int,
) {
    let fileWindowLog =
        (FIO_highbit64(maxSrcFileSize)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let cParams = ZSTD_getCParams(
        cLevel,
        maxSrcFileSize as size_t as std::ffi::c_ulonglong,
        dictSize as size_t,
    );
    FIO_adjustMemLimitForPatchFromMode(prefs, dictSize, maxSrcFileSize);
    if fileWindowLog
        > (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
            == 4 as std::ffi::c_int as std::ffi::c_ulong
        {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) as std::ffi::c_uint
        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
    {
        fprintf(
            stderr,
            b"Max window log exceeded by file (compression ratio will suffer)\n\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    (*comprParams).windowLog = if 10 as std::ffi::c_int as std::ffi::c_uint
        > (if ((if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
            == 4 as std::ffi::c_int as std::ffi::c_ulong
        {
            30 as std::ffi::c_int
        } else {
            31 as std::ffi::c_int
        }) as std::ffi::c_uint)
            < fileWindowLog
        {
            (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 4 as std::ffi::c_int as std::ffi::c_ulong
            {
                30 as std::ffi::c_int
            } else {
                31 as std::ffi::c_int
            }) as std::ffi::c_uint
        } else {
            fileWindowLog
        }) {
        10 as std::ffi::c_int as std::ffi::c_uint
    } else if ((if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong
    {
        30 as std::ffi::c_int
    } else {
        31 as std::ffi::c_int
    }) as std::ffi::c_uint)
        < fileWindowLog
    {
        (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
            == 4 as std::ffi::c_int as std::ffi::c_ulong
        {
            30 as std::ffi::c_int
        } else {
            31 as std::ffi::c_int
        }) as std::ffi::c_uint
    } else {
        fileWindowLog
    };
    if fileWindowLog > ZSTD_cycleLog(cParams.chainLog, cParams.strategy) {
        if (*prefs).ldmFlag == 0 && g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"long mode automatically triggered\n\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        FIO_setLdmFlag(prefs, 1 as std::ffi::c_int as std::ffi::c_uint);
    }
    if cParams.strategy as std::ffi::c_uint >= ZSTD_btopt as std::ffi::c_int as std::ffi::c_uint {
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"[Optimal parser notes] Consider the following to improve patch size at the cost of speed:\n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"- Set a larger targetLength (e.g. --zstd=targetLength=4096)\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"- Set a larger chainLog (e.g. --zstd=chainLog=%u)\n\0" as *const u8
                    as *const std::ffi::c_char,
                if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                    == 4 as std::ffi::c_int as std::ffi::c_ulong
                {
                    29 as std::ffi::c_int
                } else {
                    30 as std::ffi::c_int
                },
            );
        }
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"- Set a larger LDM hashLog (e.g. --zstd=ldmHashLog=%u)\n\0" as *const u8
                    as *const std::ffi::c_char,
                if (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                    == 4 as std::ffi::c_int as std::ffi::c_ulong
                {
                    30 as std::ffi::c_int
                } else {
                    31 as std::ffi::c_int
                }) < 30 as std::ffi::c_int
                {
                    if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                        == 4 as std::ffi::c_int as std::ffi::c_ulong
                    {
                        30 as std::ffi::c_int
                    } else {
                        31 as std::ffi::c_int
                    }
                } else {
                    30 as std::ffi::c_int
                },
            );
        }
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"- Set a smaller LDM rateLog (e.g. --zstd=ldmHashRateLog=%u)\n\0" as *const u8
                    as *const std::ffi::c_char,
                0 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Also consider playing around with searchLog and hashLog\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn FIO_createCResources(
    prefs: *mut FIO_prefs_t,
    mut dictFileName: *const std::ffi::c_char,
    maxSrcFileSize: std::ffi::c_ulonglong,
    mut cLevel: std::ffi::c_int,
    mut comprParams: ZSTD_compressionParameters,
) -> cRess_t {
    let mut useMMap = ((*prefs).mmapDict as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int;
    let mut forceNoUseMMap = ((*prefs).mmapDict as std::ffi::c_uint
        == ZSTD_ps_disable as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int;
    let mut dictBufferType = FIO_mallocDict;
    let mut ress = cRess_t {
        dict: FIO_Dict_t {
            dictBuffer: std::ptr::null_mut::<std::ffi::c_void>(),
            dictBufferSize: 0,
            dictBufferType: FIO_mallocDict,
        },
        dictFileName: std::ptr::null::<std::ffi::c_char>(),
        dictFileStat: stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        },
        cctx: std::ptr::null_mut::<ZSTD_CStream>(),
        writeCtx: std::ptr::null_mut::<WritePoolCtx_t>(),
        readCtx: std::ptr::null_mut::<ReadPoolCtx_t>(),
    };
    memset(
        &mut ress as *mut cRess_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<cRess_t>() as std::ffi::c_ulong,
    );
    if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
        fprintf(
            stderr,
            b"FIO_createCResources \n\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    ress.cctx = ZSTD_createCCtx();
    if (ress.cctx).is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1134 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                30 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"allocation error (%s): can't create ZSTD_CCtx\0" as *const u8
                    as *const std::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(30 as std::ffi::c_int);
    }
    FIO_getDictFileStat(dictFileName, &mut ress.dictFileStat);
    if (*prefs).patchFromMode != 0 {
        let dictSize = UTIL_getFileSizeStat(&mut ress.dictFileStat);
        let ssSize = (*prefs).streamSrcSize as std::ffi::c_ulonglong;
        useMMap |= (dictSize > (*prefs).memLimit as U64) as std::ffi::c_int;
        FIO_adjustParamsForPatchFromMode(
            prefs,
            &mut comprParams,
            dictSize as std::ffi::c_ulonglong,
            if ssSize > 0 as std::ffi::c_int as std::ffi::c_ulonglong {
                ssSize
            } else {
                maxSrcFileSize
            },
            cLevel,
        );
    }
    dictBufferType = (if useMMap != 0 && forceNoUseMMap == 0 {
        FIO_mmapDict as std::ffi::c_int
    } else {
        FIO_mallocDict as std::ffi::c_int
    }) as FIO_dictBufferType_t;
    FIO_initDict(
        &mut ress.dict,
        dictFileName,
        prefs,
        &mut ress.dictFileStat,
        dictBufferType,
    );
    ress.writeCtx = AIO_WritePool_create(prefs, ZSTD_CStreamOutSize());
    ress.readCtx = AIO_ReadPool_create(prefs, ZSTD_CStreamInSize());
    if !dictFileName.is_null() && (ress.dict.dictBuffer).is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1155 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                32 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"allocation error : can't create dictBuffer\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(32 as std::ffi::c_int);
    }
    ress.dictFileName = dictFileName;
    if (*prefs).adaptiveMode != 0 && (*prefs).ldmFlag == 0 && comprParams.windowLog == 0 {
        comprParams.windowLog = ADAPT_WINDOWLOG_DEFAULT as std::ffi::c_uint;
    }
    let mut err: size_t = 0;
    err = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_contentSizeFlag, (*prefs).contentSize);
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_contentSizeFlag, prefs->contentSize)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1161 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_0: size_t = 0;
    err_0 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_dictIDFlag, (*prefs).dictIDFlag);
    if ZSTD_isError(err_0) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_dictIDFlag, prefs->dictIDFlag)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1162 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_0),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_1: size_t = 0;
    err_1 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_checksumFlag, (*prefs).checksumFlag);
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_checksumFlag, prefs->checksumFlag)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1163 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_2: size_t = 0;
    err_2 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_compressionLevel, cLevel);
    if ZSTD_isError(err_2) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_compressionLevel, cLevel)\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1165 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_2),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_3: size_t = 0;
    err_3 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_targetCBlockSize,
        (*prefs).targetCBlockSize as std::ffi::c_int,
    );
    if ZSTD_isError(err_3) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_targetCBlockSize, (int)prefs->targetCBlockSize)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1167 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_3),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_4: size_t = 0;
    err_4 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam7, (*prefs).srcSizeHint);
    if ZSTD_isError(err_4) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam7, (int)prefs->srcSizeHint)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1169 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_4),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_5: size_t = 0;
    err_5 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_enableLongDistanceMatching,
        (*prefs).ldmFlag,
    );
    if ZSTD_isError(err_5) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_enableLongDistanceMatching, prefs->ldmFlag)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1171 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_5),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_6: size_t = 0;
    err_6 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashLog, (*prefs).ldmHashLog);
    if ZSTD_isError(err_6) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashLog, prefs->ldmHashLog)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1172 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_6),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_7: size_t = 0;
    err_7 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmMinMatch, (*prefs).ldmMinMatch);
    if ZSTD_isError(err_7) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmMinMatch, prefs->ldmMinMatch)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1173 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_7),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    if (*prefs).ldmBucketSizeLog != FIO_LDM_PARAM_NOTSET {
        let mut err_8: size_t = 0;
        err_8 = ZSTD_CCtx_setParameter(
            ress.cctx,
            ZSTD_c_ldmBucketSizeLog,
            (*prefs).ldmBucketSizeLog,
        );
        if ZSTD_isError(err_8) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmBucketSizeLog, prefs->ldmBucketSizeLog)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1175 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_8),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    }
    if (*prefs).ldmHashRateLog != FIO_LDM_PARAM_NOTSET {
        let mut err_9: size_t = 0;
        err_9 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashRateLog, (*prefs).ldmHashRateLog);
        if ZSTD_isError(err_9) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_ldmHashRateLog, prefs->ldmHashRateLog)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1178 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_9),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    }
    let mut err_10: size_t = 0;
    err_10 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_experimentalParam14,
        (*prefs).useRowMatchFinder,
    );
    if ZSTD_isError(err_10) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam14, prefs->useRowMatchFinder)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1180 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_10),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_11: size_t = 0;
    err_11 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_windowLog,
        comprParams.windowLog as std::ffi::c_int,
    );
    if ZSTD_isError(err_11) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_windowLog, (int)comprParams.windowLog)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1182 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_11),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_12: size_t = 0;
    err_12 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_chainLog,
        comprParams.chainLog as std::ffi::c_int,
    );
    if ZSTD_isError(err_12) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_chainLog, (int)comprParams.chainLog)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1183 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_12),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_13: size_t = 0;
    err_13 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_hashLog,
        comprParams.hashLog as std::ffi::c_int,
    );
    if ZSTD_isError(err_13) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_hashLog, (int)comprParams.hashLog)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1184 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_13),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_14: size_t = 0;
    err_14 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_searchLog,
        comprParams.searchLog as std::ffi::c_int,
    );
    if ZSTD_isError(err_14) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_searchLog, (int)comprParams.searchLog)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1185 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_14),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_15: size_t = 0;
    err_15 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_minMatch,
        comprParams.minMatch as std::ffi::c_int,
    );
    if ZSTD_isError(err_15) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_minMatch, (int)comprParams.minMatch)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1186 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_15),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_16: size_t = 0;
    err_16 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_targetLength,
        comprParams.targetLength as std::ffi::c_int,
    );
    if ZSTD_isError(err_16) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_targetLength, (int)comprParams.targetLength)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1187 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_16),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_17: size_t = 0;
    err_17 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_strategy,
        comprParams.strategy as std::ffi::c_int,
    );
    if ZSTD_isError(err_17) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_strategy, (int)comprParams.strategy)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1188 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_17),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_18: size_t = 0;
    err_18 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_c_experimentalParam5,
        (*prefs).literalCompressionMode as std::ffi::c_int,
    );
    if ZSTD_isError(err_18) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam5, (int)prefs->literalCompressionMode)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1189 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_18),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_19: size_t = 0;
    err_19 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam8, 1 as std::ffi::c_int);
    if ZSTD_isError(err_19) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam8, 1)\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1190 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_19),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
        fprintf(
            stderr,
            b"set nb workers = %u \n\0" as *const u8 as *const std::ffi::c_char,
            (*prefs).nbWorkers,
        );
    }
    let mut err_20: size_t = 0;
    err_20 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_nbWorkers, (*prefs).nbWorkers);
    if ZSTD_isError(err_20) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_nbWorkers, prefs->nbWorkers)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1194 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_20),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_21: size_t = 0;
    err_21 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_jobSize, (*prefs).jobSize);
    if ZSTD_isError(err_21) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_jobSize, prefs->jobSize)\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1195 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_21),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    if (*prefs).overlapLog != FIO_OVERLAP_LOG_NOTSET {
        if g_display_prefs.displayLevel >= 3 as std::ffi::c_int {
            fprintf(
                stderr,
                b"set overlapLog = %u \n\0" as *const u8 as *const std::ffi::c_char,
                (*prefs).overlapLog,
            );
        }
        let mut err_22: size_t = 0;
        err_22 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_overlapLog, (*prefs).overlapLog);
        if ZSTD_isError(err_22) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_overlapLog, prefs->overlapLog)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1198 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_22),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    }
    let mut err_23: size_t = 0;
    err_23 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam1, (*prefs).rsyncable);
    if ZSTD_isError(err_23) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_c_experimentalParam1, prefs->rsyncable)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1200 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_23),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    if (*prefs).patchFromMode != 0 {
        let mut err_24: size_t = 0;
        err_24 = ZSTD_CCtx_refPrefix(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize);
        if ZSTD_isError(err_24) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_CCtx_refPrefix(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1204 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_24),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    } else {
        let mut err_25: size_t = 0;
        err_25 = ZSTD_CCtx_loadDictionary_byReference(
            ress.cctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_25) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_CCtx_loadDictionary_byReference(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1206 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_25),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    }
    ress
}
unsafe extern "C" fn FIO_freeCResources(ress: *mut cRess_t) {
    FIO_freeDict(&mut (*ress).dict);
    AIO_WritePool_free((*ress).writeCtx);
    AIO_ReadPool_free((*ress).readCtx);
    ZSTD_freeCStream((*ress).cctx);
}
unsafe extern "C" fn FIO_compressGzFrame(
    mut ress: *const cRess_t,
    mut srcFileName: *const std::ffi::c_char,
    srcFileSize: U64,
    mut compressionLevel: std::ffi::c_int,
    mut readsize: *mut U64,
) -> std::ffi::c_ulonglong {
    let mut inFileSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    let mut outFileSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    let mut strm = z_stream_s {
        next_in: std::ptr::null_mut::<Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: std::ptr::null_mut::<Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: std::ptr::null_mut::<std::ffi::c_char>(),
        state: std::ptr::null_mut::<internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: std::ptr::null_mut::<std::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut writeJob = NULL as *mut IOJob_t;
    if compressionLevel > Z_BEST_COMPRESSION {
        compressionLevel = Z_BEST_COMPRESSION;
    }
    strm.zalloc = ::core::mem::transmute::<libc::intptr_t, alloc_func>(Z_NULL as libc::intptr_t);
    strm.zfree = ::core::mem::transmute::<libc::intptr_t, free_func>(Z_NULL as libc::intptr_t);
    strm.opaque = Z_NULL as voidpf;
    let ret = deflateInit2_(
        &mut strm,
        compressionLevel,
        8 as std::ffi::c_int,
        15 as std::ffi::c_int + 16 as std::ffi::c_int,
        8 as std::ffi::c_int,
        0 as std::ffi::c_int,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as std::ffi::c_ulong as std::ffi::c_int,
    );
    if ret != Z_OK {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1242 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                71 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: deflateInit2 error %d \n\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
                ret,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(71 as std::ffi::c_int);
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_in = std::ptr::null_mut::<Bytef>();
    strm.avail_in = 0 as std::ffi::c_int as uInt;
    strm.next_out = (*writeJob).buffer as *mut Bytef;
    strm.avail_out = (*writeJob).bufferSize as uInt;
    loop {
        let mut ret_0: std::ffi::c_int = 0;
        if strm.avail_in == 0 as std::ffi::c_int as uInt {
            AIO_ReadPool_fillBuffer((*ress).readCtx, ZSTD_CStreamInSize());
            if (*(*ress).readCtx).srcBufferLoaded == 0 as std::ffi::c_int as size_t {
                break;
            }
            inFileSize = inFileSize
                .wrapping_add((*(*ress).readCtx).srcBufferLoaded as std::ffi::c_ulonglong);
            strm.next_in = (*(*ress).readCtx).srcBuffer as *mut std::ffi::c_uchar;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded as uInt;
        }
        let availBefore = strm.avail_in as size_t;
        ret_0 = deflate(&mut strm, Z_NO_FLUSH);
        AIO_ReadPool_consumeBytes(
            (*ress).readCtx,
            availBefore.wrapping_sub(strm.avail_in as size_t),
        );
        if ret_0 != Z_OK {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1268 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    72 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: deflate error %d \n\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                    ret_0,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(72 as std::ffi::c_int);
        }
        let cSize = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out as size_t);
        if cSize != 0 {
            (*writeJob).usedBufferSize = cSize;
            AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
            outFileSize = outFileSize.wrapping_add(cSize as std::ffi::c_ulonglong);
            strm.next_out = (*writeJob).buffer as *mut Bytef;
            strm.avail_out = (*writeJob).bufferSize as uInt;
        }
        if srcFileSize == UTIL_FILESIZE_UNKNOWN as U64 {
            if g_display_prefs.progressSetting as std::ffi::c_uint
                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                    || g_display_prefs.progressSetting as std::ffi::c_uint
                        == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                && g_display_prefs.progressSetting as std::ffi::c_uint
                    != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4 as std::ffi::c_int)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"\rRead : %u MB ==> %.2f%% \0" as *const u8 as *const std::ffi::c_char,
                    (inFileSize >> 20 as std::ffi::c_int) as std::ffi::c_uint,
                    outFileSize as std::ffi::c_double / inFileSize as std::ffi::c_double
                        * 100 as std::ffi::c_int as std::ffi::c_double,
                );
                if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                    fflush(stderr);
                }
            }
        } else if g_display_prefs.progressSetting as std::ffi::c_uint
            != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                || g_display_prefs.progressSetting as std::ffi::c_uint
                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
            && g_display_prefs.progressSetting as std::ffi::c_uint
                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                || g_display_prefs.displayLevel >= 4 as std::ffi::c_int)
        {
            g_displayClock = UTIL_getTime();
            fprintf(
                stderr,
                b"\rRead : %u / %u MB ==> %.2f%% \0" as *const u8 as *const std::ffi::c_char,
                (inFileSize >> 20 as std::ffi::c_int) as std::ffi::c_uint,
                (srcFileSize >> 20 as std::ffi::c_int) as std::ffi::c_uint,
                outFileSize as std::ffi::c_double / inFileSize as std::ffi::c_double
                    * 100 as std::ffi::c_int as std::ffi::c_double,
            );
            if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                fflush(stderr);
            }
        }
    }
    loop {
        let ret_1 = deflate(&mut strm, Z_FINISH);
        let cSize_0 = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out as size_t);
        if cSize_0 != 0 {
            (*writeJob).usedBufferSize = cSize_0;
            AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
            outFileSize = outFileSize.wrapping_add(cSize_0 as std::ffi::c_ulonglong);
            strm.next_out = (*writeJob).buffer as *mut Bytef;
            strm.avail_out = (*writeJob).bufferSize as uInt;
        }
        if ret_1 == Z_STREAM_END {
            break;
        }
        if ret_1 != Z_BUF_ERROR {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1301 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    77 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: deflate error %d \n\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                    ret_1,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(77 as std::ffi::c_int);
        }
    }
    let ret_2 = deflateEnd(&mut strm);
    if ret_2 != Z_OK {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1306 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                79 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: deflateEnd error %d \n\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
                ret_2,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(79 as std::ffi::c_int);
    }
    *readsize = inFileSize as U64;
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    outFileSize
}
unsafe extern "C" fn FIO_compressLzmaFrame(
    mut ress: *mut cRess_t,
    mut srcFileName: *const std::ffi::c_char,
    srcFileSize: U64,
    mut compressionLevel: std::ffi::c_int,
    mut readsize: *mut U64,
    mut plain_lzma: std::ffi::c_int,
) -> std::ffi::c_ulonglong {
    let mut inFileSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    let mut outFileSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    let mut strm = {
        lzma_stream {
            next_in: NULL as *const uint8_t,
            avail_in: 0 as std::ffi::c_int as size_t,
            total_in: 0 as std::ffi::c_int as uint64_t,
            next_out: NULL as *mut uint8_t,
            avail_out: 0 as std::ffi::c_int as size_t,
            total_out: 0 as std::ffi::c_int as uint64_t,
            allocator: NULL as *const lzma_allocator,
            internal: NULL as *mut lzma_internal,
            reserved_ptr1: NULL as *mut std::ffi::c_void,
            reserved_ptr2: NULL as *mut std::ffi::c_void,
            reserved_ptr3: NULL as *mut std::ffi::c_void,
            reserved_ptr4: NULL as *mut std::ffi::c_void,
            seek_pos: 0 as std::ffi::c_int as uint64_t,
            reserved_int2: 0 as std::ffi::c_int as uint64_t,
            reserved_int3: 0 as std::ffi::c_int as size_t,
            reserved_int4: 0 as std::ffi::c_int as size_t,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
        }
    };
    let mut action = LZMA_RUN;
    let mut ret = LZMA_OK;
    let mut writeJob = NULL as *mut IOJob_t;
    if compressionLevel < 0 as std::ffi::c_int {
        compressionLevel = 0 as std::ffi::c_int;
    }
    if compressionLevel > 9 as std::ffi::c_int {
        compressionLevel = 9 as std::ffi::c_int;
    }
    if plain_lzma != 0 {
        let mut opt_lzma = lzma_options_lzma {
            dict_size: 0,
            preset_dict: std::ptr::null::<uint8_t>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0 as lzma_mode,
            nice_len: 0,
            mf: 0 as lzma_match_finder,
            depth: 0,
            ext_flags: 0,
            ext_size_low: 0,
            ext_size_high: 0,
            reserved_int4: 0,
            reserved_int5: 0,
            reserved_int6: 0,
            reserved_int7: 0,
            reserved_int8: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
            reserved_enum3: LZMA_RESERVED_ENUM,
            reserved_enum4: LZMA_RESERVED_ENUM,
            reserved_ptr1: std::ptr::null_mut::<std::ffi::c_void>(),
            reserved_ptr2: std::ptr::null_mut::<std::ffi::c_void>(),
        };
        if lzma_lzma_preset(&mut opt_lzma, compressionLevel as uint32_t) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1334 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    81 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_lzma_preset error\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(81 as std::ffi::c_int);
        }
        ret = lzma_alone_encoder(&mut strm, &mut opt_lzma);
        if ret as std::ffi::c_uint != LZMA_OK as std::ffi::c_int as std::ffi::c_uint {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1337 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    82 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_alone_encoder error %d\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                    ret as std::ffi::c_uint,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(82 as std::ffi::c_int);
        }
    } else {
        ret = lzma_easy_encoder(&mut strm, compressionLevel as uint32_t, LZMA_CHECK_CRC64);
        if ret as std::ffi::c_uint != LZMA_OK as std::ffi::c_int as std::ffi::c_uint {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1341 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    83 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_easy_encoder error %d\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                    ret as std::ffi::c_uint,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(83 as std::ffi::c_int);
        }
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_out = (*writeJob).buffer as *mut BYTE;
    strm.avail_out = (*writeJob).bufferSize;
    strm.next_in = std::ptr::null::<uint8_t>();
    strm.avail_in = 0 as std::ffi::c_int as size_t;
    loop {
        if strm.avail_in == 0 as std::ffi::c_int as size_t {
            let inSize = AIO_ReadPool_fillBuffer((*ress).readCtx, ZSTD_CStreamInSize());
            if (*(*ress).readCtx).srcBufferLoaded == 0 as std::ffi::c_int as size_t {
                action = LZMA_FINISH;
            }
            inFileSize = inFileSize.wrapping_add(inSize as std::ffi::c_ulonglong);
            strm.next_in = (*(*ress).readCtx).srcBuffer as *const BYTE;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded;
        }
        let availBefore = strm.avail_in;
        ret = lzma_code(&mut strm, action);
        AIO_ReadPool_consumeBytes((*ress).readCtx, availBefore.wrapping_sub(strm.avail_in));
        if ret as std::ffi::c_uint != LZMA_OK as std::ffi::c_int as std::ffi::c_uint
            && ret as std::ffi::c_uint != LZMA_STREAM_END as std::ffi::c_int as std::ffi::c_uint
        {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1367 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    84 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_code encoding error %d\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                    ret as std::ffi::c_uint,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(84 as std::ffi::c_int);
        }
        let compBytes = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out);
        if compBytes != 0 {
            (*writeJob).usedBufferSize = compBytes;
            AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
            outFileSize = outFileSize.wrapping_add(compBytes as std::ffi::c_ulonglong);
            strm.next_out = (*writeJob).buffer as *mut BYTE;
            strm.avail_out = (*writeJob).bufferSize;
        }
        if srcFileSize == UTIL_FILESIZE_UNKNOWN as U64 {
            if g_display_prefs.progressSetting as std::ffi::c_uint
                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                    || g_display_prefs.progressSetting as std::ffi::c_uint
                        == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                && g_display_prefs.progressSetting as std::ffi::c_uint
                    != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4 as std::ffi::c_int)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"\rRead : %u MB ==> %.2f%%\0" as *const u8 as *const std::ffi::c_char,
                    (inFileSize >> 20 as std::ffi::c_int) as std::ffi::c_uint,
                    outFileSize as std::ffi::c_double / inFileSize as std::ffi::c_double
                        * 100 as std::ffi::c_int as std::ffi::c_double,
                );
                if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                    fflush(stderr);
                }
            }
        } else if g_display_prefs.progressSetting as std::ffi::c_uint
            != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                || g_display_prefs.progressSetting as std::ffi::c_uint
                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
            && g_display_prefs.progressSetting as std::ffi::c_uint
                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                || g_display_prefs.displayLevel >= 4 as std::ffi::c_int)
        {
            g_displayClock = UTIL_getTime();
            fprintf(
                stderr,
                b"\rRead : %u / %u MB ==> %.2f%%\0" as *const u8 as *const std::ffi::c_char,
                (inFileSize >> 20 as std::ffi::c_int) as std::ffi::c_uint,
                (srcFileSize >> 20 as std::ffi::c_int) as std::ffi::c_uint,
                outFileSize as std::ffi::c_double / inFileSize as std::ffi::c_double
                    * 100 as std::ffi::c_int as std::ffi::c_double,
            );
            if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                fflush(stderr);
            }
        }
        if ret as std::ffi::c_uint == LZMA_STREAM_END as std::ffi::c_int as std::ffi::c_uint {
            break;
        }
    }
    lzma_end(&mut strm);
    *readsize = inFileSize as U64;
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    outFileSize
}
unsafe extern "C" fn FIO_compressZstdFrame(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ressPtr: *const cRess_t,
    mut srcFileName: *const std::ffi::c_char,
    mut fileSize: U64,
    mut compressionLevel: std::ffi::c_int,
    mut readsize: *mut U64,
) -> std::ffi::c_ulonglong {
    let ress = *ressPtr;
    let mut writeJob = AIO_WritePool_acquireJob((*ressPtr).writeCtx);
    let mut compressedfilesize = 0 as std::ffi::c_int as U64;
    let mut directive = ZSTD_e_continue;
    let mut pledgedSrcSize = ZSTD_CONTENTSIZE_UNKNOWN as U64;
    let mut previous_zfp_update = {
        ZSTD_frameProgression {
            ingested: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            consumed: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            produced: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            flushed: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            currentJobID: 0 as std::ffi::c_int as std::ffi::c_uint,
            nbActiveWorkers: 0 as std::ffi::c_int as std::ffi::c_uint,
        }
    };
    let mut previous_zfp_correction = {
        ZSTD_frameProgression {
            ingested: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            consumed: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            produced: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            flushed: 0 as std::ffi::c_int as std::ffi::c_ulonglong,
            currentJobID: 0 as std::ffi::c_int as std::ffi::c_uint,
            nbActiveWorkers: 0 as std::ffi::c_int as std::ffi::c_uint,
        }
    };
    let mut speedChange = noChange;
    let mut flushWaiting = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut inputPresented = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut inputBlocked = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut lastJobID = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut lastAdaptTime = UTIL_getTime();
    let adaptEveryMicro = REFRESH_RATE;
    let file_hrs = UTIL_makeHumanReadableSize(fileSize);
    if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
        fprintf(
            stderr,
            b"compression using zstd format \n\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64 {
        pledgedSrcSize = fileSize;
        let mut err: size_t = 0;
        err = ZSTD_CCtx_setPledgedSrcSize(ress.cctx, fileSize as std::ffi::c_ulonglong);
        if ZSTD_isError(err) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, fileSize)\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1532 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    } else if (*prefs).streamSrcSize > 0 as std::ffi::c_int as size_t {
        pledgedSrcSize = (*prefs).streamSrcSize;
        let mut err_0: size_t = 0;
        err_0 =
            ZSTD_CCtx_setPledgedSrcSize(ress.cctx, (*prefs).streamSrcSize as std::ffi::c_ulonglong);
        if ZSTD_isError(err_0) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, prefs->streamSrcSize)\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1536 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_0),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    }
    let mut windowLog: std::ffi::c_int = 0;
    let mut windowSize = UTIL_HumanReadableSize_t {
        value: 0.,
        precision: 0,
        suffix: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut err_1: size_t = 0;
    err_1 = ZSTD_CCtx_getParameter(ress.cctx, ZSTD_c_windowLog, &mut windowLog);
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_CCtx_getParameter(ress.cctx, ZSTD_c_windowLog, &windowLog)\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1541 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    if windowLog == 0 as std::ffi::c_int {
        if (*prefs).ldmFlag != 0 {
            windowLog = ZSTD_WINDOWLOG_LIMIT_DEFAULT;
        } else {
            let cParams = ZSTD_getCParams(
                compressionLevel,
                fileSize as std::ffi::c_ulonglong,
                0 as std::ffi::c_int as size_t,
            );
            windowLog = cParams.windowLog as std::ffi::c_int;
        }
    }
    windowSize = UTIL_makeHumanReadableSize(
        (if 1 as std::ffi::c_ulonglong
            > (if (1 as std::ffi::c_ulonglong) << windowLog
                < pledgedSrcSize as std::ffi::c_ulonglong
            {
                (1 as std::ffi::c_ulonglong) << windowLog
            } else {
                pledgedSrcSize as std::ffi::c_ulonglong
            })
        {
            1 as std::ffi::c_ulonglong
        } else if (1 as std::ffi::c_ulonglong) << windowLog
            < pledgedSrcSize as std::ffi::c_ulonglong
        {
            (1 as std::ffi::c_ulonglong) << windowLog
        } else {
            pledgedSrcSize as std::ffi::c_ulonglong
        }) as U64,
    );
    if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Decompression will require %.*f%s of memory\n\0" as *const u8
                as *const std::ffi::c_char,
            windowSize.precision,
            windowSize.value,
            windowSize.suffix,
        );
    }
    loop {
        let mut stillToFlush: size_t = 0;
        let inSize = AIO_ReadPool_fillBuffer(ress.readCtx, ZSTD_CStreamInSize());
        let mut inBuff = setInBuffer(
            (*ress.readCtx).srcBuffer as *const std::ffi::c_void,
            (*ress.readCtx).srcBufferLoaded,
            0 as std::ffi::c_int as size_t,
        );
        if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
            fprintf(
                stderr,
                b"fread %u bytes from source \n\0" as *const u8 as *const std::ffi::c_char,
                inSize as std::ffi::c_uint,
            );
        }
        *readsize = (*readsize as std::ffi::c_ulong).wrapping_add(inSize) as U64 as U64;
        if (*ress.readCtx).srcBufferLoaded == 0 as std::ffi::c_int as size_t
            || *readsize == fileSize
        {
            directive = ZSTD_e_end;
        }
        stillToFlush = 1 as std::ffi::c_int as size_t;
        while inBuff.pos != inBuff.size
            || directive as std::ffi::c_uint == ZSTD_e_end as std::ffi::c_int as std::ffi::c_uint
                && stillToFlush != 0 as std::ffi::c_int as size_t
        {
            let oldIPos = inBuff.pos;
            let mut outBuff = setOutBuffer(
                (*writeJob).buffer,
                (*writeJob).bufferSize,
                0 as std::ffi::c_int as size_t,
            );
            let toFlushNow = ZSTD_toFlushNow(ress.cctx);
            stillToFlush = ZSTD_compressStream2(ress.cctx, &mut outBuff, &mut inBuff, directive);
            if ZSTD_isError(stillToFlush) != 0 {
                if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                        b"ZSTD_compressStream2(ress.cctx, &outBuff, &inBuff, directive)\0"
                            as *const u8 as *const std::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const std::ffi::c_char,
                        b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                        1574 as std::ffi::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                        11 as std::ffi::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"%s\0" as *const u8 as *const std::ffi::c_char,
                        ZSTD_getErrorName(stillToFlush),
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                }
                exit(11 as std::ffi::c_int);
            }
            AIO_ReadPool_consumeBytes(ress.readCtx, (inBuff.pos).wrapping_sub(oldIPos));
            inputPresented = inputPresented.wrapping_add(1);
            inputPresented;
            if oldIPos == inBuff.pos {
                inputBlocked = inputBlocked.wrapping_add(1);
                inputBlocked;
            }
            if toFlushNow == 0 {
                flushWaiting = 1 as std::ffi::c_int as std::ffi::c_uint;
            }
            if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"ZSTD_compress_generic(end:%u) => input pos(%u)<=(%u)size ; output generated %u bytes \n\0"
                        as *const u8 as *const std::ffi::c_char,
                    directive as std::ffi::c_uint,
                    inBuff.pos as std::ffi::c_uint,
                    inBuff.size as std::ffi::c_uint,
                    outBuff.pos as std::ffi::c_uint,
                );
            }
            if outBuff.pos != 0 {
                (*writeJob).usedBufferSize = outBuff.pos;
                AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
                compressedfilesize = (compressedfilesize as std::ffi::c_ulong)
                    .wrapping_add(outBuff.pos) as U64 as U64;
            }
            if (*prefs).adaptiveMode != 0 && UTIL_clockSpanMicro(lastAdaptTime) > adaptEveryMicro {
                let zfp = ZSTD_getFrameProgression(ress.cctx);
                lastAdaptTime = UTIL_getTime();
                if zfp.currentJobID > 1 as std::ffi::c_int as std::ffi::c_uint {
                    let mut newlyProduced =
                        (zfp.produced).wrapping_sub(previous_zfp_update.produced);
                    let mut newlyFlushed = (zfp.flushed).wrapping_sub(previous_zfp_update.flushed);
                    if zfp.produced >= previous_zfp_update.produced {
                    } else {
                        __assert_fail(
                            b"zfp.produced >= previous_zfp_update.produced\0"
                                as *const u8 as *const std::ffi::c_char,
                            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                            1602 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 127],
                                &[std::ffi::c_char; 127],
                            >(
                                b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_26968: {
                        if zfp.produced >= previous_zfp_update.produced {
                        } else {
                            __assert_fail(
                                b"zfp.produced >= previous_zfp_update.produced\0"
                                    as *const u8 as *const std::ffi::c_char,
                                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                                1602 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 127],
                                    &[std::ffi::c_char; 127],
                                >(
                                    b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*prefs).nbWorkers >= 1 as std::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"prefs->nbWorkers >= 1\0" as *const u8
                                as *const std::ffi::c_char,
                            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                            1603 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 127],
                                &[std::ffi::c_char; 127],
                            >(
                                b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_26928: {
                        if (*prefs).nbWorkers >= 1 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"prefs->nbWorkers >= 1\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                                1603 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 127],
                                    &[std::ffi::c_char; 127],
                                >(
                                    b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if zfp.consumed == previous_zfp_update.consumed
                        && zfp.nbActiveWorkers == 0 as std::ffi::c_int as std::ffi::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"all buffers full : compression stopped => slow down \n\0"
                                    as *const u8
                                    as *const std::ffi::c_char,
                            );
                        }
                        speedChange = slower;
                    }
                    previous_zfp_update = zfp;
                    if newlyProduced
                        > newlyFlushed
                            .wrapping_mul(9 as std::ffi::c_int as std::ffi::c_ulonglong)
                            .wrapping_div(8 as std::ffi::c_int as std::ffi::c_ulonglong)
                        && flushWaiting == 0 as std::ffi::c_int as std::ffi::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"compression faster than flush (%llu > %llu), and flushed was never slowed down by lack of production => slow down \n\0"
                                    as *const u8 as *const std::ffi::c_char,
                                newlyProduced,
                                newlyFlushed,
                            );
                        }
                        speedChange = slower;
                    }
                    flushWaiting = 0 as std::ffi::c_int as std::ffi::c_uint;
                }
                if zfp.currentJobID > lastJobID {
                    if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"compression level adaptation check \n\0" as *const u8
                                as *const std::ffi::c_char,
                        );
                    }
                    if zfp.currentJobID
                        > ((*prefs).nbWorkers + 1 as std::ffi::c_int) as std::ffi::c_uint
                    {
                        if inputBlocked <= 0 as std::ffi::c_int as std::ffi::c_uint {
                            if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                                fprintf(
                                    stderr,
                                    b"input is never blocked => input is slower than ingestion \n\0"
                                        as *const u8
                                        as *const std::ffi::c_char,
                                );
                            }
                            speedChange = slower;
                        } else if speedChange as std::ffi::c_uint
                            == noChange as std::ffi::c_int as std::ffi::c_uint
                        {
                            let mut newlyIngested =
                                (zfp.ingested).wrapping_sub(previous_zfp_correction.ingested);
                            let mut newlyConsumed =
                                (zfp.consumed).wrapping_sub(previous_zfp_correction.consumed);
                            let mut newlyProduced_0 =
                                (zfp.produced).wrapping_sub(previous_zfp_correction.produced);
                            let mut newlyFlushed_0 =
                                (zfp.flushed).wrapping_sub(previous_zfp_correction.flushed);
                            previous_zfp_correction = zfp;
                            if inputPresented > 0 as std::ffi::c_int as std::ffi::c_uint {
                            } else {
                                __assert_fail(
                                    b"inputPresented > 0\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                                    1642 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 127],
                                        &[std::ffi::c_char; 127],
                                    >(
                                        b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_26680: {
                                if inputPresented > 0 as std::ffi::c_int as std::ffi::c_uint {
                                } else {
                                    __assert_fail(
                                        b"inputPresented > 0\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                                        1642 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 127],
                                            &[std::ffi::c_char; 127],
                                        >(
                                            b"unsigned long long FIO_compressZstdFrame(FIO_ctx_t *const, FIO_prefs_t *const, const cRess_t *, const char *, U64, int, U64 *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                                fprintf(
                                    stderr,
                                    b"input blocked %u/%u(%.2f) - ingested:%u vs %u:consumed - flushed:%u vs %u:produced \n\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    inputBlocked,
                                    inputPresented,
                                    inputBlocked as std::ffi::c_double
                                        / inputPresented as std::ffi::c_double
                                        * 100 as std::ffi::c_int as std::ffi::c_double,
                                    newlyIngested as std::ffi::c_uint,
                                    newlyConsumed as std::ffi::c_uint,
                                    newlyFlushed_0 as std::ffi::c_uint,
                                    newlyProduced_0 as std::ffi::c_uint,
                                );
                            }
                            if inputBlocked
                                > inputPresented
                                    .wrapping_div(8 as std::ffi::c_int as std::ffi::c_uint)
                                && newlyFlushed_0
                                    .wrapping_mul(33 as std::ffi::c_int as std::ffi::c_ulonglong)
                                    .wrapping_div(32 as std::ffi::c_int as std::ffi::c_ulonglong)
                                    > newlyProduced_0
                                && newlyIngested
                                    .wrapping_mul(33 as std::ffi::c_int as std::ffi::c_ulonglong)
                                    .wrapping_div(32 as std::ffi::c_int as std::ffi::c_ulonglong)
                                    > newlyConsumed
                            {
                                if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"recommend faster as in(%llu) >= (%llu)comp(%llu) <= out(%llu) \n\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        newlyIngested,
                                        newlyConsumed,
                                        newlyProduced_0,
                                        newlyFlushed_0,
                                    );
                                }
                                speedChange = faster;
                            }
                        }
                        inputBlocked = 0 as std::ffi::c_int as std::ffi::c_uint;
                        inputPresented = 0 as std::ffi::c_int as std::ffi::c_uint;
                    }
                    if speedChange as std::ffi::c_uint
                        == slower as std::ffi::c_int as std::ffi::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"slower speed , higher compression \n\0" as *const u8
                                    as *const std::ffi::c_char,
                            );
                        }
                        compressionLevel += 1;
                        compressionLevel;
                        if compressionLevel > ZSTD_maxCLevel() {
                            compressionLevel = ZSTD_maxCLevel();
                        }
                        if compressionLevel > (*prefs).maxAdaptLevel {
                            compressionLevel = (*prefs).maxAdaptLevel;
                        }
                        compressionLevel +=
                            (compressionLevel == 0 as std::ffi::c_int) as std::ffi::c_int;
                        ZSTD_CCtx_setParameter(
                            ress.cctx,
                            ZSTD_c_compressionLevel,
                            compressionLevel,
                        );
                    }
                    if speedChange as std::ffi::c_uint
                        == faster as std::ffi::c_int as std::ffi::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"faster speed , lighter compression \n\0" as *const u8
                                    as *const std::ffi::c_char,
                            );
                        }
                        compressionLevel -= 1;
                        compressionLevel;
                        if compressionLevel < (*prefs).minAdaptLevel {
                            compressionLevel = (*prefs).minAdaptLevel;
                        }
                        compressionLevel -=
                            (compressionLevel == 0 as std::ffi::c_int) as std::ffi::c_int;
                        ZSTD_CCtx_setParameter(
                            ress.cctx,
                            ZSTD_c_compressionLevel,
                            compressionLevel,
                        );
                    }
                    speedChange = noChange;
                    lastJobID = zfp.currentJobID;
                }
            }
            if g_display_prefs.progressSetting as std::ffi::c_uint
                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                    || g_display_prefs.progressSetting as std::ffi::c_uint
                        == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4 as std::ffi::c_int)
            {
                let zfp_0 = ZSTD_getFrameProgression(ress.cctx);
                let cShare = zfp_0.produced as std::ffi::c_double
                    / (zfp_0.consumed).wrapping_add(
                        (zfp_0.consumed == 0) as std::ffi::c_int as std::ffi::c_ulonglong,
                    ) as std::ffi::c_double
                    * 100 as std::ffi::c_int as std::ffi::c_double;
                let buffered_hrs = UTIL_makeHumanReadableSize(
                    (zfp_0.ingested).wrapping_sub(zfp_0.consumed) as U64,
                );
                let consumed_hrs = UTIL_makeHumanReadableSize(zfp_0.consumed as U64);
                let produced_hrs = UTIL_makeHumanReadableSize(zfp_0.produced as U64);
                g_displayClock = UTIL_getTime();
                if g_display_prefs.progressSetting as std::ffi::c_uint
                    != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                    && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                        || g_display_prefs.progressSetting as std::ffi::c_uint
                            == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                    && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                {
                    fprintf(
                        stderr,
                        b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
                        b"\0" as *const u8 as *const std::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 3 as std::ffi::c_int {
                    if g_display_prefs.progressSetting as std::ffi::c_uint
                        != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                            || g_display_prefs.progressSetting as std::ffi::c_uint
                                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                    {
                        fprintf(
                                stderr,
                                b"(L%i) Buffered:%5.*f%s - Consumed:%5.*f%s - Compressed:%5.*f%s => %.2f%% \0"
                                    as *const u8 as *const std::ffi::c_char,
                                compressionLevel,
                                buffered_hrs.precision,
                                buffered_hrs.value,
                                buffered_hrs.suffix,
                                consumed_hrs.precision,
                                consumed_hrs.value,
                                consumed_hrs.suffix,
                                produced_hrs.precision,
                                produced_hrs.value,
                                produced_hrs.suffix,
                                cShare,
                            );
                    }
                } else {
                    if (*fCtx).nbFilesTotal > 1 as std::ffi::c_int {
                        let mut srcFileNameSize = strlen(srcFileName);
                        if srcFileNameSize > 18 as std::ffi::c_int as size_t {
                            let mut truncatedSrcFileName = srcFileName
                                .offset(srcFileNameSize as isize)
                                .offset(-(15 as std::ffi::c_int as isize));
                            if g_display_prefs.progressSetting as std::ffi::c_uint
                                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                                && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                                    || g_display_prefs.progressSetting as std::ffi::c_uint
                                        == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                                && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                            {
                                fprintf(
                                    stderr,
                                    b"Compress: %u/%u files. Current: ...%s \0" as *const u8
                                        as *const std::ffi::c_char,
                                    (*fCtx).currFileIdx + 1 as std::ffi::c_int,
                                    (*fCtx).nbFilesTotal,
                                    truncatedSrcFileName,
                                );
                            }
                        } else if g_display_prefs.progressSetting as std::ffi::c_uint
                            != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                            && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                                || g_display_prefs.progressSetting as std::ffi::c_uint
                                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                        {
                            fprintf(
                                stderr,
                                b"Compress: %u/%u files. Current: %*s \0" as *const u8
                                    as *const std::ffi::c_char,
                                (*fCtx).currFileIdx + 1 as std::ffi::c_int,
                                (*fCtx).nbFilesTotal,
                                (18 as std::ffi::c_int as size_t).wrapping_sub(srcFileNameSize)
                                    as std::ffi::c_int,
                                srcFileName,
                            );
                        }
                    }
                    if g_display_prefs.progressSetting as std::ffi::c_uint
                        != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                            || g_display_prefs.progressSetting as std::ffi::c_uint
                                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                    {
                        fprintf(
                            stderr,
                            b"Read:%6.*f%4s \0" as *const u8 as *const std::ffi::c_char,
                            consumed_hrs.precision,
                            consumed_hrs.value,
                            consumed_hrs.suffix,
                        );
                    }
                    if fileSize != UTIL_FILESIZE_UNKNOWN as U64
                        && g_display_prefs.progressSetting as std::ffi::c_uint
                            != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                            || g_display_prefs.progressSetting as std::ffi::c_uint
                                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                    {
                        fprintf(
                            stderr,
                            b"/%6.*f%4s\0" as *const u8 as *const std::ffi::c_char,
                            file_hrs.precision,
                            file_hrs.value,
                            file_hrs.suffix,
                        );
                    }
                    if g_display_prefs.progressSetting as std::ffi::c_uint
                        != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                            || g_display_prefs.progressSetting as std::ffi::c_uint
                                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                    {
                        fprintf(
                            stderr,
                            b" ==> %2.f%%\0" as *const u8 as *const std::ffi::c_char,
                            cShare,
                        );
                    }
                }
            }
        }
        if directive as std::ffi::c_uint == ZSTD_e_end as std::ffi::c_int as std::ffi::c_uint {
            break;
        }
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64 && *readsize != fileSize {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1727 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                27 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Read error : Incomplete read : %llu / %llu B\0" as *const u8
                    as *const std::ffi::c_char,
                *readsize as std::ffi::c_ulonglong,
                fileSize as std::ffi::c_ulonglong,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(27 as std::ffi::c_int);
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ressPtr).writeCtx);
    compressedfilesize as std::ffi::c_ulonglong
}
unsafe extern "C" fn FIO_compressFilename_internal(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: cRess_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
    mut compressionLevel: std::ffi::c_int,
) -> std::ffi::c_int {
    let timeStart = UTIL_getTime();
    let cpuStart = clock();
    let mut readsize = 0 as std::ffi::c_int as U64;
    let mut compressedfilesize = 0 as std::ffi::c_int as U64;
    let fileSize = UTIL_getFileSize(srcFileName);
    if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
        fprintf(
            stderr,
            b"%s: %llu bytes \n\0" as *const u8 as *const std::ffi::c_char,
            srcFileName,
            fileSize as std::ffi::c_ulonglong,
        );
    }
    match (*prefs).compressionType as std::ffi::c_uint {
        1 => {
            compressedfilesize = FIO_compressGzFrame(
                &mut ress,
                srcFileName,
                fileSize,
                compressionLevel,
                &mut readsize,
            ) as U64;
        }
        2 | 3 => {
            compressedfilesize = FIO_compressLzmaFrame(
                &mut ress,
                srcFileName,
                fileSize,
                compressionLevel,
                &mut readsize,
                ((*prefs).compressionType as std::ffi::c_uint
                    == FIO_lzmaCompression as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int,
            ) as U64;
        }
        4 => {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    1789 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    20 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: file cannot be compressed as lz4 (zstd compiled without ZSTD_LZ4COMPRESS) -- ignored \n\0"
                        as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(20 as std::ffi::c_int);
        }
        0 | _ => {
            compressedfilesize = FIO_compressZstdFrame(
                fCtx,
                prefs,
                &mut ress,
                srcFileName,
                fileSize,
                compressionLevel,
                &mut readsize,
            ) as U64;
        }
    }
    (*fCtx).totalBytesInput = ((*fCtx).totalBytesInput).wrapping_add(readsize);
    (*fCtx).totalBytesOutput = ((*fCtx).totalBytesOutput).wrapping_add(compressedfilesize);
    if g_display_prefs.progressSetting as std::ffi::c_uint
        != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
        && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
            || g_display_prefs.progressSetting as std::ffi::c_uint
                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
    {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if FIO_shouldDisplayFileSummary(fCtx) != 0 {
        let mut hr_isize = UTIL_makeHumanReadableSize(readsize);
        let mut hr_osize = UTIL_makeHumanReadableSize(compressedfilesize);
        if readsize == 0 as std::ffi::c_int as U64 {
            if (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                || g_display_prefs.progressSetting as std::ffi::c_uint
                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"%-20s :  (%6.*f%s => %6.*f%s, %s) \n\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                    hr_isize.precision,
                    hr_isize.value,
                    hr_isize.suffix,
                    hr_osize.precision,
                    hr_osize.value,
                    hr_osize.suffix,
                    dstFileName,
                );
            }
        } else if (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
            || g_display_prefs.progressSetting as std::ffi::c_uint
                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
        {
            fprintf(
                stderr,
                b"%-20s :%6.2f%%   (%6.*f%s => %6.*f%s, %s) \n\0" as *const u8
                    as *const std::ffi::c_char,
                srcFileName,
                compressedfilesize as std::ffi::c_double / readsize as std::ffi::c_double
                    * 100 as std::ffi::c_int as std::ffi::c_double,
                hr_isize.precision,
                hr_isize.value,
                hr_isize.suffix,
                hr_osize.precision,
                hr_osize.value,
                hr_osize.suffix,
                dstFileName,
            );
        }
    }
    let cpuEnd = clock();
    let cpuLoad_s =
        (cpuEnd - cpuStart) as std::ffi::c_double / CLOCKS_PER_SEC as std::ffi::c_double;
    let timeLength_ns = UTIL_clockSpanNano(timeStart);
    let timeLength_s =
        timeLength_ns as std::ffi::c_double / 1000000000 as std::ffi::c_int as std::ffi::c_double;
    let cpuLoad_pct = cpuLoad_s / timeLength_s * 100 as std::ffi::c_int as std::ffi::c_double;
    if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
        fprintf(
            stderr,
            b"%-20s : Completed in %.2f sec  (cpu load : %.0f%%)\n\0" as *const u8
                as *const std::ffi::c_char,
            srcFileName,
            timeLength_s,
            cpuLoad_pct,
        );
    }
    0 as std::ffi::c_int
}
unsafe extern "C" fn FIO_compressFilename_dstFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: cRess_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
    mut srcFileStat: *const stat_t,
    mut compressionLevel: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut closeDstFile = 0 as std::ffi::c_int;
    let mut result: std::ffi::c_int = 0;
    let mut transferStat = 0 as std::ffi::c_int;
    let mut dstFd = -(1 as std::ffi::c_int);
    if !(AIO_ReadPool_getFile(ress.readCtx)).is_null() {
    } else {
        __assert_fail(
            b"AIO_ReadPool_getFile(ress.readCtx) != NULL\0" as *const u8
                as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            1852 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 129],
                &[std::ffi::c_char; 129],
            >(
                b"int FIO_compressFilename_dstFile(FIO_ctx_t *const, FIO_prefs_t *const, cRess_t, const char *, const char *, const stat_t *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_29026: {
        if !(AIO_ReadPool_getFile(ress.readCtx)).is_null() {
        } else {
            __assert_fail(
                b"AIO_ReadPool_getFile(ress.readCtx) != NULL\0" as *const u8
                    as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                1852 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 129],
                    &[std::ffi::c_char; 129],
                >(
                    b"int FIO_compressFilename_dstFile(FIO_ctx_t *const, FIO_prefs_t *const, cRess_t, const char *, const char *, const stat_t *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (AIO_WritePool_getFile(ress.writeCtx)).is_null() {
        let mut dstFileInitialPermissions = DEFAULT_FILE_PERMISSIONS;
        if strcmp(srcFileName, stdinmark.as_ptr()) != 0
            && strcmp(dstFileName, stdoutmark.as_ptr()) != 0
            && UTIL_isRegularFileStat(srcFileStat) != 0
        {
            transferStat = 1 as std::ffi::c_int;
            dstFileInitialPermissions = TEMPORARY_FILE_PERMISSIONS;
        }
        closeDstFile = 1 as std::ffi::c_int;
        if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
            fprintf(
                stderr,
                b"FIO_compressFilename_dstFile: opening dst: %s \n\0" as *const u8
                    as *const std::ffi::c_char,
                dstFileName,
            );
        }
        let mut dstFile = FIO_openDstFile(
            fCtx,
            prefs,
            srcFileName,
            dstFileName,
            dstFileInitialPermissions,
        );
        if dstFile.is_null() {
            return 1 as std::ffi::c_int;
        }
        dstFd = fileno(dstFile);
        AIO_WritePool_setFile(ress.writeCtx, dstFile);
        addHandler(dstFileName);
    }
    result = FIO_compressFilename_internal(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        compressionLevel,
    );
    if closeDstFile != 0 {
        clearHandler();
        if transferStat != 0 {
            UTIL_setFDStat(dstFd, dstFileName, srcFileStat);
        }
        if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
            fprintf(
                stderr,
                b"FIO_compressFilename_dstFile: closing dst: %s \n\0" as *const u8
                    as *const std::ffi::c_char,
                dstFileName,
            );
        }
        if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s \n\0" as *const u8 as *const std::ffi::c_char,
                    dstFileName,
                    strerror(*__errno_location()),
                );
            }
            result = 1 as std::ffi::c_int;
        }
        if transferStat != 0 {
            UTIL_utime(dstFileName, srcFileStat);
        }
        if result != 0 as std::ffi::c_int && strcmp(dstFileName, stdoutmark.as_ptr()) != 0 {
            FIO_removeFile(dstFileName);
        }
    }
    result
}
static mut compressedFileExtensions: [*const std::ffi::c_char; 114] = [
    ZSTD_EXTENSION.as_ptr(),
    TZSTD_EXTENSION.as_ptr(),
    GZ_EXTENSION.as_ptr(),
    TGZ_EXTENSION.as_ptr(),
    LZMA_EXTENSION.as_ptr(),
    XZ_EXTENSION.as_ptr(),
    TXZ_EXTENSION.as_ptr(),
    LZ4_EXTENSION.as_ptr(),
    TLZ4_EXTENSION.as_ptr(),
    b".7z\0" as *const u8 as *const std::ffi::c_char,
    b".aa3\0" as *const u8 as *const std::ffi::c_char,
    b".aac\0" as *const u8 as *const std::ffi::c_char,
    b".aar\0" as *const u8 as *const std::ffi::c_char,
    b".ace\0" as *const u8 as *const std::ffi::c_char,
    b".alac\0" as *const u8 as *const std::ffi::c_char,
    b".ape\0" as *const u8 as *const std::ffi::c_char,
    b".apk\0" as *const u8 as *const std::ffi::c_char,
    b".apng\0" as *const u8 as *const std::ffi::c_char,
    b".arc\0" as *const u8 as *const std::ffi::c_char,
    b".archive\0" as *const u8 as *const std::ffi::c_char,
    b".arj\0" as *const u8 as *const std::ffi::c_char,
    b".ark\0" as *const u8 as *const std::ffi::c_char,
    b".asf\0" as *const u8 as *const std::ffi::c_char,
    b".avi\0" as *const u8 as *const std::ffi::c_char,
    b".avif\0" as *const u8 as *const std::ffi::c_char,
    b".ba\0" as *const u8 as *const std::ffi::c_char,
    b".br\0" as *const u8 as *const std::ffi::c_char,
    b".bz2\0" as *const u8 as *const std::ffi::c_char,
    b".cab\0" as *const u8 as *const std::ffi::c_char,
    b".cdx\0" as *const u8 as *const std::ffi::c_char,
    b".chm\0" as *const u8 as *const std::ffi::c_char,
    b".cr2\0" as *const u8 as *const std::ffi::c_char,
    b".divx\0" as *const u8 as *const std::ffi::c_char,
    b".dmg\0" as *const u8 as *const std::ffi::c_char,
    b".dng\0" as *const u8 as *const std::ffi::c_char,
    b".docm\0" as *const u8 as *const std::ffi::c_char,
    b".docx\0" as *const u8 as *const std::ffi::c_char,
    b".dotm\0" as *const u8 as *const std::ffi::c_char,
    b".dotx\0" as *const u8 as *const std::ffi::c_char,
    b".dsft\0" as *const u8 as *const std::ffi::c_char,
    b".ear\0" as *const u8 as *const std::ffi::c_char,
    b".eftx\0" as *const u8 as *const std::ffi::c_char,
    b".emz\0" as *const u8 as *const std::ffi::c_char,
    b".eot\0" as *const u8 as *const std::ffi::c_char,
    b".epub\0" as *const u8 as *const std::ffi::c_char,
    b".f4v\0" as *const u8 as *const std::ffi::c_char,
    b".flac\0" as *const u8 as *const std::ffi::c_char,
    b".flv\0" as *const u8 as *const std::ffi::c_char,
    b".gho\0" as *const u8 as *const std::ffi::c_char,
    b".gif\0" as *const u8 as *const std::ffi::c_char,
    b".gifv\0" as *const u8 as *const std::ffi::c_char,
    b".gnp\0" as *const u8 as *const std::ffi::c_char,
    b".iso\0" as *const u8 as *const std::ffi::c_char,
    b".jar\0" as *const u8 as *const std::ffi::c_char,
    b".jpeg\0" as *const u8 as *const std::ffi::c_char,
    b".jpg\0" as *const u8 as *const std::ffi::c_char,
    b".jxl\0" as *const u8 as *const std::ffi::c_char,
    b".lz\0" as *const u8 as *const std::ffi::c_char,
    b".lzh\0" as *const u8 as *const std::ffi::c_char,
    b".m4a\0" as *const u8 as *const std::ffi::c_char,
    b".m4v\0" as *const u8 as *const std::ffi::c_char,
    b".mkv\0" as *const u8 as *const std::ffi::c_char,
    b".mov\0" as *const u8 as *const std::ffi::c_char,
    b".mp2\0" as *const u8 as *const std::ffi::c_char,
    b".mp3\0" as *const u8 as *const std::ffi::c_char,
    b".mp4\0" as *const u8 as *const std::ffi::c_char,
    b".mpa\0" as *const u8 as *const std::ffi::c_char,
    b".mpc\0" as *const u8 as *const std::ffi::c_char,
    b".mpe\0" as *const u8 as *const std::ffi::c_char,
    b".mpeg\0" as *const u8 as *const std::ffi::c_char,
    b".mpg\0" as *const u8 as *const std::ffi::c_char,
    b".mpl\0" as *const u8 as *const std::ffi::c_char,
    b".mpv\0" as *const u8 as *const std::ffi::c_char,
    b".msi\0" as *const u8 as *const std::ffi::c_char,
    b".odp\0" as *const u8 as *const std::ffi::c_char,
    b".ods\0" as *const u8 as *const std::ffi::c_char,
    b".odt\0" as *const u8 as *const std::ffi::c_char,
    b".ogg\0" as *const u8 as *const std::ffi::c_char,
    b".ogv\0" as *const u8 as *const std::ffi::c_char,
    b".otp\0" as *const u8 as *const std::ffi::c_char,
    b".ots\0" as *const u8 as *const std::ffi::c_char,
    b".ott\0" as *const u8 as *const std::ffi::c_char,
    b".pea\0" as *const u8 as *const std::ffi::c_char,
    b".png\0" as *const u8 as *const std::ffi::c_char,
    b".pptx\0" as *const u8 as *const std::ffi::c_char,
    b".qt\0" as *const u8 as *const std::ffi::c_char,
    b".rar\0" as *const u8 as *const std::ffi::c_char,
    b".s7z\0" as *const u8 as *const std::ffi::c_char,
    b".sfx\0" as *const u8 as *const std::ffi::c_char,
    b".sit\0" as *const u8 as *const std::ffi::c_char,
    b".sitx\0" as *const u8 as *const std::ffi::c_char,
    b".sqx\0" as *const u8 as *const std::ffi::c_char,
    b".svgz\0" as *const u8 as *const std::ffi::c_char,
    b".swf\0" as *const u8 as *const std::ffi::c_char,
    b".tbz2\0" as *const u8 as *const std::ffi::c_char,
    b".tib\0" as *const u8 as *const std::ffi::c_char,
    b".tlz\0" as *const u8 as *const std::ffi::c_char,
    b".vob\0" as *const u8 as *const std::ffi::c_char,
    b".war\0" as *const u8 as *const std::ffi::c_char,
    b".webm\0" as *const u8 as *const std::ffi::c_char,
    b".webp\0" as *const u8 as *const std::ffi::c_char,
    b".wma\0" as *const u8 as *const std::ffi::c_char,
    b".wmv\0" as *const u8 as *const std::ffi::c_char,
    b".woff\0" as *const u8 as *const std::ffi::c_char,
    b".woff2\0" as *const u8 as *const std::ffi::c_char,
    b".wvl\0" as *const u8 as *const std::ffi::c_char,
    b".xlsx\0" as *const u8 as *const std::ffi::c_char,
    b".xpi\0" as *const u8 as *const std::ffi::c_char,
    b".xps\0" as *const u8 as *const std::ffi::c_char,
    b".zip\0" as *const u8 as *const std::ffi::c_char,
    b".zipx\0" as *const u8 as *const std::ffi::c_char,
    b".zoo\0" as *const u8 as *const std::ffi::c_char,
    b".zpaq\0" as *const u8 as *const std::ffi::c_char,
    NULL as *const std::ffi::c_char,
];
unsafe extern "C" fn FIO_compressFilename_srcFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: cRess_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
    mut compressionLevel: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut result: std::ffi::c_int = 0;
    let mut srcFile = std::ptr::null_mut::<FILE>();
    let mut srcFileStat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut fileSize = UTIL_FILESIZE_UNKNOWN as U64;
    if g_display_prefs.displayLevel >= 6 as std::ffi::c_int {
        fprintf(
            stderr,
            b"FIO_compressFilename_srcFile: %s \n\0" as *const u8 as *const std::ffi::c_char,
            srcFileName,
        );
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 && UTIL_stat(srcFileName, &mut srcFileStat) != 0
    {
        if UTIL_isDirectoryStat(&mut srcFileStat) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s is a directory -- ignored \n\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                );
            }
            return 1 as std::ffi::c_int;
        }
        if !(ress.dictFileName).is_null()
            && UTIL_isSameFileStat(
                srcFileName,
                ress.dictFileName,
                &mut srcFileStat,
                &mut ress.dictFileStat,
            ) != 0
        {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: cannot use %s as an input file and dictionary \n\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                );
            }
            return 1 as std::ffi::c_int;
        }
    }
    if (*prefs).excludeCompressedFiles == 1 as std::ffi::c_int
        && UTIL_isCompressedFile(srcFileName, compressedFileExtensions.as_mut_ptr()) != 0
    {
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
            fprintf(
                stderr,
                b"File is already compressed : %s \n\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
            );
        }
        return 0 as std::ffi::c_int;
    }
    srcFile = FIO_openSrcFile(prefs, srcFileName, &mut srcFileStat);
    if srcFile.is_null() {
        return 1 as std::ffi::c_int;
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        fileSize = UTIL_getFileSizeStat(&mut srcFileStat);
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64
        && fileSize < (ZSTD_BLOCKSIZE_MAX * 3 as std::ffi::c_int) as U64
    {
        AIO_ReadPool_setAsync(ress.readCtx, 0 as std::ffi::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 0 as std::ffi::c_int);
    } else {
        AIO_ReadPool_setAsync(ress.readCtx, 1 as std::ffi::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 1 as std::ffi::c_int);
    }
    AIO_ReadPool_setFile(ress.readCtx, srcFile);
    result = FIO_compressFilename_dstFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        &mut srcFileStat,
        compressionLevel,
    );
    AIO_ReadPool_closeFile(ress.readCtx);
    if (*prefs).removeSrcFile != 0
        && result == 0 as std::ffi::c_int
        && strcmp(srcFileName, stdinmark.as_ptr()) != 0
    {
        clearHandler();
        if FIO_removeFile(srcFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2100 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    1 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(1 as std::ffi::c_int);
        }
    }
    result
}
unsafe extern "C" fn checked_index(
    mut options: *mut *const std::ffi::c_char,
    mut length: size_t,
    mut index: size_t,
) -> *const std::ffi::c_char {
    if index < length {
    } else {
        __assert_fail(
            b"index < length\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            2107 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 57], &[std::ffi::c_char; 57]>(
                b"const char *checked_index(const char **, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_14228: {
        if index < length {
        } else {
            __assert_fail(
                b"index < length\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2107 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 57], &[std::ffi::c_char; 57]>(
                    b"const char *checked_index(const char **, size_t, size_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
    *options.offset(index as isize)
}
#[no_mangle]
pub unsafe extern "C" fn FIO_displayCompressionParameters(mut prefs: *const FIO_prefs_t) {
    static mut formatOptions: [*const std::ffi::c_char; 5] = [
        ZSTD_EXTENSION.as_ptr(),
        GZ_EXTENSION.as_ptr(),
        XZ_EXTENSION.as_ptr(),
        LZMA_EXTENSION.as_ptr(),
        LZ4_EXTENSION.as_ptr(),
    ];
    static mut sparseOptions: [*const std::ffi::c_char; 3] = [
        b" --no-sparse\0" as *const u8 as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
        b" --sparse\0" as *const u8 as *const std::ffi::c_char,
    ];
    static mut checkSumOptions: [*const std::ffi::c_char; 3] = [
        b" --no-check\0" as *const u8 as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
        b" --check\0" as *const u8 as *const std::ffi::c_char,
    ];
    static mut rowMatchFinderOptions: [*const std::ffi::c_char; 3] = [
        b"\0" as *const u8 as *const std::ffi::c_char,
        b" --no-row-match-finder\0" as *const u8 as *const std::ffi::c_char,
        b" --row-match-finder\0" as *const u8 as *const std::ffi::c_char,
    ];
    static mut compressLiteralsOptions: [*const std::ffi::c_char; 3] = [
        b"\0" as *const u8 as *const std::ffi::c_char,
        b" --compress-literals\0" as *const u8 as *const std::ffi::c_char,
        b" --no-compress-literals\0" as *const u8 as *const std::ffi::c_char,
    ];
    if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
    } else {
        __assert_fail(
            b"g_display_prefs.displayLevel >= 4\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            2124 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 59], &[std::ffi::c_char; 59]>(
                b"void FIO_displayCompressionParameters(const FIO_prefs_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_14536: {
        if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
        } else {
            __assert_fail(
                b"g_display_prefs.displayLevel >= 4\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2124 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 59], &[std::ffi::c_char; 59]>(
                    b"void FIO_displayCompressionParameters(const FIO_prefs_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    fprintf(
        stderr,
        b"--format=%s\0" as *const u8 as *const std::ffi::c_char,
        *formatOptions
            .as_mut_ptr()
            .offset((*prefs).compressionType as isize),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        checked_index(
            sparseOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const std::ffi::c_char; 3]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong),
            (*prefs).sparseFileSupport as size_t,
        ),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        if (*prefs).dictIDFlag != 0 {
            b"\0" as *const u8 as *const std::ffi::c_char
        } else {
            b" --no-dictID\0" as *const u8 as *const std::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        checked_index(
            checkSumOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const std::ffi::c_char; 3]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong),
            (*prefs).checksumFlag as size_t,
        ),
    );
    fprintf(
        stderr,
        b" --jobsize=%d\0" as *const u8 as *const std::ffi::c_char,
        (*prefs).jobSize,
    );
    if (*prefs).adaptiveMode != 0 {
        fprintf(
            stderr,
            b" --adapt=min=%d,max=%d\0" as *const u8 as *const std::ffi::c_char,
            (*prefs).minAdaptLevel,
            (*prefs).maxAdaptLevel,
        );
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        checked_index(
            rowMatchFinderOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const std::ffi::c_char; 3]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong),
            (*prefs).useRowMatchFinder as size_t,
        ),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        if (*prefs).rsyncable != 0 {
            b" --rsyncable\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"\0" as *const u8 as *const std::ffi::c_char
        },
    );
    if (*prefs).streamSrcSize != 0 {
        fprintf(
            stderr,
            b" --stream-size=%u\0" as *const u8 as *const std::ffi::c_char,
            (*prefs).streamSrcSize as std::ffi::c_uint,
        );
    }
    if (*prefs).srcSizeHint != 0 {
        fprintf(
            stderr,
            b" --size-hint=%d\0" as *const u8 as *const std::ffi::c_char,
            (*prefs).srcSizeHint,
        );
    }
    if (*prefs).targetCBlockSize != 0 {
        fprintf(
            stderr,
            b" --target-compressed-block-size=%u\0" as *const u8 as *const std::ffi::c_char,
            (*prefs).targetCBlockSize as std::ffi::c_uint,
        );
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        checked_index(
            compressLiteralsOptions.as_mut_ptr(),
            (::core::mem::size_of::<[*const std::ffi::c_char; 3]>() as std::ffi::c_ulong)
                .wrapping_div(::core::mem::size_of::<*mut std::ffi::c_char>() as std::ffi::c_ulong),
            (*prefs).literalCompressionMode as size_t,
        ),
    );
    fprintf(
        stderr,
        b" --memory=%u\0" as *const u8 as *const std::ffi::c_char,
        if (*prefs).memLimit != 0 {
            (*prefs).memLimit
        } else {
            (128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int))
                as std::ffi::c_uint
        },
    );
    fprintf(
        stderr,
        b" --threads=%d\0" as *const u8 as *const std::ffi::c_char,
        (*prefs).nbWorkers,
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const std::ffi::c_char,
        if (*prefs).excludeCompressedFiles != 0 {
            b" --exclude-compressed\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"\0" as *const u8 as *const std::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b" --%scontent-size\0" as *const u8 as *const std::ffi::c_char,
        if (*prefs).contentSize != 0 {
            b"\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"no-\0" as *const u8 as *const std::ffi::c_char
        },
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn FIO_compressFilename(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
    mut dictFileName: *const std::ffi::c_char,
    mut compressionLevel: std::ffi::c_int,
    mut comprParams: ZSTD_compressionParameters,
) -> std::ffi::c_int {
    let mut ress = FIO_createCResources(
        prefs,
        dictFileName,
        UTIL_getFileSize(srcFileName) as std::ffi::c_ulonglong,
        compressionLevel,
        comprParams,
    );
    let result = FIO_compressFilename_srcFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        compressionLevel,
    );
    FIO_freeCResources(&mut ress);
    result
}
unsafe extern "C" fn FIO_determineCompressedName(
    mut srcFileName: *const std::ffi::c_char,
    mut outDirName: *const std::ffi::c_char,
    mut suffix: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    static mut dfnbCapacity: size_t = 0 as std::ffi::c_int as size_t;
    static mut dstFileNameBuffer: *mut std::ffi::c_char = NULL as *mut std::ffi::c_char;
    let mut outDirFilename = NULL as *mut std::ffi::c_char;
    let mut sfnSize = strlen(srcFileName);
    let srcSuffixLen = strlen(suffix);
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        return stdoutmark.as_ptr();
    }
    if !outDirName.is_null() {
        outDirFilename = FIO_createFilename_fromOutDir(srcFileName, outDirName, srcSuffixLen);
        sfnSize = strlen(outDirFilename);
        if !outDirFilename.is_null() {
        } else {
            __assert_fail(
                b"outDirFilename != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2185 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[std::ffi::c_char; 82],
                >(
                    b"const char *FIO_determineCompressedName(const char *, const char *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_37773: {
            if !outDirFilename.is_null() {
            } else {
                __assert_fail(
                    b"outDirFilename != NULL\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2185 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[std::ffi::c_char; 82],
                    >(
                        b"const char *FIO_determineCompressedName(const char *, const char *, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    if dfnbCapacity
        <= sfnSize
            .wrapping_add(srcSuffixLen)
            .wrapping_add(1 as std::ffi::c_int as size_t)
    {
        free(dstFileNameBuffer as *mut std::ffi::c_void);
        dfnbCapacity = sfnSize
            .wrapping_add(srcSuffixLen)
            .wrapping_add(30 as std::ffi::c_int as size_t);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut std::ffi::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2194 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    30 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s\0" as *const u8 as *const std::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(30 as std::ffi::c_int);
        }
    }
    if !dstFileNameBuffer.is_null() {
    } else {
        __assert_fail(
            b"dstFileNameBuffer != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            2197 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[std::ffi::c_char; 82],
            >(
                b"const char *FIO_determineCompressedName(const char *, const char *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_37568: {
        if !dstFileNameBuffer.is_null() {
        } else {
            __assert_fail(
                b"dstFileNameBuffer != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2197 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[std::ffi::c_char; 82],
                >(
                    b"const char *FIO_determineCompressedName(const char *, const char *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !outDirFilename.is_null() {
        memcpy(
            dstFileNameBuffer as *mut std::ffi::c_void,
            outDirFilename as *const std::ffi::c_void,
            sfnSize,
        );
        free(outDirFilename as *mut std::ffi::c_void);
    } else {
        memcpy(
            dstFileNameBuffer as *mut std::ffi::c_void,
            srcFileName as *const std::ffi::c_void,
            sfnSize,
        );
    }
    memcpy(
        dstFileNameBuffer.offset(sfnSize as isize) as *mut std::ffi::c_void,
        suffix as *const std::ffi::c_void,
        srcSuffixLen.wrapping_add(1 as std::ffi::c_int as size_t),
    );
    dstFileNameBuffer
}
unsafe extern "C" fn FIO_getLargestFileSize(
    mut inFileNames: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_uint,
) -> std::ffi::c_ulonglong {
    let mut i: size_t = 0;
    let mut fileSize: std::ffi::c_ulonglong = 0;
    let mut maxFileSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    i = 0 as std::ffi::c_int as size_t;
    while i < nbFiles as size_t {
        fileSize = UTIL_getFileSize(*inFileNames.offset(i as isize)) as std::ffi::c_ulonglong;
        maxFileSize = if fileSize > maxFileSize {
            fileSize
        } else {
            maxFileSize
        };
        i = i.wrapping_add(1);
        i;
    }
    maxFileSize
}
#[no_mangle]
pub unsafe extern "C" fn FIO_compressMultipleFilenames(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut inFileNamesTable: *mut *const std::ffi::c_char,
    mut outMirroredRootDirName: *const std::ffi::c_char,
    mut outDirName: *const std::ffi::c_char,
    mut outFileName: *const std::ffi::c_char,
    mut suffix: *const std::ffi::c_char,
    mut dictFileName: *const std::ffi::c_char,
    mut compressionLevel: std::ffi::c_int,
    mut comprParams: ZSTD_compressionParameters,
) -> std::ffi::c_int {
    let mut status: std::ffi::c_int = 0;
    let mut error = 0 as std::ffi::c_int;
    let mut ress = FIO_createCResources(
        prefs,
        dictFileName,
        FIO_getLargestFileSize(inFileNamesTable, (*fCtx).nbFilesTotal as std::ffi::c_uint),
        compressionLevel,
        comprParams,
    );
    if !outFileName.is_null() || !suffix.is_null() {
    } else {
        __assert_fail(
            b"outFileName != NULL || suffix != NULL\0" as *const u8
                as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            2242 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 190],
                &[std::ffi::c_char; 190],
            >(
                b"int FIO_compressMultipleFilenames(FIO_ctx_t *const, FIO_prefs_t *const, const char **, const char *, const char *, const char *, const char *, const char *, int, ZSTD_compressionParameters)\0",
            ))
                .as_ptr(),
        );
    }
    'c_38973: {
        if !outFileName.is_null() || !suffix.is_null() {
        } else {
            __assert_fail(
                b"outFileName != NULL || suffix != NULL\0" as *const u8
                    as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2242 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 190],
                    &[std::ffi::c_char; 190],
                >(
                    b"int FIO_compressMultipleFilenames(FIO_ctx_t *const, FIO_prefs_t *const, const char **, const char *, const char *, const char *, const char *, const char *, int, ZSTD_compressionParameters)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !outFileName.is_null() {
        let mut dstFile = std::ptr::null_mut::<FILE>();
        if FIO_multiFilesConcatWarning(fCtx, prefs, outFileName, 1 as std::ffi::c_int) != 0 {
            FIO_freeCResources(&mut ress);
            return 1 as std::ffi::c_int;
        }
        dstFile = FIO_openDstFile(
            fCtx,
            prefs,
            NULL as *const std::ffi::c_char,
            outFileName,
            DEFAULT_FILE_PERMISSIONS,
        );
        if dstFile.is_null() {
            error = 1 as std::ffi::c_int;
        } else {
            AIO_WritePool_setFile(ress.writeCtx, dstFile);
            while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
                status = FIO_compressFilename_srcFile(
                    fCtx,
                    prefs,
                    ress,
                    outFileName,
                    *inFileNamesTable.offset((*fCtx).currFileIdx as isize),
                    compressionLevel,
                );
                if status == 0 {
                    (*fCtx).nbFilesProcessed += 1;
                    (*fCtx).nbFilesProcessed;
                }
                error |= status;
                (*fCtx).currFileIdx += 1;
                (*fCtx).currFileIdx;
            }
            if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const std::ffi::c_char,
                        b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                        2261 as std::ffi::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                        29 as std::ffi::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Write error (%s) : cannot properly close %s\0" as *const u8
                            as *const std::ffi::c_char,
                        strerror(*__errno_location()),
                        outFileName,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                }
                exit(29 as std::ffi::c_int);
            }
        }
    } else {
        if !outMirroredRootDirName.is_null() {
            UTIL_mirrorSourceFilesDirectories(
                inFileNamesTable,
                (*fCtx).nbFilesTotal as std::ffi::c_uint,
                outMirroredRootDirName,
            );
        }
        let mut current_block_63: u64;
        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            let srcFileName = *inFileNamesTable.offset((*fCtx).currFileIdx as isize);
            let mut dstFileName = NULL as *const std::ffi::c_char;
            if !outMirroredRootDirName.is_null() {
                let mut validMirroredDirName =
                    UTIL_createMirroredDestDirName(srcFileName, outMirroredRootDirName);
                if !validMirroredDirName.is_null() {
                    dstFileName =
                        FIO_determineCompressedName(srcFileName, validMirroredDirName, suffix);
                    free(validMirroredDirName as *mut std::ffi::c_void);
                    current_block_63 = 5892776923941496671;
                } else {
                    if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"zstd: --output-dir-mirror cannot compress '%s' into '%s' \n\0"
                                as *const u8 as *const std::ffi::c_char,
                            srcFileName,
                            outMirroredRootDirName,
                        );
                    }
                    error = 1 as std::ffi::c_int;
                    current_block_63 = 15090052786889560393;
                }
            } else {
                dstFileName = FIO_determineCompressedName(srcFileName, outDirName, suffix);
                current_block_63 = 5892776923941496671;
            }
            if current_block_63 == 5892776923941496671 {
                status = FIO_compressFilename_srcFile(
                    fCtx,
                    prefs,
                    ress,
                    dstFileName,
                    srcFileName,
                    compressionLevel,
                );
                if status == 0 {
                    (*fCtx).nbFilesProcessed += 1;
                    (*fCtx).nbFilesProcessed;
                }
                error |= status;
            }
            (*fCtx).currFileIdx += 1;
            (*fCtx).currFileIdx;
        }
        if !outDirName.is_null() {
            FIO_checkFilenameCollisions(inFileNamesTable, (*fCtx).nbFilesTotal as std::ffi::c_uint);
        }
    }
    if FIO_shouldDisplayMultipleFileSummary(fCtx) != 0 {
        let mut hr_isize = UTIL_makeHumanReadableSize((*fCtx).totalBytesInput);
        let mut hr_osize = UTIL_makeHumanReadableSize((*fCtx).totalBytesOutput);
        if g_display_prefs.progressSetting as std::ffi::c_uint
            != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                || g_display_prefs.progressSetting as std::ffi::c_uint
                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
        {
            fprintf(
                stderr,
                b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if (*fCtx).totalBytesInput == 0 as std::ffi::c_int as size_t {
            if (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                || g_display_prefs.progressSetting as std::ffi::c_uint
                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"%3d files compressed : (%6.*f%4s => %6.*f%4s)\n\0" as *const u8
                        as *const std::ffi::c_char,
                    (*fCtx).nbFilesProcessed,
                    hr_isize.precision,
                    hr_isize.value,
                    hr_isize.suffix,
                    hr_osize.precision,
                    hr_osize.value,
                    hr_osize.suffix,
                );
            }
        } else if (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
            || g_display_prefs.progressSetting as std::ffi::c_uint
                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
        {
            fprintf(
                stderr,
                b"%3d files compressed : %.2f%% (%6.*f%4s => %6.*f%4s)\n\0" as *const u8
                    as *const std::ffi::c_char,
                (*fCtx).nbFilesProcessed,
                (*fCtx).totalBytesOutput as std::ffi::c_double
                    / (*fCtx).totalBytesInput as std::ffi::c_double
                    * 100 as std::ffi::c_int as std::ffi::c_double,
                hr_isize.precision,
                hr_isize.value,
                hr_isize.suffix,
                hr_osize.precision,
                hr_osize.value,
                hr_osize.suffix,
            );
        }
    }
    FIO_freeCResources(&mut ress);
    error
}
unsafe extern "C" fn FIO_createDResources(
    prefs: *mut FIO_prefs_t,
    mut dictFileName: *const std::ffi::c_char,
) -> dRess_t {
    let mut useMMap = ((*prefs).mmapDict as std::ffi::c_uint
        == ZSTD_ps_enable as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int;
    let mut forceNoUseMMap = ((*prefs).mmapDict as std::ffi::c_uint
        == ZSTD_ps_disable as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_int;
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut ress = dRess_t {
        dict: FIO_Dict_t {
            dictBuffer: std::ptr::null_mut::<std::ffi::c_void>(),
            dictBufferSize: 0,
            dictBufferType: FIO_mallocDict,
        },
        dctx: std::ptr::null_mut::<ZSTD_DStream>(),
        writeCtx: std::ptr::null_mut::<WritePoolCtx_t>(),
        readCtx: std::ptr::null_mut::<ReadPoolCtx_t>(),
    };
    memset(
        &mut statbuf as *mut stat_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<stat_t>() as std::ffi::c_ulong,
    );
    memset(
        &mut ress as *mut dRess_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<dRess_t>() as std::ffi::c_ulong,
    );
    FIO_getDictFileStat(dictFileName, &mut statbuf);
    if (*prefs).patchFromMode != 0 {
        let dictSize = UTIL_getFileSizeStat(&mut statbuf);
        useMMap |= (dictSize > (*prefs).memLimit as U64) as std::ffi::c_int;
        FIO_adjustMemLimitForPatchFromMode(
            prefs,
            dictSize as std::ffi::c_ulonglong,
            0 as std::ffi::c_int as std::ffi::c_ulonglong,
        );
    }
    ress.dctx = ZSTD_createDStream();
    if (ress.dctx).is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2351 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                60 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error: %s : can't create ZSTD_DStream\0" as *const u8 as *const std::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(60 as std::ffi::c_int);
    }
    let mut err: size_t = 0;
    err = ZSTD_DCtx_setMaxWindowSize(ress.dctx, (*prefs).memLimit as size_t);
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_DCtx_setMaxWindowSize(ress.dctx, prefs->memLimit)\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2352 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut err_0: size_t = 0;
    err_0 = ZSTD_DCtx_setParameter(
        ress.dctx,
        ZSTD_d_experimentalParam3,
        ((*prefs).checksumFlag == 0) as std::ffi::c_int,
    );
    if ZSTD_isError(err_0) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_DCtx_setParameter(ress.dctx, ZSTD_d_experimentalParam3, !prefs->checksumFlag)\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2353 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_0),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    let mut dictBufferType = (if useMMap != 0 && forceNoUseMMap == 0 {
        FIO_mmapDict as std::ffi::c_int
    } else {
        FIO_mallocDict as std::ffi::c_int
    }) as FIO_dictBufferType_t;
    FIO_initDict(
        &mut ress.dict,
        dictFileName,
        prefs,
        &mut statbuf,
        dictBufferType,
    );
    let mut err_1: size_t = 0;
    err_1 = ZSTD_DCtx_reset(ress.dctx, ZSTD_reset_session_only);
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_DCtx_reset(ress.dctx, ZSTD_reset_session_only)\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2360 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    if (*prefs).patchFromMode != 0 {
        let mut err_2: size_t = 0;
        err_2 = ZSTD_DCtx_refPrefix(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize);
        if ZSTD_isError(err_2) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_DCtx_refPrefix(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2363 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_2),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    } else {
        let mut err_3: size_t = 0;
        err_3 = ZSTD_DCtx_loadDictionary_byReference(
            ress.dctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_3) != 0 {
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                    b"ZSTD_DCtx_loadDictionary_byReference(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2365 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const std::ffi::c_char,
                    ZSTD_getErrorName(err_3),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(11 as std::ffi::c_int);
        }
    }
    ress.writeCtx = AIO_WritePool_create(prefs, ZSTD_DStreamOutSize());
    ress.readCtx = AIO_ReadPool_create(prefs, ZSTD_DStreamInSize());
    ress
}
unsafe extern "C" fn FIO_freeDResources(mut ress: dRess_t) {
    FIO_freeDict(&mut ress.dict);
    let mut err: size_t = 0;
    err = ZSTD_freeDStream(ress.dctx);
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const std::ffi::c_char,
                b"ZSTD_freeDStream(ress.dctx)\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2377 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                11 as std::ffi::c_int,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const std::ffi::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        exit(11 as std::ffi::c_int);
    }
    AIO_WritePool_free(ress.writeCtx);
    AIO_ReadPool_free(ress.readCtx);
}
unsafe extern "C" fn FIO_passThrough(mut ress: *mut dRess_t) -> std::ffi::c_int {
    let blockSize = if (if ((64 as std::ffi::c_int
        * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as size_t)
        < ZSTD_DStreamInSize()
    {
        (64 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as size_t
    } else {
        ZSTD_DStreamInSize()
    }) < ZSTD_DStreamOutSize()
    {
        if ((64 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as size_t)
            < ZSTD_DStreamInSize()
        {
            (64 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as size_t
        } else {
            ZSTD_DStreamInSize()
        }
    } else {
        ZSTD_DStreamOutSize()
    };
    let mut writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    AIO_ReadPool_fillBuffer((*ress).readCtx, blockSize);
    while (*(*ress).readCtx).srcBufferLoaded != 0 {
        let mut writeSize: size_t = 0;
        writeSize = if blockSize < (*(*ress).readCtx).srcBufferLoaded {
            blockSize
        } else {
            (*(*ress).readCtx).srcBufferLoaded
        };
        if writeSize <= (*writeJob).bufferSize {
        } else {
            __assert_fail(
                b"writeSize <= writeJob->bufferSize\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2393 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                    b"int FIO_passThrough(dRess_t *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_31836: {
            if writeSize <= (*writeJob).bufferSize {
            } else {
                __assert_fail(
                    b"writeSize <= writeJob->bufferSize\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2393 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                        b"int FIO_passThrough(dRess_t *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        memcpy(
            (*writeJob).buffer,
            (*(*ress).readCtx).srcBuffer as *const std::ffi::c_void,
            writeSize,
        );
        (*writeJob).usedBufferSize = writeSize;
        AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
        AIO_ReadPool_consumeBytes((*ress).readCtx, writeSize);
        AIO_ReadPool_fillBuffer((*ress).readCtx, blockSize);
    }
    if (*(*ress).readCtx).reachedEof != 0 {
    } else {
        __assert_fail(
            b"ress->readCtx->reachedEof\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            2400 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                b"int FIO_passThrough(dRess_t *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_31671: {
        if (*(*ress).readCtx).reachedEof != 0 {
        } else {
            __assert_fail(
                b"ress->readCtx->reachedEof\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2400 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 31], &[std::ffi::c_char; 31]>(
                    b"int FIO_passThrough(dRess_t *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    0 as std::ffi::c_int
}
unsafe extern "C" fn FIO_zstdErrorHelp(
    prefs: *const FIO_prefs_t,
    mut ress: *const dRess_t,
    mut err: size_t,
    mut srcFileName: *const std::ffi::c_char,
) {
    let mut header = ZSTD_FrameHeader {
        frameContentSize: 0,
        windowSize: 0,
        blockSizeMax: 0,
        frameType: ZSTD_frame,
        headerSize: 0,
        dictID: 0,
        checksumFlag: 0,
        _reserved1: 0,
        _reserved2: 0,
    };
    if ZSTD_getErrorCode(err) as std::ffi::c_uint
        != ZSTD_error_frameParameter_windowTooLarge as std::ffi::c_int as std::ffi::c_uint
    {
        return;
    }
    err = ZSTD_getFrameHeader(
        &mut header,
        (*(*ress).readCtx).srcBuffer as *const std::ffi::c_void,
        (*(*ress).readCtx).srcBufferLoaded,
    );
    if err == 0 as std::ffi::c_int as size_t {
        let windowSize = header.windowSize;
        let windowLog = (FIO_highbit64(windowSize)).wrapping_add(
            (windowSize & windowSize.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulonglong)
                != 0 as std::ffi::c_int as std::ffi::c_ulonglong) as std::ffi::c_int
                as std::ffi::c_uint,
        );
        if (*prefs).memLimit > 0 as std::ffi::c_int as std::ffi::c_uint {
        } else {
            __assert_fail(
                b"prefs->memLimit > 0\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2425 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 88],
                    &[std::ffi::c_char; 88],
                >(
                    b"void FIO_zstdErrorHelp(const FIO_prefs_t *const, const dRess_t *, size_t, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_33699: {
            if (*prefs).memLimit > 0 as std::ffi::c_int as std::ffi::c_uint {
            } else {
                __assert_fail(
                    b"prefs->memLimit > 0\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2425 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 88],
                        &[std::ffi::c_char; 88],
                    >(
                        b"void FIO_zstdErrorHelp(const FIO_prefs_t *const, const dRess_t *, size_t, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"%s : Window size larger than maximum : %llu > %u \n\0" as *const u8
                    as *const std::ffi::c_char,
                srcFileName,
                windowSize,
                (*prefs).memLimit,
            );
        }
        if windowLog
            <= (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 4 as std::ffi::c_int as std::ffi::c_ulong
            {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) as std::ffi::c_uint
        {
            let windowMB = (windowSize >> 20 as std::ffi::c_int).wrapping_add(
                (windowSize
                    & (1 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int)
                        - 1 as std::ffi::c_int) as std::ffi::c_ulonglong
                    != 0 as std::ffi::c_int as std::ffi::c_ulonglong)
                    as std::ffi::c_int as std::ffi::c_ulonglong,
            ) as std::ffi::c_uint;
            if windowSize
                < ((1 as std::ffi::c_ulonglong) << 52 as std::ffi::c_int) as U64
                    as std::ffi::c_ulonglong
            {
            } else {
                __assert_fail(
                    b"windowSize < (U64)(1ULL << 52)\0" as *const u8
                        as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    2430 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 88],
                        &[std::ffi::c_char; 88],
                    >(
                        b"void FIO_zstdErrorHelp(const FIO_prefs_t *const, const dRess_t *, size_t, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_33616: {
                if windowSize
                    < ((1 as std::ffi::c_ulonglong) << 52 as std::ffi::c_int) as U64
                        as std::ffi::c_ulonglong
                {
                } else {
                    __assert_fail(
                        b"windowSize < (U64)(1ULL << 52)\0" as *const u8
                            as *const std::ffi::c_char,
                        b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                        2430 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 88],
                            &[std::ffi::c_char; 88],
                        >(
                            b"void FIO_zstdErrorHelp(const FIO_prefs_t *const, const dRess_t *, size_t, const char *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s : Use --long=%u or --memory=%uMB \n\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                    windowLog,
                    windowMB,
                );
            }
            return;
        }
    }
    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
        fprintf(
            stderr,
            b"%s : Window log larger than ZSTD_WINDOWLOG_MAX=%u; not supported \n\0" as *const u8
                as *const std::ffi::c_char,
            srcFileName,
            if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 4 as std::ffi::c_int as std::ffi::c_ulong
            {
                30 as std::ffi::c_int
            } else {
                31 as std::ffi::c_int
            },
        );
    }
}
pub const FIO_ERROR_FRAME_DECODING: std::ffi::c_int = -(2 as std::ffi::c_int);
unsafe extern "C" fn FIO_decompressZstdFrame(
    fCtx: *mut FIO_ctx_t,
    mut ress: *mut dRess_t,
    prefs: *const FIO_prefs_t,
    mut srcFileName: *const std::ffi::c_char,
    mut alreadyDecoded: U64,
) -> std::ffi::c_ulonglong {
    let mut frameSize = 0 as std::ffi::c_int as U64;
    let mut srcFName20 = srcFileName;
    let mut writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    if !writeJob.is_null() {
    } else {
        __assert_fail(
            b"writeJob\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            2452 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[std::ffi::c_char; 117],
            >(
                b"unsigned long long FIO_decompressZstdFrame(FIO_ctx_t *const, dRess_t *, const FIO_prefs_t *const, const char *, U64)\0",
            ))
                .as_ptr(),
        );
    }
    'c_33852: {
        if !writeJob.is_null() {
        } else {
            __assert_fail(
                b"writeJob\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2452 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 117],
                    &[std::ffi::c_char; 117],
                >(
                    b"unsigned long long FIO_decompressZstdFrame(FIO_ctx_t *const, dRess_t *, const FIO_prefs_t *const, const char *, U64)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let srcFileLength = strlen(srcFileName);
    if srcFileLength > 20 as std::ffi::c_int as size_t
        && g_display_prefs.displayLevel < 3 as std::ffi::c_int
    {
        srcFName20 =
            srcFName20.offset(srcFileLength.wrapping_sub(20 as std::ffi::c_int as size_t) as isize);
    }
    ZSTD_DCtx_reset((*ress).dctx, ZSTD_reset_session_only);
    AIO_ReadPool_fillBuffer((*ress).readCtx, ZSTD_FRAMEHEADERSIZE_MAX as size_t);
    loop {
        let mut inBuff = setInBuffer(
            (*(*ress).readCtx).srcBuffer as *const std::ffi::c_void,
            (*(*ress).readCtx).srcBufferLoaded,
            0 as std::ffi::c_int as size_t,
        );
        let mut outBuff = setOutBuffer(
            (*writeJob).buffer,
            (*writeJob).bufferSize,
            0 as std::ffi::c_int as size_t,
        );
        let readSizeHint = ZSTD_decompressStream((*ress).dctx, &mut outBuff, &mut inBuff);
        let hrs = UTIL_makeHumanReadableSize(alreadyDecoded.wrapping_add(frameSize));
        if ZSTD_isError(readSizeHint) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s : Decoding error (36) : %s \n\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                    ZSTD_getErrorName(readSizeHint),
                );
            }
            FIO_zstdErrorHelp(prefs, ress, readSizeHint, srcFileName);
            AIO_WritePool_releaseIoJob(writeJob);
            return FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong;
        }
        (*writeJob).usedBufferSize = outBuff.pos;
        AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
        frameSize = (frameSize as std::ffi::c_ulong).wrapping_add(outBuff.pos) as U64 as U64;
        if (*fCtx).nbFilesTotal > 1 as std::ffi::c_int {
            if g_display_prefs.progressSetting as std::ffi::c_uint
                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                    || g_display_prefs.progressSetting as std::ffi::c_uint
                        == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
                && g_display_prefs.progressSetting as std::ffi::c_uint
                    != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4 as std::ffi::c_int)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"\rDecompress: %2u/%2u files. Current: %s : %.*f%s...    \0" as *const u8
                        as *const std::ffi::c_char,
                    (*fCtx).currFileIdx + 1 as std::ffi::c_int,
                    (*fCtx).nbFilesTotal,
                    srcFName20,
                    hrs.precision,
                    hrs.value,
                    hrs.suffix,
                );
                if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                    fflush(stderr);
                }
            }
        } else if g_display_prefs.progressSetting as std::ffi::c_uint
            != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                || g_display_prefs.progressSetting as std::ffi::c_uint
                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
            && g_display_prefs.progressSetting as std::ffi::c_uint
                != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                || g_display_prefs.displayLevel >= 4 as std::ffi::c_int)
        {
            g_displayClock = UTIL_getTime();
            fprintf(
                stderr,
                b"\r%-20.20s : %.*f%s...     \0" as *const u8 as *const std::ffi::c_char,
                srcFName20,
                hrs.precision,
                hrs.value,
                hrs.suffix,
            );
            if g_display_prefs.displayLevel >= 4 as std::ffi::c_int {
                fflush(stderr);
            }
        }
        AIO_ReadPool_consumeBytes((*ress).readCtx, inBuff.pos);
        if readSizeHint == 0 as std::ffi::c_int as size_t {
            break;
        }
        let toDecode = if readSizeHint < ZSTD_DStreamInSize() {
            readSizeHint
        } else {
            ZSTD_DStreamInSize()
        };
        if (*(*ress).readCtx).srcBufferLoaded < toDecode {
            let readSize = AIO_ReadPool_fillBuffer((*ress).readCtx, toDecode);
            if readSize == 0 as std::ffi::c_int as size_t {
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"%s : Read error (39) : premature end \n\0" as *const u8
                            as *const std::ffi::c_char,
                        srcFileName,
                    );
                }
                AIO_WritePool_releaseIoJob(writeJob);
                return FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong;
            }
        }
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    frameSize as std::ffi::c_ulonglong
}
unsafe extern "C" fn FIO_decompressGzFrame(
    mut ress: *mut dRess_t,
    mut srcFileName: *const std::ffi::c_char,
) -> std::ffi::c_ulonglong {
    let mut outFileSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    let mut strm = z_stream_s {
        next_in: std::ptr::null_mut::<Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: std::ptr::null_mut::<Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: std::ptr::null_mut::<std::ffi::c_char>(),
        state: std::ptr::null_mut::<internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: std::ptr::null_mut::<std::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut flush = Z_NO_FLUSH;
    let mut decodingError = 0 as std::ffi::c_int;
    let mut writeJob = NULL as *mut IOJob_t;
    strm.zalloc = ::core::mem::transmute::<libc::intptr_t, alloc_func>(Z_NULL as libc::intptr_t);
    strm.zfree = ::core::mem::transmute::<libc::intptr_t, free_func>(Z_NULL as libc::intptr_t);
    strm.opaque = Z_NULL as voidpf;
    strm.next_in = std::ptr::null_mut::<Bytef>();
    strm.avail_in = 0 as std::ffi::c_int as uInt;
    if inflateInit2_(
        &mut strm,
        15 as std::ffi::c_int + 16 as std::ffi::c_int,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as std::ffi::c_ulong as std::ffi::c_int,
    ) != Z_OK
    {
        return FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong;
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_out = (*writeJob).buffer as *mut Bytef;
    strm.avail_out = (*writeJob).bufferSize as uInt;
    strm.avail_in = (*(*ress).readCtx).srcBufferLoaded as uInt;
    strm.next_in = (*(*ress).readCtx).srcBuffer as *mut std::ffi::c_uchar;
    loop {
        let mut ret: std::ffi::c_int = 0;
        if strm.avail_in == 0 as std::ffi::c_int as uInt {
            AIO_ReadPool_consumeAndRefill((*ress).readCtx);
            if (*(*ress).readCtx).srcBufferLoaded == 0 as std::ffi::c_int as size_t {
                flush = Z_FINISH;
            }
            strm.next_in = (*(*ress).readCtx).srcBuffer as *mut std::ffi::c_uchar;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded as uInt;
        }
        ret = inflate(&mut strm, flush);
        if ret == Z_BUF_ERROR {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: premature gz end \n\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                );
            }
            decodingError = 1 as std::ffi::c_int;
            break;
        } else if ret != Z_OK && ret != Z_STREAM_END {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: inflate error %d \n\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                    ret,
                );
            }
            decodingError = 1 as std::ffi::c_int;
            break;
        } else {
            let decompBytes = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out as size_t);
            if decompBytes != 0 {
                (*writeJob).usedBufferSize = decompBytes;
                AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
                outFileSize = outFileSize.wrapping_add(decompBytes as std::ffi::c_ulonglong);
                strm.next_out = (*writeJob).buffer as *mut Bytef;
                strm.avail_out = (*writeJob).bufferSize as uInt;
            }
            if ret == Z_STREAM_END {
                break;
            }
        }
    }
    AIO_ReadPool_consumeBytes(
        (*ress).readCtx,
        ((*(*ress).readCtx).srcBufferLoaded).wrapping_sub(strm.avail_in as size_t),
    );
    if inflateEnd(&mut strm) != Z_OK && decodingError == 0 as std::ffi::c_int {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: inflateEnd error \n\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
            );
        }
        decodingError = 1 as std::ffi::c_int;
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    if decodingError != 0 {
        FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong
    } else {
        outFileSize
    }
}
unsafe extern "C" fn FIO_decompressLzmaFrame(
    mut ress: *mut dRess_t,
    mut srcFileName: *const std::ffi::c_char,
    mut plain_lzma: std::ffi::c_int,
) -> std::ffi::c_ulonglong {
    let mut outFileSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    let mut strm = {
        lzma_stream {
            next_in: NULL as *const uint8_t,
            avail_in: 0 as std::ffi::c_int as size_t,
            total_in: 0 as std::ffi::c_int as uint64_t,
            next_out: NULL as *mut uint8_t,
            avail_out: 0 as std::ffi::c_int as size_t,
            total_out: 0 as std::ffi::c_int as uint64_t,
            allocator: NULL as *const lzma_allocator,
            internal: NULL as *mut lzma_internal,
            reserved_ptr1: NULL as *mut std::ffi::c_void,
            reserved_ptr2: NULL as *mut std::ffi::c_void,
            reserved_ptr3: NULL as *mut std::ffi::c_void,
            reserved_ptr4: NULL as *mut std::ffi::c_void,
            seek_pos: 0 as std::ffi::c_int as uint64_t,
            reserved_int2: 0 as std::ffi::c_int as uint64_t,
            reserved_int3: 0 as std::ffi::c_int as size_t,
            reserved_int4: 0 as std::ffi::c_int as size_t,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
        }
    };
    let mut action = LZMA_RUN;
    let mut initRet = LZMA_OK;
    let mut decodingError = 0 as std::ffi::c_int;
    let mut writeJob = NULL as *mut IOJob_t;
    strm.next_in = std::ptr::null::<uint8_t>();
    strm.avail_in = 0 as std::ffi::c_int as size_t;
    if plain_lzma != 0 {
        initRet = lzma_alone_decoder(&mut strm, UINT64_MAX);
    } else {
        initRet = lzma_stream_decoder(&mut strm, UINT64_MAX, 0 as std::ffi::c_int as uint32_t);
    }
    if initRet as std::ffi::c_uint != LZMA_OK as std::ffi::c_int as std::ffi::c_uint {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: %s error %d \n\0" as *const u8 as *const std::ffi::c_char,
                if plain_lzma != 0 {
                    b"lzma_alone_decoder\0" as *const u8 as *const std::ffi::c_char
                } else {
                    b"lzma_stream_decoder\0" as *const u8 as *const std::ffi::c_char
                },
                srcFileName,
                initRet as std::ffi::c_uint,
            );
        }
        return FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong;
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_out = (*writeJob).buffer as *mut BYTE;
    strm.avail_out = (*writeJob).bufferSize;
    strm.next_in = (*(*ress).readCtx).srcBuffer as *const BYTE;
    strm.avail_in = (*(*ress).readCtx).srcBufferLoaded;
    loop {
        let mut ret = LZMA_OK;
        if strm.avail_in == 0 as std::ffi::c_int as size_t {
            AIO_ReadPool_consumeAndRefill((*ress).readCtx);
            if (*(*ress).readCtx).srcBufferLoaded == 0 as std::ffi::c_int as size_t {
                action = LZMA_FINISH;
            }
            strm.next_in = (*(*ress).readCtx).srcBuffer as *const BYTE;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded;
        }
        ret = lzma_code(&mut strm, action);
        if ret as std::ffi::c_uint == LZMA_BUF_ERROR as std::ffi::c_int as std::ffi::c_uint {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: premature lzma end \n\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                );
            }
            decodingError = 1 as std::ffi::c_int;
            break;
        } else if ret as std::ffi::c_uint != LZMA_OK as std::ffi::c_int as std::ffi::c_uint
            && ret as std::ffi::c_uint != LZMA_STREAM_END as std::ffi::c_int as std::ffi::c_uint
        {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_code decoding error %d \n\0" as *const u8
                        as *const std::ffi::c_char,
                    srcFileName,
                    ret as std::ffi::c_uint,
                );
            }
            decodingError = 1 as std::ffi::c_int;
            break;
        } else {
            let decompBytes = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out);
            if decompBytes != 0 {
                (*writeJob).usedBufferSize = decompBytes;
                AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
                outFileSize = outFileSize.wrapping_add(decompBytes as std::ffi::c_ulonglong);
                strm.next_out = (*writeJob).buffer as *mut BYTE;
                strm.avail_out = (*writeJob).bufferSize;
            }
            if ret as std::ffi::c_uint == LZMA_STREAM_END as std::ffi::c_int as std::ffi::c_uint {
                break;
            }
        }
    }
    AIO_ReadPool_consumeBytes(
        (*ress).readCtx,
        ((*(*ress).readCtx).srcBufferLoaded).wrapping_sub(strm.avail_in),
    );
    lzma_end(&mut strm);
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    if decodingError != 0 {
        FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong
    } else {
        outFileSize
    }
}
unsafe extern "C" fn FIO_decompressFrames(
    fCtx: *mut FIO_ctx_t,
    mut ress: dRess_t,
    prefs: *const FIO_prefs_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut readSomething = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut filesize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    let mut passThrough = (*prefs).passThrough;
    if passThrough == -(1 as std::ffi::c_int) {
        passThrough = ((*prefs).overwrite != 0 && strcmp(dstFileName, stdoutmark.as_ptr()) == 0)
            as std::ffi::c_int;
    }
    if passThrough == 0 as std::ffi::c_int || passThrough == 1 as std::ffi::c_int {
    } else {
        __assert_fail(
            b"passThrough == 0 || passThrough == 1\0" as *const u8
                as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            2746 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<
                &[u8; 106],
                &[std::ffi::c_char; 106],
            >(
                b"int FIO_decompressFrames(FIO_ctx_t *const, dRess_t, const FIO_prefs_t *const, const char *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_34012: {
        if passThrough == 0 as std::ffi::c_int || passThrough == 1 as std::ffi::c_int {
        } else {
            __assert_fail(
                b"passThrough == 0 || passThrough == 1\0" as *const u8
                    as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                2746 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 106],
                    &[std::ffi::c_char; 106],
                >(
                    b"int FIO_decompressFrames(FIO_ctx_t *const, dRess_t, const FIO_prefs_t *const, const char *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    loop {
        let toRead = 4 as std::ffi::c_int as size_t;
        let mut buf = std::ptr::null::<BYTE>();
        AIO_ReadPool_fillBuffer(ress.readCtx, toRead);
        buf = (*ress.readCtx).srcBuffer as *const BYTE;
        if (*ress.readCtx).srcBufferLoaded == 0 as std::ffi::c_int as size_t {
            if readSomething == 0 as std::ffi::c_int as std::ffi::c_uint {
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: unexpected end of file \n\0" as *const u8
                            as *const std::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1 as std::ffi::c_int;
            }
            break;
        } else {
            readSomething = 1 as std::ffi::c_int as std::ffi::c_uint;
            if (*ress.readCtx).srcBufferLoaded < toRead {
                if passThrough != 0 {
                    return FIO_passThrough(&mut ress);
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: unknown header \n\0" as *const u8 as *const std::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1 as std::ffi::c_int;
            }
            if ZSTD_isFrame(
                buf as *const std::ffi::c_void,
                (*ress.readCtx).srcBufferLoaded,
            ) != 0
            {
                let frameSize =
                    FIO_decompressZstdFrame(fCtx, &mut ress, prefs, srcFileName, filesize as U64);
                if frameSize == FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong {
                    return 1 as std::ffi::c_int;
                }
                filesize = filesize.wrapping_add(frameSize);
            } else if *buf.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                == 31 as std::ffi::c_int
                && *buf.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    == 139 as std::ffi::c_int
            {
                let frameSize_0 = FIO_decompressGzFrame(&mut ress, srcFileName);
                if frameSize_0 == FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong {
                    return 1 as std::ffi::c_int;
                }
                filesize = filesize.wrapping_add(frameSize_0);
            } else if *buf.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                == 0xfd as std::ffi::c_int
                && *buf.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    == 0x37 as std::ffi::c_int
                || *buf.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                    == 0x5d as std::ffi::c_int
                    && *buf.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                        == 0 as std::ffi::c_int
            {
                let frameSize_1 = FIO_decompressLzmaFrame(
                    &mut ress,
                    srcFileName,
                    (*buf.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        != 0xfd as std::ffi::c_int) as std::ffi::c_int,
                );
                if frameSize_1 == FIO_ERROR_FRAME_DECODING as std::ffi::c_ulonglong {
                    return 1 as std::ffi::c_int;
                }
                filesize = filesize.wrapping_add(frameSize_1);
            } else if MEM_readLE32(buf as *const std::ffi::c_void) == LZ4_MAGICNUMBER as U32 {
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: lz4 file cannot be uncompressed (zstd compiled without HAVE_LZ4) -- ignored \n\0"
                            as *const u8 as *const std::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1 as std::ffi::c_int;
            } else if passThrough != 0 {
                return FIO_passThrough(&mut ress);
            } else {
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"zstd: %s: unsupported format \n\0" as *const u8
                            as *const std::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1 as std::ffi::c_int;
            }
        }
    }
    (*fCtx).totalBytesOutput = ((*fCtx).totalBytesOutput).wrapping_add(filesize as size_t);
    if g_display_prefs.progressSetting as std::ffi::c_uint
        != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
        && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
            || g_display_prefs.progressSetting as std::ffi::c_uint
                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
    {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if FIO_shouldDisplayFileSummary(fCtx) != 0
        && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
            || g_display_prefs.progressSetting as std::ffi::c_uint
                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
        && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
    {
        fprintf(
            stderr,
            b"%-20s: %llu bytes \n\0" as *const u8 as *const std::ffi::c_char,
            srcFileName,
            filesize,
        );
    }
    0 as std::ffi::c_int
}
unsafe extern "C" fn FIO_decompressDstFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: dRess_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
    mut srcFileStat: *const stat_t,
) -> std::ffi::c_int {
    let mut result: std::ffi::c_int = 0;
    let mut releaseDstFile = 0 as std::ffi::c_int;
    let mut transferStat = 0 as std::ffi::c_int;
    let mut dstFd = 0 as std::ffi::c_int;
    if (AIO_WritePool_getFile(ress.writeCtx)).is_null() && (*prefs).testMode == 0 as std::ffi::c_int
    {
        let mut dstFile = std::ptr::null_mut::<FILE>();
        let mut dstFilePermissions = DEFAULT_FILE_PERMISSIONS;
        if strcmp(srcFileName, stdinmark.as_ptr()) != 0
            && strcmp(dstFileName, stdoutmark.as_ptr()) != 0
            && UTIL_isRegularFileStat(srcFileStat) != 0
        {
            transferStat = 1 as std::ffi::c_int;
            dstFilePermissions = TEMPORARY_FILE_PERMISSIONS;
        }
        releaseDstFile = 1 as std::ffi::c_int;
        dstFile = FIO_openDstFile(fCtx, prefs, srcFileName, dstFileName, dstFilePermissions);
        if dstFile.is_null() {
            return 1 as std::ffi::c_int;
        }
        dstFd = fileno(dstFile);
        AIO_WritePool_setFile(ress.writeCtx, dstFile);
        addHandler(dstFileName);
    }
    result = FIO_decompressFrames(fCtx, ress, prefs, dstFileName, srcFileName);
    if releaseDstFile != 0 {
        clearHandler();
        if transferStat != 0 {
            UTIL_setFDStat(dstFd, dstFileName, srcFileStat);
        }
        if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s \n\0" as *const u8 as *const std::ffi::c_char,
                    dstFileName,
                    strerror(*__errno_location()),
                );
            }
            result = 1 as std::ffi::c_int;
        }
        if transferStat != 0 {
            UTIL_utime(dstFileName, srcFileStat);
        }
        if result != 0 as std::ffi::c_int && strcmp(dstFileName, stdoutmark.as_ptr()) != 0 {
            FIO_removeFile(dstFileName);
        }
    }
    result
}
unsafe extern "C" fn FIO_decompressSrcFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: dRess_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut srcFile = std::ptr::null_mut::<FILE>();
    let mut srcFileStat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut result: std::ffi::c_int = 0;
    let mut fileSize = UTIL_FILESIZE_UNKNOWN as U64;
    if UTIL_isDirectory(srcFileName) != 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s is a directory -- ignored \n\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
            );
        }
        return 1 as std::ffi::c_int;
    }
    srcFile = FIO_openSrcFile(prefs, srcFileName, &mut srcFileStat);
    if srcFile.is_null() {
        return 1 as std::ffi::c_int;
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        fileSize = UTIL_getFileSizeStat(&mut srcFileStat);
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as U64
        && fileSize < (ZSTD_BLOCKSIZE_MAX * 3 as std::ffi::c_int) as U64
    {
        AIO_ReadPool_setAsync(ress.readCtx, 0 as std::ffi::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 0 as std::ffi::c_int);
    } else {
        AIO_ReadPool_setAsync(ress.readCtx, 1 as std::ffi::c_int);
        AIO_WritePool_setAsync(ress.writeCtx, 1 as std::ffi::c_int);
    }
    AIO_ReadPool_setFile(ress.readCtx, srcFile);
    result = FIO_decompressDstFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        &mut srcFileStat,
    );
    AIO_ReadPool_setFile(ress.readCtx, NULL as *mut FILE);
    if fclose(srcFile) != 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: %s \n\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
                strerror(*__errno_location()),
            );
        }
        return 1 as std::ffi::c_int;
    }
    if (*prefs).removeSrcFile != 0
        && result == 0 as std::ffi::c_int
        && strcmp(srcFileName, stdinmark.as_ptr()) != 0
    {
        clearHandler();
        if FIO_removeFile(srcFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: %s: %s \n\0" as *const u8 as *const std::ffi::c_char,
                    srcFileName,
                    strerror(*__errno_location()),
                );
            }
            return 1 as std::ffi::c_int;
        }
    }
    result
}
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressFilename(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut dstFileName: *const std::ffi::c_char,
    mut srcFileName: *const std::ffi::c_char,
    mut dictFileName: *const std::ffi::c_char,
) -> std::ffi::c_int {
    let ress = FIO_createDResources(prefs, dictFileName);
    let decodingError = FIO_decompressSrcFile(fCtx, prefs, ress, dstFileName, srcFileName);
    FIO_freeDResources(ress);
    decodingError
}
static mut suffixList: [*const std::ffi::c_char; 9] = [
    ZSTD_EXTENSION.as_ptr(),
    TZSTD_EXTENSION.as_ptr(),
    ZSTD_ALT_EXTENSION.as_ptr(),
    GZ_EXTENSION.as_ptr(),
    TGZ_EXTENSION.as_ptr(),
    LZMA_EXTENSION.as_ptr(),
    XZ_EXTENSION.as_ptr(),
    TXZ_EXTENSION.as_ptr(),
    NULL as *const std::ffi::c_char,
];
static mut suffixListStr: *const std::ffi::c_char =
    b".zst/.tzst/.gz/.tgz/.lzma/.xz/.txz\0" as *const u8 as *const std::ffi::c_char;
unsafe extern "C" fn FIO_determineDstName(
    mut srcFileName: *const std::ffi::c_char,
    mut outDirName: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    static mut dfnbCapacity: size_t = 0 as std::ffi::c_int as size_t;
    static mut dstFileNameBuffer: *mut std::ffi::c_char = NULL as *mut std::ffi::c_char;
    let mut dstFileNameEndPos: size_t = 0;
    let mut outDirFilename = NULL as *mut std::ffi::c_char;
    let mut dstSuffix = b"\0" as *const u8 as *const std::ffi::c_char;
    let mut dstSuffixLen = 0 as std::ffi::c_int as size_t;
    let mut sfnSize = strlen(srcFileName);
    let mut srcSuffixLen: size_t = 0;
    let srcSuffix: *const std::ffi::c_char = strrchr(srcFileName, '.' as i32);
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        return stdoutmark.as_ptr();
    }
    if srcSuffix.is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: unknown suffix (%s expected). Can't derive the output file name. Specify it with -o dstFileName. Ignoring.\n\0"
                    as *const u8 as *const std::ffi::c_char,
                srcFileName,
                suffixListStr,
            );
        }
        return NULL as *const std::ffi::c_char;
    }
    srcSuffixLen = strlen(srcSuffix);
    let mut matchedSuffixPtr = std::ptr::null_mut::<*const std::ffi::c_char>();
    matchedSuffixPtr = suffixList.as_mut_ptr();
    while !(*matchedSuffixPtr).is_null() {
        if strcmp(*matchedSuffixPtr, srcSuffix) == 0 {
            break;
        }
        matchedSuffixPtr = matchedSuffixPtr.offset(1);
        matchedSuffixPtr;
    }
    if sfnSize <= srcSuffixLen || (*matchedSuffixPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: %s: unknown suffix (%s expected). Can't derive the output file name. Specify it with -o dstFileName. Ignoring.\n\0"
                    as *const u8 as *const std::ffi::c_char,
                srcFileName,
                suffixListStr,
            );
        }
        return NULL as *const std::ffi::c_char;
    }
    if *(*matchedSuffixPtr).offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == 't' as i32 {
        dstSuffix = b".tar\0" as *const u8 as *const std::ffi::c_char;
        dstSuffixLen = strlen(dstSuffix);
    }
    if !outDirName.is_null() {
        outDirFilename =
            FIO_createFilename_fromOutDir(srcFileName, outDirName, 0 as std::ffi::c_int as size_t);
        sfnSize = strlen(outDirFilename);
        if !outDirFilename.is_null() {
        } else {
            __assert_fail(
                b"outDirFilename != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                3057 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 61], &[std::ffi::c_char; 61]>(
                    b"const char *FIO_determineDstName(const char *, const char *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_39574: {
            if !outDirFilename.is_null() {
            } else {
                __assert_fail(
                    b"outDirFilename != NULL\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    3057 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<&[u8; 61], &[std::ffi::c_char; 61]>(
                        b"const char *FIO_determineDstName(const char *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
    }
    if dfnbCapacity.wrapping_add(srcSuffixLen)
        <= sfnSize
            .wrapping_add(1 as std::ffi::c_int as size_t)
            .wrapping_add(dstSuffixLen)
    {
        free(dstFileNameBuffer as *mut std::ffi::c_void);
        dfnbCapacity = sfnSize.wrapping_add(20 as std::ffi::c_int as size_t);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut std::ffi::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    3067 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    74 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"%s : not enough memory for dstFileName\0" as *const u8
                        as *const std::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(74 as std::ffi::c_int);
        }
    }
    if !dstFileNameBuffer.is_null() {
    } else {
        __assert_fail(
            b"dstFileNameBuffer != NULL\0" as *const u8 as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            3071 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 61], &[std::ffi::c_char; 61]>(
                b"const char *FIO_determineDstName(const char *, const char *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_39364: {
        if !dstFileNameBuffer.is_null() {
        } else {
            __assert_fail(
                b"dstFileNameBuffer != NULL\0" as *const u8 as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                3071 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 61], &[std::ffi::c_char; 61]>(
                    b"const char *FIO_determineDstName(const char *, const char *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    dstFileNameEndPos = sfnSize.wrapping_sub(srcSuffixLen);
    if !outDirFilename.is_null() {
        memcpy(
            dstFileNameBuffer as *mut std::ffi::c_void,
            outDirFilename as *const std::ffi::c_void,
            dstFileNameEndPos,
        );
        free(outDirFilename as *mut std::ffi::c_void);
    } else {
        memcpy(
            dstFileNameBuffer as *mut std::ffi::c_void,
            srcFileName as *const std::ffi::c_void,
            dstFileNameEndPos,
        );
    }
    strcpy(
        dstFileNameBuffer.offset(dstFileNameEndPos as isize),
        dstSuffix,
    );
    dstFileNameBuffer
}
#[no_mangle]
pub unsafe extern "C" fn FIO_decompressMultipleFilenames(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut srcNamesTable: *mut *const std::ffi::c_char,
    mut outMirroredRootDirName: *const std::ffi::c_char,
    mut outDirName: *const std::ffi::c_char,
    mut outFileName: *const std::ffi::c_char,
    mut dictFileName: *const std::ffi::c_char,
) -> std::ffi::c_int {
    let mut status: std::ffi::c_int = 0;
    let mut error = 0 as std::ffi::c_int;
    let mut ress = FIO_createDResources(prefs, dictFileName);
    if !outFileName.is_null() {
        if FIO_multiFilesConcatWarning(fCtx, prefs, outFileName, 1 as std::ffi::c_int) != 0 {
            FIO_freeDResources(ress);
            return 1 as std::ffi::c_int;
        }
        if (*prefs).testMode == 0 {
            let mut dstFile = FIO_openDstFile(
                fCtx,
                prefs,
                NULL as *const std::ffi::c_char,
                outFileName,
                DEFAULT_FILE_PERMISSIONS,
            );
            if dstFile.is_null() {
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const std::ffi::c_char,
                        b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                        3107 as std::ffi::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                        19 as std::ffi::c_int,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"cannot open %s\0" as *const u8 as *const std::ffi::c_char,
                        outFileName,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                }
                exit(19 as std::ffi::c_int);
            }
            AIO_WritePool_setFile(ress.writeCtx, dstFile);
        }
        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            status = FIO_decompressSrcFile(
                fCtx,
                prefs,
                ress,
                outFileName,
                *srcNamesTable.offset((*fCtx).currFileIdx as isize),
            );
            if status == 0 {
                (*fCtx).nbFilesProcessed += 1;
                (*fCtx).nbFilesProcessed;
            }
            error |= status;
            (*fCtx).currFileIdx += 1;
            (*fCtx).currFileIdx;
        }
        if (*prefs).testMode == 0 && AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const std::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const std::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                    3117 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const std::ffi::c_char,
                    72 as std::ffi::c_int,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Write error : %s : cannot properly close output file\0" as *const u8
                        as *const std::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            exit(72 as std::ffi::c_int);
        }
    } else {
        if !outMirroredRootDirName.is_null() {
            UTIL_mirrorSourceFilesDirectories(
                srcNamesTable,
                (*fCtx).nbFilesTotal as std::ffi::c_uint,
                outMirroredRootDirName,
            );
        }
        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            let srcFileName = *srcNamesTable.offset((*fCtx).currFileIdx as isize);
            let mut dstFileName = NULL as *const std::ffi::c_char;
            if !outMirroredRootDirName.is_null() {
                let mut validMirroredDirName =
                    UTIL_createMirroredDestDirName(srcFileName, outMirroredRootDirName);
                if !validMirroredDirName.is_null() {
                    dstFileName = FIO_determineDstName(srcFileName, validMirroredDirName);
                    free(validMirroredDirName as *mut std::ffi::c_void);
                } else if g_display_prefs.displayLevel >= 2 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"zstd: --output-dir-mirror cannot decompress '%s' into '%s'\n\0"
                            as *const u8 as *const std::ffi::c_char,
                        srcFileName,
                        outMirroredRootDirName,
                    );
                }
            } else {
                dstFileName = FIO_determineDstName(srcFileName, outDirName);
            }
            if dstFileName.is_null() {
                error = 1 as std::ffi::c_int;
            } else {
                status = FIO_decompressSrcFile(fCtx, prefs, ress, dstFileName, srcFileName);
                if status == 0 {
                    (*fCtx).nbFilesProcessed += 1;
                    (*fCtx).nbFilesProcessed;
                }
                error |= status;
            }
            (*fCtx).currFileIdx += 1;
            (*fCtx).currFileIdx;
        }
        if !outDirName.is_null() {
            FIO_checkFilenameCollisions(srcNamesTable, (*fCtx).nbFilesTotal as std::ffi::c_uint);
        }
    }
    if FIO_shouldDisplayMultipleFileSummary(fCtx) != 0 {
        if g_display_prefs.progressSetting as std::ffi::c_uint
            != FIO_ps_never as std::ffi::c_int as std::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
                || g_display_prefs.progressSetting as std::ffi::c_uint
                    == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
        {
            fprintf(
                stderr,
                b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
                b"\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        if (g_display_prefs.displayLevel >= 2 as std::ffi::c_int
            || g_display_prefs.progressSetting as std::ffi::c_uint
                == FIO_ps_always as std::ffi::c_int as std::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1 as std::ffi::c_int
        {
            fprintf(
                stderr,
                b"%d files decompressed : %6llu bytes total \n\0" as *const u8
                    as *const std::ffi::c_char,
                (*fCtx).nbFilesProcessed,
                (*fCtx).totalBytesOutput as std::ffi::c_ulonglong,
            );
        }
    }
    FIO_freeDResources(ress);
    error
}
unsafe extern "C" fn FIO_analyzeFrames(mut info: *mut fileInfo_t, srcFile: *mut FILE) -> InfoError {
    loop {
        let mut headerBuffer: [BYTE; 18] = [0; 18];
        let numBytesRead = fread(
            headerBuffer.as_mut_ptr() as *mut std::ffi::c_void,
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ::core::mem::size_of::<[BYTE; 18]>() as std::ffi::c_ulong,
            srcFile,
        );
        if numBytesRead
            < (if ZSTD_f_zstd1 as std::ffi::c_int == ZSTD_f_zstd1 as std::ffi::c_int {
                6 as std::ffi::c_int
            } else {
                2 as std::ffi::c_int
            }) as size_t
        {
            if feof(srcFile) != 0
                && numBytesRead == 0 as std::ffi::c_int as size_t
                && (*info).compressedSize > 0 as std::ffi::c_int as U64
                && (*info).compressedSize != UTIL_FILESIZE_UNKNOWN as U64
            {
                let mut file_position = ftell(srcFile) as std::ffi::c_ulonglong;
                let mut file_size = (*info).compressedSize as std::ffi::c_ulonglong;
                if file_position != file_size {
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"Error: seeked to position %llu, which is beyond file size of %llu\n\0"
                                as *const u8 as *const std::ffi::c_char,
                            file_position,
                            file_size,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                    }
                    return info_truncated_input;
                }
                break;
            } else {
                if feof(srcFile) != 0 {
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"Error: reached end of file with incomplete frame\0" as *const u8
                                as *const std::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                    }
                    return info_not_zstd;
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Error: did not reach end of file but ran out of frames\0" as *const u8
                            as *const std::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                    fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                }
                return info_frame_error;
            }
        } else {
            let magicNumber = MEM_readLE32(headerBuffer.as_mut_ptr() as *const std::ffi::c_void);
            if magicNumber == ZSTD_MAGICNUMBER {
                let mut header = ZSTD_FrameHeader {
                    frameContentSize: 0,
                    windowSize: 0,
                    blockSizeMax: 0,
                    frameType: ZSTD_frame,
                    headerSize: 0,
                    dictID: 0,
                    checksumFlag: 0,
                    _reserved1: 0,
                    _reserved2: 0,
                };
                let frameContentSize = ZSTD_getFrameContentSize(
                    headerBuffer.as_mut_ptr() as *const std::ffi::c_void,
                    numBytesRead,
                ) as U64;
                if frameContentSize as std::ffi::c_ulonglong == ZSTD_CONTENTSIZE_ERROR
                    || frameContentSize as std::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
                {
                    (*info).decompUnavailable = 1 as std::ffi::c_int;
                } else {
                    (*info).decompressedSize =
                        ((*info).decompressedSize).wrapping_add(frameContentSize);
                }
                if ZSTD_getFrameHeader(
                    &mut header,
                    headerBuffer.as_mut_ptr() as *const std::ffi::c_void,
                    numBytesRead,
                ) != 0 as std::ffi::c_int as size_t
                {
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not decode frame header\0" as *const u8
                                as *const std::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                    }
                    return info_frame_error;
                }
                if (*info).dictID != 0 as std::ffi::c_int as std::ffi::c_uint
                    && (*info).dictID != header.dictID
                {
                    fprintf(
                        stderr,
                        b"WARNING: File contains multiple frames with different dictionary IDs. Showing dictID 0 instead\0"
                            as *const u8 as *const std::ffi::c_char,
                    );
                    (*info).dictID = 0 as std::ffi::c_int as std::ffi::c_uint;
                } else {
                    (*info).dictID = header.dictID;
                }
                (*info).windowSize = header.windowSize as U64;
                let headerSize = ZSTD_frameHeaderSize(
                    headerBuffer.as_mut_ptr() as *const std::ffi::c_void,
                    numBytesRead,
                );
                if ZSTD_isError(headerSize) != 0 {
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not determine frame header size\0" as *const u8
                                as *const std::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                    }
                    return info_frame_error;
                }
                if fseek(
                    srcFile,
                    headerSize as std::ffi::c_long - numBytesRead as std::ffi::c_long,
                    1 as std::ffi::c_int,
                ) != 0 as std::ffi::c_int
                {
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not move to end of frame header\0" as *const u8
                                as *const std::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                    }
                    return info_frame_error;
                }
                let mut lastBlock = 0 as std::ffi::c_int;
                loop {
                    let mut blockHeaderBuffer: [BYTE; 3] = [0; 3];
                    if fread(
                        blockHeaderBuffer.as_mut_ptr() as *mut std::ffi::c_void,
                        1 as std::ffi::c_int as std::ffi::c_ulong,
                        3 as std::ffi::c_int as std::ffi::c_ulong,
                        srcFile,
                    ) != 3 as std::ffi::c_int as std::ffi::c_ulong
                    {
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"Error while reading block header\0" as *const u8
                                    as *const std::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                    let blockHeader =
                        MEM_readLE24(blockHeaderBuffer.as_mut_ptr() as *const std::ffi::c_void);
                    let blockTypeID =
                        blockHeader >> 1 as std::ffi::c_int & 3 as std::ffi::c_int as U32;
                    let isRLE =
                        (blockTypeID == 1 as std::ffi::c_int as U32) as std::ffi::c_int as U32;
                    let isWrongBlock =
                        (blockTypeID == 3 as std::ffi::c_int as U32) as std::ffi::c_int as U32;
                    let blockSize = if isRLE != 0 {
                        1 as std::ffi::c_int as std::ffi::c_long
                    } else {
                        (blockHeader >> 3 as std::ffi::c_int) as std::ffi::c_long
                    };
                    if isWrongBlock != 0 {
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"Error: unsupported block type\0" as *const u8
                                    as *const std::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                    lastBlock = (blockHeader & 1 as std::ffi::c_int as U32) as std::ffi::c_int;
                    if fseek(srcFile, blockSize, 1 as std::ffi::c_int) != 0 as std::ffi::c_int {
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"Error: could not skip to end of block\0" as *const u8
                                    as *const std::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                    if lastBlock == 1 as std::ffi::c_int {
                        break;
                    }
                }
                let frameHeaderDescriptor = *headerBuffer
                    .as_mut_ptr()
                    .offset(4 as std::ffi::c_int as isize);
                let contentChecksumFlag = (frameHeaderDescriptor as std::ffi::c_int
                    & (1 as std::ffi::c_int) << 2 as std::ffi::c_int)
                    >> 2 as std::ffi::c_int;
                if contentChecksumFlag != 0 {
                    (*info).usesCheck = 1 as std::ffi::c_int;
                    if fread(
                        ((*info).checksum).as_mut_ptr() as *mut std::ffi::c_void,
                        1 as std::ffi::c_int as std::ffi::c_ulong,
                        4 as std::ffi::c_int as std::ffi::c_ulong,
                        srcFile,
                    ) != 4 as std::ffi::c_int as std::ffi::c_ulong
                    {
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(
                                stderr,
                                b"Error: could not read checksum\0" as *const u8
                                    as *const std::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                }
                (*info).numActualFrames += 1;
                (*info).numActualFrames;
            } else if magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK
                == ZSTD_MAGIC_SKIPPABLE_START as std::ffi::c_uint
            {
                let frameSize = MEM_readLE32(
                    headerBuffer
                        .as_mut_ptr()
                        .offset(4 as std::ffi::c_int as isize)
                        as *const std::ffi::c_void,
                );
                let seek = ((8 as std::ffi::c_int as U32).wrapping_add(frameSize) as size_t)
                    .wrapping_sub(numBytesRead) as std::ffi::c_long;
                if fseek(srcFile, seek, 1 as std::ffi::c_int) != 0 as std::ffi::c_int {
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(
                            stderr,
                            b"Error: could not find end of skippable frame\0" as *const u8
                                as *const std::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                        fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
                    }
                    return info_frame_error;
                }
                (*info).numSkippableFrames += 1;
                (*info).numSkippableFrames;
            } else {
                return info_not_zstd;
            }
        }
    }
    info_success
}
unsafe extern "C" fn getFileInfo_fileConfirmed(
    mut info: *mut fileInfo_t,
    mut inFileName: *const std::ffi::c_char,
) -> InfoError {
    let mut status = info_success;
    let mut srcFileStat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let srcFile = FIO_openSrcFile(NULL as *const FIO_prefs_t, inFileName, &mut srcFileStat);
    if srcFile.is_null() {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error: could not open source file %s\0" as *const u8 as *const std::ffi::c_char,
                inFileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        return info_file_error;
    }
    (*info).compressedSize = UTIL_getFileSizeStat(&mut srcFileStat);
    status = FIO_analyzeFrames(info, srcFile);
    fclose(srcFile);
    (*info).nbFiles = 1 as std::ffi::c_int as U32;
    status
}
unsafe extern "C" fn getFileInfo(
    mut info: *mut fileInfo_t,
    mut srcFileName: *const std::ffi::c_char,
) -> InfoError {
    if UTIL_isRegularFile(srcFileName) == 0 {
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Error : %s is not a file\0" as *const u8 as *const std::ffi::c_char,
                srcFileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
        }
        return info_file_error;
    }
    getFileInfo_fileConfirmed(info, srcFileName)
}
unsafe extern "C" fn displayInfo(
    mut inFileName: *const std::ffi::c_char,
    mut info: *const fileInfo_t,
    mut displayLevel: std::ffi::c_int,
) {
    let window_hrs = UTIL_makeHumanReadableSize((*info).windowSize);
    let compressed_hrs = UTIL_makeHumanReadableSize((*info).compressedSize);
    let decompressed_hrs = UTIL_makeHumanReadableSize((*info).decompressedSize);
    let ratio = if (*info).compressedSize == 0 as std::ffi::c_int as U64 {
        0 as std::ffi::c_int as std::ffi::c_double
    } else {
        (*info).decompressedSize as std::ffi::c_double
            / (*info).compressedSize as std::ffi::c_double
    };
    let checkString = if (*info).usesCheck != 0 {
        b"XXH64\0" as *const u8 as *const std::ffi::c_char
    } else {
        b"None\0" as *const u8 as *const std::ffi::c_char
    };
    if displayLevel <= 2 as std::ffi::c_int {
        if (*info).decompUnavailable == 0 {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s  %8.*f%4s  %5.3f  %5s  %s\n\0" as *const u8
                    as *const std::ffi::c_char,
                (*info).numSkippableFrames + (*info).numActualFrames,
                (*info).numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                decompressed_hrs.precision,
                decompressed_hrs.value,
                decompressed_hrs.suffix,
                ratio,
                checkString,
                inFileName,
            );
        } else {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s                       %5s  %s\n\0" as *const u8
                    as *const std::ffi::c_char,
                (*info).numSkippableFrames + (*info).numActualFrames,
                (*info).numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                checkString,
                inFileName,
            );
        }
    } else {
        fprintf(
            stdout,
            b"%s \n\0" as *const u8 as *const std::ffi::c_char,
            inFileName,
        );
        fprintf(
            stdout,
            b"# Zstandard Frames: %d\n\0" as *const u8 as *const std::ffi::c_char,
            (*info).numActualFrames,
        );
        if (*info).numSkippableFrames != 0 {
            fprintf(
                stdout,
                b"# Skippable Frames: %d\n\0" as *const u8 as *const std::ffi::c_char,
                (*info).numSkippableFrames,
            );
        }
        fprintf(
            stdout,
            b"DictID: %u\n\0" as *const u8 as *const std::ffi::c_char,
            (*info).dictID,
        );
        fprintf(
            stdout,
            b"Window Size: %.*f%s (%llu B)\n\0" as *const u8 as *const std::ffi::c_char,
            window_hrs.precision,
            window_hrs.value,
            window_hrs.suffix,
            (*info).windowSize as std::ffi::c_ulonglong,
        );
        fprintf(
            stdout,
            b"Compressed Size: %.*f%s (%llu B)\n\0" as *const u8 as *const std::ffi::c_char,
            compressed_hrs.precision,
            compressed_hrs.value,
            compressed_hrs.suffix,
            (*info).compressedSize as std::ffi::c_ulonglong,
        );
        if (*info).decompUnavailable == 0 {
            fprintf(
                stdout,
                b"Decompressed Size: %.*f%s (%llu B)\n\0" as *const u8 as *const std::ffi::c_char,
                decompressed_hrs.precision,
                decompressed_hrs.value,
                decompressed_hrs.suffix,
                (*info).decompressedSize as std::ffi::c_ulonglong,
            );
            fprintf(
                stdout,
                b"Ratio: %.4f\n\0" as *const u8 as *const std::ffi::c_char,
                ratio,
            );
        }
        if (*info).usesCheck != 0 && (*info).numActualFrames == 1 as std::ffi::c_int {
            fprintf(
                stdout,
                b"Check: %s %02x%02x%02x%02x\n\0" as *const u8 as *const std::ffi::c_char,
                checkString,
                *((*info).checksum)
                    .as_ptr()
                    .offset(3 as std::ffi::c_int as isize) as std::ffi::c_int,
                *((*info).checksum)
                    .as_ptr()
                    .offset(2 as std::ffi::c_int as isize) as std::ffi::c_int,
                *((*info).checksum)
                    .as_ptr()
                    .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int,
                *((*info).checksum)
                    .as_ptr()
                    .offset(0 as std::ffi::c_int as isize) as std::ffi::c_int,
            );
        } else {
            fprintf(
                stdout,
                b"Check: %s\n\0" as *const u8 as *const std::ffi::c_char,
                checkString,
            );
        }
        fprintf(stdout, b"\n\0" as *const u8 as *const std::ffi::c_char);
    };
}
unsafe extern "C" fn FIO_addFInfo(mut fi1: fileInfo_t, mut fi2: fileInfo_t) -> fileInfo_t {
    let mut total = fileInfo_t {
        decompressedSize: 0,
        compressedSize: 0,
        windowSize: 0,
        numActualFrames: 0,
        numSkippableFrames: 0,
        decompUnavailable: 0,
        usesCheck: 0,
        checksum: [0; 4],
        nbFiles: 0,
        dictID: 0,
    };
    memset(
        &mut total as *mut fileInfo_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<fileInfo_t>() as std::ffi::c_ulong,
    );
    total.numActualFrames = fi1.numActualFrames + fi2.numActualFrames;
    total.numSkippableFrames = fi1.numSkippableFrames + fi2.numSkippableFrames;
    total.compressedSize = (fi1.compressedSize).wrapping_add(fi2.compressedSize);
    total.decompressedSize = (fi1.decompressedSize).wrapping_add(fi2.decompressedSize);
    total.decompUnavailable = fi1.decompUnavailable | fi2.decompUnavailable;
    total.usesCheck = fi1.usesCheck & fi2.usesCheck;
    total.nbFiles = (fi1.nbFiles).wrapping_add(fi2.nbFiles);
    total
}
unsafe extern "C" fn FIO_listFile(
    mut total: *mut fileInfo_t,
    mut inFileName: *const std::ffi::c_char,
    mut displayLevel: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut info = fileInfo_t {
        decompressedSize: 0,
        compressedSize: 0,
        windowSize: 0,
        numActualFrames: 0,
        numSkippableFrames: 0,
        decompUnavailable: 0,
        usesCheck: 0,
        checksum: [0; 4],
        nbFiles: 0,
        dictID: 0,
    };
    memset(
        &mut info as *mut fileInfo_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<fileInfo_t>() as std::ffi::c_ulong,
    );
    let error = getFileInfo(&mut info, inFileName);
    match error as std::ffi::c_uint {
        1 => {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Error while parsing \"%s\" \n\0" as *const u8 as *const std::ffi::c_char,
                    inFileName,
                );
            }
        }
        2 => {
            fprintf(
                stdout,
                b"File \"%s\" not compressed by zstd \n\0" as *const u8 as *const std::ffi::c_char,
                inFileName,
            );
            if displayLevel > 2 as std::ffi::c_int {
                fprintf(stdout, b"\n\0" as *const u8 as *const std::ffi::c_char);
            }
            return 1 as std::ffi::c_int;
        }
        3 => {
            if displayLevel > 2 as std::ffi::c_int {
                fprintf(stdout, b"\n\0" as *const u8 as *const std::ffi::c_char);
            }
            return 1 as std::ffi::c_int;
        }
        4 => {
            fprintf(
                stdout,
                b"File \"%s\" is truncated \n\0" as *const u8 as *const std::ffi::c_char,
                inFileName,
            );
            if displayLevel > 2 as std::ffi::c_int {
                fprintf(stdout, b"\n\0" as *const u8 as *const std::ffi::c_char);
            }
            return 1 as std::ffi::c_int;
        }
        0 | _ => {}
    }
    displayInfo(inFileName, &mut info, displayLevel);
    *total = FIO_addFInfo(*total, info);
    if error as std::ffi::c_uint == info_success as std::ffi::c_int as std::ffi::c_uint
        || error as std::ffi::c_uint == info_frame_error as std::ffi::c_int as std::ffi::c_uint
    {
    } else {
        __assert_fail(
            b"error == info_success || error == info_frame_error\0" as *const u8
                as *const std::ffi::c_char,
            b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
            3414 as std::ffi::c_int as std::ffi::c_uint,
            (*::core::mem::transmute::<&[u8; 50], &[std::ffi::c_char; 50]>(
                b"int FIO_listFile(fileInfo_t *, const char *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_35927: {
        if error as std::ffi::c_uint == info_success as std::ffi::c_int as std::ffi::c_uint
            || error as std::ffi::c_uint == info_frame_error as std::ffi::c_int as std::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"error == info_success || error == info_frame_error\0" as *const u8
                    as *const std::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const std::ffi::c_char,
                3414 as std::ffi::c_int as std::ffi::c_uint,
                (*::core::mem::transmute::<&[u8; 50], &[std::ffi::c_char; 50]>(
                    b"int FIO_listFile(fileInfo_t *, const char *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    error as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn FIO_listMultipleFiles(
    mut numFiles: std::ffi::c_uint,
    mut filenameTable: *mut *const std::ffi::c_char,
    mut displayLevel: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut u: std::ffi::c_uint = 0;
    u = 0 as std::ffi::c_int as std::ffi::c_uint;
    while u < numFiles {
        if strcmp(
            *filenameTable.offset(u as isize),
            b"/*stdin*\\\0" as *const u8 as *const std::ffi::c_char,
        ) == 0
        {
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"zstd: --list does not support reading from standard input\0" as *const u8
                        as *const std::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
                fprintf(stderr, b" \n\0" as *const u8 as *const std::ffi::c_char);
            }
            return 1 as std::ffi::c_int;
        }
        u = u.wrapping_add(1);
        u;
    }
    if numFiles == 0 as std::ffi::c_int as std::ffi::c_uint {
        if UTIL_isConsole(stdin) == 0 && g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"zstd: --list does not support reading from standard input \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"No files given \n\0" as *const u8 as *const std::ffi::c_char,
            );
        }
        return 1 as std::ffi::c_int;
    }
    if displayLevel <= 2 as std::ffi::c_int {
        fprintf(
            stdout,
            b"Frames  Skips  Compressed  Uncompressed  Ratio  Check  Filename\n\0" as *const u8
                as *const std::ffi::c_char,
        );
    }
    let mut error = 0 as std::ffi::c_int;
    let mut total = fileInfo_t {
        decompressedSize: 0,
        compressedSize: 0,
        windowSize: 0,
        numActualFrames: 0,
        numSkippableFrames: 0,
        decompUnavailable: 0,
        usesCheck: 0,
        checksum: [0; 4],
        nbFiles: 0,
        dictID: 0,
    };
    memset(
        &mut total as *mut fileInfo_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<fileInfo_t>() as std::ffi::c_ulong,
    );
    total.usesCheck = 1 as std::ffi::c_int;
    let mut u_0: std::ffi::c_uint = 0;
    u_0 = 0 as std::ffi::c_int as std::ffi::c_uint;
    while u_0 < numFiles {
        error |= FIO_listFile(
            &mut total,
            *filenameTable.offset(u_0 as isize),
            displayLevel,
        );
        u_0 = u_0.wrapping_add(1);
        u_0;
    }
    if numFiles > 1 as std::ffi::c_int as std::ffi::c_uint && displayLevel <= 2 as std::ffi::c_int {
        let compressed_hrs = UTIL_makeHumanReadableSize(total.compressedSize);
        let decompressed_hrs = UTIL_makeHumanReadableSize(total.decompressedSize);
        let ratio = if total.compressedSize == 0 as std::ffi::c_int as U64 {
            0 as std::ffi::c_int as std::ffi::c_double
        } else {
            total.decompressedSize as std::ffi::c_double
                / total.compressedSize as std::ffi::c_double
        };
        let checkString = if total.usesCheck != 0 {
            b"XXH64\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"\0" as *const u8 as *const std::ffi::c_char
        };
        fprintf(
            stdout,
            b"----------------------------------------------------------------- \n\0" as *const u8
                as *const std::ffi::c_char,
        );
        if total.decompUnavailable != 0 {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s                       %5s  %u files\n\0" as *const u8
                    as *const std::ffi::c_char,
                total.numSkippableFrames + total.numActualFrames,
                total.numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                checkString,
                total.nbFiles,
            );
        } else {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s  %8.*f%4s  %5.3f  %5s  %u files\n\0" as *const u8
                    as *const std::ffi::c_char,
                total.numSkippableFrames + total.numActualFrames,
                total.numSkippableFrames,
                compressed_hrs.precision,
                compressed_hrs.value,
                compressed_hrs.suffix,
                decompressed_hrs.precision,
                decompressed_hrs.value,
                decompressed_hrs.suffix,
                ratio,
                checkString,
                total.nbFiles,
            );
        }
    }
    error
}
