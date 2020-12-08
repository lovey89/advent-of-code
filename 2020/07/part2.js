let fs = require('fs');

let bagData =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(rule => {
      let ruleSplit = rule.match(/^(.+) bags contain (.+)/);
      let mainBag = ruleSplit[1];
      let childNodes = [...ruleSplit[2].matchAll(/(\d) (.+?) bags?[.,]/g)]
        .map(match => {
          let n = Number(match[1]);
          let type = match[2];
          return {
            bagType: type,
            quanitity: n
          }
        });

      return {
        mainBag: mainBag,
        childrenBags: childNodes
      };
    });

let bagMap = new Map(bagData.map(i => [i.mainBag, i.childrenBags]));
let resultMap = new Map();

function numberOfBags(bagType) {
  if (resultMap.has(bagType)) {
    return resultMap.get(bagType);
  }

  let childNodes = bagMap.get(bagType);
  let result;  

  if (childNodes.length == 0) {
    result = 1;
  } else {
    result = childNodes
      .map(childBag => {
        return childBag.quanitity * numberOfBags(childBag.bagType);
      })
      .reduce((a, b) => a + b, 0) + 1;
  }
  
  resultMap.set(bagType, result);
  return result;
}

console.log(numberOfBags('shiny gold') - 1);

//console.log(JSON.stringify(bagData, null, 4));
/*
    .map(groupData => {
      return (new Set(groupData.replace(/\n/g,"").split('')).size);
    });

console.log(resultsPerGroup.reduce((accumulator, currentValue) => accumulator + currentValue));
*/
