use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("../input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let game_id_and_bucket_strings = ip.splitn(2, ":").collect::<Vec<&str>>();
                let bucket_string = game_id_and_bucket_strings.last().unwrap().trim();
                let game_strings = bucket_string.split(";").collect::<Vec<&str>>();
                let mut needed_red = 0;
                let mut needed_green = 0;
                let mut needed_blue = 0;
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
                        if *color_of_cubes == "red" && count_of_cubes > needed_red {
                            needed_red = count_of_cubes;
                        }
                        if *color_of_cubes == "green" && count_of_cubes > needed_green {
                            needed_green = count_of_cubes;
                        }
                        if *color_of_cubes == "blue" && count_of_cubes > needed_blue {
                            needed_blue = count_of_cubes;
                        }
                    }
                }
                sum += needed_red * needed_green * needed_blue;
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
