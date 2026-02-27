mod module;
pub mod params;

use crate::hook_utils::HookHandle;
use crate::symbols;
use crate::{Error, Result, hooks};
use core::ffi::c_void;
use std::ffi::CString;

pub use module::ModuleHandle;

pub struct HookDef {
    pub symbol: String,
    pub alias: Option<String>,
    pub detour: *mut c_void,
}

unsafe fn fn_to_ptr<F: Copy>(f: F) -> *mut c_void {
    debug_assert_eq!(
        core::mem::size_of::<F>(),
        core::mem::size_of::<*mut c_void>()
    );
    let raw: *const () = core::mem::transmute_copy(&f);
    raw as *mut c_void
}

pub fn make_hook<F: Copy>(symbol: &str, alias: &str, detour: F) -> HookDef {
    let detour = unsafe { fn_to_ptr(detour) };
    HookDef {
        symbol: symbol.to_owned(),
        alias: Some(alias.to_owned()),
        detour,
    }
}

pub fn make_hook_simple<F: Copy>(symbol: &str, detour: F) -> HookDef {
    let detour = unsafe { fn_to_ptr(detour) };
    HookDef {
        symbol: symbol.to_owned(),
        alias: None,
        detour,
    }
}

pub type ExtraAction<'a> = Box<dyn Fn(&ModuleHandle) -> Result<()> + Send + Sync + 'a>;

pub struct InlineHooksConfig<'a> {
    pub lib_name: &'a str,
    pub hooks: Vec<HookDef>,
    pub extra_action: Option<ExtraAction<'a>>,
}

pub struct InstalledHook {
    pub symbol: String,
    pub alias: Option<String>,
    pub address: *mut c_void,
    handle: Option<HookHandle>,
}

pub struct HookSession {
    pub module: ModuleHandle,
    pub installed: Vec<InstalledHook>,
}

impl HookSession {
    pub unsafe fn unhook_all(&mut self) -> Result<()> {
        for item in &mut self.installed {
            if let Some(handle) = item.handle.take() {
                handle.unhook()?;
            }
        }
        Ok(())
    }
}

pub struct InlineHooksBuilder<'a> {
    lib_name: &'a str,
    hooks: Vec<HookDef>,
    extra_action: Option<ExtraAction<'a>>,
}

impl<'a> InlineHooksBuilder<'a> {
    pub fn new(lib_name: &'a str) -> Self {
        Self {
            lib_name,
            hooks: Vec::new(),
            extra_action: None,
        }
    }
    pub fn hook_install<F: Copy>(mut self, symbol: &str, detour: F) -> Self {
        self.hooks.push(make_hook_simple(symbol, detour));
        self
    }
    pub fn hook_install_alias<F: Copy>(mut self, symbol: &str, alias: &str, detour: F) -> Self {
        self.hooks.push(make_hook(symbol, alias, detour));
        self
    }
    pub fn hook<F: Copy>(self, symbol: &str, detour: F) -> Self {
        self.hook_install(symbol, detour)
    }
    pub fn hook_alias<F: Copy>(self, symbol: &str, alias: &str, detour: F) -> Self {
        self.hook_install_alias(symbol, alias, detour)
    }
    pub fn extra_action<F>(mut self, action: F) -> Self
    where
        F: Fn(&ModuleHandle) -> Result<()> + Send + Sync + 'a,
    {
        self.extra_action = Some(Box::new(action));
        self
    }
    pub fn extra_action_fn(mut self, action: fn(&ModuleHandle) -> Result<()>) -> Self {
        self.extra_action = Some(Box::new(action));
        self
    }
    pub unsafe fn install(self) -> Result<HookSession> {
        install_inline_hooks(InlineHooksConfig {
            lib_name: self.lib_name,
            hooks: self.hooks,
            extra_action: self.extra_action,
        })
    }
}

pub fn inline_hooks(lib_name: &str) -> InlineHooksBuilder<'_> {
    InlineHooksBuilder::new(lib_name)
}

pub unsafe fn install_inline_hooks(config: InlineHooksConfig<'_>) -> Result<HookSession> {
    let module = ModuleHandle::open(config.lib_name)?;
    let mut installed = Vec::with_capacity(config.hooks.len());
    for hook in config.hooks {
        if hook.detour.is_null() {
            return Err(Error::NullPointer);
        }
        let symbol_c = CString::new(hook.symbol.as_str()).map_err(|_| Error::InvalidInput)?;
        let target = module.resolve(&symbol_c).ok_or(Error::SymbolNotFound)?;
        if let Some(alias) = hook.alias.clone() {
            symbols::register_alias_with_symbol(
                alias,
                Some(module.lib_name_cstr()),
                &symbol_c,
                target,
            )?;
        }
        let handle = hooks::install_addr(target, hook.detour)?;
        installed.push(InstalledHook {
            symbol: hook.symbol,
            alias: hook.alias,
            address: target,
            handle: Some(handle),
        });
    }
    if let Some(extra) = config.extra_action {
        extra(&module)?;
    }
    Ok(HookSession { module, installed })
}
