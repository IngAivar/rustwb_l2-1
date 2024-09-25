use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Получаем список аргументов командной строки
    let args: Vec<String> = env::args().collect();

    // Проверяем, есть ли хотя бы один аргумент (имя файла)
    if args.len() < 2 {
        // Если аргументов недостаточно, выводим сообщение об использовании программы
        eprintln!("Usage: {} [-c|-l|-w] <filename>", args[0]);
        return;
    }

    // Получаем имя файла из первого аргумента
    let filename = &args[1];

    // Устанавливаем значение по умолчанию для типа подсчета
    let default_type = "-w".to_string();

    // Получаем тип подсчета из второго аргумента (если он есть), иначе используем значение по умолчанию
    let count_type = args.get(2).unwrap_or(&default_type);

    // Открываем файл для чтения
    let file = match File::open(filename) {
        // Если файл открылся успешно, продолжаем
        Ok(file) => file,
        // Если произошла ошибка при открытии файла, выводим сообщение об ошибке и завершаем программу
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            return;
        }
    };

    // Создаем буфер для чтения строк из файла
    let reader = BufReader::new(file);

    // Инициализируем счетчики строк, слов и байтов
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    // Перебираем строки в файле
    for line in reader.lines() {
        // Извлекаем строку из результата чтения
        let line = line.as_ref().unwrap();

        // Увеличиваем счетчик строк
        lines += 1;

        // Увеличиваем счетчик байтов на длину строки
        bytes += line.len();

        // Увеличиваем счетчик слов на количество слов в строке
        words += line.split_whitespace().count();
    }

    // В зависимости от типа подсчета выводим результат
    match count_type.as_str() {
        "-c" => println!("{}", bytes),
        "-l" => println!("{}", lines),
        "-w" => println!("{}", words),
        _ => println!("Invalid option: {}", count_type),
    }
}

// src\wc.rs -c report.txt