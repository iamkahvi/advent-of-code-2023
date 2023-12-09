export {};

const text = await Deno.readTextFile("./input1.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

const input = lines.map((line) => {
  return line.split(" ").filter((t) => Boolean(t)).flatMap((token) => {
    if (!isNaN(token)) return [parseInt(token)];
    return [];
  });
})
  .filter((l) => Boolean(l));

const times = input[0];
const distances = input[1];

console.log(times);
console.log(distances);

function getDistance(holdTime: number, raceTime: number) {
  const distance = holdTime * (raceTime - holdTime);

  return distance;
}

const res = times.map((t, i) => {
  console.log(t);

  const winners = [...Array(t).keys()].map((ht) => {
    const d = getDistance(ht, t);

    console.log(`hold: ${ht} - distance: ${d}`);

    return d;
  }).filter((d) => d > distances[i]);

  console.log(winners.length);
  return winners.length;
});

console.log(res.reduce((acc, x) => acc * x));
