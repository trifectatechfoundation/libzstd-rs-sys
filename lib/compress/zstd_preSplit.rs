use core::ptr;

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use libc::size_t;

use crate::lib::common::mem::MEM_read16;
use crate::lib::compress::hist::HIST_add;

const ZSTD_SLIPBLOCK_WORKSPACESIZE: usize = 8208;

pub const BLOCKSIZE_MIN: c_int = 3500;
pub const THRESHOLD_PENALTY_RATE: c_int = 16;
pub const THRESHOLD_BASE: c_int = THRESHOLD_PENALTY_RATE - 2;
pub const THRESHOLD_PENALTY: c_int = 3;

pub const HASHLENGTH: c_int = 2;
pub const HASHLOG_MAX: c_uint = 10;
pub const HASHTABLESIZE: c_int = (1) << HASHLOG_MAX;
pub const KNUTH: c_uint = 0x9e3779b9;

/// for `hashLog` > 8, hash 2 bytes.
/// for `hashLog` == 8, just take the byte, no hashing.
/// The speed of this method relies on compile-time constant propagation
#[inline(always)]
unsafe fn hash2(p: *const c_void, hashLog: c_uint) -> c_uint {
    debug_assert!(hashLog >= 8);
    if hashLog == 8 {
        return *(p as *const u8) as u32;
    }
    debug_assert!(hashLog <= HASHLOG_MAX);
    (MEM_read16(p) as u32).wrapping_mul(KNUTH) >> (32 as c_uint).wrapping_sub(hashLog)
}

#[repr(C)]
pub struct Fingerprint {
    pub events: [c_uint; 1024],
    pub nbEvents: size_t,
}

#[repr(C)]
pub struct FPStats {
    pub pastEvents: Fingerprint,
    pub newEvents: Fingerprint,
}

unsafe fn initStats(fpstats: *mut FPStats) {
    ptr::write_bytes(fpstats as *mut u8, 0, size_of::<FPStats>());
}

#[inline(always)]
unsafe fn addEvents_generic(
    fp: *mut Fingerprint,
    src: *const c_void,
    srcSize: size_t,
    samplingRate: size_t,
    hashLog: c_uint,
) {
    let p = src as *const c_char;
    let limit = srcSize.wrapping_sub(HASHLENGTH as size_t).wrapping_add(1);
    let mut n: size_t = 0;

    debug_assert!(srcSize >= HASHLENGTH as usize);
    while n < limit {
        let fresh0 = &mut (*((*fp).events)
            .as_mut_ptr()
            .offset(hash2(p.add(n) as *const c_void, hashLog) as isize));
        *fresh0 = (*fresh0).wrapping_add(1);
        n = n.wrapping_add(samplingRate);
    }
    (*fp).nbEvents = ((*fp).nbEvents).wrapping_add(limit / samplingRate);
}

#[inline(always)]
unsafe fn recordFingerprint_generic(
    fp: *mut Fingerprint,
    src: *const c_void,
    srcSize: size_t,
    samplingRate: size_t,
    hashLog: c_uint,
) {
    ptr::write_bytes(fp as *mut u8, 0, size_of::<c_uint>() << hashLog);
    (*fp).nbEvents = 0;
    addEvents_generic(fp, src, srcSize, samplingRate, hashLog);
}

pub type RecordEvents_f = Option<unsafe fn(*mut Fingerprint, *const c_void, size_t) -> ()>;

unsafe fn ZSTD_recordFingerprint_1(fp: *mut Fingerprint, src: *const c_void, srcSize: size_t) {
    recordFingerprint_generic(fp, src, srcSize, 1, 10);
}

unsafe fn ZSTD_recordFingerprint_5(fp: *mut Fingerprint, src: *const c_void, srcSize: size_t) {
    recordFingerprint_generic(fp, src, srcSize, 5, 10);
}

unsafe fn ZSTD_recordFingerprint_11(fp: *mut Fingerprint, src: *const c_void, srcSize: size_t) {
    recordFingerprint_generic(fp, src, srcSize, 11, 9);
}

unsafe fn ZSTD_recordFingerprint_43(fp: *mut Fingerprint, src: *const c_void, srcSize: size_t) {
    recordFingerprint_generic(fp, src, srcSize, 43, 8);
}

fn abs64(s64: i64) -> u64 {
    (if s64 < 0 { -s64 } else { s64 }) as u64
}

unsafe fn fpDistance(fp1: *const Fingerprint, fp2: *const Fingerprint, hashLog: c_uint) -> u64 {
    let mut distance = 0u64;
    let mut n: size_t = 0;

    debug_assert!(hashLog <= HASHLOG_MAX);
    while n < (1) << hashLog {
        distance = distance.wrapping_add(abs64(
            *((*fp1).events).as_ptr().add(n) as i64 * (*fp2).nbEvents as i64
                - *((*fp2).events).as_ptr().add(n) as i64 * (*fp1).nbEvents as i64,
        ));
        n = n.wrapping_add(1);
    }
    distance
}

/// Compares new events with past events.
///
/// Returns `1` when the fingerprints are considered "too different", `0` otherwise.
unsafe fn compareFingerprints(
    ref_0: *const Fingerprint,
    newfp: *const Fingerprint,
    penalty: c_int,
    hashLog: c_uint,
) -> c_int {
    debug_assert!((*ref_0).nbEvents > 0);
    debug_assert!((*newfp).nbEvents > 0);

    let p50 = (*ref_0).nbEvents * (*newfp).nbEvents;
    let deviation = fpDistance(ref_0, newfp, hashLog);
    let threshold = p50 as u64 * (THRESHOLD_BASE + penalty) as u64 / THRESHOLD_PENALTY_RATE as u64;
    (deviation >= threshold) as c_int
}

unsafe fn mergeEvents(acc: *mut Fingerprint, newfp: *const Fingerprint) {
    let mut n: size_t = 0;
    n = 0;
    while n < HASHTABLESIZE as size_t {
        let fresh1 = &mut (*((*acc).events).as_mut_ptr().add(n));
        *fresh1 = (*fresh1).wrapping_add(*((*newfp).events).as_ptr().add(n));
        n = n.wrapping_add(1);
    }
    (*acc).nbEvents = ((*acc).nbEvents).wrapping_add((*newfp).nbEvents);
}

pub const CHUNKSIZE: c_int = (8) << 10;

unsafe fn ZSTD_splitBlock_byChunks(
    blockStart: *const c_void,
    blockSize: size_t,
    level: c_int,
    workspace: *mut c_void,
    wkspSize: size_t,
) -> size_t {
    static records_fs: [RecordEvents_f; 4] = [
        Some(ZSTD_recordFingerprint_43 as unsafe fn(*mut Fingerprint, *const c_void, size_t) -> ()),
        Some(ZSTD_recordFingerprint_11 as unsafe fn(*mut Fingerprint, *const c_void, size_t) -> ()),
        Some(ZSTD_recordFingerprint_5 as unsafe fn(*mut Fingerprint, *const c_void, size_t) -> ()),
        Some(ZSTD_recordFingerprint_1 as unsafe fn(*mut Fingerprint, *const c_void, size_t) -> ()),
    ];
    static hashParams: [c_uint; 4] = [8, 9, 10, 10];
    debug_assert!((0..=3).contains(&level));
    let record_f: RecordEvents_f = *records_fs.as_ptr().offset(level as isize);
    let fpstats = workspace as *mut FPStats;
    let p = blockStart as *const c_char;
    let mut penalty = THRESHOLD_PENALTY;
    let mut pos = 0;

    debug_assert_eq!(blockSize, (128 << 10));
    debug_assert!(!workspace.is_null());
    debug_assert!(workspace.cast::<FPStats>().is_aligned());
    const { debug_assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>()) }
    debug_assert!(wkspSize >= size_of::<FPStats>());

    initStats(fpstats);
    record_f.unwrap_unchecked()(
        &mut (*fpstats).pastEvents,
        p as *const c_void,
        CHUNKSIZE as size_t,
    );
    pos = CHUNKSIZE as size_t;
    while pos <= blockSize.wrapping_sub(CHUNKSIZE as size_t) {
        record_f.unwrap_unchecked()(
            &mut (*fpstats).newEvents,
            p.add(pos) as *const c_void,
            CHUNKSIZE as size_t,
        );
        if compareFingerprints(
            &(*fpstats).pastEvents,
            &(*fpstats).newEvents,
            penalty,
            *hashParams.as_ptr().offset(level as isize),
        ) != 0
        {
            return pos;
        } else {
            mergeEvents(&mut (*fpstats).pastEvents, &(*fpstats).newEvents);
            if penalty > 0 {
                penalty -= 1;
            }
        }
        pos = pos.wrapping_add(CHUNKSIZE as size_t);
    }
    debug_assert!(pos == blockSize);
    blockSize
}

/// Very fast block splitting strategy.
///
/// Compares fingerprints from the beginning and end of the block, and derives from their
/// difference whether it's preferable to split in the middle. The process is repeated a
/// second time for finer-grained decisions.
///
/// Testing showed that 3 iterations did not bring improvements, so stopped at 2.
/// Benefits are good enough for a cheap heuristic. More accurate splitting saves more,
/// but speed impact is also more perceptible.
///
/// For better accuracy, use the more elaborate `*_byChunks` variant.
unsafe fn ZSTD_splitBlock_fromBorders(
    blockStart: *const c_void,
    blockSize: size_t,
    workspace: *mut c_void,
    wkspSize: size_t,
) -> size_t {
    const SEGMENT_SIZE: c_int = 512;

    let fpstats = workspace as *mut FPStats;
    let middleEvents = (workspace as *mut c_char)
        .offset((512 as c_ulong).wrapping_mul(size_of::<c_uint>() as c_ulong) as isize)
        as *mut c_void as *mut Fingerprint;

    debug_assert_eq!(blockSize, (128 << 10));
    debug_assert!(!workspace.is_null());
    debug_assert!(workspace.cast::<FPStats>().is_aligned());
    const { assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>()) }
    debug_assert!(wkspSize >= size_of::<FPStats>());

    initStats(fpstats);
    HIST_add(
        ((*fpstats).pastEvents.events).as_mut_ptr(),
        blockStart,
        SEGMENT_SIZE as size_t,
    );
    HIST_add(
        ((*fpstats).newEvents.events).as_mut_ptr(),
        (blockStart as *const c_char)
            .add(blockSize)
            .offset(-(SEGMENT_SIZE as isize)) as *const c_void,
        SEGMENT_SIZE as size_t,
    );
    (*fpstats).newEvents.nbEvents = SEGMENT_SIZE as size_t;
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    if compareFingerprints(&(*fpstats).pastEvents, &(*fpstats).newEvents, 0, 8) == 0 {
        return blockSize;
    }
    HIST_add(
        ((*middleEvents).events).as_mut_ptr(),
        (blockStart as *const c_char)
            .add(blockSize / 2)
            .offset(-((SEGMENT_SIZE / 2) as isize)) as *const c_void,
        SEGMENT_SIZE as size_t,
    );
    (*middleEvents).nbEvents = SEGMENT_SIZE as size_t;
    let distFromBegin = fpDistance(&(*fpstats).pastEvents, middleEvents, 8);
    let distFromEnd = fpDistance(&(*fpstats).newEvents, middleEvents, 8);
    let minDistance = (SEGMENT_SIZE * SEGMENT_SIZE / 3) as u64;
    if abs64(distFromBegin as i64 - distFromEnd as i64) < minDistance {
        return (64 * ((1) << 10)) as size_t;
    }
    (if distFromBegin > distFromEnd {
        32 * ((1) << 10)
    } else {
        96 * ((1) << 10)
    }) as size_t
}

/// Splits a block to find the optimal boundary for compression.
///
/// # Parameters
///
/// * `blockStart` - Pointer to the start of the block
/// * `blockSize` - Size of the block (must be 128 KB)
/// * `level` - Detection level (0-4). Higher levels spend more energy to detect block boundaries.
/// * `workspace` - Workspace buffer (must be aligned for `size_t`)
/// * `wkspSize` - Workspace size (must be at least `ZSTD_SLIPBLOCK_WORKSPACESIZE`)
///
/// # Note
///
/// For the time being, this function only accepts full 128 KB blocks.
/// While this could be extended to smaller sizes in the future,
/// it is not yet clear if this would be useful. TBD.
pub unsafe fn ZSTD_splitBlock(
    blockStart: *const c_void,
    blockSize: size_t,
    level: c_int,
    workspace: *mut c_void,
    wkspSize: size_t,
) -> size_t {
    debug_assert!((0..=4).contains(&level));
    if level == 0 {
        return ZSTD_splitBlock_fromBorders(blockStart, blockSize, workspace, wkspSize);
    }

    // level >= 1
    ZSTD_splitBlock_byChunks(blockStart, blockSize, level - 1, workspace, wkspSize)
}
