// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func readInput() []string {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")

	instructions := make([]string, 0)
	for _, elem := range rawLines {
		if elem != "" {
			instructions = append(instructions, elem)
		}
	}
	return instructions
}

func main() {
	instructions := readInput()
	horizontal := 0
	depth := 0

	for _, instruction := range instructions {
		if strings.HasPrefix(instruction, "forward ") {
			convInt, _ := strconv.Atoi(instruction[8:])
			horizontal += convInt
		} else if strings.HasPrefix(instruction, "up ") {
			convInt, _ := strconv.Atoi(instruction[3:])
			depth -= convInt
		} else { // Down
			convInt, _ := strconv.Atoi(instruction[5:])
			depth += convInt
		}
	}
	fmt.Println(horizontal * depth)
}
