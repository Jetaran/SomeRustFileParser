use std::io::{BufRead, BufReader, Read, Write};
use crate::errors::{ParseError, WriteError};
use crate::TransactionRecord;

// READ

/// Парсит csv-строку и собирает структуру через замыкание-адаптер (аналог питонячей yield-лямбды)
fn parse_line(line: String) -> Result<TransactionRecord, ParseError> {
    let mut parts = line.trim().split(",");
    let mut next = || parts.next().ok_or(ParseError::InvalidLine);
    Ok(TransactionRecord {
        tx_id:          next()?.parse()?,
        tx_type:        next()?.parse()?,
        from_user_id:   next()?.parse()?,
        to_user_id:     next()?.parse()?,
        amount:         next()?.parse()?,
        timestamp:      next()?.parse()?,
        status:         next()?.parse()?,
        description:    next()?.to_string(),
    })
}

/// Бьёт входящий контент на строки, проверяет их и собирает вектор транзакций
pub(crate) fn parse_csv_to_transactions<R: Read>(content: R) -> Result<Vec<TransactionRecord>, ParseError> {
    let reader = BufReader::new(content);
    let mut transaction_records = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                continue;
            }
            if let Some(fc) = line.chars().next() {
                if !fc.is_ascii_digit() { continue; }
            }
            transaction_records.push(parse_line(line)?);
        }
    }
    Ok(transaction_records)
}

// WRITE

pub(crate) fn write_csv<W: Write>(transactions: Vec<TransactionRecord>, writer: &mut W) -> Result<(), WriteError> {
    for transaction in transactions {
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{}",
            transaction.tx_id,
            transaction.tx_type,
            transaction.from_user_id,
            transaction.to_user_id,
            transaction.amount,
            transaction.timestamp,
            transaction.status,
            transaction.description
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Cursor};
    use crate::{TransactionStatus, TransactionType};
    use super::*;

    #[test]
    fn test_parse_csv_line_success() {
        let test_line = String::from(
            r#"1000000000000019,TRANSFER,8969803948209661815,8940414264323298111,2000,1633038000000,PENDING,"Record number 20""#
        );
        let transaction = parse_line(test_line);

        assert!(transaction.is_ok());

        let valid_transaction = TransactionRecord {
            tx_id:          1000000000000019,
            tx_type:        TransactionType::TRANSFER,
            from_user_id:   8969803948209661815,
            to_user_id:     8940414264323298111,
            amount:         2000,
            timestamp:      1633038000000,
            status:         TransactionStatus::PENDING,
            description:    "Record number 20".to_string(),
        };

        assert_eq!(transaction.unwrap(), valid_transaction);
    }
    #[test]
    fn test_parse_csv_line_wrong_int() {
        let test_line = String::from(
            r#"abyr,abyr,abyr,valg"#
        );
        let transaction = parse_line(test_line);

        match transaction {
            Err(ParseError::ParseIntError(msg)) => {
                assert_eq!(msg.to_string(), "invalid digit found in string".to_string());
            }
            Err(other) => panic!("Ожидалась ParseIntError, получено: {:?}", other),
            Ok(_) => panic!("Ожидалась ошибка, но парсинг прошёл успешно"),
        }
    }
    #[test]
    fn test_parse_csv_line_wrong_enum() {
        let test_line = String::from(
            r#"1000000000000019,ABYRVALG,8969803948209661815,8940414264323298111,2000,1633038000000,PENDING,"Record number 20""#
        );
        let transaction = parse_line(test_line);

        match transaction {
            Err(ParseError::ParseEnumError(msg)) => {
                assert_eq!(msg.to_string(), "Unknown TransactionType: ABYRVALG".to_string());
            }
            Err(other) => panic!("Ожидалась ParseEnumError, получено: {:?}", other),
            Ok(_) => panic!("Ожидалась ошибка, но парсинг прошёл успешно"),
        }
    }
    #[test]
    fn parse_csv_to_transactions_success() {
        let fake_file = concat!(
            "1000000000000009,DEPOSIT,0,9223372036854775807,1000,1633037400000,FAILURE,", r#""Record number 10""#, "\n",
            "1000000000000994,TRANSFER,9223372036854775807,9223372036854775807,99500,1633096500000,PENDING,", r#""Record number 995""#, "\n"
        );
        let cursor = Cursor::new(fake_file.as_bytes());
        let records = parse_csv_to_transactions(cursor).unwrap();

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].description, r#""Record number 10""#);
        assert_eq!(records[1].description, r#""Record number 995""#);
    }
    #[test]
    fn parse_csv_to_transactions_wrong_line() {
        let fake_file = concat!(
            "1000000000000009,DEPOSIT,0,9223372036854775807,1000,1633037400000,FAILURE,", r#""Record number 10""#, "\n",
            "1000000000000994,TRANSFER,9223372036854775807"
        );
        let cursor = Cursor::new(fake_file.as_bytes());
        let records = parse_csv_to_transactions(cursor);

        match records {
            Err(ParseError::InvalidLine) => {
                println!("Тест parse_csv_to_transactions_wrong_line прошёл успешно");
            }
            Err(other) => panic!("Ожидалась InvalidLine, получено: {:?}", other),
            Ok(_) => panic!("Ожидалась ошибка, но парсинг прошёл успешно"),
        }
    }
    #[test]
    fn write_parsed_files_to_csv_success() {
        let fake_file = concat!(
        "1000000000000009,DEPOSIT,0,9223372036854775807,1000,1633037400000,FAILURE,", r#""Record number 10""#, "\n",
        "1000000000000994,TRANSFER,9223372036854775807,9223372036854775807,99500,1633096500000,PENDING,", r#""Record number 995""#, "\n"
        );
        let reader = Cursor::new(fake_file.as_bytes());
        let records = parse_csv_to_transactions(reader).unwrap();
        let mut buffer = Vec::new();
        // writer заимствует буфер, но дальше уже не нужен
        {
            let mut writer = BufWriter::new(&mut buffer);
            write_csv(records, &mut writer).unwrap();
            writer.flush().unwrap();
        }
        let output = String::from_utf8(buffer).unwrap();

        assert_eq!(output, fake_file);
    }
}