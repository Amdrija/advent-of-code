use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input").unwrap();

    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut elf_calories = input
        .split("\n\n")
        .map(|elf_input| {
            elf_input
                .split("\n")
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum()
        })
        .collect::<Vec<u64>>();
    elf_calories.sort_by(|a: &u64, b: &u64| b.cmp(a));
    println!("{}", elf_calories[0]);
    println!("{}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}
