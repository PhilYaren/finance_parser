use clap::Parser as ClapParser;
use std::{fs, path::Path};

use finance_parser::{Args, get_extension, read_file, write_data};

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.input_file);

    // args have priority over initial file extension thus format can be overriden
    let extension = match get_extension(&args.input_extension, path) {
        Ok(ext) => ext,
        Err(err) => {
            println!("{}", err);

            return;
        }
    };

    let data = match read_file(path, extension) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);

            return;
        }
    };

    let output_file = format!("{}.{}", args.output_name, args.output_extension);

    let new_file = match fs::File::create(output_file) {
        Ok(file) => file,
        Err(err) => {
            println!("{}", err);

            return;
        }
    };

    match write_data(&data, &new_file, args.output_extension) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
        }
    }
}
