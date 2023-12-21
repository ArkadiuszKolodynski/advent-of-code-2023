use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Grid = Vec<Vec<char>>;

trait GridExt {
    fn get_point(&self, x: usize, y: usize) -> Option<char>;
    fn next_tick(&self, points: HashSet<(usize, usize)>) -> HashSet<(usize, usize)>;
}

impl GridExt for Grid {
    fn get_point(&self, x: usize, y: usize) -> Option<char> {
        if let Some(row) = self.get(x) {
            if let Some(c) = row.get(y) {
                return Some(*c);
            }
        }
        None
    }

    fn next_tick(&self, points: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
        let mut next_points: HashSet<(usize, usize)> = HashSet::new();
        for point in points.iter() {
            let (x, y) = *point;
            if let Some(c) = self.get_point(x - 1, y) {
                if c == '.' {
                    next_points.insert((x - 1, y));
                }
            }
            if let Some(c) = self.get_point(x + 1, y) {
                if c == '.' {
                    next_points.insert((x + 1, y));
                }
            }
            if let Some(c) = self.get_point(x, y - 1) {
                if c == '.' {
                    next_points.insert((x, y - 1));
                }
            }
            if let Some(c) = self.get_point(x, y + 1) {
                if c == '.' {
                    next_points.insert((x, y + 1));
                }
            }
        }
        next_points
    }
}

fn main() {
    let (starting_point, grid) = parse_input("../input.txt");

    let ticks = 64;
    let mut points = HashSet::from([starting_point]);
    for _ in 0..ticks {
        points = grid.next_tick(points);
    }

    println!("Points: {:?}", points.len());
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
