use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::repeat;
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    eprintln!("Loaded input:");
    for c in &input {
        eprintln!("{}", c);
    }
    let result: u32 = input.iter().take(2).map(count_arrangements).sum();
    println!("Result: {}", result)
}

#[allow(unused)]
pub fn part2(input_path: &Path) {
    todo!()
}

fn count_arrangements(cond: &Condition) -> u32 {
    eprintln!("Arrangements for:\n'{}'", cond);
    // let mut test_arrangement = Vec::new();
    let group_strs: Vec<String> = cond
        .damaged_groups
        .iter()
        .map(|size| repeat('#').take(*size as usize).collect())
        .collect();

    let mut first = true;
    let cap = cond.record.len();
    let mut result_string = String::with_capacity(cap);
    // at every point in time we have a constant 'prefix' and variable 'suffix'
    // we should iterate moving suffix from its starting point to possible end point (cap - suffix.len)
    // let's play with taking first N-1 groups as prefix and last group as suffix

    for s in &group_strs[..group_strs.len() - 1] {
        if first {
            first = false;
        } else {
            result_string.push('.');
        }
        result_string.push_str(s);
    }
    for _ in 0..(result_string.capacity() - result_string.len()) {
        result_string.push('.');
    }
    println!("'{}'", &result_string);

    1
}

// Generate all positions of group sized g within capacity cap
fn all_positions(g: usize, cap: usize) -> Vec<String> {
    assert!(g <= cap);
    let g_str = repeat('#').take(g).collect::<String>();
    let mut result = Vec::with_capacity(cap as usize);
    for i in 0..(cap - g + 1) {
        let mut new_pos = String::with_capacity(cap);
        for _ in 0..i {
            new_pos.push('.');
        }
        new_pos.push_str(&g_str);
        for _ in (i + g)..cap {
            new_pos.push('.');
        }
        result.push(new_pos);
    }
    result
}

fn format_groups(groups: &[u32]) -> String {
    let mut group_str_iter = groups
        .iter()
        .map(|size| repeat('#').take(*size as usize).collect::<String>());
    let mut first = true;
    let mut result = String::with_capacity(groups.len() * 2 - 1); // just a guess
    while let Some(group) = group_str_iter.next() {
        if first {
            first = false;
        } else {
            result.push('.');
        }
        result.push_str(&group);
    }
    result
}

struct Condition {
    damaged_groups: Vec<u32>,
    record: String,
}

impl Condition {
    fn from(cond_str: &str) -> Self {
        let mut splits = cond_str.split_whitespace();
        let record = splits.next().unwrap();
        let groups: Vec<_> = splits
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        Self {
            damaged_groups: groups,
            record: record.to_string(),
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group_str = self
            .damaged_groups
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{} {}", &self.record, group_str)
    }
}

fn load_input(input_path: &Path) -> io::Result<Vec<Condition>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let lines = buf_reader.lines();
    lines
        .map(|line| line.map(|l| Condition::from(&l)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_groups() {
        assert_eq!("#", format_groups(&[1]));
        assert_eq!("###", format_groups(&[3]));
        assert_eq!("#.#.###", format_groups(&[1, 1, 3]));
    }

    #[test]
    fn test_all_positions() {
        assert_eq!(vec!["###..", ".###.", "..###"], all_positions(3, 5));
        assert_eq!(vec!["#...", ".#..", "..#.", "...#"], all_positions(1, 4))
    }

    #[test]
    fn test_simple_arrangements() {
        let cond = Condition::from("???.### 1,1,3");
        eprintln!("Testing condition {}", cond);
        assert_eq!(1, count_arrangements(&cond));
    }
}
