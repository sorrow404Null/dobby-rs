//! EN: Batch install hooks with `inline_hooks(...).install()`.
//! CN: 使用 `inline_hooks(...).install()` 批量安装 hook。

mod common;
use dobby_rs_framework::prelude::*;

// EN: Detours are shared via `common::detours`.
// CN: detour 通过 `common::detours` 共享。

fn main() -> dobby_rs_framework::Result<()> {
    // EN/CN: Keep this example tiny: install, then uninstall.

    #[cfg(unix)]
    let builder: InlineHooksBuilder<'_> = inline_hooks(common::DEMO_LIB)
        .hook_alias(
            "puts",
            "puts",
            common::detours::unix::detour_puts
                as unsafe extern "C" fn(*const core::ffi::c_char) -> core::ffi::c_int,
        )
        .extra_action_fn(|m| common::resolve_and_print(m, "puts"));
    #[cfg(unix)]
    let mut session: HookSession = unsafe { builder.install()? };

    #[cfg(windows)]
    let builder: InlineHooksBuilder<'_> = inline_hooks(common::DEMO_LIB)
        .hook_alias(
            "GetTickCount",
            "GetTickCount",
            common::detours::windows::detour_get_tick_count as unsafe extern "system" fn() -> u32,
        )
        .extra_action_fn(|m| common::resolve_and_print(m, "GetTickCount"));
    #[cfg(windows)]
    let mut session: HookSession = unsafe { builder.install()? };

    unsafe { session.unhook_all()? };
    Ok(())
}
