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

function hasShinyGold(bagType) {
  if (resultMap.has(bagType)) {
    return resultMap.get(bagType);
  }

  let result;
  if (bagType === 'shiny gold') {
    result = true;
  } else {
    result = bagMap.get(bagType)
                   .some(childBag => {
                     return hasShinyGold(childBag.bagType)
                   });
  }
  resultMap.set(bagType, result);
  return result;
}

console.log(Array.from(bagMap.keys())
     .map(hasShinyGold)
     .filter(Boolean)
     .length - 1);
