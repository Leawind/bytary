use crate::error::{BytaryError, BytaryResult};
use std::fmt::{Display, Formatter};
use strum::EnumIter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, EnumIter)]
pub enum Format {
    Bytes,
    Bin,
    Hex,
    Oct,
    Base32,
    Base64,
}
impl Default for Format {
    fn default() -> Self {
        Format::Bytes
    }
}

impl TryFrom<&str> for Format {
    type Error = BytaryError;
    fn try_from(name: &str) -> BytaryResult<Self> {
        match name {
            "bytes" => Ok(Format::Bytes),
            "bin" => Ok(Format::Bin),
            "hex" => Ok(Format::Hex),
            "oct" => Ok(Format::Oct),
            "base32" => Ok(Format::Base32),
            "base64" => Ok(Format::Base64),
            _ => Err(BytaryError::InvalidFormat(name.to_string())),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Bytes => write!(f, "bytes"),
            Format::Bin => write!(f, "bin"),
            Format::Hex => write!(f, "hex"),
            Format::Oct => write!(f, "oct"),
            Format::Base32 => write!(f, "base32"),
            Format::Base64 => write!(f, "base64"),
        }
    }
}
