use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("data/input1.txt").expect("Failed to read the file");

    let lines = contents.lines();
    let mut left = vec![];
    let mut right = vec![];
    let split = lines.map(|line| {
        line.split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().expect(&format!("Couldn't parse {}", s)))
            .collect::<Vec<_>>()
    });
    for vals in split {
        left.push(vals[0]);
        right.push(vals[1]);
    }
    let mut counts = HashMap::new();
    for value in right {
        *counts.entry(value).or_insert(0) += 1;
    }

    let result = left
        .iter()
        .map(|value| match counts.get(value) {
            Some(&count) => *value * count,
            None => 0,
        })
        .sum::<i32>();
    println!("result: {}", result);
}
