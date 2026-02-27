use crate::error::{Error, Result};
use core::ffi::c_void;
use core::ptr;
use windows_sys::Win32::Foundation::{GetLastError, HANDLE};
use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, VirtualAlloc, VirtualFree,
    VirtualProtect,
};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

unsafe fn last_error() -> Error {
    Error::Win32(GetLastError())
}

pub(crate) unsafe fn alloc_executable(size: usize) -> Result<*mut c_void> {
    let p = VirtualAlloc(
        ptr::null_mut(),
        size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    );
    if p.is_null() {
        return Err(last_error());
    }
    Ok(p)
}

pub(crate) unsafe fn free_executable(ptr: *mut c_void) -> Result<()> {
    if ptr.is_null() {
        return Ok(());
    }
    if VirtualFree(ptr, 0, MEM_RELEASE) == 0 {
        return Err(last_error());
    }
    Ok(())
}

pub(crate) unsafe fn with_rwx(
    address: *mut c_void,
    size: usize,
    f: impl FnOnce() -> Result<()>,
) -> Result<()> {
    let mut old = 0u32;
    if VirtualProtect(address, size, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
        return Err(last_error());
    }
    let r = f();
    let mut _tmp = 0u32;
    let _ = VirtualProtect(address, size, old, &mut _tmp);
    r
}

pub(crate) unsafe fn flush_icache(address: *mut c_void, size: usize) -> Result<()> {
    let proc: HANDLE = GetCurrentProcess();
    if FlushInstructionCache(proc, address, size) == 0 {
        return Err(last_error());
    }
    Ok(())
}

pub(crate) unsafe fn code_patch(
    address: *mut c_void,
    buffer: *const u8,
    size: usize,
) -> Result<()> {
    with_rwx(address, size, || {
        ptr::copy_nonoverlapping(buffer, address as *mut u8, size);
        Ok(())
    })?;
    flush_icache(address, size)
}

pub(crate) unsafe fn restore_patch(
    address: *mut c_void,
    original: &[u8],
    patch_len: usize,
) -> Result<()> {
    if patch_len != original.len() {
        return Err(Error::PatchTooSmall);
    }
    with_rwx(address, patch_len, || {
        ptr::copy_nonoverlapping(original.as_ptr(), address as *mut u8, patch_len);
        Ok(())
    })?;
    flush_icache(address, patch_len)
}
