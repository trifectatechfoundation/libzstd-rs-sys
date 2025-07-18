use std::mem::MaybeUninit;

use libc::{free, malloc, memcpy, memset};

type size_t = usize;
type XXH_errorcode = std::ffi::c_uint;
const XXH_ERROR: XXH_errorcode = 1;
const XXH_OK: XXH_errorcode = 0;
type __uint8_t = std::ffi::c_uchar;
type __uint32_t = std::ffi::c_uint;
type __uint64_t = std::ffi::c_ulong;
type uint8_t = __uint8_t;
type uint32_t = __uint32_t;
type uint64_t = __uint64_t;
type XXH32_hash_t = uint32_t;
type xxh_u32 = XXH32_hash_t;
type XXH_alignment = std::ffi::c_uint;
const XXH_unaligned: XXH_alignment = 1;
const XXH_aligned: XXH_alignment = 0;
type xxh_u8 = uint8_t;
type xxh_unalign32 = xxh_u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_state_s {
    pub total_len_32: XXH32_hash_t,
    pub large_len: XXH32_hash_t,
    pub v: [XXH32_hash_t; 4],
    pub mem32: [XXH32_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved: XXH32_hash_t,
}

type XXH32_state_t = XXH32_state_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH32_canonical_t {
    pub digest: [std::ffi::c_uchar; 4],
}

type XXH64_hash_t = uint64_t;
type xxh_u64 = XXH64_hash_t;
type xxh_unalign64 = xxh_u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_state_s {
    pub total_len: XXH64_hash_t,
    pub v: [XXH64_hash_t; 4],
    pub mem64: [XXH64_hash_t; 4],
    pub memsize: XXH32_hash_t,
    pub reserved32: XXH32_hash_t,
    pub reserved64: XXH64_hash_t,
}

pub type XXH64_state_t = XXH64_state_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XXH64_canonical_t {
    pub digest: [std::ffi::c_uchar; 8],
}

const XXH_VERSION_MAJOR: std::ffi::c_int = 0 as std::ffi::c_int;
const XXH_VERSION_MINOR: std::ffi::c_int = 8 as std::ffi::c_int;
const XXH_VERSION_RELEASE: std::ffi::c_int = 2 as std::ffi::c_int;
const XXH_VERSION_NUMBER: std::ffi::c_int =
    XXH_VERSION_MAJOR * 100 as std::ffi::c_int * 100 as std::ffi::c_int
        + XXH_VERSION_MINOR * 100 as std::ffi::c_int
        + XXH_VERSION_RELEASE;
const XXH_FORCE_ALIGN_CHECK: std::ffi::c_int = 0 as std::ffi::c_int;
const XXH32_ENDJMP: std::ffi::c_int = 0 as std::ffi::c_int;

unsafe fn XXH_malloc(mut s: size_t) -> *mut std::ffi::c_void {
    malloc(s)
}
unsafe fn XXH_free(mut p: *mut std::ffi::c_void) {
    free(p);
}
unsafe fn XXH_memcpy(
    mut dest: *mut std::ffi::c_void,
    mut src: *const std::ffi::c_void,
    mut size: size_t,
) -> *mut std::ffi::c_void {
    memcpy(dest, src, size)
}

const XXH_PRIME64_1: std::ffi::c_ulonglong = 0x9e3779b185ebca87 as std::ffi::c_ulonglong;
const XXH_PRIME64_2: std::ffi::c_ulonglong = 0xc2b2ae3d27d4eb4f as std::ffi::c_ulonglong;
const XXH_PRIME64_3: std::ffi::c_ulonglong = 0x165667b19e3779f9 as std::ffi::c_ulonglong;
const XXH_PRIME64_4: std::ffi::c_ulonglong = 0x85ebca77c2b2ae63 as std::ffi::c_ulonglong;
const XXH_PRIME64_5: std::ffi::c_ulonglong = 0x27d4eb2f165667c5 as std::ffi::c_ulonglong;

const fn XXH64_round(mut acc: u64, input: u64) -> u64 {
    input
        .wrapping_mul(XXH_PRIME64_2)
        .wrapping_add(acc)
        .rotate_left(31)
        .wrapping_mul(XXH_PRIME64_1)
}

const fn XXH64_mergeRound(mut acc: u64, val: u64) -> u64 {
    (acc ^ XXH64_round(0, val))
        .wrapping_mul(XXH_PRIME64_1)
        .wrapping_add(XXH_PRIME64_4)
}

const fn XXH64_avalanche(mut hash: u64) -> u64 {
    hash ^= hash >> 33;
    hash = hash.wrapping_mul(XXH_PRIME64_2);
    hash ^= hash >> 29;
    hash = hash.wrapping_mul(XXH_PRIME64_3);
    hash ^= hash >> 32;
    hash
}

fn XXH64_finalize(mut hash: u64, slice: &[u8], _align: XXH_alignment) -> xxh_u64 {
    let (chunks, slice) = slice.as_chunks::<8>();
    for chunk in chunks {
        let k1 = XXH64_round(0, u64::from_le_bytes(*chunk));
        hash ^= k1;
        hash = (hash.rotate_left(27))
            .wrapping_mul(XXH_PRIME64_1)
            .wrapping_add(XXH_PRIME64_4);
    }

    let (chunks, slice) = slice.as_chunks::<4>();
    for chunk in chunks {
        hash ^= (u64::from(u32::from_le_bytes(*chunk))).wrapping_mul(XXH_PRIME64_1);
        hash = (hash.rotate_left(23))
            .wrapping_mul(XXH_PRIME64_2)
            .wrapping_add(XXH_PRIME64_3);
    }

    for byte in slice {
        hash ^= (u64::from(*byte)).wrapping_mul(XXH_PRIME64_5);
        hash = (hash.rotate_left(11)).wrapping_mul(XXH_PRIME64_1);
    }

    XXH64_avalanche(hash)
}

#[inline(always)]
unsafe fn XXH64_endian_align(mut input: &[u8], mut seed: u64, mut align: XXH_alignment) -> xxh_u64 {
    let mut h64: u64;

    let (chunks, remainder) = input.as_chunks::<32>();

    if !chunks.is_empty() {
        let mut v1 = seed.wrapping_add(XXH_PRIME64_1).wrapping_add(XXH_PRIME64_2);
        let mut v2 = seed.wrapping_add(XXH_PRIME64_2);
        let mut v3 = seed.wrapping_add(0);
        let mut v4 = seed.wrapping_sub(XXH_PRIME64_1);

        for chunk in chunks {
            let ([n1, n2, n3, n4], &[]) = chunk.as_chunks() else {
                unreachable!()
            };

            v1 = XXH64_round(v1, u64::from_le_bytes(*n1));
            v2 = XXH64_round(v2, u64::from_le_bytes(*n2));
            v3 = XXH64_round(v3, u64::from_le_bytes(*n3));
            v4 = XXH64_round(v4, u64::from_le_bytes(*n4));
        }

        h64 = (v1.rotate_left(1))
            .wrapping_add(v2.rotate_left(7))
            .wrapping_add(v3.rotate_left(12))
            .wrapping_add(v4.rotate_left(18));

        h64 = XXH64_mergeRound(h64, v1);
        h64 = XXH64_mergeRound(h64, v2);
        h64 = XXH64_mergeRound(h64, v3);
        h64 = XXH64_mergeRound(h64, v4);
    } else {
        h64 = seed.wrapping_add(XXH_PRIME64_5);
    }
    h64 = h64.wrapping_add(input.len() as u64);
    XXH64_finalize(h64, remainder, align)
}
#[no_mangle]
unsafe fn ZSTD_XXH64(
    mut input: *const std::ffi::c_void,
    mut len: size_t,
    mut seed: XXH64_hash_t,
) -> XXH64_hash_t {
    let slice = if input.is_null() {
        assert_eq!(len, 0);
        &[]
    } else {
        core::slice::from_raw_parts(input.cast::<u8>(), len)
    };

    XXH64_endian_align(slice, seed, XXH_unaligned)
}
unsafe fn ZSTD_XXH64_createState() -> *mut XXH64_state_t {
    XXH_malloc(::core::mem::size_of::<XXH64_state_t>()) as *mut XXH64_state_t
}
unsafe fn ZSTD_XXH64_freeState(mut statePtr: *mut XXH64_state_t) -> XXH_errorcode {
    XXH_free(statePtr as *mut std::ffi::c_void);
    XXH_OK
}
unsafe fn ZSTD_XXH64_copyState(
    mut dstState: *mut XXH64_state_t,
    mut srcState: *const XXH64_state_t,
) {
    XXH_memcpy(
        dstState as *mut std::ffi::c_void,
        srcState as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH64_state_t>(),
    );
}

#[no_mangle]
fn ZSTD_XXH64_reset(
    statePtr: &mut MaybeUninit<XXH64_state_t>,
    mut seed: XXH64_hash_t,
) -> XXH_errorcode {
    // SAFETY: all zeros is a valid value of type XXH64_state_t.
    let state = unsafe {
        core::ptr::write_bytes(statePtr.as_mut_ptr(), 0u8, 1);
        statePtr.assume_init_mut()
    };

    state.v[0] = seed.wrapping_add(XXH_PRIME64_1).wrapping_add(XXH_PRIME64_2);
    state.v[1] = seed.wrapping_add(XXH_PRIME64_2);
    state.v[2] = seed.wrapping_add(0);
    state.v[3] = seed.wrapping_sub(XXH_PRIME64_1);

    XXH_OK
}

unsafe fn XXH_readLE64(ptr: *const std::ffi::c_void) -> u64 {
    ptr.cast::<u64>().read_unaligned().to_le()
}

#[no_mangle]
unsafe fn ZSTD_XXH64_update(
    state: &mut XXH64_state_t,
    mut input: *const std::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        ::core::hint::assert_unchecked(len == 0 as std::ffi::c_int as size_t);
        return XXH_OK;
    }
    let mut p = input as *const xxh_u8;
    let bEnd = p.add(len);
    state.total_len = (state.total_len as std::ffi::c_ulong).wrapping_add(len as u64)
        as XXH64_hash_t as XXH64_hash_t;
    if (state.memsize as size_t).wrapping_add(len) < 32 as std::ffi::c_int as size_t {
        XXH_memcpy(
            ((state.mem64).as_mut_ptr() as *mut xxh_u8).offset(state.memsize as isize)
                as *mut std::ffi::c_void,
            input,
            len,
        );
        state.memsize = (state.memsize).wrapping_add(len as xxh_u32);
        return XXH_OK;
    }
    if state.memsize != 0 {
        XXH_memcpy(
            ((state.mem64).as_mut_ptr() as *mut xxh_u8).offset(state.memsize as isize)
                as *mut std::ffi::c_void,
            input,
            (32 as std::ffi::c_int as XXH32_hash_t).wrapping_sub(state.memsize) as size_t,
        );
        *(state.v).as_mut_ptr().offset(0 as std::ffi::c_int as isize) = XXH64_round(
            *(state.v).as_mut_ptr().offset(0 as std::ffi::c_int as isize),
            XXH_readLE64(
                (state.mem64)
                    .as_mut_ptr()
                    .offset(0 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        *(state.v).as_mut_ptr().offset(1 as std::ffi::c_int as isize) = XXH64_round(
            *(state.v).as_mut_ptr().offset(1 as std::ffi::c_int as isize),
            XXH_readLE64(
                (state.mem64)
                    .as_mut_ptr()
                    .offset(1 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        *(state.v).as_mut_ptr().offset(2 as std::ffi::c_int as isize) = XXH64_round(
            *(state.v).as_mut_ptr().offset(2 as std::ffi::c_int as isize),
            XXH_readLE64(
                (state.mem64)
                    .as_mut_ptr()
                    .offset(2 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        *(state.v).as_mut_ptr().offset(3 as std::ffi::c_int as isize) = XXH64_round(
            *(state.v).as_mut_ptr().offset(3 as std::ffi::c_int as isize),
            XXH_readLE64(
                (state.mem64)
                    .as_mut_ptr()
                    .offset(3 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        p = p.offset((32 as std::ffi::c_int as XXH32_hash_t).wrapping_sub(state.memsize) as isize);
        state.memsize = 0 as std::ffi::c_int as XXH32_hash_t;
    }
    if p.offset(32 as std::ffi::c_int as isize) <= bEnd {
        let limit = bEnd.offset(-(32 as std::ffi::c_int as isize));
        loop {
            *(state.v).as_mut_ptr().offset(0 as std::ffi::c_int as isize) = XXH64_round(
                *(state.v).as_mut_ptr().offset(0 as std::ffi::c_int as isize),
                XXH_readLE64(p as *const std::ffi::c_void),
            );
            p = p.offset(8 as std::ffi::c_int as isize);
            *(state.v).as_mut_ptr().offset(1 as std::ffi::c_int as isize) = XXH64_round(
                *(state.v).as_mut_ptr().offset(1 as std::ffi::c_int as isize),
                XXH_readLE64(p as *const std::ffi::c_void),
            );
            p = p.offset(8 as std::ffi::c_int as isize);
            *(state.v).as_mut_ptr().offset(2 as std::ffi::c_int as isize) = XXH64_round(
                *(state.v).as_mut_ptr().offset(2 as std::ffi::c_int as isize),
                XXH_readLE64(p as *const std::ffi::c_void),
            );
            p = p.offset(8 as std::ffi::c_int as isize);
            *(state.v).as_mut_ptr().offset(3 as std::ffi::c_int as isize) = XXH64_round(
                *(state.v).as_mut_ptr().offset(3 as std::ffi::c_int as isize),
                XXH_readLE64(p as *const std::ffi::c_void),
            );
            p = p.offset(8 as std::ffi::c_int as isize);
            if p > limit {
                break;
            }
        }
    }
    if p < bEnd {
        XXH_memcpy(
            (state.mem64).as_mut_ptr() as *mut std::ffi::c_void,
            p as *const std::ffi::c_void,
            bEnd.offset_from(p) as std::ffi::c_long as size_t,
        );
        state.memsize = bEnd.offset_from(p) as std::ffi::c_long as std::ffi::c_uint;
    }
    XXH_OK
}

#[no_mangle]
pub fn ZSTD_XXH64_digest(state: &mut XXH64_state_t) -> XXH64_hash_t {
    let mut h64;

    if state.total_len >= 32 {
        h64 = (state.v[0].rotate_left(1))
            .wrapping_add(state.v[1].rotate_left(7))
            .wrapping_add(state.v[2].rotate_left(12))
            .wrapping_add(state.v[3].rotate_left(18));

        h64 = XXH64_mergeRound(h64, state.v[0]);
        h64 = XXH64_mergeRound(h64, state.v[1]);
        h64 = XXH64_mergeRound(h64, state.v[2]);
        h64 = XXH64_mergeRound(h64, state.v[3]);
    } else {
        h64 = state.v[2].wrapping_add(XXH_PRIME64_5);
    }

    h64 = h64.wrapping_add(state.total_len);

    // SAFETY: casting an array of u64 to u8 is valid.
    let bytes = unsafe { core::mem::transmute::<&[u64; 4], &[u8; 8 * 4]>(&state.mem64) };

    XXH64_finalize(h64, &bytes[..state.total_len as usize % 32], XXH_aligned)
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    use super::*;

    use quickcheck::quickcheck;

    extern crate test;

    fn helper_u64(input: &[u8], seed: u64) -> u64 {
        unsafe { ZSTD_XXH64(input.as_ptr().cast(), input.len(), seed) }
    }

    quickcheck! {
        fn prop_xxh64_matches(input: Vec<u8>, seed: u64) -> bool {
            let expected = xxhash_rust::xxh64::xxh64(&input, seed);
            let actual = helper_u64(&input, seed);
            assert_eq!(expected, actual);
            expected == actual
        }
    }

    fn helper_state_u64(input: &[u8], seed: u64) -> u64 {
        let mut state = MaybeUninit::uninit();
        ZSTD_XXH64_reset(&mut state, seed);
        let state = unsafe { state.assume_init_mut() };

        unsafe { ZSTD_XXH64_update(state, input.as_ptr().cast(), input.len()) };

        ZSTD_XXH64_digest(state)
    }

    quickcheck! {
        fn prop_xxh64_state_matches(input: Vec<u8>, seed: u64) -> bool {
            let expected = xxhash_rust::xxh64::xxh64(&input, seed);
            let actual = helper_state_u64(&input, seed);
            assert_eq!(expected, actual);
            expected == actual
        }
    }

    #[bench]
    fn xxh64_reference(b: &mut test::Bencher) {
        b.iter(|| {
            xxhash_rust::xxh64::xxh64(test::black_box(&[b'a'; 1024]), test::black_box(123));
        });
    }

    #[bench]
    fn xxh64_ours(b: &mut test::Bencher) {
        b.iter(|| {
            helper_u64(test::black_box(&[b'a'; 1024]), test::black_box(123));
        });
    }
}
