use std::str::FromStr;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Concatenate => {
                assert_ne!(rhs.signum(), -1, "right hand side must not be negative");
                let mut s = lhs.to_string();
                s.push_str(&rhs.to_string());
                s.parse()
                    .expect("Concatenation of two i64 should be an i64")
            }
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: i64,
    values: Vec<i64>,
}

#[derive(Debug)]
struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(": ");
        let result = iter.next().and_then(|v| v.parse().ok());
        let values = iter.next().and_then(|s| {
            s.split_ascii_whitespace()
                .map(|v| v.parse())
                .collect::<Result<_, _>>()
                .ok()
        });
        if let (Some(result), Some(values)) = (result, values) {
            Ok(Self { result, values })
        } else {
            Err(ParseEquationError)
        }
    }
}

impl Equation {
    fn is_valid(&self, ops: &Vec<Operator>) -> bool {
        if ops.len() + 1 != self.values.len() {
            panic!("invalid number of operators");
        }
        self.values
            .iter()
            .skip(1)
            .zip(ops.iter())
            .fold((self.values[0], Operator::Add), |(acc, _), (&val, &op)| {
                (op.apply(acc, val), op)
            })
            .0
            == self.result
    }

    fn is_solvable(&self) -> bool {
        let ops_count = self.values.len() - 1;
        (0..2usize.pow(ops_count.try_into().unwrap())).any(|lookup| {
            let ops = (0..ops_count)
                .map(move |shift| {
                    if lookup & (1 << shift) == 0 {
                        Operator::Add
                    } else {
                        Operator::Multiply
                    }
                })
                .collect();

            self.is_valid(&ops)
        })
    }

    fn is_solvable_with_concatenation(&self) -> bool {
        let ops_count = self.values.len() - 1;
        let all_ops = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];
        (0..ops_count)
            .map(|_| all_ops.iter().map(|&v| v))
            .multi_cartesian_product()
            .any(|ops| self.is_valid(&ops))
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");

    let equations: Vec<Equation> = contents
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("Should be able to parse equations");

    // println!("Equations: {:?}", equations);
    let equations_solvable_sum: i64 = equations
        .iter()
        .filter(|eq| eq.is_solvable())
        .map(|eq| eq.result)
        .sum();
    println!("Sum of solvable equations: {}", equations_solvable_sum);

    let equations_solvable_including_concatenation_sum: i64 = equations
        .iter()
        .filter(|eq| eq.is_solvable_with_concatenation())
        .map(|eq| eq.result)
        .sum();
    println!(
        "Sum of solvable equations (including concatenation): {}",
        equations_solvable_including_concatenation_sum
    );
}
