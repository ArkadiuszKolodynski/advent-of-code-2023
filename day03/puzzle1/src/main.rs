use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut rows = Vec::<Vec<char>>::new();
        for line in lines {
            if let Ok(ip) = line {
                let chars = ip.chars().collect::<Vec<char>>();
                rows.push(chars)
            }
        }
        let mut sum = 0;
        for i in 0..rows.len() {
            let inidices = find_numbers_indices(&rows[i].iter().collect::<String>());
            'outer: for number_indices in inidices {
                let index_before = if number_indices.0 <= 0 {
                    0
                } else {
                    number_indices.0 - 1
                };
                let index_after = if number_indices.1 >= rows[i].len() {
                    rows[i].len()
                } else {
                    number_indices.1 + 1
                };

                // check character before
                if number_indices.0 > 0 {
                    if rows[i][number_indices.0 - 1] != '.' {
                        sum +=
                            parse_char_array_to_int(&rows[i][number_indices.0..number_indices.1]);
                        continue 'outer;
                    }
                }

                // check character after
                if number_indices.1 < rows[i].len() {
                    if rows[i][number_indices.1] != '.' {
                        sum +=
                            parse_char_array_to_int(&rows[i][number_indices.0..number_indices.1]);
                        continue 'outer;
                    }
                }

                for j in index_before..index_after {
                    // check characters above
                    if i > 0 {
                        if rows[i - 1][j] != '.' {
                            sum += parse_char_array_to_int(
                                &rows[i][number_indices.0..number_indices.1],
                            );
                            continue 'outer;
                        }
                    }

                    // check characters below
                    if i < rows.len() - 1 {
                        if rows[i + 1][j] != '.' {
                            sum += parse_char_array_to_int(
                                &rows[i][number_indices.0..number_indices.1],
                            );
                            continue 'outer;
                        }
                    }
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
