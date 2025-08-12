use core::arch::asm;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
use core::ptr;

use libc::size_t;

use crate::lib::common::bitstream::BIT_DStream_t;
use crate::lib::common::entropy_common::{FSE_readNCount, FSE_readNCount_slice};
use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::mem::{MEM_32bits, MEM_64bits, MEM_readLE16, MEM_readLE24, MEM_write64};
use crate::lib::decompress::huf_decompress::{DTable, HUF_decompress4X_hufOnly_wksp, Writer};
use crate::lib::decompress::huf_decompress::{
    HUF_decompress1X1_DCtx_wksp, HUF_decompress1X_usingDTable, HUF_decompress4X_usingDTable,
};
use crate::lib::decompress::{blockProperties_t, BlockType};
use crate::lib::decompress::{
    HUF_DTable, LL_base, LitLocation, ML_base, OF_base, OF_bits, Workspace, ZSTD_DCtx, ZSTD_DCtx_s,
    ZSTD_seqSymbol, ZSTD_seqSymbol_header,
};
use crate::lib::zstd::*;

pub type ptrdiff_t = core::ffi::c_long;
pub type BIT_DStream_status = core::ffi::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
pub type ZSTD_refMultipleDDicts_e = core::ffi::c_uint;
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

pub type streaming_operation = core::ffi::c_uint;
pub const is_streaming: streaming_operation = 1;
pub const not_streaming: streaming_operation = 0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum StreamingOperation {
    NotStreaming = 0,
    IsStreaming = 1,
}

impl TryFrom<u32> for StreamingOperation {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotStreaming),
            1 => Ok(Self::IsStreaming),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Offset {
    Regular = 0,
    Long = 1,
}

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
pub type ZSTD_overlap_e = core::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_OffsetInfo {
    pub longOffsetShare: core::ffi::c_uint,
    pub maxNbAdditionalBits: core::ffi::c_uint,
}

#[repr(u32)]
enum SymbolEncodingType_e {
    set_basic = 0,
    set_rle = 1,
    set_compressed = 2,
    set_repeat = 3,
}

impl TryFrom<u8> for SymbolEncodingType_e {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SymbolEncodingType_e::set_basic),
            1 => Ok(SymbolEncodingType_e::set_rle),
            2 => Ok(SymbolEncodingType_e::set_compressed),
            3 => Ok(SymbolEncodingType_e::set_repeat),
            _ => Err(()),
        }
    }
}

pub const CACHELINE_SIZE: core::ffi::c_int = 64;
#[inline]
unsafe fn ZSTD_wrappedPtrAdd(
    mut ptr: *const core::ffi::c_void,
    mut add: ptrdiff_t,
) -> *const core::ffi::c_void {
    (ptr as *const core::ffi::c_char).offset(add as isize) as *const core::ffi::c_void
}
#[inline]
unsafe fn ZSTD_wrappedPtrSub(
    mut ptr: *const core::ffi::c_void,
    mut sub: ptrdiff_t,
) -> *const core::ffi::c_void {
    (ptr as *const core::ffi::c_char).offset(-(sub as isize)) as *const core::ffi::c_void
}
#[inline]
unsafe fn ZSTD_maybeNullPtrAdd(
    mut ptr: *mut core::ffi::c_void,
    mut add: ptrdiff_t,
) -> *mut core::ffi::c_void {
    if add > 0 {
        (ptr as *mut core::ffi::c_char).offset(add as isize) as *mut core::ffi::c_void
    } else {
        ptr
    }
}

pub const STREAM_ACCUMULATOR_MIN: core::ffi::c_int = match size_of::<usize>() {
    4 => STREAM_ACCUMULATOR_MIN_32,
    8 => STREAM_ACCUMULATOR_MIN_64,
    _ => unreachable!(),
};
pub const STREAM_ACCUMULATOR_MIN_32: core::ffi::c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: core::ffi::c_int = 57;

pub const ZSTD_BLOCKSIZELOG_MAX: core::ffi::c_int = 17;
pub const ZSTD_BLOCKSIZE_MAX: core::ffi::c_int = (1) << ZSTD_BLOCKSIZELOG_MAX;

pub const ZSTD_WINDOWLOG_MAX: core::ffi::c_int = match size_of::<usize>() {
    4 => ZSTD_WINDOWLOG_MAX_32,
    8 => ZSTD_WINDOWLOG_MAX_64,
    _ => unreachable!(),
};
pub const ZSTD_WINDOWLOG_MAX_32: core::ffi::c_int = 30;
pub const ZSTD_WINDOWLOG_MAX_64: core::ffi::c_int = 31;

#[inline]
unsafe fn ZSTD_DCtx_get_bmi2(dctx: *const ZSTD_DCtx_s) -> core::ffi::c_int {
    (*dctx).bmi2
}

pub const ZSTD_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;
pub const ZSTD_REP_NUM: core::ffi::c_int = 3;
pub const ZSTD_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
pub const MaxML: core::ffi::c_int = 52;
pub const MaxLL: core::ffi::c_int = 35;
pub const MaxOff: core::ffi::c_int = 31;
pub const MLFSELog: core::ffi::c_int = 9;
pub const LLFSELog: core::ffi::c_int = 9;
pub const OffFSELog: core::ffi::c_int = 8;
static LL_bits: [u8; 36] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16,
];
pub const LL_DEFAULTNORMLOG: core::ffi::c_int = 6;
static ML_bits: [u8; 53] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
];
pub const ML_DEFAULTNORMLOG: core::ffi::c_int = 6;
pub const OF_DEFAULTNORMLOG: core::ffi::c_int = 5;
unsafe fn ZSTD_copy8(mut dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void) {
    libc::memcpy(dst, src, 8);
}
unsafe fn ZSTD_copy16(mut dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub const WILDCOPY_OVERLENGTH: usize = 32;
pub const WILDCOPY_VECLEN: core::ffi::c_int = 16;
#[inline(always)]
unsafe fn ZSTD_wildcopy(
    mut dst: *mut core::ffi::c_void,
    mut src: *const core::ffi::c_void,
    mut length: size_t,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut u8).offset_from(src as *const u8) as core::ffi::c_long;
    let mut ip = src as *const u8;
    let mut op = dst as *mut u8;
    let oend = op.offset(length as isize);
    if ovtype as core::ffi::c_uint
        == ZSTD_overlap_src_before_dst as core::ffi::c_int as core::ffi::c_uint
        && diff < WILDCOPY_VECLEN as ptrdiff_t
    {
        loop {
            ZSTD_copy8(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(8);
            ip = ip.offset(8);
            if op >= oend {
                break;
            }
        }
    } else {
        ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
        if 16 >= length {
            return;
        }
        op = op.offset(16);
        ip = ip.offset(16);
        loop {
            ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            ZSTD_copy16(op as *mut core::ffi::c_void, ip as *const core::ffi::c_void);
            op = op.offset(16);
            ip = ip.offset(16);
            if op >= oend {
                break;
            }
        }
    };
}
pub const NULL: core::ffi::c_int = 0;
unsafe fn ZSTD_copy4(mut dst: *mut core::ffi::c_void, mut src: *const core::ffi::c_void) {
    libc::memcpy(dst, src, 4);
}

impl ZSTD_DCtx {
    fn block_size_max(&self) -> usize {
        if self.isFrameDecompression != 0 {
            self.fParams.blockSizeMax as usize
        } else {
            ZSTD_BLOCKSIZE_MAX as usize
        }
    }
}

unsafe fn ZSTD_blockSizeMax(mut dctx: *const ZSTD_DCtx) -> size_t {
    (if (*dctx).isFrameDecompression != 0 {
        (*dctx).fParams.blockSizeMax
    } else {
        ZSTD_BLOCKSIZE_MAX as core::ffi::c_uint
    }) as size_t
}

pub unsafe fn ZSTD_getcBlockSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
    bpPtr: &mut blockProperties_t,
) -> size_t {
    if srcSize < ZSTD_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    let cBlockHeader = MEM_readLE24(src);
    let cSize = cBlockHeader >> 3;
    bpPtr.lastBlock = cBlockHeader & 1;
    bpPtr.blockType = BlockType::from(cBlockHeader >> 1 & 0b11);
    bpPtr.origSize = cSize;
    if bpPtr.blockType == BlockType::Rle {
        return 1;
    }
    if bpPtr.blockType == BlockType::Reserved {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    cSize as size_t
}

unsafe fn ZSTD_allocateLiteralsBuffer(
    dctx: &mut ZSTD_DCtx,
    dst: *mut u8,
    dstCapacity: usize,
    litSize: usize,
    streaming: StreamingOperation,
    expectedWriteSize: usize,
    split_immediately: bool,
) {
    let blockSizeMax = dctx.block_size_max();
    if streaming == StreamingOperation::NotStreaming
        && dstCapacity
            > blockSizeMax
                .wrapping_add(WILDCOPY_OVERLENGTH)
                .wrapping_add(litSize)
                .wrapping_add(WILDCOPY_OVERLENGTH)
    {
        dctx.litBuffer = dst.add(blockSizeMax).add(WILDCOPY_OVERLENGTH);
        dctx.litBufferEnd = dctx.litBuffer.add(litSize);
        dctx.litBufferLocation = LitLocation::ZSTD_in_dst;
    } else if litSize <= ZSTD_LITBUFFEREXTRASIZE {
        dctx.litBuffer = (dctx.litExtraBuffer).as_mut_ptr();
        dctx.litBufferEnd = dctx.litBuffer.add(litSize);
        dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
    } else {
        if split_immediately {
            dctx.litBuffer = dst
                .add(expectedWriteSize)
                .sub(litSize)
                .add(ZSTD_LITBUFFEREXTRASIZE)
                .sub(WILDCOPY_OVERLENGTH);
            dctx.litBufferEnd = dctx.litBuffer.add(litSize).sub(ZSTD_LITBUFFEREXTRASIZE);
        } else {
            dctx.litBuffer = dst.add(expectedWriteSize).sub(litSize);
            dctx.litBufferEnd = dst.add(expectedWriteSize);
        }
        dctx.litBufferLocation = LitLocation::ZSTD_split;
    };
}

const ZSTD_LBMIN: usize = 64;
const ZSTD_LBMAX: usize = 128 << 10;

const ZSTD_DECODER_INTERNAL_BUFFER: usize = 1 << 16;

const ZSTD_LITBUFFEREXTRASIZE: usize = {
    // just a const clamp
    if ZSTD_DECODER_INTERNAL_BUFFER < ZSTD_LBMIN {
        ZSTD_LBMIN
    } else if ZSTD_DECODER_INTERNAL_BUFFER > ZSTD_LBMAX {
        ZSTD_LBMAX
    } else {
        ZSTD_DECODER_INTERNAL_BUFFER
    }
};

unsafe fn ZSTD_decodeLiteralsBlock(
    dctx: &mut ZSTD_DCtx,
    src: &[u8],
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    streaming: StreamingOperation,
) -> size_t {
    let dstCapacity = dstCapacity as usize;

    // for a non-null block
    const MIN_CBLOCK_SIZE: usize = 1 /*litCSize*/ + 1/* RLE or RAW */;
    if src.len() < MIN_CBLOCK_SIZE {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let litEncType = SymbolEncodingType_e::try_from(src[0] & 0b11).unwrap();

    let blockSizeMax = dctx.block_size_max();

    match litEncType {
        SymbolEncodingType_e::set_repeat if dctx.litEntropy == 0 => {
            return -(ZSTD_error_dictionary_corrupted as core::ffi::c_int) as size_t;
        }
        SymbolEncodingType_e::set_repeat | SymbolEncodingType_e::set_compressed => {}
        SymbolEncodingType_e::set_basic => {
            let (lhSize, litSize) = match src[0] >> 2 & 0b11 {
                1 => (2usize, (u16::from_le_bytes([src[0], src[1]]) >> 4) as usize),
                3 => {
                    let [a, b, c, ..] = *src else {
                        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
                    };

                    (3, (u32::from_le_bytes([a, b, c, 0]) >> 4) as usize)
                }
                _ => (1, (src[0] >> 3) as usize),
            };

            if litSize > 0 && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
            }
            if litSize > blockSizeMax {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }

            let expectedWriteSize = Ord::min(dstCapacity, blockSizeMax);
            if expectedWriteSize < litSize {
                return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
            }

            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst as *mut u8,
                dstCapacity,
                litSize,
                streaming,
                expectedWriteSize,
                true,
            );

            if lhSize + litSize + WILDCOPY_OVERLENGTH as usize > src.len() {
                if litSize.wrapping_add(lhSize) > src.len() {
                    return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
                }
                if dctx.litBufferLocation == LitLocation::ZSTD_split {
                    libc::memcpy(
                        dctx.litBuffer as *mut core::ffi::c_void,
                        src[lhSize..].as_ptr().cast(),
                        litSize.wrapping_sub(ZSTD_LITBUFFEREXTRASIZE),
                    );

                    dctx.litExtraBuffer[..ZSTD_LITBUFFEREXTRASIZE].copy_from_slice(
                        &src[lhSize + litSize - ZSTD_LITBUFFEREXTRASIZE..]
                            [..ZSTD_LITBUFFEREXTRASIZE],
                    );
                } else {
                    libc::memcpy(
                        dctx.litBuffer as *mut core::ffi::c_void,
                        src[lhSize..].as_ptr().cast(),
                        litSize as libc::size_t,
                    );
                }
                dctx.litPtr = dctx.litBuffer;
                dctx.litSize = litSize as size_t;
                return lhSize.wrapping_add(litSize) as size_t;
            }

            dctx.litPtr = src[lhSize..].as_ptr();
            dctx.litSize = litSize as size_t;
            dctx.litBufferEnd = (dctx.litPtr).offset(litSize as isize);
            dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;

            return lhSize.wrapping_add(litSize) as size_t;
        }
        SymbolEncodingType_e::set_rle => {
            let (lhSize, litSize) = match src[0] >> 2 & 0b11 {
                1 => {
                    let [a, b, _, ..] = *src else {
                        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
                    };

                    (2usize, (u16::from_le_bytes([a, b]) >> 4) as usize)
                }
                3 => {
                    let [a, b, c, _, ..] = *src else {
                        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
                    };

                    (3, (u32::from_le_bytes([a, b, c, 0]) >> 4) as usize)
                }
                _ => (1, (src[0] >> 3) as usize),
            };

            if litSize > 0 && dst.is_null() {
                return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
            }
            if litSize > blockSizeMax {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }

            let expectedWriteSize = Ord::min(dstCapacity, blockSizeMax);
            if expectedWriteSize < litSize {
                return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
            }

            ZSTD_allocateLiteralsBuffer(
                dctx,
                dst as *mut u8,
                dstCapacity,
                litSize,
                streaming,
                expectedWriteSize,
                true,
            );

            if dctx.litBufferLocation == LitLocation::ZSTD_split {
                ptr::write_bytes(
                    dctx.litBuffer as *mut u8,
                    src[lhSize],
                    litSize.wrapping_sub(ZSTD_LITBUFFEREXTRASIZE),
                );
                dctx.litExtraBuffer[..ZSTD_LITBUFFEREXTRASIZE].fill(src[lhSize]);
            } else {
                ptr::write_bytes(dctx.litBuffer as *mut u8, src[lhSize], litSize);
            }
            dctx.litPtr = dctx.litBuffer;
            dctx.litSize = litSize as size_t;
            return lhSize.wrapping_add(1) as size_t;
        }
    }

    let [a, b, c, d, size_correction, ..] = *src else {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    };
    let lhc = u32::from_le_bytes([a, b, c, d]) as usize;

    let flags = {
        let bmi_flag = match ZSTD_DCtx_get_bmi2(dctx) {
            0 => 0,
            _ => HUF_flags_bmi2 as core::ffi::c_int,
        };

        let disable_asm_flag = match dctx.disableHufAsm {
            0 => 0,
            _ => HUF_flags_disableAsm as core::ffi::c_int,
        };

        bmi_flag | disable_asm_flag
    };

    let lhlCode = (src[0] >> 2 & 0b11) as u32;
    let singleStream = lhlCode == 0;

    let (lhSize, litSize, litCSize) = match lhlCode {
        2 => (4, lhc >> 4 & 0x3fff, lhc >> 18),
        3 => (
            5,
            lhc >> 4 & 0x3ffff,
            (lhc >> 22) + ((size_correction as usize) << 10),
        ),
        _ => (3, lhc >> 4 & 0x3ff, lhc >> 14 & 0x3ff),
    };

    if litSize > 0 && dst.is_null() {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if litSize > blockSizeMax {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if !singleStream && litSize < 6 {
        return -(ZSTD_error_literals_headerWrong as core::ffi::c_int) as size_t;
    }
    if litCSize.wrapping_add(lhSize) > src.len() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let expectedWriteSize = Ord::min(dstCapacity, blockSizeMax);
    if expectedWriteSize < litSize {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }

    ZSTD_allocateLiteralsBuffer(
        dctx,
        dst as *mut u8,
        dstCapacity,
        litSize,
        streaming,
        expectedWriteSize,
        false,
    );

    if dctx.ddictIsCold != 0 && litSize > 768 {
        let _ptr = dctx.HUFptr as *const core::ffi::c_char;
        let _size = ::core::mem::size_of::<[HUF_DTable; 4097]>() as size_t;
        let mut _pos: size_t = 0;
        _pos = 0;
        while _pos < _size {
            _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
        }
    }

    let hufSuccess = if let SymbolEncodingType_e::set_repeat = litEncType {
        if singleStream {
            HUF_decompress1X_usingDTable(
                Writer::from_raw_parts(dctx.litBuffer, litSize as _),
                &src[lhSize..][..litCSize],
                dctx.HUFptr.cast::<DTable>().as_ref().unwrap(),
                flags,
            )
        } else {
            HUF_decompress4X_usingDTable(
                Writer::from_raw_parts(dctx.litBuffer, litSize as _),
                &src[lhSize..][..litCSize],
                dctx.HUFptr.cast::<DTable>().as_ref().unwrap(),
                flags,
            )
        }
    } else if singleStream {
        HUF_decompress1X1_DCtx_wksp(
            &mut dctx.entropy.hufTable,
            Writer::from_raw_parts(dctx.litBuffer, litSize as _),
            &src[lhSize..][..litCSize],
            &mut dctx.workspace,
            flags,
        )
    } else {
        HUF_decompress4X_hufOnly_wksp(
            &mut dctx.entropy.hufTable,
            Writer::from_raw_parts(dctx.litBuffer, litSize as _),
            &src[lhSize..][..litCSize],
            &mut dctx.workspace,
            flags,
        )
    };

    if dctx.litBufferLocation == LitLocation::ZSTD_split {
        libc::memcpy(
            (dctx.litExtraBuffer).as_mut_ptr() as *mut core::ffi::c_void,
            (dctx.litBufferEnd).sub(ZSTD_LITBUFFEREXTRASIZE) as *const core::ffi::c_void,
            ZSTD_LITBUFFEREXTRASIZE,
        );
        libc::memmove(
            (dctx.litBuffer).add(ZSTD_LITBUFFEREXTRASIZE).sub(32) as *mut core::ffi::c_void,
            dctx.litBuffer as *const core::ffi::c_void,
            litSize.wrapping_sub(ZSTD_LITBUFFEREXTRASIZE),
        );
        dctx.litBuffer = (dctx.litBuffer).add(ZSTD_LITBUFFEREXTRASIZE - WILDCOPY_OVERLENGTH);
        dctx.litBufferEnd = (dctx.litBufferEnd).offset(-(WILDCOPY_OVERLENGTH as isize));
    }

    if ERR_isError(hufSuccess) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    dctx.litPtr = dctx.litBuffer;
    dctx.litSize = litSize as size_t;
    dctx.litEntropy = 1;

    if let SymbolEncodingType_e::set_compressed = litEncType {
        dctx.HUFptr = &raw const dctx.entropy.hufTable as *const u32;
    }

    litCSize.wrapping_add(lhSize) as size_t
}

pub unsafe fn ZSTD_decodeLiteralsBlock_wrapper(
    mut dctx: *mut ZSTD_DCtx,
    mut src: *const core::ffi::c_void,
    mut srcSize: size_t,
    mut dst: *mut core::ffi::c_void,
    mut dstCapacity: size_t,
) -> size_t {
    let Some(dctx) = dctx.as_mut() else {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    };

    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast::<u8>(), srcSize as usize)
    };

    dctx.isFrameDecompression = 0;

    ZSTD_decodeLiteralsBlock(
        dctx,
        src,
        dst,
        dstCapacity,
        StreamingOperation::NotStreaming,
    )
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

unsafe fn ZSTD_buildSeqTable_rle(
    mut dt: *mut ZSTD_seqSymbol,
    mut baseValue: u32,
    mut nbAddBits: u8,
) {
    let mut ptr = dt as *mut core::ffi::c_void;
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
unsafe fn ZSTD_buildFSETable_body(
    dt: *mut ZSTD_seqSymbol,
    normalizedCounter: &[i16],
    baseValue: &'static [u32],
    nbAdditionalBits: &'static [u8],
    tableLog: core::ffi::c_uint,
    wksp: &mut FseWorkspace,
) {
    let tableDecode = dt.offset(1);
    let tableSize = ((1) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);
    let mut DTableH = ZSTD_seqSymbol_header {
        fastMode: 0,
        tableLog: 0,
    };
    DTableH.tableLog = tableLog;
    DTableH.fastMode = 1;
    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;

    for (s, &v) in normalizedCounter.iter().enumerate() {
        if v == -1 {
            (*tableDecode.offset(highThreshold as isize)).baseValue = s as u32;
            highThreshold = highThreshold.wrapping_sub(1);
            wksp.symbols[s as usize] = 1;
        } else {
            if v >= largeLimit {
                DTableH.fastMode = 0;
            }
            wksp.symbols[s as usize] = v as u16;
        }
    }

    libc::memcpy(
        dt as *mut core::ffi::c_void,
        &mut DTableH as *mut ZSTD_seqSymbol_header as *const core::ffi::c_void,
        ::core::mem::size_of::<ZSTD_seqSymbol_header>(),
    );
    if highThreshold == tableSize.wrapping_sub(1) {
        let tableMask = tableSize.wrapping_sub(1) as size_t;
        let step = (tableSize >> 1)
            .wrapping_add(tableSize >> 3)
            .wrapping_add(3) as size_t;
        let add = 0x101010101010101 as core::ffi::c_ulonglong as u64;
        let mut pos = 0usize;
        let mut sv = 0u64;
        for &v in normalizedCounter {
            let n = v as usize;
            wksp.spread[pos..][..8].copy_from_slice(&sv.to_le_bytes());
            let mut i: usize = 8;
            while i < n {
                wksp.spread[pos..][i..][..8].copy_from_slice(&sv.to_le_bytes());
                i += 8;
            }
            pos = pos.wrapping_add(n);
            sv = sv.wrapping_add(add);
        }
        let mut position = 0 as size_t;
        for s_1 in (0..tableSize as size_t).step_by(2) {
            for u in 0..2 {
                let uPosition = position.wrapping_add(u * step) & tableMask;
                (*tableDecode.offset(uPosition as isize)).baseValue =
                    wksp.spread[s_1 as usize + u as usize] as u32;
            }
            position = position.wrapping_add(2 * step) & tableMask;
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1);
        let step_0 = (tableSize >> 1)
            .wrapping_add(tableSize >> 3)
            .wrapping_add(3);
        let mut position_0 = 0u32;
        for (s_2, &v) in normalizedCounter.iter().enumerate() {
            let n_0 = i32::from(v);
            for _ in 0..n_0 {
                (*tableDecode.offset(position_0 as isize)).baseValue = s_2 as u32;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while core::hint::unlikely(position_0 > highThreshold) {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
            }
        }
    }

    for u_0 in 0..tableSize {
        let symbol = (*tableDecode.offset(u_0 as isize)).baseValue;
        let fresh1 = &mut wksp.symbols[symbol as usize];
        let fresh2 = *fresh1;
        *fresh1 = (*fresh1).wrapping_add(1);
        let nextState = fresh2 as u32;
        (*tableDecode.offset(u_0 as isize)).nbBits = tableLog.wrapping_sub({
            let mut val = nextState;
            val.ilog2()
        }) as u8;
        (*tableDecode.offset(u_0 as isize)).nextState = (nextState
            << (*tableDecode.offset(u_0 as isize)).nbBits as core::ffi::c_int)
            .wrapping_sub(tableSize) as u16;
        (*tableDecode.offset(u_0 as isize)).nbAdditionalBits = nbAdditionalBits[symbol as usize];
        (*tableDecode.offset(u_0 as isize)).baseValue = baseValue[symbol as usize];
    }
}

unsafe fn ZSTD_buildFSETable_body_default(
    dt: *mut ZSTD_seqSymbol,
    normalizedCounter: &[i16],
    baseValue: &'static [u32],
    nbAdditionalBits: &'static [u8],
    tableLog: core::ffi::c_uint,
    wksp: &mut FseWorkspace,
) {
    ZSTD_buildFSETable_body(
        dt,
        normalizedCounter,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
    );
}

unsafe fn ZSTD_buildFSETable_body_bmi2(
    dt: *mut ZSTD_seqSymbol,
    normalizedCounter: &[i16],
    baseValue: &'static [u32],
    nbAdditionalBits: &'static [u8],
    tableLog: core::ffi::c_uint,
    wksp: &mut FseWorkspace,
) {
    ZSTD_buildFSETable_body(
        dt,
        normalizedCounter,
        baseValue,
        nbAdditionalBits,
        tableLog,
        wksp,
    );
}

const fn const_max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}

const MaxSeq: usize = const_max(MaxLL as usize, MaxML as usize); /* Assumption : MaxOff < MaxLL,MaxML */

const MaxFSELog: usize = const_max(
    const_max(MLFSELog as usize, LLFSELog as usize),
    OffFSELog as usize,
);

const ZSTD_BUILD_FSE_TABLE_WKSP_SIZE: usize =
    size_of::<i16>() * (MaxSeq + 1) + (1 << MaxFSELog) + size_of::<u64>();

#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct FseWorkspace {
    symbols: [u16; MaxSeq + 1],
    spread: [u8; (1 << MaxFSELog) + size_of::<u64>()],
}

pub unsafe fn ZSTD_buildFSETable(
    dt: *mut ZSTD_seqSymbol,
    normalizedCounter: &[i16],
    baseValue: &'static [u32],
    nbAdditionalBits: &'static [u8],
    tableLog: core::ffi::c_uint,
    wksp: &mut FseWorkspace,
    bmi2: bool,
) {
    if bmi2 {
        ZSTD_buildFSETable_body_bmi2(
            dt,
            normalizedCounter,
            baseValue,
            nbAdditionalBits,
            tableLog,
            wksp,
        );
    } else {
        ZSTD_buildFSETable_body_default(
            dt,
            normalizedCounter,
            baseValue,
            nbAdditionalBits,
            tableLog,
            wksp,
        );
    }
}
unsafe fn ZSTD_buildSeqTable(
    DTableSpace: *mut ZSTD_seqSymbol,
    DTablePtr: &mut *const ZSTD_seqSymbol,
    type_0: SymbolEncodingType_e,
    mut max: core::ffi::c_uint,
    maxLog: u32,
    src: &[u8],
    baseValue: &'static [u32],
    nbAdditionalBits: &'static [u8],
    defaultTable: &'static [ZSTD_seqSymbol],
    flagRepeatTable: u32,
    ddictIsCold: core::ffi::c_int,
    nbSeq: core::ffi::c_int,
    wksp: &mut Workspace,
    bmi2: bool,
) -> size_t {
    match type_0 {
        SymbolEncodingType_e::set_rle => {
            let [symbol, ..] = *src else {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            };

            if u32::from(symbol) > max {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }

            let baseline = baseValue[usize::from(symbol)];
            let nbBits = nbAdditionalBits[usize::from(symbol)];
            ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);

            *DTablePtr = DTableSpace;
            1
        }
        SymbolEncodingType_e::set_basic => {
            *DTablePtr = defaultTable.as_ptr();
            0
        }
        SymbolEncodingType_e::set_repeat => {
            if flagRepeatTable == 0 {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            if ddictIsCold != 0 && nbSeq > 24 {
                let pStart = *DTablePtr as *const core::ffi::c_void;
                let pSize = (::core::mem::size_of::<ZSTD_seqSymbol>() as size_t)
                    .wrapping_mul((1 + ((1) << maxLog)) as size_t);
                let _ptr = pStart as *const core::ffi::c_char;
                let _size = pSize;
                let mut _pos: size_t = 0;
                _pos = 0;
                while _pos < _size {
                    _pos = _pos.wrapping_add(CACHELINE_SIZE as size_t);
                }
            }
            0
        }
        SymbolEncodingType_e::set_compressed => {
            let mut tableLog: core::ffi::c_uint = 0;
            let mut norm: [i16; 53] = [0; 53];
            let Ok(headerSize) = FSE_readNCount_slice(&mut norm, &mut max, &mut tableLog, src)
            else {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            };
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
            }
            ZSTD_buildFSETable(
                DTableSpace,
                &mut norm[..=max as usize],
                &baseValue,
                &nbAdditionalBits,
                tableLog,
                wksp.as_fse_workspace(),
                bmi2,
            );
            *DTablePtr = DTableSpace;
            headerSize
        }
    }
}

unsafe fn ZSTD_decodeSeqHeaders(
    dctx: &mut ZSTD_DCtx,
    nbSeqPtr: &mut core::ffi::c_int,
    src: &[u8],
) -> size_t {
    let mut ip = 0;
    let [nbSeq, ..] = *src else {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    };
    let mut nbSeq = i32::from(nbSeq);
    ip += 1;
    if nbSeq > 0x7f {
        if nbSeq == 0xff {
            let [_, a, b, ..] = *src else {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            };
            nbSeq = i32::from(u16::from_le_bytes([a, b])) + LONGNBSEQ;
            ip += 2;
        } else {
            if ip >= src.len() {
                return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
            }
            nbSeq = ((nbSeq - 0x80) << 8) + i32::from(src[ip]);
            ip += 1;
        }
    }
    *nbSeqPtr = nbSeq;
    if nbSeq == 0 {
        if ip != src.len() {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        return ip as size_t;
    }

    /* FSE table descriptors */

    // Minimum possible size: 1 byte for symbol encoding types.
    if ip + 1 > src.len() {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    // The last field, Reserved, must be all-zeroes.
    if src[ip] & 0b11 != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let byte = src[ip];
    let LLtype = SymbolEncodingType_e::try_from(byte >> 6).unwrap();
    let OFtype = SymbolEncodingType_e::try_from(byte >> 4 & 0b11).unwrap();
    let MLtype = SymbolEncodingType_e::try_from(byte >> 2 & 0b11).unwrap();

    /* Build DTables */

    ip += 1;
    let llhSize = ZSTD_buildSeqTable(
        (dctx.entropy.LLTable).as_mut_ptr(),
        &mut dctx.LLTptr,
        LLtype,
        MaxLL as core::ffi::c_uint,
        LLFSELog as u32,
        &src[ip..],
        &LL_base,
        &LL_bits,
        &LL_defaultDTable,
        dctx.fseEntropy,
        dctx.ddictIsCold,
        nbSeq,
        &mut dctx.workspace,
        dctx.bmi2 != 0,
    );
    if ERR_isError(llhSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    ip += llhSize as usize;
    let ofhSize = ZSTD_buildSeqTable(
        (dctx.entropy.OFTable).as_mut_ptr(),
        &mut dctx.OFTptr,
        OFtype,
        MaxOff as core::ffi::c_uint,
        OffFSELog as u32,
        &src[ip..],
        &OF_base,
        &OF_bits,
        &OF_defaultDTable,
        dctx.fseEntropy,
        dctx.ddictIsCold,
        nbSeq,
        &mut dctx.workspace,
        dctx.bmi2 != 0,
    );
    if ERR_isError(ofhSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    ip += ofhSize as usize;
    let mlhSize = ZSTD_buildSeqTable(
        (dctx.entropy.MLTable).as_mut_ptr(),
        &mut dctx.MLTptr,
        MLtype,
        MaxML as core::ffi::c_uint,
        MLFSELog as u32,
        &src[ip..],
        &ML_base,
        &ML_bits,
        &ML_defaultDTable,
        dctx.fseEntropy,
        dctx.ddictIsCold,
        nbSeq,
        &mut dctx.workspace,
        dctx.bmi2 != 0,
    );
    if ERR_isError(mlhSize) != 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    ip += mlhSize as usize;

    ip as size_t
}

#[inline(always)]
unsafe fn ZSTD_overlapCopy8(op: &mut *mut u8, ip: &mut *const u8, offset: size_t) {
    if offset < 8 {
        static dec32table: [u32; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
        static dec64table: [core::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
        let sub2 = *dec64table.as_ptr().offset(offset as isize);
        *(*op).offset(0) = *(*ip).offset(0);
        *(*op).offset(1) = *(*ip).offset(1);
        *(*op).offset(2) = *(*ip).offset(2);
        *(*op).offset(3) = *(*ip).offset(3);
        *ip = (*ip).offset(*dec32table.as_ptr().offset(offset as isize) as isize);
        ZSTD_copy4(
            (*op).offset(4) as *mut core::ffi::c_void,
            *ip as *const core::ffi::c_void,
        );
        *ip = (*ip).offset(-(sub2 as isize));
    } else {
        ZSTD_copy8(
            *op as *mut core::ffi::c_void,
            *ip as *const core::ffi::c_void,
        );
    }
    *ip = (*ip).offset(8);
    *op = (*op).offset(8);
}
unsafe fn ZSTD_safecopy(
    mut op: *mut u8,
    oend_w: *const u8,
    mut ip: *const u8,
    mut length: size_t,
    mut ovtype: ZSTD_overlap_e,
) {
    let diff = op.offset_from(ip) as core::ffi::c_long;
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
    if ovtype as core::ffi::c_uint
        == ZSTD_overlap_src_before_dst as core::ffi::c_int as core::ffi::c_uint
    {
        ZSTD_overlapCopy8(&mut op, &mut ip, diff as size_t);
        length = length.wrapping_sub(8);
    }
    if oend <= oend_w as *mut u8 {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            ip as *const core::ffi::c_void,
            length,
            ovtype,
        );
        return;
    }
    if op <= oend_w as *mut u8 {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            ip as *const core::ffi::c_void,
            oend_w.offset_from(op) as core::ffi::c_long as size_t,
            ovtype,
        );
        ip = ip.offset(oend_w.offset_from(op) as core::ffi::c_long as isize);
        op = op.offset(oend_w.offset_from(op) as core::ffi::c_long as isize);
    }
    while op < oend {
        let fresh7 = ip;
        ip = ip.offset(1);
        let fresh8 = op;
        op = op.offset(1);
        *fresh8 = *fresh7;
    }
}
unsafe fn ZSTD_safecopyDstBeforeSrc(mut op: *mut u8, mut ip: *const u8, mut length: size_t) {
    let diff = op.offset_from(ip) as core::ffi::c_long;
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
            op as *mut core::ffi::c_void,
            ip as *const core::ffi::c_void,
            oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op) as core::ffi::c_long
                as size_t,
            ZSTD_no_overlap,
        );
        ip = ip.offset(oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op)
            as core::ffi::c_long as isize);
        op = op.offset(oend.offset(-(WILDCOPY_OVERLENGTH as isize)).offset_from(op)
            as core::ffi::c_long as isize);
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
unsafe fn ZSTD_execSequenceEnd(
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
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset as usize);
    let oend_w = oend.wrapping_sub(WILDCOPY_OVERLENGTH as usize);
    if sequenceLength > oend.offset_from(op) as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as core::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    ZSTD_safecopy(op, oend_w, *litPtr, sequence.litLength, ZSTD_no_overlap);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as core::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as core::ffi::c_long as size_t {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(-(prefixStart.offset_from(match_0) as core::ffi::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as core::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
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
unsafe fn ZSTD_execSequenceEndSplitLitBuffer(
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
    if sequenceLength > oend.offset_from(op) as core::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as core::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op > *litPtr as *mut u8 && op < (*litPtr).offset(sequence.litLength as isize) as *mut u8 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    ZSTD_safecopyDstBeforeSrc(op, *litPtr, sequence.litLength);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as core::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as core::ffi::c_long as size_t {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(-(prefixStart.offset_from(match_0) as core::ffi::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as core::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
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
unsafe fn ZSTD_execSequence(
    mut op: *mut u8,
    oend: *mut u8,
    mut sequence: seq_t,
    mut litPtr: &mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_w = oend.wrapping_sub(WILDCOPY_OVERLENGTH as usize);
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const u8 = oLitEnd.wrapping_offset(-(sequence.offset as isize));
    if (iLitEnd > litLimit
        || oMatchEnd > oend_w
        || MEM_32bits() != 0
            && (oend.offset_from(op) as core::ffi::c_long as size_t)
                < sequenceLength.wrapping_add(32)) as core::ffi::c_int as core::ffi::c_long
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
        op as *mut core::ffi::c_void,
        *litPtr as *const core::ffi::c_void,
    );
    if (sequence.litLength > 16) as core::ffi::c_int as core::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op.offset(16) as *mut core::ffi::c_void,
            (*litPtr).offset(16) as *const core::ffi::c_void,
            (sequence.litLength).wrapping_sub(16),
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as core::ffi::c_long as size_t {
        if (sequence.offset > oLitEnd.offset_from(virtualStart) as core::ffi::c_long as size_t)
            as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(match_0.offset_from(prefixStart) as core::ffi::c_long as isize);
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as core::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = prefixStart;
    }
    if (sequence.offset >= 16) as core::ffi::c_int as core::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            sequence.matchLength,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            (sequence.matchLength).wrapping_sub(8),
            ZSTD_overlap_src_before_dst,
        );
    }
    sequenceLength
}
#[inline(always)]
unsafe fn ZSTD_execSequenceSplitLitBuffer(
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
            && (oend.offset_from(op) as core::ffi::c_long as size_t)
                < sequenceLength.wrapping_add(32)) as core::ffi::c_int as core::ffi::c_long
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
        op as *mut core::ffi::c_void,
        *litPtr as *const core::ffi::c_void,
    );
    if (sequence.litLength > 16) as core::ffi::c_int as core::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op.offset(16) as *mut core::ffi::c_void,
            (*litPtr).offset(16) as *const core::ffi::c_void,
            (sequence.litLength).wrapping_sub(16),
            ZSTD_no_overlap,
        );
    }
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as core::ffi::c_long as size_t {
        if (sequence.offset > oLitEnd.offset_from(virtualStart) as core::ffi::c_long as size_t)
            as core::ffi::c_int as core::ffi::c_long
            != 0
        {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(match_0.offset_from(prefixStart) as core::ffi::c_long as isize);
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            libc::memmove(
                oLitEnd as *mut core::ffi::c_void,
                match_0 as *const core::ffi::c_void,
                sequence.matchLength as libc::size_t,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as core::ffi::c_long as size_t;
        libc::memmove(
            oLitEnd as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            length1 as libc::size_t,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = prefixStart;
    }
    if (sequence.offset >= 16) as core::ffi::c_int as core::ffi::c_long != 0 {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            sequence.matchLength,
            ZSTD_no_overlap,
        );
        return sequenceLength;
    }
    ZSTD_overlapCopy8(&mut op, &mut match_0, sequence.offset);
    if sequence.matchLength > 8 {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            match_0 as *const core::ffi::c_void,
            (sequence.matchLength).wrapping_sub(8),
            ZSTD_overlap_src_before_dst,
        );
    }
    sequenceLength
}

unsafe fn ZSTD_initFseState(
    DStatePtr: &mut ZSTD_fseState,
    bitD: &mut BIT_DStream_t,
    dt: *const ZSTD_seqSymbol,
) {
    let mut ptr = dt as *const core::ffi::c_void;
    let DTableH = ptr as *const ZSTD_seqSymbol_header;
    DStatePtr.state = bitD.read_bits((*DTableH).tableLog) as size_t;
    bitD.reload();
    DStatePtr.table = dt.offset(1);
}

#[inline(always)]
fn ZSTD_updateFseStateWithDInfo(
    DStatePtr: &mut ZSTD_fseState,
    bitD: &mut BIT_DStream_t,
    nextState: u16,
    nbBits: u32,
) {
    let lowBits = bitD.read_bits(nbBits);
    DStatePtr.state = (nextState as size_t).wrapping_add(lowBits as size_t);
}

/// We need to add at most (ZSTD_WINDOWLOG_MAX_32 - 1) bits to read the maximum
/// offset bits. But we can only read at most STREAM_ACCUMULATOR_MIN_32
/// bits before reloading. This value is the maximum number of bytes we read
/// after reloading when we are decoding long offsets.
const LONG_OFFSETS_MAX_EXTRA_BITS_32: i32 =
    ZSTD_WINDOWLOG_MAX_32.saturating_sub(STREAM_ACCUMULATOR_MIN_32);

#[inline(always)]
unsafe fn ZSTD_decodeSequence(
    mut seqState: &mut seqState_t,
    longOffsets: Offset,
    is_last_sequence: bool,
) -> seq_t {
    let mut seq = seq_t {
        litLength: 0,
        matchLength: 0,
        offset: 0,
    };
    let llDInfo = (seqState.stateLL.table).offset(seqState.stateLL.state as isize);
    let mlDInfo = (seqState.stateML.table).offset(seqState.stateML.state as isize);
    let ofDInfo = (seqState.stateOffb.table).offset(seqState.stateOffb.state as isize);
    seq.matchLength = (*mlDInfo).baseValue as size_t;
    seq.litLength = (*llDInfo).baseValue as size_t;
    let ofBase = (*ofDInfo).baseValue;
    let llBits = (*llDInfo).nbAdditionalBits;
    let mlBits = (*mlDInfo).nbAdditionalBits;
    let ofBits = (*ofDInfo).nbAdditionalBits;
    let totalBits = (llBits as core::ffi::c_int
        + mlBits as core::ffi::c_int
        + ofBits as core::ffi::c_int) as u8;
    let llNext = (*llDInfo).nextState;
    let mlNext = (*mlDInfo).nextState;
    let ofNext = (*ofDInfo).nextState;
    let llnbBits = (*llDInfo).nbBits as u32;
    let mlnbBits = (*mlDInfo).nbBits as u32;
    let ofnbBits = (*ofDInfo).nbBits as u32;

    const MaxMLBits: u8 = 16;
    const MaxLLBits: u8 = 16;
    const MaxOff: u8 = 31; // NOTE: different for the legacy versions

    assert!(llBits <= MaxLLBits);
    assert!(mlBits <= MaxMLBits);
    assert!(ofBits <= MaxOff);

    let mut offset: size_t = 0;
    if ofBits > 1 {
        const { assert!(Offset::Long as usize == 1) };
        const { assert!(LONG_OFFSETS_MAX_EXTRA_BITS_32 == 5) };
        const { assert!(STREAM_ACCUMULATOR_MIN_32 > LONG_OFFSETS_MAX_EXTRA_BITS_32) };
        const { assert!(STREAM_ACCUMULATOR_MIN_32 - LONG_OFFSETS_MAX_EXTRA_BITS_32 >= MaxMLBits as i32) };

        if MEM_32bits() != 0
            && longOffsets != Offset::Regular
            && ofBits as core::ffi::c_int >= STREAM_ACCUMULATOR_MIN_32
        {
            // Always read extra bits, this keeps the logic simple,
            // avoids branches, and avoids accidentally reading 0 bits.
            let extraBits = LONG_OFFSETS_MAX_EXTRA_BITS_32 as u32;
            offset = (ofBase as size_t).wrapping_add(
                (seqState
                    .DStream
                    .read_bits_fast((ofBits as u32).wrapping_sub(extraBits))
                    as size_t)
                    << extraBits,
            );
            seqState.DStream.reload();
            offset = offset.wrapping_add(seqState.DStream.read_bits_fast(extraBits) as size_t);
        } else {
            offset = (ofBase as size_t).wrapping_add(
                seqState.DStream.read_bits_fast(ofBits as core::ffi::c_uint) as size_t,
            );
            if MEM_32bits() != 0 {
                seqState.DStream.reload();
            }
        }

        seqState.prevOffset[2] = seqState.prevOffset[1];
        seqState.prevOffset[1] = seqState.prevOffset[0];
        seqState.prevOffset[0] = offset;
    } else {
        let ll0 = usize::from((*llDInfo).baseValue == 0);
        if core::hint::likely(ofBits == 0) {
            offset = seqState.prevOffset[ll0];
            seqState.prevOffset[1] = seqState.prevOffset[usize::from(ll0 == 0)];
            seqState.prevOffset[0] = offset;
        } else {
            offset = (ofBase.wrapping_add(ll0 as u32) as size_t)
                .wrapping_add(seqState.DStream.read_bits_fast(1) as size_t);

            let mut temp = match offset {
                3 => seqState.prevOffset[0] - 1,
                _ => seqState.prevOffset[offset as usize],
            };
            temp = temp.wrapping_sub((temp == 0) as _); /* 0 is not valid: input corrupted => force offset to -1 => corruption detected at execSequence */

            if offset != 1 {
                seqState.prevOffset[2] = seqState.prevOffset[1];
            }
            seqState.prevOffset[1] = seqState.prevOffset[0];
            seqState.prevOffset[0] = temp;
            offset = temp;
        }
    }
    seq.offset = offset;

    if mlBits > 0 {
        seq.matchLength = seq
            .matchLength
            .wrapping_add(seqState.DStream.read_bits_fast(mlBits as core::ffi::c_uint) as size_t);
    }

    if cfg!(target_pointer_width = "32")
        && (i32::from(mlBits + llBits)
            >= STREAM_ACCUMULATOR_MIN_32 - LONG_OFFSETS_MAX_EXTRA_BITS_32)
    {
        seqState.DStream.reload();
    }
    if cfg!(target_pointer_width = "64")
        && (totalBits as core::ffi::c_int >= 57 - (9 + 9 + 8)) as core::ffi::c_int
            as core::ffi::c_long
            != 0
    {
        seqState.DStream.reload();
    }

    // Ensure there are enough bits to read the rest of data in 64-bit mode.
    const { assert!(16 + LLFSELog + MLFSELog + OffFSELog < STREAM_ACCUMULATOR_MIN_64) };

    if llBits > 0 {
        seq.litLength = (seq.litLength)
            .wrapping_add(seqState.DStream.read_bits_fast(llBits as core::ffi::c_uint) as size_t);
    }
    if MEM_32bits() != 0 {
        seqState.DStream.reload();
    }

    // Don't update FSE state for last Sequence.
    if !is_last_sequence {
        ZSTD_updateFseStateWithDInfo(
            &mut seqState.stateLL,
            &mut seqState.DStream,
            llNext,
            llnbBits,
        );
        ZSTD_updateFseStateWithDInfo(
            &mut seqState.stateML,
            &mut seqState.DStream,
            mlNext,
            mlnbBits,
        );
        if MEM_32bits() != 0 {
            seqState.DStream.reload();
        }
        ZSTD_updateFseStateWithDInfo(
            &mut seqState.stateOffb,
            &mut seqState.DStream,
            ofNext,
            ofnbBits,
        );
        seqState.DStream.reload();
    }

    seq
}

#[inline(always)]
unsafe fn ZSTD_decompressSequences_bodySplitLitBuffer(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seq: &[u8],
    mut nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    let ostart = dst as *mut u8;
    let oend =
        ZSTD_maybeNullPtrAdd(ostart as *mut core::ffi::c_void, maxDstSize as ptrdiff_t) as *mut u8;
    let mut op = ostart;
    let mut litPtr = dctx.litPtr;
    let mut litBufferEnd = dctx.litBufferEnd;
    let prefixStart = dctx.prefixStart as *const u8;
    let vBase = dctx.virtualStart as *const u8;
    let dictEnd = dctx.dictEnd as *const u8;
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: core::ptr::null::<core::ffi::c_char>(),
                start: core::ptr::null::<core::ffi::c_char>(),
                limitPtr: core::ptr::null::<core::ffi::c_char>(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            prevOffset: [0; 3],
        };
        dctx.fseEntropy = 1;

        seqState.prevOffset = dctx.entropy.rep.map(|v| v as size_t);

        seqState.DStream = match BIT_DStream_t::new(seq) {
            Ok(v) => v,
            Err(_) => return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t,
        };
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, dctx.LLTptr);
        ZSTD_initFseState(&mut seqState.stateOffb, &mut seqState.DStream, dctx.OFTptr);
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, dctx.MLTptr);
        let mut sequence = {
            seq_t {
                litLength: 0,
                matchLength: 0,
                offset: 0,
            }
        };
        asm!(".p2align 6", options(preserves_flags, att_syntax));
        while nbSeq != 0 {
            sequence = ZSTD_decodeSequence(&mut seqState, offset, nbSeq == 1);
            if litPtr.offset(sequence.litLength as isize) > dctx.litBufferEnd {
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
            if ERR_isError(oneSeqSize) as core::ffi::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
        }
        if nbSeq > 0 {
            let leftoverLit =
                ((*dctx).litBufferEnd).offset_from(litPtr) as core::ffi::c_long as size_t;
            if leftoverLit != 0 {
                if leftoverLit > oend.offset_from(op) as core::ffi::c_long as size_t {
                    return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
                }
                ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                sequence.litLength = (sequence.litLength).wrapping_sub(leftoverLit);
                op = op.offset(leftoverLit as isize);
            }
            litPtr = dctx.litExtraBuffer.as_mut_ptr();
            litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
            dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
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
            if ERR_isError(oneSeqSize_0) as core::ffi::c_long != 0 {
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
                let sequence_0 = ZSTD_decodeSequence(&mut seqState, offset, nbSeq == 1);
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
                if ERR_isError(oneSeqSize_1) as core::ffi::c_long != 0 {
                    return oneSeqSize_1;
                }
                op = op.offset(oneSeqSize_1 as isize);
                nbSeq -= 1;
            }
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        if !seqState.DStream.is_empty() {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }

        for i_0 in 0..ZSTD_REP_NUM {
            *(dctx.entropy.rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
        }
    }

    if dctx.litBufferLocation == LitLocation::ZSTD_split {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as core::ffi::c_long as size_t {
            return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut core::ffi::c_void,
                litPtr as *const core::ffi::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.offset(lastLLSize as isize);
        }
        litPtr = (dctx.litExtraBuffer).as_mut_ptr();
        litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
        dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
    }

    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as core::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }

    if !op.is_null() {
        libc::memcpy(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}

#[inline(always)]
unsafe fn ZSTD_decompressSequences_body(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seq: &[u8],
    mut nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    let ostart = dst as *mut u8;
    let oend = if dctx.litBufferLocation == LitLocation::ZSTD_not_in_dst {
        ZSTD_maybeNullPtrAdd(ostart as *mut core::ffi::c_void, maxDstSize as ptrdiff_t) as *mut u8
    } else {
        dctx.litBuffer
    };
    let mut op = ostart;
    let mut litPtr = dctx.litPtr;
    let litEnd = litPtr.offset(dctx.litSize as isize);
    let prefixStart = dctx.prefixStart as *const u8;
    let vBase = dctx.virtualStart as *const u8;
    let dictEnd = dctx.dictEnd as *const u8;
    if nbSeq != 0 {
        let mut seqState = seqState_t {
            DStream: BIT_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: core::ptr::null::<core::ffi::c_char>(),
                start: core::ptr::null::<core::ffi::c_char>(),
                limitPtr: core::ptr::null::<core::ffi::c_char>(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            prevOffset: [0; 3],
        };
        dctx.fseEntropy = 1;
        let mut i: u32 = 0;
        i = 0;
        while i < ZSTD_REP_NUM as u32 {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) =
                *(dctx.entropy.rep).as_mut_ptr().offset(i as isize) as size_t;
            i = i.wrapping_add(1);
        }
        seqState.DStream = match BIT_DStream_t::new(seq) {
            Ok(v) => v,
            Err(_) => return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t,
        };
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, dctx.LLTptr);
        ZSTD_initFseState(&mut seqState.stateOffb, &mut seqState.DStream, dctx.OFTptr);
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, dctx.MLTptr);
        if !cfg!(miri) {
            asm!(".p2align 6", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 4", options(preserves_flags, att_syntax));
            asm!("nop", options(preserves_flags, att_syntax));
            asm!(".p2align 3", options(preserves_flags, att_syntax));
        }
        while nbSeq != 0 {
            let sequence = ZSTD_decodeSequence(&mut seqState, offset, nbSeq == 1);
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
            if ERR_isError(oneSeqSize) as core::ffi::c_long != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
            nbSeq -= 1;
        }
        if !seqState.DStream.is_empty() {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }

        for i_0 in 0..ZSTD_REP_NUM {
            *(dctx.entropy.rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if lastLLSize > oend.offset_from(op) as core::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memcpy(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize as libc::size_t,
        );
        op = op.offset(lastLLSize as isize);
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}

unsafe fn ZSTD_decompressSequences_default(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequences_body(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
}

unsafe fn ZSTD_decompressSequencesSplitLitBuffer_default(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequences_bodySplitLitBuffer(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
}

#[inline(always)]
unsafe fn ZSTD_prefetchMatch(
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
            matchBase as *const core::ffi::c_void,
            prefetchPos as ptrdiff_t,
        ),
        sequence.offset as ptrdiff_t,
    ) as *const u8;
    ZSTD_wrappedPtrAdd(match_0 as *const core::ffi::c_void, 64);
    prefetchPos.wrapping_add(sequence.matchLength)
}

#[inline(always)]
unsafe fn ZSTD_decompressSequencesLong_body(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seq: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    let ostart = dst as *mut u8;
    let oend = if dctx.litBufferLocation == LitLocation::ZSTD_in_dst {
        dctx.litBuffer
    } else {
        ZSTD_maybeNullPtrAdd(ostart as *mut core::ffi::c_void, maxDstSize as ptrdiff_t) as *mut u8
    };
    let mut op = ostart;
    let mut litPtr = dctx.litPtr;
    let mut litBufferEnd = dctx.litBufferEnd;
    let prefixStart = dctx.prefixStart as *const u8;
    let dictStart = dctx.virtualStart as *const u8;
    let dictEnd = dctx.dictEnd as *const u8;
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
                ptr: core::ptr::null::<core::ffi::c_char>(),
                start: core::ptr::null::<core::ffi::c_char>(),
                limitPtr: core::ptr::null::<core::ffi::c_char>(),
            },
            stateLL: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateOffb: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            stateML: ZSTD_fseState {
                state: 0,
                table: core::ptr::null::<ZSTD_seqSymbol>(),
            },
            prevOffset: [0; 3],
        };
        let mut seqNb: core::ffi::c_int = 0;
        let mut prefetchPos = op.offset_from(prefixStart) as core::ffi::c_long as size_t;
        dctx.fseEntropy = 1;
        let mut i: core::ffi::c_int = 0;
        i = 0;
        while i < ZSTD_REP_NUM {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) =
                *(dctx.entropy.rep).as_mut_ptr().offset(i as isize) as size_t;
            i += 1;
        }
        seqState.DStream = match BIT_DStream_t::new(seq) {
            Ok(v) => v,
            Err(_) => return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t,
        };
        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, dctx.LLTptr);
        ZSTD_initFseState(&mut seqState.stateOffb, &mut seqState.DStream, dctx.OFTptr);
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, dctx.MLTptr);
        seqNb = 0;
        while seqNb < seqAdvance {
            let sequence = ZSTD_decodeSequence(&mut seqState, offset, seqNb == nbSeq - 1);
            prefetchPos = ZSTD_prefetchMatch(prefetchPos, sequence, prefixStart, dictEnd);
            *sequences.as_mut_ptr().offset(seqNb as isize) = sequence;
            seqNb += 1;
        }
        while seqNb < nbSeq {
            let mut sequence_0 = ZSTD_decodeSequence(&mut seqState, offset, seqNb == nbSeq - 1);
            if dctx.litBufferLocation == LitLocation::ZSTD_split
                && litPtr.offset(
                    (*sequences
                        .as_mut_ptr()
                        .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize))
                    .litLength as isize,
                ) > dctx.litBufferEnd
            {
                let leftoverLit =
                    (dctx.litBufferEnd).offset_from(litPtr) as core::ffi::c_long as size_t;
                if leftoverLit != 0 {
                    if leftoverLit > oend.offset_from(op) as core::ffi::c_long as size_t {
                        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                    let fresh13 = &mut (*sequences
                        .as_mut_ptr()
                        .offset(((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as isize))
                    .litLength;
                    *fresh13 = (*fresh13).wrapping_sub(leftoverLit);
                    op = op.offset(leftoverLit as isize);
                }
                litPtr = (dctx.litExtraBuffer).as_mut_ptr();
                litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
                dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
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
                let oneSeqSize_0 = if dctx.litBufferLocation == LitLocation::ZSTD_split {
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
        if !seqState.DStream.is_empty() {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        seqNb -= seqAdvance;
        while seqNb < nbSeq {
            let mut sequence_1: *mut seq_t = &mut *sequences
                .as_mut_ptr()
                .offset((seqNb & STORED_SEQS_MASK) as isize)
                as *mut seq_t;
            if dctx.litBufferLocation == LitLocation::ZSTD_split
                && litPtr.offset((*sequence_1).litLength as isize) > dctx.litBufferEnd
            {
                let leftoverLit_0 =
                    (dctx.litBufferEnd).offset_from(litPtr) as core::ffi::c_long as size_t;
                if leftoverLit_0 != 0 {
                    if leftoverLit_0 > oend.offset_from(op) as core::ffi::c_long as size_t {
                        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit_0);
                    (*sequence_1).litLength = ((*sequence_1).litLength).wrapping_sub(leftoverLit_0);
                    op = op.offset(leftoverLit_0 as isize);
                }
                litPtr = (dctx.litExtraBuffer).as_mut_ptr();
                litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
                dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
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
                let oneSeqSize_2 = if dctx.litBufferLocation == LitLocation::ZSTD_split {
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
            *(dctx.entropy.rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
            i_0 = i_0.wrapping_add(1);
        }
    }
    if dctx.litBufferLocation == LitLocation::ZSTD_split {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as core::ffi::c_long as size_t {
            return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut core::ffi::c_void,
                litPtr as *const core::ffi::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.offset(lastLLSize as isize);
        }
        litPtr = (dctx.litExtraBuffer).as_mut_ptr();
        litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
    }
    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as core::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if !op.is_null() {
        libc::memmove(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.offset(lastLLSize_0 as isize);
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}

pub const STORED_SEQS: core::ffi::c_int = 8;
pub const STORED_SEQS_MASK: core::ffi::c_int = STORED_SEQS - 1;
pub const ADVANCED_SEQS: core::ffi::c_int = STORED_SEQS;
unsafe fn ZSTD_decompressSequencesLong_default(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequencesLong_body(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
}

unsafe fn ZSTD_decompressSequences_bmi2(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequences_body(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
}

unsafe fn ZSTD_decompressSequencesSplitLitBuffer_bmi2(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequences_bodySplitLitBuffer(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
}

unsafe fn ZSTD_decompressSequencesLong_bmi2(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequencesLong_body(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
}

unsafe fn ZSTD_decompressSequences(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        ZSTD_decompressSequences_bmi2(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
    } else {
        ZSTD_decompressSequences_default(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
    }
}

unsafe fn ZSTD_decompressSequencesSplitLitBuffer(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        ZSTD_decompressSequencesSplitLitBuffer_bmi2(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
    } else {
        ZSTD_decompressSequencesSplitLitBuffer_default(
            dctx, dst, maxDstSize, seqStart, nbSeq, offset,
        )
    }
}

unsafe fn ZSTD_decompressSequencesLong(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        ZSTD_decompressSequencesLong_bmi2(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
    } else {
        ZSTD_decompressSequencesLong_default(dctx, dst, maxDstSize, seqStart, nbSeq, offset)
    }
}

unsafe fn ZSTD_totalHistorySize(
    curPtr: *mut core::ffi::c_void,
    virtualStart: *const core::ffi::c_void,
) -> size_t {
    (curPtr as *mut core::ffi::c_char).offset_from(virtualStart as *const core::ffi::c_char)
        as core::ffi::c_long as size_t
}

unsafe fn ZSTD_getOffsetInfo(
    offTable: *const ZSTD_seqSymbol,
    nbSeq: core::ffi::c_int,
) -> ZSTD_OffsetInfo {
    let mut info = {
        ZSTD_OffsetInfo {
            longOffsetShare: 0,
            maxNbAdditionalBits: 0,
        }
    };
    if nbSeq != 0 {
        let mut ptr = offTable as *const core::ffi::c_void;
        let tableLog = (*(ptr as *const ZSTD_seqSymbol_header).offset(0)).tableLog;
        let mut table = offTable.offset(1);
        let max = ((1) << tableLog) as u32;
        let mut u: u32 = 0;
        u = 0;
        while u < max {
            info.maxNbAdditionalBits = if info.maxNbAdditionalBits
                > (*table.offset(u as isize)).nbAdditionalBits as core::ffi::c_uint
            {
                info.maxNbAdditionalBits
            } else {
                (*table.offset(u as isize)).nbAdditionalBits as core::ffi::c_uint
            };
            if (*table.offset(u as isize)).nbAdditionalBits as core::ffi::c_int > 22 {
                info.longOffsetShare = (info.longOffsetShare).wrapping_add(1);
            }
            u = u.wrapping_add(1);
        }
        info.longOffsetShare <<= (OffFSELog as u32).wrapping_sub(tableLog);
    }
    info
}

/// @returns The maximum offset we can decode in one read of our bitstream, without
/// reloading more bits in the middle of the offset bits read. Any offsets larger
/// than this must use the long offset decoder.
const fn ZSTD_maxShortOffset() -> size_t {
    match size_of::<usize>() {
        4 => {
            // The maximum offBase is (1 << (STREAM_ACCUMULATOR_MIN + 1)) - 1.
            // This offBase would require STREAM_ACCUMULATOR_MIN extra bits.
            // Then we have to subtract ZSTD_REP_NUM to get the maximum possible offset.
            let maxOffbase = ((1 as size_t) << (STREAM_ACCUMULATOR_MIN as u32 + 1)).wrapping_sub(1);

            maxOffbase.wrapping_sub(ZSTD_REP_NUM as size_t)
        }
        8 => {
            // We can decode any offset without reloading bits.
            // This might change if the max window size grows.
            const { assert!(ZSTD_WINDOWLOG_MAX <= 31) }

            -(1 as core::ffi::c_int) as size_t
        }
        _ => unreachable!(),
    }
}

pub unsafe fn ZSTD_decompressBlock_internal(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    streaming: streaming_operation,
) -> size_t {
    let Some(dctx) = dctx.as_mut() else {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    };

    let Ok(streaming) = StreamingOperation::try_from(streaming) else {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    };

    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast::<u8>(), srcSize as usize)
    };

    ZSTD_decompressBlock_internal_help(dctx, dst, dstCapacity, src, streaming)
}

unsafe fn ZSTD_decompressBlock_internal_help(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: &[u8],
    streaming: StreamingOperation,
) -> size_t {
    if src.len() > dctx.block_size_max() {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    let litCSize = ZSTD_decodeLiteralsBlock(dctx, src, dst, dstCapacity, streaming);
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }

    let mut ip = &src[litCSize as usize..];

    let blockSizeMax = Ord::min(dstCapacity, dctx.block_size_max() as size_t);
    let totalHistorySize = ZSTD_totalHistorySize(
        ZSTD_maybeNullPtrAdd(dst, blockSizeMax as ptrdiff_t),
        dctx.virtualStart as *const u8 as *const core::ffi::c_void,
    );
    let mut offset = if MEM_32bits() != 0 && totalHistorySize > ZSTD_maxShortOffset() {
        Offset::Long
    } else {
        Offset::Regular
    };
    let mut use_prefetch_decoder = dctx.ddictIsCold != 0;
    let mut nbSeq: core::ffi::c_int = 0;
    let seqHSize = ZSTD_decodeSeqHeaders(dctx, &mut nbSeq, ip);
    if ERR_isError(seqHSize) != 0 {
        return seqHSize;
    }
    ip = &ip[seqHSize as usize..];
    if (dst.is_null() || dstCapacity == 0) && nbSeq > 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if MEM_64bits() != 0
        && ::core::mem::size_of::<size_t>() == ::core::mem::size_of::<*mut core::ffi::c_void>()
        && (-(1 as core::ffi::c_int) as size_t).wrapping_sub(dst as size_t)
            < ((1 as core::ffi::c_int) << 20) as size_t
    {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if offset == Offset::Long
        || !use_prefetch_decoder && totalHistorySize > ((1) << 24) as size_t && nbSeq > 8
    {
        let info = ZSTD_getOffsetInfo(dctx.OFTptr, nbSeq);
        if offset == Offset::Long && info.maxNbAdditionalBits <= STREAM_ACCUMULATOR_MIN as u32 {
            offset = Offset::Regular;
        }
        if !use_prefetch_decoder {
            let minShare = (if MEM_64bits() != 0 { 7 } else { 20 }) as u32;
            use_prefetch_decoder = info.longOffsetShare >= minShare;
        }
    }
    dctx.ddictIsCold = 0;
    if use_prefetch_decoder {
        return ZSTD_decompressSequencesLong(dctx, dst, dstCapacity, ip, nbSeq, offset);
    }
    if dctx.litBufferLocation == LitLocation::ZSTD_split {
        ZSTD_decompressSequencesSplitLitBuffer(dctx, dst, dstCapacity, ip, nbSeq, offset)
    } else {
        ZSTD_decompressSequences(dctx, dst, dstCapacity, ip, nbSeq, offset)
    }
}

pub unsafe fn ZSTD_checkContinuity(
    dctx: *mut ZSTD_DCtx,
    dst: *const core::ffi::c_void,
    dstSize: size_t,
) {
    if dst != (*dctx).previousDstEnd && dstSize > 0 {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx).virtualStart = (dst as *const core::ffi::c_char).offset(
            -(((*dctx).previousDstEnd as *const core::ffi::c_char)
                .offset_from((*dctx).prefixStart as *const core::ffi::c_char)
                as core::ffi::c_long as isize),
        ) as *const core::ffi::c_void;
        (*dctx).prefixStart = dst;
        (*dctx).previousDstEnd = dst;
    }
}

unsafe fn ZSTD_decompressBlock_deprecated(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
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
        (dst as *mut core::ffi::c_char).offset(dSize as isize) as *const core::ffi::c_void;
    dSize
}

#[export_name = crate::prefix!(ZSTD_decompressBlock)]
pub unsafe extern "C" fn ZSTD_decompressBlock(
    dctx: *mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    ZSTD_decompressBlock_deprecated(dctx, dst, dstCapacity, src, srcSize)
}

#[cfg(test)]
mod test {
    use core::ffi::*;

    use libc::size_t;

    #[test]
    fn basic_decompress() {
        rs(&[40, 181, 47, 253, 48, 21, 44, 0, 0, 0, 253, 49, 0, 21]);
    }

    fn rs(compressed: &[u8]) -> (usize, Vec<u8>) {
        use crate::lib::decompress::zstd_decompress::*;

        let compressed_ptr = compressed.as_ptr() as *const c_void;
        let compressed_size = compressed.len();

        // Get decompressed size from frame header
        let decompressed_size =
            unsafe { ZSTD_getFrameContentSize(compressed_ptr, compressed_size as size_t) };
        if decompressed_size == ZSTD_CONTENTSIZE_ERROR {
            return (decompressed_size as usize, vec![]);
        } else if decompressed_size == ZSTD_CONTENTSIZE_UNKNOWN {
            return (decompressed_size as usize, vec![]);
        }

        // Allocate buffer for decompressed output
        let mut decompressed = vec![0u8; Ord::min(decompressed_size as usize, 1 << 20)];
        let result = unsafe {
            ZSTD_decompress(
                decompressed.as_mut_ptr() as *mut c_void,
                decompressed.len() as size_t,
                compressed_ptr,
                compressed_size as size_t,
            )
        };

        (result as usize, decompressed)
    }
}
