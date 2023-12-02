use std::{collections::HashMap, env, path::Path, process::exit};

mod day1;
mod day2;

struct DayParts {
    part1: fn(&Path) -> (),
    part2: fn(&Path) -> (),
}

fn main() {
    let days: HashMap<String, DayParts> = HashMap::from([
        (
            "day1".to_string(),
            DayParts {
                part1: day1::part1,
                part2: day1::part2,
            },
        ),
        (
            "day2".to_string(),
            DayParts {
                part1: day2::part1,
                part2: day2::part2,
            },
        ),
    ]);

    // Program arguments:
    //  rust-aoc <day> <part> <input-file>
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <runner> <part> <path-to-input>");
        println!("Example: rust-aoc day1 part1 test.txt");
        exit(1);
    }

    let DayParts { part1, part2 } = days
        .get(&args[1])
        .unwrap_or_else(|| panic!("I don't know the day {}", &args[1]));
    let part: u8 = args[2]
        .parse()
        .unwrap_or_else(|_| panic!("Could not parse part: it should be 1 or 2"));
    let path = Path::new(&args[3]);
    match part {
        1 => part1(path),
        2 => part2(path),
        _ => {
            println!("Support only 1 and 2 parts");
            exit(1)
        }
    }
}
