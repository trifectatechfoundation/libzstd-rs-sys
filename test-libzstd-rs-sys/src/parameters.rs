use libzstd_rs_sys::*;

#[test]
fn dctx_parameter_set_reset() {
    unsafe {
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
                    "ZSTD_DCtx_getParameter failed: {}",
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
                    "ZSTD_DCtx_setParameter failed: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }

            // get parameter value again to check if it was set
            let size = ZSTD_DCtx_getParameter(dctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_getParameter failed: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            assert_eq!(value, new_value);

            // reset parameter
            let size = ZSTD_DCtx_reset(dctx, ZSTD_ResetDirective::ZSTD_reset_parameters);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_reset failed: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }

            // get parameter value again to check if it was set
            let size = ZSTD_DCtx_getParameter(dctx, param, &mut value);
            if ZSTD_isError(size) != 0 {
                let err = ZSTD_getErrorName(size);
                panic!(
                    "ZSTD_DCtx_getParameter failed: {}",
                    std::ffi::CStr::from_ptr(err).to_string_lossy()
                );
            }
            assert_eq!(value, initial_value);
        }
    }
}
