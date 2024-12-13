use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct SolutionFormula {
    pub free: f64,
    pub coef: f64,
}

impl SolutionFormula {
    pub fn new(free: f64, coef: f64) -> Self {
        Self { free, coef }
    }

    pub fn find_optimal(&self) -> Option<u64> {
        for a in 0..101 {
            if let Some(b) = self.get_b(a) {
                if b < 0 {
                    return None;
                }

                return Some(3 * a + b as u64);
            }
        }

        None
    }

    fn get_b(&self, a: u64) -> Option<i64> {
        let b = self.free + self.coef * (a as f64);
        if b > 100.0 || (b.round() - b).abs() > DELTA {
            return None;
        }

        Some(b.round() as i64)
    }
}

impl PartialEq for SolutionFormula {
    fn eq(&self, other: &Self) -> bool {
        (self.free - other.free).abs() < DELTA && (self.coef - other.coef).abs() < DELTA
    }
}

#[derive(Debug, PartialEq)]
pub enum Solution {
    Unique(u64, u64),
    Formula(SolutionFormula),
    None,
}

pub struct Equation {
    coef: [[f64; 2]; 2],
    free: [f64; 2],
}

const DELTA: f64 = 0.001;
impl Equation {
    pub fn new(coef: [[f64; 2]; 2], free: [f64; 2]) -> Self {
        Self { coef, free }
    }

    pub fn solve(&mut self) -> Solution {
        self.gauss();

        if self.coef[1][1].abs() < DELTA {
            if self.free[1] > DELTA {
                return Solution::None;
            }

            Solution::Formula(SolutionFormula::new(
                self.free[0] / self.coef[0][1],
                -self.coef[0][0] / self.coef[0][1],
            ))
        } else {
            let b_presses = self.free[1] / self.coef[1][1];
            let a_presses = (self.free[0] - b_presses * self.coef[0][1]) / self.coef[0][0];

            if (b_presses - b_presses.round()).abs() > DELTA
                || (a_presses - a_presses.round()).abs() > DELTA
                || a_presses < 0.0
                || b_presses < 0.0
            {
                return Solution::None;
            }

            Solution::Unique(a_presses.round() as u64, b_presses.round() as u64)
        }
    }

    fn gauss(&mut self) {
        let g = -self.coef[1][0] / self.coef[0][0];
        self.coef[1][0] = 0.0;
        self.coef[1][1] += g * self.coef[0][1];
        self.free[1] += g * self.free[0];
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let mut rows = value
            .lines()
            .map(|line| {
                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|coord| {
                        coord
                            .split(|ch| ch == '+' || ch == '=')
                            .nth(1)
                            .unwrap()
                            .parse::<f64>()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Equation {
            coef: [[rows[0][0], rows[1][0]], [rows[0][1], rows[1][1]]],
            free: [rows[2][0], rows[2][1]],
        }
    }
}

fn part_one(content: &str) -> u64 {
    content
        .split("\n\n")
        .map(|equation| Equation::from(equation))
        .filter_map(|mut equation| match equation.solve() {
            Solution::Unique(a, b) => Some(3 * a + b),
            Solution::Formula(formula) => formula.find_optimal(),
            Solution::None => None,
        })
        .sum()
}

fn part_two(content: &str) -> u64 {
    content
        .split("\n\n")
        .map(|equation| {
            let e = Equation::from(equation);
            Equation::new(
                e.coef,
                [10000000000000.0 + e.free[0], 10000000000000.0 + e.free[1]],
            )
        })
        .filter_map(|mut equation| match equation.solve() {
            Solution::Unique(a, b) => Some(3 * a + b),
            Solution::Formula(formula) => formula.find_optimal(),
            Solution::None => None,
        })
        .sum()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    println!("{}", part_one(&content));
    println!("{}", part_two(&content));
}

#[cfg(test)]
mod test {
    use crate::{Equation, Solution, SolutionFormula};

    #[test]
    fn test_solve() {
        assert_eq!(
            Equation::new([[2.0, 1.0], [1.0, 1.0]], [5.0, 3.0]).solve(),
            Solution::Unique(2, 1)
        );
        assert_eq!(
            Equation::new([[2.0, 1.0], [4.0, 1.0]], [2.0, 3.0]).solve(),
            Solution::None
        );
        assert_eq!(
            Equation::new([[1.0, 1.0], [2.0, 1.0]], [3.0, 1.0]).solve(),
            Solution::None
        );
        assert_eq!(
            Equation::new([[1.0, 2.0], [1.0, 4.0]], [2.0, 3.0]).solve(),
            Solution::None
        );
        assert_eq!(
            Equation::new([[1.0, 1.0], [1.0, 2.0]], [3.0, 1.0]).solve(),
            Solution::None
        );

        assert_eq!(
            Equation::new([[1.0, 2.0], [2.0, 4.0]], [2.0, 4.0]).solve(),
            Solution::Formula(SolutionFormula::new(1.0, -0.5))
        );
        assert_eq!(
            Equation::new([[1.0, 2.0], [2.0, 4.0]], [2.0, 5.0]).solve(),
            Solution::None
        );
    }

    #[test]
    fn test_find_optimal() {
        assert_eq!(SolutionFormula::new(1.5, -0.5).find_optimal(), Some(4));
        assert_eq!(SolutionFormula::new(1.7, -0.3).find_optimal(), None);
        assert_eq!(SolutionFormula::new(1000.0, -1.0).find_optimal(), None);
    }
}
