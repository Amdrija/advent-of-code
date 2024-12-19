use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Patterns {
    end: bool,
    map: HashMap<char, Patterns>,
}

impl Patterns {
    fn possible_ways(&self, design: &Vec<char>, patterns: &Patterns) -> usize {
        // Contain the number of possible ways to get the design from the ith character
        let mut possible_from = vec![0; design.len()];
        for (i, _) in design.iter().enumerate().rev() {
            let mut q = VecDeque::new();
            q.push_back((patterns, i));
            while let Some((current_patterns, current)) = q.pop_front() {
                // If the index of the current character is out of bound,
                // Then see if the current trie is an ending of some pattern
                // If it is, add 1 to the possible_form[i], since the ending cannot be combined
                // with other patterns.
                if current == design.len() {
                    if current_patterns.end {
                        possible_from[i] += 1;
                    }
                    continue;
                }

                // If the current tree is an ending of some pattern, then we can split the
                // design at the current character and make it with however many ways we can
                // make the design from the current character
                if current_patterns.end {
                    possible_from[i] += possible_from[current];
                }

                // Now, if we can find patterns which can be continued by the current character
                // We should also perform the same calculation on this continuation.
                if let Some(pattern) = current_patterns.map.get(&design[current]) {
                    q.push_back((pattern, current + 1));
                }
            }
        }

        possible_from[0]
    }
}

impl From<&str> for Patterns {
    fn from(value: &str) -> Self {
        let mut patterns = Self {
            end: false,
            map: HashMap::new(),
        };

        for pattern in value.split(", ") {
            let mut current_pattern = &mut patterns;
            for ch in pattern.chars() {
                current_pattern = current_pattern.map.entry(ch).or_insert(Self {
                    end: false,
                    map: HashMap::new(),
                });
            }
            current_pattern.end = true;
        }

        patterns
    }
}

fn parse(content: &str) -> (Patterns, Vec<String>) {
    let mut splitted = content.split("\n\n");
    let patterns = Patterns::from(splitted.next().unwrap());

    (
        patterns,
        splitted
            .next()
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect(),
    )
}

fn part_one(patterns: &Patterns, designs: &Vec<String>) -> usize {
    let count = designs
        .iter()
        .filter(|design| patterns.possible_ways(&design.chars().collect(), patterns) > 0)
        .count();

    count
}

fn part_two(patterns: &Patterns, designs: &Vec<String>) -> usize {
    let sum = designs
        .iter()
        .map(|design| patterns.possible_ways(&design.chars().collect(), patterns))
        .sum();

    sum
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (patterns, designs) = parse(&content);
    println!("{}", part_one(&patterns, &designs));
    println!("{}", part_two(&patterns, &designs));
}
