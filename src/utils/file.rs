use std::fs;
use std::path::PathBuf;

fn read_input_file(path: PathBuf) -> String {
    return fs::read_to_string(path.parent().unwrap().join("input.txt")).unwrap();
}

pub fn read_ints(path: PathBuf) -> Vec<i32> {
    return read_input_file(path)
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
}

pub fn read_strings_sep(path: PathBuf, separator: &str) -> Vec<String> {
    return read_input_file(path)
        .split(separator)
        .map(|line| String::from(line.trim()))
        .collect();
}

pub fn read_strings(path: PathBuf) -> Vec<String> {
    return read_strings_sep(path, "\n");
}
