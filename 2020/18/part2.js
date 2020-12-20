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
  let productSums = [];
  let sum = 0;
  let operator = '+'; // The operator will actually always be plus. Multiplication happens in the end
  let index = 0;

  while (index != expressionLength) {
    let nextElement = expressionArray[index];
    if (!isNaN(nextElement)) {
      sum = applyOperator(operator, sum, nextElement);
    } else if (nextElement == '(') {
      let closingIndex = findClosingParenthesesIndex(index);
      let rightSum = evaluate(expressionArray.slice(index + 1, closingIndex));
      sum = applyOperator(operator, sum, rightSum);
      index = closingIndex;
    } else if (nextElement == '*') {
      productSums.push(sum);
      sum = 0; // Note how we don't change operator. This is because we multiply all products in the end
    }
    index++;
  }

  productSums.push(sum);

  let totalSum = productSums
    .reduce((a, b) => a * b, 1);
  return totalSum;

  function findClosingParenthesesIndex(index) {
    let paranCount = 1

    while (paranCount != 0) {
      let nextElement = expressionArray[++index];
      if (nextElement == '(') {
        paranCount++;
      } else if (nextElement == ')') {
        paranCount--;
      }
    }
    return index;
  }

  function applyOperator(operator, left, right) {
    if (operator == '+') {
      return left + right;
    } else {
      return left * right;
    }
  }
}
