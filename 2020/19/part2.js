"use strict"

let fs = require('fs');

let input =
  fs.readFileSync('input2.txt').toString()
    .split("\n\n")
    .filter(Boolean);

let rulesArray =
  input[0]
    .split("\n")
    .map(ruleLine => {
      let splitRule = ruleLine.split(": ");
      let ruleNumber = Number(splitRule[0]);
      let rawRuleDefs = splitRule[1]
        .split(" ")
        .map(rulePart => {
          if (!isNaN(rulePart)) {
            return Number(rulePart);
          }
          return rulePart;
        });
      let regex = null;
      if (rawRuleDefs.length == 1 && (rawRuleDefs[0] == '"a"' || rawRuleDefs[0] == '"b"')) {
        regex = rawRuleDefs[0].charAt(1);
      }
      return {ruleNumber, rawRuleDefs, regex};
    });

let rules = new Map();

for (let rule of rulesArray) {
  rules.set(rule.ruleNumber, rule);
}

let regex = '^' + findRegexForRule(rules, 0) + '$';

let re = new RegExp(regex);

let result = input[1]
  .split("\n")
  .filter(Boolean)
  .filter(line => re.test(line))
  .length;

console.log(result);

function findRegexForRule(rules, ruleNumber) {
  let rule = rules.get(ruleNumber);

  if (rule.regex != null) {
    return rule.regex;
  }

  let regex = rule.rawRuleDefs
      .map(rawRuleDef => {
        if (!isNaN(rawRuleDef)) {
          return findRegexForRule(rules, rawRuleDef);
        }
        return rawRuleDef;
      })
      .join("");

  rule.regex = '(' + regex + ')';
  return rule.regex;
}
