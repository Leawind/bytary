use crate::Format;
use crate::error::ConvertError;
use regex::Regex;
use std::io;
use std::io::{Read, Write};

pub fn convert<R: Read, W: Write>(
    from: &Format,
    to: &Format,
    input: &mut R,
    output: &mut W,
) -> crate::error::Result {
    match (from, to) {
        (Format::Bytes, Format::Hex) => Ok(bytes_to_hex(input, output)?),
        (Format::Hex, Format::Bytes) => Ok(hex_to_bytes(input, output)?),
        _ => Err(ConvertError::UnsupportedConversion(
            from.clone(),
            to.clone(),
        )),
    }
}

pub type ConverterFn = fn(&mut dyn Read, &mut dyn Write) -> crate::error::Result;

pub fn bytes_to_hex<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);
    let mut buffer = [0u8; 1024]; // 每次读取 1KB

    loop {
        let length = reader.read(&mut buffer)?;
        if length == 0 {
            break;
        }
        let hex_str = hex::encode(&buffer[..length]);
        writer.write_all(hex_str.as_bytes())?;
    }
    Ok(())
}

pub fn hex_to_bytes<R: Read, W: Write>(input: R, output: W) -> io::Result<()> {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);
    let mut buffer = String::new();

    let re = Regex::new(r"[\n\r\t]+").unwrap();
    while reader.read_to_string(&mut buffer)? > 0 {
        match hex::decode(re.replace_all(&buffer, "").as_ref()) {
            Ok(bytes) => writer.write_all(&bytes)?,
            Err(e) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid hex string: {}", e),
                ));
            }
        }
        buffer.clear();
    }
    Ok(())
}
