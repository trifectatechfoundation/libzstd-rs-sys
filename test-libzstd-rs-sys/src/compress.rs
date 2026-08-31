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

    #[track_caller]
    fn check_strategy(strategy: i32) {
        for use_row_match_finder in 1..=2 {
            for dict_setup in [DictSetup::None, DictSetup::Prefix, DictSetup::CDict] {
                assert_eq_rs_c!({ compress!(strategy, use_row_match_finder, dict_setup) });
            }
        }
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn fast() {
        check_strategy(1);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn dfast() {
        check_strategy(2);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn greedy() {
        check_strategy(3);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn lazy() {
        check_strategy(4);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn lazy2() {
        check_strategy(5);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn btlazy2() {
        check_strategy(6);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn btopt() {
        check_strategy(7);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
    fn btultra() {
        check_strategy(8);
    }

    #[test]
    #[cfg_attr(miri, ignore = "ub")]
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
