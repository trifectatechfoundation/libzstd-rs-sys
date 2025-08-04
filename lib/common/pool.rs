use libc::{
    free, pthread_attr_t, pthread_cond_broadcast, pthread_cond_destroy, pthread_cond_init,
    pthread_cond_signal, pthread_cond_t, pthread_cond_wait, pthread_condattr_t, pthread_create,
    pthread_join, pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t,
    pthread_mutex_unlock, pthread_mutexattr_t, pthread_t,
};

use crate::lib::zstd::{ZSTD_allocFunction, ZSTD_customMem, ZSTD_freeFunction};

extern "C" {
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
}
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POOL_ctx {
    customMem: ZSTD_customMem,
    threads: *mut pthread_t,
    threadCapacity: size_t,
    threadLimit: size_t,
    queue: *mut POOL_job,
    queueHead: size_t,
    queueTail: size_t,
    queueSize: size_t,
    numThreadsBusy: size_t,
    queueEmpty: std::ffi::c_int,
    queueMutex: pthread_mutex_t,
    queuePushCond: pthread_cond_t,
    queuePopCond: pthread_cond_t,
    shutdown: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POOL_job {
    function: POOL_function,
    opaque: *mut std::ffi::c_void,
}
pub type POOL_function = Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>;
pub type ZSTD_threadPool = POOL_ctx;
static mut ZSTD_defaultCMem: ZSTD_customMem = unsafe {
    {
        ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<libc::intptr_t, ZSTD_allocFunction>(
                NULL as libc::intptr_t,
            ),
            customFree: ::core::mem::transmute::<libc::intptr_t, ZSTD_freeFunction>(
                NULL as libc::intptr_t,
            ),
            opaque: NULL as *mut std::ffi::c_void,
        }
    }
};
#[inline]
unsafe extern "C" fn ZSTD_customCalloc(
    mut size: size_t,
    mut customMem: ZSTD_customMem,
) -> *mut std::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        let ptr = (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
        libc::memset(ptr, 0, size as libc::size_t);
        return ptr;
    }
    calloc(1, size)
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(
    mut ptr: *mut std::ffi::c_void,
    mut customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree).unwrap_unchecked()(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}
pub const NULL: std::ffi::c_int = 0;
pub const NULL_0: std::ffi::c_int = 0;
unsafe extern "C" fn POOL_thread(mut opaque: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    let ctx = opaque as *mut POOL_ctx;
    if ctx.is_null() {
        return NULL_0 as *mut std::ffi::c_void;
    }
    loop {
        pthread_mutex_lock(&mut (*ctx).queueMutex);
        while (*ctx).queueEmpty != 0 || (*ctx).numThreadsBusy >= (*ctx).threadLimit {
            if (*ctx).shutdown != 0 {
                pthread_mutex_unlock(&mut (*ctx).queueMutex);
                return opaque;
            }
            pthread_cond_wait(&mut (*ctx).queuePopCond, &mut (*ctx).queueMutex);
        }
        let job = *((*ctx).queue).offset((*ctx).queueHead as isize);
        (*ctx).queueHead = ((*ctx).queueHead).wrapping_add(1) % (*ctx).queueSize;
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_add(1);
        (*ctx).numThreadsBusy;
        (*ctx).queueEmpty = ((*ctx).queueHead == (*ctx).queueTail) as std::ffi::c_int;
        pthread_cond_signal(&mut (*ctx).queuePushCond);
        pthread_mutex_unlock(&mut (*ctx).queueMutex);
        (job.function).unwrap_unchecked()(job.opaque);
        pthread_mutex_lock(&mut (*ctx).queueMutex);
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_sub(1);
        (*ctx).numThreadsBusy;
        pthread_cond_signal(&mut (*ctx).queuePushCond);
        pthread_mutex_unlock(&mut (*ctx).queueMutex);
    }
}
#[export_name = crate::prefix!(ZSTD_createThreadPool)]
pub unsafe extern "C" fn ZSTD_createThreadPool(mut numThreads: size_t) -> *mut ZSTD_threadPool {
    POOL_create(numThreads, 0)
}
#[export_name = crate::prefix!(POOL_create)]
pub unsafe extern "C" fn POOL_create(
    mut numThreads: size_t,
    mut queueSize: size_t,
) -> *mut POOL_ctx {
    POOL_create_advanced(numThreads, queueSize, ZSTD_defaultCMem)
}
#[export_name = crate::prefix!(POOL_create_advanced)]
pub unsafe extern "C" fn POOL_create_advanced(
    mut numThreads: size_t,
    mut queueSize: size_t,
    mut customMem: ZSTD_customMem,
) -> *mut POOL_ctx {
    let mut ctx = std::ptr::null_mut::<POOL_ctx>();
    if numThreads == 0 {
        return NULL_0 as *mut POOL_ctx;
    }
    ctx = ZSTD_customCalloc(
        ::core::mem::size_of::<POOL_ctx>() as std::ffi::c_ulong,
        customMem,
    ) as *mut POOL_ctx;
    if ctx.is_null() {
        return NULL_0 as *mut POOL_ctx;
    }
    (*ctx).queueSize = queueSize.wrapping_add(1);
    (*ctx).queue = ZSTD_customCalloc(
        ((*ctx).queueSize).wrapping_mul(::core::mem::size_of::<POOL_job>() as std::ffi::c_ulong),
        customMem,
    ) as *mut POOL_job;
    (*ctx).queueHead = 0;
    (*ctx).queueTail = 0;
    (*ctx).numThreadsBusy = 0;
    (*ctx).queueEmpty = 1;
    let mut error = 0;
    error |= pthread_mutex_init(
        &mut (*ctx).queueMutex,
        std::ptr::null::<pthread_mutexattr_t>(),
    );
    error |= pthread_cond_init(
        &mut (*ctx).queuePushCond,
        std::ptr::null::<pthread_condattr_t>(),
    );
    error |= pthread_cond_init(
        &mut (*ctx).queuePopCond,
        std::ptr::null::<pthread_condattr_t>(),
    );
    if error != 0 {
        POOL_free(ctx);
        return NULL_0 as *mut POOL_ctx;
    }
    (*ctx).shutdown = 0;
    (*ctx).threads = ZSTD_customCalloc(
        numThreads.wrapping_mul(::core::mem::size_of::<pthread_t>() as std::ffi::c_ulong),
        customMem,
    ) as *mut pthread_t;
    (*ctx).threadCapacity = 0;
    (*ctx).customMem = customMem;
    if ((*ctx).threads).is_null() || ((*ctx).queue).is_null() {
        POOL_free(ctx);
        return NULL_0 as *mut POOL_ctx;
    }
    let mut i: size_t = 0;
    i = 0;
    while i < numThreads {
        if pthread_create(
            &mut *((*ctx).threads).offset(i as isize),
            std::ptr::null::<pthread_attr_t>(),
            std::mem::transmute(
                POOL_thread as unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void,
            ),
            ctx as *mut std::ffi::c_void,
        ) != 0
        {
            (*ctx).threadCapacity = i;
            POOL_free(ctx);
            return NULL_0 as *mut POOL_ctx;
        }
        i = i.wrapping_add(1);
        i;
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    ctx
}
unsafe extern "C" fn POOL_join(mut ctx: *mut POOL_ctx) {
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    (*ctx).shutdown = 1;
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
    pthread_cond_broadcast(&mut (*ctx).queuePushCond);
    pthread_cond_broadcast(&mut (*ctx).queuePopCond);
    let mut i: size_t = 0;
    i = 0;
    while i < (*ctx).threadCapacity {
        pthread_join(
            *((*ctx).threads).offset(i as isize),
            NULL_0 as *mut *mut std::ffi::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
}
#[export_name = crate::prefix!(POOL_free)]
pub unsafe extern "C" fn POOL_free(mut ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    POOL_join(ctx);
    pthread_mutex_destroy(&mut (*ctx).queueMutex);
    pthread_cond_destroy(&mut (*ctx).queuePushCond);
    pthread_cond_destroy(&mut (*ctx).queuePopCond);
    ZSTD_customFree((*ctx).queue as *mut std::ffi::c_void, (*ctx).customMem);
    ZSTD_customFree((*ctx).threads as *mut std::ffi::c_void, (*ctx).customMem);
    ZSTD_customFree(ctx as *mut std::ffi::c_void, (*ctx).customMem);
}
#[export_name = crate::prefix!(POOL_joinJobs)]
pub unsafe extern "C" fn POOL_joinJobs(mut ctx: *mut POOL_ctx) {
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    while (*ctx).queueEmpty == 0 || (*ctx).numThreadsBusy > 0 {
        pthread_cond_wait(&mut (*ctx).queuePushCond, &mut (*ctx).queueMutex);
    }
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
}
#[export_name = crate::prefix!(ZSTD_freeThreadPool)]
pub unsafe extern "C" fn ZSTD_freeThreadPool(mut pool: *mut ZSTD_threadPool) {
    POOL_free(pool);
}
#[export_name = crate::prefix!(POOL_sizeof)]
pub unsafe extern "C" fn POOL_sizeof(mut ctx: *const POOL_ctx) -> size_t {
    if ctx.is_null() {
        return 0;
    }
    (::core::mem::size_of::<POOL_ctx>() as std::ffi::c_ulong)
        .wrapping_add(
            ((*ctx).queueSize)
                .wrapping_mul(::core::mem::size_of::<POOL_job>() as std::ffi::c_ulong),
        )
        .wrapping_add(
            ((*ctx).threadCapacity)
                .wrapping_mul(::core::mem::size_of::<pthread_t>() as std::ffi::c_ulong),
        )
}
unsafe extern "C" fn POOL_resize_internal(
    mut ctx: *mut POOL_ctx,
    mut numThreads: size_t,
) -> std::ffi::c_int {
    if numThreads <= (*ctx).threadCapacity {
        if numThreads == 0 {
            return 1;
        }
        (*ctx).threadLimit = numThreads;
        return 0;
    }
    let threadPool = ZSTD_customCalloc(
        numThreads.wrapping_mul(::core::mem::size_of::<pthread_t>() as std::ffi::c_ulong),
        (*ctx).customMem,
    ) as *mut pthread_t;
    if threadPool.is_null() {
        return 1;
    }
    libc::memcpy(
        threadPool as *mut std::ffi::c_void,
        (*ctx).threads as *const std::ffi::c_void,
        ((*ctx).threadCapacity)
            .wrapping_mul(::core::mem::size_of::<pthread_t>() as std::ffi::c_ulong)
            as libc::size_t,
    );
    ZSTD_customFree((*ctx).threads as *mut std::ffi::c_void, (*ctx).customMem);
    (*ctx).threads = threadPool;
    let mut threadId: size_t = 0;
    threadId = (*ctx).threadCapacity;
    while threadId < numThreads {
        if pthread_create(
            &mut *threadPool.offset(threadId as isize),
            std::ptr::null::<pthread_attr_t>(),
            std::mem::transmute(
                POOL_thread as unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void,
            ),
            ctx as *mut std::ffi::c_void,
        ) != 0
        {
            (*ctx).threadCapacity = threadId;
            return 1;
        }
        threadId = threadId.wrapping_add(1);
        threadId;
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    0
}
#[export_name = crate::prefix!(POOL_resize)]
pub unsafe extern "C" fn POOL_resize(
    mut ctx: *mut POOL_ctx,
    mut numThreads: size_t,
) -> std::ffi::c_int {
    let mut result: std::ffi::c_int = 0;
    if ctx.is_null() {
        return 1;
    }
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    result = POOL_resize_internal(ctx, numThreads);
    pthread_cond_broadcast(&mut (*ctx).queuePopCond);
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
    result
}
unsafe extern "C" fn isQueueFull(mut ctx: *const POOL_ctx) -> std::ffi::c_int {
    if (*ctx).queueSize > 1 {
        ((*ctx).queueHead == ((*ctx).queueTail).wrapping_add(1) % (*ctx).queueSize)
            as std::ffi::c_int
    } else {
        ((*ctx).numThreadsBusy == (*ctx).threadLimit || (*ctx).queueEmpty == 0) as std::ffi::c_int
    }
}
unsafe extern "C" fn POOL_add_internal(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut std::ffi::c_void,
) {
    let mut job = POOL_job {
        function: None,
        opaque: std::ptr::null_mut::<std::ffi::c_void>(),
    };
    job.function = function;
    job.opaque = opaque;
    if (*ctx).shutdown != 0 {
        return;
    }
    (*ctx).queueEmpty = 0;
    *((*ctx).queue).offset((*ctx).queueTail as isize) = job;
    (*ctx).queueTail = ((*ctx).queueTail).wrapping_add(1) % (*ctx).queueSize;
    pthread_cond_signal(&mut (*ctx).queuePopCond);
}
#[export_name = crate::prefix!(POOL_add)]
pub unsafe extern "C" fn POOL_add(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut std::ffi::c_void,
) {
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    while isQueueFull(ctx) != 0 && (*ctx).shutdown == 0 {
        pthread_cond_wait(&mut (*ctx).queuePushCond, &mut (*ctx).queueMutex);
    }
    POOL_add_internal(ctx, function, opaque);
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
}
#[export_name = crate::prefix!(POOL_tryAdd)]
pub unsafe extern "C" fn POOL_tryAdd(
    mut ctx: *mut POOL_ctx,
    mut function: POOL_function,
    mut opaque: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    pthread_mutex_lock(&mut (*ctx).queueMutex);
    if isQueueFull(ctx) != 0 {
        pthread_mutex_unlock(&mut (*ctx).queueMutex);
        return 0;
    }
    POOL_add_internal(ctx, function, opaque);
    pthread_mutex_unlock(&mut (*ctx).queueMutex);
    1
}
