// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func readInput() ([9]int) {
	dat, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(dat), "\n")
	// Second line is blank so ignore it
	stringNumbers := strings.Split(lines[0], ",")

	initState := [9]int{}

	for _, stringNumber := range stringNumbers {
		number, _ := strconv.Atoi(stringNumber)
		initState[number]++
	}

	return initState
}

func stepOneGeneration(gen *[9]int) {
	parents := gen[0]

	for i := 0; i < 8 ; i++ {
		gen[i] = gen[i + 1]
	}

	gen[6] += parents
	gen[8] = parents
}

func main() {
	initState := readInput()
	fmt.Println(initState)
	for i := 0; i < 80; i++ {
		stepOneGeneration(&initState)
		fmt.Println(i+1, initState)
	}

	var fishSum int
	for _, elem := range initState {
		fishSum += elem
	}
	fmt.Println(initState)
	fmt.Println(fishSum)
}
