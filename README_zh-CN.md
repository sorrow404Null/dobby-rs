# dobby-hook

[English](README.md) | 简体中文

[![crates.io](https://img.shields.io/crates/v/dobby-hook.svg)](https://crates.io/crates/dobby-hook)
[![docs.rs](https://img.shields.io/docsrs/dobby-hook)](https://docs.rs/dobby-hook)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

基于 [jmpews/Dobby](https://github.com/jmpews/Dobby) 思路实现的 Rust inline-hook 核心 + 上层框架。

说明：本项目是 Rust 侧的重写/封装，思路参考 Dobby，但不包含上游 Dobby 的代码，且与上游项目无隶属关系。

## Crates

- `dobby-hook`: 门面 crate；默认提供 core API，通过 feature 可选开启 framework
- `dobby-hook-core`: 底层 inline hook 核心（patch/relocate/trampoline）
- `dobby-rs-framework`: framework crate（由 `dobby-hook` 的 `framework` feature 拉取）

实际使用中通常只需要依赖 `dobby-hook`。

## 安装

推荐（开启 framework feature）：

```bash
cargo add dobby-hook --features framework
```

或在 `Cargo.toml` 中写：

```toml
[dependencies]
dobby-hook = { version = "0.1.1", features = ["framework"] }
```

只用 core（不开启 framework）：

```bash
cargo add dobby-hook
```

只依赖底层 core 包（一般不需要）：

```bash
cargo add dobby-hook-core
```

## 快速开始（Framework）

注意：`prelude` 只有在依赖开启 `framework` feature 时才会提供。

```rust
use dobby_hook::prelude::*;

static HOOK: StaticHook<fn(i32) -> i32> = StaticHook::new();

#[inline(never)]
fn target_add(x: i32) -> i32 {
    x + 1
}

#[inline(never)]
fn detour_add(x: i32) -> i32 {
    // detour 和 target 同签名，可以直接拿到参数
    let x2 = x + 100;
    // 调用 original，并对返回值做处理
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

## 示例（Framework）

示例都在 `framework/examples/`，尽量简单，并覆盖 framework 的主要用法：

- `logging.rs`
- `static_hook_basic.rs`
- `install_replace.rs`
- `module_params.rs`
- `symbols_aliases.rs`
- `inline_hooks_builder.rs`
- `inline_hooks_config.rs`
- `macros.rs`
- `common.rs`（示例共享模块）

运行一个示例：

```bash
cargo run -p dobby-rs-framework --example static_hook_basic
```

## 平台 / 架构支持

当前实现的后端：

- Windows: `x86_64`
- Unix: `x86_64`、`aarch64`

## 安全性

Inline hooking 会在运行期修改可执行代码，误用会导致崩溃或 UB：

- 把安装/卸载 hook 视为 `unsafe` 操作
- detour 必须与 target 的 ABI/签名完全一致
- Demo/测试建议 `#[inline(never)]`
- 注意并发风险

## MSRV

MSRV 以各 crate 的 `rust-version` 为准（`dobby-rs/Cargo.toml`、`framework/Cargo.toml`、`dobby-hook/Cargo.toml`）。

## 开源协议

本项目采用 Apache License 2.0。

- 协议文本：`LICENSE`
- 归属声明（如适用）：`NOTICE`
