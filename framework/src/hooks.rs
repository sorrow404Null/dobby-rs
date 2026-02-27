use crate::Result;
use crate::hook_utils;
use core::ffi::c_void;

pub use crate::hook_utils::{StaticHook, TypedHookHandle};

pub struct ReplaceHandle<F: Copy> {
    handle: TypedHookHandle<F>,
    original: F,
}
impl<F: Copy> ReplaceHandle<F> {
    pub fn original(&self) -> F {
        self.original
    }
    pub unsafe fn unreplace(self) -> Result<()> {
        self.handle.unhook()
    }
}

pub unsafe fn install<F: Copy>(target: F, detour: F) -> Result<TypedHookHandle<F>> {
    hook_utils::install(target, detour)
}
pub unsafe fn install_with<F: Copy, B, A>(
    target: F,
    detour: F,
    before: Option<B>,
    after: Option<A>,
) -> Result<TypedHookHandle<F>>
where
    B: Fn() + Send + Sync + 'static,
    A: Fn() + Send + Sync + 'static,
{
    hook_utils::hook_fn_with(target, detour, before, after)
}
pub unsafe fn install_addr(
    target: *mut c_void,
    detour: *mut c_void,
) -> Result<hook_utils::HookHandle> {
    hook_utils::install_addr(target, detour)
}
pub unsafe fn replace<F: Copy>(target: F, replacement: F) -> Result<ReplaceHandle<F>> {
    let h = install(target, replacement)?;
    let o = h.original();
    Ok(ReplaceHandle {
        handle: h,
        original: o,
    })
}
