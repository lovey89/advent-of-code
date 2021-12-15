// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

type room struct {
	large   bool
	visited bool
	paths   []string
}

type cave struct {
	caveMap           map[string]*room
	visitedSmallTwice bool
}

func (c *cave) findNumberOfPaths(start string, end string) int {
	if start == end {
		return 1
	}

	room := c.caveMap[start]

	if room.visited {
		if !c.visitedSmallTwice && start != "start" {
			c.visitedSmallTwice = true
			pathsToEnd := 0
			for _, path := range room.paths {
				pathsToEnd += c.findNumberOfPaths(path, end)
			}
			c.visitedSmallTwice = false
			return pathsToEnd
		} else {
			return 0
		}
	}

	pathsToEnd := 0
	if room.large {
		for _, path := range room.paths {
			pathsToEnd += c.findNumberOfPaths(path, end)
		}
	} else {
		room.visited = true
		for _, path := range room.paths {
			pathsToEnd += c.findNumberOfPaths(path, end)
		}
		room.visited = false
	}
	return pathsToEnd
}

func readInput() cave {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")

	cave := cave{make(map[string]*room), false}

	for _, rawLine := range rawLines {
		if rawLine == "" {
			continue
		}

		fromToSplit := strings.Split(rawLine, "-")
		from := fromToSplit[0]
		to := fromToSplit[1]

		fromRoom, fromExists := cave.caveMap[from]
		toRoom, toExists := cave.caveMap[to]
		if !fromExists {
			fromRoom = &room{unicode.IsUpper([]rune(from)[0]), false, []string{}}
			cave.caveMap[from] = fromRoom
		}
		if !toExists {
			toRoom = &room{unicode.IsUpper([]rune(to)[0]), false, []string{}}
			cave.caveMap[to] = toRoom
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
