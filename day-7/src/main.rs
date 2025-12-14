use core::net;
use std::{collections::HashMap, error::Error, f32::consts::E, fs};

use clap::Parser;

#[derive(Parser)]
struct Args {
    pub input: String,

    #[arg(long, default_value_t = false)]
    pub part_two: bool,
}

fn get_input(args: &Args) -> Result<Vec<Vec<char>>, std::io::Error> {
    let input: Vec<Vec<char>> = fs::read_to_string(&args.input)?
        .split('\n')
        .filter(|line|!line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    Ok(input)
}

fn part1(input: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>> {
    let mut split_count = 0;
    let mut next_line: Vec<char> = vec!['.'; input[0].len()];
    let mut template: Vec<char>;

    for line in input {
        template = next_line.clone();
        next_line = vec!['.'; line.len()];

        for (index, value) in line.iter().enumerate() {
            match value {
                'S' => {
                    next_line[index] = 'S';
                },
                '.' => {
                    if template[index] == '|' || template[index] == 'S' {
                        next_line[index] = '|';
                    }
                }
                '^' => {
                    if template[index] == '|' || template[index] == 'S' {
                        next_line[index -1] = '|';
                        next_line[index + 1] = '|';
                        split_count += 1;
                    }
                    next_line[index] = '^';
                }
                _ => {
                    return Err(format!("Unexpected character: {}", value).into());
                }
            } 
        }

        println!("{}", next_line.iter().collect::<String>());
    }

    Ok(split_count)
}

fn count_timelines(
    input: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> Result<u64, Box<dyn Error>> {
    if row >= input.len() || col >= input[0].len() {
        return Ok(1);
    }
    
    if let Some(&cached) = memo.get(&(row, col)) {
        return Ok(cached);
    }

    let result = match input[row][col] {
        '.' | 'S' => {
            count_timelines(input, row + 1, col, memo)?
        }
        '^' => {
            let mut total = count_timelines(input, row + 1, col - 1, memo)?;
            total += count_timelines(input, row + 1, col + 1, memo)?;
            total
        }
        _ => {
            return Err(format!("Unexpected character: {}", input[row][col]).into())
        },
    };

    memo.insert((row, col), result);

    Ok(result)
}

fn part2(input: &Vec<Vec<char>>) -> Result<u64, Box<dyn Error>> {
    let mut start_col = None;

    for (index, value) in input[0].iter().enumerate() {
        if *value == 'S' {
            start_col = Some(index);
            break;
        }
    }

    let start_col = start_col.ok_or("No S found")?;
    let mut memo = HashMap::new();

    return count_timelines(input, 1, start_col, &mut memo);
}
    
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input = get_input(&args)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}
