struct Problem {
    time: u64,
    distance: u64,
}

fn parse_nums(l: &str) -> Vec<u64> {
    l.split(" ")
        .filter_map(|l| match l.parse() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect()
}

fn parse_nums_2(l: &str) -> u64 {
    String::from_iter(l.split(" ").filter(|l| match l.parse::<u64>() {
        Ok(_) => true,
        _ => false,
    }))
    .parse()
    .unwrap()
}

fn solve(problem: Problem) -> u64 {
    // distance = t * (t0 - t)
    //     t * (t0 -t) - d0 = 0
    // <=> t*t0 -t^2 - d0 = 0
    // <=> t^2 - t*t0 + d0 = 0
    // delta = t0^2 - 4 * d0
    // x1/2  = t0 ± sqrt(4 * d0) / 2
    let delta = problem.time.pow(2) - (4 * problem.distance);
    let x1: f64 = ((problem.time as f64) - f64::sqrt(delta as f64)) / 2.0;
    let x2: f64 = ((problem.time as f64) + f64::sqrt(delta as f64)) / 2.0;

    (x2.ceil() as u64 - 1) - (x1.floor() as u64 + 1) + 1
}

pub fn part1(l: &str) -> u32 {
    let res: Vec<Vec<u64>> = l.lines().take(2).map(parse_nums).collect();
    let times = res[0].clone();
    let distances = res[1].clone();

    let res: u64 = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Problem { time, distance })
        .map(solve)
        .product();

    res as u32
}

pub fn part2(l: &str) -> u32 {
    let res: Vec<u64> = l.lines().take(2).map(parse_nums_2).collect();
    let time = res[0];
    let distance = res[1];
    solve(Problem { time, distance }) as u32
}

#[cfg(test)]
mod test {
    use crate::day6::{part1, part2, solve, Problem};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_solve() {
        let p = Problem {
            time: 7,
            distance: 9,
        };
        assert_eq!(solve(p), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 71503);
    }
}
