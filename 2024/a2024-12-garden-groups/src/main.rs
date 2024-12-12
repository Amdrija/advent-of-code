use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> Vec<Vec<char>> {
    content.lines().map(|line| line.chars().collect()).collect()
}

fn dfs(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    mut visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    visited[i][j] = true;
    let mut perimeter = 0;
    let mut area = 0;
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (di, dj) in directions {
        let new_i = i as i64 + di;
        if new_i < 0 || new_i as usize >= grid.len() {
            perimeter += 1;
            continue;
        }

        let new_j = j as i64 + dj;
        if new_j < 0 || new_j as usize >= grid[0].len() {
            perimeter += 1;
            continue;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;
        if grid[new_i][new_j] == grid[i][j] {
            if !visited[new_i][new_j] {
                let (new_area, new_perimeter) = dfs(grid, new_i, new_j, visited);
                area += new_area;
                perimeter += new_perimeter;
            }
        } else {
            perimeter += 1;
        }
    }

    (area + 1, perimeter)
}

fn part_one(grid: &Vec<Vec<char>>) -> usize {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !visited[i][j] {
                let (area, perimeter) = dfs(&grid, i, j, &mut visited);
                total += area * perimeter;
            }
        }
    }

    total
}

fn pad_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut padded = vec![vec!['7'; grid[0].len() + 2]; 1];
    for i in 0..grid.len() {
        let mut padded_row = vec!['7'];
        padded_row.extend(grid[i].iter().cloned());
        padded_row.push('7');
        padded.push(padded_row);
    }
    padded.push(vec!['7'; grid[0].len() + 2]);
    padded
}

enum Shape {
    Square,
    Rectangle2,
    Rectangle3,
    L,
    T,
    Cross,
}

impl Shape {
    fn next_shape(&self, i: usize, j: usize, neighbours: &HashSet<(usize, usize)>) -> Self {
        match self {
            Shape::Square => Shape::Rectangle2,
            Shape::Rectangle2 => {
                if Self::is_reectangle3(i, j, neighbours) {
                    Shape::Rectangle3
                } else {
                    Shape::L
                }
            }
            Shape::Rectangle3 => Shape::T,
            Shape::L => Shape::T,
            Shape::T => Shape::Cross,
            Shape::Cross => Shape::Cross,
        }
    }

    fn corners(&self, i: usize, j: usize, grid: &Vec<Vec<char>>) -> usize {
        match self {
            Shape::Square => 4,
            Shape::Rectangle2 => 2,
            Shape::Rectangle3 => 0,
            Shape::L => Self::count_L(i, j, grid),
            // A T shape loses 2 corners on the straight side
            Shape::T => Self::count_L(i, j, grid) - 2,
            // A cross loses 4 corners, since the inside corner of 1 L is an outside corner of
            // another L and a cross is just 4 L shapes
            Shape::Cross => Self::count_L(i, j, grid) - 4,
        }
    }

    fn count_L(i: usize, j: usize, grid: &Vec<Vec<char>>) -> usize {
        let corners: Vec<((i64, i64), (i64, i64), (i64, i64))> = vec![
            ((-1, 0), (-1, 1), (0, 1)),
            ((0, 1), (1, 1), (1, 0)),
            ((1, 0), (1, -1), (0, -1)),
            ((0, -1), (-1, -1), (-1, 0)),
        ];
        let mut count = 0;
        for ((di1, dj1), (di2, dj2), (di3, dj3)) in corners {
            let i1 = (i as i64 + di1) as usize;
            let j1 = (j as i64 + dj1) as usize;
            let i2 = (i as i64 + di2) as usize;
            let j2 = (j as i64 + dj2) as usize;
            let i3 = (i as i64 + di3) as usize;
            let j3 = (j as i64 + dj3) as usize;

            if grid[i1][j1] == grid[i][j] && grid[i3][j3] == grid[i][j] {
                count += 1;
                if grid[i2][j2] != grid[i][j] {
                    count += 1;
                }
            }
        }

        count
    }

    fn is_reectangle3(i: usize, j: usize, neighbours: &HashSet<(usize, usize)>) -> bool {
        (neighbours.contains(&(i + 1, j)) && neighbours.contains(&(i - 1, j)))
            || (neighbours.contains(&(i, j - 1)) && neighbours.contains(&(i, j + 1)))
    }
}

fn bfs(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    mut visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    let mut q = VecDeque::new();
    q.push_back((i, j));
    visited[i][j] = true;
    let mut area = 0;
    let mut corners = 0;
    while let Some((i, j)) = q.pop_front() {
        area += 1;
        let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut shape = Shape::Square;
        let mut neighbours = HashSet::new();
        for (di, dj) in directions {
            let new_i = (i as i64 + di) as usize;
            let new_j = (j as i64 + dj) as usize;

            if grid[new_i][new_j] == grid[i][j] {
                if !visited[new_i][new_j] {
                    visited[new_i][new_j] = true;
                    q.push_back((new_i, new_j));
                }
                neighbours.insert((new_i, new_j));
                shape = shape.next_shape(i, j, &neighbours);
            }
        }

        corners += shape.corners(i, j, grid);
    }

    (area, corners)
}

fn part_two(padded_grid: &Vec<Vec<char>>) -> usize {
    let mut visited = vec![vec![false; padded_grid[0].len()]; padded_grid.len()];
    let mut total = 0;
    for i in 1..padded_grid.len() - 1 {
        for j in 1..padded_grid[i].len() - 1 {
            if !visited[i][j] {
                let (area, sides) = bfs(&padded_grid, i, j, &mut visited);
                total += area * sides;
            }
        }
    }

    total
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let grid = parse(&content);
    println!("{}", part_one(&grid));

    let padded_grid = pad_grid(&grid);
    println!("{}", part_two(&padded_grid));
}
