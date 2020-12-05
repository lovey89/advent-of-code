let fs = require('fs');

let seatIds =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean) // Removes the trailing empty string
    .map(s => {
      return parseInt(s
        .replace(/F|L/g,'0')
        .replace(/B|R/g,'1'), 2);
    });

let lastSeatId = null;

for (seatId of seatIds.sort()) {
  if (lastSeatId == null) {
    lastSeatId = seatId;
    continue;
  }

  if (seatId - 1 != lastSeatId) {
    console.log(seatId - 1);
    return;
  }

  lastSeatId = seatId;
}
