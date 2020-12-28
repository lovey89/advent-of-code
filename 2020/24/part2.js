"use strict"

let fs = require('fs');

let hexCoords =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(row => {
      return row
        .split(/(e|w|ne|nw|se|sw)/g)
        .filter(Boolean);
    });

let tileInfo = new Map;

for (let hexCoord of hexCoords) {
  let x = 0;
  let y = 0;

  for (let step of hexCoord) {
    switch (step) {
      case "w":
        x -= 2;
        break;
      case "e":
        x += 2;
        break;
      case "nw":
        y++;
        x--;
        break;
      case "ne":
        y++;
        x++;
        break;
      case "sw":
        y--;
        x--;
        break;
      case "se":
        y--;
        x++;
        break;
    }
  }
  setCoord(tileInfo, x, y, !isCoordBlack(tileInfo, x, y));
}

for (let i = 0; i < 100; i++) {
  tileInfo = nextGen(tileInfo);
}

console.log(tileInfo.size);

function isCoordBlack(map, x, y) {
  return map.has(x + "," + y);
}

function setCoord(map, x, y, value) {
  if (value) {
    map.set(x + "," + y, {x, y});
  } else {
    map.delete(x + "," + y);
  }
}

function nextGen(lastGen) {
  let newGen = new Map();
  lastGen.forEach(({x, y}) => {
    let adjacentBlacks = numberOfAdjacentBlacks(lastGen, x, y);

    if (adjacentBlacks == 1 || adjacentBlacks == 2) {
      setCoord(newGen, x, y, true);
    }
    currentWhiteNextGen(lastGen, newGen, x - 1, y + 1);
    currentWhiteNextGen(lastGen, newGen, x + 1, y + 1);
    currentWhiteNextGen(lastGen, newGen, x - 2, y);
    currentWhiteNextGen(lastGen, newGen, x + 2, y);
    currentWhiteNextGen(lastGen, newGen, x - 1, y - 1);
    currentWhiteNextGen(lastGen, newGen, x + 1, y - 1);
  })
  return newGen;
}

function numberOfAdjacentBlacks(lastGen, x, y) {
  return [isCoordBlack(lastGen, x - 1, y + 1), isCoordBlack(lastGen, x + 1, y + 1),
          isCoordBlack(lastGen, x - 2, y), isCoordBlack(lastGen, x + 2, y),
          isCoordBlack(lastGen, x - 1, y - 1), isCoordBlack(lastGen, x + 1, y - 1)]
    .filter(Boolean)
    .length;
}

function currentWhiteNextGen(lastGen, newGen, x, y) {
  if (isCoordBlack(lastGen, x, y)) {
    return;
  }
  let adjacentBlacks = numberOfAdjacentBlacks(lastGen, x, y);
  if (adjacentBlacks == 2) {
    setCoord(newGen, x, y, true);
  }
}
