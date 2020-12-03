let fs = require('fs');

function treesFound(map, xStepSize, yStepSize) {
  let mapWidth = map[0].length;

  let x = xStepSize;
  let foundTrees = 0;

  for (let i = yStepSize; i < map.length; i += yStepSize, x = (x + xStepSize) % mapWidth) {
    if (map[i].charAt(x) == '#') {
      foundTrees++;
    }
  }
  return foundTrees;
}

let array = fs.readFileSync('input.txt').toString()
              .split("\n")
              .filter(Boolean); // Removes the trailing empty string

console.log(
  treesFound(array, 1, 1) *
  treesFound(array, 3, 1) *
  treesFound(array, 5, 1) *
  treesFound(array, 7, 1) *
  treesFound(array, 1, 2));
