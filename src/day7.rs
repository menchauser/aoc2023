use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(input_path: &Path) {
    let card_map: HashMap<char, u8> =
        HashMap::from([('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]);
    let input = load_input(input_path, |c| map_card(&card_map, c)).unwrap();
    let result = calc_answer(input);
    println!("Result: {}", result);
}

pub fn part2(input_path: &Path) {
    let card_map: HashMap<char, u8> =
        HashMap::from([('T', 10), ('J', 0), ('Q', 12), ('K', 13), ('A', 14)]);
    let input = load_input(input_path, |c| map_card(&card_map, c)).unwrap();
    let result = calc_answer(input);
    println!("Result: {}", result);
}

fn calc_answer(input: Vec<Player>) -> u32 {
    eprintln!("Debug -- Display:");
    for hand in &input {
        eprintln!("{:?} -- {}", hand, hand);
    }
    let mut players: Vec<Player> = input;
    players.sort_by(|a, b| cmp_players(hand_type, a, b));
    eprintln!("Sorted hands (ASC):");
    for h in &players {
        eprintln!("{} - {:?}", h, hand_type(h.hand));
    }
    players
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx + 1) as u32)
        .sum()
}

fn cmp_players(hand_type: fn([u8; 5]) -> HandType, lhs: &Player, rhs: &Player) -> Ordering {
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
    hand: [u8; 5],
    bid: u32,
}

fn to_string(hand: &[u8]) -> String {
    let card_rev_map: HashMap<u8, char> = HashMap::from([
        (10, 'T'),
        (11, 'J'),
        (12, 'Q'),
        (13, 'K'),
        (14, 'A'),
        (0, 'J'),
    ]);
    let mut result = String::new();
    for &c in hand {
        if (2..10).contains(&c) {
            result.push_str(format!("{}", c).as_str());
        } else {
            result.push(card_rev_map[&c]);
        }
    }
    result
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  {}", to_string(&self.hand), self.bid)
    }
}

fn hand_type(hand: [u8; 5]) -> HandType {
    let mut card_counts: HashMap<u8, usize> = HashMap::new();
    for c in hand {
        card_counts.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }

    eprintln!("Calc type of hand {}", to_string(&hand));
    // joker always merges with bigest non-joker card, except the situation where all are jokers
    let &joker_count = card_counts.get(&0).unwrap_or(&0);
    // let's get biggest non-joker card and increase its count by joker count
    if joker_count == 5 {
        // early exit
        return HandType::FiveOfKind;
    }
    if joker_count > 0 && joker_count < 5 {
        let (most_freq_card, _) = card_counts
            .iter()
            .filter(|(&k, _)| k != 0)
            .max_by_key(|(_, &count)| count)
            .unwrap();
        eprint!("{:?}: ", &hand);
        eprintln!(
            "Hand {} most freq card: {}",
            to_string(&hand),
            most_freq_card
        );
        card_counts
            .entry(*most_freq_card)
            .and_modify(|c| *c += joker_count);
    }
    // remove Joker card
    card_counts.remove(&0);
    eprintln!("After Joker card counts: {:?}", &card_counts);

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

fn map_card(card_map: &HashMap<char, u8>, c: char) -> u8 {
    if c.is_digit(10) {
        c.to_digit(10).unwrap() as u8
    } else {
        // eprintln!("map key {} from map {:?}", c, card_map);
        card_map[&c]
    }
}

fn load_input<FM>(input_path: &Path, map_card: FM) -> io::Result<Vec<Player>>
where
    FM: Fn(char) -> u8,
{
    let file = File::open(input_path)?;
    let buf_reader = io::BufReader::new(file);
    buf_reader
        .lines()
        .map(|line| {
            line.map(|l| {
                let mut split = l.split_whitespace();
                let hand = split.next().unwrap().to_string();
                let bid = split.next().unwrap().parse::<u32>().unwrap();
                let hand_vec: Vec<u8> = hand.chars().map(|c| map_card(c)).collect();
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
    use std::collections::HashMap;

    use crate::day7::{hand_type, map_card, HandType};

    fn map_hand<FC>(map_card: FC, s: String) -> [u8; 5]
    where
        FC: Fn(char) -> u8,
    {
        let hand_vec: Vec<u8> = s.chars().map(|c| map_card(c)).collect();
        hand_vec.try_into().unwrap()
    }

    #[test]
    fn hand_type_part_1_correct() {
        let card_map: HashMap<char, u8> =
            HashMap::from([('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]);
        let h = |s: &str| map_hand(|c| map_card(&card_map, c), s.to_string());
        assert_eq!(HandType::FiveOfKind, hand_type(h("AAAAA")));
        assert_eq!(HandType::FourOfKind, hand_type(h("AA8AA")));
        assert_eq!(HandType::FullHouse, hand_type(h("23332")));
        assert_eq!(HandType::ThreeOfKind, hand_type(h("TTT98")));
        assert_eq!(HandType::TwoPair, hand_type(h("23432")));
        assert_eq!(HandType::OnePair, hand_type(h("A23A4")));
        assert_eq!(HandType::HighCard, hand_type(h("23456")));
    }

    #[test]
    fn hand_type_part_2_correct() {
        let card_map: HashMap<char, u8> =
            HashMap::from([('T', 10), ('J', 0), ('Q', 12), ('K', 13), ('A', 14)]);
        let h = |s: &str| map_hand(|c| map_card(&card_map, c), s.to_string());
        assert_eq!(HandType::OnePair, hand_type(h("32T3K")));
        assert_eq!(HandType::TwoPair, hand_type(h("KK677")));
        assert_eq!(HandType::FourOfKind, hand_type(h("T55J5")));
        assert_eq!(HandType::FourOfKind, hand_type(h("KTJJT")));
        assert_eq!(HandType::FourOfKind, hand_type(h("QQQJA")));
        assert_eq!(HandType::FiveOfKind, hand_type(h("JJ2JJ")));
        assert_eq!(HandType::FiveOfKind, hand_type(h("JJJJJ")));
    }
}
