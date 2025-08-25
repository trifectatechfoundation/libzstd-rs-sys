#[inline]
pub(crate) unsafe fn ZSTD_selectAddr(
    index: u32,
    lowLimit: u32,
    mut candidate: *const u8,
    backup: *const u8,
) -> *const u8 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if !cfg!(miri) {
        // Apparently this tries to force branchless codegen.
        core::arch::asm!(
            "cmp {1:e}, {2:e}
            cmova {3}, {0}",
            inlateout(reg) candidate,
            inlateout(reg) index => _,
            inlateout(reg) lowLimit => _,
            inlateout(reg) backup => _,
            options(preserves_flags, pure, readonly, att_syntax)
        );
        return candidate;
    }

    if index >= lowLimit {
        candidate
    } else {
        backup
    }
}
