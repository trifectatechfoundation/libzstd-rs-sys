use core::ptr;

use crate::lib::common::bitstream::{BIT_DStream_t, BitContainerType, StreamStatus};
use crate::lib::common::entropy_common::HUF_readStats_wksp;
use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::fse_decompress::Error;
use crate::lib::common::mem::{
    MEM_isLittleEndian, MEM_read64, MEM_readLE16, MEM_readLEST, MEM_write16,
};
use crate::lib::decompress::Workspace;
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
pub struct DTable {
    pub description: DTableDesc,
    pub data: DTableData,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DTableData {
    data: [u32; 4096],
}

impl DTableData {
    fn as_x1(&self) -> &[HUF_DEltX1; 4096] {
        unsafe { core::mem::transmute(&self.data) }
    }

    fn as_x1_mut(&mut self) -> &mut [HUF_DEltX1; 4096] {
        unsafe { core::mem::transmute(&mut self.data) }
    }

    fn as_x2(&self) -> &[HUF_DEltX2; 4096] {
        unsafe { core::mem::transmute(&self.data) }
    }

    fn as_x2_mut(&mut self) -> &mut [HUF_DEltX2; 4096] {
        unsafe { core::mem::transmute(&mut self.data) }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DTableDesc {
    pub maxTableLog: u8,
    pub tableType: u8,
    pub tableLog: u8,
    pub reserved: u8,
}

impl DTableDesc {
    pub fn from_u32(value: u32) -> Self {
        let [maxTableLog, tableType, tableLog, reserved] = value.to_le_bytes();

        Self {
            maxTableLog,
            tableType,
            tableLog,
            reserved,
        }
    }
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
    pub statsWksp: crate::lib::common::entropy_common::Workspace,
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
    pub calleeWksp: crate::lib::common::entropy_common::Workspace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortedSymbol_t {
    pub symbol: u8,
}
#[derive(Copy, Clone, Default)]
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

impl HUF_DecompressFastArgs {
    unsafe fn new(
        dst: *mut core::ffi::c_void,
        dstSize: size_t,
        src: &[u8],
        DTable: &DTable,
    ) -> Result<Option<Self>, Error> {
        let mut args = Self::default();

        let dt = DTable.data.as_x2().as_ptr() as *const core::ffi::c_void;
        let dtLog = DTable.description.tableLog as u32;
        let srcSize = src.len() as size_t;
        let istart = src.as_ptr();
        let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;

        // The fast decoding loop assumes 64-bit little-endian.
        if cfg!(target_endian = "big") || cfg!(target_pointer_width = "32") {
            return Ok(None);
        }

        // Avoid nullptr addition
        if dstSize == 0 {
            return Ok(None);
        }
        assert!(!dst.is_null());

        // strict minimum : jump table + 1 byte per stream.
        if srcSize < 10 {
            return Err(Error::corruption_detected);
        }

        // Must have at least 8 bytes per stream because we don't handle initializing smaller bit containers.
        // If table log is not correct at this point, fallback to the old decoder.
        // On small inputs we don't have enough data to trigger the fast loop, so use the old decoder.
        if dtLog != HUF_DECODER_FAST_TABLELOG as u32 {
            return Ok(None);
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
            return Ok(None);
        }

        if length4 > srcSize {
            return Err(Error::corruption_detected);
        }

        /* ip[] contains the position that is currently loaded into bits[]. */
        args.ip[0] = args.iend[1].sub(size_of::<u64>());
        args.ip[1] = args.iend[2].sub(size_of::<u64>());
        args.ip[2] = args.iend[3].sub(size_of::<u64>());
        args.ip[3] = src.as_ptr().add(srcSize as usize - size_of::<u64>());

        /* op[] contains the output pointers. */
        args.op[0] = dst.cast::<u8>();
        args.op[1] = args.op[0].add(dstSize.div_ceil(4) as usize);
        args.op[2] = args.op[1].add(dstSize.div_ceil(4) as usize);
        args.op[3] = args.op[2].add(dstSize.div_ceil(4) as usize);

        // No point to call the ASM loop for tiny outputs.
        if *(args.op).as_mut_ptr().offset(3) >= oend {
            return Ok(None);
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

        Ok(Some(args))
    }
}

unsafe fn init_remaining_dstream(
    args: &HUF_DecompressFastArgs,
    stream: usize,
    segmentEnd: *mut u8,
) -> Result<BIT_DStream_t, Error> {
    if args.op[stream] > segmentEnd {
        return Err(Error::corruption_detected);
    }

    if args.ip[stream] < args.iend[stream].sub(8) {
        return Err(Error::corruption_detected);
    }

    let bitContainer = MEM_readLEST(args.ip[stream] as *const core::ffi::c_void) as usize;
    let bitsConsumed = args.bits[stream].trailing_zeros();
    let start = args.ilowest as *const core::ffi::c_char;
    let limitPtr = start.add(::core::mem::size_of::<size_t>());
    let ptr = args.ip[stream] as *const core::ffi::c_char;

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

/// Increase the tableLog to targetTableLog and rescales the stats.
///
/// If tableLog > targetTableLog this is a no-op.
/// @returns New tableLog
fn HUF_rescaleStats(
    huffWeight: &mut [u8; 256],
    rankVal: &mut rankValCol_t,
    nbSymbols: u32,
    tableLog: u32,
    targetTableLog: u32,
) -> u32 {
    if tableLog > targetTableLog {
        return tableLog;
    }

    if tableLog < targetTableLog {
        let scale = targetTableLog as usize - tableLog as usize;

        /* Increase the weight for all non-zero probability symbols by scale. */
        for s in 0..nbSymbols as usize {
            huffWeight[s] += (if huffWeight[s] == 0 { 0 } else { scale }) as u8;
        }

        // Update rankVal to reflect the new weights.
        // All weights except 0 get moved to weight + scale.
        // Weights [1, scale] are empty.
        let mut s = targetTableLog as usize;
        while s > scale {
            rankVal[s as usize] = rankVal[s - scale as usize];
            s -= 1;
        }

        rankVal[1..=scale as usize].fill(0);
    }

    targetTableLog
}

pub fn HUF_readDTableX1_wksp(
    DTable: &mut DTable,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    let mut dtd = DTable.description;
    let dt = DTable.data.as_x1_mut();

    let mut tableLog = 0;
    let mut nbSymbols = 0;
    let mut iSize: size_t = 0;

    let wksp = workSpace.as_x1_mut();

    iSize = HUF_readStats_wksp(
        &mut wksp.huffWeight,
        (HUF_SYMBOLVALUE_MAX + 1) as size_t,
        &mut wksp.rankVal,
        &mut nbSymbols,
        &mut tableLog,
        src,
        &mut wksp.statsWksp,
        flags,
    );
    if ERR_isError(iSize) != 0 {
        return iSize;
    }

    let maxTableLog = (dtd.maxTableLog as core::ffi::c_int + 1) as u32;
    let targetTableLog = if maxTableLog < 11 { maxTableLog } else { 11 };
    tableLog = HUF_rescaleStats(
        &mut wksp.huffWeight,
        &mut wksp.rankVal,
        nbSymbols,
        tableLog,
        targetTableLog,
    );
    if tableLog > (dtd.maxTableLog as core::ffi::c_int + 1) as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }
    dtd.tableType = 0;
    dtd.tableLog = tableLog as u8;

    DTable.description = dtd;

    // Compute symbols and rankStart given rankVal:
    //
    // rankVal already contains the number of values of each weight.
    //
    // symbols contains the symbols ordered by weight. First are the rankVal[0]
    // weight 0 symbols, followed by the rankVal[1] weight 1 symbols, and so on.
    // symbols[0] is filled (but unused) to avoid a branch.
    //
    // rankStart contains the offset where each rank belongs in the DTable.
    // rankStart[0] is not filled because there are no entries in the table for
    // weight 0.
    let mut nextRankStart = 0 as core::ffi::c_int as u32;
    let unroll = 4;
    let nLimit = nbSymbols as core::ffi::c_int - unroll + 1;
    for n in 0..tableLog as usize + 1 {
        let curr = nextRankStart;
        nextRankStart += wksp.rankVal[n];
        wksp.rankStart[n] = curr;
    }

    let mut n = 0;
    while n < nLimit {
        for u in 0..unroll {
            let w = usize::from(wksp.huffWeight[(n + u) as usize]);

            wksp.symbols[wksp.rankStart[w] as usize] = (n + u) as u8;
            wksp.rankStart[w] += 1;
        }
        n += unroll;
    }

    while n < nbSymbols as core::ffi::c_int {
        let w = usize::from(wksp.huffWeight[n as usize]);

        wksp.symbols[wksp.rankStart[w] as usize] = n as u8;
        wksp.rankStart[w] += 1;

        n += 1;
    }

    // fill DTable
    //
    // We fill all entries of each weight in order.
    // That way length is a constant for each iteration of the outer loop.
    // We can switch based on the length to a different inner loop which is
    // optimized for that particular case.
    let mut symbol = wksp.rankVal[0] as usize;
    let mut rankStart = 0;
    for w_1 in 1..tableLog.wrapping_add(1) {
        let symbolCount = wksp.rankVal[w_1 as usize] as usize;
        let length = (1) << w_1 >> 1;
        let mut dt = dt[rankStart..][..length * symbolCount as usize].chunks_exact_mut(length);
        let nbBits = tableLog.wrapping_add(1).wrapping_sub(w_1) as u8;

        // FIXME: zstd unrolls this loop for low values of `length` (a power of 2).
        // we should investigate whether that is beneficial here.
        for (s, chunk) in dt.enumerate() {
            let byte = wksp.symbols[symbol + s];
            chunk.fill(HUF_DEltX1 { nbBits, byte });
        }

        symbol += symbolCount;
        rankStart += symbolCount * length;
    }

    iSize
}

#[inline(always)]
fn HUF_decodeSymbolX1(Dstream: &mut BIT_DStream_t, dt: &[HUF_DEltX1; 4096], dtLog: u32) -> u8 {
    let HUF_DEltX1 { byte, nbBits, .. } = dt[Dstream.look_bits_fast(dtLog)];
    Dstream.skip_bits(u32::from(nbBits));
    byte
}

#[inline(always)]
unsafe fn HUF_decodeStreamX1(
    mut p: *mut u8,
    bitDPtr: &mut BIT_DStream_t,
    pEnd: *mut u8,
    dt: &[HUF_DEltX1; 4096],
    dtLog: u32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) > 3 {
        while bitDPtr.reload() == StreamStatus::Unfinished && p < pEnd.offset(-(3)) {
            if cfg!(target_pointer_width = "64") {
                let fresh17 = p;
                p = p.offset(1);
                *fresh17 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                let fresh18 = p;
                p = p.offset(1);
                *fresh18 = HUF_decodeSymbolX1(bitDPtr, dt, dtLog);
            }
            if cfg!(target_pointer_width = "64") {
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

    if cfg!(target_pointer_width = "32") {
        while bitDPtr.reload() == StreamStatus::Unfinished && p < pEnd {
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
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    let mut op = dst as *mut u8;
    let oend = ZSTD_maybeNullPtrAdd(op as *mut core::ffi::c_void, dstSize as ptrdiff_t) as *mut u8;
    let dt = DTable.data.as_x1();
    let dtd = DTable.description;
    let dtLog = dtd.tableLog as u32;

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
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    if src.len() < 10 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    if dstSize < 6 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let ostart = dst as *mut u8;
    let oend = ostart.offset(dstSize as isize);
    let olimit = oend.offset(-(3));

    let [b0, b1, b2, b3, b4, b5, ..] = *src else {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    };

    let length1 = usize::from(u16::from_le_bytes([b0, b1]));
    let length2 = usize::from(u16::from_le_bytes([b2, b3]));
    let length3 = usize::from(u16::from_le_bytes([b4, b5]));

    if 6 + length1 + length2 + length3 > src.len() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let istart1 = &src[6..][..length1];
    let istart2 = &src[6 + length1..][..length2];
    let istart3 = &src[6 + length1 + length2..][..length3];
    let istart4 = &src[6 + length1 + length2 + length3..];

    let segmentSize = dstSize.div_ceil(4);
    let opStart2 = ostart.offset(segmentSize as isize);
    let opStart3 = opStart2.offset(segmentSize as isize);
    let opStart4 = opStart3.offset(segmentSize as isize);
    let mut op1 = ostart;
    let mut op2 = opStart2;
    let mut op3 = opStart3;
    let mut op4 = opStart4;
    let mut end_signal = true;

    if opStart4 > oend {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    let mut bitD1 = match BIT_DStream_t::new(istart1) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD2 = match BIT_DStream_t::new(istart2) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD3 = match BIT_DStream_t::new(istart3) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };
    let mut bitD4 = match BIT_DStream_t::new(istart4) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    let dt = DTable.data.as_x1();
    let dtLog = DTable.description.tableLog as u32;

    if oend.offset_from(op4) >= size_of::<size_t>() as isize {
        while end_signal && op4 < olimit {
            if cfg!(target_pointer_width = "64") {
                *op1 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
                op1 = op1.offset(1);

                *op2 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
                op2 = op2.offset(1);

                *op3 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
                op3 = op3.offset(1);

                *op4 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
                op4 = op4.offset(1);
            }

            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                *op1 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
                op1 = op1.offset(1);

                *op2 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
                op2 = op2.offset(1);

                *op3 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
                op3 = op3.offset(1);

                *op4 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
                op4 = op4.offset(1);
            }

            if cfg!(target_pointer_width = "64") {
                *op1 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
                op1 = op1.offset(1);

                *op2 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
                op2 = op2.offset(1);

                *op3 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
                op3 = op3.offset(1);

                *op4 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
                op4 = op4.offset(1);
            }

            *op1 = HUF_decodeSymbolX1(&mut bitD1, dt, dtLog);
            op1 = op1.offset(1);

            *op2 = HUF_decodeSymbolX1(&mut bitD2, dt, dtLog);
            op2 = op2.offset(1);

            *op3 = HUF_decodeSymbolX1(&mut bitD3, dt, dtLog);
            op3 = op3.offset(1);

            *op4 = HUF_decodeSymbolX1(&mut bitD4, dt, dtLog);
            op4 = op4.offset(1);

            end_signal &= bitD1.reload_fast() == StreamStatus::Unfinished;
            end_signal &= bitD2.reload_fast() == StreamStatus::Unfinished;
            end_signal &= bitD3.reload_fast() == StreamStatus::Unfinished;
            end_signal &= bitD4.reload_fast() == StreamStatus::Unfinished;
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
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, src, DTable)
}

unsafe fn HUF_decompress4X1_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, dstSize, src, DTable)
}

unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast_c_loop(
    args: &mut HUF_DecompressFastArgs,
) {
    let dtable = args.dt as *const u16;
    let oend = args.oend;
    let ilowest = args.ilowest;

    // Copy the arguments to local variables.
    let mut bits = args.bits;
    let mut ip = args.ip;
    let mut op = args.op;

    assert!(cfg!(target_endian = "little"));
    assert!(cfg!(target_pointer_width = "64"));

    'out: loop {
        /* Assert loop preconditions */
        if cfg!(debug_assertions) {
            for stream in 0..4 {
                assert!(op[stream] <= (if stream == 3 { oend } else { op[stream + 1] }));
                assert!(ip[stream] >= ilowest);
            }
        }

        /* Compute olimit */

        // Each iteration consumes up to 11 bits * 5 = 55 bits < 7 bytes per stream.
        let oiters = oend.offset_from(op[3]) / 5;
        let iiters = (ip[0]).offset_from(ilowest) / 7;
        let iters = Ord::min(oiters, iiters);
        let symbols = iters * 5;

        // We can simply check that op[3] < olimit, instead of checking all
        // of our bounds, since we can't hit the other bounds until we've run
        // iters iterations, which only happens when op[3] == olimit.
        let olimit = op[3].offset(symbols);

        /* Exit fast decoding loop once we reach the end. */
        if op[3] == olimit {
            break;
        }

        // Exit the decoding loop if any input pointer has crossed the
        // previous one. This indicates corruption, and a precondition
        // to our loop is that ip[i] >= ip[0].
        for stream in 1..4 {
            if ip[stream] < ip[stream - 1] {
                break 'out;
            }
        }

        if cfg!(debug_assertions) {
            for stream in 1..4 {
                assert!(ip[stream] >= ip[stream - 1]);
            }
        }

        macro_rules! HUF_4X1_DECODE_SYMBOL {
            ($stream:expr, $symbol:expr) => {
                let index = bits[$stream] >> 53;
                let entry = *dtable.add(index as usize);

                bits[$stream] <<= entry & 0x3F;
                op[$stream]
                    .offset($symbol)
                    .write(((entry >> 8) & 0xFF) as u8)
            };
        }

        macro_rules! HUF_4X_FOR_EACH_STREAM_WITH_VAR {
            ($mac:ident, $var:literal) => {
                $mac!(0, $var);
                $mac!(1, $var);
                $mac!(2, $var);
                $mac!(3, $var);
            };
        }

        macro_rules! HUF_4X1_RELOAD_STREAM {
            ($stream: expr) => {
                let ctz = bits[$stream].trailing_zeros();
                let nbBits = ctz & 7;
                let nbBytes = ctz >> 3;

                op[$stream] = op[$stream].add(5);
                ip[$stream] = ip[$stream].sub(nbBytes as usize);
                bits[$stream] = MEM_read64(ip[$stream] as *const core::ffi::c_void) | 1;
                bits[$stream] <<= nbBits;
            };
        }

        macro_rules! HUF_4X_FOR_EACH_STREAM {
            ($mac:ident ) => {
                $mac!(0);
                $mac!(1);
                $mac!(2);
                $mac!(3);
            };
        }

        /* Manually unroll the loop because compilers don't consistently
         * unroll the inner loops, which destroys performance.
         */
        loop {
            /* Decode 5 symbols in each of the 4 streams */
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X1_DECODE_SYMBOL, 0);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X1_DECODE_SYMBOL, 1);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X1_DECODE_SYMBOL, 2);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X1_DECODE_SYMBOL, 3);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X1_DECODE_SYMBOL, 4);

            /* Reload each of the 4 the bitstreams */
            HUF_4X_FOR_EACH_STREAM!(HUF_4X1_RELOAD_STREAM);

            if op[3] >= olimit {
                break;
            }
        }
    }

    // Save the final values of each of the state variables back to args.
    args.bits = bits;
    args.ip = ip;
    args.op = op;
}

pub type HUF_DecompressFastLoopFn = unsafe extern "C" fn(&mut HUF_DecompressFastArgs) -> ();
unsafe fn HUF_decompress4X1_usingDTable_internal_fast(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
    loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;

    let mut args = match HUF_DecompressFastArgs::new(dst, dstSize, src, DTable) {
        Ok(Some(args)) => args,
        Ok(None) => return 0,
        Err(e) => return e.to_error_code(),
    };

    assert!(args.ip[0] >= args.ilowest);
    loopFn(&mut args);

    // Our loop guarantees that ip[] >= ilowest and that we haven't overwritten any op[].
    let istart = src.as_ptr();
    assert!(args.ip[0] >= istart);
    assert!(args.ip[0] >= istart);
    assert!(args.ip[1] >= istart);
    assert!(args.ip[2] >= istart);
    assert!(args.ip[3] >= istart);
    assert!(args.op[3] <= oend);

    assert_eq!(istart, args.ilowest);
    assert_eq!(istart.add(6), args.iend[0]);

    let segmentSize = dstSize.div_ceil(4);
    let mut segmentEnd = dst as *mut u8;

    // Finish bit streams one by one.
    for (i, op) in args.op.iter().copied().enumerate() {
        if segmentSize <= oend.offset_from(segmentEnd) as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }

        let mut bit = match init_remaining_dstream(&args, i, segmentEnd) {
            Ok(v) => v,
            Err(e) => return e.to_error_code(),
        };

        // Decompress and validate that we've produced exactly the expected length.
        let length = HUF_decodeStreamX1(
            op,
            &mut bit,
            segmentEnd,
            DTable.data.as_x1(),
            HUF_DECODER_FAST_TABLELOG as u32,
        );

        if op.add(length as usize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
    }

    dstSize
}

unsafe fn HUF_decompress1X1_usingDTable_internal_bmi2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, src, DTable)
}

unsafe fn HUF_decompress1X1_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, dstSize, src, DTable)
}

unsafe fn HUF_decompress1X1_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        HUF_decompress1X1_usingDTable_internal_bmi2(dst, dstSize, src, DTable)
    } else {
        HUF_decompress1X1_usingDTable_internal_default(dst, dstSize, src, DTable)
    }
}

unsafe fn HUF_decompress4X1_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        let loopFn = match flags & HUF_flags_disableAsm as i32 {
            0 => HUF_decompress4X1_usingDTable_internal_fast_asm_loop as HUF_DecompressFastLoopFn,
            _ => HUF_decompress4X1_usingDTable_internal_fast_c_loop as HUF_DecompressFastLoopFn,
        };

        if HUF_ENABLE_FAST_DECODE != 0 && flags & HUF_flags_disableFast as core::ffi::c_int == 0 {
            let ret =
                HUF_decompress4X1_usingDTable_internal_fast(dst, dstSize, src, DTable, loopFn);
            if ret != 0 {
                return ret;
            }
        }

        HUF_decompress4X1_usingDTable_internal_bmi2(dst, dstSize, src, DTable)
    } else {
        HUF_decompress4X1_usingDTable_internal_default(dst, dstSize, src, DTable)
    }
}

unsafe fn HUF_decompress4X1_DCtx_wksp(
    dctx: &mut DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    let hSize = HUF_readDTableX1_wksp(dctx, src, workSpace, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize as usize >= src.len() {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    HUF_decompress4X1_usingDTable_internal(dst, dstSize, &src[hSize as usize..], dctx, flags)
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

fn HUF_fillDTableX2ForWeight(
    mut DTableRank: &mut [HUF_DEltX2],
    sorted_symbols: &[sortedSymbol_t],
    nbBits: u32,
    tableLog: u32,
    baseSeq: u16,
    level: core::ffi::c_int,
) {
    let length = (1) << (tableLog.wrapping_sub(nbBits) & 0x1f);
    let chunks = DTableRank[..sorted_symbols.len() * length as usize].chunks_exact_mut(length);

    for (sorted_symbol, chunk) in sorted_symbols.iter().zip(chunks) {
        let DElt = HUF_buildDEltX2(sorted_symbol.symbol, nbBits, baseSeq, level);
        chunk.fill(DElt);
    }
}

fn HUF_fillDTableX2Level2(
    DTable: &mut [HUF_DEltX2],
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
        let length = 1 << (targetLog.wrapping_sub(consumedBits) & 0x1f);
        let elem = HUF_DEltX2::from_u32(HUF_buildDEltX2U32(baseSeq as u32, consumedBits, 0, 1));
        let skipSize = rankVal[minWeight as usize];
        match length {
            2 => {
                DTable[..2].fill(elem);
            }
            4 => {
                DTable[..4].fill(elem);
            }
            _ => {
                for i in (0..skipSize as usize).step_by(8) {
                    DTable[i..][..8].fill(elem);
                }
            }
        }
    }

    for w in minWeight as usize..maxWeight1 as usize {
        let nbBits = nbBitsBaseline.wrapping_sub(w as u32);
        let totalBits = nbBits.wrapping_add(consumedBits);

        HUF_fillDTableX2ForWeight(
            &mut DTable[rankVal[w] as usize..],
            &sortedSymbols[rankStart[w] as usize..rankStart[w + 1] as usize],
            totalBits,
            targetLog,
            baseSeq,
            2,
        );
    }
}

fn HUF_fillDTableX2(
    DTable: &mut [HUF_DEltX2; 4096],
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
                    &mut DTable[start as usize..],
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
                &mut DTable[rankVal[w as usize] as usize..],
                &sortedList[range],
                nbBits,
                targetLog,
                0,
                1,
            );
        }
    }
}

pub fn HUF_readDTableX2_wksp(
    DTable: &mut DTable,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    let mut dtd = DTable.description;

    let mut tableLog: u32 = 0;
    let mut nbSymbols: u32 = 0;
    let mut maxTableLog = dtd.maxTableLog as u32;
    let mut iSize: size_t = 0;

    let dt = DTable.data.as_x2_mut();
    let wksp = workSpace.as_x2_mut();

    wksp.rankStats.fill(0);
    wksp.rankStart0.fill(0);
    let rankStart = &mut wksp.rankStart0[1..];

    if maxTableLog > HUF_TABLELOG_MAX as u32 {
        return -(ZSTD_error_tableLog_tooLarge as core::ffi::c_int) as size_t;
    }

    iSize = HUF_readStats_wksp(
        &mut wksp.weightList,
        (HUF_SYMBOLVALUE_MAX + 1) as size_t,
        &mut wksp.rankStats,
        &mut nbSymbols,
        &mut tableLog,
        src,
        &mut wksp.calleeWksp,
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

    /* find maxWeight */
    let mut maxW: u32 = tableLog;
    while wksp.rankStats[maxW as usize] == 0 {
        maxW = maxW.wrapping_sub(1);
    }

    /* Get start index of each weight */
    let mut nextRankStart = 0u32;
    for w in 1..maxW + 1 {
        let mut curr = nextRankStart;
        nextRankStart += wksp.rankStats[w as usize];
        rankStart[w as usize] = curr;
    }

    rankStart[0] = nextRankStart;
    rankStart[maxW.wrapping_add(1) as usize] = nextRankStart;

    /* sort symbols by weight */
    for s in 0..nbSymbols {
        let w = usize::from(wksp.weightList[s as usize]);
        let r = rankStart[w];
        rankStart[w] += 1;
        wksp.sortedSymbol[r as usize].symbol = s as u8;
    }

    /* forget 0w symbols; this is beginning of weight(1) */
    rankStart[0] = 0;

    /* Build rankVal */
    let rescale = maxTableLog.wrapping_sub(tableLog).wrapping_sub(1) as core::ffi::c_int;
    let mut nextRankVal = 0 as core::ffi::c_int as u32;

    for w_1 in 1..maxW.wrapping_add(1) {
        let mut curr_0 = nextRankVal;
        nextRankVal = nextRankVal
            .wrapping_add(wksp.rankStats[w_1 as usize] << w_1.wrapping_add(rescale as u32));

        wksp.rankVal[0][w_1 as usize] = curr_0;
    }

    let minBits = tableLog.wrapping_add(1).wrapping_sub(maxW);
    let mut consumed: u32 = 0;
    consumed = minBits;
    while consumed < maxTableLog.wrapping_sub(minBits).wrapping_add(1) {
        for w_2 in 0..maxW.wrapping_add(1) {
            wksp.rankVal[consumed as usize][w_2 as usize] =
                wksp.rankVal[0][w_2 as usize] >> consumed;
        }
        consumed = consumed.wrapping_add(1);
    }

    HUF_fillDTableX2(
        dt,
        maxTableLog,
        &mut wksp.sortedSymbol,
        &mut wksp.rankStart0,
        &mut wksp.rankVal,
        maxW,
        tableLog.wrapping_add(1),
    );

    dtd.tableLog = maxTableLog as u8;
    dtd.tableType = 1;

    DTable.description = dtd;

    iSize
}

#[inline(always)]
unsafe fn HUF_decodeSymbolX2(
    op: *mut u8,
    DStream: &mut BIT_DStream_t,
    dt: &[HUF_DEltX2; 4096],
    dtLog: u32,
) -> u32 {
    let HUF_DEltX2 {
        sequence,
        nbBits,
        length,
    } = dt[DStream.look_bits_fast(dtLog)];

    op.cast::<u16>().write_unaligned(sequence);
    DStream.skip_bits(nbBits as u32);

    length as u32
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
        < (size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
    {
        DStream.skip_bits((*dt.offset(val as isize)).nbBits as u32);
        if (*DStream).bitsConsumed as core::ffi::c_ulong
            > (size_of::<BitContainerType>() as core::ffi::c_ulong).wrapping_mul(8)
        {
            (*DStream).bitsConsumed = (size_of::<BitContainerType>() as core::ffi::c_ulong)
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
    dt: &[HUF_DEltX2; 4096],
    dtLog: u32,
) -> size_t {
    let pStart = p;
    if pEnd.offset_from(p) as core::ffi::c_long as size_t
        >= ::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong
    {
        if dtLog <= 11 && cfg!(target_pointer_width = "64") {
            while (bitDPtr.reload() == StreamStatus::Unfinished) as core::ffi::c_int
                & (p < pEnd.offset(-(9))) as core::ffi::c_int
                != 0
            {
                p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
                p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
                p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
                p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
                p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
            }
        } else {
            while (bitDPtr.reload() == StreamStatus::Unfinished) as core::ffi::c_int
                & (p < pEnd.offset(
                    -((::core::mem::size_of::<BitContainerType>() as core::ffi::c_ulong)
                        .wrapping_sub(1) as isize),
                )) as core::ffi::c_int
                != 0
            {
                if cfg!(target_pointer_width = "64") {
                    p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
                }
                if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                    p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
                }
                if cfg!(target_pointer_width = "64") {
                    p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
                }
                p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
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
            p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
        }
        while p <= pEnd.offset(-(2)) {
            p = p.offset(HUF_decodeSymbolX2(p, bitDPtr, dt, dtLog) as isize);
        }
    }
    if p < pEnd {
        p = p.offset(HUF_decodeLastSymbolX2(
            p as *mut core::ffi::c_void,
            bitDPtr,
            dt.as_ptr(),
            dtLog,
        ) as isize);
    }
    p.offset_from(pStart) as core::ffi::c_long as size_t
}

#[inline(always)]
unsafe fn HUF_decompress1X2_usingDTable_internal_body(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    let mut bitD = match BIT_DStream_t::new(src) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    let ostart = dst as *mut u8;
    let oend =
        ZSTD_maybeNullPtrAdd(ostart as *mut core::ffi::c_void, dstSize as ptrdiff_t) as *mut u8;
    let dt = DTable.data.as_x2();
    let dtd = DTable.description;
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
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    let cSrcSize = src.len() as size_t;
    let cSrc = src.as_ptr();

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
    let dtd = DTable.description;
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

    let dt = DTable.data.as_x2();

    if oend.offset_from(op4) as core::ffi::c_long as size_t
        >= ::core::mem::size_of::<size_t>() as core::ffi::c_ulong
    {
        while endSignal & (op4 < olimit) as core::ffi::c_int as u32 != 0 {
            if cfg!(target_pointer_width = "64") {
                op1 = op1.offset(HUF_decodeSymbolX2(op1, &mut bitD1, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                op1 = op1.offset(HUF_decodeSymbolX2(op1, &mut bitD1, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") {
                op1 = op1.offset(HUF_decodeSymbolX2(op1, &mut bitD1, dt, dtLog) as isize);
            }
            op1 = op1.offset(HUF_decodeSymbolX2(op1, &mut bitD1, dt, dtLog) as isize);
            if cfg!(target_pointer_width = "64") {
                op2 = op2.offset(HUF_decodeSymbolX2(op2, &mut bitD2, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                op2 = op2.offset(HUF_decodeSymbolX2(op2, &mut bitD2, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") {
                op2 = op2.offset(HUF_decodeSymbolX2(op2, &mut bitD2, dt, dtLog) as isize);
            }
            op2 = op2.offset(HUF_decodeSymbolX2(op2, &mut bitD2, dt, dtLog) as isize);
            endSignal &= (bitD1.reload_fast() == StreamStatus::Unfinished) as u32;
            endSignal &= (bitD2.reload_fast() == StreamStatus::Unfinished) as u32;
            if cfg!(target_pointer_width = "64") {
                op3 = op3.offset(HUF_decodeSymbolX2(op3, &mut bitD3, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                op3 = op3.offset(HUF_decodeSymbolX2(op3, &mut bitD3, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") {
                op3 = op3.offset(HUF_decodeSymbolX2(op3, &mut bitD3, dt, dtLog) as isize);
            }
            op3 = op3.offset(HUF_decodeSymbolX2(op3, &mut bitD3, dt, dtLog) as isize);
            if cfg!(target_pointer_width = "64") {
                op4 = op4.offset(HUF_decodeSymbolX2(op4, &mut bitD4, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                op4 = op4.offset(HUF_decodeSymbolX2(op4, &mut bitD4, dt, dtLog) as isize);
            }
            if cfg!(target_pointer_width = "64") {
                op4 = op4.offset(HUF_decodeSymbolX2(op4, &mut bitD4, dt, dtLog) as isize);
            }
            op4 = op4.offset(HUF_decodeSymbolX2(op4, &mut bitD4, dt, dtLog) as isize);
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
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, src, DTable)
}

unsafe fn HUF_decompress4X2_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, dstSize, src, DTable)
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
    src: &[u8],
    DTable: &DTable,
    loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let oend = ZSTD_maybeNullPtrAdd(dst, dstSize as ptrdiff_t) as *mut u8;

    let mut args = match HUF_DecompressFastArgs::new(dst, dstSize, src, DTable) {
        Ok(Some(args)) => args,
        Ok(None) => return 0,
        Err(e) => return e.to_error_code(),
    };

    assert!(args.ip[0] >= args.ilowest);
    loopFn(&mut args);

    // note : op4 already verified within main loop.
    let ilowest = src.as_ptr();
    assert!(args.ip[0] >= ilowest);
    assert!(args.ip[1] >= ilowest);
    assert!(args.ip[2] >= ilowest);
    assert!(args.ip[3] >= ilowest);
    assert!(args.op[3] <= oend);

    assert_eq!(ilowest, args.ilowest);
    assert_eq!(ilowest.add(6), args.iend[0]);

    let segmentSize = dstSize.div_ceil(4);
    let mut segmentEnd = dst as *mut u8;

    for (i, op) in args.op.iter().copied().enumerate() {
        if segmentSize <= oend.offset_from(segmentEnd) as size_t {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }

        let mut bit = match init_remaining_dstream(&args, i, segmentEnd) {
            Ok(v) => v,
            Err(e) => return e.to_error_code(),
        };

        let length = HUF_decodeStreamX2(
            op,
            &mut bit,
            segmentEnd,
            DTable.data.as_x2(),
            HUF_DECODER_FAST_TABLELOG as u32,
        );

        if op.add(length as usize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
    }

    dstSize
}

unsafe fn HUF_decompress4X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        let loopFn = match flags & HUF_flags_disableAsm as core::ffi::c_int {
            0 => HUF_decompress4X2_usingDTable_internal_fast_asm_loop as HUF_DecompressFastLoopFn,
            _ => HUF_decompress4X2_usingDTable_internal_fast_c_loop as HUF_DecompressFastLoopFn,
        };

        if HUF_ENABLE_FAST_DECODE != 0 && flags & HUF_flags_disableFast as core::ffi::c_int == 0 {
            let ret =
                HUF_decompress4X2_usingDTable_internal_fast(dst, dstSize, src, DTable, loopFn);
            if ret != 0 {
                return ret;
            }
        }

        HUF_decompress4X2_usingDTable_internal_bmi2(dst, dstSize, src, DTable)
    } else {
        HUF_decompress4X2_usingDTable_internal_default(dst, dstSize, src, DTable)
    }
}

unsafe fn HUF_decompress1X2_usingDTable_internal_bmi2(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, src, DTable)
}
unsafe fn HUF_decompress1X2_usingDTable_internal_default(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, dstSize, src, DTable)
}

unsafe fn HUF_decompress1X2_usingDTable_internal(
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        HUF_decompress1X2_usingDTable_internal_bmi2(dst, dstSize, src, DTable)
    } else {
        HUF_decompress1X2_usingDTable_internal_default(dst, dstSize, src, DTable)
    }
}

pub unsafe fn HUF_decompress1X2_DCtx_wksp(
    dctx: &mut DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    let hSize = HUF_readDTableX2_wksp(dctx, src, workSpace, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize as usize >= src.len() {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    HUF_decompress1X2_usingDTable_internal(dst, dstSize, &src[hSize as usize..], dctx, flags)
}

unsafe fn HUF_decompress4X2_DCtx_wksp(
    dctx: &mut DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    let hSize = HUF_readDTableX2_wksp(dctx, src, workSpace, flags);
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize as usize >= src.len() {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    HUF_decompress4X2_usingDTable_internal(dst, dstSize, &src[hSize as usize..], dctx, flags)
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
    dctx: &mut DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    cSrcSize: size_t,
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    if dstSize == 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if cSrcSize > dstSize {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if cSrcSize == dstSize {
        libc::memcpy(dst, src.as_ptr().cast(), dstSize as libc::size_t);
        return dstSize;
    }
    if cSrcSize == 1 {
        ptr::write_bytes(dst, src[0], dstSize as usize);
        return dstSize;
    }

    match HUF_selectDecoder(dstSize, cSrcSize) {
        Decoder::A1 => HUF_decompress1X1_DCtx_wksp(dctx, dst, dstSize, src, workSpace, flags),
        Decoder::A2 => HUF_decompress1X2_DCtx_wksp(dctx, dst, dstSize, src, workSpace, flags),
    }
}

pub unsafe fn HUF_decompress1X_usingDTable(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if DTable.description.tableType != 0 {
        HUF_decompress1X2_usingDTable_internal(dst, maxDstSize, src, DTable, flags)
    } else {
        HUF_decompress1X1_usingDTable_internal(dst, maxDstSize, src, DTable, flags)
    }
}

pub unsafe fn HUF_decompress1X1_DCtx_wksp(
    dctx: &mut DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    let hSize = { HUF_readDTableX1_wksp(dctx, src, workSpace, flags) };
    if ERR_isError(hSize) != 0 {
        return hSize;
    }
    if hSize as usize >= src.len() {
        return -(ZSTD_error_srcSize_wrong as core::ffi::c_int) as size_t;
    }

    HUF_decompress1X1_usingDTable_internal(dst, dstSize, &src[hSize as usize..], dctx, flags)
}

pub unsafe fn HUF_decompress4X_usingDTable(
    dst: *mut core::ffi::c_void,
    maxDstSize: size_t,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if DTable.description.tableType != 0 {
        HUF_decompress4X2_usingDTable_internal(dst, maxDstSize, src, DTable, flags)
    } else {
        HUF_decompress4X1_usingDTable_internal(dst, maxDstSize, src, DTable, flags)
    }
}

pub unsafe fn HUF_decompress4X_hufOnly_wksp(
    dctx: &mut DTable,
    dst: *mut core::ffi::c_void,
    dstSize: size_t,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    if dstSize == 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if src.is_empty() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    match HUF_selectDecoder(dstSize, src.len() as size_t) {
        Decoder::A1 => HUF_decompress4X1_DCtx_wksp(dctx, dst, dstSize, src, workSpace, flags),
        Decoder::A2 => HUF_decompress4X2_DCtx_wksp(dctx, dst, dstSize, src, workSpace, flags),
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
