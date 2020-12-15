let fs = require('fs');

let startingNumbers =
  fs.readFileSync('input.txt').toString()
    .split(",")
    .map(Number);

let memoryMap = new Map();

for (let i = 0; i < startingNumbers.length; i++) {
  let number = startingNumbers[i];

  memoryMap.set(number, [i]);
}

let lastNumber = startingNumbers[startingNumbers.length - 1];

for (let i = startingNumbers.length; i < 30000000; i++) {
  let numberIndexes = memoryMap.get(lastNumber);

  let arrLength = numberIndexes.length;
  if (arrLength == 1) {
    lastNumber = 0;
  } else {
    lastNumber = numberIndexes[arrLength - 1] - numberIndexes[arrLength - 2];
  }
  pushToMap(lastNumber, i);
}
console.log(lastNumber);

function pushToMap(key, value) {
  let array = memoryMap.get(key);
  if (array) {
    array.push(value);
  } else {
    memoryMap.set(key, [value]);
  }
}
