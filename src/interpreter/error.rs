use std::{io::{Error, ErrorKind}, num::ParseFloatError};

#[derive(Debug)]
pub enum CalcError {
    Unexpected(String),
    Unknown(String),
    Failed(String),
    IoError(Error),
}

#[derive(Debug)]
pub enum CalcErrorKind {
    Unexpected,
    Unknown,
    Failed,
    IoError,
}

impl CalcError {
    pub fn kind(&self) -> CalcErrorKind {
        match self {
            CalcError::Unexpected(_) => CalcErrorKind::Unexpected,
            CalcError::Unknown(_) => CalcErrorKind::Unknown,
            CalcError::Failed(_) => CalcErrorKind::Failed,
            CalcError::IoError(_) => CalcErrorKind::IoError,
        }
    }
}

impl std::error::Error for CalcError {}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::Unexpected(msg) => write!(f, "Unexpected: {}", msg),
            CalcError::Unknown(msg) => write!(f, "Unknown: {}", msg),
            CalcError::Failed(msg) => write!(f, "Failed: {}", msg),
            CalcError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<Error> for CalcError {
    fn from(error: Error) -> Self {
        CalcError::IoError(error)
    }
}
impl From<ParseFloatError> for CalcError {
    fn from(error: ParseFloatError) -> Self {
        CalcError::IoError(Error::new(ErrorKind::Other, error.to_string()))
    }
}
