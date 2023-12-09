use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    for row in &input {
        eprintln!("{:?}", row);
    }
    eprintln!("Run simulation");
    let result: i32 = input.iter().map(|row| row_prediction(row)).sum();
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    for row in &input {
        eprintln!("{:?}", row);
    }
    eprintln!("Run simulation");
    let result: i32 = input
        .iter()
        .map(|row| {
            let rev_row: Vec<i32> = row.iter().rev().map(|n| *n).collect();
            row_prediction(&rev_row)
        })
        .sum();
    println!("Result: {}", result);
}

fn row_prediction(row: &Vec<i32>) -> i32 {
    let mut collected_diffs: Vec<Vec<i32>> = vec![row.clone()];
    let mut cur_row = row;
    loop {
        let diffs: Vec<i32> = cur_row.windows(2).map(|w| w[1] - w[0]).collect();
        eprintln!("Diffs: {:?}", diffs);
        if diffs.iter().all(|d| *d == 0) {
            eprintln!("Reached zero diffs!");
            break;
        } else {
            collected_diffs.push(diffs);
            cur_row = collected_diffs.last().unwrap();
        }
    }
    // now we go back: we get last value from next vector and add it to the last value of our vector
    eprintln!("{:?}", &collected_diffs);
    // predictions go from last to first
    let mut row_predictions: Vec<i32> = vec![0];
    // let's start with last value
    // predictions.push(*collected_diffs.last().unwrap().last().unwrap());
    for row in collected_diffs.iter().rev() {
        row_predictions.push(row.last().unwrap() + row_predictions.last().unwrap());
    }
    eprintln!("Predictions: {:?}", row_predictions);
    *row_predictions.last().unwrap()
}

fn load_input(input_path: &Path) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    lines
        .map(|line| {
            line.map(|l| {
                l.split_whitespace()
                    .map(|nstr| nstr.parse().unwrap())
                    .collect()
            })
        })
        .collect()
}
