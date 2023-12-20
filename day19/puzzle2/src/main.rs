use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Condition {
    operator: char,
    lhs: char,
    rhs: usize,
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
            .parse::<usize>()
            .unwrap();

        Condition {
            operator,
            lhs: lhe,
            rhs: rhe,
        }
    }
}

fn main() {
    let workflows = parse_input("../input.txt");

    println!(
        "Sum: {:?}",
        count_accepted(&workflows, "in", [(1, 4000); 4])
    );
}

fn count_accepted(
    workflows: &HashMap<String, Vec<Rule>>,
    target: &str,
    mut ranges: [(usize, usize); 4],
) -> usize {
    if target == "A" {
        return ranges.iter().map(|(lo, hi)| hi - lo + 1).product();
    }
    if target == "R" {
        return 0;
    }
    let mut ans = 0;
    let mut rules = workflows[target].to_vec();
    let next = rules.pop().unwrap().next;
    for rule in &rules {
        let i = "xmas"
            .chars()
            .position(|c| c == rule.condition.as_ref().unwrap().lhs)
            .unwrap();
        let mut newranges = ranges.clone();
        let n = rule.condition.as_ref().unwrap().rhs;
        (newranges[i], ranges[i]) = if rule.condition.as_ref().unwrap().operator == '<' {
            ((ranges[i].0, n - 1), (n, ranges[i].1))
        } else {
            ((n + 1, ranges[i].1), (ranges[i].0, n))
        };
        ans += count_accepted(workflows, &rule.next, newranges);
    }
    ans += count_accepted(workflows, &next, ranges);
    ans
}

fn parse_input<P>(filename: P) -> HashMap<String, Vec<Rule>>
where
    P: AsRef<Path>,
{
    let mut workflows = HashMap::new();

    for line in read_lines(filename) {
        if let Ok(ip) = line {
            let line = ip;
            if line.is_empty() {
                return workflows;
            }

            let (name, rest) = line.split_once("{").unwrap();
            let rules: Vec<Rule> = rest
                .replace("}", "")
                .split(",")
                .map(|r| Rule::new(r.to_string()))
                .collect();
            workflows.insert(name.to_string(), rules);
        }
    }
    panic!("No blank line found");
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
