"use strict"

class CircularArray {
  constructor(size) {
    this.cArray = new Array(size);
    this.head = 0;
    this.nextEmpty = 0;
    this.isFull = false;
  }

  pop() {
    if (this.head == this.nextEmpty && !this.isFull) {
      return undefined;
    }
    let element = this.cArray[this.head++];
    this.head %= this.cArray.length;
    this.isFull = false;
    return element;
  }

  pushToBack(element) {
    if (this.isFull) {
      return false;
    }

    this.cArray[this.nextEmpty++] = element;
    this.nextEmpty %= this.cArray.length;
    if (this.head == this.nextEmpty) {
      this.isFull = true;
    }
    return true;
  }

  createSubCopyOfSize(size) {
    let nArray = new CircularArray(this.cArray.length);
    for (let i = 0; i < size; i++) {
      nArray.pushToBack(this.cArray[(this.head + i) % this.cArray.length])
    }
    return nArray;
  }

  get size() {
    if (this.isFull) {
      return this.cArray.length;
    }
    return ((this.nextEmpty - this.head) + this.cArray.length) % this.cArray.length;
  }

  isEmpty() {
    return this.head == this.nextEmpty && !this.isFull;
  }

  toString() {
    if (this.head == this.nextEmpty && !this.isFull) {
      return "[ ]"
    }
    let tmpArray = [this.cArray[this.head]];
    let i = (this.head + 1) % this.cArray.length;
    while (i != this.nextEmpty) {
      tmpArray.push(this.cArray[i++]);
      i %= this.cArray.length;
    }
    return "[ " + tmpArray.join(", ") + " ]";
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

let {deck: winningPlayer} = recursiveCombat(player0, player1, 1);

let score = 0;
for (let i = winningPlayer.size; i > 0 ; i--) {
  score += winningPlayer.pop() * i;
}
console.log(score);

function recursiveCombat(deck0, deck1) {
  let seen = new Set();

  while (!(deck0.isEmpty() || deck1.isEmpty())) {
    let decksString = deck0.toString() + deck1.toString();

    if (seen.has(decksString)) {
      return {player: 0, deck: deck0};
    }
    seen.add(decksString);

    let card0 = deck0.pop();
    let card1 = deck1.pop();

    let winner;
    if (deck0.size >= card0 && deck1.size >= card1) {
      ({player: winner} = recursiveCombat(deck0.createSubCopyOfSize(card0), deck1.createSubCopyOfSize(card1)));
    } else {
      winner = card0 > card1 ? 0 : 1;
    }
    if (winner == 0) {
      deck0.pushToBack(card0);
      deck0.pushToBack(card1);
    } else {
      deck1.pushToBack(card1);
      deck1.pushToBack(card0);
    }
  }
  return deck0.isEmpty() ? {player: 1, deck: deck1} : {player: 0, deck: deck0};
}
