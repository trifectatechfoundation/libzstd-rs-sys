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

mod target_cblock_size {
    use crate::assert_eq_rs_c;
    use std::ffi::c_void;

    #[cfg(miri)]
    const INPUT: &[u8] = include_bytes!("../test-data/compress-input-tiny.dat");
    #[cfg(not(miri))]
    const INPUT: &[u8] = include_bytes!("../test-data/compress-input-small.dat");

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

/// The sequence-level APIs: extracting sequences from an input, and compressing sequences that
/// were produced elsewhere.
mod sequences {
    use crate::assert_eq_rs_c;
    use std::ffi::c_void;

    #[cfg(miri)]
    const INPUT: &[u8] = include_bytes!("../test-data/compress-input-tiny.dat");
    #[cfg(not(miri))]
    const INPUT: &[u8] = include_bytes!("../test-data/compress-input-small.dat");

    macro_rules! generate {
        ($input:expr, $level:expr) => {{
            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            let err =
                ZSTD_CCtx_setParameter(cctx, ZSTD_cParameter::ZSTD_c_compressionLevel, $level);
            assert_eq!(ZSTD_isError(err), 0);

            let mut seqs = vec![
                ZSTD_Sequence {
                    offset: 0,
                    litLength: 0,
                    matchLength: 0,
                    rep: 0,
                };
                ZSTD_sequenceBound($input.len())
            ];

            let count = ZSTD_generateSequences(
                cctx,
                seqs.as_mut_ptr(),
                seqs.len(),
                $input.as_ptr() as *const c_void,
                $input.len(),
            );
            assert_eq!(ZSTD_isError(count), 0);
            seqs.truncate(count);

            ZSTD_freeCCtx(cctx);

            seqs
        }};
    }

    #[cfg(not(miri))]
    const LEVELS: [i32; 3] = [1, 3, 9];
    #[cfg(miri)]
    const LEVELS: [i32; 1] = [1];

    #[test]
    fn generate_sequences() {
        for level in LEVELS {
            assert_eq_rs_c!({
                generate!(INPUT, level)
                    .iter()
                    .map(|seq| (seq.offset, seq.litLength, seq.matchLength, seq.rep))
                    .collect::<Vec<_>>()
            });
        }
    }

    macro_rules! compress_sequences_and_literals {
        ($input:expr, $level:expr, $repcode_resolution:expr) => {{
            let seqs = generate!($input, $level);

            // Everything the sequences do not cover with a match is a literal.
            let mut literals = Vec::with_capacity($input.len());
            let mut pos = 0;
            for seq in &seqs {
                literals.extend_from_slice(&$input[pos..][..seq.litLength as usize]);
                pos += (seq.litLength + seq.matchLength) as usize;
            }
            assert_eq!(pos, $input.len());

            // The literal buffer must have room for the wildcopy overread.
            let lit_size = literals.len();
            literals.resize(lit_size + 8, 0);

            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            for (parameter, value) in [
                (ZSTD_cParameter::ZSTD_c_compressionLevel, $level),
                // ZSTD_c_blockDelimiters, set to ZSTD_sf_explicitBlockDelimiters
                (ZSTD_cParameter::ZSTD_c_experimentalParam11, 1),
                // ZSTD_c_repcodeResolution
                (
                    ZSTD_cParameter::ZSTD_c_experimentalParam19,
                    $repcode_resolution,
                ),
            ] {
                let err = ZSTD_CCtx_setParameter(cctx, parameter, value);
                assert_eq!(ZSTD_isError(err), 0);
            }

            let bound = ZSTD_compressBound($input.len());
            let mut dst = vec![0u8; bound];

            let written = ZSTD_compressSequencesAndLiterals(
                cctx,
                dst.as_mut_ptr() as *mut c_void,
                dst.len(),
                seqs.as_ptr(),
                seqs.len(),
                literals.as_ptr() as *const c_void,
                lit_size,
                literals.len(),
                $input.len(),
            );
            assert_eq!(ZSTD_isError(written), 0);
            dst.truncate(written);

            ZSTD_freeCCtx(cctx);

            dst
        }};
    }

    #[test]
    fn compress_sequences_and_literals() {
        for level in LEVELS {
            // 1 = enable, 2 = disable. Only when repcode resolution is disabled are the
            // sequences converted by `convertSequences_noRepcodes`.
            for repcode_resolution in [1, 2] {
                let compressed = assert_eq_rs_c!({
                    compress_sequences_and_literals!(INPUT, level, repcode_resolution)
                });

                let mut decompressed = vec![0u8; INPUT.len()];
                let written = unsafe {
                    libzstd_rs_sys::ZSTD_decompress(
                        decompressed.as_mut_ptr() as *mut c_void,
                        decompressed.len(),
                        compressed.as_ptr() as *const c_void,
                        compressed.len(),
                    )
                };
                assert_eq!(written, INPUT.len());
                assert_eq!(decompressed, INPUT);
            }
        }
    }
}

/// Block-level external sequence producers (see `ZSTD_registerSequenceProducer`).
mod external_sequence_producer {
    use crate::assert_eq_rs_c;
    use std::ffi::{c_int, c_void};

    /// The input repeats every `UNIT` bytes, so a trivial parse exists: a literal run of `UNIT`
    /// bytes, followed by matches of `UNIT` bytes at offset `UNIT`.
    const UNIT: usize = 64;

    #[cfg(miri)]
    const SIZE: usize = 8 * UNIT;
    #[cfg(not(miri))]
    const SIZE: usize = 1 << 18;

    fn periodic(len: usize) -> Vec<u8> {
        let mut state = 1u64;
        let unit: Vec<u8> = (0..UNIT)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                (state >> 33) as u8
            })
            .collect();

        unit.into_iter().cycle().take(len).collect()
    }

    macro_rules! compress_with_producer {
        ($input:expr, $fail:expr, $fallback:expr) => {{
            unsafe extern "C" fn sequence_producer(
                state: *mut c_void,
                out_seqs: *mut ZSTD_Sequence,
                out_seqs_capacity: usize,
                _src: *const c_void,
                src_size: usize,
                _dict: *const c_void,
                _dict_size: usize,
                _compression_level: c_int,
                _window_size: usize,
            ) -> usize {
                if *(state as *const bool) {
                    // Any value above the capacity is an error.
                    return usize::MAX;
                }

                // Too short for a match: the whole block is trailing literals.
                if src_size < 2 * UNIT {
                    *out_seqs = ZSTD_Sequence {
                        offset: 0,
                        litLength: src_size as u32,
                        matchLength: 0,
                        rep: 0,
                    };
                    return 1;
                }

                let mut nb_seqs = 0;
                let mut pos = UNIT;
                while pos + UNIT <= src_size {
                    assert!(nb_seqs < out_seqs_capacity);
                    *out_seqs.add(nb_seqs) = ZSTD_Sequence {
                        offset: UNIT as u32,
                        litLength: if nb_seqs == 0 { UNIT as u32 } else { 0 },
                        matchLength: UNIT as u32,
                        rep: 0,
                    };
                    nb_seqs += 1;
                    pos += UNIT;
                }

                // Trailing literals are a block delimiter. When there are none the parse does
                // not end in a delimiter, and zstd has to append one itself.
                if pos < src_size {
                    assert!(nb_seqs < out_seqs_capacity);
                    *out_seqs.add(nb_seqs) = ZSTD_Sequence {
                        offset: 0,
                        litLength: (src_size - pos) as u32,
                        matchLength: 0,
                        rep: 0,
                    };
                    nb_seqs += 1;
                }

                nb_seqs
            }

            let cctx = ZSTD_createCCtx();
            assert!(!cctx.is_null());

            let mut fail = $fail;
            ZSTD_registerSequenceProducer(
                cctx,
                &mut fail as *mut bool as *mut c_void,
                Some(sequence_producer),
            );

            for (parameter, value) in [
                (ZSTD_cParameter::ZSTD_c_compressionLevel, 3),
                // ZSTD_c_enableSeqProducerFallback
                (ZSTD_cParameter::ZSTD_c_experimentalParam17, $fallback),
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

            ZSTD_freeCCtx(cctx);

            if ZSTD_isError(written) != 0 {
                Err(written)
            } else {
                dst.truncate(written);
                Ok(dst)
            }
        }};
    }

    #[track_caller]
    fn assert_roundtrips(input: &[u8], compressed: &[u8]) {
        let mut decompressed = vec![0u8; input.len()];
        let written = unsafe {
            libzstd_rs_sys::ZSTD_decompress(
                decompressed.as_mut_ptr() as *mut c_void,
                decompressed.len(),
                compressed.as_ptr() as *const c_void,
                compressed.len(),
            )
        };
        assert_eq!(written, input.len());
        assert_eq!(decompressed, input);
    }

    #[test]
    fn external_parse() {
        // The second input has trailing literals that don't fill a full match.
        for input in [periodic(SIZE), periodic(SIZE + 200)] {
            let compressed = assert_eq_rs_c!({ compress_with_producer!(input, false, 0) }).unwrap();
            assert_roundtrips(&input, &compressed);
        }
    }

    #[test]
    fn producer_error() {
        let input = periodic(SIZE);

        // Without the fallback, the error of the sequence producer is fatal.
        assert_eq_rs_c!({ compress_with_producer!(input, true, 0) }).unwrap_err();

        // With the fallback, zstd compresses with its own match finder.
        let compressed = assert_eq_rs_c!({ compress_with_producer!(input, true, 1) }).unwrap();
        assert_roundtrips(&input, &compressed);
    }
}
