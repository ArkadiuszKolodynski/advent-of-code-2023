use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Workflow {
    #[allow(dead_code)]
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(name: String, rules: Vec<Rule>) -> Workflow {
        Workflow { name, rules }
    }
}

#[derive(Debug)]
struct Rule {
    next: String,
    condition: Option<Condition>,
}

impl Rule {
    fn new(rule: String) -> Rule {
        if let Some((condition, next)) = rule.split_once(":") {
            Rule {
                next: next.to_string(),
                condition: Some(Condition::new(condition.to_string())),
            }
        } else {
            Rule {
                next: rule,
                condition: None,
            }
        }
    }
}

#[derive(Debug)]
struct Condition {
    operator: char,
    lhs: char,
    rhs: i32,
}

impl Condition {
    fn new(condition: String) -> Condition {
        #[allow(unused_assignments)]
        let mut operator = ' ';
        let (lhe, rhe) = if let Some(index) = condition.find('<') {
            operator = condition.chars().nth(index).unwrap();
            condition.split_at(index)
        } else if let Some(index) = condition.find('>') {
            operator = condition.chars().nth(index).unwrap();
            condition.split_at(index)
        } else {
            panic!("Condition must contain either '<' or '>' character");
        };

        let lhe = if lhe.ends_with(' ') {
            lhe.trim_end().chars().next().unwrap()
        } else {
            lhe.chars().next().unwrap()
        };

        let rhe = rhe
            .trim_start_matches(|c| c == '<' || c == '>')
            .parse::<i32>()
            .unwrap();

        Condition {
            operator,
            lhs: lhe,
            rhs: rhe,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn new(part_string: String) -> Part {
        let mut stripped_part_string = part_string
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",");

        let x = stripped_part_string
            .next()
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let m = stripped_part_string
            .next()
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let a = stripped_part_string
            .next()
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let s = stripped_part_string
            .next()
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();

        Part { x, m, a, s }
    }
}

fn main() {
    let (workflows, parts) = parse_input("../input.txt");

    let sum: i32 = parts
        .iter()
        .map(|p| {
            let mut workflow = workflows.get("in").unwrap();
            let mut next = String::new();

            while next != "A" && next != "R" {
                for rule in &workflow.rules {
                    if let Some(condition) = &rule.condition {
                        let asdf = match condition.lhs {
                            'x' => p.x,
                            'm' => p.m,
                            'a' => p.a,
                            's' => p.s,
                            _ => panic!("Invalid lhs"),
                        };
                        match condition.operator {
                            '<' => {
                                if asdf < condition.rhs {
                                    if rule.next != "A" && rule.next != "R" {
                                        workflow = workflows.get(&rule.next).unwrap();
                                    }
                                    next = rule.next.clone();
                                    break;
                                }
                            }
                            '>' => {
                                if asdf > condition.rhs {
                                    if rule.next != "A" && rule.next != "R" {
                                        workflow = workflows.get(&rule.next).unwrap();
                                    }
                                    next = rule.next.clone();
                                    break;
                                }
                            }
                            _ => panic!("Invalid operator"),
                        }
                    } else {
                        if rule.next != "A" && rule.next != "R" {
                            workflow = workflows.get(&rule.next).unwrap();
                        }
                        next = rule.next.clone();
                        break;
                    }
                }
            }

            if next == "A" {
                p.x + p.m + p.a + p.s
            } else {
                0
            }
        })
        .sum();
    println!("Sum: {}", sum);
}

fn parse_input<P>(filename: P) -> (HashMap<String, Workflow>, Vec<Part>)
where
    P: AsRef<Path>,
{
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut blank = false;
    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let line = ip;
            if line.is_empty() {
                blank = true;
                continue;
            }
            if blank == false {
                let (name, rest) = line.split_once("{").unwrap();
                let rules: Vec<Rule> = rest
                    .replace("}", "")
                    .split(",")
                    .map(|r| Rule::new(r.to_string()))
                    .collect();
                workflows.insert(name.to_string(), Workflow::new(name.to_string(), rules));
            } else {
                parts.push(Part::new(line));
            }
        }
    }
    (workflows, parts)
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
