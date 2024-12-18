use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> Vec<(usize, usize)> {
    content
        .lines()
        .map(|line| {
            let coords = line
                .split(",")
                .map(|coord| coord.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (coords[0], coords[1])
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Cell {
    Free,
    Corrupted,
}

fn simulate(grid: &mut Vec<Vec<Cell>>, blocks: &[(usize, usize)]) {
    for (x, y) in blocks {
        grid[*y][*x] = Cell::Corrupted
    }
}

fn bfs(grid: &Vec<Vec<Cell>>) -> Option<(usize, Vec<(usize, usize)>)> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut q = VecDeque::new();
    q.push_back((0, 1, 1));
    visited[1][1] = true;
    let mut paths = vec![vec![None; grid[0].len()]; grid.len()];
    while let Some((length, x, y)) = q.pop_front() {
        if x == grid[0].len() - 2 && y == grid.len() - 2 {
            return Some((length, calculate_path(&paths, x, y)));
        }

        for (new_x, new_y) in vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if grid[new_y][new_x] == Cell::Free && !visited[new_y][new_x] {
                visited[new_y][new_x] = true;
                q.push_back((length + 1, new_x, new_y));
                paths[new_y][new_x] = Some((x, y));
            }
        }
    }

    None
}

fn calculate_path(
    paths: &Vec<Vec<Option<(usize, usize)>>>,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    if x == 1 && y == 1 {
        return vec![(1, 1)];
    }
    let mut path = match paths[y][x] {
        None => panic!("Calculating a path of an unreachable cell: ({},{})", x, y),
        Some((prev_x, prev_y)) => calculate_path(paths, prev_x, prev_y),
    };
    path.push((x, y));
    path
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        for cell in row {
            match cell {
                Cell::Free => print!("."),
                Cell::Corrupted => print!("#"),
            }
        }
        print!("\n");
    }
}

fn part_one(blocks: &Vec<(usize, usize)>, grid_size: usize, blocks_to_simulate: usize) -> usize {
    let mut grid = vec![vec![Cell::Free; grid_size + 2]; grid_size + 2];
    for i in 0..grid_size + 2 {
        grid[0][i] = Cell::Corrupted;
        grid[grid_size + 1][i] = Cell::Corrupted;
        grid[i][0] = Cell::Corrupted;
        grid[i][grid_size + 1] = Cell::Corrupted;
    }
    let new_blocks = blocks
        .iter()
        .map(|(x, y)| (x + 1, y + 1))
        .collect::<Vec<(usize, usize)>>();
    simulate(&mut grid, &new_blocks[..blocks_to_simulate]);
    print_grid(&grid);

    let (length, path) = bfs(&grid).unwrap();
    println!("{:?}", path);

    length
}

fn part_two(
    blocks: &Vec<(usize, usize)>,
    grid_size: usize,
    blocks_to_simulate: usize,
) -> (usize, usize) {
    let mut grid = vec![vec![Cell::Free; grid_size + 2]; grid_size + 2];
    for i in 0..grid_size + 2 {
        grid[0][i] = Cell::Corrupted;
        grid[grid_size + 1][i] = Cell::Corrupted;
        grid[i][0] = Cell::Corrupted;
        grid[i][grid_size + 1] = Cell::Corrupted;
    }
    let new_blocks = blocks
        .iter()
        .map(|(x, y)| (x + 1, y + 1))
        .collect::<Vec<(usize, usize)>>();
    simulate(&mut grid, &new_blocks[..blocks_to_simulate]);
    print_grid(&grid);
    let (_, path) = bfs(&grid).unwrap();
    let mut path_set = path.into_iter().collect::<HashSet<_>>();
    for (x, y) in &new_blocks[blocks_to_simulate..] {
        grid[*y][*x] = Cell::Corrupted;
        if path_set.contains(&(*x, *y)) {
            match bfs(&grid) {
                None => return (*x - 1, *y - 1), // Convert back to non fenced coordinates
                Some((_, path)) => {
                    path_set = path.into_iter().collect();
                }
            }
        }
    }

    (0, 0)
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let blocks = parse(&content);

    println!("{}", part_one(&blocks, 71, 1024));
    println!("{:?}", part_two(&blocks, 71, 1024));
}
