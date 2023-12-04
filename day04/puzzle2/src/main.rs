use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut i = 0;
        let mut cards_counts: Vec<i32> = Vec::new();
        for _ in 0..256 {
            cards_counts.push(0);
        }
        for line in lines {
            if let Ok(ip) = line {
                cards_counts[i] += 1;

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

                let matches = winning_numbers.into_iter().fold(0, |acc, winning_number| {
                    if card_numbers.contains(&winning_number) {
                        return acc + 1;
                    }
                    acc
                });

                i += 1;
                for j in 0..matches {
                    cards_counts[i + j] += cards_counts[i - 1];
                }
            }
        }
        let sum = &cards_counts[0..i].into_iter().sum::<i32>();
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
