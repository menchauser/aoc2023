use std::io;
use std::io::BufRead;
use std::iter::zip;
use std::{fs::File, path::Path};

pub fn part1(input_path: &Path) {
    let stats = load_input(input_path).unwrap();
    eprintln!("Loaded race stats: {:?}", stats);
    let result: u32 = zip(&stats.times, &stats.distances)
        .map(|(race_time, beat_distance)| {
            winning_races(*race_time as u64, *beat_distance as u64) as u32
        })
        .product();
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    let (time, beat_distance) = load_input_part2(input_path).unwrap();
    eprintln!("Time: {}, Distance to beat: {}", time, beat_distance);
    let result = winning_races(time, beat_distance);
    println!("Result: {}", result);
}

#[derive(Debug)]
struct RaceStats {
    times: Vec<u32>,
    distances: Vec<u32>,
}

fn winning_races(time: u64, beat_distance: u64) -> usize {
    (1..time)
        .map(|hold_time| (time - hold_time) * hold_time)
        .filter(|&distance| distance > beat_distance)
        .count()
}

fn load_input(input_path: &Path) -> io::Result<RaceStats> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    let times = lines.next().unwrap().map(|l| {
        l["Time:".len()..]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    })?;
    let distances = lines.next().unwrap().map(|l| {
        l["Distance:".len()..]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    })?;
    Ok(RaceStats { times, distances })
}

fn load_input_part2(input_path: &Path) -> io::Result<(u64, u64)> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    let mut lines = buf_reader.lines();
    let time = lines.next().unwrap().map(|l| {
        l["Time:".len()..]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap()
    })?;
    let distance = lines.next().unwrap().map(|l| {
        l["Distance:".len()..]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap()
    })?;
    Ok((time, distance))
}
