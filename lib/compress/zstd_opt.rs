use ::libc;
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};
extern "C" {
    fn ZSTD_resetSeqStore(ssPtr: *mut SeqStore_t);
    fn HUF_getNbBitsFromCTable(symbolTable: *const HUF_CElt, symbolValue: U32) -> U32;
    fn HIST_count_simple(
        count: *mut std::ffi::c_uint,
        maxSymbolValuePtr: *mut std::ffi::c_uint,
        src: *const std::ffi::c_void,
        srcSize: size_t,
    ) -> std::ffi::c_uint;
}
pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
pub type __uint8_t = std::ffi::c_uchar;
pub type __uint16_t = std::ffi::c_ushort;
pub type __uint32_t = std::ffi::c_uint;
pub type __uint64_t = std::ffi::c_ulong;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type BYTE = uint8_t;
pub type U8 = uint8_t;
pub type U16 = uint16_t;
pub type U32 = uint32_t;
pub type U64 = uint64_t;
pub type unalign16 = U16;
pub type unalign32 = U32;
pub type unalign64 = U64;
pub type unalignArch = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqStore_t {
    pub sequencesStart: *mut SeqDef,
    pub sequences: *mut SeqDef,
    pub litStart: *mut BYTE,
    pub lit: *mut BYTE,
    pub llCode: *mut BYTE,
    pub mlCode: *mut BYTE,
    pub ofCode: *mut BYTE,
    pub maxNbSeq: size_t,
    pub maxNbLit: size_t,
    pub longLengthType: ZSTD_longLengthType_e,
    pub longLengthPos: U32,
}
pub type ZSTD_longLengthType_e = std::ffi::c_uint;
pub const ZSTD_llt_matchLength: ZSTD_longLengthType_e = 2;
pub const ZSTD_llt_literalLength: ZSTD_longLengthType_e = 1;
pub const ZSTD_llt_none: ZSTD_longLengthType_e = 0;
pub type SeqDef = SeqDef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SeqDef_s {
    pub offBase: U32,
    pub litLength: U16,
    pub mlBase: U16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_MatchState_t {
    pub window: ZSTD_window_t,
    pub loadedDictEnd: U32,
    pub nextToUpdate: U32,
    pub hashLog3: U32,
    pub rowHashLog: U32,
    pub tagTable: *mut BYTE,
    pub hashCache: [U32; 8],
    pub hashSalt: U64,
    pub hashSaltEntropy: U32,
    pub hashTable: *mut U32,
    pub hashTable3: *mut U32,
    pub chainTable: *mut U32,
    pub forceNonContiguous: std::ffi::c_int,
    pub dedicatedDictSearch: std::ffi::c_int,
    pub opt: optState_t,
    pub dictMatchState: *const ZSTD_MatchState_t,
    pub cParams: ZSTD_compressionParameters,
    pub ldmSeqStore: *const RawSeqStore_t,
    pub prefetchCDictTables: std::ffi::c_int,
    pub lazySkipping: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RawSeqStore_t {
    pub seq: *mut rawSeq,
    pub pos: size_t,
    pub posInSequence: size_t,
    pub size: size_t,
    pub capacity: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rawSeq {
    pub offset: U32,
    pub litLength: U32,
    pub matchLength: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_compressionParameters {
    pub windowLog: std::ffi::c_uint,
    pub chainLog: std::ffi::c_uint,
    pub hashLog: std::ffi::c_uint,
    pub searchLog: std::ffi::c_uint,
    pub minMatch: std::ffi::c_uint,
    pub targetLength: std::ffi::c_uint,
    pub strategy: ZSTD_strategy,
}
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
pub struct optState_t {
    pub litFreq: *mut std::ffi::c_uint,
    pub litLengthFreq: *mut std::ffi::c_uint,
    pub matchLengthFreq: *mut std::ffi::c_uint,
    pub offCodeFreq: *mut std::ffi::c_uint,
    pub matchTable: *mut ZSTD_match_t,
    pub priceTable: *mut ZSTD_optimal_t,
    pub litSum: U32,
    pub litLengthSum: U32,
    pub matchLengthSum: U32,
    pub offCodeSum: U32,
    pub litSumBasePrice: U32,
    pub litLengthSumBasePrice: U32,
    pub matchLengthSumBasePrice: U32,
    pub offCodeSumBasePrice: U32,
    pub priceType: ZSTD_OptPrice_e,
    pub symbolCosts: *const ZSTD_entropyCTables_t,
    pub literalCompressionMode: ZSTD_ParamSwitch_e,
}
pub type ZSTD_ParamSwitch_e = std::ffi::c_uint;
pub const ZSTD_ps_disable: ZSTD_ParamSwitch_e = 2;
pub const ZSTD_ps_enable: ZSTD_ParamSwitch_e = 1;
pub const ZSTD_ps_auto: ZSTD_ParamSwitch_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
pub type FSE_repeat = std::ffi::c_uint;
pub const FSE_repeat_valid: FSE_repeat = 2;
pub const FSE_repeat_check: FSE_repeat = 1;
pub const FSE_repeat_none: FSE_repeat = 0;
pub type FSE_CTable = std::ffi::c_uint;
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
pub type ZSTD_OptPrice_e = std::ffi::c_uint;
pub const zop_predef: ZSTD_OptPrice_e = 1;
pub const zop_dynamic: ZSTD_OptPrice_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optimal_t {
    pub price: std::ffi::c_int,
    pub off: U32,
    pub mlen: U32,
    pub litlen: U32,
    pub rep: [U32; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: U32,
    pub len: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_window_t {
    pub nextSrc: *const BYTE,
    pub base: *const BYTE,
    pub dictBase: *const BYTE,
    pub dictLimit: U32,
    pub lowLimit: U32,
    pub nbOverflowCorrections: U32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_CState_t {
    pub value: ptrdiff_t,
    pub stateTable: *const std::ffi::c_void,
    pub symbolTT: *const std::ffi::c_void,
    pub stateLog: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FSE_symbolCompressionTransform {
    pub deltaFindState: std::ffi::c_int,
    pub deltaNbBits: U32,
}
pub type ZSTD_overlap_e = std::ffi::c_uint;
pub const ZSTD_overlap_src_before_dst: ZSTD_overlap_e = 1;
pub const ZSTD_no_overlap: ZSTD_overlap_e = 0;
pub type ZSTD_dictMode_e = std::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repcodes_s {
    pub rep: [U32; 3],
}
pub type Repcodes_t = repcodes_s;
pub type ZSTD_getAllMatchesFn = Option<
    unsafe extern "C" fn(
        *mut ZSTD_match_t,
        *mut ZSTD_MatchState_t,
        *mut U32,
        *const BYTE,
        *const BYTE,
        *const U32,
        U32,
        U32,
    ) -> U32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_optLdm_t {
    pub seqStore: RawSeqStore_t,
    pub startPosInBlock: U32,
    pub endPosInBlock: U32,
    pub offset: U32,
}
pub type base_directive_e = std::ffi::c_uint;
pub const base_1guaranteed: base_directive_e = 1;
pub const base_0possible: base_directive_e = 0;
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 8 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
use crate::{MEM_isLittleEndian, MEM_read16, MEM_read32, MEM_readLE32, MEM_readLE64, MEM_readST};
pub const ZSTD_BLOCKSIZELOG_MAX: std::ffi::c_int = 17 as std::ffi::c_int;
pub const ZSTD_BLOCKSIZE_MAX: std::ffi::c_int = (1 as std::ffi::c_int) << ZSTD_BLOCKSIZELOG_MAX;
static mut kNullRawSeqStore: RawSeqStore_t = {
    RawSeqStore_t {
        seq: NULL as *mut rawSeq,
        pos: 0 as std::ffi::c_int as size_t,
        posInSequence: 0 as std::ffi::c_int as size_t,
        size: 0 as std::ffi::c_int as size_t,
        capacity: 0 as std::ffi::c_int as size_t,
    }
};
#[inline]
unsafe extern "C" fn ZSTD_LLcode(mut litLength: U32) -> U32 {
    static mut LL_Code: [BYTE; 64] = [
        0 as std::ffi::c_int as BYTE,
        1 as std::ffi::c_int as BYTE,
        2 as std::ffi::c_int as BYTE,
        3 as std::ffi::c_int as BYTE,
        4 as std::ffi::c_int as BYTE,
        5 as std::ffi::c_int as BYTE,
        6 as std::ffi::c_int as BYTE,
        7 as std::ffi::c_int as BYTE,
        8 as std::ffi::c_int as BYTE,
        9 as std::ffi::c_int as BYTE,
        10 as std::ffi::c_int as BYTE,
        11 as std::ffi::c_int as BYTE,
        12 as std::ffi::c_int as BYTE,
        13 as std::ffi::c_int as BYTE,
        14 as std::ffi::c_int as BYTE,
        15 as std::ffi::c_int as BYTE,
        16 as std::ffi::c_int as BYTE,
        16 as std::ffi::c_int as BYTE,
        17 as std::ffi::c_int as BYTE,
        17 as std::ffi::c_int as BYTE,
        18 as std::ffi::c_int as BYTE,
        18 as std::ffi::c_int as BYTE,
        19 as std::ffi::c_int as BYTE,
        19 as std::ffi::c_int as BYTE,
        20 as std::ffi::c_int as BYTE,
        20 as std::ffi::c_int as BYTE,
        20 as std::ffi::c_int as BYTE,
        20 as std::ffi::c_int as BYTE,
        21 as std::ffi::c_int as BYTE,
        21 as std::ffi::c_int as BYTE,
        21 as std::ffi::c_int as BYTE,
        21 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
    ];
    static mut LL_deltaCode: U32 = 19 as std::ffi::c_int as U32;
    if litLength > 63 as std::ffi::c_int as U32 {
        (ZSTD_highbit32(litLength)).wrapping_add(LL_deltaCode)
    } else {
        *LL_Code.as_ptr().offset(litLength as isize) as std::ffi::c_uint
    }
}
#[inline]
unsafe extern "C" fn ZSTD_MLcode(mut mlBase: U32) -> U32 {
    static mut ML_Code: [BYTE; 128] = [
        0 as std::ffi::c_int as BYTE,
        1 as std::ffi::c_int as BYTE,
        2 as std::ffi::c_int as BYTE,
        3 as std::ffi::c_int as BYTE,
        4 as std::ffi::c_int as BYTE,
        5 as std::ffi::c_int as BYTE,
        6 as std::ffi::c_int as BYTE,
        7 as std::ffi::c_int as BYTE,
        8 as std::ffi::c_int as BYTE,
        9 as std::ffi::c_int as BYTE,
        10 as std::ffi::c_int as BYTE,
        11 as std::ffi::c_int as BYTE,
        12 as std::ffi::c_int as BYTE,
        13 as std::ffi::c_int as BYTE,
        14 as std::ffi::c_int as BYTE,
        15 as std::ffi::c_int as BYTE,
        16 as std::ffi::c_int as BYTE,
        17 as std::ffi::c_int as BYTE,
        18 as std::ffi::c_int as BYTE,
        19 as std::ffi::c_int as BYTE,
        20 as std::ffi::c_int as BYTE,
        21 as std::ffi::c_int as BYTE,
        22 as std::ffi::c_int as BYTE,
        23 as std::ffi::c_int as BYTE,
        24 as std::ffi::c_int as BYTE,
        25 as std::ffi::c_int as BYTE,
        26 as std::ffi::c_int as BYTE,
        27 as std::ffi::c_int as BYTE,
        28 as std::ffi::c_int as BYTE,
        29 as std::ffi::c_int as BYTE,
        30 as std::ffi::c_int as BYTE,
        31 as std::ffi::c_int as BYTE,
        32 as std::ffi::c_int as BYTE,
        32 as std::ffi::c_int as BYTE,
        33 as std::ffi::c_int as BYTE,
        33 as std::ffi::c_int as BYTE,
        34 as std::ffi::c_int as BYTE,
        34 as std::ffi::c_int as BYTE,
        35 as std::ffi::c_int as BYTE,
        35 as std::ffi::c_int as BYTE,
        36 as std::ffi::c_int as BYTE,
        36 as std::ffi::c_int as BYTE,
        36 as std::ffi::c_int as BYTE,
        36 as std::ffi::c_int as BYTE,
        37 as std::ffi::c_int as BYTE,
        37 as std::ffi::c_int as BYTE,
        37 as std::ffi::c_int as BYTE,
        37 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        38 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        39 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        40 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        41 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
        42 as std::ffi::c_int as BYTE,
    ];
    static mut ML_deltaCode: U32 = 36 as std::ffi::c_int as U32;
    if mlBase > 127 as std::ffi::c_int as U32 {
        (ZSTD_highbit32(mlBase)).wrapping_add(ML_deltaCode)
    } else {
        *ML_Code.as_ptr().offset(mlBase as isize) as std::ffi::c_uint
    }
}
unsafe extern "C" fn ZSTD_safecopyLiterals(
    mut op: *mut BYTE,
    mut ip: *const BYTE,
    iend: *const BYTE,
    mut ilimit_w: *const BYTE,
) {
    if ip <= ilimit_w {
        ZSTD_wildcopy(
            op as *mut std::ffi::c_void,
            ip as *const std::ffi::c_void,
            ilimit_w.offset_from(ip) as std::ffi::c_long as size_t,
            ZSTD_no_overlap,
        );
        op = op.offset(ilimit_w.offset_from(ip) as std::ffi::c_long as isize);
        ip = ilimit_w;
    }
    while ip < iend {
        let fresh0 = ip;
        ip = ip.offset(1);
        let fresh1 = op;
        op = op.offset(1);
        *fresh1 = *fresh0;
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_storeSeqOnly(
    mut seqStorePtr: *mut SeqStore_t,
    mut litLength: size_t,
    mut offBase: U32,
    mut matchLength: size_t,
) {
    if (litLength > 0xffff as std::ffi::c_int as size_t) as std::ffi::c_int as std::ffi::c_long != 0
    {
        (*seqStorePtr).longLengthType = ZSTD_llt_literalLength;
        (*seqStorePtr).longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart)
            as std::ffi::c_long as U32;
    }
    (*((*seqStorePtr).sequences).offset(0 as std::ffi::c_int as isize)).litLength =
        litLength as U16;
    (*((*seqStorePtr).sequences).offset(0 as std::ffi::c_int as isize)).offBase = offBase;
    let mlBase = matchLength.wrapping_sub(MINMATCH as size_t);
    if (mlBase > 0xffff as std::ffi::c_int as size_t) as std::ffi::c_int as std::ffi::c_long != 0 {
        (*seqStorePtr).longLengthType = ZSTD_llt_matchLength;
        (*seqStorePtr).longLengthPos = ((*seqStorePtr).sequences)
            .offset_from((*seqStorePtr).sequencesStart)
            as std::ffi::c_long as U32;
    }
    (*((*seqStorePtr).sequences).offset(0 as std::ffi::c_int as isize)).mlBase = mlBase as U16;
    (*seqStorePtr).sequences = ((*seqStorePtr).sequences).offset(1);
    (*seqStorePtr).sequences;
}
#[inline(always)]
unsafe extern "C" fn ZSTD_storeSeq(
    mut seqStorePtr: *mut SeqStore_t,
    mut litLength: size_t,
    mut literals: *const BYTE,
    mut litLimit: *const BYTE,
    mut offBase: U32,
    mut matchLength: size_t,
) {
    let litLimit_w = litLimit.offset(-(WILDCOPY_OVERLENGTH as isize));
    let litEnd = literals.offset(litLength as isize);
    if litEnd <= litLimit_w {
        ZSTD_copy16(
            (*seqStorePtr).lit as *mut std::ffi::c_void,
            literals as *const std::ffi::c_void,
        );
        if litLength > 16 as std::ffi::c_int as size_t {
            ZSTD_wildcopy(
                ((*seqStorePtr).lit).offset(16 as std::ffi::c_int as isize)
                    as *mut std::ffi::c_void,
                literals.offset(16 as std::ffi::c_int as isize) as *const std::ffi::c_void,
                litLength.wrapping_sub(16 as std::ffi::c_int as size_t),
                ZSTD_no_overlap,
            );
        }
    } else {
        ZSTD_safecopyLiterals((*seqStorePtr).lit, literals, litEnd, litLimit_w);
    }
    (*seqStorePtr).lit = ((*seqStorePtr).lit).offset(litLength as isize);
    ZSTD_storeSeqOnly(seqStorePtr, litLength, offBase, matchLength);
}
#[inline]
unsafe extern "C" fn ZSTD_updateRep(mut rep: *mut U32, offBase: U32, ll0: U32) {
    if offBase > ZSTD_REP_NUM as U32 {
        *rep.offset(2 as std::ffi::c_int as isize) = *rep.offset(1 as std::ffi::c_int as isize);
        *rep.offset(1 as std::ffi::c_int as isize) = *rep.offset(0 as std::ffi::c_int as isize);
        *rep.offset(0 as std::ffi::c_int as isize) = offBase.wrapping_sub(ZSTD_REP_NUM as U32);
    } else {
        let repCode = offBase
            .wrapping_sub(1 as std::ffi::c_int as U32)
            .wrapping_add(ll0);
        if repCode > 0 as std::ffi::c_int as U32 {
            let currentOffset = if repCode == ZSTD_REP_NUM as U32 {
                (*rep.offset(0 as std::ffi::c_int as isize))
                    .wrapping_sub(1 as std::ffi::c_int as U32)
            } else {
                *rep.offset(repCode as isize)
            };
            *rep.offset(2 as std::ffi::c_int as isize) = if repCode >= 2 as std::ffi::c_int as U32 {
                *rep.offset(1 as std::ffi::c_int as isize)
            } else {
                *rep.offset(2 as std::ffi::c_int as isize)
            };
            *rep.offset(1 as std::ffi::c_int as isize) = *rep.offset(0 as std::ffi::c_int as isize);
            *rep.offset(0 as std::ffi::c_int as isize) = currentOffset;
        }
    };
}
#[inline]
unsafe extern "C" fn ZSTD_newRep(mut rep: *const U32, offBase: U32, ll0: U32) -> Repcodes_t {
    let mut newReps = repcodes_s { rep: [0; 3] };
    libc::memcpy(
        &mut newReps as *mut Repcodes_t as *mut std::ffi::c_void,
        rep as *const std::ffi::c_void,
        ::core::mem::size_of::<Repcodes_t>() as std::ffi::c_ulong as libc::size_t,
    );
    ZSTD_updateRep((newReps.rep).as_mut_ptr(), offBase, ll0);
    newReps
}
#[inline]
unsafe extern "C" fn ZSTD_count(
    mut pIn: *const BYTE,
    mut pMatch: *const BYTE,
    pInLimit: *const BYTE,
) -> size_t {
    let pStart = pIn;
    let pInLoopLimit = pInLimit.offset(
        -((::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as isize),
    );
    if pIn < pInLoopLimit {
        let diff = MEM_readST(pMatch as *const std::ffi::c_void)
            ^ MEM_readST(pIn as *const std::ffi::c_void);
        if diff != 0 {
            return ZSTD_NbCommonBytes(diff) as size_t;
        }
        pIn = pIn.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
        pMatch = pMatch.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
        while pIn < pInLoopLimit {
            let diff_0 = MEM_readST(pMatch as *const std::ffi::c_void)
                ^ MEM_readST(pIn as *const std::ffi::c_void);
            if diff_0 == 0 {
                pIn = pIn.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
                pMatch =
                    pMatch.offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
            } else {
                pIn = pIn.offset(ZSTD_NbCommonBytes(diff_0) as isize);
                return pIn.offset_from(pStart) as std::ffi::c_long as size_t;
            }
        }
    }
    if MEM_64bits() != 0
        && pIn < pInLimit.offset(-(3 as std::ffi::c_int as isize))
        && MEM_read32(pMatch as *const std::ffi::c_void)
            == MEM_read32(pIn as *const std::ffi::c_void)
    {
        pIn = pIn.offset(4 as std::ffi::c_int as isize);
        pMatch = pMatch.offset(4 as std::ffi::c_int as isize);
    }
    if pIn < pInLimit.offset(-(1 as std::ffi::c_int as isize))
        && MEM_read16(pMatch as *const std::ffi::c_void) as std::ffi::c_int
            == MEM_read16(pIn as *const std::ffi::c_void) as std::ffi::c_int
    {
        pIn = pIn.offset(2 as std::ffi::c_int as isize);
        pMatch = pMatch.offset(2 as std::ffi::c_int as isize);
    }
    if pIn < pInLimit && *pMatch as std::ffi::c_int == *pIn as std::ffi::c_int {
        pIn = pIn.offset(1);
        pIn;
    }
    pIn.offset_from(pStart) as std::ffi::c_long as size_t
}
#[inline]
unsafe extern "C" fn ZSTD_count_2segments(
    mut ip: *const BYTE,
    mut match_0: *const BYTE,
    mut iEnd: *const BYTE,
    mut mEnd: *const BYTE,
    mut iStart: *const BYTE,
) -> size_t {
    let vEnd = if ip.offset(mEnd.offset_from(match_0) as std::ffi::c_long as isize) < iEnd {
        ip.offset(mEnd.offset_from(match_0) as std::ffi::c_long as isize)
    } else {
        iEnd
    };
    let matchLength = ZSTD_count(ip, match_0, vEnd);
    if match_0.offset(matchLength as isize) != mEnd {
        return matchLength;
    }
    matchLength.wrapping_add(ZSTD_count(ip.offset(matchLength as isize), iStart, iEnd))
}
static mut prime3bytes: U32 = 506832829 as std::ffi::c_uint;
unsafe extern "C" fn ZSTD_hash3(mut u: U32, mut h: U32, mut s: U32) -> U32 {
    (((u << (32 as std::ffi::c_int - 24 as std::ffi::c_int)) * prime3bytes) ^ s)
        >> (32 as std::ffi::c_int as U32).wrapping_sub(h)
}
#[inline]
unsafe extern "C" fn ZSTD_hash3Ptr(mut ptr: *const std::ffi::c_void, mut h: U32) -> size_t {
    ZSTD_hash3(MEM_readLE32(ptr), h, 0 as std::ffi::c_int as U32) as size_t
}
static mut prime4bytes: U32 = 2654435761 as std::ffi::c_uint;
unsafe extern "C" fn ZSTD_hash4(mut u: U32, mut h: U32, mut s: U32) -> U32 {
    ((u * prime4bytes) ^ s) >> (32 as std::ffi::c_int as U32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash4Ptr(mut ptr: *const std::ffi::c_void, mut h: U32) -> size_t {
    ZSTD_hash4(MEM_readLE32(ptr), h, 0 as std::ffi::c_int as U32) as size_t
}
static mut prime5bytes: U64 = 889523592379 as std::ffi::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash5(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    (((u << (64 as std::ffi::c_int - 40 as std::ffi::c_int)) * prime5bytes) ^ s)
        >> (64 as std::ffi::c_int as U32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash5Ptr(mut p: *const std::ffi::c_void, mut h: U32) -> size_t {
    ZSTD_hash5(MEM_readLE64(p), h, 0 as std::ffi::c_int as U64)
}
static mut prime6bytes: U64 = 227718039650203 as std::ffi::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash6(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    (((u << (64 as std::ffi::c_int - 48 as std::ffi::c_int)) * prime6bytes) ^ s)
        >> (64 as std::ffi::c_int as U32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash6Ptr(mut p: *const std::ffi::c_void, mut h: U32) -> size_t {
    ZSTD_hash6(MEM_readLE64(p), h, 0 as std::ffi::c_int as U64)
}
static mut prime7bytes: U64 = 58295818150454627 as std::ffi::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash7(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    (((u << (64 as std::ffi::c_int - 56 as std::ffi::c_int)) * prime7bytes) ^ s)
        >> (64 as std::ffi::c_int as U32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash7Ptr(mut p: *const std::ffi::c_void, mut h: U32) -> size_t {
    ZSTD_hash7(MEM_readLE64(p), h, 0 as std::ffi::c_int as U64)
}
static mut prime8bytes: U64 = 0xcf1bbcdcb7a56463 as std::ffi::c_ulonglong as U64;
unsafe extern "C" fn ZSTD_hash8(mut u: U64, mut h: U32, mut s: U64) -> size_t {
    ((u * prime8bytes) ^ s) >> (64 as std::ffi::c_int as U32).wrapping_sub(h)
}
unsafe extern "C" fn ZSTD_hash8Ptr(mut p: *const std::ffi::c_void, mut h: U32) -> size_t {
    ZSTD_hash8(MEM_readLE64(p), h, 0 as std::ffi::c_int as U64)
}
#[inline(always)]
unsafe extern "C" fn ZSTD_hashPtr(
    mut p: *const std::ffi::c_void,
    mut hBits: U32,
    mut mls: U32,
) -> size_t {
    match mls {
        5 => ZSTD_hash5Ptr(p, hBits),
        6 => ZSTD_hash6Ptr(p, hBits),
        7 => ZSTD_hash7Ptr(p, hBits),
        8 => ZSTD_hash8Ptr(p, hBits),
        4 | _ => ZSTD_hash4Ptr(p, hBits),
    }
}
#[inline]
unsafe extern "C" fn ZSTD_getLowestMatchIndex(
    mut ms: *const ZSTD_MatchState_t,
    mut curr: U32,
    mut windowLog: std::ffi::c_uint,
) -> U32 {
    let maxDistance = (1 as std::ffi::c_uint) << windowLog;
    let lowestValid = (*ms).window.lowLimit;
    let withinWindow = if curr.wrapping_sub(lowestValid) > maxDistance {
        curr.wrapping_sub(maxDistance)
    } else {
        lowestValid
    };
    let isDictionary =
        ((*ms).loadedDictEnd != 0 as std::ffi::c_int as U32) as std::ffi::c_int as U32;

    if isDictionary != 0 {
        lowestValid
    } else {
        withinWindow
    }
}
#[inline]
unsafe extern "C" fn ZSTD_index_overlap_check(
    prefixLowestIndex: U32,
    repIndex: U32,
) -> std::ffi::c_int {
    (prefixLowestIndex
        .wrapping_sub(1 as std::ffi::c_int as U32)
        .wrapping_sub(repIndex)
        >= 3 as std::ffi::c_int as U32) as std::ffi::c_int
}
pub const ZSTD_OPT_NUM: std::ffi::c_int = (1 as std::ffi::c_int) << 12 as std::ffi::c_int;
pub const ZSTD_REP_NUM: std::ffi::c_int = 3 as std::ffi::c_int;
pub const MINMATCH: std::ffi::c_int = 3 as std::ffi::c_int;
pub const Litbits: std::ffi::c_int = 8 as std::ffi::c_int;
pub const MaxLit: std::ffi::c_int = ((1 as std::ffi::c_int) << Litbits) - 1 as std::ffi::c_int;
pub const MaxML: std::ffi::c_int = 52 as std::ffi::c_int;
pub const MaxLL: std::ffi::c_int = 35 as std::ffi::c_int;
pub const MaxOff: std::ffi::c_int = 31 as std::ffi::c_int;
static mut LL_bits: [U8; 36] = [
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    4 as std::ffi::c_int as U8,
    6 as std::ffi::c_int as U8,
    7 as std::ffi::c_int as U8,
    8 as std::ffi::c_int as U8,
    9 as std::ffi::c_int as U8,
    10 as std::ffi::c_int as U8,
    11 as std::ffi::c_int as U8,
    12 as std::ffi::c_int as U8,
    13 as std::ffi::c_int as U8,
    14 as std::ffi::c_int as U8,
    15 as std::ffi::c_int as U8,
    16 as std::ffi::c_int as U8,
];
static mut ML_bits: [U8; 53] = [
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    0 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    1 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    2 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    3 as std::ffi::c_int as U8,
    4 as std::ffi::c_int as U8,
    4 as std::ffi::c_int as U8,
    5 as std::ffi::c_int as U8,
    7 as std::ffi::c_int as U8,
    8 as std::ffi::c_int as U8,
    9 as std::ffi::c_int as U8,
    10 as std::ffi::c_int as U8,
    11 as std::ffi::c_int as U8,
    12 as std::ffi::c_int as U8,
    13 as std::ffi::c_int as U8,
    14 as std::ffi::c_int as U8,
    15 as std::ffi::c_int as U8,
    16 as std::ffi::c_int as U8,
];
unsafe extern "C" fn ZSTD_copy8(mut dst: *mut std::ffi::c_void, mut src: *const std::ffi::c_void) {
    libc::memcpy(
        dst,
        src,
        8 as std::ffi::c_int as std::ffi::c_ulong as libc::size_t,
    );
}
unsafe extern "C" fn ZSTD_copy16(mut dst: *mut std::ffi::c_void, mut src: *const std::ffi::c_void) {
    _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
}
pub const WILDCOPY_OVERLENGTH: std::ffi::c_int = 32 as std::ffi::c_int;
pub const WILDCOPY_VECLEN: std::ffi::c_int = 16 as std::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn ZSTD_wildcopy(
    mut dst: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut length: size_t,
    ovtype: ZSTD_overlap_e,
) {
    let mut diff = (dst as *mut BYTE).offset_from(src as *const BYTE) as std::ffi::c_long;
    let mut ip = src as *const BYTE;
    let mut op = dst as *mut BYTE;
    let oend = op.offset(length as isize);
    if ovtype as std::ffi::c_uint
        == ZSTD_overlap_src_before_dst as std::ffi::c_int as std::ffi::c_uint
        && diff < WILDCOPY_VECLEN as ptrdiff_t
    {
        loop {
            ZSTD_copy8(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(8 as std::ffi::c_int as isize);
            ip = ip.offset(8 as std::ffi::c_int as isize);
            if op >= oend {
                break;
            }
        }
    } else {
        ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
        if 16 as std::ffi::c_int as size_t >= length {
            return;
        }
        op = op.offset(16 as std::ffi::c_int as isize);
        ip = ip.offset(16 as std::ffi::c_int as isize);
        loop {
            ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(16 as std::ffi::c_int as isize);
            ip = ip.offset(16 as std::ffi::c_int as isize);
            ZSTD_copy16(op as *mut std::ffi::c_void, ip as *const std::ffi::c_void);
            op = op.offset(16 as std::ffi::c_int as isize);
            ip = ip.offset(16 as std::ffi::c_int as isize);
            if op >= oend {
                break;
            }
        }
    };
}
#[inline]
unsafe extern "C" fn FSE_initCState(mut statePtr: *mut FSE_CState_t, mut ct: *const FSE_CTable) {
    let mut ptr = ct as *const std::ffi::c_void;
    let mut u16ptr = ptr as *const U16;
    let tableLog = MEM_read16(ptr) as U32;
    (*statePtr).value = (1 as std::ffi::c_int as ptrdiff_t) << tableLog;
    (*statePtr).stateTable =
        u16ptr.offset(2 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    (*statePtr).symbolTT = ct.offset(1 as std::ffi::c_int as isize).offset(
        (if tableLog != 0 {
            (1 as std::ffi::c_int) << tableLog.wrapping_sub(1 as std::ffi::c_int as U32)
        } else {
            1 as std::ffi::c_int
        }) as isize,
    ) as *const std::ffi::c_void;
    (*statePtr).stateLog = tableLog;
}
#[inline]
unsafe extern "C" fn FSE_getMaxNbBits(
    mut symbolTTPtr: *const std::ffi::c_void,
    mut symbolValue: U32,
) -> U32 {
    let mut symbolTT = symbolTTPtr as *const FSE_symbolCompressionTransform;
    ((*symbolTT.offset(symbolValue as isize)).deltaNbBits).wrapping_add(
        (((1 as std::ffi::c_int) << 16 as std::ffi::c_int) - 1 as std::ffi::c_int) as U32,
    ) >> 16 as std::ffi::c_int
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros32(mut val: U32) -> std::ffi::c_uint {
    val.trailing_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: U32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: U64) -> std::ffi::c_uint {
    (val as std::ffi::c_ulonglong).trailing_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros64(mut val: U64) -> std::ffi::c_uint {
    (val as std::ffi::c_ulonglong).leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_NbCommonBytes(mut val: size_t) -> std::ffi::c_uint {
    if MEM_isLittleEndian() != 0 {
        if MEM_64bits() != 0 {
            ZSTD_countTrailingZeros64(val) >> 3 as std::ffi::c_int
        } else {
            ZSTD_countTrailingZeros32(val as U32) >> 3 as std::ffi::c_int
        }
    } else if MEM_64bits() != 0 {
        ZSTD_countLeadingZeros64(val) >> 3 as std::ffi::c_int
    } else {
        ZSTD_countLeadingZeros32(val as U32) >> 3 as std::ffi::c_int
    }
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: U32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const UINT_MAX: std::ffi::c_uint = (__INT_MAX__ as std::ffi::c_uint)
    .wrapping_mul(2 as std::ffi::c_uint)
    .wrapping_add(1 as std::ffi::c_uint);
pub const ZSTD_LITFREQ_ADD: std::ffi::c_int = 2 as std::ffi::c_int;
pub const ZSTD_MAX_PRICE: std::ffi::c_int = (1 as std::ffi::c_int) << 30 as std::ffi::c_int;
pub const ZSTD_PREDEF_THRESHOLD: std::ffi::c_int = 8 as std::ffi::c_int;
pub const BITCOST_ACCURACY: std::ffi::c_int = 8 as std::ffi::c_int;
pub const BITCOST_MULTIPLIER: std::ffi::c_int = (1 as std::ffi::c_int) << BITCOST_ACCURACY;
#[inline]
unsafe extern "C" fn ZSTD_bitWeight(mut stat: U32) -> U32 {
    (ZSTD_highbit32(stat.wrapping_add(1 as std::ffi::c_int as U32)))
        .wrapping_mul(BITCOST_MULTIPLIER as std::ffi::c_uint)
}
#[inline]
unsafe extern "C" fn ZSTD_fracWeight(mut rawStat: U32) -> U32 {
    let stat = rawStat.wrapping_add(1 as std::ffi::c_int as U32);
    let hb = ZSTD_highbit32(stat);
    let BWeight = hb * BITCOST_MULTIPLIER as U32;
    let FWeight = stat << BITCOST_ACCURACY >> hb;

    BWeight.wrapping_add(FWeight)
}
unsafe extern "C" fn ZSTD_compressedLiterals(optPtr: *const optState_t) -> std::ffi::c_int {
    ((*optPtr).literalCompressionMode as std::ffi::c_uint
        != ZSTD_ps_disable as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int
}
unsafe extern "C" fn ZSTD_setBasePrices(
    mut optPtr: *mut optState_t,
    mut optLevel: std::ffi::c_int,
) {
    if ZSTD_compressedLiterals(optPtr) != 0 {
        (*optPtr).litSumBasePrice = if optLevel != 0 {
            ZSTD_fracWeight((*optPtr).litSum)
        } else {
            ZSTD_bitWeight((*optPtr).litSum)
        };
    }
    (*optPtr).litLengthSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).litLengthSum)
    } else {
        ZSTD_bitWeight((*optPtr).litLengthSum)
    };
    (*optPtr).matchLengthSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).matchLengthSum)
    } else {
        ZSTD_bitWeight((*optPtr).matchLengthSum)
    };
    (*optPtr).offCodeSumBasePrice = if optLevel != 0 {
        ZSTD_fracWeight((*optPtr).offCodeSum)
    } else {
        ZSTD_bitWeight((*optPtr).offCodeSum)
    };
}
unsafe extern "C" fn sum_u32(mut table: *const std::ffi::c_uint, mut nbElts: size_t) -> U32 {
    let mut n: size_t = 0;
    let mut total = 0 as std::ffi::c_int as U32;
    n = 0 as std::ffi::c_int as size_t;
    while n < nbElts {
        total = (total as std::ffi::c_uint).wrapping_add(*table.offset(n as isize)) as U32 as U32;
        n = n.wrapping_add(1);
        n;
    }
    total
}
unsafe extern "C" fn ZSTD_downscaleStats(
    mut table: *mut std::ffi::c_uint,
    mut lastEltIndex: U32,
    mut shift: U32,
    mut base1: base_directive_e,
) -> U32 {
    let mut s: U32 = 0;
    let mut sum = 0 as std::ffi::c_int as U32;
    s = 0 as std::ffi::c_int as U32;
    while s < lastEltIndex.wrapping_add(1 as std::ffi::c_int as U32) {
        let base = (if base1 as std::ffi::c_uint != 0 {
            1 as std::ffi::c_int
        } else {
            (*table.offset(s as isize) > 0 as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int
        }) as std::ffi::c_uint;
        let newStat = base.wrapping_add(*table.offset(s as isize) >> shift);
        sum = (sum as std::ffi::c_uint).wrapping_add(newStat) as U32 as U32;
        *table.offset(s as isize) = newStat;
        s = s.wrapping_add(1);
        s;
    }
    sum
}
unsafe extern "C" fn ZSTD_scaleStats(
    mut table: *mut std::ffi::c_uint,
    mut lastEltIndex: U32,
    mut logTarget: U32,
) -> U32 {
    let prevsum = sum_u32(
        table as *const std::ffi::c_uint,
        lastEltIndex.wrapping_add(1 as std::ffi::c_int as U32) as size_t,
    );
    let factor = prevsum >> logTarget;
    if factor <= 1 as std::ffi::c_int as U32 {
        return prevsum;
    }
    ZSTD_downscaleStats(
        table,
        lastEltIndex,
        ZSTD_highbit32(factor),
        base_1guaranteed,
    )
}
unsafe extern "C" fn ZSTD_rescaleFreqs(
    optPtr: *mut optState_t,
    src: *const BYTE,
    srcSize: size_t,
    optLevel: std::ffi::c_int,
) {
    let compressedLiterals = ZSTD_compressedLiterals(optPtr);
    (*optPtr).priceType = zop_dynamic;
    if (*optPtr).litLengthSum == 0 as std::ffi::c_int as U32 {
        if srcSize <= ZSTD_PREDEF_THRESHOLD as size_t {
            (*optPtr).priceType = zop_predef;
        }
        if (*(*optPtr).symbolCosts).huf.repeatMode as std::ffi::c_uint
            == HUF_repeat_valid as std::ffi::c_int as std::ffi::c_uint
        {
            (*optPtr).priceType = zop_dynamic;
            if compressedLiterals != 0 {
                let mut lit: std::ffi::c_uint = 0;
                (*optPtr).litSum = 0 as std::ffi::c_int as U32;
                lit = 0 as std::ffi::c_int as std::ffi::c_uint;
                while lit <= MaxLit as std::ffi::c_uint {
                    let scaleLog = 11 as std::ffi::c_int as U32;
                    let bitCost = HUF_getNbBitsFromCTable(
                        ((*(*optPtr).symbolCosts).huf.CTable).as_ptr(),
                        lit,
                    );
                    *((*optPtr).litFreq).offset(lit as isize) = (if bitCost != 0 {
                        (1 as std::ffi::c_int) << scaleLog.wrapping_sub(bitCost)
                    } else {
                        1 as std::ffi::c_int
                    })
                        as std::ffi::c_uint;
                    (*optPtr).litSum = ((*optPtr).litSum as std::ffi::c_uint)
                        .wrapping_add(*((*optPtr).litFreq).offset(lit as isize))
                        as U32 as U32;
                    lit = lit.wrapping_add(1);
                    lit;
                }
            }
            let mut ll: std::ffi::c_uint = 0;
            let mut llstate = FSE_CState_t {
                value: 0,
                stateTable: std::ptr::null::<std::ffi::c_void>(),
                symbolTT: std::ptr::null::<std::ffi::c_void>(),
                stateLog: 0,
            };
            FSE_initCState(
                &mut llstate,
                ((*(*optPtr).symbolCosts).fse.litlengthCTable).as_ptr(),
            );
            (*optPtr).litLengthSum = 0 as std::ffi::c_int as U32;
            ll = 0 as std::ffi::c_int as std::ffi::c_uint;
            while ll <= MaxLL as std::ffi::c_uint {
                let scaleLog_0 = 10 as std::ffi::c_int as U32;
                let bitCost_0 = FSE_getMaxNbBits(llstate.symbolTT, ll);
                *((*optPtr).litLengthFreq).offset(ll as isize) = (if bitCost_0 != 0 {
                    (1 as std::ffi::c_int) << scaleLog_0.wrapping_sub(bitCost_0)
                } else {
                    1 as std::ffi::c_int
                })
                    as std::ffi::c_uint;
                (*optPtr).litLengthSum = ((*optPtr).litLengthSum as std::ffi::c_uint)
                    .wrapping_add(*((*optPtr).litLengthFreq).offset(ll as isize))
                    as U32 as U32;
                ll = ll.wrapping_add(1);
                ll;
            }
            let mut ml: std::ffi::c_uint = 0;
            let mut mlstate = FSE_CState_t {
                value: 0,
                stateTable: std::ptr::null::<std::ffi::c_void>(),
                symbolTT: std::ptr::null::<std::ffi::c_void>(),
                stateLog: 0,
            };
            FSE_initCState(
                &mut mlstate,
                ((*(*optPtr).symbolCosts).fse.matchlengthCTable).as_ptr(),
            );
            (*optPtr).matchLengthSum = 0 as std::ffi::c_int as U32;
            ml = 0 as std::ffi::c_int as std::ffi::c_uint;
            while ml <= MaxML as std::ffi::c_uint {
                let scaleLog_1 = 10 as std::ffi::c_int as U32;
                let bitCost_1 = FSE_getMaxNbBits(mlstate.symbolTT, ml);
                *((*optPtr).matchLengthFreq).offset(ml as isize) = (if bitCost_1 != 0 {
                    (1 as std::ffi::c_int) << scaleLog_1.wrapping_sub(bitCost_1)
                } else {
                    1 as std::ffi::c_int
                })
                    as std::ffi::c_uint;
                (*optPtr).matchLengthSum = ((*optPtr).matchLengthSum as std::ffi::c_uint)
                    .wrapping_add(*((*optPtr).matchLengthFreq).offset(ml as isize))
                    as U32 as U32;
                ml = ml.wrapping_add(1);
                ml;
            }
            let mut of: std::ffi::c_uint = 0;
            let mut ofstate = FSE_CState_t {
                value: 0,
                stateTable: std::ptr::null::<std::ffi::c_void>(),
                symbolTT: std::ptr::null::<std::ffi::c_void>(),
                stateLog: 0,
            };
            FSE_initCState(
                &mut ofstate,
                ((*(*optPtr).symbolCosts).fse.offcodeCTable).as_ptr(),
            );
            (*optPtr).offCodeSum = 0 as std::ffi::c_int as U32;
            of = 0 as std::ffi::c_int as std::ffi::c_uint;
            while of <= MaxOff as std::ffi::c_uint {
                let scaleLog_2 = 10 as std::ffi::c_int as U32;
                let bitCost_2 = FSE_getMaxNbBits(ofstate.symbolTT, of);
                *((*optPtr).offCodeFreq).offset(of as isize) = (if bitCost_2 != 0 {
                    (1 as std::ffi::c_int) << scaleLog_2.wrapping_sub(bitCost_2)
                } else {
                    1 as std::ffi::c_int
                })
                    as std::ffi::c_uint;
                (*optPtr).offCodeSum = ((*optPtr).offCodeSum as std::ffi::c_uint)
                    .wrapping_add(*((*optPtr).offCodeFreq).offset(of as isize))
                    as U32 as U32;
                of = of.wrapping_add(1);
                of;
            }
        } else {
            if compressedLiterals != 0 {
                let mut lit_0 = MaxLit as std::ffi::c_uint;
                HIST_count_simple(
                    (*optPtr).litFreq,
                    &mut lit_0,
                    src as *const std::ffi::c_void,
                    srcSize,
                );
                (*optPtr).litSum = ZSTD_downscaleStats(
                    (*optPtr).litFreq,
                    MaxLit as U32,
                    8 as std::ffi::c_int as U32,
                    base_0possible,
                );
            }
            let baseLLfreqs: [std::ffi::c_uint; 36] = [
                4 as std::ffi::c_int as std::ffi::c_uint,
                2 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
            ];
            libc::memcpy(
                (*optPtr).litLengthFreq as *mut std::ffi::c_void,
                baseLLfreqs.as_ptr() as *const std::ffi::c_void,
                ::core::mem::size_of::<[std::ffi::c_uint; 36]>() as std::ffi::c_ulong
                    as libc::size_t,
            );
            (*optPtr).litLengthSum = sum_u32(
                baseLLfreqs.as_ptr(),
                (MaxLL + 1 as std::ffi::c_int) as size_t,
            );
            let mut ml_0: std::ffi::c_uint = 0;
            ml_0 = 0 as std::ffi::c_int as std::ffi::c_uint;
            while ml_0 <= MaxML as std::ffi::c_uint {
                *((*optPtr).matchLengthFreq).offset(ml_0 as isize) =
                    1 as std::ffi::c_int as std::ffi::c_uint;
                ml_0 = ml_0.wrapping_add(1);
                ml_0;
            }
            (*optPtr).matchLengthSum = (MaxML + 1 as std::ffi::c_int) as U32;
            let baseOFCfreqs: [std::ffi::c_uint; 32] = [
                6 as std::ffi::c_int as std::ffi::c_uint,
                2 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                2 as std::ffi::c_int as std::ffi::c_uint,
                3 as std::ffi::c_int as std::ffi::c_uint,
                4 as std::ffi::c_int as std::ffi::c_uint,
                4 as std::ffi::c_int as std::ffi::c_uint,
                4 as std::ffi::c_int as std::ffi::c_uint,
                3 as std::ffi::c_int as std::ffi::c_uint,
                2 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
                1 as std::ffi::c_int as std::ffi::c_uint,
            ];
            libc::memcpy(
                (*optPtr).offCodeFreq as *mut std::ffi::c_void,
                baseOFCfreqs.as_ptr() as *const std::ffi::c_void,
                ::core::mem::size_of::<[std::ffi::c_uint; 32]>() as std::ffi::c_ulong
                    as libc::size_t,
            );
            (*optPtr).offCodeSum = sum_u32(
                baseOFCfreqs.as_ptr(),
                (MaxOff + 1 as std::ffi::c_int) as size_t,
            );
        }
    } else {
        if compressedLiterals != 0 {
            (*optPtr).litSum = ZSTD_scaleStats(
                (*optPtr).litFreq,
                MaxLit as U32,
                12 as std::ffi::c_int as U32,
            );
        }
        (*optPtr).litLengthSum = ZSTD_scaleStats(
            (*optPtr).litLengthFreq,
            MaxLL as U32,
            11 as std::ffi::c_int as U32,
        );
        (*optPtr).matchLengthSum = ZSTD_scaleStats(
            (*optPtr).matchLengthFreq,
            MaxML as U32,
            11 as std::ffi::c_int as U32,
        );
        (*optPtr).offCodeSum = ZSTD_scaleStats(
            (*optPtr).offCodeFreq,
            MaxOff as U32,
            11 as std::ffi::c_int as U32,
        );
    }
    ZSTD_setBasePrices(optPtr, optLevel);
}
unsafe extern "C" fn ZSTD_rawLiteralsCost(
    literals: *const BYTE,
    litLength: U32,
    optPtr: *const optState_t,
    mut optLevel: std::ffi::c_int,
) -> U32 {
    if litLength == 0 as std::ffi::c_int as U32 {
        return 0 as std::ffi::c_int as U32;
    }
    if ZSTD_compressedLiterals(optPtr) == 0 {
        return (litLength << 3 as std::ffi::c_int) * BITCOST_MULTIPLIER as U32;
    }
    if (*optPtr).priceType as std::ffi::c_uint == zop_predef as std::ffi::c_int as std::ffi::c_uint
    {
        return litLength * 6 as std::ffi::c_int as U32 * BITCOST_MULTIPLIER as U32;
    }
    let mut price = (*optPtr).litSumBasePrice * litLength;
    let litPriceMax = ((*optPtr).litSumBasePrice).wrapping_sub(BITCOST_MULTIPLIER as U32);
    let mut u: U32 = 0;
    u = 0 as std::ffi::c_int as U32;
    while u < litLength {
        let mut litPrice = if optLevel != 0 {
            ZSTD_fracWeight(*((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize))
        } else {
            ZSTD_bitWeight(*((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize))
        };
        if (litPrice > litPriceMax) as std::ffi::c_int as std::ffi::c_long != 0 {
            litPrice = litPriceMax;
        }
        price = price.wrapping_sub(litPrice);
        u = u.wrapping_add(1);
        u;
    }
    price
}
unsafe extern "C" fn ZSTD_litLengthPrice(
    litLength: U32,
    optPtr: *const optState_t,
    mut optLevel: std::ffi::c_int,
) -> U32 {
    if (*optPtr).priceType as std::ffi::c_uint == zop_predef as std::ffi::c_int as std::ffi::c_uint
    {
        return if optLevel != 0 {
            ZSTD_fracWeight(litLength)
        } else {
            ZSTD_bitWeight(litLength)
        };
    }
    if litLength == ZSTD_BLOCKSIZE_MAX as U32 {
        return (BITCOST_MULTIPLIER as U32).wrapping_add(ZSTD_litLengthPrice(
            (ZSTD_BLOCKSIZE_MAX - 1 as std::ffi::c_int) as U32,
            optPtr,
            optLevel,
        ));
    }
    let llCode = ZSTD_LLcode(litLength);
    ((*LL_bits.as_ptr().offset(llCode as isize) as std::ffi::c_int * BITCOST_MULTIPLIER) as U32)
        .wrapping_add((*optPtr).litLengthSumBasePrice)
        .wrapping_sub(if optLevel != 0 {
            ZSTD_fracWeight(*((*optPtr).litLengthFreq).offset(llCode as isize))
        } else {
            ZSTD_bitWeight(*((*optPtr).litLengthFreq).offset(llCode as isize))
        })
}
#[inline(always)]
unsafe extern "C" fn ZSTD_getMatchPrice(
    offBase: U32,
    matchLength: U32,
    optPtr: *const optState_t,
    optLevel: std::ffi::c_int,
) -> U32 {
    let mut price: U32 = 0;
    let offCode = ZSTD_highbit32(offBase);
    let mlBase = matchLength.wrapping_sub(MINMATCH as U32);
    if (*optPtr).priceType as std::ffi::c_uint == zop_predef as std::ffi::c_int as std::ffi::c_uint
    {
        return (if optLevel != 0 {
            ZSTD_fracWeight(mlBase)
        } else {
            ZSTD_bitWeight(mlBase)
        })
        .wrapping_add(
            (16 as std::ffi::c_int as U32).wrapping_add(offCode) * BITCOST_MULTIPLIER as U32,
        );
    }
    price = (offCode * BITCOST_MULTIPLIER as U32).wrapping_add(
        ((*optPtr).offCodeSumBasePrice).wrapping_sub(if optLevel != 0 {
            ZSTD_fracWeight(*((*optPtr).offCodeFreq).offset(offCode as isize))
        } else {
            ZSTD_bitWeight(*((*optPtr).offCodeFreq).offset(offCode as isize))
        }),
    );
    if optLevel < 2 as std::ffi::c_int && offCode >= 20 as std::ffi::c_int as U32 {
        price = price.wrapping_add(
            offCode.wrapping_sub(19 as std::ffi::c_int as U32)
                * 2 as std::ffi::c_int as U32
                * BITCOST_MULTIPLIER as U32,
        );
    }
    let mlCode = ZSTD_MLcode(mlBase);
    price = price.wrapping_add(
        ((*ML_bits.as_ptr().offset(mlCode as isize) as std::ffi::c_int * BITCOST_MULTIPLIER)
            as U32)
            .wrapping_add(
                ((*optPtr).matchLengthSumBasePrice).wrapping_sub(if optLevel != 0 {
                    ZSTD_fracWeight(*((*optPtr).matchLengthFreq).offset(mlCode as isize))
                } else {
                    ZSTD_bitWeight(*((*optPtr).matchLengthFreq).offset(mlCode as isize))
                }),
            ),
    );
    price = price.wrapping_add((BITCOST_MULTIPLIER / 5 as std::ffi::c_int) as U32);
    price
}
unsafe extern "C" fn ZSTD_updateStats(
    optPtr: *mut optState_t,
    mut litLength: U32,
    mut literals: *const BYTE,
    mut offBase: U32,
    mut matchLength: U32,
) {
    if ZSTD_compressedLiterals(optPtr) != 0 {
        let mut u: U32 = 0;
        u = 0 as std::ffi::c_int as U32;
        while u < litLength {
            let fresh2 = &mut (*((*optPtr).litFreq).offset(*literals.offset(u as isize) as isize));
            *fresh2 = (*fresh2).wrapping_add(ZSTD_LITFREQ_ADD as std::ffi::c_uint);
            u = u.wrapping_add(1);
            u;
        }
        (*optPtr).litSum = ((*optPtr).litSum).wrapping_add(litLength * ZSTD_LITFREQ_ADD as U32);
    }
    let llCode = ZSTD_LLcode(litLength);
    let fresh3 = &mut (*((*optPtr).litLengthFreq).offset(llCode as isize));
    *fresh3 = (*fresh3).wrapping_add(1);
    *fresh3;
    (*optPtr).litLengthSum = ((*optPtr).litLengthSum).wrapping_add(1);
    (*optPtr).litLengthSum;
    let offCode = ZSTD_highbit32(offBase);
    let fresh4 = &mut (*((*optPtr).offCodeFreq).offset(offCode as isize));
    *fresh4 = (*fresh4).wrapping_add(1);
    *fresh4;
    (*optPtr).offCodeSum = ((*optPtr).offCodeSum).wrapping_add(1);
    (*optPtr).offCodeSum;
    let mlBase = matchLength.wrapping_sub(MINMATCH as U32);
    let mlCode = ZSTD_MLcode(mlBase);
    let fresh5 = &mut (*((*optPtr).matchLengthFreq).offset(mlCode as isize));
    *fresh5 = (*fresh5).wrapping_add(1);
    *fresh5;
    (*optPtr).matchLengthSum = ((*optPtr).matchLengthSum).wrapping_add(1);
    (*optPtr).matchLengthSum;
}
#[inline]
unsafe extern "C" fn ZSTD_readMINMATCH(
    mut memPtr: *const std::ffi::c_void,
    mut length: U32,
) -> U32 {
    match length {
        3 => {
            if MEM_isLittleEndian() != 0 {
                MEM_read32(memPtr) << 8 as std::ffi::c_int
            } else {
                MEM_read32(memPtr) >> 8 as std::ffi::c_int
            }
        }
        4 | _ => MEM_read32(memPtr),
    }
}
unsafe extern "C" fn ZSTD_insertAndFindFirstIndexHash3(
    mut ms: *const ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    ip: *const BYTE,
) -> U32 {
    let hashTable3 = (*ms).hashTable3;
    let hashLog3 = (*ms).hashLog3;
    let base = (*ms).window.base;
    let mut idx = *nextToUpdate3;
    let target = ip.offset_from(base) as std::ffi::c_long as U32;
    let hash3 = ZSTD_hash3Ptr(ip as *const std::ffi::c_void, hashLog3);
    while idx < target {
        *hashTable3.offset(ZSTD_hash3Ptr(
            base.offset(idx as isize) as *const std::ffi::c_void,
            hashLog3,
        ) as isize) = idx;
        idx = idx.wrapping_add(1);
        idx;
    }
    *nextToUpdate3 = target;
    *hashTable3.offset(hash3 as isize)
}
unsafe extern "C" fn ZSTD_insertBt1(
    mut ms: *const ZSTD_MatchState_t,
    ip: *const BYTE,
    iend: *const BYTE,
    target: U32,
    mls: U32,
    extDict: std::ffi::c_int,
) -> U32 {
    let cParams: *const ZSTD_compressionParameters = &(*ms).cParams;
    let hashTable = (*ms).hashTable;
    let hashLog = (*cParams).hashLog;
    let h = ZSTD_hashPtr(ip as *const std::ffi::c_void, hashLog, mls);
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let btMask = (((1 as std::ffi::c_int) << btLog) - 1 as std::ffi::c_int) as U32;
    let mut matchIndex = *hashTable.offset(h as isize);
    let mut commonLengthSmaller = 0 as std::ffi::c_int as size_t;
    let mut commonLengthLarger = 0 as std::ffi::c_int as size_t;
    let base = (*ms).window.base;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let mut match_0 = std::ptr::null::<BYTE>();
    let curr = ip.offset_from(base) as std::ffi::c_long as U32;
    let btLow = if btMask >= curr {
        0 as std::ffi::c_int as U32
    } else {
        curr.wrapping_sub(btMask)
    };
    let mut smallerPtr = bt.offset((2 as std::ffi::c_int as U32 * (curr & btMask)) as isize);
    let mut largerPtr = smallerPtr.offset(1 as std::ffi::c_int as isize);
    let mut dummy32: U32 = 0;
    let windowLow = ZSTD_getLowestMatchIndex(ms, target, (*cParams).windowLog);
    let mut matchEndIdx = curr
        .wrapping_add(8 as std::ffi::c_int as U32)
        .wrapping_add(1 as std::ffi::c_int as U32);
    let mut bestLength = 8 as std::ffi::c_int as size_t;
    let mut nbCompares = (1 as std::ffi::c_uint) << (*cParams).searchLog;
    *hashTable.offset(h as isize) = curr;
    while nbCompares != 0 && matchIndex >= windowLow {
        let nextPtr = bt.offset((2 as std::ffi::c_int as U32 * (matchIndex & btMask)) as isize);
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        if extDict == 0 || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
            match_0 = base.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.offset(matchLength as isize),
                match_0.offset(matchLength as isize),
                iend,
            ));
        } else {
            match_0 = dictBase.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.offset(matchLength as isize),
                match_0.offset(matchLength as isize),
                iend,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                match_0 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            bestLength = matchLength;
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                matchEndIdx = matchIndex.wrapping_add(matchLength as U32);
            }
        }
        if ip.offset(matchLength as isize) == iend {
            break;
        } else {
            if (*match_0.offset(matchLength as isize) as std::ffi::c_int)
                < *ip.offset(matchLength as isize) as std::ffi::c_int
            {
                *smallerPtr = matchIndex;
                commonLengthSmaller = matchLength;
                if matchIndex <= btLow {
                    smallerPtr = &mut dummy32;
                    break;
                } else {
                    smallerPtr = nextPtr.offset(1 as std::ffi::c_int as isize);
                    matchIndex = *nextPtr.offset(1 as std::ffi::c_int as isize);
                }
            } else {
                *largerPtr = matchIndex;
                commonLengthLarger = matchLength;
                if matchIndex <= btLow {
                    largerPtr = &mut dummy32;
                    break;
                } else {
                    largerPtr = nextPtr;
                    matchIndex = *nextPtr.offset(0 as std::ffi::c_int as isize);
                }
            }
            nbCompares = nbCompares.wrapping_sub(1);
            nbCompares;
        }
    }
    *largerPtr = 0 as std::ffi::c_int as U32;
    *smallerPtr = *largerPtr;
    let mut positions = 0 as std::ffi::c_int as U32;
    if bestLength > 384 as std::ffi::c_int as size_t {
        positions = if (192 as std::ffi::c_int as U32)
            < bestLength.wrapping_sub(384 as std::ffi::c_int as size_t) as U32
        {
            192 as std::ffi::c_int as U32
        } else {
            bestLength.wrapping_sub(384 as std::ffi::c_int as size_t) as U32
        };
    }
    if positions > matchEndIdx.wrapping_sub(curr.wrapping_add(8 as std::ffi::c_int as U32)) {
        positions
    } else {
        matchEndIdx.wrapping_sub(curr.wrapping_add(8 as std::ffi::c_int as U32))
    }
}
#[inline(always)]
unsafe extern "C" fn ZSTD_updateTree_internal(
    mut ms: *mut ZSTD_MatchState_t,
    ip: *const BYTE,
    iend: *const BYTE,
    mls: U32,
    dictMode: ZSTD_dictMode_e,
) {
    let base = (*ms).window.base;
    let target = ip.offset_from(base) as std::ffi::c_long as U32;
    let mut idx = (*ms).nextToUpdate;
    while idx < target {
        let forward = ZSTD_insertBt1(
            ms,
            base.offset(idx as isize),
            iend,
            target,
            mls,
            (dictMode as std::ffi::c_uint == ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int,
        );
        idx = idx.wrapping_add(forward);
    }
    (*ms).nextToUpdate = target;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_updateTree(
    mut ms: *mut ZSTD_MatchState_t,
    mut ip: *const BYTE,
    mut iend: *const BYTE,
) {
    ZSTD_updateTree_internal(ms, ip, iend, (*ms).cParams.minMatch, ZSTD_noDict);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_insertBtAndGetAllMatches(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    ip: *const BYTE,
    iLimit: *const BYTE,
    dictMode: ZSTD_dictMode_e,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
    mls: U32,
) -> U32 {
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let sufficient_len = if (*cParams).targetLength
        < (((1 as std::ffi::c_int) << 12 as std::ffi::c_int) - 1 as std::ffi::c_int)
            as std::ffi::c_uint
    {
        (*cParams).targetLength
    } else {
        (((1 as std::ffi::c_int) << 12 as std::ffi::c_int) - 1 as std::ffi::c_int)
            as std::ffi::c_uint
    };
    let base = (*ms).window.base;
    let curr = ip.offset_from(base) as std::ffi::c_long as U32;
    let hashLog = (*cParams).hashLog;
    let minMatch = (if mls == 3 as std::ffi::c_int as U32 {
        3 as std::ffi::c_int
    } else {
        4 as std::ffi::c_int
    }) as U32;
    let hashTable = (*ms).hashTable;
    let h = ZSTD_hashPtr(ip as *const std::ffi::c_void, hashLog, mls);
    let mut matchIndex = *hashTable.offset(h as isize);
    let bt = (*ms).chainTable;
    let btLog = ((*cParams).chainLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let btMask =
        ((1 as std::ffi::c_uint) << btLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
    let mut commonLengthSmaller = 0 as std::ffi::c_int as size_t;
    let mut commonLengthLarger = 0 as std::ffi::c_int as size_t;
    let dictBase = (*ms).window.dictBase;
    let dictLimit = (*ms).window.dictLimit;
    let dictEnd = dictBase.offset(dictLimit as isize);
    let prefixStart = base.offset(dictLimit as isize);
    let btLow = if btMask >= curr {
        0 as std::ffi::c_int as U32
    } else {
        curr.wrapping_sub(btMask)
    };
    let windowLow = ZSTD_getLowestMatchIndex(ms, curr, (*cParams).windowLog);
    let matchLow = if windowLow != 0 {
        windowLow
    } else {
        1 as std::ffi::c_int as U32
    };
    let mut smallerPtr = bt.offset((2 as std::ffi::c_int as U32 * (curr & btMask)) as isize);
    let mut largerPtr = bt
        .offset((2 as std::ffi::c_int as U32 * (curr & btMask)) as isize)
        .offset(1 as std::ffi::c_int as isize);
    let mut matchEndIdx = curr
        .wrapping_add(8 as std::ffi::c_int as U32)
        .wrapping_add(1 as std::ffi::c_int as U32);
    let mut dummy32: U32 = 0;
    let mut mnum = 0 as std::ffi::c_int as U32;
    let mut nbCompares = (1 as std::ffi::c_uint) << (*cParams).searchLog;
    let mut dms = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        (*ms).dictMatchState
    } else {
        NULL as *const ZSTD_MatchState_t
    };
    let dmsCParams = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        &(*dms).cParams
    } else {
        NULL as *const ZSTD_compressionParameters
    };
    let dmsBase = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        (*dms).window.base
    } else {
        NULL as *const BYTE
    };
    let dmsEnd = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        (*dms).window.nextSrc
    } else {
        NULL as *const BYTE
    };
    let dmsHighLimit = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        dmsEnd.offset_from(dmsBase) as std::ffi::c_long as U32
    } else {
        0 as std::ffi::c_int as U32
    };
    let dmsLowLimit = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        (*dms).window.lowLimit
    } else {
        0 as std::ffi::c_int as U32
    };
    let dmsIndexDelta = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        windowLow.wrapping_sub(dmsHighLimit)
    } else {
        0 as std::ffi::c_int as U32
    };
    let dmsHashLog = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        (*dmsCParams).hashLog
    } else {
        hashLog
    };
    let dmsBtLog = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        ((*dmsCParams).chainLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
    } else {
        btLog
    };
    let dmsBtMask = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
    {
        ((1 as std::ffi::c_uint) << dmsBtLog).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
    } else {
        0 as std::ffi::c_int as std::ffi::c_uint
    };
    let dmsBtLow = if dictMode as std::ffi::c_uint
        == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
        && dmsBtMask < dmsHighLimit.wrapping_sub(dmsLowLimit)
    {
        dmsHighLimit.wrapping_sub(dmsBtMask)
    } else {
        dmsLowLimit
    };
    let mut bestLength = lengthToBeat.wrapping_sub(1 as std::ffi::c_int as U32) as size_t;
    let lastR = (ZSTD_REP_NUM as U32).wrapping_add(ll0);
    let mut repCode: U32 = 0;
    repCode = ll0;
    while repCode < lastR {
        let repOffset = if repCode == ZSTD_REP_NUM as U32 {
            (*rep.offset(0 as std::ffi::c_int as isize)).wrapping_sub(1 as std::ffi::c_int as U32)
        } else {
            *rep.offset(repCode as isize)
        };
        let repIndex = curr.wrapping_sub(repOffset);
        let mut repLen = 0 as std::ffi::c_int as U32;
        if repOffset.wrapping_sub(1 as std::ffi::c_int as U32) < curr.wrapping_sub(dictLimit) {
            if (repIndex >= windowLow) as std::ffi::c_int
                & (ZSTD_readMINMATCH(ip as *const std::ffi::c_void, minMatch)
                    == ZSTD_readMINMATCH(
                        ip.offset(-(repOffset as isize)) as *const std::ffi::c_void,
                        minMatch,
                    )) as std::ffi::c_int
                != 0
            {
                repLen = (ZSTD_count(
                    ip.offset(minMatch as isize),
                    ip.offset(minMatch as isize).offset(-(repOffset as isize)),
                    iLimit,
                ) as U32)
                    .wrapping_add(minMatch);
            }
        } else {
            let repMatch = if dictMode as std::ffi::c_uint
                == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
            {
                dmsBase
                    .offset(repIndex as isize)
                    .offset(-(dmsIndexDelta as isize))
            } else {
                dictBase.offset(repIndex as isize)
            };
            if dictMode as std::ffi::c_uint == ZSTD_extDict as std::ffi::c_int as std::ffi::c_uint
                && (repOffset.wrapping_sub(1 as std::ffi::c_int as U32)
                    < curr.wrapping_sub(windowLow)) as std::ffi::c_int
                    & ZSTD_index_overlap_check(dictLimit, repIndex)
                    != 0
                && ZSTD_readMINMATCH(ip as *const std::ffi::c_void, minMatch)
                    == ZSTD_readMINMATCH(repMatch as *const std::ffi::c_void, minMatch)
            {
                repLen = (ZSTD_count_2segments(
                    ip.offset(minMatch as isize),
                    repMatch.offset(minMatch as isize),
                    iLimit,
                    dictEnd,
                    prefixStart,
                ) as U32)
                    .wrapping_add(minMatch);
            }
            if dictMode as std::ffi::c_uint
                == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
                && (repOffset.wrapping_sub(1 as std::ffi::c_int as U32)
                    < curr.wrapping_sub(dmsLowLimit.wrapping_add(dmsIndexDelta)))
                    as std::ffi::c_int
                    & ZSTD_index_overlap_check(dictLimit, repIndex)
                    != 0
                && ZSTD_readMINMATCH(ip as *const std::ffi::c_void, minMatch)
                    == ZSTD_readMINMATCH(repMatch as *const std::ffi::c_void, minMatch)
            {
                repLen = (ZSTD_count_2segments(
                    ip.offset(minMatch as isize),
                    repMatch.offset(minMatch as isize),
                    iLimit,
                    dmsEnd,
                    prefixStart,
                ) as U32)
                    .wrapping_add(minMatch);
            }
        }
        if repLen as size_t > bestLength {
            bestLength = repLen as size_t;
            (*matches.offset(mnum as isize)).off = repCode
                .wrapping_sub(ll0)
                .wrapping_add(1 as std::ffi::c_int as U32);
            (*matches.offset(mnum as isize)).len = repLen;
            mnum = mnum.wrapping_add(1);
            mnum;
            if (repLen > sufficient_len) as std::ffi::c_int
                | (ip.offset(repLen as isize) == iLimit) as std::ffi::c_int
                != 0
            {
                return mnum;
            }
        }
        repCode = repCode.wrapping_add(1);
        repCode;
    }
    if mls == 3 as std::ffi::c_int as U32 && bestLength < mls as size_t {
        let matchIndex3 = ZSTD_insertAndFindFirstIndexHash3(ms, nextToUpdate3, ip);
        if (matchIndex3 >= matchLow) as std::ffi::c_int
            & (curr.wrapping_sub(matchIndex3)
                < ((1 as std::ffi::c_int) << 18 as std::ffi::c_int) as U32)
                as std::ffi::c_int
            != 0
        {
            let mut mlen: size_t = 0;
            if dictMode as std::ffi::c_uint == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint
                || dictMode as std::ffi::c_uint
                    == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
                || matchIndex3 >= dictLimit
            {
                let match_0 = base.offset(matchIndex3 as isize);
                mlen = ZSTD_count(ip, match_0, iLimit);
            } else {
                let match_1 = dictBase.offset(matchIndex3 as isize);
                mlen = ZSTD_count_2segments(ip, match_1, iLimit, dictEnd, prefixStart);
            }
            if mlen >= mls as size_t {
                bestLength = mlen;
                (*matches.offset(0 as std::ffi::c_int as isize)).off = curr
                    .wrapping_sub(matchIndex3)
                    .wrapping_add(ZSTD_REP_NUM as U32);
                (*matches.offset(0 as std::ffi::c_int as isize)).len = mlen as U32;
                mnum = 1 as std::ffi::c_int as U32;
                if (mlen > sufficient_len as size_t) as std::ffi::c_int
                    | (ip.offset(mlen as isize) == iLimit) as std::ffi::c_int
                    != 0
                {
                    (*ms).nextToUpdate = curr.wrapping_add(1 as std::ffi::c_int as U32);
                    return 1 as std::ffi::c_int as U32;
                }
            }
        }
    }
    *hashTable.offset(h as isize) = curr;
    while nbCompares != 0 && matchIndex >= matchLow {
        let nextPtr = bt.offset((2 as std::ffi::c_int as U32 * (matchIndex & btMask)) as isize);
        let mut match_2 = std::ptr::null::<BYTE>();
        let mut matchLength = if commonLengthSmaller < commonLengthLarger {
            commonLengthSmaller
        } else {
            commonLengthLarger
        };
        if dictMode as std::ffi::c_uint == ZSTD_noDict as std::ffi::c_int as std::ffi::c_uint
            || dictMode as std::ffi::c_uint
                == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
            || (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t
        {
            match_2 = base.offset(matchIndex as isize);
            matchIndex >= dictLimit;
            matchLength = matchLength.wrapping_add(ZSTD_count(
                ip.offset(matchLength as isize),
                match_2.offset(matchLength as isize),
                iLimit,
            ));
        } else {
            match_2 = dictBase.offset(matchIndex as isize);
            matchLength = matchLength.wrapping_add(ZSTD_count_2segments(
                ip.offset(matchLength as isize),
                match_2.offset(matchLength as isize),
                iLimit,
                dictEnd,
                prefixStart,
            ));
            if (matchIndex as size_t).wrapping_add(matchLength) >= dictLimit as size_t {
                match_2 = base.offset(matchIndex as isize);
            }
        }
        if matchLength > bestLength {
            if matchLength > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                matchEndIdx = matchIndex.wrapping_add(matchLength as U32);
            }
            bestLength = matchLength;
            (*matches.offset(mnum as isize)).off = curr
                .wrapping_sub(matchIndex)
                .wrapping_add(ZSTD_REP_NUM as U32);
            (*matches.offset(mnum as isize)).len = matchLength as U32;
            mnum = mnum.wrapping_add(1);
            mnum;
            if (matchLength > ZSTD_OPT_NUM as size_t) as std::ffi::c_int
                | (ip.offset(matchLength as isize) == iLimit) as std::ffi::c_int
                != 0
            {
                if dictMode as std::ffi::c_uint
                    == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
                {
                    nbCompares = 0 as std::ffi::c_int as U32;
                }
                break;
            }
        }
        if (*match_2.offset(matchLength as isize) as std::ffi::c_int)
            < *ip.offset(matchLength as isize) as std::ffi::c_int
        {
            *smallerPtr = matchIndex;
            commonLengthSmaller = matchLength;
            if matchIndex <= btLow {
                smallerPtr = &mut dummy32;
                break;
            } else {
                smallerPtr = nextPtr.offset(1 as std::ffi::c_int as isize);
                matchIndex = *nextPtr.offset(1 as std::ffi::c_int as isize);
            }
        } else {
            *largerPtr = matchIndex;
            commonLengthLarger = matchLength;
            if matchIndex <= btLow {
                largerPtr = &mut dummy32;
                break;
            } else {
                largerPtr = nextPtr;
                matchIndex = *nextPtr.offset(0 as std::ffi::c_int as isize);
            }
        }
        nbCompares = nbCompares.wrapping_sub(1);
        nbCompares;
    }
    *largerPtr = 0 as std::ffi::c_int as U32;
    *smallerPtr = *largerPtr;
    if dictMode as std::ffi::c_uint == ZSTD_dictMatchState as std::ffi::c_int as std::ffi::c_uint
        && nbCompares != 0
    {
        let dmsH = ZSTD_hashPtr(ip as *const std::ffi::c_void, dmsHashLog, mls);
        let mut dictMatchIndex = *((*dms).hashTable).offset(dmsH as isize);
        let dmsBt: *const U32 = (*dms).chainTable;
        commonLengthLarger = 0 as std::ffi::c_int as size_t;
        commonLengthSmaller = commonLengthLarger;
        while nbCompares != 0 && dictMatchIndex > dmsLowLimit {
            let nextPtr_0 =
                dmsBt.offset((2 as std::ffi::c_int as U32 * (dictMatchIndex & dmsBtMask)) as isize);
            let mut matchLength_0 = if commonLengthSmaller < commonLengthLarger {
                commonLengthSmaller
            } else {
                commonLengthLarger
            };
            let mut match_3 = dmsBase.offset(dictMatchIndex as isize);
            matchLength_0 = matchLength_0.wrapping_add(ZSTD_count_2segments(
                ip.offset(matchLength_0 as isize),
                match_3.offset(matchLength_0 as isize),
                iLimit,
                dmsEnd,
                prefixStart,
            ));
            if (dictMatchIndex as size_t).wrapping_add(matchLength_0) >= dmsHighLimit as size_t {
                match_3 = base
                    .offset(dictMatchIndex as isize)
                    .offset(dmsIndexDelta as isize);
            }
            if matchLength_0 > bestLength {
                matchIndex = dictMatchIndex.wrapping_add(dmsIndexDelta);
                if matchLength_0 > matchEndIdx.wrapping_sub(matchIndex) as size_t {
                    matchEndIdx = matchIndex.wrapping_add(matchLength_0 as U32);
                }
                bestLength = matchLength_0;
                (*matches.offset(mnum as isize)).off = curr
                    .wrapping_sub(matchIndex)
                    .wrapping_add(ZSTD_REP_NUM as U32);
                (*matches.offset(mnum as isize)).len = matchLength_0 as U32;
                mnum = mnum.wrapping_add(1);
                mnum;
                if (matchLength_0 > ZSTD_OPT_NUM as size_t) as std::ffi::c_int
                    | (ip.offset(matchLength_0 as isize) == iLimit) as std::ffi::c_int
                    != 0
                {
                    break;
                }
            }
            if dictMatchIndex <= dmsBtLow {
                break;
            }
            if (*match_3.offset(matchLength_0 as isize) as std::ffi::c_int)
                < *ip.offset(matchLength_0 as isize) as std::ffi::c_int
            {
                commonLengthSmaller = matchLength_0;
                dictMatchIndex = *nextPtr_0.offset(1 as std::ffi::c_int as isize);
            } else {
                commonLengthLarger = matchLength_0;
                dictMatchIndex = *nextPtr_0.offset(0 as std::ffi::c_int as isize);
            }
            nbCompares = nbCompares.wrapping_sub(1);
            nbCompares;
        }
    }
    (*ms).nextToUpdate = matchEndIdx.wrapping_sub(8 as std::ffi::c_int as U32);
    mnum
}
#[inline(always)]
unsafe extern "C" fn ZSTD_btGetAllMatches_internal(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
    dictMode: ZSTD_dictMode_e,
    mls: U32,
) -> U32 {
    if ip < ((*ms).window.base).offset((*ms).nextToUpdate as isize) {
        return 0 as std::ffi::c_int as U32;
    }
    ZSTD_updateTree_internal(ms, ip, iHighLimit, mls, dictMode);
    ZSTD_insertBtAndGetAllMatches(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        dictMode,
        rep,
        ll0,
        lengthToBeat,
        mls,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_5(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        5 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_6(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        6 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_4(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        4 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_noDict_3(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_noDict,
        3 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_5(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        5 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_6(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        6 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_4(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        4 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_extDict_3(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_extDict,
        3 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_5(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        5 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_6(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        6 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_3(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        3 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_btGetAllMatches_dictMatchState_4(
    mut matches: *mut ZSTD_match_t,
    mut ms: *mut ZSTD_MatchState_t,
    mut nextToUpdate3: *mut U32,
    mut ip: *const BYTE,
    iHighLimit: *const BYTE,
    mut rep: *const U32,
    ll0: U32,
    lengthToBeat: U32,
) -> U32 {
    ZSTD_btGetAllMatches_internal(
        matches,
        ms,
        nextToUpdate3,
        ip,
        iHighLimit,
        rep,
        ll0,
        lengthToBeat,
        ZSTD_dictMatchState,
        4 as std::ffi::c_int as U32,
    )
}
unsafe extern "C" fn ZSTD_selectBtGetAllMatches(
    mut ms: *const ZSTD_MatchState_t,
    dictMode: ZSTD_dictMode_e,
) -> ZSTD_getAllMatchesFn {
    let getAllMatchesFns: [[ZSTD_getAllMatchesFn; 4]; 3] = [
        [
            Some(
                ZSTD_btGetAllMatches_noDict_3
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_4
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_5
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_noDict_6
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
        ],
        [
            Some(
                ZSTD_btGetAllMatches_extDict_3
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_4
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_5
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_extDict_6
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
        ],
        [
            Some(
                ZSTD_btGetAllMatches_dictMatchState_3
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_4
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_5
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
            Some(
                ZSTD_btGetAllMatches_dictMatchState_6
                    as unsafe extern "C" fn(
                        *mut ZSTD_match_t,
                        *mut ZSTD_MatchState_t,
                        *mut U32,
                        *const BYTE,
                        *const BYTE,
                        *const U32,
                        U32,
                        U32,
                    ) -> U32,
            ),
        ],
    ];
    let mls = if 3 as std::ffi::c_int as std::ffi::c_uint
        > (if (*ms).cParams.minMatch < 6 as std::ffi::c_int as std::ffi::c_uint {
            (*ms).cParams.minMatch
        } else {
            6 as std::ffi::c_int as std::ffi::c_uint
        }) {
        3 as std::ffi::c_int as std::ffi::c_uint
    } else if (*ms).cParams.minMatch < 6 as std::ffi::c_int as std::ffi::c_uint {
        (*ms).cParams.minMatch
    } else {
        6 as std::ffi::c_int as std::ffi::c_uint
    };
    *(*getAllMatchesFns
        .as_ptr()
        .offset(dictMode as std::ffi::c_int as isize))
    .as_ptr()
    .offset(mls.wrapping_sub(3 as std::ffi::c_int as U32) as isize)
}
unsafe extern "C" fn ZSTD_optLdm_skipRawSeqStoreBytes(
    mut rawSeqStore: *mut RawSeqStore_t,
    mut nbBytes: size_t,
) {
    let mut currPos = ((*rawSeqStore).posInSequence).wrapping_add(nbBytes) as U32;
    while currPos != 0 && (*rawSeqStore).pos < (*rawSeqStore).size {
        let mut currSeq = *((*rawSeqStore).seq).offset((*rawSeqStore).pos as isize);
        if currPos >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            currPos = currPos.wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength));
            (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
            (*rawSeqStore).pos;
        } else {
            (*rawSeqStore).posInSequence = currPos as size_t;
            break;
        }
    }
    if currPos == 0 as std::ffi::c_int as U32 || (*rawSeqStore).pos == (*rawSeqStore).size {
        (*rawSeqStore).posInSequence = 0 as std::ffi::c_int as size_t;
    }
}
unsafe extern "C" fn ZSTD_opt_getNextMatchAndUpdateSeqStore(
    mut optLdm: *mut ZSTD_optLdm_t,
    mut currPosInBlock: U32,
    mut blockBytesRemaining: U32,
) {
    let mut currSeq = rawSeq {
        offset: 0,
        litLength: 0,
        matchLength: 0,
    };
    let mut currBlockEndPos: U32 = 0;
    let mut literalsBytesRemaining: U32 = 0;
    let mut matchBytesRemaining: U32 = 0;
    if (*optLdm).seqStore.size == 0 as std::ffi::c_int as size_t
        || (*optLdm).seqStore.pos >= (*optLdm).seqStore.size
    {
        (*optLdm).startPosInBlock = UINT_MAX;
        (*optLdm).endPosInBlock = UINT_MAX;
        return;
    }
    currSeq = *((*optLdm).seqStore.seq).offset((*optLdm).seqStore.pos as isize);
    currBlockEndPos = currPosInBlock.wrapping_add(blockBytesRemaining);
    literalsBytesRemaining = if (*optLdm).seqStore.posInSequence < currSeq.litLength as size_t {
        (currSeq.litLength).wrapping_sub((*optLdm).seqStore.posInSequence as U32)
    } else {
        0 as std::ffi::c_int as U32
    };
    matchBytesRemaining = if literalsBytesRemaining == 0 as std::ffi::c_int as U32 {
        (currSeq.matchLength)
            .wrapping_sub(((*optLdm).seqStore.posInSequence as U32).wrapping_sub(currSeq.litLength))
    } else {
        currSeq.matchLength
    };
    if literalsBytesRemaining >= blockBytesRemaining {
        (*optLdm).startPosInBlock = UINT_MAX;
        (*optLdm).endPosInBlock = UINT_MAX;
        ZSTD_optLdm_skipRawSeqStoreBytes(&mut (*optLdm).seqStore, blockBytesRemaining as size_t);
        return;
    }
    (*optLdm).startPosInBlock = currPosInBlock.wrapping_add(literalsBytesRemaining);
    (*optLdm).endPosInBlock = ((*optLdm).startPosInBlock).wrapping_add(matchBytesRemaining);
    (*optLdm).offset = currSeq.offset;
    if (*optLdm).endPosInBlock > currBlockEndPos {
        (*optLdm).endPosInBlock = currBlockEndPos;
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut (*optLdm).seqStore,
            currBlockEndPos.wrapping_sub(currPosInBlock) as size_t,
        );
    } else {
        ZSTD_optLdm_skipRawSeqStoreBytes(
            &mut (*optLdm).seqStore,
            literalsBytesRemaining.wrapping_add(matchBytesRemaining) as size_t,
        );
    };
}
unsafe extern "C" fn ZSTD_optLdm_maybeAddMatch(
    mut matches: *mut ZSTD_match_t,
    mut nbMatches: *mut U32,
    mut optLdm: *const ZSTD_optLdm_t,
    mut currPosInBlock: U32,
    mut minMatch: U32,
) {
    let posDiff = currPosInBlock.wrapping_sub((*optLdm).startPosInBlock);
    let candidateMatchLength = ((*optLdm).endPosInBlock)
        .wrapping_sub((*optLdm).startPosInBlock)
        .wrapping_sub(posDiff);
    if currPosInBlock < (*optLdm).startPosInBlock
        || currPosInBlock >= (*optLdm).endPosInBlock
        || candidateMatchLength < minMatch
    {
        return;
    }
    if *nbMatches == 0 as std::ffi::c_int as U32
        || candidateMatchLength
            > (*matches.offset((*nbMatches).wrapping_sub(1 as std::ffi::c_int as U32) as isize)).len
            && *nbMatches < ZSTD_OPT_NUM as U32
    {
        let candidateOffBase = ((*optLdm).offset).wrapping_add(ZSTD_REP_NUM as U32);
        (*matches.offset(*nbMatches as isize)).len = candidateMatchLength;
        (*matches.offset(*nbMatches as isize)).off = candidateOffBase;
        *nbMatches = (*nbMatches).wrapping_add(1);
        *nbMatches;
    }
}
unsafe extern "C" fn ZSTD_optLdm_processMatchCandidate(
    mut optLdm: *mut ZSTD_optLdm_t,
    mut matches: *mut ZSTD_match_t,
    mut nbMatches: *mut U32,
    mut currPosInBlock: U32,
    mut remainingBytes: U32,
    mut minMatch: U32,
) {
    if (*optLdm).seqStore.size == 0 as std::ffi::c_int as size_t
        || (*optLdm).seqStore.pos >= (*optLdm).seqStore.size
    {
        return;
    }
    if currPosInBlock >= (*optLdm).endPosInBlock {
        if currPosInBlock > (*optLdm).endPosInBlock {
            let posOvershoot = currPosInBlock.wrapping_sub((*optLdm).endPosInBlock);
            ZSTD_optLdm_skipRawSeqStoreBytes(&mut (*optLdm).seqStore, posOvershoot as size_t);
        }
        ZSTD_opt_getNextMatchAndUpdateSeqStore(optLdm, currPosInBlock, remainingBytes);
    }
    ZSTD_optLdm_maybeAddMatch(matches, nbMatches, optLdm, currPosInBlock, minMatch);
}
#[inline(always)]
unsafe extern "C" fn ZSTD_compressBlock_opt_generic(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    optLevel: std::ffi::c_int,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    let mut current_block: u64;
    let optStatePtr: *mut optState_t = &mut (*ms).opt;
    let istart = src as *const BYTE;
    let mut ip = istart;
    let mut anchor = istart;
    let iend = istart.offset(srcSize as isize);
    let ilimit = iend.offset(-(8 as std::ffi::c_int as isize));
    let base = (*ms).window.base;
    let prefixStart = base.offset((*ms).window.dictLimit as isize);
    let cParams: *const ZSTD_compressionParameters = &mut (*ms).cParams;
    let mut getAllMatches = ZSTD_selectBtGetAllMatches(ms, dictMode);
    let sufficient_len = if (*cParams).targetLength
        < (((1 as std::ffi::c_int) << 12 as std::ffi::c_int) - 1 as std::ffi::c_int)
            as std::ffi::c_uint
    {
        (*cParams).targetLength
    } else {
        (((1 as std::ffi::c_int) << 12 as std::ffi::c_int) - 1 as std::ffi::c_int)
            as std::ffi::c_uint
    };
    let minMatch = (if (*cParams).minMatch == 3 as std::ffi::c_int as std::ffi::c_uint {
        3 as std::ffi::c_int
    } else {
        4 as std::ffi::c_int
    }) as U32;
    let mut nextToUpdate3 = (*ms).nextToUpdate;
    let opt = (*optStatePtr).priceTable;
    let matches = (*optStatePtr).matchTable;
    let mut lastStretch = ZSTD_optimal_t {
        price: 0,
        off: 0,
        mlen: 0,
        litlen: 0,
        rep: [0; 3],
    };
    let mut optLdm = ZSTD_optLdm_t {
        seqStore: RawSeqStore_t {
            seq: std::ptr::null_mut::<rawSeq>(),
            pos: 0,
            posInSequence: 0,
            size: 0,
            capacity: 0,
        },
        startPosInBlock: 0,
        endPosInBlock: 0,
        offset: 0,
    };
    libc::memset(
        &mut lastStretch as *mut ZSTD_optimal_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<ZSTD_optimal_t>() as std::ffi::c_ulong as libc::size_t,
    );
    optLdm.seqStore = if !((*ms).ldmSeqStore).is_null() {
        *(*ms).ldmSeqStore
    } else {
        kNullRawSeqStore
    };
    optLdm.offset = 0 as std::ffi::c_int as U32;
    optLdm.startPosInBlock = optLdm.offset;
    optLdm.endPosInBlock = optLdm.startPosInBlock;
    ZSTD_opt_getNextMatchAndUpdateSeqStore(
        &mut optLdm,
        ip.offset_from(istart) as std::ffi::c_long as U32,
        iend.offset_from(ip) as std::ffi::c_long as U32,
    );
    ZSTD_rescaleFreqs(optStatePtr, src as *const BYTE, srcSize, optLevel);
    ip = ip.offset((ip == prefixStart) as std::ffi::c_int as isize);
    while ip < ilimit {
        let mut cur: U32 = 0;
        let mut last_pos = 0 as std::ffi::c_int as U32;
        let litlen = ip.offset_from(anchor) as std::ffi::c_long as U32;
        let ll0 = (litlen == 0) as std::ffi::c_int as U32;
        let mut nbMatches = getAllMatches.unwrap_unchecked()(
            matches,
            ms,
            &mut nextToUpdate3,
            ip,
            iend,
            rep as *const U32,
            ll0,
            minMatch,
        );
        ZSTD_optLdm_processMatchCandidate(
            &mut optLdm,
            matches,
            &mut nbMatches,
            ip.offset_from(istart) as std::ffi::c_long as U32,
            iend.offset_from(ip) as std::ffi::c_long as U32,
            minMatch,
        );
        if nbMatches == 0 {
            ip = ip.offset(1);
            ip;
        } else {
            (*opt.offset(0 as std::ffi::c_int as isize)).mlen = 0 as std::ffi::c_int as U32;
            (*opt.offset(0 as std::ffi::c_int as isize)).litlen = litlen;
            (*opt.offset(0 as std::ffi::c_int as isize)).price =
                ZSTD_litLengthPrice(litlen, optStatePtr, optLevel) as std::ffi::c_int;
            libc::memcpy(
                &mut (*opt.offset(0 as std::ffi::c_int as isize)).rep as *mut [U32; 3]
                    as *mut std::ffi::c_void,
                rep as *const std::ffi::c_void,
                ::core::mem::size_of::<[U32; 3]>() as std::ffi::c_ulong as libc::size_t,
            );
            let maxML =
                (*matches.offset(nbMatches.wrapping_sub(1 as std::ffi::c_int as U32) as isize)).len;
            let maxOffBase =
                (*matches.offset(nbMatches.wrapping_sub(1 as std::ffi::c_int as U32) as isize)).off;
            if maxML > sufficient_len {
                lastStretch.litlen = 0 as std::ffi::c_int as U32;
                lastStretch.mlen = maxML;
                lastStretch.off = maxOffBase;
                cur = 0 as std::ffi::c_int as U32;
                last_pos = maxML;
            } else {
                let mut pos: U32 = 0;
                let mut matchNb: U32 = 0;
                pos = 1 as std::ffi::c_int as U32;
                while pos < minMatch {
                    (*opt.offset(pos as isize)).price = ZSTD_MAX_PRICE;
                    (*opt.offset(pos as isize)).mlen = 0 as std::ffi::c_int as U32;
                    (*opt.offset(pos as isize)).litlen = litlen.wrapping_add(pos);
                    pos = pos.wrapping_add(1);
                    pos;
                }
                matchNb = 0 as std::ffi::c_int as U32;
                while matchNb < nbMatches {
                    let offBase = (*matches.offset(matchNb as isize)).off;
                    let end = (*matches.offset(matchNb as isize)).len;
                    while pos <= end {
                        let matchPrice = ZSTD_getMatchPrice(offBase, pos, optStatePtr, optLevel)
                            as std::ffi::c_int;
                        let sequencePrice =
                            (*opt.offset(0 as std::ffi::c_int as isize)).price + matchPrice;
                        (*opt.offset(pos as isize)).mlen = pos;
                        (*opt.offset(pos as isize)).off = offBase;
                        (*opt.offset(pos as isize)).litlen = 0 as std::ffi::c_int as U32;
                        (*opt.offset(pos as isize)).price = sequencePrice
                            + ZSTD_litLengthPrice(
                                0 as std::ffi::c_int as U32,
                                optStatePtr,
                                optLevel,
                            ) as std::ffi::c_int;
                        pos = pos.wrapping_add(1);
                        pos;
                    }
                    matchNb = matchNb.wrapping_add(1);
                    matchNb;
                }
                last_pos = pos.wrapping_sub(1 as std::ffi::c_int as U32);
                (*opt.offset(pos as isize)).price = ZSTD_MAX_PRICE;
                cur = 1 as std::ffi::c_int as U32;
                loop {
                    if cur > last_pos {
                        current_block = 10357520176418200368;
                        break;
                    }
                    let inr = ip.offset(cur as isize);
                    let litlen_0 = ((*opt
                        .offset(cur.wrapping_sub(1 as std::ffi::c_int as U32) as isize))
                    .litlen)
                        .wrapping_add(1 as std::ffi::c_int as U32);
                    let price = (*opt
                        .offset(cur.wrapping_sub(1 as std::ffi::c_int as U32) as isize))
                    .price
                        + ZSTD_rawLiteralsCost(
                            ip.offset(cur as isize)
                                .offset(-(1 as std::ffi::c_int as isize)),
                            1 as std::ffi::c_int as U32,
                            optStatePtr,
                            optLevel,
                        ) as std::ffi::c_int
                        + (ZSTD_litLengthPrice(litlen_0, optStatePtr, optLevel) as std::ffi::c_int
                            - ZSTD_litLengthPrice(
                                litlen_0.wrapping_sub(1 as std::ffi::c_int as U32),
                                optStatePtr,
                                optLevel,
                            ) as std::ffi::c_int);
                    if price <= (*opt.offset(cur as isize)).price {
                        let prevMatch = *opt.offset(cur as isize);
                        *opt.offset(cur as isize) =
                            *opt.offset(cur.wrapping_sub(1 as std::ffi::c_int as U32) as isize);
                        (*opt.offset(cur as isize)).litlen = litlen_0;
                        (*opt.offset(cur as isize)).price = price;
                        if optLevel >= 1 as std::ffi::c_int
                            && prevMatch.litlen == 0 as std::ffi::c_int as U32
                            && (ZSTD_litLengthPrice(
                                1 as std::ffi::c_int as U32,
                                optStatePtr,
                                optLevel,
                            ) as std::ffi::c_int
                                - ZSTD_litLengthPrice(
                                    (1 as std::ffi::c_int - 1 as std::ffi::c_int) as U32,
                                    optStatePtr,
                                    optLevel,
                                ) as std::ffi::c_int)
                                < 0 as std::ffi::c_int
                            && (ip.offset(cur as isize) < iend) as std::ffi::c_int
                                as std::ffi::c_long
                                != 0
                        {
                            let mut with1literal = prevMatch.price
                                + ZSTD_rawLiteralsCost(
                                    ip.offset(cur as isize),
                                    1 as std::ffi::c_int as U32,
                                    optStatePtr,
                                    optLevel,
                                ) as std::ffi::c_int
                                + (ZSTD_litLengthPrice(
                                    1 as std::ffi::c_int as U32,
                                    optStatePtr,
                                    optLevel,
                                ) as std::ffi::c_int
                                    - ZSTD_litLengthPrice(
                                        (1 as std::ffi::c_int - 1 as std::ffi::c_int) as U32,
                                        optStatePtr,
                                        optLevel,
                                    ) as std::ffi::c_int);
                            let mut withMoreLiterals = price
                                + ZSTD_rawLiteralsCost(
                                    ip.offset(cur as isize),
                                    1 as std::ffi::c_int as U32,
                                    optStatePtr,
                                    optLevel,
                                ) as std::ffi::c_int
                                + (ZSTD_litLengthPrice(
                                    litlen_0.wrapping_add(1 as std::ffi::c_int as U32),
                                    optStatePtr,
                                    optLevel,
                                ) as std::ffi::c_int
                                    - ZSTD_litLengthPrice(
                                        litlen_0
                                            .wrapping_add(1 as std::ffi::c_int as U32)
                                            .wrapping_sub(1 as std::ffi::c_int as U32),
                                        optStatePtr,
                                        optLevel,
                                    ) as std::ffi::c_int);
                            if with1literal < withMoreLiterals
                                && with1literal
                                    < (*opt.offset(
                                        cur.wrapping_add(1 as std::ffi::c_int as U32) as isize
                                    ))
                                    .price
                            {
                                let prev = cur.wrapping_sub(prevMatch.mlen);
                                let newReps = ZSTD_newRep(
                                    ((*opt.offset(prev as isize)).rep).as_mut_ptr() as *const U32,
                                    prevMatch.off,
                                    ((*opt.offset(prev as isize)).litlen
                                        == 0 as std::ffi::c_int as U32)
                                        as std::ffi::c_int
                                        as U32,
                                );
                                *opt.offset(
                                    cur.wrapping_add(1 as std::ffi::c_int as U32) as isize
                                ) = prevMatch;
                                libc::memcpy(
                                    ((*opt.offset(
                                        cur.wrapping_add(1 as std::ffi::c_int as U32) as isize
                                    ))
                                    .rep)
                                        .as_mut_ptr()
                                        as *mut std::ffi::c_void,
                                    &newReps as *const Repcodes_t as *const std::ffi::c_void,
                                    ::core::mem::size_of::<Repcodes_t>() as std::ffi::c_ulong
                                        as libc::size_t,
                                );
                                (*opt.offset(
                                    cur.wrapping_add(1 as std::ffi::c_int as U32) as isize
                                ))
                                .litlen = 1 as std::ffi::c_int as U32;
                                (*opt.offset(
                                    cur.wrapping_add(1 as std::ffi::c_int as U32) as isize
                                ))
                                .price = with1literal;
                                if last_pos < cur.wrapping_add(1 as std::ffi::c_int as U32) {
                                    last_pos = cur.wrapping_add(1 as std::ffi::c_int as U32);
                                }
                            }
                        }
                    }
                    if (*opt.offset(cur as isize)).litlen == 0 as std::ffi::c_int as U32 {
                        let prev_0 = cur.wrapping_sub((*opt.offset(cur as isize)).mlen);
                        let newReps_0 = ZSTD_newRep(
                            ((*opt.offset(prev_0 as isize)).rep).as_mut_ptr() as *const U32,
                            (*opt.offset(cur as isize)).off,
                            ((*opt.offset(prev_0 as isize)).litlen == 0 as std::ffi::c_int as U32)
                                as std::ffi::c_int as U32,
                        );
                        libc::memcpy(
                            ((*opt.offset(cur as isize)).rep).as_mut_ptr() as *mut std::ffi::c_void,
                            &newReps_0 as *const Repcodes_t as *const std::ffi::c_void,
                            ::core::mem::size_of::<Repcodes_t>() as std::ffi::c_ulong
                                as libc::size_t,
                        );
                    }
                    if inr <= ilimit {
                        if cur == last_pos {
                            current_block = 10357520176418200368;
                            break;
                        }
                        if !(optLevel == 0 as std::ffi::c_int
                            && (*opt
                                .offset(cur.wrapping_add(1 as std::ffi::c_int as U32) as isize))
                            .price
                                <= (*opt.offset(cur as isize)).price
                                    + BITCOST_MULTIPLIER / 2 as std::ffi::c_int)
                        {
                            let ll0_0 = ((*opt.offset(cur as isize)).litlen
                                == 0 as std::ffi::c_int as U32)
                                as std::ffi::c_int as U32;
                            let previousPrice = (*opt.offset(cur as isize)).price;
                            let basePrice = previousPrice
                                + ZSTD_litLengthPrice(
                                    0 as std::ffi::c_int as U32,
                                    optStatePtr,
                                    optLevel,
                                ) as std::ffi::c_int;
                            let mut nbMatches_0 = getAllMatches.unwrap_unchecked()(
                                matches,
                                ms,
                                &mut nextToUpdate3,
                                inr,
                                iend,
                                ((*opt.offset(cur as isize)).rep).as_mut_ptr() as *const U32,
                                ll0_0,
                                minMatch,
                            );
                            let mut matchNb_0: U32 = 0;
                            ZSTD_optLdm_processMatchCandidate(
                                &mut optLdm,
                                matches,
                                &mut nbMatches_0,
                                inr.offset_from(istart) as std::ffi::c_long as U32,
                                iend.offset_from(inr) as std::ffi::c_long as U32,
                                minMatch,
                            );
                            if nbMatches_0 != 0 {
                                let longestML = (*matches
                                    .offset(nbMatches_0.wrapping_sub(1 as std::ffi::c_int as U32)
                                        as isize))
                                .len;
                                if longestML > sufficient_len
                                    || cur.wrapping_add(longestML) >= ZSTD_OPT_NUM as U32
                                    || ip.offset(cur as isize).offset(longestML as isize) >= iend
                                {
                                    lastStretch.mlen = longestML;
                                    lastStretch.off = (*matches.offset(
                                        nbMatches_0.wrapping_sub(1 as std::ffi::c_int as U32)
                                            as isize,
                                    ))
                                    .off;
                                    lastStretch.litlen = 0 as std::ffi::c_int as U32;
                                    last_pos = cur.wrapping_add(longestML);
                                    current_block = 12608488225262500095;
                                    break;
                                } else {
                                    matchNb_0 = 0 as std::ffi::c_int as U32;
                                    while matchNb_0 < nbMatches_0 {
                                        let offset = (*matches.offset(matchNb_0 as isize)).off;
                                        let lastML = (*matches.offset(matchNb_0 as isize)).len;
                                        let startML = if matchNb_0 > 0 as std::ffi::c_int as U32 {
                                            ((*matches.offset(
                                                matchNb_0.wrapping_sub(1 as std::ffi::c_int as U32)
                                                    as isize,
                                            ))
                                            .len)
                                                .wrapping_add(1 as std::ffi::c_int as U32)
                                        } else {
                                            minMatch
                                        };
                                        let mut mlen: U32 = 0;
                                        mlen = lastML;
                                        while mlen >= startML {
                                            let pos_0 = cur.wrapping_add(mlen);
                                            let price_0 = basePrice
                                                + ZSTD_getMatchPrice(
                                                    offset,
                                                    mlen,
                                                    optStatePtr,
                                                    optLevel,
                                                )
                                                    as std::ffi::c_int;
                                            if pos_0 > last_pos
                                                || price_0 < (*opt.offset(pos_0 as isize)).price
                                            {
                                                while last_pos < pos_0 {
                                                    last_pos = last_pos.wrapping_add(1);
                                                    last_pos;
                                                    (*opt.offset(last_pos as isize)).price =
                                                        ZSTD_MAX_PRICE;
                                                    (*opt.offset(last_pos as isize)).litlen =
                                                        (0 as std::ffi::c_int == 0)
                                                            as std::ffi::c_int
                                                            as U32;
                                                }
                                                (*opt.offset(pos_0 as isize)).mlen = mlen;
                                                (*opt.offset(pos_0 as isize)).off = offset;
                                                (*opt.offset(pos_0 as isize)).litlen =
                                                    0 as std::ffi::c_int as U32;
                                                (*opt.offset(pos_0 as isize)).price = price_0;
                                            } else if optLevel == 0 as std::ffi::c_int {
                                                break;
                                            }
                                            mlen = mlen.wrapping_sub(1);
                                            mlen;
                                        }
                                        matchNb_0 = matchNb_0.wrapping_add(1);
                                        matchNb_0;
                                    }
                                    (*opt.offset(
                                        last_pos.wrapping_add(1 as std::ffi::c_int as U32) as isize,
                                    ))
                                    .price = ZSTD_MAX_PRICE;
                                }
                            }
                        }
                    }
                    cur = cur.wrapping_add(1);
                    cur;
                }
                match current_block {
                    12608488225262500095 => {}
                    _ => {
                        lastStretch = *opt.offset(last_pos as isize);
                        cur = last_pos.wrapping_sub(lastStretch.mlen);
                    }
                }
            }
            if lastStretch.mlen == 0 as std::ffi::c_int as U32 {
                ip = ip.offset(last_pos as isize);
            } else {
                if lastStretch.litlen == 0 as std::ffi::c_int as U32 {
                    let reps = ZSTD_newRep(
                        ((*opt.offset(cur as isize)).rep).as_mut_ptr() as *const U32,
                        lastStretch.off,
                        ((*opt.offset(cur as isize)).litlen == 0 as std::ffi::c_int as U32)
                            as std::ffi::c_int as U32,
                    );
                    libc::memcpy(
                        rep as *mut std::ffi::c_void,
                        &reps as *const Repcodes_t as *const std::ffi::c_void,
                        ::core::mem::size_of::<Repcodes_t>() as std::ffi::c_ulong as libc::size_t,
                    );
                } else {
                    libc::memcpy(
                        rep as *mut std::ffi::c_void,
                        (lastStretch.rep).as_mut_ptr() as *const std::ffi::c_void,
                        ::core::mem::size_of::<Repcodes_t>() as std::ffi::c_ulong as libc::size_t,
                    );
                    cur = cur.wrapping_sub(lastStretch.litlen);
                }
                let storeEnd = cur.wrapping_add(2 as std::ffi::c_int as U32);
                let mut storeStart = storeEnd;
                let mut stretchPos = cur;
                if lastStretch.litlen > 0 as std::ffi::c_int as U32 {
                    (*opt.offset(storeEnd as isize)).litlen = lastStretch.litlen;
                    (*opt.offset(storeEnd as isize)).mlen = 0 as std::ffi::c_int as U32;
                    storeStart = storeEnd.wrapping_sub(1 as std::ffi::c_int as U32);
                    *opt.offset(storeStart as isize) = lastStretch;
                }
                *opt.offset(storeEnd as isize) = lastStretch;
                storeStart = storeEnd;
                loop {
                    let mut nextStretch = *opt.offset(stretchPos as isize);
                    (*opt.offset(storeStart as isize)).litlen = nextStretch.litlen;
                    if nextStretch.mlen == 0 as std::ffi::c_int as U32 {
                        break;
                    }
                    storeStart = storeStart.wrapping_sub(1);
                    storeStart;
                    *opt.offset(storeStart as isize) = nextStretch;
                    stretchPos = stretchPos
                        .wrapping_sub((nextStretch.litlen).wrapping_add(nextStretch.mlen));
                }
                let mut storePos: U32 = 0;
                storePos = storeStart;
                while storePos <= storeEnd {
                    let llen = (*opt.offset(storePos as isize)).litlen;
                    let mlen_0 = (*opt.offset(storePos as isize)).mlen;
                    let offBase_0 = (*opt.offset(storePos as isize)).off;
                    let advance = llen.wrapping_add(mlen_0);
                    if mlen_0 == 0 as std::ffi::c_int as U32 {
                        ip = anchor.offset(llen as isize);
                    } else {
                        ZSTD_updateStats(optStatePtr, llen, anchor, offBase_0, mlen_0);
                        ZSTD_storeSeq(
                            seqStore,
                            llen as size_t,
                            anchor,
                            iend,
                            offBase_0,
                            mlen_0 as size_t,
                        );
                        anchor = anchor.offset(advance as isize);
                        ip = anchor;
                    }
                    storePos = storePos.wrapping_add(1);
                    storePos;
                }
                ZSTD_setBasePrices(optStatePtr, optLevel);
            }
        }
    }
    iend.offset_from(anchor) as std::ffi::c_long as size_t
}
unsafe extern "C" fn ZSTD_compressBlock_opt0(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    ZSTD_compressBlock_opt_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        0 as std::ffi::c_int,
        dictMode,
    )
}
unsafe extern "C" fn ZSTD_compressBlock_opt2(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    dictMode: ZSTD_dictMode_e,
) -> size_t {
    ZSTD_compressBlock_opt_generic(
        ms,
        seqStore,
        rep,
        src,
        srcSize,
        2 as std::ffi::c_int,
        dictMode,
    )
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_noDict)
}
unsafe extern "C" fn ZSTD_initStats_ultra(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) {
    let mut tmpRep: [U32; 3] = [0; 3];
    libc::memcpy(
        tmpRep.as_mut_ptr() as *mut std::ffi::c_void,
        rep as *const std::ffi::c_void,
        ::core::mem::size_of::<[U32; 3]>() as std::ffi::c_ulong as libc::size_t,
    );
    ZSTD_compressBlock_opt2(ms, seqStore, tmpRep.as_mut_ptr(), src, srcSize, ZSTD_noDict);
    ZSTD_resetSeqStore(seqStore);
    (*ms).window.base = ((*ms).window.base).offset(-(srcSize as isize));
    (*ms).window.dictLimit = ((*ms).window.dictLimit).wrapping_add(srcSize as U32);
    (*ms).window.lowLimit = (*ms).window.dictLimit;
    (*ms).nextToUpdate = (*ms).window.dictLimit;
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_noDict)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra2(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    let curr = (src as *const BYTE).offset_from((*ms).window.base) as std::ffi::c_long as U32;
    if (*ms).opt.litLengthSum == 0 as std::ffi::c_int as U32
        && (*seqStore).sequences == (*seqStore).sequencesStart
        && (*ms).window.dictLimit == (*ms).window.lowLimit
        && curr == (*ms).window.dictLimit
        && srcSize > ZSTD_PREDEF_THRESHOLD as size_t
    {
        ZSTD_initStats_ultra(ms, seqStore, rep, src, srcSize);
    }
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_noDict)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_dictMatchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_dictMatchState)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btopt_extDict(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt0(ms, seqStore, rep, src, srcSize, ZSTD_extDict)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_dictMatchState(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_dictMatchState)
}
#[no_mangle]
pub unsafe extern "C" fn ZSTD_compressBlock_btultra_extDict(
    mut ms: *mut ZSTD_MatchState_t,
    mut seqStore: *mut SeqStore_t,
    mut rep: *mut U32,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    ZSTD_compressBlock_opt2(ms, seqStore, rep, src, srcSize, ZSTD_extDict)
}
pub const __INT_MAX__: std::ffi::c_int = 2147483647 as std::ffi::c_int;
