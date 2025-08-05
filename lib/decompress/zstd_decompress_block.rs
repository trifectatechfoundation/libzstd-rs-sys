use core::arch::asm;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
use std::ptr;

use crate::lib::common::entropy_common::FSE_readNCount;
use crate::lib::decompress::huf_decompress::HUF_decompress4X_hufOnly_wksp;
use crate::lib::decompress::huf_decompress::{
    HUF_decompress1X1_DCtx_wksp, HUF_decompress1X_usingDTable, HUF_decompress4X_usingDTable,
};
use crate::lib::decompress::{
    blockType_e, bt_reserved, bt_rle, HUF_DTable, LL_base, ML_base, OF_base, OF_bits, ZSTD_DCtx,
    ZSTD_DCtx_s, ZSTD_in_dst, ZSTD_not_in_dst, ZSTD_seqSymbol, ZSTD_seqSymbol_header, ZSTD_split,
};
use crate::lib::zstd::*;
use crate::{MEM_readLE16, MEM_readLE24, MEM_readLE32, MEM_readLEST, MEM_write64};
pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __loadu_si128 {
    pub __v: __m128i,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __storeu_si128 {
    pub __v: __m128i,
}
pub type BitContainerType = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BIT_DStream_t {
    pub bitContainer: BitContainerType,
    pub bitsConsumed: std::ffi::c_uint,
    pub ptr: *const std::ffi::c_char,
    pub start: *const std::ffi::c_char,
    pub limitPtr: *const std::ffi::c_char,
}
pub type BIT_DStream_status = std::ffi::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type ZSTD_refMultipleDDicts_e = std::ffi::c_uint;
pub const ZSTD_rmd_refMultipleDDicts: ZSTD_refMultipleDDicts_e = 1;
pub const ZSTD_rmd_refSingleDDict: ZSTD_refMultipleDDicts_e = 0;
pub type XXH64_state_t = XXH64_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}
pub type XXH64_hash_t = u64;
pub type XXH32_hash_t = u32;
pub type streaming_operation = std::ffi::c_uint;
pub const is_streaming: streaming_operation = 1;
pub const not_streaming: streaming_operation = 0;
pub type ZSTD_longOffset_e = std::ffi::c_uint;
pub const ZSTD_lo_isLongOffset: ZSTD_longOffset_e = 1;
pub const ZSTD_lo_isRegularOffset: ZSTD_longOffset_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BIT_DStream_t,
    pub stateLL: ZSTD_fseState,
    pub stateOffb: ZSTD_fseState,
    pub stateML: ZSTD_fseState,
    pub prevOffset: [size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseState {
    pub state: size_t,
    pub table: *const ZSTD_seqSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: size_t,
    pub matchLength: size_t,
    pub offset: size_t,
}
pub type ZSTD_overlap_e = std::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_OffsetInfo {
    pub longOffsetShare: std::ffi::c_uint,
    pub maxNbAdditionalBits: std::ffi::c_uint,
}
pub type SymbolEncodingType_e = std::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_e,
    pub lastBlock: u32,
    pub origSize: u32,
}
pub const CACHELINE_SIZE: std::ffi::c_int = 64;
#[inline]
unsafe extern "C" fn ZSTD_wrappedPtrAdd(
    mut ptr: *const std::ffi::c_void,
    mut add: ptrdiff_t,
) -> *const std::ffi::c_void {
    (ptr as *const std::ffi::c_char).offset(add as isize) as *const std::ffi::c_void
}
#[inline]
unsafe extern "C" fn ZSTD_wrappedPtrSub(
    mut ptr: *const std::ffi::c_void,
    mut sub: ptrdiff_t,
) -> *const std::ffi::c_void {
    (ptr as *const std::ffi::c_char).offset(-(sub as isize)) as *const std::ffi::c_void
}
#[inline]
unsafe extern "C" fn ZSTD_maybeNullPtrAdd(
    mut ptr: *mut std::ffi::c_void,
    mut add: ptrdiff_t,
) -> *mut std::ffi::c_void {
    if add > 0 {
        (ptr as *mut std::ffi::c_char).offset(add as isize) as *mut std::ffi::c_void
    } else {
        ptr
    }
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong == 4) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong == 8) as std::ffi::c_int
        as std::ffi::c_uint
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn _force_has_format_string(mut format: *const std::ffi::c_char, mut args: ...) {}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const STREAM_ACCUMULATOR_MIN_32: std::ffi::c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: std::ffi::c_int = 57;
#[inline]
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 {
        ptr::write_bytes(bitD as *mut u8, 0, ::core::mem::size_of::<BIT_DStream_t>());
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    (*bitD).start = srcBuffer as *const std::ffi::c_char;
    (*bitD).limitPtr = ((*bitD).start)
        .offset(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize);
    if srcSize >= ::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong {
        (*bitD).ptr = (srcBuffer as *const std::ffi::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        let lastByte = *(srcBuffer as *const u8).offset(srcSize.wrapping_sub(1) as isize);
        (*bitD).bitsConsumed = if lastByte as std::ffi::c_int != 0 {
            (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_highbit32(lastByte as u32))
        } else {
            0
        };
        if lastByte as std::ffi::c_int == 0 {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as BitContainerType;
        let mut current_block_32: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                    (*(srcBuffer as *const u8).offset(6) as BitContainerType)
                        << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                            .wrapping_mul(8)
                            .wrapping_sub(16),
                );
                current_block_32 = 3850282667991058540;
            }
            6 => {
                current_block_32 = 3850282667991058540;
            }
            5 => {
                current_block_32 = 4516348661728506741;
            }
            4 => {
                current_block_32 = 17535799132721701948;
            }
            3 => {
                current_block_32 = 8749097731461952757;
            }
            2 => {
                current_block_32 = 2662887031065336898;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        if current_block_32 == 3850282667991058540 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(5) as BitContainerType)
                    << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                        .wrapping_mul(8)
                        .wrapping_sub(24),
            );
            current_block_32 = 4516348661728506741;
        }
        if current_block_32 == 4516348661728506741 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(4) as BitContainerType)
                    << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                        .wrapping_mul(8)
                        .wrapping_sub(32),
            );
            current_block_32 = 17535799132721701948;
        }
        if current_block_32 == 17535799132721701948 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(3) as BitContainerType) << 24);
            current_block_32 = 8749097731461952757;
        }
        if current_block_32 == 8749097731461952757 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(2) as BitContainerType) << 16);
            current_block_32 = 2662887031065336898;
        }
        if current_block_32 == 2662887031065336898 {
            (*bitD).bitContainer = ((*bitD).bitContainer)
                .wrapping_add((*(srcBuffer as *const u8).offset(1) as BitContainerType) << 8);
        }
        let lastByte_0 = *(srcBuffer as *const u8).offset(srcSize.wrapping_sub(1) as isize);
        (*bitD).bitsConsumed = if lastByte_0 as std::ffi::c_int != 0 {
            (8 as std::ffi::c_int as std::ffi::c_uint)
                .wrapping_sub(ZSTD_highbit32(lastByte_0 as u32))
        } else {
            0
        };
        if lastByte_0 as std::ffi::c_int == 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(
            (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong).wrapping_sub(srcSize)
                as u32
                * 8,
        );
    }
    srcSize
}
#[inline(always)]
unsafe extern "C" fn BIT_getMiddleBits(
    mut bitContainer: BitContainerType,
    start: u32,
    nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    bitContainer >> (start & regMask)
        & ((1 as std::ffi::c_int as u64) << nbBits).wrapping_sub(1 as std::ffi::c_int as u64)
}
#[inline(always)]
unsafe extern "C" fn BIT_lookBits(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    BIT_getMiddleBits(
        (*bitD).bitContainer,
        (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8)
            .wrapping_sub((*bitD).bitsConsumed as std::ffi::c_ulong)
            .wrapping_sub(nbBits as std::ffi::c_ulong) as u32,
        nbBits,
    )
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8)
        .wrapping_sub(1) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask.wrapping_add(1).wrapping_sub(nbBits) & regMask)
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline(always)]
unsafe extern "C" fn BIT_readBits(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> BitContainerType {
    let value = BIT_lookBits(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BIT_readBitsFast(
    mut bitD: *mut BIT_DStream_t,
    mut nbBits: std::ffi::c_uint,
) -> size_t {
    let value = BIT_lookBitsFast(bitD, nbBits);
    BIT_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BIT_reloadDStream_internal(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    (*bitD).ptr = ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3) as isize));
    (*bitD).bitsConsumed &= 7;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    BIT_DStream_unfinished
}
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t) -> BIT_DStream_status {
    if ((*bitD).bitsConsumed as std::ffi::c_ulong
        > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong).wrapping_mul(8))
        as std::ffi::c_int as std::ffi::c_long
        != 0
    {
        static zeroFilled: BitContainerType = 0;
        (*bitD).ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char;
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStream_internal(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as std::ffi::c_ulong)
            < (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong).wrapping_mul(8)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as u32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    result
}
#[inline]
unsafe extern "C" fn BIT_endOfDStream(mut DStream: *const BIT_DStream_t) -> std::ffi::c_uint {
    ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as std::ffi::c_ulong
            == (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong).wrapping_mul(8))
        as std::ffi::c_int as std::ffi::c_uint
}
pub const ZSTD_BLOCKSIZELOG_MAX: std::ffi::c_int = 17;
pub const ZSTD_BLOCKSIZE_MAX: std::ffi::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;
pub const ZSTD_WINDOWLOG_MAX_32: std::ffi::c_int = 30;
#[inline]
unsafe extern "C" fn ZSTD_DCtx_get_bmi2(mut dctx: *const ZSTD_DCtx_s) -> std::ffi::c_int {
    (*dctx).bmi2
}
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
pub const ZSTD_REP_NUM: std::ffi::c_int = 3;
pub const ZSTD_BLOCKHEADERSIZE: std::ffi::c_int = 3;
static ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub const LONGNBSEQ: std::ffi::c_int = 0x7f00 as std::ffi::c_int;
pub const MaxML: std::ffi::c_int = 52;
pub const MaxLL: std::ffi::c_int = 35;
pub const MaxOff: std::ffi::c_int = 31;
pub const MLFSELog: std::ffi::c_int = 9;
pub const LLFSELog: std::ffi::c_int = 9;
pub const OffFSELog: std::ffi::c_int = 8;
static LL_bits: [u8; 36] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16,
];
pub const LL_DEFAULTNORMLOG: std::ffi::c_int = 6;
static ML_bits: [u8; 53] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
];
pub const ML_DEFAULTNORMLOG: std::ffi::c_int = 6;
pub const OF_DEFAULTNORMLOG: std::ffi::c_int = 5;
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut std::ffi::c_void, mut src: *const std::ffi::c_void) {
    libc::memcpy(dst, src, 8 as libc::size_t);
}
unsafe extern "C" fn ZSTD_copy16(mut dst: *mut std::ffi::c_void, mut src: *const std::ffi::c_void) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub const WILDCOPY_OVERLENGTH: std::ffi::c_int = 32;
pub const WILDCOPY_VECLEN: std::ffi::c_int = 16;
#[inline(always)]
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut length: size_t,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut u8).offset_from(src as *const u8) as std::ffi::c_long;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    if ovtype as std::ffi::c_uint
        == ZSTD_overlap_src_before_dst as std::ffi::c_int as std::ffi::c_uint
        && diff < WILDCOPY_VECLEN as ptrdiff_t
    {
        loop {
            ZSTD_copy8(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(8);
            ip = ip.offset(8);
            if op >= oend {
                break;
            }
        }
    } else {
        ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
        if 16 >= length {
            return;
        }
        op = op.offset(16);
        ip = ip.offset(16);
        loop {
            ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            if op >= oend {
                break;
            }
        }
    };
}
pub const NULL: std::ffi::c_int = 0;
unsafe extern "C" fn ZSTD_copy4(mut dst: *mut std::ffi::c_void, mut src: *const std::ffi::c_void) {
    libc::memcpy(dst, src, 4 as libc::size_t);
}
unsafe extern "C" fn ZSTD_blockSizeMax(mut dctx: *const ZSTD_DCtx) -> size_t {
    (if (*dctx).isFrameDecompression != 0 {
        (*dctx).fParams.blockSizeMax
    } else {
        ZSTD_BLOCKSIZE_MAX as std::ffi::c_uint
    }) as size_t
}
#[export_name = crate::prefix!(ZSTD_getcBlockSize)]
pub unsafe extern "C" fn ZSTD_getcBlockSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut bpPtr: *mut blockProperties_t,
) -> size_t {
    if srcSize < ZSTD_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let cBlockHeader = MEM_readLE24(src);
    let cSize = cBlockHeader >> 3;
    (*bpPtr).lastBlock = cBlockHeader & 1;
    (*bpPtr).blockType = (cBlockHeader >> 1 & 3) as blockType_e;
    (*bpPtr).origSize = cSize;
    if (*bpPtr).blockType as std::ffi::c_uint == bt_rle as std::ffi::c_int as std::ffi::c_uint {
        return 1;
    }
    if (*bpPtr).blockType as std::ffi::c_uint == bt_reserved as std::ffi::c_int as std::ffi::c_uint
    {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    cSize as size_t
}
unsafe extern "C" fn ZSTD_allocateLiteralsBuffer(
    mut dctx: *mut ZSTD_DCtx,
    dst: *mut std::ffi::c_void,
    dstCapacity: size_t,
    litSize: size_t,
    streaming: streaming_operation,
    expectedWriteSize: size_t,
    splitImmediately: std::ffi::c_uint,
) {
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    if streaming as std::ffi::c_uint == not_streaming as std::ffi::c_int as std::ffi::c_uint
        && dstCapacity
            > blockSizeMax
                .wrapping_add(WILDCOPY_OVERLENGTH as size_t)
                .wrapping_add(litSize)
                .wrapping_add(WILDCOPY_OVERLENGTH as size_t)
    {
        (*dctx).litBuffer = (dst as *mut u8)
            .offset(blockSizeMax as isize)
            .offset(WILDCOPY_OVERLENGTH as isize);
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_in_dst;
    } else if litSize
        <= (if 64
            > (if ((1) << 16) < (128) << 10 {
                (1) << 16
            } else {
                (128) << 10
            }) {
            64
        } else if ((1) << 16) < (128) << 10 {
            (1) << 16
        } else {
            (128) << 10
        }) as size_t
    {
        (*dctx).litBuffer = ((*dctx).litExtraBuffer).as_mut_ptr();
        (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize);
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    } else {
        if splitImmediately != 0 {
            (*dctx).litBuffer = (dst as *mut u8)
                .offset(expectedWriteSize as isize)
                .offset(-(litSize as isize))
                .offset(
                    (if 64
                        > (if ((1) << 16) < (128) << 10 {
                            (1) << 16
                        } else {
                            (128) << 10
                        })
                    {
                        64
                    } else if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    }) as isize,
                )
                .offset(-(WILDCOPY_OVERLENGTH as isize));
            (*dctx).litBufferEnd = ((*dctx).litBuffer).offset(litSize as isize).offset(
                -((if 64
                    > (if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    })
                {
                    64
                } else if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                }) as isize),
            );
        } else {
            (*dctx).litBuffer = (dst as *mut u8)
                .offset(expectedWriteSize as isize)
                .offset(-(litSize as isize));
            (*dctx).litBufferEnd = (dst as *mut u8).offset(expectedWriteSize as isize);
        }
        (*dctx).litBufferLocation = ZSTD_split;
    };
}
unsafe extern "C" fn ZSTD_decodeLiteralsBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    streaming: streaming_operation,
) -> size_t {
    if srcSize < (1 + 1) as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let istart = src as *const u8;
    let litEncType = (*istart.offset(0) as std::ffi::c_int & 3) as SymbolEncodingType_e;
    let blockSizeMax = ZSTD_blockSizeMax(dctx);
    match litEncType as std::ffi::c_uint {
        3 => {
            if (*dctx).litEntropy == 0 {
                return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
            }
        }
        2 => {}
        0 => {
            let mut litSize_0: size_t = 0;
            let mut lhSize_0: size_t = 0;
            let lhlCode_0 = (*istart.offset(0) as std::ffi::c_int >> 2 & 3) as u32;
            let mut expectedWriteSize_0 = if blockSizeMax < dstCapacity {
                blockSizeMax
            } else {
                dstCapacity
            };
            match lhlCode_0 {
                1 => {
                    lhSize_0 = 2;
                    litSize_0 = (MEM_readLE16(istart as *const std::ffi::c_void) as std::ffi::c_int
                        >> 4) as size_t;
                }
                3 => {
                    lhSize_0 = 3;
                    if srcSize < 3 {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                    }
                    litSize_0 = (MEM_readLE24(istart as *const std::ffi::c_void) >> 4) as size_t;
                }
                0 | 2 | _ => {
                    lhSize_0 = 1;
                    litSize_0 = (*istart.offset(0) as std::ffi::c_int >> 3) as size_t;
                }
            }
            if litSize_0 > 0 && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            if litSize_0 > blockSizeMax {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if expectedWriteSize_0 < litSize_0 {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst,
                dstCapacity,
                litSize_0,
                streaming,
                expectedWriteSize_0,
                1,
            );
            if lhSize_0
                .wrapping_add(litSize_0)
                .wrapping_add(WILDCOPY_OVERLENGTH as size_t)
                > srcSize
            {
                if litSize_0.wrapping_add(lhSize_0) > srcSize {
                    return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                }
                if (*dctx).litBufferLocation as std::ffi::c_uint
                    == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
                {
                    libc::memcpy(
                        (*dctx).litBuffer as *mut std::ffi::c_void,
                        istart.offset(lhSize_0 as isize) as *const std::ffi::c_void,
                        litSize_0.wrapping_sub(
                            (if 64
                                > (if ((1) << 16) < (128) << 10 {
                                    (1) << 16
                                } else {
                                    (128) << 10
                                })
                            {
                                64
                            } else if ((1) << 16) < (128) << 10 {
                                (1) << 16
                            } else {
                                (128) << 10
                            }) as size_t,
                        ) as libc::size_t,
                    );
                    libc::memcpy(
                        ((*dctx).litExtraBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                        istart
                            .offset(lhSize_0 as isize)
                            .offset(litSize_0 as isize)
                            .offset(
                                -((if 64
                                    > (if ((1) << 16) < (128) << 10 {
                                        (1) << 16
                                    } else {
                                        (128) << 10
                                    })
                                {
                                    64
                                } else if ((1) << 16) < (128) << 10 {
                                    (1) << 16
                                } else {
                                    (128) << 10
                                }) as isize),
                            ) as *const std::ffi::c_void,
                        (if 64
                            > (if ((1) << 16) < (128) << 10 {
                                (1) << 16
                            } else {
                                (128) << 10
                            })
                        {
                            64
                        } else if ((1) << 16) < (128) << 10 {
                            (1) << 16
                        } else {
                            (128) << 10
                        }) as std::ffi::c_ulong as libc::size_t,
                    );
                } else {
                    libc::memcpy(
                        (*dctx).litBuffer as *mut std::ffi::c_void,
                        istart.offset(lhSize_0 as isize) as *const std::ffi::c_void,
                        litSize_0 as libc::size_t,
                    );
                }
                (*dctx).litPtr = (*dctx).litBuffer;
                (*dctx).litSize = litSize_0;
                return lhSize_0.wrapping_add(litSize_0);
            }
            (*dctx).litPtr = istart.offset(lhSize_0 as isize);
            (*dctx).litSize = litSize_0;
            (*dctx).litBufferEnd = ((*dctx).litPtr).offset(litSize_0 as isize);
            (*dctx).litBufferLocation = ZSTD_not_in_dst;
            return lhSize_0.wrapping_add(litSize_0);
        }
        1 => {
            let lhlCode_1 = (*istart.offset(0) as std::ffi::c_int >> 2 & 3) as u32;
            let mut litSize_1: size_t = 0;
            let mut lhSize_1: size_t = 0;
            let mut expectedWriteSize_1 = if blockSizeMax < dstCapacity {
                blockSizeMax
            } else {
                dstCapacity
            };
            match lhlCode_1 {
                1 => {
                    lhSize_1 = 2;
                    if srcSize < 3 {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                    }
                    litSize_1 = (MEM_readLE16(istart as *const std::ffi::c_void) as std::ffi::c_int
                        >> 4) as size_t;
                }
                3 => {
                    lhSize_1 = 3;
                    if srcSize < 4 {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                    }
                    litSize_1 = (MEM_readLE24(istart as *const std::ffi::c_void) >> 4) as size_t;
                }
                0 | 2 | _ => {
                    lhSize_1 = 1;
                    litSize_1 = (*istart.offset(0) as std::ffi::c_int >> 3) as size_t;
                }
            }
            if litSize_1 > 0 && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            if litSize_1 > blockSizeMax {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if expectedWriteSize_1 < litSize_1 {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst,
                dstCapacity,
                litSize_1,
                streaming,
                expectedWriteSize_1,
                1,
            );
            if (*dctx).litBufferLocation as std::ffi::c_uint
                == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
            {
                ptr::write_bytes(
                    (*dctx).litBuffer as *mut u8,
                    *istart.offset(lhSize_1 as isize),
                    litSize_1.wrapping_sub(
                        (if 64
                            > (if ((1) << 16) < (128) << 10 {
                                (1) << 16
                            } else {
                                (128) << 10
                            })
                        {
                            64
                        } else if ((1) << 16) < (128) << 10 {
                            (1) << 16
                        } else {
                            (128) << 10
                        }) as size_t,
                    ) as libc::size_t,
                );
                ptr::write_bytes(
                    ((*dctx).litExtraBuffer).as_mut_ptr() as *mut u8,
                    *istart.offset(lhSize_1 as isize),
                    (if 64
                        > (if ((1) << 16) < (128) << 10 {
                            (1) << 16
                        } else {
                            (128) << 10
                        })
                    {
                        64
                    } else if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    }) as std::ffi::c_ulong as libc::size_t,
                );
            } else {
                ptr::write_bytes(
                    (*dctx).litBuffer as *mut u8,
                    *istart.offset(lhSize_1 as isize),
                    litSize_1 as libc::size_t,
                );
            }
            (*dctx).litPtr = (*dctx).litBuffer;
            (*dctx).litSize = litSize_1;
            return lhSize_1.wrapping_add(1);
        }
        _ => return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t,
    }
    if srcSize < 5 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let mut lhSize: size_t = 0;
    let mut litSize: size_t = 0;
    let mut litCSize: size_t = 0;
    let mut singleStream = 0;
    let lhlCode = (*istart.offset(0) as std::ffi::c_int >> 2 & 3) as u32;
    let lhc = MEM_readLE32(istart as *const std::ffi::c_void);
    let mut hufSuccess: size_t = 0;
    let mut expectedWriteSize = if blockSizeMax < dstCapacity {
        blockSizeMax
    } else {
        dstCapacity
    };
    let flags =
        0 | (if ZSTD_DCtx_get_bmi2(dctx) != 0 {
            HUF_flags_bmi2 as std::ffi::c_int
        } else {
            0
        }) | (if (*dctx).disableHufAsm != 0 {
            HUF_flags_disableAsm as std::ffi::c_int
        } else {
            0
        });
    match lhlCode {
        2 => {
            lhSize = 4;
            litSize = (lhc >> 4 & 0x3fff as std::ffi::c_int as u32) as size_t;
            litCSize = (lhc >> 18) as size_t;
        }
        3 => {
            lhSize = 5;
            litSize = (lhc >> 4 & 0x3ffff as std::ffi::c_int as u32) as size_t;
            litCSize = ((lhc >> 22) as size_t).wrapping_add((*istart.offset(4) as size_t) << 10);
        }
        0 | 1 | _ => {
            singleStream = (lhlCode == 0) as std::ffi::c_int as u32;
            lhSize = 3;
            litSize = (lhc >> 4 & 0x3ff as std::ffi::c_int as u32) as size_t;
            litCSize = (lhc >> 14 & 0x3ff as std::ffi::c_int as u32) as size_t;
        }
    }
    if litSize > 0 && dst.is_null() {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if litSize > blockSizeMax {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if singleStream == 0 && litSize < 6 {
        return -(ZSTD_error_literals_headerWrong as std::ffi::c_int) as size_t;
    }
    if litCSize.wrapping_add(lhSize) > srcSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if expectedWriteSize < litSize {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    ZSTD_allocateLiteralsBuffer(
        dctx,
        dst,
        dstCapacity,
        litSize,
        streaming,
        expectedWriteSize,
        0,
    );
    if (*dctx).ddictIsCold != 0 && litSize > 768 {
        let _ptr = (*dctx).HUFptr as *const std::ffi::c_char;
        let _size = ::core::mem::size_of::<[HUF_DTable; 4097]>() as std::ffi::c_ulong;
        let mut _pos: size_t = 0;
        _pos = 0;
        while _pos < _size {
            _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
        }
    }
    if litEncType as std::ffi::c_uint == set_repeat as std::ffi::c_int as std::ffi::c_uint {
        if singleStream != 0 {
            hufSuccess = HUF_decompress1X_usingDTable(
                (*dctx).litBuffer as *mut std::ffi::c_void,
                litSize,
                istart.offset(lhSize as isize) as *const std::ffi::c_void,
                litCSize,
                (*dctx).HUFptr,
                flags,
            );
        } else {
            hufSuccess = HUF_decompress4X_usingDTable(
                (*dctx).litBuffer as *mut std::ffi::c_void,
                litSize,
                istart.offset(lhSize as isize) as *const std::ffi::c_void,
                litCSize,
                (*dctx).HUFptr,
                flags,
            );
        }
    } else if singleStream != 0 {
        hufSuccess = HUF_decompress1X1_DCtx_wksp(
            ((*dctx).entropy.hufTable).as_mut_ptr(),
            (*dctx).litBuffer as *mut std::ffi::c_void,
            litSize,
            istart.offset(lhSize as isize) as *const std::ffi::c_void,
            litCSize,
            ((*dctx).workspace).as_mut_ptr() as *mut std::ffi::c_void,
            ::core::mem::size_of::<[u32; 640]>() as std::ffi::c_ulong,
            flags,
        );
    } else {
        hufSuccess = HUF_decompress4X_hufOnly_wksp(
            ((*dctx).entropy.hufTable).as_mut_ptr(),
            (*dctx).litBuffer as *mut std::ffi::c_void,
            litSize,
            istart.offset(lhSize as isize) as *const std::ffi::c_void,
            litCSize,
            ((*dctx).workspace).as_mut_ptr() as *mut std::ffi::c_void,
            ::core::mem::size_of::<[u32; 640]>() as std::ffi::c_ulong,
            flags,
        );
    }
    if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
    {
        libc::memcpy(
            ((*dctx).litExtraBuffer).as_mut_ptr() as *mut std::ffi::c_void,
            ((*dctx).litBufferEnd).offset(
                -((if 64
                    > (if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    })
                {
                    64
                } else if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                }) as isize),
            ) as *const std::ffi::c_void,
            (if 64
                > (if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                })
            {
                64
            } else if ((1) << 16) < (128) << 10 {
                (1) << 16
            } else {
                (128) << 10
            }) as std::ffi::c_ulong as libc::size_t,
        );
        libc::memmove(
            ((*dctx).litBuffer)
                .offset(
                    (if 64
                        > (if ((1) << 16) < (128) << 10 {
                            (1) << 16
                        } else {
                            (128) << 10
                        })
                    {
                        64
                    } else if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    }) as isize,
                )
                .offset(-(32)) as *mut std::ffi::c_void,
            (*dctx).litBuffer as *const std::ffi::c_void,
            litSize.wrapping_sub(
                (if 64
                    > (if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    })
                {
                    64
                } else if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                }) as size_t,
            ) as libc::size_t,
        );
        (*dctx).litBuffer = ((*dctx).litBuffer).offset(
            ((if 64
                > (if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                })
            {
                64
            } else if ((1) << 16) < (128) << 10 {
                (1) << 16
            } else {
                (128) << 10
            }) - WILDCOPY_OVERLENGTH) as isize,
        );
        (*dctx).litBufferEnd = ((*dctx).litBufferEnd).offset(-(WILDCOPY_OVERLENGTH as isize));
    }
    if ERR_isError(hufSuccess) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    (*dctx).litPtr = (*dctx).litBuffer;
    (*dctx).litSize = litSize;
    (*dctx).litEntropy = 1;
    if litEncType as std::ffi::c_uint == set_compressed as std::ffi::c_int as std::ffi::c_uint {
        (*dctx).HUFptr = ((*dctx).entropy.hufTable).as_mut_ptr();
    }
    litCSize.wrapping_add(lhSize)
}
#[export_name = crate::prefix!(ZSTD_decodeLiteralsBlock_wrapper)]
pub unsafe extern "C" fn ZSTD_decodeLiteralsBlock_wrapper(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    (*dctx).isFrameDecompression = 0;
    ZSTD_decodeLiteralsBlock(dctx, src, srcSize, dst, dstCapacity, not_streaming)
}
static LL_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        ZSTD_seqSymbol {
            nextState: 1,
            nbAdditionalBits: 1,
            nbBits: 1,
            baseValue: LL_DEFAULTNORMLOG as u32,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 1,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 3,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 4,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 12,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 14,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 16,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 20,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 22,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 28,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 32,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 5,
            baseValue: 48,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 6,
            nbBits: 5,
            baseValue: 64,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 7,
            nbBits: 5,
            baseValue: 128,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 8,
            nbBits: 6,
            baseValue: 256,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 10,
            nbBits: 6,
            baseValue: 1024,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 12,
            nbBits: 6,
            baseValue: 4096,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 1,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 2,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 4,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 5,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 13,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 16,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 18,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 22,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 24,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 32,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 40,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 64,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 64,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 7,
            nbBits: 5,
            baseValue: 128,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 9,
            nbBits: 6,
            baseValue: 512,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 11,
            nbBits: 6,
            baseValue: 2048,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 48,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 0,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 1,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 2,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 3,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 5,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 12,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 15,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 18,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 20,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 24,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 28,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 40,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 4,
            nbBits: 5,
            baseValue: 48,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 16,
            nbBits: 6,
            baseValue: 65536,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 15,
            nbBits: 6,
            baseValue: 32768,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 14,
            nbBits: 6,
            baseValue: 16384,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 13,
            nbBits: 6,
            baseValue: 8192,
        }
    },
];
static OF_defaultDTable: [ZSTD_seqSymbol; 33] = [
    {
        ZSTD_seqSymbol {
            nextState: 1,
            nbAdditionalBits: 1,
            nbBits: 1,
            baseValue: OF_DEFAULTNORMLOG as u32,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 0,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 61,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 9,
            nbBits: 5,
            baseValue: 509,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 15,
            nbBits: 5,
            baseValue: 32765,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 21,
            nbBits: 5,
            baseValue: 2097149,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 5,
            baseValue: 5,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 7,
            nbBits: 4,
            baseValue: 125,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 12,
            nbBits: 5,
            baseValue: 4093,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 18,
            nbBits: 5,
            baseValue: 262141,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 23,
            nbBits: 5,
            baseValue: 8388605,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 5,
            nbBits: 5,
            baseValue: 29,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 8,
            nbBits: 4,
            baseValue: 253,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 14,
            nbBits: 5,
            baseValue: 16381,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 20,
            nbBits: 5,
            baseValue: 1048573,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 5,
            baseValue: 1,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 7,
            nbBits: 4,
            baseValue: 125,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 11,
            nbBits: 5,
            baseValue: 2045,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 17,
            nbBits: 5,
            baseValue: 131069,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 22,
            nbBits: 5,
            baseValue: 4194301,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 5,
            baseValue: 13,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 8,
            nbBits: 4,
            baseValue: 253,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 13,
            nbBits: 5,
            baseValue: 8189,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 19,
            nbBits: 5,
            baseValue: 524285,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 5,
            baseValue: 1,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 6,
            nbBits: 4,
            baseValue: 61,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 10,
            nbBits: 5,
            baseValue: 1021,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 16,
            nbBits: 5,
            baseValue: 65533,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 28,
            nbBits: 5,
            baseValue: 268435453,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 27,
            nbBits: 5,
            baseValue: 134217725,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 26,
            nbBits: 5,
            baseValue: 67108861,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 25,
            nbBits: 5,
            baseValue: 33554429,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 24,
            nbBits: 5,
            baseValue: 16777213,
        }
    },
];
static ML_defaultDTable: [ZSTD_seqSymbol; 65] = [
    {
        ZSTD_seqSymbol {
            nextState: 1,
            nbAdditionalBits: 1,
            nbBits: 1,
            baseValue: ML_DEFAULTNORMLOG as u32,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 3,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 5,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 13,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 16,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 19,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 22,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 25,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 28,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 31,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 34,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 37,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 41,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 6,
            baseValue: 47,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 6,
            baseValue: 59,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 6,
            baseValue: 83,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 7,
            nbBits: 6,
            baseValue: 131,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 9,
            nbBits: 6,
            baseValue: 515,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 5,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 6,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 9,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 12,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 15,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 18,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 21,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 24,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 27,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 30,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 33,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 35,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 1,
            nbBits: 6,
            baseValue: 39,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 2,
            nbBits: 6,
            baseValue: 43,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 3,
            nbBits: 6,
            baseValue: 51,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 4,
            nbBits: 6,
            baseValue: 67,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 5,
            nbBits: 6,
            baseValue: 99,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 8,
            nbBits: 6,
            baseValue: 259,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 48,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 4,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 16,
            nbAdditionalBits: 0,
            nbBits: 4,
            baseValue: 5,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 7,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 8,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 10,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 32,
            nbAdditionalBits: 0,
            nbBits: 5,
            baseValue: 11,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 14,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 17,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 20,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 23,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 26,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 29,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 0,
            nbBits: 6,
            baseValue: 32,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 16,
            nbBits: 6,
            baseValue: 65539,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 15,
            nbBits: 6,
            baseValue: 32771,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 14,
            nbBits: 6,
            baseValue: 16387,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 13,
            nbBits: 6,
            baseValue: 8195,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 12,
            nbBits: 6,
            baseValue: 4099,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 11,
            nbBits: 6,
            baseValue: 2051,
        }
    },
    {
        ZSTD_seqSymbol {
            nextState: 0,
            nbAdditionalBits: 10,
            nbBits: 6,
            baseValue: 1027,
        }
    },
];
unsafe extern "C" fn ZSTD_buildSeqTable_rle(
    mut dt: *mut ZSTD_seqSymbol,
    mut baseValue: u32,
    mut nbAddBits: u8,
) {
    let mut ptr = dt as *mut std::ffi::c_void;
    let DTableH = ptr as *mut ZSTD_seqSymbol_header;
    let cell = dt.offset(1);
    (*DTableH).tableLog = 0;
    (*DTableH).fastMode = 0;
    (*cell).nbBits = 0;
    (*cell).nextState = 0;
    (*cell).nbAdditionalBits = nbAddBits;
    (*cell).baseValue = baseValue;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_buildFSETable_body(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) {
    let tableDecode = dt.offset(1);
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1) << tableLog) as u32;
    let mut symbolNext = wksp as *mut u16;
    let mut spread = symbolNext
        .offset((if 35 > 52 { 35 } else { 52 }) as isize)
        .offset(1) as *mut u8;
    let mut highThreshold = tableSize.wrapping_sub(1);
    let mut DTableH = ZSTD_seqSymbol_header {
        fastMode: 0,
        tableLog: 0,
    };
    DTableH.tableLog = tableLog;
    DTableH.fastMode = 1;
    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;
    let mut s: u32 = 0;
    s = 0;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as std::ffi::c_int == -(1) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh0 as isize)).baseValue = s;
            *symbolNext.offset(s as isize) = 1;
        } else {
            if *normalizedCounter.offset(s as isize) as std::ffi::c_int
                >= largeLimit as std::ffi::c_int
            {
                DTableH.fastMode = 0;
            }
            *symbolNext.offset(s as isize) = *normalizedCounter.offset(s as isize) as u16;
        }
        s = s.wrapping_add(1);
    }
    libc::memcpy(
        dt as *mut std::ffi::c_void,
        &mut DTableH as *mut ZSTD_seqSymbol_header as *const std::ffi::c_void,
        ::core::mem::size_of::<ZSTD_seqSymbol_header>() as std::ffi::c_ulong as libc::size_t,
    );
    if highThreshold == tableSize.wrapping_sub(1) {
        let tableMask = tableSize.wrapping_sub(1) as size_t;
        let step = (tableSize >> 1)
            .wrapping_add(tableSize >> 3)
            .wrapping_add(3) as size_t;
        let add = 0x101010101010101 as std::ffi::c_ulonglong as u64;
        let mut pos = 0 as std::ffi::c_int as size_t;
        let mut sv = 0;
        let mut s_0: u32 = 0;
        s_0 = 0;
        while s_0 < maxSV1 {
            let mut i: std::ffi::c_int = 0;
            let n = *normalizedCounter.offset(s_0 as isize) as std::ffi::c_int;
            MEM_write64(spread.offset(pos as isize) as *mut std::ffi::c_void, sv);
            i = 8;
            while i < n {
                MEM_write64(
                    spread.offset(pos as isize).offset(i as isize) as *mut std::ffi::c_void,
                    sv,
                );
                i += 8;
            }
            pos = pos.wrapping_add(n as size_t);
            s_0 = s_0.wrapping_add(1);
            sv = sv.wrapping_add(add);
        }
        let mut position = 0 as std::ffi::c_int as size_t;
        let mut s_1: size_t = 0;
        let unroll = 2;
        s_1 = 0;
        while s_1 < tableSize as size_t {
            let mut u: size_t = 0;
            u = 0;
            while u < unroll {
                let uPosition = position.wrapping_add(u * step) & tableMask;
                (*tableDecode.offset(uPosition as isize)).baseValue =
                    *spread.offset(s_1.wrapping_add(u) as isize) as u32;
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll * step) & tableMask;
            s_1 = s_1.wrapping_add(unroll);
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1);
        let step_0 = (tableSize >> 1)
            .wrapping_add(tableSize >> 3)
            .wrapping_add(3);
        let mut s_2: u32 = 0;
        let mut position_0 = 0 as std::ffi::c_int as u32;
        s_2 = 0;
        while s_2 < maxSV1 {
            let mut i_0: std::ffi::c_int = 0;
            let n_0 = *normalizedCounter.offset(s_2 as isize) as std::ffi::c_int;
            i_0 = 0;
            while i_0 < n_0 {
                (*tableDecode.offset(position_0 as isize)).baseValue = s_2;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while (position_0 > highThreshold) as std::ffi::c_int as std::ffi::c_long != 0 {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
                i_0 += 1;
            }
            s_2 = s_2.wrapping_add(1);
        }
    }
    let mut u_0: u32 = 0;
    u_0 = 0;
    while u_0 < tableSize {
        let symbol = (*tableDecode.offset(u_0 as isize)).baseValue;
        let fresh1 = &mut (*symbolNext.offset(symbol as isize));
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as u32;
        (*tableDecode.offset(u_0 as isize)).nbBits =
            tableLog.wrapping_sub(ZSTD_highbit32(nextState)) as u8;
        (*tableDecode.offset(u_0 as isize)).nextState = (nextState
            << (*tableDecode.offset(u_0 as isize)).nbBits as std::ffi::c_int)
            .wrapping_sub(tableSize) as u16;
        (*tableDecode.offset(u_0 as isize)).nbAdditionalBits =
            *nbAdditionalBits.offset(symbol as isize);
        (*tableDecode.offset(u_0 as isize)).baseValue = *baseValue.offset(symbol as isize);
        u_0 = u_0.wrapping_add(1);
    }
}
unsafe extern "C" fn ZSTD_buildFSETable_body_default(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) {
    ZSTD_buildFSETable_body(
        dt,
        normalizedCounter,
        maxSymbolValue,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
        wkspSize,
    );
}
unsafe extern "C" fn ZSTD_buildFSETable_body_bmi2(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: size_t,
) {
    ZSTD_buildFSETable_body(
        dt,
        normalizedCounter,
        maxSymbolValue,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
        wkspSize,
    );
}
#[export_name = crate::prefix!(ZSTD_buildFSETable)]
pub unsafe extern "C" fn ZSTD_buildFSETable(
    mut dt: *mut ZSTD_seqSymbol,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut tableLog: std::ffi::c_uint,
    mut wksp: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut bmi2: std::ffi::c_int,
) {
    if bmi2 != 0 {
        ZSTD_buildFSETable_body_bmi2(
            dt,
            normalizedCounter,
            maxSymbolValue,
            baseValue,
            nbAdditionalBits,
            tableLog,
            wksp,
            wkspSize,
        );
        return;
    }
    ZSTD_buildFSETable_body_default(
        dt,
        normalizedCounter,
        maxSymbolValue,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
        wkspSize,
    );
}
unsafe extern "C" fn ZSTD_buildSeqTable(
    mut DTableSpace: *mut ZSTD_seqSymbol,
    mut DTablePtr: *mut *const ZSTD_seqSymbol,
    mut type_0: SymbolEncodingType_e,
    mut max: std::ffi::c_uint,
    mut maxLog: u32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut baseValue: *const u32,
    mut nbAdditionalBits: *const u8,
    mut defaultTable: *const ZSTD_seqSymbol,
    mut flagRepeatTable: u32,
    mut ddictIsCold: std::ffi::c_int,
    mut nbSeq: std::ffi::c_int,
    mut wksp: *mut u32,
    mut wkspSize: size_t,
    mut bmi2: std::ffi::c_int,
) -> size_t {
    match type_0 as std::ffi::c_uint {
        1 => {
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            if *(src as *const u8) as std::ffi::c_uint > max {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            let symbol = *(src as *const u8) as u32;
            let baseline = *baseValue.offset(symbol as isize);
            let nbBits = *nbAdditionalBits.offset(symbol as isize);
            ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);
            *DTablePtr = DTableSpace;
            1
        }
        0 => {
            *DTablePtr = defaultTable;
            0
        }
        3 => {
            if flagRepeatTable == 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if ddictIsCold != 0 && nbSeq > 24 {
                let pStart = *DTablePtr as *const std::ffi::c_void;
                let pSize = (::core::mem::size_of::<ZSTD_seqSymbol>() as std::ffi::c_ulong)
                    .wrapping_mul((1 + ((1) << maxLog)) as std::ffi::c_ulong);
                let _ptr = pStart as *const std::ffi::c_char;
                let _size = pSize;
                let mut _pos: size_t = 0;
                _pos = 0;
                while _pos < _size {
                    _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
                }
            }
            0
        }
        2 => {
            let mut tableLog: std::ffi::c_uint = 0;
            let mut norm: [i16; 53] = [0; 53];
            let headerSize = FSE_readNCount(&mut norm, &mut max, &mut tableLog, src, srcSize);
            if ERR_isError(headerSize) != 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            ZSTD_buildFSETable(
                DTableSpace,
                norm.as_mut_ptr(),
                max,
                baseValue,
                nbAdditionalBits,
                tableLog,
                wksp as *mut std::ffi::c_void,
                wkspSize,
                bmi2,
            );
            *DTablePtr = DTableSpace;
            headerSize
        }
        _ => -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
    }
}
#[export_name = crate::prefix!(ZSTD_decodeSeqHeaders)]
pub unsafe extern "C" fn ZSTD_decodeSeqHeaders(
    mut dctx: *mut ZSTD_DCtx,
    mut nbSeqPtr: *mut std::ffi::c_int,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const u8;
    let iend = istart.offset(srcSize as isize);
    let mut ip = istart;
    let mut nbSeq: std::ffi::c_int = 0;
    if srcSize < 1 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let fresh3 = ip;
    ip = ip.offset(1);
    nbSeq = *fresh3 as std::ffi::c_int;
    if nbSeq > 0x7f as std::ffi::c_int {
        if nbSeq == 0xff as std::ffi::c_int {
            if ip.offset(2) > iend {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            nbSeq = MEM_readLE16(ip as *const std::ffi::c_void) as std::ffi::c_int + LONGNBSEQ;
            ip = ip.offset(2);
        } else {
            if ip >= iend {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            let fresh4 = ip;
            ip = ip.offset(1);
            nbSeq = ((nbSeq - 0x80 as std::ffi::c_int) << 8) + *fresh4 as std::ffi::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if nbSeq == 0 {
        if ip != iend {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        return ip.offset_from(istart) as std::ffi::c_long as size_t;
    }
    if ip.offset(1) > iend {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if *ip as std::ffi::c_int & 3 != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let LLtype = (*ip as std::ffi::c_int >> 6) as SymbolEncodingType_e;
    let OFtype = (*ip as std::ffi::c_int >> 4 & 3) as SymbolEncodingType_e;
    let MLtype = (*ip as std::ffi::c_int >> 2 & 3) as SymbolEncodingType_e;
    ip = ip.offset(1);
    let llhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.LLTable).as_mut_ptr(),
        &mut (*dctx).LLTptr,
        LLtype,
        MaxLL as std::ffi::c_uint,
        LLFSELog as u32,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
        LL_base.as_ptr(),
        LL_bits.as_ptr(),
        LL_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; 640]>() as std::ffi::c_ulong,
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(llhSize) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(llhSize as isize);
    let ofhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.OFTable).as_mut_ptr(),
        &mut (*dctx).OFTptr,
        OFtype,
        MaxOff as std::ffi::c_uint,
        OffFSELog as u32,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
        OF_base.as_ptr(),
        OF_bits.as_ptr(),
        OF_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; 640]>() as std::ffi::c_ulong,
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(ofhSize) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(ofhSize as isize);
    let mlhSize = ZSTD_buildSeqTable(
        ((*dctx).entropy.MLTable).as_mut_ptr(),
        &mut (*dctx).MLTptr,
        MLtype,
        MaxML as std::ffi::c_uint,
        MLFSELog as u32,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
        ML_base.as_ptr(),
        ML_bits.as_ptr(),
        ML_defaultDTable.as_ptr(),
        (*dctx).fseEntropy,
        (*dctx).ddictIsCold,
        nbSeq,
        ((*dctx).workspace).as_mut_ptr(),
        ::core::mem::size_of::<[u32; 640]>() as std::ffi::c_ulong,
        ZSTD_DCtx_get_bmi2(dctx),
    );
    if ERR_isError(mlhSize) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(mlhSize as isize);
    ip.offset_from(istart) as std::ffi::c_long as size_t
}
#[inline(always)]
unsafe extern "C" fn ZSTD_overlapCopy8(
    mut op: *mut *mut u8,
    mut ip: *mut *const u8,
    mut offset: size_t,
) {
    if offset < 8 {
        static dec32table: [u32; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
        static dec64table: [std::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
        let sub2 = *dec64table.as_ptr().offset(offset as isize);
        *(*op).offset(0) = *(*ip).offset(0);
        *(*op).offset(1) = *(*ip).offset(1);
        *(*op).offset(2) = *(*ip).offset(2);
        *(*op).offset(3) = *(*ip).offset(3);
        *ip = (*ip).offset(*dec32table.as_ptr().offset(offset as isize) as isize);
        ZSTD_copy4(
            (*op).offset(4) as *mut std::ffi::c_void,
            *ip as *const std::ffi::c_void,
        );
        *ip = (*ip).offset(-(sub2 as isize));
    } else {
        ZSTD_copy8(*op as *mut std::ffi::c_void, *ip as *const std::ffi::c_void);
    }
    *ip = (*ip).offset(8);
    *op = (*op).offset(8);
}
unsafe extern "C" fn ZSTD_safecopy(
    mut op: *mut u8,
    oend_w: *const u8,
    mut ip: *const u8,
    mut length: size_t,
    mut ovtype: ZSTD_overlap_e,
) {
    let diff = op.offset_from(ip) as std::ffi::c_long;
    let oend = op.offset(length as isize);
    if length < 8 {
        while op < oend {
            let fresh5 = ip;
            ip = ip.offset(1);
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = *fresh5;
        }
        return;
    }
    if ovtype as std::ffi::c_uint
        == ZSTD_overlap_src_before_dst as std::ffi::c_int as std::ffi::c_uint
    {
        ZSTD_overlapCopy8(&mut op, &mut ip, diff as size_t);
        length = length.wrapping_sub(8);
    }
    if oend <= oend_w as *mut u8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            length,
            ovtype,
        );
        return;
    }
    if op <= oend_w as *mut u8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            oend_w.offset_from(op) as std::ffi::c_long as size_t,
            ovtype,
        );
        ip = ip.offset(oend_w.offset_from(op) as std::ffi::c_long as isize);
        op = op.offset(oend_w.offset_from(op) as std::ffi::c_long as isize);
    }
    while op < oend {
        let fresh7 = ip;
        ip = ip.offset(1);
        let fresh8 = op;
        op = op.offset(1);
        *fresh8 = *fresh7;
    }
}
unsafe extern "C" fn ZSTD_safecopyDstBeforeSrc(
    mut op: *mut u8,
    mut ip: *const u8,
    mut length: size_t,
) {
    let diff = op.offset_from(ip) as std::ffi::c_long;
    let oend = op.offset(length as isize);
    if length < 8 || diff > -(8) as ptrdiff_t {
        while op < oend {
            let fresh9 = ip;
            ip = ip.offset(1);
            let fresh10 = op;
            op = op.offset(1);
            *fresh10 = *fresh9;
        }
        return;
    }
    if op <= oend.offset(-(WILDCOPY_OVERLENGTH as isize)) && diff < -WILDCOPY_VECLEN as ptrdiff_t {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op) as std::ffi::c_long
                as size_t,
            ZSTD_no_overlap,
        );
        ip = ip.offset(oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op)
            as std::ffi::c_long as isize);
        op = op.offset(oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op)
            as std::ffi::c_long as isize);
    }
    while op < oend {
        let fresh11 = ip;
        ip = ip.offset(1);
        let fresh12 = op;
        op = op.offset(1);
        *fresh12 = *fresh11;
    }
}
#[inline(never)]
unsafe extern "C" fn ZSTD_execSequenceEnd(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    if sequenceLength > oend.offset_from(op) as std::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as std::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ZSTD_safecopy(op, oend_w, *litPtr, sequence.litLength, ZSTD_no_overlap);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as std::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as std::ffi::c_long as size_t {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(-(prefixStart.offset_from(match_0) as std::ffi::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as std::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = prefixStart;
    }
    ZSTD_safecopy(
        op,
        oend_w,
        match_0,
        sequence.matchLength,
        ZSTD_overlap_src_before_dst,
    );
    sequenceLength
}
#[inline(never)]
unsafe extern "C" fn ZSTD_execSequenceEndSplitLitBuffer(
    mut op: *mut u8,
    oend: *mut u8,
    oend_w: *const u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    if sequenceLength > oend.offset_from(op) as std::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as std::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op > *litPtr as *mut u8 && op < (*litPtr).offset(sequence.litLength as isize) as *mut u8 {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    ZSTD_safecopyDstBeforeSrc(op, *litPtr, sequence.litLength);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as std::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as std::ffi::c_long as size_t {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(-(prefixStart.offset_from(match_0) as std::ffi::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as std::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = prefixStart;
    }
    ZSTD_safecopy(
        op,
        oend_w,
        match_0,
        sequence.matchLength,
        ZSTD_overlap_src_before_dst,
    );
    sequenceLength
}
#[inline(always)]
unsafe extern "C" fn ZSTD_execSequence(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_w = oend.offset(-(WILDCOPY_OVERLENGTH as isize));
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.wrapping_offset(-(sequence.offset as isize));
    if (iLitEnd > litLimit
        || oMatchEnd > oend_w
        || MEM_32bits() != 0
            && (oend.offset_from(op) as std::ffi::c_long as size_t)
                < sequenceLength.wrapping_add(32)) as std::ffi::c_int as std::ffi::c_long
        != 0
    {
        return ZSTD_execSequenceEnd(
            op,
            oend,
            sequence,
            litPtr,
            litLimit,
            prefixStart,
            virtualStart,
            dictEnd,
        );
    }
    ZSTD_copy16(
        op as *mut std::ffi::c_void,
        *litPtr as *const std::ffi::c_void,
    );
    if (sequence.litLength > 16) as std::ffi::c_int as std::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op.offset(16) as *mut std::ffi::c_void,
            (*litPtr).offset(16) as *const std::ffi::c_void,
            (sequence.litLength).wrapping_sub(16),
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as std::ffi::c_long as size_t {
        if (sequence.offset > oLitEnd.offset_from(virtualStart) as std::ffi::c_long as size_t)
            as std::ffi::c_int as std::ffi::c_long
            != 0
        {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(match_0.offset_from(prefixStart) as std::ffi::c_long as isize);
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as std::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = prefixStart;
    }
    if (sequence.offset >= 16) as std::ffi::c_int as std::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            sequence.matchLength,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            (sequence.matchLength).wrapping_sub(8),
            ZSTD_overlap_src_before_dst,
        );
    }
    sequenceLength
}
#[inline(always)]
unsafe extern "C" fn ZSTD_execSequenceSplitLitBuffer(
    mut op: *mut u8,
    oend: *mut u8,
    oend_w: *const u8,
    mut sequence: seq_t,
    mut litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    if (iLitEnd > litLimit
        || oMatchEnd > oend_w as *mut u8
        || MEM_32bits() != 0
            && (oend.offset_from(op) as std::ffi::c_long as size_t)
                < sequenceLength.wrapping_add(32)) as std::ffi::c_int as std::ffi::c_long
        != 0
    {
        return ZSTD_execSequenceEndSplitLitBuffer(
            op,
            oend,
            oend_w,
            sequence,
            litPtr,
            litLimit,
            prefixStart,
            virtualStart,
            dictEnd,
        );
    }
    ZSTD_copy16(
        op as *mut std::ffi::c_void,
        *litPtr as *const std::ffi::c_void,
    );
    if (sequence.litLength > 16) as std::ffi::c_int as std::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op.offset(16) as *mut std::ffi::c_void,
            (*litPtr).offset(16) as *const std::ffi::c_void,
            (sequence.litLength).wrapping_sub(16),
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as std::ffi::c_long as size_t {
        if (sequence.offset > oLitEnd.offset_from(virtualStart) as std::ffi::c_long as size_t)
            as std::ffi::c_int as std::ffi::c_long
            != 0
        {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(match_0.offset_from(prefixStart) as std::ffi::c_long as isize);
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as std::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = prefixStart;
    }
    if (sequence.offset >= 16) as std::ffi::c_int as std::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            sequence.matchLength,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            (sequence.matchLength).wrapping_sub(8),
            ZSTD_overlap_src_before_dst,
        );
    }
    sequenceLength
}
unsafe extern "C" fn ZSTD_initFseState(
    mut DStatePtr: *mut ZSTD_fseState,
    mut bitD: *mut BIT_DStream_t,
    mut dt: *const ZSTD_seqSymbol,
) {
    let mut ptr = dt as *const std::ffi::c_void;
    let DTableH = ptr as *const ZSTD_seqSymbol_header;
    (*DStatePtr).state = BIT_readBits(bitD, (*DTableH).tableLog);
    BIT_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_updateFseStateWithDInfo(
    mut DStatePtr: *mut ZSTD_fseState,
    mut bitD: *mut BIT_DStream_t,
    mut nextState: u16,
    mut nbBits: u32,
) {
    let lowBits = BIT_readBits(bitD, nbBits);
    (*DStatePtr).state = (nextState as size_t).wrapping_add(lowBits);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decodeSequence(
    mut seqState: *mut seqState_t,
    longOffsets: ZSTD_longOffset_e,
    isLastSeq: std::ffi::c_int,
) -> seq_t {
    let mut seq = seq_t {
        litLength: 0,
        matchLength: 0,
        offset: 0,
    };
    let llDInfo = ((*seqState).stateLL.table).offset((*seqState).stateLL.state as isize);
    let mlDInfo = ((*seqState).stateML.table).offset((*seqState).stateML.state as isize);
    let ofDInfo = ((*seqState).stateOffb.table).offset((*seqState).stateOffb.state as isize);
    seq.matchLength = (*mlDInfo).baseValue as size_t;
    seq.litLength = (*llDInfo).baseValue as size_t;
    let ofBase = (*ofDInfo).baseValue;
    let llBits = (*llDInfo).nbAdditionalBits;
    let mlBits = (*mlDInfo).nbAdditionalBits;
    let ofBits = (*ofDInfo).nbAdditionalBits;
    let totalBits =
        (llBits as std::ffi::c_int + mlBits as std::ffi::c_int + ofBits as std::ffi::c_int) as u8;
    let llNext = (*llDInfo).nextState;
    let mlNext = (*mlDInfo).nextState;
    let ofNext = (*ofDInfo).nextState;
    let llnbBits = (*llDInfo).nbBits as u32;
    let mlnbBits = (*mlDInfo).nbBits as u32;
    let ofnbBits = (*ofDInfo).nbBits as u32;
    let mut offset: size_t = 0;
    if ofBits as std::ffi::c_int > 1 {
        if MEM_32bits() != 0
            && longOffsets as std::ffi::c_uint != 0
            && ofBits as std::ffi::c_int >= STREAM_ACCUMULATOR_MIN_32
        {
            let extraBits = (if ZSTD_WINDOWLOG_MAX_32 > STREAM_ACCUMULATOR_MIN_32 {
                ZSTD_WINDOWLOG_MAX_32 - STREAM_ACCUMULATOR_MIN_32
            } else {
                0
            }) as u32;
            offset = (ofBase as size_t).wrapping_add(
                BIT_readBitsFast(
                    &mut (*seqState).DStream,
                    (ofBits as u32).wrapping_sub(extraBits),
                ) << extraBits,
            );
            BIT_reloadDStream(&mut (*seqState).DStream);
            offset = offset.wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream, extraBits));
        } else {
            offset = (ofBase as size_t).wrapping_add(BIT_readBitsFast(
                &mut (*seqState).DStream,
                ofBits as std::ffi::c_uint,
            ));
            if MEM_32bits() != 0 {
                BIT_reloadDStream(&mut (*seqState).DStream);
            }
        }
        *((*seqState).prevOffset).as_mut_ptr().offset(2) =
            *((*seqState).prevOffset).as_mut_ptr().offset(1);
        *((*seqState).prevOffset).as_mut_ptr().offset(1) =
            *((*seqState).prevOffset).as_mut_ptr().offset(0);
        *((*seqState).prevOffset).as_mut_ptr().offset(0) = offset;
    } else {
        let ll0 = ((*llDInfo).baseValue == 0) as std::ffi::c_int as u32;
        if (ofBits as std::ffi::c_int == 0) as std::ffi::c_int as std::ffi::c_long != 0 {
            offset = *((*seqState).prevOffset).as_mut_ptr().offset(ll0 as isize);
            *((*seqState).prevOffset).as_mut_ptr().offset(1) = *((*seqState).prevOffset)
                .as_mut_ptr()
                .offset((ll0 == 0) as std::ffi::c_int as isize);
            *((*seqState).prevOffset).as_mut_ptr().offset(0) = offset;
        } else {
            offset = (ofBase.wrapping_add(ll0) as size_t)
                .wrapping_add(BIT_readBitsFast(&mut (*seqState).DStream, 1));
            let mut temp = if offset == 3 {
                (*((*seqState).prevOffset).as_mut_ptr().offset(0)).wrapping_sub(1)
            } else {
                *((*seqState).prevOffset)
                    .as_mut_ptr()
                    .offset(offset as isize)
            };
            temp = temp.wrapping_sub((temp == 0) as std::ffi::c_int as size_t);
            if offset != 1 {
                *((*seqState).prevOffset).as_mut_ptr().offset(2) =
                    *((*seqState).prevOffset).as_mut_ptr().offset(1);
            }
            *((*seqState).prevOffset).as_mut_ptr().offset(1) =
                *((*seqState).prevOffset).as_mut_ptr().offset(0);
            offset = temp;
            *((*seqState).prevOffset).as_mut_ptr().offset(0) = offset;
        }
    }
    seq.offset = offset;
    if mlBits as std::ffi::c_int > 0 {
        seq.matchLength = (seq.matchLength).wrapping_add(BIT_readBitsFast(
            &mut (*seqState).DStream,
            mlBits as std::ffi::c_uint,
        ));
    }
    if MEM_32bits() != 0
        && mlBits as std::ffi::c_int + llBits as std::ffi::c_int
            >= STREAM_ACCUMULATOR_MIN_32
                - (if ZSTD_WINDOWLOG_MAX_32 > STREAM_ACCUMULATOR_MIN_32 {
                    ZSTD_WINDOWLOG_MAX_32 - STREAM_ACCUMULATOR_MIN_32
                } else {
                    0
                })
    {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if MEM_64bits() != 0
        && (totalBits as std::ffi::c_int >= 57 - (9 + 9 + 8)) as std::ffi::c_int as std::ffi::c_long
            != 0
    {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if llBits as std::ffi::c_int > 0 {
        seq.litLength = (seq.litLength).wrapping_add(BIT_readBitsFast(
            &mut (*seqState).DStream,
            llBits as std::ffi::c_uint,
        ));
    }
    if MEM_32bits() != 0 {
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    if isLastSeq == 0 {
        ZSTD_updateFseStateWithDInfo(
            &mut (*seqState).stateLL,
            &mut (*seqState).DStream,
            llNext,
            llnbBits,
        );
        ZSTD_updateFseStateWithDInfo(
            &mut (*seqState).stateML,
            &mut (*seqState).DStream,
            mlNext,
            mlnbBits,
        );
        if MEM_32bits() != 0 {
            BIT_reloadDStream(&mut (*seqState).DStream);
        }
        ZSTD_updateFseStateWithDInfo(
            &mut (*seqState).stateOffb,
            &mut (*seqState).DStream,
            ofNext,
            ofnbBits,
        );
        BIT_reloadDStream(&mut (*seqState).DStream);
    }
    seq
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequences_bodySplitLitBuffer(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    let ostart = dst as *mut u8;
    let oend =
        ZSTD_maybeNullPtrAdd(ostart as *mut std::ffi::c_void, maxDstSize as ptrdiff_t) as *mut u8;
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let mut litBufferEnd = (*dctx).litBufferEnd;
    let prefixStart = (*dctx).prefixStart as *const u8;
    let vBase = (*dctx).virtualStart as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: std::ptr::null::<std::ffi::c_char>(),
                start: std::ptr::null::<std::ffi::c_char>(),
                limitPtr: std::ptr::null::<std::ffi::c_char>(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1;
        let mut i: u32 = 0;
        i = 0;
        while i < ZSTD_REP_NUM as u32 {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) =
                *((*dctx).entropy.rep).as_mut_ptr().offset(i as isize) as size_t;
            i = i.wrapping_add(1);
        }
        if ERR_isError(BIT_initDStream(&mut seqState.DStream, seqStart, seqSize)) != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        let mut sequence = {
            seq_t {
                litLength: 0,
                matchLength: 0,
                offset: 0,
            }
        };
        asm!(".p2align 6", options(preserves_flags, att_syntax));
        while nbSeq != 0 {
            sequence =
                ZSTD_decodeSequence(&mut seqState, isLongOffset, (nbSeq == 1) as std::ffi::c_int);
            if litPtr.offset(sequence.litLength as isize) > (*dctx).litBufferEnd {
                break;
            }
            let oneSeqSize = ZSTD_execSequenceSplitLitBuffer(
                op,
                oend,
                litPtr
                    .offset(sequence.litLength as isize)
                    .offset(-(WILDCOPY_OVERLENGTH as isize)),
                sequence,
                &mut litPtr,
                litBufferEnd,
                prefixStart,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize) as std::ffi::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
        }
        if nbSeq > 0 {
            let leftoverLit =
                ((*dctx).litBufferEnd).offset_from(litPtr) as std::ffi::c_long as size_t;
            if leftoverLit != 0 {
                if leftoverLit > oend.offset_from(op) as std::ffi::c_long as size_t {
                    return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
                }
                ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                sequence.litLength = (sequence.litLength).wrapping_sub(leftoverLit);
                op = op.offset(leftoverLit as isize);
            }
            litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
            litBufferEnd = ((*dctx).litExtraBuffer).as_mut_ptr().offset(
                (if 64
                    > (if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    })
                {
                    64
                } else if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                }) as isize,
            );
            (*dctx).litBufferLocation = ZSTD_not_in_dst;
            let oneSeqSize_0 = ZSTD_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litBufferEnd,
                prefixStart,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize_0) as std::ffi::c_long != 0 {
                return oneSeqSize_0;
            }
            op = op.offset(oneSeqSize_0 as isize);
            nbSeq -= 1;
        }
        if nbSeq > 0 {
            asm!(".p2align 6", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 4", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 3", options(preserves_flags, att_syntax));
            while nbSeq != 0 {
                let sequence_0 = ZSTD_decodeSequence(
                    &mut seqState,
                    isLongOffset,
                    (nbSeq == 1) as std::ffi::c_int,
                );
                let oneSeqSize_1 = ZSTD_execSequence(
                    op,
                    oend,
                    sequence_0,
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    vBase,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize_1) as std::ffi::c_long != 0 {
                    return oneSeqSize_1;
                }
                op = op.offset(oneSeqSize_1 as isize);
                nbSeq -= 1;
            }
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        if BIT_endOfDStream(&mut seqState.DStream) == 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTD_REP_NUM as u32 {
            *((*dctx).entropy.rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
    {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as std::ffi::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as std::ffi::c_long as size_t {
            return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut std::ffi::c_void,
                litPtr as *const std::ffi::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.offset(lastLLSize as isize);
        }
        litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
        litBufferEnd = ((*dctx).litExtraBuffer).as_mut_ptr().offset(
            (if 64
                > (if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                })
            {
                64
            } else if ((1) << 16) < (128) << 10 {
                (1) << 16
            } else {
                (128) << 10
            }) as isize,
        );
        (*dctx).litBufferLocation = ZSTD_not_in_dst;
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as std::ffi::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as std::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memcpy(
            op as *mut std::ffi::c_void,
            litPtr as *const std::ffi::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequences_body(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    let ostart = dst as *mut u8;
    let oend = if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_not_in_dst as std::ffi::c_int as std::ffi::c_uint
    {
        ZSTD_maybeNullPtrAdd(ostart as *mut std::ffi::c_void, maxDstSize as ptrdiff_t) as *mut u8
    } else {
        (*dctx).litBuffer
    };
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.offset((*dctx).litSize as isize);
    let prefixStart = (*dctx).prefixStart as *const u8;
    let vBase = (*dctx).virtualStart as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: std::ptr::null::<std::ffi::c_char>(),
                start: std::ptr::null::<std::ffi::c_char>(),
                limitPtr: std::ptr::null::<std::ffi::c_char>(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            prevOffset: [0; 3],
        };
        (*dctx).fseEntropy = 1;
        let mut i: u32 = 0;
        i = 0;
        while i < ZSTD_REP_NUM as u32 {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) =
                *((*dctx).entropy.rep).as_mut_ptr().offset(i as isize) as size_t;
            i = i.wrapping_add(1);
        }
        if ERR_isError(BIT_initDStream(&mut seqState.DStream, seqStart, seqSize)) != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        if !cfg!(miri) {
            asm!(".p2align 6", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 4", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 3", options(preserves_flags, att_syntax));
        }
        while nbSeq != 0 {
            let sequence =
                ZSTD_decodeSequence(&mut seqState, isLongOffset, (nbSeq == 1) as std::ffi::c_int);
            let oneSeqSize = ZSTD_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litEnd,
                prefixStart,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize) as std::ffi::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
        }
        if BIT_endOfDStream(&mut seqState.DStream) == 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTD_REP_NUM as u32 {
            *((*dctx).entropy.rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as std::ffi::c_long as size_t;
    if lastLLSize > oend.offset_from(op) as std::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memcpy(
            op as *mut std::ffi::c_void,
            litPtr as *const std::ffi::c_void,
            lastLLSize as libc::size_t,
        );
        op = op.offset(lastLLSize as isize);
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTD_decompressSequences_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    ZSTD_decompressSequences_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_decompressSequencesSplitLitBuffer_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    ZSTD_decompressSequences_bodySplitLitBuffer(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
#[inline(always)]
unsafe extern "C" fn ZSTD_prefetchMatch(
    mut prefetchPos: size_t,
    sequence: seq_t,
    prefixStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    prefetchPos = prefetchPos.wrapping_add(sequence.litLength);
    let matchBase = if sequence.offset > prefetchPos {
        dictEnd
    } else {
        prefixStart
    };
    let match_0 = ZSTD_wrappedPtrSub(
        ZSTD_wrappedPtrAdd(
            matchBase as *const std::ffi::c_void,
            prefetchPos as ptrdiff_t,
        ),
        sequence.offset as ptrdiff_t,
    ) as *const u8;
    ZSTD_wrappedPtrAdd(match_0 as *const std::ffi::c_void, 64);
    prefetchPos.wrapping_add(sequence.matchLength)
}
#[inline(always)]
unsafe extern "C" fn ZSTD_decompressSequencesLong_body(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    let ostart = dst as *mut u8;
    let oend = if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_in_dst as std::ffi::c_int as std::ffi::c_uint
    {
        (*dctx).litBuffer
    } else {
        ZSTD_maybeNullPtrAdd(ostart as *mut std::ffi::c_void, maxDstSize as ptrdiff_t) as *mut u8
    };
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let mut litBufferEnd = (*dctx).litBufferEnd;
    let prefixStart = (*dctx).prefixStart as *const u8;
    let dictStart = (*dctx).virtualStart as *const u8;
    let dictEnd = (*dctx).dictEnd as *const u8;
    if nbSeq != 0 {
        let mut sequences: [seq_t; 8] = [seq_t {
            litLength: 0,
            matchLength: 0,
            offset: 0,
        }; 8];
        let seqAdvance = if nbSeq < 8 { nbSeq } else { 8 };
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: std::ptr::null::<std::ffi::c_char>(),
                start: std::ptr::null::<std::ffi::c_char>(),
                limitPtr: std::ptr::null::<std::ffi::c_char>(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: std::ptr::null::<ZSTD_seqSymbol>(),
            },
            prevOffset: [0; 3],
        };
        let mut seqNb: std::ffi::c_int = 0;
        let mut prefetchPos = op.offset_from(prefixStart) as std::ffi::c_long as size_t;
        (*dctx).fseEntropy = 1;
        let mut i: std::ffi::c_int = 0;
        i = 0;
        while i < ZSTD_REP_NUM {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) =
                *((*dctx).entropy.rep).as_mut_ptr().offset(i as isize) as size_t;
            i += 1;
        }
        if ERR_isError(BIT_initDStream(&mut seqState.DStream, seqStart, seqSize)) != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, (*dctx).LLTptr);
        ZSTD_initFseState(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            (*dctx).OFTptr,
        );
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, (*dctx).MLTptr);
        seqNb = 0;
        while seqNb < seqAdvance {
            let sequence = ZSTD_decodeSequence(
                &mut seqState,
                isLongOffset,
                (seqNb == nbSeq - 1) as std::ffi::c_int,
            );
            prefetchPos = ZSTD_prefetchMatch(prefetchPos, sequence, prefixStart, dictEnd);
            *sequences.as_mut_ptr().offset(seqNb as isize) = sequence;
            seqNb += 1;
        }
        while seqNb < nbSeq {
            let mut sequence_0 = ZSTD_decodeSequence(
                &mut seqState,
                isLongOffset,
                (seqNb == nbSeq - 1) as std::ffi::c_int,
            );
            if (*dctx).litBufferLocation as std::ffi::c_uint
                == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
                && litPtr.offset(
                    (*sequences
                        .as_mut_ptr()
                        .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize))
                    .litLength as isize,
                ) > (*dctx).litBufferEnd
            {
                let leftoverLit =
                    ((*dctx).litBufferEnd).offset_from(litPtr) as std::ffi::c_long as size_t;
                if leftoverLit != 0 {
                    if leftoverLit > oend.offset_from(op) as std::ffi::c_long as size_t {
                        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                    let fresh13 = &mut (*sequences
                        .as_mut_ptr()
                        .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize))
                    .litLength;
                    *fresh13 = (*fresh13).wrapping_sub(leftoverLit);
                    op = op.offset(leftoverLit as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer).as_mut_ptr().offset(
                    (if 64
                        > (if ((1) << 16) < (128) << 10 {
                            (1) << 16
                        } else {
                            (128) << 10
                        })
                    {
                        64
                    } else if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    }) as isize,
                );
                (*dctx).litBufferLocation = ZSTD_not_in_dst;
                let oneSeqSize = ZSTD_execSequence(
                    op,
                    oend,
                    *sequences
                        .as_mut_ptr()
                        .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize),
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    dictStart,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize) != 0 {
                    return oneSeqSize;
                }
                prefetchPos = ZSTD_prefetchMatch(prefetchPos, sequence_0, prefixStart, dictEnd);
                *sequences
                    .as_mut_ptr()
                    .offset((seqNb & STORED_SEQS_MASK) as isize) = sequence_0;
                op = op.offset(oneSeqSize as isize);
            } else {
                let oneSeqSize_0 = if (*dctx).litBufferLocation as std::ffi::c_uint
                    == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
                {
                    ZSTD_execSequenceSplitLitBuffer(
                        op,
                        oend,
                        litPtr
                            .offset(
                                (*sequences
                                    .as_mut_ptr()
                                    .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize))
                                .litLength as isize,
                            )
                            .offset(-(WILDCOPY_OVERLENGTH as isize)),
                        *sequences
                            .as_mut_ptr()
                            .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize),
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                } else {
                    ZSTD_execSequence(
                        op,
                        oend,
                        *sequences
                            .as_mut_ptr()
                            .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize),
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                };
                if ERR_isError(oneSeqSize_0) != 0 {
                    return oneSeqSize_0;
                }
                prefetchPos = ZSTD_prefetchMatch(prefetchPos, sequence_0, prefixStart, dictEnd);
                *sequences
                    .as_mut_ptr()
                    .offset((seqNb & STORED_SEQS_MASK) as isize) = sequence_0;
                op = op.offset(oneSeqSize_0 as isize);
            }
            seqNb += 1;
        }
        if BIT_endOfDStream(&mut seqState.DStream) == 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        seqNb -= seqAdvance;
        while seqNb < nbSeq {
            let mut sequence_1: *mut seq_t = &mut *sequences
                .as_mut_ptr()
                .offset((seqNb & STORED_SEQS_MASK) as isize)
                as *mut seq_t;
            if (*dctx).litBufferLocation as std::ffi::c_uint
                == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
                && litPtr.offset((*sequence_1).litLength as isize) > (*dctx).litBufferEnd
            {
                let leftoverLit_0 =
                    ((*dctx).litBufferEnd).offset_from(litPtr) as std::ffi::c_long as size_t;
                if leftoverLit_0 != 0 {
                    if leftoverLit_0 > oend.offset_from(op) as std::ffi::c_long as size_t {
                        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit_0);
                    (*sequence_1).litLength = ((*sequence_1).litLength).wrapping_sub(leftoverLit_0);
                    op = op.offset(leftoverLit_0 as isize);
                }
                litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
                litBufferEnd = ((*dctx).litExtraBuffer).as_mut_ptr().offset(
                    (if 64
                        > (if ((1) << 16) < (128) << 10 {
                            (1) << 16
                        } else {
                            (128) << 10
                        })
                    {
                        64
                    } else if ((1) << 16) < (128) << 10 {
                        (1) << 16
                    } else {
                        (128) << 10
                    }) as isize,
                );
                (*dctx).litBufferLocation = ZSTD_not_in_dst;
                let oneSeqSize_1 = ZSTD_execSequence(
                    op,
                    oend,
                    *sequence_1,
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    dictStart,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize_1) != 0 {
                    return oneSeqSize_1;
                }
                op = op.offset(oneSeqSize_1 as isize);
            } else {
                let oneSeqSize_2 = if (*dctx).litBufferLocation as std::ffi::c_uint
                    == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
                {
                    ZSTD_execSequenceSplitLitBuffer(
                        op,
                        oend,
                        litPtr
                            .offset((*sequence_1).litLength as isize)
                            .offset(-(WILDCOPY_OVERLENGTH as isize)),
                        *sequence_1,
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                } else {
                    ZSTD_execSequence(
                        op,
                        oend,
                        *sequence_1,
                        &mut litPtr,
                        litBufferEnd,
                        prefixStart,
                        dictStart,
                        dictEnd,
                    )
                };
                if ERR_isError(oneSeqSize_2) != 0 {
                    return oneSeqSize_2;
                }
                op = op.offset(oneSeqSize_2 as isize);
            }
            seqNb += 1;
        }
        let mut i_0: u32 = 0;
        i_0 = 0;
        while i_0 < ZSTD_REP_NUM as u32 {
            *((*dctx).entropy.rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
    {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as std::ffi::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as std::ffi::c_long as size_t {
            return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut std::ffi::c_void,
                litPtr as *const std::ffi::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.offset(lastLLSize as isize);
        }
        litPtr = ((*dctx).litExtraBuffer).as_mut_ptr();
        litBufferEnd = ((*dctx).litExtraBuffer).as_mut_ptr().offset(
            (if 64
                > (if ((1) << 16) < (128) << 10 {
                    (1) << 16
                } else {
                    (128) << 10
                })
            {
                64
            } else if ((1) << 16) < (128) << 10 {
                (1) << 16
            } else {
                (128) << 10
            }) as isize,
        );
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as std::ffi::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as std::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memmove(
            op as *mut std::ffi::c_void,
            litPtr as *const std::ffi::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
pub const STORED_SEQS: std::ffi::c_int = 8;
pub const STORED_SEQS_MASK: std::ffi::c_int = STORED_SEQS - 1;
pub const ADVANCED_SEQS: std::ffi::c_int = STORED_SEQS;
unsafe extern "C" fn ZSTD_decompressSequencesLong_default(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    ZSTD_decompressSequencesLong_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_decompressSequences_bmi2(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    ZSTD_decompressSequences_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_decompressSequencesSplitLitBuffer_bmi2(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    ZSTD_decompressSequences_bodySplitLitBuffer(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_decompressSequencesLong_bmi2(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    ZSTD_decompressSequencesLong_body(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_decompressSequences(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        return ZSTD_decompressSequences_bmi2(
            dctx,
            dst,
            maxDstSize,
            seqStart,
            seqSize,
            nbSeq,
            isLongOffset,
        );
    }
    ZSTD_decompressSequences_default(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_decompressSequencesSplitLitBuffer(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        return ZSTD_decompressSequencesSplitLitBuffer_bmi2(
            dctx,
            dst,
            maxDstSize,
            seqStart,
            seqSize,
            nbSeq,
            isLongOffset,
        );
    }
    ZSTD_decompressSequencesSplitLitBuffer_default(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_decompressSequencesLong(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
    mut nbSeq: std::ffi::c_int,
    isLongOffset: ZSTD_longOffset_e,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        return ZSTD_decompressSequencesLong_bmi2(
            dctx,
            dst,
            maxDstSize,
            seqStart,
            seqSize,
            nbSeq,
            isLongOffset,
        );
    }
    ZSTD_decompressSequencesLong_default(
        dctx,
        dst,
        maxDstSize,
        seqStart,
        seqSize,
        nbSeq,
        isLongOffset,
    )
}
unsafe extern "C" fn ZSTD_totalHistorySize(
    mut curPtr: *mut std::ffi::c_void,
    mut virtualStart: *const std::ffi::c_void,
) -> size_t {
    (curPtr as *mut std::ffi::c_char).offset_from(virtualStart as *const std::ffi::c_char)
        as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTD_getOffsetInfo(
    mut offTable: *const ZSTD_seqSymbol,
    mut nbSeq: std::ffi::c_int,
) -> ZSTD_OffsetInfo {
    let mut info = {
        ZSTD_OffsetInfo {
            longOffsetShare: 0,
            maxNbAdditionalBits: 0,
        }
    };
    if nbSeq != 0 {
        let mut ptr = offTable as *const std::ffi::c_void;
        let tableLog = (*(ptr as *const ZSTD_seqSymbol_header).offset(0)).tableLog;
        let mut table = offTable.offset(1);
        let max = ((1) << tableLog) as u32;
        let mut u: u32 = 0;
        u = 0;
        while u < max {
            info.maxNbAdditionalBits = if info.maxNbAdditionalBits
                > (*table.offset(u as isize)).nbAdditionalBits as std::ffi::c_uint
            {
                info.maxNbAdditionalBits
            } else {
                (*table.offset(u as isize)).nbAdditionalBits as std::ffi::c_uint
            };
            if (*table.offset(u as isize)).nbAdditionalBits as std::ffi::c_int > 22 {
                info.longOffsetShare = (info.longOffsetShare).wrapping_add(1);
            }
            u = u.wrapping_add(1);
        }
        info.longOffsetShare <<= (OffFSELog as u32).wrapping_sub(tableLog);
    }
    info
}
unsafe extern "C" fn ZSTD_maxShortOffset() -> size_t {
    if MEM_64bits() != 0 {
        -(1 as std::ffi::c_int) as size_t
    } else {
        let maxOffbase = ((1 as std::ffi::c_int as size_t)
            << ((if MEM_32bits() != 0 {
                STREAM_ACCUMULATOR_MIN_32
            } else {
                STREAM_ACCUMULATOR_MIN_64
            }) as u32)
                .wrapping_add(1 as std::ffi::c_int as u32))
        .wrapping_sub(1 as std::ffi::c_int as size_t);

        maxOffbase.wrapping_sub(ZSTD_REP_NUM as size_t)
    }
}
#[export_name = crate::prefix!(ZSTD_decompressBlock_internal)]
pub unsafe extern "C" fn ZSTD_decompressBlock_internal(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    streaming: streaming_operation,
) -> size_t {
    let mut ip = src as *const u8;
    if srcSize > ZSTD_blockSizeMax(dctx) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let litCSize = ZSTD_decodeLiteralsBlock(dctx, src, srcSize, dst, dstCapacity, streaming);
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.offset(litCSize as isize);
    srcSize = srcSize.wrapping_sub(litCSize);
    let blockSizeMax = if dstCapacity < ZSTD_blockSizeMax(dctx) {
        dstCapacity
    } else {
        ZSTD_blockSizeMax(dctx)
    };
    let totalHistorySize = ZSTD_totalHistorySize(
        ZSTD_maybeNullPtrAdd(dst, blockSizeMax as ptrdiff_t),
        (*dctx).virtualStart as *const u8 as *const std::ffi::c_void,
    );
    let mut isLongOffset = (MEM_32bits() != 0 && totalHistorySize > ZSTD_maxShortOffset())
        as std::ffi::c_int as ZSTD_longOffset_e;
    let mut usePrefetchDecoder = (*dctx).ddictIsCold;
    let mut nbSeq: std::ffi::c_int = 0;
    let seqHSize = ZSTD_decodeSeqHeaders(dctx, &mut nbSeq, ip as *const std::ffi::c_void, srcSize);
    if ERR_isError(seqHSize) != 0 {
        return seqHSize;
    }
    ip = ip.offset(seqHSize as isize);
    srcSize = srcSize.wrapping_sub(seqHSize);
    if (dst.is_null() || dstCapacity == 0) && nbSeq > 0 {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if MEM_64bits() != 0
        && ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
            == ::core::mem::size_of::<*mut std::ffi::c_void>() as std::ffi::c_ulong
        && (-(1 as std::ffi::c_int) as size_t).wrapping_sub(dst as size_t)
            < ((1 as std::ffi::c_int) << 20 as std::ffi::c_int) as size_t
    {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if isLongOffset as std::ffi::c_uint != 0
        || usePrefetchDecoder == 0 && totalHistorySize > ((1) << 24) as size_t && nbSeq > 8
    {
        let info = ZSTD_getOffsetInfo((*dctx).OFTptr, nbSeq);
        if isLongOffset as std::ffi::c_uint != 0
            && info.maxNbAdditionalBits
                <= (if MEM_32bits() != 0 {
                    STREAM_ACCUMULATOR_MIN_32
                } else {
                    STREAM_ACCUMULATOR_MIN_64
                }) as u32
        {
            isLongOffset = ZSTD_lo_isRegularOffset;
        }
        if usePrefetchDecoder == 0 {
            let minShare = (if MEM_64bits() != 0 { 7 } else { 20 }) as u32;
            usePrefetchDecoder = (info.longOffsetShare >= minShare) as std::ffi::c_int;
        }
    }
    (*dctx).ddictIsCold = 0;
    if usePrefetchDecoder != 0 {
        return ZSTD_decompressSequencesLong(
            dctx,
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        );
    }
    if (*dctx).litBufferLocation as std::ffi::c_uint
        == ZSTD_split as std::ffi::c_int as std::ffi::c_uint
    {
        ZSTD_decompressSequencesSplitLitBuffer(
            dctx,
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        )
    } else {
        ZSTD_decompressSequences(
            dctx,
            dst,
            dstCapacity,
            ip as *const std::ffi::c_void,
            srcSize,
            nbSeq,
            isLongOffset,
        )
    }
}
#[export_name = crate::prefix!(ZSTD_checkContinuity)]
pub unsafe extern "C" fn ZSTD_checkContinuity(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *const std::ffi::c_void,
    mut dstSize: size_t,
) {
    if dst != (*dctx).previousDstEnd && dstSize > 0 {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx).virtualStart = (dst as *const std::ffi::c_char).offset(
            -(((*dctx).previousDstEnd as *const std::ffi::c_char)
                .offset_from((*dctx).prefixStart as *const std::ffi::c_char)
                as std::ffi::c_long as isize),
        ) as *const std::ffi::c_void;
        (*dctx).prefixStart = dst;
        (*dctx).previousDstEnd = dst;
    }
}
#[export_name = crate::prefix!(ZSTD_decompressBlock_deprecated)]
pub unsafe extern "C" fn ZSTD_decompressBlock_deprecated(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut dSize: size_t = 0;
    (*dctx).isFrameDecompression = 0;
    ZSTD_checkContinuity(dctx, dst, dstCapacity);
    dSize = ZSTD_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize, not_streaming);
    let err_code = dSize;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    (*dctx).previousDstEnd =
        (dst as *mut std::ffi::c_char).offset(dSize as isize) as *const std::ffi::c_void;
    dSize
}
#[export_name = crate::prefix!(ZSTD_decompressBlock)]
pub unsafe extern "C" fn ZSTD_decompressBlock(
    mut dctx: *mut ZSTD_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_decompressBlock_deprecated(dctx, dst, dstCapacity, src, srcSize)
}
