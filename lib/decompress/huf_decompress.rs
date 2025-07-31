use crate::lib::common::bitstream::{BIT_DStream_t, BitContainerType};
use crate::lib::zstd::*;
use crate::{
    lib::common::entropy_common::{HUF_readStats_wksp, Workspace},
    MEM_isLittleEndian, MEM_read64, MEM_readLE16, MEM_readLEST, MEM_write16, MEM_write64,
};
extern "C" {
    fn HUF_decompress4X1_usingDTable_internal_fast_asm_loop(args: *mut HUF_DecompressFastArgs);
    fn HUF_decompress4X2_usingDTable_internal_fast_asm_loop(args: *mut HUF_DecompressFastArgs);
}
pub type ptrdiff_t = std::ffi::c_long;
pub type size_t = std::ffi::c_ulong;
pub type BIT_DStream_status = std::ffi::c_uint;
pub const BIT_DStream_overflow: BIT_DStream_status = 3;
pub const BIT_DStream_completed: BIT_DStream_status = 2;
pub const BIT_DStream_endOfBuffer: BIT_DStream_status = 1;
pub const BIT_DStream_unfinished: BIT_DStream_status = 0;
pub type HUF_DTable = u32;
pub type C2RustUnnamed_0 = std::ffi::c_uint;
pub const HUF_flags_disableFast: C2RustUnnamed_0 = 32;
pub const HUF_flags_disableAsm: C2RustUnnamed_0 = 16;
pub const HUF_flags_suspectUncompressible: C2RustUnnamed_0 = 8;
pub const HUF_flags_preferRepeat: C2RustUnnamed_0 = 4;
pub const HUF_flags_optimalDepth: C2RustUnnamed_0 = 2;
pub const HUF_flags_bmi2: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct algo_time_t {
    pub tableTime: u32,
    pub decode256Time: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DTableDesc {
    pub maxTableLog: u8,
    pub tableType: u8,
    pub tableLog: u8,
    pub reserved: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX1 {
    pub nbBits: u8,
    pub byte: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_ReadDTableX1_Workspace {
    pub rankVal: [u32; 13],
    pub rankStart: [u32; 13],
    pub statsWksp: Workspace,
    pub symbols: [u8; 256],
    pub huffWeight: [u8; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DEltX2 {
    pub sequence: u16,
    pub nbBits: u8,
    pub length: u8,
}
pub type rankValCol_t = [u32; 13];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_ReadDTableX2_Workspace {
    pub rankVal: [rankValCol_t; 12],
    pub rankStats: [u32; 13],
    pub rankStart0: [u32; 15],
    pub sortedSymbol: [sortedSymbol_t; 256],
    pub weightList: [u8; 256],
    pub calleeWksp: Workspace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: u8,
}
pub type HUF_DecompressUsingDTableFn = Option<
    unsafe extern "C" fn(
        *mut std::ffi::c_void,
        size_t,
        *const std::ffi::c_void,
        size_t,
        *const HUF_DTable,
    ) -> size_t,
>;
pub type HUF_DecompressFastLoopFn = Option<unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DecompressFastArgs {
    pub ip: [*const u8; 4],
    pub op: [*mut u8; 4],
    pub bits: [u64; 4],
    pub dt: *const std::ffi::c_void,
    pub ilowest: *const u8,
    pub oend: *mut u8,
    pub iend: [*const u8; 4],
}
#[inline]
unsafe extern "C" fn ZSTD_maybeNullPtrAdd(
    mut ptr: *mut std::ffi::c_void,
    mut add: ptrdiff_t,
) -> *mut std::ffi::c_void {
    if add > 0 as std::ffi::c_int as ptrdiff_t {
        (ptr as *mut std::ffi::c_char).offset(add as isize) as *mut std::ffi::c_void
    } else {
        ptr
    }
}
#[inline]
unsafe extern "C" fn MEM_32bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 4 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn MEM_64bits() -> std::ffi::c_uint {
    (::core::mem::size_of::<size_t>() as std::ffi::c_ulong
        == 8 as std::ffi::c_int as std::ffi::c_ulong) as std::ffi::c_int as std::ffi::c_uint
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
unsafe extern "C" fn ZSTD_countTrailingZeros64(mut val: u64) -> std::ffi::c_uint {
    (val as std::ffi::c_ulonglong).trailing_zeros() as i32 as std::ffi::c_uint
}
#[inline]
unsafe extern "C" fn ZSTD_highbit32(mut val: u32) -> std::ffi::c_uint {
    (31 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_countLeadingZeros32(val))
}
#[inline]
unsafe extern "C" fn BIT_initDStream(
    mut bitD: *mut BIT_DStream_t,
    mut srcBuffer: *const std::ffi::c_void,
    mut srcSize: size_t,
) -> size_t {
    if srcSize < 1 as std::ffi::c_int as size_t {
        libc::memset(
            bitD as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            ::core::mem::size_of::<BIT_DStream_t>() as std::ffi::c_ulong as libc::size_t,
        );
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    (*bitD).start = srcBuffer as *const std::ffi::c_char;
    (*bitD).limitPtr = ((*bitD).start)
        .offset(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize);
    if srcSize >= ::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong {
        (*bitD).ptr = (srcBuffer as *const std::ffi::c_char)
            .offset(srcSize as isize)
            .offset(-(::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong as isize));
        (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void) as usize;
        let lastByte = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
        (*bitD).bitsConsumed = if lastByte as std::ffi::c_int != 0 {
            (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_highbit32(lastByte as u32))
        } else {
            0 as std::ffi::c_int as std::ffi::c_uint
        };
        if lastByte as std::ffi::c_int == 0 as std::ffi::c_int {
            return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
        }
    } else {
        (*bitD).ptr = (*bitD).start;
        (*bitD).bitContainer = *((*bitD).start as *const u8) as BitContainerType;
        let mut current_block_32: u64;
        match srcSize {
            7 => {
                (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                    (*(srcBuffer as *const u8).offset(6 as std::ffi::c_int as isize)
                        as BitContainerType)
                        << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                            .wrapping_sub(16 as std::ffi::c_int as std::ffi::c_ulong),
                );
                current_block_32 = 10674155929662485793;
            }
            6 => {
                current_block_32 = 10674155929662485793;
            }
            5 => {
                current_block_32 = 16118346003814681330;
            }
            4 => {
                current_block_32 = 11138735194807316286;
            }
            3 => {
                current_block_32 = 3119509932921184778;
            }
            2 => {
                current_block_32 = 13845399199001887291;
            }
            _ => {
                current_block_32 = 16203760046146113240;
            }
        }
        if current_block_32 == 10674155929662485793 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(5 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                        .wrapping_sub(24 as std::ffi::c_int as std::ffi::c_ulong),
            );
            current_block_32 = 16118346003814681330;
        }
        if current_block_32 == 16118346003814681330 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(4 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                        .wrapping_sub(32 as std::ffi::c_int as std::ffi::c_ulong),
            );
            current_block_32 = 11138735194807316286;
        }
        if current_block_32 == 11138735194807316286 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(3 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << 24 as std::ffi::c_int,
            );
            current_block_32 = 3119509932921184778;
        }
        if current_block_32 == 3119509932921184778 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(2 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << 16 as std::ffi::c_int,
            );
            current_block_32 = 13845399199001887291;
        }
        if current_block_32 == 13845399199001887291 {
            (*bitD).bitContainer = ((*bitD).bitContainer).wrapping_add(
                (*(srcBuffer as *const u8).offset(1 as std::ffi::c_int as isize)
                    as BitContainerType)
                    << 8 as std::ffi::c_int,
            );
        }
        let lastByte_0 = *(srcBuffer as *const u8)
            .offset(srcSize.wrapping_sub(1 as std::ffi::c_int as size_t) as isize);
        (*bitD).bitsConsumed = if lastByte_0 as std::ffi::c_int != 0 {
            (8 as std::ffi::c_int as std::ffi::c_uint)
                .wrapping_sub(ZSTD_highbit32(lastByte_0 as u32))
        } else {
            0 as std::ffi::c_int as std::ffi::c_uint
        };
        if lastByte_0 as std::ffi::c_int == 0 as std::ffi::c_int {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(
            (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong).wrapping_sub(srcSize)
                as u32
                * 8 as std::ffi::c_int as u32,
        );
    }
    srcSize
}
#[inline]
unsafe extern "C" fn BIT_lookBitsFast(
    mut bitD: *const BIT_DStream_t,
    mut nbBits: u32,
) -> BitContainerType {
    let regMask = (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
        .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as u32;
    (*bitD).bitContainer << ((*bitD).bitsConsumed & regMask)
        >> (regMask
            .wrapping_add(1 as std::ffi::c_int as u32)
            .wrapping_sub(nbBits)
            & regMask)
}
#[inline(always)]
unsafe extern "C" fn BIT_skipBits(mut bitD: *mut BIT_DStream_t, mut nbBits: u32) {
    (*bitD).bitsConsumed = ((*bitD).bitsConsumed).wrapping_add(nbBits);
}
#[inline]
unsafe extern "C" fn BIT_reloadDStream_internal(
    mut bitD: *mut BIT_DStream_t,
) -> BIT_DStream_status {
    (*bitD).ptr = ((*bitD).ptr).offset(-(((*bitD).bitsConsumed >> 3 as std::ffi::c_int) as isize));
    (*bitD).bitsConsumed &= 7 as std::ffi::c_int as std::ffi::c_uint;
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void) as usize;
    BIT_DStream_unfinished
}
#[inline]
unsafe extern "C" fn BIT_reloadDStreamFast(mut bitD: *mut BIT_DStream_t) -> BIT_DStream_status {
    if ((*bitD).ptr < (*bitD).limitPtr) as std::ffi::c_int as std::ffi::c_long != 0 {
        return BIT_DStream_overflow;
    }
    BIT_reloadDStream_internal(bitD)
}
#[inline(always)]
unsafe extern "C" fn BIT_reloadDStream(mut bitD: *mut BIT_DStream_t) -> BIT_DStream_status {
    if ((*bitD).bitsConsumed as std::ffi::c_ulong
        > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)) as std::ffi::c_int
        as std::ffi::c_long
        != 0
    {
        static mut zeroFilled: BitContainerType = 0 as std::ffi::c_int as BitContainerType;
        (*bitD).ptr = &zeroFilled as *const BitContainerType as *const std::ffi::c_char;
        return BIT_DStream_overflow;
    }
    if (*bitD).ptr >= (*bitD).limitPtr {
        return BIT_reloadDStream_internal(bitD);
    }
    if (*bitD).ptr == (*bitD).start {
        if ((*bitD).bitsConsumed as std::ffi::c_ulong)
            < (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            return BIT_DStream_endOfBuffer;
        }
        return BIT_DStream_completed;
    }
    let mut nbBytes = (*bitD).bitsConsumed >> 3 as std::ffi::c_int;
    let mut result = BIT_DStream_unfinished;
    if ((*bitD).ptr).offset(-(nbBytes as isize)) < (*bitD).start {
        nbBytes = ((*bitD).ptr).offset_from((*bitD).start) as std::ffi::c_long as u32;
        result = BIT_DStream_endOfBuffer;
    }
    (*bitD).ptr = ((*bitD).ptr).offset(-(nbBytes as isize));
    (*bitD).bitsConsumed =
        ((*bitD).bitsConsumed).wrapping_sub(nbBytes * 8 as std::ffi::c_int as u32);
    (*bitD).bitContainer = MEM_readLEST((*bitD).ptr as *const std::ffi::c_void) as usize;
    result
}
#[inline]
unsafe extern "C" fn BIT_endOfDStream(mut DStream: *const BIT_DStream_t) -> std::ffi::c_uint {
    ((*DStream).ptr == (*DStream).start
        && (*DStream).bitsConsumed as std::ffi::c_ulong
            == (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)) as std::ffi::c_int
        as std::ffi::c_uint
}
pub const HUF_TABLELOG_MAX: std::ffi::c_int = 12 as std::ffi::c_int;
pub const HUF_SYMBOLVALUE_MAX: std::ffi::c_int = 255 as std::ffi::c_int;
pub const HUF_DECODER_FAST_TABLELOG: std::ffi::c_int = 11 as std::ffi::c_int;
pub const HUF_ENABLE_FAST_DECODE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const HUF_isError: unsafe extern "C" fn(size_t) -> std::ffi::c_uint = ERR_isError;
unsafe extern "C" fn HUF_getDTableDesc(mut table: *const HUF_DTable) -> DTableDesc {
    let mut dtd = DTableDesc {
        maxTableLog: 0,
        tableType: 0,
        tableLog: 0,
        reserved: 0,
    };
    libc::memcpy(
        &mut dtd as *mut DTableDesc as *mut std::ffi::c_void,
        table as *const std::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as std::ffi::c_ulong as libc::size_t,
    );
    dtd
}
unsafe extern "C" fn HUF_initFastDStream(mut ip: *const u8) -> size_t {
    let lastByte = *ip.offset(7 as std::ffi::c_int as isize);
    let bitsConsumed = (if lastByte as std::ffi::c_int != 0 {
        (8 as std::ffi::c_int as std::ffi::c_uint).wrapping_sub(ZSTD_highbit32(lastByte as u32))
    } else {
        0 as std::ffi::c_int as std::ffi::c_uint
    }) as size_t;
    let value = MEM_readLEST(ip as *const std::ffi::c_void) | 1 as std::ffi::c_int as size_t;
    value << bitsConsumed
}
unsafe extern "C" fn HUF_DecompressFastArgs_init(
    mut args: *mut HUF_DecompressFastArgs,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    let mut dt = DTable.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    let dtLog = (HUF_getDTableDesc(DTable)).tableLog as u32;
    let istart = src as *const u8;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    if MEM_isLittleEndian() == 0 || MEM_32bits() != 0 {
        return 0 as std::ffi::c_int as size_t;
    }
    if dstSize == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    if srcSize < 10 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if dtLog != HUF_DECODER_FAST_TABLELOG as u32 {
        return 0 as std::ffi::c_int as size_t;
    }
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as size_t;
    let length2 =
        MEM_readLE16(istart.offset(2 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length3 =
        MEM_readLE16(istart.offset(4 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length4 = srcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6 as std::ffi::c_int as size_t),
    );
    let fresh0 = &mut (*((*args).iend)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize));
    *fresh0 = istart.offset(6 as std::ffi::c_int as isize);
    let fresh1 = &mut (*((*args).iend)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize));
    *fresh1 = (*((*args).iend)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize))
    .offset(length1 as isize);
    let fresh2 = &mut (*((*args).iend)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize));
    *fresh2 = (*((*args).iend)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize))
    .offset(length2 as isize);
    let fresh3 = &mut (*((*args).iend)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize));
    *fresh3 = (*((*args).iend)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize))
    .offset(length3 as isize);
    if length1 < 8 as std::ffi::c_int as size_t
        || length2 < 8 as std::ffi::c_int as size_t
        || length3 < 8 as std::ffi::c_int as size_t
        || length4 < 8 as std::ffi::c_int as size_t
    {
        return 0 as std::ffi::c_int as size_t;
    }
    if length4 > srcSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let fresh4 = &mut (*((*args).ip)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize));
    *fresh4 = (*((*args).iend)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize))
    .offset(-(::core::mem::size_of::<u64>() as std::ffi::c_ulong as isize));
    let fresh5 = &mut (*((*args).ip)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize));
    *fresh5 = (*((*args).iend)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize))
    .offset(-(::core::mem::size_of::<u64>() as std::ffi::c_ulong as isize));
    let fresh6 = &mut (*((*args).ip)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize));
    *fresh6 = (*((*args).iend)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize))
    .offset(-(::core::mem::size_of::<u64>() as std::ffi::c_ulong as isize));
    let fresh7 = &mut (*((*args).ip)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize));
    *fresh7 = (src as *const u8)
        .offset(srcSize as isize)
        .offset(-(::core::mem::size_of::<u64>() as std::ffi::c_ulong as isize));
    let fresh8 = &mut (*((*args).op)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize));
    *fresh8 = dst as *mut u8;
    let fresh9 = &mut (*((*args).op)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize));
    *fresh9 = (*((*args).op)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize))
    .offset(
        (dstSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t)
            as isize,
    );
    let fresh10 = &mut (*((*args).op)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize));
    *fresh10 = (*((*args).op)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize))
    .offset(
        (dstSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t)
            as isize,
    );
    let fresh11 = &mut (*((*args).op)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize));
    *fresh11 = (*((*args).op)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize))
    .offset(
        (dstSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t)
            as isize,
    );
    if *((*args).op)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize)
        >= oend
    {
        return 0 as std::ffi::c_int as size_t;
    }
    *((*args).bits)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) = HUF_initFastDStream(
        *((*args).ip)
            .as_mut_ptr()
            .offset(0 as std::ffi::c_int as isize),
    );
    *((*args).bits)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize) = HUF_initFastDStream(
        *((*args).ip)
            .as_mut_ptr()
            .offset(1 as std::ffi::c_int as isize),
    );
    *((*args).bits)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize) = HUF_initFastDStream(
        *((*args).ip)
            .as_mut_ptr()
            .offset(2 as std::ffi::c_int as isize),
    );
    *((*args).bits)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize) = HUF_initFastDStream(
        *((*args).ip)
            .as_mut_ptr()
            .offset(3 as std::ffi::c_int as isize),
    );
    (*args).ilowest = istart;
    (*args).oend = oend;
    (*args).dt = dt;
    1 as std::ffi::c_int as size_t
}
unsafe extern "C" fn HUF_initRemainingDStream(
    mut bit: *mut BIT_DStream_t,
    mut args: *const HUF_DecompressFastArgs,
    mut stream: std::ffi::c_int,
    mut segmentEnd: *mut u8,
) -> size_t {
    if *((*args).op).as_ptr().offset(stream as isize) > segmentEnd {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if *((*args).ip).as_ptr().offset(stream as isize)
        < (*((*args).iend).as_ptr().offset(stream as isize))
            .offset(-(8 as std::ffi::c_int as isize))
    {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    (*bit).bitContainer =
        MEM_readLEST(*((*args).ip).as_ptr().offset(stream as isize) as *const std::ffi::c_void)
            as usize;
    (*bit).bitsConsumed =
        ZSTD_countTrailingZeros64(*((*args).bits).as_ptr().offset(stream as isize));
    (*bit).start = (*args).ilowest as *const std::ffi::c_char;
    (*bit).limitPtr =
        ((*bit).start).offset(::core::mem::size_of::<size_t>() as std::ffi::c_ulong as isize);
    (*bit).ptr = *((*args).ip).as_ptr().offset(stream as isize) as *const std::ffi::c_char;
    0 as std::ffi::c_int as size_t
}
unsafe extern "C" fn HUF_DEltX1_set4(mut symbol: u8, mut nbBits: u8) -> u64 {
    let mut D4: u64 = 0;
    if MEM_isLittleEndian() != 0 {
        D4 = (((symbol as std::ffi::c_int) << 8 as std::ffi::c_int) + nbBits as std::ffi::c_int)
            as u64;
    } else {
        D4 = (symbol as std::ffi::c_int + ((nbBits as std::ffi::c_int) << 8 as std::ffi::c_int))
            as u64;
    }
    D4 = (D4 as std::ffi::c_ulonglong).wrapping_mul(0x1000100010001 as std::ffi::c_ulonglong) as u64
        as u64;
    D4
}
unsafe extern "C" fn HUF_rescaleStats(
    mut huffWeight: *mut u8,
    mut rankVal: *mut u32,
    mut nbSymbols: u32,
    mut tableLog: u32,
    mut targetTableLog: u32,
) -> u32 {
    if tableLog > targetTableLog {
        return tableLog;
    }
    if tableLog < targetTableLog {
        let scale = targetTableLog.wrapping_sub(tableLog);
        let mut s: u32 = 0;
        s = 0 as std::ffi::c_int as u32;
        while s < nbSymbols {
            let fresh12 = &mut (*huffWeight.offset(s as isize));
            *fresh12 = (*fresh12 as std::ffi::c_int
                + (if *huffWeight.offset(s as isize) as std::ffi::c_int == 0 as std::ffi::c_int {
                    0 as std::ffi::c_int as u32
                } else {
                    scale
                }) as u8 as std::ffi::c_int) as u8;
            s = s.wrapping_add(1);
            s;
        }
        s = targetTableLog;
        while s > scale {
            *rankVal.offset(s as isize) = *rankVal.offset(s.wrapping_sub(scale) as isize);
            s = s.wrapping_sub(1);
            s;
        }
        s = scale;
        while s > 0 as std::ffi::c_int as u32 {
            *rankVal.offset(s as isize) = 0 as std::ffi::c_int as u32;
            s = s.wrapping_sub(1);
            s;
        }
    }
    targetTableLog
}

pub unsafe fn HUF_readDTableX1_wksp(
    mut DTable: *mut HUF_DTable,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut tableLog = 0 as std::ffi::c_int as u32;
    let mut nbSymbols = 0 as std::ffi::c_int as u32;
    let mut iSize: size_t = 0;
    let dtPtr = DTable.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let dt = dtPtr as *mut HUF_DEltX1;
    let mut wksp = workSpace as *mut HUF_ReadDTableX1_Workspace;
    if ::core::mem::size_of::<HUF_ReadDTableX1_Workspace>() as std::ffi::c_ulong > wkspSize {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    iSize = HUF_readStats_wksp(
        &mut (*wksp).huffWeight,
        (HUF_SYMBOLVALUE_MAX + 1 as std::ffi::c_int) as size_t,
        &mut (*wksp).rankVal,
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
        &mut (*wksp).statsWksp,
        flags,
    );
    if ERR_isError(iSize) != 0 {
        return iSize;
    }
    let mut dtd = HUF_getDTableDesc(DTable);
    let maxTableLog = (dtd.maxTableLog as std::ffi::c_int + 1 as std::ffi::c_int) as u32;
    let targetTableLog = if maxTableLog < 11 as std::ffi::c_int as u32 {
        maxTableLog
    } else {
        11 as std::ffi::c_int as u32
    };
    tableLog = HUF_rescaleStats(
        ((*wksp).huffWeight).as_mut_ptr(),
        ((*wksp).rankVal).as_mut_ptr(),
        nbSymbols,
        tableLog,
        targetTableLog,
    );
    if tableLog > (dtd.maxTableLog as std::ffi::c_int + 1 as std::ffi::c_int) as u32 {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    dtd.tableType = 0 as std::ffi::c_int as u8;
    dtd.tableLog = tableLog as u8;
    libc::memcpy(
        DTable as *mut std::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const std::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as std::ffi::c_ulong as libc::size_t,
    );
    let mut n: std::ffi::c_int = 0;
    let mut nextRankStart = 0 as std::ffi::c_int as u32;
    let unroll = 4 as std::ffi::c_int;
    let nLimit = nbSymbols as std::ffi::c_int - unroll + 1 as std::ffi::c_int;
    n = 0 as std::ffi::c_int;
    while n < tableLog as std::ffi::c_int + 1 as std::ffi::c_int {
        let curr = nextRankStart;
        nextRankStart =
            nextRankStart.wrapping_add(*((*wksp).rankVal).as_mut_ptr().offset(n as isize));
        *((*wksp).rankStart).as_mut_ptr().offset(n as isize) = curr;
        n += 1;
        n;
    }
    n = 0 as std::ffi::c_int;
    while n < nLimit {
        let mut u: std::ffi::c_int = 0;
        u = 0 as std::ffi::c_int;
        while u < unroll {
            let w = *((*wksp).huffWeight).as_mut_ptr().offset((n + u) as isize) as size_t;
            let fresh13 = &mut (*((*wksp).rankStart).as_mut_ptr().offset(w as isize));
            let fresh14 = *fresh13;
            *fresh13 = (*fresh13).wrapping_add(1);
            *((*wksp).symbols).as_mut_ptr().offset(fresh14 as isize) = (n + u) as u8;
            u += 1;
            u;
        }
        n += unroll;
    }
    while n < nbSymbols as std::ffi::c_int {
        let w_0 = *((*wksp).huffWeight).as_mut_ptr().offset(n as isize) as size_t;
        let fresh15 = &mut (*((*wksp).rankStart).as_mut_ptr().offset(w_0 as isize));
        let fresh16 = *fresh15;
        *fresh15 = (*fresh15).wrapping_add(1);
        *((*wksp).symbols).as_mut_ptr().offset(fresh16 as isize) = n as u8;
        n += 1;
        n;
    }
    let mut w_1: u32 = 0;
    let mut symbol = *((*wksp).rankVal)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) as std::ffi::c_int;
    let mut rankStart = 0 as std::ffi::c_int;
    w_1 = 1 as std::ffi::c_int as u32;
    while w_1 < tableLog.wrapping_add(1 as std::ffi::c_int as u32) {
        let symbolCount = *((*wksp).rankVal).as_mut_ptr().offset(w_1 as isize) as std::ffi::c_int;
        let length = (1 as std::ffi::c_int) << w_1 >> 1 as std::ffi::c_int;
        let mut uStart = rankStart;
        let nbBits = tableLog
            .wrapping_add(1 as std::ffi::c_int as u32)
            .wrapping_sub(w_1) as u8;
        let mut s: std::ffi::c_int = 0;
        let mut u_0: std::ffi::c_int = 0;
        match length {
            1 => {
                s = 0 as std::ffi::c_int;
                while s < symbolCount {
                    let mut D = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D.byte = *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize);
                    D.nbBits = nbBits;
                    *dt.offset(uStart as isize) = D;
                    uStart += 1 as std::ffi::c_int;
                    s += 1;
                    s;
                }
            }
            2 => {
                s = 0 as std::ffi::c_int;
                while s < symbolCount {
                    let mut D_0 = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D_0.byte = *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize);
                    D_0.nbBits = nbBits;
                    *dt.offset((uStart + 0 as std::ffi::c_int) as isize) = D_0;
                    *dt.offset((uStart + 1 as std::ffi::c_int) as isize) = D_0;
                    uStart += 2 as std::ffi::c_int;
                    s += 1;
                    s;
                }
            }
            4 => {
                s = 0 as std::ffi::c_int;
                while s < symbolCount {
                    let D4 = HUF_DEltX1_set4(
                        *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize),
                        nbBits,
                    );
                    MEM_write64(dt.offset(uStart as isize) as *mut std::ffi::c_void, D4);
                    uStart += 4 as std::ffi::c_int;
                    s += 1;
                    s;
                }
            }
            8 => {
                s = 0 as std::ffi::c_int;
                while s < symbolCount {
                    let D4_0 = HUF_DEltX1_set4(
                        *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize),
                        nbBits,
                    );
                    MEM_write64(dt.offset(uStart as isize) as *mut std::ffi::c_void, D4_0);
                    MEM_write64(
                        dt.offset(uStart as isize)
                            .offset(4 as std::ffi::c_int as isize)
                            as *mut std::ffi::c_void,
                        D4_0,
                    );
                    uStart += 8 as std::ffi::c_int;
                    s += 1;
                    s;
                }
            }
            _ => {
                s = 0 as std::ffi::c_int;
                while s < symbolCount {
                    let D4_1 = HUF_DEltX1_set4(
                        *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize),
                        nbBits,
                    );
                    u_0 = 0 as std::ffi::c_int;
                    while u_0 < length {
                        MEM_write64(
                            dt.offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(0 as std::ffi::c_int as isize)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt.offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(4 as std::ffi::c_int as isize)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt.offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(8 as std::ffi::c_int as isize)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt.offset(uStart as isize)
                                .offset(u_0 as isize)
                                .offset(12 as std::ffi::c_int as isize)
                                as *mut std::ffi::c_void,
                            D4_1,
                        );
                        u_0 += 16 as std::ffi::c_int;
                    }
                    uStart += length;
                    s += 1;
                    s;
                }
            }
        }
        symbol += symbolCount;
        rankStart += symbolCount * length;
        w_1 = w_1.wrapping_add(1);
        w_1;
    }
    iSize
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeSymbolX1(
    mut Dstream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX1,
    dtLog: u32,
) -> u8 {
    let val = BIT_lookBitsFast(Dstream, dtLog);
    let c = (*dt.offset(val as isize)).byte;
    BIT_skipBits(Dstream, (*dt.offset(val as isize)).nbBits as u32);
    c
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeStreamX1(
    mut p: *mut u8,
    bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut u8,
    dt: *const HUF_DEltX1,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) as std::ffi::c_long > 3 as std::ffi::c_int as std::ffi::c_long {
        while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
            == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
            as std::ffi::c_int
            & (p < pEnd.offset(-(3 as std::ffi::c_int as isize))) as std::ffi::c_int
            != 0
        {
            if MEM_64bits() != 0 {
                let fresh17 = p;
                p = p.offset(1);
                *fresh17 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                let fresh18 = p;
                p = p.offset(1);
                *fresh18 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh19 = p;
                p = p.offset(1);
                *fresh19 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            let fresh20 = p;
            p = p.offset(1);
            *fresh20 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
        }
    } else {
        BIT_reloadDStream(bitDPtr);
    }
    if MEM_32bits() != 0 {
        while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
            == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
            as std::ffi::c_int
            & (p < pEnd) as std::ffi::c_int
            != 0
        {
            let fresh21 = p;
            p = p.offset(1);
            *fresh21 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
        }
    }
    while p < pEnd {
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
    }
    pEnd.offset_from(pStart) as std::ffi::c_long as size_t
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    let mut op = dst as *mut u8;
    let oend = ZSTD_maybeNullPtrAdd(op as *mut std::ffi::c_void, dstSize as ptrdiff_t) as *mut u8;
    let mut dtPtr = DTable.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    HUF_decodeStreamX1(op, &mut bitD, oend, dt, dtLog);
    if BIT_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    if cSrcSize < 10 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if dstSize < 6 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend.offset(-(3 as std::ffi::c_int as isize));
    let dtPtr = DTable.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as size_t;
    let length2 =
        MEM_readLE16(istart.offset(2 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length3 =
        MEM_readLE16(istart.offset(4 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6 as std::ffi::c_int as size_t),
    );
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
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let mut endSignal = 1 as std::ffi::c_int as u32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if opStart4 > oend {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let _var_err__ = BIT_initDStream(&mut bitD1, istart1 as *const std::ffi::c_void, length1);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let _var_err___0 = BIT_initDStream(&mut bitD2, istart2 as *const std::ffi::c_void, length2);
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let _var_err___1 = BIT_initDStream(&mut bitD3, istart3 as *const std::ffi::c_void, length3);
    if ERR_isError(_var_err___1) != 0 {
        return _var_err___1;
    }
    let _var_err___2 = BIT_initDStream(&mut bitD4, istart4 as *const std::ffi::c_void, length4);
    if ERR_isError(_var_err___2) != 0 {
        return _var_err___2;
    }
    if oend.offset_from(op4) as std::ffi::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
    {
        while endSignal & (op4 < olimit) as std::ffi::c_int as u32 != 0 {
            if MEM_64bits() != 0 {
                let fresh23 = op1;
                op1 = op1.offset(1);
                *fresh23 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh24 = op2;
                op2 = op2.offset(1);
                *fresh24 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh25 = op3;
                op3 = op3.offset(1);
                *fresh25 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh26 = op4;
                op4 = op4.offset(1);
                *fresh26 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                let fresh27 = op1;
                op1 = op1.offset(1);
                *fresh27 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                let fresh28 = op2;
                op2 = op2.offset(1);
                *fresh28 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                let fresh29 = op3;
                op3 = op3.offset(1);
                *fresh29 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                let fresh30 = op4;
                op4 = op4.offset(1);
                *fresh30 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh31 = op1;
                op1 = op1.offset(1);
                *fresh31 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh32 = op2;
                op2 = op2.offset(1);
                *fresh32 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh33 = op3;
                op3 = op3.offset(1);
                *fresh33 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if MEM_64bits() != 0 {
                let fresh34 = op4;
                op4 = op4.offset(1);
                *fresh34 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            }
            let fresh35 = op1;
            op1 = op1.offset(1);
            *fresh35 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            let fresh36 = op2;
            op2 = op2.offset(1);
            *fresh36 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            let fresh37 = op3;
            op3 = op3.offset(1);
            *fresh37 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            let fresh38 = op4;
            op4 = op4.offset(1);
            *fresh38 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            endSignal &= (BIT_reloadDStreamFast(&mut bitD1) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
            endSignal &= (BIT_reloadDStreamFast(&mut bitD2) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
            endSignal &= (BIT_reloadDStreamFast(&mut bitD3) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
            endSignal &= (BIT_reloadDStreamFast(&mut bitD4) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
        }
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
    HUF_decodeStreamX1(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX1(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX1(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX1(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BIT_endOfDStream(&mut bitD1)
        & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3)
        & BIT_endOfDStream(&mut bitD4);
    if endCheck == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast_c_loop(
    mut args: *mut HUF_DecompressFastArgs,
) {
    let mut bits: [u64; 4] = [0; 4];
    let mut ip: [*const u8; 4] = [std::ptr::null::<u8>(); 4];
    let mut op: [*mut u8; 4] = [std::ptr::null_mut::<u8>(); 4];
    let dtable = (*args).dt as *const u16;
    let oend = (*args).oend;
    let ilowest = (*args).ilowest;
    libc::memcpy(
        &mut bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut (*args).bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut (*args).ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut (*args).op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    's_33: loop {
        let mut olimit = std::ptr::null_mut::<u8>();
        let mut stream: std::ffi::c_int = 0;
        stream = 0 as std::ffi::c_int;
        while stream < 4 as std::ffi::c_int {
            stream += 1;
            stream;
        }
        let oiters = oend.offset_from(*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
            as std::ffi::c_long as size_t
            / 5 as std::ffi::c_int as size_t;
        let iiters = (*ip.as_mut_ptr().offset(0 as std::ffi::c_int as isize)).offset_from(ilowest)
            as std::ffi::c_long as size_t
            / 7 as std::ffi::c_int as size_t;
        let iters = if oiters < iiters { oiters } else { iiters };
        let symbols = iters * 5 as std::ffi::c_int as size_t;
        olimit = (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize)).offset(symbols as isize);
        if *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) == olimit {
            break;
        }
        stream = 1 as std::ffi::c_int;
        while stream < 4 as std::ffi::c_int {
            if *ip.as_mut_ptr().offset(stream as isize)
                < *ip
                    .as_mut_ptr()
                    .offset((stream - 1 as std::ffi::c_int) as isize)
            {
                break 's_33;
            }
            stream += 1;
            stream;
        }
        stream = 1 as std::ffi::c_int;
        while stream < 4 as std::ffi::c_int {
            stream += 1;
            stream;
        }
        loop {
            let index = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry = *dtable.offset(index as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                entry & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
                .offset(0 as std::ffi::c_int as isize) =
                (entry >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_0 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_0 = *dtable.offset(index_0 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                entry_0 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                .offset(0 as std::ffi::c_int as isize) =
                (entry_0 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_1 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_1 = *dtable.offset(index_1 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                entry_1 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize))
                .offset(0 as std::ffi::c_int as isize) =
                (entry_1 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_2 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_2 = *dtable.offset(index_2 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                entry_2 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
                .offset(0 as std::ffi::c_int as isize) =
                (entry_2 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_3 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_3 = *dtable.offset(index_3 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                entry_3 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
                .offset(1 as std::ffi::c_int as isize) =
                (entry_3 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_4 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_4 = *dtable.offset(index_4 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                entry_4 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                .offset(1 as std::ffi::c_int as isize) =
                (entry_4 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_5 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_5 = *dtable.offset(index_5 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                entry_5 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize))
                .offset(1 as std::ffi::c_int as isize) =
                (entry_5 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_6 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_6 = *dtable.offset(index_6 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                entry_6 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
                .offset(1 as std::ffi::c_int as isize) =
                (entry_6 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_7 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_7 = *dtable.offset(index_7 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                entry_7 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
                .offset(2 as std::ffi::c_int as isize) =
                (entry_7 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_8 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_8 = *dtable.offset(index_8 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                entry_8 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                .offset(2 as std::ffi::c_int as isize) =
                (entry_8 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_9 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_9 = *dtable.offset(index_9 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                entry_9 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize))
                .offset(2 as std::ffi::c_int as isize) =
                (entry_9 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_10 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_10 = *dtable.offset(index_10 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                entry_10 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
                .offset(2 as std::ffi::c_int as isize) =
                (entry_10 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_11 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_11 = *dtable.offset(index_11 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                entry_11 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
                .offset(3 as std::ffi::c_int as isize) =
                (entry_11 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_12 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_12 = *dtable.offset(index_12 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                entry_12 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                .offset(3 as std::ffi::c_int as isize) =
                (entry_12 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_13 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_13 = *dtable.offset(index_13 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                entry_13 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize))
                .offset(3 as std::ffi::c_int as isize) =
                (entry_13 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_14 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_14 = *dtable.offset(index_14 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                entry_14 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
                .offset(3 as std::ffi::c_int as isize) =
                (entry_14 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_15 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_15 = *dtable.offset(index_15 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                entry_15 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
                .offset(4 as std::ffi::c_int as isize) =
                (entry_15 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_16 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_16 = *dtable.offset(index_16 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                entry_16 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                .offset(4 as std::ffi::c_int as isize) =
                (entry_16 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_17 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_17 = *dtable.offset(index_17 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                entry_17 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize))
                .offset(4 as std::ffi::c_int as isize) =
                (entry_17 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let index_18 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                >> 53 as std::ffi::c_int) as std::ffi::c_int;
            let entry_18 = *dtable.offset(index_18 as isize) as std::ffi::c_int;
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                entry_18 & 0x3f as std::ffi::c_int;
            *(*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
                .offset(4 as std::ffi::c_int as isize) =
                (entry_18 >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as u8;
            let ctz =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits = ctz & 7 as std::ffi::c_int;
            let nbBytes = ctz >> 3 as std::ffi::c_int;
            let fresh39 = &mut (*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
            *fresh39 = (*fresh39).offset(5 as std::ffi::c_int as isize);
            let fresh40 = &mut (*ip.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
            *fresh40 = (*fresh40).offset(-(nbBytes as isize));
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<= nbBits;
            let ctz_0 =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits_0 = ctz_0 & 7 as std::ffi::c_int;
            let nbBytes_0 = ctz_0 >> 3 as std::ffi::c_int;
            let fresh41 = &mut (*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
            *fresh41 = (*fresh41).offset(5 as std::ffi::c_int as isize);
            let fresh42 = &mut (*ip.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
            *fresh42 = (*fresh42).offset(-(nbBytes_0 as isize));
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<= nbBits_0;
            let ctz_1 =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits_1 = ctz_1 & 7 as std::ffi::c_int;
            let nbBytes_1 = ctz_1 >> 3 as std::ffi::c_int;
            let fresh43 = &mut (*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
            *fresh43 = (*fresh43).offset(5 as std::ffi::c_int as isize);
            let fresh44 = &mut (*ip.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
            *fresh44 = (*fresh44).offset(-(nbBytes_1 as isize));
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<= nbBits_1;
            let ctz_2 =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits_2 = ctz_2 & 7 as std::ffi::c_int;
            let nbBytes_2 = ctz_2 >> 3 as std::ffi::c_int;
            let fresh45 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
            *fresh45 = (*fresh45).offset(5 as std::ffi::c_int as isize);
            let fresh46 = &mut (*ip.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
            *fresh46 = (*fresh46).offset(-(nbBytes_2 as isize));
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<= nbBits_2;
            if *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) >= olimit {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let mut dt = DTable.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    let ilowest = cSrc as *const u8;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    let mut args = HUF_DecompressFastArgs {
        ip: [std::ptr::null::<u8>(); 4],
        op: [std::ptr::null_mut::<u8>(); 4],
        bits: [0; 4],
        dt: std::ptr::null::<std::ffi::c_void>(),
        ilowest: std::ptr::null::<u8>(),
        oend: std::ptr::null_mut::<u8>(),
        iend: [std::ptr::null::<u8>(); 4],
    };
    let ret = HUF_DecompressFastArgs_init(&mut args, dst, dstSize, cSrc, cSrcSize, DTable);
    let err_code = ret;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if ret == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    loopFn.unwrap_unchecked()(&mut args);
    let segmentSize =
        dstSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t;
    let mut segmentEnd = dst as *mut u8;
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < 4 as std::ffi::c_int {
        let mut bit = BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: std::ptr::null::<std::ffi::c_char>(),
            start: std::ptr::null::<std::ffi::c_char>(),
            limitPtr: std::ptr::null::<std::ffi::c_char>(),
        };
        if segmentSize <= oend.offset_from(segmentEnd) as std::ffi::c_long as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }
        let err_code_0 = HUF_initRemainingDStream(&mut bit, &mut args, i, segmentEnd);
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        let fresh47 = &mut (*(args.op).as_mut_ptr().offset(i as isize));
        *fresh47 = (*fresh47).offset(HUF_decodeStreamX1(
            *(args.op).as_mut_ptr().offset(i as isize),
            &mut bit,
            segmentEnd,
            dt as *const HUF_DEltX1,
            HUF_DECODER_FAST_TABLELOG as u32,
        ) as isize);
        if *(args.op).as_mut_ptr().offset(i as isize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        i += 1;
        i;
    }
    dstSize
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress1X1_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
        return HUF_decompress1X1_usingDTable_internal_bmi2(dst, dstSize, cSrc, cSrcSize, DTable);
    }
    HUF_decompress1X1_usingDTable_internal_default(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut fallbackFn: HUF_DecompressUsingDTableFn = Some(
        HUF_decompress4X1_usingDTable_internal_default
            as unsafe extern "C" fn(
                *mut std::ffi::c_void,
                size_t,
                *const std::ffi::c_void,
                size_t,
                *const HUF_DTable,
            ) -> size_t,
    );
    let mut loopFn: HUF_DecompressFastLoopFn = Some(
        HUF_decompress4X1_usingDTable_internal_fast_c_loop
            as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
    );
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
        fallbackFn = Some(
            HUF_decompress4X1_usingDTable_internal_bmi2
                as unsafe extern "C" fn(
                    *mut std::ffi::c_void,
                    size_t,
                    *const std::ffi::c_void,
                    size_t,
                    *const HUF_DTable,
                ) -> size_t,
        );
        if flags & HUF_flags_disableAsm as std::ffi::c_int == 0 {
            loopFn = Some(
                HUF_decompress4X1_usingDTable_internal_fast_asm_loop
                    as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
            );
        }
    } else {
        return fallbackFn.unwrap_unchecked()(dst, dstSize, cSrc, cSrcSize, DTable);
    }
    if HUF_ENABLE_FAST_DECODE != 0 && flags & HUF_flags_disableFast as std::ffi::c_int == 0 {
        let ret = HUF_decompress4X1_usingDTable_internal_fast(
            dst, dstSize, cSrc, cSrcSize, DTable, loopFn,
        );
        if ret != 0 as std::ffi::c_int as size_t {
            return ret;
        }
    }
    fallbackFn.unwrap_unchecked()(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe fn HUF_decompress4X1_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress4X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    )
}
unsafe extern "C" fn HUF_buildDEltX2U32(
    mut symbol: u32,
    mut nbBits: u32,
    mut baseSeq: u32,
    mut level: std::ffi::c_int,
) -> u32 {
    let mut seq: u32 = 0;
    if MEM_isLittleEndian() != 0 {
        seq = if level == 1 as std::ffi::c_int {
            symbol
        } else {
            baseSeq.wrapping_add(symbol << 8 as std::ffi::c_int)
        };
        seq.wrapping_add(nbBits << 16 as std::ffi::c_int)
            .wrapping_add((level as u32) << 24 as std::ffi::c_int)
    } else {
        seq = if level == 1 as std::ffi::c_int {
            symbol << 8 as std::ffi::c_int
        } else {
            (baseSeq << 8 as std::ffi::c_int).wrapping_add(symbol)
        };
        (seq << 16 as std::ffi::c_int)
            .wrapping_add(nbBits << 8 as std::ffi::c_int)
            .wrapping_add(level as u32)
    }
}
unsafe extern "C" fn HUF_buildDEltX2(
    mut symbol: u32,
    mut nbBits: u32,
    mut baseSeq: u32,
    mut level: std::ffi::c_int,
) -> HUF_DEltX2 {
    let mut DElt = HUF_DEltX2 {
        sequence: 0,
        nbBits: 0,
        length: 0,
    };
    let val = HUF_buildDEltX2U32(symbol, nbBits, baseSeq, level);
    libc::memcpy(
        &mut DElt as *mut HUF_DEltX2 as *mut std::ffi::c_void,
        &val as *const u32 as *const std::ffi::c_void,
        ::core::mem::size_of::<u32>() as std::ffi::c_ulong as libc::size_t,
    );
    DElt
}
unsafe extern "C" fn HUF_buildDEltX2U64(
    mut symbol: u32,
    mut nbBits: u32,
    mut baseSeq: u16,
    mut level: std::ffi::c_int,
) -> u64 {
    let mut DElt = HUF_buildDEltX2U32(symbol, nbBits, baseSeq as u32, level);
    (DElt as u64).wrapping_add((DElt as u64) << 32 as std::ffi::c_int)
}
unsafe extern "C" fn HUF_fillDTableX2ForWeight(
    mut DTableRank: *mut HUF_DEltX2,
    mut begin: *const sortedSymbol_t,
    mut end: *const sortedSymbol_t,
    mut nbBits: u32,
    mut tableLog: u32,
    mut baseSeq: u16,
    level: std::ffi::c_int,
) {
    let length =
        (1 as std::ffi::c_uint) << (tableLog.wrapping_sub(nbBits) & 0x1f as std::ffi::c_int as u32);
    let mut ptr = std::ptr::null::<sortedSymbol_t>();
    match length {
        1 => {
            ptr = begin;
            while ptr != end {
                let DElt = HUF_buildDEltX2((*ptr).symbol as u32, nbBits, baseSeq as u32, level);
                let fresh48 = DTableRank;
                DTableRank = DTableRank.offset(1);
                *fresh48 = DElt;
                ptr = ptr.offset(1);
                ptr;
            }
        }
        2 => {
            ptr = begin;
            while ptr != end {
                let DElt_0 = HUF_buildDEltX2((*ptr).symbol as u32, nbBits, baseSeq as u32, level);
                *DTableRank.offset(0 as std::ffi::c_int as isize) = DElt_0;
                *DTableRank.offset(1 as std::ffi::c_int as isize) = DElt_0;
                DTableRank = DTableRank.offset(2 as std::ffi::c_int as isize);
                ptr = ptr.offset(1);
                ptr;
            }
        }
        4 => {
            ptr = begin;
            while ptr != end {
                let DEltX2 = HUF_buildDEltX2U64((*ptr).symbol as u32, nbBits, baseSeq, level);
                libc::memcpy(
                    DTableRank.offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
                DTableRank = DTableRank.offset(4 as std::ffi::c_int as isize);
                ptr = ptr.offset(1);
                ptr;
            }
        }
        8 => {
            ptr = begin;
            while ptr != end {
                let DEltX2_0 = HUF_buildDEltX2U64((*ptr).symbol as u32, nbBits, baseSeq, level);
                libc::memcpy(
                    DTableRank.offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(4 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(6 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
                DTableRank = DTableRank.offset(8 as std::ffi::c_int as isize);
                ptr = ptr.offset(1);
                ptr;
            }
        }
        _ => {
            ptr = begin;
            while ptr != end {
                let DEltX2_1 = HUF_buildDEltX2U64((*ptr).symbol as u32, nbBits, baseSeq, level);
                let DTableRankEnd = DTableRank.offset(length as isize);
                while DTableRank != DTableRankEnd {
                    libc::memcpy(
                        DTableRank.offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(4 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(6 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    DTableRank = DTableRank.offset(8 as std::ffi::c_int as isize);
                }
                ptr = ptr.offset(1);
                ptr;
            }
        }
    };
}
unsafe extern "C" fn HUF_fillDTableX2Level2(
    mut DTable: *mut HUF_DEltX2,
    mut targetLog: u32,
    consumedBits: u32,
    mut rankVal: *const u32,
    minWeight: std::ffi::c_int,
    maxWeight1: std::ffi::c_int,
    mut sortedSymbols: *const sortedSymbol_t,
    mut rankStart: *const u32,
    mut nbBitsBaseline: u32,
    mut baseSeq: u16,
) {
    if minWeight > 1 as std::ffi::c_int {
        let length = (1 as std::ffi::c_uint)
            << (targetLog.wrapping_sub(consumedBits) & 0x1f as std::ffi::c_int as u32);
        let DEltX2 = HUF_buildDEltX2U64(
            baseSeq as u32,
            consumedBits,
            0 as std::ffi::c_int as u16,
            1 as std::ffi::c_int,
        );
        let skipSize = *rankVal.offset(minWeight as isize) as std::ffi::c_int;
        match length {
            2 => {
                libc::memcpy(
                    DTable as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
            }
            4 => {
                libc::memcpy(
                    DTable.offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTable.offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    &DEltX2 as *const u64 as *const std::ffi::c_void,
                    ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                );
            }
            _ => {
                let mut i: std::ffi::c_int = 0;
                i = 0 as std::ffi::c_int;
                while i < skipSize {
                    libc::memcpy(
                        DTable
                            .offset(i as isize)
                            .offset(0 as std::ffi::c_int as isize)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable
                            .offset(i as isize)
                            .offset(2 as std::ffi::c_int as isize)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable
                            .offset(i as isize)
                            .offset(4 as std::ffi::c_int as isize)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable
                            .offset(i as isize)
                            .offset(6 as std::ffi::c_int as isize)
                            as *mut std::ffi::c_void,
                        &DEltX2 as *const u64 as *const std::ffi::c_void,
                        ::core::mem::size_of::<u64>() as std::ffi::c_ulong as libc::size_t,
                    );
                    i += 8 as std::ffi::c_int;
                }
            }
        }
    }
    let mut w: std::ffi::c_int = 0;
    w = minWeight;
    while w < maxWeight1 {
        let begin = *rankStart.offset(w as isize) as std::ffi::c_int;
        let end = *rankStart.offset((w + 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
        let nbBits = nbBitsBaseline.wrapping_sub(w as u32);
        let totalBits = nbBits.wrapping_add(consumedBits);
        HUF_fillDTableX2ForWeight(
            DTable.offset(*rankVal.offset(w as isize) as isize),
            sortedSymbols.offset(begin as isize),
            sortedSymbols.offset(end as isize),
            totalBits,
            targetLog,
            baseSeq,
            2 as std::ffi::c_int,
        );
        w += 1;
        w;
    }
}
unsafe extern "C" fn HUF_fillDTableX2(
    mut DTable: *mut HUF_DEltX2,
    targetLog: u32,
    mut sortedList: *const sortedSymbol_t,
    mut rankStart: *const u32,
    mut rankValOrigin: *mut rankValCol_t,
    maxWeight: u32,
    nbBitsBaseline: u32,
) {
    let rankVal = (*rankValOrigin.offset(0 as std::ffi::c_int as isize)).as_mut_ptr();
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as std::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);
    let mut w: std::ffi::c_int = 0;
    let wEnd = maxWeight as std::ffi::c_int + 1 as std::ffi::c_int;
    w = 1 as std::ffi::c_int;
    while w < wEnd {
        let begin = *rankStart.offset(w as isize) as std::ffi::c_int;
        let end = *rankStart.offset((w + 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
        let nbBits = nbBitsBaseline.wrapping_sub(w as u32);
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut start = *rankVal.offset(w as isize) as std::ffi::c_int;
            let length = (1 as std::ffi::c_uint)
                << (targetLog.wrapping_sub(nbBits) & 0x1f as std::ffi::c_int as u32);
            let mut minWeight = nbBits.wrapping_add(scaleLog as u32) as std::ffi::c_int;
            let mut s: std::ffi::c_int = 0;
            if minWeight < 1 as std::ffi::c_int {
                minWeight = 1 as std::ffi::c_int;
            }
            s = begin;
            while s != end {
                HUF_fillDTableX2Level2(
                    DTable.offset(start as isize),
                    targetLog,
                    nbBits,
                    (*rankValOrigin.offset(nbBits as isize)).as_mut_ptr(),
                    minWeight,
                    wEnd,
                    sortedList,
                    rankStart,
                    nbBitsBaseline,
                    (*sortedList.offset(s as isize)).symbol as u16,
                );
                start = (start as u32).wrapping_add(length) as std::ffi::c_int as std::ffi::c_int;
                s += 1;
                s;
            }
        } else {
            HUF_fillDTableX2ForWeight(
                DTable.offset(*rankVal.offset(w as isize) as isize),
                sortedList.offset(begin as isize),
                sortedList.offset(end as isize),
                nbBits,
                targetLog,
                0 as std::ffi::c_int as u16,
                1 as std::ffi::c_int,
            );
        }
        w += 1;
        w;
    }
}
#[export_name = crate::prefix!(HUF_readDTableX2_wksp)]
pub unsafe extern "C" fn HUF_readDTableX2_wksp(
    mut DTable: *mut HUF_DTable,
    mut src: *const std::ffi::c_void,
    mut srcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut tableLog: u32 = 0;
    let mut maxW: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let mut dtd = HUF_getDTableDesc(DTable);
    let mut maxTableLog = dtd.maxTableLog as u32;
    let mut iSize: size_t = 0;
    let mut dtPtr = DTable.offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void;
    let dt = dtPtr as *mut HUF_DEltX2;
    let mut rankStart = std::ptr::null_mut::<u32>();
    let wksp = workSpace as *mut HUF_ReadDTableX2_Workspace;
    if ::core::mem::size_of::<HUF_ReadDTableX2_Workspace>() as std::ffi::c_ulong > wkspSize {
        return -(ZSTD_error_GENERIC as std::ffi::c_int) as size_t;
    }
    rankStart = core::ptr::addr_of_mut!((*wksp).rankStart0)
        .cast::<u32>()
        .offset(1 as std::ffi::c_int as isize);
    core::ptr::write_bytes(
        ((*wksp).rankStats).as_mut_ptr() as *mut u8,
        0,
        ::core::mem::size_of::<[u32; 13]>() as std::ffi::c_ulong as libc::size_t,
    );
    core::ptr::write_bytes(
        ((*wksp).rankStart0).as_mut_ptr() as *mut u8,
        0,
        ::core::mem::size_of::<[u32; 15]>() as std::ffi::c_ulong as libc::size_t,
    );
    if maxTableLog > HUF_TABLELOG_MAX as u32 {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    iSize = HUF_readStats_wksp(
        &mut (*wksp).weightList,
        (HUF_SYMBOLVALUE_MAX + 1 as std::ffi::c_int) as size_t,
        &mut (*wksp).rankStats,
        &mut nbSymbols,
        &mut tableLog,
        src,
        srcSize,
        &mut (*wksp).calleeWksp,
        flags,
    );
    if ERR_isError(iSize) != 0 {
        return iSize;
    }
    if tableLog > maxTableLog {
        return -(ZSTD_error_tableLog_tooLarge as std::ffi::c_int) as size_t;
    }
    if tableLog <= HUF_DECODER_FAST_TABLELOG as u32
        && maxTableLog > HUF_DECODER_FAST_TABLELOG as u32
    {
        maxTableLog = HUF_DECODER_FAST_TABLELOG as u32;
    }
    maxW = tableLog;
    while *((*wksp).rankStats).as_mut_ptr().offset(maxW as isize) == 0 as std::ffi::c_int as u32 {
        maxW = maxW.wrapping_sub(1);
        maxW;
    }
    let mut w: u32 = 0;
    let mut nextRankStart = 0 as std::ffi::c_int as u32;
    w = 1 as std::ffi::c_int as u32;
    while w < maxW.wrapping_add(1 as std::ffi::c_int as u32) {
        let mut curr = nextRankStart;
        nextRankStart =
            nextRankStart.wrapping_add(*((*wksp).rankStats).as_mut_ptr().offset(w as isize));
        *rankStart.offset(w as isize) = curr;
        w = w.wrapping_add(1);
        w;
    }
    *rankStart.offset(0 as std::ffi::c_int as isize) = nextRankStart;
    *rankStart.offset(maxW.wrapping_add(1 as std::ffi::c_int as u32) as isize) = nextRankStart;
    let mut s: u32 = 0;
    s = 0 as std::ffi::c_int as u32;
    while s < nbSymbols {
        let w_0 = *((*wksp).weightList).as_mut_ptr().offset(s as isize) as u32;
        let fresh49 = &mut (*rankStart.offset(w_0 as isize));
        let fresh50 = *fresh49;
        *fresh49 = (*fresh49).wrapping_add(1);
        let r = fresh50;
        (*((*wksp).sortedSymbol).as_mut_ptr().offset(r as isize)).symbol = s as u8;
        s = s.wrapping_add(1);
        s;
    }
    *rankStart.offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int as u32;
    let rankVal0 = (*((*wksp).rankVal)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize))
    .as_mut_ptr();
    let rescale = maxTableLog
        .wrapping_sub(tableLog)
        .wrapping_sub(1 as std::ffi::c_int as u32) as std::ffi::c_int;
    let mut nextRankVal = 0 as std::ffi::c_int as u32;
    let mut w_1: u32 = 0;
    w_1 = 1 as std::ffi::c_int as u32;
    while w_1 < maxW.wrapping_add(1 as std::ffi::c_int as u32) {
        let mut curr_0 = nextRankVal;
        nextRankVal = nextRankVal.wrapping_add(
            *((*wksp).rankStats).as_mut_ptr().offset(w_1 as isize)
                << w_1.wrapping_add(rescale as u32),
        );
        *rankVal0.offset(w_1 as isize) = curr_0;
        w_1 = w_1.wrapping_add(1);
        w_1;
    }
    let minBits = tableLog
        .wrapping_add(1 as std::ffi::c_int as u32)
        .wrapping_sub(maxW);
    let mut consumed: u32 = 0;
    consumed = minBits;
    while consumed
        < maxTableLog
            .wrapping_sub(minBits)
            .wrapping_add(1 as std::ffi::c_int as u32)
    {
        let rankValPtr = (*((*wksp).rankVal).as_mut_ptr().offset(consumed as isize)).as_mut_ptr();
        let mut w_2: u32 = 0;
        w_2 = 1 as std::ffi::c_int as u32;
        while w_2 < maxW.wrapping_add(1 as std::ffi::c_int as u32) {
            *rankValPtr.offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
            w_2;
        }
        consumed = consumed.wrapping_add(1);
        consumed;
    }
    HUF_fillDTableX2(
        dt,
        maxTableLog,
        ((*wksp).sortedSymbol).as_mut_ptr(),
        ((*wksp).rankStart0).as_mut_ptr(),
        ((*wksp).rankVal).as_mut_ptr(),
        maxW,
        tableLog.wrapping_add(1 as std::ffi::c_int as u32),
    );
    dtd.tableLog = maxTableLog as u8;
    dtd.tableType = 1 as std::ffi::c_int as u8;
    libc::memcpy(
        DTable as *mut std::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const std::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as std::ffi::c_ulong as libc::size_t,
    );
    iSize
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeSymbolX2(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX2,
    dtLog: u32,
) -> u32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    libc::memcpy(
        op,
        &(*dt.offset(val as isize)).sequence as *const u16 as *const std::ffi::c_void,
        2 as std::ffi::c_int as std::ffi::c_ulong as libc::size_t,
    );
    BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    (*dt.offset(val as isize)).length as u32
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeLastSymbolX2(
    mut op: *mut std::ffi::c_void,
    mut DStream: *mut BIT_DStream_t,
    mut dt: *const HUF_DEltX2,
    dtLog: u32,
) -> u32 {
    let val = BIT_lookBitsFast(DStream, dtLog);
    libc::memcpy(
        op,
        &(*dt.offset(val as isize)).sequence as *const u16 as *const std::ffi::c_void,
        1 as std::ffi::c_int as std::ffi::c_ulong as libc::size_t,
    );
    if (*dt.offset(val as isize)).length as std::ffi::c_int == 1 as std::ffi::c_int {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
    } else if ((*DStream).bitsConsumed as std::ffi::c_ulong)
        < (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
            .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
    {
        BIT_skipBits(DStream, (*dt.offset(val as isize)).nbBits as u32);
        if (*DStream).bitsConsumed as std::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
        {
            (*DStream).bitsConsumed = (::core::mem::size_of::<BitContainerType>()
                as std::ffi::c_ulong)
                .wrapping_mul(8 as std::ffi::c_int as std::ffi::c_ulong)
                as std::ffi::c_uint;
        }
    }
    1 as std::ffi::c_int as u32
}
#[inline(always)]
unsafe extern "C" fn HUF_decodeStreamX2(
    mut p: *mut u8,
    mut bitDPtr: *mut BIT_DStream_t,
    pEnd: *mut u8,
    dt: *const HUF_DEltX2,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) as std::ffi::c_long as size_t
        >= ::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong
    {
        if dtLog <= 11 as std::ffi::c_int as u32 && MEM_64bits() != 0 {
            while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int
                & (p < pEnd.offset(-(9 as std::ffi::c_int as isize))) as std::ffi::c_int
                != 0
            {
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
            }
        } else {
            while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int
                & (p < pEnd.offset(
                    -((::core::mem::size_of::<BitContainerType>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                        as isize),
                )) as std::ffi::c_int
                != 0
            {
                if MEM_64bits() != 0 {
                    p = p.offset(
                        HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                    );
                }
                if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                    p = p.offset(
                        HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                    );
                }
                if MEM_64bits() != 0 {
                    p = p.offset(
                        HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                    );
                }
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
            }
        }
    } else {
        BIT_reloadDStream(bitDPtr);
    }
    if pEnd.offset_from(p) as std::ffi::c_long as size_t >= 2 as std::ffi::c_int as size_t {
        while (BIT_reloadDStream(bitDPtr) as std::ffi::c_uint
            == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
            as std::ffi::c_int
            & (p <= pEnd.offset(-(2 as std::ffi::c_int as isize))) as std::ffi::c_int
            != 0
        {
            p = p.offset(
                HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        while p <= pEnd.offset(-(2 as std::ffi::c_int as isize)) {
            p = p.offset(
                HUF_decodeSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
    }
    if p < pEnd {
        p = p.offset(
            HUF_decodeLastSymbolX2(p as *mut std::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as std::ffi::c_long as size_t
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    let mut bitD = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let _var_err__ = BIT_initDStream(&mut bitD, cSrc, cSrcSize);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let ostart = dst as *mut u8;
    let oend =
        ZSTD_maybeNullPtrAdd(ostart as *mut std::ffi::c_void, dstSize as ptrdiff_t) as *mut u8;
    let dtPtr = DTable.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let dtd = HUF_getDTableDesc(DTable);
    HUF_decodeStreamX2(ostart, &mut bitD, oend, dt, dtd.tableLog as u32);
    if BIT_endOfDStream(&mut bitD) == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
#[inline(always)]
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_body(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    if cSrcSize < 10 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if dstSize < 6 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend.offset(
        -((::core::mem::size_of::<size_t>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong) as isize),
    );
    let dtPtr = DTable.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let mut bitD1 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD2 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD3 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let mut bitD4 = BIT_DStream_t {
        bitContainer: 0,
        bitsConsumed: 0,
        ptr: std::ptr::null::<std::ffi::c_char>(),
        start: std::ptr::null::<std::ffi::c_char>(),
        limitPtr: std::ptr::null::<std::ffi::c_char>(),
    };
    let length1 = MEM_readLE16(istart as *const std::ffi::c_void) as size_t;
    let length2 =
        MEM_readLE16(istart.offset(2 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length3 =
        MEM_readLE16(istart.offset(4 as std::ffi::c_int as isize) as *const std::ffi::c_void)
            as size_t;
    let length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6 as std::ffi::c_int as size_t),
    );
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
    let mut endSignal = 1 as std::ffi::c_int as u32;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if opStart4 > oend {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let _var_err__ = BIT_initDStream(&mut bitD1, istart1 as *const std::ffi::c_void, length1);
    if ERR_isError(_var_err__) != 0 {
        return _var_err__;
    }
    let _var_err___0 = BIT_initDStream(&mut bitD2, istart2 as *const std::ffi::c_void, length2);
    if ERR_isError(_var_err___0) != 0 {
        return _var_err___0;
    }
    let _var_err___1 = BIT_initDStream(&mut bitD3, istart3 as *const std::ffi::c_void, length3);
    if ERR_isError(_var_err___1) != 0 {
        return _var_err___1;
    }
    let _var_err___2 = BIT_initDStream(&mut bitD4, istart4 as *const std::ffi::c_void, length4);
    if ERR_isError(_var_err___2) != 0 {
        return _var_err___2;
    }
    if oend.offset_from(op4) as std::ffi::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as std::ffi::c_ulong
    {
        while endSignal & (op4 < olimit) as std::ffi::c_int as u32 != 0 {
            if MEM_64bits() != 0 {
                op1 = op1.offset(HUF_decodeSymbolX2(
                    op1 as *mut std::ffi::c_void,
                    &mut bitD1,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                op1 = op1.offset(HUF_decodeSymbolX2(
                    op1 as *mut std::ffi::c_void,
                    &mut bitD1,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op1 = op1.offset(HUF_decodeSymbolX2(
                    op1 as *mut std::ffi::c_void,
                    &mut bitD1,
                    dt,
                    dtLog,
                ) as isize);
            }
            op1 = op1.offset(
                HUF_decodeSymbolX2(op1 as *mut std::ffi::c_void, &mut bitD1, dt, dtLog) as isize,
            );
            if MEM_64bits() != 0 {
                op2 = op2.offset(HUF_decodeSymbolX2(
                    op2 as *mut std::ffi::c_void,
                    &mut bitD2,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                op2 = op2.offset(HUF_decodeSymbolX2(
                    op2 as *mut std::ffi::c_void,
                    &mut bitD2,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op2 = op2.offset(HUF_decodeSymbolX2(
                    op2 as *mut std::ffi::c_void,
                    &mut bitD2,
                    dt,
                    dtLog,
                ) as isize);
            }
            op2 = op2.offset(
                HUF_decodeSymbolX2(op2 as *mut std::ffi::c_void, &mut bitD2, dt, dtLog) as isize,
            );
            endSignal &= (BIT_reloadDStreamFast(&mut bitD1) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
            endSignal &= (BIT_reloadDStreamFast(&mut bitD2) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
            if MEM_64bits() != 0 {
                op3 = op3.offset(HUF_decodeSymbolX2(
                    op3 as *mut std::ffi::c_void,
                    &mut bitD3,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                op3 = op3.offset(HUF_decodeSymbolX2(
                    op3 as *mut std::ffi::c_void,
                    &mut bitD3,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op3 = op3.offset(HUF_decodeSymbolX2(
                    op3 as *mut std::ffi::c_void,
                    &mut bitD3,
                    dt,
                    dtLog,
                ) as isize);
            }
            op3 = op3.offset(
                HUF_decodeSymbolX2(op3 as *mut std::ffi::c_void, &mut bitD3, dt, dtLog) as isize,
            );
            if MEM_64bits() != 0 {
                op4 = op4.offset(HUF_decodeSymbolX2(
                    op4 as *mut std::ffi::c_void,
                    &mut bitD4,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 as std::ffi::c_int {
                op4 = op4.offset(HUF_decodeSymbolX2(
                    op4 as *mut std::ffi::c_void,
                    &mut bitD4,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op4 = op4.offset(HUF_decodeSymbolX2(
                    op4 as *mut std::ffi::c_void,
                    &mut bitD4,
                    dt,
                    dtLog,
                ) as isize);
            }
            op4 = op4.offset(
                HUF_decodeSymbolX2(op4 as *mut std::ffi::c_void, &mut bitD4, dt, dtLog) as isize,
            );
            endSignal &= (BIT_reloadDStreamFast(&mut bitD3) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
            endSignal &= (BIT_reloadDStreamFast(&mut bitD4) as std::ffi::c_uint
                == BIT_DStream_unfinished as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int as u32;
        }
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
    HUF_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);
    let endCheck = BIT_endOfDStream(&mut bitD1)
        & BIT_endOfDStream(&mut bitD2)
        & BIT_endOfDStream(&mut bitD3)
        & BIT_endOfDStream(&mut bitD4);
    if endCheck == 0 {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    dstSize
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_fast_c_loop(
    mut args: *mut HUF_DecompressFastArgs,
) {
    let mut bits: [u64; 4] = [0; 4];
    let mut ip: [*const u8; 4] = [std::ptr::null::<u8>(); 4];
    let mut op: [*mut u8; 4] = [std::ptr::null_mut::<u8>(); 4];
    let mut oend: [*mut u8; 4] = [std::ptr::null_mut::<u8>(); 4];
    let dtable = (*args).dt as *const HUF_DEltX2;
    let ilowest = (*args).ilowest;
    libc::memcpy(
        &mut bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut (*args).bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut (*args).ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut (*args).op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    let fresh51 = &mut (*oend.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
    *fresh51 = *op.as_mut_ptr().offset(1 as std::ffi::c_int as isize);
    let fresh52 = &mut (*oend.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
    *fresh52 = *op.as_mut_ptr().offset(2 as std::ffi::c_int as isize);
    let fresh53 = &mut (*oend.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
    *fresh53 = *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize);
    let fresh54 = &mut (*oend.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
    *fresh54 = (*args).oend;
    's_45: loop {
        let mut olimit = std::ptr::null_mut::<u8>();
        let mut stream: std::ffi::c_int = 0;
        stream = 0 as std::ffi::c_int;
        while stream < 4 as std::ffi::c_int {
            stream += 1;
            stream;
        }
        let mut iters = (*ip.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
            .offset_from(ilowest) as std::ffi::c_long as size_t
            / 7 as std::ffi::c_int as size_t;
        stream = 0 as std::ffi::c_int;
        while stream < 4 as std::ffi::c_int {
            let oiters = (*oend.as_mut_ptr().offset(stream as isize))
                .offset_from(*op.as_mut_ptr().offset(stream as isize))
                as std::ffi::c_long as size_t
                / 10 as std::ffi::c_int as size_t;
            iters = if iters < oiters { iters } else { oiters };
            stream += 1;
            stream;
        }
        olimit = (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
            .offset((iters * 5 as std::ffi::c_int as size_t) as isize);
        if *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) == olimit {
            break;
        }
        stream = 1 as std::ffi::c_int;
        while stream < 4 as std::ffi::c_int {
            if *ip.as_mut_ptr().offset(stream as isize)
                < *ip
                    .as_mut_ptr()
                    .offset((stream - 1 as std::ffi::c_int) as isize)
            {
                break 's_45;
            }
            stream += 1;
            stream;
        }
        stream = 1 as std::ffi::c_int;
        while stream < 4 as std::ffi::c_int {
            stream += 1;
            stream;
        }
        loop {
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry = *dtable.offset(index as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry.sequence,
                );
                *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                    entry.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh55 = &mut (*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
                *fresh55 = (*fresh55).offset(entry.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_0 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_0 = *dtable.offset(index_0 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_0.sequence,
                );
                *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                    entry_0.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh56 = &mut (*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
                *fresh56 = (*fresh56).offset(entry_0.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_1 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_1 = *dtable.offset(index_1 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_1.sequence,
                );
                *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                    entry_1.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh57 = &mut (*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
                *fresh57 = (*fresh57).offset(entry_1.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_2 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_2 = *dtable.offset(index_2 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_2.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_2.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh58 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh58 = (*fresh58).offset(entry_2.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_3 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_3 = *dtable.offset(index_3 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_3.sequence,
                );
                *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                    entry_3.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh59 = &mut (*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
                *fresh59 = (*fresh59).offset(entry_3.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_4 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_4 = *dtable.offset(index_4 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_4.sequence,
                );
                *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                    entry_4.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh60 = &mut (*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
                *fresh60 = (*fresh60).offset(entry_4.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_5 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_5 = *dtable.offset(index_5 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_5.sequence,
                );
                *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                    entry_5.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh61 = &mut (*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
                *fresh61 = (*fresh61).offset(entry_5.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_6 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_6 = *dtable.offset(index_6 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_6.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_6.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh62 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh62 = (*fresh62).offset(entry_6.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_7 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_7 = *dtable.offset(index_7 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_7.sequence,
                );
                *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                    entry_7.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh63 = &mut (*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
                *fresh63 = (*fresh63).offset(entry_7.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_8 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_8 = *dtable.offset(index_8 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_8.sequence,
                );
                *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                    entry_8.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh64 = &mut (*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
                *fresh64 = (*fresh64).offset(entry_8.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_9 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_9 = *dtable.offset(index_9 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_9.sequence,
                );
                *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                    entry_9.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh65 = &mut (*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
                *fresh65 = (*fresh65).offset(entry_9.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_10 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_10 = *dtable.offset(index_10 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_10.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_10.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh66 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh66 = (*fresh66).offset(entry_10.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_11 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_11 = *dtable.offset(index_11 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_11.sequence,
                );
                *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                    entry_11.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh67 = &mut (*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
                *fresh67 = (*fresh67).offset(entry_11.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_12 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_12 = *dtable.offset(index_12 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_12.sequence,
                );
                *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                    entry_12.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh68 = &mut (*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
                *fresh68 = (*fresh68).offset(entry_12.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_13 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_13 = *dtable.offset(index_13 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_13.sequence,
                );
                *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                    entry_13.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh69 = &mut (*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
                *fresh69 = (*fresh69).offset(entry_13.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_14 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_14 = *dtable.offset(index_14 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_14.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_14.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh70 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh70 = (*fresh70).offset(entry_14.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 0 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_15 = (*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_15 = *dtable.offset(index_15 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_15.sequence,
                );
                *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<=
                    entry_15.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh71 = &mut (*op.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
                *fresh71 = (*fresh71).offset(entry_15.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 1 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_16 = (*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_16 = *dtable.offset(index_16 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_16.sequence,
                );
                *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<=
                    entry_16.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh72 = &mut (*op.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
                *fresh72 = (*fresh72).offset(entry_16.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 2 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_17 = (*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_17 = *dtable.offset(index_17 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_17.sequence,
                );
                *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<=
                    entry_17.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh73 = &mut (*op.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
                *fresh73 = (*fresh73).offset(entry_17.length as std::ffi::c_int as isize);
            }
            if 0 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_18 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_18 = *dtable.offset(index_18 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_18.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_18.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh74 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh74 = (*fresh74).offset(entry_18.length as std::ffi::c_int as isize);
            }
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_19 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_19 = *dtable.offset(index_19 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_19.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_19.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh75 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh75 = (*fresh75).offset(entry_19.length as std::ffi::c_int as isize);
            }
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_20 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_20 = *dtable.offset(index_20 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_20.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_20.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh76 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh76 = (*fresh76).offset(entry_20.length as std::ffi::c_int as isize);
            }
            let ctz =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits = ctz & 7 as std::ffi::c_int;
            let nbBytes = ctz >> 3 as std::ffi::c_int;
            let fresh77 = &mut (*ip.as_mut_ptr().offset(0 as std::ffi::c_int as isize));
            *fresh77 = (*fresh77).offset(-(nbBytes as isize));
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(0 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(0 as std::ffi::c_int as isize) <<= nbBits;
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_21 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_21 = *dtable.offset(index_21 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_21.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_21.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh78 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh78 = (*fresh78).offset(entry_21.length as std::ffi::c_int as isize);
            }
            let ctz_0 =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits_0 = ctz_0 & 7 as std::ffi::c_int;
            let nbBytes_0 = ctz_0 >> 3 as std::ffi::c_int;
            let fresh79 = &mut (*ip.as_mut_ptr().offset(1 as std::ffi::c_int as isize));
            *fresh79 = (*fresh79).offset(-(nbBytes_0 as isize));
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(1 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(1 as std::ffi::c_int as isize) <<= nbBits_0;
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_22 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_22 = *dtable.offset(index_22 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_22.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_22.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh80 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh80 = (*fresh80).offset(entry_22.length as std::ffi::c_int as isize);
            }
            let ctz_1 =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits_1 = ctz_1 & 7 as std::ffi::c_int;
            let nbBytes_1 = ctz_1 >> 3 as std::ffi::c_int;
            let fresh81 = &mut (*ip.as_mut_ptr().offset(2 as std::ffi::c_int as isize));
            *fresh81 = (*fresh81).offset(-(nbBytes_1 as isize));
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(2 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(2 as std::ffi::c_int as isize) <<= nbBits_1;
            if 1 as std::ffi::c_int != 0 || 3 as std::ffi::c_int != 3 as std::ffi::c_int {
                let index_23 = (*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    >> 53 as std::ffi::c_int) as std::ffi::c_int;
                let entry_23 = *dtable.offset(index_23 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) as *mut std::ffi::c_void,
                    entry_23.sequence,
                );
                *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<=
                    entry_23.nbBits as std::ffi::c_int & 0x3f as std::ffi::c_int;
                let fresh82 = &mut (*op.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
                *fresh82 = (*fresh82).offset(entry_23.length as std::ffi::c_int as isize);
            }
            let ctz_2 =
                ZSTD_countTrailingZeros64(*bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize))
                    as std::ffi::c_int;
            let nbBits_2 = ctz_2 & 7 as std::ffi::c_int;
            let nbBytes_2 = ctz_2 >> 3 as std::ffi::c_int;
            let fresh83 = &mut (*ip.as_mut_ptr().offset(3 as std::ffi::c_int as isize));
            *fresh83 = (*fresh83).offset(-(nbBytes_2 as isize));
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) =
                MEM_read64(*ip.as_mut_ptr().offset(3 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void)
                    | 1 as std::ffi::c_int as u64;
            *bits.as_mut_ptr().offset(3 as std::ffi::c_int as isize) <<= nbBits_2;
            if *op.as_mut_ptr().offset(3 as std::ffi::c_int as isize) >= olimit {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [u64; 4] as *mut std::ffi::c_void,
        &mut bits as *mut [u64; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const u8; 4] as *mut std::ffi::c_void,
        &mut ip as *mut [*const u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut u8; 4] as *mut std::ffi::c_void,
        &mut op as *mut [*mut u8; 4] as *const std::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as std::ffi::c_ulong as libc::size_t,
    );
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_fast(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let mut dt = DTable.offset(1 as std::ffi::c_int as isize) as *const std::ffi::c_void;
    let ilowest = cSrc as *const u8;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    let mut args = HUF_DecompressFastArgs {
        ip: [std::ptr::null::<u8>(); 4],
        op: [std::ptr::null_mut::<u8>(); 4],
        bits: [0; 4],
        dt: std::ptr::null::<std::ffi::c_void>(),
        ilowest: std::ptr::null::<u8>(),
        oend: std::ptr::null_mut::<u8>(),
        iend: [std::ptr::null::<u8>(); 4],
    };
    let ret = HUF_DecompressFastArgs_init(&mut args, dst, dstSize, cSrc, cSrcSize, DTable);
    let err_code = ret;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if ret == 0 as std::ffi::c_int as size_t {
        return 0 as std::ffi::c_int as size_t;
    }
    loopFn.unwrap_unchecked()(&mut args);
    let segmentSize =
        dstSize.wrapping_add(3 as std::ffi::c_int as size_t) / 4 as std::ffi::c_int as size_t;
    let mut segmentEnd = dst as *mut u8;
    let mut i: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < 4 as std::ffi::c_int {
        let mut bit = BIT_DStream_t {
            bitContainer: 0,
            bitsConsumed: 0,
            ptr: std::ptr::null::<std::ffi::c_char>(),
            start: std::ptr::null::<std::ffi::c_char>(),
            limitPtr: std::ptr::null::<std::ffi::c_char>(),
        };
        if segmentSize <= oend.offset_from(segmentEnd) as std::ffi::c_long as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }
        let err_code_0 = HUF_initRemainingDStream(&mut bit, &mut args, i, segmentEnd);
        if ERR_isError(err_code_0) != 0 {
            return err_code_0;
        }
        let fresh84 = &mut (*(args.op).as_mut_ptr().offset(i as isize));
        *fresh84 = (*fresh84).offset(HUF_decodeStreamX2(
            *(args.op).as_mut_ptr().offset(i as isize),
            &mut bit,
            segmentEnd,
            dt as *const HUF_DEltX2,
            HUF_DECODER_FAST_TABLELOG as u32,
        ) as isize);
        if *(args.op).as_mut_ptr().offset(i as isize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
        }
        i += 1;
        i;
    }
    dstSize
}
unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut fallbackFn: HUF_DecompressUsingDTableFn = Some(
        HUF_decompress4X2_usingDTable_internal_default
            as unsafe extern "C" fn(
                *mut std::ffi::c_void,
                size_t,
                *const std::ffi::c_void,
                size_t,
                *const HUF_DTable,
            ) -> size_t,
    );
    let mut loopFn: HUF_DecompressFastLoopFn = Some(
        HUF_decompress4X2_usingDTable_internal_fast_c_loop
            as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
    );
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
        fallbackFn = Some(
            HUF_decompress4X2_usingDTable_internal_bmi2
                as unsafe extern "C" fn(
                    *mut std::ffi::c_void,
                    size_t,
                    *const std::ffi::c_void,
                    size_t,
                    *const HUF_DTable,
                ) -> size_t,
        );
        if flags & HUF_flags_disableAsm as std::ffi::c_int == 0 {
            loopFn = Some(
                HUF_decompress4X2_usingDTable_internal_fast_asm_loop
                    as unsafe extern "C" fn(*mut HUF_DecompressFastArgs) -> (),
            );
        }
    } else {
        return fallbackFn.unwrap_unchecked()(dst, dstSize, cSrc, cSrcSize, DTable);
    }
    if HUF_ENABLE_FAST_DECODE != 0 && flags & HUF_flags_disableFast as std::ffi::c_int == 0 {
        let ret = HUF_decompress4X2_usingDTable_internal_fast(
            dst, dstSize, cSrc, cSrcSize, DTable, loopFn,
        );
        if ret != 0 as std::ffi::c_int as size_t {
            return ret;
        }
    }
    fallbackFn.unwrap_unchecked()(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_bmi2(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal_default(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe extern "C" fn HUF_decompress1X2_usingDTable_internal(
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as std::ffi::c_int != 0 {
        return HUF_decompress1X2_usingDTable_internal_bmi2(dst, dstSize, cSrc, cSrcSize, DTable);
    }
    HUF_decompress1X2_usingDTable_internal_default(dst, dstSize, cSrc, cSrcSize, DTable)
}
#[export_name = crate::prefix!(HUF_decompress1X2_DCtx_wksp)]
pub unsafe extern "C" fn HUF_decompress1X2_DCtx_wksp(
    mut DCtx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX2_wksp(DCtx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress1X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        DCtx,
        flags,
    )
}
unsafe extern "C" fn HUF_decompress4X2_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let mut hSize = HUF_readDTableX2_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    )
}
static mut algoTime: [[algo_time_t; 2]; 16] = [
    [
        {
            algo_time_t {
                tableTime: 0 as std::ffi::c_int as u32,
                decode256Time: 0 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1 as std::ffi::c_int as u32,
                decode256Time: 1 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 0 as std::ffi::c_int as u32,
                decode256Time: 0 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1 as std::ffi::c_int as u32,
                decode256Time: 1 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 150 as std::ffi::c_int as u32,
                decode256Time: 216 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 381 as std::ffi::c_int as u32,
                decode256Time: 119 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 170 as std::ffi::c_int as u32,
                decode256Time: 205 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 514 as std::ffi::c_int as u32,
                decode256Time: 112 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 177 as std::ffi::c_int as u32,
                decode256Time: 199 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 539 as std::ffi::c_int as u32,
                decode256Time: 110 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 197 as std::ffi::c_int as u32,
                decode256Time: 194 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 644 as std::ffi::c_int as u32,
                decode256Time: 107 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 221 as std::ffi::c_int as u32,
                decode256Time: 192 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 735 as std::ffi::c_int as u32,
                decode256Time: 107 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 256 as std::ffi::c_int as u32,
                decode256Time: 189 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 881 as std::ffi::c_int as u32,
                decode256Time: 106 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 359 as std::ffi::c_int as u32,
                decode256Time: 188 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1167 as std::ffi::c_int as u32,
                decode256Time: 109 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 582 as std::ffi::c_int as u32,
                decode256Time: 187 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1570 as std::ffi::c_int as u32,
                decode256Time: 114 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 688 as std::ffi::c_int as u32,
                decode256Time: 187 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1712 as std::ffi::c_int as u32,
                decode256Time: 122 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 825 as std::ffi::c_int as u32,
                decode256Time: 186 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1965 as std::ffi::c_int as u32,
                decode256Time: 136 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 976 as std::ffi::c_int as u32,
                decode256Time: 185 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 2131 as std::ffi::c_int as u32,
                decode256Time: 150 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1180 as std::ffi::c_int as u32,
                decode256Time: 186 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 2070 as std::ffi::c_int as u32,
                decode256Time: 175 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1377 as std::ffi::c_int as u32,
                decode256Time: 185 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1731 as std::ffi::c_int as u32,
                decode256Time: 202 as std::ffi::c_int as u32,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1412 as std::ffi::c_int as u32,
                decode256Time: 185 as std::ffi::c_int as u32,
            }
        },
        {
            algo_time_t {
                tableTime: 1695 as std::ffi::c_int as u32,
                decode256Time: 202 as std::ffi::c_int as u32,
            }
        },
    ],
];
#[export_name = crate::prefix!(HUF_selectDecoder)]
pub unsafe extern "C" fn HUF_selectDecoder(mut dstSize: size_t, mut cSrcSize: size_t) -> u32 {
    let Q = if cSrcSize >= dstSize {
        15 as std::ffi::c_int as u32
    } else {
        (cSrcSize * 16 as std::ffi::c_int as size_t / dstSize) as u32
    };
    let D256 = (dstSize >> 8 as std::ffi::c_int) as u32;
    let DTime0 = ((*(*algoTime.as_ptr().offset(Q as isize))
        .as_ptr()
        .offset(0 as std::ffi::c_int as isize))
    .tableTime)
        .wrapping_add(
            (*(*algoTime.as_ptr().offset(Q as isize))
                .as_ptr()
                .offset(0 as std::ffi::c_int as isize))
            .decode256Time
                * D256,
        );
    let mut DTime1 = ((*(*algoTime.as_ptr().offset(Q as isize))
        .as_ptr()
        .offset(1 as std::ffi::c_int as isize))
    .tableTime)
        .wrapping_add(
            (*(*algoTime.as_ptr().offset(Q as isize))
                .as_ptr()
                .offset(1 as std::ffi::c_int as isize))
            .decode256Time
                * D256,
        );
    DTime1 = DTime1.wrapping_add(DTime1 >> 5 as std::ffi::c_int);
    (DTime1 < DTime0) as std::ffi::c_int as u32
}
#[export_name = crate::prefix!(HUF_decompress1X_DCtx_wksp)]
pub unsafe extern "C" fn HUF_decompress1X_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    if dstSize == 0 as std::ffi::c_int as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    if cSrcSize == dstSize {
        libc::memcpy(dst, cSrc, dstSize as libc::size_t);
        return dstSize;
    }
    if cSrcSize == 1 as std::ffi::c_int as size_t {
        libc::memset(
            dst,
            *(cSrc as *const u8) as std::ffi::c_int,
            dstSize as libc::size_t,
        );
        return dstSize;
    }
    let algoNb = HUF_selectDecoder(dstSize, cSrcSize);
    if algoNb != 0 {
        HUF_decompress1X2_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        )
    } else {
        HUF_decompress1X1_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        )
    }
}
#[export_name = crate::prefix!(HUF_decompress1X_usingDTable)]
pub unsafe extern "C" fn HUF_decompress1X_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> size_t {
    let dtd = HUF_getDTableDesc(DTable);
    if dtd.tableType as std::ffi::c_int != 0 {
        HUF_decompress1X2_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    } else {
        HUF_decompress1X1_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    }
}
#[export_name = crate::prefix!(HUF_decompress1X1_DCtx_wksp)]
pub unsafe extern "C" fn HUF_decompress1X1_DCtx_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as std::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress1X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const std::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    )
}
#[export_name = crate::prefix!(HUF_decompress4X_usingDTable)]
pub unsafe extern "C" fn HUF_decompress4X_usingDTable(
    mut dst: *mut std::ffi::c_void,
    mut maxDstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut DTable: *const HUF_DTable,
    mut flags: std::ffi::c_int,
) -> size_t {
    let dtd = HUF_getDTableDesc(DTable);
    if dtd.tableType as std::ffi::c_int != 0 {
        HUF_decompress4X2_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    } else {
        HUF_decompress4X1_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    }
}

pub unsafe fn HUF_decompress4X_hufOnly_wksp(
    mut dctx: *mut HUF_DTable,
    mut dst: *mut std::ffi::c_void,
    mut dstSize: size_t,
    mut cSrc: *const std::ffi::c_void,
    mut cSrcSize: size_t,
    mut workSpace: *mut std::ffi::c_void,
    mut wkspSize: size_t,
    mut flags: std::ffi::c_int,
) -> size_t {
    if dstSize == 0 as std::ffi::c_int as size_t {
        return -(ZSTD_error_dstSize_tooSmall as std::ffi::c_int) as size_t;
    }
    if cSrcSize == 0 as std::ffi::c_int as size_t {
        return -(ZSTD_error_corruption_detected as std::ffi::c_int) as size_t;
    }
    let algoNb = HUF_selectDecoder(dstSize, cSrcSize);
    if algoNb != 0 {
        HUF_decompress4X2_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        )
    } else {
        HUF_decompress4X1_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        )
    }
}
