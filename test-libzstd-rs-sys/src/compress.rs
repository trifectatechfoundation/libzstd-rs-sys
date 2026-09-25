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

    const DICT_SETUPS: [DictSetup; 3] = [DictSetup::None, DictSetup::Prefix, DictSetup::CDict];

    macro_rules! compress_with_param {
        ($strategy:expr, $dict_setup:expr $(, $param:expr, $value:expr)?) => {{
            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            let err = ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_strategy, $strategy);
            assert_eq!(ZSTD_isError(err), 0);

            $(
                let err = ZSTD_CCtx_setParameter(cctx, $param, $value);
                assert_eq!(ZSTD_isError(err), 0);
            )?

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

    mod basic {
        use super::*;

        #[track_caller]
        fn check_strategy(strategy: i32) {
            if cfg!(miri) {
                // Pick just one combination to save time
                let dict_setup = DICT_SETUPS[(strategy as usize - 1) % DICT_SETUPS.len()];
                assert_eq_rs_c!({ compress_with_param!(strategy, dict_setup) });
            } else {
                for dict_setup in DICT_SETUPS {
                    assert_eq_rs_c!({ compress_with_param!(strategy, dict_setup) });
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

    /// `ZSTD_c_useRowMatchFinder` only affects greedy/lazy/lazy2
    /// (see`ZSTD_rowMatchFinderSupported` in zstd_compress.rs)]
    mod row_match_finder {
        use super::*;

        macro_rules! compress {
            ($strategy:expr, $use_row_match_finder:expr, $dict_setup:expr) => {
                compress_with_param!(
                    $strategy,
                    $dict_setup,
                    ZSTD_cParameter::ZSTD_c_experimentalParam14, // ZSTD_c_useRowMatchFinder
                    $use_row_match_finder
                )
            };
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
                let (use_row_match_finder, dict_setup) =
                    COMBOS[(strategy as usize - 1) % COMBOS.len()];
                assert_eq_rs_c!({ compress!(strategy, use_row_match_finder, dict_setup) });
            } else {
                for (use_row_match_finder, dict_setup) in COMBOS {
                    assert_eq_rs_c!({ compress!(strategy, use_row_match_finder, dict_setup) });
                }
            }
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
    }

    /// Only interesting for btopt/btultra/btultra2 which use zstd_opt.rs, and fast
    mod target_length {
        use super::*;

        macro_rules! compress {
            ($strategy:expr, $dict_setup:expr, $target_length:expr) => {
                compress_with_param!(
                    $strategy,
                    $dict_setup,
                    ZSTD_cParameter::ZSTD_c_targetLength,
                    $target_length
                )
            };
        }

        #[track_caller]
        fn check_strategy(strategy: i32, target_length: i32) {
            if cfg!(miri) {
                let dict_setup = DICT_SETUPS[(strategy as usize) % DICT_SETUPS.len()];
                assert_eq_rs_c!({ compress!(strategy, dict_setup, target_length) });
            } else {
                for dict_setup in DICT_SETUPS {
                    assert_eq_rs_c!({ compress!(strategy, dict_setup, target_length) });
                }
            }
        }

        /// For fast targetLength sets the match-sampling step size in zstd_fast.rs
        #[test]
        fn fast() {
            check_strategy(1, 64);
        }

        #[test]
        fn btopt() {
            check_strategy(7, 64);
        }

        #[test]
        fn btultra() {
            check_strategy(8, 64);
        }

        #[test]
        #[cfg_attr(miri, ignore = "slow")]
        fn btultra2() {
            check_strategy(9, 64);
        }
    }

    mod target_cblock_size {
        use super::*;

        const INCOMPRESSIBLE_INPUT: &[u8] = include_bytes!("../test-data/random-input.dat");

        macro_rules! compress_target {
            ($strategy:expr, $target_size:expr, $input:expr) => {{
                let cctx = ZSTD_createCCtx();
                assert!(!cctx.is_null());

                let err = ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_strategy, $strategy);
                assert_eq!(ZSTD_isError(err), 0);

                // set targetCBlockSize to use ZSTD_compressSuperBlock
                let err = ZSTD_CCtx_setParameter(
                    cctx,
                    ZSTD_cParameter::ZSTD_c_targetCBlockSize,
                    $target_size,
                );
                assert_eq!(ZSTD_isError(err), 0);

                let bound = ZSTD_compressBound($input.len());
                let mut dst = vec![0u8; bound];

                let written = ZSTD_compress2(
                    cctx,
                    dst.as_mut_ptr() as *mut c_void,
                    dst.len(),
                    $input.as_ptr() as *const c_void,
                    $input.len(),
                );
                assert_eq!(ZSTD_isError(written), 0);
                dst.truncate(written);

                ZSTD_freeCCtx(cctx);

                dst
            }};
        }

        #[cfg(not(miri))]
        const STRATEGIES: [i32; 4] = [1, 3, 6, 9];
        #[cfg(miri)]
        const STRATEGIES: [i32; 1] = [1];

        #[test]
        fn compressible_input() {
            for strategy in STRATEGIES {
                // test both ZSTD_TARGETCBLOCKSIZE_MIN and ZSTD_TARGETCBLOCKSIZE_MAX
                for target_size in [1340, 131072] {
                    assert_eq_rs_c!({ compress_target!(strategy, target_size, INPUT) });
                }
            }
        }

        #[test]
        fn incompressible_input() {
            for strategy in STRATEGIES {
                assert_eq_rs_c!({ compress_target!(strategy, 1340, INCOMPRESSIBLE_INPUT) });
            }
        }
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

/// Long-distance matching (see `lib/compress/zstd_ldm.rs`).
mod long_distance_matching {
    use crate::assert_eq_rs_c;
    use std::ffi::c_void;

    #[cfg(miri)]
    const SIZE: usize = 1 << 13;
    #[cfg(not(miri))]
    const SIZE: usize = 1 << 18;

    fn pseudo_random_step(state: &mut u64) -> u64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *state >> 33
    }

    fn pseudo_random(len: usize, seed: u64) -> Vec<u8> {
        let mut state = seed | 1;
        (0..len)
            .map(|_| pseudo_random_step(&mut state) as u8)
            .collect()
    }

    /// Random data with many short repeats copied from the first half into the
    /// second, at lengths around the default `ldmMinMatch` of 64.
    fn mosaic() -> Vec<u8> {
        let mut input = pseudo_random(SIZE, 5);

        let mut state = 12345u64;
        let mut next = move || pseudo_random_step(&mut state) as usize;

        for _ in 0..SIZE / 512 {
            let len = 40 + next() % 200;
            let from = next() % (SIZE / 2);
            let to = SIZE / 2 + next() % (SIZE / 2 - len - 1);
            let piece = input[from..from + len].to_vec();
            input[to..to + len].copy_from_slice(&piece);
        }

        input
    }

    /// One long verbatim block repeated far enough away that only LDM can reach
    /// back to it.
    fn far_repeat() -> Vec<u8> {
        let mut input = pseudo_random(SIZE, 1);
        let chunk = SIZE / 8;
        let tail = input[..chunk].to_vec();
        input.truncate(SIZE - chunk);
        input.extend_from_slice(&tail);
        input
    }

    fn run_of_zeros() -> Vec<u8> {
        vec![0u8; SIZE]
    }

    fn periodic() -> Vec<u8> {
        (0..SIZE).map(|i| (i % 7) as u8).collect()
    }

    macro_rules! compress_ldm {
        ($input:expr, $strategy:expr, $min_match:expr, $hash_rate_log:expr, $bucket_size_log:expr) => {{
            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            for (parameter, value) in [
                (ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching, 1),
                (ZSTD_cParameter::ZSTD_c_strategy, $strategy),
                (ZSTD_cParameter::ZSTD_c_ldmMinMatch, $min_match),
                (ZSTD_cParameter::ZSTD_c_ldmHashRateLog, $hash_rate_log),
                (ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog, $bucket_size_log),
            ] {
                let err = ZSTD_CCtx_setParameter(cctx, parameter, value);
                assert_eq!(ZSTD_isError(err), 0);
            }

            let bound = ZSTD_compressBound($input.len());
            let mut dst = vec![0u8; bound];

            let written = ZSTD_compress2(
                cctx,
                dst.as_mut_ptr() as *mut c_void,
                dst.len(),
                $input.as_ptr() as *const c_void,
                $input.len(),
            );
            assert_eq!(ZSTD_isError(written), 0);
            dst.truncate(written);

            ZSTD_freeCCtx(cctx);

            dst
        }};
    }

    // 1 = fast, 6 = btlazy2, 9 = btultra2.
    #[cfg(not(miri))]
    const STRATEGIES: [i32; 3] = [1, 6, 9];
    #[cfg(miri)]
    const STRATEGIES: [i32; 1] = [1];

    #[test]
    fn many_short_repeats() {
        let input = mosaic();
        for strategy in STRATEGIES {
            assert_eq_rs_c!({ compress_ldm!(input, strategy, 64, 7, 3) });
        }
    }

    #[test]
    fn one_long_repeat() {
        let input = far_repeat();
        for strategy in STRATEGIES {
            assert_eq_rs_c!({ compress_ldm!(input, strategy, 64, 7, 3) });
        }
    }

    #[test]
    fn overlapping_matches_all_zeros() {
        let input = run_of_zeros();
        for strategy in STRATEGIES {
            assert_eq_rs_c!({ compress_ldm!(input, strategy, 64, 7, 3) });
        }
    }

    #[test]
    fn overlapping_matches_periodic() {
        let input = periodic();
        for strategy in STRATEGIES {
            assert_eq_rs_c!({ compress_ldm!(input, strategy, 8, 0, 3) });
        }
    }

    #[test]
    fn no_matches() {
        let input = pseudo_random(SIZE, 2);
        for strategy in STRATEGIES {
            assert_eq_rs_c!({ compress_ldm!(input, strategy, 64, 7, 3) });
        }
    }

    #[test]
    fn min_match_lengths() {
        let input = mosaic();
        for min_match in [4, 5, 7, 32, 64, 999, 4096] {
            assert_eq_rs_c!({ compress_ldm!(input, 1, min_match, 7, 3) });
        }
    }

    #[test]
    fn hash_rate_logs() {
        let input = mosaic();
        for hash_rate_log in [0, 1, 4, 7, 12] {
            assert_eq_rs_c!({ compress_ldm!(input, 1, 64, hash_rate_log, 3) });
        }
    }

    #[test]
    fn bucket_size_logs() {
        let input = mosaic();
        for bucket_size_log in [1, 3, 8] {
            assert_eq_rs_c!({ compress_ldm!(input, 1, 64, 7, bucket_size_log) });
        }
    }

    #[test]
    fn with_prefix_dictionary() {
        let input = mosaic();
        // The dictionary is the half of the input the repeats were copied from,
        // so filling the table from it actually produces matches.
        let dict = input[..SIZE / 2].to_vec();

        for strategy in STRATEGIES {
            assert_eq_rs_c!({
                let cctx = ZSTD_createCCtx();
                assert!(!cctx.is_null());

                for (parameter, value) in [
                    (ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching, 1),
                    (ZSTD_cParameter::ZSTD_c_strategy, strategy),
                    (ZSTD_cParameter::ZSTD_c_ldmMinMatch, 64),
                ] {
                    let err = ZSTD_CCtx_setParameter(cctx, parameter, value);
                    assert_eq!(ZSTD_isError(err), 0);
                }

                let err = ZSTD_CCtx_refPrefix(cctx, dict.as_ptr() as *const c_void, dict.len());
                assert_eq!(ZSTD_isError(err), 0);

                let bound = ZSTD_compressBound(input.len());
                let mut dst = vec![0u8; bound];

                let written = ZSTD_compress2(
                    cctx,
                    dst.as_mut_ptr() as *mut c_void,
                    dst.len(),
                    input.as_ptr() as *const c_void,
                    input.len(),
                );
                assert_eq!(ZSTD_isError(written), 0);
                dst.truncate(written);

                ZSTD_freeCCtx(cctx);

                dst
            });
        }
    }

    #[test]
    fn streaming_in_chunks() {
        let input = mosaic();

        assert_eq_rs_c!({
            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            for (parameter, value) in [
                (ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching, 1),
                (ZSTD_cParameter::ZSTD_c_ldmMinMatch, 64),
            ] {
                let err = ZSTD_CCtx_setParameter(cctx, parameter, value);
                assert_eq!(ZSTD_isError(err), 0);
            }

            let mut dst = vec![0u8; ZSTD_compressBound(input.len())];
            let mut out_buffer = ZSTD_outBuffer {
                dst: dst.as_mut_ptr() as *mut c_void,
                size: dst.len(),
                pos: 0,
            };

            // A chunk size that is not a multiple of four, so the gear hash sees
            // a different tail remainder on every call.
            for chunk in input.chunks(1021) {
                let mut in_buffer = ZSTD_inBuffer {
                    src: chunk.as_ptr() as *const c_void,
                    size: chunk.len(),
                    pos: 0,
                };
                while in_buffer.pos < in_buffer.size {
                    let err = ZSTD_compressStream(cctx, &mut out_buffer, &mut in_buffer);
                    assert_eq!(ZSTD_isError(err), 0);
                }
            }

            while ZSTD_endStream(cctx, &mut out_buffer) != 0 {}

            ZSTD_freeCCtx(cctx);

            dst.truncate(out_buffer.pos);
            dst
        });
    }

    #[test]
    fn roundtrip() {
        use libzstd_rs_sys::*;

        let input = mosaic();

        let (compressed, written, decompressed) = unsafe {
            let compressed = compress_ldm!(input, 1, 64, 7, 3);

            let mut decompressed = vec![0u8; input.len()];
            let written = ZSTD_decompress(
                decompressed.as_mut_ptr() as *mut c_void,
                decompressed.len(),
                compressed.as_ptr() as *const c_void,
                compressed.len(),
            );
            (compressed, written, decompressed)
        };

        assert_eq!(ZSTD_isError(written), 0);
        assert_eq!(written, input.len());
        assert_eq!(decompressed, input);
        assert!(compressed.len() < input.len());
    }
}
