use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    todo!()
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
struct Part {
    ratings: HashMap<char, u32>,
}

enum WorkfowRule {
    // category, op, threshold, target workflow
    Condition(char, char, u32, String),
    // target workflow
    Terminate(String),
}

struct Workflow {
    name: String,
    rules: Vec<WorkfowRule>,
}

struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

fn parse_workfow(line: &str) -> Workflow {
    todo!();
}

fn parse_part(line: &str) -> Part {
    let mut ratings: HashMap<char, u32> = HashMap::new();
    line[1..line.len() - 1]
        .split(',')
        .map(|rat_str| {
            (
                rat_str.chars().nth(0).unwrap(),
                rat_str[2..].parse::<u32>().unwrap(),
            )
        })
        .for_each(|(c, r)| {
            ratings.insert(c, r);
        });
    Part { ratings }
}

fn load_input(input_path: &Path) -> io::Result<Input> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<_> = vec![];
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let workflow = parse_workfow(&line);
        workflows.insert(workflow.name.clone(), workflow);
    }
    while let Some(line) = lines.next() {
        let line = line?;
        parts.push(parse_part(&line));
    }
    Ok(Input { workflows, parts })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_part() {
        let expected_ratings: HashMap<char, u32> =
            HashMap::from([('x', 787), ('m', 2655), ('a', 1222), ('s', 2876)]);
        assert_eq!(
            Part {
                ratings: expected_ratings
            },
            parse_part("{x=787,m=2655,a=1222,s=2876}")
        );
    }
}
