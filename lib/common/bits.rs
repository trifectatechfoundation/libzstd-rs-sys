use crate::lib::common::mem::MEM_isLittleEndian;

#[inline]
pub(crate) const fn ZSTD_NbCommonBytes(val: usize) -> u32 {
    if MEM_isLittleEndian() != 0 {
        val.trailing_zeros() >> 3
    } else {
        val.leading_zeros() >> 3
    }
}

#[inline]
pub(crate) const fn ZSTD_highbit32(val: u32) -> u32 {
    assert!(val != 0);
    31 - val.leading_zeros()
}
