// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func readInput() []string {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")

	instructions := make([]string, 0)
	for _, elem := range rawLines {
		if elem != "" {
			instructions = append(instructions, elem)
		}
	}
	return instructions
}

func partition(ss []string, test func(string) bool) (trueSlice []string, falseSlice []string) {
	for _, s := range ss {
		if test(s) {
			trueSlice = append(trueSlice, s)
		} else {
			falseSlice = append(falseSlice, s)
		}
	}
	return
}

func main() {
	input := readInput()

	ones, zeros := partition(input, func(s string) bool { return s[0] == '1' })
	var oxy []string
	var co2 []string
	if len(ones) >= len(zeros) {
		oxy = ones
		co2 = zeros
	} else {
		oxy = zeros
		co2 = ones
	}

	index := 1
	for len(oxy) > 1 {
		ones, zeros = partition(oxy, func(s string) bool { return s[index] == '1' })
		if len(ones) >= len(zeros) {
			oxy = ones
		} else {
			oxy = zeros
		}
		index++
	}
	index = 1
	for len(co2) > 1 {
		ones, zeros = partition(co2, func(s string) bool { return s[index] == '1' })
		if len(ones) < len(zeros) {
			co2 = ones
		} else {
			co2 = zeros
		}
		index++
	}
	oxyDec, _ := strconv.ParseInt(oxy[0], 2, 32)
	co2Dec, _ := strconv.ParseInt(co2[0], 2, 32)

	fmt.Println(oxyDec * co2Dec)
}
