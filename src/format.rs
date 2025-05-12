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

impl From<&str> for Format {
    fn from(s: &str) -> Self {
        match s {
            "bytes" => Format::Bytes,
            "bin" => Format::Bin,
            "hex" => Format::Hex,
            "oct" => Format::Oct,
            "base32" => Format::Base32,
            "base64" => Format::Base64,
            _ => panic!("Invalid format {}", s),
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

impl Default for Format {
    fn default() -> Self {
        Format::Bytes
    }
}
