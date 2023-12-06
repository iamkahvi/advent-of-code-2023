import { intersection } from "https://deno.land/x/set_operations/mod.ts";

export {};

const text = await Deno.readTextFile("./input1.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

const cardMap = new Map();

for (let i = 0; i < lines.length; i++) {
  cardMap.set(i, 1);
}

const points = lines.map((line, i) => {
  const winningNums = new Set();
  const nums = new Set();

  const firstHalf = line.split(":")[1].split("|")[0];
  const secondHalf = line.split(":")[1].split("|")[1];

  firstHalf.split(" ").filter((x) => Boolean(x)).forEach((str) => {
    winningNums.add(parseInt(str));
  });

  secondHalf.split(" ").filter((x) => Boolean(x)).forEach((str) => {
    nums.add(parseInt(str));
  });

  console.log("\n" + line.split(":")[0]);
  console.log(intersection(winningNums, nums));
  const size = intersection(winningNums, nums).size;

  const copies = cardMap.get(i);
  console.log(`copies: ${copies}`);

  for (let x = 0; x < size; x++) {
    const prevVal = cardMap.get(x + i + 1);

    cardMap.set(x + i + 1, prevVal + copies);

    console.log(`Card ${x + i + 1}: ${prevVal + copies}`);
  }

  // console.log(size, cardMap);
});

const total = Array.from(cardMap.entries()).map(([card, copies]) => {
  console.log(`Card ${card + 1}: ${copies}`);
  return copies;
}).reduce((x, acc) => x + acc, 0);

console.log(total);
