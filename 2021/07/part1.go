// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func readInput() ([]int, int, int) {
	dat, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(dat), "\n")
	// Second line is blank so ignore it
	stringNumbers := strings.Split(lines[0], ",")

	var positions []int

	min := 0
	max := 0

	for _, stringNumber := range stringNumbers {
		position, _ := strconv.Atoi(stringNumber)
		positions = append(positions, position)
		if position < min {
			min = position
		} else if position > max {
			max = position
		}
	}

	return positions, min, max
}

// Since there is no 'Abs' function in the standard library
func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	positions, min, max := readInput()

	minFuel := -1
	for i := min; i <= max; i++ {
		fuel := 0
		for _, position := range positions {
			fuel += Abs(position - i)
		}
		if minFuel == -1 || fuel < minFuel {
			minFuel = fuel
		}
	}
	fmt.Println(minFuel)
}
