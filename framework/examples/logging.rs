//! EN: Minimal logging setup.
//! CN: 最小日志初始化。

use dobby_rs_framework::prelude::*;

fn main() {
    // EN: `init_logging` is optional; hooks work without it.
    // CN: `init_logging` 是可选的；不初始化日志也能正常 hook。
    let _ = init_logging(LogOptions {
        level: LogLevel::Info,
        output: LogOutput::Terminal,
    });

    log::info!("hello from dobby-rs-framework example");
}
