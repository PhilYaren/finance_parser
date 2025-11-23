use std::convert::From;
use std::fs::File;
use std::path::Path;
use std::{fmt::Display, io};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

pub mod args;
pub mod bin_parser;
pub mod csv_parser;
pub mod error;
pub mod txt_parser;

pub(crate) use crate::bin_parser::BinRecord;
pub(crate) use crate::txt_parser::TxtRecord;
pub use args::*;
pub use bin_parser::BinParser;
pub use csv_parser::CsvParser;
pub(crate) use csv_parser::CsvRecord;
pub use error::*;
pub use txt_parser::TxtParser;

#[derive(Clone, Debug, PartialEq, ValueEnum, Copy)]
pub enum Extension {
    Bin,
    Txt,
    Csv,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Bin => write!(f, "bin"),
            Self::Txt => write!(f, "txt"),
            Self::Csv => write!(f, "csv"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Status {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "PENDING")]
    Pending,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Success => write!(f, "SUCCESS"),
            Status::Pending => write!(f, "FAILURE"),
            Status::Failure => write!(f, "PENDING"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Type {
    #[serde(rename = "DEPOSIT")]
    Deposit,
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "WITHDRAWAL")]
    Withdraw,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Deposit => write!(f, "DEPOSIT"),
            Self::Transfer => write!(f, "TRANSFER"),
            Self::Withdraw => write!(f, "WITHDRAWAL"),
        }
    }
}

pub fn check_extension(path: &Path) -> Result<Extension, FileError> {
    let file_extension = path.extension();

    if let Some(ext) = file_extension {
        if let Some(ext) = ext.to_str() {
            match ext {
                "csv" => Ok(Extension::Csv),
                "bin" => Ok(Extension::Bin),
                "txt" => Ok(Extension::Txt),
                ext => Err(FileError::ExtensionError(ext.into())),
            }
        } else {
            Err(FileError::ExtensionError("Non utf-8 value".into()))
        }
    } else {
        Err(FileError::NonExistentExtension)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    tx_id: u64,
    tx_type: Type,
    sender_id: u64,
    recipient_id: u64,
    amount: u64,
    timestamp: u64,
    status: Status,
    description: String,
}

impl From<CsvRecord> for Transaction {
    fn from(record: CsvRecord) -> Self {
        Self {
            tx_id: record.tx_id,
            tx_type: record.tx_type,
            sender_id: record.from_user_id,
            recipient_id: record.to_user_id,
            amount: record.amount,
            timestamp: record.timestamp,
            status: record.status,
            description: record.description,
        }
    }
}

impl From<BinRecord> for Transaction {
    fn from(record: BinRecord) -> Self {
        Self {
            tx_id: record.tx_id,
            tx_type: record.tx_type,
            sender_id: record.from_user_id,
            recipient_id: record.to_user_id,
            amount: record.amount,
            timestamp: record.timestamp,
            status: record.status,
            description: record.description,
        }
    }
}

impl From<TxtRecord> for Transaction {
    fn from(value: TxtRecord) -> Self {
        Self {
            tx_id: value.tx_id,
            tx_type: value.tx_type,
            sender_id: value.from_user_id,
            recipient_id: value.to_user_id,
            amount: value.amount,
            timestamp: value.timestamp,
            status: value.status,
            description: value.description,
        }
    }
}

pub trait Parser {
    fn read<R: io::Read>(reader: &mut R) -> Result<Vec<self::Transaction>, ReadError>;
    fn write<W: io::Write>(
        transactions: &[self::Transaction],
        writer: &mut W,
    ) -> Result<(), WriteError>;
}

pub fn get_extension(
    arg_ext: &Option<Extension>,
    file_path: &Path,
) -> Result<Extension, FileError> {
    match arg_ext {
        Some(ext) => Ok(*ext),
        None => check_extension(file_path),
    }
}

pub fn read_file(path: &Path, ext: Extension) -> Result<Vec<Transaction>, ReadError> {
    let mut file = std::fs::File::open(path)?;

    match ext {
        Extension::Csv => CsvParser::read(&mut file),
        Extension::Bin => BinParser::read(&mut file),
        Extension::Txt => TxtParser::read(&mut file),
    }
}

pub fn write_data(data: &Vec<Transaction>, file: &File, ext: Extension) -> Result<(), WriteError> {
    let mut writer = io::BufWriter::new(file);

    match ext {
        Extension::Csv => CsvParser::write(&data, &mut writer),
        Extension::Bin => BinParser::write(&data, &mut writer),
        Extension::Txt => TxtParser::write(&data, &mut writer),
    }
}
