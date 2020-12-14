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

let x = 0;
let y = 0;
let wX = 10;
let wY = 1;

for (let instruction of instructions) {
  executeInstruction(instruction);
  //console.log(instruction, 'x:', x, 'y:', y, 'wX:', wX, 'wY:', wY);
}

console.log(Math.abs(x) + Math.abs(y));

function executeInstruction(instruction) {
  let op = instruction.op;
  let arg = instruction.arg;

  switch(op) {
    case 'N':
      wY += arg;
      break;
    case 'S':
      wY -= arg;
      break;
    case 'E':
      wX += arg;
      break;
    case 'W':
      wX -= arg;
      break;
    case 'L':
      {
        let oldwX = wX;
        wX = -wY;
        wY = oldwX;
      }
      break;
    case 'R':
      {
        let oldwX = wX;
        wX = wY;
        wY = -oldwX;
      }
      break;
    case 'B':
      wX *= -1;
      wY *= -1;
      break;
    case 'F':
      x += wX * arg;
      y += wY * arg;
      break;
  }
}


