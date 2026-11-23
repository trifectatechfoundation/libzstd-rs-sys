use core::ptr;
use std::sync::{Condvar, Mutex};
use std::thread::JoinHandle;

use crate::lib::common::allocations::{ZSTD_customCalloc, ZSTD_customFree};
use crate::lib::zstd::ZSTD_customMem;

pub struct POOL_ctx {
    customMem: ZSTD_customMem,

    /// The mutex that protects the queue
    state: Mutex<PoolState>,
    /// Condition variable for pushers to wait on when the queue is full
    queuePushCond: Condvar,
    /// Condition variables for poppers to wait on when the queue is empty
    queuePopCond: Condvar,
}

/// The part of [`POOL_ctx`] that is protected by its mutex
struct PoolState {
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
        let mut state = (*ctx).state.lock().unwrap();
        while state.queueEmpty != 0 || state.numThreadsBusy >= state.threadLimit {
            if state.shutdown != 0 {
                // Even if !queueEmpty, (possible if numThreadsBusy >= threadLimit),
                // a few threads will be shutdown while !queueEmpty, but enough threads
                // will remain active to finish the queue
                return;
            }
            state = (*ctx).queuePopCond.wait(state).unwrap();
        }

        // Pop a job off the queue
        let job = *state.queue.add(state.queueHead);
        state.queueHead = state.queueHead.wrapping_add(1) % state.queueSize;
        state.numThreadsBusy += 1;
        state.queueEmpty = (state.queueHead == state.queueTail) as core::ffi::c_int;

        // Unlock the mutex, signal a pusher, and run the job
        (*ctx).queuePushCond.notify_one();
        drop(state);
        (job.function)(job.opaque);

        // If the intended queue size was 0, signal after finishing job
        state = (*ctx).state.lock().unwrap();
        state.numThreadsBusy -= 1;
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
    let queueSize = queueSize.wrapping_add(1);
    let queue = ZSTD_customCalloc(queueSize * size_of::<POOL_job>(), customMem) as *mut POOL_job;
    let threads = ZSTD_customCalloc(numThreads * size_of::<JoinHandle<()>>(), customMem)
        as *mut JoinHandle<()>;
    let state = PoolState {
        threads,
        threadCapacity: 0,
        threadLimit: 0,
        queue,
        queueHead: 0,
        queueTail: 0,
        queueSize,
        numThreadsBusy: 0,
        queueEmpty: 1,
        shutdown: 0,
    };
    ptr::write(
        ctx,
        POOL_ctx {
            customMem,
            state: Mutex::new(state),
            queuePushCond: Condvar::new(),
            queuePopCond: Condvar::new(),
        },
    );
    if threads.is_null() || queue.is_null() {
        POOL_free(ctx);
        return core::ptr::null_mut();
    }
    // Initialize the threads
    for i in 0..numThreads {
        let ctx = SendPoolCtxPtr(ctx);
        core::ptr::write(
            threads.add(i),
            std::thread::spawn(|| {
                let ctx = ctx;
                POOL_thread(ctx.0)
            }),
        );
    }
    let mut state = (*ctx).state.lock().unwrap();
    state.threadCapacity = numThreads;
    state.threadLimit = numThreads;
    drop(state);
    ctx
}

/// Shutdown the queue, wake any sleeping threads, and join all of the threads.
unsafe fn POOL_join(ctx: *mut POOL_ctx) {
    let mut state = (*ctx).state.lock().unwrap();
    state.shutdown = 1;
    let threads = state.threads;
    let threadCapacity = state.threadCapacity;
    drop(state);
    (*ctx).queuePushCond.notify_all();
    (*ctx).queuePopCond.notify_all();
    for i in 0..threadCapacity {
        core::ptr::read(threads.add(i)).join().unwrap();
    }
}

/// Free a thread pool returned by [`POOL_create`]
pub unsafe fn POOL_free(ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    POOL_join(ctx);
    // All threads have been joined, so we have exclusive access to the state
    let PoolState {
        queue,
        queueSize,
        threads,
        threadCapacity,
        ..
    } = *(*ctx).state.get_mut().unwrap();
    ptr::drop_in_place(&raw mut (*ctx).state);
    ptr::drop_in_place(&raw mut (*ctx).queuePushCond);
    ptr::drop_in_place(&raw mut (*ctx).queuePopCond);
    ZSTD_customFree(
        queue as *mut core::ffi::c_void,
        queueSize * size_of::<POOL_job>(),
        (*ctx).customMem,
    );
    ZSTD_customFree(
        threads as *mut core::ffi::c_void,
        threadCapacity * size_of::<JoinHandle<()>>(),
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
    let mut state = (*ctx).state.lock().unwrap();
    while state.queueEmpty == 0 || state.numThreadsBusy > 0 {
        state = (*ctx).queuePushCond.wait(state).unwrap();
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
    let state = (*ctx).state.lock().unwrap();
    size_of::<POOL_ctx>()
        + state.queueSize * size_of::<POOL_job>()
        + state.threadCapacity * size_of::<JoinHandle<()>>()
}

unsafe fn POOL_resize_internal(
    ctx: *mut POOL_ctx,
    state: &mut PoolState,
    numThreads: usize,
) -> Result<(), ()> {
    if numThreads <= state.threadCapacity {
        if numThreads == 0 {
            return Err(());
        }
        state.threadLimit = numThreads;
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
    ptr::copy_nonoverlapping(state.threads, threadPool, state.threadCapacity);
    ZSTD_customFree(
        state.threads as *mut core::ffi::c_void,
        state.threadCapacity * size_of::<JoinHandle<()>>(),
        (*ctx).customMem,
    );
    state.threads = threadPool;
    // Initialize additional threads
    for threadId in state.threadCapacity..numThreads {
        let ctx = SendPoolCtxPtr(ctx);
        core::ptr::write(
            state.threads.add(threadId),
            std::thread::spawn(|| {
                let ctx = ctx;
                POOL_thread(ctx.0)
            }),
        );
    }
    // Successfully expanded
    state.threadCapacity = numThreads;
    state.threadLimit = numThreads;
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
    let mut state = (*ctx).state.lock().unwrap();
    let result = POOL_resize_internal(ctx, &mut state, numThreads);
    (*ctx).queuePopCond.notify_all();
    result
}

impl PoolState {
    /// Returns true if the queue is full.
    ///
    /// When queueSize is 1 (pool was created with an intended queueSize of 0),
    /// then a queue is empty if there is a thread free _and_ no job is waiting.
    fn isQueueFull(&self) -> bool {
        if self.queueSize > 1 {
            self.queueHead == self.queueTail.wrapping_add(1) % self.queueSize
        } else {
            self.numThreadsBusy == self.threadLimit || self.queueEmpty == 0
        }
    }
}

unsafe fn POOL_add_internal(
    ctx: *mut POOL_ctx,
    state: &mut PoolState,
    function: POOL_function,
    opaque: *mut core::ffi::c_void,
) {
    let job = POOL_job { function, opaque };
    if state.shutdown != 0 {
        return;
    }

    state.queueEmpty = 0;
    *state.queue.add(state.queueTail) = job;
    state.queueTail = state.queueTail.wrapping_add(1) % state.queueSize;
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
    let mut state = (*ctx).state.lock().unwrap();
    // Wait until there is space in the queue for the new job
    while state.isQueueFull() && state.shutdown == 0 {
        state = (*ctx).queuePushCond.wait(state).unwrap();
    }
    POOL_add_internal(ctx, &mut state, function, opaque);
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
    let mut state = (*ctx).state.lock().unwrap();
    if state.isQueueFull() {
        return Err(());
    }
    POOL_add_internal(ctx, &mut state, function, opaque);
    Ok(())
}
