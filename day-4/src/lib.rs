use clap::Parser;
use std::{error::Error, fs};

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

    for y in 0..grid.rows.len() {
        for x in 0..grid.rows()[y].len() {
            if !grid.is_roll(x as isize, y as isize) {
                continue;
            }

            if grid.is_accessible(x as isize, y as isize) {
                accessible_count += 1;

                if second_part {
                    extra = true;
                    grid.remove_roll(x, y);
                }
            }
        }
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
            rows: input_data
                .split('\n')
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    pub fn rows(&self) -> &Vec<Vec<char>> {
        &self.rows
    }

    pub fn char_at(&self, x: isize, y: isize) -> char {
        if x < 0
            || y < 0
            || y >= self.rows.len() as isize
            || x >= self.rows[y as usize].len() as isize
        {
            return '.';
        }

        return self.rows[y as usize][x as usize];
    }

    pub fn is_roll(&self, x: isize, y: isize) -> bool {
        return self.char_at(x, y) == '@';
    }

    pub fn remove_roll(&mut self, x: usize, y: usize) {
        self.rows[y][x] = '.';
    }

    pub fn is_accessible(&self, x: isize, y: isize) -> bool {
        let mut adjacent_rolls = 0;

        for offset_x in -1..=1 {
            for offset_y in -1..=1 {
                let new_x = x + offset_x;
                let new_y = y + offset_y;

                if new_y == y && new_x == x {
                    continue;
                }

                if self.is_roll(new_x, new_y) {
                    adjacent_rolls += 1;

                    if adjacent_rolls >= 4 {
                        return false;
                    }
                }
            }
        }

        true
    }
}
