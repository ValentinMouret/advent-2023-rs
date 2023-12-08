#[derive(Debug, PartialEq)]
struct Symbol {
    row: usize,
    column: usize,
    is_star: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Num<T> {
    value: T,
    start_column: usize,
    end_column: usize,
}

struct Engine {
    symbols: Vec<Symbol>,
    rows: Vec<Vec<Num<u64>>>,
}

impl Engine {
    fn find_neigbors(&self, s: &Symbol) -> Vec<u64> {
        let mut matches: Vec<u64> = Vec::new();
        let last_row_id = self.rows.len();

        let mut adjacent_row_ids: Vec<usize> = Vec::new();

        if s.row > 0 {
            adjacent_row_ids.push(s.row - 1);
        }

        if s.row + 1 <= last_row_id {
            adjacent_row_ids.push(s.row + 1);
        }

        adjacent_row_ids.push(s.row);

        for row_id in adjacent_row_ids {
            for num in &self.rows[row_id] {
                if ((num.start_column <= s.column) && (num.end_column >= s.column))
                    || ((num.start_column <= s.column - 1) && (num.end_column >= s.column - 1))
                    || ((num.start_column <= s.column + 1) && (num.end_column >= s.column + 1))
                {
                    matches.push(num.value);
                }
            }
        }

        matches
    }
}

fn parse_line(line: &str, line_nb: usize) -> (Vec<Num<u64>>, Vec<Symbol>) {
    let mut nums: Vec<Num<String>> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (i, c) in line.chars().enumerate() {
        match c {
            '0'..='9' => {
                nums.push(Num {
                    value: c.to_string(),
                    start_column: i,
                    end_column: i,
                });
            }
            '.' => (),
            s => {
                symbols.push(Symbol {
                    row: line_nb,
                    column: i,
                    is_star: s == '*',
                });
            }
        }
    }

    let mut _numbers: Vec<Num<String>> = Vec::new();
    for n in nums {
        match _numbers.pop() {
            None => _numbers.push(n),
            Some(num) => {
                if num.end_column + 1 == n.end_column {
                    _numbers.push(Num::<String> {
                        value: format!("{}{}", num.value, n.value),
                        start_column: num.start_column,
                        end_column: num.end_column + 1,
                    });
                } else {
                    _numbers.push(num);
                    _numbers.push(n);
                }
            }
        }
    }

    let numbers: Vec<Num<u64>> = _numbers
        .into_iter()
        .map(|f| Num::<u64> {
            value: f.value.parse::<u64>().unwrap(),
            start_column: f.start_column,
            end_column: f.end_column,
        })
        .collect();

    (numbers, symbols)
}

fn parse_input(input: &str) -> Engine {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| parse_line(l, i.try_into().unwrap()))
        .fold(
            Engine {
                symbols: Vec::new(),
                rows: Vec::new(),
            },
            |mut acc, mut n| {
                acc.symbols.append(&mut n.1);
                acc.rows.push(n.0);
                acc
            },
        )
}

pub fn part1(input: &str) -> u64 {
    let engine = parse_input(input);

    let mut matches: Vec<u64> = Vec::new();

    for symbol in &engine.symbols {
        let mut neighbors = engine.find_neigbors(&symbol);
        matches.append(&mut neighbors);
    }

    matches.into_iter().sum()
}

pub fn part2(input: &str) -> u64 {
    let engine = parse_input(input);

    let mut gear_ratios: Vec<u64> = Vec::new();

    for symbol in &engine.symbols {
        if !symbol.is_star {
            continue;
        }
        let neighbors = engine.find_neigbors(&symbol);

        if neighbors.len() == 2 {
            gear_ratios.push(neighbors[0] * neighbors[1]);
        }
    }

    gear_ratios.into_iter().sum()
}

#[cfg(test)]
mod test {
    use crate::day3::{parse_line, part1, part2, Num, Symbol};

    #[test]
    fn test_solve1() {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = part1(test_input);
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_solve2() {
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = part2(test_input);
        assert_eq!(res, 467835);
    }

    #[test]
    fn test_parse_line() {
        let test_input = "467..114..";
        let result = parse_line(test_input, 0);
        assert_eq!(
            result.0,
            vec![
                Num::<u64> {
                    value: 467,
                    start_column: 0,
                    end_column: 2,
                },
                Num::<u64> {
                    value: 114,
                    start_column: 5,
                    end_column: 7,
                }
            ]
        );
        assert_eq!(result.1, Vec::new());
    }

    #[test]
    fn test_parse_line_with_symbol() {
        let test_input = "617*......";
        let result = parse_line(test_input, 4);
        assert_eq!(
            result.0,
            vec![Num::<u64> {
                value: 617,
                start_column: 0,
                end_column: 2,
            },]
        );
        assert_eq!(
            result.1,
            vec![
                (Symbol {
                    row: 4,
                    column: 3,
                    is_star: true
                })
            ]
        );
    }
}
