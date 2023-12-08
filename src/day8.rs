use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct Branch {
    left: String,
    right: String,
}

#[derive(Debug, PartialEq)]
struct Game {
    move_sequence: Vec<Move>,
    move_cursor: usize,
    nodes: HashMap<String, Branch>,
    current_node: String,
}

impl Game {
    fn next(&mut self) -> &Self {
        let current_node = self.nodes.get(&self.current_node).unwrap();
        let next_node = match &self.move_sequence[self.move_cursor] {
            Move::Left => &current_node.left,
            Move::Right => &current_node.right,
        };

        if self.move_cursor == self.move_sequence.len() - 1 {
            self.move_cursor = 0;
        } else {
            self.move_cursor += 1;
        }

        self.current_node = next_node.clone();

        self
    }
}

fn parse_input(i: &str) -> Game {
    let mut lines = i.lines();
    let move_sequence = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Move::Left,
            _ => Move::Right,
        })
        .collect();
    lines.next();

    let re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    let nodes: HashMap<String, Branch> = HashMap::from_iter(lines.map(|l| {
        let caps = re.captures(l).unwrap();
        let node = caps.get(1).unwrap().as_str().to_string();
        let branch = Branch {
            left: caps.get(2).unwrap().as_str().to_string(),
            right: caps.get(3).unwrap().as_str().to_string(),
        };
        (node, branch)
    }));

    Game {
        move_sequence,
        current_node: String::from("AAA"),
        move_cursor: 0,
        nodes,
    }
}

pub fn part1(i: &str) -> u64 {
    let mut game = parse_input(i);
    let mut moves = 0;
    while game.current_node != String::from("ZZZ") {
        game.next();
        moves += 1;
    }
    moves
}

pub fn part2(i: &str) -> u64 {
    let game = parse_input(i);
    let starting_nodes: Vec<String> = game
        .nodes
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| n.clone())
        .collect();

    let mut moves: Vec<u64> = Vec::new();
    starting_nodes.into_iter().for_each(|n| {
        let mut g = Game {
            move_sequence: game.move_sequence.clone(),
            move_cursor: 0,
            nodes: game.nodes.clone(),
            current_node: n,
        };

        let mut _moves: u64 = 0;
        while !g.current_node.ends_with("Z") {
            g.next();
            _moves += 1;
        }

        moves.push(_moves);
    });
    least_common_multiplier(moves)
}

fn least_common_multiplier(nums: Vec<u64>) -> u64 {
    let mut _nums = nums.clone();
    _nums.sort();
    let biggest: u64 = _nums.last().unwrap().clone();
    let others: Vec<u64> = _nums.into_iter().filter(|n| *n != biggest).collect();

    let mut i: u64 = 1;
    loop {
        let n = biggest * i;
        if (&others).into_iter().find(|m| n % *m != 0).is_none() {
            return i * biggest;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::day8::{least_common_multiplier, parse_input, part1, part2, Branch, Game, Move};

    const TEST_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            Game {
                move_sequence: vec![Move::Left, Move::Left, Move::Right],
                move_cursor: 0,
                current_node: String::from("AAA"),
                nodes: HashMap::from([
                    (
                        String::from("AAA"),
                        Branch {
                            left: String::from("BBB"),
                            right: String::from("BBB")
                        }
                    ),
                    (
                        String::from("BBB"),
                        Branch {
                            left: String::from("AAA"),
                            right: String::from("ZZZ")
                        }
                    ),
                    (
                        String::from("ZZZ"),
                        Branch {
                            left: String::from("ZZZ"),
                            right: String::from("ZZZ")
                        }
                    )
                ])
            }
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }

    #[test]
    fn test_part2() {
        let i = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(i), 6);
    }

    #[test]
    fn test_least_common_multiplier() {
        assert_eq!(least_common_multiplier(vec![2, 3, 7]), 42);
    }
}
