//! EN: Symbol resolution + alias registry + hook-by-alias/symbol.
//! CN: 符号解析 + 别名注册表 + 按 alias/symbol 安装 hook。

mod common;

use core::ffi::c_void;
use dobby_rs_framework::hook_utils;
use dobby_rs_framework::prelude::*;
use std::ffi::CString;

#[inline(never)]
fn detour_add(x: i32) -> i32 {
    let original = unsafe {
        hook_utils::original::<fn(i32) -> i32>(detour_add as *const () as *mut c_void)
            .expect("original not found")
    };
    original(x) + 1000
}

// EN: --- Unix demo: hook libc `puts` by symbol name.
// CN: --- Unix 演示：按 symbol 名 hook libc 的 `puts`。
#[cfg(unix)]
mod unix_demo {
    use super::*;

    pub fn run() -> dobby_rs_framework::Result<()> {
        let sym = CString::new("puts").unwrap();
        let img = CString::new(common::LIBC_IMAGE).unwrap();

        // EN: Resolve address and register an alias.
        // CN: 解析地址并注册 alias。
        let addr = resolve_and_register_alias("libc_puts", None, sym.as_c_str())?;
        println!("resolved puts addr = {addr:p}");

        // EN: Resolve with explicit image.
        // CN: 指定 image 名解析。
        let _ = resolve_and_register_alias_in("libc_puts_in", img.as_c_str(), sym.as_c_str())?;
        register_alias_with_symbol_in("libc_puts_meta_in", img.as_c_str(), sym.as_c_str(), addr)?;

        // EN: Hook by symbol (default image search).
        // CN: 按 symbol 安装 hook（默认 image 搜索）。
        let h1 = unsafe {
            hook_symbol_default(
                sym.as_c_str(),
                common::detours::unix::detour_puts as *const () as *mut c_void,
                Some("puts"),
            )?
        };
        unsafe { h1.unhook()? };

        // EN: Fully generic form.
        // CN: 完整通用形式。
        let h2 = unsafe {
            hook_symbol(
                None,
                sym.as_c_str(),
                common::detours::unix::detour_puts as *const () as *mut c_void,
                Some("puts2"),
            )?
        };
        unsafe { h2.unhook()? };

        Ok(())
    }
}

// EN: --- Windows demo: hook a simple kernel32 API by symbol.
// CN: --- Windows 演示：按 symbol hook kernel32 的简单 API。
#[cfg(windows)]
mod windows_demo {
    use super::*;

    pub fn run() -> dobby_rs_framework::Result<()> {
        let image = CString::new("kernel32.dll").unwrap();
        let sym = CString::new("GetTickCount").unwrap();

        let _ = resolve_and_register_alias_in("k32_get_tick", image.as_c_str(), sym.as_c_str())?;
        let h = unsafe {
            hook_symbol_in(
                image.as_c_str(),
                sym.as_c_str(),
                common::detours::windows::detour_get_tick_count as *const () as *mut c_void,
                Some("GetTickCount"),
            )?
        };
        unsafe { h.unhook()? };
        Ok(())
    }
}

fn main() -> dobby_rs_framework::Result<()> {
    common::init_example_logging();

    // EN: Alias registry is just a map: alias -> address (+optional metadata).
    // CN: alias 注册表本质就是：alias -> 地址（以及可选的元信息）。
    let add_addr = common::target_add as *const () as *mut c_void;
    register_alias("demo_add", add_addr)?;
    println!("alias demo_add addr = {:p}", get_alias("demo_add").unwrap());

    // EN: Hook by alias.
    // CN: 按 alias 安装 hook。
    let h = unsafe { hook_alias("demo_add", detour_add as *const () as *mut c_void)? };
    println!("target_add(1) after hook_alias = {}", common::target_add(1));
    unsafe { h.unhook()? };

    // EN/CN: Optional: store richer alias metadata.
    let fake_sym = CString::new("target_add").unwrap();
    register_alias_with_symbol("demo_add_meta", None, fake_sym.as_c_str(), add_addr)?;

    #[cfg(unix)]
    unix_demo::run()?;
    #[cfg(windows)]
    windows_demo::run()?;

    Ok(())
}
