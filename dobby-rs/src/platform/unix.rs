use crate::error::{Error, Result};
use core::ffi::c_void;
use core::ptr;

#[cfg(any(target_os = "linux", target_os = "android"))]
unsafe fn errno() -> i32 {
    *libc::__errno_location()
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe fn errno() -> i32 {
    *libc::__error()
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
)))]
unsafe fn errno() -> i32 {
    0
}

pub(crate) unsafe fn page_size() -> usize {
    let ps = libc::sysconf(libc::_SC_PAGESIZE);
    if ps <= 0 { 4096 } else { ps as usize }
}

pub(crate) unsafe fn page_align_down(addr: usize) -> usize {
    addr & !(page_size() - 1)
}
pub(crate) unsafe fn page_align_up(addr: usize) -> usize {
    (addr + page_size() - 1) & !(page_size() - 1)
}

pub(crate) unsafe fn alloc_executable(size: usize) -> Result<*mut c_void> {
    let p = libc::mmap(
        ptr::null_mut(),
        size,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANON,
        -1,
        0,
    );
    if p == libc::MAP_FAILED {
        return Err(Error::Unix(errno()));
    }
    Ok(p)
}

pub(crate) unsafe fn free_executable(ptr: *mut c_void, size: usize) -> Result<()> {
    if ptr.is_null() {
        return Ok(());
    }
    if libc::munmap(ptr, size) != 0 {
        return Err(Error::Unix(errno()));
    }
    Ok(())
}

pub(crate) unsafe fn with_rwx(
    address: *mut c_void,
    size: usize,
    f: impl FnOnce() -> Result<()>,
) -> Result<()> {
    let start = page_align_down(address as usize) as *mut c_void;
    let end = page_align_up(address as usize + size);
    let len = end - start as usize;
    if libc::mprotect(
        start,
        len,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    ) != 0
    {
        return Err(Error::Unix(errno()));
    }
    let r = f();
    let _ = libc::mprotect(start, len, libc::PROT_READ | libc::PROT_EXEC);
    r
}

#[allow(dead_code)]
pub(crate) unsafe fn with_writeable(
    address: *mut c_void,
    size: usize,
    f: impl FnOnce() -> Result<()>,
) -> Result<()> {
    let start = page_align_down(address as usize) as *mut c_void;
    let end = page_align_up(address as usize + size);
    let len = end - start as usize;
    if libc::mprotect(
        start,
        len,
        libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
    ) != 0
    {
        return Err(Error::Unix(errno()));
    }
    let r = f();
    let _ = libc::mprotect(start, len, libc::PROT_READ | libc::PROT_EXEC);
    r
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
unsafe extern "C" {
    fn __clear_cache(begin: *mut u8, end: *mut u8);
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub(crate) unsafe fn flush_icache(address: *mut c_void, size: usize) {
    __clear_cache(address as *mut u8, (address as *mut u8).add(size));
}

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub(crate) unsafe fn flush_icache(_address: *mut c_void, _size: usize) {}

pub(crate) unsafe fn code_patch(
    address: *mut c_void,
    buffer: *const u8,
    size: usize,
) -> Result<()> {
    with_rwx(address, size, || {
        ptr::copy_nonoverlapping(buffer, address as *mut u8, size);
        Ok(())
    })?;
    flush_icache(address, size);
    Ok(())
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
    flush_icache(address, patch_len);
    Ok(())
}

pub(crate) unsafe fn symbol_resolver(
    image_name: *const core::ffi::c_char,
    symbol_name: *const core::ffi::c_char,
) -> *mut c_void {
    if symbol_name.is_null() {
        return ptr::null_mut();
    }
    let handle = if image_name.is_null() {
        libc::RTLD_DEFAULT
    } else {
        libc::dlopen(image_name, libc::RTLD_NOW)
    };
    if handle.is_null() {
        return ptr::null_mut();
    }
    libc::dlsym(handle, symbol_name)
}
