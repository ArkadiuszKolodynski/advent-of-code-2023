use std::fs;

fn main() {
    let sentences = fs::read_to_string("../input.txt").unwrap();
    let mut boxes: Vec<(Vec<String>, Vec<usize>)> =
        (0..256).map(|_| (Vec::new(), Vec::new())).collect();

    sentences.split(",").for_each(|sentence| {
        let (label, value) = sentence.split_once(|c| c == '=' || c == '-').unwrap();
        let index = apply_hash_algorithm(label);
        let (ref mut labels, ref mut values) = boxes[index];
        let found_label = labels.iter().position(|x| x == label);
        if found_label.is_some() {
            let i = found_label.unwrap();
            if !value.is_empty() {
                values[i] = value.parse().unwrap();
            } else {
                labels.remove(i);
                values.remove(i);
            }
        } else {
            if !value.is_empty() {
                labels.push(label.to_owned());
                values.push(value.parse().unwrap());
            }
        }
    });

    let sum: usize = boxes
        .into_iter()
        .enumerate()
        .map(|(i, abox)| {
            let (labels, values) = abox;
            labels
                .into_iter()
                .zip(values)
                .enumerate()
                .fold(0, |mut acc, (j, (_, value))| {
                    let focusing_power = (i + 1) * (j + 1) * value;
                    acc = acc + focusing_power;
                    acc
                })
        })
        .sum();
    println!("Sum: {}", sum);
}

fn apply_hash_algorithm(sentence: &str) -> usize {
    sentence.chars().fold(0, |acc, char| {
        let mut current_value = acc;
        current_value += char as usize;
        current_value *= 17;
        current_value %= 256;
        current_value
    })
}
