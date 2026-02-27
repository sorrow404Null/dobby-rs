//! EN: Manual config via `InlineHooksConfig` + `install_inline_hooks`.
//! CN: 通过 `InlineHooksConfig` 手工配置并调用 `install_inline_hooks`。

mod common;
use dobby_rs_framework::prelude::*;

// EN: Detours are shared via `common::detours`.
// CN: detour 通过 `common::detours` 共享。

fn main() -> dobby_rs_framework::Result<()> {
    common::init_example_logging();

    // EN: Build HookDef(s) using helpers.
    // CN: 使用 helper 构造 HookDef。
    #[cfg(unix)]
    let hooks: Vec<HookDef> = vec![make_hook(
        "puts",
        "puts_cfg",
        common::detours::unix::detour_puts
            as unsafe extern "C" fn(*const core::ffi::c_char) -> core::ffi::c_int,
    )];
    #[cfg(windows)]
    let hooks: Vec<HookDef> = vec![make_hook_simple(
        "GetCurrentProcessId",
        common::detours::windows::detour_get_current_process_id
            as unsafe extern "system" fn() -> u32,
    )];

    let mut session: HookSession = unsafe {
        install_inline_hooks(InlineHooksConfig {
            lib_name: common::DEMO_LIB,
            hooks,
            extra_action: Some(Box::new(|m: &ModuleHandle| {
                // EN: `ModuleHandle` is available here.
                // CN: 这里可以拿到 `ModuleHandle`。
                let sym = if cfg!(windows) {
                    "GetCurrentProcessId"
                } else {
                    "puts"
                };
                common::resolve_and_print(m, sym)
            })),
        })?
    };

    unsafe { session.unhook_all()? };
    Ok(())
}
