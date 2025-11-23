use crate::Extension;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    /// input file which you'd like to convert
    #[arg(long)]
    pub input_file: String,

    /// output file name
    #[arg(long)]
    pub output_name: String,

    /// output file extension
    #[arg(long = "to-format", short)]
    pub extension: Extension,
}
