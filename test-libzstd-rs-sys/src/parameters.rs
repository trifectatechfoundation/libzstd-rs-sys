use crate::assert_eq_rs_c;

#[test]
fn dctx_parameter_set_reset() {
    assert_eq_rs_c!({
        let dctx = ZSTD_createDCtx();
        if dctx.is_null() {
            panic!("Failed to create DCtx");
        }

        for param in [
            ZSTD_dParameter::ZSTD_d_windowLogMax,
            ZSTD_dParameter::ZSTD_d_experimentalParam1,
            ZSTD_dParameter::ZSTD_d_experimentalParam2,
            ZSTD_dParameter::ZSTD_d_experimentalParam3,
            ZSTD_dParameter::ZSTD_d_experimentalParam4,
            ZSTD_dParameter::ZSTD_d_experimentalParam5,
            ZSTD_dParameter::ZSTD_d_experimentalParam6,
        ] {
            let bounds = ZSTD_dParam_getBounds(param);

            // get parameter value
            let mut value = 37;
            let size = ZSTD_DCtx_getParameter(dctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_getParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            let initial_value = value;

            // set parameter value
            let new_value = if value == bounds.lowerBound {
                bounds.upperBound
            } else {
                bounds.lowerBound
            };
            let size = ZSTD_DCtx_setParameter(dctx, param, new_value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_setParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }

            // get parameter value again to check if it was set
            let size = ZSTD_DCtx_getParameter(dctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_getParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            assert_eq!(value, new_value);

            // reset parameter
            let size = ZSTD_DCtx_reset(dctx, ZSTD_ResetDirective::ZSTD_reset_parameters);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_reset failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }

            // get parameter value again to check if it was set
            let size = ZSTD_DCtx_getParameter(dctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_getParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            assert_eq!(value, initial_value);
        }
    });
}

#[test]
fn cctx_parameter_set_reset() {
    assert_eq_rs_c!({
        let cctx = ZSTD_createCCtx();
        if cctx.is_null() {
            panic!("Failed to create DCtx");
        }

        for param in [
            ZSTD_cParameter::ZSTD_c_compressionLevel,
            ZSTD_cParameter::ZSTD_c_windowLog,
            ZSTD_cParameter::ZSTD_c_hashLog,
            ZSTD_cParameter::ZSTD_c_chainLog,
            ZSTD_cParameter::ZSTD_c_searchLog,
            ZSTD_cParameter::ZSTD_c_minMatch,
            ZSTD_cParameter::ZSTD_c_targetLength,
            ZSTD_cParameter::ZSTD_c_strategy,
            ZSTD_cParameter::ZSTD_c_targetCBlockSize,
            ZSTD_cParameter::ZSTD_c_enableLongDistanceMatching,
            ZSTD_cParameter::ZSTD_c_ldmHashLog,
            ZSTD_cParameter::ZSTD_c_ldmMinMatch,
            ZSTD_cParameter::ZSTD_c_ldmBucketSizeLog,
            ZSTD_cParameter::ZSTD_c_ldmHashRateLog,
            ZSTD_cParameter::ZSTD_c_contentSizeFlag,
            ZSTD_cParameter::ZSTD_c_checksumFlag,
            ZSTD_cParameter::ZSTD_c_dictIDFlag,
            ZSTD_cParameter::ZSTD_c_nbWorkers,
            // The following 3 parameters give an "Unsupported parameter" error for the C implementation
            // ZSTD_cParameter::ZSTD_c_jobSize,
            // ZSTD_cParameter::ZSTD_c_overlapLog,
            // ZSTD_cParameter::ZSTD_c_experimentalParam1,
            ZSTD_cParameter::ZSTD_c_experimentalParam2,
            ZSTD_cParameter::ZSTD_c_experimentalParam3,
            ZSTD_cParameter::ZSTD_c_experimentalParam4,
            ZSTD_cParameter::ZSTD_c_experimentalParam5,
            ZSTD_cParameter::ZSTD_c_experimentalParam7,
            ZSTD_cParameter::ZSTD_c_experimentalParam8,
            ZSTD_cParameter::ZSTD_c_experimentalParam9,
            ZSTD_cParameter::ZSTD_c_experimentalParam10,
            ZSTD_cParameter::ZSTD_c_experimentalParam11,
            ZSTD_cParameter::ZSTD_c_experimentalParam12,
            ZSTD_cParameter::ZSTD_c_experimentalParam13,
            ZSTD_cParameter::ZSTD_c_experimentalParam14,
            ZSTD_cParameter::ZSTD_c_experimentalParam15,
            ZSTD_cParameter::ZSTD_c_experimentalParam16,
            ZSTD_cParameter::ZSTD_c_experimentalParam17,
            ZSTD_cParameter::ZSTD_c_experimentalParam18,
            ZSTD_cParameter::ZSTD_c_experimentalParam19,
            ZSTD_cParameter::ZSTD_c_experimentalParam20,
        ] {
            let bounds = ZSTD_cParam_getBounds(param);

            // get parameter value
            let mut value = 37;
            let size = ZSTD_CCtx_getParameter(cctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_CCtx_getParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            let initial_value = value;

            // set parameter value
            let new_value = if value == bounds.lowerBound {
                bounds.upperBound
            } else {
                bounds.lowerBound
            };
            let size = ZSTD_CCtx_setParameter(cctx, param, new_value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_CCtx_setParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }

            // get parameter value again to check if it was set
            let size = ZSTD_CCtx_getParameter(cctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_CCtx_getParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            assert_eq!(value, new_value);

            // reset parameter
            let size = ZSTD_CCtx_reset(cctx, ZSTD_ResetDirective::ZSTD_reset_parameters);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_CCtx_reset failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }

            // get parameter value again to check if it was set
            let size = ZSTD_CCtx_getParameter(cctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_CCtx_getParameter failed for {param:?}: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            assert_eq!(value, initial_value);
        }
    });
}
