#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

mod arch;
mod engine;
mod error;
mod options;
mod platform;

pub use crate::engine::{
    code_patch, destroy, hook, import_table_replace, instrument, resolve_symbol, symbol_resolver,
};
pub use crate::error::{Error, Result};
pub use crate::options::{register_alloc_near_code_callback, set_near_trampoline, set_options};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
