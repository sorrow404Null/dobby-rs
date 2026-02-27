use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl From<LogLevel> for LevelFilter {
    fn from(v: LogLevel) -> Self {
        match v {
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LogOutput {
    Terminal,
    File(PathBuf),
    Both(PathBuf),
}

#[derive(Debug, Clone)]
pub struct LogOptions {
    pub level: LogLevel,
    pub output: LogOutput,
}
impl Default for LogOptions {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            output: LogOutput::Terminal,
        }
    }
}

#[derive(Debug)]
pub enum LoggingError {
    AlreadyInitialized,
    Io(std::io::Error),
    SetLogger(log::SetLoggerError),
}
impl core::fmt::Display for LoggingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoggingError::AlreadyInitialized => write!(f, "logger is already initialized"),
            LoggingError::Io(e) => write!(f, "io error: {e}"),
            LoggingError::SetLogger(e) => write!(f, "logger setup error: {e}"),
        }
    }
}
impl std::error::Error for LoggingError {}

static LOG_INIT: OnceLock<()> = OnceLock::new();
pub fn init_logging(options: LogOptions) -> Result<(), LoggingError> {
    if LOG_INIT.get().is_some() {
        return Err(LoggingError::AlreadyInitialized);
    }
    let mut b = ConfigBuilder::new();
    b.set_time_level(LevelFilter::Info)
        .set_thread_level(LevelFilter::Debug)
        .set_target_level(LevelFilter::Debug)
        .set_location_level(LevelFilter::Debug);
    b.set_time_format_rfc3339();
    let cfg = b.build();
    let level = LevelFilter::from(options.level);
    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();
    match options.output {
        LogOutput::Terminal => loggers.push(TermLogger::new(
            level,
            cfg.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )),
        LogOutput::File(path) => {
            loggers.push(WriteLogger::new(level, cfg.clone(), open_log_file(&path)?))
        }
        LogOutput::Both(path) => {
            loggers.push(TermLogger::new(
                level,
                cfg.clone(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ));
            loggers.push(WriteLogger::new(level, cfg.clone(), open_log_file(&path)?));
        }
    }
    CombinedLogger::init(loggers).map_err(LoggingError::SetLogger)?;
    let _ = LOG_INIT.set(());
    Ok(())
}

fn open_log_file(path: &Path) -> Result<File, LoggingError> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(LoggingError::Io)?;
    }
    File::create(path).map_err(LoggingError::Io)
}
