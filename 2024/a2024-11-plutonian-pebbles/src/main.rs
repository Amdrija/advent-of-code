use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> Vec<u64> {
    content
        .split(" ")
        .map(|stone| stone.parse().unwrap())
        .collect()
}

fn digit_count(mut stone: u64) -> u32 {
    let mut count = 0;
    while stone > 0 {
        stone /= 10;
        count += 1;
    }

    count
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::with_capacity(stones.len());
    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
        } else {
            let digits = digit_count(*stone);
            if digits % 2 == 0 {
                let half_digits = 10u64.pow(digits / 2);
                let first = stone / half_digits;
                let second = stone % half_digits;
                new_stones.push(first);
                new_stones.push(second);
            } else {
                let first = stone * 2024;
                new_stones.push(first);
            }
        }
    }
    new_stones
}

fn blink_recursive(
    stone: u64,
    times: usize,
    mut cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if let Some(stone_count) = cache.get(&(stone, times)) {
        return *stone_count;
    }

    if times == 0 {
        return 1;
    }

    let stone_count = if stone == 0 {
        blink_recursive(1, times - 1, &mut cache)
    } else {
        let digits = digit_count(stone);
        if digits % 2 == 0 {
            let half_digits = 10u64.pow(digits / 2);
            let left_half = stone / half_digits;
            let right_half = stone % half_digits;
            blink_recursive(left_half, times - 1, &mut cache)
                + blink_recursive(right_half, times - 1, &mut cache)
        } else {
            blink_recursive(stone * 2024, times - 1, &mut cache)
        }
    };

    cache.insert((stone, times), stone_count);
    stone_count
}

fn part_one(stones: &Vec<u64>, times: usize) -> usize {
    let mut current_stones = stones.clone();
    for i in 0..times {
        if i % 5 == 0 {
            println!("{i}");
        }
        current_stones = blink(&current_stones);
    }

    current_stones.len()
}

fn part_two(stones: &Vec<u64>, times: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|stone| blink_recursive(*stone, times, &mut cache))
        .sum()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let stones = parse(&content);
    println!("{:?}", part_one(&stones, 25));
    println!("{:?}", part_two(&stones, 75));
}
