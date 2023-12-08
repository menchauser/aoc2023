use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::ControlFlow;
use std::path::Path;
use std::str::Chars;

pub fn part1(input_path: &Path) {
    let map = load_input(input_path).unwrap();
    eprintln!("Loaded map: {:?}", map);
    // now let's iterate over instructions and jump one by one
    let rep_instr = RepeatedString::new(&map.instructions);
    let result = rep_instr
        .enumerate()
        .try_fold("AAA", |node, (step, direction)| {
            if node == "ZZZ" {
                ControlFlow::Break(step)
            } else {
                let (next_l, next_r) = &map.network[node];
                match direction {
                    'L' => ControlFlow::Continue(next_l.as_str()),
                    'R' => ControlFlow::Continue(next_r.as_str()),
                    _ => unreachable!(),
                }
            }
        });

    match result {
        ControlFlow::Break(steps) => println!("Result: {:}", steps),
        ControlFlow::Continue(_) => unreachable!(),
    }
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

// Iterator which emits characters from string, repeated from the beginning when string ends
struct RepeatedString<'a> {
    string: &'a str,
    current_iter: Chars<'a>,
}

impl RepeatedString<'_> {
    fn new<'a>(string: &'a str) -> RepeatedString<'a> {
        let current_iter = string.chars();
        RepeatedString {
            string: string,
            current_iter: current_iter,
        }
    }
}

impl<'a> Iterator for RepeatedString<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.current_iter.next() {
            Some(next)
        } else {
            // restart
            self.current_iter = self.string.chars();
            self.next()
        }
    }
}

#[derive(Debug)]
struct DesertMap {
    instructions: String,
    network: HashMap<String, (String, String)>,
}

fn load_input(input_path: &Path) -> io::Result<DesertMap> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    let instructions = lines.next().unwrap()?;
    let _ = lines.next().unwrap()?; // skip empty line
    let network_res: io::Result<HashMap<_, _>> = lines
        .map(|line| {
            line.map(|l|
        // network nodes are always 3-letter
        (l[0..3].to_string(), (l[7..10].to_string(), l[12..15].to_string())))
        })
        .collect();
    let network = network_res?;
    Ok(DesertMap {
        instructions,
        network,
    })
}

#[cfg(test)]
mod tests {
    use super::RepeatedString;

    #[test]
    fn repeated_string_repeats() {
        let str = "LLR";
        let rep_str = RepeatedString::new(str);
        let result: String = rep_str.take(8).collect();
        assert_eq!("LLRLLRLL", result)
    }
}
