use std::fs;

fn main() {
    let sentences = fs::read_to_string("../input.txt").unwrap();
    let sum: u128 = sentences.split(",").map(apply_hash_algorithm).sum();
    println!("Sum: {}", sum);
}

fn apply_hash_algorithm(sentence: &str) -> u128 {
    sentence.chars().fold(0, |acc, char| {
        let mut current_value = acc;
        current_value += char as u128;
        current_value *= 17;
        current_value %= 256;
        current_value
    })
}
