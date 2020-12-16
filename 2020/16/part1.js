"use strict"

let fs = require('fs');

let inputGroups =
  fs.readFileSync('input.txt').toString()
    .split("\n\n")
    .filter(Boolean);

let rules = inputGroups[0]
  .split('\n')
  .filter(Boolean)
  .flatMap(rule => {
    let splitRule = rule.split(/: | or /);
    return splitRule
      .splice(1)
      .map(r => {
        let lowHigh = r
          .split('-')
          .map(Number)
        return {
          low: lowHigh[0],
          high: lowHigh[1]
        }
      });
  });

let myTicket = inputGroups[1]
  .split('\n')[1]
  .split(',')
  .map(Number);

let otherTickets = inputGroups[2]
  .split('\n')
  .filter(Boolean)
  .splice(1)
  .map(ticket => {
    return ticket
      .split(',')
      .map(Number);
  });

let invalidSum = 0;

for (let otherTicket of otherTickets) {
  invalidSum += otherTicket
    .filter(field => !isValid(field, rules))
    .reduce((a,b) => a + b, 0);
}

console.log(invalidSum);

function isValid(value, rules) {
  for (let {low, high} of rules) {
    if (value >= low && value <= high) {
      return true;
    }
  }
  return false;
}
