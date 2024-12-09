use std::collections::HashMap;
use std::{fs::File, io::Read};

#[derive(Clone)]
enum Block {
    Free(usize),
    File(usize),
}

impl Block {
    fn is_free(&self) -> bool {
        match self {
            Block::Free(_) => true,
            Block::File(_) => false,
        }
    }

    fn is_file(&self) -> bool {
        !self.is_free()
    }

    fn get_file_id(&self) -> Option<usize> {
        match self {
            Block::Free(_) => None,
            Block::File(file_id) => Some(*file_id),
        }
    }
}

fn parse(content: &str) -> (Vec<Block>, HashMap<usize, usize>, HashMap<usize, usize>) {
    let parsed = content.chars().map(|ch| ch.to_digit(10).unwrap() as usize);

    let mut disk = Vec::new();
    let mut file_sizes = HashMap::new();
    let mut free_sizes = HashMap::new();
    for (i, blocks) in parsed.enumerate() {
        let block_id = if i % 2 == 0 {
            Block::File(i / 2)
        } else {
            Block::Free(i / 2)
        };
        let mut new_blocks = vec![block_id; blocks];
        disk.append(&mut new_blocks);
        if i % 2 == 0 {
            file_sizes.insert(i / 2, blocks);
        } else {
            free_sizes.insert(i / 2, blocks);
        }
    }

    (disk, file_sizes, free_sizes)
}

fn format1(disk: &mut Vec<Block>) {
    let mut free_index = 0;
    let mut file_index = disk.len() - 1;

    loop {
        while disk[free_index].is_file() {
            free_index += 1;
        }

        while disk[file_index].is_free() {
            file_index -= 1;
        }

        if free_index >= file_index {
            return;
        }

        disk.swap(free_index, file_index);
        free_index += 1;
        file_index -= 1;
    }
}

fn part_one(mut disk: &mut Vec<Block>) -> usize {
    format1(&mut disk);
    calculate_checksum(&disk)
}

fn calculate_checksum(disk: &Vec<Block>) -> usize {
    disk.iter()
        .enumerate()
        .fold(0, |checksum, (index, block)| match block {
            Block::File(file_id) => checksum + index * file_id,
            Block::Free(_) => checksum,
        })
}

fn format2(
    disk: &Vec<Block>,
    mut file_sizes: HashMap<usize, usize>,
    mut free_sizes: HashMap<usize, usize>,
) -> Vec<Block> {
    let mut formatted = disk.clone();
    let mut file_index = disk.len() - 1;
    let max_file_id = file_sizes.keys().max().unwrap();
    for file_id in (0..max_file_id + 1).rev() {
        if file_id % 100 == 0 {
            println!("{file_id}");
        }
        while formatted[file_index].is_free()
            || formatted[file_index].get_file_id().unwrap() != file_id
        {
            file_index -= 1;
        }

        let mut free_index = 0;
        while free_index < file_index {
            match formatted[free_index] {
                Block::Free(free_id) => {
                    if free_sizes[&free_id] < file_sizes[&file_id] {
                        free_index += free_sizes[&free_id];
                    } else {
                        for i in 0..file_sizes[&file_id] {
                            formatted.swap(free_index + i, file_index - i);
                        }
                        *free_sizes.get_mut(&free_id).unwrap() -= file_sizes[&file_id];
                        break;
                    }
                }
                Block::File(_) => free_index += 1,
            }
        }
    }

    formatted
}

fn part_two(
    mut disk: &mut Vec<Block>,
    file_sizes: HashMap<usize, usize>,
    mut free_sizes: HashMap<usize, usize>,
) -> usize {
    let formatted = format2(&mut disk, file_sizes, free_sizes);
    calculate_checksum(&formatted)
}

fn main() {
    let mut content = String::new();
    File::open("input")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let (mut disk, file_sizes, free_sizes) = parse(&content);
    println!("{}", part_one(&mut disk.clone()));
    println!("{}", part_two(&mut disk, file_sizes, free_sizes));
}
