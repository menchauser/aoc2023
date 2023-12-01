use std::{collections::HashMap, env, path::Path, process::exit};

mod day;
mod day1;

fn main() {
    let mut days: HashMap<String, Box<dyn day::Day>> = HashMap::new();
    days.insert("day1".into(), Box::new(day1::Day1 {}));

    // Program arguments:
    //  rust-aoc <day> <part> <input-file>
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <runner> <part> <path-to-input>");
        println!("Example: rust-aoc day1 part1 test.txt");
        exit(1);
    }

    let day = days
        .get(&args[1])
        .unwrap_or_else(|| panic!("I don't know the day {}", &args[1]));
    let part: u8 = args[2]
        .parse()
        .unwrap_or_else(|_| panic!("Could not parse part: it should be 1 or 2"));
    let path = Path::new(&args[3]);
    match part {
        1 => day.part1(path),
        2 => day.part2(path),
        _ => {
            println!("Support only 1 and 2 parts");
            exit(1)
        }
    }
}
