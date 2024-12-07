use std::{fs::File, io::Read};

fn parse(content: &str) -> Vec<(u64, Vec<u64>)> {
    content
        .lines()
        .map(|line| {
            let mut splitted = line.split(": ");
            let test_value = splitted.next().unwrap().parse().unwrap();
            let operands = splitted
                .next()
                .unwrap()
                .split(" ")
                .map(|operand| operand.parse().unwrap())
                .collect();

            (test_value, operands)
        })
        .collect()
}

fn solve(
    equations: &[(u64, Vec<u64>)],
    is_combineable: &dyn Fn(u64, &[u64], u64, usize) -> bool,
) -> u64 {
    equations
        .iter()
        .filter(|(target, operands)| is_combineable(*target, operands, operands[0], 1))
        .map(|(target, _)| target)
        .sum()
}

fn is_combinable(
    target: u64,
    operands: &[u64],
    current_result: u64,
    current_operand: usize,
) -> bool {
    if current_operand == operands.len() {
        return current_result == target;
    }

    is_combinable(
        target,
        operands,
        current_result + operands[current_operand],
        current_operand + 1,
    ) || is_combinable(
        target,
        operands,
        current_result * operands[current_operand],
        current_operand + 1,
    )
}

fn is_combinable2(
    target: u64,
    operands: &[u64],
    current_result: u64,
    current_operand: usize,
) -> bool {
    if current_operand == operands.len() {
        return current_result == target;
    }

    is_combinable2(
        target,
        operands,
        current_result + operands[current_operand],
        current_operand + 1,
    ) || is_combinable2(
        target,
        operands,
        current_result * operands[current_operand],
        current_operand + 1,
    ) || is_combinable2(
        target,
        operands,
        fuse_numbers(current_result, operands[current_operand]),
        current_operand + 1,
    )
}

fn fuse_numbers(mut left: u64, right: u64) -> u64 {
    let mut right_copy = right;
    while right_copy > 0 {
        left = left * 10;
        right_copy /= 10;
    }

    left + right
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let equations = parse(&content);
    println!("{}", solve(&equations, &is_combinable));
    println!("{}", solve(&equations, &is_combinable2));
}
