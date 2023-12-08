use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    todo!()
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

fn load_input(input_path: &Path) -> HashMap<&str, (&str, &str)> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    todo!()
}
