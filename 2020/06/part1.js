let fs = require('fs');

let resultsPerGroup =
  fs.readFileSync('input.txt').toString()
    .split("\n\n")
    .map(groupData => {
      return (new Set(groupData.replace(/\n/g,"").split('')).size);
    });

console.log(resultsPerGroup.reduce((accumulator, currentValue) => accumulator + currentValue));
