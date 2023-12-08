use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (steps_pattern, coords) = parse_input("../input.txt");
    let ending_point = "ZZZ";

    let mut current_point = "AAA";
    let mut steps_count = 0;
    while current_point != ending_point {
        let (x, y) = coords.get(current_point).unwrap();
        let direction = steps_pattern
            .chars()
            .nth(steps_count % steps_pattern.len())
            .unwrap();
        if direction == 'L' {
            current_point = x;
        }
        if direction == 'R' {
            current_point = y;
        }
        steps_count += 1;
    }

    println!("Steps count: {}", steps_count);
}

fn parse_input<P>(filename: P) -> (String, HashMap<String, (String, String)>)
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename);
    let steps_pattern = lines.next().unwrap().unwrap();
    lines.next();

    let mut coords: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        if let Ok(ip) = line {
            let (key, value) = ip.split_once(" = ").unwrap();
            let value_with_stripped_brackets = value.replace(&['(', ')'][..], "");
            let (x, y) = value_with_stripped_brackets.split_once(", ").unwrap();
            let (x, y) = (x.to_string(), y.to_string());
            coords.insert(key.to_string(), (x, y));
        }
    }
    (steps_pattern, coords)
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
