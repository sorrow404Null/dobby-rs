use super::x86_64_common::{self, X64HookPlatform};
use super::{Backend, HookBuild};
use crate::error::Result;
use crate::platform;
use core::ffi::{c_char, c_void};
use core::ptr;

pub(crate) static BACKEND: UnixX86_64 = UnixX86_64;
pub(crate) struct UnixX86_64;
struct PlatformOps;

impl X64HookPlatform for PlatformOps {
    unsafe fn alloc_executable(size: usize) -> Result<*mut c_void> {
        platform::unix::alloc_executable(size)
    }
    unsafe fn free_executable(ptr: *mut c_void, size: usize) -> Result<()> {
        platform::unix::free_executable(ptr, size)
    }
    unsafe fn flush_icache(address: *mut c_void, size: usize) -> Result<()> {
        platform::unix::flush_icache(address, size);
        Ok(())
    }
    unsafe fn write_detour_with_nops(
        address: *mut c_void,
        stolen_len: usize,
        detour: &[u8; 14],
    ) -> Result<()> {
        platform::unix::with_rwx(address, stolen_len, || {
            ptr::copy_nonoverlapping(detour.as_ptr(), address as *mut u8, detour.len());
            for i in detour.len()..stolen_len {
                *(address as *mut u8).add(i) = 0x90;
            }
            Ok(())
        })
    }
}

impl Backend for UnixX86_64 {
    unsafe fn code_patch(
        &self,
        address: *mut c_void,
        buffer: *const u8,
        size: usize,
    ) -> Result<()> {
        platform::unix::code_patch(address, buffer, size)
    }
    unsafe fn hook_build(&self, address: *mut c_void, fake_func: *mut c_void) -> Result<HookBuild> {
        x86_64_common::hook_build::<PlatformOps>(address, fake_func)
    }
    unsafe fn hook_destroy(
        &self,
        address: *mut c_void,
        original: &[u8],
        patch_len: usize,
        trampoline: *mut c_void,
        trampoline_size: usize,
    ) -> Result<()> {
        platform::unix::restore_patch(address, original, patch_len)?;
        platform::unix::free_executable(trampoline, trampoline_size)
    }
    unsafe fn symbol_resolver(
        &self,
        image_name: *const c_char,
        symbol_name: *const c_char,
    ) -> *mut c_void {
        platform::unix::symbol_resolver(image_name, symbol_name)
    }
}
