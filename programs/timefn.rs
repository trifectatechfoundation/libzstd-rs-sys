use libc::{abort, clock_gettime, perror, timespec, CLOCK_MONOTONIC};

pub type PTime = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
pub unsafe fn UTIL_getTime() -> UTIL_time_t {
    let mut time = core::mem::zeroed::<timespec>();
    if clock_gettime(CLOCK_MONOTONIC, &mut time) != 0 {
        perror(
            b"timefn::clock_gettime(CLOCK_MONOTONIC)\0" as *const u8 as *const core::ffi::c_char,
        );
        abort();
    }
    let mut r = UTIL_time_t { t: 0 };
    r.t = (time.tv_sec as PTime as core::ffi::c_ulonglong)
        .wrapping_mul(1000000000)
        .wrapping_add(time.tv_nsec as PTime as core::ffi::c_ulonglong) as PTime;
    r
}
pub unsafe fn UTIL_getSpanTimeNano(
    mut clockStart: UTIL_time_t,
    mut clockEnd: UTIL_time_t,
) -> PTime {
    (clockEnd.t).wrapping_sub(clockStart.t)
}
pub unsafe fn UTIL_getSpanTimeMicro(mut begin: UTIL_time_t, mut end: UTIL_time_t) -> PTime {
    (UTIL_getSpanTimeNano(begin, end) as core::ffi::c_ulonglong).wrapping_div(1000) as PTime
}
pub unsafe fn UTIL_clockSpanMicro(mut clockStart: UTIL_time_t) -> PTime {
    let clockEnd = UTIL_getTime();
    UTIL_getSpanTimeMicro(clockStart, clockEnd)
}
pub unsafe fn UTIL_clockSpanNano(mut clockStart: UTIL_time_t) -> PTime {
    let clockEnd = UTIL_getTime();
    UTIL_getSpanTimeNano(clockStart, clockEnd)
}
pub unsafe fn UTIL_waitForNextTick() {
    let clockStart = UTIL_getTime();
    let mut clockEnd = UTIL_time_t { t: 0 };
    loop {
        clockEnd = UTIL_getTime();
        if UTIL_getSpanTimeNano(clockStart, clockEnd) != 0 {
            break;
        }
    }
}
pub unsafe fn UTIL_support_MT_measurements() -> core::ffi::c_int {
    1
}
