//! EN: Shared helpers for `dobby-rs-framework` examples.
//! CN: `dobby-rs-framework` 示例的公共辅助代码。

#![allow(dead_code)]

use dobby_rs_framework::framework::ModuleHandle;
use dobby_rs_framework::prelude::{LogOptions, init_logging};
use dobby_rs_framework::{Error, Result};
use std::ffi::CString;

// EN: Tiny local targets used by multiple examples.
// CN: 多个示例复用的本地 target。

#[inline(never)]
pub fn target_add(x: i32) -> i32 {
    x + 1
}

#[inline(never)]
pub fn target_mul(x: i32) -> i32 {
    x * 2
}

#[inline(never)]
pub fn target_sub(x: i32) -> i32 {
    x - 1
}

// EN: Initialize logging for examples (safe to ignore errors).
// CN: 初始化示例日志（失败可忽略）。
pub fn init_example_logging() {
    let _ = init_logging(LogOptions::default());
}

// EN: A small cross-platform library used in symbol/module examples.
// CN: 符号/模块示例里用到的跨平台动态库名。

#[cfg(windows)]
pub const DEMO_LIB: &str = "kernel32.dll";
#[cfg(target_os = "linux")]
pub const DEMO_LIB: &str = "libc.so.6";
#[cfg(target_os = "macos")]
pub const DEMO_LIB: &str = "/usr/lib/libSystem.B.dylib";

// EN/CN: libc image path (used when an explicit image is required).

#[cfg(target_os = "linux")]
pub const LIBC_IMAGE: &str = "libc.so.6";
#[cfg(target_os = "macos")]
pub const LIBC_IMAGE: &str = "/usr/lib/libSystem.B.dylib";

// EN: Common "resolve and print" helper used by examples.
// CN: 示例复用的“解析并打印”辅助函数。
pub fn resolve_and_print(m: &ModuleHandle, symbol: &str) -> Result<()> {
    let c = CString::new(symbol).map_err(|_| Error::InvalidInput)?;
    let p = m.resolve(c.as_c_str()).ok_or(Error::SymbolNotFound)?;
    println!("module.resolve({symbol}) = {p:p}");
    Ok(())
}

/// EN: Reusable detours used by multiple examples.
/// CN: 多个示例复用的 detour。
pub mod detours {
    #[cfg(unix)]
    pub mod unix {
        use core::ffi::{c_char, c_int};

        pub type PutsFn = unsafe extern "C" fn(*const c_char) -> c_int;

        #[inline(never)]
        pub unsafe extern "C" fn detour_puts(s: *const c_char) -> c_int {
            let original: PutsFn = dobby_rs_framework::dobby_original!(detour_puts, PutsFn);
            unsafe { original(s) }
        }
    }

    #[cfg(windows)]
    pub mod windows {
        pub type GetTickCountFn = unsafe extern "system" fn() -> u32;
        pub type GetCurrentProcessIdFn = unsafe extern "system" fn() -> u32;

        #[inline(never)]
        pub unsafe extern "system" fn detour_get_tick_count() -> u32 {
            let original: GetTickCountFn =
                dobby_rs_framework::dobby_original!(detour_get_tick_count, GetTickCountFn);
            (unsafe { original() }) + 1
        }

        #[inline(never)]
        pub unsafe extern "system" fn detour_get_current_process_id() -> u32 {
            let original: GetCurrentProcessIdFn = dobby_rs_framework::dobby_original!(
                detour_get_current_process_id,
                GetCurrentProcessIdFn
            );
            unsafe { original() }
        }
    }
}
