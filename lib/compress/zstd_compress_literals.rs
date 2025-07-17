use ::libc;
extern "C" {
    fn HUF_compress4X_repeat(
        dst: *mut std::ffi::c_void,
        dstSize: size_t,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        maxSymbolValue: std::ffi::c_uint,
        tableLog: std::ffi::c_uint,
        workSpace: *mut std::ffi::c_void,
        wkspSize: size_t,
        hufTable: *mut HUF_CElt,
        repeat: *mut HUF_repeat,
        flags: std::ffi::c_int,
    ) -> size_t;
    fn HUF_compress1X_repeat(
        dst: *mut std::ffi::c_void,
        dstSize: size_t,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        maxSymbolValue: std::ffi::c_uint,
        tableLog: std::ffi::c_uint,
        workSpace: *mut std::ffi::c_void,
        wkspSize: size_t,
        hufTable: *mut HUF_CElt,
        repeat: *mut HUF_repeat,
        flags: std::ffi::c_int,
    ) -> size_t;
}
pub type size_t = std::ffi::c_ulong;
pub type __uint8_t = std::ffi::c_uchar;
pub type __uint16_t = std::ffi::c_ushort;
pub type __uint32_t = std::ffi::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type unalign16 = U16;
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
pub type SymbolEncodingType_e = std::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type ZSTD_strategy = std::ffi::c_uint;
pub const ZSTD_btultra2: ZSTD_strategy = 9;
pub const ZSTD_btultra: ZSTD_strategy = 8;
pub const ZSTD_btopt: ZSTD_strategy = 7;
pub const ZSTD_btlazy2: ZSTD_strategy = 6;
pub const ZSTD_lazy2: ZSTD_strategy = 5;
pub const ZSTD_lazy: ZSTD_strategy = 4;
pub const ZSTD_greedy: ZSTD_strategy = 3;
pub const ZSTD_dfast: ZSTD_strategy = 2;
pub const ZSTD_fast: ZSTD_strategy = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
pub type HUF_repeat = std::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_CElt = size_t;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type huf_compress_f = Option<
    unsafe extern "C" fn(
        *mut std::ffi::c_void,
        size_t,
        *const std::ffi::c_void,
        size_t,
        std::ffi::c_uint,
        std::ffi::c_uint,
        *mut std::ffi::c_void,
        size_t,
        *mut HUF_CElt,
        *mut HUF_repeat,
        std::ffi::c_int,
    ) -> size_t,
>;
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    1 as std::ffi::c_int as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut std::ffi::c_void, mut value: U16) {
    *(memPtr as *mut unalign16) = value;
}
#[inline]
unsafe extern "C" fn MEM_write32(mut memPtr: *mut std::ffi::c_void, mut value: U32) {
    *(memPtr as *mut unalign32) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    in_0.swap_bytes()
}
#[inline]
unsafe extern "C" fn MEM_writeLE16(mut memPtr: *mut std::ffi::c_void, mut val: U16) {
    if MEM_isLittleEndian() != 0 {
        MEM_write16(memPtr, val);
    } else {
        let mut p = memPtr as *mut BYTE;
        *p.offset(0 as std::ffi::c_int as isize) = val as BYTE;
        *p.offset(1 as std::ffi::c_int as isize) =
            (val as std::ffi::c_int >> 8 as std::ffi::c_int) as BYTE;
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE24(mut memPtr: *mut std::ffi::c_void, mut val: U32) {
    MEM_writeLE16(memPtr, val as U16);
    *(memPtr as *mut BYTE).offset(2 as std::ffi::c_int as isize) =
        (val >> 16 as std::ffi::c_int) as BYTE;
}
#[inline]
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut std::ffi::c_void, mut val32: U32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn _force_has_format_string(mut format: *const std::ffi::c_char, mut args: ...) {}
#[inline]
unsafe extern "C" fn ZSTD_minGain(mut srcSize: size_t, mut strat: ZSTD_strategy) -> size_t {
    let minlog = if strat as std::ffi::c_uint >= ZSTD_btultra as std::ffi::c_int as std::ffi::c_uint
    {
        (strat as U32).wrapping_sub(1 as std::ffi::c_int as U32)
    } else {
        6 as std::ffi::c_int as U32
    };
    (srcSize >> minlog).wrapping_add(2 as std::ffi::c_int as size_t)
}
pub const LitHufLog: std::ffi::c_int = 11 as std::ffi::c_int;
pub const HUF_SYMBOLVALUE_MAX: std::ffi::c_int = 255 as std::ffi::c_int;
pub const HUF_OPTIMAL_DEPTH_THRESHOLD: std::ffi::c_int = ZSTD_btultra as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_noCompressLiterals(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let flSize = (1 as std::ffi::c_int
        + (srcSize > 31 as std::ffi::c_int as size_t) as std::ffi::c_int
        + (srcSize > 4095 as std::ffi::c_int as size_t) as std::ffi::c_int) as U32;
    if srcSize.wrapping_add(flSize as size_t) > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    match flSize {
        1 => {
            *ostart.offset(0 as std::ffi::c_int as isize) =
                (set_basic as std::ffi::c_int as U32 as size_t)
                    .wrapping_add(srcSize << 3 as std::ffi::c_int) as BYTE;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut std::ffi::c_void,
                ((set_basic as std::ffi::c_int as U32)
                    .wrapping_add(((1 as std::ffi::c_int) << 2 as std::ffi::c_int) as U32)
                    as size_t)
                    .wrapping_add(srcSize << 4 as std::ffi::c_int) as U16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut std::ffi::c_void,
                ((set_basic as std::ffi::c_int as U32)
                    .wrapping_add(((3 as std::ffi::c_int) << 2 as std::ffi::c_int) as U32)
                    as size_t)
                    .wrapping_add(srcSize << 4 as std::ffi::c_int) as U32,
            );
        }
        _ => {}
    }
    libc::memcpy(
        ostart.offset(flSize as isize) as *mut std::ffi::c_void,
        src,
        srcSize as libc::size_t,
    );
    srcSize.wrapping_add(flSize as size_t)
}
unsafe extern "C" fn allBytesIdentical(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> std::ffi::c_int {
    let b = *(src as *const BYTE).offset(0 as std::ffi::c_int as isize);
    let mut p: size_t = 0;
    p = 1 as std::ffi::c_int as size_t;
    while p < srcSize {
        if *(src as *const BYTE).offset(p as isize) as std::ffi::c_int != b as std::ffi::c_int {
            return 0 as std::ffi::c_int;
        }
        p = p.wrapping_add(1);
        p;
    }
    1 as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressRleLiteralsBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let flSize = (1 as std::ffi::c_int
        + (srcSize > 31 as std::ffi::c_int as size_t) as std::ffi::c_int
        + (srcSize > 4095 as std::ffi::c_int as size_t) as std::ffi::c_int) as U32;
    match flSize {
        1 => {
            *ostart.offset(0 as std::ffi::c_int as isize) =
                (set_rle as std::ffi::c_int as U32 as size_t)
                    .wrapping_add(srcSize << 3 as std::ffi::c_int) as BYTE;
        }
        2 => {
            MEM_writeLE16(
                ostart as *mut std::ffi::c_void,
                ((set_rle as std::ffi::c_int as U32)
                    .wrapping_add(((1 as std::ffi::c_int) << 2 as std::ffi::c_int) as U32)
                    as size_t)
                    .wrapping_add(srcSize << 4 as std::ffi::c_int) as U16,
            );
        }
        3 => {
            MEM_writeLE32(
                ostart as *mut std::ffi::c_void,
                ((set_rle as std::ffi::c_int as U32)
                    .wrapping_add(((3 as std::ffi::c_int) << 2 as std::ffi::c_int) as U32)
                    as size_t)
                    .wrapping_add(srcSize << 4 as std::ffi::c_int) as U32,
            );
        }
        _ => {}
    }
    *ostart.offset(flSize as isize) = *(src as *const BYTE);
    flSize.wrapping_add(1 as std::ffi::c_int as U32) as size_t
}
unsafe extern "C" fn ZSTD_minLiteralsToCompress(
    mut strategy: ZSTD_strategy,
    mut huf_repeat: HUF_repeat,
) -> size_t {
    let shift = if (9 as std::ffi::c_int - strategy as std::ffi::c_int) < 3 as std::ffi::c_int {
        9 as std::ffi::c_int - strategy as std::ffi::c_int
    } else {
        3 as std::ffi::c_int
    };
    let mintc = if huf_repeat as std::ffi::c_uint
        == HUF_repeat_valid as std::ffi::c_int as std::ffi::c_uint
    {
        6 as std::ffi::c_int as size_t
    } else {
        (8 as std::ffi::c_int as size_t) << shift
    };
    mintc
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressLiterals(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut entropyWorkspace: *mut std::ffi::c_void,
    mut entropyWorkspaceSize: size_t,
    mut prevHuf: *const ZSTD_hufCTables_t,
    mut nextHuf: *mut ZSTD_hufCTables_t,
    mut strategy: ZSTD_strategy,
    mut disableLiteralCompression: std::ffi::c_int,
    mut suspectUncompressible: std::ffi::c_int,
    mut bmi2: std::ffi::c_int,
) -> size_t {
    let lhSize = (3 as std::ffi::c_int
        + (srcSize
            >= (1 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as size_t)
            as std::ffi::c_int
        + (srcSize
            >= (16 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int))
                as size_t) as std::ffi::c_int) as size_t;
    let ostart = dst as *mut BYTE;
    let mut singleStream = (srcSize < 256 as std::ffi::c_int as size_t) as std::ffi::c_int as U32;
    let mut hType = set_compressed;
    let mut cLitSize: size_t = 0;
    libc::memcpy(
        nextHuf as *mut std::ffi::c_void,
        prevHuf as *const std::ffi::c_void,
        ::core::mem::size_of::<ZSTD_hufCTables_t>() as std::ffi::c_ulong as libc::size_t,
    );
    if disableLiteralCompression != 0 {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if srcSize < ZSTD_minLiteralsToCompress(strategy, (*prevHuf).repeatMode) {
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if dstCapacity < lhSize.wrapping_add(1 as std::ffi::c_int as size_t) {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    let mut repeat = (*prevHuf).repeatMode;
    let flags = 0 as std::ffi::c_int
        | (if bmi2 != 0 {
            HUF_flags_bmi2 as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        })
        | (if (strategy as std::ffi::c_uint) < ZSTD_lazy as std::ffi::c_int as std::ffi::c_uint
            && srcSize <= 1024 as std::ffi::c_int as size_t
        {
            HUF_flags_preferRepeat as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        })
        | (if strategy as std::ffi::c_uint >= HUF_OPTIMAL_DEPTH_THRESHOLD as std::ffi::c_uint {
            HUF_flags_optimalDepth as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        })
        | (if suspectUncompressible != 0 {
            HUF_flags_suspectUncompressible as std::ffi::c_int
        } else {
            0 as std::ffi::c_int
        });
    let mut huf_compress: huf_compress_f = None;
    if repeat as std::ffi::c_uint == HUF_repeat_valid as std::ffi::c_int as std::ffi::c_uint
        && lhSize == 3 as std::ffi::c_int as size_t
    {
        singleStream = 1 as std::ffi::c_int as U32;
    }
    huf_compress = if singleStream != 0 {
        Some(
            HUF_compress1X_repeat
                as unsafe extern "C" fn(
                    *mut std::ffi::c_void,
                    size_t,
                    *const std::ffi::c_void,
                    size_t,
                    std::ffi::c_uint,
                    std::ffi::c_uint,
                    *mut std::ffi::c_void,
                    size_t,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    std::ffi::c_int,
                ) -> size_t,
        )
    } else {
        Some(
            HUF_compress4X_repeat
                as unsafe extern "C" fn(
                    *mut std::ffi::c_void,
                    size_t,
                    *const std::ffi::c_void,
                    size_t,
                    std::ffi::c_uint,
                    std::ffi::c_uint,
                    *mut std::ffi::c_void,
                    size_t,
                    *mut HUF_CElt,
                    *mut HUF_repeat,
                    std::ffi::c_int,
                ) -> size_t,
        )
    };
    cLitSize = huf_compress.unwrap_unchecked()(
        ostart.offset(lhSize as isize) as *mut std::ffi::c_void,
        dstCapacity.wrapping_sub(lhSize),
        src,
        srcSize,
        HUF_SYMBOLVALUE_MAX as std::ffi::c_uint,
        LitHufLog as std::ffi::c_uint,
        entropyWorkspace,
        entropyWorkspaceSize,
        ((*nextHuf).CTable).as_mut_ptr(),
        &mut repeat,
        flags,
    );
    if repeat as std::ffi::c_uint != HUF_repeat_none as std::ffi::c_int as std::ffi::c_uint {
        hType = set_repeat;
    }
    let minGain = ZSTD_minGain(srcSize, strategy);
    if cLitSize == 0 as std::ffi::c_int as size_t
        || cLitSize >= srcSize.wrapping_sub(minGain)
        || ERR_isError(cLitSize) != 0
    {
        libc::memcpy(
            nextHuf as *mut std::ffi::c_void,
            prevHuf as *const std::ffi::c_void,
            ::core::mem::size_of::<ZSTD_hufCTables_t>() as std::ffi::c_ulong as libc::size_t,
        );
        return ZSTD_noCompressLiterals(dst, dstCapacity, src, srcSize);
    }
    if cLitSize == 1 as std::ffi::c_int as size_t
        && (srcSize >= 8 as std::ffi::c_int as size_t || allBytesIdentical(src, srcSize) != 0) {
            libc::memcpy(
                nextHuf as *mut std::ffi::c_void,
                prevHuf as *const std::ffi::c_void,
                ::core::mem::size_of::<ZSTD_hufCTables_t>() as std::ffi::c_ulong as libc::size_t,
            );
            return ZSTD_compressRleLiteralsBlock(dst, dstCapacity, src, srcSize);
        }
    if hType as std::ffi::c_uint == set_compressed as std::ffi::c_int as std::ffi::c_uint {
        (*nextHuf).repeatMode = HUF_repeat_check;
    }
    match lhSize {
        3 => {
            singleStream == 0;
            let lhc = (hType as std::ffi::c_uint)
                .wrapping_add(
                    ((singleStream == 0) as std::ffi::c_int as U32) << 2 as std::ffi::c_int,
                )
                .wrapping_add((srcSize as U32) << 4 as std::ffi::c_int)
                .wrapping_add((cLitSize as U32) << 14 as std::ffi::c_int);
            MEM_writeLE24(ostart as *mut std::ffi::c_void, lhc);
        }
        4 => {
            let lhc_0 = (hType as std::ffi::c_uint)
                .wrapping_add(((2 as std::ffi::c_int) << 2 as std::ffi::c_int) as std::ffi::c_uint)
                .wrapping_add((srcSize as U32) << 4 as std::ffi::c_int)
                .wrapping_add((cLitSize as U32) << 18 as std::ffi::c_int);
            MEM_writeLE32(ostart as *mut std::ffi::c_void, lhc_0);
        }
        5 => {
            let lhc_1 = (hType as std::ffi::c_uint)
                .wrapping_add(((3 as std::ffi::c_int) << 2 as std::ffi::c_int) as std::ffi::c_uint)
                .wrapping_add((srcSize as U32) << 4 as std::ffi::c_int)
                .wrapping_add((cLitSize as U32) << 22 as std::ffi::c_int);
            MEM_writeLE32(ostart as *mut std::ffi::c_void, lhc_1);
            *ostart.offset(4 as std::ffi::c_int as isize) =
                (cLitSize >> 10 as std::ffi::c_int) as BYTE;
        }
        _ => {}
    }
    lhSize.wrapping_add(cLitSize)
}
