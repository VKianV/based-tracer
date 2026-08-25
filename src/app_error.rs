use std::{
    error, fmt, io,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
};

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Config(ConfigError),
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "[I/O Failure] {e}"),
            Self::Config(e) => write!(f, "[Config Failure] {e}"),
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

#[derive(Debug)]
pub enum ConfigError {
    KeyNotFound(String),
    ParseF64(ParseFloatError),
    ParseU32(ParseIntError),
    ParseBool(ParseBoolError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyNotFound(key) => write!(f, "Configuration key '{key}' not found"),
            Self::ParseF64(e) => write!(f, "Failed to parse float: {e}"),
            Self::ParseU32(e) => write!(f, "Failed to parse integer: {e}"),
            Self::ParseBool(e) => write!(f, "Failed to parse boolean: {e}"),
        }
    }
}

impl error::Error for ConfigError {}

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
