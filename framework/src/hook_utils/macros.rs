#[macro_export]
macro_rules! dobby_hook {
    ($target:path, $detour:path) => {{
        unsafe {
            $crate::hook_utils::HookBuilder::new(
                $target as *const () as *mut core::ffi::c_void,
                $detour as *const () as *mut core::ffi::c_void,
            )
            .install()
        }
    }};
}

#[macro_export]
macro_rules! dobby_original {
    ($detour:path, $ty:ty) => {{
        unsafe {
            $crate::hook_utils::original::<$ty>($detour as *const () as *mut core::ffi::c_void)
                .expect("original function not found")
        }
    }};
}
