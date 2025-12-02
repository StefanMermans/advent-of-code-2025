use std::{fs::File, io::Error};
use clap::Parser;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    pub file_path: String,
    #[arg(short, long, default_value_t = false, help = "Part 2 flag")]
    pub pt2: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file_reader = input_file_reader(&args)?;
    
    let mut count = 0;
    let mut current_number = 50;

    for line in file_reader.lines() {
        let (next_number, ticked_0) = next_number(current_number, parse_line(line?)?);
        current_number = next_number;

        if current_number == 0 {
            count += 1;
        }

        if args.pt2  {
            count += ticked_0;
        }
    }

    println!("Total 0's: {}", count);

    return Ok(());
}

fn parse_line(line: String) -> Result<i32, std::num::ParseIntError> {
    let number = line[1..].parse::<i32>();

    if line.starts_with('L') {
        return number.map(|n| n * -1);
    }

    return number;
}

fn input_file_reader(args: &Args) -> Result<BufReader<File>, Box<Error>> {
    Ok(BufReader::new(File::open(&args.file_path)?))
}

fn next_number(current: i32, line_number: i32) -> (i32, i32) {
    let mut next = current + (line_number % 100);
    let mut ticked_0 = line_number.abs() / 100;

    if next < 0 {
        if current != 0 {
            ticked_0 += 1;
        }
        next = 100 + next;
    } else if next > 100 {
        if current != 0 {
            ticked_0 += 1;
        }
        next = next - 100;
    }
    
    return (next % 100, ticked_0);
}


