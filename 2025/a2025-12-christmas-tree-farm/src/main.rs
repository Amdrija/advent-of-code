use std::{fs::File, io::Read};

#[derive(Debug)]
struct ChristmasTree {
    width: u64,
    height: u64,
    gifts: [u64; 6],
}

impl ChristmasTree {
    fn fits(&self, sizes: &[u64; 6]) -> bool {
        return self.width * self.height
            >= self
                .gifts
                .iter()
                .zip(sizes)
                .map(|(count, size)| count * size)
                .sum();
    }
}

fn parse(input: &str) -> Vec<ChristmasTree> {
    input
        .lines()
        .map(|line| {
            let splitted = line.split(": ").collect::<Vec<_>>();
            let tree_size = splitted[0]
                .split("x")
                .map(|size| size.parse().unwrap())
                .collect::<Vec<_>>();
            let gift_counts = splitted[1]
                .split(" ")
                .map(|count| count.parse().unwrap())
                .collect::<Vec<_>>();

            ChristmasTree {
                width: tree_size[0],
                height: tree_size[1],
                gifts: gift_counts.try_into().unwrap(),
            }
        })
        .collect()
}
fn main() {
    let mut input = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let sizes = [5, 6, 7, 7, 7, 7];
    let trees = parse(&input);
    for tree in &trees {
        println!("{:?}", tree);
    }
    println!("{}", trees.iter().filter(|tree| tree.fits(&sizes)).count());
}
