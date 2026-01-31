use core::ptr;

use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::MEM_read32;

#[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
pub const HIST_WKSP_SIZE_U32: usize = 0;

#[cfg(not(target_feature = "sve2"))]
pub const HIST_WKSP_SIZE_U32: usize = 1024;

pub const HIST_WKSP_SIZE: usize = HIST_WKSP_SIZE_U32 * size_of::<core::ffi::c_uint>();

#[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
pub const HIST_FAST_THRESHOLD: core::ffi::c_int = 500;

#[cfg(not(target_feature = "sve2"))]
pub const HIST_FAST_THRESHOLD: core::ffi::c_int = 1500;

pub unsafe fn HIST_isError(code: size_t) -> core::ffi::c_uint {
    ERR_isError(code) as _
}

/// Lowest level: just add nb of occurrences of characters from `src` into `count`.
/// `count` is not reset. `count` array is presumed large enough (i.e. 1 KB).
/// This function does not need any additional stack memory.
pub unsafe fn HIST_add(
    count: *mut core::ffi::c_uint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    let mut ip = src as *const u8;
    let end = ip.add(srcSize);
    while ip < end {
        let fresh0 = ip;
        ip = ip.add(1);
        let fresh1 = &mut (*count.offset(*fresh0 as isize));
        *fresh1 = (*fresh1).wrapping_add(1);
    }
}

/// Same as [`HIST_countFast`], this function is unsafe,
/// and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
/// It is also a bit slower for large inputs.
/// However, it does not need any additional memory (not even on stack).
/// @return : count of the most frequent symbol.
/// Note this function doesn't produce any error (i.e. it must succeed).
pub unsafe fn HIST_count_simple(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: *mut core::ffi::c_uint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> core::ffi::c_uint {
    let mut ip = src as *const u8;
    let end = ip.add(srcSize);
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
        ip = ip.add(1);
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

enum HIST_checkInput_e {
    checkMaxSymbolValue = 1,
    trustInput = 0,
}

#[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
#[inline(always)]
fn min_size(a: usize, b: usize) -> usize {
    return if a < b { a } else { b };
}

#[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
use core::arch::aarch64::{svuint16_t, svuint8_t};

#[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
unsafe fn HIST_count_6_sve2(
    src: *const BYTE,
    size: usize,
    dst: *const U32,
    c0: svuint8_t,
    c1: svuint8_t,
    c2: svuint8_t,
    c3: svuint8_t,
    c4: svuint8_t,
    c5: svuint8_t,
    histmax: svuint16_t,
    maxCount: usize,
) -> svuint16_t {
    unimplemented!("SVE2 histogram counting not yet implemented");
}

#[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
unsafe fn HIST_count_sve2(
    count: *mut c_uint,
    maxSymbolValuePtr: *mut c_uint,
    source: *const c_void,
    sourceSize: usize,
    check: HIST_checkInput_e,
) -> usize {
    unimplemented!("SVE2 histogram counting not yet implemented");
}

/// store histogram into 4 intermediate tables, recombined at the end.
/// this design makes better use of OoO cpus,
/// and is noticeably faster when some values are heavily repeated.
/// But it needs some additional workspace for intermediate tables.
/// `workSpace` must be a U32 table of size >= HIST_WKSP_SIZE_U32.
/// @return : largest histogram frequency,
///           or an error code (notably when histogram's alphabet is larger than *maxSymbolValuePtr)
unsafe fn HIST_count_parallel_wksp(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: *mut core::ffi::c_uint,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
    check: HIST_checkInput_e,
    workSpace: *mut u32,
) -> size_t {
    let mut ip = source as *const u8;
    let iend = ip.add(sourceSize);
    let countSize = ((*maxSymbolValuePtr).wrapping_add(1) as core::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong);
    let mut max = 0;
    let Counting1 = workSpace;
    let Counting2 = Counting1.add(256);
    let Counting3 = Counting2.add(256);
    let Counting4 = Counting3.add(256);

    /* safety checks */
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

    /* by stripes of 16 bytes */
    {
        let mut cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        while ip < iend.sub(15) {
            let mut c = cached;
            cached = MEM_read32(ip as *const core::ffi::c_void);
            ip = ip.add(4);
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
            ip = ip.add(4);
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
            ip = ip.add(4);
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
            ip = ip.add(4);
            let fresh16 = &mut (*Counting1.offset(c as u8 as isize));
            *fresh16 = (*fresh16).wrapping_add(1);
            let fresh17 = &mut (*Counting2.offset((c >> 8) as u8 as isize));
            *fresh17 = (*fresh17).wrapping_add(1);
            let fresh18 = &mut (*Counting3.offset((c >> 16) as u8 as isize));
            *fresh18 = (*fresh18).wrapping_add(1);
            let fresh19 = &mut (*Counting4.offset((c >> 24) as isize));
            *fresh19 = (*fresh19).wrapping_add(1);
        }
        ip = ip.sub(4);
    }

    /* finish last symbols */
    while ip < iend {
        let fresh20 = ip;
        ip = ip.add(1);
        let fresh21 = &mut (*Counting1.offset(*fresh20 as isize));
        *fresh21 = (*fresh21).wrapping_add(1);
    }

    {
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
    }

    {
        let mut maxSymbolValue = 255 as core::ffi::c_uint;
        while *Counting1.offset(maxSymbolValue as isize) == 0 {
            maxSymbolValue = maxSymbolValue.wrapping_sub(1);
        }
        if check as core::ffi::c_uint != 0 && maxSymbolValue > *maxSymbolValuePtr {
            return Error::maxSymbolValue_tooSmall.to_error_code();
        }
        *maxSymbolValuePtr = maxSymbolValue;

        /* in case count & Counting1 are overlapping */
        core::ptr::copy(Counting1 as *const u8, count as *mut u8, countSize as usize);
    }
    max as size_t
}

/// Same as [`HIST_countFast`], but using an externally provided scratch buffer.
/// `workSpace` is a writable buffer which must be 4-bytes aligned,
/// `workSpaceSize` must be >= `HIST_WKSP_SIZE`
pub unsafe fn HIST_countFast_wksp(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: *mut core::ffi::c_uint,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
    workSpace: *mut core::ffi::c_void,
    workSpaceSize: size_t,
) -> size_t {
    // heuristic threshold
    if sourceSize < HIST_FAST_THRESHOLD as size_t {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as size_t;
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
    {
        return HIST_count_sve2(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            HIST_checkInput_e::trustInput,
        );
    }

    #[cfg(not(target_feature = "sve2"))]
    {
        if workSpace as size_t & 3 != 0 {
            // must be aligned on 4-bytes boundaries
            return Error::GENERIC.to_error_code();
        }

        if workSpaceSize < HIST_WKSP_SIZE {
            return Error::workSpace_tooSmall.to_error_code();
        }
        HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            HIST_checkInput_e::trustInput,
            workSpace as *mut u32,
        )
    }
}

///  Same as [`HIST_count`], but using an externally provided scratch buffer.
///  Benefit is this function will use very little stack space.
/// `workSpace` is a writable buffer which must be 4-bytes aligned,
/// `workSpaceSize` must be >= HIST_WKSP_SIZE
pub unsafe fn HIST_count_wksp(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: *mut core::ffi::c_uint,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
    workSpace: *mut core::ffi::c_void,
    workSpaceSize: size_t,
) -> size_t {
    #[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
    if (*maxSymbolValuePtr < 255) {
        return HIST_count_sve2(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            checkMaxSymbolValue,
        );
    }

    #[cfg(not(target_feature = "sve2"))]
    {
        if workSpace as size_t & 3 != 0 {
            // must be aligned on 4-bytes boundaries
            return Error::GENERIC.to_error_code();
        }
        if workSpaceSize < HIST_WKSP_SIZE {
            return Error::workSpace_tooSmall.to_error_code();
        }

        if *maxSymbolValuePtr < 255 {
            return HIST_count_parallel_wksp(
                count,
                maxSymbolValuePtr,
                source,
                sourceSize,
                HIST_checkInput_e::checkMaxSymbolValue,
                workSpace as *mut u32,
            );
        }
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

/// same as [`HIST_count`], but blindly trusts that all byte values within src are <= *maxSymbolValuePtr.
/// This function is unsafe, and will segfault if any value within `src` is `> *maxSymbolValuePtr`
/// fast variant (unsafe : won't check if src contains values beyond count[] limit)
pub unsafe fn HIST_countFast(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: *mut core::ffi::c_uint,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
) -> size_t {
    let mut tmpCounters: [core::ffi::c_uint; 1024] = [0; 1024];
    HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_uint; 1024]>(),
    )
}

/// Provides the precise count of each byte within a table 'count'.
/// 'count' is a table of unsigned int, of minimum size (*maxSymbolValuePtr+1).
/// Updates *maxSymbolValuePtr with actual largest symbol value detected.
/// @return : count of the most frequent symbol (which isn't identified).
///           or an error code, which can be tested using HIST_isError().
///           note : if return == srcSize, there is only one symbol.
pub unsafe fn HIST_count(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: *mut core::ffi::c_uint,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut tmpCounters: [core::ffi::c_uint; 1024] = [0; 1024];
    HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr() as *mut core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_uint; 1024]>(),
    )
}
