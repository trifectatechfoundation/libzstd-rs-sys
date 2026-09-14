mod compress2_strats {
    use crate::assert_eq_rs_c;
    use std::ffi::c_void;

    #[cfg(miri)]
    const INPUT: &[u8] = include_bytes!("../test-data/compress-input-tiny.dat");
    #[cfg(not(miri))]
    const INPUT: &[u8] = include_bytes!("../test-data/compress-input-small.dat");

    const DICT: &[u8] = include_bytes!("../test-data/compression-corpus.zstd");

    #[derive(Clone, Copy)]
    enum DictSetup {
        None,
        Prefix,
        CDict,
    }

    macro_rules! compress {
        ($strategy:expr, $use_row_match_finder:expr, $dict_setup:expr) => {{
            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            let err = ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_strategy, $strategy);
            assert_eq!(ZSTD_isError(err), 0);

            let err = ZSTD_CCtx_setParameter(
                cctx,
                ZSTD_cParameter::ZSTD_c_experimentalParam14, // ZSTD_c_useRowMatchFinder
                $use_row_match_finder,
            );
            assert_eq!(ZSTD_isError(err), 0);

            let cdict = match $dict_setup {
                DictSetup::None => core::ptr::null_mut(),
                DictSetup::Prefix => {
                    let err = ZSTD_CCtx_refPrefix(cctx, DICT.as_ptr() as *const c_void, DICT.len());
                    assert_eq!(ZSTD_isError(err), 0);
                    core::ptr::null_mut()
                }
                DictSetup::CDict => {
                    let cdict = ZSTD_createCDict(DICT.as_ptr() as *const c_void, DICT.len(), 3);
                    assert!(!cdict.is_null());
                    let err = ZSTD_CCtx_refCDict(cctx, cdict);
                    assert_eq!(ZSTD_isError(err), 0);
                    cdict
                }
            };

            let bound = ZSTD_compressBound(INPUT.len());
            let mut dst = vec![0u8; bound];

            let written = ZSTD_compress2(
                cctx,
                dst.as_mut_ptr() as *mut c_void,
                dst.len(),
                INPUT.as_ptr() as *const c_void,
                INPUT.len(),
            );
            assert_eq!(ZSTD_isError(written), 0);
            dst.truncate(written);

            ZSTD_freeCCtx(cctx);
            if !cdict.is_null() {
                ZSTD_freeCDict(cdict);
            }

            dst
        }};
    }

    // The full cross product of (useRowMatchFinder, dictSetup), in a fixed order
    const COMBOS: [(i32, DictSetup); 6] = [
        (1, DictSetup::None),
        (1, DictSetup::Prefix),
        (1, DictSetup::CDict),
        (2, DictSetup::None),
        (2, DictSetup::Prefix),
        (2, DictSetup::CDict),
    ];

    #[track_caller]
    fn check_strategy(strategy: i32) {
        if cfg!(miri) {
            // Pick just one combination to save time
            let (use_row_match_finder, dict_setup) = COMBOS[(strategy as usize - 1) % COMBOS.len()];
            assert_eq_rs_c!({ compress!(strategy, use_row_match_finder, dict_setup) });
        } else {
            for (use_row_match_finder, dict_setup) in COMBOS {
                assert_eq_rs_c!({ compress!(strategy, use_row_match_finder, dict_setup) });
            }
        }
    }

    #[test]
    fn fast() {
        check_strategy(1);
    }

    #[test]
    fn dfast() {
        check_strategy(2);
    }

    #[test]
    fn greedy() {
        check_strategy(3);
    }

    #[test]
    fn lazy() {
        check_strategy(4);
    }

    #[test]
    fn lazy2() {
        check_strategy(5);
    }

    #[test]
    fn btlazy2() {
        check_strategy(6);
    }

    #[test]
    fn btopt() {
        check_strategy(7);
    }

    #[test]
    #[cfg_attr(miri, ignore = "slow")]
    fn btultra() {
        check_strategy(8);
    }

    #[test]
    #[cfg_attr(miri, ignore = "slow")]
    fn btultra2() {
        check_strategy(9);
    }
}

#[test]
#[cfg_attr(miri, ignore = "slow")]
fn test_compress_stream_2() {
    use libzstd_rs_sys::lib::compress::zstd_compress::*;
    use libzstd_rs_sys::lib::decompress::zstd_decompress::ZSTD_decompress;
    use libzstd_rs_sys::{ZSTD_ResetDirective, ZSTD_cParameter, ZSTD_inBuffer, ZSTD_outBuffer};

    const INPUT: &[u8] = include_bytes!("../test-data/compress-input.dat");

    let c_size = 3388908;
    let compressed_buffer_size = 10731528;

    let mut buffer = vec![0; 847227];

    unsafe {
        let mut out_buf = ZSTD_outBuffer {
            dst: buffer.as_mut_ptr().cast(),
            size: buffer.len(),
            pos: 0,
        };

        let mut in_buf = ZSTD_inBuffer {
            src: INPUT.as_ptr().cast(),
            size: INPUT.len(),
            pos: 0,
        };

        let cctx = ZSTD_createCCtx();
        assert!(!cctx.is_null());

        let err = ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_session_and_parameters);
        assert_eq!(libzstd_rs_sys::ZSTD_isError(err), 0);

        let err = ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_checksumFlag, 1);
        assert_eq!(libzstd_rs_sys::ZSTD_isError(err), 0);

        // ZSTD_c_experimentalParam9 is ZSTD_c_stableInBuffer
        let err = ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_experimentalParam9, 1);
        assert_eq!(libzstd_rs_sys::ZSTD_isError(err), 0);

        out_buf.size = c_size / 4;
        loop {
            let ret = ZSTD_compressStream2(
                cctx,
                &mut out_buf,
                &mut in_buf,
                zstd_sys::ZSTD_EndDirective::ZSTD_e_end as _,
            );
            assert_eq!(libzstd_rs_sys::ZSTD_isError(ret), 0);
            if ret == 0 {
                break;
            }
            out_buf.size = Ord::min(out_buf.size + c_size / 4, compressed_buffer_size);
        }

        let mut decoded = vec![0; INPUT.len()];

        let ret = ZSTD_decompress(
            decoded.as_mut_ptr().cast(),
            decoded.len(),
            out_buf.dst,
            out_buf.pos,
        );
        assert_eq!(libzstd_rs_sys::ZSTD_isError(ret), 0);
    }
}

mod fse_canonical_tables {
    use crate::assert_eq_rs_c;
    use std::ffi::c_void;

    struct TestCase {
        name: &'static str,
        input: Vec<u8>,
        level: i32,
        strategy: Option<i32>,
        repeats: usize,
    }

    fn run_test_case(tc: &TestCase) {
        let _ = tc.name;
        let _rs_output = assert_eq_rs_c!({
            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            let err = ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_compressionLevel, tc.level);
            assert_eq!(ZSTD_isError(err), 0);

            if let Some(strat) = tc.strategy {
                let err = ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_strategy, strat);
                assert_eq!(ZSTD_isError(err), 0);
            }

            let bound = ZSTD_compressBound(tc.input.len());
            let mut dst = vec![0u8; bound];
            let mut last_written = 0;

            for iter in 0..tc.repeats {
                let written = ZSTD_compress2(
                    cctx,
                    dst.as_mut_ptr() as *mut c_void,
                    dst.len(),
                    tc.input.as_ptr() as *const c_void,
                    tc.input.len(),
                );
                assert_eq!(ZSTD_isError(written), 0);
                last_written = written;

                if iter + 1 < tc.repeats {
                    let err = ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_session_only);
                    assert_eq!(ZSTD_isError(err), 0);
                }
            }

            ZSTD_freeCCtx(cctx);
            dst.truncate(last_written);
            dst
        });
    }

    #[test]
    fn test_basic_mode_small_records_repeated() {
        let record = b"{\"user_id\":10023,\"status\":\"active\",\"code\":200}\n";
        let mut input = Vec::with_capacity(512);
        while input.len() < 512 {
            let to_copy = (512 - input.len()).min(record.len());
            input.extend_from_slice(&record[..to_copy]);
        }

        run_test_case(&TestCase {
            name: "basic-mode-small-records-repeated",
            input,
            level: 1,
            strategy: Some(1),
            repeats: 2,
        });
    }

    #[test]
    fn test_fse_basic_mode_canonical_tables_activation() {
        // Generate semi-structured small records with varied fields to produce
        // varied matches and sequences (avoiding pure RLE collapse).
        let mut input = Vec::new();
        for i in 0..10 {
            let s = format!(
                r#"{{"id":{},"status":"{}","tag":"tag_{}","val":{}}}"#,
                1000 + i,
                if i % 2 == 0 { "active" } else { "pending" },
                i % 5,
                i * 17
            );
            input.extend_from_slice(s.as_bytes());
            input.push(b'\n');
        }

        // Run both Rust and C compression and verify exact byte identity on Basic-mode workloads
        run_test_case(&TestCase {
            name: "basic-mode-varied-small-records",
            input: input.clone(),
            level: 1,
            strategy: Some(1),
            repeats: 2,
        });

        // Also test silesia slice (real-world data, activates Basic mode for all 3 tables)
        let silesia = include_bytes!("../../silesia-small.tar");
        run_test_case(&TestCase {
            name: "basic-mode-silesia-1024",
            input: silesia[..1024].to_vec(),
            level: 1,
            strategy: Some(1),
            repeats: 2,
        });

        // Test short-matches pattern as generated by compress.rs
        let generate_short_matches = |size: usize| {
            let mut data = Vec::with_capacity(size);
            let mut lcg = 42424242u64;
            let tokens = [b"abcd", b"efgh", b"1234", b"wxyz"];
            let mut tok_idx = 0;
            while data.len() < size {
                let tok = tokens[tok_idx % tokens.len()];
                tok_idx += 1;
                data.extend_from_slice(tok);
                for _ in 0..4 {
                    if data.len() >= size {
                        break;
                    }
                    lcg = lcg.wrapping_mul(6364136223846793005).wrapping_add(1);
                    data.push((lcg >> 32) as u8);
                }
            }
            data.truncate(size);
            data
        };

        for lvl in [1, 2, 3] {
            run_test_case(&TestCase {
                name: "basic-mode-short-matches-lvl",
                input: generate_short_matches(256),
                level: lvl,
                strategy: None,
                repeats: 2,
            });
        }
    }
}

