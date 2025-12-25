use core::ptr;
use std::ffi::CStr;
use std::io;

use libc::{
    abort, calloc, exit, fclose, fflush, fopen, fprintf, fread, free, malloc, memcpy, setpriority,
    size_t, strlen, strrchr, FILE, PRIO_PROCESS,
};
use libzstd_rs_sys::internal::ZSTD_XXH64;
use libzstd_rs_sys::lib::common::zstd_common::{ZSTD_getErrorName, ZSTD_isError};
use libzstd_rs_sys::lib::compress::zstd_compress::{
    ZSTD_CCtx, ZSTD_CCtx_loadDictionary, ZSTD_CCtx_reset, ZSTD_CCtx_setParameter, ZSTD_compress2,
    ZSTD_compressBound, ZSTD_createCCtx, ZSTD_freeCCtx, ZSTD_maxCLevel, ZSTD_sizeof_CCtx,
};
use libzstd_rs_sys::lib::decompress::zstd_decompress::{
    ZSTD_DCtx_loadDictionary, ZSTD_DCtx_reset, ZSTD_createDCtx, ZSTD_decompressStream,
    ZSTD_findDecompressedSize, ZSTD_freeDCtx,
};
use libzstd_rs_sys::lib::decompress::ZSTD_DCtx;
use libzstd_rs_sys::lib::zstd::*;

use crate::benchfn::{
    BMK_benchParams_t, BMK_benchTimedFn, BMK_createTimedFnState, BMK_extract_runTime,
    BMK_freeTimedFnState, BMK_isCompleted_TimedFn, BMK_isSuccessful_runOutcome, BMK_timedFnState_t,
};
use crate::datagen::RDG_genBuffer;
use crate::lorem::LOREM_genBuffer;
use crate::timefn::UTIL_support_MT_measurements;
use crate::util::{UTIL_getFileSize, UTIL_getTotalFileSize, UTIL_isDirectory};

extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_benchResult_t {
    pub cSize: size_t,
    pub cSpeed: core::ffi::c_ulonglong,
    pub dSpeed: core::ffi::c_ulonglong,
    pub cMem: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_benchOutcome_t {
    pub internal_never_use_directly: BMK_benchResult_t,
    pub tag: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_advancedParams_t {
    pub mode: BMK_mode_t,
    pub nbSeconds: core::ffi::c_uint,
    pub chunkSizeMax: size_t,
    pub targetCBlockSize: size_t,
    pub nbWorkers: core::ffi::c_int,
    pub realTime: core::ffi::c_uint,
    pub additionalParam: core::ffi::c_int,
    pub ldmFlag: core::ffi::c_int,
    pub ldmMinMatch: core::ffi::c_int,
    pub ldmHashLog: core::ffi::c_int,
    pub ldmBucketSizeLog: core::ffi::c_int,
    pub ldmHashRateLog: core::ffi::c_int,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
    pub useRowMatchFinder: core::ffi::c_int,
}
pub type BMK_mode_t = core::ffi::c_uint;
pub const BMK_compressOnly: BMK_mode_t = 2;
pub const BMK_decodeOnly: BMK_mode_t = 1;
pub const BMK_both: BMK_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_initDCtxArgs {
    pub dctx: *mut ZSTD_DCtx,
    pub dictBuffer: *const core::ffi::c_void,
    pub dictBufferSize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BMK_initCCtxArgs {
    pub cctx: *mut ZSTD_CCtx,
    pub dictBuffer: *const core::ffi::c_void,
    pub dictBufferSize: size_t,
    pub cLevel: core::ffi::c_int,
    pub comprParams: *const ZSTD_compressionParameters,
    pub adv: *const BMK_advancedParams_t,
}
pub const BMK_TIMETEST_DEFAULT_S: core::ffi::c_int = 3;
pub const UTIL_FILESIZE_UNKNOWN: core::ffi::c_int = -(1);
pub const MB_UNIT: core::ffi::c_int = 1000000;
pub const TIMELOOP_NANOSEC: core::ffi::c_ulonglong =
    (1 as core::ffi::c_ulonglong).wrapping_mul(1000000000);
pub const BMK_RUNTEST_DEFAULT_MS: core::ffi::c_int = 1000;
static maxMemory: usize = if ::core::mem::size_of::<size_t>() == 4 {
    2 * (1 << 30) - 64 * (1 << 20)
} else {
    1usize << (::core::mem::size_of::<usize>() * 8 - 31)
};
pub const DEBUG: core::ffi::c_int = 0;
unsafe fn uintSize(mut value: core::ffi::c_uint) -> size_t {
    let mut size = 1 as size_t;
    while value >= 10 {
        size = size.wrapping_add(1);
        value = value.wrapping_div(10);
    }
    size
}
unsafe fn writeUint_varLen(
    buffer: *mut core::ffi::c_char,
    capacity: size_t,
    mut value: core::ffi::c_uint,
) {
    let mut endPos = uintSize(value) as core::ffi::c_int - 1;
    assert!(uintSize(value) >= 1);
    assert!(uintSize(value) < capacity);
    while endPos >= 0 {
        let c = ('0' as i32 + core::ffi::c_int::from(value.wrapping_rem(10) as core::ffi::c_char))
            as core::ffi::c_char;
        let fresh1 = endPos;
        endPos -= 1;
        *buffer.offset(fresh1 as isize) = c;
        value = value.wrapping_div(10);
    }
}
unsafe fn formatString_u(
    buffer: *mut core::ffi::c_char,
    buffer_size: size_t,
    formatString: *const core::ffi::c_char,
    value: core::ffi::c_uint,
) -> core::ffi::c_int {
    let valueSize = uintSize(value);
    let mut written = 0;
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while core::ffi::c_int::from(*formatString.offset(i as isize)) != '\0' as i32
        && written < buffer_size.wrapping_sub(1)
    {
        if core::ffi::c_int::from(*formatString.offset(i as isize)) != '%' as i32 {
            let fresh2 = written;
            written = written.wrapping_add(1);
            *buffer.add(fresh2) = *formatString.offset(i as isize);
        } else {
            i += 1;
            if core::ffi::c_int::from(*formatString.offset(i as isize)) == 'u' as i32 {
                if written.wrapping_add(valueSize) >= buffer_size {
                    abort();
                }
                writeUint_varLen(
                    buffer.add(written),
                    buffer_size.wrapping_sub(written),
                    value,
                );
                written = written.wrapping_add(valueSize);
            } else if core::ffi::c_int::from(*formatString.offset(i as isize)) == '%' as i32 {
                let fresh3 = written;
                written = written.wrapping_add(1);
                *buffer.add(fresh3) = '%' as i32 as core::ffi::c_char;
            } else {
                abort();
            }
        }
        i += 1;
    }
    if written < buffer_size {
        *buffer.add(written) = '\0' as i32 as core::ffi::c_char;
    } else {
        abort();
    }
    written as core::ffi::c_int
}
pub unsafe fn BMK_initAdvancedParams() -> BMK_advancedParams_t {
    {
        BMK_advancedParams_t {
            mode: BMK_both,
            nbSeconds: BMK_TIMETEST_DEFAULT_S as core::ffi::c_uint,
            chunkSizeMax: 0,
            targetCBlockSize: 0,
            nbWorkers: 0,
            realTime: 0,
            additionalParam: 0,
            ldmFlag: 0,
            ldmMinMatch: 0,
            ldmHashLog: 0,
            ldmBucketSizeLog: 0,
            ldmHashRateLog: 0,
            literalCompressionMode: ZSTD_ParamSwitch_e::ZSTD_ps_auto,
            useRowMatchFinder: 0,
        }
    }
}
unsafe fn BMK_initCCtx(
    ctx: *mut ZSTD_CCtx,
    dictBuffer: *const core::ffi::c_void,
    dictBufferSize: size_t,
    cLevel: core::ffi::c_int,
    comprParams: *const ZSTD_compressionParameters,
    adv: *const BMK_advancedParams_t,
) {
    ZSTD_CCtx_reset(ctx, ZSTD_ResetDirective::ZSTD_reset_session_and_parameters);
    if (*adv).nbWorkers == 1 {
        let zerr = ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_nbWorkers, 0);
        if ZSTD_isError(zerr) != 0 {
            fprintf(
                stderr,
                b"Error : \0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
            fprintf(
                stderr,
                b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_nbWorkers, 0)\0" as *const u8
                    as *const core::ffi::c_char,
                ZSTD_getErrorName(zerr),
            );
            fflush(core::ptr::null_mut());
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            fflush(core::ptr::null_mut());
            exit(1);
        }
    } else {
        let zerr_0 =
            ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_nbWorkers, (*adv).nbWorkers);
        if ZSTD_isError(zerr_0) != 0 {
            fprintf(
                stderr,
                b"Error : \0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
            fprintf(
                stderr,
                b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
                b"ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_nbWorkers, adv->nbWorkers)\0"
                    as *const u8 as *const core::ffi::c_char,
                ZSTD_getErrorName(zerr_0),
            );
            fflush(core::ptr::null_mut());
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            fflush(core::ptr::null_mut());
            exit(1);
        }
    }
    let zerr_1 = ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_compressionLevel, cLevel);
    if ZSTD_isError(zerr_1) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_compressionLevel, cLevel)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_1),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_2 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_experimentalParam14,
        (*adv).useRowMatchFinder,
    );
    if ZSTD_isError(zerr_2) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_useRowMatchFinder, adv->useRowMatchFinder)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_2),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_3 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching,
        (*adv).ldmFlag,
    );
    if ZSTD_isError(zerr_3) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching, adv->ldmFlag)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_3),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_4 =
        ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_ldmMinMatch, (*adv).ldmMinMatch);
    if ZSTD_isError(zerr_4) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_ldmMinMatch, adv->ldmMinMatch)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_4),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_5 = ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_ldmHashLog, (*adv).ldmHashLog);
    if ZSTD_isError(zerr_5) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter(ctx, ZSTD_cParameter::ZSTD_c_ldmHashLog, adv->ldmHashLog)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_5),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_6 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog,
        (*adv).ldmBucketSizeLog,
    );
    if ZSTD_isError(zerr_6) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog, adv->ldmBucketSizeLog)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_6),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_7 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_ldmHashRateLog,
        (*adv).ldmHashRateLog,
    );
    if ZSTD_isError(zerr_7) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_ldmHashRateLog, adv->ldmHashRateLog)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_7),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_8 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_windowLog,
        (*comprParams).windowLog as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_8) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_windowLog, (int)comprParams->windowLog)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_8),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_9 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_hashLog,
        (*comprParams).hashLog as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_9) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_hashLog, (int)comprParams->hashLog)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_9),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_10 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_chainLog,
        (*comprParams).chainLog as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_10) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_chainLog, (int)comprParams->chainLog)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_10),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_11 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_searchLog,
        (*comprParams).searchLog as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_11) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_searchLog, (int)comprParams->searchLog)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_11),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_12 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_minMatch,
        (*comprParams).minMatch as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_12) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_minMatch, (int)comprParams->minMatch)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_12),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_13 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_targetLength,
        (*comprParams).targetLength as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_13) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_targetLength, (int)comprParams->targetLength)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_13),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_14 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_experimentalParam5,
        (*adv).literalCompressionMode.to_i32(),
    );
    if ZSTD_isError(zerr_14) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_literalCompressionMode, (int)adv->literalCompressionMode)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_14),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_15 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_strategy,
        (*comprParams).strategy as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_15) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_strategy, (int)comprParams->strategy)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_15),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_16 = ZSTD_CCtx_setParameter(
        ctx,
        ZSTD_cParameter::ZSTD_c_targetCBlockSize,
        (*adv).targetCBlockSize as core::ffi::c_int,
    );
    if ZSTD_isError(zerr_16) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_setParameter( ctx, ZSTD_cParameter::ZSTD_c_targetCBlockSize, (int)adv->targetCBlockSize)\0"
                as *const u8 as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_16),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_17 = ZSTD_CCtx_loadDictionary(ctx, dictBuffer, dictBufferSize);
    if ZSTD_isError(zerr_17) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_CCtx_loadDictionary(ctx, dictBuffer, dictBufferSize)\0" as *const u8
                as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_17),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
}
unsafe fn BMK_initDCtx(
    dctx: *mut ZSTD_DCtx,
    dictBuffer: *const core::ffi::c_void,
    dictBufferSize: size_t,
) {
    let zerr = ZSTD_DCtx_reset(dctx, ZSTD_ResetDirective::ZSTD_reset_session_and_parameters);
    if ZSTD_isError(zerr) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_DCtx_reset(dctx, ZSTD_reset_session_and_parameters)\0" as *const u8
                as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
    let zerr_0 = ZSTD_DCtx_loadDictionary(dctx, dictBuffer, dictBufferSize);
    if ZSTD_isError(zerr_0) != 0 {
        fprintf(
            stderr,
            b"Error : \0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
        fprintf(
            stderr,
            b"%s failed : %s\0" as *const u8 as *const core::ffi::c_char,
            b"ZSTD_DCtx_loadDictionary(dctx, dictBuffer, dictBufferSize)\0" as *const u8
                as *const core::ffi::c_char,
            ZSTD_getErrorName(zerr_0),
        );
        fflush(core::ptr::null_mut());
        fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
        fflush(core::ptr::null_mut());
        exit(1);
    }
}
unsafe fn local_initCCtx(payload: *mut core::ffi::c_void) -> size_t {
    let ag = payload as *mut BMK_initCCtxArgs;
    BMK_initCCtx(
        (*ag).cctx,
        (*ag).dictBuffer,
        (*ag).dictBufferSize,
        (*ag).cLevel,
        (*ag).comprParams,
        (*ag).adv,
    );
    0
}
unsafe fn local_initDCtx(payload: *mut core::ffi::c_void) -> size_t {
    let ag = payload as *mut BMK_initDCtxArgs;
    BMK_initDCtx((*ag).dctx, (*ag).dictBuffer, (*ag).dictBufferSize);
    0
}
unsafe fn local_defaultCompress(
    srcBuffer: *const core::ffi::c_void,
    srcSize: size_t,
    dstBuffer: *mut core::ffi::c_void,
    dstSize: size_t,
    addArgs: *mut core::ffi::c_void,
) -> size_t {
    let cctx = addArgs as *mut ZSTD_CCtx;
    ZSTD_compress2(cctx, dstBuffer, dstSize, srcBuffer, srcSize)
}
unsafe fn local_defaultDecompress(
    srcBuffer: *const core::ffi::c_void,
    srcSize: size_t,
    dstBuffer: *mut core::ffi::c_void,
    dstCapacity: size_t,
    addArgs: *mut core::ffi::c_void,
) -> size_t {
    let mut moreToFlush = 1;
    let dctx = addArgs as *mut ZSTD_DCtx;
    let mut in_0 = ZSTD_inBuffer_s {
        src: core::ptr::null::<core::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    let mut out = ZSTD_outBuffer_s {
        dst: core::ptr::null_mut::<core::ffi::c_void>(),
        size: 0,
        pos: 0,
    };
    in_0.src = srcBuffer;
    in_0.size = srcSize;
    in_0.pos = 0;
    out.dst = dstBuffer;
    out.size = dstCapacity;
    out.pos = 0;
    while moreToFlush != 0 {
        if out.pos == out.size {
            return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
        }
        moreToFlush = ZSTD_decompressStream(dctx, &mut out, &mut in_0);
        if ZSTD_isError(moreToFlush) != 0 {
            return moreToFlush;
        }
    }
    out.pos
}
pub unsafe fn BMK_isSuccessful_benchOutcome(outcome: BMK_benchOutcome_t) -> core::ffi::c_int {
    core::ffi::c_int::from(outcome.tag == 0)
}
pub unsafe fn BMK_extract_benchResult(outcome: BMK_benchOutcome_t) -> BMK_benchResult_t {
    assert!(outcome.tag == 0);
    outcome.internal_never_use_directly
}
unsafe fn BMK_benchOutcome_error() -> BMK_benchOutcome_t {
    let mut b = BMK_benchOutcome_t {
        internal_never_use_directly: BMK_benchResult_t {
            cSize: 0,
            cSpeed: 0,
            dSpeed: 0,
            cMem: 0,
        },
        tag: 0,
    };
    ptr::write_bytes(
        &mut b as *mut BMK_benchOutcome_t as *mut u8,
        0,
        ::core::mem::size_of::<BMK_benchOutcome_t>(),
    );
    b.tag = 1;
    b
}
unsafe fn BMK_benchOutcome_setValidResult(result: BMK_benchResult_t) -> BMK_benchOutcome_t {
    let mut b = BMK_benchOutcome_t {
        internal_never_use_directly: BMK_benchResult_t {
            cSize: 0,
            cSpeed: 0,
            dSpeed: 0,
            cMem: 0,
        },
        tag: 0,
    };
    b.tag = 0;
    b.internal_never_use_directly = result;
    b
}
unsafe fn BMK_benchMemAdvancedNoAlloc(
    srcPtrs: *mut *const core::ffi::c_void,
    srcSizes: *mut size_t,
    cPtrs: *mut *mut core::ffi::c_void,
    cCapacities: *mut size_t,
    cSizes: *mut size_t,
    resPtrs: *mut *mut core::ffi::c_void,
    resSizes: *mut size_t,
    resultBufferPtr: *mut *mut core::ffi::c_void,
    compressedBuffer: *mut core::ffi::c_void,
    maxCompressedSize: size_t,
    timeStateCompress: *mut BMK_timedFnState_t,
    timeStateDecompress: *mut BMK_timedFnState_t,
    srcBuffer: *const core::ffi::c_void,
    mut srcSize: size_t,
    fileSizes: *const size_t,
    nbFiles: core::ffi::c_uint,
    cLevel: core::ffi::c_int,
    comprParams: *const ZSTD_compressionParameters,
    dictBuffer: *const core::ffi::c_void,
    dictBufferSize: size_t,
    cctx: *mut ZSTD_CCtx,
    dctx: *mut ZSTD_DCtx,
    displayLevel: core::ffi::c_int,
    mut displayName: *const core::ffi::c_char,
    adv: *const BMK_advancedParams_t,
) -> BMK_benchOutcome_t {
    let chunkSizeMax = (if (*adv).chunkSizeMax >= 32
        && (*adv).mode as core::ffi::c_uint
            != BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
    {
        (*adv).chunkSizeMax
    } else {
        srcSize
    })
    .wrapping_add(core::ffi::c_int::from(srcSize == 0) as size_t);
    let mut benchResult = BMK_benchResult_t {
        cSize: 0,
        cSpeed: 0,
        dSpeed: 0,
        cMem: 0,
    };
    let loadedCompressedSize = srcSize;
    let mut cSize = 0;
    let mut ratio = 0.0f64;
    let mut nbChunks = 0;
    assert!(!cctx.is_null());
    assert!(!dctx.is_null());
    ptr::write_bytes(
        &mut benchResult as *mut BMK_benchResult_t as *mut u8,
        0,
        ::core::mem::size_of::<BMK_benchResult_t>(),
    );
    if strlen(displayName) > 17 {
        displayName = displayName.add((strlen(displayName)).wrapping_sub(17));
    }
    if (*adv).mode as core::ffi::c_uint == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint {
        let mut srcPtr = srcBuffer as *const core::ffi::c_char;
        let mut totalDSize64 = 0u64;
        let mut fileNb: u32 = 0;
        fileNb = 0;
        while fileNb < nbFiles {
            let fSize64 = ZSTD_findDecompressedSize(
                srcPtr as *const core::ffi::c_void,
                *fileSizes.offset(fileNb as isize),
            ) as u64;
            if fSize64 as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_UNKNOWN {
                let mut r = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                ptr::write_bytes(
                    &mut r as *mut BMK_benchOutcome_t as *mut u8,
                    0,
                    ::core::mem::size_of::<BMK_benchOutcome_t>(),
                );
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                        32,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Decompressed size cannot be determined: cannot benchmark\0" as *const u8
                            as *const core::ffi::c_char,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    fflush(core::ptr::null_mut());
                }
                r.tag = 32;
                return r;
            }
            if fSize64 as core::ffi::c_ulonglong == ZSTD_CONTENTSIZE_ERROR {
                let mut r_0 = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                ptr::write_bytes(
                    &mut r_0 as *mut BMK_benchOutcome_t as *mut u8,
                    0,
                    ::core::mem::size_of::<BMK_benchOutcome_t>(),
                );
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                        32,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error while trying to assess decompressed size: data may be invalid\0"
                            as *const u8 as *const core::ffi::c_char,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    fflush(core::ptr::null_mut());
                }
                r_0.tag = 32;
                return r_0;
            }
            totalDSize64 = totalDSize64.wrapping_add(fSize64);
            srcPtr = srcPtr.add(*fileSizes.offset(fileNb as isize));
            fileNb = fileNb.wrapping_add(1);
        }
        let decodedSize = totalDSize64 as size_t;
        assert!(decodedSize as u64 == totalDSize64);
        free(*resultBufferPtr);
        if totalDSize64 > decodedSize as u64 {
            let mut r_1 = BMK_benchOutcome_t {
                internal_never_use_directly: BMK_benchResult_t {
                    cSize: 0,
                    cSpeed: 0,
                    dSpeed: 0,
                    cMem: 0,
                },
                tag: 0,
            };
            ptr::write_bytes(
                &mut r_1 as *mut BMK_benchOutcome_t as *mut u8,
                0,
                ::core::mem::size_of::<BMK_benchOutcome_t>(),
            );
            if displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                    32,
                );
                fflush(core::ptr::null_mut());
            }
            if displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"decompressed size is too large for local system\0" as *const u8
                        as *const core::ffi::c_char,
                );
                fflush(core::ptr::null_mut());
            }
            if displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                fflush(core::ptr::null_mut());
            }
            r_1.tag = 32;
            return r_1;
        }
        *resultBufferPtr = malloc(decodedSize);
        if (*resultBufferPtr).is_null() {
            let mut r_2 = BMK_benchOutcome_t {
                internal_never_use_directly: BMK_benchResult_t {
                    cSize: 0,
                    cSpeed: 0,
                    dSpeed: 0,
                    cMem: 0,
                },
                tag: 0,
            };
            ptr::write_bytes(
                &mut r_2 as *mut BMK_benchOutcome_t as *mut u8,
                0,
                ::core::mem::size_of::<BMK_benchOutcome_t>(),
            );
            if displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                    33,
                );
                fflush(core::ptr::null_mut());
            }
            if displayLevel >= 1 {
                fprintf(
                    stderr,
                    b"allocation error: not enough memory\0" as *const u8
                        as *const core::ffi::c_char,
                );
                fflush(core::ptr::null_mut());
            }
            if displayLevel >= 1 {
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                fflush(core::ptr::null_mut());
            }
            r_2.tag = 33;
            return r_2;
        }
        cSize = srcSize;
        srcSize = decodedSize;
        ratio = srcSize as core::ffi::c_double / cSize as core::ffi::c_double;
    }
    let mut srcPtr_0 = srcBuffer as *const core::ffi::c_char;
    let mut cPtr = compressedBuffer as *mut core::ffi::c_char;
    let mut resPtr = *resultBufferPtr as *mut core::ffi::c_char;
    let mut fileNb_0: u32 = 0;
    let mut chunkID: u32 = 0;
    chunkID = 0;
    fileNb_0 = 0;
    while fileNb_0 < nbFiles {
        let mut reing = *fileSizes.offset(fileNb_0 as isize);
        let nbChunksforThisFile = if (*adv).mode as core::ffi::c_uint
            == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
        {
            1
        } else {
            (reing.wrapping_add(chunkSizeMax.wrapping_sub(1)) / chunkSizeMax) as u32
        };
        let chunkIdEnd = chunkID.wrapping_add(nbChunksforThisFile);
        while chunkID < chunkIdEnd {
            let chunkSize = if reing < chunkSizeMax {
                reing
            } else {
                chunkSizeMax
            };
            let fresh4 = &mut (*srcPtrs.offset(chunkID as isize));
            *fresh4 = srcPtr_0 as *const core::ffi::c_void;
            *srcSizes.offset(chunkID as isize) = chunkSize;
            let fresh5 = &mut (*cPtrs.offset(chunkID as isize));
            *fresh5 = cPtr as *mut core::ffi::c_void;
            *cCapacities.offset(chunkID as isize) = if (*adv).mode as core::ffi::c_uint
                == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
            {
                chunkSize
            } else {
                ZSTD_compressBound(chunkSize)
            };
            let fresh6 = &mut (*resPtrs.offset(chunkID as isize));
            *fresh6 = resPtr as *mut core::ffi::c_void;
            *resSizes.offset(chunkID as isize) = if (*adv).mode as core::ffi::c_uint
                == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
            {
                ZSTD_findDecompressedSize(srcPtr_0 as *const core::ffi::c_void, chunkSize) as size_t
            } else {
                chunkSize
            };
            srcPtr_0 = srcPtr_0.add(chunkSize);
            cPtr = cPtr.add(*cCapacities.offset(chunkID as isize));
            resPtr = resPtr.add(chunkSize);
            reing = reing.wrapping_sub(chunkSize);
            if (*adv).mode as core::ffi::c_uint
                == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
            {
                *cSizes.offset(chunkID as isize) = chunkSize;
                benchResult.cSize = chunkSize;
            }
            chunkID = chunkID.wrapping_add(1);
        }
        fileNb_0 = fileNb_0.wrapping_add(1);
    }
    nbChunks = chunkID;
    if (*adv).mode as core::ffi::c_uint == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint {
        memcpy(compressedBuffer, srcBuffer, loadedCompressedSize);
    } else {
        RDG_genBuffer(compressedBuffer, maxCompressedSize, 0.10f64, 0.50f64, 1);
    }
    if UTIL_support_MT_measurements() == 0 && (*adv).nbWorkers > 1 && displayLevel >= 2 {
        fprintf(
            stdout,
            b"Warning : time measurements may be incorrect in multithreading mode... \n\0"
                as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
    }
    let crcOrig = if (*adv).mode as core::ffi::c_uint
        == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
    {
        0
    } else {
        ZSTD_XXH64(srcBuffer, srcSize, 0)
    };
    let mut marks: [*const core::ffi::c_char; 4] = [
        b" |\0" as *const u8 as *const core::ffi::c_char,
        b" /\0" as *const u8 as *const core::ffi::c_char,
        b" =\0" as *const u8 as *const core::ffi::c_char,
        b" \\\0" as *const u8 as *const core::ffi::c_char,
    ];
    let mut markNb = 0u32;
    let mut compressionCompleted = core::ffi::c_int::from(
        (*adv).mode as core::ffi::c_uint == BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint,
    );
    let mut decompressionCompleted = core::ffi::c_int::from(
        (*adv).mode as core::ffi::c_uint
            == BMK_compressOnly as core::ffi::c_int as core::ffi::c_uint,
    );
    let mut cbp = BMK_benchParams_t {
        benchFn: None,
        benchPayload: core::ptr::null_mut::<core::ffi::c_void>(),
        initFn: None,
        initPayload: core::ptr::null_mut::<core::ffi::c_void>(),
        errorFn: None,
        blockCount: 0,
        srcBuffers: core::ptr::null::<*const core::ffi::c_void>(),
        srcSizes: core::ptr::null::<size_t>(),
        dstBuffers: core::ptr::null::<*mut core::ffi::c_void>(),
        dstCapacities: core::ptr::null::<size_t>(),
        blockResults: core::ptr::null_mut::<size_t>(),
    };
    let mut dbp = BMK_benchParams_t {
        benchFn: None,
        benchPayload: core::ptr::null_mut::<core::ffi::c_void>(),
        initFn: None,
        initPayload: core::ptr::null_mut::<core::ffi::c_void>(),
        errorFn: None,
        blockCount: 0,
        srcBuffers: core::ptr::null::<*const core::ffi::c_void>(),
        srcSizes: core::ptr::null::<size_t>(),
        dstBuffers: core::ptr::null::<*mut core::ffi::c_void>(),
        dstCapacities: core::ptr::null::<size_t>(),
        blockResults: core::ptr::null_mut::<size_t>(),
    };
    let mut cctxprep = BMK_initCCtxArgs {
        cctx: core::ptr::null_mut::<ZSTD_CCtx>(),
        dictBuffer: core::ptr::null::<core::ffi::c_void>(),
        dictBufferSize: 0,
        cLevel: 0,
        comprParams: core::ptr::null::<ZSTD_compressionParameters>(),
        adv: core::ptr::null::<BMK_advancedParams_t>(),
    };
    let mut dctxprep = BMK_initDCtxArgs {
        dctx: core::ptr::null_mut::<ZSTD_DCtx>(),
        dictBuffer: core::ptr::null::<core::ffi::c_void>(),
        dictBufferSize: 0,
    };
    cbp.benchFn = Some(
        local_defaultCompress
            as unsafe fn(
                *const core::ffi::c_void,
                size_t,
                *mut core::ffi::c_void,
                size_t,
                *mut core::ffi::c_void,
            ) -> size_t,
    );
    cbp.benchPayload = cctx as *mut core::ffi::c_void;
    cbp.initFn = Some(local_initCCtx as unsafe fn(*mut core::ffi::c_void) -> size_t);
    cbp.initPayload = &mut cctxprep as *mut BMK_initCCtxArgs as *mut core::ffi::c_void;
    cbp.errorFn = Some(ZSTD_isError as unsafe extern "C" fn(size_t) -> core::ffi::c_uint);
    cbp.blockCount = nbChunks as size_t;
    cbp.srcBuffers = srcPtrs;
    cbp.srcSizes = srcSizes;
    cbp.dstBuffers = cPtrs;
    cbp.dstCapacities = cCapacities;
    cbp.blockResults = cSizes;
    cctxprep.cctx = cctx;
    cctxprep.dictBuffer = dictBuffer;
    cctxprep.dictBufferSize = dictBufferSize;
    cctxprep.cLevel = cLevel;
    cctxprep.comprParams = comprParams;
    cctxprep.adv = adv;
    dbp.benchFn = Some(
        local_defaultDecompress
            as unsafe fn(
                *const core::ffi::c_void,
                size_t,
                *mut core::ffi::c_void,
                size_t,
                *mut core::ffi::c_void,
            ) -> size_t,
    );
    dbp.benchPayload = dctx as *mut core::ffi::c_void;
    dbp.initFn = Some(local_initDCtx as unsafe fn(*mut core::ffi::c_void) -> size_t);
    dbp.initPayload = &mut dctxprep as *mut BMK_initDCtxArgs as *mut core::ffi::c_void;
    dbp.errorFn = Some(ZSTD_isError as unsafe extern "C" fn(size_t) -> core::ffi::c_uint);
    dbp.blockCount = nbChunks as size_t;
    dbp.srcBuffers = cPtrs as *const *const core::ffi::c_void;
    dbp.srcSizes = cSizes;
    dbp.dstBuffers = resPtrs;
    dbp.dstCapacities = resSizes;
    dbp.blockResults = core::ptr::null_mut();
    dctxprep.dctx = dctx;
    dctxprep.dictBuffer = dictBuffer;
    dctxprep.dictBufferSize = dictBufferSize;
    if displayLevel >= 2 {
        fprintf(
            stdout,
            b"\r%70s\r\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
        );
        fflush(core::ptr::null_mut());
    }
    assert!(srcSize < core::ffi::c_uint::MAX as size_t);
    if displayLevel >= 2 {
        fprintf(
            stdout,
            b"%2s-%-17.17s :%10u -> \r\0" as *const u8 as *const core::ffi::c_char,
            *marks.as_mut_ptr().offset(markNb as isize),
            displayName,
            srcSize as core::ffi::c_uint,
        );
        fflush(core::ptr::null_mut());
    }
    while !(compressionCompleted != 0 && decompressionCompleted != 0) {
        if compressionCompleted == 0 {
            let cOutcome = BMK_benchTimedFn(timeStateCompress, cbp);
            if BMK_isSuccessful_runOutcome(cOutcome) == 0 {
                let mut r_3 = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                ptr::write_bytes(
                    &mut r_3 as *mut BMK_benchOutcome_t as *mut u8,
                    0,
                    ::core::mem::size_of::<BMK_benchOutcome_t>(),
                );
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                        30,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"compression error\0" as *const u8 as *const core::ffi::c_char,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    fflush(core::ptr::null_mut());
                }
                r_3.tag = 30;
                return r_3;
            }
            let cResult = BMK_extract_runTime(cOutcome);
            cSize = cResult.sumOfReturn;
            ratio = srcSize as core::ffi::c_double / cSize as core::ffi::c_double;
            let mut newResult = BMK_benchResult_t {
                cSize: 0,
                cSpeed: 0,
                dSpeed: 0,
                cMem: 0,
            };
            newResult.cSpeed =
                (srcSize as core::ffi::c_double * TIMELOOP_NANOSEC as core::ffi::c_double
                    / cResult.nanoSecPerRun) as u64 as core::ffi::c_ulonglong;
            benchResult.cSize = cSize;
            if newResult.cSpeed > benchResult.cSpeed {
                benchResult.cSpeed = newResult.cSpeed;
            }
            let ratioDigits = 1
                + core::ffi::c_int::from(ratio < 100.0f64)
                + core::ffi::c_int::from(ratio < 10.0f64);
            assert!(cSize < core::ffi::c_uint::MAX as size_t);
            if displayLevel >= 2 {
                fprintf(
                    stdout,
                    b"%2s-%-17.17s :%10u ->%10u (x%5.*f), %6.*f MB/s \r\0" as *const u8
                        as *const core::ffi::c_char,
                    *marks.as_mut_ptr().offset(markNb as isize),
                    displayName,
                    srcSize as core::ffi::c_uint,
                    cSize as core::ffi::c_uint,
                    ratioDigits,
                    ratio,
                    if benchResult.cSpeed < (10 * 1000000) as core::ffi::c_ulonglong {
                        2
                    } else {
                        1
                    },
                    benchResult.cSpeed as core::ffi::c_double / 1000000.0,
                );
                fflush(core::ptr::null_mut());
            }
            compressionCompleted = BMK_isCompleted_TimedFn(timeStateCompress);
        }
        if decompressionCompleted == 0 {
            let dOutcome = BMK_benchTimedFn(timeStateDecompress, dbp);
            if BMK_isSuccessful_runOutcome(dOutcome) == 0 {
                let mut r_4 = BMK_benchOutcome_t {
                    internal_never_use_directly: BMK_benchResult_t {
                        cSize: 0,
                        cSpeed: 0,
                        dSpeed: 0,
                        cMem: 0,
                    },
                    tag: 0,
                };
                ptr::write_bytes(
                    &mut r_4 as *mut BMK_benchOutcome_t as *mut u8,
                    0,
                    ::core::mem::size_of::<BMK_benchOutcome_t>(),
                );
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                        30,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"decompression error\0" as *const u8 as *const core::ffi::c_char,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    fflush(core::ptr::null_mut());
                }
                r_4.tag = 30;
                return r_4;
            }
            let dResult = BMK_extract_runTime(dOutcome);
            let newDSpeed = (srcSize as core::ffi::c_double
                * TIMELOOP_NANOSEC as core::ffi::c_double
                / dResult.nanoSecPerRun) as u64;
            if newDSpeed as core::ffi::c_ulonglong > benchResult.dSpeed {
                benchResult.dSpeed = newDSpeed as core::ffi::c_ulonglong;
            }
            let ratioDigits_0 = 1
                + core::ffi::c_int::from(ratio < 100.0f64)
                + core::ffi::c_int::from(ratio < 10.0f64);
            if displayLevel >= 2 {
                fprintf(
                    stdout,
                    b"%2s-%-17.17s :%10u ->%10u (x%5.*f), %6.*f MB/s, %6.1f MB/s\r\0" as *const u8
                        as *const core::ffi::c_char,
                    *marks.as_mut_ptr().offset(markNb as isize),
                    displayName,
                    srcSize as core::ffi::c_uint,
                    cSize as core::ffi::c_uint,
                    ratioDigits_0,
                    ratio,
                    if benchResult.cSpeed < (10 * 1000000) as core::ffi::c_ulonglong {
                        2
                    } else {
                        1
                    },
                    benchResult.cSpeed as core::ffi::c_double / 1000000.0,
                    benchResult.dSpeed as core::ffi::c_double / 1000000.0,
                );
                fflush(core::ptr::null_mut());
            }
            decompressionCompleted = BMK_isCompleted_TimedFn(timeStateDecompress);
        }
        markNb = markNb.wrapping_add(1) % NB_MARKS as u32;
    }
    let resultBuffer = *resultBufferPtr as *const u8;
    let crcCheck = ZSTD_XXH64(resultBuffer as *const core::ffi::c_void, srcSize, 0);
    if (*adv).mode as core::ffi::c_uint == BMK_both as core::ffi::c_int as core::ffi::c_uint
        && crcOrig != crcCheck
    {
        let mut u: size_t = 0;
        fprintf(
            stderr,
            b"!!! WARNING !!! %14s : Invalid Checksum : %x != %x   \n\0" as *const u8
                as *const core::ffi::c_char,
            displayName,
            crcOrig as core::ffi::c_uint,
            crcCheck as core::ffi::c_uint,
        );
        fflush(core::ptr::null_mut());
        u = 0;
        while u < srcSize {
            if core::ffi::c_int::from(*(srcBuffer as *const u8).add(u))
                != core::ffi::c_int::from(*resultBuffer.add(u))
            {
                let mut segNb: core::ffi::c_uint = 0;
                let mut bNb: core::ffi::c_uint = 0;
                let mut pos: core::ffi::c_uint = 0;
                let mut bacc = 0 as size_t;
                fprintf(
                    stderr,
                    b"Decoding error at pos %u \0" as *const u8 as *const core::ffi::c_char,
                    u as core::ffi::c_uint,
                );
                fflush(core::ptr::null_mut());
                segNb = 0;
                while segNb < nbChunks {
                    if bacc.wrapping_add(*srcSizes.offset(segNb as isize)) > u {
                        break;
                    }
                    bacc = bacc.wrapping_add(*srcSizes.offset(segNb as isize));
                    segNb = segNb.wrapping_add(1);
                }
                pos = u.wrapping_sub(bacc) as u32;
                bNb = pos.wrapping_div((128 * ((1) << 10)) as core::ffi::c_uint);
                fprintf(
                    stderr,
                    b"(sample %u, chunk %u, pos %u) \n\0" as *const u8 as *const core::ffi::c_char,
                    segNb,
                    bNb,
                    pos,
                );
                fflush(core::ptr::null_mut());
                let lowest = if u > 5 { 5 } else { u };
                let mut n: size_t = 0;
                fprintf(
                    stderr,
                    b"origin: \0" as *const u8 as *const core::ffi::c_char,
                );
                fflush(core::ptr::null_mut());
                n = lowest;
                while n > 0 {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const core::ffi::c_char,
                        core::ffi::c_int::from(*(srcBuffer as *const u8).add(u.wrapping_sub(n))),
                    );
                    fflush(core::ptr::null_mut());
                    n = n.wrapping_sub(1);
                }
                fprintf(
                    stderr,
                    b" :%02X:  \0" as *const u8 as *const core::ffi::c_char,
                    core::ffi::c_int::from(*(srcBuffer as *const u8).add(u)),
                );
                fflush(core::ptr::null_mut());
                n = 1;
                while n < 3 {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const core::ffi::c_char,
                        core::ffi::c_int::from(*(srcBuffer as *const u8).add(u.wrapping_add(n))),
                    );
                    fflush(core::ptr::null_mut());
                    n = n.wrapping_add(1);
                }
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                fflush(core::ptr::null_mut());
                fprintf(
                    stderr,
                    b"decode: \0" as *const u8 as *const core::ffi::c_char,
                );
                fflush(core::ptr::null_mut());
                n = lowest;
                while n > 0 {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const core::ffi::c_char,
                        core::ffi::c_int::from(*resultBuffer.add(u.wrapping_sub(n))),
                    );
                    fflush(core::ptr::null_mut());
                    n = n.wrapping_sub(1);
                }
                fprintf(
                    stderr,
                    b" :%02X:  \0" as *const u8 as *const core::ffi::c_char,
                    core::ffi::c_int::from(*resultBuffer.add(u)),
                );
                fflush(core::ptr::null_mut());
                n = 1;
                while n < 3 {
                    fprintf(
                        stderr,
                        b"%02X \0" as *const u8 as *const core::ffi::c_char,
                        core::ffi::c_int::from(*resultBuffer.add(u.wrapping_add(n))),
                    );
                    fflush(core::ptr::null_mut());
                    n = n.wrapping_add(1);
                }
                fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                fflush(core::ptr::null_mut());
                break;
            } else {
                if u == srcSize.wrapping_sub(1) {
                    fprintf(
                        stderr,
                        b"no difference detected\n\0" as *const u8 as *const core::ffi::c_char,
                    );
                    fflush(core::ptr::null_mut());
                }
                u = u.wrapping_add(1);
            }
        }
    }
    if displayLevel == 1 {
        let cSpeed = benchResult.cSpeed as core::ffi::c_double / core::ffi::c_double::from(MB_UNIT);
        let dSpeed = benchResult.dSpeed as core::ffi::c_double / core::ffi::c_double::from(MB_UNIT);
        if (*adv).additionalParam != 0 {
            fprintf(
                stdout,
                b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s (param=%d)\n\0" as *const u8
                    as *const core::ffi::c_char,
                cLevel,
                cSize as core::ffi::c_int,
                ratio,
                cSpeed,
                dSpeed,
                displayName,
                (*adv).additionalParam,
            );
            fflush(core::ptr::null_mut());
        } else {
            fprintf(
                stdout,
                b"-%-3i%11i (%5.3f) %6.2f MB/s %6.1f MB/s  %s\n\0" as *const u8
                    as *const core::ffi::c_char,
                cLevel,
                cSize as core::ffi::c_int,
                ratio,
                cSpeed,
                dSpeed,
                displayName,
            );
            fflush(core::ptr::null_mut());
        }
    }
    if displayLevel >= 2 {
        fprintf(
            stdout,
            b"%2i#\n\0" as *const u8 as *const core::ffi::c_char,
            cLevel,
        );
        fflush(core::ptr::null_mut());
    }
    benchResult.cMem = ((1 as core::ffi::c_ulonglong) << (*comprParams).windowLog)
        .wrapping_add(ZSTD_sizeof_CCtx(cctx) as core::ffi::c_ulonglong)
        as size_t;
    BMK_benchOutcome_setValidResult(benchResult)
}
pub const NB_MARKS: core::ffi::c_int = 4;
pub unsafe fn BMK_benchMemAdvanced(
    srcBuffer: *const core::ffi::c_void,
    srcSize: size_t,
    dstBuffer: *mut core::ffi::c_void,
    dstCapacity: size_t,
    fileSizes: *const size_t,
    nbFiles: core::ffi::c_uint,
    cLevel: core::ffi::c_int,
    comprParams: *const ZSTD_compressionParameters,
    dictBuffer: *const core::ffi::c_void,
    dictBufferSize: size_t,
    displayLevel: core::ffi::c_int,
    displayName: *const core::ffi::c_char,
    adv: *const BMK_advancedParams_t,
) -> BMK_benchOutcome_t {
    let dstParamsError =
        core::ffi::c_int::from(dstBuffer.is_null()) ^ core::ffi::c_int::from(dstCapacity == 0);
    let chunkSize = (if (*adv).chunkSizeMax >= 32
        && (*adv).mode as core::ffi::c_uint
            != BMK_decodeOnly as core::ffi::c_int as core::ffi::c_uint
    {
        (*adv).chunkSizeMax
    } else {
        srcSize
    })
    .wrapping_add(core::ffi::c_int::from(srcSize == 0) as size_t);
    let nbChunksMax = ((srcSize.wrapping_add(chunkSize.wrapping_sub(1)) / chunkSize) as u32)
        .wrapping_add(nbFiles);
    let srcPtrs = malloc(
        (nbChunksMax as size_t).wrapping_mul(::core::mem::size_of::<*mut core::ffi::c_void>()),
    ) as *mut *const core::ffi::c_void;
    let srcSizes = malloc((nbChunksMax as size_t).wrapping_mul(::core::mem::size_of::<size_t>()))
        as *mut size_t;
    let cPtrs = malloc(
        (nbChunksMax as size_t).wrapping_mul(::core::mem::size_of::<*mut core::ffi::c_void>()),
    ) as *mut *mut core::ffi::c_void;
    let cSizes = malloc((nbChunksMax as size_t).wrapping_mul(::core::mem::size_of::<size_t>()))
        as *mut size_t;
    let cCapacities = malloc((nbChunksMax as size_t).wrapping_mul(::core::mem::size_of::<size_t>()))
        as *mut size_t;
    let resPtrs = malloc(
        (nbChunksMax as size_t).wrapping_mul(::core::mem::size_of::<*mut core::ffi::c_void>()),
    ) as *mut *mut core::ffi::c_void;
    let resSizes = malloc((nbChunksMax as size_t).wrapping_mul(::core::mem::size_of::<size_t>()))
        as *mut size_t;
    let timeStateCompress = BMK_createTimedFnState(
        ((*adv).nbSeconds).wrapping_mul(1000),
        BMK_RUNTEST_DEFAULT_MS as core::ffi::c_uint,
    );
    let timeStateDecompress = BMK_createTimedFnState(
        ((*adv).nbSeconds).wrapping_mul(1000),
        BMK_RUNTEST_DEFAULT_MS as core::ffi::c_uint,
    );
    let cctx = ZSTD_createCCtx();
    let dctx = ZSTD_createDCtx();
    let maxCompressedSize = if dstCapacity != 0 {
        dstCapacity
    } else {
        (ZSTD_compressBound(srcSize)).wrapping_add((nbChunksMax * 1024) as size_t)
    };
    let internalDstBuffer = if !dstBuffer.is_null() {
        core::ptr::null_mut()
    } else {
        malloc(maxCompressedSize)
    };
    let compressedBuffer = if !dstBuffer.is_null() {
        dstBuffer
    } else {
        internalDstBuffer
    };
    let mut outcome = BMK_benchOutcome_error();
    let mut resultBuffer = if srcSize != 0 {
        malloc(srcSize)
    } else {
        core::ptr::null_mut()
    };
    let allocationincomplete = core::ffi::c_int::from(
        srcPtrs.is_null()
            || srcSizes.is_null()
            || cPtrs.is_null()
            || cSizes.is_null()
            || cCapacities.is_null()
            || resPtrs.is_null()
            || resSizes.is_null()
            || timeStateCompress.is_null()
            || timeStateDecompress.is_null()
            || cctx.is_null()
            || dctx.is_null()
            || compressedBuffer.is_null()
            || resultBuffer.is_null(),
    );
    if allocationincomplete == 0 && dstParamsError == 0 {
        outcome = BMK_benchMemAdvancedNoAlloc(
            srcPtrs,
            srcSizes,
            cPtrs,
            cCapacities,
            cSizes,
            resPtrs,
            resSizes,
            &mut resultBuffer,
            compressedBuffer,
            maxCompressedSize,
            timeStateCompress,
            timeStateDecompress,
            srcBuffer,
            srcSize,
            fileSizes,
            nbFiles,
            cLevel,
            comprParams,
            dictBuffer,
            dictBufferSize,
            cctx,
            dctx,
            displayLevel,
            displayName,
            adv,
        );
    }
    BMK_freeTimedFnState(timeStateCompress);
    BMK_freeTimedFnState(timeStateDecompress);
    ZSTD_freeCCtx(cctx);
    ZSTD_freeDCtx(dctx);
    free(internalDstBuffer);
    free(resultBuffer);
    free(srcPtrs as *mut core::ffi::c_void);
    free(srcSizes as *mut core::ffi::c_void);
    free(cPtrs as *mut core::ffi::c_void);
    free(cSizes as *mut core::ffi::c_void);
    free(cCapacities as *mut core::ffi::c_void);
    free(resPtrs as *mut core::ffi::c_void);
    free(resSizes as *mut core::ffi::c_void);
    if allocationincomplete != 0 {
        let mut r = BMK_benchOutcome_t {
            internal_never_use_directly: BMK_benchResult_t {
                cSize: 0,
                cSpeed: 0,
                dSpeed: 0,
                cMem: 0,
            },
            tag: 0,
        };
        ptr::write_bytes(
            &mut r as *mut BMK_benchOutcome_t as *mut u8,
            0,
            ::core::mem::size_of::<BMK_benchOutcome_t>(),
        );
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                31,
            );
            fflush(core::ptr::null_mut());
        }
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"allocation error : not enough memory\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
        }
        if displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            fflush(core::ptr::null_mut());
        }
        r.tag = 31;
        return r;
    }
    if dstParamsError != 0 {
        let mut r_0 = BMK_benchOutcome_t {
            internal_never_use_directly: BMK_benchResult_t {
                cSize: 0,
                cSpeed: 0,
                dSpeed: 0,
                cMem: 0,
            },
            tag: 0,
        };
        ptr::write_bytes(
            &mut r_0 as *mut BMK_benchOutcome_t as *mut u8,
            0,
            ::core::mem::size_of::<BMK_benchOutcome_t>(),
        );
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                32,
            );
            fflush(core::ptr::null_mut());
        }
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Dst parameters not coherent\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
        }
        if displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            fflush(core::ptr::null_mut());
        }
        r_0.tag = 32;
        return r_0;
    }
    outcome
}
pub unsafe fn BMK_benchMem(
    srcBuffer: *const core::ffi::c_void,
    srcSize: size_t,
    fileSizes: *const size_t,
    nbFiles: core::ffi::c_uint,
    cLevel: core::ffi::c_int,
    comprParams: *const ZSTD_compressionParameters,
    dictBuffer: *const core::ffi::c_void,
    dictBufferSize: size_t,
    displayLevel: core::ffi::c_int,
    displayName: *const core::ffi::c_char,
) -> BMK_benchOutcome_t {
    let adv = BMK_initAdvancedParams();
    BMK_benchMemAdvanced(
        srcBuffer,
        srcSize,
        core::ptr::null_mut(),
        0,
        fileSizes,
        nbFiles,
        cLevel,
        comprParams,
        dictBuffer,
        dictBufferSize,
        displayLevel,
        displayName,
        &adv,
    )
}
unsafe fn BMK_benchCLevels(
    srcBuffer: *const core::ffi::c_void,
    benchedSize: size_t,
    fileSizes: *const size_t,
    nbFiles: core::ffi::c_uint,
    startCLevel: core::ffi::c_int,
    endCLevel: core::ffi::c_int,
    comprParams: *const ZSTD_compressionParameters,
    dictBuffer: *const core::ffi::c_void,
    dictBufferSize: size_t,
    displayLevel: core::ffi::c_int,
    mut displayName: *const core::ffi::c_char,
    adv: *const BMK_advancedParams_t,
) -> core::ffi::c_int {
    let mut level: core::ffi::c_int = 0;
    let mut pch: *const core::ffi::c_char = strrchr(displayName, '\\' as i32);
    if pch.is_null() {
        pch = strrchr(displayName, '/' as i32);
    }
    if !pch.is_null() {
        displayName = pch.offset(1);
    }
    if endCLevel > ZSTD_maxCLevel() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Invalid Compression Level \n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
        }
        return 15;
    }
    if endCLevel < startCLevel {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Invalid Compression Level Range \n\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
        }
        return 15;
    }
    if (*adv).realTime != 0 {
        if displayLevel >= 2 {
            fprintf(
                stderr,
                b"Note : switching to real-time priority \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
        }
        setpriority(PRIO_PROCESS, 0, -20);
    }
    if displayLevel == 1 && (*adv).additionalParam == 0 {
        fprintf(
            stdout,
            b"bench %s %s: input %u bytes, %u seconds, %u KB chunks\n\0" as *const u8
                as *const core::ffi::c_char,
            b"1.5.8\0" as *const u8 as *const core::ffi::c_char,
            b"\0" as *const u8 as *const core::ffi::c_char,
            benchedSize as core::ffi::c_uint,
            (*adv).nbSeconds,
            ((*adv).chunkSizeMax >> 10) as core::ffi::c_uint,
        );
        fflush(core::ptr::null_mut());
    }
    level = startCLevel;
    while level <= endCLevel {
        let res = BMK_benchMemAdvanced(
            srcBuffer,
            benchedSize,
            core::ptr::null_mut(),
            0,
            fileSizes,
            nbFiles,
            level,
            comprParams,
            dictBuffer,
            dictBufferSize,
            displayLevel,
            displayName,
            adv,
        );
        if BMK_isSuccessful_benchOutcome(res) == 0 {
            return 1;
        }
        level += 1;
    }
    0
}
pub unsafe fn BMK_syntheticTest(
    compressibility: core::ffi::c_double,
    startingCLevel: core::ffi::c_int,
    endCLevel: core::ffi::c_int,
    compressionParams: *const ZSTD_compressionParameters,
    displayLevel: core::ffi::c_int,
    adv: *const BMK_advancedParams_t,
) -> core::ffi::c_int {
    let mut nameBuff: [core::ffi::c_char; 20] = [0; 20];
    let mut name: *const core::ffi::c_char = nameBuff.as_mut_ptr();
    let benchedSize = if (*adv).chunkSizeMax != 0 {
        (*adv).chunkSizeMax
    } else {
        10000000
    };
    let srcBuffer = malloc(benchedSize);
    if srcBuffer.is_null() {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"allocation error : not enough memory \n\0" as *const u8
                    as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
        }
        return 16;
    }
    if compressibility < 0.0f64 {
        LOREM_genBuffer(srcBuffer, benchedSize, 0);
        name = b"Lorem ipsum\0" as *const u8 as *const core::ffi::c_char;
    } else {
        RDG_genBuffer(srcBuffer, benchedSize, compressibility, 0.0f64, 0);
        formatString_u(
            nameBuff.as_mut_ptr(),
            ::core::mem::size_of::<[core::ffi::c_char; 20]>(),
            b"Synthetic %u%%\0" as *const u8 as *const core::ffi::c_char,
            (compressibility * 100.0) as core::ffi::c_uint,
        );
    }
    let res = BMK_benchCLevels(
        srcBuffer,
        benchedSize,
        &benchedSize,
        1,
        startingCLevel,
        endCLevel,
        compressionParams,
        core::ptr::null(),
        0,
        displayLevel,
        name,
        adv,
    );
    free(srcBuffer);
    res
}
unsafe fn BMK_findMaxMem(mut requiredMem: u64) -> size_t {
    let step = (64 * ((1) << 20)) as size_t;
    let mut testmem = core::ptr::null_mut();
    requiredMem = (requiredMem >> 26).wrapping_add(1) << 26;
    requiredMem = requiredMem.wrapping_add(step as u64);
    if requiredMem > maxMemory as u64 {
        requiredMem = maxMemory as u64;
    }
    loop {
        testmem = malloc(requiredMem as size_t) as *mut u8;
        requiredMem = requiredMem.wrapping_sub(step as u64);
        if !(testmem.is_null() && requiredMem > 0) {
            break;
        }
    }
    free(testmem as *mut core::ffi::c_void);
    requiredMem as size_t
}
unsafe fn BMK_loadFiles(
    buffer: *mut core::ffi::c_void,
    bufferSize: size_t,
    fileSizes: *mut size_t,
    fileNamesTable: *const *const core::ffi::c_char,
    mut nbFiles: core::ffi::c_uint,
    displayLevel: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut pos = 0;
    let mut totalSize = 0 as size_t;
    let mut n: core::ffi::c_uint = 0;
    n = 0;
    while n < nbFiles {
        let filename = *fileNamesTable.offset(n as isize);
        let mut fileSize = UTIL_getFileSize(filename);
        if UTIL_isDirectory(filename) != 0 {
            if displayLevel >= 2 {
                fprintf(
                    stderr,
                    b"Ignoring %s directory...       \n\0" as *const u8 as *const core::ffi::c_char,
                    filename,
                );
                fflush(core::ptr::null_mut());
            }
            *fileSizes.offset(n as isize) = 0;
        } else if fileSize == UTIL_FILESIZE_UNKNOWN as u64 {
            if displayLevel >= 2 {
                fprintf(
                    stderr,
                    b"Cannot evaluate size of %s, ignoring ... \n\0" as *const u8
                        as *const core::ffi::c_char,
                    filename,
                );
                fflush(core::ptr::null_mut());
            }
            *fileSizes.offset(n as isize) = 0;
        } else {
            if fileSize > bufferSize.wrapping_sub(pos) as u64 {
                fileSize = bufferSize.wrapping_sub(pos) as u64;
                nbFiles = n;
            }
            let f = fopen(filename, b"rb\0" as *const u8 as *const core::ffi::c_char);
            if f.is_null() {
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                        10,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"cannot open file %s\0" as *const u8 as *const core::ffi::c_char,
                        filename,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    fflush(core::ptr::null_mut());
                }
                return 10;
            }
            if displayLevel >= 2 {
                fprintf(
                    stdout,
                    b"Loading %s...       \r\0" as *const u8 as *const core::ffi::c_char,
                    filename,
                );
                fflush(core::ptr::null_mut());
            }
            let readSize = fread(
                (buffer as *mut core::ffi::c_char).add(pos) as *mut core::ffi::c_void,
                1,
                fileSize as size_t,
                f,
            );
            if readSize != fileSize as size_t {
                fclose(f);
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                        11,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(
                        stderr,
                        b"invalid read %s\0" as *const u8 as *const core::ffi::c_char,
                        filename,
                    );
                    fflush(core::ptr::null_mut());
                }
                if displayLevel >= 1 {
                    fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
                    fflush(core::ptr::null_mut());
                }
                return 11;
            }
            pos = pos.wrapping_add(readSize);
            *fileSizes.offset(n as isize) = fileSize as size_t;
            totalSize = totalSize.wrapping_add(fileSize as size_t);
            fclose(f);
        }
        n = n.wrapping_add(1);
    }
    if totalSize == 0 {
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"Error %i : \0" as *const u8 as *const core::ffi::c_char,
                12,
            );
            fflush(core::ptr::null_mut());
        }
        if displayLevel >= 1 {
            fprintf(
                stderr,
                b"no data to bench\0" as *const u8 as *const core::ffi::c_char,
            );
            fflush(core::ptr::null_mut());
        }
        if displayLevel >= 1 {
            fprintf(stderr, b" \n\0" as *const u8 as *const core::ffi::c_char);
            fflush(core::ptr::null_mut());
        }
        return 12;
    }
    0
}
pub unsafe fn BMK_benchFilesAdvanced(
    fileNamesTable: *const *const core::ffi::c_char,
    nbFiles: core::ffi::c_uint,
    dictFileName: *const core::ffi::c_char,
    startCLevel: core::ffi::c_int,
    endCLevel: core::ffi::c_int,
    compressionParams: *const ZSTD_compressionParameters,
    displayLevel: core::ffi::c_int,
    adv: *const BMK_advancedParams_t,
) -> core::ffi::c_int {
    let mut srcBuffer = core::ptr::null_mut();
    let mut benchedSize: size_t = 0;
    let mut dictBuffer = core::ptr::null_mut();
    let mut dictBufferSize = 0;
    let mut fileSizes = core::ptr::null_mut();
    let mut res = 1;
    let totalSizeToLoad = UTIL_getTotalFileSize(fileNamesTable, nbFiles);

    if nbFiles == 0 {
        if displayLevel >= 1 {
            eprintln!("No Files to Benchmark");
        }
        return 13;
    }

    if endCLevel > ZSTD_maxCLevel() {
        if displayLevel >= 1 {
            eprintln!("Invalid Compression Level");
        }
        return 14;
    }

    if totalSizeToLoad == UTIL_FILESIZE_UNKNOWN as u64 {
        if displayLevel >= 1 {
            eprintln!("Error loading files");
        }
        return 15;
    }

    fileSizes = calloc(nbFiles as size_t, ::core::mem::size_of::<size_t>()) as *mut size_t;
    if fileSizes.is_null() {
        if displayLevel >= 1 {
            eprintln!("not enough memory for fileSizes");
        }
        return 16;
    }

    '_cleanUp: {
        // Load dictionary
        if !dictFileName.is_null() {
            let dictFileSize = UTIL_getFileSize(dictFileName);
            if dictFileSize == UTIL_FILESIZE_UNKNOWN as u64 {
                if displayLevel >= 1 {
                    eprintln!(
                        "error loading {} : {}",
                        CStr::from_ptr(dictFileName).to_string_lossy(),
                        io::Error::last_os_error(),
                    );
                }
                free(fileSizes as *mut core::ffi::c_void);
                if displayLevel >= 1 {
                    eprintln!("benchmark aborted");
                }
                return 17;
            }
            if dictFileSize > (64 * ((1) << 20)) as u64 {
                free(fileSizes as *mut core::ffi::c_void);
                if displayLevel >= 1 {
                    eprintln!(
                        "dictionary file {} too large",
                        CStr::from_ptr(dictFileName).to_string_lossy(),
                    );
                }
                return 18;
            }
            dictBufferSize = dictFileSize as size_t;
            dictBuffer = malloc(dictBufferSize);
            if dictBuffer.is_null() {
                free(fileSizes as *mut core::ffi::c_void);
                if displayLevel >= 1 {
                    eprintln!(
                        "not enough memory for dictionary ({} bytes)\0",
                        dictBufferSize,
                    );
                }
                return 19;
            }

            let errorCode = BMK_loadFiles(
                dictBuffer,
                dictBufferSize,
                fileSizes,
                &dictFileName,
                1,
                displayLevel,
            );
            if errorCode != 0 {
                break '_cleanUp;
            }
        }

        // Memory allocation & restrictions
        benchedSize = BMK_findMaxMem(totalSizeToLoad * 3) / 3;
        if benchedSize > totalSizeToLoad as size_t {
            benchedSize = totalSizeToLoad as size_t;
        }
        if benchedSize < totalSizeToLoad as size_t {
            eprintln!(
                "Not enough memory; testing {} MB only...",
                benchedSize >> 20,
            );
        }

        srcBuffer = if benchedSize != 0 {
            malloc(benchedSize)
        } else {
            core::ptr::null_mut()
        };
        if srcBuffer.is_null() {
            free(dictBuffer);
            free(fileSizes as *mut core::ffi::c_void);
            if displayLevel >= 1 {
                eprintln!("not enough memory for srcBuffer");
                fflush(core::ptr::null_mut());
            }
            return 20;
        }

        // Load input buffer
        let errorCode = BMK_loadFiles(
            srcBuffer,
            benchedSize,
            fileSizes,
            fileNamesTable,
            nbFiles,
            displayLevel,
        );
        if errorCode != 0 {
            break '_cleanUp;
        }

        // Bench
        let mut mfName: [core::ffi::c_char; 20] = [0; 20];
        formatString_u(
            mfName.as_mut_ptr(),
            ::core::mem::size_of::<[core::ffi::c_char; 20]>(),
            c" %u files".as_ptr(),
            nbFiles,
        );
        let displayName = if nbFiles > 1 {
            mfName.as_mut_ptr() as *const core::ffi::c_char
        } else {
            *fileNamesTable.offset(0)
        };
        res = BMK_benchCLevels(
            srcBuffer,
            benchedSize,
            fileSizes,
            nbFiles,
            startCLevel,
            endCLevel,
            compressionParams,
            dictBuffer,
            dictBufferSize,
            displayLevel,
            displayName,
            adv,
        );
    }

    free(srcBuffer);
    free(dictBuffer);
    free(fileSizes as *mut core::ffi::c_void);
    res
}

pub unsafe fn BMK_benchFiles(
    fileNamesTable: *const *const core::ffi::c_char,
    nbFiles: core::ffi::c_uint,
    dictFileName: *const core::ffi::c_char,
    cLevel: core::ffi::c_int,
    compressionParams: *const ZSTD_compressionParameters,
    displayLevel: core::ffi::c_int,
) -> core::ffi::c_int {
    let adv = BMK_initAdvancedParams();
    BMK_benchFilesAdvanced(
        fileNamesTable,
        nbFiles,
        dictFileName,
        cLevel,
        cLevel,
        compressionParams,
        displayLevel,
        &adv,
    )
}
