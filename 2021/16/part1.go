// go run part1.go

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

func versionSumOfPacket(binString string) (int, int) {
	version := binStringToInt(binString[0:3])
	typeId := binStringToInt(binString[3:6])
	lenTypeId := binStringToInt(binString[6:7]) // Not used if typeId == 4
	var offset int

	if typeId == 4 { // Literal value
		offset = 6
		for {
			startBit := binString[offset]
			offset += 5
			if startBit == '0' {
				break
			}
		}
	} else if lenTypeId == 0 {
		subPacketLength := binStringToInt(binString[7:22])
		offset = 22
		endPacket := subPacketLength + offset
		for offset != endPacket {
			subVersion, subOffset := versionSumOfPacket(binString[offset:])
			offset += subOffset
			version += subVersion
		}
	} else {
		noOfSubPackets := binStringToInt(binString[7:18])
		offset = 18
		for i := 0; i < noOfSubPackets; i++ {
			subVersion, subOffset := versionSumOfPacket(binString[offset:])
			version += subVersion
			offset += subOffset
		}
	}
	return version, offset
}

func main() {
	binString := readInput()
	versionSum, _ := versionSumOfPacket(binString)
	fmt.Println(versionSum)
}
