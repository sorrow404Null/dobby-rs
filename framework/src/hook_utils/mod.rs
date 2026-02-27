mod builder;
mod handle;
mod macros;
mod registry;
mod static_hook;

use crate::Result;
use crate::hook;
use core::ffi::c_void;
use core::marker::PhantomData;

pub(crate) type Callback = std::sync::Arc<dyn Fn() + Send + Sync + 'static>;
pub use builder::HookBuilder;
pub use handle::{HookHandle, TypedHookHandle};
pub use static_hook::StaticHook;

unsafe fn fn_to_ptr<F: Copy>(f: F) -> *mut c_void {
    debug_assert_eq!(
        core::mem::size_of::<F>(),
        core::mem::size_of::<*mut c_void>()
    );
    let raw: *const () = core::mem::transmute_copy(&f);
    raw as *mut c_void
}

pub unsafe fn hook_fn<F: Copy>(target: F, detour: F) -> Result<TypedHookHandle<F>> {
    let handle = HookBuilder::new(fn_to_ptr(target), fn_to_ptr(detour)).install()?;
    Ok(TypedHookHandle {
        inner: handle,
        _marker: PhantomData,
    })
}

pub unsafe fn install<F: Copy>(target: F, detour: F) -> Result<TypedHookHandle<F>> {
    hook_fn(target, detour)
}

pub unsafe fn hook_fn_with<F: Copy, B, A>(
    target: F,
    detour: F,
    before: Option<B>,
    after: Option<A>,
) -> Result<TypedHookHandle<F>>
where
    B: Fn() + Send + Sync + 'static,
    A: Fn() + Send + Sync + 'static,
{
    let mut b = HookBuilder::new(fn_to_ptr(target), fn_to_ptr(detour));
    if let Some(before) = before {
        b = b.before(before);
    }
    if let Some(after) = after {
        b = b.after(after);
    }
    let handle = b.install()?;
    Ok(TypedHookHandle {
        inner: handle,
        _marker: PhantomData,
    })
}

pub fn call_before(detour: *mut c_void) {
    if let Some(cb) = registry::get_before(detour as usize) {
        cb();
    }
}
pub fn call_after(detour: *mut c_void) {
    if let Some(cb) = registry::get_after(detour as usize) {
        cb();
    }
}
pub unsafe fn original<T: Copy>(detour: *mut c_void) -> Option<T> {
    let p = registry::get_original(detour as usize)?;
    debug_assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<usize>());
    Some(core::mem::transmute_copy(&p))
}

pub unsafe fn install_addr(target: *mut c_void, detour: *mut c_void) -> Result<HookHandle> {
    HookBuilder::new(target, detour).install()
}

pub(crate) unsafe fn install_raw(
    target: *mut c_void,
    detour: *mut c_void,
    before: Option<Callback>,
    after: Option<Callback>,
) -> Result<HookHandle> {
    let original = hook(target, detour)?;
    let h = HookHandle {
        target: target as usize,
        detour: detour as usize,
        original: original as usize,
    };
    registry::insert(h.target, h.detour, h.original, before, after);
    Ok(h)
}
