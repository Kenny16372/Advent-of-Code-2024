use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct TopographicalMap {
    map: Vec<Vec<i8>>,
}

#[derive(Debug)]
struct ParseTopographicalMapError;

impl FromStr for TopographicalMap {
    type Err = ParseTopographicalMapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .expect("Should be able to parse digit")
                            .try_into()
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()
            .map(|map| Self { map })
            .map_err(|_| ParseTopographicalMapError)
    }
}

impl TopographicalMap {
    fn trailhead_sum(&self) -> (usize, usize) {
        self.trailheads()
            .into_iter()
            .map(|trailhead| self.trails(vec![trailhead], 1))
            .map(|trails| {
                let score = trails
                    .iter()
                    .map(|trail| {
                        trail
                            .last()
                            .expect("trail should have at least one step")
                            .clone()
                    })
                    .collect::<HashSet<_>>()
                    .len();
                // println!("start: {:?}, count: {:?}", trails[0][0], score);
                (score, trails.len())
            })
            .fold((0, 0), |(acc_score, acc_len), (val_score, val_len)| {
                (acc_score + val_score, acc_len + val_len)
            })
    }

    fn trails(&self, trail: Vec<(usize, usize)>, height: i8) -> Vec<Vec<(usize, usize)>> {
        if height == 10 {
            return vec![trail];
        }
        self.get_surrounding_of_height(*trail.last().expect("trail shouldn't be empty"), height)
            .into_iter()
            .map(|next_step| {
                let mut trail_new = trail.clone();
                trail_new.push(next_step);
                trail_new
            })
            .flat_map(|trail| self.trails(trail, height + 1).into_iter())
            .collect()
    }

    fn get_surrounding_of_height(
        &self,
        position: (usize, usize),
        height: i8,
    ) -> Vec<(usize, usize)> {
        self.get_surrounding(position.0, position.1)
            .iter()
            .filter(|&&(row, col)| self.map[row][col] == height)
            .map(|&pos| pos)
            .collect()
    }

    fn trailheads(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &height)| height == 0)
                    .map(move |(j, _)| (i, j))
            })
            .collect()
    }

    fn get_surrounding(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .filter_map(move |&(i, j)| {
                let row = row
                    .checked_add_signed(i)
                    .filter(|&row| row < self.map.len());
                let col = col
                    .checked_add_signed(j)
                    .filter(|&col| col < self.map[0].len());
                if let (Some(row), Some(col)) = (row, col) {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let topographical_map: TopographicalMap = contents
        .parse()
        .expect("Should be able to parse topographical map");
    // println!("Topographical map: {:?}", topographical_map);
    let (trailhead_score, trailhead_rating) = topographical_map.trailhead_sum();
    println!("Score of trailheads: {}", trailhead_score);
    println!("Rating of trailheads: {}", trailhead_rating);
}
