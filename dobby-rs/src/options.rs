use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static NEAR_TRAMPOLINE: AtomicBool = AtomicBool::new(false);
static ALLOC_NEAR_CODE_CB: AtomicUsize = AtomicUsize::new(0);

pub type AllocNearCodeCallback = unsafe fn(size: u32, pos: usize, range: usize) -> usize;

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
