use libc::{abort, clock_gettime, perror, timespec, CLOCK_MONOTONIC};

pub type PTime = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getTime() -> UTIL_time_t {
    let mut time = core::mem::zeroed::<timespec>();
    if clock_gettime(CLOCK_MONOTONIC, &mut time) != 0 {
        perror(b"timefn::clock_gettime(CLOCK_MONOTONIC)\0" as *const u8 as *const std::ffi::c_char);
        abort();
    }
    let mut r = UTIL_time_t { t: 0 };
    r.t = (time.tv_sec as PTime as std::ffi::c_ulonglong)
        .wrapping_mul(1000000000)
        .wrapping_add(time.tv_nsec as PTime as std::ffi::c_ulonglong) as PTime;
    r
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getSpanTimeNano(
    mut clockStart: UTIL_time_t,
    mut clockEnd: UTIL_time_t,
) -> PTime {
    (clockEnd.t).wrapping_sub(clockStart.t)
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_getSpanTimeMicro(
    mut begin: UTIL_time_t,
    mut end: UTIL_time_t,
) -> PTime {
    (UTIL_getSpanTimeNano(begin, end) as std::ffi::c_ulonglong).wrapping_div(1000) as PTime
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_clockSpanMicro(mut clockStart: UTIL_time_t) -> PTime {
    let clockEnd = UTIL_getTime();
    UTIL_getSpanTimeMicro(clockStart, clockEnd)
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_clockSpanNano(mut clockStart: UTIL_time_t) -> PTime {
    let clockEnd = UTIL_getTime();
    UTIL_getSpanTimeNano(clockStart, clockEnd)
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_waitForNextTick() {
    let clockStart = UTIL_getTime();
    let mut clockEnd = UTIL_time_t { t: 0 };
    loop {
        clockEnd = UTIL_getTime();
        if UTIL_getSpanTimeNano(clockStart, clockEnd) != 0 {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_support_MT_measurements() -> std::ffi::c_int {
    1
}
