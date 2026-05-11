use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use crate::errors::ParseError;
use crate::TransactionRecord;

/// Парсит txt-талицу и собирает структуру через замыкание-адаптер (аналог питонячей yield-лямбды)
fn parse_map(transaction_map: &HashMap<String, String>) -> Result<TransactionRecord, ParseError> {
    let get = |key: &str| {
        transaction_map.get(key).ok_or(ParseError::InvalidLine)
    };
    Ok(TransactionRecord {
        tx_id:          get("TX_ID")?.parse()?,
        tx_type:        get("TX_TYPE")?.parse()?,
        from_user_id:   get("FROM_USER_ID")?.parse()?,
        to_user_id:     get("TO_USER_ID")?.parse()?,
        amount:         get("AMOUNT")?.parse()?,
        timestamp:      get("TIMESTAMP")?.parse()?,
        status:         get("STATUS")?.parse()?,
        description:    get("DESCRIPTION")?.to_string(),
    })
}
/// Бьёт входящий контент на строки, проверяет их и собирает вектор транзакций
pub fn parse_txt_to_transactions<R: Read>(content: R) -> Result<Vec<TransactionRecord>, ParseError> {
    let reader = BufReader::new(content);
    let mut counter = "";
    let mut transaction_map: HashMap<String, String> = HashMap::new();
    let mut transactions = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with('#') {
                counter = "start";
                continue;
            }
            if line.is_empty() {
                counter = "end";
                if !transaction_map.is_empty() {
                    let transaction = parse_map(&transaction_map)?;
                    transactions.push(transaction);
                    transaction_map.clear();
                };
                continue;
            }
            match counter {
                "start" => {
                    if let Some((key, value)) = line.trim().split_once(": ") {
                        transaction_map.insert(key.to_string(), value.to_string());
                    }
                }
                _ => {
                    continue;
                }
            }
        }
    }
    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{TransactionStatus, TransactionType};
    use super::*;

    #[test]
    fn test_parse_txt_map_success() {
        let mut map = HashMap::new();
        map.insert("DESCRIPTION".to_string(), r#""Record number 2""#.to_string());
        map.insert("TIMESTAMP".to_string(), "1633036920000".to_string());
        map.insert("STATUS".to_string(), "PENDING".to_string());
        map.insert("AMOUNT".to_string(), "200".to_string());
        map.insert("TX_ID".to_string(), "1000000000000001".to_string());
        map.insert("TX_TYPE".to_string(), "TRANSFER".to_string());
        map.insert("FROM_USER_ID".to_string(), "9223372036854775807".to_string());
        map.insert("TO_USER_ID".to_string(), "9223372036854775807".to_string());
        let transaction = parse_map(&map);

        assert!(transaction.is_ok());

        let valid_transaction = TransactionRecord {
            tx_id:          1000000000000001,
            tx_type:        TransactionType::TRANSFER,
            from_user_id:   9223372036854775807,
            to_user_id:     9223372036854775807,
            amount:         200,
            timestamp:      1633036920000,
            status:         TransactionStatus::PENDING,
            description:    "Record number 2".to_string(),
        };

        assert_eq!(transaction.unwrap(), valid_transaction);
    }
    #[test]
    fn test_parse_txt_line_wrong_int() {
        let mut map = HashMap::new();
        map.insert("DESCRIPTION".to_string(), r#""Record number 2""#.to_string());
        map.insert("TIMESTAMP".to_string(), "abyr".to_string());
        map.insert("STATUS".to_string(), "PENDING".to_string());
        map.insert("AMOUNT".to_string(), "abyr".to_string());
        map.insert("TX_ID".to_string(), "abyr".to_string());
        map.insert("TX_TYPE".to_string(), "TRANSFER".to_string());
        map.insert("FROM_USER_ID".to_string(), "abyr".to_string());
        map.insert("TO_USER_ID".to_string(), "valg".to_string());
        let transaction = parse_map(&map);

        match transaction {
            Err(ParseError::ParseIntError(msg)) => {
                assert_eq!(msg.to_string(), "invalid digit found in string".to_string());
            }
            Err(other) => panic!("Ожидалась ParseIntError, получено: {:?}", other),
            Ok(_) => panic!("Ожидалась ошибка, но парсинг прошёл успешно"),
        }
    }
    #[test]
    fn test_parse_txt_line_wrong_enum() {
        let mut map = HashMap::new();
        map.insert("DESCRIPTION".to_string(), r#""Record number 2""#.to_string());
        map.insert("TIMESTAMP".to_string(), "1633036920000".to_string());
        map.insert("STATUS".to_string(), "ABYRVALG".to_string());
        map.insert("AMOUNT".to_string(), "200".to_string());
        map.insert("TX_ID".to_string(), "1000000000000001".to_string());
        map.insert("TX_TYPE".to_string(), "ABYRVALG".to_string());
        map.insert("FROM_USER_ID".to_string(), "9223372036854775807".to_string());
        map.insert("TO_USER_ID".to_string(), "9223372036854775807".to_string());
        let transaction = parse_map(&map);

        match transaction {
            Err(ParseError::ParseEnumError(msg)) => {
                assert_eq!(msg.to_string(), "Unknown TransactionType: ABYRVALG".to_string());
            }
            Err(other) => panic!("Ожидалась ParseEnumError, получено: {:?}", other),
            Ok(_) => panic!("Ожидалась ошибка, но парсинг прошёл успешно"),
        }
    }
    #[test]
    fn parse_txt_to_transactions_success() {
        let fake_file = concat!(
            "\n", "# Record 446 (TRANSFER)", "\n", "AMOUNT: 44600", "\n", r#"DESCRIPTION: "Record number 446""#,
            "\n", "TX_TYPE: TRANSFER", "\n", "TX_ID: 1000000000000445", "\n", "TO_USER_ID: 9173880151496138473",
            "\n", "TIMESTAMP: 1633063560000", "\n", "STATUS: PENDING", "\n", "FROM_USER_ID: 9223372036854775807", "\n",
            "\n", "# Record 447 (WITHDRAWAL)", "\n", "STATUS: SUCCESS", "\n", "TIMESTAMP: 1633063620000",
            "\n", "AMOUNT: 44700", "\n", "FROM_USER_ID: 6899145982714634482", "\n", "TX_ID: 1000000000000446",
            "\n", "TX_TYPE: WITHDRAWAL", "\n", "TO_USER_ID: 0", "\n", r#"DESCRIPTION: "Record number 447""#, "\n", "\n",
        );
        let cursor = Cursor::new(fake_file.as_bytes());
        let records = parse_txt_to_transactions(cursor).unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].description, r#""Record number 446""#);
        assert_eq!(records[1].description, r#""Record number 447""#);
    }
    #[test]
    fn parse_txt_to_transactions_wrong_line() {
        let fake_file = concat!(
        "\n", "# Record 446 (TRANSFER)", "\n", "AMOUNT: 44600", "\n", r#"DESCRIPTION: "Record number 446""#,
        "\n", "TX_TYPE: TRANSFER", "\n", "TX_ID: 1000000000000445", "\n", "TO_USER_ID: 9173880151496138473",
        "\n", "TIMESTAMP: 1633063560000", "\n", "STATUS: PENDING", "\n",
        "\n", "# Record 447 (WITHDRAWAL)", "\n", "STATUS: SUCCESS", "\n", "TIMESTAMP: 1633063620000",
        "\n", "AMOUNT: 44700", "\n", "FROM_USER_ID: 6899145982714634482", "\n", "TX_ID: 1000000000000446",
        "\n", "TX_TYPE: WITHDRAWAL", "\n", "TO_USER_ID: 0", "\n", r#"DESCRIPTION: "Record number 447""#, "\n", "\n",
        );
        let cursor = Cursor::new(fake_file.as_bytes());
        let records = parse_txt_to_transactions(cursor);

        match records {
            Err(ParseError::InvalidLine) => {
                println!("Тест parse_txt_to_transactions_wrong_line прошёл успешно");
            }
            Err(other) => panic!("Ожидалась InvalidLine, получено: {:?}", other),
            Ok(_) => panic!("Ожидалась ошибка, но парсинг прошёл успешно"),
        }
    }
}