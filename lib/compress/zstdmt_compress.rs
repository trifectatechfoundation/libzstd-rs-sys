use core::ptr;
use std::sync::{Condvar, Mutex};

use libc::size_t;

use crate::lib::common::allocations::{ZSTD_customCalloc, ZSTD_customFree, ZSTD_customMalloc};
use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::{MEM_32bits, MEM_writeLE32};
use crate::lib::common::pool::{
    POOL_create_advanced, POOL_free, POOL_resize, POOL_sizeof, POOL_tryAdd, ZSTD_threadPool,
};
use crate::lib::common::xxhash::{
    XXH64_state_t, ZSTD_XXH64_digest, ZSTD_XXH64_reset, ZSTD_XXH64_update_slice,
};
use crate::lib::compress::zstd_compress::{
    rawSeq, RawSeqStore_t, ZSTD_CCtx, ZSTD_CCtxParams_setParameter, ZSTD_CCtx_params,
    ZSTD_CCtx_trace, ZSTD_CDict, ZSTD_compressBegin_advanced_internal, ZSTD_compressBound,
    ZSTD_compressContinue_public, ZSTD_compressEnd_public, ZSTD_createCCtx_advanced,
    ZSTD_createCDict_advanced, ZSTD_cycleLog, ZSTD_freeCCtx, ZSTD_freeCDict,
    ZSTD_getCParamsFromCCtxParams, ZSTD_invalidateRepCodes, ZSTD_referenceExternalSequences,
    ZSTD_sizeof_CCtx, ZSTD_sizeof_CDict, ZSTD_window_t, ZSTD_writeLastEmptyBlock,
};
use crate::lib::compress::zstd_compress_internal::{ZSTD_window_hasExtDict, ZSTD_window_update};
use crate::lib::compress::zstd_ldm::{
    ldmEntry_t, ldmParams_t, ldmState_t, ZSTD_ldm_adjustParameters, ZSTD_ldm_fillHashTable,
    ZSTD_ldm_generateSequences, ZSTD_ldm_getMaxNbSeq,
};
use crate::lib::zstd::{
    ZSTD_ParamSwitch_e, ZSTD_cParameter, ZSTD_customMem, ZSTD_dct_auto, ZSTD_dct_rawContent,
    ZSTD_dictContentType_e, ZSTD_dlm_byCopy, ZSTD_dlm_byRef, ZSTD_frameProgression, ZSTD_inBuffer,
    ZSTD_outBuffer_s, ZSTD_strategy, ZSTD_BLOCKSIZELOG_MAX, ZSTD_BLOCKSIZE_MAX,
    ZSTD_CONTENTSIZE_UNKNOWN,
};

#[repr(C)]
pub struct ZSTDMT_CCtx {
    factory: *mut ZSTD_threadPool,
    jobs: *mut ZSTDMT_jobDescription,
    bufPool: *mut ZSTDMT_bufferPool,
    cctxPool: *mut ZSTDMT_CCtxPool,
    seqPool: *mut ZSTDMT_seqPool,
    params: ZSTD_CCtx_params,
    targetSectionSize: size_t,
    targetPrefixSize: size_t,
    jobReady: core::ffi::c_int,
    inBuff: InBuff_t,
    roundBuff: RoundBuff_t,
    serial: SerialState,
    rsync: RSyncState_t,
    jobIDMask: core::ffi::c_uint,
    doneJobID: core::ffi::c_uint,
    nextJobID: core::ffi::c_uint,
    frameEnded: core::ffi::c_uint,
    allJobsCompleted: core::ffi::c_uint,
    frameContentSize: core::ffi::c_ulonglong,
    consumed: core::ffi::c_ulonglong,
    produced: core::ffi::c_ulonglong,
    cMem: ZSTD_customMem,
    cdictLocal: *mut ZSTD_CDict,
    cdict: *const ZSTD_CDict,
    providedFactory: bool,
}

#[repr(C)]
struct RSyncState_t {
    hash: u64,
    hitMask: u64,
    primePower: u64,
}

struct SerialState {
    // All variables in the struct are protected by mutex.
    mutex: Mutex<()>,
    cond: Condvar,
    params: ZSTD_CCtx_params,
    ldmState: ldmState_t,
    xxhState: XXH64_state_t,
    nextJobID: core::ffi::c_uint,
    // Protects ldmWindow.
    // Must be acquired after the main mutex when acquiring both.
    ldmWindowMutex: Mutex<()>,
    ldmWindowCond: Condvar,   // Signaled when ldmWindow is updated
    ldmWindow: ZSTD_window_t, // A thread-safe copy of ldmState.window
}

#[derive(Copy, Clone)]
#[repr(C)]
struct RoundBuff_t {
    buffer: *mut u8,
    capacity: size_t,
    pos: size_t,
}

#[repr(C)]
struct InBuff_t {
    prefix: Range,
    buffer: Buffer,
    filled: size_t,
}
type Buffer = buffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
struct buffer_s {
    start: *mut core::ffi::c_void,
    capacity: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct Range {
    start: *const core::ffi::c_void,
    size: size_t,
}

type ZSTDMT_seqPool = ZSTDMT_bufferPool;
type ZSTDMT_bufferPool = ZSTDMT_bufferPool_s;

struct ZSTDMT_bufferPool_s {
    poolMutex: Mutex<()>,
    bufferSize: size_t,
    totalBuffers: core::ffi::c_uint,
    nbBuffers: core::ffi::c_uint,
    cMem: ZSTD_customMem,
    buffers: *mut Buffer,
}

struct ZSTDMT_CCtxPool {
    poolMutex: Mutex<()>,
    totalCCtx: core::ffi::c_int,
    availCCtx: core::ffi::c_int,
    cMem: ZSTD_customMem,
    cctxs: *mut *mut ZSTD_CCtx,
}

struct ZSTDMT_jobDescription {
    /// SHARED - set0 by mtctx, then modified by worker AND read by mtctx
    consumed: size_t,
    /// SHARED - set0 by mtctx, then modified by worker AND read by mtctx, then set0 by mtctx
    cSize: size_t,
    /// Thread-safe - used by mtctx and worker
    job_mutex: Mutex<()>,
    /// Thread-safe - used by mtctx and worker
    job_cond: Condvar,
    /// Thread-safe - used by mtctx and (all) workers
    cctxPool: *mut ZSTDMT_CCtxPool,
    /// Thread-safe - used by mtctx and (all) workers
    bufPool: *mut ZSTDMT_bufferPool,
    /// Thread-safe - used by mtctx and (all) workers
    seqPool: *mut ZSTDMT_seqPool,
    /// Thread-safe - used by mtctx and (all) workers
    serial: *mut SerialState,
    /// set by worker (or mtctx), then read by worker & mtctx, then modified by mtctx => no barrier
    dstBuff: Buffer,
    /// set by mtctx, then read by worker & mtctx => no barrier
    prefix: Range,
    /// set by mtctx, then read by worker & mtctx => no barrier
    src: Range,
    /// set by mtctx, then read by worker => no barrier
    jobID: core::ffi::c_uint,
    /// set by mtctx, then read by worker => no barrier
    firstJob: core::ffi::c_uint,
    /// set by mtctx, then read by worker => no barrier
    lastJob: core::ffi::c_uint,
    /// set by mtctx, then read by worker => no barrier
    params: ZSTD_CCtx_params,
    /// set by mtctx, then read by worker => no barrier
    cdict: *const ZSTD_CDict,
    /// set by mtctx, then read by worker => no barrier
    fullFrameSize: core::ffi::c_ulonglong,
    /// used only by mtctx
    dstFlushed: size_t,
    /// used only by mtctx
    frameChecksumNeeded: core::ffi::c_uint,
}

type ZSTD_outBuffer = ZSTD_outBuffer_s;
type ZSTD_EndDirective = core::ffi::c_uint;
const ZSTD_e_end: ZSTD_EndDirective = 2;
const ZSTD_e_flush: ZSTD_EndDirective = 1;
const ZSTD_e_continue: ZSTD_EndDirective = 0;
type ZSTD_dictTableLoadMethod_e = core::ffi::c_uint;
const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;

#[repr(C)]
struct SyncPoint {
    toLoad: size_t,
    flush: core::ffi::c_int,
}

type ZSTD_CParamMode_e = core::ffi::c_uint;
const ZSTD_cpm_noAttachDict: ZSTD_CParamMode_e = 0;
const ZSTD_c_forceMaxWindow: ZSTD_cParameter = ZSTD_cParameter::ZSTD_c_experimentalParam3;
const ZSTD_c_deterministicRefPrefix: ZSTD_cParameter = ZSTD_cParameter::ZSTD_c_experimentalParam15;

const kNullRawSeqStore: RawSeqStore_t = RawSeqStore_t {
    seq: core::ptr::null_mut(),
    pos: 0,
    posInSequence: 0,
    size: 0,
    capacity: 0,
};

const ZSTD_WINDOW_START_INDEX: core::ffi::c_int = 2;
static prime8bytes: u64 = 0xcf1bbcdcb7a56463 as core::ffi::c_ulonglong;

/// Return base^exponent
fn ZSTD_ipow(mut base: u64, mut exponent: u64) -> u64 {
    let mut power = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            power *= base;
        }
        exponent >>= 1;
        base = base * base;
    }
    power
}

const ZSTD_ROLL_HASH_CHAR_OFFSET: core::ffi::c_int = 10;

/// Add the buffer to the hash value
unsafe fn ZSTD_rollingHash_append(
    mut hash: u64,
    buf: *const core::ffi::c_void,
    size: size_t,
) -> u64 {
    let istart = buf as *const u8;
    for pos in 0..size {
        hash *= prime8bytes;
        hash = hash.wrapping_add(
            (*istart.add(pos) as core::ffi::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64,
        );
    }
    hash
}

/// Compute the rolling hash value of the buffer.
#[inline]
unsafe fn ZSTD_rollingHash_compute(buf: *const core::ffi::c_void, size: size_t) -> u64 {
    ZSTD_rollingHash_append(0, buf, size)
}

/// Compute the primePower to be passed to ZSTD_rollingHash_rotate() for a hash
/// over a window of length bytes.
#[inline]
fn ZSTD_rollingHash_primePower(length: u32) -> u64 {
    ZSTD_ipow(prime8bytes, length.wrapping_sub(1) as u64)
}

/// Rotate the rolling hash by one byte.
#[inline]
fn ZSTD_rollingHash_rotate(mut hash: u64, toRemove: u8, toAdd: u8, primePower: u64) -> u64 {
    hash = hash.wrapping_sub(
        (toRemove as core::ffi::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64 * primePower,
    );
    hash *= prime8bytes;
    hash = hash.wrapping_add((toAdd as core::ffi::c_int + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64);
    hash
}

/// Clears the window containing the history by simply setting it to empty.
#[inline]
unsafe fn ZSTD_window_clear(window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).offset_from((*window).base) as size_t;
    let end = endT as u32;

    (*window).lowLimit = end;
    (*window).dictLimit = end;
}

#[inline]
unsafe fn ZSTD_window_init(window: *mut ZSTD_window_t) {
    ptr::write_bytes(window as *mut u8, 0, size_of::<ZSTD_window_t>());
    (*window).base = c" ".as_ptr() as *const u8;
    (*window).dictBase = c" ".as_ptr() as *const u8;
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32; // start from >0, so that 1st position is valid
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32; // it ensures first and later CCtx usages compress the same
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize);
    (*window).nbOverflowCorrections = 0;
}

const ZSTDMT_JOBSIZE_MIN: core::ffi::c_int = 512 * ((1) << 10);

static mut g_nullBuffer: Buffer = buffer_s {
    start: core::ptr::null_mut(),
    capacity: 0,
};

unsafe fn ZSTDMT_freeBufferPool(bufPool: *mut ZSTDMT_bufferPool) {
    if bufPool.is_null() {
        return;
    }

    if !((*bufPool).buffers).is_null() {
        for u in 0..(*bufPool).totalBuffers {
            ZSTD_customFree(
                (*((*bufPool).buffers).offset(u as isize)).start,
                (*((*bufPool).buffers).offset(u as isize)).capacity,
                (*bufPool).cMem,
            );
        }
        ZSTD_customFree(
            (*bufPool).buffers as *mut core::ffi::c_void,
            (*bufPool).bufferSize,
            (*bufPool).cMem,
        );
    }

    core::ptr::drop_in_place(core::ptr::addr_of_mut!((*bufPool).poolMutex));
    ZSTD_customFree(
        bufPool as *mut core::ffi::c_void,
        size_of::<ZSTDMT_bufferPool>(),
        (*bufPool).cMem,
    );
}

unsafe fn ZSTDMT_createBufferPool(
    maxNbBuffers: core::ffi::c_uint,
    cMem: ZSTD_customMem,
) -> *mut ZSTDMT_bufferPool {
    let bufPool = ZSTD_customCalloc(size_of::<ZSTDMT_bufferPool>(), cMem) as *mut ZSTDMT_bufferPool;
    if bufPool.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::write(
        core::ptr::addr_of_mut!((*bufPool).poolMutex),
        Mutex::new(()),
    );
    (*bufPool).buffers = ZSTD_customCalloc(
        (maxNbBuffers as usize).wrapping_mul(size_of::<Buffer>()),
        cMem,
    ) as *mut Buffer;
    if ((*bufPool).buffers).is_null() {
        ZSTDMT_freeBufferPool(bufPool);
        return core::ptr::null_mut();
    }
    (*bufPool).bufferSize = (64 * ((1) << 10)) as size_t;
    (*bufPool).totalBuffers = maxNbBuffers;
    (*bufPool).nbBuffers = 0;
    (*bufPool).cMem = cMem;
    bufPool
}

/// Only works at initialization, not during compression.
unsafe fn ZSTDMT_sizeof_bufferPool(bufPool: *mut ZSTDMT_bufferPool) -> size_t {
    let poolSize = size_of::<ZSTDMT_bufferPool>();
    let arraySize = ((*bufPool).totalBuffers as size_t).wrapping_mul(size_of::<Buffer>());
    let mut totalBufferSize = 0 as size_t;
    let _guard = (*bufPool).poolMutex.lock().unwrap();
    for u in 0..(*bufPool).totalBuffers {
        totalBufferSize =
            totalBufferSize.wrapping_add((*((*bufPool).buffers).offset(u as isize)).capacity);
    }

    poolSize
        .wrapping_add(arraySize)
        .wrapping_add(totalBufferSize)
}

/// All future buffers provided by this buffer pool will have _at least_ this size.
///
/// Note: it's better for all buffers to have same size, as they become freely
/// interchangeable, reducing malloc/free usages and memory fragmentation.
unsafe fn ZSTDMT_setBufferSize(bufPool: *mut ZSTDMT_bufferPool, bSize: size_t) {
    let _guard = (*bufPool).poolMutex.lock().unwrap();
    (*bufPool).bufferSize = bSize;
}

unsafe fn ZSTDMT_expandBufferPool(
    srcBufPool: *mut ZSTDMT_bufferPool,
    maxNbBuffers: core::ffi::c_uint,
) -> *mut ZSTDMT_bufferPool {
    if srcBufPool.is_null() {
        return core::ptr::null_mut();
    }

    if (*srcBufPool).totalBuffers >= maxNbBuffers {
        return srcBufPool; // good enough
    }

    // need a larger buffer pool
    let cMem = (*srcBufPool).cMem;
    let bSize = (*srcBufPool).bufferSize; // forward parameters
    let mut newBufPool = core::ptr::null_mut::<ZSTDMT_bufferPool>();
    ZSTDMT_freeBufferPool(srcBufPool);
    newBufPool = ZSTDMT_createBufferPool(maxNbBuffers, cMem);
    if newBufPool.is_null() {
        return newBufPool;
    }
    ZSTDMT_setBufferSize(newBufPool, bSize);
    newBufPool
}

/// Assumption: bufPool must be valid
///
/// Note: allocation may fail, in this case, start==NULL and size==0
///
/// # Returns
///
/// A buffer, with start pointer and size.
unsafe fn ZSTDMT_getBuffer(bufPool: *mut ZSTDMT_bufferPool) -> Buffer {
    let bSize = (*bufPool).bufferSize;
    let guard = (*bufPool).poolMutex.lock().unwrap();
    if (*bufPool).nbBuffers != 0 {
        // try to use an existing buffer
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_sub(1);
        let buf = *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize);
        let availBufferSize = buf.capacity;
        *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize) = g_nullBuffer;
        if (availBufferSize >= bSize) as core::ffi::c_int
            & (availBufferSize >> 3 <= bSize) as core::ffi::c_int
            != 0
        {
            // large enough, but not too much
            return buf;
        }
        // size conditions not respected: scratch this buffer, create new one
        ZSTD_customFree(buf.start, buf.capacity, (*bufPool).cMem);
    }
    drop(guard);

    let mut buffer = buffer_s {
        start: core::ptr::null_mut::<core::ffi::c_void>(),
        capacity: 0,
    };
    let start = ZSTD_customMalloc(bSize, (*bufPool).cMem);
    buffer.start = start; // note : start can be NULL if malloc fails
    buffer.capacity = if start.is_null() { 0 } else { bSize };
    start.is_null();
    buffer
}

/// Store buffer for later re-use, up to pool capacity.
unsafe fn ZSTDMT_releaseBuffer(bufPool: *mut ZSTDMT_bufferPool, buf: Buffer) {
    if (buf.start).is_null() {
        return;
    }
    let guard = (*bufPool).poolMutex.lock().unwrap();
    if (*bufPool).nbBuffers < (*bufPool).totalBuffers {
        let fresh0 = (*bufPool).nbBuffers;
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_add(1);
        *((*bufPool).buffers).offset(fresh0 as isize) = buf; // stored for later use
        return;
    }
    drop(guard);

    // Reached bufferPool capacity (note: should not happen)
    ZSTD_customFree(buf.start, buf.capacity, (*bufPool).cMem);
}

unsafe fn ZSTDMT_sizeof_seqPool(seqPool: *mut ZSTDMT_seqPool) -> size_t {
    ZSTDMT_sizeof_bufferPool(seqPool)
}

fn bufferToSeq(buffer: Buffer) -> RawSeqStore_t {
    let mut seq = kNullRawSeqStore;
    seq.seq = buffer.start as *mut rawSeq;
    seq.capacity = (buffer.capacity).wrapping_div(size_of::<rawSeq>());
    seq
}

fn seqToBuffer(seq: RawSeqStore_t) -> Buffer {
    let mut buffer = buffer_s {
        start: core::ptr::null_mut::<core::ffi::c_void>(),
        capacity: 0,
    };
    buffer.start = seq.seq as *mut core::ffi::c_void;
    buffer.capacity = (seq.capacity).wrapping_mul(size_of::<rawSeq>());
    buffer
}

unsafe fn ZSTDMT_getSeq(seqPool: *mut ZSTDMT_seqPool) -> RawSeqStore_t {
    if (*seqPool).bufferSize == 0 {
        return kNullRawSeqStore;
    }
    bufferToSeq(ZSTDMT_getBuffer(seqPool))
}

unsafe fn ZSTDMT_releaseSeq(seqPool: *mut ZSTDMT_seqPool, seq: RawSeqStore_t) {
    ZSTDMT_releaseBuffer(seqPool, seqToBuffer(seq));
}

unsafe fn ZSTDMT_setNbSeq(seqPool: *mut ZSTDMT_seqPool, nbSeq: size_t) {
    ZSTDMT_setBufferSize(seqPool, nbSeq.wrapping_mul(size_of::<rawSeq>()));
}

unsafe fn ZSTDMT_createSeqPool(
    nbWorkers: core::ffi::c_uint,
    cMem: ZSTD_customMem,
) -> *mut ZSTDMT_seqPool {
    let seqPool = ZSTDMT_createBufferPool(nbWorkers, cMem);
    if seqPool.is_null() {
        return core::ptr::null_mut();
    }
    ZSTDMT_setNbSeq(seqPool, 0);
    seqPool
}

unsafe fn ZSTDMT_freeSeqPool(seqPool: *mut ZSTDMT_seqPool) {
    ZSTDMT_freeBufferPool(seqPool);
}

unsafe fn ZSTDMT_expandSeqPool(pool: *mut ZSTDMT_seqPool, nbWorkers: u32) -> *mut ZSTDMT_seqPool {
    ZSTDMT_expandBufferPool(pool, nbWorkers)
}

/// Note: all CCtx borrowed from the pool must be reverted back to the pool _before_ freeing the pool
unsafe fn ZSTDMT_freeCCtxPool(pool: *mut ZSTDMT_CCtxPool) {
    if pool.is_null() {
        return;
    }
    core::ptr::drop_in_place(core::ptr::addr_of_mut!((*pool).poolMutex));
    if !((*pool).cctxs).is_null() {
        for cid in 0..(*pool).totalCCtx {
            ZSTD_freeCCtx(*((*pool).cctxs).offset(cid as isize));
        }
        ZSTD_customFree(
            (*pool).cctxs as *mut core::ffi::c_void,
            ((*pool).totalCCtx as usize).wrapping_mul(size_of::<*mut ZSTD_CCtx>()),
            (*pool).cMem,
        );
    }
    ZSTD_customFree(
        pool as *mut core::ffi::c_void,
        size_of::<ZSTDMT_CCtxPool>(),
        (*pool).cMem,
    );
}

/// implies nbWorkers >= 1, checked by caller ZSTDMT_createCCtx()
unsafe fn ZSTDMT_createCCtxPool(
    nbWorkers: core::ffi::c_int,
    cMem: ZSTD_customMem,
) -> *mut ZSTDMT_CCtxPool {
    let cctxPool = ZSTD_customCalloc(size_of::<ZSTDMT_CCtxPool>(), cMem) as *mut ZSTDMT_CCtxPool;
    if cctxPool.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::write(
        core::ptr::addr_of_mut!((*cctxPool).poolMutex),
        Mutex::new(()),
    );
    (*cctxPool).totalCCtx = nbWorkers;
    (*cctxPool).cctxs = ZSTD_customCalloc(
        (nbWorkers as usize).wrapping_mul(size_of::<*mut ZSTD_CCtx>()),
        cMem,
    ) as *mut *mut ZSTD_CCtx;
    if ((*cctxPool).cctxs).is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return core::ptr::null_mut();
    }
    (*cctxPool).cMem = cMem;
    let fresh1 = &mut (*((*cctxPool).cctxs));
    *fresh1 = ZSTD_createCCtx_advanced(cMem);
    if (*((*cctxPool).cctxs)).is_null() {
        ZSTDMT_freeCCtxPool(cctxPool);
        return core::ptr::null_mut();
    }
    (*cctxPool).availCCtx = 1; // at least one cctx for single-thread mode
    cctxPool
}

unsafe fn ZSTDMT_expandCCtxPool(
    srcPool: *mut ZSTDMT_CCtxPool,
    nbWorkers: core::ffi::c_int,
) -> *mut ZSTDMT_CCtxPool {
    if srcPool.is_null() {
        return core::ptr::null_mut();
    }
    if nbWorkers <= (*srcPool).totalCCtx {
        return srcPool; // good enough
    }

    // need a larger cctx pool
    let cMem = (*srcPool).cMem;
    ZSTDMT_freeCCtxPool(srcPool);
    ZSTDMT_createCCtxPool(nbWorkers, cMem)
}

/// Only works during initialization phase, not during compression.
unsafe fn ZSTDMT_sizeof_CCtxPool(cctxPool: *mut ZSTDMT_CCtxPool) -> size_t {
    let _guard = (*cctxPool).poolMutex.lock().unwrap();
    let nbWorkers = (*cctxPool).totalCCtx as core::ffi::c_uint;
    let poolSize = size_of::<ZSTDMT_CCtxPool>();
    let arraySize = ((*cctxPool).totalCCtx as usize).wrapping_mul(size_of::<*mut ZSTD_CCtx>());
    let mut totalCCtxSize = 0 as size_t;
    for u in 0..nbWorkers {
        totalCCtxSize =
            totalCCtxSize.wrapping_add(ZSTD_sizeof_CCtx(*((*cctxPool).cctxs).offset(u as isize)));
    }
    poolSize.wrapping_add(arraySize).wrapping_add(totalCCtxSize)
}

unsafe fn ZSTDMT_getCCtx(cctxPool: *mut ZSTDMT_CCtxPool) -> *mut ZSTD_CCtx {
    let _guard = (*cctxPool).poolMutex.lock().unwrap();
    if (*cctxPool).availCCtx != 0 {
        (*cctxPool).availCCtx -= 1;
        let cctx = *((*cctxPool).cctxs).offset((*cctxPool).availCCtx as isize);
        return cctx;
    }
    ZSTD_createCCtx_advanced((*cctxPool).cMem) // note: can be NULL, when creation fails!
}

unsafe fn ZSTDMT_releaseCCtx(pool: *mut ZSTDMT_CCtxPool, cctx: *mut ZSTD_CCtx) {
    if cctx.is_null() {
        return;
    }

    let _guard = (*pool).poolMutex.lock().unwrap();
    if (*pool).availCCtx < (*pool).totalCCtx {
        let fresh2 = (*pool).availCCtx;
        (*pool).availCCtx += 1;
        let fresh3 = &mut (*((*pool).cctxs).offset(fresh2 as isize));
        *fresh3 = cctx;
    } else {
        // pool overflow: should not happen, since totalCCtx==nbWorkers
        ZSTD_freeCCtx(cctx);
    }
}

unsafe fn ZSTDMT_serialState_reset(
    serialState: &mut SerialState,
    seqPool: *mut ZSTDMT_seqPool,
    mut params: ZSTD_CCtx_params,
    jobSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
) -> core::ffi::c_int {
    // Adjust parameters
    if params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams, &params.cParams);
    } else {
        ptr::write_bytes(
            &mut params.ldmParams as *mut ldmParams_t as *mut u8,
            0,
            size_of::<ldmParams_t>(),
        );
    }
    serialState.nextJobID = 0;
    if params.fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(&mut serialState.xxhState, 0);
    }
    if params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        let cMem = params.customMem;
        let hashLog = params.ldmParams.hashLog;
        let hashSize = ((1 as size_t) << hashLog).wrapping_mul(size_of::<ldmEntry_t>());
        let bucketLog = (params.ldmParams.hashLog).wrapping_sub(params.ldmParams.bucketSizeLog);
        let prevBucketLog = (serialState.params.ldmParams.hashLog)
            .wrapping_sub(serialState.params.ldmParams.bucketSizeLog);
        let numBuckets = (1) << bucketLog;
        // Size the seq pool tables
        ZSTDMT_setNbSeq(seqPool, ZSTD_ldm_getMaxNbSeq(params.ldmParams, jobSize));
        // Reset the window
        ZSTD_window_init(&mut serialState.ldmState.window);
        // Resize tables and output space if necessary.
        if (serialState.ldmState.hashTable).is_null()
            || serialState.params.ldmParams.hashLog < hashLog
        {
            ZSTD_customFree(
                serialState.ldmState.hashTable as *mut core::ffi::c_void,
                hashSize,
                cMem,
            );
            serialState.ldmState.hashTable = ZSTD_customMalloc(hashSize, cMem) as *mut ldmEntry_t;
        }
        if (serialState.ldmState.bucketOffsets).is_null() || prevBucketLog < bucketLog {
            ZSTD_customFree(
                serialState.ldmState.bucketOffsets as *mut core::ffi::c_void,
                1 << prevBucketLog,
                cMem,
            );
            serialState.ldmState.bucketOffsets = ZSTD_customMalloc(numBuckets, cMem) as *mut u8;
        }
        if (serialState.ldmState.hashTable).is_null()
            || (serialState.ldmState.bucketOffsets).is_null()
        {
            return 1;
        }
        // Zero the tables
        ptr::write_bytes(serialState.ldmState.hashTable as *mut u8, 0, hashSize);
        ptr::write_bytes(serialState.ldmState.bucketOffsets, 0, numBuckets);

        // Update window state and fill hash table with dict
        serialState.ldmState.loadedDictEnd = 0;
        if dictSize > 0
            && dictContentType as core::ffi::c_uint
                == ZSTD_dct_rawContent as core::ffi::c_int as core::ffi::c_uint
        {
            let dictEnd = (dict as *const u8).add(dictSize);
            ZSTD_window_update(&mut serialState.ldmState.window, dict, dictSize, false);
            ZSTD_ldm_fillHashTable(
                &mut serialState.ldmState,
                dict as *const u8,
                dictEnd,
                &params.ldmParams,
            );
            serialState.ldmState.loadedDictEnd = if params.forceWindow != 0 {
                0
            } else {
                dictEnd.offset_from(serialState.ldmState.window.base) as core::ffi::c_long as u32
            };
        }

        // Initialize serialState's copy of ldmWindow.
        serialState.ldmWindow = serialState.ldmState.window;
    }

    serialState.params = params;
    serialState.params.jobSize = jobSize as u32 as size_t;

    0
}

unsafe fn ZSTDMT_serialState_init(serialState: *mut SerialState) -> core::ffi::c_int {
    ptr::write_bytes(serialState as *mut u8, 0, size_of::<SerialState>());
    core::ptr::write(
        core::ptr::addr_of_mut!((*serialState).mutex),
        Mutex::new(()),
    );
    core::ptr::write(core::ptr::addr_of_mut!((*serialState).cond), Condvar::new());
    core::ptr::write(
        core::ptr::addr_of_mut!((*serialState).ldmWindowMutex),
        Mutex::new(()),
    );
    core::ptr::write(
        core::ptr::addr_of_mut!((*serialState).ldmWindowCond),
        Condvar::new(),
    );
    0
}

unsafe fn ZSTDMT_serialState_free(serialState: *mut SerialState) {
    let cMem = (*serialState).params.customMem;
    core::ptr::drop_in_place(core::ptr::addr_of_mut!((*serialState).mutex));
    core::ptr::drop_in_place(core::ptr::addr_of_mut!((*serialState).cond));
    core::ptr::drop_in_place(core::ptr::addr_of_mut!((*serialState).ldmWindowMutex));
    core::ptr::drop_in_place(core::ptr::addr_of_mut!((*serialState).ldmWindowCond));
    let hashLog = (*serialState).params.ldmParams.hashLog;
    let hashSize = ((1 as size_t) << hashLog).wrapping_mul(size_of::<ldmEntry_t>());
    let bucketLog = ((*serialState).params.ldmParams.hashLog)
        .wrapping_sub((*serialState).params.ldmParams.bucketSizeLog);
    let numBuckets = 1usize << bucketLog;
    ZSTD_customFree(
        (*serialState).ldmState.hashTable as *mut core::ffi::c_void,
        hashSize,
        cMem,
    );
    ZSTD_customFree(
        (*serialState).ldmState.bucketOffsets as *mut core::ffi::c_void,
        numBuckets,
        cMem,
    );
}

unsafe fn ZSTDMT_serialState_genSequences(
    serialState: *mut SerialState,
    seqStore: &mut RawSeqStore_t,
    src: Range,
    jobID: core::ffi::c_uint,
) {
    // Wait for our turn
    let mut guard = (*serialState).mutex.lock().unwrap();
    #[expect(clippy::while_immutable_condition)]
    while (*serialState).nextJobID < jobID {
        guard = (*serialState).cond.wait(guard).unwrap();
    }

    // A future job may error and skip our job
    if (*serialState).nextJobID == jobID {
        // It is now our turn, do any processing necessary
        if (*serialState).params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
            ZSTD_window_update(
                &mut (*serialState).ldmState.window,
                src.start,
                src.size,
                false,
            );
            let error = ZSTD_ldm_generateSequences(
                &mut (*serialState).ldmState,
                seqStore,
                &(*serialState).params.ldmParams,
                src.start,
                src.size,
            );

            // We provide a large enough buffer to never fail.
            assert!(!ERR_isError(error));

            // Update ldmWindow to match the ldmState.window and signal the main
            // thread if it is waiting for a buffer.
            let _guard = (*serialState).ldmWindowMutex.lock().unwrap();
            (*serialState).ldmWindow = (*serialState).ldmState.window;
            (*serialState).ldmWindowCond.notify_one();
        }
        if (*serialState).params.fParams.checksumFlag != 0 && src.size > 0 {
            ZSTD_XXH64_update_slice(
                &mut (*serialState).xxhState,
                core::slice::from_raw_parts(src.start as *const u8, src.size),
            );
        }
    }

    // Now it is the next job's turn
    (*serialState).nextJobID = ((*serialState).nextJobID).wrapping_add(1);
    (*serialState).cond.notify_all();
}

unsafe fn ZSTDMT_serialState_applySequences(
    _serialState: *const SerialState,
    jobCCtx: *mut ZSTD_CCtx,
    seqStore: &RawSeqStore_t,
) {
    if seqStore.size > 0 {
        ZSTD_referenceExternalSequences(jobCCtx, seqStore.seq, seqStore.size);
    }
}

unsafe fn ZSTDMT_serialState_ensureFinished(
    serialState: *mut SerialState,
    jobID: core::ffi::c_uint,
    _cSize: size_t,
) {
    let _guard = (*serialState).mutex.lock().unwrap();
    if (*serialState).nextJobID <= jobID {
        // Skipping past job because of error
        (*serialState).nextJobID = jobID.wrapping_add(1);
        (*serialState).cond.notify_all();
        let _guard = (*serialState).ldmWindowMutex.lock().unwrap();
        ZSTD_window_clear(&mut (*serialState).ldmWindow);
        (*serialState).ldmWindowCond.notify_one();
    }
}

static mut kNullRange: Range = Range {
    start: core::ptr::null(),
    size: 0,
};

/// This is a POOL_function type
unsafe fn ZSTDMT_compressionJob(jobDescription: *mut core::ffi::c_void) {
    let mut current_block: u64;
    let job = jobDescription as *mut ZSTDMT_jobDescription;
    let mut jobParams = (*job).params; // do not modify job->params ! copy it, modify the copy
    let cctx = ZSTDMT_getCCtx((*job).cctxPool);
    let mut rawSeqStore = ZSTDMT_getSeq((*job).seqPool);
    let mut dstBuff = (*job).dstBuff;
    let mut lastCBlockSize = 0;

    if cctx.is_null() {
        let guard = (*job).job_mutex.lock().unwrap();
        (*job).cSize = Error::memory_allocation.to_error_code();
        drop(guard);
    } else {
        if (dstBuff.start).is_null() {
            // streaming job: doesn't provide a dstBuffer
            dstBuff = ZSTDMT_getBuffer((*job).bufPool);
            if (dstBuff.start).is_null() {
                let guard = (*job).job_mutex.lock().unwrap();
                (*job).cSize = Error::memory_allocation.to_error_code();
                drop(guard);
                current_block = 17100290475540901977;
            } else {
                (*job).dstBuff = dstBuff; // this value can be read in ZSTDMT_flush, when it copies the whole job
                current_block = 7976072742316086414;
            }
        } else {
            current_block = 7976072742316086414;
        }

        match current_block {
            17100290475540901977 => {}
            _ => {
                if jobParams.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable
                    && (rawSeqStore.seq).is_null()
                {
                    let guard = (*job).job_mutex.lock().unwrap();
                    (*job).cSize = Error::memory_allocation.to_error_code();
                    drop(guard);
                } else {
                    // Don't compute the checksum for chunks, since we compute it externally,
                    // but write it in the header.
                    if (*job).jobID != 0 {
                        jobParams.fParams.checksumFlag = 0;
                    }

                    // Don't run LDM for the chunks, since we handle it externally
                    jobParams.ldmParams.enableLdm = ZSTD_ParamSwitch_e::ZSTD_ps_disable;
                    // Correct nbWorkers to 0.
                    jobParams.nbWorkers = 0;

                    // init

                    // Perform serial step as early as possible
                    ZSTDMT_serialState_genSequences(
                        (*job).serial,
                        &mut rawSeqStore,
                        (*job).src,
                        (*job).jobID,
                    );

                    if !((*job).cdict).is_null() {
                        let initError = ZSTD_compressBegin_advanced_internal(
                            cctx,
                            core::ptr::null(),
                            0,
                            ZSTD_dct_auto,
                            ZSTD_dtlm_fast,
                            (*job).cdict,
                            &jobParams,
                            (*job).fullFrameSize,
                        );
                        if ERR_isError(initError) {
                            let guard = (*job).job_mutex.lock().unwrap();
                            (*job).cSize = initError;
                            drop(guard);
                            current_block = 17100290475540901977;
                        } else {
                            current_block = 16738040538446813684;
                        }
                    } else {
                        let pledgedSrcSize = if (*job).firstJob != 0 {
                            (*job).fullFrameSize
                        } else {
                            (*job).src.size as core::ffi::c_ulonglong
                        };
                        let forceWindowError = ZSTD_CCtxParams_setParameter(
                            &mut jobParams,
                            ZSTD_c_forceMaxWindow as ZSTD_cParameter,
                            ((*job).firstJob == 0) as core::ffi::c_int,
                        );
                        if ERR_isError(forceWindowError) {
                            let guard = (*job).job_mutex.lock().unwrap();
                            (*job).cSize = forceWindowError;
                            drop(guard);
                            current_block = 17100290475540901977;
                        } else {
                            if (*job).firstJob == 0 {
                                let err = ZSTD_CCtxParams_setParameter(
                                    &mut jobParams,
                                    ZSTD_c_deterministicRefPrefix as ZSTD_cParameter,
                                    0,
                                );
                                if ERR_isError(err) {
                                    let guard = (*job).job_mutex.lock().unwrap();
                                    (*job).cSize = err;
                                    drop(guard);
                                    current_block = 17100290475540901977;
                                } else {
                                    current_block = 2543120759711851213;
                                }
                            } else {
                                current_block = 2543120759711851213;
                            }

                            match current_block {
                                17100290475540901977 => {}
                                _ => {
                                    let initError_0 = ZSTD_compressBegin_advanced_internal(
                                        cctx,
                                        (*job).prefix.start,
                                        (*job).prefix.size,
                                        ZSTD_dct_rawContent,
                                        ZSTD_dtlm_fast,
                                        core::ptr::null(),
                                        &jobParams,
                                        pledgedSrcSize as core::ffi::c_ulonglong,
                                    );
                                    if ERR_isError(initError_0) {
                                        let guard = (*job).job_mutex.lock().unwrap();
                                        (*job).cSize = initError_0;
                                        drop(guard);
                                        current_block = 17100290475540901977;
                                    } else {
                                        current_block = 16738040538446813684;
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        17100290475540901977 => {}
                        _ => {
                            // External Sequences can only be applied after CCtx initialization
                            ZSTDMT_serialState_applySequences((*job).serial, cctx, &rawSeqStore);

                            if (*job).firstJob == 0 {
                                // flush and overwrite frame header when it's not first job
                                let hSize = ZSTD_compressContinue_public(
                                    cctx,
                                    dstBuff.start,
                                    dstBuff.capacity,
                                    (*job).src.start,
                                    0,
                                );
                                if ERR_isError(hSize) {
                                    let guard = (*job).job_mutex.lock().unwrap();
                                    (*job).cSize = hSize;
                                    drop(guard);
                                    current_block = 17100290475540901977;
                                } else {
                                    ZSTD_invalidateRepCodes(cctx);
                                    current_block = 6560072651652764009;
                                }
                            } else {
                                current_block = 6560072651652764009;
                            }

                            match current_block {
                                17100290475540901977 => {}
                                _ => {
                                    // compress the entire job by smaller chunks, for better granularity
                                    let chunkSize = (4 * ZSTD_BLOCKSIZE_MAX) as size_t;
                                    let nbChunks = (((*job).src.size)
                                        .wrapping_add(chunkSize.wrapping_sub(1))
                                        / chunkSize)
                                        as core::ffi::c_int;
                                    let mut ip = (*job).src.start as *const u8;
                                    let ostart = dstBuff.start as *mut u8;
                                    let mut op = ostart;
                                    let oend = op.add(dstBuff.capacity);
                                    let mut chunkNb: core::ffi::c_int = 0;

                                    if size_of::<size_t>() > size_of::<i32>() {
                                        /* check overflow */
                                        assert!(
                                            ((*job).src.size as u64)
                                                < i32::MAX as u64 * chunkSize as u64
                                        );
                                    }
                                    assert!((*job).cSize == 0);

                                    chunkNb = 1;
                                    loop {
                                        if chunkNb >= nbChunks {
                                            current_block = 851619935621435220;
                                            break;
                                        }
                                        let cSize = ZSTD_compressContinue_public(
                                            cctx,
                                            op as *mut core::ffi::c_void,
                                            oend.offset_from_unsigned(op),
                                            ip as *const core::ffi::c_void,
                                            chunkSize,
                                        );
                                        if ERR_isError(cSize) {
                                            let guard = (*job).job_mutex.lock().unwrap();
                                            (*job).cSize = cSize;
                                            drop(guard);
                                            current_block = 17100290475540901977;
                                            break;
                                        } else {
                                            ip = ip.add(chunkSize);
                                            op = op.add(cSize);
                                            // stats
                                            let guard = (*job).job_mutex.lock().unwrap();
                                            (*job).cSize = ((*job).cSize).wrapping_add(cSize);
                                            (*job).consumed = chunkSize * chunkNb as size_t;
                                            (*job).job_cond.notify_one(); // warns some more data is ready to be flushed
                                            drop(guard);
                                            chunkNb += 1;
                                        }
                                    }

                                    match current_block {
                                        17100290475540901977 => {}
                                        _ => {
                                            // last block
                                            if (nbChunks > 0) as core::ffi::c_int
                                                as core::ffi::c_uint
                                                | (*job).lastJob
                                                != 0
                                            {
                                                // must output a "last block" flag
                                                let lastBlockSize1 =
                                                    (*job).src.size & chunkSize.wrapping_sub(1);
                                                let lastBlockSize = if (lastBlockSize1 == 0)
                                                    as core::ffi::c_int
                                                    & ((*job).src.size >= chunkSize)
                                                        as core::ffi::c_int
                                                    != 0
                                                {
                                                    chunkSize
                                                } else {
                                                    lastBlockSize1
                                                };
                                                let cSize_0 = if (*job).lastJob != 0 {
                                                    ZSTD_compressEnd_public(
                                                        cctx,
                                                        op as *mut core::ffi::c_void,
                                                        oend.offset_from(op) as core::ffi::c_long
                                                            as size_t,
                                                        ip as *const core::ffi::c_void,
                                                        lastBlockSize,
                                                    )
                                                } else {
                                                    ZSTD_compressContinue_public(
                                                        cctx,
                                                        op as *mut core::ffi::c_void,
                                                        oend.offset_from(op) as core::ffi::c_long
                                                            as size_t,
                                                        ip as *const core::ffi::c_void,
                                                        lastBlockSize,
                                                    )
                                                };
                                                if ERR_isError(cSize_0) {
                                                    let guard = (*job).job_mutex.lock().unwrap();
                                                    (*job).cSize = cSize_0;
                                                    drop(guard);
                                                    current_block = 17100290475540901977;
                                                } else {
                                                    lastCBlockSize = cSize_0;
                                                    current_block = 200744462051969938;
                                                }
                                            } else {
                                                current_block = 200744462051969938;
                                            }
                                            match current_block {
                                                17100290475540901977 => {}
                                                _ => {
                                                    if (*job).firstJob == 0 {
                                                        // Double check that we don't have an ext-dict, because then our
                                                        // repcode invalidation doesn't work.
                                                        assert!(!ZSTD_window_hasExtDict(
                                                            (*cctx).blockState.matchState.window
                                                        ));
                                                    }
                                                    ZSTD_CCtx_trace(cctx, 0);
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

    ZSTDMT_serialState_ensureFinished((*job).serial, (*job).jobID, (*job).cSize);

    // release resources
    ZSTDMT_releaseSeq((*job).seqPool, rawSeqStore);
    ZSTDMT_releaseCCtx((*job).cctxPool, cctx);

    // report
    let _guard = (*job).job_mutex.lock().unwrap();
    if ERR_isError((*job).cSize) {
        assert_eq!(lastCBlockSize, 0);
    }
    (*job).cSize = ((*job).cSize).wrapping_add(lastCBlockSize);
    (*job).consumed = (*job).src.size; // when job->consumed == job->src.size , compression job is presumed completed
    (*job).job_cond.notify_one();
}

static mut kNullRoundBuff: RoundBuff_t = RoundBuff_t {
    buffer: core::ptr::null_mut(),
    capacity: 0,
    pos: 0,
};

const RSYNC_LENGTH: core::ffi::c_int = 32;

/// Don't create chunks smaller than the zstd block size.
/// This stops us from regressing compression ratio too much,
/// and ensures our output fits in ZSTD_compressBound().
///
/// If this is shrunk < ZSTD_BLOCKSIZELOG_MIN then
/// ZSTD_COMPRESSBOUND() will need to be updated.
const RSYNC_MIN_BLOCK_LOG: core::ffi::c_int = ZSTD_BLOCKSIZELOG_MAX;
const RSYNC_MIN_BLOCK_SIZE: core::ffi::c_int = (1) << RSYNC_MIN_BLOCK_LOG;

unsafe fn ZSTDMT_freeJobsTable(
    jobTable: *mut ZSTDMT_jobDescription,
    nbJobs: u32,
    cMem: ZSTD_customMem,
) {
    let mut jobNb: u32 = 0;
    if jobTable.is_null() {
        return;
    }

    jobNb = 0;
    while jobNb < nbJobs {
        core::ptr::drop_in_place(core::ptr::addr_of_mut!(
            (*jobTable.offset(jobNb as isize)).job_mutex
        ));
        core::ptr::drop_in_place(core::ptr::addr_of_mut!(
            (*jobTable.offset(jobNb as isize)).job_cond
        ));
        jobNb = jobNb.wrapping_add(1);
    }

    ZSTD_customFree(
        jobTable as *mut core::ffi::c_void,
        (nbJobs as usize).wrapping_mul(size_of::<ZSTDMT_jobDescription>()),
        cMem,
    );
}

/// Allocate and init a job table.
/// Updates *nbJobsPtr to the next power of 2 value, as size of table.
unsafe fn ZSTDMT_createJobsTable(
    nbJobsPtr: *mut u32,
    cMem: ZSTD_customMem,
) -> *mut ZSTDMT_jobDescription {
    let nbJobsLog2 = (ZSTD_highbit32(*nbJobsPtr)).wrapping_add(1);
    let nbJobs = ((1) << nbJobsLog2) as u32;
    let mut jobNb: u32 = 0;

    let jobTable = ZSTD_customCalloc(
        (nbJobs as usize).wrapping_mul(size_of::<ZSTDMT_jobDescription>()),
        cMem,
    ) as *mut ZSTDMT_jobDescription;
    if jobTable.is_null() {
        return core::ptr::null_mut();
    }

    *nbJobsPtr = nbJobs;
    jobNb = 0;
    while jobNb < nbJobs {
        core::ptr::write(
            core::ptr::addr_of_mut!((*jobTable.offset(jobNb as isize)).job_mutex),
            Mutex::new(()),
        );
        core::ptr::write(
            core::ptr::addr_of_mut!((*jobTable.offset(jobNb as isize)).job_cond),
            Condvar::new(),
        );
        jobNb = jobNb.wrapping_add(1);
    }

    jobTable
}

unsafe fn ZSTDMT_expandJobsTable(mtctx: *mut ZSTDMT_CCtx, nbWorkers: u32) -> size_t {
    let mut nbJobs = nbWorkers.wrapping_add(2);
    if nbJobs > ((*mtctx).jobIDMask).wrapping_add(1) {
        // need more job capacity
        ZSTDMT_freeJobsTable(
            (*mtctx).jobs,
            ((*mtctx).jobIDMask).wrapping_add(1),
            (*mtctx).cMem,
        );
        (*mtctx).jobIDMask = 0;
        (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, (*mtctx).cMem);
        if ((*mtctx).jobs).is_null() {
            return Error::memory_allocation.to_error_code();
        }
        (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    }

    0
}

/// Internal use only.
unsafe fn ZSTDMT_CCtxParam_setNbWorkers(
    params: &mut ZSTD_CCtx_params,
    nbWorkers: core::ffi::c_uint,
) -> size_t {
    ZSTD_CCtxParams_setParameter(
        params,
        ZSTD_cParameter::ZSTD_c_nbWorkers,
        nbWorkers as core::ffi::c_int,
    )
}

#[inline]
unsafe fn ZSTDMT_createCCtx_advanced_internal(
    mut nbWorkers: core::ffi::c_uint,
    cMem: ZSTD_customMem,
    pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    let mut nbJobs = nbWorkers.wrapping_add(2);
    let mut initError: core::ffi::c_int = 0;
    if nbWorkers < 1 {
        return core::ptr::null_mut();
    }
    nbWorkers = nbWorkers.min(
        (if size_of::<*mut core::ffi::c_void>() as core::ffi::c_ulong == 4 {
            64
        } else {
            256
        }) as core::ffi::c_uint,
    );

    let mtctx = ZSTD_customCalloc(size_of::<ZSTDMT_CCtx>(), cMem) as *mut ZSTDMT_CCtx;
    if mtctx.is_null() {
        return core::ptr::null_mut();
    }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    (*mtctx).cMem = cMem;
    (*mtctx).allJobsCompleted = 1;
    if !pool.is_null() {
        (*mtctx).factory = pool;
        (*mtctx).providedFactory = true;
    } else {
        (*mtctx).factory = POOL_create_advanced(nbWorkers as size_t, 0, cMem);
        (*mtctx).providedFactory = false;
    }
    (*mtctx).jobs = ZSTDMT_createJobsTable(&mut nbJobs, cMem);
    (*mtctx).jobIDMask = nbJobs.wrapping_sub(1);
    (*mtctx).bufPool = ZSTDMT_createBufferPool(
        (2 as core::ffi::c_uint)
            .wrapping_mul(nbWorkers)
            .wrapping_add(3),
        cMem,
    );
    (*mtctx).cctxPool = ZSTDMT_createCCtxPool(nbWorkers as core::ffi::c_int, cMem);
    (*mtctx).seqPool = ZSTDMT_createSeqPool(nbWorkers, cMem);
    initError = ZSTDMT_serialState_init(&mut (*mtctx).serial);
    (*mtctx).roundBuff = kNullRoundBuff;
    if ((*mtctx).factory).is_null() as core::ffi::c_int
        | ((*mtctx).jobs).is_null() as core::ffi::c_int
        | ((*mtctx).bufPool).is_null() as core::ffi::c_int
        | ((*mtctx).cctxPool).is_null() as core::ffi::c_int
        | ((*mtctx).seqPool).is_null() as core::ffi::c_int
        | initError
        != 0
    {
        ZSTDMT_freeCCtx(mtctx);
        return core::ptr::null_mut();
    }
    mtctx
}

pub unsafe fn ZSTDMT_createCCtx_advanced(
    nbWorkers: core::ffi::c_uint,
    cMem: ZSTD_customMem,
    pool: *mut ZSTD_threadPool,
) -> *mut ZSTDMT_CCtx {
    ZSTDMT_createCCtx_advanced_internal(nbWorkers, cMem, pool)
}

/// Note: ensure all workers are killed first!
unsafe fn ZSTDMT_releaseAllJobResources(mtctx: *mut ZSTDMT_CCtx) {
    let mut jobID: core::ffi::c_uint = 0;
    jobID = 0;
    while jobID <= (*mtctx).jobIDMask {
        // Copy the mutex/cond out
        let mutex = core::ptr::read(core::ptr::addr_of!(
            (*((*mtctx).jobs).offset(jobID as isize)).job_mutex
        ));
        let cond = core::ptr::read(core::ptr::addr_of!(
            (*((*mtctx).jobs).offset(jobID as isize)).job_cond
        ));

        ZSTDMT_releaseBuffer(
            (*mtctx).bufPool,
            (*((*mtctx).jobs).offset(jobID as isize)).dstBuff,
        );

        // Clear the job description, but keep the mutex/cond
        ptr::write_bytes(
            &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription as *mut u8,
            0,
            size_of::<ZSTDMT_jobDescription>(),
        );
        core::ptr::write(
            core::ptr::addr_of_mut!((*((*mtctx).jobs).offset(jobID as isize)).job_mutex),
            mutex,
        );
        core::ptr::write(
            core::ptr::addr_of_mut!((*((*mtctx).jobs).offset(jobID as isize)).job_cond),
            cond,
        );
        jobID = jobID.wrapping_add(1);
    }

    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0;
    (*mtctx).allJobsCompleted = 1;
}

unsafe fn ZSTDMT_waitForAllJobsCompleted(mtctx: *mut ZSTDMT_CCtx) {
    while (*mtctx).doneJobID < (*mtctx).nextJobID {
        let jobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;
        let mut guard = (*((*mtctx).jobs).offset(jobID as isize))
            .job_mutex
            .lock()
            .unwrap();
        while (*((*mtctx).jobs).offset(jobID as isize)).consumed
            < (*((*mtctx).jobs).offset(jobID as isize)).src.size
        {
            guard = (*((*mtctx).jobs).offset(jobID as isize))
                .job_cond
                .wait(guard)
                .unwrap();
        }
        (*mtctx).doneJobID += 1;
    }
}

pub unsafe fn ZSTDMT_freeCCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t {
    if mtctx.is_null() {
        return 0;
    }
    if !(*mtctx).providedFactory {
        POOL_free((*mtctx).factory); // stop and free worker threads
    }
    ZSTDMT_releaseAllJobResources(mtctx); // release job resources into pools first
    ZSTDMT_freeJobsTable(
        (*mtctx).jobs,
        ((*mtctx).jobIDMask).wrapping_add(1),
        (*mtctx).cMem,
    );
    ZSTDMT_freeBufferPool((*mtctx).bufPool);
    ZSTDMT_freeCCtxPool((*mtctx).cctxPool);
    ZSTDMT_freeSeqPool((*mtctx).seqPool);
    ZSTDMT_serialState_free(&mut (*mtctx).serial);
    ZSTD_freeCDict((*mtctx).cdictLocal);
    if !((*mtctx).roundBuff.buffer).is_null() {
        ZSTD_customFree(
            (*mtctx).roundBuff.buffer as *mut core::ffi::c_void,
            (*mtctx).roundBuff.capacity,
            (*mtctx).cMem,
        );
    }
    ZSTD_customFree(
        mtctx as *mut core::ffi::c_void,
        size_of::<ZSTDMT_CCtx>(),
        (*mtctx).cMem,
    );
    0
}

pub unsafe fn ZSTDMT_sizeof_CCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t {
    if mtctx.is_null() {
        return 0;
    }
    size_of::<ZSTDMT_CCtx>()
        .wrapping_add(POOL_sizeof((*mtctx).factory))
        .wrapping_add(ZSTDMT_sizeof_bufferPool((*mtctx).bufPool))
        .wrapping_add(
            (((*mtctx).jobIDMask).wrapping_add(1) as size_t)
                .wrapping_mul(size_of::<ZSTDMT_jobDescription>()),
        )
        .wrapping_add(ZSTDMT_sizeof_CCtxPool((*mtctx).cctxPool))
        .wrapping_add(ZSTDMT_sizeof_seqPool((*mtctx).seqPool))
        .wrapping_add(ZSTD_sizeof_CDict((*mtctx).cdictLocal))
        .wrapping_add((*mtctx).roundBuff.capacity)
}

/// # Returns
///
/// An error code if resize fails, or 0 on success
unsafe fn ZSTDMT_resize(mtctx: *mut ZSTDMT_CCtx, nbWorkers: core::ffi::c_uint) -> size_t {
    if POOL_resize((*mtctx).factory, nbWorkers as size_t) != 0 {
        return Error::memory_allocation.to_error_code();
    }
    let err_code = ZSTDMT_expandJobsTable(mtctx, nbWorkers);
    if ERR_isError(err_code) {
        return err_code;
    }
    (*mtctx).bufPool = ZSTDMT_expandBufferPool(
        (*mtctx).bufPool,
        (2 as core::ffi::c_uint)
            .wrapping_mul(nbWorkers)
            .wrapping_add(3),
    );
    if ((*mtctx).bufPool).is_null() {
        return Error::memory_allocation.to_error_code();
    }
    (*mtctx).cctxPool = ZSTDMT_expandCCtxPool((*mtctx).cctxPool, nbWorkers as core::ffi::c_int);
    if ((*mtctx).cctxPool).is_null() {
        return Error::memory_allocation.to_error_code();
    }
    (*mtctx).seqPool = ZSTDMT_expandSeqPool((*mtctx).seqPool, nbWorkers);
    if ((*mtctx).seqPool).is_null() {
        return Error::memory_allocation.to_error_code();
    }
    ZSTDMT_CCtxParam_setNbWorkers(&mut (*mtctx).params, nbWorkers);
    0
}

/// Updates a selected set of compression parameters, remaining compatible with currently active frame.
/// New parameters will be applied to next compression job.
pub unsafe fn ZSTDMT_updateCParams_whileCompressing(
    mtctx: *mut ZSTDMT_CCtx,
    cctxParams: &ZSTD_CCtx_params,
) {
    let saved_wlog = (*mtctx).params.cParams.windowLog; // Do not modify windowLog while compressing
    let compressionLevel = cctxParams.compressionLevel;
    (*mtctx).params.compressionLevel = compressionLevel;

    let mut cParams = ZSTD_getCParamsFromCCtxParams(
        cctxParams,
        ZSTD_CONTENTSIZE_UNKNOWN,
        0,
        ZSTD_cpm_noAttachDict,
    );
    cParams.windowLog = saved_wlog;
    (*mtctx).params.cParams = cParams;
}

/// Tells how much data has been consumed (input) and produced (output) for current frame.
/// Able to count progression inside worker threads.
/// Note: mutex will be acquired during statistics collection inside workers.
pub unsafe fn ZSTDMT_getFrameProgression(mtctx: *mut ZSTDMT_CCtx) -> ZSTD_frameProgression {
    let mut fps = ZSTD_frameProgression {
        ingested: 0,
        consumed: 0,
        produced: 0,
        flushed: 0,
        currentJobID: 0,
        nbActiveWorkers: 0,
    };
    fps.ingested =
        ((*mtctx).consumed).wrapping_add((*mtctx).inBuff.filled as core::ffi::c_ulonglong);
    fps.consumed = (*mtctx).consumed;
    fps.flushed = (*mtctx).produced;
    fps.produced = fps.flushed;
    fps.currentJobID = (*mtctx).nextJobID;
    fps.nbActiveWorkers = 0;

    let mut jobNb: core::ffi::c_uint = 0;
    let lastJobNb = ((*mtctx).nextJobID).wrapping_add((*mtctx).jobReady as core::ffi::c_uint);
    jobNb = (*mtctx).doneJobID;
    while jobNb < lastJobNb {
        let wJobID = jobNb & (*mtctx).jobIDMask;
        let jobPtr: *mut ZSTDMT_jobDescription =
            &mut *((*mtctx).jobs).offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
        let _guard = (*jobPtr).job_mutex.lock().unwrap();
        let cResult = (*jobPtr).cSize;
        let produced = if ERR_isError(cResult) { 0 } else { cResult };
        let flushed = if ERR_isError(cResult) {
            0
        } else {
            (*jobPtr).dstFlushed
        };
        fps.ingested = (fps.ingested).wrapping_add((*jobPtr).src.size as core::ffi::c_ulonglong);
        fps.consumed = (fps.consumed).wrapping_add((*jobPtr).consumed as core::ffi::c_ulonglong);
        fps.produced = (fps.produced).wrapping_add(produced as core::ffi::c_ulonglong);
        fps.flushed = (fps.flushed).wrapping_add(flushed as core::ffi::c_ulonglong);
        fps.nbActiveWorkers = (fps.nbActiveWorkers).wrapping_add(
            ((*jobPtr).consumed < (*jobPtr).src.size) as core::ffi::c_int as core::ffi::c_uint,
        );
        jobNb += 1;
    }
    fps
}

pub unsafe fn ZSTDMT_toFlushNow(mtctx: *mut ZSTDMT_CCtx) -> size_t {
    let mut toFlush: size_t = 0;
    let jobID = (*mtctx).doneJobID;
    if jobID == (*mtctx).nextJobID {
        return 0; // no active job => nothing to flush
    }

    // look into oldest non-fully-flushed job
    let wJobID = jobID & (*mtctx).jobIDMask;
    let jobPtr: *mut ZSTDMT_jobDescription =
        &mut *((*mtctx).jobs).offset(wJobID as isize) as *mut ZSTDMT_jobDescription;
    let _guard = (*jobPtr).job_mutex.lock().unwrap();

    let cResult = (*jobPtr).cSize;
    let produced = if ERR_isError(cResult) { 0 } else { cResult };
    let flushed = if ERR_isError(cResult) {
        0
    } else {
        (*jobPtr).dstFlushed
    };
    toFlush = produced.wrapping_sub(flushed);
    // if toFlush==0, nothing is available to flush.
    // However, jobID is expected to still be active:
    // if jobID was already completed and fully flushed,
    // ZSTDMT_flushProduced() should have already moved onto next job.
    // Therefore, some input has not yet been consumed.
    if toFlush == 0 {
        assert!((*jobPtr).consumed < (*jobPtr).src.size);
    }

    toFlush
}

unsafe fn ZSTDMT_computeTargetJobLog(params: *const ZSTD_CCtx_params) -> core::ffi::c_uint {
    let mut jobLog: core::ffi::c_uint = 0;
    if (*params).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        // In Long Range Mode, the windowLog is typically oversized.
        // In which case, it's preferable to determine the jobSize
        // based on cycleLog instead.
        jobLog = (ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy))
            .wrapping_add(3)
            .max(21);
    } else {
        jobLog = ((*params).cParams.windowLog).wrapping_add(2).max(20);
    }
    jobLog.min((if MEM_32bits() { 29 } else { 30 }) as core::ffi::c_uint)
}

fn ZSTDMT_overlapLog_default(strat: ZSTD_strategy) -> core::ffi::c_int {
    match strat as core::ffi::c_uint {
        9 => return 9,
        8 | 7 => return 8,
        6 | 5 => return 7,
        4 | 3 | 2 | 1 | _ => {}
    }
    6
}

fn ZSTDMT_overlapLog(ovlog: core::ffi::c_int, strat: ZSTD_strategy) -> core::ffi::c_int {
    if ovlog == 0 {
        return ZSTDMT_overlapLog_default(strat);
    }
    ovlog
}

unsafe fn ZSTDMT_computeOverlapSize(params: &ZSTD_CCtx_params) -> size_t {
    let overlapRLog = 9 - ZSTDMT_overlapLog(params.overlapLog, params.cParams.strategy);
    let mut ovLog = (if overlapRLog >= 8 {
        0
    } else {
        (params.cParams.windowLog).wrapping_sub(overlapRLog as core::ffi::c_uint)
    }) as core::ffi::c_int;
    if params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        // In Long Range Mode, the windowLog is typically oversized.
        // In which case, it's preferable to determine the jobSize
        // based on chainLog instead.
        // Then, ovLog becomes a fraction of the jobSize, rather than windowSize
        ovLog = (params
            .cParams
            .windowLog
            .min((ZSTDMT_computeTargetJobLog(params)).wrapping_sub(2)))
        .wrapping_sub(overlapRLog as core::ffi::c_uint) as core::ffi::c_int;
    }
    if ovLog == 0 {
        0
    } else {
        (1) << ovLog
    }
}

pub unsafe fn ZSTDMT_initCStream_internal(
    mtctx: *mut ZSTDMT_CCtx,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
    cdict: *const ZSTD_CDict,
    mut params: ZSTD_CCtx_params,
    pledgedSrcSize: core::ffi::c_ulonglong,
) -> size_t {
    // init
    if params.nbWorkers != (*mtctx).params.nbWorkers {
        let err_code = ZSTDMT_resize(mtctx, params.nbWorkers as core::ffi::c_uint);
        if ERR_isError(err_code) {
            return err_code;
        }
    }

    if params.jobSize != 0 && params.jobSize < ZSTDMT_JOBSIZE_MIN as size_t {
        params.jobSize = ZSTDMT_JOBSIZE_MIN as size_t;
    }
    if params.jobSize
        > (if MEM_32bits() {
            512 * (1 << 20)
        } else {
            1024 * (1 << 20)
        }) as size_t
    {
        params.jobSize = (if MEM_32bits() {
            512 * ((1) << 20)
        } else {
            1024 * ((1) << 20)
        }) as size_t;
    }

    if (*mtctx).allJobsCompleted == 0 {
        // previous compression not correctly finished
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        (*mtctx).allJobsCompleted = 1;
    }

    (*mtctx).params = params;
    (*mtctx).frameContentSize = pledgedSrcSize;
    ZSTD_freeCDict((*mtctx).cdictLocal);
    if !dict.is_null() {
        (*mtctx).cdictLocal = ZSTD_createCDict_advanced(
            dict,
            dictSize,
            ZSTD_dlm_byCopy,
            dictContentType, // note: a loadPrefix becomes an internal CDict
            params.cParams,
            (*mtctx).cMem,
        );
        (*mtctx).cdict = (*mtctx).cdictLocal;
        if ((*mtctx).cdictLocal).is_null() {
            return Error::memory_allocation.to_error_code();
        }
    } else {
        (*mtctx).cdictLocal = core::ptr::null_mut();
        (*mtctx).cdict = cdict;
    }

    (*mtctx).targetPrefixSize = ZSTDMT_computeOverlapSize(&params);
    (*mtctx).targetSectionSize = params.jobSize;
    if (*mtctx).targetSectionSize == 0 {
        (*mtctx).targetSectionSize = ((1) << ZSTDMT_computeTargetJobLog(&params)) as size_t;
    }

    if params.rsyncable != 0 {
        // Aim for the targetSectionSize as the average job size.
        let jobSizeKB = ((*mtctx).targetSectionSize >> 10) as u32;
        let rsyncBits = (ZSTD_highbit32(jobSizeKB)).wrapping_add(10);
        // We refuse to create jobs < RSYNC_MIN_BLOCK_SIZE bytes, so make sure our
        // expected job size is at least 4x larger.
        (*mtctx).rsync.hash = 0;
        (*mtctx).rsync.hitMask = (1u64 << rsyncBits).wrapping_sub(1);
        (*mtctx).rsync.primePower = ZSTD_rollingHash_primePower(RSYNC_LENGTH as u32);
    }
    if (*mtctx).targetSectionSize < (*mtctx).targetPrefixSize {
        (*mtctx).targetSectionSize = (*mtctx).targetPrefixSize; // job size must be >= overlap size
    }
    ZSTDMT_setBufferSize(
        (*mtctx).bufPool,
        ZSTD_compressBound((*mtctx).targetSectionSize),
    );

    // If ldm is enabled we need windowSize space.
    let windowSize = (if (*mtctx).params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        (1) << (*mtctx).params.cParams.windowLog
    } else {
        0
    }) as size_t;
    // Two buffers of slack, plus extra space for the overlap.
    // This is the minimum slack that LDM works with. One extra because
    // flush might waste up to targetSectionSize-1 bytes. Another extra
    // for the overlap (if > 0), then one to fill which doesn't overlap
    // with the LDM window.
    let nbSlackBuffers = (2 + ((*mtctx).targetPrefixSize > 0) as core::ffi::c_int) as size_t;
    let slackSize = (*mtctx).targetSectionSize * nbSlackBuffers;
    // Compute the total size, and always have enough slack
    let nbWorkers = ((*mtctx).params.nbWorkers.max(1)) as size_t;
    let sectionsSize = (*mtctx).targetSectionSize * nbWorkers;
    let capacity = (windowSize.max(sectionsSize)).wrapping_add(slackSize);
    if (*mtctx).roundBuff.capacity < capacity {
        if !((*mtctx).roundBuff.buffer).is_null() {
            ZSTD_customFree(
                (*mtctx).roundBuff.buffer as *mut core::ffi::c_void,
                (*mtctx).roundBuff.capacity,
                (*mtctx).cMem,
            );
        }
        (*mtctx).roundBuff.buffer = ZSTD_customMalloc(capacity, (*mtctx).cMem) as *mut u8;
        if ((*mtctx).roundBuff.buffer).is_null() {
            (*mtctx).roundBuff.capacity = 0;
            return Error::memory_allocation.to_error_code();
        }
        (*mtctx).roundBuff.capacity = capacity;
    }

    (*mtctx).roundBuff.pos = 0;
    (*mtctx).inBuff.buffer = g_nullBuffer;
    (*mtctx).inBuff.filled = 0;
    (*mtctx).inBuff.prefix = kNullRange;
    (*mtctx).doneJobID = 0;
    (*mtctx).nextJobID = 0;
    (*mtctx).frameEnded = 0;
    (*mtctx).allJobsCompleted = 0;
    (*mtctx).consumed = 0;
    (*mtctx).produced = 0;

    // update dictionary
    ZSTD_freeCDict((*mtctx).cdictLocal);
    (*mtctx).cdictLocal = core::ptr::null_mut();
    (*mtctx).cdict = core::ptr::null();
    if !dict.is_null() {
        if dictContentType as core::ffi::c_uint
            == ZSTD_dct_rawContent as core::ffi::c_int as core::ffi::c_uint
        {
            (*mtctx).inBuff.prefix.start = dict as *const u8 as *const core::ffi::c_void;
            (*mtctx).inBuff.prefix.size = dictSize;
        } else {
            // note: a loadPrefix becomes an internal CDict
            (*mtctx).cdictLocal = ZSTD_createCDict_advanced(
                dict,
                dictSize,
                ZSTD_dlm_byRef,
                dictContentType,
                params.cParams,
                (*mtctx).cMem,
            );
            (*mtctx).cdict = (*mtctx).cdictLocal;
            if ((*mtctx).cdictLocal).is_null() {
                return Error::memory_allocation.to_error_code();
            }
        }
    } else {
        (*mtctx).cdict = cdict;
    }

    if ZSTDMT_serialState_reset(
        &mut (*mtctx).serial,
        (*mtctx).seqPool,
        params,
        (*mtctx).targetSectionSize,
        dict,
        dictSize,
        dictContentType,
    ) != 0
    {
        return Error::memory_allocation.to_error_code();
    }

    0
}

/// Write a single empty block with an end-of-frame to finish a frame.
/// Job must be created from streaming variant.
/// This function is always successful if expected conditions are fulfilled.
unsafe fn ZSTDMT_writeLastEmptyBlock(job: *mut ZSTDMT_jobDescription) {
    (*job).dstBuff = ZSTDMT_getBuffer((*job).bufPool);
    if ((*job).dstBuff.start).is_null() {
        (*job).cSize = Error::memory_allocation.to_error_code();
        return;
    }
    (*job).src = kNullRange;
    (*job).cSize = ZSTD_writeLastEmptyBlock((*job).dstBuff.start, (*job).dstBuff.capacity);
}

unsafe fn ZSTDMT_createCompressionJob(
    mtctx: *mut ZSTDMT_CCtx,
    srcSize: size_t,
    endOp: ZSTD_EndDirective,
) -> size_t {
    let jobID = (*mtctx).nextJobID & (*mtctx).jobIDMask;
    let endFrame = (endOp as core::ffi::c_uint
        == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint)
        as core::ffi::c_int;

    if (*mtctx).nextJobID > ((*mtctx).doneJobID).wrapping_add((*mtctx).jobIDMask) {
        // will not create new job: table is full
        return 0;
    }

    if (*mtctx).jobReady == 0 {
        let src = (*mtctx).inBuff.buffer.start as *const u8;
        let fresh4 = &mut (*((*mtctx).jobs).offset(jobID as isize)).src.start;
        *fresh4 = src as *const core::ffi::c_void;
        (*((*mtctx).jobs).offset(jobID as isize)).src.size = srcSize;
        (*((*mtctx).jobs).offset(jobID as isize)).prefix = (*mtctx).inBuff.prefix;
        (*((*mtctx).jobs).offset(jobID as isize)).consumed = 0;
        (*((*mtctx).jobs).offset(jobID as isize)).cSize = 0;
        (*((*mtctx).jobs).offset(jobID as isize)).params = (*mtctx).params;
        let fresh5 = &mut (*((*mtctx).jobs).offset(jobID as isize)).cdict;
        *fresh5 = if (*mtctx).nextJobID == 0 {
            (*mtctx).cdict
        } else {
            core::ptr::null()
        };
        (*((*mtctx).jobs).offset(jobID as isize)).fullFrameSize = (*mtctx).frameContentSize;
        (*((*mtctx).jobs).offset(jobID as isize)).dstBuff = g_nullBuffer;
        let fresh6 = &mut (*((*mtctx).jobs).offset(jobID as isize)).cctxPool;
        *fresh6 = (*mtctx).cctxPool;
        let fresh7 = &mut (*((*mtctx).jobs).offset(jobID as isize)).bufPool;
        *fresh7 = (*mtctx).bufPool;
        let fresh8 = &mut (*((*mtctx).jobs).offset(jobID as isize)).seqPool;
        *fresh8 = (*mtctx).seqPool;
        let fresh9 = &mut (*((*mtctx).jobs).offset(jobID as isize)).serial;
        *fresh9 = &mut (*mtctx).serial;
        (*((*mtctx).jobs).offset(jobID as isize)).jobID = (*mtctx).nextJobID;
        (*((*mtctx).jobs).offset(jobID as isize)).firstJob =
            ((*mtctx).nextJobID == 0) as core::ffi::c_int as core::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).lastJob = endFrame as core::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).frameChecksumNeeded =
            ((*mtctx).params.fParams.checksumFlag != 0 && endFrame != 0 && (*mtctx).nextJobID > 0)
                as core::ffi::c_int as core::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).dstFlushed = 0;

        // Update the round buffer pos and clear the input buffer to be reset
        (*mtctx).roundBuff.pos = ((*mtctx).roundBuff.pos).wrapping_add(srcSize);
        (*mtctx).inBuff.buffer = g_nullBuffer;
        (*mtctx).inBuff.filled = 0;

        // Set the prefix for next job
        if endFrame == 0 {
            let newPrefixSize = srcSize.min((*mtctx).targetPrefixSize);
            (*mtctx).inBuff.prefix.start =
                src.add(srcSize).offset(-(newPrefixSize as isize)) as *const core::ffi::c_void;
            (*mtctx).inBuff.prefix.size = newPrefixSize;
        } else {
            // endFrame==1 => no need for another input buffer
            (*mtctx).inBuff.prefix = kNullRange;
            (*mtctx).frameEnded = endFrame as core::ffi::c_uint;
            if (*mtctx).nextJobID == 0 {
                // single job exception: checksum is already calculated directly within worker thread
                (*mtctx).params.fParams.checksumFlag = 0;
            }
        }

        if srcSize == 0 && (*mtctx).nextJobID > 0 {
            // single job must also write frame header
            ZSTDMT_writeLastEmptyBlock(((*mtctx).jobs).offset(jobID as isize));
            (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
            return 0;
        }
    }

    if POOL_tryAdd(
        (*mtctx).factory,
        ZSTDMT_compressionJob,
        &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription
            as *mut core::ffi::c_void,
    ) != 0
    {
        (*mtctx).nextJobID = ((*mtctx).nextJobID).wrapping_add(1);
        (*mtctx).jobReady = 0;
    } else {
        (*mtctx).jobReady = 1;
    }

    0
}

/// Flush whatever data has been produced but not yet flushed in current job.
/// Move to next job if current one is fully flushed.
/// `output`: `pos` will be updated with amount of data flushed.
/// `blockToFlush`: if >0, the function will block and wait if there is no data available to flush.
/// @return: amount of data remaining within internal buffer, 0 if no more, 1 if unknown but > 0, or an error code
unsafe fn ZSTDMT_flushProduced(
    mtctx: *mut ZSTDMT_CCtx,
    output: *mut ZSTD_outBuffer,
    blockToFlush: core::ffi::c_uint,
    end: ZSTD_EndDirective,
) -> size_t {
    let wJobID = (*mtctx).doneJobID & (*mtctx).jobIDMask;

    let mut guard = (*((*mtctx).jobs).offset(wJobID as isize))
        .job_mutex
        .lock()
        .unwrap();

    if blockToFlush != 0 && (*mtctx).doneJobID < (*mtctx).nextJobID {
        while (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed
            == (*((*mtctx).jobs).offset(wJobID as isize)).cSize
        {
            // nothing to flush
            if (*((*mtctx).jobs).offset(wJobID as isize)).consumed
                == (*((*mtctx).jobs).offset(wJobID as isize)).src.size
            {
                break;
            }
            // block when nothing to flush but some to come
            guard = (*((*mtctx).jobs).offset(wJobID as isize))
                .job_cond
                .wait(guard)
                .unwrap();
        }
    }

    // try to flush something
    let mut cSize = (*((*mtctx).jobs).offset(wJobID as isize)).cSize; // shared
    let srcConsumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed; // shared
    let srcSize = (*((*mtctx).jobs).offset(wJobID as isize)).src.size; // read-only, could be done after mutex lock, but no-declaration-after-statement
    drop(guard);
    if ERR_isError(cSize) {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        return cSize;
    }

    // add frame checksum if necessary (can only happen once)
    if srcConsumed == srcSize && (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded != 0
    {
        // job completed -> worker no longer active
        let checksum = ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState) as u32;
        MEM_writeLE32(
            ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start as *mut core::ffi::c_char)
                .add((*((*mtctx).jobs).offset(wJobID as isize)).cSize)
                as *mut core::ffi::c_void,
            checksum,
        );
        cSize = cSize.wrapping_add(4);
        let fresh10 = &mut (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
        *fresh10 = (*fresh10).wrapping_add(4); // can write this shared value, as worker is no longer active
        (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded = 0;
    }

    if cSize > 0 {
        // compression is ongoing or completed
        let toFlush = cSize
            .wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
            .min(((*output).size).wrapping_sub((*output).pos));
        if toFlush > 0 {
            libc::memcpy(
                ((*output).dst as *mut core::ffi::c_char).add((*output).pos)
                    as *mut core::ffi::c_void,
                ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start
                    as *const core::ffi::c_char)
                    .add((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
                    as *const core::ffi::c_void,
                toFlush as libc::size_t,
            );
        }
        (*output).pos = ((*output).pos).wrapping_add(toFlush);
        let fresh11 = &mut (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed;
        *fresh11 = (*fresh11).wrapping_add(toFlush); // can write: this value is only used by mtctx

        if srcConsumed == srcSize && (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed == cSize
        {
            // job is completed and output buffer fully flushed
            ZSTDMT_releaseBuffer(
                (*mtctx).bufPool,
                (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff,
            );
            (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff = g_nullBuffer;
            (*((*mtctx).jobs).offset(wJobID as isize)).cSize = 0; // ensure this job slot is considered "not started" in future check
            (*mtctx).consumed = ((*mtctx).consumed).wrapping_add(srcSize as core::ffi::c_ulonglong);
            (*mtctx).produced = ((*mtctx).produced).wrapping_add(cSize as core::ffi::c_ulonglong);
            (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
        }
    }

    // return value: how many bytes left in buffer ; fake it to 1 when unknown but >0
    if cSize > (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed {
        return cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed);
    }
    if srcSize > srcConsumed {
        return 1; // current job not completely compressed
    }

    if (*mtctx).doneJobID < (*mtctx).nextJobID {
        return 1; // some more jobs ongoing
    }
    if (*mtctx).jobReady != 0 {
        return 1; // one job is ready to push, just not yet in the list
    }
    if (*mtctx).inBuff.filled > 0 {
        return 1; // input is not empty, and still needs to be converted into a job
    }
    (*mtctx).allJobsCompleted = (*mtctx).frameEnded; // all jobs are entirely flushed => if this one is last one, frame is completed
    if end as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint {
        return ((*mtctx).frameEnded == 0) as core::ffi::c_int as size_t; // for ZSTD_e_end, question becomes: is frame completed ?
    }

    0 // internal buffers fully flushed
}

/// Returns the range of data used by the earliest job that is not yet complete.
/// If the data of the first job is broken up into two segments, we cover both sections.
unsafe fn ZSTDMT_getInputDataInUse(mtctx: *mut ZSTDMT_CCtx) -> Range {
    let firstJobID = (*mtctx).doneJobID;
    let lastJobID = (*mtctx).nextJobID;
    let mut jobID: core::ffi::c_uint = 0;

    // no need to check during first round
    let roundBuffCapacity = (*mtctx).roundBuff.capacity;
    let nbJobs1stRoundMin = roundBuffCapacity / (*mtctx).targetSectionSize;
    if (lastJobID as size_t) < nbJobs1stRoundMin {
        return kNullRange;
    }

    jobID = firstJobID;
    while jobID < lastJobID {
        let wJobID = jobID & (*mtctx).jobIDMask;
        let mut consumed: size_t = 0;

        let guard = (*((*mtctx).jobs).offset(wJobID as isize))
            .job_mutex
            .lock()
            .unwrap();
        consumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed;
        drop(guard);

        if consumed < (*((*mtctx).jobs).offset(wJobID as isize)).src.size {
            let mut range = (*((*mtctx).jobs).offset(wJobID as isize)).prefix;
            if range.size == 0 {
                // Empty prefix
                range = (*((*mtctx).jobs).offset(wJobID as isize)).src;
            }
            // Job source in multiple segments not supported yet
            return range;
        }

        jobID = jobID.wrapping_add(1);
    }

    kNullRange
}

/// Returns `true` iff buffer and range overlap.
unsafe fn ZSTDMT_isOverlapped(buffer: Buffer, range: Range) -> bool {
    let bufferStart = buffer.start as *const u8;
    let rangeStart = range.start as *const u8;

    if rangeStart.is_null() || bufferStart.is_null() {
        return false;
    }

    let bufferEnd = bufferStart.add(buffer.capacity);
    let rangeEnd = rangeStart.add(range.size);

    // Empty ranges cannot overlap
    if bufferStart == bufferEnd || rangeStart == rangeEnd {
        return false;
    }

    bufferStart < rangeEnd && rangeStart < bufferEnd
}

unsafe fn ZSTDMT_doesOverlapWindow(buffer: Buffer, window: ZSTD_window_t) -> bool {
    let mut extDict = Range {
        start: core::ptr::null::<core::ffi::c_void>(),
        size: 0,
    };
    let mut prefix = Range {
        start: core::ptr::null::<core::ffi::c_void>(),
        size: 0,
    };

    extDict.start = (window.dictBase).offset(window.lowLimit as isize) as *const core::ffi::c_void;
    extDict.size = (window.dictLimit).wrapping_sub(window.lowLimit) as size_t;

    prefix.start = (window.base).offset(window.dictLimit as isize) as *const core::ffi::c_void;
    prefix.size =
        (window.nextSrc).offset_from((window.base).offset(window.dictLimit as isize)) as size_t;

    ZSTDMT_isOverlapped(buffer, extDict) || ZSTDMT_isOverlapped(buffer, prefix)
}

unsafe fn ZSTDMT_waitForLdmComplete(mtctx: *mut ZSTDMT_CCtx, buffer: Buffer) {
    if (*mtctx).params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        let mut guard = (*mtctx).serial.ldmWindowMutex.lock().unwrap();
        while ZSTDMT_doesOverlapWindow(buffer, (*mtctx).serial.ldmWindow) {
            guard = (*mtctx).serial.ldmWindowCond.wait(guard).unwrap();
        }
    }
}

/// Attempts to set the inBuff to the next section to fill.
/// If any part of the new section is still in use we give up.
/// Returns `true` if the buffer is filled.
unsafe fn ZSTDMT_tryGetInputRange(mtctx: *mut ZSTDMT_CCtx) -> bool {
    let inUse = ZSTDMT_getInputDataInUse(mtctx);
    let spaceLeft = ((*mtctx).roundBuff.capacity).wrapping_sub((*mtctx).roundBuff.pos);
    let spaceNeeded = (*mtctx).targetSectionSize;
    let mut buffer = buffer_s {
        start: core::ptr::null_mut::<core::ffi::c_void>(),
        capacity: 0,
    };

    if spaceLeft < spaceNeeded {
        // ZSTD_invalidateRepCodes() doesn't work for extDict variants.
        // Simply copy the prefix to the beginning in that case.
        let start = (*mtctx).roundBuff.buffer;
        let prefixSize = (*mtctx).inBuff.prefix.size;
        buffer.start = start as *mut core::ffi::c_void;
        buffer.capacity = prefixSize;
        if ZSTDMT_isOverlapped(buffer, inUse) {
            return false;
        }
        ZSTDMT_waitForLdmComplete(mtctx, buffer);
        core::ptr::copy((*mtctx).inBuff.prefix.start.cast::<u8>(), start, prefixSize);
        (*mtctx).inBuff.prefix.start = start as *const core::ffi::c_void;
        (*mtctx).roundBuff.pos = prefixSize;
    }
    buffer.start =
        ((*mtctx).roundBuff.buffer).add((*mtctx).roundBuff.pos) as *mut core::ffi::c_void;
    buffer.capacity = spaceNeeded;

    if ZSTDMT_isOverlapped(buffer, inUse) {
        return false;
    }

    ZSTDMT_waitForLdmComplete(mtctx, buffer);

    (*mtctx).inBuff.buffer = buffer;
    (*mtctx).inBuff.filled = 0;

    true
}

/// Searches through the input for a synchronization point. If one is found, we
/// will instruct the caller to flush, and return the number of bytes to load.
/// Otherwise, we will load as many bytes as possible and instruct the caller
/// to continue as normal.
unsafe fn findSynchronizationPoint(mtctx: *const ZSTDMT_CCtx, input: ZSTD_inBuffer) -> SyncPoint {
    let istart = (input.src as *const u8).add(input.pos);
    let primePower = (*mtctx).rsync.primePower;
    let hitMask = (*mtctx).rsync.hitMask;

    let mut syncPoint = SyncPoint {
        toLoad: 0,
        flush: 0,
    };
    let mut hash: u64 = 0;
    let mut prev = core::ptr::null::<u8>();
    let mut pos: size_t = 0;

    syncPoint.toLoad = (input.size)
        .wrapping_sub(input.pos)
        .min(((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled));
    syncPoint.flush = 0;
    if (*mtctx).params.rsyncable == 0 {
        // Rsync is disabled.
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled)
        .wrapping_add(input.size)
        .wrapping_sub(input.pos)
        < RSYNC_MIN_BLOCK_SIZE as size_t
    {
        // We don't emit synchronization points if it would produce too small blocks.
        // We don't have enough input to find a synchronization point, so don't look.
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad) < RSYNC_LENGTH as size_t {
        // Not enough to compute the hash.
        // We will miss any synchronization points in this RSYNC_LENGTH byte
        // window. However, since it depends only in the internal buffers, if the
        // state is already synchronized, we will remain synchronized.
        // Additionally, the probability that we miss a synchronization point is
        // low: RSYNC_LENGTH / targetSectionSize.
        return syncPoint;
    }

    // Initialize the loop variables.
    if (*mtctx).inBuff.filled < RSYNC_MIN_BLOCK_SIZE as size_t {
        // We don't need to scan the first RSYNC_MIN_BLOCK_SIZE positions
        // because they can't possibly be a sync point. So we can start
        // part way through the input buffer.
        pos = (RSYNC_MIN_BLOCK_SIZE as size_t).wrapping_sub((*mtctx).inBuff.filled);
        if pos >= RSYNC_LENGTH as size_t {
            prev = istart.add(pos).offset(-(RSYNC_LENGTH as isize));
            hash =
                ZSTD_rollingHash_compute(prev as *const core::ffi::c_void, RSYNC_LENGTH as size_t);
        } else {
            prev = ((*mtctx).inBuff.buffer.start as *const u8)
                .add((*mtctx).inBuff.filled)
                .offset(-(RSYNC_LENGTH as isize));
            hash = ZSTD_rollingHash_compute(
                prev.add(pos) as *const core::ffi::c_void,
                (RSYNC_LENGTH as size_t).wrapping_sub(pos),
            );
            hash = ZSTD_rollingHash_append(hash, istart as *const core::ffi::c_void, pos);
        }
    } else {
        // We have enough bytes buffered to initialize the hash,
        // and have processed enough bytes to find a sync point.
        // Start scanning at the beginning of the input.
        pos = 0;
        prev = ((*mtctx).inBuff.buffer.start as *const u8)
            .add((*mtctx).inBuff.filled)
            .offset(-(RSYNC_LENGTH as isize));
        hash = ZSTD_rollingHash_compute(prev as *const core::ffi::c_void, RSYNC_LENGTH as size_t);
        if hash & hitMask == hitMask {
            // We're already at a sync point so don't load any more until
            // we're able to flush this sync point.
            // This likely happened because the job table was full so we
            // couldn't add our job.
            syncPoint.toLoad = 0;
            syncPoint.flush = 1;
            return syncPoint;
        }
    }

    // Starting with the hash of the previous RSYNC_LENGTH bytes, roll
    // through the input. If we hit a synchronization point, then cut the
    // job off, and tell the compressor to flush the job. Otherwise, load
    // all the bytes and continue as normal.
    // If we go too long without a synchronization point (targetSectionSize)
    // then a block will be emitted anyways, but this is okay, since if we
    // are already synchronized we will remain synchronized.
    for pos in pos..syncPoint.toLoad {
        let toRemove = (if pos < RSYNC_LENGTH as size_t {
            *prev.add(pos) as core::ffi::c_int
        } else {
            *istart.add(pos.wrapping_sub(RSYNC_LENGTH as size_t)) as core::ffi::c_int
        }) as u8;
        hash = ZSTD_rollingHash_rotate(hash, toRemove, *istart.add(pos), primePower);
        if hash & hitMask == hitMask {
            syncPoint.toLoad = pos.wrapping_add(1);
            syncPoint.flush = 1;
            break;
        }
    }
    syncPoint
}

pub unsafe fn ZSTDMT_nextInputSizeHint(mtctx: *const ZSTDMT_CCtx) -> size_t {
    let mut hintInSize = ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled);
    if hintInSize == 0 {
        hintInSize = (*mtctx).targetSectionSize;
    }
    hintInSize
}

/// internal use only - exposed to be invoked from zstd_compress.c
/// assumption: output and input are valid (pos <= size)
///
/// # Returns
///
/// The minimum amount of data remaining to flush, 0 if none
pub unsafe fn ZSTDMT_compressStream_generic(
    mtctx: *mut ZSTDMT_CCtx,
    output: *mut ZSTD_outBuffer,
    input: *mut ZSTD_inBuffer,
    mut endOp: ZSTD_EndDirective,
) -> size_t {
    let mut forwardInputProgress = 0;

    if (*mtctx).frameEnded != 0
        && endOp as core::ffi::c_uint == ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
    {
        // current frame being ended. Only flush/end are allowed
        return Error::stage_wrong.to_error_code();
    }

    // fill input buffer
    if (*mtctx).jobReady == 0 && (*input).size > (*input).pos {
        // support NULL input
        if ((*mtctx).inBuff.buffer.start).is_null() {
            if !ZSTDMT_tryGetInputRange(mtctx) {
                // It is only possible for this operation to fail if there are
                // still compression jobs ongoing.
                assert_ne!((*mtctx).doneJobID, (*mtctx).nextJobID);
            }
        }
        if !((*mtctx).inBuff.buffer.start).is_null() {
            let syncPoint = findSynchronizationPoint(mtctx, *input);
            if syncPoint.flush != 0
                && endOp as core::ffi::c_uint
                    == ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
            {
                endOp = ZSTD_e_flush;
            }
            libc::memcpy(
                ((*mtctx).inBuff.buffer.start as *mut core::ffi::c_char).add((*mtctx).inBuff.filled)
                    as *mut core::ffi::c_void,
                ((*input).src as *const core::ffi::c_char).add((*input).pos)
                    as *const core::ffi::c_void,
                syncPoint.toLoad as libc::size_t,
            );
            (*input).pos = ((*input).pos).wrapping_add(syncPoint.toLoad);
            (*mtctx).inBuff.filled = ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad);
            forwardInputProgress = (syncPoint.toLoad > 0) as core::ffi::c_int as core::ffi::c_uint;
        }
    }
    if (*input).pos < (*input).size
        && endOp as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
    {
        // Can't end yet because the input is not fully consumed.
        // We are in one of these cases:
        // - mtctx->inBuff is NULL & empty: we couldn't get an input buffer so don't create a new job.
        // - We filled the input buffer: flush this job but don't end the frame.
        // - We hit a synchronization point: flush this job but don't end the frame.
        endOp = ZSTD_e_flush;
    }

    if (*mtctx).jobReady != 0
        || (*mtctx).inBuff.filled >= (*mtctx).targetSectionSize  // filled enough: let's compress
        || endOp as core::ffi::c_uint != ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
            && (*mtctx).inBuff.filled > 0  // something to flush: let's go
        || endOp as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint // must finish the frame with a zero-size block
            && (*mtctx).frameEnded == 0
    {
        let jobSize = (*mtctx).inBuff.filled;
        let err_code = ZSTDMT_createCompressionJob(mtctx, jobSize, endOp);
        if ERR_isError(err_code) {
            return err_code;
        }
    }

    // check for potential compressed data ready to be flushed
    let remainingToFlush = ZSTDMT_flushProduced(
        mtctx,
        output,
        (forwardInputProgress == 0) as core::ffi::c_int as core::ffi::c_uint, // block if there was no forward input progress
        endOp,
    );
    if (*input).pos < (*input).size {
        return remainingToFlush.max(1); // input not consumed: do not end flush yet
    }
    remainingToFlush
}
