use std::fs::File;
use std::io::Read;
use regex::Regex;

fn part_one(content: &str) -> i64 {
    let regex = Regex::new(r"mul\((?<op1>\d{1,3}),(?<op2>\d{1,3})\)").unwrap();
    let multiplications = regex.captures_iter(content).map(|captured| (captured["op1"].parse::<i64>().unwrap(), captured["op2"].parse::<i64>().unwrap()));

    multiplications.map(|(op1, op2)| op1 * op2).sum()
}


#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(i64, i64)
}

fn part_two(content: &str) -> i64 {
    let regex = Regex::new(r"mul\((?<op1>\d{1,3}),(?<op2>\d{1,3})\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let instructions = regex.captures_iter(content).map(|captured| {
        if captured.name("do").is_some() {
            return Instruction::Do;
        }

        if captured.name("dont").is_some() {
            return Instruction::Dont;
        }

        Instruction::Mul(captured["op1"].parse().unwrap(), captured["op2"].parse().unwrap())
    });

    instructions.fold((0, true), |(sum, mut should_add), instruction| {
        match instruction {
            Instruction::Do => (sum, true),
            Instruction::Dont => (sum, false),
            Instruction::Mul(op1, op2) => if should_add {
                (sum + op1 * op2, true)
            } else {
                (sum, false)
            }
        }
    }).0
}

fn main() {
    let mut content = String::new();
    File::open("input").unwrap().read_to_string(&mut content).unwrap();
    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}
