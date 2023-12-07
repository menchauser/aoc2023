use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let input = load_input(input_path).unwrap();
    eprintln!("Loaded new input:");
    eprintln!("Debug -- Display:");
    for hand in &input {
        eprintln!("{:?} -- {}", hand, hand);
    }
    let mut hands = input;
    hands.sort_by(cmp_players);
    eprintln!("Sorted hands (ASC):");
    for h in &hands {
        eprintln!("{} - {:?}", h, hand_type(h.hand));
    }
    let result: u32 = hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1) as u32)
        .sum();
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    todo!()
}

fn cmp_players(lhs: &Player, rhs: &Player) -> Ordering {
    // we compare hand types and if they are equal: "lexicographically" compare strings
    let hand_cmp = hand_type(lhs.hand).cmp(&hand_type(rhs.hand));
    if hand_cmp != Ordering::Equal {
        return hand_cmp;
    } else {
        lhs.hand.cmp(&rhs.hand)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Player {
    hand: [u8; 5], // the size of hand is fixed
    bid: u32,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card_rev_map: HashMap<u8, char> =
            HashMap::from([(10, 'T'), (11, 'J'), (12, 'Q'), (13, 'K'), (14, 'A')]);
        for c in self.hand {
            if c < 10 {
                write!(f, "{}", c)?;
            } else {
                write!(f, "{}", card_rev_map[&c])?;
            }
        }
        write!(f, "  {}", self.bid)?;
        Ok(())
    }
}

fn hand_type(hand: [u8; 5]) -> HandType {
    let mut card_counts: HashMap<u8, usize> = HashMap::new();
    for c in hand {
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
            "Unexpected hand passed with {} different cards: {:?}",
            count, hand
        ),
    };
}

fn load_input(input_path: &Path) -> io::Result<Vec<Player>> {
    let card_map: HashMap<char, u8> =
        HashMap::from([('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]);
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| {
            line.map(|l| {
                let mut split = l.split_whitespace();
                let hand = split.next().unwrap().to_string();
                let bid = split.next().unwrap().parse::<u32>().unwrap();
                let hand_vec: Vec<u8> = hand
                    .chars()
                    .map(|c| {
                        if c.is_digit(10) {
                            c.to_digit(10).unwrap() as u8
                        } else {
                            card_map[&c]
                        }
                    })
                    .collect();
                Player {
                    hand: hand_vec.try_into().unwrap(),
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
