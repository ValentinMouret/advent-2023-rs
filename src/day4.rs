use std::{collections::HashMap, u64};

fn parse_nums(nums: &str) -> Vec<u64> {
    nums.split(" ")
        .filter(|n| !n.is_empty())
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn parse_line(l: &str) -> (Vec<u64>, Vec<u64>) {
    let right_side = l.split(": ").nth(1).unwrap();
    let matches: Vec<&str> = right_side.split(" | ").collect();
    (parse_nums(matches[0]), parse_nums(matches[1]))
}

const TWO: u64 = 2;

pub fn part1(i: &str) -> u64 {
    i.lines()
        .map(parse_line)
        .map(|(winning, hand)| hand.into_iter().filter(|c| winning.contains(c)).count())
        .map(|c| {
            if c == 0 {
                0
            } else {
                let x: u64 = c.try_into().unwrap();
                TWO.pow((x - 1).try_into().unwrap())
            }
        })
        .sum()
}

pub fn part2(i: &str) -> u64 {
    let mut card_instances: HashMap<usize, u64> = HashMap::new();
    let mut card_copies: HashMap<usize, u64> = HashMap::new();

    let matches = i
        .lines()
        .map(parse_line)
        .map(|(winning, hand)| hand.into_iter().filter(|c| winning.contains(c)).count())
        .enumerate();

    for (i, m) in matches {
        card_instances.insert(i, 1);

        if m > 0 {
            for j in 0..m {
                let current_card_copies = card_copies.get(&i).or_else(|| Some(&0)).unwrap();
                *card_copies.entry(i + j + 1).or_insert(0) += 1 * (current_card_copies + 1);
            }
        }
    }

    card_copies.values().sum::<u64>() + card_instances.values().sum::<u64>()
}

#[cfg(test)]
mod test {
    use crate::day4::{part1, part2};

    use super::parse_line;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part2(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_parse_line() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let parsed = parse_line(input);
        assert_eq!(
            parsed,
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
        )
    }
}
