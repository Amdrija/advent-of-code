use std::{fs::File, io::Read};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multipy,
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn calculate(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Multipy => self.numbers.iter().product(),
        }
    }
}

fn parse(input: &str) -> Vec<Problem> {
    let numbers = input
        .lines()
        .take(input.lines().count() - 1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    
    let problems = numbers[0].len();
    let numbers = numbers.into_iter().fold(vec![vec![]; problems], |mut numbers, row| {
        row.into_iter().enumerate().for_each(|(i, number)| numbers[i].push(number));
        numbers
    });

    let operations = input.lines().last().unwrap().split_ascii_whitespace().map(|operation| match operation {
        "+" => Operation::Add,
        "*" => Operation::Multipy,
        s => panic!("Unknown operation: {}", s)
    });

    numbers.into_iter().zip(operations).map(|(problem_numbers, operation)| Problem {
        numbers: problem_numbers,
        operation
    }).collect()
}

fn parse2(input: &str) -> Vec<Problem> {
    let lines = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let m = lines[0].len();
    let n = lines.len();

    let mut problems = Vec::new();
    let mut col = (m - 1) as isize;
    while col >= 0 {
        let mut all_whitespace = false;
        let mut problem = Problem {
            numbers: vec![],
            operation: Operation::Add
        };

        while !all_whitespace && col >= 0 {
            all_whitespace = true;
            let mut number = 0;
            for row in 0..n {
                if lines[row][col as usize].is_numeric() {
                    all_whitespace = false;
                    number = number * 10 + (lines[row][col as usize] as u8 - '0' as u8) as u64;
                } else if lines[row][col as usize] == '*' {
                    all_whitespace = false;
                    problem.operation = Operation::Multipy;
                } else if lines[row][col as usize] == '+' {
                    all_whitespace = false;
                    problem.operation = Operation::Add;
                }
            }
            if number != 0 {
                problem.numbers.push(number);
            }
            col -= 1;
        }

        problems.push(problem);
        // col -= 1;
    }

    problems
}

fn main() {
    let mut input: String = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();
    let problems = parse(&input);
    let result = problems.iter().map(Problem::calculate).sum::<u64>();
    println!("{}", result);

    let problems = parse2(&input);
    let result = problems.iter().map(Problem::calculate).sum::<u64>();
    println!("{}", result);
}
