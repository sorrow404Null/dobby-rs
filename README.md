# dobby-hook

English | [简体中文](README_zh-CN.md)

[![crates.io](https://img.shields.io/crates/v/dobby-hook.svg)](https://crates.io/crates/dobby-hook)
[![docs.rs](https://img.shields.io/docsrs/dobby-hook)](https://docs.rs/dobby-hook)
[![License](https://img.shields.io/badge/License-Apache_2.0-red.svg)](LICENSE)

Rust inline-hook core + a small framework inspired by [jmpews/Dobby](https://github.com/jmpews/Dobby).

This repository is a Rust rewrite/wrapper of the ideas (not the code) of Dobby. It is not affiliated with the upstream
Dobby project.

## Crates

- `dobby-hook`: facade crate; core APIs by default, optional framework via feature
- `dobby-hook-core`: low-level inline hook core (patch/relocate/trampoline)
- `dobby-rs-framework`: framework crate used by the facade feature `framework`

In practice, you usually only depend on `dobby-hook`.

## Install

Recommended (enable framework feature):

```bash
cargo add dobby-hook --features framework
```

Or in `Cargo.toml`:

```toml
[dependencies]
dobby-hook = { version = "0.1.2", features = ["framework"] }
```

Core only (no framework):

```bash
cargo add dobby-hook
```

Low-level core crate (if you really want only the core package):

```bash
cargo add dobby-hook-core
```

## Quick Start (Framework)

Note: the `prelude` module is available when you enable the dependency feature `framework`.

```rust
use dobby_hook::prelude::*;

static HOOK: StaticHook<fn(i32) -> i32> = StaticHook::new();

#[inline(never)]
fn target_add(x: i32) -> i32 {
    x + 1
}

#[inline(never)]
fn detour_add(x: i32) -> i32 {
    // EN: Detour has the same signature, so you already have the arguments.
    // CN: detour 和目标函数同签名，因此可以直接拿到参数。
    let x2 = x + 100;
    (HOOK.original())(x2) + 10
}

fn main() -> dobby_hook::Result<()> {
    let _ = init_logging(LogOptions::default());

    unsafe {
        HOOK.install(target_add as fn(i32) -> i32, detour_add as fn(i32) -> i32)?;
    }

    println!("{}", target_add(1));

    unsafe {
        HOOK.uninstall()?;
    }
    Ok(())
}
```

## Examples

Examples live in `framework/examples/` and cover the whole framework surface:

- `logging.rs`
- `static_hook_basic.rs`
- `install_replace.rs`
- `module_params.rs`
- `symbols_aliases.rs`
- `inline_hooks_builder.rs`
- `inline_hooks_config.rs`
- `macros.rs`
- `common.rs` (shared module for examples)

Run one:

```bash
cargo run -p dobby-rs-framework --example static_hook_basic
```

## Supported Targets

Implemented backends (current workspace state):

- Windows: `x86_64`
- Unix: `x86_64`, `aarch64`

## Safety

Inline hooking patches executable code at runtime. Misuse can crash the process or cause undefined behavior.

- Treat hook install/uninstall APIs as `unsafe`.
- Ensure detour ABI/signature matches the target.
- Use `#[inline(never)]` for demo targets.
- Be careful with concurrency.

## MSRV

MSRV is defined by `rust-version` in each crate (`dobby-rs/Cargo.toml`, `framework/Cargo.toml`, `dobby-hook/Cargo.toml`).

## License

This project is licensed under the Apache License 2.0.

- License text: `LICENSE`
- Attribution notice (if applicable): `NOTICE`
 
