use libc::{
    __errno_location, exit, fclose, feof, ferror, fprintf, pthread_cond_destroy, pthread_cond_init,
    pthread_cond_signal, pthread_cond_t, pthread_cond_wait, pthread_condattr_t,
    pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
    pthread_mutex_unlock, pthread_mutexattr_t, strerror, FILE,
};
use libzstd_rs::lib::common::pool::{
    POOL_add, POOL_create, POOL_ctx, POOL_free, POOL_function, POOL_joinJobs,
};

use crate::fileio::g_display_prefs;

extern "C" {
    static mut stderr: *mut FILE;
    fn fread(
        _: *mut core::ffi::c_void,
        _: core::ffi::c_ulong,
        _: core::ffi::c_ulong,
        _: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fwrite(
        _: *const core::ffi::c_void,
        _: core::ffi::c_ulong,
        _: core::ffi::c_ulong,
        _: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: core::ffi::c_long,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn malloc(_: core::ffi::c_ulong) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: core::ffi::c_ulong,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = core::ffi::c_ulong;
pub type ZSTD_ParamSwitch_e = core::ffi::c_uint;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: core::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: core::ffi::c_uint,
    pub __high: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOPoolCtx_t {
    pub threadPool: *mut POOL_ctx,
    pub threadPoolActive: core::ffi::c_int,
    pub totalIoJobs: core::ffi::c_int,
    pub prefs: *const FIO_prefs_t,
    pub poolFunction: POOL_function,
    pub file: *mut FILE,
    pub ioJobsMutex: pthread_mutex_t,
    pub availableJobs: [*mut core::ffi::c_void; 10],
    pub availableJobsCount: core::ffi::c_int,
    pub jobBufferSize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReadPoolCtx_t {
    pub base: IOPoolCtx_t,
    pub reachedEof: core::ffi::c_int,
    pub nextReadOffset: u64,
    pub waitingOnOffset: u64,
    pub currentJobHeld: *mut core::ffi::c_void,
    pub coalesceBuffer: *mut u8,
    pub srcBuffer: *mut u8,
    pub srcBufferLoaded: size_t,
    pub completedJobs: [*mut core::ffi::c_void; 10],
    pub completedJobsCount: core::ffi::c_int,
    pub jobCompletedCond: pthread_cond_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WritePoolCtx_t {
    pub base: IOPoolCtx_t,
    pub storedSkips: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOJob_t {
    pub ctx: *mut core::ffi::c_void,
    pub file: *mut FILE,
    pub buffer: *mut core::ffi::c_void,
    pub bufferSize: size_t,
    pub usedBufferSize: size_t,
    pub offset: u64,
}
pub const SEEK_CUR: core::ffi::c_int = 1;
pub const MAX_IO_JOBS: core::ffi::c_int = 10;
pub const LONG_SEEK: unsafe extern "C" fn(
    *mut FILE,
    core::ffi::c_long,
    core::ffi::c_int,
) -> core::ffi::c_int = fseek;
pub const NULL: core::ffi::c_int = 0;
static mut segmentSizeT: size_t = 0;
static mut maskT: size_t = 0;
unsafe extern "C" fn AIO_fwriteSparse(
    mut file: *mut FILE,
    mut buffer: *const core::ffi::c_void,
    mut bufferSize: size_t,
    prefs: *const FIO_prefs_t,
    mut storedSkips: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let bufferT = buffer as *const size_t;
    let mut bufferSizeT =
        bufferSize.wrapping_div(::core::mem::size_of::<size_t>() as core::ffi::c_ulong);
    let bufferTEnd = bufferT.offset(bufferSizeT as isize);
    let mut ptrT = bufferT;
    if (*prefs).testMode != 0 {
        return 0;
    }
    if (*prefs).sparseFileSupport == 0 {
        let sizeCheck = fwrite(buffer, 1, bufferSize, file);
        if sizeCheck != bufferSize {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    50,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    70,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Write error : cannot write block : %s\0" as *const u8
                        as *const core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(70);
        }
        return 0;
    }
    if storedSkips
        > (1 as core::ffi::c_int as core::ffi::c_uint)
            .wrapping_mul((1 as core::ffi::c_uint) << 30 as core::ffi::c_int)
    {
        if fseek(
            file,
            (1 as core::ffi::c_int as core::ffi::c_uint)
                .wrapping_mul((1 as core::ffi::c_uint) << 30 as core::ffi::c_int)
                as core::ffi::c_long,
            SEEK_CUR,
        ) != 0
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    57,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    91,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"1 GB skip error (sparse file support)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(91);
        }
        storedSkips = storedSkips.wrapping_sub(
            (1 as core::ffi::c_int as core::ffi::c_uint)
                .wrapping_mul((1 as core::ffi::c_uint) << 30 as core::ffi::c_int),
        );
    }
    while ptrT < bufferTEnd {
        let mut nb0T: size_t = 0;
        let mut seg0SizeT = segmentSizeT;
        if seg0SizeT > bufferSizeT {
            seg0SizeT = bufferSizeT;
        }
        bufferSizeT = bufferSizeT.wrapping_sub(seg0SizeT);
        nb0T = 0;
        while nb0T < seg0SizeT && *ptrT.offset(nb0T as isize) == 0 {
            nb0T = nb0T.wrapping_add(1);
        }
        storedSkips = storedSkips.wrapping_add(
            nb0T.wrapping_mul(::core::mem::size_of::<size_t>() as core::ffi::c_ulong)
                as core::ffi::c_uint,
        );
        if nb0T != seg0SizeT {
            let nbNon0ST = seg0SizeT.wrapping_sub(nb0T);
            if fseek(file, storedSkips as core::ffi::c_long, SEEK_CUR) != 0 {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const core::ffi::c_char,
                        b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                        77,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                        92,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Sparse skip error ; try --no-sparse\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                }
                exit(92);
            }
            storedSkips = 0;
            if fwrite(
                ptrT.offset(nb0T as isize) as *const core::ffi::c_void,
                ::core::mem::size_of::<size_t>() as core::ffi::c_ulong,
                nbNon0ST,
                file,
            ) != nbNon0ST
            {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const core::ffi::c_char,
                        b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                        82,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                        93,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Write error : cannot write block : %s\0" as *const u8
                            as *const core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                }
                exit(93);
            }
        }
        ptrT = ptrT.offset(seg0SizeT as isize);
    }
    if bufferSize & maskT != 0 {
        let restStart = bufferTEnd as *const core::ffi::c_char;
        let mut restPtr = restStart;
        let restEnd = (buffer as *const core::ffi::c_char).offset(bufferSize as isize);
        if restEnd > restStart
            && restEnd
                < restStart.offset(::core::mem::size_of::<size_t>() as core::ffi::c_ulong as isize)
        {
        } else {
            __assert_fail(
                b"restEnd > restStart && restEnd < restStart + sizeof(size_t)\0"
                    as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                93,
                (*::core::mem::transmute::<
                    &[u8; 100],
                    &[core::ffi::c_char; 100],
                >(
                    b"unsigned int AIO_fwriteSparse(FILE *, const void *, size_t, const FIO_prefs_t *const, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
        while restPtr < restEnd && *restPtr as core::ffi::c_int == 0 {
            restPtr = restPtr.offset(1);
        }
        storedSkips = storedSkips
            .wrapping_add(restPtr.offset_from(restStart) as core::ffi::c_long as core::ffi::c_uint);
        if restPtr != restEnd {
            let restSize = restEnd.offset_from(restPtr) as core::ffi::c_long as size_t;
            if fseek(file, storedSkips as core::ffi::c_long, SEEK_CUR) != 0 {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const core::ffi::c_char,
                        b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                        100,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                        92,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Sparse skip error ; try --no-sparse\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                }
                exit(92);
            }
            if fwrite(restPtr as *const core::ffi::c_void, 1, restSize, file) != restSize {
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
                }
                if g_display_prefs.displayLevel >= 5 {
                    fprintf(
                        stderr,
                        b"Error defined at %s, line %i : \n\0" as *const u8
                            as *const core::ffi::c_char,
                        b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                        103,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                        95,
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Write error : cannot write end of decoded block : %s\0" as *const u8
                            as *const core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                }
                if g_display_prefs.displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                }
                exit(95);
            }
            storedSkips = 0;
        }
    }
    storedSkips
}
unsafe extern "C" fn AIO_fwriteSparseEnd(
    prefs: *const FIO_prefs_t,
    mut file: *mut FILE,
    mut storedSkips: core::ffi::c_uint,
) {
    if (*prefs).testMode != 0 {
        if storedSkips == 0 {
        } else {
            __assert_fail(
                b"storedSkips == 0\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                113,
                (*::core::mem::transmute::<&[u8; 73], &[core::ffi::c_char; 73]>(
                    b"void AIO_fwriteSparseEnd(const FIO_prefs_t *const, FILE *, unsigned int)\0",
                ))
                .as_ptr(),
            );
        }
    }
    if storedSkips > 0 {
        if (*prefs).sparseFileSupport > 0 {
        } else {
            __assert_fail(
                b"prefs->sparseFileSupport > 0\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                115,
                (*::core::mem::transmute::<&[u8; 73], &[core::ffi::c_char; 73]>(
                    b"void AIO_fwriteSparseEnd(const FIO_prefs_t *const, FILE *, unsigned int)\0",
                ))
                .as_ptr(),
            );
        }
        if fseek(
            file,
            storedSkips.wrapping_sub(1) as core::ffi::c_long,
            SEEK_CUR,
        ) != 0
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    118,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    69,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Final skip error (sparse file support)\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(69);
        }
        let lastZeroByte: [core::ffi::c_char; 1] = [0; 1];
        if fwrite(
            lastZeroByte.as_ptr() as *const core::ffi::c_void,
            1,
            1,
            file,
        ) != 1
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    123,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    69,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Write error : cannot write last zero : %s\0" as *const u8
                        as *const core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(69);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn AIO_supported() -> core::ffi::c_int {
    1
}
unsafe extern "C" fn AIO_IOPool_createIoJob(
    mut ctx: *mut IOPoolCtx_t,
    mut bufferSize: size_t,
) -> *mut IOJob_t {
    let job = malloc(::core::mem::size_of::<IOJob_t>() as core::ffi::c_ulong) as *mut IOJob_t;
    let buffer = malloc(bufferSize);
    if job.is_null() || buffer.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                150,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                101,
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
        exit(101);
    }
    (*job).buffer = buffer;
    (*job).bufferSize = bufferSize;
    (*job).usedBufferSize = 0;
    (*job).file = NULL as *mut FILE;
    (*job).ctx = ctx as *mut core::ffi::c_void;
    (*job).offset = 0;
    job
}
unsafe extern "C" fn AIO_IOPool_createThreadPool(
    mut ctx: *mut IOPoolCtx_t,
    mut prefs: *const FIO_prefs_t,
) {
    (*ctx).threadPool = NULL as *mut POOL_ctx;
    (*ctx).threadPoolActive = 0;
    if (*prefs).asyncIO != 0 {
        if pthread_mutex_init(
            &mut (*ctx).ioJobsMutex,
            core::ptr::null::<pthread_mutexattr_t>(),
        ) != 0
        {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    169,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    102,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Failed creating ioJobsMutex mutex\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(102);
        }
        if 10 >= 2 {
        } else {
            __assert_fail(
                b"MAX_IO_JOBS >= 2\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                172,
                (*::core::mem::transmute::<&[u8; 69], &[core::ffi::c_char; 69]>(
                    b"void AIO_IOPool_createThreadPool(IOPoolCtx_t *, const FIO_prefs_t *)\0",
                ))
                .as_ptr(),
            );
        }
        (*ctx).threadPool = POOL_create(1, (MAX_IO_JOBS - 2) as size_t);
        (*ctx).threadPoolActive = 1;
        if ((*ctx).threadPool).is_null() {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    176,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    104,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Failed creating I/O thread pool\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(104);
        }
    }
}
unsafe extern "C" fn AIO_IOPool_init(
    mut ctx: *mut IOPoolCtx_t,
    mut prefs: *const FIO_prefs_t,
    mut poolFunction: POOL_function,
    mut bufferSize: size_t,
) {
    let mut i: core::ffi::c_int = 0;
    AIO_IOPool_createThreadPool(ctx, prefs);
    (*ctx).prefs = prefs;
    (*ctx).poolFunction = poolFunction;
    (*ctx).totalIoJobs = if !((*ctx).threadPool).is_null() {
        MAX_IO_JOBS
    } else {
        2
    };
    (*ctx).availableJobsCount = (*ctx).totalIoJobs;
    i = 0;
    while i < (*ctx).availableJobsCount {
        let fresh0 = &mut (*((*ctx).availableJobs).as_mut_ptr().offset(i as isize));
        *fresh0 = AIO_IOPool_createIoJob(ctx, bufferSize) as *mut core::ffi::c_void;
        i += 1;
    }
    (*ctx).jobBufferSize = bufferSize;
    (*ctx).file = NULL as *mut FILE;
}
unsafe extern "C" fn AIO_IOPool_threadPoolActive(mut ctx: *mut IOPoolCtx_t) -> core::ffi::c_int {
    (!((*ctx).threadPool).is_null() && (*ctx).threadPoolActive != 0) as core::ffi::c_int
}
unsafe extern "C" fn AIO_IOPool_lockJobsMutex(mut ctx: *mut IOPoolCtx_t) {
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        pthread_mutex_lock(&mut (*ctx).ioJobsMutex);
    }
}
unsafe extern "C" fn AIO_IOPool_unlockJobsMutex(mut ctx: *mut IOPoolCtx_t) {
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        pthread_mutex_unlock(&mut (*ctx).ioJobsMutex);
    }
}
unsafe extern "C" fn AIO_IOPool_releaseIoJob(mut job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut IOPoolCtx_t;
    AIO_IOPool_lockJobsMutex(ctx);
    if (*ctx).availableJobsCount < (*ctx).totalIoJobs {
    } else {
        __assert_fail(
            b"ctx->availableJobsCount < ctx->totalIoJobs\0" as *const u8
                as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            224,
            (*::core::mem::transmute::<&[u8; 40], &[core::ffi::c_char; 40]>(
                b"void AIO_IOPool_releaseIoJob(IOJob_t *)\0",
            ))
            .as_ptr(),
        );
    }
    let fresh1 = (*ctx).availableJobsCount;
    (*ctx).availableJobsCount += 1;
    let fresh2 = &mut (*((*ctx).availableJobs).as_mut_ptr().offset(fresh1 as isize));
    *fresh2 = job as *mut core::ffi::c_void;
    AIO_IOPool_unlockJobsMutex(ctx);
}
unsafe extern "C" fn AIO_IOPool_join(mut ctx: *mut IOPoolCtx_t) {
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        POOL_joinJobs((*ctx).threadPool);
    }
}
unsafe extern "C" fn AIO_IOPool_setThreaded(
    mut ctx: *mut IOPoolCtx_t,
    mut threaded: core::ffi::c_int,
) {
    if threaded == 0 || threaded == 1 {
    } else {
        __assert_fail(
            b"threaded == 0 || threaded == 1\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            240,
            (*::core::mem::transmute::<&[u8; 48], &[core::ffi::c_char; 48]>(
                b"void AIO_IOPool_setThreaded(IOPoolCtx_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if !ctx.is_null() {
    } else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            241,
            (*::core::mem::transmute::<&[u8; 48], &[core::ffi::c_char; 48]>(
                b"void AIO_IOPool_setThreaded(IOPoolCtx_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*ctx).threadPoolActive != threaded {
        AIO_IOPool_join(ctx);
        (*ctx).threadPoolActive = threaded;
    }
}
unsafe extern "C" fn AIO_IOPool_destroy(mut ctx: *mut IOPoolCtx_t) {
    let mut i: core::ffi::c_int = 0;
    if !((*ctx).threadPool).is_null() {
        AIO_IOPool_join(ctx);
        if (*ctx).availableJobsCount == (*ctx).totalIoJobs {
        } else {
            __assert_fail(
                b"ctx->availableJobsCount == ctx->totalIoJobs\0" as *const u8
                    as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                256,
                (*::core::mem::transmute::<&[u8; 39], &[core::ffi::c_char; 39]>(
                    b"void AIO_IOPool_destroy(IOPoolCtx_t *)\0",
                ))
                .as_ptr(),
            );
        }
        POOL_free((*ctx).threadPool);
        pthread_mutex_destroy(&mut (*ctx).ioJobsMutex);
    }
    if ((*ctx).file).is_null() {
    } else {
        __assert_fail(
            b"ctx->file == NULL\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            260,
            (*::core::mem::transmute::<&[u8; 39], &[core::ffi::c_char; 39]>(
                b"void AIO_IOPool_destroy(IOPoolCtx_t *)\0",
            ))
            .as_ptr(),
        );
    }
    i = 0;
    while i < (*ctx).availableJobsCount {
        let mut job = *((*ctx).availableJobs).as_mut_ptr().offset(i as isize) as *mut IOJob_t;
        free((*job).buffer);
        free(job as *mut core::ffi::c_void);
        i += 1;
    }
}
unsafe extern "C" fn AIO_IOPool_acquireJob(mut ctx: *mut IOPoolCtx_t) -> *mut IOJob_t {
    let mut job = core::ptr::null_mut::<IOJob_t>();
    if !((*ctx).file).is_null() || (*(*ctx).prefs).testMode != 0 {
    } else {
        __assert_fail(
            b"ctx->file != NULL || ctx->prefs->testMode\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            272,
            (*::core::mem::transmute::<&[u8; 46], &[core::ffi::c_char; 46]>(
                b"IOJob_t *AIO_IOPool_acquireJob(IOPoolCtx_t *)\0",
            ))
            .as_ptr(),
        );
    }
    AIO_IOPool_lockJobsMutex(ctx);
    if (*ctx).availableJobsCount > 0 {
    } else {
        __assert_fail(
            b"ctx->availableJobsCount > 0\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            274,
            (*::core::mem::transmute::<&[u8; 46], &[core::ffi::c_char; 46]>(
                b"IOJob_t *AIO_IOPool_acquireJob(IOPoolCtx_t *)\0",
            ))
            .as_ptr(),
        );
    }
    (*ctx).availableJobsCount -= 1;
    job = *((*ctx).availableJobs)
        .as_mut_ptr()
        .offset((*ctx).availableJobsCount as isize) as *mut IOJob_t;
    AIO_IOPool_unlockJobsMutex(ctx);
    (*job).usedBufferSize = 0;
    (*job).file = (*ctx).file;
    (*job).offset = 0;
    job
}
unsafe extern "C" fn AIO_IOPool_setFile(mut ctx: *mut IOPoolCtx_t, mut file: *mut FILE) {
    if !ctx.is_null() {
    } else {
        __assert_fail(
            b"ctx!=NULL\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            288,
            (*::core::mem::transmute::<&[u8; 47], &[core::ffi::c_char; 47]>(
                b"void AIO_IOPool_setFile(IOPoolCtx_t *, FILE *)\0",
            ))
            .as_ptr(),
        );
    }
    AIO_IOPool_join(ctx);
    if (*ctx).availableJobsCount == (*ctx).totalIoJobs {
    } else {
        __assert_fail(
            b"ctx->availableJobsCount == ctx->totalIoJobs\0" as *const u8
                as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            290,
            (*::core::mem::transmute::<&[u8; 47], &[core::ffi::c_char; 47]>(
                b"void AIO_IOPool_setFile(IOPoolCtx_t *, FILE *)\0",
            ))
            .as_ptr(),
        );
    }
    (*ctx).file = file;
}
unsafe extern "C" fn AIO_IOPool_getFile(mut ctx: *const IOPoolCtx_t) -> *mut FILE {
    (*ctx).file
}
unsafe extern "C" fn AIO_IOPool_enqueueJob(mut job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut IOPoolCtx_t;
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        POOL_add(
            (*ctx).threadPool,
            (*ctx).poolFunction,
            job as *mut core::ffi::c_void,
        );
    } else {
        ((*ctx).poolFunction).unwrap_unchecked()(job as *mut core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_acquireJob(mut ctx: *mut WritePoolCtx_t) -> *mut IOJob_t {
    AIO_IOPool_acquireJob(&mut (*ctx).base)
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_enqueueAndReacquireWriteJob(mut job: *mut *mut IOJob_t) {
    AIO_IOPool_enqueueJob(*job);
    *job = AIO_IOPool_acquireJob((**job).ctx as *mut IOPoolCtx_t);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_sparseWriteEnd(mut ctx: *mut WritePoolCtx_t) {
    if !ctx.is_null() {
    } else {
        __assert_fail(
            b"ctx != NULL\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            333,
            (*::core::mem::transmute::<&[u8; 52], &[core::ffi::c_char; 52]>(
                b"void AIO_WritePool_sparseWriteEnd(WritePoolCtx_t *)\0",
            ))
            .as_ptr(),
        );
    }
    AIO_IOPool_join(&mut (*ctx).base);
    AIO_fwriteSparseEnd((*ctx).base.prefs, (*ctx).base.file, (*ctx).storedSkips);
    (*ctx).storedSkips = 0;
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_setFile(mut ctx: *mut WritePoolCtx_t, mut file: *mut FILE) {
    AIO_IOPool_setFile(&mut (*ctx).base, file);
    if (*ctx).storedSkips == 0 {
    } else {
        __assert_fail(
            b"ctx->storedSkips == 0\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            345,
            (*::core::mem::transmute::<&[u8; 53], &[core::ffi::c_char; 53]>(
                b"void AIO_WritePool_setFile(WritePoolCtx_t *, FILE *)\0",
            ))
            .as_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_getFile(mut ctx: *const WritePoolCtx_t) -> *mut FILE {
    AIO_IOPool_getFile(&(*ctx).base)
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_releaseIoJob(mut job: *mut IOJob_t) {
    AIO_IOPool_releaseIoJob(job);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_closeFile(mut ctx: *mut WritePoolCtx_t) -> core::ffi::c_int {
    let dstFile = (*ctx).base.file;
    if !dstFile.is_null() || (*(*ctx).base.prefs).testMode != 0 {
    } else {
        __assert_fail(
            b"dstFile!=NULL || ctx->base.prefs->testMode!=0\0" as *const u8
                as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            365,
            (*::core::mem::transmute::<&[u8; 46], &[core::ffi::c_char; 46]>(
                b"int AIO_WritePool_closeFile(WritePoolCtx_t *)\0",
            ))
            .as_ptr(),
        );
    }
    AIO_WritePool_sparseWriteEnd(ctx);
    AIO_IOPool_setFile(&mut (*ctx).base, NULL as *mut FILE);
    fclose(dstFile)
}
unsafe extern "C" fn AIO_WritePool_executeWriteJob(mut opaque: *mut core::ffi::c_void) {
    let job = opaque as *mut IOJob_t;
    let ctx = (*job).ctx as *mut WritePoolCtx_t;
    (*ctx).storedSkips = AIO_fwriteSparse(
        (*job).file,
        (*job).buffer,
        (*job).usedBufferSize,
        (*ctx).base.prefs,
        (*ctx).storedSkips,
    );
    AIO_IOPool_releaseIoJob(job);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_create(
    mut prefs: *const FIO_prefs_t,
    mut bufferSize: size_t,
) -> *mut WritePoolCtx_t {
    let ctx = malloc(::core::mem::size_of::<WritePoolCtx_t>() as core::ffi::c_ulong)
        as *mut WritePoolCtx_t;
    if ctx.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                384,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                100,
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
        exit(100);
    }
    AIO_IOPool_init(
        &mut (*ctx).base,
        prefs,
        Some(AIO_WritePool_executeWriteJob as unsafe extern "C" fn(*mut core::ffi::c_void) -> ()),
        bufferSize,
    );
    (*ctx).storedSkips = 0;
    ctx
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_free(mut ctx: *mut WritePoolCtx_t) {
    if !(AIO_WritePool_getFile(ctx)).is_null() {
        AIO_WritePool_closeFile(ctx);
    }
    AIO_IOPool_destroy(&mut (*ctx).base);
    if (*ctx).storedSkips == 0 {
    } else {
        __assert_fail(
            b"ctx->storedSkips==0\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            397,
            (*::core::mem::transmute::<&[u8; 42], &[core::ffi::c_char; 42]>(
                b"void AIO_WritePool_free(WritePoolCtx_t *)\0",
            ))
            .as_ptr(),
        );
    }
    free(ctx as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_WritePool_setAsync(
    mut ctx: *mut WritePoolCtx_t,
    mut async_0: core::ffi::c_int,
) {
    AIO_IOPool_setThreaded(&mut (*ctx).base, async_0);
}
unsafe extern "C" fn AIO_ReadPool_releaseAllCompletedJobs(mut ctx: *mut ReadPoolCtx_t) {
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < (*ctx).completedJobsCount {
        let mut job = *((*ctx).completedJobs).as_mut_ptr().offset(i as isize) as *mut IOJob_t;
        AIO_IOPool_releaseIoJob(job);
        i += 1;
    }
    (*ctx).completedJobsCount = 0;
}
unsafe extern "C" fn AIO_ReadPool_addJobToCompleted(mut job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut ReadPoolCtx_t;
    AIO_IOPool_lockJobsMutex(&mut (*ctx).base);
    if (*ctx).completedJobsCount < 10 {
    } else {
        __assert_fail(
            b"ctx->completedJobsCount < MAX_IO_JOBS\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            424,
            (*::core::mem::transmute::<&[u8; 47], &[core::ffi::c_char; 47]>(
                b"void AIO_ReadPool_addJobToCompleted(IOJob_t *)\0",
            ))
            .as_ptr(),
        );
    }
    let fresh3 = (*ctx).completedJobsCount;
    (*ctx).completedJobsCount += 1;
    let fresh4 = &mut (*((*ctx).completedJobs).as_mut_ptr().offset(fresh3 as isize));
    *fresh4 = job as *mut core::ffi::c_void;
    if AIO_IOPool_threadPoolActive(&mut (*ctx).base) != 0 {
        pthread_cond_signal(&mut (*ctx).jobCompletedCond);
    }
    AIO_IOPool_unlockJobsMutex(&mut (*ctx).base);
}
unsafe extern "C" fn AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(
    mut ctx: *mut ReadPoolCtx_t,
) -> *mut IOJob_t {
    let mut job = NULL as *mut IOJob_t;
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < (*ctx).completedJobsCount {
        job = *((*ctx).completedJobs).as_mut_ptr().offset(i as isize) as *mut IOJob_t;
        if (*job).offset == (*ctx).waitingOnOffset {
            (*ctx).completedJobsCount -= 1;
            let fresh5 = &mut (*((*ctx).completedJobs).as_mut_ptr().offset(i as isize));
            *fresh5 = *((*ctx).completedJobs)
                .as_mut_ptr()
                .offset((*ctx).completedJobsCount as isize);
            return job;
        }
        i += 1;
    }
    NULL as *mut IOJob_t
}
unsafe extern "C" fn AIO_ReadPool_numReadsInFlight(mut ctx: *mut ReadPoolCtx_t) -> size_t {
    let jobsHeld = if ((*ctx).currentJobHeld).is_null() {
        0
    } else {
        1
    };
    ((*ctx).base.totalIoJobs
        - ((*ctx).base.availableJobsCount + (*ctx).completedJobsCount + jobsHeld)) as size_t
}
unsafe extern "C" fn AIO_ReadPool_getNextCompletedJob(mut ctx: *mut ReadPoolCtx_t) -> *mut IOJob_t {
    let mut job = NULL as *mut IOJob_t;
    AIO_IOPool_lockJobsMutex(&mut (*ctx).base);
    job = AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(ctx);
    while job.is_null() && AIO_ReadPool_numReadsInFlight(ctx) > 0 {
        if !((*ctx).base.threadPool).is_null() {
        } else {
            __assert_fail(
                b"ctx->base.threadPool != NULL\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                471,
                (*::core::mem::transmute::<&[u8; 59], &[core::ffi::c_char; 59]>(
                    b"IOJob_t *AIO_ReadPool_getNextCompletedJob(ReadPoolCtx_t *)\0",
                ))
                .as_ptr(),
            );
        }
        pthread_cond_wait(&mut (*ctx).jobCompletedCond, &mut (*ctx).base.ioJobsMutex);
        job = AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(ctx);
    }
    if !job.is_null() {
        if (*job).offset == (*ctx).waitingOnOffset {
        } else {
            __assert_fail(
                b"job->offset == ctx->waitingOnOffset\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                477,
                (*::core::mem::transmute::<&[u8; 59], &[core::ffi::c_char; 59]>(
                    b"IOJob_t *AIO_ReadPool_getNextCompletedJob(ReadPoolCtx_t *)\0",
                ))
                .as_ptr(),
            );
        }
        (*ctx).waitingOnOffset = ((*ctx).waitingOnOffset as core::ffi::c_ulong)
            .wrapping_add((*job).usedBufferSize) as u64 as u64;
    }
    AIO_IOPool_unlockJobsMutex(&mut (*ctx).base);
    job
}
unsafe extern "C" fn AIO_ReadPool_executeReadJob(mut opaque: *mut core::ffi::c_void) {
    let job = opaque as *mut IOJob_t;
    let ctx = (*job).ctx as *mut ReadPoolCtx_t;
    if (*ctx).reachedEof != 0 {
        (*job).usedBufferSize = 0;
        AIO_ReadPool_addJobToCompleted(job);
        return;
    }
    (*job).usedBufferSize = fread((*job).buffer, 1, (*job).bufferSize, (*job).file);
    if (*job).usedBufferSize < (*job).bufferSize {
        if ferror((*job).file) != 0 {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    499,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    37,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Read error\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(37);
        } else if feof((*job).file) != 0 {
            (*ctx).reachedEof = 1;
        } else {
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
            }
            if g_display_prefs.displayLevel >= 5 {
                fprintf(
                    stderr,
                    b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                    b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                    503,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                    37,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Unexpected short read\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            if g_display_prefs.displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            }
            exit(37);
        }
    }
    AIO_ReadPool_addJobToCompleted(job);
}
unsafe extern "C" fn AIO_ReadPool_enqueueRead(mut ctx: *mut ReadPoolCtx_t) {
    let job = AIO_IOPool_acquireJob(&mut (*ctx).base);
    (*job).offset = (*ctx).nextReadOffset;
    (*ctx).nextReadOffset =
        ((*ctx).nextReadOffset as core::ffi::c_ulong).wrapping_add((*job).bufferSize) as u64 as u64;
    AIO_IOPool_enqueueJob(job);
}
unsafe extern "C" fn AIO_ReadPool_startReading(mut ctx: *mut ReadPoolCtx_t) {
    while (*ctx).base.availableJobsCount != 0 {
        AIO_ReadPool_enqueueRead(ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_setFile(mut ctx: *mut ReadPoolCtx_t, mut file: *mut FILE) {
    if !ctx.is_null() {
    } else {
        __assert_fail(
            b"ctx!=NULL\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            526,
            (*::core::mem::transmute::<&[u8; 51], &[core::ffi::c_char; 51]>(
                b"void AIO_ReadPool_setFile(ReadPoolCtx_t *, FILE *)\0",
            ))
            .as_ptr(),
        );
    }
    AIO_IOPool_join(&mut (*ctx).base);
    AIO_ReadPool_releaseAllCompletedJobs(ctx);
    if !((*ctx).currentJobHeld).is_null() {
        AIO_IOPool_releaseIoJob((*ctx).currentJobHeld as *mut IOJob_t);
        (*ctx).currentJobHeld = NULL as *mut core::ffi::c_void;
    }
    AIO_IOPool_setFile(&mut (*ctx).base, file);
    (*ctx).nextReadOffset = 0;
    (*ctx).waitingOnOffset = 0;
    (*ctx).srcBuffer = (*ctx).coalesceBuffer;
    (*ctx).srcBufferLoaded = 0;
    (*ctx).reachedEof = 0;
    if !file.is_null() {
        AIO_ReadPool_startReading(ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_create(
    mut prefs: *const FIO_prefs_t,
    mut bufferSize: size_t,
) -> *mut ReadPoolCtx_t {
    let ctx =
        malloc(::core::mem::size_of::<ReadPoolCtx_t>() as core::ffi::c_ulong) as *mut ReadPoolCtx_t;
    if ctx.is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                549,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                100,
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
        exit(100);
    }
    AIO_IOPool_init(
        &mut (*ctx).base,
        prefs,
        Some(AIO_ReadPool_executeReadJob as unsafe extern "C" fn(*mut core::ffi::c_void) -> ()),
        bufferSize,
    );
    (*ctx).coalesceBuffer = malloc(bufferSize * 2) as *mut u8;
    if ((*ctx).coalesceBuffer).is_null() {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                553,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                100,
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
        exit(100);
    }
    (*ctx).srcBuffer = (*ctx).coalesceBuffer;
    (*ctx).srcBufferLoaded = 0;
    (*ctx).completedJobsCount = 0;
    (*ctx).currentJobHeld = NULL as *mut core::ffi::c_void;
    if !((*ctx).base.threadPool).is_null()
        && pthread_cond_init(
            &mut (*ctx).jobCompletedCond,
            core::ptr::null::<pthread_condattr_t>(),
        ) != 0
    {
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b"zstd: \0" as *const u8 as *const core::ffi::c_char);
        }
        if g_display_prefs.displayLevel >= 5 {
            fprintf(
                stderr,
                b"Error defined at %s, line %i : \n\0" as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                561,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"error %i : \0" as *const u8 as *const core::ffi::c_char,
                103,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed creating jobCompletedCond cond\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        if g_display_prefs.displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        }
        exit(103);
    }
    ctx
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_free(mut ctx: *mut ReadPoolCtx_t) {
    if !(AIO_ReadPool_getFile(ctx)).is_null() {
        AIO_ReadPool_closeFile(ctx);
    }
    if !((*ctx).base.threadPool).is_null() {
        pthread_cond_destroy(&mut (*ctx).jobCompletedCond);
    }
    AIO_IOPool_destroy(&mut (*ctx).base);
    free((*ctx).coalesceBuffer as *mut core::ffi::c_void);
    free(ctx as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_consumeBytes(mut ctx: *mut ReadPoolCtx_t, mut n: size_t) {
    if n <= (*ctx).srcBufferLoaded {
    } else {
        __assert_fail(
            b"n <= ctx->srcBufferLoaded\0" as *const u8 as *const core::ffi::c_char,
            b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
            581,
            (*::core::mem::transmute::<&[u8; 56], &[core::ffi::c_char; 56]>(
                b"void AIO_ReadPool_consumeBytes(ReadPoolCtx_t *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    (*ctx).srcBufferLoaded = ((*ctx).srcBufferLoaded).wrapping_sub(n);
    (*ctx).srcBuffer = ((*ctx).srcBuffer).offset(n as isize);
}
unsafe extern "C" fn AIO_ReadPool_releaseCurrentHeldAndGetNext(
    mut ctx: *mut ReadPoolCtx_t,
) -> *mut IOJob_t {
    if !((*ctx).currentJobHeld).is_null() {
        AIO_IOPool_releaseIoJob((*ctx).currentJobHeld as *mut IOJob_t);
        (*ctx).currentJobHeld = NULL as *mut core::ffi::c_void;
        AIO_ReadPool_enqueueRead(ctx);
    }
    (*ctx).currentJobHeld = AIO_ReadPool_getNextCompletedJob(ctx) as *mut core::ffi::c_void;
    (*ctx).currentJobHeld as *mut IOJob_t
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_fillBuffer(
    mut ctx: *mut ReadPoolCtx_t,
    mut n: size_t,
) -> size_t {
    let mut job = core::ptr::null_mut::<IOJob_t>();
    let mut useCoalesce = 0;
    if n > (*ctx).base.jobBufferSize {
        n = (*ctx).base.jobBufferSize;
    }
    if (*ctx).srcBufferLoaded >= n {
        return 0;
    }
    if (*ctx).srcBufferLoaded > 0 {
        useCoalesce = 1;
        memcpy(
            (*ctx).coalesceBuffer as *mut core::ffi::c_void,
            (*ctx).srcBuffer as *const core::ffi::c_void,
            (*ctx).srcBufferLoaded,
        );
        (*ctx).srcBuffer = (*ctx).coalesceBuffer;
    }
    job = AIO_ReadPool_releaseCurrentHeldAndGetNext(ctx);
    if job.is_null() {
        return 0;
    }
    if useCoalesce != 0 {
        if ((*ctx).srcBufferLoaded).wrapping_add((*job).usedBufferSize)
            <= 2 * (*ctx).base.jobBufferSize
        {
        } else {
            __assert_fail(
                b"ctx->srcBufferLoaded + job->usedBufferSize <= 2*ctx->base.jobBufferSize\0"
                    as *const u8 as *const core::ffi::c_char,
                b"fileio_asyncio.c\0" as *const u8 as *const core::ffi::c_char,
                626,
                (*::core::mem::transmute::<&[u8; 56], &[core::ffi::c_char; 56]>(
                    b"size_t AIO_ReadPool_fillBuffer(ReadPoolCtx_t *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        memcpy(
            ((*ctx).coalesceBuffer).offset((*ctx).srcBufferLoaded as isize)
                as *mut core::ffi::c_void,
            (*job).buffer,
            (*job).usedBufferSize,
        );
        (*ctx).srcBufferLoaded = ((*ctx).srcBufferLoaded).wrapping_add((*job).usedBufferSize);
    } else {
        (*ctx).srcBuffer = (*job).buffer as *mut u8;
        (*ctx).srcBufferLoaded = (*job).usedBufferSize;
    }
    (*job).usedBufferSize
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_consumeAndRefill(mut ctx: *mut ReadPoolCtx_t) -> size_t {
    AIO_ReadPool_consumeBytes(ctx, (*ctx).srcBufferLoaded);
    AIO_ReadPool_fillBuffer(ctx, (*ctx).base.jobBufferSize)
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_getFile(mut ctx: *const ReadPoolCtx_t) -> *mut FILE {
    AIO_IOPool_getFile(&(*ctx).base)
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_closeFile(mut ctx: *mut ReadPoolCtx_t) -> core::ffi::c_int {
    let file = AIO_ReadPool_getFile(ctx);
    AIO_ReadPool_setFile(ctx, NULL as *mut FILE);
    fclose(file)
}
#[no_mangle]
pub unsafe extern "C" fn AIO_ReadPool_setAsync(
    mut ctx: *mut ReadPoolCtx_t,
    mut async_0: core::ffi::c_int,
) {
    AIO_IOPool_setThreaded(&mut (*ctx).base, async_0);
}
unsafe extern "C" fn run_static_initializers() {
    segmentSizeT = ((32 * ((1) << 10)) as core::ffi::c_ulong)
        .wrapping_div(::core::mem::size_of::<size_t>() as core::ffi::c_ulong);
    maskT = (::core::mem::size_of::<size_t>() as core::ffi::c_ulong).wrapping_sub(1);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: unsafe extern "C" fn() = run_static_initializers;
