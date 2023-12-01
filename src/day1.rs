use crate::day::Day;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, input_path: &Path) {
        let input = parse_input(input_path).unwrap();
        let result: u32 = input.iter().sum();
        println!("Sum: {}", result);
    }

    fn part2(&self, _input_path: &Path) {
        todo!()
    }
}

type Part1Input = Vec<u32>;

fn parse_input(input_path: &Path) -> io::Result<Part1Input> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut result = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
        let num_string = format!("{}{}", first_digit, last_digit);
        let num: u32 = num_string
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Cannot parse number"))?;
        result.push(num);
    }
    return Ok(result);
}
