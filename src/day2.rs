use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Set {
    green: u32,
    blue: u32,
    red: u32,
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_line(line: &str) -> Game {
    let game_regex: Regex = Regex::new(r"^Game (\d+):").unwrap();
    let blue_regex: Regex = Regex::new(r"(\d+) blue").unwrap();
    let red_regex: Regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex: Regex = Regex::new(r"(\d+) green").unwrap();

    let game_id: &str = &game_regex.captures(line).unwrap()[1];
    let sets: Vec<Set> = line
        .split(";")
        .map(|s| {
            let blue: u32 = match blue_regex.captures(s) {
                Some(caps) => caps[1].parse::<u32>().unwrap(),
                None => 0,
            };
            let green: u32 = match green_regex.captures(s) {
                Some(caps) => caps[1].parse::<u32>().unwrap(),
                None => 0,
            };
            let red: u32 = match red_regex.captures(s) {
                Some(caps) => caps[1].parse::<u32>().unwrap(),
                None => 0,
            };
            Set { green, blue, red }
        })
        .collect();
    Game {
        id: game_id.parse().unwrap(),
        sets,
    }
}

fn is_possible(game: &Game, constraints: &Set) -> bool {
    game.sets
        .clone()
        .into_iter()
        .find(|s| {
            s.blue > constraints.blue || s.red > constraints.red || s.green > constraints.green
        })
        .is_none()
}

pub fn part1(input: &str) -> u32 {
    let constraints = Set {
        green: 13,
        blue: 14,
        red: 12,
    };
    input
        .lines()
        .map(parse_line)
        .filter(|g| is_possible(g, &constraints))
        .map(|g| g.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .map(|g| {
            g.sets.clone().into_iter().fold(
                Set {
                    green: 0,
                    red: 0,
                    blue: 0,
                },
                |acc, s| Set {
                    green: acc.green.max(s.green),
                    red: acc.red.max(s.red),
                    blue: acc.blue.max(s.blue),
                },
            )
        })
        .map(|s| s.green * s.red * s.blue)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::day2::{parse_line, part1, Set};

    #[test]
    fn test_parse_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_line(line);
        assert_eq!(game.id, 1);
        assert_eq!(
            game.sets,
            vec![
                Set {
                    green: 0,
                    blue: 3,
                    red: 4
                },
                Set {
                    green: 2,
                    blue: 6,
                    red: 1
                },
                Set {
                    green: 2,
                    blue: 0,
                    red: 0
                }
            ]
        );
    }
}
