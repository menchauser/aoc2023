use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::ControlFlow;
use std::path::Path;
use std::str::Chars;

pub fn part1(input_path: &Path) {
    let map = load_input(input_path).unwrap();
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

pub fn part2(input_path: &Path) {
    let map = load_input(input_path).unwrap();
    // now let's iterate over instructions and jump one by one
    let rep_instr = RepeatedString::new(&map.instructions);
    // now the state is now just single node but list of nodes
    let start_nodes: Vec<&String> = map.network.keys().filter(|n| n.ends_with("A")).collect();
    eprintln!("Starting nodes: {:?}", start_nodes);
    // another possibility: let's count how many steps from each starting point until Z
    // and then find least common multiple
    let steps_by_node: Vec<u64> = start_nodes
        .iter()
        .map(|start_node| {
            let curr_instr = rep_instr.clone();
            let result =
                curr_instr
                    .enumerate()
                    .try_fold(start_node.as_str(), |node, (step, direction)| {
                        if node.ends_with("Z") {
                            ControlFlow::Break((step, node))
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
                ControlFlow::Break((steps, node)) => {
                    eprintln!("{} reached {} in {} steps", &start_node, node, steps);
                    steps as u64
                }
                ControlFlow::Continue(_) => unreachable!(),
            }
        })
        .collect();
    let result = steps_by_node.iter().fold(1u64, |curr_lcm, val| {
        eprintln!("Calculate LCM of {} and {}", curr_lcm, val);
        lcm(curr_lcm, *val)
    });
    println!("Result: {}", result);
}

// Test searching for a loop
pub fn part3(input_path: &Path) {
    let map = load_input(input_path).unwrap();
    // let's go through a loop and check how often we will encounter Z
    let start_nodes: Vec<&String> = map.network.keys().filter(|n| n.ends_with("A")).collect();
    eprintln!("Start nodes: {:?}", start_nodes);
    for start_node in start_nodes {
        eprintln!("Check start node: {}", start_node);

        let rep_instr = RepeatedString::new(&map.instructions);
        rep_instr
            .take(50)
            .enumerate()
            .fold(start_node, |n: &String, (step, direction)| {
                if n.ends_with("Z") {
                    println!("Step: {}, node: {}", step, n);
                }
                let (next_l, next_r) = &map.network[n];
                match direction {
                    'L' => next_l,
                    'R' => next_r,
                    _ => unreachable!(),
                }
            });
    }
    // the idea would be for each starting position to find its "cycle" length and then find multiple
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

impl<'a> Clone for RepeatedString<'a> {
    fn clone(&self) -> Self {
        Self::new(self.string)
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
