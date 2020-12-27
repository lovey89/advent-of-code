"use strict"

let fs = require('fs');

let input =
  fs.readFileSync('input.txt').toString()
    .split("")
    .filter(c => c != '\n')
    .map(Number);

const SIZE = 1000000;

let succ = [];

for (let i = 1; i < SIZE; i++) {
  succ[i] = i + 1;
}
// Make the very last number point towards the first number in the input
succ[SIZE] = input[0];

for (let i = 0; i < input.length; i++) {
  let cup = input[i];
  succ[input[i]] = input[(i + 1) % SIZE];
}
// Make the last number in the input point towards the correct next value
succ[input[input.length - 1]] = input.length + 1;

console.log(succ.slice(0,11));

let currentCupLabel = input[0];

for (let i = 0; i < 10000000; i++) {
  const following0 = succ[currentCupLabel];
  const following1 = succ[following0];
  const following2 = succ[following1];

  let destinationCupLabel;
  for (let j = 1; j <= 4; j++) {
    destinationCupLabel = (currentCupLabel + SIZE - j - 1) % SIZE + 1;
    if (destinationCupLabel != following0 && destinationCupLabel != following1 && destinationCupLabel != following2) {
      break;
    }
  }

  [succ[currentCupLabel], succ[following2], succ[destinationCupLabel]] =
    [succ[following2], succ[destinationCupLabel], succ[currentCupLabel]];

  currentCupLabel = succ[currentCupLabel];
}


let num0 = succ[1];
let num1 = succ[num0];

console.log(num0, num1, num0 * num1);
