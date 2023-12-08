// https://adventofcode.com/2023/day/1
//
// Part 1
// ------
// The input is a series of rows where each row contains at least one digit.
// Each line should be parsed to extract the first and last digits of the line
// and assembleinto a two digit number.
// The solution should be the sum of all numbers.
//
// Part 2
// ------
// The inputs can now have literal numbers, e.g. 'one', or 'eight',
// and they should be treated like any other number.

use std::collections::HashMap;

fn parse_row_1(row: &str) -> u64 {
    let digits: Vec<u64> = row
        .chars()
        .filter_map(|d| d.to_digit(10))
        .map(|n| n as u64)
        .collect();

    let (first_digit, last_digit) = match (digits.first(), digits.last()) {
        (Some(&f), Some(&s)) => (f, s),
        (Some(&f), None) => (f, f),
        _ => {
            eprintln!("found no numbers in the row");
            (0, 0)
        }
    };

    first_digit * 10 + last_digit
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(parse_row_1).fold(0, |acc, n| acc + n)
}

const KEY_VALUE_PAIRS: &[(&str, u64)] = &[
    ("one", 1),
    ("two", 2),
    ("thre", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn parse_row_2(row: &str, map: &HashMap<&str, u64>) -> u64 {
    let number_idx = map.into_iter().flat_map(|(number_str, number_value)| {
        vec![
            (number_value, row.find(number_str)),
            (number_value, row.rfind(number_str)),
        ]
    });
    let mut digits: Vec<(&u64, Option<usize>)> =
        number_idx.filter(|(_, idx)| idx.is_some()).collect();
    digits.sort_by(|(_, idx1), (_, idx2)| idx1.cmp(idx2));

    let (first_digit, last_digit) = match (digits.first(), digits.last()) {
        (Some((&f, _)), Some((&l, _))) => (f, l),
        (Some((&f, _)), None) => (f, f),
        _ => (0, 0),
    };

    first_digit * 10 + last_digit
}

pub fn part2(input: &str) -> u64 {
    let mapper: HashMap<&str, u64> = KEY_VALUE_PAIRS.iter().copied().collect();
    input
        .lines()
        .map(|row| parse_row_2(row, &mapper))
        .fold(0, |acc, n| acc + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row_1() {
        let test_row = "1abc2";
        let parsed = parse_row_1(test_row);
        assert_eq!(parsed, 12);
    }

    #[test]
    fn test_part1() {
        let test_input = "1abc2
1abc";
        assert_eq!(part1(test_input), 23);
    }

    #[test]
    fn test_parse_row_2() {
        let test_row = "eightwo3";
        let mapper: HashMap<&str, u64> = KEY_VALUE_PAIRS.iter().copied().collect();
        let parsed: u64 = parse_row_2(test_row, &mapper);
        assert_eq!(parsed, 83);
    }

    #[test]
    fn test_part_2() {
        let test_input = "1abc2
1abc
eightwo3";
        assert_eq!(part2(test_input), 106);
    }
}
