use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 1,
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

    let hands: Vec<(Vec<Card>, Hand)> = lines
        .iter()
        .map(|line| {
            let mut words = line.split_whitespace();
            let card_str = words.next().unwrap();
            let bid_str = words.next().unwrap();

            let cards: Vec<Card> = card_str
                .chars()
                .into_iter()
                .map(|c| match Card::from_char(&c) {
                    Some(card) => card,
                    _ => panic!("asfadf"),
                })
                .collect();

            dbg!(&cards);

            let ht = parse_handtype(&cards);
            let bid = bid_str.to_string().parse::<i32>().unwrap();

            (cards, Hand { ht, bid })
        })
        .collect();

    let sorted_hands: Vec<_> = hands
        .into_iter()
        .sorted_by(|a, b| match (&a.1.ht, &b.1.ht) {
            (a_ht, b_ht) if a_ht == b_ht => {
                for (ai, bi) in a.0.iter().zip(&b.0) {
                    if ai == bi {
                        continue;
                    } else {
                        return ai.cmp(&bi);
                    }
                }
                panic!("ahh")
            }
            (a, b) => b.cmp(&a),
        })
        .collect();

    // dbg!(&sorted_hands);

    let res: i32 = sorted_hands
        .into_iter()
        .enumerate()
        .map(|(i, el)| {
            println!(
                "hand: {:?}, bid: {}, rank: {}, \nht: {:?}",
                el.0,
                el.1.bid,
                i + 1,
                el.1.ht
            );
            el.1.bid * (i + 1) as i32
        })
        .sum();

    println!("res: {:?}", res);
}

fn parse_handtype(cards: &Vec<Card>) -> HandType {
    // there should be no Js in this counts vec
    let counts = get_counts(&cards);

    // match on number of unique numbers in the hand
    match counts.len() {
        1 => {
            let c = counts[0].0;
            return HandType::Five(c);
        }
        2 => {
            let (c1, n1) = counts[0];
            let (c2, n2) = counts[1];

            match ((c1, n1), (c2, n2)) {
                ((_, 1), (card2, 4)) => HandType::Four(card2),
                ((card1, 2), (card2, 3)) => HandType::FullHouse(card2, card1),
                ((card1, 3), (card2, 2)) => HandType::FullHouse(card1, card2),
                ((card1, 4), (_, 1)) => HandType::Four(card1),
                _ => panic!("AAhh"),
            }
        }
        3 => {
            let (c1, n1) = counts[0];
            let (c2, n2) = counts[1];
            let (c3, n3) = counts[2];

            // 2,2,1
            // 3,1,1

            match ((c1, n1), (c2, n2), (c3, n3)) {
                ((_, 1), (card2, 2), (card3, 2)) => HandType::TwoPair(card2, card3),
                ((_, 1), (card2, 3), (_, 1)) => HandType::Three(card2),
                ((_, 1), (_, 1), (card3, 3)) => HandType::Three(card3),
                ((card1, 2), (card2, 2), (_, 1)) => HandType::TwoPair(card1, card2),
                ((card1, 2), (_, 1), (card3, 2)) => HandType::TwoPair(card1, card3),
                ((card1, 3), (_, 1), (_, 1)) => HandType::Three(card1),
                _ => panic!("ahhhhhh"),
            }
        }
        4 => {
            for (card, count) in counts.iter() {
                if *count == 2 {
                    return HandType::OnePair(*card);
                }
            }

            panic!("asdfadf")
        }
        5 => {
            let cloned_counts = counts.clone();
            let ranked_cards: Vec<_> = cloned_counts
                .iter()
                .map(|(c, _)| c)
                .sorted_by(|a, b| b.cmp(a))
                .collect();

            return HandType::High(*ranked_cards[0]);
        }
        _ => panic!("{:?} more than 5 or 0 tokens", &counts),
    }
}

fn get_counts(cards: &Vec<Card>) -> Vec<(Card, i32)> {
    let mut counts: HashMap<Card, i32> = HashMap::new();

    for c in cards {
        if counts.contains_key(c) {
            counts.entry(*c).and_modify(|e| *e += 1);
        } else {
            counts.insert(*c, 1);
        }
    }

    let num_js = if let Some(c) = counts.get(&Card::J) {
        c.clone()
    } else {
        0
    };

    let result: Vec<(Card, i32)> = counts
        .iter()
        .filter(|(card, _)| num_js == cards.len() as i32 || card != &&Card::J)
        .map(|(card, num)| (card.clone(), num.clone()))
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect_vec()
        .chunks(2)
        .flat_map(|chunk| match chunk {
            [a, b] => {
                let (c1, n1) = a;
                let (c2, n2) = b;
                let (e1, e2) = if n1 > n2 {
                    ((*c1, n1 + num_js), (*c2, *n2))
                } else if n2 > n1 {
                    ((*c1, *n1), (*c2, n2 + num_js))
                } else if c1.cmp(c2) == Ordering::Greater {
                    ((*c1, n1 + num_js), (*c2, *n2))
                } else {
                    ((*c1, *n1), (*c2, n2 + num_js))
                };
                vec![e1, e2]
            }
            _ => chunk.to_vec(),
        })
        .collect();

    dbg!(&result);

    return result;
}
