use parser::{Parser, TransactionRecord};
use std::{env, io};
use std::collections::BTreeSet;
use std::io::Write;

/// Сравнивает два вектора с транзакциями, возвращает массив с сетами идентификаторов различий
fn compare_vecs(left: &Vec<TransactionRecord>, right: &Vec<TransactionRecord>) -> [BTreeSet<u64>;2] {
    let set_left: BTreeSet<_> = left.into_iter().collect();
    let set_right: BTreeSet<_> = right.into_iter().collect();
    let left_has_ids: BTreeSet<u64> = set_left.difference(&set_right)
        .map(|t| t.tx_id.clone())
        .collect();
    let right_has_ids: BTreeSet<u64> = set_right.difference(&set_left)
        .map(|t| t.tx_id.clone())
        .collect();
    [left_has_ids, right_has_ids]
}

fn main() {
    let mut stdout = io::stdout();
    // первым в аргументы передаётся путь утилиты
    let mut args = env::args().skip(1);
    let mut left = Vec::new();
    let mut right: Vec<TransactionRecord>;

    while let Some(arg) = args.next() {
        let file_name = match arg {
            arg if arg.starts_with("--file") => { args.next() },
            arg => Some(arg)
        };
        if let Some(file_name) = file_name {
            if left.is_empty() {
                if let Ok(_left) = Parser::new().parse_file(file_name) {
                    left = _left;
                    continue;
                } else { eprintln!("Ошибка чтения файла слева"); break; }
            } else {
                if let Ok(_right) = Parser::new().parse_file(file_name) {
                    right = _right;
                } else { eprintln!("Ошибка чтения файла справа"); break; }
                let [left_ids, right_ids] = compare_vecs(&left, &right);
                if left_ids.is_empty() && right_ids.is_empty() {
                    println!("Транзакции в обоих файлах идентичны")
                } else if right_ids.is_empty() {
                    println!("В левом файле отсутствует {} Транзакций: {:?}", &left_ids.len(), &left_ids);
                } else {
                    println!("В правом файле отсутствует {} Транзакций: {:?}", &right_ids.len(), &right_ids);
                }
                left.clear();
                right.clear();
            }
        }
        if let Err(e) = stdout.flush() {
            eprintln!("Ошибка вывода: {}", e);
            std::process::exit(1);
        };
    }
}