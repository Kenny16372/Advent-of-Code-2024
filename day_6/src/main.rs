use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Write},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

enum StepError {
    OutOfBounds,
    Loop,
}

impl Guard {
    fn try_step(&self, lab: &mut Lab) -> Result<Self, StepError> {
        if !lab.guard_history.insert(GuardHistory {
            row: self.position.0,
            col: self.position.1,
            dir: self.direction,
        }) {
            println!("loop at ({}, {})", self.position.0, self.position.1);
            return Err(StepError::Loop);
        }
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
            None => return Err(StepError::OutOfBounds),
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

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct GuardHistory {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Debug, Clone)]
struct Lab {
    map: Vec<Vec<LabTile>>,
    guard: Guard,
    guard_history: HashSet<GuardHistory>,
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
                    guard_history: HashSet::new(),
                }
            })
            .map_err(|_| ParseLabError)
    }
}

impl Lab {
    fn forward_time(&mut self) -> bool {
        let mut guard = self.guard.clone();
        loop {
            match guard.try_step(self) {
                Ok(guard_new) => guard = guard_new,
                Err(StepError::Loop) => return true,
                Err(StepError::OutOfBounds) => return false,
            }
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

    let lab_template: Lab = contents.parse().expect("Should be able to parse input");
    let mut first_lab = lab_template.clone();
    let loop_detected = first_lab.forward_time();
    if loop_detected {
        panic!("Initial run must not contain a loop");
    }

    println!("Lab:\n{}", first_lab);
    let tiles_visited_count = first_lab
        .map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&tile| tile == LabTile::Visited)
        .count();

    println!("Number of visited tiles: {}", tiles_visited_count);

    let (test_row, test_col) = (6, 3);
    let mut lab_test = lab_template.clone();
    lab_test.map[test_row][test_col] = LabTile::Obstacle;
    println!("Test Lab:\n{}", lab_test);
    println!("{}", lab_test.forward_time());

    let loop_count = first_lab
        .map
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, &tile)| ((row_idx, col_idx), tile))
        })
        .filter(|(position, tile)| {
            *tile == LabTile::Visited
                && !(position.0 == lab_template.guard.position.0
                    && position.1 == lab_template.guard.position.1)
        })
        .filter_map(|((row, col), _)| {
            // println!("row: {}, col: {}", row, col);
            let mut lab = lab_template.clone();
            lab.map[row][col] = LabTile::Obstacle;

            let loop_detected = lab.forward_time();
            // println!("{}", lab);
            if loop_detected {
                Some(())
            } else {
                None
            }
        })
        // .take(0)
        .count();

    println!("Number of possible loops: {}", loop_count);
}
