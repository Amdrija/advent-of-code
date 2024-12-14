use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

#[derive(Hash, Eq, PartialEq)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

impl Robot {
    fn position_after(&self, time: i64, size_x: i64, size_y: i64) -> Robot {
        Self {
            x: (self.x + time * self.vx).rem_euclid(size_x),
            y: (self.y + time * self.vy).rem_euclid(size_y),
            vx: self.vx,
            vy: self.vy,
        }
    }

    fn get_quadrant(&self, size_x: i64, size_y: i64) -> Option<Quadrant> {
        let middle_x = size_x / 2;
        let middle_y = size_y / 2;

        if self.x < middle_x {
            if self.y < middle_y {
                return Some(Quadrant::First);
            } else if self.y > middle_y + 1 || (size_y % 2 == 1 && self.y > middle_y) {
                return Some(Quadrant::Second);
            }
        } else if self.x > middle_x + 1 || (size_x % 2 == 1 && self.x > middle_x) {
            if self.y < middle_y {
                return Some(Quadrant::Third);
            } else if self.y > middle_y + 1 || (size_y % 2 == 1 && self.y > middle_y) {
                return Some(Quadrant::Fourth);
            }
        }

        None
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let mut splitted = value.split(" ");
        let position = splitted
            .next()
            .unwrap()
            .strip_prefix("p=")
            .unwrap()
            .split(",")
            .map(|coord| coord.parse().unwrap())
            .collect::<Vec<_>>();
        let velocity = splitted
            .next()
            .unwrap()
            .strip_prefix("v=")
            .unwrap()
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>();

        Self {
            x: position[0],
            y: position[1],
            vx: velocity[0],
            vy: velocity[1],
        }
    }
}

fn solve(content: &str, time: i64, size_x: i64, size_y: i64) -> u64 {
    content
        .lines()
        .map(|line| Robot::from(line))
        .map(|robot| robot.position_after(time, size_x, size_y))
        .fold(HashMap::<Quadrant, u64>::new(), |mut quadrants, robot| {
            let robot_quadrant = robot.get_quadrant(size_x, size_y);
            match robot_quadrant {
                None => quadrants,
                Some(robot_quadrant) => {
                    *quadrants.entry(robot_quadrant).or_insert(0) += 1;
                    quadrants
                }
            }
        })
        .values()
        .product()
}

fn part_two(content: &str) {
    let mut robots = content
        .lines()
        .map(|line| Robot::from(line))
        .collect::<Vec<_>>();
    let mut buffer = String::new();
    let mut time = 0;
    loop {
        let robot_map = robots.iter().map(|r| ((r.x, r.y), r)).collect();
        if is_tree(&robot_map) {
            print_robots(&robot_map, 101, 103);
            println!("Time {}", time);
            io::stdin().read_line(&mut buffer);
        } else if time % 100 == 0 {
            println!("Time {}", time);
        }
        time += 1;

        robots = robots
            .into_iter()
            .map(|robot| robot.position_after(1, 101, 103))
            .collect();
    }
}

fn is_tree(robots: &HashMap<(i64, i64), &Robot>) -> bool {
    robots.values().any(|robot| {
        robots.contains_key(&(robot.x - 2, robot.y - 2))
            && robots.contains_key(&(robot.x - 1, robot.y - 1))
            && robots.contains_key(&(robot.x + 1, robot.y + 1))
            && robots.contains_key(&(robot.x + 2, robot.y + 2))
    }) && robots.values().any(|robot| {
        robots.contains_key(&(robot.x - 2, robot.y + 2))
            && robots.contains_key(&(robot.x - 1, robot.y + 1))
            && robots.contains_key(&(robot.x + 1, robot.y - 1))
            && robots.contains_key(&(robot.x + 2, robot.y - 2))
    })
}

fn print_robots(robots: &HashMap<(i64, i64), &Robot>, size_x: i64, size_y: i64) {
    for y in 0..size_y {
        for x in 0..size_x {
            match robots.get(&(x, y)) {
                None => print!("."),
                Some(_) => print!("*"),
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    println!("{}", solve(&content, 100, 101, 103));
    part_two(&content);
}
