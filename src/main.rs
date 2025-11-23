use clap::Parser as ClapParser;
use std::{fs, io, path::Path};

use finance_parser::{Args, CsvParser, Extension, FileError, Parser, check_extension};

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.input_file);

    let extension = match check_extension(&path) {
        Ok(extension) => extension,
        Err(error) => {
            println!("{error}");

            return;
        }
    };

    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => {
            let error = FileError::NonExistentFileError;

            println!("{error}");

            return;
        }
    };

    let data = match extension {
        Extension::Csv => match CsvParser::read(&mut file) {
            Ok(data) => data,
            Err(err) => {
                println!("{}", err.to_string());

                return;
            }
        },
        _ => return,
    };

    let output_name = args.output_name;
    let extension = args.extension;

    let new_file = match fs::File::create(format!("{output_name}.{extension}")) {
        Ok(file) => file,
        Err(err) => {
            println!("{}", err.to_string());

            return;
        }
    };

    let mut writer = io::BufWriter::new(new_file);

    match extension {
        Extension::Csv => match CsvParser::write(&data, &mut writer) {
            Ok(_) => (),
            Err(err) => {
                println!("{}", err.to_string());

                return;
            }
        },
        _ => return,
    }

    println!("{data:?}")
}
