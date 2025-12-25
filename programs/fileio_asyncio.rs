use std::io;

use libc::{
    exit, fclose, feof, ferror, fprintf, fread, free, fseek, fwrite, malloc, memcpy,
    pthread_cond_destroy, pthread_cond_init, pthread_cond_signal, pthread_cond_t,
    pthread_cond_wait, pthread_condattr_t, pthread_mutex_destroy, pthread_mutex_init,
    pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock, pthread_mutexattr_t, size_t, FILE,
};
use libzstd_rs_sys::internal::{
    POOL_add, POOL_create, POOL_ctx, POOL_free, POOL_function, POOL_joinJobs,
};
use libzstd_rs_sys::ZSTD_ParamSwitch_e;

use crate::fileio::g_display_prefs;

extern "C" {
    static mut stderr: *mut FILE;
}
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
static segmentSizeT: usize = 32usize * (1 << 10) / ::core::mem::size_of::<size_t>();
static maskT: usize = ::core::mem::size_of::<size_t>() - 1;
unsafe fn AIO_fwriteSparse(
    file: *mut FILE,
    buffer: *const core::ffi::c_void,
    bufferSize: size_t,
    prefs: *const FIO_prefs_t,
    mut storedSkips: core::ffi::c_uint,
) -> core::ffi::c_uint {
    let bufferT = buffer as *const size_t;
    let mut bufferSizeT = bufferSize.wrapping_div(::core::mem::size_of::<size_t>());
    let bufferTEnd = bufferT.add(bufferSizeT);
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
                eprintln!(
                    "Write error : cannot write block : {}",
                    io::Error::last_os_error(),
                );
            }
            exit(70);
        }
        return 0;
    }
    if storedSkips > (1 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30) {
        if fseek(
            file,
            (1 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30)
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
        storedSkips = storedSkips
            .wrapping_sub((1 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30));
    }
    while ptrT < bufferTEnd {
        let mut nb0T: size_t = 0;
        let mut seg0SizeT = segmentSizeT;
        if seg0SizeT > bufferSizeT {
            seg0SizeT = bufferSizeT;
        }
        bufferSizeT = bufferSizeT.wrapping_sub(seg0SizeT);
        nb0T = 0;
        while nb0T < seg0SizeT && *ptrT.add(nb0T) == 0 {
            nb0T = nb0T.wrapping_add(1);
        }
        storedSkips = storedSkips
            .wrapping_add(nb0T.wrapping_mul(::core::mem::size_of::<size_t>()) as core::ffi::c_uint);
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
                ptrT.add(nb0T) as *const core::ffi::c_void,
                ::core::mem::size_of::<size_t>(),
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
                    eprintln!(
                        "Write error : cannot write block : {}",
                        io::Error::last_os_error(),
                    );
                }
                exit(93);
            }
        }
        ptrT = ptrT.add(seg0SizeT);
    }
    if bufferSize & maskT != 0 {
        let restStart = bufferTEnd as *const core::ffi::c_char;
        let mut restPtr = restStart;
        let restEnd = (buffer as *const core::ffi::c_char).add(bufferSize);
        assert!(restEnd > restStart && restEnd < restStart.add(::core::mem::size_of::<size_t>()));
        while restPtr < restEnd && core::ffi::c_int::from(*restPtr) == 0 {
            restPtr = restPtr.offset(1);
        }
        storedSkips = storedSkips
            .wrapping_add(restPtr.offset_from(restStart) as core::ffi::c_long as core::ffi::c_uint);
        if restPtr != restEnd {
            let restSize = restEnd.offset_from(restPtr) as size_t;
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
                    eprintln!(
                        "Write error : cannot write end of decoded block : {}",
                        io::Error::last_os_error(),
                    );
                }
                exit(95);
            }
            storedSkips = 0;
        }
    }
    storedSkips
}
unsafe fn AIO_fwriteSparseEnd(
    prefs: *const FIO_prefs_t,
    file: *mut FILE,
    storedSkips: core::ffi::c_uint,
) {
    if (*prefs).testMode != 0 {
        assert!(storedSkips == 0);
    }
    if storedSkips > 0 {
        assert!((*prefs).sparseFileSupport > 0);
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
                eprintln!(
                    "Write error : cannot write last zero : {}",
                    io::Error::last_os_error(),
                );
            }
            exit(69);
        }
    }
}
pub unsafe fn AIO_supported() -> core::ffi::c_int {
    1
}
unsafe fn AIO_IOPool_createIoJob(ctx: *mut IOPoolCtx_t, bufferSize: size_t) -> *mut IOJob_t {
    let job = malloc(::core::mem::size_of::<IOJob_t>()) as *mut IOJob_t;
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
    (*job).file = core::ptr::null_mut();
    (*job).ctx = ctx as *mut core::ffi::c_void;
    (*job).offset = 0;
    job
}
unsafe fn AIO_IOPool_createThreadPool(ctx: *mut IOPoolCtx_t, prefs: *const FIO_prefs_t) {
    (*ctx).threadPool = core::ptr::null_mut();
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
        assert!(MAX_IO_JOBS >= 2);
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
unsafe fn AIO_IOPool_init(
    ctx: *mut IOPoolCtx_t,
    prefs: *const FIO_prefs_t,
    poolFunction: POOL_function,
    bufferSize: size_t,
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
    (*ctx).file = core::ptr::null_mut();
}
unsafe fn AIO_IOPool_threadPoolActive(ctx: *mut IOPoolCtx_t) -> core::ffi::c_int {
    core::ffi::c_int::from(!((*ctx).threadPool).is_null() && (*ctx).threadPoolActive != 0)
}
unsafe fn AIO_IOPool_lockJobsMutex(ctx: *mut IOPoolCtx_t) {
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        pthread_mutex_lock(&mut (*ctx).ioJobsMutex);
    }
}
unsafe fn AIO_IOPool_unlockJobsMutex(ctx: *mut IOPoolCtx_t) {
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        pthread_mutex_unlock(&mut (*ctx).ioJobsMutex);
    }
}
unsafe fn AIO_IOPool_releaseIoJob(job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut IOPoolCtx_t;
    AIO_IOPool_lockJobsMutex(ctx);
    assert!((*ctx).availableJobsCount < (*ctx).totalIoJobs);
    let fresh1 = (*ctx).availableJobsCount;
    (*ctx).availableJobsCount += 1;
    let fresh2 = &mut (*((*ctx).availableJobs).as_mut_ptr().offset(fresh1 as isize));
    *fresh2 = job as *mut core::ffi::c_void;
    AIO_IOPool_unlockJobsMutex(ctx);
}
unsafe fn AIO_IOPool_join(ctx: *mut IOPoolCtx_t) {
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        POOL_joinJobs((*ctx).threadPool);
    }
}
unsafe fn AIO_IOPool_setThreaded(ctx: *mut IOPoolCtx_t, threaded: core::ffi::c_int) {
    assert!(threaded == 0 || threaded == 1);
    assert!(!ctx.is_null());
    if (*ctx).threadPoolActive != threaded {
        AIO_IOPool_join(ctx);
        (*ctx).threadPoolActive = threaded;
    }
}
unsafe fn AIO_IOPool_destroy(ctx: *mut IOPoolCtx_t) {
    let mut i: core::ffi::c_int = 0;
    if !((*ctx).threadPool).is_null() {
        AIO_IOPool_join(ctx);
        assert!((*ctx).availableJobsCount == (*ctx).totalIoJobs);
        POOL_free((*ctx).threadPool);
        pthread_mutex_destroy(&mut (*ctx).ioJobsMutex);
    }
    assert!(((*ctx).file).is_null());
    i = 0;
    while i < (*ctx).availableJobsCount {
        let job = *((*ctx).availableJobs).as_mut_ptr().offset(i as isize) as *mut IOJob_t;
        free((*job).buffer);
        free(job as *mut core::ffi::c_void);
        i += 1;
    }
}
unsafe fn AIO_IOPool_acquireJob(ctx: *mut IOPoolCtx_t) -> *mut IOJob_t {
    let mut job = core::ptr::null_mut::<IOJob_t>();
    assert!(!((*ctx).file).is_null() || (*(*ctx).prefs).testMode != 0);
    AIO_IOPool_lockJobsMutex(ctx);
    assert!((*ctx).availableJobsCount > 0);
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
unsafe fn AIO_IOPool_setFile(ctx: *mut IOPoolCtx_t, file: *mut FILE) {
    assert!(!ctx.is_null());
    AIO_IOPool_join(ctx);
    assert!((*ctx).availableJobsCount == (*ctx).totalIoJobs);
    (*ctx).file = file;
}
unsafe fn AIO_IOPool_getFile(ctx: *const IOPoolCtx_t) -> *mut FILE {
    (*ctx).file
}
unsafe fn AIO_IOPool_enqueueJob(job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut IOPoolCtx_t;
    if AIO_IOPool_threadPoolActive(ctx) != 0 {
        POOL_add(
            (*ctx).threadPool,
            (*ctx).poolFunction,
            job as *mut core::ffi::c_void,
        );
    } else {
        ((*ctx).poolFunction)(job as *mut core::ffi::c_void);
    };
}
pub unsafe fn AIO_WritePool_acquireJob(ctx: *mut WritePoolCtx_t) -> *mut IOJob_t {
    AIO_IOPool_acquireJob(&mut (*ctx).base)
}
pub unsafe fn AIO_WritePool_enqueueAndReacquireWriteJob(job: *mut *mut IOJob_t) {
    AIO_IOPool_enqueueJob(*job);
    *job = AIO_IOPool_acquireJob((**job).ctx as *mut IOPoolCtx_t);
}
pub unsafe fn AIO_WritePool_sparseWriteEnd(ctx: *mut WritePoolCtx_t) {
    assert!(!ctx.is_null());
    AIO_IOPool_join(&mut (*ctx).base);
    AIO_fwriteSparseEnd((*ctx).base.prefs, (*ctx).base.file, (*ctx).storedSkips);
    (*ctx).storedSkips = 0;
}
pub unsafe fn AIO_WritePool_setFile(ctx: *mut WritePoolCtx_t, file: *mut FILE) {
    AIO_IOPool_setFile(&mut (*ctx).base, file);
    assert!((*ctx).storedSkips == 0);
}
pub unsafe fn AIO_WritePool_getFile(ctx: *const WritePoolCtx_t) -> *mut FILE {
    AIO_IOPool_getFile(&(*ctx).base)
}
pub unsafe fn AIO_WritePool_releaseIoJob(job: *mut IOJob_t) {
    AIO_IOPool_releaseIoJob(job);
}
pub unsafe fn AIO_WritePool_closeFile(ctx: *mut WritePoolCtx_t) -> core::ffi::c_int {
    let dstFile = (*ctx).base.file;
    assert!(!dstFile.is_null() || (*(*ctx).base.prefs).testMode != 0);
    AIO_WritePool_sparseWriteEnd(ctx);
    AIO_IOPool_setFile(&mut (*ctx).base, core::ptr::null_mut());
    fclose(dstFile)
}
unsafe fn AIO_WritePool_executeWriteJob(opaque: *mut core::ffi::c_void) {
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
pub unsafe fn AIO_WritePool_create(
    prefs: *const FIO_prefs_t,
    bufferSize: size_t,
) -> *mut WritePoolCtx_t {
    let ctx = malloc(::core::mem::size_of::<WritePoolCtx_t>()) as *mut WritePoolCtx_t;
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
        AIO_WritePool_executeWriteJob,
        bufferSize,
    );
    (*ctx).storedSkips = 0;
    ctx
}
pub unsafe fn AIO_WritePool_free(ctx: *mut WritePoolCtx_t) {
    if !(AIO_WritePool_getFile(ctx)).is_null() {
        AIO_WritePool_closeFile(ctx);
    }
    AIO_IOPool_destroy(&mut (*ctx).base);
    assert!((*ctx).storedSkips == 0);
    free(ctx as *mut core::ffi::c_void);
}
pub unsafe fn AIO_WritePool_setAsync(ctx: *mut WritePoolCtx_t, async_0: core::ffi::c_int) {
    AIO_IOPool_setThreaded(&mut (*ctx).base, async_0);
}
unsafe fn AIO_ReadPool_releaseAllCompletedJobs(ctx: *mut ReadPoolCtx_t) {
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < (*ctx).completedJobsCount {
        let job = *((*ctx).completedJobs).as_mut_ptr().offset(i as isize) as *mut IOJob_t;
        AIO_IOPool_releaseIoJob(job);
        i += 1;
    }
    (*ctx).completedJobsCount = 0;
}
unsafe fn AIO_ReadPool_addJobToCompleted(job: *mut IOJob_t) {
    let ctx = (*job).ctx as *mut ReadPoolCtx_t;
    AIO_IOPool_lockJobsMutex(&mut (*ctx).base);
    assert!((*ctx).completedJobsCount < MAX_IO_JOBS);
    let fresh3 = (*ctx).completedJobsCount;
    (*ctx).completedJobsCount += 1;
    let fresh4 = &mut (*((*ctx).completedJobs).as_mut_ptr().offset(fresh3 as isize));
    *fresh4 = job as *mut core::ffi::c_void;
    if AIO_IOPool_threadPoolActive(&mut (*ctx).base) != 0 {
        pthread_cond_signal(&mut (*ctx).jobCompletedCond);
    }
    AIO_IOPool_unlockJobsMutex(&mut (*ctx).base);
}
unsafe fn AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(
    ctx: *mut ReadPoolCtx_t,
) -> *mut IOJob_t {
    let mut job = core::ptr::null_mut();
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
    core::ptr::null_mut()
}
unsafe fn AIO_ReadPool_numReadsInFlight(ctx: *mut ReadPoolCtx_t) -> size_t {
    let jobsHeld = if ((*ctx).currentJobHeld).is_null() {
        0
    } else {
        1
    };
    ((*ctx).base.totalIoJobs
        - ((*ctx).base.availableJobsCount + (*ctx).completedJobsCount + jobsHeld)) as size_t
}
unsafe fn AIO_ReadPool_getNextCompletedJob(ctx: *mut ReadPoolCtx_t) -> *mut IOJob_t {
    let mut job = core::ptr::null_mut();
    AIO_IOPool_lockJobsMutex(&mut (*ctx).base);
    job = AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(ctx);
    while job.is_null() && AIO_ReadPool_numReadsInFlight(ctx) > 0 {
        assert!(!((*ctx).base.threadPool).is_null());
        pthread_cond_wait(&mut (*ctx).jobCompletedCond, &mut (*ctx).base.ioJobsMutex);
        job = AIO_ReadPool_findNextWaitingOffsetCompletedJob_locked(ctx);
    }
    if !job.is_null() {
        assert!((*job).offset == (*ctx).waitingOnOffset);
        (*ctx).waitingOnOffset = (*ctx)
            .waitingOnOffset
            .wrapping_add((*job).usedBufferSize as u64);
    }
    AIO_IOPool_unlockJobsMutex(&mut (*ctx).base);
    job
}
unsafe fn AIO_ReadPool_executeReadJob(opaque: *mut core::ffi::c_void) {
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
unsafe fn AIO_ReadPool_enqueueRead(ctx: *mut ReadPoolCtx_t) {
    let job = AIO_IOPool_acquireJob(&mut (*ctx).base);
    (*job).offset = (*ctx).nextReadOffset;
    (*ctx).nextReadOffset = (*ctx).nextReadOffset.wrapping_add((*job).bufferSize as u64);
    AIO_IOPool_enqueueJob(job);
}
unsafe fn AIO_ReadPool_startReading(ctx: *mut ReadPoolCtx_t) {
    #[expect(clippy::while_immutable_condition)]
    while (*ctx).base.availableJobsCount != 0 {
        AIO_ReadPool_enqueueRead(ctx);
    }
}
pub unsafe fn AIO_ReadPool_setFile(ctx: *mut ReadPoolCtx_t, file: *mut FILE) {
    assert!(!ctx.is_null());
    AIO_IOPool_join(&mut (*ctx).base);
    AIO_ReadPool_releaseAllCompletedJobs(ctx);
    if !((*ctx).currentJobHeld).is_null() {
        AIO_IOPool_releaseIoJob((*ctx).currentJobHeld as *mut IOJob_t);
        (*ctx).currentJobHeld = core::ptr::null_mut();
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
pub unsafe fn AIO_ReadPool_create(
    prefs: *const FIO_prefs_t,
    bufferSize: size_t,
) -> *mut ReadPoolCtx_t {
    let ctx = malloc(::core::mem::size_of::<ReadPoolCtx_t>()) as *mut ReadPoolCtx_t;
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
        AIO_ReadPool_executeReadJob,
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
    (*ctx).currentJobHeld = core::ptr::null_mut();
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
pub unsafe fn AIO_ReadPool_free(ctx: *mut ReadPoolCtx_t) {
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
pub unsafe fn AIO_ReadPool_consumeBytes(ctx: *mut ReadPoolCtx_t, n: size_t) {
    assert!(n <= (*ctx).srcBufferLoaded);
    (*ctx).srcBufferLoaded = ((*ctx).srcBufferLoaded).wrapping_sub(n);
    (*ctx).srcBuffer = ((*ctx).srcBuffer).add(n);
}
unsafe fn AIO_ReadPool_releaseCurrentHeldAndGetNext(ctx: *mut ReadPoolCtx_t) -> *mut IOJob_t {
    if !((*ctx).currentJobHeld).is_null() {
        AIO_IOPool_releaseIoJob((*ctx).currentJobHeld as *mut IOJob_t);
        (*ctx).currentJobHeld = core::ptr::null_mut();
        AIO_ReadPool_enqueueRead(ctx);
    }
    (*ctx).currentJobHeld = AIO_ReadPool_getNextCompletedJob(ctx) as *mut core::ffi::c_void;
    (*ctx).currentJobHeld as *mut IOJob_t
}
pub unsafe fn AIO_ReadPool_fillBuffer(ctx: *mut ReadPoolCtx_t, mut n: size_t) -> size_t {
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
        assert!(
            ((*ctx).srcBufferLoaded).wrapping_add((*job).usedBufferSize)
                <= 2 * (*ctx).base.jobBufferSize
        );
        memcpy(
            ((*ctx).coalesceBuffer).add((*ctx).srcBufferLoaded) as *mut core::ffi::c_void,
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
pub unsafe fn AIO_ReadPool_consumeAndRefill(ctx: *mut ReadPoolCtx_t) -> size_t {
    AIO_ReadPool_consumeBytes(ctx, (*ctx).srcBufferLoaded);
    AIO_ReadPool_fillBuffer(ctx, (*ctx).base.jobBufferSize)
}
pub unsafe fn AIO_ReadPool_getFile(ctx: *const ReadPoolCtx_t) -> *mut FILE {
    AIO_IOPool_getFile(&(*ctx).base)
}
pub unsafe fn AIO_ReadPool_closeFile(ctx: *mut ReadPoolCtx_t) -> core::ffi::c_int {
    let file = AIO_ReadPool_getFile(ctx);
    AIO_ReadPool_setFile(ctx, core::ptr::null_mut());
    fclose(file)
}
pub unsafe fn AIO_ReadPool_setAsync(ctx: *mut ReadPoolCtx_t, async_0: core::ffi::c_int) {
    AIO_IOPool_setThreaded(&mut (*ctx).base, async_0);
}
