use libc::{free, malloc, size_t};

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
pub type trbudget_t = _trbudget_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trbudget_t {
    pub chance: core::ffi::c_int,
    pub remain: core::ffi::c_int,
    pub incval: core::ffi::c_int,
    pub count: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub a: *const core::ffi::c_int,
    pub b: *mut core::ffi::c_int,
    pub c: *mut core::ffi::c_int,
    pub d: core::ffi::c_int,
    pub e: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub a: *mut core::ffi::c_int,
    pub b: *mut core::ffi::c_int,
    pub c: core::ffi::c_int,
    pub d: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub a: *mut core::ffi::c_int,
    pub b: *mut core::ffi::c_int,
    pub c: *mut core::ffi::c_int,
    pub d: core::ffi::c_int,
}
pub const NULL: core::ffi::c_int = 0;
pub const ALPHABET_SIZE: core::ffi::c_int = 256;
pub const BUCKET_A_SIZE: core::ffi::c_int = 256;
pub const BUCKET_B_SIZE: core::ffi::c_int = ALPHABET_SIZE * ALPHABET_SIZE;
pub const SS_INSERTIONSORT_THRESHOLD: core::ffi::c_int = 8;
pub const SS_BLOCKSIZE: core::ffi::c_int = 1024;
pub const TR_INSERTIONSORT_THRESHOLD: core::ffi::c_int = 8;
static lg_table: [core::ffi::c_int; 256] = [
    -(1),
    0,
    1,
    1,
    2,
    2,
    2,
    2,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    5,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    6,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
    7,
];
#[inline]
unsafe fn ss_ilg(mut n: core::ffi::c_int) -> core::ffi::c_int {
    if n & 0xff00 as core::ffi::c_int != 0 {
        8 + *lg_table
            .as_ptr()
            .offset((n >> 8 & 0xff as core::ffi::c_int) as isize)
    } else {
        *lg_table
            .as_ptr()
            .offset((n & 0xff as core::ffi::c_int) as isize)
    }
}
static sqq_table: [core::ffi::c_int; 256] = [
    0, 16, 22, 27, 32, 35, 39, 42, 45, 48, 50, 53, 55, 57, 59, 61, 64, 65, 67, 69, 71, 73, 75, 76,
    78, 80, 81, 83, 84, 86, 87, 89, 90, 91, 93, 94, 96, 97, 98, 99, 101, 102, 103, 104, 106, 107,
    108, 109, 110, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 128,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 144, 145,
    146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155, 155, 156, 157, 158, 159, 160, 160, 161,
    162, 163, 163, 164, 165, 166, 167, 167, 168, 169, 170, 170, 171, 172, 173, 173, 174, 175, 176,
    176, 177, 178, 178, 179, 180, 181, 181, 182, 183, 183, 184, 185, 185, 186, 187, 187, 188, 189,
    189, 190, 191, 192, 192, 193, 193, 194, 195, 195, 196, 197, 197, 198, 199, 199, 200, 201, 201,
    202, 203, 203, 204, 204, 205, 206, 206, 207, 208, 208, 209, 209, 210, 211, 211, 212, 212, 213,
    214, 214, 215, 215, 216, 217, 217, 218, 218, 219, 219, 220, 221, 221, 222, 222, 223, 224, 224,
    225, 225, 226, 226, 227, 227, 228, 229, 229, 230, 230, 231, 231, 232, 232, 233, 234, 234, 235,
    235, 236, 236, 237, 237, 238, 238, 239, 240, 240, 241, 241, 242, 242, 243, 243, 244, 244, 245,
    245, 246, 246, 247, 247, 248, 248, 249, 249, 250, 250, 251, 251, 252, 252, 253, 253, 254, 254,
    255,
];
#[inline]
unsafe fn ss_isqrt(mut x: core::ffi::c_int) -> core::ffi::c_int {
    let mut y: core::ffi::c_int = 0;
    let mut e: core::ffi::c_int = 0;
    if x >= SS_BLOCKSIZE * SS_BLOCKSIZE {
        return SS_BLOCKSIZE;
    }
    e = if x as core::ffi::c_uint & 0xffff0000 as core::ffi::c_uint != 0 {
        if x as core::ffi::c_uint & 0xff000000 as core::ffi::c_uint != 0 {
            24 + *lg_table
                .as_ptr()
                .offset((x >> 24 & 0xff as core::ffi::c_int) as isize)
        } else {
            16 + *lg_table
                .as_ptr()
                .offset((x >> 16 & 0xff as core::ffi::c_int) as isize)
        }
    } else if x & 0xff00 as core::ffi::c_int != 0 {
        8 + *lg_table
            .as_ptr()
            .offset((x >> 8 & 0xff as core::ffi::c_int) as isize)
    } else {
        *lg_table
            .as_ptr()
            .offset((x & 0xff as core::ffi::c_int) as isize)
    };
    if e >= 16 {
        y = *sqq_table.as_ptr().offset((x >> (e - 6 - (e & 1))) as isize) << ((e >> 1) - 7);
        if e >= 24 {
            y = (y + 1 + x / y) >> 1;
        }
        y = (y + 1 + x / y) >> 1;
    } else if e >= 8 {
        y = (*sqq_table.as_ptr().offset((x >> (e - 6 - (e & 1))) as isize) >> (7 - (e >> 1))) + 1;
    } else {
        return *sqq_table.as_ptr().offset(x as isize) >> 4;
    }
    if x < y * y {
        y - 1
    } else {
        y
    }
}
#[inline]
unsafe fn ss_compare(
    mut T: *const core::ffi::c_uchar,
    mut p1: *const core::ffi::c_int,
    mut p2: *const core::ffi::c_int,
    mut depth: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut U1 = core::ptr::null::<core::ffi::c_uchar>();
    let mut U2 = core::ptr::null::<core::ffi::c_uchar>();
    let mut U1n = core::ptr::null::<core::ffi::c_uchar>();
    let mut U2n = core::ptr::null::<core::ffi::c_uchar>();
    U1 = T.offset(depth as isize).offset(*p1 as isize);
    U2 = T.offset(depth as isize).offset(*p2 as isize);
    U1n = T.offset(*p1.offset(1) as isize).offset(2);
    U2n = T.offset(*p2.offset(1) as isize).offset(2);
    while U1 < U1n && U2 < U2n && *U1 as core::ffi::c_int == *U2 as core::ffi::c_int {
        U1 = U1.offset(1);
        U2 = U2.offset(1);
    }
    if U1 < U1n {
        if U2 < U2n {
            *U1 as core::ffi::c_int - *U2 as core::ffi::c_int
        } else {
            1
        }
    } else if U2 < U2n {
        -(1)
    } else {
        0
    }
}
unsafe fn ss_insertionsort(
    mut T: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut i = core::ptr::null_mut::<core::ffi::c_int>();
    let mut j = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    i = last.offset(-(2));
    while first <= i {
        t = *i;
        j = i.offset(1);
        loop {
            r = ss_compare(T, PA.offset(t as isize), PA.offset(*j as isize), depth);
            if (0) >= r {
                break;
            }
            loop {
                *j.offset(-(1)) = *j;
                j = j.offset(1);
                if !(j < last && *j < 0) {
                    break;
                }
            }
            if last <= j {
                break;
            }
        }
        if r == 0 {
            *j = !*j;
        }
        *j.offset(-(1)) = t;
        i = i.offset(-1);
    }
}
#[inline]
unsafe fn ss_fixdown(
    mut Td: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut SA: *mut core::ffi::c_int,
    mut i: core::ffi::c_int,
    mut size: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut v: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut d: core::ffi::c_int = 0;
    let mut e: core::ffi::c_int = 0;
    v = *SA.offset(i as isize);
    c = *Td.offset(*PA.offset(v as isize) as isize) as core::ffi::c_int;
    loop {
        j = 2 * i + 1;
        if j >= size {
            break;
        }
        let fresh0 = j;
        j += 1;
        k = fresh0;
        d = *Td.offset(*PA.offset(*SA.offset(k as isize) as isize) as isize) as core::ffi::c_int;
        e = *Td.offset(*PA.offset(*SA.offset(j as isize) as isize) as isize) as core::ffi::c_int;
        if d < e {
            k = j;
            d = e;
        }
        if d <= c {
            break;
        }
        *SA.offset(i as isize) = *SA.offset(k as isize);
        i = k;
    }
    *SA.offset(i as isize) = v;
}
unsafe fn ss_heapsort(
    mut Td: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut SA: *mut core::ffi::c_int,
    mut size: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut m: core::ffi::c_int = 0;
    let mut t: core::ffi::c_int = 0;
    m = size;
    if size % 2 == 0 {
        m -= 1;
        if (*Td.offset(*PA.offset(*SA.offset((m / 2) as isize) as isize) as isize)
            as core::ffi::c_int)
            < *Td.offset(*PA.offset(*SA.offset(m as isize) as isize) as isize) as core::ffi::c_int
        {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2) as isize);
            *SA.offset((m / 2) as isize) = t;
        }
    }
    i = m / 2 - 1;
    while 0 <= i {
        ss_fixdown(Td, PA, SA, i, m);
        i -= 1;
    }
    if size % 2 == 0 {
        t = *SA.offset(0);
        *SA.offset(0) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        ss_fixdown(Td, PA, SA, 0, m);
    }
    i = m - 1;
    while (0) < i {
        t = *SA.offset(0);
        *SA.offset(0) = *SA.offset(i as isize);
        ss_fixdown(Td, PA, SA, 0, i);
        *SA.offset(i as isize) = t;
        i -= 1;
    }
}
#[inline]
unsafe fn ss_median3(
    mut Td: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut v1: *mut core::ffi::c_int,
    mut v2: *mut core::ffi::c_int,
    mut v3: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut t = core::ptr::null_mut::<core::ffi::c_int>();
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v2 as isize) as isize) as core::ffi::c_int
    {
        t = v1;
        v1 = v2;
        v2 = t;
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as core::ffi::c_int
    {
        if *Td.offset(*PA.offset(*v1 as isize) as isize) as core::ffi::c_int
            > *Td.offset(*PA.offset(*v3 as isize) as isize) as core::ffi::c_int
        {
            return v1;
        } else {
            return v3;
        }
    }
    v2
}
#[inline]
unsafe fn ss_median5(
    mut Td: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut v1: *mut core::ffi::c_int,
    mut v2: *mut core::ffi::c_int,
    mut v3: *mut core::ffi::c_int,
    mut v4: *mut core::ffi::c_int,
    mut v5: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut t = core::ptr::null_mut::<core::ffi::c_int>();
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as core::ffi::c_int
    {
        t = v2;
        v2 = v3;
        v3 = t;
    }
    if *Td.offset(*PA.offset(*v4 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v5 as isize) as isize) as core::ffi::c_int
    {
        t = v4;
        v4 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as core::ffi::c_int
    {
        t = v2;
        v2 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as core::ffi::c_int
    {
        t = v1;
        v1 = v3;
        v3 = t;
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as core::ffi::c_int
    {
        t = v1;
        v1 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v3 as isize) as isize) as core::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as core::ffi::c_int
    {
        return v4;
    }
    v3
}
#[inline]
unsafe fn ss_pivot(
    mut Td: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut middle = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    t = last.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
    middle = first.offset((t / 2) as isize);
    if t <= 512 {
        if t <= 32 {
            return ss_median3(Td, PA, first, middle, last.offset(-(1)));
        } else {
            t >>= 2;
            return ss_median5(
                Td,
                PA,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1)).offset(-(t as isize)),
                last.offset(-(1)),
            );
        }
    }
    t >>= 3;
    first = ss_median3(
        Td,
        PA,
        first,
        first.offset(t as isize),
        first.offset((t << 1) as isize),
    );
    middle = ss_median3(
        Td,
        PA,
        middle.offset(-(t as isize)),
        middle,
        middle.offset(t as isize),
    );
    last = ss_median3(
        Td,
        PA,
        last.offset(-(1)).offset(-((t << 1) as isize)),
        last.offset(-(1)).offset(-(t as isize)),
        last.offset(-(1)),
    );
    ss_median3(Td, PA, first, middle, last)
}
#[inline]
unsafe fn ss_partition(
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    a = first.offset(-(1));
    b = last;
    loop {
        loop {
            a = a.offset(1);
            if !(a < b && *PA.offset(*a as isize) + depth >= *PA.offset((*a + 1) as isize) + 1) {
                break;
            }
            *a = !*a;
        }
        loop {
            b = b.offset(-1);
            if !(a < b && *PA.offset(*b as isize) + depth < *PA.offset((*b + 1) as isize) + 1) {
                break;
            }
        }
        if b <= a {
            break;
        }
        t = !*b;
        *b = *a;
        *a = t;
    }
    if first < a {
        *first = !*first;
    }
    a
}
unsafe fn ss_mintrosort(
    mut T: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut stack: [C2RustUnnamed_0; 16] = [C2RustUnnamed_0 {
        a: core::ptr::null_mut::<core::ffi::c_int>(),
        b: core::ptr::null_mut::<core::ffi::c_int>(),
        c: 0,
        d: 0,
    }; 16];
    let mut Td = core::ptr::null::<core::ffi::c_uchar>();
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut d = core::ptr::null_mut::<core::ffi::c_int>();
    let mut e = core::ptr::null_mut::<core::ffi::c_int>();
    let mut f = core::ptr::null_mut::<core::ffi::c_int>();
    let mut s: core::ffi::c_int = 0;
    let mut t: core::ffi::c_int = 0;
    let mut ssize: core::ffi::c_int = 0;
    let mut limit: core::ffi::c_int = 0;
    let mut v: core::ffi::c_int = 0;
    let mut x = 0;
    ssize = 0;
    limit = ss_ilg(last.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
    loop {
        if last.offset_from(first) as core::ffi::c_long
            <= SS_INSERTIONSORT_THRESHOLD as core::ffi::c_long
        {
            if (1) < last.offset_from(first) as core::ffi::c_long {
                ss_insertionsort(T, PA, first, last, depth);
            }
            if 0 <= ssize {
            } else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    418,
                    (*::core::mem::transmute::<
                        &[u8; 74],
                        &[core::ffi::c_char; 74],
                    >(
                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ssize == 0 {
                return;
            }
            ssize -= 1;
            first = (*stack.as_mut_ptr().offset(ssize as isize)).a;
            last = (*stack.as_mut_ptr().offset(ssize as isize)).b;
            depth = (*stack.as_mut_ptr().offset(ssize as isize)).c;
            limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
        } else {
            Td = T.offset(depth as isize);
            let fresh1 = limit;
            limit -= 1;
            if fresh1 == 0 {
                ss_heapsort(
                    Td,
                    PA,
                    first,
                    last.offset_from(first) as core::ffi::c_long as core::ffi::c_int,
                );
            }
            if limit < 0 {
                a = first.offset(1);
                v = *Td.offset(*PA.offset(*first as isize) as isize) as core::ffi::c_int;
                while a < last {
                    x = *Td.offset(*PA.offset(*a as isize) as isize) as core::ffi::c_int;
                    if x != v {
                        if (1) < a.offset_from(first) as core::ffi::c_long {
                            break;
                        }
                        v = x;
                        first = a;
                    }
                    a = a.offset(1);
                }
                if (*Td.offset((*PA.offset(*first as isize) - 1) as isize) as core::ffi::c_int) < v
                {
                    first = ss_partition(PA, first, a, depth);
                }
                if a.offset_from(first) as core::ffi::c_long
                    <= last.offset_from(a) as core::ffi::c_long
                {
                    if (1) < a.offset_from(first) as core::ffi::c_long {
                        if ssize < 16 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                437,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[core::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh2 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh2 = a;
                        let fresh3 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh3 = last;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh4 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh4 as isize)).d = -(1);
                        last = a;
                        depth += 1;
                        limit =
                            ss_ilg(a.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
                    } else {
                        first = a;
                        limit = -(1);
                    }
                } else if (1) < last.offset_from(a) as core::ffi::c_long {
                    if ssize < 16 {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            444,
                            (*::core::mem::transmute::<
                                &[u8; 74],
                                &[core::ffi::c_char; 74],
                            >(
                                b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh5 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh5 = first;
                    let fresh6 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh6 = a;
                    (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1;
                    let fresh7 = ssize;
                    ssize += 1;
                    (*stack.as_mut_ptr().offset(fresh7 as isize)).d =
                        ss_ilg(a.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
                    first = a;
                    limit = -(1);
                } else {
                    last = a;
                    depth += 1;
                    limit = ss_ilg(a.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
                }
            } else {
                a = ss_pivot(Td, PA, first, last);
                v = *Td.offset(*PA.offset(*a as isize) as isize) as core::ffi::c_int;
                t = *first;
                *first = *a;
                *a = t;
                b = first;
                loop {
                    b = b.offset(1);
                    if !(b < last && {
                        x = *Td.offset(*PA.offset(*b as isize) as isize) as core::ffi::c_int;
                        x == v
                    }) {
                        break;
                    }
                }
                a = b;
                if a < last && x < v {
                    loop {
                        b = b.offset(1);
                        if !(b < last && {
                            x = *Td.offset(*PA.offset(*b as isize) as isize) as core::ffi::c_int;
                            x <= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *b;
                            *b = *a;
                            *a = t;
                            a = a.offset(1);
                        }
                    }
                }
                c = last;
                loop {
                    c = c.offset(-1);
                    if !(b < c && {
                        x = *Td.offset(*PA.offset(*c as isize) as isize) as core::ffi::c_int;
                        x == v
                    }) {
                        break;
                    }
                }
                d = c;
                if b < d && x > v {
                    loop {
                        c = c.offset(-1);
                        if !(b < c && {
                            x = *Td.offset(*PA.offset(*c as isize) as isize) as core::ffi::c_int;
                            x >= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *c;
                            *c = *d;
                            *d = t;
                            d = d.offset(-1);
                        }
                    }
                }
                while b < c {
                    t = *b;
                    *b = *c;
                    *c = t;
                    loop {
                        b = b.offset(1);
                        if !(b < c && {
                            x = *Td.offset(*PA.offset(*b as isize) as isize) as core::ffi::c_int;
                            x <= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *b;
                            *b = *a;
                            *a = t;
                            a = a.offset(1);
                        }
                    }
                    loop {
                        c = c.offset(-1);
                        if !(b < c && {
                            x = *Td.offset(*PA.offset(*c as isize) as isize) as core::ffi::c_int;
                            x >= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *c;
                            *c = *d;
                            *d = t;
                            d = d.offset(-1);
                        }
                    }
                }
                if a <= d {
                    c = b.offset(-(1));
                    s = a.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
                    t = b.offset_from(a) as core::ffi::c_long as core::ffi::c_int;
                    if s > t {
                        s = t;
                    }
                    e = first;
                    f = b.offset(-(s as isize));
                    while (0) < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        e = e.offset(1);
                        f = f.offset(1);
                    }
                    s = d.offset_from(c) as core::ffi::c_long as core::ffi::c_int;
                    t = (last.offset_from(d) as core::ffi::c_long - 1) as core::ffi::c_int;
                    if s > t {
                        s = t;
                    }
                    e = b;
                    f = last.offset(-(s as isize));
                    while (0) < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        e = e.offset(1);
                        f = f.offset(1);
                    }
                    a = first.offset(b.offset_from(a) as core::ffi::c_long as isize);
                    c = last.offset(-(d.offset_from(c) as core::ffi::c_long as isize));
                    b = if v
                        <= *Td.offset((*PA.offset(*a as isize) - 1) as isize) as core::ffi::c_int
                    {
                        a
                    } else {
                        ss_partition(PA, a, c, depth)
                    };
                    if a.offset_from(first) as core::ffi::c_long
                        <= last.offset_from(c) as core::ffi::c_long
                    {
                        if last.offset_from(c) as core::ffi::c_long
                            <= c.offset_from(b) as core::ffi::c_long
                        {
                            if ssize < 16 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    494,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[core::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh8 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh8 = b;
                            let fresh9 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh9 = c;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1;
                            let fresh10 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh10 as isize)).d =
                                ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                            if ssize < 16 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    495,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[core::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh11 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh11 = c;
                            let fresh12 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh12 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh13 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh13 as isize)).d = limit;
                            last = a;
                        } else if a.offset_from(first) as core::ffi::c_long
                            <= c.offset_from(b) as core::ffi::c_long
                        {
                            if ssize < 16 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    498,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[core::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh14 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh14 = c;
                            let fresh15 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh15 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh16 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh16 as isize)).d = limit;
                            if ssize < 16 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    499,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[core::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh17 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh17 = b;
                            let fresh18 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh18 = c;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1;
                            let fresh19 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh19 as isize)).d =
                                ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                            last = a;
                        } else {
                            if ssize < 16 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    502,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[core::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh20 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh20 = c;
                            let fresh21 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh21 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh22 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh22 as isize)).d = limit;
                            if ssize < 16 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    503,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[core::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh23 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh23 = first;
                            let fresh24 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh24 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh25 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh25 as isize)).d = limit;
                            first = b;
                            last = c;
                            depth += 1;
                            limit =
                                ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                        }
                    } else if a.offset_from(first) as core::ffi::c_long
                        <= c.offset_from(b) as core::ffi::c_long
                    {
                        if ssize < 16 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                508,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[core::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh26 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh26 = b;
                        let fresh27 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh27 = c;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1;
                        let fresh28 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh28 as isize)).d =
                            ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                        if ssize < 16 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                509,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[core::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh29 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh29 = first;
                        let fresh30 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh30 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh31 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh31 as isize)).d = limit;
                        first = c;
                    } else if last.offset_from(c) as core::ffi::c_long
                        <= c.offset_from(b) as core::ffi::c_long
                    {
                        if ssize < 16 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                512,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[core::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh32 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh32 = first;
                        let fresh33 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh33 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh34 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh34 as isize)).d = limit;
                        if ssize < 16 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                513,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[core::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh35 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh35 = b;
                        let fresh36 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh36 = c;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1;
                        let fresh37 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh37 as isize)).d =
                            ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                        first = c;
                    } else {
                        if ssize < 16 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                516,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[core::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh38 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh38 = first;
                        let fresh39 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh39 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh40 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh40 as isize)).d = limit;
                        if ssize < 16 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                517,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[core::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh41 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh41 = c;
                        let fresh42 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh42 = last;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh43 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh43 as isize)).d = limit;
                        first = b;
                        last = c;
                        depth += 1;
                        limit = ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                    }
                } else {
                    limit += 1;
                    if (*Td.offset((*PA.offset(*first as isize) - 1) as isize) as core::ffi::c_int)
                        < v
                    {
                        first = ss_partition(PA, first, last, depth);
                        limit = ss_ilg(
                            last.offset_from(first) as core::ffi::c_long as core::ffi::c_int
                        );
                    }
                    depth += 1;
                }
            }
        }
    }
}
#[inline]
unsafe fn ss_blockswap(
    mut a: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
) {
    let mut t: core::ffi::c_int = 0;
    while (0) < n {
        t = *a;
        *a = *b;
        *b = t;
        n -= 1;
        a = a.offset(1);
        b = b.offset(1);
    }
}
#[inline]
unsafe fn ss_rotate(
    mut first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
) {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut l: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    l = middle.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
    r = last.offset_from(middle) as core::ffi::c_long as core::ffi::c_int;
    while (0) < l && (0) < r {
        if l == r {
            ss_blockswap(first, middle, l);
            break;
        } else if l < r {
            a = last.offset(-(1));
            b = middle.offset(-(1));
            t = *a;
            loop {
                let fresh44 = a;
                a = a.offset(-1);
                *fresh44 = *b;
                let fresh45 = b;
                b = b.offset(-1);
                *fresh45 = *a;
                if b >= first {
                    continue;
                }
                *a = t;
                last = a;
                r -= l + 1;
                if r <= l {
                    break;
                }
                a = a.offset(-(1));
                b = middle.offset(-(1));
                t = *a;
            }
        } else {
            a = first;
            b = middle;
            t = *a;
            loop {
                let fresh46 = a;
                a = a.offset(1);
                *fresh46 = *b;
                let fresh47 = b;
                b = b.offset(1);
                *fresh47 = *a;
                if last > b {
                    continue;
                }
                *a = t;
                first = a.offset(1);
                l -= r + 1;
                if l <= r {
                    break;
                }
                a = a.offset(1);
                b = middle;
                t = *a;
            }
        }
    }
}
unsafe fn ss_inplacemerge(
    mut T: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut p = core::ptr::null::<core::ffi::c_int>();
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut len: core::ffi::c_int = 0;
    let mut half: core::ffi::c_int = 0;
    let mut q: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    loop {
        if *last.offset(-(1)) < 0 {
            x = 1;
            p = PA.offset(!*last.offset(-(1)) as isize);
        } else {
            x = 0;
            p = PA.offset(*last.offset(-(1)) as isize);
        }
        a = first;
        len = middle.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
        half = len >> 1;
        r = -(1);
        while (0) < len {
            b = a.offset(half as isize);
            q = ss_compare(
                T,
                PA.offset((if 0 <= *b { *b } else { !*b }) as isize),
                p,
                depth,
            );
            if q < 0 {
                a = b.offset(1);
                half -= len & 1 ^ 1;
            } else {
                r = q;
            }
            len = half;
            half >>= 1;
        }
        if a < middle {
            if r == 0 {
                *a = !*a;
            }
            ss_rotate(a, middle, last);
            last = last.offset(-(middle.offset_from(a) as core::ffi::c_long as isize));
            middle = a;
            if first == middle {
                break;
            }
        }
        last = last.offset(-1);
        if x != 0 {
            loop {
                last = last.offset(-1);
                if *last >= 0 {
                    break;
                }
            }
        }
        if middle == last {
            break;
        }
    }
}
unsafe fn ss_mergeforward(
    mut T: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut buf: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut bufend = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    bufend = buf
        .offset(middle.offset_from(first) as core::ffi::c_long as isize)
        .offset(-(1));
    ss_blockswap(
        buf,
        first,
        middle.offset_from(first) as core::ffi::c_long as core::ffi::c_int,
    );
    a = first;
    t = *a;
    b = buf;
    c = middle;
    loop {
        r = ss_compare(T, PA.offset(*b as isize), PA.offset(*c as isize), depth);
        if r < 0 {
            loop {
                let fresh48 = a;
                a = a.offset(1);
                *fresh48 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh49 = b;
                b = b.offset(1);
                *fresh49 = *a;
                if *b >= 0 {
                    break;
                }
            }
        } else if r > 0 {
            loop {
                let fresh50 = a;
                a = a.offset(1);
                *fresh50 = *c;
                let fresh51 = c;
                c = c.offset(1);
                *fresh51 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh52 = a;
                        a = a.offset(1);
                        *fresh52 = *b;
                        let fresh53 = b;
                        b = b.offset(1);
                        *fresh53 = *a;
                    }
                    *a = *b;
                    *b = t;
                    return;
                }
                if *c >= 0 {
                    break;
                }
            }
        } else {
            *c = !*c;
            loop {
                let fresh54 = a;
                a = a.offset(1);
                *fresh54 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh55 = b;
                b = b.offset(1);
                *fresh55 = *a;
                if *b >= 0 {
                    break;
                }
            }
            loop {
                let fresh56 = a;
                a = a.offset(1);
                *fresh56 = *c;
                let fresh57 = c;
                c = c.offset(1);
                *fresh57 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh58 = a;
                        a = a.offset(1);
                        *fresh58 = *b;
                        let fresh59 = b;
                        b = b.offset(1);
                        *fresh59 = *a;
                    }
                    *a = *b;
                    *b = t;
                    return;
                }
                if *c >= 0 {
                    break;
                }
            }
        }
    }
}
unsafe fn ss_mergebackward(
    mut T: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut buf: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut p1 = core::ptr::null::<core::ffi::c_int>();
    let mut p2 = core::ptr::null::<core::ffi::c_int>();
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut bufend = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    let mut x: core::ffi::c_int = 0;
    bufend = buf
        .offset(last.offset_from(middle) as core::ffi::c_long as isize)
        .offset(-(1));
    ss_blockswap(
        buf,
        middle,
        last.offset_from(middle) as core::ffi::c_long as core::ffi::c_int,
    );
    x = 0;
    if *bufend < 0 {
        p1 = PA.offset(!*bufend as isize);
        x |= 1;
    } else {
        p1 = PA.offset(*bufend as isize);
    }
    if *middle.offset(-(1)) < 0 {
        p2 = PA.offset(!*middle.offset(-(1)) as isize);
        x |= 2;
    } else {
        p2 = PA.offset(*middle.offset(-(1)) as isize);
    }
    a = last.offset(-(1));
    t = *a;
    b = bufend;
    c = middle.offset(-(1));
    loop {
        r = ss_compare(T, p1, p2, depth);
        if (0) < r {
            if x & 1 != 0 {
                loop {
                    let fresh60 = a;
                    a = a.offset(-1);
                    *fresh60 = *b;
                    let fresh61 = b;
                    b = b.offset(-1);
                    *fresh61 = *a;
                    if *b >= 0 {
                        break;
                    }
                }
                x ^= 1;
            }
            let fresh62 = a;
            a = a.offset(-1);
            *fresh62 = *b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh63 = b;
                b = b.offset(-1);
                *fresh63 = *a;
                if *b < 0 {
                    p1 = PA.offset(!*b as isize);
                    x |= 1;
                } else {
                    p1 = PA.offset(*b as isize);
                }
            }
        } else if r < 0 {
            if x & 2 != 0 {
                loop {
                    let fresh64 = a;
                    a = a.offset(-1);
                    *fresh64 = *c;
                    let fresh65 = c;
                    c = c.offset(-1);
                    *fresh65 = *a;
                    if *c >= 0 {
                        break;
                    }
                }
                x ^= 2;
            }
            let fresh66 = a;
            a = a.offset(-1);
            *fresh66 = *c;
            let fresh67 = c;
            c = c.offset(-1);
            *fresh67 = *a;
            if c < first {
                while buf < b {
                    let fresh68 = a;
                    a = a.offset(-1);
                    *fresh68 = *b;
                    let fresh69 = b;
                    b = b.offset(-1);
                    *fresh69 = *a;
                }
                *a = *b;
                *b = t;
                break;
            } else if *c < 0 {
                p2 = PA.offset(!*c as isize);
                x |= 2;
            } else {
                p2 = PA.offset(*c as isize);
            }
        } else {
            if x & 1 != 0 {
                loop {
                    let fresh70 = a;
                    a = a.offset(-1);
                    *fresh70 = *b;
                    let fresh71 = b;
                    b = b.offset(-1);
                    *fresh71 = *a;
                    if *b >= 0 {
                        break;
                    }
                }
                x ^= 1;
            }
            let fresh72 = a;
            a = a.offset(-1);
            *fresh72 = !*b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh73 = b;
                b = b.offset(-1);
                *fresh73 = *a;
                if x & 2 != 0 {
                    loop {
                        let fresh74 = a;
                        a = a.offset(-1);
                        *fresh74 = *c;
                        let fresh75 = c;
                        c = c.offset(-1);
                        *fresh75 = *a;
                        if *c >= 0 {
                            break;
                        }
                    }
                    x ^= 2;
                }
                let fresh76 = a;
                a = a.offset(-1);
                *fresh76 = *c;
                let fresh77 = c;
                c = c.offset(-1);
                *fresh77 = *a;
                if c < first {
                    while buf < b {
                        let fresh78 = a;
                        a = a.offset(-1);
                        *fresh78 = *b;
                        let fresh79 = b;
                        b = b.offset(-1);
                        *fresh79 = *a;
                    }
                    *a = *b;
                    *b = t;
                    break;
                } else {
                    if *b < 0 {
                        p1 = PA.offset(!*b as isize);
                        x |= 1;
                    } else {
                        p1 = PA.offset(*b as isize);
                    }
                    if *c < 0 {
                        p2 = PA.offset(!*c as isize);
                        x |= 2;
                    } else {
                        p2 = PA.offset(*c as isize);
                    }
                }
            }
        }
    }
}
unsafe fn ss_swapmerge(
    mut T: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut buf: *mut core::ffi::c_int,
    mut bufsize: core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut stack: [C2RustUnnamed_1; 32] = [C2RustUnnamed_1 {
        a: core::ptr::null_mut::<core::ffi::c_int>(),
        b: core::ptr::null_mut::<core::ffi::c_int>(),
        c: core::ptr::null_mut::<core::ffi::c_int>(),
        d: 0,
    }; 32];
    let mut l = core::ptr::null_mut::<core::ffi::c_int>();
    let mut r = core::ptr::null_mut::<core::ffi::c_int>();
    let mut lm = core::ptr::null_mut::<core::ffi::c_int>();
    let mut rm = core::ptr::null_mut::<core::ffi::c_int>();
    let mut m: core::ffi::c_int = 0;
    let mut len: core::ffi::c_int = 0;
    let mut half: core::ffi::c_int = 0;
    let mut ssize: core::ffi::c_int = 0;
    let mut check: core::ffi::c_int = 0;
    let mut next: core::ffi::c_int = 0;
    check = 0;
    ssize = 0;
    loop {
        if last.offset_from(middle) as core::ffi::c_long <= bufsize as core::ffi::c_long {
            if first < middle && middle < last {
                ss_mergebackward(T, PA, first, middle, last, buf, depth);
            }
            if check & 1 != 0
                || check & 2 != 0
                    && ss_compare(
                        T,
                        PA.offset(
                            (if 0 <= *first.offset(-(1)) {
                                *first.offset(-(1))
                            } else {
                                !*first.offset(-(1))
                            }) as isize,
                        ),
                        PA.offset(*first as isize),
                        depth,
                    ) == 0
            {
                *first = !*first;
            }
            if check & 4 != 0
                && ss_compare(
                    T,
                    PA.offset(
                        (if 0 <= *last.offset(-(1)) {
                            *last.offset(-(1))
                        } else {
                            !*last.offset(-(1))
                        }) as isize,
                    ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0
            {
                *last = !*last;
            }
            if 0 <= ssize {
            } else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    771,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[core::ffi::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ssize == 0 {
                return;
            }
            ssize -= 1;
            first = (*stack.as_mut_ptr().offset(ssize as isize)).a;
            middle = (*stack.as_mut_ptr().offset(ssize as isize)).b;
            last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
            check = (*stack.as_mut_ptr().offset(ssize as isize)).d;
        } else if middle.offset_from(first) as core::ffi::c_long <= bufsize as core::ffi::c_long {
            if first < middle {
                ss_mergeforward(T, PA, first, middle, last, buf, depth);
            }
            if check & 1 != 0
                || check & 2 != 0
                    && ss_compare(
                        T,
                        PA.offset(
                            (if 0 <= *first.offset(-(1)) {
                                *first.offset(-(1))
                            } else {
                                !*first.offset(-(1))
                            }) as isize,
                        ),
                        PA.offset(*first as isize),
                        depth,
                    ) == 0
            {
                *first = !*first;
            }
            if check & 4 != 0
                && ss_compare(
                    T,
                    PA.offset(
                        (if 0 <= *last.offset(-(1)) {
                            *last.offset(-(1))
                        } else {
                            !*last.offset(-(1))
                        }) as isize,
                    ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0
            {
                *last = !*last;
            }
            if 0 <= ssize {
            } else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    780,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[core::ffi::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            if ssize == 0 {
                return;
            }
            ssize -= 1;
            first = (*stack.as_mut_ptr().offset(ssize as isize)).a;
            middle = (*stack.as_mut_ptr().offset(ssize as isize)).b;
            last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
            check = (*stack.as_mut_ptr().offset(ssize as isize)).d;
        } else {
            m = 0;
            len = (if (middle.offset_from(first) as core::ffi::c_long)
                < last.offset_from(middle) as core::ffi::c_long
            {
                middle.offset_from(first) as core::ffi::c_long
            } else {
                last.offset_from(middle) as core::ffi::c_long
            }) as core::ffi::c_int;
            half = len >> 1;
            while (0) < len {
                if ss_compare(
                    T,
                    PA.offset(
                        (if 0 <= *middle.offset(m as isize).offset(half as isize) {
                            *middle.offset(m as isize).offset(half as isize)
                        } else {
                            !*middle.offset(m as isize).offset(half as isize)
                        }) as isize,
                    ),
                    PA.offset(
                        (if 0
                            <= *middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .offset(-(1))
                        {
                            *middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .offset(-(1))
                        } else {
                            !*middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .offset(-(1))
                        }) as isize,
                    ),
                    depth,
                ) < 0
                {
                    m += half + 1;
                    half -= len & 1 ^ 1;
                }
                len = half;
                half >>= 1;
            }
            if (0) < m {
                lm = middle.offset(-(m as isize));
                rm = middle.offset(m as isize);
                ss_blockswap(lm, middle, m);
                r = middle;
                l = r;
                next = 0;
                if rm < last {
                    if *rm < 0 {
                        *rm = !*rm;
                        if first < lm {
                            loop {
                                l = l.offset(-1);
                                if *l >= 0 {
                                    break;
                                }
                            }
                            next |= 4;
                        }
                        next |= 1;
                    } else if first < lm {
                        while *r < 0 {
                            r = r.offset(1);
                        }
                        next |= 2;
                    }
                }
                if l.offset_from(first) as core::ffi::c_long
                    <= last.offset_from(r) as core::ffi::c_long
                {
                    if ssize < 32 {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            810,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[core::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh80 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh80 = r;
                    let fresh81 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh81 = rm;
                    let fresh82 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh82 = last;
                    let fresh83 = ssize;
                    ssize += 1;
                    (*stack.as_mut_ptr().offset(fresh83 as isize)).d = next & 3 | check & 4;
                    middle = lm;
                    last = l;
                    check = check & 3 | next & 4;
                } else {
                    if next & 2 != 0 && r == middle {
                        next ^= 6;
                    }
                    if ssize < 32 {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            814,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[core::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh84 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh84 = first;
                    let fresh85 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh85 = lm;
                    let fresh86 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh86 = l;
                    let fresh87 = ssize;
                    ssize += 1;
                    (*stack.as_mut_ptr().offset(fresh87 as isize)).d = check & 3 | next & 4;
                    first = r;
                    middle = rm;
                    check = next & 3 | check & 4;
                }
            } else {
                if ss_compare(
                    T,
                    PA.offset(
                        (if 0 <= *middle.offset(-(1)) {
                            *middle.offset(-(1))
                        } else {
                            !*middle.offset(-(1))
                        }) as isize,
                    ),
                    PA.offset(*middle as isize),
                    depth,
                ) == 0
                {
                    *middle = !*middle;
                }
                if check & 1 != 0
                    || check & 2 != 0
                        && ss_compare(
                            T,
                            PA.offset(
                                (if 0 <= *first.offset(-(1)) {
                                    *first.offset(-(1))
                                } else {
                                    !*first.offset(-(1))
                                }) as isize,
                            ),
                            PA.offset(*first as isize),
                            depth,
                        ) == 0
                {
                    *first = !*first;
                }
                if check & 4 != 0
                    && ss_compare(
                        T,
                        PA.offset(
                            (if 0 <= *last.offset(-(1)) {
                                *last.offset(-(1))
                            } else {
                                !*last.offset(-(1))
                            }) as isize,
                        ),
                        PA.offset(*last as isize),
                        depth,
                    ) == 0
                {
                    *last = !*last;
                }
                if 0 <= ssize {
                } else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const core::ffi::c_char,
                        822,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[core::ffi::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if ssize == 0 {
                    return;
                }
                ssize -= 1;
                first = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                middle = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                check = (*stack.as_mut_ptr().offset(ssize as isize)).d;
            }
        }
    }
}
unsafe fn sssort(
    mut T: *const core::ffi::c_uchar,
    mut PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut buf: *mut core::ffi::c_int,
    mut bufsize: core::ffi::c_int,
    mut depth: core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut lastsuffix: core::ffi::c_int,
) {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut middle = core::ptr::null_mut::<core::ffi::c_int>();
    let mut curbuf = core::ptr::null_mut::<core::ffi::c_int>();
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut curbufsize: core::ffi::c_int = 0;
    let mut limit: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    if lastsuffix != 0 {
        first = first.offset(1);
    }
    if bufsize < SS_BLOCKSIZE
        && (bufsize as core::ffi::c_long) < last.offset_from(first) as core::ffi::c_long
        && {
            limit = ss_isqrt(last.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
            bufsize < limit
        }
    {
        if SS_BLOCKSIZE < limit {
            limit = SS_BLOCKSIZE;
        }
        middle = last.offset(-(limit as isize));
        buf = middle;
        bufsize = limit;
    } else {
        middle = last;
        limit = 0;
    }
    a = first;
    i = 0;
    while (SS_BLOCKSIZE as core::ffi::c_long) < middle.offset_from(a) as core::ffi::c_long {
        ss_mintrosort(T, PA, a, a.offset(SS_BLOCKSIZE as isize), depth);
        curbufsize = last.offset_from(a.offset(SS_BLOCKSIZE as isize)) as core::ffi::c_long
            as core::ffi::c_int;
        curbuf = a.offset(SS_BLOCKSIZE as isize);
        if curbufsize <= bufsize {
            curbufsize = bufsize;
            curbuf = buf;
        }
        b = a;
        k = SS_BLOCKSIZE;
        j = i;
        while j & 1 != 0 {
            ss_swapmerge(
                T,
                PA,
                b.offset(-(k as isize)),
                b,
                b.offset(k as isize),
                curbuf,
                curbufsize,
                depth,
            );
            b = b.offset(-(k as isize));
            k <<= 1;
            j >>= 1;
        }
        a = a.offset(SS_BLOCKSIZE as isize);
        i += 1;
    }
    ss_mintrosort(T, PA, a, middle, depth);
    k = SS_BLOCKSIZE;
    while i != 0 {
        if i & 1 != 0 {
            ss_swapmerge(
                T,
                PA,
                a.offset(-(k as isize)),
                a,
                middle,
                buf,
                bufsize,
                depth,
            );
            a = a.offset(-(k as isize));
        }
        k <<= 1;
        i >>= 1;
    }
    if limit != 0 {
        ss_mintrosort(T, PA, middle, last, depth);
        ss_inplacemerge(T, PA, first, middle, last, depth);
    }
    if lastsuffix != 0 {
        let mut PAi: [core::ffi::c_int; 2] = [0; 2];
        *PAi.as_mut_ptr().offset(0) = *PA.offset(*first.offset(-(1)) as isize);
        *PAi.as_mut_ptr().offset(1) = n - 2;
        a = first;
        i = *first.offset(-(1));
        while a < last
            && (*a < 0
                || (0)
                    < ss_compare(
                        T,
                        &*PAi.as_mut_ptr().offset(0),
                        PA.offset(*a as isize),
                        depth,
                    ))
        {
            *a.offset(-(1)) = *a;
            a = a.offset(1);
        }
        *a.offset(-(1)) = i;
    }
}
#[inline]
unsafe fn tr_ilg(mut n: core::ffi::c_int) -> core::ffi::c_int {
    if n as core::ffi::c_uint & 0xffff0000 as core::ffi::c_uint != 0 {
        if n as core::ffi::c_uint & 0xff000000 as core::ffi::c_uint != 0 {
            24 + *lg_table
                .as_ptr()
                .offset((n >> 24 & 0xff as core::ffi::c_int) as isize)
        } else {
            16 + *lg_table
                .as_ptr()
                .offset((n >> 16 & 0xff as core::ffi::c_int) as isize)
        }
    } else if n & 0xff00 as core::ffi::c_int != 0 {
        8 + *lg_table
            .as_ptr()
            .offset((n >> 8 & 0xff as core::ffi::c_int) as isize)
    } else {
        *lg_table
            .as_ptr()
            .offset((n & 0xff as core::ffi::c_int) as isize)
    }
}
unsafe fn tr_insertionsort(
    mut ISAd: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
) {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    a = first.offset(1);
    while a < last {
        t = *a;
        b = a.offset(-(1));
        loop {
            r = *ISAd.offset(t as isize) - *ISAd.offset(*b as isize);
            if 0 <= r {
                break;
            }
            loop {
                *b.offset(1) = *b;
                b = b.offset(-1);
                if !(first <= b && *b < 0) {
                    break;
                }
            }
            if b < first {
                break;
            }
        }
        if r == 0 {
            *b = !*b;
        }
        *b.offset(1) = t;
        a = a.offset(1);
    }
}
#[inline]
unsafe fn tr_fixdown(
    mut ISAd: *const core::ffi::c_int,
    mut SA: *mut core::ffi::c_int,
    mut i: core::ffi::c_int,
    mut size: core::ffi::c_int,
) {
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut v: core::ffi::c_int = 0;
    let mut c: core::ffi::c_int = 0;
    let mut d: core::ffi::c_int = 0;
    let mut e: core::ffi::c_int = 0;
    v = *SA.offset(i as isize);
    c = *ISAd.offset(v as isize);
    loop {
        j = 2 * i + 1;
        if j >= size {
            break;
        }
        let fresh88 = j;
        j += 1;
        k = fresh88;
        d = *ISAd.offset(*SA.offset(k as isize) as isize);
        e = *ISAd.offset(*SA.offset(j as isize) as isize);
        if d < e {
            k = j;
            d = e;
        }
        if d <= c {
            break;
        }
        *SA.offset(i as isize) = *SA.offset(k as isize);
        i = k;
    }
    *SA.offset(i as isize) = v;
}
unsafe fn tr_heapsort(
    mut ISAd: *const core::ffi::c_int,
    mut SA: *mut core::ffi::c_int,
    mut size: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut m: core::ffi::c_int = 0;
    let mut t: core::ffi::c_int = 0;
    m = size;
    if size % 2 == 0 {
        m -= 1;
        if *ISAd.offset(*SA.offset((m / 2) as isize) as isize)
            < *ISAd.offset(*SA.offset(m as isize) as isize)
        {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2) as isize);
            *SA.offset((m / 2) as isize) = t;
        }
    }
    i = m / 2 - 1;
    while 0 <= i {
        tr_fixdown(ISAd, SA, i, m);
        i -= 1;
    }
    if size % 2 == 0 {
        t = *SA.offset(0);
        *SA.offset(0) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        tr_fixdown(ISAd, SA, 0, m);
    }
    i = m - 1;
    while (0) < i {
        t = *SA.offset(0);
        *SA.offset(0) = *SA.offset(i as isize);
        tr_fixdown(ISAd, SA, 0, i);
        *SA.offset(i as isize) = t;
        i -= 1;
    }
}
#[inline]
unsafe fn tr_median3(
    mut ISAd: *const core::ffi::c_int,
    mut v1: *mut core::ffi::c_int,
    mut v2: *mut core::ffi::c_int,
    mut v3: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut t = core::ptr::null_mut::<core::ffi::c_int>();
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v2 as isize) {
        t = v1;
        v1 = v2;
        v2 = t;
    }
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v3 as isize) {
        if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v3 as isize) {
            return v1;
        } else {
            return v3;
        }
    }
    v2
}
#[inline]
unsafe fn tr_median5(
    mut ISAd: *const core::ffi::c_int,
    mut v1: *mut core::ffi::c_int,
    mut v2: *mut core::ffi::c_int,
    mut v3: *mut core::ffi::c_int,
    mut v4: *mut core::ffi::c_int,
    mut v5: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut t = core::ptr::null_mut::<core::ffi::c_int>();
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v3 as isize) {
        t = v2;
        v2 = v3;
        v3 = t;
    }
    if *ISAd.offset(*v4 as isize) > *ISAd.offset(*v5 as isize) {
        t = v4;
        v4 = v5;
        v5 = t;
    }
    if *ISAd.offset(*v2 as isize) > *ISAd.offset(*v4 as isize) {
        t = v2;
        v2 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v3 as isize) {
        t = v1;
        v1 = v3;
        v3 = t;
    }
    if *ISAd.offset(*v1 as isize) > *ISAd.offset(*v4 as isize) {
        t = v1;
        v1 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *ISAd.offset(*v3 as isize) > *ISAd.offset(*v4 as isize) {
        return v4;
    }
    v3
}
#[inline]
unsafe fn tr_pivot(
    mut ISAd: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut middle = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    t = last.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
    middle = first.offset((t / 2) as isize);
    if t <= 512 {
        if t <= 32 {
            return tr_median3(ISAd, first, middle, last.offset(-(1)));
        } else {
            t >>= 2;
            return tr_median5(
                ISAd,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1)).offset(-(t as isize)),
                last.offset(-(1)),
            );
        }
    }
    t >>= 3;
    first = tr_median3(
        ISAd,
        first,
        first.offset(t as isize),
        first.offset((t << 1) as isize),
    );
    middle = tr_median3(
        ISAd,
        middle.offset(-(t as isize)),
        middle,
        middle.offset(t as isize),
    );
    last = tr_median3(
        ISAd,
        last.offset(-(1)).offset(-((t << 1) as isize)),
        last.offset(-(1)).offset(-(t as isize)),
        last.offset(-(1)),
    );
    tr_median3(ISAd, first, middle, last)
}
#[inline]
unsafe fn trbudget_init(
    mut budget: *mut trbudget_t,
    mut chance: core::ffi::c_int,
    mut incval: core::ffi::c_int,
) {
    (*budget).chance = chance;
    (*budget).incval = incval;
    (*budget).remain = (*budget).incval;
}
#[inline]
unsafe fn trbudget_check(
    mut budget: *mut trbudget_t,
    mut size: core::ffi::c_int,
) -> core::ffi::c_int {
    if size <= (*budget).remain {
        (*budget).remain -= size;
        return 1;
    }
    if (*budget).chance == 0 {
        (*budget).count += size;
        return 0;
    }
    (*budget).remain += (*budget).incval - size;
    (*budget).chance -= 1;
    1
}
#[inline]
unsafe fn tr_partition(
    mut ISAd: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut pa: *mut *mut core::ffi::c_int,
    mut pb: *mut *mut core::ffi::c_int,
    mut v: core::ffi::c_int,
) {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut d = core::ptr::null_mut::<core::ffi::c_int>();
    let mut e = core::ptr::null_mut::<core::ffi::c_int>();
    let mut f = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut s: core::ffi::c_int = 0;
    let mut x = 0;
    b = middle.offset(-(1));
    loop {
        b = b.offset(1);
        if !(b < last && {
            x = *ISAd.offset(*b as isize);
            x == v
        }) {
            break;
        }
    }
    a = b;
    if a < last && x < v {
        loop {
            b = b.offset(1);
            if !(b < last && {
                x = *ISAd.offset(*b as isize);
                x <= v
            }) {
                break;
            }
            if x == v {
                t = *b;
                *b = *a;
                *a = t;
                a = a.offset(1);
            }
        }
    }
    c = last;
    loop {
        c = c.offset(-1);
        if !(b < c && {
            x = *ISAd.offset(*c as isize);
            x == v
        }) {
            break;
        }
    }
    d = c;
    if b < d && x > v {
        loop {
            c = c.offset(-1);
            if !(b < c && {
                x = *ISAd.offset(*c as isize);
                x >= v
            }) {
                break;
            }
            if x == v {
                t = *c;
                *c = *d;
                *d = t;
                d = d.offset(-1);
            }
        }
    }
    while b < c {
        t = *b;
        *b = *c;
        *c = t;
        loop {
            b = b.offset(1);
            if !(b < c && {
                x = *ISAd.offset(*b as isize);
                x <= v
            }) {
                break;
            }
            if x == v {
                t = *b;
                *b = *a;
                *a = t;
                a = a.offset(1);
            }
        }
        loop {
            c = c.offset(-1);
            if !(b < c && {
                x = *ISAd.offset(*c as isize);
                x >= v
            }) {
                break;
            }
            if x == v {
                t = *c;
                *c = *d;
                *d = t;
                d = d.offset(-1);
            }
        }
    }
    if a <= d {
        c = b.offset(-(1));
        s = a.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
        t = b.offset_from(a) as core::ffi::c_long as core::ffi::c_int;
        if s > t {
            s = t;
        }
        e = first;
        f = b.offset(-(s as isize));
        while (0) < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            e = e.offset(1);
            f = f.offset(1);
        }
        s = d.offset_from(c) as core::ffi::c_long as core::ffi::c_int;
        t = (last.offset_from(d) as core::ffi::c_long - 1) as core::ffi::c_int;
        if s > t {
            s = t;
        }
        e = b;
        f = last.offset(-(s as isize));
        while (0) < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            e = e.offset(1);
            f = f.offset(1);
        }
        first = first.offset(b.offset_from(a) as core::ffi::c_long as isize);
        last = last.offset(-(d.offset_from(c) as core::ffi::c_long as isize));
    }
    *pa = first;
    *pb = last;
}
unsafe fn tr_copy(
    mut ISA: *mut core::ffi::c_int,
    mut SA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut a: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut d = core::ptr::null_mut::<core::ffi::c_int>();
    let mut e = core::ptr::null_mut::<core::ffi::c_int>();
    let mut s: core::ffi::c_int = 0;
    let mut v: core::ffi::c_int = 0;
    v = (b.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
    c = first;
    d = a.offset(-(1));
    while c <= d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
        }
        c = c.offset(1);
    }
    c = last.offset(-(1));
    e = d.offset(1);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
        }
        c = c.offset(-1);
    }
}
unsafe fn tr_partialcopy(
    mut ISA: *mut core::ffi::c_int,
    mut SA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut a: *mut core::ffi::c_int,
    mut b: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut d = core::ptr::null_mut::<core::ffi::c_int>();
    let mut e = core::ptr::null_mut::<core::ffi::c_int>();
    let mut s: core::ffi::c_int = 0;
    let mut v: core::ffi::c_int = 0;
    let mut rank: core::ffi::c_int = 0;
    let mut lastrank: core::ffi::c_int = 0;
    let mut newrank = -(1);
    v = (b.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
    lastrank = -(1);
    c = first;
    d = a.offset(-(1));
    while c <= d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(1);
    }
    lastrank = -(1);
    e = d;
    while first <= e {
        rank = *ISA.offset(*e as isize);
        if lastrank != rank {
            lastrank = rank;
            newrank = e.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
        }
        if newrank != rank {
            *ISA.offset(*e as isize) = newrank;
        }
        e = e.offset(-1);
    }
    lastrank = -(1);
    c = last.offset(-(1));
    e = d.offset(1);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(-1);
    }
}
unsafe fn tr_introsort(
    mut ISA: *mut core::ffi::c_int,
    mut ISAd: *const core::ffi::c_int,
    mut SA: *mut core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut budget: *mut trbudget_t,
) {
    let mut stack: [C2RustUnnamed; 64] = [C2RustUnnamed {
        a: core::ptr::null::<core::ffi::c_int>(),
        b: core::ptr::null_mut::<core::ffi::c_int>(),
        c: core::ptr::null_mut::<core::ffi::c_int>(),
        d: 0,
        e: 0,
    }; 64];
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut v: core::ffi::c_int = 0;
    let mut x = 0;
    let mut incr = ISAd.offset_from(ISA) as core::ffi::c_long as core::ffi::c_int;
    let mut limit: core::ffi::c_int = 0;
    let mut next: core::ffi::c_int = 0;
    let mut ssize: core::ffi::c_int = 0;
    let mut trlink = -(1);
    ssize = 0;
    limit = tr_ilg(last.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
    loop {
        if limit < 0 {
            if limit == -(1) {
                tr_partition(
                    ISAd.offset(-(incr as isize)),
                    first,
                    first,
                    last,
                    &mut a,
                    &mut b,
                    (last.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int,
                );
                if a < last {
                    c = first;
                    v = (a.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                    }
                }
                if b < last {
                    c = a;
                    v = (b.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                    while c < b {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                    }
                }
                if (1) < b.offset_from(a) as core::ffi::c_long {
                    if ssize < 64 {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1204,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[core::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh89 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh89 = core::ptr::null::<core::ffi::c_int>();
                    let fresh90 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh90 = a;
                    let fresh91 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh91 = b;
                    (*stack.as_mut_ptr().offset(ssize as isize)).d = 0;
                    let fresh92 = ssize;
                    ssize += 1;
                    (*stack.as_mut_ptr().offset(fresh92 as isize)).e = 0;
                    if ssize < 64 {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1205,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[core::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh93 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh93 = ISAd.offset(-(incr as isize));
                    let fresh94 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh94 = first;
                    let fresh95 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh95 = last;
                    (*stack.as_mut_ptr().offset(ssize as isize)).d = -(2);
                    let fresh96 = ssize;
                    ssize += 1;
                    (*stack.as_mut_ptr().offset(fresh96 as isize)).e = trlink;
                    trlink = ssize - 2;
                }
                if a.offset_from(first) as core::ffi::c_long
                    <= last.offset_from(b) as core::ffi::c_long
                {
                    if (1) < a.offset_from(first) as core::ffi::c_long {
                        if ssize < 64 {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                1210,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[core::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        let fresh97 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh97 = ISAd;
                        let fresh98 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh98 = b;
                        let fresh99 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                        *fresh99 = last;
                        (*stack.as_mut_ptr().offset(ssize as isize)).d =
                            tr_ilg(last.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                        let fresh100 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh100 as isize)).e = trlink;
                        last = a;
                        limit =
                            tr_ilg(a.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
                    } else if (1) < last.offset_from(b) as core::ffi::c_long {
                        first = b;
                        limit =
                            tr_ilg(last.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                    } else {
                        if 0 <= ssize {
                        } else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const core::ffi::c_char,
                                1215,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[core::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        if ssize == 0 {
                            return;
                        }
                        ssize -= 1;
                        ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                        limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                        trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                    }
                } else if (1) < last.offset_from(b) as core::ffi::c_long {
                    if ssize < 64 {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1219,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[core::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh101 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh101 = ISAd;
                    let fresh102 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh102 = first;
                    let fresh103 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh103 = a;
                    (*stack.as_mut_ptr().offset(ssize as isize)).d =
                        tr_ilg(a.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
                    let fresh104 = ssize;
                    ssize += 1;
                    (*stack.as_mut_ptr().offset(fresh104 as isize)).e = trlink;
                    first = b;
                    limit = tr_ilg(last.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                } else if (1) < a.offset_from(first) as core::ffi::c_long {
                    last = a;
                    limit = tr_ilg(a.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
                } else {
                    if 0 <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1224,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[core::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if ssize == 0 {
                        return;
                    }
                    ssize -= 1;
                    ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                    trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                }
            } else if limit == -(2) {
                ssize -= 1;
                a = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                b = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                if (*stack.as_mut_ptr().offset(ssize as isize)).d == 0 {
                    tr_copy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as core::ffi::c_long as core::ffi::c_int,
                    );
                } else {
                    if 0 <= trlink {
                        (*stack.as_mut_ptr().offset(trlink as isize)).d = -(1);
                    }
                    tr_partialcopy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as core::ffi::c_long as core::ffi::c_int,
                    );
                }
                if 0 <= ssize {
                } else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const core::ffi::c_char,
                        1236,
                        (*::core::mem::transmute::<
                            &[u8; 73],
                            &[core::ffi::c_char; 73],
                        >(
                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if ssize == 0 {
                    return;
                }
                ssize -= 1;
                ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
            } else {
                if 0 <= *first {
                    a = first;
                    loop {
                        *ISA.offset(*a as isize) =
                            a.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                        a = a.offset(1);
                        if !(a < last && 0 <= *a) {
                            break;
                        }
                    }
                    first = a;
                }
                if first < last {
                    a = first;
                    loop {
                        *a = !*a;
                        a = a.offset(1);
                        if *a >= 0 {
                            break;
                        }
                    }
                    next = if *ISA.offset(*a as isize) != *ISAd.offset(*a as isize) {
                        tr_ilg((a.offset_from(first) as core::ffi::c_long + 1) as core::ffi::c_int)
                    } else {
                        -(1)
                    };
                    a = a.offset(1);
                    if a < last {
                        b = first;
                        v = (a.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                        while b < a {
                            *ISA.offset(*b as isize) = v;
                            b = b.offset(1);
                        }
                    }
                    if trbudget_check(
                        budget,
                        a.offset_from(first) as core::ffi::c_long as core::ffi::c_int,
                    ) != 0
                    {
                        if a.offset_from(first) as core::ffi::c_long
                            <= last.offset_from(a) as core::ffi::c_long
                        {
                            if ssize < 64 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    1252,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[core::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh105 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh105 = ISAd;
                            let fresh106 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh106 = a;
                            let fresh107 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh107 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = -(3);
                            let fresh108 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh108 as isize)).e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        } else if (1) < last.offset_from(a) as core::ffi::c_long {
                            if ssize < 64 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    1256,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[core::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh109 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh109 = ISAd.offset(incr as isize);
                            let fresh110 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh110 = first;
                            let fresh111 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh111 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                            let fresh112 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh112 as isize)).e = trlink;
                            first = a;
                            limit = -(3);
                        } else {
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        }
                    } else {
                        if 0 <= trlink {
                            (*stack.as_mut_ptr().offset(trlink as isize)).d = -(1);
                        }
                        if (1) < last.offset_from(a) as core::ffi::c_long {
                            first = a;
                            limit = -(3);
                        } else {
                            if 0 <= ssize {
                            } else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    1267,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[core::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if ssize == 0 {
                                return;
                            }
                            ssize -= 1;
                            ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                            trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                        }
                    }
                } else {
                    if 0 <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1271,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[core::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if ssize == 0 {
                        return;
                    }
                    ssize -= 1;
                    ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                    trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                }
            }
        } else if last.offset_from(first) as core::ffi::c_long
            <= TR_INSERTIONSORT_THRESHOLD as core::ffi::c_long
        {
            tr_insertionsort(ISAd, first, last);
            limit = -(3);
        } else {
            let fresh113 = limit;
            limit -= 1;
            if fresh113 == 0 {
                tr_heapsort(
                    ISAd,
                    first,
                    last.offset_from(first) as core::ffi::c_long as core::ffi::c_int,
                );
                a = last.offset(-(1));
                while first < a {
                    x = *ISAd.offset(*a as isize);
                    b = a.offset(-(1));
                    while first <= b && *ISAd.offset(*b as isize) == x {
                        *b = !*b;
                        b = b.offset(-1);
                    }
                    a = b;
                }
                limit = -(3);
            } else {
                a = tr_pivot(ISAd, first, last);
                t = *first;
                *first = *a;
                *a = t;
                v = *ISAd.offset(*first as isize);
                tr_partition(ISAd, first, first.offset(1), last, &mut a, &mut b, v);
                if last.offset_from(first) as core::ffi::c_long
                    != b.offset_from(a) as core::ffi::c_long
                {
                    next = if *ISA.offset(*a as isize) != v {
                        tr_ilg(b.offset_from(a) as core::ffi::c_long as core::ffi::c_int)
                    } else {
                        -(1)
                    };
                    c = first;
                    v = (a.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                    }
                    if b < last {
                        c = a;
                        v = (b.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                        while c < b {
                            *ISA.offset(*c as isize) = v;
                            c = c.offset(1);
                        }
                    }
                    if (1) < b.offset_from(a) as core::ffi::c_long
                        && trbudget_check(
                            budget,
                            b.offset_from(a) as core::ffi::c_long as core::ffi::c_int,
                        ) != 0
                    {
                        if a.offset_from(first) as core::ffi::c_long
                            <= last.offset_from(b) as core::ffi::c_long
                        {
                            if last.offset_from(b) as core::ffi::c_long
                                <= b.offset_from(a) as core::ffi::c_long
                            {
                                if (1) < a.offset_from(first) as core::ffi::c_long {
                                    if ssize < 64 {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const core::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            1311,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[core::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    let fresh114 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh114 = ISAd.offset(incr as isize);
                                    let fresh115 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh115 = a;
                                    let fresh116 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh116 = b;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                    let fresh117 = ssize;
                                    ssize += 1;
                                    (*stack.as_mut_ptr().offset(fresh117 as isize)).e = trlink;
                                    if ssize < 64 {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const core::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            1312,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[core::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    let fresh118 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh118 = ISAd;
                                    let fresh119 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh119 = b;
                                    let fresh120 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh120 = last;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                    let fresh121 = ssize;
                                    ssize += 1;
                                    (*stack.as_mut_ptr().offset(fresh121 as isize)).e = trlink;
                                    last = a;
                                } else if (1) < last.offset_from(b) as core::ffi::c_long {
                                    if ssize < 64 {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const core::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            1315,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[core::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    let fresh122 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh122 = ISAd.offset(incr as isize);
                                    let fresh123 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh123 = a;
                                    let fresh124 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh124 = b;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                    let fresh125 = ssize;
                                    ssize += 1;
                                    (*stack.as_mut_ptr().offset(fresh125 as isize)).e = trlink;
                                    first = b;
                                } else {
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else if a.offset_from(first) as core::ffi::c_long
                                <= b.offset_from(a) as core::ffi::c_long
                            {
                                if (1) < a.offset_from(first) as core::ffi::c_long {
                                    if ssize < 64 {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const core::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            1322,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[core::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    let fresh126 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh126 = ISAd;
                                    let fresh127 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh127 = b;
                                    let fresh128 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh128 = last;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                    let fresh129 = ssize;
                                    ssize += 1;
                                    (*stack.as_mut_ptr().offset(fresh129 as isize)).e = trlink;
                                    if ssize < 64 {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const core::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            1323,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[core::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    let fresh130 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh130 = ISAd.offset(incr as isize);
                                    let fresh131 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh131 = a;
                                    let fresh132 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh132 = b;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                    let fresh133 = ssize;
                                    ssize += 1;
                                    (*stack.as_mut_ptr().offset(fresh133 as isize)).e = trlink;
                                    last = a;
                                } else {
                                    if ssize < 64 {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const core::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const core::ffi::c_char,
                                            1326,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[core::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    let fresh134 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh134 = ISAd;
                                    let fresh135 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh135 = b;
                                    let fresh136 =
                                        &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh136 = last;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                    let fresh137 = ssize;
                                    ssize += 1;
                                    (*stack.as_mut_ptr().offset(fresh137 as isize)).e = trlink;
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else {
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1330,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh138 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh138 = ISAd;
                                let fresh139 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh139 = b;
                                let fresh140 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh140 = last;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh141 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh141 as isize)).e = trlink;
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1331,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh142 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh142 = ISAd;
                                let fresh143 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh143 = first;
                                let fresh144 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh144 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh145 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh145 as isize)).e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if a.offset_from(first) as core::ffi::c_long
                            <= b.offset_from(a) as core::ffi::c_long
                        {
                            if (1) < last.offset_from(b) as core::ffi::c_long {
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1337,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh146 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh146 = ISAd.offset(incr as isize);
                                let fresh147 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh147 = a;
                                let fresh148 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh148 = b;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                let fresh149 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh149 as isize)).e = trlink;
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1338,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh150 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh150 = ISAd;
                                let fresh151 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh151 = first;
                                let fresh152 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh152 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh153 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh153 as isize)).e = trlink;
                                first = b;
                            } else if (1) < a.offset_from(first) as core::ffi::c_long {
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1341,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh154 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh154 = ISAd.offset(incr as isize);
                                let fresh155 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh155 = a;
                                let fresh156 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh156 = b;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                let fresh157 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh157 as isize)).e = trlink;
                                last = a;
                            } else {
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if last.offset_from(b) as core::ffi::c_long
                            <= b.offset_from(a) as core::ffi::c_long
                        {
                            if (1) < last.offset_from(b) as core::ffi::c_long {
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1348,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh158 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh158 = ISAd;
                                let fresh159 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh159 = first;
                                let fresh160 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh160 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh161 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh161 as isize)).e = trlink;
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1349,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh162 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh162 = ISAd.offset(incr as isize);
                                let fresh163 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh163 = a;
                                let fresh164 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh164 = b;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                let fresh165 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh165 as isize)).e = trlink;
                                first = b;
                            } else {
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1352,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh166 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh166 = ISAd;
                                let fresh167 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh167 = first;
                                let fresh168 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh168 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh169 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh169 as isize)).e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else {
                            if ssize < 64 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    1356,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[core::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh170 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh170 = ISAd;
                            let fresh171 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh171 = first;
                            let fresh172 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh172 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                            let fresh173 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh173 as isize)).e = trlink;
                            if ssize < 64 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    1357,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[core::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh174 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh174 = ISAd;
                            let fresh175 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh175 = b;
                            let fresh176 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh176 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                            let fresh177 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh177 as isize)).e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            first = a;
                            last = b;
                            limit = next;
                        }
                    } else {
                        if (1) < b.offset_from(a) as core::ffi::c_long && 0 <= trlink {
                            (*stack.as_mut_ptr().offset(trlink as isize)).d = -(1);
                        }
                        if a.offset_from(first) as core::ffi::c_long
                            <= last.offset_from(b) as core::ffi::c_long
                        {
                            if (1) < a.offset_from(first) as core::ffi::c_long {
                                if ssize < 64 {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1365,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                let fresh178 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh178 = ISAd;
                                let fresh179 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh179 = b;
                                let fresh180 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh180 = last;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh181 = ssize;
                                ssize += 1;
                                (*stack.as_mut_ptr().offset(fresh181 as isize)).e = trlink;
                                last = a;
                            } else if (1) < last.offset_from(b) as core::ffi::c_long {
                                first = b;
                            } else {
                                if 0 <= ssize {
                                } else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const core::ffi::c_char,
                                        1370,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[core::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                if ssize == 0 {
                                    return;
                                }
                                ssize -= 1;
                                ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                                trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                            }
                        } else if (1) < last.offset_from(b) as core::ffi::c_long {
                            if ssize < 64 {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    1374,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[core::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            let fresh182 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh182 = ISAd;
                            let fresh183 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh183 = first;
                            let fresh184 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh184 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                            let fresh185 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh185 as isize)).e = trlink;
                            first = b;
                        } else if (1) < a.offset_from(first) as core::ffi::c_long {
                            last = a;
                        } else {
                            if 0 <= ssize {
                            } else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const core::ffi::c_char,
                                    1379,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[core::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            if ssize == 0 {
                                return;
                            }
                            ssize -= 1;
                            ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                            trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                        }
                    }
                } else if trbudget_check(
                    budget,
                    last.offset_from(first) as core::ffi::c_long as core::ffi::c_int,
                ) != 0
                {
                    limit =
                        tr_ilg(last.offset_from(first) as core::ffi::c_long as core::ffi::c_int);
                    ISAd = ISAd.offset(incr as isize);
                } else {
                    if 0 <= trlink {
                        (*stack.as_mut_ptr().offset(trlink as isize)).d = -(1);
                    }
                    if 0 <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1388,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[core::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if ssize == 0 {
                        return;
                    }
                    ssize -= 1;
                    ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                    trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                }
            }
        }
    }
}
unsafe fn trsort(
    mut ISA: *mut core::ffi::c_int,
    mut SA: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    let mut ISAd = core::ptr::null_mut::<core::ffi::c_int>();
    let mut first = core::ptr::null_mut::<core::ffi::c_int>();
    let mut last = core::ptr::null_mut::<core::ffi::c_int>();
    let mut budget = _trbudget_t {
        chance: 0,
        remain: 0,
        incval: 0,
        count: 0,
    };
    let mut t: core::ffi::c_int = 0;
    let mut skip: core::ffi::c_int = 0;
    let mut unsorted: core::ffi::c_int = 0;
    trbudget_init(&mut budget, tr_ilg(n) * 2 / 3, n);
    ISAd = ISA.offset(depth as isize);
    while -n < *SA {
        first = SA;
        skip = 0;
        unsorted = 0;
        loop {
            t = *first;
            if t < 0 {
                first = first.offset(-(t as isize));
                skip += t;
            } else {
                if skip != 0 {
                    *first.offset(skip as isize) = skip;
                    skip = 0;
                }
                last = SA.offset(*ISA.offset(t as isize) as isize).offset(1);
                if (1) < last.offset_from(first) as core::ffi::c_long {
                    budget.count = 0;
                    tr_introsort(ISA, ISAd, SA, first, last, &mut budget);
                    if budget.count != 0 {
                        unsorted += budget.count;
                    } else {
                        skip = first.offset_from(last) as core::ffi::c_long as core::ffi::c_int;
                    }
                } else if last.offset_from(first) as core::ffi::c_long == 1 {
                    skip = -(1);
                }
                first = last;
            }
            if first >= SA.offset(n as isize) {
                break;
            }
        }
        if skip != 0 {
            *first.offset(skip as isize) = skip;
        }
        if unsorted == 0 {
            break;
        }
        ISAd = ISAd.offset(ISAd.offset_from(ISA) as core::ffi::c_long as isize);
    }
}
unsafe fn sort_typeBstar(
    mut T: *const core::ffi::c_uchar,
    mut SA: *mut core::ffi::c_int,
    mut bucket_A: *mut core::ffi::c_int,
    mut bucket_B: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut openMP: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut PAb = core::ptr::null_mut::<core::ffi::c_int>();
    let mut ISAb = core::ptr::null_mut::<core::ffi::c_int>();
    let mut buf = core::ptr::null_mut::<core::ffi::c_int>();
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let mut t: core::ffi::c_int = 0;
    let mut m: core::ffi::c_int = 0;
    let mut bufsize: core::ffi::c_int = 0;
    let mut c0: core::ffi::c_int = 0;
    let mut c1: core::ffi::c_int = 0;
    i = 0;
    while i < BUCKET_A_SIZE {
        *bucket_A.offset(i as isize) = 0;
        i += 1;
    }
    i = 0;
    while i < BUCKET_B_SIZE {
        *bucket_B.offset(i as isize) = 0;
        i += 1;
    }
    i = n - 1;
    m = n;
    c0 = *T.offset((n - 1) as isize) as core::ffi::c_int;
    while 0 <= i {
        loop {
            c1 = c0;
            let fresh186 = &mut (*bucket_A.offset(c1 as isize));
            *fresh186 += 1;
            i -= 1;
            if !(0 <= i && {
                c0 = *T.offset(i as isize) as core::ffi::c_int;
                c0 >= c1
            }) {
                break;
            }
        }
        if 0 <= i {
            let fresh187 = &mut (*bucket_B.offset((c0 << 8 | c1) as isize));
            *fresh187 += 1;
            m -= 1;
            *SA.offset(m as isize) = i;
            i -= 1;
            c1 = c0;
            while 0 <= i && {
                c0 = *T.offset(i as isize) as core::ffi::c_int;
                c0 <= c1
            } {
                let fresh188 = &mut (*bucket_B.offset((c1 << 8 | c0) as isize));
                *fresh188 += 1;
                i -= 1;
                c1 = c0;
            }
        }
    }
    m = n - m;
    c0 = 0;
    i = 0;
    j = 0;
    while c0 < ALPHABET_SIZE {
        t = i + *bucket_A.offset(c0 as isize);
        *bucket_A.offset(c0 as isize) = i + j;
        i = t + *bucket_B.offset((c0 << 8 | c0) as isize);
        c1 = c0 + 1;
        while c1 < ALPHABET_SIZE {
            j += *bucket_B.offset((c0 << 8 | c1) as isize);
            *bucket_B.offset((c0 << 8 | c1) as isize) = j;
            i += *bucket_B.offset((c1 << 8 | c0) as isize);
            c1 += 1;
        }
        c0 += 1;
    }
    if (0) < m {
        PAb = SA.offset(n as isize).offset(-(m as isize));
        ISAb = SA.offset(m as isize);
        i = m - 2;
        while 0 <= i {
            t = *PAb.offset(i as isize);
            c0 = *T.offset(t as isize) as core::ffi::c_int;
            c1 = *T.offset((t + 1) as isize) as core::ffi::c_int;
            let fresh189 = &mut (*bucket_B.offset((c0 << 8 | c1) as isize));
            *fresh189 -= 1;
            *SA.offset(*fresh189 as isize) = i;
            i -= 1;
        }
        t = *PAb.offset((m - 1) as isize);
        c0 = *T.offset(t as isize) as core::ffi::c_int;
        c1 = *T.offset((t + 1) as isize) as core::ffi::c_int;
        let fresh190 = &mut (*bucket_B.offset((c0 << 8 | c1) as isize));
        *fresh190 -= 1;
        *SA.offset(*fresh190 as isize) = m - 1;
        buf = SA.offset(m as isize);
        bufsize = n - 2 * m;
        c0 = ALPHABET_SIZE - 2;
        j = m;
        while (0) < j {
            c1 = ALPHABET_SIZE - 1;
            while c0 < c1 {
                i = *bucket_B.offset((c0 << 8 | c1) as isize);
                if (1) < j - i {
                    sssort(
                        T,
                        PAb,
                        SA.offset(i as isize),
                        SA.offset(j as isize),
                        buf,
                        bufsize,
                        2,
                        n,
                        (*SA.offset(i as isize) == m - 1) as core::ffi::c_int,
                    );
                }
                j = i;
                c1 -= 1;
            }
            c0 -= 1;
        }
        i = m - 1;
        while 0 <= i {
            if 0 <= *SA.offset(i as isize) {
                j = i;
                loop {
                    *ISAb.offset(*SA.offset(i as isize) as isize) = i;
                    i -= 1;
                    if !(0 <= i && 0 <= *SA.offset(i as isize)) {
                        break;
                    }
                }
                *SA.offset((i + 1) as isize) = i - j;
                if i <= 0 {
                    break;
                }
            }
            j = i;
            loop {
                let fresh191 = &mut (*SA.offset(i as isize));
                *fresh191 = !*SA.offset(i as isize);
                *ISAb.offset(*fresh191 as isize) = j;
                i -= 1;
                if *SA.offset(i as isize) >= 0 {
                    break;
                }
            }
            *ISAb.offset(*SA.offset(i as isize) as isize) = j;
            i -= 1;
        }
        trsort(ISAb, SA, m, 1);
        i = n - 1;
        j = m;
        c0 = *T.offset((n - 1) as isize) as core::ffi::c_int;
        while 0 <= i {
            i -= 1;
            c1 = c0;
            while 0 <= i && {
                c0 = *T.offset(i as isize) as core::ffi::c_int;
                c0 >= c1
            } {
                i -= 1;
                c1 = c0;
            }
            if 0 <= i {
                t = i;
                i -= 1;
                c1 = c0;
                while 0 <= i && {
                    c0 = *T.offset(i as isize) as core::ffi::c_int;
                    c0 <= c1
                } {
                    i -= 1;
                    c1 = c0;
                }
                j -= 1;
                *SA.offset(*ISAb.offset(j as isize) as isize) =
                    if t == 0 || (1) < t - i { t } else { !t };
            }
        }
        *bucket_B.offset((((256 - 1) << 8) | (256 - 1)) as isize) = n;
        c0 = ALPHABET_SIZE - 2;
        k = m - 1;
        while 0 <= c0 {
            i = *bucket_A.offset((c0 + 1) as isize) - 1;
            c1 = ALPHABET_SIZE - 1;
            while c0 < c1 {
                t = i - *bucket_B.offset((c1 << 8 | c0) as isize);
                *bucket_B.offset((c1 << 8 | c0) as isize) = i;
                i = t;
                j = *bucket_B.offset((c0 << 8 | c1) as isize);
                while j <= k {
                    *SA.offset(i as isize) = *SA.offset(k as isize);
                    i -= 1;
                    k -= 1;
                }
                c1 -= 1;
            }
            *bucket_B.offset(((c0 << 8) | (c0 + 1)) as isize) =
                i - *bucket_B.offset((c0 << 8 | c0) as isize) + 1;
            *bucket_B.offset((c0 << 8 | c0) as isize) = i;
            c0 -= 1;
        }
    }
    m
}
unsafe fn construct_SA(
    mut T: *const core::ffi::c_uchar,
    mut SA: *mut core::ffi::c_int,
    mut bucket_A: *mut core::ffi::c_int,
    mut bucket_B: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut m: core::ffi::c_int,
) {
    let mut i = core::ptr::null_mut::<core::ffi::c_int>();
    let mut j = core::ptr::null_mut::<core::ffi::c_int>();
    let mut k = core::ptr::null_mut::<core::ffi::c_int>();
    let mut s: core::ffi::c_int = 0;
    let mut c0: core::ffi::c_int = 0;
    let mut c1: core::ffi::c_int = 0;
    let mut c2: core::ffi::c_int = 0;
    if (0) < m {
        c1 = ALPHABET_SIZE - 2;
        while 0 <= c1 {
            i = SA.offset(*bucket_B.offset(((c1 << 8) | (c1 + 1)) as isize) as isize);
            j = SA
                .offset(*bucket_A.offset((c1 + 1) as isize) as isize)
                .offset(-(1));
            k = NULL as *mut core::ffi::c_int;
            c2 = -(1);
            while i <= j {
                s = *j;
                if (0) < s {
                    assert_eq!(*T.offset(s as isize) as core::ffi::c_int, c1);
                    if (s + 1) < n
                        && *T.offset(s as isize) as core::ffi::c_int
                            <= *T.offset((s + 1) as isize) as core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1631,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if *T.offset((s - 1) as isize) as core::ffi::c_int
                        <= *T.offset(s as isize) as core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1632,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    *j = !s;
                    s -= 1;
                    c0 = *T.offset(s as isize) as core::ffi::c_int;
                    if (0) < s && *T.offset((s - 1) as isize) as core::ffi::c_int > c0 {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 <= c2 {
                            *bucket_B.offset((c1 << 8 | c2) as isize) =
                                k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                        }
                        c2 = c0;
                        k = SA.offset(*bucket_B.offset((c1 << 8 | c2) as isize) as isize);
                    }
                    if k < j {
                    } else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1640,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if !k.is_null() {
                    } else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1640,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh192 = k;
                    k = k.offset(-1);
                    *fresh192 = s;
                } else {
                    assert!(s == 0 && *T.offset(s as isize) as core::ffi::c_int == c1 || s < 0);
                    *j = !s;
                }
                j = j.offset(-1);
            }
            c1 -= 1;
        }
    }
    c2 = *T.offset((n - 1) as isize) as core::ffi::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh193 = k;
    k = k.offset(1);
    *fresh193 = if (*T.offset((n - 2) as isize) as core::ffi::c_int) < c2 {
        !(n - 1)
    } else {
        n - 1
    };
    i = SA;
    j = SA.offset(n as isize);
    while i < j {
        s = *i;
        if (0) < s {
            if *T.offset((s - 1) as isize) as core::ffi::c_int
                >= *T.offset(s as isize) as core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1657,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[core::ffi::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            s -= 1;
            c0 = *T.offset(s as isize) as core::ffi::c_int;
            if s == 0 || (*T.offset((s - 1) as isize) as core::ffi::c_int) < c0 {
                s = !s;
            }
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {
            } else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1664,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[core::ffi::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            let fresh194 = k;
            k = k.offset(1);
            *fresh194 = s;
        } else {
            if s < 0 {
            } else {
                __assert_fail(
                    b"s < 0\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1667,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[core::ffi::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            *i = !s;
        }
        i = i.offset(1);
    }
}
unsafe fn construct_BWT(
    mut T: *const core::ffi::c_uchar,
    mut SA: *mut core::ffi::c_int,
    mut bucket_A: *mut core::ffi::c_int,
    mut bucket_B: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut m: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i = core::ptr::null_mut::<core::ffi::c_int>();
    let mut j = core::ptr::null_mut::<core::ffi::c_int>();
    let mut k = core::ptr::null_mut::<core::ffi::c_int>();
    let mut orig = core::ptr::null_mut::<core::ffi::c_int>();
    let mut s: core::ffi::c_int = 0;
    let mut c0: core::ffi::c_int = 0;
    let mut c1: core::ffi::c_int = 0;
    let mut c2: core::ffi::c_int = 0;
    if (0) < m {
        c1 = ALPHABET_SIZE - 2;
        while 0 <= c1 {
            i = SA.offset(*bucket_B.offset(((c1 << 8) | (c1 + 1)) as isize) as isize);
            j = SA
                .offset(*bucket_A.offset((c1 + 1) as isize) as isize)
                .offset(-(1));
            k = NULL as *mut core::ffi::c_int;
            c2 = -(1);
            while i <= j {
                s = *j;
                if (0) < s {
                    assert_eq!(*T.offset(s as isize) as core::ffi::c_int, c1);
                    if (s + 1) < n
                        && *T.offset(s as isize) as core::ffi::c_int
                            <= *T.offset((s + 1) as isize) as core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1695,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if *T.offset((s - 1) as isize) as core::ffi::c_int
                        <= *T.offset(s as isize) as core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1696,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    s -= 1;
                    c0 = *T.offset(s as isize) as core::ffi::c_int;
                    *j = !c0;
                    if (0) < s && *T.offset((s - 1) as isize) as core::ffi::c_int > c0 {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 <= c2 {
                            *bucket_B.offset((c1 << 8 | c2) as isize) =
                                k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                        }
                        c2 = c0;
                        k = SA.offset(*bucket_B.offset((c1 << 8 | c2) as isize) as isize);
                    }
                    if k < j {
                    } else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1704,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if !k.is_null() {
                    } else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1704,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[core::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh195 = k;
                    k = k.offset(-1);
                    *fresh195 = s;
                } else if s != 0 {
                    *j = !s;
                } else {
                    assert_eq!(*T.offset(s as isize) as core::ffi::c_int, c1);
                }
                j = j.offset(-1);
            }
            c1 -= 1;
        }
    }
    c2 = *T.offset((n - 1) as isize) as core::ffi::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh196 = k;
    k = k.offset(1);
    *fresh196 = if (*T.offset((n - 2) as isize) as core::ffi::c_int) < c2 {
        !(*T.offset((n - 2) as isize) as core::ffi::c_int)
    } else {
        n - 1
    };
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0) < s {
            if *T.offset((s - 1) as isize) as core::ffi::c_int
                >= *T.offset(s as isize) as core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1724,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[core::ffi::c_char; 72],
                    >(
                        b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            s -= 1;
            c0 = *T.offset(s as isize) as core::ffi::c_int;
            *i = c0;
            if (0) < s && (*T.offset((s - 1) as isize) as core::ffi::c_int) < c0 {
                s = !(*T.offset((s - 1) as isize) as core::ffi::c_int);
            }
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {
            } else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1732,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[core::ffi::c_char; 72],
                    >(
                        b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            let fresh197 = k;
            k = k.offset(1);
            *fresh197 = s;
        } else if s != 0 {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
    }
    orig.offset_from(SA) as core::ffi::c_long as core::ffi::c_int
}
unsafe fn construct_BWT_indexes(
    mut T: *const core::ffi::c_uchar,
    mut SA: *mut core::ffi::c_int,
    mut bucket_A: *mut core::ffi::c_int,
    mut bucket_B: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut m: core::ffi::c_int,
    mut num_indexes: *mut core::ffi::c_uchar,
    mut indexes: *mut core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i = core::ptr::null_mut::<core::ffi::c_int>();
    let mut j = core::ptr::null_mut::<core::ffi::c_int>();
    let mut k = core::ptr::null_mut::<core::ffi::c_int>();
    let mut orig = core::ptr::null_mut::<core::ffi::c_int>();
    let mut s: core::ffi::c_int = 0;
    let mut c0: core::ffi::c_int = 0;
    let mut c1: core::ffi::c_int = 0;
    let mut c2: core::ffi::c_int = 0;
    let mut mod_0 = n / 8;
    mod_0 |= mod_0 >> 1;
    mod_0 |= mod_0 >> 2;
    mod_0 |= mod_0 >> 4;
    mod_0 |= mod_0 >> 8;
    mod_0 |= mod_0 >> 16;
    mod_0 >>= 1;
    *num_indexes = ((n - 1) / (mod_0 + 1)) as core::ffi::c_uchar;
    if (0) < m {
        c1 = ALPHABET_SIZE - 2;
        while 0 <= c1 {
            i = SA.offset(*bucket_B.offset(((c1 << 8) | (c1 + 1)) as isize) as isize);
            j = SA
                .offset(*bucket_A.offset((c1 + 1) as isize) as isize)
                .offset(-(1));
            k = NULL as *mut core::ffi::c_int;
            c2 = -(1);
            while i <= j {
                s = *j;
                if (0) < s {
                    assert_eq!(*T.offset(s as isize) as core::ffi::c_int, c1);
                    if (s + 1) < n
                        && *T.offset(s as isize) as core::ffi::c_int
                            <= *T.offset((s + 1) as isize) as core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1776,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[core::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if *T.offset((s - 1) as isize) as core::ffi::c_int
                        <= *T.offset(s as isize) as core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8
                                as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1777,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[core::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if s & mod_0 == 0 {
                        *indexes.offset((s / (mod_0 + 1) - 1) as isize) =
                            j.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                    }
                    s -= 1;
                    c0 = *T.offset(s as isize) as core::ffi::c_int;
                    *j = !c0;
                    if (0) < s && *T.offset((s - 1) as isize) as core::ffi::c_int > c0 {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 <= c2 {
                            *bucket_B.offset((c1 << 8 | c2) as isize) =
                                k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                        }
                        c2 = c0;
                        k = SA.offset(*bucket_B.offset((c1 << 8 | c2) as isize) as isize);
                    }
                    if k < j {
                    } else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1788,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[core::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    if !k.is_null() {
                    } else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const core::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const core::ffi::c_char,
                            1788,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[core::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    let fresh198 = k;
                    k = k.offset(-1);
                    *fresh198 = s;
                } else if s != 0 {
                    *j = !s;
                } else {
                    assert_eq!(*T.offset(s as isize) as core::ffi::c_int, c1);
                }
                j = j.offset(-1);
            }
            c1 -= 1;
        }
    }
    c2 = *T.offset((n - 1) as isize) as core::ffi::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    if (*T.offset((n - 2) as isize) as core::ffi::c_int) < c2 {
        if (n - 1) & mod_0 == 0 {
            *indexes.offset(((n - 1) / (mod_0 + 1) - 1) as isize) =
                k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
        }
        let fresh199 = k;
        k = k.offset(1);
        *fresh199 = !(*T.offset((n - 2) as isize) as core::ffi::c_int);
    } else {
        let fresh200 = k;
        k = k.offset(1);
        *fresh200 = n - 1;
    }
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0) < s {
            if *T.offset((s - 1) as isize) as core::ffi::c_int
                >= *T.offset(s as isize) as core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1815,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[core::ffi::c_char; 104],
                    >(
                        b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if s & mod_0 == 0 {
                *indexes.offset((s / (mod_0 + 1) - 1) as isize) =
                    i.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
            }
            s -= 1;
            c0 = *T.offset(s as isize) as core::ffi::c_int;
            *i = c0;
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {
            } else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const core::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const core::ffi::c_char,
                    1825,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[core::ffi::c_char; 104],
                    >(
                        b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            if (0) < s && (*T.offset((s - 1) as isize) as core::ffi::c_int) < c0 {
                if s & mod_0 == 0 {
                    *indexes.offset((s / (mod_0 + 1) - 1) as isize) =
                        k.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
                }
                let fresh201 = k;
                k = k.offset(1);
                *fresh201 = !(*T.offset((s - 1) as isize) as core::ffi::c_int);
            } else {
                let fresh202 = k;
                k = k.offset(1);
                *fresh202 = s;
            }
        } else if s != 0 {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
    }
    orig.offset_from(SA) as core::ffi::c_long as core::ffi::c_int
}
pub unsafe fn divsufsort(
    mut T: *const core::ffi::c_uchar,
    mut SA: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut openMP: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut bucket_A = core::ptr::null_mut::<core::ffi::c_int>();
    let mut bucket_B = core::ptr::null_mut::<core::ffi::c_int>();
    let mut m: core::ffi::c_int = 0;
    let mut err = 0;
    if T.is_null() || SA.is_null() || n < 0 {
        return -(1);
    } else if n == 0 {
        return 0;
    } else if n == 1 {
        *SA.offset(0) = 0;
        return 0;
    } else if n == 2 {
        m = ((*T.offset(0) as core::ffi::c_int) < *T.offset(1) as core::ffi::c_int)
            as core::ffi::c_int;
        *SA.offset((m ^ 1) as isize) = 0;
        *SA.offset(m as isize) = 1;
        return 0;
    }
    bucket_A =
        malloc((BUCKET_A_SIZE as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_int>()))
            as *mut core::ffi::c_int;
    bucket_B =
        malloc((BUCKET_B_SIZE as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_int>()))
            as *mut core::ffi::c_int;
    if !bucket_A.is_null() && !bucket_B.is_null() {
        m = sort_typeBstar(T, SA, bucket_A, bucket_B, n, openMP);
        construct_SA(T, SA, bucket_A, bucket_B, n, m);
    } else {
        err = -(2);
    }
    free(bucket_B as *mut core::ffi::c_void);
    free(bucket_A as *mut core::ffi::c_void);
    err
}
pub unsafe fn divbwt(
    mut T: *const core::ffi::c_uchar,
    mut U: *mut core::ffi::c_uchar,
    mut A: *mut core::ffi::c_int,
    mut n: core::ffi::c_int,
    mut num_indexes: *mut core::ffi::c_uchar,
    mut indexes: *mut core::ffi::c_int,
    mut openMP: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut B = core::ptr::null_mut::<core::ffi::c_int>();
    let mut bucket_A = core::ptr::null_mut::<core::ffi::c_int>();
    let mut bucket_B = core::ptr::null_mut::<core::ffi::c_int>();
    let mut m: core::ffi::c_int = 0;
    let mut pidx: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;
    if T.is_null() || U.is_null() || n < 0 {
        return -(1);
    } else if n <= 1 {
        if n == 1 {
            *U.offset(0) = *T.offset(0);
        }
        return n;
    }
    B = A;
    if B.is_null() {
        B = malloc(((n + 1) as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_int>()))
            as *mut core::ffi::c_int;
    }
    bucket_A =
        malloc((BUCKET_A_SIZE as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_int>()))
            as *mut core::ffi::c_int;
    bucket_B =
        malloc((BUCKET_B_SIZE as size_t).wrapping_mul(::core::mem::size_of::<core::ffi::c_int>()))
            as *mut core::ffi::c_int;
    if !B.is_null() && !bucket_A.is_null() && !bucket_B.is_null() {
        m = sort_typeBstar(T, B, bucket_A, bucket_B, n, openMP);
        if num_indexes.is_null() || indexes.is_null() {
            pidx = construct_BWT(T, B, bucket_A, bucket_B, n, m);
        } else {
            pidx = construct_BWT_indexes(T, B, bucket_A, bucket_B, n, m, num_indexes, indexes);
        }
        *U.offset(0) = *T.offset((n - 1) as isize);
        i = 0;
        while i < pidx {
            *U.offset((i + 1) as isize) = *B.offset(i as isize) as core::ffi::c_uchar;
            i += 1;
        }
        i += 1;
        while i < n {
            *U.offset(i as isize) = *B.offset(i as isize) as core::ffi::c_uchar;
            i += 1;
        }
        pidx += 1;
    } else {
        pidx = -(2);
    }
    free(bucket_B as *mut core::ffi::c_void);
    free(bucket_A as *mut core::ffi::c_void);
    if A.is_null() {
        free(B as *mut core::ffi::c_void);
    }
    pidx
}
