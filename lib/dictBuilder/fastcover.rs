use core::ptr;

use libc::{
    calloc, fflush, fprintf, free, malloc, memcpy, size_t, FILE, PTHREAD_COND_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER,
};

use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::mem::MEM_readLE64;
use crate::lib::common::pool::{POOL_add, POOL_create, POOL_ctx, POOL_free};
use crate::lib::dictBuilder::cover::{
    COVER_best_destroy, COVER_best_finish, COVER_best_init, COVER_best_s, COVER_best_start,
    COVER_best_t, COVER_best_wait, COVER_computeEpochs, COVER_dictSelectionError,
    COVER_dictSelectionFree, COVER_dictSelectionIsError, COVER_selectDict, COVER_sum,
    COVER_warnOnSmallCorpus, ZDICT_cover_params_t,
};
use crate::lib::dictBuilder::zdict::{ZDICT_finalizeDictionary, ZDICT_params_t};
use crate::lib::zstd::*;

extern "C" {
    static mut stderr: *mut FILE;
    fn clock() -> clock_t;
}
pub type __clock_t = core::ffi::c_long;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_fastCover_params_t {
    pub k: core::ffi::c_uint,
    pub d: core::ffi::c_uint,
    pub f: core::ffi::c_uint,
    pub steps: core::ffi::c_uint,
    pub nbThreads: core::ffi::c_uint,
    pub splitPoint: core::ffi::c_double,
    pub accel: core::ffi::c_uint,
    pub shrinkDict: core::ffi::c_uint,
    pub shrinkDictMaxRegression: core::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_accel_t {
    pub finalize: core::ffi::c_uint,
    pub skip: core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_ctx_t {
    pub samples: *const u8,
    pub offsets: *mut size_t,
    pub samplesSizes: *const size_t,
    pub nbSamples: size_t,
    pub nbTrainSamples: size_t,
    pub nbTestSamples: size_t,
    pub nbDmers: size_t,
    pub freqs: *mut u32,
    pub d: core::ffi::c_uint,
    pub f: core::ffi::c_uint,
    pub accelParams: FASTCOVER_accel_t,
    pub displayLevel: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_segment_t {
    pub begin: u32,
    pub end: u32,
    pub score: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_epoch_info_t {
    pub num: u32,
    pub size: u32,
}
pub type FASTCOVER_tryParameters_data_t = FASTCOVER_tryParameters_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FASTCOVER_tryParameters_data_s {
    pub ctx: *const FASTCOVER_ctx_t,
    pub best: *mut COVER_best_t,
    pub dictBufferCapacity: size_t,
    pub parameters: ZDICT_cover_params_t,
}
pub type COVER_dictSelection_t = COVER_dictSelection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_dictSelection {
    pub dictContent: *mut u8,
    pub dictSize: size_t,
    pub totalCompressedSize: size_t,
}
static prime6bytes: u64 = 227718039650203;
unsafe fn ZSTD_hash6(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    ((((u << (64 as core::ffi::c_int - 48 as core::ffi::c_int)) * prime6bytes) ^ s)
        >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash6Ptr(mut p: *const core::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash6(MEM_readLE64(p), h, 0)
}
static prime8bytes: u64 = 0xcf1bbcdcb7a56463;
unsafe fn ZSTD_hash8(mut u: u64, mut h: u32, mut s: u64) -> size_t {
    (((u * prime8bytes) ^ s) >> 64u32.wrapping_sub(h)) as size_t
}
unsafe fn ZSTD_hash8Ptr(mut p: *const core::ffi::c_void, mut h: u32) -> size_t {
    ZSTD_hash8(MEM_readLE64(p), h, 0)
}
pub const ZSTD_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const ZDICT_DICTSIZE_MIN: core::ffi::c_int = 256;
pub const NULL: core::ffi::c_int = 0;
pub const CLOCKS_PER_SEC: core::ffi::c_int = 1000000;
pub const FASTCOVER_MAX_F: core::ffi::c_int = 31;
pub const FASTCOVER_MAX_ACCEL: core::ffi::c_int = 10;
pub const FASTCOVER_DEFAULT_SPLITPOINT: core::ffi::c_double = 0.75f64;
pub const DEFAULT_F: core::ffi::c_int = 20;
pub const DEFAULT_ACCEL: core::ffi::c_int = 1;
unsafe fn FASTCOVER_hashPtrToIndex(
    mut p: *const core::ffi::c_void,
    mut f: u32,
    mut d: core::ffi::c_uint,
) -> size_t {
    if d == 6 {
        return ZSTD_hash6Ptr(p, f);
    }
    ZSTD_hash8Ptr(p, f)
}
static FASTCOVER_defaultAccelParameters: [FASTCOVER_accel_t; 11] = [
    {
        FASTCOVER_accel_t {
            finalize: 100,
            skip: 0,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 100,
            skip: 0,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 50,
            skip: 1,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 34,
            skip: 2,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 25,
            skip: 3,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 20,
            skip: 4,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 17,
            skip: 5,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 14,
            skip: 6,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 13,
            skip: 7,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 11,
            skip: 8,
        }
    },
    {
        FASTCOVER_accel_t {
            finalize: 10,
            skip: 9,
        }
    },
];
unsafe fn FASTCOVER_selectSegment(
    mut ctx: *const FASTCOVER_ctx_t,
    mut freqs: *mut u32,
    mut begin: u32,
    mut end: u32,
    mut parameters: ZDICT_cover_params_t,
    mut segmentFreqs: *mut u16,
) -> COVER_segment_t {
    let k = parameters.k;
    let d = parameters.d;
    let f = (*ctx).f;
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
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0;
    while activeSegment.end < end {
        let idx = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(activeSegment.end as isize) as *const core::ffi::c_void,
            f,
            d,
        );
        if *segmentFreqs.add(idx) as core::ffi::c_int == 0 {
            activeSegment.score = (activeSegment.score).wrapping_add(*freqs.add(idx));
        }
        activeSegment.end = (activeSegment.end).wrapping_add(1);
        let fresh0 = &mut (*segmentFreqs.add(idx));
        *fresh0 = (*fresh0 as core::ffi::c_int + 1) as u16;
        if (activeSegment.end).wrapping_sub(activeSegment.begin) == dmersInK.wrapping_add(1) {
            let delIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).offset(activeSegment.begin as isize) as *const core::ffi::c_void,
                f,
                d,
            );
            let fresh1 = &mut (*segmentFreqs.add(delIndex));
            *fresh1 = (*fresh1 as core::ffi::c_int - 1) as u16;
            if *segmentFreqs.add(delIndex) as core::ffi::c_int == 0 {
                activeSegment.score = (activeSegment.score).wrapping_sub(*freqs.add(delIndex));
            }
            activeSegment.begin = (activeSegment.begin).wrapping_add(1);
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment;
        }
    }
    while activeSegment.begin < end {
        let delIndex_0 = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(activeSegment.begin as isize) as *const core::ffi::c_void,
            f,
            d,
        );
        let fresh2 = &mut (*segmentFreqs.add(delIndex_0));
        *fresh2 = (*fresh2 as core::ffi::c_int - 1) as u16;
        activeSegment.begin = (activeSegment.begin).wrapping_add(1);
    }
    let mut pos: u32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let i = FASTCOVER_hashPtrToIndex(
            ((*ctx).samples).offset(pos as isize) as *const core::ffi::c_void,
            f,
            d,
        );
        *freqs.add(i) = 0;
        pos = pos.wrapping_add(1);
    }
    bestSegment
}
unsafe fn FASTCOVER_checkParameters(
    mut parameters: ZDICT_cover_params_t,
    mut maxDictSize: size_t,
    mut f: core::ffi::c_uint,
    mut accel: core::ffi::c_uint,
) -> core::ffi::c_int {
    if parameters.d == 0 || parameters.k == 0 {
        return 0;
    }
    if parameters.d != 6 && parameters.d != 8 {
        return 0;
    }
    if parameters.k as size_t > maxDictSize {
        return 0;
    }
    if parameters.d > parameters.k {
        return 0;
    }
    if f > FASTCOVER_MAX_F as core::ffi::c_uint || f == 0 {
        return 0;
    }
    if parameters.splitPoint <= 0.0 || parameters.splitPoint > 1.0 {
        return 0;
    }
    if accel > 10 || accel == 0 {
        return 0;
    }
    1
}
unsafe fn FASTCOVER_ctx_destroy(mut ctx: *mut FASTCOVER_ctx_t) {
    if ctx.is_null() {
        return;
    }
    free((*ctx).freqs as *mut core::ffi::c_void);
    (*ctx).freqs = NULL as *mut u32;
    free((*ctx).offsets as *mut core::ffi::c_void);
    (*ctx).offsets = NULL as *mut size_t;
}
unsafe fn FASTCOVER_computeFrequency(mut freqs: *mut u32, mut ctx: *const FASTCOVER_ctx_t) {
    let f = (*ctx).f;
    let d = (*ctx).d;
    let skip = (*ctx).accelParams.skip;
    let readLength = if d > 8 { d } else { 8 };
    let mut i: size_t = 0;
    i = 0;
    while i < (*ctx).nbTrainSamples {
        let mut start = *((*ctx).offsets).add(i);
        let currSampleEnd = *((*ctx).offsets).add(i.wrapping_add(1));
        while start.wrapping_add(readLength as size_t) <= currSampleEnd {
            let dmerIndex = FASTCOVER_hashPtrToIndex(
                ((*ctx).samples).add(start) as *const core::ffi::c_void,
                f,
                d,
            );
            let fresh3 = &mut (*freqs.add(dmerIndex));
            *fresh3 = (*fresh3).wrapping_add(1);
            start = start.wrapping_add(skip as size_t).wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe fn FASTCOVER_ctx_init(
    mut ctx: *mut FASTCOVER_ctx_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
    mut d: core::ffi::c_uint,
    mut splitPoint: core::ffi::c_double,
    mut f: core::ffi::c_uint,
    mut accelParams: FASTCOVER_accel_t,
    mut displayLevel: core::ffi::c_int,
) -> size_t {
    let samples = samplesBuffer as *const u8;
    let totalSamplesSize = COVER_sum(samplesSizes, nbSamples);
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
        COVER_sum(samplesSizes, nbTrainSamples)
    } else {
        totalSamplesSize
    };
    let testSamplesSize = if splitPoint < 1.0f64 {
        COVER_sum(samplesSizes.offset(nbTrainSamples as isize), nbTestSamples)
    } else {
        totalSamplesSize
    };
    (*ctx).displayLevel = displayLevel;
    if totalSamplesSize
        < (if d as size_t > ::core::mem::size_of::<u64>() as size_t {
            d as size_t
        } else {
            ::core::mem::size_of::<u64>() as size_t
        })
        || totalSamplesSize
            >= (if ::core::mem::size_of::<size_t>() == 8 {
                -(1 as core::ffi::c_int) as core::ffi::c_uint
            } else {
                (1 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30)
            }) as size_t
    {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Total samples size is too large (%u MB), maximum size is %u MB\n\0" as *const u8
                    as *const core::ffi::c_char,
                (totalSamplesSize >> 20) as core::ffi::c_uint,
                (if ::core::mem::size_of::<size_t>() == 8 {
                    -(1 as core::ffi::c_int) as core::ffi::c_uint
                } else {
                    (1 as core::ffi::c_uint).wrapping_mul((1 as core::ffi::c_uint) << 30)
                }) >> 20,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if nbTrainSamples < 5 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Total number of training samples is %u and is invalid\n\0" as *const u8
                    as *const core::ffi::c_char,
                nbTrainSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if nbTestSamples < 1 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Total number of testing samples is %u and is invalid.\n\0" as *const u8
                    as *const core::ffi::c_char,
                nbTestSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ptr::write_bytes(ctx as *mut u8, 0, ::core::mem::size_of::<FASTCOVER_ctx_t>());
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Training on %u samples of total size %u\n\0" as *const u8 as *const core::ffi::c_char,
            nbTrainSamples,
            trainingSamplesSize as core::ffi::c_uint,
        );
        fflush(stderr);
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Testing on %u samples of total size %u\n\0" as *const u8 as *const core::ffi::c_char,
            nbTestSamples,
            testSamplesSize as core::ffi::c_uint,
        );
        fflush(stderr);
    }
    (*ctx).samples = samples;
    (*ctx).samplesSizes = samplesSizes;
    (*ctx).nbSamples = nbSamples as size_t;
    (*ctx).nbTrainSamples = nbTrainSamples as size_t;
    (*ctx).nbTestSamples = nbTestSamples as size_t;
    (*ctx).nbDmers = trainingSamplesSize
        .wrapping_sub(if d as size_t > ::core::mem::size_of::<u64>() as size_t {
            d as size_t
        } else {
            ::core::mem::size_of::<u64>() as size_t
        })
        .wrapping_add(1);
    (*ctx).d = d;
    (*ctx).f = f;
    (*ctx).accelParams = accelParams;
    (*ctx).offsets = calloc(
        nbSamples.wrapping_add(1) as size_t,
        ::core::mem::size_of::<size_t>() as size_t,
    ) as *mut size_t;
    if ((*ctx).offsets).is_null() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to allocate scratch buffers \n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    }
    let mut i: u32 = 0;
    *((*ctx).offsets).offset(0) = 0;
    i = 1;
    while i <= nbSamples {
        *((*ctx).offsets).offset(i as isize) = (*((*ctx).offsets)
            .offset(i.wrapping_sub(1) as isize))
        .wrapping_add(*samplesSizes.offset(i.wrapping_sub(1) as isize));
        i = i.wrapping_add(1);
    }
    (*ctx).freqs = calloc((1) << f, ::core::mem::size_of::<u32>() as size_t) as *mut u32;
    if ((*ctx).freqs).is_null() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to allocate frequency table \n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        FASTCOVER_ctx_destroy(ctx);
        return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Computing frequencies\n\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    FASTCOVER_computeFrequency((*ctx).freqs, ctx);
    0
}
unsafe fn FASTCOVER_buildDictionary(
    mut ctx: *const FASTCOVER_ctx_t,
    mut freqs: *mut u32,
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut parameters: ZDICT_cover_params_t,
    mut segmentFreqs: *mut u16,
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut tail = dictBufferCapacity;
    let epochs = COVER_computeEpochs(
        dictBufferCapacity as u32,
        (*ctx).nbDmers as u32,
        parameters.k,
        1,
    );
    let maxZeroScoreRun = 10;
    let displayLevel = (*ctx).displayLevel;
    let mut zeroScoreRun = 0 as size_t;
    let mut lastUpdateTime = 0;
    let mut epoch: size_t = 0;
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Breaking content into %u epochs of size %u\n\0" as *const u8
                as *const core::ffi::c_char,
            epochs.num,
            epochs.size,
        );
        fflush(stderr);
    }
    epoch = 0;
    while tail > 0 {
        let epochBegin = (epoch * epochs.size as size_t) as u32;
        let epochEnd = epochBegin.wrapping_add(epochs.size);
        let mut segmentSize: size_t = 0;
        let mut segment =
            FASTCOVER_selectSegment(ctx, freqs, epochBegin, epochEnd, parameters, segmentFreqs);
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
            memcpy(
                dict.add(tail) as *mut core::ffi::c_void,
                ((*ctx).samples).offset(segment.begin as isize) as *const core::ffi::c_void,
                segmentSize,
            );
            if displayLevel >= 2 {
                let refreshRate = CLOCKS_PER_SEC as __clock_t * 15 / 100;
                if clock() - lastUpdateTime > refreshRate || displayLevel >= 4 {
                    lastUpdateTime = clock();
                    fprintf(
                        stderr,
                        b"\r%u%%       \0" as *const u8 as *const core::ffi::c_char,
                        (dictBufferCapacity.wrapping_sub(tail) * 100 / dictBufferCapacity)
                            as core::ffi::c_uint,
                    );
                    fflush(stderr);
                }
            }
        }
        epoch = epoch.wrapping_add(1) % epochs.num as size_t;
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    tail
}
unsafe extern "C" fn FASTCOVER_tryParameters(mut opaque: *mut core::ffi::c_void) {
    let data = opaque as *mut FASTCOVER_tryParameters_data_t;
    let ctx = (*data).ctx;
    let parameters = (*data).parameters;
    let mut dictBufferCapacity = (*data).dictBufferCapacity;
    let mut totalCompressedSize = -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    let mut segmentFreqs =
        calloc((1) << (*ctx).f, ::core::mem::size_of::<u16>() as size_t) as *mut u16;
    let dict = malloc(dictBufferCapacity) as *mut u8;
    let mut selection =
        COVER_dictSelectionError(-(ZSTD_error_GENERIC as core::ffi::c_int) as size_t);
    let mut freqs =
        malloc(((1 as size_t) << (*ctx).f).wrapping_mul(::core::mem::size_of::<u32>() as size_t))
            as *mut u32;
    let displayLevel = (*ctx).displayLevel;
    if segmentFreqs.is_null() || dict.is_null() || freqs.is_null() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to allocate buffers: out of memory\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
    } else {
        memcpy(
            freqs as *mut core::ffi::c_void,
            (*ctx).freqs as *const core::ffi::c_void,
            ((1 as size_t) << (*ctx).f).wrapping_mul(::core::mem::size_of::<u32>() as size_t),
        );
        let tail = FASTCOVER_buildDictionary(
            ctx,
            freqs,
            dict as *mut core::ffi::c_void,
            dictBufferCapacity,
            parameters,
            segmentFreqs,
        );
        let nbFinalizeSamples = ((*ctx).nbTrainSamples * (*ctx).accelParams.finalize as size_t
            / 100) as core::ffi::c_uint;
        selection = COVER_selectDict(
            dict.add(tail),
            dictBufferCapacity,
            dictBufferCapacity.wrapping_sub(tail),
            (*ctx).samples,
            (*ctx).samplesSizes,
            nbFinalizeSamples,
            (*ctx).nbTrainSamples,
            (*ctx).nbSamples,
            parameters,
            (*ctx).offsets,
            totalCompressedSize,
        );
        if COVER_dictSelectionIsError(selection) != 0 && displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to select dictionary\n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
    }
    free(dict as *mut core::ffi::c_void);
    COVER_best_finish((*data).best, parameters, selection);
    free(data as *mut core::ffi::c_void);
    free(segmentFreqs as *mut core::ffi::c_void);
    COVER_dictSelectionFree(selection);
    free(freqs as *mut core::ffi::c_void);
}
unsafe fn FASTCOVER_convertToCoverParams(
    mut fastCoverParams: ZDICT_fastCover_params_t,
    mut coverParams: *mut ZDICT_cover_params_t,
) {
    (*coverParams).k = fastCoverParams.k;
    (*coverParams).d = fastCoverParams.d;
    (*coverParams).steps = fastCoverParams.steps;
    (*coverParams).nbThreads = fastCoverParams.nbThreads;
    (*coverParams).splitPoint = fastCoverParams.splitPoint;
    (*coverParams).zParams = fastCoverParams.zParams;
    (*coverParams).shrinkDict = fastCoverParams.shrinkDict;
}
unsafe fn FASTCOVER_convertToFastCoverParams(
    mut coverParams: ZDICT_cover_params_t,
    mut fastCoverParams: *mut ZDICT_fastCover_params_t,
    mut f: core::ffi::c_uint,
    mut accel: core::ffi::c_uint,
) {
    (*fastCoverParams).k = coverParams.k;
    (*fastCoverParams).d = coverParams.d;
    (*fastCoverParams).steps = coverParams.steps;
    (*fastCoverParams).nbThreads = coverParams.nbThreads;
    (*fastCoverParams).splitPoint = coverParams.splitPoint;
    (*fastCoverParams).f = f;
    (*fastCoverParams).accel = accel;
    (*fastCoverParams).zParams = coverParams.zParams;
    (*fastCoverParams).shrinkDict = coverParams.shrinkDict;
}
#[export_name = crate::prefix!(ZDICT_trainFromBuffer_fastCover)]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_fastCover(
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
    mut parameters: ZDICT_fastCover_params_t,
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut ctx = FASTCOVER_ctx_t {
        samples: core::ptr::null::<u8>(),
        offsets: core::ptr::null_mut::<size_t>(),
        samplesSizes: core::ptr::null::<size_t>(),
        nbSamples: 0,
        nbTrainSamples: 0,
        nbTestSamples: 0,
        nbDmers: 0,
        freqs: core::ptr::null_mut::<u32>(),
        d: 0,
        f: 0,
        accelParams: FASTCOVER_accel_t {
            finalize: 0,
            skip: 0,
        },
        displayLevel: 0,
    };
    let mut coverParams = ZDICT_cover_params_t {
        k: 0,
        d: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    let mut accelParams = FASTCOVER_accel_t {
        finalize: 0,
        skip: 0,
    };
    let displayLevel = parameters.zParams.notificationLevel as core::ffi::c_int;
    parameters.splitPoint = 1.0f64;
    parameters.f = if parameters.f == 0 {
        DEFAULT_F as core::ffi::c_uint
    } else {
        parameters.f
    };
    parameters.accel = if parameters.accel == 0 {
        DEFAULT_ACCEL as core::ffi::c_uint
    } else {
        parameters.accel
    };
    ptr::write_bytes(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
    FASTCOVER_convertToCoverParams(parameters, &mut coverParams);
    if FASTCOVER_checkParameters(
        coverParams,
        dictBufferCapacity,
        parameters.f,
        parameters.accel,
    ) == 0
    {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"FASTCOVER parameters incorrect\n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as core::ffi::c_int) as size_t;
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"FASTCOVER must have at least one input file\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const core::ffi::c_char,
                256,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    accelParams = *FASTCOVER_defaultAccelParameters
        .as_ptr()
        .offset(parameters.accel as isize);
    let initVal = FASTCOVER_ctx_init(
        &mut ctx,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        coverParams.d,
        parameters.splitPoint,
        parameters.f,
        accelParams,
        displayLevel,
    );
    if ERR_isError(initVal) != 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Failed to initialize context\n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Building dictionary\n\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    let mut segmentFreqs =
        calloc(1 << parameters.f, ::core::mem::size_of::<u16>() as size_t) as *mut u16;
    let tail = FASTCOVER_buildDictionary(
        &ctx,
        ctx.freqs,
        dictBuffer,
        dictBufferCapacity,
        coverParams,
        segmentFreqs,
    );
    let nbFinalizeSamples =
        (ctx.nbTrainSamples * ctx.accelParams.finalize as size_t / 100) as core::ffi::c_uint;
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut core::ffi::c_void,
        dictBufferCapacity,
        dict.add(tail) as *const core::ffi::c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes,
        nbFinalizeSamples,
        coverParams.zParams,
    );
    if ERR_isError(dictionarySize) == 0 && displayLevel >= 2 {
        fprintf(
            stderr,
            b"Constructed dictionary of size %u\n\0" as *const u8 as *const core::ffi::c_char,
            dictionarySize as core::ffi::c_uint,
        );
        fflush(stderr);
    }
    FASTCOVER_ctx_destroy(&mut ctx);
    free(segmentFreqs as *mut core::ffi::c_void);
    dictionarySize
}
#[export_name = crate::prefix!(ZDICT_optimizeTrainFromBuffer_fastCover)]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_fastCover(
    mut dictBuffer: *mut core::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const core::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: core::ffi::c_uint,
    mut parameters: *mut ZDICT_fastCover_params_t,
) -> size_t {
    let mut coverParams = ZDICT_cover_params_t {
        k: 0,
        d: 0,
        steps: 0,
        nbThreads: 0,
        splitPoint: 0.,
        shrinkDict: 0,
        shrinkDictMaxRegression: 0,
        zParams: ZDICT_params_t {
            compressionLevel: 0,
            notificationLevel: 0,
            dictID: 0,
        },
    };
    let mut accelParams = FASTCOVER_accel_t {
        finalize: 0,
        skip: 0,
    };
    let nbThreads = (*parameters).nbThreads;
    let splitPoint = if (*parameters).splitPoint <= 0.0f64 {
        FASTCOVER_DEFAULT_SPLITPOINT
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
    let f = if (*parameters).f == 0 {
        DEFAULT_F as core::ffi::c_uint
    } else {
        (*parameters).f
    };
    let accel = if (*parameters).accel == 0 {
        DEFAULT_ACCEL as core::ffi::c_uint
    } else {
        (*parameters).accel
    };
    let shrinkDict = 0;
    let displayLevel = (*parameters).zParams.notificationLevel as core::ffi::c_int;
    let mut iteration = 1 as core::ffi::c_uint;
    let mut d: core::ffi::c_uint = 0;
    let mut k: core::ffi::c_uint = 0;
    let mut best = COVER_best_s {
        mutex: PTHREAD_MUTEX_INITIALIZER,
        cond: PTHREAD_COND_INITIALIZER,
        liveJobs: 0,
        dict: core::ptr::null_mut::<core::ffi::c_void>(),
        dictSize: 0,
        parameters: ZDICT_cover_params_t {
            k: 0,
            d: 0,
            steps: 0,
            nbThreads: 0,
            splitPoint: 0.,
            shrinkDict: 0,
            shrinkDictMaxRegression: 0,
            zParams: ZDICT_params_t {
                compressionLevel: 0,
                notificationLevel: 0,
                dictID: 0,
            },
        },
        compressedSize: 0,
    };
    let mut pool = NULL as *mut POOL_ctx;
    let mut warned = 0;
    let mut lastUpdateTime = 0;
    if splitPoint <= 0.0 || splitPoint > 1.0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Incorrect splitPoint\n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as core::ffi::c_int) as size_t;
    }
    if accel == 0 || accel > FASTCOVER_MAX_ACCEL as core::ffi::c_uint {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Incorrect accel\n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as core::ffi::c_int) as size_t;
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Incorrect k\n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as core::ffi::c_int) as size_t;
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"FASTCOVER must have at least one input file\n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const core::ffi::c_char,
                256,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if nbThreads > 1 {
        pool = POOL_create(nbThreads as size_t, 1);
        if pool.is_null() {
            return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
        }
    }
    COVER_best_init(&mut best);
    ptr::write_bytes(
        &mut coverParams as *mut ZDICT_cover_params_t as *mut u8,
        0,
        ::core::mem::size_of::<ZDICT_cover_params_t>(),
    );
    FASTCOVER_convertToCoverParams(*parameters, &mut coverParams);
    accelParams = *FASTCOVER_defaultAccelParameters
        .as_ptr()
        .offset(accel as isize);
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"Trying %u different sets of parameters\n\0" as *const u8 as *const core::ffi::c_char,
            kIterations,
        );
        fflush(stderr);
    }
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = FASTCOVER_ctx_t {
            samples: core::ptr::null::<u8>(),
            offsets: core::ptr::null_mut::<size_t>(),
            samplesSizes: core::ptr::null::<size_t>(),
            nbSamples: 0,
            nbTrainSamples: 0,
            nbTestSamples: 0,
            nbDmers: 0,
            freqs: core::ptr::null_mut::<u32>(),
            d: 0,
            f: 0,
            accelParams: FASTCOVER_accel_t {
                finalize: 0,
                skip: 0,
            },
            displayLevel: 0,
        };
        if displayLevel >= 3 {
            fprintf(
                stderr,
                b"d=%u\n\0" as *const u8 as *const core::ffi::c_char,
                d,
            );
            fflush(stderr);
        }
        let childDisplayLevel = if displayLevel == 0 {
            0
        } else {
            displayLevel - 1
        };
        let initVal = FASTCOVER_ctx_init(
            &mut ctx,
            samplesBuffer,
            samplesSizes,
            nbSamples,
            d,
            splitPoint,
            f,
            accelParams,
            childDisplayLevel,
        );
        if ERR_isError(initVal) != 0 {
            if displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Failed to initialize context\n\0" as *const u8 as *const core::ffi::c_char,
                );
                fflush(stderr);
            }
            COVER_best_destroy(&mut best);
            POOL_free(pool);
            return initVal;
        }
        if warned == 0 {
            COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
            warned = 1;
        }
        k = kMinK;
        while k <= kMaxK {
            let mut data = malloc(::core::mem::size_of::<FASTCOVER_tryParameters_data_t>() as size_t)
                as *mut FASTCOVER_tryParameters_data_t;
            if displayLevel >= 3 {
                fprintf(
                    stderr,
                    b"k=%u\n\0" as *const u8 as *const core::ffi::c_char,
                    k,
                );
                fflush(stderr);
            }
            if data.is_null() {
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Failed to allocate parameters\n\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    fflush(stderr);
                }
                COVER_best_destroy(&mut best);
                FASTCOVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return -(ZSTD_error_memory_allocation as core::ffi::c_int) as size_t;
            }
            (*data).ctx = &mut ctx;
            (*data).best = &mut best;
            (*data).dictBufferCapacity = dictBufferCapacity;
            (*data).parameters = coverParams;
            (*data).parameters.k = k;
            (*data).parameters.d = d;
            (*data).parameters.splitPoint = splitPoint;
            (*data).parameters.steps = kSteps;
            (*data).parameters.shrinkDict = shrinkDict;
            (*data).parameters.zParams.notificationLevel = ctx.displayLevel as core::ffi::c_uint;
            if FASTCOVER_checkParameters(
                (*data).parameters,
                dictBufferCapacity,
                (*(*data).ctx).f,
                accel,
            ) == 0
            {
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"FASTCOVER parameters incorrect\n\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    fflush(stderr);
                }
                free(data as *mut core::ffi::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(
                        pool,
                        Some(
                            FASTCOVER_tryParameters
                                as unsafe extern "C" fn(*mut core::ffi::c_void) -> (),
                        ),
                        data as *mut core::ffi::c_void,
                    );
                } else {
                    FASTCOVER_tryParameters(data as *mut core::ffi::c_void);
                }
                if displayLevel >= 2 {
                    let refreshRate = CLOCKS_PER_SEC as __clock_t * 15 / 100;
                    if clock() - lastUpdateTime > refreshRate || displayLevel >= 4 {
                        lastUpdateTime = clock();
                        fprintf(
                            stderr,
                            b"\r%u%%       \0" as *const u8 as *const core::ffi::c_char,
                            iteration.wrapping_mul(100).wrapping_div(kIterations),
                        );
                        fflush(stderr);
                    }
                }
                iteration = iteration.wrapping_add(1);
            }
            k = k.wrapping_add(kStepSize);
        }
        COVER_best_wait(&mut best);
        FASTCOVER_ctx_destroy(&mut ctx);
        d = d.wrapping_add(2);
    }
    if displayLevel >= 2 {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(stderr);
    }
    let dictSize = best.dictSize;
    if ERR_isError(best.compressedSize) != 0 {
        let compressedSize = best.compressedSize;
        COVER_best_destroy(&mut best);
        POOL_free(pool);
        return compressedSize;
    }
    FASTCOVER_convertToFastCoverParams(best.parameters, parameters, f, accel);
    memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    dictSize
}
