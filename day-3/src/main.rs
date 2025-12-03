use std::{error::Error, fs};
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
    let total_joltage = fs::read_to_string(args.input)?
        .split('\n')
        .filter(|bank|!bank.is_empty())
        .map(|bank|get_joltage(bank).unwrap())
        .sum::<i32>();

    println!("Total joltage: {} jolts", total_joltage);
    Ok(())
}

fn get_joltage(bank: &str) -> Result<i32, Box<dyn Error>> {
    print!("{}", bank);

    let (index, largest_start) = get_largest_value(bank[0..(bank.len()-1)].to_string());
    let (_, largest_end) = get_largest_value(bank[(index+1)..=(bank.len()-1)].to_string());
    let result = format!("{}{}", largest_start, largest_end);

    println!(" = {}", result);

    return Ok(result.parse::<i32>()?);
}

fn get_largest_value(bank: String) -> (usize, char) {
    let mut largest: char = '0';
    let mut largest_index = 0;

    for (index, char) in bank.char_indices() {
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
