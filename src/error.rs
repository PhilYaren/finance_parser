//! App related errors
// use std::error::Error;
use std::fmt::Display;
use std::io;
use thiserror::Error;

/// Errors that can occur while reading or parsing transaction data.
///
/// The variants are split by format (CSV, binary, text) so that
/// additional context (entry index, byte offset, line number) can be
/// attached to the error.
#[derive(Debug, Error)]
pub enum ReadError {
    /// Low-level I/O error passed by reader.
    Io(#[from] io::Error),

    /// Error parsing CSV data.
    Csv { entry: Option<u32>, message: String },

    /// Error parsing binary data.
    Bin {
        offset: Option<usize>,
        message: String,
    },

    /// Error parsing the custom text format.
    ///
    /// If `record` is present it holds the 1-based line number of the
    /// record that failed to parse.
    Txt {
        record: Option<usize>,
        message: String,
    },
}

impl Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::Io(e) => write!(f, "I/O read error: {e}"),
            ReadError::Csv { entry, message } => match entry {
                Some(entry) => write!(f, "CSV read error occured in entry {entry}: {message}"),
                None => write!(f, "CSV read error occured: {message}"),
            },
            ReadError::Bin { offset, message } => match offset {
                Some(offset) => {
                    write!(f, "Binary read error occured at offset {offset}: {message}")
                }
                None => write!(f, "Binary read error occured: {message}"),
            },
            ReadError::Txt { record, message } => match record {
                Some(record) => write!(f, "Text read error occured at line {record}: {message}"),
                None => write!(f, "Text read error occured: {message}"),
            },
        }
    }
}

/// Errors that can occur while writing or serializing transaction data.
///
/// The variants are split by format (CSV, binary, text) so that
/// additional context (entry index, byte offset, record number) can be
/// attached to the error.
#[derive(Debug, Error)]
pub enum WriteError {
    /// Low-level I/O error passed by writer.
    Io(#[from] io::Error),

    /// Error serializing CSV data.
    Csv { entry: Option<u32>, message: String },

    /// Error serializing binary data.
    Bin {
        offset: Option<usize>,
        message: String,
    },

    /// Error serializing the custom text format.
    Txt {
        record: Option<usize>,
        message: String,
    },
}

impl Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::Io(e) => write!(f, "I/O write error: {e}"),
            WriteError::Csv { entry, message } => {
                if let Some(n) = entry {
                    write!(f, "CSV write error occured in entry {n}: {message}")
                } else {
                    write!(f, "CSV write error: {message}")
                }
            }
            WriteError::Bin { offset, message } => {
                if let Some(off) = offset {
                    write!(f, "Binary write error occured at offset {off}: {message}")
                } else {
                    write!(f, "Binary write error: {message}")
                }
            }
            WriteError::Txt { record, message } => {
                if let Some(record) = record {
                    write!(f, "Text write error occured at record {record}: {message}")
                } else {
                    write!(f, "Text write error: {message}")
                }
            }
        }
    }
}

/// Unified IO errors
#[derive(Debug, Error)]
pub enum ParserError {
    /// Error that occurred while reading.
    #[error("{0}")]
    Read(#[from] ReadError),
    /// Error that occurred while writing.
    #[error("{0}")]
    Write(#[from] WriteError),
}

/// Errors related to files or their extensions.
///
/// This type is used when dealing with paths and file system issues,
/// such as missing files or unsupported extensions.
#[derive(Debug, Error)]
pub enum FileError {
    /// The file has an extension, but it is not one of the supported formats.
    #[error("Unsupported extension: {0}")]
    ExtensionError(String),
    /// The file path has no extension at all.
    #[error("File extension is not provided - please write file in format [example_name].[ext]")]
    NonExistentExtension,
    /// The file does not exist at the given path.
    #[error("File does not exist")]
    NonExistentFileError,
}
