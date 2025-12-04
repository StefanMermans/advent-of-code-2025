use day_4::{Args, compute_result};
use std::error::Error;

#[test]
fn part_1_example() -> Result<(), Box<dyn Error>> {
    let args = Args {
        input: "input-example.txt".to_string(),
        second_part: false,
    };
    let result = compute_result(args)?;

    assert_eq!(result, 13);
    Ok(())
}

#[test]
fn part_2_example() -> Result<(), Box<dyn Error>> {
    let args = Args {
        input: "input-example.txt".to_string(),
        second_part: true,
    };
    let result = compute_result(args)?;

    assert_eq!(result, 43);
    Ok(())
}
