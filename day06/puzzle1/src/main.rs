use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let input = read_input("../input.txt");
    let races = parse_races(&input);
    let product: u32 = races.into_iter().map(calculate_wins_count).product();
    println!("Product: {}", product);
}

fn calculate_wins_count((time, distance): (u32, u32)) -> u32 {
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

fn parse_races(input: &str) -> Vec<(u32, u32)> {
    let lines: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| {
            let (_, values) = line.split_once(":").unwrap();
            values
                .trim()
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();
    (0..lines[0].len())
        .map(|i| (lines[0][i], lines[1][i]))
        .collect()
}
