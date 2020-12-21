"use strict"

let fs = require('fs');

let ingredientsArray =
  fs.readFileSync('input.txt').toString()
    .split("\n")
    .filter(Boolean)
    .map(line => {
      let splittedLine = line.split(" (")
      let ingredients = new Set(splittedLine[0].split(" "));
      let allergenes = new Set(splittedLine[1].slice(9, -1).split(", "));
      return {ingredients, allergenes};
    });

let allAllergenes = ingredientsArray
  .map(o => o.allergenes)
  .reduce((a, b) => union(a, b), new Set());

let candidateMap = new Map();
for (let allergene of allAllergenes) {
  let candidates = ingredientsArray
    .filter(ingredientsInfo => ingredientsInfo.allergenes.has(allergene))
    .map(ingredientsInfo => ingredientsInfo.ingredients)
    .reduce((a, b) => intersection(a, b));
  candidateMap.set(allergene, candidates);
}

let knownIngredients = new Set();
for (let i = 0; i < allAllergenes.size; i++) {
  let [allergene, candidates] = findAllergeneWithSingleCandidate(candidateMap);
  let ingredient = [...candidates][0];
  knownIngredients.add(ingredient);
  candidateMap.delete(allergene);
  deleteIngredientAsAlternative(candidateMap, ingredient);
}

let sum = ingredientsArray
  .map(ingredientsInfo => {
    return difference(ingredientsInfo.ingredients, knownIngredients).size
  })
  .reduce((a, b) => a + b);

console.log(sum);

function findAllergeneWithSingleCandidate(candidateMap) {
  for (const [allergene, candidates] of candidateMap.entries()) {
    if (candidates.size == 1) {
      return [allergene, candidates];
    }
  }
  return null;
}

function deleteIngredientAsAlternative(candidateMap, ingredient) {
  candidateMap.forEach(candidates => candidates.delete(ingredient));
}

function difference(setA, setB) {
    let _difference = new Set(setA)
    for (let elem of setB) {
        _difference.delete(elem);
    }
    return _difference;
}

function union(setA, setB) {
    let _union = new Set(setA)
    for (let elem of setB) {
        _union.add(elem);
    }
    return _union;
}

function intersection(setA, setB) {
  let _intersection = new Set()
  for (let elem of setB) {
    if (setA.has(elem)) {
      _intersection.add(elem);
    }
  }
  return _intersection;
}
