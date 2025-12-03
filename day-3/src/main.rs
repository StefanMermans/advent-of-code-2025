use std::{error::Error, fs, num::ParseIntError};
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    input: String,

    #[arg(long, short, default_value_t = false)]
    second_part: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let batteries_to_turn_on = batteries_to_turn_on(&args);

    let total_joltage = fs::read_to_string(args.input)?
        .split('\n')
        .filter(|bank|!bank.is_empty())
        .map(|bank|get_joltage(bank, batteries_to_turn_on).unwrap())
        .sum::<i64>();

    println!("Total joltage: {} jolts", total_joltage);
    Ok(())
}

fn get_joltage(bank: &str, remaining: usize) -> Result<i64, ParseIntError> {
    let mut result = String::new();
    let mut remaining = remaining;
    let mut offset = 0;
    
    while remaining > 0 {
        let slice = bank[offset..=(bank.len() - remaining)].to_string();
        let (largest_index, largets_value)= get_largest_value(slice);
        result.push(largets_value);
        offset += largest_index + 1;
        remaining -= 1;
    }

    return result.parse::<i64>();
}

fn batteries_to_turn_on(args: &Args) -> usize {
    if args.second_part {
        return 12;
    }

    return 2;
}

fn get_largest_value(bank_slice: String) -> (usize, char) {
    let mut largest: char = '0';
    let mut largest_index = 0;

    for (index, char) in bank_slice.char_indices() {
        if char > largest {
            largest = char;
            largest_index = index;

            if largest == '9' {
                break;
            }
        }
    }

    return (largest_index, largest);
}
