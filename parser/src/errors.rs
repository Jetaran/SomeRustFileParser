use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io::Error as IoError;
use std::num::ParseIntError;

/// Ошибки ввода
#[derive(Debug)]
pub enum InputError {
    FileNotFound(String),
    InvalidName(String),
    InvalidFormat { expected: String },
}
impl Error for InputError {}
impl Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InputError::FileNotFound(err) => {
                write!(f, "Не удалось прочитать файл: {}", err)
            }
            InputError::InvalidName(err) => {
                write!(f, "Неверное имя файла: {}", err)
            }
            InputError::InvalidFormat { expected } => {
                write!(f, "Неверное формат файла, ожидается: {}", expected)
            }
        }
    }
}

/// Ошибки парсинга
#[derive(Debug)]
pub enum ParseError {
    InvalidLine,
    InvalidMagic,
    InvalidDescription,
    ParseIntError(ParseIntError),
    ParseEnumError(String),
}
impl Error for ParseError {}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ParseError::InvalidLine => {
                write!(f, "Ошибка парсинга строки")
            }
            ParseError::InvalidMagic => {
                write!(f, "Ожидался MAGIC 'YPBN' в начале записи")
            }
            ParseError::InvalidDescription => {
                write!(f, "Ошибка парсинга описания")
            }
            ParseError::ParseIntError(err) => {
                write!(f, "Ошибка парсинга целочисленного значения: {}", err)
            }
            ParseError::ParseEnumError(err) => {
                write!(f, "Ошибка парсинга перечисления: {}", err)
            }
        }
    }
}
impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError::ParseIntError(err)
    }
}
impl From<String> for ParseError {
    fn from(err: String) -> Self {
        ParseError::ParseEnumError(err)
    }
}
impl From<IoError> for ParseError {
    fn from(_err: IoError) -> Self {
        ParseError::InvalidLine
    }
}

/// Ошибки записи
#[derive(Debug)]
pub enum WriteError {
    IOError(IoError),
    CsvFormatError(String),
    InputError(InputError),
}
impl Error for WriteError {}
impl Display for WriteError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            WriteError::IOError(err) => {
                write!(f, "Системная ошибка записи файла {}", err)
            }
            WriteError::CsvFormatError(err) => {
                write!(f, "Ожидался записи в формат CSV {}", err)
            }
            WriteError::InputError(err) => {
                write!(f, "Ошибка ввода {}", err)
            }
        }
    }
}
impl From<IoError> for WriteError {
    fn from(err: IoError) -> Self {
        Self::IOError(err)
    }
}