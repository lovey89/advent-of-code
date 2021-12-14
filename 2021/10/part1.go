// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
)

type stack []byte

func (s *stack) isEmpty() bool {
	return len(*s) == 0
}

func (s *stack) push(b byte) {
	*s = append(*s, b)
}

func (s *stack) pop() (byte, bool) {
	if s.isEmpty() {
		return 0, false
	} else {
		index := len(*s) - 1
		element := (*s)[index]
		*s = (*s)[:index]
		return element, true
	}
}

func readInput() [][]byte {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")

	lines := make([][]byte, 0)

	for _, rawLine := range rawLines {
		if rawLine == "" {
			continue
		}

		lines = append(lines, []byte(rawLine))
	}
	return lines
}

func checkCorruptedValue(line []byte) int {
	s := stack{}
	for _, b := range line {
		switch b {
		case '(', '[', '{', '<':
			s.push(b)
		case ')':
			f, found := s.pop()
			if !found || f != '(' {
				return 3
			}
		case ']':
			f, found := s.pop()
			if !found || f != '[' {
				return 57
			}
		case '}':
			f, found := s.pop()
			if !found || f != '{' {
				return 1197
			}
		case '>':
			f, found := s.pop()
			if !found || f != '<' {
				return 25137
			}
		}
	}
	return 0
}

func main() {
	lines := readInput()

	corruptedSum := 0
	for _, line := range lines {
		corruptedSum += checkCorruptedValue(line)
	}
	fmt.Println(corruptedSum)
}
