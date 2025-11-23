use std::convert::From;
use std::path::Path;
use std::{fmt::Display, io};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

pub mod args;
pub mod bin_parser;
pub mod csv_parser;
pub mod error;
pub mod txt_parser;

pub use args::*;
pub use csv_parser::CsvParser;
pub(crate) use csv_parser::CsvRecord;
pub use error::*;

use crate::bin_parser::BinRecord;

#[derive(Clone, Debug, PartialEq, ValueEnum, Copy)]
pub enum Extension {
    Bin,
    Text,
    Csv,
}

impl Display for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Bin => write!(f, "bin"),
            Self::Text => write!(f, "text"),
            Self::Csv => write!(f, "csv"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Status {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "FAILURE")]
    Failure,
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

pub fn check_extension(path: &Path) -> Result<Extension, FileError> {
    let file_extension = path.extension();

    if let Some(ext) = file_extension {
        if let Some(ext) = ext.to_str() {
            match ext {
                "csv" => Ok(Extension::Csv),
                "bin" => Ok(Extension::Bin),
                "txt" => Ok(Extension::Text),
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
            description: record.description
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
