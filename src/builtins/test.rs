use crate::convert::ConversionGraph;
use crate::format::Format;
use std::io::{Cursor, Result};
use strum::IntoEnumIterator;

#[test]
fn test_builtins() -> Result<()> {
    use crate::format::Format::*;

    FromTo(Bin, Hex).expect_eq(b"0001 1011\n0011 0100\n", b"1b34")?;
    FromTo(Hex, Bin).expect_eq(b"1b34", b"0001101100110100")?;

    FromTo(Oct, Bytes).expect_eq(b"016070", &[0o16, 0o70])?;

    FromTo(Hex, Bytes).expect_eq(b"1b34", &[0x1b, 0x34])?;
    FromTo(Hex, Bytes).expect_ne(b"1b34", &[0x1b, 0x35])?;
    FromTo(Hex, Bytes).expect_eq(b"1b348fFf000e", &[0x1b, 0x34, 0x8f, 0xff, 0x00, 0x0e])?;

    Ok(())
}

#[test]
fn test_all() -> Result<()> {
    use crate::format::Format;

    let graph = ConversionGraph::builtins();

    let data = [
        0x00, 0xff, 0x01, 0x20, 0x17, 0x1b, 0x34, 0x41, 0x65, 0x8f, 0x0e,
    ];

    let from = Format::default();

    for to in Format::iter() {
        if to == from {
            continue;
        }
        let forward = graph.get_converter(&from, &to);
        let backward = graph.get_converter(&to, &from);

        if forward.is_none() || backward.is_none() {
            continue;
        }

        println!("Testing {} <-> {}", from, to);

        let forward = forward.unwrap();
        let backward = backward.unwrap();

        let mut input = Vec::from(data);
        let mut output = Vec::new();

        forward(&mut Cursor::new(input.clone()), &mut output)?;
        input.clear();
        backward(&mut Cursor::new(output.clone()), &mut input)?;

        assert_eq!(input, data);
    }

    Ok(())
}

struct FromTo(Format, Format);
impl FromTo {
    fn output(&self, input: &[u8]) -> Result<Vec<u8>> {
        let converter = ConversionGraph::builtins()
            .get_converter(&self.0, &self.1)
            .unwrap();
        let mut output = Vec::new();
        converter(&mut Cursor::new(input), &mut output)?;
        Ok(output)
    }

    fn expect_eq(&self, input: &[u8], expect_output: &[u8]) -> Result<()> {
        assert_eq!(self.output(input)?, expect_output);
        Ok(())
    }
    fn expect_ne(&self, input: &[u8], expect_output: &[u8]) -> Result<()> {
        assert_ne!(self.output(input)?, expect_output);
        Ok(())
    }
}
