// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

type vector struct {
	x0, y0, x1, y1 int
}

func readInput() ([]vector) {
	dat, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(dat), "\n")

	vectors := make([]vector, 0)

	for _, line := range lines {
		if line == "" {
			continue
		}
		coordSplit := strings.Split(line, " -> ")
		coordSplit0 := strings.Split(coordSplit[0], ",")
		coordSplit1 := strings.Split(coordSplit[1], ",")
		x0, _ := strconv.Atoi(coordSplit0[0])
		y0, _ := strconv.Atoi(coordSplit0[1])
		x1, _ := strconv.Atoi(coordSplit1[0])
		y1, _ := strconv.Atoi(coordSplit1[1])

		if x0 == x1 || y0 == y1 {
			vectors = append(vectors, vector{x0, y0, x1, y1})
		}
	}

	return vectors
}

func minMax(x, y int) (int, int) {
    if x < y {
        return x, y
    }
    return y, x
}

func main() {
	vectors := readInput()

	overlaps := 0
	coords := [1000][1000]int{}

	for _, v := range vectors {
		if v.x0 == v.x1 {
			start, end := minMax(v.y0, v.y1)
			for i := start; i <= end; i++ {
				if coords[v.x0][i] == 1 {
					overlaps++
				}
				coords[v.x0][i]++
			}
		} else {
			start, end := minMax(v.x0, v.x1)
			for i := start; i <= end; i++ {
				if coords[i][v.y0] == 1 {
					overlaps++
				}
				coords[i][v.y0]++
			}
		}
	}
	fmt.Println(overlaps)
}
