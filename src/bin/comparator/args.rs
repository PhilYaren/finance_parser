use clap::Parser;
use finance_parser::Extension;

#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    /// first file to compare
    #[arg(long)]
    pub first_file: String,

    /// extension of the first file
    #[arg(long)]
    pub first_ext: Option<Extension>,

    /// second file to compare
    #[arg(long)]
    pub second_file: String,

    /// extension of the second file
    #[arg(long)]
    pub second_ext: Option<Extension>,
}
