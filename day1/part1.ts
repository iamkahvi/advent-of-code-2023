const text = await Deno.readTextFile("./input.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

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
