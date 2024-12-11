use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
struct Stones {
    stones: Vec<String>,
}

#[derive(Debug)]
struct ParseStonesError;

impl FromStr for Stones {
    type Err = ParseStonesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            stones: s
                .split_ascii_whitespace()
                .map(|stone| stone.to_owned())
                .collect(),
        })
    }
}

impl Stones {
    fn blink(&self) -> Self {
        Self {
            stones: self
                .stones
                .iter()
                .flat_map(|stone| Self::blink_stone(stone).into_iter())
                .collect(),
        }
    }

    fn blink_stone(stone: &String) -> Vec<String> {
        match stone {
            _ if stone == "0" => vec!["1".to_owned()],
            stone if stone.len() % 2 == 0 => {
                let second_half = stone[stone.len() / 2..].trim_start_matches("0").to_owned();
                vec![
                    stone[..stone.len() / 2].to_owned(),
                    if second_half.is_empty() {
                        "0".to_owned()
                    } else {
                        second_half
                    },
                ]
            }
            _ => {
                if let Ok(number) = stone.parse::<usize>() {
                    vec![(number * 2024).to_string()]
                } else {
                    unreachable!()
                }
            }
        }
    }

    fn blink_count(&self, blink_count: usize) -> usize {
        let mut lookup = HashMap::new();
        self.stones
            .iter()
            .map(|stone| self.stone_count(stone, blink_count, &mut lookup))
            .sum()
    }

    fn stone_count(
        &self,
        stone: &String,
        blink_count: usize,
        lookup: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        if blink_count == 0 {
            1
        } else if let Some(&count) = lookup.get(&(stone.clone(), blink_count)) {
            count
        } else {
            let count = Self::blink_stone(stone)
                .into_iter()
                .map(|stone| self.stone_count(&stone, blink_count - 1, lookup))
                .sum();
            lookup.insert((stone.clone(), blink_count), count);
            count
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let stones: Stones = contents.parse().expect("Should be able to parse stones");
    // println!("{:?}", stones);
    // let mut stones_blinking = stones.clone();
    // for _ in 0..25 {
    //     stones_blinking = stones_blinking.blink();
    //     println!("{:?}", stones_blinking.stones.len());
    // }
    let stones_blinking_cached = stones.blink_count(25);
    println!("{:?}", stones_blinking_cached);
    let stones_blinking_cached_many = stones.blink_count(75);
    println!("{:?}", stones_blinking_cached_many);
}
