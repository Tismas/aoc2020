use std::fs;
use std::path::PathBuf;

pub fn read_ints(path: PathBuf) -> Vec<i32> {
    return fs::read_to_string(path.parent().unwrap().join("input.txt"))
        .unwrap()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
}
