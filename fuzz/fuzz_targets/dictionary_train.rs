#![no_main]
#![allow(deprecated)]

use c2rust_out_fuzz::{assert_eq_rs_c, ArbitrarySamples};
use libfuzzer_sys::fuzz_target;

// COVER dictionary builder tests
fuzz_target!(|data: (u8, u8, ArbitrarySamples)| {
    let (k, d, samples) = data;

    let d = 1 + (d as u32 % 16);
    let k = d + (k as u32 % 256);

    // train
    assert_eq_rs_c!({
        let mut params = libzstd_rs_sys::ZDICT_cover_params_t::default();
        params.d = d;
        params.k = k;

        let mut dict = vec![0u8; samples.dict_size];
        let dict_size = ZDICT_trainFromBuffer_cover(
            dict.as_mut_ptr().cast(),
            samples.dict_size,
            samples.samples.as_ptr().cast(),
            samples.sample_sizes.as_ptr(),
            samples.nb_samples as u32,
            std::mem::transmute(params),
        );

        (dict_size, dict)
    });

    // optimize
    assert_eq_rs_c!({
        let mut opt_params = libzstd_rs_sys::ZDICT_cover_params_t::default();
        opt_params.steps = 4;

        let mut dict = vec![0u8; samples.dict_size];
        let opt_dict_size = ZDICT_optimizeTrainFromBuffer_cover(
            dict.as_mut_ptr().cast(),
            samples.dict_size,
            samples.samples.as_ptr().cast(),
            samples.sample_sizes.as_ptr(),
            samples.nb_samples as u32,
            std::mem::transmute(&mut opt_params),
        );

        (opt_dict_size, dict)
    });
});
