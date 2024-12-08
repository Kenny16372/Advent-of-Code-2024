use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn try_add(&self, diff: (isize, isize), bounds: (usize, usize)) -> Option<Self> {
        let x = self.x.checked_add_signed(diff.1).filter(|&x| x < bounds.1);
        let y = self.y.checked_add_signed(diff.0).filter(|&y| y < bounds.0);

        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else {
            None
        }
    }

    fn try_sub(&self, diff: (isize, isize), bounds: (usize, usize)) -> Option<Self> {
        let x = self.x.checked_add_signed(-diff.1).filter(|&x| x < bounds.1);
        let y = self.y.checked_add_signed(-diff.0).filter(|&y| y < bounds.0);

        if let (Some(x), Some(y)) = (x, y) {
            Some(Self { x, y })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Antenna {
    frequency: char,
    position: Position,
}

impl Hash for Antenna {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PartialEq for Antenna {
    fn eq(&self, other: &Antenna) -> bool {
        self.position == other.position
    }
}

impl Eq for Antenna {}

#[derive(Debug)]
struct City {
    bounds: (usize, usize),
    antennas: Vec<Antenna>,
}

#[derive(Debug)]
struct ParseCityError;

impl FromStr for City {
    type Err = ParseCityError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antennas = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(col, c)| match c {
                        '.' => None,
                        c => Some(Antenna {
                            position: Position { x: col, y: row },
                            frequency: c,
                        }),
                    })
            })
            .collect();
        let width = s.split_ascii_whitespace().take(1).collect::<String>().len();
        let height = s.lines().count();
        Ok(Self {
            antennas,
            bounds: (height, width),
        })
    }
}

impl City {
    fn antinodes(&self) -> HashSet<Antenna> {
        let mut result = HashSet::new();

        for (frequency, chunk) in &self
            .antennas
            .iter()
            .sorted_by_key(|a| a.frequency)
            .chunk_by(|a| a.frequency)
        {
            for combination in chunk.combinations(2) {
                let [a, b] = combination[..] else {
                    panic!("found a pair consisting of some other number, but not two, elements. Hopefully I'll get a nobel prize for this");
                };
                let diff = (
                    a.position.y as isize - b.position.y as isize,
                    a.position.x as isize - b.position.x as isize,
                );
                if let Some(position) = a.position.try_add(diff, self.bounds) {
                    result.insert(Antenna {
                        frequency,
                        position,
                    });
                }
                if let Some(position) = b.position.try_sub(diff, self.bounds) {
                    result.insert(Antenna {
                        frequency,
                        position,
                    });
                }
            }
        }

        result
    }

    fn antinodes_resonant_harmonics(&self) -> HashSet<Antenna> {
        let mut result = HashSet::new();

        for (frequency, chunk) in &self
            .antennas
            .iter()
            .sorted_by_key(|a| a.frequency)
            .chunk_by(|a| a.frequency)
        {
            for combination in chunk.combinations(2) {
                let [a, b] = combination[..] else {
                    panic!("found a pair consisting of some other number, but not two, elements. Hopefully I'll get a nobel prize for this");
                };
                let diff = (
                    a.position.y as isize - b.position.y as isize,
                    a.position.x as isize - b.position.x as isize,
                );

                result.insert(Antenna {
                    frequency,
                    position: a.position,
                });
                result.insert(Antenna {
                    frequency,
                    position: b.position,
                });

                let mut position_current = a.position;
                while let Some(position) = position_current.try_add(diff, self.bounds) {
                    result.insert(Antenna {
                        frequency,
                        position,
                    });
                    position_current = position;
                }

                let mut position_current = a.position;
                while let Some(position) = position_current.try_sub(diff, self.bounds) {
                    result.insert(Antenna {
                        frequency,
                        position,
                    });
                    position_current = position;
                }
            }
        }

        result
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");
    let city: City = contents.parse().expect("Should be able to parse city");
    // println!("City: {:?}", city);
    let antinodes = city.antinodes();
    println!("Antinodes: {}", antinodes.len());
    let antinodes_resonant_harmonics = city.antinodes_resonant_harmonics();
    println!(
        "Antinodes (considering resonant harmonics): {}",
        antinodes_resonant_harmonics.len()
    );
}
