use core::ptr;
use std::ffi::CStr;

use libc::{exit, fprintf, getchar, getenv, size_t, strcmp, strlen, strncmp, strrchr, FILE};
use libzstd_rs::lib::common::zstd_common::{ZSTD_isDeterministicBuild, ZSTD_versionString};
use libzstd_rs::lib::compress::zstd_compress::{
    ZSTD_cParam_getBounds, ZSTD_getCParams, ZSTD_maxCLevel, ZSTD_minCLevel,
};
use libzstd_rs::lib::dictBuilder::cover::ZDICT_cover_params_t;
use libzstd_rs::lib::dictBuilder::fastcover::ZDICT_fastCover_params_t;
use libzstd_rs::lib::dictBuilder::zdict::{ZDICT_legacy_params_t, ZDICT_params_t};
use libzstd_rs::lib::zstd::*;

use crate::benchzstd::{BMK_benchFilesAdvanced, BMK_initAdvancedParams, BMK_syntheticTest};
use crate::dibio::DiB_trainFromFiles;
use crate::fileio::{
    FIO_addAbortHandler, FIO_compressFilename, FIO_compressMultipleFilenames, FIO_createContext,
    FIO_createPreferences, FIO_decompressFilename, FIO_decompressMultipleFilenames,
    FIO_determineHasStdinInput, FIO_displayCompressionParameters, FIO_freeContext,
    FIO_freePreferences, FIO_listMultipleFiles, FIO_lz4Version, FIO_lzmaVersion, FIO_overwriteMode,
    FIO_setAdaptMax, FIO_setAdaptMin, FIO_setAdaptiveMode, FIO_setAllowBlockDevices,
    FIO_setAsyncIOFlag, FIO_setChecksumFlag, FIO_setCompressionType, FIO_setContentSize,
    FIO_setDictIDFlag, FIO_setExcludeCompressedFile, FIO_setHasStdoutOutput, FIO_setJobSize,
    FIO_setLdmBucketSizeLog, FIO_setLdmFlag, FIO_setLdmHashLog, FIO_setLdmHashRateLog,
    FIO_setLdmMinMatch, FIO_setLiteralCompressionMode, FIO_setMMapDict, FIO_setMemLimit,
    FIO_setNbFilesTotal, FIO_setNbWorkers, FIO_setNotificationLevel, FIO_setOverlapLog,
    FIO_setPassThroughFlag, FIO_setPatchFromMode, FIO_setProgressSetting, FIO_setRemoveSrcFile,
    FIO_setRsyncable, FIO_setSparseWrite, FIO_setSrcSizeHint, FIO_setStreamSrcSize,
    FIO_setTargetCBlockSize, FIO_setTestMode, FIO_setUseRowMatchFinder, FIO_zlibVersion,
};
use crate::fileio_asyncio::AIO_supported;
use crate::util::{
    g_utilDisplayLevel, UTIL_allocateFileNamesTable, UTIL_countLogicalCores,
    UTIL_countPhysicalCores, UTIL_createFileNamesTable_fromFileList, UTIL_expandFNT,
    UTIL_fakeStderrIsConsole, UTIL_fakeStdinIsConsole, UTIL_fakeStdoutIsConsole,
    UTIL_freeFileNamesTable, UTIL_getFileSize, UTIL_isConsole, UTIL_isFIFO, UTIL_isLink,
    UTIL_mergeFileNamesTable, UTIL_refFilename, UTIL_searchFileNamesTable, UTIL_traceFileStat,
};
use crate::zstdcli_trace::{TRACE_enable, TRACE_finish};
extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileNamesTable {
    pub fileNames: *mut *const core::ffi::c_char,
    pub buf: *mut core::ffi::c_char,
    pub tableSize: size_t,
    pub tableCapacity: size_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_bounds {
    pub error: size_t,
    pub lowerBound: core::ffi::c_int,
    pub upperBound: core::ffi::c_int,
}
pub type ZSTD_ParamSwitch_e = core::ffi::c_uint;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
pub type FIO_progressSetting_e = core::ffi::c_uint;
pub const FIO_ps_always: FIO_progressSetting_e = 2;
pub const FIO_ps_never: FIO_progressSetting_e = 1;
pub const FIO_ps_auto: FIO_progressSetting_e = 0;
pub type FIO_compressionType_t = core::ffi::c_uint;
pub const FIO_lz4Compression: FIO_compressionType_t = 4;
pub const FIO_lzmaCompression: FIO_compressionType_t = 3;
pub const FIO_xzCompression: FIO_compressionType_t = 2;
pub const FIO_gzipCompression: FIO_compressionType_t = 1;
pub const FIO_zstdCompression: FIO_compressionType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FIO_prefs_s {
    pub compressionType: FIO_compressionType_t,
    pub sparseFileSupport: core::ffi::c_int,
    pub dictIDFlag: core::ffi::c_int,
    pub checksumFlag: core::ffi::c_int,
    pub jobSize: core::ffi::c_int,
    pub overlapLog: core::ffi::c_int,
    pub adaptiveMode: core::ffi::c_int,
    pub useRowMatchFinder: core::ffi::c_int,
    pub rsyncable: core::ffi::c_int,
    pub minAdaptLevel: core::ffi::c_int,
    pub maxAdaptLevel: core::ffi::c_int,
    pub ldmFlag: core::ffi::c_int,
    pub ldmHashLog: core::ffi::c_int,
    pub ldmMinMatch: core::ffi::c_int,
    pub ldmBucketSizeLog: core::ffi::c_int,
    pub ldmHashRateLog: core::ffi::c_int,
    pub streamSrcSize: size_t,
    pub targetCBlockSize: size_t,
    pub srcSizeHint: core::ffi::c_int,
    pub testMode: core::ffi::c_int,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
    pub removeSrcFile: core::ffi::c_int,
    pub overwrite: core::ffi::c_int,
    pub asyncIO: core::ffi::c_int,
    pub memLimit: core::ffi::c_uint,
    pub nbWorkers: core::ffi::c_int,
    pub excludeCompressedFiles: core::ffi::c_int,
    pub patchFromMode: core::ffi::c_int,
    pub contentSize: core::ffi::c_int,
    pub allowBlockDevices: core::ffi::c_int,
    pub passThrough: core::ffi::c_int,
    pub mmapDict: ZSTD_ParamSwitch_e,
}
pub type FIO_prefs_t = FIO_prefs_s;
pub type BMK_mode_t = core::ffi::c_uint;
pub const BMK_compressOnly: BMK_mode_t = 2;
pub const BMK_decodeOnly: BMK_mode_t = 1;
pub const BMK_both: BMK_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_advancedParams_t {
    pub mode: BMK_mode_t,
    pub nbSeconds: core::ffi::c_uint,
    pub chunkSizeMax: size_t,
    pub targetCBlockSize: size_t,
    pub nbWorkers: core::ffi::c_int,
    pub realTime: core::ffi::c_uint,
    pub additionalParam: core::ffi::c_int,
    pub ldmFlag: core::ffi::c_int,
    pub ldmMinMatch: core::ffi::c_int,
    pub ldmHashLog: core::ffi::c_int,
    pub ldmBucketSizeLog: core::ffi::c_int,
    pub ldmHashRateLog: core::ffi::c_int,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
    pub useRowMatchFinder: core::ffi::c_int,
}
pub type dictType = core::ffi::c_uint;
pub const legacy: dictType = 2;
pub const fastCover: dictType = 1;
pub const cover: dictType = 0;
pub type zstd_operation_mode = core::ffi::c_uint;
pub const zom_list: zstd_operation_mode = 5;
pub const zom_train: zstd_operation_mode = 4;
pub const zom_bench: zstd_operation_mode = 3;
pub const zom_test: zstd_operation_mode = 2;
pub const zom_decompress: zstd_operation_mode = 1;
pub const zom_compress: zstd_operation_mode = 0;
pub const UTIL_FILESIZE_UNKNOWN: core::ffi::c_int = -(1);
pub const ZSTD_BLOCKSIZELOG_MAX: core::ffi::c_int = 17;
pub const ZSTD_BLOCKSIZE_MAX: core::ffi::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_WINDOWLOG_MAX_32: core::ffi::c_int = 30;
pub const ZSTD_WINDOWLOG_MAX_64: core::ffi::c_int = 31;
pub const ZSTD_CHAINLOG_MAX_32: core::ffi::c_int = 29;
pub const ZSTD_CHAINLOG_MAX_64: core::ffi::c_int = 30;
pub const ZSTD_MINMATCH_MIN: core::ffi::c_int = 3;
pub const ZSTD_TARGETLENGTH_MAX: core::ffi::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_STRATEGY_MAX: core::ffi::c_int = ZSTD_btultra2 as core::ffi::c_int;
pub const ZSTD_OVERLAPLOG_MAX: core::ffi::c_int = 9;
pub const ZSTD_LDM_BUCKETSIZELOG_MAX: core::ffi::c_int = 8;
pub const stdinmark: [core::ffi::c_char; 10] =
    unsafe { *::core::mem::transmute::<&[u8; 10], &[core::ffi::c_char; 10]>(b"/*stdin*\\\0") };
pub const stdoutmark: [core::ffi::c_char; 11] =
    unsafe { *::core::mem::transmute::<&[u8; 11], &[core::ffi::c_char; 11]>(b"/*stdout*\\\0") };
pub const nulmark: [core::ffi::c_char; 10] =
    unsafe { *::core::mem::transmute::<&[u8; 10], &[core::ffi::c_char; 10]>(b"/dev/null\0") };
pub const LZMA_EXTENSION: [core::ffi::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[core::ffi::c_char; 6]>(b".lzma\0") };
pub const XZ_EXTENSION: [core::ffi::c_char; 4] =
    unsafe { *::core::mem::transmute::<&[u8; 4], &[core::ffi::c_char; 4]>(b".xz\0") };
pub const GZ_EXTENSION: [core::ffi::c_char; 4] =
    unsafe { *::core::mem::transmute::<&[u8; 4], &[core::ffi::c_char; 4]>(b".gz\0") };
pub const ZSTD_EXTENSION: [core::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[core::ffi::c_char; 5]>(b".zst\0") };
pub const LZ4_EXTENSION: [core::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[core::ffi::c_char; 5]>(b".lz4\0") };
pub const NULL: core::ffi::c_int = 0;
pub const ZSTDCLI_CLEVEL_DEFAULT: core::ffi::c_int = 3;
pub const ZSTDCLI_CLEVEL_MAX: core::ffi::c_int = 19;
pub const ZSTD_ZSTDMT: [core::ffi::c_char; 7] =
    unsafe { *::core::mem::transmute::<&[u8; 7], &[core::ffi::c_char; 7]>(b"zstdmt\0") };
pub const ZSTD_UNZSTD: [core::ffi::c_char; 7] =
    unsafe { *::core::mem::transmute::<&[u8; 7], &[core::ffi::c_char; 7]>(b"unzstd\0") };
pub const ZSTD_CAT: [core::ffi::c_char; 8] =
    unsafe { *::core::mem::transmute::<&[u8; 8], &[core::ffi::c_char; 8]>(b"zstdcat\0") };
pub const ZSTD_ZCAT: [core::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[core::ffi::c_char; 5]>(b"zcat\0") };
pub const ZSTD_GZ: [core::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[core::ffi::c_char; 5]>(b"gzip\0") };
pub const ZSTD_GUNZIP: [core::ffi::c_char; 7] =
    unsafe { *::core::mem::transmute::<&[u8; 7], &[core::ffi::c_char; 7]>(b"gunzip\0") };
pub const ZSTD_GZCAT: [core::ffi::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[core::ffi::c_char; 6]>(b"gzcat\0") };
pub const ZSTD_LZMA: [core::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[core::ffi::c_char; 5]>(b"lzma\0") };
pub const ZSTD_UNLZMA: [core::ffi::c_char; 7] =
    unsafe { *::core::mem::transmute::<&[u8; 7], &[core::ffi::c_char; 7]>(b"unlzma\0") };
pub const ZSTD_XZ: [core::ffi::c_char; 3] =
    unsafe { *::core::mem::transmute::<&[u8; 3], &[core::ffi::c_char; 3]>(b"xz\0") };
pub const ZSTD_UNXZ: [core::ffi::c_char; 5] =
    unsafe { *::core::mem::transmute::<&[u8; 5], &[core::ffi::c_char; 5]>(b"unxz\0") };
pub const ZSTD_LZ4: [core::ffi::c_char; 4] =
    unsafe { *::core::mem::transmute::<&[u8; 4], &[core::ffi::c_char; 4]>(b"lz4\0") };
pub const ZSTD_UNLZ4: [core::ffi::c_char; 6] =
    unsafe { *::core::mem::transmute::<&[u8; 6], &[core::ffi::c_char; 6]>(b"unlz4\0") };
pub const DISPLAY_LEVEL_DEFAULT: core::ffi::c_int = 2;
static mut g_defaultDictName: *const core::ffi::c_char =
    b"dictionary\0" as *const u8 as *const core::ffi::c_char;
static g_defaultMaxDictSize: core::ffi::c_uint = (110 * ((1) << 10)) as core::ffi::c_uint;
static g_defaultDictCLevel: core::ffi::c_int = 3;
static g_defaultSelectivityLevel: core::ffi::c_uint = 9;
static g_defaultMaxWindowLog: core::ffi::c_uint = 27;
pub const OVERLAP_LOG_DEFAULT: core::ffi::c_int = 9999;
pub const LDM_PARAM_DEFAULT: core::ffi::c_int = 9999;
static mut g_overlapLog: u32 = OVERLAP_LOG_DEFAULT as u32;
static mut g_ldmHashLog: u32 = 0;
static mut g_ldmMinMatch: u32 = 0;
static mut g_ldmHashRateLog: u32 = LDM_PARAM_DEFAULT as u32;
static mut g_ldmBucketSizeLog: u32 = LDM_PARAM_DEFAULT as u32;
pub const DEFAULT_ACCEL: core::ffi::c_int = 1;
pub const NBWORKERS_AUTOCPU: core::ffi::c_int = 0;
static mut g_displayLevel: core::ffi::c_int = DISPLAY_LEVEL_DEFAULT;
unsafe fn checkLibVersion() {
    if strcmp(
        b"1.5.8\0" as *const u8 as *const core::ffi::c_char,
        ZSTD_versionString(),
    ) != 0
    {
        if g_displayLevel >= 1 {
            fprintf(
                stderr,
                b"Error : incorrect library version (expecting : %s ; actual : %s ) \n\0"
                    as *const u8 as *const core::ffi::c_char,
                b"1.5.8\0" as *const u8 as *const core::ffi::c_char,
                ZSTD_versionString(),
            );
        }
        if g_displayLevel >= 1 {
            fprintf(
                stderr,
                b"Please update library to version %s, or use stand-alone zstd binary \n\0"
                    as *const u8 as *const core::ffi::c_char,
                b"1.5.8\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
}
unsafe fn exeNameMatch(
    mut exeName: *const core::ffi::c_char,
    mut test: *const core::ffi::c_char,
) -> core::ffi::c_int {
    (strncmp(exeName, test, strlen(test)) == 0
        && (*exeName.add(strlen(test)) as core::ffi::c_int == '\0' as i32
            || *exeName.add(strlen(test)) as core::ffi::c_int == '.' as i32))
        as core::ffi::c_int
}
unsafe fn usage(mut f: *mut FILE, mut programName: *const core::ffi::c_char) {
    fprintf(
        f,
        b"Compress or decompress the INPUT file(s); reads from STDIN if INPUT is `-` or not provided.\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"Usage: %s [OPTIONS...] [INPUT... | -] [-o OUTPUT]\n\n\0" as *const u8
            as *const core::ffi::c_char,
        programName,
    );
    fprintf(f, b"Options:\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        f,
        b"  -o OUTPUT                     Write output to a single file, OUTPUT.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"  -k, --keep                    Preserve INPUT file(s). [Default] \n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"  --rm                          Remove INPUT file(s) after successful (de)compression to file.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    if exeNameMatch(programName, ZSTD_GZ.as_ptr()) != 0 {
        fprintf(
            f,
            b"  -n, --no-name                 Do not store original filename when compressing.\n\n\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    fprintf(f, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        f,
        b"  -#                            Desired compression level, where `#` is a number between 1 and %d;\n\0"
            as *const u8 as *const core::ffi::c_char,
        19,
    );
    fprintf(
        f,
        b"                                lower numbers provide faster compression, higher numbers yield\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"                                better compression ratios. [Default: %d]\n\n\0"
            as *const u8 as *const core::ffi::c_char,
        3,
    );
    fprintf(
        f,
        b"  -d, --decompress              Perform decompression.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"  -D DICT                       Use DICT as the dictionary for compression or decompression.\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"  -f, --force                   Disable input and output checks. Allows overwriting existing files,\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"                                receiving input from the console, printing output to STDOUT, and\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"                                operating on links, block devices, etc. Unrecognized formats will be\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"                                passed-through through as-is.\n\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"  -h                            Display short usage and exit.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"  -H, --help                    Display full help and exit.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        f,
        b"  -V, --version                 Display the program version and exit.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(f, b"\n\0" as *const u8 as *const core::ffi::c_char);
}
unsafe fn usageAdvanced(mut programName: *const core::ffi::c_char) {
    fprintf(
        stdout,
        b"*** %s (%i-bit) %s, by %s ***\n\0" as *const u8 as *const core::ffi::c_char,
        b"Zstandard CLI\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::size_of::<size_t>()).wrapping_mul(8) as core::ffi::c_int,
        b"v1.5.8\0" as *const u8 as *const core::ffi::c_char,
        b"Yann Collet\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    usage(stdout, programName);
    fprintf(
        stdout,
        b"Advanced options:\n\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -c, --stdout                  Write to STDOUT (even if it is a console) and keep the INPUT file(s).\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -v, --verbose                 Enable verbose output; pass multiple times to increase verbosity.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -q, --quiet                   Suppress warnings; pass twice to suppress errors.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --trace LOG                   Log tracing information to LOG.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"  --[no-]progress               Forcibly show/hide the progress counter. NOTE: Any (de)compressed\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"                                output to terminal will mix with progress counter text.\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -r                            Operate recursively on directories.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --filelist LIST               Read a list of files to operate on from LIST.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --output-dir-flat DIR         Store processed files in DIR.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --output-dir-mirror DIR       Store processed files in DIR, respecting original directory structure.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    if AIO_supported() != 0 {
        fprintf(
            stdout,
            b"  --[no-]asyncio                Use asynchronous IO. [Default: Enabled]\n\0"
                as *const u8 as *const core::ffi::c_char,
        );
    }
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"  --[no-]check                  Add XXH64 integrity checksums during compression. [Default: Add, Validate]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"                                If `-d` is present, ignore/validate checksums during decompression.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"  --                            Treat remaining arguments after `--` as files.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"Advanced compression options:\n\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --ultra                       Enable levels beyond %i, up to %i; requires more memory.\n\0"
            as *const u8 as *const core::ffi::c_char,
        19,
        ZSTD_maxCLevel(),
    );
    fprintf(
        stdout,
        b"  --fast[=#]                    Use to very fast compression levels. [Default: %u]\n\0"
            as *const u8 as *const core::ffi::c_char,
        1,
    );
    if exeNameMatch(programName, ZSTD_GZ.as_ptr()) != 0 {
        fprintf(
            stdout,
            b"  --best                        Compatibility alias for `-9`.\n\0" as *const u8
                as *const core::ffi::c_char,
        );
    }
    fprintf(
        stdout,
        b"  --adapt                       Dynamically adapt compression level to I/O conditions.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --long[=#]                    Enable long distance matching with window log #. [Default: %u]\n\0"
            as *const u8 as *const core::ffi::c_char,
        g_defaultMaxWindowLog,
    );
    fprintf(
        stdout,
        b"  --patch-from=REF              Use REF as the reference point for Zstandard's diff engine. \n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --patch-apply                 Equivalent for `-d --patch-from` \n\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -T#                           Spawn # compression threads. [Default: 1; pass 0 for core count.]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --single-thread               Share a single thread for I/O and compression (slightly different than `-T1`).\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --auto-threads={physical|logical}\n\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"                                Use physical/logical cores when using `-T0`. [Default: Physical]\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --jobsize=#                   Set job size to #. [Default: 0 (automatic)]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --rsyncable                   Compress using a rsync-friendly method (`--jobsize=#` sets unit size). \n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"  --exclude-compressed          Only compress files that are not already compressed.\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --stream-size=#               Specify size of streaming input from STDIN.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --size-hint=#                 Optimize compression parameters for streaming input of approximately size #.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --target-compressed-block-size=#\n\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"                                Generate compressed blocks of approximately # size.\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --no-dictID                   Don't write `dictID` into the header (dictionary compression only).\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]compress-literals      Force (un)compressed literals.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]row-match-finder       Explicitly enable/disable the fast, row-based matchfinder for\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"                                the 'greedy', 'lazy', and 'lazy2' strategies.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"  --format=zstd                 Compress files to the `.zst` format. [Default]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]mmap-dict              Memory-map dictionary file rather than mallocing and loading all at once\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --format=gzip                 Compress files to the `.gz` format.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --format=xz                   Compress files to the `.xz` format.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --format=lzma                 Compress files to the `.lzma` format.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"Advanced decompression options:\n\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -l                            Print information about Zstandard-compressed files.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --test                        Test compressed file integrity.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -M#                           Set the memory usage limit to # megabytes.\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --[no-]sparse                 Enable sparse mode. [Default: Enabled for files, disabled for STDOUT.]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    let mut passThroughDefault = b"Disabled\0" as *const u8 as *const core::ffi::c_char;
    if exeNameMatch(programName, ZSTD_CAT.as_ptr()) != 0
        || exeNameMatch(programName, ZSTD_ZCAT.as_ptr()) != 0
        || exeNameMatch(programName, ZSTD_GZCAT.as_ptr()) != 0
    {
        passThroughDefault = b"Enabled\0" as *const u8 as *const core::ffi::c_char;
    }
    fprintf(
        stdout,
        b"  --[no-]pass-through           Pass through uncompressed files as-is. [Default: %s]\n\0"
            as *const u8 as *const core::ffi::c_char,
        passThroughDefault,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"Dictionary builder:\n\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --train                       Create a dictionary from a training set of files.\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --train-cover[=k=#,d=#,steps=#,split=#,shrink[=#]]\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"                                Use the cover algorithm (with optional arguments).\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --train-fastcover[=k=#,d=#,f=#,steps=#,split=#,accel=#,shrink[=#]]\n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"                                Use the fast cover algorithm (with optional arguments).\n\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --train-legacy[=s=#]          Use the legacy algorithm with selectivity #. [Default: %u]\n\0"
            as *const u8 as *const core::ffi::c_char,
        g_defaultSelectivityLevel,
    );
    fprintf(
        stdout,
        b"  -o NAME                       Use NAME as dictionary name. [Default: %s]\n\0"
            as *const u8 as *const core::ffi::c_char,
        g_defaultDictName,
    );
    fprintf(
        stdout,
        b"  --maxdict=#                   Limit dictionary to specified size #. [Default: %u]\n\0"
            as *const u8 as *const core::ffi::c_char,
        g_defaultMaxDictSize,
    );
    fprintf(
        stdout,
        b"  --dictID=#                    Force dictionary ID to #. [Default: Random]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
    fprintf(
        stdout,
        b"Benchmark options:\n\0" as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -b#                           Perform benchmarking with compression level #. [Default: %d]\n\0"
            as *const u8 as *const core::ffi::c_char,
        3,
    );
    fprintf(
        stdout,
        b"  -e#                           Test all compression levels up to #; starting level is `-b#`. [Default: 1]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -i#                           Set the minimum evaluation to time # seconds. [Default: 3]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --split=#                     Split input into independent chunks of size #. [Default: No chunking]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -S                            Output one benchmark result per input file. [Default: Consolidated result]\n\0"
            as *const u8 as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  -D dictionary                 Benchmark using dictionary \n\0" as *const u8
            as *const core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"  --priority=rt                 Set process priority to real-time.\n\0" as *const u8
            as *const core::ffi::c_char,
    );
}
unsafe fn badUsage(
    mut programName: *const core::ffi::c_char,
    mut parameter: *const core::ffi::c_char,
) {
    if g_displayLevel >= 1 {
        fprintf(
            stderr,
            b"Incorrect parameter: %s \n\0" as *const u8 as *const core::ffi::c_char,
            parameter,
        );
    }
    if g_displayLevel >= 2 {
        usage(stderr, programName);
    }
}
unsafe fn waitEnter() {
    fprintf(
        stderr,
        b"Press enter to continue... \n\0" as *const u8 as *const core::ffi::c_char,
    );
    getchar();
}
unsafe fn lastNameFromPath(mut path: *const core::ffi::c_char) -> *const core::ffi::c_char {
    let mut name = path;
    if !(strrchr(name, '/' as i32)).is_null() {
        name = (strrchr(name, '/' as i32)).offset(1);
    }
    if !(strrchr(name, '\\' as i32)).is_null() {
        name = (strrchr(name, '\\' as i32)).offset(1);
    }
    name
}
unsafe fn errorOut(mut msg: *const core::ffi::c_char) {
    if g_displayLevel >= 1 {
        fprintf(
            stderr,
            b"%s \n\0" as *const u8 as *const core::ffi::c_char,
            msg,
        );
    }
    exit(1);
}
unsafe fn readU32FromCharChecked(
    mut stringPtr: *mut *const core::ffi::c_char,
    mut value: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut result = 0;
    while **stringPtr as core::ffi::c_int >= '0' as i32
        && **stringPtr as core::ffi::c_int <= '9' as i32
    {
        let max = (-(1 as core::ffi::c_int) as core::ffi::c_uint).wrapping_div(10);
        let mut last = result;
        if result > max {
            return 1;
        }
        result = result.wrapping_mul(10);
        result = result
            .wrapping_add((**stringPtr as core::ffi::c_int - '0' as i32) as core::ffi::c_uint);
        if result < last {
            return 1;
        }
        *stringPtr = (*stringPtr).offset(1);
    }
    if **stringPtr as core::ffi::c_int == 'K' as i32
        || **stringPtr as core::ffi::c_int == 'M' as i32
    {
        let maxK = -(1 as core::ffi::c_int) as core::ffi::c_uint >> 10;
        if result > maxK {
            return 1;
        }
        result <<= 10;
        if **stringPtr as core::ffi::c_int == 'M' as i32 {
            if result > maxK {
                return 1;
            }
            result <<= 10;
        }
        *stringPtr = (*stringPtr).offset(1);
        if **stringPtr as core::ffi::c_int == 'i' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
        if **stringPtr as core::ffi::c_int == 'B' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
    }
    *value = result;
    0
}
unsafe fn readU32FromChar(mut stringPtr: *mut *const core::ffi::c_char) -> core::ffi::c_uint {
    let mut result: core::ffi::c_uint = 0;
    if readU32FromCharChecked(stringPtr, &mut result) != 0 {
        errorOut(c"error: numeric value overflows 32-bit unsigned int".as_ptr());
    }
    result
}
unsafe fn readIntFromChar(mut stringPtr: *mut *const core::ffi::c_char) -> core::ffi::c_int {
    let mut sign = 1;
    let mut result: core::ffi::c_uint = 0;
    if **stringPtr as core::ffi::c_int == '-' as i32 {
        *stringPtr = (*stringPtr).offset(1);
        sign = -(1);
    }
    if readU32FromCharChecked(stringPtr, &mut result) != 0 {
        errorOut(c"error: numeric value overflows 32-bit int".as_ptr());
    }
    result as core::ffi::c_int * sign
}
unsafe fn readSizeTFromCharChecked(
    mut stringPtr: *mut *const core::ffi::c_char,
    mut value: *mut size_t,
) -> core::ffi::c_int {
    let mut result = 0;
    while **stringPtr as core::ffi::c_int >= '0' as i32
        && **stringPtr as core::ffi::c_int <= '9' as i32
    {
        let max = -(1 as core::ffi::c_int) as size_t / 10;
        let mut last = result;
        if result > max {
            return 1;
        }
        result *= 10;
        result = result.wrapping_add((**stringPtr as core::ffi::c_int - '0' as i32) as size_t);
        if result < last {
            return 1;
        }
        *stringPtr = (*stringPtr).offset(1);
    }
    if **stringPtr as core::ffi::c_int == 'K' as i32
        || **stringPtr as core::ffi::c_int == 'M' as i32
    {
        let maxK = -(1 as core::ffi::c_int) as size_t >> 10;
        if result > maxK {
            return 1;
        }
        result <<= 10;
        if **stringPtr as core::ffi::c_int == 'M' as i32 {
            if result > maxK {
                return 1;
            }
            result <<= 10;
        }
        *stringPtr = (*stringPtr).offset(1);
        if **stringPtr as core::ffi::c_int == 'i' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
        if **stringPtr as core::ffi::c_int == 'B' as i32 {
            *stringPtr = (*stringPtr).offset(1);
        }
    }
    *value = result;
    0
}
unsafe fn readSizeTFromChar(mut stringPtr: *mut *const core::ffi::c_char) -> size_t {
    let mut result: size_t = 0;
    if readSizeTFromCharChecked(stringPtr, &mut result) != 0 {
        errorOut(c"error: numeric value overflows size_t".as_ptr());
    }
    result
}
unsafe fn longCommandWArg(
    mut stringPtr: *mut *const core::ffi::c_char,
    mut longCommand: *const core::ffi::c_char,
) -> core::ffi::c_int {
    let comSize = strlen(longCommand);
    let result = (strncmp(*stringPtr, longCommand, comSize) == 0) as core::ffi::c_int;
    if result != 0 {
        *stringPtr = (*stringPtr).add(comSize);
    }
    result
}
static mut kDefaultRegression: core::ffi::c_uint = 1;
unsafe fn parseCoverParameters(
    mut stringPtr: *const core::ffi::c_char,
    mut params: *mut ZDICT_cover_params_t,
) -> core::ffi::c_uint {
    ptr::write_bytes(
        params as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
    loop {
        if longCommandWArg(
            &mut stringPtr,
            b"k=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).k = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"d=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).d = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"steps=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).steps = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"split=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            let mut splitPercentage = readU32FromChar(&mut stringPtr);
            (*params).splitPoint = splitPercentage as core::ffi::c_double / 100.0f64;
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"shrink\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).shrinkDictMaxRegression = kDefaultRegression;
            (*params).shrinkDict = 1;
            if *stringPtr.offset(0) as core::ffi::c_int == '=' as i32 {
                stringPtr = stringPtr.offset(1);
                (*params).shrinkDictMaxRegression = readU32FromChar(&mut stringPtr);
            }
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else {
            return 0;
        }
    }
    if *stringPtr.offset(0) as core::ffi::c_int != 0 {
        return 0;
    }
    if g_displayLevel >= 4 {
        fprintf(
            stderr,
            b"cover: k=%u\nd=%u\nsteps=%u\nsplit=%u\nshrink%u\n\0" as *const u8
                as *const core::ffi::c_char,
            (*params).k,
            (*params).d,
            (*params).steps,
            ((*params).splitPoint * 100.0) as core::ffi::c_uint,
            (*params).shrinkDictMaxRegression,
        );
    }
    1
}
unsafe fn parseFastCoverParameters(
    mut stringPtr: *const core::ffi::c_char,
    mut params: *mut ZDICT_fastCover_params_t,
) -> core::ffi::c_uint {
    ptr::write_bytes(
        params as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_fastCover_params_t>(),
    );
    loop {
        if longCommandWArg(
            &mut stringPtr,
            b"k=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).k = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"d=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).d = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"f=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).f = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"steps=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).steps = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"accel=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).accel = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"split=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            let mut splitPercentage = readU32FromChar(&mut stringPtr);
            (*params).splitPoint = splitPercentage as core::ffi::c_double / 100.0f64;
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"shrink\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            (*params).shrinkDictMaxRegression = kDefaultRegression;
            (*params).shrinkDict = 1;
            if *stringPtr.offset(0) as core::ffi::c_int == '=' as i32 {
                stringPtr = stringPtr.offset(1);
                (*params).shrinkDictMaxRegression = readU32FromChar(&mut stringPtr);
            }
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else {
            return 0;
        }
    }
    if *stringPtr.offset(0) as core::ffi::c_int != 0 {
        return 0;
    }
    if g_displayLevel >= 4 {
        fprintf(
            stderr,
            b"cover: k=%u\nd=%u\nf=%u\nsteps=%u\nsplit=%u\naccel=%u\nshrink=%u\n\0" as *const u8
                as *const core::ffi::c_char,
            (*params).k,
            (*params).d,
            (*params).f,
            (*params).steps,
            ((*params).splitPoint * 100.0) as core::ffi::c_uint,
            (*params).accel,
            (*params).shrinkDictMaxRegression,
        );
    }
    1
}
unsafe fn parseLegacyParameters(
    mut stringPtr: *const core::ffi::c_char,
    mut selectivity: *mut core::ffi::c_uint,
) -> core::ffi::c_uint {
    if longCommandWArg(
        &mut stringPtr,
        b"s=\0" as *const u8 as *const core::ffi::c_char,
    ) == 0
        && longCommandWArg(
            &mut stringPtr,
            b"selectivity=\0" as *const u8 as *const core::ffi::c_char,
        ) == 0
    {
        return 0;
    }
    *selectivity = readU32FromChar(&mut stringPtr);
    if *stringPtr.offset(0) as core::ffi::c_int != 0 {
        return 0;
    }
    if g_displayLevel >= 4 {
        fprintf(
            stderr,
            b"legacy: selectivity=%u\n\0" as *const u8 as *const core::ffi::c_char,
            *selectivity,
        );
    }
    1
}
unsafe fn defaultCoverParams() -> ZDICT_cover_params_t {
    let mut params = ZDICT_cover_params_t {
        k: 0,
        d: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    ptr::write_bytes(
        &mut params as *mut ZDICT_cover_params_t as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
    params.d = 8;
    params.steps = 4;
    params.splitPoint = 1.0f64;
    params.shrinkDict = 0;
    params.shrinkDictMaxRegression = kDefaultRegression;
    params
}
unsafe fn defaultFastCoverParams() -> ZDICT_fastCover_params_t {
    let mut params = ZDICT_fastCover_params_t {
        k: 0,
        d: 0,
        f: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        accel: 0,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    ptr::write_bytes(
        &mut params as *mut ZDICT_fastCover_params_t as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_fastCover_params_t>(),
    );
    params.d = 8;
    params.f = 20;
    params.steps = 4;
    params.splitPoint = 0.75f64;
    params.accel = DEFAULT_ACCEL as core::ffi::c_uint;
    params.shrinkDict = 0;
    params.shrinkDictMaxRegression = kDefaultRegression;
    params
}
unsafe fn parseAdaptParameters(
    mut stringPtr: *const core::ffi::c_char,
    mut adaptMinPtr: *mut core::ffi::c_int,
    mut adaptMaxPtr: *mut core::ffi::c_int,
) -> core::ffi::c_uint {
    loop {
        if longCommandWArg(
            &mut stringPtr,
            b"min=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            *adaptMinPtr = readIntFromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"max=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
        {
            *adaptMaxPtr = readIntFromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else {
            if g_displayLevel >= 4 {
                fprintf(
                    stderr,
                    b"invalid compression parameter \n\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            return 0;
        }
    }
    if *stringPtr.offset(0) as core::ffi::c_int != 0 {
        return 0;
    }
    if *adaptMinPtr > *adaptMaxPtr {
        if g_displayLevel >= 4 {
            fprintf(
                stderr,
                b"incoherent adaptation limits \n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        return 0;
    }
    1
}
unsafe fn parseCompressionParameters(
    mut stringPtr: *const core::ffi::c_char,
    mut params: *mut ZSTD_compressionParameters,
) -> core::ffi::c_uint {
    loop {
        if longCommandWArg(
            &mut stringPtr,
            b"windowLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"wlog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            (*params).windowLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"chainLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"clog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            (*params).chainLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"hashLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"hlog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            (*params).hashLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"searchLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"slog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            (*params).searchLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"minMatch=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"mml=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            (*params).minMatch = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"targetLength=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"tlen=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            (*params).targetLength = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"strategy=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"strat=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            (*params).strategy = readU32FromChar(&mut stringPtr) as ZSTD_strategy;
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"overlapLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"ovlog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            g_overlapLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmHashLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lhlog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            g_ldmHashLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmMinMatch=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lmml=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            g_ldmMinMatch = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmBucketSizeLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lblog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            g_ldmBucketSizeLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else if longCommandWArg(
            &mut stringPtr,
            b"ldmHashRateLog=\0" as *const u8 as *const core::ffi::c_char,
        ) != 0
            || longCommandWArg(
                &mut stringPtr,
                b"lhrlog=\0" as *const u8 as *const core::ffi::c_char,
            ) != 0
        {
            g_ldmHashRateLog = readU32FromChar(&mut stringPtr);
            if *stringPtr.offset(0) as core::ffi::c_int != ',' as i32 {
                break;
            }
            stringPtr = stringPtr.offset(1);
        } else {
            if g_displayLevel >= 4 {
                fprintf(
                    stderr,
                    b"invalid compression parameter \n\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            return 0;
        }
    }
    if *stringPtr.offset(0) as core::ffi::c_int != 0 {
        return 0;
    }
    1
}
unsafe fn setMaxCompression(mut params: *mut ZSTD_compressionParameters) {
    (*params).windowLog = (if ::core::mem::size_of::<size_t>() == 4 {
        ZSTD_WINDOWLOG_MAX_32
    } else {
        ZSTD_WINDOWLOG_MAX_64
    }) as core::ffi::c_uint;
    (*params).chainLog = (if ::core::mem::size_of::<size_t>() == 4 {
        ZSTD_CHAINLOG_MAX_32
    } else {
        ZSTD_CHAINLOG_MAX_64
    }) as core::ffi::c_uint;
    (*params).hashLog = (if (if ::core::mem::size_of::<size_t>() == 4 {
        ZSTD_WINDOWLOG_MAX_32
    } else {
        ZSTD_WINDOWLOG_MAX_64
    }) < 30
    {
        if ::core::mem::size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }
    } else {
        30
    }) as core::ffi::c_uint;
    (*params).searchLog = ((if ::core::mem::size_of::<size_t>() == 4 {
        ZSTD_WINDOWLOG_MAX_32
    } else {
        ZSTD_WINDOWLOG_MAX_64
    }) - 1) as core::ffi::c_uint;
    (*params).minMatch = ZSTD_MINMATCH_MIN as core::ffi::c_uint;
    (*params).targetLength = ZSTD_TARGETLENGTH_MAX as core::ffi::c_uint;
    (*params).strategy = ZSTD_STRATEGY_MAX as ZSTD_strategy;
    g_overlapLog = ZSTD_OVERLAPLOG_MAX as u32;
    g_ldmHashLog = (if (if ::core::mem::size_of::<size_t>() == 4 {
        ZSTD_WINDOWLOG_MAX_32
    } else {
        ZSTD_WINDOWLOG_MAX_64
    }) < 30
    {
        if ::core::mem::size_of::<size_t>() == 4 {
            ZSTD_WINDOWLOG_MAX_32
        } else {
            ZSTD_WINDOWLOG_MAX_64
        }
    } else {
        30
    }) as u32;
    g_ldmHashRateLog = 0;
    g_ldmMinMatch = 16;
    g_ldmBucketSizeLog = ZSTD_LDM_BUCKETSIZELOG_MAX as u32;
}
unsafe fn printVersion() {
    if g_displayLevel < DISPLAY_LEVEL_DEFAULT {
        fprintf(
            stdout,
            b"%s\n\0" as *const u8 as *const core::ffi::c_char,
            b"1.5.8\0" as *const u8 as *const core::ffi::c_char,
        );
        return;
    }
    fprintf(
        stdout,
        b"*** %s (%i-bit) %s, by %s ***\n\0" as *const u8 as *const core::ffi::c_char,
        b"Zstandard CLI\0" as *const u8 as *const core::ffi::c_char,
        (::core::mem::size_of::<size_t>()).wrapping_mul(8) as core::ffi::c_int,
        b"v1.5.8\0" as *const u8 as *const core::ffi::c_char,
        b"Yann Collet\0" as *const u8 as *const core::ffi::c_char,
    );
    if g_displayLevel >= 3 {
        fprintf(
            stdout,
            b"*** supports: zstd\0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(
            stdout,
            b", zstd legacy v0.%d+\0" as *const u8 as *const core::ffi::c_char,
            5,
        );
        fprintf(stdout, b", gzip\0" as *const u8 as *const core::ffi::c_char);
        fprintf(
            stdout,
            b", lzma, xz \0" as *const u8 as *const core::ffi::c_char,
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const core::ffi::c_char);
        if g_displayLevel >= 4 {
            fprintf(
                stdout,
                b"zlib version %s\n\0" as *const u8 as *const core::ffi::c_char,
                FIO_zlibVersion(),
            );
            fprintf(
                stdout,
                b"lz4 version %s\n\0" as *const u8 as *const core::ffi::c_char,
                FIO_lz4Version(),
            );
            fprintf(
                stdout,
                b"lzma version %s\n\0" as *const u8 as *const core::ffi::c_char,
                FIO_lzmaVersion(),
            );
            fprintf(
                stdout,
                b"supports Multithreading \n\0" as *const u8 as *const core::ffi::c_char,
            );
            fprintf(
                stdout,
                b"_POSIX_C_SOURCE defined: %ldL\n\0" as *const u8 as *const core::ffi::c_char,
                200809,
            );
            fprintf(
                stdout,
                b"_POSIX_VERSION defined: %ldL \n\0" as *const u8 as *const core::ffi::c_char,
                200809,
            );
            fprintf(
                stdout,
                b"PLATFORM_POSIX_VERSION defined: %ldL\n\0" as *const u8
                    as *const core::ffi::c_char,
                200809,
            );
            if ZSTD_isDeterministicBuild() == 0 {
                fprintf(
                    stdout,
                    b"non-deterministic build\n\0" as *const u8 as *const core::ffi::c_char,
                );
            }
        }
    }
}
const ZSTD_NB_STRATEGIES: usize = 9;
static ZSTD_strategyMap: [&CStr; ZSTD_NB_STRATEGIES + 1] = [
    c"",
    c"ZSTD_fast",
    c"ZSTD_dfast",
    c"ZSTD_greedy",
    c"ZSTD_lazy",
    c"ZSTD_lazy2",
    c"ZSTD_btlazy2",
    c"ZSTD_btopt",
    c"ZSTD_btultra",
    c"ZSTD_btultra2",
];
unsafe fn printDefaultCParams(
    mut filename: *const core::ffi::c_char,
    mut dictFileName: *const core::ffi::c_char,
    mut cLevel: core::ffi::c_int,
) {
    let mut fileSize = UTIL_getFileSize(filename) as core::ffi::c_ulonglong;
    let dictSize = if !dictFileName.is_null() {
        UTIL_getFileSize(dictFileName) as size_t
    } else {
        0
    };
    let cParams = ZSTD_getCParams(cLevel, fileSize, dictSize);
    if fileSize != UTIL_FILESIZE_UNKNOWN as core::ffi::c_ulonglong {
        fprintf(
            stderr,
            b"%s (%llu bytes)\n\0" as *const u8 as *const core::ffi::c_char,
            filename,
            fileSize,
        );
    } else {
        fprintf(
            stderr,
            b"%s (src size unknown)\n\0" as *const u8 as *const core::ffi::c_char,
            filename,
        );
    }
    fprintf(
        stderr,
        b" - windowLog     : %u\n\0" as *const u8 as *const core::ffi::c_char,
        cParams.windowLog,
    );
    fprintf(
        stderr,
        b" - chainLog      : %u\n\0" as *const u8 as *const core::ffi::c_char,
        cParams.chainLog,
    );
    fprintf(
        stderr,
        b" - hashLog       : %u\n\0" as *const u8 as *const core::ffi::c_char,
        cParams.hashLog,
    );
    fprintf(
        stderr,
        b" - searchLog     : %u\n\0" as *const u8 as *const core::ffi::c_char,
        cParams.searchLog,
    );
    fprintf(
        stderr,
        b" - minMatch      : %u\n\0" as *const u8 as *const core::ffi::c_char,
        cParams.minMatch,
    );
    fprintf(
        stderr,
        b" - targetLength  : %u\n\0" as *const u8 as *const core::ffi::c_char,
        cParams.targetLength,
    );
    assert!(
        (cParams.strategy as core::ffi::c_uint) < (ZSTD_NB_STRATEGIES + 1) as core::ffi::c_uint
    );
    fprintf(
        stderr,
        b" - strategy      : %s (%u)\n\0" as *const u8 as *const core::ffi::c_char,
        ZSTD_strategyMap[cParams.strategy as usize].as_ptr(),
        cParams.strategy as core::ffi::c_uint,
    );
}
unsafe fn printActualCParams(
    mut filename: *const core::ffi::c_char,
    mut dictFileName: *const core::ffi::c_char,
    mut cLevel: core::ffi::c_int,
    mut cParams: *const ZSTD_compressionParameters,
) {
    let mut fileSize = UTIL_getFileSize(filename) as core::ffi::c_ulonglong;
    let dictSize = if !dictFileName.is_null() {
        UTIL_getFileSize(dictFileName) as size_t
    } else {
        0
    };
    let mut actualCParams = ZSTD_getCParams(cLevel, fileSize, dictSize);
    assert!(g_displayLevel >= 4);
    actualCParams.windowLog = if (*cParams).windowLog == 0 {
        actualCParams.windowLog
    } else {
        (*cParams).windowLog
    };
    actualCParams.chainLog = if (*cParams).chainLog == 0 {
        actualCParams.chainLog
    } else {
        (*cParams).chainLog
    };
    actualCParams.hashLog = if (*cParams).hashLog == 0 {
        actualCParams.hashLog
    } else {
        (*cParams).hashLog
    };
    actualCParams.searchLog = if (*cParams).searchLog == 0 {
        actualCParams.searchLog
    } else {
        (*cParams).searchLog
    };
    actualCParams.minMatch = if (*cParams).minMatch == 0 {
        actualCParams.minMatch
    } else {
        (*cParams).minMatch
    };
    actualCParams.targetLength = if (*cParams).targetLength == 0 {
        actualCParams.targetLength
    } else {
        (*cParams).targetLength
    };
    actualCParams.strategy = (if (*cParams).strategy as core::ffi::c_uint == 0 {
        actualCParams.strategy as core::ffi::c_uint
    } else {
        (*cParams).strategy as core::ffi::c_uint
    }) as ZSTD_strategy;
    fprintf(
        stderr,
        b"--zstd=wlog=%d,clog=%d,hlog=%d,slog=%d,mml=%d,tlen=%d,strat=%d\n\0" as *const u8
            as *const core::ffi::c_char,
        actualCParams.windowLog,
        actualCParams.chainLog,
        actualCParams.hashLog,
        actualCParams.searchLog,
        actualCParams.minMatch,
        actualCParams.targetLength,
        actualCParams.strategy as core::ffi::c_uint,
    );
}
pub const ENV_CLEVEL: [core::ffi::c_char; 12] =
    unsafe { *::core::mem::transmute::<&[u8; 12], &[core::ffi::c_char; 12]>(b"ZSTD_CLEVEL\0") };
pub const ENV_NBWORKERS: [core::ffi::c_char; 15] =
    unsafe { *::core::mem::transmute::<&[u8; 15], &[core::ffi::c_char; 15]>(b"ZSTD_NBTHREADS\0") };
unsafe fn init_cLevel() -> core::ffi::c_int {
    let env: *const core::ffi::c_char = getenv(ENV_CLEVEL.as_ptr());
    if !env.is_null() {
        let mut ptr = env;
        let mut sign = 1;
        if *ptr as core::ffi::c_int == '-' as i32 {
            sign = -(1);
            ptr = ptr.offset(1);
        } else if *ptr as core::ffi::c_int == '+' as i32 {
            ptr = ptr.offset(1);
        }
        if *ptr as core::ffi::c_int >= '0' as i32 && *ptr as core::ffi::c_int <= '9' as i32 {
            let mut absLevel: core::ffi::c_uint = 0;
            if readU32FromCharChecked(&mut ptr, &mut absLevel) != 0 {
                if g_displayLevel >= 2 {
                    fprintf(
                        stderr,
                        b"Ignore environment variable setting %s=%s: numeric value too large \n\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"ZSTD_CLEVEL\0" as *const u8 as *const core::ffi::c_char,
                        env,
                    );
                }
                return ZSTDCLI_CLEVEL_DEFAULT;
            } else if *ptr as core::ffi::c_int == 0 {
                return sign * absLevel as core::ffi::c_int;
            }
        }
        if g_displayLevel >= 2 {
            fprintf(
                stderr,
                b"Ignore environment variable setting %s=%s: not a valid integer value \n\0"
                    as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CLEVEL\0" as *const u8 as *const core::ffi::c_char,
                env,
            );
        }
    }
    ZSTDCLI_CLEVEL_DEFAULT
}
unsafe fn init_nbWorkers() -> core::ffi::c_uint {
    let env: *const core::ffi::c_char = getenv(ENV_NBWORKERS.as_ptr());
    if !env.is_null() {
        let mut ptr = env;
        if *ptr as core::ffi::c_int >= '0' as i32 && *ptr as core::ffi::c_int <= '9' as i32 {
            let mut nbThreads: core::ffi::c_uint = 0;
            if readU32FromCharChecked(&mut ptr, &mut nbThreads) != 0 {
                if g_displayLevel >= 2 {
                    fprintf(
                        stderr,
                        b"Ignore environment variable setting %s=%s: numeric value too large \n\0"
                            as *const u8 as *const core::ffi::c_char,
                        b"ZSTD_NBTHREADS\0" as *const u8 as *const core::ffi::c_char,
                        env,
                    );
                }
                return (if 1
                    > (if (4) < UTIL_countLogicalCores() / 4 {
                        4
                    } else {
                        UTIL_countLogicalCores() / 4
                    }) {
                    1
                } else if (4) < UTIL_countLogicalCores() / 4 {
                    4
                } else {
                    UTIL_countLogicalCores() / 4
                }) as core::ffi::c_uint;
            } else if *ptr as core::ffi::c_int == 0 {
                return nbThreads;
            }
        }
        if g_displayLevel >= 2 {
            fprintf(
                stderr,
                b"Ignore environment variable setting %s=%s: not a valid unsigned value \n\0"
                    as *const u8 as *const core::ffi::c_char,
                b"ZSTD_NBTHREADS\0" as *const u8 as *const core::ffi::c_char,
                env,
            );
        }
    }
    (if 1
        > (if (4) < UTIL_countLogicalCores() / 4 {
            4
        } else {
            UTIL_countLogicalCores() / 4
        })
    {
        1
    } else if (4) < UTIL_countLogicalCores() / 4 {
        4
    } else {
        UTIL_countLogicalCores() / 4
    }) as core::ffi::c_uint
}
pub const MINCLEVEL: core::ffi::c_int = ZSTD_minCLevel();
pub const MAXCLEVEL: core::ffi::c_int = ZSTD_maxCLevel();
unsafe fn main_0(
    mut argCount: core::ffi::c_int,
    mut argv: *mut *const core::ffi::c_char,
) -> core::ffi::c_int {
    let mut current_block: u64;
    let mut argNb: core::ffi::c_int = 0;
    let mut followLinks = 0;
    let mut allowBlockDevices = 0;
    let mut forceStdin = 0;
    let mut forceStdout = 0;
    let mut hasStdout = 0;
    let mut ldmFlag = 0;
    let mut main_pause = 0;
    let mut adapt = 0;
    let mut adaptMin = MINCLEVEL;
    let mut adaptMax = MAXCLEVEL;
    let mut rsyncable = 0;
    let mut nextArgumentsAreFiles = 0;
    let mut operationResult = 0;
    let mut separateFiles = 0;
    let mut setRealTimePrio = 0;
    let mut singleThread = 0;
    let mut defaultLogicalCores = 0;
    let mut showDefaultCParams = 0;
    let mut contentSize = 1;
    let mut removeSrcFile = 0;
    let mut cLevel = init_cLevel();
    let mut ultra = 0;
    let mut cLevelLast = MINCLEVEL - 1;
    let mut setThreads_non1 = 0;
    let mut nbWorkers = init_nbWorkers();
    let mut mmapDict = ZSTD_ps_auto;
    let mut useRowMatchFinder = ZSTD_ps_auto;
    let mut cType = FIO_zstdCompression;
    let mut compressibility = -1.0f64;
    let mut bench_nbSeconds = 3;
    let mut chunkSize = 0;
    let prefs = FIO_createPreferences();
    let fCtx = FIO_createContext();
    let mut progress = FIO_ps_auto;
    let mut operation = zom_compress;
    let mut compressionParams = ZSTD_compressionParameters {
        windowLog: 0,
        chainLog: 0,
        hashLog: 0,
        searchLog: 0,
        minMatch: 0,
        targetLength: 0,
        strategy: 0,
    };
    let mut recursive = 0;
    let mut memLimit = 0;
    let mut filenames = UTIL_allocateFileNamesTable(argCount as size_t);
    let mut file_of_names = UTIL_allocateFileNamesTable(argCount as size_t);
    let mut programName = *argv.offset(0);
    let mut outFileName = NULL as *const core::ffi::c_char;
    let mut outDirName = NULL as *const core::ffi::c_char;
    let mut outMirroredDirName = NULL as *const core::ffi::c_char;
    let mut dictFileName = NULL as *const core::ffi::c_char;
    let mut patchFromDictFileName = NULL as *const core::ffi::c_char;
    let mut suffix = ZSTD_EXTENSION.as_ptr();
    let mut maxDictSize = g_defaultMaxDictSize;
    let mut dictID = 0;
    let mut streamSrcSize = 0;
    let mut targetCBlockSize = 0;
    let mut srcSizeHint = 0;
    let mut nbInputFileNames = 0;
    let mut dictCLevel = g_defaultDictCLevel;
    let mut dictSelect = g_defaultSelectivityLevel;
    let mut coverParams = defaultCoverParams();
    let mut fastCoverParams = defaultFastCoverParams();
    let mut dict = fastCover;
    let mut benchParams = BMK_initAdvancedParams();
    let mut literalCompressionMode = ZSTD_ps_auto;
    checkLibVersion();
    assert!(argCount >= 1);
    if filenames.is_null() || file_of_names.is_null() {
        if g_displayLevel >= 1 {
            fprintf(
                stderr,
                b"zstd: allocation error \n\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        exit(1);
    }
    programName = lastNameFromPath(programName);
    if exeNameMatch(programName, ZSTD_ZSTDMT.as_ptr()) != 0 {
        nbWorkers = NBWORKERS_AUTOCPU as core::ffi::c_uint;
        singleThread = 0;
    }
    if exeNameMatch(programName, ZSTD_UNZSTD.as_ptr()) != 0 {
        operation = zom_decompress;
    }
    if exeNameMatch(programName, ZSTD_CAT.as_ptr()) != 0 {
        operation = zom_decompress;
        FIO_overwriteMode(prefs);
        forceStdout = 1;
        followLinks = 1;
        FIO_setPassThroughFlag(prefs, 1);
        outFileName = stdoutmark.as_ptr();
        g_displayLevel = 1;
    }
    if exeNameMatch(programName, ZSTD_ZCAT.as_ptr()) != 0 {
        operation = zom_decompress;
        FIO_overwriteMode(prefs);
        forceStdout = 1;
        followLinks = 1;
        FIO_setPassThroughFlag(prefs, 1);
        outFileName = stdoutmark.as_ptr();
        g_displayLevel = 1;
    }
    if exeNameMatch(programName, ZSTD_GZ.as_ptr()) != 0 {
        suffix = GZ_EXTENSION.as_ptr();
        cType = FIO_gzipCompression;
        removeSrcFile = 1;
        cLevel = 6;
        dictCLevel = cLevel;
    }
    if exeNameMatch(programName, ZSTD_GUNZIP.as_ptr()) != 0 {
        operation = zom_decompress;
        removeSrcFile = 1;
    }
    if exeNameMatch(programName, ZSTD_GZCAT.as_ptr()) != 0 {
        operation = zom_decompress;
        FIO_overwriteMode(prefs);
        forceStdout = 1;
        followLinks = 1;
        FIO_setPassThroughFlag(prefs, 1);
        outFileName = stdoutmark.as_ptr();
        g_displayLevel = 1;
    }
    if exeNameMatch(programName, ZSTD_LZMA.as_ptr()) != 0 {
        suffix = LZMA_EXTENSION.as_ptr();
        cType = FIO_lzmaCompression;
        removeSrcFile = 1;
    }
    if exeNameMatch(programName, ZSTD_UNLZMA.as_ptr()) != 0 {
        operation = zom_decompress;
        cType = FIO_lzmaCompression;
        removeSrcFile = 1;
    }
    if exeNameMatch(programName, ZSTD_XZ.as_ptr()) != 0 {
        suffix = XZ_EXTENSION.as_ptr();
        cType = FIO_xzCompression;
        removeSrcFile = 1;
    }
    if exeNameMatch(programName, ZSTD_UNXZ.as_ptr()) != 0 {
        operation = zom_decompress;
        cType = FIO_xzCompression;
        removeSrcFile = 1;
    }
    if exeNameMatch(programName, ZSTD_LZ4.as_ptr()) != 0 {
        suffix = LZ4_EXTENSION.as_ptr();
        cType = FIO_lz4Compression;
    }
    if exeNameMatch(programName, ZSTD_UNLZ4.as_ptr()) != 0 {
        operation = zom_decompress;
        cType = FIO_lz4Compression;
    }
    ptr::write_bytes(
        &mut compressionParams as *mut ZSTD_compressionParameters as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_compressionParameters>(),
    );
    FIO_addAbortHandler();
    argNb = 1;
    's_373: loop {
        if argNb >= argCount {
            current_block = 17866802397806708230;
            break;
        }
        let mut argument = *argv.offset(argNb as isize);
        let originalArgument = argument;
        if !argument.is_null() {
            if nextArgumentsAreFiles != 0 {
                UTIL_refFilename(filenames, argument);
            } else if strcmp(argument, b"-\0" as *const u8 as *const core::ffi::c_char) == 0 {
                UTIL_refFilename(filenames, stdinmark.as_ptr());
            } else if *argument.offset(0) as core::ffi::c_int == '-' as i32 {
                if *argument.offset(1) as core::ffi::c_int == '-' as i32 {
                    if strcmp(argument, b"--\0" as *const u8 as *const core::ffi::c_char) == 0 {
                        nextArgumentsAreFiles = 1;
                    } else if strcmp(
                        argument,
                        b"--list\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        operation = zom_list;
                    } else if strcmp(
                        argument,
                        b"--compress\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        operation = zom_compress;
                    } else if strcmp(
                        argument,
                        b"--decompress\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        operation = zom_decompress;
                    } else if strcmp(
                        argument,
                        b"--uncompress\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        operation = zom_decompress;
                    } else if strcmp(
                        argument,
                        b"--force\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_overwriteMode(prefs);
                        forceStdin = 1;
                        forceStdout = 1;
                        followLinks = 1;
                        allowBlockDevices = 1;
                    } else if strcmp(
                        argument,
                        b"--version\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        printVersion();
                        operationResult = 0;
                        current_block = 16277912505878250739;
                        break;
                    } else if strcmp(
                        argument,
                        b"--help\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        usageAdvanced(programName);
                        operationResult = 0;
                        current_block = 16277912505878250739;
                        break;
                    } else if strcmp(
                        argument,
                        b"--verbose\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        g_displayLevel += 1;
                    } else if strcmp(
                        argument,
                        b"--quiet\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        g_displayLevel -= 1;
                    } else if strcmp(
                        argument,
                        b"--stdout\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        forceStdout = 1;
                        outFileName = stdoutmark.as_ptr();
                    } else if strcmp(
                        argument,
                        b"--ultra\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        ultra = 1;
                    } else if strcmp(
                        argument,
                        b"--check\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setChecksumFlag(prefs, 2);
                    } else if strcmp(
                        argument,
                        b"--no-check\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setChecksumFlag(prefs, 0);
                    } else if strcmp(
                        argument,
                        b"--sparse\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setSparseWrite(prefs, 2);
                    } else if strcmp(
                        argument,
                        b"--no-sparse\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setSparseWrite(prefs, 0);
                    } else if strcmp(
                        argument,
                        b"--pass-through\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setPassThroughFlag(prefs, 1);
                    } else if strcmp(
                        argument,
                        b"--no-pass-through\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setPassThroughFlag(prefs, 0);
                    } else if strcmp(
                        argument,
                        b"--test\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        operation = zom_test;
                    } else if strcmp(
                        argument,
                        b"--asyncio\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setAsyncIOFlag(prefs, 1);
                    } else if strcmp(
                        argument,
                        b"--no-asyncio\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setAsyncIOFlag(prefs, 0);
                    } else if strcmp(
                        argument,
                        b"--train\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        operation = zom_train;
                        if outFileName.is_null() {
                            outFileName = g_defaultDictName;
                        }
                    } else if strcmp(
                        argument,
                        b"--no-dictID\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        FIO_setDictIDFlag(prefs, 0);
                    } else if strcmp(
                        argument,
                        b"--keep\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        removeSrcFile = 0;
                    } else if strcmp(argument, b"--rm\0" as *const u8 as *const core::ffi::c_char)
                        == 0
                    {
                        removeSrcFile = 1;
                    } else if strcmp(
                        argument,
                        b"--priority=rt\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        setRealTimePrio = 1;
                    } else if strcmp(
                        argument,
                        b"--show-default-cparams\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        showDefaultCParams = 1;
                    } else if strcmp(
                        argument,
                        b"--content-size\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        contentSize = 1;
                    } else if strcmp(
                        argument,
                        b"--no-content-size\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        contentSize = 0;
                    } else if strcmp(
                        argument,
                        b"--adapt\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        adapt = 1;
                    } else if strcmp(
                        argument,
                        b"--no-row-match-finder\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        useRowMatchFinder = ZSTD_ps_disable;
                    } else if strcmp(
                        argument,
                        b"--row-match-finder\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        useRowMatchFinder = ZSTD_ps_enable;
                    } else if longCommandWArg(
                        &mut argument,
                        b"--adapt=\0" as *const u8 as *const core::ffi::c_char,
                    ) != 0
                    {
                        adapt = 1;
                        if parseAdaptParameters(argument, &mut adaptMin, &mut adaptMax) == 0 {
                            badUsage(programName, originalArgument);
                            operationResult = 1;
                            current_block = 16277912505878250739;
                            break;
                        }
                    } else if strcmp(
                        argument,
                        b"--single-thread\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        nbWorkers = 0;
                        singleThread = 1;
                    } else if strcmp(
                        argument,
                        b"--format=zstd\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        suffix = ZSTD_EXTENSION.as_ptr();
                        cType = FIO_zstdCompression;
                    } else if strcmp(
                        argument,
                        b"--mmap-dict\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        mmapDict = ZSTD_ps_enable;
                    } else if strcmp(
                        argument,
                        b"--no-mmap-dict\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        mmapDict = ZSTD_ps_disable;
                    } else if strcmp(
                        argument,
                        b"--format=gzip\0" as *const u8 as *const core::ffi::c_char,
                    ) == 0
                    {
                        suffix = GZ_EXTENSION.as_ptr();
                        cType = FIO_gzipCompression;
                    } else {
                        if exeNameMatch(programName, ZSTD_GZ.as_ptr()) != 0 {
                            if strcmp(
                                argument,
                                b"--best\0" as *const u8 as *const core::ffi::c_char,
                            ) == 0
                            {
                                cLevel = 9;
                                dictCLevel = cLevel;
                                current_block = 8834769789432328951;
                            } else if strcmp(
                                argument,
                                b"--no-name\0" as *const u8 as *const core::ffi::c_char,
                            ) == 0
                            {
                                current_block = 8834769789432328951;
                            } else {
                                current_block = 2925215368761540503;
                            }
                        } else {
                            current_block = 2925215368761540503;
                        }
                        match current_block {
                            8834769789432328951 => {}
                            _ => {
                                if strcmp(
                                    argument,
                                    b"--format=lzma\0" as *const u8 as *const core::ffi::c_char,
                                ) == 0
                                {
                                    suffix = LZMA_EXTENSION.as_ptr();
                                    cType = FIO_lzmaCompression;
                                } else if strcmp(
                                    argument,
                                    b"--format=xz\0" as *const u8 as *const core::ffi::c_char,
                                ) == 0
                                {
                                    suffix = XZ_EXTENSION.as_ptr();
                                    cType = FIO_xzCompression;
                                } else if strcmp(
                                    argument,
                                    b"--rsyncable\0" as *const u8 as *const core::ffi::c_char,
                                ) == 0
                                {
                                    rsyncable = 1;
                                } else if strcmp(
                                    argument,
                                    b"--compress-literals\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) == 0
                                {
                                    literalCompressionMode = ZSTD_ps_enable;
                                } else if strcmp(
                                    argument,
                                    b"--no-compress-literals\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) == 0
                                {
                                    literalCompressionMode = ZSTD_ps_disable;
                                } else if strcmp(
                                    argument,
                                    b"--no-progress\0" as *const u8 as *const core::ffi::c_char,
                                ) == 0
                                {
                                    progress = FIO_ps_never;
                                } else if strcmp(
                                    argument,
                                    b"--progress\0" as *const u8 as *const core::ffi::c_char,
                                ) == 0
                                {
                                    progress = FIO_ps_always;
                                } else if strcmp(
                                    argument,
                                    b"--exclude-compressed\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) == 0
                                {
                                    FIO_setExcludeCompressedFile(prefs, 1);
                                } else if strcmp(
                                    argument,
                                    b"--fake-stdin-is-console\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) == 0
                                {
                                    UTIL_fakeStdinIsConsole();
                                } else if strcmp(
                                    argument,
                                    b"--fake-stdout-is-console\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) == 0
                                {
                                    UTIL_fakeStdoutIsConsole();
                                } else if strcmp(
                                    argument,
                                    b"--fake-stderr-is-console\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) == 0
                                {
                                    UTIL_fakeStderrIsConsole();
                                } else if strcmp(
                                    argument,
                                    b"--trace-file-stat\0" as *const u8 as *const core::ffi::c_char,
                                ) == 0
                                {
                                    UTIL_traceFileStat();
                                } else if strcmp(
                                    argument,
                                    b"--max\0" as *const u8 as *const core::ffi::c_char,
                                ) == 0
                                {
                                    if ::core::mem::size_of::<*mut core::ffi::c_void>()
                                        as core::ffi::c_ulong
                                        == 4
                                    {
                                        if g_displayLevel >= 2 {
                                            fprintf(
                                                stderr,
                                                b"--max is incompatible with 32-bit mode \n\0"
                                                    as *const u8
                                                    as *const core::ffi::c_char,
                                            );
                                        }
                                        badUsage(programName, originalArgument);
                                        operationResult = 1;
                                        current_block = 16277912505878250739;
                                        break;
                                    } else {
                                        ultra = 1;
                                        ldmFlag = 1;
                                        setMaxCompression(&mut compressionParams);
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--train-cover\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    operation = zom_train;
                                    if outFileName.is_null() {
                                        outFileName = g_defaultDictName;
                                    }
                                    dict = cover;
                                    if *argument as core::ffi::c_int == 0 {
                                        ptr::write_bytes(
                                            &mut coverParams as *mut ZDICT_cover_params_t
                                                as *mut u8,
                                            0,
                                            ::core::mem::size_of::<ZDICT_cover_params_t>(),
                                        );
                                    } else {
                                        let fresh0 = argument;
                                        argument = argument.offset(1);
                                        if *fresh0 as core::ffi::c_int != '=' as i32 {
                                            badUsage(programName, originalArgument);
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else if parseCoverParameters(argument, &mut coverParams)
                                            == 0
                                        {
                                            badUsage(programName, originalArgument);
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        }
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--train-fastcover\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    operation = zom_train;
                                    if outFileName.is_null() {
                                        outFileName = g_defaultDictName;
                                    }
                                    dict = fastCover;
                                    if *argument as core::ffi::c_int == 0 {
                                        ptr::write_bytes(
                                            &mut fastCoverParams as *mut ZDICT_fastCover_params_t
                                                as *mut u8,
                                            0,
                                            ::core::mem::size_of::<ZDICT_fastCover_params_t>(),
                                        );
                                    } else {
                                        let fresh1 = argument;
                                        argument = argument.offset(1);
                                        if *fresh1 as core::ffi::c_int != '=' as i32 {
                                            badUsage(programName, originalArgument);
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else if parseFastCoverParameters(
                                            argument,
                                            &mut fastCoverParams,
                                        ) == 0
                                        {
                                            badUsage(programName, originalArgument);
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        }
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--train-legacy\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    operation = zom_train;
                                    if outFileName.is_null() {
                                        outFileName = g_defaultDictName;
                                    }
                                    dict = legacy;
                                    if *argument as core::ffi::c_int != 0 {
                                        let fresh2 = argument;
                                        argument = argument.offset(1);
                                        if *fresh2 as core::ffi::c_int != '=' as i32 {
                                            badUsage(programName, originalArgument);
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else if parseLegacyParameters(argument, &mut dictSelect)
                                            == 0
                                        {
                                            badUsage(programName, originalArgument);
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        }
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--threads\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb = argument;
                                        argument = argument.add(strlen(__nb));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb = *argv.offset(argNb as isize);
                                            if !__nb.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1086,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    nbWorkers = readU32FromChar(&mut __nb);
                                    if *__nb as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                    setThreads_non1 = (nbWorkers != 1) as core::ffi::c_int;
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--memlimit\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_0 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_0 = argument;
                                        argument = argument.add(strlen(__nb_0));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_0 = *argv.offset(argNb as isize);
                                            if !__nb_0.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1087,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_0.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    memLimit = readU32FromChar(&mut __nb_0);
                                    if *__nb_0 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--memory\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_1 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_1 = argument;
                                        argument = argument.add(strlen(__nb_1));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_1 = *argv.offset(argNb as isize);
                                            if !__nb_1.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1088,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_1.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    memLimit = readU32FromChar(&mut __nb_1);
                                    if *__nb_1 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--memlimit-decompress\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_2 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_2 = argument;
                                        argument = argument.add(strlen(__nb_2));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_2 = *argv.offset(argNb as isize);
                                            if !__nb_2.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1089,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_2.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    memLimit = readU32FromChar(&mut __nb_2);
                                    if *__nb_2 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--block-size\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_3 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_3 = argument;
                                        argument = argument.add(strlen(__nb_3));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_3 = *argv.offset(argNb as isize);
                                            if !__nb_3.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1090,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_3.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    chunkSize = readSizeTFromChar(&mut __nb_3);
                                    if *__nb_3 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--split\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_4 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_4 = argument;
                                        argument = argument.add(strlen(__nb_4));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_4 = *argv.offset(argNb as isize);
                                            if !__nb_4.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1091,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_4.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    chunkSize = readSizeTFromChar(&mut __nb_4);
                                    if *__nb_4 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--jobsize\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_5 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_5 = argument;
                                        argument = argument.add(strlen(__nb_5));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_5 = *argv.offset(argNb as isize);
                                            if !__nb_5.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1092,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_5.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    chunkSize = readSizeTFromChar(&mut __nb_5);
                                    if *__nb_5 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--maxdict\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_6 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_6 = argument;
                                        argument = argument.add(strlen(__nb_6));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_6 = *argv.offset(argNb as isize);
                                            if !__nb_6.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1093,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_6.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    maxDictSize = readU32FromChar(&mut __nb_6);
                                    if *__nb_6 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--dictID\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_7 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_7 = argument;
                                        argument = argument.add(strlen(__nb_7));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_7 = *argv.offset(argNb as isize);
                                            if !__nb_7.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1094,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_7.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    dictID = readU32FromChar(&mut __nb_7);
                                    if *__nb_7 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--zstd=\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    if parseCompressionParameters(argument, &mut compressionParams)
                                        == 0
                                    {
                                        badUsage(programName, originalArgument);
                                        operationResult = 1;
                                        current_block = 16277912505878250739;
                                        break;
                                    } else {
                                        cType = FIO_zstdCompression;
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--stream-size\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_8 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_8 = argument;
                                        argument = argument.add(strlen(__nb_8));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_8 = *argv.offset(argNb as isize);
                                            if !__nb_8.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1096,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_8.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    streamSrcSize = readSizeTFromChar(&mut __nb_8);
                                    if *__nb_8 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--target-compressed-block-size\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_9 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_9 = argument;
                                        argument = argument.add(strlen(__nb_9));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_9 = *argv.offset(argNb as isize);
                                            if !__nb_9.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1097,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_9.offset(0) as core::ffi::c_int == '-' as i32 {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    targetCBlockSize = readSizeTFromChar(&mut __nb_9);
                                    if *__nb_9 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--size-hint\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut __nb_10 = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        __nb_10 = argument;
                                        argument = argument.add(strlen(__nb_10));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            __nb_10 = *argv.offset(argNb as isize);
                                            if !__nb_10.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"__nb != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1098,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *__nb_10.offset(0) as core::ffi::c_int == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    srcSizeHint = readSizeTFromChar(&mut __nb_10);
                                    if *__nb_10 as core::ffi::c_int != 0 {
                                        errorOut(
                                            b"error: only numeric values with optional suffixes K, KB, KiB, M, MB, MiB are allowed\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--output-dir-flat\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        outDirName = argument;
                                        argument = argument.add(strlen(outDirName));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            outDirName = *argv.offset(argNb as isize);
                                            if !outDirName.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"outDirName != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1100,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *outDirName.offset(0) as core::ffi::c_int
                                                == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    if strlen(outDirName) == 0 {
                                        if g_displayLevel >= 1 {
                                            fprintf(
                                                stderr,
                                                b"error: output dir cannot be empty string (did you mean to pass '.' instead?)\n\0"
                                                    as *const u8 as *const core::ffi::c_char,
                                            );
                                        }
                                        operationResult = 1;
                                        current_block = 16277912505878250739;
                                        break;
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--auto-threads\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut threadDefault = NULL as *const core::ffi::c_char;
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        threadDefault = argument;
                                        argument = argument.add(strlen(threadDefault));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            threadDefault = *argv.offset(argNb as isize);
                                            if !threadDefault.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"threadDefault != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1109,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *threadDefault.offset(0) as core::ffi::c_int
                                                == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    if strcmp(
                                        threadDefault,
                                        b"logical\0" as *const u8 as *const core::ffi::c_char,
                                    ) == 0
                                    {
                                        defaultLogicalCores = 1;
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--output-dir-mirror\0" as *const u8
                                        as *const core::ffi::c_char,
                                ) != 0
                                {
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        outMirroredDirName = argument;
                                        argument =
                                            argument.add(strlen(outMirroredDirName));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            outMirroredDirName = *argv.offset(argNb as isize);
                                            if !outMirroredDirName.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"outMirroredDirName != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1116,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *outMirroredDirName.offset(0) as core::ffi::c_int
                                                == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    if strlen(outMirroredDirName) == 0 {
                                        if g_displayLevel >= 1 {
                                            fprintf(
                                                stderr,
                                                b"error: output dir cannot be empty string (did you mean to pass '.' instead?)\n\0"
                                                    as *const u8 as *const core::ffi::c_char,
                                            );
                                        }
                                        operationResult = 1;
                                        current_block = 16277912505878250739;
                                        break;
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--trace\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut traceFile = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        traceFile = argument;
                                        argument = argument.add(strlen(traceFile));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            traceFile = *argv.offset(argNb as isize);
                                            if !traceFile.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"traceFile != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1125,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *traceFile.offset(0) as core::ffi::c_int
                                                == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    TRACE_enable(traceFile);
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--patch-from\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        patchFromDictFileName = argument;
                                        argument =
                                            argument.add(strlen(patchFromDictFileName));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            patchFromDictFileName = *argv.offset(argNb as isize);
                                            if !patchFromDictFileName.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"patchFromDictFileName != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1127,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *patchFromDictFileName.offset(0) as core::ffi::c_int
                                                == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    ultra = 1;
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--patch-apply\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    operation = zom_decompress;
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        patchFromDictFileName = argument;
                                        argument =
                                            argument.add(strlen(patchFromDictFileName));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            patchFromDictFileName = *argv.offset(argNb as isize);
                                            if !patchFromDictFileName.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"patchFromDictFileName != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1128,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *patchFromDictFileName.offset(0) as core::ffi::c_int
                                                == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    memLimit = (1)
                                        << (if ::core::mem::size_of::<size_t>() == 4 {
                                            ZSTD_WINDOWLOG_MAX_32
                                        } else {
                                            ZSTD_WINDOWLOG_MAX_64
                                        });
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--long\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut ldmWindowLog = 0;
                                    ldmFlag = 1;
                                    ultra = 1;
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        ldmWindowLog = readU32FromChar(&mut argument);
                                    } else if *argument as core::ffi::c_int != 0 {
                                        badUsage(programName, originalArgument);
                                        operationResult = 1;
                                        current_block = 16277912505878250739;
                                        break;
                                    } else {
                                        ldmWindowLog = g_defaultMaxWindowLog;
                                    }
                                    if compressionParams.windowLog == 0 {
                                        compressionParams.windowLog = ldmWindowLog;
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--fast\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        let maxFast = -ZSTD_minCLevel() as u32;
                                        let mut fastLevel: u32 = 0;
                                        argument = argument.offset(1);
                                        fastLevel = readU32FromChar(&mut argument);
                                        if fastLevel > maxFast {
                                            fastLevel = maxFast;
                                        }
                                        if fastLevel != 0 {
                                            cLevel = -(fastLevel as core::ffi::c_int);
                                            dictCLevel = cLevel;
                                        } else {
                                            badUsage(programName, originalArgument);
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        }
                                    } else if *argument as core::ffi::c_int != 0 {
                                        badUsage(programName, originalArgument);
                                        operationResult = 1;
                                        current_block = 16277912505878250739;
                                        break;
                                    } else {
                                        cLevel = -(1);
                                    }
                                } else if longCommandWArg(
                                    &mut argument,
                                    b"--filelist\0" as *const u8 as *const core::ffi::c_char,
                                ) != 0
                                {
                                    let mut listName = core::ptr::null::<core::ffi::c_char>();
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        listName = argument;
                                        argument = argument.add(strlen(listName));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break;
                                        } else {
                                            listName = *argv.offset(argNb as isize);
                                            if !listName.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"listName != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1177,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *listName.offset(0) as core::ffi::c_int == '-' as i32
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error: command cannot be separated from its argument by another command \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                                break;
                                            }
                                        }
                                    }
                                    UTIL_refFilename(file_of_names, listName);
                                } else {
                                    badUsage(programName, originalArgument);
                                    operationResult = 1;
                                    current_block = 16277912505878250739;
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    argument = argument.offset(1);
                    while *argument.offset(0) as core::ffi::c_int != 0 {
                        if *argument as core::ffi::c_int >= '0' as i32
                            && *argument as core::ffi::c_int <= '9' as i32
                        {
                            cLevel = readU32FromChar(&mut argument) as core::ffi::c_int;
                            dictCLevel = cLevel;
                        } else {
                            match *argument.offset(0) as core::ffi::c_int {
                                86 => {
                                    printVersion();
                                    operationResult = 0;
                                    current_block = 16277912505878250739;
                                    break 's_373;
                                }
                                72 => {
                                    usageAdvanced(programName);
                                    operationResult = 0;
                                    current_block = 16277912505878250739;
                                    break 's_373;
                                }
                                104 => {
                                    usage(stdout, programName);
                                    operationResult = 0;
                                    current_block = 16277912505878250739;
                                    break 's_373;
                                }
                                122 => {
                                    operation = zom_compress;
                                    argument = argument.offset(1);
                                }
                                100 => {
                                    benchParams.mode = BMK_decodeOnly;
                                    if operation as core::ffi::c_uint
                                        == zom_bench as core::ffi::c_int as core::ffi::c_uint
                                    {
                                        argument = argument.offset(1);
                                    } else {
                                        operation = zom_decompress;
                                        argument = argument.offset(1);
                                    }
                                }
                                99 => {
                                    forceStdout = 1;
                                    outFileName = stdoutmark.as_ptr();
                                    argument = argument.offset(1);
                                }
                                111 => {
                                    argument = argument.offset(1);
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        outFileName = argument;
                                        argument = argument.add(strlen(outFileName));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break 's_373;
                                        } else {
                                            outFileName = *argv.offset(argNb as isize);
                                            if !outFileName.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"outFileName != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1219,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *outFileName.offset(0) as core::ffi::c_int
                                                != '-' as i32
                                            {
                                                continue;
                                            }
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: command cannot be separated from its argument by another command \n\0"
                                                        as *const u8 as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break 's_373;
                                        }
                                    }
                                }
                                110 => {
                                    argument = argument.offset(1);
                                }
                                68 => {
                                    argument = argument.offset(1);
                                    if *argument as core::ffi::c_int == '=' as i32 {
                                        argument = argument.offset(1);
                                        dictFileName = argument;
                                        argument = argument.add(strlen(dictFileName));
                                    } else {
                                        argNb += 1;
                                        if argNb >= argCount {
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: missing command argument \n\0"
                                                        as *const u8
                                                        as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break 's_373;
                                        } else {
                                            dictFileName = *argv.offset(argNb as isize);
                                            if !dictFileName.is_null() {
                                            } else {
                                                __assert_fail(
                                                    b"dictFileName != NULL\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    b"zstdcli.c\0" as *const u8
                                                        as *const core::ffi::c_char,
                                                    1225,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 29],
                                                        &[core::ffi::c_char; 29],
                                                    >(
                                                        b"int main(int, const char **)\0"
                                                    ))
                                                    .as_ptr(),
                                                );
                                            }
                                            if *dictFileName.offset(0) as core::ffi::c_int
                                                != '-' as i32
                                            {
                                                continue;
                                            }
                                            if g_displayLevel >= 1 {
                                                fprintf(
                                                    stderr,
                                                    b"error: command cannot be separated from its argument by another command \n\0"
                                                        as *const u8 as *const core::ffi::c_char,
                                                );
                                            }
                                            operationResult = 1;
                                            current_block = 16277912505878250739;
                                            break 's_373;
                                        }
                                    }
                                }
                                102 => {
                                    FIO_overwriteMode(prefs);
                                    forceStdin = 1;
                                    forceStdout = 1;
                                    followLinks = 1;
                                    allowBlockDevices = 1;
                                    argument = argument.offset(1);
                                }
                                118 => {
                                    g_displayLevel += 1;
                                    argument = argument.offset(1);
                                }
                                113 => {
                                    g_displayLevel -= 1;
                                    argument = argument.offset(1);
                                }
                                107 => {
                                    removeSrcFile = 0;
                                    argument = argument.offset(1);
                                }
                                67 => {
                                    FIO_setChecksumFlag(prefs, 2);
                                    argument = argument.offset(1);
                                }
                                116 => {
                                    operation = zom_test;
                                    argument = argument.offset(1);
                                }
                                77 => {
                                    argument = argument.offset(1);
                                    memLimit = readU32FromChar(&mut argument);
                                }
                                108 => {
                                    operation = zom_list;
                                    argument = argument.offset(1);
                                }
                                114 => {
                                    recursive = 1;
                                    argument = argument.offset(1);
                                }
                                98 => {
                                    operation = zom_bench;
                                    argument = argument.offset(1);
                                }
                                101 => {
                                    argument = argument.offset(1);
                                    cLevelLast = readU32FromChar(&mut argument) as core::ffi::c_int;
                                }
                                105 => {
                                    argument = argument.offset(1);
                                    bench_nbSeconds = readU32FromChar(&mut argument);
                                }
                                66 => {
                                    argument = argument.offset(1);
                                    chunkSize = readU32FromChar(&mut argument) as size_t;
                                }
                                83 => {
                                    argument = argument.offset(1);
                                    separateFiles = 1;
                                }
                                84 => {
                                    argument = argument.offset(1);
                                    nbWorkers = readU32FromChar(&mut argument);
                                    setThreads_non1 = (nbWorkers != 1) as core::ffi::c_int;
                                }
                                115 => {
                                    argument = argument.offset(1);
                                    dictSelect = readU32FromChar(&mut argument);
                                }
                                112 => {
                                    argument = argument.offset(1);
                                    if *argument as core::ffi::c_int >= '0' as i32
                                        && *argument as core::ffi::c_int <= '9' as i32
                                    {
                                        benchParams.additionalParam =
                                            readU32FromChar(&mut argument) as core::ffi::c_int;
                                    } else {
                                        main_pause = 1;
                                    }
                                }
                                80 => {
                                    argument = argument.offset(1);
                                    compressibility = readU32FromChar(&mut argument)
                                        as core::ffi::c_double
                                        / 100.0;
                                }
                                _ => {
                                    let mut shortArgument: [core::ffi::c_char; 3] =
                                        ['-' as i32 as core::ffi::c_char, 0, 0];
                                    *shortArgument.as_mut_ptr().offset(1) = *argument.offset(0);
                                    badUsage(programName, shortArgument.as_mut_ptr());
                                    operationResult = 1;
                                    current_block = 16277912505878250739;
                                    break 's_373;
                                }
                            }
                        }
                    }
                }
            } else {
                UTIL_refFilename(filenames, argument);
            }
        }
        argNb += 1;
    }
    if current_block == 17866802397806708230 {
        if g_displayLevel >= 3 {
            fprintf(
                stderr,
                b"*** %s (%i-bit) %s, by %s ***\n\0" as *const u8 as *const core::ffi::c_char,
                b"Zstandard CLI\0" as *const u8 as *const core::ffi::c_char,
                (::core::mem::size_of::<size_t>()).wrapping_mul(8) as core::ffi::c_int,
                b"v1.5.8\0" as *const u8 as *const core::ffi::c_char,
                b"Yann Collet\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        if operation as core::ffi::c_uint == zom_decompress as core::ffi::c_int as core::ffi::c_uint
            && setThreads_non1 != 0
            && g_displayLevel >= 2
        {
            fprintf(
                stderr,
                b"Warning : decompression does not support multi-threading\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        if nbWorkers == NBWORKERS_AUTOCPU as core::ffi::c_uint && singleThread == 0 {
            if defaultLogicalCores != 0 {
                nbWorkers = UTIL_countLogicalCores() as core::ffi::c_uint;
                if g_displayLevel >= 3 {
                    fprintf(
                        stderr,
                        b"Note: %d logical core(s) detected \n\0" as *const u8
                            as *const core::ffi::c_char,
                        nbWorkers,
                    );
                }
            } else {
                nbWorkers = UTIL_countPhysicalCores() as core::ffi::c_uint;
                if g_displayLevel >= 3 {
                    fprintf(
                        stderr,
                        b"Note: %d physical core(s) detected \n\0" as *const u8
                            as *const core::ffi::c_char,
                        nbWorkers,
                    );
                }
            }
        }
        if operation as core::ffi::c_uint == zom_compress as core::ffi::c_int as core::ffi::c_uint
            && g_displayLevel >= 4
        {
            fprintf(
                stderr,
                b"Compressing with %u worker threads \n\0" as *const u8 as *const core::ffi::c_char,
                nbWorkers,
            );
        }
        g_utilDisplayLevel = g_displayLevel;
        if followLinks == 0 {
            let mut u: core::ffi::c_uint = 0;
            let mut fileNamesNb: core::ffi::c_uint = 0;
            let nbFilenames = (*filenames).tableSize as core::ffi::c_uint;
            u = 0;
            fileNamesNb = 0;
            while u < nbFilenames {
                if UTIL_isLink(*((*filenames).fileNames).offset(u as isize)) != 0
                    && UTIL_isFIFO(*((*filenames).fileNames).offset(u as isize)) == 0
                {
                    if g_displayLevel >= 2 {
                        fprintf(
                            stderr,
                            b"Warning : %s is a symbolic link, ignoring \n\0" as *const u8
                                as *const core::ffi::c_char,
                            *((*filenames).fileNames).offset(u as isize),
                        );
                    }
                } else {
                    let fresh3 = fileNamesNb;
                    fileNamesNb = fileNamesNb.wrapping_add(1);
                    let fresh4 = &mut (*((*filenames).fileNames).offset(fresh3 as isize));
                    *fresh4 = *((*filenames).fileNames).offset(u as isize);
                }
                u = u.wrapping_add(1);
            }
            if fileNamesNb == 0 && nbFilenames > 0 {
                operationResult = 1;
                current_block = 16277912505878250739;
            } else {
                (*filenames).tableSize = fileNamesNb as size_t;
                current_block = 665450585616957159;
            }
        } else {
            current_block = 665450585616957159;
        }
        match current_block {
            16277912505878250739 => {}
            _ => {
                if (*file_of_names).tableSize != 0 {
                    let nbFileLists = (*file_of_names).tableSize;
                    let mut flNb: size_t = 0;
                    flNb = 0;
                    loop {
                        if flNb >= nbFileLists {
                            current_block = 2697231409163282360;
                            break;
                        }
                        let fnt = UTIL_createFileNamesTable_fromFileList(
                            *((*file_of_names).fileNames).add(flNb),
                        );
                        if fnt.is_null() {
                            if g_displayLevel >= 1 {
                                fprintf(
                                    stderr,
                                    b"zstd: error reading %s \n\0" as *const u8
                                        as *const core::ffi::c_char,
                                    *((*file_of_names).fileNames).add(flNb),
                                );
                            }
                            operationResult = 1;
                            current_block = 16277912505878250739;
                            break;
                        } else {
                            filenames = UTIL_mergeFileNamesTable(filenames, fnt);
                            flNb = flNb.wrapping_add(1);
                        }
                    }
                } else {
                    current_block = 2697231409163282360;
                }
                match current_block {
                    16277912505878250739 => {}
                    _ => {
                        nbInputFileNames = (*filenames).tableSize;
                        if recursive != 0 {
                            UTIL_expandFNT(&mut filenames, followLinks);
                        }
                        if operation as core::ffi::c_uint
                            == zom_list as core::ffi::c_int as core::ffi::c_uint
                        {
                            let ret = FIO_listMultipleFiles(
                                (*filenames).tableSize as core::ffi::c_uint,
                                (*filenames).fileNames,
                                g_displayLevel,
                            );
                            operationResult = ret;
                        } else if operation as core::ffi::c_uint
                            == zom_bench as core::ffi::c_int as core::ffi::c_uint
                        {
                            if cType as core::ffi::c_uint
                                != FIO_zstdCompression as core::ffi::c_int as core::ffi::c_uint
                            {
                                if g_displayLevel >= 1 {
                                    fprintf(
                                        stderr,
                                        b"benchmark mode is only compatible with zstd format \n\0"
                                            as *const u8
                                            as *const core::ffi::c_char,
                                    );
                                }
                                operationResult = 1;
                            } else {
                                benchParams.chunkSizeMax = chunkSize;
                                benchParams.targetCBlockSize = targetCBlockSize;
                                benchParams.nbWorkers = nbWorkers as core::ffi::c_int;
                                benchParams.realTime = setRealTimePrio as core::ffi::c_uint;
                                benchParams.nbSeconds = bench_nbSeconds;
                                benchParams.ldmFlag = ldmFlag;
                                benchParams.ldmMinMatch = g_ldmMinMatch as core::ffi::c_int;
                                benchParams.ldmHashLog = g_ldmHashLog as core::ffi::c_int;
                                benchParams.useRowMatchFinder =
                                    useRowMatchFinder as core::ffi::c_int;
                                if g_ldmBucketSizeLog != LDM_PARAM_DEFAULT as u32 {
                                    benchParams.ldmBucketSizeLog =
                                        g_ldmBucketSizeLog as core::ffi::c_int;
                                }
                                if g_ldmHashRateLog != LDM_PARAM_DEFAULT as u32 {
                                    benchParams.ldmHashRateLog =
                                        g_ldmHashRateLog as core::ffi::c_int;
                                }
                                benchParams.literalCompressionMode = literalCompressionMode;
                                if benchParams.mode as core::ffi::c_uint
                                    == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
                                {
                                    cLevelLast = 0;
                                    cLevel = cLevelLast;
                                }
                                if cLevel > ZSTD_maxCLevel() {
                                    cLevel = ZSTD_maxCLevel();
                                }
                                if cLevelLast > ZSTD_maxCLevel() {
                                    cLevelLast = ZSTD_maxCLevel();
                                }
                                if cLevelLast < cLevel {
                                    cLevelLast = cLevel;
                                }
                                if g_displayLevel >= 3 {
                                    fprintf(
                                        stderr,
                                        b"Benchmarking \0" as *const u8 as *const core::ffi::c_char,
                                    );
                                }
                                if (*filenames).tableSize > 1 && g_displayLevel >= 3 {
                                    fprintf(
                                        stderr,
                                        b"%u files \0" as *const u8 as *const core::ffi::c_char,
                                        (*filenames).tableSize as core::ffi::c_uint,
                                    );
                                }
                                if cLevelLast > cLevel {
                                    if g_displayLevel >= 3 {
                                        fprintf(
                                            stderr,
                                            b"from level %d to %d \0" as *const u8
                                                as *const core::ffi::c_char,
                                            cLevel,
                                            cLevelLast,
                                        );
                                    }
                                } else if g_displayLevel >= 3 {
                                    fprintf(
                                        stderr,
                                        b"at level %d \0" as *const u8 as *const core::ffi::c_char,
                                        cLevel,
                                    );
                                }
                                if g_displayLevel >= 3 {
                                    fprintf(
                                        stderr,
                                        b"using %i threads \n\0" as *const u8
                                            as *const core::ffi::c_char,
                                        nbWorkers,
                                    );
                                }
                                if (*filenames).tableSize > 0 {
                                    if separateFiles != 0 {
                                        let mut i: core::ffi::c_uint = 0;
                                        i = 0;
                                        while (i as size_t) < (*filenames).tableSize {
                                            operationResult = BMK_benchFilesAdvanced(
                                                &mut *((*filenames).fileNames).offset(i as isize),
                                                1,
                                                dictFileName,
                                                cLevel,
                                                cLevelLast,
                                                &mut compressionParams,
                                                g_displayLevel,
                                                &mut benchParams,
                                            );
                                            i = i.wrapping_add(1);
                                        }
                                    } else {
                                        operationResult = BMK_benchFilesAdvanced(
                                            (*filenames).fileNames,
                                            (*filenames).tableSize as core::ffi::c_uint,
                                            dictFileName,
                                            cLevel,
                                            cLevelLast,
                                            &mut compressionParams,
                                            g_displayLevel,
                                            &mut benchParams,
                                        );
                                    }
                                } else {
                                    operationResult = BMK_syntheticTest(
                                        compressibility,
                                        cLevel,
                                        cLevelLast,
                                        &mut compressionParams,
                                        g_displayLevel,
                                        &mut benchParams,
                                    );
                                }
                            }
                        } else if operation as core::ffi::c_uint
                            == zom_train as core::ffi::c_int as core::ffi::c_uint
                        {
                            let mut zParams = ZDICT_params_t {
                                compressionLevel: 0,
                                notificationLevel: 0,
                                dictID: 0,
                            };
                            zParams.compressionLevel = dictCLevel;
                            zParams.notificationLevel = g_displayLevel as core::ffi::c_uint;
                            zParams.dictID = dictID;
                            if dict as core::ffi::c_uint
                                == cover as core::ffi::c_int as core::ffi::c_uint
                            {
                                let optimize =
                                    (coverParams.k == 0 || coverParams.d == 0) as core::ffi::c_int;
                                coverParams.nbThreads = nbWorkers;
                                coverParams.zParams = zParams;
                                operationResult = DiB_trainFromFiles(
                                    outFileName,
                                    maxDictSize as size_t,
                                    (*filenames).fileNames,
                                    (*filenames).tableSize as core::ffi::c_int,
                                    chunkSize,
                                    NULL as *mut ZDICT_legacy_params_t,
                                    &mut coverParams,
                                    NULL as *mut ZDICT_fastCover_params_t,
                                    optimize,
                                    memLimit,
                                );
                            } else if dict as core::ffi::c_uint
                                == fastCover as core::ffi::c_int as core::ffi::c_uint
                            {
                                let optimize_0 = (fastCoverParams.k == 0 || fastCoverParams.d == 0)
                                    as core::ffi::c_int;
                                fastCoverParams.nbThreads = nbWorkers;
                                fastCoverParams.zParams = zParams;
                                operationResult = DiB_trainFromFiles(
                                    outFileName,
                                    maxDictSize as size_t,
                                    (*filenames).fileNames,
                                    (*filenames).tableSize as core::ffi::c_int,
                                    chunkSize,
                                    NULL as *mut ZDICT_legacy_params_t,
                                    NULL as *mut ZDICT_cover_params_t,
                                    &mut fastCoverParams,
                                    optimize_0,
                                    memLimit,
                                );
                            } else {
                                let mut dictParams = ZDICT_legacy_params_t {
                                    selectivityLevel: 0,
                                    zParams: ZDICT_params_t {
                                        compressionLevel: 0,
                                        notificationLevel: 0,
                                        dictID: 0,
                                    },
                                };
                                ptr::write_bytes(
                                    &mut dictParams as *mut ZDICT_legacy_params_t as *mut u8,
                                    0,
                                    ::core::mem::size_of::<ZDICT_legacy_params_t>(),
                                );
                                dictParams.selectivityLevel = dictSelect;
                                dictParams.zParams = zParams;
                                operationResult = DiB_trainFromFiles(
                                    outFileName,
                                    maxDictSize as size_t,
                                    (*filenames).fileNames,
                                    (*filenames).tableSize as core::ffi::c_int,
                                    chunkSize,
                                    &mut dictParams,
                                    NULL as *mut ZDICT_cover_params_t,
                                    NULL as *mut ZDICT_fastCover_params_t,
                                    0,
                                    memLimit,
                                );
                            }
                        } else {
                            if operation as core::ffi::c_uint
                                == zom_test as core::ffi::c_int as core::ffi::c_uint
                            {
                                FIO_setTestMode(prefs, 1);
                                outFileName = nulmark.as_ptr();
                                removeSrcFile = 0;
                            }
                            if (*filenames).tableSize == 0 {
                                if nbInputFileNames > 0 {
                                    if g_displayLevel >= 1 {
                                        fprintf(
                                            stderr,
                                            b"please provide correct input file(s) or non-empty directories -- ignored \n\0"
                                                as *const u8 as *const core::ffi::c_char,
                                        );
                                    }
                                    operationResult = 0;
                                    current_block = 16277912505878250739;
                                } else {
                                    UTIL_refFilename(filenames, stdinmark.as_ptr());
                                    current_block = 2266625133249563118;
                                }
                            } else {
                                current_block = 2266625133249563118;
                            }
                            match current_block {
                                16277912505878250739 => {}
                                _ => {
                                    if (*filenames).tableSize == 1
                                        && strcmp(
                                            *((*filenames).fileNames).offset(0),
                                            stdinmark.as_ptr(),
                                        ) == 0
                                        && outFileName.is_null()
                                    {
                                        outFileName = stdoutmark.as_ptr();
                                    }
                                    if forceStdin == 0
                                        && UTIL_searchFileNamesTable(filenames, stdinmark.as_ptr())
                                            != -(1)
                                        && UTIL_isConsole(stdin) != 0
                                    {
                                        if g_displayLevel >= 1 {
                                            fprintf(
                                                stderr,
                                                b"stdin is a console, aborting\n\0" as *const u8
                                                    as *const core::ffi::c_char,
                                            );
                                        }
                                        operationResult = 1;
                                    } else if (outFileName.is_null()
                                        || strcmp(outFileName, stdoutmark.as_ptr()) == 0)
                                        && UTIL_isConsole(stdout) != 0
                                        && UTIL_searchFileNamesTable(filenames, stdinmark.as_ptr())
                                            != -(1)
                                        && forceStdout == 0
                                        && operation as core::ffi::c_uint
                                            != zom_decompress as core::ffi::c_int
                                                as core::ffi::c_uint
                                    {
                                        if g_displayLevel >= 1 {
                                            fprintf(
                                                stderr,
                                                b"stdout is a console, aborting\n\0" as *const u8
                                                    as *const core::ffi::c_char,
                                            );
                                        }
                                        operationResult = 1;
                                    } else {
                                        let maxCLevel = if ultra != 0 {
                                            ZSTD_maxCLevel()
                                        } else {
                                            ZSTDCLI_CLEVEL_MAX
                                        };
                                        if cLevel > maxCLevel {
                                            if g_displayLevel >= 2 {
                                                fprintf(
                                                    stderr,
                                                    b"Warning : compression level higher than max, reduced to %i \n\0"
                                                        as *const u8 as *const core::ffi::c_char,
                                                    maxCLevel,
                                                );
                                            }
                                            cLevel = maxCLevel;
                                        }
                                        if showDefaultCParams != 0 {
                                            if operation as core::ffi::c_uint
                                                == zom_decompress as core::ffi::c_int
                                                    as core::ffi::c_uint
                                            {
                                                if g_displayLevel >= 1 {
                                                    fprintf(
                                                        stderr,
                                                        b"error : can't use --show-default-cparams in decompression mode \n\0"
                                                            as *const u8 as *const core::ffi::c_char,
                                                    );
                                                }
                                                operationResult = 1;
                                                current_block = 16277912505878250739;
                                            } else {
                                                current_block = 16042738330116972545;
                                            }
                                        } else {
                                            current_block = 16042738330116972545;
                                        }
                                        match current_block {
                                            16277912505878250739 => {}
                                            _ => {
                                                if !dictFileName.is_null()
                                                    && !patchFromDictFileName.is_null()
                                                {
                                                    if g_displayLevel >= 1 {
                                                        fprintf(
                                                            stderr,
                                                            b"error : can't use -D and --patch-from=# at the same time \n\0"
                                                                as *const u8 as *const core::ffi::c_char,
                                                        );
                                                    }
                                                    operationResult = 1;
                                                } else if !patchFromDictFileName.is_null()
                                                    && (*filenames).tableSize > 1
                                                {
                                                    if g_displayLevel >= 1 {
                                                        fprintf(
                                                            stderr,
                                                            b"error : can't use --patch-from=# on multiple files \n\0"
                                                                as *const u8 as *const core::ffi::c_char,
                                                        );
                                                    }
                                                    operationResult = 1;
                                                } else {
                                                    hasStdout = (!outFileName.is_null()
                                                        && strcmp(outFileName, stdoutmark.as_ptr())
                                                            == 0)
                                                        as core::ffi::c_int;
                                                    if hasStdout != 0 && g_displayLevel == 2 {
                                                        g_displayLevel = 1;
                                                    }
                                                    if UTIL_isConsole(stderr) == 0
                                                        && progress as core::ffi::c_uint
                                                            != FIO_ps_always as core::ffi::c_int
                                                                as core::ffi::c_uint
                                                    {
                                                        progress = FIO_ps_never;
                                                    }
                                                    FIO_setProgressSetting(progress);
                                                    if hasStdout != 0 && removeSrcFile != 0 {
                                                        if g_displayLevel >= 3 {
                                                            fprintf(
                                                                stderr,
                                                                b"Note: src files are not removed when output is stdout \n\0"
                                                                    as *const u8 as *const core::ffi::c_char,
                                                            );
                                                        }
                                                        removeSrcFile = 0;
                                                    }
                                                    FIO_setRemoveSrcFile(prefs, removeSrcFile);
                                                    FIO_setHasStdoutOutput(fCtx, hasStdout);
                                                    FIO_setNbFilesTotal(
                                                        fCtx,
                                                        (*filenames).tableSize as core::ffi::c_int,
                                                    );
                                                    FIO_determineHasStdinInput(fCtx, filenames);
                                                    FIO_setNotificationLevel(g_displayLevel);
                                                    FIO_setAllowBlockDevices(
                                                        prefs,
                                                        allowBlockDevices,
                                                    );
                                                    FIO_setPatchFromMode(
                                                        prefs,
                                                        (patchFromDictFileName
                                                            != NULL as *const core::ffi::c_char)
                                                            as core::ffi::c_int,
                                                    );
                                                    FIO_setMMapDict(prefs, mmapDict);
                                                    if memLimit == 0 {
                                                        if compressionParams.windowLog
                                                            == 0 as core::ffi::c_uint
                                                        {
                                                            memLimit =
                                                                (1 as u32) << g_defaultMaxWindowLog;
                                                        } else {
                                                            memLimit = (1 as u32)
                                                                << (compressionParams.windowLog
                                                                    & 31);
                                                        }
                                                    }
                                                    if !patchFromDictFileName.is_null() {
                                                        dictFileName = patchFromDictFileName;
                                                    }
                                                    FIO_setMemLimit(prefs, memLimit);
                                                    if operation as core::ffi::c_uint
                                                        == zom_compress as core::ffi::c_int
                                                            as core::ffi::c_uint
                                                    {
                                                        FIO_setCompressionType(prefs, cType);
                                                        FIO_setContentSize(prefs, contentSize);
                                                        FIO_setNbWorkers(
                                                            prefs,
                                                            nbWorkers as core::ffi::c_int,
                                                        );
                                                        FIO_setJobSize(
                                                            prefs,
                                                            chunkSize as core::ffi::c_int,
                                                        );
                                                        if g_overlapLog
                                                            != OVERLAP_LOG_DEFAULT as u32
                                                        {
                                                            FIO_setOverlapLog(
                                                                prefs,
                                                                g_overlapLog as core::ffi::c_int,
                                                            );
                                                        }
                                                        FIO_setLdmFlag(
                                                            prefs,
                                                            ldmFlag as core::ffi::c_uint,
                                                        );
                                                        FIO_setLdmHashLog(
                                                            prefs,
                                                            g_ldmHashLog as core::ffi::c_int,
                                                        );
                                                        FIO_setLdmMinMatch(
                                                            prefs,
                                                            g_ldmMinMatch as core::ffi::c_int,
                                                        );
                                                        if g_ldmBucketSizeLog
                                                            != LDM_PARAM_DEFAULT as u32
                                                        {
                                                            FIO_setLdmBucketSizeLog(
                                                                prefs,
                                                                g_ldmBucketSizeLog
                                                                    as core::ffi::c_int,
                                                            );
                                                        }
                                                        if g_ldmHashRateLog
                                                            != LDM_PARAM_DEFAULT as u32
                                                        {
                                                            FIO_setLdmHashRateLog(
                                                                prefs,
                                                                g_ldmHashRateLog
                                                                    as core::ffi::c_int,
                                                            );
                                                        }
                                                        FIO_setAdaptiveMode(prefs, adapt);
                                                        FIO_setUseRowMatchFinder(
                                                            prefs,
                                                            useRowMatchFinder as core::ffi::c_int,
                                                        );
                                                        FIO_setAdaptMin(prefs, adaptMin);
                                                        FIO_setAdaptMax(prefs, adaptMax);
                                                        FIO_setRsyncable(prefs, rsyncable);
                                                        FIO_setStreamSrcSize(prefs, streamSrcSize);
                                                        FIO_setTargetCBlockSize(
                                                            prefs,
                                                            targetCBlockSize,
                                                        );
                                                        FIO_setSrcSizeHint(prefs, srcSizeHint);
                                                        FIO_setLiteralCompressionMode(
                                                            prefs,
                                                            literalCompressionMode,
                                                        );
                                                        FIO_setSparseWrite(prefs, 0);
                                                        if adaptMin > cLevel {
                                                            cLevel = adaptMin;
                                                        }
                                                        if adaptMax < cLevel {
                                                            cLevel = adaptMax;
                                                        }
                                                        let mut strategyBounds =
                                                            ZSTD_cParam_getBounds(ZSTD_c_strategy);
                                                        assert!(
                                                            ZSTD_NB_STRATEGIES as core::ffi::c_int
                                                                == strategyBounds.upperBound
                                                        );
                                                        if showDefaultCParams != 0
                                                            || g_displayLevel >= 4
                                                        {
                                                            let mut fileNb: size_t = 0;
                                                            fileNb = 0;
                                                            while fileNb < (*filenames).tableSize {
                                                                if showDefaultCParams != 0 {
                                                                    printDefaultCParams(
                                                                        *((*filenames).fileNames).add(fileNb),
                                                                        dictFileName,
                                                                        cLevel,
                                                                    );
                                                                }
                                                                if g_displayLevel >= 4 {
                                                                    printActualCParams(
                                                                        *((*filenames).fileNames).add(fileNb),
                                                                        dictFileName,
                                                                        cLevel,
                                                                        &mut compressionParams,
                                                                    );
                                                                }
                                                                fileNb = fileNb.wrapping_add(1);
                                                            }
                                                        }
                                                        if g_displayLevel >= 4 {
                                                            FIO_displayCompressionParameters(prefs);
                                                        }
                                                        if (*filenames).tableSize == 1
                                                            && !outFileName.is_null()
                                                        {
                                                            operationResult = FIO_compressFilename(
                                                                fCtx,
                                                                prefs,
                                                                outFileName,
                                                                *((*filenames).fileNames).offset(0),
                                                                dictFileName,
                                                                cLevel,
                                                                compressionParams,
                                                            );
                                                        } else {
                                                            operationResult =
                                                                FIO_compressMultipleFilenames(
                                                                    fCtx,
                                                                    prefs,
                                                                    (*filenames).fileNames,
                                                                    outMirroredDirName,
                                                                    outDirName,
                                                                    outFileName,
                                                                    suffix,
                                                                    dictFileName,
                                                                    cLevel,
                                                                    compressionParams,
                                                                );
                                                        }
                                                    } else if (*filenames).tableSize == 1
                                                        && !outFileName.is_null()
                                                    {
                                                        operationResult = FIO_decompressFilename(
                                                            fCtx,
                                                            prefs,
                                                            outFileName,
                                                            *((*filenames).fileNames).offset(0),
                                                            dictFileName,
                                                        );
                                                    } else {
                                                        operationResult =
                                                            FIO_decompressMultipleFilenames(
                                                                fCtx,
                                                                prefs,
                                                                (*filenames).fileNames,
                                                                outMirroredDirName,
                                                                outDirName,
                                                                outFileName,
                                                                dictFileName,
                                                            );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    FIO_freePreferences(prefs);
    FIO_freeContext(fCtx);
    if main_pause != 0 {
        waitEnter();
    }
    UTIL_freeFileNamesTable(filenames);
    UTIL_freeFileNamesTable(file_of_names);
    TRACE_finish();
    operationResult
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::ffi::c_int,
            args.as_mut_ptr() as *mut *const std::ffi::c_char,
        ) as i32)
    }
}
