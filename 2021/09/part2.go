// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
	"sort"
)

var basinIdCtr int = 0
var basins []*basin

type coord struct {
	height    int
	subBasin  *subBasin
}

type subBasin struct {
	basin  *basin
}

type basin struct {
	sum        int
	subBasins  []*subBasin
	id         int // to print the heightMap
}

type heightMap struct {
	heightMatrix  [][]coord
	rows          int
	cols          int
}

func (b0 *basin) mergeWith(b1 *basin) {
	b0.sum += b1.sum
	for _, subBasin := range b1.subBasins {
		subBasin.basin = b0
	}
	b0.subBasins = append(b0.subBasins, b1.subBasins...)
	b1.sum = 0
	b1.subBasins = []*subBasin{} // Should not be needed
}

func createNewSubBasin() *subBasin {
	basinId := basinIdCtr
	basinIdCtr++
	sb := &subBasin{nil}
	basins = append(basins, &basin{0, []*subBasin{sb}, basinId})
	sb.basin = basins[len(basins) - 1]
	return sb
}

func (hm *heightMap) updateBasin(x, y int) {
	if hm.heightMatrix[y][x].height == 9 {
		return
	}

	if x != 0 && hm.heightMatrix[y][x - 1].height != 9 {
		hm.heightMatrix[y][x].subBasin = hm.heightMatrix[y][x - 1].subBasin
		hm.heightMatrix[y][x].subBasin.basin.sum++
	} else if y != 0 && hm.heightMatrix[y - 1][x].height != 9 {
		hm.heightMatrix[y][x].subBasin = hm.heightMatrix[y - 1][x].subBasin
		hm.heightMatrix[y][x].subBasin.basin.sum++
		return
	} else {
		newSubBasin := createNewSubBasin()
		hm.heightMatrix[y][x].subBasin = newSubBasin
		hm.heightMatrix[y][x].subBasin.basin.sum++
		return
	}

	if y == 0 {
		return
	}
	// We only reached here if we copied the value to the left and we're not on the first row
	// Check if we must merge basin on the left and top
	if hm.heightMatrix[y - 1][x].height == 9 {
		return
	}

	if hm.heightMatrix[y][x].subBasin.basin.id != hm.heightMatrix[y - 1][x].subBasin.basin.id {
		hm.heightMatrix[y][x].subBasin.basin.mergeWith(hm.heightMatrix[y - 1][x].subBasin.basin)
	}
	return
}

func (hm *heightMap) calculateBasins() {
	for y := 0; y < hm.rows; y++ {
		for x := 0; x < hm.cols; x++ {
			hm.updateBasin(x, y)
		}
	}
}

func readInput() heightMap {
	dat, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(dat), "\n")

	matrix := make([][]coord, 0)

	for _, line := range lines {
		if line == "" {
			continue
		}
		row := make([]coord, 0)

		for _, stringElem := range line {
			row = append(row, coord{int(stringElem - '0'), nil})
		}
		matrix = append(matrix, row)
	}
	return heightMap{matrix, len(matrix), len(matrix[0])}
}

func main() {
	heightMap := readInput()
	heightMap.calculateBasins()

	basinSizes := make([]int, len(basins))
	for i, basin := range basins {
		basinSizes[i] = basin.sum
	}
	// Sorted in ascending order
	sort.Ints(basinSizes)
	mul := 1

	for _, basinSize := range basinSizes[len(basinSizes) - 3 :] {
		mul *= basinSize
	}

	fmt.Println(mul)
}
