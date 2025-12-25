use core::ptr;
use std::ffi::CStr;
use std::io;
use std::time::Instant;

use libc::{
    calloc, close, exit, fclose, fdopen, feof, fflush, fileno, fopen, fprintf, fread, free, fseek,
    ftell, malloc, memcpy, mmap, mode_t, munmap, open, remove, sighandler_t, signal, size_t,
    strcmp, strcpy, strlen, strrchr, timespec, FILE, O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY, SIGINT,
    SIG_DFL, SIG_IGN, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR,
};
use libzstd_rs_sys::internal::{MEM_readLE24, MEM_readLE32};
use libzstd_rs_sys::lib::common::zstd_common::{
    ZSTD_getErrorCode, ZSTD_getErrorName, ZSTD_isError,
};
use libzstd_rs_sys::lib::compress::zstd_compress::{
    ZSTD_CCtx_getParameter, ZSTD_CCtx_loadDictionary_byReference, ZSTD_CCtx_refPrefix,
    ZSTD_CCtx_setParameter, ZSTD_CCtx_setPledgedSrcSize, ZSTD_CStream, ZSTD_CStreamInSize,
    ZSTD_CStreamOutSize, ZSTD_compressStream2, ZSTD_createCCtx, ZSTD_freeCStream, ZSTD_getCParams,
    ZSTD_getFrameProgression, ZSTD_maxCLevel, ZSTD_minCLevel, ZSTD_toFlushNow,
};
use libzstd_rs_sys::lib::decompress::zstd_decompress::{
    ZSTD_DCtx_loadDictionary_byReference, ZSTD_DCtx_refPrefix, ZSTD_DCtx_reset,
    ZSTD_DCtx_setMaxWindowSize, ZSTD_DCtx_setParameter, ZSTD_DStreamInSize, ZSTD_DStreamOutSize,
    ZSTD_createDStream, ZSTD_decompressStream, ZSTD_frameHeaderSize, ZSTD_freeDStream,
    ZSTD_getFrameContentSize, ZSTD_getFrameHeader, ZSTD_isFrame,
};
use libzstd_rs_sys::lib::decompress::{ZSTD_DCtx, ZSTD_FrameHeader, ZSTD_frame};
use libzstd_rs_sys::lib::zstd::*;

use crate::fileio_asyncio::{
    AIO_ReadPool_closeFile, AIO_ReadPool_consumeAndRefill, AIO_ReadPool_consumeBytes,
    AIO_ReadPool_create, AIO_ReadPool_fillBuffer, AIO_ReadPool_free, AIO_ReadPool_getFile,
    AIO_ReadPool_setAsync, AIO_ReadPool_setFile, AIO_WritePool_acquireJob, AIO_WritePool_closeFile,
    AIO_WritePool_create, AIO_WritePool_enqueueAndReacquireWriteJob, AIO_WritePool_free,
    AIO_WritePool_getFile, AIO_WritePool_releaseIoJob, AIO_WritePool_setAsync,
    AIO_WritePool_setFile, AIO_WritePool_sparseWriteEnd, AIO_supported, FIO_prefs_t, ReadPoolCtx_t,
    WritePoolCtx_t,
};
use crate::timefn::{PTime, UTIL_clockSpanMicro, UTIL_clockSpanNano, UTIL_getTime, UTIL_time_t};
use crate::util::{
    stat, stat_t, FileNamesTable, UTIL_HumanReadableSize_t, UTIL_compareStr,
    UTIL_createMirroredDestDirName, UTIL_getFileSize, UTIL_getFileSizeStat, UTIL_isBlockDevStat,
    UTIL_isCompressedFile, UTIL_isConsole, UTIL_isDirectory, UTIL_isDirectoryStat, UTIL_isFIFOStat,
    UTIL_isFdRegularFile, UTIL_isFileDescriptorPipe, UTIL_isRegularFile, UTIL_isRegularFileStat,
    UTIL_isSameFile, UTIL_isSameFileStat, UTIL_makeHumanReadableSize,
    UTIL_mirrorSourceFilesDirectories, UTIL_requireUserConfirmation, UTIL_setFDStat, UTIL_stat,
    UTIL_utime,
};

enum lzma_internal_s {}
enum internal_state {}

extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut core::ffi::c_char,
        __modes: core::ffi::c_int,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn zlibVersion() -> *const core::ffi::c_char;
    fn deflate(strm: z_streamp, flush: core::ffi::c_int) -> core::ffi::c_int;
    fn deflateEnd(strm: z_streamp) -> core::ffi::c_int;
    fn inflate(strm: z_streamp, flush: core::ffi::c_int) -> core::ffi::c_int;
    fn inflateEnd(strm: z_streamp) -> core::ffi::c_int;
    fn deflateInit2_(
        strm: z_streamp,
        level: core::ffi::c_int,
        method: core::ffi::c_int,
        windowBits: core::ffi::c_int,
        memLevel: core::ffi::c_int,
        strategy: core::ffi::c_int,
        version: *const core::ffi::c_char,
        stream_size: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: core::ffi::c_int,
        version: *const core::ffi::c_char,
        stream_size: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn lzma_version_string() -> *const core::ffi::c_char;
    fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_lzma_preset(options: *mut lzma_options_lzma, preset: u32) -> lzma_bool;
    fn lzma_easy_encoder(strm: *mut lzma_stream, preset: u32, check: lzma_check) -> lzma_ret;
    fn lzma_alone_encoder(strm: *mut lzma_stream, options: *const lzma_options_lzma) -> lzma_ret;
    fn lzma_stream_decoder(strm: *mut lzma_stream, memlimit: u64, flags: u32) -> lzma_ret;
    fn lzma_alone_decoder(strm: *mut lzma_stream, memlimit: u64) -> lzma_ret;
}
type __compar_fn_t = Option<
    unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> core::ffi::c_int,
>;
pub type ZSTD_EndDirective = core::ffi::c_uint;
pub const ZSTD_e_end: ZSTD_EndDirective = 2;
pub const ZSTD_e_flush: ZSTD_EndDirective = 1;
pub const ZSTD_e_continue: ZSTD_EndDirective = 0;
pub type ZSTD_DStream = ZSTD_DCtx;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const ZSTD_f_zstd1_magicless: C2RustUnnamed_0 = 1;
pub const ZSTD_f_zstd1: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_display_prefs_s {
    pub displayLevel: core::ffi::c_int,
    pub progressSetting: FIO_progressSetting_e,
}
pub type FIO_progressSetting_e = core::ffi::c_uint;
pub const FIO_ps_always: FIO_progressSetting_e = 2;
pub const FIO_ps_never: FIO_progressSetting_e = 1;
pub const FIO_ps_auto: FIO_progressSetting_e = 0;
pub type FIO_display_prefs_t = FIO_display_prefs_s;
pub type FIO_compressionType_t = core::ffi::c_uint;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
pub type FIO_dictBufferType_t = core::ffi::c_uint;
pub const FIO_mmapDict: FIO_dictBufferType_t = 1;
pub const FIO_mallocDict: FIO_dictBufferType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_Dict_t {
    pub dictBuffer: *mut core::ffi::c_void,
    pub dictBufferSize: size_t,
    pub dictBufferType: FIO_dictBufferType_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_ctx_s {
    pub nbFilesTotal: core::ffi::c_int,
    pub hasStdinInput: core::ffi::c_int,
    pub hasStdoutOutput: core::ffi::c_int,
    pub currFileIdx: core::ffi::c_int,
    pub nbFilesProcessed: core::ffi::c_int,
    pub totalBytesInput: size_t,
    pub totalBytesOutput: size_t,
}
pub type FIO_ctx_t = FIO_ctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cRess_t {
    pub dict: FIO_Dict_t,
    pub dictFileName: *const core::ffi::c_char,
    pub dictFileStat: stat_t,
    pub cctx: *mut ZSTD_CStream,
    pub writeCtx: *mut WritePoolCtx_t,
    pub readCtx: *mut ReadPoolCtx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct lzma_stream {
    next_in: *const u8,
    avail_in: size_t,
    total_in: u64,
    next_out: *mut u8,
    avail_out: size_t,
    total_out: u64,
    allocator: *const lzma_allocator,
    internal: *mut lzma_internal,
    reserved_ptr1: *mut core::ffi::c_void,
    reserved_ptr2: *mut core::ffi::c_void,
    reserved_ptr3: *mut core::ffi::c_void,
    reserved_ptr4: *mut core::ffi::c_void,
    seek_pos: u64,
    reserved_int2: u64,
    reserved_int3: size_t,
    reserved_int4: size_t,
    reserved_enum1: lzma_reserved_enum,
    reserved_enum2: lzma_reserved_enum,
}
type lzma_reserved_enum = core::ffi::c_uint;
const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
struct lzma_allocator {
    alloc: Option<
        unsafe extern "C" fn(*mut core::ffi::c_void, size_t, size_t) -> *mut core::ffi::c_void,
    >,
    free: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> ()>,
    opaque: *mut core::ffi::c_void,
}
const LZMA_STREAM_END: lzma_ret = 1;
type lzma_ret = core::ffi::c_uint;
const LZMA_BUF_ERROR: lzma_ret = 10;
const LZMA_OK: lzma_ret = 0;
type lzma_action = core::ffi::c_uint;
const LZMA_FINISH: lzma_action = 3;
const LZMA_RUN: lzma_action = 0;
type lzma_check = core::ffi::c_uint;
const LZMA_CHECK_CRC64: lzma_check = 4;
#[derive(Copy, Clone)]
#[repr(C)]
struct lzma_options_lzma {
    dict_size: u32,
    preset_dict: *const u8,
    preset_dict_size: u32,
    lc: u32,
    lp: u32,
    pb: u32,
    mode: lzma_mode,
    nice_len: u32,
    mf: lzma_match_finder,
    depth: u32,
    ext_flags: u32,
    ext_size_low: u32,
    ext_size_high: u32,
    reserved_int4: u32,
    reserved_int5: u32,
    reserved_int6: u32,
    reserved_int7: u32,
    reserved_int8: u32,
    reserved_enum1: lzma_reserved_enum,
    reserved_enum2: lzma_reserved_enum,
    reserved_enum3: lzma_reserved_enum,
    reserved_enum4: lzma_reserved_enum,
    reserved_ptr1: *mut core::ffi::c_void,
    reserved_ptr2: *mut core::ffi::c_void,
}
type lzma_match_finder = core::ffi::c_uint;
type lzma_mode = core::ffi::c_uint;
type lzma_bool = core::ffi::c_uchar;
type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
struct z_stream_s {
    next_in: *mut Bytef,
    avail_in: uInt,
    total_in: uLong,
    next_out: *mut Bytef,
    avail_out: uInt,
    total_out: uLong,
    msg: *mut core::ffi::c_char,
    state: *mut internal_state,
    zalloc: alloc_func,
    zfree: free_func,
    opaque: voidpf,
    data_type: core::ffi::c_int,
    adler: uLong,
    reserved: uLong,
}
type uLong = core::ffi::c_ulong;
type voidpf = *mut core::ffi::c_void;
type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
type uInt = core::ffi::c_uint;
type Bytef = Byte;
type Byte = core::ffi::c_uchar;
type z_streamp = *mut z_stream;
type speedChange_e = core::ffi::c_uint;
const faster: speedChange_e = 2;
const slower: speedChange_e = 1;
const noChange: speedChange_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
struct dRess_t {
    dict: FIO_Dict_t,
    dctx: *mut ZSTD_DStream,
    writeCtx: *mut WritePoolCtx_t,
    readCtx: *mut ReadPoolCtx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct fileInfo_t {
    decompressedSize: u64,
    compressedSize: u64,
    windowSize: u64,
    numActualFrames: core::ffi::c_int,
    numSkippableFrames: core::ffi::c_int,
    decompUnavailable: core::ffi::c_int,
    usesCheck: core::ffi::c_int,
    checksum: [u8; 4],
    nbFiles: u32,
    dictID: core::ffi::c_uint,
}
type InfoError = core::ffi::c_uint;
const info_truncated_input: InfoError = 4;
const info_file_error: InfoError = 3;
const info_not_zstd: InfoError = 2;
const info_frame_error: InfoError = 1;
const info_success: InfoError = 0;
const _IOFBF: core::ffi::c_int = 0;
const UINT64_MAX: u64 = 18446744073709551615;

const PATH_SEP: core::ffi::c_int = '/' as i32;
const UTIL_FILESIZE_UNKNOWN: core::ffi::c_int = -(1);
const SEC_TO_MICRO: core::ffi::c_int = 1000000;
const stdinmark: &CStr = c"/*stdin*\\";
const stdoutmark: &CStr = c"/*stdout*\\";
const nulmark: &CStr = c"/dev/null";
const LZMA_EXTENSION: &CStr = c".lzma";
const XZ_EXTENSION: &CStr = c".xz";
const TXZ_EXTENSION: &CStr = c".txz";
const GZ_EXTENSION: &CStr = c".gz";
const TGZ_EXTENSION: &CStr = c".tgz";
const ZSTD_EXTENSION: &CStr = c".zst";
const TZSTD_EXTENSION: &CStr = c"tzst";
const ZSTD_ALT_EXTENSION: &CStr = c"zstd";
const LZ4_EXTENSION: &CStr = c".lz4";
const TLZ4_EXTENSION: &CStr = c"tlz4";
pub static mut g_display_prefs: FIO_display_prefs_t = {
    FIO_display_prefs_s {
        displayLevel: 2,
        progressSetting: FIO_ps_auto,
    }
};
static mut g_displayClock: UTIL_time_t = UTIL_time_t { t: 0 };
const ZLIB_VERSION: &CStr = c"1.3";
const Z_NO_FLUSH: core::ffi::c_int = 0;
const Z_FINISH: core::ffi::c_int = 4;
const Z_OK: core::ffi::c_int = 0;
const Z_STREAM_END: core::ffi::c_int = 1;
const Z_BUF_ERROR: core::ffi::c_int = -(5);
const Z_BEST_COMPRESSION: core::ffi::c_int = 9;
const ZSTD_SPARSE_DEFAULT: core::ffi::c_int = 1;
const REFRESH_RATE: PTime = SEC_TO_MICRO as PTime / 6;
const LZ4_MAGICNUMBER: core::ffi::c_int = 0x184d2204 as core::ffi::c_int;
pub unsafe fn FIO_zlibVersion() -> *const core::ffi::c_char {
    zlibVersion()
}
pub unsafe fn FIO_lz4Version() -> *const core::ffi::c_char {
    b"Unsupported\0" as *const u8 as *const core::ffi::c_char
}
pub unsafe fn FIO_lzmaVersion() -> *const core::ffi::c_char {
    lzma_version_string()
}
pub const ADAPT_WINDOWLOG_DEFAULT: core::ffi::c_int = 23;
pub const DICTSIZE_MAX: core::ffi::c_int = 32 * ((1) << 20);
pub const DEFAULT_FILE_PERMISSIONS: mode_t =
    S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
pub const TEMPORARY_FILE_PERMISSIONS: mode_t = S_IRUSR | S_IWUSR;
static mut g_artefact: *const core::ffi::c_char = core::ptr::null();
unsafe extern "C" fn INThandler(sig: core::ffi::c_int) {
    assert!(sig == SIGINT);
    signal(sig, SIG_IGN);
    if !g_artefact.is_null() {
        assert!(UTIL_isRegularFile(g_artefact) != 0);
        remove(g_artefact);
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
    exit(2);
}
unsafe fn addHandler(dstFileName: *const core::ffi::c_char) {
    if UTIL_isRegularFile(dstFileName) != 0 {
        g_artefact = dstFileName;
        signal(SIGINT, INThandler as *const () as sighandler_t);
    } else {
        g_artefact = core::ptr::null();
    };
}
unsafe fn clearHandler() {
    if !g_artefact.is_null() {
        signal(SIGINT, SIG_DFL);
    }
    g_artefact = core::ptr::null();
}
pub unsafe fn FIO_addAbortHandler() {}
unsafe fn FIO_shouldDisplayFileSummary(fCtx: *const FIO_ctx_t) -> core::ffi::c_int {
    core::ffi::c_int::from((*fCtx).nbFilesTotal <= 1 || g_display_prefs.displayLevel >= 3)
}
unsafe fn FIO_shouldDisplayMultipleFileSummary(fCtx: *const FIO_ctx_t) -> core::ffi::c_int {
    let shouldDisplay =
        core::ffi::c_int::from((*fCtx).nbFilesProcessed >= 1 && (*fCtx).nbFilesTotal > 1);
    assert!(
        shouldDisplay != 0
            || FIO_shouldDisplayFileSummary(fCtx) != 0
            || (*fCtx).nbFilesProcessed == 0
    );
    shouldDisplay
}
pub const FIO_OVERLAP_LOG_NOTSET: core::ffi::c_int = 9999;
pub const FIO_LDM_PARAM_NOTSET: core::ffi::c_int = 9999;
pub unsafe fn FIO_createPreferences() -> *mut FIO_prefs_t {
    let ret = malloc(::core::mem::size_of::<FIO_prefs_t>()) as *mut FIO_prefs_t;
    if ret.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                281,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                21,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Allocation error : not enough memory\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(21);
    }
    (*ret).compressionType = FIO_zstdCompression;
    (*ret).overwrite = 0;
    (*ret).sparseFileSupport = ZSTD_SPARSE_DEFAULT;
    (*ret).dictIDFlag = 1;
    (*ret).checksumFlag = 1;
    (*ret).removeSrcFile = 0;
    (*ret).memLimit = 0;
    (*ret).nbWorkers = 1;
    (*ret).jobSize = 0;
    (*ret).overlapLog = FIO_OVERLAP_LOG_NOTSET;
    (*ret).adaptiveMode = 0;
    (*ret).rsyncable = 0;
    (*ret).minAdaptLevel = -(50);
    (*ret).maxAdaptLevel = 22;
    (*ret).ldmFlag = 0;
    (*ret).ldmHashLog = 0;
    (*ret).ldmMinMatch = 0;
    (*ret).ldmBucketSizeLog = FIO_LDM_PARAM_NOTSET;
    (*ret).ldmHashRateLog = FIO_LDM_PARAM_NOTSET;
    (*ret).streamSrcSize = 0;
    (*ret).targetCBlockSize = 0;
    (*ret).srcSizeHint = 0;
    (*ret).testMode = 0;
    (*ret).literalCompressionMode = ZSTD_ParamSwitch_e::ZSTD_ps_auto;
    (*ret).excludeCompressedFiles = 0;
    (*ret).allowBlockDevices = 0;
    (*ret).asyncIO = AIO_supported();
    (*ret).passThrough = -(1);
    ret
}
pub unsafe fn FIO_createContext() -> *mut FIO_ctx_t {
    let ret = malloc(::core::mem::size_of::<FIO_ctx_t>()) as *mut FIO_ctx_t;
    if ret.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                317,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                21,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Allocation error : not enough memory\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(21);
    }
    (*ret).currFileIdx = 0;
    (*ret).hasStdinInput = 0;
    (*ret).hasStdoutOutput = 0;
    (*ret).nbFilesTotal = 1;
    (*ret).nbFilesProcessed = 0;
    (*ret).totalBytesInput = 0;
    (*ret).totalBytesOutput = 0;
    ret
}
pub unsafe fn FIO_freePreferences(prefs: *mut FIO_prefs_t) {
    free(prefs as *mut core::ffi::c_void);
}
pub unsafe fn FIO_freeContext(fCtx: *mut FIO_ctx_t) {
    free(fCtx as *mut core::ffi::c_void);
}
pub unsafe fn FIO_setNotificationLevel(level: core::ffi::c_int) {
    g_display_prefs.displayLevel = level;
}
pub unsafe fn FIO_setProgressSetting(setting: FIO_progressSetting_e) {
    g_display_prefs.progressSetting = setting;
}
pub unsafe fn FIO_setCompressionType(
    prefs: *mut FIO_prefs_t,
    compressionType: FIO_compressionType_t,
) {
    (*prefs).compressionType = compressionType;
}
pub unsafe fn FIO_overwriteMode(prefs: *mut FIO_prefs_t) {
    (*prefs).overwrite = 1;
}
pub unsafe fn FIO_setSparseWrite(prefs: *mut FIO_prefs_t, sparse: core::ffi::c_int) {
    (*prefs).sparseFileSupport = sparse;
}
pub unsafe fn FIO_setDictIDFlag(prefs: *mut FIO_prefs_t, dictIDFlag: core::ffi::c_int) {
    (*prefs).dictIDFlag = dictIDFlag;
}
pub unsafe fn FIO_setChecksumFlag(prefs: *mut FIO_prefs_t, checksumFlag: core::ffi::c_int) {
    (*prefs).checksumFlag = checksumFlag;
}
pub unsafe fn FIO_setRemoveSrcFile(prefs: *mut FIO_prefs_t, flag: core::ffi::c_int) {
    (*prefs).removeSrcFile = core::ffi::c_int::from(flag != 0);
}
pub unsafe fn FIO_setMemLimit(prefs: *mut FIO_prefs_t, memLimit: core::ffi::c_uint) {
    (*prefs).memLimit = memLimit;
}
pub unsafe fn FIO_setNbWorkers(prefs: *mut FIO_prefs_t, nbWorkers: core::ffi::c_int) {
    (*prefs).nbWorkers = nbWorkers;
}
pub unsafe fn FIO_setExcludeCompressedFile(
    prefs: *mut FIO_prefs_t,
    excludeCompressedFiles: core::ffi::c_int,
) {
    (*prefs).excludeCompressedFiles = excludeCompressedFiles;
}
pub unsafe fn FIO_setAllowBlockDevices(
    prefs: *mut FIO_prefs_t,
    allowBlockDevices: core::ffi::c_int,
) {
    (*prefs).allowBlockDevices = allowBlockDevices;
}
pub unsafe fn FIO_setJobSize(prefs: *mut FIO_prefs_t, jobSize: core::ffi::c_int) {
    if jobSize != 0 && (*prefs).nbWorkers == 0 && g_display_prefs.displayLevel >= 2 {
        fprintf(
            stderr,
            b"Setting block size is useless in single-thread mode \n\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*prefs).jobSize = jobSize;
}
pub unsafe fn FIO_setOverlapLog(prefs: *mut FIO_prefs_t, overlapLog: core::ffi::c_int) {
    if overlapLog != 0 && (*prefs).nbWorkers == 0 && g_display_prefs.displayLevel >= 2 {
        fprintf(
            stderr,
            b"Setting overlapLog is useless in single-thread mode \n\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*prefs).overlapLog = overlapLog;
}
pub unsafe fn FIO_setAdaptiveMode(prefs: *mut FIO_prefs_t, adapt: core::ffi::c_int) {
    if adapt > 0 && (*prefs).nbWorkers == 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                394,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                1,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Adaptive mode is not compatible with single thread mode \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(1);
    }
    (*prefs).adaptiveMode = adapt;
}
pub unsafe fn FIO_setUseRowMatchFinder(
    prefs: *mut FIO_prefs_t,
    useRowMatchFinder: core::ffi::c_int,
) {
    (*prefs).useRowMatchFinder = useRowMatchFinder;
}
pub unsafe fn FIO_setRsyncable(prefs: *mut FIO_prefs_t, rsyncable: core::ffi::c_int) {
    if rsyncable > 0 && (*prefs).nbWorkers == 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                404,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                1,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Rsyncable mode is not compatible with single thread mode \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(1);
    }
    (*prefs).rsyncable = rsyncable;
}
pub unsafe fn FIO_setStreamSrcSize(prefs: *mut FIO_prefs_t, streamSrcSize: size_t) {
    (*prefs).streamSrcSize = streamSrcSize;
}
pub unsafe fn FIO_setTargetCBlockSize(prefs: *mut FIO_prefs_t, targetCBlockSize: size_t) {
    (*prefs).targetCBlockSize = targetCBlockSize;
}
pub unsafe fn FIO_setSrcSizeHint(prefs: *mut FIO_prefs_t, srcSizeHint: size_t) {
    (*prefs).srcSizeHint = (if (2147483647) < srcSizeHint {
        2147483647
    } else {
        srcSizeHint
    }) as core::ffi::c_int;
}
pub unsafe fn FIO_setTestMode(prefs: *mut FIO_prefs_t, testMode: core::ffi::c_int) {
    (*prefs).testMode = core::ffi::c_int::from(testMode != 0);
}
pub unsafe fn FIO_setLiteralCompressionMode(prefs: *mut FIO_prefs_t, mode: ZSTD_ParamSwitch_e) {
    (*prefs).literalCompressionMode = mode;
}
pub unsafe fn FIO_setAdaptMin(prefs: *mut FIO_prefs_t, minCLevel: core::ffi::c_int) {
    assert!(minCLevel >= ZSTD_minCLevel());
    (*prefs).minAdaptLevel = minCLevel;
}
pub unsafe fn FIO_setAdaptMax(prefs: *mut FIO_prefs_t, maxCLevel: core::ffi::c_int) {
    (*prefs).maxAdaptLevel = maxCLevel;
}
pub unsafe fn FIO_setLdmFlag(prefs: *mut FIO_prefs_t, ldmFlag: core::ffi::c_uint) {
    (*prefs).ldmFlag = core::ffi::c_int::from(ldmFlag > 0);
}
pub unsafe fn FIO_setLdmHashLog(prefs: *mut FIO_prefs_t, ldmHashLog: core::ffi::c_int) {
    (*prefs).ldmHashLog = ldmHashLog;
}
pub unsafe fn FIO_setLdmMinMatch(prefs: *mut FIO_prefs_t, ldmMinMatch: core::ffi::c_int) {
    (*prefs).ldmMinMatch = ldmMinMatch;
}
pub unsafe fn FIO_setLdmBucketSizeLog(prefs: *mut FIO_prefs_t, ldmBucketSizeLog: core::ffi::c_int) {
    (*prefs).ldmBucketSizeLog = ldmBucketSizeLog;
}
pub unsafe fn FIO_setLdmHashRateLog(prefs: *mut FIO_prefs_t, ldmHashRateLog: core::ffi::c_int) {
    (*prefs).ldmHashRateLog = ldmHashRateLog;
}
pub unsafe fn FIO_setPatchFromMode(prefs: *mut FIO_prefs_t, value: core::ffi::c_int) {
    (*prefs).patchFromMode = core::ffi::c_int::from(value != 0);
}
pub unsafe fn FIO_setContentSize(prefs: *mut FIO_prefs_t, value: core::ffi::c_int) {
    (*prefs).contentSize = core::ffi::c_int::from(value != 0);
}
pub unsafe fn FIO_setAsyncIOFlag(prefs: *mut FIO_prefs_t, value: core::ffi::c_int) {
    (*prefs).asyncIO = value;
}
pub unsafe fn FIO_setPassThroughFlag(prefs: *mut FIO_prefs_t, value: core::ffi::c_int) {
    (*prefs).passThrough = core::ffi::c_int::from(value != 0);
}
pub unsafe fn FIO_setMMapDict(prefs: *mut FIO_prefs_t, value: ZSTD_ParamSwitch_e) {
    (*prefs).mmapDict = value;
}
pub unsafe fn FIO_setHasStdoutOutput(fCtx: *mut FIO_ctx_t, value: core::ffi::c_int) {
    (*fCtx).hasStdoutOutput = value;
}
pub unsafe fn FIO_setNbFilesTotal(fCtx: *mut FIO_ctx_t, value: core::ffi::c_int) {
    (*fCtx).nbFilesTotal = value;
}
pub unsafe fn FIO_determineHasStdinInput(fCtx: *mut FIO_ctx_t, filenames: *const FileNamesTable) {
    let mut i = 0;
    while i < (*filenames).tableSize {
        if strcmp(stdinmark.as_ptr(), *((*filenames).fileNames).add(i)) == 0 {
            (*fCtx).hasStdinInput = 1;
            return;
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn FIO_removeFile(path: *const core::ffi::c_char) -> core::ffi::c_int {
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
        if g_display_prefs.displayLevel >= 2 {
            fprintf(
                stderr,
                b"zstd: Failed to stat %s while trying to remove it\n\0" as *const u8
                    as *const core::ffi::c_char,
                path,
            );
        }
        return 0;
    }
    if UTIL_isRegularFileStat(&statbuf) == 0 {
        if g_display_prefs.displayLevel >= 2 {
            fprintf(
                stderr,
                b"zstd: Refusing to remove non-regular file %s\n\0" as *const u8
                    as *const core::ffi::c_char,
                path,
            );
        }
        return 0;
    }
    remove(path)
}
unsafe fn FIO_openSrcFile(
    prefs: *const FIO_prefs_t,
    srcFileName: *const core::ffi::c_char,
    statbuf: *mut stat_t,
) -> *mut FILE {
    let allowBlockDevices = if !prefs.is_null() {
        (*prefs).allowBlockDevices
    } else {
        0
    };
    assert!(!srcFileName.is_null());
    assert!(!statbuf.is_null());
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"Using stdin for input \n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        return stdin;
    }
    if UTIL_stat(srcFileName, statbuf) == 0 {
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "zstd: can't stat {} : {} -- ignored",
                CStr::from_ptr(srcFileName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        return core::ptr::null_mut();
    }
    if UTIL_isRegularFileStat(statbuf) == 0
        && UTIL_isFIFOStat(statbuf) == 0
        && UTIL_isFileDescriptorPipe(srcFileName) == 0
        && !(allowBlockDevices != 0 && UTIL_isBlockDevStat(statbuf) != 0)
    {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s is not a regular file -- ignored \n\0" as *const u8
                    as *const core::ffi::c_char,
                srcFileName,
            );
        }
        return core::ptr::null_mut();
    }
    let f = fopen(
        srcFileName,
        b"rb\0" as *const u8 as *const core::ffi::c_char,
    );
    if f.is_null() && g_display_prefs.displayLevel >= 1 {
        eprintln!(
            "zstd: {}: {}",
            CStr::from_ptr(srcFileName).to_string_lossy(),
            io::Error::last_os_error(),
        );
    }
    f
}
unsafe fn FIO_openDstFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    srcFileName: *const core::ffi::c_char,
    dstFileName: *const core::ffi::c_char,
    mode: mode_t,
) -> *mut FILE {
    if (*prefs).testMode != 0 {
        return core::ptr::null_mut();
    }
    assert!(!dstFileName.is_null());
    if strcmp(dstFileName, stdoutmark.as_ptr()) == 0 {
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"Using stdout for output \n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        if (*prefs).sparseFileSupport == 1 {
            (*prefs).sparseFileSupport = 0;
            if g_display_prefs.displayLevel >= 4 {
                fprintf(
                    stderr,
                    b"Sparse File Support is automatically disabled on stdout ; try --sparse \n\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
        }
        return stdout;
    }
    if !srcFileName.is_null() && UTIL_isSameFile(srcFileName, dstFileName) != 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: Refusing to open an output file which will overwrite the input file \n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        return core::ptr::null_mut();
    }
    if UTIL_isRegularFile(dstFileName) != 0 {
        if strcmp(dstFileName, nulmark.as_ptr()) == 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    614,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    40,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s is unexpectedly categorized as a regular file\0" as *const u8
                        as *const core::ffi::c_char,
                    dstFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(40);
        }
        if (*prefs).overwrite == 0 {
            if g_display_prefs.displayLevel <= 1 {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"zstd: %s already exists; not overwritten  \n\0" as *const u8
                            as *const core::ffi::c_char,
                        dstFileName,
                    );
                }
                return core::ptr::null_mut();
            }
            fprintf(
                stderr,
                b"zstd: %s already exists; \0" as *const u8 as *const core::ffi::c_char,
                dstFileName,
            );
            if UTIL_requireUserConfirmation(
                b"overwrite (y/n) ? \0" as *const u8 as *const core::ffi::c_char,
                b"Not overwritten  \n\0" as *const u8 as *const core::ffi::c_char,
                b"yY\0" as *const u8 as *const core::ffi::c_char,
                (*fCtx).hasStdinInput,
            ) != 0
            {
                return core::ptr::null_mut();
            }
        }
        FIO_removeFile(dstFileName);
    }
    let mut isDstRegFile: core::ffi::c_int = 0;
    let openflags = O_WRONLY | O_CREAT | O_TRUNC;
    let fd = open(dstFileName, openflags, mode as core::ffi::c_uint);
    let mut f = core::ptr::null_mut();
    if fd != -(1) {
        f = fdopen(fd, b"wb\0" as *const u8 as *const core::ffi::c_char);
    }
    isDstRegFile = UTIL_isFdRegularFile(fd);
    if (*prefs).sparseFileSupport == 1 {
        (*prefs).sparseFileSupport = ZSTD_SPARSE_DEFAULT;
        if isDstRegFile == 0 {
            (*prefs).sparseFileSupport = 0;
            if g_display_prefs.displayLevel >= 4 {
                fprintf(
                    stderr,
                    b"Sparse File Support is disabled when output is not a file \n\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        }
    }
    if f.is_null() {
        if UTIL_isFileDescriptorPipe(dstFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: error: no output specified (use -o or -c). \n\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
        } else if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "zstd: {}: {}",
                CStr::from_ptr(dstFileName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
    } else if setvbuf(f, core::ptr::null_mut(), _IOFBF, ((1) << 20) as size_t) != 0
        && g_display_prefs.displayLevel >= 2
    {
        fprintf(
            stderr,
            b"Warning: setvbuf failed for %s\n\0" as *const u8 as *const core::ffi::c_char,
            dstFileName,
        );
    }
    f
}
unsafe fn FIO_getDictFileStat(fileName: *const core::ffi::c_char, dictFileStat: *mut stat_t) {
    assert!(!dictFileStat.is_null());
    if fileName.is_null() {
        return;
    }
    if UTIL_stat(fileName, dictFileStat) == 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                698,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                31,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "Stat failed on dictionary file {}: {}",
                CStr::from_ptr(fileName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        exit(31);
    }
    if UTIL_isRegularFileStat(dictFileStat) == 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                702,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                32,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Dictionary %s must be a regular file.\0" as *const u8 as *const core::ffi::c_char,
                fileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(32);
    }
}
unsafe fn FIO_setDictBufferMalloc(
    dict: *mut FIO_Dict_t,
    fileName: *const core::ffi::c_char,
    prefs: *mut FIO_prefs_t,
    dictFileStat: *mut stat_t,
) -> size_t {
    let mut fileHandle = core::ptr::null_mut::<FILE>();
    let mut fileSize: size_t = 0;
    let bufferPtr: *mut *mut core::ffi::c_void = &mut (*dict).dictBuffer;
    assert!(!bufferPtr.is_null());
    assert!(!dictFileStat.is_null());
    *bufferPtr = core::ptr::null_mut();
    if fileName.is_null() {
        return 0;
    }
    if g_display_prefs.displayLevel >= 4 {
        fprintf(
            stderr,
            b"Loading %s as dictionary \n\0" as *const u8 as *const core::ffi::c_char,
            fileName,
        );
    }
    fileHandle = fopen(fileName, b"rb\0" as *const u8 as *const core::ffi::c_char);
    if fileHandle.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                728,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                33,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "Couldn't open dictionary {}: {}",
                CStr::from_ptr(fileName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        exit(33);
    }
    fileSize = UTIL_getFileSizeStat(dictFileStat) as size_t;
    let dictSizeMax = (if (*prefs).patchFromMode != 0 {
        (*prefs).memLimit
    } else {
        DICTSIZE_MAX as core::ffi::c_uint
    }) as size_t;
    if fileSize > dictSizeMax {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                736,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                34,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Dictionary file %s is too large (> %u bytes)\0" as *const u8
                    as *const core::ffi::c_char,
                fileName,
                dictSizeMax as core::ffi::c_uint,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(34);
    }
    *bufferPtr = malloc(fileSize);
    if (*bufferPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                740,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                34,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!("{}", io::Error::last_os_error());
        }
        exit(34);
    }
    let readSize = fread(*bufferPtr, 1, fileSize, fileHandle);
    if readSize != fileSize {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                744,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                35,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "Error reading dictionary file {} : {}",
                CStr::from_ptr(fileName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        exit(35);
    }
    fclose(fileHandle);
    fileSize
}
pub const PROT_READ: core::ffi::c_int = 0x1 as core::ffi::c_int;
pub const MAP_PRIVATE: core::ffi::c_int = 0x2 as core::ffi::c_int;
unsafe fn FIO_munmap(dict: *mut FIO_Dict_t) {
    munmap((*dict).dictBuffer, (*dict).dictBufferSize);
    (*dict).dictBuffer = core::ptr::null_mut();
    (*dict).dictBufferSize = 0;
}
unsafe fn FIO_setDictBufferMMap(
    dict: *mut FIO_Dict_t,
    fileName: *const core::ffi::c_char,
    prefs: *mut FIO_prefs_t,
    dictFileStat: *mut stat_t,
) -> size_t {
    let mut fileHandle: core::ffi::c_int = 0;
    let mut fileSize: u64 = 0;
    let bufferPtr: *mut *mut core::ffi::c_void = &mut (*dict).dictBuffer;
    assert!(!bufferPtr.is_null());
    assert!(!dictFileStat.is_null());
    *bufferPtr = core::ptr::null_mut();
    if fileName.is_null() {
        return 0;
    }
    if g_display_prefs.displayLevel >= 4 {
        fprintf(
            stderr,
            b"Loading %s as dictionary \n\0" as *const u8 as *const core::ffi::c_char,
            fileName,
        );
    }
    fileHandle = open(fileName, O_RDONLY);
    if fileHandle == -(1) {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                775,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                33,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "Couldn't open dictionary {}: {}",
                CStr::from_ptr(fileName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        exit(33);
    }
    fileSize = UTIL_getFileSizeStat(dictFileStat);
    let dictSizeMax = (if (*prefs).patchFromMode != 0 {
        (*prefs).memLimit
    } else {
        DICTSIZE_MAX as core::ffi::c_uint
    }) as size_t;
    if fileSize as size_t > dictSizeMax {
        if g_display_prefs.displayLevel >= 1 as core::ffi::c_int {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                783,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                34,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Dictionary file %s is too large (> %u bytes)\0" as *const u8
                    as *const core::ffi::c_char,
                fileName,
                dictSizeMax as core::ffi::c_uint,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(34);
    }
    *bufferPtr = mmap(
        core::ptr::null_mut(),
        fileSize as size_t,
        PROT_READ,
        MAP_PRIVATE,
        fileHandle,
        0,
    );
    if (*bufferPtr).is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                788,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                34,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!("{}", io::Error::last_os_error());
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(34);
    }
    close(fileHandle);
    fileSize as size_t
}
unsafe fn FIO_freeDict(dict: *mut FIO_Dict_t) {
    if (*dict).dictBufferType as core::ffi::c_uint
        == FIO_mallocDict as core::ffi::c_int as core::ffi::c_uint
    {
        free((*dict).dictBuffer);
        (*dict).dictBuffer = core::ptr::null_mut();
        (*dict).dictBufferSize = 0;
    } else if (*dict).dictBufferType as core::ffi::c_uint
        == FIO_mmapDict as core::ffi::c_int as core::ffi::c_uint
    {
        FIO_munmap(dict);
    } else {
        unreachable!();
    };
}
unsafe fn FIO_initDict(
    dict: *mut FIO_Dict_t,
    fileName: *const core::ffi::c_char,
    prefs: *mut FIO_prefs_t,
    dictFileStat: *mut stat_t,
    dictBufferType: FIO_dictBufferType_t,
) {
    (*dict).dictBufferType = dictBufferType;
    if (*dict).dictBufferType as core::ffi::c_uint
        == FIO_mallocDict as core::ffi::c_int as core::ffi::c_uint
    {
        (*dict).dictBufferSize = FIO_setDictBufferMalloc(dict, fileName, prefs, dictFileStat);
    } else if (*dict).dictBufferType as core::ffi::c_uint
        == FIO_mmapDict as core::ffi::c_int as core::ffi::c_uint
    {
        (*dict).dictBufferSize = FIO_setDictBufferMMap(dict, fileName, prefs, dictFileStat);
    } else {
        unreachable!();
    };
}
pub unsafe fn FIO_checkFilenameCollisions(
    filenameTable: *mut *const core::ffi::c_char,
    nbFiles: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut filenameTableSorted = core::ptr::null_mut::<*const core::ffi::c_char>();
    let mut prevElem = core::ptr::null::<core::ffi::c_char>();
    let mut filename = core::ptr::null::<core::ffi::c_char>();
    let mut u: core::ffi::c_uint = 0;
    filenameTableSorted =
        malloc((::core::mem::size_of::<*mut core::ffi::c_char>()).wrapping_mul(nbFiles as size_t))
            as *mut *const core::ffi::c_char;
    if filenameTableSorted.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Allocation error during filename collision checking \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        return 1;
    }
    u = 0;
    while u < nbFiles {
        filename = strrchr(*filenameTable.offset(u as isize), PATH_SEP);
        if filename.is_null() {
            let fresh0 = &mut (*filenameTableSorted.offset(u as isize));
            *fresh0 = *filenameTable.offset(u as isize);
        } else {
            let fresh1 = &mut (*filenameTableSorted.offset(u as isize));
            *fresh1 = filename.offset(1);
        }
        u = u.wrapping_add(1);
    }
    qsort(
        filenameTableSorted as *mut core::ffi::c_void,
        nbFiles as size_t,
        ::core::mem::size_of::<*mut core::ffi::c_char>(),
        Some(
            UTIL_compareStr
                as unsafe extern "C" fn(
                    *const core::ffi::c_void,
                    *const core::ffi::c_void,
                ) -> core::ffi::c_int,
        ),
    );
    prevElem = *filenameTableSorted.offset(0);
    u = 1;
    while u < nbFiles {
        if strcmp(prevElem, *filenameTableSorted.offset(u as isize)) == 0
            && g_display_prefs.displayLevel >= 2
        {
            fprintf(
                stderr,
                b"WARNING: Two files have same filename: %s\n\0" as *const u8
                    as *const core::ffi::c_char,
                prevElem,
            );
        }
        prevElem = *filenameTableSorted.offset(u as isize);
        u = u.wrapping_add(1);
    }
    free(filenameTableSorted as *mut core::ffi::c_void);
    0
}
unsafe fn extractFilename(
    path: *const core::ffi::c_char,
    separator: core::ffi::c_char,
) -> *const core::ffi::c_char {
    let search: *const core::ffi::c_char = strrchr(path, core::ffi::c_int::from(separator));
    if search.is_null() {
        return path;
    }
    search.offset(1)
}
unsafe fn FIO_createFilename_fromOutDir(
    path: *const core::ffi::c_char,
    outDirName: *const core::ffi::c_char,
    suffixLen: size_t,
) -> *mut core::ffi::c_char {
    let mut filenameStart = core::ptr::null::<core::ffi::c_char>();
    let mut separator: core::ffi::c_char = 0;
    let mut result = core::ptr::null_mut::<core::ffi::c_char>();
    separator = '/' as i32 as core::ffi::c_char;
    filenameStart = extractFilename(path, separator);
    result = calloc(
        1,
        (strlen(outDirName))
            .wrapping_add(1)
            .wrapping_add(strlen(filenameStart))
            .wrapping_add(suffixLen)
            .wrapping_add(1),
    ) as *mut core::ffi::c_char;
    if result.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                945,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                30,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "zstd: FIO_createFilename_fromOutDir: {}",
                io::Error::last_os_error(),
            );
        }
        exit(30);
    }
    memcpy(
        result as *mut core::ffi::c_void,
        outDirName as *const core::ffi::c_void,
        strlen(outDirName),
    );
    if core::ffi::c_int::from(*outDirName.add((strlen(outDirName)).wrapping_sub(1)))
        == core::ffi::c_int::from(separator)
    {
        memcpy(
            result.add(strlen(outDirName)) as *mut core::ffi::c_void,
            filenameStart as *const core::ffi::c_void,
            strlen(filenameStart),
        );
    } else {
        memcpy(
            result.add(strlen(outDirName)) as *mut core::ffi::c_void,
            &mut separator as *mut core::ffi::c_char as *const core::ffi::c_void,
            1,
        );
        memcpy(
            result.add(strlen(outDirName)).offset(1) as *mut core::ffi::c_void,
            filenameStart as *const core::ffi::c_void,
            strlen(filenameStart),
        );
    }
    result
}
unsafe fn FIO_highbit64(mut v: core::ffi::c_ulonglong) -> core::ffi::c_uint {
    let mut count = 0 as core::ffi::c_uint;
    assert!(v != 0);
    v >>= 1;
    while v != 0 {
        v >>= 1;
        count = count.wrapping_add(1);
    }
    count
}
unsafe fn FIO_adjustMemLimitForPatchFromMode(
    prefs: *mut FIO_prefs_t,
    dictSize: core::ffi::c_ulonglong,
    maxSrcFileSize: core::ffi::c_ulonglong,
) {
    let maxSize = if core::ffi::c_ulonglong::from((*prefs).memLimit)
        > (if dictSize > maxSrcFileSize {
            dictSize
        } else {
            maxSrcFileSize
        }) {
        core::ffi::c_ulonglong::from((*prefs).memLimit)
    } else if dictSize > maxSrcFileSize {
        dictSize
    } else {
        maxSrcFileSize
    };
    let maxWindowSize = (1 as core::ffi::c_uint)
        << (if ::core::mem::size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        });
    if maxSize == UTIL_FILESIZE_UNKNOWN as core::ffi::c_ulonglong {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                979,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                42,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Using --patch-from with stdin requires --stream-size\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(42);
    }
    assert!(maxSize != UTIL_FILESIZE_UNKNOWN as core::ffi::c_ulonglong);
    if maxSize > core::ffi::c_ulonglong::from(maxWindowSize) {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                982,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                42,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Can't handle files larger than %u GB\n\0" as *const u8
                    as *const core::ffi::c_char,
                maxWindowSize.wrapping_div(
                    (1 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30),
                ),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(42);
    }
    FIO_setMemLimit(prefs, maxSize as core::ffi::c_uint);
}
unsafe fn FIO_multiFilesConcatWarning(
    fCtx: *const FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    outFileName: *const core::ffi::c_char,
    displayLevelCutoff: core::ffi::c_int,
) -> core::ffi::c_int {
    if (*fCtx).hasStdoutOutput != 0 && (*prefs).removeSrcFile != 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1009,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                43,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                    stderr,
                    b"It's not allowed to remove input files when processed output is piped to stdout. This scenario is not supposed to be possible. This is a programming error. File an issue for it to be fixed.\0"
                        as *const u8 as *const core::ffi::c_char,
                );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(43);
    }
    if (*prefs).testMode != 0 {
        if (*prefs).removeSrcFile != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1017,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    43,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Test mode shall not remove input files! This scenario is not supposed to be possible. This is a programming error. File an issue for it to be fixed.\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(43);
        }
        return 0;
    }
    if (*fCtx).nbFilesTotal == 1 {
        return 0;
    }
    assert!((*fCtx).nbFilesTotal > 1);
    if outFileName.is_null() {
        return 0;
    }
    if (*fCtx).hasStdoutOutput != 0 {
        if g_display_prefs.displayLevel >= 2 {
            fprintf(
                stderr,
                b"zstd: WARNING: all input files will be processed and concatenated into stdout. \n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
    } else if g_display_prefs.displayLevel >= 2 {
        fprintf(
            stderr,
            b"zstd: WARNING: all input files will be processed and concatenated into a single output file: %s \n\0"
                as *const u8 as *const core::ffi::c_char,
            outFileName,
        );
    }
    if g_display_prefs.displayLevel >= 2 {
        fprintf(
            stderr,
            b"The concatenated output CANNOT regenerate original file names nor directory structure. \n\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    if (*prefs).removeSrcFile != 0 {
        if g_display_prefs.displayLevel >= 2 {
            fprintf(
                stderr,
                b"Since it's a destructive operation, input files will not be removed. \n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        (*prefs).removeSrcFile = 0;
    }
    if (*fCtx).hasStdoutOutput != 0 {
        return 0;
    }
    if (*prefs).overwrite != 0 {
        return 0;
    }
    if g_display_prefs.displayLevel <= displayLevelCutoff {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Concatenating multiple processed inputs into a single output loses file metadata. \n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Aborting. \n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        return 1;
    }
    UTIL_requireUserConfirmation(
        b"Proceed? (y/n): \0" as *const u8 as *const core::ffi::c_char,
        b"Aborting...\0" as *const u8 as *const core::ffi::c_char,
        b"yY\0" as *const u8 as *const core::ffi::c_char,
        (*fCtx).hasStdinInput,
    )
}
unsafe fn setInBuffer(buf: *const core::ffi::c_void, s: size_t, pos: size_t) -> ZSTD_inBuffer {
    let mut i = ZSTD_inBuffer_s {
        src: core::ptr::null::<core::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    i.src = buf;
    i.size = s;
    i.pos = pos;
    i
}
unsafe fn setOutBuffer(buf: *mut core::ffi::c_void, s: size_t, pos: size_t) -> ZSTD_outBuffer {
    let mut o = ZSTD_outBuffer_s {
        dst: core::ptr::null_mut::<core::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    o.dst = buf;
    o.size = s;
    o.pos = pos;
    o
}
unsafe fn ZSTD_cycleLog(hashLog: u32, strat: ZSTD_strategy) -> u32 {
    let btScale = core::ffi::c_int::from(strat >= ZSTD_btlazy2 as core::ffi::c_int as u32) as u32;
    assert!(hashLog > 1);
    hashLog.wrapping_sub(btScale)
}
unsafe fn FIO_adjustParamsForPatchFromMode(
    prefs: *mut FIO_prefs_t,
    comprParams: *mut ZSTD_compressionParameters,
    dictSize: core::ffi::c_ulonglong,
    maxSrcFileSize: core::ffi::c_ulonglong,
    cLevel: core::ffi::c_int,
) {
    let fileWindowLog = (FIO_highbit64(maxSrcFileSize)).wrapping_add(1);
    let cParams = ZSTD_getCParams(
        cLevel,
        maxSrcFileSize as size_t as core::ffi::c_ulonglong,
        dictSize as size_t,
    );
    FIO_adjustMemLimitForPatchFromMode(prefs, dictSize, maxSrcFileSize);
    if fileWindowLog
        > (if ::core::mem::size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }) as core::ffi::c_uint
        && g_display_prefs.displayLevel >= 1
    {
        fprintf(
            stderr,
            b"Max window log exceeded by file (compression ratio will suffer)\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    (*comprParams).windowLog = if 10
        > (if ((if ::core::mem::size_of::<size_t>() == 4 {
            30
        } else {
            31
        }) as core::ffi::c_uint)
            < fileWindowLog
        {
            (if ::core::mem::size_of::<size_t>() == 4 {
                30
            } else {
                31
            }) as core::ffi::c_uint
        } else {
            fileWindowLog
        }) {
        10
    } else if ((if ::core::mem::size_of::<size_t>() == 4 {
        30
    } else {
        31
    }) as core::ffi::c_uint)
        < fileWindowLog
    {
        (if ::core::mem::size_of::<size_t>() == 4 {
            30
        } else {
            31
        }) as core::ffi::c_uint
    } else {
        fileWindowLog
    };
    if fileWindowLog > ZSTD_cycleLog(cParams.chainLog, cParams.strategy) {
        if (*prefs).ldmFlag == 0 && g_display_prefs.displayLevel >= 2 {
            fprintf(
                stderr,
                b"long mode automatically triggered\n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        FIO_setLdmFlag(prefs, 1);
    }
    if cParams.strategy as core::ffi::c_uint >= ZSTD_btopt as core::ffi::c_int as core::ffi::c_uint
    {
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"[Optimal parser notes] Consider the following to improve patch size at the cost of speed:\n\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"- Set a larger targetLength (e.g. --zstd=targetLength=4096)\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"- Set a larger chainLog (e.g. --zstd=chainLog=%u)\n\0" as *const u8
                    as *const core::ffi::c_char,
                if ::core::mem::size_of::<size_t>() == 4 {
                    29
                } else {
                    30
                },
            );
        }
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"- Set a larger LDM hashLog (e.g. --zstd=ldmHashLog=%u)\n\0" as *const u8
                    as *const core::ffi::c_char,
                if (if ::core::mem::size_of::<size_t>() == 4 {
                    30
                } else {
                    31
                }) < 30
                {
                    if ::core::mem::size_of::<size_t>() == 4 {
                        30
                    } else {
                        31
                    }
                } else {
                    30
                },
            );
        }
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"- Set a smaller LDM rateLog (e.g. --zstd=ldmHashRateLog=%u)\n\0" as *const u8
                    as *const core::ffi::c_char,
                0,
            );
        }
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"Also consider playing around with searchLog and hashLog\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
    }
}
unsafe fn FIO_createCResources(
    prefs: *mut FIO_prefs_t,
    dictFileName: *const core::ffi::c_char,
    maxSrcFileSize: core::ffi::c_ulonglong,
    cLevel: core::ffi::c_int,
    mut comprParams: ZSTD_compressionParameters,
) -> cRess_t {
    let mut useMMap =
        core::ffi::c_int::from((*prefs).mmapDict == ZSTD_ParamSwitch_e::ZSTD_ps_enable);
    let forceNoUseMMap =
        core::ffi::c_int::from((*prefs).mmapDict == ZSTD_ParamSwitch_e::ZSTD_ps_disable);
    let mut dictBufferType = FIO_mallocDict;
    let mut ress = cRess_t {
        dict: FIO_Dict_t {
            dictBuffer: core::ptr::null_mut::<core::ffi::c_void>(),
            dictBufferSize: 0,
            dictBufferType: FIO_mallocDict,
        },
        dictFileName: core::ptr::null::<core::ffi::c_char>(),
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
        cctx: core::ptr::null_mut::<ZSTD_CStream>(),
        writeCtx: core::ptr::null_mut::<WritePoolCtx_t>(),
        readCtx: core::ptr::null_mut::<ReadPoolCtx_t>(),
    };
    ptr::write_bytes(
        &mut ress as *mut cRess_t as *mut u8,
        0,
        ::core::mem::size_of::<cRess_t>(),
    );
    if g_display_prefs.displayLevel >= 6 {
        fprintf(
            stderr,
            b"FIO_createCResources \n\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    ress.cctx = ZSTD_createCCtx();
    if (ress.cctx).is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1134,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                30,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "allocation error ({}): can't create ZSTD_CCtx",
                io::Error::last_os_error(),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(30);
    }
    FIO_getDictFileStat(dictFileName, &mut ress.dictFileStat);
    if (*prefs).patchFromMode != 0 {
        let dictSize = UTIL_getFileSizeStat(&ress.dictFileStat);
        let ssSize = (*prefs).streamSrcSize as core::ffi::c_ulonglong;
        useMMap |= core::ffi::c_int::from(dictSize > u64::from((*prefs).memLimit));
        FIO_adjustParamsForPatchFromMode(
            prefs,
            &mut comprParams,
            dictSize as core::ffi::c_ulonglong,
            if ssSize > 0 { ssSize } else { maxSrcFileSize },
            cLevel,
        );
    }
    dictBufferType = (if useMMap != 0 && forceNoUseMMap == 0 {
        FIO_mmapDict as core::ffi::c_int
    } else {
        FIO_mallocDict as core::ffi::c_int
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
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1155,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                32,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"allocation error : can't create dictBuffer\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(32);
    }
    ress.dictFileName = dictFileName;
    if (*prefs).adaptiveMode != 0 && (*prefs).ldmFlag == 0 && comprParams.windowLog == 0 {
        comprParams.windowLog = ADAPT_WINDOWLOG_DEFAULT as core::ffi::c_uint;
    }
    let mut err: size_t = 0;
    err = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_contentSizeFlag,
        (*prefs).contentSize,
    );
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_contentSizeFlag, prefs->contentSize)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1161,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_0: size_t = 0;
    err_0 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_dictIDFlag,
        (*prefs).dictIDFlag,
    );
    if ZSTD_isError(err_0) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_dictIDFlag, prefs->dictIDFlag)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1162,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_0),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_1: size_t = 0;
    err_1 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_checksumFlag,
        (*prefs).checksumFlag,
    );
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_checksumFlag, prefs->checksumFlag)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1163,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_2: size_t = 0;
    err_2 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_compressionLevel, cLevel);
    if ZSTD_isError(err_2) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_compressionLevel, cLevel)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1165,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_2),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_3: size_t = 0;
    err_3 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_targetCBlockSize,
        (*prefs).targetCBlockSize as core::ffi::c_int,
    );
    if ZSTD_isError(err_3) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_targetCBlockSize, (int)prefs->targetCBlockSize)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1167,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_3),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_4: size_t = 0;
    err_4 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_experimentalParam7,
        (*prefs).srcSizeHint,
    );
    if ZSTD_isError(err_4) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_experimentalParam7, (int)prefs->srcSizeHint)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1169,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_4),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_5: size_t = 0;
    err_5 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching,
        (*prefs).ldmFlag,
    );
    if ZSTD_isError(err_5) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching, prefs->ldmFlag)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1171,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_5),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_6: size_t = 0;
    err_6 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_ldmHashLog,
        (*prefs).ldmHashLog,
    );
    if ZSTD_isError(err_6) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_ldmHashLog, prefs->ldmHashLog)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1172,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_6),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_7: size_t = 0;
    err_7 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_ldmMinMatch,
        (*prefs).ldmMinMatch,
    );
    if ZSTD_isError(err_7) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_ldmMinMatch, prefs->ldmMinMatch)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1173,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_7),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    if (*prefs).ldmBucketSizeLog != FIO_LDM_PARAM_NOTSET {
        let mut err_8: size_t = 0;
        err_8 = ZSTD_CCtx_setParameter(
            ress.cctx,
            ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog,
            (*prefs).ldmBucketSizeLog,
        );
        if ZSTD_isError(err_8) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog, prefs->ldmBucketSizeLog)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1175,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_8),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    }
    if (*prefs).ldmHashRateLog != FIO_LDM_PARAM_NOTSET {
        let mut err_9: size_t = 0;
        err_9 = ZSTD_CCtx_setParameter(
            ress.cctx,
            ZSTD_cParameter::ZSTD_c_ldmHashRateLog,
            (*prefs).ldmHashRateLog,
        );
        if ZSTD_isError(err_9) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_ldmHashRateLog, prefs->ldmHashRateLog)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1178,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_9),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    }
    let mut err_10: size_t = 0;
    err_10 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_experimentalParam14,
        (*prefs).useRowMatchFinder,
    );
    if ZSTD_isError(err_10) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_experimentalParam14, prefs->useRowMatchFinder)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1180,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_10),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_11: size_t = 0;
    err_11 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_windowLog,
        comprParams.windowLog as core::ffi::c_int,
    );
    if ZSTD_isError(err_11) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_windowLog, (int)comprParams.windowLog)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1182,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_11),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_12: size_t = 0;
    err_12 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_chainLog,
        comprParams.chainLog as core::ffi::c_int,
    );
    if ZSTD_isError(err_12) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_chainLog, (int)comprParams.chainLog)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1183,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_12),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_13: size_t = 0;
    err_13 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_hashLog,
        comprParams.hashLog as core::ffi::c_int,
    );
    if ZSTD_isError(err_13) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_hashLog, (int)comprParams.hashLog)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1184,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_13),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_14: size_t = 0;
    err_14 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_searchLog,
        comprParams.searchLog as core::ffi::c_int,
    );
    if ZSTD_isError(err_14) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_searchLog, (int)comprParams.searchLog)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1185,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_14),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_15: size_t = 0;
    err_15 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_minMatch,
        comprParams.minMatch as core::ffi::c_int,
    );
    if ZSTD_isError(err_15) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_minMatch, (int)comprParams.minMatch)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1186,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_15),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_16: size_t = 0;
    err_16 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_targetLength,
        comprParams.targetLength as core::ffi::c_int,
    );
    if ZSTD_isError(err_16) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_targetLength, (int)comprParams.targetLength)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1187,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_16),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_17: size_t = 0;
    err_17 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_strategy,
        comprParams.strategy as core::ffi::c_int,
    );
    if ZSTD_isError(err_17) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_strategy, (int)comprParams.strategy)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1188,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_17),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_18: size_t = 0;
    err_18 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_experimentalParam5,
        (*prefs).literalCompressionMode.to_i32(),
    );
    if ZSTD_isError(err_18) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_experimentalParam5, (int)prefs->literalCompressionMode)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1189,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_18),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_19: size_t = 0;
    err_19 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_experimentalParam8, 1);
    if ZSTD_isError(err_19) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_experimentalParam8, 1)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1190,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_19),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    if g_display_prefs.displayLevel >= 5 {
        fprintf(
            stderr,
            b"set nb workers = %u \n\0" as *const u8 as *const core::ffi::c_char,
            (*prefs).nbWorkers,
        );
    }
    let mut err_20: size_t = 0;
    err_20 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_nbWorkers,
        (*prefs).nbWorkers,
    );
    if ZSTD_isError(err_20) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_nbWorkers, prefs->nbWorkers)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1194,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_20),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_21: size_t = 0;
    err_21 = ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_jobSize, (*prefs).jobSize);
    if ZSTD_isError(err_21) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_jobSize, prefs->jobSize)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1195,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_21),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    if (*prefs).overlapLog != FIO_OVERLAP_LOG_NOTSET {
        if g_display_prefs.displayLevel >= 3 {
            fprintf(
                stderr,
                b"set overlapLog = %u \n\0" as *const u8 as *const core::ffi::c_char,
                (*prefs).overlapLog,
            );
        }
        let mut err_22: size_t = 0;
        err_22 = ZSTD_CCtx_setParameter(
            ress.cctx,
            ZSTD_cParameter::ZSTD_c_overlapLog,
            (*prefs).overlapLog,
        );
        if ZSTD_isError(err_22) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_overlapLog, prefs->overlapLog)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1198,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_22),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    }
    let mut err_23: size_t = 0;
    err_23 = ZSTD_CCtx_setParameter(
        ress.cctx,
        ZSTD_cParameter::ZSTD_c_experimentalParam1,
        (*prefs).rsyncable,
    );
    if ZSTD_isError(err_23) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_experimentalParam1, prefs->rsyncable)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1200,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_23),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    if (*prefs).patchFromMode != 0 {
        let mut err_24: size_t = 0;
        err_24 = ZSTD_CCtx_refPrefix(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize);
        if ZSTD_isError(err_24) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_CCtx_refPrefix(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1204,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_24),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    } else {
        let mut err_25: size_t = 0;
        err_25 = ZSTD_CCtx_loadDictionary_byReference(
            ress.cctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_25) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_CCtx_loadDictionary_byReference(ress.cctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1206,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_25),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    }
    ress
}
unsafe fn FIO_freeCResources(ress: *mut cRess_t) {
    FIO_freeDict(&mut (*ress).dict);
    AIO_WritePool_free((*ress).writeCtx);
    AIO_ReadPool_free((*ress).readCtx);
    ZSTD_freeCStream((*ress).cctx);
}
unsafe fn FIO_compressGzFrame(
    ress: *const cRess_t,
    srcFileName: *const core::ffi::c_char,
    srcFileSize: u64,
    mut compressionLevel: core::ffi::c_int,
    readsize: *mut u64,
) -> core::ffi::c_ulonglong {
    let mut inFileSize = 0 as core::ffi::c_ulonglong;
    let mut outFileSize = 0 as core::ffi::c_ulonglong;
    let mut strm = z_stream_s {
        next_in: core::ptr::null_mut::<Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: core::ptr::null_mut::<Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: core::ptr::null_mut::<core::ffi::c_char>(),
        state: core::ptr::null_mut::<internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: core::ptr::null_mut::<core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut writeJob = core::ptr::null_mut();
    if compressionLevel > Z_BEST_COMPRESSION {
        compressionLevel = Z_BEST_COMPRESSION;
    }
    strm.zalloc = None;
    strm.zfree = None;
    strm.opaque = core::ptr::null_mut();
    let ret = deflateInit2_(
        &mut strm,
        compressionLevel,
        8,
        15 + 16,
        8,
        0,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as core::ffi::c_int,
    );
    if ret != Z_OK {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1242,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                71,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s: deflateInit2 error %d \n\0" as *const u8 as *const core::ffi::c_char,
                srcFileName,
                ret,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(71);
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_in = core::ptr::null_mut::<Bytef>();
    strm.avail_in = 0;
    strm.next_out = (*writeJob).buffer as *mut Bytef;
    strm.avail_out = (*writeJob).bufferSize as uInt;
    loop {
        let mut ret_0: core::ffi::c_int = 0;
        if strm.avail_in == 0 {
            AIO_ReadPool_fillBuffer((*ress).readCtx, ZSTD_CStreamInSize());
            if (*(*ress).readCtx).srcBufferLoaded == 0 {
                break;
            }
            inFileSize = inFileSize
                .wrapping_add((*(*ress).readCtx).srcBufferLoaded as core::ffi::c_ulonglong);
            strm.next_in = (*(*ress).readCtx).srcBuffer as *mut core::ffi::c_uchar;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded as uInt;
        }
        let availBefore = strm.avail_in as size_t;
        ret_0 = deflate(&mut strm, Z_NO_FLUSH);
        AIO_ReadPool_consumeBytes(
            (*ress).readCtx,
            availBefore.wrapping_sub(strm.avail_in as size_t),
        );
        if ret_0 != Z_OK {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1268,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    72,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: deflate error %d \n\0" as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                    ret_0,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(72);
        }
        let cSize = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out as size_t);
        if cSize != 0 {
            (*writeJob).usedBufferSize = cSize;
            AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
            outFileSize = outFileSize.wrapping_add(cSize as core::ffi::c_ulonglong);
            strm.next_out = (*writeJob).buffer as *mut Bytef;
            strm.avail_out = (*writeJob).bufferSize as uInt;
        }
        if srcFileSize == UTIL_FILESIZE_UNKNOWN as u64 {
            if g_display_prefs.progressSetting as core::ffi::c_uint
                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2
                    || g_display_prefs.progressSetting as core::ffi::c_uint
                        == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1
                && g_display_prefs.progressSetting as core::ffi::c_uint
                    != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"\rRead : %u MB ==> %.2f%% \0" as *const u8 as *const core::ffi::c_char,
                    (inFileSize >> 20) as core::ffi::c_uint,
                    outFileSize as core::ffi::c_double / inFileSize as core::ffi::c_double * 100.0,
                );
                if g_display_prefs.displayLevel >= 4 {
                    fflush(stderr);
                }
            }
        } else if g_display_prefs.progressSetting as core::ffi::c_uint
            != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2
                || g_display_prefs.progressSetting as core::ffi::c_uint
                    == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1
            && g_display_prefs.progressSetting as core::ffi::c_uint
                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
            && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                || g_display_prefs.displayLevel >= 4)
        {
            g_displayClock = UTIL_getTime();
            fprintf(
                stderr,
                b"\rRead : %u / %u MB ==> %.2f%% \0" as *const u8 as *const core::ffi::c_char,
                (inFileSize >> 20) as core::ffi::c_uint,
                (srcFileSize >> 20) as core::ffi::c_uint,
                outFileSize as core::ffi::c_double / inFileSize as core::ffi::c_double * 100.0,
            );
            if g_display_prefs.displayLevel >= 4 {
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
            outFileSize = outFileSize.wrapping_add(cSize_0 as core::ffi::c_ulonglong);
            strm.next_out = (*writeJob).buffer as *mut Bytef;
            strm.avail_out = (*writeJob).bufferSize as uInt;
        }
        if ret_1 == Z_STREAM_END {
            break;
        }
        if ret_1 != Z_BUF_ERROR {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1301,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    77,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: deflate error %d \n\0" as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                    ret_1,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(77);
        }
    }
    let ret_2 = deflateEnd(&mut strm);
    if ret_2 != Z_OK {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1306,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                79,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s: deflateEnd error %d \n\0" as *const u8 as *const core::ffi::c_char,
                srcFileName,
                ret_2,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(79);
    }
    *readsize = inFileSize as u64;
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    outFileSize
}
unsafe fn FIO_compressLzmaFrame(
    ress: *mut cRess_t,
    srcFileName: *const core::ffi::c_char,
    srcFileSize: u64,
    mut compressionLevel: core::ffi::c_int,
    readsize: *mut u64,
    plain_lzma: core::ffi::c_int,
) -> core::ffi::c_ulonglong {
    let mut inFileSize = 0 as core::ffi::c_ulonglong;
    let mut outFileSize = 0 as core::ffi::c_ulonglong;
    let mut strm = {
        lzma_stream {
            next_in: core::ptr::null(),
            avail_in: 0,
            total_in: 0,
            next_out: core::ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            allocator: core::ptr::null(),
            internal: core::ptr::null_mut(),
            reserved_ptr1: core::ptr::null_mut(),
            reserved_ptr2: core::ptr::null_mut(),
            reserved_ptr3: core::ptr::null_mut(),
            reserved_ptr4: core::ptr::null_mut(),
            seek_pos: 0,
            reserved_int2: 0,
            reserved_int3: 0,
            reserved_int4: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
        }
    };
    let mut action = LZMA_RUN;
    let mut ret = LZMA_OK;
    let mut writeJob = core::ptr::null_mut();
    if compressionLevel < 0 {
        compressionLevel = 0;
    }
    if compressionLevel > 9 {
        compressionLevel = 9;
    }
    if plain_lzma != 0 {
        let mut opt_lzma = lzma_options_lzma {
            dict_size: 0,
            preset_dict: core::ptr::null::<u8>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0,
            nice_len: 0,
            mf: 0,
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
            reserved_ptr1: core::ptr::null_mut::<core::ffi::c_void>(),
            reserved_ptr2: core::ptr::null_mut::<core::ffi::c_void>(),
        };
        if lzma_lzma_preset(&mut opt_lzma, compressionLevel as u32) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1334,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    81,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_lzma_preset error\0" as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(81);
        }
        ret = lzma_alone_encoder(&mut strm, &opt_lzma);
        if ret as core::ffi::c_uint != LZMA_OK as core::ffi::c_int as core::ffi::c_uint {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1337,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    82,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_alone_encoder error %d\0" as *const u8
                        as *const core::ffi::c_char,
                    srcFileName,
                    ret as core::ffi::c_uint,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(82);
        }
    } else {
        ret = lzma_easy_encoder(&mut strm, compressionLevel as u32, LZMA_CHECK_CRC64);
        if ret as core::ffi::c_uint != LZMA_OK as core::ffi::c_int as core::ffi::c_uint {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1341,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    83,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_easy_encoder error %d\0" as *const u8
                        as *const core::ffi::c_char,
                    srcFileName,
                    ret as core::ffi::c_uint,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(83);
        }
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_out = (*writeJob).buffer as *mut u8;
    strm.avail_out = (*writeJob).bufferSize;
    strm.next_in = core::ptr::null::<u8>();
    strm.avail_in = 0;
    loop {
        if strm.avail_in == 0 {
            let inSize = AIO_ReadPool_fillBuffer((*ress).readCtx, ZSTD_CStreamInSize());
            if (*(*ress).readCtx).srcBufferLoaded == 0 {
                action = LZMA_FINISH;
            }
            inFileSize = inFileSize.wrapping_add(inSize as core::ffi::c_ulonglong);
            strm.next_in = (*(*ress).readCtx).srcBuffer as *const u8;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded;
        }
        let availBefore = strm.avail_in;
        ret = lzma_code(&mut strm, action);
        AIO_ReadPool_consumeBytes((*ress).readCtx, availBefore.wrapping_sub(strm.avail_in));
        if ret as core::ffi::c_uint != LZMA_OK as core::ffi::c_int as core::ffi::c_uint
            && ret as core::ffi::c_uint != LZMA_STREAM_END as core::ffi::c_int as core::ffi::c_uint
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1367,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    84,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_code encoding error %d\0" as *const u8
                        as *const core::ffi::c_char,
                    srcFileName,
                    ret as core::ffi::c_uint,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(84);
        }
        let compBytes = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out);
        if compBytes != 0 {
            (*writeJob).usedBufferSize = compBytes;
            AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
            outFileSize = outFileSize.wrapping_add(compBytes as core::ffi::c_ulonglong);
            strm.next_out = (*writeJob).buffer as *mut u8;
            strm.avail_out = (*writeJob).bufferSize;
        }
        if srcFileSize == UTIL_FILESIZE_UNKNOWN as u64 {
            if g_display_prefs.progressSetting as core::ffi::c_uint
                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2
                    || g_display_prefs.progressSetting as core::ffi::c_uint
                        == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1
                && g_display_prefs.progressSetting as core::ffi::c_uint
                    != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"\rRead : %u MB ==> %.2f%%\0" as *const u8 as *const core::ffi::c_char,
                    (inFileSize >> 20) as core::ffi::c_uint,
                    outFileSize as core::ffi::c_double / inFileSize as core::ffi::c_double * 100.0,
                );
                if g_display_prefs.displayLevel >= 4 {
                    fflush(stderr);
                }
            }
        } else if g_display_prefs.progressSetting as core::ffi::c_uint
            != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2
                || g_display_prefs.progressSetting as core::ffi::c_uint
                    == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1
            && g_display_prefs.progressSetting as core::ffi::c_uint
                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
            && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                || g_display_prefs.displayLevel >= 4)
        {
            g_displayClock = UTIL_getTime();
            fprintf(
                stderr,
                b"\rRead : %u / %u MB ==> %.2f%%\0" as *const u8 as *const core::ffi::c_char,
                (inFileSize >> 20) as core::ffi::c_uint,
                (srcFileSize >> 20) as core::ffi::c_uint,
                outFileSize as core::ffi::c_double / inFileSize as core::ffi::c_double * 100.0,
            );
            if g_display_prefs.displayLevel >= 4 {
                fflush(stderr);
            }
        }
        if ret as core::ffi::c_uint == LZMA_STREAM_END as core::ffi::c_int as core::ffi::c_uint {
            break;
        }
    }
    lzma_end(&mut strm);
    *readsize = inFileSize as u64;
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    outFileSize
}
unsafe fn FIO_compressZstdFrame(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    ressPtr: *const cRess_t,
    srcFileName: *const core::ffi::c_char,
    fileSize: u64,
    mut compressionLevel: core::ffi::c_int,
    readsize: *mut u64,
) -> core::ffi::c_ulonglong {
    let ress = *ressPtr;
    let mut writeJob = AIO_WritePool_acquireJob((*ressPtr).writeCtx);
    let mut compressedfilesize = 0u64;
    let mut directive = ZSTD_e_continue;
    let mut pledgedSrcSize = ZSTD_CONTENTSIZE_UNKNOWN;
    let mut previous_zfp_update = {
        ZSTD_frameProgression {
            ingested: 0,
            consumed: 0,
            produced: 0,
            flushed: 0,
            currentJobID: 0,
            nbActiveWorkers: 0,
        }
    };
    let mut previous_zfp_correction = {
        ZSTD_frameProgression {
            ingested: 0,
            consumed: 0,
            produced: 0,
            flushed: 0,
            currentJobID: 0,
            nbActiveWorkers: 0,
        }
    };
    let mut speedChange = noChange;
    let mut flushWaiting = 0;
    let mut inputPresented = 0 as core::ffi::c_uint;
    let mut inputBlocked = 0 as core::ffi::c_uint;
    let mut lastJobID = 0;
    let mut lastAdaptTime = UTIL_getTime();
    let adaptEveryMicro = REFRESH_RATE;
    let file_hrs = UTIL_makeHumanReadableSize(fileSize);
    if g_display_prefs.displayLevel >= 6 {
        fprintf(
            stderr,
            b"compression using zstd format \n\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as u64 {
        pledgedSrcSize = fileSize;
        let mut err: size_t = 0;
        err = ZSTD_CCtx_setPledgedSrcSize(ress.cctx, fileSize as core::ffi::c_ulonglong);
        if ZSTD_isError(err) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, fileSize)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1532,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    } else if (*prefs).streamSrcSize > 0 {
        pledgedSrcSize = (*prefs).streamSrcSize as u64;
        let mut err_0: size_t = 0;
        err_0 = ZSTD_CCtx_setPledgedSrcSize(
            ress.cctx,
            (*prefs).streamSrcSize as core::ffi::c_ulonglong,
        );
        if ZSTD_isError(err_0) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_CCtx_setPledgedSrcSize(ress.cctx, prefs->streamSrcSize)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1536,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_0),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    }
    let mut windowLog: core::ffi::c_int = 0;
    let mut windowSize = UTIL_HumanReadableSize_t {
        value: 0.,
        precision: 0,
        suffix: core::ptr::null::<core::ffi::c_char>(),
    };
    let mut err_1: size_t = 0;
    err_1 = ZSTD_CCtx_getParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_windowLog, &mut windowLog);
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_getParameter(ress.cctx, ZSTD_cParameter::ZSTD_c_windowLog, &windowLog)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1541,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    if windowLog == 0 {
        if (*prefs).ldmFlag != 0 {
            windowLog = ZSTD_WINDOWLOG_LIMIT_DEFAULT;
        } else {
            let cParams = ZSTD_getCParams(compressionLevel, fileSize as core::ffi::c_ulonglong, 0);
            windowLog = cParams.windowLog as core::ffi::c_int;
        }
    }
    windowSize = UTIL_makeHumanReadableSize(
        (if 1
            > (if (1) << windowLog < pledgedSrcSize as core::ffi::c_ulonglong {
                (1) << windowLog
            } else {
                pledgedSrcSize as core::ffi::c_ulonglong
            })
        {
            1
        } else if (1) << windowLog < pledgedSrcSize as core::ffi::c_ulonglong {
            (1) << windowLog
        } else {
            pledgedSrcSize as core::ffi::c_ulonglong
        }) as u64,
    );
    if g_display_prefs.displayLevel >= 4 {
        fprintf(
            stderr,
            b"Decompression will require %.*f%s of memory\n\0" as *const u8
                as *const core::ffi::c_char,
            windowSize.precision,
            windowSize.value,
            windowSize.suffix,
        );
    }
    loop {
        let mut stillToFlush: size_t = 0;
        let inSize = AIO_ReadPool_fillBuffer(ress.readCtx, ZSTD_CStreamInSize());
        let mut inBuff = setInBuffer(
            (*ress.readCtx).srcBuffer as *const core::ffi::c_void,
            (*ress.readCtx).srcBufferLoaded,
            0,
        );
        if g_display_prefs.displayLevel >= 6 {
            fprintf(
                stderr,
                b"fread %u bytes from source \n\0" as *const u8 as *const core::ffi::c_char,
                inSize as core::ffi::c_uint,
            );
        }
        *readsize = (*readsize).wrapping_add(inSize as u64);
        if (*ress.readCtx).srcBufferLoaded == 0 || *readsize == fileSize {
            directive = ZSTD_e_end;
        }
        stillToFlush = 1;
        while inBuff.pos != inBuff.size
            || directive as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
                && stillToFlush != 0
        {
            let oldIPos = inBuff.pos;
            let mut outBuff = setOutBuffer((*writeJob).buffer, (*writeJob).bufferSize, 0);
            let toFlushNow = ZSTD_toFlushNow(ress.cctx);
            stillToFlush = ZSTD_compressStream2(ress.cctx, &mut outBuff, &mut inBuff, directive);
            if ZSTD_isError(stillToFlush) != 0 {
                if g_display_prefs.displayLevel >= 5 {
                    fprintf(
                        stderr,
                        b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                        b"ZSTD_compressStream2(ress.cctx, &outBuff, &inBuff, directive)\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const core::ffi::c_char,
                        b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                        1574,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                        11,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"%s\0" as *const u8 as *const core::ffi::c_char,
                        ZSTD_getErrorName(stillToFlush),
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                }
                exit(11);
            }
            AIO_ReadPool_consumeBytes(ress.readCtx, (inBuff.pos).wrapping_sub(oldIPos));
            inputPresented = inputPresented.wrapping_add(1);
            if oldIPos == inBuff.pos {
                inputBlocked = inputBlocked.wrapping_add(1);
            }
            if toFlushNow == 0 {
                flushWaiting = 1;
            }
            if g_display_prefs.displayLevel >= 6 {
                fprintf(
                    stderr,
                    b"ZSTD_compress_generic(end:%u) => input pos(%u)<=(%u)size ; output generated %u bytes \n\0"
                        as *const u8 as *const core::ffi::c_char,
                    directive as core::ffi::c_uint,
                    inBuff.pos as core::ffi::c_uint,
                    inBuff.size as core::ffi::c_uint,
                    outBuff.pos as core::ffi::c_uint,
                );
            }
            if outBuff.pos != 0 {
                (*writeJob).usedBufferSize = outBuff.pos;
                AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
                compressedfilesize = compressedfilesize.wrapping_add(outBuff.pos as u64);
            }
            if (*prefs).adaptiveMode != 0 && UTIL_clockSpanMicro(lastAdaptTime) > adaptEveryMicro {
                let zfp = ZSTD_getFrameProgression(ress.cctx);
                lastAdaptTime = UTIL_getTime();
                if zfp.currentJobID > 1 {
                    let newlyProduced = (zfp.produced).wrapping_sub(previous_zfp_update.produced);
                    let newlyFlushed = (zfp.flushed).wrapping_sub(previous_zfp_update.flushed);
                    assert!(zfp.produced >= previous_zfp_update.produced);
                    assert!((*prefs).nbWorkers >= 1);
                    if zfp.consumed == previous_zfp_update.consumed && zfp.nbActiveWorkers == 0 {
                        if g_display_prefs.displayLevel >= 6 {
                            fprintf(
                                stderr,
                                b"all buffers full : compression stopped => slow down \n\0"
                                    as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        speedChange = slower;
                    }
                    previous_zfp_update = zfp;
                    if newlyProduced > newlyFlushed.wrapping_mul(9).wrapping_div(8)
                        && flushWaiting == 0
                    {
                        if g_display_prefs.displayLevel >= 6 {
                            fprintf(
                                stderr,
                                b"compression faster than flush (%llu > %llu), and flushed was never slowed down by lack of production => slow down \n\0"
                                    as *const u8 as *const core::ffi::c_char,
                                newlyProduced,
                                newlyFlushed,
                            );
                        }
                        speedChange = slower;
                    }
                    flushWaiting = 0;
                }
                if zfp.currentJobID > lastJobID {
                    if g_display_prefs.displayLevel >= 6 {
                        fprintf(
                            stderr,
                            b"compression level adaptation check \n\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if zfp.currentJobID > ((*prefs).nbWorkers + 1) as core::ffi::c_uint {
                        if inputBlocked <= 0 {
                            if g_display_prefs.displayLevel >= 6 {
                                fprintf(
                                    stderr,
                                    b"input is never blocked => input is slower than ingestion \n\0"
                                        as *const u8
                                        as *const core::ffi::c_char,
                                );
                            }
                            speedChange = slower;
                        } else if speedChange as core::ffi::c_uint
                            == noChange as core::ffi::c_int as core::ffi::c_uint
                        {
                            let newlyIngested =
                                (zfp.ingested).wrapping_sub(previous_zfp_correction.ingested);
                            let newlyConsumed =
                                (zfp.consumed).wrapping_sub(previous_zfp_correction.consumed);
                            let newlyProduced_0 =
                                (zfp.produced).wrapping_sub(previous_zfp_correction.produced);
                            let newlyFlushed_0 =
                                (zfp.flushed).wrapping_sub(previous_zfp_correction.flushed);
                            previous_zfp_correction = zfp;
                            assert!(inputPresented > 0);
                            if g_display_prefs.displayLevel >= 6 {
                                fprintf(
                                    stderr,
                                    b"input blocked %u/%u(%.2f) - ingested:%u vs %u:consumed - flushed:%u vs %u:produced \n\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    inputBlocked,
                                    inputPresented,
                                    core::ffi::c_double::from(inputBlocked)
                                        / core::ffi::c_double::from(inputPresented)
                                        * 100.0,
                                    newlyIngested as core::ffi::c_uint,
                                    newlyConsumed as core::ffi::c_uint,
                                    newlyFlushed_0 as core::ffi::c_uint,
                                    newlyProduced_0 as core::ffi::c_uint,
                                );
                            }
                            if inputBlocked > inputPresented.wrapping_div(8)
                                && newlyFlushed_0.wrapping_mul(33).wrapping_div(32)
                                    > newlyProduced_0
                                && newlyIngested.wrapping_mul(33).wrapping_div(32) > newlyConsumed
                            {
                                if g_display_prefs.displayLevel >= 6 {
                                    fprintf(
                                        stderr,
                                        b"recommend faster as in(%llu) >= (%llu)comp(%llu) <= out(%llu) \n\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        newlyIngested,
                                        newlyConsumed,
                                        newlyProduced_0,
                                        newlyFlushed_0,
                                    );
                                }
                                speedChange = faster;
                            }
                        }
                        inputBlocked = 0;
                        inputPresented = 0;
                    }
                    if speedChange as core::ffi::c_uint
                        == slower as core::ffi::c_int as core::ffi::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 {
                            fprintf(
                                stderr,
                                b"slower speed , higher compression \n\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        compressionLevel += 1;
                        if compressionLevel > ZSTD_maxCLevel() {
                            compressionLevel = ZSTD_maxCLevel();
                        }
                        if compressionLevel > (*prefs).maxAdaptLevel {
                            compressionLevel = (*prefs).maxAdaptLevel;
                        }
                        compressionLevel += core::ffi::c_int::from(compressionLevel == 0);
                        ZSTD_CCtx_setParameter(
                            ress.cctx,
                            ZSTD_cParameter::ZSTD_c_compressionLevel,
                            compressionLevel,
                        );
                    }
                    if speedChange as core::ffi::c_uint
                        == faster as core::ffi::c_int as core::ffi::c_uint
                    {
                        if g_display_prefs.displayLevel >= 6 {
                            fprintf(
                                stderr,
                                b"faster speed , lighter compression \n\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        compressionLevel -= 1;
                        if compressionLevel < (*prefs).minAdaptLevel {
                            compressionLevel = (*prefs).minAdaptLevel;
                        }
                        compressionLevel -= core::ffi::c_int::from(compressionLevel == 0);
                        ZSTD_CCtx_setParameter(
                            ress.cctx,
                            ZSTD_cParameter::ZSTD_c_compressionLevel,
                            compressionLevel,
                        );
                    }
                    speedChange = noChange;
                    lastJobID = zfp.currentJobID;
                }
            }
            if g_display_prefs.progressSetting as core::ffi::c_uint
                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2
                    || g_display_prefs.progressSetting as core::ffi::c_uint
                        == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4)
            {
                let zfp_0 = ZSTD_getFrameProgression(ress.cctx);
                let cShare = zfp_0.produced as core::ffi::c_double
                    / (zfp_0.consumed).wrapping_add(
                        core::ffi::c_int::from(zfp_0.consumed == 0) as core::ffi::c_ulonglong
                    ) as core::ffi::c_double
                    * 100.0;
                let buffered_hrs = UTIL_makeHumanReadableSize(
                    (zfp_0.ingested).wrapping_sub(zfp_0.consumed) as u64,
                );
                let consumed_hrs = UTIL_makeHumanReadableSize(zfp_0.consumed as u64);
                let produced_hrs = UTIL_makeHumanReadableSize(zfp_0.produced as u64);
                g_displayClock = UTIL_getTime();
                if g_display_prefs.progressSetting as core::ffi::c_uint
                    != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                    && (g_display_prefs.displayLevel >= 2
                        || g_display_prefs.progressSetting as core::ffi::c_uint
                            == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                    && g_display_prefs.displayLevel >= 1
                {
                    fprintf(
                        stderr,
                        b"\r%79s\r\0" as *const u8 as *const core::ffi::c_char,
                        b"\0" as *const u8 as *const core::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 3 {
                    if g_display_prefs.progressSetting as core::ffi::c_uint
                        != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2
                            || g_display_prefs.progressSetting as core::ffi::c_uint
                                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1
                    {
                        fprintf(
                                stderr,
                                b"(L%i) Buffered:%5.*f%s - Consumed:%5.*f%s - Compressed:%5.*f%s => %.2f%% \0"
                                    as *const u8 as *const core::ffi::c_char,
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
                    if (*fCtx).nbFilesTotal > 1 {
                        let srcFileNameSize = strlen(srcFileName);
                        if srcFileNameSize > 18 {
                            let truncatedSrcFileName =
                                srcFileName.add(srcFileNameSize).offset(-(15));
                            if g_display_prefs.progressSetting as core::ffi::c_uint
                                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                                && (g_display_prefs.displayLevel >= 2
                                    || g_display_prefs.progressSetting as core::ffi::c_uint
                                        == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                                && g_display_prefs.displayLevel >= 1
                            {
                                fprintf(
                                    stderr,
                                    b"Compress: %u/%u files. Current: ...%s \0" as *const u8
                                        as *const core::ffi::c_char,
                                    (*fCtx).currFileIdx + 1,
                                    (*fCtx).nbFilesTotal,
                                    truncatedSrcFileName,
                                );
                            }
                        } else if g_display_prefs.progressSetting as core::ffi::c_uint
                            != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                            && (g_display_prefs.displayLevel >= 2
                                || g_display_prefs.progressSetting as core::ffi::c_uint
                                    == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                            && g_display_prefs.displayLevel >= 1
                        {
                            fprintf(
                                stderr,
                                b"Compress: %u/%u files. Current: %*s \0" as *const u8
                                    as *const core::ffi::c_char,
                                (*fCtx).currFileIdx + 1,
                                (*fCtx).nbFilesTotal,
                                (18 as size_t).wrapping_sub(srcFileNameSize) as core::ffi::c_int,
                                srcFileName,
                            );
                        }
                    }
                    if g_display_prefs.progressSetting as core::ffi::c_uint
                        != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2
                            || g_display_prefs.progressSetting as core::ffi::c_uint
                                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1
                    {
                        fprintf(
                            stderr,
                            b"Read:%6.*f%4s \0" as *const u8 as *const core::ffi::c_char,
                            consumed_hrs.precision,
                            consumed_hrs.value,
                            consumed_hrs.suffix,
                        );
                    }
                    if fileSize != UTIL_FILESIZE_UNKNOWN as u64
                        && g_display_prefs.progressSetting as core::ffi::c_uint
                            != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2
                            || g_display_prefs.progressSetting as core::ffi::c_uint
                                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1
                    {
                        fprintf(
                            stderr,
                            b"/%6.*f%4s\0" as *const u8 as *const core::ffi::c_char,
                            file_hrs.precision,
                            file_hrs.value,
                            file_hrs.suffix,
                        );
                    }
                    if g_display_prefs.progressSetting as core::ffi::c_uint
                        != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                        && (g_display_prefs.displayLevel >= 2
                            || g_display_prefs.progressSetting as core::ffi::c_uint
                                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                        && g_display_prefs.displayLevel >= 1
                    {
                        fprintf(
                            stderr,
                            b" ==> %2.f%%\0" as *const u8 as *const core::ffi::c_char,
                            cShare,
                        );
                    }
                }
            }
        }
        if directive as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint {
            break;
        }
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as u64 && *readsize != fileSize {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                1727,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                27,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Read error : Incomplete read : %llu / %llu B\0" as *const u8
                    as *const core::ffi::c_char,
                *readsize as core::ffi::c_ulonglong,
                fileSize as core::ffi::c_ulonglong,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(27);
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ressPtr).writeCtx);
    compressedfilesize
}
unsafe fn FIO_compressFilename_internal(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    mut ress: cRess_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
    compressionLevel: core::ffi::c_int,
) -> core::ffi::c_int {
    let timeStart = UTIL_getTime();
    let cpuStart = Instant::now();
    let mut readsize = 0;
    let mut compressedfilesize = 0;
    let fileSize = UTIL_getFileSize(srcFileName);
    if g_display_prefs.displayLevel >= 5 {
        fprintf(
            stderr,
            b"%s: %llu bytes \n\0" as *const u8 as *const core::ffi::c_char,
            srcFileName,
            fileSize as core::ffi::c_ulonglong,
        );
    }
    match (*prefs).compressionType as core::ffi::c_uint {
        1 => {
            compressedfilesize = FIO_compressGzFrame(
                &ress,
                srcFileName,
                fileSize,
                compressionLevel,
                &mut readsize,
            ) as u64;
        }
        2 | 3 => {
            compressedfilesize = FIO_compressLzmaFrame(
                &mut ress,
                srcFileName,
                fileSize,
                compressionLevel,
                &mut readsize,
                core::ffi::c_int::from(
                    (*prefs).compressionType as core::ffi::c_uint
                        == FIO_lzmaCompression as core::ffi::c_int as core::ffi::c_uint,
                ),
            ) as u64;
        }
        4 => {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    1789,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    20,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: file cannot be compressed as lz4 (zstd compiled without ZSTD_LZ4COMPRESS) -- ignored \n\0"
                        as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(20);
        }
        0 | _ => {
            compressedfilesize = FIO_compressZstdFrame(
                fCtx,
                prefs,
                &ress,
                srcFileName,
                fileSize,
                compressionLevel,
                &mut readsize,
            ) as u64;
        }
    }
    (*fCtx).totalBytesInput = ((*fCtx).totalBytesInput).wrapping_add(readsize as size_t);
    (*fCtx).totalBytesOutput =
        ((*fCtx).totalBytesOutput).wrapping_add(compressedfilesize as size_t);
    if g_display_prefs.progressSetting as core::ffi::c_uint
        != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
        && (g_display_prefs.displayLevel >= 2
            || g_display_prefs.progressSetting as core::ffi::c_uint
                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
        && g_display_prefs.displayLevel >= 1
    {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if FIO_shouldDisplayFileSummary(fCtx) != 0 {
        let hr_isize = UTIL_makeHumanReadableSize(readsize);
        let hr_osize = UTIL_makeHumanReadableSize(compressedfilesize);
        if readsize == 0 {
            if (g_display_prefs.displayLevel >= 2
                || g_display_prefs.progressSetting as core::ffi::c_uint
                    == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1
            {
                fprintf(
                    stderr,
                    b"%-20s :  (%6.*f%s => %6.*f%s, %s) \n\0" as *const u8
                        as *const core::ffi::c_char,
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
        } else if (g_display_prefs.displayLevel >= 2
            || g_display_prefs.progressSetting as core::ffi::c_uint
                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1
        {
            fprintf(
                stderr,
                b"%-20s :%6.2f%%   (%6.*f%s => %6.*f%s, %s) \n\0" as *const u8
                    as *const core::ffi::c_char,
                srcFileName,
                compressedfilesize as core::ffi::c_double / readsize as core::ffi::c_double * 100.0,
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
    let cpuLoad_s = cpuStart.elapsed().as_secs_f64();
    let timeLength_ns = UTIL_clockSpanNano(timeStart);
    let timeLength_s = timeLength_ns as core::ffi::c_double / 1000000000.0;
    let cpuLoad_pct = cpuLoad_s / timeLength_s * 100.0;
    if g_display_prefs.displayLevel >= 4 {
        fprintf(
            stderr,
            b"%-20s : Completed in %.2f sec  (cpu load : %.0f%%)\n\0" as *const u8
                as *const core::ffi::c_char,
            srcFileName,
            timeLength_s,
            cpuLoad_pct,
        );
    }
    0
}
unsafe fn FIO_compressFilename_dstFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    ress: cRess_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
    srcFileStat: *const stat_t,
    compressionLevel: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut closeDstFile = 0;
    let mut result: core::ffi::c_int = 0;
    let mut transferStat = 0;
    let mut dstFd = -(1);
    assert!(!(AIO_ReadPool_getFile(ress.readCtx)).is_null());
    if (AIO_WritePool_getFile(ress.writeCtx)).is_null() {
        let mut dstFileInitialPermissions = DEFAULT_FILE_PERMISSIONS;
        if strcmp(srcFileName, stdinmark.as_ptr()) != 0
            && strcmp(dstFileName, stdoutmark.as_ptr()) != 0
            && UTIL_isRegularFileStat(srcFileStat) != 0
        {
            transferStat = 1;
            dstFileInitialPermissions = TEMPORARY_FILE_PERMISSIONS;
        }
        closeDstFile = 1;
        if g_display_prefs.displayLevel >= 6 {
            fprintf(
                stderr,
                b"FIO_compressFilename_dstFile: opening dst: %s \n\0" as *const u8
                    as *const core::ffi::c_char,
                dstFileName,
            );
        }
        let dstFile = FIO_openDstFile(
            fCtx,
            prefs,
            srcFileName,
            dstFileName,
            dstFileInitialPermissions,
        );
        if dstFile.is_null() {
            return 1;
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
        if g_display_prefs.displayLevel >= 6 {
            fprintf(
                stderr,
                b"FIO_compressFilename_dstFile: closing dst: %s \n\0" as *const u8
                    as *const core::ffi::c_char,
                dstFileName,
            );
        }
        if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                eprintln!(
                    "zstd: {}: {}",
                    CStr::from_ptr(dstFileName).to_string_lossy(),
                    io::Error::last_os_error(),
                );
            }
            result = 1;
        }
        if transferStat != 0 {
            UTIL_utime(dstFileName, srcFileStat);
        }
        if result != 0 && strcmp(dstFileName, stdoutmark.as_ptr()) != 0 {
            FIO_removeFile(dstFileName);
        }
    }
    result
}
static compressedFileExtensions: [&CStr; 113] = [
    ZSTD_EXTENSION,
    TZSTD_EXTENSION,
    GZ_EXTENSION,
    TGZ_EXTENSION,
    LZMA_EXTENSION,
    XZ_EXTENSION,
    TXZ_EXTENSION,
    LZ4_EXTENSION,
    TLZ4_EXTENSION,
    c".7z",
    c".aa3",
    c".aac",
    c".aar",
    c".ace",
    c".alac",
    c".ape",
    c".apk",
    c".apng",
    c".arc",
    c".archive",
    c".arj",
    c".ark",
    c".asf",
    c".avi",
    c".avif",
    c".ba",
    c".br",
    c".bz2",
    c".cab",
    c".cdx",
    c".chm",
    c".cr2",
    c".divx",
    c".dmg",
    c".dng",
    c".docm",
    c".docx",
    c".dotm",
    c".dotx",
    c".dsft",
    c".ear",
    c".eftx",
    c".emz",
    c".eot",
    c".epub",
    c".f4v",
    c".flac",
    c".flv",
    c".gho",
    c".gif",
    c".gifv",
    c".gnp",
    c".iso",
    c".jar",
    c".jpeg",
    c".jpg",
    c".jxl",
    c".lz",
    c".lzh",
    c".m4a",
    c".m4v",
    c".mkv",
    c".mov",
    c".mp2",
    c".mp3",
    c".mp4",
    c".mpa",
    c".mpc",
    c".mpe",
    c".mpeg",
    c".mpg",
    c".mpl",
    c".mpv",
    c".msi",
    c".odp",
    c".ods",
    c".odt",
    c".ogg",
    c".ogv",
    c".otp",
    c".ots",
    c".ott",
    c".pea",
    c".png",
    c".pptx",
    c".qt",
    c".rar",
    c".s7z",
    c".sfx",
    c".sit",
    c".sitx",
    c".sqx",
    c".svgz",
    c".swf",
    c".tbz2",
    c".tib",
    c".tlz",
    c".vob",
    c".war",
    c".webm",
    c".webp",
    c".wma",
    c".wmv",
    c".woff",
    c".woff2",
    c".wvl",
    c".xlsx",
    c".xpi",
    c".xps",
    c".zip",
    c".zipx",
    c".zoo",
    c".zpaq",
];
unsafe fn FIO_compressFilename_srcFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    ress: cRess_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
    compressionLevel: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut result: core::ffi::c_int = 0;
    let mut srcFile = core::ptr::null_mut::<FILE>();
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
    let mut fileSize = UTIL_FILESIZE_UNKNOWN as u64;
    if g_display_prefs.displayLevel >= 6 {
        fprintf(
            stderr,
            b"FIO_compressFilename_srcFile: %s \n\0" as *const u8 as *const core::ffi::c_char,
            srcFileName,
        );
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 && UTIL_stat(srcFileName, &mut srcFileStat) != 0
    {
        if UTIL_isDirectoryStat(&srcFileStat) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s is a directory -- ignored \n\0" as *const u8
                        as *const core::ffi::c_char,
                    srcFileName,
                );
            }
            return 1;
        }
        if !(ress.dictFileName).is_null()
            && UTIL_isSameFileStat(
                srcFileName,
                ress.dictFileName,
                &srcFileStat,
                &ress.dictFileStat,
            ) != 0
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: cannot use %s as an input file and dictionary \n\0" as *const u8
                        as *const core::ffi::c_char,
                    srcFileName,
                );
            }
            return 1;
        }
    }
    if (*prefs).excludeCompressedFiles == 1
        && UTIL_isCompressedFile(srcFileName, &compressedFileExtensions) != 0
    {
        if g_display_prefs.displayLevel >= 4 {
            fprintf(
                stderr,
                b"File is already compressed : %s \n\0" as *const u8 as *const core::ffi::c_char,
                srcFileName,
            );
        }
        return 0;
    }
    srcFile = FIO_openSrcFile(prefs, srcFileName, &mut srcFileStat);
    if srcFile.is_null() {
        return 1;
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        fileSize = UTIL_getFileSizeStat(&srcFileStat);
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as u64 && fileSize < (ZSTD_BLOCKSIZE_MAX * 3) as u64 {
        AIO_ReadPool_setAsync(ress.readCtx, 0);
        AIO_WritePool_setAsync(ress.writeCtx, 0);
    } else {
        AIO_ReadPool_setAsync(ress.readCtx, 1);
        AIO_WritePool_setAsync(ress.writeCtx, 1);
    }
    AIO_ReadPool_setFile(ress.readCtx, srcFile);
    result = FIO_compressFilename_dstFile(
        fCtx,
        prefs,
        ress,
        dstFileName,
        srcFileName,
        &srcFileStat,
        compressionLevel,
    );
    AIO_ReadPool_closeFile(ress.readCtx);
    if (*prefs).removeSrcFile != 0 && result == 0 && strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        clearHandler();
        if FIO_removeFile(srcFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    2100,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    1,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                eprintln!(
                    "zstd: {}: {}",
                    CStr::from_ptr(srcFileName).to_string_lossy(),
                    io::Error::last_os_error(),
                );
            }
            exit(1);
        }
    }
    result
}
pub unsafe fn FIO_displayCompressionParameters(prefs: *const FIO_prefs_t) {
    static formatOptions: [&CStr; 5] = [
        ZSTD_EXTENSION,
        GZ_EXTENSION,
        XZ_EXTENSION,
        LZMA_EXTENSION,
        LZ4_EXTENSION,
    ];
    static sparseOptions: [&CStr; 3] = [c" --no-sparse", c"", c" --sparse"];
    static checkSumOptions: [&CStr; 3] = [c" --no-check", c"", c" --check"];
    static rowMatchFinderOptions: [&CStr; 3] =
        [c"", c" --no-row-match-finder", c" --row-match-finder"];
    static compressLiteralsOptions: [&CStr; 3] =
        [c"", c" --compress-literals", c" --no-compress-literals"];
    assert!(g_display_prefs.displayLevel >= 4);
    fprintf(
        stderr,
        b"--format=%s\0" as *const u8 as *const core::ffi::c_char,
        formatOptions[(*prefs).compressionType as usize].as_ptr(),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        sparseOptions[(*prefs).sparseFileSupport as usize].as_ptr(),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        if (*prefs).dictIDFlag != 0 {
            b"\0" as *const u8 as *const core::ffi::c_char
        } else {
            b" --no-dictID\0" as *const u8 as *const core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        checkSumOptions[(*prefs).checksumFlag as usize].as_ptr(),
    );
    fprintf(
        stderr,
        b" --jobsize=%d\0" as *const u8 as *const core::ffi::c_char,
        (*prefs).jobSize,
    );
    if (*prefs).adaptiveMode != 0 {
        fprintf(
            stderr,
            b" --adapt=min=%d,max=%d\0" as *const u8 as *const core::ffi::c_char,
            (*prefs).minAdaptLevel,
            (*prefs).maxAdaptLevel,
        );
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        rowMatchFinderOptions[(*prefs).useRowMatchFinder as usize].as_ptr(),
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        if (*prefs).rsyncable != 0 {
            b" --rsyncable\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const core::ffi::c_char
        },
    );
    if (*prefs).streamSrcSize != 0 {
        fprintf(
            stderr,
            b" --stream-size=%u\0" as *const u8 as *const core::ffi::c_char,
            (*prefs).streamSrcSize as core::ffi::c_uint,
        );
    }
    if (*prefs).srcSizeHint != 0 {
        fprintf(
            stderr,
            b" --size-hint=%d\0" as *const u8 as *const core::ffi::c_char,
            (*prefs).srcSizeHint,
        );
    }
    if (*prefs).targetCBlockSize != 0 {
        fprintf(
            stderr,
            b" --target-compressed-block-size=%u\0" as *const u8 as *const core::ffi::c_char,
            (*prefs).targetCBlockSize as core::ffi::c_uint,
        );
    }
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        compressLiteralsOptions[(*prefs).literalCompressionMode.to_i32() as usize].as_ptr(),
    );
    fprintf(
        stderr,
        b" --memory=%u\0" as *const u8 as *const core::ffi::c_char,
        if (*prefs).memLimit != 0 {
            (*prefs).memLimit
        } else {
            (128 * ((1) << 20)) as core::ffi::c_uint
        },
    );
    fprintf(
        stderr,
        b" --threads=%d\0" as *const u8 as *const core::ffi::c_char,
        (*prefs).nbWorkers,
    );
    fprintf(
        stderr,
        b"%s\0" as *const u8 as *const core::ffi::c_char,
        if (*prefs).excludeCompressedFiles != 0 {
            b" --exclude-compressed\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const core::ffi::c_char
        },
    );
    fprintf(
        stderr,
        b" --%scontent-size\0" as *const u8 as *const core::ffi::c_char,
        if (*prefs).contentSize != 0 {
            b"\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"no-\0" as *const u8 as *const core::ffi::c_char
        },
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const core::ffi::c_char);
}
pub unsafe fn FIO_compressFilename(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
    dictFileName: *const core::ffi::c_char,
    compressionLevel: core::ffi::c_int,
    comprParams: ZSTD_compressionParameters,
) -> core::ffi::c_int {
    let mut ress = FIO_createCResources(
        prefs,
        dictFileName,
        UTIL_getFileSize(srcFileName) as core::ffi::c_ulonglong,
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
unsafe fn FIO_determineCompressedName(
    srcFileName: *const core::ffi::c_char,
    outDirName: *const core::ffi::c_char,
    suffix: *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    static mut dfnbCapacity: size_t = 0;
    static mut dstFileNameBuffer: *mut core::ffi::c_char = core::ptr::null_mut();
    let mut outDirFilename = core::ptr::null_mut();
    let mut sfnSize = strlen(srcFileName);
    let srcSuffixLen = strlen(suffix);
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        return stdoutmark.as_ptr();
    }
    if !outDirName.is_null() {
        outDirFilename = FIO_createFilename_fromOutDir(srcFileName, outDirName, srcSuffixLen);
        sfnSize = strlen(outDirFilename);
        assert!(!outDirFilename.is_null());
    }
    if dfnbCapacity <= sfnSize.wrapping_add(srcSuffixLen).wrapping_add(1) {
        free(dstFileNameBuffer as *mut core::ffi::c_void);
        dfnbCapacity = sfnSize.wrapping_add(srcSuffixLen).wrapping_add(30);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut core::ffi::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    2194,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    30,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                eprintln!("zstd: {}", io::Error::last_os_error());
            }
            exit(30);
        }
    }
    assert!(!dstFileNameBuffer.is_null());
    if !outDirFilename.is_null() {
        memcpy(
            dstFileNameBuffer as *mut core::ffi::c_void,
            outDirFilename as *const core::ffi::c_void,
            sfnSize,
        );
        free(outDirFilename as *mut core::ffi::c_void);
    } else {
        memcpy(
            dstFileNameBuffer as *mut core::ffi::c_void,
            srcFileName as *const core::ffi::c_void,
            sfnSize,
        );
    }
    memcpy(
        dstFileNameBuffer.add(sfnSize) as *mut core::ffi::c_void,
        suffix as *const core::ffi::c_void,
        srcSuffixLen.wrapping_add(1),
    );
    dstFileNameBuffer
}
unsafe fn FIO_getLargestFileSize(
    inFileNames: *mut *const core::ffi::c_char,
    nbFiles: core::ffi::c_uint,
) -> core::ffi::c_ulonglong {
    let mut i: size_t = 0;
    let mut fileSize: core::ffi::c_ulonglong = 0;
    let mut maxFileSize = 0;
    i = 0;
    while i < nbFiles as size_t {
        fileSize = UTIL_getFileSize(*inFileNames.add(i)) as core::ffi::c_ulonglong;
        maxFileSize = if fileSize > maxFileSize {
            fileSize
        } else {
            maxFileSize
        };
        i = i.wrapping_add(1);
    }
    maxFileSize
}

/// FIO_compressMultipleFilenames() :
/// compress nbFiles files
/// into either one destination (outFileName),
/// or into one file each (outFileName == NULL, but suffix != NULL),
/// or into a destination folder (specified with -O)
pub unsafe fn FIO_compressMultipleFilenames(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    inFileNamesTable: *mut *const core::ffi::c_char,
    outMirroredRootDirName: *const core::ffi::c_char,
    outDirName: *const core::ffi::c_char,
    outFileName: *const core::ffi::c_char,
    suffix: *const core::ffi::c_char,
    dictFileName: *const core::ffi::c_char,
    compressionLevel: core::ffi::c_int,
    comprParams: ZSTD_compressionParameters,
) -> core::ffi::c_int {
    let mut status: core::ffi::c_int = 0;
    let mut error = 0;
    let mut ress = FIO_createCResources(
        prefs,
        dictFileName,
        FIO_getLargestFileSize(inFileNamesTable, (*fCtx).nbFilesTotal as core::ffi::c_uint),
        compressionLevel,
        comprParams,
    );

    // init
    assert!(!outFileName.is_null() || !suffix.is_null());
    if !outFileName.is_null() {
        // output into a single destination (stdout typically)
        let mut dstFile = core::ptr::null_mut::<FILE>();
        if FIO_multiFilesConcatWarning(fCtx, prefs, outFileName, 1) != 0 {
            FIO_freeCResources(&mut ress);
            return 1;
        }
        dstFile = FIO_openDstFile(
            fCtx,
            prefs,
            core::ptr::null(),
            outFileName,
            DEFAULT_FILE_PERMISSIONS,
        );
        if dstFile.is_null() {
            // could not open outFileName
            error = 1;
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
                }
                error |= status;
                (*fCtx).currFileIdx += 1;
            }
            if AIO_WritePool_closeFile(ress.writeCtx) != 0 {
                if g_display_prefs.displayLevel >= 1 {
                    eprint!("zstd: ");
                }
                if g_display_prefs.displayLevel >= 5 {
                    eprintln!("Error defined at {}, line {} : ", file!(), line!());
                }
                if g_display_prefs.displayLevel >= 1 {
                    eprint!("error {} : ", 29);
                }
                if g_display_prefs.displayLevel >= 1 {
                    eprintln!(
                        "Write error ({}) : cannot properly close {}",
                        io::Error::last_os_error(),
                        CStr::from_ptr(outFileName).to_string_lossy(),
                    );
                }
                exit(29);
            }
        }
    } else {
        if !outMirroredRootDirName.is_null() {
            UTIL_mirrorSourceFilesDirectories(
                inFileNamesTable,
                (*fCtx).nbFilesTotal as core::ffi::c_uint,
                outMirroredRootDirName,
            );
        }

        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            let srcFileName = *inFileNamesTable.offset((*fCtx).currFileIdx as isize);
            let mut dstFileName = core::ptr::null::<core::ffi::c_char>();
            if !outMirroredRootDirName.is_null() {
                let validMirroredDirName =
                    UTIL_createMirroredDestDirName(srcFileName, outMirroredRootDirName);
                if !validMirroredDirName.is_null() {
                    dstFileName =
                        FIO_determineCompressedName(srcFileName, validMirroredDirName, suffix);
                    free(validMirroredDirName as *mut core::ffi::c_void);
                } else {
                    if g_display_prefs.displayLevel >= 2 {
                        eprintln!(
                            "zstd: --output-dir-mirror cannot compress '{}' into '{}'",
                            CStr::from_ptr(srcFileName).to_string_lossy(),
                            CStr::from_ptr(outMirroredRootDirName).to_string_lossy(),
                        );
                    }
                    error = 1;
                    (*fCtx).currFileIdx += 1;
                    continue;
                }
            } else {
                dstFileName = FIO_determineCompressedName(srcFileName, outDirName, suffix);
            }
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
            }
            error |= status;

            (*fCtx).currFileIdx += 1;
        }
        if !outDirName.is_null() {
            FIO_checkFilenameCollisions(
                inFileNamesTable,
                (*fCtx).nbFilesTotal as core::ffi::c_uint,
            );
        }
    }
    if FIO_shouldDisplayMultipleFileSummary(fCtx) != 0 {
        let hr_isize = UTIL_makeHumanReadableSize((*fCtx).totalBytesInput as u64);
        let hr_osize = UTIL_makeHumanReadableSize((*fCtx).totalBytesOutput as u64);

        if g_display_prefs.progressSetting != FIO_ps_never
            && (g_display_prefs.displayLevel >= 2
                || g_display_prefs.progressSetting == FIO_ps_always)
            && g_display_prefs.displayLevel >= 1
        {
            eprintln!("\r{:79 }\r", "");
        }

        if (*fCtx).totalBytesInput == 0 {
            if (g_display_prefs.displayLevel >= 2
                || g_display_prefs.progressSetting == FIO_ps_always)
                && g_display_prefs.displayLevel >= 1
            {
                fprintf(
                    stderr,
                    b"%3d files compressed : (%6.*f%4s => %6.*f%4s)\n\0" as *const u8
                        as *const core::ffi::c_char,
                    (*fCtx).nbFilesProcessed,
                    hr_isize.precision,
                    hr_isize.value,
                    hr_isize.suffix,
                    hr_osize.precision,
                    hr_osize.value,
                    hr_osize.suffix,
                );
            }
        } else if (g_display_prefs.displayLevel >= 2
            || g_display_prefs.progressSetting == FIO_ps_always)
            && g_display_prefs.displayLevel >= 1
        {
            fprintf(
                stderr,
                b"%3d files compressed : %.2f%% (%6.*f%4s => %6.*f%4s)\n\0" as *const u8
                    as *const core::ffi::c_char,
                (*fCtx).nbFilesProcessed,
                (*fCtx).totalBytesOutput as core::ffi::c_double
                    / (*fCtx).totalBytesInput as core::ffi::c_double
                    * 100.0,
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
unsafe fn FIO_createDResources(
    prefs: *mut FIO_prefs_t,
    dictFileName: *const core::ffi::c_char,
) -> dRess_t {
    let mut useMMap =
        core::ffi::c_int::from((*prefs).mmapDict == ZSTD_ParamSwitch_e::ZSTD_ps_enable);
    let forceNoUseMMap =
        core::ffi::c_int::from((*prefs).mmapDict == ZSTD_ParamSwitch_e::ZSTD_ps_disable);
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
            dictBuffer: core::ptr::null_mut::<core::ffi::c_void>(),
            dictBufferSize: 0,
            dictBufferType: FIO_mallocDict,
        },
        dctx: core::ptr::null_mut::<ZSTD_DStream>(),
        writeCtx: core::ptr::null_mut::<WritePoolCtx_t>(),
        readCtx: core::ptr::null_mut::<ReadPoolCtx_t>(),
    };
    ptr::write_bytes(
        &mut statbuf as *mut stat_t as *mut u8,
        0,
        ::core::mem::size_of::<stat_t>(),
    );
    ptr::write_bytes(
        &mut ress as *mut dRess_t as *mut u8,
        0,
        ::core::mem::size_of::<dRess_t>(),
    );
    FIO_getDictFileStat(dictFileName, &mut statbuf);
    if (*prefs).patchFromMode != 0 {
        let dictSize = UTIL_getFileSizeStat(&statbuf);
        useMMap |= core::ffi::c_int::from(dictSize > u64::from((*prefs).memLimit));
        FIO_adjustMemLimitForPatchFromMode(prefs, dictSize as core::ffi::c_ulonglong, 0);
    }
    ress.dctx = ZSTD_createDStream();
    if (ress.dctx).is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                2351,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                60,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "Error: {} : can't create ZSTD_DStream",
                io::Error::last_os_error(),
            );
        }
        exit(60);
    }
    let mut err: size_t = 0;
    err = ZSTD_DCtx_setMaxWindowSize(ress.dctx, (*prefs).memLimit as size_t);
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_DCtx_setMaxWindowSize(ress.dctx, prefs->memLimit)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                2352,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let mut err_0: size_t = 0;
    err_0 = ZSTD_DCtx_setParameter(
        ress.dctx,
        ZSTD_dParameter::ZSTD_d_experimentalParam3,
        core::ffi::c_int::from((*prefs).checksumFlag == 0),
    );
    if ZSTD_isError(err_0) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_DCtx_setParameter(ress.dctx, ZSTD_d_experimentalParam3, !prefs->checksumFlag)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                2353,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_0),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    let dictBufferType = (if useMMap != 0 && forceNoUseMMap == 0 {
        FIO_mmapDict as core::ffi::c_int
    } else {
        FIO_mallocDict as core::ffi::c_int
    }) as FIO_dictBufferType_t;
    FIO_initDict(
        &mut ress.dict,
        dictFileName,
        prefs,
        &mut statbuf,
        dictBufferType,
    );
    let mut err_1: size_t = 0;
    err_1 = ZSTD_DCtx_reset(ress.dctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
    if ZSTD_isError(err_1) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_DCtx_reset(ress.dctx, ZSTD_ResetDirective::ZSTD_reset_session_only)\0"
                    as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                2360,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err_1),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    if (*prefs).patchFromMode != 0 {
        let mut err_2: size_t = 0;
        err_2 = ZSTD_DCtx_refPrefix(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize);
        if ZSTD_isError(err_2) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_DCtx_refPrefix(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    2363,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_2),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    } else {
        let mut err_3: size_t = 0;
        err_3 = ZSTD_DCtx_loadDictionary_byReference(
            ress.dctx,
            ress.dict.dictBuffer,
            ress.dict.dictBufferSize,
        );
        if ZSTD_isError(err_3) != 0 {
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                    b"ZSTD_DCtx_loadDictionary_byReference(ress.dctx, ress.dict.dictBuffer, ress.dict.dictBufferSize)\0"
                        as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    2365,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    11,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const core::ffi::c_char,
                    ZSTD_getErrorName(err_3),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(11);
        }
    }
    ress.writeCtx = AIO_WritePool_create(prefs, ZSTD_DStreamOutSize());
    ress.readCtx = AIO_ReadPool_create(prefs, ZSTD_DStreamInSize());
    ress
}
unsafe fn FIO_freeDResources(mut ress: dRess_t) {
    FIO_freeDict(&mut ress.dict);
    let mut err: size_t = 0;
    err = ZSTD_freeDStream(ress.dctx);
    if ZSTD_isError(err) != 0 {
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"%s \n\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_freeDStream(ress.dctx)\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                2377,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                11,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(err),
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(11);
    }
    AIO_WritePool_free(ress.writeCtx);
    AIO_ReadPool_free(ress.readCtx);
}
unsafe fn FIO_passThrough(ress: *mut dRess_t) -> core::ffi::c_int {
    let blockSize = if (if ((64 * ((1) << 10)) as size_t) < ZSTD_DStreamInSize() {
        (64 * ((1) << 10)) as size_t
    } else {
        ZSTD_DStreamInSize()
    }) < ZSTD_DStreamOutSize()
    {
        if ((64 * ((1) << 10)) as size_t) < ZSTD_DStreamInSize() {
            (64 * ((1) << 10)) as size_t
        } else {
            ZSTD_DStreamInSize()
        }
    } else {
        ZSTD_DStreamOutSize()
    };
    let mut writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    AIO_ReadPool_fillBuffer((*ress).readCtx, blockSize);
    #[expect(clippy::while_immutable_condition)]
    while (*(*ress).readCtx).srcBufferLoaded != 0 {
        let mut writeSize: size_t = 0;
        writeSize = if blockSize < (*(*ress).readCtx).srcBufferLoaded {
            blockSize
        } else {
            (*(*ress).readCtx).srcBufferLoaded
        };
        assert!(writeSize <= (*writeJob).bufferSize);
        memcpy(
            (*writeJob).buffer,
            (*(*ress).readCtx).srcBuffer as *const core::ffi::c_void,
            writeSize,
        );
        (*writeJob).usedBufferSize = writeSize;
        AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
        AIO_ReadPool_consumeBytes((*ress).readCtx, writeSize);
        AIO_ReadPool_fillBuffer((*ress).readCtx, blockSize);
    }
    assert!((*(*ress).readCtx).reachedEof != 0);
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    0
}
unsafe fn FIO_zstdErrorHelp(
    prefs: *const FIO_prefs_t,
    ress: *const dRess_t,
    mut err: size_t,
    srcFileName: *const core::ffi::c_char,
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
    if ZSTD_getErrorCode(err) as core::ffi::c_uint
        != ZSTD_error_frameParameter_windowTooLarge as core::ffi::c_int as core::ffi::c_uint
    {
        return;
    }
    err = ZSTD_getFrameHeader(
        &mut header,
        (*(*ress).readCtx).srcBuffer as *const core::ffi::c_void,
        (*(*ress).readCtx).srcBufferLoaded,
    );
    if err == 0 {
        let windowSize = header.windowSize;
        let windowLog = (FIO_highbit64(windowSize)).wrapping_add(core::ffi::c_int::from(
            windowSize & windowSize.wrapping_sub(1) != 0,
        ) as core::ffi::c_uint);
        assert!((*prefs).memLimit > 0);
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"%s : Window size larger than maximum : %llu > %u \n\0" as *const u8
                    as *const core::ffi::c_char,
                srcFileName,
                windowSize,
                (*prefs).memLimit,
            );
        }
        if windowLog
            <= (if ::core::mem::size_of::<size_t>() == 4 {
                ZSTD_WINDOWLOG_MAX_32
            } else {
                ZSTD_WINDOWLOG_MAX_64
            }) as core::ffi::c_uint
        {
            let windowMB = (windowSize >> 20).wrapping_add(core::ffi::c_int::from(
                windowSize & (((1) << 20) - 1) as core::ffi::c_ulonglong != 0,
            ) as core::ffi::c_ulonglong) as core::ffi::c_uint;
            assert!(windowSize < ((1 as core::ffi::c_ulonglong) << 52) as core::ffi::c_ulonglong);
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s : Use --long=%u or --memory=%uMB \n\0" as *const u8
                        as *const core::ffi::c_char,
                    srcFileName,
                    windowLog,
                    windowMB,
                );
            }
            return;
        }
    }
    if g_display_prefs.displayLevel >= 1 {
        fprintf(
            stderr,
            b"%s : Window log larger than ZSTD_WINDOWLOG_MAX=%u; not supported \n\0" as *const u8
                as *const core::ffi::c_char,
            srcFileName,
            if ::core::mem::size_of::<size_t>() == 4 {
                30
            } else {
                31
            },
        );
    }
}
pub const FIO_ERROR_FRAME_DECODING: core::ffi::c_int = -(2);
unsafe fn FIO_decompressZstdFrame(
    fCtx: *mut FIO_ctx_t,
    ress: *mut dRess_t,
    prefs: *const FIO_prefs_t,
    srcFileName: *const core::ffi::c_char,
    alreadyDecoded: u64,
) -> core::ffi::c_ulonglong {
    let mut frameSize = 0;
    let mut srcFName20 = srcFileName;
    let mut writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    assert!(!writeJob.is_null());
    let srcFileLength = strlen(srcFileName);
    if srcFileLength > 20 && g_display_prefs.displayLevel < 3 {
        srcFName20 = srcFName20.add(srcFileLength.wrapping_sub(20));
    }
    ZSTD_DCtx_reset((*ress).dctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
    AIO_ReadPool_fillBuffer((*ress).readCtx, ZSTD_FRAMEHEADERSIZE_MAX as size_t);
    loop {
        let mut inBuff = setInBuffer(
            (*(*ress).readCtx).srcBuffer as *const core::ffi::c_void,
            (*(*ress).readCtx).srcBufferLoaded,
            0,
        );
        let mut outBuff = setOutBuffer((*writeJob).buffer, (*writeJob).bufferSize, 0);
        let readSizeHint = ZSTD_decompressStream((*ress).dctx, &mut outBuff, &mut inBuff);
        let hrs = UTIL_makeHumanReadableSize(alreadyDecoded.wrapping_add(frameSize));
        if ZSTD_isError(readSizeHint) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"%s : Decoding error (36) : %s \n\0" as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                    ZSTD_getErrorName(readSizeHint),
                );
            }
            FIO_zstdErrorHelp(prefs, ress, readSizeHint, srcFileName);
            AIO_WritePool_releaseIoJob(writeJob);
            return FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong;
        }
        (*writeJob).usedBufferSize = outBuff.pos;
        AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
        frameSize = frameSize.wrapping_add(outBuff.pos as u64);
        if (*fCtx).nbFilesTotal > 1 {
            if g_display_prefs.progressSetting as core::ffi::c_uint
                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                && (g_display_prefs.displayLevel >= 2
                    || g_display_prefs.progressSetting as core::ffi::c_uint
                        == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
                && g_display_prefs.displayLevel >= 1
                && g_display_prefs.progressSetting as core::ffi::c_uint
                    != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
                && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                    || g_display_prefs.displayLevel >= 4)
            {
                g_displayClock = UTIL_getTime();
                fprintf(
                    stderr,
                    b"\rDecompress: %2u/%2u files. Current: %s : %.*f%s...    \0" as *const u8
                        as *const core::ffi::c_char,
                    (*fCtx).currFileIdx + 1,
                    (*fCtx).nbFilesTotal,
                    srcFName20,
                    hrs.precision,
                    hrs.value,
                    hrs.suffix,
                );
                if g_display_prefs.displayLevel >= 4 {
                    fflush(stderr);
                }
            }
        } else if g_display_prefs.progressSetting as core::ffi::c_uint
            != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2
                || g_display_prefs.progressSetting as core::ffi::c_uint
                    == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1
            && g_display_prefs.progressSetting as core::ffi::c_uint
                != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
            && (UTIL_clockSpanMicro(g_displayClock) > REFRESH_RATE
                || g_display_prefs.displayLevel >= 4)
        {
            g_displayClock = UTIL_getTime();
            fprintf(
                stderr,
                b"\r%-20.20s : %.*f%s...     \0" as *const u8 as *const core::ffi::c_char,
                srcFName20,
                hrs.precision,
                hrs.value,
                hrs.suffix,
            );
            if g_display_prefs.displayLevel >= 4 {
                fflush(stderr);
            }
        }
        AIO_ReadPool_consumeBytes((*ress).readCtx, inBuff.pos);
        if readSizeHint == 0 {
            break;
        }
        let toDecode = if readSizeHint < ZSTD_DStreamInSize() {
            readSizeHint
        } else {
            ZSTD_DStreamInSize()
        };
        if (*(*ress).readCtx).srcBufferLoaded < toDecode {
            let readSize = AIO_ReadPool_fillBuffer((*ress).readCtx, toDecode);
            if readSize == 0 {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"%s : Read error (39) : premature end \n\0" as *const u8
                            as *const core::ffi::c_char,
                        srcFileName,
                    );
                }
                AIO_WritePool_releaseIoJob(writeJob);
                return FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong;
            }
        }
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    frameSize as core::ffi::c_ulonglong
}
unsafe fn FIO_decompressGzFrame(
    ress: *mut dRess_t,
    srcFileName: *const core::ffi::c_char,
) -> core::ffi::c_ulonglong {
    let mut outFileSize = 0 as core::ffi::c_ulonglong;
    let mut strm = z_stream_s {
        next_in: core::ptr::null_mut::<Bytef>(),
        avail_in: 0,
        total_in: 0,
        next_out: core::ptr::null_mut::<Bytef>(),
        avail_out: 0,
        total_out: 0,
        msg: core::ptr::null_mut::<core::ffi::c_char>(),
        state: core::ptr::null_mut::<internal_state>(),
        zalloc: None,
        zfree: None,
        opaque: core::ptr::null_mut::<core::ffi::c_void>(),
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut flush = Z_NO_FLUSH;
    let mut decodingError = 0;
    let mut writeJob = core::ptr::null_mut();
    strm.zalloc = None;
    strm.zfree = None;
    strm.opaque = core::ptr::null_mut();
    strm.next_in = core::ptr::null_mut::<Bytef>();
    strm.avail_in = 0;
    if inflateInit2_(
        &mut strm,
        15 + 16,
        ZLIB_VERSION.as_ptr(),
        ::core::mem::size_of::<z_stream>() as core::ffi::c_int,
    ) != Z_OK
    {
        return FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong;
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_out = (*writeJob).buffer as *mut Bytef;
    strm.avail_out = (*writeJob).bufferSize as uInt;
    strm.avail_in = (*(*ress).readCtx).srcBufferLoaded as uInt;
    strm.next_in = (*(*ress).readCtx).srcBuffer as *mut core::ffi::c_uchar;
    loop {
        let mut ret: core::ffi::c_int = 0;
        if strm.avail_in == 0 {
            AIO_ReadPool_consumeAndRefill((*ress).readCtx);
            if (*(*ress).readCtx).srcBufferLoaded == 0 {
                flush = Z_FINISH;
            }
            strm.next_in = (*(*ress).readCtx).srcBuffer as *mut core::ffi::c_uchar;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded as uInt;
        }
        ret = inflate(&mut strm, flush);
        if ret == Z_BUF_ERROR {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: premature gz end \n\0" as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                );
            }
            decodingError = 1;
            break;
        } else if ret != Z_OK && ret != Z_STREAM_END {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: inflate error %d \n\0" as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                    ret,
                );
            }
            decodingError = 1;
            break;
        } else {
            let decompBytes = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out as size_t);
            if decompBytes != 0 {
                (*writeJob).usedBufferSize = decompBytes;
                AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
                outFileSize = outFileSize.wrapping_add(decompBytes as core::ffi::c_ulonglong);
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
    if inflateEnd(&mut strm) != Z_OK && decodingError == 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s: inflateEnd error \n\0" as *const u8 as *const core::ffi::c_char,
                srcFileName,
            );
        }
        decodingError = 1;
    }
    AIO_WritePool_releaseIoJob(writeJob);
    AIO_WritePool_sparseWriteEnd((*ress).writeCtx);
    if decodingError != 0 {
        FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong
    } else {
        outFileSize
    }
}
unsafe fn FIO_decompressLzmaFrame(
    ress: *mut dRess_t,
    srcFileName: *const core::ffi::c_char,
    plain_lzma: core::ffi::c_int,
) -> core::ffi::c_ulonglong {
    let mut outFileSize = 0 as core::ffi::c_ulonglong;
    let mut strm = {
        lzma_stream {
            next_in: core::ptr::null(),
            avail_in: 0,
            total_in: 0,
            next_out: core::ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            allocator: core::ptr::null(),
            internal: core::ptr::null_mut(),
            reserved_ptr1: core::ptr::null_mut(),
            reserved_ptr2: core::ptr::null_mut(),
            reserved_ptr3: core::ptr::null_mut(),
            reserved_ptr4: core::ptr::null_mut(),
            seek_pos: 0,
            reserved_int2: 0,
            reserved_int3: 0,
            reserved_int4: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
        }
    };
    let mut action = LZMA_RUN;
    let mut initRet = LZMA_OK;
    let mut decodingError = 0;
    let mut writeJob = core::ptr::null_mut();
    strm.next_in = core::ptr::null::<u8>();
    strm.avail_in = 0;
    if plain_lzma != 0 {
        initRet = lzma_alone_decoder(&mut strm, UINT64_MAX);
    } else {
        initRet = lzma_stream_decoder(&mut strm, UINT64_MAX, 0);
    }
    if initRet as core::ffi::c_uint != LZMA_OK as core::ffi::c_int as core::ffi::c_uint {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s: %s error %d \n\0" as *const u8 as *const core::ffi::c_char,
                if plain_lzma != 0 {
                    b"lzma_alone_decoder\0" as *const u8 as *const core::ffi::c_char
                } else {
                    b"lzma_stream_decoder\0" as *const u8 as *const core::ffi::c_char
                },
                srcFileName,
                initRet as core::ffi::c_uint,
            );
        }
        return FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong;
    }
    writeJob = AIO_WritePool_acquireJob((*ress).writeCtx);
    strm.next_out = (*writeJob).buffer as *mut u8;
    strm.avail_out = (*writeJob).bufferSize;
    strm.next_in = (*(*ress).readCtx).srcBuffer as *const u8;
    strm.avail_in = (*(*ress).readCtx).srcBufferLoaded;
    loop {
        let mut ret = LZMA_OK;
        if strm.avail_in == 0 {
            AIO_ReadPool_consumeAndRefill((*ress).readCtx);
            if (*(*ress).readCtx).srcBufferLoaded == 0 {
                action = LZMA_FINISH;
            }
            strm.next_in = (*(*ress).readCtx).srcBuffer as *const u8;
            strm.avail_in = (*(*ress).readCtx).srcBufferLoaded;
        }
        ret = lzma_code(&mut strm, action);
        if ret as core::ffi::c_uint == LZMA_BUF_ERROR as core::ffi::c_int as core::ffi::c_uint {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: premature lzma end \n\0" as *const u8 as *const core::ffi::c_char,
                    srcFileName,
                );
            }
            decodingError = 1;
            break;
        } else if ret as core::ffi::c_uint != LZMA_OK as core::ffi::c_int as core::ffi::c_uint
            && ret as core::ffi::c_uint != LZMA_STREAM_END as core::ffi::c_int as core::ffi::c_uint
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: %s: lzma_code decoding error %d \n\0" as *const u8
                        as *const core::ffi::c_char,
                    srcFileName,
                    ret as core::ffi::c_uint,
                );
            }
            decodingError = 1;
            break;
        } else {
            let decompBytes = ((*writeJob).bufferSize).wrapping_sub(strm.avail_out);
            if decompBytes != 0 {
                (*writeJob).usedBufferSize = decompBytes;
                AIO_WritePool_enqueueAndReacquireWriteJob(&mut writeJob);
                outFileSize = outFileSize.wrapping_add(decompBytes as core::ffi::c_ulonglong);
                strm.next_out = (*writeJob).buffer as *mut u8;
                strm.avail_out = (*writeJob).bufferSize;
            }
            if ret as core::ffi::c_uint == LZMA_STREAM_END as core::ffi::c_int as core::ffi::c_uint
            {
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
        FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong
    } else {
        outFileSize
    }
}
unsafe fn FIO_decompressFrames(
    fCtx: *mut FIO_ctx_t,
    mut ress: dRess_t,
    prefs: *const FIO_prefs_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut readSomething = 0;
    let mut filesize = 0 as core::ffi::c_ulonglong;
    let mut passThrough = (*prefs).passThrough;
    if passThrough == -(1) {
        passThrough = core::ffi::c_int::from(
            (*prefs).overwrite != 0 && strcmp(dstFileName, stdoutmark.as_ptr()) == 0,
        );
    }
    assert!(passThrough == 0 || passThrough == 1);
    loop {
        let toRead = 4;
        let mut buf = core::ptr::null::<u8>();
        AIO_ReadPool_fillBuffer(ress.readCtx, toRead);
        buf = (*ress.readCtx).srcBuffer as *const u8;
        if (*ress.readCtx).srcBufferLoaded == 0 {
            if readSomething == 0 {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"zstd: %s: unexpected end of file \n\0" as *const u8
                            as *const core::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1;
            }
            break;
        } else {
            readSomething = 1;
            if (*ress.readCtx).srcBufferLoaded < toRead {
                if passThrough != 0 {
                    return FIO_passThrough(&mut ress);
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"zstd: %s: unknown header \n\0" as *const u8 as *const core::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1;
            }
            if ZSTD_isFrame(
                buf as *const core::ffi::c_void,
                (*ress.readCtx).srcBufferLoaded,
            ) != 0
            {
                let frameSize =
                    FIO_decompressZstdFrame(fCtx, &mut ress, prefs, srcFileName, filesize as u64);
                if frameSize == FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong {
                    return 1;
                }
                filesize = filesize.wrapping_add(frameSize);
            } else if core::ffi::c_int::from(*buf.offset(0)) == 31
                && core::ffi::c_int::from(*buf.offset(1)) == 139
            {
                let frameSize_0 = FIO_decompressGzFrame(&mut ress, srcFileName);
                if frameSize_0 == FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong {
                    return 1;
                }
                filesize = filesize.wrapping_add(frameSize_0);
            } else if core::ffi::c_int::from(*buf.offset(0)) == 0xfd as core::ffi::c_int
                && core::ffi::c_int::from(*buf.offset(1)) == 0x37 as core::ffi::c_int
                || core::ffi::c_int::from(*buf.offset(0)) == 0x5d as core::ffi::c_int
                    && core::ffi::c_int::from(*buf.offset(1)) == 0
            {
                let frameSize_1 = FIO_decompressLzmaFrame(
                    &mut ress,
                    srcFileName,
                    core::ffi::c_int::from(
                        core::ffi::c_int::from(*buf.offset(0)) != 0xfd as core::ffi::c_int,
                    ),
                );
                if frameSize_1 == FIO_ERROR_FRAME_DECODING as core::ffi::c_ulonglong {
                    return 1;
                }
                filesize = filesize.wrapping_add(frameSize_1);
            } else if MEM_readLE32(buf as *const core::ffi::c_void) == LZ4_MAGICNUMBER as u32 {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"zstd: %s: lz4 file cannot be uncompressed (zstd compiled without HAVE_LZ4) -- ignored \n\0"
                            as *const u8 as *const core::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1;
            } else if passThrough != 0 {
                return FIO_passThrough(&mut ress);
            } else {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"zstd: %s: unsupported format \n\0" as *const u8
                            as *const core::ffi::c_char,
                        srcFileName,
                    );
                }
                return 1;
            }
        }
    }
    (*fCtx).totalBytesOutput = ((*fCtx).totalBytesOutput).wrapping_add(filesize as size_t);
    if g_display_prefs.progressSetting as core::ffi::c_uint
        != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
        && (g_display_prefs.displayLevel >= 2
            || g_display_prefs.progressSetting as core::ffi::c_uint
                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
        && g_display_prefs.displayLevel >= 1
    {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
    }
    if FIO_shouldDisplayFileSummary(fCtx) != 0
        && (g_display_prefs.displayLevel >= 2
            || g_display_prefs.progressSetting as core::ffi::c_uint
                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
        && g_display_prefs.displayLevel >= 1
    {
        fprintf(
            stderr,
            b"%-20s: %llu bytes \n\0" as *const u8 as *const core::ffi::c_char,
            srcFileName,
            filesize,
        );
    }
    0
}
unsafe fn FIO_decompressDstFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    ress: dRess_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
    srcFileStat: *const stat_t,
) -> core::ffi::c_int {
    let mut result: core::ffi::c_int = 0;
    let mut releaseDstFile = 0;
    let mut transferStat = 0;
    let mut dstFd = 0;
    if (AIO_WritePool_getFile(ress.writeCtx)).is_null() && (*prefs).testMode == 0 {
        let mut dstFile = core::ptr::null_mut::<FILE>();
        let mut dstFilePermissions = DEFAULT_FILE_PERMISSIONS;
        if strcmp(srcFileName, stdinmark.as_ptr()) != 0
            && strcmp(dstFileName, stdoutmark.as_ptr()) != 0
            && UTIL_isRegularFileStat(srcFileStat) != 0
        {
            transferStat = 1;
            dstFilePermissions = TEMPORARY_FILE_PERMISSIONS;
        }
        releaseDstFile = 1;
        dstFile = FIO_openDstFile(fCtx, prefs, srcFileName, dstFileName, dstFilePermissions);
        if dstFile.is_null() {
            return 1;
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
            if g_display_prefs.displayLevel >= 1 {
                eprintln!(
                    "zstd: {}: {}",
                    CStr::from_ptr(dstFileName).to_string_lossy(),
                    io::Error::last_os_error(),
                );
            }
            result = 1;
        }
        if transferStat != 0 {
            UTIL_utime(dstFileName, srcFileStat);
        }
        if result != 0 && strcmp(dstFileName, stdoutmark.as_ptr()) != 0 {
            FIO_removeFile(dstFileName);
        }
    }
    result
}
unsafe fn FIO_decompressSrcFile(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    ress: dRess_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut srcFile = core::ptr::null_mut::<FILE>();
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
    let mut result: core::ffi::c_int = 0;
    let mut fileSize = UTIL_FILESIZE_UNKNOWN as u64;
    if UTIL_isDirectory(srcFileName) != 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s is a directory -- ignored \n\0" as *const u8 as *const core::ffi::c_char,
                srcFileName,
            );
        }
        return 1;
    }
    srcFile = FIO_openSrcFile(prefs, srcFileName, &mut srcFileStat);
    if srcFile.is_null() {
        return 1;
    }
    if strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        fileSize = UTIL_getFileSizeStat(&srcFileStat);
    }
    if fileSize != UTIL_FILESIZE_UNKNOWN as u64 && fileSize < (ZSTD_BLOCKSIZE_MAX * 3) as u64 {
        AIO_ReadPool_setAsync(ress.readCtx, 0);
        AIO_WritePool_setAsync(ress.writeCtx, 0);
    } else {
        AIO_ReadPool_setAsync(ress.readCtx, 1);
        AIO_WritePool_setAsync(ress.writeCtx, 1);
    }
    AIO_ReadPool_setFile(ress.readCtx, srcFile);
    result = FIO_decompressDstFile(fCtx, prefs, ress, dstFileName, srcFileName, &srcFileStat);
    AIO_ReadPool_setFile(ress.readCtx, core::ptr::null_mut());
    if fclose(srcFile) != 0 {
        if g_display_prefs.displayLevel >= 1 {
            eprintln!(
                "zstd: {}: {}",
                CStr::from_ptr(srcFileName).to_string_lossy(),
                io::Error::last_os_error(),
            );
        }
        return 1;
    }
    if (*prefs).removeSrcFile != 0 && result == 0 && strcmp(srcFileName, stdinmark.as_ptr()) != 0 {
        clearHandler();
        if FIO_removeFile(srcFileName) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                eprintln!(
                    "zstd: {}: {}",
                    CStr::from_ptr(srcFileName).to_string_lossy(),
                    io::Error::last_os_error(),
                );
            }
            return 1;
        }
    }
    result
}
pub unsafe fn FIO_decompressFilename(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    dstFileName: *const core::ffi::c_char,
    srcFileName: *const core::ffi::c_char,
    dictFileName: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let ress = FIO_createDResources(prefs, dictFileName);
    let decodingError = FIO_decompressSrcFile(fCtx, prefs, ress, dstFileName, srcFileName);
    FIO_freeDResources(ress);
    decodingError
}
static suffixList: [&CStr; 8] = [
    ZSTD_EXTENSION,
    TZSTD_EXTENSION,
    ZSTD_ALT_EXTENSION,
    GZ_EXTENSION,
    TGZ_EXTENSION,
    LZMA_EXTENSION,
    XZ_EXTENSION,
    TXZ_EXTENSION,
];
static suffixListStr: &CStr = c".zst/.tzst/.gz/.tgz/.lzma/.xz/.txz";
unsafe fn FIO_determineDstName(
    srcFileName: *const core::ffi::c_char,
    outDirName: *const core::ffi::c_char,
) -> *const core::ffi::c_char {
    static mut dfnbCapacity: size_t = 0;
    static mut dstFileNameBuffer: *mut core::ffi::c_char = core::ptr::null_mut();
    let mut dstFileNameEndPos: size_t = 0;
    let mut outDirFilename = core::ptr::null_mut();
    let mut dstSuffix = b"\0" as *const u8 as *const core::ffi::c_char;
    let mut dstSuffixLen = 0;
    let mut sfnSize = strlen(srcFileName);
    let mut srcSuffixLen: size_t = 0;
    let srcSuffix: *const core::ffi::c_char = strrchr(srcFileName, '.' as i32);
    if strcmp(srcFileName, stdinmark.as_ptr()) == 0 {
        return stdoutmark.as_ptr();
    }
    if srcSuffix.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s: unknown suffix (%s expected). Can't derive the output file name. Specify it with -o dstFileName. Ignoring.\n\0"
                    as *const u8 as *const core::ffi::c_char,
                srcFileName,
                suffixListStr.as_ptr(),
            );
        }
        return core::ptr::null();
    }
    srcSuffixLen = strlen(srcSuffix);
    let matchedSuffix = suffixList
        .iter()
        .find(|suffix| strcmp(suffix.as_ptr(), srcSuffix) == 0);
    if sfnSize <= srcSuffixLen || matchedSuffix.is_none() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: %s: unknown suffix (%s expected). Can't derive the output file name. Specify it with -o dstFileName. Ignoring.\n\0"
                    as *const u8 as *const core::ffi::c_char,
                srcFileName,
                suffixListStr.as_ptr(),
            );
        }
        return core::ptr::null();
    }
    if core::ffi::c_int::from(*matchedSuffix.unwrap().as_ptr().offset(1)) == 't' as i32 {
        dstSuffix = b".tar\0" as *const u8 as *const core::ffi::c_char;
        dstSuffixLen = strlen(dstSuffix);
    }
    if !outDirName.is_null() {
        outDirFilename = FIO_createFilename_fromOutDir(srcFileName, outDirName, 0);
        sfnSize = strlen(outDirFilename);
        assert!(!outDirFilename.is_null());
    }
    if dfnbCapacity.wrapping_add(srcSuffixLen) <= sfnSize.wrapping_add(1).wrapping_add(dstSuffixLen)
    {
        free(dstFileNameBuffer as *mut core::ffi::c_void);
        dfnbCapacity = sfnSize.wrapping_add(20);
        dstFileNameBuffer = malloc(dfnbCapacity) as *mut core::ffi::c_char;
        if dstFileNameBuffer.is_null() {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    3067,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    74,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                eprintln!(
                    "{} : not enough memory for dstFileName",
                    io::Error::last_os_error(),
                );
            }
            exit(74);
        }
    }
    assert!(!dstFileNameBuffer.is_null());
    dstFileNameEndPos = sfnSize.wrapping_sub(srcSuffixLen);
    if !outDirFilename.is_null() {
        memcpy(
            dstFileNameBuffer as *mut core::ffi::c_void,
            outDirFilename as *const core::ffi::c_void,
            dstFileNameEndPos,
        );
        free(outDirFilename as *mut core::ffi::c_void);
    } else {
        memcpy(
            dstFileNameBuffer as *mut core::ffi::c_void,
            srcFileName as *const core::ffi::c_void,
            dstFileNameEndPos,
        );
    }
    strcpy(dstFileNameBuffer.add(dstFileNameEndPos), dstSuffix);
    dstFileNameBuffer
}
pub unsafe fn FIO_decompressMultipleFilenames(
    fCtx: *mut FIO_ctx_t,
    prefs: *mut FIO_prefs_t,
    srcNamesTable: *mut *const core::ffi::c_char,
    outMirroredRootDirName: *const core::ffi::c_char,
    outDirName: *const core::ffi::c_char,
    outFileName: *const core::ffi::c_char,
    dictFileName: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut status: core::ffi::c_int = 0;
    let mut error = 0;
    let ress = FIO_createDResources(prefs, dictFileName);
    if !outFileName.is_null() {
        if FIO_multiFilesConcatWarning(fCtx, prefs, outFileName, 1) != 0 {
            FIO_freeDResources(ress);
            return 1;
        }
        if (*prefs).testMode == 0 {
            let dstFile = FIO_openDstFile(
                fCtx,
                prefs,
                core::ptr::null(),
                outFileName,
                DEFAULT_FILE_PERMISSIONS,
            );
            if dstFile.is_null() {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const core::ffi::c_char,
                        b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                        3107,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                        19,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"cannot open %s\0" as *const u8 as *const core::ffi::c_char,
                        outFileName,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                }
                exit(19);
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
            }
            error |= status;
            (*fCtx).currFileIdx += 1;
        }
        if (*prefs).testMode == 0 && AIO_WritePool_closeFile(ress.writeCtx) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio.c\0" as *const u8 as *const core::ffi::c_char,
                    3117,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    72,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                eprintln!(
                    "Write error : {} : cannot properly close output file",
                    io::Error::last_os_error(),
                );
            }
            exit(72);
        }
    } else {
        if !outMirroredRootDirName.is_null() {
            UTIL_mirrorSourceFilesDirectories(
                srcNamesTable,
                (*fCtx).nbFilesTotal as core::ffi::c_uint,
                outMirroredRootDirName,
            );
        }
        while (*fCtx).currFileIdx < (*fCtx).nbFilesTotal {
            let srcFileName = *srcNamesTable.offset((*fCtx).currFileIdx as isize);
            let mut dstFileName = core::ptr::null();
            if !outMirroredRootDirName.is_null() {
                let validMirroredDirName =
                    UTIL_createMirroredDestDirName(srcFileName, outMirroredRootDirName);
                if !validMirroredDirName.is_null() {
                    dstFileName = FIO_determineDstName(srcFileName, validMirroredDirName);
                    free(validMirroredDirName as *mut core::ffi::c_void);
                } else if g_display_prefs.displayLevel >= 2 {
                    fprintf(
                        stderr,
                        b"zstd: --output-dir-mirror cannot decompress '%s' into '%s'\n\0"
                            as *const u8 as *const core::ffi::c_char,
                        srcFileName,
                        outMirroredRootDirName,
                    );
                }
            } else {
                dstFileName = FIO_determineDstName(srcFileName, outDirName);
            }
            if dstFileName.is_null() {
                error = 1;
            } else {
                status = FIO_decompressSrcFile(fCtx, prefs, ress, dstFileName, srcFileName);
                if status == 0 {
                    (*fCtx).nbFilesProcessed += 1;
                }
                error |= status;
            }
            (*fCtx).currFileIdx += 1;
        }
        if !outDirName.is_null() {
            FIO_checkFilenameCollisions(srcNamesTable, (*fCtx).nbFilesTotal as core::ffi::c_uint);
        }
    }
    if FIO_shouldDisplayMultipleFileSummary(fCtx) != 0 {
        if g_display_prefs.progressSetting as core::ffi::c_uint
            != FIO_ps_never as core::ffi::c_int as core::ffi::c_uint
            && (g_display_prefs.displayLevel >= 2
                || g_display_prefs.progressSetting as core::ffi::c_uint
                    == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1
        {
            fprintf(
                stderr,
                b"\r%79s\r\0" as *const u8 as *const core::ffi::c_char,
                b"\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        if (g_display_prefs.displayLevel >= 2
            || g_display_prefs.progressSetting as core::ffi::c_uint
                == FIO_ps_always as core::ffi::c_int as core::ffi::c_uint)
            && g_display_prefs.displayLevel >= 1
        {
            fprintf(
                stderr,
                b"%d files decompressed : %6llu bytes total \n\0" as *const u8
                    as *const core::ffi::c_char,
                (*fCtx).nbFilesProcessed,
                (*fCtx).totalBytesOutput as core::ffi::c_ulonglong,
            );
        }
    }
    FIO_freeDResources(ress);
    error
}
unsafe fn FIO_analyzeFrames(info: *mut fileInfo_t, srcFile: *mut FILE) -> InfoError {
    loop {
        let mut headerBuffer: [u8; 18] = [0; 18];
        let numBytesRead = fread(
            headerBuffer.as_mut_ptr() as *mut core::ffi::c_void,
            1,
            ::core::mem::size_of::<[u8; 18]>(),
            srcFile,
        );
        if numBytesRead
            < (if ZSTD_f_zstd1 as core::ffi::c_int == ZSTD_f_zstd1 as core::ffi::c_int {
                6
            } else {
                2
            }) as size_t
        {
            if feof(srcFile) != 0
                && numBytesRead == 0
                && (*info).compressedSize > 0
                && (*info).compressedSize != UTIL_FILESIZE_UNKNOWN as u64
            {
                let file_position = ftell(srcFile) as core::ffi::c_ulonglong;
                let file_size = (*info).compressedSize as core::ffi::c_ulonglong;
                if file_position != file_size {
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(
                            stderr,
                            b"Error: seeked to position %llu, which is beyond file size of %llu\n\0"
                                as *const u8
                                as *const core::ffi::c_char,
                            file_position,
                            file_size,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    }
                    return info_truncated_input;
                }
                break;
            } else {
                if feof(srcFile) != 0 {
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(
                            stderr,
                            b"Error: reached end of file with incomplete frame\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    }
                    return info_not_zstd;
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error: did not reach end of file but ran out of frames\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                }
                return info_frame_error;
            }
        } else {
            let magicNumber = MEM_readLE32(headerBuffer.as_mut_ptr() as *const core::ffi::c_void);
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
                    headerBuffer.as_mut_ptr() as *const core::ffi::c_void,
                    numBytesRead,
                ) as u64;
                if frameContentSize as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_ERROR
                    || frameContentSize as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN
                {
                    (*info).decompUnavailable = 1;
                } else {
                    (*info).decompressedSize =
                        ((*info).decompressedSize).wrapping_add(frameContentSize);
                }
                if ZSTD_getFrameHeader(
                    &mut header,
                    headerBuffer.as_mut_ptr() as *const core::ffi::c_void,
                    numBytesRead,
                ) != 0
                {
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(
                            stderr,
                            b"Error: could not decode frame header\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    }
                    return info_frame_error;
                }
                if (*info).dictID != 0 && (*info).dictID != header.dictID {
                    fprintf(
                        stderr,
                        b"WARNING: File contains multiple frames with different dictionary IDs. Showing dictID 0 instead\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                    (*info).dictID = 0;
                } else {
                    (*info).dictID = header.dictID;
                }
                (*info).windowSize = header.windowSize;
                let headerSize = ZSTD_frameHeaderSize(
                    headerBuffer.as_mut_ptr() as *const core::ffi::c_void,
                    numBytesRead,
                );
                if ZSTD_isError(headerSize) != 0 {
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(
                            stderr,
                            b"Error: could not determine frame header size\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    }
                    return info_frame_error;
                }
                if fseek(
                    srcFile,
                    headerSize as core::ffi::c_long - numBytesRead as core::ffi::c_long,
                    1,
                ) != 0
                {
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(
                            stderr,
                            b"Error: could not move to end of frame header\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    }
                    return info_frame_error;
                }
                let mut lastBlock = 0;
                loop {
                    let mut blockHeaderBuffer: [u8; 3] = [0; 3];
                    if fread(
                        blockHeaderBuffer.as_mut_ptr() as *mut core::ffi::c_void,
                        1,
                        3,
                        srcFile,
                    ) != 3
                    {
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(
                                stderr,
                                b"Error while reading block header\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                    let blockHeader =
                        MEM_readLE24(blockHeaderBuffer.as_mut_ptr() as *const core::ffi::c_void);
                    let blockTypeID = blockHeader >> 1 & 3;
                    let isRLE = core::ffi::c_int::from(blockTypeID == 1) as u32;
                    let isWrongBlock = core::ffi::c_int::from(blockTypeID == 3) as u32;
                    let blockSize = if isRLE != 0 {
                        1
                    } else {
                        (blockHeader >> 3) as core::ffi::c_long
                    };
                    if isWrongBlock != 0 {
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(
                                stderr,
                                b"Error: unsupported block type\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                    lastBlock = (blockHeader & 1) as core::ffi::c_int;
                    if fseek(srcFile, blockSize, 1) != 0 {
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(
                                stderr,
                                b"Error: could not skip to end of block\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                    if lastBlock == 1 {
                        break;
                    }
                }
                let frameHeaderDescriptor = *headerBuffer.as_mut_ptr().offset(4);
                let contentChecksumFlag =
                    (core::ffi::c_int::from(frameHeaderDescriptor) & (1) << 2) >> 2;
                if contentChecksumFlag != 0 {
                    (*info).usesCheck = 1;
                    if fread(
                        ((*info).checksum).as_mut_ptr() as *mut core::ffi::c_void,
                        1,
                        4,
                        srcFile,
                    ) != 4
                    {
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(
                                stderr,
                                b"Error: could not read checksum\0" as *const u8
                                    as *const core::ffi::c_char,
                            );
                        }
                        if g_display_prefs.displayLevel >= 1 {
                            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                        }
                        return info_frame_error;
                    }
                }
                (*info).numActualFrames += 1;
            } else if magicNumber & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START {
                let frameSize =
                    MEM_readLE32(headerBuffer.as_mut_ptr().offset(4) as *const core::ffi::c_void);
                let seek = (8u32.wrapping_add(frameSize) as size_t).wrapping_sub(numBytesRead)
                    as core::ffi::c_long;
                if fseek(srcFile, seek, 1) != 0 {
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(
                            stderr,
                            b"Error: could not find end of skippable frame\0" as *const u8
                                as *const core::ffi::c_char,
                        );
                    }
                    if g_display_prefs.displayLevel >= 1 {
                        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    }
                    return info_frame_error;
                }
                (*info).numSkippableFrames += 1;
            } else {
                return info_not_zstd;
            }
        }
    }
    info_success
}
unsafe fn getFileInfo_fileConfirmed(
    info: *mut fileInfo_t,
    inFileName: *const core::ffi::c_char,
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
    let srcFile = FIO_openSrcFile(core::ptr::null(), inFileName, &mut srcFileStat);
    if srcFile.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Error: could not open source file %s\0" as *const u8 as *const core::ffi::c_char,
                inFileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        return info_file_error;
    }
    (*info).compressedSize = UTIL_getFileSizeStat(&srcFileStat);
    status = FIO_analyzeFrames(info, srcFile);
    fclose(srcFile);
    (*info).nbFiles = 1;
    status
}
unsafe fn getFileInfo(info: *mut fileInfo_t, srcFileName: *const core::ffi::c_char) -> InfoError {
    if UTIL_isRegularFile(srcFileName) == 0 {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : %s is not a file\0" as *const u8 as *const core::ffi::c_char,
                srcFileName,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        return info_file_error;
    }
    getFileInfo_fileConfirmed(info, srcFileName)
}
unsafe fn displayInfo(
    inFileName: *const core::ffi::c_char,
    info: *const fileInfo_t,
    displayLevel: core::ffi::c_int,
) {
    let window_hrs = UTIL_makeHumanReadableSize((*info).windowSize);
    let compressed_hrs = UTIL_makeHumanReadableSize((*info).compressedSize);
    let decompressed_hrs = UTIL_makeHumanReadableSize((*info).decompressedSize);
    let ratio = if (*info).compressedSize == 0 {
        0.0
    } else {
        (*info).decompressedSize as core::ffi::c_double
            / (*info).compressedSize as core::ffi::c_double
    };
    let checkString = if (*info).usesCheck != 0 {
        b"XXH64\0" as *const u8 as *const core::ffi::c_char
    } else {
        b"None\0" as *const u8 as *const core::ffi::c_char
    };
    if displayLevel <= 2 {
        if (*info).decompUnavailable == 0 {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s  %8.*f%4s  %5.3f  %5s  %s\n\0" as *const u8
                    as *const core::ffi::c_char,
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
                    as *const core::ffi::c_char,
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
            b"%s \n\0" as *const u8 as *const core::ffi::c_char,
            inFileName,
        );
        fprintf(
            stdout,
            b"# Zstandard Frames: %d\n\0" as *const u8 as *const core::ffi::c_char,
            (*info).numActualFrames,
        );
        if (*info).numSkippableFrames != 0 {
            fprintf(
                stdout,
                b"# Skippable Frames: %d\n\0" as *const u8 as *const core::ffi::c_char,
                (*info).numSkippableFrames,
            );
        }
        fprintf(
            stdout,
            b"DictID: %u\n\0" as *const u8 as *const core::ffi::c_char,
            (*info).dictID,
        );
        fprintf(
            stdout,
            b"Window Size: %.*f%s (%llu B)\n\0" as *const u8 as *const core::ffi::c_char,
            window_hrs.precision,
            window_hrs.value,
            window_hrs.suffix,
            (*info).windowSize as core::ffi::c_ulonglong,
        );
        fprintf(
            stdout,
            b"Compressed Size: %.*f%s (%llu B)\n\0" as *const u8 as *const core::ffi::c_char,
            compressed_hrs.precision,
            compressed_hrs.value,
            compressed_hrs.suffix,
            (*info).compressedSize as core::ffi::c_ulonglong,
        );
        if (*info).decompUnavailable == 0 {
            fprintf(
                stdout,
                b"Decompressed Size: %.*f%s (%llu B)\n\0" as *const u8 as *const core::ffi::c_char,
                decompressed_hrs.precision,
                decompressed_hrs.value,
                decompressed_hrs.suffix,
                (*info).decompressedSize as core::ffi::c_ulonglong,
            );
            fprintf(
                stdout,
                b"Ratio: %.4f\n\0" as *const u8 as *const core::ffi::c_char,
                ratio,
            );
        }
        if (*info).usesCheck != 0 && (*info).numActualFrames == 1 {
            fprintf(
                stdout,
                b"Check: %s %02x%02x%02x%02x\n\0" as *const u8 as *const core::ffi::c_char,
                checkString,
                core::ffi::c_int::from(*((*info).checksum).as_ptr().offset(3)),
                core::ffi::c_int::from(*((*info).checksum).as_ptr().offset(2)),
                core::ffi::c_int::from(*((*info).checksum).as_ptr().offset(1)),
                core::ffi::c_int::from(*((*info).checksum).as_ptr().offset(0)),
            );
        } else {
            fprintf(
                stdout,
                b"Check: %s\n\0" as *const u8 as *const core::ffi::c_char,
                checkString,
            );
        }
        fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    };
}
unsafe fn FIO_addFInfo(fi1: fileInfo_t, fi2: fileInfo_t) -> fileInfo_t {
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
    ptr::write_bytes(
        &mut total as *mut fileInfo_t as *mut u8,
        0,
        ::core::mem::size_of::<fileInfo_t>(),
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
unsafe fn FIO_listFile(
    total: *mut fileInfo_t,
    inFileName: *const core::ffi::c_char,
    displayLevel: core::ffi::c_int,
) -> core::ffi::c_int {
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
    ptr::write_bytes(
        &mut info as *mut fileInfo_t as *mut u8,
        0,
        ::core::mem::size_of::<fileInfo_t>(),
    );
    let error = getFileInfo(&mut info, inFileName);
    match error as core::ffi::c_uint {
        1 => {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Error while parsing \"%s\" \n\0" as *const u8 as *const core::ffi::c_char,
                    inFileName,
                );
            }
        }
        2 => {
            fprintf(
                stdout,
                b"File \"%s\" not compressed by zstd \n\0" as *const u8 as *const core::ffi::c_char,
                inFileName,
            );
            if displayLevel > 2 {
                fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
            }
            return 1;
        }
        3 => {
            if displayLevel > 2 {
                fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
            }
            return 1;
        }
        4 => {
            fprintf(
                stdout,
                b"File \"%s\" is truncated \n\0" as *const u8 as *const core::ffi::c_char,
                inFileName,
            );
            if displayLevel > 2 {
                fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
            }
            return 1;
        }
        0 | _ => {}
    }
    displayInfo(inFileName, &info, displayLevel);
    *total = FIO_addFInfo(*total, info);
    assert!(
        error as core::ffi::c_uint == info_success as core::ffi::c_int as core::ffi::c_uint
            || error as core::ffi::c_uint
                == info_frame_error as core::ffi::c_int as core::ffi::c_uint
    );
    error as core::ffi::c_int
}
pub unsafe fn FIO_listMultipleFiles(
    numFiles: core::ffi::c_uint,
    filenameTable: *mut *const core::ffi::c_char,
    displayLevel: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut u: core::ffi::c_uint = 0;
    u = 0;
    while u < numFiles {
        if strcmp(
            *filenameTable.offset(u as isize),
            b"/*stdin*\\\0" as *const u8 as *const core::ffi::c_char,
        ) == 0
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"zstd: --list does not support reading from standard input\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            return 1;
        }
        u = u.wrapping_add(1);
    }
    if numFiles == 0 {
        if UTIL_isConsole(stdin) == 0 && g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: --list does not support reading from standard input \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"No files given \n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        return 1;
    }
    if displayLevel <= 2 {
        fprintf(
            stdout,
            b"Frames  Skips  Compressed  Uncompressed  Ratio  Check  Filename\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    let mut error = 0;
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
    ptr::write_bytes(
        &mut total as *mut fileInfo_t as *mut u8,
        0,
        ::core::mem::size_of::<fileInfo_t>(),
    );
    total.usesCheck = 1;
    let mut u_0: core::ffi::c_uint = 0;
    u_0 = 0;
    while u_0 < numFiles {
        error |= FIO_listFile(
            &mut total,
            *filenameTable.offset(u_0 as isize),
            displayLevel,
        );
        u_0 = u_0.wrapping_add(1);
    }
    if numFiles > 1 && displayLevel <= 2 {
        let compressed_hrs = UTIL_makeHumanReadableSize(total.compressedSize);
        let decompressed_hrs = UTIL_makeHumanReadableSize(total.decompressedSize);
        let ratio = if total.compressedSize == 0 {
            0.0
        } else {
            total.decompressedSize as core::ffi::c_double
                / total.compressedSize as core::ffi::c_double
        };
        let checkString = if total.usesCheck != 0 {
            b"XXH64\0" as *const u8 as *const core::ffi::c_char
        } else {
            b"\0" as *const u8 as *const core::ffi::c_char
        };
        fprintf(
            stdout,
            b"----------------------------------------------------------------- \n\0" as *const u8
                as *const core::ffi::c_char,
        );
        if total.decompUnavailable != 0 {
            fprintf(
                stdout,
                b"%6d  %5d  %6.*f%4s                       %5s  %u files\n\0" as *const u8
                    as *const core::ffi::c_char,
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
                    as *const core::ffi::c_char,
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
