use core::ptr;

use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::MEM_read32;
pub const HIST_WKSP_SIZE_U32: usize = 1024;
pub const HIST_WKSP_SIZE: size_t =
    (HIST_WKSP_SIZE_U32 as size_t).wrapping_mul(size_of::<core::ffi::c_uint>());
pub const HIST_FAST_THRESHOLD: core::ffi::c_int = 1500;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CheckInput {
    Trust,
    CheckMaxSymbolValue,
}

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
    check: CheckInput,
    workSpace: &mut [u32; 1024],
) -> size_t {
    let mut ip = source as *const u8;
    let iend = ip.add(sourceSize);
    let countSize = (usize::from(*maxSymbolValuePtr) + 1) * size_of::<core::ffi::c_uint>();
    let mut max = 0;

    let ([Counting1, Counting2, Counting3, Counting4], &mut []) = workSpace.as_chunks_mut::<256>()
    else {
        unreachable!()
    };

    // safety checks
    if sourceSize == 0 {
        ptr::write_bytes(count as *mut u8, 0, countSize);
        *maxSymbolValuePtr = 0;
        return 0;
    }

    // by stripes of 16 bytes
    let mut cached = MEM_read32(ip as *const core::ffi::c_void);
    ip = ip.add(4);
    while ip < iend.sub(15) {
        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        Counting1[usize::from(c3)] += 1;
        Counting2[usize::from(c2)] += 1;
        Counting3[usize::from(c1)] += 1;
        Counting4[usize::from(c0)] += 1;

        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        Counting1[usize::from(c3)] += 1;
        Counting2[usize::from(c2)] += 1;
        Counting3[usize::from(c1)] += 1;
        Counting4[usize::from(c0)] += 1;

        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        Counting1[usize::from(c3)] += 1;
        Counting2[usize::from(c2)] += 1;
        Counting3[usize::from(c1)] += 1;
        Counting4[usize::from(c0)] += 1;

        let [c3, c2, c1, c0] = cached.to_le_bytes();
        cached = MEM_read32(ip as *const core::ffi::c_void);
        ip = ip.add(4);
        Counting1[usize::from(c3)] += 1;
        Counting2[usize::from(c2)] += 1;
        Counting3[usize::from(c1)] += 1;
        Counting4[usize::from(c0)] += 1;
    }
    ip = ip.sub(4);

    // finish last symbols
    while ip < iend {
        Counting1[usize::from(*ip)] += 1;
        ip = ip.add(1);
    }

    for s in 0..256 {
        Counting1[s] += Counting2[s] + Counting3[s] + Counting4[s];
        max = Ord::max(max, Counting1[s]);
    }

    // `sourceSize` is non-zero, so at least one symbol has a non-zero count
    let mut maxSymbolValue = u8::MAX;
    let mut it = Counting1.iter().rev();
    while let Some(0) = it.next() {
        maxSymbolValue -= 1;
    }

    if check != CheckInput::Trust && maxSymbolValue > *maxSymbolValuePtr {
        return Error::maxSymbolValue_tooSmall.to_error_code();
    }
    *maxSymbolValuePtr = maxSymbolValue;
    core::ptr::copy(workSpace.as_ptr().cast::<u8>(), count as *mut u8, countSize);

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

    // SAFETY: we've validated the length, and the memory is initialized.
    unsafe { core::ptr::write_bytes(workSpace, 0u8, HIST_WKSP_SIZE) };
    let workspace = unsafe { &mut *workSpace.cast::<[u32; HIST_WKSP_SIZE_U32]>() };

    HIST_count_parallel_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        CheckInput::Trust,
        workspace,
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
        // SAFETY: we've validated the length, and the memory is initialized.
        unsafe { core::ptr::write_bytes(workSpace, 0u8, HIST_WKSP_SIZE) };
        let workspace = unsafe { &mut *workSpace.cast::<[u32; HIST_WKSP_SIZE_U32]>() };

        return HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            CheckInput::CheckMaxSymbolValue,
            workspace,
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
