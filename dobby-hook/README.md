# dobby-hook

[![crates.io](https://img.shields.io/crates/v/dobby-hook.svg)](https://crates.io/crates/dobby-hook)
[![docs.rs](https://img.shields.io/docsrs/dobby-hook)](https://docs.rs/dobby-hook)
[![License](https://img.shields.io/badge/License-Apache_2.0-red.svg)](https://github.com/sorrow404Null/dobby-rs/blob/master/LICENSE)

Facade crate for the `dobby-hook-core` inline-hook engine, with an optional higher-level framework.

This project is inspired by [jmpews/Dobby](https://github.com/jmpews/Dobby) (ideas, not code) and is not affiliated with it.

## Install

Recommended (enable framework feature):

```toml
[dependencies]
dobby-hook = { version = "0.1.2", features = ["framework"] }
```

Core only:

```toml
[dependencies]
dobby-hook = "0.1.2"
```

## What You Get

- Default: re-exports/typed access to the low-level core (`dobby-hook-core`).
- Feature `framework`: ergonomic utilities (`StaticHook<T>`, typed handles, symbol helpers, etc.).

## Quick Start (Framework)

```rust
use dobby_hook::prelude::*;

static HOOK: StaticHook<fn(i32) -> i32> = StaticHook::new();

#[inline(never)]
fn target_add(x: i32) -> i32 {
    x + 1
}

#[inline(never)]
fn detour_add(x: i32) -> i32 {
    (HOOK.original())(x + 100) + 10
}

fn main() -> dobby_hook::Result<()> {
    unsafe { HOOK.install(target_add, detour_add)?; }
    let _ = target_add(1);
    unsafe { HOOK.uninstall()?; }
    Ok(())
}
```

## Examples

The full set of runnable examples lives in the repository:

- GitHub: https://github.com/sorrow404Null/dobby-rs/tree/master/framework/examples
- Run one: `cargo run -p dobby-rs-framework --example static_hook_basic`

## Project Overview

- Repository README: https://github.com/sorrow404Null/dobby-rs#readme

## Safety

Inline hooking patches executable code at runtime. Misuse can crash the process or cause undefined behavior.
