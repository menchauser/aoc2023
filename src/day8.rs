use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::ControlFlow;
use std::path::Path;
use std::str::Chars;
use std::time::Instant;

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
    let start = Instant::now();
    let map = load_input(input_path).unwrap();
    // now let's iterate over instructions and jump one by one
    let rep_instr = RepeatedString::new(&map.instructions);
    // now the state is now just single node but list of nodes
    let start_nodes: Vec<&String> = map.network.keys().filter(|n| n.ends_with("A")).collect();
    let path_count = start_nodes.len();
    eprintln!("Starting nodes: {:?}", start_nodes);
    let result = rep_instr
        .enumerate()
        .try_fold(start_nodes, |nodes, (step, direction)| {
            if step % 10_000_000 == 0 {
                eprintln!(
                    "{:16?}: steps: {}, current position: {:?}",
                    start.elapsed(),
                    step,
                    &nodes
                );
            }
            // try optimizing without hashset for small one
            let mut unique_count = 0;
            for (i, &n) in nodes.iter().enumerate() {
                if nodes.iter().take(i).all(|&x| x != n) {
                    unique_count += 1
                }
            }
            // let mut unique_nodes: HashSet<&String> = HashSet::new();
            // for n in &nodes {
            //     unique_nodes.insert(*n);
            // }
            // if unique_nodes.len() == path_count && unique_nodes.iter().all(|n| n.ends_with("Z")) {
            //     ControlFlow::Break((step, nodes))
            if unique_count == path_count && nodes.iter().all(|n| n.ends_with("Z")) {
                ControlFlow::Break((step, nodes))
            } else {
                // else let's make a step for each node
                let next_nodes = nodes
                    .iter()
                    .map(|n| {
                        let next_step = &map.network[*n];
                        match direction {
                            'L' => &next_step.0,
                            'R' => &next_step.1,
                            _ => unreachable!(),
                        }
                    })
                    .collect::<Vec<&String>>();

                ControlFlow::Continue(next_nodes)
            }
        });

    match result {
        ControlFlow::Break((steps, nodes)) => {
            println!("Result: {} (final nodes: {:?})", steps, nodes)
        }
        ControlFlow::Continue(_) => unreachable!(),
    }
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
