use std::{fs::File, io::Read, ops::RangeInclusive};

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .split(",")
        .map(|range| {
            let dash_index = range.find('-').unwrap();
            let start = range[..dash_index].parse().unwrap();
            let end = range[dash_index + 1..].parse().unwrap();
            start..=end
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let ranges = parse(&input);
    let result = ranges
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .filter(|id| {
            let digits = id.ilog10() + 1;
            if digits % 2 == 1 {
                return false;
            }

            let half_mask = 10u64.pow(digits / 2);
            return id / half_mask == id % half_mask;
        })
        .sum::<u64>();
    println!("{}", result);

    let result = ranges
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .filter(|id| {
            let id = id.to_string();
            for sequence_length in 1..=id.len() / 2 {
                if id.len() % sequence_length != 0 {
                    continue;
                }

                let sequence = &id[..sequence_length];
                let mut found = true;
                for start in (sequence_length..id.len()).step_by(sequence_length) {
                    if sequence != &id[start..start + sequence_length] {
                        found = false;
                        break;
                    }
                }

                if found {
                    return true;
                }
            }

            false
        })
        .sum::<u64>();
    println!("{}", result);
}
