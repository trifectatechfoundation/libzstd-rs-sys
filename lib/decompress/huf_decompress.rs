use core::ptr;

use crate::lib::common::bitstream::{BIT_DStream_t, BitContainerType, StreamStatus};
use crate::lib::common::entropy_common::HUF_readStats_wksp;
use crate::lib::common::error_private::ERR_isError;
use crate::lib::common::fse_decompress::Error;
use crate::lib::common::mem::{MEM_isLittleEndian, MEM_read64, MEM_readLEST, MEM_write16};
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

    fn as_symbols(&self) -> &[u16; 2 * 4096] {
        unsafe { core::mem::transmute(&self.data) }
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HUF_DecompressFastArgs<'a> {
    pub ip: [*const u8; 4],
    pub op: [*mut u8; 4],
    pub bits: [u64; 4],
    pub dt: &'a DTableData,
    pub ilowest: *const u8,
    pub oend: *mut u8,
    pub iend: [*const u8; 4],
}

pub const HUF_TABLELOG_MAX: core::ffi::c_int = 12;
pub const HUF_SYMBOLVALUE_MAX: core::ffi::c_int = 255;
pub const HUF_DECODER_FAST_TABLELOG: core::ffi::c_int = 11;
pub const HUF_ENABLE_FAST_DECODE: core::ffi::c_int = 1;
pub const HUF_isError: fn(size_t) -> core::ffi::c_uint = ERR_isError;

unsafe fn HUF_initFastDStream(mut ip: *const u8) -> size_t {
    let lastByte = *ip.offset(7);
    let bitsConsumed = (if lastByte as core::ffi::c_int != 0 {
        (8 as core::ffi::c_uint).wrapping_sub(lastByte.ilog2())
    } else {
        0
    }) as size_t;
    let value = MEM_readLEST(ip as *const core::ffi::c_void) | 1;
    value << bitsConsumed
}

impl<'a> HUF_DecompressFastArgs<'a> {
    unsafe fn new(
        mut dst: Writer<'_>,
        src: &[u8],
        DTable: &'a DTable,
    ) -> Result<Option<Self>, Error> {
        // The fast decoding loop assumes 64-bit little-endian.
        if cfg!(target_endian = "big") || cfg!(target_pointer_width = "32") {
            return Ok(None);
        }

        if dst.is_empty() {
            return Ok(None);
        }

        // strict minimum : jump table + 1 byte per stream.
        let [b0, b1, b2, b3, b4, b5, _, _, _, _, ..] = *src else {
            return Err(Error::corruption_detected);
        };

        // Must have at least 8 bytes per stream because we don't handle initializing smaller bit containers.
        // If table log is not correct at this point, fallback to the old decoder.
        // On small inputs we don't have enough data to trigger the fast loop, so use the old decoder.
        let dtLog = DTable.description.tableLog as u32;
        if dtLog != HUF_DECODER_FAST_TABLELOG as u32 {
            return Ok(None);
        }

        let length1 = usize::from(u16::from_le_bytes([b0, b1]));
        let length2 = usize::from(u16::from_le_bytes([b2, b3]));
        let length3 = usize::from(u16::from_le_bytes([b4, b5]));
        let length4 = src.len().wrapping_sub(6 + length1 + length2 + length3);

        if 6 + length1 + length2 + length3 > src.len() {
            return Err(Error::corruption_detected);
        }

        let istart = src.as_ptr();
        let mut iend = [core::ptr::null(); 4];
        iend[0] = istart.add(6); /* jumpTable */
        iend[1] = iend[0].add(length1 as usize);
        iend[2] = iend[1].add(length2 as usize);
        iend[3] = iend[2].add(length3 as usize);

        // HUF_initFastDStream() requires this, and this small of an input won't benefit from the ASM loop anyways.
        if length1 < 8 || length2 < 8 || length3 < 8 || length4 < 8 {
            return Ok(None);
        }

        /* ip[] contains the position that is currently loaded into bits[]. */
        let mut ip = [core::ptr::null(); 4];
        ip[0] = iend[1].sub(size_of::<u64>());
        ip[1] = iend[2].sub(size_of::<u64>());
        ip[2] = iend[3].sub(size_of::<u64>());
        ip[3] = src.as_ptr().add(src.len() as usize - size_of::<u64>());

        /* op[] contains the output pointers. */
        let mut op = [core::ptr::null_mut(); 4];
        op[0] = dst.as_mut_ptr();
        op[1] = op[0].add(dst.capacity().div_ceil(4));
        op[2] = op[1].add(dst.capacity().div_ceil(4));
        op[3] = op[2].add(dst.capacity().div_ceil(4));

        // No point to call the ASM loop for tiny outputs.
        if op[3] >= dst.as_mut_ptr_range().end {
            return Ok(None);
        }

        // bits[] is the bit container.
        //
        // It is read from the MSB down to the LSB.
        // It is shifted left as it is read, and zeros are
        // shifted in. After the lowest valid bit a 1 is
        // set, so that CountTrailingZeros(bits[]) can be used
        // to count how many bits we've consumed.
        let bits = ip.map(|v| HUF_initFastDStream(v));

        // The decoders must be sure to never read beyond ilowest.
        // This is lower than iend[0], but allowing decoders to read
        // down to ilowest can allow an extra iteration or two in the
        // fast loop.
        let args = Self {
            ip,
            op,
            bits,
            dt: &DTable.data,
            ilowest: istart,
            oend: dst.as_mut_ptr_range().end,
            iend,
        };

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
    let mut nextRankStart = 0u32;
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
fn HUF_decodeStreamX1(
    mut p: Writer<'_>,
    bitDPtr: &mut BIT_DStream_t,
    dt: &[HUF_DEltX1; 4096],
    dtLog: u32,
) -> size_t {
    let capacity = p.capacity();

    if p.capacity() >= 4 {
        while bitDPtr.reload() == StreamStatus::Unfinished && p.capacity() >= 4 {
            if cfg!(target_pointer_width = "64") {
                p.write_u8(HUF_decodeSymbolX1(bitDPtr, dt, dtLog));
            }
            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                p.write_u8(HUF_decodeSymbolX1(bitDPtr, dt, dtLog));
            }
            if cfg!(target_pointer_width = "64") {
                p.write_u8(HUF_decodeSymbolX1(bitDPtr, dt, dtLog));
            }
            p.write_u8(HUF_decodeSymbolX1(bitDPtr, dt, dtLog));
        }
    } else {
        bitDPtr.reload();
    }

    if cfg!(target_pointer_width = "32") {
        while bitDPtr.reload() == StreamStatus::Unfinished && !p.is_empty() {
            p.write_u8(HUF_decodeSymbolX1(bitDPtr, dt, dtLog));
        }
    }

    while !p.is_empty() {
        p.write_u8(HUF_decodeSymbolX1(bitDPtr, dt, dtLog));
    }

    (capacity - p.capacity()) as size_t
}

#[inline(always)]
fn HUF_decompress1X1_usingDTable_internal_body(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    let dt = DTable.data.as_x1();
    let dtd = DTable.description;
    let dtLog = dtd.tableLog as u32;

    let mut bitD = match BIT_DStream_t::new(src) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    HUF_decodeStreamX1(dst.subslice(..), &mut bitD, dt, dtLog);

    if !bitD.is_empty() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    dst.capacity() as size_t
}

#[inline(always)]
fn HUF_decompress4X1_usingDTable_internal_body(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    // strict minimum : jump table + 1 byte per stream.
    let [b0, b1, b2, b3, b4, b5, _, _, _, _, ..] = *src else {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    };

    if dst.capacity() < 6 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

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

    let Some((mut w1, mut w2, mut w3, mut w4)) = dst.quarter() else {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    };

    let mut end_signal = true;

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

    if w4.capacity() >= size_of::<size_t>() {
        while end_signal && w4.capacity() >= 4 {
            if cfg!(target_pointer_width = "64") {
                w1.write_u8(HUF_decodeSymbolX1(&mut bitD1, dt, dtLog));
                w2.write_u8(HUF_decodeSymbolX1(&mut bitD2, dt, dtLog));
                w3.write_u8(HUF_decodeSymbolX1(&mut bitD3, dt, dtLog));
                w4.write_u8(HUF_decodeSymbolX1(&mut bitD4, dt, dtLog));
            }

            if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
                w1.write_u8(HUF_decodeSymbolX1(&mut bitD1, dt, dtLog));
                w2.write_u8(HUF_decodeSymbolX1(&mut bitD2, dt, dtLog));
                w3.write_u8(HUF_decodeSymbolX1(&mut bitD3, dt, dtLog));
                w4.write_u8(HUF_decodeSymbolX1(&mut bitD4, dt, dtLog));
            }

            if cfg!(target_pointer_width = "64") {
                w1.write_u8(HUF_decodeSymbolX1(&mut bitD1, dt, dtLog));
                w2.write_u8(HUF_decodeSymbolX1(&mut bitD2, dt, dtLog));
                w3.write_u8(HUF_decodeSymbolX1(&mut bitD3, dt, dtLog));
                w4.write_u8(HUF_decodeSymbolX1(&mut bitD4, dt, dtLog));
            }

            w1.write_u8(HUF_decodeSymbolX1(&mut bitD1, dt, dtLog));
            w2.write_u8(HUF_decodeSymbolX1(&mut bitD2, dt, dtLog));
            w3.write_u8(HUF_decodeSymbolX1(&mut bitD3, dt, dtLog));
            w4.write_u8(HUF_decodeSymbolX1(&mut bitD4, dt, dtLog));

            end_signal &= bitD1.reload_fast() == StreamStatus::Unfinished;
            end_signal &= bitD2.reload_fast() == StreamStatus::Unfinished;
            end_signal &= bitD3.reload_fast() == StreamStatus::Unfinished;
            end_signal &= bitD4.reload_fast() == StreamStatus::Unfinished;
        }
    }

    HUF_decodeStreamX1(w1, &mut bitD1, dt, dtLog);
    HUF_decodeStreamX1(w2, &mut bitD2, dt, dtLog);
    HUF_decodeStreamX1(w3, &mut bitD3, dt, dtLog);
    HUF_decodeStreamX1(w4, &mut bitD4, dt, dtLog);

    if !(bitD1.is_empty() && bitD2.is_empty() && bitD3.is_empty() && bitD4.is_empty()) {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    dst.capacity() as size_t
}

fn HUF_decompress4X1_usingDTable_internal_bmi2(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, src, DTable)
}

fn HUF_decompress4X1_usingDTable_internal_default(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X1_usingDTable_internal_body(dst, src, DTable)
}

macro_rules! HUF_4X_FOR_EACH_STREAM_WITH_VAR {
    ($mac:ident, $var:literal) => {
        $mac!(0, $var);
        $mac!(1, $var);
        $mac!(2, $var);
        $mac!(3, $var);
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

unsafe extern "C" fn HUF_decompress4X1_usingDTable_internal_fast_c_loop(
    args: &mut HUF_DecompressFastArgs,
) {
    let dtable = args.dt.as_symbols();
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
                let entry = dtable[index as usize];

                bits[$stream] <<= entry & 0x3F;
                op[$stream]
                    .offset($symbol)
                    .write(((entry >> 8) & 0xFF) as u8)
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
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
    loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let oend = dst.as_mut_ptr_range().end;

    let mut args = match HUF_DecompressFastArgs::new(dst.subslice(..), src, DTable) {
        Ok(Some(args)) => args,
        Ok(None) => return 0,
        Err(e) => return e.to_error_code(),
    };

    assert!(args.ip[0] >= args.ilowest);
    loopFn(&mut args);

    // Our loop guarantees that ip[] >= ilowest and that we haven't overwritten any op[].
    let istart = src.as_ptr();
    assert!(args.ip[0] >= istart);
    assert!(args.ip[1] >= istart);
    assert!(args.ip[2] >= istart);
    assert!(args.ip[3] >= istart);
    assert!(args.op[3] <= oend);

    assert_eq!(istart, args.ilowest);
    assert_eq!(istart.add(6), args.iend[0]);

    let segmentSize = dst.capacity().div_ceil(4) as isize;
    let mut segmentEnd = dst.as_mut_ptr_range().start;

    // Finish bit streams one by one.
    for (i, op) in args.op.iter().copied().enumerate() {
        if segmentSize <= oend.offset_from(segmentEnd) {
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
            Writer::from_raw_parts(op, segmentEnd as usize - op as usize),
            &mut bit,
            DTable.data.as_x1(),
            HUF_DECODER_FAST_TABLELOG as u32,
        );

        if op.add(length as usize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
    }

    dst.capacity() as size_t
}

unsafe fn HUF_decompress1X1_usingDTable_internal_bmi2(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, src, DTable)
}

unsafe fn HUF_decompress1X1_usingDTable_internal_default(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X1_usingDTable_internal_body(dst, src, DTable)
}

unsafe fn HUF_decompress1X1_usingDTable_internal(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        HUF_decompress1X1_usingDTable_internal_bmi2(dst, src, DTable)
    } else {
        HUF_decompress1X1_usingDTable_internal_default(dst, src, DTable)
    }
}

unsafe fn HUF_decompress4X1_usingDTable_internal(
    mut dst: Writer<'_>,
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
                HUF_decompress4X1_usingDTable_internal_fast(dst.subslice(..), src, DTable, loopFn);
            if ret != 0 {
                return ret;
            }
        }

        HUF_decompress4X1_usingDTable_internal_bmi2(dst, src, DTable)
    } else {
        HUF_decompress4X1_usingDTable_internal_default(dst, src, DTable)
    }
}

unsafe fn HUF_decompress4X1_DCtx_wksp(
    dctx: &mut DTable,
    mut dst: Writer<'_>,
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

    HUF_decompress4X1_usingDTable_internal(dst, &src[hSize as usize..], dctx, flags)
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
    let mut nextRankVal = 0u32;

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
unsafe fn HUF_decodeSymbolX2_new(
    w: &mut Writer<'_>,
    DStream: &mut BIT_DStream_t,
    dt: &[HUF_DEltX2; 4096],
    dtLog: u32,
) {
    let HUF_DEltX2 {
        sequence,
        nbBits,
        length,
    } = dt[DStream.look_bits_fast(dtLog)];

    DStream.skip_bits(nbBits as u32);
    w.write_symbol_x2(sequence, length);
}

#[inline(always)]
unsafe fn HUF_decodeLastSymbolX2(
    w: &mut Writer<'_>,
    DStream: &mut BIT_DStream_t,
    dt: &[HUF_DEltX2; 4096],
    dtLog: u32,
) {
    let HUF_DEltX2 {
        sequence,
        nbBits,
        length,
    } = dt[DStream.look_bits_fast(dtLog)];

    w.write_u8(sequence.to_le_bytes()[0]);

    if length == 1 {
        DStream.skip_bits(u32::from(nbBits));
    } else if (DStream.bitsConsumed as usize) < size_of::<BitContainerType>() * 8 {
        DStream.skip_bits(u32::from(nbBits));
        DStream.bitsConsumed = Ord::min(
            DStream.bitsConsumed,
            (size_of::<BitContainerType>() * 8) as u32,
        );
    }
}

macro_rules! HUF_DECODE_SYMBOLX2_0 {
    ($($args:expr),*) => {
        HUF_decodeSymbolX2_new($($args),*)
    }
}

macro_rules! HUF_DECODE_SYMBOLX2_1 {
    ($($args:expr),*) => {
        if cfg!(target_pointer_width = "64") || HUF_TABLELOG_MAX <= 12 {
            HUF_decodeSymbolX2_new($($args),*)
        }
    }
}

macro_rules! HUF_DECODE_SYMBOLX2_2 {
    ($($args:expr),*) => {
        if cfg!(target_pointer_width = "64") {
            HUF_decodeSymbolX2_new($($args),*)
        }
    }
}

#[inline(always)]
unsafe fn HUF_decodeStreamX2(
    mut p: Writer<'_>,
    bitDPtr: &mut BIT_DStream_t,
    dt: &[HUF_DEltX2; 4096],
    dtLog: u32,
) -> size_t {
    let capacity = p.capacity();

    /* up to 8 symbols at a time */
    if p.capacity() >= size_of::<usize>() {
        if dtLog <= 11 && cfg!(target_pointer_width = "64") {
            /* up to 10 symbols at a time */
            while (bitDPtr.reload() == StreamStatus::Unfinished) && p.capacity() >= 10 {
                HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
            }
        } else {
            /* up to 8 symbols at a time */
            while bitDPtr.reload() == StreamStatus::Unfinished && p.capacity() >= size_of::<usize>()
            {
                HUF_DECODE_SYMBOLX2_2!(&mut p, bitDPtr, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut p, bitDPtr, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut p, bitDPtr, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
            }
        }
    } else {
        bitDPtr.reload();
    }

    /* closer to end : up to 2 symbols at a time */
    if p.capacity() >= 2 {
        while (bitDPtr.reload() == StreamStatus::Unfinished) && p.capacity() >= 2 {
            HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
        }

        while p.capacity() >= 2 {
            HUF_DECODE_SYMBOLX2_0!(&mut p, bitDPtr, dt, dtLog);
        }
    }

    if !p.is_empty() {
        HUF_decodeLastSymbolX2(&mut p, bitDPtr, dt, dtLog);
    }

    (capacity - p.capacity()) as size_t
}

#[inline(always)]
unsafe fn HUF_decompress1X2_usingDTable_internal_body(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    let mut bitD = match BIT_DStream_t::new(src) {
        Ok(v) => v,
        Err(e) => return e.to_error_code(),
    };

    let dt = DTable.data.as_x2();
    let dtd = DTable.description;
    HUF_decodeStreamX2(dst.subslice(..), &mut bitD, dt, dtd.tableLog as u32);
    if !bitD.is_empty() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    dst.capacity() as size_t
}

#[inline(always)]
unsafe fn HUF_decompress4X2_usingDTable_internal_body(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    // Strict minimum : jump table + 1 byte per stream.
    let [b0, b1, b2, b3, b4, b5, _, _, _, _, ..] = *src else {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    };

    // Stream 4-way split would not work.
    if dst.capacity() < 6 {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

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

    let Some((mut op1, mut op2, mut op3, mut op4)) = dst.quarter() else {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    };

    let mut end_signal = true;

    let dtLog = DTable.description.tableLog as u32;

    if op4.is_empty() {
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

    let dt = DTable.data.as_x2();

    /* 16-32 symbols per loop (4-8 symbols per stream) */
    if op4.capacity() >= size_of::<usize>() {
        while end_signal && op4.capacity() >= size_of::<usize>() {
            if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
                HUF_DECODE_SYMBOLX2_2!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op2, &mut bitD2, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op2, &mut bitD2, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op2, &mut bitD2, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op2, &mut bitD2, dt, dtLog);

                end_signal &= bitD1.reload_fast() == StreamStatus::Unfinished;
                end_signal &= bitD2.reload_fast() == StreamStatus::Unfinished;

                HUF_DECODE_SYMBOLX2_2!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op4, &mut bitD4, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op4, &mut bitD4, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op4, &mut bitD4, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op4, &mut bitD4, dt, dtLog);

                end_signal &= bitD3.reload_fast() == StreamStatus::Unfinished;
                end_signal &= bitD4.reload_fast() == StreamStatus::Unfinished;
            } else {
                HUF_DECODE_SYMBOLX2_2!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op2, &mut bitD2, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op4, &mut bitD4, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op2, &mut bitD2, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_1!(&mut op4, &mut bitD4, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op2, &mut bitD2, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_2!(&mut op4, &mut bitD4, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op1, &mut bitD1, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op2, &mut bitD2, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op3, &mut bitD3, dt, dtLog);
                HUF_DECODE_SYMBOLX2_0!(&mut op4, &mut bitD4, dt, dtLog);

                end_signal &= bitD1.reload_fast() == StreamStatus::Unfinished;
                end_signal &= bitD2.reload_fast() == StreamStatus::Unfinished;
                end_signal &= bitD3.reload_fast() == StreamStatus::Unfinished;
                end_signal &= bitD4.reload_fast() == StreamStatus::Unfinished;
            }
        }
    }

    // Check for corruption.
    // NOTE: these conditions do in fact trigger for invalid input. That is why currently
    // `Writer::write_symbol_x2` does not assert that it is in-bounds.
    if op1.ptr > op1.end {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op2.ptr > op2.end {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if op3.ptr > op3.end {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    // NOTE: op4 is already verified within main loop.

    // Finish bit streams one by one.
    HUF_decodeStreamX2(op1, &mut bitD1, dt, dtLog);
    HUF_decodeStreamX2(op2, &mut bitD2, dt, dtLog);
    HUF_decodeStreamX2(op3, &mut bitD3, dt, dtLog);
    HUF_decodeStreamX2(op4, &mut bitD4, dt, dtLog);

    // Check.
    if !(bitD1.is_empty() && bitD2.is_empty() && bitD3.is_empty() && bitD4.is_empty()) {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    // The decoded size.
    dst.capacity() as size_t
}

unsafe fn HUF_decompress4X2_usingDTable_internal_bmi2(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, src, DTable)
}

unsafe fn HUF_decompress4X2_usingDTable_internal_default(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress4X2_usingDTable_internal_body(dst, src, DTable)
}

unsafe extern "C" fn HUF_decompress4X2_usingDTable_internal_fast_c_loop(
    args: &mut HUF_DecompressFastArgs,
) {
    let dtable: &[HUF_DEltX2; 4096] = core::mem::transmute(args.dt);
    let ilowest = args.ilowest;

    let mut bits = args.bits;
    let mut ip = args.ip;
    let mut op = args.op;

    let mut oend: [*mut u8; 4] = [core::ptr::null_mut::<u8>(); 4];
    oend[0] = op[1];
    oend[1] = op[2];
    oend[2] = op[3];
    oend[3] = args.oend;

    'out: loop {
        let mut olimit = core::ptr::null_mut::<u8>();

        /* Assert loop preconditions */
        if cfg!(debug_assertions) {
            for stream in 0..4 {
                assert!(op[stream] <= oend[stream]);
                assert!(ip[stream] >= ilowest);
            }
        }

        /* Compute olimit */

        /* Each loop does 5 table lookups for each of the 4 streams.
         * Each table lookup consumes up to 11 bits of input, and produces
         * up to 2 bytes of output.
         */
        /* We can consume up to 7 bytes of input per iteration per stream.
         * We also know that each input pointer is >= ip[0]. So we can run
         * iters loops before running out of input.
         */
        let mut iters = ip[0].offset_from(ilowest) as size_t / 7;

        /* Each iteration can produce up to 10 bytes of output per stream.
         * Each output stream my advance at different rates. So take the
         * minimum number of safe iterations among all the output streams.
         */
        for stream in 0..4 {
            let oiters = oend[stream].offset_from(op[stream]) as size_t / 10;
            iters = Ord::min(iters, oiters);
        }

        /* Each iteration produces at least 5 output symbols. So until
         * op[3] crosses olimit, we know we haven't executed iters
         * iterations yet. This saves us maintaining an iters counter,
         * at the expense of computing the remaining # of iterations
         * more frequently.
         */
        olimit = op[3].add(iters as usize * 5);

        /* Exit the fast decoding loop once we reach the end. */
        if op[3] == olimit {
            break;
        }

        /* Exit the decoding loop if any input pointer has crossed the
         * previous one. This indicates corruption, and a precondition
         * to our loop is that ip[i] >= ip[0].
         */
        for stream in 1..4 {
            if ip[stream] < ip[stream - 1] {
                break 'out;
            }
        }

        for stream in 1..4 {
            assert!(ip[stream] >= ip[stream - 1]);
        }

        macro_rules! HUF_4X2_DECODE_SYMBOL {
            ($stream:expr, $decode3:expr) => {
                if (($decode3) != 0 || ($stream) != 3) {
                    let index = (bits[($stream)] >> 53);
                    let entry = dtable[index as usize];
                    MEM_write16(op[($stream)].cast(), entry.sequence);
                    bits[($stream)] <<= (entry.nbBits) & 0x3F;
                    op[($stream)] = op[($stream)].add(usize::from(entry.length));
                }
            };
        }

        macro_rules! HUF_4X2_RELOAD_STREAM {
            ($stream:expr) => {
                HUF_4X2_DECODE_SYMBOL!(3, 1);
                {
                    let ctz = bits[($stream)].trailing_zeros();
                    let nbBits = ctz & 7;
                    let nbBytes = ctz >> 3;
                    ip[($stream)] = ip[$stream].sub(nbBytes as usize);
                    bits[($stream)] = MEM_read64(ip[($stream)].cast()) | 1;
                    bits[($stream)] <<= nbBits;
                }
            };
        }

        /* Manually unroll the loop because compilers don't consistently
         * unroll the inner loops, which destroys performance.
         */

        loop {
            /* Decode 5 symbols from each of the first 3 streams.
             * The final stream will be decoded during the reload phase
             * to reduce register pressure.
             */
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X2_DECODE_SYMBOL, 0);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X2_DECODE_SYMBOL, 0);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X2_DECODE_SYMBOL, 0);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X2_DECODE_SYMBOL, 0);
            HUF_4X_FOR_EACH_STREAM_WITH_VAR!(HUF_4X2_DECODE_SYMBOL, 0);

            /* Decode one symbol from the final stream */
            HUF_4X2_DECODE_SYMBOL!(3, 1);

            /* Decode 4 symbols from the final stream & reload bitstreams.
             * The final stream is reloaded last, meaning that all 5 symbols
             * are decoded from the final stream before it is reloaded.
             */
            HUF_4X_FOR_EACH_STREAM!(HUF_4X2_RELOAD_STREAM);

            if !(op[3] < olimit) {
                break;
            }
        }
    }

    // Save the final values of each of the state variables back to args.
    args.bits = bits;
    args.ip = ip;
    args.op = op;
}

unsafe fn HUF_decompress4X2_usingDTable_internal_fast(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
    loopFn: HUF_DecompressFastLoopFn,
) -> size_t {
    let oend = dst.as_mut_ptr_range().end;

    let mut args = match HUF_DecompressFastArgs::new(dst.subslice(..), src, DTable) {
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

    let segmentSize = dst.capacity().div_ceil(4) as isize;
    let mut segmentEnd = dst.as_mut_ptr();

    for (i, op) in args.op.iter().copied().enumerate() {
        if segmentSize <= oend.offset_from(segmentEnd) {
            segmentEnd = segmentEnd.offset(segmentSize as isize);
        } else {
            segmentEnd = oend;
        }

        let mut bit = match init_remaining_dstream(&args, i, segmentEnd) {
            Ok(v) => v,
            Err(e) => return e.to_error_code(),
        };

        let length = HUF_decodeStreamX2(
            Writer::from_raw_parts(op, segmentEnd as usize - op as usize),
            &mut bit,
            DTable.data.as_x2(),
            HUF_DECODER_FAST_TABLELOG as u32,
        );

        if op.add(length as usize) != segmentEnd {
            return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
        }
    }

    dst.capacity() as size_t
}

unsafe fn HUF_decompress4X2_usingDTable_internal(
    mut dst: Writer<'_>,
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
                HUF_decompress4X2_usingDTable_internal_fast(dst.subslice(..), src, DTable, loopFn);
            if ret != 0 {
                return ret;
            }
        }

        HUF_decompress4X2_usingDTable_internal_bmi2(dst, src, DTable)
    } else {
        HUF_decompress4X2_usingDTable_internal_default(dst, src, DTable)
    }
}

unsafe fn HUF_decompress1X2_usingDTable_internal_bmi2(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, src, DTable)
}
unsafe fn HUF_decompress1X2_usingDTable_internal_default(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
) -> size_t {
    HUF_decompress1X2_usingDTable_internal_body(dst, src, DTable)
}

unsafe fn HUF_decompress1X2_usingDTable_internal(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if flags & HUF_flags_bmi2 as core::ffi::c_int != 0 {
        HUF_decompress1X2_usingDTable_internal_bmi2(dst, src, DTable)
    } else {
        HUF_decompress1X2_usingDTable_internal_default(dst, src, DTable)
    }
}

pub unsafe fn HUF_decompress1X2_DCtx_wksp(
    dctx: &mut DTable,
    mut dst: Writer<'_>,
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

    HUF_decompress1X2_usingDTable_internal(dst, &src[hSize as usize..], dctx, flags)
}

unsafe fn HUF_decompress4X2_DCtx_wksp(
    dctx: &mut DTable,
    mut dst: Writer<'_>,
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

    HUF_decompress4X2_usingDTable_internal(dst, &src[hSize as usize..], dctx, flags)
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
///  Assumption : 0 < dst_size <= 128 KB */
fn HUF_selectDecoder(dst_size: usize, src_size: usize) -> Decoder {
    let D256 = (dst_size >> 8) as u32;

    let Q = if src_size >= dst_size {
        15
    } else {
        (src_size * 16 / dst_size) as usize
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
    mut dst: Writer<'_>,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    if dst.capacity() == 0 {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if src.len() > dst.capacity() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }
    if src.len() == dst.capacity() {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), dst.capacity());
        return dst.capacity() as size_t;
    }
    if src.len() == 1 {
        ptr::write_bytes(dst.as_mut_ptr(), src[0], dst.capacity());
        return dst.capacity() as size_t;
    }

    match HUF_selectDecoder(dst.capacity(), src.len()) {
        Decoder::A1 => HUF_decompress1X1_DCtx_wksp(dctx, dst, src, workSpace, flags),
        Decoder::A2 => HUF_decompress1X2_DCtx_wksp(dctx, dst, src, workSpace, flags),
    }
}

pub unsafe fn HUF_decompress1X_usingDTable(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if DTable.description.tableType != 0 {
        HUF_decompress1X2_usingDTable_internal(dst, src, DTable, flags)
    } else {
        HUF_decompress1X1_usingDTable_internal(dst, src, DTable, flags)
    }
}

pub unsafe fn HUF_decompress1X1_DCtx_wksp(
    dctx: &mut DTable,
    mut dst: Writer<'_>,
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

    HUF_decompress1X1_usingDTable_internal(dst, &src[hSize as usize..], dctx, flags)
}

pub unsafe fn HUF_decompress4X_usingDTable(
    mut dst: Writer<'_>,
    src: &[u8],
    DTable: &DTable,
    flags: core::ffi::c_int,
) -> size_t {
    if DTable.description.tableType != 0 {
        HUF_decompress4X2_usingDTable_internal(dst, src, DTable, flags)
    } else {
        HUF_decompress4X1_usingDTable_internal(dst, src, DTable, flags)
    }
}

pub unsafe fn HUF_decompress4X_hufOnly_wksp(
    dctx: &mut DTable,
    mut dst: Writer<'_>,
    src: &[u8],
    workSpace: &mut Workspace,
    flags: core::ffi::c_int,
) -> size_t {
    if dst.is_empty() {
        return -(ZSTD_error_dstSize_tooSmall as core::ffi::c_int) as size_t;
    }
    if src.is_empty() {
        return -(ZSTD_error_corruption_detected as core::ffi::c_int) as size_t;
    }

    match HUF_selectDecoder(dst.capacity(), src.len()) {
        Decoder::A1 => HUF_decompress4X1_DCtx_wksp(dctx, dst, src, workSpace, flags),
        Decoder::A2 => HUF_decompress4X2_DCtx_wksp(dctx, dst, src, workSpace, flags),
    }
}

#[derive(Debug)]
pub struct Writer<'a> {
    ptr: *mut u8,
    end: *mut u8,
    _marker: core::marker::PhantomData<&'a mut [u8]>,
}

impl<'a> Writer<'a> {
    /// # Safety
    ///
    /// - `ptr` must point to `len` readable and writable bytes
    /// - `ptr` may be NULL only if `len == 0`
    pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> Self {
        let ptr = if ptr.is_null() {
            assert_eq!(len, 0);
            core::ptr::dangling_mut()
        } else {
            ptr
        };

        assert!(!ptr.is_null());

        Self {
            ptr,
            end: unsafe { ptr.add(len) },
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    fn capacity(&self) -> usize {
        unsafe { self.end.offset_from_unsigned(self.ptr) }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.ptr == self.end
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    #[inline]
    fn as_mut_ptr_range(&mut self) -> core::ops::Range<*mut u8> {
        self.ptr..self.end
    }

    fn subslice<R: core::ops::RangeBounds<usize>>(&mut self, range: R) -> Self {
        let ptr = match range.start_bound() {
            std::ops::Bound::Included(&count) => self.ptr.wrapping_add(count),
            std::ops::Bound::Excluded(_) => unreachable!("I think?"),
            std::ops::Bound::Unbounded => self.ptr,
        };

        if ptr > self.end {
            panic!("out of bounds");
        }

        let end = match range.end_bound() {
            std::ops::Bound::Included(&count) => self.ptr.wrapping_add(count + 1),
            std::ops::Bound::Excluded(&count) => self.ptr.wrapping_add(count),
            std::ops::Bound::Unbounded => self.end,
        };

        if ptr > self.end {
            panic!("out of bounds");
        }

        Self {
            ptr,
            end,
            _marker: core::marker::PhantomData,
        }
    }

    fn quarter(&mut self) -> Option<(Self, Self, Self, Self)> {
        let capacity = self.capacity();
        let segment_size = capacity.div_ceil(4);
        let range = self.as_mut_ptr_range();
        let remainder = capacity - 3 * segment_size;

        unsafe {
            let w1 = Self::from_raw_parts(range.start, segment_size);
            let w2 = Self::from_raw_parts(range.start.add(segment_size), segment_size);
            let w3 = Self::from_raw_parts(range.start.add(2 * segment_size), segment_size);
            let w4 = Self::from_raw_parts(range.start.add(3 * segment_size), remainder);

            // If the capacity is 6, `6.div_ceil(4)` is 2, but 4 * 2 > 6.
            if !range.contains(&w4.ptr) {
                return None;
            }

            assert!(w1.end <= range.end);
            assert!(w2.end <= range.end);
            assert!(w3.end <= range.end);
            assert!(w4.end <= range.end);

            Some((w1, w2, w3, w4))
        }
    }

    fn write_u8(&mut self, byte: u8) {
        if self.ptr >= self.end {
            panic!("write out of bounds");
        }

        // SAFETY: `ptr < end` and we're allowed to write to this memory.
        unsafe { self.ptr.write(byte) }

        // SAFETY: `ptr..end` is a contiguous allocation.
        self.ptr = unsafe { self.ptr.add(1) }
    }

    fn write_symbol_x2(&mut self, value: u16, length: u8) {
        debug_assert!(length <= 2);

        // we can't actually assert this, an earlier reader may write into the next.
        // that then returns an error. We should return a result here later.
        // assert!( self.ptr.wrapping_add(length as usize) <= self.end, "write out of bounds {:?} {length}", self.as_mut_ptr_range());

        // SAFETY: `ptr < end` and we're allowed to write to this memory.
        unsafe { self.ptr.cast::<u16>().write_unaligned(value) }

        // SAFETY: `ptr..end` is a contiguous allocation.
        self.ptr = unsafe { self.ptr.add(length as usize) }
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

    #[test]
    fn writer_subslice() {
        let mut arr = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let mut w = unsafe { Writer::from_raw_parts(arr.as_mut_ptr(), arr.len()) };

        assert_eq!(w.subslice(..).capacity(), 8);
        assert_eq!(w.subslice(..4).capacity(), 4);
        assert_eq!(w.subslice(4..).capacity(), 4);

        assert_eq!(w.subslice(..=4).capacity(), 5);
    }
}
