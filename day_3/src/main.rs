use regex::{Captures, Regex};

#[derive(Debug)]
struct Multiply {
    factor1: i32,
    factor2: i32,
}

impl Multiply {
    fn product(&self) -> i32 {
        self.factor1 * self.factor2
    }
}

impl From<Captures<'_>> for Multiply {
    fn from(value: Captures<'_>) -> Self {
        let (_, [first, second]) = value.extract();
        Multiply {
            factor1: first.parse().expect("Should be able to parse first factor"),
            factor2: second
                .parse()
                .expect("Should be able to parse second factor"),
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");

    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Regex should be valid");

    let multiplies = regex
        .captures_iter(&contents)
        .map(|c| c.into())
        .collect::<Vec<Multiply>>();

    println!("Matches: {:?}", multiplies);
    println!(
        "Sum of products: {}",
        multiplies.iter().map(|m| m.product()).sum::<i32>()
    );
}
