use std::cmp::{max, min};
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
}

fn main() {
    let grid_wrapper = Grid::new(parse_input("../input.txt"));

    let (rows_to_expand, columns_to_expand) = get_indexes_of_vectors_to_expand(&grid_wrapper);
    let points = get_points(&grid_wrapper);
    let pairs = get_pairs(&points);

    const EXPANSION: usize = 1_000_000;
    let distances = get_distances(&pairs, &rows_to_expand, &columns_to_expand, &EXPANSION);

    println!("Sum: {}", distances.iter().sum::<usize>());
}

fn get_indexes_of_vectors_to_expand(grid_wrapper: &Grid) -> (Vec<usize>, Vec<usize>) {
    let mut rows_to_expand: Vec<usize> = Vec::new();
    let mut columns_to_expand: Vec<usize> = Vec::new();
    for i in 0..grid_wrapper.grid.len() {
        let row = grid_wrapper.get_row(i).unwrap();
        if !row.iter().any(|c| *c == '#') {
            rows_to_expand.push(i);
        }
    }
    for i in 0..grid_wrapper.grid[0].len() {
        let column = grid_wrapper.get_column(i).unwrap();
        if !column.iter().any(|c| *c == '#') {
            columns_to_expand.push(i);
        }
    }
    (rows_to_expand, columns_to_expand)
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

fn get_pairs(points: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            pairs.push((points[i], points[j]));
        }
    }
    pairs
}

fn get_distances(
    pairs: &Vec<((usize, usize), (usize, usize))>,
    rows_to_expand: &Vec<usize>,
    columns_to_expand: &Vec<usize>,
    expansion: &usize,
) -> Vec<usize> {
    pairs
        .into_iter()
        .map(|pair| {
            calculate_distance(pair.0, pair.1, rows_to_expand, columns_to_expand, expansion)
        })
        .collect()
}

fn calculate_distance(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    rows_to_expand: &Vec<usize>,
    columns_to_expand: &Vec<usize>,
    expansion: &usize,
) -> usize {
    let dx = (x1 as isize - x2 as isize).abs();
    let dy = (y1 as isize - y2 as isize).abs();
    let mut distance = (dx + dy) as usize;

    for row in min(x1, x2) + 1..max(x1, x2) {
        if rows_to_expand.contains(&row) {
            distance += expansion - 1
        }
    }
    for col in min(y1, y2) + 1..max(y1, y2) {
        if columns_to_expand.contains(&col) {
            distance += expansion - 1
        }
    }
    distance
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
