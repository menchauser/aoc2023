use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let Input { workflows, parts } = load_input(input_path).unwrap();
    eprintln!("Workflows:");
    for (_, w) in &workflows {
        eprintln!("{:?}", w);
    }
    eprintln!("Parts:");
    for p in &parts {
        eprintln!("{:?}", p);
    }
    // let's run first part through workflows
    let mut accepted: Vec<&Part> = vec![];
    for p in &parts {
        eprintln!("Checking part: {:?}", p);
        let mut w = &workflows[&"in".to_string()];
        eprintln!("Starting workflow: {:?}", w);
        'w_loop: loop {
            'rule_loop: for r in &w.rules {
                if let Some(wname) = match_rule(&p, &r) {
                    eprintln!("Part matched for rule '{}'", wname);
                    if wname == "A" {
                        eprintln!("Accepted!");
                        accepted.push(&p);
                        break 'w_loop;
                    } else if wname == "R" {
                        eprintln!("Rejected!");
                        break 'w_loop;
                    } else {
                        w = &workflows[wname];
                        break 'rule_loop;
                    }
                }
            }
        }
    }
    let result: u32 = accepted
        .iter()
        .map(|p| p.ratings.values().sum::<u32>())
        .sum();
    println!("Result: {}", result);
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

// Returns Some(workflow) if matched or None if not
fn match_rule<'a>(part: &Part, rule: &'a WorkflowRule) -> Option<&'a String> {
    match rule {
        WorkflowRule::Condition(c, op, threshold, wname) => {
            let rating = part.ratings[c];
            match op {
                '<' if rating < *threshold => Some(wname),
                '>' if rating > *threshold => Some(wname),
                _ => None,
            }
        }
        WorkflowRule::Terminate(wname) => Some(wname),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Part {
    ratings: HashMap<char, u32>,
}

#[derive(Debug, PartialEq, Eq)]
enum WorkflowRule {
    // category, op, threshold, target workflow
    Condition(char, char, u32, String),
    // target workflow
    Terminate(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow {
    name: String,
    rules: Vec<WorkflowRule>,
}

struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

fn parse_workflow(line: &str) -> Workflow {
    // px{a<2006:qkq,m>2090:A,rfg}
    let start_idx = line.find('{').unwrap();
    let name = &line[..start_idx];
    let rules: Vec<_> = line[start_idx + 1..line.len() - 1]
        .split(',')
        .map(|rule_str| {
            if rule_str.contains(['<', '>']) {
                let mut rule_chars = rule_str.chars();
                let col_pos = rule_str.find(':').unwrap();
                WorkflowRule::Condition(
                    rule_chars.next().unwrap(),
                    rule_chars.next().unwrap(),
                    rule_str[2..col_pos].parse().unwrap(),
                    rule_str[col_pos + 1..].to_string(),
                )
            } else {
                WorkflowRule::Terminate(rule_str.to_string())
            }
        })
        .collect();
    Workflow {
        name: name.to_string(),
        rules: rules,
    }
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
        let workflow = parse_workflow(&line);
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

    #[test]
    fn test_parse_workfow() {
        let expected_workflow = Workflow {
            name: "px".to_string(),
            rules: vec![
                WorkflowRule::Condition('a', '<', 2006, "qkq".to_string()),
                WorkflowRule::Condition('m', '>', 2090, "A".to_string()),
                WorkflowRule::Terminate("rfg".to_string()),
            ],
        };
        assert_eq!(
            expected_workflow,
            parse_workflow("px{a<2006:qkq,m>2090:A,rfg}")
        );
    }
}
