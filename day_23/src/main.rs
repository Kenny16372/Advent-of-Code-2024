use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Network<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Network<'a> {
    fn from_str(s: &'a str) -> Network<'a> {
        let mut connections: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
        for result in s.lines().map(|line| line.split_once('-')) {
            let Some((a, b)) = result else { unreachable!() };
            connections.entry(a).or_default().push(b);
            connections.entry(b).or_default().push(a);
        }
        Self { connections }
    }

    fn sets_of_three(&self) -> HashSet<Vec<&str>> {
        let mut result = HashSet::new();

        for (first, firsts) in self.connections.iter() {
            for second in firsts.iter() {
                let seconds = self
                    .connections
                    .get(second)
                    .expect("should not disconnect machines");
                for third in seconds {
                    if firsts.contains(third) {
                        if first.starts_with('t')
                            || second.starts_with('t')
                            || third.starts_with('t')
                        {
                            let mut triple = vec![*first, *second, *third];
                            triple.sort();
                            result.insert(triple);
                        }
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let network = Network::from_str(&contents);

    println!("{:?}", network.sets_of_three().len());
}
