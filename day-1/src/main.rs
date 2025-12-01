use std::fs::File;
use std::{env};
use std::io::{BufRead, BufReader};

fn main() {
    let file_reader = match input_file_reader() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    
    let mut count = 0;
    let mut current_number = 50;

    for line in file_reader.lines() {
        let number = match parse_line(line.unwrap()) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Failed to parse line to number.");
                
                return;
            },
        };

        current_number = next_number(current_number, number);

        if current_number == 0 {
            count += 1;
        }
    }

    println!("Total 0's: {}", count);
}

fn parse_line(line: String) -> Result<i32, std::num::ParseIntError> {
    let number = line[1..].parse::<i32>();

    if line.starts_with('L') {
        return number.map(|n| n * -1);
    }

    return number;
}

fn input_file_reader() -> Result<BufReader<File>, String> {
    let args: Vec<String> = env::args().collect();

     let file_path = match args.get(1) {
        Some(path) => path.to_string(),
        None => return Err("Please provide the input file path as a command line argument.".into()),
    };

    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Failed to open the specified file. {}", e)),
    };
    
    return Ok(BufReader::new(file));
}

fn next_number(current: i32, line_number: i32) -> i32 {
    return (current + line_number) % 100;
}


