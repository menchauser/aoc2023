use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let map = load_input(input_path).unwrap();
    eprintln!("Loaded map: {:?}", map);
    // now let's iterate over instructions and jump one by one
    let mut instrs = map.instructions.chars();
    let mut current_node = "AAA";
    let mut steps = 0;
    loop {
        if current_node == "ZZZ" {
            break;
        }
        if let Some(direction) = instrs.next() {
            let (next_l, next_r) = &map.network[current_node];
            match direction {
                'L' => current_node = next_l,
                'R' => current_node = next_r,
                _ => unreachable!(),
            }
            steps += 1;
        } else {
            // restart
            instrs = map.instructions.chars();
        }
    }
    println!("Result: {}", steps);
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
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
