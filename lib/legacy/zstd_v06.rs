use ::libc;
extern "C" {
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memmove(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn ERR_getErrorString(code: ERR_enum) -> *const std::ffi::c_char;
}
pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
pub type ZSTDv06_DCtx = ZSTDv06_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv06_DCtx_s {
    pub LLTable: [FSEv06_DTable; 513],
    pub OffTable: [FSEv06_DTable; 257],
    pub MLTable: [FSEv06_DTable; 513],
    pub hufTableX4: [std::ffi::c_uint; 4097],
    pub previousDstEnd: *const std::ffi::c_void,
    pub base: *const std::ffi::c_void,
    pub vBase: *const std::ffi::c_void,
    pub dictEnd: *const std::ffi::c_void,
    pub expected: size_t,
    pub headerSize: size_t,
    pub fParams: ZSTDv06_frameParams,
    pub bType: blockType_t,
    pub stage: ZSTDv06_dStage,
    pub flagRepeatTable: U32,
    pub litPtr: *const BYTE,
    pub litSize: size_t,
    pub litBuffer: [BYTE; 131080],
    pub headerBuffer: [BYTE; 13],
}
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = std::ffi::c_uchar;
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = std::ffi::c_uint;
pub type ZSTDv06_dStage = std::ffi::c_uint;
pub const ZSTDds_decompressBlock: ZSTDv06_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTDv06_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTDv06_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTDv06_dStage = 0;
pub type blockType_t = std::ffi::c_uint;
pub const bt_end: blockType_t = 3;
pub const bt_rle: blockType_t = 2;
pub const bt_raw: blockType_t = 1;
pub const bt_compressed: blockType_t = 0;
pub type ZSTDv06_frameParams = ZSTDv06_frameParams_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTDv06_frameParams_s {
    pub frameContentSize: std::ffi::c_ulonglong,
    pub windowLog: std::ffi::c_uint,
}
pub type FSEv06_DTable = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockProperties_t {
    pub blockType: blockType_t,
    pub origSize: U32,
}
pub const ZSTD_error_srcSize_wrong: ERR_enum = 72;
pub const ZSTD_error_maxCode: ERR_enum = 120;
pub const ZSTD_error_GENERIC: ERR_enum = 1;
pub const ZSTD_error_dstSize_tooSmall: ERR_enum = 70;
pub const ZSTD_error_corruption_detected: ERR_enum = 20;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq_t {
    pub litLength: size_t,
    pub matchLength: size_t,
    pub offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seqState_t {
    pub DStream: BITv06_DStream_t,
    pub stateLL: FSEv06_DState_t,
    pub stateOffb: FSEv06_DState_t,
    pub stateML: FSEv06_DState_t,
    pub prevOffset: [size_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv06_DState_t {
    pub state: size_t,
    pub table: *const std::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BITv06_DStream_t {
    pub bitContainer: size_t,
    pub bitsConsumed: std::ffi::c_uint,
    pub ptr: *const std::ffi::c_char,
    pub start: *const std::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv06_decode_t {
    pub newState: std::ffi::c_ushort,
    pub symbol: std::ffi::c_uchar,
    pub nbBits: std::ffi::c_uchar,
}
pub type BITv06_DStream_status = std::ffi::c_uint;
pub const BITv06_DStream_overflow: BITv06_DStream_status = 3;
pub const BITv06_DStream_completed: BITv06_DStream_status = 2;
pub const BITv06_DStream_endOfBuffer: BITv06_DStream_status = 1;
pub const BITv06_DStream_unfinished: BITv06_DStream_status = 0;
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = std::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: U32,
    pub c: [BYTE; 4],
}
pub type U16 = uint16_t;
pub type uint16_t = __uint16_t;
pub type __uint16_t = std::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSEv06_DTableHeader {
    pub tableLog: U16,
    pub fastMode: U16,
}
pub type S16 = int16_t;
pub type int16_t = __int16_t;
pub type __int16_t = std::ffi::c_short;
pub const ZSTD_error_maxSymbolValue_tooSmall: ERR_enum = 48;
pub const ZSTD_error_tableLog_tooLarge: ERR_enum = 44;
pub const ZSTD_error_maxSymbolValue_tooLarge: ERR_enum = 46;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv06_DEltX4 {
    pub sequence: U16,
    pub nbBits: BYTE,
    pub length: BYTE,
}
pub const ZSTD_error_dictionary_corrupted: ERR_enum = 30;
pub type decompressionAlgo = Option<
    unsafe extern "C" fn(*mut std::ffi::c_void, size_t, *const std::ffi::c_void, size_t) -> size_t,
>;
pub type rankVal_t = [[U32; 17]; 16];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: BYTE,
    pub weight: BYTE,
}
pub type DTable_max_t = [U32; 4097];
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUFv06_static_assert: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUFv06_DEltX2 {
    pub byte: BYTE,
    pub nbBits: BYTE,
}
pub type C2RustUnnamed_1 = std::ffi::c_uint;
pub const HUFv06_static_assert_0: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: U32,
    pub decode256Time: U32,
}
pub const ZSTD_error_frameParameter_unsupported: ERR_enum = 14;
pub const ZSTD_error_prefix_unknown: ERR_enum = 10;
pub const ZSTD_error_memory_allocation: ERR_enum = 64;
pub type ZSTD_ErrorCode = ERR_enum;
pub type ERR_enum = std::ffi::c_uint;
pub const ZSTD_error_externalSequences_invalid: ERR_enum = 107;
pub const ZSTD_error_sequenceProducer_failed: ERR_enum = 106;
pub const ZSTD_error_srcBuffer_wrong: ERR_enum = 105;
pub const ZSTD_error_dstBuffer_wrong: ERR_enum = 104;
pub const ZSTD_error_seekableIO: ERR_enum = 102;
pub const ZSTD_error_frameIndex_tooLarge: ERR_enum = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: ERR_enum = 82;
pub const ZSTD_error_noForwardProgress_destFull: ERR_enum = 80;
pub const ZSTD_error_dstBuffer_null: ERR_enum = 74;
pub const ZSTD_error_workSpace_tooSmall: ERR_enum = 66;
pub const ZSTD_error_init_missing: ERR_enum = 62;
pub const ZSTD_error_stage_wrong: ERR_enum = 60;
pub const ZSTD_error_stabilityCondition_notRespected: ERR_enum = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: ERR_enum = 49;
pub const ZSTD_error_parameter_outOfBound: ERR_enum = 42;
pub const ZSTD_error_parameter_combination_unsupported: ERR_enum = 41;
pub const ZSTD_error_parameter_unsupported: ERR_enum = 40;
pub const ZSTD_error_dictionaryCreation_failed: ERR_enum = 34;
pub const ZSTD_error_dictionary_wrong: ERR_enum = 32;
pub const ZSTD_error_literals_headerWrong: ERR_enum = 24;
pub const ZSTD_error_checksum_wrong: ERR_enum = 22;
pub const ZSTD_error_frameParameter_windowTooLarge: ERR_enum = 16;
pub const ZSTD_error_version_unsupported: ERR_enum = 12;
pub const ZSTD_error_no_error: ERR_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZBUFFv06_DCtx_s {
    pub zd: *mut ZSTDv06_DCtx,
    pub fParams: ZSTDv06_frameParams,
    pub stage: ZBUFFv06_dStage,
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub blockSize: size_t,
    pub headerBuffer: [BYTE; 13],
    pub lhSize: size_t,
}
pub type ZBUFFv06_dStage = std::ffi::c_uint;
pub const ZBUFFds_flush: ZBUFFv06_dStage = 4;
pub const ZBUFFds_load: ZBUFFv06_dStage = 3;
pub const ZBUFFds_read: ZBUFFv06_dStage = 2;
pub const ZBUFFds_loadHeader: ZBUFFv06_dStage = 1;
pub const ZBUFFds_init: ZBUFFv06_dStage = 0;
pub type ZBUFFv06_DCtx = ZBUFFv06_DCtx_s;
pub const ZSTDv06_MAGICNUMBER: std::ffi::c_uint = 0xfd2fb526 as std::ffi::c_uint;
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
unsafe extern "C" fn ERR_getErrorCode(mut code: size_t) -> ERR_enum {
    if ERR_isError(code) == 0 {
        return ZSTD_error_no_error;
    }
    (0 as std::ffi::c_int as size_t).wrapping_sub(code) as ERR_enum
}
unsafe extern "C" fn ERR_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorString(ERR_getErrorCode(code))
}
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 8 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    let one = C2RustUnnamed {
        u: 1 as std::ffi::c_int as U32,
    };
    *(one.c).as_ptr().offset(0 as std::ffi::c_int as isize) as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_read16(mut memPtr: *const std::ffi::c_void) -> U16 {
    let mut val: U16 = 0;
    memcpy(
        &mut val as *mut U16 as *mut std::ffi::c_void,
        memPtr,
        ::core::mem::size_of::<U16>() as std::ffi::c_ulong,
    );
    val
}
#[inline]
unsafe extern "C" fn MEM_read32(mut memPtr: *const std::ffi::c_void) -> U32 {
    let mut val: U32 = 0;
    memcpy(
        &mut val as *mut U32 as *mut std::ffi::c_void,
        memPtr,
        ::core::mem::size_of::<U32>() as std::ffi::c_ulong,
    );
    val
}
#[inline]
unsafe extern "C" fn MEM_read64(mut memPtr: *const std::ffi::c_void) -> U64 {
    let mut val: U64 = 0;
    memcpy(
        &mut val as *mut U64 as *mut std::ffi::c_void,
        memPtr,
        ::core::mem::size_of::<U64>() as std::ffi::c_ulong,
    );
    val
}
#[inline]
unsafe extern "C" fn MEM_write16(mut memPtr: *mut std::ffi::c_void, mut value: U16) {
    memcpy(
        memPtr,
        &mut value as *mut U16 as *const std::ffi::c_void,
        ::core::mem::size_of::<U16>() as std::ffi::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    in_0 << 24 as std::ffi::c_int & 0xff000000 as std::ffi::c_uint
        | in_0 << 8 as std::ffi::c_int & 0xff0000 as std::ffi::c_int as U32
        | in_0 >> 8 as std::ffi::c_int & 0xff00 as std::ffi::c_int as U32
        | in_0 >> 24 as std::ffi::c_int & 0xff as std::ffi::c_int as U32
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: U64) -> U64 {
    ((in_0 << 56 as std::ffi::c_int) as std::ffi::c_ulonglong
        & 0xff00000000000000 as std::ffi::c_ulonglong
        | (in_0 << 40 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff000000000000 as std::ffi::c_ulonglong
        | (in_0 << 24 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff0000000000 as std::ffi::c_ulonglong
        | (in_0 << 8 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff00000000 as std::ffi::c_ulonglong
        | (in_0 >> 8 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff000000 as std::ffi::c_ulonglong
        | (in_0 >> 24 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff0000 as std::ffi::c_ulonglong
        | (in_0 >> 40 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff00 as std::ffi::c_ulonglong
        | (in_0 >> 56 as std::ffi::c_int) as std::ffi::c_ulonglong & 0xff as std::ffi::c_ulonglong)
        as U64
}
#[inline]
unsafe extern "C" fn MEM_readLE16(mut memPtr: *const std::ffi::c_void) -> U16 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read16(memPtr);
    } else {
        let mut p = memPtr as *const BYTE;
        (*p.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
            + ((*p.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                << 8 as std::ffi::c_int)) as U16
    }
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
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const std::ffi::c_void) -> U32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr);
    } else {
        MEM_swap32(MEM_read32(memPtr))
    }
}
#[inline]
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const std::ffi::c_void) -> U64 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read64(memPtr);
    } else {
        MEM_swap64(MEM_read64(memPtr))
    }
}
#[inline]
unsafe extern "C" fn MEM_readLEST(mut memPtr: *const std::ffi::c_void) -> size_t {
    if MEM_32bits() != 0 {
        return MEM_readLE32(memPtr) as size_t;
    } else {
        MEM_readLE64(memPtr)
    }
}
pub const ZSTDv06_FRAMEHEADERSIZE_MAX: std::ffi::c_int = 13 as std::ffi::c_int;
static mut ZSTDv06_frameHeaderSize_min: size_t = 5 as std::ffi::c_int as size_t;
static mut ZSTDv06_frameHeaderSize_max: size_t = ZSTDv06_FRAMEHEADERSIZE_MAX as size_t;
pub const ZSTDv06_BLOCKSIZE_MAX: std::ffi::c_int = 128 as std::ffi::c_int * 1024 as std::ffi::c_int;
pub const ZSTDv06_DICT_MAGIC: std::ffi::c_uint = 0xec30a436 as std::ffi::c_uint;
pub const ZSTDv06_REP_NUM: std::ffi::c_int = 3 as std::ffi::c_int;
pub const ZSTDv06_REP_INIT: std::ffi::c_int = 3 as std::ffi::c_int;
pub const ZSTDv06_REP_MOVE: std::ffi::c_int = ZSTDv06_REP_NUM - 1 as std::ffi::c_int;
pub const ZSTDv06_WINDOWLOG_ABSOLUTEMIN: std::ffi::c_int = 12 as std::ffi::c_int;
static mut ZSTDv06_fcs_fieldSize: [size_t; 4] = [
    0 as std::ffi::c_int as size_t,
    1 as std::ffi::c_int as size_t,
    2 as std::ffi::c_int as size_t,
    8 as std::ffi::c_int as size_t,
];
pub const ZSTDv06_BLOCKHEADERSIZE: std::ffi::c_int = 3 as std::ffi::c_int;
static mut ZSTDv06_blockHeaderSize: size_t = ZSTDv06_BLOCKHEADERSIZE as size_t;
pub const MIN_SEQUENCES_SIZE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const MIN_CBLOCK_SIZE: std::ffi::c_int =
    1 as std::ffi::c_int + 1 as std::ffi::c_int + MIN_SEQUENCES_SIZE;
pub const ZSTD_HUFFDTABLE_CAPACITY_LOG: std::ffi::c_int = 12 as std::ffi::c_int;
pub const IS_HUF: std::ffi::c_int = 0;
pub const IS_PCH: std::ffi::c_int = 1;
pub const IS_RAW: std::ffi::c_int = 2;
pub const IS_RLE: std::ffi::c_int = 3;
pub const LONGNBSEQ: std::ffi::c_int = 0x7f00 as std::ffi::c_int;
pub const MINMATCH: std::ffi::c_int = 3 as std::ffi::c_int;
pub const REPCODE_STARTVALUE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const MaxML: std::ffi::c_int = 52 as std::ffi::c_int;
pub const MaxLL: std::ffi::c_int = 35 as std::ffi::c_int;
pub const MaxOff: std::ffi::c_int = 28 as std::ffi::c_int;
pub const MLFSELog: std::ffi::c_int = 9 as std::ffi::c_int;
pub const LLFSELog: std::ffi::c_int = 9 as std::ffi::c_int;
pub const OffFSELog: std::ffi::c_int = 8 as std::ffi::c_int;
pub const FSEv06_ENCODING_RAW: std::ffi::c_int = 0;
pub const FSEv06_ENCODING_RLE: std::ffi::c_int = 1;
pub const FSEv06_ENCODING_STATIC: std::ffi::c_int = 2;
pub const FSEv06_ENCODING_DYNAMIC: std::ffi::c_int = 3;
pub const ZSTD_CONTENTSIZE_ERROR: std::ffi::c_ulonglong =
    (0 as std::ffi::c_ulonglong).wrapping_sub(2 as std::ffi::c_int as std::ffi::c_ulonglong);
static mut LL_bits: [U32; 36] = [
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    2 as std::ffi::c_int as U32,
    2 as std::ffi::c_int as U32,
    3 as std::ffi::c_int as U32,
    3 as std::ffi::c_int as U32,
    4 as std::ffi::c_int as U32,
    6 as std::ffi::c_int as U32,
    7 as std::ffi::c_int as U32,
    8 as std::ffi::c_int as U32,
    9 as std::ffi::c_int as U32,
    10 as std::ffi::c_int as U32,
    11 as std::ffi::c_int as U32,
    12 as std::ffi::c_int as U32,
    13 as std::ffi::c_int as U32,
    14 as std::ffi::c_int as U32,
    15 as std::ffi::c_int as U32,
    16 as std::ffi::c_int as U32,
];
static mut LL_defaultNorm: [S16; 36] = [
    4 as std::ffi::c_int as S16,
    3 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    3 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
];
static mut LL_defaultNormLog: U32 = 6 as std::ffi::c_int as U32;
static mut ML_bits: [U32; 53] = [
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    0 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    1 as std::ffi::c_int as U32,
    2 as std::ffi::c_int as U32,
    2 as std::ffi::c_int as U32,
    3 as std::ffi::c_int as U32,
    3 as std::ffi::c_int as U32,
    4 as std::ffi::c_int as U32,
    4 as std::ffi::c_int as U32,
    5 as std::ffi::c_int as U32,
    7 as std::ffi::c_int as U32,
    8 as std::ffi::c_int as U32,
    9 as std::ffi::c_int as U32,
    10 as std::ffi::c_int as U32,
    11 as std::ffi::c_int as U32,
    12 as std::ffi::c_int as U32,
    13 as std::ffi::c_int as U32,
    14 as std::ffi::c_int as U32,
    15 as std::ffi::c_int as U32,
    16 as std::ffi::c_int as U32,
];
static mut ML_defaultNorm: [S16; 53] = [
    1 as std::ffi::c_int as S16,
    4 as std::ffi::c_int as S16,
    3 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
];
static mut ML_defaultNormLog: U32 = 6 as std::ffi::c_int as U32;
static mut OF_defaultNorm: [S16; 29] = [
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    2 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    1 as std::ffi::c_int as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
    -(1 as std::ffi::c_int) as S16,
];
static mut OF_defaultNormLog: U32 = 5 as std::ffi::c_int as U32;
unsafe extern "C" fn ZSTDv06_copy8(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
) {
    memcpy(dst, src, 8 as std::ffi::c_int as std::ffi::c_ulong);
}
pub const WILDCOPY_OVERLENGTH: std::ffi::c_int = 8 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn ZSTDv06_wildcopy(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut length: ptrdiff_t,
) {
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let oend = op.offset(length as isize);
    loop {
        ZSTDv06_copy8(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
        op = op.offset(8 as std::ffi::c_int as isize);
        ip = ip.offset(8 as std::ffi::c_int as isize);
        if (op >= oend) {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn BITv06_highbit32(mut val: U32) -> std::ffi::c_uint {
    (val.leading_zeros() as i32 ^ 31 as std::ffi::c_int) as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn BITv06_initDStream(
    mut bitD: *mut BITv06_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 as std::ffi::c_int as size_t {
        memset(
            bitD as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            ::core::mem::size_of::<BITv06_DStream_t>() as std::ffi::c_ulong,
        );
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if srcSize >= ::core::mem::size_of::<size_t>() as std::ffi::c_ulong {
        (*bitD).start = srcBuffer as *const std::ffi::c_char;
        (*bitD).ptr = (srcBuffer as *const std::ffi::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        let lastByte = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
        if lastByte as std::ffi::c_int == 0 as std::ffi::c_int {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed = (8 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_sub(BITv06_highbit32(lastByte as U32));
    } else {
        (*bitD).start = srcBuffer as *const std::ffi::c_char;
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const BYTE) as size_t;
        let mut current_block_20: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                    (*(srcBuffer as *const BYTE).offset(6 as std::ffi::c_int as isize) as size_t)
                        << (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                            .wrapping_sub(16 as std::ffi::c_int as std::ffi::c_ulong),
                );
                current_block_20 = 11220331375136032509;
            }
            6 => {
                current_block_20 = 11220331375136032509;
            }
            5 => {
                current_block_20 = 10901957826175510184;
            }
            4 => {
                current_block_20 = 3201895511516222412;
            }
            3 => {
                current_block_20 = 12760952191649157579;
            }
            2 => {
                current_block_20 = 13935781298497728377;
            }
            _ => {
                current_block_20 = 5689001924483802034;
            }
        }
        if current_block_20 == 11220331375136032509 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(5 as std::ffi::c_int as isize) as size_t)
                    << (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                        .wrapping_sub(24 as std::ffi::c_int as std::ffi::c_ulong),
            );
            current_block_20 = 10901957826175510184;
        }
        if current_block_20 == 10901957826175510184 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(4 as std::ffi::c_int as isize) as size_t)
                    << (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                        .wrapping_sub(32 as std::ffi::c_int as std::ffi::c_ulong),
            );
            current_block_20 = 3201895511516222412;
        }
        if current_block_20 == 3201895511516222412 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(3 as std::ffi::c_int as isize) as size_t)
                    << 24 as std::ffi::c_int,
            );
            current_block_20 = 12760952191649157579;
        }
        if current_block_20 == 12760952191649157579 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(2 as std::ffi::c_int as isize) as size_t)
                    << 16 as std::ffi::c_int,
            );
            current_block_20 = 13935781298497728377;
        }
        if current_block_20 == 13935781298497728377 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const BYTE).offset(1 as std::ffi::c_int as isize) as size_t)
                    << 8 as std::ffi::c_int,
            );
        }
        let lastByte_0 = *(srcBuffer as *const BYTE)
            .offset(srcSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
        if lastByte_0 as std::ffi::c_int == 0 as std::ffi::c_int {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed = (8 as std::ffi::c_int as std::ffi::c_uint)
            .wrapping_sub(BITv06_highbit32(lastByte_0 as U32));
        (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(
            (::core::mem::size_of::<size_t>() as std::ffi::c_ulong).wrapping_sub(srcSize) as U32
                * 8 as std::ffi::c_int as U32,
        );
    }
    srcSize
}
#[inline]
unsafe extern "C" fn BITv06_lookBits(mut bitD: *const BITv06_DStream_t, mut nbBits: U32) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as U32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> 1 as std::ffi::c_int
        >> (bitMask.wrapping_sub(nbBits) & bitMask)
}
#[inline]
unsafe extern "C" fn BITv06_lookBitsFast(
    mut bitD: *const BITv06_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let bitMask = (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as U32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & bitMask)
        >> (bitMask
            .wrapping_add(1 as std::ffi::c_int as U32)
            .wrapping_sub(nbBits)
            & bitMask)
}
#[inline]
unsafe extern "C" fn BITv06_skipBits(mut bitD: *mut BITv06_DStream_t, mut nbBits: U32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline]
unsafe extern "C" fn BITv06_readBits(mut bitD: *mut BITv06_DStream_t, mut nbBits: U32) -> size_t {
    let value = BITv06_lookBits(bitD, nbBits);
    BITv06_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BITv06_readBitsFast(
    mut bitD: *mut BITv06_DStream_t,
    mut nbBits: U32,
) -> size_t {
    let value = BITv06_lookBitsFast(bitD, nbBits);
    BITv06_skipBits(bitD, nbBits);
    value
}
#[inline]
unsafe extern "C" fn BITv06_reloadDStream(
    mut bitD: *mut BITv06_DStream_t,
) -> BITv06_DStream_status {
    if (*bitD).bitsConsumed as std::ffi::c_ulong
        > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
    {
        return BITv06_DStream_overflow;
    }
    if (*bitD).ptr
        >= ((*bitD).start).offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize)
    {
        (*bitD).ptr =
            ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3 as std::ffi::c_int) as isize));
        (*bitD).bitsConsumed &= 7 as std::ffi::c_int as std::ffi::c_uint;
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
        return BITv06_DStream_unfinished;
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as std::ffi::c_ulong)
            < (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            return BITv06_DStream_endOfBuffer;
        }
        return BITv06_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3 as std::ffi::c_int;
    let mut result = BITv06_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as U32;
        result = BITv06_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed =
        ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8 as std::ffi::c_int as U32);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void);
    result
}
#[inline]
unsafe extern "C" fn BITv06_endOfDStream(mut DStream: *const BITv06_DStream_t) -> std::ffi::c_uint {
    ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as std::ffi::c_ulong
            == (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong))
        as std::ffi::c_int as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn FSEv06_initDState(
    mut DStatePtr: *mut FSEv06_DState_t,
    mut bitD: *mut BITv06_DStream_t,
    mut dt: *const FSEv06_DTable,
) {
    let mut ptr = dt as *const std::ffi::c_void;
    let DTableH = ptr as *const FSEv06_DTableHeader;
    (*DStatePtr).state = BITv06_readBits(bitD, (*DTableH).tableLog as std::ffi::c_uint);
    BITv06_reloadDStream(bitD);
    (*DStatePtr).table = dt.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
}
#[inline]
unsafe extern "C" fn FSEv06_peekSymbol(mut DStatePtr: *const FSEv06_DState_t) -> BYTE {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).offset((*DStatePtr).state as isize);
    DInfo.symbol
}
#[inline]
unsafe extern "C" fn FSEv06_updateState(
    mut DStatePtr: *mut FSEv06_DState_t,
    mut bitD: *mut BITv06_DStream_t,
) {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let lowBits = BITv06_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
}
#[inline]
unsafe extern "C" fn FSEv06_decodeSymbol(
    mut DStatePtr: *mut FSEv06_DState_t,
    mut bitD: *mut BITv06_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let symbol = DInfo.symbol;
    let lowBits = BITv06_readBits(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
#[inline]
unsafe extern "C" fn FSEv06_decodeSymbolFast(
    mut DStatePtr: *mut FSEv06_DState_t,
    mut bitD: *mut BITv06_DStream_t,
) -> std::ffi::c_uchar {
    let DInfo = *((*DStatePtr).table as *const FSEv06_decode_t).offset((*DStatePtr).state as isize);
    let nbBits = DInfo.nbBits as U32;
    let symbol = DInfo.symbol;
    let lowBits = BITv06_readBitsFast(bitD, nbBits);
    (*DStatePtr).state = (DInfo.newState as size_t).wrapping_add(lowBits);
    symbol
}
pub const FSEv06_MAX_MEMORY_USAGE: std::ffi::c_int = 14 as std::ffi::c_int;
pub const FSEv06_MAX_SYMBOL_VALUE: std::ffi::c_int = 255 as std::ffi::c_int;
pub const FSEv06_MAX_TABLELOG: std::ffi::c_int = FSEv06_MAX_MEMORY_USAGE - 2 as std::ffi::c_int;
pub const FSEv06_MIN_TABLELOG: std::ffi::c_int = 5 as std::ffi::c_int;
pub const FSEv06_TABLELOG_ABSOLUTE_MAX: std::ffi::c_int = 15 as std::ffi::c_int;
#[export_name = "FSEv06_isError"]
pub unsafe extern "C" fn FSEv06_isError_1(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(code)
}
unsafe extern "C" fn HUFv06_isError(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}
unsafe extern "C" fn FSEv06_abs(mut a: std::ffi::c_short) -> std::ffi::c_short {
    ((if (a as std::ffi::c_int) < 0 as std::ffi::c_int {
        -(a as std::ffi::c_int)
    } else {
        a as std::ffi::c_int
    }) as std::ffi::c_short)
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_readNCount(
    mut normalizedCounter: *mut std::ffi::c_short,
    mut maxSVPtr: *mut std::ffi::c_uint,
    mut tableLogPtr: *mut std::ffi::c_uint,
    mut headerBuffer: *const std::ffi::c_void,
    mut hbSize: size_t,
) -> size_t {
    let istart = headerBuffer as *const BYTE;
    let iend = istart.offset(hbSize as isize);
    let mut ip = istart;
    let mut nbBits: std::ffi::c_int = 0;
    let mut remaining: std::ffi::c_int = 0;
    let mut threshold: std::ffi::c_int = 0;
    let mut bitStream: U32 = 0;
    let mut bitCount: std::ffi::c_int = 0;
    let mut charnum = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut previous0 = 0 as std::ffi::c_int;
    if hbSize < 4 as std::ffi::c_int as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    bitStream = MEM_readLE32(ip as *const std::ffi::c_void);
    nbBits = (bitStream & 0xf as std::ffi::c_int as U32).wrapping_add(FSEv06_MIN_TABLELOG as U32)
        as std::ffi::c_int;
    if nbBits > FSEv06_TABLELOG_ABSOLUTE_MAX {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    bitStream >>= 4 as std::ffi::c_int;
    bitCount = 4 as std::ffi::c_int;
    *tableLogPtr = nbBits as std::ffi::c_uint;
    remaining = ((1 as std::ffi::c_int) << nbBits) + 1 as std::ffi::c_int;
    threshold = (1 as std::ffi::c_int) << nbBits;
    nbBits += 1;
    nbBits;
    while remaining > 1 as std::ffi::c_int && charnum <= *maxSVPtr {
        if previous0 != 0 {
            let mut n0 = charnum;
            while bitStream & 0xffff as std::ffi::c_int as U32 == 0xffff as std::ffi::c_int as U32 {
                n0 = n0.wrapping_add(24 as std::ffi::c_int as std::ffi::c_uint);
                if ip < iend.offset(-(5 as std::ffi::c_int as isize)) {
                    ip = ip.offset(2 as std::ffi::c_int as isize);
                    bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
                } else {
                    bitStream >>= 16 as std::ffi::c_int;
                    bitCount += 16 as std::ffi::c_int;
                }
            }
            while bitStream & 3 as std::ffi::c_int as U32 == 3 as std::ffi::c_int as U32 {
                n0 = n0.wrapping_add(3 as std::ffi::c_int as std::ffi::c_uint);
                bitStream >>= 2 as std::ffi::c_int;
                bitCount += 2 as std::ffi::c_int;
            }
            n0 = n0.wrapping_add(bitStream & 3 as std::ffi::c_int as U32);
            bitCount += 2 as std::ffi::c_int;
            if n0 > *maxSVPtr {
                return -(ZSTD_error_maxSymbolValue_tooSmall as std::ffi::c_int) as size_t;
            }
            while charnum < n0 {
                let fresh0 = charnum;
                charnum = charnum.wrapping_add(1);
                *normalizedCounter.offset(fresh0 as isize) =
                    0 as std::ffi::c_int as std::ffi::c_short;
            }
            if ip <= iend.offset(-(7 as std::ffi::c_int as isize))
                || ip.offset((bitCount >> 3 as std::ffi::c_int) as isize)
                    <= iend.offset(-(4 as std::ffi::c_int as isize))
            {
                ip = ip.offset((bitCount >> 3 as std::ffi::c_int) as isize);
                bitCount &= 7 as std::ffi::c_int;
                bitStream = MEM_readLE32(ip as *const std::ffi::c_void) >> bitCount;
            } else {
                bitStream >>= 2 as std::ffi::c_int;
            }
        }
        let max = (2 as std::ffi::c_int * threshold - 1 as std::ffi::c_int - remaining)
            as std::ffi::c_short;
        let mut count: std::ffi::c_short = 0;
        if (bitStream & (threshold - 1 as std::ffi::c_int) as U32) < max as U32 {
            count = (bitStream & (threshold - 1 as std::ffi::c_int) as U32) as std::ffi::c_short;
            bitCount += nbBits - 1 as std::ffi::c_int;
        } else {
            count = (bitStream & (2 as std::ffi::c_int * threshold - 1 as std::ffi::c_int) as U32)
                as std::ffi::c_short;
            if count as std::ffi::c_int >= threshold {
                count = (count as std::ffi::c_int - max as std::ffi::c_int) as std::ffi::c_short;
            }
            bitCount += nbBits;
        }
        count -= 1;
        count;
        remaining -= FSEv06_abs(count) as std::ffi::c_int;
        let fresh1 = charnum;
        charnum = charnum.wrapping_add(1);
        *normalizedCounter.offset(fresh1 as isize) = count;
        previous0 = (count == 0) as std::ffi::c_int;
        while remaining < threshold {
            nbBits -= 1;
            nbBits;
            threshold >>= 1 as std::ffi::c_int;
        }
        if ip <= iend.offset(-(7 as std::ffi::c_int as isize))
            || ip.offset((bitCount >> 3 as std::ffi::c_int) as isize)
                <= iend.offset(-(4 as std::ffi::c_int as isize))
        {
            ip = ip.offset((bitCount >> 3 as std::ffi::c_int) as isize);
            bitCount &= 7 as std::ffi::c_int;
        } else {
            bitCount -= (8 as std::ffi::c_int as std::ffi::c_long
                * iend
                    .offset(-(4 as std::ffi::c_int as isize))
                    .offset_from(ip) as std::ffi::c_long)
                as std::ffi::c_int;
            ip = iend.offset(-(4 as std::ffi::c_int as isize));
        }
        bitStream =
            MEM_readLE32(ip as *const std::ffi::c_void) >> (bitCount & 31 as std::ffi::c_int);
    }
    if remaining != 1 as std::ffi::c_int {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    *maxSVPtr = charnum.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    ip = ip.offset(((bitCount + 7 as std::ffi::c_int) >> 3 as std::ffi::c_int) as isize);
    if ip.offset_from(istart) as std::ffi::c_long as size_t > hbSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip.offset_from(istart) as std::ffi::c_long as size_t
}
pub const FSEv06_isError_0: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
#[no_mangle]
pub unsafe extern "C" fn FSEv06_createDTable(mut tableLog: std::ffi::c_uint) -> *mut FSEv06_DTable {
    if tableLog > FSEv06_TABLELOG_ABSOLUTE_MAX as std::ffi::c_uint {
        tableLog = FSEv06_TABLELOG_ABSOLUTE_MAX as std::ffi::c_uint;
    }
    malloc(
        ((1 as std::ffi::c_int + ((1 as std::ffi::c_int) << tableLog)) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<U32>() as std::ffi::c_ulong),
    ) as *mut FSEv06_DTable
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_freeDTable(mut dt: *mut FSEv06_DTable) {
    free(dt as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_buildDTable(
    mut dt: *mut FSEv06_DTable,
    mut normalizedCounter: *const std::ffi::c_short,
    mut maxSymbolValue: std::ffi::c_uint,
    mut tableLog: std::ffi::c_uint,
) -> size_t {
    let tdPtr = dt.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let tableDecode = tdPtr as *mut FSEv06_decode_t;
    let mut symbolNext: [U16; 256] = [0; 256];
    let maxSV1 = maxSymbolValue.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let tableSize = ((1 as std::ffi::c_int) << tableLog) as U32;
    let mut highThreshold = tableSize.wrapping_sub(1 as std::ffi::c_int as U32);
    if maxSymbolValue > FSEv06_MAX_SYMBOL_VALUE as std::ffi::c_uint {
        return -(ZSTD_error_maxSymbolValue_tooLarge as std::ffi::c_int) as size_t;
    }
    if tableLog > FSEv06_MAX_TABLELOG as std::ffi::c_uint {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    let mut DTableH = FSEv06_DTableHeader {
        tableLog: 0,
        fastMode: 0,
    };
    DTableH.tableLog = tableLog as U16;
    DTableH.fastMode = 1 as std::ffi::c_int as U16;
    let largeLimit = ((1 as std::ffi::c_int)
        << tableLog.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint))
        as S16;
    let mut s: U32 = 0;
    s = 0 as std::ffi::c_int as U32;
    while s < maxSV1 {
        if *normalizedCounter.offset(s as isize) as std::ffi::c_int == -(1 as std::ffi::c_int) {
            let fresh2 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            (*tableDecode.offset(fresh2 as isize)).symbol = s as BYTE;
            *symbolNext.as_mut_ptr().offset(s as isize) = 1 as std::ffi::c_int as U16;
        } else {
            if *normalizedCounter.offset(s as isize) as std::ffi::c_int
                >= largeLimit as std::ffi::c_int
            {
                DTableH.fastMode = 0 as std::ffi::c_int as U16;
            }
            *symbolNext.as_mut_ptr().offset(s as isize) =
                *normalizedCounter.offset(s as isize) as U16;
        }
        s = s.wrapping_add(1);
        s;
    }
    memcpy(
        dt as *mut std::ffi::c_void,
        &mut DTableH as *mut FSEv06_DTableHeader as *const std::ffi::c_void,
        ::core::mem::size_of::<FSEv06_DTableHeader>() as std::ffi::c_ulong,
    );
    let tableMask = tableSize.wrapping_sub(1 as std::ffi::c_int as U32);
    let step = (tableSize >> 1 as std::ffi::c_int)
        .wrapping_add(tableSize >> 3 as std::ffi::c_int)
        .wrapping_add(3 as std::ffi::c_int as U32);
    let mut s_0: U32 = 0;
    let mut position = 0 as std::ffi::c_int as U32;
    s_0 = 0 as std::ffi::c_int as U32;
    while s_0 < maxSV1 {
        let mut i: std::ffi::c_int = 0;
        i = 0 as std::ffi::c_int;
        while i < *normalizedCounter.offset(s_0 as isize) as std::ffi::c_int {
            (*tableDecode.offset(position as isize)).symbol = s_0 as BYTE;
            position = position.wrapping_add(step) & tableMask;
            while position > highThreshold {
                position = position.wrapping_add(step) & tableMask;
            }
            i += 1;
            i;
        }
        s_0 = s_0.wrapping_add(1);
        s_0;
    }
    if position != 0 as std::ffi::c_int as U32 {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    let mut u: U32 = 0;
    u = 0 as std::ffi::c_int as U32;
    while u < tableSize {
        let symbol = (*tableDecode.offset(u as isize)).symbol;
        let fresh3 = &mut (*symbolNext.as_mut_ptr().offset(symbol as isize));
        let fresh4 = *fresh3;
        *fresh3 = (*fresh3).wrapping_add(1);
        let mut nextState = fresh4;
        (*tableDecode.offset(u as isize)).nbBits =
            tableLog.wrapping_sub(BITv06_highbit32(nextState as U32)) as BYTE;
        (*tableDecode.offset(u as isize)).newState = (((nextState as std::ffi::c_int)
            << (*tableDecode.offset(u as isize)).nbBits as std::ffi::c_int)
            as U32)
            .wrapping_sub(tableSize) as U16;
        u = u.wrapping_add(1);
        u;
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_buildDTable_rle(
    mut dt: *mut FSEv06_DTable,
    mut symbolValue: BYTE,
) -> size_t {
    let mut ptr = dt as *mut std::ffi::c_void;
    let DTableH = ptr as *mut FSEv06_DTableHeader;
    let mut dPtr = dt.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let cell = dPtr as *mut FSEv06_decode_t;
    (*DTableH).tableLog = 0 as std::ffi::c_int as U16;
    (*DTableH).fastMode = 0 as std::ffi::c_int as U16;
    (*cell).newState = 0 as std::ffi::c_int as std::ffi::c_ushort;
    (*cell).symbol = symbolValue;
    (*cell).nbBits = 0 as std::ffi::c_int as std::ffi::c_uchar;
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_buildDTable_raw(
    mut dt: *mut FSEv06_DTable,
    mut nbBits: std::ffi::c_uint,
) -> size_t {
    let mut ptr = dt as *mut std::ffi::c_void;
    let DTableH = ptr as *mut FSEv06_DTableHeader;
    let mut dPtr = dt.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let dinfo = dPtr as *mut FSEv06_decode_t;
    let tableSize = ((1 as std::ffi::c_int) << nbBits) as std::ffi::c_uint;
    let tableMask = tableSize.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let maxSV1 = tableMask.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    let mut s: std::ffi::c_uint = 0;
    if nbBits < 1 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    (*DTableH).tableLog = nbBits as U16;
    (*DTableH).fastMode = 1 as std::ffi::c_int as U16;
    s = 0 as std::ffi::c_int as std::ffi::c_uint;
    while s < maxSV1 {
        (*dinfo.offset(s as isize)).newState = 0 as std::ffi::c_int as std::ffi::c_ushort;
        (*dinfo.offset(s as isize)).symbol = s as BYTE;
        (*dinfo.offset(s as isize)).nbBits = nbBits as BYTE;
        s = s.wrapping_add(1);
        s;
    }
    0 as std::ffi::c_int as size_t
}
#[inline(always)]
unsafe extern "C" fn FSEv06_decompress_usingDTable_generic(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSEv06_DTable,
    fast: std::ffi::c_uint,
) -> size_t {
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let omax = op.offset(maxDstSize as isize);
    let olimit = omax.offset(-(3 as std::ffi::c_int as isize));
    let mut bitD = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut state1 = FSEv06_DState_t {
        state: 0,
        table: std::ptr::null::<std::ffi::c_void>(),
    };
    let mut state2 = FSEv06_DState_t {
        state: 0,
        table: std::ptr::null::<std::ffi::c_void>(),
    };
    let errorCode = BITv06_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv06_initDState(&mut state1, &mut bitD, dt);
    FSEv06_initDState(&mut state2, &mut bitD, dt);
    while BITv06_reloadDStream(&mut bitD) as std::ffi::c_uint
        == BITv06_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && op < olimit
    {
        *op.offset(0 as std::ffi::c_int as isize) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if (FSEv06_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            BITv06_reloadDStream(&mut bitD);
        }
        *op.offset(1 as std::ffi::c_int as isize) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if (FSEv06_MAX_TABLELOG * 4 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
            && BITv06_reloadDStream(&mut bitD) as std::ffi::c_uint
                > BITv06_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
            {
                op = op.offset(2 as std::ffi::c_int as isize);
                break;
            }
        *op.offset(2 as std::ffi::c_int as isize) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if (FSEv06_MAX_TABLELOG * 2 as std::ffi::c_int + 7 as std::ffi::c_int) as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            BITv06_reloadDStream(&mut bitD);
        }
        *op.offset(3 as std::ffi::c_int as isize) = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        op = op.offset(4 as std::ffi::c_int as isize);
    }
    loop {
        if op > omax.offset(-(2 as std::ffi::c_int as isize)) {
            return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
        }
        let fresh5 = op;
        op = op.offset(1);
        *fresh5 = (if fast != 0 {
            FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
        } else {
            FSEv06_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
        }) as BYTE;
        if BITv06_reloadDStream(&mut bitD) as std::ffi::c_uint
            == BITv06_DStream_overflow as std::ffi::c_int as std::ffi::c_uint
        {
            let fresh6 = op;
            op = op.offset(1);
            *fresh6 = (if fast != 0 {
                FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
            } else {
                FSEv06_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
            }) as BYTE;
            break;
        } else {
            if op > omax.offset(-(2 as std::ffi::c_int as isize)) {
                return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
            }
            let fresh7 = op;
            op = op.offset(1);
            *fresh7 = (if fast != 0 {
                FSEv06_decodeSymbolFast(&mut state2, &mut bitD) as std::ffi::c_int
            } else {
                FSEv06_decodeSymbol(&mut state2, &mut bitD) as std::ffi::c_int
            }) as BYTE;
            if (BITv06_reloadDStream(&mut bitD) as std::ffi::c_uint != BITv06_DStream_overflow as std::ffi::c_int as std::ffi::c_uint)
            {
                continue;
            }
            let fresh8 = op;
            op = op.offset(1);
            *fresh8 = (if fast != 0 {
                FSEv06_decodeSymbolFast(&mut state1, &mut bitD) as std::ffi::c_int
            } else {
                FSEv06_decodeSymbol(&mut state1, &mut bitD) as std::ffi::c_int
            }) as BYTE;
            break;
        }
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_decompress_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut originalSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut dt: *const FSEv06_DTable,
) -> size_t {
    let mut ptr = dt as *const std::ffi::c_void;
    let mut DTableH = ptr as *const FSEv06_DTableHeader;
    let fastMode = (*DTableH).fastMode as U32;
    if fastMode != 0 {
        return FSEv06_decompress_usingDTable_generic(
            dst,
            originalSize,
            cSrc,
            cSrcSize,
            dt,
            1 as std::ffi::c_int as std::ffi::c_uint,
        );
    }
    FSEv06_decompress_usingDTable_generic(
        dst,
        originalSize,
        cSrc,
        cSrcSize,
        dt,
        0 as std::ffi::c_int as std::ffi::c_uint,
    )
}
#[no_mangle]
pub unsafe extern "C" fn FSEv06_decompress(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let istart = cSrc as *const BYTE;
    let mut ip = istart;
    let mut counting: [std::ffi::c_short; 256] = [0; 256];
    let mut dt: DTable_max_t = [0; 4097];
    let mut tableLog: std::ffi::c_uint = 0;
    let mut maxSymbolValue = FSEv06_MAX_SYMBOL_VALUE as std::ffi::c_uint;
    if cSrcSize < 2 as std::ffi::c_int as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let NCountLength = FSEv06_readNCount(
        counting.as_mut_ptr(),
        &mut maxSymbolValue,
        &mut tableLog,
        istart as *const std::ffi::c_void,
        cSrcSize,
    );
    if ERR_isError(NCountLength) != 0 {
        return NCountLength;
    }
    if NCountLength >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(NCountLength as isize);
    cSrcSize = cSrcSize.wrapping_sub(NCountLength);
    let errorCode = FSEv06_buildDTable(
        dt.as_mut_ptr(),
        counting.as_mut_ptr(),
        maxSymbolValue,
        tableLog,
    );
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    FSEv06_decompress_usingDTable(
        dst,
        maxDstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dt.as_mut_ptr(),
    )
}
pub const HUFv06_ABSOLUTEMAX_TABLELOG: std::ffi::c_int = 16 as std::ffi::c_int;
pub const HUFv06_MAX_TABLELOG: std::ffi::c_int = 12 as std::ffi::c_int;
pub const HUFv06_MAX_SYMBOL_VALUE: std::ffi::c_int = 255 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn HUFv06_readStats(
    mut huffWeight: *mut BYTE,
    mut hwSize: size_t,
    mut rankStats: *mut U32,
    mut nbSymbolsPtr: *mut U32,
    mut tableLogPtr: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightTotal: U32 = 0;
    let mut ip = src as *const BYTE;
    let mut iSize: size_t = 0;
    let mut oSize: size_t = 0;
    if srcSize == 0 {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    iSize = *ip.offset(0 as std::ffi::c_int as isize) as size_t;
    if iSize >= 128 as std::ffi::c_int as size_t {
        if iSize >= 242 as std::ffi::c_int as size_t {
            static mut l: [U32; 14] = [
                1 as std::ffi::c_int as U32,
                2 as std::ffi::c_int as U32,
                3 as std::ffi::c_int as U32,
                4 as std::ffi::c_int as U32,
                7 as std::ffi::c_int as U32,
                8 as std::ffi::c_int as U32,
                15 as std::ffi::c_int as U32,
                16 as std::ffi::c_int as U32,
                31 as std::ffi::c_int as U32,
                32 as std::ffi::c_int as U32,
                63 as std::ffi::c_int as U32,
                64 as std::ffi::c_int as U32,
                127 as std::ffi::c_int as U32,
                128 as std::ffi::c_int as U32,
            ];
            oSize = *l
                .as_mut_ptr()
                .offset(iSize.wrapping_sub(242 as std::ffi::c_int as size_t) as isize)
                as size_t;
            memset(
                huffWeight as *mut std::ffi::c_void,
                1 as std::ffi::c_int,
                hwSize,
            );
            iSize = 0 as std::ffi::c_int as size_t;
        } else {
            oSize = iSize.wrapping_sub(127 as std::ffi::c_int as size_t);
            iSize =
                oSize.wrapping_add(1 as std::ffi::c_int as size_t) / 2 as std::ffi::c_int as size_t;
            if iSize.wrapping_add(1 as std::ffi::c_int as size_t) > srcSize {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            if oSize >= hwSize {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            ip = ip.offset(1 as std::ffi::c_int as isize);
            let mut n: U32 = 0;
            n = 0 as std::ffi::c_int as U32;
            while (n as size_t) < oSize {
                *huffWeight.offset(n as isize) =
                    (*ip.offset((n / 2 as std::ffi::c_int as U32) as isize) as std::ffi::c_int
                        >> 4 as std::ffi::c_int) as BYTE;
                *huffWeight.offset(n.wrapping_add(1 as std::ffi::c_int as U32) as isize) =
                    (*ip.offset((n / 2 as std::ffi::c_int as U32) as isize) as std::ffi::c_int
                        & 15 as std::ffi::c_int) as BYTE;
                n = n.wrapping_add(2 as std::ffi::c_int as U32);
            }
        }
    } else {
        if iSize.wrapping_add(1 as std::ffi::c_int as size_t) > srcSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        oSize = FSEv06_decompress(
            huffWeight as *mut std::ffi::c_void,
            hwSize.wrapping_sub(1 as std::ffi::c_int as size_t),
            ip.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void,
            iSize,
        );
        if ERR_isError(oSize) != 0 {
            return oSize;
        }
    }
    memset(
        rankStats as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ((HUFv06_ABSOLUTEMAX_TABLELOG + 1 as std::ffi::c_int) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<U32>() as std::ffi::c_ulong),
    );
    weightTotal = 0 as std::ffi::c_int as U32;
    let mut n_0: U32 = 0;
    n_0 = 0 as std::ffi::c_int as U32;
    while (n_0 as size_t) < oSize {
        if *huffWeight.offset(n_0 as isize) as std::ffi::c_int >= HUFv06_ABSOLUTEMAX_TABLELOG {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        let fresh9 = &mut (*rankStats.offset(*huffWeight.offset(n_0 as isize) as isize));
        *fresh9 = (*fresh9).wrapping_add(1);
        *fresh9;
        weightTotal = weightTotal.wrapping_add(
            ((1 as std::ffi::c_int) << *huffWeight.offset(n_0 as isize) as std::ffi::c_int
                >> 1 as std::ffi::c_int) as U32,
        );
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    if weightTotal == 0 as std::ffi::c_int as U32 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let tableLog =
        (BITv06_highbit32(weightTotal)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    if tableLog > HUFv06_ABSOLUTEMAX_TABLELOG as U32 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *tableLogPtr = tableLog;
    let total = ((1 as std::ffi::c_int) << tableLog) as U32;
    let rest = total.wrapping_sub(weightTotal);
    let verif = ((1 as std::ffi::c_int) << BITv06_highbit32(rest)) as U32;
    let lastWeight =
        (BITv06_highbit32(rest)).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
    if verif != rest {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *huffWeight.offset(oSize as isize) = lastWeight as BYTE;
    let fresh10 = &mut (*rankStats.offset(lastWeight as isize));
    *fresh10 = (*fresh10).wrapping_add(1);
    *fresh10;
    if *rankStats.offset(1 as std::ffi::c_int as isize) < 2 as std::ffi::c_int as U32
        || *rankStats.offset(1 as std::ffi::c_int as isize) & 1 as std::ffi::c_int as U32 != 0
    {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    *nbSymbolsPtr = oSize.wrapping_add(1 as std::ffi::c_int as size_t) as U32;
    iSize.wrapping_add(1 as std::ffi::c_int as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_readDTableX2(
    mut DTable: *mut U16,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut huffWeight: [BYTE; 256] = [0; 256];
    let mut rankVal: [U32; 17] = [0; 17];
    let mut tableLog = 0 as std::ffi::c_int as U32;
    let mut iSize: size_t = 0;
    let mut nbSymbols = 0 as std::ffi::c_int as U32;
    let mut n: U32 = 0;
    let mut nextRankStart: U32 = 0;
    let dtPtr = DTable.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let dt = dtPtr as *mut HUFv06_DEltX2;
    iSize = HUFv06_readStats(
        huffWeight.as_mut_ptr(),
        (HUFv06_MAX_SYMBOL_VALUE + 1 as std::ffi::c_int) as size_t,
        rankVal.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv06_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > *DTable.offset(0 as std::ffi::c_int as isize) as U32 {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    *DTable.offset(0 as std::ffi::c_int as isize) = tableLog as U16;
    nextRankStart = 0 as std::ffi::c_int as U32;
    n = 1 as std::ffi::c_int as U32;
    while n < tableLog.wrapping_add(1 as std::ffi::c_int as U32) {
        let mut current = nextRankStart;
        nextRankStart = nextRankStart.wrapping_add(
            *rankVal.as_mut_ptr().offset(n as isize) << n.wrapping_sub(1 as std::ffi::c_int as U32),
        );
        *rankVal.as_mut_ptr().offset(n as isize) = current;
        n = n.wrapping_add(1);
        n;
    }
    n = 0 as std::ffi::c_int as U32;
    while n < nbSymbols {
        let w = *huffWeight.as_mut_ptr().offset(n as isize) as U32;
        let length = ((1 as std::ffi::c_int) << w >> 1 as std::ffi::c_int) as U32;
        let mut i: U32 = 0;
        let mut D = HUFv06_DEltX2 { byte: 0, nbBits: 0 };
        D.byte = n as BYTE;
        D.nbBits = tableLog
            .wrapping_add(1 as std::ffi::c_int as U32)
            .wrapping_sub(w) as BYTE;
        i = *rankVal.as_mut_ptr().offset(w as isize);
        while i < (*rankVal.as_mut_ptr().offset(w as isize)).wrapping_add(length) {
            *dt.offset(i as isize) = D;
            i = i.wrapping_add(1);
            i;
        }
        let fresh11 = &mut (*rankVal.as_mut_ptr().offset(w as isize));
        *fresh11 = (*fresh11).wrapping_add(length);
        n = n.wrapping_add(1);
        n;
    }
    iSize
}
unsafe extern "C" fn HUFv06_decodeSymbolX2(
    mut Dstream: *mut BITv06_DStream_t,
    mut dt: *const HUFv06_DEltX2,
    dtLog: U32,
) -> BYTE {
    let val = BITv06_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    BITv06_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as U32);
    c
}
#[inline]
unsafe extern "C" fn HUFv06_decodeStreamX2(
    mut p: *mut BYTE,
    bitDPtr: *mut BITv06_DStream_t,
    pEnd: *mut BYTE,
    dt: *const HUFv06_DEltX2,
    dtLog: U32,
) -> size_t {
    let pStart = p;
    while BITv06_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv06_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p <= pEnd.offset(-(4 as std::ffi::c_int as isize))
    {
        if MEM_64bits() != 0 {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh14 = p;
            p = p.offset(1);
            *fresh14 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
        }
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while BITv06_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv06_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p < pEnd
    {
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    while p < pEnd {
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = HUFv06_decodeSymbolX2(bitDPtr, dt, dtLog);
    }
    pEnd.offset_from(pStart) as std::ffi::c_long as size_t
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress1X2_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const U16,
) -> size_t {
    let mut op = dst as *mut BYTE;
    let oend = op.offset(dstSize as isize);
    let dtLog = *DTable.offset(0 as std::ffi::c_int as isize) as U32;
    let mut dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX2).offset(1 as std::ffi::c_int as isize);
    let mut bitD = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let errorCode = BITv06_initDStream(&mut bitD, cSrc, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv06_decodeStreamX2(op, &mut bitD, oend, dt, dtLog);
    if BITv06_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress1X2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_ushort; 4097] =
        [12 as std::ffi::c_int as std::ffi::c_ushort; 4097];
    let mut ip = cSrc as *const BYTE;
    let errorCode = HUFv06_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv06_decompress1X2_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress4X2_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const U16,
) -> size_t {
    if cSrcSize < 10 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX2).offset(1 as std::ffi::c_int as isize);
    let dtLog = *DTable.offset(0 as std::ffi::c_int as isize) as U32;
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD2 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD3 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD4 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as size_t;
    let length2 =
        MEM_readLE16(istart.offset(2 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length3 =
        MEM_readLE16(istart.offset(4 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let mut length4: size_t = 0;
    let istart1 = istart.offset(6 as std::ffi::c_int as isize);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize =
        dstSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal: U32 = 0;
    length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6 as std::ffi::c_int as size_t),
    );
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    errorCode = BITv06_initDStream(&mut bitD1, istart1 as *const std::ffi::c_void, length1);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD2, istart2 as *const std::ffi::c_void, length2);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD3, istart3 as *const std::ffi::c_void, length3);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD4, istart4 as *const std::ffi::c_void, length4);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BITv06_reloadDStream(&mut bitD1) as std::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD2) as std::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD3) as std::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    while endSignal == BITv06_DStream_unfinished as std::ffi::c_int as U32
        && op4 < oend.offset(-(7 as std::ffi::c_int as isize))
    {
        if MEM_64bits() != 0 {
            let fresh18 = op1;
            op1 = op1.offset(1);
            *fresh18 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh19 = op2;
            op2 = op2.offset(1);
            *fresh19 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh20 = op3;
            op3 = op3.offset(1);
            *fresh20 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh21 = op4;
            op4 = op4.offset(1);
            *fresh21 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            let fresh22 = op1;
            op1 = op1.offset(1);
            *fresh22 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            let fresh23 = op2;
            op2 = op2.offset(1);
            *fresh23 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            let fresh24 = op3;
            op3 = op3.offset(1);
            *fresh24 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            let fresh25 = op4;
            op4 = op4.offset(1);
            *fresh25 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh26 = op1;
            op1 = op1.offset(1);
            *fresh26 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh27 = op2;
            op2 = op2.offset(1);
            *fresh27 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh28 = op3;
            op3 = op3.offset(1);
            *fresh28 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        }
        if MEM_64bits() != 0 {
            let fresh29 = op4;
            op4 = op4.offset(1);
            *fresh29 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        }
        let fresh30 = op1;
        op1 = op1.offset(1);
        *fresh30 = HUFv06_decodeSymbolX2(&mut bitD1, dt, dtLog);
        let fresh31 = op2;
        op2 = op2.offset(1);
        *fresh31 = HUFv06_decodeSymbolX2(&mut bitD2, dt, dtLog);
        let fresh32 = op3;
        op3 = op3.offset(1);
        *fresh32 = HUFv06_decodeSymbolX2(&mut bitD3, dt, dtLog);
        let fresh33 = op4;
        op4 = op4.offset(1);
        *fresh33 = HUFv06_decodeSymbolX2(&mut bitD4, dt, dtLog);
        endSignal = BITv06_reloadDStream(&mut bitD1) as std::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD2) as std::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD3) as std::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    HUFv06_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv06_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv06_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv06_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv06_endOfDStream(&mut bitD1)
        & BITv06_endOfDStream(&mut bitD2)
        & BITv06_endOfDStream(&mut bitD3)
        & BITv06_endOfDStream(&mut bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress4X2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_ushort; 4097] =
        [12 as std::ffi::c_int as std::ffi::c_ushort; 4097];
    let mut ip = cSrc as *const BYTE;
    let errorCode = HUFv06_readDTableX2(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    if errorCode >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(errorCode as isize);
    cSrcSize = cSrcSize.wrapping_sub(errorCode);
    HUFv06_decompress4X2_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
unsafe extern "C" fn HUFv06_fillDTableX4Level2(
    mut DTable: *mut HUFv06_DEltX4,
    mut sizeLog: U32,
    consumed: U32,
    mut rankValOrigin: *const U32,
    minWeight: std::ffi::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    sortedListSize: U32,
    mut nbBitsBaseline: U32,
    mut baseSeq: U16,
) {
    let mut DElt = HUFv06_DEltX4 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let mut rankVal: [U32; 17] = [0; 17];
    memcpy(
        rankVal.as_mut_ptr() as *mut std::ffi::c_void,
        rankValOrigin as *const std::ffi::c_void,
        ::core::mem::size_of::<[U32; 17]>() as std::ffi::c_ulong,
    );
    if minWeight > 1 as std::ffi::c_int {
        let mut i: U32 = 0;
        let mut skipSize = *rankVal.as_mut_ptr().offset(minWeight as isize);
        MEM_writeLE16(
            &mut DElt.sequence as *mut U16 as *mut std::ffi::c_void,
            baseSeq,
        );
        DElt.nbBits = consumed as BYTE;
        DElt.length = 1 as std::ffi::c_int as BYTE;
        i = 0 as std::ffi::c_int as U32;
        while i < skipSize {
            *DTable.offset(i as isize) = DElt;
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut s: U32 = 0;
    s = 0 as std::ffi::c_int as U32;
    while s < sortedListSize {
        let symbol = (*sortedSymbols.offset(s as isize)).symbol as U32;
        let weight = (*sortedSymbols.offset(s as isize)).weight as U32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let length = ((1 as std::ffi::c_int) << sizeLog.wrapping_sub(nbBits)) as U32;
        let start = *rankVal.as_mut_ptr().offset(weight as isize);
        let mut i_0 = start;
        let end = start.wrapping_add(length);
        MEM_writeLE16(
            &mut DElt.sequence as *mut U16 as *mut std::ffi::c_void,
            (baseSeq as U32).wrapping_add(symbol << 8 as std::ffi::c_int) as U16,
        );
        DElt.nbBits = nbBits.wrapping_add(consumed) as BYTE;
        DElt.length = 2 as std::ffi::c_int as BYTE;
        loop {
            let fresh34 = i_0;
            i_0 = i_0.wrapping_add(1);
            *DTable.offset(fresh34 as isize) = DElt;
            if (i_0 >= end) {
                break;
            }
        }
        let fresh35 = &mut (*rankVal.as_mut_ptr().offset(weight as isize));
        *fresh35 = (*fresh35).wrapping_add(length);
        s = s.wrapping_add(1);
        s;
    }
}
unsafe extern "C" fn HUFv06_fillDTableX4(
    mut DTable: *mut HUFv06_DEltX4,
    targetLog: U32,
    mut sortedList: *const sortedSymbol_t,
    sortedListSize: U32,
    mut rankStart: *const U32,
    mut rankValOrigin: *mut [U32; 17],
    maxWeight: U32,
    nbBitsBaseline: U32,
) {
    let mut rankVal: [U32; 17] = [0; 17];
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as std::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut s: U32 = 0;
    memcpy(
        rankVal.as_mut_ptr() as *mut std::ffi::c_void,
        rankValOrigin as *const std::ffi::c_void,
        ::core::mem::size_of::<[U32; 17]>() as std::ffi::c_ulong,
    );
    s = 0 as std::ffi::c_int as U32;
    while s < sortedListSize {
        let symbol = (*sortedList.offset(s as isize)).symbol as U16;
        let weight = (*sortedList.offset(s as isize)).weight as U32;
        let nbBits = nbBitsBaseline.wrapping_sub(weight);
        let start = *rankVal.as_mut_ptr().offset(weight as isize);
        let length = ((1 as std::ffi::c_int) << targetLog.wrapping_sub(nbBits)) as U32;
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut sortedRank: U32 = 0;
            let mut minWeight = nbBits.wrapping_add(scaleLog as U32) as std::ffi::c_int;
            if minWeight < 1 as std::ffi::c_int {
                minWeight = 1 as std::ffi::c_int;
            }
            sortedRank = *rankStart.offset(minWeight as isize);
            HUFv06_fillDTableX4Level2(
                DTable.offset(start as isize),
                targetLog.wrapping_sub(nbBits),
                nbBits,
                (*rankValOrigin.offset(nbBits as isize)).as_mut_ptr(),
                minWeight,
                sortedList.offset(sortedRank as isize),
                sortedListSize.wrapping_sub(sortedRank),
                nbBitsBaseline,
                symbol,
            );
        } else {
            let mut DElt = HUFv06_DEltX4 {
                sequence: 0,
                nbBits: 0,
                length: 0,
            };
            MEM_writeLE16(
                &mut DElt.sequence as *mut U16 as *mut std::ffi::c_void,
                symbol,
            );
            DElt.nbBits = nbBits as BYTE;
            DElt.length = 1 as std::ffi::c_int as BYTE;
            let mut u: U32 = 0;
            let end = start.wrapping_add(length);
            u = start;
            while u < end {
                *DTable.offset(u as isize) = DElt;
                u = u.wrapping_add(1);
                u;
            }
        }
        let fresh36 = &mut (*rankVal.as_mut_ptr().offset(weight as isize));
        *fresh36 = (*fresh36).wrapping_add(length);
        s = s.wrapping_add(1);
        s;
    }
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_readDTableX4(
    mut DTable: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut weightList: [BYTE; 256] = [0; 256];
    let mut sortedSymbol: [sortedSymbol_t; 256] = [sortedSymbol_t {
        symbol: 0,
        weight: 0,
    }; 256];
    let mut rankStats: [U32; 17] = [0 as std::ffi::c_int as U32; 17];
    let mut rankStart0: [U32; 18] = [0 as std::ffi::c_int as U32; 18];
    let rankStart = rankStart0
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize);
    let mut rankVal: rankVal_t = [[0; 17]; 16];
    let mut tableLog: U32 = 0;
    let mut maxW: U32 = 0;
    let mut sizeOfSort: U32 = 0;
    let mut nbSymbols: U32 = 0;
    let memLog = *DTable.offset(0 as std::ffi::c_int as isize);
    let mut iSize: size_t = 0;
    let mut dtPtr = DTable as *mut std::ffi::c_void;
    let dt = (dtPtr as *mut HUFv06_DEltX4).offset(1 as std::ffi::c_int as isize);
    if memLog > HUFv06_ABSOLUTEMAX_TABLELOG as U32 {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    iSize = HUFv06_readStats(
        weightList.as_mut_ptr(),
        (HUFv06_MAX_SYMBOL_VALUE + 1 as std::ffi::c_int) as size_t,
        rankStats.as_mut_ptr(),
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
    );
    if HUFv06_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > memLog {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    maxW = tableLog;
    while *rankStats.as_mut_ptr().offset(maxW as isize) == 0 as std::ffi::c_int as U32 {
        maxW = maxW.wrapping_sub(1);
        maxW;
    }
    let mut w: U32 = 0;
    let mut nextRankStart = 0 as std::ffi::c_int as U32;
    w = 1 as std::ffi::c_int as U32;
    while w < maxW.wrapping_add(1 as std::ffi::c_int as U32) {
        let mut current = nextRankStart;
        nextRankStart = nextRankStart.wrapping_add(*rankStats.as_mut_ptr().offset(w as isize));
        *rankStart.offset(w as isize) = current;
        w = w.wrapping_add(1);
        w;
    }
    *rankStart.offset(0 as std::ffi::c_int as isize) = nextRankStart;
    sizeOfSort = nextRankStart;
    let mut s: U32 = 0;
    s = 0 as std::ffi::c_int as U32;
    while s < nbSymbols {
        let w_0 = *weightList.as_mut_ptr().offset(s as isize) as U32;
        let fresh37 = &mut (*rankStart.offset(w_0 as isize));
        let fresh38 = *fresh37;
        *fresh37 = (*fresh37).wrapping_add(1);
        let r = fresh38;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).symbol = s as BYTE;
        (*sortedSymbol.as_mut_ptr().offset(r as isize)).weight = w_0 as BYTE;
        s = s.wrapping_add(1);
        s;
    }
    *rankStart.offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as U32;
    let rankVal0 = (*rankVal.as_mut_ptr().offset(0 as std::ffi::c_int as isize)).as_mut_ptr();
    let rescale = memLog
        .wrapping_sub(tableLog)
        .wrapping_sub(1 as std::ffi::c_int as U32) as std::ffi::c_int;
    let mut nextRankVal = 0 as std::ffi::c_int as U32;
    let mut w_1: U32 = 0;
    w_1 = 1 as std::ffi::c_int as U32;
    while w_1 < maxW.wrapping_add(1 as std::ffi::c_int as U32) {
        let mut current_0 = nextRankVal;
        nextRankVal = nextRankVal.wrapping_add(
            *rankStats.as_mut_ptr().offset(w_1 as isize) << w_1.wrapping_add(rescale as U32),
        );
        *rankVal0.offset(w_1 as isize) = current_0;
        w_1 = w_1.wrapping_add(1);
        w_1;
    }
    let minBits = tableLog
        .wrapping_add(1 as std::ffi::c_int as U32)
        .wrapping_sub(maxW);
    let mut consumed: U32 = 0;
    consumed = minBits;
    while consumed
        < memLog
            .wrapping_sub(minBits)
            .wrapping_add(1 as std::ffi::c_int as U32)
    {
        let rankValPtr = (*rankVal.as_mut_ptr().offset(consumed as isize)).as_mut_ptr();
        let mut w_2: U32 = 0;
        w_2 = 1 as std::ffi::c_int as U32;
        while w_2 < maxW.wrapping_add(1 as std::ffi::c_int as U32) {
            *rankValPtr.offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
            w_2;
        }
        consumed = consumed.wrapping_add(1);
        consumed;
    }
    HUFv06_fillDTableX4(
        dt,
        memLog,
        sortedSymbol.as_mut_ptr(),
        sizeOfSort,
        rankStart0.as_mut_ptr(),
        rankVal.as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1 as std::ffi::c_int as U32),
    );
    iSize
}
unsafe extern "C" fn HUFv06_decodeSymbolX4(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BITv06_DStream_t,
    mut dt: *const HUFv06_DEltX4,
    dtLog: U32,
) -> U32 {
    let val = BITv06_lookBitsFast(DStream, dtLog);
    memcpy(
        op,
        dt.offset(val as isize) as *const std::ffi::c_void,
        2 as std::ffi::c_int as std::ffi::c_ulong,
    );
    BITv06_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
    (*dt.offset(val as isize)).length as U32
}
unsafe extern "C" fn HUFv06_decodeLastSymbolX4(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BITv06_DStream_t,
    mut dt: *const HUFv06_DEltX4,
    dtLog: U32,
) -> U32 {
    let val = BITv06_lookBitsFast(DStream, dtLog);
    memcpy(
        op,
        dt.offset(val as isize) as *const std::ffi::c_void,
        1 as std::ffi::c_int as std::ffi::c_ulong,
    );
    if (*dt.offset(val as isize)).length as std::ffi::c_int == 1 as std::ffi::c_int {
        BITv06_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
    } else if ((*DStream).bitsConsumed as std::ffi::c_ulong)
        < (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
    {
        BITv06_skipBits(DStream, (*dt.offset(val as isize)).nbBits as U32);
        if (*DStream).bitsConsumed as std::ffi::c_ulong
            > (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            (*DStream).bitsConsumed = (::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                as std::ffi::c_uint;
        }
    }
    1 as std::ffi::c_int as U32
}
#[inline]
unsafe extern "C" fn HUFv06_decodeStreamX4(
    mut p: *mut BYTE,
    mut bitDPtr: *mut BITv06_DStream_t,
    pEnd: *mut BYTE,
    dt: *const HUFv06_DEltX4,
    dtLog: U32,
) -> size_t {
    let pStart = p;
    while BITv06_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv06_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p < pEnd.offset(-(7 as std::ffi::c_int as isize))
    {
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv06_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            p = p.offset(
                HUFv06_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        if MEM_64bits() != 0 {
            p = p.offset(
                HUFv06_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        p = p
            .offset(HUFv06_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize);
    }
    while BITv06_reloadDStream(bitDPtr) as std::ffi::c_uint
        == BITv06_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint
        && p <= pEnd.offset(-(2 as std::ffi::c_int as isize))
    {
        p = p
            .offset(HUFv06_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize);
    }
    while p <= pEnd.offset(-(2 as std::ffi::c_int as isize)) {
        p = p
            .offset(HUFv06_decodeSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize);
    }
    if p < pEnd {
        p = p.offset(
            HUFv06_decodeLastSymbolX4(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as std::ffi::c_long as size_t
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress1X4_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const U32,
) -> size_t {
    let istart = cSrc as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let dtLog = *DTable.offset(0 as std::ffi::c_int as isize);
    let dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX4).offset(1 as std::ffi::c_int as isize);
    let mut bitD = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let errorCode = BITv06_initDStream(&mut bitD, istart as *const std::ffi::c_void, cSrcSize);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    HUFv06_decodeStreamX4(ostart, &mut bitD, oend, dt, dtLog);
    if BITv06_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress1X4(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_uint; 4097] = [12 as std::ffi::c_int as std::ffi::c_uint; 4097];
    let mut ip = cSrc as *const BYTE;
    let hSize = HUFv06_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv06_decompress1X4_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress4X4_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const U32,
) -> size_t {
    if cSrcSize < 10 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const BYTE;
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(dstSize as isize);
    let dtPtr = DTable as *const std::ffi::c_void;
    let dt = (dtPtr as *const HUFv06_DEltX4).offset(1 as std::ffi::c_int as isize);
    let dtLog = *DTable.offset(0 as std::ffi::c_int as isize);
    let mut errorCode: size_t = 0;
    let mut bitD1 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD2 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD3 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD4 = BITv06_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as size_t;
    let length2 =
        MEM_readLE16(istart.offset(2 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length3 =
        MEM_readLE16(istart.offset(4 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let mut length4: size_t = 0;
    let istart1 = istart.offset(6 as std::ffi::c_int as isize);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize =
        dstSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal: U32 = 0;
    length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6 as std::ffi::c_int as size_t),
    );
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    errorCode = BITv06_initDStream(&mut bitD1, istart1 as *const std::ffi::c_void, length1);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD2, istart2 as *const std::ffi::c_void, length2);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD3, istart3 as *const std::ffi::c_void, length3);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    errorCode = BITv06_initDStream(&mut bitD4, istart4 as *const std::ffi::c_void, length4);
    if HUFv06_isError(errorCode) != 0 {
        return errorCode;
    }
    endSignal = BITv06_reloadDStream(&mut bitD1) as std::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD2) as std::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD3) as std::ffi::c_uint
        | BITv06_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    while endSignal == BITv06_DStream_unfinished as std::ffi::c_int as U32
        && op4 < oend.offset(-(7 as std::ffi::c_int as isize))
    {
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv06_decodeSymbolX4(
                op1 as *mut std::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv06_decodeSymbolX4(
                op2 as *mut std::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv06_decodeSymbolX4(
                op3 as *mut std::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv06_decodeSymbolX4(
                op4 as *mut std::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            op1 = op1.offset(HUFv06_decodeSymbolX4(
                op1 as *mut std::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            op2 = op2.offset(HUFv06_decodeSymbolX4(
                op2 as *mut std::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            op3 = op3.offset(HUFv06_decodeSymbolX4(
                op3 as *mut std::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 || HUFv06_MAX_TABLELOG <= 12 as std::ffi::c_int {
            op4 = op4.offset(HUFv06_decodeSymbolX4(
                op4 as *mut std::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op1 = op1.offset(HUFv06_decodeSymbolX4(
                op1 as *mut std::ffi::c_void,
                &mut bitD1,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op2 = op2.offset(HUFv06_decodeSymbolX4(
                op2 as *mut std::ffi::c_void,
                &mut bitD2,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op3 = op3.offset(HUFv06_decodeSymbolX4(
                op3 as *mut std::ffi::c_void,
                &mut bitD3,
                dt,
                dtLog,
            ) as isize);
        }
        if MEM_64bits() != 0 {
            op4 = op4.offset(HUFv06_decodeSymbolX4(
                op4 as *mut std::ffi::c_void,
                &mut bitD4,
                dt,
                dtLog,
            ) as isize);
        }
        op1 = op1.offset(
            HUFv06_decodeSymbolX4(op1 as *mut std::ffi::c_void, &mut bitD1, dt, dtLog) as isize,
        );
        op2 = op2.offset(
            HUFv06_decodeSymbolX4(op2 as *mut std::ffi::c_void, &mut bitD2, dt, dtLog) as isize,
        );
        op3 = op3.offset(
            HUFv06_decodeSymbolX4(op3 as *mut std::ffi::c_void, &mut bitD3, dt, dtLog) as isize,
        );
        op4 = op4.offset(
            HUFv06_decodeSymbolX4(op4 as *mut std::ffi::c_void, &mut bitD4, dt, dtLog) as isize,
        );
        endSignal = BITv06_reloadDStream(&mut bitD1) as std::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD2) as std::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD3) as std::ffi::c_uint
            | BITv06_reloadDStream(&mut bitD4) as std::ffi::c_uint;
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    HUFv06_decodeStreamX4(op1, &mut bitD1, opStart2, dt, dtLog);
    HUFv06_decodeStreamX4(op2, &mut bitD2, opStart3, dt, dtLog);
    HUFv06_decodeStreamX4(op3, &mut bitD3, opStart4, dt, dtLog);
    HUFv06_decodeStreamX4(op4, &mut bitD4, oend, dt, dtLog);
    endSignal = BITv06_endOfDStream(&mut bitD1)
        & BITv06_endOfDStream(&mut bitD2)
        & BITv06_endOfDStream(&mut bitD3)
        & BITv06_endOfDStream(&mut bitD4);
    if endSignal == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress4X4(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    let mut DTable: [std::ffi::c_uint; 4097] = [12 as std::ffi::c_int as std::ffi::c_uint; 4097];
    let mut ip = cSrc as *const BYTE;
    let mut hSize = HUFv06_readDTableX4(DTable.as_mut_ptr(), cSrc, cSrcSize);
    if HUFv06_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUFv06_decompress4X4_usingDTable(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DTable.as_mut_ptr(),
    )
}
static mut algoTime: [[algo_time_t; 3]; 16] = [
    [
        {
            
            algo_time_t {
                tableTime: 0 as std::ffi::c_int as U32,
                decode256Time: 0 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1 as std::ffi::c_int as U32,
                decode256Time: 1 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2 as std::ffi::c_int as U32,
                decode256Time: 2 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 0 as std::ffi::c_int as U32,
                decode256Time: 0 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1 as std::ffi::c_int as U32,
                decode256Time: 1 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2 as std::ffi::c_int as U32,
                decode256Time: 2 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 38 as std::ffi::c_int as U32,
                decode256Time: 130 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1313 as std::ffi::c_int as U32,
                decode256Time: 74 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2151 as std::ffi::c_int as U32,
                decode256Time: 38 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 448 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1353 as std::ffi::c_int as U32,
                decode256Time: 74 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2238 as std::ffi::c_int as U32,
                decode256Time: 41 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 556 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1353 as std::ffi::c_int as U32,
                decode256Time: 74 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2238 as std::ffi::c_int as U32,
                decode256Time: 47 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 714 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1418 as std::ffi::c_int as U32,
                decode256Time: 74 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2436 as std::ffi::c_int as U32,
                decode256Time: 53 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 883 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1437 as std::ffi::c_int as U32,
                decode256Time: 74 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2464 as std::ffi::c_int as U32,
                decode256Time: 61 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 897 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1515 as std::ffi::c_int as U32,
                decode256Time: 75 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2622 as std::ffi::c_int as U32,
                decode256Time: 68 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 926 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1613 as std::ffi::c_int as U32,
                decode256Time: 75 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2730 as std::ffi::c_int as U32,
                decode256Time: 75 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 947 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1729 as std::ffi::c_int as U32,
                decode256Time: 77 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 3359 as std::ffi::c_int as U32,
                decode256Time: 77 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 1107 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2083 as std::ffi::c_int as U32,
                decode256Time: 81 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 4006 as std::ffi::c_int as U32,
                decode256Time: 84 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 1177 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2379 as std::ffi::c_int as U32,
                decode256Time: 87 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 4785 as std::ffi::c_int as U32,
                decode256Time: 88 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 1242 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2415 as std::ffi::c_int as U32,
                decode256Time: 93 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 5155 as std::ffi::c_int as U32,
                decode256Time: 84 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 1349 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2644 as std::ffi::c_int as U32,
                decode256Time: 106 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 5260 as std::ffi::c_int as U32,
                decode256Time: 106 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 1455 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 2422 as std::ffi::c_int as U32,
                decode256Time: 124 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 4174 as std::ffi::c_int as U32,
                decode256Time: 124 as std::ffi::c_int as U32,
            }
        },
    ],
    [
        {
            
            algo_time_t {
                tableTime: 722 as std::ffi::c_int as U32,
                decode256Time: 128 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1891 as std::ffi::c_int as U32,
                decode256Time: 145 as std::ffi::c_int as U32,
            }
        },
        {
            
            algo_time_t {
                tableTime: 1936 as std::ffi::c_int as U32,
                decode256Time: 146 as std::ffi::c_int as U32,
            }
        },
    ],
];
#[no_mangle]
pub unsafe extern "C" fn HUFv06_decompress(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
) -> size_t {
    static mut decompress: [decompressionAlgo; 3] = unsafe {
        [
            Some(
                HUFv06_decompress4X2
                    as unsafe extern "C" fn(
                        *mut std::ffi::c_void,
                        size_t,
                        *const std::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            Some(
                HUFv06_decompress4X4
                    as unsafe extern "C" fn(
                        *mut std::ffi::c_void,
                        size_t,
                        *const std::ffi::c_void,
                        size_t,
                    ) -> size_t,
            ),
            ::core::mem::transmute::<libc::intptr_t, decompressionAlgo>(NULL as libc::intptr_t),
        ]
    };
    let mut Dtime: [U32; 3] = [0; 3];
    if dstSize == 0 as std::ffi::c_int as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if cSrcSize == dstSize {
        memcpy(dst, cSrc, dstSize);
        return dstSize;
    }
    if cSrcSize == 1 as std::ffi::c_int as size_t {
        memset(dst, *(cSrc as *const BYTE) as std::ffi::c_int, dstSize);
        return dstSize;
    }
    let Q = (cSrcSize * 16 as std::ffi::c_int as size_t / dstSize) as U32;
    let D256 = (dstSize >> 8 as std::ffi::c_int) as U32;
    let mut n: U32 = 0;
    n = 0 as std::ffi::c_int as U32;
    while n < 3 as std::ffi::c_int as U32 {
        *Dtime.as_mut_ptr().offset(n as isize) = ((*(*algoTime.as_ptr().offset(Q as isize))
            .as_ptr()
            .offset(n as isize))
        .tableTime)
            .wrapping_add(
                (*(*algoTime.as_ptr().offset(Q as isize))
                    .as_ptr()
                    .offset(n as isize))
                .decode256Time
                    * D256,
            );
        n = n.wrapping_add(1);
        n;
    }
    let fresh39 = &mut (*Dtime.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
    *fresh39 = (*fresh39).wrapping_add(
        *Dtime.as_mut_ptr().offset(1 as std::ffi::c_int as isize) >> 4 as std::ffi::c_int,
    );
    let fresh40 = &mut (*Dtime.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
    *fresh40 = (*fresh40).wrapping_add(
        *Dtime.as_mut_ptr().offset(2 as std::ffi::c_int as isize) >> 3 as std::ffi::c_int,
    );
    let mut algoNb = 0 as std::ffi::c_int as U32;
    if *Dtime.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
        < *Dtime.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
    {
        algoNb = 1 as std::ffi::c_int as U32;
    }
    (*decompress.as_ptr().offset(algoNb as isize)).unwrap_unchecked()(
        dst, dstSize, cSrc, cSrcSize,
    )
}
#[export_name = "ZSTDv06_isError"]
pub unsafe extern "C" fn ZSTDv06_isError_0(mut code: size_t) -> std::ffi::c_uint {
    ERR_isError(code)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_getErrorName(mut code: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(code)
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_isError(mut errorCode: size_t) -> std::ffi::c_uint {
    ERR_isError(errorCode)
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_getErrorName(mut errorCode: size_t) -> *const std::ffi::c_char {
    ERR_getErrorName(errorCode)
}
pub const ZSTDv06_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
pub const FSEv06_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
pub const HUFv06_isError_0: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
unsafe extern "C" fn ZSTDv06_copy4(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
) {
    memcpy(dst, src, 4 as std::ffi::c_int as std::ffi::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_sizeofDCtx() -> size_t {
    ::core::mem::size_of::<ZSTDv06_DCtx>() as std::ffi::c_ulong
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompressBegin(mut dctx: *mut ZSTDv06_DCtx) -> size_t {
    (*dctx).expected = ZSTDv06_frameHeaderSize_min;
    (*dctx).stage = ZSTDds_getFrameHeaderSize;
    (*dctx).previousDstEnd = NULL as *const std::ffi::c_void;
    (*dctx).base = NULL as *const std::ffi::c_void;
    (*dctx).vBase = NULL as *const std::ffi::c_void;
    (*dctx).dictEnd = NULL as *const std::ffi::c_void;
    *((*dctx).hufTableX4)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) = ZSTD_HUFFDTABLE_CAPACITY_LOG as std::ffi::c_uint;
    (*dctx).flagRepeatTable = 0 as std::ffi::c_int as U32;
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_createDCtx() -> *mut ZSTDv06_DCtx {
    let mut dctx =
        malloc(::core::mem::size_of::<ZSTDv06_DCtx>() as std::ffi::c_ulong) as *mut ZSTDv06_DCtx;
    if dctx.is_null() {
        return NULL as *mut ZSTDv06_DCtx;
    }
    ZSTDv06_decompressBegin(dctx);
    dctx
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_freeDCtx(mut dctx: *mut ZSTDv06_DCtx) -> size_t {
    free(dctx as *mut std::ffi::c_void);
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_copyDCtx(
    mut dstDCtx: *mut ZSTDv06_DCtx,
    mut srcDCtx: *const ZSTDv06_DCtx,
) {
    memcpy(
        dstDCtx as *mut std::ffi::c_void,
        srcDCtx as *const std::ffi::c_void,
        (::core::mem::size_of::<ZSTDv06_DCtx>() as std::ffi::c_ulong).wrapping_sub(
            ((ZSTDv06_BLOCKSIZE_MAX + WILDCOPY_OVERLENGTH) as size_t)
                .wrapping_add(ZSTDv06_frameHeaderSize_max),
        ),
    );
}
unsafe extern "C" fn ZSTDv06_frameHeaderSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < ZSTDv06_frameHeaderSize_min {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let fcsId = (*(src as *const BYTE).offset(4 as std::ffi::c_int as isize) as std::ffi::c_int
        >> 6 as std::ffi::c_int) as U32;
    ZSTDv06_frameHeaderSize_min
        .wrapping_add(*ZSTDv06_fcs_fieldSize.as_ptr().offset(fcsId as isize))
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_getFrameParams(
    mut fparamsPtr: *mut ZSTDv06_frameParams,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    if srcSize < ZSTDv06_frameHeaderSize_min {
        return ZSTDv06_frameHeaderSize_min;
    }
    if MEM_readLE32(src) != ZSTDv06_MAGICNUMBER {
        return -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t;
    }
    let fhsize = ZSTDv06_frameHeaderSize(src, srcSize);
    if srcSize < fhsize {
        return fhsize;
    }
    memset(
        fparamsPtr as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<ZSTDv06_frameParams>() as std::ffi::c_ulong,
    );
    let frameDesc = *ip.offset(4 as std::ffi::c_int as isize);
    (*fparamsPtr).windowLog = ((frameDesc as std::ffi::c_int & 0xf as std::ffi::c_int)
        + ZSTDv06_WINDOWLOG_ABSOLUTEMIN) as std::ffi::c_uint;
    if frameDesc as std::ffi::c_int & 0x20 as std::ffi::c_int != 0 as std::ffi::c_int {
        return -(ZSTD_error_frameParameter_unsupported as std::ffi::c_int) as size_t;
    }
    let mut current_block_14: u64;
    match frameDesc as std::ffi::c_int >> 6 as std::ffi::c_int {
        0 => {
            current_block_14 = 2128447534733725941;
        }
        1 => {
            (*fparamsPtr).frameContentSize =
                *ip.offset(5 as std::ffi::c_int as isize) as std::ffi::c_ulonglong;
            current_block_14 = 17407779659766490442;
        }
        2 => {
            (*fparamsPtr).frameContentSize =
                (MEM_readLE16(ip.offset(5 as std::ffi::c_int as isize) as *const std::ffi::c_void)
                    as std::ffi::c_int
                    + 256 as std::ffi::c_int) as std::ffi::c_ulonglong;
            current_block_14 = 17407779659766490442;
        }
        3 => {
            (*fparamsPtr).frameContentSize =
                MEM_readLE64(ip.offset(5 as std::ffi::c_int as isize) as *const std::ffi::c_void)
                    as std::ffi::c_ulonglong;
            current_block_14 = 17407779659766490442;
        }
        _ => {
            current_block_14 = 2128447534733725941;
        }
    }
    if current_block_14 == 2128447534733725941 {
        (*fparamsPtr).frameContentSize = 0 as std::ffi::c_int as std::ffi::c_ulonglong;
    }
    0 as std::ffi::c_int as size_t
}
unsafe extern "C" fn ZSTDv06_decodeFrameHeader(
    mut zc: *mut ZSTDv06_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let result = ZSTDv06_getFrameParams(&mut (*zc).fParams, src, srcSize);
    if MEM_32bits() != 0 && (*zc).fParams.windowLog > 25 as std::ffi::c_int as std::ffi::c_uint {
        return -(ZSTD_error_frameParameter_unsupported as std::ffi::c_int) as size_t;
    }
    result
}
unsafe extern "C" fn ZSTDv06_getcBlockSize(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut bpPtr: *mut blockProperties_t,
) -> size_t {
    let in_0 = src as *const BYTE;
    let mut cSize: U32 = 0;
    if srcSize < ZSTDv06_blockHeaderSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    (*bpPtr).blockType = (*in_0 as std::ffi::c_int >> 6 as std::ffi::c_int) as blockType_t;
    cSize = (*in_0.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
        + ((*in_0.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
            << 8 as std::ffi::c_int)
        + ((*in_0.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int & 7 as std::ffi::c_int)
            << 16 as std::ffi::c_int)) as U32;
    (*bpPtr).origSize = if (*bpPtr).blockType as std::ffi::c_uint
        == bt_rle as std::ffi::c_int as std::ffi::c_uint
    {
        cSize
    } else {
        0 as std::ffi::c_int as U32
    };
    if (*bpPtr).blockType as std::ffi::c_uint == bt_end as std::ffi::c_int as std::ffi::c_uint {
        return 0 as std::ffi::c_int as size_t;
    }
    if (*bpPtr).blockType as std::ffi::c_uint == bt_rle as std::ffi::c_int as std::ffi::c_uint {
        return 1 as std::ffi::c_int as size_t;
    }
    cSize as size_t
}
unsafe extern "C" fn ZSTDv06_copyRawBlock(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if dst.is_null() {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if srcSize > dstCapacity {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    memcpy(dst, src, srcSize);
    srcSize
}
unsafe extern "C" fn ZSTDv06_decodeLiteralsBlock(
    mut dctx: *mut ZSTDv06_DCtx,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const BYTE;
    if srcSize < MIN_CBLOCK_SIZE as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    match *istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int >> 6 as std::ffi::c_int {
        IS_HUF => {
            let mut litSize: size_t = 0;
            let mut litCSize: size_t = 0;
            let mut singleStream = 0 as std::ffi::c_int as size_t;
            let mut lhSize = (*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                >> 4 as std::ffi::c_int
                & 3 as std::ffi::c_int) as U32;
            if srcSize < 5 as std::ffi::c_int as size_t {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            match lhSize {
                2 => {
                    lhSize = 4 as std::ffi::c_int as U32;
                    litSize = (((*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        & 15 as std::ffi::c_int)
                        << 10 as std::ffi::c_int)
                        + ((*istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                            << 2 as std::ffi::c_int)
                        + (*istart.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                            >> 6 as std::ffi::c_int)) as size_t;
                    litCSize = (((*istart.offset(2 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 63 as std::ffi::c_int)
                        << 8 as std::ffi::c_int)
                        + *istart.offset(3 as std::ffi::c_int as isize) as std::ffi::c_int)
                        as size_t;
                }
                3 => {
                    lhSize = 5 as std::ffi::c_int as U32;
                    litSize = (((*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        & 15 as std::ffi::c_int)
                        << 14 as std::ffi::c_int)
                        + ((*istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                            << 6 as std::ffi::c_int)
                        + (*istart.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                            >> 2 as std::ffi::c_int)) as size_t;
                    litCSize = (((*istart.offset(2 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 3 as std::ffi::c_int)
                        << 16 as std::ffi::c_int)
                        + ((*istart.offset(3 as std::ffi::c_int as isize) as std::ffi::c_int)
                            << 8 as std::ffi::c_int)
                        + *istart.offset(4 as std::ffi::c_int as isize) as std::ffi::c_int)
                        as size_t;
                }
                0 | 1 | _ => {
                    lhSize = 3 as std::ffi::c_int as U32;
                    singleStream = (*istart.offset(0 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 16 as std::ffi::c_int) as size_t;
                    litSize = (((*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        & 15 as std::ffi::c_int)
                        << 6 as std::ffi::c_int)
                        + (*istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                            >> 2 as std::ffi::c_int)) as size_t;
                    litCSize = (((*istart.offset(1 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 3 as std::ffi::c_int)
                        << 8 as std::ffi::c_int)
                        + *istart.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int)
                        as size_t;
                }
            }
            if litSize > ZSTDv06_BLOCKSIZE_MAX as size_t {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if litCSize.wrapping_add(lhSize as size_t) > srcSize {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if ERR_isError(if singleStream != 0 {
                HUFv06_decompress1X2(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const std::ffi::c_void,
                    litCSize,
                )
            } else {
                HUFv06_decompress(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                    litSize,
                    istart.offset(lhSize as isize) as *const std::ffi::c_void,
                    litCSize,
                )
            }) != 0
            {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize;
            memset(
                ((*dctx).litBuffer)
                    .as_mut_ptr()
                    .offset((*dctx).litSize as isize) as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                WILDCOPY_OVERLENGTH as std::ffi::c_ulong,
            );
            return litCSize.wrapping_add(lhSize as size_t);
        }
        IS_PCH => {
            let mut litSize_0: size_t = 0;
            let mut litCSize_0: size_t = 0;
            let mut lhSize_0 = (*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                >> 4 as std::ffi::c_int
                & 3 as std::ffi::c_int) as U32;
            if lhSize_0 != 1 as std::ffi::c_int as U32 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if (*dctx).flagRepeatTable == 0 {
                return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
            }
            lhSize_0 = 3 as std::ffi::c_int as U32;
            litSize_0 = (((*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                & 15 as std::ffi::c_int)
                << 6 as std::ffi::c_int)
                + (*istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                    >> 2 as std::ffi::c_int)) as size_t;
            litCSize_0 = (((*istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                & 3 as std::ffi::c_int)
                << 8 as std::ffi::c_int)
                + *istart.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int)
                as size_t;
            if litCSize_0.wrapping_add(lhSize_0 as size_t) > srcSize {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            let errorCode = HUFv06_decompress1X4_usingDTable(
                ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                litSize_0,
                istart.offset(lhSize_0 as isize) as *const std::ffi::c_void,
                litCSize_0,
                ((*dctx).hufTableX4).as_mut_ptr(),
            );
            if ERR_isError(errorCode) != 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_0;
            memset(
                ((*dctx).litBuffer)
                    .as_mut_ptr()
                    .offset((*dctx).litSize as isize) as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                WILDCOPY_OVERLENGTH as std::ffi::c_ulong,
            );
            return litCSize_0.wrapping_add(lhSize_0 as size_t);
        }
        IS_RAW => {
            let mut litSize_1: size_t = 0;
            let mut lhSize_1 = (*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                >> 4 as std::ffi::c_int
                & 3 as std::ffi::c_int) as U32;
            match lhSize_1 {
                2 => {
                    litSize_1 = (((*istart.offset(0 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 15 as std::ffi::c_int)
                        << 8 as std::ffi::c_int)
                        + *istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                        as size_t;
                }
                3 => {
                    litSize_1 = (((*istart.offset(0 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 15 as std::ffi::c_int)
                        << 16 as std::ffi::c_int)
                        + ((*istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                            << 8 as std::ffi::c_int)
                        + *istart.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int)
                        as size_t;
                }
                0 | 1 | _ => {
                    lhSize_1 = 1 as std::ffi::c_int as U32;
                    litSize_1 = (*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        & 31 as std::ffi::c_int) as size_t;
                }
            }
            if (lhSize_1 as size_t)
                .wrapping_add(litSize_1)
                .wrapping_add(WILDCOPY_OVERLENGTH as size_t)
                > srcSize
            {
                if litSize_1.wrapping_add(lhSize_1 as size_t) > srcSize {
                    return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                }
                memcpy(
                    ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                    istart.offset(lhSize_1 as isize) as *const std::ffi::c_void,
                    litSize_1,
                );
                (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
                (*dctx).litSize = litSize_1;
                memset(
                    ((*dctx).litBuffer)
                        .as_mut_ptr()
                        .offset((*dctx).litSize as isize)
                        as *mut std::ffi::c_void,
                    0 as std::ffi::c_int,
                    WILDCOPY_OVERLENGTH as std::ffi::c_ulong,
                );
                return (lhSize_1 as size_t).wrapping_add(litSize_1);
            }
            (*dctx).litPtr = istart.offset(lhSize_1 as isize);
            (*dctx).litSize = litSize_1;
            return (lhSize_1 as size_t).wrapping_add(litSize_1);
        }
        IS_RLE => {
            let mut litSize_2: size_t = 0;
            let mut lhSize_2 = (*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                >> 4 as std::ffi::c_int
                & 3 as std::ffi::c_int) as U32;
            match lhSize_2 {
                2 => {
                    litSize_2 = (((*istart.offset(0 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 15 as std::ffi::c_int)
                        << 8 as std::ffi::c_int)
                        + *istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                        as size_t;
                }
                3 => {
                    litSize_2 = (((*istart.offset(0 as std::ffi::c_int as isize)
                        as std::ffi::c_int
                        & 15 as std::ffi::c_int)
                        << 16 as std::ffi::c_int)
                        + ((*istart.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                            << 8 as std::ffi::c_int)
                        + *istart.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int)
                        as size_t;
                    if srcSize < 4 as std::ffi::c_int as size_t {
                        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
                    }
                }
                0 | 1 | _ => {
                    lhSize_2 = 1 as std::ffi::c_int as U32;
                    litSize_2 = (*istart.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                        & 31 as std::ffi::c_int) as size_t;
                }
            }
            if litSize_2 > ZSTDv06_BLOCKSIZE_MAX as size_t {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            memset(
                ((*dctx).litBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                *istart.offset(lhSize_2 as isize) as std::ffi::c_int,
                litSize_2.wrapping_add(WILDCOPY_OVERLENGTH as size_t),
            );
            (*dctx).litPtr = ((*dctx).litBuffer).as_mut_ptr();
            (*dctx).litSize = litSize_2;
            return lhSize_2.wrapping_add(1 as std::ffi::c_int as U32) as size_t;
        }
        _ => -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t,
    }
}
unsafe extern "C" fn ZSTDv06_buildSeqTable(
    mut DTable: *mut FSEv06_DTable,
    mut type_0: U32,
    mut max: U32,
    mut maxLog: U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut defaultNorm: *const S16,
    mut defaultLog: U32,
    mut flagRepeatTable: U32,
) -> size_t {
    match type_0 {
        1 => {
            if srcSize == 0 {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            if *(src as *const BYTE) as U32 > max {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            FSEv06_buildDTable_rle(DTable, *(src as *const BYTE));
            return 1 as std::ffi::c_int as size_t;
        }
        0 => {
            FSEv06_buildDTable(DTable, defaultNorm, max, defaultLog);
            return 0 as std::ffi::c_int as size_t;
        }
        2 => {
            if flagRepeatTable == 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            return 0 as std::ffi::c_int as size_t;
        }
        3 | _ => {
            let mut tableLog: U32 = 0;
            let mut norm: [S16; 53] = [0; 53];
            let headerSize =
                FSEv06_readNCount(norm.as_mut_ptr(), &mut max, &mut tableLog, src, srcSize);
            if ERR_isError(headerSize) != 0 {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            if tableLog > maxLog {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            FSEv06_buildDTable(DTable, norm.as_mut_ptr(), max, tableLog);
            headerSize
        }
    }
}
unsafe extern "C" fn ZSTDv06_decodeSeqHeaders(
    mut nbSeqPtr: *mut std::ffi::c_int,
    mut DTableLL: *mut FSEv06_DTable,
    mut DTableML: *mut FSEv06_DTable,
    mut DTableOffb: *mut FSEv06_DTable,
    mut flagRepeatTable: U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let istart = src as *const BYTE;
    let iend = istart.offset(srcSize as isize);
    let mut ip = istart;
    if srcSize < MIN_SEQUENCES_SIZE as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let fresh41 = ip;
    ip = ip.offset(1);
    let mut nbSeq = *fresh41 as std::ffi::c_int;
    if nbSeq == 0 {
        *nbSeqPtr = 0 as std::ffi::c_int;
        return 1 as std::ffi::c_int as size_t;
    }
    if nbSeq > 0x7f as std::ffi::c_int {
        if nbSeq == 0xff as std::ffi::c_int {
            if ip.offset(2 as std::ffi::c_int as isize) > iend {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            nbSeq = MEM_readLE16(ip as *const std::ffi::c_void) as std::ffi::c_int + LONGNBSEQ;
            ip = ip.offset(2 as std::ffi::c_int as isize);
        } else {
            if ip >= iend {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            let fresh42 = ip;
            ip = ip.offset(1);
            nbSeq = ((nbSeq - 0x80 as std::ffi::c_int) << 8 as std::ffi::c_int)
                + *fresh42 as std::ffi::c_int;
        }
    }
    *nbSeqPtr = nbSeq;
    if ip.offset(4 as std::ffi::c_int as isize) > iend {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let LLtype = (*ip as std::ffi::c_int >> 6 as std::ffi::c_int) as U32;
    let Offtype = (*ip as std::ffi::c_int >> 4 as std::ffi::c_int & 3 as std::ffi::c_int) as U32;
    let MLtype = (*ip as std::ffi::c_int >> 2 as std::ffi::c_int & 3 as std::ffi::c_int) as U32;
    ip = ip.offset(1);
    ip;
    let bhSize = ZSTDv06_buildSeqTable(
        DTableLL,
        LLtype,
        MaxLL as U32,
        LLFSELog as U32,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
        LL_defaultNorm.as_ptr(),
        LL_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(bhSize) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(bhSize as isize);
    let bhSize_0 = ZSTDv06_buildSeqTable(
        DTableOffb,
        Offtype,
        MaxOff as U32,
        OffFSELog as U32,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
        OF_defaultNorm.as_ptr(),
        OF_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(bhSize_0) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(bhSize_0 as isize);
    let bhSize_1 = ZSTDv06_buildSeqTable(
        DTableML,
        MLtype,
        MaxML as U32,
        MLFSELog as U32,
        ip as *const std::ffi::c_void,
        iend.offset_from(ip) as std::ffi::c_long as size_t,
        ML_defaultNorm.as_ptr(),
        ML_defaultNormLog,
        flagRepeatTable,
    );
    if ERR_isError(bhSize_1) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(bhSize_1 as isize);
    ip.offset_from(istart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTDv06_decodeSequence(mut seq: *mut seq_t, mut seqState: *mut seqState_t) {
    let llCode = FSEv06_peekSymbol(&mut (*seqState).stateLL) as U32;
    let mlCode = FSEv06_peekSymbol(&mut (*seqState).stateML) as U32;
    let ofCode = FSEv06_peekSymbol(&mut (*seqState).stateOffb) as U32;
    let llBits = *LL_bits.as_ptr().offset(llCode as isize);
    let mlBits = *ML_bits.as_ptr().offset(mlCode as isize);
    let ofBits = ofCode;
    let totalBits = llBits.wrapping_add(mlBits).wrapping_add(ofBits);
    static mut LL_base: [U32; 36] = [
        0 as std::ffi::c_int as U32,
        1 as std::ffi::c_int as U32,
        2 as std::ffi::c_int as U32,
        3 as std::ffi::c_int as U32,
        4 as std::ffi::c_int as U32,
        5 as std::ffi::c_int as U32,
        6 as std::ffi::c_int as U32,
        7 as std::ffi::c_int as U32,
        8 as std::ffi::c_int as U32,
        9 as std::ffi::c_int as U32,
        10 as std::ffi::c_int as U32,
        11 as std::ffi::c_int as U32,
        12 as std::ffi::c_int as U32,
        13 as std::ffi::c_int as U32,
        14 as std::ffi::c_int as U32,
        15 as std::ffi::c_int as U32,
        16 as std::ffi::c_int as U32,
        18 as std::ffi::c_int as U32,
        20 as std::ffi::c_int as U32,
        22 as std::ffi::c_int as U32,
        24 as std::ffi::c_int as U32,
        28 as std::ffi::c_int as U32,
        32 as std::ffi::c_int as U32,
        40 as std::ffi::c_int as U32,
        48 as std::ffi::c_int as U32,
        64 as std::ffi::c_int as U32,
        0x80 as std::ffi::c_int as U32,
        0x100 as std::ffi::c_int as U32,
        0x200 as std::ffi::c_int as U32,
        0x400 as std::ffi::c_int as U32,
        0x800 as std::ffi::c_int as U32,
        0x1000 as std::ffi::c_int as U32,
        0x2000 as std::ffi::c_int as U32,
        0x4000 as std::ffi::c_int as U32,
        0x8000 as std::ffi::c_int as U32,
        0x10000 as std::ffi::c_int as U32,
    ];
    static mut ML_base: [U32; 53] = [
        0 as std::ffi::c_int as U32,
        1 as std::ffi::c_int as U32,
        2 as std::ffi::c_int as U32,
        3 as std::ffi::c_int as U32,
        4 as std::ffi::c_int as U32,
        5 as std::ffi::c_int as U32,
        6 as std::ffi::c_int as U32,
        7 as std::ffi::c_int as U32,
        8 as std::ffi::c_int as U32,
        9 as std::ffi::c_int as U32,
        10 as std::ffi::c_int as U32,
        11 as std::ffi::c_int as U32,
        12 as std::ffi::c_int as U32,
        13 as std::ffi::c_int as U32,
        14 as std::ffi::c_int as U32,
        15 as std::ffi::c_int as U32,
        16 as std::ffi::c_int as U32,
        17 as std::ffi::c_int as U32,
        18 as std::ffi::c_int as U32,
        19 as std::ffi::c_int as U32,
        20 as std::ffi::c_int as U32,
        21 as std::ffi::c_int as U32,
        22 as std::ffi::c_int as U32,
        23 as std::ffi::c_int as U32,
        24 as std::ffi::c_int as U32,
        25 as std::ffi::c_int as U32,
        26 as std::ffi::c_int as U32,
        27 as std::ffi::c_int as U32,
        28 as std::ffi::c_int as U32,
        29 as std::ffi::c_int as U32,
        30 as std::ffi::c_int as U32,
        31 as std::ffi::c_int as U32,
        32 as std::ffi::c_int as U32,
        34 as std::ffi::c_int as U32,
        36 as std::ffi::c_int as U32,
        38 as std::ffi::c_int as U32,
        40 as std::ffi::c_int as U32,
        44 as std::ffi::c_int as U32,
        48 as std::ffi::c_int as U32,
        56 as std::ffi::c_int as U32,
        64 as std::ffi::c_int as U32,
        80 as std::ffi::c_int as U32,
        96 as std::ffi::c_int as U32,
        0x80 as std::ffi::c_int as U32,
        0x100 as std::ffi::c_int as U32,
        0x200 as std::ffi::c_int as U32,
        0x400 as std::ffi::c_int as U32,
        0x800 as std::ffi::c_int as U32,
        0x1000 as std::ffi::c_int as U32,
        0x2000 as std::ffi::c_int as U32,
        0x4000 as std::ffi::c_int as U32,
        0x8000 as std::ffi::c_int as U32,
        0x10000 as std::ffi::c_int as U32,
    ];
    static mut OF_base: [U32; 29] = [
        0 as std::ffi::c_int as U32,
        1 as std::ffi::c_int as U32,
        3 as std::ffi::c_int as U32,
        7 as std::ffi::c_int as U32,
        0xf as std::ffi::c_int as U32,
        0x1f as std::ffi::c_int as U32,
        0x3f as std::ffi::c_int as U32,
        0x7f as std::ffi::c_int as U32,
        0xff as std::ffi::c_int as U32,
        0x1ff as std::ffi::c_int as U32,
        0x3ff as std::ffi::c_int as U32,
        0x7ff as std::ffi::c_int as U32,
        0xfff as std::ffi::c_int as U32,
        0x1fff as std::ffi::c_int as U32,
        0x3fff as std::ffi::c_int as U32,
        0x7fff as std::ffi::c_int as U32,
        0xffff as std::ffi::c_int as U32,
        0x1ffff as std::ffi::c_int as U32,
        0x3ffff as std::ffi::c_int as U32,
        0x7ffff as std::ffi::c_int as U32,
        0xfffff as std::ffi::c_int as U32,
        0x1fffff as std::ffi::c_int as U32,
        0x3fffff as std::ffi::c_int as U32,
        0x7fffff as std::ffi::c_int as U32,
        0xffffff as std::ffi::c_int as U32,
        0x1ffffff as std::ffi::c_int as U32,
        0x3ffffff as std::ffi::c_int as U32,
        1 as std::ffi::c_int as U32,
        1 as std::ffi::c_int as U32,
    ];
    let mut offset: size_t = 0;
    if ofCode == 0 {
        offset = 0 as std::ffi::c_int as size_t;
    } else {
        offset = (*OF_base.as_ptr().offset(ofCode as isize) as size_t)
            .wrapping_add(BITv06_readBits(&mut (*seqState).DStream, ofBits));
        if MEM_32bits() != 0 {
            BITv06_reloadDStream(&mut (*seqState).DStream);
        }
    }
    if offset < ZSTDv06_REP_NUM as size_t {
        if llCode == 0 as std::ffi::c_int as U32 && offset <= 1 as std::ffi::c_int as size_t {
            offset = (1 as std::ffi::c_int as size_t).wrapping_sub(offset);
        }
        if offset != 0 as std::ffi::c_int as size_t {
            let mut temp = *((*seqState).prevOffset)
                .as_mut_ptr()
                .offset(offset as isize);
            if offset != 1 as std::ffi::c_int as size_t {
                *((*seqState).prevOffset)
                    .as_mut_ptr()
                    .offset(2 as std::ffi::c_int as isize) = *((*seqState).prevOffset)
                    .as_mut_ptr()
                    .offset(1 as std::ffi::c_int as isize);
            }
            *((*seqState).prevOffset)
                .as_mut_ptr()
                .offset(1 as std::ffi::c_int as isize) = *((*seqState).prevOffset)
                .as_mut_ptr()
                .offset(0 as std::ffi::c_int as isize);
            offset = temp;
            *((*seqState).prevOffset)
                .as_mut_ptr()
                .offset(0 as std::ffi::c_int as isize) = offset;
        } else {
            offset = *((*seqState).prevOffset)
                .as_mut_ptr()
                .offset(0 as std::ffi::c_int as isize);
        }
    } else {
        offset = offset.wrapping_sub(ZSTDv06_REP_MOVE as size_t);
        *((*seqState).prevOffset)
            .as_mut_ptr()
            .offset(2 as std::ffi::c_int as isize) = *((*seqState).prevOffset)
            .as_mut_ptr()
            .offset(1 as std::ffi::c_int as isize);
        *((*seqState).prevOffset)
            .as_mut_ptr()
            .offset(1 as std::ffi::c_int as isize) = *((*seqState).prevOffset)
            .as_mut_ptr()
            .offset(0 as std::ffi::c_int as isize);
        *((*seqState).prevOffset)
            .as_mut_ptr()
            .offset(0 as std::ffi::c_int as isize) = offset;
    }
    (*seq).offset = offset;
    (*seq).matchLength = ((*ML_base.as_ptr().offset(mlCode as isize)).wrapping_add(MINMATCH as U32)
        as size_t)
        .wrapping_add(
            if mlCode > 31 as std::ffi::c_int as U32 {
                BITv06_readBits(&mut (*seqState).DStream, mlBits)
            } else {
                0 as std::ffi::c_int as size_t
            },
        );
    if MEM_32bits() != 0 && mlBits.wrapping_add(llBits) > 24 as std::ffi::c_int as U32 {
        BITv06_reloadDStream(&mut (*seqState).DStream);
    }
    (*seq).litLength = (*LL_base.as_ptr().offset(llCode as isize) as size_t).wrapping_add(
        if llCode > 15 as std::ffi::c_int as U32 {
            BITv06_readBits(&mut (*seqState).DStream, llBits)
        } else {
            0 as std::ffi::c_int as size_t
        },
    );
    if MEM_32bits() != 0
        || totalBits
            > (64 as std::ffi::c_int - 7 as std::ffi::c_int - (LLFSELog + MLFSELog + OffFSELog))
                as U32
    {
        BITv06_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv06_updateState(&mut (*seqState).stateLL, &mut (*seqState).DStream);
    FSEv06_updateState(&mut (*seqState).stateML, &mut (*seqState).DStream);
    if MEM_32bits() != 0 {
        BITv06_reloadDStream(&mut (*seqState).DStream);
    }
    FSEv06_updateState(&mut (*seqState).stateOffb, &mut (*seqState).DStream);
}
unsafe extern "C" fn ZSTDv06_execSequence(
    mut op: *mut BYTE,
    oend: *mut BYTE,
    mut sequence: seq_t,
    mut litPtr: *mut *const BYTE,
    litLimit: *const BYTE,
    base: *const BYTE,
    vBase: *const BYTE,
    dictEnd: *const BYTE,
) -> size_t {
    let oLitEnd = op.offset(sequence.litLength as isize);
    let sequenceLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    let oMatchEnd = op.offset(sequenceLength as isize);
    let oend_8 = oend.offset(-(8 as std::ffi::c_int as isize));
    let iLitEnd = (*litPtr).offset(sequence.litLength as isize);
    let mut match_0: *const BYTE = oLitEnd.offset(-(sequence.offset as isize));
    let seqLength = (sequence.litLength).wrapping_add(sequence.matchLength);
    if seqLength > oend.offset_from(op) as std::ffi::c_long as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if sequence.litLength > litLimit.offset_from(*litPtr) as std::ffi::c_long as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if oLitEnd > oend_8 {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if oMatchEnd > oend {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if iLitEnd > litLimit {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ZSTDv06_wildcopy(
        op as *mut std::ffi::c_void,
        *litPtr as *const std::ffi::c_void,
        sequence.litLength as ptrdiff_t,
    );
    op = oLitEnd;
    *litPtr = iLitEnd;
    if sequence.offset > oLitEnd.offset_from(base) as std::ffi::c_long as size_t {
        if sequence.offset > oLitEnd.offset_from(vBase) as std::ffi::c_long as size_t {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        match_0 = dictEnd.offset(-(base.offset_from(match_0) as std::ffi::c_long as isize));
        if match_0.offset(sequence.matchLength as isize) <= dictEnd {
            memmove(
                oLitEnd as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                sequence.matchLength,
            );
            return sequenceLength;
        }
        let length1 = dictEnd.offset_from(match_0) as std::ffi::c_long as size_t;
        memmove(
            oLitEnd as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            length1,
        );
        op = oLitEnd.offset(length1 as isize);
        sequence.matchLength = (sequence.matchLength).wrapping_sub(length1);
        match_0 = base;
        if op > oend_8 || sequence.matchLength < MINMATCH as size_t {
            while op < oMatchEnd {
                let fresh43 = match_0;
                match_0 = match_0.offset(1);
                let fresh44 = op;
                op = op.offset(1);
                *fresh44 = *fresh43;
            }
            return sequenceLength;
        }
    }
    if sequence.offset < 8 as std::ffi::c_int as size_t {
        static mut dec32table: [U32; 8] = [
            0 as std::ffi::c_int as U32,
            1 as std::ffi::c_int as U32,
            2 as std::ffi::c_int as U32,
            1 as std::ffi::c_int as U32,
            4 as std::ffi::c_int as U32,
            4 as std::ffi::c_int as U32,
            4 as std::ffi::c_int as U32,
            4 as std::ffi::c_int as U32,
        ];
        static mut dec64table: [std::ffi::c_int; 8] = [
            8 as std::ffi::c_int,
            8 as std::ffi::c_int,
            8 as std::ffi::c_int,
            7 as std::ffi::c_int,
            8 as std::ffi::c_int,
            9 as std::ffi::c_int,
            10 as std::ffi::c_int,
            11 as std::ffi::c_int,
        ];
        let sub2 = *dec64table.as_ptr().offset(sequence.offset as isize);
        *op.offset(0 as std::ffi::c_int as isize) = *match_0.offset(0 as std::ffi::c_int as isize);
        *op.offset(1 as std::ffi::c_int as isize) = *match_0.offset(1 as std::ffi::c_int as isize);
        *op.offset(2 as std::ffi::c_int as isize) = *match_0.offset(2 as std::ffi::c_int as isize);
        *op.offset(3 as std::ffi::c_int as isize) = *match_0.offset(3 as std::ffi::c_int as isize);
        match_0 = match_0.offset(*dec32table.as_ptr().offset(sequence.offset as isize) as isize);
        ZSTDv06_copy4(
            op.offset(4 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
        );
        match_0 = match_0.offset(-(sub2 as isize));
    } else {
        ZSTDv06_copy8(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
        );
    }
    op = op.offset(8 as std::ffi::c_int as isize);
    match_0 = match_0.offset(8 as std::ffi::c_int as isize);
    if oMatchEnd > oend.offset(-((16 as std::ffi::c_int - MINMATCH) as isize)) {
        if op < oend_8 {
            ZSTDv06_wildcopy(
                op as *mut std::ffi::c_void,
                match_0 as *const std::ffi::c_void,
                oend_8.offset_from(op) as std::ffi::c_long,
            );
            match_0 = match_0.offset(oend_8.offset_from(op) as std::ffi::c_long as isize);
            op = oend_8;
        }
        while op < oMatchEnd {
            let fresh45 = match_0;
            match_0 = match_0.offset(1);
            let fresh46 = op;
            op = op.offset(1);
            *fresh46 = *fresh45;
        }
    } else {
        ZSTDv06_wildcopy(
            op as *mut std::ffi::c_void,
            match_0 as *const std::ffi::c_void,
            sequence.matchLength as ptrdiff_t - 8 as std::ffi::c_int as ptrdiff_t,
        );
    }
    sequenceLength
}
unsafe extern "C" fn ZSTDv06_decompressSequences(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut seqStart: *const std::ffi::c_void,
    mut seqSize: size_t,
) -> size_t {
    let mut ip = seqStart as *const BYTE;
    let iend = ip.offset(seqSize as isize);
    let ostart = dst as *mut BYTE;
    let oend = ostart.offset(maxDstSize as isize);
    let mut op = ostart;
    let mut litPtr = (*dctx).litPtr;
    let litEnd = litPtr.offset((*dctx).litSize as isize);
    let mut DTableLL = ((*dctx).LLTable).as_mut_ptr();
    let mut DTableML = ((*dctx).MLTable).as_mut_ptr();
    let mut DTableOffb = ((*dctx).OffTable).as_mut_ptr();
    let base = (*dctx).base as *const BYTE;
    let vBase = (*dctx).vBase as *const BYTE;
    let dictEnd = (*dctx).dictEnd as *const BYTE;
    let mut nbSeq: std::ffi::c_int = 0;
    let seqHSize = ZSTDv06_decodeSeqHeaders(
        &mut nbSeq,
        DTableLL,
        DTableML,
        DTableOffb,
        (*dctx).flagRepeatTable,
        ip as *const std::ffi::c_void,
        seqSize,
    );
    if ERR_isError(seqHSize) != 0 {
        return seqHSize;
    }
    ip = ip.offset(seqHSize as isize);
    (*dctx).flagRepeatTable = 0 as std::ffi::c_int as U32;
    if nbSeq != 0 {
        let mut sequence = seq_t {
            litLength: 0,
            matchLength: 0,
            offset: 0,
        };
        let mut seqState = seqState_t {
            DStream: BITv06_DStream_t {
                bitContainer: 0,
                bitsConsumed: 0,
                ptr: std::ptr::null::<std::ffi::c_char>(),
                start: std::ptr::null::<std::ffi::c_char>(),
            },
            stateLL: FSEv06_DState_t {
                state: 0,
                table: std::ptr::null::<std::ffi::c_void>(),
            },
            stateOffb: FSEv06_DState_t {
                state: 0,
                table: std::ptr::null::<std::ffi::c_void>(),
            },
            stateML: FSEv06_DState_t {
                state: 0,
                table: std::ptr::null::<std::ffi::c_void>(),
            },
            prevOffset: [0; 3],
        };
        memset(
            &mut sequence as *mut seq_t as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            ::core::mem::size_of::<seq_t>() as std::ffi::c_ulong,
        );
        sequence.offset = REPCODE_STARTVALUE as size_t;
        let mut i: U32 = 0;
        i = 0 as std::ffi::c_int as U32;
        while i < ZSTDv06_REP_INIT as U32 {
            *(seqState.prevOffset).as_mut_ptr().offset(i as isize) = REPCODE_STARTVALUE as size_t;
            i = i.wrapping_add(1);
            i;
        }
        let errorCode = BITv06_initDStream(
            &mut seqState.DStream,
            ip as *const std::ffi::c_void,
            iend.offset_from(ip) as std::ffi::c_long as size_t,
        );
        if ERR_isError(errorCode) != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        FSEv06_initDState(&mut seqState.stateLL, &mut seqState.DStream, DTableLL);
        FSEv06_initDState(&mut seqState.stateOffb, &mut seqState.DStream, DTableOffb);
        FSEv06_initDState(&mut seqState.stateML, &mut seqState.DStream, DTableML);
        while BITv06_reloadDStream(&mut seqState.DStream) as std::ffi::c_uint
            <= BITv06_DStream_completed as std::ffi::c_int as std::ffi::c_uint
            && nbSeq != 0
        {
            nbSeq -= 1;
            nbSeq;
            ZSTDv06_decodeSequence(&mut sequence, &mut seqState);
            let oneSeqSize = ZSTDv06_execSequence(
                op,
                oend,
                sequence,
                &mut litPtr,
                litEnd,
                base,
                vBase,
                dictEnd,
            );
            if ERR_isError(oneSeqSize) != 0 {
                return oneSeqSize;
            }
            op = op.offset(oneSeqSize as isize);
        }
        if nbSeq != 0 {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
    }
    let lastLLSize = litEnd.offset_from(litPtr) as std::ffi::c_long as size_t;
    if litPtr > litEnd {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if op.offset(lastLLSize as isize) > oend {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if lastLLSize > 0 as std::ffi::c_int as size_t {
        memcpy(
            op as *mut std::ffi::c_void,
            litPtr as *const std::ffi::c_void,
            lastLLSize,
        );
        op = op.offset(lastLLSize as isize);
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTDv06_checkContinuity(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *const std::ffi::c_void,
) {
    if dst != (*dctx).previousDstEnd {
        (*dctx).dictEnd = (*dctx).previousDstEnd;
        (*dctx).vBase = (dst as *const std::ffi::c_char).offset(
            -(((*dctx).previousDstEnd as *const std::ffi::c_char)
                .offset_from((*dctx).base as *const std::ffi::c_char)
                as std::ffi::c_long as isize),
        ) as *const std::ffi::c_void;
        (*dctx).base = dst;
        (*dctx).previousDstEnd = dst;
    }
}
unsafe extern "C" fn ZSTDv06_decompressBlock_internal(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    if srcSize >= ZSTDv06_BLOCKSIZE_MAX as size_t {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let litCSize = ZSTDv06_decodeLiteralsBlock(dctx, src, srcSize);
    if ERR_isError(litCSize) != 0 {
        return litCSize;
    }
    ip = ip.offset(litCSize as isize);
    srcSize = srcSize.wrapping_sub(litCSize);
    ZSTDv06_decompressSequences(
        dctx,
        dst,
        dstCapacity,
        ip as *const std::ffi::c_void,
        srcSize,
    )
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompressBlock(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv06_checkContinuity(dctx, dst);
    ZSTDv06_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize)
}
unsafe extern "C" fn ZSTDv06_decompressFrame(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut ip = src as *const BYTE;
    let iend = ip.offset(srcSize as isize);
    let ostart = dst as *mut BYTE;
    let mut op = ostart;
    let oend = ostart.offset(dstCapacity as isize);
    let mut remainingSize = srcSize;
    let mut blockProperties = {
        
        blockProperties_t {
            blockType: bt_compressed,
            origSize: 0 as std::ffi::c_int as U32,
        }
    };
    if srcSize < ZSTDv06_frameHeaderSize_min.wrapping_add(ZSTDv06_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    let frameHeaderSize = ZSTDv06_frameHeaderSize(src, ZSTDv06_frameHeaderSize_min);
    if ERR_isError(frameHeaderSize) != 0 {
        return frameHeaderSize;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv06_blockHeaderSize) {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if ZSTDv06_decodeFrameHeader(dctx, src, frameHeaderSize) != 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(frameHeaderSize as isize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    loop {
        let mut decodedSize = 0 as std::ffi::c_int as size_t;
        let cBlockSize = ZSTDv06_getcBlockSize(
            ip as *const std::ffi::c_void,
            iend.offset_from(ip) as std::ffi::c_long as size_t,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            return cBlockSize;
        }
        ip = ip.offset(ZSTDv06_blockHeaderSize as isize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv06_blockHeaderSize);
        if cBlockSize > remainingSize {
            return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
        }
        match blockProperties.blockType as std::ffi::c_uint {
            0 => {
                decodedSize = ZSTDv06_decompressBlock_internal(
                    dctx,
                    op as *mut std::ffi::c_void,
                    oend.offset_from(op) as std::ffi::c_long as size_t,
                    ip as *const std::ffi::c_void,
                    cBlockSize,
                );
            }
            1 => {
                decodedSize = ZSTDv06_copyRawBlock(
                    op as *mut std::ffi::c_void,
                    oend.offset_from(op) as std::ffi::c_long as size_t,
                    ip as *const std::ffi::c_void,
                    cBlockSize,
                );
            }
            2 => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
            3 => {
                if remainingSize != 0 {
                    return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
                }
            }
            _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
        }
        if cBlockSize == 0 as std::ffi::c_int as size_t {
            break;
        }
        if ERR_isError(decodedSize) != 0 {
            return decodedSize;
        }
        op = op.offset(decodedSize as isize);
        ip = ip.offset(cBlockSize as isize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
    }
    op.offset_from(ostart) as std::ffi::c_long as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompress_usingPreparedDCtx(
    mut dctx: *mut ZSTDv06_DCtx,
    mut refDCtx: *const ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv06_copyDCtx(dctx, refDCtx);
    ZSTDv06_checkContinuity(dctx, dst);
    ZSTDv06_decompressFrame(dctx, dst, dstCapacity, src, srcSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompress_usingDict(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    ZSTDv06_decompressBegin_usingDict(dctx, dict, dictSize);
    ZSTDv06_checkContinuity(dctx, dst);
    ZSTDv06_decompressFrame(dctx, dst, dstCapacity, src, srcSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompressDCtx(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTDv06_decompress_usingDict(
        dctx,
        dst,
        dstCapacity,
        src,
        srcSize,
        NULL as *const std::ffi::c_void,
        0 as std::ffi::c_int as size_t,
    )
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompress(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut regenSize: size_t = 0;
    let mut dctx = ZSTDv06_createDCtx();
    if dctx.is_null() {
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    regenSize = ZSTDv06_decompressDCtx(dctx, dst, dstCapacity, src, srcSize);
    ZSTDv06_freeDCtx(dctx);
    regenSize
}
unsafe extern "C" fn ZSTD_errorFrameSizeInfoLegacy(
    mut cSize: *mut size_t,
    mut dBound: *mut std::ffi::c_ulonglong,
    mut ret: size_t,
) {
    *cSize = ret;
    *dBound = ZSTD_CONTENTSIZE_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_findFrameSizeInfoLegacy(
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut cSize: *mut size_t,
    mut dBound: *mut std::ffi::c_ulonglong,
) {
    let mut ip = src as *const BYTE;
    let mut remainingSize = srcSize;
    let mut nbBlocks = 0 as std::ffi::c_int as size_t;
    let mut blockProperties = {
        
        blockProperties_t {
            blockType: bt_compressed,
            origSize: 0 as std::ffi::c_int as U32,
        }
    };
    let frameHeaderSize = ZSTDv06_frameHeaderSize(src, srcSize);
    if ERR_isError(frameHeaderSize) != 0 {
        ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, frameHeaderSize);
        return;
    }
    if MEM_readLE32(src) != ZSTDv06_MAGICNUMBER {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_prefix_unknown as std::ffi::c_int) as size_t,
        );
        return;
    }
    if srcSize < frameHeaderSize.wrapping_add(ZSTDv06_blockHeaderSize) {
        ZSTD_errorFrameSizeInfoLegacy(
            cSize,
            dBound,
            -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t,
        );
        return;
    }
    ip = ip.offset(frameHeaderSize as isize);
    remainingSize = remainingSize.wrapping_sub(frameHeaderSize);
    loop {
        let cBlockSize = ZSTDv06_getcBlockSize(
            ip as *const std::ffi::c_void,
            remainingSize,
            &mut blockProperties,
        );
        if ERR_isError(cBlockSize) != 0 {
            ZSTD_errorFrameSizeInfoLegacy(cSize, dBound, cBlockSize);
            return;
        }
        ip = ip.offset(ZSTDv06_blockHeaderSize as isize);
        remainingSize = remainingSize.wrapping_sub(ZSTDv06_blockHeaderSize);
        if cBlockSize > remainingSize {
            ZSTD_errorFrameSizeInfoLegacy(
                cSize,
                dBound,
                -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t,
            );
            return;
        }
        if cBlockSize == 0 as std::ffi::c_int as size_t {
            break;
        }
        ip = ip.offset(cBlockSize as isize);
        remainingSize = remainingSize.wrapping_sub(cBlockSize);
        nbBlocks = nbBlocks.wrapping_add(1);
        nbBlocks;
    }
    *cSize = ip.offset_from(src as *const BYTE) as std::ffi::c_long as size_t;
    *dBound = (nbBlocks * ZSTDv06_BLOCKSIZE_MAX as size_t) as std::ffi::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_nextSrcSizeToDecompress(mut dctx: *mut ZSTDv06_DCtx) -> size_t {
    (*dctx).expected
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompressContinue(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize != (*dctx).expected {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if dstCapacity != 0 {
        ZSTDv06_checkContinuity(dctx, dst);
    }
    match (*dctx).stage as std::ffi::c_uint {
        0 => {
            if srcSize != ZSTDv06_frameHeaderSize_min {
                return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
            }
            (*dctx).headerSize = ZSTDv06_frameHeaderSize(src, ZSTDv06_frameHeaderSize_min);
            if ERR_isError((*dctx).headerSize) != 0 {
                return (*dctx).headerSize;
            }
            memcpy(
                ((*dctx).headerBuffer).as_mut_ptr() as *mut std::ffi::c_void,
                src,
                ZSTDv06_frameHeaderSize_min,
            );
            if (*dctx).headerSize > ZSTDv06_frameHeaderSize_min {
                (*dctx).expected = ((*dctx).headerSize).wrapping_sub(ZSTDv06_frameHeaderSize_min);
                (*dctx).stage = ZSTDds_decodeFrameHeader;
                return 0 as std::ffi::c_int as size_t;
            }
            (*dctx).expected = 0 as std::ffi::c_int as size_t;
        }
        1 => {}
        2 => {
            let mut bp = blockProperties_t {
                blockType: bt_compressed,
                origSize: 0,
            };
            let cBlockSize = ZSTDv06_getcBlockSize(src, ZSTDv06_blockHeaderSize, &mut bp);
            if ERR_isError(cBlockSize) != 0 {
                return cBlockSize;
            }
            if bp.blockType as std::ffi::c_uint == bt_end as std::ffi::c_int as std::ffi::c_uint {
                (*dctx).expected = 0 as std::ffi::c_int as size_t;
                (*dctx).stage = ZSTDds_getFrameHeaderSize;
            } else {
                (*dctx).expected = cBlockSize;
                (*dctx).bType = bp.blockType;
                (*dctx).stage = ZSTDds_decompressBlock;
            }
            return 0 as std::ffi::c_int as size_t;
        }
        3 => {
            let mut rSize: size_t = 0;
            match (*dctx).bType as std::ffi::c_uint {
                0 => {
                    rSize = ZSTDv06_decompressBlock_internal(dctx, dst, dstCapacity, src, srcSize);
                }
                1 => {
                    rSize = ZSTDv06_copyRawBlock(dst, dstCapacity, src, srcSize);
                }
                2 => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
                3 => {
                    rSize = 0 as std::ffi::c_int as size_t;
                }
                _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
            }
            (*dctx).stage = ZSTDds_decodeBlockHeader;
            (*dctx).expected = ZSTDv06_blockHeaderSize;
            if ERR_isError(rSize) != 0 {
                return rSize;
            }
            (*dctx).previousDstEnd =
                (dst as *mut std::ffi::c_char).offset(rSize as isize) as *const std::ffi::c_void;
            return rSize;
        }
        _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
    }
    let mut result: size_t = 0;
    memcpy(
        ((*dctx).headerBuffer)
            .as_mut_ptr()
            .offset(ZSTDv06_frameHeaderSize_min as isize) as *mut std::ffi::c_void,
        src,
        (*dctx).expected,
    );
    result = ZSTDv06_decodeFrameHeader(
        dctx,
        ((*dctx).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
        (*dctx).headerSize,
    );
    if ERR_isError(result) != 0 {
        return result;
    }
    (*dctx).expected = ZSTDv06_blockHeaderSize;
    (*dctx).stage = ZSTDds_decodeBlockHeader;
    0 as std::ffi::c_int as size_t
}
unsafe extern "C" fn ZSTDv06_refDictContent(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) {
    (*dctx).dictEnd = (*dctx).previousDstEnd;
    (*dctx).vBase = (dict as *const std::ffi::c_char).offset(
        -(((*dctx).previousDstEnd as *const std::ffi::c_char)
            .offset_from((*dctx).base as *const std::ffi::c_char) as std::ffi::c_long
            as isize),
    ) as *const std::ffi::c_void;
    (*dctx).base = dict;
    (*dctx).previousDstEnd =
        (dict as *const std::ffi::c_char).offset(dictSize as isize) as *const std::ffi::c_void;
}
unsafe extern "C" fn ZSTDv06_loadEntropy(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut hSize: size_t = 0;
    let mut offcodeHeaderSize: size_t = 0;
    let mut matchlengthHeaderSize: size_t = 0;
    let mut litlengthHeaderSize: size_t = 0;
    hSize = HUFv06_readDTableX4(((*dctx).hufTableX4).as_mut_ptr(), dict, dictSize);
    if ERR_isError(hSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(hSize as isize) as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(hSize);
    let mut offcodeNCount: [std::ffi::c_short; 29] = [0; 29];
    let mut offcodeMaxValue = MaxOff as U32;
    let mut offcodeLog: U32 = 0;
    offcodeHeaderSize = FSEv06_readNCount(
        offcodeNCount.as_mut_ptr(),
        &mut offcodeMaxValue,
        &mut offcodeLog,
        dict,
        dictSize,
    );
    if ERR_isError(offcodeHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if offcodeLog > OffFSELog as U32 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    let errorCode = FSEv06_buildDTable(
        ((*dctx).OffTable).as_mut_ptr(),
        offcodeNCount.as_mut_ptr(),
        offcodeMaxValue,
        offcodeLog,
    );
    if ERR_isError(errorCode) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(offcodeHeaderSize as isize)
        as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(offcodeHeaderSize);
    let mut matchlengthNCount: [std::ffi::c_short; 53] = [0; 53];
    let mut matchlengthMaxValue = MaxML as std::ffi::c_uint;
    let mut matchlengthLog: std::ffi::c_uint = 0;
    matchlengthHeaderSize = FSEv06_readNCount(
        matchlengthNCount.as_mut_ptr(),
        &mut matchlengthMaxValue,
        &mut matchlengthLog,
        dict,
        dictSize,
    );
    if ERR_isError(matchlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if matchlengthLog > MLFSELog as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    let errorCode_0 = FSEv06_buildDTable(
        ((*dctx).MLTable).as_mut_ptr(),
        matchlengthNCount.as_mut_ptr(),
        matchlengthMaxValue,
        matchlengthLog,
    );
    if ERR_isError(errorCode_0) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(matchlengthHeaderSize as isize)
        as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(matchlengthHeaderSize);
    let mut litlengthNCount: [std::ffi::c_short; 36] = [0; 36];
    let mut litlengthMaxValue = MaxLL as std::ffi::c_uint;
    let mut litlengthLog: std::ffi::c_uint = 0;
    litlengthHeaderSize = FSEv06_readNCount(
        litlengthNCount.as_mut_ptr(),
        &mut litlengthMaxValue,
        &mut litlengthLog,
        dict,
        dictSize,
    );
    if ERR_isError(litlengthHeaderSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    if litlengthLog > LLFSELog as std::ffi::c_uint {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    let errorCode_1 = FSEv06_buildDTable(
        ((*dctx).LLTable).as_mut_ptr(),
        litlengthNCount.as_mut_ptr(),
        litlengthMaxValue,
        litlengthLog,
    );
    if ERR_isError(errorCode_1) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    (*dctx).flagRepeatTable = 1 as std::ffi::c_int as U32;
    hSize
        .wrapping_add(offcodeHeaderSize)
        .wrapping_add(matchlengthHeaderSize)
        .wrapping_add(litlengthHeaderSize)
}
unsafe extern "C" fn ZSTDv06_decompress_insertDictionary(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let mut eSize: size_t = 0;
    let magic = MEM_readLE32(dict);
    if magic != ZSTDv06_DICT_MAGIC {
        ZSTDv06_refDictContent(dctx, dict, dictSize);
        return 0 as std::ffi::c_int as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(4 as std::ffi::c_int as isize)
        as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(4 as std::ffi::c_int as size_t);
    eSize = ZSTDv06_loadEntropy(dctx, dict, dictSize);
    if ERR_isError(eSize) != 0 {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    dict = (dict as *const std::ffi::c_char).offset(eSize as isize) as *const std::ffi::c_void;
    dictSize = dictSize.wrapping_sub(eSize);
    ZSTDv06_refDictContent(dctx, dict, dictSize);
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZSTDv06_decompressBegin_usingDict(
    mut dctx: *mut ZSTDv06_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    let errorCode = ZSTDv06_decompressBegin(dctx);
    if ERR_isError(errorCode) != 0 {
        return errorCode;
    }
    if !dict.is_null() && dictSize != 0 {
        let errorCode_0 = ZSTDv06_decompress_insertDictionary(dctx, dict, dictSize);
        if ERR_isError(errorCode_0) != 0 {
            return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
        }
    }
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_createDCtx() -> *mut ZBUFFv06_DCtx {
    let mut zbd =
        malloc(::core::mem::size_of::<ZBUFFv06_DCtx>() as std::ffi::c_ulong) as *mut ZBUFFv06_DCtx;
    if zbd.is_null() {
        return NULL as *mut ZBUFFv06_DCtx;
    }
    memset(
        zbd as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<ZBUFFv06_DCtx>() as std::ffi::c_ulong,
    );
    (*zbd).zd = ZSTDv06_createDCtx();
    if ((*zbd).zd).is_null() {
        ZBUFFv06_freeDCtx(zbd);
        return NULL as *mut ZBUFFv06_DCtx;
    }
    (*zbd).stage = ZBUFFds_init;
    zbd
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_freeDCtx(mut zbd: *mut ZBUFFv06_DCtx) -> size_t {
    if zbd.is_null() {
        return 0 as std::ffi::c_int as size_t;
    }
    ZSTDv06_freeDCtx((*zbd).zd);
    free((*zbd).inBuff as *mut std::ffi::c_void);
    free((*zbd).outBuff as *mut std::ffi::c_void);
    free(zbd as *mut std::ffi::c_void);
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_decompressInitDictionary(
    mut zbd: *mut ZBUFFv06_DCtx,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> size_t {
    (*zbd).stage = ZBUFFds_loadHeader;
    (*zbd).outEnd = 0 as std::ffi::c_int as size_t;
    (*zbd).outStart = (*zbd).outEnd;
    (*zbd).inPos = (*zbd).outStart;
    (*zbd).lhSize = (*zbd).inPos;
    ZSTDv06_decompressBegin_usingDict((*zbd).zd, dict, dictSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_decompressInit(mut zbd: *mut ZBUFFv06_DCtx) -> size_t {
    ZBUFFv06_decompressInitDictionary(
        zbd,
        NULL as *const std::ffi::c_void,
        0 as std::ffi::c_int as size_t,
    )
}
#[inline]
unsafe extern "C" fn ZBUFFv06_limitCopy(
    mut dst: *mut std::ffi::c_void,
    mut dstCapacity: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let mut length = if dstCapacity < srcSize {
        dstCapacity
    } else {
        srcSize
    };
    if length > 0 as std::ffi::c_int as size_t {
        memcpy(dst, src, length);
    }
    length
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_decompressContinue(
    mut zbd: *mut ZBUFFv06_DCtx,
    mut dst: *mut std::ffi::c_void,
    mut dstCapacityPtr: *mut size_t,
    mut src: *const std::ffi::c_void,
    mut srcSizePtr: *mut size_t,
) -> size_t {
    let istart = src as *const std::ffi::c_char;
    let iend = istart.offset(*srcSizePtr as isize);
    let mut ip = istart;
    let ostart = dst as *mut std::ffi::c_char;
    let oend = ostart.offset(*dstCapacityPtr as isize);
    let mut op = ostart;
    let mut notDone = 1 as std::ffi::c_int as U32;
    while notDone != 0 {
        let mut current_block_65: u64;
        match (*zbd).stage as std::ffi::c_uint {
            0 => return -(ZSTD_error_init_missing as std::ffi::c_int) as size_t,
            1 => {
                let hSize = ZSTDv06_getFrameParams(
                    &mut (*zbd).fParams,
                    ((*zbd).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                    (*zbd).lhSize,
                );
                if hSize != 0 as std::ffi::c_int as size_t {
                    let toLoad = hSize.wrapping_sub((*zbd).lhSize);
                    if ERR_isError(hSize) != 0 {
                        return hSize;
                    }
                    if toLoad > iend.offset_from(ip) as std::ffi::c_long as size_t {
                        if !ip.is_null() {
                            memcpy(
                                ((*zbd).headerBuffer)
                                    .as_mut_ptr()
                                    .offset((*zbd).lhSize as isize)
                                    as *mut std::ffi::c_void,
                                ip as *const std::ffi::c_void,
                                iend.offset_from(ip) as std::ffi::c_long as std::ffi::c_ulong,
                            );
                        }
                        (*zbd).lhSize = ((*zbd).lhSize)
                            .wrapping_add(iend.offset_from(ip) as std::ffi::c_long as size_t);
                        *dstCapacityPtr = 0 as std::ffi::c_int as size_t;
                        return hSize
                            .wrapping_sub((*zbd).lhSize)
                            .wrapping_add(ZSTDv06_blockHeaderSize);
                    }
                    memcpy(
                        ((*zbd).headerBuffer)
                            .as_mut_ptr()
                            .offset((*zbd).lhSize as isize)
                            as *mut std::ffi::c_void,
                        ip as *const std::ffi::c_void,
                        toLoad,
                    );
                    (*zbd).lhSize = hSize;
                    ip = ip.offset(toLoad as isize);
                    current_block_65 = 13853033528615664019;
                } else {
                    let h1Size = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
                    let h1Result = ZSTDv06_decompressContinue(
                        (*zbd).zd,
                        NULL as *mut std::ffi::c_void,
                        0 as std::ffi::c_int as size_t,
                        ((*zbd).headerBuffer).as_mut_ptr() as *const std::ffi::c_void,
                        h1Size,
                    );
                    if ERR_isError(h1Result) != 0 {
                        return h1Result;
                    }
                    if h1Size < (*zbd).lhSize {
                        let h2Size = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
                        let h2Result = ZSTDv06_decompressContinue(
                            (*zbd).zd,
                            NULL as *mut std::ffi::c_void,
                            0 as std::ffi::c_int as size_t,
                            ((*zbd).headerBuffer).as_mut_ptr().offset(h1Size as isize)
                                as *const std::ffi::c_void,
                            h2Size,
                        );
                        if ERR_isError(h2Result) != 0 {
                            return h2Result;
                        }
                    }
                    let blockSize = (if (1 as std::ffi::c_int) << (*zbd).fParams.windowLog
                        < 128 as std::ffi::c_int * 1024 as std::ffi::c_int
                    {
                        (1 as std::ffi::c_int) << (*zbd).fParams.windowLog
                    } else {
                        128 as std::ffi::c_int * 1024 as std::ffi::c_int
                    }) as size_t;
                    (*zbd).blockSize = blockSize;
                    if (*zbd).inBuffSize < blockSize {
                        free((*zbd).inBuff as *mut std::ffi::c_void);
                        (*zbd).inBuffSize = blockSize;
                        (*zbd).inBuff = malloc(blockSize) as *mut std::ffi::c_char;
                        if ((*zbd).inBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                        }
                    }
                    let neededOutSize = ((1 as std::ffi::c_int as size_t)
                        << (*zbd).fParams.windowLog)
                        .wrapping_add(blockSize)
                        .wrapping_add((WILDCOPY_OVERLENGTH * 2 as std::ffi::c_int) as size_t);
                    if (*zbd).outBuffSize < neededOutSize {
                        free((*zbd).outBuff as *mut std::ffi::c_void);
                        (*zbd).outBuffSize = neededOutSize;
                        (*zbd).outBuff = malloc(neededOutSize) as *mut std::ffi::c_char;
                        if ((*zbd).outBuff).is_null() {
                            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
                        }
                    }
                    (*zbd).stage = ZBUFFds_read;
                    current_block_65 = 11048769245176032998;
                }
            }
            2 => {
                current_block_65 = 11048769245176032998;
            }
            3 => {
                current_block_65 = 14220266465818359136;
            }
            4 => {
                current_block_65 = 15594603006322722090;
            }
            _ => return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t,
        }
        if current_block_65 == 11048769245176032998 {
            let neededInSize = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
            if neededInSize == 0 as std::ffi::c_int as size_t {
                (*zbd).stage = ZBUFFds_init;
                notDone = 0 as std::ffi::c_int as U32;
                current_block_65 = 13853033528615664019;
            } else if iend.offset_from(ip) as std::ffi::c_long as size_t >= neededInSize {
                let decodedSize = ZSTDv06_decompressContinue(
                    (*zbd).zd,
                    ((*zbd).outBuff).offset((*zbd).outStart as isize) as *mut std::ffi::c_void,
                    ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart),
                    ip as *const std::ffi::c_void,
                    neededInSize,
                );
                if ERR_isError(decodedSize) != 0 {
                    return decodedSize;
                }
                ip = ip.offset(neededInSize as isize);
                if decodedSize == 0 {
                    current_block_65 = 13853033528615664019;
                } else {
                    (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize);
                    (*zbd).stage = ZBUFFds_flush;
                    current_block_65 = 13853033528615664019;
                }
            } else if ip == iend {
                notDone = 0 as std::ffi::c_int as U32;
                current_block_65 = 13853033528615664019;
            } else {
                (*zbd).stage = ZBUFFds_load;
                current_block_65 = 14220266465818359136;
            }
        }
        if current_block_65 == 14220266465818359136 {
            let neededInSize_0 = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
            let toLoad_0 = neededInSize_0.wrapping_sub((*zbd).inPos);
            let mut loadedSize: size_t = 0;
            if toLoad_0 > ((*zbd).inBuffSize).wrapping_sub((*zbd).inPos) {
                return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
            }
            loadedSize = ZBUFFv06_limitCopy(
                ((*zbd).inBuff).offset((*zbd).inPos as isize) as *mut std::ffi::c_void,
                toLoad_0,
                ip as *const std::ffi::c_void,
                iend.offset_from(ip) as std::ffi::c_long as size_t,
            );
            ip = ip.offset(loadedSize as isize);
            (*zbd).inPos = ((*zbd).inPos).wrapping_add(loadedSize);
            if loadedSize < toLoad_0 {
                notDone = 0 as std::ffi::c_int as U32;
                current_block_65 = 13853033528615664019;
            } else {
                let decodedSize_0 = ZSTDv06_decompressContinue(
                    (*zbd).zd,
                    ((*zbd).outBuff).offset((*zbd).outStart as isize) as *mut std::ffi::c_void,
                    ((*zbd).outBuffSize).wrapping_sub((*zbd).outStart),
                    (*zbd).inBuff as *const std::ffi::c_void,
                    neededInSize_0,
                );
                if ERR_isError(decodedSize_0) != 0 {
                    return decodedSize_0;
                }
                (*zbd).inPos = 0 as std::ffi::c_int as size_t;
                if decodedSize_0 == 0 {
                    (*zbd).stage = ZBUFFds_read;
                    current_block_65 = 13853033528615664019;
                } else {
                    (*zbd).outEnd = ((*zbd).outStart).wrapping_add(decodedSize_0);
                    (*zbd).stage = ZBUFFds_flush;
                    current_block_65 = 15594603006322722090;
                }
            }
        }
        if current_block_65 == 15594603006322722090 {
            let toFlushSize = ((*zbd).outEnd).wrapping_sub((*zbd).outStart);
            let flushedSize = ZBUFFv06_limitCopy(
                op as *mut std::ffi::c_void,
                oend.offset_from(op) as std::ffi::c_long as size_t,
                ((*zbd).outBuff).offset((*zbd).outStart as isize) as *const std::ffi::c_void,
                toFlushSize,
            );
            op = op.offset(flushedSize as isize);
            (*zbd).outStart = ((*zbd).outStart).wrapping_add(flushedSize);
            if flushedSize == toFlushSize {
                (*zbd).stage = ZBUFFds_read;
                if ((*zbd).outStart).wrapping_add((*zbd).blockSize) > (*zbd).outBuffSize {
                    (*zbd).outEnd = 0 as std::ffi::c_int as size_t;
                    (*zbd).outStart = (*zbd).outEnd;
                }
            } else {
                notDone = 0 as std::ffi::c_int as U32;
            }
        }
    }
    *srcSizePtr = ip.offset_from(istart) as std::ffi::c_long as size_t;
    *dstCapacityPtr = op.offset_from(ostart) as std::ffi::c_long as size_t;
    let mut nextSrcSizeHint = ZSTDv06_nextSrcSizeToDecompress((*zbd).zd);
    if nextSrcSizeHint > ZSTDv06_blockHeaderSize {
        nextSrcSizeHint = nextSrcSizeHint.wrapping_add(ZSTDv06_blockHeaderSize);
    }
    nextSrcSizeHint = nextSrcSizeHint.wrapping_sub((*zbd).inPos);
    nextSrcSizeHint
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_recommendedDInSize() -> size_t {
    (ZSTDv06_BLOCKSIZE_MAX as size_t).wrapping_add(ZSTDv06_blockHeaderSize)
}
#[no_mangle]
pub unsafe extern "C" fn ZBUFFv06_recommendedDOutSize() -> size_t {
    ZSTDv06_BLOCKSIZE_MAX as size_t
}
