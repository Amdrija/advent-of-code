use std::{fs::File, io::Read};

#[derive(Debug, Clone, PartialEq)]
struct Point {
    id: usize,
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z))
            .sqrt()
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let point_vec = line
                .split(",")
                .map(|coord| coord.parse().unwrap())
                .collect::<Vec<_>>();
            Point {
                id: i,
                x: point_vec[0],
                y: point_vec[1],
                z: point_vec[2],
            }
        })
        .collect()
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>
}

impl UnionFind {
    fn new(nodes: usize) -> Self {
        Self { parent: (0..nodes).collect(), size: vec![1; nodes] }
    }

    fn find(&mut self, node: usize) -> usize {
        let mut current = node;
        while current != self.parent[current] {
            current = self.parent[current];
        }

        let root = current;
        current = node;
        while current != root {
            let parent = self.parent[current];
            self.parent[current] = root;
            current = parent
        }

        root
    }

    fn add(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.size[root_a] > self.size[root_b] {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        } else {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        }

        true
    }
}

fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let points = parse(&input);
    let mut distances = points
        .iter()
        .enumerate()
        .flat_map(|(i, point)| points.iter().skip(i + 1).map(move |point2| (point, point2)))
        .collect::<Vec<_>>();
    distances.sort_by(|pair1, pair2| {
        pair1
            .0
            .distance(pair1.1)
            .total_cmp(&pair2.0.distance(pair2.1))
    });

    let mut uf = UnionFind::new(points.len());
    for i in 0..1000 {
        uf.add(distances[i].0.id, distances[i].1.id);
    }

    let mut sizes = uf.size.clone();
    sizes.sort_by(|a, b| b.cmp(a));
    println!("{:?}", &sizes[..3].iter().product::<usize>());

    let mut added = 0;
    for distance in distances {
        if uf.add(distance.0.id, distance.1.id) {
            println!("{:?} {:?} {}", distance.0, distance.1, distance.0.x * distance.1.x);
            added += 1;
            if added == points.len() - 1 {
                println!("{:?} {:?} {}", distance.0, distance.1, distance.0.x * distance.1.x);
            }
        }
    }
}
