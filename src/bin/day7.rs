use std::{collections::HashMap, path::PathBuf};

fn main() {
    let input = include_str!("./day7.prod");
    let p1_result = part_one(input);
    let p2_result = part_two(input);

    println!("Part One Result: {p1_result}");
    println!("Part Two Result: {p2_result}");
}

fn part_one(input: &str) -> i32 {
    let dir_sizes = get_sizes(input);
    return dir_sizes.into_values().filter(|size| *size <= 100_000).sum();
}

fn part_two(input: &str) -> i32 {
    let dir_sizes = get_sizes(input);
    let max_size = 70_000_000;
    let min_size = 30_000_000;
    let root = dir_sizes.get(&PathBuf::from("/")).unwrap();
    let available = max_size - root;

    return dir_sizes.into_values().filter(|s| available + s >= min_size).min().unwrap();
}

fn get_sizes(input: &str) -> HashMap<PathBuf, i32> {
    let mut dir_sizes = HashMap::new();
    let mut directories = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                directories.pop();
            }
            ["$", "cd", name] => {
                directories.push(name);
            }
            [size, name] => {
                let size: i32 = size.parse().unwrap();
                for i in 0..directories.len() {
                    let path = PathBuf::from_iter(&directories[..=i]);
                    *dir_sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }
    return dir_sizes;
}
