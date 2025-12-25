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
    ZSTD_sizeof_CCtx, ZSTD_sizeof_CDict, ZSTD_window_hasExtDict, ZSTD_window_t,
    ZSTD_writeLastEmptyBlock,
};
use crate::lib::compress::zstd_ldm::{
    ldmEntry_t, ldmParams_t, ldmState_t, ZSTD_ldm_adjustParameters, ZSTD_ldm_fillHashTable,
    ZSTD_ldm_generateSequences, ZSTD_ldm_getMaxNbSeq,
};
use crate::lib::zstd::*;
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
    mutex: Mutex<()>,
    cond: Condvar,
    params: ZSTD_CCtx_params,
    ldmState: ldmState_t,
    xxhState: XXH64_state_t,
    nextJobID: core::ffi::c_uint,
    ldmWindowMutex: Mutex<()>,
    ldmWindowCond: Condvar,
    ldmWindow: ZSTD_window_t,
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
    consumed: size_t,
    cSize: size_t,
    job_mutex: Mutex<()>,
    job_cond: Condvar,
    cctxPool: *mut ZSTDMT_CCtxPool,
    bufPool: *mut ZSTDMT_bufferPool,
    seqPool: *mut ZSTDMT_seqPool,
    serial: *mut SerialState,
    dstBuff: Buffer,
    prefix: Range,
    src: Range,
    jobID: core::ffi::c_uint,
    firstJob: core::ffi::c_uint,
    lastJob: core::ffi::c_uint,
    params: ZSTD_CCtx_params,
    cdict: *const ZSTD_CDict,
    fullFrameSize: core::ffi::c_ulonglong,
    dstFlushed: size_t,
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
const HASH_READ_SIZE: core::ffi::c_int = 8;
static mut kNullRawSeqStore: RawSeqStore_t = RawSeqStore_t {
    seq: core::ptr::null_mut(),
    pos: 0,
    posInSequence: 0,
    size: 0,
    capacity: 0,
};
const ZSTD_WINDOW_START_INDEX: core::ffi::c_int = 2;
static prime8bytes: u64 = 0xcf1bbcdcb7a56463 as core::ffi::c_ulonglong;
unsafe fn ZSTD_ipow(mut base: u64, mut exponent: u64) -> u64 {
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
unsafe fn ZSTD_rollingHash_append(
    mut hash: u64,
    buf: *const core::ffi::c_void,
    size: size_t,
) -> u64 {
    let istart = buf as *const u8;
    let mut pos: size_t = 0;
    pos = 0;
    while pos < size {
        hash *= prime8bytes;
        hash = hash.wrapping_add(
            (core::ffi::c_int::from(*istart.add(pos)) + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64,
        );
        pos = pos.wrapping_add(1);
    }
    hash
}
#[inline]
unsafe fn ZSTD_rollingHash_compute(buf: *const core::ffi::c_void, size: size_t) -> u64 {
    ZSTD_rollingHash_append(0, buf, size)
}
#[inline]
unsafe fn ZSTD_rollingHash_primePower(length: u32) -> u64 {
    ZSTD_ipow(prime8bytes, u64::from(length.wrapping_sub(1)))
}
#[inline]
unsafe fn ZSTD_rollingHash_rotate(mut hash: u64, toRemove: u8, toAdd: u8, primePower: u64) -> u64 {
    hash = hash.wrapping_sub(
        (core::ffi::c_int::from(toRemove) + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64 * primePower,
    );
    hash *= prime8bytes;
    hash = hash.wrapping_add((core::ffi::c_int::from(toAdd) + ZSTD_ROLL_HASH_CHAR_OFFSET) as u64);
    hash
}
#[inline]
unsafe fn ZSTD_window_clear(window: *mut ZSTD_window_t) {
    let endT = ((*window).nextSrc).offset_from((*window).base) as size_t;
    let end = endT as u32;
    (*window).lowLimit = end;
    (*window).dictLimit = end;
}
#[inline]
unsafe fn ZSTD_window_init(window: *mut ZSTD_window_t) {
    ptr::write_bytes(
        window as *mut u8,
        0,
        ::core::mem::size_of::<ZSTD_window_t>(),
    );
    (*window).base = b" \0" as *const u8 as *const core::ffi::c_char as *const u8;
    (*window).dictBase = b" \0" as *const u8 as *const core::ffi::c_char as *const u8;
    (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32;
    (*window).nextSrc = ((*window).base).offset(ZSTD_WINDOW_START_INDEX as isize);
    (*window).nbOverflowCorrections = 0;
}
#[inline]
unsafe fn ZSTD_window_update(
    window: *mut ZSTD_window_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    forceNonContiguous: core::ffi::c_int,
) -> u32 {
    let ip = src as *const u8;
    let mut contiguous = 1;
    if srcSize == 0 {
        return contiguous;
    }
    if src != (*window).nextSrc as *const core::ffi::c_void || forceNonContiguous != 0 {
        let distanceFromBase = ((*window).nextSrc).offset_from((*window).base) as size_t;
        (*window).lowLimit = (*window).dictLimit;
        (*window).dictLimit = distanceFromBase as u32;
        (*window).dictBase = (*window).base;
        (*window).base = ip.offset(-(distanceFromBase as isize));
        if ((*window).dictLimit).wrapping_sub((*window).lowLimit) < HASH_READ_SIZE as u32 {
            (*window).lowLimit = (*window).dictLimit;
        }
        contiguous = 0;
    }
    (*window).nextSrc = ip.add(srcSize);
    if core::ffi::c_int::from(
        ip.add(srcSize) > ((*window).dictBase).offset((*window).lowLimit as isize),
    ) & core::ffi::c_int::from(ip < ((*window).dictBase).offset((*window).dictLimit as isize))
        != 0
    {
        let highInputIdx = ip.add(srcSize).offset_from((*window).dictBase) as size_t;
        let lowLimitMax = if highInputIdx > (*window).dictLimit as size_t {
            (*window).dictLimit
        } else {
            highInputIdx as u32
        };
        (*window).lowLimit = lowLimitMax;
    }
    contiguous
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
        let mut u: core::ffi::c_uint = 0;
        u = 0;
        while u < (*bufPool).totalBuffers {
            ZSTD_customFree(
                (*((*bufPool).buffers).offset(u as isize)).start,
                (*((*bufPool).buffers).offset(u as isize)).capacity,
                (*bufPool).cMem,
            );
            u = u.wrapping_add(1);
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
    let bufPool = ZSTD_customCalloc(::core::mem::size_of::<ZSTDMT_bufferPool>(), cMem)
        as *mut ZSTDMT_bufferPool;
    if bufPool.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::write(
        core::ptr::addr_of_mut!((*bufPool).poolMutex),
        Mutex::new(()),
    );
    (*bufPool).buffers = ZSTD_customCalloc(
        (maxNbBuffers as usize).wrapping_mul(::core::mem::size_of::<Buffer>()),
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
unsafe fn ZSTDMT_sizeof_bufferPool(bufPool: *mut ZSTDMT_bufferPool) -> size_t {
    let poolSize = ::core::mem::size_of::<ZSTDMT_bufferPool>();
    let arraySize =
        ((*bufPool).totalBuffers as size_t).wrapping_mul(::core::mem::size_of::<Buffer>());
    let mut u: core::ffi::c_uint = 0;
    let mut totalBufferSize = 0 as size_t;
    let _guard = (*bufPool).poolMutex.lock().unwrap();
    u = 0;
    while u < (*bufPool).totalBuffers {
        totalBufferSize =
            totalBufferSize.wrapping_add((*((*bufPool).buffers).offset(u as isize)).capacity);
        u = u.wrapping_add(1);
    }
    poolSize
        .wrapping_add(arraySize)
        .wrapping_add(totalBufferSize)
}
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
        return srcBufPool;
    }
    let cMem = (*srcBufPool).cMem;
    let bSize = (*srcBufPool).bufferSize;
    let mut newBufPool = core::ptr::null_mut::<ZSTDMT_bufferPool>();
    ZSTDMT_freeBufferPool(srcBufPool);
    newBufPool = ZSTDMT_createBufferPool(maxNbBuffers, cMem);
    if newBufPool.is_null() {
        return newBufPool;
    }
    ZSTDMT_setBufferSize(newBufPool, bSize);
    newBufPool
}
unsafe fn ZSTDMT_getBuffer(bufPool: *mut ZSTDMT_bufferPool) -> Buffer {
    let bSize = (*bufPool).bufferSize;
    let guard = (*bufPool).poolMutex.lock().unwrap();
    if (*bufPool).nbBuffers != 0 {
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_sub(1);
        let buf = *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize);
        let availBufferSize = buf.capacity;
        *((*bufPool).buffers).offset((*bufPool).nbBuffers as isize) = g_nullBuffer;
        if core::ffi::c_int::from(availBufferSize >= bSize)
            & core::ffi::c_int::from(availBufferSize >> 3 <= bSize)
            != 0
        {
            return buf;
        }
        ZSTD_customFree(buf.start, buf.capacity, (*bufPool).cMem);
    }
    drop(guard);
    let mut buffer = buffer_s {
        start: core::ptr::null_mut::<core::ffi::c_void>(),
        capacity: 0,
    };
    let start = ZSTD_customMalloc(bSize, (*bufPool).cMem);
    buffer.start = start;
    buffer.capacity = if start.is_null() { 0 } else { bSize };
    start.is_null();
    buffer
}
unsafe fn ZSTDMT_releaseBuffer(bufPool: *mut ZSTDMT_bufferPool, buf: Buffer) {
    if (buf.start).is_null() {
        return;
    }
    let guard = (*bufPool).poolMutex.lock().unwrap();
    if (*bufPool).nbBuffers < (*bufPool).totalBuffers {
        let fresh0 = (*bufPool).nbBuffers;
        (*bufPool).nbBuffers = ((*bufPool).nbBuffers).wrapping_add(1);
        *((*bufPool).buffers).offset(fresh0 as isize) = buf;
        return;
    }
    drop(guard);
    ZSTD_customFree(buf.start, buf.capacity, (*bufPool).cMem);
}
unsafe fn ZSTDMT_sizeof_seqPool(seqPool: *mut ZSTDMT_seqPool) -> size_t {
    ZSTDMT_sizeof_bufferPool(seqPool)
}
unsafe fn bufferToSeq(buffer: Buffer) -> RawSeqStore_t {
    let mut seq = kNullRawSeqStore;
    seq.seq = buffer.start as *mut rawSeq;
    seq.capacity = (buffer.capacity).wrapping_div(::core::mem::size_of::<rawSeq>());
    seq
}
unsafe fn seqToBuffer(seq: RawSeqStore_t) -> Buffer {
    let mut buffer = buffer_s {
        start: core::ptr::null_mut::<core::ffi::c_void>(),
        capacity: 0,
    };
    buffer.start = seq.seq as *mut core::ffi::c_void;
    buffer.capacity = (seq.capacity).wrapping_mul(::core::mem::size_of::<rawSeq>());
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
    ZSTDMT_setBufferSize(
        seqPool,
        nbSeq.wrapping_mul(::core::mem::size_of::<rawSeq>()),
    );
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
unsafe fn ZSTDMT_freeCCtxPool(pool: *mut ZSTDMT_CCtxPool) {
    if pool.is_null() {
        return;
    }
    core::ptr::drop_in_place(core::ptr::addr_of_mut!((*pool).poolMutex));
    if !((*pool).cctxs).is_null() {
        let mut cid: core::ffi::c_int = 0;
        cid = 0;
        while cid < (*pool).totalCCtx {
            ZSTD_freeCCtx(*((*pool).cctxs).offset(cid as isize));
            cid += 1;
        }
        ZSTD_customFree(
            (*pool).cctxs as *mut core::ffi::c_void,
            ((*pool).totalCCtx as usize).wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>()),
            (*pool).cMem,
        );
    }
    ZSTD_customFree(
        pool as *mut core::ffi::c_void,
        size_of::<ZSTDMT_CCtxPool>(),
        (*pool).cMem,
    );
}
unsafe fn ZSTDMT_createCCtxPool(
    nbWorkers: core::ffi::c_int,
    cMem: ZSTD_customMem,
) -> *mut ZSTDMT_CCtxPool {
    let cctxPool =
        ZSTD_customCalloc(::core::mem::size_of::<ZSTDMT_CCtxPool>(), cMem) as *mut ZSTDMT_CCtxPool;
    if cctxPool.is_null() {
        return core::ptr::null_mut();
    }
    core::ptr::write(
        core::ptr::addr_of_mut!((*cctxPool).poolMutex),
        Mutex::new(()),
    );
    (*cctxPool).totalCCtx = nbWorkers;
    (*cctxPool).cctxs = ZSTD_customCalloc(
        (nbWorkers as usize).wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>()),
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
    (*cctxPool).availCCtx = 1;
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
        return srcPool;
    }
    let cMem = (*srcPool).cMem;
    ZSTDMT_freeCCtxPool(srcPool);
    ZSTDMT_createCCtxPool(nbWorkers, cMem)
}
unsafe fn ZSTDMT_sizeof_CCtxPool(cctxPool: *mut ZSTDMT_CCtxPool) -> size_t {
    let _guard = (*cctxPool).poolMutex.lock().unwrap();
    let nbWorkers = (*cctxPool).totalCCtx as core::ffi::c_uint;
    let poolSize = ::core::mem::size_of::<ZSTDMT_CCtxPool>();
    let arraySize =
        ((*cctxPool).totalCCtx as usize).wrapping_mul(::core::mem::size_of::<*mut ZSTD_CCtx>());
    let mut totalCCtxSize = 0 as size_t;
    let mut u: core::ffi::c_uint = 0;
    u = 0;
    while u < nbWorkers {
        totalCCtxSize =
            totalCCtxSize.wrapping_add(ZSTD_sizeof_CCtx(*((*cctxPool).cctxs).offset(u as isize)));
        u = u.wrapping_add(1);
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
    ZSTD_createCCtx_advanced((*cctxPool).cMem)
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
        ZSTD_freeCCtx(cctx);
    }
}
unsafe fn ZSTDMT_serialState_reset(
    serialState: *mut SerialState,
    seqPool: *mut ZSTDMT_seqPool,
    mut params: ZSTD_CCtx_params,
    jobSize: size_t,
    dict: *const core::ffi::c_void,
    dictSize: size_t,
    dictContentType: ZSTD_dictContentType_e,
) -> core::ffi::c_int {
    if params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        ZSTD_ldm_adjustParameters(&mut params.ldmParams, &params.cParams);
    } else {
        ptr::write_bytes(
            &mut params.ldmParams as *mut ldmParams_t as *mut u8,
            0,
            ::core::mem::size_of::<ldmParams_t>(),
        );
    }
    (*serialState).nextJobID = 0;
    if params.fParams.checksumFlag != 0 {
        ZSTD_XXH64_reset(&mut (*serialState).xxhState, 0);
    }
    if params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        let cMem = params.customMem;
        let hashLog = params.ldmParams.hashLog;
        let hashSize =
            ((1 as size_t) << hashLog).wrapping_mul(::core::mem::size_of::<ldmEntry_t>());
        let bucketLog = (params.ldmParams.hashLog).wrapping_sub(params.ldmParams.bucketSizeLog);
        let prevBucketLog = ((*serialState).params.ldmParams.hashLog)
            .wrapping_sub((*serialState).params.ldmParams.bucketSizeLog);
        let numBuckets = (1) << bucketLog;
        ZSTDMT_setNbSeq(seqPool, ZSTD_ldm_getMaxNbSeq(params.ldmParams, jobSize));
        ZSTD_window_init(&mut (*serialState).ldmState.window);
        if ((*serialState).ldmState.hashTable).is_null()
            || (*serialState).params.ldmParams.hashLog < hashLog
        {
            ZSTD_customFree(
                (*serialState).ldmState.hashTable as *mut core::ffi::c_void,
                hashSize,
                cMem,
            );
            (*serialState).ldmState.hashTable =
                ZSTD_customMalloc(hashSize, cMem) as *mut ldmEntry_t;
        }
        if ((*serialState).ldmState.bucketOffsets).is_null() || prevBucketLog < bucketLog {
            ZSTD_customFree(
                (*serialState).ldmState.bucketOffsets as *mut core::ffi::c_void,
                1 << prevBucketLog,
                cMem,
            );
            (*serialState).ldmState.bucketOffsets = ZSTD_customMalloc(numBuckets, cMem) as *mut u8;
        }
        if ((*serialState).ldmState.hashTable).is_null()
            || ((*serialState).ldmState.bucketOffsets).is_null()
        {
            return 1;
        }
        ptr::write_bytes((*serialState).ldmState.hashTable as *mut u8, 0, hashSize);
        ptr::write_bytes((*serialState).ldmState.bucketOffsets, 0, numBuckets);
        (*serialState).ldmState.loadedDictEnd = 0;
        if dictSize > 0
            && dictContentType as core::ffi::c_uint
                == ZSTD_dct_rawContent as core::ffi::c_int as core::ffi::c_uint
        {
            let dictEnd = (dict as *const u8).add(dictSize);
            ZSTD_window_update(&mut (*serialState).ldmState.window, dict, dictSize, 0);
            ZSTD_ldm_fillHashTable(
                &mut (*serialState).ldmState,
                dict as *const u8,
                dictEnd,
                &params.ldmParams,
            );
            (*serialState).ldmState.loadedDictEnd = if params.forceWindow != 0 {
                0
            } else {
                dictEnd.offset_from((*serialState).ldmState.window.base) as core::ffi::c_long as u32
            };
        }
        (*serialState).ldmWindow = (*serialState).ldmState.window;
    }
    (*serialState).params = params;
    (*serialState).params.jobSize = jobSize as u32 as size_t;
    0
}
unsafe fn ZSTDMT_serialState_init(serialState: *mut SerialState) -> core::ffi::c_int {
    ptr::write_bytes(
        serialState as *mut u8,
        0,
        ::core::mem::size_of::<SerialState>(),
    );
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
    let hashSize = ((1 as size_t) << hashLog).wrapping_mul(::core::mem::size_of::<ldmEntry_t>());
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
    seqStore: *mut RawSeqStore_t,
    src: Range,
    jobID: core::ffi::c_uint,
) {
    let mut guard = (*serialState).mutex.lock().unwrap();
    #[expect(clippy::while_immutable_condition)]
    while (*serialState).nextJobID < jobID {
        guard = (*serialState).cond.wait(guard).unwrap();
    }
    if (*serialState).nextJobID == jobID {
        if (*serialState).params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
            ZSTD_window_update(&mut (*serialState).ldmState.window, src.start, src.size, 0);
            let error = ZSTD_ldm_generateSequences(
                &mut (*serialState).ldmState,
                seqStore,
                &(*serialState).params.ldmParams,
                src.start,
                src.size,
            );
            // We provide a large enough buffer to never fail.
            assert!(!ERR_isError(error));
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
    (*serialState).nextJobID = ((*serialState).nextJobID).wrapping_add(1);
    (*serialState).cond.notify_all();
}
unsafe fn ZSTDMT_serialState_applySequences(
    _serialState: *const SerialState,
    jobCCtx: *mut ZSTD_CCtx,
    seqStore: *const RawSeqStore_t,
) {
    if (*seqStore).size > 0 {
        ZSTD_referenceExternalSequences(jobCCtx, (*seqStore).seq, (*seqStore).size);
    }
}
unsafe fn ZSTDMT_serialState_ensureFinished(
    serialState: *mut SerialState,
    jobID: core::ffi::c_uint,
    _cSize: size_t,
) {
    let _guard = (*serialState).mutex.lock().unwrap();
    if (*serialState).nextJobID <= jobID {
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
unsafe fn ZSTDMT_compressionJob(jobDescription: *mut core::ffi::c_void) {
    let mut current_block: u64;
    let job = jobDescription as *mut ZSTDMT_jobDescription;
    let mut jobParams = (*job).params;
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
            dstBuff = ZSTDMT_getBuffer((*job).bufPool);
            if (dstBuff.start).is_null() {
                let guard = (*job).job_mutex.lock().unwrap();
                (*job).cSize = Error::memory_allocation.to_error_code();
                drop(guard);
                current_block = 17100290475540901977;
            } else {
                (*job).dstBuff = dstBuff;
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
                    if (*job).jobID != 0 {
                        jobParams.fParams.checksumFlag = 0;
                    }
                    jobParams.ldmParams.enableLdm = ZSTD_ParamSwitch_e::ZSTD_ps_disable;
                    jobParams.nbWorkers = 0;
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
                            core::ffi::c_int::from((*job).firstJob == 0),
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
                            ZSTDMT_serialState_applySequences((*job).serial, cctx, &rawSeqStore);
                            if (*job).firstJob == 0 {
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
                                            let guard = (*job).job_mutex.lock().unwrap();
                                            (*job).cSize = ((*job).cSize).wrapping_add(cSize);
                                            (*job).consumed = chunkSize * chunkNb as size_t;
                                            (*job).job_cond.notify_one();
                                            drop(guard);
                                            chunkNb += 1;
                                        }
                                    }
                                    match current_block {
                                        17100290475540901977 => {}
                                        _ => {
                                            if core::ffi::c_int::from(nbChunks > 0)
                                                as core::ffi::c_uint
                                                | (*job).lastJob
                                                != 0
                                            {
                                                let lastBlockSize1 =
                                                    (*job).src.size & chunkSize.wrapping_sub(1);
                                                let lastBlockSize =
                                                    if core::ffi::c_int::from(lastBlockSize1 == 0)
                                                        & core::ffi::c_int::from(
                                                            (*job).src.size >= chunkSize,
                                                        )
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
                                                        assert!(
                                                            ZSTD_window_hasExtDict(
                                                                (*cctx)
                                                                    .blockState
                                                                    .matchState
                                                                    .window
                                                            ) == 0
                                                        );
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
    ZSTDMT_releaseSeq((*job).seqPool, rawSeqStore);
    ZSTDMT_releaseCCtx((*job).cctxPool, cctx);
    let _guard = (*job).job_mutex.lock().unwrap();
    if ERR_isError((*job).cSize) {
        assert_eq!(lastCBlockSize, 0);
    }
    (*job).cSize = ((*job).cSize).wrapping_add(lastCBlockSize);
    (*job).consumed = (*job).src.size;
    (*job).job_cond.notify_one();
}
static mut kNullRoundBuff: RoundBuff_t = RoundBuff_t {
    buffer: core::ptr::null_mut(),
    capacity: 0,
    pos: 0,
};
const RSYNC_LENGTH: core::ffi::c_int = 32;
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
        (nbJobs as usize).wrapping_mul(::core::mem::size_of::<ZSTDMT_jobDescription>()),
        cMem,
    );
}
unsafe fn ZSTDMT_createJobsTable(
    nbJobsPtr: *mut u32,
    cMem: ZSTD_customMem,
) -> *mut ZSTDMT_jobDescription {
    let nbJobsLog2 = (ZSTD_highbit32(*nbJobsPtr)).wrapping_add(1);
    let nbJobs = ((1) << nbJobsLog2) as u32;
    let mut jobNb: u32 = 0;
    let jobTable = ZSTD_customCalloc(
        (nbJobs as usize).wrapping_mul(::core::mem::size_of::<ZSTDMT_jobDescription>()),
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
unsafe fn ZSTDMT_CCtxParam_setNbWorkers(
    params: *mut ZSTD_CCtx_params,
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
    nbWorkers = if nbWorkers
        < (if ::core::mem::size_of::<*mut core::ffi::c_void>() as core::ffi::c_ulong == 4 {
            64
        } else {
            256
        }) as core::ffi::c_uint
    {
        nbWorkers
    } else {
        (if ::core::mem::size_of::<*mut core::ffi::c_void>() as core::ffi::c_ulong == 4 {
            64
        } else {
            256
        }) as core::ffi::c_uint
    };
    let mtctx = ZSTD_customCalloc(::core::mem::size_of::<ZSTDMT_CCtx>(), cMem) as *mut ZSTDMT_CCtx;
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
    if core::ffi::c_int::from(((*mtctx).factory).is_null())
        | core::ffi::c_int::from(((*mtctx).jobs).is_null())
        | core::ffi::c_int::from(((*mtctx).bufPool).is_null())
        | core::ffi::c_int::from(((*mtctx).cctxPool).is_null())
        | core::ffi::c_int::from(((*mtctx).seqPool).is_null())
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
unsafe fn ZSTDMT_releaseAllJobResources(mtctx: *mut ZSTDMT_CCtx) {
    let mut jobID: core::ffi::c_uint = 0;
    jobID = 0;
    while jobID <= (*mtctx).jobIDMask {
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
        ptr::write_bytes(
            &mut *((*mtctx).jobs).offset(jobID as isize) as *mut ZSTDMT_jobDescription as *mut u8,
            0,
            ::core::mem::size_of::<ZSTDMT_jobDescription>(),
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
        POOL_free((*mtctx).factory);
    }
    ZSTDMT_releaseAllJobResources(mtctx);
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
        ::core::mem::size_of::<ZSTDMT_CCtx>(),
        (*mtctx).cMem,
    );
    0
}
pub unsafe fn ZSTDMT_sizeof_CCtx(mtctx: *mut ZSTDMT_CCtx) -> size_t {
    if mtctx.is_null() {
        return 0;
    }
    (::core::mem::size_of::<ZSTDMT_CCtx>())
        .wrapping_add(POOL_sizeof((*mtctx).factory))
        .wrapping_add(ZSTDMT_sizeof_bufferPool((*mtctx).bufPool))
        .wrapping_add(
            (((*mtctx).jobIDMask).wrapping_add(1) as size_t)
                .wrapping_mul(::core::mem::size_of::<ZSTDMT_jobDescription>()),
        )
        .wrapping_add(ZSTDMT_sizeof_CCtxPool((*mtctx).cctxPool))
        .wrapping_add(ZSTDMT_sizeof_seqPool((*mtctx).seqPool))
        .wrapping_add(ZSTD_sizeof_CDict((*mtctx).cdictLocal))
        .wrapping_add((*mtctx).roundBuff.capacity)
}
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
pub unsafe fn ZSTDMT_updateCParams_whileCompressing(
    mtctx: *mut ZSTDMT_CCtx,
    cctxParams: *const ZSTD_CCtx_params,
) {
    let saved_wlog = (*mtctx).params.cParams.windowLog;
    let compressionLevel = (*cctxParams).compressionLevel;
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
        fps.nbActiveWorkers = (fps.nbActiveWorkers).wrapping_add(core::ffi::c_int::from(
            (*jobPtr).consumed < (*jobPtr).src.size,
        ) as core::ffi::c_uint);
        jobNb += 1;
    }
    fps
}
pub unsafe fn ZSTDMT_toFlushNow(mtctx: *mut ZSTDMT_CCtx) -> size_t {
    let mut toFlush: size_t = 0;
    let jobID = (*mtctx).doneJobID;
    if jobID == (*mtctx).nextJobID {
        return 0;
    }
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
    if toFlush == 0 {
        assert!((*jobPtr).consumed < (*jobPtr).src.size);
    }
    toFlush
}
unsafe fn ZSTDMT_computeTargetJobLog(params: *const ZSTD_CCtx_params) -> core::ffi::c_uint {
    let mut jobLog: core::ffi::c_uint = 0;
    if (*params).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        jobLog = if 21
            > (ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy))
                .wrapping_add(3)
        {
            21
        } else {
            (ZSTD_cycleLog((*params).cParams.chainLog, (*params).cParams.strategy)).wrapping_add(3)
        };
    } else {
        jobLog = if 20 > ((*params).cParams.windowLog).wrapping_add(2) {
            20
        } else {
            ((*params).cParams.windowLog).wrapping_add(2)
        };
    }
    if jobLog < (if MEM_32bits() { 29 } else { 30 }) as core::ffi::c_uint {
        jobLog
    } else {
        (if MEM_32bits() { 29 } else { 30 }) as core::ffi::c_uint
    }
}
unsafe fn ZSTDMT_overlapLog_default(strat: ZSTD_strategy) -> core::ffi::c_int {
    match strat as core::ffi::c_uint {
        9 => return 9,
        8 | 7 => return 8,
        6 | 5 => return 7,
        4 | 3 | 2 | 1 | _ => {}
    }
    6
}
unsafe fn ZSTDMT_overlapLog(ovlog: core::ffi::c_int, strat: ZSTD_strategy) -> core::ffi::c_int {
    if ovlog == 0 {
        return ZSTDMT_overlapLog_default(strat);
    }
    ovlog
}
unsafe fn ZSTDMT_computeOverlapSize(params: *const ZSTD_CCtx_params) -> size_t {
    let overlapRLog = 9 - ZSTDMT_overlapLog((*params).overlapLog, (*params).cParams.strategy);
    let mut ovLog = (if overlapRLog >= 8 {
        0
    } else {
        ((*params).cParams.windowLog).wrapping_sub(overlapRLog as core::ffi::c_uint)
    }) as core::ffi::c_int;
    if (*params).ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        ovLog = (if (*params).cParams.windowLog
            < (ZSTDMT_computeTargetJobLog(params)).wrapping_sub(2)
        {
            (*params).cParams.windowLog
        } else {
            (ZSTDMT_computeTargetJobLog(params)).wrapping_sub(2)
        })
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
            dictContentType,
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
        let jobSizeKB = ((*mtctx).targetSectionSize >> 10) as u32;
        let rsyncBits = (ZSTD_highbit32(jobSizeKB)).wrapping_add(10);
        (*mtctx).rsync.hash = 0;
        (*mtctx).rsync.hitMask =
            ((1 as core::ffi::c_ulonglong) << rsyncBits).wrapping_sub(1) as u64;
        (*mtctx).rsync.primePower = ZSTD_rollingHash_primePower(RSYNC_LENGTH as u32);
    }
    if (*mtctx).targetSectionSize < (*mtctx).targetPrefixSize {
        (*mtctx).targetSectionSize = (*mtctx).targetPrefixSize;
    }
    ZSTDMT_setBufferSize(
        (*mtctx).bufPool,
        ZSTD_compressBound((*mtctx).targetSectionSize),
    );
    let windowSize = (if (*mtctx).params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        (1) << (*mtctx).params.cParams.windowLog
    } else {
        0
    }) as size_t;
    let nbSlackBuffers = (2 + core::ffi::c_int::from((*mtctx).targetPrefixSize > 0)) as size_t;
    let slackSize = (*mtctx).targetSectionSize * nbSlackBuffers;
    let nbWorkers = (if (*mtctx).params.nbWorkers > 1 {
        (*mtctx).params.nbWorkers
    } else {
        1
    }) as size_t;
    let sectionsSize = (*mtctx).targetSectionSize * nbWorkers;
    let capacity = (if windowSize > sectionsSize {
        windowSize
    } else {
        sectionsSize
    })
    .wrapping_add(slackSize);
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
    let endFrame = core::ffi::c_int::from(
        endOp as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint,
    );
    if (*mtctx).nextJobID > ((*mtctx).doneJobID).wrapping_add((*mtctx).jobIDMask) {
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
            core::ffi::c_int::from((*mtctx).nextJobID == 0) as core::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).lastJob = endFrame as core::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).frameChecksumNeeded = core::ffi::c_int::from(
            (*mtctx).params.fParams.checksumFlag != 0 && endFrame != 0 && (*mtctx).nextJobID > 0,
        )
            as core::ffi::c_uint;
        (*((*mtctx).jobs).offset(jobID as isize)).dstFlushed = 0;
        (*mtctx).roundBuff.pos = ((*mtctx).roundBuff.pos).wrapping_add(srcSize);
        (*mtctx).inBuff.buffer = g_nullBuffer;
        (*mtctx).inBuff.filled = 0;
        if endFrame == 0 {
            let newPrefixSize = if srcSize < (*mtctx).targetPrefixSize {
                srcSize
            } else {
                (*mtctx).targetPrefixSize
            };
            (*mtctx).inBuff.prefix.start =
                src.add(srcSize).offset(-(newPrefixSize as isize)) as *const core::ffi::c_void;
            (*mtctx).inBuff.prefix.size = newPrefixSize;
        } else {
            (*mtctx).inBuff.prefix = kNullRange;
            (*mtctx).frameEnded = endFrame as core::ffi::c_uint;
            if (*mtctx).nextJobID == 0 {
                (*mtctx).params.fParams.checksumFlag = 0;
            }
        }
        if srcSize == 0 && (*mtctx).nextJobID > 0 {
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
            if (*((*mtctx).jobs).offset(wJobID as isize)).consumed
                == (*((*mtctx).jobs).offset(wJobID as isize)).src.size
            {
                break;
            }
            guard = (*((*mtctx).jobs).offset(wJobID as isize))
                .job_cond
                .wait(guard)
                .unwrap();
        }
    }
    let mut cSize = (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
    let srcConsumed = (*((*mtctx).jobs).offset(wJobID as isize)).consumed;
    let srcSize = (*((*mtctx).jobs).offset(wJobID as isize)).src.size;
    drop(guard);
    if ERR_isError(cSize) {
        ZSTDMT_waitForAllJobsCompleted(mtctx);
        ZSTDMT_releaseAllJobResources(mtctx);
        return cSize;
    }
    if srcConsumed == srcSize && (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded != 0
    {
        let checksum = ZSTD_XXH64_digest(&mut (*mtctx).serial.xxhState) as u32;
        MEM_writeLE32(
            ((*((*mtctx).jobs).offset(wJobID as isize)).dstBuff.start as *mut core::ffi::c_char)
                .add((*((*mtctx).jobs).offset(wJobID as isize)).cSize)
                as *mut core::ffi::c_void,
            checksum,
        );
        cSize = cSize.wrapping_add(4);
        let fresh10 = &mut (*((*mtctx).jobs).offset(wJobID as isize)).cSize;
        *fresh10 = (*fresh10).wrapping_add(4);
        (*((*mtctx).jobs).offset(wJobID as isize)).frameChecksumNeeded = 0;
    }
    if cSize > 0 {
        let toFlush = if cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
            < ((*output).size).wrapping_sub((*output).pos)
        {
            cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed)
        } else {
            ((*output).size).wrapping_sub((*output).pos)
        };
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
        *fresh11 = (*fresh11).wrapping_add(toFlush);
        if srcConsumed == srcSize && (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed == cSize
        {
            ZSTDMT_releaseBuffer(
                (*mtctx).bufPool,
                (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff,
            );
            (*((*mtctx).jobs).offset(wJobID as isize)).dstBuff = g_nullBuffer;
            (*((*mtctx).jobs).offset(wJobID as isize)).cSize = 0;
            (*mtctx).consumed = ((*mtctx).consumed).wrapping_add(srcSize as core::ffi::c_ulonglong);
            (*mtctx).produced = ((*mtctx).produced).wrapping_add(cSize as core::ffi::c_ulonglong);
            (*mtctx).doneJobID = ((*mtctx).doneJobID).wrapping_add(1);
        }
    }
    if cSize > (*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed {
        return cSize.wrapping_sub((*((*mtctx).jobs).offset(wJobID as isize)).dstFlushed);
    }
    if srcSize > srcConsumed {
        return 1;
    }
    if (*mtctx).doneJobID < (*mtctx).nextJobID {
        return 1;
    }
    if (*mtctx).jobReady != 0 {
        return 1;
    }
    if (*mtctx).inBuff.filled > 0 {
        return 1;
    }
    (*mtctx).allJobsCompleted = (*mtctx).frameEnded;
    if end as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint {
        return core::ffi::c_int::from((*mtctx).frameEnded == 0) as size_t;
    }
    0
}
unsafe fn ZSTDMT_getInputDataInUse(mtctx: *mut ZSTDMT_CCtx) -> Range {
    let firstJobID = (*mtctx).doneJobID;
    let lastJobID = (*mtctx).nextJobID;
    let mut jobID: core::ffi::c_uint = 0;
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
                range = (*((*mtctx).jobs).offset(wJobID as isize)).src;
            }
            return range;
        }
        jobID = jobID.wrapping_add(1);
    }
    kNullRange
}
unsafe fn ZSTDMT_isOverlapped(buffer: Buffer, range: Range) -> core::ffi::c_int {
    let bufferStart = buffer.start as *const u8;
    let rangeStart = range.start as *const u8;
    if rangeStart.is_null() || bufferStart.is_null() {
        return 0;
    }
    let bufferEnd = bufferStart.add(buffer.capacity);
    let rangeEnd = rangeStart.add(range.size);
    if bufferStart == bufferEnd || rangeStart == rangeEnd {
        return 0;
    }
    core::ffi::c_int::from(bufferStart < rangeEnd && rangeStart < bufferEnd)
}
unsafe fn ZSTDMT_doesOverlapWindow(buffer: Buffer, window: ZSTD_window_t) -> core::ffi::c_int {
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
    core::ffi::c_int::from(
        ZSTDMT_isOverlapped(buffer, extDict) != 0 || ZSTDMT_isOverlapped(buffer, prefix) != 0,
    )
}
unsafe fn ZSTDMT_waitForLdmComplete(mtctx: *mut ZSTDMT_CCtx, buffer: Buffer) {
    if (*mtctx).params.ldmParams.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        let mut guard = (*mtctx).serial.ldmWindowMutex.lock().unwrap();
        while ZSTDMT_doesOverlapWindow(buffer, (*mtctx).serial.ldmWindow) != 0 {
            guard = (*mtctx).serial.ldmWindowCond.wait(guard).unwrap();
        }
    }
}
unsafe fn ZSTDMT_tryGetInputRange(mtctx: *mut ZSTDMT_CCtx) -> core::ffi::c_int {
    let inUse = ZSTDMT_getInputDataInUse(mtctx);
    let spaceLeft = ((*mtctx).roundBuff.capacity).wrapping_sub((*mtctx).roundBuff.pos);
    let spaceNeeded = (*mtctx).targetSectionSize;
    let mut buffer = buffer_s {
        start: core::ptr::null_mut::<core::ffi::c_void>(),
        capacity: 0,
    };
    if spaceLeft < spaceNeeded {
        let start = (*mtctx).roundBuff.buffer;
        let prefixSize = (*mtctx).inBuff.prefix.size;
        buffer.start = start as *mut core::ffi::c_void;
        buffer.capacity = prefixSize;
        if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
            return 0;
        }
        ZSTDMT_waitForLdmComplete(mtctx, buffer);
        core::ptr::copy((*mtctx).inBuff.prefix.start.cast::<u8>(), start, prefixSize);
        (*mtctx).inBuff.prefix.start = start as *const core::ffi::c_void;
        (*mtctx).roundBuff.pos = prefixSize;
    }
    buffer.start =
        ((*mtctx).roundBuff.buffer).add((*mtctx).roundBuff.pos) as *mut core::ffi::c_void;
    buffer.capacity = spaceNeeded;
    if ZSTDMT_isOverlapped(buffer, inUse) != 0 {
        return 0;
    }
    ZSTDMT_waitForLdmComplete(mtctx, buffer);
    (*mtctx).inBuff.buffer = buffer;
    (*mtctx).inBuff.filled = 0;
    1
}
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
    syncPoint.toLoad = if (input.size).wrapping_sub(input.pos)
        < ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled)
    {
        (input.size).wrapping_sub(input.pos)
    } else {
        ((*mtctx).targetSectionSize).wrapping_sub((*mtctx).inBuff.filled)
    };
    syncPoint.flush = 0;
    if (*mtctx).params.rsyncable == 0 {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled)
        .wrapping_add(input.size)
        .wrapping_sub(input.pos)
        < RSYNC_MIN_BLOCK_SIZE as size_t
    {
        return syncPoint;
    }
    if ((*mtctx).inBuff.filled).wrapping_add(syncPoint.toLoad) < RSYNC_LENGTH as size_t {
        return syncPoint;
    }
    if (*mtctx).inBuff.filled < RSYNC_MIN_BLOCK_SIZE as size_t {
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
        pos = 0;
        prev = ((*mtctx).inBuff.buffer.start as *const u8)
            .add((*mtctx).inBuff.filled)
            .offset(-(RSYNC_LENGTH as isize));
        hash = ZSTD_rollingHash_compute(prev as *const core::ffi::c_void, RSYNC_LENGTH as size_t);
        if hash & hitMask == hitMask {
            syncPoint.toLoad = 0;
            syncPoint.flush = 1;
            return syncPoint;
        }
    }
    while pos < syncPoint.toLoad {
        let toRemove = (if pos < RSYNC_LENGTH as size_t {
            core::ffi::c_int::from(*prev.add(pos))
        } else {
            core::ffi::c_int::from(*istart.add(pos.wrapping_sub(RSYNC_LENGTH as size_t)))
        }) as u8;
        hash = ZSTD_rollingHash_rotate(hash, toRemove, *istart.add(pos), primePower);
        if hash & hitMask == hitMask {
            syncPoint.toLoad = pos.wrapping_add(1);
            syncPoint.flush = 1;
            pos = pos.wrapping_add(1);
            break;
        } else {
            pos = pos.wrapping_add(1);
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
        return Error::stage_wrong.to_error_code();
    }
    if (*mtctx).jobReady == 0 && (*input).size > (*input).pos {
        if ((*mtctx).inBuff.buffer.start).is_null() {
            if ZSTDMT_tryGetInputRange(mtctx) == 0 {
                // It is only possible for this operation to fail if there are
                // still compression jobs ongoing.
                // DEBUGLOG(5, "ZSTDMT_tryGetInputRange failed");
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
            forwardInputProgress =
                core::ffi::c_int::from(syncPoint.toLoad > 0) as core::ffi::c_uint;
        }
    }
    if (*input).pos < (*input).size
        && endOp as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
    {
        endOp = ZSTD_e_flush;
    }
    if (*mtctx).jobReady != 0
        || (*mtctx).inBuff.filled >= (*mtctx).targetSectionSize
        || endOp as core::ffi::c_uint != ZSTD_e_continue as core::ffi::c_int as core::ffi::c_uint
            && (*mtctx).inBuff.filled > 0
        || endOp as core::ffi::c_uint == ZSTD_e_end as core::ffi::c_int as core::ffi::c_uint
            && (*mtctx).frameEnded == 0
    {
        let jobSize = (*mtctx).inBuff.filled;
        let err_code = ZSTDMT_createCompressionJob(mtctx, jobSize, endOp);
        if ERR_isError(err_code) {
            return err_code;
        }
    }
    let remainingToFlush = ZSTDMT_flushProduced(
        mtctx,
        output,
        core::ffi::c_int::from(forwardInputProgress == 0) as core::ffi::c_uint,
        endOp,
    );
    if (*input).pos < (*input).size {
        return if remainingToFlush > 1 {
            remainingToFlush
        } else {
            1
        };
    }
    remainingToFlush
}
