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

    let seatRow = seatMatrix[y];
    for (let x = 0; x < seatRow.length; x++) {
      let seat = seatRow[x];
      if (seat == 'L' && howManyAdjacentSeatsAreOccupied(seatMatrix, x, y) == 0) {
        nextGenSeatMatrix[y][x] = '#';
        wasUpdated = true;
      } else if (seat == '#' && howManyAdjacentSeatsAreOccupied(seatMatrix, x, y) >= 5) {
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

    for (let y_dir = -1; y_dir <= 1; y_dir++) {
      for (let x_dir = -1; x_dir <= 1; x_dir++) {
        if (y_dir == 0 && x_dir == 0) {
          continue;
        }
        if (occupiedInDirection(x, y, x_dir, y_dir)) {
          occupiedSeats++;
        }
      }
    }

    return occupiedSeats;
  }

  function occupiedInDirection(x, y, xDir, yDir) {
    for (let cx = x + xDir, cy = y + yDir;
      cx >= 0 && cy >= 0 && cy < seatMatrix.length && cx < seatMatrix[0].length;
      cx += xDir, cy += yDir) {
      seat = seatMatrix[cy][cx];
      if (seat == '#') {
        return true;
      } else if (seat == 'L') {
        return false;
      }
    }
    return false;
  }
}
