use core::mem::{self, MaybeUninit};
use std::sync::{Condvar, Mutex};
use std::thread::JoinHandle;

use crate::lib::common::allocations::ZstdAllocator;
use crate::lib::polyfill::{box_try_new_uninit_in, box_try_new_uninit_slice_in};
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
    threads: Vec<JoinHandle<()>, ZstdAllocator>,
    threadLimit: usize,

    /// The queue is a circular buffer, the slots in `queueHead..queueTail` hold the pending jobs
    queue: Box<[Option<POOL_job>], ZstdAllocator>,
    queueHead: usize,
    queueTail: usize,

    /// The number of threads working on jobs
    numThreadsBusy: usize,
    /// Indicates if the queue is shutting down
    shutdown: core::ffi::c_int,
}

struct SendPoolCtxPtr(*const POOL_ctx);
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
fn POOL_thread(ctx: &POOL_ctx) {
    loop {
        // Lock the mutex and wait for a non-empty queue or until shutdown
        let mut state = ctx.state.lock().unwrap();
        while state.isQueueEmpty() || state.numThreadsBusy >= state.threadLimit {
            if state.shutdown != 0 {
                // Even if the queue is not empty, (possible if numThreadsBusy >= threadLimit),
                // a few threads will be shutdown while the queue is not empty, but enough threads
                // will remain active to finish the queue
                return;
            }
            state = ctx.queuePopCond.wait(state).unwrap();
        }

        // Pop a job off the queue
        let queueHead = state.queueHead;
        let job = state.queue[queueHead].take().unwrap();
        state.queueHead = queueHead.wrapping_add(1) % state.queue.len();
        state.numThreadsBusy += 1;

        // Unlock the mutex, signal a pusher, and run the job
        ctx.queuePushCond.notify_one();
        drop(state);
        // SAFETY: The caller that queued the job guarantees it can run on a worker thread
        unsafe { (job.function)(job.opaque) };

        // If the intended queue size was 0, signal after finishing job
        state = ctx.state.lock().unwrap();
        state.numThreadsBusy -= 1;
        ctx.queuePushCond.notify_one();
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

    // Initialize the job queue.
    // It needs one extra space since one space is wasted to differentiate empty and full queues.
    let queueSize = queueSize + 1;
    let alloc = ZstdAllocator::new(customMem);
    let Ok(mut queue) = box_try_new_uninit_slice_in(queueSize, alloc) else {
        return core::ptr::null_mut();
    };
    queue.fill(MaybeUninit::new(None));
    // SAFETY: every element was initialized by the `fill` above.
    let queue = unsafe { queue.assume_init() };

    let Ok(ctx) = box_try_new_uninit_in(alloc) else {
        return core::ptr::null_mut();
    };
    let state = PoolState {
        threads: Vec::new_in(alloc),
        threadLimit: 0,
        queue,
        queueHead: 0,
        queueTail: 0,
        numThreadsBusy: 0,
        shutdown: 0,
    };
    let ctx = Box::write(
        ctx,
        POOL_ctx {
            customMem,
            state: Mutex::new(state),
            queuePushCond: Condvar::new(),
            queuePopCond: Condvar::new(),
        },
    );
    let (ctx, _) = Box::into_raw_with_allocator(ctx);
    // Initialize the threads
    if POOL_resize_internal(&*ctx, numThreads).is_err() {
        POOL_free(ctx);
        return core::ptr::null_mut();
    }
    ctx
}

/// Shutdown the queue, wake any sleeping threads, and join all of the threads.
fn POOL_join(ctx: &POOL_ctx) {
    let mut state = ctx.state.lock().unwrap();
    state.shutdown = 1;
    let alloc = ZstdAllocator::new(ctx.customMem);
    let threads = mem::replace(&mut state.threads, Vec::new_in(alloc));
    drop(state);
    ctx.queuePushCond.notify_all();
    ctx.queuePopCond.notify_all();
    for thread in threads {
        thread.join().unwrap();
    }
}

/// Free a thread pool returned by [`POOL_create`]
pub unsafe fn POOL_free(ctx: *mut POOL_ctx) {
    if ctx.is_null() {
        return;
    }
    POOL_join(&*ctx);
    drop(Box::from_raw_in(ctx, ZstdAllocator::new((*ctx).customMem)));
}

/// Waits for all queued jobs to finish executing.
pub fn POOL_joinJobs(ctx: &POOL_ctx) {
    let mut state = ctx.state.lock().unwrap();
    while !state.isQueueEmpty() || state.numThreadsBusy > 0 {
        state = ctx.queuePushCond.wait(state).unwrap();
    }
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_freeThreadPool))]
pub unsafe extern "C" fn ZSTD_freeThreadPool(pool: *mut ZSTD_threadPool) {
    POOL_free(pool);
}

/// Returns the memory usage of the thread pool, or zero for `None`.
pub(crate) fn POOL_sizeof(ctx: Option<&POOL_ctx>) -> usize {
    let Some(ctx) = ctx else { return 0 };
    let state = ctx.state.lock().unwrap();
    size_of::<POOL_ctx>()
        + state.queue.len() * size_of::<Option<POOL_job>>()
        + state.threads.capacity() * size_of::<JoinHandle<()>>()
}

/// # Safety
///
/// The caller must keep `ctx` valid until all workers spawned here have been joined.
unsafe fn POOL_resize_internal(ctx: &POOL_ctx, numThreads: usize) -> Result<(), ()> {
    let mut state = ctx.state.lock().unwrap();
    let existingThreads = state.threads.len();
    if numThreads <= existingThreads {
        if numThreads == 0 {
            return Err(());
        }
        state.threadLimit = numThreads;
        return Ok(());
    }

    state
        .threads
        .try_reserve_exact(numThreads - existingThreads)
        .map_err(|_| ())?;
    // Initialize additional threads
    for _ in existingThreads..numThreads {
        let ctx = SendPoolCtxPtr(ctx);
        state.threads.push(std::thread::spawn(|| {
            let ctx = ctx;
            // SAFETY: POOL_free joins this worker before freeing the pool
            POOL_thread(unsafe { &*ctx.0 })
        }));
    }
    // Successfully expanded
    state.threadLimit = numThreads;
    Ok(())
}

/// Expands or shrinks pool's number of threads.
///
/// This is more efficient than releasing + creating a new context,
/// since it tries to preserve and reuse existing threads.
///
/// - `numThreads` must be at least 1.
///
/// # Safety
///
/// The caller must keep the pool valid until all workers spawned here have been joined.
pub(crate) unsafe fn POOL_resize(ctx: Option<&POOL_ctx>, numThreads: usize) -> Result<(), ()> {
    let ctx = ctx.ok_or(())?;
    let result = POOL_resize_internal(ctx, numThreads);
    ctx.queuePopCond.notify_all();
    result
}

impl PoolState {
    /// Returns true if no job is waiting in the queue.
    fn isQueueEmpty(&self) -> bool {
        self.queue[self.queueHead].is_none()
    }

    /// Returns true if the queue is full.
    ///
    /// When `queue.len()` is 1 (pool was created with an intended queueSize of 0),
    /// then a queue is empty if there is a thread free _and_ no job is waiting.
    fn isQueueFull(&self) -> bool {
        match self.queue.len() {
            1 => self.numThreadsBusy == self.threadLimit || !self.isQueueEmpty(),
            n => self.queueHead == (self.queueTail + 1) % n,
        }
    }
}

impl POOL_ctx {
    /// # Safety
    ///
    /// The caller must ensure that `function(opaque)` can run safely on a worker thread
    /// and that any data it accesses stays valid until `function(opaque)` returns.
    unsafe fn add_internal(
        &self,
        state: &mut PoolState,
        function: POOL_function,
        opaque: *mut core::ffi::c_void,
    ) {
        let job = POOL_job { function, opaque };
        if state.shutdown != 0 {
            return;
        }

        let queueTail = state.queueTail;
        state.queue[queueTail] = Some(job);
        state.queueTail = (queueTail + 1) % state.queue.len();
        self.queuePopCond.notify_one();
    }

    /// Adds `function(opaque)` to the thread pool, waiting for room if the queue is full.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `function(opaque)` can run safely on a worker thread
    /// and that any data it accesses stays valid until `function(opaque)` returns.
    pub unsafe fn add(&self, function: POOL_function, opaque: *mut core::ffi::c_void) {
        let mut state = self.state.lock().unwrap();
        // Wait until there is space in the queue for the new job
        while state.isQueueFull() && state.shutdown == 0 {
            state = self.queuePushCond.wait(state).unwrap();
        }
        self.add_internal(&mut state, function, opaque);
    }

    /// Add the job `function(opaque)` to thread pool _if_ a queue slot is available.
    /// Returns `Err(())` immediately if not (does not block).
    ///
    /// # Safety
    ///
    /// If the job is enqueued, the caller must ensure that `function(opaque)` can run
    /// safely on a worker thread and that its data stays valid until `function(opaque)` returns.
    pub(crate) unsafe fn try_add(
        &self,
        function: POOL_function,
        opaque: *mut core::ffi::c_void,
    ) -> Result<(), ()> {
        let mut state = self.state.lock().unwrap();
        if state.isQueueFull() {
            return Err(());
        }
        self.add_internal(&mut state, function, opaque);
        Ok(())
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use core::ffi::c_void;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    const JOBS: usize = 16;

    #[test]
    fn test_create_zero_threads() {
        assert!(unsafe { POOL_create(0, 1) }.is_null());
    }

    unsafe fn increment(opaque: *mut c_void) {
        let counter = &*opaque.cast::<AtomicUsize>();
        counter.fetch_add(1, Ordering::Relaxed);
    }

    unsafe fn delayed_increment(opaque: *mut c_void) {
        thread::sleep(Duration::from_millis(1));
        increment(opaque);
    }

    // The callback must accept a shared AtomicUsize.
    unsafe fn check_jobs_complete(job: POOL_function) {
        for workers in 1..=4 {
            for queue_size in 0..=2 {
                let counts = [const { AtomicUsize::new(0) }; JOBS];
                let pool = POOL_create(workers, queue_size);
                assert!(!pool.is_null());
                for count in &counts {
                    (*pool).add(job, &raw const *count as *mut c_void);
                }
                POOL_free(pool);
                assert_eq!(
                    counts.map(AtomicUsize::into_inner),
                    [1; JOBS],
                    "workers={workers}, queue_size={queue_size}"
                );
            }
        }
    }

    #[test]
    fn test_order() {
        unsafe { check_jobs_complete(increment) }
    }

    #[test]
    fn test_wait() {
        unsafe { check_jobs_complete(delayed_increment) }
    }

    #[derive(Default)]
    struct JobCounts {
        active: usize,
        peak: usize,
    }

    struct Concurrency {
        expected: usize,
        counts: Mutex<JobCounts>,
        started: Condvar,
    }

    unsafe fn measure_concurrency(opaque: *mut c_void) {
        let jobs = &*opaque.cast::<Concurrency>();
        let mut counts = jobs.counts.lock().unwrap();
        counts.active += 1;
        counts.peak = counts.peak.max(counts.active);
        if counts.active == jobs.expected {
            jobs.started.notify_all();
        }
        // Let the first wave overlap even if some workers are slow to start.
        let (counts, _) = jobs
            .started
            .wait_timeout_while(counts, Duration::from_secs(5), |counts| {
                counts.peak < jobs.expected
            })
            .unwrap();
        drop(counts);
        thread::sleep(Duration::from_millis(10));
        jobs.counts.lock().unwrap().active -= 1;
    }

    unsafe fn assert_peak_concurrency(pool: *mut POOL_ctx, expected: usize) {
        let jobs = Concurrency {
            expected,
            counts: Mutex::default(),
            started: Condvar::new(),
        };
        for _ in 0..JOBS {
            (*pool).add(measure_concurrency, (&raw const jobs).cast_mut().cast());
        }
        POOL_joinJobs(&*pool);
        let counts = jobs.counts.into_inner().unwrap();
        assert_eq!(counts.active, 0);
        assert_eq!(counts.peak, expected);
    }

    #[test]
    fn test_thread_reduction() {
        unsafe {
            let pool = mem::DropGuard::new(POOL_create(4, 2), |pool| POOL_free(pool));
            assert!(!pool.is_null());
            assert_peak_concurrency(*pool, 4);
            POOL_resize(pool.as_ref(), 2).unwrap();
            assert_peak_concurrency(*pool, 2);
        }
    }

    #[test]
    fn test_abrupt_ending() {
        unsafe fn count_job(opaque: *mut c_void) {
            let completed = &*opaque.cast::<Mutex<usize>>();
            *completed.lock().unwrap() += 1;
        }

        let completed = Mutex::new(0usize);
        unsafe {
            let pool = POOL_create(3, JOBS);
            assert!(!pool.is_null());
            // Hold the jobs until the pool has shrunk with work still queued.
            let guard = completed.lock().unwrap();
            for _ in 0..JOBS {
                (*pool).add(count_job, (&raw const completed).cast_mut().cast());
            }
            let resized = POOL_resize(pool.as_ref(), 1);
            drop(guard);
            POOL_free(pool);
            assert!(resized.is_ok());
        }
        assert_eq!(completed.into_inner().unwrap(), JOBS);
    }
}
