use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum HandKind {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn main() {
    let mut hands_with_bids: Vec<(String, u32)> = parse_hands_with_bids("../input.txt");

    hands_with_bids.sort_by(order_hands_by_rank);
    let result: u32 = hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1) as u32)
        .sum();

    println!("Sum: {}", result);
}

fn order_hands_by_rank((hand1, _): &(String, u32), (hand2, _): &(String, u32)) -> Ordering {
    let order_result = kind_of_hand(hand1).cmp(&kind_of_hand(hand2));
    if order_result != Ordering::Equal {
        return order_result;
    }

    let index_weights_map = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);
    for i in 0..hand1.len() {
        let order_result = index_weights_map
            .get(&hand1.chars().nth(i).unwrap())
            .unwrap()
            .cmp(
                &index_weights_map
                    .get(&hand2.chars().nth(i).unwrap())
                    .unwrap(),
            );
        if order_result != Ordering::Equal {
            return order_result;
        }
    }

    Ordering::Equal
}

fn kind_of_hand(hand: &str) -> u32 {
    let mut chars_map = hand.chars().fold(HashMap::new(), |mut chars_map, char| {
        let count = chars_map.entry(char).or_insert(0);
        *count += 1;
        chars_map
    });

    if chars_map.get(&'J') != Some(&5) {
        let j_count = chars_map.remove(&'J').unwrap_or(0);
        let highest_value_key = chars_map
            .iter()
            .max_by_key(|(_, &value)| value)
            .map(|(key, _)| *key)
            .unwrap();
        let new_highest = *chars_map.get(&highest_value_key).unwrap_or(&0) + j_count;
        chars_map.insert(highest_value_key, new_highest);
    }

    if chars_map.len() == 1 {
        return HandKind::FiveOfAKind as u32;
    }
    if chars_map.values().any(|&count| count == 4) {
        return HandKind::FourOfAKind as u32;
    }
    if chars_map.len() == 2 && chars_map.values().any(|&count| count == 3) {
        return HandKind::FullHouse as u32;
    }
    if chars_map.values().any(|&count| count == 3) {
        return HandKind::ThreeOfAKind as u32;
    }
    if chars_map.len() == 3 && chars_map.values().any(|&count| count == 2) {
        return HandKind::TwoPair as u32;
    }
    if chars_map.values().len() == 4 {
        return HandKind::OnePair as u32;
    }
    HandKind::HighCard as u32
}

fn parse_hands_with_bids<P>(filename: P) -> Vec<(String, u32)>
where
    P: AsRef<Path>,
{
    let mut hands_with_bids: Vec<(String, u32)> = Vec::new();

    let lines = read_lines(filename);
    for line in lines {
        if let Ok(ip) = line {
            let (hand, rank) = ip.split_once(char::is_whitespace).unwrap();
            hands_with_bids.push((hand.to_string(), rank.to_string().parse().unwrap()));
        }
    }

    hands_with_bids
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
