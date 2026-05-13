use std::io::{BufReader, Cursor, ErrorKind, Read};
use crate::{TransactionRecord, TransactionStatus, TransactionType};
use crate::errors::ParseError;

/// Заголовок для валидации
const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];
/// Читает фиксированное количество байтов из источника
macro_rules! read_fixed {
    ($reader:expr, $size:expr) => {{
        let mut buf = [0u8; $size];
        $reader.read_exact(&mut buf)?;
        buf
    }};
}
/// Парсит тело Транзакции
fn parse_bin_record_body<R: Read>(mut reader: R) -> Result<TransactionRecord, ParseError> {
    let tx_id = u64::from_be_bytes(read_fixed!(reader, 8));
    let tx_type_byte = u8::from_le_bytes(read_fixed!(reader, 1));
    let from_user_id = u64::from_be_bytes(read_fixed!(reader, 8));
    let to_user_id = u64::from_be_bytes(read_fixed!(reader, 8));
    let amount = i64::from_be_bytes(read_fixed!(reader, 8));
    let timestamp = u64::from_be_bytes(read_fixed!(reader, 8));
    let status_byte = u8::from_le_bytes(read_fixed!(reader, 1));
    let desc_len = u32::from_be_bytes(read_fixed!(reader, 4)) as usize;

    let mut desc_bytes = vec![0u8; desc_len];
    reader.read_exact(&mut desc_bytes)?;
    let description = String::from_utf8(desc_bytes).map_err(|_| ParseError::InvalidDescription)?;

    Ok(TransactionRecord {
        tx_id,
        tx_type: match tx_type_byte {
            0 => TransactionType::DEPOSIT,
            1 => TransactionType::WITHDRAWAL,
            2 => TransactionType::TRANSFER,
            _ => return Err(ParseError::ParseEnumError(tx_type_byte.to_string()))
        },
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status: match status_byte {
            0 => TransactionStatus::SUCCESS,
            1 => TransactionStatus::FAILURE,
            2 => TransactionStatus::PENDING,
            _ => return Err(ParseError::ParseEnumError(status_byte.to_string()))
        },
        description,
    })
}
/// Парсит входящий источник до разделителя, выделяет тело Транзакции и передаёт дальше
fn parse_bin_record<R: Read>(mut reader: R) -> Result<Option<TransactionRecord>, ParseError> {
    let mut magic = [0u8; 4];
    match reader.read_exact(&mut magic) {
        Ok(_) => {},
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
        Err(_e) => return Err(ParseError::InvalidLine),
    }
    if magic != MAGIC {
        return Err(ParseError::InvalidMagic);
    }
    let body_size = u32::from_be_bytes(read_fixed!(reader, 4)) as usize;
    let mut body = vec![0u8; body_size];
    reader.read_exact(&mut body)?;
    let transaction = parse_bin_record_body(Cursor::new(&body))?;
    Ok(Some(transaction))
}
/// Читает сам источник
pub(crate) fn parse_bin_to_transaction<R: Read>(content: R) -> Result<Vec<TransactionRecord>, ParseError> {
    let mut buf_reader = BufReader::new(content);
    let mut transactions = Vec::new();

    loop {
        match parse_bin_record(&mut buf_reader) {
            Ok(Some(transaction)) => transactions.push(transaction),
            Ok(None) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(transactions)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;
    #[test]
    fn test_parse_bin_to_transaction() {
        let file = File::open("records_example.bin").unwrap();
        let transactions = parse_bin_to_transaction(file);
        assert!(transactions.is_ok());
        match transactions {
            Ok(transactions) => { println!("{:?}", transactions) },
            Err(e) => { panic!("{}", e) }
        }
    }
}