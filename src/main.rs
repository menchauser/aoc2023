use std::{collections::HashMap, env, path::Path, process::exit};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

type PartFn = fn(&Path) -> ();

fn main() {
    let days2: HashMap<&str, Vec<PartFn>> = HashMap::from([
        ("day1", vec![day1::part1, day1::part2]),
        ("day2", vec![day2::part1, day2::part2]),
        ("day3", vec![day3::part1, day3::part2]),
        ("day4", vec![day4::part1, day4::part2]),
        ("day5", vec![day5::part1, day5::part2, day5::part3]),
    ]);

    // Program arguments:
    //  rust-aoc <day> <part> <input-file>
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <runner> <part> <path-to-input>");
        println!("Example: rust-aoc day1 part1 test.txt");
        exit(1);
    }

    let day_parts = days2
        .get(&args[1].as_str())
        .unwrap_or_else(|| panic!("I don't know the day {}", &args[1]));
    let part: usize = if args[2].starts_with("part") {
        &args[2][4..args[2].len()]
    } else {
        args[2].as_str()
    }
    .parse::<usize>()
    .map(|p| p - 1)
    .unwrap_or_else(|_| panic!("Could not parse part: it should be a number"));
    let path = Path::new(&args[3]);

    if part >= day_parts.len() {
        println!(
            "Unknown part: {}, that day has only {} parts",
            (part + 1),
            day_parts.len()
        );
        exit(1);
    }
    let part_fn = day_parts[part];
    part_fn(path);
}
