let fs = require('fs');

let inputLines =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)

let earliestTime = Number(inputLines[0]);
let buses = inputLines[1]
  .split(",")
  .filter(busId => busId != 'x')
  .map(Number);


let busInfoSortedByWaitingTime = buses
  .map(busId => {
    let waitingTime = Math.ceil(earliestTime/busId) * busId - earliestTime;
    return {
      busId: busId,
      waitingTime: waitingTime
    };
  })
  .sort((a, b) => a.waitingTime - b.waitingTime);


let shortestWaitTimeBus = busInfoSortedByWaitingTime[0];
console.log(shortestWaitTimeBus.busId * shortestWaitTimeBus.waitingTime);
