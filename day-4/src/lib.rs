use std::{error::Error, fs};
use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Args {
    pub input: String,

    #[arg(long, short, default_value_t = false)]
    pub second_part: bool,
}

pub fn compute_result(args: Args) -> Result<i32, Box<dyn Error>> {
    let input = fs::read_to_string(args.input)?;
    let grid = Grid::new(input);
    let mut accessible_count = 0;

    for (row_index, row) in grid.rows().iter().enumerate() {
        for (col_index, position) in row.iter().enumerate() {
            if *position != '@' {
                print!("{}", position);
                continue;
            }

            let rol_count = grid
                .neighbors(col_index as isize, row_index as isize)
                .iter()
                .filter(|char|**char == '@')
                .count();

            if rol_count < 4 {
                accessible_count += 1;
                print!("x")
            } else {
                print!("{}", position);
            }
        }

        println!("")
    }

    return Ok(accessible_count);
}

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(input_data: String) -> Self {
        Self {
            rows: input_data.split('\n').map(|line| line.chars().collect()).collect()
        }
    }

    pub fn rows(&self) -> &Vec<Vec<char>> {
        &self.rows
    }

    pub fn char_at(&self, x: isize, y: isize) -> char {
        if y < 0 || x < 0 {
            return '.';
        }

        let row_index = y as usize;
        let col_index = x as usize;

        if row_index >= self.rows.len() {
            return '.';
        }

        if col_index >= self.rows[row_index].len() {
            return '.';
        }

        return self.rows[row_index][col_index];
    }

    pub fn neighbors(&self, x: isize, y: isize) -> Vec<char> {
        (-1..=1).flat_map(move |offset_x|{
            (-1..=1).flat_map(move |offset_y|{
                 let new_x = x as isize + offset_x;
                 let new_y = y as isize + offset_y;

                if new_y == y && new_x == x {
                    return None;
                }

                 let value = self.char_at(new_x, new_y);

                 return Some(value);
            })
        })
        .collect()
    }
}

