use std::fs::File;
use std::io::{self, stderr, BufRead, Write};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    // println_panel(&mut stderr(), &input);
    let panel = slide_north(&input);
    // println!("After full tilt: ");
    // println_panel(&mut stderr(), &panel);
    let result: usize = panel
        .iter()
        .enumerate()
        .map(|(row_idx, row)| row.iter().filter(|c| **c == 'O').count() * (panel.len() - row_idx))
        .sum();
    println!("Result: {}", result);
}

fn println_panel(w: &mut dyn Write, input: &Vec<Vec<char>>) {
    for row in input {
        for c in row {
            write!(w, "{}", c).unwrap();
        }
        writeln!(w).unwrap();
    }
}

fn slide_north(panel: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // we go column over column
    let mut result = panel.clone();
    for j in 0..result[0].len() {
        slide_column_north(&mut result, j);
    }
    result
}

fn slide_column_north(panel: &mut Vec<Vec<char>>, col: usize) {
    // in column we scan from top to bottom
    // if current value is O => skip
    // if current value is # => skip
    // if current value is . => find next O and swap it here
    let mut cur_row = 0;
    while cur_row < panel.len() {
        // eprint!("row {}: ", cur_row);
        let cur_char = panel[cur_row][col];
        match cur_char {
            'O' | '#' => {
                // eprintln!("char {}, skip", cur_char);
                cur_row += 1
            }
            '.' => {
                // eprint!("char ., look for next O => ");
                // take all forward rows and find 'O' char
                let next_o = panel[cur_row + 1..]
                    .iter()
                    .enumerate()
                    .find(|(_, c)| c[col] == 'O' || c[col] == '#');
                // eprint!("{:?}", &next_o);
                if let Some((delta, c)) = next_o {
                    let swap_row = cur_row + delta + 1;
                    if c[col] == 'O' {
                        // eprintln!(" => swap rows {} - {}", cur_row, swap_row);
                        panel[cur_row][col] = 'O';
                        panel[swap_row][col] = '.';
                        cur_row += 1;
                    } else {
                        // if we encounter '#' we may continue from it
                        // eprintln!(" => jump to {}", swap_row + 1);
                        cur_row = swap_row + 1;
                    }
                } else {
                    cur_row += 1;
                }
                // eprintln!();
            }
            _ => unreachable!(),
        }
    }
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

fn load_input(input_path: &Path) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| line.map(|l| l.chars().collect()))
        .collect()
}
