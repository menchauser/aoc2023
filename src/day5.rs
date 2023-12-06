use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
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
            eprintln!(
                "Start seed range {} of {}: start={}, length={}",
                i,
                almanac.seeds.len() / 2,
                &seed_range.start,
                &seed_range.end - &seed_range.start
            );
            eprintln!("Total running time before: {:?}", start.elapsed());
            seed_range
                .map(|seed| map_to_location(seed, &almanac.rule_book))
                .min()
                .unwrap()
        })
        .min();

    println!("Result: {}", result.unwrap())
}

// Part 2 parallelized
pub fn part3(input_path: &Path) {
    let start = Instant::now();
    let almanac = load_input(input_path).unwrap();
    eprintln!("Loaded almanac:");
    eprintln!("{}", &almanac);
    let chunks: Vec<_> = almanac
        .seeds
        .chunks(2)
        .map(|c| c[0]..(c[0] + c[1]))
        .enumerate()
        .collect();
    let result = chunks
        .par_iter()
        .map(|(i, seed_range)| {
            eprintln!(
                "Start seed range {} of {}: start={}, length={}",
                i,
                almanac.seeds.len() / 2,
                &seed_range.start,
                &seed_range.end - &seed_range.start
            );
            eprintln!("Total running time before {}: {:?}", i, start.elapsed());
            let result = seed_range
                .clone()
                .map(|seed| map_to_location(seed, &almanac.rule_book))
                .min()
                .unwrap();
            eprintln!("Total running time after {}: {:?}", i, start.elapsed());
            result
        })
        .min();

    println!("Result: {}", result.unwrap())
}

fn apply_rules(seed_ranges: &Vec<Range<u64>>, rule_map: &Vec<RangeRule>) -> Vec<Range<u64>> {
    // for each range
    //  for each rule
    //    if range intersects with rule: extract intersected part and put it in the result
    // remember the part which was not intersected: it should be added to result as it is
    // how do we remember part of range which is not affected?
    // rules do not overlap so we can "nibble" from original range
    let mut result = Vec::new();
    for seed_range in seed_ranges {
        let mut sr = seed_range.clone();
        for rule in rule_map {
            if rule.contains(sr.start) || rule.contains(sr.end - 1) {
                eprintln!("range {:?} intersects with rule {}", sr, &rule);
                // now split range
                // there are three possible cases:
                // - sr is completely covered by rule
                // - rule bites left part of sr (sr.start is covered by rule)
                // - rule bites right part of sr (sr.end is covered by rule)
                if rule.contains(sr.start) {
                    if rule.contains(sr.end) {
                        // rule completely matches, early exit
                        todo!()
                    } else {
                        // left bite, continue
                        // we extract range part and immediately shift it according to rule
                        let extracted_sr = Range {
                            start: sr.start - rule.src_key + rule.dst_key,
                            end: rule.src_key + rule.range_len,
                        };
                        result.push(extracted_sr);
                        // remaining range
                        sr = Range {
                            start: rule.src_key + rule.range_len,
                            end: sr.end,
                        };
                    }
                } else {
                    // right bite, continue
                    // we extract range part and immediately shift it according to rule
                    let extracted_sr = Range {
                        start: rule.dst_key,
                        end: sr.end - rule.src_key + rule.dst_key,
                    };
                    result.push(extracted_sr);
                    // remaining range
                    sr = Range {
                        start: sr.start,
                        end: rule.src_key,
                    };
                }
            } else if sr.start <= rule.src_key && sr.end >= rule.src_key + rule.range_len {
                // special case: seed range contains rule within itself
                todo!()
            }
        }
        if !sr.is_empty() {
            result.push(sr);
        }
    }
    result = Vec::clone(seed_ranges);
    result.sort_by_key(|r| r.start);
    result
}

// Part 2: non-bruteforce solution
pub fn part4(input_path: &Path) {
    // the idea is that we go through rule maps keeping only ranges in memory (not seeds)
    // for each rule map
    //   new ranges = apply rules to ranges
    //   ranges = merge new ranges
    // in the end we take minimal value from minimal range
    let almanac = load_input(input_path).unwrap();
    println!("Loaded almanac: {}", &almanac);
    let seed_ranges: Vec<_> = almanac
        .seeds
        .chunks(2)
        .map(|c| c[0]..(c[0] + c[1]))
        .collect();
    println!("Seed ranges: {:?}", &seed_ranges);
    println!("First step: apply first rule map to seed ranges");
    let next = apply_rules(&seed_ranges, &almanac.rule_book[0]);
    println!("Ranges after apply:");
    println!("{:?}", next);
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

impl RangeRule {
    fn contains(&self, key: u64) -> bool {
        return self.src_key <= key && key < self.src_key + self.range_len;
    }
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

// we expect rule_map to be sorted, so we can implement binary search over it to be a bit faster
fn map_key_sorted(key: u64, rule_map: &Vec<RangeRule>) -> u64 {
    // we go through rules trying to find it key is in between [src..src+len]
    // if not, we just return as it is
    // we first binary search for rule which captures key
    let found = rule_map.binary_search_by(|rr| {
        if rr.contains(key) {
            Ordering::Equal
        } else {
            return rr.src_key.cmp(&key);
        }
    });
    match found {
        Ok(idx) => {
            let rr = &rule_map[idx];
            rr.dst_key + key - rr.src_key
        }
        Err(_) => key,
    }
}

#[allow(dead_code)]
fn map_key(key: u64, rule_map: &Vec<RangeRule>) -> u64 {
    rule_map
        .iter()
        .find(|rr| rr.contains(key))
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
        let result = map_key_sorted(key, rules);
        // eprintln!("Key mapped to next key: {}", result);
        result
    })
}

fn parse_rules(lines: &[String]) -> Vec<RangeRule> {
    // skip first line with header
    let mut result: Vec<_> = lines[1..]
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
        .collect();
    result.sort_by_key(|rr| rr.src_key);
    result
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
