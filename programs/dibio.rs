extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> std::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn fread(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn fwrite(
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn exit(_: std::ffi::c_int) -> !;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strerror(_: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn UTIL_getFileSize(infilename: *const std::ffi::c_char) -> U64;
    fn __errno_location() -> *mut std::ffi::c_int;
    fn UTIL_getTime() -> UTIL_time_t;
    fn UTIL_clockSpanMicro(clockStart: UTIL_time_t) -> PTime;
    fn ZDICT_isError(errorCode: size_t) -> std::ffi::c_uint;
    fn ZDICT_getErrorName(errorCode: size_t) -> *const std::ffi::c_char;
    fn ZDICT_trainFromBuffer_cover(
        dictBuffer: *mut std::ffi::c_void,
        dictBufferCapacity: size_t,
        samplesBuffer: *const std::ffi::c_void,
        samplesSizes: *const size_t,
        nbSamples: std::ffi::c_uint,
        parameters: ZDICT_cover_params_t,
    ) -> size_t;
    fn ZDICT_optimizeTrainFromBuffer_cover(
        dictBuffer: *mut std::ffi::c_void,
        dictBufferCapacity: size_t,
        samplesBuffer: *const std::ffi::c_void,
        samplesSizes: *const size_t,
        nbSamples: std::ffi::c_uint,
        parameters: *mut ZDICT_cover_params_t,
    ) -> size_t;
    fn ZDICT_trainFromBuffer_fastCover(
        dictBuffer: *mut std::ffi::c_void,
        dictBufferCapacity: size_t,
        samplesBuffer: *const std::ffi::c_void,
        samplesSizes: *const size_t,
        nbSamples: std::ffi::c_uint,
        parameters: ZDICT_fastCover_params_t,
    ) -> size_t;
    fn ZDICT_optimizeTrainFromBuffer_fastCover(
        dictBuffer: *mut std::ffi::c_void,
        dictBufferCapacity: size_t,
        samplesBuffer: *const std::ffi::c_void,
        samplesSizes: *const size_t,
        nbSamples: std::ffi::c_uint,
        parameters: *mut ZDICT_fastCover_params_t,
    ) -> size_t;
    fn ZDICT_trainFromBuffer_legacy(
        dictBuffer: *mut std::ffi::c_void,
        dictBufferCapacity: size_t,
        samplesBuffer: *const std::ffi::c_void,
        samplesSizes: *const size_t,
        nbSamples: std::ffi::c_uint,
        parameters: ZDICT_legacy_params_t,
    ) -> size_t;
}
pub type __uint32_t = std::ffi::c_uint;
pub type __int64_t = std::ffi::c_long;
pub type __uint64_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
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
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type S64 = int64_t;
pub type PTime = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
pub type C2RustUnnamed = std::ffi::c_uint;
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed = 74;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed = 72;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed = 22;
pub const ZSTD_error_corruption_detected: C2RustUnnamed = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed = 12;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed = 10;
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
pub const ZSTD_error_no_error: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: std::ffi::c_int,
    pub notificationLevel: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_cover_params_t {
    pub k: std::ffi::c_uint,
    pub d: std::ffi::c_uint,
    pub steps: std::ffi::c_uint,
    pub nbThreads: std::ffi::c_uint,
    pub splitPoint: std::ffi::c_double,
    pub shrinkDict: std::ffi::c_uint,
    pub shrinkDictMaxRegression: std::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_fastCover_params_t {
    pub k: std::ffi::c_uint,
    pub d: std::ffi::c_uint,
    pub f: std::ffi::c_uint,
    pub steps: std::ffi::c_uint,
    pub nbThreads: std::ffi::c_uint,
    pub splitPoint: std::ffi::c_double,
    pub accel: std::ffi::c_uint,
    pub shrinkDict: std::ffi::c_uint,
    pub shrinkDictMaxRegression: std::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_legacy_params_t {
    pub selectivityLevel: std::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileStats {
    pub totalSizeToLoad: S64,
    pub nbSamples: std::ffi::c_int,
    pub oneSampleTooLarge: std::ffi::c_int,
}
pub const UTIL_FILESIZE_UNKNOWN: std::ffi::c_int = -(1 as std::ffi::c_int);
pub const SEC_TO_MICRO: std::ffi::c_int = 1000000 as std::ffi::c_int;
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const SAMPLESIZE_MAX: std::ffi::c_int =
    128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int);
pub const MEMMULT: std::ffi::c_int = 11 as std::ffi::c_int;
pub const COVER_MEMMULT: std::ffi::c_int = 9 as std::ffi::c_int;
pub const FASTCOVER_MEMMULT: std::ffi::c_int = 1 as std::ffi::c_int;
static mut g_maxMemory: size_t = 0;
pub const NOISELENGTH: std::ffi::c_int = 32 as std::ffi::c_int;
static mut g_refreshRate: U64 = 0;
static mut g_displayClock: UTIL_time_t = {
    
    UTIL_time_t {
        t: 0 as std::ffi::c_int as PTime,
    }
};
pub const DEBUG: std::ffi::c_int = 0 as std::ffi::c_int;
unsafe extern "C" fn DiB_getFileSize(mut fileName: *const std::ffi::c_char) -> S64 {
    let fileSize = UTIL_getFileSize(fileName);
    if fileSize == UTIL_FILESIZE_UNKNOWN as U64 {
        -(1 as std::ffi::c_int) as S64
    } else {
        fileSize as S64
    }
}
unsafe extern "C" fn DiB_loadFiles(
    mut buffer: *mut std::ffi::c_void,
    mut bufferSizePtr: *mut size_t,
    mut sampleSizes: *mut size_t,
    mut sstSize: std::ffi::c_int,
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_int,
    mut targetChunkSize: size_t,
    mut displayLevel: std::ffi::c_int,
) -> std::ffi::c_int {
    let buff = buffer as *mut std::ffi::c_char;
    let mut totalDataLoaded = 0 as std::ffi::c_int as size_t;
    let mut nbSamplesLoaded = 0 as std::ffi::c_int;
    let mut fileIndex = 0 as std::ffi::c_int;
    let mut f = NULL as *mut FILE;
    while nbSamplesLoaded < sstSize && fileIndex < nbFiles {
        let mut fileDataLoaded: size_t = 0;
        let fileSize = DiB_getFileSize(*fileNamesTable.offset(fileIndex as isize));
        if fileSize <= 0 as std::ffi::c_int as S64 {
            fileIndex += 1;
            fileIndex;
        } else {
            f = fopen(
                *fileNamesTable.offset(fileIndex as isize),
                b"rb\0" as *const u8 as *const std::ffi::c_char,
            );
            if f.is_null() {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
                    10 as std::ffi::c_int,
                );
                fprintf(
                    stderr,
                    b"zstd: dictBuilder: %s %s \0" as *const u8 as *const std::ffi::c_char,
                    *fileNamesTable.offset(fileIndex as isize),
                    strerror(*__errno_location()),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
                exit(10 as std::ffi::c_int);
            }
            if displayLevel >= 2 as std::ffi::c_int
                && (UTIL_clockSpanMicro(g_displayClock) > g_refreshRate
                    || displayLevel >= 4 as std::ffi::c_int)
                {
                    g_displayClock = UTIL_getTime();
                    fprintf(
                        stderr,
                        b"Loading %s...       \r\0" as *const u8 as *const std::ffi::c_char,
                        *fileNamesTable.offset(fileIndex as isize),
                    );
                    if displayLevel >= 4 as std::ffi::c_int {
                        fflush(stderr);
                    }
                }
            fileDataLoaded = if targetChunkSize > 0 as std::ffi::c_int as size_t {
                (if fileSize < targetChunkSize as S64 {
                    fileSize
                } else {
                    targetChunkSize as S64
                }) as size_t
            } else {
                (if fileSize
                    < (128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int))
                        as S64
                {
                    fileSize
                } else {
                    (128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int))
                        as S64
                }) as size_t
            };
            if totalDataLoaded.wrapping_add(fileDataLoaded) > *bufferSizePtr {
                break;
            }
            if fread(
                buff.offset(totalDataLoaded as isize) as *mut std::ffi::c_void,
                1 as std::ffi::c_int as std::ffi::c_ulong,
                fileDataLoaded,
                f,
            ) != fileDataLoaded
            {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
                    11 as std::ffi::c_int,
                );
                fprintf(
                    stderr,
                    b"Pb reading %s\0" as *const u8 as *const std::ffi::c_char,
                    *fileNamesTable.offset(fileIndex as isize),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
                exit(11 as std::ffi::c_int);
            }
            let fresh0 = nbSamplesLoaded;
            nbSamplesLoaded += 1;
            *sampleSizes.offset(fresh0 as isize) = fileDataLoaded;
            totalDataLoaded = totalDataLoaded.wrapping_add(fileDataLoaded);
            if targetChunkSize > 0 as std::ffi::c_int as size_t {
                while (fileDataLoaded as S64) < fileSize && nbSamplesLoaded < sstSize {
                    let chunkSize =
                        if (fileSize as size_t).wrapping_sub(fileDataLoaded) < targetChunkSize {
                            (fileSize as size_t).wrapping_sub(fileDataLoaded)
                        } else {
                            targetChunkSize
                        };
                    if totalDataLoaded.wrapping_add(chunkSize) > *bufferSizePtr {
                        break;
                    }
                    if fread(
                        buff.offset(totalDataLoaded as isize) as *mut std::ffi::c_void,
                        1 as std::ffi::c_int as std::ffi::c_ulong,
                        chunkSize,
                        f,
                    ) != chunkSize
                    {
                        fprintf(
                            stderr,
                            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
                            11 as std::ffi::c_int,
                        );
                        fprintf(
                            stderr,
                            b"Pb reading %s\0" as *const u8 as *const std::ffi::c_char,
                            *fileNamesTable.offset(fileIndex as isize),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
                        exit(11 as std::ffi::c_int);
                    }
                    let fresh1 = nbSamplesLoaded;
                    nbSamplesLoaded += 1;
                    *sampleSizes.offset(fresh1 as isize) = chunkSize;
                    totalDataLoaded = totalDataLoaded.wrapping_add(chunkSize);
                    fileDataLoaded = fileDataLoaded.wrapping_add(chunkSize);
                }
            }
            fileIndex += 1 as std::ffi::c_int;
            fclose(f);
            f = NULL as *mut FILE;
        }
    }
    if !f.is_null() {
        fclose(f);
    }
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    if displayLevel >= 4 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Loaded %d KB total training data, %d nb samples \n\0" as *const u8
                as *const std::ffi::c_char,
            (totalDataLoaded
                / (1 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int))
                    as size_t) as std::ffi::c_int,
            nbSamplesLoaded,
        );
    }
    *bufferSizePtr = totalDataLoaded;
    nbSamplesLoaded
}
unsafe extern "C" fn DiB_rand(mut src: *mut U32) -> U32 {
    static mut prime1: U32 = 2654435761 as std::ffi::c_uint;
    static mut prime2: U32 = 2246822519 as std::ffi::c_uint;
    let mut rand32 = *src;
    rand32 *= prime1;
    rand32 ^= prime2;
    rand32 =
        rand32 << 13 as std::ffi::c_int | rand32 >> (32 as std::ffi::c_int - 13 as std::ffi::c_int);
    *src = rand32;
    rand32 >> 5 as std::ffi::c_int
}
unsafe extern "C" fn DiB_shuffle(
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_uint,
) {
    let mut seed = 0xfd2fb528 as std::ffi::c_uint;
    let mut i: std::ffi::c_uint = 0;
    if nbFiles == 0 as std::ffi::c_int as std::ffi::c_uint {
        return;
    }
    i = nbFiles.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    while i > 0 as std::ffi::c_int as std::ffi::c_uint {
        let j = (DiB_rand(&mut seed))
            .wrapping_rem(i.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint));
        let tmp = *fileNamesTable.offset(j as isize);
        let fresh2 = &mut (*fileNamesTable.offset(j as isize));
        *fresh2 = *fileNamesTable.offset(i as isize);
        let fresh3 = &mut (*fileNamesTable.offset(i as isize));
        *fresh3 = tmp;
        i = i.wrapping_sub(1);
        i;
    }
}
unsafe extern "C" fn DiB_findMaxMem(mut requiredMem: std::ffi::c_ulonglong) -> size_t {
    let step = (8 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int)) as size_t;
    let mut testmem = NULL as *mut std::ffi::c_void;
    requiredMem = (requiredMem >> 23 as std::ffi::c_int)
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulonglong)
        << 23 as std::ffi::c_int;
    requiredMem = requiredMem.wrapping_add(step as std::ffi::c_ulonglong);
    if requiredMem > g_maxMemory as std::ffi::c_ulonglong {
        requiredMem = g_maxMemory as std::ffi::c_ulonglong;
    }
    while testmem.is_null() {
        testmem = malloc(requiredMem as size_t);
        requiredMem = requiredMem.wrapping_sub(step as std::ffi::c_ulonglong);
    }
    free(testmem);
    requiredMem as size_t
}
unsafe extern "C" fn DiB_fillNoise(mut buffer: *mut std::ffi::c_void, mut length: size_t) {
    let prime1 = 2654435761 as std::ffi::c_uint;
    let prime2 = 2246822519 as std::ffi::c_uint;
    let mut acc = prime1;
    let mut p = 0 as std::ffi::c_int as size_t;
    p = 0 as std::ffi::c_int as size_t;
    while p < length {
        acc = acc.wrapping_mul(prime2);
        *(buffer as *mut std::ffi::c_uchar).offset(p as isize) =
            (acc >> 21 as std::ffi::c_int) as std::ffi::c_uchar;
        p = p.wrapping_add(1);
        p;
    }
}
unsafe extern "C" fn DiB_saveDict(
    mut dictFileName: *const std::ffi::c_char,
    mut buff: *const std::ffi::c_void,
    mut buffSize: size_t,
) {
    let f = fopen(
        dictFileName,
        b"wb\0" as *const u8 as *const std::ffi::c_char,
    );
    if f.is_null() {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            3 as std::ffi::c_int,
        );
        fprintf(
            stderr,
            b"cannot open %s \0" as *const u8 as *const std::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(3 as std::ffi::c_int);
    }
    let n = fwrite(buff, 1 as std::ffi::c_int as std::ffi::c_ulong, buffSize, f);
    if n != buffSize {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            4 as std::ffi::c_int,
        );
        fprintf(
            stderr,
            b"%s : write error\0" as *const u8 as *const std::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(4 as std::ffi::c_int);
    }
    let n_0 = fclose(f) as size_t;
    if n_0 != 0 as std::ffi::c_int as size_t {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            5 as std::ffi::c_int,
        );
        fprintf(
            stderr,
            b"%s : flush error\0" as *const u8 as *const std::ffi::c_char,
            dictFileName,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(5 as std::ffi::c_int);
    }
}
unsafe extern "C" fn DiB_fileStats(
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_int,
    mut chunkSize: size_t,
    mut displayLevel: std::ffi::c_int,
) -> fileStats {
    let mut fs = fileStats {
        totalSizeToLoad: 0,
        nbSamples: 0,
        oneSampleTooLarge: 0,
    };
    let mut n: std::ffi::c_int = 0;
    memset(
        &mut fs as *mut fileStats as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<fileStats>() as std::ffi::c_ulong,
    );
    n = 0 as std::ffi::c_int;
    while n < nbFiles {
        let fileSize = DiB_getFileSize(*fileNamesTable.offset(n as isize));
        if fileSize == 0 as std::ffi::c_int as S64 {
            if displayLevel >= 3 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Sample file '%s' has zero size, skipping...\n\0" as *const u8
                        as *const std::ffi::c_char,
                    *fileNamesTable.offset(n as isize),
                );
            }
        } else if chunkSize > 0 as std::ffi::c_int as size_t {
            fs.nbSamples += ((fileSize as size_t)
                .wrapping_add(chunkSize)
                .wrapping_sub(1 as std::ffi::c_int as size_t)
                / chunkSize) as std::ffi::c_int;
            fs.totalSizeToLoad += fileSize;
        } else {
            if fileSize > SAMPLESIZE_MAX as S64 {
                fs.oneSampleTooLarge |=
                    (fileSize > (2 as std::ffi::c_int * SAMPLESIZE_MAX) as S64) as std::ffi::c_int;
                if displayLevel >= 3 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Sample file '%s' is too large, limiting to %d KB\n\0" as *const u8
                            as *const std::ffi::c_char,
                        *fileNamesTable.offset(n as isize),
                        128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)
                            / (1 as std::ffi::c_int
                                * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)),
                    );
                }
            }
            fs.nbSamples += 1 as std::ffi::c_int;
            fs.totalSizeToLoad += if fileSize
                < (128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int))
                    as S64
            {
                fileSize
            } else {
                (128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as S64
            };
        }
        n += 1;
        n;
    }
    if displayLevel >= 4 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Found training data %d files, %d KB, %d samples\n\0" as *const u8
                as *const std::ffi::c_char,
            nbFiles,
            (fs.totalSizeToLoad
                / (1 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as S64)
                as std::ffi::c_int,
            fs.nbSamples,
        );
    }
    fs
}
#[no_mangle]
pub unsafe extern "C" fn DiB_trainFromFiles(
    mut dictFileName: *const std::ffi::c_char,
    mut maxDictSize: size_t,
    mut fileNamesTable: *mut *const std::ffi::c_char,
    mut nbFiles: std::ffi::c_int,
    mut chunkSize: size_t,
    mut params: *mut ZDICT_legacy_params_t,
    mut coverParams: *mut ZDICT_cover_params_t,
    mut fastCoverParams: *mut ZDICT_fastCover_params_t,
    mut optimize: std::ffi::c_int,
    mut memLimit: std::ffi::c_uint,
) -> std::ffi::c_int {
    let mut fs = fileStats {
        totalSizeToLoad: 0,
        nbSamples: 0,
        oneSampleTooLarge: 0,
    };
    let mut sampleSizes = std::ptr::null_mut::<size_t>();
    let mut nbSamplesLoaded: std::ffi::c_int = 0;
    let mut loadedSize: size_t = 0;
    let mut srcBuffer = std::ptr::null_mut::<std::ffi::c_void>();
    let dictBuffer = malloc(maxDictSize);
    let mut result = 0 as std::ffi::c_int;
    let displayLevel = (if !params.is_null() {
        (*params).zParams.notificationLevel
    } else if !coverParams.is_null() {
        (*coverParams).zParams.notificationLevel
    } else if !fastCoverParams.is_null() {
        (*fastCoverParams).zParams.notificationLevel
    } else {
        0 as std::ffi::c_int as std::ffi::c_uint
    }) as std::ffi::c_int;
    if displayLevel >= 3 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Shuffling input files\n\0" as *const u8 as *const std::ffi::c_char,
        );
    }
    DiB_shuffle(fileNamesTable, nbFiles as std::ffi::c_uint);
    fs = DiB_fileStats(fileNamesTable, nbFiles, chunkSize, displayLevel);
    let memMult = if !params.is_null() {
        MEMMULT
    } else if !coverParams.is_null() {
        COVER_MEMMULT
    } else {
        FASTCOVER_MEMMULT
    };
    let maxMem = DiB_findMaxMem((fs.totalSizeToLoad * memMult as S64) as std::ffi::c_ulonglong)
        / memMult as size_t;
    loadedSize = (if (if (maxMem as S64) < fs.totalSizeToLoad {
        maxMem as S64
    } else {
        fs.totalSizeToLoad
    }) < (2 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int)
        as S64
    {
        if (maxMem as S64) < fs.totalSizeToLoad {
            maxMem as S64
        } else {
            fs.totalSizeToLoad
        }
    } else {
        (2 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int) as S64
    }) as size_t;
    if memLimit != 0 as std::ffi::c_int as std::ffi::c_uint {
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Warning : setting manual memory limit for dictionary training data at %u MB \n\0"
                    as *const u8 as *const std::ffi::c_char,
                memLimit
                    .wrapping_div(
                        (1 as std::ffi::c_int
                            * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int))
                            as std::ffi::c_uint,
                    ),
            );
        }
        loadedSize = if loadedSize < memLimit as size_t {
            loadedSize
        } else {
            memLimit as size_t
        };
    }
    srcBuffer = malloc(loadedSize.wrapping_add(NOISELENGTH as size_t));
    sampleSizes = malloc(
        (fs.nbSamples as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as std::ffi::c_ulong),
    ) as *mut size_t;
    if fs.nbSamples != 0 && sampleSizes.is_null() || srcBuffer.is_null() || dictBuffer.is_null() {
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            12 as std::ffi::c_int,
        );
        fprintf(
            stderr,
            b"not enough memory for DiB_trainFiles\0" as *const u8 as *const std::ffi::c_char,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(12 as std::ffi::c_int);
    }
    if fs.oneSampleTooLarge != 0 {
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Warning : some sample(s) are very large \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Note that dictionary is only useful for small samples. \n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  As a consequence, only the first %u bytes of each sample are loaded \n\0"
                    as *const u8 as *const std::ffi::c_char,
                128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int),
            );
        }
    }
    if fs.nbSamples < 5 as std::ffi::c_int {
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Warning : nb of samples too low for proper processing !\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Please provide _one file per sample_.\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Alternatively, split file(s) into fixed-size samples, with --split=#\n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        fprintf(
            stderr,
            b"Error %i : \0" as *const u8 as *const std::ffi::c_char,
            14 as std::ffi::c_int,
        );
        fprintf(
            stderr,
            b"nb of samples too low\0" as *const u8 as *const std::ffi::c_char,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const std::ffi::c_char);
        exit(14 as std::ffi::c_int);
    }
    if fs.totalSizeToLoad < maxDictSize as S64 * 8 as std::ffi::c_int as S64 {
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Warning : data size of samples too small for target dictionary size \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"!  Samples should be about 100x larger than target dictionary size \n\0"
                    as *const u8 as *const std::ffi::c_char,
            );
        }
    }
    if (loadedSize as S64) < fs.totalSizeToLoad
        && displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Training samples set too large (%u MB); training on %u MB only...\n\0"
                    as *const u8 as *const std::ffi::c_char,
                (fs.totalSizeToLoad
                    / (1 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int))
                        as S64) as std::ffi::c_uint,
                (loadedSize
                    / (1 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int))
                        as size_t) as std::ffi::c_uint,
            );
        }
    nbSamplesLoaded = DiB_loadFiles(
        srcBuffer,
        &mut loadedSize,
        sampleSizes,
        fs.nbSamples,
        fileNamesTable,
        nbFiles,
        chunkSize,
        displayLevel,
    );
    let mut dictSize = ZSTD_error_GENERIC as std::ffi::c_int as size_t;
    if !params.is_null() {
        DiB_fillNoise(
            (srcBuffer as *mut std::ffi::c_char).offset(loadedSize as isize)
                as *mut std::ffi::c_void,
            NOISELENGTH as size_t,
        );
        dictSize = ZDICT_trainFromBuffer_legacy(
            dictBuffer,
            maxDictSize,
            srcBuffer,
            sampleSizes,
            nbSamplesLoaded as std::ffi::c_uint,
            *params,
        );
    } else if !coverParams.is_null() {
        if optimize != 0 {
            dictSize = ZDICT_optimizeTrainFromBuffer_cover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as std::ffi::c_uint,
                coverParams,
            );
            if ZDICT_isError(dictSize) == 0 {
                let mut splitPercentage = ((*coverParams).splitPoint
                    * 100 as std::ffi::c_int as std::ffi::c_double)
                    as std::ffi::c_uint;
                if displayLevel >= 2 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"k=%u\nd=%u\nsteps=%u\nsplit=%u\n\0" as *const u8
                            as *const std::ffi::c_char,
                        (*coverParams).k,
                        (*coverParams).d,
                        (*coverParams).steps,
                        splitPercentage,
                    );
                }
            }
        } else {
            dictSize = ZDICT_trainFromBuffer_cover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as std::ffi::c_uint,
                *coverParams,
            );
        }
    } else if !fastCoverParams.is_null() {
        if optimize != 0 {
            dictSize = ZDICT_optimizeTrainFromBuffer_fastCover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as std::ffi::c_uint,
                fastCoverParams,
            );
            if ZDICT_isError(dictSize) == 0 {
                let mut splitPercentage_0 = ((*fastCoverParams).splitPoint
                    * 100 as std::ffi::c_int as std::ffi::c_double)
                    as std::ffi::c_uint;
                if displayLevel >= 2 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"k=%u\nd=%u\nf=%u\nsteps=%u\nsplit=%u\naccel=%u\n\0" as *const u8
                            as *const std::ffi::c_char,
                        (*fastCoverParams).k,
                        (*fastCoverParams).d,
                        (*fastCoverParams).f,
                        (*fastCoverParams).steps,
                        splitPercentage_0,
                        (*fastCoverParams).accel,
                    );
                }
            }
        } else {
            dictSize = ZDICT_trainFromBuffer_fastCover(
                dictBuffer,
                maxDictSize,
                srcBuffer,
                sampleSizes,
                nbSamplesLoaded as std::ffi::c_uint,
                *fastCoverParams,
            );
        }
    }
    if ZDICT_isError(dictSize) != 0 {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"dictionary training failed : %s \n\0" as *const u8 as *const std::ffi::c_char,
                ZDICT_getErrorName(dictSize),
            );
        }
        result = 1 as std::ffi::c_int;
    } else {
        if displayLevel >= 2 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Save dictionary of size %u into file %s \n\0" as *const u8
                    as *const std::ffi::c_char,
                dictSize as std::ffi::c_uint,
                dictFileName,
            );
        }
        DiB_saveDict(dictFileName, dictBuffer, dictSize);
    }
    free(srcBuffer);
    free(sampleSizes as *mut std::ffi::c_void);
    free(dictBuffer);
    result
}
unsafe extern "C" fn run_static_initializers() {
    g_maxMemory = if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong
    {
        (2 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int)
            .wrapping_sub(
                (64 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int))
                    as std::ffi::c_uint,
            ) as size_t
    } else {
        ((512 as std::ffi::c_int * ((1 as std::ffi::c_int) << 20 as std::ffi::c_int)) as size_t)
            << ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
    };
    g_refreshRate = SEC_TO_MICRO as PTime / 6 as std::ffi::c_int as PTime;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
