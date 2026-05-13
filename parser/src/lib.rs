use std::fmt::Display;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::str::FromStr;
use crate::bin_format::parse_bin_to_transaction;
use crate::csv_format::{parse_csv_to_transactions, write_csv};
use crate::errors::{InputError, WriteError};
use crate::txt_format::parse_txt_to_transactions;

mod errors;
pub mod bin_format;
pub mod txt_format;
pub mod csv_format;

// CONSTS

const FORMATS: [&str; 3] = ["txt", "csv", "bin" ];

// ENUMS

#[derive(Debug, Hash, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
    TRANSFER,
}
#[derive(Debug, Hash, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TransactionStatus {
    SUCCESS,
    FAILURE,
    PENDING,
}

// STRUCTS

#[derive(Debug, Hash, Clone, Ord, PartialOrd)]
pub struct TransactionRecord {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: i64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub description: String,
}

pub struct Parser {}

// TRAITS

impl PartialEq for TransactionRecord {
    fn eq(&self, other: &Self) -> bool {
        self.tx_id == other.tx_id
    }
}
impl Eq for TransactionRecord {}
impl FromStr for TransactionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DEPOSIT" => Ok(Self::DEPOSIT),
            "WITHDRAWAL" => Ok(Self::WITHDRAWAL),
            "TRANSFER" => Ok(Self::TRANSFER),
            _ => Err(format!("Unknown TransactionType: {}", s)),
        }
    }
}
impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DEPOSIT => f.write_str("DEPOSIT"),
            Self::WITHDRAWAL => f.write_str("WITHDRAWAL"),
            Self::TRANSFER => f.write_str("TRANSFER"),
        }
    }
}
impl FromStr for TransactionStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUCCESS" => Ok(Self::SUCCESS),
            "FAILURE" => Ok(Self::FAILURE),
            "PENDING" => Ok(Self::PENDING),
            _ => Err(format!("Unknown TransactionStatus: {}", s)),
        }
    }
}
impl Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::FAILURE => f.write_str("FAILURE"),
            Self::PENDING => f.write_str("PENDING"),
        }
    }
}

// METHODS

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }
    /// Пользовательский интерфейс со всеми проверками на ошибки типа InputError
    /// TODO Переписать тип возврата на HashSet
    pub fn parse_file(self, file_name: String) -> Result<Vec<TransactionRecord>, InputError> {
        let file_content = File::open(&file_name);
        match file_content {
            Ok(_file) => {
                let file_name_vec = file_name.splitn(2, ".").collect::<Vec<_>>();
                if let [_, format] = file_name_vec.as_slice() {
                    match format {
                        &"txt" => Ok(self.parse_transactions_from_txt(_file)),
                        &"csv" => Ok(self.parse_transactions_from_csv(_file)),
                        &"bin" => Ok(self.parse_transactions_from_bin(_file)),
                        _ => Err(InputError::InvalidFormat { expected: FORMATS.join(", ") }),
                    }
                } else {
                    Err(InputError::InvalidName(file_name))
                }
            }
            _ => Err(InputError::FileNotFound(file_name))
        }
    }

    fn parse_transactions_from_txt<R: Read>(self, file: R) -> Vec<TransactionRecord> {
        println!("Парсим Транзакции из txt-файла...");
        match parse_txt_to_transactions(file) {
            Ok(transactions) => transactions,
            Err(e) => panic!("{}", e)
        }
    }
    fn parse_transactions_from_csv<R: Read>(self, file: R) -> Vec<TransactionRecord> {
        println!("Парсим Транзакции из csv-файла...");
        match parse_csv_to_transactions(file) {
            Ok(transactions) => transactions,
            Err(e) => panic!("{}", e)
        }
    }
    fn parse_transactions_from_bin<R: Read>(self, file: R) -> Vec<TransactionRecord> {
        println!("Парсим Транзакции из bin-файла...");
        match parse_bin_to_transaction(file) {
            Ok(transactions) => transactions,
            Err(e) => panic!("{}", e)
        }
    }
    pub fn write_to_file(self, output_file_name: &str, transactions: Vec<TransactionRecord>) -> Result<(), WriteError> {
        let file = File::create(output_file_name)?;
        let writer: Result<_, WriteError> = Ok(BufWriter::new(file));
        match writer {
            Ok(writer) => {
                let file_name_vec = output_file_name.splitn(2, ".").collect::<Vec<_>>();
                if let [_, format] = file_name_vec.as_slice() {
                    match format {
                        &"txt" => Ok(self.write_transactions_to_txt(transactions, writer)),
                        &"csv" => Ok(self.write_transactions_to_csv(transactions, writer)),
                        &"bin" => Ok(self.write_transactions_to_bin(transactions, writer)),
                        _ => Err(WriteError::InputError(InputError::InvalidFormat { expected: FORMATS.join(", ") })),
                    }?;
                } else {
                    return Err(WriteError::InputError(InputError::InvalidName(output_file_name.to_string())));
                }
            }
            _ => {}
        }
        Ok(())
    }
    fn write_transactions_to_txt<W: Write>(self, transactions: Vec<TransactionRecord>, mut file: W) {
        println!("Записываем Транзакции в txt-файл...");
        match write_csv(transactions, &mut file) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
    fn write_transactions_to_csv<W: Write>(self, transactions: Vec<TransactionRecord>, mut file: W) {
        println!("Записываем Транзакции в csv-файл...");
        match write_csv(transactions, &mut file) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
    fn write_transactions_to_bin<W: Write>(self, transactions: Vec<TransactionRecord>, mut file: W) {
        println!("Записываем Транзакции в bin-файл...");
        match write_csv(transactions, &mut file) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_record_partial_eq_trait() {
        let tr = TransactionRecord {
            tx_id: 0,
            tx_type: TransactionType::DEPOSIT,
            from_user_id: 0,
            to_user_id: 0,
            amount: 0,
            timestamp: 0,
            status: TransactionStatus::PENDING,
            description: "".to_string(),
        };
        let tr1 = TransactionRecord {
            tx_id: 0,
            tx_type: TransactionType::WITHDRAWAL,
            from_user_id: 12,
            to_user_id: 15,
            amount: 626,
            timestamp: 3435,
            status: TransactionStatus::SUCCESS,
            description: "".to_string(),
        };
        assert_eq!(tr, tr1);
    }
}
