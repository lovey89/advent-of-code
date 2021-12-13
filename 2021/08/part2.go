// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"math"
)

type segmentSignals struct {
	activeSignals     [7]bool
	noOfActiveSignals int
}

func (s0 segmentSignals) isSubSetOf(s1 segmentSignals) bool {
	for i, activeSignal := range s0.activeSignals {
		if ! activeSignal {
			continue
		}
		if ! s1.activeSignals[i] {
			return false
		}
	}
	return true
}

type displayInfo struct {
	input  [10]segmentSignals
	output [4]segmentSignals
}

func createSignmentSignalsStruct(input string) segmentSignals {
	var segmentSignals segmentSignals
	segmentSignals.noOfActiveSignals = len(input)

	for _, signal := range input {
		segmentSignals.activeSignals[byte(signal) - 'a'] = true
	}

	return segmentSignals
}


func readInput() ([]displayInfo) {
	dat, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(dat), "\n")

	var displayInfos []displayInfo

	for _, line := range lines {
		if line == "" {
			continue
		}
		display := displayInfo{}
		inOutSplit := strings.Split(line, " | ")
		for i, in := range strings.Split(inOutSplit[0], " ") {
			display.input[i] = createSignmentSignalsStruct(in)
		}
		for i, out := range strings.Split(inOutSplit[1], " ") {
			display.output[i] = createSignmentSignalsStruct(out)
		}
		displayInfos = append(displayInfos, display)
	}
	return displayInfos
}

func findTrivialNumbers(display *displayInfo, numberMap *[10]segmentSignals) {
	for _, segmentSignals := range display.input {
		switch segmentSignals.noOfActiveSignals {
		case 2:
			numberMap[1] = segmentSignals
		case 3:
			numberMap[7] = segmentSignals
		case 4:
			numberMap[4] = segmentSignals
		case 7:
			numberMap[8] = segmentSignals
		}
	}
}

// Number 3 is the only 5 segment number which segments are superset of 1
func findNumber3(display *displayInfo, numberMap *[10]segmentSignals) {
	number1 := numberMap[1]
	for _, segmentSignals := range display.input {
		if segmentSignals.noOfActiveSignals == 5 &&
			number1.isSubSetOf(segmentSignals) {
			numberMap[3] = segmentSignals
			return
		}
	}
}

// Number 9 is the only 6 segment number which segments are superset of 4
func findNumber9(display *displayInfo, numberMap *[10]segmentSignals) {
	number4 := numberMap[4]
	for _, segmentSignals := range display.input {
		if segmentSignals.noOfActiveSignals == 6 &&
			number4.isSubSetOf(segmentSignals) {
			numberMap[9] = segmentSignals
			return
		}
	}
}

// Number 6 is the only 6 segment number which segments is not a superset of 1
func findNumber6(display *displayInfo, numberMap *[10]segmentSignals) {
	number1 := numberMap[1]
	for _, segmentSignals := range display.input {
		if segmentSignals.noOfActiveSignals == 6 &&
			! number1.isSubSetOf(segmentSignals) {
			numberMap[6] = segmentSignals
			return
		}
	}
}

// Number 0 is the only 6 segment number left after 6 and 9 have been found
func findNumber0(display *displayInfo, numberMap *[10]segmentSignals) {
	for _, segmentSignals := range display.input {
		if segmentSignals.noOfActiveSignals == 6 &&
			segmentSignals != numberMap[6] &&
			segmentSignals != numberMap[9] {
			numberMap[0] = segmentSignals
			return
		}
	}
}

// Number 5 is the only 5 segment number which segments are a subset of 6
func findNumber5(display *displayInfo, numberMap *[10]segmentSignals) {
	number6 := numberMap[6]
	for _, segmentSignals := range display.input {
		if segmentSignals.noOfActiveSignals == 5 &&
			segmentSignals.isSubSetOf(number6) {
			numberMap[5] = segmentSignals
			return
		}
	}
}

// Number 2 is the only 5 segment number left after 3 and 5 have been found
func findNumber2(display *displayInfo, numberMap *[10]segmentSignals) {
	for _, segmentSignals := range display.input {
		if segmentSignals.noOfActiveSignals == 5 &&
			segmentSignals != numberMap[3] &&
			segmentSignals != numberMap[5] {
			numberMap[2] = segmentSignals
			return
		}
	}
}

func getIndexOfSegment(segment segmentSignals, numberMap [10]segmentSignals) int {
	for i, elem := range numberMap {
		if segment == elem {
			return i
		}
	}
	return -1
}

func powInt(x, y int) int {
    return int(math.Pow(float64(x), float64(y)))
}

func main() {
	displayInfos := readInput()
	sum := 0

	for _, displayInfo := range displayInfos {
		numberMap := [10]segmentSignals{}
		findTrivialNumbers(&displayInfo, &numberMap)
		findNumber3(&displayInfo, &numberMap)
		findNumber9(&displayInfo, &numberMap)
		findNumber6(&displayInfo, &numberMap)
		findNumber0(&displayInfo, &numberMap)
		findNumber5(&displayInfo, &numberMap)
		findNumber2(&displayInfo, &numberMap)

		for i, output := range displayInfo.output {
			number := getIndexOfSegment(output, numberMap)
			sum += number * powInt(10, 3 - i)
		}
	}
	fmt.Println(sum)
}
