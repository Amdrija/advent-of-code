use std::{collections::VecDeque, fs::File, io::Read};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    PaperRoll,
    Empty,
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '@' => Cell::PaperRoll,
                    '.' => Cell::Empty,
                    _ => panic!("unknown character"),
                })
                .collect()
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut cells = parse(&input);
    let mut adjacent_rolls = vec![vec![0i32; cells[0].len()]; cells.len()];
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            if cells[i][j] == Cell::PaperRoll {
                let i = i as i32;
                let j = j as i32;
                let delta_i = [i - 1, i, i + 1];
                let delta_j = [j - 1, j, j + 1];
                adjacent_rolls[i as usize][j as usize] = delta_i
                    .into_iter()
                    .flat_map(|i| delta_j.clone().into_iter().map(move |j| (i, j)))
                    .filter(|(new_i, new_j)| {
                        *new_i >= 0
                            && *new_i < cells.len() as i32
                            && *new_j >= 0
                            && *new_j < cells[0].len() as i32
                            && cells
                                .get(*new_i as usize)
                                .unwrap()
                                .get(*new_j as usize)
                                .unwrap()
                                == &Cell::PaperRoll
                    })
                    .count() as i32;
            }
        }
    }

    println!(
        "{}",
        adjacent_rolls
            .iter()
            .map(|row| row.iter().filter(|count| **count < 5 && **count > 0).count())
            .sum::<usize>()
    );

    let mut q = VecDeque::new();
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            if cells[i][j] == Cell::PaperRoll && adjacent_rolls[i][j] < 5{
                q.push_back((i, j));
                cells[i][j] = Cell::Empty;
            }
        }
    }

    let mut removed = 0;
    while let Some((i, j)) = q.pop_front() {
        removed += 1;
        let i = i as i32;
        let j = j as i32;
        let delta_i = [i - 1, i, i + 1];
        let delta_j = [j - 1, j, j + 1];
        let candidates  = delta_i
            .into_iter()
            .flat_map(|i| delta_j.clone().into_iter().map(move |j| (i, j)))
            .filter(|(new_i, new_j)| {
                *new_i >= 0
                    && *new_i < cells.len() as i32
                    && *new_j >= 0
                    && *new_j < cells[0].len() as i32
                    && cells
                        .get(*new_i as usize)
                        .unwrap()
                        .get(*new_j as usize)
                        .unwrap()
                        == &Cell::PaperRoll
            }).collect::<Vec<_>>();
        for (next_i, next_j) in candidates {
            adjacent_rolls[next_i as usize][next_j as usize] -= 1;
            if adjacent_rolls[next_i as usize][next_j as usize] < 5 {
                q.push_back((next_i as usize, next_j as usize));
                cells[next_i as usize][next_j as usize] = Cell::Empty;
            }
        }
    }

    println!("{}", removed);
}
