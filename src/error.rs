use crate::format::Format;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BytaryError {
    #[error("Invalid Format: '{0}'")]
    InvalidFormat(String),

    #[error("Unsupported conversion: {0} => {1}")]
    UnsupportedConversion(Format, Format),

    #[error("Invalid input data: {0}")]
    InvalidInputData(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl BytaryError {
    pub fn err<T>(self) -> BytaryResult<T> {
        Err(self)
    }
}

pub type BytaryResult<T> = Result<T, BytaryError>;
