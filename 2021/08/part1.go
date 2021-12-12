// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
)

type segmentInfo struct {
	input  [10]string
	output [4]string
}


func readInput() ([]segmentInfo) {
	dat, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(dat), "\n")

	var segmentInfos []segmentInfo

	for _, line := range lines {
		if line == "" {
			continue
		}
		segment := segmentInfo{}
		inOutSplit := strings.Split(line, " | ")
		for i, in := range strings.Split(inOutSplit[0], " ") {
			segment.input[i] = in
		}
		for i, out := range strings.Split(inOutSplit[1], " ") {
			segment.output[i] = out
		}
		segmentInfos = append(segmentInfos, segment)
	}
	return segmentInfos
}

func main() {
	segmentInfos := readInput()

	unique := 0

	for _, segmentInfo := range segmentInfos {
		for _, output := range segmentInfo.output {
			signalLength := len(output)
			switch signalLength {
			case 2, 3, 4, 7:
				unique++
			}
		}
	}
	fmt.Println(unique)
}
