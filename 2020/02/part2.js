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

let validPasswords = array.filter(isValidPassword).length

console.log(validPasswords)

function isValidPassword(passwordObj) {
  let firstLetter = passwordObj.password.charAt(passwordObj.firstNumber - 1);
  let secondLetter = passwordObj.password.charAt(passwordObj.secondNumber - 1);
  let specialLetter = passwordObj.letter;

  return (firstLetter == specialLetter) != (secondLetter == specialLetter);
}
