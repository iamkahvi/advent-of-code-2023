export {};

const text = await Deno.readTextFile("./test1.txt");

const lines = text.split("\n").filter((line) => Boolean(line)).map((line) => {
  return line.split(" ").map((x) => parseInt(x));
});

// console.log(lines);

const res = [];

for (const history of lines) {
  // console.log(history);
  const diffs = [history];
  let i = 1;

  while (true) {
    diffs[i] = [];
    for (let j = 0; j < history.length - i; j++) {
      const diff = diffs[i - 1][j + 1] - diffs[i - 1][j];
      diffs[i][j] = diff;
    }

    // console.log(diffs[i]);

    if (diffs[i].every((x) => x === 0)) break;

    i++;
  }

  console.log(diffs);

  const tally = [];
  for (let i = 0; i < diffs.length; i++) {
    const j = diffs.length - i - 1;
    const diff = diffs[j];

    const lastEl = diff[diff.length - 1];

    tally.push(lastEl === 0 ? 0 : lastEl + tally[i - 1]);
  }
  console.log(tally);
  res.push(tally[tally.length - 1]);

  // console.log(tally);

  console.log("-----");
}

console.log(res);
console.log(res.reduce((a, b) => a + b));
