use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    println!("Loaded patterns:");
    for pattern in &input {
        println!("{}", pattern);
    }
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

struct Pattern {
    data: Vec<Vec<char>>,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

type Input = Vec<Pattern>;

fn load_input(input_path: &Path) -> io::Result<Input> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    todo!()
}
