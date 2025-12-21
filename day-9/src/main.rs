mod part1;
mod part2;
mod utils;
use clap::Parser;
use crate::part1::solve_part1;
use crate::part2::solve_part2;
use crate::utils::{get_input, Point};

#[derive(Parser)]
struct Args {
    pub input: String,
}



fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input = get_input(&args.input)?;

    println!("Part 1: {}", solve_part1(&input)?);
    println!("Part 2: {}", solve_part2(&input)?);

    Ok(())
}
