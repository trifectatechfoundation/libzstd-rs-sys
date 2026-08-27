pub type HIST_checkInput_e = core::ffi::c_uint;
pub const checkMaxSymbolValue: HIST_checkInput_e = 1;
pub const trustInput: HIST_checkInput_e = 0;
use core::ptr;

use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::MEM_read32;
pub const HIST_WKSP_SIZE_U32: usize = 1024;
pub const HIST_WKSP_SIZE: size_t =
    (HIST_WKSP_SIZE_U32 as size_t).wrapping_mul(size_of::<core::ffi::c_uint>());
pub const HIST_FAST_THRESHOLD: core::ffi::c_int = 1500;

pub fn HIST_isError(code: size_t) -> core::ffi::c_uint {
    ERR_isError(code) as _
}

pub unsafe fn HIST_add(
    count: &mut [core::ffi::c_uint; 1024],
    src: *const core::ffi::c_void,
    srcSize: size_t,
) {
    let mut ip = src as *const u8;
    let end = ip.add(srcSize);
    while ip < end {
        count[usize::from(*ip)] += 1;
        ip = ip.add(1);
    }
}

pub unsafe fn HIST_count_simple(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: &mut u8,
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
        (usize::from(maxSymbolValue) + 1) * size_of::<core::ffi::c_uint>(),
    );
    if srcSize == 0 {
        *maxSymbolValuePtr = 0;
        return 0;
    }

    while ip < end {
        *count.add(usize::from(*ip)) += 1;
        ip = ip.add(1);
    }

    // `srcSize` is non-zero, so (assuming no symbol exceeds `maxSymbolValue`, which this
    // variant deliberately does not check) at least one symbol has a non-zero count
    while *count.add(usize::from(maxSymbolValue)) == 0 {
        maxSymbolValue -= 1;
    }
    *maxSymbolValuePtr = maxSymbolValue;

    for s in 0..usize::from(maxSymbolValue) + 1 {
        if *count.add(s) > largestCount {
            largestCount = *count.add(s);
        }
    }

    largestCount
}

/// Store histogram into 4 intermediate tables, recombined at the end.
/// this design makes better use of OoO cpus,
/// and is noticeably faster when some values are heavily repeated.
/// But it needs some additional workspace for intermediate tables.
/// `workSpace` must be a U32 table of size >= HIST_WKSP_SIZE_U32.
///
/// # Returns
///
/// largest histogram frequency, or an error code (notably when
/// histogram's alphabet is larger than *maxSymbolValuePtr)
unsafe fn HIST_count_parallel_wksp(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: &mut u8,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
    check: HIST_checkInput_e,
    workSpace: *mut u32,
) -> size_t {
    let mut ip = source as *const u8;
    let iend = ip.add(sourceSize);
    let countSize = (usize::from(*maxSymbolValuePtr) + 1) * size_of::<core::ffi::c_uint>();
    let mut max = 0;
    let Counting1 = workSpace;
    let Counting2 = Counting1.add(256);
    let Counting3 = Counting2.add(256);
    let Counting4 = Counting3.add(256);

    // safety checks
    if sourceSize == 0 {
        ptr::write_bytes(count as *mut u8, 0, countSize);
        *maxSymbolValuePtr = 0;
        return 0;
    }
    ptr::write_bytes(
        workSpace as *mut u8,
        0,
        ((4 * 256) as core::ffi::c_ulong)
            .wrapping_mul(size_of::<core::ffi::c_uint>() as core::ffi::c_ulong)
            as libc::size_t,
    );

    // by stripes of 16 bytes
    let mut cached = MEM_read32(ip as *const core::ffi::c_void);
    ip = ip.add(4);
    while ip < iend.sub(15) {
        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        *Counting1.offset(c3 as isize) += 1;
        *Counting2.offset(c2 as isize) += 1;
        *Counting3.offset(c1 as isize) += 1;
        *Counting4.offset(c0 as isize) += 1;

        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        *Counting1.offset(c3 as isize) += 1;
        *Counting2.offset(c2 as isize) += 1;
        *Counting3.offset(c1 as isize) += 1;
        *Counting4.offset(c0 as isize) += 1;

        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        *Counting1.offset(c3 as isize) += 1;
        *Counting2.offset(c2 as isize) += 1;
        *Counting3.offset(c1 as isize) += 1;
        *Counting4.offset(c0 as isize) += 1;

        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        *Counting1.offset(c3 as isize) += 1;
        *Counting2.offset(c2 as isize) += 1;
        *Counting3.offset(c1 as isize) += 1;
        *Counting4.offset(c0 as isize) += 1;
    }
    ip = ip.sub(4);

    // finish last symbols
    while ip < iend {
        *Counting1.offset(*ip as isize) += 1;
        ip = ip.add(1);
    }

    for s in 0u32..256 {
        *Counting1.offset(s as isize) += *Counting2.offset(s as isize)
            + *Counting3.offset(s as isize)
            + *Counting4.offset(s as isize);
        if *Counting1.offset(s as isize) > max {
            max = *Counting1.offset(s as isize);
        }
    }

    // `sourceSize` is non-zero, so at least one symbol has a non-zero count
    let mut maxSymbolValue = u8::MAX;
    while *Counting1.add(usize::from(maxSymbolValue)) == 0 {
        maxSymbolValue -= 1;
    }
    if check != 0 && maxSymbolValue > *maxSymbolValuePtr {
        return Error::maxSymbolValue_tooSmall.to_error_code();
    }
    *maxSymbolValuePtr = maxSymbolValue;
    core::ptr::copy(Counting1 as *const u8, count as *mut u8, countSize);

    max as size_t
}

/// Same as [`HIST_countFast`], but using an externally provided scratch buffer.
/// `workSpace` is a writable buffer which must be 4-bytes aligned,
/// `workSpaceSize` must be >= HIST_WKSP_SIZE
pub unsafe fn HIST_countFast_wksp(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: &mut u8,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
    workSpace: *mut core::ffi::c_void,
    workSpaceSize: size_t,
) -> size_t {
    if sourceSize < HIST_FAST_THRESHOLD as size_t {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as size_t;
    }
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
        trustInput,
        workSpace as *mut u32,
    )
}

/// Same as [`HIST_count`], but using an externally provided scratch buffer.
/// `workSpace` size must be table of >= HIST_WKSP_SIZE_U32 unsigned
pub unsafe fn HIST_count_wksp(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: &mut u8,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
    workSpace: *mut core::ffi::c_void,
    workSpaceSize: size_t,
) -> size_t {
    if workSpace as size_t & 3 != 0 {
        // must be aligned on 4-bytes boundaries
        return Error::GENERIC.to_error_code();
    }
    if workSpaceSize < HIST_WKSP_SIZE {
        return Error::workSpace_tooSmall.to_error_code();
    }
    if *maxSymbolValuePtr < u8::MAX {
        return HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            checkMaxSymbolValue,
            workSpace as *mut u32,
        );
    }
    *maxSymbolValuePtr = u8::MAX;
    HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        workSpace,
        workSpaceSize,
    )
}

/// fast variant (unsafe : won't check if src contains values beyond count[] limit)
pub unsafe fn HIST_countFast(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: &mut u8,
    source: *const core::ffi::c_void,
    sourceSize: size_t,
) -> size_t {
    let mut tmpCounters: [core::ffi::c_uint; HIST_WKSP_SIZE_U32] = [0; HIST_WKSP_SIZE_U32];
    HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr() as *mut core::ffi::c_void,
        size_of::<[core::ffi::c_uint; HIST_WKSP_SIZE_U32]>(),
    )
}

pub unsafe fn HIST_count(
    count: *mut core::ffi::c_uint,
    maxSymbolValuePtr: &mut u8,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let mut tmpCounters: [core::ffi::c_uint; HIST_WKSP_SIZE_U32] = [0; HIST_WKSP_SIZE_U32];
    HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr() as *mut core::ffi::c_void,
        size_of::<[core::ffi::c_uint; HIST_WKSP_SIZE_U32]>(),
    )
}
