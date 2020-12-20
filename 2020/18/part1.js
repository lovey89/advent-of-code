"use strict"

let fs = require('fs');

let input =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(line => {
      let expressionArray = line
        .split("")
        .filter(element => element != ' ')
        .map(element => {
          let t = parseInt(element);
          if (!isNaN(t)) { // You can't compare the NaN value
            return t;
          }
          return element;
        });
      return evaluate(expressionArray);
    })
    .reduce((a, b) => a + b, 0);

console.log(input);

function evaluate(expressionArray) {
  let expressionLength = expressionArray.length;
  return evaluateFromIndex(0).sum;

  function evaluateFromIndex(index) {
    let sum = 0;
    let currentElement = expressionArray[index];
    let nextIndex = index + 1;
    if (!isNaN(currentElement)) {
      sum = currentElement;
    } else { // Should be an opening parenthesis
      ({sum, nextIndex} = evaluateFromIndex(nextIndex));
    }

    while (nextIndex != expressionLength) {
      let nextElement = expressionArray[nextIndex];
      if (nextElement == ')') {
        return {sum, nextIndex: nextIndex + 1};
      } else if (nextElement == "*" || nextElement == "+") {
        let operator = nextElement;
        nextElement = expressionArray[++nextIndex];
        let rightSum;
        if (isNaN(nextElement)) { // opening parenthesis
          ({sum: rightSum, nextIndex} = evaluateFromIndex(nextIndex + 1));
        } else {
          rightSum = nextElement;
          nextIndex++;
        }
        if (operator == "*") {
          sum *= rightSum;
        } else {
          sum += rightSum;
        }
      }
    }
    return {sum, nextIndex};
  }
}
