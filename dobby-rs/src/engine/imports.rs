use crate::error::{Error, Result};
use core::ffi::{CStr, c_void};

pub(super) fn import_table_replace(
    _image_name: Option<&CStr>,
    _symbol_name: &CStr,
    _fake_func: *mut c_void,
) -> Result<*mut c_void> {
    Err(Error::UnsupportedPlatform)
}
