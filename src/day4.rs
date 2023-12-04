use std::cmp::max;
use std::collections::HashSet;
use std::fmt::{write, Display};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let cards = load_input(input_path).unwrap();
    println!("Loaded cards:");
    for card in &cards {
        eprintln!("{:?}", card);
    }
    // for each card we calculate a number
    let result: u32 = (&cards)
        .iter()
        .map(|card| {
            let win_c: HashSet<u8> = HashSet::from_iter(card.winning_nums.iter().cloned());
            let pres_c: HashSet<u8> = HashSet::from_iter(card.present_nums.iter().cloned());
            win_c.intersection(&pres_c).count()
        })
        .map(|n| if n == 0 { 0 } else { 2u32.pow(n as u32 - 1) })
        .sum();
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    todo!()
}

#[derive(Debug)]
struct Card {
    winning_nums: Vec<u8>,
    present_nums: Vec<u8>,
}

fn parse_card(s: String) -> Card {
    eprintln!("String: '{}'", s);
    let colon_idx = s.find(":").unwrap();
    let bar_idx = s.find("|").unwrap();
    let winning_nums: Vec<_> = (&s[colon_idx + 2..bar_idx - 1])
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|ns| ns.parse::<u8>().unwrap())
        .collect();
    let present_nums: Vec<_> = (&s[bar_idx + 2..s.len()])
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|ns| ns.parse::<u8>().unwrap())
        .collect();
    Card {
        winning_nums: winning_nums,
        present_nums: present_nums,
    }
}

fn load_input(input_path: &Path) -> io::Result<Vec<Card>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    // return buf_reader.lines().map(|res| res.map(parse_game)).collect();
    return buf_reader.lines().map(|r| r.map(parse_card)).collect();
}
