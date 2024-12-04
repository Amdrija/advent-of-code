use std::fs::File;
use std::io::Read;

fn parse(content: &str) -> Vec<Vec<char>> {
    content.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(crossword: &Vec<Vec<char>>) -> u64 {
    let directions = vec![
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ];
    let mut count = 0;
    for i in 0..crossword.len() {
        for j in 0..crossword[i].len() {
            if crossword[i][j] == 'X' {
                for direction in &directions {
                    if is_xmas(crossword, i, j, *direction) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn is_xmas(crossword: &Vec<Vec<char>>, i: usize, j: usize, direction: (i64, i64)) -> bool {
    let xmas = "XMAS";
    let (delta_i, delta_j) = direction;
    for (k, target) in xmas.chars().enumerate() {
        let next_i = i as i64 + k as i64 * delta_i;
        if next_i < 0 || next_i >= crossword.len() as i64 {
            return false;
        }

        let next_j = j as i64 + k as i64 * delta_j;
        if next_j < 0 || next_j >= crossword[i].len() as i64 {
            return false;
        }

        if crossword[next_i as usize][next_j as usize] != target {
            return false;
        }
    }

    true
}

fn part_two(crossword: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    // If you encounter an A on the edge of the matrix
    // it is not possible to form a cross, so don't even bother
    // with these i and j pairs
    for i in 1..crossword.len() - 1 {
        for j in 1..crossword[i].len() - 1 {
            if crossword[i][j] == 'A' && is_cross_mas(crossword, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn is_cross_mas(crossword: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let diagonal = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut m_count = 0;
    let mut s_count = 0;
    for (di, dj) in diagonal {
        let current_char = crossword[(i as i64 + di) as usize][(j as i64 + dj) as usize];
        if current_char == 'M' {
            m_count += 1;
        } else if current_char == 'S' {
            s_count += 1;
        } else {
            return false;
        }
    }

    // In order to form the:
    // M.S
    // .A.
    // M.S
    // we need exactly 2 Ms and 2 Ss, plus the diagonal elements must be different
    // it is sufficient to check only 1 diagonal, because if the elements on it are
    // the same, they must be the same on the other diagonal as well.
    m_count == 2 && s_count == 2 && crossword[i - 1][j - 1] != crossword[i + 1][j + 1]
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let crossword = parse(&content);
    println!("{}", part_one(&crossword));
    println!("{}", part_two(&crossword));
}
