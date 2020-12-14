let fs = require('fs');

let inputLines =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)

let buses = inputLines[1]
  .split(",")
  .map((busId, index) => {
    return {
      busId: Number(busId),
      ix: index
    }
  })
  .filter(busInfo => Boolean(busInfo.busId));

/* https://en.wikipedia.org/wiki/Chinese_remainder_theorem 
   Also keep in mind that all ids are primes making it easy to calculate least common multiple*/
let pos = 0;
let increment = buses[0].busId;
for (let {busId, ix} of buses.slice(1)) {
  while ((pos + ix) % busId != 0) {
    pos += increment;
  }
  increment *= busId;
}

console.log(pos);
