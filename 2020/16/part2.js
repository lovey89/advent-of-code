"use strict"

let fs = require('fs');

let inputGroups =
  fs.readFileSync('input.txt').toString()
    .split("\n\n")
    .filter(Boolean);

let rawRules = inputGroups[0]
  .split('\n')
  .filter(Boolean);

let numberOfRules = rawRules.length;

let rules = rawRules
  .flatMap(rule => {
    let splitRule = rule.split(/: | or /);
    let highLows = splitRule
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

    return {
      ruleName: splitRule[0],
      candidates: Array(numberOfRules).fill(true),
      highLows: highLows
    };
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
  })
  .filter(ticket => validTicket(ticket, rules));

for (let otherTicket of otherTickets) {
  verifyWithTicket(otherTicket, rules);
}

let ruleByIndex = [];

for (let i = 0; i < numberOfRules; i++) {
  let ruleWithSingleAlternative = rules
    .filter(rule => {
      return rule
        .candidates
        .filter(Boolean)
        .length == 1;
    })[0];
  let indexToClaim;
  for (let j = 0; j < numberOfRules; j++) {
    if (ruleWithSingleAlternative.candidates[j]) {
      indexToClaim = j;
      break;
    }
  }
  claimIndex(indexToClaim, rules);
  ruleByIndex[indexToClaim] = ruleWithSingleAlternative.ruleName;
}

let result = 1;

ruleByIndex.forEach((ruleName, index) => {
  if (ruleName.startsWith('departure')) {
    result *= myTicket[index];
  }
});

console.log(result);

function claimIndex(index, rules) {
  for (let rule of rules) {
    rule.candidates[index] = false;
  }
}

function verifyWithTicket(ticket, rules) {
  ticket.forEach((value, index) => {
    for (let rule of rules) {
      if (!matchRule(value, rule)) {
        rule.candidates[index] = false;
      }
    }
  });
}

function validTicket(ticket, rules) {
  for (let value of ticket) {
    if (!isValidValue(value, rules)) {
      return false;
    }
  }
  return true;
}

function isValidValue(value, rules) {
  for (let rule of rules) {
    if (matchRule(value, rule)) {
      return true;
    }
  }
  return false;
}

function matchRule(value, rule) {
  for (let {low, high} of rule.highLows) {
    if (value >= low && value <= high) {
      return true;
    }
  }
  return false;
}
