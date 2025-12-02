use std::collections::HashSet;
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
            if !is_id_valid(id, args.second_part) {
                println!("id {} is invlaid", id);
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

    #[arg(short, long, default_value_t = false)]
    second_part: bool,
}

fn is_id_repeating(id: String) -> bool {
    let mut part_size = 1;

    while part_size < (id.len() + 1) / 2 {
        if id.len() % part_size != 0 {
            part_size += 1;
            continue;
        }

        let parts_count = id.len() / part_size;
        let mut parts: Vec<String> = Vec::new();
        
        
        for part_index in 0..parts_count {
            let start = part_index * part_size;
            parts.push(id[start..start + part_size].to_string());
        }

        if parts
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .count() 
            == 1
         {
            return true;
        }

        part_size += 1;
    }

    return false;
}

fn is_id_valid(id: usize, part2: bool) -> bool {
    let id = id.to_string();

    if id.len() % 2 == 0 {
        let (first_half, second_half) = id.split_at(id.len() / 2);
    
        if first_half == second_half {
            return false;
        }
    }

    if part2 {
        return !is_id_repeating(id);
    }

    return true;
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
