#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use dobby_rs::{
    Error, Result, code_patch, destroy, hook, import_table_replace, instrument,
    register_alloc_near_code_callback, resolve_symbol, set_near_trampoline, set_options,
    symbol_resolver,
};

pub mod framework;
pub mod hook_utils;
pub mod hooks;
pub mod logging;
pub mod symbols;

pub mod prelude {
    pub use crate::framework::{
        HookDef, HookSession, InlineHooksBuilder, InlineHooksConfig, ModuleHandle, inline_hooks,
        install_inline_hooks, make_hook, make_hook_simple,
    };
    pub use crate::hooks::{
        ReplaceHandle, StaticHook, TypedHookHandle, install, install_addr, install_with, replace,
    };
    pub use crate::logging::{LogLevel, LogOptions, LogOutput, init_logging};
    pub use crate::symbols::{
        get_alias, get_alias_info, hook_alias, hook_symbol, hook_symbol_default, hook_symbol_in,
        register_alias, register_alias_with_symbol, register_alias_with_symbol_in,
        resolve_and_register_alias, resolve_and_register_alias_in,
    };
}
