use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
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

fn dijkstra(grid: &Vec<Vec<char>>, start: &Coordinates, end: &Coordinates) -> u64 {
    let mut min_distance = vec![vec![vec![u64::MAX; 4]; grid[0].len()]; grid.len()];
    let mut pq = BinaryHeap::new();
    min_distance[start.row][start.col][Direction::Right as usize] = 0;
    pq.push(State::new(0, start.clone(), Direction::Right));

    while let Some(state) = pq.pop() {
        if state.score > min_distance[state.coords.row][state.coords.col][state.direction as usize]
        {
            continue;
        }

        for next_direction in state.direction.next_directions() {
            let next_coords = next_direction.next_coord(&state.coords);
            let next_score = state.score + state.direction.rotation_score(&next_direction) + 1;
            if grid[next_coords.row][next_coords.col] == '.'
                && next_score
                    <= min_distance[next_coords.row][next_coords.col][next_direction as usize]
            {
                min_distance[next_coords.row][next_coords.col][next_direction as usize] =
                    next_score;
                pq.push(State::new(next_score, next_coords, next_direction));
            }
        }
    }

    *min_distance[end.row][end.col].iter().min().unwrap()
}

fn main() {
    let mut content = String::new();
    File::open("test1")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (grid, start, end) = parse(&content);
    let min_distance = dijkstra(&grid, &start, &end);
    println!("{}", min_distance);
}
