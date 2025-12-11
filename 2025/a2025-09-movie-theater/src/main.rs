use std::{cmp, collections::VecDeque, fmt::Display, fs::File, io::Read};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn points(r1: &Point, r2: &Point) -> impl Iterator<Item = Point> {
        let max_x = cmp::max(r1.x, r2.x);
        let min_x = cmp::min(r1.x, r2.x);
        let max_y = cmp::max(r1.y, r2.y);
        let min_y = cmp::min(r1.y, r2.y);

        return (min_x..=max_x).flat_map(move |x| (min_y..=max_y).map(move |y| Point { x, y }));
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn surface(&self) -> usize {
        let diff_x = if self.p1.x > self.p2.x {
            self.p1.x - self.p2.x + 1
        } else {
            self.p2.x - self.p1.x + 1
        };

        let diff_y = if self.p1.y > self.p2.y {
            self.p1.y - self.p2.y + 1
        } else {
            self.p2.y - self.p1.y
        };

        return diff_x * diff_y;
    }

    fn points(&self) -> Vec<Point> {
        return vec![
            self.p1.clone(),
            Point {
                x: self.p1.x,
                y: self.p2.y,
            },
            Point {
                x: self.p2.x,
                y: self.p1.y,
            },
            self.p2.clone(),
        ];
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut splitted = line.split(",");
            Point {
                x: splitted.next().unwrap().parse().unwrap(),
                y: splitted.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

struct Polygon {
    grid: Vec<Vec<char>>,
}

impl Polygon {
    fn new(points: &[Point]) -> Self {
        let max_x = points.iter().map(|p| p.x).max().unwrap() as usize;
        let max_y = points.iter().map(|p| p.y).max().unwrap() as usize;

        println!("{} {} {}", max_x, max_y, max_x * max_y);

        let mut grid = vec![vec!['.'; max_x + 2]; max_y + 2];
        for i in 0..points.len() {
            if points[i].x == points[(i + 1) % points.len()].x {
                let start = cmp::min(points[i].y, points[(i + 1) % points.len()].y);
                let end = cmp::max(points[i].y, points[(i + 1) % points.len()].y);
                for y in start..=end {
                    grid[y][points[i].x] = '#';
                }
            } else {
                let start = cmp::min(points[i].x, points[(i + 1) % points.len()].x);
                let end = cmp::max(points[i].x, points[(i + 1) % points.len()].x);
                for x in start..=end {
                    grid[points[i].y][x] = '#';
                }
            }
        }

        let mut inside_point;
        loop {
            inside_point = Point {
                x: rand::random_range(0..grid[0].len()),
                y: rand::random_range(0..grid.len()),
            };
            if grid[inside_point.y][inside_point.x] == '.'
                && Self::point_is_inside(&grid, &inside_point)
            {
                break;
            }
        }
        println!(
            "{} {}",
            Self::point_is_inside(&grid, &inside_point),
            inside_point
        );
        let mut q = VecDeque::new();
        q.push_back((inside_point.y as i64, inside_point.x as i64));
        let mut i = 0i64;
        while let Some((y, x)) = q.pop_front() {
            i += 1;
            if i % 10000000 == 0 {
                println!("{i}");
            }
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let y = y + dy;
                let x = x + dx;
                if y >= 0
                    && y < grid.len() as i64
                    && x >= 0
                    && x < grid[0].len() as i64
                    && grid[y as usize][x as usize] == '.'
                {
                    grid[y as usize][x as usize] = '#';
                    q.push_back((y, x));
                }
            }
        }
        Self { grid }
    }

    fn point_is_inside(grid: &Vec<Vec<char>>, point: &Point) -> bool {
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let mut y = point.y as i64;
            let mut x = point.x as i64;
            let mut inside = false;
            while y >= 0 && y < grid.len() as i64 && x >= 0 && x < grid[0].len() as i64 {
                if grid[y as usize][x as usize] == '#' {
                    inside = true;
                    break;
                }
                y += dy;
                x += dx;
            }

            if !inside {
                return false;
            }
        }

        true
    }

    fn is_inside(&self, rectangle: &Rectangle) -> bool {
        let min_x = cmp::min(rectangle.p1.x, rectangle.p2.x);
        let max_x = cmp::max(rectangle.p1.x, rectangle.p2.x);
        let min_y = cmp::min(rectangle.p1.y, rectangle.p2.y);
        let max_y = cmp::max(rectangle.p1.y, rectangle.p2.y);
        for x in min_x..=max_x {
            if self.grid[min_y][x] != '#' || self.grid[max_y][x] != '#' {
                return false;
            }
        }

        for y in min_y..=max_y {
            if self.grid[y][min_x] != '#' || self.grid[y][max_x] != '#' {
                return false;
            }
        }

        return true;
    }
}

impl Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let points = parse(&input);

    let mut rectangles = points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points[i + 1..].iter().map(move |p2| Rectangle {
                p1: p1.clone(),
                p2: p2.clone(),
            })
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        rectangles.iter().map(Rectangle::surface).max().unwrap()
    );

    rectangles.sort_by(|r1, r2| r2.surface().cmp(&r1.surface()));

    let polygon = Polygon::new(&points);
    let mut i = 0i64;
    for rectangle in rectangles {
        println!("{}: {} {}", i, rectangle.p1, rectangle.p2);
        i += 1;
        if polygon.is_inside(&rectangle) {
            let points = rectangle.points();
            println!("{:?}", points);
            println!("{}", points.len());
            println!("{:?}", rectangle);
            println!("{}", rectangle.surface());
            break;
        }
    }

    // println!("{}", polygon);
}
