// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

type coord struct {
	x, y int
}

type paper [][]bool

func (pp *paper) fold(foldHorizontally bool) {
	p := *pp
	if foldHorizontally {
		height := len(p)
		width := len(p[0])
		for y := 0; y < height / 2; y++ {
			for x := 0; x < width; x++ {
				p[y][x] = p[y][x] || p[height - 1 - y][x]
			}
		}
		*pp = p[0:height/2]
	} else {
		width := len(p[0])
		for y, row := range p {
			for i := 0; i < width / 2; i++ {
				row[i] = row[i] || row[width - 1 - i]
			}
			p[y] = p[y][0:width/2]
		}
	}
}

func (pp *paper) countDots() int {
	count := 0
	for _, row := range *pp {
		for _, coord := range row {
			if coord {
				count++
			}
		}
	}
	return count
}

func (pp *paper) print() {
	for _, row := range *pp {
		for _, coord := range row {
			if coord {
				fmt.Print("#")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

func readInput() (paper, []bool) {
	dat, _ := os.ReadFile("./input.txt")
	rawSplit := strings.Split(string(dat), "\n\n")

	dotsData := rawSplit[0]
	foldInstructionsData := rawSplit[1]


	foldHorizontally := make([]bool, 0)
	xMaxFold := 0
	yMaxFold := 0
	for _, foldInstruction := range strings.Split(foldInstructionsData, "\n") {
		if foldInstruction == "" {
			continue
		}
		foldString := foldInstruction[13:]
		fold, _ := strconv.Atoi(foldString)
		horizontalFold := foldInstruction[11] == 'y'

		if horizontalFold && fold > yMaxFold {
			yMaxFold = fold
		} else if !horizontalFold && fold > xMaxFold {
			xMaxFold = fold
		}
		foldHorizontally = append(foldHorizontally, horizontalFold)
	}

	// Calculating max x and y this way is better than looking at the coordinates
	// since the last couples of rows or columns may be empty
	xMax := xMaxFold * 2 + 1
	yMax := yMaxFold * 2 + 1
	dots := make([]coord, 0)
	for _, rawDot := range strings.Split(dotsData, "\n") {
		dotSplit := strings.Split(rawDot, ",")
		x, _ := strconv.Atoi(dotSplit[0])
		y, _ := strconv.Atoi(dotSplit[1])
		dots = append(dots, coord{x, y})
	}

	var paper paper = make([][]bool, yMax)
	for i := range paper {
		paper[i] = make([]bool, xMax)
	}

	for _, dot := range dots {
		paper[dot.y][dot.x] = true
	}

	return paper, foldHorizontally
}

func main() {
	paper, foldHorizontallyList := readInput()
	for i, foldHorizontally := range foldHorizontallyList {
		paper.fold(foldHorizontally)
		if i == 0 {
			fmt.Println("Answer part 1:", paper.countDots())
		}
	}
	paper.print()
}
