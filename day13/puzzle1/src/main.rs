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
    let grid_wrappers = parse_input("../input.txt");

    let sum = grid_wrappers.into_iter().fold(0, |acc, grid_wrapper| {
        acc + find_reflection_line(&grid_wrapper, &true)
            .unwrap_or_else(|| find_reflection_line(&grid_wrapper, &false).unwrap_or(0) * 100)
    });

    println!("Sum: {}", sum);
}

fn find_reflection_line(grid_wrapper: &Grid, is_vertical: &bool) -> Option<usize> {
    let len = if is_vertical == &true {
        grid_wrapper.get_row(0)?.len()
    } else {
        grid_wrapper.get_column(0)?.len()
    };
    let middle_index = len / 2;

    for i in (0..middle_index).rev() {
        let reflection_line = find_reflection(grid_wrapper, len, i, is_vertical, &true);
        if reflection_line.is_some() {
            return reflection_line;
        }
    }

    for i in middle_index..len - 1 {
        let reflection_line = find_reflection(grid_wrapper, len, i, is_vertical, &false);
        if reflection_line.is_some() {
            return reflection_line;
        }
    }

    None
}

fn find_reflection(
    grid_wrapper: &Grid,
    len: usize,
    i: usize,
    is_vertical: &bool,
    is_first_half: &bool,
) -> Option<usize> {
    let (mut j, mut k) = (i as i32, i + 1);
    let reflection_line = 'found_ref: {
        while if is_first_half == &true {
            j >= 0
        } else {
            k < len
        } {
            if is_vertical == &true {
                if grid_wrapper.get_column(j as usize)? != grid_wrapper.get_column(k)? {
                    break 'found_ref None;
                }
            } else {
                if grid_wrapper.get_row(j as usize)? != grid_wrapper.get_row(k)? {
                    break 'found_ref None;
                }
            }
            j -= 1;
            k += 1;
        }
        Some(i + 1)
    };

    reflection_line
}

fn parse_input<P>(filename: P) -> Vec<Grid>
where
    P: AsRef<Path>,
{
    let mut grids: Vec<Grid> = Vec::new();
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            if ip.is_empty() {
                grids.push(Grid::new(input));
                input = Vec::new();
                continue;
            }
            input.push(ip.chars().collect());
        }
    }
    grids.push(Grid::new(input));
    grids
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
