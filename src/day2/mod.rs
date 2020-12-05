use crate::utils;
use regex::Regex;
use std::path::PathBuf;

fn parse_password_data(password_data: &String) -> (usize, usize, char, String) {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    let captures = re.captures(password_data).unwrap();

    let min = captures[1].parse::<usize>().unwrap();
    let max = captures[2].parse::<usize>().unwrap();
    let character = captures[3].parse::<char>().unwrap();
    let password = String::from(&captures[4]);

    return (min, max, character, password);
}

fn part1(input: &Vec<String>) -> i32 {
    let mut correct_count = 0;

    for line in input {
        let (min, max, character, password) = parse_password_data(line);
        let character_occurences_count = password.match_indices(character).count();

        if character_occurences_count >= min && character_occurences_count <= max {
            correct_count += 1;
        }
    }
    return correct_count;
}

fn part2(input: &Vec<String>) -> i32 {
    let mut correct_count = 0;

    for line in input {
        let (low, high, character, password) = parse_password_data(line);
        let low_char = password.chars().nth(low - 1);
        let high_char = password.chars().nth(high - 1);
        let low_match = low_char == Some(character);
        let high_match = high_char == Some(character);

        if low_match as i8 + high_match as i8 == 1 {
            correct_count += 1;
        }
    }

    return correct_count;
}

pub fn solve() {
    let input = utils::file::read_strings(PathBuf::from(file!()));

    println!("Day 2 Part 1: {}", part1(&input));
    println!("Day 2 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let example_input = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        assert_eq!(part1(&example_input), 2);
        let file_input = utils::file::read_strings(PathBuf::from(file!()));
        assert_eq!(part1(&file_input), 620);
    }

    #[test]
    fn part2_works() {
        let example_input = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        assert_eq!(part2(&example_input), 1);
        let file_input = utils::file::read_strings(PathBuf::from(file!()));
        assert_eq!(part2(&file_input), 727);
    }
}
