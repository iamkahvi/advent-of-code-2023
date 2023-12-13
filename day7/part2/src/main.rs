use itertools::Itertools;
use std::collections::HashMap;

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

    let lines = include_str!("../../test1.txt")
        .lines()
        .collect::<Vec<&str>>();

    dbg!(&lines);

    let hands: Vec<(Vec<Card>, Hand)> = lines
        .iter()
        .map(|line| {
            if let [first, second, ..] = &line.split_whitespace().collect::<Vec<_>>()[0..2] {
                let cards: Vec<Card> = first
                    .chars()
                    .into_iter()
                    .map(|c| match Card::from_char(&c) {
                        Some(card) => card,
                        _ => panic!("asfadf"),
                    })
                    .collect();

                dbg!(&cards);

                let ht = parse_handtype(&cards);
                let bid = second.to_string().parse::<i32>().unwrap();

                return (cards, Hand { ht, bid });
            } else {
                panic!("aahhhh");
            }
        })
        .collect();

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
    let counts = get_counts(&cards);

    // println!("{:?}", counts);
    // println!("{:?}", counts_vec);

    // replace Card::J with the best card in the hand

    match counts.len() {
        1 => {
            let c = counts[0].0;
            return HandType::Five(c);
        }
        2 => {
            let (c1, n1) = counts[0];
            let (c2, n2) = counts[1];

            match ((c1, n1), (c2, n2)) {
                ((card1, 1), (card2, 4)) => {
                    if card2 == Card::J {
                        HandType::Five(card1)
                    } else if card1 == Card::J {
                        HandType::Five(card2)
                    } else {
                        HandType::Four(card2)
                    }
                }
                ((card1, 2), (card2, 3)) => match (card1 == Card::J, card2 == Card::J) {
                    (true, false) => HandType::Five(card2),
                    (false, true) => HandType::Five(card1),
                    _ => HandType::FullHouse(card2, card1),
                },
                ((card1, 3), (card2, 2)) => match (card1 == Card::J, card2 == Card::J) {
                    (true, false) => HandType::Five(card2),
                    (false, true) => HandType::Five(card1),
                    _ => HandType::FullHouse(card1, card2),
                },
                ((card1, 4), (card2, 1)) => {
                    if card2 == Card::J {
                        HandType::Five(card1)
                    } else if card1 == Card::J {
                        HandType::Five(card2)
                    } else {
                        HandType::Four(card1)
                    }
                }
                _ => panic!("AAhh"),
            }
        }
        3 => {
            let (c1, n1) = counts[0];
            let (c2, n2) = counts[1];
            let (c3, n3) = counts[2];

            // 2,2,1
            // 3,1,1

            if c1 == Card::J {
                match n1 {
                    2 => return HandType::Four(c2),
                    3 => {
                        if c2 > c3 {
                            return HandType::Four(c2);
                        } else {
                            return HandType::Four(c3);
                        }
                    }
                    _ => panic!("unknown count {}", n1),
                }
            }

            if c2 == Card::J {
                match n2 {
                    1 | 2 => return HandType::Four(c1),
                    _ => panic!("uh oh"),
                }
            }

            if c3 == Card::J {
                match n1 {
                    2 => {
                        if c1 > c2 {
                            return HandType::Three(c1);
                        } else {
                            return HandType::Three(c2);
                        }
                    }
                    3 => return HandType::Four(c1),
                    _ => panic!("ADADDAA"),
                }
            }

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
                    if card == &Card::J {
                        let max = cards.iter().max().unwrap();
                        return HandType::Three(*max);
                    }
                    if cards.contains(&Card::J) {
                        return HandType::Three(*card);
                    }
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

            if cards.contains(&Card::J) {
                return HandType::OnePair(*ranked_cards[0]);
            }

            return HandType::High(*ranked_cards[0]);
        }
        _ => panic!("{:?} more than 5 tokens", &counts),
    }
}

fn get_counts(cards: &Vec<Card>) -> Vec<(Card, i32)> {
    let best = cards.iter().max().unwrap();
    let mut counts: HashMap<Card, i32> = HashMap::new();

    for c in cards {
        if counts.contains_key(c) {
            counts.entry(*c).and_modify(|e| *e += 1);
        } else {
            if c == &Card::J {
                counts.entry(*best).and_modify(|e| *e += 1);
            }
            counts.insert(*c, 1);
        }
    }

    let result: Vec<_> = counts
        .clone()
        .iter()
        .map(|(card, num)| (card.clone(), num.clone()))
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect();

    dbg!(&result);

    return result;
}
