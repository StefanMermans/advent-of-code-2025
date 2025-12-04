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
    let mut grid = Grid::new(input);
    
    return count_accessible(&mut grid, args.second_part);
}

fn count_accessible(grid: &mut Grid, second_part: bool) -> Result<i32, Box<dyn Error>> {
    let mut accessible_count = 0;
    let mut extra = false;

    for row_index in 0..grid.rows.len() {
        for col_index in 0..grid.rows()[row_index].len() {
            let position = grid.rows()[row_index][col_index];

            if position != '@' {
                print!("{}", position);
                continue;
            }

            let rol_count = grid
                .neighbors(col_index as isize, row_index as isize)
                .iter()
                .filter(|char| **char == '@')
                .count();

            if rol_count < 4 {
                accessible_count += 1;
                
                if second_part {
                    extra = true;
                    grid.remove_roll(col_index, row_index);
                }

                print!("x")
            } else {
                print!("{}", position);
            }
        }

        println!("")
    }

    if extra {
        accessible_count += count_accessible(grid, second_part)?;
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

    pub fn remove_roll(&mut self, x: usize, y: usize) {
        self.rows[y][x] = '.';
    }

    pub fn neighbors(&mut self, x: isize, y: isize) -> Vec<char> {
        let mut result: Vec<char> = Vec::with_capacity(8);

        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                let new_x = x as isize + offset_x;
                let new_y = y as isize + offset_y;

                if new_y == y && new_x == x {
                    continue;
                }

                let value = self.char_at(new_x, new_y);

                result.push(value);
            }
        }

        println!("{:?}", result);

        result
    }
}

