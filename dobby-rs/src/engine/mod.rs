use core::ffi::{CStr, c_char, c_void};

mod backend;
mod imports;
mod instrument;
mod manager;

use crate::error::{Error, Result};

pub unsafe fn code_patch(address: *mut c_void, buffer: *const u8, buffer_size: u32) -> Result<()> {
    if address.is_null() || buffer.is_null() {
        return Err(Error::NullPointer);
    }
    backend::get().code_patch(address, buffer, buffer_size as usize)
}

pub unsafe fn hook(address: *mut c_void, fake_func: *mut c_void) -> Result<*mut c_void> {
    if address.is_null() || fake_func.is_null() {
        return Err(Error::NullPointer);
    }
    manager::hook(address, fake_func)
}

pub unsafe fn destroy(address: *mut c_void) -> Result<()> {
    if address.is_null() {
        return Err(Error::NullPointer);
    }
    manager::destroy(address)
}

pub unsafe fn symbol_resolver(
    image_name: *const c_char,
    symbol_name: *const c_char,
) -> *mut c_void {
    backend::get().symbol_resolver(image_name, symbol_name)
}

pub fn resolve_symbol(image_name: Option<&CStr>, symbol_name: &CStr) -> *mut c_void {
    unsafe {
        symbol_resolver(
            image_name.map_or(core::ptr::null(), CStr::as_ptr),
            symbol_name.as_ptr(),
        )
    }
}

pub fn import_table_replace(
    image_name: Option<&CStr>,
    symbol_name: &CStr,
    fake_func: *mut c_void,
) -> Result<*mut c_void> {
    if fake_func.is_null() {
        return Err(Error::NullPointer);
    }
    imports::import_table_replace(image_name, symbol_name, fake_func)
}

pub fn instrument(
    address: *mut c_void,
    pre_handler: unsafe fn(address: *mut c_void, context: *mut c_void),
) -> Result<()> {
    instrument::instrument(address, pre_handler)
}
