use std::error::Error;
use clap::Parser;
use day_3::{Args, total_joltage};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let total_joltage = total_joltage(args)?;

    println!("Total joltage: {} jolts", total_joltage);
    Ok(())
}
