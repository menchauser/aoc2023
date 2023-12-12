use std::{env, path::Path, process::exit};

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[macro_export]
macro_rules! day {
    ($day:ident, $($fn:ident),*) => {
        (stringify!($day), vec![$($day::$fn),*])
    };
}

#[macro_export]
macro_rules! gen_days {
    ($($day:tt),*) => {
        {
            let map: std::collections::HashMap<&str, Vec<fn(&Path) -> ()>> = std::collections::HashMap::from([
                $(day!$day),*
            ]);
            map
        }
    };
}

fn main() {
    let days = gen_days!(
        (day1, part1, part2),
        (day2, part1, part2),
        (day3, part1, part2),
        (day4, part1, part2),
        (day5, part1, part2, part3, part4),
        (day6, part1, part2),
        (day7, part1, part2),
        (day8, part1, part2, part3),
        (day9, part1, part2),
        (day10, part1, part2),
        (day11, part1, part2),
        (day12, part1, part2)
    );

    // Program arguments:
    //  rust-aoc <day> <part> <input-file>
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <runner> <part> <path-to-input>");
        println!("Example: rust-aoc day1 part1 test.txt");
        exit(1);
    }

    let day_parts = days
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
