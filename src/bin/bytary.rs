use bytary::convert::convert;
use bytary::{Format, FormattedWriter};
use clap::Parser;
use std::io;

/// Usage:
///
/// ```sh
/// bytary <from> <to>
/// ```
///
/// ```sh
/// bytary <to>
/// ```
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct BytaryCli {
    /// Output format
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
    #[arg(short, long = "wrap", default_value_t = 0)]
    pub wrap_interval: usize,

    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

fn main() {
    let args = BytaryCli::parse();

    let to = Format::from(args.to.as_str());
    let from = Format::from(args.from.as_str());

    if args.verbose {
        eprintln!("Convert: {:?} ==> {:?}", from, to);
        eprintln!(
            "Formatting: space every {} bytes, break line every {} bytes",
            args.space_interval, args.wrap_interval
        );
    }

    let mut writer = FormattedWriter::new(io::stdout(), args.space_interval, args.wrap_interval);
    match convert(&from, &to, &mut io::stdin(), &mut writer) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
