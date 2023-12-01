use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./assets/input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                const RADIX: u32 = 10;
                let first_digit = ip.chars().find(|c| c.is_digit(RADIX)).unwrap();
                let second_digit = ip.chars().rfind(|c| c.is_digit(RADIX)).unwrap();
                let combined = format!("{}{}", first_digit, second_digit);
                sum += combined.parse::<i32>().unwrap();
            }
        }
        println!("Sum: {}", sum)
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
