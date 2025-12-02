use std::fs;
use std::{error::Error};
use std::ops::Range;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let ranges = parse_to_ranges(args.input_file)?;
    let mut sum = 0;

    for range in &ranges {
        for id in range.start..=range.end {
            if !is_id_valid(id) {
                println!("Found invalid id: {}", id);
                sum += id;
            }
        }
    }

    println!("Sum of invalid IDs: {}", sum);
    Ok(())
}

#[derive(Parser)]
#[command(version)]
struct Args {
    input_file: String,
}

fn is_id_valid(id: usize) -> bool {
    let id = id.to_string();

    if id.len() % 2 != 0 {
        return true
    }

    let (first_half, second_half) = id.split_at(id.len() / 2);

    return first_half != second_half;
}

fn parse_to_ranges(file_path: String) -> Result<Vec<Range<usize>>, Box<dyn Error>> {
    Ok(
        fs::read_to_string(file_path)?
        .split(',')
        .map(create_range_from_str)
        .collect::<Result<Vec<Range<usize>>, Box<dyn Error>>>()?
        .into()
    )
}

fn create_range_from_str(input: &str) -> Result<Range<usize>, Box<dyn Error>> {
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid range format \"{}\"", input).into());
    }        

    return Ok(Range {
        start: parts[0].parse()?,
        end: parts[1].parse()?,
    })
}
