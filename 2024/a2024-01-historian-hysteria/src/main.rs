use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn parse(content: String) -> (Vec<i64>, Vec<i64>) {
    content
        .lines()
        .into_iter()
        .map(|line| {
            let numbers = line
                .split("   ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (numbers[0], numbers[1])
        })
        .fold(
            (Vec::new(), Vec::new()),
            |(mut list1, mut list2), (location1, location2)| {
                list1.push(location1);
                list2.push(location2);
                (list1, list2)
            },
        )
}

fn part_one(list1: &mut Vec<i64>, list2: &mut Vec<i64>) -> i64 {
    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2.iter())
        .map(|(location1, location2)| (location1 - location2).abs())
        .sum::<i64>()
}

fn part_two(list1: &Vec<i64>, list2: &Vec<i64>) -> i64 {
    let list2_counts = list2.iter().fold(HashMap::new(), |mut counts, location| {
        let entry = counts.entry(location).or_default();
        *entry += 1;
        counts
    });

    list1
        .iter()
        .map(|location| location * list2_counts.get(location).unwrap_or(&0))
        .sum()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let (mut list1, mut list2) = parse(content);

    let total_distance = part_one(&mut list1, &mut list2);
    println!("{total_distance}");

    let similarity_score = part_two(&list1, &list2);
    println!("{similarity_score}");
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_one, part_two};
    const LIST1: [i64; 6] = [3, 4, 2, 1, 3, 3];
    const LIST2: [i64; 6] = [4, 3, 5, 3, 9, 3];
    #[test]
    fn test_parse() {
        let content = String::from(
            r#"3   4
4   3
2   5
1   3
3   9
3   3"#,
        );
        let (list1, list2) = parse(content);

        assert_eq!(list1, LIST1.to_vec());
        assert_eq!(list2, LIST2.to_vec());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&mut LIST1.to_vec(), &mut LIST2.to_vec()), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&LIST1.to_vec(), &LIST2.to_vec()), 31);
    }
}
