use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
struct PageRules {
    is_before: HashMap<usize, Vec<usize>>,
    is_after: HashMap<usize, Vec<usize>>,
}

impl From<Vec<OrderingRule>> for PageRules {
    fn from(value: Vec<OrderingRule>) -> Self {
        let mut is_before = HashMap::new();
        let mut is_after = HashMap::new();

        for rule in value {
            is_before
                .entry(rule.page_number_before)
                .or_insert_with(|| vec![])
                .push(rule.page_number_after);

            is_after
                .entry(rule.page_number_after)
                .or_insert_with(|| vec![])
                .push(rule.page_number_before);
        }

        Self {
            is_before,
            is_after,
        }
    }
}

impl PageRules {
    fn is_order_valid(&self, order: &Vec<usize>) -> bool {
        (0..order.len()).all(|i| {
            let curr = order[i];

            let pages_before_forced = self.is_after.get(&curr);
            let pages_after_forced = self.is_before.get(&curr);

            let violates_pages_before = pages_before_forced.is_some_and(|pages_before_forced| {
                order[i + 1..]
                    .iter()
                    .any(|next| pages_before_forced.contains(next))
            });

            let violates_pages_after = pages_after_forced.is_some_and(|pages_after_forced| {
                order[..i]
                    .iter()
                    .any(|prev| pages_after_forced.contains(prev))
            });

            return !violates_pages_before && !violates_pages_after;
        })
    }
}

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

impl Update {
    fn get_middle_page(&self) -> usize {
        assert_eq!(
            self.page_numbers.len() % 2,
            1,
            "There should be an odd number of pages"
        );
        self.page_numbers[self.page_numbers.len() / 2]
    }

    fn reorder(&self, page_rules: &PageRules) -> Vec<usize> {
        let comes_before_rules: HashMap<usize, HashSet<usize>> = page_rules
            .is_after
            .iter()
            .map(|(&key, values)| (key, values.iter().map(|&v| v).collect::<HashSet<_>>()))
            .collect();

        let pages: HashSet<_> = self.page_numbers.iter().map(|&v| v).collect();
        let mut result = Vec::with_capacity(self.page_numbers.len());
        let mut step = 0;

        while result.len() != self.page_numbers.len() {
            // println!("result: {:?}", result);
            if step >= pages.len() {
                panic!("too many iterations. We should be able to place at least one element per iteration");
            } else {
                step += 1;
            }
            let already_inserted: HashSet<_> = result.iter().map(|&v| v).collect();
            let to_be_inserted: HashSet<_> =
                pages.difference(&already_inserted).map(|&v| v).collect();
            to_be_inserted.iter().for_each(|&number| {
                if let Some(rules) = comes_before_rules.get(&number) {
                    let rules_that_apply: Vec<_> = rules.intersection(&to_be_inserted).collect();
                    // println!("rules that apply to {}: {:?}", number, rules_that_apply);
                    if rules_that_apply.len() == 0 {
                        result.push(number);
                    }
                } else {
                    result.push(number);
                }
            });
        }

        if page_rules.is_order_valid(&result) {
            return result;
        } else {
            panic!("ordering didn't work")
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");
    let mut lines = contents.lines();
    let ordering_rules: Vec<OrderingRule> = (&mut lines)
        .take_while(|&line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("Ordering rules should be valid");
    let mut updates: Vec<Update> = lines
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .expect("Updates should be valid");
    let page_rules: PageRules = ordering_rules.into();

    let updates_valid_middle_page_number_sum: usize = (&updates)
        .iter()
        .filter_map(|update| {
            if page_rules.is_order_valid(&update.page_numbers) {
                Some(update.get_middle_page())
            } else {
                None
            }
        })
        .sum();

    let updates_invalid_reordered_middle_page_number_sum: usize = updates
        .iter_mut()
        .filter_map(|update| {
            if !page_rules.is_order_valid(&update.page_numbers) {
                println!("before: {:?}", update.page_numbers);
                let updated_order = update.reorder(&page_rules);
                println!("after: {:?}", updated_order);
                Some(
                    Update {
                        page_numbers: updated_order,
                    }
                    .get_middle_page(),
                )
            } else {
                None
            }
        })
        .sum();

    // println!("Ordering rules: {:?}", ordering_rules);
    // println!("Updates: {:?}", updates);
    // println!("Page rules: {:?}", page_rules);
    println!(
        "Sum of middle pages of valid updates: {}",
        updates_valid_middle_page_number_sum
    );
    println!(
        "Sum of middle pages of invalid updates after reordering: {}",
        updates_invalid_reordered_middle_page_number_sum
    );
}
