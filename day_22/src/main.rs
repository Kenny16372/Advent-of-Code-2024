use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct SecretNumber(i64);

#[derive(Debug)]
struct ParseSecretNumberError;

impl FromStr for SecretNumber {
    type Err = ParseSecretNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(|val| Self(val))
            .map_err(|_| ParseSecretNumberError {})
    }
}

impl SecretNumber {
    fn nth(&self, n: usize) -> Self {
        let mut result = self.clone();
        for _ in 0..n {
            result = result.next();
        }
        result
    }

    fn next(&self) -> Self {
        let next = self.mix(self.0 * 64).prune();
        let next = next.mix(next.0 / 32).prune();
        next.mix(next.0 * 2048).prune()
    }

    fn mix(&self, value: i64) -> Self {
        Self(self.0 ^ value).prune()
    }

    fn prune(&self) -> Self {
        Self(self.0 % 16777216)
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let secret_numbers_sum: i64 = contents
        .lines()
        .map(|line| {
            line.parse::<SecretNumber>()
                .expect("Should be able to parse to secret number")
        })
        .map(|secret_number| secret_number.nth(2000).0)
        .sum();

    println!("Sum of secret numbers: {}", secret_numbers_sum);
}
