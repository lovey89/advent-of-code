let fs = require('fs');

let opArray =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(rawOp => {
      let rawOpSplit = rawOp.split(" ");
      return {
        op: rawOpSplit[0],
        arg: Number(rawOpSplit[1])
      }
    });


function execute(program, switchOpIndex) {
  let acc = 0;
  let currentIndex = 0;
  let visited = [];

  let pLength = program.length;
  
  while (true) {
    console.log(currentIndex);

    if (visited[currentIndex]) {
      console.log('break');
      return null;
    } else if (currentIndex < 0 || currentIndex >= pLength) {
      return acc;
    }
  
    let currentOp = opArray[currentIndex];
    visited[currentIndex] = true;
    console.log(currentOp);
  
    if ((currentOp.op === 'jmp' && currentIndex === switchOpIndex) || currentOp.op === 'nop') {
      console.log('nop');
      currentIndex++;
    } else if ((currentOp.op === 'nop' && currentIndex === switchOpIndex) || currentOp.op === 'jmp') {
      console.log('jmp');
      currentIndex += currentOp.arg;
    } else {
      console.log('acc');
      acc += currentOp.arg;
      currentIndex++;
    }
  }
  return acc;
}

for (let i = 0; i < opArray.length; i++) {
  let result = execute(opArray, i);
  if (result != null) {
    console.log(result);
    break;
  }
}

//console.log(execute(opArray));

/*
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
*/
