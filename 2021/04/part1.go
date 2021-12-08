// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

type bingoCoord struct {
	board, x, y int
}

type numberInformation struct {
	locations []bingoCoord
	filled bool
}

func readInput() ([]int, [][5][5]int, [100]numberInformation) {
	dat, _ := os.ReadFile("./input.txt")
	sections := strings.Split(string(dat), "\n\n")

	rawDrawnNumbers := sections[0]

	drawnNumbers := make([]int, 0)
	for _, numberString := range strings.Split(rawDrawnNumbers, ",") {
		number, _ := strconv.Atoi(numberString)
		drawnNumbers = append(drawnNumbers, number)
	}

	rawBoards := sections[1:]
	boards := make([][5][5]int, 0)
	numberInformations := [100]numberInformation{}

	for b, rawBoard := range rawBoards {
		board := [5][5]int{}
		for y, row := range strings.Split(rawBoard, "\n") {
			for x, numberString := range strings.Fields(row) {
				number, _ := strconv.Atoi(numberString)
				board[x][y] = number
				numberInformations[number].locations =
					append(numberInformations[number].locations, bingoCoord{b, x, y})
			}
		}
		boards = append(boards, board)
	}

	return drawnNumbers, boards, numberInformations
}

func checkBingo(numberInformations [100]numberInformation, board [5][5]int, x, y int) bool {
	isBingo := true
	for i := 0; i < len(board); i++ {
		number := board[x][i]
		if !numberInformations[number].filled {
			isBingo = false
			break
		}
	}
	if isBingo {
		return true
	}

	for i := 0; i < len(board); i++ {
		number := board[i][y]
		if !numberInformations[number].filled {
			return false
		}
	}
	return true
}

func calculateRemaining(numberInformations [100]numberInformation, board [5][5]int) (sum int) {
	for i := 0; i < len(board); i++ {
		// I'll just assume it's a quadratic board
		for j := 0; j < len(board); j++ {
			number := board[i][j]
			if !numberInformations[number].filled {
				sum += number
			}
		}
	}
	return
}

func main() {
	drawnNumbers, boards, numberInformations := readInput()
	for _, number := range drawnNumbers {
		numberInformations[number].filled = true
		numberLocations := numberInformations[number].locations
		for _, numberLocation := range numberLocations {
			board := boards[numberLocation.board]
			x := numberLocation.x
			y := numberLocation.y
			if checkBingo(numberInformations, board, x, y) {
				rem := calculateRemaining(numberInformations, board)
				fmt.Println(number * rem)
				return
			}
		}
	}
}
