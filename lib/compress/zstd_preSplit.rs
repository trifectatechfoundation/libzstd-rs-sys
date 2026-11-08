use core::ptr;

use core::ffi::{c_int, c_uint, c_void};

use crate::lib::compress::hist::HIST_add;

pub const ZSTD_SLIPBLOCK_WORKSPACESIZE: usize = 8208;

pub const BLOCKSIZE_MIN: c_int = 3500;
pub const THRESHOLD_PENALTY_RATE: c_int = 16;
pub const THRESHOLD_BASE: c_int = THRESHOLD_PENALTY_RATE - 2;
pub const THRESHOLD_PENALTY: c_int = 3;

pub const HASHLENGTH: usize = 2;
pub const HASHLOG_MAX: c_uint = 10;
pub const HASHTABLESIZE: usize = 1 << HASHLOG_MAX;
pub const KNUTH: c_uint = 0x9e3779b9;

/// for `hashLog` > 8, hash 2 bytes.
/// for `hashLog` == 8, just take the byte, no hashing.
/// The speed of this method relies on compile-time constant propagation
#[inline(always)]
fn hash2(p: [u8; HASHLENGTH], hashLog: c_uint) -> c_uint {
    debug_assert!(hashLog >= 8);
    if hashLog == 8 {
        return u32::from(p[0]);
    }
    debug_assert!(hashLog <= HASHLOG_MAX);
    u32::from(u16::from_ne_bytes(p)).wrapping_mul(KNUTH) >> 32u32.wrapping_sub(hashLog)
}

#[repr(C)]
pub struct Fingerprint<const N: usize = HASHTABLESIZE> {
    pub events: [c_uint; N],
    pub nbEvents: usize,
}

#[repr(C)]
pub struct FPStats {
    pub pastEvents: Fingerprint,
    pub newEvents: Fingerprint,
}

#[inline(always)]
fn addEvents_generic(fp: &mut Fingerprint, src: &[u8], samplingRate: usize, hashLog: c_uint) {
    let limit = src.len().wrapping_sub(HASHLENGTH).wrapping_add(1);

    debug_assert!(src.len() >= HASHLENGTH);
    for i in 0..limit.div_ceil(samplingRate) {
        let n = i * samplingRate;
        fp.events[hash2([src[n], src[n + 1]], hashLog) as usize] += 1;
    }

    fp.nbEvents += limit / samplingRate;
}

#[inline(always)]
fn recordFingerprint_generic<const SAMPLING_RATE: usize, const HASH_LOG: c_uint>(
    fp: &mut Fingerprint,
    src: &[u8],
) {
    fp.events[..1usize << HASH_LOG].fill(0);
    fp.nbEvents = 0;
    addEvents_generic(fp, src, SAMPLING_RATE, HASH_LOG);
}

fn fpDistance<const N: usize>(fp1: &Fingerprint<N>, fp2: &Fingerprint<N>, hashLog: c_uint) -> u64 {
    let mut distance = 0u64;

    debug_assert!(1usize << hashLog <= N);

    for n in 0..(1 << hashLog) {
        let dst1 = fp1.events[n] as i64 * fp2.nbEvents as i64;
        let dst2 = fp2.events[n] as i64 * fp1.nbEvents as i64;
        distance = distance.wrapping_add(dst1.abs_diff(dst2));
    }

    distance
}

/// Compares new events with past events.
///
/// Returns `true` when the fingerprints are considered "too different".
fn compareFingerprints<const N: usize>(
    ref_0: &Fingerprint<N>,
    newfp: &Fingerprint<N>,
    penalty: c_int,
    hashLog: c_uint,
) -> bool {
    debug_assert!(ref_0.nbEvents > 0);
    debug_assert!(newfp.nbEvents > 0);

    let p50 = ref_0.nbEvents * newfp.nbEvents;
    let deviation = fpDistance(ref_0, newfp, hashLog);
    let threshold = p50 as u64 * (THRESHOLD_BASE + penalty) as u64 / THRESHOLD_PENALTY_RATE as u64;
    deviation >= threshold
}

fn mergeEvents(acc: &mut Fingerprint, newfp: &Fingerprint) {
    for n in 0..HASHTABLESIZE {
        acc.events[n] += newfp.events[n];
    }
    acc.nbEvents += newfp.nbEvents;
}

pub const CHUNKSIZE: c_int = 8 << 10;

/// `fpstats` must be zeroed before each call.
fn ZSTD_splitBlock_byChunks(block: &[u8; 128 << 10], level: c_int, fpstats: &mut FPStats) -> usize {
    let blockSize = block.len();
    // NOTE: the sampling rates go from coarse to fine, so that the hash log of each
    // entry matches the corresponding entry of `hashParams` below.
    static records_fs: [fn(&mut Fingerprint, &[u8]) -> (); 4] = [
        recordFingerprint_generic::<43, 8>,
        recordFingerprint_generic::<11, 9>,
        recordFingerprint_generic::<5, 10>,
        recordFingerprint_generic::<1, 10>,
    ];

    static hashParams: [c_uint; 4] = [8, 9, 10, 10];
    debug_assert!((0..=3).contains(&level));
    let record_f = records_fs[level as usize];
    let mut penalty = THRESHOLD_PENALTY;

    let chunk = CHUNKSIZE as usize;

    record_f(&mut fpstats.pastEvents, &block[..chunk]);
    let mut pos = CHUNKSIZE as usize;
    while pos <= blockSize.wrapping_sub(CHUNKSIZE as usize) {
        record_f(&mut fpstats.newEvents, &block[pos..][..chunk]);
        if compareFingerprints(
            &fpstats.pastEvents,
            &fpstats.newEvents,
            penalty,
            hashParams[level as usize],
        ) {
            return pos;
        } else {
            mergeEvents(&mut fpstats.pastEvents, &fpstats.newEvents);
            if penalty > 0 {
                penalty -= 1;
            }
        }
        pos = pos.wrapping_add(CHUNKSIZE as usize);
    }
    debug_assert_eq!(pos, blockSize);
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
///
/// `fpstats` must be zeroed before each call.
fn ZSTD_splitBlock_fromBorders(block: &[u8; 128 << 10], fpstats: &mut FPStats) -> usize {
    const SEGMENT_SIZE: c_int = 512;
    let blockSize = block.len();

    let segment = SEGMENT_SIZE as usize;

    HIST_add(
        fpstats.pastEvents.events.first_chunk_mut().unwrap(),
        &block[..segment],
    );
    HIST_add(
        fpstats.newEvents.events.first_chunk_mut().unwrap(),
        &block[blockSize - segment..],
    );
    fpstats.newEvents.nbEvents = SEGMENT_SIZE as usize;
    fpstats.pastEvents.nbEvents = fpstats.newEvents.nbEvents;
    if !compareFingerprints(&fpstats.pastEvents, &fpstats.newEvents, 0, 8) {
        return blockSize;
    }
    let middle = blockSize / 2 - segment / 2;
    let mut middleEvents = Fingerprint {
        events: [0; HASHTABLESIZE],
        nbEvents: SEGMENT_SIZE as usize,
    };
    HIST_add(
        middleEvents.events.first_chunk_mut().unwrap(),
        &block[middle..][..segment],
    );
    let distFromBegin = fpDistance(&fpstats.pastEvents, &middleEvents, 8);
    let distFromEnd = fpDistance(&fpstats.newEvents, &middleEvents, 8);
    let minDistance = (SEGMENT_SIZE * SEGMENT_SIZE / 3) as u64;
    if distFromBegin.abs_diff(distFromEnd) < minDistance {
        return 64 * (1 << 10);
    }
    (if distFromBegin > distFromEnd {
        32 * (1 << 10)
    } else {
        96 * (1 << 10)
    }) as usize
}

/// Splits a block to find the optimal boundary for compression.
///
/// # Parameters
///
/// * `block` - The block (must be 128 KB)
/// * `level` - Detection level (0-4). Higher levels spend more energy to detect block boundaries.
/// * `workspace` - Workspace buffer (must be aligned for `usize`)
/// * `wkspSize` - Workspace size (must be at least `ZSTD_SLIPBLOCK_WORKSPACESIZE`)
///
/// # Note
///
/// For the time being, this function only accepts full 128 KB blocks.
/// While this could be extended to smaller sizes in the future,
/// it is not yet clear if this would be useful. TBD.
pub unsafe fn ZSTD_splitBlock(
    block: &[u8; 128 << 10],
    level: c_int,
    workspace: *mut c_void,
    wkspSize: usize,
) -> usize {
    debug_assert!((0..=4).contains(&level));

    debug_assert!(!workspace.is_null());
    debug_assert!(workspace.cast::<FPStats>().is_aligned());
    const { assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>()) }
    debug_assert!(wkspSize >= size_of::<FPStats>());

    let fpstats = workspace.cast::<FPStats>();
    ptr::write_bytes(fpstats, 0, 1);
    // SAFETY: the caller provides aligned, writable workspace disjoint from the
    // input block. `write_bytes` initializes all integer fields before borrowing.
    let fpstats = unsafe { &mut *fpstats };
    if level == 0 {
        return ZSTD_splitBlock_fromBorders(block, fpstats);
    }

    // level >= 1
    ZSTD_splitBlock_byChunks(block, level - 1, fpstats)
}
