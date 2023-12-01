use crate::day::Day;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Day1 {}

impl Day for Day1 {
    fn part1(&self, input_path: &Path) {
        let input = parse_input(input_path, extract_num_simple).unwrap();
        let result: u32 = input.iter().sum();
        println!("Sum: {}", result);
    }

    fn part2(&self, input_path: &Path) {
        let digit_values: HashMap<&str, u32> = HashMap::from([
            ("0", 0),
            ("zero", 0),
            ("1", 1),
            ("one", 1),
            ("2", 2),
            ("two", 2),
            ("3", 3),
            ("three", 3),
            ("4", 4),
            ("four", 4),
            ("5", 5),
            ("five", 5),
            ("6", 6),
            ("six", 6),
            ("7", 7),
            ("seven", 7),
            ("8", 8),
            ("eight", 8),
            ("9", 9),
            ("nine", 9),
        ]);
        let input = parse_input(input_path, extract_num_complex(digit_values)).unwrap();
        let result: u32 = input.iter().sum();
        println!("Sum: {}", result);
    }
}

type Part1Input = Vec<u32>;

fn extract_num_simple(line: &str) -> u32 {
    let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
    let num_string = format!("{}{}", first_digit, last_digit);
    let num = num_string
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Cannot parse number"));
    return num.unwrap();
}

fn extract_num_complex(digit_values: HashMap<&str, u32>) -> Box<dyn Fn(&str) -> u32 + '_> {
    return Box::new(move |line| -> u32 {
        // for each digit string we search for its first and last index in line
        // then we take the value of first string and last one in each str
        let digit_indices: Vec<(&str, Option<usize>, Option<usize>)> = digit_values
            .iter()
            .map(|(value, _)| {
                let first_idx = line.find(value);
                let last_idx = line.rfind(value);
                (*value, first_idx, last_idx)
            })
            .collect();
        let min_digit = digit_indices
            .iter()
            .filter(|(_, idx, _)| idx.is_some())
            .min_by_key(|(_, idx, _)| idx.unwrap_or(usize::MAX));
        let max_digit = digit_indices
            .iter()
            .filter(|(_, idx, _)| idx.is_some())
            .max_by_key(|(_, _, idx)| idx.unwrap_or(usize::MIN));
        let first_value = digit_values[min_digit.unwrap().0];
        let last_value = digit_values[max_digit.unwrap().0];
        first_value * 10 + last_value
    });
}

fn parse_input<F>(input_path: &Path, extractor: F) -> io::Result<Part1Input>
where
    F: Fn(&str) -> u32,
{
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut result = Vec::new();
    for line in buf_reader.lines() {
        let line = line?;
        let num = extractor(&line);
        result.push(num);
    }
    return Ok(result);
}
