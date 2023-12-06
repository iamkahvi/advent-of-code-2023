import { intersection } from "https://deno.land/x/set_operations/mod.ts";

export {};

const text = await Deno.readTextFile("./input1.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

const points = lines.map((line) => {
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

  // console.log(winningNums, nums);
  console.log(intersection(winningNums, nums));
  const size = intersection(winningNums, nums).size;
  return Math.floor(Math.pow(2, size - 1));
});

console.log(points.reduce((x, acc) => x + acc));
