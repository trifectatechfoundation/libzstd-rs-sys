pub type size_t = core::ffi::c_ulong;
pub type HIST_checkInput_e = core::ffi::c_uint;
pub const checkMaxSymbolValue: HIST_checkInput_e = 1;
pub const trustInput: HIST_checkInput_e = 0;
use core::ptr;

use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::mem::MEM_read32;
use crate::lib::zstd::*;
pub const HIST_WKSP_SIZE_U32: core::ffi::c_int = 1024;
pub const HIST_WKSP_SIZE: size_t =
    (HIST_WKSP_SIZE_U32 as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as size_t);
pub const HIST_FAST_THRESHOLD: core::ffi::c_int = 1500;
pub unsafe fn HIST_isError(mut code: size_t) -> core::ffi::c_uint {
    ERR_isError(code)
}
pub unsafe fn HIST_add(
    mut count: *mut core::ffi::c_uint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) {
    let mut ip = src as *const u8;
    let end = ip.offset(srcSize as isize);
    while ip < end {
        let fresh0 = ip;
        ip = ip.offset(1);
        let fresh1 = &mut (*count.offset(*fresh0 as isize));
        *fresh1 = (*fresh1).wrapping_add(1);
    }
}
pub unsafe fn HIST_count_simple(
    mut count: *mut core::ffi::c_uint,
    mut maxSymbolValuePtr: *mut core::ffi::c_uint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> core::ffi::c_uint {
    let mut ip = src as *const u8;
    let end = ip.offset(srcSize as isize);
    let mut maxSymbolValue = *maxSymbolValuePtr;
    let mut largestCount = 0;
    ptr::write_bytes(
        count as *mut u8,
        0,
        (maxSymbolValue.wrapping_add(1) as core::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong)
            as libc::size_t,
    );
    if srcSize == 0 {
        *maxSymbolValuePtr = 0;
        return 0;
    }
    while ip < end {
        let fresh2 = ip;
        ip = ip.offset(1);
        let fresh3 = &mut (*count.offset(*fresh2 as isize));
        *fresh3 = (*fresh3).wrapping_add(1);
    }
    while *count.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
    }
    *maxSymbolValuePtr = maxSymbolValue;
    let mut s: u32 = 0;
    s = 0;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) > largestCount {
            largestCount = *count.offset(s as isize);
        }
        s = s.wrapping_add(1);
    }
    largestCount
}
unsafe fn HIST_count_parallel_wksp(
    mut count: *mut core::ffi::c_uint,
    mut maxSymbolValuePtr: *mut core::ffi::c_uint,
    mut source: *const core::ffi::c_void,
    mut sourceSize: size_t,
    mut check: HIST_checkInput_e,
    workSpace: *mut u32,
) -> size_t {
    let mut ip = source as *const u8;
    let iend = ip.offset(sourceSize as isize);
    let countSize = ((*maxSymbolValuePtr).wrapping_add(1) as core::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong);
    let mut max = 0;
    let Counting1 = workSpace;
    let Counting2 = Counting1.offset(256);
    let Counting3 = Counting2.offset(256);
    let Counting4 = Counting3.offset(256);
    if sourceSize == 0 {
        ptr::write_bytes(count as *mut u8, 0, countSize as libc::size_t);
        *maxSymbolValuePtr = 0;
        return 0;
    }
    ptr::write_bytes(
        workSpace as *mut u8,
        0,
        ((4 * 256) as core::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong)
            as libc::size_t,
    );
    let mut cached = MEM_read32(ip as *const core::ffi::c_void);
    ip = ip.offset(4);
    while ip < iend.offset(-(15)) {
        let mut c = cached;
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.offset(4);
        let fresh4 = &mut (*Counting1.offset(c as u8 as isize));
        *fresh4 = (*fresh4).wrapping_add(1);
        let fresh5 = &mut (*Counting2.offset((c >> 8) as u8 as isize));
        *fresh5 = (*fresh5).wrapping_add(1);
        let fresh6 = &mut (*Counting3.offset((c >> 16) as u8 as isize));
        *fresh6 = (*fresh6).wrapping_add(1);
        let fresh7 = &mut (*Counting4.offset((c >> 24) as isize));
        *fresh7 = (*fresh7).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.offset(4);
        let fresh8 = &mut (*Counting1.offset(c as u8 as isize));
        *fresh8 = (*fresh8).wrapping_add(1);
        let fresh9 = &mut (*Counting2.offset((c >> 8) as u8 as isize));
        *fresh9 = (*fresh9).wrapping_add(1);
        let fresh10 = &mut (*Counting3.offset((c >> 16) as u8 as isize));
        *fresh10 = (*fresh10).wrapping_add(1);
        let fresh11 = &mut (*Counting4.offset((c >> 24) as isize));
        *fresh11 = (*fresh11).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.offset(4);
        let fresh12 = &mut (*Counting1.offset(c as u8 as isize));
        *fresh12 = (*fresh12).wrapping_add(1);
        let fresh13 = &mut (*Counting2.offset((c >> 8) as u8 as isize));
        *fresh13 = (*fresh13).wrapping_add(1);
        let fresh14 = &mut (*Counting3.offset((c >> 16) as u8 as isize));
        *fresh14 = (*fresh14).wrapping_add(1);
        let fresh15 = &mut (*Counting4.offset((c >> 24) as isize));
        *fresh15 = (*fresh15).wrapping_add(1);
        c = cached;
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.offset(4);
        let fresh16 = &mut (*Counting1.offset(c as u8 as isize));
        *fresh16 = (*fresh16).wrapping_add(1);
        let fresh17 = &mut (*Counting2.offset((c >> 8) as u8 as isize));
        *fresh17 = (*fresh17).wrapping_add(1);
        let fresh18 = &mut (*Counting3.offset((c >> 16) as u8 as isize));
        *fresh18 = (*fresh18).wrapping_add(1);
        let fresh19 = &mut (*Counting4.offset((c >> 24) as isize));
        *fresh19 = (*fresh19).wrapping_add(1);
    }
    ip = ip.offset(-(4));
    while ip < iend {
        let fresh20 = ip;
        ip = ip.offset(1);
        let fresh21 = &mut (*Counting1.offset(*fresh20 as isize));
        *fresh21 = (*fresh21).wrapping_add(1);
    }
    let mut s: u32 = 0;
    s = 0;
    while s < 256 {
        let fresh22 = &mut (*Counting1.offset(s as isize));
        *fresh22 = (*fresh22).wrapping_add(
            (*Counting2.offset(s as isize))
                .wrapping_add(*Counting3.offset(s as isize))
                .wrapping_add(*Counting4.offset(s as isize)),
        );
        if *Counting1.offset(s as isize) > max {
            max = *Counting1.offset(s as isize);
        }
        s = s.wrapping_add(1);
    }
    let mut maxSymbolValue = 255 as core::ffi::c_uint;
    while *Counting1.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
    }
    if check as core::ffi::c_uint != 0 && maxSymbolValue > *maxSymbolValuePtr {
        return -(ZSTD_error_maxSymbolValue_tooSmall as core::ffi::c_int) as size_t;
    }
    *maxSymbolValuePtr = maxSymbolValue;
    libc::memmove(
        count as *mut core::ffi::c_void,
        Counting1 as *const core::ffi::c_void,
        countSize as libc::size_t,
    );
    max as size_t
}
pub unsafe fn HIST_countFast_wksp(
    mut count: *mut core::ffi::c_uint,
    mut maxSymbolValuePtr: *mut core::ffi::c_uint,
    mut source: *const core::ffi::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut core::ffi::c_void,
    mut workSpaceSize: size_t,
) -> size_t {
    if sourceSize < HIST_FAST_THRESHOLD as size_t {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as size_t;
    }
    if workSpace as size_t & 3 != 0 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    if workSpaceSize < HIST_WKSP_SIZE {
        return -(ZSTD_error_workSpace_tooSmall as core::ffi::c_int) as size_t;
    }
    HIST_count_parallel_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        trustInput,
        workSpace as *mut u32,
    )
}
pub unsafe fn HIST_count_wksp(
    mut count: *mut core::ffi::c_uint,
    mut maxSymbolValuePtr: *mut core::ffi::c_uint,
    mut source: *const core::ffi::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut core::ffi::c_void,
    mut workSpaceSize: size_t,
) -> size_t {
    if workSpace as size_t & 3 != 0 {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }
    if workSpaceSize < HIST_WKSP_SIZE {
        return -(ZSTD_error_workSpace_tooSmall as core::ffi::c_int) as size_t;
    }
    if *maxSymbolValuePtr < 255 {
        return HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            checkMaxSymbolValue,
            workSpace as *mut u32,
        );
    }
    *maxSymbolValuePtr = 255;
    HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        workSpace,
        workSpaceSize,
    )
}
pub unsafe fn HIST_countFast(
    mut count: *mut core::ffi::c_uint,
    mut maxSymbolValuePtr: *mut core::ffi::c_uint,
    mut source: *const core::ffi::c_void,
    mut sourceSize: size_t,
) -> size_t {
    let mut tmpCounters: [core::ffi::c_uint; 1024] = [0; 1024];
    HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_uint; 1024]>() as size_t,
    )
}
pub unsafe fn HIST_count(
    mut count: *mut core::ffi::c_uint,
    mut maxSymbolValuePtr: *mut core::ffi::c_uint,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut tmpCounters: [core::ffi::c_uint; 1024] = [0; 1024];
    HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_uint; 1024]>() as size_t,
    )
}
