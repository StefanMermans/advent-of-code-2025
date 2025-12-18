use std::fmt::Debug;

use clap::Parser;

#[derive(Parser)]
struct Args {
    pub input: String,
}

struct Point {
    x: i64,
    y: i64,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn area_between(p1: &Point, p2: &Point) -> i64 {
    let width = (p1.x - p2.x + 1).abs();
    let height = (p1.y - p2.y + 1).abs();
    let area = width * height;

    area
}

fn part1(points: Vec<Point>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut largest_area = 0;
    
    for point in &points {
        for other_point in &points {
            if point.x == other_point.x && point.y == other_point.y {
                continue;
            }

            let area = area_between(&point, other_point);
            
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    Ok(largest_area)
}

fn line_to_point(line: &str) -> Result<Point, Box<dyn std::error::Error>> {
    let parts = line.split(',').collect::<Vec<_>>();

    if parts.len() != 2 {
        return Err(format!("Invalid line format {}", line).into());
    }

    let x = parts[0].parse()?;
    let y = parts[1].parse()?;

    Ok(Point { x, y })
}

fn get_input(input: &str) -> Result<Vec<Point>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(input)?;
    let lines = contents.lines();
    let mut points = vec![];

    for line in lines {
        points.push(line_to_point(line)?);
    }

    Ok(points)
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input = get_input(&args.input)?;

    println!("Part 1: {}", part1(input)?);

    Ok(())
}
