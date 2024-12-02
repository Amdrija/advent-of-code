use std::fs::File;
use std::io::Read;

fn parse(content: String) -> Vec<Vec<i64>> {
    content
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_safe(report: &[i64]) -> bool {
    let differences = report
        .get(1..)
        .unwrap()
        .iter()
        .zip(report.get(0..report.len() - 1).unwrap())
        .map(|(current, previous)| current - previous)
        .collect::<Vec<_>>();

    differences.iter().all(|diff| diff.abs() >= 1 && diff.abs() <= 3)
        && (differences.iter().all(|diff| *diff < 0) || differences.iter().all(|diff| *diff > 0))
}

fn is_safe_with_1_unsafe_level(report: &Vec<i64>) -> bool {
    for i in 0..report.len() {
        let mut removed_i = report.clone();
        removed_i.remove(i);
        if is_safe(&removed_i) {
            return true
        }
    }

    false
}

fn part_one(reports: &Vec<Vec<i64>>) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn part_two(reports: &Vec<Vec<i64>>) -> usize {
    reports.iter().filter(|report| is_safe(report) || is_safe_with_1_unsafe_level(report)).count()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let reports = parse(content);
    println!("{}", part_one(&reports));
    println!("{}", part_two(&reports));
}
