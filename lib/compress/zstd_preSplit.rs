use core::ptr;

use libc::size_t;

use crate::lib::common::mem::MEM_read16;
use crate::lib::compress::hist::HIST_add;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Fingerprint {
    pub events: [core::ffi::c_uint; 1024],
    pub nbEvents: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FPStats {
    pub pastEvents: Fingerprint,
    pub newEvents: Fingerprint,
}
pub type RecordEvents_f =
    Option<unsafe fn(*mut Fingerprint, *const core::ffi::c_void, size_t) -> ()>;
pub const THRESHOLD_PENALTY_RATE: core::ffi::c_int = 16;
pub const THRESHOLD_BASE: core::ffi::c_int = THRESHOLD_PENALTY_RATE - 2;
pub const THRESHOLD_PENALTY: core::ffi::c_int = 3;
pub const HASHLENGTH: core::ffi::c_int = 2;
pub const HASHLOG_MAX: core::ffi::c_int = 10;
pub const HASHTABLESIZE: core::ffi::c_int = (1) << HASHLOG_MAX;
pub const KNUTH: core::ffi::c_uint = 0x9e3779b9;
#[inline(always)]
unsafe fn hash2(
    mut p: *const core::ffi::c_void,
    mut hashLog: core::ffi::c_uint,
) -> core::ffi::c_uint {
    if hashLog == 8 {
        return *(p as *const u8).offset(0) as u32;
    }
    (MEM_read16(p) as u32).wrapping_mul(KNUTH) >> (32 as core::ffi::c_uint).wrapping_sub(hashLog)
}
unsafe fn initStats(mut fpstats: *mut FPStats) {
    ptr::write_bytes(fpstats as *mut u8, 0, ::core::mem::size_of::<FPStats>());
}
#[inline(always)]
unsafe fn addEvents_generic(
    mut fp: *mut Fingerprint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut samplingRate: size_t,
    mut hashLog: core::ffi::c_uint,
) {
    let mut p = src as *const core::ffi::c_char;
    let mut limit = srcSize.wrapping_sub(HASHLENGTH as size_t).wrapping_add(1);
    let mut n: size_t = 0;
    n = 0;
    while n < limit {
        let fresh0 = &mut (*((*fp).events)
            .as_mut_ptr()
            .offset(hash2(p.add(n) as *const core::ffi::c_void, hashLog) as isize));
        *fresh0 = (*fresh0).wrapping_add(1);
        n = n.wrapping_add(samplingRate);
    }
    (*fp).nbEvents = ((*fp).nbEvents).wrapping_add(limit / samplingRate);
}
#[inline(always)]
unsafe fn recordFingerprint_generic(
    mut fp: *mut Fingerprint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut samplingRate: size_t,
    mut hashLog: core::ffi::c_uint,
) {
    ptr::write_bytes(
        fp as *mut u8,
        0,
        ::core::mem::size_of::<core::ffi::c_uint>() * 1 << hashLog,
    );
    (*fp).nbEvents = 0;
    addEvents_generic(fp, src, srcSize, samplingRate, hashLog);
}
unsafe fn ZSTD_recordFingerprint_1(
    mut fp: *mut Fingerprint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 1, 10);
}
unsafe fn ZSTD_recordFingerprint_5(
    mut fp: *mut Fingerprint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 5, 10);
}
unsafe fn ZSTD_recordFingerprint_11(
    mut fp: *mut Fingerprint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 11, 9);
}
unsafe fn ZSTD_recordFingerprint_43(
    mut fp: *mut Fingerprint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 43, 8);
}
unsafe fn abs64(mut s64: i64) -> u64 {
    (if s64 < 0 { -s64 } else { s64 }) as u64
}
unsafe fn fpDistance(
    mut fp1: *const Fingerprint,
    mut fp2: *const Fingerprint,
    mut hashLog: core::ffi::c_uint,
) -> u64 {
    let mut distance = 0u64;
    let mut n: size_t = 0;
    n = 0;
    while n < (1) << hashLog {
        distance = distance.wrapping_add(abs64(
            *((*fp1).events).as_ptr().add(n) as i64 * (*fp2).nbEvents as i64
                - *((*fp2).events).as_ptr().add(n) as i64 * (*fp1).nbEvents as i64,
        ));
        n = n.wrapping_add(1);
    }
    distance
}
unsafe fn compareFingerprints(
    mut ref_0: *const Fingerprint,
    mut newfp: *const Fingerprint,
    mut penalty: core::ffi::c_int,
    mut hashLog: core::ffi::c_uint,
) -> core::ffi::c_int {
    let mut p50 = (*ref_0).nbEvents * (*newfp).nbEvents;
    let mut deviation = fpDistance(ref_0, newfp, hashLog);
    let mut threshold =
        p50 as u64 * (THRESHOLD_BASE + penalty) as u64 / THRESHOLD_PENALTY_RATE as u64;
    (deviation >= threshold) as core::ffi::c_int
}
unsafe fn mergeEvents(mut acc: *mut Fingerprint, mut newfp: *const Fingerprint) {
    let mut n: size_t = 0;
    n = 0;
    while n < HASHTABLESIZE as size_t {
        let fresh1 = &mut (*((*acc).events).as_mut_ptr().add(n));
        *fresh1 = (*fresh1).wrapping_add(*((*newfp).events).as_ptr().add(n));
        n = n.wrapping_add(1);
    }
    (*acc).nbEvents = ((*acc).nbEvents).wrapping_add((*newfp).nbEvents);
}
unsafe fn flushEvents(mut fpstats: *mut FPStats) {
    let mut n: size_t = 0;
    n = 0;
    while n < HASHTABLESIZE as size_t {
        *((*fpstats).pastEvents.events)
            .as_mut_ptr().add(n) = *((*fpstats).newEvents.events)
            .as_mut_ptr().add(n);
        n = n.wrapping_add(1);
    }
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    ptr::write_bytes(
        &mut (*fpstats).newEvents as *mut Fingerprint as *mut u8,
        0,
        ::core::mem::size_of::<Fingerprint>(),
    );
}
unsafe fn removeEvents(mut acc: *mut Fingerprint, mut slice: *const Fingerprint) {
    let mut n: size_t = 0;
    n = 0;
    while n < HASHTABLESIZE as size_t {
        let fresh2 = &mut (*((*acc).events).as_mut_ptr().add(n));
        *fresh2 = (*fresh2).wrapping_sub(*((*slice).events).as_ptr().add(n));
        n = n.wrapping_add(1);
    }
    (*acc).nbEvents = ((*acc).nbEvents).wrapping_sub((*slice).nbEvents);
}
pub const CHUNKSIZE: core::ffi::c_int = (8) << 10;
unsafe fn ZSTD_splitBlock_byChunks(
    mut blockStart: *const core::ffi::c_void,
    mut blockSize: size_t,
    mut level: core::ffi::c_int,
    mut workspace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    static records_fs: [RecordEvents_f; 4] = [
        Some(
            ZSTD_recordFingerprint_43
                as unsafe fn(*mut Fingerprint, *const core::ffi::c_void, size_t) -> (),
        ),
        Some(
            ZSTD_recordFingerprint_11
                as unsafe fn(*mut Fingerprint, *const core::ffi::c_void, size_t) -> (),
        ),
        Some(
            ZSTD_recordFingerprint_5
                as unsafe fn(*mut Fingerprint, *const core::ffi::c_void, size_t) -> (),
        ),
        Some(
            ZSTD_recordFingerprint_1
                as unsafe fn(*mut Fingerprint, *const core::ffi::c_void, size_t) -> (),
        ),
    ];
    static hashParams: [core::ffi::c_uint; 4] = [8, 9, 10, 10];
    let record_f: RecordEvents_f = *records_fs.as_ptr().offset(level as isize);
    let fpstats = workspace as *mut FPStats;
    let mut p = blockStart as *const core::ffi::c_char;
    let mut penalty = THRESHOLD_PENALTY;
    let mut pos = 0;
    initStats(fpstats);
    record_f.unwrap_unchecked()(
        &mut (*fpstats).pastEvents,
        p as *const core::ffi::c_void,
        CHUNKSIZE as size_t,
    );
    pos = CHUNKSIZE as size_t;
    while pos <= blockSize.wrapping_sub(CHUNKSIZE as size_t) {
        record_f.unwrap_unchecked()(
            &mut (*fpstats).newEvents,
            p.add(pos) as *const core::ffi::c_void,
            CHUNKSIZE as size_t,
        );
        if compareFingerprints(
            &mut (*fpstats).pastEvents,
            &mut (*fpstats).newEvents,
            penalty,
            *hashParams.as_ptr().offset(level as isize),
        ) != 0
        {
            return pos;
        } else {
            mergeEvents(&mut (*fpstats).pastEvents, &mut (*fpstats).newEvents);
            if penalty > 0 {
                penalty -= 1;
            }
        }
        pos = pos.wrapping_add(CHUNKSIZE as size_t);
    }
    blockSize
}
unsafe fn ZSTD_splitBlock_fromBorders(
    mut blockStart: *const core::ffi::c_void,
    mut blockSize: size_t,
    mut workspace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let fpstats = workspace as *mut FPStats;
    let mut middleEvents = (workspace as *mut core::ffi::c_char).offset(
        (512 as core::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong)
            as isize,
    ) as *mut core::ffi::c_void as *mut Fingerprint;
    initStats(fpstats);
    HIST_add(
        ((*fpstats).pastEvents.events).as_mut_ptr(),
        blockStart,
        SEGMENT_SIZE as size_t,
    );
    HIST_add(
        ((*fpstats).newEvents.events).as_mut_ptr(),
        (blockStart as *const core::ffi::c_char).add(blockSize)
            .offset(-(SEGMENT_SIZE as isize)) as *const core::ffi::c_void,
        SEGMENT_SIZE as size_t,
    );
    (*fpstats).newEvents.nbEvents = SEGMENT_SIZE as size_t;
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    if compareFingerprints(&mut (*fpstats).pastEvents, &mut (*fpstats).newEvents, 0, 8) == 0 {
        return blockSize;
    }
    HIST_add(
        ((*middleEvents).events).as_mut_ptr(),
        (blockStart as *const core::ffi::c_char).add(blockSize / 2)
            .offset(-((SEGMENT_SIZE / 2) as isize)) as *const core::ffi::c_void,
        SEGMENT_SIZE as size_t,
    );
    (*middleEvents).nbEvents = SEGMENT_SIZE as size_t;
    let distFromBegin = fpDistance(&mut (*fpstats).pastEvents, middleEvents, 8);
    let distFromEnd = fpDistance(&mut (*fpstats).newEvents, middleEvents, 8);
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
pub const SEGMENT_SIZE: core::ffi::c_int = 512;
pub unsafe fn ZSTD_splitBlock(
    mut blockStart: *const core::ffi::c_void,
    mut blockSize: size_t,
    mut level: core::ffi::c_int,
    mut workspace: *mut core::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    if level == 0 {
        return ZSTD_splitBlock_fromBorders(blockStart, blockSize, workspace, wkspSize);
    }
    ZSTD_splitBlock_byChunks(blockStart, blockSize, level - 1, workspace, wkspSize)
}
