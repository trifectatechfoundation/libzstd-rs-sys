use core::ptr;

use libc::size_t;

use crate::lib::common::mem::MEM_read16;
use crate::lib::compress::hist::HIST_add;

#[repr(C)]
pub struct Fingerprint {
    pub events: [core::ffi::c_uint; 1024],
    pub nbEvents: size_t,
}
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
unsafe fn hash2(p: *const core::ffi::c_void, hashLog: core::ffi::c_uint) -> core::ffi::c_uint {
    if hashLog == 8 {
        return u32::from(*(p as *const u8));
    }
    u32::from(MEM_read16(p)).wrapping_mul(KNUTH) >> (32 as core::ffi::c_uint).wrapping_sub(hashLog)
}
unsafe fn initStats(fpstats: *mut FPStats) {
    ptr::write_bytes(fpstats as *mut u8, 0, ::core::mem::size_of::<FPStats>());
}
#[inline(always)]
unsafe fn addEvents_generic(
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    samplingRate: size_t,
    hashLog: core::ffi::c_uint,
) {
    let p = src as *const core::ffi::c_char;
    let limit = srcSize.wrapping_sub(HASHLENGTH as size_t).wrapping_add(1);
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
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    samplingRate: size_t,
    hashLog: core::ffi::c_uint,
) {
    ptr::write_bytes(
        fp as *mut u8,
        0,
        ::core::mem::size_of::<core::ffi::c_uint>() << hashLog,
    );
    (*fp).nbEvents = 0;
    addEvents_generic(fp, src, srcSize, samplingRate, hashLog);
}
unsafe fn ZSTD_recordFingerprint_1(
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 1, 10);
}
unsafe fn ZSTD_recordFingerprint_5(
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 5, 10);
}
unsafe fn ZSTD_recordFingerprint_11(
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 11, 9);
}
unsafe fn ZSTD_recordFingerprint_43(
    fp: *mut Fingerprint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    recordFingerprint_generic(fp, src, srcSize, 43, 8);
}
unsafe fn abs64(s64: i64) -> u64 {
    (if s64 < 0 { -s64 } else { s64 }) as u64
}
unsafe fn fpDistance(
    fp1: *const Fingerprint,
    fp2: *const Fingerprint,
    hashLog: core::ffi::c_uint,
) -> u64 {
    let mut distance = 0u64;
    let mut n: size_t = 0;
    n = 0;
    while n < (1) << hashLog {
        distance = distance.wrapping_add(abs64(
            i64::from(*((*fp1).events).as_ptr().add(n)) * (*fp2).nbEvents as i64
                - i64::from(*((*fp2).events).as_ptr().add(n)) * (*fp1).nbEvents as i64,
        ));
        n = n.wrapping_add(1);
    }
    distance
}
unsafe fn compareFingerprints(
    ref_0: *const Fingerprint,
    newfp: *const Fingerprint,
    penalty: core::ffi::c_int,
    hashLog: core::ffi::c_uint,
) -> core::ffi::c_int {
    let p50 = (*ref_0).nbEvents * (*newfp).nbEvents;
    let deviation = fpDistance(ref_0, newfp, hashLog);
    let threshold = p50 as u64 * (THRESHOLD_BASE + penalty) as u64 / THRESHOLD_PENALTY_RATE as u64;
    core::ffi::c_int::from(deviation >= threshold)
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
pub const CHUNKSIZE: core::ffi::c_int = (8) << 10;
const ZSTD_SLIPBLOCK_WORKSPACESIZE: usize = 8208;
unsafe fn ZSTD_splitBlock_byChunks(
    blockStart: *const core::ffi::c_void,
    blockSize: size_t,
    level: core::ffi::c_int,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
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
    let p = blockStart as *const core::ffi::c_char;
    let mut penalty = THRESHOLD_PENALTY;
    let mut pos = 0;

    assert_eq!(blockSize, (128 << 10));
    assert!(!workspace.is_null());
    assert!(workspace.cast::<FPStats>().is_aligned());
    const { assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>()) }
    assert!(wkspSize >= size_of::<FPStats>());

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
    blockSize
}
unsafe fn ZSTD_splitBlock_fromBorders(
    blockStart: *const core::ffi::c_void,
    blockSize: size_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let fpstats = workspace as *mut FPStats;
    let middleEvents = (workspace as *mut core::ffi::c_char).offset(
        (512 as core::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong)
            as isize,
    ) as *mut core::ffi::c_void as *mut Fingerprint;

    assert_eq!(blockSize, (128 << 10));
    assert!(!workspace.is_null());
    assert!(workspace.cast::<FPStats>().is_aligned());
    const { assert!(ZSTD_SLIPBLOCK_WORKSPACESIZE >= size_of::<FPStats>()) }
    assert!(wkspSize >= size_of::<FPStats>());

    initStats(fpstats);
    HIST_add(
        ((*fpstats).pastEvents.events).as_mut_ptr(),
        blockStart,
        SEGMENT_SIZE as size_t,
    );
    HIST_add(
        ((*fpstats).newEvents.events).as_mut_ptr(),
        (blockStart as *const core::ffi::c_char)
            .add(blockSize)
            .offset(-(SEGMENT_SIZE as isize)) as *const core::ffi::c_void,
        SEGMENT_SIZE as size_t,
    );
    (*fpstats).newEvents.nbEvents = SEGMENT_SIZE as size_t;
    (*fpstats).pastEvents.nbEvents = (*fpstats).newEvents.nbEvents;
    if compareFingerprints(&(*fpstats).pastEvents, &(*fpstats).newEvents, 0, 8) == 0 {
        return blockSize;
    }
    HIST_add(
        ((*middleEvents).events).as_mut_ptr(),
        (blockStart as *const core::ffi::c_char)
            .add(blockSize / 2)
            .offset(-((SEGMENT_SIZE / 2) as isize)) as *const core::ffi::c_void,
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
pub const SEGMENT_SIZE: core::ffi::c_int = 512;
pub unsafe fn ZSTD_splitBlock(
    blockStart: *const core::ffi::c_void,
    blockSize: size_t,
    level: core::ffi::c_int,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    if level == 0 {
        return ZSTD_splitBlock_fromBorders(blockStart, blockSize, workspace, wkspSize);
    }
    ZSTD_splitBlock_byChunks(blockStart, blockSize, level - 1, workspace, wkspSize)
}
