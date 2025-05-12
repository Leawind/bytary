use crate::format::Format;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BytaryError {
    #[error("Invalid Format: '{0}'")]
    InvalidFormat(String),

    #[error("Unsupported conversion: {0} => {1}")]
    UnsupportedConversion(Format, Format),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type BytaryResult<T> = Result<T, BytaryError>;
