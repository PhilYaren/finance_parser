//! mod with cli args of internal cli app
use crate::Extension;
use clap::Parser;

/// Command-line arguments for the converter binary.
///
/// This struct is exposed by the library as a convenience when
/// building custom frontends or wrappers around the core parser.
#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    /// Input file you want to convert.
    #[arg(long)]
    pub input_file: String,

    /// Optional: file format of the input, if you don't want to infer it from extension.
    #[arg(long)]
    pub input_extension: Option<Extension>,

    /// Base name of the output file (without extension).
    #[arg(long)]
    pub output_name: String,

    /// Output file format.
    #[arg(long = "to-format", short)]
    pub output_extension: Extension,
}
