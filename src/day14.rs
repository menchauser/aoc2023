use std::fs::File;
use std::io::{self, BufRead, Write};
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

#[allow(unused)]
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

#[allow(unused)]
fn slide_column_north2(panel: &mut Vec<Vec<char>>, col: usize) {
    // another approach:
    //   find all possible 'stopping' locations (top, first O in column, #)
    //   count the number of O between stopping locations
    //   write down number of Os after each stopping location
    // top is always a 'stopping' position
    let mut stop_cols = vec![0];
    // if column starts with '.' - its a stopping position, otherwise we look for first 'O'
    if panel[0][col] == '.' {
        stop_cols.push(0);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_north() {
        let input = load_input(Path::new("day141.test")).unwrap();
        let result = slide_north(&input);
        let expected = vec![
            vec!['O', 'O', 'O', 'O', '.', '#', '.', 'O', '.', '.'],
            vec!['O', 'O', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['O', 'O', '.', '.', 'O', '#', '#', '.', '.', 'O'],
            vec!['O', '.', '.', '#', '.', 'O', 'O', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '#'],
            vec!['.', '.', 'O', '.', '.', '#', '.', 'O', '.', 'O'],
            vec!['.', '.', 'O', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.'],
        ];
        assert_eq!(expected, result);
    }
}
