use core::arch::asm;
use core::ptr;

use libc::{ptrdiff_t, size_t};

use crate::lib::common::bitstream::BIT_DStream_t;
use crate::lib::common::entropy_common::FSE_readNCount_slice;
use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::mem::{MEM_32bits, MEM_64bits, MEM_readLE24};
use crate::lib::common::zstd_internal::{
    LLFSELog, LL_bits, MLFSELog, ML_bits, MaxFSELog, MaxLL, MaxLLBits, MaxML, MaxMLBits, MaxOff,
    MaxSeq, OffFSELog, ZSTD_copy16, ZSTD_copy8, ZSTD_wildcopy, LL_DEFAULTNORMLOG,
    ML_DEFAULTNORMLOG, OF_DEFAULTNORMLOG, WILDCOPY_OVERLENGTH, WILDCOPY_VECLEN, ZSTD_REP_NUM,
};
use crate::lib::decompress::huf_decompress::{DTable, HUF_decompress4X_hufOnly_wksp, Writer};
use crate::lib::decompress::huf_decompress::{
    HUF_decompress1X1_DCtx_wksp, HUF_decompress1X_usingDTable, HUF_decompress4X_usingDTable,
};
use crate::lib::decompress::{blockProperties_t, BlockType, SymbolTable};
use crate::lib::decompress::{
    HUF_DTable, LL_base, LitLocation, ML_base, OF_base, OF_bits, Workspace, ZSTD_DCtx, ZSTD_DCtx_s,
    ZSTD_seqSymbol, ZSTD_seqSymbol_header,
};

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
type XXH64_hash_t = u64;
type XXH32_hash_t = u32;

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
#[derive(Copy, Clone, Default)]
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
unsafe fn ZSTD_maybeNullPtrAdd(
    ptr: *mut core::ffi::c_void,
    add: ptrdiff_t,
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
pub const ZSTD_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
unsafe fn ZSTD_copy4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
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

pub unsafe fn ZSTD_getcBlockSize(
    src: *const core::ffi::c_void,
    srcSize: size_t,
    bpPtr: &mut blockProperties_t,
) -> size_t {
    if srcSize < ZSTD_blockHeaderSize {
        return Error::srcSize_wrong.to_error_code();
    }
    let cBlockHeader = MEM_readLE24(src);
    let cSize = cBlockHeader >> 3;

    bpPtr.lastBlock = cBlockHeader & 1;
    bpPtr.blockType = BlockType::from(cBlockHeader >> 1 & 0b11);
    bpPtr.origSize = cSize;

    match bpPtr.blockType {
        BlockType::Raw | BlockType::Compressed => cSize as size_t,
        BlockType::Rle => 1,
        BlockType::Reserved => Error::corruption_detected.to_error_code(),
    }
}

pub fn getc_block_size(src: &[u8]) -> Result<(blockProperties_t, usize), Error> {
    let [a, b, c, ..] = *src else {
        return Err(Error::srcSize_wrong);
    };

    let cBlockHeader = u32::from_le_bytes([a, b, c, 0]);
    let cSize = cBlockHeader >> 3;

    let bp = blockProperties_t {
        lastBlock: cBlockHeader & 1,
        blockType: BlockType::from(cBlockHeader >> 1 & 0b11),
        origSize: cSize,
    };

    match bp.blockType {
        BlockType::Raw | BlockType::Compressed => Ok((bp, cSize as size_t)),
        BlockType::Rle => Ok((bp, 1)),
        BlockType::Reserved => Err(Error::corruption_detected),
    }
}

unsafe fn ZSTD_allocateLiteralsBuffer(
    dctx: &mut ZSTD_DCtx,
    mut dst: Writer<'_>,
    litSize: usize,
    streaming: StreamingOperation,
    expectedWriteSize: usize,
    split_immediately: bool,
) {
    let dstCapacity = dst.capacity();
    let dst = dst.as_mut_ptr();

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
    dst: Writer<'_>,
    streaming: StreamingOperation,
) -> size_t {
    // for a non-null block
    const MIN_CBLOCK_SIZE: usize = 1 /*litCSize*/ + 1/* RLE or RAW */;
    if src.len() < MIN_CBLOCK_SIZE {
        return Error::corruption_detected.to_error_code();
    }

    let litEncType = SymbolEncodingType_e::try_from(src[0] & 0b11).unwrap();

    let blockSizeMax = dctx.block_size_max();

    match litEncType {
        SymbolEncodingType_e::set_repeat if dctx.litEntropy == 0 => {
            return Error::dictionary_corrupted.to_error_code();
        }
        SymbolEncodingType_e::set_repeat | SymbolEncodingType_e::set_compressed => {}
        SymbolEncodingType_e::set_basic => {
            let (lhSize, litSize) = match src[0] >> 2 & 0b11 {
                1 => (2usize, (u16::from_le_bytes([src[0], src[1]]) >> 4) as usize),
                3 => {
                    let [a, b, c, ..] = *src else {
                        return Error::corruption_detected.to_error_code();
                    };

                    (3, (u32::from_le_bytes([a, b, c, 0]) >> 4) as usize)
                }
                _ => (1, (src[0] >> 3) as usize),
            };

            if litSize > 0 && dst.is_null() {
                return Error::dstSize_tooSmall.to_error_code();
            }
            if litSize > blockSizeMax {
                return Error::corruption_detected.to_error_code();
            }

            let expectedWriteSize = Ord::min(dst.capacity(), blockSizeMax);
            if expectedWriteSize < litSize {
                return Error::dstSize_tooSmall.to_error_code();
            }

            ZSTD_allocateLiteralsBuffer(dctx, dst, litSize, streaming, expectedWriteSize, true);

            if lhSize + litSize + WILDCOPY_OVERLENGTH > src.len() {
                if litSize.wrapping_add(lhSize) > src.len() {
                    return Error::corruption_detected.to_error_code();
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
            dctx.litBufferEnd = (dctx.litPtr).add(litSize);
            dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;

            return lhSize.wrapping_add(litSize) as size_t;
        }
        SymbolEncodingType_e::set_rle => {
            let (lhSize, litSize) = match src[0] >> 2 & 0b11 {
                1 => {
                    let [a, b, _, ..] = *src else {
                        return Error::corruption_detected.to_error_code();
                    };

                    (2usize, (u16::from_le_bytes([a, b]) >> 4) as usize)
                }
                3 => {
                    let [a, b, c, _, ..] = *src else {
                        return Error::corruption_detected.to_error_code();
                    };

                    (3, (u32::from_le_bytes([a, b, c, 0]) >> 4) as usize)
                }
                _ => (1, (src[0] >> 3) as usize),
            };

            if litSize > 0 && dst.is_null() {
                return Error::dstSize_tooSmall.to_error_code();
            }
            if litSize > blockSizeMax {
                return Error::corruption_detected.to_error_code();
            }

            let expectedWriteSize = Ord::min(dst.capacity(), blockSizeMax);
            if expectedWriteSize < litSize {
                return Error::dstSize_tooSmall.to_error_code();
            }

            ZSTD_allocateLiteralsBuffer(dctx, dst, litSize, streaming, expectedWriteSize, true);

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
        return Error::corruption_detected.to_error_code();
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
        return Error::dstSize_tooSmall.to_error_code();
    }
    if litSize > blockSizeMax {
        return Error::corruption_detected.to_error_code();
    }
    if !singleStream && litSize < 6 {
        return Error::literals_headerWrong.to_error_code();
    }
    if litCSize.wrapping_add(lhSize) > src.len() {
        return Error::corruption_detected.to_error_code();
    }

    let expectedWriteSize = Ord::min(dst.capacity(), blockSizeMax);
    if expectedWriteSize < litSize {
        return Error::dstSize_tooSmall.to_error_code();
    }

    ZSTD_allocateLiteralsBuffer(dctx, dst, litSize, streaming, expectedWriteSize, false);

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
        dctx.litBufferEnd = (dctx.litBufferEnd).sub(WILDCOPY_OVERLENGTH);
    }

    if ERR_isError(hufSuccess) != 0 {
        return Error::corruption_detected.to_error_code();
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
    dctx: *mut ZSTD_DCtx,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
) -> size_t {
    let Some(dctx) = dctx.as_mut() else {
        return Error::GENERIC.to_error_code();
    };

    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast::<u8>(), srcSize)
    };

    // NOTE: already handles the `dst.is_null()` case.
    let dst = Writer::from_raw_parts(dst.cast::<u8>(), dstCapacity);

    dctx.isFrameDecompression = 0;

    ZSTD_decodeLiteralsBlock(dctx, src, dst, StreamingOperation::NotStreaming)
}

const fn sequence_symbol(
    nextState: u16,
    nbAdditionalBits: u8,
    nbBits: u8,
    baseValue: u32,
) -> ZSTD_seqSymbol {
    ZSTD_seqSymbol {
        nextState,
        nbAdditionalBits,
        nbBits,
        baseValue,
    }
}

/// Default FSE distribution table for Literal Lengths.
#[rustfmt::skip]
static LL_defaultDTable: [ZSTD_seqSymbol; (1 << LL_DEFAULTNORMLOG) + 1] = [
    /* header : fastMode, tableLog */
    sequence_symbol(1,  1,  1, LL_DEFAULTNORMLOG),
    /* nextState, nbAddBits, nbBits, baseVal */
    sequence_symbol( 0,  0,  4,    0), sequence_symbol(16,  0,  4,    0),
    sequence_symbol(32,  0,  5,    1), sequence_symbol( 0,  0,  5,    3),
    sequence_symbol( 0,  0,  5,    4), sequence_symbol( 0,  0,  5,    6),
    sequence_symbol( 0,  0,  5,    7), sequence_symbol( 0,  0,  5,    9),
    sequence_symbol( 0,  0,  5,   10), sequence_symbol( 0,  0,  5,   12),
    sequence_symbol( 0,  0,  6,   14), sequence_symbol( 0,  1,  5,   16),
    sequence_symbol( 0,  1,  5,   20), sequence_symbol( 0,  1,  5,   22),
    sequence_symbol( 0,  2,  5,   28), sequence_symbol( 0,  3,  5,   32),
    sequence_symbol( 0,  4,  5,   48), sequence_symbol(32,  6,  5,   64),
    sequence_symbol( 0,  7,  5,  128), sequence_symbol( 0,  8,  6,  256),
    sequence_symbol( 0, 10,  6, 1024), sequence_symbol( 0, 12,  6, 4096),
    sequence_symbol(32,  0,  4,    0), sequence_symbol( 0,  0,  4,    1),
    sequence_symbol( 0,  0,  5,    2), sequence_symbol(32,  0,  5,    4),
    sequence_symbol( 0,  0,  5,    5), sequence_symbol(32,  0,  5,    7),
    sequence_symbol( 0,  0,  5,    8), sequence_symbol(32,  0,  5,   10),
    sequence_symbol( 0,  0,  5,   11), sequence_symbol( 0,  0,  6,   13),
    sequence_symbol(32,  1,  5,   16), sequence_symbol( 0,  1,  5,   18),
    sequence_symbol(32,  1,  5,   22), sequence_symbol( 0,  2,  5,   24),
    sequence_symbol(32,  3,  5,   32), sequence_symbol( 0,  3,  5,   40),
    sequence_symbol( 0,  6,  4,   64), sequence_symbol(16,  6,  4,   64),
    sequence_symbol(32,  7,  5,  128), sequence_symbol( 0,  9,  6,  512),
    sequence_symbol( 0, 11,  6, 2048), sequence_symbol(48,  0,  4,    0),
    sequence_symbol(16,  0,  4,    1), sequence_symbol(32,  0,  5,    2),
    sequence_symbol(32,  0,  5,    3), sequence_symbol(32,  0,  5,    5),
    sequence_symbol(32,  0,  5,    6), sequence_symbol(32,  0,  5,    8),
    sequence_symbol(32,  0,  5,    9), sequence_symbol(32,  0,  5,   11),
    sequence_symbol(32,  0,  5,   12), sequence_symbol( 0,  0,  6,   15),
    sequence_symbol(32,  1,  5,   18), sequence_symbol(32,  1,  5,   20),
    sequence_symbol(32,  2,  5,   24), sequence_symbol(32,  2,  5,   28),
    sequence_symbol(32,  3,  5,   40), sequence_symbol(32,  4,  5,   48),
    sequence_symbol( 0, 16,  6,65536), sequence_symbol( 0, 15,  6,32768),
    sequence_symbol( 0, 14,  6,16384), sequence_symbol( 0, 13,  6, 8192),
];

/// Default FSE distribution table for Offset Codes.
#[rustfmt::skip]
static OF_defaultDTable: [ZSTD_seqSymbol; (1 << OF_DEFAULTNORMLOG) + 1] = [
    /* header : fastMode, tableLog */
    sequence_symbol(1,  1,  1, OF_DEFAULTNORMLOG),
    /* nextState, nbAddBits, nbBits, baseVal */
    sequence_symbol( 0,  0,  5,    0),     sequence_symbol( 0,  6,  4,   61),
    sequence_symbol( 0,  9,  5,  509),     sequence_symbol( 0, 15,  5,32765),
    sequence_symbol( 0, 21,  5,2097149),   sequence_symbol( 0,  3,  5,    5),
    sequence_symbol( 0,  7,  4,  125),     sequence_symbol( 0, 12,  5, 4093),
    sequence_symbol( 0, 18,  5,262141),    sequence_symbol( 0, 23,  5,8388605),
    sequence_symbol( 0,  5,  5,   29),     sequence_symbol( 0,  8,  4,  253),
    sequence_symbol( 0, 14,  5,16381),     sequence_symbol( 0, 20,  5,1048573),
    sequence_symbol( 0,  2,  5,    1),     sequence_symbol(16,  7,  4,  125),
    sequence_symbol( 0, 11,  5, 2045),     sequence_symbol( 0, 17,  5,131069),
    sequence_symbol( 0, 22,  5,4194301),   sequence_symbol( 0,  4,  5,   13),
    sequence_symbol(16,  8,  4,  253),     sequence_symbol( 0, 13,  5, 8189),
    sequence_symbol( 0, 19,  5,524285),    sequence_symbol( 0,  1,  5,    1),
    sequence_symbol(16,  6,  4,   61),     sequence_symbol( 0, 10,  5, 1021),
    sequence_symbol( 0, 16,  5,65533),     sequence_symbol( 0, 28,  5,268435453),
    sequence_symbol( 0, 27,  5,134217725), sequence_symbol( 0, 26,  5,67108861),
    sequence_symbol( 0, 25,  5,33554429),  sequence_symbol( 0, 24,  5,16777213),
];

/// Default FSE distribution table for Match Lengths.
#[rustfmt::skip]
static ML_defaultDTable: [ZSTD_seqSymbol; (1 << ML_DEFAULTNORMLOG) + 1] = [
    /* header : fastMode, tableLog */
    sequence_symbol(1,  1,  1, ML_DEFAULTNORMLOG),
    /* nextState, nbAddBits, nbBits, baseVal */
    sequence_symbol( 0,  0,  6,    3),  sequence_symbol( 0,  0,  4,    4),
    sequence_symbol(32,  0,  5,    5),  sequence_symbol( 0,  0,  5,    6),
    sequence_symbol( 0,  0,  5,    8),  sequence_symbol( 0,  0,  5,    9),
    sequence_symbol( 0,  0,  5,   11),  sequence_symbol( 0,  0,  6,   13),
    sequence_symbol( 0,  0,  6,   16),  sequence_symbol( 0,  0,  6,   19),
    sequence_symbol( 0,  0,  6,   22),  sequence_symbol( 0,  0,  6,   25),
    sequence_symbol( 0,  0,  6,   28),  sequence_symbol( 0,  0,  6,   31),
    sequence_symbol( 0,  0,  6,   34),  sequence_symbol( 0,  1,  6,   37),
    sequence_symbol( 0,  1,  6,   41),  sequence_symbol( 0,  2,  6,   47),
    sequence_symbol( 0,  3,  6,   59),  sequence_symbol( 0,  4,  6,   83),
    sequence_symbol( 0,  7,  6,  131),  sequence_symbol( 0,  9,  6,  515),
    sequence_symbol(16,  0,  4,    4),  sequence_symbol( 0,  0,  4,    5),
    sequence_symbol(32,  0,  5,    6),  sequence_symbol( 0,  0,  5,    7),
    sequence_symbol(32,  0,  5,    9),  sequence_symbol( 0,  0,  5,   10),
    sequence_symbol( 0,  0,  6,   12),  sequence_symbol( 0,  0,  6,   15),
    sequence_symbol( 0,  0,  6,   18),  sequence_symbol( 0,  0,  6,   21),
    sequence_symbol( 0,  0,  6,   24),  sequence_symbol( 0,  0,  6,   27),
    sequence_symbol( 0,  0,  6,   30),  sequence_symbol( 0,  0,  6,   33),
    sequence_symbol( 0,  1,  6,   35),  sequence_symbol( 0,  1,  6,   39),
    sequence_symbol( 0,  2,  6,   43),  sequence_symbol( 0,  3,  6,   51),
    sequence_symbol( 0,  4,  6,   67),  sequence_symbol( 0,  5,  6,   99),
    sequence_symbol( 0,  8,  6,  259),  sequence_symbol(32,  0,  4,    4),
    sequence_symbol(48,  0,  4,    4),  sequence_symbol(16,  0,  4,    5),
    sequence_symbol(32,  0,  5,    7),  sequence_symbol(32,  0,  5,    8),
    sequence_symbol(32,  0,  5,   10),  sequence_symbol(32,  0,  5,   11),
    sequence_symbol( 0,  0,  6,   14),  sequence_symbol( 0,  0,  6,   17),
    sequence_symbol( 0,  0,  6,   20),  sequence_symbol( 0,  0,  6,   23),
    sequence_symbol( 0,  0,  6,   26),  sequence_symbol( 0,  0,  6,   29),
    sequence_symbol( 0,  0,  6,   32),  sequence_symbol( 0, 16,  6,65539),
    sequence_symbol( 0, 15,  6,32771),  sequence_symbol( 0, 14,  6,16387),
    sequence_symbol( 0, 13,  6, 8195),  sequence_symbol( 0, 12,  6, 4099),
    sequence_symbol( 0, 11,  6, 2051),  sequence_symbol( 0, 10,  6, 1027),
];

fn ZSTD_buildSeqTable_rle<const N: usize>(dt: &mut SymbolTable<N>, baseValue: u32, nbAddBits: u8) {
    dt.header = ZSTD_seqSymbol_header {
        fastMode: 0,
        tableLog: 0,
    };

    dt.symbols[0] = ZSTD_seqSymbol {
        nbBits: 0,
        nextState: 0,
        nbAdditionalBits: nbAddBits,
        baseValue,
    };
}

#[inline(always)]
fn ZSTD_buildFSETable_body<const N: usize>(
    dt: &mut SymbolTable<N>,
    normalizedCounter: &[i16],
    baseValue: &'static [u32],
    nbAdditionalBits: &'static [u8],
    tableLog: core::ffi::c_uint,
    wksp: &mut FseWorkspace,
) {
    let tableDecode = &mut dt.symbols;
    let tableSize = 1usize << tableLog;
    let mut highThreshold = tableSize.wrapping_sub(1);
    let mut DTableH = ZSTD_seqSymbol_header {
        fastMode: 1,
        tableLog,
    };

    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;

    for (s, &v) in normalizedCounter.iter().enumerate() {
        if v == -1 {
            tableDecode[highThreshold].baseValue = s as u32;
            highThreshold = highThreshold.wrapping_sub(1);
            wksp.symbols[s] = 1;
        } else {
            if v >= largeLimit {
                DTableH.fastMode = 0;
            }
            wksp.symbols[s] = v as u16;
        }
    }

    dt.header = DTableH;

    if highThreshold == tableSize - 1 {
        let tableMask = tableSize - 1;
        let step = (tableSize >> 1) + (tableSize >> 3) + 3;
        let add = 0x101010101010101u64;
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

        let mut position = 0usize;
        for s in (0..tableSize).step_by(2) {
            for u in 0..2 {
                let uPosition = position.wrapping_add(u * step) & tableMask;
                tableDecode[uPosition].baseValue = wksp.spread[s + u] as u32;
            }
            position = position.wrapping_add(2 * step) & tableMask;
        }
    } else {
        let tableMask = tableSize - 1;
        let step = (tableSize >> 1) + (tableSize >> 3) + 3;
        let mut position = 0usize;
        for (s, &v) in normalizedCounter.iter().enumerate() {
            for _ in 0..i32::from(v) {
                tableDecode[position].baseValue = s as u32;
                position = position.wrapping_add(step) & tableMask;
                while core::hint::unlikely(position > highThreshold) {
                    position = position.wrapping_add(step) & tableMask;
                }
            }
        }
    }

    for u in 0..tableSize {
        let symbol = tableDecode[u].baseValue as usize;
        let nextState = wksp.symbols[symbol] as u32;
        wksp.symbols[symbol] += 1;

        let nbBits = tableLog.wrapping_sub(nextState.ilog2()) as u8;

        tableDecode[u] = ZSTD_seqSymbol {
            nbBits,
            nextState: (nextState << nbBits).wrapping_sub(tableSize as u32) as u16,
            nbAdditionalBits: nbAdditionalBits[symbol],
            baseValue: baseValue[symbol],
        };
    }
}

fn ZSTD_buildFSETable_body_default<const N: usize>(
    dt: &mut SymbolTable<N>,
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

fn ZSTD_buildFSETable_body_bmi2<const N: usize>(
    dt: &mut SymbolTable<N>,
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

#[derive(Copy, Clone)]
#[repr(C, align(4))]
pub struct FseWorkspace {
    symbols: [u16; MaxSeq + 1],
    spread: [u8; (1 << MaxFSELog) + size_of::<u64>()],
}

pub fn ZSTD_buildFSETable<const N: usize>(
    dt: &mut SymbolTable<N>,
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

fn ZSTD_buildSeqTable<const N: usize>(
    DTableSpace: &mut SymbolTable<N>,
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
                return Error::srcSize_wrong.to_error_code();
            };

            if u32::from(symbol) > max {
                return Error::corruption_detected.to_error_code();
            }

            let baseline = baseValue[usize::from(symbol)];
            let nbBits = nbAdditionalBits[usize::from(symbol)];
            ZSTD_buildSeqTable_rle(DTableSpace, baseline, nbBits);

            *DTablePtr = DTableSpace.as_mut_ptr();
            1
        }
        SymbolEncodingType_e::set_basic => {
            *DTablePtr = defaultTable.as_ptr();
            0
        }
        SymbolEncodingType_e::set_repeat => {
            if flagRepeatTable == 0 {
                return Error::corruption_detected.to_error_code();
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
                return Error::corruption_detected.to_error_code();
            };
            if tableLog > maxLog {
                return Error::corruption_detected.to_error_code();
            }
            ZSTD_buildFSETable(
                DTableSpace,
                &norm[..=max as usize],
                baseValue,
                nbAdditionalBits,
                tableLog,
                wksp.as_fse_workspace(),
                bmi2,
            );
            *DTablePtr = DTableSpace.as_mut_ptr();
            headerSize
        }
    }
}

fn ZSTD_decodeSeqHeaders(
    dctx: &mut ZSTD_DCtx,
    nbSeqPtr: &mut core::ffi::c_int,
    src: &[u8],
) -> size_t {
    let mut ip = 0;
    let [nbSeq, ..] = *src else {
        return Error::srcSize_wrong.to_error_code();
    };
    let mut nbSeq = i32::from(nbSeq);
    ip += 1;
    if nbSeq > 0x7f {
        if nbSeq == 0xff {
            let [_, a, b, ..] = *src else {
                return Error::srcSize_wrong.to_error_code();
            };
            nbSeq = i32::from(u16::from_le_bytes([a, b])) + LONGNBSEQ;
            ip += 2;
        } else {
            if ip >= src.len() {
                return Error::srcSize_wrong.to_error_code();
            }
            nbSeq = ((nbSeq - 0x80) << 8) + i32::from(src[ip]);
            ip += 1;
        }
    }
    *nbSeqPtr = nbSeq;
    if nbSeq == 0 {
        if ip != src.len() {
            return Error::corruption_detected.to_error_code();
        }
        return ip as size_t;
    }

    /* FSE table descriptors */

    // Minimum possible size: 1 byte for symbol encoding types.
    if ip + 1 > src.len() {
        return Error::srcSize_wrong.to_error_code();
    }

    // The last field, Reserved, must be all-zeroes.
    if src[ip] & 0b11 != 0 {
        return Error::corruption_detected.to_error_code();
    }

    let byte = src[ip];
    let LLtype = SymbolEncodingType_e::try_from(byte >> 6).unwrap();
    let OFtype = SymbolEncodingType_e::try_from(byte >> 4 & 0b11).unwrap();
    let MLtype = SymbolEncodingType_e::try_from(byte >> 2 & 0b11).unwrap();

    /* Build DTables */

    ip += 1;
    let llhSize = ZSTD_buildSeqTable(
        &mut dctx.entropy.LLTable,
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
        return Error::corruption_detected.to_error_code();
    }

    ip += llhSize as usize;
    let ofhSize = ZSTD_buildSeqTable(
        &mut dctx.entropy.OFTable,
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
        return Error::corruption_detected.to_error_code();
    }

    ip += ofhSize as usize;
    let mlhSize = ZSTD_buildSeqTable(
        &mut dctx.entropy.MLTable,
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
        return Error::corruption_detected.to_error_code();
    }

    ip += mlhSize as usize;

    ip as size_t
}

#[inline(always)]
unsafe fn ZSTD_overlapCopy8(op: &mut *mut u8, ip: &mut *const u8, offset: size_t) {
    if offset < 8 {
        static dec32table: [u32; 8] = [0, 1, 2, 1, 4, 4, 4, 4];
        static dec64table: [core::ffi::c_int; 8] = [8, 8, 8, 7, 8, 9, 10, 11];
        let sub2 = *dec64table.as_ptr().add(offset);
        *(*op).offset(0) = *(*ip).offset(0);
        *(*op).offset(1) = *(*ip).offset(1);
        *(*op).offset(2) = *(*ip).offset(2);
        *(*op).offset(3) = *(*ip).offset(3);
        *ip = (*ip).offset(*dec32table.as_ptr().add(offset) as isize);
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
    ovtype: ZSTD_overlap_e,
) {
    let diff = op.offset_from(ip) as core::ffi::c_long;
    let oend = op.add(length);
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
unsafe fn ZSTD_safecopyDstBeforeSrc(mut op: *mut u8, mut ip: *const u8, length: size_t) {
    let diff = op.offset_from(ip) as ptrdiff_t;
    let oend = op.add(length);
    if length < 8 || diff > -8 as ptrdiff_t {
        while op < oend {
            let fresh9 = ip;
            ip = ip.offset(1);
            let fresh10 = op;
            op = op.offset(1);
            *fresh10 = *fresh9;
        }
        return;
    }
    if op <= oend.sub(WILDCOPY_OVERLENGTH) && diff < -WILDCOPY_VECLEN as ptrdiff_t {
        ZSTD_wildcopy(
            op as *mut core::ffi::c_void,
            ip as *const core::ffi::c_void,
            oend.sub(WILDCOPY_OVERLENGTH).offset_from(op) as core::ffi::c_long as size_t,
            ZSTD_no_overlap,
        );
        ip = ip.offset(oend.sub(WILDCOPY_OVERLENGTH).offset_from(op) as core::ffi::c_long as isize);
        op = op.offset(oend.sub(WILDCOPY_OVERLENGTH).offset_from(op) as core::ffi::c_long as isize);
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
    litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).add(sequence.litLength);
    let mut match_0: *const u8 = oLitEnd.wrapping_sub(sequence.offset);
    let oend_w = oend.wrapping_sub(WILDCOPY_OVERLENGTH);
    if sequenceLength > oend.offset_from(op) as size_t {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as core::ffi::c_long as size_t {
        return Error::corruption_detected.to_error_code();
    }
    ZSTD_safecopy(op, oend_w, *litPtr, sequence.litLength, ZSTD_no_overlap);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as core::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as core::ffi::c_long as size_t {
            return Error::corruption_detected.to_error_code();
        }
        match_0 = dictEnd.offset(-(prefixStart.offset_from(match_0) as core::ffi::c_long as isize));
        if match_0.add(sequence.matchLength) <= dictEnd {
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
        op = oLitEnd.add(length1);
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
    litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let iLitEnd = (*litPtr).add(sequence.litLength);
    let mut match_0: *const u8 = oLitEnd.offset(-(sequence.offset as isize));
    if sequenceLength > oend.offset_from(op) as core::ffi::c_long as size_t {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as core::ffi::c_long as size_t {
        return Error::corruption_detected.to_error_code();
    }
    if op > *litPtr as *mut u8 && op < (*litPtr).add(sequence.litLength) as *mut u8 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    ZSTD_safecopyDstBeforeSrc(op, *litPtr, sequence.litLength);
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(prefixStart) as core::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(virtualStart) as core::ffi::c_long as size_t {
            return Error::corruption_detected.to_error_code();
        }
        match_0 = dictEnd.offset(-(prefixStart.offset_from(match_0) as core::ffi::c_long as isize));
        if match_0.add(sequence.matchLength) <= dictEnd {
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
        op = oLitEnd.add(length1);
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
    litPtr: &mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.add(sequenceLength);
    let oend_w = oend.wrapping_sub(WILDCOPY_OVERLENGTH);
    let iLitEnd = (*litPtr).add(sequence.litLength);
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
            return Error::corruption_detected.to_error_code();
        }
        match_0 = dictEnd.offset(match_0.offset_from(prefixStart) as core::ffi::c_long as isize);
        if match_0.add(sequence.matchLength) <= dictEnd {
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
        op = oLitEnd.add(length1);
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
    litPtr: *mut *const u8,
    litLimit: *const u8,
    prefixStart: *const u8,
    virtualStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let oLitEnd = op.add(sequence.litLength);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.add(sequenceLength);
    let iLitEnd = (*litPtr).add(sequence.litLength);
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
            return Error::corruption_detected.to_error_code();
        }
        match_0 = dictEnd.offset(match_0.offset_from(prefixStart) as core::ffi::c_long as isize);
        if match_0.add(sequence.matchLength) <= dictEnd {
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
        op = oLitEnd.add(length1);
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
    let ptr = dt as *const core::ffi::c_void;
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
    seqState: &mut seqState_t,
    longOffsets: Offset,
    is_last_sequence: bool,
) -> seq_t {
    let mut seq = seq_t {
        litLength: 0,
        matchLength: 0,
        offset: 0,
    };
    let llDInfo = (seqState.stateLL.table).add(seqState.stateLL.state);
    let mlDInfo = (seqState.stateML.table).add(seqState.stateML.state);
    let ofDInfo = (seqState.stateOffb.table).add(seqState.stateOffb.state);
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

    assert!(llBits <= MaxLLBits);
    assert!(mlBits <= MaxMLBits);
    assert!(ofBits as core::ffi::c_int <= MaxOff);

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
            Err(_) => return Error::corruption_detected.to_error_code(),
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

            if litPtr.wrapping_add(sequence.litLength) > dctx.litBufferEnd {
                break;
            }

            let oneSeqSize = ZSTD_execSequenceSplitLitBuffer(
                op,
                oend,
                litPtr.add(sequence.litLength).sub(WILDCOPY_OVERLENGTH),
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

            op = op.add(oneSeqSize);
            nbSeq -= 1;
        }
        if nbSeq > 0 {
            let leftoverLit =
                (dctx.litBufferEnd).offset_from(litPtr) as core::ffi::c_long as size_t;
            if leftoverLit != 0 {
                if leftoverLit > oend.offset_from(op) as core::ffi::c_long as size_t {
                    return Error::dstSize_tooSmall.to_error_code();
                }
                ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                sequence.litLength = (sequence.litLength).wrapping_sub(leftoverLit);
                op = op.add(leftoverLit);
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
            op = op.add(oneSeqSize_0);
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
                op = op.add(oneSeqSize_1);
                nbSeq -= 1;
            }
        }
        if nbSeq != 0 {
            return Error::corruption_detected.to_error_code();
        }
        if !seqState.DStream.is_empty() {
            return Error::corruption_detected.to_error_code();
        }

        for i_0 in 0..ZSTD_REP_NUM {
            *(dctx.entropy.rep).as_mut_ptr().offset(i_0 as isize) =
                *(seqState.prevOffset).as_mut_ptr().offset(i_0 as isize) as u32;
        }
    }

    if dctx.litBufferLocation == LitLocation::ZSTD_split {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as core::ffi::c_long as size_t {
            return Error::dstSize_tooSmall.to_error_code();
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut core::ffi::c_void,
                litPtr as *const core::ffi::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.add(lastLLSize);
        }
        litPtr = (dctx.litExtraBuffer).as_mut_ptr();
        litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
        dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
    }

    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as core::ffi::c_long as size_t {
        return Error::dstSize_tooSmall.to_error_code();
    }

    if !op.is_null() {
        libc::memcpy(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.add(lastLLSize_0);
    }
    op.offset_from(ostart) as core::ffi::c_long as size_t
}

#[inline(always)]
unsafe fn ZSTD_decompressSequences_body(
    dctx: &mut ZSTD_DCtx,
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    seq: &[u8],
    nbSeq: core::ffi::c_int,
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
    let litEnd = litPtr.add(dctx.litSize);
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
        seqState.prevOffset = dctx.entropy.rep.map(|v| v as usize);
        seqState.DStream = match BIT_DStream_t::new(seq) {
            Ok(v) => v,
            Err(_) => return Error::corruption_detected.to_error_code(),
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

        for nbSeq in (1..=nbSeq).rev() {
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

            op = op.add(oneSeqSize);
        }

        if !seqState.DStream.is_empty() {
            return Error::corruption_detected.to_error_code();
        }

        dctx.entropy.rep = seqState.prevOffset.map(|v| v as u32);
    }

    let lastLLSize = litEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if lastLLSize > oend.offset_from(op) as core::ffi::c_long as size_t {
        return Error::dstSize_tooSmall.to_error_code();
    }

    if !op.is_null() {
        libc::memcpy(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize as libc::size_t,
        );
        op = op.add(lastLLSize);
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
fn prefetch_l1<T>(ptr: *const T) {
    if cfg!(feature = "no-prefetch") {
        return;
    }

    #[cfg(target_arch = "x86_64")]
    {
        use core::arch::x86_64;
        unsafe { x86_64::_mm_prefetch(ptr as *const i8, x86_64::_MM_HINT_T0) };
        return;
    }

    #[cfg(target_arch = "x86")]
    if cfg!(target_feature(enable = "sse2")) {
        use core::arch::x86;
        unsafe { x86::_mm_prefetch(ptr as *const i8, x86::_MM_HINT_T0) };
        return;
    }

    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64;
        // emits `prfm pldl1keep`
        unsafe {
            aarch64::_prefetch(
                ptr as *const i8,
                aarch64::_PREFETCH_READ,
                aarch64::_PREFETCH_LOCALITY3,
            )
        };
        return;
    }
}

#[inline(always)]
unsafe fn ZSTD_prefetchMatch(
    prefetchPos: size_t,
    sequence: seq_t,
    prefixStart: *const u8,
    dictEnd: *const u8,
) -> size_t {
    let matchBase = if sequence.offset > prefetchPos.wrapping_add(sequence.litLength) {
        dictEnd
    } else {
        prefixStart
    };

    let match_ = matchBase
        .wrapping_add(prefetchPos)
        .wrapping_sub(sequence.offset);

    prefetch_l1(match_);
    prefetch_l1(match_.wrapping_add(64));

    prefetchPos.wrapping_add(sequence.matchLength)
}

#[inline(always)]
unsafe fn ZSTD_decompressSequencesLong_body(
    dctx: &mut ZSTD_DCtx,
    mut dst: Writer<'_>,
    seq: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    let ostart = dst.as_mut_ptr();
    let oend = if dctx.litBufferLocation == LitLocation::ZSTD_in_dst {
        dctx.litBuffer
    } else {
        dst.as_mut_ptr_range().end
    };
    let mut op = ostart;
    let mut litPtr = dctx.litPtr;
    let mut litBufferEnd = dctx.litBufferEnd;
    let prefixStart = dctx.prefixStart as *const u8;
    let dictStart = dctx.virtualStart as *const u8;
    let dictEnd = dctx.dictEnd as *const u8;
    if nbSeq != 0 {
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
        dctx.fseEntropy = 1;
        seqState.prevOffset = dctx.entropy.rep.map(|v| v as usize);
        seqState.DStream = match BIT_DStream_t::new(seq) {
            Ok(v) => v,
            Err(_) => return Error::corruption_detected.to_error_code(),
        };

        ZSTD_initFseState(&mut seqState.stateLL, &mut seqState.DStream, dctx.LLTptr);
        ZSTD_initFseState(&mut seqState.stateOffb, &mut seqState.DStream, dctx.OFTptr);
        ZSTD_initFseState(&mut seqState.stateML, &mut seqState.DStream, dctx.MLTptr);

        let mut prefetchPos = op.offset_from(prefixStart) as usize;
        let mut sequences: [seq_t; 8] = [seq_t::default(); 8];

        for seqNb in 0..seqAdvance {
            let sequence = ZSTD_decodeSequence(&mut seqState, offset, seqNb == nbSeq - 1);
            prefetchPos = ZSTD_prefetchMatch(prefetchPos, sequence, prefixStart, dictEnd);
            sequences[seqNb as usize] = sequence;
        }

        for seqNb in seqAdvance..nbSeq {
            let sequence_0 = ZSTD_decodeSequence(&mut seqState, offset, seqNb == nbSeq - 1);
            if dctx.litBufferLocation == LitLocation::ZSTD_split
                && litPtr.add(
                    (sequences[((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as usize]).litLength,
                ) > dctx.litBufferEnd
            {
                let leftoverLit =
                    (dctx.litBufferEnd).offset_from(litPtr) as core::ffi::c_long as size_t;
                if leftoverLit != 0 {
                    if leftoverLit > oend.offset_from(op) as core::ffi::c_long as size_t {
                        return Error::dstSize_tooSmall.to_error_code();
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit);
                    sequences[((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as usize].litLength += 1;
                    op = op.add(leftoverLit);
                }
                litPtr = (dctx.litExtraBuffer).as_mut_ptr();
                litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
                dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
                let oneSeqSize = ZSTD_execSequence(
                    op,
                    oend,
                    sequences[((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as usize],
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
                sequences[(seqNb & STORED_SEQS_MASK) as usize] = sequence_0;
                op = op.add(oneSeqSize);
            } else {
                let sequence = sequences[((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as usize];
                let oneSeqSize_0 = if dctx.litBufferLocation == LitLocation::ZSTD_split {
                    ZSTD_execSequenceSplitLitBuffer(
                        op,
                        oend,
                        litPtr.add(sequence.litLength).sub(WILDCOPY_OVERLENGTH),
                        sequences[((seqNb - ADVANCED_SEQS) & STORED_SEQS_MASK) as usize],
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
                        sequence,
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
                sequences[(seqNb & STORED_SEQS_MASK) as usize] = sequence_0;
                op = op.add(oneSeqSize_0);
            }
        }

        if !seqState.DStream.is_empty() {
            return Error::corruption_detected.to_error_code();
        }

        for seqNb in nbSeq - seqAdvance..nbSeq {
            let sequence = &mut sequences[(seqNb & STORED_SEQS_MASK) as usize];
            if dctx.litBufferLocation == LitLocation::ZSTD_split
                && litPtr.add((*sequence).litLength) > dctx.litBufferEnd
            {
                let leftoverLit_0 =
                    (dctx.litBufferEnd).offset_from(litPtr) as core::ffi::c_long as size_t;
                if leftoverLit_0 != 0 {
                    if leftoverLit_0 > oend.offset_from(op) as core::ffi::c_long as size_t {
                        return Error::dstSize_tooSmall.to_error_code();
                    }
                    ZSTD_safecopyDstBeforeSrc(op, litPtr, leftoverLit_0);
                    (*sequence).litLength = ((*sequence).litLength).wrapping_sub(leftoverLit_0);
                    op = op.add(leftoverLit_0);
                }
                litPtr = (dctx.litExtraBuffer).as_mut_ptr();
                litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
                dctx.litBufferLocation = LitLocation::ZSTD_not_in_dst;
                let oneSeqSize_1 = ZSTD_execSequence(
                    op,
                    oend,
                    *sequence,
                    &mut litPtr,
                    litBufferEnd,
                    prefixStart,
                    dictStart,
                    dictEnd,
                );
                if ERR_isError(oneSeqSize_1) != 0 {
                    return oneSeqSize_1;
                }
                op = op.add(oneSeqSize_1);
            } else {
                let oneSeqSize_2 = if dctx.litBufferLocation == LitLocation::ZSTD_split {
                    ZSTD_execSequenceSplitLitBuffer(
                        op,
                        oend,
                        litPtr.add((*sequence).litLength).sub(WILDCOPY_OVERLENGTH),
                        *sequence,
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
                        *sequence,
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
                op = op.add(oneSeqSize_2);
            }
        }

        dctx.entropy.rep = seqState.prevOffset.map(|v| v as u32);
    }

    if dctx.litBufferLocation == LitLocation::ZSTD_split {
        let lastLLSize = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
        if lastLLSize > oend.offset_from(op) as core::ffi::c_long as size_t {
            return Error::dstSize_tooSmall.to_error_code();
        }
        if !op.is_null() {
            libc::memmove(
                op as *mut core::ffi::c_void,
                litPtr as *const core::ffi::c_void,
                lastLLSize as libc::size_t,
            );
            op = op.add(lastLLSize);
        }
        litPtr = (dctx.litExtraBuffer).as_mut_ptr();
        litBufferEnd = dctx.litExtraBuffer[ZSTD_LITBUFFEREXTRASIZE..].as_mut_ptr();
    }

    let lastLLSize_0 = litBufferEnd.offset_from(litPtr) as core::ffi::c_long as size_t;
    if lastLLSize_0 > oend.offset_from(op) as core::ffi::c_long as size_t {
        return Error::dstSize_tooSmall.to_error_code();
    }

    if !op.is_null() {
        libc::memmove(
            op as *mut core::ffi::c_void,
            litPtr as *const core::ffi::c_void,
            lastLLSize_0 as libc::size_t,
        );
        op = op.add(lastLLSize_0);
    }

    op.offset_from(ostart) as core::ffi::c_long as size_t
}

pub const STORED_SEQS: core::ffi::c_int = 8;
pub const STORED_SEQS_MASK: core::ffi::c_int = STORED_SEQS - 1;
pub const ADVANCED_SEQS: core::ffi::c_int = STORED_SEQS;
unsafe fn ZSTD_decompressSequencesLong_default(
    dctx: &mut ZSTD_DCtx,
    dst: Writer<'_>,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequencesLong_body(dctx, dst, seqStart, nbSeq, offset)
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
    dst: Writer<'_>,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    ZSTD_decompressSequencesLong_body(dctx, dst, seqStart, nbSeq, offset)
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
    dst: Writer<'_>,
    seqStart: &[u8],
    nbSeq: core::ffi::c_int,
    offset: Offset,
) -> size_t {
    if ZSTD_DCtx_get_bmi2(dctx) != 0 {
        ZSTD_decompressSequencesLong_bmi2(dctx, dst, seqStart, nbSeq, offset)
    } else {
        ZSTD_decompressSequencesLong_default(dctx, dst, seqStart, nbSeq, offset)
    }
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
        let ptr = offTable as *const core::ffi::c_void;
        let tableLog = (*(ptr as *const ZSTD_seqSymbol_header).offset(0)).tableLog;
        let table = offTable.offset(1);
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
        return Error::GENERIC.to_error_code();
    };

    let Ok(streaming) = StreamingOperation::try_from(streaming) else {
        return Error::GENERIC.to_error_code();
    };

    let src = if src.is_null() {
        &[]
    } else {
        core::slice::from_raw_parts(src.cast::<u8>(), srcSize)
    };

    // NOTE: already handles the `dst.is_null()` case.
    let dst = Writer::from_raw_parts(dst.cast::<u8>(), dstCapacity);

    ZSTD_decompressBlock_internal_help(dctx, dst, src, streaming)
}

unsafe fn ZSTD_decompressBlock_internal_help(
    dctx: &mut ZSTD_DCtx,
    mut dst: Writer<'_>,
    src: &[u8],
    streaming: StreamingOperation,
) -> size_t {
    if src.len() > dctx.block_size_max() {
        return Error::srcSize_wrong.to_error_code();
    }

    let litCSize = ZSTD_decodeLiteralsBlock(dctx, src, dst.subslice(..), streaming);
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }

    let mut ip = &src[litCSize as usize..];

    let blockSizeMax = Ord::min(dst.capacity(), dctx.block_size_max());
    let totalHistorySize =
        dst.as_mut_ptr().wrapping_add(blockSizeMax) as usize - dctx.virtualStart as usize;
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
    if dst.is_empty() && nbSeq > 0 {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if MEM_64bits() != 0
        && ::core::mem::size_of::<size_t>() == ::core::mem::size_of::<*mut core::ffi::c_void>()
        && (usize::MAX - dst.as_mut_ptr() as usize) < (1 << 20)
    {
        return Error::dstSize_tooSmall.to_error_code();
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
        return ZSTD_decompressSequencesLong(dctx, dst.subslice(..), ip, nbSeq, offset);
    }

    if dctx.litBufferLocation == LitLocation::ZSTD_split {
        ZSTD_decompressSequencesSplitLitBuffer(
            dctx,
            dst.as_mut_ptr().cast(),
            dst.capacity(),
            ip,
            nbSeq,
            offset,
        )
    } else {
        ZSTD_decompressSequences(
            dctx,
            dst.as_mut_ptr().cast(),
            dst.capacity(),
            ip,
            nbSeq,
            offset,
        )
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
    (*dctx).previousDstEnd = (dst as *mut core::ffi::c_char).add(dSize) as *const core::ffi::c_void;
    dSize
}

#[cfg_attr(feature = "export-symbols", export_name = crate::prefix!(ZSTD_decompressBlock))]
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
