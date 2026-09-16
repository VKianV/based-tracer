use std::{
    fmt, io,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

// app errors
#[derive(Debug)]
pub enum AppError {
    Config(ConfigError),
    Thread(io::Error),
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thread(e) => write!(f, "[Thread Failure] A rendering thread panicked {e}"),
            Self::Config(e) => write!(f, "[Config Failure] {e}"),
            Self::Io(e) => write!(f, "[I/O Failure] {e}"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        Self::Config(e)
    }
}

// config errors
#[derive(Debug)]
pub enum ConfigError {
    ParseF64(ParseFloatError),
    ParseBool(ParseBoolError),
    ParseU32(ParseIntError),
    FileNotFound(String),
    KeyNotFound(String),
    Io(io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound(path) => write!(f, "Configuration file '{path}' not found"),
            Self::KeyNotFound(key) => write!(f, "Configuration key '{key}' not found"),
            Self::ParseBool(e) => write!(f, "Failed to parse boolean: {e}"),
            Self::ParseU32(e) => write!(f, "Failed to parse integer: {e}"),
            Self::ParseF64(e) => write!(f, "Failed to parse float: {e}"),
            Self::Io(e) => write!(f, "I/O error reading config: {e}"),
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ParseFloatError> for ConfigError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseF64(e)
    }
}

impl From<ParseBoolError> for ConfigError {
    fn from(e: ParseBoolError) -> Self {
        Self::ParseBool(e)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseU32(e)
    }
}
