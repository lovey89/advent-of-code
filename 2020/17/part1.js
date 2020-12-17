"use strict"

let fs = require('fs');

let input =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(line => line.split(""));

const generations = 6;
const zMax = generations * 2 + 1;
const yMax = generations * 2 + input.length;
const xMax = generations * 2 + input[0].length;

let initialCube = createCube();

for (let y = 0; y < input.length; y++) {
  for (let x = 0; x < input[0].length; x++) {
    initialCube[generations][generations + y][generations + x] = input[y][x];
  }
}

let cube = initialCube;
for (let i = 0; i < generations; i++) {
  cube = nextGen(cube);
}

console.log(countActive(cube));

function countActive(cube) {
  let count = 0;
  for (let z = 0; z < zMax; z++) {
    for (let y = 0; y < yMax; y++) {
      for (let x = 0; x < xMax; x++) {
        if (cube[z][y][x] == '#') {
          count++;
        }
      }
    }
  }
  return count;
}

function nextGen(cube) {
  let nextGenCube = createCube();

  for (let z = 0; z < generations * 2 + 1; z++) {
    for (let y = 0; y < yMax; y++) {
      for (let x = 0; x < xMax; x++) {
        let count = neighbors(cube, x, y, z);
        let char = cube[z][y][x];
        if (char == '.' && count == 3) {
          nextGenCube[z][y][x] = '#';
        } else if (char == '#' && !(count == 3 || count == 2)) {
          nextGenCube[z][y][x] = '.';
        } else {
          nextGenCube[z][y][x] = char;
        }
      }
    }
  }
  return nextGenCube;
}

function neighbors(cube, xc, yc, zc) {
  let count = 0;
  for (let z = Math.max(0, zc - 1); z < Math.min(zMax, zc + 2); z++) {
    for (let y = Math.max(0, yc - 1); y < Math.min(yMax, yc + 2); y++) {
      for (let x = Math.max(0, xc - 1); x < Math.min(xMax, xc + 2); x++) {
        if (x == xc && y == yc && z == zc) {
          continue;
        }
        let char = cube[z][y][x];
        if (char == '#') {
          count++;
        }
      }
    }
  }
  return count;
}

function createCube() {
  return Array(zMax)
    .fill(null)
    .map(ignore => Array(yMax)
      .fill(null)
      .map(ignore2 => Array(xMax).fill(".")));
}
