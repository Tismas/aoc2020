use crate::utils;
use reduce::Reduce;
use std::path::PathBuf;

fn count_tree(map: &Vec<String>, (right, down): (usize, usize)) -> i32 {
    let mut sum = 0;

    for (i, line) in map.iter().step_by(down).enumerate() {
        if line.chars().nth((i * right) % line.len()) == Some('#') {
            sum += 1;
        }
    }

    return sum;
}

fn part1(input: &Vec<String>) -> i32 {
    return count_tree(input, (3, 1));
}

fn part2(input: &Vec<String>) -> i32 {
    let values = vec![
        count_tree(input, (1, 1)),
        count_tree(input, (3, 1)),
        count_tree(input, (5, 1)),
        count_tree(input, (7, 1)),
        count_tree(input, (1, 2)),
    ];

    return values.into_iter().reduce(|a, b| a * b).unwrap();
}

pub fn solve() {
    let input = utils::file::read_strings(PathBuf::from(file!()));

    println!("Day 3 Part 1: {}", part1(&input));
    println!("Day 3 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let example_input = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ];
        assert_eq!(part1(&example_input), 7);
        let file_input = utils::file::read_strings(PathBuf::from(file!()));
        assert_eq!(part1(&file_input), 270);
    }

    #[test]
    fn part2_works() {
        let example_input = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ];
        assert_eq!(part2(&example_input), 336);
        let file_input = utils::file::read_strings(PathBuf::from(file!()));
        assert_eq!(part2(&file_input), 2122848000);
    }
}
