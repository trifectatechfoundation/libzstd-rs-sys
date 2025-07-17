extern "C" {
    fn perror(__s: *const std::ffi::c_char);
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> std::ffi::c_int;
    fn abort() -> !;
}
pub type __uint64_t = std::ffi::c_ulong;
pub type __time_t = std::ffi::c_long;
pub type __clockid_t = std::ffi::c_int;
pub type __syscall_slong_t = std::ffi::c_long;
pub type uint64_t = __uint64_t;
pub type PTime = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UTIL_time_t {
    pub t: PTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub const CLOCK_MONOTONIC: std::ffi::c_int = 1 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn UTIL_getTime() -> UTIL_time_t {
    let mut time = {
        timespec {
            tv_sec: 0 as std::ffi::c_int as __time_t,
            tv_nsec: 0 as std::ffi::c_int as __syscall_slong_t,
        }
    };
    if clock_gettime(CLOCK_MONOTONIC, &mut time) != 0 as std::ffi::c_int {
        perror(b"timefn::clock_gettime(CLOCK_MONOTONIC)\0" as *const u8 as *const std::ffi::c_char);
        abort();
    }
    let mut r = UTIL_time_t { t: 0 };
    r.t = (time.tv_sec as PTime as std::ffi::c_ulonglong)
        .wrapping_mul(1000000000 as std::ffi::c_ulonglong)
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
    (UTIL_getSpanTimeNano(begin, end) as std::ffi::c_ulonglong)
        .wrapping_div(1000 as std::ffi::c_ulonglong) as PTime
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
        if UTIL_getSpanTimeNano(clockStart, clockEnd) != 0 as std::ffi::c_int as PTime {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn UTIL_support_MT_measurements() -> std::ffi::c_int {
    1 as std::ffi::c_int
}
