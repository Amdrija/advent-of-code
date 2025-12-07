use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input").unwrap().read_to_string(&mut input).unwrap();

    let input = input.replace('S', "|");
    let mut map = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut splits = 0;
    let mut timelines = vec![vec![0; map[0].len()]; map.len()];
    timelines[0][map[0].iter().position(|el| *el == '|').unwrap()] = 1;
    for row in 0..map.len() - 1 {
        for col in 0..map[row].len() {
            if map[row][col] == '|' {
                if map[row + 1][col] == '^' {
                    splits += 1;
                    if col > 0 {
                        map[row + 1][col - 1] = '|';
                        timelines[row + 1][col - 1] += timelines[row][col];
                    }
                    if col < map[row].len() {
                        map[row + 1][col + 1] = '|';
                        timelines[row + 1][col + 1] += timelines[row][col];
                    }
                } else {
                    map[row + 1][col] = '|';
                    timelines[row + 1][col] += timelines[row][col];
                }
            }
        }
    }

    println!("{}", splits);

    for row in &timelines {
        println!("{}", row.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" "));
    }
    println!("{}", timelines.last().unwrap().iter().sum::<u64>());
}
