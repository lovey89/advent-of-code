// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
)

type manual struct {
	pairs           map[string]int
	occurences      map[rune]int
	instructionMap  map[string]rune
}

func (m *manual) step() {
	nextPairs := make(map[string]int)

	for pair, occurenceOfPair := range m.pairs {
		middleChar := m.instructionMap[pair]
		m.occurences[middleChar] += occurenceOfPair
		runeSlice := []rune(pair)
		nextPairs[string([]rune{runeSlice[0], middleChar})] += occurenceOfPair
		nextPairs[string([]rune{middleChar, runeSlice[1]})] += occurenceOfPair
	}
	m.pairs = nextPairs
}

func (m *manual) findMinMaxOccurences() (min, max int) {
	for _, occurences := range m.occurences {
		if occurences > max {
			max = occurences
		}
		if (min == 0 || occurences < min) {
			min = occurences
		}
	}
	return
}

func readInput() manual {
	m := manual{make(map[string]int), make(map[rune]int), make(map[string]rune)}

	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n\n")

	origString := rawLines[0]

	for _, c := range origString {
		m.occurences[c]++
	}

	for i := 0; i < len(origString) - 1; i++ {
		m.pairs[origString[i:i+2]]++
	}

	rawInstructions := rawLines[1]

	for _, rawInstruction := range strings.Split(rawInstructions, "\n") {
		if rawInstruction == "" {
			continue
		}

		instructionSplit := strings.Split(rawInstruction, " -> ")
		fromInstruction := instructionSplit[0]
		toInstruction := instructionSplit[1]
		m.instructionMap[fromInstruction] = []rune(toInstruction)[0]
	}

	return m
}

func main() {
	manual := readInput()

	for i := 0; i < 40; i++ {
		manual.step()
	}

	min, max := manual.findMinMaxOccurences()
	fmt.Println(max - min)
}
