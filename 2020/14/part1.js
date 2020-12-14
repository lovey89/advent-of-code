let fs = require('fs');

let instructions =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(line => {
      let splittedLine = line.split(" = ");
      let rawAddr = splittedLine[0];
      let rawArg = splittedLine[1];
      let addr = 'mask';
      let arg = splittedLine[1];
      if (rawAddr != 'mask') {
        let match = rawAddr.match(/^mem\[(\d+)\]/)
        addr = match[1];
        arg = Number(rawArg);
      }
      return {
        addr: addr,
        arg: arg
      };
    });

let bits31 = Math.pow(2, 31);
let bits31mask = bits31 - 1;
let andMask = 0;
let orMask = 0;
let resultMap = new Map();

for (let instruction of instructions) {
  execute(instruction);
}

let sum = [...resultMap.values()]
  .reduce((a, b) => a + b, 0);

console.log(sum);

function execute(instruction) {
  let addr = instruction.addr;

  if (addr == 'mask') {
    calculateMask(instruction.arg);
  } else {
    /* JavaScript only works with 32 bytes when doing bitwise operations (also for shifting
       and thats why I shift mathematically). It also treats them as signed numbers so here
       I only work with 31 bits at a time.*/
    let arg = instruction.arg;
    let high = (((Math.floor(arg / bits31) & Math.floor(andMask / bits31)) | Math.floor(orMask / bits31)) * bits31);
    let low = (((arg & bits31mask) & (andMask & bits31mask)) | (orMask & bits31mask));
    
    resultMap.set(addr, high + low);
  }
}

function calculateMask(maskString) {
  andMask = 0;
  orMask = 0;
  let charArray = maskString.split("");

  for (let i = 0; i < 36; i++) {
    let char = charArray[i];
    let value = Math.pow(2, 35 - i);
    if (char == '1') {
      orMask += value;
      andMask += value;
    } else if (char == 'X') {
      andMask += value;
    }
  }
}
