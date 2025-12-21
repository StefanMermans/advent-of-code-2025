use std::fmt::Debug;

pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn area_between(p1: &Point, p2: &Point) -> i64 {
    let width = (p1.x - p2.x + 1).abs();
    let height = (p1.y - p2.y + 1).abs();
    let area = width * height;

    area
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

pub fn get_input(input: &str) -> Result<Vec<Point>, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(input)?;
    let lines = contents.lines();
    let mut points = vec![];

    for line in lines {
        points.push(line_to_point(line)?);
    }

    Ok(points)
}