mod args;

use args::Args;
use clap::Parser as ClapParser;
use finance_parser::{get_extension, read_file};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let first_file_path = Path::new(&args.first_file);
    let second_file_path = Path::new(&args.second_file);

    let first_ext = get_extension(&args.first_ext, first_file_path)?;

    let second_ext = get_extension(&args.second_ext, second_file_path)?;

    let first_file_content = read_file(first_file_path, first_ext)?;

    let second_file_content = read_file(second_file_path, second_ext)?;

    match first_file_content == second_file_content {
        true => println!(
            "Files {} and {} are identical",
            args.first_file, args.second_file
        ),
        false => println!(
            "Error: files {} and {} are not identical",
            args.first_file, args.second_file
        ),
    }

    Ok(())
}
