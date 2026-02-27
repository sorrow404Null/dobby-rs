//! EN: `install`, `install_with`, `install_addr`, and `replace` helpers.
//! CN: `install` / `install_with` / `install_addr` / `replace` 四种常用辅助 API。

mod common;

use core::ffi::c_void;
use dobby_rs_framework::hook_utils;
use dobby_rs_framework::prelude::*;

#[inline(never)]
fn detour_mul(x: i32) -> i32 {
    // EN: Fetch the original function pointer by detour address.
    // CN: 通过 detour 的地址拿到 original 函数指针。
    let original = unsafe {
        hook_utils::original::<fn(i32) -> i32>(detour_mul as *const () as *mut c_void)
            .expect("original not found")
    };
    original(x) + 1
}

#[inline(never)]
fn replacement_sub(_x: i32) -> i32 {
    // EN: A replacement can ignore inputs and return a fixed value.
    // CN: replace 可以完全忽略输入并返回固定值。
    123
}

#[inline(never)]
fn target_with_callbacks(x: i32) -> i32 {
    x + 10
}

#[inline(never)]
fn detour_with_callbacks(x: i32) -> i32 {
    // EN: `install_with` stores callbacks, but you decide when to invoke them.
    // CN: `install_with` 只负责注册回调；何时调用由 detour 自己决定。
    hook_utils::call_before(detour_with_callbacks as *const () as *mut c_void);

    let original = unsafe {
        hook_utils::original::<fn(i32) -> i32>(detour_with_callbacks as *const () as *mut c_void)
            .expect("original not found")
    };
    let out = original(x);

    hook_utils::call_after(detour_with_callbacks as *const () as *mut c_void);
    out
}

fn main() -> dobby_rs_framework::Result<()> {
    common::init_example_logging();

    // EN: 1) `install` -> typed handle.
    // CN: 1) `install` -> 返回带类型的句柄。
    let h1 = unsafe {
        install(
            common::target_mul as fn(i32) -> i32,
            detour_mul as fn(i32) -> i32,
        )?
    };
    println!("target_mul(3) after install = {}", common::target_mul(3));
    unsafe { h1.unhook()? };

    // EN: 2) `install_addr` -> raw pointer variant.
    // CN: 2) `install_addr` -> 按地址安装（裸指针版本）。
    let h1_addr = unsafe {
        install_addr(
            common::target_mul as *const () as *mut c_void,
            detour_mul as *const () as *mut c_void,
        )?
    };
    println!(
        "target_mul(3) after install_addr = {}",
        common::target_mul(3)
    );
    unsafe { h1_addr.unhook()? };

    // EN: 3) `replace` -> temporary swap, with easy access to original.
    // CN: 3) `replace` -> 临时替换，并且可以拿到 original。
    let r = unsafe {
        replace(
            common::target_sub as fn(i32) -> i32,
            replacement_sub as fn(i32) -> i32,
        )?
    };
    println!("target_sub(10) after replace = {}", common::target_sub(10));
    println!("original target_sub(10) = {}", (r.original())(10));
    unsafe { r.unreplace()? };

    // EN: 4) `install_with` -> before/after callbacks (invoked by detour).
    // CN: 4) `install_with` -> before/after 回调（由 detour 调用）。
    let h2 = unsafe {
        install_with(
            target_with_callbacks as fn(i32) -> i32,
            detour_with_callbacks as fn(i32) -> i32,
            Some(|| log::info!("before (install_with)")),
            Some(|| log::info!("after (install_with)")),
        )?
    };
    println!(
        "target_with_callbacks(1) after install_with = {}",
        target_with_callbacks(1)
    );
    unsafe { h2.unhook()? };

    Ok(())
}
