use std::str::FromStr;

#[derive(Debug)]
struct Door {
    locks: Vec<i32>,
    keys: Vec<i32>,
}

#[derive(Debug)]
struct ParseDoorError;

impl FromStr for Door {
    type Err = ParseDoorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for thing in s.split("\n\n") {
            let thing: Vec<Vec<i32>> = thing
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => 0,
                            '#' => 1,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect();

            let is_lock = thing[0][0] == 1;

            let mut result = 0;

            for col in 0..thing[0].len() {
                for row in 1..thing.len() - 1 {
                    result <<= 1;
                    result |= thing[row][col];
                }
            }

            if is_lock {
                locks.push(result);
            } else {
                keys.push(result);
            }
        }

        Ok(Self { locks, keys })
    }
}

impl Door {
    fn num_fitting_pairs(&self) -> usize {
        self.locks
            .iter()
            .flat_map(|&lock| self.keys.iter().map(move |&key| (lock, key)))
            .filter(|&(lock, key)| lock & key == 0)
            .count()
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let door: Door = contents.parse().expect("should be able to parse door");

    let fitting_pairs_count = door.num_fitting_pairs();
    println!("Number of fitting pairs: {fitting_pairs_count}");
}
