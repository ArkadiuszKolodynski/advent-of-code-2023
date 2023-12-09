use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: isize = 0;
    for line in read_lines("../input.txt") {
        if let Ok(ip) = line {
            let mut sequences: Vec<Vec<isize>> = Vec::new();
            let mut initial_sequence: Vec<isize> =
                ip.split_whitespace().map(|x| x.parse().unwrap()).collect();
            while initial_sequence.iter().any(|x| x != &0) {
                sequences.insert(0, initial_sequence.clone());
                initial_sequence = initial_sequence.windows(2).map(|x| x[1] - x[0]).collect();
            }
            sum += sequences
                .into_iter()
                .fold(0, |acc, x| x.first().unwrap_or(&0) - acc);
        }
    }
    println!("Sum: {}", sum)
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
