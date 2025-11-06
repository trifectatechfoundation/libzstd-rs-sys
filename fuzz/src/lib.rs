use std::cmp::{max, min};

use libfuzzer_sys::arbitrary;

#[derive(Debug)]
pub struct ArbitrarySamples {
    pub src: Vec<u8>,
    pub dict_size: usize,
    pub nb_samples: usize,
    pub samples: Vec<u8>,
    pub sample_sizes: Vec<usize>,
}

impl arbitrary::Arbitrary<'_> for ArbitrarySamples {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let src = Vec::<u8>::arbitrary(u)?;

        let dict_size = max(src.len() / 8, 1024);
        let total_sample_size = dict_size * 11;
        // pick number of samples based on remaining randomness that is left
        let nb_samples = u.arbitrary_len::<usize>()?;

        // generate random samples from src
        let mut samples = Vec::with_capacity(total_sample_size);
        let mut sample_sizes = Vec::with_capacity(nb_samples);
        let mut remaining = total_sample_size;
        for sample in 0..nb_samples {
            let offset = u.int_in_range(0..=src.len().saturating_sub(1))?;
            let limit = min(src.len() - offset, remaining);
            let to_copy = min(limit, remaining / (nb_samples - sample));

            samples.extend_from_slice(&src[offset..(offset + to_copy)]);
            remaining -= to_copy;
            sample_sizes.push(to_copy);
        }
        samples.resize(total_sample_size, 0);

        Ok(ArbitrarySamples {
            src,
            dict_size,
            nb_samples,
            samples,
            sample_sizes,
        })
    }
}

#[macro_export]
macro_rules! assert_eq_rs_c {
    ($tt:tt) => {{
        #[cfg(not(miri))]
        #[allow(clippy::macro_metavars_in_unsafe)]
        let _c = unsafe {
            use zstd_sys::*;

            #[allow(unused_braces)]
            $tt
        };

        #[allow(clippy::macro_metavars_in_unsafe)]
        let _rs = unsafe {
            use libzstd_rs_sys::*;

            #[allow(unused_braces)]
            $tt
        };

        #[cfg(not(miri))]
        assert_eq!(_rs, _c);

        _rs
    }};
}
