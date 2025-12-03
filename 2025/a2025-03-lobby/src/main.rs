use std::{fs::File, io::Read, time::Instant};

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let start = Instant::now();
    let joltages = input.lines().map(|battery_bank| {
        let (index, first_digit) = battery_bank[..battery_bank.len() - 1]
            .char_indices()
            .max_by(|(i1, c1), (i2, c2)| match c1.cmp(c2) {
                std::cmp::Ordering::Equal => i2.cmp(i1),
                ordering => ordering,
            })
            .unwrap();
        let second_digit = battery_bank[index + 1..].chars().max().unwrap();
        return (first_digit as u64 - '0' as u64) * 10 + second_digit as u64 - '0' as u64;
    });
    let end = Instant::now();

    println!(
        "{}, time: {}ns",
        joltages.sum::<u64>(),
        end.duration_since(start).as_nanos()
    );

    let start = Instant::now();
    let joltages = input.lines().map(|battery_bank| {
        let mut start_index = 0;
        let mut joltage = 0;
        for end_index in (0..=11).rev() {
            let (index, first_digit) = battery_bank[start_index..battery_bank.len() - end_index]
                .char_indices()
                .max_by(|(i1, c1), (i2, c2)| match c1.cmp(c2) {
                    std::cmp::Ordering::Equal => i2.cmp(i1),
                    ordering => ordering,
                })
                .unwrap();
            start_index += index + 1;
            joltage = joltage * 10 + first_digit as u64 - '0' as u64;
        }
        return joltage;
    });
    let end = Instant::now();

    println!(
        "{}, time: {}ns",
        joltages.sum::<u64>(),
        end.duration_since(start).as_nanos()
    );
}
