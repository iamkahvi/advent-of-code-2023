export {};

const text = await Deno.readTextFile("./input.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

const instructions = lines[0];

type NetworkNode = {
  left: string;
  right: string;
};

const network = new Map<string, NetworkNode>();
const startKeys = [];
const stepsArr = [];

lines.forEach((line, i) => {
  if (i < 1) return;

  // Splitting the input string into two parts at '='
  const parts = line.split("=");

  if (parts.length !== 2) {
    throw new Error("Invalid string format");
  }

  // Extracting the key and trimming any whitespace
  const key = parts[0].trim();
  if (key[2] === "A") startKeys.push(key);

  // Removing parentheses and splitting the inside part by ','
  const values = parts[1].trim().replace("(", "").replace(")", "").split(",");

  if (values.length !== 2) {
    throw new Error("Invalid string format");
  }

  // Trimming any whitespace around left and right values
  const left = values[0].trim();
  const right = values[1].trim();

  network.set(key, { left, right });
});

console.log(network);
console.log(`startKeys: ${startKeys}`);

for (const startKey of startKeys) {
  let steps = 0;
  let currentKey = startKey;

  while (true) {
    const c = instructions[steps % instructions.length];
    console.log(c);

    const currentNode = network.get(currentKey);

    steps++;
    if (c === "L") {
      if (currentNode.left[2] === "Z") {
        break;
      }

      currentKey = currentNode.left;
    } else if (c === "R") {
      if (currentNode.right[2] === "Z") {
        break;
      }
      currentKey = currentNode.right;
    }
  }

  stepsArr.push(steps);

  console.log(`startNode: ${startKey}, steps: ${steps}`);
}

const gcd = (a, b) => a ? gcd(b % a, a) : b;

const lcm = (a, b) => a * b / gcd(a, b);

console.log(stepsArr);
console.log(stepsArr.reduce(lcm));
