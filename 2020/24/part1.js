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
  let key = x + "," + y;

  let value = tileInfo.get(key);
  tileInfo.set(key, !value);
}

let sum = 0;
tileInfo.forEach(v => {
  if (v) {
    sum++;
  }
})

console.log(sum);
