use core::ptr;
use std::sync::{Condvar, Mutex};

use libc::{calloc, free, pthread_attr_t, pthread_create, pthread_join, pthread_t, size_t};

use crate::lib::zstd::{ZSTD_customMem, ZSTD_defaultCMem};

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
    queueEmpty: core::ffi::c_int,
    queueMutex: Mutex<()>,
    queuePushCond: Condvar,
    queuePopCond: Condvar,
    shutdown: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct POOL_job {
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
}
pub type POOL_function = Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> ()>;
pub type ZSTD_threadPool = POOL_ctx;
#[inline]
unsafe extern "C" fn ZSTD_customCalloc(
    size: size_t,
    customMem: ZSTD_customMem,
) -> *mut core::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        let ptr = (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
        ptr::write_bytes(ptr, 0, size as libc::size_t);
        return ptr;
    }
    calloc(1, size)
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(ptr: *mut core::ffi::c_void, customMem: ZSTD_customMem) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree).unwrap_unchecked()(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}
unsafe fn POOL_thread(opaque: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let ctx = opaque as *mut POOL_ctx;
    if ctx.is_null() {
        return core::ptr::null_mut();
    }
    loop {
        let mut guard = (*ctx).queueMutex.lock().unwrap();
        while (*ctx).queueEmpty != 0 || (*ctx).numThreadsBusy >= (*ctx).threadLimit {
            if (*ctx).shutdown != 0 {
                return opaque;
            }
            guard = (*ctx).queuePopCond.wait(guard).unwrap();
        }
        let job = *((*ctx).queue).add((*ctx).queueHead);
        (*ctx).queueHead = ((*ctx).queueHead).wrapping_add(1) % (*ctx).queueSize;
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_add(1);
        (*ctx).numThreadsBusy;
        (*ctx).queueEmpty = ((*ctx).queueHead == (*ctx).queueTail) as core::ffi::c_int;
        (*ctx).queuePushCond.notify_one();
        drop(guard);
        (job.function).unwrap_unchecked()(job.opaque);
        guard = (*ctx).queueMutex.lock().unwrap();
        (*ctx).numThreadsBusy = ((*ctx).numThreadsBusy).wrapping_sub(1);
        (*ctx).numThreadsBusy;
        (*ctx).queuePushCond.notify_one();
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createThreadPool))]
pub unsafe extern "C" fn ZSTD_createThreadPool(numThreads: size_t) -> *mut ZSTD_threadPool {
    POOL_create(numThreads, 0)
}
pub unsafe fn POOL_create(numThreads: size_t, queueSize: size_t) -> *mut POOL_ctx {
    POOL_create_advanced(numThreads, queueSize, ZSTD_defaultCMem)
}
pub(crate) unsafe fn POOL_create_advanced(
    numThreads: size_t,
    queueSize: size_t,
    customMem: ZSTD_customMem,
) -> *mut POOL_ctx {
    let mut ctx = core::ptr::null_mut::<POOL_ctx>();
    if numThreads == 0 {
        return core::ptr::null_mut();
    }
    ctx = ZSTD_customCalloc(::core::mem::size_of::<POOL_ctx>(), customMem) as *mut POOL_ctx;
    if ctx.is_null() {
        return core::ptr::null_mut();
    }
    (*ctx).queueSize = queueSize.wrapping_add(1);
    (*ctx).queue = ZSTD_customCalloc(
        ((*ctx).queueSize).wrapping_mul(::core::mem::size_of::<POOL_job>()),
        customMem,
    ) as *mut POOL_job;
    (*ctx).queueHead = 0;
    (*ctx).queueTail = 0;
    (*ctx).numThreadsBusy = 0;
    (*ctx).queueEmpty = 1;
    ptr::write(ptr::addr_of_mut!((*ctx).queueMutex), Mutex::new(()));
    ptr::write(ptr::addr_of_mut!((*ctx).queuePushCond), Condvar::new());
    ptr::write(ptr::addr_of_mut!((*ctx).queuePopCond), Condvar::new());
    (*ctx).shutdown = 0;
    (*ctx).threads = ZSTD_customCalloc(
        numThreads.wrapping_mul(::core::mem::size_of::<pthread_t>()),
        customMem,
    ) as *mut pthread_t;
    (*ctx).threadCapacity = 0;
    (*ctx).customMem = customMem;
    if ((*ctx).threads).is_null() || ((*ctx).queue).is_null() {
        POOL_free(ctx);
        return core::ptr::null_mut();
    }
    let mut i: size_t = 0;
    i = 0;
    while i < numThreads {
        if pthread_create(
            &mut *((*ctx).threads).add(i),
            core::ptr::null::<pthread_attr_t>(),
            core::mem::transmute(
                POOL_thread as unsafe fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
            ),
            ctx as *mut core::ffi::c_void,
        ) != 0
        {
            (*ctx).threadCapacity = i;
            POOL_free(ctx);
            return core::ptr::null_mut();
        }
        i = i.wrapping_add(1);
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    ctx
}
unsafe fn POOL_join(ctx: *mut POOL_ctx) {
    let guard = (*ctx).queueMutex.lock().unwrap();
    (*ctx).shutdown = 1;
    drop(guard);
    (*ctx).queuePushCond.notify_all();
    (*ctx).queuePopCond.notify_all();
    let mut i: size_t = 0;
    i = 0;
    while i < (*ctx).threadCapacity {
        pthread_join(*((*ctx).threads).add(i), core::ptr::null_mut());
        i = i.wrapping_add(1);
    }
}
pub unsafe fn POOL_free(ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    POOL_join(ctx);
    ptr::drop_in_place(ptr::addr_of_mut!((*ctx).queueMutex));
    ptr::drop_in_place(ptr::addr_of_mut!((*ctx).queuePushCond));
    ptr::drop_in_place(ptr::addr_of_mut!((*ctx).queuePopCond));
    ZSTD_customFree((*ctx).queue as *mut core::ffi::c_void, (*ctx).customMem);
    ZSTD_customFree((*ctx).threads as *mut core::ffi::c_void, (*ctx).customMem);
    ZSTD_customFree(ctx as *mut core::ffi::c_void, (*ctx).customMem);
}
pub unsafe fn POOL_joinJobs(ctx: *mut POOL_ctx) {
    let mut guard = (*ctx).queueMutex.lock().unwrap();
    while (*ctx).queueEmpty == 0 || (*ctx).numThreadsBusy > 0 {
        guard = (*ctx).queuePushCond.wait(guard).unwrap();
    }
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeThreadPool))]
pub unsafe extern "C" fn ZSTD_freeThreadPool(pool: *mut ZSTD_threadPool) {
    POOL_free(pool);
}
pub(crate) unsafe fn POOL_sizeof(ctx: *const POOL_ctx) -> size_t {
    if ctx.is_null() {
        return 0;
    }
    (::core::mem::size_of::<POOL_ctx>())
        .wrapping_add(((*ctx).queueSize).wrapping_mul(::core::mem::size_of::<POOL_job>()))
        .wrapping_add(((*ctx).threadCapacity).wrapping_mul(::core::mem::size_of::<pthread_t>()))
}
unsafe fn POOL_resize_internal(ctx: *mut POOL_ctx, numThreads: size_t) -> core::ffi::c_int {
    if numThreads <= (*ctx).threadCapacity {
        if numThreads == 0 {
            return 1;
        }
        (*ctx).threadLimit = numThreads;
        return 0;
    }
    let threadPool = ZSTD_customCalloc(
        numThreads.wrapping_mul(::core::mem::size_of::<pthread_t>()),
        (*ctx).customMem,
    ) as *mut pthread_t;
    if threadPool.is_null() {
        return 1;
    }
    libc::memcpy(
        threadPool as *mut core::ffi::c_void,
        (*ctx).threads as *const core::ffi::c_void,
        ((*ctx).threadCapacity).wrapping_mul(::core::mem::size_of::<pthread_t>()),
    );
    ZSTD_customFree((*ctx).threads as *mut core::ffi::c_void, (*ctx).customMem);
    (*ctx).threads = threadPool;
    let mut threadId: size_t = 0;
    threadId = (*ctx).threadCapacity;
    while threadId < numThreads {
        if pthread_create(
            &mut *threadPool.add(threadId),
            core::ptr::null::<pthread_attr_t>(),
            core::mem::transmute(
                POOL_thread as unsafe fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
            ),
            ctx as *mut core::ffi::c_void,
        ) != 0
        {
            (*ctx).threadCapacity = threadId;
            return 1;
        }
        threadId = threadId.wrapping_add(1);
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    0
}
pub(crate) unsafe fn POOL_resize(ctx: *mut POOL_ctx, numThreads: size_t) -> core::ffi::c_int {
    let mut result: core::ffi::c_int = 0;
    if ctx.is_null() {
        return 1;
    }
    let _guard = (*ctx).queueMutex.lock().unwrap();
    result = POOL_resize_internal(ctx, numThreads);
    (*ctx).queuePopCond.notify_all();
    result
}
unsafe fn isQueueFull(ctx: *const POOL_ctx) -> core::ffi::c_int {
    if (*ctx).queueSize > 1 {
        ((*ctx).queueHead == ((*ctx).queueTail).wrapping_add(1) % (*ctx).queueSize)
            as core::ffi::c_int
    } else {
        ((*ctx).numThreadsBusy == (*ctx).threadLimit || (*ctx).queueEmpty == 0) as core::ffi::c_int
    }
}
unsafe fn POOL_add_internal(
    ctx: *mut POOL_ctx,
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
) {
    let mut job = POOL_job {
        function: None,
        opaque: core::ptr::null_mut::<core::ffi::c_void>(),
    };
    job.function = function;
    job.opaque = opaque;
    if (*ctx).shutdown != 0 {
        return;
    }
    (*ctx).queueEmpty = 0;
    *((*ctx).queue).add((*ctx).queueTail) = job;
    (*ctx).queueTail = ((*ctx).queueTail).wrapping_add(1) % (*ctx).queueSize;
    (*ctx).queuePopCond.notify_one();
}
pub unsafe fn POOL_add(
    ctx: *mut POOL_ctx,
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
) {
    let mut guard = (*ctx).queueMutex.lock().unwrap();
    while isQueueFull(ctx) != 0 && (*ctx).shutdown == 0 {
        guard = (*ctx).queuePushCond.wait(guard).unwrap();
    }
    POOL_add_internal(ctx, function, opaque);
}
pub(crate) unsafe fn POOL_tryAdd(
    ctx: *mut POOL_ctx,
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    let _guard = (*ctx).queueMutex.lock().unwrap();
    if isQueueFull(ctx) != 0 {
        return 0;
    }
    POOL_add_internal(ctx, function, opaque);
    1
}
