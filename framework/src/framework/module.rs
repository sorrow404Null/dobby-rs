use crate::{Error, Result};
use core::ffi::{CStr, c_void};
use std::ffi::CString;

pub struct ModuleHandle {
    lib_name: CString,
    raw: *mut c_void,
}

impl ModuleHandle {
    pub fn open(lib_name: &str) -> Result<Self> {
        let lib_name = CString::new(lib_name).map_err(|_| Error::InvalidInput)?;
        let raw = unsafe { open_module(lib_name.as_c_str()) };
        if raw.is_null() {
            return Err(Error::SymbolNotFound);
        }
        Ok(Self { lib_name, raw })
    }
    pub fn lib_name(&self) -> &str {
        self.lib_name.to_str().unwrap_or("")
    }
    pub fn lib_name_cstr(&self) -> &CStr {
        self.lib_name.as_c_str()
    }
    pub fn raw(&self) -> *mut c_void {
        self.raw
    }
    pub fn resolve(&self, symbol: &CStr) -> Option<*mut c_void> {
        let p = unsafe { resolve_in_module(self.raw, symbol) };
        if p.is_null() { None } else { Some(p) }
    }
    pub fn wrapped_sym(&self, symbol: &str) -> Option<*mut c_void> {
        let c = CString::new(symbol).ok()?;
        self.resolve(c.as_c_str())
    }
}

#[cfg(unix)]
unsafe fn open_module(lib_name: &CStr) -> *mut c_void {
    libc::dlopen(lib_name.as_ptr(), libc::RTLD_NOW)
}
#[cfg(unix)]
unsafe fn resolve_in_module(module: *mut c_void, symbol: &CStr) -> *mut c_void {
    libc::dlsym(module, symbol.as_ptr())
}

#[cfg(windows)]
unsafe fn open_module(lib_name: &CStr) -> *mut c_void {
    use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, LoadLibraryA};
    let h = GetModuleHandleA(lib_name.as_ptr() as *const u8);
    if h.is_null() {
        LoadLibraryA(lib_name.as_ptr() as *const u8)
    } else {
        h
    }
}
#[cfg(windows)]
unsafe fn resolve_in_module(module: *mut c_void, symbol: &CStr) -> *mut c_void {
    use windows_sys::Win32::System::LibraryLoader::GetProcAddress;
    match GetProcAddress(module, symbol.as_ptr() as *const u8) {
        Some(p) => p as *const () as *mut c_void,
        None => core::ptr::null_mut(),
    }
}
