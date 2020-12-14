let fs = require('fs');

let seatMatrix =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(row => Array.from(row));

let updated;

do {
  let {nextGenSeatMatrix, wasUpdated} = nextGen(seatMatrix);
  updated = wasUpdated;
  seatMatrix = nextGenSeatMatrix;
} while(updated);


console.log(findOccupied(seatMatrix));

function findOccupied(seatMatrix) {

 let occupied = 0;
  for (let y = 0; y < seatMatrix.length; y++) {
    let seatRow = seatMatrix[y];
    for (let x = 0; x < seatRow.length; x++) {
      let seat = seatRow[x];
      if (seat == '#') {
        occupied++;
      }
    }
  }
  return occupied;
}

function nextGen(seatMatrix) {

  let nextGenSeatMatrix = Array(seatMatrix.length).fill().map(() => Array(seatMatrix[0].length).fill());
  let wasUpdated = false;
  
  for (let y = 0; y < seatMatrix.length; y++) {
    //console.log(y);
    let seatRow = seatMatrix[y];
    for (let x = 0; x < seatRow.length; x++) {
      let seat = seatRow[x];
      if (seat == 'L' && howManyAdjacentSeatsAreOccupied(seatMatrix, x, y) == 0) {
        nextGenSeatMatrix[y][x] = '#';
        wasUpdated = true;
      } else if (seat == '#' && howManyAdjacentSeatsAreOccupied(seatMatrix, x, y) >= 4) {
        nextGenSeatMatrix[y][x] = 'L';
        wasUpdated = true;
      } else {
        nextGenSeatMatrix[y][x] = seat;
      }
    }
  }

  return {nextGenSeatMatrix, wasUpdated};
  
  function howManyAdjacentSeatsAreOccupied(seatMatrix, x, y) {
    let occupiedSeats = 0;

    if (y != 0) {
      let seatRow = seatMatrix[y - 1];
      for (let i = Math.max(x - 1, 0); i <= Math.min(x + 1, seatRow.length); i++) {
        let seat = seatRow[i];
        if (seat == '#') {
          occupiedSeats++;
        }
      }
    }

    let currentRow = seatMatrix[y];
    if (x != 0) {
      let seat = currentRow[x - 1];
      if (seat == '#') {
        occupiedSeats++;
      }
    }

    if (x != currentRow.length - 1) {
      let seat = currentRow[x + 1];
      if (seat == '#') {
        occupiedSeats++;
      }
    }
    

    if (y != seatMatrix.length - 1) {
      let seatRow = seatMatrix[y + 1];
      for (let i = Math.max(x - 1, 0); i <= Math.min(x + 1, seatRow.length); i++) {
        let seat = seatRow[i];
        if (seat == '#') {
          occupiedSeats++;
        }
      }
    }
    return occupiedSeats;
  }
}
