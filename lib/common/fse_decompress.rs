use libc::size_t;

use crate::lib::common::fse::{
    FSE_DTableHeader, FSE_decode_t, FSE_MAX_SYMBOL_VALUE, FSE_MAX_TABLELOG,
};
use crate::lib::common::{
    bitstream::{BIT_DStream_t, StreamStatus},
    entropy_common::{DTable, FSE_readNCount_bmi2, Workspace},
    error_private::Error,
};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, align(4))]
pub(crate) struct FSE_DTable {
    pub header: FSE_DTableHeader,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub(crate) struct FSE_DecompressWksp {
    pub ncount: [core::ffi::c_short; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
struct FSE_DState_t<'a> {
    state: usize,
    table: &'a [FSE_decode_t; 90],
}

impl<'a> FSE_DState_t<'a> {
    fn new(bitD: &mut BIT_DStream_t, dt: &'a DTable) -> Self {
        let state = bitD.read_bits(core::ffi::c_uint::from(dt.header.tableLog));
        let _ = bitD.reload();
        let table = &dt.elements;

        Self { state, table }
    }

    #[inline]
    fn decode_symbol(&mut self, bitD: &mut BIT_DStream_t) -> u8 {
        let FSE_decode_t {
            nbBits,
            symbol,
            newState,
        } = self.table[self.state];

        let lowBits = bitD.read_bits(u32::from(nbBits));
        self.state = usize::from(newState) + lowBits;

        symbol
    }

    #[inline]
    fn decode_symbol_fast(&mut self, bitD: &mut BIT_DStream_t) -> u8 {
        let FSE_decode_t {
            nbBits,
            symbol,
            newState,
        } = self.table[self.state];

        let lowBits = bitD.read_bits_fast(u32::from(nbBits));
        self.state = usize::from(newState) + lowBits;

        symbol
    }
}

fn FSE_buildDTable_internal(
    dt: &mut DTable,
    normalizedCounter: &[core::ffi::c_short; 256],
    maxSymbolValue: core::ffi::c_uint,
    tableLog: core::ffi::c_uint,
) -> Result<(), Error> {
    let wkspSize = dt.elements[(1 << tableLog)..].len() * 4;
    let (header, elements, symbols, spread) = dt.destructure_mut(maxSymbolValue, tableLog);
    let maxSV1 = maxSymbolValue.wrapping_add(1);
    let tableSize = ((1) << tableLog) as u32;
    let mut highThreshold = tableSize.wrapping_sub(1);

    if ((::core::mem::size_of::<core::ffi::c_short>() as core::ffi::c_ulong)
        .wrapping_mul(core::ffi::c_ulong::from(maxSymbolValue.wrapping_add(1)))
        as core::ffi::c_ulonglong)
        .wrapping_add((1) << tableLog)
        .wrapping_add(8)
        > wkspSize as core::ffi::c_ulonglong
    {
        return Err(Error::maxSymbolValue_tooLarge);
    }

    if maxSymbolValue > FSE_MAX_SYMBOL_VALUE as core::ffi::c_uint {
        return Err(Error::maxSymbolValue_tooLarge);
    }

    if tableLog > FSE_MAX_TABLELOG as core::ffi::c_uint {
        return Err(Error::tableLog_tooLarge);
    }

    let mut DTableH = FSE_DTableHeader {
        tableLog: tableLog as u16,
        fastMode: 1,
    };

    let largeLimit = ((1) << tableLog.wrapping_sub(1)) as i16;
    let mut s: u32 = 0;
    s = 0;
    while s < maxSV1 {
        if core::ffi::c_int::from(normalizedCounter[s as usize]) == -(1) {
            let fresh0 = highThreshold;
            highThreshold = highThreshold.wrapping_sub(1);
            elements[fresh0 as usize].symbol = s as u8;
            symbols[s as usize] = 1;
        } else {
            if core::ffi::c_int::from(normalizedCounter[s as usize])
                >= core::ffi::c_int::from(largeLimit)
            {
                DTableH.fastMode = 0;
            }
            symbols[s as usize] = normalizedCounter[s as usize] as u16;
        }
        s = s.wrapping_add(1);
    }

    *header = DTableH;

    if highThreshold == tableSize.wrapping_sub(1) {
        let tableMask = tableSize.wrapping_sub(1) as size_t;
        let step = (tableSize >> 1)
            .wrapping_add(tableSize >> 3)
            .wrapping_add(3) as size_t;
        let add = 0x101010101010101u64;
        let mut pos = 0 as size_t;
        let mut sv = 0u64;

        for s_0 in 0..maxSV1 {
            let mut i: core::ffi::c_int = 0;
            let n = core::ffi::c_int::from(normalizedCounter[s_0 as usize]);
            spread[pos as usize..][..8].copy_from_slice(&sv.to_le_bytes());
            i = 8;
            while i < n {
                spread[pos as usize..][i as usize..][..8].copy_from_slice(&sv.to_le_bytes());
                i += 8;
            }
            pos = pos.wrapping_add(n as size_t);
            sv = sv.wrapping_add(add);
        }

        let mut position = 0 as size_t;
        let mut s_1: size_t = 0;
        let unroll = 2;
        s_1 = 0;
        while s_1 < tableSize as size_t {
            let mut u: size_t = 0;
            u = 0;
            while u < unroll {
                let uPosition = position.wrapping_add(u * step) & tableMask;
                elements[uPosition as usize].symbol = spread[(s_1 + u) as usize];
                u = u.wrapping_add(1);
            }
            position = position.wrapping_add(unroll * step) & tableMask;
            s_1 = s_1.wrapping_add(unroll);
        }
    } else {
        let tableMask_0 = tableSize.wrapping_sub(1);
        let step_0 = (tableSize >> 1)
            .wrapping_add(tableSize >> 3)
            .wrapping_add(3);

        let mut position_0 = 0u32;
        for s_2 in 0..maxSV1 {
            for _ in 0..normalizedCounter[s_2 as usize] {
                elements[position_0 as usize].symbol = s_2 as u8;
                position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                while position_0 > highThreshold {
                    position_0 = position_0.wrapping_add(step_0) & tableMask_0;
                }
            }
        }

        if position_0 != 0 {
            return Err(Error::GENERIC);
        }
    }

    for u in 0..tableSize {
        let symbol = usize::from((elements[u as usize]).symbol);
        let nextState = u32::from(symbols[symbol]);
        symbols[symbol] += 1;
        (elements[u as usize]).nbBits = tableLog.wrapping_sub(nextState.ilog2()) as u8;
        (elements[u as usize]).newState = (nextState
            << core::ffi::c_int::from((elements[u as usize]).nbBits))
        .wrapping_sub(tableSize) as u16;
    }

    Ok(())
}

#[inline(always)]
fn FSE_decompress_usingDTable_generic(
    dst: &mut [u8],
    cSrc: &[u8],
    dt: &DTable,
    fast: bool,
) -> Result<usize, Error> {
    enum Mode {
        Slow,
        Fast,
    }

    let mode = if fast { Mode::Fast } else { Mode::Slow };

    let mut op = 0;
    let omax = dst.len();
    let olimit = omax - 3;

    let mut bitD = BIT_DStream_t::new(cSrc)?;

    let mut state1 = FSE_DState_t::new(&mut bitD, dt);
    let mut state2 = FSE_DState_t::new(&mut bitD, dt);

    if let StreamStatus::Overflow = bitD.reload() {
        return Err(Error::corruption_detected);
    }

    while bitD.reload() == StreamStatus::Unfinished && op < olimit {
        dst[op] = match mode {
            Mode::Fast => state1.decode_symbol_fast(&mut bitD),
            Mode::Slow => state1.decode_symbol(&mut bitD),
        };

        if (FSE_MAX_TABLELOG * 2 + 7) as core::ffi::c_ulong
            > (::core::mem::size_of::<usize>() as core::ffi::c_ulong).wrapping_mul(8)
        {
            let _ = bitD.reload();
        }

        dst[op + 1] = match mode {
            Mode::Fast => state2.decode_symbol_fast(&mut bitD),
            Mode::Slow => state2.decode_symbol(&mut bitD),
        };

        if (FSE_MAX_TABLELOG * 4 + 7) as core::ffi::c_ulong
            > (size_of::<usize>() as core::ffi::c_ulong).wrapping_mul(8)
            && bitD.reload() != StreamStatus::Unfinished
        {
            op += 2;
            break;
        }

        dst[op + 2] = match mode {
            Mode::Fast => state1.decode_symbol_fast(&mut bitD),
            Mode::Slow => state1.decode_symbol(&mut bitD),
        };

        if (FSE_MAX_TABLELOG * 2 + 7) as core::ffi::c_ulong
            > (::core::mem::size_of::<usize>() as core::ffi::c_ulong).wrapping_mul(8)
        {
            let _ = bitD.reload();
        }

        dst[op + 3] = match mode {
            Mode::Fast => state2.decode_symbol_fast(&mut bitD),
            Mode::Slow => state2.decode_symbol(&mut bitD),
        };

        op += 4;
    }

    loop {
        if op > omax - 2 {
            return Err(Error::dstSize_tooSmall);
        }

        dst[op] = match mode {
            Mode::Fast => state1.decode_symbol_fast(&mut bitD),
            Mode::Slow => state1.decode_symbol(&mut bitD),
        };
        op += 1;

        if let StreamStatus::Overflow = bitD.reload() {
            dst[op] = match mode {
                Mode::Fast => state2.decode_symbol_fast(&mut bitD),
                Mode::Slow => state2.decode_symbol(&mut bitD),
            };
            op += 1;
            break;
        } else {
            if op > omax - 2 {
                return Err(Error::dstSize_tooSmall);
            }

            dst[op] = match mode {
                Mode::Fast => state2.decode_symbol_fast(&mut bitD),
                Mode::Slow => state2.decode_symbol(&mut bitD),
            };
            op += 1;

            match bitD.reload() {
                StreamStatus::Overflow => { /* fall through */ }
                _ => continue,
            }

            dst[op] = match mode {
                Mode::Fast => state1.decode_symbol_fast(&mut bitD),
                Mode::Slow => state1.decode_symbol(&mut bitD),
            };
            op += 1;

            break;
        }
    }

    Ok(op)
}

#[inline(always)]
fn FSE_decompress_wksp_body(
    dst: &mut [u8],
    cSrc: &[u8],
    maxLog: core::ffi::c_uint,
    workspace: &mut Workspace,
    bmi2: core::ffi::c_int,
) -> Result<size_t, Error> {
    let mut wkspSize = size_of::<Workspace>();

    let mut tableLog: core::ffi::c_uint = 0;
    let mut maxSymbolValue = FSE_MAX_SYMBOL_VALUE as core::ffi::c_uint;
    if wkspSize < ::core::mem::size_of::<FSE_DecompressWksp>() {
        return Err(Error::GENERIC);
    }
    let NCountLength = FSE_readNCount_bmi2(
        &mut workspace.a.ncount,
        &mut maxSymbolValue,
        &mut tableLog,
        cSrc,
        bmi2,
    )?;

    if tableLog > maxLog {
        return Err(Error::tableLog_tooLarge);
    }
    let ip = &cSrc[NCountLength as usize..];
    if ((1 + ((1) << tableLog) + 1) as core::ffi::c_ulonglong)
        .wrapping_add(
            ((::core::mem::size_of::<core::ffi::c_short>() as core::ffi::c_ulong)
                .wrapping_mul(core::ffi::c_ulong::from(maxSymbolValue.wrapping_add(1)))
                as core::ffi::c_ulonglong)
                .wrapping_add((1) << tableLog)
                .wrapping_add(8)
                .wrapping_add(
                    ::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong
                        as core::ffi::c_ulonglong,
                )
                .wrapping_sub(1)
                .wrapping_div(
                    ::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong
                        as core::ffi::c_ulonglong,
                ),
        )
        .wrapping_add(FSE_MAX_SYMBOL_VALUE.div_ceil(2) as core::ffi::c_ulonglong)
        .wrapping_add(1)
        .wrapping_mul(
            ::core::mem::size_of::<core::ffi::c_uint>() as core::ffi::c_ulong
                as core::ffi::c_ulonglong,
        )
        > wkspSize as core::ffi::c_ulonglong
    {
        return Err(Error::tableLog_tooLarge);
    }
    wkspSize = wkspSize.wrapping_sub((::core::mem::size_of::<FSE_DecompressWksp>()).wrapping_add(
        (1usize + (1 << tableLog)).wrapping_mul(::core::mem::size_of::<FSE_DTable>()),
    ));

    let () = FSE_buildDTable_internal(
        &mut workspace.dtable,
        &workspace.a.ncount,
        maxSymbolValue,
        tableLog,
    )?;

    FSE_decompress_usingDTable_generic(
        dst,
        ip,
        &workspace.dtable,
        workspace.dtable.header.fastMode != 0,
    )
}

fn FSE_decompress_wksp_body_default(
    dst: &mut [u8],
    cSrc: &[u8],
    maxLog: core::ffi::c_uint,
    workSpace: &mut Workspace,
) -> Result<size_t, Error> {
    FSE_decompress_wksp_body(dst, cSrc, maxLog, workSpace, 0)
}

fn FSE_decompress_wksp_body_bmi2(
    dst: &mut [u8],
    cSrc: &[u8],
    maxLog: core::ffi::c_uint,
    workSpace: &mut Workspace,
) -> Result<size_t, Error> {
    FSE_decompress_wksp_body(dst, cSrc, maxLog, workSpace, 1)
}

pub(super) fn FSE_decompress_wksp_bmi2(
    dst: &mut [u8],
    cSrc: &[u8],
    maxLog: core::ffi::c_uint,
    workSpace: &mut Workspace,
    bmi2: bool,
) -> Result<size_t, Error> {
    if bmi2 {
        FSE_decompress_wksp_body_bmi2(dst, cSrc, maxLog, workSpace)
    } else {
        FSE_decompress_wksp_body_default(dst, cSrc, maxLog, workSpace)
    }
}
