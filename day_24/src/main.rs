use std::collections::HashMap;

#[derive(Clone)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Value(bool),
}

struct Device {
    gates: HashMap<String, Gate>,
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let inputs = lines.by_ref().take_while(|line| !line.is_empty());

        let mut gates = HashMap::new();

        for (input, value) in
            inputs.map(|input| input.split_once(": ").expect("should be a valid input"))
        {
            let value = value == "1";
            gates.insert(input.to_owned(), Gate::Value(value));
        }

        for input in lines.map(|input| input.split_ascii_whitespace()) {
            let [a, op, b, _, res] = input.collect::<Vec<_>>()[..] else {
                unreachable!()
            };
            let value = match op {
                "AND" => Gate::And(a.to_owned(), b.to_owned()),
                "OR" => Gate::Or(a.to_owned(), b.to_owned()),
                "XOR" => Gate::Xor(a.to_owned(), b.to_owned()),
                _ => unreachable!(),
            };
            gates.insert(res.to_owned(), value);
        }

        Self { gates }
    }
}

impl Device {
    fn calculate(&mut self, target: &String) -> Option<bool> {
        if let Some(gate) = self.gates.get(target).cloned() {
            return Some(match gate {
                Gate::Value(value) => value,
                Gate::And(a, b) => {
                    let a = self.calculate(&a).expect("a should be there");
                    let b = self.calculate(&b).expect("b should be there");
                    let result = a & b;
                    self.gates.insert(target.clone(), Gate::Value(result));
                    result
                }
                Gate::Or(a, b) => {
                    let a = self.calculate(&a).expect("a should be there");
                    let b = self.calculate(&b).expect("b should be there");
                    let result = a | b;
                    self.gates.insert(target.clone(), Gate::Value(result));
                    result
                }
                Gate::Xor(a, b) => {
                    let a = self.calculate(&a).expect("a should be there");
                    let b = self.calculate(&b).expect("b should be there");
                    let result = a ^ b;
                    self.gates.insert(target.clone(), Gate::Value(result));
                    result
                }
            });
        } else {
            None
        }
    }

    fn output(&mut self) -> i64 {
        (0..64)
            .map(|i| {
                let output = format!("z{:02}", i);
                self.calculate(&output)
                    .map(|value| if value { 1 << i } else { 0 })
            })
            .take_while(|o| o.is_some())
            .fold(0, |acc, val| acc + val.unwrap())
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Should be able to read input");

    let mut device: Device = contents.as_str().into();

    println!("Output: {}", device.output());
}
