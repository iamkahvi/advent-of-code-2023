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
};

const digitStrings = Object.keys(digits);

let regex = /(one|two|three|four|five|six|seven|eight|nine)/;

// const replacedText = Object.keys(digits).reduce((acc, d) => {
//   return acc.replaceAll(d, digits[d])
// }, text)

// console.log(replacedText);

const lines = text.split("\n").filter((line) => Boolean(line));

// eig      []
// eigh     []
// eight    []
// 8        [8]
// 8w       [8]
// 8wo      [8]
// ...      [8]
// 8wothree [8]
// 8wo3     [8,3]

const calibrationValues = lines.map((line) => {
  let numsRes: number[] = [];

  console.log(`\n--${line}--\n`);

  let j = 0;
  for (let i = 0; i < line.length; i++) {
    const lineSegment = line.substring(j, i + 1);
    console.log(lineSegment);

    if (/\d/.test(lineSegment)) {
      const digit = lineSegment.match(/\d/g);
      numsRes.push(parseInt(digit));
      j = i + 1;
    } else if (regex.test(lineSegment)) {
      digitStrings.forEach((digitString) => {
        if (lineSegment.includes(digitString)) {
          numsRes.push(digits[digitString]);
          j = i + 1;
        }
      });
    }
    console.log(numsRes);
  }

  if (numsRes.length === 0) return 0;

  if (numsRes.length === 1) return (numsRes[0] * 10 + numsRes[0]);

  return (numsRes[0] * 10 + numsRes[numsRes.length - 1]);
});

console.log(calibrationValues);

console.log(calibrationValues.reduce((acc, x) => acc + x, 0));
