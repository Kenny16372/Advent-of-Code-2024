use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    direction: Direction,
    position: (usize, usize),
}

impl Guard {
    fn try_step(&self, lab: &mut Lab) -> Result<Self, ()> {
        let offset = self.direction.to_offset();
        let position_new = (self.position.0 as i32, self.position.1 as i32);
        let position_new = (position_new.0 + offset.0, position_new.1 + offset.1);
        match lab
            .map
            .get(position_new.0 as usize)
            .and_then(|row| row.get(position_new.1 as usize))
        {
            Some(LabTile::Floor) => {
                lab.map[position_new.0 as usize][position_new.1 as usize] = LabTile::Visited;
            }
            Some(LabTile::Visited) => {}
            Some(LabTile::Obstacle) => {
                let mut guard_new = self.clone();
                guard_new.direction = guard_new.direction.turn_right();
                return guard_new.try_step(lab);
            }
            None => return Err(()),
        }
        Ok(Self {
            position: (position_new.0 as usize, position_new.1 as usize),
            direction: self.direction,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LabTile {
    Floor,
    Obstacle,
    Visited,
}

impl Display for LabTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Floor => '.',
            Self::Obstacle => '#',
            Self::Visited => 'X',
        })
    }
}
struct ParseLabTileError;

impl TryFrom<char> for LabTile {
    type Error = ParseLabTileError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Floor),
            '#' => Ok(Self::Obstacle),
            '^' => Ok(Self::Visited),
            _ => Err(ParseLabTileError),
        }
    }
}

#[derive(Debug)]
struct Lab {
    map: Vec<Vec<LabTile>>,
    guard: Guard,
}

#[derive(Debug)]
struct ParseLabError;

impl FromStr for Lab {
    type Err = ParseLabError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.chars().map(|c| c.try_into()).collect())
            .collect::<Result<_, _>>()
            .map(|map: Vec<Vec<LabTile>>| {
                let position = map
                    .iter()
                    .enumerate()
                    .find_map(|(row_idx, row)| {
                        row.iter().enumerate().find_map(|(col_idx, &tile)| {
                            if tile == LabTile::Visited {
                                Some((row_idx, col_idx))
                            } else {
                                None
                            }
                        })
                    })
                    .expect("There should be a starting position");
                Self {
                    map,
                    guard: Guard {
                        direction: Direction::Up,
                        position,
                    },
                }
            })
            .map_err(|_| ParseLabError)
    }
}

impl Lab {
    fn forward_time(&mut self) {
        let mut guard = self.guard.clone();
        while let Ok(guard_new) = guard.try_step(self) {
            guard = guard_new;
        }
    }
}

impl Display for Lab {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, tile) in row.iter().enumerate() {
                if row_idx == self.guard.position.0 && col_idx == self.guard.position.1 {
                    match self.guard.direction {
                        Direction::Down => f.write_char('v')?,
                        Direction::Up => f.write_char('^')?,
                        Direction::Left => f.write_char('<')?,
                        Direction::Right => f.write_char('>')?,
                    }
                    continue;
                }
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");

    let mut lab: Lab = contents.parse().expect("Should be able to parse input");
    lab.forward_time();

    println!("Lab:\n{}", lab);
    let tiles_visited_count = lab
        .map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&tile| tile == LabTile::Visited)
        .count();

    println!("Number of visited tiles: {}", tiles_visited_count);
}
