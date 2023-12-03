export {};

const text = await Deno.readTextFile("./input1.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

let total = 0;

const isAdjacentMap = lines.map((line, i, lines) => {
  return line.split("").map((token, j, tokens) => {
    if (isSymbol(token)) return token;

    const above = lines[Math.max(i - 1, 0)][j];
    const left = lines[i][Math.max(j - 1, 0)];
    const right = lines[i][Math.min(j + 1, tokens.length - 1)];
    const below = lines[Math.min(i + 1, lines.length - 1)][j];

    const tr = lines[Math.max(i - 1, 0)][Math.min(j + 1, tokens.length - 1)];
    const tl = lines[Math.max(i - 1, 0)][Math.max(j - 1, 0)];
    const bl = lines[Math.min(i + 1, lines.length - 1)][Math.max(j - 1, 0)];
    const br = lines[Math.min(i + 1, lines.length - 1)][
      Math.min(j + 1, tokens.length - 1)
    ];

    const isMatch = isSymbol(above) ||
      isSymbol(left) ||
      isSymbol(right) ||
      isSymbol(below) ||
      isSymbol(tr) ||
      isSymbol(tl) ||
      isSymbol(bl) ||
      isSymbol(br);

    return isMatch ? true : false;
  });
});

lines.map((line, i, lines) => {
  console.log(line);
  return line.split("").map((token, j, tokens) => {
    const numberStack: number[] = [];

    const isNumber = !isNaN(token);
    const nextIsNumber = !isNaN(tokens[j + 1]);
    const prevIsNumber = !isNaN(tokens[j - 1]);

    if (prevIsNumber) {
      return;
    }

    if (isNumber) {
      let x = j;
      let isNumAdjacent = false;

      while (!isNaN(tokens[x])) {
        if (isAdjacentMap[i][x]) isNumAdjacent = true;
        numberStack.push(parseInt(tokens[x]));

        if (x >= tokens.length - 1) break;

        x++;
      }

      if (isNumAdjacent) {
        console.log(sumArr(numberStack));
        total += sumArr(numberStack);
      }

      // clearing an array like this?
      numberStack.length = 0;
    }
  });
});

function toString(grid: string[][]): string {
  return grid.map((line) => {
    return line.map((t) => {
      return t ? "X" : "-";
    }).join("") + "\n";
  }).join("");
}

function sumArr(arr: number[]): number {
  return arr.reduce((total, num, i) => {
    const multiplier = Math.pow(10, arr.length - i - 1);
    return total + num * multiplier;
  }, 0);
}

function isSymbol(token: string): boolean {
  return Boolean(token.match(/[^.\d]/g));
}

console.log(toString(isAdjacentMap));
console.log(total);
