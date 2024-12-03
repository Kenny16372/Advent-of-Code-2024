use std::str::FromStr;

struct Report {
    levels: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseReportError;

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map(|levels| Self { levels })
            .map_err(|_| ParseReportError)
    }
}

impl Report {
    fn is_stable(&self) -> bool {
        assert!(
            self.levels.len() >= 2,
            "Should contain at least 2 levels per report",
        );
        let is_increasing = self.levels[1] > self.levels[0];
        self.levels.windows(2).all(|window| {
            let left = window[0];
            let right = window[1];

            let diff = (left - right).abs();
            if diff < 1 || diff > 3 {
                return false;
            }

            if is_increasing {
                if left > right {
                    return false;
                }
            } else {
                if right > left {
                    return false;
                }
            }
            true
        })
    }

    fn is_stable_dampened(&self) -> bool {
        (0..self.levels.len())
            .map(|idx_skip| {
                self.levels
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &v)| if i == idx_skip { None } else { Some(v) })
                    .collect()
            })
            .any(|levels| Self { levels }.is_stable())
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");

    let lines = contents.lines();
    let reports = lines.map(|line| {
        line.parse::<Report>()
            .expect("Should be able to parse all reports")
    });
    let safe_reports_count = reports.clone().filter(|r| r.is_stable()).count();
    println!("Number of safe reports: {}", safe_reports_count);

    let safe_reports_count_dampened = reports.filter(|r| r.is_stable_dampened()).count();
    println!(
        "Number of safe reports after dampening: {}",
        safe_reports_count_dampened
    );
}
