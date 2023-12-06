use std::cmp::{max, min};
use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut initial_ranges: Vec<(i64, i64)> = Vec::new();
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
                    let initial_numbers = parse_initial_numbers_from_line(&ip);
                    initial_ranges = initial_numbers
                        .chunks(2)
                        .map(|x| (x[0], x[0] + x[1]))
                        .collect();
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

        maps.into_iter().for_each(|map| {
            initial_ranges = map_src_to_dest_ranges(initial_ranges.clone(), &map);
        });

        println!(
            "Nearest location: {}",
            initial_ranges.into_iter().min().unwrap().0
        );
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

fn map_src_to_dest_ranges(
    mut initial_ranges: Vec<(i64, i64)>,
    map: &Vec<(i64, i64, i64)>,
) -> Vec<(i64, i64)> {
    let mut new_ranges: Vec<(i64, i64)> = Vec::new();
    while initial_ranges.len() > 0 {
        let (range_start, range_end) = initial_ranges.pop().unwrap();
        let mut matched = false;
        for &(dest_start, src_start, length) in map {
            let os = max(range_start, src_start);
            let oe = min(range_end, src_start + length);
            if os < oe {
                new_ranges.push((os - src_start + dest_start, oe - src_start + dest_start));
                if os > range_start {
                    initial_ranges.push((range_start, os));
                }
                if range_end > oe {
                    initial_ranges.push((oe, range_end));
                }
                matched = true;
                break;
            }
        }
        if !matched {
            new_ranges.push((range_start, range_end));
        }
    }
    new_ranges
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
