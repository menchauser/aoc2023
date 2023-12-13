use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    let result: u32 = input
        .iter()
        .map(|pat| {
            eprintln!("Scan pattern:\n{}", pat);
            find_horizontal_mirror(pat)
                .map(|row| {
                    eprintln!("Horizontal mirror at position {}", row + 1);
                    ((row + 1) * 100) as u32
                })
                .or_else(|| {
                    find_vertical_mirror(pat).map(|col| {
                        eprintln!("Vertical mirror at position {}", col + 1);
                        (col + 1) as u32
                    })
                })
                .unwrap()
        })
        .sum();
    println!("Result: {}", result);
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

fn find_vertical_mirror(pat: &Pattern) -> Option<usize> {
    // find first possible position where two cols are equal
    let width = pat.data[0].len();
    let mirror_col_hyp = (0..width - 1).filter(|col| cols_eq(pat, *col, *col + 1));
    for col in mirror_col_hyp {
        // early exist for first/last columns
        if col == 0 || col == width - 1 {
            return Some(col);
        }
        // otherwise, let's check sideways
        if zip((0..col).rev(), (col + 2)..width).all(|(col1, col2)| {
            eprintln!("Compare columns {} - {}", col1, col2);
            cols_eq(pat, col1, col2)
        }) {
            return Some(col);
        }
    }
    None
}

fn find_horizontal_mirror(pat: &Pattern) -> Option<usize> {
    let height = pat.data.len();
    let mirror_row_hyp = (0..height - 1).filter(|row| rows_eq(pat, *row, *row + 1));
    for row in mirror_row_hyp {
        // early exist for first/last rows
        if row == 0 || row == height - 1 {
            return Some(row);
        }
        // otherwise, let's check sideways
        if zip((0..row).rev(), (row + 2)..height).all(|(row1, row2)| {
            eprintln!("Compare rows {} - {}", row1, row2);
            rows_eq(pat, row1, row2)
        }) {
            return Some(row);
        }
    }
    None
}

fn cols_eq(pat: &Pattern, col1: usize, col2: usize) -> bool {
    let this_col_iter = pat.data.iter().map(|row| row[col1]);
    let next_col_iter = pat.data.iter().map(|row| row[col2]);
    this_col_iter.eq(next_col_iter)
}

fn rows_eq(pat: &Pattern, row1: usize, row2: usize) -> bool {
    let this_row_iter = pat.data[row1].iter();
    let next_row_iter = pat.data[row2].iter();
    this_row_iter.eq(next_row_iter)
}

struct Pattern {
    data: Vec<Vec<char>>,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        for j in 0..self.data[0].len() {
            write!(f, "{:X}", j)?;
        }
        writeln!(f)?;
        for (i, row) in self.data.iter().enumerate() {
            writeln!(f, "{:X} {}", i, row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

type Input = Vec<Pattern>;

fn load_input(input_path: &Path) -> io::Result<Input> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    let mut result: Input = Vec::new();
    let mut pattern: Vec<Vec<char>> = Vec::new();
    while let Some(line_res) = lines.next() {
        let line = line_res?;
        if line.is_empty() {
            result.push(Pattern { data: pattern });
            pattern = Vec::new();
        } else {
            pattern.push(line.chars().collect());
        }
    }
    if !pattern.is_empty() {
        result.push(Pattern { data: pattern })
    }
    Ok(result)
}
