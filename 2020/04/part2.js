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
    .filter(isValidPassport);

function isValidPassport(passport) {
  let numberOfFields = Object.keys(passport).length;
  if (!(numberOfFields == 8 || (numberOfFields == 7 && !("cid" in passport)))) {
    return false;
  }

  let byr = Number(passport.byr);
  if (byr < 1920 || byr > 2002) {
    return false;
  }

  let iyr = Number(passport.iyr);
  if (iyr < 2010 || iyr > 2020) {
    return false;
  }

  let eyr = Number(passport.eyr);
  if (eyr < 2020 || eyr > 2030) {
    return false;
  }

  let hgtMatchResult = passport.hgt.match(/^(\d+)(cm|in)$/);
  if (hgtMatchResult) {
    let height = Number(hgtMatchResult[1]);
    let unit = hgtMatchResult[2];
    if (unit === 'cm' && (height < 150 || height > 193)) {
      return false;
    } else if (unit === 'in' && (height < 59 || height > 76)) {
      return false;
    }
  } else {
    return false;
  }

  if (!/^#[0-9a-fA-F]{6}$/.test(passport.hcl)) {
    return false;
  }

  if (!/^(amb|blu|brn|gry|grn|hzl|oth)$/.test(passport.ecl)) {
    return false;
  }

  if (!/^\d{9}$/.test(passport.pid)) {
    return false;
  }

  return true;
}

console.log(validPassports.length);
