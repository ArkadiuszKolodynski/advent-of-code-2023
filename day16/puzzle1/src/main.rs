use std::collections::HashSet;
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
}

#[derive(Clone)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

#[derive(Clone)]
struct Beam {
    live: bool,
    row_length: usize,
    col_length: usize,
    row_i: usize,
    col_j: usize,
    direction: Direction,
}

impl Beam {
    fn move_forward(&mut self) {
        match self.direction {
            #[rustfmt::skip]
            Direction::Up => if self.row_i > 0 { self.row_i -= 1 } else { self.live = false },
            #[rustfmt::skip]
            Direction::Down => if self.row_i < self.row_length - 1 { self.row_i += 1 } else { self.live = false },
            #[rustfmt::skip]
            Direction::Left => if self.col_j > 0  { self.col_j -= 1 } else { self.live = false },
            #[rustfmt::skip]
            Direction::Right => if self.col_j < self.col_length - 1 { self.col_j += 1 } else { self.live = false },
        };
    }

    fn split(&mut self) -> Vec<Beam> {
        self.live = false;
        match self.direction {
            Direction::Up | Direction::Down => {
                let mut beams = Vec::new();
                if self.col_j > 0 {
                    beams.push(Beam {
                        live: true,
                        row_length: self.row_length,
                        col_length: self.col_length,
                        row_i: self.row_i,
                        col_j: self.col_j - 1,
                        direction: Direction::Left,
                    });
                }
                if self.col_j < self.col_length - 1 {
                    beams.push(Beam {
                        live: true,
                        row_length: self.row_length,
                        col_length: self.col_length,
                        row_i: self.row_i,
                        col_j: self.col_j + 1,
                        direction: Direction::Right,
                    });
                }
                beams
            }
            Direction::Left | Direction::Right => {
                let mut beams = Vec::new();
                if self.row_i > 0 {
                    beams.push(Beam {
                        live: true,
                        row_length: self.row_length,
                        col_length: self.col_length,
                        row_i: self.row_i - 1,
                        col_j: self.col_j,
                        direction: Direction::Up,
                    });
                }
                if self.row_i < self.row_length - 1 {
                    beams.push(Beam {
                        live: true,
                        row_length: self.row_length,
                        col_length: self.col_length,
                        row_i: self.row_i + 1,
                        col_j: self.col_j,
                        direction: Direction::Down,
                    });
                }
                beams
            }
        }
    }
}

fn main() {
    let grid_wrapper = parse_input("../input.txt");
    let mut beams = vec![Beam {
        live: true,
        row_length: grid_wrapper.grid.len(),
        col_length: grid_wrapper.grid[0].len(),
        row_i: 0,
        col_j: 0,
        direction: Direction::Right,
    }];
    let mut visited: HashSet<(usize, usize)> = HashSet::from([(0, 0)]);

    while beams.len() > 0 {
        for i in (0..beams.len()).rev() {
            if !beams[i].live {
                beams.remove(i);
            }
        }

        for i in (0..beams.len()).rev() {
            let mem_beam = beams[i].clone();
            let beam = &mut beams[i];

            match grid_wrapper.grid[beam.row_i][beam.col_j] {
                '.' => beam.move_forward(),
                '\\' => {
                    beam.direction = match beam.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    beam.move_forward()
                }
                '/' => {
                    beam.direction = match beam.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    beam.move_forward()
                }
                '|' => {
                    if matches!(beam.direction, Direction::Left)
                        || matches!(beam.direction, Direction::Right)
                    {
                        if visited.contains(&(beam.row_i, beam.col_j)) {
                            beam.live = false;
                        } else {
                            let mut new_beams = beam.split();
                            beams.append(new_beams.as_mut());
                        }
                    } else {
                        beam.move_forward()
                    }
                }
                '-' => {
                    if matches!(beam.direction, Direction::Up)
                        || matches!(beam.direction, Direction::Down)
                    {
                        if visited.contains(&(beam.row_i, beam.col_j)) {
                            beam.live = false;
                        } else {
                            let mut new_beams = beam.split();
                            beams.append(new_beams.as_mut());
                        }
                    } else {
                        beam.move_forward()
                    }
                }
                _ => (),
            };
            visited.insert((mem_beam.row_i, mem_beam.col_j));
        }
    }

    println!("Energized tiles: {}", visited.len());
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
