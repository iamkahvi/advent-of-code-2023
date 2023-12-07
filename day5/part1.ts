export {};

import {
  fertilizerToWater,
  humidityToLocation,
  lightToTemp,
  seeds,
  seedToSoil,
  soilToFertilizer,
  tempToHumidity,
  waterToLight,
} from "./map.mjs";

// const text = await Deno.readTextFile("./test1.txt");

// const lines = text.split("\n").filter((line) => Boolean(line));

const seedToSoilMap = new Map();

seedToSoil.forEach((arr, i) => {
  const [destStart, sourceStart, range] = arr;

  // console.log(destStart, sourceStart, range);

  for (let i = 0; i < range; i++) {
    seedToSoilMap.set(destStart + i, sourceStart + i);
  }
});

console.log(seedToSoilMap);

enum SoilType {
  X,
}

type Seed = {
  soil: SoilType;
  fertilizer: string;
  water: string;
  light: string;
  temp: string;
  humidity: string;
};
