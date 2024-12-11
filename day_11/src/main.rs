use std::str::FromStr;

#[derive(Debug, Clone)]
struct Stones(Vec<String>);

#[derive(Debug)]
struct ParseStonesError;

impl FromStr for Stones {
    type Err = ParseStonesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_ascii_whitespace()
                .map(|stone| stone.to_owned())
                .collect(),
        ))
    }
}

impl Stones {
    fn blink(&self) -> Self {
        Self(
            self.0
                .iter()
                .flat_map(|stone| Self::blink_stone(stone).into_iter())
                .collect(),
        )
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
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let stones: Stones = contents.parse().expect("Should be able to parse stones");
    // println!("{:?}", stones);
    let mut stones_blinking = stones.clone();
    for _ in 0..25 {
        stones_blinking = stones_blinking.blink();
        println!("{:?}", stones_blinking.0.len());
    }
}
