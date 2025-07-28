extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type ZSTD_CCtx_s;
    pub type ZSTD_CDict_s;
    pub type POOL_ctx_s;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> std::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn qsort_r(
        __base: *mut std::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_d_fn_t,
        __arg: *mut std::ffi::c_void,
    );
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memcmp(
        _: *const std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn clock() -> clock_t;
    fn ZSTD_compressBound(srcSize: size_t) -> size_t;
    fn ZSTD_createCCtx() -> *mut ZSTD_CCtx;
    fn ZSTD_freeCCtx(cctx: *mut ZSTD_CCtx) -> size_t;
    fn ZSTD_createCDict(
        dictBuffer: *const std::ffi::c_void,
        dictSize: size_t,
        compressionLevel: std::ffi::c_int,
    ) -> *mut ZSTD_CDict;
    fn ZSTD_freeCDict(CDict: *mut ZSTD_CDict) -> size_t;
    fn ZSTD_compress_usingCDict(
        cctx: *mut ZSTD_CCtx,
        dst: *mut std::ffi::c_void,
        dstCapacity: size_t,
        src: *const std::ffi::c_void,
        srcSize: size_t,
        cdict: *const ZSTD_CDict,
    ) -> size_t;
    fn POOL_create(numThreads: size_t, queueSize: size_t) -> *mut POOL_ctx;
    fn POOL_free(ctx: *mut POOL_ctx);
    fn POOL_add(ctx: *mut POOL_ctx, function: POOL_function, opaque: *mut std::ffi::c_void);
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> std::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> std::ffi::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> std::ffi::c_int;
    fn ZDICT_finalizeDictionary(
        dstDictBuffer: *mut std::ffi::c_void,
        maxDictSize: size_t,
        dictContent: *const std::ffi::c_void,
        dictContentSize: size_t,
        samplesBuffer: *const std::ffi::c_void,
        samplesSizes: *const size_t,
        nbSamples: std::ffi::c_uint,
        parameters: ZDICT_params_t,
    ) -> size_t;
    fn ZDICT_isError(errorCode: size_t) -> std::ffi::c_uint;
}
pub type size_t = std::ffi::c_ulong;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __clock_t = std::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    pub _flags2: std::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: std::ffi::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: std::ffi::c_uint,
    pub __high: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: std::ffi::c_int,
    pub __count: std::ffi::c_uint,
    pub __owner: std::ffi::c_int,
    pub __nusers: std::ffi::c_uint,
    pub __kind: std::ffi::c_int,
    pub __spins: std::ffi::c_short,
    pub __elision: std::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [std::ffi::c_uint; 2],
    pub __g_size: [std::ffi::c_uint; 2],
    pub __g1_orig_size: std::ffi::c_uint,
    pub __wrefs: std::ffi::c_uint,
    pub __g_signals: [std::ffi::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [std::ffi::c_char; 4],
    pub __align: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [std::ffi::c_char; 4],
    pub __align: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [std::ffi::c_char; 40],
    pub __align: std::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [std::ffi::c_char; 48],
    pub __align: std::ffi::c_longlong,
}
pub type __compar_d_fn_t = Option<
    unsafe extern "C" fn(
        *const std::ffi::c_void,
        *const std::ffi::c_void,
        *mut std::ffi::c_void,
    ) -> std::ffi::c_int,
>;
pub type unalign64 = u64;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const ZSTD_error_maxCode: C2RustUnnamed_0 = 120;
pub const ZSTD_error_externalSequences_invalid: C2RustUnnamed_0 = 107;
pub const ZSTD_error_sequenceProducer_failed: C2RustUnnamed_0 = 106;
pub const ZSTD_error_srcBuffer_wrong: C2RustUnnamed_0 = 105;
pub const ZSTD_error_dstBuffer_wrong: C2RustUnnamed_0 = 104;
pub const ZSTD_error_seekableIO: C2RustUnnamed_0 = 102;
pub const ZSTD_error_frameIndex_tooLarge: C2RustUnnamed_0 = 100;
pub const ZSTD_error_noForwardProgress_inputEmpty: C2RustUnnamed_0 = 82;
pub const ZSTD_error_noForwardProgress_destFull: C2RustUnnamed_0 = 80;
pub const ZSTD_error_dstBuffer_null: C2RustUnnamed_0 = 74;
pub const ZSTD_error_srcSize_wrong: C2RustUnnamed_0 = 72;
pub const ZSTD_error_dstSize_tooSmall: C2RustUnnamed_0 = 70;
pub const ZSTD_error_workSpace_tooSmall: C2RustUnnamed_0 = 66;
pub const ZSTD_error_memory_allocation: C2RustUnnamed_0 = 64;
pub const ZSTD_error_init_missing: C2RustUnnamed_0 = 62;
pub const ZSTD_error_stage_wrong: C2RustUnnamed_0 = 60;
pub const ZSTD_error_stabilityCondition_notRespected: C2RustUnnamed_0 = 50;
pub const ZSTD_error_cannotProduce_uncompressedBlock: C2RustUnnamed_0 = 49;
pub const ZSTD_error_maxSymbolValue_tooSmall: C2RustUnnamed_0 = 48;
pub const ZSTD_error_maxSymbolValue_tooLarge: C2RustUnnamed_0 = 46;
pub const ZSTD_error_tableLog_tooLarge: C2RustUnnamed_0 = 44;
pub const ZSTD_error_parameter_outOfBound: C2RustUnnamed_0 = 42;
pub const ZSTD_error_parameter_combination_unsupported: C2RustUnnamed_0 = 41;
pub const ZSTD_error_parameter_unsupported: C2RustUnnamed_0 = 40;
pub const ZSTD_error_dictionaryCreation_failed: C2RustUnnamed_0 = 34;
pub const ZSTD_error_dictionary_wrong: C2RustUnnamed_0 = 32;
pub const ZSTD_error_dictionary_corrupted: C2RustUnnamed_0 = 30;
pub const ZSTD_error_literals_headerWrong: C2RustUnnamed_0 = 24;
pub const ZSTD_error_checksum_wrong: C2RustUnnamed_0 = 22;
pub const ZSTD_error_corruption_detected: C2RustUnnamed_0 = 20;
pub const ZSTD_error_frameParameter_windowTooLarge: C2RustUnnamed_0 = 16;
pub const ZSTD_error_frameParameter_unsupported: C2RustUnnamed_0 = 14;
pub const ZSTD_error_version_unsupported: C2RustUnnamed_0 = 12;
pub const ZSTD_error_prefix_unknown: C2RustUnnamed_0 = 10;
pub const ZSTD_error_GENERIC: C2RustUnnamed_0 = 1;
pub const ZSTD_error_no_error: C2RustUnnamed_0 = 0;
pub type ZSTD_CCtx = ZSTD_CCtx_s;
pub type ZSTD_CDict = ZSTD_CDict_s;
pub type POOL_ctx = POOL_ctx_s;
pub type POOL_function = Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_params_t {
    pub compressionLevel: std::ffi::c_int,
    pub notificationLevel: std::ffi::c_uint,
    pub dictID: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZDICT_cover_params_t {
    pub k: std::ffi::c_uint,
    pub d: std::ffi::c_uint,
    pub steps: std::ffi::c_uint,
    pub nbThreads: std::ffi::c_uint,
    pub splitPoint: std::ffi::c_double,
    pub shrinkDict: std::ffi::c_uint,
    pub shrinkDictMaxRegression: std::ffi::c_uint,
    pub zParams: ZDICT_params_t,
}
pub type COVER_map_t = COVER_map_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_map_s {
    pub data: *mut COVER_map_pair_t,
    pub sizeLog: u32,
    pub size: u32,
    pub sizeMask: u32,
}
pub type COVER_map_pair_t = COVER_map_pair_t_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_map_pair_t_s {
    pub key: u32,
    pub value: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_ctx_t {
    pub samples: *const u8,
    pub offsets: *mut size_t,
    pub samplesSizes: *const size_t,
    pub nbSamples: size_t,
    pub nbTrainSamples: size_t,
    pub nbTestSamples: size_t,
    pub suffix: *mut u32,
    pub suffixSize: size_t,
    pub freqs: *mut u32,
    pub dmerAt: *mut u32,
    pub d: std::ffi::c_uint,
    pub displayLevel: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_segment_t {
    pub begin: u32,
    pub end: u32,
    pub score: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_epoch_info_t {
    pub num: u32,
    pub size: u32,
}
pub type COVER_best_t = COVER_best_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_best_s {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub liveJobs: size_t,
    pub dict: *mut std::ffi::c_void,
    pub dictSize: size_t,
    pub parameters: ZDICT_cover_params_t,
    pub compressedSize: size_t,
}
pub type COVER_tryParameters_data_t = COVER_tryParameters_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_tryParameters_data_s {
    pub ctx: *const COVER_ctx_t,
    pub best: *mut COVER_best_t,
    pub dictBufferCapacity: size_t,
    pub parameters: ZDICT_cover_params_t,
}
pub type COVER_dictSelection_t = COVER_dictSelection;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct COVER_dictSelection {
    pub dictContent: *mut u8,
    pub dictSize: size_t,
    pub totalCompressedSize: size_t,
}
pub const CLOCKS_PER_SEC: std::ffi::c_int = 1000000 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn MEM_isLittleEndian() -> std::ffi::c_uint {
    1 as std::ffi::c_int as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_read64(mut ptr: *const std::ffi::c_void) -> u64 {
    *(ptr as *const unalign64)
}
#[inline]
unsafe extern "C" fn MEM_swap64(mut in_0: u64) -> u64 {
    in_0.swap_bytes()
}
#[inline]
unsafe extern "C" fn MEM_readLE64(mut memPtr: *const std::ffi::c_void) -> u64 {
    if MEM_isLittleEndian() != 0 {
        MEM_read64(memPtr)
    } else {
        MEM_swap64(MEM_read64(memPtr))
    }
}
unsafe extern "C" fn ERR_isError(mut code: size_t) -> std::ffi::c_uint {
    (code > -(ZSTD_error_maxCode as std::ffi::c_int) as size_t) as std::ffi::c_int
        as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_countLeadingZeros32(mut val: u32) -> std::ffi::c_uint {
    val.leading_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
pub const ZSTD_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
pub const ZDICT_DICTSIZE_MIN: std::ffi::c_int = 256 as std::ffi::c_int;
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const COVER_DEFAULT_SPLITPOINT: std::ffi::c_double = 1.0f64;
pub const MAP_EMPTY_VALUE: std::ffi::c_int = -(1 as std::ffi::c_int);
unsafe extern "C" fn COVER_map_clear(mut map: *mut COVER_map_t) {
    memset(
        (*map).data as *mut std::ffi::c_void,
        MAP_EMPTY_VALUE,
        ((*map).size as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<COVER_map_pair_t>() as std::ffi::c_ulong),
    );
}
unsafe extern "C" fn COVER_map_init(mut map: *mut COVER_map_t, mut size: u32) -> std::ffi::c_int {
    (*map).sizeLog = (ZSTD_highbit32(size)).wrapping_add(2 as std::ffi::c_int as std::ffi::c_uint);
    (*map).size = (1 as std::ffi::c_int as u32) << (*map).sizeLog;
    (*map).sizeMask = ((*map).size).wrapping_sub(1 as std::ffi::c_int as u32);
    (*map).data = malloc(
        ((*map).size as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<COVER_map_pair_t>() as std::ffi::c_ulong),
    ) as *mut COVER_map_pair_t;
    if ((*map).data).is_null() {
        (*map).sizeLog = 0 as std::ffi::c_int as u32;
        (*map).size = 0 as std::ffi::c_int as u32;
        return 0 as std::ffi::c_int;
    }
    COVER_map_clear(map);
    1 as std::ffi::c_int
}
static mut COVER_prime4bytes: u32 = 2654435761 as std::ffi::c_uint;
unsafe extern "C" fn COVER_map_hash(mut map: *mut COVER_map_t, mut key: u32) -> u32 {
    (key * COVER_prime4bytes) >> (32 as std::ffi::c_int as u32).wrapping_sub((*map).sizeLog)
}
unsafe extern "C" fn COVER_map_index(mut map: *mut COVER_map_t, mut key: u32) -> u32 {
    let hash = COVER_map_hash(map, key);
    let mut i: u32 = 0;
    i = hash;
    loop {
        let mut pos: *mut COVER_map_pair_t =
            &mut *((*map).data).offset(i as isize) as *mut COVER_map_pair_t;
        if (*pos).value == MAP_EMPTY_VALUE as u32 {
            return i;
        }
        if (*pos).key == key {
            return i;
        }
        i = i.wrapping_add(1 as std::ffi::c_int as u32) & (*map).sizeMask;
    }
}
unsafe extern "C" fn COVER_map_at(mut map: *mut COVER_map_t, mut key: u32) -> *mut u32 {
    let mut pos: *mut COVER_map_pair_t = &mut *((*map).data).offset((COVER_map_index
        as unsafe extern "C" fn(*mut COVER_map_t, u32) -> u32)(
        map, key
    ) as isize) as *mut COVER_map_pair_t;
    if (*pos).value == MAP_EMPTY_VALUE as u32 {
        (*pos).key = key;
        (*pos).value = 0 as std::ffi::c_int as u32;
    }
    &mut (*pos).value
}
unsafe extern "C" fn COVER_map_remove(mut map: *mut COVER_map_t, mut key: u32) {
    let mut i = COVER_map_index(map, key);
    let mut del: *mut COVER_map_pair_t =
        &mut *((*map).data).offset(i as isize) as *mut COVER_map_pair_t;
    let mut shift = 1 as std::ffi::c_int as u32;
    if (*del).value == MAP_EMPTY_VALUE as u32 {
        return;
    }
    i = i.wrapping_add(1 as std::ffi::c_int as u32) & (*map).sizeMask;
    loop {
        let pos: *mut COVER_map_pair_t =
            &mut *((*map).data).offset(i as isize) as *mut COVER_map_pair_t;
        if (*pos).value == MAP_EMPTY_VALUE as u32 {
            (*del).value = MAP_EMPTY_VALUE as u32;
            return;
        }
        if i.wrapping_sub(COVER_map_hash(map, (*pos).key)) & (*map).sizeMask >= shift {
            (*del).key = (*pos).key;
            (*del).value = (*pos).value;
            del = pos;
            shift = 1 as std::ffi::c_int as u32;
        } else {
            shift = shift.wrapping_add(1);
            shift;
        }
        i = i.wrapping_add(1 as std::ffi::c_int as u32) & (*map).sizeMask;
    }
}
unsafe extern "C" fn COVER_map_destroy(mut map: *mut COVER_map_t) {
    if !((*map).data).is_null() {
        free((*map).data as *mut std::ffi::c_void);
    }
    (*map).data = NULL as *mut COVER_map_pair_t;
    (*map).size = 0 as std::ffi::c_int as u32;
}
#[no_mangle]
pub unsafe extern "C" fn COVER_sum(
    mut samplesSizes: *const size_t,
    mut nbSamples: std::ffi::c_uint,
) -> size_t {
    let mut sum = 0 as std::ffi::c_int as size_t;
    let mut i: std::ffi::c_uint = 0;
    i = 0 as std::ffi::c_int as std::ffi::c_uint;
    while i < nbSamples {
        sum = sum.wrapping_add(*samplesSizes.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    sum
}
unsafe extern "C" fn COVER_cmp(
    mut ctx: *mut COVER_ctx_t,
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
) -> std::ffi::c_int {
    let lhs = *(lp as *const u32);
    let rhs = *(rp as *const u32);
    memcmp(
        ((*ctx).samples).offset(lhs as isize) as *const std::ffi::c_void,
        ((*ctx).samples).offset(rhs as isize) as *const std::ffi::c_void,
        (*ctx).d as std::ffi::c_ulong,
    )
}
unsafe extern "C" fn COVER_cmp8(
    mut ctx: *mut COVER_ctx_t,
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
) -> std::ffi::c_int {
    let mask = if (*ctx).d == 8 as std::ffi::c_int as std::ffi::c_uint {
        -(1 as std::ffi::c_int) as u64
    } else {
        ((1 as std::ffi::c_int as u64)
            << (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_mul((*ctx).d))
        .wrapping_sub(1 as std::ffi::c_int as u64)
    };
    let lhs = MEM_readLE64(
        ((*ctx).samples).offset(*(lp as *const u32) as isize) as *const std::ffi::c_void
    ) & mask;
    let rhs = MEM_readLE64(
        ((*ctx).samples).offset(*(rp as *const u32) as isize) as *const std::ffi::c_void
    ) & mask;
    if lhs < rhs {
        return -(1 as std::ffi::c_int);
    }
    (lhs > rhs) as std::ffi::c_int
}
unsafe extern "C" fn COVER_strict_cmp(
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
    mut g_coverCtx: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    let mut result = COVER_cmp(g_coverCtx as *mut COVER_ctx_t, lp, rp);
    if result == 0 as std::ffi::c_int {
        result = if lp < rp {
            -(1 as std::ffi::c_int)
        } else {
            1 as std::ffi::c_int
        };
    }
    result
}
unsafe extern "C" fn COVER_strict_cmp8(
    mut lp: *const std::ffi::c_void,
    mut rp: *const std::ffi::c_void,
    mut g_coverCtx: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    let mut result = COVER_cmp8(g_coverCtx as *mut COVER_ctx_t, lp, rp);
    if result == 0 as std::ffi::c_int {
        result = if lp < rp {
            -(1 as std::ffi::c_int)
        } else {
            1 as std::ffi::c_int
        };
    }
    result
}
unsafe extern "C" fn stableSort(mut ctx: *mut COVER_ctx_t) {
    qsort_r(
        (*ctx).suffix as *mut std::ffi::c_void,
        (*ctx).suffixSize,
        ::core::mem::size_of::<u32>() as std::ffi::c_ulong,
        if (*ctx).d <= 8 as std::ffi::c_int as std::ffi::c_uint {
            Some(
                COVER_strict_cmp8
                    as unsafe extern "C" fn(
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                        *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        } else {
            Some(
                COVER_strict_cmp
                    as unsafe extern "C" fn(
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                        *mut std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        },
        ctx as *mut std::ffi::c_void,
    );
}
unsafe extern "C" fn COVER_lower_bound(
    mut first: *const size_t,
    mut last: *const size_t,
    mut value: size_t,
) -> *const size_t {
    let mut count = last.offset_from(first) as std::ffi::c_long as size_t;
    while count != 0 as std::ffi::c_int as size_t {
        let mut step = count / 2 as std::ffi::c_int as size_t;
        let mut ptr = first;
        ptr = ptr.offset(step as isize);
        if *ptr < value {
            ptr = ptr.offset(1);
            first = ptr;
            count = count.wrapping_sub(step.wrapping_add(1 as std::ffi::c_int as size_t));
        } else {
            count = step;
        }
    }
    first
}
unsafe extern "C" fn COVER_groupBy(
    mut data: *const std::ffi::c_void,
    mut count: size_t,
    mut size: size_t,
    mut ctx: *mut COVER_ctx_t,
    mut cmp: Option<
        unsafe extern "C" fn(
            *mut COVER_ctx_t,
            *const std::ffi::c_void,
            *const std::ffi::c_void,
        ) -> std::ffi::c_int,
    >,
    mut grp: Option<
        unsafe extern "C" fn(
            *mut COVER_ctx_t,
            *const std::ffi::c_void,
            *const std::ffi::c_void,
        ) -> (),
    >,
) {
    let mut ptr = data as *const u8;
    let mut num = 0 as std::ffi::c_int as size_t;
    while num < count {
        let mut grpEnd = ptr.offset(size as isize);
        num = num.wrapping_add(1);
        num;
        while num < count
            && cmp.unwrap_unchecked()(
                ctx,
                ptr as *const std::ffi::c_void,
                grpEnd as *const std::ffi::c_void,
            ) == 0 as std::ffi::c_int
        {
            grpEnd = grpEnd.offset(size as isize);
            num = num.wrapping_add(1);
            num;
        }
        grp.unwrap_unchecked()(
            ctx,
            ptr as *const std::ffi::c_void,
            grpEnd as *const std::ffi::c_void,
        );
        ptr = grpEnd;
    }
}
unsafe extern "C" fn COVER_group(
    mut ctx: *mut COVER_ctx_t,
    mut group: *const std::ffi::c_void,
    mut groupEnd: *const std::ffi::c_void,
) {
    let mut grpPtr = group as *const u32;
    let mut grpEnd = groupEnd as *const u32;
    let dmerId = grpPtr.offset_from((*ctx).suffix) as std::ffi::c_long as u32;
    let mut freq = 0 as std::ffi::c_int as u32;
    let mut curOffsetPtr: *const size_t = (*ctx).offsets;
    let mut offsetsEnd: *const size_t = ((*ctx).offsets).offset((*ctx).nbSamples as isize);
    let mut curSampleEnd = *((*ctx).offsets).offset(0 as std::ffi::c_int as isize);
    while grpPtr != grpEnd {
        *((*ctx).dmerAt).offset(*grpPtr as isize) = dmerId;
        if (*grpPtr as size_t) >= curSampleEnd {
            freq = freq.wrapping_add(1 as std::ffi::c_int as u32);
            if grpPtr.offset(1 as std::ffi::c_int as isize) != grpEnd {
                let mut sampleEndPtr =
                    COVER_lower_bound(curOffsetPtr, offsetsEnd, *grpPtr as size_t);
                curSampleEnd = *sampleEndPtr;
                curOffsetPtr = sampleEndPtr.offset(1 as std::ffi::c_int as isize);
            }
        }
        grpPtr = grpPtr.offset(1);
        grpPtr;
    }
    *((*ctx).suffix).offset(dmerId as isize) = freq;
}
unsafe extern "C" fn COVER_selectSegment(
    mut ctx: *const COVER_ctx_t,
    mut freqs: *mut u32,
    mut activeDmers: *mut COVER_map_t,
    mut begin: u32,
    mut end: u32,
    mut parameters: ZDICT_cover_params_t,
) -> COVER_segment_t {
    let k = parameters.k;
    let d = parameters.d;
    let dmersInK = k.wrapping_sub(d).wrapping_add(1 as std::ffi::c_int as u32);
    let mut bestSegment = {
        COVER_segment_t {
            begin: 0 as std::ffi::c_int as u32,
            end: 0 as std::ffi::c_int as u32,
            score: 0 as std::ffi::c_int as u32,
        }
    };
    let mut activeSegment = COVER_segment_t {
        begin: 0,
        end: 0,
        score: 0,
    };
    COVER_map_clear(activeDmers);
    activeSegment.begin = begin;
    activeSegment.end = begin;
    activeSegment.score = 0 as std::ffi::c_int as u32;
    while activeSegment.end < end {
        let mut newDmer = *((*ctx).dmerAt).offset(activeSegment.end as isize);
        let mut newDmerOcc = COVER_map_at(activeDmers, newDmer);
        if *newDmerOcc == 0 as std::ffi::c_int as u32 {
            activeSegment.score =
                (activeSegment.score).wrapping_add(*freqs.offset(newDmer as isize));
        }
        activeSegment.end = (activeSegment.end).wrapping_add(1 as std::ffi::c_int as u32);
        *newDmerOcc = (*newDmerOcc).wrapping_add(1 as std::ffi::c_int as u32);
        if (activeSegment.end).wrapping_sub(activeSegment.begin)
            == dmersInK.wrapping_add(1 as std::ffi::c_int as u32)
        {
            let mut delDmer = *((*ctx).dmerAt).offset(activeSegment.begin as isize);
            let mut delDmerOcc = COVER_map_at(activeDmers, delDmer);
            activeSegment.begin = (activeSegment.begin).wrapping_add(1 as std::ffi::c_int as u32);
            *delDmerOcc = (*delDmerOcc).wrapping_sub(1 as std::ffi::c_int as u32);
            if *delDmerOcc == 0 as std::ffi::c_int as u32 {
                COVER_map_remove(activeDmers, delDmer);
                activeSegment.score =
                    (activeSegment.score).wrapping_sub(*freqs.offset(delDmer as isize));
            }
        }
        if activeSegment.score > bestSegment.score {
            bestSegment = activeSegment;
        }
    }
    let mut newBegin = bestSegment.end;
    let mut newEnd = bestSegment.begin;
    let mut pos: u32 = 0;
    pos = bestSegment.begin;
    while pos != bestSegment.end {
        let mut freq = *freqs.offset(*((*ctx).dmerAt).offset(pos as isize) as isize);
        if freq != 0 as std::ffi::c_int as u32 {
            newBegin = if newBegin < pos { newBegin } else { pos };
            newEnd = pos.wrapping_add(1 as std::ffi::c_int as u32);
        }
        pos = pos.wrapping_add(1);
        pos;
    }
    bestSegment.begin = newBegin;
    bestSegment.end = newEnd;
    let mut pos_0: u32 = 0;
    pos_0 = bestSegment.begin;
    while pos_0 != bestSegment.end {
        *freqs.offset(*((*ctx).dmerAt).offset(pos_0 as isize) as isize) =
            0 as std::ffi::c_int as u32;
        pos_0 = pos_0.wrapping_add(1);
        pos_0;
    }
    bestSegment
}
unsafe extern "C" fn COVER_checkParameters(
    mut parameters: ZDICT_cover_params_t,
    mut maxDictSize: size_t,
) -> std::ffi::c_int {
    if parameters.d == 0 as std::ffi::c_int as std::ffi::c_uint
        || parameters.k == 0 as std::ffi::c_int as std::ffi::c_uint
    {
        return 0 as std::ffi::c_int;
    }
    if parameters.k as size_t > maxDictSize {
        return 0 as std::ffi::c_int;
    }
    if parameters.d > parameters.k {
        return 0 as std::ffi::c_int;
    }
    if parameters.splitPoint <= 0 as std::ffi::c_int as std::ffi::c_double
        || parameters.splitPoint > 1 as std::ffi::c_int as std::ffi::c_double
    {
        return 0 as std::ffi::c_int;
    }
    1 as std::ffi::c_int
}
unsafe extern "C" fn COVER_ctx_destroy(mut ctx: *mut COVER_ctx_t) {
    if ctx.is_null() {
        return;
    }
    if !((*ctx).suffix).is_null() {
        free((*ctx).suffix as *mut std::ffi::c_void);
        (*ctx).suffix = NULL as *mut u32;
    }
    if !((*ctx).freqs).is_null() {
        free((*ctx).freqs as *mut std::ffi::c_void);
        (*ctx).freqs = NULL as *mut u32;
    }
    if !((*ctx).dmerAt).is_null() {
        free((*ctx).dmerAt as *mut std::ffi::c_void);
        (*ctx).dmerAt = NULL as *mut u32;
    }
    if !((*ctx).offsets).is_null() {
        free((*ctx).offsets as *mut std::ffi::c_void);
        (*ctx).offsets = NULL as *mut size_t;
    }
}
unsafe extern "C" fn COVER_ctx_init(
    mut ctx: *mut COVER_ctx_t,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: std::ffi::c_uint,
    mut d: std::ffi::c_uint,
    mut splitPoint: std::ffi::c_double,
    mut displayLevel: std::ffi::c_int,
) -> size_t {
    let samples = samplesBuffer as *const u8;
    let totalSamplesSize = COVER_sum(samplesSizes, nbSamples);
    let nbTrainSamples = if splitPoint < 1.0f64 {
        (nbSamples as std::ffi::c_double * splitPoint) as std::ffi::c_uint
    } else {
        nbSamples
    };
    let nbTestSamples = if splitPoint < 1.0f64 {
        nbSamples.wrapping_sub(nbTrainSamples)
    } else {
        nbSamples
    };
    let trainingSamplesSize = if splitPoint < 1.0f64 {
        COVER_sum(samplesSizes, nbTrainSamples)
    } else {
        totalSamplesSize
    };
    let testSamplesSize = if splitPoint < 1.0f64 {
        COVER_sum(samplesSizes.offset(nbTrainSamples as isize), nbTestSamples)
    } else {
        totalSamplesSize
    };
    (*ctx).displayLevel = displayLevel;
    if totalSamplesSize
        < (if d as std::ffi::c_ulong > ::core::mem::size_of::<u64>() as std::ffi::c_ulong {
            d as std::ffi::c_ulong
        } else {
            ::core::mem::size_of::<u64>() as std::ffi::c_ulong
        })
        || totalSamplesSize
            >= (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                == 8 as std::ffi::c_int as std::ffi::c_ulong
            {
                -(1 as std::ffi::c_int) as std::ffi::c_uint
            } else {
                (1 as std::ffi::c_int as std::ffi::c_uint)
                    .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int)
            }) as size_t
    {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Total samples size is too large (%u MB), maximum size is %u MB\n\0" as *const u8
                    as *const std::ffi::c_char,
                (totalSamplesSize >> 20 as std::ffi::c_int) as std::ffi::c_uint,
                (if ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
                    == 8 as std::ffi::c_int as std::ffi::c_ulong
                {
                    -(1 as std::ffi::c_int) as std::ffi::c_uint
                } else {
                    (1 as std::ffi::c_int as std::ffi::c_uint)
                        .wrapping_mul((1 as std::ffi::c_uint) << 30 as std::ffi::c_int)
                }) >> 20 as std::ffi::c_int,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if nbTrainSamples < 5 as std::ffi::c_int as std::ffi::c_uint {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Total number of training samples is %u and is invalid.\0" as *const u8
                    as *const std::ffi::c_char,
                nbTrainSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if nbTestSamples < 1 as std::ffi::c_int as std::ffi::c_uint {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Total number of testing samples is %u and is invalid.\0" as *const u8
                    as *const std::ffi::c_char,
                nbTestSamples,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    memset(
        ctx as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<COVER_ctx_t>() as std::ffi::c_ulong,
    );
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Training on %u samples of total size %u\n\0" as *const u8 as *const std::ffi::c_char,
            nbTrainSamples,
            trainingSamplesSize as std::ffi::c_uint,
        );
        fflush(stderr);
    }
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Testing on %u samples of total size %u\n\0" as *const u8 as *const std::ffi::c_char,
            nbTestSamples,
            testSamplesSize as std::ffi::c_uint,
        );
        fflush(stderr);
    }
    (*ctx).samples = samples;
    (*ctx).samplesSizes = samplesSizes;
    (*ctx).nbSamples = nbSamples as size_t;
    (*ctx).nbTrainSamples = nbTrainSamples as size_t;
    (*ctx).nbTestSamples = nbTestSamples as size_t;
    (*ctx).suffixSize = trainingSamplesSize
        .wrapping_sub(
            if d as std::ffi::c_ulong > ::core::mem::size_of::<u64>() as std::ffi::c_ulong {
                d as std::ffi::c_ulong
            } else {
                ::core::mem::size_of::<u64>() as std::ffi::c_ulong
            },
        )
        .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong);
    (*ctx).suffix = malloc(
        ((*ctx).suffixSize).wrapping_mul(::core::mem::size_of::<u32>() as std::ffi::c_ulong),
    ) as *mut u32;
    (*ctx).dmerAt = malloc(
        ((*ctx).suffixSize).wrapping_mul(::core::mem::size_of::<u32>() as std::ffi::c_ulong),
    ) as *mut u32;
    (*ctx).offsets = malloc(
        (nbSamples.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as std::ffi::c_ulong),
    ) as *mut size_t;
    if ((*ctx).suffix).is_null() || ((*ctx).dmerAt).is_null() || ((*ctx).offsets).is_null() {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Failed to allocate scratch buffers\n\0" as *const u8 as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        COVER_ctx_destroy(ctx);
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    (*ctx).freqs = NULL as *mut u32;
    (*ctx).d = d;
    let mut i: u32 = 0;
    *((*ctx).offsets).offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as size_t;
    i = 1 as std::ffi::c_int as u32;
    while i <= nbSamples {
        *((*ctx).offsets).offset(i as isize) = (*((*ctx).offsets)
            .offset(i.wrapping_sub(1 as std::ffi::c_int as u32) as isize))
        .wrapping_add(*samplesSizes.offset(i.wrapping_sub(1 as std::ffi::c_int as u32) as isize));
        i = i.wrapping_add(1);
        i;
    }
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Constructing partial suffix array\n\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    let mut i_0: u32 = 0;
    i_0 = 0 as std::ffi::c_int as u32;
    while (i_0 as size_t) < (*ctx).suffixSize {
        *((*ctx).suffix).offset(i_0 as isize) = i_0;
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    stableSort(ctx);
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Computing frequencies\n\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    COVER_groupBy(
        (*ctx).suffix as *const std::ffi::c_void,
        (*ctx).suffixSize,
        ::core::mem::size_of::<u32>() as std::ffi::c_ulong,
        ctx,
        if (*ctx).d <= 8 as std::ffi::c_int as std::ffi::c_uint {
            Some(
                COVER_cmp8
                    as unsafe extern "C" fn(
                        *mut COVER_ctx_t,
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        } else {
            Some(
                COVER_cmp
                    as unsafe extern "C" fn(
                        *mut COVER_ctx_t,
                        *const std::ffi::c_void,
                        *const std::ffi::c_void,
                    ) -> std::ffi::c_int,
            )
        },
        Some(
            COVER_group
                as unsafe extern "C" fn(
                    *mut COVER_ctx_t,
                    *const std::ffi::c_void,
                    *const std::ffi::c_void,
                ) -> (),
        ),
    );
    (*ctx).freqs = (*ctx).suffix;
    (*ctx).suffix = NULL as *mut u32;
    0 as std::ffi::c_int as size_t
}
#[no_mangle]
pub unsafe extern "C" fn COVER_warnOnSmallCorpus(
    mut maxDictSize: size_t,
    mut nbDmers: size_t,
    mut displayLevel: std::ffi::c_int,
) {
    let ratio = nbDmers as std::ffi::c_double / maxDictSize as std::ffi::c_double;
    if ratio >= 10 as std::ffi::c_int as std::ffi::c_double {
        return;
    }
    if displayLevel >= 1 as std::ffi::c_int {
        fprintf(
            stderr,
            b"WARNING: The maximum dictionary size %u is too large compared to the source size %u! size(source)/size(dictionary) = %f, but it should be >= 10! This may lead to a subpar dictionary! We recommend training on sources at least 10x, and preferably 100x the size of the dictionary! \n\0"
                as *const u8 as *const std::ffi::c_char,
            maxDictSize as u32,
            nbDmers as u32,
            ratio,
        );
        fflush(stderr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn COVER_computeEpochs(
    mut maxDictSize: u32,
    mut nbDmers: u32,
    mut k: u32,
    mut passes: u32,
) -> COVER_epoch_info_t {
    let minEpochSize = k * 10 as std::ffi::c_int as u32;
    let mut epochs = COVER_epoch_info_t { num: 0, size: 0 };
    epochs.num = if 1 as std::ffi::c_int as u32 > maxDictSize / k / passes {
        1 as std::ffi::c_int as u32
    } else {
        maxDictSize / k / passes
    };
    epochs.size = nbDmers / epochs.num;
    if epochs.size >= minEpochSize {
        return epochs;
    }
    epochs.size = if minEpochSize < nbDmers {
        minEpochSize
    } else {
        nbDmers
    };
    epochs.num = nbDmers / epochs.size;
    epochs
}
unsafe extern "C" fn COVER_buildDictionary(
    mut ctx: *const COVER_ctx_t,
    mut freqs: *mut u32,
    mut activeDmers: *mut COVER_map_t,
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut parameters: ZDICT_cover_params_t,
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut tail = dictBufferCapacity;
    let epochs = COVER_computeEpochs(
        dictBufferCapacity as u32,
        (*ctx).suffixSize as u32,
        parameters.k,
        4 as std::ffi::c_int as u32,
    );
    let maxZeroScoreRun = (if 10 as std::ffi::c_int as u32
        > (if (100 as std::ffi::c_int as u32) < epochs.num >> 3 as std::ffi::c_int {
            100 as std::ffi::c_int as u32
        } else {
            epochs.num >> 3 as std::ffi::c_int
        }) {
        10 as std::ffi::c_int as u32
    } else if (100 as std::ffi::c_int as u32) < epochs.num >> 3 as std::ffi::c_int {
        100 as std::ffi::c_int as u32
    } else {
        epochs.num >> 3 as std::ffi::c_int
    }) as size_t;
    let mut zeroScoreRun = 0 as std::ffi::c_int as size_t;
    let mut epoch: size_t = 0;
    let mut lastUpdateTime = 0 as std::ffi::c_int as clock_t;
    let displayLevel = (*ctx).displayLevel;
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Breaking content into %u epochs of size %u\n\0" as *const u8
                as *const std::ffi::c_char,
            epochs.num,
            epochs.size,
        );
        fflush(stderr);
    }
    epoch = 0 as std::ffi::c_int as size_t;
    while tail > 0 as std::ffi::c_int as size_t {
        let epochBegin = (epoch * epochs.size as size_t) as u32;
        let epochEnd = epochBegin.wrapping_add(epochs.size);
        let mut segmentSize: size_t = 0;
        let mut segment =
            COVER_selectSegment(ctx, freqs, activeDmers, epochBegin, epochEnd, parameters);
        if segment.score == 0 as std::ffi::c_int as u32 {
            zeroScoreRun = zeroScoreRun.wrapping_add(1);
            if zeroScoreRun >= maxZeroScoreRun {
                break;
            }
        } else {
            zeroScoreRun = 0 as std::ffi::c_int as size_t;
            segmentSize = if ((segment.end)
                .wrapping_sub(segment.begin)
                .wrapping_add(parameters.d)
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                as size_t)
                < tail
            {
                (segment.end)
                    .wrapping_sub(segment.begin)
                    .wrapping_add(parameters.d)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                    as size_t
            } else {
                tail
            };
            if segmentSize < parameters.d as size_t {
                break;
            }
            tail = tail.wrapping_sub(segmentSize);
            memcpy(
                dict.offset(tail as isize) as *mut std::ffi::c_void,
                ((*ctx).samples).offset(segment.begin as isize) as *const std::ffi::c_void,
                segmentSize,
            );
            if displayLevel >= 2 as std::ffi::c_int {
                let refreshRate = CLOCKS_PER_SEC as __clock_t * 15 as std::ffi::c_int as __clock_t
                    / 100 as std::ffi::c_int as __clock_t;
                if clock() - lastUpdateTime > refreshRate || displayLevel >= 4 as std::ffi::c_int {
                    lastUpdateTime = clock();
                    fprintf(
                        stderr,
                        b"\r%u%%       \0" as *const u8 as *const std::ffi::c_char,
                        (dictBufferCapacity.wrapping_sub(tail) * 100 as std::ffi::c_int as size_t
                            / dictBufferCapacity) as std::ffi::c_uint,
                    );
                    fflush(stderr);
                }
            }
        }
        epoch = epoch.wrapping_add(1 as std::ffi::c_int as size_t) % epochs.num as size_t;
    }
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    tail
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_trainFromBuffer_cover(
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: std::ffi::c_uint,
    mut parameters: ZDICT_cover_params_t,
) -> size_t {
    let dict = dictBuffer as *mut u8;
    let mut ctx = COVER_ctx_t {
        samples: std::ptr::null::<u8>(),
        offsets: std::ptr::null_mut::<size_t>(),
        samplesSizes: std::ptr::null::<size_t>(),
        nbSamples: 0,
        nbTrainSamples: 0,
        nbTestSamples: 0,
        suffix: std::ptr::null_mut::<u32>(),
        suffixSize: 0,
        freqs: std::ptr::null_mut::<u32>(),
        dmerAt: std::ptr::null_mut::<u32>(),
        d: 0,
        displayLevel: 0,
    };
    let mut activeDmers = COVER_map_s {
        data: std::ptr::null_mut::<COVER_map_pair_t>(),
        sizeLog: 0,
        size: 0,
        sizeMask: 0,
    };
    let displayLevel = parameters.zParams.notificationLevel as std::ffi::c_int;
    parameters.splitPoint = 1.0f64;
    if COVER_checkParameters(parameters, dictBufferCapacity) == 0 {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Cover parameters incorrect\n\0" as *const u8 as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
    }
    if nbSamples == 0 as std::ffi::c_int as std::ffi::c_uint {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Cover must have at least one input file\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const std::ffi::c_char,
                256 as std::ffi::c_int,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    let initVal = COVER_ctx_init(
        &mut ctx,
        samplesBuffer,
        samplesSizes,
        nbSamples,
        parameters.d,
        parameters.splitPoint,
        displayLevel,
    );
    if ERR_isError(initVal) != 0 {
        return initVal;
    }
    COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.suffixSize, displayLevel);
    if COVER_map_init(
        &mut activeDmers,
        (parameters.k)
            .wrapping_sub(parameters.d)
            .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint),
    ) == 0
    {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Failed to allocate dmer map: out of memory\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        COVER_ctx_destroy(&mut ctx);
        return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
    }
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Building dictionary\n\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    let tail = COVER_buildDictionary(
        &mut ctx,
        ctx.freqs,
        &mut activeDmers,
        dictBuffer,
        dictBufferCapacity,
        parameters,
    );
    let dictionarySize = ZDICT_finalizeDictionary(
        dict as *mut std::ffi::c_void,
        dictBufferCapacity,
        dict.offset(tail as isize) as *const std::ffi::c_void,
        dictBufferCapacity.wrapping_sub(tail),
        samplesBuffer,
        samplesSizes,
        nbSamples,
        parameters.zParams,
    );
    if ERR_isError(dictionarySize) == 0 && displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Constructed dictionary of size %u\n\0" as *const u8 as *const std::ffi::c_char,
            dictionarySize as std::ffi::c_uint,
        );
        fflush(stderr);
    }
    COVER_ctx_destroy(&mut ctx);
    COVER_map_destroy(&mut activeDmers);
    dictionarySize
}
#[no_mangle]
pub unsafe extern "C" fn COVER_checkTotalCompressedSize(
    parameters: ZDICT_cover_params_t,
    mut samplesSizes: *const size_t,
    mut samples: *const u8,
    mut offsets: *mut size_t,
    mut nbTrainSamples: size_t,
    mut nbSamples: size_t,
    dict: *mut u8,
    mut dictBufferCapacity: size_t,
) -> size_t {
    let mut totalCompressedSize = -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    let mut cctx = std::ptr::null_mut::<ZSTD_CCtx>();
    let mut cdict = std::ptr::null_mut::<ZSTD_CDict>();
    let mut dst = std::ptr::null_mut::<std::ffi::c_void>();
    let mut dstCapacity: size_t = 0;
    let mut i: size_t = 0;
    let mut maxSampleSize = 0 as std::ffi::c_int as size_t;
    i = if parameters.splitPoint < 1.0f64 {
        nbTrainSamples
    } else {
        0 as std::ffi::c_int as size_t
    };
    while i < nbSamples {
        maxSampleSize = if *samplesSizes.offset(i as isize) > maxSampleSize {
            *samplesSizes.offset(i as isize)
        } else {
            maxSampleSize
        };
        i = i.wrapping_add(1);
        i;
    }
    dstCapacity = ZSTD_compressBound(maxSampleSize);
    dst = malloc(dstCapacity);
    cctx = ZSTD_createCCtx();
    cdict = ZSTD_createCDict(
        dict as *const std::ffi::c_void,
        dictBufferCapacity,
        parameters.zParams.compressionLevel,
    );
    if !(dst.is_null() || cctx.is_null() || cdict.is_null()) {
        totalCompressedSize = dictBufferCapacity;
        i = if parameters.splitPoint < 1.0f64 {
            nbTrainSamples
        } else {
            0 as std::ffi::c_int as size_t
        };
        while i < nbSamples {
            let size = ZSTD_compress_usingCDict(
                cctx,
                dst,
                dstCapacity,
                samples.offset(*offsets.offset(i as isize) as isize) as *const std::ffi::c_void,
                *samplesSizes.offset(i as isize),
                cdict,
            );
            if ERR_isError(size) != 0 {
                totalCompressedSize = size;
                break;
            } else {
                totalCompressedSize = totalCompressedSize.wrapping_add(size);
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    ZSTD_freeCCtx(cctx);
    ZSTD_freeCDict(cdict);
    if !dst.is_null() {
        free(dst);
    }
    totalCompressedSize
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_init(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    pthread_mutex_init(&mut (*best).mutex, std::ptr::null::<pthread_mutexattr_t>());
    pthread_cond_init(&mut (*best).cond, std::ptr::null::<pthread_condattr_t>());
    (*best).liveJobs = 0 as std::ffi::c_int as size_t;
    (*best).dict = NULL as *mut std::ffi::c_void;
    (*best).dictSize = 0 as std::ffi::c_int as size_t;
    (*best).compressedSize = -(1 as std::ffi::c_int) as size_t;
    memset(
        &mut (*best).parameters as *mut ZDICT_cover_params_t as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<ZDICT_cover_params_t>() as std::ffi::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_wait(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    pthread_mutex_lock(&mut (*best).mutex);
    while (*best).liveJobs != 0 as std::ffi::c_int as size_t {
        pthread_cond_wait(&mut (*best).cond, &mut (*best).mutex);
    }
    pthread_mutex_unlock(&mut (*best).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_destroy(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    COVER_best_wait(best);
    if !((*best).dict).is_null() {
        free((*best).dict);
    }
    pthread_mutex_destroy(&mut (*best).mutex);
    pthread_cond_destroy(&mut (*best).cond);
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_start(mut best: *mut COVER_best_t) {
    if best.is_null() {
        return;
    }
    pthread_mutex_lock(&mut (*best).mutex);
    (*best).liveJobs = ((*best).liveJobs).wrapping_add(1);
    (*best).liveJobs;
    pthread_mutex_unlock(&mut (*best).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn COVER_best_finish(
    mut best: *mut COVER_best_t,
    mut parameters: ZDICT_cover_params_t,
    mut selection: COVER_dictSelection_t,
) {
    let mut dict = selection.dictContent as *mut std::ffi::c_void;
    let mut compressedSize = selection.totalCompressedSize;
    let mut dictSize = selection.dictSize;
    if best.is_null() {
        return;
    }
    let mut liveJobs: size_t = 0;
    pthread_mutex_lock(&mut (*best).mutex);
    (*best).liveJobs = ((*best).liveJobs).wrapping_sub(1);
    (*best).liveJobs;
    liveJobs = (*best).liveJobs;
    if compressedSize < (*best).compressedSize {
        if ((*best).dict).is_null() || (*best).dictSize < dictSize {
            if !((*best).dict).is_null() {
                free((*best).dict);
            }
            (*best).dict = malloc(dictSize);
            if ((*best).dict).is_null() {
                (*best).compressedSize = -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
                (*best).dictSize = 0 as std::ffi::c_int as size_t;
                pthread_cond_signal(&mut (*best).cond);
                pthread_mutex_unlock(&mut (*best).mutex);
                return;
            }
        }
        if !dict.is_null() {
            memcpy((*best).dict, dict, dictSize);
            (*best).dictSize = dictSize;
            (*best).parameters = parameters;
            (*best).compressedSize = compressedSize;
        }
    }
    if liveJobs == 0 as std::ffi::c_int as size_t {
        pthread_cond_broadcast(&mut (*best).cond);
    }
    pthread_mutex_unlock(&mut (*best).mutex);
}
unsafe extern "C" fn setDictSelection(
    mut buf: *mut u8,
    mut s: size_t,
    mut csz: size_t,
) -> COVER_dictSelection_t {
    let mut ds = COVER_dictSelection {
        dictContent: std::ptr::null_mut::<u8>(),
        dictSize: 0,
        totalCompressedSize: 0,
    };
    ds.dictContent = buf;
    ds.dictSize = s;
    ds.totalCompressedSize = csz;
    ds
}
#[no_mangle]
pub unsafe extern "C" fn COVER_dictSelectionError(mut error: size_t) -> COVER_dictSelection_t {
    setDictSelection(NULL as *mut u8, 0 as std::ffi::c_int as size_t, error)
}
#[no_mangle]
pub unsafe extern "C" fn COVER_dictSelectionIsError(
    mut selection: COVER_dictSelection_t,
) -> std::ffi::c_uint {
    (ERR_isError(selection.totalCompressedSize) != 0 || (selection.dictContent).is_null())
        as std::ffi::c_int as std::ffi::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn COVER_dictSelectionFree(mut selection: COVER_dictSelection_t) {
    free(selection.dictContent as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn COVER_selectDict(
    mut customDictContent: *mut u8,
    mut dictBufferCapacity: size_t,
    mut dictContentSize: size_t,
    mut samplesBuffer: *const u8,
    mut samplesSizes: *const size_t,
    mut nbFinalizeSamples: std::ffi::c_uint,
    mut nbCheckSamples: size_t,
    mut nbSamples: size_t,
    mut params: ZDICT_cover_params_t,
    mut offsets: *mut size_t,
    mut totalCompressedSize: size_t,
) -> COVER_dictSelection_t {
    let mut largestDict = 0 as std::ffi::c_int as size_t;
    let mut largestCompressed = 0 as std::ffi::c_int as size_t;
    let mut customDictContentEnd = customDictContent.offset(dictContentSize as isize);
    let mut largestDictbuffer = malloc(dictBufferCapacity) as *mut u8;
    let mut candidateDictBuffer = malloc(dictBufferCapacity) as *mut u8;
    let mut regressionTolerance =
        params.shrinkDictMaxRegression as std::ffi::c_double / 100.0f64 + 1.00f64;
    if largestDictbuffer.is_null() || candidateDictBuffer.is_null() {
        free(largestDictbuffer as *mut std::ffi::c_void);
        free(candidateDictBuffer as *mut std::ffi::c_void);
        return COVER_dictSelectionError(dictContentSize);
    }
    memcpy(
        largestDictbuffer as *mut std::ffi::c_void,
        customDictContent as *const std::ffi::c_void,
        dictContentSize,
    );
    dictContentSize = ZDICT_finalizeDictionary(
        largestDictbuffer as *mut std::ffi::c_void,
        dictBufferCapacity,
        customDictContent as *const std::ffi::c_void,
        dictContentSize,
        samplesBuffer as *const std::ffi::c_void,
        samplesSizes,
        nbFinalizeSamples,
        params.zParams,
    );
    if ZDICT_isError(dictContentSize) != 0 {
        free(largestDictbuffer as *mut std::ffi::c_void);
        free(candidateDictBuffer as *mut std::ffi::c_void);
        return COVER_dictSelectionError(dictContentSize);
    }
    totalCompressedSize = COVER_checkTotalCompressedSize(
        params,
        samplesSizes,
        samplesBuffer,
        offsets,
        nbCheckSamples,
        nbSamples,
        largestDictbuffer,
        dictContentSize,
    );
    if ERR_isError(totalCompressedSize) != 0 {
        free(largestDictbuffer as *mut std::ffi::c_void);
        free(candidateDictBuffer as *mut std::ffi::c_void);
        return COVER_dictSelectionError(totalCompressedSize);
    }
    if params.shrinkDict == 0 as std::ffi::c_int as std::ffi::c_uint {
        free(candidateDictBuffer as *mut std::ffi::c_void);
        return setDictSelection(largestDictbuffer, dictContentSize, totalCompressedSize);
    }
    largestDict = dictContentSize;
    largestCompressed = totalCompressedSize;
    dictContentSize = ZDICT_DICTSIZE_MIN as size_t;
    while dictContentSize < largestDict {
        memcpy(
            candidateDictBuffer as *mut std::ffi::c_void,
            largestDictbuffer as *const std::ffi::c_void,
            largestDict,
        );
        dictContentSize = ZDICT_finalizeDictionary(
            candidateDictBuffer as *mut std::ffi::c_void,
            dictBufferCapacity,
            customDictContentEnd.offset(-(dictContentSize as isize)) as *const std::ffi::c_void,
            dictContentSize,
            samplesBuffer as *const std::ffi::c_void,
            samplesSizes,
            nbFinalizeSamples,
            params.zParams,
        );
        if ZDICT_isError(dictContentSize) != 0 {
            free(largestDictbuffer as *mut std::ffi::c_void);
            free(candidateDictBuffer as *mut std::ffi::c_void);
            return COVER_dictSelectionError(dictContentSize);
        }
        totalCompressedSize = COVER_checkTotalCompressedSize(
            params,
            samplesSizes,
            samplesBuffer,
            offsets,
            nbCheckSamples,
            nbSamples,
            candidateDictBuffer,
            dictContentSize,
        );
        if ERR_isError(totalCompressedSize) != 0 {
            free(largestDictbuffer as *mut std::ffi::c_void);
            free(candidateDictBuffer as *mut std::ffi::c_void);
            return COVER_dictSelectionError(totalCompressedSize);
        }
        if totalCompressedSize as std::ffi::c_double
            <= largestCompressed as std::ffi::c_double * regressionTolerance
        {
            free(largestDictbuffer as *mut std::ffi::c_void);
            return setDictSelection(candidateDictBuffer, dictContentSize, totalCompressedSize);
        }
        dictContentSize *= 2 as std::ffi::c_int as size_t;
    }
    dictContentSize = largestDict;
    totalCompressedSize = largestCompressed;
    free(candidateDictBuffer as *mut std::ffi::c_void);
    setDictSelection(largestDictbuffer, dictContentSize, totalCompressedSize)
}
unsafe extern "C" fn COVER_tryParameters(mut opaque: *mut std::ffi::c_void) {
    let data = opaque as *mut COVER_tryParameters_data_t;
    let ctx = (*data).ctx;
    let parameters = (*data).parameters;
    let mut dictBufferCapacity = (*data).dictBufferCapacity;
    let mut totalCompressedSize = -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    let mut activeDmers = COVER_map_s {
        data: std::ptr::null_mut::<COVER_map_pair_t>(),
        sizeLog: 0,
        size: 0,
        sizeMask: 0,
    };
    let dict = malloc(dictBufferCapacity) as *mut u8;
    let mut selection =
        COVER_dictSelectionError(-(ZSTD_error_GENERIC as std::ffi::c_int) as size_t);
    let freqs = malloc(
        ((*ctx).suffixSize).wrapping_mul(::core::mem::size_of::<u32>() as std::ffi::c_ulong),
    ) as *mut u32;
    let displayLevel = (*ctx).displayLevel;
    if COVER_map_init(
        &mut activeDmers,
        (parameters.k)
            .wrapping_sub(parameters.d)
            .wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint),
    ) == 0
    {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Failed to allocate dmer map: out of memory\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
    } else if dict.is_null() || freqs.is_null() {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Failed to allocate buffers: out of memory\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
    } else {
        memcpy(
            freqs as *mut std::ffi::c_void,
            (*ctx).freqs as *const std::ffi::c_void,
            ((*ctx).suffixSize).wrapping_mul(::core::mem::size_of::<u32>() as std::ffi::c_ulong),
        );
        let tail = COVER_buildDictionary(
            ctx,
            freqs,
            &mut activeDmers,
            dict as *mut std::ffi::c_void,
            dictBufferCapacity,
            parameters,
        );
        selection = COVER_selectDict(
            dict.offset(tail as isize),
            dictBufferCapacity,
            dictBufferCapacity.wrapping_sub(tail),
            (*ctx).samples,
            (*ctx).samplesSizes,
            (*ctx).nbTrainSamples as std::ffi::c_uint,
            (*ctx).nbTrainSamples,
            (*ctx).nbSamples,
            parameters,
            (*ctx).offsets,
            totalCompressedSize,
        );
        if COVER_dictSelectionIsError(selection) != 0 && displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Failed to select dictionary\n\0" as *const u8 as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
    }
    free(dict as *mut std::ffi::c_void);
    COVER_best_finish((*data).best, parameters, selection);
    free(data as *mut std::ffi::c_void);
    COVER_map_destroy(&mut activeDmers);
    COVER_dictSelectionFree(selection);
    free(freqs as *mut std::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ZDICT_optimizeTrainFromBuffer_cover(
    mut dictBuffer: *mut std::ffi::c_void,
    mut dictBufferCapacity: size_t,
    mut samplesBuffer: *const std::ffi::c_void,
    mut samplesSizes: *const size_t,
    mut nbSamples: std::ffi::c_uint,
    mut parameters: *mut ZDICT_cover_params_t,
) -> size_t {
    let nbThreads = (*parameters).nbThreads;
    let splitPoint = if (*parameters).splitPoint <= 0.0f64 {
        COVER_DEFAULT_SPLITPOINT
    } else {
        (*parameters).splitPoint
    };
    let kMinD = if (*parameters).d == 0 as std::ffi::c_int as std::ffi::c_uint {
        6 as std::ffi::c_int as std::ffi::c_uint
    } else {
        (*parameters).d
    };
    let kMaxD = if (*parameters).d == 0 as std::ffi::c_int as std::ffi::c_uint {
        8 as std::ffi::c_int as std::ffi::c_uint
    } else {
        (*parameters).d
    };
    let kMinK = if (*parameters).k == 0 as std::ffi::c_int as std::ffi::c_uint {
        50 as std::ffi::c_int as std::ffi::c_uint
    } else {
        (*parameters).k
    };
    let kMaxK = if (*parameters).k == 0 as std::ffi::c_int as std::ffi::c_uint {
        2000 as std::ffi::c_int as std::ffi::c_uint
    } else {
        (*parameters).k
    };
    let kSteps = if (*parameters).steps == 0 as std::ffi::c_int as std::ffi::c_uint {
        40 as std::ffi::c_int as std::ffi::c_uint
    } else {
        (*parameters).steps
    };
    let kStepSize = if kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
        > 1 as std::ffi::c_int as std::ffi::c_uint
    {
        kMaxK.wrapping_sub(kMinK).wrapping_div(kSteps)
    } else {
        1 as std::ffi::c_int as std::ffi::c_uint
    };
    let kIterations = (1 as std::ffi::c_int as std::ffi::c_uint)
        .wrapping_add(
            kMaxD
                .wrapping_sub(kMinD)
                .wrapping_div(2 as std::ffi::c_int as std::ffi::c_uint),
        )
        .wrapping_mul(
            (1 as std::ffi::c_int as std::ffi::c_uint)
                .wrapping_add(kMaxK.wrapping_sub(kMinK).wrapping_div(kStepSize)),
        );
    let shrinkDict = 0 as std::ffi::c_int as std::ffi::c_uint;
    let mut displayLevel = (*parameters).zParams.notificationLevel as std::ffi::c_int;
    let mut iteration = 1 as std::ffi::c_int as std::ffi::c_uint;
    let mut d: std::ffi::c_uint = 0;
    let mut k: std::ffi::c_uint = 0;
    let mut best = COVER_best_s {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: std::ptr::null_mut::<__pthread_internal_list>(),
                    __next: std::ptr::null_mut::<__pthread_internal_list>(),
                },
            },
        },
        cond: pthread_cond_t {
            __data: __pthread_cond_s {
                __wseq: __atomic_wide_counter { __value64: 0 },
                __g1_start: __atomic_wide_counter { __value64: 0 },
                __g_refs: [0; 2],
                __g_size: [0; 2],
                __g1_orig_size: 0,
                __wrefs: 0,
                __g_signals: [0; 2],
            },
        },
        liveJobs: 0,
        dict: std::ptr::null_mut::<std::ffi::c_void>(),
        dictSize: 0,
        parameters: ZDICT_cover_params_t {
            k: 0,
            d: 0,
            steps: 0,
            nbThreads: 0,
            splitPoint: 0.,
            shrinkDict: 0,
            shrinkDictMaxRegression: 0,
            zParams: ZDICT_params_t {
                compressionLevel: 0,
                notificationLevel: 0,
                dictID: 0,
            },
        },
        compressedSize: 0,
    };
    let mut pool = NULL as *mut POOL_ctx;
    let mut warned = 0 as std::ffi::c_int;
    let mut lastUpdateTime = 0 as std::ffi::c_int as clock_t;
    if splitPoint <= 0 as std::ffi::c_int as std::ffi::c_double
        || splitPoint > 1 as std::ffi::c_int as std::ffi::c_double
    {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Incorrect parameters\n\0" as *const u8 as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
    }
    if kMinK < kMaxD || kMaxK < kMinK {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Incorrect parameters\n\0" as *const u8 as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_parameter_outOfBound as std::ffi::c_int) as size_t;
    }
    if nbSamples == 0 as std::ffi::c_int as std::ffi::c_uint {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"Cover must have at least one input file\n\0" as *const u8
                    as *const std::ffi::c_char,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    if dictBufferCapacity < ZDICT_DICTSIZE_MIN as size_t {
        if displayLevel >= 1 as std::ffi::c_int {
            fprintf(
                stderr,
                b"dictBufferCapacity must be at least %u\n\0" as *const u8
                    as *const std::ffi::c_char,
                256 as std::ffi::c_int,
            );
            fflush(stderr);
        }
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if nbThreads > 1 as std::ffi::c_int as std::ffi::c_uint {
        pool = POOL_create(nbThreads as size_t, 1 as std::ffi::c_int as size_t);
        if pool.is_null() {
            return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
        }
    }
    COVER_best_init(&mut best);
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"Trying %u different sets of parameters\n\0" as *const u8 as *const std::ffi::c_char,
            kIterations,
        );
        fflush(stderr);
    }
    d = kMinD;
    while d <= kMaxD {
        let mut ctx = COVER_ctx_t {
            samples: std::ptr::null::<u8>(),
            offsets: std::ptr::null_mut::<size_t>(),
            samplesSizes: std::ptr::null::<size_t>(),
            nbSamples: 0,
            nbTrainSamples: 0,
            nbTestSamples: 0,
            suffix: std::ptr::null_mut::<u32>(),
            suffixSize: 0,
            freqs: std::ptr::null_mut::<u32>(),
            dmerAt: std::ptr::null_mut::<u32>(),
            d: 0,
            displayLevel: 0,
        };
        if displayLevel >= 3 as std::ffi::c_int {
            fprintf(
                stderr,
                b"d=%u\n\0" as *const u8 as *const std::ffi::c_char,
                d,
            );
            fflush(stderr);
        }
        let childDisplayLevel = if displayLevel == 0 as std::ffi::c_int {
            0 as std::ffi::c_int
        } else {
            displayLevel - 1 as std::ffi::c_int
        };
        let initVal = COVER_ctx_init(
            &mut ctx,
            samplesBuffer,
            samplesSizes,
            nbSamples,
            d,
            splitPoint,
            childDisplayLevel,
        );
        if ERR_isError(initVal) != 0 {
            if displayLevel >= 1 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"Failed to initialize context\n\0" as *const u8 as *const std::ffi::c_char,
                );
                fflush(stderr);
            }
            COVER_best_destroy(&mut best);
            POOL_free(pool);
            return initVal;
        }
        if warned == 0 {
            COVER_warnOnSmallCorpus(dictBufferCapacity, ctx.suffixSize, displayLevel);
            warned = 1 as std::ffi::c_int;
        }
        k = kMinK;
        while k <= kMaxK {
            let mut data =
                malloc(::core::mem::size_of::<COVER_tryParameters_data_t>() as std::ffi::c_ulong)
                    as *mut COVER_tryParameters_data_t;
            if displayLevel >= 3 as std::ffi::c_int {
                fprintf(
                    stderr,
                    b"k=%u\n\0" as *const u8 as *const std::ffi::c_char,
                    k,
                );
                fflush(stderr);
            }
            if data.is_null() {
                if displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Failed to allocate parameters\n\0" as *const u8
                            as *const std::ffi::c_char,
                    );
                    fflush(stderr);
                }
                COVER_best_destroy(&mut best);
                COVER_ctx_destroy(&mut ctx);
                POOL_free(pool);
                return -(ZSTD_error_memory_allocation as std::ffi::c_int) as size_t;
            }
            (*data).ctx = &mut ctx;
            (*data).best = &mut best;
            (*data).dictBufferCapacity = dictBufferCapacity;
            (*data).parameters = *parameters;
            (*data).parameters.k = k;
            (*data).parameters.d = d;
            (*data).parameters.splitPoint = splitPoint;
            (*data).parameters.steps = kSteps;
            (*data).parameters.shrinkDict = shrinkDict;
            (*data).parameters.zParams.notificationLevel = ctx.displayLevel as std::ffi::c_uint;
            if COVER_checkParameters((*data).parameters, dictBufferCapacity) == 0 {
                if displayLevel >= 1 as std::ffi::c_int {
                    fprintf(
                        stderr,
                        b"Cover parameters incorrect\n\0" as *const u8 as *const std::ffi::c_char,
                    );
                    fflush(stderr);
                }
                free(data as *mut std::ffi::c_void);
            } else {
                COVER_best_start(&mut best);
                if !pool.is_null() {
                    POOL_add(
                        pool,
                        Some(
                            COVER_tryParameters
                                as unsafe extern "C" fn(*mut std::ffi::c_void) -> (),
                        ),
                        data as *mut std::ffi::c_void,
                    );
                } else {
                    COVER_tryParameters(data as *mut std::ffi::c_void);
                }
                if displayLevel >= 2 as std::ffi::c_int {
                    let refreshRate = CLOCKS_PER_SEC as __clock_t
                        * 15 as std::ffi::c_int as __clock_t
                        / 100 as std::ffi::c_int as __clock_t;
                    if clock() - lastUpdateTime > refreshRate
                        || displayLevel >= 4 as std::ffi::c_int
                    {
                        lastUpdateTime = clock();
                        fprintf(
                            stderr,
                            b"\r%u%%       \0" as *const u8 as *const std::ffi::c_char,
                            iteration
                                .wrapping_mul(100 as std::ffi::c_int as std::ffi::c_uint)
                                .wrapping_div(kIterations),
                        );
                        fflush(stderr);
                    }
                }
                iteration = iteration.wrapping_add(1);
                iteration;
            }
            k = k.wrapping_add(kStepSize);
        }
        COVER_best_wait(&mut best);
        COVER_ctx_destroy(&mut ctx);
        d = d.wrapping_add(2 as std::ffi::c_int as std::ffi::c_uint);
    }
    if displayLevel >= 2 as std::ffi::c_int {
        fprintf(
            stderr,
            b"\r%79s\r\0" as *const u8 as *const std::ffi::c_char,
            b"\0" as *const u8 as *const std::ffi::c_char,
        );
        fflush(stderr);
    }
    let dictSize = best.dictSize;
    if ERR_isError(best.compressedSize) != 0 {
        let compressedSize = best.compressedSize;
        COVER_best_destroy(&mut best);
        POOL_free(pool);
        return compressedSize;
    }
    *parameters = best.parameters;
    memcpy(dictBuffer, best.dict, dictSize);
    COVER_best_destroy(&mut best);
    POOL_free(pool);
    dictSize
}
