let fs = require('fs');

let rawPassports =
  fs.readFileSync('input.txt').toString()
    .split("\n\n")
    .map(s => s.split(/ |\n/));

let passports =
  rawPassports
    .map(rawPassport => {
      let x = rawPassport
        .map(entry => entry.split(":"))
      return Object.fromEntries(x);
    });

let validPassports =
  passports
    .filter(p => {
      let numberOfFields = Object.keys(p).length;
      return numberOfFields == 8 || (numberOfFields == 7 && !("cid" in p));
    });

console.log(validPassports.length);
