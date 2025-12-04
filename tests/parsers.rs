mod utils;

#[cfg(test)]
mod parsers_test {
    use std::io::{BufWriter, Cursor};

    use finance_parser::{BinParser, CsvParser, Parser, TxtParser};

    use crate::utils::{TEST_BIN, TEST_CSV, TEST_TXT, generate_test_vector};

    #[test]
    fn read_txt_data() {
        let mut data_buf = Cursor::new(TEST_TXT);
        let transactions = TxtParser::read(&mut data_buf).unwrap();

        assert_eq!(generate_test_vector(), transactions)
    }

    #[test]
    fn read_csv_data() {
        let mut data_buf = Cursor::new(TEST_CSV);
        let transactions = CsvParser::read(&mut data_buf).unwrap();

        assert_eq!(generate_test_vector(), transactions)
    }

    #[test]
    fn read_bin_data() {
        let mut data_buf = Cursor::new(TEST_BIN);
        let transactions = BinParser::read(&mut data_buf).unwrap();

        assert_eq!(generate_test_vector(), transactions)
    }

    #[test]
    fn write_csv_data() {
        let transactions = generate_test_vector();
        let mut buffer = vec![];

        CsvParser::write(&transactions, &mut buffer).unwrap();

        assert_eq!(TEST_CSV, String::from_utf8(buffer).unwrap())
    }

    #[test]
    fn write_bin_data() {
        let transactions = generate_test_vector();
        let mut cursor = Cursor::new([]);
        let mut writer = BufWriter::new(&mut cursor);

        BinParser::write(&transactions, &mut writer).unwrap();

        assert_eq!(TEST_BIN, writer.buffer())
    }

    #[test]
    fn write_txt_data() {
        let transactions = generate_test_vector();
        let mut buffer = vec![];

        TxtParser::write(&transactions, &mut buffer).unwrap();

        assert_eq!(TEST_TXT, String::from_utf8(buffer).unwrap())
    }
}
