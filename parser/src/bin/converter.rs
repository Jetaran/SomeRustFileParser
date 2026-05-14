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
                transactions.append(&mut parser.parse_file(args.next().unwrap().to_string()).unwrap())
            },
            arg if arg.starts_with("--output") => {
                let parser = Parser::new();
                let output_file = args.next().unwrap().to_string();
                parser.write_to_file(&*output_file, transactions.clone()).unwrap();
                println!("Запись прошла успешно, {} транзакций записано в файл {}", transactions.len(), output_file);
            },
            _ => ()
        };
    }
    stdout.flush().unwrap();
}