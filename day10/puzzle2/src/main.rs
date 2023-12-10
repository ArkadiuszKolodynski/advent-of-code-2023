use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let pipes_map = parse_input("../input.txt");
    let mut pipe_loop: Vec<(usize, usize)> = Vec::new();
    let possible_moves = get_possible_moves();
    let starting_point = get_starting_point(&pipes_map);
    pipe_loop.push(starting_point);
    let mut current_move = possible_moves
        .values()
        .find_map(|moves| get_next_move(moves, starting_point, None, &pipes_map))
        .unwrap();
    while pipes_map[current_move.0][current_move.1] != 'S' {
        pipe_loop.push(current_move);
        let current_char = pipes_map[current_move.0][current_move.1];
        let last_char_possible_moves = possible_moves.get(&current_char).unwrap();
        let next_move = get_next_move(
            last_char_possible_moves,
            (current_move.0, current_move.1),
            Some(pipe_loop[pipe_loop.len() - 2]),
            &pipes_map,
        )
        .unwrap();
        current_move = next_move;
    }

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let polygon_area = shoelace_formula(&pipe_loop);
    let polygon_boundary_points_count = pipe_loop.len();
    let interior_points_count = polygon_area - (polygon_boundary_points_count / 2) + 1;

    println!("Interior points count: {}", interior_points_count);
}

fn shoelace_formula(vertices: &Vec<(usize, usize)>) -> usize {
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let n = vertices.len();

    for i in 0..n {
        let j = (i + 1) % n;
        sum1 += vertices[i].0 as f64 * vertices[j].1 as f64;
        sum2 += vertices[j].0 as f64 * vertices[i].1 as f64;
    }

    ((sum1 - sum2) / 2.0).abs() as usize
}

fn get_next_move(
    moves: &HashMap<(i32, i32), Vec<char>>,
    starting_point: (usize, usize),
    previous_point: Option<(usize, usize)>,
    pipes_map: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    moves.iter().find_map(|((x, y), chars)| {
        let new_x = starting_point.0 as i32 + x;
        let new_y = starting_point.1 as i32 + y;
        if (new_x >= 0 && new_y >= 0) && previous_point != Some((new_x as usize, new_y as usize)) {
            if chars.contains(&pipes_map[new_x as usize][new_y as usize]) {
                return Some((new_x as usize, new_y as usize));
            }
        }
        None
    })
}

fn get_possible_moves() -> HashMap<char, HashMap<(i32, i32), Vec<char>>> {
    HashMap::from([
        (
            '|',
            HashMap::from([
                ((-1, 0), vec!['|', 'L', 'J', '7', 'F', 'S']),
                ((1, 0), vec!['|', 'L', 'J', '7', 'F', 'S']),
            ]),
        ),
        (
            '-',
            HashMap::from([
                ((0, -1), vec!['-', 'L', 'F', 'S']),
                ((0, 1), vec!['-', 'J', '7', 'S']),
            ]),
        ),
        (
            'L',
            HashMap::from([
                ((-1, 0), vec!['|', '7', 'F', 'S']),
                ((0, 1), vec!['-', '7', 'J', 'S']),
            ]),
        ),
        (
            'J',
            HashMap::from([
                ((-1, 0), vec!['|', '7', 'F', 'S']),
                ((0, -1), vec!['-', 'L', 'F', 'S']),
            ]),
        ),
        (
            'F',
            HashMap::from([
                ((0, 1), vec!['-', 'J', '7', 'S']),
                ((1, 0), vec!['|', 'L', 'J', 'S']),
            ]),
        ),
        (
            '7',
            HashMap::from([
                ((0, -1), vec!['-', 'L', 'F', 'S']),
                ((1, 0), vec!['|', 'L', 'J', 'S']),
            ]),
        ),
    ])
}

fn get_starting_point(pipes_map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..pipes_map.len() {
        for j in 0..pipes_map[0].len() {
            if pipes_map[i][j] == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No starting point found");
}

fn parse_input<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            input.push(ip.chars().collect());
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
