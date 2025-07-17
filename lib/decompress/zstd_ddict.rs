use ::libc;
extern "C" {
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn ZSTD_loadDEntropy(
        entropy: *mut ZSTD_entropyDTables_t,
        dict: *const std::ffi::c_void,
        dictSize: size_t,
    ) -> size_t;
}
pub type size_t = std::ffi::c_ulong;
pub type __uint8_t = std::ffi::c_uchar;
pub type __uint16_t = std::ffi::c_ushort;
pub type __uint32_t = std::ffi::c_uint;
pub type __uint64_t = std::ffi::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DCtx_s {
    pub LLTptr: *const ZSTD_seqSymbol,
    pub MLTptr: *const ZSTD_seqSymbol,
    pub OFTptr: *const ZSTD_seqSymbol,
    pub HUFptr: *const HUF_DTable,
    pub entropy: ZSTD_entropyDTables_t,
    pub workspace: [U32; 640],
    pub previousDstEnd: *const std::ffi::c_void,
    pub prefixStart: *const std::ffi::c_void,
    pub virtualStart: *const std::ffi::c_void,
    pub dictEnd: *const std::ffi::c_void,
    pub expected: size_t,
    pub fParams: ZSTD_FrameHeader,
    pub processedCSize: U64,
    pub decodedSize: U64,
    pub bType: blockType_e,
    pub stage: ZSTD_dStage,
    pub litEntropy: U32,
    pub fseEntropy: U32,
    pub xxhState: XXH64_state_t,
    pub headerSize: size_t,
    pub format: ZSTD_format_e,
    pub forceIgnoreChecksum: ZSTD_forceIgnoreChecksum_e,
    pub validateChecksum: U32,
    pub litPtr: *const BYTE,
    pub customMem: ZSTD_customMem,
    pub litSize: size_t,
    pub rleSize: size_t,
    pub staticSize: size_t,
    pub isFrameDecompression: std::ffi::c_int,
    pub bmi2: std::ffi::c_int,
    pub ddictLocal: *mut ZSTD_DDict,
    pub ddict: *const ZSTD_DDict,
    pub dictID: U32,
    pub ddictIsCold: std::ffi::c_int,
    pub dictUses: ZSTD_dictUses_e,
    pub ddictSet: *mut ZSTD_DDictHashSet,
    pub refMultipleDDicts: ZSTD_refMultipleDDicts_e,
    pub disableHufAsm: std::ffi::c_int,
    pub maxBlockSizeParam: std::ffi::c_int,
    pub streamStage: ZSTD_dStreamStage,
    pub inBuff: *mut std::ffi::c_char,
    pub inBuffSize: size_t,
    pub inPos: size_t,
    pub maxWindowSize: size_t,
    pub outBuff: *mut std::ffi::c_char,
    pub outBuffSize: size_t,
    pub outStart: size_t,
    pub outEnd: size_t,
    pub lhSize: size_t,
    pub legacyContext: *mut std::ffi::c_void,
    pub previousLegacyVersion: U32,
    pub legacyVersion: U32,
    pub hostageByte: U32,
    pub noForwardProgress: std::ffi::c_int,
    pub outBufferMode: ZSTD_bufferMode_e,
    pub expectedOutBuffer: ZSTD_outBuffer,
    pub litBuffer: *mut BYTE,
    pub litBufferEnd: *const BYTE,
    pub litBufferLocation: ZSTD_litLocation_e,
    pub litExtraBuffer: [BYTE; 65568],
    pub headerBuffer: [BYTE; 18],
    pub oversizedDuration: size_t,
    pub traceCtx: ZSTD_TraceCtx,
}
pub type ZSTD_TraceCtx = std::ffi::c_ulonglong;
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type ZSTD_litLocation_e = std::ffi::c_uint;
pub const ZSTD_split: ZSTD_litLocation_e = 2;
pub const ZSTD_in_dst: ZSTD_litLocation_e = 1;
pub const ZSTD_not_in_dst: ZSTD_litLocation_e = 0;
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut std::ffi::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_bufferMode_e = std::ffi::c_uint;
pub const ZSTD_bm_stable: ZSTD_bufferMode_e = 1;
pub const ZSTD_bm_buffered: ZSTD_bufferMode_e = 0;
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type ZSTD_dStreamStage = std::ffi::c_uint;
pub const zdss_flush: ZSTD_dStreamStage = 4;
pub const zdss_load: ZSTD_dStreamStage = 3;
pub const zdss_read: ZSTD_dStreamStage = 2;
pub const zdss_loadHeader: ZSTD_dStreamStage = 1;
pub const zdss_init: ZSTD_dStreamStage = 0;
pub type ZSTD_refMultipleDDicts_e = std::ffi::c_uint;
pub const ZSTD_rmd_refMultipleDDicts: ZSTD_refMultipleDDicts_e = 1;
pub const ZSTD_rmd_refSingleDDict: ZSTD_refMultipleDDicts_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DDictHashSet {
    pub ddictPtrTable: *mut *const ZSTD_DDict,
    pub ddictPtrTableSize: size_t,
    pub ddictPtrCount: size_t,
}
pub type ZSTD_DDict = ZSTD_DDict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_DDict_s {
    pub dictBuffer: *mut std::ffi::c_void,
    pub dictContent: *const std::ffi::c_void,
    pub dictSize: size_t,
    pub entropy: ZSTD_entropyDTables_t,
    pub dictID: U32,
    pub entropyPresent: U32,
    pub cMem: ZSTD_customMem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_customMem {
    pub customAlloc: ZSTD_allocFunction,
    pub customFree: ZSTD_freeFunction,
    pub opaque: *mut std::ffi::c_void,
}
pub type ZSTD_freeFunction =
    Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void) -> ()>;
pub type ZSTD_allocFunction =
    Option<unsafe extern "C" fn(*mut std::ffi::c_void, size_t) -> *mut std::ffi::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyDTables_t {
    pub LLTable: [ZSTD_seqSymbol; 513],
    pub OFTable: [ZSTD_seqSymbol; 257],
    pub MLTable: [ZSTD_seqSymbol; 513],
    pub hufTable: [HUF_DTable; 4097],
    pub rep: [U32; 3],
    pub workspace: [U32; 157],
}
pub type HUF_DTable = U32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_seqSymbol {
    pub nextState: U16,
    pub nbAdditionalBits: BYTE,
    pub nbBits: BYTE,
    pub baseValue: U32,
}
pub type U16 = uint16_t;
pub type uint16_t = __uint16_t;
pub type ZSTD_dictUses_e = std::ffi::c_int;
pub const ZSTD_use_once: ZSTD_dictUses_e = 1;
pub const ZSTD_dont_use: ZSTD_dictUses_e = 0;
pub const ZSTD_use_indefinitely: ZSTD_dictUses_e = -1;
pub type ZSTD_forceIgnoreChecksum_e = std::ffi::c_uint;
pub const ZSTD_d_ignoreChecksum: ZSTD_forceIgnoreChecksum_e = 1;
pub const ZSTD_d_validateChecksum: ZSTD_forceIgnoreChecksum_e = 0;
pub type ZSTD_format_e = std::ffi::c_uint;
pub const ZSTD_f_zstd1_magicless: ZSTD_format_e = 1;
pub const ZSTD_f_zstd1: ZSTD_format_e = 0;
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
pub type XXH64_hash_t = uint64_t;
pub type uint64_t = __uint64_t;
pub type XXH32_hash_t = uint32_t;
pub type ZSTD_dStage = std::ffi::c_uint;
pub const ZSTDds_skipFrame: ZSTD_dStage = 7;
pub const ZSTDds_decodeSkippableHeader: ZSTD_dStage = 6;
pub const ZSTDds_checkChecksum: ZSTD_dStage = 5;
pub const ZSTDds_decompressLastBlock: ZSTD_dStage = 4;
pub const ZSTDds_decompressBlock: ZSTD_dStage = 3;
pub const ZSTDds_decodeBlockHeader: ZSTD_dStage = 2;
pub const ZSTDds_decodeFrameHeader: ZSTD_dStage = 1;
pub const ZSTDds_getFrameHeaderSize: ZSTD_dStage = 0;
pub type blockType_e = std::ffi::c_uint;
pub const bt_reserved: blockType_e = 3;
pub const bt_compressed: blockType_e = 2;
pub const bt_rle: blockType_e = 1;
pub const bt_raw: blockType_e = 0;
pub type U64 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_FrameHeader {
    pub frameContentSize: std::ffi::c_ulonglong,
    pub windowSize: std::ffi::c_ulonglong,
    pub blockSizeMax: std::ffi::c_uint,
    pub frameType: ZSTD_FrameType_e,
    pub headerSize: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
    pub checksumFlag: std::ffi::c_uint,
    pub _reserved1: std::ffi::c_uint,
    pub _reserved2: std::ffi::c_uint,
}
pub type ZSTD_FrameType_e = std::ffi::c_uint;
pub const ZSTD_skippableFrame: ZSTD_FrameType_e = 1;
pub const ZSTD_frame: ZSTD_FrameType_e = 0;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
pub type ZSTD_dictContentType_e = std::ffi::c_uint;
pub const ZSTD_dct_fullDict: ZSTD_dictContentType_e = 2;
pub const ZSTD_dct_rawContent: ZSTD_dictContentType_e = 1;
pub const ZSTD_dct_auto: ZSTD_dictContentType_e = 0;
pub type ZSTD_dictLoadMethod_e = std::ffi::c_uint;
pub const ZSTD_dlm_byRef: ZSTD_dictLoadMethod_e = 1;
pub const ZSTD_dlm_byCopy: ZSTD_dictLoadMethod_e = 0;
pub type unalign32 = U32;
pub const ZSTD_MAGIC_DICTIONARY: std::ffi::c_uint = 0xec30a437 as std::ffi::c_uint;
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    return 1 as std::ffi::c_int as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn MEM_read32(mut ptr: *const std::ffi::c_void) -> U32 {
    return *(ptr as *const unalign32);
}
#[inline]
unsafe extern "C" fn MEM_swap32(mut in_0: U32) -> U32 {
    return in_0.swap_bytes();
}
#[inline]
unsafe extern "C" fn MEM_readLE32(mut memPtr: *const std::ffi::c_void) -> U32 {
    if MEM_isLittleEndian() != 0 {
        return MEM_read32(memPtr);
    } else {
        return MEM_swap32(MEM_read32(memPtr));
    };
}
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
pub const ZSTD_FRAMEIDSIZE: std::ffi::c_int = 4 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn ZSTD_customMalloc(
    mut size: size_t,
    mut customMem: ZSTD_customMem,
) -> *mut std::ffi::c_void {
    if (customMem.customAlloc).is_some() {
        return (customMem.customAlloc).unwrap_unchecked()(customMem.opaque, size);
    }
    return malloc(size);
}
#[inline]
unsafe extern "C" fn ZSTD_customFree(
    mut ptr: *mut std::ffi::c_void,
    mut customMem: ZSTD_customMem,
) {
    if !ptr.is_null() {
        if (customMem.customFree).is_some() {
            (customMem.customFree).unwrap_unchecked()(customMem.opaque, ptr);
        } else {
            free(ptr);
        }
    }
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    return (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn _force_has_format_string(mut format: *const std::ffi::c_char, mut args: ...) {}
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DDict_dictContent(
    mut ddict: *const ZSTD_DDict,
) -> *const std::ffi::c_void {
    return (*ddict).dictContent;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_DDict_dictSize(mut ddict: *const ZSTD_DDict) -> size_t {
    return (*ddict).dictSize;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_copyDDictParameters(
    mut dctx: *mut ZSTD_DCtx,
    mut ddict: *const ZSTD_DDict,
) {
    (*dctx).dictID = (*ddict).dictID;
    (*dctx).prefixStart = (*ddict).dictContent;
    (*dctx).virtualStart = (*ddict).dictContent;
    (*dctx).dictEnd = ((*ddict).dictContent as *const BYTE).offset((*ddict).dictSize as isize)
        as *const std::ffi::c_void;
    (*dctx).previousDstEnd = (*dctx).dictEnd;
    if (*ddict).entropyPresent != 0 {
        (*dctx).litEntropy = 1 as std::ffi::c_int as U32;
        (*dctx).fseEntropy = 1 as std::ffi::c_int as U32;
        (*dctx).LLTptr = ((*ddict).entropy.LLTable).as_ptr();
        (*dctx).MLTptr = ((*ddict).entropy.MLTable).as_ptr();
        (*dctx).OFTptr = ((*ddict).entropy.OFTable).as_ptr();
        (*dctx).HUFptr = ((*ddict).entropy.hufTable).as_ptr();
        *((*dctx).entropy.rep)
            .as_mut_ptr()
            .offset(0 as std::ffi::c_int as isize) = *((*ddict).entropy.rep)
            .as_ptr()
            .offset(0 as std::ffi::c_int as isize);
        *((*dctx).entropy.rep)
            .as_mut_ptr()
            .offset(1 as std::ffi::c_int as isize) = *((*ddict).entropy.rep)
            .as_ptr()
            .offset(1 as std::ffi::c_int as isize);
        *((*dctx).entropy.rep)
            .as_mut_ptr()
            .offset(2 as std::ffi::c_int as isize) = *((*ddict).entropy.rep)
            .as_ptr()
            .offset(2 as std::ffi::c_int as isize);
    } else {
        (*dctx).litEntropy = 0 as std::ffi::c_int as U32;
        (*dctx).fseEntropy = 0 as std::ffi::c_int as U32;
    };
}
unsafe extern "C" fn ZSTD_loadEntropy_intoDDict(
    mut ddict: *mut ZSTD_DDict,
    mut dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    (*ddict).dictID = 0 as std::ffi::c_int as U32;
    (*ddict).entropyPresent = 0 as std::ffi::c_int as U32;
    if dictContentType as std::ffi::c_uint
        == ZSTD_dct_rawContent as std::ffi::c_int as std::ffi::c_uint
    {
        return 0 as std::ffi::c_int as size_t;
    }
    if (*ddict).dictSize < 8 as std::ffi::c_int as size_t {
        if dictContentType as std::ffi::c_uint
            == ZSTD_dct_fullDict as std::ffi::c_int as std::ffi::c_uint
        {
            return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
        }
        return 0 as std::ffi::c_int as size_t;
    }
    let magic = MEM_readLE32((*ddict).dictContent);
    if magic != ZSTD_MAGIC_DICTIONARY {
        if dictContentType as std::ffi::c_uint
            == ZSTD_dct_fullDict as std::ffi::c_int as std::ffi::c_uint
        {
            return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
        }
        return 0 as std::ffi::c_int as size_t;
    }
    (*ddict).dictID = MEM_readLE32(
        ((*ddict).dictContent as *const std::ffi::c_char).offset(ZSTD_FRAMEIDSIZE as isize)
            as *const std::ffi::c_void,
    );
    if ERR_isError(ZSTD_loadDEntropy(
        &mut (*ddict).entropy,
        (*ddict).dictContent,
        (*ddict).dictSize,
    )) != 0
    {
        return -(ZSTD_error_dictionary_corrupted as std::ffi::c_int) as size_t;
    }
    (*ddict).entropyPresent = 1 as std::ffi::c_int as U32;
    return 0 as std::ffi::c_int as size_t;
}
unsafe extern "C" fn ZSTD_initDDict_internal(
    mut ddict: *mut ZSTD_DDict,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
) -> size_t {
    if dictLoadMethod as std::ffi::c_uint == ZSTD_dlm_byRef as std::ffi::c_int as std::ffi::c_uint
        || dict.is_null()
        || dictSize == 0
    {
        (*ddict).dictBuffer = NULL as *mut std::ffi::c_void;
        (*ddict).dictContent = dict;
        if dict.is_null() {
            dictSize = 0 as std::ffi::c_int as size_t;
        }
    } else {
        let internalBuffer = ZSTD_customMalloc(dictSize, (*ddict).cMem);
        (*ddict).dictBuffer = internalBuffer;
        (*ddict).dictContent = internalBuffer;
        if internalBuffer.is_null() {
            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        }
        libc::memcpy(internalBuffer, dict, dictSize as libc::size_t);
    }
    (*ddict).dictSize = dictSize;
    *((*ddict).entropy.hufTable)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) =
        (12 as std::ffi::c_int * 0x1000001 as std::ffi::c_int) as HUF_DTable;
    let err_code = ZSTD_loadEntropy_intoDDict(ddict, dictContentType);
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    return 0 as std::ffi::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_advanced(
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
    mut customMem: ZSTD_customMem,
) -> *mut ZSTD_DDict {
    if (customMem.customAlloc).is_none() as std::ffi::c_int
        ^ (customMem.customFree).is_none() as std::ffi::c_int
        != 0
    {
        return NULL as *mut ZSTD_DDict;
    }
    let ddict = ZSTD_customMalloc(
        ::core::mem::size_of::<ZSTD_DDict>() as std::ffi::c_ulong,
        customMem,
    ) as *mut ZSTD_DDict;
    if ddict.is_null() {
        return NULL as *mut ZSTD_DDict;
    }
    (*ddict).cMem = customMem;
    let initResult =
        ZSTD_initDDict_internal(ddict, dict, dictSize, dictLoadMethod, dictContentType);
    if ERR_isError(initResult) != 0 {
        ZSTD_freeDDict(ddict);
        return NULL as *mut ZSTD_DDict;
    }
    return ddict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict(
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> *mut ZSTD_DDict {
    let allocator = {
        let mut init = ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<libc::intptr_t, ZSTD_allocFunction>(
                NULL as libc::intptr_t,
            ),
            customFree: ::core::mem::transmute::<libc::intptr_t, ZSTD_freeFunction>(
                NULL as libc::intptr_t,
            ),
            opaque: NULL as *mut std::ffi::c_void,
        };
        init
    };
    return ZSTD_createDDict_advanced(dict, dictSize, ZSTD_dlm_byCopy, ZSTD_dct_auto, allocator);
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_createDDict_byReference(
    mut dictBuffer: *const std::ffi::c_void,
    mut dictSize: size_t,
) -> *mut ZSTD_DDict {
    let allocator = {
        let mut init = ZSTD_customMem {
            customAlloc: ::core::mem::transmute::<libc::intptr_t, ZSTD_allocFunction>(
                NULL as libc::intptr_t,
            ),
            customFree: ::core::mem::transmute::<libc::intptr_t, ZSTD_freeFunction>(
                NULL as libc::intptr_t,
            ),
            opaque: NULL as *mut std::ffi::c_void,
        };
        init
    };
    return ZSTD_createDDict_advanced(
        dictBuffer,
        dictSize,
        ZSTD_dlm_byRef,
        ZSTD_dct_auto,
        allocator,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_initStaticDDict(
    mut sBuffer: *mut std::ffi::c_void,
    mut sBufferSize: size_t,
    mut dict: *const std::ffi::c_void,
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
    mut dictContentType: ZSTD_dictContentType_e,
) -> *const ZSTD_DDict {
    let neededSpace = (::core::mem::size_of::<ZSTD_DDict>() as std::ffi::c_ulong).wrapping_add(
        (if dictLoadMethod as std::ffi::c_uint
            == ZSTD_dlm_byRef as std::ffi::c_int as std::ffi::c_uint
        {
            0 as std::ffi::c_int as size_t
        } else {
            dictSize
        }),
    );
    let ddict = sBuffer as *mut ZSTD_DDict;
    if sBuffer as size_t & 7 as std::ffi::c_int as size_t != 0 {
        return NULL as *const ZSTD_DDict;
    }
    if sBufferSize < neededSpace {
        return NULL as *const ZSTD_DDict;
    }
    if dictLoadMethod as std::ffi::c_uint == ZSTD_dlm_byCopy as std::ffi::c_int as std::ffi::c_uint
    {
        libc::memcpy(
            ddict.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
            dict,
            dictSize as libc::size_t,
        );
        dict = ddict.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    }
    if ERR_isError(ZSTD_initDDict_internal(
        ddict,
        dict,
        dictSize,
        ZSTD_dlm_byRef,
        dictContentType,
    )) != 0
    {
        return NULL as *const ZSTD_DDict;
    }
    return ddict;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_freeDDict(mut ddict: *mut ZSTD_DDict) -> size_t {
    if ddict.is_null() {
        return 0 as std::ffi::c_int as size_t;
    }
    let cMem = (*ddict).cMem;
    ZSTD_customFree((*ddict).dictBuffer, cMem);
    ZSTD_customFree(ddict as *mut std::ffi::c_void, cMem);
    return 0 as std::ffi::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_estimateDDictSize(
    mut dictSize: size_t,
    mut dictLoadMethod: ZSTD_dictLoadMethod_e,
) -> size_t {
    return (::core::mem::size_of::<ZSTD_DDict>() as std::ffi::c_ulong).wrapping_add(
        (if dictLoadMethod as std::ffi::c_uint
            == ZSTD_dlm_byRef as std::ffi::c_int as std::ffi::c_uint
        {
            0 as std::ffi::c_int as size_t
        } else {
            dictSize
        }),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_sizeof_DDict(mut ddict: *const ZSTD_DDict) -> size_t {
    if ddict.is_null() {
        return 0 as std::ffi::c_int as size_t;
    }
    return (::core::mem::size_of::<ZSTD_DDict>() as std::ffi::c_ulong).wrapping_add(
        (if !((*ddict).dictBuffer).is_null() {
            (*ddict).dictSize
        } else {
            0 as std::ffi::c_int as size_t
        }),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_getDictID_fromDDict(
    mut ddict: *const ZSTD_DDict,
) -> std::ffi::c_uint {
    if ddict.is_null() {
        return 0 as std::ffi::c_int as std::ffi::c_uint;
    }
    return (*ddict).dictID;
}
