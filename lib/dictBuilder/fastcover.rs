use core::ptr;

use libc::{free, malloc, memcpy, size_t};

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::pool::{POOL_add, POOL_create, POOL_free};
use crate::lib::compress::zstd_compress_internal::{
    ZSTD_hash6Ptr, ZSTD_hash6Ptr_array, ZSTD_hash8Ptr, ZSTD_hash8Ptr_array,
};
use crate::lib::dictBuilder::cover::{
    COVER_best_finish, COVER_best_start, COVER_best_t, COVER_best_wait, COVER_computeEpochs,
    COVER_dictSelectionError, COVER_dictSelectionFree, COVER_dictSelectionIsError, COVER_segment_t,
    COVER_selectDict, COVER_warnOnSmallCorpus,
};
use crate::lib::zdict::experimental::{
    ZDICT_cover_params_t, ZDICT_fastCover_params_t, ZDICT_DICTSIZE_MIN,
};
use crate::lib::zdict::ZDICT_finalizeDictionary;

extern "C" {
    fn clock() -> clock_t;
}
type __clock_t = core::ffi::c_long;
type clock_t = __clock_t;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
struct FASTCOVER_accel_t {
    finalize: core::ffi::c_uint,
    skip: core::ffi::c_uint,
}

#[repr(C)]
#[derive(Debug, Default)]
struct FASTCOVER_ctx_t<'a> {
    samples: &'a [u8],
    offsets: Box<[size_t]>,
    samplesSizes: &'a [size_t],
    nbSamples: size_t,
    nbTrainSamples: size_t,
    nbTestSamples: size_t,
    nbDmers: size_t,
    freqs: Box<[u32]>,
    d: core::ffi::c_uint,
    f: core::ffi::c_uint,
    accelParams: FASTCOVER_accel_t,
    displayLevel: core::ffi::c_int,
}

#[repr(C)]
struct FASTCOVER_tryParameters_data_t<'a> {
    ctx: *const FASTCOVER_ctx_t<'a>,
    best: *mut COVER_best_t,
    dictBufferCapacity: size_t,
    parameters: ZDICT_cover_params_t,
}

const CLOCKS_PER_SEC: core::ffi::c_int = 1000000;
const FASTCOVER_MAX_F: core::ffi::c_int = 31;
const FASTCOVER_MAX_ACCEL: core::ffi::c_int = 10;
const FASTCOVER_DEFAULT_SPLITPOINT: core::ffi::c_double = 0.75f64;
const DEFAULT_F: core::ffi::c_int = 20;
const DEFAULT_ACCEL: core::ffi::c_int = 1;

unsafe fn FASTCOVER_hashPtrToIndex(
    p: *const core::ffi::c_void,
    f: u32,
    d: core::ffi::c_uint,
) -> size_t {
    match d {
        6 => ZSTD_hash6Ptr(p, f),
        _ => ZSTD_hash8Ptr(p, f),
    }
}

fn FASTCOVER_hashPtrToIndex_array(p: &[u8; 8], f: u32, d: core::ffi::c_uint) -> size_t {
    match d {
        6 => ZSTD_hash6Ptr_array(p, f),
        _ => ZSTD_hash8Ptr_array(p, f),
    }
}

static FASTCOVER_defaultAccelParameters: [FASTCOVER_accel_t; 11] = {
    const fn accel(finalize: core::ffi::c_uint, skip: core::ffi::c_uint) -> FASTCOVER_accel_t {
        FASTCOVER_accel_t { finalize, skip }
    }

    [
        accel(100, 0),
        accel(100, 0),
        accel(50, 1),
        accel(34, 2),
        accel(25, 3),
        accel(20, 4),
        accel(17, 5),
        accel(14, 6),
        accel(13, 7),
        accel(11, 8),
        accel(10, 9),
    ]
};

unsafe fn FASTCOVER_selectSegment(
    ctx: *const FASTCOVER_ctx_t,
    freqs: &mut [u32],
    begin: u32,
    end: u32,
    parameters: ZDICT_cover_params_t,
    segmentFreqs: &mut [u16],
) -> COVER_segment_t {
    let k = parameters.k;
    let d = parameters.d;
    let samples = (*ctx).samples;
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
            (*ctx).samples.as_ptr().offset(activeSegment.end as isize) as *const core::ffi::c_void,
            f,
            d,
        );
        if segmentFreqs[idx] == 0 {
            activeSegment.score = (activeSegment.score).wrapping_add(freqs[idx]);
        }
        activeSegment.end = (activeSegment.end).wrapping_add(1);
        segmentFreqs[idx] += 1;
        if (activeSegment.end).wrapping_sub(activeSegment.begin) == dmersInK.wrapping_add(1) {
            let delIndex = FASTCOVER_hashPtrToIndex_array(
                samples[activeSegment.begin as usize..][..8]
                    .try_into()
                    .unwrap(),
                f,
                d,
            );
            segmentFreqs[delIndex] -= 1;
            if segmentFreqs[delIndex] == 0 {
                activeSegment.score = (activeSegment.score).wrapping_sub(freqs[delIndex]);
            }
            activeSegment.begin = (activeSegment.begin).wrapping_add(1);
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment;
        }
    }
    while activeSegment.begin < end {
        let delIndex_0 = FASTCOVER_hashPtrToIndex_array(
            samples[activeSegment.begin as usize..][..8]
                .try_into()
                .unwrap(),
            f,
            d,
        );
        segmentFreqs[delIndex_0] -= 1;
        activeSegment.begin += 1;
    }
    let mut pos: u32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let i =
            FASTCOVER_hashPtrToIndex_array(samples[pos as usize..][..8].try_into().unwrap(), f, d);
        freqs[i] = 0;
        pos = pos.wrapping_add(1);
    }
    bestSegment
}

fn FASTCOVER_checkParameters(
    parameters: ZDICT_cover_params_t,
    maxDictSize: size_t,
    f: core::ffi::c_uint,
    accel: core::ffi::c_uint,
) -> bool {
    if parameters.d == 0 || parameters.k == 0 {
        return false;
    }
    if parameters.d != 6 && parameters.d != 8 {
        return false;
    }
    if parameters.k as size_t > maxDictSize {
        return false;
    }
    if parameters.d > parameters.k {
        return false;
    }
    if f > FASTCOVER_MAX_F as core::ffi::c_uint || f == 0 {
        return false;
    }
    if parameters.splitPoint <= 0.0 || parameters.splitPoint > 1.0 {
        return false;
    }
    if accel > 10 || accel == 0 {
        return false;
    }

    true
}

unsafe fn FASTCOVER_ctx_destroy(ctx: *mut FASTCOVER_ctx_t) {
    if ctx.is_null() {
        return;
    }
    drop(core::mem::take(&mut ((*ctx).freqs)));
    drop(core::mem::take(&mut ((*ctx).offsets)));
}

fn FASTCOVER_computeFrequency(ctx: &mut FASTCOVER_ctx_t) {
    let f = ctx.f;
    let d = ctx.d;
    let skip = ctx.accelParams.skip;
    let readLength = if d > 8 { d } else { 8 };
    let mut i: size_t = 0;
    i = 0;
    while i < ctx.nbTrainSamples {
        let mut start = ctx.offsets[i];
        let currSampleEnd = ctx.offsets[i + 1];
        while start.wrapping_add(readLength as size_t) <= currSampleEnd {
            let dmerIndex =
                FASTCOVER_hashPtrToIndex_array(ctx.samples[start..][..8].try_into().unwrap(), f, d);
            ctx.freqs[dmerIndex] += 1;
            start = start.wrapping_add(skip as size_t).wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}

fn FASTCOVER_ctx_init<'a>(
    ctx: &mut FASTCOVER_ctx_t<'a>,
    samples: &'a [u8],
    samplesSizes: &'a [size_t],
    nbSamples: core::ffi::c_uint,
    d: core::ffi::c_uint,
    splitPoint: core::ffi::c_double,
    f: core::ffi::c_uint,
    accelParams: FASTCOVER_accel_t,
    displayLevel: core::ffi::c_int,
) -> size_t {
    let totalSamplesSize = samplesSizes.iter().sum::<usize>();
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

    const GB: usize = (1) << 30;
    const FASTCOVER_MAX_SAMPLES_SIZE: usize = match usize::BITS {
        64 => 4 * GB,
        _ => GB,
    };

    ctx.displayLevel = displayLevel;
    if totalSamplesSize < Ord::max(d as size_t, ::core::mem::size_of::<u64>())
        || totalSamplesSize >= FASTCOVER_MAX_SAMPLES_SIZE
    {
        if displayLevel >= 1 {
            eprintln!(
                "Total samples size is too large ({} MB), maximum size is {} MB",
                totalSamplesSize >> 20,
                FASTCOVER_MAX_SAMPLES_SIZE >> 20,
            );
        }
        return Error::srcSize_wrong.to_error_code();
    }
    if nbTrainSamples < 5 {
        if displayLevel >= 1 {
            eprintln!(
                "Total number of training samples is {} and is invalid",
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
    ctx.nbDmers =
        trainingSamplesSize.wrapping_sub(Ord::max(d as size_t, ::core::mem::size_of::<u64>())) + 1;
    ctx.d = d;
    ctx.f = f;
    ctx.accelParams = accelParams;
    ctx.offsets = Box::from(vec![0usize; nbSamples as usize + 1]);

    for i in 1..nbSamples as usize + 1 {
        ctx.offsets[i] = ctx.offsets[i - 1] + samplesSizes[i - 1];
    }

    ctx.freqs = Box::from(vec![0u32; 1 << f]);
    if displayLevel >= 2 {
        eprintln!("Computing frequencies");
    }
    FASTCOVER_computeFrequency(ctx);
    0
}

unsafe fn FASTCOVER_buildDictionary(
    ctx: &FASTCOVER_ctx_t,
    freqs: &mut [u32],
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    parameters: ZDICT_cover_params_t,
    segmentFreqs: &mut [u16],
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut tail = dictBufferCapacity;
    let epochs = COVER_computeEpochs(
        dictBufferCapacity as u32,
        ctx.nbDmers as u32,
        parameters.k,
        1,
    );
    let maxZeroScoreRun = 10;
    let displayLevel = ctx.displayLevel;
    let mut zeroScoreRun = 0 as size_t;
    let mut lastUpdateTime = 0;
    let mut epoch: size_t = 0;
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
                ctx.samples.as_ptr().offset(segment.begin as isize) as *const core::ffi::c_void,
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
unsafe fn FASTCOVER_tryParameters(opaque: *mut core::ffi::c_void) {
    let data = opaque as *mut FASTCOVER_tryParameters_data_t;
    let ctx = (*data).ctx.cast_mut().as_mut().unwrap();
    let parameters = (*data).parameters;
    let dictBufferCapacity = (*data).dictBufferCapacity;
    let totalCompressedSize = Error::GENERIC.to_error_code();
    let mut segmentFreqs: Box<[u16]> = Box::from(vec![0u16; 1 << ctx.f]);
    let mut dict: Box<[u8]> = Box::from(vec![0; dictBufferCapacity]);
    let mut selection = COVER_dictSelectionError(Error::GENERIC.to_error_code());

    let displayLevel = ctx.displayLevel;
    let mut freqs = ctx.freqs.clone();

    let tail = FASTCOVER_buildDictionary(
        ctx,
        &mut freqs,
        dict.as_mut_ptr() as *mut core::ffi::c_void,
        dictBufferCapacity,
        parameters,
        &mut segmentFreqs,
    );
    let nbFinalizeSamples =
        (ctx.nbTrainSamples * ctx.accelParams.finalize as size_t / 100) as core::ffi::c_uint;
    selection = COVER_selectDict(
        &dict[tail..],
        dictBufferCapacity,
        dictBufferCapacity.wrapping_sub(tail),
        ctx.samples,
        ctx.samplesSizes,
        nbFinalizeSamples,
        ctx.nbTrainSamples,
        ctx.nbSamples,
        parameters,
        &ctx.offsets,
        totalCompressedSize,
    );
    if COVER_dictSelectionIsError(&selection) && displayLevel >= 1 {
        eprintln!("Failed to select dictionary");
    }

    drop(dict);
    COVER_best_finish((*data).best.as_mut().unwrap(), parameters, &selection);
    free(data as *mut core::ffi::c_void);
    drop(segmentFreqs);
    COVER_dictSelectionFree(selection);
    drop(freqs);
}

fn FASTCOVER_convertToCoverParams(
    fastCoverParams: ZDICT_fastCover_params_t,
    coverParams: &mut ZDICT_cover_params_t,
) {
    coverParams.k = fastCoverParams.k;
    coverParams.d = fastCoverParams.d;
    coverParams.steps = fastCoverParams.steps;
    coverParams.nbThreads = fastCoverParams.nbThreads;
    coverParams.splitPoint = fastCoverParams.splitPoint;
    coverParams.zParams = fastCoverParams.zParams;
    coverParams.shrinkDict = fastCoverParams.shrinkDict;
}

unsafe fn FASTCOVER_convertToFastCoverParams(
    coverParams: ZDICT_cover_params_t,
    fastCoverParams: *mut ZDICT_fastCover_params_t,
    f: core::ffi::c_uint,
    accel: core::ffi::c_uint,
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
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_trainFromBuffer_fastCover))]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_fastCover(
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    mut parameters: ZDICT_fastCover_params_t,
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut ctx = FASTCOVER_ctx_t::default();
    let mut coverParams = ZDICT_cover_params_t::default();
    let mut accelParams = FASTCOVER_accel_t::default();
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
    if !FASTCOVER_checkParameters(
        coverParams,
        dictBufferCapacity,
        parameters.f,
        parameters.accel,
    ) {
        if displayLevel >= 1 {
            eprintln!("FASTCOVER parameters incorrect");
        }
        return Error::parameter_outOfBound.to_error_code();
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            eprintln!("FASTCOVER must have at least one input file");
        }
        return Error::srcSize_wrong.to_error_code();
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        if displayLevel >= 1 {
            eprintln!("dictBufferCapacity must be at least {}", 256);
        }
        return Error::dstSize_tooSmall.to_error_code();
    }
    accelParams = *FASTCOVER_defaultAccelParameters
        .as_ptr()
        .offset(parameters.accel as isize);

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

    let initVal = FASTCOVER_ctx_init(
        &mut ctx,
        samples,
        samplesSizes,
        nbSamples,
        coverParams.d,
        parameters.splitPoint,
        parameters.f,
        accelParams,
        displayLevel,
    );
    if ERR_isError(initVal) {
        if displayLevel >= 1 {
            eprintln!("Failed to initialize context");
        }
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
    if displayLevel >= 2 {
        eprintln!("Building dictionary");
    }
    let mut segmentFreqs: Box<[u16]> = Box::from(vec![0u16; 1 << parameters.f]);

    let mut freqs = core::mem::take(&mut ctx.freqs);
    let tail = FASTCOVER_buildDictionary(
        &ctx,
        &mut freqs,
        dictBuffer,
        dictBufferCapacity,
        coverParams,
        &mut segmentFreqs,
    );
    ctx.freqs = freqs;

    let nbFinalizeSamples =
        (ctx.nbTrainSamples * ctx.accelParams.finalize as size_t / 100) as core::ffi::c_uint;
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut core::ffi::c_void,
        dictBufferCapacity,
        dict.add(tail) as *const core::ffi::c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes.as_ptr(),
        nbFinalizeSamples,
        coverParams.zParams,
    );
    if !ERR_isError(dictionarySize) && displayLevel >= 2 {
        eprintln!("Constructed dictionary of size {}", dictionarySize);
    }
    FASTCOVER_ctx_destroy(&mut ctx);
    drop(segmentFreqs);
    dictionarySize
}
#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZDICT_optimizeTrainFromBuffer_fastCover))]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_fastCover(
    dictBuffer: *mut core::ffi::c_void,
    dictBufferCapacity: size_t,
    samplesBuffer: *const core::ffi::c_void,
    samplesSizes: *const size_t,
    nbSamples: core::ffi::c_uint,
    parameters: *mut ZDICT_fastCover_params_t,
) -> size_t {
    let mut coverParams = ZDICT_cover_params_t::default();
    let mut accelParams = FASTCOVER_accel_t::default();
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
    let mut pool = core::ptr::null_mut();
    let mut warned = 0;
    let mut lastUpdateTime = 0;
    if splitPoint <= 0.0 || splitPoint > 1.0 {
        if displayLevel >= 1 {
            eprintln!("Incorrect splitPoint");
        }
        return Error::parameter_outOfBound.to_error_code();
    }
    if accel == 0 || accel > FASTCOVER_MAX_ACCEL as core::ffi::c_uint {
        if displayLevel >= 1 {
            eprintln!("Incorrect accel");
        }
        return Error::parameter_outOfBound.to_error_code();
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1 {
            eprintln!("Incorrect k");
        }
        return Error::parameter_outOfBound.to_error_code();
    }
    if nbSamples == 0 {
        if displayLevel >= 1 {
            eprintln!("FASTCOVER must have at least one input file");
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
    let mut best = COVER_best_t::new();
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
        eprintln!("Trying {} different sets of parameters", kIterations);
    }
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = FASTCOVER_ctx_t::default();
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

        let initVal = FASTCOVER_ctx_init(
            &mut ctx,
            samples,
            samplesSizes,
            nbSamples,
            d,
            splitPoint,
            f,
            accelParams,
            childDisplayLevel,
        );
        if ERR_isError(initVal) {
            if displayLevel >= 1 {
                eprintln!("Failed to initialize context");
            }
            drop(COVER_best_wait(&best));
            POOL_free(pool);
            return initVal;
        }
        if warned == 0 {
            COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.nbDmers, displayLevel);
            warned = 1;
        }
        k = kMinK;
        while k <= kMaxK {
            let data = malloc(::core::mem::size_of::<FASTCOVER_tryParameters_data_t>())
                as *mut FASTCOVER_tryParameters_data_t;
            if displayLevel >= 3 {
                eprintln!("k={}", k);
            }
            if data.is_null() {
                if displayLevel >= 1 {
                    eprintln!("Failed to allocate parameters");
                }
                drop(COVER_best_wait(&best));
                FASTCOVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return Error::memory_allocation.to_error_code();
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
            if !FASTCOVER_checkParameters(
                (*data).parameters,
                dictBufferCapacity,
                (*(*data).ctx).f,
                accel,
            ) {
                if displayLevel >= 1 {
                    eprintln!("FASTCOVER parameters incorrect");
                }
                free(data as *mut core::ffi::c_void);
            } else {
                COVER_best_start(&best);
                if !pool.is_null() {
                    POOL_add(
                        pool,
                        FASTCOVER_tryParameters,
                        data as *mut core::ffi::c_void,
                    );
                } else {
                    FASTCOVER_tryParameters(data as *mut core::ffi::c_void);
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
        drop(COVER_best_wait(&best));
        FASTCOVER_ctx_destroy(&mut ctx);
        d = d.wrapping_add(2);
    }
    if displayLevel >= 2 {
        println!("\r{:79 }\r", "");
    }

    let best = COVER_best_wait(&best);

    let dictSize = best.dictSize;
    if ERR_isError(best.compressedSize) {
        let compressedSize = best.compressedSize;
        POOL_free(pool);
        return compressedSize;
    }
    FASTCOVER_convertToFastCoverParams(best.parameters, parameters, f, accel);
    memcpy(dictBuffer, best.dict.as_ptr().cast(), dictSize);
    POOL_free(pool);
    dictSize
}
