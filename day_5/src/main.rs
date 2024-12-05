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
    fn is_update_valid(&self, update: &Update) -> bool {
        (0..update.page_numbers.len()).all(|i| {
            let curr = update.page_numbers[i];

            let pages_before_forced = self.is_after.get(&curr);
            let pages_after_forced = self.is_before.get(&curr);

            let violates_pages_before = pages_before_forced.is_some_and(|pages_before_forced| {
                update.page_numbers[i + 1..]
                    .iter()
                    .any(|next| pages_before_forced.contains(next))
            });

            let violates_pages_after = pages_after_forced.is_some_and(|pages_after_forced| {
                update.page_numbers[..i]
                    .iter()
                    .any(|prev| pages_after_forced.contains(prev))
            });

            return !violates_pages_before && !violates_pages_after;
        })
    }

    fn to_pages(&self) -> Vec<Page> {
        self.is_before
            .keys()
            .map(|&number| Page {
                number,
                comes_before: self.is_before.get(&number).unwrap(),
            })
            .collect()
    }
}

#[derive(Clone)]
struct Page<'a> {
    number: usize,
    comes_before: &'a Vec<usize>,
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

    fn reorder(&mut self, page_order: &mut HashMap<usize, usize>) {
        for page in self.page_numbers.iter() {
            page_order.entry(*page).or_insert(0);
        }
        println!("Page order: {:?}", page_order);

        self.page_numbers.sort_by(|a, b| {
            page_order
                .get(a)
                .expect("I'm pretty confident I've inserted all pages")
                .cmp(
                    page_order
                        .get(b)
                        .expect("Maybe I didn't, but who knows these days"),
                )
        });
    }
}

fn get_page_order(pages: Vec<Page>) -> HashMap<usize, usize> {
    let mut result: HashMap<usize, usize> = HashMap::new();
    for page in &pages[..] {
        let mut pls_check_me: Vec<_> = page.comes_before.iter().collect();
        while let Some(number) = pls_check_me.pop() {
            println!("to be checked: {}", pls_check_me.len());
            let entry = *result.entry(*number).or_insert(1);
            if let Some(&this) = result.get(&page.number) {
                if this + 1 > entry {
                    result.insert(*number, this + 1);
                }
            } else {
                result.insert(page.number, 0);
            }

            pages
                .iter()
                .find(|page| page.number == *number)
                .expect("Should exist...")
                .comes_before
                .iter()
                .for_each(|number| {
                    if !pls_check_me.contains(&number) {
                        pls_check_me.push(number)
                    }
                });
        }
    }

    result
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
            if page_rules.is_update_valid(update) {
                Some(update.get_middle_page())
            } else {
                None
            }
        })
        .sum();
    let pages = page_rules.to_pages();
    let mut page_order = get_page_order(pages);

    let updates_invalid_reordered_middle_page_number_sum: usize = updates
        .iter_mut()
        .filter_map(|update| {
            if !page_rules.is_update_valid(update) {
                println!("before: {:?}", update.page_numbers);
                update.reorder(&mut page_order);
                println!("after: {:?}", update.page_numbers);
                Some(update.get_middle_page())
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
