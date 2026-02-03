use core::ffi::{c_uint, c_ulong, c_void};
use core::ptr;

use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::MEM_read32;

#[cfg(all(target_arch = "aarch64", target_feature = "sve2"))]
pub const HIST_WKSP_SIZE_U32: usize = 0;

#[cfg(not(target_feature = "sve2"))]
pub const HIST_WKSP_SIZE_U32: usize = 1024;

pub const HIST_WKSP_SIZE: usize = HIST_WKSP_SIZE_U32 * size_of::<c_uint>();

pub const HIST_FAST_THRESHOLD: core::ffi::c_int = {
    if cfg!(all(target_arch = "aarch64", target_feature = "sve2")) {
        500
    } else {
        1500
    }
};

pub unsafe fn HIST_isError(code: size_t) -> c_uint {
    ERR_isError(code) as _
}

/// Lowest level: just add nb of occurrences of characters from `src` into `count`.
/// `count` is not reset. `count` array is presumed large enough (i.e. 1 KB).
/// This function does not need any additional stack memory.
pub unsafe fn HIST_add(count: &mut [c_uint], src: *const c_void, srcSize: size_t) {
    let ip = core::slice::from_raw_parts(src as *const u8, srcSize);

    for item in ip.iter() {
        count[*item as usize] += 1;
    }
}

/// Same as [`HIST_countFast`], this function is unsafe,
/// and will segfault if any value within `src` is `> *maxSymbolValuePtr`.
/// It is also a bit slower for large inputs.
/// However, it does not need any additional memory (not even on stack).
/// @return : count of the most frequent symbol.
/// Note this function doesn't produce any error (i.e. it must succeed).
pub unsafe fn HIST_count_simple(
    count: *mut c_uint,
    maxSymbolValuePtr: *mut c_uint,
    src: *const c_void,
    srcSize: size_t,
) -> c_uint {
    let mut ip = src as *const u8;
    let end = ip.add(srcSize);
    let mut maxSymbolValue = *maxSymbolValuePtr;
    let mut largestCount = 0;

    ptr::write_bytes(
        count as *mut u8,
        0,
        (maxSymbolValue.wrapping_add(1) as c_ulong).wrapping_mul(size_of::<c_uint>() as c_ulong)
            as libc::size_t,
    );
    if srcSize == 0 {
        *maxSymbolValuePtr = 0;
        return 0;
    }

    while ip < end {
        debug_assert!(*ip as u32 <= maxSymbolValue);
        let fresh2 = ip;
        ip = ip.add(1);
        let fresh3 = &mut (*count.offset(*fresh2 as isize));
        *fresh3 = (*fresh3).wrapping_add(1);
    }

    while *count.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
    }
    *maxSymbolValuePtr = maxSymbolValue;

    {
        let mut s: u32 = 0;
        s = 0;
        while s <= maxSymbolValue {
            if *count.offset(s as isize) > largestCount {
                largestCount = *count.offset(s as isize);
            }
            s = s.wrapping_add(1);
        }
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
    count: *mut c_uint,
    maxSymbolValuePtr: *mut c_uint,
    source: *const c_void,
    sourceSize: size_t,
    check: HIST_checkInput_e,
    workSpace: &mut [u32],
) -> size_t {
    let mut ip = source as *const u8;
    let iend = ip.add(sourceSize);
    let countSize = ((*maxSymbolValuePtr).wrapping_add(1) as c_ulong)
        .wrapping_mul(size_of::<c_uint>() as c_ulong);
    let mut max = 0;

    debug_assert!(workSpace.len() >= HIST_WKSP_SIZE_U32);

    /* safety checks */
    debug_assert!(*maxSymbolValuePtr <= 255);
    if sourceSize == 0 {
        ptr::write_bytes(count as *mut u8, 0, countSize as libc::size_t);
        *maxSymbolValuePtr = 0;
        return 0;
    }

    workSpace[..1024].fill(0);

    // Split workspace into 4 counting tables of 256 u32 each
    let ([Counting1, Counting2, Counting3, Counting4], &mut []) = workSpace.as_chunks_mut::<256>()
    else {
        unreachable!();
    };

    /* by stripes of 16 bytes */
    {
        let mut cached = MEM_read32(ip as *const c_void);
        ip = ip.add(4);
        while ip < iend.sub(15) {
            let mut c = cached;
            cached = MEM_read32(ip as *const c_void);
            ip = ip.add(4);
            let indices: [u8; 4] = c.to_le_bytes();
            Counting1[indices[0] as usize] += 1;
            Counting2[indices[1] as usize] += 1;
            Counting3[indices[2] as usize] += 1;
            Counting4[indices[3] as usize] += 1;

            c = cached;
            cached = MEM_read32(ip as *const c_void);
            ip = ip.add(4);
            let indices: [u8; 4] = c.to_le_bytes();
            Counting1[indices[0] as usize] += 1;
            Counting2[indices[1] as usize] += 1;
            Counting3[indices[2] as usize] += 1;
            Counting4[indices[3] as usize] += 1;

            c = cached;
            cached = MEM_read32(ip as *const c_void);
            ip = ip.add(4);
            let indices: [u8; 4] = c.to_le_bytes();
            Counting1[indices[0] as usize] += 1;
            Counting2[indices[1] as usize] += 1;
            Counting3[indices[2] as usize] += 1;
            Counting4[indices[3] as usize] += 1;

            c = cached;
            cached = MEM_read32(ip as *const c_void);
            ip = ip.add(4);
            let indices: [u8; 4] = c.to_le_bytes();
            Counting1[indices[0] as usize] += 1;
            Counting2[indices[1] as usize] += 1;
            Counting3[indices[2] as usize] += 1;
            Counting4[indices[3] as usize] += 1;
        }
        ip = ip.sub(4);
    }

    /* finish last symbols */
    while ip < iend {
        Counting1[*ip as usize] += 1;
        ip = ip.add(1);
    }

    {
        for s in 0..256 {
            Counting1[s] += Counting2[s] + Counting3[s] + Counting4[s];
            if Counting1[s] > max {
                max = Counting1[s];
            }
        }
    }

    {
        let mut maxSymbolValue = 255 as c_uint;
        while Counting1[maxSymbolValue as usize] == 0 {
            maxSymbolValue = maxSymbolValue.wrapping_sub(1);
        }
        if check as c_uint != 0 && maxSymbolValue > *maxSymbolValuePtr {
            return Error::maxSymbolValue_tooSmall.to_error_code();
        }
        *maxSymbolValuePtr = maxSymbolValue;

        /* in case count & Counting1 are overlapping */
        core::ptr::copy(
            Counting1.as_ptr() as *const u8,
            count as *mut u8,
            countSize as usize,
        );
    }
    max as size_t
}

/// Same as [`HIST_countFast`], but using an externally provided scratch buffer.
/// `workSpace` is a writable buffer which must be 4-bytes aligned,
/// `workSpaceSize` must be >= `HIST_WKSP_SIZE`
pub unsafe fn HIST_countFast_wksp(
    count: *mut c_uint,
    maxSymbolValuePtr: *mut c_uint,
    source: *const c_void,
    sourceSize: size_t,
    workSpace: &mut [u32],
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
        if workSpace.as_ptr() as size_t & 3 != 0 {
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
            workSpace,
        )
    }
}

///  Same as [`HIST_count`], but using an externally provided scratch buffer.
///  Benefit is this function will use very little stack space.
/// `workSpace` is a writable buffer which must be 4-bytes aligned,
/// `workSpaceSize` must be >= HIST_WKSP_SIZE
pub unsafe fn HIST_count_wksp(
    count: *mut c_uint,
    maxSymbolValuePtr: *mut c_uint,
    source: *const c_void,
    sourceSize: size_t,
    workSpace: &mut [u32],
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
        if workSpace.as_ptr() as size_t & 3 != 0 {
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
                workSpace,
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
    count: *mut c_uint,
    maxSymbolValuePtr: *mut c_uint,
    source: *const c_void,
    sourceSize: size_t,
) -> size_t {
    let mut tmpCounters: [c_uint; 1024] = [0; 1024];

    HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        &mut tmpCounters,
        size_of::<[c_uint; 1024]>(),
    )
}

/// Provides the precise count of each byte within a table 'count'.
/// 'count' is a table of unsigned int, of minimum size (*maxSymbolValuePtr+1).
/// Updates *maxSymbolValuePtr with actual largest symbol value detected.
/// @return : count of the most frequent symbol (which isn't identified).
///           or an error code, which can be tested using HIST_isError().
///           note : if return == srcSize, there is only one symbol.
pub unsafe fn HIST_count(
    count: *mut c_uint,
    maxSymbolValuePtr: *mut c_uint,
    src: *const c_void,
    srcSize: size_t,
) -> size_t {
    let mut tmpCounters: [c_uint; 1024] = [0; 1024];
    HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        &mut tmpCounters,
        size_of::<[c_uint; 1024]>(),
    )
}
