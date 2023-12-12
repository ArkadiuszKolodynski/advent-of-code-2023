use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = parse_input("../input.txt");
    let possible_strings: Vec<Vec<String>> = input.into_iter().map(find_possible_strings).collect();

    println!(
        "Sum: {:?}",
        possible_strings.into_iter().map(|a| a.len()).sum::<usize>()
    );
}

fn replace_question_marks(
    pattern: &str,
    possible_string: String,
    lengths: &Vec<usize>,
    possible_strings: &mut Vec<String>,
) {
    if let Some(index) = pattern.find('?') {
        replace_question_marks(
            &pattern[(index + 1)..],
            format!("{}{}{}", &possible_string, &pattern[..index], '#'),
            lengths,
            possible_strings,
        );
        replace_question_marks(
            &pattern[(index + 1)..],
            format!("{}{}{}", &possible_string, &pattern[..index], '.'),
            lengths,
            possible_strings,
        );
    } else {
        let final_string = format!("{}{}", possible_string, pattern);
        let hash_lengths: Vec<usize> = final_string
            .split('.')
            .filter(|&s| !s.is_empty())
            .map(|s| s.len())
            .collect();
        if hash_lengths == *lengths && !possible_strings.contains(&final_string) {
            possible_strings.push(final_string);
        }
    }
}

fn find_possible_strings((pattern, lengths): (String, Vec<usize>)) -> Vec<String> {
    let mut possible_strings = Vec::new();
    replace_question_marks(&pattern, "".to_string(), &lengths, &mut possible_strings);
    possible_strings
}

fn parse_input<P>(filename: P) -> Vec<(String, Vec<usize>)>
where
    P: AsRef<Path>,
{
    let mut input: Vec<(String, Vec<usize>)> = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let (pattern, lengths_str) = ip.split_once(' ').unwrap();
            let lengths: Vec<usize> = lengths_str.split(',').map(|s| s.parse().unwrap()).collect();
            input.push((pattern.to_string(), lengths));
        }
    }
    input
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
