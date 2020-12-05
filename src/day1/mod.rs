use crate::utils;
use std::path::PathBuf;

fn find_sum_equal(elements: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    let mut sorted = elements.clone();
    sorted.sort();

    let mut left = 0;
    let mut right = sorted.len() - 1;

    while left != right {
        let left_val = sorted[left];
        let right_val = sorted[right];
        if left_val + right_val == sum {
            return Some((left_val, right_val));
        }
        if left_val + right_val < sum {
            left = left + 1;
        } else {
            right = right - 1;
        }
    }
    return None;
}

fn part1(input: &Vec<i32>) -> i32 {
    let (low, high) = find_sum_equal(input, 2020).unwrap();
    return low * high;
}

fn part2(input: &Vec<i32>) -> i32 {
    for x in input {
        let sum_elements = find_sum_equal(input, 2020 - x);
        if sum_elements.is_some() {
            let (low, high) = sum_elements.unwrap();
            return x * low * high;
        }
    }
    return -1;
}

pub fn solve() {
    let input = utils::file::read_ints(PathBuf::from(file!()));

    println!("Day 1 Part 1: {}", part1(&input));
    println!("Day 1 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let example_input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&example_input), 514579);
        let file_input = utils::file::read_ints(PathBuf::from(file!()));
        assert_eq!(part1(&file_input), 793524);
    }

    #[test]
    fn part2_works() {
        let example_input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&example_input), 241861950);
        let file_input = utils::file::read_ints(PathBuf::from(file!()));
        assert_eq!(part2(&file_input), 61515678);
    }
}
