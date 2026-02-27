use crate::engine::backend;
use crate::error::{Error, Result};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Mutex;

#[derive(Debug)]
struct HookInfo {
    original: Vec<u8>,
    patch_len: usize,
    trampoline: usize,
    trampoline_size: usize,
}

static HOOKS: OnceCell<Mutex<HashMap<usize, HookInfo>>> = OnceCell::new();
fn hooks() -> &'static Mutex<HashMap<usize, HookInfo>> {
    HOOKS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) unsafe fn hook(address: *mut c_void, fake_func: *mut c_void) -> Result<*mut c_void> {
    let key = address as usize;
    if hooks().lock().unwrap().contains_key(&key) {
        return Err(Error::AlreadyHooked);
    }
    let build = backend::get().hook_build(address, fake_func)?;
    hooks().lock().unwrap().insert(
        key,
        HookInfo {
            original: build.original,
            patch_len: build.patch_len,
            trampoline: build.trampoline as usize,
            trampoline_size: build.trampoline_size,
        },
    );
    Ok(build.trampoline)
}

pub(crate) unsafe fn destroy(address: *mut c_void) -> Result<()> {
    let key = address as usize;
    let info = hooks()
        .lock()
        .unwrap()
        .remove(&key)
        .ok_or(Error::HookNotFound)?;
    backend::get().hook_destroy(
        address,
        &info.original,
        info.patch_len,
        info.trampoline as *mut c_void,
        info.trampoline_size,
    )
}
