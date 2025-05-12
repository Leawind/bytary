use std::io;
use std::io::Write;

pub struct FormattedWriter<W: Write> {
    target: W,
    space_interval: usize,
    wrap_interval: usize,
    current_position: usize,
}

impl<W: Write> FormattedWriter<W> {
    pub fn new(target: W, space_interval: usize, wrap_interval: usize) -> Self {
        Self {
            target,
            space_interval,
            wrap_interval,
            current_position: 0,
        }
    }
}

impl<W: Write> Write for FormattedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for &byte in buf {
            self.target.write_all(&[byte])?;
            self.current_position += 1;

            if self.space_interval > 0 && self.current_position % self.space_interval == 0 {
                self.target.write_all(b" ")?;
            }

            if self.wrap_interval > 0 && self.current_position % self.wrap_interval == 0 {
                self.target.write_all(b"\n")?;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.target.flush()
    }
}
