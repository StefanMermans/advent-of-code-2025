use std::error::Error;
use clap::Parser;
use day_4::{Args, compute_result};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let result = compute_result(args)?;

    println!("Result: {}", result);
    Ok(())
}
