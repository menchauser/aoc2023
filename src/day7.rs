use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    println!("Read hands, bids and types");
    for hand in &input {
        println!(
            "{} - {:?}    {}",
            hand.hand,
            hand_type(&hand.hand),
            hand.bid
        );
    }
    let mut hands = input;
    hands.sort();
    println!("Sorted hands (ASC):");
    for h in &hands {
        println!("{} - {:?}", h.hand, hand_type(&h.hand));
    }
    let mut result = 0u32;
    for (idx, hand) in hands.iter().rev().enumerate() {
        let rank = hands.len() - idx;
        result += hand.bid * rank as u32;
    }
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    todo!()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    hand: String,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // we compare hand types and if they are equal: "lexicographically" compare strings
        let hand_cmp = hand_type(&self.hand).cmp(&hand_type(&other.hand));
        if hand_cmp != Ordering::Equal {
            return hand_cmp;
        } else {
            // else compare char by char
            // A > K > Q  > J > T > 9 ...
            // remap for lexigographical comparison
            let replacements = vec![("A", "Z"), ("K", "Y"), ("Q", "X"), ("J", "W"), ("T", "V")];
            let mut self_hand = self.hand.clone();
            let mut other_hand = other.hand.clone();
            for (from_c, to_c) in replacements {
                self_hand = self_hand.replace(from_c, to_c);
                other_hand = other_hand.replace(from_c, to_c)
            }
            self_hand.cmp(&other_hand)
        }
    }
}

fn hand_type(hand: &str) -> HandType {
    let mut card_counts: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        card_counts.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }
    // if all cards are equal: five of a kind
    return match card_counts.len() {
        1 => HandType::FiveOfKind,
        2 => {
            let random_count = card_counts.values().next().unwrap();
            if *random_count == 1 || *random_count == 4 {
                return HandType::FourOfKind;
            } else {
                return HandType::FullHouse;
            }
        }
        3 => {
            if card_counts.values().any(|c| *c == 3) {
                HandType::ThreeOfKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        count => panic!(
            "Unexpected hand passed with {} different cards: {}",
            count, hand
        ),
    };
}

fn load_input(input_path: &Path) -> io::Result<Vec<Hand>> {
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| {
            line.map(|l| {
                let mut split = l.split_whitespace();
                let hand = split.next().unwrap().to_string();
                let bid = split.next().unwrap().parse::<u32>().unwrap();
                Hand {
                    hand: hand,
                    bid: bid,
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day7::HandType;

    use super::hand_type;

    #[test]
    fn hand_type_correct() {
        assert_eq!(HandType::FiveOfKind, hand_type("AAAAA"));
        assert_eq!(HandType::FourOfKind, hand_type("AA8AA"));
        assert_eq!(HandType::FullHouse, hand_type("23332"));
        assert_eq!(HandType::ThreeOfKind, hand_type("TTT98"));
        assert_eq!(HandType::TwoPair, hand_type("23432"));
        assert_eq!(HandType::OnePair, hand_type("A23A4"));
        assert_eq!(HandType::HighCard, hand_type("23456"));
    }
}
