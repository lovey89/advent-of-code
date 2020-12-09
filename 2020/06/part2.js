let fs = require('fs');

let resultsPerGroup =
  fs.readFileSync('input.txt').toString()
    .split("\n\n")
    .map(groupData => {
      let personDatas =
        groupData
          .split("\n")
          .map(personData => {
            return new Set(personData.split(''));
          })
          .filter(x => x.size != 0);
      let res = personDatas.reduce((accumulator, currentValue) => {
        return new Set(Array.from(currentValue).filter(x => accumulator.has(x)));
      }, personDatas[0]);

      return res.size;
    });

console.log(resultsPerGroup.reduce((accumulator, currentValue) => accumulator + currentValue));
