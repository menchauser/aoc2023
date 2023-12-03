use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;
use std::{fs::File, path::Path};

pub fn part1(input_path: &Path) {
    let input = parse_input(input_path).unwrap();
    // easy approach:
    //  - iterate every string char by char
    //  - if encounter number - build a number
    //  - if number finished (., or end of line) - check adjacent chars
    // in practice:
    //  find first digit-char, remember position
    //  find last digit-char
    //  then check prev and next row for chars in interval (start - 1, end + 1)
    let mut num_positions: Vec<NumPosition> = Vec::new();
    // first we scan for all possible number positions
    for row_idx in 0..input.len() {
        let row = &input[row_idx];
        // we need current num start, end
        let mut num_start: Option<usize> = None;
        for (col_idx, c) in row.char_indices() {
            if c.is_digit(10) {
                if num_start.is_none() {
                    // first time we encounter number
                    num_start = Some(col_idx);
                }
            } else {
                if num_start.is_some() {
                    // number is finished
                    num_positions.push(NumPosition {
                        row: row_idx,
                        start_col: num_start.unwrap(),
                        end_col: col_idx,
                    });
                    num_start = None;
                }
            }
        }
        // the case for number at the end of the row
        if num_start.is_some() {
            // number is finished
            num_positions.push(NumPosition {
                row: row_idx,
                start_col: num_start.unwrap(),
                end_col: row.len(),
            });
        }
    }
    // debug: print all found numbers
    eprintln!("Input:");
    for row in &input {
        eprintln!("{}", row);
    }
    eprintln!("Found numbers: ");
    for num_pos in &num_positions {
        let row = &input[num_pos.row];
        let num_str = &row[num_pos.start_col..num_pos.end_col];
        eprintln!("{:?}: {}", num_pos, num_str)
    }

    // now we go over all num positions and check if they are surrounded by parts
    let mut part_numbers: Vec<u32> = Vec::new();
    'loop_by_positions: for pos in &num_positions {
        // let's build a range: which rows and cols to scan
        let start_col = if pos.start_col == 0 {
            pos.start_col
        } else {
            pos.start_col - 1
        };
        let end_col = if pos.end_col == (&input[pos.row]).len() {
            pos.end_col
        } else {
            pos.end_col + 1
        };
        let start_row = if pos.row == 0 { pos.row } else { pos.row - 1 };
        let end_row = if pos.row == input.len() - 1 {
            pos.row
        } else {
            pos.row + 2
        };
        eprintln!(
            "Position: {:?}\nscan rows: {}..{}, cols: {}..{}",
            pos, start_row, end_row, start_col, end_col
        );
        // now let's scan for part characters
        for row_idx in start_row..end_row {
            let row = &input[row_idx];
            eprintln!("Scanning string: {}", &row[start_col..end_col]);
            if row[start_col..end_col].chars().any(is_part) {
                part_numbers.push(parse_number(&input, pos));
                continue 'loop_by_positions;
            }
        }
    }
    eprintln!("Part numbers: {:?}", &part_numbers);
    println!("Result: {}", part_numbers.iter().sum::<u32>());
}

pub fn part2(input_path: &Path) {
    todo!()
}

#[derive(Debug)]
struct NumPosition {
    row: usize,
    start_col: usize,
    end_col: usize,
}

fn is_part(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

fn parse_number(input: &Vec<String>, position: &NumPosition) -> u32 {
    let row = &input[position.row];
    row[position.start_col..position.end_col].parse().unwrap()
}

fn parse_input(input_path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    // return buf_reader.lines().map(|res| res.map(parse_game)).collect();
    return buf_reader.lines().collect();
}
