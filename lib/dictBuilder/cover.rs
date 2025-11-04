use std::cmp::Ordering;
use std::ops::Range;
use std::sync::{Condvar, Mutex};

use libc::{free, malloc, memcpy, size_t};

use crate::lib::common::bits::ZSTD_highbit32;
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::pool::{POOL_add, POOL_create, POOL_free};
use crate::lib::compress::zstd_compress::{
    ZSTD_CCtx, ZSTD_CDict, ZSTD_compressBound, ZSTD_compress_usingCDict, ZSTD_createCCtx,
    ZSTD_createCDict, ZSTD_freeCCtx, ZSTD_freeCDict,
};
use crate::lib::dictBuilder::zdict::{ZDICT_finalizeDictionary, ZDICT_isError};
use crate::lib::zdict::experimental::{ZDICT_cover_params_t, ZDICT_DICTSIZE_MIN};

extern "C" {
    fn clock() -> clock_t;
}
type __clock_t = core::ffi::c_long;
type clock_t = __clock_t;
#[repr(C)]
struct COVER_map_t {
    data: Box<[COVER_map_pair_t]>,
    sizeLog: u32,
    size: u32,
    sizeMask: u32,
}

impl COVER_map_t {
    fn new(size: u32) -> Self {
        let sizeLog = ZSTD_highbit32(size) + 2;
        let size = 1 << sizeLog;
        let sizeMask = size - 1;
        let data = Box::from(vec![COVER_map_pair_t::EMPTY; size as size_t]);

        let mut this = Self {
            data,
            sizeLog,
            size,
            sizeMask,
        };

        COVER_map_clear(&mut this);

        this
    }
}

const MAP_EMPTY_VALUE: core::ffi::c_int = -(1);

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
struct COVER_map_pair_t {
    key: u32,
    value: u32,
}

impl COVER_map_pair_t {
    const EMPTY: Self = Self {
        key: MAP_EMPTY_VALUE as u32,
        value: MAP_EMPTY_VALUE as u32,
    };
}

#[derive(Debug, Default)]
#[repr(C)]
struct COVER_ctx_t<'a> {
    samples: &'a [u8],
    offsets: Box<[size_t]>,
    samplesSizes: &'a [size_t],
    nbSamples: size_t,
    nbTrainSamples: size_t,
    nbTestSamples: size_t,
    suffix: Box<[u32]>,
    suffixSize: size_t,
    freqs: Box<[u32]>,
    dmerAt: Box<[u32]>,
    d: core::ffi::c_uint,
    displayLevel: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(super) struct COVER_segment_t {
    pub(super) begin: u32,
    pub(super) end: u32,
    pub(super) score: u32,
}
#[repr(C)]
pub(super) struct COVER_epoch_info_t {
    pub(super) num: u32,
    pub(super) size: u32,
}
pub(super) struct COVER_best_t {
    pub(super) mutex: Mutex<()>,
    pub(super) cond: Condvar,
    pub(super) liveJobs: size_t,
    pub(super) dict: *mut core::ffi::c_void,
    pub(super) dictSize: size_t,
    pub(super) parameters: ZDICT_cover_params_t,
    pub(super) compressedSize: size_t,
}
#[repr(C)]
struct COVER_tryParameters_data_t<'a> {
    ctx: *const COVER_ctx_t<'a>,
    best: *mut COVER_best_t,
    dictBufferCapacity: size_t,
    parameters: ZDICT_cover_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(super) struct COVER_dictSelection_t {
    dictContent: *mut u8,
    dictSize: size_t,
    totalCompressedSize: size_t,
}
const CLOCKS_PER_SEC: core::ffi::c_int = 1000000;

const COVER_DEFAULT_SPLITPOINT: core::ffi::c_double = 1.0f64;

fn COVER_map_clear(map: &mut COVER_map_t) {
    map.data.fill(COVER_map_pair_t::EMPTY);
}

fn COVER_map_hash(map: &mut COVER_map_t, key: u32) -> u32 {
    const COVER_prime4bytes: u32 = 2654435761;
    (key.wrapping_mul(COVER_prime4bytes)) >> 32u32.wrapping_sub(map.sizeLog)
}

fn COVER_map_index(map: &mut COVER_map_t, key: u32) -> u32 {
    let hash = COVER_map_hash(map, key);
    let mut i = hash;
    loop {
        let entry = map.data[i as usize];
        if entry.value == MAP_EMPTY_VALUE as u32 {
            return i;
        }
        if entry.key == key {
            return i;
        }
        i = i.wrapping_add(1) & map.sizeMask;
    }
}

fn COVER_map_at(map: &mut COVER_map_t, key: u32) -> &mut u32 {
    let index = COVER_map_index(map, key) as usize;
    let entry = &mut map.data[index];
    if entry.value == MAP_EMPTY_VALUE as u32 {
        entry.key = key;
        entry.value = 0;
    }
    &mut entry.value
}

fn COVER_map_remove(map: &mut COVER_map_t, key: u32) {
    let mut i = COVER_map_index(map, key);
    let mut del = i as usize;
    let mut shift = 1;
    if map.data[del].value == MAP_EMPTY_VALUE as u32 {
        return;
    }
    i = (i + 1) & map.sizeMask;
    loop {
        let pos = i as usize;
        if map.data[pos].value == MAP_EMPTY_VALUE as u32 {
            map.data[del].value = MAP_EMPTY_VALUE as u32;
            return;
        }
        if i.wrapping_sub(COVER_map_hash(map, map.data[pos].key)) & map.sizeMask >= shift {
            map.data[del] = map.data[pos];
            del = pos;
            shift = 1;
        } else {
            shift = shift.wrapping_add(1);
        }
        i = (i + 1) & map.sizeMask;
    }
}

fn COVER_map_destroy(map: &mut COVER_map_t) {
    drop(core::mem::take(&mut map.data));
    map.size = 0;
}

pub(super) unsafe fn COVER_sum(
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
) -> size_t {
    let mut sum = 0 as size_t;
    let mut i: core::ffi::c_uint = 0;
    i = 0;
    while i < nbSamples {
        sum = sum.wrapping_add(*samplesSizes.offset(i as isize));
        i = i.wrapping_add(1);
    }
    sum
}

fn COVER_cmp(ctx: &COVER_ctx_t, lp: &u32, rp: &u32) -> Ordering {
    let lhs = *lp as usize;
    let rhs = *rp as usize;

    let lhs = &ctx.samples[lhs..][..ctx.d as usize];
    let rhs = &ctx.samples[rhs..][..ctx.d as usize];

    lhs.cmp(rhs)
}

fn COVER_cmp8(ctx: &COVER_ctx_t, lp: &u32, rp: &u32) -> Ordering {
    let mask = match ctx.d {
        8 => u64::MAX,
        n => (1u64 << (8u32 * n)) - 1,
    };

    let lhs = u64::from_le_bytes(ctx.samples[*lp as usize..][..8].try_into().unwrap()) & mask;
    let rhs = u64::from_le_bytes(ctx.samples[*rp as usize..][..8].try_into().unwrap()) & mask;

    Ord::cmp(&lhs, &rhs)
}

unsafe extern "C" fn COVER_strict_cmp(
    a: *const core::ffi::c_void,
    b: *const core::ffi::c_void,
    c: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let (lp, rp, g_coverCtx) = if cfg!(any(target_vendor = "apple", windows)) {
        (b, c, a)
    } else {
        (a, b, c)
    };

    let g_coverCtx = g_coverCtx.cast::<COVER_ctx_t>().as_ref().unwrap();
    let lhs = lp.cast::<u32>().as_ref().unwrap();
    let rhs = rp.cast::<u32>().as_ref().unwrap();
    COVER_cmp(g_coverCtx, lhs, rhs).then(if lp < rp {
        Ordering::Less
    } else {
        Ordering::Greater
    }) as _
}

unsafe extern "C" fn COVER_strict_cmp8(
    a: *const core::ffi::c_void,
    b: *const core::ffi::c_void,
    c: *const core::ffi::c_void,
) -> core::ffi::c_int {
    let (lp, rp, g_coverCtx) = if cfg!(any(target_vendor = "apple", windows)) {
        (b, c, a)
    } else {
        (a, b, c)
    };

    let g_coverCtx = g_coverCtx.cast::<COVER_ctx_t>().as_ref().unwrap();

    assert!(g_coverCtx.suffix.as_ptr_range().contains(&lp.cast()));
    assert!(g_coverCtx.suffix.as_ptr_range().contains(&rp.cast()));

    let lhs = lp.cast::<u32>().as_ref().unwrap();
    let rhs = rp.cast::<u32>().as_ref().unwrap();
    COVER_cmp8(g_coverCtx, lhs, rhs).then(if lp < rp {
        Ordering::Less
    } else {
        Ordering::Greater
    }) as _
}

crate::cfg_select! {
    miri => {
        /* a fallback implementation is used */
    }
    target_vendor = "apple" => {
        extern "C" {
            fn qsort_r(
                __base: *mut core::ffi::c_void,
                __nmemb: size_t,
                __size: size_t,
                __arg: *mut core::ffi::c_void,
                __compar: __compar_d_fn_t,
            );
        }
    }
    windows => {
        extern "C" {
            fn qsort_s(
                __base: *mut core::ffi::c_void,
                __nmemb: size_t,
                __size: size_t,
                __compar: __compar_d_fn_t,
                __arg: *mut core::ffi::c_void,
            );
        }
    }
    unix => {
        extern "C" {
            fn qsort_r(
                __base: *mut core::ffi::c_void,
                __nmemb: size_t,
                __size: size_t,
                __compar: __compar_d_fn_t,
                __arg: *mut core::ffi::c_void,
            );
        }
    }
    _ => {
        /* a fallback implementation is used */
    }
}

type __compar_d_fn_t = unsafe extern "C" fn(
    *const core::ffi::c_void,
    *const core::ffi::c_void,
    *const core::ffi::c_void,
) -> core::ffi::c_int;

fn stableSort(ctx: &mut COVER_ctx_t) {
    let compare_fn = if ctx.d <= 8 {
        COVER_strict_cmp8 as __compar_d_fn_t
    } else {
        COVER_strict_cmp as __compar_d_fn_t
    };

    debug_assert_eq!(ctx.suffixSize, ctx.suffix.len());

    crate::cfg_select! {
        all(not(miri), target_vendor = "apple") => {
            unsafe {
                qsort_r(
                    ctx.suffix.as_mut_ptr() as *mut core::ffi::c_void,
                    ctx.suffix.len(),
                    ::core::mem::size_of::<u32>(),
                    &raw mut *ctx as *mut core::ffi::c_void,
                    compare_fn,
                );
            }
        }
        all(not(miri), windows) => {
            unsafe {
                qsort_s(
                    ctx.suffix.as_mut_ptr() as *mut core::ffi::c_void,
                    ctx.suffix.len(),
                    ::core::mem::size_of::<u32>(),
                    compare_fn,
                    &raw mut *ctx as *mut core::ffi::c_void,
                );
            }
        }
        all(not(miri), unix) => {
            unsafe {
                qsort_r(
                    ctx.suffix.as_mut_ptr() as *mut core::ffi::c_void,
                    ctx.suffix.len(),
                    ::core::mem::size_of::<u32>(),
                    compare_fn,
                    &raw mut *ctx as *mut core::ffi::c_void,
                );
            }
        }
        _ => {
            ctx.suffix.sort_by(|lp, rp| {
                let lhs = &ctx.samples[*lp as usize..][.. ctx.d as size_t];
                let rhs = &ctx.samples[*rp as usize..][.. ctx.d as size_t];

                lhs.cmp(rhs)
            });
        }
    }
}

fn COVER_lower_bound(slice: &[usize], value: size_t) -> usize {
    let mut count = slice.len();
    let mut first = slice;
    while count != 0 {
        let step = count / 2;
        let mut ptr = first;
        ptr = &ptr[step..];
        if ptr[0] < value {
            ptr = &ptr[1..];
            first = ptr;
            count = count.wrapping_sub(step.wrapping_add(1));
        } else {
            count = step;
        }
    }

    slice.len() - first.len()
}

fn COVER_groupBy(ctx: &mut COVER_ctx_t, cmp: fn(&COVER_ctx_t, &u32, &u32) -> Ordering) {
    let data = &mut ctx.suffix;
    let count = data.len();

    let mut ptr = 0;
    let mut index = 0;
    while index < count {
        let mut grpEnd = ptr + 1;
        index = index.wrapping_add(1);
        while index < count && cmp(ctx, &ctx.suffix[ptr], &ctx.suffix[grpEnd]) == Ordering::Equal {
            grpEnd += 1;
            index = index.wrapping_add(1);
        }
        COVER_group(ctx, ptr..grpEnd);
        ptr = grpEnd;
    }
}

fn COVER_group(ctx: &mut COVER_ctx_t, range: Range<usize>) {
    let dmerId = range.start as u32;
    let group = &mut ctx.suffix[range];
    let mut freq = 0u32;
    let mut curOffsetPtr = &ctx.offsets[..ctx.nbSamples];
    let mut curSampleEnd = ctx.offsets[0];
    let mut it = group.iter().map(|v| *v as usize).peekable();
    while let Some(v) = it.next() {
        ctx.dmerAt[v] = dmerId;
        if v >= curSampleEnd {
            freq += 1;
            if it.peek().is_some() {
                let sampleEndPtr = COVER_lower_bound(curOffsetPtr, v);
                curSampleEnd = curOffsetPtr[sampleEndPtr];
                curOffsetPtr = &curOffsetPtr[sampleEndPtr + 1..];
            }
        }
    }
    group[0] = freq;
}

unsafe fn COVER_selectSegment(
    ctx: *const COVER_ctx_t,
    freqs: *mut u32,
    activeDmers: &mut COVER_map_t,
    begin: u32,
    end: u32,
    parameters: ZDICT_cover_params_t,
) -> COVER_segment_t {
    let k = parameters.k;
    let d = parameters.d;
    let dmersInK = k.wrapping_sub(d).wrapping_add(1);
    let mut bestSegment = {
        COVER_segment_t {
            begin: 0,
            end: 0,
            score: 0,
        }
    };
    let mut activeSegment = COVER_segment_t {
        begin: 0,
        end: 0,
        score: 0,
    };
    COVER_map_clear(activeDmers);
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0;
    while activeSegment.end < end {
        let newDmer = (*ctx).dmerAt[activeSegment.end as usize];
        let newDmerOcc = COVER_map_at(activeDmers, newDmer);
        if *newDmerOcc == 0 {
            activeSegment.score =
                (activeSegment.score).wrapping_add(*freqs.offset(newDmer as isize));
        }
        activeSegment.end = (activeSegment.end).wrapping_add(1);
        *newDmerOcc = (*newDmerOcc).wrapping_add(1);
        if (activeSegment.end).wrapping_sub(activeSegment.begin) == dmersInK.wrapping_add(1) {
            let delDmer = (*ctx).dmerAt[activeSegment.begin as usize];
            let delDmerOcc = COVER_map_at(activeDmers, delDmer);
            activeSegment.begin = (activeSegment.begin).wrapping_add(1);
            *delDmerOcc = (*delDmerOcc).wrapping_sub(1);
            if *delDmerOcc == 0 {
                COVER_map_remove(activeDmers, delDmer);
                activeSegment.score =
                    (activeSegment.score).wrapping_sub(*freqs.offset(delDmer as isize));
            }
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment;
        }
    }
    let mut newBegin = bestSegment.end;
    let mut newEnd = bestSegment.begin;
    let mut pos: u32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let freq = *freqs.offset((*ctx).dmerAt[pos as usize] as isize);
        if freq != 0 {
            newBegin = if newBegin < pos { newBegin } else { pos };
            newEnd = pos.wrapping_add(1);
        }
        pos = pos.wrapping_add(1);
    }
    bestSegment.begin = newBegin;
    bestSegment.end = newEnd;
    let mut pos_0: u32 = 0;
    pos_0 = bestSegment.begin;
    while pos_0 != bestSegment.end {
        *freqs.offset((*ctx).dmerAt[pos_0 as usize] as isize) = 0;
        pos_0 = pos_0.wrapping_add(1);
    }
    bestSegment
}

fn COVER_checkParameters(parameters: ZDICT_cover_params_t, maxDictSize: size_t) -> bool {
    if parameters.d == 0 || parameters.k == 0 {
        return false;
    }
    if parameters.k as size_t > maxDictSize {
        return false;
    }
    if parameters.d > parameters.k {
        return false;
    }
    if parameters.splitPoint <= 0.0 || parameters.splitPoint > 1.0 {
        return false;
    }

    true
}

fn COVER_ctx_destroy(ctx: &mut COVER_ctx_t) {
    drop(core::mem::take(&mut ctx.suffix));
    drop(core::mem::take(&mut ctx.freqs));
    drop(core::mem::take(&mut ctx.dmerAt));
    drop(core::mem::take(&mut ctx.offsets));
}

fn COVER_ctx_init<'a>(
    ctx: &'_ mut COVER_ctx_t<'a>,
    samples: &'a [u8],
    samplesSizes: &'a [size_t],
    nbSamples: core::ffi::c_uint,
    d: core::ffi::c_uint,
    splitPoint: core::ffi::c_double,
    displayLevel: core::ffi::c_int,
) -> size_t {
    let totalSamplesSize = samples.len();
    let nbTrainSamples = if splitPoint < 1.0f64 {
        (nbSamples as core::ffi::c_double * splitPoint) as core::ffi::c_uint
    } else {
        nbSamples
    };
    let nbTestSamples = if splitPoint < 1.0f64 {
        nbSamples.wrapping_sub(nbTrainSamples)
    } else {
        nbSamples
    };
    let trainingSamplesSize = if splitPoint < 1.0f64 {
        samplesSizes[..nbTrainSamples as usize].iter().sum()
    } else {
        totalSamplesSize
    };
    let testSamplesSize = if splitPoint < 1.0f64 {
        samplesSizes[nbTrainSamples as usize..][..nbTestSamples as usize]
            .iter()
            .sum()
    } else {
        totalSamplesSize
    };
    ctx.displayLevel = displayLevel;
    if totalSamplesSize
        < (if d as size_t > ::core::mem::size_of::<u64>() {
            d as size_t
        } else {
            ::core::mem::size_of::<u64>()
        })
        || totalSamplesSize
            >= (if ::core::mem::size_of::<size_t>() == 8 {
                -(1 as core::ffi::c_int) as core::ffi::c_uint
            } else {
                (1 as core::ffi::c_int as core::ffi::c_uint).wrapping_mul((1) << 30)
            }) as size_t
    {
        if displayLevel >= 1 {
            eprintln!(
                "Total samples size is too large ({} MB), maximum size is {} MB",
                (totalSamplesSize >> 20) as core::ffi::c_uint,
                (if ::core::mem::size_of::<size_t>() == 8 {
                    -(1 as core::ffi::c_int) as core::ffi::c_uint
                } else {
                    (1 as core::ffi::c_uint).wrapping_mul((1) << 30)
                }) >> 20,
            );
        }
        return Error::srcSize_wrong.to_error_code();
    }
    if nbTrainSamples < 5 {
        if displayLevel >= 1 {
            eprintln!(
                "Total number of training samples is {} and is invalid.",
                nbTrainSamples,
            );
        }
        return Error::srcSize_wrong.to_error_code();
    }
    if nbTestSamples < 1 {
        if displayLevel >= 1 {
            eprintln!(
                "Total number of testing samples is {} and is invalid.",
                nbTestSamples,
            );
        }
        return Error::srcSize_wrong.to_error_code();
    }
    *ctx = COVER_ctx_t::default();
    if displayLevel >= 2 {
        eprintln!(
            "Training on {} samples of total size {}",
            nbTrainSamples, trainingSamplesSize,
        );
    }
    if displayLevel >= 2 {
        eprintln!(
            "Testing on {} samples of total size {}",
            nbTestSamples, testSamplesSize,
        );
    }
    ctx.samples = samples;
    ctx.samplesSizes = samplesSizes;
    ctx.nbSamples = nbSamples as size_t;
    ctx.nbTrainSamples = nbTrainSamples as size_t;
    ctx.nbTestSamples = nbTestSamples as size_t;
    ctx.suffixSize = trainingSamplesSize
        .wrapping_sub(if d as size_t > ::core::mem::size_of::<u64>() {
            d as size_t
        } else {
            ::core::mem::size_of::<u64>()
        })
        .wrapping_add(1);
    ctx.suffix = (0..ctx.suffixSize as u32).collect();
    ctx.dmerAt = Box::from(vec![0u32; ctx.suffixSize]);
    ctx.offsets = Box::from(vec![0usize; nbSamples as usize + 1]);
    ctx.freqs = Box::default();
    ctx.d = d;
    let mut i: usize = 0;
    ctx.offsets[0] = 0;
    i = 1;
    while i <= nbSamples as usize {
        ctx.offsets[i] =
            ctx.offsets[i.wrapping_sub(1)].wrapping_add(samplesSizes[i.wrapping_sub(1)]);
        i = i.wrapping_add(1);
    }
    if displayLevel >= 2 {
        eprintln!("Constructing partial suffix array");
    }
    stableSort(ctx);
    if displayLevel >= 2 {
        eprintln!("Computing frequencies");
    }
    COVER_groupBy(
        ctx,
        if ctx.d <= 8 {
            COVER_cmp8 as fn(&COVER_ctx_t, &u32, &u32) -> Ordering
        } else {
            COVER_cmp as fn(&COVER_ctx_t, &u32, &u32) -> Ordering
        },
    );

    core::mem::swap(&mut ctx.freqs, &mut ctx.suffix);

    0
}

pub(super) fn COVER_warnOnSmallCorpus(
    maxDictSize: size_t,
    nbDmers: size_t,
    displayLevel: core::ffi::c_int,
) {
    let ratio = nbDmers as core::ffi::c_double / maxDictSize as core::ffi::c_double;
    if ratio >= 10.0 {
        return;
    }
    if displayLevel >= 1 {
        eprintln!(
            "WARNING: The maximum dictionary size {} is too large compared to the source size {}! \
                size(source)/size(dictionary) = {}, but it should be >= 10! \
                This may lead to a subpar dictionary! \
                We recommend training on sources at least 10x, \
                and preferably 100x the size of the dictionary!",
            maxDictSize, nbDmers, ratio,
        );
    }
}

pub(super) fn COVER_computeEpochs(
    maxDictSize: u32,
    nbDmers: u32,
    k: u32,
    passes: u32,
) -> COVER_epoch_info_t {
    let minEpochSize = k * 10;
    let mut epochs = COVER_epoch_info_t { num: 0, size: 0 };
    epochs.num = if 1 > maxDictSize / k / passes {
        1
    } else {
        maxDictSize / k / passes
    };
    epochs.size = nbDmers / epochs.num;
    if epochs.size >= minEpochSize {
        return epochs;
    }
    epochs.size = if minEpochSize < nbDmers {
        minEpochSize
    } else {
        nbDmers
    };
    epochs.num = nbDmers / epochs.size;
    epochs
}

unsafe fn COVER_buildDictionary(
    ctx: *const COVER_ctx_t,
    freqs: *mut u32,
    activeDmers: &mut COVER_map_t,
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    parameters: ZDICT_cover_params_t,
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut tail = dictBufferCapacity;
    let epochs = COVER_computeEpochs(
        dictBufferCapacity as u32,
        (*ctx).suffixSize as u32,
        parameters.k,
        4,
    );
    let maxZeroScoreRun = (if 10
        > (if (100) < epochs.num >> 3 {
            100
        } else {
            epochs.num >> 3
        }) {
        10
    } else if (100) < epochs.num >> 3 {
        100
    } else {
        epochs.num >> 3
    }) as size_t;
    let mut zeroScoreRun = 0 as size_t;
    let mut epoch: size_t = 0;
    let mut lastUpdateTime = 0;
    let displayLevel = (*ctx).displayLevel;
    if displayLevel >= 2 {
        eprintln!(
            "Breaking content into {} epochs of size {}",
            epochs.num, epochs.size,
        );
    }
    epoch = 0;
    while tail > 0 {
        let epochBegin = (epoch * epochs.size as size_t) as u32;
        let epochEnd = epochBegin.wrapping_add(epochs.size);
        let mut segmentSize: size_t = 0;
        let segment =
            COVER_selectSegment(ctx, freqs, activeDmers, epochBegin, epochEnd, parameters);
        if segment.score == 0 {
            zeroScoreRun = zeroScoreRun.wrapping_add(1);
            if zeroScoreRun >= maxZeroScoreRun {
                break;
            }
        } else {
            zeroScoreRun = 0;
            segmentSize = if ((segment.end)
                .wrapping_sub(segment.begin)
                .wrapping_add(parameters.d)
                .wrapping_sub(1) as size_t)
                < tail
            {
                (segment.end)
                    .wrapping_sub(segment.begin)
                    .wrapping_add(parameters.d)
                    .wrapping_sub(1) as size_t
            } else {
                tail
            };
            if segmentSize < parameters.d as size_t {
                break;
            }
            tail = tail.wrapping_sub(segmentSize);
            let samples = (*ctx).samples;
            memcpy(
                dict.add(tail) as *mut core::ffi::c_void,
                samples[segment.begin as usize..][..segmentSize as usize]
                    .as_ptr()
                    .cast(),
                segmentSize,
            );
            if displayLevel >= 2 {
                let refreshRate = CLOCKS_PER_SEC as __clock_t * 15 / 100;
                if clock() - lastUpdateTime > refreshRate || displayLevel >= 4 {
                    lastUpdateTime = clock();
                    eprint!(
                        "\r{}%       ",
                        (dictBufferCapacity.wrapping_sub(tail) * 100 / dictBufferCapacity)
                            as core::ffi::c_uint,
                    );
                }
            }
        }
        epoch = epoch.wrapping_add(1) % epochs.num as size_t;
    }
    if displayLevel >= 2 {
        println!("\r{:79 }\r", "");
    }
    tail
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_trainFromBuffer_cover))]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_cover(
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    mut parameters: ZDICT_cover_params_t,
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut ctx = COVER_ctx_t::default();
    let displayLevel = parameters.zParams.notificationLevel as core::ffi::c_int;
    parameters.splitPoint = 1.0f64;
    if !COVER_checkParameters(parameters, dictBufferCapacity) {
        if displayLevel >= 1 {
            eprintln!("Cover parameters incorrect");
        }
        return Error::parameter_outOfBound.to_error_code();
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            eprintln!("Cover must have at least one input file");
        }
        return Error::srcSize_wrong.to_error_code();
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        if displayLevel >= 1 {
            eprintln!("dictBufferCapacity must be at least {}", 256,);
        }
        return Error::dstSize_tooSmall.to_error_code();
    }

    let samplesSizes = if samplesSizes.is_null() || nbSamples == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(samplesSizes, nbSamples as usize)
    };
    let totalSamplesSize = samplesSizes.iter().sum::<usize>();
    let samples = if samplesBuffer.is_null() || totalSamplesSize == 0 {
        &[]
    } else {
        core::slice::from_raw_parts(samplesBuffer.cast::<u8>(), totalSamplesSize)
    };

    let initVal = COVER_ctx_init(
        &mut ctx,
        samples,
        samplesSizes,
        nbSamples,
        parameters.d,
        parameters.splitPoint,
        displayLevel,
    );
    if ERR_isError(initVal) {
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.suffixSize, displayLevel);
    let mut activeDmers =
        COVER_map_t::new((parameters.k).wrapping_sub(parameters.d).wrapping_add(1));
    if displayLevel >= 2 {
        eprintln!("Building dictionary");
    }
    let tail = COVER_buildDictionary(
        &ctx,
        ctx.freqs.as_mut_ptr(),
        &mut activeDmers,
        dictBuffer,
        dictBufferCapacity,
        parameters,
    );
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut core::ffi::c_void,
        dictBufferCapacity,
        dict.add(tail) as *const core::ffi::c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes.as_ptr(),
        nbSamples,
        parameters.zParams,
    );
    if !ERR_isError(dictionarySize) && displayLevel >= 2 {
        eprintln!("Constructed dictionary of size {}", dictionarySize,);
    }
    COVER_ctx_destroy(&mut ctx);
    COVER_map_destroy(&mut activeDmers);
    dictionarySize
}

pub(super) unsafe fn COVER_checkTotalCompressedSize(
    parameters: ZDICT_cover_params_t,
    samplesSizes: *const size_t,
    samples: *const u8,
    offsets: *mut size_t,
    nbTrainSamples: size_t,
    nbSamples: size_t,
    dict: *mut u8,
    dictBufferCapacity: size_t,
) -> size_t {
    let mut totalCompressedSize = Error::GENERIC.to_error_code();
    let mut cctx = core::ptr::null_mut::<ZSTD_CCtx>();
    let mut cdict = core::ptr::null_mut::<ZSTD_CDict>();
    let mut dst = core::ptr::null_mut::<core::ffi::c_void>();
    let mut dstCapacity: size_t = 0;
    let mut i: size_t = 0;
    let mut maxSampleSize = 0;
    i = if parameters.splitPoint < 1.0f64 {
        nbTrainSamples
    } else {
        0
    };
    while i < nbSamples {
        maxSampleSize = if *samplesSizes.add(i) > maxSampleSize {
            *samplesSizes.add(i)
        } else {
            maxSampleSize
        };
        i = i.wrapping_add(1);
    }
    dstCapacity = ZSTD_compressBound(maxSampleSize);
    dst = malloc(dstCapacity);
    cctx = ZSTD_createCCtx();
    cdict = ZSTD_createCDict(
        dict as *const core::ffi::c_void,
        dictBufferCapacity,
        parameters.zParams.compressionLevel,
    );
    if !(dst.is_null() || cctx.is_null() || cdict.is_null()) {
        totalCompressedSize = dictBufferCapacity;
        i = if parameters.splitPoint < 1.0f64 {
            nbTrainSamples
        } else {
            0
        };
        while i < nbSamples {
            let size = ZSTD_compress_usingCDict(
                cctx,
                dst,
                dstCapacity,
                samples.add(*offsets.add(i)) as *const core::ffi::c_void,
                *samplesSizes.add(i),
                cdict,
            );
            if ERR_isError(size) {
                totalCompressedSize = size;
                break;
            } else {
                totalCompressedSize = totalCompressedSize.wrapping_add(size);
                i = i.wrapping_add(1);
            }
        }
    }
    ZSTD_freeCCtx(cctx);
    ZSTD_freeCDict(cdict);
    if !dst.is_null() {
        free(dst);
    }
    totalCompressedSize
}

pub(super) fn COVER_best_init(best: &mut COVER_best_t) {
    best.liveJobs = 0;
    best.dict = core::ptr::null_mut();
    best.dictSize = 0;
    best.compressedSize = -(1 as core::ffi::c_int) as size_t;
    best.parameters = ZDICT_cover_params_t::default();
}

pub(super) fn COVER_best_wait(best: &mut COVER_best_t) {
    let mut guard = best.mutex.lock().unwrap();
    #[expect(clippy::while_immutable_condition)]
    while best.liveJobs != 0 {
        guard = best.cond.wait(guard).unwrap();
    }
}

pub(super) unsafe fn COVER_best_destroy(best: &mut COVER_best_t) {
    COVER_best_wait(best);
    if !(best.dict).is_null() {
        free(best.dict);
    }
}

pub(super) unsafe fn COVER_best_start(best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    let _guard = (*best).mutex.lock().unwrap();
    (*best).liveJobs = ((*best).liveJobs).wrapping_add(1);
}

pub(super) unsafe fn COVER_best_finish(
    best: *mut COVER_best_t,
    parameters: ZDICT_cover_params_t,
    selection: COVER_dictSelection_t,
) {
    let dict = selection.dictContent as *mut core::ffi::c_void;
    let compressedSize = selection.totalCompressedSize;
    let dictSize = selection.dictSize;
    if best.is_null() {
        return;
    }
    let mut liveJobs: size_t = 0;
    let _guard = (*best).mutex.lock().unwrap();
    (*best).liveJobs = ((*best).liveJobs).wrapping_sub(1);
    liveJobs = (*best).liveJobs;
    if compressedSize < (*best).compressedSize {
        if ((*best).dict).is_null() || (*best).dictSize < dictSize {
            if !((*best).dict).is_null() {
                free((*best).dict);
            }
            (*best).dict = malloc(dictSize);
            if ((*best).dict).is_null() {
                (*best).compressedSize = Error::GENERIC.to_error_code();
                (*best).dictSize = 0;
                (*best).cond.notify_one();
                return;
            }
        }
        if !dict.is_null() {
            memcpy((*best).dict, dict, dictSize);
            (*best).dictSize = dictSize;
            (*best).parameters = parameters;
            (*best).compressedSize = compressedSize;
        }
    }
    if liveJobs == 0 {
        (*best).cond.notify_all();
    }
}

fn setDictSelection(buf: *mut u8, s: size_t, csz: size_t) -> COVER_dictSelection_t {
    let mut ds = COVER_dictSelection_t {
        dictContent: core::ptr::null_mut::<u8>(),
        dictSize: 0,
        totalCompressedSize: 0,
    };
    ds.dictContent = buf;
    ds.dictSize = s;
    ds.totalCompressedSize = csz;
    ds
}

pub(super) fn COVER_dictSelectionError(error: size_t) -> COVER_dictSelection_t {
    setDictSelection(core::ptr::null_mut(), 0, error)
}

pub(super) fn COVER_dictSelectionIsError(selection: COVER_dictSelection_t) -> core::ffi::c_uint {
    (ERR_isError(selection.totalCompressedSize) || (selection.dictContent).is_null())
        as core::ffi::c_int as core::ffi::c_uint
}

pub(super) unsafe fn COVER_dictSelectionFree(selection: COVER_dictSelection_t) {
    free(selection.dictContent as *mut core::ffi::c_void);
}

pub(super) unsafe fn COVER_selectDict(
    customDictContent: *mut u8,
    dictBufferCapacity: size_t,
    mut dictContentSize: size_t,
    samplesBuffer: *const u8,
    samplesSizes: *const size_t,
    nbFinalizeSamples: core::ffi::c_uint,
    nbCheckSamples: size_t,
    nbSamples: size_t,
    params: ZDICT_cover_params_t,
    offsets: *mut size_t,
    mut totalCompressedSize: size_t,
) -> COVER_dictSelection_t {
    let mut largestDict = 0;
    let mut largestCompressed = 0;
    let customDictContentEnd = customDictContent.add(dictContentSize);
    let largestDictbuffer = malloc(dictBufferCapacity) as *mut u8;
    let candidateDictBuffer = malloc(dictBufferCapacity) as *mut u8;
    let regressionTolerance =
        params.shrinkDictMaxRegression as core::ffi::c_double / 100.0f64 + 1.00f64;
    if largestDictbuffer.is_null() || candidateDictBuffer.is_null() {
        free(largestDictbuffer as *mut core::ffi::c_void);
        free(candidateDictBuffer as *mut core::ffi::c_void);
        return COVER_dictSelectionError(dictContentSize);
    }
    memcpy(
        largestDictbuffer as *mut core::ffi::c_void,
        customDictContent as *const core::ffi::c_void,
        dictContentSize,
    );
    dictContentSize = ZDICT_finalizeDictionary(
        largestDictbuffer as *mut core::ffi::c_void,
        dictBufferCapacity,
        customDictContent as *const core::ffi::c_void,
        dictContentSize,
        samplesBuffer as *const core::ffi::c_void,
        samplesSizes,
        nbFinalizeSamples,
        params.zParams,
    );
    if ZDICT_isError(dictContentSize) != 0 {
        free(largestDictbuffer as *mut core::ffi::c_void);
        free(candidateDictBuffer as *mut core::ffi::c_void);
        return COVER_dictSelectionError(dictContentSize);
    }
    totalCompressedSize = COVER_checkTotalCompressedSize(
        params,
        samplesSizes,
        samplesBuffer,
        offsets,
        nbCheckSamples,
        nbSamples,
        largestDictbuffer,
        dictContentSize,
    );
    if ERR_isError(totalCompressedSize) {
        free(largestDictbuffer as *mut core::ffi::c_void);
        free(candidateDictBuffer as *mut core::ffi::c_void);
        return COVER_dictSelectionError(totalCompressedSize);
    }
    if params.shrinkDict == 0 {
        free(candidateDictBuffer as *mut core::ffi::c_void);
        return setDictSelection(largestDictbuffer, dictContentSize, totalCompressedSize);
    }
    largestDict = dictContentSize;
    largestCompressed = totalCompressedSize;
    dictContentSize = ZDICT_DICTSIZE_MIN as size_t;
    while dictContentSize < largestDict {
        memcpy(
            candidateDictBuffer as *mut core::ffi::c_void,
            largestDictbuffer as *const core::ffi::c_void,
            largestDict,
        );
        dictContentSize = ZDICT_finalizeDictionary(
            candidateDictBuffer as *mut core::ffi::c_void,
            dictBufferCapacity,
            customDictContentEnd.offset(-(dictContentSize as isize)) as *const core::ffi::c_void,
            dictContentSize,
            samplesBuffer as *const core::ffi::c_void,
            samplesSizes,
            nbFinalizeSamples,
            params.zParams,
        );
        if ZDICT_isError(dictContentSize) != 0 {
            free(largestDictbuffer as *mut core::ffi::c_void);
            free(candidateDictBuffer as *mut core::ffi::c_void);
            return COVER_dictSelectionError(dictContentSize);
        }
        totalCompressedSize = COVER_checkTotalCompressedSize(
            params,
            samplesSizes,
            samplesBuffer,
            offsets,
            nbCheckSamples,
            nbSamples,
            candidateDictBuffer,
            dictContentSize,
        );
        if ERR_isError(totalCompressedSize) {
            free(largestDictbuffer as *mut core::ffi::c_void);
            free(candidateDictBuffer as *mut core::ffi::c_void);
            return COVER_dictSelectionError(totalCompressedSize);
        }
        if totalCompressedSize as core::ffi::c_double
            <= largestCompressed as core::ffi::c_double * regressionTolerance
        {
            free(largestDictbuffer as *mut core::ffi::c_void);
            return setDictSelection(candidateDictBuffer, dictContentSize, totalCompressedSize);
        }
        dictContentSize *= 2;
    }
    dictContentSize = largestDict;
    totalCompressedSize = largestCompressed;
    free(candidateDictBuffer as *mut core::ffi::c_void);
    setDictSelection(largestDictbuffer, dictContentSize, totalCompressedSize)
}
unsafe fn COVER_tryParameters(opaque: *mut core::ffi::c_void) {
    let data = opaque as *mut COVER_tryParameters_data_t;
    let ctx = (*data).ctx.cast_mut();
    let parameters = (*data).parameters;
    let dictBufferCapacity = (*data).dictBufferCapacity;
    let totalCompressedSize = Error::GENERIC.to_error_code();
    let dict = malloc(dictBufferCapacity) as *mut u8;
    let mut selection = COVER_dictSelectionError(Error::GENERIC.to_error_code());
    let freqs = malloc(((*ctx).suffixSize).wrapping_mul(::core::mem::size_of::<u32>())) as *mut u32;
    let displayLevel = (*ctx).displayLevel;
    let mut activeDmers =
        COVER_map_t::new((parameters.k).wrapping_sub(parameters.d).wrapping_add(1));
    if dict.is_null() || freqs.is_null() {
        if displayLevel >= 1 {
            eprintln!("Failed to allocate buffers: out of memory");
        }
    } else {
        memcpy(
            freqs as *mut core::ffi::c_void,
            (*ctx).freqs.as_ptr() as *const core::ffi::c_void,
            ((*ctx).suffixSize).wrapping_mul(::core::mem::size_of::<u32>()),
        );
        let tail = COVER_buildDictionary(
            ctx,
            freqs,
            &mut activeDmers,
            dict as *mut core::ffi::c_void,
            dictBufferCapacity,
            parameters,
        );
        selection = COVER_selectDict(
            dict.add(tail),
            dictBufferCapacity,
            dictBufferCapacity.wrapping_sub(tail),
            (*ctx).samples.as_ptr(),
            (*ctx).samplesSizes.as_ptr(),
            (*ctx).nbTrainSamples as core::ffi::c_uint,
            (*ctx).nbTrainSamples,
            (*ctx).nbSamples,
            parameters,
            (*ctx).offsets.as_mut_ptr(),
            totalCompressedSize,
        );
        if COVER_dictSelectionIsError(selection) != 0 && displayLevel >= 1 {
            eprintln!("Failed to select dictionary");
        }
    }
    free(dict as *mut core::ffi::c_void);
    COVER_best_finish((*data).best, parameters, selection);
    free(data as *mut core::ffi::c_void);
    COVER_map_destroy(&mut activeDmers);
    COVER_dictSelectionFree(selection);
    free(freqs as *mut core::ffi::c_void);
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_optimizeTrainFromBuffer_cover))]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_cover(
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    parameters: *mut ZDICT_cover_params_t,
) -> size_t {
    let nbThreads = (*parameters).nbThreads;
    let splitPoint = if (*parameters).splitPoint <= 0.0f64 {
        COVER_DEFAULT_SPLITPOINT
    } else {
        (*parameters).splitPoint
    };
    let kMinD = if (*parameters).d == 0 {
        6
    } else {
        (*parameters).d
    };
    let kMaxD = if (*parameters).d == 0 {
        8
    } else {
        (*parameters).d
    };
    let kMinK = if (*parameters).k == 0 {
        50
    } else {
        (*parameters).k
    };
    let kMaxK = if (*parameters).k == 0 {
        2000
    } else {
        (*parameters).k
    };
    let kSteps = if (*parameters).steps == 0 {
        40
    } else {
        (*parameters).steps
    };
    let kStepSize = if kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps) > 1 {
        kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
    } else {
        1
    };
    let kIterations = (1 as core::ffi::c_uint)
        .wrapping_add(kMaxD.wrapping_sub(kMinD).wrapping_div(2))
        .wrapping_mul(
            (1 as core::ffi::c_uint)
                .wrapping_add(kMaxK.wrapping_sub(kMinK).wrapping_div(kStepSize)),
        );
    let shrinkDict = 0 as core::ffi::c_uint;
    let displayLevel = (*parameters).zParams.notificationLevel as core::ffi::c_int;
    let mut iteration = 1 as core::ffi::c_uint;
    let mut d: core::ffi::c_uint = 0;
    let mut k: core::ffi::c_uint = 0;
    let mut best = COVER_best_t {
        mutex: Mutex::new(()),
        cond: Condvar::new(),
        liveJobs: 0,
        dict: core::ptr::null_mut::<core::ffi::c_void>(),
        dictSize: 0,
        parameters: ZDICT_cover_params_t::default(),
        compressedSize: 0,
    };
    let mut pool = core::ptr::null_mut();
    let mut warned = 0;
    let mut lastUpdateTime = 0;
    if splitPoint <= 0.0 || splitPoint > 1.0 {
        if displayLevel >= 1 {
            eprintln!("Incorrect parameters");
        }
        return Error::parameter_outOfBound.to_error_code();
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1 {
            eprintln!("Incorrect parameters");
        }
        return Error::parameter_outOfBound.to_error_code();
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            eprintln!("Cover must have at least one input file");
        }
        return Error::srcSize_wrong.to_error_code();
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        if displayLevel >= 1 {
            eprintln!("dictBufferCapacity must be at least {}", 256);
        }
        return Error::dstSize_tooSmall.to_error_code();
    }
    if nbThreads > 1 {
        pool = POOL_create(nbThreads as size_t, 1);
        if pool.is_null() {
            return Error::memory_allocation.to_error_code();
        }
    }
    COVER_best_init(&mut best);
    if displayLevel >= 2 {
        eprintln!("Trying {} different sets of parameters", kIterations);
    }
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = COVER_ctx_t::default();
        if displayLevel >= 3 {
            eprintln!("d={}", d);
        }
        let childDisplayLevel = if displayLevel == 0 {
            0
        } else {
            displayLevel - 1
        };

        let samplesSizes = if samplesSizes.is_null() || nbSamples == 0 {
            &[]
        } else {
            core::slice::from_raw_parts(samplesSizes, nbSamples as usize)
        };
        let totalSamplesSize = samplesSizes.iter().sum::<usize>();
        let samples = if samplesBuffer.is_null() || totalSamplesSize == 0 {
            &[]
        } else {
            core::slice::from_raw_parts(samplesBuffer.cast::<u8>(), totalSamplesSize)
        };

        let initVal = COVER_ctx_init(
            &mut ctx,
            samples,
            samplesSizes,
            nbSamples,
            d,
            splitPoint,
            childDisplayLevel,
        );
        if ERR_isError(initVal) {
            if displayLevel >= 1 {
                eprintln!("Failed to initialize context");
            }
            COVER_best_destroy(&mut best);
            POOL_free(pool);
            return initVal;
        }
        if warned == 0 {
            COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.suffixSize, displayLevel);
            warned = 1;
        }
        k = kMinK;
        while k <= kMaxK {
            let data = malloc(::core::mem::size_of::<COVER_tryParameters_data_t>())
                as *mut COVER_tryParameters_data_t;
            if displayLevel >= 3 {
                eprintln!("k={}", k);
            }
            if data.is_null() {
                if displayLevel >= 1 {
                    eprintln!("Failed to allocate parameters");
                }
                COVER_best_destroy(&mut best);
                COVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return Error::memory_allocation.to_error_code();
            }
            (*data).ctx = &mut ctx;
            (*data).best = &mut best;
            (*data).dictBufferCapacity = dictBufferCapacity;
            (*data).parameters = *parameters;
            (*data).parameters.k = k;
            (*data).parameters.d = d;
            (*data).parameters.splitPoint = splitPoint;
            (*data).parameters.steps = kSteps;
            (*data).parameters.shrinkDict = shrinkDict;
            (*data).parameters.zParams.notificationLevel = ctx.displayLevel as core::ffi::c_uint;
            if !COVER_checkParameters((*data).parameters, dictBufferCapacity) {
                if displayLevel >= 1 {
                    eprintln!("Cover parameters incorrect");
                }
                free(data as *mut core::ffi::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(pool, COVER_tryParameters, data as *mut core::ffi::c_void);
                } else {
                    COVER_tryParameters(data as *mut core::ffi::c_void);
                }
                if displayLevel >= 2 {
                    let refreshRate = CLOCKS_PER_SEC as __clock_t * 15 / 100;
                    if clock() - lastUpdateTime > refreshRate || displayLevel >= 4 {
                        lastUpdateTime = clock();
                        eprintln!(
                            "\r{}%       ",
                            iteration.wrapping_mul(100).wrapping_div(kIterations),
                        );
                    }
                }
                iteration = iteration.wrapping_add(1);
            }
            k = k.wrapping_add(kStepSize);
        }
        COVER_best_wait(&mut best);
        COVER_ctx_destroy(&mut ctx);
        d = d.wrapping_add(2);
    }
    if displayLevel >= 2 {
        println!("\r{:79 }\r", "");
    }
    let dictSize = best.dictSize;
    if ERR_isError(best.compressedSize) {
        let compressedSize = best.compressedSize;
        COVER_best_destroy(&mut best);
        POOL_free(pool);
        return compressedSize;
    }
    *parameters = best.parameters;
    memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    dictSize
}
