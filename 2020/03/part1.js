let fs = require('fs');

function treesFound(map, stepSize) {
  let mapWidth = map[0].length;

  let x = stepSize;
  let foundTrees = 0;

  for (let i = 1; i < map.length; i++, x = (x + stepSize) % mapWidth) {
    if (map[i].charAt(x) == '#') {
      foundTrees++;
    }
  }
  return foundTrees;
}

let array = fs.readFileSync('input.txt').toString()
              .split("\n")
              .filter(Boolean); // Removes the trailing empty string

console.log(treesFound(array, 3));
