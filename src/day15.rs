use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let sequence = load_raw_input(input_path).unwrap();
    let result: u32 = sequence.split(",").map(hash).sum();
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    let input = load_raw_input(input_path).unwrap();
    let seq = parse_input(&input);
    eprintln!("Loaded op sequence:");
    eprintln!("{:?}", seq);
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for op in seq {
        match op {
            Operation::Add(label, focal_length) => {
                let target_box = &mut boxes[hash(label) as usize];
                if let Some(pos) = target_box.iter().position(|lens| lens.label == label) {
                    target_box[pos].focal_length = focal_length;
                } else {
                    target_box.push(Lens {
                        label,
                        focal_length,
                    })
                }
            }
            Operation::Remove(label) => {
                let target_box = &mut boxes[hash(label) as usize];
                if let Some(pos) = target_box.iter().position(|lens| lens.label == label) {
                    target_box.remove(pos);
                }
            }
        }
    }
    eprintln!("Resulting boxes after sequence:");
    for (idx, boxx) in boxes.iter().enumerate() {
        if !boxx.is_empty() {
            eprintln!("{:02} - {:?}", idx, boxx);
        }
    }
    let result: u32 = boxes
        .iter()
        .enumerate()
        .map(|(b_id, b)| {
            b.iter()
                .enumerate()
                .map(|(l_id, lens)| (b_id as u32 + 1) * (l_id as u32 + 1) * lens.focal_length)
                .sum::<u32>()
        })
        .sum();
    println!("Result: {}", result);
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |h, c| ((h + c as u32) * 17) % 256)
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

#[derive(Debug)]
enum Operation<'a> {
    // Add lens with given label and focal length
    Add(&'a str, u32),
    // Remove lens with given label
    Remove(&'a str),
}

fn parse_op(op_str: &str) -> Operation {
    if let Some(pos) = op_str.chars().position(|c| c == '-') {
        Operation::Remove(&op_str[0..pos])
    } else if let Some(pos) = op_str.chars().position(|c| c == '=') {
        Operation::Add(&op_str[0..pos], op_str[pos + 1..].parse().unwrap())
    } else {
        panic!("Unexpected op string: '{}'", op_str)
    }
}

fn parse_input(input: &str) -> Vec<Operation> {
    input.split(",").map(parse_op).collect()
}

fn load_raw_input(input_path: &Path) -> io::Result<String> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader.lines().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"))
    }
}
