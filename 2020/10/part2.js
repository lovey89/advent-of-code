let fs = require('fs');

let sortedAdapters =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(Number)
    .sort((a, b) => a - b); // By default it converts the values to string and string compare them...

sortedAdapters.unshift(0); // Add the charging outlet to the list as well

let lengthForIndex = new Map();

function alternativesToReachEnd(sortedAdapters, index) {
  if (lengthForIndex.has(index)) {
    return lengthForIndex.get(index);
  }

  if (index == sortedAdapters.length - 1) {
    lengthForIndex.set(index, 1);
    return 1;
  }

  let sumOfAlternatives = 0;
  
  for (let nextIndex = index + 1; nextIndex < sortedAdapters.length; nextIndex++) {
    if (sortedAdapters[nextIndex] - sortedAdapters[index] <= 3) {
      sumOfAlternatives += alternativesToReachEnd(sortedAdapters, nextIndex);
    } else {
      break;
    }
  }
  lengthForIndex.set(index, sumOfAlternatives);
  return sumOfAlternatives;
}

console.log(alternativesToReachEnd(sortedAdapters, 0));

