use crate::utils::{Point, area_between};

pub fn solve_part1(points: &Vec<Point>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut largest_area = 0;

    for point in points {
        for other_point in points {
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
