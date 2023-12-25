use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

#[derive(Debug)]
struct Hailstone {
    coords: [i64; 3],
    velocity: [i64; 3],
}

impl Hailstone {
    fn new(coords: [i64; 3], velocity: [i64; 3]) -> Self {
        Self { coords, velocity }
    }
}

fn main() {
    let hailstones = parse_input("../input.txt");

    println!("Sum: {}", solve(&hailstones));
}

fn solve(hailstones: &Vec<Hailstone>) -> String {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hailstones {
        let pxn = Int::from_i64(&ctx, hailstone.coords[0]);
        let pyn = Int::from_i64(&ctx, hailstone.coords[1]);
        let pzn = Int::from_i64(&ctx, hailstone.coords[2]);
        let vxn = Int::from_i64(&ctx, hailstone.velocity[0]);
        let vyn = Int::from_i64(&ctx, hailstone.velocity[1]);
        let vzn = Int::from_i64(&ctx, hailstone.velocity[2]);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    (x + y + z).to_string()
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
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let velocities = velocities_string
                .split(", ")
                .map(|x| x.trim().parse::<i64>().unwrap())
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
