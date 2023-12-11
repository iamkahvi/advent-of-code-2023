use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

// impl Ord for Card {
//     fn cmp(&self, other: &Self) -> Ordering {}
// }

impl Card {
    fn from_char(c: &char) -> Option<Card> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum HandType {
    Five(Card),
    Four(Card),
    FullHouse(Card, Card),
    Three(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    High(Card),
}

#[derive(Debug)]
struct Hand {
    cards: HandType,
    bid: i32,
}

fn main() {
    println!("Hello, world!");

    let lines = include_str!("../../test2.txt")
        .lines()
        .collect::<Vec<&str>>();

    dbg!(&lines);

    let hands: HashMap<String, Hand> = lines
        .iter()
        .map(|line| {
            // println!("{:?}", &line.split(" ").collect::<Vec<_>>());
            // dbg!(&line);
            if let [first, second, ..] = &*line.split_whitespace().collect::<Vec<_>>() {
                let cards = parse_handtype(first.to_string());
                let bid = second.to_string().parse::<i32>().unwrap();

                dbg!(&cards);

                return (first.to_string(), Hand { cards, bid });
            } else {
                panic!("aahhhh");
            }
        })
        .collect();

    println!("{:?}", hands);
}

fn parse_handtype(str: String) -> HandType {
    dbg!(&str);

    let counts = get_counts(&str);

    // println!("{:?}", counts);

    let counts_vec = counts
        .iter()
        .sorted_by(|a, b| b.1.cmp(a.1))
        .collect::<Vec<_>>();

    // println!("{:?}", counts_vec);

    match counts_vec.len() {
        1 => {
            let c = counts_vec[0].0;
            match Card::from_char(c) {
                Some(card) => HandType::Five(card),
                None => panic!("AAhh"),
            }
        }
        2 => {
            let c1 = Card::from_char(counts_vec[0].0);
            let n1 = counts_vec[0].1;
            let c2 = Card::from_char(counts_vec[1].0);
            let n2 = counts_vec[1].1;

            match ((c1, n1), (c2, n2)) {
                ((Some(_), 1), (Some(card2), 4)) => HandType::Four(card2),
                ((Some(card1), 2), (Some(card2), 3)) => HandType::FullHouse(card2, card1),
                ((Some(card1), 3), (Some(card2), 2)) => HandType::FullHouse(card1, card2),
                ((Some(card1), 4), (Some(_), 1)) => HandType::Four(card1),
                _ => panic!("AAhh"),
            }
        }
        3 => {
            let c1 = Card::from_char(counts_vec[0].0);
            let n1 = counts_vec[0].1;
            let c2 = Card::from_char(counts_vec[1].0);
            let n2 = counts_vec[1].1;
            let c3 = Card::from_char(counts_vec[2].0);
            let n3 = counts_vec[2].1;

            match ((c1, n1), (c2, n2), (c3, n3)) {
                ((Some(_), 1), (Some(card2), 2), (Some(card3), 2)) => {
                    HandType::TwoPair(card2, card3)
                }
                ((Some(_), 1), (Some(card2), 3), (Some(_), 1)) => HandType::Three(card2),
                ((Some(_), 1), (Some(_), 1), (Some(card3), 3)) => HandType::Three(card3),
                ((Some(card1), 2), (Some(card2), 2), (Some(_), 1)) => {
                    HandType::TwoPair(card1, card2)
                }
                ((Some(card1), 2), (Some(_), 1), (Some(card3), 2)) => {
                    HandType::TwoPair(card1, card3)
                }
                ((Some(card1), 3), (Some(_), 1), (Some(_), 1)) => HandType::Three(card1),
                _ => panic!("ahhhhhh"),
            }
        }
        4 => {
            for (card, count) in counts_vec.iter() {
                if let (Some(card), 2) = (Card::from_char(card), count) {
                    return HandType::OnePair(card);
                }
            }
            panic!("asdfadf")
        }
        5 => {
            // counts_vec.sort_by(compare)
            panic!("adfdf")
        }
        _ => HandType::Five(Card::A),
    }
}

fn get_counts(str: &String) -> HashMap<char, i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut current_char = None;
    let mut current_count = 0;

    for c in str.chars() {
        match current_char {
            Some(ch) if ch == c => current_count += 1,
            Some(ch) => {
                *counts.entry(ch).or_insert(0) += current_count;
                current_char = Some(c);
                current_count = 1;
            }
            None => {
                current_char = Some(c);
                current_count = 1;
            }
        }
    }

    if let Some(ch) = current_char {
        *counts.entry(ch).or_insert(0) += current_count;
    }

    return counts;
}
