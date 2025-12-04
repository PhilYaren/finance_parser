//! mod with CSV parser logic
use crate::{Parser, ReadError, Status, Transaction, Type, WriteError};
use serde::{Deserialize, Serialize};

/// Record as it appears in the CSV file format.
///
/// Field names are controlled via `serde(rename = ...)` to match
/// the expected column headers in the input / output CSV.
#[derive(Debug, Serialize, Deserialize)]
pub struct CsvRecord {
    #[serde(rename = "TX_ID")]
    /// Unique transaction identifier.
    pub tx_id: u64,
    #[serde(rename = "TX_TYPE")]
    /// Kind of transaction.
    pub tx_type: Type,
    #[serde(rename = "FROM_USER_ID")]
    /// Identifier of the sender.
    pub from_user_id: u64,
    #[serde(rename = "TO_USER_ID")]
    /// Identifier of the recipient.
    pub to_user_id: u64,
    #[serde(rename = "AMOUNT")]
    /// Transaction amount.
    pub amount: u64,
    #[serde(rename = "TIMESTAMP")]
    /// Timestamp of the transaction.
    pub timestamp: u64,
    #[serde(rename = "STATUS")]
    /// Status of the transaction.
    pub status: Status,
    #[serde(rename = "DESCRIPTION")]
    /// Free-form description.
    pub description: String,
}

impl From<&Transaction> for CsvRecord {
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

/// Parser for the CSV transaction format.
///
/// This type implements [`Parser`] for CSV input and output.
pub struct CsvParser();

impl Parser for CsvParser {
    fn read<R: std::io::Read>(reader: &mut R) -> Result<Vec<crate::Transaction>, ReadError> {
        let mut rdr = csv::ReaderBuilder::new().quoting(false).from_reader(reader);
        let mut transaction = Vec::new();

        for (entry, record) in rdr.deserialize::<CsvRecord>().enumerate() {
            let record = match record {
                Ok(record) => record,
                Err(err) => {
                    return Err(ReadError::Csv {
                        entry: Some(entry as u32 + 1),
                        message: err.to_string(),
                    });
                }
            };

            transaction.push(Transaction::from(record));
        }

        Ok(transaction)
    }
    fn write<W: std::io::Write>(
        transactions: &[crate::Transaction],
        writer: &mut W,
    ) -> Result<(), WriteError> {
        let csv_array = transactions.iter().map(CsvRecord::from);
        let mut csv_writer = csv::WriterBuilder::new()
            .quote_style(csv::QuoteStyle::Never)
            .from_writer(writer);

        for (entry, record) in csv_array.enumerate() {
            match csv_writer.serialize::<CsvRecord>(record) {
                Ok(_) => continue,
                Err(err) => {
                    return Err(WriteError::Csv {
                        entry: Some(entry as u32 + 1),
                        message: err.to_string(),
                    });
                }
            };
        }

        csv_writer.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn test_csv_parser_read() {
        // Создаем тестовый CsvRecord
        let record = CsvRecord {
            tx_id: 123,
            tx_type: Type::Withdraw,
            from_user_id: 456,
            to_user_id: 789,
            amount: 300,
            timestamp: 1620000000,
            status: Status::Failure,
            description: "Test withdrawal".to_string(),
        };

        // Создаем CSV-строку
        let csv_data = format!(
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
             {},{},{},{},{},{},{},{}",
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
        let mut reader = Cursor::new(csv_data);
        let transactions = CsvParser::read(&mut reader).unwrap();

        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].tx_id, 123);
        assert_eq!(transactions[0].tx_type, Type::Withdraw);
    }

    #[test]
    fn test_csv_parser_write() {
        // Создаем тестовые транзакции
        let mut transactions = Vec::new();
        let record = CsvRecord {
            tx_id: 123,
            tx_type: Type::Deposit,
            from_user_id: 456,
            to_user_id: 789,
            amount: 400,
            timestamp: 1620000000,
            status: Status::Success,
            description: "Test deposit".to_string(),
        };
        transactions.push(Transaction::from(record));

        // Создаем память для записи
        let mut buffer = Vec::new();

        // Проверяем запись
        CsvParser::write(&transactions, &mut buffer).unwrap();

        let mut content = String::new();
        Cursor::new(buffer).read_to_string(&mut content).unwrap();

        // Проверяем содержимое
        assert_eq!(
            content,
            "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n123,DEPOSIT,456,789,400,1620000000,SUCCESS,Test deposit\n"
        );
    }
}
