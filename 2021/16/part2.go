// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func convertHexRuneToDecimal(hex rune) int {
	if hex < rune('A') {
		return int(hex - rune('0'))
	} else {
		return int(hex - rune('A') + 10)
	}
}

func convertHexRuneToBinaryString(hex rune) string {
	var dToB = [...]string {
		"0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111",
			"1000", "1001", "1010", "1011", "1100", "1101", "1110", "1111"}
	return dToB[convertHexRuneToDecimal(hex)]
}

func binStringToInt(binString string) int {
	i, _ := strconv.ParseInt(binString, 2, 64)
	return int(i)
}

func readInput() string {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")
	input := rawLines[0]
	binString := ""

	for _, char := range input {
		binString += convertHexRuneToBinaryString(char)
	}
	return binString
}

func applyOperation(typeId int, args []int) int {
	var value int
	switch typeId {
	case 0: // Sum
		for _, n := range args {
			value += n
		}
	case 1: // Mul
		value = 1
		for _, n := range args {
			value *= n
		}
	case 2: // Min
		value = args[0]
		for _, n := range args {
			if n < value {
				value = n
			}
		}
	case 3: // Max
		value = args[0]
		for _, n := range args {
			if n > value {
				value = n
			}
		}
	case 5:
		if args[0] > args[1] {
			value = 1
		}
	case 6:
		if args[0] < args[1] {
			value = 1
		}
	case 7:
		if args[0] == args[1] {
			value = 1
		}
	}
	return value
}

func versionSumOfPacket(binString string) (int, int) {
	typeId := binStringToInt(binString[3:6])
	lenTypeId := binStringToInt(binString[6:7]) // Not used if typeId == 4
	var offset int
	var value int

	if typeId == 4 { // Literal value
		offset = 6
		stringValue := ""
		for {
			startBit := binString[offset]
			stringValue += binString[offset + 1:offset + 5]
			offset += 5
			if startBit == '0' {
				value = binStringToInt(stringValue)
				break
			}
		}
	} else if lenTypeId == 0 {
		subPacketLength := binStringToInt(binString[7:22])
		offset = 22
		endPacket := subPacketLength + offset
		args := make([]int, 0)
		for offset != endPacket {
			subValue, subOffset := versionSumOfPacket(binString[offset:])
			offset += subOffset
			args = append(args, subValue)
		}
		value = applyOperation(typeId, args)
	} else {
		noOfSubPackets := binStringToInt(binString[7:18])
		offset = 18
		args := make([]int, 0)
		for i := 0; i < noOfSubPackets; i++ {
			subValue, subOffset := versionSumOfPacket(binString[offset:])
			offset += subOffset
			args = append(args, subValue)
		}
		value = applyOperation(typeId, args)
	}
	return value, offset
}

func main() {
	binString := readInput()
	value, _ := versionSumOfPacket(binString)
	fmt.Println(value)
}
