use std::{error::Error, fmt::Debug, fs};

use clap::Parser;

#[derive(Parser)]
struct Args {
    pub input: String,
    #[clap(short, long)]
    pub connections: i32,
}

#[derive(Clone)]
struct JunctionBox {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub direct_connections: Vec<usize>,
    pub indirect_connections: Vec<usize>,
}

impl Debug for JunctionBox {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "({},{},{})", self.x, self.y, self.z)
    }    
}

impl JunctionBox {
    fn distance_to(&self, other: &JunctionBox) -> f64 {
        let delta_x = (self.x - other.x).abs();
        let delta_y = (self.y - other.y).abs();
        let delta_z = (self.z - other.z).abs();

        return (
            delta_x * delta_x +
            delta_y * delta_y +
            delta_z * delta_z
        ).sqrt();
    }

    fn connect(&mut self, other: &mut JunctionBox) {
        self.direct_connections.push(other.id);
        other.direct_connections.push(self.id);

        for &indirect_id in &self.indirect_connections {
            if indirect_id != other.id && !other.direct_connections.contains(&indirect_id) && !other.indirect_connections.contains(&indirect_id) {
                other.indirect_connections.push(indirect_id);
            }
        }
    }

    fn is_directly_connected_to(&self, other: &JunctionBox) -> bool {
        self.direct_connections.contains(&other.id)
    }

    pub fn circuit_size(&self, boxes: &Vec<JunctionBox>, visited: &mut Vec<bool>) -> usize {
        let mut size = 0;
        let mut queue = vec![self.id];
        visited[self.id] = true;
        while let Some(idx) = queue.pop() {
            size += 1;
            let box_ref = &boxes[idx];
            let neighbors = box_ref.direct_connections.iter().chain(box_ref.indirect_connections.iter());
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push(neighbor);
                }
            }
        }
        size
    }
}

fn line_to_junction_box(line: &str, index: usize) -> Result<JunctionBox, Box<dyn Error>> {
    let values: Vec<f64> = line
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect();

    if values.len() != 3 {
        return Err(format!("Expected exactly 3 number in line {}", line).into());
    }   

    Ok(JunctionBox {
        id: index,
        x: values[0],
        y: values[1],
        z: values[2],
        direct_connections: vec![],
        indirect_connections: vec![],
    })
}

fn parse_input(input_path: &str) -> Result<Vec<JunctionBox>, Box<dyn Error>> {
    return Ok(fs::read_to_string(input_path)?
        .split('\n')
        .enumerate()
        .map(|(index, line)| line_to_junction_box(line, index))
        .filter_map(Result::ok)
        .collect::<Vec<JunctionBox>>());
}

fn find_shortest_not_directly_connected(
    boxes: &Vec<JunctionBox>,
) -> Result<(usize, usize), Box<dyn Error>>  {
    let mut shortest_a = None;
    let mut shortest_b = None;
    let mut shortest_distance = f64::MAX;

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let box_a = &boxes[i];
            let box_b = &boxes[j];

            if box_a.is_directly_connected_to(box_b) {
                continue;
            }

            let distance = box_a.distance_to(box_b);
            if distance < shortest_distance {
                shortest_distance = distance;
                shortest_a = Some(i);
                shortest_b = Some(j);
            }
        }
    }

    match (shortest_a, shortest_b) {
        (Some(a), Some(b)) => Ok((a, b)),
        _ => Err("No not directly connected boxes found".into()),
    }
}

fn make_connections(boxes: &mut Vec<JunctionBox>, connections_to_make: i32) -> Result<(), Box<dyn Error>> {
    for _ in 0..connections_to_make {
        let (a_index, b_index) = find_shortest_not_directly_connected(boxes)?;

        let (a_box_ptr, b_box_ptr) = if a_index < b_index {
            let (left, right) = boxes.split_at_mut(b_index);
            (&mut left[a_index], &mut right[0])
        } else {
            let (left, right) = boxes.split_at_mut(a_index);
            (&mut right[0], &mut left[b_index])
        };

        println!("Connecting box {:?} to box {:?}", a_box_ptr, b_box_ptr);
        a_box_ptr.connect(b_box_ptr);
    }
    Ok(())
}

fn three_largest_circuit_sizes(boxes: &Vec<JunctionBox>) -> Vec<usize> {
    let mut visited = vec![false; boxes.len()];
    let mut sizes = vec![];

    for box_index in 0..boxes.len() {
        if visited[box_index] {
            continue;
        }
        let size = boxes[box_index].circuit_size(boxes, &mut visited);
        sizes.push(size);
    }
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.truncate(3);
    sizes
}

fn part1(boxes: &mut Vec<JunctionBox>, connections_to_make: i32) -> Result<i32, Box<dyn Error>> {
    make_connections(boxes, connections_to_make)?;
    let sizes = three_largest_circuit_sizes(boxes);
    println!("Three largest circuit sizes: {:?}", sizes);
    let result = sizes.iter().copied().reduce(|a, b| a * b).unwrap_or(0) as i32;
    Ok(result)
}

fn main()  -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut input = parse_input(&args.input)?;
    
    println!("Part 1: {}", part1(&mut input, args.connections)?);

    Ok(())
}
