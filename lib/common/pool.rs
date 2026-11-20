use core::ptr;
use std::sync::{Condvar, Mutex};
use std::thread::JoinHandle;

use crate::lib::common::allocations::{ZSTD_customCalloc, ZSTD_customFree};
use crate::lib::zstd::ZSTD_customMem;

pub struct POOL_ctx {
    customMem: ZSTD_customMem,
    /// Keep track of the threads
    threads: *mut JoinHandle<()>,
    threadCapacity: usize,
    threadLimit: usize,

    /// The queue is a circular buffer
    queue: *mut POOL_job,
    queueHead: usize,
    queueTail: usize,
    queueSize: usize,

    /// The number of threads working on jobs
    numThreadsBusy: usize,
    /// Indicates if the queue is empty
    queueEmpty: core::ffi::c_int,

    /// The mutex that protects the queue
    queueMutex: Mutex<()>,
    /// Condition variable for pushers to wait on when the queue is full
    queuePushCond: Condvar,
    /// Condition variables for poppers to wait on when the queue is empty
    queuePopCond: Condvar,
    /// Indicates if the queue is shutting down
    shutdown: core::ffi::c_int,
}

struct SendPoolCtxPtr(*mut POOL_ctx);
unsafe impl Send for SendPoolCtxPtr {}

/// A job is a function and an opaque argument
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct POOL_job {
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
}

/// The function type that can be added to a thread pool
pub type POOL_function = unsafe fn(*mut core::ffi::c_void);

pub type ZSTD_threadPool = POOL_ctx;

/// Work thread for the thread pool.
///
/// Waits for jobs and executes them.
unsafe fn POOL_thread(ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    loop {
        // Lock the mutex and wait for a non-empty queue or until shutdown
        let mut guard = (*ctx).queueMutex.lock().unwrap();
        #[expect(clippy::while_immutable_condition)]
        while (*ctx).queueEmpty != 0 || (*ctx).numThreadsBusy >= (*ctx).threadLimit {
            if (*ctx).shutdown != 0 {
                // Even if !queueEmpty, (possible if numThreadsBusy >= threadLimit),
                // a few threads will be shutdown while !queueEmpty, but enough threads
                // will remain active to finish the queue
                return;
            }
            guard = (*ctx).queuePopCond.wait(guard).unwrap();
        }

        // Pop a job off the queue
        let job = *((*ctx).queue).add((*ctx).queueHead);
        (*ctx).queueHead = ((*ctx).queueHead).wrapping_add(1) % (*ctx).queueSize;
        (*ctx).numThreadsBusy += 1;
        (*ctx).queueEmpty = ((*ctx).queueHead == (*ctx).queueTail) as core::ffi::c_int;

        // Unlock the mutex, signal a pusher, and run the job
        (*ctx).queuePushCond.notify_one();
        drop(guard);
        (job.function)(job.opaque);

        // If the intended queue size was 0, signal after finishing job
        guard = (*ctx).queueMutex.lock().unwrap();
        (*ctx).numThreadsBusy -= 1;
        (*ctx).queuePushCond.notify_one();
    }
}

/// Public access point to create thread pool
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_createThreadPool))]
pub unsafe extern "C" fn ZSTD_createThreadPool(numThreads: usize) -> *mut ZSTD_threadPool {
    POOL_create(numThreads, 0)
}

/// Create a thread pool with at most `numThreads` threads.
///
/// - `numThreads` must be at least 1.
/// - The maximum number of queued jobs before blocking is `queueSize`.
///
/// # Returns
///
/// - `POOL_ctx` pointer on success
/// - a null pointer otherwise
pub unsafe fn POOL_create(numThreads: usize, queueSize: usize) -> *mut POOL_ctx {
    POOL_create_advanced(numThreads, queueSize, ZSTD_customMem::default())
}

/// Same as [`POOL_create`], but with a [`ZSTD_customMem`].
pub(crate) unsafe fn POOL_create_advanced(
    numThreads: usize,
    queueSize: usize,
    customMem: ZSTD_customMem,
) -> *mut POOL_ctx {
    if numThreads == 0 {
        return core::ptr::null_mut();
    }
    let ctx = ZSTD_customCalloc(size_of::<POOL_ctx>(), customMem) as *mut POOL_ctx;
    if ctx.is_null() {
        return core::ptr::null_mut();
    }

    // Initialize the job queue.
    // It needs one extra space since one space is wasted to differentiate empty and full queues.
    (*ctx).queueSize = queueSize.wrapping_add(1);
    (*ctx).queue =
        ZSTD_customCalloc((*ctx).queueSize * size_of::<POOL_job>(), customMem) as *mut POOL_job;
    (*ctx).queueHead = 0;
    (*ctx).queueTail = 0;
    (*ctx).numThreadsBusy = 0;
    (*ctx).queueEmpty = 1;
    ptr::write(&raw mut (*ctx).queueMutex, Mutex::new(()));
    ptr::write(&raw mut (*ctx).queuePushCond, Condvar::new());
    ptr::write(&raw mut (*ctx).queuePopCond, Condvar::new());
    (*ctx).shutdown = 0;
    (*ctx).threads = ZSTD_customCalloc(numThreads * size_of::<JoinHandle<()>>(), customMem)
        as *mut JoinHandle<()>;
    (*ctx).threadCapacity = 0;
    (*ctx).customMem = customMem;
    if (*ctx).threads.is_null() || (*ctx).queue.is_null() {
        POOL_free(ctx);
        return core::ptr::null_mut();
    }
    // Initialize the threads
    for i in 0..numThreads {
        let ctx = SendPoolCtxPtr(ctx);
        core::ptr::write(
            (*ctx.0).threads.add(i),
            std::thread::spawn(|| {
                let ctx = ctx;
                POOL_thread(ctx.0)
            }),
        );
    }
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    ctx
}

/// Shutdown the queue, wake any sleeping threads, and join all of the threads.
unsafe fn POOL_join(ctx: *mut POOL_ctx) {
    let guard = (*ctx).queueMutex.lock().unwrap();
    (*ctx).shutdown = 1;
    drop(guard);
    (*ctx).queuePushCond.notify_all();
    (*ctx).queuePopCond.notify_all();
    for i in 0..(*ctx).threadCapacity {
        core::ptr::read((*ctx).threads.add(i)).join().unwrap();
    }
}

/// Free a thread pool returned by [`POOL_create`]
pub unsafe fn POOL_free(ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    POOL_join(ctx);
    ptr::drop_in_place(&raw mut (*ctx).queueMutex);
    ptr::drop_in_place(&raw mut (*ctx).queuePushCond);
    ptr::drop_in_place(&raw mut (*ctx).queuePopCond);
    ZSTD_customFree(
        (*ctx).queue as *mut core::ffi::c_void,
        (*ctx).queueSize * size_of::<POOL_job>(),
        (*ctx).customMem,
    );
    ZSTD_customFree(
        (*ctx).threads as *mut core::ffi::c_void,
        (*ctx).threadCapacity * size_of::<JoinHandle<()>>(),
        (*ctx).customMem,
    );
    ZSTD_customFree(
        ctx as *mut core::ffi::c_void,
        size_of::<POOL_ctx>(),
        (*ctx).customMem,
    );
}

/// Waits for all queued jobs to finish executing.
pub unsafe fn POOL_joinJobs(ctx: *mut POOL_ctx) {
    let mut guard = (*ctx).queueMutex.lock().unwrap();
    #[expect(clippy::while_immutable_condition)]
    while (*ctx).queueEmpty == 0 || (*ctx).numThreadsBusy > 0 {
        guard = (*ctx).queuePushCond.wait(guard).unwrap();
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeThreadPool))]
pub unsafe extern "C" fn ZSTD_freeThreadPool(pool: *mut ZSTD_threadPool) {
    POOL_free(pool);
}

/// # Returns
///
/// The memory usage of the threadpool.
///
/// Note: compatible with NULL (returns 0 in this case)
pub(crate) unsafe fn POOL_sizeof(ctx: *const POOL_ctx) -> usize {
    if ctx.is_null() {
        return 0;
    }
    size_of::<POOL_ctx>()
        + (*ctx).queueSize * size_of::<POOL_job>()
        + (*ctx).threadCapacity * size_of::<JoinHandle<()>>()
}

unsafe fn POOL_resize_internal(ctx: *mut POOL_ctx, numThreads: usize) -> Result<(), ()> {
    if numThreads <= (*ctx).threadCapacity {
        if numThreads == 0 {
            return Err(());
        }
        (*ctx).threadLimit = numThreads;
        return Ok(());
    }

    let threadPool = ZSTD_customCalloc(
        numThreads.wrapping_mul(size_of::<JoinHandle<()>>()),
        (*ctx).customMem,
    ) as *mut JoinHandle<()>;
    if threadPool.is_null() {
        return Err(());
    }
    // Replace existing thread pool
    libc::memcpy(
        threadPool as *mut core::ffi::c_void,
        (*ctx).threads as *const core::ffi::c_void,
        (*ctx).threadCapacity * size_of::<JoinHandle<()>>(),
    );
    ZSTD_customFree(
        (*ctx).threads as *mut core::ffi::c_void,
        (*ctx).threadCapacity * size_of::<JoinHandle<()>>(),
        (*ctx).customMem,
    );
    (*ctx).threads = threadPool;
    // Initialize additional threads
    for threadId in (*ctx).threadCapacity..numThreads {
        let ctx = SendPoolCtxPtr(ctx);
        core::ptr::write(
            ((*ctx.0).threads).add(threadId),
            std::thread::spawn(|| {
                let ctx = ctx;
                POOL_thread(ctx.0)
            }),
        );
    }
    // Successfully expanded
    (*ctx).threadCapacity = numThreads;
    (*ctx).threadLimit = numThreads;
    Ok(())
}

/// Expands or shrinks pool's number of threads.
///
/// This is more efficient than releasing + creating a new context,
/// since it tries to preserve and reuse existing threads.
///
/// - `numThreads` must be at least 1.
pub(crate) unsafe fn POOL_resize(ctx: *mut POOL_ctx, numThreads: usize) -> Result<(), ()> {
    if ctx.is_null() {
        return Err(());
    }
    let _guard = (*ctx).queueMutex.lock().unwrap();
    let result = POOL_resize_internal(ctx, numThreads);
    (*ctx).queuePopCond.notify_all();
    result
}

/// Returns true if the queue is full.
///
/// When queueSize is 1 (pool was created with an intended queueSize of 0),
/// then a queue is empty if there is a thread free _and_ no job is waiting.
unsafe fn isQueueFull(ctx: *const POOL_ctx) -> bool {
    if (*ctx).queueSize > 1 {
        (*ctx).queueHead == ((*ctx).queueTail).wrapping_add(1) % (*ctx).queueSize
    } else {
        (*ctx).numThreadsBusy == (*ctx).threadLimit || (*ctx).queueEmpty == 0
    }
}

unsafe fn POOL_add_internal(
    ctx: *mut POOL_ctx,
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
) {
    let job = POOL_job { function, opaque };
    if (*ctx).shutdown != 0 {
        return;
    }

    (*ctx).queueEmpty = 0;
    *((*ctx).queue).add((*ctx).queueTail) = job;
    (*ctx).queueTail = ((*ctx).queueTail).wrapping_add(1) % (*ctx).queueSize;
    (*ctx).queuePopCond.notify_one();
}

/// Add the job `function(opaque)` to the thread pool. `ctx` must be valid.
/// Possibly blocks until there is room in the queue.
///
/// Note: the function may be executed asynchronously, therefore, `opaque` must live until function has been completed.
pub unsafe fn POOL_add(
    ctx: *mut POOL_ctx,
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
) {
    let mut guard = (*ctx).queueMutex.lock().unwrap();
    // Wait until there is space in the queue for the new job
    while isQueueFull(ctx) && (*ctx).shutdown == 0 {
        guard = (*ctx).queuePushCond.wait(guard).unwrap();
    }
    POOL_add_internal(ctx, function, opaque);
}

/// Add the job `function(opaque)` to thread pool _if_ a queue slot is available.
/// Returns immediately even if not (does not block).
///
/// # Returns
///
/// - `Ok(())` on success
/// - `Err(())` if the queue is full
pub(crate) unsafe fn POOL_tryAdd(
    ctx: *mut POOL_ctx,
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
) -> Result<(), ()> {
    let _guard = (*ctx).queueMutex.lock().unwrap();
    if isQueueFull(ctx) {
        return Err(());
    }
    POOL_add_internal(ctx, function, opaque);
    Ok(())
}
