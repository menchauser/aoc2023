use std::cmp::{max, min};
use std::io;
use std::io::BufRead;
use std::{fs::File, path::Path};

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    // first we scan for all possible number positions
    let num_positions = parse_num_positions(&input);
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
        let start_col = max(0, pos.start_col as i32 - 1) as usize;
        let end_col = min(input[pos.row].len(), pos.end_col + 1);
        let start_row = max(0, pos.row as i32 - 1) as usize;
        // pos.row + 2, because we need to scan the row next to the current one's
        let end_row = min(input.len(), pos.row + 2);
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
    let input = load_input(input_path).unwrap();
    let num_positions = parse_num_positions(&input);
    for line in &input {
        eprintln!("{}", line);
    }
    // scan rows for positions of gears
    // for each gear find adjacent nums
    let mut gear_ratios = Vec::new();
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.char_indices() {
            if c == '*' {
                // check adjacent numbers
                // first build window of surrounding idx'es
                let start_col = max(0, col as i32 - 1) as usize;
                // col + 2, because we need to scan the col next to the current one's
                let end_col = min(line.len(), col + 2);
                let start_row = max(0, row as i32 - 1) as usize;
                // row + 2, because we need to scan the row next to the current one's
                let end_row = min(input.len(), row + 2);
                eprintln!(
                    "Found '*' at {}, {}. Surrounding window rows: {}..{}, cols: {}..{}",
                    row, col, start_row, end_row, start_col, end_col
                );
                // how to check it? we need to scan list of numbers and check if any of them has row within given
                // and col within given
                let mut gear_nums = Vec::new();
                for pos in &num_positions {
                    if pos.row >= start_row
                        && pos.row < end_row
                        // does number begin in window
                        && (pos.start_col >= start_col && pos.start_col < end_col
                            // or ends in it
                            || pos.end_col - 1 >= start_col && pos.end_col - 1 < end_col)
                    {
                        eprintln!("Number {:?} overlaps with this gear", pos);
                        gear_nums.push(parse_number(&input, pos));
                    }
                }
                if gear_nums.len() == 2 {
                    let gear_ratio = gear_nums[0] * gear_nums[1];
                    eprintln!("This part is a gear! Ratio: {}", gear_ratio);
                    gear_ratios.push(gear_ratio);
                }
            }
        }
    }
    let result: u32 = gear_ratios.iter().sum();
    println!("Result: {}", result);
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

fn parse_num_positions(input: &Vec<String>) -> Vec<NumPosition> {
    let mut num_positions: Vec<NumPosition> = Vec::new();
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
    num_positions
}

fn parse_number(input: &Vec<String>, position: &NumPosition) -> u32 {
    let row = &input[position.row];
    row[position.start_col..position.end_col].parse().unwrap()
}

fn load_input(input_path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    // return buf_reader.lines().map(|res| res.map(parse_game)).collect();
    return buf_reader.lines().collect();
}
