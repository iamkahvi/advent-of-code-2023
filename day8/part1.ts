const text = await Deno.readTextFile("./input.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

const instructions = lines[0];

type NetworkNode = {
  left: string;
  right: string;
};

const network = new Map<string, NetworkNode>();

lines.forEach((line, i) => {
  if (i < 1) return;

  // Splitting the input string into two parts at '='
  const parts = line.split("=");

  if (parts.length !== 2) {
    throw new Error("Invalid string format");
  }

  // Extracting the key and trimming any whitespace
  const key = parts[0].trim();

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

let currentNode = network.get("AAA");
let steps = 0;

while (true) {
  if (!currentNode) {
    throw new Error("Invalid node");
  }

  const c = instructions[steps % instructions.length];
  console.log(c);
  console.log(currentNode);

  steps++;
  if (c === "L") {
    if (currentNode.left === "ZZZ") {
      break;
    }

    currentNode = network.get(currentNode.left);
  } else if (c === "R") {
    if (currentNode.right === "ZZZ") {
      break;
    }

    currentNode = network.get(currentNode.right);
  }
}

console.log(steps);
