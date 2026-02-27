//! EN: Convenience macros: `dobby_hook!` + `dobby_original!`.
//! CN: 便捷宏：`dobby_hook!` + `dobby_original!`。

mod common;

#[inline(never)]
fn detour_add(x: i32) -> i32 {
    // EN: `dobby_original!` fetches the original function pointer for this detour.
    // CN: `dobby_original!` 会根据 detour 找到对应的 original 函数指针。
    let original: fn(i32) -> i32 = dobby_rs_framework::dobby_original!(detour_add, fn(i32) -> i32);
    original(x) + 10
}

fn main() -> dobby_rs_framework::Result<()> {
    common::init_example_logging();

    // EN: `dobby_hook!` installs a hook using function items.
    // CN: `dobby_hook!` 直接用函数名安装 hook。
    let h = dobby_rs_framework::dobby_hook!(common::target_add, detour_add)?;

    println!("target_add(1) after dobby_hook = {}", common::target_add(1));

    unsafe { h.unhook()? };
    Ok(())
}
