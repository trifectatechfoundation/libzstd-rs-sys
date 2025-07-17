use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const std::ffi::c_char,
        __file: *const std::ffi::c_char,
        __line: std::ffi::c_uint,
        __function: *const std::ffi::c_char,
    ) -> !;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
}
pub type size_t = std::ffi::c_ulong;
pub type trbudget_t = _trbudget_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _trbudget_t {
    pub chance: std::ffi::c_int,
    pub remain: std::ffi::c_int,
    pub incval: std::ffi::c_int,
    pub count: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub a: *const std::ffi::c_int,
    pub b: *mut std::ffi::c_int,
    pub c: *mut std::ffi::c_int,
    pub d: std::ffi::c_int,
    pub e: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub a: *mut std::ffi::c_int,
    pub b: *mut std::ffi::c_int,
    pub c: std::ffi::c_int,
    pub d: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub a: *mut std::ffi::c_int,
    pub b: *mut std::ffi::c_int,
    pub c: *mut std::ffi::c_int,
    pub d: std::ffi::c_int,
}
pub const NULL: std::ffi::c_int = 0 as std::ffi::c_int;
pub const ALPHABET_SIZE: std::ffi::c_int = 256 as std::ffi::c_int;
pub const BUCKET_A_SIZE: std::ffi::c_int = 256 as std::ffi::c_int;
pub const BUCKET_B_SIZE: std::ffi::c_int = ALPHABET_SIZE * ALPHABET_SIZE;
pub const SS_INSERTIONSORT_THRESHOLD: std::ffi::c_int = 8 as std::ffi::c_int;
pub const SS_BLOCKSIZE: std::ffi::c_int = 1024 as std::ffi::c_int;
pub const TR_INSERTIONSORT_THRESHOLD: std::ffi::c_int = 8 as std::ffi::c_int;
static mut lg_table: [std::ffi::c_int; 256] = [
    -(1 as std::ffi::c_int),
    0 as std::ffi::c_int,
    1 as std::ffi::c_int,
    1 as std::ffi::c_int,
    2 as std::ffi::c_int,
    2 as std::ffi::c_int,
    2 as std::ffi::c_int,
    2 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    3 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    4 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    5 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    6 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
    7 as std::ffi::c_int,
];
#[inline]
unsafe extern "C" fn ss_ilg(mut n: std::ffi::c_int) -> std::ffi::c_int {
    return if n & 0xff00 as std::ffi::c_int != 0 {
        8 as std::ffi::c_int
            + *lg_table
                .as_ptr()
                .offset((n >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
    } else {
        0 as std::ffi::c_int
            + *lg_table
                .as_ptr()
                .offset((n >> 0 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
    };
}
static mut sqq_table: [std::ffi::c_int; 256] = [
    0 as std::ffi::c_int,
    16 as std::ffi::c_int,
    22 as std::ffi::c_int,
    27 as std::ffi::c_int,
    32 as std::ffi::c_int,
    35 as std::ffi::c_int,
    39 as std::ffi::c_int,
    42 as std::ffi::c_int,
    45 as std::ffi::c_int,
    48 as std::ffi::c_int,
    50 as std::ffi::c_int,
    53 as std::ffi::c_int,
    55 as std::ffi::c_int,
    57 as std::ffi::c_int,
    59 as std::ffi::c_int,
    61 as std::ffi::c_int,
    64 as std::ffi::c_int,
    65 as std::ffi::c_int,
    67 as std::ffi::c_int,
    69 as std::ffi::c_int,
    71 as std::ffi::c_int,
    73 as std::ffi::c_int,
    75 as std::ffi::c_int,
    76 as std::ffi::c_int,
    78 as std::ffi::c_int,
    80 as std::ffi::c_int,
    81 as std::ffi::c_int,
    83 as std::ffi::c_int,
    84 as std::ffi::c_int,
    86 as std::ffi::c_int,
    87 as std::ffi::c_int,
    89 as std::ffi::c_int,
    90 as std::ffi::c_int,
    91 as std::ffi::c_int,
    93 as std::ffi::c_int,
    94 as std::ffi::c_int,
    96 as std::ffi::c_int,
    97 as std::ffi::c_int,
    98 as std::ffi::c_int,
    99 as std::ffi::c_int,
    101 as std::ffi::c_int,
    102 as std::ffi::c_int,
    103 as std::ffi::c_int,
    104 as std::ffi::c_int,
    106 as std::ffi::c_int,
    107 as std::ffi::c_int,
    108 as std::ffi::c_int,
    109 as std::ffi::c_int,
    110 as std::ffi::c_int,
    112 as std::ffi::c_int,
    113 as std::ffi::c_int,
    114 as std::ffi::c_int,
    115 as std::ffi::c_int,
    116 as std::ffi::c_int,
    117 as std::ffi::c_int,
    118 as std::ffi::c_int,
    119 as std::ffi::c_int,
    120 as std::ffi::c_int,
    121 as std::ffi::c_int,
    122 as std::ffi::c_int,
    123 as std::ffi::c_int,
    124 as std::ffi::c_int,
    125 as std::ffi::c_int,
    126 as std::ffi::c_int,
    128 as std::ffi::c_int,
    128 as std::ffi::c_int,
    129 as std::ffi::c_int,
    130 as std::ffi::c_int,
    131 as std::ffi::c_int,
    132 as std::ffi::c_int,
    133 as std::ffi::c_int,
    134 as std::ffi::c_int,
    135 as std::ffi::c_int,
    136 as std::ffi::c_int,
    137 as std::ffi::c_int,
    138 as std::ffi::c_int,
    139 as std::ffi::c_int,
    140 as std::ffi::c_int,
    141 as std::ffi::c_int,
    142 as std::ffi::c_int,
    143 as std::ffi::c_int,
    144 as std::ffi::c_int,
    144 as std::ffi::c_int,
    145 as std::ffi::c_int,
    146 as std::ffi::c_int,
    147 as std::ffi::c_int,
    148 as std::ffi::c_int,
    149 as std::ffi::c_int,
    150 as std::ffi::c_int,
    150 as std::ffi::c_int,
    151 as std::ffi::c_int,
    152 as std::ffi::c_int,
    153 as std::ffi::c_int,
    154 as std::ffi::c_int,
    155 as std::ffi::c_int,
    155 as std::ffi::c_int,
    156 as std::ffi::c_int,
    157 as std::ffi::c_int,
    158 as std::ffi::c_int,
    159 as std::ffi::c_int,
    160 as std::ffi::c_int,
    160 as std::ffi::c_int,
    161 as std::ffi::c_int,
    162 as std::ffi::c_int,
    163 as std::ffi::c_int,
    163 as std::ffi::c_int,
    164 as std::ffi::c_int,
    165 as std::ffi::c_int,
    166 as std::ffi::c_int,
    167 as std::ffi::c_int,
    167 as std::ffi::c_int,
    168 as std::ffi::c_int,
    169 as std::ffi::c_int,
    170 as std::ffi::c_int,
    170 as std::ffi::c_int,
    171 as std::ffi::c_int,
    172 as std::ffi::c_int,
    173 as std::ffi::c_int,
    173 as std::ffi::c_int,
    174 as std::ffi::c_int,
    175 as std::ffi::c_int,
    176 as std::ffi::c_int,
    176 as std::ffi::c_int,
    177 as std::ffi::c_int,
    178 as std::ffi::c_int,
    178 as std::ffi::c_int,
    179 as std::ffi::c_int,
    180 as std::ffi::c_int,
    181 as std::ffi::c_int,
    181 as std::ffi::c_int,
    182 as std::ffi::c_int,
    183 as std::ffi::c_int,
    183 as std::ffi::c_int,
    184 as std::ffi::c_int,
    185 as std::ffi::c_int,
    185 as std::ffi::c_int,
    186 as std::ffi::c_int,
    187 as std::ffi::c_int,
    187 as std::ffi::c_int,
    188 as std::ffi::c_int,
    189 as std::ffi::c_int,
    189 as std::ffi::c_int,
    190 as std::ffi::c_int,
    191 as std::ffi::c_int,
    192 as std::ffi::c_int,
    192 as std::ffi::c_int,
    193 as std::ffi::c_int,
    193 as std::ffi::c_int,
    194 as std::ffi::c_int,
    195 as std::ffi::c_int,
    195 as std::ffi::c_int,
    196 as std::ffi::c_int,
    197 as std::ffi::c_int,
    197 as std::ffi::c_int,
    198 as std::ffi::c_int,
    199 as std::ffi::c_int,
    199 as std::ffi::c_int,
    200 as std::ffi::c_int,
    201 as std::ffi::c_int,
    201 as std::ffi::c_int,
    202 as std::ffi::c_int,
    203 as std::ffi::c_int,
    203 as std::ffi::c_int,
    204 as std::ffi::c_int,
    204 as std::ffi::c_int,
    205 as std::ffi::c_int,
    206 as std::ffi::c_int,
    206 as std::ffi::c_int,
    207 as std::ffi::c_int,
    208 as std::ffi::c_int,
    208 as std::ffi::c_int,
    209 as std::ffi::c_int,
    209 as std::ffi::c_int,
    210 as std::ffi::c_int,
    211 as std::ffi::c_int,
    211 as std::ffi::c_int,
    212 as std::ffi::c_int,
    212 as std::ffi::c_int,
    213 as std::ffi::c_int,
    214 as std::ffi::c_int,
    214 as std::ffi::c_int,
    215 as std::ffi::c_int,
    215 as std::ffi::c_int,
    216 as std::ffi::c_int,
    217 as std::ffi::c_int,
    217 as std::ffi::c_int,
    218 as std::ffi::c_int,
    218 as std::ffi::c_int,
    219 as std::ffi::c_int,
    219 as std::ffi::c_int,
    220 as std::ffi::c_int,
    221 as std::ffi::c_int,
    221 as std::ffi::c_int,
    222 as std::ffi::c_int,
    222 as std::ffi::c_int,
    223 as std::ffi::c_int,
    224 as std::ffi::c_int,
    224 as std::ffi::c_int,
    225 as std::ffi::c_int,
    225 as std::ffi::c_int,
    226 as std::ffi::c_int,
    226 as std::ffi::c_int,
    227 as std::ffi::c_int,
    227 as std::ffi::c_int,
    228 as std::ffi::c_int,
    229 as std::ffi::c_int,
    229 as std::ffi::c_int,
    230 as std::ffi::c_int,
    230 as std::ffi::c_int,
    231 as std::ffi::c_int,
    231 as std::ffi::c_int,
    232 as std::ffi::c_int,
    232 as std::ffi::c_int,
    233 as std::ffi::c_int,
    234 as std::ffi::c_int,
    234 as std::ffi::c_int,
    235 as std::ffi::c_int,
    235 as std::ffi::c_int,
    236 as std::ffi::c_int,
    236 as std::ffi::c_int,
    237 as std::ffi::c_int,
    237 as std::ffi::c_int,
    238 as std::ffi::c_int,
    238 as std::ffi::c_int,
    239 as std::ffi::c_int,
    240 as std::ffi::c_int,
    240 as std::ffi::c_int,
    241 as std::ffi::c_int,
    241 as std::ffi::c_int,
    242 as std::ffi::c_int,
    242 as std::ffi::c_int,
    243 as std::ffi::c_int,
    243 as std::ffi::c_int,
    244 as std::ffi::c_int,
    244 as std::ffi::c_int,
    245 as std::ffi::c_int,
    245 as std::ffi::c_int,
    246 as std::ffi::c_int,
    246 as std::ffi::c_int,
    247 as std::ffi::c_int,
    247 as std::ffi::c_int,
    248 as std::ffi::c_int,
    248 as std::ffi::c_int,
    249 as std::ffi::c_int,
    249 as std::ffi::c_int,
    250 as std::ffi::c_int,
    250 as std::ffi::c_int,
    251 as std::ffi::c_int,
    251 as std::ffi::c_int,
    252 as std::ffi::c_int,
    252 as std::ffi::c_int,
    253 as std::ffi::c_int,
    253 as std::ffi::c_int,
    254 as std::ffi::c_int,
    254 as std::ffi::c_int,
    255 as std::ffi::c_int,
];
#[inline]
unsafe extern "C" fn ss_isqrt(mut x: std::ffi::c_int) -> std::ffi::c_int {
    let mut y: std::ffi::c_int = 0;
    let mut e: std::ffi::c_int = 0;
    if x >= SS_BLOCKSIZE * SS_BLOCKSIZE {
        return SS_BLOCKSIZE;
    }
    e = if x as std::ffi::c_uint & 0xffff0000 as std::ffi::c_uint != 0 {
        if x as std::ffi::c_uint & 0xff000000 as std::ffi::c_uint != 0 {
            24 as std::ffi::c_int
                + *lg_table
                    .as_ptr()
                    .offset((x >> 24 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
        } else {
            16 as std::ffi::c_int
                + *lg_table
                    .as_ptr()
                    .offset((x >> 16 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
        }
    } else if x & 0xff00 as std::ffi::c_int != 0 {
        8 as std::ffi::c_int
            + *lg_table
                .as_ptr()
                .offset((x >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
    } else {
        0 as std::ffi::c_int
            + *lg_table
                .as_ptr()
                .offset((x >> 0 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
    };
    if e >= 16 as std::ffi::c_int {
        y = *sqq_table
            .as_ptr()
            .offset((x >> e - 6 as std::ffi::c_int - (e & 1 as std::ffi::c_int)) as isize)
            << (e >> 1 as std::ffi::c_int) - 7 as std::ffi::c_int;
        if e >= 24 as std::ffi::c_int {
            y = y + 1 as std::ffi::c_int + x / y >> 1 as std::ffi::c_int;
        }
        y = y + 1 as std::ffi::c_int + x / y >> 1 as std::ffi::c_int;
    } else if e >= 8 as std::ffi::c_int {
        y = (*sqq_table
            .as_ptr()
            .offset((x >> e - 6 as std::ffi::c_int - (e & 1 as std::ffi::c_int)) as isize)
            >> 7 as std::ffi::c_int - (e >> 1 as std::ffi::c_int))
            + 1 as std::ffi::c_int;
    } else {
        return *sqq_table.as_ptr().offset(x as isize) >> 4 as std::ffi::c_int;
    }
    return if x < y * y {
        y - 1 as std::ffi::c_int
    } else {
        y
    };
}
#[inline]
unsafe extern "C" fn ss_compare(
    mut T: *const std::ffi::c_uchar,
    mut p1: *const std::ffi::c_int,
    mut p2: *const std::ffi::c_int,
    mut depth: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut U1 = 0 as *const std::ffi::c_uchar;
    let mut U2 = 0 as *const std::ffi::c_uchar;
    let mut U1n = 0 as *const std::ffi::c_uchar;
    let mut U2n = 0 as *const std::ffi::c_uchar;
    U1 = T.offset(depth as isize).offset(*p1 as isize);
    U2 = T.offset(depth as isize).offset(*p2 as isize);
    U1n = T
        .offset(*p1.offset(1 as std::ffi::c_int as isize) as isize)
        .offset(2 as std::ffi::c_int as isize);
    U2n = T
        .offset(*p2.offset(1 as std::ffi::c_int as isize) as isize)
        .offset(2 as std::ffi::c_int as isize);
    while U1 < U1n && U2 < U2n && *U1 as std::ffi::c_int == *U2 as std::ffi::c_int {
        U1 = U1.offset(1);
        U1;
        U2 = U2.offset(1);
        U2;
    }
    return if U1 < U1n {
        if U2 < U2n {
            *U1 as std::ffi::c_int - *U2 as std::ffi::c_int
        } else {
            1 as std::ffi::c_int
        }
    } else if U2 < U2n {
        -(1 as std::ffi::c_int)
    } else {
        0 as std::ffi::c_int
    };
}
unsafe extern "C" fn ss_insertionsort(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    i = last.offset(-(2 as std::ffi::c_int as isize));
    while first <= i {
        t = *i;
        j = i.offset(1 as std::ffi::c_int as isize);
        loop {
            r = ss_compare(T, PA.offset(t as isize), PA.offset(*j as isize), depth);
            if !((0 as std::ffi::c_int) < r) {
                break;
            }
            loop {
                *j.offset(-(1 as std::ffi::c_int as isize)) = *j;
                j = j.offset(1);
                if !(j < last && *j < 0 as std::ffi::c_int) {
                    break;
                }
            }
            if last <= j {
                break;
            }
        }
        if r == 0 as std::ffi::c_int {
            *j = !*j;
        }
        *j.offset(-(1 as std::ffi::c_int as isize)) = t;
        i = i.offset(-1);
        i;
    }
}
#[inline]
unsafe extern "C" fn ss_fixdown(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut i: std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut c: std::ffi::c_int = 0;
    let mut d: std::ffi::c_int = 0;
    let mut e: std::ffi::c_int = 0;
    v = *SA.offset(i as isize);
    c = *Td.offset(*PA.offset(v as isize) as isize) as std::ffi::c_int;
    loop {
        j = 2 as std::ffi::c_int * i + 1 as std::ffi::c_int;
        if !(j < size) {
            break;
        }
        let fresh0 = j;
        j = j + 1;
        k = fresh0;
        d = *Td.offset(*PA.offset(*SA.offset(k as isize) as isize) as isize) as std::ffi::c_int;
        e = *Td.offset(*PA.offset(*SA.offset(j as isize) as isize) as isize) as std::ffi::c_int;
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
unsafe extern "C" fn ss_heapsort(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let mut m: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    m = size;
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        m -= 1;
        m;
        if (*Td
            .offset(*PA.offset(*SA.offset((m / 2 as std::ffi::c_int) as isize) as isize) as isize)
            as std::ffi::c_int)
            < *Td.offset(*PA.offset(*SA.offset(m as isize) as isize) as isize) as std::ffi::c_int
        {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2 as std::ffi::c_int) as isize);
            *SA.offset((m / 2 as std::ffi::c_int) as isize) = t;
        }
    }
    i = m / 2 as std::ffi::c_int - 1 as std::ffi::c_int;
    while 0 as std::ffi::c_int <= i {
        ss_fixdown(Td, PA, SA, i, m);
        i -= 1;
        i;
    }
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        t = *SA.offset(0 as std::ffi::c_int as isize);
        *SA.offset(0 as std::ffi::c_int as isize) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        ss_fixdown(Td, PA, SA, 0 as std::ffi::c_int, m);
    }
    i = m - 1 as std::ffi::c_int;
    while (0 as std::ffi::c_int) < i {
        t = *SA.offset(0 as std::ffi::c_int as isize);
        *SA.offset(0 as std::ffi::c_int as isize) = *SA.offset(i as isize);
        ss_fixdown(Td, PA, SA, 0 as std::ffi::c_int, i);
        *SA.offset(i as isize) = t;
        i -= 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn ss_median3(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
    {
        t = v1;
        v1 = v2;
        v2 = t;
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
    {
        if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
            > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
        {
            return v1;
        } else {
            return v3;
        }
    }
    return v2;
}
#[inline]
unsafe extern "C" fn ss_median5(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
    mut v4: *mut std::ffi::c_int,
    mut v5: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
    {
        t = v2;
        v2 = v3;
        v3 = t;
    }
    if *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v5 as isize) as isize) as std::ffi::c_int
    {
        t = v4;
        v4 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v2 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
    {
        t = v2;
        v2 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
    {
        t = v1;
        v1 = v3;
        v3 = t;
    }
    if *Td.offset(*PA.offset(*v1 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
    {
        t = v1;
        v1 = v4;
        v4 = t;
        t = v3;
        v3 = v5;
        v5 = t;
    }
    if *Td.offset(*PA.offset(*v3 as isize) as isize) as std::ffi::c_int
        > *Td.offset(*PA.offset(*v4 as isize) as isize) as std::ffi::c_int
    {
        return v4;
    }
    return v3;
}
#[inline]
unsafe extern "C" fn ss_pivot(
    mut Td: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut middle = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    t = last.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
    middle = first.offset((t / 2 as std::ffi::c_int) as isize);
    if t <= 512 as std::ffi::c_int {
        if t <= 32 as std::ffi::c_int {
            return ss_median3(
                Td,
                PA,
                first,
                middle,
                last.offset(-(1 as std::ffi::c_int as isize)),
            );
        } else {
            t >>= 2 as std::ffi::c_int;
            return ss_median5(
                Td,
                PA,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1 as std::ffi::c_int as isize))
                    .offset(-(t as isize)),
                last.offset(-(1 as std::ffi::c_int as isize)),
            );
        }
    }
    t >>= 3 as std::ffi::c_int;
    first = ss_median3(
        Td,
        PA,
        first,
        first.offset(t as isize),
        first.offset((t << 1 as std::ffi::c_int) as isize),
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
        last.offset(-(1 as std::ffi::c_int as isize))
            .offset(-((t << 1 as std::ffi::c_int) as isize)),
        last.offset(-(1 as std::ffi::c_int as isize))
            .offset(-(t as isize)),
        last.offset(-(1 as std::ffi::c_int as isize)),
    );
    return ss_median3(Td, PA, first, middle, last);
}
#[inline]
unsafe extern "C" fn ss_partition(
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    a = first.offset(-(1 as std::ffi::c_int as isize));
    b = last;
    loop {
        loop {
            a = a.offset(1);
            if !(a < b
                && *PA.offset(*a as isize) + depth
                    >= *PA.offset((*a + 1 as std::ffi::c_int) as isize) + 1 as std::ffi::c_int)
            {
                break;
            }
            *a = !*a;
        }
        loop {
            b = b.offset(-1);
            if !(a < b
                && *PA.offset(*b as isize) + depth
                    < *PA.offset((*b + 1 as std::ffi::c_int) as isize) + 1 as std::ffi::c_int)
            {
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
    return a;
}
unsafe extern "C" fn ss_mintrosort(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut stack: [C2RustUnnamed_0; 16] = [C2RustUnnamed_0 {
        a: 0 as *mut std::ffi::c_int,
        b: 0 as *mut std::ffi::c_int,
        c: 0,
        d: 0,
    }; 16];
    let mut Td = 0 as *const std::ffi::c_uchar;
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut f = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    let mut ssize: std::ffi::c_int = 0;
    let mut limit: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut x = 0 as std::ffi::c_int;
    ssize = 0 as std::ffi::c_int;
    limit = ss_ilg(last.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
    loop {
        if last.offset_from(first) as std::ffi::c_long
            <= SS_INSERTIONSORT_THRESHOLD as std::ffi::c_long
        {
            if (1 as std::ffi::c_int as std::ffi::c_long)
                < last.offset_from(first) as std::ffi::c_long
            {
                ss_insertionsort(T, PA, first, last, depth);
            }
            if 0 as std::ffi::c_int <= ssize {
            } else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    418 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 74],
                        &[std::ffi::c_char; 74],
                    >(
                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_15864: {
                if 0 as std::ffi::c_int <= ssize {
                } else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        418 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 74],
                            &[std::ffi::c_char; 74],
                        >(
                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if ssize == 0 as std::ffi::c_int {
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
            limit = limit - 1;
            if fresh1 == 0 as std::ffi::c_int {
                ss_heapsort(
                    Td,
                    PA,
                    first,
                    last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                );
            }
            if limit < 0 as std::ffi::c_int {
                a = first.offset(1 as std::ffi::c_int as isize);
                v = *Td.offset(*PA.offset(*first as isize) as isize) as std::ffi::c_int;
                while a < last {
                    x = *Td.offset(*PA.offset(*a as isize) as isize) as std::ffi::c_int;
                    if x != v {
                        if (1 as std::ffi::c_int as std::ffi::c_long)
                            < a.offset_from(first) as std::ffi::c_long
                        {
                            break;
                        }
                        v = x;
                        first = a;
                    }
                    a = a.offset(1);
                    a;
                }
                if (*Td.offset((*PA.offset(*first as isize) - 1 as std::ffi::c_int) as isize)
                    as std::ffi::c_int)
                    < v
                {
                    first = ss_partition(PA, first, a, depth);
                }
                if a.offset_from(first) as std::ffi::c_long
                    <= last.offset_from(a) as std::ffi::c_long
                {
                    if (1 as std::ffi::c_int as std::ffi::c_long)
                        < a.offset_from(first) as std::ffi::c_long
                    {
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                437 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_15214: {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    437 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh2 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh2 = a;
                        let ref mut fresh3 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh3 = last;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh4 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh4 as isize)).d = -(1 as std::ffi::c_int);
                        last = a;
                        depth += 1 as std::ffi::c_int;
                        limit = ss_ilg(a.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                    } else {
                        first = a;
                        limit = -(1 as std::ffi::c_int);
                    }
                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                    < last.offset_from(a) as std::ffi::c_long
                {
                    if ssize < 16 as std::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            444 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 74],
                                &[std::ffi::c_char; 74],
                            >(
                                b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_15085: {
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                444 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let ref mut fresh5 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh5 = first;
                    let ref mut fresh6 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh6 = a;
                    (*stack.as_mut_ptr().offset(ssize as isize)).c = depth + 1 as std::ffi::c_int;
                    let fresh7 = ssize;
                    ssize = ssize + 1;
                    (*stack.as_mut_ptr().offset(fresh7 as isize)).d =
                        ss_ilg(a.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                    first = a;
                    limit = -(1 as std::ffi::c_int);
                } else {
                    last = a;
                    depth += 1 as std::ffi::c_int;
                    limit = ss_ilg(a.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                }
            } else {
                a = ss_pivot(Td, PA, first, last);
                v = *Td.offset(*PA.offset(*a as isize) as isize) as std::ffi::c_int;
                t = *first;
                *first = *a;
                *a = t;
                b = first;
                loop {
                    b = b.offset(1);
                    if !(b < last && {
                        x = *Td.offset(*PA.offset(*b as isize) as isize) as std::ffi::c_int;
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
                            x = *Td.offset(*PA.offset(*b as isize) as isize) as std::ffi::c_int;
                            x <= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *b;
                            *b = *a;
                            *a = t;
                            a = a.offset(1);
                            a;
                        }
                    }
                }
                c = last;
                loop {
                    c = c.offset(-1);
                    if !(b < c && {
                        x = *Td.offset(*PA.offset(*c as isize) as isize) as std::ffi::c_int;
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
                            x = *Td.offset(*PA.offset(*c as isize) as isize) as std::ffi::c_int;
                            x >= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *c;
                            *c = *d;
                            *d = t;
                            d = d.offset(-1);
                            d;
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
                            x = *Td.offset(*PA.offset(*b as isize) as isize) as std::ffi::c_int;
                            x <= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *b;
                            *b = *a;
                            *a = t;
                            a = a.offset(1);
                            a;
                        }
                    }
                    loop {
                        c = c.offset(-1);
                        if !(b < c && {
                            x = *Td.offset(*PA.offset(*c as isize) as isize) as std::ffi::c_int;
                            x >= v
                        }) {
                            break;
                        }
                        if x == v {
                            t = *c;
                            *c = *d;
                            *d = t;
                            d = d.offset(-1);
                            d;
                        }
                    }
                }
                if a <= d {
                    c = b.offset(-(1 as std::ffi::c_int as isize));
                    s = a.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
                    t = b.offset_from(a) as std::ffi::c_long as std::ffi::c_int;
                    if s > t {
                        s = t;
                    }
                    e = first;
                    f = b.offset(-(s as isize));
                    while (0 as std::ffi::c_int) < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        s;
                        e = e.offset(1);
                        e;
                        f = f.offset(1);
                        f;
                    }
                    s = d.offset_from(c) as std::ffi::c_long as std::ffi::c_int;
                    t = (last.offset_from(d) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long)
                        as std::ffi::c_int;
                    if s > t {
                        s = t;
                    }
                    e = b;
                    f = last.offset(-(s as isize));
                    while (0 as std::ffi::c_int) < s {
                        t = *e;
                        *e = *f;
                        *f = t;
                        s -= 1;
                        s;
                        e = e.offset(1);
                        e;
                        f = f.offset(1);
                        f;
                    }
                    a = first.offset(b.offset_from(a) as std::ffi::c_long as isize);
                    c = last.offset(-(d.offset_from(c) as std::ffi::c_long as isize));
                    b = if v
                        <= *Td.offset((*PA.offset(*a as isize) - 1 as std::ffi::c_int) as isize)
                            as std::ffi::c_int
                    {
                        a
                    } else {
                        ss_partition(PA, a, c, depth)
                    };
                    if a.offset_from(first) as std::ffi::c_long
                        <= last.offset_from(c) as std::ffi::c_long
                    {
                        if last.offset_from(c) as std::ffi::c_long
                            <= c.offset_from(b) as std::ffi::c_long
                        {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    494 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_13548: {
                                if ssize < 16 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        494 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh8 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh8 = b;
                            let ref mut fresh9 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh9 = c;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c =
                                depth + 1 as std::ffi::c_int;
                            let fresh10 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh10 as isize)).d =
                                ss_ilg(c.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    495 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_13454: {
                                if ssize < 16 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        495 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh11 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh11 = c;
                            let ref mut fresh12 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh12 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh13 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh13 as isize)).d = limit;
                            last = a;
                        } else if a.offset_from(first) as std::ffi::c_long
                            <= c.offset_from(b) as std::ffi::c_long
                        {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    498 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_13350: {
                                if ssize < 16 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        498 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh14 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh14 = c;
                            let ref mut fresh15 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh15 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh16 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh16 as isize)).d = limit;
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    499 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_13265: {
                                if ssize < 16 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        499 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh17 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh17 = b;
                            let ref mut fresh18 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh18 = c;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c =
                                depth + 1 as std::ffi::c_int;
                            let fresh19 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh19 as isize)).d =
                                ss_ilg(c.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                            last = a;
                        } else {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    502 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_13164: {
                                if ssize < 16 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        502 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh20 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh20 = c;
                            let ref mut fresh21 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh21 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh22 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh22 as isize)).d = limit;
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    503 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_13079: {
                                if ssize < 16 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        503 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 74],
                                            &[std::ffi::c_char; 74],
                                        >(
                                            b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh23 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh23 = first;
                            let ref mut fresh24 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh24 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                            let fresh25 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh25 as isize)).d = limit;
                            first = b;
                            last = c;
                            depth += 1 as std::ffi::c_int;
                            limit = ss_ilg(c.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                        }
                    } else if a.offset_from(first) as std::ffi::c_long
                        <= c.offset_from(b) as std::ffi::c_long
                    {
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                508 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12947: {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    508 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh26 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh26 = b;
                        let ref mut fresh27 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh27 = c;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c =
                            depth + 1 as std::ffi::c_int;
                        let fresh28 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh28 as isize)).d =
                            ss_ilg(c.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                509 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12853: {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    509 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh29 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh29 = first;
                        let ref mut fresh30 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh30 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh31 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh31 as isize)).d = limit;
                        first = c;
                    } else if last.offset_from(c) as std::ffi::c_long
                        <= c.offset_from(b) as std::ffi::c_long
                    {
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                512 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12749: {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    512 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh32 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh32 = first;
                        let ref mut fresh33 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh33 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh34 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh34 as isize)).d = limit;
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                513 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12664: {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    513 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh35 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh35 = b;
                        let ref mut fresh36 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh36 = c;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c =
                            depth + 1 as std::ffi::c_int;
                        let fresh37 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh37 as isize)).d =
                            ss_ilg(c.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                        first = c;
                    } else {
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                516 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12563: {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    516 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh38 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh38 = first;
                        let ref mut fresh39 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh39 = a;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh40 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh40 as isize)).d = limit;
                        if ssize < 16 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                517 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 74],
                                    &[std::ffi::c_char; 74],
                                >(
                                    b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_12476: {
                            if ssize < 16 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    517 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 74],
                                        &[std::ffi::c_char; 74],
                                    >(
                                        b"void ss_mintrosort(const unsigned char *, const int *, int *, int *, int)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh41 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh41 = c;
                        let ref mut fresh42 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh42 = last;
                        (*stack.as_mut_ptr().offset(ssize as isize)).c = depth;
                        let fresh43 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh43 as isize)).d = limit;
                        first = b;
                        last = c;
                        depth += 1 as std::ffi::c_int;
                        limit = ss_ilg(c.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                    }
                } else {
                    limit += 1 as std::ffi::c_int;
                    if (*Td.offset((*PA.offset(*first as isize) - 1 as std::ffi::c_int) as isize)
                        as std::ffi::c_int)
                        < v
                    {
                        first = ss_partition(PA, first, last, depth);
                        limit =
                            ss_ilg(last.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                    }
                    depth += 1 as std::ffi::c_int;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn ss_blockswap(
    mut a: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
) {
    let mut t: std::ffi::c_int = 0;
    while (0 as std::ffi::c_int) < n {
        t = *a;
        *a = *b;
        *b = t;
        n -= 1;
        n;
        a = a.offset(1);
        a;
        b = b.offset(1);
        b;
    }
}
#[inline]
unsafe extern "C" fn ss_rotate(
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut l: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    l = middle.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
    r = last.offset_from(middle) as std::ffi::c_long as std::ffi::c_int;
    while (0 as std::ffi::c_int) < l && (0 as std::ffi::c_int) < r {
        if l == r {
            ss_blockswap(first, middle, l);
            break;
        } else if l < r {
            a = last.offset(-(1 as std::ffi::c_int as isize));
            b = middle.offset(-(1 as std::ffi::c_int as isize));
            t = *a;
            loop {
                let fresh44 = a;
                a = a.offset(-1);
                *fresh44 = *b;
                let fresh45 = b;
                b = b.offset(-1);
                *fresh45 = *a;
                if !(b < first) {
                    continue;
                }
                *a = t;
                last = a;
                r -= l + 1 as std::ffi::c_int;
                if r <= l {
                    break;
                }
                a = a.offset(-(1 as std::ffi::c_int as isize));
                b = middle.offset(-(1 as std::ffi::c_int as isize));
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
                if !(last <= b) {
                    continue;
                }
                *a = t;
                first = a.offset(1 as std::ffi::c_int as isize);
                l -= r + 1 as std::ffi::c_int;
                if l <= r {
                    break;
                }
                a = a.offset(1 as std::ffi::c_int as isize);
                b = middle;
                t = *a;
            }
        }
    }
}
unsafe extern "C" fn ss_inplacemerge(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut p = 0 as *const std::ffi::c_int;
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut len: std::ffi::c_int = 0;
    let mut half: std::ffi::c_int = 0;
    let mut q: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    loop {
        if *last.offset(-(1 as std::ffi::c_int as isize)) < 0 as std::ffi::c_int {
            x = 1 as std::ffi::c_int;
            p = PA.offset(!*last.offset(-(1 as std::ffi::c_int as isize)) as isize);
        } else {
            x = 0 as std::ffi::c_int;
            p = PA.offset(*last.offset(-(1 as std::ffi::c_int as isize)) as isize);
        }
        a = first;
        len = middle.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
        half = len >> 1 as std::ffi::c_int;
        r = -(1 as std::ffi::c_int);
        while (0 as std::ffi::c_int) < len {
            b = a.offset(half as isize);
            q = ss_compare(
                T,
                PA.offset((if 0 as std::ffi::c_int <= *b { *b } else { !*b }) as isize),
                p,
                depth,
            );
            if q < 0 as std::ffi::c_int {
                a = b.offset(1 as std::ffi::c_int as isize);
                half -= len & 1 as std::ffi::c_int ^ 1 as std::ffi::c_int;
            } else {
                r = q;
            }
            len = half;
            half >>= 1 as std::ffi::c_int;
        }
        if a < middle {
            if r == 0 as std::ffi::c_int {
                *a = !*a;
            }
            ss_rotate(a, middle, last);
            last = last.offset(-(middle.offset_from(a) as std::ffi::c_long as isize));
            middle = a;
            if first == middle {
                break;
            }
        }
        last = last.offset(-1);
        last;
        if x != 0 as std::ffi::c_int {
            loop {
                last = last.offset(-1);
                if !(*last < 0 as std::ffi::c_int) {
                    break;
                }
            }
        }
        if middle == last {
            break;
        }
    }
}
unsafe extern "C" fn ss_mergeforward(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut bufend = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    bufend = buf
        .offset(middle.offset_from(first) as std::ffi::c_long as isize)
        .offset(-(1 as std::ffi::c_int as isize));
    ss_blockswap(
        buf,
        first,
        middle.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
    );
    a = first;
    t = *a;
    b = buf;
    c = middle;
    loop {
        r = ss_compare(T, PA.offset(*b as isize), PA.offset(*c as isize), depth);
        if r < 0 as std::ffi::c_int {
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
                if !(*b < 0 as std::ffi::c_int) {
                    break;
                }
            }
        } else if r > 0 as std::ffi::c_int {
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
                if !(*c < 0 as std::ffi::c_int) {
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
                if !(*b < 0 as std::ffi::c_int) {
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
                if !(*c < 0 as std::ffi::c_int) {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn ss_mergebackward(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut p1 = 0 as *const std::ffi::c_int;
    let mut p2 = 0 as *const std::ffi::c_int;
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut bufend = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    let mut x: std::ffi::c_int = 0;
    bufend = buf
        .offset(last.offset_from(middle) as std::ffi::c_long as isize)
        .offset(-(1 as std::ffi::c_int as isize));
    ss_blockswap(
        buf,
        middle,
        last.offset_from(middle) as std::ffi::c_long as std::ffi::c_int,
    );
    x = 0 as std::ffi::c_int;
    if *bufend < 0 as std::ffi::c_int {
        p1 = PA.offset(!*bufend as isize);
        x |= 1 as std::ffi::c_int;
    } else {
        p1 = PA.offset(*bufend as isize);
    }
    if *middle.offset(-(1 as std::ffi::c_int as isize)) < 0 as std::ffi::c_int {
        p2 = PA.offset(!*middle.offset(-(1 as std::ffi::c_int as isize)) as isize);
        x |= 2 as std::ffi::c_int;
    } else {
        p2 = PA.offset(*middle.offset(-(1 as std::ffi::c_int as isize)) as isize);
    }
    a = last.offset(-(1 as std::ffi::c_int as isize));
    t = *a;
    b = bufend;
    c = middle.offset(-(1 as std::ffi::c_int as isize));
    loop {
        r = ss_compare(T, p1, p2, depth);
        if (0 as std::ffi::c_int) < r {
            if x & 1 as std::ffi::c_int != 0 {
                loop {
                    let fresh60 = a;
                    a = a.offset(-1);
                    *fresh60 = *b;
                    let fresh61 = b;
                    b = b.offset(-1);
                    *fresh61 = *a;
                    if !(*b < 0 as std::ffi::c_int) {
                        break;
                    }
                }
                x ^= 1 as std::ffi::c_int;
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
                if *b < 0 as std::ffi::c_int {
                    p1 = PA.offset(!*b as isize);
                    x |= 1 as std::ffi::c_int;
                } else {
                    p1 = PA.offset(*b as isize);
                }
            }
        } else if r < 0 as std::ffi::c_int {
            if x & 2 as std::ffi::c_int != 0 {
                loop {
                    let fresh64 = a;
                    a = a.offset(-1);
                    *fresh64 = *c;
                    let fresh65 = c;
                    c = c.offset(-1);
                    *fresh65 = *a;
                    if !(*c < 0 as std::ffi::c_int) {
                        break;
                    }
                }
                x ^= 2 as std::ffi::c_int;
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
            } else if *c < 0 as std::ffi::c_int {
                p2 = PA.offset(!*c as isize);
                x |= 2 as std::ffi::c_int;
            } else {
                p2 = PA.offset(*c as isize);
            }
        } else {
            if x & 1 as std::ffi::c_int != 0 {
                loop {
                    let fresh70 = a;
                    a = a.offset(-1);
                    *fresh70 = *b;
                    let fresh71 = b;
                    b = b.offset(-1);
                    *fresh71 = *a;
                    if !(*b < 0 as std::ffi::c_int) {
                        break;
                    }
                }
                x ^= 1 as std::ffi::c_int;
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
                if x & 2 as std::ffi::c_int != 0 {
                    loop {
                        let fresh74 = a;
                        a = a.offset(-1);
                        *fresh74 = *c;
                        let fresh75 = c;
                        c = c.offset(-1);
                        *fresh75 = *a;
                        if !(*c < 0 as std::ffi::c_int) {
                            break;
                        }
                    }
                    x ^= 2 as std::ffi::c_int;
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
                    if *b < 0 as std::ffi::c_int {
                        p1 = PA.offset(!*b as isize);
                        x |= 1 as std::ffi::c_int;
                    } else {
                        p1 = PA.offset(*b as isize);
                    }
                    if *c < 0 as std::ffi::c_int {
                        p2 = PA.offset(!*c as isize);
                        x |= 2 as std::ffi::c_int;
                    } else {
                        p2 = PA.offset(*c as isize);
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn ss_swapmerge(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut bufsize: std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut stack: [C2RustUnnamed_1; 32] = [C2RustUnnamed_1 {
        a: 0 as *mut std::ffi::c_int,
        b: 0 as *mut std::ffi::c_int,
        c: 0 as *mut std::ffi::c_int,
        d: 0,
    }; 32];
    let mut l = 0 as *mut std::ffi::c_int;
    let mut r = 0 as *mut std::ffi::c_int;
    let mut lm = 0 as *mut std::ffi::c_int;
    let mut rm = 0 as *mut std::ffi::c_int;
    let mut m: std::ffi::c_int = 0;
    let mut len: std::ffi::c_int = 0;
    let mut half: std::ffi::c_int = 0;
    let mut ssize: std::ffi::c_int = 0;
    let mut check: std::ffi::c_int = 0;
    let mut next: std::ffi::c_int = 0;
    check = 0 as std::ffi::c_int;
    ssize = 0 as std::ffi::c_int;
    loop {
        if last.offset_from(middle) as std::ffi::c_long <= bufsize as std::ffi::c_long {
            if first < middle && middle < last {
                ss_mergebackward(T, PA, first, middle, last, buf, depth);
            }
            if check & 1 as std::ffi::c_int != 0
                || check & 2 as std::ffi::c_int != 0
                    && ss_compare(
                        T,
                        PA.offset(
                            (if 0 as std::ffi::c_int
                                <= *first.offset(-(1 as std::ffi::c_int as isize))
                            {
                                *first.offset(-(1 as std::ffi::c_int as isize))
                            } else {
                                !*first.offset(-(1 as std::ffi::c_int as isize))
                            }) as isize,
                        ),
                        PA.offset(*first as isize),
                        depth,
                    ) == 0 as std::ffi::c_int
            {
                *first = !*first;
            }
            if check & 4 as std::ffi::c_int != 0
                && ss_compare(
                    T,
                    PA.offset(
                        (if 0 as std::ffi::c_int <= *last.offset(-(1 as std::ffi::c_int as isize)) {
                            *last.offset(-(1 as std::ffi::c_int as isize))
                        } else {
                            !*last.offset(-(1 as std::ffi::c_int as isize))
                        }) as isize,
                    ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0 as std::ffi::c_int
            {
                *last = !*last;
            }
            if 0 as std::ffi::c_int <= ssize {
            } else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    771 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[std::ffi::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_17752: {
                if 0 as std::ffi::c_int <= ssize {
                } else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        771 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[std::ffi::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if ssize == 0 as std::ffi::c_int {
                return;
            }
            ssize -= 1;
            first = (*stack.as_mut_ptr().offset(ssize as isize)).a;
            middle = (*stack.as_mut_ptr().offset(ssize as isize)).b;
            last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
            check = (*stack.as_mut_ptr().offset(ssize as isize)).d;
        } else if middle.offset_from(first) as std::ffi::c_long <= bufsize as std::ffi::c_long {
            if first < middle {
                ss_mergeforward(T, PA, first, middle, last, buf, depth);
            }
            if check & 1 as std::ffi::c_int != 0
                || check & 2 as std::ffi::c_int != 0
                    && ss_compare(
                        T,
                        PA.offset(
                            (if 0 as std::ffi::c_int
                                <= *first.offset(-(1 as std::ffi::c_int as isize))
                            {
                                *first.offset(-(1 as std::ffi::c_int as isize))
                            } else {
                                !*first.offset(-(1 as std::ffi::c_int as isize))
                            }) as isize,
                        ),
                        PA.offset(*first as isize),
                        depth,
                    ) == 0 as std::ffi::c_int
            {
                *first = !*first;
            }
            if check & 4 as std::ffi::c_int != 0
                && ss_compare(
                    T,
                    PA.offset(
                        (if 0 as std::ffi::c_int <= *last.offset(-(1 as std::ffi::c_int as isize)) {
                            *last.offset(-(1 as std::ffi::c_int as isize))
                        } else {
                            !*last.offset(-(1 as std::ffi::c_int as isize))
                        }) as isize,
                    ),
                    PA.offset(*last as isize),
                    depth,
                ) == 0 as std::ffi::c_int
            {
                *last = !*last;
            }
            if 0 as std::ffi::c_int <= ssize {
            } else {
                __assert_fail(
                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    780 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 92],
                        &[std::ffi::c_char; 92],
                    >(
                        b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_17111: {
                if 0 as std::ffi::c_int <= ssize {
                } else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        780 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[std::ffi::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if ssize == 0 as std::ffi::c_int {
                return;
            }
            ssize -= 1;
            first = (*stack.as_mut_ptr().offset(ssize as isize)).a;
            middle = (*stack.as_mut_ptr().offset(ssize as isize)).b;
            last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
            check = (*stack.as_mut_ptr().offset(ssize as isize)).d;
        } else {
            m = 0 as std::ffi::c_int;
            len = (if (middle.offset_from(first) as std::ffi::c_long)
                < last.offset_from(middle) as std::ffi::c_long
            {
                middle.offset_from(first) as std::ffi::c_long
            } else {
                last.offset_from(middle) as std::ffi::c_long
            }) as std::ffi::c_int;
            half = len >> 1 as std::ffi::c_int;
            while (0 as std::ffi::c_int) < len {
                if ss_compare(
                    T,
                    PA.offset(
                        (if 0 as std::ffi::c_int <= *middle.offset(m as isize).offset(half as isize)
                        {
                            *middle.offset(m as isize).offset(half as isize)
                        } else {
                            !*middle.offset(m as isize).offset(half as isize)
                        }) as isize,
                    ),
                    PA.offset(
                        (if 0 as std::ffi::c_int
                            <= *middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .offset(-(1 as std::ffi::c_int as isize))
                        {
                            *middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .offset(-(1 as std::ffi::c_int as isize))
                        } else {
                            !*middle
                                .offset(-(m as isize))
                                .offset(-(half as isize))
                                .offset(-(1 as std::ffi::c_int as isize))
                        }) as isize,
                    ),
                    depth,
                ) < 0 as std::ffi::c_int
                {
                    m += half + 1 as std::ffi::c_int;
                    half -= len & 1 as std::ffi::c_int ^ 1 as std::ffi::c_int;
                }
                len = half;
                half >>= 1 as std::ffi::c_int;
            }
            if (0 as std::ffi::c_int) < m {
                lm = middle.offset(-(m as isize));
                rm = middle.offset(m as isize);
                ss_blockswap(lm, middle, m);
                r = middle;
                l = r;
                next = 0 as std::ffi::c_int;
                if rm < last {
                    if *rm < 0 as std::ffi::c_int {
                        *rm = !*rm;
                        if first < lm {
                            loop {
                                l = l.offset(-1);
                                if !(*l < 0 as std::ffi::c_int) {
                                    break;
                                }
                            }
                            next |= 4 as std::ffi::c_int;
                        }
                        next |= 1 as std::ffi::c_int;
                    } else if first < lm {
                        while *r < 0 as std::ffi::c_int {
                            r = r.offset(1);
                            r;
                        }
                        next |= 2 as std::ffi::c_int;
                    }
                }
                if l.offset_from(first) as std::ffi::c_long
                    <= last.offset_from(r) as std::ffi::c_long
                {
                    if ssize < 32 as std::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            810 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[std::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_16710: {
                        if ssize < 32 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                810 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 92],
                                    &[std::ffi::c_char; 92],
                                >(
                                    b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let ref mut fresh80 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh80 = r;
                    let ref mut fresh81 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh81 = rm;
                    let ref mut fresh82 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh82 = last;
                    let fresh83 = ssize;
                    ssize = ssize + 1;
                    (*stack.as_mut_ptr().offset(fresh83 as isize)).d =
                        next & 3 as std::ffi::c_int | check & 4 as std::ffi::c_int;
                    middle = lm;
                    last = l;
                    check = check & 3 as std::ffi::c_int | next & 4 as std::ffi::c_int;
                } else {
                    if next & 2 as std::ffi::c_int != 0 && r == middle {
                        next ^= 6 as std::ffi::c_int;
                    }
                    if ssize < 32 as std::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            814 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[std::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_16574: {
                        if ssize < 32 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                814 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 92],
                                    &[std::ffi::c_char; 92],
                                >(
                                    b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let ref mut fresh84 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh84 = first;
                    let ref mut fresh85 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh85 = lm;
                    let ref mut fresh86 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh86 = l;
                    let fresh87 = ssize;
                    ssize = ssize + 1;
                    (*stack.as_mut_ptr().offset(fresh87 as isize)).d =
                        check & 3 as std::ffi::c_int | next & 4 as std::ffi::c_int;
                    first = r;
                    middle = rm;
                    check = next & 3 as std::ffi::c_int | check & 4 as std::ffi::c_int;
                }
            } else {
                if ss_compare(
                    T,
                    PA.offset(
                        (if 0 as std::ffi::c_int <= *middle.offset(-(1 as std::ffi::c_int as isize))
                        {
                            *middle.offset(-(1 as std::ffi::c_int as isize))
                        } else {
                            !*middle.offset(-(1 as std::ffi::c_int as isize))
                        }) as isize,
                    ),
                    PA.offset(*middle as isize),
                    depth,
                ) == 0 as std::ffi::c_int
                {
                    *middle = !*middle;
                }
                if check & 1 as std::ffi::c_int != 0
                    || check & 2 as std::ffi::c_int != 0
                        && ss_compare(
                            T,
                            PA.offset(
                                (if 0 as std::ffi::c_int
                                    <= *first.offset(-(1 as std::ffi::c_int as isize))
                                {
                                    *first.offset(-(1 as std::ffi::c_int as isize))
                                } else {
                                    !*first.offset(-(1 as std::ffi::c_int as isize))
                                }) as isize,
                            ),
                            PA.offset(*first as isize),
                            depth,
                        ) == 0 as std::ffi::c_int
                {
                    *first = !*first;
                }
                if check & 4 as std::ffi::c_int != 0
                    && ss_compare(
                        T,
                        PA.offset(
                            (if 0 as std::ffi::c_int
                                <= *last.offset(-(1 as std::ffi::c_int as isize))
                            {
                                *last.offset(-(1 as std::ffi::c_int as isize))
                            } else {
                                !*last.offset(-(1 as std::ffi::c_int as isize))
                            }) as isize,
                        ),
                        PA.offset(*last as isize),
                        depth,
                    ) == 0 as std::ffi::c_int
                {
                    *last = !*last;
                }
                if 0 as std::ffi::c_int <= ssize {
                } else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        822 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 92],
                            &[std::ffi::c_char; 92],
                        >(
                            b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_16217: {
                    if 0 as std::ffi::c_int <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            822 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 92],
                                &[std::ffi::c_char; 92],
                            >(
                                b"void ss_swapmerge(const unsigned char *, const int *, int *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if ssize == 0 as std::ffi::c_int {
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
unsafe extern "C" fn sssort(
    mut T: *const std::ffi::c_uchar,
    mut PA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut buf: *mut std::ffi::c_int,
    mut bufsize: std::ffi::c_int,
    mut depth: std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut lastsuffix: std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut middle = 0 as *mut std::ffi::c_int;
    let mut curbuf = 0 as *mut std::ffi::c_int;
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut curbufsize: std::ffi::c_int = 0;
    let mut limit: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    if lastsuffix != 0 as std::ffi::c_int {
        first = first.offset(1);
        first;
    }
    if bufsize < SS_BLOCKSIZE
        && (bufsize as std::ffi::c_long) < last.offset_from(first) as std::ffi::c_long
        && {
            limit = ss_isqrt(last.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
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
        limit = 0 as std::ffi::c_int;
    }
    a = first;
    i = 0 as std::ffi::c_int;
    while (SS_BLOCKSIZE as std::ffi::c_long) < middle.offset_from(a) as std::ffi::c_long {
        ss_mintrosort(T, PA, a, a.offset(SS_BLOCKSIZE as isize), depth);
        curbufsize = last.offset_from(a.offset(SS_BLOCKSIZE as isize)) as std::ffi::c_long
            as std::ffi::c_int;
        curbuf = a.offset(SS_BLOCKSIZE as isize);
        if curbufsize <= bufsize {
            curbufsize = bufsize;
            curbuf = buf;
        }
        b = a;
        k = SS_BLOCKSIZE;
        j = i;
        while j & 1 as std::ffi::c_int != 0 {
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
            k <<= 1 as std::ffi::c_int;
            j >>= 1 as std::ffi::c_int;
        }
        a = a.offset(SS_BLOCKSIZE as isize);
        i += 1;
        i;
    }
    ss_mintrosort(T, PA, a, middle, depth);
    k = SS_BLOCKSIZE;
    while i != 0 as std::ffi::c_int {
        if i & 1 as std::ffi::c_int != 0 {
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
        k <<= 1 as std::ffi::c_int;
        i >>= 1 as std::ffi::c_int;
    }
    if limit != 0 as std::ffi::c_int {
        ss_mintrosort(T, PA, middle, last, depth);
        ss_inplacemerge(T, PA, first, middle, last, depth);
    }
    if lastsuffix != 0 as std::ffi::c_int {
        let mut PAi: [std::ffi::c_int; 2] = [0; 2];
        *PAi.as_mut_ptr().offset(0 as std::ffi::c_int as isize) =
            *PA.offset(*first.offset(-(1 as std::ffi::c_int as isize)) as isize);
        *PAi.as_mut_ptr().offset(1 as std::ffi::c_int as isize) = n - 2 as std::ffi::c_int;
        a = first;
        i = *first.offset(-(1 as std::ffi::c_int as isize));
        while a < last
            && (*a < 0 as std::ffi::c_int
                || (0 as std::ffi::c_int)
                    < ss_compare(
                        T,
                        &mut *PAi.as_mut_ptr().offset(0 as std::ffi::c_int as isize),
                        PA.offset(*a as isize),
                        depth,
                    ))
        {
            *a.offset(-(1 as std::ffi::c_int as isize)) = *a;
            a = a.offset(1);
            a;
        }
        *a.offset(-(1 as std::ffi::c_int as isize)) = i;
    }
}
#[inline]
unsafe extern "C" fn tr_ilg(mut n: std::ffi::c_int) -> std::ffi::c_int {
    return if n as std::ffi::c_uint & 0xffff0000 as std::ffi::c_uint != 0 {
        if n as std::ffi::c_uint & 0xff000000 as std::ffi::c_uint != 0 {
            24 as std::ffi::c_int
                + *lg_table
                    .as_ptr()
                    .offset((n >> 24 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
        } else {
            16 as std::ffi::c_int
                + *lg_table
                    .as_ptr()
                    .offset((n >> 16 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
        }
    } else if n & 0xff00 as std::ffi::c_int != 0 {
        8 as std::ffi::c_int
            + *lg_table
                .as_ptr()
                .offset((n >> 8 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
    } else {
        0 as std::ffi::c_int
            + *lg_table
                .as_ptr()
                .offset((n >> 0 as std::ffi::c_int & 0xff as std::ffi::c_int) as isize)
    };
}
unsafe extern "C" fn tr_insertionsort(
    mut ISAd: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut r: std::ffi::c_int = 0;
    a = first.offset(1 as std::ffi::c_int as isize);
    while a < last {
        t = *a;
        b = a.offset(-(1 as std::ffi::c_int as isize));
        loop {
            r = *ISAd.offset(t as isize) - *ISAd.offset(*b as isize);
            if !(0 as std::ffi::c_int > r) {
                break;
            }
            loop {
                *b.offset(1 as std::ffi::c_int as isize) = *b;
                b = b.offset(-1);
                if !(first <= b && *b < 0 as std::ffi::c_int) {
                    break;
                }
            }
            if b < first {
                break;
            }
        }
        if r == 0 as std::ffi::c_int {
            *b = !*b;
        }
        *b.offset(1 as std::ffi::c_int as isize) = t;
        a = a.offset(1);
        a;
    }
}
#[inline]
unsafe extern "C" fn tr_fixdown(
    mut ISAd: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut i: std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut c: std::ffi::c_int = 0;
    let mut d: std::ffi::c_int = 0;
    let mut e: std::ffi::c_int = 0;
    v = *SA.offset(i as isize);
    c = *ISAd.offset(v as isize);
    loop {
        j = 2 as std::ffi::c_int * i + 1 as std::ffi::c_int;
        if !(j < size) {
            break;
        }
        let fresh88 = j;
        j = j + 1;
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
unsafe extern "C" fn tr_heapsort(
    mut ISAd: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut size: std::ffi::c_int,
) {
    let mut i: std::ffi::c_int = 0;
    let mut m: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    m = size;
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        m -= 1;
        m;
        if *ISAd.offset(*SA.offset((m / 2 as std::ffi::c_int) as isize) as isize)
            < *ISAd.offset(*SA.offset(m as isize) as isize)
        {
            t = *SA.offset(m as isize);
            *SA.offset(m as isize) = *SA.offset((m / 2 as std::ffi::c_int) as isize);
            *SA.offset((m / 2 as std::ffi::c_int) as isize) = t;
        }
    }
    i = m / 2 as std::ffi::c_int - 1 as std::ffi::c_int;
    while 0 as std::ffi::c_int <= i {
        tr_fixdown(ISAd, SA, i, m);
        i -= 1;
        i;
    }
    if size % 2 as std::ffi::c_int == 0 as std::ffi::c_int {
        t = *SA.offset(0 as std::ffi::c_int as isize);
        *SA.offset(0 as std::ffi::c_int as isize) = *SA.offset(m as isize);
        *SA.offset(m as isize) = t;
        tr_fixdown(ISAd, SA, 0 as std::ffi::c_int, m);
    }
    i = m - 1 as std::ffi::c_int;
    while (0 as std::ffi::c_int) < i {
        t = *SA.offset(0 as std::ffi::c_int as isize);
        *SA.offset(0 as std::ffi::c_int as isize) = *SA.offset(i as isize);
        tr_fixdown(ISAd, SA, 0 as std::ffi::c_int, i);
        *SA.offset(i as isize) = t;
        i -= 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn tr_median3(
    mut ISAd: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
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
    return v2;
}
#[inline]
unsafe extern "C" fn tr_median5(
    mut ISAd: *const std::ffi::c_int,
    mut v1: *mut std::ffi::c_int,
    mut v2: *mut std::ffi::c_int,
    mut v3: *mut std::ffi::c_int,
    mut v4: *mut std::ffi::c_int,
    mut v5: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut t = 0 as *mut std::ffi::c_int;
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
    return v3;
}
#[inline]
unsafe extern "C" fn tr_pivot(
    mut ISAd: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
) -> *mut std::ffi::c_int {
    let mut middle = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    t = last.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
    middle = first.offset((t / 2 as std::ffi::c_int) as isize);
    if t <= 512 as std::ffi::c_int {
        if t <= 32 as std::ffi::c_int {
            return tr_median3(
                ISAd,
                first,
                middle,
                last.offset(-(1 as std::ffi::c_int as isize)),
            );
        } else {
            t >>= 2 as std::ffi::c_int;
            return tr_median5(
                ISAd,
                first,
                first.offset(t as isize),
                middle,
                last.offset(-(1 as std::ffi::c_int as isize))
                    .offset(-(t as isize)),
                last.offset(-(1 as std::ffi::c_int as isize)),
            );
        }
    }
    t >>= 3 as std::ffi::c_int;
    first = tr_median3(
        ISAd,
        first,
        first.offset(t as isize),
        first.offset((t << 1 as std::ffi::c_int) as isize),
    );
    middle = tr_median3(
        ISAd,
        middle.offset(-(t as isize)),
        middle,
        middle.offset(t as isize),
    );
    last = tr_median3(
        ISAd,
        last.offset(-(1 as std::ffi::c_int as isize))
            .offset(-((t << 1 as std::ffi::c_int) as isize)),
        last.offset(-(1 as std::ffi::c_int as isize))
            .offset(-(t as isize)),
        last.offset(-(1 as std::ffi::c_int as isize)),
    );
    return tr_median3(ISAd, first, middle, last);
}
#[inline]
unsafe extern "C" fn trbudget_init(
    mut budget: *mut trbudget_t,
    mut chance: std::ffi::c_int,
    mut incval: std::ffi::c_int,
) {
    (*budget).chance = chance;
    (*budget).incval = incval;
    (*budget).remain = (*budget).incval;
}
#[inline]
unsafe extern "C" fn trbudget_check(
    mut budget: *mut trbudget_t,
    mut size: std::ffi::c_int,
) -> std::ffi::c_int {
    if size <= (*budget).remain {
        (*budget).remain -= size;
        return 1 as std::ffi::c_int;
    }
    if (*budget).chance == 0 as std::ffi::c_int {
        (*budget).count += size;
        return 0 as std::ffi::c_int;
    }
    (*budget).remain += (*budget).incval - size;
    (*budget).chance -= 1 as std::ffi::c_int;
    return 1 as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn tr_partition(
    mut ISAd: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut middle: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut pa: *mut *mut std::ffi::c_int,
    mut pb: *mut *mut std::ffi::c_int,
    mut v: std::ffi::c_int,
) {
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut f = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut s: std::ffi::c_int = 0;
    let mut x = 0 as std::ffi::c_int;
    b = middle.offset(-(1 as std::ffi::c_int as isize));
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
                a;
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
                d;
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
                a;
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
                d;
            }
        }
    }
    if a <= d {
        c = b.offset(-(1 as std::ffi::c_int as isize));
        s = a.offset_from(first) as std::ffi::c_long as std::ffi::c_int;
        t = b.offset_from(a) as std::ffi::c_long as std::ffi::c_int;
        if s > t {
            s = t;
        }
        e = first;
        f = b.offset(-(s as isize));
        while (0 as std::ffi::c_int) < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            s;
            e = e.offset(1);
            e;
            f = f.offset(1);
            f;
        }
        s = d.offset_from(c) as std::ffi::c_long as std::ffi::c_int;
        t = (last.offset_from(d) as std::ffi::c_long - 1 as std::ffi::c_int as std::ffi::c_long)
            as std::ffi::c_int;
        if s > t {
            s = t;
        }
        e = b;
        f = last.offset(-(s as isize));
        while (0 as std::ffi::c_int) < s {
            t = *e;
            *e = *f;
            *f = t;
            s -= 1;
            s;
            e = e.offset(1);
            e;
            f = f.offset(1);
            f;
        }
        first = first.offset(b.offset_from(a) as std::ffi::c_long as isize);
        last = last.offset(-(d.offset_from(c) as std::ffi::c_long as isize));
    }
    *pa = first;
    *pb = last;
}
unsafe extern "C" fn tr_copy(
    mut ISA: *mut std::ffi::c_int,
    mut SA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut a: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    v = (b.offset_from(SA) as std::ffi::c_long - 1 as std::ffi::c_int as std::ffi::c_long)
        as std::ffi::c_int;
    c = first;
    d = a.offset(-(1 as std::ffi::c_int as isize));
    while c <= d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        c = c.offset(1);
        c;
    }
    c = last.offset(-(1 as std::ffi::c_int as isize));
    e = d.offset(1 as std::ffi::c_int as isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            *ISA.offset(s as isize) = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        c = c.offset(-1);
        c;
    }
}
unsafe extern "C" fn tr_partialcopy(
    mut ISA: *mut std::ffi::c_int,
    mut SA: *const std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut a: *mut std::ffi::c_int,
    mut b: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut c = 0 as *mut std::ffi::c_int;
    let mut d = 0 as *mut std::ffi::c_int;
    let mut e = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut rank: std::ffi::c_int = 0;
    let mut lastrank: std::ffi::c_int = 0;
    let mut newrank = -(1 as std::ffi::c_int);
    v = (b.offset_from(SA) as std::ffi::c_long - 1 as std::ffi::c_int as std::ffi::c_long)
        as std::ffi::c_int;
    lastrank = -(1 as std::ffi::c_int);
    c = first;
    d = a.offset(-(1 as std::ffi::c_int as isize));
    while c <= d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(1);
        c;
    }
    lastrank = -(1 as std::ffi::c_int);
    e = d;
    while first <= e {
        rank = *ISA.offset(*e as isize);
        if lastrank != rank {
            lastrank = rank;
            newrank = e.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        if newrank != rank {
            *ISA.offset(*e as isize) = newrank;
        }
        e = e.offset(-1);
        e;
    }
    lastrank = -(1 as std::ffi::c_int);
    c = last.offset(-(1 as std::ffi::c_int as isize));
    e = d.offset(1 as std::ffi::c_int as isize);
    d = b;
    while e < d {
        s = *c - depth;
        if 0 as std::ffi::c_int <= s && *ISA.offset(s as isize) == v {
            d = d.offset(-1);
            *d = s;
            rank = *ISA.offset((s + depth) as isize);
            if lastrank != rank {
                lastrank = rank;
                newrank = d.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
            }
            *ISA.offset(s as isize) = newrank;
        }
        c = c.offset(-1);
        c;
    }
}
unsafe extern "C" fn tr_introsort(
    mut ISA: *mut std::ffi::c_int,
    mut ISAd: *const std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut first: *mut std::ffi::c_int,
    mut last: *mut std::ffi::c_int,
    mut budget: *mut trbudget_t,
) {
    let mut stack: [C2RustUnnamed; 64] = [C2RustUnnamed {
        a: 0 as *const std::ffi::c_int,
        b: 0 as *mut std::ffi::c_int,
        c: 0 as *mut std::ffi::c_int,
        d: 0,
        e: 0,
    }; 64];
    let mut a = 0 as *mut std::ffi::c_int;
    let mut b = 0 as *mut std::ffi::c_int;
    let mut c = 0 as *mut std::ffi::c_int;
    let mut t: std::ffi::c_int = 0;
    let mut v: std::ffi::c_int = 0;
    let mut x = 0 as std::ffi::c_int;
    let mut incr = ISAd.offset_from(ISA) as std::ffi::c_long as std::ffi::c_int;
    let mut limit: std::ffi::c_int = 0;
    let mut next: std::ffi::c_int = 0;
    let mut ssize: std::ffi::c_int = 0;
    let mut trlink = -(1 as std::ffi::c_int);
    ssize = 0 as std::ffi::c_int;
    limit = tr_ilg(last.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
    loop {
        if limit < 0 as std::ffi::c_int {
            if limit == -(1 as std::ffi::c_int) {
                tr_partition(
                    ISAd.offset(-(incr as isize)),
                    first,
                    first,
                    last,
                    &mut a,
                    &mut b,
                    (last.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long)
                        as std::ffi::c_int,
                );
                if a < last {
                    c = first;
                    v = (a.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long)
                        as std::ffi::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                        c;
                    }
                }
                if b < last {
                    c = a;
                    v = (b.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long)
                        as std::ffi::c_int;
                    while c < b {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                        c;
                    }
                }
                if (1 as std::ffi::c_int as std::ffi::c_long) < b.offset_from(a) as std::ffi::c_long
                {
                    if ssize < 64 as std::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1204 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_10711: {
                        if ssize < 64 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1204 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let ref mut fresh89 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh89 = 0 as *const std::ffi::c_int;
                    let ref mut fresh90 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh90 = a;
                    let ref mut fresh91 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh91 = b;
                    (*stack.as_mut_ptr().offset(ssize as isize)).d = 0 as std::ffi::c_int;
                    let fresh92 = ssize;
                    ssize = ssize + 1;
                    (*stack.as_mut_ptr().offset(fresh92 as isize)).e = 0 as std::ffi::c_int;
                    if ssize < 64 as std::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1205 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_10615: {
                        if ssize < 64 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1205 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let ref mut fresh93 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh93 = ISAd.offset(-(incr as isize));
                    let ref mut fresh94 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh94 = first;
                    let ref mut fresh95 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh95 = last;
                    (*stack.as_mut_ptr().offset(ssize as isize)).d = -(2 as std::ffi::c_int);
                    let fresh96 = ssize;
                    ssize = ssize + 1;
                    (*stack.as_mut_ptr().offset(fresh96 as isize)).e = trlink;
                    trlink = ssize - 2 as std::ffi::c_int;
                }
                if a.offset_from(first) as std::ffi::c_long
                    <= last.offset_from(b) as std::ffi::c_long
                {
                    if (1 as std::ffi::c_int as std::ffi::c_long)
                        < a.offset_from(first) as std::ffi::c_long
                    {
                        if ssize < 64 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1210 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_10485: {
                            if ssize < 64 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1210 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh97 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        *fresh97 = ISAd;
                        let ref mut fresh98 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        *fresh98 = b;
                        let ref mut fresh99 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                        *fresh99 = last;
                        (*stack.as_mut_ptr().offset(ssize as isize)).d =
                            tr_ilg(last.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                        let fresh100 = ssize;
                        ssize = ssize + 1;
                        (*stack.as_mut_ptr().offset(fresh100 as isize)).e = trlink;
                        last = a;
                        limit = tr_ilg(a.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                    } else if (1 as std::ffi::c_int as std::ffi::c_long)
                        < last.offset_from(b) as std::ffi::c_long
                    {
                        first = b;
                        limit = tr_ilg(last.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                    } else {
                        if 0 as std::ffi::c_int <= ssize {
                        } else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1215 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_10341: {
                            if 0 as std::ffi::c_int <= ssize {
                            } else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1215 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        if ssize == 0 as std::ffi::c_int {
                            return;
                        }
                        ssize -= 1;
                        ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                        first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                        last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                        limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                        trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                    }
                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                    < last.offset_from(b) as std::ffi::c_long
                {
                    if ssize < 64 as std::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"ssize < STACK_SIZE\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1219 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_10222: {
                        if ssize < 64 as std::ffi::c_int {
                        } else {
                            __assert_fail(
                                b"ssize < STACK_SIZE\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1219 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let ref mut fresh101 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    *fresh101 = ISAd;
                    let ref mut fresh102 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    *fresh102 = first;
                    let ref mut fresh103 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    *fresh103 = a;
                    (*stack.as_mut_ptr().offset(ssize as isize)).d =
                        tr_ilg(a.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                    let fresh104 = ssize;
                    ssize = ssize + 1;
                    (*stack.as_mut_ptr().offset(fresh104 as isize)).e = trlink;
                    first = b;
                    limit = tr_ilg(last.offset_from(b) as std::ffi::c_long as std::ffi::c_int);
                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                    < a.offset_from(first) as std::ffi::c_long
                {
                    last = a;
                    limit = tr_ilg(a.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                } else {
                    if 0 as std::ffi::c_int <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1224 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_10078: {
                        if 0 as std::ffi::c_int <= ssize {
                        } else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1224 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if ssize == 0 as std::ffi::c_int {
                        return;
                    }
                    ssize -= 1;
                    ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                    first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                    last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                    limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                    trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                }
            } else if limit == -(2 as std::ffi::c_int) {
                ssize -= 1;
                a = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                b = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                if (*stack.as_mut_ptr().offset(ssize as isize)).d == 0 as std::ffi::c_int {
                    tr_copy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as std::ffi::c_long as std::ffi::c_int,
                    );
                } else {
                    if 0 as std::ffi::c_int <= trlink {
                        (*stack.as_mut_ptr().offset(trlink as isize)).d = -(1 as std::ffi::c_int);
                    }
                    tr_partialcopy(
                        ISA,
                        SA,
                        first,
                        a,
                        b,
                        last,
                        ISAd.offset_from(ISA) as std::ffi::c_long as std::ffi::c_int,
                    );
                }
                if 0 as std::ffi::c_int <= ssize {
                } else {
                    __assert_fail(
                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1236 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 73],
                            &[std::ffi::c_char; 73],
                        >(
                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_9382: {
                    if 0 as std::ffi::c_int <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1236 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if ssize == 0 as std::ffi::c_int {
                    return;
                }
                ssize -= 1;
                ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
            } else {
                if 0 as std::ffi::c_int <= *first {
                    a = first;
                    loop {
                        *ISA.offset(*a as isize) =
                            a.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                        a = a.offset(1);
                        if !(a < last && 0 as std::ffi::c_int <= *a) {
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
                        if !(*a < 0 as std::ffi::c_int) {
                            break;
                        }
                    }
                    next = if *ISA.offset(*a as isize) != *ISAd.offset(*a as isize) {
                        tr_ilg(
                            (a.offset_from(first) as std::ffi::c_long
                                + 1 as std::ffi::c_int as std::ffi::c_long)
                                as std::ffi::c_int,
                        )
                    } else {
                        -(1 as std::ffi::c_int)
                    };
                    a = a.offset(1);
                    if a < last {
                        b = first;
                        v = (a.offset_from(SA) as std::ffi::c_long
                            - 1 as std::ffi::c_int as std::ffi::c_long)
                            as std::ffi::c_int;
                        while b < a {
                            *ISA.offset(*b as isize) = v;
                            b = b.offset(1);
                            b;
                        }
                    }
                    if trbudget_check(
                        budget,
                        a.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                    ) != 0
                    {
                        if a.offset_from(first) as std::ffi::c_long
                            <= last.offset_from(a) as std::ffi::c_long
                        {
                            if ssize < 64 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1252 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_9103: {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1252 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh105 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh105 = ISAd;
                            let ref mut fresh106 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh106 = a;
                            let ref mut fresh107 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh107 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d =
                                -(3 as std::ffi::c_int);
                            let fresh108 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh108 as isize)).e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        } else if (1 as std::ffi::c_int as std::ffi::c_long)
                            < last.offset_from(a) as std::ffi::c_long
                        {
                            if ssize < 64 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1256 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_8983: {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1256 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh109 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh109 = ISAd.offset(incr as isize);
                            let ref mut fresh110 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh110 = first;
                            let ref mut fresh111 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh111 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                            let fresh112 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh112 as isize)).e = trlink;
                            first = a;
                            limit = -(3 as std::ffi::c_int);
                        } else {
                            ISAd = ISAd.offset(incr as isize);
                            last = a;
                            limit = next;
                        }
                    } else {
                        if 0 as std::ffi::c_int <= trlink {
                            (*stack.as_mut_ptr().offset(trlink as isize)).d =
                                -(1 as std::ffi::c_int);
                        }
                        if (1 as std::ffi::c_int as std::ffi::c_long)
                            < last.offset_from(a) as std::ffi::c_long
                        {
                            first = a;
                            limit = -(3 as std::ffi::c_int);
                        } else {
                            if 0 as std::ffi::c_int <= ssize {
                            } else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1267 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_8819: {
                                if 0 as std::ffi::c_int <= ssize {
                                } else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1267 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if ssize == 0 as std::ffi::c_int {
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
                    if 0 as std::ffi::c_int <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1271 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_8704: {
                        if 0 as std::ffi::c_int <= ssize {
                        } else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1271 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if ssize == 0 as std::ffi::c_int {
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
        } else if last.offset_from(first) as std::ffi::c_long
            <= TR_INSERTIONSORT_THRESHOLD as std::ffi::c_long
        {
            tr_insertionsort(ISAd, first, last);
            limit = -(3 as std::ffi::c_int);
        } else {
            let fresh113 = limit;
            limit = limit - 1;
            if fresh113 == 0 as std::ffi::c_int {
                tr_heapsort(
                    ISAd,
                    first,
                    last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                );
                a = last.offset(-(1 as std::ffi::c_int as isize));
                while first < a {
                    x = *ISAd.offset(*a as isize);
                    b = a.offset(-(1 as std::ffi::c_int as isize));
                    while first <= b && *ISAd.offset(*b as isize) == x {
                        *b = !*b;
                        b = b.offset(-1);
                        b;
                    }
                    a = b;
                }
                limit = -(3 as std::ffi::c_int);
            } else {
                a = tr_pivot(ISAd, first, last);
                t = *first;
                *first = *a;
                *a = t;
                v = *ISAd.offset(*first as isize);
                tr_partition(
                    ISAd,
                    first,
                    first.offset(1 as std::ffi::c_int as isize),
                    last,
                    &mut a,
                    &mut b,
                    v,
                );
                if last.offset_from(first) as std::ffi::c_long
                    != b.offset_from(a) as std::ffi::c_long
                {
                    next = if *ISA.offset(*a as isize) != v {
                        tr_ilg(b.offset_from(a) as std::ffi::c_long as std::ffi::c_int)
                    } else {
                        -(1 as std::ffi::c_int)
                    };
                    c = first;
                    v = (a.offset_from(SA) as std::ffi::c_long
                        - 1 as std::ffi::c_int as std::ffi::c_long)
                        as std::ffi::c_int;
                    while c < a {
                        *ISA.offset(*c as isize) = v;
                        c = c.offset(1);
                        c;
                    }
                    if b < last {
                        c = a;
                        v = (b.offset_from(SA) as std::ffi::c_long
                            - 1 as std::ffi::c_int as std::ffi::c_long)
                            as std::ffi::c_int;
                        while c < b {
                            *ISA.offset(*c as isize) = v;
                            c = c.offset(1);
                            c;
                        }
                    }
                    if (1 as std::ffi::c_int as std::ffi::c_long)
                        < b.offset_from(a) as std::ffi::c_long
                        && trbudget_check(
                            budget,
                            b.offset_from(a) as std::ffi::c_long as std::ffi::c_int,
                        ) != 0
                    {
                        if a.offset_from(first) as std::ffi::c_long
                            <= last.offset_from(b) as std::ffi::c_long
                        {
                            if last.offset_from(b) as std::ffi::c_long
                                <= b.offset_from(a) as std::ffi::c_long
                            {
                                if (1 as std::ffi::c_int as std::ffi::c_long)
                                    < a.offset_from(first) as std::ffi::c_long
                                {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1311 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_6540: {
                                        if ssize < 64 as std::ffi::c_int {
                                        } else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1311 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    let ref mut fresh114 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh114 = ISAd.offset(incr as isize);
                                    let ref mut fresh115 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh115 = a;
                                    let ref mut fresh116 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh116 = b;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                    let fresh117 = ssize;
                                    ssize = ssize + 1;
                                    (*stack.as_mut_ptr().offset(fresh117 as isize)).e = trlink;
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1312 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_6441: {
                                        if ssize < 64 as std::ffi::c_int {
                                        } else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1312 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    let ref mut fresh118 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh118 = ISAd;
                                    let ref mut fresh119 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh119 = b;
                                    let ref mut fresh120 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh120 = last;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                    let fresh121 = ssize;
                                    ssize = ssize + 1;
                                    (*stack.as_mut_ptr().offset(fresh121 as isize)).e = trlink;
                                    last = a;
                                } else if (1 as std::ffi::c_int as std::ffi::c_long)
                                    < last.offset_from(b) as std::ffi::c_long
                                {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1315 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_6330: {
                                        if ssize < 64 as std::ffi::c_int {
                                        } else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1315 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    let ref mut fresh122 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh122 = ISAd.offset(incr as isize);
                                    let ref mut fresh123 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh123 = a;
                                    let ref mut fresh124 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh124 = b;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                    let fresh125 = ssize;
                                    ssize = ssize + 1;
                                    (*stack.as_mut_ptr().offset(fresh125 as isize)).e = trlink;
                                    first = b;
                                } else {
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else if a.offset_from(first) as std::ffi::c_long
                                <= b.offset_from(a) as std::ffi::c_long
                            {
                                if (1 as std::ffi::c_int as std::ffi::c_long)
                                    < a.offset_from(first) as std::ffi::c_long
                                {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1322 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_6178: {
                                        if ssize < 64 as std::ffi::c_int {
                                        } else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1322 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    let ref mut fresh126 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh126 = ISAd;
                                    let ref mut fresh127 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh127 = b;
                                    let ref mut fresh128 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh128 = last;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                    let fresh129 = ssize;
                                    ssize = ssize + 1;
                                    (*stack.as_mut_ptr().offset(fresh129 as isize)).e = trlink;
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1323 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_6082: {
                                        if ssize < 64 as std::ffi::c_int {
                                        } else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1323 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    let ref mut fresh130 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh130 = ISAd.offset(incr as isize);
                                    let ref mut fresh131 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh131 = a;
                                    let ref mut fresh132 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh132 = b;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                    let fresh133 = ssize;
                                    ssize = ssize + 1;
                                    (*stack.as_mut_ptr().offset(fresh133 as isize)).e = trlink;
                                    last = a;
                                } else {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1326 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_5976: {
                                        if ssize < 64 as std::ffi::c_int {
                                        } else {
                                            __assert_fail(
                                                b"ssize < STACK_SIZE\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                    as *const u8 as *const std::ffi::c_char,
                                                1326 as std::ffi::c_int as std::ffi::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 73],
                                                    &[std::ffi::c_char; 73],
                                                >(
                                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    let ref mut fresh134 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                    *fresh134 = ISAd;
                                    let ref mut fresh135 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                    *fresh135 = b;
                                    let ref mut fresh136 =
                                        (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                    *fresh136 = last;
                                    (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                    let fresh137 = ssize;
                                    ssize = ssize + 1;
                                    (*stack.as_mut_ptr().offset(fresh137 as isize)).e = trlink;
                                    ISAd = ISAd.offset(incr as isize);
                                    first = a;
                                    last = b;
                                    limit = next;
                                }
                            } else {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1330 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5855: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1330 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh138 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh138 = ISAd;
                                let ref mut fresh139 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh139 = b;
                                let ref mut fresh140 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh140 = last;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh141 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh141 as isize)).e = trlink;
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1331 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5759: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1331 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh142 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh142 = ISAd;
                                let ref mut fresh143 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh143 = first;
                                let ref mut fresh144 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh144 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh145 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh145 as isize)).e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if a.offset_from(first) as std::ffi::c_long
                            <= b.offset_from(a) as std::ffi::c_long
                        {
                            if (1 as std::ffi::c_int as std::ffi::c_long)
                                < last.offset_from(b) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1337 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5614: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1337 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh146 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh146 = ISAd.offset(incr as isize);
                                let ref mut fresh147 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh147 = a;
                                let ref mut fresh148 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh148 = b;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                let fresh149 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh149 as isize)).e = trlink;
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1338 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5515: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1338 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh150 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh150 = ISAd;
                                let ref mut fresh151 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh151 = first;
                                let ref mut fresh152 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh152 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh153 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh153 as isize)).e = trlink;
                                first = b;
                            } else if (1 as std::ffi::c_int as std::ffi::c_long)
                                < a.offset_from(first) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1341 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5404: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1341 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh154 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh154 = ISAd.offset(incr as isize);
                                let ref mut fresh155 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh155 = a;
                                let ref mut fresh156 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh156 = b;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                let fresh157 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh157 as isize)).e = trlink;
                                last = a;
                            } else {
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else if last.offset_from(b) as std::ffi::c_long
                            <= b.offset_from(a) as std::ffi::c_long
                        {
                            if (1 as std::ffi::c_int as std::ffi::c_long)
                                < last.offset_from(b) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1348 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5252: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1348 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh158 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh158 = ISAd;
                                let ref mut fresh159 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh159 = first;
                                let ref mut fresh160 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh160 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh161 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh161 as isize)).e = trlink;
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1349 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5156: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1349 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh162 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh162 = ISAd.offset(incr as isize);
                                let ref mut fresh163 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh163 = a;
                                let ref mut fresh164 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh164 = b;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = next;
                                let fresh165 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh165 as isize)).e = trlink;
                                first = b;
                            } else {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1352 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_5050: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1352 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh166 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh166 = ISAd;
                                let ref mut fresh167 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh167 = first;
                                let ref mut fresh168 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh168 = a;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh169 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh169 as isize)).e = trlink;
                                ISAd = ISAd.offset(incr as isize);
                                first = a;
                                last = b;
                                limit = next;
                            }
                        } else {
                            if ssize < 64 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1356 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_4929: {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1356 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh170 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh170 = ISAd;
                            let ref mut fresh171 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh171 = first;
                            let ref mut fresh172 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh172 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                            let fresh173 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh173 as isize)).e = trlink;
                            if ssize < 64 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1357 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_4833: {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1357 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh174 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh174 = ISAd;
                            let ref mut fresh175 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh175 = b;
                            let ref mut fresh176 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh176 = last;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                            let fresh177 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh177 as isize)).e = trlink;
                            ISAd = ISAd.offset(incr as isize);
                            first = a;
                            last = b;
                            limit = next;
                        }
                    } else {
                        if (1 as std::ffi::c_int as std::ffi::c_long)
                            < b.offset_from(a) as std::ffi::c_long
                            && 0 as std::ffi::c_int <= trlink
                        {
                            (*stack.as_mut_ptr().offset(trlink as isize)).d =
                                -(1 as std::ffi::c_int);
                        }
                        if a.offset_from(first) as std::ffi::c_long
                            <= last.offset_from(b) as std::ffi::c_long
                        {
                            if (1 as std::ffi::c_int as std::ffi::c_long)
                                < a.offset_from(first) as std::ffi::c_long
                            {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1365 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4656: {
                                    if ssize < 64 as std::ffi::c_int {
                                    } else {
                                        __assert_fail(
                                            b"ssize < STACK_SIZE\0" as *const u8
                                                as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1365 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                let ref mut fresh178 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                *fresh178 = ISAd;
                                let ref mut fresh179 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                *fresh179 = b;
                                let ref mut fresh180 =
                                    (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                *fresh180 = last;
                                (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                                let fresh181 = ssize;
                                ssize = ssize + 1;
                                (*stack.as_mut_ptr().offset(fresh181 as isize)).e = trlink;
                                last = a;
                            } else if (1 as std::ffi::c_int as std::ffi::c_long)
                                < last.offset_from(b) as std::ffi::c_long
                            {
                                first = b;
                            } else {
                                if 0 as std::ffi::c_int <= ssize {
                                } else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1370 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                                'c_4543: {
                                    if 0 as std::ffi::c_int <= ssize {
                                    } else {
                                        __assert_fail(
                                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                                as *const u8 as *const std::ffi::c_char,
                                            1370 as std::ffi::c_int as std::ffi::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 73],
                                                &[std::ffi::c_char; 73],
                                            >(
                                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                if ssize == 0 as std::ffi::c_int {
                                    return;
                                }
                                ssize -= 1;
                                ISAd = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                                first = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                                last = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                                limit = (*stack.as_mut_ptr().offset(ssize as isize)).d;
                                trlink = (*stack.as_mut_ptr().offset(ssize as isize)).e;
                            }
                        } else if (1 as std::ffi::c_int as std::ffi::c_long)
                            < last.offset_from(b) as std::ffi::c_long
                        {
                            if ssize < 64 as std::ffi::c_int {
                            } else {
                                __assert_fail(
                                    b"ssize < STACK_SIZE\0" as *const u8
                                        as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1374 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_4423: {
                                if ssize < 64 as std::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"ssize < STACK_SIZE\0" as *const u8
                                            as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1374 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            let ref mut fresh182 = (*stack.as_mut_ptr().offset(ssize as isize)).a;
                            *fresh182 = ISAd;
                            let ref mut fresh183 = (*stack.as_mut_ptr().offset(ssize as isize)).b;
                            *fresh183 = first;
                            let ref mut fresh184 = (*stack.as_mut_ptr().offset(ssize as isize)).c;
                            *fresh184 = a;
                            (*stack.as_mut_ptr().offset(ssize as isize)).d = limit;
                            let fresh185 = ssize;
                            ssize = ssize + 1;
                            (*stack.as_mut_ptr().offset(fresh185 as isize)).e = trlink;
                            first = b;
                        } else if (1 as std::ffi::c_int as std::ffi::c_long)
                            < a.offset_from(first) as std::ffi::c_long
                        {
                            last = a;
                        } else {
                            if 0 as std::ffi::c_int <= ssize {
                            } else {
                                __assert_fail(
                                    b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                        as *const u8 as *const std::ffi::c_char,
                                    1379 as std::ffi::c_int as std::ffi::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 73],
                                        &[std::ffi::c_char; 73],
                                    >(
                                        b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                            'c_4308: {
                                if 0 as std::ffi::c_int <= ssize {
                                } else {
                                    __assert_fail(
                                        b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                            as *const u8 as *const std::ffi::c_char,
                                        1379 as std::ffi::c_int as std::ffi::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 73],
                                            &[std::ffi::c_char; 73],
                                        >(
                                            b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                        ))
                                            .as_ptr(),
                                    );
                                }
                            };
                            if ssize == 0 as std::ffi::c_int {
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
                    last.offset_from(first) as std::ffi::c_long as std::ffi::c_int,
                ) != 0
                {
                    limit = tr_ilg(last.offset_from(first) as std::ffi::c_long as std::ffi::c_int);
                    ISAd = ISAd.offset(incr as isize);
                } else {
                    if 0 as std::ffi::c_int <= trlink {
                        (*stack.as_mut_ptr().offset(trlink as isize)).d = -(1 as std::ffi::c_int);
                    }
                    if 0 as std::ffi::c_int <= ssize {
                    } else {
                        __assert_fail(
                            b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1388 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 73],
                                &[std::ffi::c_char; 73],
                            >(
                                b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_3736: {
                        if 0 as std::ffi::c_int <= ssize {
                        } else {
                            __assert_fail(
                                b"0 <= ssize\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1388 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 73],
                                    &[std::ffi::c_char; 73],
                                >(
                                    b"void tr_introsort(int *, const int *, int *, int *, int *, trbudget_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if ssize == 0 as std::ffi::c_int {
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
unsafe extern "C" fn trsort(
    mut ISA: *mut std::ffi::c_int,
    mut SA: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut depth: std::ffi::c_int,
) {
    let mut ISAd = 0 as *mut std::ffi::c_int;
    let mut first = 0 as *mut std::ffi::c_int;
    let mut last = 0 as *mut std::ffi::c_int;
    let mut budget = _trbudget_t {
        chance: 0,
        remain: 0,
        incval: 0,
        count: 0,
    };
    let mut t: std::ffi::c_int = 0;
    let mut skip: std::ffi::c_int = 0;
    let mut unsorted: std::ffi::c_int = 0;
    trbudget_init(
        &mut budget,
        tr_ilg(n) * 2 as std::ffi::c_int / 3 as std::ffi::c_int,
        n,
    );
    ISAd = ISA.offset(depth as isize);
    while -n < *SA {
        first = SA;
        skip = 0 as std::ffi::c_int;
        unsorted = 0 as std::ffi::c_int;
        loop {
            t = *first;
            if t < 0 as std::ffi::c_int {
                first = first.offset(-(t as isize));
                skip += t;
            } else {
                if skip != 0 as std::ffi::c_int {
                    *first.offset(skip as isize) = skip;
                    skip = 0 as std::ffi::c_int;
                }
                last = SA
                    .offset(*ISA.offset(t as isize) as isize)
                    .offset(1 as std::ffi::c_int as isize);
                if (1 as std::ffi::c_int as std::ffi::c_long)
                    < last.offset_from(first) as std::ffi::c_long
                {
                    budget.count = 0 as std::ffi::c_int;
                    tr_introsort(ISA, ISAd, SA, first, last, &mut budget);
                    if budget.count != 0 as std::ffi::c_int {
                        unsorted += budget.count;
                    } else {
                        skip = first.offset_from(last) as std::ffi::c_long as std::ffi::c_int;
                    }
                } else if last.offset_from(first) as std::ffi::c_long
                    == 1 as std::ffi::c_int as std::ffi::c_long
                {
                    skip = -(1 as std::ffi::c_int);
                }
                first = last;
            }
            if !(first < SA.offset(n as isize)) {
                break;
            }
        }
        if skip != 0 as std::ffi::c_int {
            *first.offset(skip as isize) = skip;
        }
        if unsorted == 0 as std::ffi::c_int {
            break;
        }
        ISAd = ISAd.offset(ISAd.offset_from(ISA) as std::ffi::c_long as isize);
    }
}
unsafe extern "C" fn sort_typeBstar(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut openMP: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut PAb = 0 as *mut std::ffi::c_int;
    let mut ISAb = 0 as *mut std::ffi::c_int;
    let mut buf = 0 as *mut std::ffi::c_int;
    let mut i: std::ffi::c_int = 0;
    let mut j: std::ffi::c_int = 0;
    let mut k: std::ffi::c_int = 0;
    let mut t: std::ffi::c_int = 0;
    let mut m: std::ffi::c_int = 0;
    let mut bufsize: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    i = 0 as std::ffi::c_int;
    while i < BUCKET_A_SIZE {
        *bucket_A.offset(i as isize) = 0 as std::ffi::c_int;
        i += 1;
        i;
    }
    i = 0 as std::ffi::c_int;
    while i < BUCKET_B_SIZE {
        *bucket_B.offset(i as isize) = 0 as std::ffi::c_int;
        i += 1;
        i;
    }
    i = n - 1 as std::ffi::c_int;
    m = n;
    c0 = *T.offset((n - 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
    while 0 as std::ffi::c_int <= i {
        loop {
            c1 = c0;
            let ref mut fresh186 = *bucket_A.offset(c1 as isize);
            *fresh186 += 1;
            *fresh186;
            i -= 1;
            if !(0 as std::ffi::c_int <= i && {
                c0 = *T.offset(i as isize) as std::ffi::c_int;
                c0 >= c1
            }) {
                break;
            }
        }
        if 0 as std::ffi::c_int <= i {
            let ref mut fresh187 = *bucket_B.offset((c0 << 8 as std::ffi::c_int | c1) as isize);
            *fresh187 += 1;
            *fresh187;
            m -= 1;
            *SA.offset(m as isize) = i;
            i -= 1;
            i;
            c1 = c0;
            while 0 as std::ffi::c_int <= i && {
                c0 = *T.offset(i as isize) as std::ffi::c_int;
                c0 <= c1
            } {
                let ref mut fresh188 = *bucket_B.offset((c1 << 8 as std::ffi::c_int | c0) as isize);
                *fresh188 += 1;
                *fresh188;
                i -= 1;
                i;
                c1 = c0;
            }
        }
    }
    m = n - m;
    c0 = 0 as std::ffi::c_int;
    i = 0 as std::ffi::c_int;
    j = 0 as std::ffi::c_int;
    while c0 < ALPHABET_SIZE {
        t = i + *bucket_A.offset(c0 as isize);
        *bucket_A.offset(c0 as isize) = i + j;
        i = t + *bucket_B.offset((c0 << 8 as std::ffi::c_int | c0) as isize);
        c1 = c0 + 1 as std::ffi::c_int;
        while c1 < ALPHABET_SIZE {
            j += *bucket_B.offset((c0 << 8 as std::ffi::c_int | c1) as isize);
            *bucket_B.offset((c0 << 8 as std::ffi::c_int | c1) as isize) = j;
            i += *bucket_B.offset((c1 << 8 as std::ffi::c_int | c0) as isize);
            c1 += 1;
            c1;
        }
        c0 += 1;
        c0;
    }
    if (0 as std::ffi::c_int) < m {
        PAb = SA.offset(n as isize).offset(-(m as isize));
        ISAb = SA.offset(m as isize);
        i = m - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= i {
            t = *PAb.offset(i as isize);
            c0 = *T.offset(t as isize) as std::ffi::c_int;
            c1 = *T.offset((t + 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
            let ref mut fresh189 = *bucket_B.offset((c0 << 8 as std::ffi::c_int | c1) as isize);
            *fresh189 -= 1;
            *SA.offset(*fresh189 as isize) = i;
            i -= 1;
            i;
        }
        t = *PAb.offset((m - 1 as std::ffi::c_int) as isize);
        c0 = *T.offset(t as isize) as std::ffi::c_int;
        c1 = *T.offset((t + 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
        let ref mut fresh190 = *bucket_B.offset((c0 << 8 as std::ffi::c_int | c1) as isize);
        *fresh190 -= 1;
        *SA.offset(*fresh190 as isize) = m - 1 as std::ffi::c_int;
        buf = SA.offset(m as isize);
        bufsize = n - 2 as std::ffi::c_int * m;
        c0 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        j = m;
        while (0 as std::ffi::c_int) < j {
            c1 = ALPHABET_SIZE - 1 as std::ffi::c_int;
            while c0 < c1 {
                i = *bucket_B.offset((c0 << 8 as std::ffi::c_int | c1) as isize);
                if (1 as std::ffi::c_int) < j - i {
                    sssort(
                        T,
                        PAb,
                        SA.offset(i as isize),
                        SA.offset(j as isize),
                        buf,
                        bufsize,
                        2 as std::ffi::c_int,
                        n,
                        (*SA.offset(i as isize) == m - 1 as std::ffi::c_int) as std::ffi::c_int,
                    );
                }
                j = i;
                c1 -= 1;
                c1;
            }
            c0 -= 1;
            c0;
        }
        i = m - 1 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= i {
            if 0 as std::ffi::c_int <= *SA.offset(i as isize) {
                j = i;
                loop {
                    *ISAb.offset(*SA.offset(i as isize) as isize) = i;
                    i -= 1;
                    if !(0 as std::ffi::c_int <= i
                        && 0 as std::ffi::c_int <= *SA.offset(i as isize))
                    {
                        break;
                    }
                }
                *SA.offset((i + 1 as std::ffi::c_int) as isize) = i - j;
                if i <= 0 as std::ffi::c_int {
                    break;
                }
            }
            j = i;
            loop {
                let ref mut fresh191 = *SA.offset(i as isize);
                *fresh191 = !*SA.offset(i as isize);
                *ISAb.offset(*fresh191 as isize) = j;
                i -= 1;
                if !(*SA.offset(i as isize) < 0 as std::ffi::c_int) {
                    break;
                }
            }
            *ISAb.offset(*SA.offset(i as isize) as isize) = j;
            i -= 1;
            i;
        }
        trsort(ISAb, SA, m, 1 as std::ffi::c_int);
        i = n - 1 as std::ffi::c_int;
        j = m;
        c0 = *T.offset((n - 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
        while 0 as std::ffi::c_int <= i {
            i -= 1;
            i;
            c1 = c0;
            while 0 as std::ffi::c_int <= i && {
                c0 = *T.offset(i as isize) as std::ffi::c_int;
                c0 >= c1
            } {
                i -= 1;
                i;
                c1 = c0;
            }
            if 0 as std::ffi::c_int <= i {
                t = i;
                i -= 1;
                i;
                c1 = c0;
                while 0 as std::ffi::c_int <= i && {
                    c0 = *T.offset(i as isize) as std::ffi::c_int;
                    c0 <= c1
                } {
                    i -= 1;
                    i;
                    c1 = c0;
                }
                j -= 1;
                *SA.offset(*ISAb.offset(j as isize) as isize) =
                    if t == 0 as std::ffi::c_int || (1 as std::ffi::c_int) < t - i {
                        t
                    } else {
                        !t
                    };
            }
        }
        *bucket_B.offset(
            ((256 as std::ffi::c_int - 1 as std::ffi::c_int) << 8 as std::ffi::c_int
                | 256 as std::ffi::c_int - 1 as std::ffi::c_int) as isize,
        ) = n;
        c0 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        k = m - 1 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c0 {
            i = *bucket_A.offset((c0 + 1 as std::ffi::c_int) as isize) - 1 as std::ffi::c_int;
            c1 = ALPHABET_SIZE - 1 as std::ffi::c_int;
            while c0 < c1 {
                t = i - *bucket_B.offset((c1 << 8 as std::ffi::c_int | c0) as isize);
                *bucket_B.offset((c1 << 8 as std::ffi::c_int | c0) as isize) = i;
                i = t;
                j = *bucket_B.offset((c0 << 8 as std::ffi::c_int | c1) as isize);
                while j <= k {
                    *SA.offset(i as isize) = *SA.offset(k as isize);
                    i -= 1;
                    i;
                    k -= 1;
                    k;
                }
                c1 -= 1;
                c1;
            }
            *bucket_B.offset((c0 << 8 as std::ffi::c_int | c0 + 1 as std::ffi::c_int) as isize) = i
                - *bucket_B.offset((c0 << 8 as std::ffi::c_int | c0) as isize)
                + 1 as std::ffi::c_int;
            *bucket_B.offset((c0 << 8 as std::ffi::c_int | c0) as isize) = i;
            c0 -= 1;
            c0;
        }
    }
    return m;
}
unsafe extern "C" fn construct_SA(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut m: std::ffi::c_int,
) {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut k = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    let mut c2: std::ffi::c_int = 0;
    if (0 as std::ffi::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c1 {
            i = SA.offset(
                *bucket_B.offset((c1 << 8 as std::ffi::c_int | c1 + 1 as std::ffi::c_int) as isize)
                    as isize,
            );
            j = SA
                .offset(*bucket_A.offset((c1 + 1 as std::ffi::c_int) as isize) as isize)
                .offset(-(1 as std::ffi::c_int as isize));
            k = NULL as *mut std::ffi::c_int;
            c2 = -(1 as std::ffi::c_int);
            while i <= j {
                s = *j;
                if (0 as std::ffi::c_int) < s {
                    if *T.offset(s as isize) as std::ffi::c_int == c1 {
                    } else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1630 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2912: {
                        if *T.offset(s as isize) as std::ffi::c_int == c1 {
                        } else {
                            __assert_fail(
                                b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1630 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (s + 1 as std::ffi::c_int) < n
                        && *T.offset(s as isize) as std::ffi::c_int
                            <= *T.offset((s + 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1631 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2827: {
                        if (s + 1 as std::ffi::c_int) < n
                            && *T.offset(s as isize) as std::ffi::c_int
                                <= *T.offset((s + 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1631 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                        <= *T.offset(s as isize) as std::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1632 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2765: {
                        if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                            <= *T.offset(s as isize) as std::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"T[s - 1] <= T[s]\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1632 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    *j = !s;
                    s -= 1;
                    c0 = *T.offset(s as isize) as std::ffi::c_int;
                    if (0 as std::ffi::c_int) < s
                        && *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as std::ffi::c_int <= c2 {
                            *bucket_B.offset((c1 << 8 as std::ffi::c_int | c2) as isize) =
                                k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                        }
                        c2 = c0;
                        k = SA
                            .offset(*bucket_B.offset((c1 << 8 as std::ffi::c_int | c2) as isize)
                                as isize);
                    }
                    if k < j {
                    } else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1640 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2627: {
                        if k < j {
                        } else {
                            __assert_fail(
                                b"k < j\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1640 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if !k.is_null() {
                    } else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1640 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2584: {
                        if !k.is_null() {
                        } else {
                            __assert_fail(
                                b"k != NULL\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1640 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let fresh192 = k;
                    k = k.offset(-1);
                    *fresh192 = s;
                } else {
                    if s == 0 as std::ffi::c_int && *T.offset(s as isize) as std::ffi::c_int == c1
                        || s < 0 as std::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"((s == 0) && (T[s] == c1)) || (s < 0)\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1643 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_2490: {
                        if s == 0 as std::ffi::c_int
                            && *T.offset(s as isize) as std::ffi::c_int == c1
                            || s < 0 as std::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"((s == 0) && (T[s] == c1)) || (s < 0)\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1643 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    *j = !s;
                }
                j = j.offset(-1);
                j;
            }
            c1 -= 1;
            c1;
        }
    }
    c2 = *T.offset((n - 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh193 = k;
    k = k.offset(1);
    *fresh193 = if (*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int) < c2 {
        !(n - 1 as std::ffi::c_int)
    } else {
        n - 1 as std::ffi::c_int
    };
    i = SA;
    j = SA.offset(n as isize);
    while i < j {
        s = *i;
        if (0 as std::ffi::c_int) < s {
            if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                >= *T.offset(s as isize) as std::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    1657 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[std::ffi::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2326: {
                if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                    >= *T.offset(s as isize) as std::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"T[s - 1] >= T[s]\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1657 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[std::ffi::c_char; 72],
                        >(
                            b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            s -= 1;
            c0 = *T.offset(s as isize) as std::ffi::c_int;
            if s == 0 as std::ffi::c_int
                || (*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int) < c0
            {
                s = !s;
            }
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {
            } else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    1664 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[std::ffi::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2215: {
                if i < k {
                } else {
                    __assert_fail(
                        b"i < k\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1664 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[std::ffi::c_char; 72],
                        >(
                            b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            let fresh194 = k;
            k = k.offset(1);
            *fresh194 = s;
        } else {
            if s < 0 as std::ffi::c_int {
            } else {
                __assert_fail(
                    b"s < 0\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    1667 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[std::ffi::c_char; 72],
                    >(
                        b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2161: {
                if s < 0 as std::ffi::c_int {
                } else {
                    __assert_fail(
                        b"s < 0\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1667 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[std::ffi::c_char; 72],
                        >(
                            b"void construct_SA(const unsigned char *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            *i = !s;
        }
        i = i.offset(1);
        i;
    }
}
unsafe extern "C" fn construct_BWT(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut m: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut k = 0 as *mut std::ffi::c_int;
    let mut orig = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    let mut c2: std::ffi::c_int = 0;
    if (0 as std::ffi::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c1 {
            i = SA.offset(
                *bucket_B.offset((c1 << 8 as std::ffi::c_int | c1 + 1 as std::ffi::c_int) as isize)
                    as isize,
            );
            j = SA
                .offset(*bucket_A.offset((c1 + 1 as std::ffi::c_int) as isize) as isize)
                .offset(-(1 as std::ffi::c_int as isize));
            k = NULL as *mut std::ffi::c_int;
            c2 = -(1 as std::ffi::c_int);
            while i <= j {
                s = *j;
                if (0 as std::ffi::c_int) < s {
                    if *T.offset(s as isize) as std::ffi::c_int == c1 {
                    } else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1694 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_22144: {
                        if *T.offset(s as isize) as std::ffi::c_int == c1 {
                        } else {
                            __assert_fail(
                                b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1694 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (s + 1 as std::ffi::c_int) < n
                        && *T.offset(s as isize) as std::ffi::c_int
                            <= *T.offset((s + 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1695 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_22060: {
                        if (s + 1 as std::ffi::c_int) < n
                            && *T.offset(s as isize) as std::ffi::c_int
                                <= *T.offset((s + 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1695 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                        <= *T.offset(s as isize) as std::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1696 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21998: {
                        if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                            <= *T.offset(s as isize) as std::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"T[s - 1] <= T[s]\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1696 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    s -= 1;
                    c0 = *T.offset(s as isize) as std::ffi::c_int;
                    *j = !c0;
                    if (0 as std::ffi::c_int) < s
                        && *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as std::ffi::c_int <= c2 {
                            *bucket_B.offset((c1 << 8 as std::ffi::c_int | c2) as isize) =
                                k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                        }
                        c2 = c0;
                        k = SA
                            .offset(*bucket_B.offset((c1 << 8 as std::ffi::c_int | c2) as isize)
                                as isize);
                    }
                    if k < j {
                    } else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1704 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21859: {
                        if k < j {
                        } else {
                            __assert_fail(
                                b"k < j\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1704 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if !k.is_null() {
                    } else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1704 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21817: {
                        if !k.is_null() {
                        } else {
                            __assert_fail(
                                b"k != NULL\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1704 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let fresh195 = k;
                    k = k.offset(-1);
                    *fresh195 = s;
                } else if s != 0 as std::ffi::c_int {
                    *j = !s;
                } else {
                    if *T.offset(s as isize) as std::ffi::c_int == c1 {
                    } else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1710 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 72],
                                &[std::ffi::c_char; 72],
                            >(
                                b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21741: {
                        if *T.offset(s as isize) as std::ffi::c_int == c1 {
                        } else {
                            __assert_fail(
                                b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1710 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 72],
                                    &[std::ffi::c_char; 72],
                                >(
                                    b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                j = j.offset(-1);
                j;
            }
            c1 -= 1;
            c1;
        }
    }
    c2 = *T.offset((n - 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    let fresh196 = k;
    k = k.offset(1);
    *fresh196 = if (*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int) < c2 {
        !(*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int)
    } else {
        n - 1 as std::ffi::c_int
    };
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0 as std::ffi::c_int) < s {
            if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                >= *T.offset(s as isize) as std::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    1724 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[std::ffi::c_char; 72],
                    >(
                        b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_21573: {
                if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                    >= *T.offset(s as isize) as std::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"T[s - 1] >= T[s]\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1724 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[std::ffi::c_char; 72],
                        >(
                            b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            s -= 1;
            c0 = *T.offset(s as isize) as std::ffi::c_int;
            *i = c0;
            if (0 as std::ffi::c_int) < s
                && (*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int) < c0
            {
                s = !(*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int);
            }
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {
            } else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    1732 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[std::ffi::c_char; 72],
                    >(
                        b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_21449: {
                if i < k {
                } else {
                    __assert_fail(
                        b"i < k\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1732 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 72],
                            &[std::ffi::c_char; 72],
                        >(
                            b"int construct_BWT(const unsigned char *, int *, int *, int *, int, int)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            let fresh197 = k;
            k = k.offset(1);
            *fresh197 = s;
        } else if s != 0 as std::ffi::c_int {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
        i;
    }
    return orig.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
}
unsafe extern "C" fn construct_BWT_indexes(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut bucket_A: *mut std::ffi::c_int,
    mut bucket_B: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut m: std::ffi::c_int,
    mut num_indexes: *mut std::ffi::c_uchar,
    mut indexes: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    let mut i = 0 as *mut std::ffi::c_int;
    let mut j = 0 as *mut std::ffi::c_int;
    let mut k = 0 as *mut std::ffi::c_int;
    let mut orig = 0 as *mut std::ffi::c_int;
    let mut s: std::ffi::c_int = 0;
    let mut c0: std::ffi::c_int = 0;
    let mut c1: std::ffi::c_int = 0;
    let mut c2: std::ffi::c_int = 0;
    let mut mod_0 = n / 8 as std::ffi::c_int;
    mod_0 |= mod_0 >> 1 as std::ffi::c_int;
    mod_0 |= mod_0 >> 2 as std::ffi::c_int;
    mod_0 |= mod_0 >> 4 as std::ffi::c_int;
    mod_0 |= mod_0 >> 8 as std::ffi::c_int;
    mod_0 |= mod_0 >> 16 as std::ffi::c_int;
    mod_0 >>= 1 as std::ffi::c_int;
    *num_indexes =
        ((n - 1 as std::ffi::c_int) / (mod_0 + 1 as std::ffi::c_int)) as std::ffi::c_uchar;
    if (0 as std::ffi::c_int) < m {
        c1 = ALPHABET_SIZE - 2 as std::ffi::c_int;
        while 0 as std::ffi::c_int <= c1 {
            i = SA.offset(
                *bucket_B.offset((c1 << 8 as std::ffi::c_int | c1 + 1 as std::ffi::c_int) as isize)
                    as isize,
            );
            j = SA
                .offset(*bucket_A.offset((c1 + 1 as std::ffi::c_int) as isize) as isize)
                .offset(-(1 as std::ffi::c_int as isize));
            k = NULL as *mut std::ffi::c_int;
            c2 = -(1 as std::ffi::c_int);
            while i <= j {
                s = *j;
                if (0 as std::ffi::c_int) < s {
                    if *T.offset(s as isize) as std::ffi::c_int == c1 {
                    } else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1775 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[std::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21183: {
                        if *T.offset(s as isize) as std::ffi::c_int == c1 {
                        } else {
                            __assert_fail(
                                b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1775 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 104],
                                    &[std::ffi::c_char; 104],
                                >(
                                    b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (s + 1 as std::ffi::c_int) < n
                        && *T.offset(s as isize) as std::ffi::c_int
                            <= *T.offset((s + 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1776 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[std::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21099: {
                        if (s + 1 as std::ffi::c_int) < n
                            && *T.offset(s as isize) as std::ffi::c_int
                                <= *T.offset((s + 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"((s + 1) < n) && (T[s] <= T[s + 1])\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1776 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 104],
                                    &[std::ffi::c_char; 104],
                                >(
                                    b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                        <= *T.offset(s as isize) as std::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"T[s - 1] <= T[s]\0" as *const u8
                                as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1777 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[std::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_21037: {
                        if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                            <= *T.offset(s as isize) as std::ffi::c_int
                        {
                        } else {
                            __assert_fail(
                                b"T[s - 1] <= T[s]\0" as *const u8
                                    as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1777 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 104],
                                    &[std::ffi::c_char; 104],
                                >(
                                    b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if s & mod_0 == 0 as std::ffi::c_int {
                        *indexes.offset(
                            (s / (mod_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int) as isize,
                        ) = j.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                    }
                    s -= 1;
                    c0 = *T.offset(s as isize) as std::ffi::c_int;
                    *j = !c0;
                    if (0 as std::ffi::c_int) < s
                        && *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int > c0
                    {
                        s = !s;
                    }
                    if c0 != c2 {
                        if 0 as std::ffi::c_int <= c2 {
                            *bucket_B.offset((c1 << 8 as std::ffi::c_int | c2) as isize) =
                                k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                        }
                        c2 = c0;
                        k = SA
                            .offset(*bucket_B.offset((c1 << 8 as std::ffi::c_int | c2) as isize)
                                as isize);
                    }
                    if k < j {
                    } else {
                        __assert_fail(
                            b"k < j\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1788 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[std::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_20869: {
                        if k < j {
                        } else {
                            __assert_fail(
                                b"k < j\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1788 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 104],
                                    &[std::ffi::c_char; 104],
                                >(
                                    b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    if !k.is_null() {
                    } else {
                        __assert_fail(
                            b"k != NULL\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1788 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[std::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_20827: {
                        if !k.is_null() {
                        } else {
                            __assert_fail(
                                b"k != NULL\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1788 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 104],
                                    &[std::ffi::c_char; 104],
                                >(
                                    b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    let fresh198 = k;
                    k = k.offset(-1);
                    *fresh198 = s;
                } else if s != 0 as std::ffi::c_int {
                    *j = !s;
                } else {
                    if *T.offset(s as isize) as std::ffi::c_int == c1 {
                    } else {
                        __assert_fail(
                            b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                            b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                as *const u8 as *const std::ffi::c_char,
                            1794 as std::ffi::c_int as std::ffi::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 104],
                                &[std::ffi::c_char; 104],
                            >(
                                b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_20750: {
                        if *T.offset(s as isize) as std::ffi::c_int == c1 {
                        } else {
                            __assert_fail(
                                b"T[s] == c1\0" as *const u8 as *const std::ffi::c_char,
                                b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0"
                                    as *const u8 as *const std::ffi::c_char,
                                1794 as std::ffi::c_int as std::ffi::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 104],
                                    &[std::ffi::c_char; 104],
                                >(
                                    b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                j = j.offset(-1);
                j;
            }
            c1 -= 1;
            c1;
        }
    }
    c2 = *T.offset((n - 1 as std::ffi::c_int) as isize) as std::ffi::c_int;
    k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
    if (*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int) < c2 {
        if n - 1 as std::ffi::c_int & mod_0 == 0 as std::ffi::c_int {
            *indexes.offset(
                ((n - 1 as std::ffi::c_int) / (mod_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int)
                    as isize,
            ) = k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
        }
        let fresh199 = k;
        k = k.offset(1);
        *fresh199 = !(*T.offset((n - 2 as std::ffi::c_int) as isize) as std::ffi::c_int);
    } else {
        let fresh200 = k;
        k = k.offset(1);
        *fresh200 = n - 1 as std::ffi::c_int;
    }
    i = SA;
    j = SA.offset(n as isize);
    orig = SA;
    while i < j {
        s = *i;
        if (0 as std::ffi::c_int) < s {
            if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                >= *T.offset(s as isize) as std::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"T[s - 1] >= T[s]\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    1815 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[std::ffi::c_char; 104],
                    >(
                        b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_20541: {
                if *T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int
                    >= *T.offset(s as isize) as std::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"T[s - 1] >= T[s]\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1815 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 104],
                            &[std::ffi::c_char; 104],
                        >(
                            b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if s & mod_0 == 0 as std::ffi::c_int {
                *indexes
                    .offset((s / (mod_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int) as isize) =
                    i.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
            }
            s -= 1;
            c0 = *T.offset(s as isize) as std::ffi::c_int;
            *i = c0;
            if c0 != c2 {
                *bucket_A.offset(c2 as isize) =
                    k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                c2 = c0;
                k = SA.offset(*bucket_A.offset(c2 as isize) as isize);
            }
            if i < k {
            } else {
                __assert_fail(
                    b"i < k\0" as *const u8 as *const std::ffi::c_char,
                    b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                        as *const std::ffi::c_char,
                    1825 as std::ffi::c_int as std::ffi::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[std::ffi::c_char; 104],
                    >(
                        b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_20422: {
                if i < k {
                } else {
                    __assert_fail(
                        b"i < k\0" as *const u8 as *const std::ffi::c_char,
                        b"/tmp/zstd-c2rust/lib//dictBuilder/divsufsort.c\0" as *const u8
                            as *const std::ffi::c_char,
                        1825 as std::ffi::c_int as std::ffi::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 104],
                            &[std::ffi::c_char; 104],
                        >(
                            b"int construct_BWT_indexes(const unsigned char *, int *, int *, int *, int, int, unsigned char *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if (0 as std::ffi::c_int) < s
                && (*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int) < c0
            {
                if s & mod_0 == 0 as std::ffi::c_int {
                    *indexes.offset(
                        (s / (mod_0 + 1 as std::ffi::c_int) - 1 as std::ffi::c_int) as isize,
                    ) = k.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
                }
                let fresh201 = k;
                k = k.offset(1);
                *fresh201 = !(*T.offset((s - 1 as std::ffi::c_int) as isize) as std::ffi::c_int);
            } else {
                let fresh202 = k;
                k = k.offset(1);
                *fresh202 = s;
            }
        } else if s != 0 as std::ffi::c_int {
            *i = !s;
        } else {
            orig = i;
        }
        i = i.offset(1);
        i;
    }
    return orig.offset_from(SA) as std::ffi::c_long as std::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn divsufsort(
    mut T: *const std::ffi::c_uchar,
    mut SA: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut openMP: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut bucket_A = 0 as *mut std::ffi::c_int;
    let mut bucket_B = 0 as *mut std::ffi::c_int;
    let mut m: std::ffi::c_int = 0;
    let mut err = 0 as std::ffi::c_int;
    if T.is_null() || SA.is_null() || n < 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    } else if n == 0 as std::ffi::c_int {
        return 0 as std::ffi::c_int;
    } else if n == 1 as std::ffi::c_int {
        *SA.offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int;
        return 0 as std::ffi::c_int;
    } else if n == 2 as std::ffi::c_int {
        m = ((*T.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int)
            < *T.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
            as std::ffi::c_int;
        *SA.offset((m ^ 1 as std::ffi::c_int) as isize) = 0 as std::ffi::c_int;
        *SA.offset(m as isize) = 1 as std::ffi::c_int;
        return 0 as std::ffi::c_int;
    }
    bucket_A = malloc(
        (BUCKET_A_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
    ) as *mut std::ffi::c_int;
    bucket_B = malloc(
        (BUCKET_B_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
    ) as *mut std::ffi::c_int;
    if !bucket_A.is_null() && !bucket_B.is_null() {
        m = sort_typeBstar(T, SA, bucket_A, bucket_B, n, openMP);
        construct_SA(T, SA, bucket_A, bucket_B, n, m);
    } else {
        err = -(2 as std::ffi::c_int);
    }
    free(bucket_B as *mut std::ffi::c_void);
    free(bucket_A as *mut std::ffi::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn divbwt(
    mut T: *const std::ffi::c_uchar,
    mut U: *mut std::ffi::c_uchar,
    mut A: *mut std::ffi::c_int,
    mut n: std::ffi::c_int,
    mut num_indexes: *mut std::ffi::c_uchar,
    mut indexes: *mut std::ffi::c_int,
    mut openMP: std::ffi::c_int,
) -> std::ffi::c_int {
    let mut B = 0 as *mut std::ffi::c_int;
    let mut bucket_A = 0 as *mut std::ffi::c_int;
    let mut bucket_B = 0 as *mut std::ffi::c_int;
    let mut m: std::ffi::c_int = 0;
    let mut pidx: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    if T.is_null() || U.is_null() || n < 0 as std::ffi::c_int {
        return -(1 as std::ffi::c_int);
    } else if n <= 1 as std::ffi::c_int {
        if n == 1 as std::ffi::c_int {
            *U.offset(0 as std::ffi::c_int as isize) = *T.offset(0 as std::ffi::c_int as isize);
        }
        return n;
    }
    B = A;
    if B.is_null() {
        B = malloc(
            ((n + 1 as std::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
        ) as *mut std::ffi::c_int;
    }
    bucket_A = malloc(
        (BUCKET_A_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
    ) as *mut std::ffi::c_int;
    bucket_B = malloc(
        (BUCKET_B_SIZE as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
    ) as *mut std::ffi::c_int;
    if !B.is_null() && !bucket_A.is_null() && !bucket_B.is_null() {
        m = sort_typeBstar(T, B, bucket_A, bucket_B, n, openMP);
        if num_indexes.is_null() || indexes.is_null() {
            pidx = construct_BWT(T, B, bucket_A, bucket_B, n, m);
        } else {
            pidx = construct_BWT_indexes(T, B, bucket_A, bucket_B, n, m, num_indexes, indexes);
        }
        *U.offset(0 as std::ffi::c_int as isize) = *T.offset((n - 1 as std::ffi::c_int) as isize);
        i = 0 as std::ffi::c_int;
        while i < pidx {
            *U.offset((i + 1 as std::ffi::c_int) as isize) =
                *B.offset(i as isize) as std::ffi::c_uchar;
            i += 1;
            i;
        }
        i += 1 as std::ffi::c_int;
        while i < n {
            *U.offset(i as isize) = *B.offset(i as isize) as std::ffi::c_uchar;
            i += 1;
            i;
        }
        pidx += 1 as std::ffi::c_int;
    } else {
        pidx = -(2 as std::ffi::c_int);
    }
    free(bucket_B as *mut std::ffi::c_void);
    free(bucket_A as *mut std::ffi::c_void);
    if A.is_null() {
        free(B as *mut std::ffi::c_void);
    }
    return pidx;
}
