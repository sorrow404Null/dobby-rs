use super::x86_64_common::{self, X64HookPlatform};
use super::{Backend, HookBuild};
use crate::error::Result;
use crate::platform;
use core::ffi::{c_char, c_void};
use core::ptr;

pub(crate) static BACKEND: WindowsX86_64 = WindowsX86_64;
pub(crate) struct WindowsX86_64;
struct PlatformOps;

impl X64HookPlatform for PlatformOps {
    unsafe fn alloc_executable(size: usize) -> Result<*mut c_void> {
        platform::windows::alloc_executable(size)
    }
    unsafe fn alloc_executable_near(size: usize, pos: usize, range: usize) -> Result<*mut c_void> {
        platform::windows::alloc_executable_near(size, pos, range)
    }
    unsafe fn free_executable(ptr: *mut c_void, _size: usize) -> Result<()> {
        platform::windows::free_executable(ptr)
    }
    unsafe fn flush_icache(address: *mut c_void, size: usize) -> Result<()> {
        platform::windows::flush_icache(address, size)
    }
    unsafe fn write_detour_with_nops(
        address: *mut c_void,
        stolen_len: usize,
        detour: &[u8; 14],
    ) -> Result<()> {
        platform::windows::with_rwx(address, stolen_len, || {
            ptr::copy_nonoverlapping(detour.as_ptr(), address as *mut u8, detour.len());
            for i in detour.len()..stolen_len {
                *(address as *mut u8).add(i) = 0x90;
            }
            Ok(())
        })
    }
}

impl Backend for WindowsX86_64 {
    unsafe fn code_patch(
        &self,
        address: *mut c_void,
        buffer: *const u8,
        size: usize,
    ) -> Result<()> {
        platform::windows::code_patch(address, buffer, size)
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
        _trampoline_size: usize,
    ) -> Result<()> {
        platform::windows::restore_patch(address, original, patch_len)?;
        platform::windows::free_executable(trampoline)
    }
    unsafe fn symbol_resolver(
        &self,
        image_name: *const c_char,
        symbol_name: *const c_char,
    ) -> *mut c_void {
        use core::ffi::CStr;
        use windows_sys::Win32::System::LibraryLoader::{
            GetModuleHandleA, GetProcAddress, LoadLibraryA,
        };
        if symbol_name.is_null() {
            return core::ptr::null_mut();
        }
        let sym = CStr::from_ptr(symbol_name);
        let module = if image_name.is_null() {
            GetModuleHandleA(core::ptr::null())
        } else {
            let img = CStr::from_ptr(image_name);
            let h = GetModuleHandleA(img.as_ptr() as *const u8);
            if h.is_null() {
                LoadLibraryA(img.as_ptr() as *const u8)
            } else {
                h
            }
        };
        if module.is_null() {
            return core::ptr::null_mut();
        }
        match GetProcAddress(module, sym.as_ptr() as *const u8) {
            Some(p) => p as *const () as *mut c_void,
            None => core::ptr::null_mut(),
        }
    }
}
