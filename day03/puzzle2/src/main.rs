use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct NumbersIndices {
    in_row_above: Vec<(usize, usize)>,
    in_considered_row: Vec<(usize, usize)>,
    in_row_below: Vec<(usize, usize)>,
}

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut rows = Vec::<Vec<char>>::new();
        for line in lines {
            if let Ok(ip) = line {
                let chars = ip.chars().collect::<Vec<char>>();
                rows.push(chars)
            }
        }
        let mut numbers_indices = NumbersIndices {
            in_row_above: Vec::<(usize, usize)>::new(),
            in_considered_row: Vec::<(usize, usize)>::new(),
            in_row_below: Vec::<(usize, usize)>::new(),
        };
        let mut sum = 0;
        for i in 0..rows.len() {
            if rows.len() == 1 {
                numbers_indices.in_considered_row =
                    find_numbers_indices(&rows[i].iter().collect::<String>());
            } else if i == 0 {
                numbers_indices.in_considered_row =
                    find_numbers_indices(&rows[i].iter().collect::<String>());
                numbers_indices.in_row_below =
                    find_numbers_indices(&rows[i + 1].iter().collect::<String>());
            } else if i > 0 && i < rows.len() - 1 {
                numbers_indices.in_row_above = numbers_indices.in_considered_row;
                numbers_indices.in_considered_row = numbers_indices.in_row_below;
                numbers_indices.in_row_below =
                    find_numbers_indices(&rows[i + 1].iter().collect::<String>());
            } else if i == rows.len() - 1 {
                numbers_indices.in_row_above = numbers_indices.in_considered_row;
                numbers_indices.in_considered_row = numbers_indices.in_row_below;
                numbers_indices.in_row_below = Vec::<(usize, usize)>::new();
            }

            let mut asterisks_indices = Vec::<(usize, usize)>::new();

            for j in 0..rows[i].len() {
                if rows[i][j] == '*' {
                    asterisks_indices.push((i, j));
                }
            }

            if asterisks_indices.len() == 0 {
                continue;
            }

            for asterisk_indices in &asterisks_indices {
                let mut adjacent_numbers = HashSet::<(usize, (usize, usize))>::new();
                for j in asterisk_indices.1 - 1..asterisk_indices.1 + 2 {
                    if numbers_indices.in_row_above.len() > 0 {
                        if rows[i - 1][j].is_digit(10) {
                            numbers_indices.in_row_above.iter().for_each(|x| {
                                if (x.0..x.1).contains(&j) {
                                    adjacent_numbers.insert((i - 1, *x));
                                }
                            });
                        }
                    }
                    if numbers_indices.in_considered_row.len() > 0 {
                        if rows[i][j].is_digit(10) {
                            numbers_indices.in_considered_row.iter().for_each(|x| {
                                if (x.0..x.1).contains(&j) {
                                    adjacent_numbers.insert((i, *x));
                                }
                            });
                        }
                    }
                    if numbers_indices.in_row_below.len() > 0 {
                        if rows[i + 1][j].is_digit(10) {
                            numbers_indices.in_row_below.iter().for_each(|x| {
                                if (x.0..x.1).contains(&j) {
                                    adjacent_numbers.insert((i + 1, *x));
                                }
                            });
                        }
                    }
                }
                if adjacent_numbers.len() == 2 {
                    let numbers = adjacent_numbers
                        .into_iter()
                        .map(|x| {
                            return parse_char_array_to_int(&rows[x.0][x.1 .0..x.1 .1]);
                        })
                        .collect::<Vec<i32>>();
                    sum += numbers[0] * numbers[1];
                }
            }
        }
        println!("Sum: {}", sum);
    }
}

fn find_numbers_indices(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input).map(|m| (m.start(), m.end())).collect()
}

fn parse_char_array_to_int(input: &[char]) -> i32 {
    input.iter().collect::<String>().parse::<i32>().unwrap()
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
