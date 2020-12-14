let fs = require('fs');

let numberArray =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(Number);

for (let i = 25; i < numberArray.length; i++) {
  if (!validNumber(numberArray, i)) {
    console.log(numberArray[i]);
    return;
  }
}

function validNumber(xmasArray, number) {
  let currentNumber = xmasArray[number];

  for (let low = number - 25; low < number - 1; low++) {
    for (let high = low + 1; high < number; high++) {
      if (xmasArray[low] + xmasArray[high] == currentNumber) {
        return true;
      }
    }
  }

  return false;
}

