let fs = require('fs');

let regex = /(\d+)-(\d+) (.): (.+)/;

let array = fs.readFileSync('input.txt').toString()
              .split("\n")
              .filter(Boolean) // Removes the trailing empty string
              .map(row => {
                let result = row.match(regex);
                let passwordObj = {
                  firstNumber: Number(result[1]),
                  secondNumber: Number(result[2]),
                  letter: result[3],
                  password: result[4]
                };
                //console.log(passwordObj);
                return passwordObj;
              });

let validPasswords = 0;

// Part 2 is done in a slightly different way which is better but I keep this anyway
for (const passwordObj of array) {
  if (isValidPassword(passwordObj)) {
    validPasswords++;
  }
}
console.log(validPasswords)

function isValidPassword(passwordObj) {
  let occurences = (passwordObj.password.match(new RegExp(passwordObj.letter, "g")) || []).length;

  return occurences >= passwordObj.firstNumber && occurences <= passwordObj.secondNumber;
}
