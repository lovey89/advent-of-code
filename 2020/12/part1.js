let fs = require('fs');

let instructions =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(rawInstruction => {
      let logicalInstruction = rawInstruction;
      if (rawInstruction == 'L180' || rawInstruction == 'R180') {
        logicalInstruction = 'B0';
      } else if (rawInstruction == 'L270') {
        logicalInstruction = 'R90';
      } else if (rawInstruction == 'R270') {
        logicalInstruction = 'L90';
      }
      return {
        op: logicalInstruction.charAt(0),
        arg: Number(logicalInstruction.substring(1))
      };
    });

let fX = 1;
let fY = 0;
let x = 0;
let y = 0;

for (let instruction of instructions) {
  executeInstruction(instruction);
}

console.log(Math.abs(x) + Math.abs(y));

function executeInstruction(instruction) {
  let op = instruction.op;
  let arg = instruction.arg;

  switch(op) {
    case 'N':
      y += arg;
      break;
    case 'S':
      y -= arg;
      break;
    case 'E':
      x += arg;
      break;
    case 'W':
      x -= arg;
      break;
    case 'L':
      {
        let oldfY = fY;
        fY = fX;
        fX = -oldfY;
      }
      break;
    case 'R':
      {
        let oldfY = fY;
        fY = -fX;
        fX = oldfY;
      }
      break;
    case 'B':
      fX *= -1;
      fY *= -1;
      break;
    case 'F':
      x += fX * arg;
      y += fY * arg;
      break;
  }
}


