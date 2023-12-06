use std::fs::read_to_string;

fn main() {
    let input = read_input("../input.txt");
    let race = parse_race(&input);
    println!("Wins count: {}", calculate_wins_count(race));
}

fn calculate_wins_count((time, distance): (u64, u64)) -> u64 {
    (1..time).fold(0, |wins, i| {
        if (time - i) * i > distance {
            wins + 1
        } else {
            wins
        }
    })
}

fn read_input(path: &str) -> String {
    let input = read_to_string(path);
    match input {
        Ok(input) => input,
        Err(e) => panic!("Error: {}", e),
    }
}

fn parse_race(input: &str) -> (u64, u64) {
    let lines: Vec<String> = input
        .split("\n")
        .map(|line| {
            let (_, values) = line.split_once(":").unwrap();
            values
                .trim()
                .split_whitespace()
                .fold(String::new(), |acc, value| (acc + value))
        })
        .collect();
    (lines[0].parse().unwrap(), lines[1].parse().unwrap())
}
