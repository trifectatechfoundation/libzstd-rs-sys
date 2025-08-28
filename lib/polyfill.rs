#[macro_export]
macro_rules! cfg_select {
    ({ $($tt:tt)* }) => {{
        $crate::cfg_select! { $($tt)* }
    }};
    (_ => { $($output:tt)* }) => {
        $($output)*
    };
    (
        $cfg:meta => $output:tt
        $($( $rest:tt )+)?
    ) => {
        #[cfg($cfg)]
        $crate::cfg_select! { _ => $output }
        $(
            #[cfg(not($cfg))]
            $crate::cfg_select! { $($rest)+ }
        )?
    }
}

#[expect(dead_code)]
pub enum Locality {
    None = 0,
    L3 = 1,
    L2 = 2,
    L1 = 3,
}

pub fn prefetch_read_data<T>(data: *const T, locality: Locality) {
    // The match ensures that the locality argument is a constant value, as required.

    // SAFETY: the prefetch intrinsics do not modify the behavior of the program. They cannot trap
    // and do not produce a value. Hence it is safe to provide an arbitrary pointer.
    unsafe {
        match locality {
            Locality::None => prefetch_read_data_internal::<_, 0>(data),
            Locality::L3 => prefetch_read_data_internal::<_, 1>(data),
            Locality::L2 => prefetch_read_data_internal::<_, 2>(data),
            Locality::L1 => prefetch_read_data_internal::<_, 3>(data),
        }
    }
}

unsafe fn prefetch_read_data_internal<T, const LOCALITY: i32>(ptr: *const T) {
    cfg_select! {
        feature = "no-prefetch" => {
            let _ = ptr;
        }
        target_arch = "x86_64" => {
            use core::arch::x86_64;
            x86_64::_mm_prefetch(ptr as *const i8, LOCALITY)
        }
        target_arch = "x86" => {
            use core::arch::x86;
            unsafe { x86::_mm_prefetch(ptr as *const i8, LOCALITY) };
        }
        target_arch = "aarch64" => {
            core::arch::asm!(
                "prfm {op}, [{addr}]",
                op = const {
                    match LOCALITY {
                        0 => 0b00000, // pldl1strm
                        1 => 0b00001, // pldl1keep
                        2 => 0b00010, // pldl2keep
                        3 => 0b00011, // pldl3keep
                        _ => panic!(),
                    }
                },
                addr = in(reg) ptr,
                options(nostack, preserves_flags)
            );
        }
        _ => {
            let _ = ptr;
        }
    }
}
