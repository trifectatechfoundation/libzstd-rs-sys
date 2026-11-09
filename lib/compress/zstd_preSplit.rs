use core::ptr;

use core::ffi::{c_int, c_uint, c_void};

use crate::lib::compress::hist::HIST_add;

pub const ZSTD_SLIPBLOCK_WORKSPACESIZE: usize = 8208;

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

#[repr(C)]
pub struct BorderStats {
    pub pastEvents: Fingerprint<256>,
    pub newEvents: Fingerprint<256>,
    pub middleEvents: Fingerprint<256>,
}

impl<const N: usize> Fingerprint<N> {
    #[inline(always)]
    fn add_events(&mut self, src: &[u8], samplingRate: usize, hashLog: c_uint) {
        debug_assert!(src.len() >= HASHLENGTH);
        let limit = src.len() - HASHLENGTH + 1;

        for i in 0..limit.div_ceil(samplingRate) {
            let n = i * samplingRate;
            self.events[hash2([src[n], src[n + 1]], hashLog) as usize] += 1;
        }

        self.nbEvents += limit / samplingRate;
    }

    #[inline(always)]
    fn record<const SAMPLING_RATE: usize, const HASH_LOG: c_uint>(&mut self, src: &[u8]) {
        self.events[..1usize << HASH_LOG].fill(0);
        self.nbEvents = 0;
        self.add_events(src, SAMPLING_RATE, HASH_LOG);
    }

    fn distance(&self, other: &Self, hashLog: c_uint) -> u64 {
        let mut distance = 0u64;

        debug_assert!(1usize << hashLog <= N);

        for n in 0..(1 << hashLog) {
            let dst1 = self.events[n] as i64 * other.nbEvents as i64;
            let dst2 = other.events[n] as i64 * self.nbEvents as i64;
            distance += dst1.abs_diff(dst2);
        }

        distance
    }

    /// Compares new events with past events.
    ///
    /// Returns `true` when the fingerprints are considered "too different".
    fn compare(&self, newfp: &Self, penalty: c_int, hashLog: c_uint) -> bool {
        debug_assert!(self.nbEvents > 0);
        debug_assert!(newfp.nbEvents > 0);

        let p50 = self.nbEvents * newfp.nbEvents;
        let deviation = self.distance(newfp, hashLog);
        let threshold =
            p50 as u64 * (THRESHOLD_BASE + penalty) as u64 / THRESHOLD_PENALTY_RATE as u64;
        deviation >= threshold
    }

    fn merge_events(&mut self, newfp: &Self) {
        for n in 0..N {
            self.events[n] += newfp.events[n];
        }
        self.nbEvents += newfp.nbEvents;
    }
}

pub const CHUNKSIZE: usize = 8 << 10;

/// `fpstats` must be zeroed before each call.
fn ZSTD_splitBlock_byChunks(block: &[u8; 128 << 10], level: c_int, fpstats: &mut FPStats) -> usize {
    let blockSize = block.len();
    // NOTE: the sampling rates go from coarse to fine, so that the hash log of each
    // entry matches the corresponding entry of `hashParams` below.
    static records_fs: [fn(&mut Fingerprint, &[u8]) -> (); 4] = [
        Fingerprint::record::<43, 8>,
        Fingerprint::record::<11, 9>,
        Fingerprint::record::<5, 10>,
        Fingerprint::record::<1, 10>,
    ];

    static hashParams: [c_uint; 4] = [8, 9, 10, 10];
    debug_assert!((0..=3).contains(&level));
    let record_f = records_fs[level as usize];
    let mut penalty = THRESHOLD_PENALTY;

    debug_assert!(blockSize.is_multiple_of(CHUNKSIZE));
    let (chunks, _) = block.as_chunks::<CHUNKSIZE>();
    record_f(&mut fpstats.pastEvents, &chunks[0]);
    for (i, chunk) in chunks[1..].iter().enumerate() {
        record_f(&mut fpstats.newEvents, chunk);
        if fpstats
            .pastEvents
            .compare(&fpstats.newEvents, penalty, hashParams[level as usize])
        {
            return (i + 1) * CHUNKSIZE;
        } else {
            fpstats.pastEvents.merge_events(&fpstats.newEvents);
            if penalty > 0 {
                penalty -= 1;
            }
        }
    }
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
/// `stats` must be zeroed before each call.
fn ZSTD_splitBlock_fromBorders(block: &[u8; 128 << 10], stats: &mut BorderStats) -> usize {
    const SEGMENT_SIZE: usize = 512;
    let blockSize = block.len();

    let segment = SEGMENT_SIZE;

    HIST_add(&mut stats.pastEvents.events, &block[..segment]);
    HIST_add(&mut stats.newEvents.events, &block[blockSize - segment..]);
    stats.newEvents.nbEvents = SEGMENT_SIZE;
    stats.pastEvents.nbEvents = stats.newEvents.nbEvents;
    if !stats.pastEvents.compare(&stats.newEvents, 0, 8) {
        return blockSize;
    }
    let middle = blockSize / 2 - segment / 2;
    HIST_add(&mut stats.middleEvents.events, &block[middle..][..segment]);
    stats.middleEvents.nbEvents = SEGMENT_SIZE;
    let distFromBegin = stats.pastEvents.distance(&stats.middleEvents, 8);
    let distFromEnd = stats.newEvents.distance(&stats.middleEvents, 8);
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

    match level {
        0 => {
            debug_assert!(workspace.cast::<BorderStats>().is_aligned());
            const { assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<BorderStats>()) }
            debug_assert!(wkspSize >= size_of::<BorderStats>());

            let stats = workspace.cast::<BorderStats>();
            ptr::write_bytes(stats, 0, 1);
            // SAFETY: the caller provides aligned, writable workspace disjoint from the
            // input block. `write_bytes` initializes all integer fields before borrowing.
            let stats = unsafe { &mut *stats };
            ZSTD_splitBlock_fromBorders(block, stats)
        }
        _ => {
            debug_assert!(workspace.cast::<FPStats>().is_aligned());
            const { assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>()) }
            debug_assert!(wkspSize >= size_of::<FPStats>());

            let fpstats = workspace.cast::<FPStats>();
            ptr::write_bytes(fpstats, 0, 1);
            // SAFETY: the caller provides aligned, writable workspace disjoint from the
            // input block. `write_bytes` initializes all integer fields before borrowing.
            let fpstats = unsafe { &mut *fpstats };
            ZSTD_splitBlock_byChunks(block, level - 1, fpstats)
        }
    }
}
