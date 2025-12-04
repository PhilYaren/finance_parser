//! mod with Binary parser logic
use crate::{Parser, ReadError, Status, Transaction, Type, WriteError};

/// Raw record used by the binary file format.
///
/// This struct mirrors the on-disk representation used by [`BinParser`]
/// and is converted to and from the higher-level [`Transaction`] type.
#[derive(Debug)]
pub struct BinRecord {
    /// Unique transaction identifier.
    pub tx_id: u64,
    /// Kind of transaction.
    pub tx_type: Type,
    /// Identifier of the sender.
    pub from_user_id: u64,
    /// Identifier of the recipient.
    pub to_user_id: u64,
    /// Transaction amount.
    pub amount: u64,
    /// Timestamp associated with the record.
    pub timestamp: u64,
    /// Status as stored in the binary stream.
    pub status: Status,
    /// Free-form description.
    pub description: String,
}

impl BinRecord {
    fn convert_to_bin(self: &Self) -> (usize, Vec<u8>) {
        let mut bin_vec = Vec::new();

        bin_vec.append(&mut self.tx_id.to_be_bytes().to_vec());
        bin_vec.push(self.tx_type as u8);
        bin_vec.append(&mut self.from_user_id.to_be_bytes().to_vec());
        bin_vec.append(&mut self.to_user_id.to_be_bytes().to_vec());
        bin_vec.append(&mut self.amount.to_be_bytes().to_vec());
        bin_vec.append(&mut self.timestamp.to_be_bytes().to_vec());
        bin_vec.push(self.status as u8);
        let mut desc_bytes = self.description.as_bytes().to_vec();
        let mut length = (desc_bytes.len() as u32).to_be_bytes().to_vec();

        bin_vec.append(&mut length);
        bin_vec.append(&mut desc_bytes);

        (bin_vec.len(), bin_vec)
    }
}

impl From<&Transaction> for BinRecord {
    fn from(value: &Transaction) -> Self {
        Self {
            tx_id: value.tx_id,
            tx_type: value.tx_type,
            from_user_id: value.sender_id,
            to_user_id: value.recipient_id,
            amount: value.amount,
            timestamp: value.timestamp,
            status: value.status,

            description: value.description.to_string(),
        }
    }
}

/// Parser for the custom binary transaction format.
///
/// This type implements [`Parser`] for binary input and output.
pub struct BinParser();

impl BinParser {
    fn get_slice(buffer: &[u8], offset: usize, n: usize) -> Result<&[u8], ReadError> {
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
                message: "Error: magic word is incorrect".into(),
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
            description,
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
        let parsed_array: [u8; 4] = bytes.try_into().map_err(|error| ReadError::Bin {
            offset: None,
            message: format!("incorrect byte section expected length of 4 to parse u64: {error}"),
        })?;

        Ok(u32::from_be_bytes(parsed_array))
    }

    fn parse_u64(bytes: &[u8]) -> Result<u64, ReadError> {
        let parsed_array: [u8; 8] = bytes.try_into().map_err(|error| ReadError::Bin {
            offset: None,
            message: format!("incorrect byte section expected length of 8 to parse u64: {error}"),
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
                message: "incorrect Type in data".into(),
            }),
        }
    }

    fn parse_status(byte: &u8) -> Result<Status, ReadError> {
        match byte {
            0 => Ok(Status::Success),
            1 => Ok(Status::Failure),
            2 => Ok(Status::Pending),
            _ => Err(ReadError::Bin {
                offset: None,
                message: "incorrect Status in data".into(),
            }),
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
            let head_slice = Self::get_slice(&mut buf, current_ofset, 8)?;
            let body_size = Self::check_head(head_slice)? as usize;

            let body_slice = Self::get_slice(&mut buf, body_offset, body_size)?;

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
        transactions: &[crate::Transaction],
        writer: &mut W,
    ) -> Result<(), WriteError> {
        let bin_records = transactions.iter().map(BinRecord::from);

        for record in bin_records {
            let mut buffer = vec![0x59, 0x50, 0x42, 0x4E];

            let (length, mut byte_record) = record.convert_to_bin();
            buffer.append(&mut (length as u32).to_be_bytes().to_vec());
            buffer.append(&mut byte_record);

            writer.write(&buffer[..])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const TEST_DATA: &[u8] = &[
        89, 80, 66, 78, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 123, 0, 0, 0, 0, 0, 0, 0, 1, 200, 0, 0,
        0, 0, 0, 0, 3, 21, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 0, 0, 96, 143, 61, 0, 0, 0, 0, 0, 16,
        84, 101, 115, 116, 32, 116, 114, 97, 110, 115, 97, 99, 116, 105, 111, 110,
    ];

    #[test]
    fn test_bin_parser_read() {
        // Создаем тестовый BinRecord
        let record = BinRecord {
            tx_id: 123,
            tx_type: Type::Deposit,
            from_user_id: 456,
            to_user_id: 789,
            amount: 100,
            timestamp: 1620000000,
            status: Status::Success,
            description: "Test transaction".to_string(),
        };
        let transaction = Transaction::from(record);

        // Создаем Cursor для чтения
        let mut reader = Cursor::new(TEST_DATA);
        let parsed_transaction = BinParser::read(&mut reader).unwrap()[0].clone();

        // Проверяем чтение

        assert_eq!(transaction, parsed_transaction);
    }

    #[test]
    fn test_bin_parser_write() {
        // Создаем тестовые транзакции
        let mut transactions = Vec::new();
        let record = BinRecord {
            tx_id: 123,
            tx_type: Type::Deposit,
            from_user_id: 456,
            to_user_id: 789,
            amount: 100,
            timestamp: 1620000000,
            status: Status::Success,
            description: "Test transaction".to_string(),
        };
        transactions.push(Transaction::from(record));

        // Создаем память для записи
        let mut writer = Vec::new();

        // Проверяем запись
        BinParser::write(&transactions, &mut writer).unwrap();

        // Проверяем содержимое
        assert_eq!(writer, TEST_DATA);
    }
}
