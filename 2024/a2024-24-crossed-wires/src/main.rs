use crate::GateType::Constant;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

enum GateType<'a> {
    Constant(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

struct Gate<'a> {
    gate_type: GateType<'a>,
    next: Vec<&'a str>,
    name: &'a str,
}

fn parse(content: &str) -> HashMap<&str, Gate> {
    let mut splitted = content.split("\n\n");

    let mut graph = HashMap::new();
    for line in splitted.next().unwrap().lines() {
        let mut split_line = line.split(": ");
        let name = split_line.next().unwrap();
        let value = split_line.next().unwrap() == "1";

        graph.insert(
            name,
            Gate {
                gate_type: Constant(value),
                next: vec![],
                name,
            },
        );
    }

    let mut edges = vec![];
    for line in splitted.next().unwrap().lines() {
        let mut splitted = line.split(" -> ");
        let inputs = splitted.next().unwrap();
        let name = splitted.next().unwrap();
        let (gate_type, inputs) = if inputs.contains(" AND ") {
            let inputs = inputs.split(" AND ").collect::<Vec<_>>();
            (GateType::And(inputs[0], inputs[1]), inputs)
        } else if inputs.contains(" OR ") {
            let inputs = inputs.split(" OR ").collect::<Vec<_>>();
            (GateType::Or(inputs[0], inputs[1]), inputs)
        } else if inputs.contains(" XOR ") {
            let inputs = inputs.split(" XOR ").collect::<Vec<_>>();
            (GateType::Xor(inputs[0], inputs[1]), inputs)
        } else {
            panic!("Uknown operation: line={line}");
        };
        edges.push((inputs[0], name));
        edges.push((inputs[1], name));
        graph.insert(
            name,
            Gate {
                gate_type,
                next: vec![],
                name,
            },
        );
    }

    for (from, to) in edges {
        graph.get_mut(from).unwrap().next.push(to);
    }

    graph
}

fn print_gate(gate: &Gate) {
    match gate.gate_type {
        Constant(_) => println!("{}", gate.name),
        GateType::And(i1, i2) => println!("{} = {} & {}", gate.name, i1, i2),
        GateType::Or(i1, i2) => println!("{} = {} | {}", gate.name, i1, i2),
        GateType::Xor(i1, i2) => println!("{} = {} ^ {}", gate.name, i1, i2),
    };
}

fn topo_sort<'a>(graph: &'a HashMap<&'a str, Gate>) -> (Vec<&'a str>, HashMap<&'a str, bool>) {
    let mut in_degrees: HashMap<&str, u32> =
        graph.keys().map(|key| (*key, 0)).collect::<HashMap<_, _>>();
    for gate in graph.values() {
        for next in &gate.next {
            *in_degrees.get_mut(next).unwrap() += 1;
        }
    }

    let mut q = VecDeque::new();
    for (gate, degree) in &in_degrees {
        if *degree == 0 {
            q.push_back(*gate);
        }
    }

    let mut values = HashMap::new();
    let mut sorted = vec![];
    while let Some(gate) = q.pop_front() {
        let gate = graph.get(&gate).unwrap();
        print_gate(gate);
        sorted.push(gate.name);
        let value = match gate.gate_type {
            GateType::Constant(value) => value,
            GateType::And(in1, in2) => {
                let in1 = values[&in1];
                let in2 = values[&in2];
                in1 && in2
            }
            GateType::Or(in1, in2) => {
                let in1 = values[&in1];
                let in2 = values[&in2];
                in1 || in2
            }
            GateType::Xor(in1, in2) => {
                let in1 = values[&in1];
                let in2 = values[&in2];
                in1 ^ in2
            }
        };
        values.insert(gate.name, value);
        for next in &gate.next {
            *(in_degrees.get_mut(next).unwrap()) -= 1;
            if in_degrees[next] == 0 {
                q.push_back(next);
            }
        }
    }

    (sorted, values)
}

fn get_numbers(values: &HashMap<&str, bool>, start: &str) -> u64 {
    let mut zs = values
        .keys()
        .filter(|key| key.starts_with(start))
        .collect::<Vec<_>>();
    zs.sort_by(|a, b| b.cmp(a)); // reverse order
    let mut number = 0;
    for z in zs {
        number = (number << 1) + (values[z] as u64);
    }

    number
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let graph = parse(&content);
    let (_, values) = topo_sort(&graph);
    // println!("{:?}", sorted);
    // println!("{:?}", values);
    println!("{:?}", get_numbers(&values, "x"));
    println!("{:?}", get_numbers(&values, "y"));
    println!(
        "{:?}",
        get_numbers(&values, "x") + get_numbers(&values, "y")
    );
    println!("{:?}", get_numbers(&values, "z"));
}
