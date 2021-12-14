// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

type matrix struct {
	energyMatrix [10][10]int
	flashes      int
}

func (m *matrix) step() {
	for y := 0; y < len(m.energyMatrix); y++ {
		for x := 0; x < len(m.energyMatrix[0]); x++ {
			m.energyMatrix[y][x]++
		}
	}
	m.flashStep()
}

func (m *matrix) flashStep() {
	for y := 0; y < len(m.energyMatrix); y++ {
		for x := 0; x < len(m.energyMatrix[0]); x++ {
			m.checkFlash(x, y, false)
		}
	}
}

func (m *matrix) checkFlash(x, y int, inc bool) {
	if x < 0 || y < 0 || y >= len(m.energyMatrix) || x >= len(m.energyMatrix[0]) ||
		m.energyMatrix[y][x] == 0 {
		return
	}

	if inc {
		m.energyMatrix[y][x]++
	}

	if m.energyMatrix[y][x] > 9 {
		m.flashes++
		m.energyMatrix[y][x] = 0
		for i := -1; i <= 1; i++ {
			for j := -1; j <= 1; j++ {
				// There is no harm in checking this coordinate again
				m.checkFlash(x + i, y + j, true)
			}
		}
	}
}

func readInput() matrix {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")

	matrix := matrix{}

	for y, rawLine := range rawLines {
		if rawLine == "" {
			continue
		}

		for x, c := range rawLine {
			i, _ := strconv.Atoi(string(c))
			matrix.energyMatrix[y][x] = i
		}
	}
	return matrix
}

func main() {
	matrix := readInput()

	for i := 0; i < 100; i++ {
		matrix.step()
	}

	fmt.Println(matrix.flashes)
}
