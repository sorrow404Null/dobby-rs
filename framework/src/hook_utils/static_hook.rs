use super::{Callback, HookHandle, hook_fn};
use crate::{Error, Result};
use core::marker::PhantomData;
use log::debug;
use std::sync::{Arc, Mutex};

pub struct StaticHook<F> {
    handle: Mutex<Option<HookHandle>>,
    before: Mutex<Option<Callback>>,
    after: Mutex<Option<Callback>>,
    _marker: PhantomData<F>,
}

impl<F: Copy> StaticHook<F> {
    pub const fn new() -> Self {
        Self {
            handle: Mutex::new(None),
            before: Mutex::new(None),
            after: Mutex::new(None),
            _marker: PhantomData,
        }
    }
    pub fn set_before<C: Fn() + Send + Sync + 'static>(&self, cb: C) {
        *self.before.lock().unwrap() = Some(Arc::new(cb));
    }
    pub fn set_after<C: Fn() + Send + Sync + 'static>(&self, cb: C) {
        *self.after.lock().unwrap() = Some(Arc::new(cb));
    }
    pub fn call_before(&self) {
        debug!("static hook before-callback");
        if let Some(cb) = self.before.lock().unwrap().clone() {
            cb();
        }
    }
    pub fn call_after(&self) {
        debug!("static hook after-callback");
        if let Some(cb) = self.after.lock().unwrap().clone() {
            cb();
        }
    }
    pub unsafe fn install(&self, target: F, detour: F) -> Result<()> {
        let mut s = self.handle.lock().unwrap();
        if s.is_some() {
            return Err(Error::AlreadyHooked);
        }
        *s = Some(hook_fn(target, detour)?.inner);
        Ok(())
    }
    pub unsafe fn uninstall(&self) -> Result<()> {
        let h = self
            .handle
            .lock()
            .unwrap()
            .take()
            .ok_or(Error::HookNotFound)?;
        h.unhook()
    }
    pub fn original(&self) -> F {
        unsafe {
            self.handle
                .lock()
                .unwrap()
                .as_ref()
                .expect("hook not installed")
                .original()
        }
    }
}

impl<F: Copy> Default for StaticHook<F> {
    fn default() -> Self {
        Self::new()
    }
}
