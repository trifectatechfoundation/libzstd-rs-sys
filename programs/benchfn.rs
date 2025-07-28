extern "C" {
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn __assert_fail(
        __assertion: *const std::ffi::c_char,
        __file: *const std::ffi::c_char,
        __line: std::ffi::c_uint,
        __function: *const std::ffi::c_char,
    ) -> !;
    fn UTIL_getTime() -> UTIL_time_t;
    fn UTIL_clockSpanNano(clockStart: UTIL_time_t) -> PTime;
}
pub type size_t = std::ffi::c_ulong;
pub type PTime = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_runTime_t {
    pub nanoSecPerRun: std::ffi::c_double,
    pub sumOfReturn: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_runOutcome_t {
    pub internal_never_ever_use_directly: BMK_runTime_t,
    pub error_result_never_ever_use_directly: size_t,
    pub error_tag_never_ever_use_directly: std::ffi::c_int,
}
pub type BMK_benchFn_t = Option<
    unsafe extern "C" fn(
        *const std::ffi::c_void,
        size_t,
        *mut std::ffi::c_void,
        size_t,
        *mut std::ffi::c_void,
    ) -> size_t,
>;
pub type BMK_initFn_t = Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> size_t>;
pub type BMK_errorFn_t = Option<unsafe extern "C" fn(size_t) -> std::ffi::c_uint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_benchParams_t {
    pub benchFn: BMK_benchFn_t,
    pub benchPayload: *mut std::ffi::c_void,
    pub initFn: BMK_initFn_t,
    pub initPayload: *mut std::ffi::c_void,
    pub errorFn: BMK_errorFn_t,
    pub blockCount: size_t,
    pub srcBuffers: *const *const std::ffi::c_void,
    pub srcSizes: *const size_t,
    pub dstBuffers: *const *mut std::ffi::c_void,
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
    pub nbLoops: std::ffi::c_uint,
    pub coolTime: UTIL_time_t,
}
pub type BMK_timedFnState_t = BMK_timedFnState_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tfs_align {
    pub c: check_size,
    pub tfs: BMK_timedFnState_t,
}
pub type check_size = [std::ffi::c_char; 1];
pub const __ASSERT_FUNCTION: [std::ffi::c_char; 75] = unsafe {
    *::core::mem::transmute::<&[u8; 75], &[std::ffi::c_char; 75]>(
        b"BMK_runOutcome_t BMK_benchTimedFn(BMK_timedFnState_t *, BMK_benchParams_t)\0",
    )
};
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const TIMELOOP_NANOSEC: std::ffi::c_ulonglong = (1 as std::ffi::c_int as std::ffi::c_ulonglong)
    .wrapping_mul(1000000000 as std::ffi::c_ulonglong);
#[no_mangle]
pub unsafe extern "C" fn BMK_isSuccessful_runOutcome(
    mut outcome: BMK_runOutcome_t,
) -> std::ffi::c_int {
    (outcome.error_tag_never_ever_use_directly == 0 as std::ffi::c_int) as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn BMK_extract_runTime(mut outcome: BMK_runOutcome_t) -> BMK_runTime_t {
    if outcome.error_tag_never_ever_use_directly != 0 as std::ffi::c_int {
        abort();
    }
    outcome.internal_never_ever_use_directly
}
#[no_mangle]
pub unsafe extern "C" fn BMK_extract_errorResult(mut outcome: BMK_runOutcome_t) -> size_t {
    if outcome.error_tag_never_ever_use_directly == 0 as std::ffi::c_int {
        abort();
    }
    outcome.error_result_never_ever_use_directly
}
unsafe extern "C" fn BMK_runOutcome_error(mut errorResult: size_t) -> BMK_runOutcome_t {
    let mut b = BMK_runOutcome_t {
        internal_never_ever_use_directly: BMK_runTime_t {
            nanoSecPerRun: 0.,
            sumOfReturn: 0,
        },
        error_result_never_ever_use_directly: 0,
        error_tag_never_ever_use_directly: 0,
    };
    memset(
        &mut b as *mut BMK_runOutcome_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<BMK_runOutcome_t>() as std::ffi::c_ulong,
    );
    b.error_tag_never_ever_use_directly = 1 as std::ffi::c_int;
    b.error_result_never_ever_use_directly = errorResult;
    b
}
unsafe extern "C" fn BMK_setValid_runTime(mut runTime: BMK_runTime_t) -> BMK_runOutcome_t {
    let mut outcome = BMK_runOutcome_t {
        internal_never_ever_use_directly: BMK_runTime_t {
            nanoSecPerRun: 0.,
            sumOfReturn: 0,
        },
        error_result_never_ever_use_directly: 0,
        error_tag_never_ever_use_directly: 0,
    };
    outcome.error_tag_never_ever_use_directly = 0 as std::ffi::c_int;
    outcome.internal_never_ever_use_directly = runTime;
    outcome
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchFunction(
    mut p: BMK_benchParams_t,
    mut nbLoops: std::ffi::c_uint,
) -> BMK_runOutcome_t {
    nbLoops = nbLoops.wrapping_add((nbLoops == 0) as std::ffi::c_int as std::ffi::c_uint);
    let mut i: size_t = 0;
    i = 0 as std::ffi::c_int as size_t;
    while i < p.blockCount {
        memset(
            *(p.dstBuffers).offset(i as isize),
            0xe5 as std::ffi::c_int,
            *(p.dstCapacities).offset(i as isize),
        );
        i = i.wrapping_add(1);
        i;
    }
    let mut dstSize = 0 as std::ffi::c_int as size_t;
    let clockStart = UTIL_getTime();
    let mut loopNb: std::ffi::c_uint = 0;
    let mut blockNb: std::ffi::c_uint = 0;
    if (p.initFn).is_some() {
        (p.initFn).unwrap_unchecked()(p.initPayload);
    }
    loopNb = 0 as std::ffi::c_int as std::ffi::c_uint;
    while loopNb < nbLoops {
        blockNb = 0 as std::ffi::c_int as std::ffi::c_uint;
        while (blockNb as size_t) < p.blockCount {
            let res = (p.benchFn).unwrap_unchecked()(
                *(p.srcBuffers).offset(blockNb as isize),
                *(p.srcSizes).offset(blockNb as isize),
                *(p.dstBuffers).offset(blockNb as isize),
                *(p.dstCapacities).offset(blockNb as isize),
                p.benchPayload,
            );
            if loopNb == 0 as std::ffi::c_int as std::ffi::c_uint {
                if !(p.blockResults).is_null() {
                    *(p.blockResults).offset(blockNb as isize) = res;
                }
                if (p.errorFn).is_some() && (p.errorFn).unwrap_unchecked()(res) != 0 {
                    return BMK_runOutcome_error(res);
                }
                dstSize = dstSize.wrapping_add(res);
            }
            blockNb = blockNb.wrapping_add(1);
            blockNb;
        }
        loopNb = loopNb.wrapping_add(1);
        loopNb;
    }
    let totalTime = UTIL_clockSpanNano(clockStart);
    let mut rt = BMK_runTime_t {
        nanoSecPerRun: 0.,
        sumOfReturn: 0,
    };
    rt.nanoSecPerRun = totalTime as std::ffi::c_double / nbLoops as std::ffi::c_double;
    rt.sumOfReturn = dstSize;
    BMK_setValid_runTime(rt)
}
#[no_mangle]
pub unsafe extern "C" fn BMK_createTimedFnState(
    mut total_ms: std::ffi::c_uint,
    mut run_ms: std::ffi::c_uint,
) -> *mut BMK_timedFnState_t {
    let r = malloc(::core::mem::size_of::<BMK_timedFnState_t>() as std::ffi::c_ulong)
        as *mut BMK_timedFnState_t;
    if r.is_null() {
        return NULL as *mut BMK_timedFnState_t;
    }
    BMK_resetTimedFnState(r, total_ms, run_ms);
    r
}
#[no_mangle]
pub unsafe extern "C" fn BMK_freeTimedFnState(mut state: *mut BMK_timedFnState_t) {
    free(state as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BMK_initStatic_timedFnState(
    mut buffer: *mut std::ffi::c_void,
    mut size: size_t,
    mut total_ms: std::ffi::c_uint,
    mut run_ms: std::ffi::c_uint,
) -> *mut BMK_timedFnState_t {
    let tfs_alignment = 8 as std::ffi::c_ulong;
    let r = buffer as *mut BMK_timedFnState_t;
    if buffer.is_null() {
        return NULL as *mut BMK_timedFnState_t;
    }
    if size < ::core::mem::size_of::<BMK_timedFnState_s>() as std::ffi::c_ulong {
        return NULL as *mut BMK_timedFnState_t;
    }
    if !(buffer as size_t).is_multiple_of(tfs_alignment) {
        return NULL as *mut BMK_timedFnState_t;
    }
    BMK_resetTimedFnState(r, total_ms, run_ms);
    r
}
#[no_mangle]
pub unsafe extern "C" fn BMK_resetTimedFnState(
    mut timedFnState: *mut BMK_timedFnState_t,
    mut total_ms: std::ffi::c_uint,
    mut run_ms: std::ffi::c_uint,
) {
    if total_ms == 0 {
        total_ms = 1 as std::ffi::c_int as std::ffi::c_uint;
    }
    if run_ms == 0 {
        run_ms = 1 as std::ffi::c_int as std::ffi::c_uint;
    }
    if run_ms > total_ms {
        run_ms = total_ms;
    }
    (*timedFnState).timeSpent_ns = 0 as std::ffi::c_int as PTime;
    (*timedFnState).timeBudget_ns = (total_ms as PTime as std::ffi::c_ulonglong)
        .wrapping_mul(TIMELOOP_NANOSEC)
        .wrapping_div(1000 as std::ffi::c_int as std::ffi::c_ulonglong)
        as PTime;
    (*timedFnState).runBudget_ns = (run_ms as PTime as std::ffi::c_ulonglong)
        .wrapping_mul(TIMELOOP_NANOSEC)
        .wrapping_div(1000 as std::ffi::c_int as std::ffi::c_ulonglong)
        as PTime;
    (*timedFnState).fastestRun.nanoSecPerRun = TIMELOOP_NANOSEC as std::ffi::c_double
        * 2000000000 as std::ffi::c_int as std::ffi::c_double;
    (*timedFnState).fastestRun.sumOfReturn = -(1 as std::ffi::c_longlong) as size_t;
    (*timedFnState).nbLoops = 1 as std::ffi::c_int as std::ffi::c_uint;
    (*timedFnState).coolTime = UTIL_getTime();
}
#[no_mangle]
pub unsafe extern "C" fn BMK_isCompleted_TimedFn(
    mut timedFnState: *const BMK_timedFnState_t,
) -> std::ffi::c_int {
    ((*timedFnState).timeSpent_ns >= (*timedFnState).timeBudget_ns) as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn BMK_benchTimedFn(
    mut cont: *mut BMK_timedFnState_t,
    mut p: BMK_benchParams_t,
) -> BMK_runOutcome_t {
    let runBudget_ns = (*cont).runBudget_ns;
    let runTimeMin_ns = runBudget_ns / 2 as std::ffi::c_int as PTime;
    let mut completed = 0 as std::ffi::c_int;
    let mut bestRunTime = (*cont).fastestRun;
    while completed == 0 {
        let runResult = BMK_benchFunction(p, (*cont).nbLoops);
        if BMK_isSuccessful_runOutcome(runResult) == 0 {
            return runResult;
        }
        let newRunTime = BMK_extract_runTime(runResult);
        let loopDuration_ns = newRunTime.nanoSecPerRun * (*cont).nbLoops as std::ffi::c_double;
        (*cont).timeSpent_ns = ((*cont).timeSpent_ns as std::ffi::c_ulonglong)
            .wrapping_add(loopDuration_ns as std::ffi::c_ulonglong)
            as PTime as PTime;
        if loopDuration_ns
            > runBudget_ns as std::ffi::c_double / 50 as std::ffi::c_int as std::ffi::c_double
        {
            let fastestRun_ns = if bestRunTime.nanoSecPerRun < newRunTime.nanoSecPerRun {
                bestRunTime.nanoSecPerRun
            } else {
                newRunTime.nanoSecPerRun
            };
            (*cont).nbLoops = ((runBudget_ns as std::ffi::c_double / fastestRun_ns)
                as std::ffi::c_uint)
                .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
        } else {
            let multiplier = 10 as std::ffi::c_int as std::ffi::c_uint;
            if (*cont).nbLoops
                < (-(1 as std::ffi::c_int) as std::ffi::c_uint).wrapping_div(multiplier)
            {
            } else {
                __assert_fail(
                    b"cont->nbLoops < ((unsigned)-1) / multiplier\0" as *const u8
                        as *const std::ffi::c_char,
                    b"benchfn.c\0" as *const u8 as *const std::ffi::c_char,
                    238 as std::ffi::c_int as std::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
            'c_2223: {
                if (*cont).nbLoops
                    < (-(1 as std::ffi::c_int) as std::ffi::c_uint).wrapping_div(multiplier)
                {
                } else {
                    __assert_fail(
                        b"cont->nbLoops < ((unsigned)-1) / multiplier\0" as *const u8
                            as *const std::ffi::c_char,
                        b"benchfn.c\0" as *const u8 as *const std::ffi::c_char,
                        238 as std::ffi::c_int as std::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                }
            };
            (*cont).nbLoops = ((*cont).nbLoops).wrapping_mul(multiplier);
        }
        if loopDuration_ns < runTimeMin_ns as std::ffi::c_double {
            if completed == 0 as std::ffi::c_int {
            } else {
                __assert_fail(
                    b"completed == 0\0" as *const u8 as *const std::ffi::c_char,
                    b"benchfn.c\0" as *const u8 as *const std::ffi::c_char,
                    244 as std::ffi::c_int as std::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
            'c_2141: {
                if completed == 0 as std::ffi::c_int {
                } else {
                    __assert_fail(
                        b"completed == 0\0" as *const u8 as *const std::ffi::c_char,
                        b"benchfn.c\0" as *const u8 as *const std::ffi::c_char,
                        244 as std::ffi::c_int as std::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                }
            };
        } else {
            if newRunTime.nanoSecPerRun < bestRunTime.nanoSecPerRun {
                bestRunTime = newRunTime;
            }
            completed = 1 as std::ffi::c_int;
        }
    }
    BMK_setValid_runTime(bestRunTime)
}
