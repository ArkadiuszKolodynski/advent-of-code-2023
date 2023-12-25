use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Grid = Vec<Vec<char>>;

fn main() {
    let (starting_point, ending_point, grid) = parse_input("../input.txt");

    let steps = walk(&grid, starting_point, ending_point);

    println!("Steps: {}", steps);
}

fn walk(grid: &Grid, start: (isize, isize), end: (isize, isize)) -> isize {
    let mut points = Vec::from([start, end]);
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'#' {
                continue;
            }
            let mut neighbours = 0;
            for (new_row, new_column) in [
                (r as isize + 1, c as isize),
                (r as isize - 1, c as isize),
                (r as isize, c as isize + 1),
                (r as isize, c as isize - 1),
            ] {
                if new_row >= 0
                    && new_row < grid.len() as isize
                    && new_column >= 0
                    && new_column < grid[0].len() as isize
                    && grid[new_row as usize][new_column as usize] != '#'
                {
                    neighbours += 1;
                }
            }
            if neighbours >= 3 {
                points.push((r as isize, c as isize));
            }
        }
    }

    let mut graph = HashMap::new();
    // let dirs = HashMap::from([
    //     ('^', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    //     ('v', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    //     ('<', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    //     ('>', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    //     ('.', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
    // ]);

    for (starting_row, starting_column) in &points {
        let mut stack = VecDeque::new();
        let mut seen = HashSet::new();
        stack.push_front((0, *starting_row, *starting_column));
        seen.insert((*starting_row, *starting_column));

        while let Some((n, row, column)) = stack.pop_back() {
            if n != 0 && points.contains(&(row, column)) {
                graph
                    .entry((*starting_row, *starting_column))
                    .and_modify(|e: &mut HashMap<(isize, isize), isize>| {
                        e.insert((row, column), n);
                    })
                    .or_insert(HashMap::from([((row, column), n)]));
                continue;
            }

            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_row = row + dr;
                let new_column = column + dc;

                if new_row >= 0
                    && new_row < grid.len() as isize
                    && new_column >= 0
                    && new_column < grid[0].len() as isize
                    && grid[new_row as usize][new_column as usize] != '#'
                    && !seen.contains(&(new_row, new_column))
                {
                    {
                        stack.push_back((n + 1, new_row, new_column));
                        seen.insert((new_row, new_column));
                    }
                }
            }
        }
    }

    dfs(start, end, &graph, &mut HashSet::new())
}

fn dfs(
    start: (isize, isize),
    end: (isize, isize),
    graph: &HashMap<(isize, isize), HashMap<(isize, isize), isize>>,
    seen: &mut HashSet<(isize, isize)>,
) -> isize {
    if start == end {
        return 0;
    }

    let mut steps = isize::MIN;

    seen.insert(start);
    for (next_point, value) in graph.get(&start).unwrap() {
        if !seen.contains(next_point) {
            steps = max(steps, dfs(*next_point, end, graph, seen) + value);
        }
    }
    seen.remove(&start);

    steps
}

fn parse_input<P>(filename: P) -> ((isize, isize), (isize, isize), Grid)
where
    P: AsRef<Path>,
{
    let mut starting_point = (0, 0);
    let mut starting_point_found = false;
    let mut ending_point = (0, 0);
    let mut grid = Grid::new();
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            if ip.chars().filter(|&c| c == '.').count() == 1 {
                let i = ip.find('.').unwrap();
                if !starting_point_found {
                    starting_point = (grid.len() as isize, i as isize);
                    starting_point_found = true;
                } else {
                    ending_point = (grid.len() as isize, i as isize);
                }
            }
            grid.push(ip.chars().collect());
        }
    }
    (starting_point, ending_point, grid)
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
