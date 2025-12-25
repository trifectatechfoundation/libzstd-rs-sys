use core::ptr;

use libc::{abort, free, malloc, size_t};

use crate::timefn::{UTIL_clockSpanNano, UTIL_getTime, UTIL_time_t};

pub type PTime = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_runTime_t {
    pub nanoSecPerRun: core::ffi::c_double,
    pub sumOfReturn: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_runOutcome_t {
    pub internal_never_ever_use_directly: BMK_runTime_t,
    pub error_result_never_ever_use_directly: size_t,
    pub error_tag_never_ever_use_directly: core::ffi::c_int,
}
pub type BMK_benchFn_t = Option<
    unsafe fn(
        *const core::ffi::c_void,
        size_t,
        *mut core::ffi::c_void,
        size_t,
        *mut core::ffi::c_void,
    ) -> size_t,
>;
pub type BMK_initFn_t = Option<unsafe fn(*mut core::ffi::c_void) -> size_t>;
pub type BMK_errorFn_t = Option<unsafe extern "C" fn(size_t) -> core::ffi::c_uint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_benchParams_t {
    pub benchFn: BMK_benchFn_t,
    pub benchPayload: *mut core::ffi::c_void,
    pub initFn: BMK_initFn_t,
    pub initPayload: *mut core::ffi::c_void,
    pub errorFn: BMK_errorFn_t,
    pub blockCount: size_t,
    pub srcBuffers: *const *const core::ffi::c_void,
    pub srcSizes: *const size_t,
    pub dstBuffers: *const *mut core::ffi::c_void,
    pub dstCapacities: *const size_t,
    pub blockResults: *mut size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_timedFnState_s {
    pub timeSpent_ns: PTime,
    pub timeBudget_ns: PTime,
    pub runBudget_ns: PTime,
    pub fastestRun: BMK_runTime_t,
    pub nbLoops: core::ffi::c_uint,
    pub coolTime: UTIL_time_t,
}
pub type BMK_timedFnState_t = BMK_timedFnState_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfs_align {
    pub c: check_size,
    pub tfs: BMK_timedFnState_t,
}
pub type check_size = [core::ffi::c_char; 1];
pub const TIMELOOP_NANOSEC: core::ffi::c_ulonglong =
    (1 as core::ffi::c_ulonglong).wrapping_mul(1000000000);
pub unsafe fn BMK_isSuccessful_runOutcome(outcome: BMK_runOutcome_t) -> core::ffi::c_int {
    core::ffi::c_int::from(outcome.error_tag_never_ever_use_directly == 0)
}
pub unsafe fn BMK_extract_runTime(outcome: BMK_runOutcome_t) -> BMK_runTime_t {
    if outcome.error_tag_never_ever_use_directly != 0 {
        abort();
    }
    outcome.internal_never_ever_use_directly
}
pub unsafe fn BMK_extract_errorResult(outcome: BMK_runOutcome_t) -> size_t {
    if outcome.error_tag_never_ever_use_directly == 0 {
        abort();
    }
    outcome.error_result_never_ever_use_directly
}
unsafe fn BMK_runOutcome_error(errorResult: size_t) -> BMK_runOutcome_t {
    let mut b = BMK_runOutcome_t {
        internal_never_ever_use_directly: BMK_runTime_t {
            nanoSecPerRun: 0.,
            sumOfReturn: 0,
        },
        error_result_never_ever_use_directly: 0,
        error_tag_never_ever_use_directly: 0,
    };
    ptr::write_bytes(
        &mut b as *mut BMK_runOutcome_t as *mut u8,
        0,
        ::core::mem::size_of::<BMK_runOutcome_t>(),
    );
    b.error_tag_never_ever_use_directly = 1;
    b.error_result_never_ever_use_directly = errorResult;
    b
}
unsafe fn BMK_setValid_runTime(runTime: BMK_runTime_t) -> BMK_runOutcome_t {
    let mut outcome = BMK_runOutcome_t {
        internal_never_ever_use_directly: BMK_runTime_t {
            nanoSecPerRun: 0.,
            sumOfReturn: 0,
        },
        error_result_never_ever_use_directly: 0,
        error_tag_never_ever_use_directly: 0,
    };
    outcome.error_tag_never_ever_use_directly = 0;
    outcome.internal_never_ever_use_directly = runTime;
    outcome
}
pub unsafe fn BMK_benchFunction(
    p: BMK_benchParams_t,
    mut nbLoops: core::ffi::c_uint,
) -> BMK_runOutcome_t {
    nbLoops = nbLoops.wrapping_add(core::ffi::c_int::from(nbLoops == 0) as core::ffi::c_uint);
    let mut i: size_t = 0;
    i = 0;
    while i < p.blockCount {
        core::ptr::write_bytes(*(p.dstBuffers).add(i), 0xe5, *(p.dstCapacities).add(i));
        i = i.wrapping_add(1);
    }
    let mut dstSize = 0 as size_t;
    let clockStart = UTIL_getTime();
    let mut loopNb: core::ffi::c_uint = 0;
    let mut blockNb: core::ffi::c_uint = 0;
    if (p.initFn).is_some() {
        (p.initFn).unwrap_unchecked()(p.initPayload);
    }
    loopNb = 0;
    while loopNb < nbLoops {
        blockNb = 0;
        while (blockNb as size_t) < p.blockCount {
            let res = (p.benchFn).unwrap_unchecked()(
                *(p.srcBuffers).offset(blockNb as isize),
                *(p.srcSizes).offset(blockNb as isize),
                *(p.dstBuffers).offset(blockNb as isize),
                *(p.dstCapacities).offset(blockNb as isize),
                p.benchPayload,
            );
            if loopNb == 0 {
                if !(p.blockResults).is_null() {
                    *(p.blockResults).offset(blockNb as isize) = res;
                }
                if (p.errorFn).is_some() && (p.errorFn).unwrap_unchecked()(res) != 0 {
                    return BMK_runOutcome_error(res);
                }
                dstSize = dstSize.wrapping_add(res);
            }
            blockNb = blockNb.wrapping_add(1);
        }
        loopNb = loopNb.wrapping_add(1);
    }
    let totalTime = UTIL_clockSpanNano(clockStart);
    let mut rt = BMK_runTime_t {
        nanoSecPerRun: 0.,
        sumOfReturn: 0,
    };
    rt.nanoSecPerRun = totalTime as core::ffi::c_double / core::ffi::c_double::from(nbLoops);
    rt.sumOfReturn = dstSize;
    BMK_setValid_runTime(rt)
}
pub unsafe fn BMK_createTimedFnState(
    total_ms: core::ffi::c_uint,
    run_ms: core::ffi::c_uint,
) -> *mut BMK_timedFnState_t {
    let r = malloc(::core::mem::size_of::<BMK_timedFnState_t>()) as *mut BMK_timedFnState_t;
    if r.is_null() {
        return core::ptr::null_mut();
    }
    BMK_resetTimedFnState(r, total_ms, run_ms);
    r
}
pub unsafe fn BMK_freeTimedFnState(state: *mut BMK_timedFnState_t) {
    free(state as *mut core::ffi::c_void);
}
pub unsafe fn BMK_initStatic_timedFnState(
    buffer: *mut core::ffi::c_void,
    size: size_t,
    total_ms: core::ffi::c_uint,
    run_ms: core::ffi::c_uint,
) -> *mut BMK_timedFnState_t {
    let tfs_alignment = 8;
    let r = buffer as *mut BMK_timedFnState_t;
    if buffer.is_null() {
        return core::ptr::null_mut();
    }
    if size < ::core::mem::size_of::<BMK_timedFnState_s>() {
        return core::ptr::null_mut();
    }
    if !(buffer as size_t).is_multiple_of(tfs_alignment) {
        return core::ptr::null_mut();
    }
    BMK_resetTimedFnState(r, total_ms, run_ms);
    r
}
pub unsafe fn BMK_resetTimedFnState(
    timedFnState: *mut BMK_timedFnState_t,
    mut total_ms: core::ffi::c_uint,
    mut run_ms: core::ffi::c_uint,
) {
    if total_ms == 0 {
        total_ms = 1;
    }
    if run_ms == 0 {
        run_ms = 1;
    }
    if run_ms > total_ms {
        run_ms = total_ms;
    }
    (*timedFnState).timeSpent_ns = 0;
    (*timedFnState).timeBudget_ns = (PTime::from(total_ms) as core::ffi::c_ulonglong)
        .wrapping_mul(TIMELOOP_NANOSEC)
        .wrapping_div(1000) as PTime;
    (*timedFnState).runBudget_ns = (PTime::from(run_ms) as core::ffi::c_ulonglong)
        .wrapping_mul(TIMELOOP_NANOSEC)
        .wrapping_div(1000) as PTime;
    (*timedFnState).fastestRun.nanoSecPerRun =
        TIMELOOP_NANOSEC as core::ffi::c_double * 2000000000.0;
    (*timedFnState).fastestRun.sumOfReturn = -(1 as core::ffi::c_longlong) as size_t;
    (*timedFnState).nbLoops = 1;
    (*timedFnState).coolTime = UTIL_getTime();
}
pub unsafe fn BMK_isCompleted_TimedFn(timedFnState: *const BMK_timedFnState_t) -> core::ffi::c_int {
    core::ffi::c_int::from((*timedFnState).timeSpent_ns >= (*timedFnState).timeBudget_ns)
}
pub unsafe fn BMK_benchTimedFn(
    cont: *mut BMK_timedFnState_t,
    p: BMK_benchParams_t,
) -> BMK_runOutcome_t {
    let runBudget_ns = (*cont).runBudget_ns;
    let runTimeMin_ns = runBudget_ns / 2;
    let mut completed = 0;
    let mut bestRunTime = (*cont).fastestRun;
    while completed == 0 {
        let runResult = BMK_benchFunction(p, (*cont).nbLoops);
        if BMK_isSuccessful_runOutcome(runResult) == 0 {
            return runResult;
        }
        let newRunTime = BMK_extract_runTime(runResult);
        let loopDuration_ns = newRunTime.nanoSecPerRun * core::ffi::c_double::from((*cont).nbLoops);
        (*cont).timeSpent_ns = ((*cont).timeSpent_ns as core::ffi::c_ulonglong)
            .wrapping_add(loopDuration_ns as core::ffi::c_ulonglong)
            as PTime as PTime;
        if loopDuration_ns > runBudget_ns as core::ffi::c_double / 50.0 {
            let fastestRun_ns = if bestRunTime.nanoSecPerRun < newRunTime.nanoSecPerRun {
                bestRunTime.nanoSecPerRun
            } else {
                newRunTime.nanoSecPerRun
            };
            (*cont).nbLoops = ((runBudget_ns as core::ffi::c_double / fastestRun_ns)
                as core::ffi::c_uint)
                .wrapping_add(1);
        } else {
            let multiplier = 10;
            assert!(
                (*cont).nbLoops
                    < (-(1 as core::ffi::c_int) as core::ffi::c_uint).wrapping_div(multiplier)
            );
            (*cont).nbLoops = ((*cont).nbLoops).wrapping_mul(multiplier);
        }
        if loopDuration_ns < runTimeMin_ns as core::ffi::c_double {
            assert!(completed == 0);
        } else {
            if newRunTime.nanoSecPerRun < bestRunTime.nanoSecPerRun {
                bestRunTime = newRunTime;
            }
            completed = 1;
        }
    }
    BMK_setValid_runTime(bestRunTime)
}
