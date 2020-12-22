"use strict"

class CircularArray {
  constructor(size) {
    this.cArray = new Array(size);
    this.head = 0;
    this.nextEmpty = 0;
    this.size = size;
    this.isFull = false;
  }

  pop() {
    if (this.head == this.nextEmpty && !this.isFull) {
      return undefined;
    }
    let element = this.cArray[this.head++];
    this.head %= this.size;
    this.isFull = false;
    return element;
  }

  pushToBack(element) {
    if (this.isFull) {
      return false;
    }

    this.cArray[this.nextEmpty++] = element;
    this.nextEmpty %= this.size;
    if (this.head == this.nextEmpty) {
      this.isFull = true;
    }
    return true;
  }

  isEmpty() {
    return this.head == this.nextEmpty && !this.isFull;
  }
}

let fs = require('fs');

let decks =
  fs.readFileSync('input.txt').toString()
    .split("\n\n")
    .filter(Boolean)
    .map(playerInfo => {
      let cards = playerInfo
        .split("\n")
        .filter(Boolean)
        .slice(1)
        .map(Number);

      let playerDeck = new CircularArray(cards.length * 2);
      for (const card of cards) {
        playerDeck.pushToBack(card);
      }
      return playerDeck;
    });

let player0 = decks[0];
let player1 = decks[1];

while (!(player0.isEmpty() || player1.isEmpty())) {
  let card0 = player0.pop();
  let card1 = player1.pop();

  if (card0 > card1) {
    player0.pushToBack(card0);
    player0.pushToBack(card1);
  } else {
    player1.pushToBack(card1);
    player1.pushToBack(card0);
  }
}

let winningPlayer = player0.isEmpty() ? player1 : player0;

let score = 0;
for (let i = player0.size; i > 0 ; i--) {
  score += winningPlayer.pop() * i;
}
console.log(score);
