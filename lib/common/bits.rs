#[inline]
pub(crate) const fn ZSTD_highbit32(val: u32) -> u32 {
    // Normally ilog2 panics when the input is zero.
    debug_assert!(val != 0);
    (val | 1).ilog2()
}
