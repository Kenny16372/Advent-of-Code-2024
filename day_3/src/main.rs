use std::iter::once;

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

#[derive(Debug)]
enum Token {
    Do,
    Dont,
    Mul(Multiply),
}

impl From<Captures<'_>> for Token {
    fn from(value: Captures<'_>) -> Self {
        let tokens = value.iter().skip(1).collect::<Vec<_>>();
        match &tokens[..] {
            [Some(_), None, None, None] => Token::Do,
            [None, Some(_), None, None] => Token::Dont,
            [None, None, Some(first), Some(second)] => Token::Mul(Multiply {
                factor1: first
                    .as_str()
                    .parse()
                    .expect("Should be able to parse first factor"),
                factor2: second
                    .as_str()
                    .parse()
                    .expect("Should be able to parse second factor"),
            }),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");

    let regex = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)")
        .expect("Regex should be valid");

    let tokens = regex
        .captures_iter(&contents)
        .map(|c| c.into())
        .collect::<Vec<Token>>();

    println!("Matches: {:?}", tokens);
    println!(
        "Sum of products: {}",
        tokens
            .iter()
            .filter_map(|t| match t {
                Token::Mul(v) => Some(v.product()),
                _ => None,
            })
            .sum::<i32>()
    );

    println!(
        "Sum of filtered products: {}",
        once(&Token::Do)
            .chain(tokens.iter())
            .rfold((0, 0), |(tmp, acc), val| {
                match val {
                    Token::Mul(v) => (tmp + v.product(), acc),
                    Token::Dont => (0, acc),
                    Token::Do => (0, acc + tmp),
                }
            })
            .1
    );
}
