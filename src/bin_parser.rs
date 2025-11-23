use crate::{Parser, ReadError, Status, Transaction, Type, WriteError};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BinRecord {
    pub tx_id: u64,
    pub tx_type: Type,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: Status,
    pub description: String,
}

pub struct BinParser();

impl BinParser {
    fn get_slice(buffer: &mut Vec<u8>, offset: usize, n: usize) -> Result<&[u8], ReadError> {
        let end = offset + n;

        if buffer.len() < end {
            let remaining = buffer.len() - offset;

            return Err(ReadError::Bin {
                offset: Some(offset),
                message: format!(
                    "Failed to get slice of bytes - reached EOF. Tried to get {n} but there are only {remaining} bytes left"
                ),
            });
        }

        Ok(&buffer[offset..end])
    }

    fn check_head(bytes: &[u8]) -> Result<u32, ReadError> {
        let magic = Self::parse_string(&bytes[..4])?;
        if magic != "YPBN" {
            return Err(ReadError::Bin {
                offset: None,
                message: "Error: magic word is incorrect".into()
            });
        }

        Self::parse_u32(&bytes[4..])
    }

    fn parse_body(bytes: &[u8]) -> Result<BinRecord, ReadError> {
        let tx_id = Self::parse_u64(&bytes[..8])?;
        let tx_type = Self::parse_type(&bytes[8])?;
        let from_user_id = Self::parse_u64(&bytes[9..17])?;
        let to_user_id = Self::parse_u64(&bytes[17..25])?;
        let amount = Self::parse_u64(&bytes[25..33])?;
        let timestamp = Self::parse_u64(&bytes[33..41])?;
        let status = Self::parse_status(&bytes[41])?;
        let desc_length = Self::parse_u32(&bytes[42..46])?;
        let description: String = if desc_length == 0 {
            "".into()
        } else {
            Self::parse_string(&bytes[46..])?
        };

        Ok(BinRecord {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            description
        })
    }

    fn parse_string(bytes: &[u8]) -> Result<String, ReadError> {
        let parsed_string = std::str::from_utf8(bytes).map_err(|error| ReadError::Bin {
            offset: None,
            message: format!("Invalid utf8 while attempting to parse string: {error}"),
        })?;

        Ok(parsed_string.into())
    }

    fn parse_u32(bytes: &[u8]) -> Result<u32, ReadError> {
        let parsed_array: [u8; 4] = bytes.try_into()
            .map_err(| error | ReadError::Bin {
                offset: None,
                message: format!("incorrect byte section expected length of 4 to parse u64: {error}")
            })?;

        Ok(u32::from_be_bytes(parsed_array))
    }

    fn parse_u64(bytes: &[u8]) -> Result<u64, ReadError> {
        let parsed_array: [u8; 8] = bytes.try_into()
            .map_err(| error | ReadError::Bin {
                offset: None,
                message: format!("incorrect byte section expected length of 8 to parse u64: {error}")
            })?;

        Ok(u64::from_be_bytes(parsed_array))
    }

    fn parse_type(byte: &u8) -> Result<Type, ReadError> {
        match byte {
            0 => Ok(Type::Deposit),
            1 => Ok(Type::Transfer),
            2 => Ok(Type::Withdraw),
            _ => Err(ReadError::Bin {
                offset: None,
                message: "incorrect Status in data".into()
            })
        }
    }

    fn parse_status(byte: &u8) -> Result<Status, ReadError> {
        match byte {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::Pending),
            _ => Err(ReadError::Bin {
                offset: None,
                message: "incorrect Status in data".into()
            })
        }
    }
}

impl Parser for BinParser {
    fn read<R: std::io::Read>(reader: &mut R) -> Result<Vec<crate::Transaction>, ReadError> {
        let mut buf = Vec::new();

        reader.read_to_end(&mut buf)?;

        let mut current_ofset = 0;
        let mut parsed_data = Vec::new();

        loop {

            let body_offset = current_ofset + 8;
            let head_slice = Self::get_slice(&mut buf, current_ofset, current_ofset + 8)?;
            let body_size = Self::check_head(head_slice)? as usize;

            let body_slice = Self::get_slice(&mut buf, body_offset, body_offset + body_size)?;

            let binary_record = Self::parse_body(body_slice)?;

            parsed_data.push(Transaction::from(binary_record));

            current_ofset = body_offset + body_size;

            if current_ofset >= buf.len() {
                break;
            }
        }

        Ok(parsed_data)
    }

    fn write<W: std::io::Write>(
        _transactions: &[crate::Transaction],
        _writer: &mut W,
    ) -> Result<(), WriteError> {
        Ok(())
    }
}
