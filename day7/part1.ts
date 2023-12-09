export {};

const text = await Deno.readTextFile("./test1.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

type Hand = {
  cards: string;
  bid: number;
};

const hands = lines.map((line) => {
  const [cards, bidStr] = line.split(" ");

  return {
    cards,
    bid: parseInt(bidStr),
  };
});

function compareCards(a: Hand, b: Hand): number {
  return 0;
}

enum HandType {
  Five,
  Four,
  FullHouse,
  Three,
  Two,
  One,
  High,
}

function isFiveOfAKind(cards: string): boolean {}
function isFourOfAKind(cards: string): boolean {}
function isFullHouse(cards: string): boolean {}
function isThreeOfAKind(cards: string): boolean {}
function isTwoPair(cards: string): boolean {}
function isOnePair(cards: string): boolean {}
function getHighCard(cards: string): string {}

console.log(hands.sort(compareCards));
