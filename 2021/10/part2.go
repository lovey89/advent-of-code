// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
	"sort"
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

func checkAutocompleteValue(line []byte) int {
	s := stack{}
	for _, b := range line {
		if b == '(' || b == '[' || b == '{' || b == '<' {
			s.push(b)
			continue
		}
		f, found := s.pop()
		if !found || (f == '(' && b != ')') || f != '(' && b != f + 2 {
			return 0
		}
	}
	score := 0
	for {
		f, found := s.pop()
		if !found {
			break
		}
		score *= 5
		switch f {
		case '(':
			score += 1
		case '[':
			score += 2
		case '{':
			score += 3
		case '<':
			score += 4
		}
	}
	return score
}

func main() {
	lines := readInput()

	scores := make([]int, 0)
	for _, line := range lines {
		autocompleteValue := checkAutocompleteValue(line)
		if autocompleteValue != 0 {
			scores = append(scores, autocompleteValue)
		}
	}
	sort.Ints(scores)
	fmt.Println(scores[len(scores) / 2])
}
