use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let mut splitted = content.split("\n\n");
    let page_before_relations = splitted.next().unwrap();
    let manuals = splitted.next().unwrap();

    (
        parse_page_before(page_before_relations),
        parse_manuals(manuals),
    )
}

fn parse_page_before(page_before_relations: &str) -> HashMap<u64, Vec<u64>> {
    page_before_relations
        .lines()
        .map(|line| {
            let mut splitted = line.split("|");
            (
                splitted.next().unwrap().parse().unwrap(),
                splitted.next().unwrap().parse().unwrap(),
            )
        })
        .fold(HashMap::new(), |mut rules, (before, after)| {
            rules.entry(before).or_insert(Vec::new()).push(after);
            rules
        })
}

fn parse_manuals(manuals: &str) -> Vec<Vec<u64>> {
    manuals
        .lines()
        .map(|line| {
            line.split(",")
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(rules: &HashMap<u64, Vec<u64>>, manuals: &Vec<Vec<u64>>) -> u64 {
    manuals
        .iter()
        .filter(|manual| is_correct(rules, manual))
        .map(|manual| manual[manual.len() / 2])
        .sum()
}

fn is_correct(rules: &HashMap<u64, Vec<u64>>, manual: &Vec<u64>) -> bool {
    let mut seen = HashSet::new();
    for page in manual {
        for page_after in rules.get(page).unwrap_or(&Vec::new()) {
            if seen.contains(page_after) {
                return false;
            }
        }
        seen.insert(page);
    }

    true
}

fn part_two(rules: &HashMap<u64, Vec<u64>>, manuals: &Vec<Vec<u64>>) -> u64 {
    manuals
        .iter()
        .filter(|manual| !is_correct(rules, manual))
        .map(|manual| order_correctly(rules, manual))
        .map(|manual| manual[manual.len() / 2])
        .sum()
}

fn order_correctly(rules: &HashMap<u64, Vec<u64>>, manual: &Vec<u64>) -> Vec<u64> {
    let mut sub_graph = HashMap::new();
    let mut in_degrees = manual
        .iter()
        .zip(vec![0; manual.len()])
        .collect::<HashMap<_, _>>();

    for page in manual {
        sub_graph.insert(*page, Vec::new());
        for after in rules.get(page).unwrap_or(&Vec::new()) {
            if let Some(entry) = in_degrees.get_mut(after) {
                *entry += 1;
                sub_graph.get_mut(page).unwrap().push(*after);
            }
        }
    }

    let mut q = VecDeque::new();
    for (page, in_degree) in &in_degrees {
        if *in_degree == 0 {
            q.push_back(**page);
        }
    }

    let mut ordered_manual = Vec::new();
    while let Some(current) = q.pop_front() {
        for after in &sub_graph[&current] {
            let in_degree = in_degrees.get_mut(after).unwrap();
            *in_degree -= 1;
            if *in_degree == 0 {
                q.push_back(*after);
            }
        }
        ordered_manual.push(current);
    }

    ordered_manual
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (rules, manuals) = parse(&content);
    println!("{}", part_one(&rules, &manuals));
    println!("{}", part_two(&rules, &manuals));
}
