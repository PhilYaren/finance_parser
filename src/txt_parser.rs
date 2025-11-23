use crate::{Parser, ReadError, Status, Transaction, Type, WriteError};

pub struct TxtRecord {
    pub tx_id: u64,
    pub tx_type: Type,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: Status,
    pub description: String,
}

pub struct TxtParser();

impl From<&Transaction> for TxtRecord {
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

impl Parser for TxtParser {
    fn read<R: std::io::Read>(reader: &mut R) -> Result<Vec<Transaction>, ReadError> {
        let mut transactions = Vec::new();

        let mut text_buffer = String::new();
        reader.read_to_string(&mut text_buffer)?;

        let txt_array = text_buffer.trim().split("\n\n");
        for (index, record) in txt_array.enumerate() {
            let mut tx_id: Option<u64> = None;
            let mut tx_type: Option<Type> = None;
            let mut from_user_id: Option<u64> = None;
            let mut to_user_id: Option<u64> = None;
            let mut amount: Option<u64> = None;
            let mut timestamp: Option<u64> = None;
            let mut status: Option<Status> = None;
            let mut description: Option<String> = None;

            for entry in record.lines() {
                if entry.starts_with("# Record") {
                    continue;
                }

                let (key, value) = entry.split_once(": ").unwrap_or_else(|| (entry, ""));

                match key {
                    "TX_ID" => {
                        tx_id = match u64::from_str_radix(value, 10) {
                            Ok(value) => Some(value),
                            Err(err) => {
                                return Err(ReadError::Txt {
                                    record: Some(index + 1),
                                    message: format!(
                                        "Error while reading text data in record {}: TX_ID is not unsigned integer. Original error: {}",
                                        index + 1,
                                        err
                                    ),
                                });
                            }
                        }
                    }
                    "TX_TYPE" => {
                        tx_type = match value {
                            "DEPOSIT" => Some(Type::Deposit),
                            "TRANSFER" => Some(Type::Transfer),
                            "WITHDRAWAL" => Some(Type::Withdraw),
                            _ => {
                                return Err(ReadError::Txt {
                                    record: Some(index + 1),
                                    message: format!(
                                        "Error while reading text data in record {}: TX_TYPE should be DEPOSIT, TRANSFER or WITHDRAWAL",
                                        index + 1
                                    ),
                                });
                            }
                        }
                    }
                    "STATUS" => {
                        status = match value {
                            "SUCCESS" => Some(Status::Success),
                            "FAILURE" => Some(Status::Failure),
                            "PENDING" => Some(Status::Pending),
                            _ => {
                                return Err(ReadError::Txt {
                                    record: Some(index + 1),
                                    message: format!(
                                        "Error while reading text data in record {}: STATUS should be SUCCESS, FAILURE or PENDING",
                                        index + 1
                                    ),
                                });
                            }
                        }
                    }
                    "TO_USER_ID" => {
                        to_user_id = match u64::from_str_radix(value, 10) {
                            Ok(value) => Some(value),
                            Err(err) => {
                                return Err(ReadError::Txt {
                                    record: Some(index + 1),
                                    message: format!(
                                        "Error while reading text data in record {}: TO_USER_ID is not unsigned integer. Original error: {}",
                                        index + 1,
                                        err
                                    ),
                                });
                            }
                        };
                    }
                    "FROM_USER_ID" => {
                        from_user_id = match u64::from_str_radix(value, 10) {
                            Ok(value) => Some(value),
                            Err(err) => {
                                return Err(ReadError::Txt {
                                    record: Some(index + 1),
                                    message: format!(
                                        "Error while reading text data in record {}: FROM_USER_ID is not unsigned integer. Original error: {}",
                                        index + 1,
                                        err
                                    ),
                                });
                            }
                        };
                    }
                    "AMOUNT" => {
                        amount = match u64::from_str_radix(value, 10) {
                            Ok(value) => Some(value),
                            Err(err) => {
                                return Err(ReadError::Txt {
                                    record: Some(index + 1),
                                    message: format!(
                                        "Error while reading text data in record {}: AMOUNT is not unsigned integer. Original error: {}",
                                        index + 1,
                                        err
                                    ),
                                });
                            }
                        };
                    }
                    "TIMESTAMP" => {
                        timestamp = match u64::from_str_radix(value, 10) {
                            Ok(value) => Some(value),
                            Err(err) => {
                                return Err(ReadError::Txt {
                                    record: Some(index + 1),
                                    message: format!(
                                        "Error while reading text data in record {}: AMOUNT is not unsigned integer. Original error: {}",
                                        index + 1,
                                        err
                                    ),
                                });
                            }
                        };
                    }
                    "DESCRIPTION" => {
                        description = Some(value.to_string());
                    }
                    _ => continue,
                }
            }

            let txt_record = TxtRecord {
                tx_id: tx_id.expect("missing TX_ID"),
                tx_type: tx_type.expect("missing TX_TYPE"),
                status: status.expect("missing STATUS"),
                to_user_id: to_user_id.expect("missing TO_USER_ID"),
                from_user_id: from_user_id.expect("missing FROM USER_ID"),
                amount: amount.expect("missing AMMOUNT"),
                timestamp: timestamp.expect("missing TIMETAMP"),
                description: description.expect("missing DESCRIPTION"),
            };

            transactions.push(Transaction::from(txt_record));
        }

        Ok(transactions)
    }

    fn write<W: std::io::Write>(
        transactions: &[Transaction],
        writer: &mut W,
    ) -> Result<(), WriteError> {
        let record_array = transactions.iter().map(TxtRecord::from);

        for (index, record) in record_array.enumerate() {
            writer.write_fmt(format_args!(
                "# Record {} ({})\n",
                index + 1,
                record.tx_type
            ))?;
            writer.write_fmt(format_args!("TX_ID: {}\n", record.tx_id))?;
            writer.write_fmt(format_args!("TX_TYPE: {}\n", record.tx_type))?;
            writer.write_fmt(format_args!("STATUS: {}\n", record.status))?;
            writer.write_fmt(format_args!("FROM_USER_ID: {}\n", record.from_user_id))?;
            writer.write_fmt(format_args!("TO_USER_ID: {}\n", record.to_user_id))?;
            writer.write_fmt(format_args!("TIMESTAMP: {}\n", record.timestamp))?;
            writer.write_fmt(format_args!("AMOUNT: {}\n", record.amount))?;
            writer.write_fmt(format_args!("DESCRIPTION: {}\n", record.description))?;
            writer.write_fmt(format_args!("\n"))?;
        }

        writer.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn test_txt_parser_read() {
        // Создаем тестовый TxtRecord
        let record = TxtRecord {
            tx_id: 123,
            tx_type: Type::Transfer,
            from_user_id: 456,
            to_user_id: 789,
            amount: 500,
            timestamp: 1620000000,
            status: Status::Pending,
            description: "Test transfer".to_string(),
        };

        // Создаем текстовый формат
        let txt_data = format!(
            "# Record 1 ({})\n\
            TX_ID: {}\n\
            TX_TYPE: {}\n\
            FROM_USER_ID: {}\n\
            TO_USER_ID: {}\n\
            AMOUNT: {}\n\
            TIMESTAMP: {}\n\
            STATUS: {}\n\
            DESCRIPTION: {}\n",
            record.tx_type,
            record.tx_id,
            record.tx_type,
            record.from_user_id,
            record.to_user_id,
            record.amount,
            record.timestamp,
            record.status,
            record.description
        );

        // Проверяем чтение
        let mut reader = Cursor::new(txt_data);
        let transactions = TxtParser::read(&mut reader).unwrap();

        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].tx_id, 123);
        assert_eq!(transactions[0].tx_type, Type::Transfer);
    }

    #[test]
    fn test_txt_parser_write() {
        // Создаем тестовые транзакции
        let mut transactions = Vec::new();
        let record = TxtRecord {
            tx_id: 123,
            tx_type: Type::Withdraw,
            from_user_id: 456,
            to_user_id: 789,
            amount: 600,
            timestamp: 1620000000,
            status: Status::Success,
            description: "Test withdrawal".to_string(),
        };
        transactions.push(Transaction::from(record));

        // Создаем память для записи
        let mut buff: Vec<u8> = Vec::new();

        // Проверяем запись
        TxtParser::write(&transactions, &mut buff).unwrap();

        let mut writer = Cursor::new(buff);
        let mut string_buf = String::new();
        writer.read_to_string(&mut string_buf).unwrap();

        // Проверяем содержимое
        assert_eq!(
            string_buf,
            "# Record 1 (WITHDRAWAL)\nTX_ID: 123\nTX_TYPE: WITHDRAWAL\nSTATUS: SUCCESS\nFROM_USER_ID: 456\nTO_USER_ID: 789\nTIMESTAMP: 1620000000\nAMOUNT: 600\nDESCRIPTION: Test withdrawal\n\n"
        );
    }
}
