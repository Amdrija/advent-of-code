use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    ops::{Add, Neg, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Antenna {
    i: i64,
    j: i64,
}

impl Antenna {
    fn new(i: i64, j: i64) -> Self {
        Self { i, j }
    }

    fn in_bounds(&self, m: usize, n: usize) -> bool {
        let m: i64 = m as i64;
        let n: i64 = n as i64;
        self.i >= 0 && self.i < m && self.j >= 0 && self.j < n
    }
}

impl Add for Antenna {
    type Output = Antenna;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.i + rhs.i, self.j + rhs.j)
    }
}

impl Sub for Antenna {
    type Output = Antenna;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.i - rhs.i, self.j - rhs.j)
    }
}

impl Neg for Antenna {
    type Output = Antenna;

    fn neg(self) -> Self::Output {
        Self::new(-self.i, -self.j)
    }
}

fn parse(content: &str) -> (HashMap<char, Vec<Antenna>>, usize, usize) {
    let mut antennas = HashMap::new();
    for (i, line) in content.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas
                    .entry(ch)
                    .or_insert(Vec::new())
                    .push(Antenna::new(i as i64, j as i64));
            }
        }
    }

    (
        antennas,
        content.lines().count(),
        content.lines().next().unwrap().len(),
    )
}

fn solve(
    antennas: &HashMap<char, Vec<Antenna>>,
    m: usize,
    n: usize,
    antinodes: &dyn Fn(&Vec<Antenna>, usize, usize) -> HashSet<Antenna>,
) -> u64 {
    antennas
        .values()
        .map(|same_antennas| antinodes(same_antennas, m, n))
        .fold(HashSet::new(), |mut all, same_antinodes| {
            all.extend(same_antinodes);
            all
        })
        .len() as u64
}

fn antinodes(antennas: &Vec<Antenna>, m: usize, n: usize) -> HashSet<Antenna> {
    let mut antinodes = HashSet::new();
    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let diff = antennas[i] - antennas[j];
            let antinode1 = antennas[i] + diff;
            if antinode1.in_bounds(m, n) {
                antinodes.insert(antinode1);
            }

            let antinode2 = antennas[j] - diff;
            if antinode2.in_bounds(m, n) {
                antinodes.insert(antinode2);
            }
        }
    }

    antinodes
}

fn antinodes2(antennas: &Vec<Antenna>, m: usize, n: usize) -> HashSet<Antenna> {
    let mut antinodes = HashSet::new();
    for i in 0..antennas.len() - 1 {
        for j in i + 1..antennas.len() {
            let diff = antennas[i] - antennas[j];
            let mut current = antennas[i];
            while current.in_bounds(m, n) {
                antinodes.insert(current);
                current = current + diff;
            }

            let mut current = antennas[j];
            while current.in_bounds(m, n) {
                antinodes.insert(current);
                current = current - diff;
            }
        }
    }

    antinodes
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let (antennas, m, n) = parse(&content);
    println!("{}", solve(&antennas, m, n, &antinodes));
    println!("{}", solve(&antennas, m, n, &antinodes2));
}
