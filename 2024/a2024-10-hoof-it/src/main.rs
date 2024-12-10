use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> Vec<Vec<u8>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn dfs_score(
    grid: &Vec<Vec<u8>>,
    i: usize,
    j: usize,
    mut visited_nines: &mut HashSet<(usize, usize)>,
) -> u64 {
    if grid[i][j] == 9 {
        if visited_nines.contains(&(i, j)) {
            return 0;
        }
        visited_nines.insert((i, j));
        return 1;
    }

    let delta = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut score = 0;
    for (delta_i, delta_j) in delta {
        let new_i = (i as i64) + delta_i;
        let new_j = (j as i64) + delta_j;
        if new_i >= 0
            && (new_i as usize) < grid.len()
            && new_j >= 0
            && (new_j as usize) < grid[0].len()
            && grid[new_i as usize][new_j as usize] == grid[i][j] + 1
        {
            score += dfs_score(grid, new_i as usize, new_j as usize, &mut visited_nines);
        }
    }

    score
}

fn part_one(grid: &Vec<Vec<u8>>) -> u64 {
    let mut total_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 0 {
                total_score += dfs_score(grid, i, j, &mut HashSet::new());
            }
        }
    }

    total_score
}

fn dfs_rating(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> u64 {
    if grid[i][j] == 9 {
        return 1;
    }

    let delta = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut score = 0;
    for (delta_i, delta_j) in delta {
        let new_i = (i as i64) + delta_i;
        let new_j = (j as i64) + delta_j;
        if new_i >= 0
            && (new_i as usize) < grid.len()
            && new_j >= 0
            && (new_j as usize) < grid[0].len()
            && grid[new_i as usize][new_j as usize] == grid[i][j] + 1
        {
            score += dfs_rating(grid, new_i as usize, new_j as usize);
        }
    }

    score
}

fn part_two(grid: &Vec<Vec<u8>>) -> u64 {
    let mut total_score = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 0 {
                total_score += dfs_rating(grid, i, j);
            }
        }
    }

    total_score
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let grid = parse(&content);
    println!("{}", part_one(&grid));
    println!("{}", part_two(&grid));
}
