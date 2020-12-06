use crate::utils;
use std::collections::HashSet;
use std::path::PathBuf;

fn part1(input: &Vec<String>) -> usize {
    let mut sum = 0;
    for group in input {
        let mut yes_in_group = HashSet::new();
        for answer in group.chars().filter(|a| a.is_alphabetic()) {
            yes_in_group.insert(answer);
        }
        sum += yes_in_group.len();
    }
    return sum;
}

fn part2(input: &Vec<String>) -> usize {
    let mut sum = 0;
    for group in input {
        let mut yes_in_group: Option<HashSet<char>> = None;
        for person_answers in group.split("\n") {
            let persons_yes_answers: HashSet<char> = person_answers
                .chars()
                .filter(|a| a.is_alphabetic())
                .collect();
            if yes_in_group.is_none() {
                yes_in_group = Some(persons_yes_answers);
            } else {
                yes_in_group = Some(
                    yes_in_group
                        .unwrap()
                        .intersection(&persons_yes_answers)
                        .cloned()
                        .collect::<HashSet<char>>(),
                );
            }
        }
        sum += yes_in_group.unwrap().len();
    }
    return sum;
}

pub fn solve() {
    let input = utils::file::read_strings_sep(PathBuf::from(file!()), "\n\n");

    println!("Day 6 Part 1: {}", part1(&input));
    println!("Day 6 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let one_group = vec![String::from("abcx\nabcy\nabcz")];
        let multiple_groups = vec![
            String::from("abc"),
            String::from("a\nb\nc"),
            String::from("ab\nac"),
            String::from("a\na\na\na"),
            String::from("b"),
        ];
        assert_eq!(part1(&one_group), 6);
        assert_eq!(part1(&multiple_groups), 11);
        let file_input = utils::file::read_strings_sep(PathBuf::from(file!()), "\n\n");
        assert_eq!(part1(&file_input), 6775);
    }

    #[test]
    fn part2_works() {
        let multiple_groups = vec![
            String::from("abc"),
            String::from("a\nb\nc"),
            String::from("ab\nac"),
            String::from("a\na\na\na"),
            String::from("b"),
        ];

        assert_eq!(part2(&multiple_groups), 6);
        let file_input = utils::file::read_strings_sep(PathBuf::from(file!()), "\n\n");
        assert_eq!(part2(&file_input), 3356);
    }
}
