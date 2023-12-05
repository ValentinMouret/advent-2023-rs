use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day> [part]");
        std::process::exit(1);
    }

    let day: &str = args[1].as_str();
    let part = args.get(2);
    let puzzle_input_path = format!("inputs/{}.txt", day);
    let puzzle_input = fs::read_to_string(puzzle_input_path).unwrap();

    // todo: refactor
    match day {
        "day1" => handle_day(day1::part1, day1::part2, &puzzle_input, part),
        "day2" => handle_day(day2::part1, day2::part2, &puzzle_input, part),
        "day3" => handle_day(day3::part1, day3::part2, &puzzle_input, part),
        "day4" => handle_day(day4::part1, day4::part2, &puzzle_input, part),
        _ => eprintln!("Invalid day"),
    }
}

fn handle_day<F, G>(part1: F, part2: G, input: &str, part: Option<&String>)
where
    F: Fn(&str) -> u32,
    G: Fn(&str) -> u32,
{
    let res = if part == Some(&"part2".to_string()) {
        part2(input)
    } else {
        part1(input)
    };
    println!("{}", res);
}
