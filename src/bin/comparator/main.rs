mod args;

use args::Args;
use clap::Parser as ClapParser;
use finance_parser::{get_extension, read_file};
use std::path::Path;

fn main() {
    let args = Args::parse();

    let first_file_path = Path::new(&args.first_file);
    let second_file_path = Path::new(&args.second_file);

    let first_ext = match get_extension(&args.first_ext, first_file_path) {
        Ok(ext) => ext,
        Err(err) => {
            println!("{}", err);

            return;
        }
    };

    let second_ext = match get_extension(&args.second_ext, second_file_path) {
        Ok(ext) => ext,
        Err(err) => {
            println!("{}", err);

            return;
        }
    };

    let first_file_content = match read_file(first_file_path, first_ext) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);

            return;
        }
    };

    let second_file_content = match read_file(second_file_path, second_ext) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);

            return;
        }
    };

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
}
