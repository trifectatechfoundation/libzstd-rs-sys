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
unsafe fn XXH_read32(mut ptr: *const std::ffi::c_void) -> xxh_u32 {
    *(ptr as *const xxh_unalign32)
}
const XXH_CPU_LITTLE_ENDIAN: std::ffi::c_int = 1 as std::ffi::c_int;
unsafe fn XXH_swap32(mut x: xxh_u32) -> xxh_u32 {
    x << 24 as std::ffi::c_int & 0xff000000 as std::ffi::c_uint
        | x << 8 as std::ffi::c_int & 0xff0000 as std::ffi::c_int as xxh_u32
        | x >> 8 as std::ffi::c_int & 0xff00 as std::ffi::c_int as xxh_u32
        | x >> 24 as std::ffi::c_int & 0xff as std::ffi::c_int as xxh_u32
}
#[inline(always)]
unsafe fn XXH_readLE32(mut ptr: *const std::ffi::c_void) -> xxh_u32 {
    if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read32(ptr)
    } else {
        XXH_swap32(XXH_read32(ptr))
    }
}
unsafe fn XXH_readBE32(mut ptr: *const std::ffi::c_void) -> xxh_u32 {
    if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_swap32(XXH_read32(ptr))
    } else {
        XXH_read32(ptr)
    }
}
#[inline(always)]
unsafe fn XXH_readLE32_align(
    mut ptr: *const std::ffi::c_void,
    mut align: XXH_alignment,
) -> xxh_u32 {
    if align as std::ffi::c_uint == XXH_unaligned as std::ffi::c_int as std::ffi::c_uint {
        XXH_readLE32(ptr)
    } else if XXH_CPU_LITTLE_ENDIAN != 0 {
        *(ptr as *const xxh_u32)
    } else {
        XXH_swap32(*(ptr as *const xxh_u32))
    }
}
fn ZSTD_XXH_versionNumber() -> std::ffi::c_uint {
    XXH_VERSION_NUMBER as std::ffi::c_uint
}

const XXH_PRIME32_1: std::ffi::c_uint = 0x9e3779b1 as std::ffi::c_uint;
const XXH_PRIME32_2: std::ffi::c_uint = 0x85ebca77 as std::ffi::c_uint;
const XXH_PRIME32_3: std::ffi::c_uint = 0xc2b2ae3d as std::ffi::c_uint;
const XXH_PRIME32_4: std::ffi::c_uint = 0x27d4eb2f as std::ffi::c_uint;
const XXH_PRIME32_5: std::ffi::c_uint = 0x165667b1 as std::ffi::c_uint;

unsafe fn XXH32_round(mut acc: xxh_u32, mut input: xxh_u32) -> xxh_u32 {
    acc = (acc as std::ffi::c_uint).wrapping_add(input.wrapping_mul(XXH_PRIME32_2)) as xxh_u32
        as xxh_u32;
    acc = ::core::intrinsics::rotate_left(acc, 13 as std::ffi::c_int as std::ffi::c_uint);
    acc = (acc as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_1) as xxh_u32 as xxh_u32;
    acc
}
unsafe fn XXH32_avalanche(mut hash: xxh_u32) -> xxh_u32 {
    hash ^= hash >> 15 as std::ffi::c_int;
    hash = (hash as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_2) as xxh_u32 as xxh_u32;
    hash ^= hash >> 13 as std::ffi::c_int;
    hash = (hash as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_3) as xxh_u32 as xxh_u32;
    hash ^= hash >> 16 as std::ffi::c_int;
    hash
}
unsafe fn XXH32_finalize(
    mut hash: xxh_u32,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u32 {
    if ptr.is_null() {
        ::core::hint::assert_unchecked(len == 0 as std::ffi::c_int as size_t);
    }
    if XXH32_ENDJMP == 0 {
        len &= 15 as std::ffi::c_int as size_t;
        while len >= 4 as std::ffi::c_int as size_t {
            hash = (hash as std::ffi::c_uint).wrapping_add(
                (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                    .wrapping_mul(XXH_PRIME32_3),
            ) as xxh_u32 as xxh_u32;
            ptr = ptr.offset(4 as std::ffi::c_int as isize);
            hash =
                (::core::intrinsics::rotate_left(hash, 17 as std::ffi::c_int as std::ffi::c_uint))
                    .wrapping_mul(XXH_PRIME32_4);
            len = len.wrapping_sub(4 as std::ffi::c_int as size_t);
        }
        while len > 0 as std::ffi::c_int as size_t {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            hash = (hash as std::ffi::c_uint)
                .wrapping_add((*fresh0 as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                as xxh_u32 as xxh_u32;
            hash =
                (::core::intrinsics::rotate_left(hash, 11 as std::ffi::c_int as std::ffi::c_uint))
                    .wrapping_mul(XXH_PRIME32_1);
            len = len.wrapping_sub(1);
            len;
        }
        XXH32_avalanche(hash)
    } else {
        's_489: {
            let mut current_block_119: u64;
            match len & 15 as std::ffi::c_int as size_t {
                12 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 12388749052780255754;
                }
                8 => {
                    current_block_119 = 12388749052780255754;
                }
                4 => {
                    current_block_119 = 8149943935827593142;
                }
                13 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 874927311066059277;
                }
                9 => {
                    current_block_119 = 874927311066059277;
                }
                5 => {
                    current_block_119 = 16230436841524390768;
                }
                14 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 12259350387199304931;
                }
                10 => {
                    current_block_119 = 12259350387199304931;
                }
                6 => {
                    current_block_119 = 6721905202649677722;
                }
                15 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 14965632903131415258;
                }
                11 => {
                    current_block_119 = 14965632903131415258;
                }
                7 => {
                    current_block_119 = 11798019441063049682;
                }
                3 => {
                    current_block_119 = 8569828448656383210;
                }
                2 => {
                    current_block_119 = 1194116980654867030;
                }
                1 => {
                    current_block_119 = 9573252646183719905;
                }
                0 => {
                    current_block_119 = 10034200926836878083;
                }
                _ => {
                    break 's_489;
                }
            }
            match current_block_119 {
                12388749052780255754 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 8149943935827593142;
                }
                874927311066059277 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 16230436841524390768;
                }
                12259350387199304931 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 6721905202649677722;
                }
                14965632903131415258 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 11798019441063049682;
                }
                _ => {}
            }
            match current_block_119 {
                6721905202649677722 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    let fresh2 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as std::ffi::c_uint)
                        .wrapping_add((*fresh2 as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        11 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_1);
                    let fresh3 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as std::ffi::c_uint)
                        .wrapping_add((*fresh3 as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        11 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_1);
                    return XXH32_avalanche(hash);
                }
                16230436841524390768 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    let fresh1 = ptr;
                    ptr = ptr.offset(1);
                    hash = (hash as std::ffi::c_uint)
                        .wrapping_add((*fresh1 as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                        as xxh_u32 as xxh_u32;
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        11 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_1);
                    return XXH32_avalanche(hash);
                }
                8149943935827593142 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    return XXH32_avalanche(hash);
                }
                11798019441063049682 => {
                    hash = (hash as std::ffi::c_uint).wrapping_add(
                        (XXH_readLE32_align(ptr as *const std::ffi::c_void, align))
                            .wrapping_mul(XXH_PRIME32_3),
                    ) as xxh_u32 as xxh_u32;
                    ptr = ptr.offset(4 as std::ffi::c_int as isize);
                    hash = (::core::intrinsics::rotate_left(
                        hash,
                        17 as std::ffi::c_int as std::ffi::c_uint,
                    ))
                    .wrapping_mul(XXH_PRIME32_4);
                    current_block_119 = 8569828448656383210;
                }
                _ => {}
            }
            if current_block_119 == 8569828448656383210 {
                let fresh4 = ptr;
                ptr = ptr.offset(1);
                hash = (hash as std::ffi::c_uint)
                    .wrapping_add((*fresh4 as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                    as xxh_u32 as xxh_u32;
                hash = (::core::intrinsics::rotate_left(
                    hash,
                    11 as std::ffi::c_int as std::ffi::c_uint,
                ))
                .wrapping_mul(XXH_PRIME32_1);
                current_block_119 = 1194116980654867030;
            }
            if current_block_119 == 1194116980654867030 {
                let fresh5 = ptr;
                ptr = ptr.offset(1);
                hash = (hash as std::ffi::c_uint)
                    .wrapping_add((*fresh5 as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                    as xxh_u32 as xxh_u32;
                hash = (::core::intrinsics::rotate_left(
                    hash,
                    11 as std::ffi::c_int as std::ffi::c_uint,
                ))
                .wrapping_mul(XXH_PRIME32_1);
                current_block_119 = 9573252646183719905;
            }
            if current_block_119 == 9573252646183719905 {
                let fresh6 = ptr;
                ptr = ptr.offset(1);
                hash = (hash as std::ffi::c_uint)
                    .wrapping_add((*fresh6 as std::ffi::c_uint).wrapping_mul(XXH_PRIME32_5))
                    as xxh_u32 as xxh_u32;
                hash = (::core::intrinsics::rotate_left(
                    hash,
                    11 as std::ffi::c_int as std::ffi::c_uint,
                ))
                .wrapping_mul(XXH_PRIME32_1);
            }
            return XXH32_avalanche(hash);
        }
        ::core::hint::assert_unchecked(0 as std::ffi::c_int != 0);
        hash
    }
}
#[inline(always)]
unsafe fn XXH32_endian_align(
    mut input: *const xxh_u8,
    mut len: size_t,
    mut seed: xxh_u32,
    mut align: XXH_alignment,
) -> xxh_u32 {
    let mut h32: xxh_u32 = 0;
    if input.is_null() {
        ::core::hint::assert_unchecked(len == 0 as std::ffi::c_int as size_t);
    }
    if len >= 16 as std::ffi::c_int as size_t {
        let bEnd = input.offset(len as isize);
        let limit = bEnd.offset(-(15 as std::ffi::c_int as isize));
        let mut v1 = seed.wrapping_add(XXH_PRIME32_1).wrapping_add(XXH_PRIME32_2);
        let mut v2 = seed.wrapping_add(XXH_PRIME32_2);
        let mut v3 = seed.wrapping_add(0 as std::ffi::c_int as xxh_u32);
        let mut v4 = seed.wrapping_sub(XXH_PRIME32_1);
        loop {
            v1 = XXH32_round(
                v1,
                XXH_readLE32_align(input as *const std::ffi::c_void, align),
            );
            input = input.offset(4 as std::ffi::c_int as isize);
            v2 = XXH32_round(
                v2,
                XXH_readLE32_align(input as *const std::ffi::c_void, align),
            );
            input = input.offset(4 as std::ffi::c_int as isize);
            v3 = XXH32_round(
                v3,
                XXH_readLE32_align(input as *const std::ffi::c_void, align),
            );
            input = input.offset(4 as std::ffi::c_int as isize);
            v4 = XXH32_round(
                v4,
                XXH_readLE32_align(input as *const std::ffi::c_void, align),
            );
            input = input.offset(4 as std::ffi::c_int as isize);
            if input >= limit {
                break;
            }
        }
        h32 = (::core::intrinsics::rotate_left(v1, 1 as std::ffi::c_int as std::ffi::c_uint))
            .wrapping_add(::core::intrinsics::rotate_left(
                v2,
                7 as std::ffi::c_int as std::ffi::c_uint,
            ))
            .wrapping_add(::core::intrinsics::rotate_left(
                v3,
                12 as std::ffi::c_int as std::ffi::c_uint,
            ))
            .wrapping_add(::core::intrinsics::rotate_left(
                v4,
                18 as std::ffi::c_int as std::ffi::c_uint,
            ));
    } else {
        h32 = seed.wrapping_add(XXH_PRIME32_5);
    }
    h32 = h32.wrapping_add(len as xxh_u32);
    XXH32_finalize(h32, input, len & 15 as std::ffi::c_int as size_t, align)
}
unsafe fn ZSTD_XXH32(
    mut input: *const std::ffi::c_void,
    mut len: size_t,
    mut seed: XXH32_hash_t,
) -> XXH32_hash_t {
    XXH32_endian_align(input as *const xxh_u8, len, seed, XXH_unaligned)
}
unsafe fn ZSTD_XXH32_createState() -> *mut XXH32_state_t {
    XXH_malloc(::core::mem::size_of::<XXH32_state_t>()) as *mut XXH32_state_t
}
unsafe fn ZSTD_XXH32_freeState(mut statePtr: *mut XXH32_state_t) -> XXH_errorcode {
    XXH_free(statePtr as *mut std::ffi::c_void);
    XXH_OK
}
unsafe fn ZSTD_XXH32_copyState(
    mut dstState: *mut XXH32_state_t,
    mut srcState: *const XXH32_state_t,
) {
    XXH_memcpy(
        dstState as *mut std::ffi::c_void,
        srcState as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH32_state_t>(),
    );
}
unsafe fn ZSTD_XXH32_reset(
    mut statePtr: *mut XXH32_state_t,
    mut seed: XXH32_hash_t,
) -> XXH_errorcode {
    ::core::hint::assert_unchecked(!statePtr.is_null());
    memset(
        statePtr as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<XXH32_state_t>(),
    );
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) =
        seed.wrapping_add(XXH_PRIME32_1).wrapping_add(XXH_PRIME32_2);
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize) = seed.wrapping_add(XXH_PRIME32_2);
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize) =
        seed.wrapping_add(0 as std::ffi::c_int as XXH32_hash_t);
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize) = seed.wrapping_sub(XXH_PRIME32_1);
    XXH_OK
}
unsafe fn ZSTD_XXH32_update(
    mut state: *mut XXH32_state_t,
    mut input: *const std::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        ::core::hint::assert_unchecked(len == 0 as std::ffi::c_int as size_t);
        return XXH_OK;
    }
    let mut p = input as *const xxh_u8;
    let bEnd = p.offset(len as isize);
    (*state).total_len_32 = ((*state).total_len_32).wrapping_add(len as XXH32_hash_t);
    (*state).large_len |= ((len >= 16 as std::ffi::c_int as size_t) as std::ffi::c_int
        | ((*state).total_len_32 >= 16 as std::ffi::c_int as XXH32_hash_t) as std::ffi::c_int)
        as XXH32_hash_t;
    if ((*state).memsize as size_t).wrapping_add(len) < 16 as std::ffi::c_int as size_t {
        XXH_memcpy(
            (((*state).mem32).as_mut_ptr() as *mut xxh_u8).offset((*state).memsize as isize)
                as *mut std::ffi::c_void,
            input,
            len,
        );
        (*state).memsize = ((*state).memsize).wrapping_add(len as XXH32_hash_t);
        return XXH_OK;
    }
    if (*state).memsize != 0 {
        XXH_memcpy(
            (((*state).mem32).as_mut_ptr() as *mut xxh_u8).offset((*state).memsize as isize)
                as *mut std::ffi::c_void,
            input,
            (16 as std::ffi::c_int as XXH32_hash_t).wrapping_sub((*state).memsize) as size_t,
        );
        let mut p32: *const xxh_u32 = ((*state).mem32).as_mut_ptr();
        *((*state).v)
            .as_mut_ptr()
            .offset(0 as std::ffi::c_int as isize) = XXH32_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(0 as std::ffi::c_int as isize),
            XXH_readLE32(p32 as *const std::ffi::c_void),
        );
        p32 = p32.offset(1);
        p32;
        *((*state).v)
            .as_mut_ptr()
            .offset(1 as std::ffi::c_int as isize) = XXH32_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(1 as std::ffi::c_int as isize),
            XXH_readLE32(p32 as *const std::ffi::c_void),
        );
        p32 = p32.offset(1);
        p32;
        *((*state).v)
            .as_mut_ptr()
            .offset(2 as std::ffi::c_int as isize) = XXH32_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(2 as std::ffi::c_int as isize),
            XXH_readLE32(p32 as *const std::ffi::c_void),
        );
        p32 = p32.offset(1);
        p32;
        *((*state).v)
            .as_mut_ptr()
            .offset(3 as std::ffi::c_int as isize) = XXH32_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(3 as std::ffi::c_int as isize),
            XXH_readLE32(p32 as *const std::ffi::c_void),
        );
        p = p.offset(
            (16 as std::ffi::c_int as XXH32_hash_t).wrapping_sub((*state).memsize) as isize,
        );
        (*state).memsize = 0 as std::ffi::c_int as XXH32_hash_t;
    }
    if p <= bEnd.offset(-(16 as std::ffi::c_int as isize)) {
        let limit = bEnd.offset(-(16 as std::ffi::c_int as isize));
        loop {
            *((*state).v)
                .as_mut_ptr()
                .offset(0 as std::ffi::c_int as isize) = XXH32_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(0 as std::ffi::c_int as isize),
                XXH_readLE32(p as *const std::ffi::c_void),
            );
            p = p.offset(4 as std::ffi::c_int as isize);
            *((*state).v)
                .as_mut_ptr()
                .offset(1 as std::ffi::c_int as isize) = XXH32_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(1 as std::ffi::c_int as isize),
                XXH_readLE32(p as *const std::ffi::c_void),
            );
            p = p.offset(4 as std::ffi::c_int as isize);
            *((*state).v)
                .as_mut_ptr()
                .offset(2 as std::ffi::c_int as isize) = XXH32_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(2 as std::ffi::c_int as isize),
                XXH_readLE32(p as *const std::ffi::c_void),
            );
            p = p.offset(4 as std::ffi::c_int as isize);
            *((*state).v)
                .as_mut_ptr()
                .offset(3 as std::ffi::c_int as isize) = XXH32_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(3 as std::ffi::c_int as isize),
                XXH_readLE32(p as *const std::ffi::c_void),
            );
            p = p.offset(4 as std::ffi::c_int as isize);
            if p > limit {
                break;
            }
        }
    }
    if p < bEnd {
        XXH_memcpy(
            ((*state).mem32).as_mut_ptr() as *mut std::ffi::c_void,
            p as *const std::ffi::c_void,
            bEnd.offset_from(p) as std::ffi::c_long as size_t,
        );
        (*state).memsize = bEnd.offset_from(p) as std::ffi::c_long as std::ffi::c_uint;
    }
    XXH_OK
}
unsafe fn ZSTD_XXH32_digest(mut state: *const XXH32_state_t) -> XXH32_hash_t {
    let mut h32: xxh_u32 = 0;
    if (*state).large_len != 0 {
        h32 = (::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(0 as std::ffi::c_int as isize),
            1 as std::ffi::c_int as std::ffi::c_uint,
        ))
        .wrapping_add(::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(1 as std::ffi::c_int as isize),
            7 as std::ffi::c_int as std::ffi::c_uint,
        ))
        .wrapping_add(::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(2 as std::ffi::c_int as isize),
            12 as std::ffi::c_int as std::ffi::c_uint,
        ))
        .wrapping_add(::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(3 as std::ffi::c_int as isize),
            18 as std::ffi::c_int as std::ffi::c_uint,
        ));
    } else {
        h32 = (*((*state).v).as_ptr().offset(2 as std::ffi::c_int as isize))
            .wrapping_add(XXH_PRIME32_5);
    }
    h32 = (h32 as XXH32_hash_t).wrapping_add((*state).total_len_32) as xxh_u32 as xxh_u32;
    XXH32_finalize(
        h32,
        ((*state).mem32).as_ptr() as *const xxh_u8,
        (*state).memsize as size_t,
        XXH_aligned,
    )
}
unsafe fn ZSTD_XXH32_canonicalFromHash(mut dst: *mut XXH32_canonical_t, mut hash: XXH32_hash_t) {
    hash = XXH_swap32(hash);
    XXH_memcpy(
        dst as *mut std::ffi::c_void,
        &mut hash as *mut XXH32_hash_t as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH32_canonical_t>(),
    );
}
unsafe fn ZSTD_XXH32_hashFromCanonical(mut src: *const XXH32_canonical_t) -> XXH32_hash_t {
    XXH_readBE32(src as *const std::ffi::c_void)
}
unsafe fn XXH_read64(mut ptr: *const std::ffi::c_void) -> xxh_u64 {
    *(ptr as *const xxh_unalign64)
}
unsafe fn XXH_swap64(mut x: xxh_u64) -> xxh_u64 {
    ((x << 56 as std::ffi::c_int) as std::ffi::c_ulonglong
        & 0xff00000000000000 as std::ffi::c_ulonglong
        | (x << 40 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff000000000000 as std::ffi::c_ulonglong
        | (x << 24 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff0000000000 as std::ffi::c_ulonglong
        | (x << 8 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff00000000 as std::ffi::c_ulonglong
        | (x >> 8 as std::ffi::c_int) as std::ffi::c_ulonglong
            & 0xff000000 as std::ffi::c_ulonglong
        | (x >> 24 as std::ffi::c_int) as std::ffi::c_ulonglong & 0xff0000 as std::ffi::c_ulonglong
        | (x >> 40 as std::ffi::c_int) as std::ffi::c_ulonglong & 0xff00 as std::ffi::c_ulonglong
        | (x >> 56 as std::ffi::c_int) as std::ffi::c_ulonglong & 0xff as std::ffi::c_ulonglong)
        as xxh_u64
}
#[inline(always)]
unsafe fn XXH_readLE64(mut ptr: *const std::ffi::c_void) -> xxh_u64 {
    if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_read64(ptr)
    } else {
        XXH_swap64(XXH_read64(ptr))
    }
}
unsafe fn XXH_readBE64(mut ptr: *const std::ffi::c_void) -> xxh_u64 {
    if XXH_CPU_LITTLE_ENDIAN != 0 {
        XXH_swap64(XXH_read64(ptr))
    } else {
        XXH_read64(ptr)
    }
}
#[inline(always)]
unsafe fn XXH_readLE64_align(
    mut ptr: *const std::ffi::c_void,
    mut align: XXH_alignment,
) -> xxh_u64 {
    if align as std::ffi::c_uint == XXH_unaligned as std::ffi::c_int as std::ffi::c_uint {
        XXH_readLE64(ptr)
    } else if XXH_CPU_LITTLE_ENDIAN != 0 {
        *(ptr as *const xxh_u64)
    } else {
        XXH_swap64(*(ptr as *const xxh_u64))
    }
}

const XXH_PRIME64_1: std::ffi::c_ulonglong = 0x9e3779b185ebca87 as std::ffi::c_ulonglong;
const XXH_PRIME64_2: std::ffi::c_ulonglong = 0xc2b2ae3d27d4eb4f as std::ffi::c_ulonglong;
const XXH_PRIME64_3: std::ffi::c_ulonglong = 0x165667b19e3779f9 as std::ffi::c_ulonglong;
const XXH_PRIME64_4: std::ffi::c_ulonglong = 0x85ebca77c2b2ae63 as std::ffi::c_ulonglong;
const XXH_PRIME64_5: std::ffi::c_ulonglong = 0x27d4eb2f165667c5 as std::ffi::c_ulonglong;

unsafe fn XXH64_round(mut acc: xxh_u64, mut input: xxh_u64) -> xxh_u64 {
    acc = (acc as std::ffi::c_ulonglong)
        .wrapping_add((input as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2))
        as xxh_u64 as xxh_u64;
    acc = ::core::intrinsics::rotate_left(acc, 31 as std::ffi::c_int as std::ffi::c_ulong as u32);
    acc = (acc as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_1) as xxh_u64 as xxh_u64;
    acc
}
unsafe fn XXH64_mergeRound(mut acc: xxh_u64, mut val: xxh_u64) -> xxh_u64 {
    val = XXH64_round(0 as std::ffi::c_int as xxh_u64, val);
    acc ^= val;
    acc = (acc as std::ffi::c_ulonglong)
        .wrapping_mul(XXH_PRIME64_1)
        .wrapping_add(XXH_PRIME64_4) as xxh_u64;
    acc
}
unsafe fn XXH64_avalanche(mut hash: xxh_u64) -> xxh_u64 {
    hash ^= hash >> 33 as std::ffi::c_int;
    hash = (hash as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_2) as xxh_u64 as xxh_u64;
    hash ^= hash >> 29 as std::ffi::c_int;
    hash = (hash as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_3) as xxh_u64 as xxh_u64;
    hash ^= hash >> 32 as std::ffi::c_int;
    hash
}
unsafe fn XXH64_finalize(
    mut hash: xxh_u64,
    mut ptr: *const xxh_u8,
    mut len: size_t,
    mut align: XXH_alignment,
) -> xxh_u64 {
    if ptr.is_null() {
        ::core::hint::assert_unchecked(len == 0 as std::ffi::c_int as size_t);
    }
    len &= 31 as std::ffi::c_int as size_t;
    while len >= 8 as std::ffi::c_int as size_t {
        let k1 = XXH64_round(
            0 as std::ffi::c_int as xxh_u64,
            XXH_readLE64_align(ptr as *const std::ffi::c_void, align),
        );
        ptr = ptr.offset(8 as std::ffi::c_int as isize);
        hash ^= k1;
        hash = (::core::intrinsics::rotate_left(
            hash,
            27 as std::ffi::c_int as std::ffi::c_ulong as u32,
        ) as std::ffi::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_1)
            .wrapping_add(XXH_PRIME64_4) as xxh_u64;
        len = len.wrapping_sub(8 as std::ffi::c_int as size_t);
    }
    if len >= 4 as std::ffi::c_int as size_t {
        hash = (hash as std::ffi::c_ulonglong
            ^ (XXH_readLE32_align(ptr as *const std::ffi::c_void, align) as xxh_u64
                as std::ffi::c_ulonglong)
                .wrapping_mul(XXH_PRIME64_1)) as xxh_u64;
        ptr = ptr.offset(4 as std::ffi::c_int as isize);
        hash = (::core::intrinsics::rotate_left(
            hash,
            23 as std::ffi::c_int as std::ffi::c_ulong as u32,
        ) as std::ffi::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_2)
            .wrapping_add(XXH_PRIME64_3) as xxh_u64;
        len = len.wrapping_sub(4 as std::ffi::c_int as size_t);
    }
    while len > 0 as std::ffi::c_int as size_t {
        let fresh7 = ptr;
        ptr = ptr.offset(1);
        hash = (hash as std::ffi::c_ulonglong
            ^ (*fresh7 as std::ffi::c_ulonglong).wrapping_mul(XXH_PRIME64_5))
            as xxh_u64;
        hash = (::core::intrinsics::rotate_left(
            hash,
            11 as std::ffi::c_int as std::ffi::c_ulong as u32,
        ) as std::ffi::c_ulonglong)
            .wrapping_mul(XXH_PRIME64_1) as xxh_u64;
        len = len.wrapping_sub(1);
        len;
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
    XXH64_finalize(h64, remainder.as_ptr().cast(), input.len(), align)
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
unsafe fn ZSTD_XXH64_reset(
    mut statePtr: *mut XXH64_state_t,
    mut seed: XXH64_hash_t,
) -> XXH_errorcode {
    ::core::hint::assert_unchecked(!statePtr.is_null());
    memset(
        statePtr as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<XXH64_state_t>(),
    );
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(0 as std::ffi::c_int as isize) = (seed as std::ffi::c_ulonglong)
        .wrapping_add(XXH_PRIME64_1)
        .wrapping_add(XXH_PRIME64_2)
        as XXH64_hash_t;
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(1 as std::ffi::c_int as isize) =
        (seed as std::ffi::c_ulonglong).wrapping_add(XXH_PRIME64_2) as XXH64_hash_t;
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(2 as std::ffi::c_int as isize) =
        seed.wrapping_add(0 as std::ffi::c_int as XXH64_hash_t);
    *((*statePtr).v)
        .as_mut_ptr()
        .offset(3 as std::ffi::c_int as isize) =
        (seed as std::ffi::c_ulonglong).wrapping_sub(XXH_PRIME64_1) as XXH64_hash_t;
    XXH_OK
}
#[no_mangle]
unsafe fn ZSTD_XXH64_update(
    mut state: *mut XXH64_state_t,
    mut input: *const std::ffi::c_void,
    mut len: size_t,
) -> XXH_errorcode {
    if input.is_null() {
        ::core::hint::assert_unchecked(len == 0 as std::ffi::c_int as size_t);
        return XXH_OK;
    }
    let mut p = input as *const xxh_u8;
    let bEnd = p.offset(len as isize);
    (*state).total_len = ((*state).total_len as std::ffi::c_ulong).wrapping_add(len as u64)
        as XXH64_hash_t as XXH64_hash_t;
    if ((*state).memsize as size_t).wrapping_add(len) < 32 as std::ffi::c_int as size_t {
        XXH_memcpy(
            (((*state).mem64).as_mut_ptr() as *mut xxh_u8).offset((*state).memsize as isize)
                as *mut std::ffi::c_void,
            input,
            len,
        );
        (*state).memsize = ((*state).memsize).wrapping_add(len as xxh_u32);
        return XXH_OK;
    }
    if (*state).memsize != 0 {
        XXH_memcpy(
            (((*state).mem64).as_mut_ptr() as *mut xxh_u8).offset((*state).memsize as isize)
                as *mut std::ffi::c_void,
            input,
            (32 as std::ffi::c_int as XXH32_hash_t).wrapping_sub((*state).memsize) as size_t,
        );
        *((*state).v)
            .as_mut_ptr()
            .offset(0 as std::ffi::c_int as isize) = XXH64_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(0 as std::ffi::c_int as isize),
            XXH_readLE64(
                ((*state).mem64)
                    .as_mut_ptr()
                    .offset(0 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        *((*state).v)
            .as_mut_ptr()
            .offset(1 as std::ffi::c_int as isize) = XXH64_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(1 as std::ffi::c_int as isize),
            XXH_readLE64(
                ((*state).mem64)
                    .as_mut_ptr()
                    .offset(1 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        *((*state).v)
            .as_mut_ptr()
            .offset(2 as std::ffi::c_int as isize) = XXH64_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(2 as std::ffi::c_int as isize),
            XXH_readLE64(
                ((*state).mem64)
                    .as_mut_ptr()
                    .offset(2 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        *((*state).v)
            .as_mut_ptr()
            .offset(3 as std::ffi::c_int as isize) = XXH64_round(
            *((*state).v)
                .as_mut_ptr()
                .offset(3 as std::ffi::c_int as isize),
            XXH_readLE64(
                ((*state).mem64)
                    .as_mut_ptr()
                    .offset(3 as std::ffi::c_int as isize)
                    as *const std::ffi::c_void,
            ),
        );
        p = p.offset(
            (32 as std::ffi::c_int as XXH32_hash_t).wrapping_sub((*state).memsize) as isize,
        );
        (*state).memsize = 0 as std::ffi::c_int as XXH32_hash_t;
    }
    if p.offset(32 as std::ffi::c_int as isize) <= bEnd {
        let limit = bEnd.offset(-(32 as std::ffi::c_int as isize));
        loop {
            *((*state).v)
                .as_mut_ptr()
                .offset(0 as std::ffi::c_int as isize) = XXH64_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(0 as std::ffi::c_int as isize),
                XXH_readLE64(p as *const std::ffi::c_void),
            );
            p = p.offset(8 as std::ffi::c_int as isize);
            *((*state).v)
                .as_mut_ptr()
                .offset(1 as std::ffi::c_int as isize) = XXH64_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(1 as std::ffi::c_int as isize),
                XXH_readLE64(p as *const std::ffi::c_void),
            );
            p = p.offset(8 as std::ffi::c_int as isize);
            *((*state).v)
                .as_mut_ptr()
                .offset(2 as std::ffi::c_int as isize) = XXH64_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(2 as std::ffi::c_int as isize),
                XXH_readLE64(p as *const std::ffi::c_void),
            );
            p = p.offset(8 as std::ffi::c_int as isize);
            *((*state).v)
                .as_mut_ptr()
                .offset(3 as std::ffi::c_int as isize) = XXH64_round(
                *((*state).v)
                    .as_mut_ptr()
                    .offset(3 as std::ffi::c_int as isize),
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
            ((*state).mem64).as_mut_ptr() as *mut std::ffi::c_void,
            p as *const std::ffi::c_void,
            bEnd.offset_from(p) as std::ffi::c_long as size_t,
        );
        (*state).memsize = bEnd.offset_from(p) as std::ffi::c_long as std::ffi::c_uint;
    }
    XXH_OK
}
#[no_mangle]
pub unsafe fn ZSTD_XXH64_digest(mut state: *const XXH64_state_t) -> XXH64_hash_t {
    let mut h64: xxh_u64 = 0;
    if (*state).total_len >= 32 as std::ffi::c_int as XXH64_hash_t {
        h64 = (::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(0 as std::ffi::c_int as isize),
            1 as std::ffi::c_int as std::ffi::c_ulong as u32,
        ))
        .wrapping_add(::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(1 as std::ffi::c_int as isize),
            7 as std::ffi::c_int as std::ffi::c_ulong as u32,
        ))
        .wrapping_add(::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(2 as std::ffi::c_int as isize),
            12 as std::ffi::c_int as std::ffi::c_ulong as u32,
        ))
        .wrapping_add(::core::intrinsics::rotate_left(
            *((*state).v).as_ptr().offset(3 as std::ffi::c_int as isize),
            18 as std::ffi::c_int as std::ffi::c_ulong as u32,
        ));
        h64 = XXH64_mergeRound(
            h64,
            *((*state).v).as_ptr().offset(0 as std::ffi::c_int as isize),
        );
        h64 = XXH64_mergeRound(
            h64,
            *((*state).v).as_ptr().offset(1 as std::ffi::c_int as isize),
        );
        h64 = XXH64_mergeRound(
            h64,
            *((*state).v).as_ptr().offset(2 as std::ffi::c_int as isize),
        );
        h64 = XXH64_mergeRound(
            h64,
            *((*state).v).as_ptr().offset(3 as std::ffi::c_int as isize),
        );
    } else {
        h64 = (*((*state).v).as_ptr().offset(2 as std::ffi::c_int as isize)
            as std::ffi::c_ulonglong)
            .wrapping_add(XXH_PRIME64_5) as xxh_u64;
    }
    h64 = h64.wrapping_add((*state).total_len);
    XXH64_finalize(
        h64,
        ((*state).mem64).as_ptr() as *const xxh_u8,
        (*state).total_len as usize,
        XXH_aligned,
    )
}
unsafe fn ZSTD_XXH64_canonicalFromHash(mut dst: *mut XXH64_canonical_t, mut hash: XXH64_hash_t) {
    hash = XXH_swap64(hash);
    XXH_memcpy(
        dst as *mut std::ffi::c_void,
        &mut hash as *mut XXH64_hash_t as *const std::ffi::c_void,
        ::core::mem::size_of::<XXH64_canonical_t>(),
    );
}
unsafe fn ZSTD_XXH64_hashFromCanonical(mut src: *const XXH64_canonical_t) -> XXH64_hash_t {
    XXH_readBE64(src as *const std::ffi::c_void)
}
const NULL: std::ffi::c_int = 0 as std::ffi::c_int;

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::quickcheck;

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

    fn helper_u32(input: &[u8], seed: u32) -> u32 {
        unsafe { ZSTD_XXH32(input.as_ptr().cast(), input.len(), seed) }
    }

    quickcheck! {
        fn prop_xxh32_matches(input: Vec<u8>, seed: u32) -> bool {
            let expected = xxhash_rust::xxh32::xxh32(&input, seed);
            let actual = helper_u32(&input, seed);
            assert_eq!(expected, actual);
            expected == actual
        }
    }
}
