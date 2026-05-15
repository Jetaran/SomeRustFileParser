use std::{env, io};
use std::io::Write;
use parser::{Parser, TransactionRecord};

fn main()  {
    let mut stdout = io::stdout();
    // первым в аргументы передаётся путь утилиты
    let mut args = env::args().skip(1);
    let mut transactions: Vec<TransactionRecord> = vec![];

    while let Some(arg) = args.next() {
        match arg {
            arg if arg.starts_with("--input") => {
                let parser = Parser::new();
                let input_file: String = args.next().unwrap_or(String::new());
                let transactions_from_file = parser.parse_file(input_file);
                match transactions_from_file {
                    Ok(mut transactions_from_file) => { transactions.append(&mut transactions_from_file)}
                    Err(err) => { println!("Чтение файла не удалось {}", err); break }
                }
            },
            arg if arg.starts_with("--output") => {
                let parser = Parser::new();
                let output_file: String = args.next().unwrap_or(String::new());
                let result = parser.write_to_file(&*output_file, transactions.clone());
                match result {
                    Ok(_) => { println!("Запись прошла успешно, {} транзакций записано в файл {}", transactions.len(), output_file); }
                    Err(err) => { println!("Запись файла не удалась {}", err); break }
                }

            },
            _ => ()
        };
    }
    stdout.flush().unwrap();
}