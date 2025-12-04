use clap::Parser as ClapParser;
use std::{fs, path::Path};

use finance_parser::{Args, get_extension, read_file, write_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = Path::new(&args.input_file);

    // args have priority over initial file extension thus format can be overriden
    let extension = get_extension(&args.input_extension, path)?;

    let data = read_file(path, extension)?;

    let output_file = format!("{}.{}", args.output_name, args.output_extension);

    let new_file = fs::File::create(output_file)?;

    write_data(&data, &new_file, args.output_extension)?;

    Ok(())
}
