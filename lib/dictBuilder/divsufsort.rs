use libc::{free, malloc, size_t};

type trbudget_t = _trbudget_t;
#[repr(C)]
struct _trbudget_t {
    chance: core::ffi::c_int,
    remain: core::ffi::c_int,
    incval: core::ffi::c_int,
    count: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed {
    a: *const core::ffi::c_int,
    b: *mut core::ffi::c_int,
    c: *mut core::ffi::c_int,
    d: core::ffi::c_int,
    e: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_0 {
    a: *mut core::ffi::c_int,
    b: *mut core::ffi::c_int,
    c: core::ffi::c_int,
    d: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct C2RustUnnamed_1 {
    a: *mut core::ffi::c_int,
    b: *mut core::ffi::c_int,
    c: *mut core::ffi::c_int,
    d: core::ffi::c_int,
}
const ALPHABET_SIZE: core::ffi::c_int = 256;
const BUCKET_A_SIZE: core::ffi::c_int = 256;
const BUCKET_B_SIZE: core::ffi::c_int = ALPHABET_SIZE * ALPHABET_SIZE;
const SS_INSERTIONSORT_THRESHOLD: core::ffi::c_int = 8;
const SS_BLOCKSIZE: core::ffi::c_int = 1024;
const TR_INSERTIONSORT_THRESHOLD: core::ffi::c_int = 8;
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
unsafe fn ss_ilg(n: core::ffi::c_int) -> core::ffi::c_int {
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
unsafe fn ss_isqrt(x: core::ffi::c_int) -> core::ffi::c_int {
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
    T: *const core::ffi::c_uchar,
    p1: *const core::ffi::c_int,
    p2: *const core::ffi::c_int,
    depth: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut U1 = core::ptr::null::<core::ffi::c_uchar>();
    let mut U2 = core::ptr::null::<core::ffi::c_uchar>();
    let mut U1n = core::ptr::null::<core::ffi::c_uchar>();
    let mut U2n = core::ptr::null::<core::ffi::c_uchar>();
    U1 = T.offset(depth as isize).offset(*p1 as isize);
    U2 = T.offset(depth as isize).offset(*p2 as isize);
    U1n = T.offset(*p1.add(1) as isize).add(2);
    U2n = T.offset(*p2.add(1) as isize).add(2);
    while U1 < U1n && U2 < U2n && *U1 as core::ffi::c_int == *U2 as core::ffi::c_int {
        U1 = U1.add(1);
        U2 = U2.add(1);
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
    T: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
    depth: core::ffi::c_int,
) {
    let mut i = core::ptr::null_mut::<core::ffi::c_int>();
    let mut j = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    i = last.sub(2);
    while first <= i {
        t = *i;
        j = i.add(1);
        loop {
            r = ss_compare(T, PA.offset(t as isize), PA.offset(*j as isize), depth);
            if (0) >= r {
                break;
            }
            loop {
                *j.sub(1) = *j;
                j = j.add(1);
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
        *j.sub(1) = t;
        i = i.sub(1);
    }
}
#[inline]
unsafe fn ss_fixdown(
    Td: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    SA: *mut core::ffi::c_int,
    mut i: core::ffi::c_int,
    size: core::ffi::c_int,
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
    Td: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    SA: *mut core::ffi::c_int,
    size: core::ffi::c_int,
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
        t = *SA;
        *SA = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        ss_fixdown(Td, PA, SA, 0, m);
    }
    i = m - 1;
    while (0) < i {
        t = *SA;
        *SA = *SA.offset(i as isize);
        ss_fixdown(Td, PA, SA, 0, i);
        *SA.offset(i as isize) = t;
        i -= 1;
    }
}
#[inline]
unsafe fn ss_median3(
    Td: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    mut v1: *mut core::ffi::c_int,
    mut v2: *mut core::ffi::c_int,
    v3: *mut core::ffi::c_int,
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
    Td: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
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
    Td: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut middle = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    t = last.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
    middle = first.offset((t / 2) as isize);
    if t <= 512 {
        if t <= 32 {
            return ss_median3(Td, PA, first, middle, last.sub(1));
        } else {
            t >>= 2;
            return ss_median5(
                Td,
                PA,
                first,
                first.offset(t as isize),
                middle,
                last.sub(1).offset(-(t as isize)),
                last.sub(1),
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
        last.sub(1).offset(-((t << 1) as isize)),
        last.sub(1).offset(-(t as isize)),
        last.sub(1),
    );
    ss_median3(Td, PA, first, middle, last)
}
#[inline]
unsafe fn ss_partition(
    PA: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
    depth: core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    a = first.sub(1);
    b = last;
    loop {
        loop {
            a = a.add(1);
            if !(a < b && *PA.offset(*a as isize) + depth >= *PA.offset((*a + 1) as isize) + 1) {
                break;
            }
            *a = !*a;
        }
        loop {
            b = b.sub(1);
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
    T: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    mut depth: core::ffi::c_int,
) {
    const STACK_SIZE: core::ffi::c_int = 16;

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
            assert!(0 <= ssize);
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
                a = first.add(1);
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
                    a = a.add(1);
                }
                if (*Td.offset((*PA.offset(*first as isize) - 1) as isize) as core::ffi::c_int) < v
                {
                    first = ss_partition(PA, first, a, depth);
                }
                if a.offset_from(first) as core::ffi::c_long
                    <= last.offset_from(a) as core::ffi::c_long
                {
                    if (1) < a.offset_from(first) as core::ffi::c_long {
                        assert!(ssize < STACK_SIZE);
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
                    assert!(ssize < STACK_SIZE);
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
                    b = b.add(1);
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
                        b = b.add(1);
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
                            a = a.add(1);
                        }
                    }
                }
                c = last;
                loop {
                    c = c.sub(1);
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
                        c = c.sub(1);
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
                            d = d.sub(1);
                        }
                    }
                }
                while b < c {
                    t = *b;
                    *b = *c;
                    *c = t;
                    loop {
                        b = b.add(1);
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
                            a = a.add(1);
                        }
                    }
                    loop {
                        c = c.sub(1);
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
                            d = d.sub(1);
                        }
                    }
                }
                if a <= d {
                    c = b.sub(1);
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
                        e = e.add(1);
                        f = f.add(1);
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
                        e = e.add(1);
                        f = f.add(1);
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
                            assert!(ssize < STACK_SIZE);
                            let fresh8 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh8 = b;
                            let fresh9 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh9 = c;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1;
                            let fresh10 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh10 as isize)).d =
                                ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                            assert!(ssize < STACK_SIZE);
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
                            assert!(ssize < STACK_SIZE);
                            let fresh14 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh14 = c;
                            let fresh15 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh15 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh16 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh16 as isize)).d = limit;
                            assert!(ssize < STACK_SIZE);
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
                            assert!(ssize < STACK_SIZE);
                            let fresh20 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh20 = c;
                            let fresh21 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh21 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh22 = ssize;
                            ssize += 1;
                            (*stack.as_mut_ptr().offset(fresh22 as isize)).d = limit;
                            assert!(ssize < STACK_SIZE);
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
                        assert!(ssize < STACK_SIZE);
                        let fresh26 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh26 = b;
                        let fresh27 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh27 = c;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1;
                        let fresh28 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh28 as isize)).d =
                            ss_ilg(c.offset_from(b) as core::ffi::c_long as core::ffi::c_int);
                        assert!(ssize < STACK_SIZE);
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
                        assert!(ssize < STACK_SIZE);
                        let fresh32 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh32 = first;
                        let fresh33 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh33 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh34 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh34 as isize)).d = limit;
                        assert!(ssize < STACK_SIZE);
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
                        assert!(ssize < STACK_SIZE);
                        let fresh38 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh38 = first;
                        let fresh39 = &mut (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh39 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh40 = ssize;
                        ssize += 1;
                        (*stack.as_mut_ptr().offset(fresh40 as isize)).d = limit;
                        assert!(ssize < STACK_SIZE);
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
        a = a.add(1);
        b = b.add(1);
    }
}
#[inline]
unsafe fn ss_rotate(
    mut first: *mut core::ffi::c_int,
    middle: *mut core::ffi::c_int,
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
            a = last.sub(1);
            b = middle.sub(1);
            t = *a;
            loop {
                let fresh44 = a;
                a = a.sub(1);
                *fresh44 = *b;
                let fresh45 = b;
                b = b.sub(1);
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
                a = a.sub(1);
                b = middle.sub(1);
                t = *a;
            }
        } else {
            a = first;
            b = middle;
            t = *a;
            loop {
                let fresh46 = a;
                a = a.add(1);
                *fresh46 = *b;
                let fresh47 = b;
                b = b.add(1);
                *fresh47 = *a;
                if last > b {
                    continue;
                }
                *a = t;
                first = a.add(1);
                l -= r + 1;
                if l <= r {
                    break;
                }
                a = a.add(1);
                b = middle;
                t = *a;
            }
        }
    }
}
unsafe fn ss_inplacemerge(
    T: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    depth: core::ffi::c_int,
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
        if *last.sub(1) < 0 {
            x = 1;
            p = PA.offset(!*last.sub(1) as isize);
        } else {
            x = 0;
            p = PA.offset(*last.sub(1) as isize);
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
                a = b.add(1);
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
        last = last.sub(1);
        if x != 0 {
            loop {
                last = last.sub(1);
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
    T: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    middle: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
    buf: *mut core::ffi::c_int,
    depth: core::ffi::c_int,
) {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut bufend = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    bufend = buf
        .offset(middle.offset_from(first) as core::ffi::c_long as isize)
        .sub(1);
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
                a = a.add(1);
                *fresh48 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh49 = b;
                b = b.add(1);
                *fresh49 = *a;
                if *b >= 0 {
                    break;
                }
            }
        } else if r > 0 {
            loop {
                let fresh50 = a;
                a = a.add(1);
                *fresh50 = *c;
                let fresh51 = c;
                c = c.add(1);
                *fresh51 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh52 = a;
                        a = a.add(1);
                        *fresh52 = *b;
                        let fresh53 = b;
                        b = b.add(1);
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
                a = a.add(1);
                *fresh54 = *b;
                if bufend <= b {
                    *bufend = t;
                    return;
                }
                let fresh55 = b;
                b = b.add(1);
                *fresh55 = *a;
                if *b >= 0 {
                    break;
                }
            }
            loop {
                let fresh56 = a;
                a = a.add(1);
                *fresh56 = *c;
                let fresh57 = c;
                c = c.add(1);
                *fresh57 = *a;
                if last <= c {
                    while b < bufend {
                        let fresh58 = a;
                        a = a.add(1);
                        *fresh58 = *b;
                        let fresh59 = b;
                        b = b.add(1);
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
    T: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    middle: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
    buf: *mut core::ffi::c_int,
    depth: core::ffi::c_int,
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
        .sub(1);
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
    if *middle.sub(1) < 0 {
        p2 = PA.offset(!*middle.sub(1) as isize);
        x |= 2;
    } else {
        p2 = PA.offset(*middle.sub(1) as isize);
    }
    a = last.sub(1);
    t = *a;
    b = bufend;
    c = middle.sub(1);
    loop {
        r = ss_compare(T, p1, p2, depth);
        if (0) < r {
            if x & 1 != 0 {
                loop {
                    let fresh60 = a;
                    a = a.sub(1);
                    *fresh60 = *b;
                    let fresh61 = b;
                    b = b.sub(1);
                    *fresh61 = *a;
                    if *b >= 0 {
                        break;
                    }
                }
                x ^= 1;
            }
            let fresh62 = a;
            a = a.sub(1);
            *fresh62 = *b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh63 = b;
                b = b.sub(1);
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
                    a = a.sub(1);
                    *fresh64 = *c;
                    let fresh65 = c;
                    c = c.sub(1);
                    *fresh65 = *a;
                    if *c >= 0 {
                        break;
                    }
                }
                x ^= 2;
            }
            let fresh66 = a;
            a = a.sub(1);
            *fresh66 = *c;
            let fresh67 = c;
            c = c.sub(1);
            *fresh67 = *a;
            if c < first {
                while buf < b {
                    let fresh68 = a;
                    a = a.sub(1);
                    *fresh68 = *b;
                    let fresh69 = b;
                    b = b.sub(1);
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
                    a = a.sub(1);
                    *fresh70 = *b;
                    let fresh71 = b;
                    b = b.sub(1);
                    *fresh71 = *a;
                    if *b >= 0 {
                        break;
                    }
                }
                x ^= 1;
            }
            let fresh72 = a;
            a = a.sub(1);
            *fresh72 = !*b;
            if b <= buf {
                *buf = t;
                break;
            } else {
                let fresh73 = b;
                b = b.sub(1);
                *fresh73 = *a;
                if x & 2 != 0 {
                    loop {
                        let fresh74 = a;
                        a = a.sub(1);
                        *fresh74 = *c;
                        let fresh75 = c;
                        c = c.sub(1);
                        *fresh75 = *a;
                        if *c >= 0 {
                            break;
                        }
                    }
                    x ^= 2;
                }
                let fresh76 = a;
                a = a.sub(1);
                *fresh76 = *c;
                let fresh77 = c;
                c = c.sub(1);
                *fresh77 = *a;
                if c < first {
                    while buf < b {
                        let fresh78 = a;
                        a = a.sub(1);
                        *fresh78 = *b;
                        let fresh79 = b;
                        b = b.sub(1);
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
    T: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    buf: *mut core::ffi::c_int,
    bufsize: core::ffi::c_int,
    depth: core::ffi::c_int,
) {
    const STACK_SIZE: core::ffi::c_int = 32;

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
                            (if 0 <= *first.sub(1) {
                                *first.sub(1)
                            } else {
                                !*first.sub(1)
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
                        (if 0 <= *last.sub(1) {
                            *last.sub(1)
                        } else {
                            !*last.sub(1)
                        }) as isize,
                    ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0
            {
                *last = !*last;
            }
            assert!(0 <= ssize);
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
                            (if 0 <= *first.sub(1) {
                                *first.sub(1)
                            } else {
                                !*first.sub(1)
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
                        (if 0 <= *last.sub(1) {
                            *last.sub(1)
                        } else {
                            !*last.sub(1)
                        }) as isize,
                    ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0
            {
                *last = !*last;
            }
            assert!(0 <= ssize);
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
                                .sub(1)
                        {
                            *middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .sub(1)
                        } else {
                            !*middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .sub(1)
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
                                l = l.sub(1);
                                if *l >= 0 {
                                    break;
                                }
                            }
                            next |= 4;
                        }
                        next |= 1;
                    } else if first < lm {
                        while *r < 0 {
                            r = r.add(1);
                        }
                        next |= 2;
                    }
                }
                if l.offset_from(first) as core::ffi::c_long
                    <= last.offset_from(r) as core::ffi::c_long
                {
                    assert!(ssize < STACK_SIZE);
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
                    assert!(ssize < STACK_SIZE);
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
                        (if 0 <= *middle.sub(1) {
                            *middle.sub(1)
                        } else {
                            !*middle.sub(1)
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
                                (if 0 <= *first.sub(1) {
                                    *first.sub(1)
                                } else {
                                    !*first.sub(1)
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
                            (if 0 <= *last.sub(1) {
                                *last.sub(1)
                            } else {
                                !*last.sub(1)
                            }) as isize,
                        ),
                        PA.offset(*last as isize),
                        depth,
                    ) == 0
                {
                    *last = !*last;
                }
                assert!(0 <= ssize);
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
    T: *const core::ffi::c_uchar,
    PA: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
    mut buf: *mut core::ffi::c_int,
    mut bufsize: core::ffi::c_int,
    depth: core::ffi::c_int,
    n: core::ffi::c_int,
    lastsuffix: core::ffi::c_int,
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
        first = first.add(1);
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
        *PAi.as_mut_ptr() = *PA.offset(*first.sub(1) as isize);
        *PAi.as_mut_ptr().add(1) = n - 2;
        a = first;
        i = *first.sub(1);
        while a < last
            && (*a < 0
                || (0)
                    < ss_compare(
                        T,
                        &*PAi.as_mut_ptr(),
                        PA.offset(*a as isize),
                        depth,
                    ))
        {
            *a.sub(1) = *a;
            a = a.add(1);
        }
        *a.sub(1) = i;
    }
}
#[inline]
unsafe fn tr_ilg(n: core::ffi::c_int) -> core::ffi::c_int {
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
    ISAd: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
) {
    let mut a = core::ptr::null_mut::<core::ffi::c_int>();
    let mut b = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    let mut r: core::ffi::c_int = 0;
    a = first.add(1);
    while a < last {
        t = *a;
        b = a.sub(1);
        loop {
            r = *ISAd.offset(t as isize) - *ISAd.offset(*b as isize);
            if 0 <= r {
                break;
            }
            loop {
                *b.add(1) = *b;
                b = b.sub(1);
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
        *b.add(1) = t;
        a = a.add(1);
    }
}
#[inline]
unsafe fn tr_fixdown(
    ISAd: *const core::ffi::c_int,
    SA: *mut core::ffi::c_int,
    mut i: core::ffi::c_int,
    size: core::ffi::c_int,
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
    ISAd: *const core::ffi::c_int,
    SA: *mut core::ffi::c_int,
    size: core::ffi::c_int,
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
        t = *SA;
        *SA = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        tr_fixdown(ISAd, SA, 0, m);
    }
    i = m - 1;
    while (0) < i {
        t = *SA;
        *SA = *SA.offset(i as isize);
        tr_fixdown(ISAd, SA, 0, i);
        *SA.offset(i as isize) = t;
        i -= 1;
    }
}
#[inline]
unsafe fn tr_median3(
    ISAd: *const core::ffi::c_int,
    mut v1: *mut core::ffi::c_int,
    mut v2: *mut core::ffi::c_int,
    v3: *mut core::ffi::c_int,
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
    ISAd: *const core::ffi::c_int,
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
    ISAd: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
) -> *mut core::ffi::c_int {
    let mut middle = core::ptr::null_mut::<core::ffi::c_int>();
    let mut t: core::ffi::c_int = 0;
    t = last.offset_from(first) as core::ffi::c_long as core::ffi::c_int;
    middle = first.offset((t / 2) as isize);
    if t <= 512 {
        if t <= 32 {
            return tr_median3(ISAd, first, middle, last.sub(1));
        } else {
            t >>= 2;
            return tr_median5(
                ISAd,
                first,
                first.offset(t as isize),
                middle,
                last.sub(1).offset(-(t as isize)),
                last.sub(1),
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
        last.sub(1).offset(-((t << 1) as isize)),
        last.sub(1).offset(-(t as isize)),
        last.sub(1),
    );
    tr_median3(ISAd, first, middle, last)
}
#[inline]
unsafe fn trbudget_init(
    budget: *mut trbudget_t,
    chance: core::ffi::c_int,
    incval: core::ffi::c_int,
) {
    (*budget).chance = chance;
    (*budget).incval = incval;
    (*budget).remain = (*budget).incval;
}
#[inline]
unsafe fn trbudget_check(budget: *mut trbudget_t, size: core::ffi::c_int) -> core::ffi::c_int {
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
    ISAd: *const core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    middle: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    pa: *mut *mut core::ffi::c_int,
    pb: *mut *mut core::ffi::c_int,
    v: core::ffi::c_int,
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
    b = middle.sub(1);
    loop {
        b = b.add(1);
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
            b = b.add(1);
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
                a = a.add(1);
            }
        }
    }
    c = last;
    loop {
        c = c.sub(1);
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
            c = c.sub(1);
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
                d = d.sub(1);
            }
        }
    }
    while b < c {
        t = *b;
        *b = *c;
        *c = t;
        loop {
            b = b.add(1);
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
                a = a.add(1);
            }
        }
        loop {
            c = c.sub(1);
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
                d = d.sub(1);
            }
        }
    }
    if a <= d {
        c = b.sub(1);
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
            e = e.add(1);
            f = f.add(1);
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
            e = e.add(1);
            f = f.add(1);
        }
        first = first.offset(b.offset_from(a) as core::ffi::c_long as isize);
        last = last.offset(-(d.offset_from(c) as core::ffi::c_long as isize));
    }
    *pa = first;
    *pb = last;
}
unsafe fn tr_copy(
    ISA: *mut core::ffi::c_int,
    SA: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    a: *mut core::ffi::c_int,
    b: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
    depth: core::ffi::c_int,
) {
    let mut c = core::ptr::null_mut::<core::ffi::c_int>();
    let mut d = core::ptr::null_mut::<core::ffi::c_int>();
    let mut e = core::ptr::null_mut::<core::ffi::c_int>();
    let mut s: core::ffi::c_int = 0;
    let mut v: core::ffi::c_int = 0;
    v = (b.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
    c = first;
    d = a.sub(1);
    while c <= d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.add(1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
        }
        c = c.add(1);
    }
    c = last.sub(1);
    e = d.add(1);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.sub(1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
        }
        c = c.sub(1);
    }
}
unsafe fn tr_partialcopy(
    ISA: *mut core::ffi::c_int,
    SA: *const core::ffi::c_int,
    first: *mut core::ffi::c_int,
    a: *mut core::ffi::c_int,
    b: *mut core::ffi::c_int,
    last: *mut core::ffi::c_int,
    depth: core::ffi::c_int,
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
    d = a.sub(1);
    while c <= d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.add(1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.add(1);
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
        e = e.sub(1);
    }
    lastrank = -(1);
    c = last.sub(1);
    e = d.add(1);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 <= s && *ISA.offset(s as isize) == v {
            d = d.sub(1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as core::ffi::c_long as core::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.sub(1);
    }
}
unsafe fn tr_introsort(
    ISA: *mut core::ffi::c_int,
    mut ISAd: *const core::ffi::c_int,
    SA: *mut core::ffi::c_int,
    mut first: *mut core::ffi::c_int,
    mut last: *mut core::ffi::c_int,
    budget: *mut trbudget_t,
) {
    const STACK_SIZE: core::ffi::c_int = 64;

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
    let incr = ISAd.offset_from(ISA) as core::ffi::c_long as core::ffi::c_int;
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
                        c = c.add(1);
                    }
                }
                if b < last {
                    c = a;
                    v = (b.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                    while c < b {
                        *ISA.offset(*c as isize) = v;
                        c = c.add(1);
                    }
                }
                if (1) < b.offset_from(a) as core::ffi::c_long {
                    assert!(ssize < STACK_SIZE);
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
                    assert!(ssize < STACK_SIZE);
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
                        assert!(ssize < STACK_SIZE);
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
                        assert!(0 <= ssize);
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
                    assert!(ssize < STACK_SIZE);
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
                    assert!(0 <= ssize);
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
                assert!(0 <= ssize);
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
                        a = a.add(1);
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
                        a = a.add(1);
                        if *a >= 0 {
                            break;
                        }
                    }
                    next = if *ISA.offset(*a as isize) != *ISAd.offset(*a as isize) {
                        tr_ilg((a.offset_from(first) as core::ffi::c_long + 1) as core::ffi::c_int)
                    } else {
                        -(1)
                    };
                    a = a.add(1);
                    if a < last {
                        b = first;
                        v = (a.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                        while b < a {
                            *ISA.offset(*b as isize) = v;
                            b = b.add(1);
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
                            assert!(ssize < STACK_SIZE);
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
                            assert!(ssize < STACK_SIZE);
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
                            assert!(0 <= ssize);
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
                    assert!(0 <= ssize);
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
                a = last.sub(1);
                while first < a {
                    x = *ISAd.offset(*a as isize);
                    b = a.sub(1);
                    while first <= b && *ISAd.offset(*b as isize) == x {
                        *b = !*b;
                        b = b.sub(1);
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
                tr_partition(ISAd, first, first.add(1), last, &mut a, &mut b, v);
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
                        c = c.add(1);
                    }
                    if b < last {
                        c = a;
                        v = (b.offset_from(SA) as core::ffi::c_long - 1) as core::ffi::c_int;
                        while c < b {
                            *ISA.offset(*c as isize) = v;
                            c = c.add(1);
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
                                    assert!(ssize < STACK_SIZE);
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
                                    assert!(ssize < STACK_SIZE);
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
                                    assert!(ssize < STACK_SIZE);
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
                                    assert!(ssize < STACK_SIZE);
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
                                    assert!(ssize < STACK_SIZE);
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
                                    assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                            assert!(ssize < STACK_SIZE);
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
                            assert!(ssize < STACK_SIZE);
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
                                assert!(ssize < STACK_SIZE);
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
                                assert!(0 <= ssize);
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
                            assert!(ssize < STACK_SIZE);
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
                            assert!(0 <= ssize);
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
                    assert!(0 <= ssize);
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
    ISA: *mut core::ffi::c_int,
    SA: *mut core::ffi::c_int,
    n: core::ffi::c_int,
    depth: core::ffi::c_int,
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
    #[expect(clippy::while_immutable_condition)]
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
                last = SA.offset(*ISA.offset(t as isize) as isize).add(1);
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
    T: *const core::ffi::c_uchar,
    SA: *mut core::ffi::c_int,
    bucket_A: *mut core::ffi::c_int,
    bucket_B: *mut core::ffi::c_int,
    n: core::ffi::c_int,
    _openMP: core::ffi::c_int,
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
    T: *const core::ffi::c_uchar,
    SA: *mut core::ffi::c_int,
    bucket_A: *mut core::ffi::c_int,
    bucket_B: *mut core::ffi::c_int,
    n: core::ffi::c_int,
    m: core::ffi::c_int,
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
                .sub(1);
            k = core::ptr::null_mut();
            c2 = -(1);
            while i <= j {
                s = *j;
                if (0) < s {
                    assert_eq!(*T.offset(s as isize) as core::ffi::c_int, c1);
                    assert!(
                        (s + 1) < n
                            && *T.offset(s as isize) as core::ffi::c_int
                                <= *T.offset((s + 1) as isize) as core::ffi::c_int
                    );
                    assert!(
                        *T.offset((s - 1) as isize) as core::ffi::c_int
                            <= *T.offset(s as isize) as core::ffi::c_int
                    );
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
                    assert!(k < j);
                    assert!(!k.is_null());
                    let fresh192 = k;
                    k = k.sub(1);
                    *fresh192 = s;
                } else {
                    assert!(s == 0 && *T.offset(s as isize) as core::ffi::c_int == c1 || s < 0);
                    *j = !s;
                }
                j = j.sub(1);
            }
            c1 -= 1;
        }
    }
    c2 = *T.offset((n - 1) as isize) as core::ffi::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh193 = k;
    k = k.add(1);
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
            assert!(
                *T.offset((s - 1) as isize) as core::ffi::c_int
                    >= *T.offset(s as isize) as core::ffi::c_int
            );
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
            assert!(i < k);
            let fresh194 = k;
            k = k.add(1);
            *fresh194 = s;
        } else {
            assert!(s < 0);
            *i = !s;
        }
        i = i.add(1);
    }
}
pub(super) unsafe fn divsufsort(
    T: *const core::ffi::c_uchar,
    SA: *mut core::ffi::c_int,
    n: core::ffi::c_int,
    openMP: core::ffi::c_int,
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
        *SA = 0;
        return 0;
    } else if n == 2 {
        m = ((*T as core::ffi::c_int) < *T.add(1) as core::ffi::c_int)
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
