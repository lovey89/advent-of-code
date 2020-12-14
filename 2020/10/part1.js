let fs = require('fs');

let sortedAdapters =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(Number)
    .sort((a, b) => a - b); // By default it converts the values to string and string compare them...

let lastJolt = 0;
let noOfOneDiffs = 0;
let noOfThreeDiffs = 1; // There will be 3 volts between the last adapter and the device

for (adapterJolt of sortedAdapters) {
  let diff = adapterJolt - lastJolt;

  if (diff == 1) {
    noOfOneDiffs++;
  } else if (diff == 3) {
    noOfThreeDiffs++;
  }

  lastJolt = adapterJolt;
}
console.log(noOfOneDiffs * noOfThreeDiffs);
