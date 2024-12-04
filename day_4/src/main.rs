use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Character {
    X,
    M,
    A,
    S,
}

struct ParseCharacterError;

impl TryFrom<char> for Character {
    type Error = ParseCharacterError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Character::X),
            'M' => Ok(Character::M),
            'A' => Ok(Character::A),
            'S' => Ok(Character::S),
            _ => Err(ParseCharacterError),
        }
    }
}

#[derive(Debug)]
struct WordSearch {
    lines: Vec<Vec<Character>>,
}

#[derive(Debug)]
struct ParseWordSearchError;

impl FromStr for WordSearch {
    type Err = ParseWordSearchError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.chars().map(|c| c.try_into()).collect())
            .collect::<Result<_, _>>()
            .map(|lines| Self { lines })
            .map_err(|_| ParseWordSearchError)
    }
}

impl WordSearch {
    fn matches_sequence_right(&self, seq: &Vec<Character>, row: usize, col: usize) -> bool {
        let line = &self.lines[row][col..];
        if line.len() < seq.len() {
            return false;
        }
        line.iter().zip(seq.iter()).all(|(&l, &s)| l == s)
    }
    fn matches_sequence_left(&self, seq: &Vec<Character>, row: usize, col: usize) -> bool {
        let line = &self.lines[row][..=col];
        if line.len() < seq.len() {
            return false;
        }
        line.iter().rev().zip(seq.iter()).all(|(&l, &s)| l == s)
    }
    fn matches_sequence_up(&self, seq: &Vec<Character>, row: usize, col: usize) -> bool {
        seq.iter().enumerate().all(|(offset, &c)| {
            if let Some(real_row) = row.checked_sub(offset) {
                self.lines
                    .get(real_row)
                    .and_then(|r| r.get(col))
                    .is_some_and(|&v| v == c)
            } else {
                false
            }
        })
    }
    fn matches_sequence_down(&self, seq: &Vec<Character>, row: usize, col: usize) -> bool {
        seq.iter().enumerate().all(|(offset, &c)| {
            self.lines
                .get(row + offset)
                .and_then(|r| r.get(col))
                .is_some_and(|&v| v == c)
        })
    }
    fn matches_sequence_diagonally_down(
        &self,
        seq: &Vec<Character>,
        row: usize,
        col: usize,
    ) -> bool {
        seq.iter().enumerate().all(|(offset, &c)| {
            self.lines
                .get(row + offset)
                .and_then(|r| r.get(col + offset))
                .is_some_and(|&v| v == c)
        })
    }
    fn matches_sequence_diagonally_up(&self, seq: &Vec<Character>, row: usize, col: usize) -> bool {
        seq.iter().enumerate().all(|(offset, &c)| {
            if let (Some(real_row), Some(real_col)) =
                (row.checked_sub(offset), col.checked_sub(offset))
            {
                self.lines
                    .get(real_row)
                    .and_then(|r| r.get(real_col))
                    .is_some_and(|&v| v == c)
            } else {
                false
            }
        })
    }
    fn matches_sequence_crossdiagonally_up(
        &self,
        seq: &Vec<Character>,
        row: usize,
        col: usize,
    ) -> bool {
        seq.iter().enumerate().all(|(offset, &c)| {
            if let Some(real_row) = row.checked_sub(offset) {
                self.lines
                    .get(real_row)
                    .and_then(|r| r.get(col + offset))
                    .is_some_and(|&v| v == c)
            } else {
                false
            }
        })
    }
    fn matches_sequence_crossdiagonally_down(
        &self,
        seq: &Vec<Character>,
        row: usize,
        col: usize,
    ) -> bool {
        seq.iter().enumerate().all(|(offset, &c)| {
            if let Some(real_col) = col.checked_sub(offset) {
                self.lines
                    .get(row + offset)
                    .and_then(|r| r.get(real_col))
                    .is_some_and(|&v| v == c)
            } else {
                false
            }
        })
    }
    fn count_matches_at(&self, seq: &Vec<Character>, row: usize, col: usize) -> usize {
        vec![
            self.matches_sequence_crossdiagonally_down(seq, row, col),
            self.matches_sequence_crossdiagonally_up(seq, row, col),
            self.matches_sequence_diagonally_down(seq, row, col),
            self.matches_sequence_diagonally_up(seq, row, col),
            self.matches_sequence_right(seq, row, col),
            self.matches_sequence_left(seq, row, col),
            self.matches_sequence_up(seq, row, col),
            self.matches_sequence_down(seq, row, col),
        ]
        .iter()
        .map(|&found| if found { 1 } else { 0 })
        .sum()
    }

    fn count_matches(&self, seq: &Vec<Character>) -> usize {
        (0..self.lines[0].len())
            .flat_map(|row| (0..self.lines.len()).map(move |col| (row, col)))
            .map(|(row, col)| {
                let res = self.count_matches_at(seq, row, col);
                if res != 0 {
                    println!("found {} at row {}, col {}", res, row, col);
                }
                res
            })
            .sum()
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");

    let word_search: WordSearch = contents.parse().expect("Word search should be valid");

    // println!("Word search:\n{:?}", word_search)
    let seq = vec![Character::X, Character::M, Character::A, Character::S];
    println!("{:?}", word_search.count_matches(&seq));
}
