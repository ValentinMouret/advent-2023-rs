fn parse_seeds_1(l: &str) -> Vec<u32> {
    l.split(" ")
        .filter_map(|l| match l.parse::<u32>() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
struct Step {
    destination: u32,
    source: u32,
    size: u32,
}

impl Step {
    fn is_in_source(&self, x: u32) -> bool {
        self.source <= x && x <= self.source + self.size
    }

    fn is_in_destination(&self, x: u32) -> bool {
        self.destination <= x && x <= self.destination + self.size
    }

    fn source_to_destination(&self, x: u32) -> Option<u32> {
        if self.is_in_source(x) {
            Some(self.destination + x - self.source)
        } else {
            None
        }
    }

    fn destination_to_source(&self, x: u32) -> Option<u32> {
        if self.is_in_destination(x) {
            Some(self.source + x - self.destination)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Stage {
    steps: Vec<Step>,
}

impl Stage {
    fn forward(&self, x: u32) -> u32 {
        (&self.steps)
            .into_iter()
            .find_map(|s| s.source_to_destination(x))
            .or_else(|| Some(x))
            .unwrap()
    }

    fn backward(&self, x: u32) -> u32 {
        (&self.steps)
            .into_iter()
            .rev()
            .find_map(|s| s.destination_to_source(x))
            .or_else(|| Some(x))
            .unwrap()
    }
}

fn parse_step(l: &str) -> Step {
    let vals: Vec<u32> = l.split(" ").map(|n| n.parse::<u32>().unwrap()).collect();
    Step {
        destination: vals[0],
        source: vals[1],
        size: vals[2],
    }
}

fn parse_stage(i: &str) -> Stage {
    let mut lines = i.lines().into_iter();
    lines.next();
    Stage {
        steps: lines.map(parse_step).collect(),
    }
}

fn parse_input_1(i: &str) -> (Vec<u32>, Vec<Stage>) {
    let mut blocks = i.split("\n\n");
    let seeds = parse_seeds_1(blocks.next().unwrap());

    (seeds, blocks.map(parse_stage).collect())
}

pub fn part1(i: &str) -> u32 {
    let (seeds, stages) = parse_input_1(i);

    seeds
        .into_iter()
        .map(|seed| {
            stages
                .clone()
                .into_iter()
                .fold(seed, |acc, stage| stage.forward(acc))
        })
        .min()
        .unwrap()
}

#[derive(Clone)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, x: u32) -> bool {
        self.start <= x && x <= self.end
    }
}

fn parse_seeds_2(i: &str) -> Vec<Range> {
    parse_seeds_1(i)
        .chunks(2)
        .filter_map(|slice| match slice {
            [start, size] => Some(Range {
                start: *start,
                end: start + size,
            }),
            _ => None,
        })
        .collect()
}

fn parse_input_2(i: &str) -> (Vec<Range>, Vec<Stage>) {
    let mut blocks = i.split("\n\n");
    let seeds = parse_seeds_2(blocks.next().unwrap());

    (seeds, blocks.map(parse_stage).collect())
}

pub fn part2(i: &str) -> u32 {
    let (seeds, stages) = parse_input_2(i);

    let mut y: u32 = 0;
    loop {
        let x = stages
            .clone()
            .into_iter()
            .rev()
            .fold(y, |acc, stage| stage.backward(acc));
        if seeds.clone().into_iter().find(|s| s.contains(x)).is_some() {
            return y;
        }
        y += 1;
    }
}

#[cfg(test)]
mod test {
    use crate::day5::{parse_input_1, parse_seeds_1, parse_stage, part1, part2, Stage, Step};

    #[test]
    fn test_parse_seeds_1() {
        let i = "seeds: 79 14 55 13";
        assert_eq!(parse_seeds_1(i), vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_stage() {
        let i = "seed-to-soil map:
50 98 2
52 50 48";
        let stage = parse_stage(i);
        assert_eq!(
            stage,
            Stage {
                steps: vec![
                    Step {
                        destination: 50,
                        source: 98,
                        size: 2,
                    },
                    Step {
                        destination: 52,
                        source: 50,
                        size: 48
                    }
                ]
            }
        );

        assert_eq!(stage.forward(3), 3);
        assert_eq!(stage.forward(98), 50);
        assert_eq!(stage.forward(99), 51);

        assert_eq!(stage.backward(3), 3);
        assert_eq!(stage.backward(50), 98);
        assert_eq!(stage.backward(51), 99);
    }

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_parse_input_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48";
        let parsed = parse_input_1(input);
        assert_eq!(parsed.0, vec![79, 14, 55, 13]);
        assert_eq!(
            parsed.1,
            vec![Stage {
                steps: vec![
                    Step {
                        destination: 50,
                        source: 98,
                        size: 2,
                    },
                    Step {
                        destination: 52,
                        source: 50,
                        size: 48
                    }
                ]
            }]
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }
}
