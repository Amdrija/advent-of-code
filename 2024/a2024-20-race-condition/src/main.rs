use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let grid = content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| {
                    if ch == 'S' {
                        start = (i, j);
                        '.'
                    } else if ch == 'E' {
                        end = (i, j);
                        '.'
                    } else {
                        ch
                    }
                })
                .collect()
        })
        .collect();

    (grid, start, end)
}

struct PathCell {
    distance_to_end: usize,
    i: usize,
    j: usize,
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<PathCell> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut paths = vec![vec![(usize::MAX, usize::MAX); grid[0].len()]; grid.len()];
    let mut q = VecDeque::new();
    visited[start.0][start.1] = true;
    q.push_back((0, start.0, start.1));
    while let Some((length, i, j)) = q.pop_front() {
        if (i, j) == end {
            break;
        }

        for (next_i, next_j) in vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            if grid[next_i][next_j] == '.' && !visited[next_i][next_j] {
                q.push_back((length + 1, next_i, next_j));
                visited[next_i][next_j] = true;
                paths[next_i][next_j] = (i, j);
            }
        }
    }

    construct_paths(&paths, end, 0, start)
}

fn construct_paths(
    paths: &Vec<Vec<(usize, usize)>>,
    current: (usize, usize),
    length: usize,
    start: (usize, usize),
) -> Vec<PathCell> {
    if current == start {
        return vec![PathCell {
            distance_to_end: length,
            i: current.0,
            j: current.1,
        }];
    }

    let mut path_before = construct_paths(paths, paths[current.0][current.1], length + 1, start);
    path_before.push(PathCell {
        distance_to_end: length,
        i: current.0,
        j: current.1,
    });

    path_before
}

fn cheat(
    grid: &Vec<Vec<char>>,
    cells_on_path: &HashMap<(usize, usize), usize>,
    current: (usize, usize),
    max_cheats: usize,
    cheat_threshold: usize,
) -> (usize, usize) {
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    visited.insert((current.0, current.1), max_cheats);
    q.push_back((max_cheats, current.0, current.1));
    let current_distance_to_end = cells_on_path[&current];
    let mut savings = 0;
    let mut count = 0;
    while let Some((cheats, i, j)) = q.pop_front() {
        if let Some(distance_to_end) = cells_on_path.get(&(i, j)) {
            if current_distance_to_end >= *distance_to_end + max_cheats - cheats {
                savings = max(
                    savings,
                    current_distance_to_end - (distance_to_end + max_cheats - cheats),
                );
                if current_distance_to_end - (distance_to_end + max_cheats - cheats)
                    >= cheat_threshold
                {
                    count += 1;
                }
            }
        }

        if cheats == 0 {
            continue;
        }

        for (di, dj) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_i = i as i64 + di;
            let new_j = j as i64 + dj;
            if new_i < 0
                || new_i as usize >= grid.len()
                || new_j < 0
                || new_j as usize >= grid.len()
            {
                continue;
            }

            let next = (new_i as usize, new_j as usize);
            if !visited.contains_key(&next) || visited[&next] < cheats - 1 {
                q.push_back((cheats - 1, next.0, next.1));
                *visited.entry(next).or_default() = cheats - 1;
            }
        }
    }

    (savings, count)
}

fn solve(
    grid: &Vec<Vec<char>>,
    paths: &Vec<PathCell>,
    max_cheats: usize,
    cheat_threshold: usize,
) -> usize {
    let cells_on_path = paths
        .iter()
        .map(|path| ((path.i, path.j), path.distance_to_end))
        .collect();
    paths
        .iter()
        .map(|path| {
            cheat(
                grid,
                &cells_on_path,
                (path.i, path.j),
                max_cheats,
                cheat_threshold,
            )
            .1
        })
        .sum()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let (grid, start, end) = parse(&content);
    let paths = bfs(&grid, start, end);
    println!("{}", solve(&grid, &paths, 20, 100));
}
