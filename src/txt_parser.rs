// use serde::{ Serialize, Deserialize };
// use crate::{ Type, Status, Parser };

// #[derive(Debug, Serialize, Deserialize)]
// pub struct CsvRecord {
//     #[serde(rename = "TX_ID")]
//     tx_id: u64,
//     #[serde(rename = "TX_TYPE")]
//     tx_type: Type,
//     #[serde(rename = "FROM_USER_ID")]
//     from_user_id: u64,
//     #[serde(rename = "TO_USER_ID")]
//     to_user_id: u64,
//     #[serde(rename = "AMOUNT")]
//     amount: u64,
//     #[serde(rename = "TIMESTAMP")]
//     timestamp: i64,
//     #[serde(rename = "STATUS")]
//     status: Status,
//     #[serde[rename = "DESCRIPTION"]]
//     description: String
// }

// pub struct TxtParser ();

// impl Parser for TxtParser {
//     fn read<R: std::io::Read>(reader: &mut R) -> Result<Vec<crate::Transaction>, Box<dyn std::error::Error>> {

//     }
//     fn write<W: std::io::Write>(transactions: &[crate::Transaction] ,writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(())
//     }
// }
