use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse(content: &str) -> (Vec<Vec<char>>, usize, usize, Direction) {
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '#' && grid[i][j] != '.' {
                let direction = char_to_direction(grid[i][j]);
                return (grid, i, j, direction);
            }
        }
    }

    (grid, 0, 0, Direction::Up)
}

fn char_to_direction(ch: char) -> Direction {
    match ch {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("Unknown character {}", ch),
    }
}

fn visit_grid(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    direction: Direction,
    visited: &mut HashSet<(usize, usize, Direction)>,
) -> bool {
    if visited.contains(&(i, j, direction)) {
        return true;
    }

    visited.insert((i, j, direction));

    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid.len() - 1 {
        return false;
    }

    let (next_i, next_j, next_direction) = match direction {
        Direction::Up => {
            if grid[i - 1][j] == '#' {
                (i, j, Direction::Right)
            } else {
                (i - 1, j, Direction::Up)
            }
        }
        Direction::Right => {
            if grid[i][j + 1] == '#' {
                (i, j, Direction::Down)
            } else {
                (i, j + 1, Direction::Right)
            }
        }
        Direction::Down => {
            if grid[i + 1][j] == '#' {
                (i, j, Direction::Left)
            } else {
                (i + 1, j, Direction::Down)
            }
        }
        Direction::Left => {
            if grid[i][j - 1] == '#' {
                (i, j, Direction::Up)
            } else {
                (i, j - 1, Direction::Left)
            }
        }
    };

    visit_grid(grid, next_i, next_j, next_direction, visited)
}

fn part_one(
    grid: &Vec<Vec<char>>,
    start_i: usize,
    start_j: usize,
    direction: Direction,
) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    visit_grid(grid, start_i, start_j, direction, &mut visited);
    visited
        .into_iter()
        .map(|(i, j, _)| (i, j))
        .collect::<HashSet<_>>()
}

fn part_two(
    grid: &mut Vec<Vec<char>>,
    start_i: usize,
    start_j: usize,
    direction: Direction,
    candidates: HashSet<(usize, usize)>,
) -> usize {
    let mut count = 0;

    for (i, j) in candidates {
        if grid[i][j] == '.' {
            grid[i][j] = '#';
            if visit_grid(&grid, start_i, start_j, direction, &mut HashSet::new()) {
                count += 1;
            }
            grid[i][j] = '.';
        }
    }

    count
}

fn to_string(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| String::from_utf8(row.iter().map(|ch| *ch as u8).collect()).unwrap())
        .fold(String::new(), |str, row| str + &row + "\n")
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (mut grid, start_i, start_j, direction) = parse(&content);
    let visited = part_one(&grid, start_i, start_j, direction);
    println!("{}", visited.len());

    let before = SystemTime::now();
    let part2 = part_two(&mut grid, start_i, start_j, direction, visited);
    println!(
        "{part2} time {}s",
        SystemTime::now()
            .duration_since(before)
            .unwrap()
            .as_secs_f32()
    );

    let visited = (0..grid.len())
        .flat_map(|i| vec![i; grid[i].len()].into_iter().zip(0..grid[i].len()))
        .collect::<HashSet<_>>();
    let before = SystemTime::now();
    let part2 = part_two(&mut grid, start_i, start_j, direction, visited);
    println!(
        "{part2} time {}s",
        SystemTime::now()
            .duration_since(before)
            .unwrap()
            .as_secs_f32()
    );
}
