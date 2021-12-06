// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func readInput() []int {
	dat, _ := os.ReadFile("./input.txt")
	sliceData := strings.Split(string(dat), "\n")

	ints := make([]int, 0)
	for _, elem := range sliceData {
		convInt, err := strconv.Atoi(elem)
		if err == nil {
			ints = append(ints, convInt)
		}
	}
	return ints
}

func main() {
	ints := readInput()
	depthIncrease := 0
	for i := 0; i < len(ints) - 3; i++ {
		// ints[i + 1] and ints[i + 2] are equal on both sides and
		// cancels each other out
		if ints[i] < ints[i + 3] {
			depthIncrease++
		}
	}
	fmt.Println(depthIncrease)
}
