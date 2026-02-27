use super::{Callback, HookHandle, install_raw, registry};
use crate::{Error, Result};
use core::ffi::c_void;
use log::{debug, info};
use std::sync::Arc;

pub struct HookBuilder {
    target: *mut c_void,
    detour: *mut c_void,
    before: Option<Callback>,
    after: Option<Callback>,
}

impl HookBuilder {
    pub fn new(target: *mut c_void, detour: *mut c_void) -> Self {
        Self {
            target,
            detour,
            before: None,
            after: None,
        }
    }
    pub fn before<F: Fn() + Send + Sync + 'static>(mut self, callback: F) -> Self {
        self.before = Some(Arc::new(callback));
        self
    }
    pub fn after<F: Fn() + Send + Sync + 'static>(mut self, callback: F) -> Self {
        self.after = Some(Arc::new(callback));
        self
    }
    pub unsafe fn install(self) -> Result<HookHandle> {
        if self.target.is_null() || self.detour.is_null() {
            return Err(Error::NullPointer);
        }
        if registry::contains(self.target as usize, self.detour as usize) {
            return Err(Error::AlreadyHooked);
        }
        debug!(
            "installing hook target={:p} detour={:p}",
            self.target, self.detour
        );
        let h = install_raw(self.target, self.detour, self.before, self.after)?;
        info!(
            "hook installed target={:p} detour={:p}",
            h.target_ptr(),
            h.detour_ptr()
        );
        Ok(h)
    }
}
