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

    fn transpose(&self) -> Self {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for i in 0..self.grid[0].len() {
            grid.push(self.get_column(i).unwrap());
        }
        Self::new(grid)
    }
}

fn main() {
    let grid_wrappers = parse_input("../input.txt");

    let sum = grid_wrappers.into_iter().fold(0, |acc, grid_wrapper| {
        acc + find_reflection_line(&grid_wrapper)
            .unwrap_or_else(|| find_reflection_line(&grid_wrapper.transpose()).unwrap_or(0) * 100)
    });

    println!("Sum: {}", sum);
}

fn find_reflection_line(grid_wrapper: &Grid) -> Option<usize> {
    let len = grid_wrapper.get_row(0)?.len();
    let middle_index = len / 2;

    for i in (0..middle_index).rev() {
        let reflection_line = find_reflection(grid_wrapper, len, i, &true);
        if reflection_line.is_some() {
            return reflection_line;
        }
    }

    for i in middle_index..len - 1 {
        let reflection_line = find_reflection(grid_wrapper, len, i, &false);
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
    is_first_half: &bool,
) -> Option<usize> {
    let mut diffs = 0;
    let (mut j, mut k) = (i as i32, i + 1);
    let reflection_line = 'found_ref: {
        while if is_first_half == &true {
            j >= 0
        } else {
            k < len
        } {
            if grid_wrapper.get_column(j as usize)? != grid_wrapper.get_column(k)? {
                diffs += count_diffrences_between_two_char_vectors(
                    &grid_wrapper.get_column(j as usize)?,
                    &grid_wrapper.get_column(k)?,
                );
                if diffs > 1 {
                    break 'found_ref None;
                }
            }
            j -= 1;
            k += 1;
        }
        Some(i + 1)
    };

    if diffs == 1 { reflection_line } else { None }
}

fn count_diffrences_between_two_char_vectors(v1: &Vec<char>, v2: &Vec<char>) -> usize {
    let mut diffs = 0;
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            diffs += 1;
        }
    }
    diffs
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
