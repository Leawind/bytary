use crate::format::Format;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("Unsupported conversion: {0} => {1}")]
    UnsupportedConversion(Format, Format),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type BytaryResult = Result<(), ConvertError>;
