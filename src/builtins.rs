use crate::convert::ConversionGraph;
use crate::format::Format;
use regex::Regex;
use std::io;
use std::io::{Read, Write};

#[cfg(test)]
mod test;

impl ConversionGraph {
    /// Create a new graph with built-in conversions.
    pub fn builtins() -> Self {
        let mut graph = ConversionGraph::new();
        graph.add_direct(Format::Bytes, Format::Bin, bytes_to_bin, 1);
        graph.add_direct(Format::Bin, Format::Hex, bin_to_hex, 1);

        graph.add_direct(Format::Bytes, Format::Oct, bytes_to_oct, 1);
        graph.add_direct(Format::Oct, Format::Bytes, oct_to_bytes, 1);

        graph.add_direct(Format::Bytes, Format::Hex, bytes_to_hex, 1);
        graph.add_direct(Format::Hex, Format::Bytes, hex_to_bytes, 1);

        graph
    }
}

pub fn bytes_to_bin(input: &mut dyn Read, output: &mut dyn Write) -> io::Result<()> {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);
    let mut buffer = [0u8; 1024];

    loop {
        let length = reader.read(&mut buffer)?;
        if length == 0 {
            break;
        }

        let bin_str = buffer[..length]
            .iter()
            .map(|byte| format!("{:08b}", byte))
            .collect::<String>();

        writer.write_all(bin_str.as_bytes())?;
    }
    Ok(())
}

pub fn bin_to_hex(input: &mut dyn Read, output: &mut dyn Write) -> io::Result<()> {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);
    let mut buffer = String::new();

    let re = Regex::new(r"[^01]").unwrap();

    while reader.read_to_string(&mut buffer)? > 0 {
        let clean_bin = re.replace_all(&buffer, "");

        if clean_bin.is_empty() {
            buffer.clear();
            continue;
        }

        let chunks = clean_bin
            .as_bytes()
            .chunks(4)
            .map(|chunk| {
                let mut padded = String::new();
                if chunk.len() < 4 {
                    padded.push_str(&"0".repeat(4 - chunk.len()));
                }
                padded.push_str(std::str::from_utf8(chunk).unwrap());
                padded
            })
            .collect::<Vec<String>>();

        let hex_str = chunks
            .iter()
            .map(|bin4| {
                u8::from_str_radix(bin4, 2)
                    .map(|n| format!("{:x}", n))
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<io::Result<String>>()?;

        writer.write_all(hex_str.as_bytes())?;
        buffer.clear();
    }

    Ok(())
}
pub fn bytes_to_oct(input: &mut dyn Read, output: &mut dyn Write) -> io::Result<()> {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);
    let mut buffer = [0u8; 1024];

    loop {
        let length = reader.read(&mut buffer)?;
        if length == 0 {
            break;
        }

        let oct_str = buffer[..length]
            .iter()
            .map(|byte| format!("{:03o}", byte)) // Each byte as 3-digit octal
            .collect::<String>();

        writer.write_all(oct_str.as_bytes())?;
    }
    Ok(())
}

pub fn oct_to_bytes(input: &mut dyn Read, output: &mut dyn Write) -> io::Result<()> {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);
    let mut buffer = String::new();

    // Remove any non-octal digits (0-7)
    let re = Regex::new(r"[^0-7]").unwrap();

    while reader.read_to_string(&mut buffer)? > 0 {
        let clean_oct = re.replace_all(&buffer, "");

        if clean_oct.is_empty() {
            buffer.clear();
            continue;
        }

        // Process in chunks of 3 digits (since each byte is represented by 3 octal digits)
        let chunks = clean_oct
            .as_bytes()
            .chunks(3)
            .map(|chunk| {
                let oct_str = std::str::from_utf8(chunk)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                // Handle cases where the last chunk might be shorter than 3 digits
                let oct_str = if oct_str.len() < 3 {
                    // Pad with leading zeros if needed
                    format!("{:0<3}", oct_str)
                } else {
                    oct_str.to_string()
                };

                u8::from_str_radix(&oct_str, 8)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
            .collect::<io::Result<Vec<u8>>>()?;

        writer.write_all(&chunks)?;
        buffer.clear();
    }

    Ok(())
}
pub fn bytes_to_hex(input: &mut dyn Read, output: &mut dyn Write) -> io::Result<()> {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);
    let mut buffer = [0u8; 1024];

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

pub fn hex_to_bytes(input: &mut dyn Read, output: &mut dyn Write) -> io::Result<()> {
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
