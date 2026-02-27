use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnsupportedPlatform,
    NullPointer,
    InvalidInput,
    AlreadyHooked,
    HookNotFound,
    SymbolNotFound,
    DecodeFailed,
    RelocationFailed,
    EncodeFailed,
    PatchTooSmall,
    Unix(i32),
    Win32(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnsupportedPlatform => write!(f, "unsupported platform"),
            Error::NullPointer => write!(f, "null pointer"),
            Error::InvalidInput => write!(f, "invalid input"),
            Error::AlreadyHooked => write!(f, "target already hooked"),
            Error::HookNotFound => write!(f, "hook not found"),
            Error::SymbolNotFound => write!(f, "symbol not found"),
            Error::DecodeFailed => write!(f, "instruction decode failed"),
            Error::RelocationFailed => write!(f, "instruction relocation failed"),
            Error::EncodeFailed => write!(f, "instruction encode failed"),
            Error::PatchTooSmall => write!(f, "patch region too small"),
            Error::Unix(code) => write!(f, "unix error: {code}"),
            Error::Win32(code) => write!(f, "win32 error: {code}"),
        }
    }
}

impl std::error::Error for Error {}
