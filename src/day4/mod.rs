use crate::utils;
mod passport;
use regex::Regex;
use std::path::PathBuf;

fn parse_passport(passport_data: &String) -> Option<passport::Passport> {
    let mut passport: passport::Passport = passport::Passport::default();
    let split_regex = Regex::new(r"\s").unwrap();

    for key_value_pair in split_regex
        .split(passport_data)
        .map(|pair| pair.trim())
        .filter(|pair| !pair.is_empty())
    {
        let splitted = key_value_pair.split(':');
        let key = splitted.clone().nth(0).unwrap();
        let value = splitted.clone().nth(1).unwrap();
        passport[key] = Some(String::from(value));
    }

    for required_field in ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter() {
        if passport[required_field].is_none() {
            return None;
        }
    }

    return Some(passport);
}

fn part1(input: &Vec<String>) -> i32 {
    let mut valid_count = 0;

    for passport_data in input {
        let passport = parse_passport(passport_data);
        if passport.is_some() {
            valid_count += 1;
        }
    }

    return valid_count;
}

fn part2(input: &Vec<String>) -> i32 {
    let mut valid_count = 0;

    for passport_data in input {
        let passport = parse_passport(passport_data);
        if passport.is_some() && passport.unwrap().is_valid() {
            valid_count += 1;
        }
    }

    return valid_count;
}

pub fn solve() {
    let input = utils::file::read_strings_sep(PathBuf::from(file!()), "\n\n");

    println!("Day 4 Part 1: {}", part1(&input));
    println!("Day 4 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let example_input = vec![
            String::from(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
            ),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"),
            String::from(
                "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
            ),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"),
        ];
        assert_eq!(part1(&example_input), 2);
        let file_input = utils::file::read_strings_sep(PathBuf::from(file!()), "\n\n");
        assert_eq!(part1(&file_input), 213);
    }

    #[test]
    fn part2_works() {
        let example_invalid = vec![
            String::from(
                "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            ),
            String::from(
                "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946",
            ),
            String::from(
                "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            ),
            String::from("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007"),
        ];
        let example_valid = vec![
            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f"),
            String::from(
                "eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            ),
            String::from(
                "hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022",
            ),
            String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
        ];

        assert_eq!(part2(&example_valid), 4);
        assert_eq!(part2(&example_invalid), 0);

        let file_input = utils::file::read_strings_sep(PathBuf::from(file!()), "\n\n");
        assert_eq!(part2(&file_input), 147);
    }
}
