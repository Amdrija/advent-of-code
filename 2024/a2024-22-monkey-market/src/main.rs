use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

fn parse(content: &str) -> Vec<u64> {
    content.lines().map(|line| line.parse().unwrap()).collect()
}

fn next(number: u64) -> u64 {
    let mut next = ((number * 64) ^ number) % 16777216;
    next = ((next / 32) ^ next) % 16777216;
    next = ((next * 2048) ^ next) % 16777216;
    next
}

fn part_one(seeds: &Vec<u64>) -> u64 {
    seeds
        .iter()
        .map(|seed| {
            let mut number = *seed;
            for _ in 0..2000 {
                number = next(number);
            }
            number
        })
        .sum()
}

fn generate_prices(seed: u64, count: usize) -> Vec<u64> {
    let mut number = seed;
    let mut prices = vec![seed % 10];
    for _ in 0..count {
        number = next(number);
        prices.push(number % 10);
    }
    prices
}

fn get_changes(prices: &Vec<u64>) -> Vec<i64> {
    prices[0..prices.len() - 1]
        .iter()
        .zip(prices[1..prices.len()].iter())
        .map(|(before, after)| *after as i64 - *before as i64)
        .collect()
}

fn get_change_indexes(changes: &[i64]) -> HashMap<[i64; 4], usize> {
    let mut sequence = [changes[0], changes[1], changes[2], changes[3]];
    let mut sequences = HashMap::new();
    sequences.insert(sequence.clone(), 4);
    for i in 4..changes.len() {
        sequence[0] = sequence[1];
        sequence[1] = sequence[2];
        sequence[2] = sequence[3];
        sequence[3] = changes[i];
        if let Entry::Vacant(entry) = sequences.entry(sequence.clone()) {
            entry.insert(i + 1);
        }
    }

    sequences
}

fn part_two(seeds: &Vec<u64>) -> u64 {
    let prices_per_seed = seeds
        .iter()
        .map(|seed| generate_prices(*seed, 2000))
        .collect::<Vec<_>>();

    let changes_per_seed = prices_per_seed
        .iter()
        .map(|prices| get_changes(prices))
        .collect::<Vec<Vec<i64>>>();
    let change_indexes_per_seed = changes_per_seed
        .iter()
        .map(|changes| get_change_indexes(changes))
        .collect::<Vec<_>>();
    let mut needle = [0; 4];
    let mut max_bananas = 0;
    for c1 in -9..10 {
        println!("{c1}");
        needle[0] = c1;
        for c2 in -9..10 {
            needle[1] = c2;
            for c3 in -9..10 {
                needle[2] = c3;
                for c4 in -9..10 {
                    needle[3] = c4;
                    let total_bananas = changes_per_seed
                        .iter()
                        .enumerate()
                        .filter_map(|(i, changes)| {
                            change_indexes_per_seed[i]
                                .get(&needle)
                                .map(|index| prices_per_seed[i][*index])
                        })
                        .sum();
                    max_bananas = max(total_bananas, max_bananas)
                }
            }
        }
    }
    max_bananas
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let seeds = parse(&content);
    println!("{}", part_one(&seeds));
    let before = SystemTime::now();
    let result = part_two(&seeds);
    let time = SystemTime::now().duration_since(before).unwrap();
    println!("{}, {}s", result, time.as_secs_f32());
}

#[cfg(test)]
mod tests {
    use crate::{generate_prices, get_change_indexes, get_changes, next};

    #[test]
    fn test_next() {
        let mut number = 123;
        let next_secret_numbers = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for next_secret_number in next_secret_numbers {
            number = next(number);
            assert_eq!(number, next_secret_number)
        }
    }

    #[test]
    fn test_changes() {
        let number = 123;
        let prices = generate_prices(number, 9);
        let changes = get_changes(&prices);
        assert_eq!(changes, vec![-3, 6, -1, -1, 0, 2, -2, 0, -2])
    }

    #[test]
    fn test_get_change_index() {
        let number = 123;
        let prices = generate_prices(number, 9);
        let changes = get_changes(&prices);
        let change_indexes = get_change_indexes(&changes);
        assert_eq!(change_indexes.get(&[-1, -1, 0, 2]), Some(&6));
        assert_eq!(change_indexes.get(&[9, -1, 0, 2]), None);
        assert_eq!(change_indexes.get(&[2, -2, 0, -2]), Some(&9));
    }
}
