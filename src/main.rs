use std::fs;

mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day> [part]");
        std::process::exit(1);
    }

    let day: &str = args[1].as_str();
    let puzzle_input_path = format!("inputs/{}.txt", day);
    let puzzle_input = fs::read_to_string(puzzle_input_path).unwrap();

    // todo: refactor
    match args[1].as_str() {
        "day1" => {
            let res = if args.get(2) == Some(&"part2".to_string()) {
                day1::part2(&puzzle_input)
            } else {
                day1::part1(&puzzle_input)
            };
            println!("{}", res);
        }
        _ => eprintln!("Invalid day"),
    }
}
