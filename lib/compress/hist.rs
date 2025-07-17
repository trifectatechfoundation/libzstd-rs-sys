use ::libc;
pub type size_t = std::ffi::c_ulong;
pub type __uint8_t = std::ffi::c_uchar;
pub type __uint32_t = std::ffi::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type BYTE = uint8_t;
pub type U32 = uint32_t;
pub type unalign32 = U32;
pub type C2RustUnnamed = std::ffi::c_uint;
pub const ZSTD_error_maxCode: C2RustUnnamed = 120;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed = 74;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed = 72;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed = 70;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed = 22;
pub const ZSTD_error_corruption_detected: C2RustUnnamed = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed = 12;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed = 10;
pub const ZSTD_error_GENERIC: C2RustUnnamed = 1;
pub const ZSTD_error_no_error: C2RustUnnamed = 0;
pub type HIST_checkInput_e = std::ffi::c_uint;
pub const checkMaxSymbolValue: HIST_checkInput_e = 1;
pub const trustInput: HIST_checkInput_e = 0;
use crate::{MEM_read32, MEM_readLE16, MEM_readLE32, MEM_readLE64, MEM_writeLE32};
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    return (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint;
}
pub const HIST_WKSP_SIZE_U32: std::ffi::c_int = 1024 as std::ffi::c_int;
pub const HIST_WKSP_SIZE: std::ffi::c_ulong = (HIST_WKSP_SIZE_U32 as std::ffi::c_ulong)
    .wrapping_mul(::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong);
pub const HIST_FAST_THRESHOLD: std::ffi::c_int = 1500 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn HIST_isError(mut code: size_t) -> std::ffi::c_uint {
    return ERR_isError(code);
}
#[no_mangle]
pub unsafe extern "C" fn HIST_add(
    mut count: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) {
    let mut ip = src as *const BYTE;
    let end = ip.offset(srcSize as isize);
    while ip < end {
        let fresh0 = ip;
        ip = ip.offset(1);
        let ref mut fresh1 = *count.offset(*fresh0 as isize);
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count_simple(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_uint {
    let mut ip = src as *const BYTE;
    let end = ip.offset(srcSize as isize);
    let mut maxSymbolValue = *maxSymbolValuePtr;
    let mut largestCount = 0 as std::ffi::c_int as std::ffi::c_uint;
    libc::memset(
        count as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong)
            as libc::size_t,
    );
    if srcSize == 0 as std::ffi::c_int as size_t {
        *maxSymbolValuePtr = 0 as std::ffi::c_int as std::ffi::c_uint;
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    while ip < end {
        let fresh2 = ip;
        ip = ip.offset(1);
        let ref mut fresh3 = *count.offset(*fresh2 as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        *fresh3;
    }
    while *count.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
        maxSymbolValue;
    }
    *maxSymbolValuePtr = maxSymbolValue;
    let mut s: U32 = 0;
    s = 0 as std::ffi::c_int as U32;
    while s <= maxSymbolValue {
        if *count.offset(s as isize) > largestCount {
            largestCount = *count.offset(s as isize);
        }
        s = s.wrapping_add(1);
        s;
    }
    return largestCount;
}
unsafe extern "C" fn HIST_count_parallel_wksp(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: size_t,
    mut check: HIST_checkInput_e,
    workSpace: *mut U32,
) -> size_t {
    let mut ip = source as *const BYTE;
    let iend = ip.offset(sourceSize as isize);
    let countSize = ((*maxSymbolValuePtr).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
        as std::ffi::c_ulong)
        .wrapping_mul(::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong);
    let mut max = 0 as std::ffi::c_int as std::ffi::c_uint;
    let Counting1 = workSpace;
    let Counting2 = Counting1.offset(256 as std::ffi::c_int as isize);
    let Counting3 = Counting2.offset(256 as std::ffi::c_int as isize);
    let Counting4 = Counting3.offset(256 as std::ffi::c_int as isize);
    if sourceSize == 0 {
        libc::memset(
            count as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            countSize as libc::size_t,
        );
        *maxSymbolValuePtr = 0 as std::ffi::c_int as std::ffi::c_uint;
        return 0 as std::ffi::c_int as size_t;
    }
    libc::memset(
        workSpace as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ((4 as std::ffi::c_int * 256 as std::ffi::c_int) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong)
            as libc::size_t,
    );
    let mut cached = MEM_read32(ip as *const std::ffi::c_void);
    ip = ip.offset(4 as std::ffi::c_int as isize);
    while ip < iend.offset(-(15 as std::ffi::c_int as isize)) {
        let mut c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4 as std::ffi::c_int as isize);
        let ref mut fresh4 = *Counting1.offset(c as BYTE as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *fresh4;
        let ref mut fresh5 = *Counting2.offset((c >> 8 as std::ffi::c_int) as BYTE as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        *fresh5;
        let ref mut fresh6 = *Counting3.offset((c >> 16 as std::ffi::c_int) as BYTE as isize);
        *fresh6 = (*fresh6).wrapping_add(1);
        *fresh6;
        let ref mut fresh7 = *Counting4.offset((c >> 24 as std::ffi::c_int) as isize);
        *fresh7 = (*fresh7).wrapping_add(1);
        *fresh7;
        c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4 as std::ffi::c_int as isize);
        let ref mut fresh8 = *Counting1.offset(c as BYTE as isize);
        *fresh8 = (*fresh8).wrapping_add(1);
        *fresh8;
        let ref mut fresh9 = *Counting2.offset((c >> 8 as std::ffi::c_int) as BYTE as isize);
        *fresh9 = (*fresh9).wrapping_add(1);
        *fresh9;
        let ref mut fresh10 = *Counting3.offset((c >> 16 as std::ffi::c_int) as BYTE as isize);
        *fresh10 = (*fresh10).wrapping_add(1);
        *fresh10;
        let ref mut fresh11 = *Counting4.offset((c >> 24 as std::ffi::c_int) as isize);
        *fresh11 = (*fresh11).wrapping_add(1);
        *fresh11;
        c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4 as std::ffi::c_int as isize);
        let ref mut fresh12 = *Counting1.offset(c as BYTE as isize);
        *fresh12 = (*fresh12).wrapping_add(1);
        *fresh12;
        let ref mut fresh13 = *Counting2.offset((c >> 8 as std::ffi::c_int) as BYTE as isize);
        *fresh13 = (*fresh13).wrapping_add(1);
        *fresh13;
        let ref mut fresh14 = *Counting3.offset((c >> 16 as std::ffi::c_int) as BYTE as isize);
        *fresh14 = (*fresh14).wrapping_add(1);
        *fresh14;
        let ref mut fresh15 = *Counting4.offset((c >> 24 as std::ffi::c_int) as isize);
        *fresh15 = (*fresh15).wrapping_add(1);
        *fresh15;
        c = cached;
        cached = MEM_read32(ip as *const std::ffi::c_void);
        ip = ip.offset(4 as std::ffi::c_int as isize);
        let ref mut fresh16 = *Counting1.offset(c as BYTE as isize);
        *fresh16 = (*fresh16).wrapping_add(1);
        *fresh16;
        let ref mut fresh17 = *Counting2.offset((c >> 8 as std::ffi::c_int) as BYTE as isize);
        *fresh17 = (*fresh17).wrapping_add(1);
        *fresh17;
        let ref mut fresh18 = *Counting3.offset((c >> 16 as std::ffi::c_int) as BYTE as isize);
        *fresh18 = (*fresh18).wrapping_add(1);
        *fresh18;
        let ref mut fresh19 = *Counting4.offset((c >> 24 as std::ffi::c_int) as isize);
        *fresh19 = (*fresh19).wrapping_add(1);
        *fresh19;
    }
    ip = ip.offset(-(4 as std::ffi::c_int as isize));
    while ip < iend {
        let fresh20 = ip;
        ip = ip.offset(1);
        let ref mut fresh21 = *Counting1.offset(*fresh20 as isize);
        *fresh21 = (*fresh21).wrapping_add(1);
        *fresh21;
    }
    let mut s: U32 = 0;
    s = 0 as std::ffi::c_int as U32;
    while s < 256 as std::ffi::c_int as U32 {
        let ref mut fresh22 = *Counting1.offset(s as isize);
        *fresh22 = (*fresh22).wrapping_add(
            (*Counting2.offset(s as isize))
                .wrapping_add(*Counting3.offset(s as isize))
                .wrapping_add(*Counting4.offset(s as isize)),
        );
        if *Counting1.offset(s as isize) > max {
            max = *Counting1.offset(s as isize);
        }
        s = s.wrapping_add(1);
        s;
    }
    let mut maxSymbolValue = 255 as std::ffi::c_int as std::ffi::c_uint;
    while *Counting1.offset(maxSymbolValue as isize) == 0 {
        maxSymbolValue = maxSymbolValue.wrapping_sub(1);
        maxSymbolValue;
    }
    if check as std::ffi::c_uint != 0 && maxSymbolValue > *maxSymbolValuePtr {
        return -(ZSTD_error_maxSymbolValue_tooSmall as std::ffi::c_int) as size_t;
    }
    *maxSymbolValuePtr = maxSymbolValue;
    libc::memmove(
        count as *mut std::ffi::c_void,
        Counting1 as *const std::ffi::c_void,
        countSize as libc::size_t,
    );
    return max as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast_wksp(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut workSpaceSize: size_t,
) -> size_t {
    if sourceSize < HIST_FAST_THRESHOLD as size_t {
        return HIST_count_simple(count, maxSymbolValuePtr, source, sourceSize) as size_t;
    }
    if workSpace as size_t & 3 as std::ffi::c_int as size_t != 0 {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if workSpaceSize < HIST_WKSP_SIZE {
        return -(ZSTD_error_workSpace_tooSmall as std::ffi::c_int) as size_t;
    }
    return HIST_count_parallel_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        trustInput,
        workSpace as *mut U32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count_wksp(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut workSpaceSize: size_t,
) -> size_t {
    if workSpace as size_t & 3 as std::ffi::c_int as size_t != 0 {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if workSpaceSize < HIST_WKSP_SIZE {
        return -(ZSTD_error_workSpace_tooSmall as std::ffi::c_int) as size_t;
    }
    if *maxSymbolValuePtr < 255 as std::ffi::c_int as std::ffi::c_uint {
        return HIST_count_parallel_wksp(
            count,
            maxSymbolValuePtr,
            source,
            sourceSize,
            checkMaxSymbolValue,
            workSpace as *mut U32,
        );
    }
    *maxSymbolValuePtr = 255 as std::ffi::c_int as std::ffi::c_uint;
    return HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        workSpace,
        workSpaceSize,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_countFast(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut source: *const std::ffi::c_void,
    mut sourceSize: size_t,
) -> size_t {
    let mut tmpCounters: [std::ffi::c_uint; 1024] = [0; 1024];
    return HIST_countFast_wksp(
        count,
        maxSymbolValuePtr,
        source,
        sourceSize,
        tmpCounters.as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[std::ffi::c_uint; 1024]>() as std::ffi::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HIST_count(
    mut count: *mut std::ffi::c_uint,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut tmpCounters: [std::ffi::c_uint; 1024] = [0; 1024];
    return HIST_count_wksp(
        count,
        maxSymbolValuePtr,
        src,
        srcSize,
        tmpCounters.as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[std::ffi::c_uint; 1024]>() as std::ffi::c_ulong,
    );
}
