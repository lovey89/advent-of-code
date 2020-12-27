"use strict"

let fs = require('fs');

let cups =
  fs.readFileSync('input.txt').toString()
    .split("")
    .filter(c => c != '\n')
    .map(Number);

const SIZE = 9;

for (let i = 0; i < 100; i++) {
  let currentCupIndex = i % SIZE;
  const following0 = (currentCupIndex + 1) % SIZE;
  const following1 = (currentCupIndex + 2) % SIZE;
  const following2 = (currentCupIndex + 3) % SIZE;

  let currentCupLabel = cups[currentCupIndex];
  let destinationIndex;
  for (let j = 1; j <= 4; j++) {
    let tmp = (currentCupLabel + SIZE - j - 1) % SIZE + 1;
    destinationIndex = cups.indexOf(tmp);
    if (destinationIndex != following0 && destinationIndex != following1 && destinationIndex != following2) {
      break;
    }
  }

  move3(currentCupIndex, destinationIndex);
}

let indexOf1 = cups.indexOf(1);

for (let i = 1; i < SIZE; i++) {
  process.stdout.write("" + cups[(indexOf1 + i) % SIZE]);
}
process.stdout.write('\n');

function move3(fromIndex, toIndex) {
  let cup0 = cups[(fromIndex + 1) % SIZE];
  let cup1 = cups[(fromIndex + 2) % SIZE];
  let cup2 = cups[(fromIndex + 3) % SIZE];

  let currentIndex = fromIndex;
  do {
    currentIndex = (currentIndex + 1) % SIZE;
    cups[currentIndex] = cups[(currentIndex + 3) % SIZE];
  } while (((currentIndex + 3) % SIZE) != toIndex);

  cups[(currentIndex + 1) % SIZE] = cup0;
  cups[(currentIndex + 2) % SIZE] = cup1;
  cups[(currentIndex + 3) % SIZE] = cup2;
}
