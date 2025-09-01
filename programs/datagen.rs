use core::ptr;

use libc::{exit, perror, FILE};

extern "C" {
    static mut stdout: *mut FILE;
    fn fwrite(
        _: *const core::ffi::c_void,
        _: core::ffi::c_ulong,
        _: core::ffi::c_ulong,
        _: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn malloc(_: core::ffi::c_ulong) -> *mut core::ffi::c_void;
    fn free(_: *mut core::ffi::c_void);
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: core::ffi::c_ulong,
    ) -> *mut core::ffi::c_void;
    fn memset(
        _: *mut core::ffi::c_void,
        _: core::ffi::c_int,
        _: core::ffi::c_ulong,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = core::ffi::c_ulong;
pub type fixedPoint_24_8 = u32;
pub const NULL: core::ffi::c_int = 0;
pub const LTLOG: core::ffi::c_int = 13;
pub const LTSIZE: core::ffi::c_int = (1) << LTLOG;
pub const LTMASK: core::ffi::c_int = LTSIZE - 1;
unsafe fn RDG_rand(mut src: *mut u32) -> u32 {
    static prime1: u32 = 2654435761;
    static prime2: u32 = 2246822519;
    let mut rand32 = *src;
    rand32 *= prime1;
    rand32 ^= prime2;
    rand32 = rand32 << 13 | rand32 >> (32 - 13);
    *src = rand32;
    rand32 >> 5
}
unsafe fn RDG_fillLiteralDistrib(mut ldt: *mut u8, mut ld: fixedPoint_24_8) {
    let firstChar = (if ld as core::ffi::c_double <= 0.0f64 {
        0
    } else {
        '(' as i32
    }) as u8;
    let lastChar = (if ld as core::ffi::c_double <= 0.0f64 {
        255
    } else {
        '}' as i32
    }) as u8;
    let mut character = (if ld as core::ffi::c_double <= 0.0f64 {
        0
    } else {
        '0' as i32
    }) as u8;
    let mut u: u32 = 0;
    if ld <= 0 {
        ld = 0;
    }
    u = 0;
    while u < LTSIZE as u32 {
        let weight = (((LTSIZE as u32).wrapping_sub(u) * ld) >> 8).wrapping_add(1);
        let end = if u.wrapping_add(weight) < ((1) << 13) as u32 {
            u.wrapping_add(weight)
        } else {
            ((1) << 13) as u32
        };
        while u < end {
            let fresh0 = u;
            u = u.wrapping_add(1);
            *ldt.offset(fresh0 as isize) = character;
        }
        character = character.wrapping_add(1);
        if character as core::ffi::c_int > lastChar as core::ffi::c_int {
            character = firstChar;
        }
    }
}
unsafe fn RDG_genChar(mut seed: *mut u32, mut ldt: *const u8) -> u8 {
    let id = RDG_rand(seed) & LTMASK as u32;
    *ldt.offset(id as isize)
}
unsafe fn RDG_rand15Bits(mut seedPtr: *mut u32) -> u32 {
    RDG_rand(seedPtr) & 0x7fff as core::ffi::c_int as u32
}
unsafe fn RDG_randLength(mut seedPtr: *mut u32) -> u32 {
    if RDG_rand(seedPtr) & 7 != 0 {
        return RDG_rand(seedPtr) & 0xf as core::ffi::c_int as u32;
    }
    (RDG_rand(seedPtr) & 0x1ff as core::ffi::c_int as u32)
        .wrapping_add(0xf as core::ffi::c_int as u32)
}
unsafe fn RDG_genBlock(
    mut buffer: *mut core::ffi::c_void,
    mut buffSize: size_t,
    mut prefixSize: size_t,
    mut matchProba: core::ffi::c_double,
    mut ldt: *const u8,
    mut seedPtr: *mut u32,
) {
    let buffPtr = buffer as *mut u8;
    let matchProba32 = (32768.0f64 * matchProba) as u32;
    let mut pos = prefixSize;
    let mut prevOffset = 1;
    while matchProba >= 1.0f64 {
        let mut size0 = (RDG_rand(seedPtr) & 3u32) as size_t;
        size0 = (1 as size_t) << (16 as size_t).wrapping_add(size0 * 2);
        size0 = size0.wrapping_add(RDG_rand(seedPtr) as size_t & size0.wrapping_sub(1));
        if buffSize < pos.wrapping_add(size0) {
            memset(
                buffPtr.offset(pos as isize) as *mut core::ffi::c_void,
                0,
                buffSize.wrapping_sub(pos),
            );
            return;
        }
        memset(
            buffPtr.offset(pos as isize) as *mut core::ffi::c_void,
            0,
            size0,
        );
        pos = pos.wrapping_add(size0);
        *buffPtr.offset(pos.wrapping_sub(1) as isize) = RDG_genChar(seedPtr, ldt);
    }
    if pos == 0 {
        *buffPtr.offset(0) = RDG_genChar(seedPtr, ldt);
        pos = 1;
    }
    while pos < buffSize {
        if RDG_rand15Bits(seedPtr) < matchProba32 {
            let length = (RDG_randLength(seedPtr)).wrapping_add(4);
            let d = (if pos.wrapping_add(length as size_t) < buffSize {
                pos.wrapping_add(length as size_t)
            } else {
                buffSize
            }) as u32;
            let repeatOffset = (RDG_rand(seedPtr) & 15 == 2) as core::ffi::c_int as u32;
            let randOffset = (RDG_rand15Bits(seedPtr)).wrapping_add(1);
            let offset = if repeatOffset != 0 {
                prevOffset
            } else {
                (if (randOffset as size_t) < pos {
                    randOffset as size_t
                } else {
                    pos
                }) as u32
            };
            let mut match_0 = pos.wrapping_sub(offset as size_t);
            while pos < d as size_t {
                let fresh1 = match_0;
                match_0 = match_0.wrapping_add(1);
                let fresh2 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh2 as isize) = *buffPtr.offset(fresh1 as isize);
            }
            prevOffset = offset;
        } else {
            let length_0 = RDG_randLength(seedPtr);
            let d_0 = (if pos.wrapping_add(length_0 as size_t) < buffSize {
                pos.wrapping_add(length_0 as size_t)
            } else {
                buffSize
            }) as u32;
            while pos < d_0 as size_t {
                let fresh3 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh3 as isize) = RDG_genChar(seedPtr, ldt);
            }
        }
    }
}
pub unsafe fn RDG_genBuffer(
    mut buffer: *mut core::ffi::c_void,
    mut size: size_t,
    mut matchProba: core::ffi::c_double,
    mut litProba: core::ffi::c_double,
    mut seed: core::ffi::c_uint,
) {
    let mut seed32 = seed;
    let mut ldt: [u8; 8192] = [0; 8192];
    ptr::write_bytes(
        ldt.as_mut_ptr() as *mut u8,
        b'0',
        ::core::mem::size_of::<[u8; 8192]>(),
    );
    if litProba <= 0.0f64 {
        litProba = matchProba / 4.5f64;
    }
    RDG_fillLiteralDistrib(
        ldt.as_mut_ptr(),
        (litProba * 256.0 + 0.001f64) as fixedPoint_24_8,
    );
    RDG_genBlock(buffer, size, 0, matchProba, ldt.as_mut_ptr(), &mut seed32);
}
pub unsafe fn RDG_genStdout(
    mut size: core::ffi::c_ulonglong,
    mut matchProba: core::ffi::c_double,
    mut litProba: core::ffi::c_double,
    mut seed: core::ffi::c_uint,
) {
    let mut seed32 = seed;
    let stdBlockSize = (128 * ((1) << 10)) as size_t;
    let stdDictSize = (32 * ((1) << 10)) as size_t;
    let buff = malloc(stdDictSize.wrapping_add(stdBlockSize)) as *mut u8;
    let mut total = 0;
    let mut ldt: [u8; 8192] = [0; 8192];
    if buff.is_null() {
        perror(b"datagen\0" as *const u8 as *const core::ffi::c_char);
        exit(1);
    }
    if litProba <= 0.0f64 {
        litProba = matchProba / 4.5f64;
    }
    ptr::write_bytes(
        ldt.as_mut_ptr() as *mut u8,
        b'0',
        ::core::mem::size_of::<[u8; 8192]>(),
    );
    RDG_fillLiteralDistrib(
        ldt.as_mut_ptr(),
        (litProba * 256.0 + 0.001f64) as fixedPoint_24_8,
    );
    RDG_genBlock(
        buff as *mut core::ffi::c_void,
        stdDictSize,
        0,
        matchProba,
        ldt.as_mut_ptr(),
        &mut seed32,
    );
    while (total as core::ffi::c_ulonglong) < size {
        let genBlockSize = (if (stdBlockSize as core::ffi::c_ulonglong)
            < size.wrapping_sub(total as core::ffi::c_ulonglong)
        {
            stdBlockSize as core::ffi::c_ulonglong
        } else {
            size.wrapping_sub(total as core::ffi::c_ulonglong)
        }) as size_t;
        RDG_genBlock(
            buff as *mut core::ffi::c_void,
            stdDictSize.wrapping_add(stdBlockSize),
            stdDictSize,
            matchProba,
            ldt.as_mut_ptr(),
            &mut seed32,
        );
        total = (total as core::ffi::c_ulong).wrapping_add(genBlockSize) as u64 as u64;
        let unused = fwrite(buff as *const core::ffi::c_void, 1, genBlockSize, stdout);
        memcpy(
            buff as *mut core::ffi::c_void,
            buff.offset(stdBlockSize as isize) as *const core::ffi::c_void,
            stdDictSize,
        );
    }
    free(buff as *mut core::ffi::c_void);
}
