use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::FSE_CTable;
use crate::lib::common::huf::{HUF_CElt, HUF_flags_bmi2, HUF_CTABLE_SIZE_ST};
use crate::lib::common::mem::{MEM_32bits, MEM_writeLE16, MEM_writeLE24, MEM_writeLE32};
use crate::lib::common::zstd_internal::{
    bt_compressed, bt_raw, DefaultMaxOff, LL_bits, LL_defaultNorm, LL_defaultNormLog, ML_bits,
    ML_defaultNorm, ML_defaultNormLog, MaxLL, MaxML, MaxOff, OF_defaultNorm, OF_defaultNormLog,
    MINMATCH, ZSTD_MAX_HUF_HEADER_SIZE,
};
use crate::lib::compress::hist::{HIST_countFast_wksp, HIST_count_wksp};
use crate::lib::compress::huf_compress::{
    HUF_compress1X_usingCTable, HUF_compress4X_usingCTable, HUF_estimateCompressedSize,
};
use crate::lib::compress::zstd_compress::{
    SeqDef, SeqStore_t, ZSTD_CCtx, ZSTD_CCtx_params, ZSTD_CDict, ZSTD_MatchState_t,
    ZSTD_buildBlockEntropyStats, ZSTD_compressedBlockState_t, ZSTD_entropyCTablesMetadata_t,
    ZSTD_entropyCTables_t, ZSTD_fseCTablesMetadata_t, ZSTD_fseCTables_t, ZSTD_hufCTablesMetadata_t,
    ZSTD_hufCTables_t, ZSTD_match_t, ZSTD_optimal_t, ZSTD_MAX_NB_BLOCK_SPLITS,
    ZSTD_TARGETCBLOCKSIZE_MIN,
};
use crate::lib::compress::zstd_compress_internal::{
    repcodes_s, ZSTD_OptPrice_e, ZSTD_llt_literalLength, ZSTD_llt_matchLength, ZSTD_updateRep,
};
use crate::lib::compress::zstd_compress_literals::{
    ZSTD_compressRleLiteralsBlock, ZSTD_noCompressLiterals,
};
use crate::lib::compress::zstd_compress_sequences::{
    ZSTD_crossEntropyCost, ZSTD_encodeSequences, ZSTD_fseBitCost,
};
use crate::lib::zstd::{ZSTD_ParamSwitch_e, ZSTD_dictContentType_e};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_Sequence {
    pub offset: core::ffi::c_uint,
    pub litLength: core::ffi::c_uint,
    pub matchLength: core::ffi::c_uint,
    pub rep: core::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_blockSplitCtx {
    pub fullSeqStoreChunk: SeqStore_t,
    pub firstHalfSeqStore: SeqStore_t,
    pub secondHalfSeqStore: SeqStore_t,
    pub currSeqStore: SeqStore_t,
    pub nextSeqStore: SeqStore_t,
    pub partitions: [u32; ZSTD_MAX_NB_BLOCK_SPLITS],
    pub entropyMetadata: ZSTD_entropyCTablesMetadata_t,
}

pub type SymbolEncodingType_e = core::ffi::c_uint;
pub const set_repeat: SymbolEncodingType_e = 3;
pub const set_compressed: SymbolEncodingType_e = 2;
pub const set_rle: SymbolEncodingType_e = 1;
pub const set_basic: SymbolEncodingType_e = 0;
pub type ZSTD_prefixDict = ZSTD_prefixDict_s;

#[repr(C)]
pub struct ZSTD_prefixDict_s {
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
}

#[repr(C)]
pub struct ZSTD_localDict {
    pub dictBuffer: *mut core::ffi::c_void,
    pub dict: *const core::ffi::c_void,
    pub dictSize: size_t,
    pub dictContentType: ZSTD_dictContentType_e,
    pub cdict: *mut ZSTD_CDict,
}

pub type ZSTD_inBuffer = ZSTD_inBuffer_s;

#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const core::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}

pub type ZSTD_cStreamStage = core::ffi::c_uint;
pub const zcss_flush: ZSTD_cStreamStage = 2;
pub const zcss_load: ZSTD_cStreamStage = 1;
pub const zcss_init: ZSTD_cStreamStage = 0;
pub type ZSTD_buffered_policy_e = core::ffi::c_uint;
pub const ZSTDb_buffered: ZSTD_buffered_policy_e = 1;
pub const ZSTDb_not_buffered: ZSTD_buffered_policy_e = 0;

#[repr(C)]
pub struct ZSTD_blockState_t {
    pub prevCBlock: *mut ZSTD_compressedBlockState_t,
    pub nextCBlock: *mut ZSTD_compressedBlockState_t,
    pub matchState: ZSTD_MatchState_t,
}

#[repr(C)]
pub struct optState_t {
    pub litFreq: *mut core::ffi::c_uint,
    pub litLengthFreq: *mut core::ffi::c_uint,
    pub matchLengthFreq: *mut core::ffi::c_uint,
    pub offCodeFreq: *mut core::ffi::c_uint,
    pub matchTable: *mut ZSTD_match_t,
    pub priceTable: *mut ZSTD_optimal_t,
    pub litSum: u32,
    pub litLengthSum: u32,
    pub matchLengthSum: u32,
    pub offCodeSum: u32,
    pub litSumBasePrice: u32,
    pub litLengthSumBasePrice: u32,
    pub matchLengthSumBasePrice: u32,
    pub offCodeSumBasePrice: u32,
    pub priceType: ZSTD_OptPrice_e,
    pub symbolCosts: *const ZSTD_entropyCTables_t,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqCollector {
    pub collectSequences: core::ffi::c_int,
    pub seqStart: *mut ZSTD_Sequence,
    pub seqIndex: size_t,
    pub maxSequences: size_t,
}

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

#[repr(C)]
pub struct ZSTD_cwksp {
    pub workspace: *mut core::ffi::c_void,
    pub workspaceEnd: *mut core::ffi::c_void,
    pub objectEnd: *mut core::ffi::c_void,
    pub tableEnd: *mut core::ffi::c_void,
    pub tableValidEnd: *mut core::ffi::c_void,
    pub allocStart: *mut core::ffi::c_void,
    pub initOnceStart: *mut core::ffi::c_void,
    pub allocFailed: u8,
    pub workspaceOversizedDuration: core::ffi::c_int,
    pub phase: ZSTD_cwksp_alloc_phase_e,
    pub isStatic: ZSTD_cwksp_static_alloc_e,
}

pub type ZSTD_cwksp_static_alloc_e = core::ffi::c_uint;
pub const ZSTD_cwksp_static_alloc: ZSTD_cwksp_static_alloc_e = 1;
pub const ZSTD_cwksp_dynamic_alloc: ZSTD_cwksp_static_alloc_e = 0;
pub type ZSTD_cwksp_alloc_phase_e = core::ffi::c_uint;
pub const ZSTD_cwksp_alloc_buffers: ZSTD_cwksp_alloc_phase_e = 3;
pub const ZSTD_cwksp_alloc_aligned: ZSTD_cwksp_alloc_phase_e = 2;
pub const ZSTD_cwksp_alloc_aligned_init_once: ZSTD_cwksp_alloc_phase_e = 1;
pub const ZSTD_cwksp_alloc_objects: ZSTD_cwksp_alloc_phase_e = 0;

pub type ZSTD_sequenceProducer_F = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        *mut ZSTD_Sequence,
        size_t,
        *const core::ffi::c_void,
        size_t,
        *const core::ffi::c_void,
        size_t,
        core::ffi::c_int,
        size_t,
    ) -> size_t,
>;

pub type ZSTD_SequenceFormat_e = core::ffi::c_uint;
pub const ZSTD_sf_explicitBlockDelimiters: ZSTD_SequenceFormat_e = 1;
pub const ZSTD_sf_noBlockDelimiters: ZSTD_SequenceFormat_e = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_frameParameters {
    pub contentSizeFlag: core::ffi::c_int,
    pub checksumFlag: core::ffi::c_int,
    pub noDictIDFlag: core::ffi::c_int,
}

pub type ZSTD_compressionStage_e = core::ffi::c_uint;
pub const ZSTDcs_ending: ZSTD_compressionStage_e = 3;
pub const ZSTDcs_ongoing: ZSTD_compressionStage_e = 2;
pub const ZSTDcs_init: ZSTD_compressionStage_e = 1;
pub const ZSTDcs_created: ZSTD_compressionStage_e = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_SequenceLength {
    pub litLength: u32,
    pub matchLength: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EstimatedBlockSize {
    pub estLitSize: size_t,
    pub estBlockSize: size_t,
}

#[inline]
unsafe fn ZSTD_getSequenceLength(
    seqStore: *const SeqStore_t,
    seq: *const SeqDef,
) -> ZSTD_SequenceLength {
    let mut seqLen = ZSTD_SequenceLength {
        litLength: 0,
        matchLength: 0,
    };
    seqLen.litLength = (*seq).litLength as u32;
    seqLen.matchLength = ((*seq).mlBase as core::ffi::c_int + MINMATCH) as u32;
    if (*seqStore).longLengthPos
        == seq.offset_from((*seqStore).sequencesStart) as core::ffi::c_long as u32
    {
        if (*seqStore).longLengthType == ZSTD_llt_literalLength {
            seqLen.litLength = (seqLen.litLength).wrapping_add(0x10000 as core::ffi::c_int as u32);
        }
        if (*seqStore).longLengthType == ZSTD_llt_matchLength {
            seqLen.matchLength =
                (seqLen.matchLength).wrapping_add(0x10000 as core::ffi::c_int as u32);
        }
    }
    seqLen
}

#[inline]
unsafe fn ZSTD_noCompressBlock(
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastBlock: u32,
) -> size_t {
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_raw as core::ffi::c_int as u32) << 1)
        .wrapping_add((srcSize << 3) as u32);
    if srcSize.wrapping_add(ZSTD_blockHeaderSize) > dstCapacity {
        return Error::dstSize_tooSmall.to_error_code();
    }
    MEM_writeLE24(dst, cBlockHeader24);
    core::ptr::copy_nonoverlapping(
        src.cast::<u8>(),
        dst.byte_add(ZSTD_blockHeaderSize).cast::<u8>(),
        srcSize,
    );
    ZSTD_blockHeaderSize.wrapping_add(srcSize)
}

pub const ZSTD_BLOCKHEADERSIZE: core::ffi::c_int = 3;
static ZSTD_blockHeaderSize: size_t = ZSTD_BLOCKHEADERSIZE as size_t;
pub const LONGNBSEQ: core::ffi::c_int = 0x7f00 as core::ffi::c_int;
pub const STREAM_ACCUMULATOR_MIN_32: core::ffi::c_int = 25;
pub const STREAM_ACCUMULATOR_MIN_64: core::ffi::c_int = 57;

/// Compresses literals section for a sub-block.
/// When we have to write the Huffman table we will sometimes choose a header
/// size larger than necessary. This is because we have to pick the header size
/// before we know the table size + compressed size, so we have a bound on the
/// table size. If we guessed incorrectly, we fall back to uncompressed literals.
///
/// We write the header when writeEntropy=1 and set entropyWritten=1 when we succeeded
/// in writing the header, otherwise it is set to 0.
///
/// hufMetadata->hType has literals block type info.
///     If it is set_basic, all sub-blocks literals section will be Raw_Literals_Block.
///     If it is set_rle, all sub-blocks literals section will be RLE_Literals_Block.
///     If it is set_compressed, first sub-block's literals section will be Compressed_Literals_Block
///     If it is set_compressed, first sub-block's literals section will be Treeless_Literals_Block
///     and the following sub-blocks' literals sections will be Treeless_Literals_Block.
///
/// # Returns
///
/// - The compressed size of literals section of a sub-block
/// - Or 0 if unable to compress
/// - Or an error code
unsafe fn ZSTD_compressSubBlock_literal(
    hufTable: &[HUF_CElt; HUF_CTABLE_SIZE_ST(255)],
    hufMetadata: &ZSTD_hufCTablesMetadata_t,
    literals: *const u8,
    litSize: size_t,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    bmi2: core::ffi::c_int,
    writeEntropy: bool,
    entropyWritten: &mut bool,
) -> size_t {
    let header = (if writeEntropy { 200 } else { 0 }) as size_t;
    let lhSize = (3
        + (litSize >= ((1 << 10) as size_t).wrapping_sub(header)) as core::ffi::c_int
        + (litSize >= ((16 * (1 << 10)) as size_t).wrapping_sub(header)) as core::ffi::c_int)
        as size_t;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstSize);
    let mut op = ostart.add(lhSize);
    let singleStream = lhSize == 3;
    let hType = (if writeEntropy {
        hufMetadata.hType as core::ffi::c_uint
    } else {
        set_repeat
    }) as SymbolEncodingType_e;
    let mut cLitSize = 0usize;

    *entropyWritten = false;
    if litSize == 0 || hufMetadata.hType == set_basic {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const core::ffi::c_void,
            litSize,
        );
    } else if hufMetadata.hType == set_rle {
        return ZSTD_compressRleLiteralsBlock(
            dst,
            dstSize,
            literals as *const core::ffi::c_void,
            litSize,
        );
    }

    if writeEntropy && hufMetadata.hType == set_compressed {
        core::ptr::copy_nonoverlapping(
            hufMetadata.hufDesBuffer.as_ptr(),
            op,
            hufMetadata.hufDesSize,
        );
        op = op.add(hufMetadata.hufDesSize);
        cLitSize = cLitSize.wrapping_add(hufMetadata.hufDesSize);
    }

    let flags = if bmi2 != 0 {
        HUF_flags_bmi2 as core::ffi::c_int
    } else {
        0
    };
    let cSize = if singleStream {
        HUF_compress1X_usingCTable(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            literals as *const core::ffi::c_void,
            litSize,
            hufTable,
            flags,
        )
    } else {
        HUF_compress4X_usingCTable(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            literals as *const core::ffi::c_void,
            litSize,
            hufTable,
            flags,
        )
    };
    op = op.add(cSize);
    cLitSize = cLitSize.wrapping_add(cSize);
    if cSize == 0 || ERR_isError(cSize) {
        return 0;
    }
    // If we expand and we aren't writing a header then emit uncompressed.
    if !writeEntropy && cLitSize >= litSize {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const core::ffi::c_void,
            litSize,
        );
    }
    // If we are writing headers then allow expansion that doesn't change our header size.
    if lhSize
        < (3 + (cLitSize >= (1 << 10) as size_t) as core::ffi::c_int
            + (cLitSize >= (16 * (1 << 10)) as size_t) as core::ffi::c_int) as size_t
    {
        return ZSTD_noCompressLiterals(
            dst,
            dstSize,
            literals as *const core::ffi::c_void,
            litSize,
        );
    }

    // Build header
    match lhSize {
        3 => {
            // 2 - 2 - 10 - 10
            let lhc = (hType as core::ffi::c_uint)
                .wrapping_add(((!singleStream) as core::ffi::c_int as u32) << 2)
                .wrapping_add((litSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 14);
            MEM_writeLE24(ostart as *mut core::ffi::c_void, lhc);
        }
        4 => {
            // 2 - 2 - 14 - 14
            let lhc_0 = (hType as core::ffi::c_uint)
                .wrapping_add((2 << 2) as core::ffi::c_uint)
                .wrapping_add((litSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 18);
            MEM_writeLE32(ostart as *mut core::ffi::c_void, lhc_0);
        }
        5 => {
            // 2 - 2 - 18 - 18
            let lhc_1 = (hType as core::ffi::c_uint)
                .wrapping_add((3 << 2) as core::ffi::c_uint)
                .wrapping_add((litSize as u32) << 4)
                .wrapping_add((cLitSize as u32) << 22);
            MEM_writeLE32(ostart as *mut core::ffi::c_void, lhc_1);
            *ostart.add(4) = (cLitSize >> 10) as u8;
        }
        _ => {} // not possible : lhSize is {3,4,5}
    }
    *entropyWritten = true;
    op.offset_from_unsigned(ostart)
}

unsafe fn ZSTD_seqDecompressedSize(
    seqStore: &SeqStore_t,
    sequences: *const SeqDef,
    nbSeqs: size_t,
    litSize: size_t,
    lastSubBlock: core::ffi::c_int,
) -> size_t {
    let mut matchLengthSum = 0usize;
    let mut litLengthSum = 0usize;
    for n in 0..nbSeqs {
        let seqLen = ZSTD_getSequenceLength(seqStore, sequences.add(n));
        litLengthSum = litLengthSum.wrapping_add(seqLen.litLength as size_t);
        matchLengthSum = matchLengthSum.wrapping_add(seqLen.matchLength as size_t);
    }

    if lastSubBlock == 0 {
        assert!(litLengthSum == litSize);
    } else {
        assert!(litLengthSum <= litSize);
    }

    matchLengthSum.wrapping_add(litSize)
}

/// Compresses sequences section for a sub-block.
/// fseMetadata->llType, fseMetadata->ofType, and fseMetadata->mlType have
/// symbol compression modes for the super-block.
/// The first successfully compressed block will have these in its header.
/// We set entropyWritten=1 when we succeed in compressing the sequences.
/// The following sub-blocks will always have repeat mode.
///
/// # Returns
///
/// - The compressed size of sequences section of a sub-block
/// - Or 0 if it is unable to compress
/// - Or error code.
unsafe fn ZSTD_compressSubBlock_sequences(
    fseTables: &ZSTD_fseCTables_t,
    fseMetadata: &ZSTD_fseCTablesMetadata_t,
    sequences: *const SeqDef,
    nbSeq: size_t,
    llCode: *const u8,
    mlCode: *const u8,
    ofCode: *const u8,
    cctxParams: &ZSTD_CCtx_params,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    bmi2: core::ffi::c_int,
    writeEntropy: bool,
    entropyWritten: &mut bool,
) -> size_t {
    let longOffsets = cctxParams.cParams.windowLog
        > (if MEM_32bits() {
            STREAM_ACCUMULATOR_MIN_32
        } else {
            STREAM_ACCUMULATOR_MIN_64
        }) as u32;
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstCapacity);
    let mut op = ostart;
    let mut seqHead = core::ptr::null_mut::<u8>();

    *entropyWritten = false;
    // Sequences Header
    if (oend.offset_from(op) as core::ffi::c_long) < (3 + 1) as core::ffi::c_long {
        return Error::dstSize_tooSmall.to_error_code();
    }
    if nbSeq < 128 {
        *op = nbSeq as u8;
        op = op.add(1);
    } else if nbSeq < LONGNBSEQ as size_t {
        *op = (nbSeq >> 8).wrapping_add(0x80 as core::ffi::c_int as size_t) as u8;
        *op.add(1) = nbSeq as u8;
        op = op.add(2);
    } else {
        *op = 0xff as core::ffi::c_int as u8;
        MEM_writeLE16(
            op.add(1) as *mut core::ffi::c_void,
            nbSeq.wrapping_sub(LONGNBSEQ as size_t) as u16,
        );
        op = op.add(3);
    }
    if nbSeq == 0 {
        return op.offset_from_unsigned(ostart);
    }

    // seqHead : flags for FSE encoding type
    seqHead = op;
    op = op.add(1);

    if writeEntropy {
        let LLtype = fseMetadata.llType;
        let Offtype = fseMetadata.ofType;
        let MLtype = fseMetadata.mlType;
        *seqHead = (LLtype << 6)
            .wrapping_add(Offtype << 4)
            .wrapping_add(MLtype << 2) as u8;
        core::ptr::copy_nonoverlapping(
            fseMetadata.fseTablesBuffer.as_ptr(),
            op,
            fseMetadata.fseTablesSize,
        );
        op = op.add(fseMetadata.fseTablesSize);
    } else {
        let repeat = set_repeat as core::ffi::c_int as u32;
        *seqHead = (repeat << 6)
            .wrapping_add(repeat << 4)
            .wrapping_add(repeat << 2) as u8;
    }

    let bitstreamSize = ZSTD_encodeSequences(
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        &fseTables.matchlengthCTable,
        mlCode,
        &fseTables.offcodeCTable,
        ofCode,
        &fseTables.litlengthCTable,
        llCode,
        sequences,
        nbSeq,
        longOffsets,
        bmi2,
    );
    let err_code = bitstreamSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    op = op.add(bitstreamSize);
    // zstd versions <= 1.3.4 mistakenly report corruption when
    // FSE_readNCount() receives a buffer < 4 bytes.
    // Fixed by https://github.com/facebook/zstd/pull/1146.
    // This can happen when the last set_compressed table present is 2
    // bytes and the bitstream is only one byte.
    // In this exceedingly rare case, we will simply emit an uncompressed
    // block, since it isn't worth optimizing.
    if writeEntropy
        && fseMetadata.lastCountSize != 0
        && (fseMetadata.lastCountSize).wrapping_add(bitstreamSize) < 4
    {
        return 0;
    }

    // zstd versions <= 1.4.0 mistakenly report error when
    // sequences section body size is less than 3 bytes.
    // Fixed by https://github.com/facebook/zstd/pull/1664.
    // This can happen when the previous sequences section block is compressed
    // with rle mode and the current block's sequences section is compressed
    // with repeat mode where sequences section body size can be 1 byte.
    if (op.offset_from(seqHead) as core::ffi::c_long) < 4 {
        return 0;
    }

    *entropyWritten = true;
    op.offset_from_unsigned(ostart)
}

/// Compresses a single sub-block.
///
/// # Returns
///
/// - The compressed size of the sub-block
/// - Or 0 if it failed to compress.
unsafe fn ZSTD_compressSubBlock(
    entropy: &ZSTD_entropyCTables_t,
    entropyMetadata: &ZSTD_entropyCTablesMetadata_t,
    sequences: *const SeqDef,
    nbSeq: size_t,
    literals: *const u8,
    litSize: size_t,
    llCode: *const u8,
    mlCode: *const u8,
    ofCode: *const u8,
    cctxParams: &ZSTD_CCtx_params,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    bmi2: core::ffi::c_int,
    writeLitEntropy: bool,
    writeSeqEntropy: bool,
    litEntropyWritten: &mut bool,
    seqEntropyWritten: &mut bool,
    lastBlock: u32,
) -> size_t {
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstCapacity);
    let mut op = ostart.add(ZSTD_blockHeaderSize);

    let cLitSize = ZSTD_compressSubBlock_literal(
        &entropy.huf.CTable,
        &entropyMetadata.hufMetadata,
        literals,
        litSize,
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        bmi2,
        writeLitEntropy,
        litEntropyWritten,
    );
    let err_code = cLitSize;
    if ERR_isError(err_code) {
        return err_code;
    }
    if cLitSize == 0 {
        return 0;
    }
    op = op.add(cLitSize);

    let cSeqSize = ZSTD_compressSubBlock_sequences(
        &entropy.fse,
        &entropyMetadata.fseMetadata,
        sequences,
        nbSeq,
        llCode,
        mlCode,
        ofCode,
        cctxParams,
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        bmi2,
        writeSeqEntropy,
        seqEntropyWritten,
    );
    let err_code_0 = cSeqSize;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }
    if cSeqSize == 0 {
        return 0;
    }
    op = op.add(cSeqSize);

    // Write block header
    let cSize = (op.offset_from_unsigned(ostart)).wrapping_sub(ZSTD_blockHeaderSize);
    let cBlockHeader24 = lastBlock
        .wrapping_add((bt_compressed as core::ffi::c_int as u32) << 1)
        .wrapping_add((cSize << 3) as u32);
    MEM_writeLE24(ostart as *mut core::ffi::c_void, cBlockHeader24);

    op.offset_from_unsigned(ostart)
}

unsafe fn ZSTD_estimateSubBlockSize_literal(
    literals: *const u8,
    litSize: size_t,
    huf: &ZSTD_hufCTables_t,
    hufMetadata: &ZSTD_hufCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeEntropy: bool,
) -> size_t {
    let countWksp = workspace as *mut core::ffi::c_uint;
    let mut maxSymbolValue = 255;
    let literalSectionHeaderSize = 3; // Use hard coded size of 3 bytes

    if hufMetadata.hType == set_basic {
        return litSize;
    } else if hufMetadata.hType == set_rle {
        return 1;
    } else if hufMetadata.hType == set_compressed || hufMetadata.hType == set_repeat {
        let largest = HIST_count_wksp(
            countWksp,
            &mut maxSymbolValue,
            literals as *const core::ffi::c_void,
            litSize,
            workspace,
            wkspSize,
        );
        if ERR_isError(largest) {
            return litSize;
        }
        let mut cLitSizeEstimate =
            HUF_estimateCompressedSize((huf.CTable).as_ptr(), countWksp, maxSymbolValue);
        if writeEntropy {
            cLitSizeEstimate = cLitSizeEstimate.wrapping_add(hufMetadata.hufDesSize);
        }
        return cLitSizeEstimate.wrapping_add(literalSectionHeaderSize);
    }
    0
}

unsafe fn ZSTD_estimateSubBlockSize_symbolType(
    type_0: SymbolEncodingType_e,
    codeTable: *const u8,
    maxCode: core::ffi::c_uint,
    nbSeq: size_t,
    fseCTable: &[FSE_CTable],
    additionalBits: *const u8,
    defaultNorm: *const core::ffi::c_short,
    defaultNormLog: u32,
    defaultMax: u32,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let countWksp = workspace as *mut core::ffi::c_uint;
    let mut ctp = codeTable;
    let ctStart = ctp;
    let ctEnd = ctStart.add(nbSeq);
    let mut cSymbolTypeSizeEstimateInBits = 0;
    let mut max = maxCode;

    HIST_countFast_wksp(
        countWksp,
        &mut max,
        codeTable as *const core::ffi::c_void,
        nbSeq,
        workspace,
        wkspSize,
    );
    if type_0 == set_basic {
        // We selected this encoding type, so it must be valid.
        cSymbolTypeSizeEstimateInBits = if max <= defaultMax {
            ZSTD_crossEntropyCost(defaultNorm, defaultNormLog, countWksp, max)
        } else {
            Error::GENERIC.to_error_code()
        };
    } else if type_0 == set_rle {
        cSymbolTypeSizeEstimateInBits = 0;
    } else if type_0 == set_compressed || type_0 == set_repeat {
        cSymbolTypeSizeEstimateInBits = ZSTD_fseBitCost(fseCTable, countWksp, max);
    }
    if ERR_isError(cSymbolTypeSizeEstimateInBits) {
        return nbSeq * 10;
    }
    while ctp < ctEnd {
        if !additionalBits.is_null() {
            cSymbolTypeSizeEstimateInBits = cSymbolTypeSizeEstimateInBits
                .wrapping_add(*additionalBits.offset(*ctp as isize) as size_t);
        } else {
            // for offset, offset code is also the number of additional bits
            cSymbolTypeSizeEstimateInBits =
                cSymbolTypeSizeEstimateInBits.wrapping_add(*ctp as size_t);
        }
        ctp = ctp.add(1);
    }
    cSymbolTypeSizeEstimateInBits / 8
}

unsafe fn ZSTD_estimateSubBlockSize_sequences(
    ofCodeTable: *const u8,
    llCodeTable: *const u8,
    mlCodeTable: *const u8,
    nbSeq: size_t,
    fseTables: &ZSTD_fseCTables_t,
    fseMetadata: &ZSTD_fseCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeEntropy: bool,
) -> size_t {
    let sequencesSectionHeaderSize = 3; // Use hard coded size of 3 bytes
    let mut cSeqSizeEstimate = 0usize;
    if nbSeq == 0 {
        return sequencesSectionHeaderSize;
    }
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateSubBlockSize_symbolType(
        fseMetadata.ofType,
        ofCodeTable,
        MaxOff,
        nbSeq,
        &fseTables.offcodeCTable,
        core::ptr::null(),
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        DefaultMaxOff,
        workspace,
        wkspSize,
    ));
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateSubBlockSize_symbolType(
        fseMetadata.llType,
        llCodeTable,
        MaxLL,
        nbSeq,
        &fseTables.litlengthCTable,
        LL_bits.as_ptr(),
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        MaxLL,
        workspace,
        wkspSize,
    ));
    cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(ZSTD_estimateSubBlockSize_symbolType(
        fseMetadata.mlType,
        mlCodeTable,
        MaxML,
        nbSeq,
        &fseTables.matchlengthCTable,
        ML_bits.as_ptr(),
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        MaxML,
        workspace,
        wkspSize,
    ));
    if writeEntropy {
        cSeqSizeEstimate = cSeqSizeEstimate.wrapping_add(fseMetadata.fseTablesSize);
    }
    cSeqSizeEstimate.wrapping_add(sequencesSectionHeaderSize)
}

unsafe fn ZSTD_estimateSubBlockSize(
    literals: *const u8,
    litSize: size_t,
    ofCodeTable: *const u8,
    llCodeTable: *const u8,
    mlCodeTable: *const u8,
    nbSeq: size_t,
    entropy: &ZSTD_entropyCTables_t,
    entropyMetadata: &ZSTD_entropyCTablesMetadata_t,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
    writeLitEntropy: bool,
    writeSeqEntropy: bool,
) -> EstimatedBlockSize {
    let mut ebs = EstimatedBlockSize {
        estLitSize: 0,
        estBlockSize: 0,
    };
    ebs.estLitSize = ZSTD_estimateSubBlockSize_literal(
        literals,
        litSize,
        &entropy.huf,
        &entropyMetadata.hufMetadata,
        workspace,
        wkspSize,
        writeLitEntropy,
    );
    ebs.estBlockSize = ZSTD_estimateSubBlockSize_sequences(
        ofCodeTable,
        llCodeTable,
        mlCodeTable,
        nbSeq,
        &entropy.fse,
        &entropyMetadata.fseMetadata,
        workspace,
        wkspSize,
        writeSeqEntropy,
    );
    ebs.estBlockSize =
        (ebs.estBlockSize).wrapping_add((ebs.estLitSize).wrapping_add(ZSTD_blockHeaderSize));
    ebs
}

fn ZSTD_needSequenceEntropyTables(fseMetadata: &ZSTD_fseCTablesMetadata_t) -> bool {
    if fseMetadata.llType == set_compressed || fseMetadata.llType == set_rle {
        return true;
    }
    if fseMetadata.mlType == set_compressed || fseMetadata.mlType == set_rle {
        return true;
    }
    if fseMetadata.ofType == set_compressed || fseMetadata.ofType == set_rle {
        return true;
    }
    false
}

unsafe fn countLiterals(seqStore: &SeqStore_t, sp: *const SeqDef, seqCount: size_t) -> size_t {
    let mut total = 0usize;
    for n in 0..seqCount {
        total =
            total.wrapping_add((ZSTD_getSequenceLength(seqStore, sp.add(n))).litLength as size_t);
    }
    total
}

pub const BYTESCALE: core::ffi::c_int = 256;

unsafe fn sizeBlockSequences(
    sp: *const SeqDef,
    nbSeqs: size_t,
    targetBudget: size_t,
    avgLitCost: size_t,
    avgSeqCost: size_t,
    firstSubBlock: bool,
) -> size_t {
    let mut n: size_t = 0;
    let mut budget = 0usize;
    let mut inSize = 0;

    // entropy headers, generous estimate
    let headerSize = firstSubBlock as size_t * 120 * BYTESCALE as size_t;
    budget = budget.wrapping_add(headerSize);

    // first sequence => at least one sequence
    budget = budget.wrapping_add(((*sp).litLength as size_t * avgLitCost).wrapping_add(avgSeqCost));
    if budget > targetBudget {
        return 1;
    }
    inSize = ((*sp).litLength as core::ffi::c_int + ((*sp).mlBase as core::ffi::c_int + MINMATCH))
        as size_t;

    // loop over sequences
    n = 1;
    while n < nbSeqs {
        let currentCost = ((*sp.add(n)).litLength as size_t * avgLitCost).wrapping_add(avgSeqCost);
        budget = budget.wrapping_add(currentCost);
        inSize = inSize.wrapping_add(
            ((*sp.add(n)).litLength as core::ffi::c_int
                + ((*sp.add(n)).mlBase as core::ffi::c_int + MINMATCH)) as size_t,
        );
        // stop when sub-block budget is reached,
        // though continue to expand until the sub-block is deemed compressible
        if budget > targetBudget && budget < inSize * BYTESCALE as size_t {
            break;
        }
        n = n.wrapping_add(1);
    }

    n
}

/// Breaks super-block into multiple sub-blocks and compresses them.
/// Entropy will be written into the first block.
/// The following blocks use repeat_mode to compress.
/// Sub-blocks are all compressed, except the last one when beneficial.
///
/// # Returns
///
/// - The compressed size of the super block (which features multiple ZSTD blocks)
/// - or 0 if it failed to compress.
unsafe fn ZSTD_compressSubBlock_multi(
    seqStorePtr: &SeqStore_t,
    prevCBlock: *const ZSTD_compressedBlockState_t,
    nextCBlock: *mut ZSTD_compressedBlockState_t,
    entropyMetadata: &ZSTD_entropyCTablesMetadata_t,
    cctxParams: &ZSTD_CCtx_params,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    bmi2: core::ffi::c_int,
    lastBlock: u32,
    workspace: *mut core::ffi::c_void,
    wkspSize: size_t,
) -> size_t {
    let sstart: *const SeqDef = seqStorePtr.sequencesStart;
    let send: *const SeqDef = seqStorePtr.sequences;
    let mut sp = sstart; // tracks progresses within seqStorePtr->sequences
    let nbSeqs = send.offset_from_unsigned(sstart);
    let lstart: *const u8 = seqStorePtr.litStart;
    let lend: *const u8 = seqStorePtr.lit;
    let mut lp = lstart;
    let nbLiterals = lend.offset_from_unsigned(lstart);
    let mut ip = src as *const u8;
    let iend = ip.add(srcSize);
    let ostart = dst as *mut u8;
    let oend = ostart.add(dstCapacity);
    let mut op = ostart;
    let mut llCodePtr: *const u8 = seqStorePtr.llCode;
    let mut mlCodePtr: *const u8 = seqStorePtr.mlCode;
    let mut ofCodePtr: *const u8 = seqStorePtr.ofCode;
    let minTarget = ZSTD_TARGETCBLOCKSIZE_MIN as size_t; // enforce minimum size, to reduce undesirable side effects
    let targetCBlockSize = minTarget.max(cctxParams.targetCBlockSize);
    let mut writeLitEntropy = entropyMetadata.hufMetadata.hType == set_compressed;
    let mut writeSeqEntropy = true;

    // let's start by a general estimation for the full block
    if nbSeqs > 0 {
        let ebs = ZSTD_estimateSubBlockSize(
            lp,
            nbLiterals,
            ofCodePtr,
            llCodePtr,
            mlCodePtr,
            nbSeqs,
            &(*nextCBlock).entropy,
            entropyMetadata,
            workspace,
            wkspSize,
            writeLitEntropy,
            writeSeqEntropy,
        );

        // quick estimation
        let avgLitCost = (ebs.estLitSize * BYTESCALE as size_t)
            .checked_div(nbLiterals)
            .unwrap_or(BYTESCALE as size_t);

        let avgSeqCost =
            (ebs.estBlockSize).wrapping_sub(ebs.estLitSize) * BYTESCALE as size_t / nbSeqs;

        let nbSubBlocks =
            ((ebs.estBlockSize).wrapping_add(targetCBlockSize / 2) / targetCBlockSize).max(1);
        let mut avgBlockBudget: size_t = 0;
        let mut blockBudgetSupp = 0;
        avgBlockBudget = ebs.estBlockSize * BYTESCALE as size_t / nbSubBlocks;
        // simplification: if estimates states that the full superblock doesn't compress, just bail out immediately
        // this will result in the production of a single uncompressed block covering srcSize.
        if ebs.estBlockSize > srcSize {
            return 0;
        }

        // compress and write sub-blocks
        for n in 0..nbSubBlocks.wrapping_sub(1) {
            // determine nb of sequences for current sub-block + nbLiterals from next sequence
            let seqCount = sizeBlockSequences(
                sp,
                send.offset_from_unsigned(sp),
                avgBlockBudget.wrapping_add(blockBudgetSupp),
                avgLitCost,
                avgSeqCost,
                n == 0,
            );
            // if reached last sequence : break to last sub-block (simplification)
            if sp.add(seqCount) == send {
                break;
            }

            // compress sub-block
            let mut litEntropyWritten = false;
            let mut seqEntropyWritten = false;
            let litSize = countLiterals(seqStorePtr, sp, seqCount);
            let decompressedSize = ZSTD_seqDecompressedSize(seqStorePtr, sp, seqCount, litSize, 0);
            let cSize = ZSTD_compressSubBlock(
                &(*nextCBlock).entropy,
                entropyMetadata,
                sp,
                seqCount,
                lp,
                litSize,
                llCodePtr,
                mlCodePtr,
                ofCodePtr,
                cctxParams,
                op as *mut core::ffi::c_void,
                oend.offset_from_unsigned(op),
                bmi2,
                writeLitEntropy,
                writeSeqEntropy,
                &mut litEntropyWritten,
                &mut seqEntropyWritten,
                0,
            );
            let err_code = cSize;
            if ERR_isError(err_code) {
                return err_code;
            }

            // check compressibility, update state components
            if cSize > 0 && cSize < decompressedSize {
                ip = ip.add(decompressedSize);
                lp = lp.add(litSize);
                op = op.add(cSize);
                llCodePtr = llCodePtr.add(seqCount);
                mlCodePtr = mlCodePtr.add(seqCount);
                ofCodePtr = ofCodePtr.add(seqCount);
                // Entropy only needs to be written once
                if litEntropyWritten {
                    writeLitEntropy = false;
                }
                if seqEntropyWritten {
                    writeSeqEntropy = false;
                }
                sp = sp.add(seqCount);
                blockBudgetSupp = 0;
            }
            // otherwise : do not compress yet, coalesce current sub-block with following one
        }
    }

    // write last block
    let mut litEntropyWritten = false;
    let mut seqEntropyWritten = false;
    let litSize_0 = lend.offset_from_unsigned(lp);
    let seqCount_0 = send.offset_from_unsigned(sp);
    let decompressedSize_0 = ZSTD_seqDecompressedSize(seqStorePtr, sp, seqCount_0, litSize_0, 1);
    let cSize_0 = ZSTD_compressSubBlock(
        &(*nextCBlock).entropy,
        entropyMetadata,
        sp,
        seqCount_0,
        lp,
        litSize_0,
        llCodePtr,
        mlCodePtr,
        ofCodePtr,
        cctxParams,
        op as *mut core::ffi::c_void,
        oend.offset_from_unsigned(op),
        bmi2,
        writeLitEntropy,
        writeSeqEntropy,
        &mut litEntropyWritten,
        &mut seqEntropyWritten,
        lastBlock,
    );
    let err_code_0 = cSize_0;
    if ERR_isError(err_code_0) {
        return err_code_0;
    }

    // update pointers, the nb of literals borrowed from next sequence must be preserved
    if cSize_0 > 0 && cSize_0 < decompressedSize_0 {
        ip = ip.add(decompressedSize_0);
        lp = lp.add(litSize_0);
        op = op.add(cSize_0);
        llCodePtr = llCodePtr.add(seqCount_0);
        mlCodePtr = mlCodePtr.add(seqCount_0);
        ofCodePtr = ofCodePtr.add(seqCount_0);
        // Entropy only needs to be written once
        if litEntropyWritten {
            writeLitEntropy = false;
        }
        if seqEntropyWritten {
            writeSeqEntropy = false;
        }
        sp = sp.add(seqCount_0);
    }

    if writeLitEntropy {
        core::ptr::copy_nonoverlapping(
            &raw const (*prevCBlock).entropy.huf,
            &raw mut (*nextCBlock).entropy.huf,
            1,
        );
    }
    if writeSeqEntropy && ZSTD_needSequenceEntropyTables(&entropyMetadata.fseMetadata) {
        // If we haven't written our entropy tables, then we've violated our contract and
        // must emit an uncompressed block.
        return 0;
    }

    if ip < iend {
        // some data left: last part of the block sent uncompressed
        let rSize = iend.offset_from_unsigned(ip);
        let cSize_1 = ZSTD_noCompressBlock(
            op as *mut core::ffi::c_void,
            oend.offset_from_unsigned(op),
            ip as *const core::ffi::c_void,
            rSize,
            lastBlock,
        );
        let err_code_1 = cSize_1;
        if ERR_isError(err_code_1) {
            return err_code_1;
        }
        op = op.add(cSize_1);

        // We have to regenerate the repcodes because we've skipped some sequences
        if sp < send {
            let mut seq = core::ptr::null::<SeqDef>();
            let mut rep = repcodes_s { rep: [0; 3] };
            rep.rep = (*prevCBlock).rep;
            seq = sstart;
            while seq < sp {
                ZSTD_updateRep(
                    &mut rep.rep,
                    (*seq).offBase,
                    ((ZSTD_getSequenceLength(seqStorePtr, seq)).litLength == 0) as core::ffi::c_int
                        as u32,
                );
                seq = seq.add(1);
            }
            (*nextCBlock).rep = rep.rep;
        }
    }

    op.offset_from_unsigned(ostart)
}

pub unsafe fn ZSTD_compressSuperBlock(
    zc: *mut ZSTD_CCtx,
    dst: *mut core::ffi::c_void,
    dstCapacity: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    lastBlock: core::ffi::c_uint,
) -> size_t {
    let mut entropyMetadata = ZSTD_entropyCTablesMetadata_t {
        hufMetadata: ZSTD_hufCTablesMetadata_t {
            hType: set_basic,
            hufDesBuffer: [0; ZSTD_MAX_HUF_HEADER_SIZE],
            hufDesSize: 0,
        },
        fseMetadata: ZSTD_fseCTablesMetadata_t {
            llType: set_basic,
            ofType: set_basic,
            mlType: set_basic,
            fseTablesBuffer: [0; 133],
            fseTablesSize: 0,
            lastCountSize: 0,
        },
    };

    let err_code = ZSTD_buildBlockEntropyStats(
        &(*zc).seqStore,
        &(*(*zc).blockState.prevCBlock).entropy,
        &mut (*(*zc).blockState.nextCBlock).entropy,
        &(*zc).appliedParams,
        &mut entropyMetadata,
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
    );
    if ERR_isError(err_code) {
        return err_code;
    }

    ZSTD_compressSubBlock_multi(
        &(*zc).seqStore,
        (*zc).blockState.prevCBlock,
        (*zc).blockState.nextCBlock,
        &entropyMetadata,
        &(*zc).appliedParams,
        dst,
        dstCapacity,
        src,
        srcSize,
        (*zc).bmi2,
        lastBlock,
        (*zc).tmpWorkspace,
        (*zc).tmpWkspSize,
    )
}
