const text = await Deno.readTextFile("./input1.txt");

const lines = text.split("\n").filter((line) => Boolean(line));

interface SetRecord {
  blue: number;
  red: number;
  green: number;
}

interface GameRecord {
  id: number;
  sets: SetRecord[];
}

const games: GameRecord[] = lines.map((line) => {
  const setSplits = line.split(/[:;]/);
  const id = parseInt(setSplits[0].match(/\d+/)[0]);

  const sets = setSplits.slice(1).map((setStr) => {
    return setStr.split(",").reduce((set, cubeStr) => {
      const [_, number, color] = cubeStr.match(/(\d+)\s+(\w+)/);

      set[color] = parseInt(number);
      return set;
    }, { blue: 0, red: 0, green: 0 });
  });

  return { id, sets };
});

// only 12 red cubes, 13 green cubes, and 14 blue cubes

const maxGameSets = games.map((game) => {
  let maxBlue = 0;
  let maxRed = 0;
  let maxGreen = 0;

  const { id, sets } = game;

  sets.map((set) => {
    if (set.blue >= maxBlue) maxBlue = set.blue;
    if (set.red >= maxRed) maxRed = set.red;
    if (set.green >= maxGreen) maxGreen = set.green;
  });

  return {
    id,
    maxSet: {
      blue: maxBlue,
      red: maxRed,
      green: maxGreen,
    },
  };
});

// console.log(games);
console.log(maxGameSets);

const total = maxGameSets.reduce((total, game) => {
  console.log(total, game);
  const { blue, red, green } = game.maxSet;
  return total + (blue * red * green);
}, 0);

console.log(total);
