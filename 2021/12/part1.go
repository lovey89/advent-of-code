// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

type room struct {
	large    bool
	visited  bool
	paths    []string
}

type cave map[string]*room

func (c cave) findNumberOfPaths(start string, end string) int {
	if start == end {
		return 1
	}
	room := c[start]
	if room.visited {
		return 0
	}
	if !room.large {
		room.visited = true
	}
	pathsToEnd := 0
	for _, path := range room.paths {
		pathsToEnd += c.findNumberOfPaths(path, end)
	}
	room.visited = false
	return pathsToEnd
}

func readInput() cave {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")

	cave := cave{}

	for _, rawLine := range rawLines {
		if rawLine == "" {
			continue
		}

		fromToSplit := strings.Split(rawLine, "-")
		from := fromToSplit[0]
		to := fromToSplit[1]

		fromRoom, fromExists := cave[from]
		toRoom, toExists := cave[to]
		if !fromExists {
			fromRoom = &room{unicode.IsUpper([]rune(from)[0]), false, []string{}}
			cave[from] = fromRoom
		}
		if !toExists {
			toRoom = &room{unicode.IsUpper([]rune(to)[0]), false, []string{}}
			cave[to] = toRoom
		}
		fromRoom.paths = append(fromRoom.paths, to)
		toRoom.paths = append(toRoom.paths, from)
	}
	return cave
}

func main() {
	cave := readInput()
	fmt.Println(cave.findNumberOfPaths("start", "end"))
}
