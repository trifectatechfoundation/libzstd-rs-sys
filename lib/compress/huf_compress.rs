use crate::lib::{
    common::entropy_common::HUF_readStats,
    compress::{
        fse_compress::{
            FSE_buildCTable_wksp, FSE_compress_usingCTable, FSE_normalizeCount,
            FSE_optimalTableLog, FSE_optimalTableLog_internal, FSE_writeNCount,
        },
        hist::{HIST_count_simple, HIST_count_wksp},
    },
};
pub type size_t = std::ffi::c_ulong;
pub type __uint8_t = std::ffi::c_uchar;
pub type __int16_t = std::ffi::c_short;
pub type __uint16_t = std::ffi::c_ushort;
pub type __uint32_t = std::ffi::c_uint;
pub type __uint64_t = std::ffi::c_ulong;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U16 = uint16_t;
pub type S16 = int16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type unalign64 = U64;
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
pub type FSE_CTable = std::ffi::c_uint;
pub type HUF_CElt = size_t;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type nodeElt = nodeElt_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nodeElt_s {
    pub count: U32,
    pub parent: U16,
    pub byte: BYTE,
    pub nbBits: BYTE,
}
pub type huffNodeTable = [nodeElt; 512];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_buildCTable_wksp_tables {
    pub huffNodeTbl: huffNodeTable,
    pub rankPosition: [rankPos; 192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rankPos {
    pub base: U16,
    pub curr: U16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CTableHeader {
    pub tableLog: BYTE,
    pub maxSymbolValue: BYTE,
    pub unused: [BYTE; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_WriteCTableWksp {
    pub wksp: HUF_CompressWeightsWksp,
    pub bitsToWeight: [BYTE; 13],
    pub huffWeight: [BYTE; 255],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CompressWeightsWksp {
    pub CTable: [FSE_CTable; 59],
    pub scratchBuffer: [U32; 41],
    pub count: [std::ffi::c_uint; 13],
    pub norm: [S16; 13],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_CStream_t {
    pub bitContainer: [size_t; 2],
    pub bitPos: [size_t; 2],
    pub startPtr: *mut BYTE,
    pub ptr: *mut BYTE,
    pub endPtr: *mut BYTE,
}
pub type HUF_repeat = std::ffi::c_uint;
pub const HUF_repeat_valid: HUF_repeat = 2;
pub const HUF_repeat_check: HUF_repeat = 1;
pub const HUF_repeat_none: HUF_repeat = 0;
pub type HUF_nbStreams_e = std::ffi::c_uint;
pub const HUF_fourStreams: HUF_nbStreams_e = 1;
pub const HUF_singleStream: HUF_nbStreams_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_compress_tables_t {
    pub count: [std::ffi::c_uint; 256],
    pub CTable: [HUF_CElt; 257],
    pub wksps: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub buildCTable_wksp: HUF_buildCTable_wksp_tables,
    pub writeCTable_wksp: HUF_WriteCTableWksp,
    pub hist_wksp: [U32; 1024],
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
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
unsafe extern "C" fn MEM_write64(mut memPtr: *mut std::ffi::c_void, mut value: U64) {
    *(memPtr as *mut unalign64) = value;
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    in_0.swap_bytes()
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
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
unsafe extern "C" fn MEM_writeLE32(mut memPtr: *mut std::ffi::c_void, mut val32: U32) {
    if MEM_isLittleEndian() != 0 {
        MEM_write32(memPtr, val32);
    } else {
        MEM_write32(memPtr, MEM_swap32(val32));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLE64(mut memPtr: *mut std::ffi::c_void, mut val64: U64) {
    if MEM_isLittleEndian() != 0 {
        MEM_write64(memPtr, val64);
    } else {
        MEM_write64(memPtr, MEM_swap64(val64));
    };
}
#[inline]
unsafe extern "C" fn MEM_writeLEST(mut memPtr: *mut std::ffi::c_void, mut val: size_t) {
    if MEM_32bits() != 0 {
        MEM_writeLE32(memPtr, val as U32);
    } else {
        MEM_writeLE64(memPtr, val);
    };
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
const fn ZSTD_countLeadingZeros32(mut val: U32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
const fn ZSTD_highbit32(mut val: U32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const HUF_BLOCKSIZE_MAX: std::ffi::c_int = 128 as std::ffi::c_int * 1024 as std::ffi::c_int;
pub const HUF_TABLELOG_MAX: std::ffi::c_int = 12 as std::ffi::c_int;
pub const HUF_TABLELOG_DEFAULT: std::ffi::c_int = 11 as std::ffi::c_int;
pub const HUF_SYMBOLVALUE_MAX: std::ffi::c_int = 255 as std::ffi::c_int;
pub const HUF_CTABLEBOUND: std::ffi::c_int = 129 as std::ffi::c_int;
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const HUF_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
unsafe extern "C" fn HUF_alignUpWorkspace(
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSizePtr: *mut size_t,
    mut align: size_t,
) -> *mut std::ffi::c_void {
    let mask = align.wrapping_sub(1 as std::ffi::c_int as size_t);
    let rem = workspace as size_t & mask;
    let add = align.wrapping_sub(rem) & mask;
    let aligned = (workspace as *mut BYTE).offset(add as isize);
    if *workspaceSizePtr >= add {
        *workspaceSizePtr = (*workspaceSizePtr).wrapping_sub(add);
        aligned as *mut std::ffi::c_void
    } else {
        *workspaceSizePtr = 0 as std::ffi::c_int as size_t;
        NULL as *mut std::ffi::c_void
    }
}
pub const MAX_FSE_TABLELOG_FOR_HUFF_HEADER: std::ffi::c_int = 6 as std::ffi::c_int;
unsafe extern "C" fn HUF_compressWeights(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut weightTable: *const std::ffi::c_void,
    mut wtSize: size_t,
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let oend = ostart.offset(dstSize as isize);
    let mut maxSymbolValue = HUF_TABLELOG_MAX as std::ffi::c_uint;
    let mut tableLog = MAX_FSE_TABLELOG_FOR_HUFF_HEADER as U32;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<U32>() as std::ffi::c_ulong,
    ) as *mut HUF_CompressWeightsWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_CompressWeightsWksp>() as std::ffi::c_ulong {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if wtSize <= 1 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    let maxCount = HIST_count_simple(
        ((*wksp).count).as_mut_ptr(),
        &mut maxSymbolValue,
        weightTable,
        wtSize,
    );
    if maxCount as size_t == wtSize {
        return 1 as std::ffi::c_int as size_t;
    }
    if maxCount == 1 as std::ffi::c_int as std::ffi::c_uint {
        return 0 as std::ffi::c_int as size_t;
    }
    tableLog = FSE_optimalTableLog(tableLog, wtSize, maxSymbolValue);
    let _var_err__ = FSE_normalizeCount(
        ((*wksp).norm).as_mut_ptr(),
        tableLog,
        ((*wksp).count).as_mut_ptr(),
        wtSize,
        maxSymbolValue,
        0 as std::ffi::c_int as std::ffi::c_uint,
    );
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let hSize = FSE_writeNCount(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as size_t,
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    op = op.offset(hSize as isize);
    let _var_err___0 = FSE_buildCTable_wksp(
        ((*wksp).CTable).as_mut_ptr(),
        ((*wksp).norm).as_mut_ptr(),
        maxSymbolValue,
        tableLog,
        ((*wksp).scratchBuffer).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[U32; 41]>() as std::ffi::c_ulong,
    );
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let cSize = FSE_compress_usingCTable(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as size_t,
        weightTable,
        wtSize,
        ((*wksp).CTable).as_mut_ptr(),
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    op = op.offset(cSize as isize);
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn HUF_getNbBits(mut elt: HUF_CElt) -> size_t {
    elt & 0xff as std::ffi::c_int as HUF_CElt
}
unsafe extern "C" fn HUF_getNbBitsFast(mut elt: HUF_CElt) -> size_t {
    elt
}
unsafe extern "C" fn HUF_getValue(mut elt: HUF_CElt) -> size_t {
    elt & !(0xff as std::ffi::c_int as size_t)
}
unsafe extern "C" fn HUF_getValueFast(mut elt: HUF_CElt) -> size_t {
    elt
}
unsafe extern "C" fn HUF_setNbBits(mut elt: *mut HUF_CElt, mut nbBits: size_t) {
    *elt = nbBits;
}
unsafe extern "C" fn HUF_setValue(mut elt: *mut HUF_CElt, mut value: size_t) {
    let nbBits = HUF_getNbBits(*elt);
    if nbBits > 0 as std::ffi::c_int as size_t {
        *elt |= value
            << (::core::mem::size_of::<HUF_CElt>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                .wrapping_sub(nbBits);
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTableHeader(mut ctable: *const HUF_CElt) -> HUF_CTableHeader {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    libc::memcpy(
        &mut header as *mut HUF_CTableHeader as *mut std::ffi::c_void,
        ctable as *const std::ffi::c_void,
        ::core::mem::size_of::<HUF_CTableHeader>() as std::ffi::c_ulong as libc::size_t,
    );
    header
}
unsafe extern "C" fn HUF_writeCTableHeader(
    mut ctable: *mut HUF_CElt,
    mut tableLog: U32,
    mut maxSymbolValue: U32,
) {
    let mut header = HUF_CTableHeader {
        tableLog: 0,
        maxSymbolValue: 0,
        unused: [0; 6],
    };
    libc::memset(
        &mut header as *mut HUF_CTableHeader as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<HUF_CTableHeader>() as std::ffi::c_ulong as libc::size_t,
    );
    header.tableLog = tableLog as BYTE;
    header.maxSymbolValue = maxSymbolValue as BYTE;
    libc::memcpy(
        ctable as *mut std::ffi::c_void,
        &mut header as *mut HUF_CTableHeader as *const std::ffi::c_void,
        ::core::mem::size_of::<HUF_CTableHeader>() as std::ffi::c_ulong as libc::size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn HUF_writeCTable_wksp(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut CTable: *const HUF_CElt,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut workspace: *mut std::ffi::c_void,
    mut workspaceSize: size_t,
) -> size_t {
    let ct = CTable.offset(1 as std::ffi::c_int as isize);
    let mut op = dst as *mut BYTE;
    let mut n: U32 = 0;
    let mut wksp = HUF_alignUpWorkspace(
        workspace,
        &mut workspaceSize,
        ::core::mem::align_of::<U32>() as std::ffi::c_ulong,
    ) as *mut HUF_WriteCTableWksp;
    if workspaceSize < ::core::mem::size_of::<HUF_WriteCTableWksp>() as std::ffi::c_ulong {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as std::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as std::ffi::c_int) as size_t;
    }
    *((*wksp).bitsToWeight)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as BYTE;
    n = 1 as std::ffi::c_int as U32;
    while n < huffLog.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) {
        *((*wksp).bitsToWeight).as_mut_ptr().offset(n as isize) = huffLog
            .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_sub(n)
            as BYTE;
        n = n.wrapping_add(1);
        n;
    }
    n = 0 as std::ffi::c_int as U32;
    while n < maxSymbolValue {
        *((*wksp).huffWeight).as_mut_ptr().offset(n as isize) = *((*wksp).bitsToWeight)
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n as isize)) as isize);
        n = n.wrapping_add(1);
        n;
    }
    if maxDstSize < 1 as std::ffi::c_int as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    let hSize = HUF_compressWeights(
        op.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
        maxDstSize.wrapping_sub(1 as std::ffi::c_int as size_t),
        ((*wksp).huffWeight).as_mut_ptr() as *const std::ffi::c_void,
        maxSymbolValue as size_t,
        &mut (*wksp).wksp as *mut HUF_CompressWeightsWksp as *mut std::ffi::c_void,
        ::core::mem::size_of::<HUF_CompressWeightsWksp>() as std::ffi::c_ulong,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if (hSize > 1 as std::ffi::c_int as size_t) as std::ffi::c_int
        & (hSize < maxSymbolValue.wrapping_div(2 as std::ffi::c_int as std::ffi::c_uint) as size_t)
            as std::ffi::c_int
        != 0
    {
        *op.offset(0 as std::ffi::c_int as isize) = hSize as BYTE;
        return hSize.wrapping_add(1 as std::ffi::c_int as size_t);
    }
    if maxSymbolValue > (256 as std::ffi::c_int - 128 as std::ffi::c_int) as std::ffi::c_uint {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    if maxSymbolValue
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_div(2 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as size_t
        > maxDstSize
    {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    *op.offset(0 as std::ffi::c_int as isize) = (128 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_add(maxSymbolValue.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint))
        as BYTE;
    *((*wksp).huffWeight)
        .as_mut_ptr()
        .offset(maxSymbolValue as isize) = 0 as std::ffi::c_int as BYTE;
    n = 0 as std::ffi::c_int as U32;
    while n < maxSymbolValue {
        *op.offset(
            (n / 2 as std::ffi::c_int as U32).wrapping_add(1 as std::ffi::c_int as U32) as isize,
        ) = (((*((*wksp).huffWeight).as_mut_ptr().offset(n as isize) as std::ffi::c_int)
            << 4 as std::ffi::c_int)
            + *((*wksp).huffWeight)
                .as_mut_ptr()
                .offset(n.wrapping_add(1 as std::ffi::c_int as U32) as isize)
                as std::ffi::c_int) as BYTE;
        n = n.wrapping_add(2 as std::ffi::c_int as U32);
    }
    maxSymbolValue
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_div(2 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as size_t
}
#[no_mangle]
pub unsafe extern "C" fn HUF_readCTable(
    mut CTable: *mut HUF_CElt,
    mut maxSymbolValuePtr: *mut std::ffi::c_uint,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut hasZeroWeights: *mut std::ffi::c_uint,
) -> size_t {
    let mut huffWeight: [BYTE; 256] = [0; 256];
    let mut rankVal: [U32; 13] = [0; 13];
    let mut tableLog = 0 as std::ffi::c_int as U32;
    let mut nbSymbols = 0 as std::ffi::c_int as U32;
    let ct = CTable.offset(1 as std::ffi::c_int as isize);
    let readSize = HUF_readStats(
        &mut huffWeight,
        (255 as std::ffi::c_int + 1 as std::ffi::c_int) as size_t,
        &mut rankVal,
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if ERR_isError(readSize) != 0 {
        return readSize;
    }
    *hasZeroWeights = (*rankVal.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
        > 0 as std::ffi::c_int as U32) as std::ffi::c_int as std::ffi::c_uint;
    if tableLog > HUF_TABLELOG_MAX as U32 {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    if nbSymbols > (*maxSymbolValuePtr).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) {
        return -(ZSTD_error_maxSymbolValue_tooSmall as std::ffi::c_int) as size_t;
    }
    *maxSymbolValuePtr = nbSymbols.wrapping_sub(1 as std::ffi::c_int as U32);
    HUF_writeCTableHeader(CTable, tableLog, *maxSymbolValuePtr);
    let mut n: U32 = 0;
    let mut nextRankStart = 0 as std::ffi::c_int as U32;
    n = 1 as std::ffi::c_int as U32;
    while n <= tableLog {
        let mut curr = nextRankStart;
        nextRankStart = nextRankStart.wrapping_add(
            *rankVal.as_mut_ptr().offset(n as isize) << n.wrapping_sub(1 as std::ffi::c_int as U32),
        );
        *rankVal.as_mut_ptr().offset(n as isize) = curr;
        n = n.wrapping_add(1);
        n;
    }
    let mut n_0: U32 = 0;
    n_0 = 0 as std::ffi::c_int as U32;
    while n_0 < nbSymbols {
        let w = *huffWeight.as_mut_ptr().offset(n_0 as isize) as U32;
        HUF_setNbBits(
            ct.offset(n_0 as isize),
            (tableLog
                .wrapping_add(1 as std::ffi::c_int as U32)
                .wrapping_sub(w) as BYTE as std::ffi::c_int
                & -((w != 0 as std::ffi::c_int as U32) as std::ffi::c_int)) as size_t,
        );
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    let mut nbPerRank: [U16; 14] = [0 as std::ffi::c_int as U16; 14];
    let mut valPerRank: [U16; 14] = [0 as std::ffi::c_int as U16; 14];
    let mut n_1: U32 = 0;
    n_1 = 0 as std::ffi::c_int as U32;
    while n_1 < nbSymbols {
        let fresh0 = &mut (*nbPerRank
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n_1 as isize)) as isize));
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
        n_1 = n_1.wrapping_add(1);
        n_1;
    }
    *valPerRank
        .as_mut_ptr()
        .offset(tableLog.wrapping_add(1 as std::ffi::c_int as U32) as isize) =
        0 as std::ffi::c_int as U16;
    let mut min = 0 as std::ffi::c_int as U16;
    let mut n_2: U32 = 0;
    n_2 = tableLog;
    while n_2 > 0 as std::ffi::c_int as U32 {
        *valPerRank.as_mut_ptr().offset(n_2 as isize) = min;
        min = (min as std::ffi::c_int
            + *nbPerRank.as_mut_ptr().offset(n_2 as isize) as std::ffi::c_int) as U16;
        min = (min as std::ffi::c_int >> 1 as std::ffi::c_int) as U16;
        n_2 = n_2.wrapping_sub(1);
        n_2;
    }
    let mut n_3: U32 = 0;
    n_3 = 0 as std::ffi::c_int as U32;
    while n_3 < nbSymbols {
        let fresh1 = &mut (*valPerRank
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n_3 as isize)) as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        HUF_setValue(ct.offset(n_3 as isize), fresh2 as size_t);
        n_3 = n_3.wrapping_add(1);
        n_3;
    }
    readSize
}
#[no_mangle]
pub unsafe extern "C" fn HUF_getNbBitsFromCTable(
    mut CTable: *const HUF_CElt,
    mut symbolValue: U32,
) -> U32 {
    let ct = CTable.offset(1 as std::ffi::c_int as isize);
    if symbolValue > (HUF_readCTableHeader(CTable)).maxSymbolValue as U32 {
        return 0 as std::ffi::c_int as U32;
    }
    HUF_getNbBits(*ct.offset(symbolValue as isize)) as U32
}
unsafe extern "C" fn HUF_setMaxHeight(
    mut huffNode: *mut nodeElt,
    mut lastNonNull: U32,
    mut targetNbBits: U32,
) -> U32 {
    let largestBits = (*huffNode.offset(lastNonNull as isize)).nbBits as U32;
    if largestBits <= targetNbBits {
        return largestBits;
    }
    let mut totalCost = 0 as std::ffi::c_int;
    let baseCost = ((1 as std::ffi::c_int) << largestBits.wrapping_sub(targetNbBits)) as U32;
    let mut n = lastNonNull as std::ffi::c_int;
    while (*huffNode.offset(n as isize)).nbBits as U32 > targetNbBits {
        totalCost = (totalCost as U32).wrapping_add(baseCost.wrapping_sub(
            ((1 as std::ffi::c_int)
                << largestBits.wrapping_sub((*huffNode.offset(n as isize)).nbBits as U32))
                as U32,
        )) as std::ffi::c_int as std::ffi::c_int;
        (*huffNode.offset(n as isize)).nbBits = targetNbBits as BYTE;
        n -= 1;
        n;
    }
    while (*huffNode.offset(n as isize)).nbBits as U32 == targetNbBits {
        n -= 1;
        n;
    }
    totalCost >>= largestBits.wrapping_sub(targetNbBits);
    let noSymbol = 0xf0f0f0f0 as std::ffi::c_uint;
    let mut rankLast: [U32; 14] = [0; 14];
    libc::memset(
        rankLast.as_mut_ptr() as *mut std::ffi::c_void,
        0xf0 as std::ffi::c_int,
        ::core::mem::size_of::<[U32; 14]>() as std::ffi::c_ulong as libc::size_t,
    );
    let mut currentNbBits = targetNbBits;
    let mut pos: std::ffi::c_int = 0;
    pos = n;
    while pos >= 0 as std::ffi::c_int {
        if ((*huffNode.offset(pos as isize)).nbBits as U32) < currentNbBits {
            currentNbBits = (*huffNode.offset(pos as isize)).nbBits as U32;
            *rankLast
                .as_mut_ptr()
                .offset(targetNbBits.wrapping_sub(currentNbBits) as isize) = pos as U32;
        }
        pos -= 1;
        pos;
    }
    while totalCost > 0 as std::ffi::c_int {
        let mut nBitsToDecrease = (ZSTD_highbit32(totalCost as U32))
            .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
        while nBitsToDecrease > 1 as std::ffi::c_int as U32 {
            let highPos = *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize);
            let lowPos = *rankLast
                .as_mut_ptr()
                .offset(nBitsToDecrease.wrapping_sub(1 as std::ffi::c_int as U32) as isize);
            if highPos != noSymbol {
                if lowPos == noSymbol {
                    break;
                }
                let highTotal = (*huffNode.offset(highPos as isize)).count;
                let lowTotal =
                    2 as std::ffi::c_int as U32 * (*huffNode.offset(lowPos as isize)).count;
                if highTotal <= lowTotal {
                    break;
                }
            }
            nBitsToDecrease = nBitsToDecrease.wrapping_sub(1);
            nBitsToDecrease;
        }
        while nBitsToDecrease <= HUF_TABLELOG_MAX as U32
            && *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) == noSymbol
        {
            nBitsToDecrease = nBitsToDecrease.wrapping_add(1);
            nBitsToDecrease;
        }
        totalCost -=
            (1 as std::ffi::c_int) << nBitsToDecrease.wrapping_sub(1 as std::ffi::c_int as U32);
        let fresh3 = &mut (*huffNode
            .offset(*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) as isize))
        .nbBits;
        *fresh3 = (*fresh3).wrapping_add(1);
        *fresh3;
        if *rankLast
            .as_mut_ptr()
            .offset(nBitsToDecrease.wrapping_sub(1 as std::ffi::c_int as U32) as isize)
            == noSymbol
        {
            *rankLast
                .as_mut_ptr()
                .offset(nBitsToDecrease.wrapping_sub(1 as std::ffi::c_int as U32) as isize) =
                *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize);
        }
        if *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) == 0 as std::ffi::c_int as U32 {
            *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) = noSymbol;
        } else {
            let fresh4 = &mut (*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize));
            *fresh4 = (*fresh4).wrapping_sub(1);
            *fresh4;
            if (*huffNode.offset(*rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) as isize))
                .nbBits as U32
                != targetNbBits.wrapping_sub(nBitsToDecrease)
            {
                *rankLast.as_mut_ptr().offset(nBitsToDecrease as isize) = noSymbol;
            }
        }
    }
    while totalCost < 0 as std::ffi::c_int {
        if *rankLast.as_mut_ptr().offset(1 as std::ffi::c_int as isize) == noSymbol {
            while (*huffNode.offset(n as isize)).nbBits as U32 == targetNbBits {
                n -= 1;
                n;
            }
            let fresh5 = &mut (*huffNode.offset((n + 1 as std::ffi::c_int) as isize)).nbBits;
            *fresh5 = (*fresh5).wrapping_sub(1);
            *fresh5;
            *rankLast.as_mut_ptr().offset(1 as std::ffi::c_int as isize) =
                (n + 1 as std::ffi::c_int) as U32;
            totalCost += 1;
            totalCost;
        } else {
            let fresh6 = &mut (*huffNode.offset(
                (*rankLast.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                    .wrapping_add(1 as std::ffi::c_int as U32) as isize,
            ))
            .nbBits;
            *fresh6 = (*fresh6).wrapping_sub(1);
            *fresh6;
            let fresh7 = &mut (*rankLast.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
            *fresh7 = (*fresh7).wrapping_add(1);
            *fresh7;
            totalCost += 1;
            totalCost;
        }
    }
    targetNbBits
}
pub const RANK_POSITION_TABLE_SIZE: std::ffi::c_int = 192 as std::ffi::c_int;
pub const RANK_POSITION_MAX_COUNT_LOG: std::ffi::c_int = 32 as std::ffi::c_int;
pub const RANK_POSITION_LOG_BUCKETS_BEGIN: std::ffi::c_int = RANK_POSITION_TABLE_SIZE
    - 1 as std::ffi::c_int
    - RANK_POSITION_MAX_COUNT_LOG
    - 1 as std::ffi::c_int;
pub const RANK_POSITION_DISTINCT_COUNT_CUTOFF: std::ffi::c_uint = (RANK_POSITION_LOG_BUCKETS_BEGIN
    as std::ffi::c_uint)
    .wrapping_add(ZSTD_highbit32(RANK_POSITION_LOG_BUCKETS_BEGIN as U32));
unsafe extern "C" fn HUF_getIndex(count: U32) -> U32 {
    if count < RANK_POSITION_DISTINCT_COUNT_CUTOFF {
        count
    } else {
        (ZSTD_highbit32(count)).wrapping_add(RANK_POSITION_LOG_BUCKETS_BEGIN as std::ffi::c_uint)
    }
}
unsafe extern "C" fn HUF_swapNodes(mut a: *mut nodeElt, mut b: *mut nodeElt) {
    core::ptr::swap(a, b);
}
#[inline(always)]
unsafe extern "C" fn HUF_insertionSort(
    mut huffNode: *mut nodeElt,
    low: std::ffi::c_int,
    high: std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let size = high - low + 1 as std::ffi::c_int;
    huffNode = huffNode.offset(low as isize);
    i = 1 as std::ffi::c_int;
    while i < size {
        let key = *huffNode.offset(i as isize);
        let mut j = i - 1 as std::ffi::c_int;
        while j >= 0 as std::ffi::c_int && (*huffNode.offset(j as isize)).count < key.count {
            *huffNode.offset((j + 1 as std::ffi::c_int) as isize) = *huffNode.offset(j as isize);
            j -= 1;
            j;
        }
        *huffNode.offset((j + 1 as std::ffi::c_int) as isize) = key;
        i += 1;
        i;
    }
}
unsafe extern "C" fn HUF_quickSortPartition(
    mut arr: *mut nodeElt,
    low: std::ffi::c_int,
    high: std::ffi::c_int,
) -> std::ffi::c_int {
    let pivot = (*arr.offset(high as isize)).count;
    let mut i = low - 1 as std::ffi::c_int;
    let mut j = low;
    while j < high {
        if (*arr.offset(j as isize)).count > pivot {
            i += 1;
            i;
            HUF_swapNodes(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1;
        j;
    }
    HUF_swapNodes(
        &mut *arr.offset((i + 1 as std::ffi::c_int) as isize),
        &mut *arr.offset(high as isize),
    );
    i + 1 as std::ffi::c_int
}
unsafe extern "C" fn HUF_simpleQuickSort(
    mut arr: *mut nodeElt,
    mut low: std::ffi::c_int,
    mut high: std::ffi::c_int,
) {
    let kInsertionSortThreshold = 8 as std::ffi::c_int;
    if high - low < kInsertionSortThreshold {
        HUF_insertionSort(arr, low, high);
        return;
    }
    while low < high {
        let idx = HUF_quickSortPartition(arr, low, high);
        if idx - low < high - idx {
            HUF_simpleQuickSort(arr, low, idx - 1 as std::ffi::c_int);
            low = idx + 1 as std::ffi::c_int;
        } else {
            HUF_simpleQuickSort(arr, idx + 1 as std::ffi::c_int, high);
            high = idx - 1 as std::ffi::c_int;
        }
    }
}
unsafe extern "C" fn HUF_sort(
    mut huffNode: *mut nodeElt,
    mut count: *const std::ffi::c_uint,
    maxSymbolValue: U32,
    mut rankPosition: *mut rankPos,
) {
    let mut n: U32 = 0;
    let maxSymbolValue1 = maxSymbolValue.wrapping_add(1 as std::ffi::c_int as U32);
    libc::memset(
        rankPosition as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        (::core::mem::size_of::<rankPos>() as std::ffi::c_ulong)
            .wrapping_mul(192 as std::ffi::c_int as std::ffi::c_ulong) as libc::size_t,
    );
    n = 0 as std::ffi::c_int as U32;
    while n < maxSymbolValue1 {
        let mut lowerRank = HUF_getIndex(*count.offset(n as isize));
        let fresh8 = &mut (*rankPosition.offset(lowerRank as isize)).base;
        *fresh8 = (*fresh8).wrapping_add(1);
        *fresh8;
        n = n.wrapping_add(1);
        n;
    }
    n = (RANK_POSITION_TABLE_SIZE - 1 as std::ffi::c_int) as U32;
    while n > 0 as std::ffi::c_int as U32 {
        let fresh9 =
            &mut (*rankPosition.offset(n.wrapping_sub(1 as std::ffi::c_int as U32) as isize)).base;
        *fresh9 = (*fresh9 as std::ffi::c_int
            + (*rankPosition.offset(n as isize)).base as std::ffi::c_int) as U16;
        (*rankPosition.offset(n.wrapping_sub(1 as std::ffi::c_int as U32) as isize)).curr =
            (*rankPosition.offset(n.wrapping_sub(1 as std::ffi::c_int as U32) as isize)).base;
        n = n.wrapping_sub(1);
        n;
    }
    n = 0 as std::ffi::c_int as U32;
    while n < maxSymbolValue1 {
        let c = *count.offset(n as isize);
        let r = (HUF_getIndex(c)).wrapping_add(1 as std::ffi::c_int as U32);
        let fresh10 = &mut (*rankPosition.offset(r as isize)).curr;
        let fresh11 = *fresh10;
        *fresh10 = (*fresh10).wrapping_add(1);
        let pos = fresh11 as U32;
        (*huffNode.offset(pos as isize)).count = c;
        (*huffNode.offset(pos as isize)).byte = n as BYTE;
        n = n.wrapping_add(1);
        n;
    }
    n = RANK_POSITION_DISTINCT_COUNT_CUTOFF;
    while n < (RANK_POSITION_TABLE_SIZE - 1 as std::ffi::c_int) as U32 {
        let bucketSize = (*rankPosition.offset(n as isize)).curr as std::ffi::c_int
            - (*rankPosition.offset(n as isize)).base as std::ffi::c_int;
        let bucketStartIdx = (*rankPosition.offset(n as isize)).base as U32;
        if bucketSize > 1 as std::ffi::c_int {
            HUF_simpleQuickSort(
                huffNode.offset(bucketStartIdx as isize),
                0 as std::ffi::c_int,
                bucketSize - 1 as std::ffi::c_int,
            );
        }
        n = n.wrapping_add(1);
        n;
    }
}
pub const STARTNODE: std::ffi::c_int = HUF_SYMBOLVALUE_MAX + 1 as std::ffi::c_int;
unsafe extern "C" fn HUF_buildTree(
    mut huffNode: *mut nodeElt,
    mut maxSymbolValue: U32,
) -> std::ffi::c_int {
    let huffNode0 = huffNode.offset(-(1 as std::ffi::c_int as isize));
    let mut nonNullRank: std::ffi::c_int = 0;
    let mut lowS: std::ffi::c_int = 0;
    let mut lowN: std::ffi::c_int = 0;
    let mut nodeNb = STARTNODE;
    let mut n: std::ffi::c_int = 0;
    let mut nodeRoot: std::ffi::c_int = 0;
    nonNullRank = maxSymbolValue as std::ffi::c_int;
    while (*huffNode.offset(nonNullRank as isize)).count == 0 as std::ffi::c_int as U32 {
        nonNullRank -= 1;
        nonNullRank;
    }
    lowS = nonNullRank;
    nodeRoot = nodeNb + lowS - 1 as std::ffi::c_int;
    lowN = nodeNb;
    (*huffNode.offset(nodeNb as isize)).count = ((*huffNode.offset(lowS as isize)).count)
        .wrapping_add((*huffNode.offset((lowS - 1 as std::ffi::c_int) as isize)).count);
    let fresh12 = &mut (*huffNode.offset((lowS - 1 as std::ffi::c_int) as isize)).parent;
    *fresh12 = nodeNb as U16;
    (*huffNode.offset(lowS as isize)).parent = *fresh12;
    nodeNb += 1;
    nodeNb;
    lowS -= 2 as std::ffi::c_int;
    n = nodeNb;
    while n <= nodeRoot {
        (*huffNode.offset(n as isize)).count = (1 as std::ffi::c_uint) << 30 as std::ffi::c_int;
        n += 1;
        n;
    }
    (*huffNode0.offset(0 as std::ffi::c_int as isize)).count =
        (1 as std::ffi::c_uint) << 31 as std::ffi::c_int;
    while nodeNb <= nodeRoot {
        let n1 =
            if (*huffNode.offset(lowS as isize)).count < (*huffNode.offset(lowN as isize)).count {
                let fresh13 = lowS;
                lowS -= 1;
                fresh13
            } else {
                let fresh14 = lowN;
                lowN += 1;
                fresh14
            };
        let n2 =
            if (*huffNode.offset(lowS as isize)).count < (*huffNode.offset(lowN as isize)).count {
                let fresh15 = lowS;
                lowS -= 1;
                fresh15
            } else {
                let fresh16 = lowN;
                lowN += 1;
                fresh16
            };
        (*huffNode.offset(nodeNb as isize)).count = ((*huffNode.offset(n1 as isize)).count)
            .wrapping_add((*huffNode.offset(n2 as isize)).count);
        let fresh17 = &mut (*huffNode.offset(n2 as isize)).parent;
        *fresh17 = nodeNb as U16;
        (*huffNode.offset(n1 as isize)).parent = *fresh17;
        nodeNb += 1;
        nodeNb;
    }
    (*huffNode.offset(nodeRoot as isize)).nbBits = 0 as std::ffi::c_int as BYTE;
    n = nodeRoot - 1 as std::ffi::c_int;
    while n >= STARTNODE {
        (*huffNode.offset(n as isize)).nbBits = ((*huffNode
            .offset((*huffNode.offset(n as isize)).parent as isize))
        .nbBits as std::ffi::c_int
            + 1 as std::ffi::c_int) as BYTE;
        n -= 1;
        n;
    }
    n = 0 as std::ffi::c_int;
    while n <= nonNullRank {
        (*huffNode.offset(n as isize)).nbBits = ((*huffNode
            .offset((*huffNode.offset(n as isize)).parent as isize))
        .nbBits as std::ffi::c_int
            + 1 as std::ffi::c_int) as BYTE;
        n += 1;
        n;
    }
    nonNullRank
}
unsafe extern "C" fn HUF_buildCTableFromTree(
    mut CTable: *mut HUF_CElt,
    mut huffNode: *const nodeElt,
    mut nonNullRank: std::ffi::c_int,
    mut maxSymbolValue: U32,
    mut maxNbBits: U32,
) {
    let ct = CTable.offset(1 as std::ffi::c_int as isize);
    let mut n: std::ffi::c_int = 0;
    let mut nbPerRank: [U16; 13] = [0 as std::ffi::c_int as U16; 13];
    let mut valPerRank: [U16; 13] = [0 as std::ffi::c_int as U16; 13];
    let alphabetSize = maxSymbolValue.wrapping_add(1 as std::ffi::c_int as U32) as std::ffi::c_int;
    n = 0 as std::ffi::c_int;
    while n <= nonNullRank {
        let fresh18 = &mut (*nbPerRank
            .as_mut_ptr()
            .offset((*huffNode.offset(n as isize)).nbBits as isize));
        *fresh18 = (*fresh18).wrapping_add(1);
        *fresh18;
        n += 1;
        n;
    }
    let mut min = 0 as std::ffi::c_int as U16;
    n = maxNbBits as std::ffi::c_int;
    while n > 0 as std::ffi::c_int {
        *valPerRank.as_mut_ptr().offset(n as isize) = min;
        min = (min as std::ffi::c_int
            + *nbPerRank.as_mut_ptr().offset(n as isize) as std::ffi::c_int) as U16;
        min = (min as std::ffi::c_int >> 1 as std::ffi::c_int) as U16;
        n -= 1;
        n;
    }
    n = 0 as std::ffi::c_int;
    while n < alphabetSize {
        HUF_setNbBits(
            ct.offset((*huffNode.offset(n as isize)).byte as std::ffi::c_int as isize),
            (*huffNode.offset(n as isize)).nbBits as size_t,
        );
        n += 1;
        n;
    }
    n = 0 as std::ffi::c_int;
    while n < alphabetSize {
        let fresh19 = &mut (*valPerRank
            .as_mut_ptr()
            .offset(HUF_getNbBits(*ct.offset(n as isize)) as isize));
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).wrapping_add(1);
        HUF_setValue(ct.offset(n as isize), fresh20 as size_t);
        n += 1;
        n;
    }
    HUF_writeCTableHeader(CTable, maxNbBits, maxSymbolValue);
}
#[no_mangle]
pub unsafe extern "C" fn HUF_buildCTable_wksp(
    mut CTable: *mut HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: U32,
    mut maxNbBits: U32,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) -> size_t {
    let wksp_tables = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<U32>() as std::ffi::c_ulong,
    ) as *mut HUF_buildCTable_wksp_tables;
    let huffNode0 = ((*wksp_tables).huffNodeTbl).as_mut_ptr();
    let huffNode = huffNode0.offset(1 as std::ffi::c_int as isize);
    let mut nonNullRank: std::ffi::c_int = 0;
    if wkspSize < ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() as std::ffi::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as std::ffi::c_int) as size_t;
    }
    if maxNbBits == 0 as std::ffi::c_int as U32 {
        maxNbBits = HUF_TABLELOG_DEFAULT as U32;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as U32 {
        return -(ZSTD_error_maxSymbolValue_tooLarge as std::ffi::c_int) as size_t;
    }
    libc::memset(
        huffNode0 as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<huffNodeTable>() as std::ffi::c_ulong as libc::size_t,
    );
    HUF_sort(
        huffNode,
        count,
        maxSymbolValue,
        ((*wksp_tables).rankPosition).as_mut_ptr(),
    );
    nonNullRank = HUF_buildTree(huffNode, maxSymbolValue);
    maxNbBits = HUF_setMaxHeight(huffNode, nonNullRank as U32, maxNbBits);
    if maxNbBits > HUF_TABLELOG_MAX as U32 {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    HUF_buildCTableFromTree(CTable, huffNode, nonNullRank, maxSymbolValue, maxNbBits);
    maxNbBits as size_t
}
#[no_mangle]
pub unsafe extern "C" fn HUF_estimateCompressedSize(
    mut CTable: *const HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: std::ffi::c_uint,
) -> size_t {
    let mut ct = CTable.offset(1 as std::ffi::c_int as isize);
    let mut nbBits = 0 as std::ffi::c_int as size_t;
    let mut s: std::ffi::c_int = 0;
    s = 0 as std::ffi::c_int;
    while s <= maxSymbolValue as std::ffi::c_int {
        nbBits = nbBits.wrapping_add(
            HUF_getNbBits(*ct.offset(s as isize)) * *count.offset(s as isize) as size_t,
        );
        s += 1;
        s;
    }
    nbBits >> 3 as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn HUF_validateCTable(
    mut CTable: *const HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: std::ffi::c_uint,
) -> std::ffi::c_int {
    let mut header = HUF_readCTableHeader(CTable);
    let mut ct = CTable.offset(1 as std::ffi::c_int as isize);
    let mut bad = 0 as std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    if (header.maxSymbolValue as std::ffi::c_uint) < maxSymbolValue {
        return 0 as std::ffi::c_int;
    }
    s = 0 as std::ffi::c_int;
    while s <= maxSymbolValue as std::ffi::c_int {
        bad |= (*count.offset(s as isize) != 0 as std::ffi::c_int as std::ffi::c_uint)
            as std::ffi::c_int
            & (HUF_getNbBits(*ct.offset(s as isize)) == 0 as std::ffi::c_int as size_t)
                as std::ffi::c_int;
        s += 1;
        s;
    }
    (bad == 0) as std::ffi::c_int
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compressBound(mut size: size_t) -> size_t {
    (HUF_CTABLEBOUND as size_t).wrapping_add(
        size.wrapping_add(size >> 8 as std::ffi::c_int)
            .wrapping_add(8 as std::ffi::c_int as size_t),
    )
}
pub const HUF_BITS_IN_CONTAINER: std::ffi::c_ulong = (::core::mem::size_of::<size_t>()
    as std::ffi::c_ulong)
    .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong);
unsafe extern "C" fn HUF_initCStream(
    mut bitC: *mut HUF_CStream_t,
    mut startPtr: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    libc::memset(
        bitC as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<HUF_CStream_t>() as std::ffi::c_ulong as libc::size_t,
    );
    (*bitC).startPtr = startPtr as *mut BYTE;
    (*bitC).ptr = (*bitC).startPtr;
    (*bitC).endPtr = ((*bitC).startPtr)
        .offset(dstCapacity as isize)
        .offset(-(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize));
    if dstCapacity <= ::core::mem::size_of::<size_t>() as std::ffi::c_ulong {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    0 as std::ffi::c_int as size_t
}
#[inline(always)]
unsafe extern "C" fn HUF_addBits(
    mut bitC: *mut HUF_CStream_t,
    mut elt: HUF_CElt,
    mut idx: std::ffi::c_int,
    mut kFast: std::ffi::c_int,
) {
    *((*bitC).bitContainer).as_mut_ptr().offset(idx as isize) >>= HUF_getNbBits(elt);
    *((*bitC).bitContainer).as_mut_ptr().offset(idx as isize) |= if kFast != 0 {
        HUF_getValueFast(elt)
    } else {
        HUF_getValue(elt)
    };
    let fresh21 = &mut (*((*bitC).bitPos).as_mut_ptr().offset(idx as isize));
    *fresh21 = (*fresh21).wrapping_add(HUF_getNbBitsFast(elt));
}
#[inline(always)]
unsafe extern "C" fn HUF_zeroIndex1(mut bitC: *mut HUF_CStream_t) {
    *((*bitC).bitContainer)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as size_t;
    *((*bitC).bitPos)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as size_t;
}
#[inline(always)]
unsafe extern "C" fn HUF_mergeIndex1(mut bitC: *mut HUF_CStream_t) {
    *((*bitC).bitContainer)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) >>= *((*bitC).bitPos)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize)
        & 0xff as std::ffi::c_int as size_t;
    *((*bitC).bitContainer)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) |= *((*bitC).bitContainer)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize);
    let fresh22 = &mut (*((*bitC).bitPos)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize));
    *fresh22 = (*fresh22).wrapping_add(
        *((*bitC).bitPos)
            .as_mut_ptr()
            .offset(1 as std::ffi::c_int as isize),
    );
}
#[inline(always)]
unsafe extern "C" fn HUF_flushBits(mut bitC: *mut HUF_CStream_t, mut kFast: std::ffi::c_int) {
    let nbBits = *((*bitC).bitPos)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize)
        & 0xff as std::ffi::c_int as size_t;
    let nbBytes = nbBits >> 3 as std::ffi::c_int;
    let bitContainer = *((*bitC).bitContainer)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize)
        >> HUF_BITS_IN_CONTAINER.wrapping_sub(nbBits);
    *((*bitC).bitPos)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) &= 7 as std::ffi::c_int as size_t;
    MEM_writeLEST((*bitC).ptr as *mut std::ffi::c_void, bitContainer);
    (*bitC).ptr = ((*bitC).ptr).offset(nbBytes as isize);
    if kFast == 0 && (*bitC).ptr > (*bitC).endPtr {
        (*bitC).ptr = (*bitC).endPtr;
    }
}
unsafe extern "C" fn HUF_endMark() -> HUF_CElt {
    let mut endMark: HUF_CElt = 0;
    HUF_setNbBits(&mut endMark, 1 as std::ffi::c_int as size_t);
    HUF_setValue(&mut endMark, 1 as std::ffi::c_int as size_t);
    endMark
}
unsafe extern "C" fn HUF_closeCStream(mut bitC: *mut HUF_CStream_t) -> size_t {
    HUF_addBits(
        bitC,
        HUF_endMark(),
        0 as std::ffi::c_int,
        0 as std::ffi::c_int,
    );
    HUF_flushBits(bitC, 0 as std::ffi::c_int);
    let nbBits = *((*bitC).bitPos)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize)
        & 0xff as std::ffi::c_int as size_t;
    if (*bitC).ptr >= (*bitC).endPtr {
        return 0 as std::ffi::c_int as size_t;
    }
    (((*bitC).ptr).offset_from((*bitC).startPtr) as std::ffi::c_long as size_t)
        .wrapping_add((nbBits > 0 as std::ffi::c_int as size_t) as std::ffi::c_int as size_t)
}
#[inline(always)]
unsafe extern "C" fn HUF_encodeSymbol(
    mut bitCPtr: *mut HUF_CStream_t,
    mut symbol: U32,
    mut CTable: *const HUF_CElt,
    mut idx: std::ffi::c_int,
    mut fast: std::ffi::c_int,
) {
    HUF_addBits(bitCPtr, *CTable.offset(symbol as isize), idx, fast);
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body_loop(
    mut bitC: *mut HUF_CStream_t,
    mut ip: *const BYTE,
    mut srcSize: size_t,
    mut ct: *const HUF_CElt,
    mut kUnroll: std::ffi::c_int,
    mut kFastFlush: std::ffi::c_int,
    mut kLastFast: std::ffi::c_int,
) {
    let mut n = srcSize as std::ffi::c_int;
    let mut rem = n % kUnroll;
    if rem > 0 as std::ffi::c_int {
        while rem > 0 as std::ffi::c_int {
            n -= 1;
            HUF_encodeSymbol(
                bitC,
                *ip.offset(n as isize) as U32,
                ct,
                0 as std::ffi::c_int,
                0 as std::ffi::c_int,
            );
            rem -= 1;
            rem;
        }
        HUF_flushBits(bitC, kFastFlush);
    }
    if n % (2 as std::ffi::c_int * kUnroll) != 0 {
        let mut u: std::ffi::c_int = 0;
        u = 1 as std::ffi::c_int;
        while u < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u) as isize) as U32,
                ct,
                0 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
            u += 1;
            u;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as U32,
            ct,
            0 as std::ffi::c_int,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        n -= kUnroll;
    }
    while n > 0 as std::ffi::c_int {
        let mut u_0: std::ffi::c_int = 0;
        u_0 = 1 as std::ffi::c_int;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - u_0) as isize) as U32,
                ct,
                0 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
            u_0 += 1;
            u_0;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll) as isize) as U32,
            ct,
            0 as std::ffi::c_int,
            kLastFast,
        );
        HUF_flushBits(bitC, kFastFlush);
        HUF_zeroIndex1(bitC);
        u_0 = 1 as std::ffi::c_int;
        while u_0 < kUnroll {
            HUF_encodeSymbol(
                bitC,
                *ip.offset((n - kUnroll - u_0) as isize) as U32,
                ct,
                1 as std::ffi::c_int,
                1 as std::ffi::c_int,
            );
            u_0 += 1;
            u_0;
        }
        HUF_encodeSymbol(
            bitC,
            *ip.offset((n - kUnroll - kUnroll) as isize) as U32,
            ct,
            1 as std::ffi::c_int,
            kLastFast,
        );
        HUF_mergeIndex1(bitC);
        HUF_flushBits(bitC, kFastFlush);
        n -= 2 as std::ffi::c_int * kUnroll;
    }
}
unsafe extern "C" fn HUF_tightCompressBound(mut srcSize: size_t, mut tableLog: size_t) -> size_t {
    ((srcSize * tableLog) >> 3 as std::ffi::c_int).wrapping_add(8 as std::ffi::c_int as size_t)
}
#[inline(always)]
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    let tableLog = (HUF_readCTableHeader(CTable)).tableLog as U32;
    let mut ct = CTable.offset(1 as std::ffi::c_int as isize);
    let mut ip = src as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut bitC = HUF_CStream_t {
        bitContainer: [0; 2],
        bitPos: [0; 2],
        startPtr: std::ptr::null_mut::<BYTE>(),
        ptr: std::ptr::null_mut::<BYTE>(),
        endPtr: std::ptr::null_mut::<BYTE>(),
    };
    if dstSize < 8 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    let mut op = ostart;
    let initErr = HUF_initCStream(
        &mut bitC,
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as size_t,
    );
    if ERR_isError(initErr) != 0 {
        return 0 as std::ffi::c_int as size_t;
    }
    if dstSize < HUF_tightCompressBound(srcSize, tableLog as size_t)
        || tableLog > 11 as std::ffi::c_int as U32
    {
        HUF_compress1X_usingCTable_internal_body_loop(
            &mut bitC,
            ip,
            srcSize,
            ct,
            if MEM_32bits() != 0 {
                2 as std::ffi::c_int
            } else {
                4 as std::ffi::c_int
            },
            0 as std::ffi::c_int,
            0 as std::ffi::c_int,
        );
    } else if MEM_32bits() != 0 {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    2 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
            }
            8..=10 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    2 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                );
            }
            7 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    3 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                );
            }
        }
    } else {
        match tableLog {
            11 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    5 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
            }
            10 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    5 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                );
            }
            9 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    6 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
            }
            8 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    7 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
            }
            7 => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    8 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    0 as std::ffi::c_int,
                );
            }
            6 | _ => {
                HUF_compress1X_usingCTable_internal_body_loop(
                    &mut bitC,
                    ip,
                    srcSize,
                    ct,
                    9 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                    1 as std::ffi::c_int,
                );
            }
        }
    }
    HUF_closeCStream(&mut bitC)
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
) -> size_t {
    HUF_compress1X_usingCTable_internal_body(dst, dstSize, src, srcSize, CTable)
}
unsafe extern "C" fn HUF_compress1X_usingCTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    flags: std::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
        return HUF_compress1X_usingCTable_internal_bmi2(dst, dstSize, src, srcSize, CTable);
    }
    HUF_compress1X_usingCTable_internal_default(dst, dstSize, src, srcSize, CTable)
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X_usingCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: std::ffi::c_int,
) -> size_t {
    HUF_compress1X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}
unsafe extern "C" fn HUF_compress4X_usingCTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: std::ffi::c_int,
) -> size_t {
    let segmentSize =
        srcSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t;
    let mut ip = src as *const BYTE;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if dstSize
        < (6 as std::ffi::c_int
            + 1 as std::ffi::c_int
            + 1 as std::ffi::c_int
            + 1 as std::ffi::c_int
            + 8 as std::ffi::c_int) as size_t
    {
        return 0 as std::ffi::c_int as size_t;
    }
    if srcSize < 12 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    op = op.offset(6 as std::ffi::c_int as isize);
    let cSize = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as size_t,
        ip as *const std::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 as std::ffi::c_int as size_t || cSize > 65535 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    MEM_writeLE16(ostart as *mut std::ffi::c_void, cSize as U16);
    op = op.offset(cSize as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_0 = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as size_t,
        ip as *const std::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_0) != 0 {
        return cSize_0;
    }
    if cSize_0 == 0 as std::ffi::c_int as size_t || cSize_0 > 65535 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    MEM_writeLE16(
        ostart.offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
        cSize_0 as U16,
    );
    op = op.offset(cSize_0 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_1 = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as size_t,
        ip as *const std::ffi::c_void,
        segmentSize,
        CTable,
        flags,
    );
    if ERR_isError(cSize_1) != 0 {
        return cSize_1;
    }
    if cSize_1 == 0 as std::ffi::c_int as size_t || cSize_1 > 65535 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    MEM_writeLE16(
        ostart.offset(4 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
        cSize_1 as U16,
    );
    op = op.offset(cSize_1 as isize);
    ip = ip.offset(segmentSize as isize);
    let cSize_2 = HUF_compress1X_usingCTable_internal(
        op as *mut std::ffi::c_void,
        oend.offset_from(op) as std::ffi::c_long as size_t,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
        CTable,
        flags,
    );
    if ERR_isError(cSize_2) != 0 {
        return cSize_2;
    }
    if cSize_2 == 0 as std::ffi::c_int as size_t || cSize_2 > 65535 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    op = op.offset(cSize_2 as isize);
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_usingCTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut CTable: *const HUF_CElt,
    mut flags: std::ffi::c_int,
) -> size_t {
    HUF_compress4X_usingCTable_internal(dst, dstSize, src, srcSize, CTable, flags)
}
unsafe extern "C" fn HUF_compressCTable_internal(
    ostart: *mut BYTE,
    mut op: *mut BYTE,
    oend: *mut BYTE,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut nbStreams: HUF_nbStreams_e,
    mut CTable: *const HUF_CElt,
    flags: std::ffi::c_int,
) -> size_t {
    let cSize = if nbStreams as std::ffi::c_uint
        == HUF_singleStream as std::ffi::c_int as std::ffi::c_uint
    {
        HUF_compress1X_usingCTable_internal(
            op as *mut std::ffi::c_void,
            oend.offset_from(op) as std::ffi::c_long as size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable_internal(
            op as *mut std::ffi::c_void,
            oend.offset_from(op) as std::ffi::c_long as size_t,
            src,
            srcSize,
            CTable,
            flags,
        )
    };
    if ERR_isError(cSize) != 0 {
        return cSize;
    }
    if cSize == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    op = op.offset(cSize as isize);
    if op.offset_from(ostart) as std::ffi::c_long as size_t
        >= srcSize.wrapping_sub(1 as std::ffi::c_int as size_t)
    {
        return 0 as std::ffi::c_int as size_t;
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE: std::ffi::c_int = 4096 as std::ffi::c_int;
pub const SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO: std::ffi::c_int = 10 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn HUF_cardinality(
    mut count: *const std::ffi::c_uint,
    mut maxSymbolValue: std::ffi::c_uint,
) -> std::ffi::c_uint {
    let mut cardinality = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut i: std::ffi::c_uint = 0;
    i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) {
        if *count.offset(i as isize) != 0 as std::ffi::c_int as std::ffi::c_uint {
            cardinality = cardinality.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
        }
        i = i.wrapping_add(1);
        i;
    }
    cardinality
}
#[no_mangle]
pub unsafe extern "C" fn HUF_minTableLog(
    mut symbolCardinality: std::ffi::c_uint,
) -> std::ffi::c_uint {
    (ZSTD_highbit32(symbolCardinality)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
}
#[no_mangle]
pub unsafe extern "C" fn HUF_optimalTableLog(
    mut maxTableLog: std::ffi::c_uint,
    mut srcSize: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut table: *mut HUF_CElt,
    mut count: *const std::ffi::c_uint,
    mut flags: std::ffi::c_int,
) -> std::ffi::c_uint {
    if flags & HUF_flags_optimalDepth as std::ffi::c_int == 0 {
        return FSE_optimalTableLog_internal(
            maxTableLog,
            srcSize,
            maxSymbolValue,
            1 as std::ffi::c_int as std::ffi::c_uint,
        );
    }
    let mut dst = (workSpace as *mut BYTE)
        .offset(::core::mem::size_of::<HUF_WriteCTableWksp>() as std::ffi::c_ulong as isize);
    let mut dstSize =
        wkspSize.wrapping_sub(::core::mem::size_of::<HUF_WriteCTableWksp>() as std::ffi::c_ulong);
    let mut hSize: size_t = 0;
    let mut newSize: size_t = 0;
    let symbolCardinality = HUF_cardinality(count, maxSymbolValue);
    let minTableLog = HUF_minTableLog(symbolCardinality);
    let mut optSize =
        (!(0 as std::ffi::c_int) as size_t).wrapping_sub(1 as std::ffi::c_int as size_t);
    let mut optLog = maxTableLog;
    let mut optLogGuess: std::ffi::c_uint = 0;
    optLogGuess = minTableLog;
    while optLogGuess <= maxTableLog {
        let mut maxBits = HUF_buildCTable_wksp(
            table,
            count,
            maxSymbolValue,
            optLogGuess,
            workSpace,
            wkspSize,
        );
        if ERR_isError(maxBits) == 0 {
            if maxBits < optLogGuess as size_t && optLogGuess > minTableLog {
                break;
            }
            hSize = HUF_writeCTable_wksp(
                dst as *mut std::ffi::c_void,
                dstSize,
                table,
                maxSymbolValue,
                maxBits as U32,
                workSpace,
                wkspSize,
            );
            if ERR_isError(hSize) == 0 {
                newSize =
                    (HUF_estimateCompressedSize(table, count, maxSymbolValue)).wrapping_add(hSize);
                if newSize > optSize.wrapping_add(1 as std::ffi::c_int as size_t) {
                    break;
                }
                if newSize < optSize {
                    optSize = newSize;
                    optLog = optLogGuess;
                }
            }
        }
        optLogGuess = optLogGuess.wrapping_add(1);
        optLogGuess;
    }
    optLog
}
unsafe extern "C" fn HUF_compress_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut nbStreams: HUF_nbStreams_e,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut oldHufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: std::ffi::c_int,
) -> size_t {
    let table = HUF_alignUpWorkspace(
        workSpace,
        &mut wkspSize,
        ::core::mem::align_of::<size_t>() as std::ffi::c_ulong,
    ) as *mut HUF_compress_tables_t;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let mut op = ostart;
    if wkspSize < ::core::mem::size_of::<HUF_compress_tables_t>() as std::ffi::c_ulong {
        return -(ZSTD_error_workSpace_tooSmall as std::ffi::c_int) as size_t;
    }
    if srcSize == 0 {
        return 0 as std::ffi::c_int as size_t;
    }
    if dstSize == 0 {
        return 0 as std::ffi::c_int as size_t;
    }
    if srcSize > HUF_BLOCKSIZE_MAX as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if huffLog > HUF_TABLELOG_MAX as std::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    if maxSymbolValue > HUF_SYMBOLVALUE_MAX as std::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as std::ffi::c_int) as size_t;
    }
    if maxSymbolValue == 0 {
        maxSymbolValue = HUF_SYMBOLVALUE_MAX as std::ffi::c_uint;
    }
    if huffLog == 0 {
        huffLog = HUF_TABLELOG_DEFAULT as std::ffi::c_uint;
    }
    if flags & HUF_flags_preferRepeat as std::ffi::c_int != 0
        && !repeat.is_null()
        && *repeat as std::ffi::c_uint == HUF_repeat_valid as std::ffi::c_int as std::ffi::c_uint
    {
        return HUF_compressCTable_internal(
            ostart,
            op,
            oend,
            src,
            srcSize,
            nbStreams,
            oldHufTable,
            flags,
        );
    }
    if flags & HUF_flags_suspectUncompressible as std::ffi::c_int != 0
        && srcSize
            >= (SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE * SUSPECT_INCOMPRESSIBLE_SAMPLE_RATIO) as size_t
    {
        let mut largestTotal = 0 as std::ffi::c_int as size_t;
        let mut maxSymbolValueBegin = maxSymbolValue;
        let largestBegin = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueBegin,
            src as *const BYTE as *const std::ffi::c_void,
            4096 as std::ffi::c_int as size_t,
        ) as size_t;
        if ERR_isError(largestBegin) != 0 {
            return largestBegin;
        }
        largestTotal = largestTotal.wrapping_add(largestBegin);
        let mut maxSymbolValueEnd = maxSymbolValue;
        let largestEnd = HIST_count_simple(
            ((*table).count).as_mut_ptr(),
            &mut maxSymbolValueEnd,
            (src as *const BYTE)
                .offset(srcSize as isize)
                .offset(-(4096 as std::ffi::c_int as isize)) as *const std::ffi::c_void,
            4096 as std::ffi::c_int as size_t,
        ) as size_t;
        if ERR_isError(largestEnd) != 0 {
            return largestEnd;
        }
        largestTotal = largestTotal.wrapping_add(largestEnd);
        if largestTotal
            <= (((2 as std::ffi::c_int * SUSPECT_INCOMPRESSIBLE_SAMPLE_SIZE)
                >> 7 as std::ffi::c_int)
                + 4 as std::ffi::c_int) as size_t
        {
            return 0 as std::ffi::c_int as size_t;
        }
    }
    let largest = HIST_count_wksp(
        ((*table).count).as_mut_ptr(),
        &mut maxSymbolValue,
        src as *const BYTE as *const std::ffi::c_void,
        srcSize,
        ((*table).wksps.hist_wksp).as_mut_ptr() as *mut std::ffi::c_void,
        ::core::mem::size_of::<[U32; 1024]>() as std::ffi::c_ulong,
    );
    if ERR_isError(largest) != 0 {
        return largest;
    }
    if largest == srcSize {
        *ostart = *(src as *const BYTE).offset(0 as std::ffi::c_int as isize);
        return 1 as std::ffi::c_int as size_t;
    }
    if largest <= (srcSize >> 7 as std::ffi::c_int).wrapping_add(4 as std::ffi::c_int as size_t) {
        return 0 as std::ffi::c_int as size_t;
    }
    if !repeat.is_null()
        && *repeat as std::ffi::c_uint == HUF_repeat_check as std::ffi::c_int as std::ffi::c_uint
        && HUF_validateCTable(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue) == 0
    {
        *repeat = HUF_repeat_none;
    }
    if flags & HUF_flags_preferRepeat as std::ffi::c_int != 0
        && !repeat.is_null()
        && *repeat as std::ffi::c_uint != HUF_repeat_none as std::ffi::c_int as std::ffi::c_uint
    {
        return HUF_compressCTable_internal(
            ostart,
            op,
            oend,
            src,
            srcSize,
            nbStreams,
            oldHufTable,
            flags,
        );
    }
    huffLog = HUF_optimalTableLog(
        huffLog,
        srcSize,
        maxSymbolValue,
        &mut (*table).wksps as *mut C2RustUnnamed_1 as *mut std::ffi::c_void,
        ::core::mem::size_of::<C2RustUnnamed_1>() as std::ffi::c_ulong,
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        flags,
    );
    let maxBits = HUF_buildCTable_wksp(
        ((*table).CTable).as_mut_ptr(),
        ((*table).count).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.buildCTable_wksp as *mut HUF_buildCTable_wksp_tables
            as *mut std::ffi::c_void,
        ::core::mem::size_of::<HUF_buildCTable_wksp_tables>() as std::ffi::c_ulong,
    );
    let _var_err__ = maxBits;
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    huffLog = maxBits as U32;
    let hSize = HUF_writeCTable_wksp(
        op as *mut std::ffi::c_void,
        dstSize,
        ((*table).CTable).as_mut_ptr(),
        maxSymbolValue,
        huffLog,
        &mut (*table).wksps.writeCTable_wksp as *mut HUF_WriteCTableWksp as *mut std::ffi::c_void,
        ::core::mem::size_of::<HUF_WriteCTableWksp>() as std::ffi::c_ulong,
    );
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if !repeat.is_null()
        && *repeat as std::ffi::c_uint != HUF_repeat_none as std::ffi::c_int as std::ffi::c_uint
    {
        let oldSize =
            HUF_estimateCompressedSize(oldHufTable, ((*table).count).as_mut_ptr(), maxSymbolValue);
        let newSize = HUF_estimateCompressedSize(
            ((*table).CTable).as_mut_ptr(),
            ((*table).count).as_mut_ptr(),
            maxSymbolValue,
        );
        if oldSize <= hSize.wrapping_add(newSize)
            || hSize.wrapping_add(12 as std::ffi::c_int as size_t) >= srcSize
        {
            return HUF_compressCTable_internal(
                ostart,
                op,
                oend,
                src,
                srcSize,
                nbStreams,
                oldHufTable,
                flags,
            );
        }
    }
    if hSize.wrapping_add(12 as std::ffi::c_ulong) >= srcSize {
        return 0 as std::ffi::c_int as size_t;
    }
    op = op.offset(hSize as isize);
    if !repeat.is_null() {
        *repeat = HUF_repeat_none;
    }
    if !oldHufTable.is_null() {
        libc::memcpy(
            oldHufTable as *mut std::ffi::c_void,
            ((*table).CTable).as_mut_ptr() as *const std::ffi::c_void,
            ::core::mem::size_of::<[HUF_CElt; 257]>() as std::ffi::c_ulong as libc::size_t,
        );
    }
    HUF_compressCTable_internal(
        ostart,
        op,
        oend,
        src,
        srcSize,
        nbStreams,
        ((*table).CTable).as_mut_ptr(),
        flags,
    )
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress1X_repeat(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: std::ffi::c_int,
) -> size_t {
    HUF_compress_internal(
        dst,
        dstSize,
        src,
        srcSize,
        maxSymbolValue,
        huffLog,
        HUF_singleStream,
        workSpace,
        wkspSize,
        hufTable,
        repeat,
        flags,
    )
}
#[no_mangle]
pub unsafe extern "C" fn HUF_compress4X_repeat(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut maxSymbolValue: std::ffi::c_uint,
    mut huffLog: std::ffi::c_uint,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut hufTable: *mut HUF_CElt,
    mut repeat: *mut HUF_repeat,
    mut flags: std::ffi::c_int,
) -> size_t {
    HUF_compress_internal(
        dst,
        dstSize,
        src,
        srcSize,
        maxSymbolValue,
        huffLog,
        HUF_fourStreams,
        workSpace,
        wkspSize,
        hufTable,
        repeat,
        flags,
    )
}
