#[derive(Debug)]
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

#[derive(Debug)]
enum HandType {
    Five(Card),
    Four(Card),
    FullHouse(Card, Card),
    Three(Card),
    TwoPair(Card),
    OnePair(Card),
    High(Card),
}

fn main() {
    println!("Hello, world!");
}
