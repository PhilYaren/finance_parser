use std::error::Error;
use std::fmt::Display;
use std::io;

// errors related to the reading of data specified by format or input
#[derive(Debug)]
pub enum ReadError {
    Io(io::Error),

    Csv {
        entry: Option<u32>,
        message: String,
    },

    Bin {
        offset: Option<usize>,
        message: String,
    },

    Txt {
        record: Option<usize>,
        message: String,
    },
}

impl Error for ReadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReadError::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(error: io::Error) -> Self {
        ReadError::Io(error)
    }
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

// error related to the writing of data
#[derive(Debug)]
pub enum WriteError {
    Io(io::Error),

    Csv {
        entry: Option<u32>,
        message: String,
    },

    Bin {
        offset: Option<usize>,
        message: String,
    },

    Txt {
        record: Option<u32>,
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

impl Error for WriteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WriteError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for WriteError {
    fn from(error: io::Error) -> Self {
        WriteError::Io(error)
    }
}

// Unified IO errors
#[derive(Debug)]
pub enum ParserError {
    Read(ReadError),
    Write(WriteError),
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParserError::Read(error) => error.source(),
            ParserError::Write(error) => error.source(),
        }
    }
}

impl From<ReadError> for ParserError {
    fn from(error: ReadError) -> Self {
        Self::Read(error)
    }
}

impl From<WriteError> for ParserError {
    fn from(error: WriteError) -> Self {
        Self::Write(error)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Read(error) => error.fmt(f),
            ParserError::Write(error) => error.fmt(f),
        }
    }
}

// Errors related to files or their extensions
#[derive(Debug)]
pub enum FileError {
    ExtensionError(String),
    NonExistentExtension,
    NonExistentFileError,
}

impl Error for FileError {}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileError::ExtensionError(ref extension) => {
                write!(f, "Unsupported extension: {extension}")
            }
            FileError::NonExistentExtension => write!(
                f,
                "File extension is not provided - please write file in format [example_name].[ext]"
            ),
            FileError::NonExistentFileError => write!(f, "File does not exist"),
        }
    }
}
