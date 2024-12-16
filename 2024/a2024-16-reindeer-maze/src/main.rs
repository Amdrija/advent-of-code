use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
struct Coordinates {
    row: usize,
    col: usize,
}

impl Coordinates {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn parse(content: &str) -> (Vec<Vec<char>>, Coordinates, Coordinates) {
    let mut start = Coordinates::new(0, 0);
    let mut end = Coordinates::new(0, 0);
    let mut grid = Vec::new();
    for (i, line) in content.lines().enumerate() {
        grid.push(Vec::new());
        for (j, mut ch) in line.chars().enumerate() {
            ch = if ch == 'S' {
                start = Coordinates::new(i, j);
                '.'
            } else if ch == 'E' {
                end = Coordinates::new(i, j);
                '.'
            } else {
                ch
            };
            grid[i].push(ch);
        }
    }

    (grid, start, end)
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn next_coord(&self, coord: &Coordinates) -> Coordinates {
        match self {
            Direction::Up => Coordinates::new(coord.row - 1, coord.col),
            Direction::Right => Coordinates::new(coord.row, coord.col + 1),
            Direction::Down => Coordinates::new(coord.row + 1, coord.col),
            Direction::Left => Coordinates::new(coord.row, coord.col - 1),
        }
    }

    fn next_directions(&self) -> Vec<Direction> {
        match self {
            Direction::Up => vec![Direction::Up, Direction::Right, Direction::Left],
            Direction::Right => vec![Direction::Right, Direction::Up, Direction::Down],
            Direction::Down => vec![Direction::Down, Direction::Right, Direction::Left],
            Direction::Left => vec![Direction::Left, Direction::Up, Direction::Down],
        }
    }

    fn rotation_score(&self, next: &Direction) -> u64 {
        if self == next {
            return 0;
        }

        if self.opposite() == *next {
            return 2000;
        }

        1000
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Right),
            2 => Ok(Direction::Down),
            3 => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    score: u64,
    coords: Coordinates,
    direction: Direction,
}

impl State {
    fn new(score: u64, coords: Coordinates, direction: Direction) -> Self {
        Self {
            score,
            coords,
            direction,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score.cmp(&other.score) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

fn dijkstra(
    grid: &Vec<Vec<char>>,
    start: &Coordinates,
) -> (
    Vec<Vec<Vec<u64>>>,
    Vec<Vec<Vec<HashSet<(Coordinates, usize)>>>>,
) {
    let mut min_distances = vec![vec![vec![u64::MAX; 4]; grid[0].len()]; grid.len()];
    let mut paths = vec![vec![vec![HashSet::new(); 4]; grid[0].len()]; grid.len()];
    let mut pq = BinaryHeap::new();
    min_distances[start.row][start.col][Direction::Right as usize] = 0;
    pq.push(State::new(0, start.clone(), Direction::Right));

    while let Some(state) = pq.pop() {
        for next_direction in state.direction.next_directions() {
            let next_coords = next_direction.next_coord(&state.coords);
            let next_score = state.score + state.direction.rotation_score(&next_direction) + 1;
            if grid[next_coords.row][next_coords.col] == '.'
                && next_score
                    <= min_distances[next_coords.row][next_coords.col][next_direction as usize]
            {
                if next_score
                    < min_distances[next_coords.row][next_coords.col][next_direction as usize]
                {
                    paths[next_coords.row][next_coords.col][next_direction as usize].clear();
                }
                paths[next_coords.row][next_coords.col][next_direction as usize]
                    .insert((state.coords.clone(), state.direction as usize));
                min_distances[next_coords.row][next_coords.col][next_direction as usize] =
                    next_score;
                pq.push(State::new(next_score, next_coords, next_direction));
            }
        }
    }

    (min_distances, paths)
}

fn mark_paths(
    mut grid: &mut Vec<Vec<char>>,
    paths: &Vec<Vec<Vec<HashSet<(Coordinates, usize)>>>>,
    min_distances: &Vec<Vec<Vec<u64>>>,
    coordinates: &Coordinates,
    direction: usize,
    start: &Coordinates,
) {
    grid[coordinates.row][coordinates.col] = 'O';
    if coordinates == start {
        return;
    }

    for (previous_coord, previous_direction) in &paths[coordinates.row][coordinates.col][direction]
    {
        mark_paths(
            grid,
            paths,
            min_distances,
            previous_coord,
            *previous_direction,
            start,
        );
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{cell}");
        }
        print!("\n");
    }
}

fn find_tiles_on_path(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|cell| **cell == 'O').count())
        .sum()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (mut grid, start, end) = parse(&content);
    let (min_distances, paths) = dijkstra(&grid, &start);
    let min_distance = *min_distances[end.row][end.col].iter().min().unwrap();
    println!("{}", min_distance);
    for (direction, _) in min_distances[end.row][end.col]
        .iter()
        .enumerate()
        .filter(|(_, dir)| **dir == min_distance)
    {
        mark_paths(&mut grid, &paths, &min_distances, &end, direction, &start);
    }
    print_grid(&grid);
    println!("{}", find_tiles_on_path(&grid));
}
