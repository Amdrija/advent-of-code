use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn key_path(start: char, end: char) -> Vec<Vec<char>> {
    let mut path = match (start, end) {
        ('A', '0') => vec![vec!['<']],
        ('0', '2') => vec![vec!['^']],
        ('2', '9') => vec![vec!['>', '^', '^'], vec!['^', '^', '>']],
        ('9', 'A') => vec![vec!['v', 'v', 'v']],
        ('A', '9') => vec![vec!['^', '^', '^']],
        ('9', '8') => vec![vec!['<']],
        ('8', '0') => vec![vec!['v', 'v', 'v']],
        ('0', 'A') => vec![vec!['>']],
        ('A', '1') => vec![vec!['^', '<', '<']],
        ('1', '7') => vec![vec!['^', '^']],
        ('7', '9') => vec![vec!['>', '>']],
        ('A', '4') => vec![vec!['^', '^', '<', '<']],
        ('4', '5') => vec![vec!['>']],
        ('5', '6') => vec![vec!['>']],
        ('6', 'A') => vec![vec!['v', 'v']],
        ('A', '3') => vec![vec!['^']],
        ('3', '7') => vec![vec!['<', '<', '^', '^'], vec!['^', '^', '<', '<']],
        // INPUT FROM HERE
        ('1', '4') => vec![vec!['^']],
        ('4', '0') => vec![vec!['>', 'v', 'v']],
        ('4', '3') => vec![vec!['v', '>', '>'], vec!['>', '>', 'v']],
        ('3', 'A') => vec![vec!['v']],
        ('3', '4') => vec![vec!['<', '<', '^'], vec!['^', '<', '<']],
        ('4', '9') => vec![vec!['^', '>', '>'], vec!['>', '>', '^']],
        ('A', '5') => vec![vec!['<', '^', '^'], vec!['^', '^', '<']],
        ('5', '8') => vec![vec!['^']],
        ('8', '2') => vec![vec!['v', 'v']],
        ('2', 'A') => vec![vec!['v', '>'], vec!['>', 'v']],
        ('9', '6') => vec![vec!['v']],
        ('6', '4') => vec![vec!['<', '<']],
        ('4', 'A') => vec![vec!['>', '>', 'v', 'v']],
        (_, _) => panic!("Uknown path from start={} to end={} in test", start, end),
    };
    path.into_iter()
        .map(|mut path| {
            path.push('A');
            path
        })
        .collect()
}

fn arrow_path(start: char, end: char) -> Vec<Vec<char>> {
    let mut path = match (start, end) {
        ('^', '^') => vec![vec![]],
        ('^', '>') => vec![vec!['v', '>'], vec!['>', 'v']],
        ('^', '<') => vec![vec!['v', '<']],
        ('^', 'v') => vec![vec!['v']],
        ('^', 'A') => vec![vec!['>']],
        ('>', '^') => vec![vec!['<', '^'], vec!['^', '<']],
        ('>', '>') => vec![vec![]],
        ('>', '<') => vec![vec!['<', '<']],
        ('>', 'v') => vec![vec!['<']],
        ('>', 'A') => vec![vec!['^']],
        ('<', '^') => vec![vec!['>', '^']],
        ('<', '>') => vec![vec!['>', '>']],
        ('<', '<') => vec![vec![]],
        ('<', 'v') => vec![vec!['>']],
        ('<', 'A') => vec![vec!['>', '>', '^']],
        ('v', '^') => vec![vec!['^']],
        ('v', '>') => vec![vec!['>']],
        ('v', '<') => vec![vec!['<']],
        ('v', 'v') => vec![vec![]],
        ('v', 'A') => vec![vec!['>', '^'], vec!['^', '>']],
        ('A', '^') => vec![vec!['<']],
        ('A', '>') => vec![vec!['v']],
        ('A', '<') => vec![vec!['v', '<', '<']],
        ('A', 'v') => vec![vec!['v', '<'], vec!['<', 'v']],
        ('A', 'A') => vec![vec![]],
        (_, _) => panic!("Uknown characters start={}, end={}", start, end),
    };
    path.into_iter()
        .map(|mut path| {
            path.push('A');
            path
        })
        .collect()
}

fn calculate_len(
    cache: &mut HashMap<(usize, char, char), usize>,
    level: usize,
    start: char,
    end: char,
    keypad: bool,
) -> usize {
    if level == 0 {
        return 1;
    }

    if cache.contains_key(&(level, start, end)) {
        return cache[&(level, start, end)];
    }

    let paths = if keypad {
        key_path(start, end)
    } else {
        arrow_path(start, end)
    };

    let len = paths
        .iter()
        .map(|path| {
            let mut len = calculate_len(cache, level - 1, 'A', path[0], false);
            for i in 0..path.len() - 1 {
                len += calculate_len(cache, level - 1, path[i], path[i + 1], false);
            }

            len
        })
        .min()
        .unwrap();

    cache.insert((level, start, end), len);
    len
}

fn solve(content: &str, levels: usize) -> usize {
    let mut cache = HashMap::new();
    let lens = content
        .lines()
        .map(|line| {
            let mut code = line.chars().collect::<Vec<_>>();
            let mut len = calculate_len(&mut cache, levels, 'A', code[0], true);
            for i in 0..code.len() - 1 {
                len += calculate_len(&mut cache, levels, code[i], code[i + 1], true);
            }

            let number = line[..3].parse::<usize>().unwrap();
            (len, number)
        })
        .collect::<Vec<_>>();

    lens.iter().map(|(l, n)| l * n).sum()
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    println!("{}", solve(&content, 3));
    println!("{}", solve(&content, 26));
}
