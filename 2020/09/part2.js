let fs = require('fs');

let secretNumber = 1309761972;

let numberArray =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(Number);

for (let lowerIndex = 0; lowerIndex < numberArray.length; lowerIndex++) {
  let maybeHigherIndex = findHigherIndexInSet(numberArray, lowerIndex);

  if (maybeHigherIndex != null) {
    console.log(findSumOfHighestAndLowestInRange(numberArray, lowerIndex, maybeHigherIndex));
    return;
  }
}

function findSumOfHighestAndLowestInRange(xmasArray, lowerIndex, higherIndex) {
  let low = xmasArray[lowerIndex];
  let high = xmasArray[lowerIndex];

  for (let i = lowerIndex + 1; i <= higherIndex; i++) {
    currentValue = xmasArray[i];
    if (currentValue < low) {
      low = currentValue;
    }
    if (currentValue > high) {
      high = currentValue;
    }
  }
  return low + high;
}

function findHigherIndexInSet(xmasArray, lowerIndex) {
  let sum = 0;

  for (let i = lowerIndex; i < xmasArray.length || sum > secretNumber; i++) {
    sum += xmasArray[i];

    if (sum == secretNumber) {
      return i;
    }
  }

  return null;
}

