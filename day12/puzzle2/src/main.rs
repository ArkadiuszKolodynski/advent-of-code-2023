use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    let mut cache: HashMap<(String, Vec<usize>), u128> = HashMap::new();
    for line in read_lines("../input.txt") {
        if let Ok(ip) = line {
            let (pattern, lengths_str) = ip.split_once(' ').unwrap();
            let lengths: Vec<usize> = lengths_str.split(',').map(|s| s.parse().unwrap()).collect();
            let (extended_pattern, extended_lengths) =
                (vec![pattern.to_string(); 5].join("?"), lengths.repeat(5));
            sum += count_possible_strings(
                extended_pattern.clone(),
                extended_lengths.clone(),
                &mut cache,
            );
        }
    }
    println!("Sum: {}", sum);
}

fn count_possible_strings(
    pattern: String,
    lengths: Vec<usize>,
    cache: &mut HashMap<(String, Vec<usize>), u128>,
) -> u128 {
    if pattern == "" {
        return if lengths.is_empty() { 1 } else { 0 };
    }

    if lengths.is_empty() {
        return if pattern.contains('#') { 0 } else { 1 };
    }

    let key = (pattern.clone(), lengths.clone());
    if let Some(&count) = cache.get(&key) {
        return count;
    }

    let mut count = 0;
    if pattern.starts_with(".") || pattern.starts_with("?") {
        count += count_possible_strings(pattern[1..].to_string(), lengths.to_vec(), cache);
    }

    if pattern.starts_with("#") || pattern.starts_with("?") {
        if lengths[0] <= pattern.len()
            && !pattern[..lengths[0]].contains('.')
            && (lengths[0] == pattern.len() || pattern.chars().nth(lengths[0]).unwrap() != '#')
        {
            count += count_possible_strings(
                pattern
                    .get(lengths[0] + 1..)
                    .unwrap_or_else(|| "")
                    .to_string(),
                lengths[1..].to_vec(),
                cache,
            );
        }
    }

    cache.insert(key, count);
    count
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
