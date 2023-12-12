use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    eprintln!("Loaded input:");
    for c in &input {
        eprintln!("{}", c);
    }
    let result: u32 = input.iter().map(count_arrangements).sum();
    println!("Result: {}", result)
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

fn count_arrangements(cond: &Condition) -> u32 {
    0
}

struct Condition {
    damaged_groups: Vec<u32>,
    record: String,
}

impl Condition {
    fn from(cond_str: &str) -> Self {
        let mut splits = cond_str.split_whitespace();
        let record = splits.next().unwrap();
        let groups: Vec<_> = splits
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Self {
            damaged_groups: groups,
            record: record.to_string(),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group_str = self
            .damaged_groups
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{} {}", &self.record, group_str)
    }
}

fn load_input(input_path: &Path) -> io::Result<Vec<Condition>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    lines
        .map(|line| line.map(|l| Condition::from(&l)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arrangements() {
        let cond = Condition::from("???.### 1,1,3");
        eprintln!("Testing condition {}", cond);
        assert_eq!(1, count_arrangements(&cond));
    }
}
