use std::fs;

mod day1;
mod day2;

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
        "day1" => {
            let res = if part == Some(&"part2".to_string()) {
                day1::part2(&puzzle_input)
            } else {
                day1::part1(&puzzle_input)
            };
            println!("{}", res);
        }
        "day2" => {
            let res = if part == Some(&"part2".to_string()) {
                day2::part2(&puzzle_input)
            } else {
                day2::part1(&puzzle_input)
            };
            println!("{}", res);
        }
        _ => eprintln!("Invalid day"),
    }
}
