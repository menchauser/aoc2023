use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    for (i, row) in input.iter().enumerate() {
        let found = row.iter().enumerate().find(|(col, c)| **c == 'S');
        if found.is_some() {
            eprintln!("Found S position: {}, {}", i, found.unwrap().0)
        }
    }
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

// Load 2d slice
fn load_input(input_path: &Path) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    lines
        .map(|line| line.map(|l| Vec::from_iter(l.chars())))
        .collect()
}
