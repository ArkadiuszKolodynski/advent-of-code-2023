use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let cubes_of_color_max_contraint = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    if let Ok(lines) = read_lines("../input.txt") {
        let mut sum = 0;
        'outer: for line in lines {
            if let Ok(ip) = line {
                let game_id_and_bucket_strings = ip.splitn(2, ":").collect::<Vec<&str>>();
                let bucket_string = game_id_and_bucket_strings.last().unwrap().trim();
                let game_strings = bucket_string.split(";").collect::<Vec<&str>>();
                for game_string in game_strings {
                    let cube_of_color_count_strings = game_string.split(",").collect::<Vec<&str>>();
                    for cube_of_color_count_string in cube_of_color_count_strings {
                        let count_and_color = cube_of_color_count_string
                            .trim()
                            .splitn(2, " ")
                            .collect::<Vec<&str>>();
                        let count_of_cubes =
                            count_and_color.first().unwrap().parse::<i32>().unwrap();
                        let color_of_cubes = count_and_color.last().unwrap();
                        let max_cubes = cubes_of_color_max_contraint.get(color_of_cubes).unwrap();
                        if count_of_cubes > *max_cubes {
                            continue 'outer;
                        }
                    }
                }
                let game_id = get_game_id(game_id_and_bucket_strings.first().unwrap());
                sum += game_id;
            }
        }
        println!("Sum: {}", sum)
    }
}

fn get_game_id(game_id_string: &str) -> i32 {
    game_id_string.split(" ").collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap()
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
