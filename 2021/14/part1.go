// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
)

type manual struct {
	template        string
	instructionMap  map[string]string
}

func (m *manual) step() {
	var b strings.Builder
	fmt.Fprintf(&b, "%c", m.template[0])
	for i := 0; i < len(m.template) - 1; i++ {
		instruction := m.instructionMap[m.template[i:i+2]]
		fmt.Fprintf(&b, "%s%c", instruction, m.template[i+1])
	}
	m.template = b.String()
}

func (m *manual) countChars() (min, max int) {
	charMap := make(map[rune]int)
	for _, char := range m.template {
		charMap[char]++
	}

	for _, v := range charMap {
		if v > max {
			max = v
		}
		if (min == 0 || v < min) {
			min = v
		}
	}

	return
}

func readInput() manual {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n\n")

	origString := rawLines[0]

	rawInstructions := rawLines[1]

	instructionMap := make(map[string]string)

	for _, rawInstruction := range strings.Split(rawInstructions, "\n") {
		if rawInstruction == "" {
			continue
		}

		instructionSplit := strings.Split(rawInstruction, " -> ")
		fromInstruction := instructionSplit[0]
		toInstruction := instructionSplit[1]
		instructionMap[fromInstruction] = toInstruction
	}

	return manual{origString, instructionMap}
}

func main() {
	manual := readInput()
	for i := 0; i < 10; i++ {
		manual.step()
	}
	min, max := manual.countChars()
	fmt.Println(max - min)
}
