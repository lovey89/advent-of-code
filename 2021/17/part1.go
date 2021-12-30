// go run part1.go

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
	"math"
)

func readInput() (minX, maxX, minY, maxY int) {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")
	input := rawLines[0][15:]

	xySplit := strings.Split(input, ", y=")
	xSplit := strings.Split(xySplit[0], "..")
	ySplit := strings.Split(xySplit[1], "..")
	x0, _ := strconv.ParseInt(xSplit[0], 10, 32)
	x1, _ := strconv.ParseInt(xSplit[1], 10, 32)
	y0, _ := strconv.ParseInt(ySplit[0], 10, 32)
	y1, _ := strconv.ParseInt(ySplit[1], 10, 32)
	if x0 < x1 {
		minX, maxX = int(x0), int(x1)
	} else {
		minX, maxX = int(x1), int(x0)
	}
	if y0 < y1 {
		minY, maxY = int(y0), int(y1)
	} else {
		minY, maxY = int(y1), int(y0)
	}
	return
}

func findValidXVelocities(minX, maxX int) (map[int]struct{}, int) {
	// Reverse the arithmetic sum to find the minimum valid x velocity
	// Arithmetic sum: n * (n+1) / 2 = s
	// Reverse it: n = -0.5 + sqrt(0.25 + 2s)
	minXVel := int(math.Ceil(-0.5 + math.Sqrt(0.25 + 2 * float64(minX))))

	validTimes := make([]int, 0)
	minimumSafeTime := 0

	validVelocities := make(map[int]struct{})
	for i := minXVel; i <= maxX; i++ {
		time := 0
		velocity := i
		position := 0
		for {
			time++
			position += velocity
			velocity--

			if position >= minX && position <= maxX {
				validTimes = append(validTimes, time)
				if velocity == 0 && (minimumSafeTime == 0 || time < minimumSafeTime) {
					minimumSafeTime = time
				}
			}
			if velocity == 0 || position > maxX {
				break
			}
		}
	}

	for _, t := range validTimes {
		if t < minimumSafeTime {
			validVelocities[t] = struct{}{}
		}
	}

	return validVelocities, minimumSafeTime
}

func findMaxAltitudeGivenValidTimes(minY, maxY, minimumSafeTime int, validTimes map[int]struct{}) int {
	for i := -minY; i >= 0; i-- {
		p := 0
		t := 0
		for v := i; p >= minY; v++ {
			p -= v
			t++
			if p <= maxY && p >= minY {
				iv := i - 1
				timeToTravel := iv * 2 + 1 + t // iv * 2 + 1 is the time to reach height 0 again
				_, isValidTime := validTimes[timeToTravel]
				if isValidTime || timeToTravel >= minimumSafeTime {
					return iv * (iv + 1) / 2
				}
			}
		}
	}
	return -1
}

func main() {
	minX, maxX, minY, maxY := readInput()
	validTimes, minimumSafeTime := findValidXVelocities(minX, maxX)
	fmt.Println(findMaxAltitudeGivenValidTimes(minY, maxY, minimumSafeTime, validTimes))
}
