use bytary::convert::ConversionGraph;
use bytary::error::{BytaryResult, ConvertError};
use bytary::format::Format;
use bytary::utils::FormattedWriter;
use clap::Parser;
use std::io;
use strum::IntoEnumIterator;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct BytaryArgs {
    /// List all supported formats and exit
    ///
    /// If set, all other arguments are ignored.
    #[arg(short, long, default_value_t = false)]
    pub list_formats: bool,

    /// Output format
    #[arg(default_value = "bytes")]
    pub to: String,

    /// Input format
    ///
    /// Default is bytes
    #[arg(default_value = "bytes")]
    pub from: String,

    /// Space interval between bytes
    ///
    /// 0 means no space
    #[arg(short, long = "space", default_value_t = 0)]
    pub space_interval: usize,

    /// Line wrap interval
    ///
    /// 0 means no line wrap
    #[arg(short, long = "wrap", default_value_t = 0)]
    pub wrap_interval: usize,

    /// Use verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

fn bytary_cli(
    args: BytaryArgs,
    input: &mut dyn io::Read,
    output: &mut dyn io::Write,
) -> BytaryResult {
    let graph = ConversionGraph::builtins();

    if args.list_formats {
        println!(
            "Available formats: {}",
            Format::iter()
                .filter(|to| graph.can_convert_both(&Format::default(), &to))
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        return Ok(());
    }

    let to = Format::from(args.to.as_str());
    let from = Format::from(args.from.as_str());

    let path = graph
        .find_shortest_path(&from, &to)
        .ok_or(ConvertError::UnsupportedConversion(from, to))?;

    let converters = graph.path_to_converters(&path).unwrap();

    if args.verbose {
        if converters.is_empty() {
            eprintln!("Operation: Copy data")
        } else {
            eprintln!(
                "Operation: {}",
                path.iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(" => ")
            )
        }
        eprintln!(
            "Formatting: space every {} bytes, break line every {} bytes",
            args.space_interval, args.wrap_interval
        );
    }

    let converter = match converters.is_empty() {
        true => ConversionGraph::get_copy_converter(),
        false => ConversionGraph::compose(converters),
    };

    let mut writer = FormattedWriter::new(output, args.space_interval, args.wrap_interval);
    Ok(converter(input, &mut writer)?)
}

fn main() {
    if let Err(e) = bytary_cli(BytaryArgs::parse(), &mut io::stdin(), &mut io::stdout()) {
        eprintln!("{}", e);
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::io::Cursor;

    #[test]
    pub fn test_cli_bytes_to_hex() {
        let mut output = Vec::new();
        bytary_cli(
            BytaryArgs {
                list_formats: false,
                to: "hex".to_string(),
                from: "bytes".to_string(),
                space_interval: 0,
                wrap_interval: 0,
                verbose: true,
            },
            &mut Cursor::new(vec![0x1b, 0x34, 0x8f, 0xff, 0x00, 0x0e]),
            &mut output,
        )
        .unwrap();
        assert_eq!(output, b"1b348fff000e");
    }

    #[test]
    pub fn test_bytes_to_bytes() {
        let mut output = Vec::new();
        let data = [0x1b, 0x34, 0x8f, 0xff, 0x00, 0x0e];
        bytary_cli(
            BytaryArgs {
                list_formats: false,
                to: "bytes".to_string(),
                from: "bytes".to_string(),
                space_interval: 0,
                wrap_interval: 0,
                verbose: true,
            },
            &mut Cursor::new(&data),
            &mut output,
        )
        .unwrap();
        assert_eq!(output, &data);
    }
}
