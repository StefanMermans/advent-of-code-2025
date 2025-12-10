use std::{error::Error, fs, str::Chars};

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let input = get_input(&args)?;
    let mut split_count = 0;
    let mut next_line: Vec<char> = vec!['.'; input[0].len()];
    let mut template: Vec<char> = vec!['.'; input[0].len()];

    // println!("Template {}", template.iter().collect::<String>());
    // println!("prev {}", next_line.iter().collect::<String>());

    for line in input {
        template = next_line.clone();
        next_line = vec!['.'; line.len()];
        // println!("line {}", line.iter().collect::<String>());

        for (index, value) in line.iter().enumerate() {
            
            match value {
                'S' => {
                    next_line[index] = '|';
                },
                '.' => {
                    if template[index] == '|' {
                        next_line[index] = '|';
                    }
                }
                '^' => {
                    if template[index] == '|' {
                        next_line[index -1] = '|';
                        next_line[index + 1] = '|';
                        split_count += 1;
                    }
                    next_line[index] = '^';
                }
                _ => {
                    next_line[index] = *value;
                }
            } 
        }

        println!("{}", next_line.iter().collect::<String>());

    }

    println!("Part 1 solution: {}", split_count);
    Ok(())
}
