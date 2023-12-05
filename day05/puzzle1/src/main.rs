use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut initial_numbers: Vec<i64> = Vec::new();
        let mut maps: LinkedList<Vec<(i64, i64, i64)>> = LinkedList::new();
        let map_names = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];

        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("seeds") {
                    initial_numbers = parse_initial_numbers_from_line(&ip);
                    continue;
                }

                if map_names.iter().any(|x| ip.starts_with(x)) {
                    maps.push_back(Vec::new());
                    continue;
                }

                if ip.is_empty() || !ip.chars().next().unwrap().is_numeric() {
                    continue;
                }

                let nums: Vec<i64> = ip.split_whitespace().map(|s| s.parse().unwrap()).collect();
                maps.back_mut().unwrap().push((nums[0], nums[1], nums[2]));
            }
        }

        maps.iter().for_each(|map| {
            initial_numbers = initial_numbers
                .iter()
                .map(|&initial_number| map_source_to_dest(initial_number, &map))
                .collect();
        });

        println!("Nearest location: {}", initial_numbers.into_iter().min().unwrap());
    }
}

fn parse_initial_numbers_from_line(line: &str) -> Vec<i64> {
    let (_, initial_number_strings) = line.split_once(":").unwrap();
    initial_number_strings
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn map_source_to_dest(initial_number: i64, map: &Vec<(i64, i64, i64)>) -> i64 {
    for &(dest_start, src_start, length) in map {
        if initial_number >= src_start && initial_number < src_start + length {
            return dest_start + (initial_number - src_start);
        }
    }
    initial_number
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
