use std::{fs::File, io::Read};

enum Rotation {
    Left(i32),
    Right(i32),
}

fn parse(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| match line.chars().nth(0).unwrap() {
            'L' => Rotation::Left(line[1..].parse().unwrap()),
            'R' => Rotation::Right(line[1..].parse().unwrap()),
            ch => panic!("Unknown character {}", ch),
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let rotations = parse(&input);

    let (_, zeroes) = rotations
        .iter()
        .fold((50, 0), |(position, zeroes), rotation| {
            let new_position = match rotation {
                Rotation::Left(offset) => position - offset,
                Rotation::Right(offset) => position + offset,
            };

            if new_position % 100 == 0 {
                return (0, zeroes + 1);
            }

            (new_position, zeroes)
        });

    println!("{}", zeroes);

    let (_, zeroes) =
        rotations
            .iter()
            .fold((50, 0), |(position, zeroes), rotation| match rotation {
                Rotation::Left(offset) => {
                    let new_position = (position - offset % 100 + 100) % 100;
                    let zeroes = zeroes + offset.abs() / 100;
                    if (new_position > position && position != 0) || new_position == 0 {
                        return (new_position, zeroes + 1);
                    }
                    (new_position, zeroes)
                }
                Rotation::Right(offset) => {
                    let new_position = (position + offset) % 100;
                    let zeroes = zeroes + offset.abs() / 100;
                    if (new_position < position && position != 0) || new_position == 0 {
                        return (new_position, zeroes + 1);
                    }
                    (new_position, zeroes)
                }
            });

    println!("{}", zeroes);
}
