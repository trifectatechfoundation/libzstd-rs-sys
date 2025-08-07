use core::ptr;

use crate::lib::common::bitstream::{BIT_DStream_t, BitContainerType, StreamStatus};
use crate::lib::common::entropy_common::{HUF_readStats_wksp, Workspace};
use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::mem::{
    MEM_32bits, MEM_64bits, MEM_isLittleEndian, MEM_read64, MEM_readLE16, MEM_readLEST,
    MEM_write16, MEM_write64,
};
use crate::lib::zstd::*;
extern "C" {
    fn HUF_decompress4X1_usingDTable_internal_fast_asm_loop(args: &mut HUF_DecompressFastArgs);
    fn HUF_decompress4X2_usingDTable_internal_fast_asm_loop(args: &mut HUF_DecompressFastArgs);
}
pub type ptrdiff_t = core::ffi::c_long;
pub type size_t = core::ffi::c_ulong;
pub type HUF_DTable = u32;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DecompressFastArgs {
    pub ip: [*const u8; 4],
    pub op: [*mut u8; 4],
    pub bits: [u64; 4],
    pub dt: *const core::ffi::c_void,
    pub ilowest: *const u8,
    pub oend: *mut u8,
    pub iend: [*const u8; 4],
}

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

pub const HUF_TABLELOG_MAX: core::ffi::c_int = 12;
pub const HUF_SYMBOLVALUE_MAX: core::ffi::c_int = 255;
pub const HUF_DECODER_FAST_TABLELOG: core::ffi::c_int = 11;
pub const HUF_ENABLE_FAST_DECODE: core::ffi::c_int = 1;
pub const HUF_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;

unsafe fn HUF_getDTableDesc(mut table: *const HUF_DTable) -> DTableDesc {
    let mut dtd = DTableDesc {
        maxTableLog: 0,
        tableType: 0,
        tableLog: 0,
        reserved: 0,
    };
    libc::memcpy(
        &mut dtd as *mut DTableDesc as *mut core::ffi::c_void,
        table as *const core::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as core::ffi::c_ulong as libc::size_t,
    );
    dtd
}

unsafe fn HUF_initFastDStream(mut ip: *const u8) -> size_t {
    let lastByte = *ip.offset(7);
    let bitsConsumed = (if lastByte as core::ffi::c_int != 0 {
        (8 as core::ffi::c_int as core::ffi::c_uint).wrapping_sub(lastByte.ilog2())
    } else {
        0
    }) as size_t;
    let value = MEM_readLEST(ip as *const core::ffi::c_void) | 1;
    value << bitsConsumed
}

unsafe fn HUF_DecompressFastArgs_init(
    args: &mut HUF_DecompressFastArgs,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    let mut dt = DTable.offset(1) as *const core::ffi::c_void;
    let dtLog = (HUF_getDTableDesc(DTable)).tableLog as u32;
    let istart = src as *const u8;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;

    // The fast decoding loop assumes 64-bit little-endian.
    if cfg!(target_endian = "big") || MEM_32bits() != 0 {
        return 0;
    }

    // Avoid nullptr addition
    if dstSize == 0 {
        return 0;
    }
    assert!(!dst.is_null());

    // strict minimum : jump table + 1 byte per stream.
    if srcSize < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    // Must have at least 8 bytes per stream because we don't handle initializing smaller bit containers.
    // If table log is not correct at this point, fallback to the old decoder.
    // On small inputs we don't have enough data to trigger the fast loop, so use the old decoder.
    if dtLog != HUF_DECODER_FAST_TABLELOG as u32 {
        return 0;
    }

    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const core::ffi::c_void) as size_t;
    let length4 = srcSize.wrapping_sub(length1 + length2 + length3 + 6);

    args.iend[0] = istart.add(6); /* jumpTable */
    args.iend[1] = args.iend[0].add(length1 as usize);
    args.iend[2] = args.iend[1].add(length2 as usize);
    args.iend[3] = args.iend[2].add(length3 as usize);

    // HUF_initFastDStream() requires this, and this small of an input won't benefit from the ASM loop anyways.
    if length1 < 8 || length2 < 8 || length3 < 8 || length4 < 8 {
        return 0;
    }

    if length4 > srcSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    /* ip[] contains the position that is currently loaded into bits[]. */
    args.ip[0] = args.iend[1].sub(size_of::<u64>());
    args.ip[1] = args.iend[2].sub(size_of::<u64>());
    args.ip[2] = args.iend[3].sub(size_of::<u64>());
    args.ip[3] = src.cast::<u8>().add(srcSize as usize - size_of::<u64>());

    /* op[] contains the output pointers. */
    args.op[0] = dst.cast::<u8>();
    args.op[1] = args.op[0].add(dstSize.div_ceil(4) as usize);
    args.op[2] = args.op[1].add(dstSize.div_ceil(4) as usize);
    args.op[3] = args.op[2].add(dstSize.div_ceil(4) as usize);

    // No point to call the ASM loop for tiny outputs.
    if *(args.op).as_mut_ptr().offset(3) >= oend {
        return 0;
    }

    // bits[] is the bit container.
    //
    // It is read from the MSB down to the LSB.
    // It is shifted left as it is read, and zeros are
    // shifted in. After the lowest valid bit a 1 is
    // set, so that CountTrailingZeros(bits[]) can be used
    // to count how many bits we've consumed.
    args.bits[0] = HUF_initFastDStream(args.ip[0]);
    args.bits[1] = HUF_initFastDStream(args.ip[1]);
    args.bits[2] = HUF_initFastDStream(args.ip[2]);
    args.bits[3] = HUF_initFastDStream(args.ip[3]);

    // The decoders must be sure to never read beyond ilowest.
    // This is lower than iend[0], but allowing decoders to read
    // down to ilowest can allow an extra iteration or two in the
    // fast loop.
    args.ilowest = istart;

    args.oend = oend;
    args.dt = dt;

    1
}

unsafe fn init_remaining_dstream(
    args: &HUF_DecompressFastArgs,
    stream: core::ffi::c_int,
    segmentEnd: *mut u8,
) -> Result<BIT_DStream_t, size_t> {
    if *(args.op).as_ptr().offset(stream as isize) > segmentEnd {
        return Err(-(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t);
    }
    if *(args.ip).as_ptr().offset(stream as isize)
        < (*(args.iend).as_ptr().offset(stream as isize)).sub(8)
    {
        return Err(-(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t);
    }

    let bitContainer =
        MEM_readLEST(*(args.ip).as_ptr().offset(stream as isize) as *const core::ffi::c_void)
            as usize;
    let bitsConsumed = (*args.bits.as_ptr().offset(stream as isize)).trailing_zeros();
    let start = args.ilowest as *const core::ffi::c_char;
    let limitPtr = start.add(::core::mem::size_of::<size_t>());
    let ptr = *(args.ip).as_ptr().offset(stream as isize) as *const core::ffi::c_char;

    Ok(BIT_DStream_t {
        bitContainer,
        bitsConsumed,
        ptr,
        start,
        limitPtr,
    })
}

fn HUF_DEltX1_set4(mut symbol: u8, mut nbBits: u8) -> u64 {
    let mut D4: u64 = 0;
    if MEM_isLittleEndian() != 0 {
        D4 = (((symbol as core::ffi::c_int) << 8) + nbBits as core::ffi::c_int) as u64;
    } else {
        D4 = (symbol as core::ffi::c_int + ((nbBits as core::ffi::c_int) << 8)) as u64;
    }
    D4 = (D4 as core::ffi::c_ulonglong).wrapping_mul(0x1000100010001 as core::ffi::c_ulonglong)
        as u64 as u64;
    D4
}

unsafe fn HUF_rescaleStats(
    huffWeight: *mut u8,
    rankVal: *mut u32,
    nbSymbols: u32,
    tableLog: u32,
    targetTableLog: u32,
) -> u32 {
    if tableLog > targetTableLog {
        return tableLog;
    }
    if tableLog < targetTableLog {
        let scale = targetTableLog.wrapping_sub(tableLog);
        let mut s: u32 = 0;
        s = 0;
        while s < nbSymbols {
            let fresh12 = &mut (*huffWeight.offset(s as isize));
            *fresh12 = (*fresh12 as core::ffi::c_int
                + (if *huffWeight.offset(s as isize) as core::ffi::c_int == 0 {
                    0
                } else {
                    scale
                }) as u8 as core::ffi::c_int) as u8;
            s = s.wrapping_add(1);
        }
        s = targetTableLog;
        while s > scale {
            *rankVal.offset(s as isize) = *rankVal.offset(s.wrapping_sub(scale) as isize);
            s = s.wrapping_sub(1);
        }
        s = scale;
        while s > 0 {
            *rankVal.offset(s as isize) = 0;
            s = s.wrapping_sub(1);
        }
    }
    targetTableLog
}

pub unsafe fn HUF_readDTableX1_wksp(
    DTable: *mut HUF_DTable,
    src: *const core::ffi::c_void,
    srcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    let mut tableLog = 0;
    let mut nbSymbols = 0;
    let mut iSize: size_t = 0;
    let dtPtr = DTable.offset(1) as *mut core::ffi::c_void;
    let dt = dtPtr as *mut HUF_DEltX1;
    let mut wksp = workSpace as *mut HUF_ReadDTableX1_Workspace;
    if ::core::mem::size_of::<HUF_ReadDTableX1_Workspace>() as core::ffi::c_ulong > wkspSize {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    iSize = HUF_readStats_wksp(
        &mut (*wksp).huffWeight,
        (HUF_SYMBOLVALUE_MAX + 1) as size_t,
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
    let maxTableLog = (dtd.maxTableLog as core::ffi::c_int + 1) as u32;
    let targetTableLog = if maxTableLog < 11 { maxTableLog } else { 11 };
    tableLog = HUF_rescaleStats(
        ((*wksp).huffWeight).as_mut_ptr(),
        ((*wksp).rankVal).as_mut_ptr(),
        nbSymbols,
        tableLog,
        targetTableLog,
    );
    if tableLog > (dtd.maxTableLog as core::ffi::c_int + 1) as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    dtd.tableType = 0;
    dtd.tableLog = tableLog as u8;
    libc::memcpy(
        DTable as *mut core::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const core::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as core::ffi::c_ulong as libc::size_t,
    );
    let mut n: core::ffi::c_int = 0;
    let mut nextRankStart = 0 as core::ffi::c_int as u32;
    let unroll = 4;
    let nLimit = nbSymbols as core::ffi::c_int - unroll + 1;
    n = 0;
    while n < tableLog as core::ffi::c_int + 1 {
        let curr = nextRankStart;
        nextRankStart =
            nextRankStart.wrapping_add(*((*wksp).rankVal).as_mut_ptr().offset(n as isize));
        *((*wksp).rankStart).as_mut_ptr().offset(n as isize) = curr;
        n += 1;
    }
    n = 0;
    while n < nLimit {
        let mut u: core::ffi::c_int = 0;
        u = 0;
        while u < unroll {
            let w = *((*wksp).huffWeight).as_mut_ptr().offset((n + u) as isize) as size_t;
            let fresh13 = &mut (*((*wksp).rankStart).as_mut_ptr().offset(w as isize));
            let fresh14 = *fresh13;
            *fresh13 = (*fresh13).wrapping_add(1);
            *((*wksp).symbols).as_mut_ptr().offset(fresh14 as isize) = (n + u) as u8;
            u += 1;
        }
        n += unroll;
    }
    while n < nbSymbols as core::ffi::c_int {
        let w_0 = *((*wksp).huffWeight).as_mut_ptr().offset(n as isize) as size_t;
        let fresh15 = &mut (*((*wksp).rankStart).as_mut_ptr().offset(w_0 as isize));
        let fresh16 = *fresh15;
        *fresh15 = (*fresh15).wrapping_add(1);
        *((*wksp).symbols).as_mut_ptr().offset(fresh16 as isize) = n as u8;
        n += 1;
    }
    let mut w_1: u32 = 0;
    let mut symbol = *((*wksp).rankVal).as_mut_ptr().offset(0) as core::ffi::c_int;
    let mut rankStart = 0;
    w_1 = 1;
    while w_1 < tableLog.wrapping_add(1) {
        let symbolCount = *((*wksp).rankVal).as_mut_ptr().offset(w_1 as isize) as core::ffi::c_int;
        let length = (1) << w_1 >> 1;
        let mut uStart = rankStart;
        let nbBits = tableLog.wrapping_add(1).wrapping_sub(w_1) as u8;
        let mut s: core::ffi::c_int = 0;
        let mut u_0: core::ffi::c_int = 0;
        match length {
            1 => {
                s = 0;
                while s < symbolCount {
                    let mut D = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D.byte = *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize);
                    D.nbBits = nbBits;
                    *dt.offset(uStart as isize) = D;
                    uStart += 1;
                    s += 1;
                }
            }
            2 => {
                s = 0;
                while s < symbolCount {
                    let mut D_0 = HUF_DEltX1 { nbBits: 0, byte: 0 };
                    D_0.byte = *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize);
                    D_0.nbBits = nbBits;
                    *dt.offset((uStart + 0) as isize) = D_0;
                    *dt.offset((uStart + 1) as isize) = D_0;
                    uStart += 2;
                    s += 1;
                }
            }
            4 => {
                s = 0;
                while s < symbolCount {
                    let D4 = HUF_DEltX1_set4(
                        *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize),
                        nbBits,
                    );
                    MEM_write64(dt.offset(uStart as isize) as *mut core::ffi::c_void, D4);
                    uStart += 4;
                    s += 1;
                }
            }
            8 => {
                s = 0;
                while s < symbolCount {
                    let D4_0 = HUF_DEltX1_set4(
                        *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize),
                        nbBits,
                    );
                    MEM_write64(dt.offset(uStart as isize) as *mut core::ffi::c_void, D4_0);
                    MEM_write64(
                        dt.offset(uStart as isize).offset(4) as *mut core::ffi::c_void,
                        D4_0,
                    );
                    uStart += 8;
                    s += 1;
                }
            }
            _ => {
                s = 0;
                while s < symbolCount {
                    let D4_1 = HUF_DEltX1_set4(
                        *((*wksp).symbols).as_mut_ptr().offset((symbol + s) as isize),
                        nbBits,
                    );
                    u_0 = 0;
                    while u_0 < length {
                        MEM_write64(
                            dt.offset(uStart as isize).offset(u_0 as isize).offset(0)
                                as *mut core::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt.offset(uStart as isize).offset(u_0 as isize).offset(4)
                                as *mut core::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt.offset(uStart as isize).offset(u_0 as isize).offset(8)
                                as *mut core::ffi::c_void,
                            D4_1,
                        );
                        MEM_write64(
                            dt.offset(uStart as isize).offset(u_0 as isize).offset(12)
                                as *mut core::ffi::c_void,
                            D4_1,
                        );
                        u_0 += 16;
                    }
                    uStart += length;
                    s += 1;
                }
            }
        }
        symbol += symbolCount;
        rankStart += symbolCount * length;
        w_1 = w_1.wrapping_add(1);
    }
    iSize
}

#[inline(always)]
unsafe fn HUF_decodeSymbolX1(Dstream: &mut BIT_DStream_t, dt: *const HUF_DEltX1, dtLog: u32) -> u8 {
    let val = Dstream.look_bits_fast(dtLog);
    let c = (*dt.offset(val as isize)).byte;
    Dstream.skip_bits((*dt.offset(val as isize)).nbBits as u32);
    c
}

#[inline(always)]
unsafe fn HUF_decodeStreamX1(
    mut p: *mut u8,
    bitDPtr: &mut BIT_DStream_t,
    pEnd: *mut u8,
    dt: *const HUF_DEltX1,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) as core::ffi::c_long > 3 {
        while (bitDPtr.reload() == StreamStatus::Unfinished) as core::ffi::c_int
            & (p < pEnd.offset(-(3))) as core::ffi::c_int
            != 0
        {
            if MEM_64bits() != 0 {
                let fresh17 = p;
                p = p.offset(1);
                *fresh17 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
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
        bitDPtr.reload();
    }
    if MEM_32bits() != 0 {
        while (bitDPtr.reload() == StreamStatus::Unfinished) as core::ffi::c_int
            & (p < pEnd) as core::ffi::c_int
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
    pEnd.offset_from(pStart) as core::ffi::c_long as size_t
}

#[inline(always)]
unsafe fn HUF_decompress1X1_usingDTable_internal_body(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    let mut op = dst as *mut u8;
    let oend = ZSTD_maybeNullPtrAdd(op as *mut core::ffi::c_void, dstSize as ptrdiff_t) as *mut u8;
    let mut dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;

    let src = core::slice::from_raw_parts(cSrc.cast::<u8>(), cSrcSize as usize);
    let mut bitD = match BIT_DStream_t::new(src) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    HUF_decodeStreamX1(op, &mut bitD, oend, dt, dtLog);

    if !bitD.is_empty() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    dstSize
}
#[inline(always)]
unsafe fn HUF_decompress4X1_usingDTable_internal_body(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if dstSize < 6 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend.offset(-(3));
    let dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX1;
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const core::ffi::c_void) as size_t;
    let length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    let istart1 = istart.offset(6);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    let mut endSignal = 1;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if opStart4 > oend {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let mut bitD1 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart1, length1 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD2 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart2, length2 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD3 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart3, length3 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD4 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart4, length4 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    if oend.offset_from(op4) as core::ffi::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as core::ffi::c_ulong
    {
        while endSignal & (op4 < olimit) as core::ffi::c_int as u32 != 0 {
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
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                let fresh27 = op1;
                op1 = op1.offset(1);
                *fresh27 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                let fresh28 = op2;
                op2 = op2.offset(1);
                *fresh28 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                let fresh29 = op3;
                op3 = op3.offset(1);
                *fresh29 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
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
            endSignal &= (bitD1.reload_fast() == StreamStatus::Unfinished) as u32;
            endSignal &= (bitD2.reload_fast() == StreamStatus::Unfinished) as u32;
            endSignal &= (bitD3.reload_fast() == StreamStatus::Unfinished) as u32;
            endSignal &= (bitD4.reload_fast() == StreamStatus::Unfinished) as u32;
        }
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    HUF_decodeStreamX1(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX1(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX1(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX1(op4, &mut bitD4, oend, dt, dtLog);

    if !(bitD1.is_empty() && bitD2.is_empty() && bitD3.is_empty() && bitD4.is_empty()) {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    dstSize
}

unsafe fn HUF_decompress4X1_usingDTable_internal_bmi2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe fn HUF_decompress4X1_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast_c_loop(
    args: &mut HUF_DecompressFastArgs,
) {
    let mut bits: [u64; 4] = [0; 4];
    let mut ip: [*const u8; 4] = [core::ptr::null::<u8>(); 4];
    let mut op: [*mut u8; 4] = [core::ptr::null_mut::<u8>(); 4];
    let dtable = (*args).dt as *const u16;
    let oend = (*args).oend;
    let ilowest = (*args).ilowest;
    libc::memcpy(
        &mut bits as *mut [u64; 4] as *mut core::ffi::c_void,
        &mut (*args).bits as *mut [u64; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut ip as *mut [*const u8; 4] as *mut core::ffi::c_void,
        &mut (*args).ip as *mut [*const u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut op as *mut [*mut u8; 4] as *mut core::ffi::c_void,
        &mut (*args).op as *mut [*mut u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    's_33: loop {
        let mut olimit = core::ptr::null_mut::<u8>();
        let mut stream: core::ffi::c_int = 0;
        stream = 0;
        while stream < 4 {
            stream += 1;
        }
        let oiters =
            oend.offset_from(*op.as_mut_ptr().offset(3)) as core::ffi::c_long as size_t / 5;
        let iiters =
            (*ip.as_mut_ptr().offset(0)).offset_from(ilowest) as core::ffi::c_long as size_t / 7;
        let iters = if oiters < iiters { oiters } else { iiters };
        let symbols = iters * 5;
        olimit = (*op.as_mut_ptr().offset(3)).offset(symbols as isize);
        if *op.as_mut_ptr().offset(3) == olimit {
            break;
        }
        stream = 1;
        while stream < 4 {
            if *ip.as_mut_ptr().offset(stream as isize)
                < *ip.as_mut_ptr().offset((stream - 1) as isize)
            {
                break 's_33;
            }
            stream += 1;
        }
        stream = 1;
        while stream < 4 {
            stream += 1;
        }
        loop {
            let index = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
            let entry = *dtable.offset(index as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(0) <<= entry & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(0)).offset(0) = (entry >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_0 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
            let entry_0 = *dtable.offset(index_0 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(1) <<= entry_0 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(1)).offset(0) =
                (entry_0 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_1 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
            let entry_1 = *dtable.offset(index_1 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(2) <<= entry_1 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(2)).offset(0) =
                (entry_1 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_2 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
            let entry_2 = *dtable.offset(index_2 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(3) <<= entry_2 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(3)).offset(0) =
                (entry_2 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_3 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
            let entry_3 = *dtable.offset(index_3 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(0) <<= entry_3 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(0)).offset(1) =
                (entry_3 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_4 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
            let entry_4 = *dtable.offset(index_4 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(1) <<= entry_4 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(1)).offset(1) =
                (entry_4 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_5 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
            let entry_5 = *dtable.offset(index_5 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(2) <<= entry_5 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(2)).offset(1) =
                (entry_5 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_6 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
            let entry_6 = *dtable.offset(index_6 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(3) <<= entry_6 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(3)).offset(1) =
                (entry_6 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_7 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
            let entry_7 = *dtable.offset(index_7 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(0) <<= entry_7 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(0)).offset(2) =
                (entry_7 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_8 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
            let entry_8 = *dtable.offset(index_8 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(1) <<= entry_8 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(1)).offset(2) =
                (entry_8 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_9 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
            let entry_9 = *dtable.offset(index_9 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(2) <<= entry_9 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(2)).offset(2) =
                (entry_9 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_10 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
            let entry_10 = *dtable.offset(index_10 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(3) <<= entry_10 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(3)).offset(2) =
                (entry_10 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_11 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
            let entry_11 = *dtable.offset(index_11 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(0) <<= entry_11 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(0)).offset(3) =
                (entry_11 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_12 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
            let entry_12 = *dtable.offset(index_12 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(1) <<= entry_12 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(1)).offset(3) =
                (entry_12 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_13 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
            let entry_13 = *dtable.offset(index_13 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(2) <<= entry_13 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(2)).offset(3) =
                (entry_13 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_14 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
            let entry_14 = *dtable.offset(index_14 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(3) <<= entry_14 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(3)).offset(3) =
                (entry_14 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_15 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
            let entry_15 = *dtable.offset(index_15 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(0) <<= entry_15 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(0)).offset(4) =
                (entry_15 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_16 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
            let entry_16 = *dtable.offset(index_16 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(1) <<= entry_16 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(1)).offset(4) =
                (entry_16 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_17 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
            let entry_17 = *dtable.offset(index_17 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(2) <<= entry_17 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(2)).offset(4) =
                (entry_17 >> 8 & 0xff as core::ffi::c_int) as u8;
            let index_18 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
            let entry_18 = *dtable.offset(index_18 as isize) as core::ffi::c_int;
            *bits.as_mut_ptr().offset(3) <<= entry_18 & 0x3f as core::ffi::c_int;
            *(*op.as_mut_ptr().offset(3)).offset(4) =
                (entry_18 >> 8 & 0xff as core::ffi::c_int) as u8;
            let ctz = {
                let mut val = *bits.as_mut_ptr().offset(0);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits = ctz & 7;
            let nbBytes = ctz >> 3;
            let fresh39 = &mut (*op.as_mut_ptr().offset(0));
            *fresh39 = (*fresh39).offset(5);
            let fresh40 = &mut (*ip.as_mut_ptr().offset(0));
            *fresh40 = (*fresh40).offset(-(nbBytes as isize));
            *bits.as_mut_ptr().offset(0) =
                MEM_read64(*ip.as_mut_ptr().offset(0) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(0) <<= nbBits;
            let ctz_0 = {
                let mut val = *bits.as_mut_ptr().offset(1);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits_0 = ctz_0 & 7;
            let nbBytes_0 = ctz_0 >> 3;
            let fresh41 = &mut (*op.as_mut_ptr().offset(1));
            *fresh41 = (*fresh41).offset(5);
            let fresh42 = &mut (*ip.as_mut_ptr().offset(1));
            *fresh42 = (*fresh42).offset(-(nbBytes_0 as isize));
            *bits.as_mut_ptr().offset(1) =
                MEM_read64(*ip.as_mut_ptr().offset(1) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(1) <<= nbBits_0;
            let ctz_1 = {
                let mut val = *bits.as_mut_ptr().offset(2);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits_1 = ctz_1 & 7;
            let nbBytes_1 = ctz_1 >> 3;
            let fresh43 = &mut (*op.as_mut_ptr().offset(2));
            *fresh43 = (*fresh43).offset(5);
            let fresh44 = &mut (*ip.as_mut_ptr().offset(2));
            *fresh44 = (*fresh44).offset(-(nbBytes_1 as isize));
            *bits.as_mut_ptr().offset(2) =
                MEM_read64(*ip.as_mut_ptr().offset(2) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(2) <<= nbBits_1;
            let ctz_2 = {
                let mut val = *bits.as_mut_ptr().offset(3);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits_2 = ctz_2 & 7;
            let nbBytes_2 = ctz_2 >> 3;
            let fresh45 = &mut (*op.as_mut_ptr().offset(3));
            *fresh45 = (*fresh45).offset(5);
            let fresh46 = &mut (*ip.as_mut_ptr().offset(3));
            *fresh46 = (*fresh46).offset(-(nbBytes_2 as isize));
            *bits.as_mut_ptr().offset(3) =
                MEM_read64(*ip.as_mut_ptr().offset(3) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(3) <<= nbBits_2;
            if *op.as_mut_ptr().offset(3) >= olimit {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [u64; 4] as *mut core::ffi::c_void,
        &mut bits as *mut [u64; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const u8; 4] as *mut core::ffi::c_void,
        &mut ip as *mut [*const u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut u8; 4] as *mut core::ffi::c_void,
        &mut op as *mut [*mut u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
}

pub type HUF_DecompressFastLoopFn = unsafe extern "C" fn(&mut HUF_DecompressFastArgs) -> ();
unsafe fn HUF_decompress4X1_usingDTable_internal_fast(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let mut dt = DTable.offset(1) as *const core::ffi::c_void;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    let mut args = HUF_DecompressFastArgs {
        ip: [core::ptr::null::<u8>(); 4],
        op: [core::ptr::null_mut::<u8>(); 4],
        bits: [0; 4],
        dt: core::ptr::null::<core::ffi::c_void>(),
        ilowest: core::ptr::null::<u8>(),
        oend: core::ptr::null_mut::<u8>(),
        iend: [core::ptr::null::<u8>(); 4],
    };
    let ret = HUF_DecompressFastArgs_init(&mut args, dst, dstSize, cSrc, cSrcSize, DTable);
    let err_code = ret;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if ret == 0 {
        return 0;
    }
    loopFn(&mut args);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let mut segmentEnd = dst as *mut u8;
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < 4 {
        if segmentSize <= oend.offset_from(segmentEnd) as core::ffi::c_long as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }

        let mut bit = match init_remaining_dstream(&args, i, segmentEnd) {
            Ok(v) => v,
            Err(e) => return e,
        };

        let fresh47 = &mut (*(args.op).as_mut_ptr().offset(i as isize));
        *fresh47 = (*fresh47).offset(HUF_decodeStreamX1(
            *(args.op).as_mut_ptr().offset(i as isize),
            &mut bit,
            segmentEnd,
            dt as *const HUF_DEltX1,
            HUF_DECODER_FAST_TABLELOG as u32,
        ) as isize);
        if *(args.op).as_mut_ptr().offset(i as isize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        i += 1;
    }
    dstSize
}
unsafe fn HUF_decompress1X1_usingDTable_internal_bmi2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe fn HUF_decompress1X1_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe fn HUF_decompress1X1_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        return HUF_decompress1X1_usingDTable_internal_bmi2(dst, dstSize, cSrc, cSrcSize, DTable);
    }
    HUF_decompress1X1_usingDTable_internal_default(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe fn HUF_decompress4X1_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        let loopFn = match flags & HUF_flags_disableAsm as i32 {
            0 => HUF_decompress4X1_usingDTable_internal_fast_asm_loop as HUF_DecompressFastLoopFn,
            _ => HUF_decompress4X1_usingDTable_internal_fast_c_loop as HUF_DecompressFastLoopFn,
        };

        if HUF_ENABLE_FAST_DECODE != 0 && flags & HUF_flags_disableFast as core::ffi::c_int == 0 {
            let ret = HUF_decompress4X1_usingDTable_internal_fast(
                dst, dstSize, cSrc, cSrcSize, DTable, loopFn,
            );
            if ret != 0 {
                return ret;
            }
        }

        HUF_decompress4X1_usingDTable_internal_bmi2(dst, dstSize, cSrc, cSrcSize, DTable)
    } else {
        HUF_decompress4X1_usingDTable_internal_default(dst, dstSize, cSrc, cSrcSize, DTable)
    }
}

unsafe fn HUF_decompress4X1_DCtx_wksp(
    dctx: &mut [HUF_DTable; 4097],
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    let dctx = dctx.as_mut_ptr();

    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress4X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    )
}

impl HUF_DEltX2 {
    #[inline]
    fn from_u32(value: u32) -> Self {
        let [a, b, c, d] = value.to_le_bytes();

        Self {
            sequence: u16::from_le_bytes([a, b]),
            nbBits: c,
            length: d,
        }
    }
}

fn HUF_buildDEltX2U32(symbol: u32, nbBits: u32, baseSeq: u32, level: core::ffi::c_int) -> u32 {
    if cfg!(target_endian = "little") {
        let mut seq = if level == 1 {
            symbol
        } else {
            baseSeq.wrapping_add(symbol << 8)
        };
        seq.wrapping_add(nbBits << 16)
            .wrapping_add((level as u32) << 24)
    } else {
        let mut seq = if level == 1 {
            symbol << 8
        } else {
            (baseSeq << 8).wrapping_add(symbol)
        };
        (seq << 16)
            .wrapping_add(nbBits << 8)
            .wrapping_add(level as u32)
    }
}

fn HUF_buildDEltX2(symbol: u8, nbBits: u32, baseSeq: u16, level: core::ffi::c_int) -> HUF_DEltX2 {
    HUF_DEltX2::from_u32(HUF_buildDEltX2U32(
        symbol as u32,
        nbBits,
        baseSeq as u32,
        level,
    ))
}

fn HUF_buildDEltX2U64(symbol: u32, nbBits: u32, baseSeq: u16, level: core::ffi::c_int) -> u64 {
    let DElt = HUF_buildDEltX2U32(symbol, nbBits, baseSeq as u32, level);
    DElt as u64 | (DElt as u64) << 32
}

unsafe fn HUF_fillDTableX2ForWeight(
    mut DTableRank: *mut HUF_DEltX2,
    sorted_symbols: &[sortedSymbol_t],
    nbBits: u32,
    tableLog: u32,
    baseSeq: u16,
    level: core::ffi::c_int,
) {
    let length = (1) << (tableLog.wrapping_sub(nbBits) & 0x1f as core::ffi::c_int as u32);
    let mut ptr = core::ptr::null::<sortedSymbol_t>();
    match length {
        1 => {
            for sorted_symbol in sorted_symbols {
                let DElt = HUF_buildDEltX2(sorted_symbol.symbol, nbBits, baseSeq, level);
                *DTableRank = DElt;
                DTableRank = DTableRank.offset(1);
            }
        }
        2 => {
            for sorted_symbol in sorted_symbols {
                let DElt_0 = HUF_buildDEltX2(sorted_symbol.symbol, nbBits, baseSeq, level);
                *DTableRank.offset(0) = DElt_0;
                *DTableRank.offset(1) = DElt_0;
                DTableRank = DTableRank.offset(2);
                ptr = ptr.offset(1);
            }
        }
        4 => {
            for sorted_symbol in sorted_symbols {
                let DEltX2 =
                    HUF_buildDEltX2U64(sorted_symbol.symbol as u32, nbBits, baseSeq, level);
                libc::memcpy(
                    DTableRank.offset(0) as *mut core::ffi::c_void,
                    &DEltX2 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(2) as *mut core::ffi::c_void,
                    &DEltX2 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
                DTableRank = DTableRank.offset(4);
                ptr = ptr.offset(1);
            }
        }
        8 => {
            for sorted_symbol in sorted_symbols {
                let DEltX2_0 =
                    HUF_buildDEltX2U64(sorted_symbol.symbol as u32, nbBits, baseSeq, level);
                libc::memcpy(
                    DTableRank.offset(0) as *mut core::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(2) as *mut core::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(4) as *mut core::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTableRank.offset(6) as *mut core::ffi::c_void,
                    &DEltX2_0 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
                DTableRank = DTableRank.offset(8);
                ptr = ptr.offset(1);
            }
        }
        _ => {
            for sorted_symbol in sorted_symbols {
                let DEltX2_1 =
                    HUF_buildDEltX2U64(sorted_symbol.symbol as u32, nbBits, baseSeq, level);
                let DTableRankEnd = DTableRank.offset(length as isize);
                while DTableRank != DTableRankEnd {
                    libc::memcpy(
                        DTableRank.offset(0) as *mut core::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(2) as *mut core::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(4) as *mut core::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTableRank.offset(6) as *mut core::ffi::c_void,
                        &DEltX2_1 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    DTableRank = DTableRank.offset(8);
                }
                ptr = ptr.offset(1);
            }
        }
    };
}
unsafe fn HUF_fillDTableX2Level2(
    DTable: *mut HUF_DEltX2,
    targetLog: u32,
    consumedBits: u32,
    rankVal: &[u32; 13],
    minWeight: core::ffi::c_int,
    maxWeight1: core::ffi::c_int,
    sortedSymbols: &[sortedSymbol_t],
    rankStart: &[u32; 15],
    nbBitsBaseline: u32,
    baseSeq: u16,
) {
    if minWeight > 1 {
        let length =
            (1) << (targetLog.wrapping_sub(consumedBits) & 0x1f as core::ffi::c_int as u32);
        let DEltX2 = HUF_buildDEltX2U64(baseSeq as u32, consumedBits, 0, 1);
        let skipSize = rankVal[minWeight as usize] as core::ffi::c_int;
        match length {
            2 => {
                libc::memcpy(
                    DTable as *mut core::ffi::c_void,
                    &DEltX2 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
            }
            4 => {
                libc::memcpy(
                    DTable.offset(0) as *mut core::ffi::c_void,
                    &DEltX2 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
                libc::memcpy(
                    DTable.offset(2) as *mut core::ffi::c_void,
                    &DEltX2 as *const u64 as *const core::ffi::c_void,
                    ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                );
            }
            _ => {
                let mut i: core::ffi::c_int = 0;
                i = 0;
                while i < skipSize {
                    libc::memcpy(
                        DTable.offset(i as isize).offset(0) as *mut core::ffi::c_void,
                        &DEltX2 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(2) as *mut core::ffi::c_void,
                        &DEltX2 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(4) as *mut core::ffi::c_void,
                        &DEltX2 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    libc::memcpy(
                        DTable.offset(i as isize).offset(6) as *mut core::ffi::c_void,
                        &DEltX2 as *const u64 as *const core::ffi::c_void,
                        ::core::mem::size_of::<u64>() as core::ffi::c_ulong as libc::size_t,
                    );
                    i += 8;
                }
            }
        }
    }

    for w in minWeight as usize..maxWeight1 as usize {
        let nbBits = nbBitsBaseline.wrapping_sub(w as u32);
        let totalBits = nbBits.wrapping_add(consumedBits);

        HUF_fillDTableX2ForWeight(
            DTable.add(rankVal[w] as usize),
            &sortedSymbols[rankStart[w] as usize..rankStart[w + 1] as usize],
            totalBits,
            targetLog,
            baseSeq,
            2,
        );
    }
}

unsafe fn HUF_fillDTableX2(
    DTable: *mut HUF_DEltX2,
    targetLog: u32,
    sortedList: &[sortedSymbol_t; 256],
    rankStart: &[u32; 15],
    rankValOrigin: &mut [rankValCol_t; 12],
    maxWeight: u32,
    nbBitsBaseline: u32,
) {
    let rankVal = rankValOrigin[0];
    let scaleLog = nbBitsBaseline.wrapping_sub(targetLog) as core::ffi::c_int;
    let minBits = nbBitsBaseline.wrapping_sub(maxWeight);

    let wEnd = maxWeight as core::ffi::c_int + 1;
    for w in 1..wEnd as usize {
        let range = rankStart[w as usize] as usize..rankStart[(w + 1) as usize] as usize;

        let nbBits = nbBitsBaseline.wrapping_sub(w as u32);
        if targetLog.wrapping_sub(nbBits) >= minBits {
            let mut start = rankVal[w as usize] as core::ffi::c_int;
            let length = (1) << (targetLog.wrapping_sub(nbBits) & 0x1f as core::ffi::c_int as u32);
            let minWeight = Ord::max(nbBits.wrapping_add(scaleLog as u32) as core::ffi::c_int, 1);

            for s in range {
                HUF_fillDTableX2Level2(
                    DTable.offset(start as isize),
                    targetLog,
                    nbBits,
                    &rankValOrigin[nbBits as usize],
                    minWeight,
                    wEnd,
                    sortedList,
                    rankStart,
                    nbBitsBaseline,
                    u16::from(sortedList[s].symbol),
                );
                start = (start as u32).wrapping_add(length) as core::ffi::c_int as core::ffi::c_int;
            }
        } else {
            HUF_fillDTableX2ForWeight(
                DTable.offset(rankVal[w as usize] as isize),
                &sortedList[range],
                nbBits,
                targetLog,
                0,
                1,
            );
        }
    }
}

pub unsafe fn HUF_readDTableX2_wksp(
    DTable: &mut [HUF_DTable; 4097],
    src: *const core::ffi::c_void,
    srcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    let DTable = DTable.as_mut_ptr();

    let mut tableLog: u32 = 0;
    let mut maxW: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let mut dtd = HUF_getDTableDesc(DTable);
    let mut maxTableLog = dtd.maxTableLog as u32;
    let mut iSize: size_t = 0;
    let mut dtPtr = DTable.offset(1) as *mut core::ffi::c_void;
    let dt = dtPtr as *mut HUF_DEltX2;
    let wksp = workSpace as *mut HUF_ReadDTableX2_Workspace;

    if ::core::mem::size_of::<HUF_ReadDTableX2_Workspace>() as core::ffi::c_ulong > wkspSize {
        return -(ZSTD_error_GENERIC as core::ffi::c_int) as size_t;
    }

    let mut rankStart = core::ptr::addr_of_mut!((*wksp).rankStart0)
        .cast::<u32>()
        .offset(1);

    (*wksp).rankStats.fill(0);
    (*wksp).rankStart0.fill(0);

    if maxTableLog > HUF_TABLELOG_MAX as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }

    iSize = HUF_readStats_wksp(
        &mut (*wksp).weightList,
        (HUF_SYMBOLVALUE_MAX + 1) as size_t,
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
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    if tableLog <= HUF_DECODER_FAST_TABLELOG as u32
        && maxTableLog > HUF_DECODER_FAST_TABLELOG as u32
    {
        maxTableLog = HUF_DECODER_FAST_TABLELOG as u32;
    }
    maxW = tableLog;
    while *((*wksp).rankStats).as_mut_ptr().offset(maxW as isize) == 0 {
        maxW = maxW.wrapping_sub(1);
    }
    let mut w: u32 = 0;
    let mut nextRankStart = 0 as core::ffi::c_int as u32;
    w = 1;
    while w < maxW.wrapping_add(1) {
        let mut curr = nextRankStart;
        nextRankStart =
            nextRankStart.wrapping_add(*((*wksp).rankStats).as_mut_ptr().offset(w as isize));
        *rankStart.offset(w as isize) = curr;
        w = w.wrapping_add(1);
    }
    *rankStart.offset(0) = nextRankStart;
    *rankStart.offset(maxW.wrapping_add(1) as isize) = nextRankStart;
    let mut s: u32 = 0;
    s = 0;
    while s < nbSymbols {
        let w_0 = *((*wksp).weightList).as_mut_ptr().offset(s as isize) as u32;
        let fresh49 = &mut (*rankStart.offset(w_0 as isize));
        let fresh50 = *fresh49;
        *fresh49 = (*fresh49).wrapping_add(1);
        let r = fresh50;
        (*((*wksp).sortedSymbol).as_mut_ptr().offset(r as isize)).symbol = s as u8;
        s = s.wrapping_add(1);
    }
    *rankStart.offset(0) = 0;
    let rankVal0 = (*((*wksp).rankVal).as_mut_ptr().offset(0)).as_mut_ptr();
    let rescale = maxTableLog.wrapping_sub(tableLog).wrapping_sub(1) as core::ffi::c_int;
    let mut nextRankVal = 0 as core::ffi::c_int as u32;
    let mut w_1: u32 = 0;
    w_1 = 1;
    while w_1 < maxW.wrapping_add(1) {
        let mut curr_0 = nextRankVal;
        nextRankVal = nextRankVal.wrapping_add(
            *((*wksp).rankStats).as_mut_ptr().offset(w_1 as isize)
                << w_1.wrapping_add(rescale as u32),
        );
        *rankVal0.offset(w_1 as isize) = curr_0;
        w_1 = w_1.wrapping_add(1);
    }
    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut consumed: u32 = 0;
    consumed = minBits;
    while consumed < maxTableLog.wrapping_sub(minBits).wrapping_add(1) {
        let rankValPtr = (*((*wksp).rankVal).as_mut_ptr().offset(consumed as isize)).as_mut_ptr();
        let mut w_2: u32 = 0;
        w_2 = 1;
        while w_2 < maxW.wrapping_add(1) {
            *rankValPtr.offset(w_2 as isize) = *rankVal0.offset(w_2 as isize) >> consumed;
            w_2 = w_2.wrapping_add(1);
        }
        consumed = consumed.wrapping_add(1);
    }
    HUF_fillDTableX2(
        dt,
        maxTableLog,
        &mut (*wksp).sortedSymbol,
        &mut (*wksp).rankStart0,
        &mut (*wksp).rankVal,
        maxW,
        tableLog.wrapping_add(1),
    );
    dtd.tableLog = maxTableLog as u8;
    dtd.tableType = 1;
    libc::memcpy(
        DTable as *mut core::ffi::c_void,
        &mut dtd as *mut DTableDesc as *const core::ffi::c_void,
        ::core::mem::size_of::<DTableDesc>() as core::ffi::c_ulong as libc::size_t,
    );
    iSize
}

#[inline(always)]
unsafe fn HUF_decodeSymbolX2(
    op: *mut core::ffi::c_void,
    DStream: &mut BIT_DStream_t,
    dt: *const HUF_DEltX2,
    dtLog: u32,
) -> u32 {
    let val = DStream.look_bits_fast(dtLog);
    libc::memcpy(
        op,
        &(*dt.offset(val as isize)).sequence as *const u16 as *const core::ffi::c_void,
        2,
    );
    DStream.skip_bits((*dt.offset(val as isize)).nbBits as u32);
    (*dt.offset(val as isize)).length as u32
}

#[inline(always)]
unsafe fn HUF_decodeLastSymbolX2(
    op: *mut core::ffi::c_void,
    DStream: &mut BIT_DStream_t,
    dt: *const HUF_DEltX2,
    dtLog: u32,
) -> u32 {
    let val = DStream.look_bits_fast(dtLog);
    libc::memcpy(
        op,
        &(*dt.offset(val as isize)).sequence as *const u16 as *const core::ffi::c_void,
        1 as libc::size_t,
    );
    if (*dt.offset(val as isize)).length as core::ffi::c_int == 1 {
        DStream.skip_bits((*dt.offset(val as isize)).nbBits as u32);
    } else if ((*DStream).bitsConsumed as core::ffi::c_ulong)
        < (::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
    {
        DStream.skip_bits((*dt.offset(val as isize)).nbBits as u32);
        if (*DStream).bitsConsumed as core::ffi::c_ulong
            > (::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
        {
            (*DStream).bitsConsumed = (::core::mem::size_of::<BitContainerType>()
                as core::ffi::c_ulong)
                .wrapping_mul(8) as core::ffi::c_uint;
        }
    }
    1
}
#[inline(always)]
unsafe fn HUF_decodeStreamX2(
    mut p: *mut u8,
    bitDPtr: &mut BIT_DStream_t,
    pEnd: *mut u8,
    dt: *const HUF_DEltX2,
    dtLog: u32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) as core::ffi::c_long as size_t
        >= ::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong
    {
        if dtLog <= 11 && MEM_64bits() != 0 {
            while (bitDPtr.reload() == StreamStatus::Unfinished) as core::ffi::c_int
                & (p < pEnd.offset(-(9))) as core::ffi::c_int
                != 0
            {
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
            }
        } else {
            while (bitDPtr.reload() == StreamStatus::Unfinished) as core::ffi::c_int
                & (p < pEnd.offset(
                    -((::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong)
                        .wrapping_sub(1) as isize),
                )) as core::ffi::c_int
                != 0
            {
                if MEM_64bits() != 0 {
                    p = p.offset(
                        HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
                }
                if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                    p = p.offset(
                        HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
                }
                if MEM_64bits() != 0 {
                    p = p.offset(
                        HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog)
                            as isize,
                    );
                }
                p = p.offset(
                    HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
                );
            }
        }
    } else {
        bitDPtr.reload();
    }
    if pEnd.offset_from(p) as core::ffi::c_long as size_t >= 2 {
        while (bitDPtr.reload() == StreamStatus::Unfinished) as core::ffi::c_int
            & (p <= pEnd.offset(-(2))) as core::ffi::c_int
            != 0
        {
            p = p.offset(
                HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
        while p <= pEnd.offset(-(2)) {
            p = p.offset(
                HUF_decodeSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
            );
        }
    }
    if p < pEnd {
        p = p.offset(
            HUF_decodeLastSymbolX2(p as *mut core::ffi::c_void, bitDPtr, dt, dtLog) as isize,
        );
    }
    p.offset_from(pStart) as core::ffi::c_long as size_t
}

#[inline(always)]
unsafe fn HUF_decompress1X2_usingDTable_internal_body(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    let src = core::slice::from_raw_parts(cSrc.cast::<u8>(), cSrcSize as usize);
    let mut bitD = match BIT_DStream_t::new(src) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    let ostart = dst as *mut u8;
    let oend =
        ZSTD_maybeNullPtrAdd(ostart as *mut core::ffi::c_void, dstSize as ptrdiff_t) as *mut u8;
    let dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let dtd = HUF_getDTableDesc(DTable);
    HUF_decodeStreamX2(ostart, &mut bitD, oend, dt, dtd.tableLog as u32);
    if !bitD.is_empty() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    dstSize
}

#[inline(always)]
unsafe fn HUF_decompress4X2_usingDTable_internal_body(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    if cSrcSize < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if dstSize < 6 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    let istart = cSrc as *const u8;
    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend.offset(
        -((::core::mem::size_of::<size_t>() as core::ffi::c_ulong).wrapping_sub(1) as isize),
    );
    let dtPtr = DTable.offset(1) as *const core::ffi::c_void;
    let dt = dtPtr as *const HUF_DEltX2;
    let length1 = MEM_readLE16(istart as *const core::ffi::c_void) as size_t;
    let length2 = MEM_readLE16(istart.offset(2) as *const core::ffi::c_void) as size_t;
    let length3 = MEM_readLE16(istart.offset(4) as *const core::ffi::c_void) as size_t;
    let length4 = cSrcSize.wrapping_sub(
        length1
            .wrapping_add(length2)
            .wrapping_add(length3)
            .wrapping_add(6),
    );
    let istart1 = istart.offset(6);
    let istart2 = istart1.offset(length1 as isize);
    let istart3 = istart2.offset(length2 as isize);
    let istart4 = istart3.offset(length3 as isize);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut endSignal = 1;
    let dtd = HUF_getDTableDesc(DTable);
    let dtLog = dtd.tableLog as u32;
    if length4 > cSrcSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if opStart4 > oend {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let mut bitD1 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart1, length1 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD2 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart2, length2 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD3 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart3, length3 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD4 = match BIT_DStream_t::new(core::slice::from_raw_parts(istart4, length4 as usize))
    {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    if oend.offset_from(op4) as core::ffi::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as core::ffi::c_ulong
    {
        while endSignal & (op4 < olimit) as core::ffi::c_int as u32 != 0 {
            if MEM_64bits() != 0 {
                op1 = op1.offset(HUF_decodeSymbolX2(
                    op1 as *mut core::ffi::c_void,
                    &mut bitD1,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                op1 = op1.offset(HUF_decodeSymbolX2(
                    op1 as *mut core::ffi::c_void,
                    &mut bitD1,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op1 = op1.offset(HUF_decodeSymbolX2(
                    op1 as *mut core::ffi::c_void,
                    &mut bitD1,
                    dt,
                    dtLog,
                ) as isize);
            }
            op1 =
                op1.offset(
                    HUF_decodeSymbolX2(op1 as *mut core::ffi::c_void, &mut bitD1, dt, dtLog)
                        as isize,
                );
            if MEM_64bits() != 0 {
                op2 = op2.offset(HUF_decodeSymbolX2(
                    op2 as *mut core::ffi::c_void,
                    &mut bitD2,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                op2 = op2.offset(HUF_decodeSymbolX2(
                    op2 as *mut core::ffi::c_void,
                    &mut bitD2,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op2 = op2.offset(HUF_decodeSymbolX2(
                    op2 as *mut core::ffi::c_void,
                    &mut bitD2,
                    dt,
                    dtLog,
                ) as isize);
            }
            op2 =
                op2.offset(
                    HUF_decodeSymbolX2(op2 as *mut core::ffi::c_void, &mut bitD2, dt, dtLog)
                        as isize,
                );
            endSignal &= (bitD1.reload_fast() == StreamStatus::Unfinished) as u32;
            endSignal &= (bitD2.reload_fast() == StreamStatus::Unfinished) as u32;
            if MEM_64bits() != 0 {
                op3 = op3.offset(HUF_decodeSymbolX2(
                    op3 as *mut core::ffi::c_void,
                    &mut bitD3,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                op3 = op3.offset(HUF_decodeSymbolX2(
                    op3 as *mut core::ffi::c_void,
                    &mut bitD3,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op3 = op3.offset(HUF_decodeSymbolX2(
                    op3 as *mut core::ffi::c_void,
                    &mut bitD3,
                    dt,
                    dtLog,
                ) as isize);
            }
            op3 =
                op3.offset(
                    HUF_decodeSymbolX2(op3 as *mut core::ffi::c_void, &mut bitD3, dt, dtLog)
                        as isize,
                );
            if MEM_64bits() != 0 {
                op4 = op4.offset(HUF_decodeSymbolX2(
                    op4 as *mut core::ffi::c_void,
                    &mut bitD4,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 || HUF_TABLELOG_MAX <= 12 {
                op4 = op4.offset(HUF_decodeSymbolX2(
                    op4 as *mut core::ffi::c_void,
                    &mut bitD4,
                    dt,
                    dtLog,
                ) as isize);
            }
            if MEM_64bits() != 0 {
                op4 = op4.offset(HUF_decodeSymbolX2(
                    op4 as *mut core::ffi::c_void,
                    &mut bitD4,
                    dt,
                    dtLog,
                ) as isize);
            }
            op4 =
                op4.offset(
                    HUF_decodeSymbolX2(op4 as *mut core::ffi::c_void, &mut bitD4, dt, dtLog)
                        as isize,
                );
            endSignal &= (bitD3.reload_fast() == StreamStatus::Unfinished) as u32;
            endSignal &= (bitD4.reload_fast() == StreamStatus::Unfinished) as u32;
        }
    }
    if op1 > opStart2 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op2 > opStart3 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op3 > opStart4 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    HUF_decodeStreamX2(op1, &mut bitD1, opStart2, dt, dtLog);
    HUF_decodeStreamX2(op2, &mut bitD2, opStart3, dt, dtLog);
    HUF_decodeStreamX2(op3, &mut bitD3, opStart4, dt, dtLog);
    HUF_decodeStreamX2(op4, &mut bitD4, oend, dt, dtLog);

    if !(bitD1.is_empty() && bitD2.is_empty() && bitD3.is_empty() && bitD4.is_empty()) {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    dstSize
}

unsafe fn HUF_decompress4X2_usingDTable_internal_bmi2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe fn HUF_decompress4X2_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_fast_c_loop(
    args: &mut HUF_DecompressFastArgs,
) {
    let mut bits: [u64; 4] = [0; 4];
    let mut ip: [*const u8; 4] = [core::ptr::null::<u8>(); 4];
    let mut op: [*mut u8; 4] = [core::ptr::null_mut::<u8>(); 4];
    let mut oend: [*mut u8; 4] = [core::ptr::null_mut::<u8>(); 4];
    let dtable = (*args).dt as *const HUF_DEltX2;
    let ilowest = (*args).ilowest;
    libc::memcpy(
        &mut bits as *mut [u64; 4] as *mut core::ffi::c_void,
        &mut (*args).bits as *mut [u64; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut ip as *mut [*const u8; 4] as *mut core::ffi::c_void,
        &mut (*args).ip as *mut [*const u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut op as *mut [*mut u8; 4] as *mut core::ffi::c_void,
        &mut (*args).op as *mut [*mut u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    let fresh51 = &mut (*oend.as_mut_ptr().offset(0));
    *fresh51 = *op.as_mut_ptr().offset(1);
    let fresh52 = &mut (*oend.as_mut_ptr().offset(1));
    *fresh52 = *op.as_mut_ptr().offset(2);
    let fresh53 = &mut (*oend.as_mut_ptr().offset(2));
    *fresh53 = *op.as_mut_ptr().offset(3);
    let fresh54 = &mut (*oend.as_mut_ptr().offset(3));
    *fresh54 = (*args).oend;
    's_45: loop {
        let mut olimit = core::ptr::null_mut::<u8>();
        let mut stream: core::ffi::c_int = 0;
        stream = 0;
        while stream < 4 {
            stream += 1;
        }
        let mut iters =
            (*ip.as_mut_ptr().offset(0)).offset_from(ilowest) as core::ffi::c_long as size_t / 7;
        stream = 0;
        while stream < 4 {
            let oiters = (*oend.as_mut_ptr().offset(stream as isize))
                .offset_from(*op.as_mut_ptr().offset(stream as isize))
                as core::ffi::c_long as size_t
                / 10;
            iters = if iters < oiters { iters } else { oiters };
            stream += 1;
        }
        olimit = (*op.as_mut_ptr().offset(3)).offset((iters * 5) as isize);
        if *op.as_mut_ptr().offset(3) == olimit {
            break;
        }
        stream = 1;
        while stream < 4 {
            if *ip.as_mut_ptr().offset(stream as isize)
                < *ip.as_mut_ptr().offset((stream - 1) as isize)
            {
                break 's_45;
            }
            stream += 1;
        }
        stream = 1;
        while stream < 4 {
            stream += 1;
        }
        loop {
            if 0 != 0 || 0 != 3 {
                let index = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
                let entry = *dtable.offset(index as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0) as *mut core::ffi::c_void,
                    entry.sequence,
                );
                *bits.as_mut_ptr().offset(0) <<=
                    entry.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh55 = &mut (*op.as_mut_ptr().offset(0));
                *fresh55 = (*fresh55).offset(entry.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 1 != 3 {
                let index_0 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
                let entry_0 = *dtable.offset(index_0 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1) as *mut core::ffi::c_void,
                    entry_0.sequence,
                );
                *bits.as_mut_ptr().offset(1) <<=
                    entry_0.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh56 = &mut (*op.as_mut_ptr().offset(1));
                *fresh56 = (*fresh56).offset(entry_0.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 2 != 3 {
                let index_1 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
                let entry_1 = *dtable.offset(index_1 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2) as *mut core::ffi::c_void,
                    entry_1.sequence,
                );
                *bits.as_mut_ptr().offset(2) <<=
                    entry_1.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh57 = &mut (*op.as_mut_ptr().offset(2));
                *fresh57 = (*fresh57).offset(entry_1.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 3 != 3 {
                let index_2 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_2 = *dtable.offset(index_2 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_2.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_2.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh58 = &mut (*op.as_mut_ptr().offset(3));
                *fresh58 = (*fresh58).offset(entry_2.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 0 != 3 {
                let index_3 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
                let entry_3 = *dtable.offset(index_3 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0) as *mut core::ffi::c_void,
                    entry_3.sequence,
                );
                *bits.as_mut_ptr().offset(0) <<=
                    entry_3.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh59 = &mut (*op.as_mut_ptr().offset(0));
                *fresh59 = (*fresh59).offset(entry_3.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 1 != 3 {
                let index_4 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
                let entry_4 = *dtable.offset(index_4 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1) as *mut core::ffi::c_void,
                    entry_4.sequence,
                );
                *bits.as_mut_ptr().offset(1) <<=
                    entry_4.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh60 = &mut (*op.as_mut_ptr().offset(1));
                *fresh60 = (*fresh60).offset(entry_4.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 2 != 3 {
                let index_5 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
                let entry_5 = *dtable.offset(index_5 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2) as *mut core::ffi::c_void,
                    entry_5.sequence,
                );
                *bits.as_mut_ptr().offset(2) <<=
                    entry_5.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh61 = &mut (*op.as_mut_ptr().offset(2));
                *fresh61 = (*fresh61).offset(entry_5.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 3 != 3 {
                let index_6 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_6 = *dtable.offset(index_6 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_6.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_6.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh62 = &mut (*op.as_mut_ptr().offset(3));
                *fresh62 = (*fresh62).offset(entry_6.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 0 != 3 {
                let index_7 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
                let entry_7 = *dtable.offset(index_7 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0) as *mut core::ffi::c_void,
                    entry_7.sequence,
                );
                *bits.as_mut_ptr().offset(0) <<=
                    entry_7.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh63 = &mut (*op.as_mut_ptr().offset(0));
                *fresh63 = (*fresh63).offset(entry_7.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 1 != 3 {
                let index_8 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
                let entry_8 = *dtable.offset(index_8 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1) as *mut core::ffi::c_void,
                    entry_8.sequence,
                );
                *bits.as_mut_ptr().offset(1) <<=
                    entry_8.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh64 = &mut (*op.as_mut_ptr().offset(1));
                *fresh64 = (*fresh64).offset(entry_8.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 2 != 3 {
                let index_9 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
                let entry_9 = *dtable.offset(index_9 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2) as *mut core::ffi::c_void,
                    entry_9.sequence,
                );
                *bits.as_mut_ptr().offset(2) <<=
                    entry_9.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh65 = &mut (*op.as_mut_ptr().offset(2));
                *fresh65 = (*fresh65).offset(entry_9.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 3 != 3 {
                let index_10 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_10 = *dtable.offset(index_10 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_10.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_10.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh66 = &mut (*op.as_mut_ptr().offset(3));
                *fresh66 = (*fresh66).offset(entry_10.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 0 != 3 {
                let index_11 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
                let entry_11 = *dtable.offset(index_11 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0) as *mut core::ffi::c_void,
                    entry_11.sequence,
                );
                *bits.as_mut_ptr().offset(0) <<=
                    entry_11.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh67 = &mut (*op.as_mut_ptr().offset(0));
                *fresh67 = (*fresh67).offset(entry_11.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 1 != 3 {
                let index_12 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
                let entry_12 = *dtable.offset(index_12 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1) as *mut core::ffi::c_void,
                    entry_12.sequence,
                );
                *bits.as_mut_ptr().offset(1) <<=
                    entry_12.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh68 = &mut (*op.as_mut_ptr().offset(1));
                *fresh68 = (*fresh68).offset(entry_12.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 2 != 3 {
                let index_13 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
                let entry_13 = *dtable.offset(index_13 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2) as *mut core::ffi::c_void,
                    entry_13.sequence,
                );
                *bits.as_mut_ptr().offset(2) <<=
                    entry_13.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh69 = &mut (*op.as_mut_ptr().offset(2));
                *fresh69 = (*fresh69).offset(entry_13.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 3 != 3 {
                let index_14 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_14 = *dtable.offset(index_14 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_14.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_14.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh70 = &mut (*op.as_mut_ptr().offset(3));
                *fresh70 = (*fresh70).offset(entry_14.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 0 != 3 {
                let index_15 = (*bits.as_mut_ptr().offset(0) >> 53) as core::ffi::c_int;
                let entry_15 = *dtable.offset(index_15 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(0) as *mut core::ffi::c_void,
                    entry_15.sequence,
                );
                *bits.as_mut_ptr().offset(0) <<=
                    entry_15.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh71 = &mut (*op.as_mut_ptr().offset(0));
                *fresh71 = (*fresh71).offset(entry_15.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 1 != 3 {
                let index_16 = (*bits.as_mut_ptr().offset(1) >> 53) as core::ffi::c_int;
                let entry_16 = *dtable.offset(index_16 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(1) as *mut core::ffi::c_void,
                    entry_16.sequence,
                );
                *bits.as_mut_ptr().offset(1) <<=
                    entry_16.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh72 = &mut (*op.as_mut_ptr().offset(1));
                *fresh72 = (*fresh72).offset(entry_16.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 2 != 3 {
                let index_17 = (*bits.as_mut_ptr().offset(2) >> 53) as core::ffi::c_int;
                let entry_17 = *dtable.offset(index_17 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(2) as *mut core::ffi::c_void,
                    entry_17.sequence,
                );
                *bits.as_mut_ptr().offset(2) <<=
                    entry_17.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh73 = &mut (*op.as_mut_ptr().offset(2));
                *fresh73 = (*fresh73).offset(entry_17.length as core::ffi::c_int as isize);
            }
            if 0 != 0 || 3 != 3 {
                let index_18 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_18 = *dtable.offset(index_18 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_18.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_18.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh74 = &mut (*op.as_mut_ptr().offset(3));
                *fresh74 = (*fresh74).offset(entry_18.length as core::ffi::c_int as isize);
            }
            if 1 != 0 || 3 != 3 {
                let index_19 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_19 = *dtable.offset(index_19 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_19.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_19.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh75 = &mut (*op.as_mut_ptr().offset(3));
                *fresh75 = (*fresh75).offset(entry_19.length as core::ffi::c_int as isize);
            }
            if 1 != 0 || 3 != 3 {
                let index_20 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_20 = *dtable.offset(index_20 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_20.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_20.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh76 = &mut (*op.as_mut_ptr().offset(3));
                *fresh76 = (*fresh76).offset(entry_20.length as core::ffi::c_int as isize);
            }
            let ctz = {
                let mut val = *bits.as_mut_ptr().offset(0);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits = ctz & 7;
            let nbBytes = ctz >> 3;
            let fresh77 = &mut (*ip.as_mut_ptr().offset(0));
            *fresh77 = (*fresh77).offset(-(nbBytes as isize));
            *bits.as_mut_ptr().offset(0) =
                MEM_read64(*ip.as_mut_ptr().offset(0) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(0) <<= nbBits;
            if 1 != 0 || 3 != 3 {
                let index_21 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_21 = *dtable.offset(index_21 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_21.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_21.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh78 = &mut (*op.as_mut_ptr().offset(3));
                *fresh78 = (*fresh78).offset(entry_21.length as core::ffi::c_int as isize);
            }
            let ctz_0 = {
                let mut val = *bits.as_mut_ptr().offset(1);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits_0 = ctz_0 & 7;
            let nbBytes_0 = ctz_0 >> 3;
            let fresh79 = &mut (*ip.as_mut_ptr().offset(1));
            *fresh79 = (*fresh79).offset(-(nbBytes_0 as isize));
            *bits.as_mut_ptr().offset(1) =
                MEM_read64(*ip.as_mut_ptr().offset(1) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(1) <<= nbBits_0;
            if 1 != 0 || 3 != 3 {
                let index_22 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_22 = *dtable.offset(index_22 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_22.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_22.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh80 = &mut (*op.as_mut_ptr().offset(3));
                *fresh80 = (*fresh80).offset(entry_22.length as core::ffi::c_int as isize);
            }
            let ctz_1 = {
                let mut val = *bits.as_mut_ptr().offset(2);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits_1 = ctz_1 & 7;
            let nbBytes_1 = ctz_1 >> 3;
            let fresh81 = &mut (*ip.as_mut_ptr().offset(2));
            *fresh81 = (*fresh81).offset(-(nbBytes_1 as isize));
            *bits.as_mut_ptr().offset(2) =
                MEM_read64(*ip.as_mut_ptr().offset(2) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(2) <<= nbBits_1;
            if 1 != 0 || 3 != 3 {
                let index_23 = (*bits.as_mut_ptr().offset(3) >> 53) as core::ffi::c_int;
                let entry_23 = *dtable.offset(index_23 as isize);
                MEM_write16(
                    *op.as_mut_ptr().offset(3) as *mut core::ffi::c_void,
                    entry_23.sequence,
                );
                *bits.as_mut_ptr().offset(3) <<=
                    entry_23.nbBits as core::ffi::c_int & 0x3f as core::ffi::c_int;
                let fresh82 = &mut (*op.as_mut_ptr().offset(3));
                *fresh82 = (*fresh82).offset(entry_23.length as core::ffi::c_int as isize);
            }
            let ctz_2 = {
                let mut val = *bits.as_mut_ptr().offset(3);
                val.trailing_zeros()
            } as core::ffi::c_int;
            let nbBits_2 = ctz_2 & 7;
            let nbBytes_2 = ctz_2 >> 3;
            let fresh83 = &mut (*ip.as_mut_ptr().offset(3));
            *fresh83 = (*fresh83).offset(-(nbBytes_2 as isize));
            *bits.as_mut_ptr().offset(3) =
                MEM_read64(*ip.as_mut_ptr().offset(3) as *const core::ffi::c_void) | 1;
            *bits.as_mut_ptr().offset(3) <<= nbBits_2;
            if *op.as_mut_ptr().offset(3) >= olimit {
                break;
            }
        }
    }
    libc::memcpy(
        &mut (*args).bits as *mut [u64; 4] as *mut core::ffi::c_void,
        &mut bits as *mut [u64; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[u64; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).ip as *mut [*const u8; 4] as *mut core::ffi::c_void,
        &mut ip as *mut [*const u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*const u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
    libc::memcpy(
        &mut (*args).op as *mut [*mut u8; 4] as *mut core::ffi::c_void,
        &mut op as *mut [*mut u8; 4] as *const core::ffi::c_void,
        ::core::mem::size_of::<[*mut u8; 4]>() as core::ffi::c_ulong as libc::size_t,
    );
}

unsafe fn HUF_decompress4X2_usingDTable_internal_fast(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let mut dt = DTable.offset(1) as *const core::ffi::c_void;
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;
    let mut args = HUF_DecompressFastArgs {
        ip: [core::ptr::null::<u8>(); 4],
        op: [core::ptr::null_mut::<u8>(); 4],
        bits: [0; 4],
        dt: core::ptr::null::<core::ffi::c_void>(),
        ilowest: core::ptr::null::<u8>(),
        oend: core::ptr::null_mut::<u8>(),
        iend: [core::ptr::null::<u8>(); 4],
    };
    let ret = HUF_DecompressFastArgs_init(&mut args, dst, dstSize, cSrc, cSrcSize, DTable);
    let err_code = ret;
    if ERR_isError(err_code) != 0 {
        return err_code;
    }
    if ret == 0 {
        return 0;
    }
    loopFn(&mut args);
    let segmentSize = dstSize.wrapping_add(3) / 4;
    let mut segmentEnd = dst as *mut u8;
    let mut i: core::ffi::c_int = 0;
    i = 0;
    while i < 4 {
        if segmentSize <= oend.offset_from(segmentEnd) as core::ffi::c_long as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }

        let mut bit = match init_remaining_dstream(&args, i, segmentEnd) {
            Ok(v) => v,
            Err(e) => return e,
        };

        let fresh84 = &mut (*(args.op).as_mut_ptr().offset(i as isize));
        *fresh84 = (*fresh84).offset(HUF_decodeStreamX2(
            *(args.op).as_mut_ptr().offset(i as isize),
            &mut bit,
            segmentEnd,
            dt as *const HUF_DEltX2,
            HUF_DECODER_FAST_TABLELOG as u32,
        ) as isize);
        if *(args.op).as_mut_ptr().offset(i as isize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
        i += 1;
    }
    dstSize
}

unsafe fn HUF_decompress4X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        let loopFn = match flags & HUF_flags_disableAsm as core::ffi::c_int {
            0 => HUF_decompress4X2_usingDTable_internal_fast_asm_loop as HUF_DecompressFastLoopFn,
            _ => HUF_decompress4X2_usingDTable_internal_fast_c_loop as HUF_DecompressFastLoopFn,
        };

        if HUF_ENABLE_FAST_DECODE != 0 && flags & HUF_flags_disableFast as core::ffi::c_int == 0 {
            let ret = HUF_decompress4X2_usingDTable_internal_fast(
                dst, dstSize, cSrc, cSrcSize, DTable, loopFn,
            );
            if ret != 0 {
                return ret;
            }
        }

        HUF_decompress4X2_usingDTable_internal_bmi2(dst, dstSize, cSrc, cSrcSize, DTable)
    } else {
        HUF_decompress4X2_usingDTable_internal_default(dst, dstSize, cSrc, cSrcSize, DTable)
    }
}

unsafe fn HUF_decompress1X2_usingDTable_internal_bmi2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}
unsafe fn HUF_decompress1X2_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, cSrc, cSrcSize, DTable)
}

unsafe fn HUF_decompress1X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        return HUF_decompress1X2_usingDTable_internal_bmi2(dst, dstSize, cSrc, cSrcSize, DTable);
    }
    HUF_decompress1X2_usingDTable_internal_default(dst, dstSize, cSrc, cSrcSize, DTable)
}

pub unsafe fn HUF_decompress1X2_DCtx_wksp(
    dctx: &mut [HUF_DTable; 4097],
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX2_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    let dctx = dctx.as_mut_ptr();
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress1X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    )
}
unsafe fn HUF_decompress4X2_DCtx_wksp(
    dctx: &mut [HUF_DTable; 4097],
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    let mut ip = cSrc as *const u8;
    let mut hSize = HUF_readDTableX2_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    let dctx = dctx.as_mut_ptr();
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress4X2_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    )
}
static algoTime: [[algo_time_t; 2]; 16] = [
    [
        {
            algo_time_t {
                tableTime: 0,
                decode256Time: 0,
            }
        },
        {
            algo_time_t {
                tableTime: 1,
                decode256Time: 1,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 0,
                decode256Time: 0,
            }
        },
        {
            algo_time_t {
                tableTime: 1,
                decode256Time: 1,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 150,
                decode256Time: 216,
            }
        },
        {
            algo_time_t {
                tableTime: 381,
                decode256Time: 119,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 170,
                decode256Time: 205,
            }
        },
        {
            algo_time_t {
                tableTime: 514,
                decode256Time: 112,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 177,
                decode256Time: 199,
            }
        },
        {
            algo_time_t {
                tableTime: 539,
                decode256Time: 110,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 197,
                decode256Time: 194,
            }
        },
        {
            algo_time_t {
                tableTime: 644,
                decode256Time: 107,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 221,
                decode256Time: 192,
            }
        },
        {
            algo_time_t {
                tableTime: 735,
                decode256Time: 107,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 256,
                decode256Time: 189,
            }
        },
        {
            algo_time_t {
                tableTime: 881,
                decode256Time: 106,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 359,
                decode256Time: 188,
            }
        },
        {
            algo_time_t {
                tableTime: 1167,
                decode256Time: 109,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 582,
                decode256Time: 187,
            }
        },
        {
            algo_time_t {
                tableTime: 1570,
                decode256Time: 114,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 688,
                decode256Time: 187,
            }
        },
        {
            algo_time_t {
                tableTime: 1712,
                decode256Time: 122,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 825,
                decode256Time: 186,
            }
        },
        {
            algo_time_t {
                tableTime: 1965,
                decode256Time: 136,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 976,
                decode256Time: 185,
            }
        },
        {
            algo_time_t {
                tableTime: 2131,
                decode256Time: 150,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1180,
                decode256Time: 186,
            }
        },
        {
            algo_time_t {
                tableTime: 2070,
                decode256Time: 175,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1377,
                decode256Time: 185,
            }
        },
        {
            algo_time_t {
                tableTime: 1731,
                decode256Time: 202,
            }
        },
    ],
    [
        {
            algo_time_t {
                tableTime: 1412,
                decode256Time: 185,
            }
        },
        {
            algo_time_t {
                tableTime: 1695,
                decode256Time: 202,
            }
        },
    ],
];

enum Decoder {
    A1,
    A2,
}

///  Tells which decoder is likely to decode faster,
///  based on a set of pre-computed metrics.
///
/// @return : 0==HUF_decompress4X1, 1==HUF_decompress4X2 .
///  Assumption : 0 < dstSize <= 128 KB */
fn HUF_selectDecoder(dstSize: size_t, cSrcSize: size_t) -> Decoder {
    let D256 = (dstSize >> 8) as u32;

    let Q = if cSrcSize >= dstSize {
        15
    } else {
        (cSrcSize * 16 / dstSize) as usize
    };

    let [time0, time1] = algoTime[Q];
    let DTime0 = time0.tableTime + time0.decode256Time * D256;
    let DTime1 = time1.tableTime + time1.decode256Time * D256;

    if (DTime1 + (DTime1 >> 5)) < DTime0 {
        Decoder::A2
    } else {
        Decoder::A1
    }
}

pub unsafe fn HUF_decompress1X_DCtx_wksp(
    dctx: &mut [HUF_DTable; 4097],
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    if dstSize == 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if cSrcSize == dstSize {
        libc::memcpy(dst, cSrc, dstSize as libc::size_t);
        return dstSize;
    }
    if cSrcSize == 1 {
        ptr::write_bytes(dst, *(cSrc as *const u8), dstSize as usize);
        return dstSize;
    }

    match HUF_selectDecoder(dstSize, cSrcSize) {
        Decoder::A1 => HUF_decompress1X1_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        ),
        Decoder::A2 => HUF_decompress1X2_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        ),
    }
}
pub unsafe fn HUF_decompress1X_usingDTable(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    flags: core::ffi::c_int,
) -> size_t {
    let dtd = HUF_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 0 {
        HUF_decompress1X2_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    } else {
        HUF_decompress1X1_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    }
}
pub unsafe fn HUF_decompress1X1_DCtx_wksp(
    dctx: &mut [HUF_DTable; 4097],
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    mut cSrcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    let dctx = dctx.as_mut_ptr();

    let mut ip = cSrc as *const u8;
    let hSize = HUF_readDTableX1_wksp(dctx, cSrc, cSrcSize, workSpace, wkspSize, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize >= cSrcSize {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }
    ip = ip.offset(hSize as isize);
    cSrcSize = cSrcSize.wrapping_sub(hSize);
    HUF_decompress1X1_usingDTable_internal(
        dst,
        dstSize,
        ip as *const core::ffi::c_void,
        cSrcSize,
        dctx,
        flags,
    )
}
pub unsafe fn HUF_decompress4X_usingDTable(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    DTable: *const HUF_DTable,
    flags: core::ffi::c_int,
) -> size_t {
    let dtd = HUF_getDTableDesc(DTable);
    if dtd.tableType as core::ffi::c_int != 0 {
        HUF_decompress4X2_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    } else {
        HUF_decompress4X1_usingDTable_internal(dst, maxDstSize, cSrc, cSrcSize, DTable, flags)
    }
}

pub unsafe fn HUF_decompress4X_hufOnly_wksp(
    dctx: &mut [HUF_DTable; 4097],
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    cSrc: *const core::ffi::c_void,
    cSrcSize: size_t,
    workSpace: *mut core::ffi::c_void,
    wkspSize: size_t,
    flags: core::ffi::c_int,
) -> size_t {
    if dstSize == 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if cSrcSize == 0 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    match HUF_selectDecoder(dstSize, cSrcSize) {
        Decoder::A1 => HUF_decompress4X1_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        ),
        Decoder::A2 => HUF_decompress4X2_DCtx_wksp(
            dctx, dst, dstSize, cSrc, cSrcSize, workSpace, wkspSize, flags,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_DEltX2_to_u32() {
        assert_eq!(
            HUF_buildDEltX2U32(0xAABB, 0xCC, 0xDD, 0xEE).to_le_bytes(),
            [0xDD, 0xBB, 0x76, 0xEF]
        );
        assert_eq!(HUF_buildDEltX2U32(1, 2, 3, 4).to_le_bytes(), [3, 1, 2, 4]);
    }

    #[test]
    fn test_buildDEltX2() {
        assert_eq!(
            HUF_buildDEltX2(0xAA, 0xBB, 0xCC, 0xDD),
            HUF_DEltX2 {
                sequence: 0xAACC,
                nbBits: 0xBB,
                length: 0xDD,
            }
        );

        assert_eq!(
            HUF_buildDEltX2(1, 2, 3, 4),
            HUF_DEltX2 {
                sequence: 0x0103,
                nbBits: 2,
                length: 4,
            }
        );
    }
}
