use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Grid = Vec<Vec<char>>;

fn main() {
    let (starting_point, grid) = parse_input("../input.txt");

    let ticks = 26501365;
    let grid_size = grid.len();
    let grids = ticks / grid_size;
    let rem = ticks % grid_size;

    let sequence: Vec<_> = (0..3)
        .map(|n| bfs(&grid, starting_point, n * grid_size + rem))
        .collect();

    let a = (sequence[2] - sequence[0] - 2 * (sequence[1] - sequence[0])) / 2;
    let b = sequence[1] - sequence[0] - a;
    let c = sequence[0];

    let points = a * (grids * grids) + b * grids + c;
    println!("Points: {}", points);
}

fn bfs(grid: &Grid, start: (usize, usize), ticks: usize) -> usize {
    let mut visited = vec![HashSet::new(); ticks + 1];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some(((x, y), t)) = queue.pop_front() {
        if t > ticks {
            continue;
        }
        let normalized_pos = (x, y);
        if visited[t].insert(normalized_pos) {
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
                let new_t = t + 1;
                let new_pos = (new_x as usize, new_y as usize);
                if grid[new_x.rem_euclid(grid.len() as isize) as usize]
                    [new_y.rem_euclid(grid[0].len() as isize) as usize]
                    != '#'
                {
                    queue.push_back((new_pos, new_t));
                }
            }
        }
    }

    visited[ticks].len()
}

fn parse_input<P>(filename: P) -> ((usize, usize), Grid)
where
    P: AsRef<Path>,
{
    let mut starting_point = (0, 0);
    let mut grid = Grid::new();
    for line in read_lines(filename) {
        if let Ok(mut ip) = line {
            if ip.contains("S") {
                let i = ip.find('S').unwrap();
                ip = ip.replace("S", ".");
                starting_point = (grid.len(), i);
            }
            grid.push(ip.chars().collect());
        }
    }
    (starting_point, grid)
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
