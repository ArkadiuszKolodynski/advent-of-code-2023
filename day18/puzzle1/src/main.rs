use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = parse_input("../input.txt");

    let mut polygon_boundary_points = Vec::new();
    polygon_boundary_points.push((0, 0));
    for (direction, steps) in input {
        match direction {
            'R' => {
                let (last_row, last_col) =
                    polygon_boundary_points[polygon_boundary_points.len() - 1];
                for col in (last_col + 1)..(last_col + steps + 1) {
                    polygon_boundary_points.push((last_row.clone(), col));
                }
            }
            'L' => {
                let (last_row, last_col) =
                    polygon_boundary_points[polygon_boundary_points.len() - 1];
                for col in ((last_col - steps)..(last_col)).rev() {
                    polygon_boundary_points.push((last_row.clone(), col));
                }
            }
            'D' => {
                let (last_row, last_col) =
                    polygon_boundary_points[polygon_boundary_points.len() - 1];
                for row in (last_row + 1)..(last_row + steps + 1) {
                    polygon_boundary_points.push((row, last_col.clone()));
                }
            }
            'U' => {
                let (last_row, last_col) =
                    polygon_boundary_points[polygon_boundary_points.len() - 1];
                for row in ((last_row - steps)..(last_row)).rev() {
                    polygon_boundary_points.push((row, last_col.clone()));
                }
            }
            _ => panic!("Invalid direction"),
        }
    }
    polygon_boundary_points.pop(); // remove doubled (0, 0)
    let polygon_boundary_points_count = polygon_boundary_points.len() as i32;
    let polygon_area = shoelace_formula(&polygon_boundary_points);
    let interior_points_count = polygon_area - (polygon_boundary_points_count / 2) + 1;

    println!(
        "{:?}",
        interior_points_count + polygon_boundary_points_count
    );
}

fn shoelace_formula(vertices: &Vec<(i32, i32)>) -> i32 {
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;
        sum1 += vertices[i].0 as f64 * vertices[j].1 as f64;
        sum2 += vertices[j].0 as f64 * vertices[i].1 as f64;
    }

    ((sum1 - sum2) / 2.0).abs() as i32
}

fn parse_input<P>(filename: P) -> Vec<(char, i32)>
where
    P: AsRef<Path>,
{
    let mut result = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let line = ip;
            let mut asdf = line.split_whitespace();
            let direction = asdf.next().unwrap().chars().next().unwrap();
            let steps = asdf.next().unwrap().parse::<i32>().unwrap();
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
