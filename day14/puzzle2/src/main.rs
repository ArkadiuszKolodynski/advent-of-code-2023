use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

    fn set_row(&mut self, x: usize, row: Vec<char>) {
        self.grid[x] = row;
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

    fn set_column(&mut self, y: usize, column: Vec<char>) {
        for (i, row) in self.grid.iter_mut().enumerate() {
            if let Some(c) = column.get(i) {
                row[y] = *c;
            }
        }
    }
}

fn main() {
    let mut grid_wrapper = parse_input("../input.txt");

    let mut cache = HashMap::new();
    cache.insert(grid_wrapper.clone(), 0);

    let (start, end) = loop {
        tilt_north(&mut grid_wrapper);
        tilt_west(&mut grid_wrapper);
        tilt_south(&mut grid_wrapper);
        tilt_east(&mut grid_wrapper);

        if let Some(previous) = cache.insert(grid_wrapper.clone(), cache.len()) {
            break (previous, cache.len());
        }
    };

    let offset = 1_000_000_000 - start;
    let cycle_width = end - start;
    let remainder = offset % cycle_width;
    let target = start + remainder;

    let (grid, _) = cache.iter().find(|(_, &i)| i == target).unwrap();
    let sum: usize = (0..grid.grid.len())
        .map(|i| {
            let row = grid.get_row(i).unwrap();
            row.iter().filter(|c| **c == 'O').collect::<Vec<_>>().len() * (grid.grid.len() - i)
        })
        .sum();

    println!("Sum: {}", sum);
}

fn tilt_north(grid_wrapper: &mut Grid) {
    for i in 0..grid_wrapper.grid[0].len() {
        let col = grid_wrapper.get_column(i).unwrap();
        grid_wrapper.set_column(i, sort_vec(col, true));
    }
}

fn tilt_west(grid_wrapper: &mut Grid) {
    for i in 0..grid_wrapper.grid.len() {
        let row = grid_wrapper.get_row(i).unwrap().clone();
        grid_wrapper.set_row(i, sort_vec(row, true));
    }
}

fn tilt_south(grid_wrapper: &mut Grid) {
    for i in 0..grid_wrapper.grid[0].len() {
        let col = grid_wrapper.get_column(i).unwrap();
        grid_wrapper.set_column(i, sort_vec(col, false));
    }
}

fn tilt_east(grid_wrapper: &mut Grid) {
    for i in 0..grid_wrapper.grid.len() {
        let row = grid_wrapper.get_row(i).unwrap().clone();
        grid_wrapper.set_row(i, sort_vec(row, false));
    }
}

fn sort_vec(vec: Vec<char>, reverse_order: bool) -> Vec<char> {
    vec.split(|c| *c == '#')
        .map(|slice| {
            let mut sorted_slice = slice.to_vec();
            sorted_slice.sort_by(|a, b| if reverse_order { b.cmp(a) } else { a.cmp(b) });
            sorted_slice
        })
        .collect::<Vec<Vec<char>>>()
        .join(&'#')
}

fn parse_input<P>(filename: P) -> Grid
where
    P: AsRef<Path>,
{
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            input.push(ip.chars().collect());
        }
    }
    Grid::new(input)
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
