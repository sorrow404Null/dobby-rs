use crate::error::Result;
use core::ffi::{c_char, c_void};

pub(crate) struct HookBuild {
    pub(crate) trampoline: *mut c_void,
    pub(crate) trampoline_size: usize,
    pub(crate) original: Vec<u8>,
    pub(crate) patch_len: usize,
}

pub(crate) trait Backend: Sync {
    unsafe fn code_patch(&self, address: *mut c_void, buffer: *const u8, size: usize)
    -> Result<()>;
    unsafe fn hook_build(&self, address: *mut c_void, fake_func: *mut c_void) -> Result<HookBuild>;
    unsafe fn hook_destroy(
        &self,
        address: *mut c_void,
        original: &[u8],
        patch_len: usize,
        trampoline: *mut c_void,
        trampoline_size: usize,
    ) -> Result<()>;
    unsafe fn symbol_resolver(
        &self,
        image_name: *const c_char,
        symbol_name: *const c_char,
    ) -> *mut c_void;
}

#[cfg(all(unix, target_arch = "aarch64"))]
mod unix_aarch64;
#[cfg(all(unix, target_arch = "x86_64"))]
mod unix_x86_64;
#[cfg(all(windows, target_arch = "x86_64"))]
mod windows_x86_64;
#[cfg(target_arch = "x86_64")]
mod x86_64_common;

pub(crate) fn get() -> &'static dyn Backend {
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        &windows_x86_64::BACKEND
    }
    #[cfg(all(unix, target_arch = "x86_64"))]
    {
        &unix_x86_64::BACKEND
    }
    #[cfg(all(unix, target_arch = "aarch64"))]
    {
        &unix_aarch64::BACKEND
    }
    #[cfg(not(any(
        all(windows, target_arch = "x86_64"),
        all(unix, target_arch = "x86_64"),
        all(unix, target_arch = "aarch64")
    )))]
    {
        struct Unsupported;
        impl Backend for Unsupported {
            unsafe fn code_patch(&self, _a: *mut c_void, _b: *const u8, _s: usize) -> Result<()> {
                Err(crate::error::Error::UnsupportedPlatform)
            }
            unsafe fn hook_build(&self, _a: *mut c_void, _f: *mut c_void) -> Result<HookBuild> {
                Err(crate::error::Error::UnsupportedPlatform)
            }
            unsafe fn hook_destroy(
                &self,
                _a: *mut c_void,
                _o: &[u8],
                _p: usize,
                _t: *mut c_void,
                _ts: usize,
            ) -> Result<()> {
                Err(crate::error::Error::UnsupportedPlatform)
            }
            unsafe fn symbol_resolver(&self, _i: *const c_char, _s: *const c_char) -> *mut c_void {
                core::ptr::null_mut()
            }
        }
        static U: Unsupported = Unsupported;
        &U
    }
}
