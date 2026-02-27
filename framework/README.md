# dobby-rs-framework

[![crates.io](https://img.shields.io/crates/v/dobby-rs-framework.svg)](https://crates.io/crates/dobby-rs-framework)
[![docs.rs](https://img.shields.io/docsrs/dobby-rs-framework)](https://docs.rs/dobby-rs-framework)
[![License](https://img.shields.io/badge/License-Apache_2.0-red.svg)](../LICENSE)

Ergonomic hooking utilities built on top of `dobby-rs`.

Most users should depend on this crate (it pulls `dobby-rs` automatically).

## Install

```bash
cargo add dobby-rs-framework
```

## Highlights

- `StaticHook<T>`: global hook handle + typed `original()`
- `TypedHookHandle<T>` / `ReplaceHandle<T>`: typed handles for inline hooks/replacements
- Symbols + aliases: resolve + alias registry + hook-by-symbol

## Examples

Examples are in `examples/`:

- `logging.rs`
- `static_hook_basic.rs`
- `install_replace.rs`
- `module_params.rs`
- `symbols_aliases.rs`
- `inline_hooks_builder.rs`
- `inline_hooks_config.rs`
- `macros.rs`

Run one:

```bash
cargo run -p dobby-rs-framework --example static_hook_basic
```

## Safety

Hook installation is `unsafe`. The detour must match the target signature/ABI.
