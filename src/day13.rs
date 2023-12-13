use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    // println!("Loaded patterns:");
    // for pattern in &input {
    //     println!("{}", pattern);
    // }

    for pattern in &input {
        eprintln!("Check pattern:\n{}", pattern);
        let result = find_vertical_mirror(pattern)
            .map(|col| {
                eprintln!("Vertical mirror at position {}", col + 1);
                col + 1
            })
            .or_else(|| {
                find_horizontal_mirror(pattern).map(|row| {
                    eprintln!("Horizontal mirror at position {}", row + 1);
                    (row + 1) * 100
                })
            })
            .unwrap();
        println!("Result: {}", result)
    }
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

fn find_vertical_mirror(pat: &Pattern) -> Option<usize> {
    // we need to find first position where two cols are equal
    // and then go sideways from that position
    for j in 0..pat.data[0].len() - 1 {
        // compare this column to next
        let this_col_iter = pat.data.iter().map(|row| row[j]);
        let next_col_iter = pat.data.iter().map(|row| row[j + 1]);

        if this_col_iter.eq(next_col_iter) {
            return Some(j);
        }
    }
    None
}

fn find_horizontal_mirror(pat: &Pattern) -> Option<usize> {
    // compare this row to next
    for i in 0..pat.data.len() - 1 {
        eprintln!("Compare rows:");
        eprintln!("{:?}", pat.data[i]);
        eprintln!("{:?}", pat.data[i + 1]);
        let this_row_iter = pat.data[i].iter();
        let next_row_iter = pat.data[i + 1].iter();
        if this_row_iter.eq(next_row_iter) {
            return Some(i);
        }
    }
    None
}

struct Pattern {
    data: Vec<Vec<char>>,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            writeln!(f, "{}", row.iter().collect::<String>())?;
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
