use crate::Extension;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    /// input file which you'd like to convert
    #[arg(long)]
    pub input_file: String,

    /// Optional: specify format manualy
    #[arg(long)]
    pub input_extension: Option<Extension>,

    /// output file name
    #[arg(long)]
    pub output_name: String,

    /// output file extension
    #[arg(long = "to-format", short)]
    pub output_extension: Extension,
}
