use std::{cmp, fs::File, io::Read, ops::RangeInclusive};

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut split_lines = input.split("\n\n");
    let ranges = split_lines
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split_range = line.split("-");
            let start = split_range.next().unwrap().parse().unwrap();
            let end = split_range.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect();

    let ids = split_lines
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, ids)
}

fn merge_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let mut merged_ranges = vec![ranges[0].clone()];
    for range in &ranges[1..] {
        let last_range = merged_ranges.last_mut().unwrap();
        if *last_range.end() + 1 >= *range.start() {
            *last_range = *last_range.start()..=cmp::max(*last_range.end(), *range.end());
        } else {
            merged_ranges.push(range.clone());
        }
    }
    merged_ranges
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let (mut ranges, ids) = parse(&input);
    ranges.sort_by(|a, b| match a.start().cmp(b.start()) {
        std::cmp::Ordering::Equal => a.end().cmp(b.end()),
        comparison_result => comparison_result,
    });

    let merged_ranges = merge_ranges(&ranges);
    let found = ids
        .iter()
        .filter(|id| {
            merged_ranges
                .binary_search_by(|range| {
                    if *id < range.start() {
                        std::cmp::Ordering::Greater
                    } else if *id > range.end() {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
        })
        .count();
    println!("{}", found);

    let found2 = merged_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>();
    println!("{}", found2);
}

#[cfg(test)]
mod tests {
    use crate::merge_ranges;

    #[test]
    fn test_merge() {
        let ranges = [
            0..=5,
            2..=8,
            8..=10,
            11..=14,
            16..=17,
            18..=20,
            30..=40,
            31..=35,
            36..=39,
            36..=39,
            41..=42,
            42..=42,
            50..=60,
            70..=80,
        ];
        let merged_ranges = merge_ranges(&ranges);
        assert_eq!(merged_ranges, [0..=14, 16..=20, 30..=42, 50..=60, 70..=80]);
    }
}
