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
#[repr(C)]
pub struct ZSTD_entropyCTables_t {
    pub huf: ZSTD_hufCTables_t,
    pub fse: ZSTD_fseCTables_t,
}
#[repr(C)]
pub struct ZSTD_fseCTables_t {
    pub offcodeCTable: [FSE_CTable; 193],
    pub matchlengthCTable: [FSE_CTable; 363],
    pub litlengthCTable: [FSE_CTable; 329],
    pub offcode_repeatMode: FSE_repeat,
    pub matchlength_repeatMode: FSE_repeat,
    pub litlength_repeatMode: FSE_repeat,
}
#[repr(C)]
pub struct ZSTD_hufCTables_t {
    pub CTable: [HUF_CElt; 257],
    pub repeatMode: HUF_repeat,
}
#[repr(C)]
pub struct ZSTD_match_t {
    pub off: u32,
    pub len: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmState_t {
    pub window: ZSTD_window_t,
    pub hashTable: *mut ldmEntry_t,
    pub loadedDictEnd: u32,
    pub bucketOffsets: *mut u8,
    pub splitIndices: [size_t; 64],
    pub matchCandidates: [ldmMatchCandidate_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmMatchCandidate_t {
    pub split: *const u8,
    pub hash: u32,
    pub checksum: u32,
    pub bucket: *mut ldmEntry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmEntry_t {
    pub offset: u32,
    pub checksum: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldmParams_t {
    pub enableLdm: ZSTD_ParamSwitch_e,
    pub hashLog: u32,
    pub bucketSizeLog: u32,
    pub minMatchLength: u32,
    pub hashRateLog: u32,
    pub windowLog: u32,
}
pub type ZSTD_dictTableLoadMethod_e = core::ffi::c_uint;
pub const ZSTD_dtlm_full: ZSTD_dictTableLoadMethod_e = 1;
pub const ZSTD_dtlm_fast: ZSTD_dictTableLoadMethod_e = 0;
pub type ZSTD_tableFillPurpose_e = core::ffi::c_uint;
pub const ZSTD_tfp_forCDict: ZSTD_tableFillPurpose_e = 1;
pub const ZSTD_tfp_forCCtx: ZSTD_tableFillPurpose_e = 0;
pub type ZSTD_dictMode_e = core::ffi::c_uint;
pub const ZSTD_dedicatedDictSearch: ZSTD_dictMode_e = 3;
pub const ZSTD_dictMatchState: ZSTD_dictMode_e = 2;
pub const ZSTD_extDict: ZSTD_dictMode_e = 1;
pub const ZSTD_noDict: ZSTD_dictMode_e = 0;
pub type ZSTD_BlockCompressor_f = Option<
    unsafe fn(
        &mut ZSTD_MatchState_t,
        &mut SeqStore_t,
        *mut u32,
        *const core::ffi::c_void,
        size_t,
    ) -> size_t,
>;
#[repr(C)]
pub struct ldmRollingHashState_t {
    pub rolling: u64,
    pub stopMask: u64,
}

use libc::size_t;

use crate::lib::common::error_private::{ERR_isError, Error};
use crate::lib::common::fse::{FSE_CTable, FSE_repeat};
use crate::lib::common::huf::{HUF_CElt, HUF_repeat};
use crate::lib::common::xxhash::ZSTD_XXH64;
use crate::lib::common::zstd_internal::ZSTD_REP_NUM;
use crate::lib::compress::zstd_compress::{
    rawSeq, RawSeqStore_t, SeqStore_t, ZSTD_MatchState_t, ZSTD_optimal_t,
    ZSTD_selectBlockCompressor, ZSTD_window_t,
};
use crate::lib::compress::zstd_compress_internal::{
    ZSTD_OptPrice_e, ZSTD_count, ZSTD_count_2segments, ZSTD_storeSeq,
    ZSTD_window_needOverflowCorrection, ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY,
};
use crate::lib::compress::zstd_double_fast::ZSTD_fillDoubleHashTable;
use crate::lib::compress::zstd_fast::ZSTD_fillHashTable;
use crate::lib::zstd::*;
pub const HASH_READ_SIZE: core::ffi::c_int = 8;
pub const ZSTD_WINDOW_START_INDEX: core::ffi::c_int = 2;
pub const LDM_BATCH_SIZE: core::ffi::c_int = 64;

#[inline]
unsafe fn ZSTD_window_hasExtDict(window: ZSTD_window_t) -> u32 {
    core::ffi::c_int::from(window.lowLimit < window.dictLimit) as u32
}
#[inline]
unsafe fn ZSTD_matchState_dictMode(ms: *const ZSTD_MatchState_t) -> ZSTD_dictMode_e {
    (if ZSTD_window_hasExtDict((*ms).window) != 0 {
        ZSTD_extDict as core::ffi::c_int
    } else if !((*ms).dictMatchState).is_null() {
        if (*(*ms).dictMatchState).dedicatedDictSearch != 0 {
            ZSTD_dedicatedDictSearch as core::ffi::c_int
        } else {
            ZSTD_dictMatchState as core::ffi::c_int
        }
    } else {
        ZSTD_noDict as core::ffi::c_int
    }) as ZSTD_dictMode_e
}
#[inline]
unsafe fn ZSTD_window_correctOverflow(
    window: *mut ZSTD_window_t,
    cycleLog: u32,
    maxDist: u32,
    src: *const core::ffi::c_void,
) -> u32 {
    let cycleSize = (1 as core::ffi::c_uint) << cycleLog;
    let cycleMask = cycleSize.wrapping_sub(1);
    let curr = (src as *const u8).offset_from((*window).base) as core::ffi::c_long as u32;
    let currentCycle = curr & cycleMask;
    let currentCycleCorrection = if currentCycle < ZSTD_WINDOW_START_INDEX as u32 {
        if cycleSize > 2 {
            cycleSize
        } else {
            2
        }
    } else {
        0
    };
    let newCurrent = currentCycle
        .wrapping_add(currentCycleCorrection)
        .wrapping_add(if maxDist > cycleSize {
            maxDist
        } else {
            cycleSize
        });
    let correction = curr.wrapping_sub(newCurrent);
    if ZSTD_WINDOW_OVERFLOW_CORRECT_FREQUENTLY == 0 {
        // Loose bound, should be around 1<<29 (see above)
        assert!(correction > 1 << 28);
    }
    (*window).base = ((*window).base).offset(correction as isize);
    (*window).dictBase = ((*window).dictBase).offset(correction as isize);
    if (*window).lowLimit < correction.wrapping_add(ZSTD_WINDOW_START_INDEX as u32) {
        (*window).lowLimit = ZSTD_WINDOW_START_INDEX as u32;
    } else {
        (*window).lowLimit = ((*window).lowLimit).wrapping_sub(correction);
    }
    if (*window).dictLimit < correction.wrapping_add(ZSTD_WINDOW_START_INDEX as u32) {
        (*window).dictLimit = ZSTD_WINDOW_START_INDEX as u32;
    } else {
        (*window).dictLimit = ((*window).dictLimit).wrapping_sub(correction);
    }
    (*window).nbOverflowCorrections = ((*window).nbOverflowCorrections).wrapping_add(1);
    correction
}
#[inline]
unsafe fn ZSTD_window_enforceMaxDist(
    window: *mut ZSTD_window_t,
    blockEnd: *const core::ffi::c_void,
    maxDist: u32,
    loadedDictEndPtr: *mut u32,
    dictMatchStatePtr: *mut *const ZSTD_MatchState_t,
) {
    let blockEndIdx =
        (blockEnd as *const u8).offset_from((*window).base) as core::ffi::c_long as u32;
    let loadedDictEnd = if !loadedDictEndPtr.is_null() {
        *loadedDictEndPtr
    } else {
        0
    };
    if blockEndIdx > maxDist.wrapping_add(loadedDictEnd) {
        let newLowLimit = blockEndIdx.wrapping_sub(maxDist);
        if (*window).lowLimit < newLowLimit {
            (*window).lowLimit = newLowLimit;
        }
        if (*window).dictLimit < (*window).lowLimit {
            (*window).dictLimit = (*window).lowLimit;
        }
        if !loadedDictEndPtr.is_null() {
            *loadedDictEndPtr = 0;
        }
        if !dictMatchStatePtr.is_null() {
            *dictMatchStatePtr = core::ptr::null();
        }
    }
}
#[inline]
unsafe fn ZSTD_cwksp_alloc_size(size: size_t) -> size_t {
    if size == 0 {
        return 0;
    }
    size
}

static ZSTD_ldm_gearTab: [u64; 256] = [
    0xf5b8f72c5f77775c,
    0x84935f266b7ac412,
    0xb647ada9ca730ccc,
    0xb065bb4b114fb1de,
    0x34584e7e8c3a9fd0,
    0x4e97e17c6ae26b05,
    0x3a03d743bc99a604,
    0xcecd042422c4044f,
    0x76de76c58524259e,
    0x9c8528f65badeaca,
    0x86563706e2097529,
    0x2902475fa375d889,
    0xafb32a9739a5ebe6,
    0xce2714da3883e639,
    0x21eaf821722e69e,
    0x37b628620b628,
    0x49a8d455d88caf5,
    0x8556d711e6958140,
    0x4f7ae74fc605c1f,
    0x829f0c3468bd3a20,
    0x4ffdc885c625179e,
    0x8473de048a3daf1b,
    0x51008822b05646b2,
    0x69d75d12b2d1cc5f,
    0x8c9d4a19159154bc,
    0xc3cc10f4abbd4003,
    0xd06ddc1cecb97391,
    0xbe48e6e7ed80302e,
    0x3481db31cee03547,
    0xacc3f67cdaa1d210,
    0x65cb771d8c7f96cc,
    0x8eb27177055723dd,
    0xc789950d44cd94be,
    0x934feadc3700b12b,
    0x5e485f11edbdf182,
    0x1e2e2a46fd64767a,
    0x2969ca71d82efa7c,
    0x9d46e9935ebbba2e,
    0xe056b67e05e6822b,
    0x94d73f55739d03a0,
    0xcd7010bdb69b5a03,
    0x455ef9fcd79b82f4,
    0x869cb54a8749c161,
    0x38d1a4fa6185d225,
    0xb475166f94bbe9bb,
    0xa4143548720959f1,
    0x7aed4780ba6b26ba,
    0xd0ce264439e02312,
    0x84366d746078d508,
    0xa8ce973c72ed17be,
    0x21c323a29a430b01,
    0x9962d617e3af80ee,
    0xab0ce91d9c8cf75b,
    0x530e8ee6d19a4dbc,
    0x2ef68c0cf53f5d72,
    0xc03a681640a85506,
    0x496e4e9f9c310967,
    0x78580472b59b14a0,
    0x273824c23b388577,
    0x66bf923ad45cb553,
    0x47ae1a5a2492ba86,
    0x35e304569e229659,
    0x4765182a46870b6f,
    0x6cbab625e9099412,
    0xddac9a2e598522c1,
    0x7172086e666624f2,
    0xdf5003ca503b7837,
    0x88c0c1db78563d09,
    0x58d51865acfc289d,
    0x177671aec65224f1,
    0xfb79d8a241e967d7,
    0x2be1e101cad9a49a,
    0x6625682f6e29186b,
    0x399553457ac06e50,
    0x35dffb4c23abb74,
    0x429db2591f54aade,
    0xc52802a8037d1009,
    0x6acb27381f0b25f3,
    0xf45e2551ee4f823b,
    0x8b0ea2d99580c2f7,
    0x3bed519cbcb4e1e1,
    0xff452823dbb010a,
    0x9d42ed614f3dd267,
    0x5b9313c06257c57b,
    0xa114b8008b5e1442,
    0xc1fe311c11c13d4b,
    0x66e8763ea34c5568,
    0x8b982af1c262f05d,
    0xee8876faaa75fbb7,
    0x8a62a4d0d172bb2a,
    0xc13d94a3b7449a97,
    0x6dbbba9dc15d037c,
    0xc786101f1d92e0f1,
    0xd78681a907a0b79b,
    0xf61aaf2962c9abb9,
    0x2cfd16fcd3cb7ad9,
    0x868c5b6744624d21,
    0x25e650899c74ddd7,
    0xba042af4a7c37463,
    0x4eb1a539465a3eca,
    0xbe09dbf03b05d5ca,
    0x774e5a362b5472ba,
    0x47a1221229d183cd,
    0x504b0ca18ef5a2df,
    0xdffbdfbde2456eb9,
    0x46cd2b2fbee34634,
    0xf2aef8fe819d98c3,
    0x357f5276d4599d61,
    0x24a5483879c453e3,
    0x88026889192b4b9,
    0x28da96671782dbec,
    0x4ef37c40588e9aaa,
    0x8837b90651bc9fb3,
    0xc164f741d3f0e5d6,
    0xbc135a0a704b70ba,
    0x69cd868f7622ada,
    0xbc37ba89e0b9c0ab,
    0x47c14a01323552f6,
    0x4f00794bacee98bb,
    0x7107de7d637a69d5,
    0x88af793bb6f2255e,
    0xf3c6466b8799b598,
    0xc288c616aa7f3b59,
    0x81ca63cf42fca3fd,
    0x88d85ace36a2674b,
    0xd056bd3792389e7,
    0xe55c396c4e9dd32d,
    0xbefb504571e6c0a6,
    0x96ab32115e91e8cc,
    0xbf8acb18de8f38d1,
    0x66dae58801672606,
    0x833b6017872317fb,
    0xb87c16f2d1c92864,
    0xdb766a74e58b669c,
    0x89659f85c61417be,
    0xc8daad856011ea0c,
    0x76a4b565b6fe7eae,
    0xa469d085f6237312,
    0xaaf0365683a3e96c,
    0x4dbb746f8424f7b8,
    0x638755af4e4acc1,
    0x3d7807f5bde64486,
    0x17be6d8f5bbb7639,
    0x903f0cd44dc35dc,
    0x67b672eafdf1196c,
    0xa676ff93ed4c82f1,
    0x521d1004c5053d9d,
    0x37ba9ad09ccc9202,
    0x84e54d297aacfb51,
    0xa0b4b776a143445,
    0x820d471e20b348e,
    0x1874383cb83d46dc,
    0x97edeec7a1efe11c,
    0xb330e50b1bdc42aa,
    0x1dd91955ce70e032,
    0xa514cdb88f2939d5,
    0x2791233fd90db9d3,
    0x7b670a4cc50f7a9b,
    0x77c07d2a05c6dfa5,
    0xe3778b6646d0a6fa,
    0xb39c8eda47b56749,
    0x933ed448addbef28,
    0xaf846af6ab7d0bf4,
    0xe5af208eb666e49,
    0x5e6622f73534cd6a,
    0x297daeca42ef5b6e,
    0x862daef3d35539a6,
    0xe68722498f8e1ea9,
    0x981c53093dc0d572,
    0xfa09b0bfbf86fbf5,
    0x30b1e96166219f15,
    0x70e7d466bdc4fb83,
    0x5a66736e35f2a8e9,
    0xcddb59d2b7c1baef,
    0xd6c7d247d26d8996,
    0xea4e39eac8de1ba3,
    0x539c8bb19fa3aff2,
    0x9f90e4c5fd508d8,
    0xa34e5956fbaf3385,
    0x2e2f8e151d3ef375,
    0x173691e9b83faec1,
    0xb85a8d56bf016379,
    0x8382381267408ae3,
    0xb90f901bbdc0096d,
    0x7c6ad32933bcec65,
    0x76bb5e2f2c8ad595,
    0x390f851a6cf46d28,
    0xc3e6064da1c2da72,
    0xc52a0c101cfa5389,
    0xd78eaf84a3fbc530,
    0x3781b9e2288b997e,
    0x73c2f6dea83d05c4,
    0x4228e364c5b5ed7,
    0x9d7a3edf0da43911,
    0x8edcfeda24686756,
    0x5e7667a7b7a9b3a1,
    0x4c4f389fa143791d,
    0xb08bc1023da7cddc,
    0x7ab4be3ae529b1cc,
    0x754e6132dbe74ff9,
    0x71635442a839df45,
    0x2f6fb1643fbe52de,
    0x961e0a42cf7a8177,
    0xf3b45d83d89ef2ea,
    0xee3de4cf4a6e3e9b,
    0xcd6848542c3295e7,
    0xe4cee1664c78662f,
    0x9947548b474c68c4,
    0x25d73777a5ed8b0b,
    0xc915b1d636b7fc,
    0x21c2ba75d9b0d2da,
    0x5f6b5dcf608a64a1,
    0xdcf333255ff9570c,
    0x633b922418ced4ee,
    0xc136dde0b004b34a,
    0x58cc83b05d4b2f5a,
    0x5eb424dda28e42d2,
    0x62df47369739cd98,
    0xb4e0b42485e4ce17,
    0x16e1f0c1f9a8d1e7,
    0x8ec3916707560ebf,
    0x62ba6e2df2cc9db3,
    0xcbf9f4ff77d83a16,
    0x78d9d7d07d2bbcc4,
    0xef554ce1e02c41f4,
    0x8d7581127eccf94d,
    0xa9b53336cb3c8a05,
    0x38c42c0bf45c4f91,
    0x640893cdf4488863,
    0x80ec34bc575ea568,
    0x39f324f5b48eaa40,
    0xe9d9ed1f8eff527f,
    0x9224fc058cc5a214,
    0xbaba00b04cfe7741,
    0x309a9f120fcf52af,
    0xa558f3ec65626212,
    0x424bec8b7adabe2f,
    0x41622513a6aea433,
    0xb88da2d5324ca798,
    0xd287733b245528a4,
    0x9a44697e6d68aec3,
    0x7b1093be2f49bb28,
    0x50bbec632e3d8aad,
    0x6cd90723e1ea8283,
    0x897b9e7431b02bf3,
    0x219efdcb338a7047,
    0x3b0311f0a27c0656,
    0xdb17bf91c0db96e7,
    0x8cd4fd6b4e85a5b2,
    0xfab071054ba6409d,
    0x40d6fe831fa9dfd9,
    0xaf358debad7d791e,
    0xeb8d0e25a65e3e58,
    0xbbcbd3df14e08580,
    0xcf751f27ecdab2b,
    0x2b4da14f2613d8f4,
];
pub const LDM_MIN_MATCH_LENGTH: core::ffi::c_int = 64;
unsafe fn ZSTD_ldm_gear_init(state: *mut ldmRollingHashState_t, params: *const ldmParams_t) {
    let maxBitsInMask = if (*params).minMatchLength < 64 {
        (*params).minMatchLength
    } else {
        64
    };
    let hashRateLog = (*params).hashRateLog;
    (*state).rolling = u64::from(!0u32);
    if hashRateLog > 0 as core::ffi::c_uint && hashRateLog <= maxBitsInMask {
        (*state).stopMask =
            (1u64 << hashRateLog).wrapping_sub(1) << maxBitsInMask.wrapping_sub(hashRateLog);
    } else {
        (*state).stopMask = (1u64 << hashRateLog).wrapping_sub(1);
    };
}
unsafe fn ZSTD_ldm_gear_reset(
    state: *mut ldmRollingHashState_t,
    data: *const u8,
    minMatchLength: size_t,
) {
    let mut hash = (*state).rolling;
    let mut n = 0 as size_t;
    while n.wrapping_add(3) < minMatchLength {
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
    }
    while n < minMatchLength {
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
    }
}
unsafe fn ZSTD_ldm_gear_feed(
    state: *mut ldmRollingHashState_t,
    data: *const u8,
    size: size_t,
    splits: *mut size_t,
    numSplits: *mut core::ffi::c_uint,
) -> size_t {
    let mut current_block: u64;
    let mut n: size_t = 0;
    let mut hash: u64 = 0;
    let mut mask: u64 = 0;
    hash = (*state).rolling;
    mask = (*state).stopMask;
    n = 0;
    loop {
        if n.wrapping_add(3) >= size {
            current_block = 5689316957504528238;
            break;
        }
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
        if hash & mask == 0 {
            *splits.offset(*numSplits as isize) = n;
            *numSplits = (*numSplits).wrapping_add(1);
            if *numSplits == LDM_BATCH_SIZE as core::ffi::c_uint {
                current_block = 12351618399163395313;
                break;
            }
        }
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
        if hash & mask == 0 {
            *splits.offset(*numSplits as isize) = n;
            *numSplits = (*numSplits).wrapping_add(1);
            if *numSplits == LDM_BATCH_SIZE as core::ffi::c_uint {
                current_block = 12351618399163395313;
                break;
            }
        }
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
        if hash & mask == 0 {
            *splits.offset(*numSplits as isize) = n;
            *numSplits = (*numSplits).wrapping_add(1);
            if *numSplits == LDM_BATCH_SIZE as core::ffi::c_uint {
                current_block = 12351618399163395313;
                break;
            }
        }
        hash =
            (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
            ));
        n = n.wrapping_add(1);
        if core::ffi::c_long::from(core::ffi::c_int::from(hash & mask == 0)) == 0 {
            continue;
        }
        *splits.offset(*numSplits as isize) = n;
        *numSplits = (*numSplits).wrapping_add(1);
        if *numSplits == LDM_BATCH_SIZE as core::ffi::c_uint {
            current_block = 12351618399163395313;
            break;
        }
    }
    loop {
        match current_block {
            12351618399163395313 => {
                (*state).rolling = hash;
                break;
            }
            _ => {
                if n >= size {
                    current_block = 12351618399163395313;
                    continue;
                }
                hash = (hash << 1).wrapping_add(*ZSTD_ldm_gearTab.as_ptr().offset(
                    (core::ffi::c_int::from(*data.add(n)) & 0xff as core::ffi::c_int) as isize,
                ));
                n = n.wrapping_add(1);
                if core::ffi::c_long::from(core::ffi::c_int::from(hash & mask == 0)) == 0 {
                    current_block = 5689316957504528238;
                    continue;
                }
                *splits.offset(*numSplits as isize) = n;
                *numSplits = (*numSplits).wrapping_add(1);
                if *numSplits == LDM_BATCH_SIZE as core::ffi::c_uint {
                    current_block = 12351618399163395313;
                } else {
                    current_block = 5689316957504528238;
                }
            }
        }
    }
    n
}
pub unsafe fn ZSTD_ldm_adjustParameters(
    params: *mut ldmParams_t,
    cParams: *const ZSTD_compressionParameters,
) {
    (*params).windowLog = (*cParams).windowLog;
    if (*params).hashRateLog == 0 {
        if (*params).hashLog > 0 {
            if (*params).windowLog > (*params).hashLog {
                (*params).hashRateLog = ((*params).windowLog).wrapping_sub((*params).hashLog);
            }
        } else {
            (*params).hashRateLog = (7 as core::ffi::c_uint)
                .wrapping_sub(((*cParams).strategy as core::ffi::c_uint).wrapping_div(3));
        }
    }
    if (*params).hashLog == 0 {
        (*params).hashLog = if 6
            > (if ((*params).windowLog).wrapping_sub((*params).hashRateLog)
                < (if (if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                    30
                } else {
                    31
                }) < 30
                {
                    if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                        30
                    } else {
                        31
                    }
                } else {
                    30
                }) as u32
            {
                ((*params).windowLog).wrapping_sub((*params).hashRateLog)
            } else {
                (if (if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                    30
                } else {
                    31
                }) < 30
                {
                    if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                        30
                    } else {
                        31
                    }
                } else {
                    30
                }) as u32
            }) {
            6
        } else if ((*params).windowLog).wrapping_sub((*params).hashRateLog)
            < (if (if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                30
            } else {
                31
            }) < 30
            {
                if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                    30
                } else {
                    31
                }
            } else {
                30
            }) as u32
        {
            ((*params).windowLog).wrapping_sub((*params).hashRateLog)
        } else {
            (if (if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                30
            } else {
                31
            }) < 30
            {
                if ::core::mem::size_of::<size_t>() as core::ffi::c_ulong == 4 {
                    30
                } else {
                    31
                }
            } else {
                30
            }) as u32
        };
    }
    if (*params).minMatchLength == 0 {
        (*params).minMatchLength = LDM_MIN_MATCH_LENGTH as u32;
        if (*cParams).strategy as core::ffi::c_uint
            >= ZSTD_btultra as core::ffi::c_int as core::ffi::c_uint
        {
            (*params).minMatchLength /= 2;
        }
    }
    if (*params).bucketSizeLog == 0 {
        (*params).bucketSizeLog = if 4
            > (if (*cParams).strategy < 8 {
                (*cParams).strategy
            } else {
                8
            }) {
            4
        } else if (*cParams).strategy < 8 {
            (*cParams).strategy
        } else {
            8
        };
    }
    (*params).bucketSizeLog = if (*params).bucketSizeLog < (*params).hashLog {
        (*params).bucketSizeLog
    } else {
        (*params).hashLog
    };
}
pub unsafe fn ZSTD_ldm_getTableSize(params: ldmParams_t) -> size_t {
    let ldmHSize = (1 as size_t) << params.hashLog;
    let ldmBucketSizeLog = (if params.bucketSizeLog < params.hashLog {
        params.bucketSizeLog
    } else {
        params.hashLog
    }) as size_t;
    let ldmBucketSize = (1) << (params.hashLog as size_t).wrapping_sub(ldmBucketSizeLog);
    let totalSize = (ZSTD_cwksp_alloc_size(ldmBucketSize)).wrapping_add(ZSTD_cwksp_alloc_size(
        ldmHSize.wrapping_mul(::core::mem::size_of::<ldmEntry_t>()),
    ));
    if params.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        totalSize
    } else {
        0
    }
}
pub unsafe fn ZSTD_ldm_getMaxNbSeq(params: ldmParams_t, maxChunkSize: size_t) -> size_t {
    if params.enableLdm == ZSTD_ParamSwitch_e::ZSTD_ps_enable {
        maxChunkSize / params.minMatchLength as size_t
    } else {
        0
    }
}
unsafe fn ZSTD_ldm_getBucket(
    ldmState: *const ldmState_t,
    hash: size_t,
    bucketSizeLog: u32,
) -> *mut ldmEntry_t {
    ((*ldmState).hashTable).add(hash << bucketSizeLog)
}
unsafe fn ZSTD_ldm_insertEntry(
    ldmState: *mut ldmState_t,
    hash: size_t,
    entry: ldmEntry_t,
    bucketSizeLog: u32,
) {
    let pOffset = ((*ldmState).bucketOffsets).add(hash);
    let offset = core::ffi::c_uint::from(*pOffset);
    *(ZSTD_ldm_getBucket(ldmState, hash, bucketSizeLog)).offset(offset as isize) = entry;
    *pOffset = (offset.wrapping_add(1)
        & ((1 as core::ffi::c_uint) << bucketSizeLog).wrapping_sub(1)) as u8;
}
unsafe fn ZSTD_ldm_countBackwardsMatch(
    mut pIn: *const u8,
    pAnchor: *const u8,
    mut pMatch: *const u8,
    pMatchBase: *const u8,
) -> size_t {
    let mut matchLength = 0 as size_t;
    while pIn > pAnchor
        && pMatch > pMatchBase
        && core::ffi::c_int::from(*pIn.sub(1)) == core::ffi::c_int::from(*pMatch.sub(1))
    {
        pIn = pIn.sub(1);
        pMatch = pMatch.sub(1);
        matchLength = matchLength.wrapping_add(1);
    }
    matchLength
}
unsafe fn ZSTD_ldm_countBackwardsMatch_2segments(
    pIn: *const u8,
    pAnchor: *const u8,
    pMatch: *const u8,
    pMatchBase: *const u8,
    pExtDictStart: *const u8,
    pExtDictEnd: *const u8,
) -> size_t {
    let mut matchLength = ZSTD_ldm_countBackwardsMatch(pIn, pAnchor, pMatch, pMatchBase);
    if pMatch.offset(-(matchLength as isize)) != pMatchBase || pMatchBase == pExtDictStart {
        return matchLength;
    }
    matchLength = matchLength.wrapping_add(ZSTD_ldm_countBackwardsMatch(
        pIn.offset(-(matchLength as isize)),
        pAnchor,
        pExtDictEnd,
        pExtDictStart,
    ));
    matchLength
}
unsafe fn ZSTD_ldm_fillFastTables(
    ms: &mut ZSTD_MatchState_t,
    end: *const core::ffi::c_void,
) -> size_t {
    let iend = end as *const u8;
    match ms.cParams.strategy as core::ffi::c_uint {
        1 => {
            ZSTD_fillHashTable(
                ms,
                iend as *const core::ffi::c_void,
                ZSTD_dtlm_fast,
                ZSTD_tfp_forCCtx,
            );
        }
        2 => {
            ZSTD_fillDoubleHashTable(
                ms,
                iend as *const core::ffi::c_void,
                ZSTD_dtlm_fast,
                ZSTD_tfp_forCCtx,
            );
        }
        3 | 4 | 5 | 6 | 7 | 8 | 9 | _ => {}
    }
    0
}
pub unsafe fn ZSTD_ldm_fillHashTable(
    ldmState: *mut ldmState_t,
    mut ip: *const u8,
    iend: *const u8,
    params: *const ldmParams_t,
) {
    let minMatchLength = (*params).minMatchLength;
    let bucketSizeLog = (*params).bucketSizeLog;
    let hBits = ((*params).hashLog).wrapping_sub(bucketSizeLog);
    let base = (*ldmState).window.base;
    let istart = ip;
    let mut hashState = ldmRollingHashState_t {
        rolling: 0,
        stopMask: 0,
    };
    let splits = ((*ldmState).splitIndices).as_mut_ptr();
    let mut numSplits: core::ffi::c_uint = 0;
    ZSTD_ldm_gear_init(&mut hashState, params);
    while ip < iend {
        let mut hashed: size_t = 0;
        let mut n: core::ffi::c_uint = 0;
        numSplits = 0;
        hashed = ZSTD_ldm_gear_feed(
            &mut hashState,
            ip,
            iend.offset_from_unsigned(ip),
            splits,
            &mut numSplits,
        );
        n = 0;
        while n < numSplits {
            if ip.add(*splits.offset(n as isize)) >= istart.offset(minMatchLength as isize) {
                let split = ip
                    .add(*splits.offset(n as isize))
                    .offset(-(minMatchLength as isize));
                let xxhash = ZSTD_XXH64(
                    split as *const core::ffi::c_void,
                    minMatchLength as usize,
                    0,
                );
                let hash = (xxhash & u64::from((1u32 << hBits).wrapping_sub(1))) as u32;
                let mut entry = ldmEntry_t {
                    offset: 0,
                    checksum: 0,
                };
                entry.offset = split.offset_from(base) as core::ffi::c_long as u32;
                entry.checksum = (xxhash >> 32) as u32;
                ZSTD_ldm_insertEntry(ldmState, hash as size_t, entry, (*params).bucketSizeLog);
            }
            n = n.wrapping_add(1);
        }
        ip = ip.add(hashed);
    }
}
unsafe fn ZSTD_ldm_limitTableUpdate(ms: &mut ZSTD_MatchState_t, anchor: *const u8) {
    let curr = anchor.offset_from(ms.window.base) as core::ffi::c_long as u32;
    if curr > (ms.nextToUpdate).wrapping_add(1024) {
        ms.nextToUpdate = curr.wrapping_sub(
            if (512) < curr.wrapping_sub(ms.nextToUpdate).wrapping_sub(1024) {
                512
            } else {
                curr.wrapping_sub(ms.nextToUpdate).wrapping_sub(1024)
            },
        );
    }
}
unsafe fn ZSTD_ldm_generateSequences_internal(
    ldmState: *mut ldmState_t,
    rawSeqStore: *mut RawSeqStore_t,
    params: *const ldmParams_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let extDict = ZSTD_window_hasExtDict((*ldmState).window) as core::ffi::c_int;
    let minMatchLength = (*params).minMatchLength;
    let entsPerBucket = (1) << (*params).bucketSizeLog;
    let hBits = ((*params).hashLog).wrapping_sub((*params).bucketSizeLog);
    let dictLimit = (*ldmState).window.dictLimit;
    let lowestIndex = if extDict != 0 {
        (*ldmState).window.lowLimit
    } else {
        dictLimit
    };
    let base = (*ldmState).window.base;
    let dictBase = if extDict != 0 {
        (*ldmState).window.dictBase
    } else {
        core::ptr::null()
    };
    let dictStart = if extDict != 0 {
        dictBase.offset(lowestIndex as isize)
    } else {
        core::ptr::null()
    };
    let dictEnd = if extDict != 0 {
        dictBase.offset(dictLimit as isize)
    } else {
        core::ptr::null()
    };
    let lowPrefixPtr = base.offset(dictLimit as isize);
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let ilimit = iend.offset(-(HASH_READ_SIZE as isize));
    let mut anchor = istart;
    let mut ip = istart;
    let mut hashState = ldmRollingHashState_t {
        rolling: 0,
        stopMask: 0,
    };
    let splits = ((*ldmState).splitIndices).as_mut_ptr();
    let candidates = ((*ldmState).matchCandidates).as_mut_ptr();
    let mut numSplits: core::ffi::c_uint = 0;
    if srcSize < minMatchLength as size_t {
        return iend.offset_from_unsigned(anchor);
    }
    ZSTD_ldm_gear_init(&mut hashState, params);
    ZSTD_ldm_gear_reset(&mut hashState, ip, minMatchLength as size_t);
    ip = ip.offset(minMatchLength as isize);
    while ip < ilimit {
        let mut hashed: size_t = 0;
        let mut n: core::ffi::c_uint = 0;
        numSplits = 0;
        hashed = ZSTD_ldm_gear_feed(
            &mut hashState,
            ip,
            ilimit.offset_from_unsigned(ip),
            splits,
            &mut numSplits,
        );
        n = 0;
        while n < numSplits {
            let split = ip
                .add(*splits.offset(n as isize))
                .offset(-(minMatchLength as isize));
            let xxhash = ZSTD_XXH64(
                split as *const core::ffi::c_void,
                minMatchLength as usize,
                0,
            );
            let hash = (xxhash & u64::from((1u32 << hBits).wrapping_sub(1))) as u32;
            let fresh2 = &mut (*candidates.offset(n as isize)).split;
            *fresh2 = split;
            (*candidates.offset(n as isize)).hash = hash;
            (*candidates.offset(n as isize)).checksum = (xxhash >> 32) as u32;
            let fresh3 = &mut (*candidates.offset(n as isize)).bucket;
            *fresh3 = ZSTD_ldm_getBucket(ldmState, hash as size_t, (*params).bucketSizeLog);
            n = n.wrapping_add(1);
        }
        n = 0;
        while n < numSplits {
            let mut forwardMatchLength = 0;
            let mut backwardMatchLength = 0;
            let mut bestMatchLength = 0;
            let mut mLength: size_t = 0;
            let mut offset: u32 = 0;
            let split_0 = (*candidates.offset(n as isize)).split;
            let checksum = (*candidates.offset(n as isize)).checksum;
            let hash_0 = (*candidates.offset(n as isize)).hash;
            let bucket = (*candidates.offset(n as isize)).bucket;
            let mut cur = core::ptr::null::<ldmEntry_t>();
            let mut bestEntry = core::ptr::null();
            let mut newEntry = ldmEntry_t {
                offset: 0,
                checksum: 0,
            };
            newEntry.offset = split_0.offset_from(base) as core::ffi::c_long as u32;
            newEntry.checksum = checksum;
            if split_0 < anchor {
                ZSTD_ldm_insertEntry(
                    ldmState,
                    hash_0 as size_t,
                    newEntry,
                    (*params).bucketSizeLog,
                );
            } else {
                let mut current_block_30: u64;
                cur = bucket;
                while cur < bucket.offset(entsPerBucket as isize) as *const ldmEntry_t {
                    let mut curForwardMatchLength: size_t = 0;
                    let mut curBackwardMatchLength: size_t = 0;
                    let mut curTotalMatchLength: size_t = 0;
                    if !((*cur).checksum != checksum || (*cur).offset <= lowestIndex) {
                        if extDict != 0 {
                            let curMatchBase = if (*cur).offset < dictLimit {
                                dictBase
                            } else {
                                base
                            };
                            let pMatch = curMatchBase.offset((*cur).offset as isize);
                            let matchEnd = if (*cur).offset < dictLimit {
                                dictEnd
                            } else {
                                iend
                            };
                            let lowMatchPtr = if (*cur).offset < dictLimit {
                                dictStart
                            } else {
                                lowPrefixPtr
                            };
                            curForwardMatchLength =
                                ZSTD_count_2segments(split_0, pMatch, iend, matchEnd, lowPrefixPtr);
                            if curForwardMatchLength < minMatchLength as size_t {
                                current_block_30 = 17788412896529399552;
                            } else {
                                curBackwardMatchLength = ZSTD_ldm_countBackwardsMatch_2segments(
                                    split_0,
                                    anchor,
                                    pMatch,
                                    lowMatchPtr,
                                    dictStart,
                                    dictEnd,
                                );
                                current_block_30 = 15512526488502093901;
                            }
                        } else {
                            let pMatch_0 = base.offset((*cur).offset as isize);
                            curForwardMatchLength = ZSTD_count(split_0, pMatch_0, iend);
                            if curForwardMatchLength < minMatchLength as size_t {
                                current_block_30 = 17788412896529399552;
                            } else {
                                curBackwardMatchLength = ZSTD_ldm_countBackwardsMatch(
                                    split_0,
                                    anchor,
                                    pMatch_0,
                                    lowPrefixPtr,
                                );
                                current_block_30 = 15512526488502093901;
                            }
                        }
                        match current_block_30 {
                            17788412896529399552 => {}
                            _ => {
                                curTotalMatchLength =
                                    curForwardMatchLength.wrapping_add(curBackwardMatchLength);
                                if curTotalMatchLength > bestMatchLength {
                                    bestMatchLength = curTotalMatchLength;
                                    forwardMatchLength = curForwardMatchLength;
                                    backwardMatchLength = curBackwardMatchLength;
                                    bestEntry = cur;
                                }
                            }
                        }
                    }
                    cur = cur.add(1);
                }
                if bestEntry.is_null() {
                    ZSTD_ldm_insertEntry(
                        ldmState,
                        hash_0 as size_t,
                        newEntry,
                        (*params).bucketSizeLog,
                    );
                } else {
                    offset = (split_0.offset_from(base) as core::ffi::c_long as u32)
                        .wrapping_sub((*bestEntry).offset);
                    mLength = forwardMatchLength.wrapping_add(backwardMatchLength);
                    let seq = ((*rawSeqStore).seq).add((*rawSeqStore).size);
                    if (*rawSeqStore).size == (*rawSeqStore).capacity {
                        return Error::dstSize_tooSmall.to_error_code();
                    }
                    (*seq).litLength = split_0
                        .offset(-(backwardMatchLength as isize))
                        .offset_from(anchor)
                        as core::ffi::c_long as u32;
                    (*seq).matchLength = mLength as u32;
                    (*seq).offset = offset;
                    (*rawSeqStore).size = ((*rawSeqStore).size).wrapping_add(1);
                    ZSTD_ldm_insertEntry(
                        ldmState,
                        hash_0 as size_t,
                        newEntry,
                        (*params).bucketSizeLog,
                    );
                    anchor = split_0.add(forwardMatchLength);
                    if anchor > ip.add(hashed) {
                        ZSTD_ldm_gear_reset(
                            &mut hashState,
                            anchor.offset(-(minMatchLength as isize)),
                            minMatchLength as size_t,
                        );
                        ip = anchor.offset(-(hashed as isize));
                        break;
                    }
                }
            }
            n = n.wrapping_add(1);
        }
        ip = ip.add(hashed);
    }
    iend.offset_from_unsigned(anchor)
}
unsafe fn ZSTD_ldm_reduceTable(table: *mut ldmEntry_t, size: u32, reducerValue: u32) {
    let mut u: u32 = 0;
    u = 0;
    while u < size {
        if (*table.offset(u as isize)).offset < reducerValue {
            (*table.offset(u as isize)).offset = 0;
        } else {
            let fresh4 = &mut (*table.offset(u as isize)).offset;
            *fresh4 = (*fresh4).wrapping_sub(reducerValue);
        }
        u = u.wrapping_add(1);
    }
}
pub unsafe fn ZSTD_ldm_generateSequences(
    ldmState: *mut ldmState_t,
    sequences: *mut RawSeqStore_t,
    params: *const ldmParams_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let maxDist = (1) << (*params).windowLog;
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let kMaxChunkSize = ((1) << 20) as size_t;
    let nbChunks = (srcSize / kMaxChunkSize)
        .wrapping_add(core::ffi::c_int::from(!srcSize.is_multiple_of(kMaxChunkSize)) as size_t);
    let mut chunk: size_t = 0;
    let mut leftoverSize = 0;
    chunk = 0;
    while chunk < nbChunks && (*sequences).size < (*sequences).capacity {
        let chunkStart = istart.add(chunk * kMaxChunkSize);
        let remaining = iend.offset_from_unsigned(chunkStart);
        let chunkEnd = if remaining < kMaxChunkSize {
            iend
        } else {
            chunkStart.add(kMaxChunkSize)
        };
        let chunkSize = chunkEnd.offset_from_unsigned(chunkStart);
        let mut newLeftoverSize: size_t = 0;
        let prevSize = (*sequences).size;
        if ZSTD_window_needOverflowCorrection(
            (*ldmState).window,
            0,
            maxDist,
            (*ldmState).loadedDictEnd,
            chunkStart as *const core::ffi::c_void,
            chunkEnd as *const core::ffi::c_void,
        ) {
            let ldmHSize = (1) << (*params).hashLog;
            let correction = ZSTD_window_correctOverflow(
                &mut (*ldmState).window,
                0,
                maxDist,
                chunkStart as *const core::ffi::c_void,
            );
            ZSTD_ldm_reduceTable((*ldmState).hashTable, ldmHSize, correction);
            (*ldmState).loadedDictEnd = 0;
        }
        ZSTD_window_enforceMaxDist(
            &mut (*ldmState).window,
            chunkEnd as *const core::ffi::c_void,
            maxDist,
            &mut (*ldmState).loadedDictEnd,
            core::ptr::null_mut(),
        );
        newLeftoverSize = ZSTD_ldm_generateSequences_internal(
            ldmState,
            sequences,
            params,
            chunkStart as *const core::ffi::c_void,
            chunkSize,
        );
        if ERR_isError(newLeftoverSize) {
            return newLeftoverSize;
        }
        if prevSize < (*sequences).size {
            let fresh5 = &mut (*((*sequences).seq).add(prevSize)).litLength;
            *fresh5 = (*fresh5).wrapping_add(leftoverSize as u32);
            leftoverSize = newLeftoverSize;
        } else {
            leftoverSize = leftoverSize.wrapping_add(chunkSize);
        }
        chunk = chunk.wrapping_add(1);
    }
    0
}
pub unsafe fn ZSTD_ldm_skipSequences(
    rawSeqStore: *mut RawSeqStore_t,
    mut srcSize: size_t,
    minMatch: u32,
) {
    while srcSize > 0 && (*rawSeqStore).pos < (*rawSeqStore).size {
        let seq = ((*rawSeqStore).seq).add((*rawSeqStore).pos);
        if srcSize <= (*seq).litLength as size_t {
            (*seq).litLength = ((*seq).litLength).wrapping_sub(srcSize as u32);
            return;
        }
        srcSize = srcSize.wrapping_sub((*seq).litLength as size_t);
        (*seq).litLength = 0;
        if srcSize < (*seq).matchLength as size_t {
            (*seq).matchLength = ((*seq).matchLength).wrapping_sub(srcSize as u32);
            if (*seq).matchLength < minMatch {
                if ((*rawSeqStore).pos).wrapping_add(1) < (*rawSeqStore).size {
                    let fresh6 = &mut (*seq.add(1)).litLength;
                    *fresh6 = (*fresh6).wrapping_add((*seq).matchLength);
                }
                (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
            }
            return;
        }
        srcSize = srcSize.wrapping_sub((*seq).matchLength as size_t);
        (*seq).matchLength = 0;
        (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
    }
}
unsafe fn maybeSplitSequence(
    rawSeqStore: *mut RawSeqStore_t,
    remaining: u32,
    minMatch: u32,
) -> rawSeq {
    let mut sequence = *((*rawSeqStore).seq).add((*rawSeqStore).pos);
    if remaining >= (sequence.litLength).wrapping_add(sequence.matchLength) {
        (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
        return sequence;
    }
    if remaining <= sequence.litLength {
        sequence.offset = 0;
    } else if remaining < (sequence.litLength).wrapping_add(sequence.matchLength) {
        sequence.matchLength = remaining.wrapping_sub(sequence.litLength);
        if sequence.matchLength < minMatch {
            sequence.offset = 0;
        }
    }
    ZSTD_ldm_skipSequences(rawSeqStore, remaining as size_t, minMatch);
    sequence
}
pub unsafe fn ZSTD_ldm_skipRawSeqStoreBytes(rawSeqStore: *mut RawSeqStore_t, nbBytes: size_t) {
    let mut currPos = ((*rawSeqStore).posInSequence).wrapping_add(nbBytes) as u32;
    while currPos != 0 && (*rawSeqStore).pos < (*rawSeqStore).size {
        let currSeq = *((*rawSeqStore).seq).add((*rawSeqStore).pos);
        if currPos >= (currSeq.litLength).wrapping_add(currSeq.matchLength) {
            currPos = currPos.wrapping_sub((currSeq.litLength).wrapping_add(currSeq.matchLength));
            (*rawSeqStore).pos = ((*rawSeqStore).pos).wrapping_add(1);
        } else {
            (*rawSeqStore).posInSequence = currPos as size_t;
            break;
        }
    }
    if currPos == 0 || (*rawSeqStore).pos == (*rawSeqStore).size {
        (*rawSeqStore).posInSequence = 0;
    }
}
pub unsafe fn ZSTD_ldm_blockCompress(
    rawSeqStore: *mut RawSeqStore_t,
    ms: &mut ZSTD_MatchState_t,
    seqStore: &mut SeqStore_t,
    rep: *mut u32,
    useRowMatchFinder: ZSTD_ParamSwitch_e,
    src: *const core::ffi::c_void,
    srcSize: size_t,
) -> size_t {
    let cParams: *const ZSTD_compressionParameters = &mut ms.cParams;
    let minMatch = (*cParams).minMatch;
    let blockCompressor = ZSTD_selectBlockCompressor(
        (*cParams).strategy,
        useRowMatchFinder,
        ZSTD_matchState_dictMode(ms),
    );
    let istart = src as *const u8;
    let iend = istart.add(srcSize);
    let mut ip = istart;
    if (*cParams).strategy as core::ffi::c_uint
        >= ZSTD_btopt as core::ffi::c_int as core::ffi::c_uint
    {
        let mut lastLLSize: size_t = 0;
        ms.ldmSeqStore = rawSeqStore;
        lastLLSize = blockCompressor.unwrap_unchecked()(ms, seqStore, rep, src, srcSize);
        ZSTD_ldm_skipRawSeqStoreBytes(rawSeqStore, srcSize);
        return lastLLSize;
    }
    while (*rawSeqStore).pos < (*rawSeqStore).size && ip < iend {
        let sequence = maybeSplitSequence(
            rawSeqStore,
            iend.offset_from(ip) as core::ffi::c_long as u32,
            minMatch,
        );
        if sequence.offset == 0 {
            break;
        }
        ZSTD_ldm_limitTableUpdate(ms, ip);
        ZSTD_ldm_fillFastTables(ms, ip as *const core::ffi::c_void);
        let mut i: core::ffi::c_int = 0;
        let newLitLength = blockCompressor.unwrap_unchecked()(
            ms,
            seqStore,
            rep,
            ip as *const core::ffi::c_void,
            sequence.litLength as size_t,
        );
        ip = ip.offset(sequence.litLength as isize);
        i = ZSTD_REP_NUM - 1;
        while i > 0 {
            *rep.offset(i as isize) = *rep.offset((i - 1) as isize);
            i -= 1;
        }
        *rep = sequence.offset;
        ZSTD_storeSeq(
            seqStore,
            newLitLength,
            ip.offset(-(newLitLength as isize)),
            iend,
            (sequence.offset).wrapping_add(ZSTD_REP_NUM as u32),
            sequence.matchLength as size_t,
        );
        ip = ip.offset(sequence.matchLength as isize);
    }
    ZSTD_ldm_limitTableUpdate(ms, ip);
    ZSTD_ldm_fillFastTables(ms, ip as *const core::ffi::c_void);
    blockCompressor.unwrap_unchecked()(
        ms,
        seqStore,
        rep,
        ip as *const core::ffi::c_void,
        iend.offset_from_unsigned(ip),
    )
}
