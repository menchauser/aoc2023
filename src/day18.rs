use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use num::iter;

pub fn part1(input_path: &Path) {
    let dig_plan = load_input(input_path, parse_instruction_1).unwrap();
    let mut plan = draw_plan(&dig_plan);
    // now the naive approach is to build a 'picture' and then fill it horizontally
    eprintln!("Result field");
    for (i, row) in plan.iter().enumerate() {
        print!("{:04}  ", i);
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    // now let's find first 'inner' position and try to fill from it
    let start_point = find_inner_point(&plan);
    let mut next_points = VecDeque::from([start_point]);
    eprintln!("Found inside point: {:?}", start_point);
    while !next_points.is_empty() {
        let (i, j) = next_points.pop_front().unwrap();
        plan[i][j] = '#';
        // now let's try going in 4 directions
        let directions = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
        for (di, dj) in directions {
            let new_i = (i as i32 + di) as usize;
            let new_j = (j as i32 + dj) as usize;
            if plan[new_i][new_j] == '.' {
                plan[new_i][new_j] = '#';
                next_points.push_back((new_i, new_j));
            }
        }
    }
    eprintln!("Filled plan:");
    for (i, row) in plan.iter().enumerate() {
        print!("{:04}  ", i);
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    let result: usize = plan
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum();
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    let dig_plan = load_input(input_path, parse_instruction_1).unwrap();
    eprintln!("Plan:");
    for ds in &dig_plan {
        eprintln!("{}", ds);
    }
    eprintln!("Estimage plan size as keys of keys");
    let result: usize = dig_plan.iter().map(|ds| ds.meters).sum();
    eprintln!("{} pixels map to {} bytes", result, result * 24);
    let plan = draw_plan_sparse(&dig_plan);
    for row in &plan {
        println!("{:?}", row)
    }
    print_plan_sparse(&plan);

    // let mut plan = draw_plan(&dig_plan);
    // eprintln!("Result field");
    // for (i, row) in plan.iter().enumerate() {
    //     print!("{:06}  ", i);
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    // now let's find first 'inner' position and try to fill from it
    // let start_point = find_inner_point(&plan);
    // let mut next_points = VecDeque::from([start_point]);
    // println!("Found inside point: {:?}", start_point);
    // while !next_points.is_empty() {
    //     let (i, j) = next_points.pop_front().unwrap();
    //     plan[i][j] = '#';
    //     // now let's try going in 4 directions
    //     let directions = [(-1i32, 0i32), (0, 1), (1, 0), (0, -1)];
    //     for (di, dj) in directions {
    //         let new_i = (i as i32 + di) as usize;
    //         let new_j = (j as i32 + dj) as usize;
    //         if plan[new_i][new_j] == '.' {
    //             plan[new_i][new_j] = '#';
    //             next_points.push_back((new_i, new_j));
    //         }
    //     }
    // }
    // println!("Filled plan:");
    // for (i, row) in plan.iter().enumerate() {
    //     print!("{:06}  ", i);
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    // let result: usize = plan
    //     .iter()
    //     .map(|row| row.iter().filter(|c| **c == '#').count())
    //     .sum();
    // println!("Result: {}", result);
}

// Take list of instructions and return drawn map
fn draw_plan(instructions: &Vec<DigStep>) -> Vec<Vec<char>> {
    let mut plan: Vec<Vec<char>> = vec![vec!['.']];
    let mut row_idx: usize = 0;
    let mut col_idx: usize = 0;
    for step in instructions {
        eprintln!(
            "Processing step: {}, row_idx={}, col_idx={}",
            step, row_idx, col_idx
        );
        match step.dir {
            Direction::Up => {
                // we should insert (step.meters - row) rows up
                let needed_rows = step.meters as i32 - row_idx as i32;
                let needed_cols = plan[row_idx].len();
                if needed_rows > 0 {
                    plan.reserve(needed_rows as usize);
                    for _ in 0..needed_rows {
                        plan.insert(0, vec!['.'; needed_cols]);
                    }
                    // for every inserted row our current position is shifted forward
                    row_idx += needed_rows as usize;
                }
                for i in (row_idx - step.meters)..row_idx {
                    plan[i][col_idx] = '#';
                }
                // row should move from current row meters back
                row_idx = row_idx - step.meters;
            }
            Direction::Down => {
                // we should insert (step.meters - (len - row - 1)) rows down
                let needed_rows = step.meters as i32 - (plan.len() - row_idx - 1) as i32;
                let needed_cols = plan[row_idx].len();
                if needed_rows > 0 {
                    eprintln!("D: Reserving {} additional rows...", needed_rows);
                    plan.reserve(needed_rows as usize);
                    eprintln!("D: Reserved. Adding row values...");
                    let new_row = vec!['.'; needed_cols];
                    for i in 0..needed_rows {
                        plan.push(new_row.clone());
                    }
                    eprintln!("D: Added");
                }
                eprintln!("D: Filling down chars with '#':");
                // insert '#' chars
                for i in row_idx..(row_idx + step.meters + 1) {
                    plan[i][col_idx] = '#';
                }
                eprintln!("D: Filled");
                row_idx = row_idx + step.meters;
            }
            Direction::Left => {
                // we should insert (step.meters - col) cols left
                let needed_cols = step.meters as i32 - col_idx as i32;
                if needed_cols > 0 {
                    for _ in 0..needed_cols {
                        for row in plan.iter_mut() {
                            row.reserve(needed_cols as usize);
                            row.insert(0, '.');
                        }
                    }
                    // for every inserted col our current position is shifted right
                    col_idx += needed_cols as usize;
                }
                // insert '#' chars
                for j in (col_idx - step.meters)..col_idx {
                    plan[row_idx][j] = '#';
                }
                // col should move to the left
                col_idx = col_idx - step.meters;
            }
            Direction::Right => {
                // we should insert (step.meters - (len - col - 1)) cols right
                let needed_cols = step.meters as i32 - (plan[row_idx].len() - col_idx - 1) as i32;
                if needed_cols > 0 {
                    for _ in 0..needed_cols {
                        for row in plan.iter_mut() {
                            row.reserve(needed_cols as usize);
                            row.push('.');
                        }
                    }
                }
                // insert '#' chars
                for j in col_idx..(col_idx + step.meters) {
                    plan[row_idx][j] = '#';
                }
                col_idx = col_idx + step.meters;
            }
        }
    }
    plan
}

// Return list of lists: each row is represented by a list, each list contains (column, character)
fn draw_plan_sparse(instructions: &Vec<DigStep>) -> Vec<Vec<(usize, char)>> {
    let mut plan: Vec<Vec<(usize, char)>> = vec![vec![]];
    let mut row_idx: usize = 0;
    let mut col_idx: usize = 0;
    let mut max_height = 1;
    let mut max_width = 1;
    for step in instructions {
        eprintln!(
            "Processing step: {}, row_idx={}, col_idx={}",
            step, row_idx, col_idx
        );
        match step.dir {
            Direction::Up => {
                // we should insert (step.meters - row) rows up
                let needed_rows = step.meters as i32 - row_idx as i32;
                if needed_rows > 0 {
                    plan.reserve(needed_rows as usize);
                    for _ in 0..needed_rows {
                        // we need to insert 'empty' row before
                        plan.insert(0, vec![]);
                    }
                    // for every inserted row our current position is shifted forward
                    row_idx += needed_rows as usize;
                    max_height += needed_rows as usize;
                }
                // now insert an element in every new row
                for i in (row_idx - step.meters)..row_idx {
                    plan[i].push((col_idx, '#'));
                }
                // row should move from current row meters back
                row_idx = row_idx - step.meters;
            }
            Direction::Down => {
                // we should insert (step.meters - (len - row - 1)) rows down
                let needed_rows = step.meters as i32 - (max_height - row_idx - 1) as i32;
                if needed_rows > 0 {
                    plan.reserve(needed_rows as usize);
                    for _ in 0..needed_rows {
                        plan.push(vec![]);
                    }
                    max_height += needed_rows as usize;
                }
                // insert '#' chars
                for i in row_idx..(row_idx + step.meters + 1) {
                    plan[i].push((col_idx, '#'));
                }
                row_idx = row_idx + step.meters;
            }
            Direction::Left => {
                // we should insert (step.meters - col) cols left
                let needed_cols = step.meters as i32 - col_idx as i32;
                if needed_cols > 0 {
                    // for _ in 0..needed_cols {
                    //     for row in plan.iter_mut() {
                    //         row.reserve(needed_cols as usize);
                    //     }
                    // }
                    // for every inserted col our current position is shifted right
                    plan[row_idx].reserve(needed_cols as usize);
                    col_idx += needed_cols as usize;
                    max_width += needed_cols as usize;
                }
                // insert '#' chars
                for j in (col_idx - step.meters)..col_idx {
                    plan[row_idx].push((j, '#'));
                }
                // col should move to the left
                col_idx = col_idx - step.meters;
            }
            Direction::Right => {
                // we should insert (step.meters - (len - col - 1)) cols right
                let needed_cols = step.meters as i32 - (max_width - col_idx - 1) as i32;
                if needed_cols > 0 {
                    // for _ in 0..needed_cols {
                    //     for row in plan.iter_mut() {
                    //         row.reserve(needed_cols as usize);
                    //     }
                    // }
                    plan[row_idx].reserve(needed_cols as usize);
                    max_width += needed_cols as usize;
                }
                // insert '#' chars
                for j in col_idx..(col_idx + step.meters) {
                    plan[row_idx].push((j, '#'));
                }
                col_idx = col_idx + step.meters;
            }
        }
    }
    // now let's sort each row by col position
    for row in plan.iter_mut() {
        row.sort_by_key(|(col, _)| *col);
    }
    plan
}

fn print_plan_sparse(sparse_plan: &Vec<Vec<(usize, char)>>) {
    let max_width = sparse_plan
        .iter()
        .map(|row| row.last().map(|(col, _)| col).unwrap())
        .max()
        .unwrap();
    for row in sparse_plan {
        let mut cur_col = 0;
        for (col, c) in row {
            if *col > cur_col {
                print!("{}", ".".repeat(*col - cur_col));
            }
            print!("{}", c);
            cur_col = col + 1;
        }
        if cur_col < *max_width {
            print!("{}", ".".repeat(max_width - cur_col));
        }
        println!();
    }
}

// Find first point inside given plan
fn find_inner_point(plan: &Vec<Vec<char>>) -> (usize, usize) {
    // to do that we find first elements with following pattern. The coordinates of x are the ones we looking for.
    // 11
    // 1x
    for i in 0..plan.len() - 1 {
        for j in 0..plan[i].len() - 1 {
            let window = [
                plan[i][j],
                plan[i][j + 1],
                plan[i + 1][j],
                plan[i + 1][j + 1],
            ];
            if window == ['#', '#', '#', '.'] {
                return (i + 1, j + 1);
            }
        }
    }
    panic!("The plan has no inside points");
}

#[allow(dead_code)]
fn scan_fill(plan: &Vec<Vec<char>>) {
    // now we scan each row from left to right
    // let's try scanning following symbols by mask of 1s:
    // 11
    // 11
    // that will give us possibility to distinguish top or bottom horizontal line
    // now when we are on top or bottom horizontal line, we are not 'inside'
    // some examples of matched patterns
    // 10     00
    // 11 ... 10 - means that we went from L to 7 so we are inside
    //
    // 10     10
    // 11 ... 10 - means that we went from L to J so we are not inside
    //
    // 00     01
    // 01 ... 11
    let mut target_plan = vec![vec!['.'; plan[0].len()]; plan.len()];
    // we scan row by row starting with second one
    for row_idx in 1..plan.len() {
        let mut inside = false;
        let mut on_hor_line = false;
        let row = &plan[row_idx];
        for j in 1..row.len() {
            let window = [
                plan[row_idx - 1][j - 1],
                plan[row_idx - 1][j],
                plan[row_idx][j - 1],
                plan[row_idx][j],
            ];
            match window {
                ['#', '#', '.', _] => {
                    // we've crossed the line
                    inside = !inside;
                    target_plan[row_idx][j - 1] = window[1];
                }
                [_, '#', '#', _] => {
                    // either top or bottom horizontal line
                    on_hor_line = true;
                    target_plan[row_idx][j - 1] = window[1];
                }
                ['.', '#', '.', _] => {
                    if on_hor_line {
                        on_hor_line = false
                    } else {
                        inside = !inside;
                    }
                    target_plan[row_idx][j - 1] = window[1];
                    //     //
                    //     if on_hor_line {
                    //         // if we are finishing horizontal line - then we should switch 'inside' flag as well
                    //         on_hor_line = false;
                    //         inside = false;
                    //     }
                    //     if inside {
                    //         target_plan[row_idx][j - 1] = '#'
                    //     } else {
                    //         target_plan[row_idx][j - 1] = window[1]
                    //     }
                }
                _ => {
                    if inside {
                        target_plan[row_idx][j - 1] = '#'
                    } else {
                        target_plan[row_idx][j - 1] = window[1]
                    }
                }
            }
        }
    }
    eprintln!("Filled plan:");
    for (i, row) in target_plan.iter().enumerate() {
        print!("{:04}  ", i);
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    let result: usize = target_plan
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum();
    println!("Result: {}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct DigStep {
    dir: Direction,
    meters: usize,
}

impl Display for DigStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_s = match self.dir {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{} {}", dir_s, self.meters)
    }
}

fn parse_dir(s: &str) -> Direction {
    match s {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unexpected direction: {}", s),
    }
}

fn parse_instruction_1(line: &str) -> DigStep {
    DigStep {
        dir: parse_dir(&line[0..1]),
        meters: line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap(),
    }
}

fn parse_instruction_2(line: &str) -> DigStep {
    let hex_s = &line[line.find('#').unwrap() + 1..(line.len() - 1)];
    let dir_n: u8 = hex_s[5..6].parse().unwrap();
    let dir = match dir_n {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Up,
        _ => unimplemented!(),
    };
    DigStep {
        dir: dir,
        meters: usize::from_str_radix(&hex_s[0..5], 16).unwrap(),
    }
}

fn load_input(input_path: &Path, parse_i: fn(&str) -> DigStep) -> io::Result<Vec<DigStep>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| line.map(|l| parse_i(&l)))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_part_2() {
        assert_eq!(
            DigStep {
                dir: Direction::Right,
                meters: 461937
            },
            parse_instruction_2("R 6 (#70c710)")
        );
        assert_eq!(
            DigStep {
                dir: Direction::Down,
                meters: 56407
            },
            parse_instruction_2("D 5 (#0dc571)")
        );
    }
}
