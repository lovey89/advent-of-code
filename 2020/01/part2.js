let fs = require('fs');

let array = fs.readFileSync('input.txt').toString()
              .split("\n")
              .filter(Boolean) // Removes the trailing empty string
              .map(Number);

arraylength = array.length;

for (let i = 0; i < arraylength; i++) {
  for (let j = i + 1; j < arraylength; j++) {
    for (let k = j + 1; k < arraylength; k++) {
      let val0 = array[i];
      let val1 = array[j];
      let val2 = array[k];
      if (val0 + val1 + val2 == 2020) {
        console.log(val0 * val1 * val2);
        return;
      }
    }
  }
}
