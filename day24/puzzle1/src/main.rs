use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Hailstone {
    coords: [f64; 3],
    velocity: [f64; 3],
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new(coords: [f64; 3], velocity: [f64; 3]) -> Self {
        Self {
            coords,
            velocity,
            a: velocity[1],
            b: -velocity[0],
            c: velocity[1] * coords[0] - velocity[0] * coords[1],
        }
    }
}

fn main() {
    let hailstones = parse_input("../input.txt");

    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let (hs1, hs2) = (&hailstones[i], &hailstones[j]);
            if hs1.a * hs2.b == hs1.b * hs2.a {
                continue;
            }
            let x = (hs1.c * hs2.b - hs2.c * hs1.b) / (hs1.a * hs2.b - hs2.a * hs1.b);
            let y = (hs1.a * hs2.c - hs2.a * hs1.c) / (hs1.a * hs2.b - hs2.a * hs1.b);
            if x >= 200000000000000.0
                && x <= 400000000000000.0
                && y >= 200000000000000.0
                && y <= 400000000000000.0
            {
                if (x - hs1.coords[0]) * hs1.velocity[0] >= 0.0
                    && (y - hs1.coords[1]) * hs1.velocity[1] >= 0.0
                    && (x - hs2.coords[0]) * hs2.velocity[0] >= 0.0
                    && (y - hs2.coords[1]) * hs2.velocity[1] >= 0.0
                {
                    count += 1;
                }
            }
        }
    }

    println!("Intersects: {:?}", count);
}

fn parse_input<P>(filename: P) -> Vec<Hailstone>
where
    P: AsRef<Path>,
{
    let mut hailstones = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let (coords_string, velocities_string) = ip.split_once(" @ ").unwrap();
            let coords = coords_string
                .split(", ")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let velocities = velocities_string
                .split(", ")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            hailstones.push(Hailstone::new(coords, velocities));
        }
    }
    hailstones
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
