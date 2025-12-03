use std::{error::Error};
use day_3::{Args, total_joltage};

#[test]
fn part_1_example() -> Result<(), Box<dyn Error>>{
    let args = Args {
        input: "input-example.txt".to_string(),
        second_part: false
    };

    let result = total_joltage(args)?;

    assert_eq!(result, 357);
    Ok(())
}

#[test]
fn part_2_example() -> Result<(), Box<dyn Error>> {
    let args = Args {
        input: "input-example.txt".to_string(),
        second_part: true,
    };

    let result = total_joltage(args)?;

    assert_eq!(result, 3121910778619);
    Ok(())
} 

#[test]
fn file_not_found() -> Result<(), Box<dyn Error>> {
    let args = Args {
        input: "invalid-file.not-real".to_string(),
        second_part: true,
    };

    let result = total_joltage(args);

    assert_eq!(result.is_err(), true);
    Ok(())
}
