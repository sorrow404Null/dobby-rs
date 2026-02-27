use crate::hook_utils::HookHandle;
use crate::{Error, Result, hook_utils, resolve_symbol};
use core::ffi::{CStr, c_void};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SymbolAlias {
    pub alias: String,
    pub symbol_name: Option<String>,
    pub image_name: Option<String>,
    address: usize,
}
impl SymbolAlias {
    pub fn address(&self) -> *mut c_void {
        self.address as *mut c_void
    }
}

static ALIASES: OnceCell<Mutex<HashMap<String, SymbolAlias>>> = OnceCell::new();
fn alias_map() -> &'static Mutex<HashMap<String, SymbolAlias>> {
    ALIASES.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_alias(alias: impl Into<String>, address: *mut c_void) -> Result<()> {
    if address.is_null() {
        return Err(Error::NullPointer);
    }
    let alias = alias.into();
    alias_map().lock().unwrap().insert(
        alias.clone(),
        SymbolAlias {
            alias,
            symbol_name: None,
            image_name: None,
            address: address as usize,
        },
    );
    Ok(())
}
pub fn register_alias_with_symbol(
    alias: impl Into<String>,
    image_name: Option<&CStr>,
    symbol_name: &CStr,
    address: *mut c_void,
) -> Result<()> {
    if address.is_null() {
        return Err(Error::NullPointer);
    }
    let alias = alias.into();
    alias_map().lock().unwrap().insert(
        alias.clone(),
        SymbolAlias {
            alias,
            symbol_name: Some(symbol_name.to_string_lossy().into_owned()),
            image_name: image_name.map(|s| s.to_string_lossy().into_owned()),
            address: address as usize,
        },
    );
    Ok(())
}
pub fn register_alias_with_symbol_in(
    alias: impl Into<String>,
    image_name: &CStr,
    symbol_name: &CStr,
    address: *mut c_void,
) -> Result<()> {
    register_alias_with_symbol(alias, Some(image_name), symbol_name, address)
}
pub fn get_alias(name: &str) -> Option<*mut c_void> {
    alias_map()
        .lock()
        .unwrap()
        .get(name)
        .map(SymbolAlias::address)
}
pub fn get_alias_info(name: &str) -> Option<SymbolAlias> {
    alias_map().lock().unwrap().get(name).cloned()
}
pub fn resolve_and_register_alias(
    alias: impl Into<String>,
    image_name: Option<&CStr>,
    symbol_name: &CStr,
) -> Result<*mut c_void> {
    let ptr = resolve_symbol(image_name, symbol_name);
    if ptr.is_null() {
        return Err(Error::SymbolNotFound);
    }
    register_alias_with_symbol(alias, image_name, symbol_name, ptr)?;
    Ok(ptr)
}
pub fn resolve_and_register_alias_in(
    alias: impl Into<String>,
    image_name: &CStr,
    symbol_name: &CStr,
) -> Result<*mut c_void> {
    resolve_and_register_alias(alias, Some(image_name), symbol_name)
}
pub unsafe fn hook_alias(name: &str, detour: *mut c_void) -> Result<HookHandle> {
    let target = get_alias(name).ok_or(Error::SymbolNotFound)?;
    hook_utils::install_addr(target, detour)
}
pub unsafe fn hook_symbol(
    image_name: Option<&CStr>,
    symbol_name: &CStr,
    detour: *mut c_void,
    alias: Option<&str>,
) -> Result<HookHandle> {
    if detour.is_null() {
        return Err(Error::NullPointer);
    }
    let target = resolve_symbol(image_name, symbol_name);
    if target.is_null() {
        return Err(Error::SymbolNotFound);
    }
    if let Some(alias) = alias {
        register_alias_with_symbol(alias, image_name, symbol_name, target)?;
    }
    hook_utils::install_addr(target, detour)
}
pub unsafe fn hook_symbol_default(
    symbol_name: &CStr,
    detour: *mut c_void,
    alias: Option<&str>,
) -> Result<HookHandle> {
    hook_symbol(None, symbol_name, detour, alias)
}
pub unsafe fn hook_symbol_in(
    image_name: &CStr,
    symbol_name: &CStr,
    detour: *mut c_void,
    alias: Option<&str>,
) -> Result<HookHandle> {
    hook_symbol(Some(image_name), symbol_name, detour, alias)
}
