use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                let numbers_strings = ip
                    .splitn(2, ":")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .splitn(2, "|")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();
                let winning_numbers = numbers_strings
                    .first()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                let card_numbers = numbers_strings
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                let mut matches = 0;
                for winning_number in winning_numbers {
                    if card_numbers.contains(&winning_number) {
                        matches += 1;
                    }
                }
                let card_points = if matches == 0 { 0 } else { (2 as i32).pow(matches - 1) };
                sum += card_points;
            }
        }

        println!("Sum: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e),
    };
    Ok(io::BufReader::new(file).lines())
}
