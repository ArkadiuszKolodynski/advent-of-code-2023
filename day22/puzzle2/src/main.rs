use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Brick {
    left: [usize; 3],
    right: [usize; 3],
}

impl Brick {
    fn new(left: [usize; 3], right: [usize; 3]) -> Self {
        Self { left, right }
    }

    fn set_z(&mut self, left_z: usize, right_z: usize) {
        self.left[2] = left_z;
        self.right[2] = right_z;
    }

    fn overlaps_below(&self, other: &Brick) -> bool {
        max(self.left[0], other.left[0]) <= min(self.right[0], other.right[0])
            && max(self.left[1], other.left[1]) <= min(self.right[1], other.right[1])
    }
}

fn main() {
    let mut bricks = parse_input("../input.txt");

    settle_bricks(&mut bricks);
    let (supports, supported) = supporters(&bricks);

    let sum = (0..bricks.len()).fold(0, |mut acc, i| {
        let queue_init: Vec<_> = supports
            .get(&i)
            .unwrap()
            .iter()
            .filter(|&j| supported.get(j).unwrap().len() == 1)
            .map(|j| *j)
            .collect();
        let mut falling = HashSet::new();
        falling.insert(i);
        queue_init.iter().for_each(|&j| {
            falling.insert(j);
        });
        let mut queue = VecDeque::from(queue_init);

        while let Some(j) = queue.pop_front() {
            for k in supports.get(&j).unwrap() - &falling {
                if falling.is_superset(supported.get(&k).unwrap()) {
                    queue.push_back(k);
                    falling.insert(k);
                }
            }
        }

        acc += falling.len() - 1;
        acc
    });

    println!("Sum: {:?}", sum);
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        let mut max_z = 1;
        for j in 0..i {
            if bricks[i].overlaps_below(&bricks[j]) {
                max_z = max(max_z, bricks[j].right[2] + 1);
            }
        }
        let brick = &mut bricks[i];
        brick.set_z(max_z, brick.right[2] - (brick.left[2] - max_z));
    }
    bricks.sort_by(|a, b| a.left[2].cmp(&b.left[2]));
}

fn supporters(
    bricks: &Vec<Brick>,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
) {
    let mut above = HashMap::new();
    let mut below = HashMap::new();
    for (i, upper) in bricks.iter().enumerate() {
        above.entry(i).or_insert_with(HashSet::new);
        below.entry(i).or_insert_with(HashSet::new);
        for (j, lower) in bricks[..i].iter().enumerate() {
            if lower.overlaps_below(upper) && upper.left[2] == lower.right[2] + 1 {
                above.get_mut(&j).unwrap().insert(i);
                below.get_mut(&i).unwrap().insert(j);
            }
        }
    }
    (above, below)
}

fn parse_input<P>(filename: P) -> Vec<Brick>
where
    P: AsRef<Path>,
{
    let mut bricks = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let (left, right) = ip.split_once("~").unwrap();
            let coords: Vec<_> = [left, right]
                .into_iter()
                .map(|coords_str| {
                    coords_str
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect();
            bricks.push(Brick::new(coords[0], coords[1]));
        }
    }
    bricks.sort_by(|a, b| a.left[2].cmp(&b.left[2]));
    bricks
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
