use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let dig_plan = load_input(input_path).unwrap();
    // now the naive approach is to build a 'picture' and then fill it horizontally
    let mut plan: Vec<Vec<char>> = vec![vec!['.']];
    let mut row_idx: usize = 0;
    let mut col_idx: usize = 0;
    for step in dig_plan {
        match step.dir {
            Direction::Up => {
                // we should insert (step.meters - row) rows up
                // eprintln!(
                //     "Try going up {} meters: row={}, total rows={}",
                //     step.meters,
                //     row_idx,
                //     plan.len()
                // );
                let needed_rows = step.meters as i16 - row_idx as i16;
                let needed_cols = plan[row_idx].len();
                if needed_rows > 0 {
                    plan.reserve(needed_rows as usize);
                    for _ in 0..needed_rows {
                        plan.insert(0, vec!['.'; needed_cols]);
                    }
                    // for every inserted row our current position is shifted forward
                    row_idx += needed_rows as usize;
                }
                // insert '#' chars
                // eprintln!("row_idx: {}", row_idx);
                // eprintln!("needed_ros: {}", needed_rows as usize);
                // eprintln!("meters: {}", step.meters);
                for i in (row_idx - step.meters)..row_idx {
                    plan[i][col_idx] = '#';
                }
                // row should move from current row meters back
                row_idx = row_idx - step.meters;
            }
            Direction::Down => {
                // we should insert (step.meters - (len - row - 1)) rows down
                // eprintln!(
                //     "Try going down {} meters: row={}, total rows={}",
                //     step.meters,
                //     row_idx,
                //     plan.len()
                // );
                let needed_rows = step.meters as i16 - (plan.len() - row_idx - 1) as i16;
                let needed_cols = plan[row_idx].len();
                if needed_rows > 0 {
                    plan.reserve(needed_rows as usize);
                    for _ in 0..needed_rows {
                        plan.push(vec!['.'; needed_cols])
                    }
                }
                // insert '#' chars
                for i in row_idx..(row_idx + step.meters + 1) {
                    plan[i][col_idx] = '#';
                }
                row_idx = row_idx + step.meters;
            }
            Direction::Left => {
                // we should insert (step.meters - col) cols left
                // eprintln!(
                //     "Try going left {} meters: col={}, total cols={}",
                //     step.meters,
                //     col_idx,
                //     plan[row_idx].len()
                // );
                let needed_cols = step.meters as i16 - col_idx as i16;
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
                // eprintln!(
                //     "Try going right {} meters: col={}, total cols={}",
                //     step.meters,
                //     col_idx,
                //     plan[row_idx].len()
                // );
                // let needed_rows = plan.len();
                let needed_cols = step.meters as i16 - (plan[row_idx].len() - col_idx - 1) as i16;
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
    // now second part: fill field
    // to do that we insert one more col to the left and start filling
    // for row in plan.iter_mut() {
    //     // row.insert(0, '.');
    //     row.push('.');
    // }
    // plan.insert(0, vec!['.'; plan[0].len()]);
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

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct DigStep {
    dir: Direction,
    meters: usize,
    // TODO: rgb
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

fn load_input(input_path: &Path) -> io::Result<Vec<DigStep>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| {
            line.map(|l| DigStep {
                dir: parse_dir(&l[0..1]),
                meters: l
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap(),
            })
        })
        .collect()
}
