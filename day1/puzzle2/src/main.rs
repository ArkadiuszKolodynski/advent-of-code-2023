use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use double_map::DHashMap;

fn main() {
    let digits_map = DHashMap::from([
        ("1", "one", 1),
        ("2", "two", 2),
        ("3", "three", 3),
        ("4", "four", 4),
        ("5", "five", 5),
        ("6", "six", 6),
        ("7", "seven", 7),
        ("8", "eight", 8),
        ("9", "nine", 9),
    ]);

    let digit_strings = digits_map
        .keys()
        .collect::<Vec<_>>()
        .into_iter()
        .flat_map(|(a, b)| vec![*a, *b])
        .collect::<Vec<_>>();

    if let Ok(lines) = read_lines("../input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let mut first = (9999, "nineninenine");
                let mut last = (0, "zero");
                for digit in &digit_strings {
                    let matches = ip.match_indices(digit).collect::<Vec<_>>();
                    if matches.len() > 0 {
                        let first_index = matches[0].0;
                        let last_index = matches[matches.len() - 1].0;
                        if first_index <= first.0 {
                            first = (first_index, digit);
                        }
                        if last_index >= last.0 {
                            last = (last_index, digit);
                        }
                    }
                }

                let first_digit = digits_map
                    .get_key1(first.1)
                    .or(digits_map.get_key2(first.1))
                    .unwrap();
                let last_digit = digits_map
                    .get_key1(last.1)
                    .or(digits_map.get_key2(last.1))
                    .unwrap();
                sum += first_digit * 10 + last_digit;
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
