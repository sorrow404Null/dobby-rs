use super::registry;
use crate::{Result, destroy};
use core::ffi::c_void;
use core::marker::PhantomData;
use log::info;

pub struct HookHandle {
    pub(crate) target: usize,
    pub(crate) detour: usize,
    pub(crate) original: usize,
}

impl HookHandle {
    pub fn target_ptr(&self) -> *mut c_void {
        self.target as *mut c_void
    }
    pub fn detour_ptr(&self) -> *mut c_void {
        self.detour as *mut c_void
    }
    pub fn original_ptr(&self) -> *mut c_void {
        self.original as *mut c_void
    }
    pub unsafe fn original<T: Copy>(&self) -> T {
        debug_assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<usize>());
        core::mem::transmute_copy(&self.original)
    }
    pub unsafe fn unhook(self) -> Result<()> {
        info!(
            "uninstalling hook target={:p} detour={:p}",
            self.target_ptr(),
            self.detour_ptr()
        );
        destroy(self.target_ptr())?;
        registry::remove(self.target, self.detour);
        Ok(())
    }
}

pub struct TypedHookHandle<F> {
    pub(crate) inner: HookHandle,
    pub(crate) _marker: PhantomData<F>,
}
impl<F: Copy> TypedHookHandle<F> {
    pub fn original(&self) -> F {
        unsafe { self.inner.original() }
    }
    pub unsafe fn unhook(self) -> Result<()> {
        self.inner.unhook()
    }
}
