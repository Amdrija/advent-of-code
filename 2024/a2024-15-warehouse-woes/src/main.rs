use std::fs::File;
use std::io::Read;

#[derive(Debug, Copy, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Cell {
    Empty,
    Box,
    Wall,
}

fn parse_grid(content: &str) -> (Vec<Vec<Cell>>, usize, usize) {
    let mut robot_x = 0;
    let mut robot_y = 0;
    let grid = content
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '.' => Cell::Empty,
                    'O' => Cell::Box,
                    '#' => Cell::Wall,
                    '@' => {
                        robot_x = x;
                        robot_y = y;
                        Cell::Empty
                    }
                    _ => panic!("Unknown character {}", ch),
                })
                .collect()
        })
        .collect();

    (grid, robot_x, robot_y)
}

fn parse_moves(content: &str) -> Vec<Move> {
    content
        .lines()
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                '^' => Move::Up,
                '>' => Move::Right,
                'v' => Move::Down,
                '<' => Move::Left,
                _ => panic!("Uknown move character {}", ch),
            })
        })
        .collect()
}

fn parse(content: &str) -> (Vec<Vec<Cell>>, usize, usize, Vec<Move>) {
    let mut splitted = content.split("\n\n");
    let (grid, robot_x, robot_y) = parse_grid(splitted.next().unwrap());
    let moves = parse_moves(splitted.next().unwrap());

    (grid, robot_x, robot_y, moves)
}

fn traverse(mut grid: &mut Vec<Vec<Cell>>, mut x: usize, mut y: usize, moves: &Vec<Move>) {
    for current_move in moves {
        let (x_to_swap, y_to_swap) = match current_move {
            Move::Up => {
                let mut y_to_swap = y - 1;
                while y_to_swap >= 1 && grid[y_to_swap][x] == Cell::Box {
                    y_to_swap -= 1;
                }

                (x, y_to_swap)
            }
            Move::Right => {
                let mut x_to_swap = x + 1;
                while x_to_swap < grid[0].len() - 1 && grid[y][x_to_swap] == Cell::Box {
                    x_to_swap += 1;
                }

                (x_to_swap, y)
            }
            Move::Down => {
                let mut y_to_swap = y + 1;
                while y_to_swap < grid.len() && grid[y_to_swap][x] == Cell::Box {
                    y_to_swap += 1;
                }

                (x, y_to_swap)
            }
            Move::Left => {
                let mut x_to_swap = x - 1;
                while x_to_swap >= 1 && grid[y][x_to_swap] == Cell::Box {
                    x_to_swap -= 1;
                }

                (x_to_swap, y)
            }
        };

        match grid[y_to_swap][x_to_swap] {
            Cell::Empty => {
                match current_move {
                    Move::Up => y -= 1,
                    Move::Right => x += 1,
                    Move::Down => y += 1,
                    Move::Left => x -= 1,
                };

                let t = grid[y][x];
                grid[y][x] = grid[y_to_swap][x_to_swap];
                grid[y_to_swap][x_to_swap] = t;
            }
            Cell::Box => panic!(
                "Not possible to have box at ({},{}) after trying to move",
                y_to_swap, x_to_swap
            ),
            Cell::Wall => {}
        }
    }
}

fn grid_to_string(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> String {
    let mut s = String::new();
    for row in grid {
        for cell in row {
            match cell {
                Cell::Empty => s.push('.'),
                Cell::Box => s.push('O'),
                Cell::Wall => s.push('#'),
            }
        }
        s.push('\n');
    }
    let robot_index = y * (grid[0].len() + 1) + x;
    s.replace_range(robot_index..robot_index + 1, "@");

    s
}

fn part_one(mut grid: &mut Vec<Vec<Cell>>, x: usize, y: usize, moves: &Vec<Move>) -> usize {
    traverse(grid, x, y, moves);

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, cell)| match cell {
                    Cell::Empty => None,
                    Cell::Box => Some(100 * y + x),
                    Cell::Wall => None,
                })
                .sum::<usize>()
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell2 {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
}

fn double(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell2>> {
    grid.iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| match cell {
                    Cell::Empty => vec![Cell2::Empty, Cell2::Empty],
                    Cell::Box => vec![Cell2::BoxLeft, Cell2::BoxRight],
                    Cell::Wall => vec![Cell2::Wall, Cell2::Wall],
                })
                .collect()
        })
        .collect()
}

fn next_coord(x: usize, y: usize, current_move: &Move) -> (usize, usize) {
    match current_move {
        Move::Up => (y - 1, x),
        Move::Right => (y, x + 1),
        Move::Down => (y + 1, x),
        Move::Left => (y, x - 1),
    }
}

fn can_move(grid: &Vec<Vec<Cell2>>, coord: (usize, usize), current_move: &Move) -> bool {
    let (y, x) = coord;
    match grid[y][x] {
        Cell2::Empty => true,
        Cell2::BoxLeft => match current_move {
            Move::Up | Move::Down => {
                can_move(grid, next_coord(x, y, current_move), current_move)
                    && can_move(grid, next_coord(x + 1, y, current_move), current_move)
            }
            Move::Right | Move::Left => {
                can_move(grid, next_coord(x, y, current_move), current_move)
            }
        },

        Cell2::BoxRight => match current_move {
            Move::Up | Move::Down => {
                can_move(grid, next_coord(x, y, current_move), current_move)
                    && can_move(grid, next_coord(x - 1, y, current_move), current_move)
            }
            Move::Right | Move::Left => {
                can_move(grid, next_coord(x, y, current_move), current_move)
            }
        },
        Cell2::Wall => false,
    }
}

fn move_cells(mut grid: &mut Vec<Vec<Cell2>>, coord: (usize, usize), current_move: &Move) {
    let (y, x) = coord;
    match grid[y][x] {
        Cell2::Empty => {}
        Cell2::BoxLeft => {
            let (left_y, left_x) = next_coord(x, y, current_move);
            move_cells(grid, (left_y, left_x), current_move);
            grid[left_y][left_x] = Cell2::BoxLeft;
            grid[y][x] = Cell2::Empty;
            match current_move {
                Move::Up | Move::Down => {
                    let (right_y, right_x) = next_coord(x + 1, y, current_move);
                    move_cells(grid, (right_y, right_x), current_move);
                    grid[right_y][right_x] = Cell2::BoxRight;
                    grid[y][x + 1] = Cell2::Empty;
                }
                _ => {}
            };
        }
        Cell2::BoxRight => {
            let (right_y, right_x) = next_coord(x, y, current_move);
            move_cells(grid, (right_y, right_x), current_move);
            grid[right_y][right_x] = Cell2::BoxRight;
            grid[y][x] = Cell2::Empty;
            match current_move {
                Move::Up | Move::Down => {
                    let (left_y, left_x) = next_coord(x - 1, y, current_move);
                    move_cells(grid, (left_y, left_x), current_move);
                    grid[left_y][left_x] = Cell2::BoxLeft;
                    grid[y][x - 1] = Cell2::Empty;
                }
                _ => {}
            };
        }
        Cell2::Wall => panic!("Cannot move a wall ({},{})", y, x),
    }
}

fn traverse2(grid: &mut Vec<Vec<Cell2>>, mut x: usize, mut y: usize, moves: &Vec<Move>) {
    for current_move in moves {
        let (next_y, next_x) = next_coord(x, y, current_move);
        if can_move(grid, (next_y, next_x), current_move) {
            move_cells(grid, (next_y, next_x), current_move);
            x = next_x;
            y = next_y;
        }
    }
    println!("{}", grid_to_string2(grid, x, y));
}

fn part_two(grid: &mut Vec<Vec<Cell2>>, x: usize, y: usize, moves: &Vec<Move>) -> usize {
    traverse2(grid, 2 * x, y, moves);
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, cell)| match cell {
                    Cell2::BoxLeft => Some(100 * y + x),
                    _ => None,
                })
                .sum::<usize>()
        })
        .sum()
}

fn grid_to_string2(grid: &Vec<Vec<Cell2>>, x: usize, y: usize) -> String {
    println!("({y},{x})");
    let mut s = String::new();
    for row in grid {
        for cell in row {
            match cell {
                Cell2::Empty => s.push('.'),
                Cell2::BoxLeft => s.push('['),
                Cell2::BoxRight => s.push(']'),
                Cell2::Wall => s.push('#'),
            }
        }
        s.push('\n');
    }
    let robot_index = y * (grid[0].len() + 1) + x;
    s.replace_range(robot_index..robot_index + 1, "@");

    s
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (mut grid, x, y, moves) = parse(&content);
    let mut grid2 = double(&grid);
    println!("{}", part_one(&mut grid, x, y, &moves));
    println!("{}", part_two(&mut grid2, x, y, &moves));
}
