let fs = require('fs');

let array = fs.readFileSync('input.txt').toString()
              .split("\n")
              .filter(Boolean) // Removes the trailing empty string
              .map(Number);

arraylength = array.length;

for (let i = 0; i < arraylength; i++) {
  for (let j = i + 1; j < arraylength; j++) {
    let val0 = array[i];
    let val1 = array[j];
    if (val0 + val1 == 2020) {
      console.log(val0 * val1);
      return;
    }
  }
}
