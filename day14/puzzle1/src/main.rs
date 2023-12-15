use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
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

    for i in 0..grid_wrapper.grid[0].len() {
        let mut col = grid_wrapper.get_column(i).unwrap();
        let mut j = 0;
        while j < col.len() {
            if col[j] == 'O' {
                let mut k = j;
                while k > 0 && col[k - 1] != '#' {
                    col.swap(k, k - 1);
                    k -= 1;
                }
            }
            j += 1;
        }
        grid_wrapper.set_column(i, col.clone());
    }

    let sum: usize = (0..grid_wrapper.grid.len())
        .map(|i| {
            let row = grid_wrapper.get_row(i).unwrap();
            row.iter().filter(|c| **c == 'O').collect::<Vec<_>>().len()
                * (grid_wrapper.grid.len() - i)
        })
        .sum();

    println!("Sum: {}", sum);
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
