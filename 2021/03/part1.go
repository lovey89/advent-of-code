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
	input := readInput()
	totalNumbers := len(input)
	bitsPerRow := len(input[0])

	var oneOccurences = make([]int, bitsPerRow)

	for _, row := range input {
		for j, bit := range row {
			if bit == '1' {
				oneOccurences[j]++
			}
		}
	}
	gammaBits := make([]byte, bitsPerRow)
	epsilonBits := make([]byte, bitsPerRow)
	for i, ones := range oneOccurences {
		if ones > totalNumbers / 2 {
			gammaBits[i] = '1'
			epsilonBits[i] = '0'
		} else {
			gammaBits[i] = '0'
			epsilonBits[i] = '1'
		}
	}

	gamma := string(gammaBits)
	epsilon := string(epsilonBits)

	gammaDec, _ := strconv.ParseInt(gamma, 2, 32)
	epsilonDec, _ := strconv.ParseInt(epsilon, 2, 32)

	fmt.Println(gammaDec * epsilonDec)
}
