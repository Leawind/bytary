use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;

pub mod convert;
mod error;

#[derive(Debug, Clone)]
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

pub struct FormattedWriter<W: Write> {
    inner: W,
    space_interval: usize,
    wrap_interval: usize,
    current_position: usize,
}

impl<W: Write> FormattedWriter<W> {
    pub fn new(inner: W, space_interval: usize, wrap_interval: usize) -> Self {
        Self {
            inner,
            space_interval,
            wrap_interval,
            current_position: 0,
        }
    }
}

impl<W: Write> Write for FormattedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &byte in buf {
            self.inner.write_all(&[byte])?;
            self.current_position += 1;

            if self.space_interval > 0 && self.current_position % self.space_interval == 0 {
                self.inner.write_all(b" ")?;
            }

            if self.wrap_interval > 0 && self.current_position % self.wrap_interval == 0 {
                self.inner.write_all(b"\n")?;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
