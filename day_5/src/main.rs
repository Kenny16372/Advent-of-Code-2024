use std::str::FromStr;

#[derive(Debug)]
struct OrderingRule {
    page_number_before: usize,
    page_number_after: usize,
}

#[derive(Debug)]
struct ParseOrderingRuleError;

impl FromStr for OrderingRule {
    type Err = ParseOrderingRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('|');
        let first = split.next();
        let second = split.next();

        if let (Some(first), Some(second)) = (first, second) {
            if let (Ok(page_number_before), Ok(page_number_after)) = (first.parse(), second.parse())
            {
                return Ok(Self {
                    page_number_before,
                    page_number_after,
                });
            } else {
                return Err(ParseOrderingRuleError);
            }
        } else {
            return Err(ParseOrderingRuleError);
        }
    }
}

#[derive(Debug)]
struct Update {
    page_numbers: Vec<usize>,
}

#[derive(Debug)]
struct ParseUpdateError;

impl FromStr for Update {
    type Err = ParseUpdateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map(|page_numbers| Self { page_numbers })
            .map_err(|_| ParseUpdateError)
    }
}

fn main() {
    let contents =
        std::fs::read_to_string("data/input_small.txt").expect("Failed to read the input");
    let mut lines = contents.lines();
    let ordering_rules: Vec<OrderingRule> = (&mut lines)
        .take_while(|&line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("Ordering rules should be valid");
    let updates: Vec<Update> = lines
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("Updates should be valid");

    // println!("Ordering rules: {:?}", ordering_rules);
    // println!("Updates: {:?}", updates);
}
