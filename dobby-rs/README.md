# dobby-hook-core

[![crates.io](https://img.shields.io/crates/v/dobby-hook-core.svg)](https://crates.io/crates/dobby-hook-core)
[![docs.rs](https://img.shields.io/docsrs/dobby-hook-core)](https://docs.rs/dobby-hook-core)
[![License](https://img.shields.io/badge/License-Apache_2.0-red.svg)](https://github.com/sorrow404Null/dobby-rs/blob/master/LICENSE)

`dobby-hook-core` is the low-level inline hook core used by `dobby-hook`.

- Backends: Windows `x86_64`, Unix `x86_64`/`aarch64`
- Public API: `hook` / `destroy` / `code_patch` / `resolve_symbol`

Most users should start with `dobby-hook` unless you explicitly need the low-level primitives.

## Install

```bash
cargo add dobby-hook-core
```

## Minimal Usage

```rust,no_run
use core::ffi::c_void;

#[inline(never)]
fn target_add(x: i32) -> i32 {
    x + 1
}

#[inline(never)]
fn detour_add(x: i32) -> i32 {
    x + 100
}

fn main() -> dobby_hook_core::Result<()> {
    unsafe {
        // EN: `hook` returns a trampoline pointer for calling the original.
        // CN: `hook` 返回 trampoline 指针用于调用原函数。
        let trampoline = dobby_hook_core::hook(
            target_add as *const () as *mut c_void,
            detour_add as *const () as *mut c_void,
        )?;

        let _ = trampoline;
        dobby_hook_core::destroy(target_add as *const () as *mut c_void)?;
    }
    Ok(())
}
```

## Safety

All hook APIs are `unsafe` by nature. You must ensure correct ABI/signature, address validity, and be careful with
concurrency.

This crate is inspired by [jmpews/Dobby](https://github.com/jmpews/Dobby) (not affiliated).
