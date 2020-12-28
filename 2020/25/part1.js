"use strict"

let fs = require('fs');

let publicKeys =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(Number);

let cardPublicKey = publicKeys[0];
let doorPublicKey = publicKeys[1];

let cardLoopSize = findLoopSize(cardPublicKey);
//let doorLoopSize = findLoopSize(doorPublicKey);

console.log(calculate(doorPublicKey, cardLoopSize));
//console.log(calculate(cardPublicKey, doorLoopSize));

function findLoopSize(publicKey) {
  let subjectNumber = 1;

  let i = 0;
  while (subjectNumber != publicKey) {
    i++;
    subjectNumber = (subjectNumber * 7) % 20201227;
  }
  return i;
}

function calculate(publicKey, loopSize) {
  let subjectNumber = 1;
  for (let i = 0; i < loopSize; i++) {
    subjectNumber = (subjectNumber * publicKey) % 20201227;
  }
  return subjectNumber;
}
