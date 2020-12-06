use crate::utils;
use std::path::PathBuf;

fn get_seat_id(location_data: &String) -> i32 {
    return i32::from_str_radix(
        &location_data
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"),
        2,
    )
    .unwrap();
}

fn part1(input: &Vec<String>) -> i32 {
    let mut highest_seat_id = 0;

    for seat_info in input {
        let seat_id = get_seat_id(seat_info);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    return highest_seat_id;
}

fn part2(input: &Vec<String>) -> i32 {
    let mut seats: Vec<i32> = (0..7 * 127).map(i32::from).collect();

    for seat_info in input {
        let seat_id = get_seat_id(seat_info);
        seats.remove(seats.binary_search(&seat_id).unwrap());
    }

    let mut most_front = 0;
    while seats.binary_search(&most_front).is_ok() {
        seats.remove(0);
        most_front += 1;
    }
    let mut most_back = seats[seats.len() - 1];
    while seats.binary_search(&most_back).is_ok() {
        seats.remove(seats.len() - 1);
        most_back -= 1;
    }

    assert_eq!(seats.len(), 1);

    return seats[0];
}

pub fn solve() {
    let input = utils::file::read_strings(PathBuf::from(file!()));

    println!("Day 5 Part 1: {}", part1(&input));
    println!("Day 5 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        assert_eq!(part1(&vec!(String::from("BFFFBBFRRR"))), 567);
        assert_eq!(part1(&vec!(String::from("FFFBBBFRRR"))), 119);
        assert_eq!(part1(&vec!(String::from("BBFFBBFRLL"))), 820);
        let file_input = utils::file::read_strings(PathBuf::from(file!()));
        assert_eq!(part1(&file_input), 871);
    }

    #[test]
    fn part2_works() {
        let file_input = utils::file::read_strings(PathBuf::from(file!()));
        assert_eq!(part2(&file_input), 640);
    }
}
