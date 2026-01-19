#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use libzstd_rs_sys::*;
use libzstd_rs_sys::lib::common::huf::*;
use libzstd_rs_sys::lib::zstd::ZSTD_error_tableLog_tooLarge;
use std::ffi::*;

#[derive(Arbitrary, Debug)]
struct HufRoundTripInput {
    /// Use 1X (false) or 4X (true) stream compression
    streams: bool,
    /// Use X1 (false) or X2 (true) decompression table
    symbols: bool,
    /// HUF compression flags
    flags: HufFlags,
    /// Compression buffer size multiplier (0-4)
    cbuf_size_mult: u8,
    /// Initial table log value (will be clamped to 1-12)
    table_log: u8,
    /// The data to compress
    data: Vec<u8>,
}

#[derive(Arbitrary, Debug)]
struct HufFlags {
    bmi2: bool,
    optimal_depth: bool,
    prefer_repeat: bool,
    suspect_uncompressible: bool,
    disable_asm: bool,
    disable_fast: bool,
}

impl HufFlags {
    fn to_c_int(&self) -> c_int {
        let mut flags = 0;
        if self.bmi2 {
            flags |= 1; // HUF_flags_bmi2
        }
        if self.optimal_depth {
            flags |= 2; // HUF_flags_optimalDepth
        }
        if self.prefer_repeat {
            flags |= 4; // HUF_flags_preferRepeat
        }
        if self.suspect_uncompressible {
            flags |= 8; // HUF_flags_suspectUncompressible
        }
        if self.disable_asm {
            flags |= 16; // HUF_flags_disableAsm
        }
        if self.disable_fast {
            flags |= 32; // HUF_flags_disableFast
        }
        flags
    }
}

/// Adjust table log based on alphabet size (port of C helper function)
fn adjust_table_log(table_log: usize, max_symbol: usize) -> usize {
    let alphabet_size: usize = max_symbol + 1;
    let mut min_table_log: usize = (alphabet_size as u32).ilog2() as usize + 1;


    // If not a power of 2, need one more bit
    if (alphabet_size & (alphabet_size - 1)) != 0 {
        min_table_log += 1;
    }

    debug_assert!(min_table_log <= 9);

    if table_log < min_table_log {
        min_table_log
    } else {
        table_log
    }
}

fuzz_target!(|input: HufRoundTripInput| {
    // Limit input size to 256KB like the C version
    const MAX_SIZE: usize = 256 * 1024;
    let mut size = input.data.len();
    if size > MAX_SIZE {
        size = MAX_SIZE;
    }

    // C version checks size <= 1
    if size <= 1 {
        return;
    }
    let flags = input.flags.to_c_int();

    // Calculate compression buffer size (0 to 4 * size)
    let cbuf_size = (input.cbuf_size_mult as usize % 5) * size;
    if cbuf_size == 0 {
        return; // Need some buffer space
    }

    // Clamp table_log to valid range (1-12)
    let mut table_log = (input.table_log as c_uint % 12).max(1);

    // Step 1: Get histogram
    const HUF_SYMBOLVALUE_MAX: c_uint = 255;
    let mut count = vec![0u32; 256];
    let mut max_symbol = HUF_SYMBOLVALUE_MAX;

    let most_frequent = unsafe {
        lib::compress::hist::HIST_count(
            count.as_mut_ptr(),
            &mut max_symbol,
            input.data[..size].as_ptr().cast(),
            size,
        )
    };

    // Skip if error or RLE (all bytes the same)
    if ZSTD_isError(most_frequent) == 1 || most_frequent == size {
        return;
    }

    if max_symbol <= 255 {
        return;
    }

    // Step 2: Adjust table_log based on alphabet size
    table_log =  adjust_table_log(table_log as usize, max_symbol as usize) as u32;

    // Step 3: Allocate buffers
    let mut workspace = vec![0u8; HUF_WORKSPACE_SIZE];
    let mut r_buf = vec![0u8; size];
    let mut c_buf = vec![0u8; cbuf_size];

    // Compression table size: (maxSymbol + 2) * sizeof(usize)
    let ct_size = HUF_CTABLE_SIZE(max_symbol as usize);
    let mut ct = vec![0u8; ct_size];

    // Step 4: Optimize table log
    let table_log = unsafe {
        lib::compress::huf_compress::HUF_optimalTableLog(
            table_log,
            size,
            max_symbol,
            workspace.as_mut_ptr().cast(),
            HUF_WORKSPACE_SIZE,
            ct.as_mut_ptr() as *mut usize,
            count.as_ptr(),
            flags,
        )
    };

    // Step 5: Build compression table (returns actual tableLog)
    let table_log = unsafe {
        lib::compress::huf_compress::HUF_buildCTable_wksp(
            ct.as_mut_ptr() as *mut usize,
            count.as_ptr(),
            max_symbol,
            table_log,
            workspace.as_mut_ptr().cast(),
            HUF_WORKSPACE_SIZE,
        )
    };

    if ZSTD_isError(table_log) == 1 {
        return;
    }

    // Step 6: Write compression table to buffer
    let table_size = unsafe {
        lib::compress::huf_compress::HUF_writeCTable_wksp(
            c_buf.as_mut_ptr().cast(),
            cbuf_size,
            ct.as_ptr() as *const usize,
            max_symbol,
            table_log as c_uint,
            workspace.as_mut_ptr().cast(),
            HUF_WORKSPACE_SIZE,
        )
    };

    if ZSTD_isError(table_size) == 1 {
        return;
    }

    // Step 7: Read decompression table (X1 or X2)
    let mut dt = lib::decompress::huf_decompress::DTable::default();

    // C code does: dt[0] = tableLog * 0x01000001
    // This initializes the DTable description with the tableLog
    // In Rust, we need to set this via the description field
    let desc_value = (table_log as u32) * 0x01000001;
    dt.description.maxTableLog = (desc_value & 0xFF) as u8;
    dt.description.tableType = ((desc_value >> 8) & 0xFF) as u8;
    dt.description.tableLog = ((desc_value >> 16) & 0xFF) as u8;
    dt.description.reserved = ((desc_value >> 24) & 0xFF) as u8;

    let dt_read_size = if input.symbols {
        // Try X2 first
        let mut x2_workspace: lib::decompress::huf_decompress::HUF_ReadDTableX2_Workspace =
            lib::decompress::huf_decompress::HUF_ReadDTableX2_Workspace::default();
        let result = lib::decompress::huf_decompress::HUF_readDTableX2_wksp(
            &mut dt,
            &c_buf[..table_size],
            &mut x2_workspace,
            flags,
        );

        // Fall back to X1 if tableLog_tooLarge
        if ZSTD_isError(result) == 1 {
            let err_code = ERR_getErrorCode(result);
            if err_code ==  ZSTD_error_tableLog_tooLarge as usize {
                // tableLog_tooLarge error code
                let mut x1_workspace: lib::decompress::Workspace = lib::decompress::Workspace::default();
                lib::decompress::huf_decompress::HUF_readDTableX1_wksp(
                    &mut dt,
                    &c_buf[..table_size],
                    &mut x1_workspace,
                    flags,
                )
            } else {
                return; // Other error
            }
        } else {
            result
        }
    } else {
        // Use X1
        let mut x1_workspace: lib::decompress::Workspace = lib::decompress::Workspace::default();
        lib::decompress::huf_decompress::HUF_readDTableX1_wksp(
            &mut dt,
            &c_buf[..table_size],
            &mut x1_workspace,
            flags,
        )
    };

    if ZSTD_isError(dt_read_size) == 1 {
        return;
    }

    // Step 8: Compress the data (overwrites cBuf)
    let compress_size = unsafe {
        if !input.streams {
            // 1X stream
            lib::compress::huf_compress::HUF_compress1X_usingCTable(
                c_buf.as_mut_ptr().cast(),
                cbuf_size,
                input.data[..size].as_ptr().cast(),
                size,
                ct.as_ptr() as *const usize,
                flags,
            )
        } else {
            // 4X streams
            lib::compress::huf_compress::HUF_compress4X_usingCTable(
                c_buf.as_mut_ptr().cast(),
                cbuf_size,
                input.data[..size].as_ptr().cast(),
                size,
                ct.as_ptr() as *const usize,
                flags,
            )
        }
    };

    if ZSTD_isError(compress_size) == 1 {
        return;
    }

    // C code skips decompression if cSize == 0
    if compress_size == 0 {
        return;
    }

    // Step 9: Decompress the data
    let writer = unsafe {
        lib::decompress::huf_decompress::Writer::from_raw_parts(r_buf.as_mut_ptr(), r_buf.len())
    };

    let decompress_size = if !input.streams {
        // 1X stream
        lib::decompress::huf_decompress::HUF_decompress1X_usingDTable(
            writer,
            &c_buf[..compress_size],
            &dt,
            flags,
        )
    } else {
        // 4X streams
        lib::decompress::huf_decompress::HUF_decompress4X_usingDTable(
            writer,
            &c_buf[..compress_size],
            &dt,
            flags,
        )
    };

    // Step 10: Verify decompression
    if ZSTD_isError(decompress_size) == 1 {
        panic!("Decompression failed");
    }

    assert_eq!(decompress_size, size, "Decompressed size doesn't match original");
    assert_eq!(&r_buf[..size], &input.data[..size], "Decompressed data doesn't match original");
});

#[allow(non_snake_case)]
const fn ERR_getErrorCode(code: usize) -> usize {
    if ERR_isError(code) == 0 {
        return 0;
    }
    (0 as c_int as usize).wrapping_sub(code) as usize
}

#[allow(non_snake_case)]
const fn ERR_isError(code: usize) -> c_uint {
    use zstd_sys::ZSTD_ErrorCode::ZSTD_error_maxCode;
    (code > -(ZSTD_error_maxCode as c_int) as usize) as c_int as c_uint
}
