//! Library for parsing and converting financial transactions.
//!
//! This crate provides:
//! - detection of file format by extension (`csv`, `bin`, `txt`),
//! - reading transactions from different file formats,
//! - writing transactions to a chosen format,
//! - a common [`Transaction`] structure used as an internal data model.
//!
//! Typical entry points are:
//! - [`get_extension`] to determine file extension,
//! - [`read_file`] to read data from a path,
//! - [`write_data`] to write a list of transactions.
//!
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

/// Supported file extensions for transaction data.
///
/// This enum is used both for CLI and internally
/// to dispatch to a specific parser implementation.
#[derive(Clone, Debug, PartialEq, ValueEnum, Copy)]
pub enum Extension {
    /// Binary format (`.bin`).
    Bin,
    /// Plain text format (`.txt`).
    Txt,
    /// CSV format (`.csv`).
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

/// Status of a transaction.
///
/// The `serde(rename = ...)` attributes are used to match
/// the external csv and textual representation used in files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Status {
    /// Transaction was processed successfully.
    #[serde(rename = "SUCCESS")]
    Success,
    /// Transaction failed (rejected or errored).
    #[serde(rename = "FAILURE")]
    Failure,
    /// Transaction is still being processed.
    #[serde(rename = "PENDING")]
    Pending,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Success => write!(f, "SUCCESS"),
            Status::Failure => write!(f, "FAILURE"),
            Status::Pending => write!(f, "PENDING"),
        }
    }
}

/// Kind of financial transaction.
///
/// Serialized / deserialized as textual values `DEPOSIT`,
/// `TRANSFER`, or `WITHDRAWAL`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Type {
    /// Deposit into an account.
    #[serde(rename = "DEPOSIT")]
    Deposit,
    /// Transfer between accounts.
    #[serde(rename = "TRANSFER")]
    Transfer,
    /// Withdrawal from an account.
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

/// Determine the file extension from a path and map it to [`Extension`].
///
/// # Errors
///
/// Returns [`FileError::ExtensionError`] or [`FileError::NonExistentExtension`]
/// if the extension is missing or unsupported.
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

/// In-memory representation of a single financial transaction.
///
/// This structure is used as a common model for data parsed from
/// different file formats (CSV, binary, plain text).
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// Unique transaction identifier.
    pub tx_id: u64,
    /// Transaction kind (deposit, transfer, withdrawal).
    pub tx_type: Type,
    /// Identifier of the sender.
    pub sender_id: u64,
    /// Identifier of the receiver.
    pub recipient_id: u64,
    /// Amount of the transaction.
    pub amount: u64,
    /// Timestamp of the transaction (Unix timestamp).
    pub timestamp: u64,
    /// Current status of the transaction.
    pub status: Status,
    /// Free-form description or comment for the transaction.
    pub description: String,
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

/// Common interface for all format-specific parsers.
///
/// Types implementing this trait (`CsvParser`, `BinParser`, `TxtParser`)
/// know how to read and write lists of [`Transaction`] values.
/// can be used to implement your own Parsers
pub trait Parser {
    /// Read transactions from the given byte stream.
    ///
    /// Implementations are expected to parse the whole input and
    /// return a `Vec<Transaction>` on success.
    ///
    /// # Errors
    ///
    /// Returns a [`ReadError`] if reading or parsing fails.
    fn read<R: io::Read>(reader: &mut R) -> Result<Vec<self::Transaction>, ReadError>;

    /// Write transactions to the given byte stream.
    ///
    /// Implementations are expected to serialize all provided
    /// transactions in the target format.
    ///
    /// # Errors
    ///
    /// Returns a [`WriteError`] if writing fails.
    fn write<W: io::Write>(
        transactions: &[self::Transaction],
        writer: &mut W,
    ) -> Result<(), WriteError>;
}

/// Decide which [`Extension`] should be used, based on CLI args and file path.
///
/// If an explicit extension is provided in `arg_ext`, it is returned as-is.
/// Otherwise, the extension is inferred from `file_path` via [`check_extension`].
///
/// # Errors
///
/// Propagates errors from [`check_extension`] when the extension
/// cannot be inferred from the path.
pub fn get_extension(
    arg_ext: &Option<Extension>,
    file_path: &Path,
) -> Result<Extension, FileError> {
    match arg_ext {
        Some(ext) => Ok(*ext),
        None => check_extension(file_path),
    }
}

/// Read a transaction file of a known format.
///
/// A format-specific parser is selected based on the provided [`Extension`]
/// and a list of [`Transaction`] values is returned.
///
/// # Errors
///
/// Returns a [`ReadError`] if the file cannot be opened or parsed.
pub fn read_file(path: &Path, ext: Extension) -> Result<Vec<Transaction>, ReadError> {
    let mut file = std::fs::File::open(path)?;

    match ext {
        Extension::Csv => CsvParser::read(&mut file),
        Extension::Bin => BinParser::read(&mut file),
        Extension::Txt => TxtParser::read(&mut file),
    }
}

/// Write transaction data to a file in a chosen format.
///
/// A format-specific parser is selected based on the provided [`Extension`]
/// and the list of [`Transaction`] values is serialized to `file`.
///
/// # Errors
///
/// Returns a [`WriteError`] if serialization or writing fails.
pub fn write_data(data: &[Transaction], file: &File, ext: Extension) -> Result<(), WriteError> {
    let mut writer = io::BufWriter::new(file);

    match ext {
        Extension::Csv => CsvParser::write(data, &mut writer),
        Extension::Bin => BinParser::write(data, &mut writer),
        Extension::Txt => TxtParser::write(data, &mut writer),
    }
}
