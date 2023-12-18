use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = parse_input("../input.txt");

    let polygon_boundary_points_count: i128 = input.iter().map(|(_, steps)| steps).sum();
    let mut polygon_boundary_points = Vec::new();
    for (direction, steps) in input {
        let (last_row, last_col) = if !polygon_boundary_points.is_empty() {
            polygon_boundary_points[polygon_boundary_points.len() - 1]
        } else {
            (0, 0)
        };
        match direction {
            'R' => {
                polygon_boundary_points.push((last_row, last_col + steps));
            }
            'L' => {
                polygon_boundary_points.push((last_row, last_col - steps));
            }
            'D' => {
                polygon_boundary_points.push((last_row + steps, last_col));
            }
            'U' => {
                polygon_boundary_points.push((last_row - steps, last_col));
            }
            _ => panic!("Invalid direction"),
        }
    }
    let polygon_area = shoelace_formula(&polygon_boundary_points);
    let interior_points_count = polygon_area - (polygon_boundary_points_count / 2) + 1;

    println!(
        "Cubic meters: {}",
        interior_points_count + polygon_boundary_points_count
    );
}

fn shoelace_formula(vertices: &Vec<(i128, i128)>) -> i128 {
    let mut sum1 = 0;
    let mut sum2 = 0;
    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;
        sum1 += vertices[i].0 * vertices[j].1;
        sum2 += vertices[j].0 * vertices[i].1;
    }

    ((sum1 - sum2) / 2).abs()
}

fn parse_input<P>(filename: P) -> Vec<(char, i128)>
where
    P: AsRef<Path>,
{
    let mut result = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let line = ip;
            let splitted_line = line.split_whitespace();
            let last_str = splitted_line.last().unwrap();
            let direction = match last_str.chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("Invalid direction"),
            };
            let hex = last_str[2..7].to_string();
            let Ok(steps) = i128::from_str_radix(&hex, 16) else {
                panic!("Invalid steps")
            };
            result.push((direction, steps));
        }
    }
    result
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };
    io::BufReader::new(file).lines()
}
