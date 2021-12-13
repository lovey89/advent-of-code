// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
)

type heightMap struct {
	heightMatrix  [][]int
	rows          int
	cols          int
}

func (hm *heightMap) isLowPoint(x, y int) bool {
	currentPoint := hm.heightMatrix[y][x]
	if y != 0 && hm.heightMatrix[y - 1][x] <= currentPoint {
		return false
	}
	if y < hm.rows - 1 && hm.heightMatrix[y + 1][x] <= currentPoint {
		return false
	}
	if x != 0 && hm.heightMatrix[y][x - 1] <= currentPoint {
		return false
	}
	if x < hm.cols - 1 && hm.heightMatrix[y][x + 1] <= currentPoint {
		return false
	}
	return true
}

func (hm *heightMap) calculateRiskLevel() int {
	riskLevel := 0
	for y := 0; y < hm.rows; y++ {
		for x := 0; x < hm.cols; x++ {
			if hm.isLowPoint(x, y) {
				riskLevel += hm.heightMatrix[y][x] + 1
			}
		}
	}
	return riskLevel
}

func readInput() heightMap {
	dat, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(dat), "\n")

	matrix := make([][]int, 0)

	for _, line := range lines {
		if line == "" {
			continue
		}
		row := make([]int, 0)

		for _, stringElem := range line {
			row = append(row, int(stringElem - '0'))
		}
		matrix = append(matrix, row)
	}
	return heightMap{matrix, len(matrix), len(matrix[0])}
}

func main() {
	heightMap := readInput()
	fmt.Println(heightMap.calculateRiskLevel())
}
