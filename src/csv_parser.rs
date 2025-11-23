use crate::{Parser, ReadError, Status, Transaction, Type, WriteError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CsvRecord {
    #[serde(rename = "TX_ID")]
    pub tx_id: u64,
    #[serde(rename = "TX_TYPE")]
    pub tx_type: Type,
    #[serde(rename = "FROM_USER_ID")]
    pub from_user_id: u64,
    #[serde(rename = "TO_USER_ID")]
    pub to_user_id: u64,
    #[serde(rename = "AMOUNT")]
    pub amount: u64,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: u64,
    #[serde(rename = "STATUS")]
    pub status: Status,
    #[serde(rename = "DESCRIPTION")]
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

pub struct CsvParser();

impl Parser for CsvParser {
    fn read<R: std::io::Read>(reader: &mut R) -> Result<Vec<crate::Transaction>, ReadError> {
        let mut rdr = csv::Reader::from_reader(reader);
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
        let mut csv_writer = csv::Writer::from_writer(writer);

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
