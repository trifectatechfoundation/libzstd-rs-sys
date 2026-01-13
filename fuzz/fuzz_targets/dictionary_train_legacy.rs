#![no_main]
#![allow(deprecated)]

use c2rust_out_fuzz::{assert_eq_rs_c, ArbitrarySamples};
use libfuzzer_sys::fuzz_target;

// COVER dictionary builder tests
fuzz_target!(|samples: ArbitrarySamples| {
    // train
    assert_eq_rs_c!({
        let params = libzstd_rs_sys::ZDICT_legacy_params_t::default();

        let mut dict = vec![0u8; samples.dict_size];
        let dict_size = ZDICT_trainFromBuffer_legacy(
            dict.as_mut_ptr().cast(),
            samples.dict_size,
            samples.samples.as_ptr().cast(),
            samples.sample_sizes.as_ptr(),
            samples.nb_samples as u32,
            std::mem::transmute(params),
        );

        (dict_size, dict)
    });
});
