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

// Source: https://stackoverflow.com/q/60835260/16185675

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BTreeSet;

type Nodes<'a> = BTreeSet<&'a str>;
type Graph<'a> = HashMap<&'a str, Nodes<'a>>;
type Record<'a> = (&'a str, &'a str);

fn init_nodes<'a>(records: &'a [Record]) -> Graph<'a> {
    let mut nodes: Graph = Graph::with_capacity(records.len());
    for r in records.iter() {
        let n: &mut Nodes = match nodes.entry(r.0) {
            Vacant(entry) => entry.insert(Nodes::new()),
            Occupied(entry) => entry.into_mut(),
        };
        n.insert(r.1);
        let n: &mut Nodes = match nodes.entry(r.1) {
            Vacant(entry) => entry.insert(Nodes::new()),
            Occupied(entry) => entry.into_mut(),
        };
        n.insert(r.0);
    }
    nodes.shrink_to_fit();
    nodes
}

fn bron1<'a>(
    graph: &'a Graph,
    r: Nodes<'a>,
    mut p: Nodes<'a>,
    mut x: Nodes<'a>,
    cliques: &mut Vec<Nodes<'a>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
    } else if !p.is_empty() {
        let nodes = p.iter().cloned().collect::<Nodes>();
        nodes.iter().for_each(|node| {
            let neighbours: &Nodes = graph.get(node).unwrap();
            let mut to_add: Nodes = Nodes::new();
            to_add.insert(*node);
            bron1(
                graph,
                r.union(&to_add).cloned().collect(),
                p.intersection(&neighbours).cloned().collect(),
                x.intersection(&neighbours).cloned().collect(),
                cliques,
            );
            p.remove(node);
            x.insert(*node);
        });
    }
}

fn display_cliques(cliques: &[Nodes]) {
    let max = (&cliques[0]).len();
    let mut count = 0;
    for (idx, cl) in cliques.iter().enumerate() {
        if cl.len() != max {
            count = idx;
            break;
        }
    }
    let clique_largest = cliques
        .iter()
        .max_by_key(|c| c.len())
        .expect("should contain at least one clique");
    println!("{:?}", clique_largest);
    let mut clique_largest: Vec<_> = clique_largest.into_iter().collect();
    clique_largest.sort();
    println!(
        "{}",
        clique_largest
            .iter()
            .map(|&&s| s)
            .collect::<Vec<_>>()
            .join(",")
    );
    println!(
        "Found {} cliques of {} nodes on a total of {} cliques",
        count,
        max,
        cliques.len()
    )
}

fn cliques<'a>(s: &'a str) {
    let records: Vec<Record> = s
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();

    let nodes = init_nodes(&records);
    let r: Nodes = nodes.keys().copied().collect();
    let mut cliques: Vec<Nodes> = Vec::new();
    bron1(&nodes, Nodes::new(), r, Nodes::new(), &mut cliques);
    cliques.sort_unstable_by(|a, b| a.len().cmp(&b.len()).reverse());
    display_cliques(&cliques);
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let network = Network::from_str(&contents);

    println!("{:?}", network.sets_of_three().len());

    cliques(&contents);
}
