const text = await Deno.readTextFile("./input2.txt");

const digits = {
  "one": 1,
  "two": 2,
  "three": 3,
  "four": 4,
  "five": 5,
  "six": 6,
  "seven": 7,
  "eight": 8,
  "nine": 9,
  "1": 1,
  "2": 2,
  "3": 3,
  "4": 4,
  "5": 5,
  "6": 6,
  "7": 7,
  "8": 8,
  "9": 9,
};

const digitStrings = Object.keys(digits);

const lines = text.split("\n").filter((line) => Boolean(line));

const calibrationValues = lines.map((line) => {
  let numsRes: number[] = [];

  console.log(`\n--${line}--\n`);

  let first = null;
  let last = null;
  let i = 0;

  while (!first || !last) {
    const startLineSegment = line.substring(i, i + 5);
    const endLineSegment = line.substring(line.length - 5 - i, line.length - i);

    const startMatchedDigit = digitStrings.find((digit) => {
      return startLineSegment.startsWith(digit);
    });
    const endMatchedDigit = digitStrings.find((digit) => {
      return endLineSegment.endsWith(digit);
    });

    // console.log(`s: ${startLineSegment}`);
    if (!first && startMatchedDigit) {
      first = digits[startMatchedDigit];
      console.log(`match: ${startMatchedDigit}`);
    }

    // console.log(`e: ${endLineSegment}`);
    if (!last && endMatchedDigit) {
      last = digits[endMatchedDigit];
      console.log(`match: ${endMatchedDigit}`);
    }

    if (i + 1 >= line.length) break;

    i++;
  }

  if (first === null && last === null) return 0;

  if (first === null && last !== null) return last * 10 + last;

  if (first !== null && last === null) return first * 10 + first;

  return (first! * 10 + last!);
});

//  29, 83, 13, 24, 42, 14, and 76.

console.log(calibrationValues);

console.log(calibrationValues.reduce((acc, x) => acc + x, 0));
