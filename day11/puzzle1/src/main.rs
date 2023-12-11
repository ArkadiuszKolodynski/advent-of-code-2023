use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    fn get_row(&self, x: usize) -> Option<&Vec<char>> {
        self.grid.get(x)
    }

    fn insert_row_at(&mut self, x: usize, row: Vec<char>) {
        self.grid.insert(x, row);
    }

    fn get_column(&self, y: usize) -> Option<Vec<char>> {
        let mut column: Vec<char> = Vec::new();
        for row in self.grid.iter() {
            if let Some(c) = row.get(y) {
                column.push(*c);
            }
        }
        if column.is_empty() {
            None
        } else {
            Some(column)
        }
    }

    fn insert_column_at(&mut self, y: usize, column: Vec<char>) {
        for (i, row) in self.grid.iter_mut().enumerate() {
            if let Some(c) = column.get(i) {
                row.insert(y, *c);
            }
        }
    }
}

fn main() {
    let mut grid_wrapper = Grid::new(parse_input("../input.txt"));

    expand_grid(&mut grid_wrapper);

    let points = get_points(&grid_wrapper);
    let pairs = get_pairs(&points);
    let distances = get_distances(&pairs);
    println!("Sum: {}", distances.iter().sum::<usize>());
}

fn calculate_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let dx = (x1 as isize - x2 as isize).abs();
    let dy = (y1 as isize - y2 as isize).abs();
    (dx + dy) as usize
}

fn get_distances(pairs: &Vec<((usize, usize), (usize, usize))>) -> Vec<usize> {
    pairs
        .into_iter()
        .map(|pair| calculate_distance(pair.0 .1, pair.0 .0, pair.1 .1, pair.1 .0))
        .collect()
}

fn get_pairs(points: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            pairs.push((points[i], points[j]));
        }
    }
    pairs
}

fn get_points(grid_wrapper: &Grid) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    for (i, row) in grid_wrapper.grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                points.push((i, j));
            }
        }
    }
    points
}

fn expand_grid(grid_wrapper: &mut Grid) {
    for i in (0..grid_wrapper.grid.len()).rev() {
        let row = grid_wrapper.get_row(i).unwrap();
        if row.iter().all(|c| *c == '.') {
            grid_wrapper.insert_row_at(i + 1, vec!['.'; grid_wrapper.grid[0].len()])
        }
    }
    for i in (0..grid_wrapper.grid[0].len()).rev() {
        let column = grid_wrapper.get_column(i).unwrap();
        if column.iter().all(|c| *c == '.') {
            grid_wrapper.insert_column_at(i + 1, vec!['.'; grid_wrapper.grid.len()])
        }
    }
}

fn parse_input<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            input.push(ip.chars().collect());
        }
    }
    input
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
