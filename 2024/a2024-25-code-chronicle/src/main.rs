use std::fs::File;
use std::io::Read;

fn parse_schematic(schematic: &str) -> (bool, Vec<usize>) {
    let mut lines = schematic.lines().enumerate();
    let first_line = lines.next().unwrap().1;

    let is_lock = first_line == String::from_utf8(vec!['#' as u8; first_line.len()]).unwrap();
    let mut heights = if is_lock {
        vec![0; first_line.len()]
    } else {
        vec![5; first_line.len()]
    };

    for (row, line) in lines {
        for (i, ch) in line.chars().enumerate() {
            if is_lock {
                if ch == '#' {
                    heights[i] = row;
                }
            } else {
                if ch == '.' {
                    heights[i] = 5 - row;
                }
            }
        }
    }

    (is_lock, heights)
}

fn parse(content: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = vec![];
    let mut keys = vec![];
    for schematic in content.split("\n\n") {
        let (is_lock, heights) = parse_schematic(schematic);
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

fn fits(lock: &Vec<usize>, key: &Vec<usize>, total_height: usize) -> bool {
    for i in 0..lock.len() {
        if lock[i] + key[i] > total_height {
            return false;
        }
    }

    true
}

fn part_one(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>) -> usize {
    let mut combinations = 0;
    for lock in locks {
        for key in keys {
            if fits(lock, key, 5) {
                combinations += 1;
            }
        }
    }

    combinations
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let (locks, keys) = parse(&content);
    println!("{}", part_one(&locks, &keys));
}
