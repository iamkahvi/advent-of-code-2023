use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Debug, PartialOrd, Ord)]
enum HandType {
    Five(Card),
    Four(Card),
    FullHouse(Card, Card),
    Three(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    High(Card),
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HandType::High(_), HandType::High(_)) => true,
            (HandType::OnePair(_), HandType::OnePair(_)) => true,
            (HandType::TwoPair(_, _), HandType::TwoPair(_, _)) => true,
            (HandType::Three(_), HandType::Three(_)) => true,
            (HandType::FullHouse(_, _), HandType::FullHouse(_, _)) => true,
            (HandType::Four(_), HandType::Four(_)) => true,
            (HandType::Five(_), HandType::Five(_)) => true,
            _ => false,
        }
    }
}

impl Eq for HandType {}

#[derive(Debug)]
struct Hand {
    ht: HandType,
    bid: i32,
}

fn main() {
    println!("Hello, world!");

    let lines = include_str!("../../input1.txt")
        .lines()
        .collect::<Vec<&str>>();

    dbg!(&lines);

    let hands: Vec<(Vec<Card>, Hand)> = lines
        .iter()
        .map(|line| {
            // println!("{:?}", &line.split(" ").collect::<Vec<_>>());
            // dbg!(&line);
            if let [first, second, ..] = &*line.split_whitespace().collect::<Vec<_>>() {
                let ht = parse_handtype(first.to_string());
                let bid = second.to_string().parse::<i32>().unwrap();

                let cards: Vec<Card> = first
                    .chars()
                    .into_iter()
                    .map(|c| match Card::from_char(&c) {
                        Some(card) => card,
                        _ => panic!("asfadf"),
                    })
                    .collect();

                return (cards, Hand { ht, bid });
            } else {
                panic!("aahhhh");
            }
        })
        .collect();

    // If two hands have the same type, a second ordering rule takes effect.
    // Start by comparing the first card in each hand.
    // If these cards are different, the hand with the stronger first card is
    // considered stronger. If the first card in each hand have the same label,
    // however, then move on to considering the second card in each hand.
    // If they differ, the hand with the higher second card wins;
    // otherwise, continue with the third card in each hand,
    // then the fourth, then the fifth.

    let sorted_hands: Vec<_> = hands
        .into_iter()
        .sorted_by(|a, b| match (&a.1.ht, &b.1.ht) {
            (a_ht, b_ht) if a_ht == b_ht => {
                for (ai, bi) in a.0.clone().into_iter().zip(b.0.clone().into_iter()) {
                    if ai == bi {
                        continue;
                    } else {
                        dbg!((&ai, &bi));
                        dbg!(&ai.cmp(&bi));
                        return ai.cmp(&bi);
                    }
                }
                panic!("ahh")
            }
            (a, b) => {
                dbg!((&a, &b));
                dbg!(&(a.cmp(&b)));
                b.cmp(&a)
            }
        })
        .collect();

    dbg!(&sorted_hands);

    let res: i32 = sorted_hands
        .into_iter()
        .enumerate()
        .map(|(i, el)| {
            println!("hand: {:?}, bid: {}, rank: {}", el.0, el.1.bid, i + 1);
            el.1.bid * (i + 1) as i32
        })
        .sum();

    println!("res: {:?}", res);
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
            let ranked_cards: Vec<_> = counts_vec
                .clone()
                .iter()
                .map(|(ch, _)| match Card::from_char(ch) {
                    Some(card) => card,
                    _ => panic!("Adfsf"),
                })
                .sorted_by(|a, b| b.cmp(a))
                .collect();

            return HandType::High(ranked_cards[0]);
        }
        _ => panic!("more than 5 tokens"),
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
