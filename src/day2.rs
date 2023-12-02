use std::cmp::max;
use std::io;
use std::io::BufRead;
use std::{fs::File, path::Path};

pub fn part1(input_path: &Path) {
    let input = parse_input(input_path).unwrap();
    // now we search for games which has no more than 12r, 13g, 14b in each hand
    let result: u32 = input
        .iter()
        .filter(|g| g.sets.iter().all(|h| h.r <= 12 && h.g <= 13 && h.b <= 14))
        .map(|g| g.id)
        .sum();
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    let input = parse_input(input_path).unwrap();
    // first we calculate the minimal cube set for each game
    let result: u32 = input
        .iter()
        .map(|g| {
            g.sets
                .iter()
                .fold(CubeSet { r: 0, g: 0, b: 0 }, |acc, set| CubeSet {
                    r: max(acc.r, set.r),
                    g: max(acc.g, set.g),
                    b: max(acc.b, set.b),
                })
        })
        .map(|set| set.r * set.g * set.b)
        .sum();
    println!("Result: {}", result);
}

#[derive(Debug)]
struct CubeSet {
    r: u32,
    g: u32,
    b: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

fn parse_hand(hand_str: &str) -> CubeSet {
    let hand_str = hand_str.trim();
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    for elem in hand_str.split(',') {
        let elem = elem.trim();
        let num: u32 = elem[0..(elem.find(" ").unwrap())].parse().unwrap();
        if elem.ends_with("red") {
            r += num;
        } else if elem.ends_with("green") {
            g += num;
        } else if elem.ends_with("blue") {
            b += num;
        }
    }
    return CubeSet { r, g, b };
}

fn parse_game(line: String) -> Game {
    // read game ID: parse a number between "Game " and ":"
    let id_end_idx = line.find(":").unwrap();
    let id: u32 = line[5..id_end_idx].parse().unwrap();
    // now we can split remaining line by ';' and parse each hand separately
    let hands_str = &line[id_end_idx + 1..line.len()];
    let hands = hands_str.split(';').map(|s| parse_hand(s)).collect();
    Game { id, sets: hands }
}

fn parse_input(input_path: &Path) -> io::Result<Vec<Game>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    return buf_reader.lines().map(|res| res.map(parse_game)).collect();
}
