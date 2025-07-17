use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fwrite(
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
        _: std::ffi::c_ulong,
        _: *mut FILE,
    ) -> std::ffi::c_ulong;
    fn perror(__s: *const std::ffi::c_char);
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn exit(_: std::ffi::c_int) -> !;
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
}
pub type size_t = std::ffi::c_ulong;
pub type BYTE = uint8_t;
pub type uint8_t = __uint8_t;
pub type __uint8_t = std::ffi::c_uchar;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    pub _flags2: std::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type __off64_t = std::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = std::ffi::c_long;
pub type U64 = uint64_t;
pub type uint64_t = __uint64_t;
pub type __uint64_t = std::ffi::c_ulong;
pub type U32 = uint32_t;
pub type uint32_t = __uint32_t;
pub type __uint32_t = std::ffi::c_uint;
pub type fixedPoint_24_8 = U32;
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const LTLOG: std::ffi::c_int = 13 as std::ffi::c_int;
pub const LTSIZE: std::ffi::c_int = (1 as std::ffi::c_int) << LTLOG;
pub const LTMASK: std::ffi::c_int = LTSIZE - 1 as std::ffi::c_int;
unsafe extern "C" fn RDG_rand(mut src: *mut U32) -> U32 {
    static mut prime1: U32 = 2654435761 as std::ffi::c_uint;
    static mut prime2: U32 = 2246822519 as std::ffi::c_uint;
    let mut rand32 = *src;
    rand32 = rand32 * prime1;
    rand32 ^= prime2;
    rand32 =
        rand32 << 13 as std::ffi::c_int | rand32 >> 32 as std::ffi::c_int - 13 as std::ffi::c_int;
    *src = rand32;
    return rand32 >> 5 as std::ffi::c_int;
}
unsafe extern "C" fn RDG_fillLiteralDistrib(mut ldt: *mut BYTE, mut ld: fixedPoint_24_8) {
    let firstChar = (if ld as std::ffi::c_double <= 0.0f64 {
        0 as std::ffi::c_int
    } else {
        '(' as i32
    }) as BYTE;
    let lastChar = (if ld as std::ffi::c_double <= 0.0f64 {
        255 as std::ffi::c_int
    } else {
        '}' as i32
    }) as BYTE;
    let mut character = (if ld as std::ffi::c_double <= 0.0f64 {
        0 as std::ffi::c_int
    } else {
        '0' as i32
    }) as BYTE;
    let mut u: U32 = 0;
    if ld <= 0 as std::ffi::c_int as fixedPoint_24_8 {
        ld = 0 as std::ffi::c_int as fixedPoint_24_8;
    }
    u = 0 as std::ffi::c_int as U32;
    while u < LTSIZE as U32 {
        let weight = ((LTSIZE as U32).wrapping_sub(u) * ld >> 8 as std::ffi::c_int)
            .wrapping_add(1 as std::ffi::c_int as U32);
        let end =
            if u.wrapping_add(weight) < ((1 as std::ffi::c_int) << 13 as std::ffi::c_int) as U32 {
                u.wrapping_add(weight)
            } else {
                ((1 as std::ffi::c_int) << 13 as std::ffi::c_int) as U32
            };
        while u < end {
            let fresh0 = u;
            u = u.wrapping_add(1);
            *ldt.offset(fresh0 as isize) = character;
        }
        character = character.wrapping_add(1);
        character;
        if character as std::ffi::c_int > lastChar as std::ffi::c_int {
            character = firstChar;
        }
    }
}
unsafe extern "C" fn RDG_genChar(mut seed: *mut U32, mut ldt: *const BYTE) -> BYTE {
    let id = RDG_rand(seed) & LTMASK as U32;
    return *ldt.offset(id as isize);
}
unsafe extern "C" fn RDG_rand15Bits(mut seedPtr: *mut U32) -> U32 {
    return RDG_rand(seedPtr) & 0x7fff as std::ffi::c_int as U32;
}
unsafe extern "C" fn RDG_randLength(mut seedPtr: *mut U32) -> U32 {
    if RDG_rand(seedPtr) & 7 as std::ffi::c_int as U32 != 0 {
        return RDG_rand(seedPtr) & 0xf as std::ffi::c_int as U32;
    }
    return (RDG_rand(seedPtr) & 0x1ff as std::ffi::c_int as U32)
        .wrapping_add(0xf as std::ffi::c_int as U32);
}
unsafe extern "C" fn RDG_genBlock(
    mut buffer: *mut std::ffi::c_void,
    mut buffSize: size_t,
    mut prefixSize: size_t,
    mut matchProba: std::ffi::c_double,
    mut ldt: *const BYTE,
    mut seedPtr: *mut U32,
) {
    let buffPtr = buffer as *mut BYTE;
    let matchProba32 = (32768 as std::ffi::c_int as std::ffi::c_double * matchProba) as U32;
    let mut pos = prefixSize;
    let mut prevOffset = 1 as std::ffi::c_int as U32;
    while matchProba >= 1.0f64 {
        let mut size0 = (RDG_rand(seedPtr) & 3 as std::ffi::c_int as U32) as size_t;
        size0 = (1 as std::ffi::c_int as size_t)
            << (16 as std::ffi::c_int as size_t)
                .wrapping_add(size0 * 2 as std::ffi::c_int as size_t);
        size0 = size0.wrapping_add(
            RDG_rand(seedPtr) as size_t & size0.wrapping_sub(1 as std::ffi::c_int as size_t),
        );
        if buffSize < pos.wrapping_add(size0) {
            memset(
                buffPtr.offset(pos as isize) as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                buffSize.wrapping_sub(pos),
            );
            return;
        }
        memset(
            buffPtr.offset(pos as isize) as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            size0,
        );
        pos = pos.wrapping_add(size0);
        *buffPtr.offset(pos.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) =
            RDG_genChar(seedPtr, ldt);
    }
    if pos == 0 as std::ffi::c_int as size_t {
        *buffPtr.offset(0 as std::ffi::c_int as isize) = RDG_genChar(seedPtr, ldt);
        pos = 1 as std::ffi::c_int as size_t;
    }
    while pos < buffSize {
        if RDG_rand15Bits(seedPtr) < matchProba32 {
            let length = (RDG_randLength(seedPtr)).wrapping_add(4 as std::ffi::c_int as U32);
            let d = (if pos.wrapping_add(length as size_t) < buffSize {
                pos.wrapping_add(length as size_t)
            } else {
                buffSize
            }) as U32;
            let repeatOffset = (RDG_rand(seedPtr) & 15 as std::ffi::c_int as U32
                == 2 as std::ffi::c_int as U32) as std::ffi::c_int
                as U32;
            let randOffset = (RDG_rand15Bits(seedPtr)).wrapping_add(1 as std::ffi::c_int as U32);
            let offset = if repeatOffset != 0 {
                prevOffset
            } else {
                (if (randOffset as size_t) < pos {
                    randOffset as size_t
                } else {
                    pos
                }) as U32
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
            }) as U32;
            while pos < d_0 as size_t {
                let fresh3 = pos;
                pos = pos.wrapping_add(1);
                *buffPtr.offset(fresh3 as isize) = RDG_genChar(seedPtr, ldt);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genBuffer(
    mut buffer: *mut std::ffi::c_void,
    mut size: size_t,
    mut matchProba: std::ffi::c_double,
    mut litProba: std::ffi::c_double,
    mut seed: std::ffi::c_uint,
) {
    let mut seed32 = seed;
    let mut ldt: [BYTE; 8192] = [0; 8192];
    memset(
        ldt.as_mut_ptr() as *mut std::ffi::c_void,
        '0' as i32,
        ::core::mem::size_of::<[BYTE; 8192]>() as std::ffi::c_ulong,
    );
    if litProba <= 0.0f64 {
        litProba = matchProba / 4.5f64;
    }
    RDG_fillLiteralDistrib(
        ldt.as_mut_ptr(),
        (litProba * 256 as std::ffi::c_int as std::ffi::c_double + 0.001f64) as fixedPoint_24_8,
    );
    RDG_genBlock(
        buffer,
        size,
        0 as std::ffi::c_int as size_t,
        matchProba,
        ldt.as_mut_ptr(),
        &mut seed32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RDG_genStdout(
    mut size: std::ffi::c_ulonglong,
    mut matchProba: std::ffi::c_double,
    mut litProba: std::ffi::c_double,
    mut seed: std::ffi::c_uint,
) {
    let mut seed32 = seed;
    let stdBlockSize =
        (128 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as size_t;
    let stdDictSize =
        (32 as std::ffi::c_int * ((1 as std::ffi::c_int) << 10 as std::ffi::c_int)) as size_t;
    let buff = malloc(stdDictSize.wrapping_add(stdBlockSize)) as *mut BYTE;
    let mut total = 0 as std::ffi::c_int as U64;
    let mut ldt: [BYTE; 8192] = [0; 8192];
    if buff.is_null() {
        perror(b"datagen\0" as *const u8 as *const std::ffi::c_char);
        exit(1 as std::ffi::c_int);
    }
    if litProba <= 0.0f64 {
        litProba = matchProba / 4.5f64;
    }
    memset(
        ldt.as_mut_ptr() as *mut std::ffi::c_void,
        '0' as i32,
        ::core::mem::size_of::<[BYTE; 8192]>() as std::ffi::c_ulong,
    );
    RDG_fillLiteralDistrib(
        ldt.as_mut_ptr(),
        (litProba * 256 as std::ffi::c_int as std::ffi::c_double + 0.001f64) as fixedPoint_24_8,
    );
    RDG_genBlock(
        buff as *mut std::ffi::c_void,
        stdDictSize,
        0 as std::ffi::c_int as size_t,
        matchProba,
        ldt.as_mut_ptr(),
        &mut seed32,
    );
    while (total as std::ffi::c_ulonglong) < size {
        let genBlockSize = (if (stdBlockSize as std::ffi::c_ulonglong)
            < size.wrapping_sub(total as std::ffi::c_ulonglong)
        {
            stdBlockSize as std::ffi::c_ulonglong
        } else {
            size.wrapping_sub(total as std::ffi::c_ulonglong)
        }) as size_t;
        RDG_genBlock(
            buff as *mut std::ffi::c_void,
            stdDictSize.wrapping_add(stdBlockSize),
            stdDictSize,
            matchProba,
            ldt.as_mut_ptr(),
            &mut seed32,
        );
        total = (total as std::ffi::c_ulong).wrapping_add(genBlockSize) as U64 as U64;
        let unused = fwrite(
            buff as *const std::ffi::c_void,
            1 as std::ffi::c_int as std::ffi::c_ulong,
            genBlockSize,
            stdout,
        );
        memcpy(
            buff as *mut std::ffi::c_void,
            buff.offset(stdBlockSize as isize) as *const std::ffi::c_void,
            stdDictSize,
        );
    }
    free(buff as *mut std::ffi::c_void);
}
