use crate::error::{Error, Result};
use core::ffi::c_void;

pub(super) fn instrument(
    address: *mut c_void,
    _pre_handler: unsafe fn(address: *mut c_void, context: *mut c_void),
) -> Result<()> {
    if address.is_null() {
        return Err(Error::NullPointer);
    }
    Err(Error::UnsupportedPlatform)
}
