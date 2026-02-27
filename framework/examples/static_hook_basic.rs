//! EN: `StaticHook<T>` - the simplest typed inline hook pattern.
//! CN: `StaticHook<T>` - 最简单、最常用的带类型 inline hook 写法。

mod common;

use dobby_rs_framework::prelude::*;

// EN: A global hook handle is convenient for examples and small tools.
// CN: 全局 hook 句柄适合示例/小工具使用。
static HOOK: StaticHook<fn(i32) -> i32> = StaticHook::new();

#[inline(never)]
fn detour_add(x: i32) -> i32 {
    // EN: Optional: run user callbacks.
    // CN: 可选：执行用户的 before/after 回调。
    HOOK.call_before();

    // EN: Call the original implementation with modified arguments.
    // CN: 修改参数后调用原函数。
    let out = (HOOK.original())(x + 100);

    HOOK.call_after();

    // EN: Post-process return value.
    // CN: 对返回值做二次处理。
    out + 10
}

fn main() -> dobby_rs_framework::Result<()> {
    common::init_example_logging();

    // EN: Set optional callbacks (not required).
    // CN: 设置可选回调（非必须）。
    HOOK.set_before(|| log::info!("before"));
    HOOK.set_after(|| log::info!("after"));

    unsafe {
        HOOK.install(
            common::target_add as fn(i32) -> i32,
            detour_add as fn(i32) -> i32,
        )?;
    }

    let v = common::target_add(1);
    println!("target_add(1) = {v}");

    unsafe {
        HOOK.uninstall()?;
    }
    Ok(())
}
