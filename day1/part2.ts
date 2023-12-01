const text = await Deno.readTextFile("./test1.txt");

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
}

const replacedText = Object.keys(digits).reduce((acc, d) => {
  return acc.replaceAll(d, digits[d])
}, text)

console.log(replacedText);


const lines = replacedText.split("\n").filter((line) => Boolean(line));


const calibrationValues = lines.map((line) => {

  const nums = line.replace(/\D/g,'');

  if (nums.length === 0) {
    return 0;
  }

  if (nums.length === 1) {
    return parseInt(nums[0] + nums[0])
  }

  return parseInt(nums[0] + nums[nums.length - 1])
})

console.log(calibrationValues);

console.log(calibrationValues.reduce((acc, x) => acc + x, 0))
