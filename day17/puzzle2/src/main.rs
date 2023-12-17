use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<usize>>,
}

impl Grid {
    fn new(grid: Vec<Vec<usize>>) -> Self {
        Self { grid }
    }
}

fn main() {
    let grid_wrapper = parse_input("../input.txt");

    let mut heap = BinaryHeap::from([(0 as i32, (0, 0), (0, 0), 0)]);
    let mut map = HashMap::new();

    while let Some((heat_loss, (cur_row, cur_col), (dir_row, dir_col), dir_steps)) = heap.pop() {
        if cur_row == grid_wrapper.grid.len() as i32 - 1
            && cur_col == grid_wrapper.grid[0].len() as i32 - 1
            && dir_steps >= 4
        {
            println!("Heat loss: {}", -heat_loss);
            break;
        }

        if map.contains_key(&((cur_row, cur_col), (dir_row, dir_col), dir_steps)) {
            continue;
        }

        map.insert(
            ((cur_row, cur_col), (dir_row, dir_col), dir_steps),
            heat_loss,
        );

        if dir_steps < 10 && (dir_row, dir_col) != (0, 0) {
            let (next_row, next_col) = (cur_row + dir_row, cur_col + dir_col);
            if 0 <= next_row
                && next_row < grid_wrapper.grid.len() as i32
                && 0 <= next_col
                && next_col < grid_wrapper.grid[0].len() as i32
            {
                let next_heat_loss = grid_wrapper.grid[next_row as usize][next_col as usize] as i32;
                heap.push((
                    heat_loss - next_heat_loss,
                    (next_row, next_col),
                    (dir_row, dir_col),
                    dir_steps + 1,
                ));
            }
        }

        if dir_steps >= 4 || (dir_row, dir_col) == (0, 0) {
            for (next_dir_row, next_dir_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if (next_dir_row, next_dir_col) != (dir_row, dir_col)
                    && (next_dir_row, next_dir_col) != (-dir_row, -dir_col)
                {
                    let (next_row, next_col) = (cur_row + next_dir_row, cur_col + next_dir_col);
                    if 0 <= next_row
                        && next_row < grid_wrapper.grid.len() as i32
                        && 0 <= next_col
                        && next_col < grid_wrapper.grid[0].len() as i32
                    {
                        let next_heat_lose =
                            grid_wrapper.grid[next_row as usize][next_col as usize] as i32;
                        heap.push((
                            heat_loss - next_heat_lose,
                            (next_row, next_col),
                            (next_dir_row, next_dir_col),
                            1,
                        ));
                    }
                }
            }
        }
    }
}

fn parse_input<P>(filename: P) -> Grid
where
    P: AsRef<Path>,
{
    let mut input: Vec<Vec<usize>> = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            input.push(
                ip.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .collect(),
            );
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
