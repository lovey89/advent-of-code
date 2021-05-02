"use strict"

class ImageTile {
  constructor(tileInfo) {
    let splittedTileInfo = tileInfo
      .split("\n")
      .filter(Boolean);
    this.tileId = Number(splittedTileInfo[0].slice(5,-1));
    this.matrix = splittedTileInfo
      .slice(1)
      .map(line => line.split(""));
    [this.top]
  }

  _getRowValues(rowIndex) {
    let array = Array.from(this.matrix[rowIndex]);
    return _toBinaryArray(array);
  }

  _getColumnValues(columnIndex) {
    let array = this.matrix
      .map(row => row[columnIndex]);
    return _toBinaryArray(array);
  }

  _toBinaryArray(array) {
    let binaryArray = array
      .map(c => {
        if (c == '.') {
          return '0';
        } else {
          return '1';
        }
      });
    let value = parseInt(binaryArray.join(""), 2);
    let reverseValue = parseInt(binaryArray.reverse().join(""), 2);
    return [value, reverseValue];
    /*
    console.log(array);
    console.log(binaryArray.join(""));
    console.log(parseInt(binaryArray.join(""), 2));
    console.log(binaryArray.reverse().join(""));
    console.log(parseInt(binaryArray.join(""), 2));
    console.log(array);*/
  }
}

let fs = require('fs');



toBinaryArray(['#', '.', '#', '#', '#', '#', '#', '#', '#', '#']);

let input =
  fs.readFileSync('input.txt').toString()
    .split("\n\n")
    .filter(Boolean);

console.log(input.length);

let tileMap = new Map();

for (let tileInfo of input) {
  let splittedTileInfo = tileInfo
    .split("\n")
    .filter(Boolean);
  let tileId = Number(splittedTileInfo[0].slice(5,-1));
  let matrix = splittedTileInfo
    .slice(1)
    .map(line => line.split(""));

  //let top = 
  
  tileMap.set(tileId, matrix);
}

function getRowValues(matrix, rowIndex) {
  let array = Array.from(matrix[rowIndex]);
}

function getColumnValues(matrix, columnIndex) {
  return matrix
    .map(row => row[columnIndex]);
}

function calulateValues(array) {
  
}


/*
let a = [1,2,3];

console.log(Array.from(a).reverse());
console.log(a);
console.log(a[0], a[1], a[2]);
console.log("Tile 1787:".slice(5,-1));
console.log(tileMap);
*/
