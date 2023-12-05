use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

pub fn part1(input_path: &Path) {
    let almanac = load_input(input_path).unwrap();
    eprintln!("Loaded almanac:");
    eprintln!("{}", &almanac);
    let result = almanac
        .seeds
        .iter()
        .map(|seed| map_to_location(*seed, &almanac.rule_book))
        .min();
    println!("Result: {}", result.unwrap())
}

pub fn part2(input_path: &Path) {
    let start = Instant::now();
    let almanac = load_input(input_path).unwrap();
    eprintln!("Loaded almanac:");
    eprintln!("{}", &almanac);
    let result = almanac
        .seeds
        .chunks(2)
        .map(|c| c[0]..(c[0] + c[1]))
        .enumerate()
        .map(|(i, seed_range)| {
            eprintln!("Start seed range {} of {}: start={}, length={}", 
                      i, almanac.seeds.len() / 2, &seed_range.start, &seed_range.end - &seed_range.start);
            eprintln!("Total running time before: {:?}", start.elapsed());
            seed_range
                .map(|seed| map_to_location(seed, &almanac.rule_book))
                .min()
                .unwrap()
        })
        .min();

    println!("Result: {}", result.unwrap())
}

struct RangeRule {
    src_key: u64,
    dst_key: u64,
    range_len: u64,
}

struct Almanac {
    seeds: Vec<u64>,
    rule_book: Vec<Vec<RangeRule>>,
}

impl Display for RangeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.dst_key, self.src_key, self.range_len)
    }
}

impl Display for Almanac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "seeds: {:?}\n", self.seeds)?;
        for (idx, rules_map) in self.rule_book.iter().enumerate() {
            writeln!(f, "map {}:", idx)?;
            for rr in rules_map {
                writeln!(f, "{}", rr)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn map_key(key: u64, rule_map: &Vec<RangeRule>) -> u64 {
    // we go through rules trying to find it key is in between [src..src+len]
    // if not, we just return as it is
    rule_map
        .iter()
        .find(|r| (r.src_key..(r.src_key + r.range_len)).contains(&key))
        .map(|rr| {
            // eprintln!("key: {}, map rule: {}", key, rr);
            // eprint!("{} + {} - {} = ", rr.dst_key, key, rr.src_key);
            rr.dst_key + key - rr.src_key
        })
        .unwrap_or(key)
}

fn map_to_location(seed: u64, rule_book: &Vec<Vec<RangeRule>>) -> u64 {
    rule_book.iter().fold(seed, |key, rules| {
        // eprintln!("Check key {}", key);
        let result = map_key(key, rules);
        // eprintln!("Key mapped to next key: {}", result);
        result
    })
}

fn parse_rules(lines: &[String]) -> Vec<RangeRule> {
    // skip first line with header
    lines[1..]
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.parse::<u64>().unwrap()))
        .map(|mut nums| {
            let dst = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();
            RangeRule {
                src_key: src,
                dst_key: dst,
                range_len: len,
            }
        })
        .collect()
}

fn load_input(input_path: &Path) -> io::Result<Almanac> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    let seeds_str = lines.next().unwrap()?;
    let seeds: Vec<u64> = seeds_str[7..]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    // skip empty line
    let _ = lines.next().unwrap();
    // map remaining lines to rulebook
    let maps_lines: Vec<String> = lines.collect::<io::Result<Vec<String>>>().unwrap();
    let maps = maps_lines
        .split(|line| line.is_empty())
        .map(|lines| parse_rules(lines))
        .collect();
    return Ok(Almanac {
        seeds,
        rule_book: maps,
    });
}
