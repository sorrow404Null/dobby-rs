use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static NEAR_TRAMPOLINE: AtomicBool = AtomicBool::new(false);
static ALLOC_NEAR_CODE_CB: AtomicUsize = AtomicUsize::new(0);

pub type AllocNearCodeCallback = unsafe fn(size: u32, pos: usize, range: usize) -> usize;

// These helpers are currently only used by the x86_64 trampoline relocation path.
// Keep them cfg-gated so non-x86_64 builds don't trip -D dead-code.
#[cfg(target_arch = "x86_64")]
pub(crate) fn near_trampoline_enabled() -> bool {
    NEAR_TRAMPOLINE.load(Ordering::Relaxed)
}

#[cfg(target_arch = "x86_64")]
pub(crate) fn alloc_near_code_callback() -> Option<AllocNearCodeCallback> {
    let p = ALLOC_NEAR_CODE_CB.load(Ordering::Relaxed);
    if p == 0 {
        None
    } else {
        Some(unsafe { core::mem::transmute::<usize, AllocNearCodeCallback>(p) })
    }
}

pub fn set_near_trampoline(enable: bool) {
    NEAR_TRAMPOLINE.store(enable, Ordering::Relaxed);
}

pub fn register_alloc_near_code_callback(handler: Option<AllocNearCodeCallback>) {
    ALLOC_NEAR_CODE_CB.store(handler.map_or(0, |f| f as usize), Ordering::Relaxed);
}

pub fn set_options(
    enable_near_trampoline: bool,
    alloc_near_code_callback: Option<AllocNearCodeCallback>,
) {
    set_near_trampoline(enable_near_trampoline);
    register_alloc_near_code_callback(alloc_near_code_callback);
}
