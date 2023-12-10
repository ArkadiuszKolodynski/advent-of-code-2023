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
    let first_move = possible_moves
        .values()
        .find_map(|moves| get_next_move(moves, starting_point, None, &pipes_map))
        .unwrap();
    pipe_loop.push(first_move);
    let (mut current_char_x, mut current_char_y) = pipe_loop.last().unwrap();
    while pipes_map[current_char_x][current_char_y] != 'S' {
        let current_char = pipes_map[current_char_x][current_char_y];
        let last_char_possible_moves = possible_moves.get(&current_char).unwrap();
        let next_move = get_next_move(
            last_char_possible_moves,
            (current_char_x, current_char_y),
            Some(pipe_loop[pipe_loop.len() - 2]),
            &pipes_map,
        )
        .unwrap();
        pipe_loop.push(next_move);
        current_char_x = next_move.0;
        current_char_y = next_move.1;
    }

    println!("Steps count: {:?}", pipe_loop.len() / 2);
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
                // ((0, -1), vec![]),
                // ((0, 1), vec![]),
                ((1, 0), vec!['|', 'L', 'J', '7', 'F', 'S']),
            ]),
        ),
        (
            '-',
            HashMap::from([
                // ((-1, 0), vec![]),
                ((0, -1), vec!['-', 'L', 'F', 'S']),
                ((0, 1), vec!['-', 'J', '7', 'S']),
                // ((1, 0), vec![]),
            ]),
        ),
        (
            'L',
            HashMap::from([
                ((-1, 0), vec!['|', '7', 'F', 'S']),
                // ((0, -1), vec![]),
                ((0, 1), vec!['-', '7', 'J', 'S']),
                // ((1, 0), vec![]),
            ]),
        ),
        (
            'J',
            HashMap::from([
                ((-1, 0), vec!['|', '7', 'F', 'S']),
                ((0, -1), vec!['-', 'L', 'F', 'S']),
                // ((0, 1), vec![]),
                // ((1, 0), vec![]),
            ]),
        ),
        (
            'F',
            HashMap::from([
                // ((-1, 0), vec![]),
                // ((0, -1), vec![]),
                ((0, 1), vec!['-', 'J', '7', 'S']),
                ((1, 0), vec!['|', 'L', 'J', 'S']),
            ]),
        ),
        (
            '7',
            HashMap::from([
                // ((-1, 0), vec![]),
                ((0, -1), vec!['-', 'L', 'F', 'S']),
                // ((0, 1), vec![]),
                ((1, 0), vec!['|', 'L', 'J', 'S']),
            ]),
        ),
        // (
        //     '.',
        //     HashMap::from([
        //         ((-1, 0), vec![]),
        //         ((0, -1), vec![]),
        //         ((0, 1), vec![]),
        //         ((1, 0), vec![]),
        //     ]),
        // ),
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
