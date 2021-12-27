// go run part2.go

package main

import (
	"fmt"
	"os"
	"strings"
	pq "github.com/lovey89/advent-of-code/2021/15/priorityqueue"
	"math"
)

type cave struct {
	riskLevel [][]int
	visited   [][]bool
	distance  [][]pq.Node
	height    int
	width     int
	scale     int
}

func (c *cave) getRiskLevel(x, y int) int {
	riskHeight := c.height / c.scale
	riskWidth := c.width / c.scale

	yRiskAddition := y / riskHeight
	xRiskAddition := x / riskWidth

	yCoord := y % riskHeight
	xCoord := x % riskWidth

	origRisk := c.riskLevel[yCoord][xCoord]
	return (origRisk - 1 + yRiskAddition + xRiskAddition) % 9 + 1
}

func readInput() [][]int {
	dat, _ := os.ReadFile("./input.txt")
	rawLines := strings.Split(string(dat), "\n")

	cave := make([][]int, 0)

	for _, line := range rawLines {
		if line == "" {
			continue
		}
		caveLine := make([]int, 0)
		for _, char := range line {
			caveLine = append(caveLine, int(char - '0'))
		}
		cave = append(cave, caveLine)
	}
	return cave
}

func initCave(c [][]int) cave {
	scale := 5
	dy := len(c) * scale
	dx := len(c[0]) * scale

	visited := make([][]bool, dy)
	for i := range visited {
		visited[i] = make([]bool, dx)
	}

	distance := make([][]pq.Node, dy)
	for y := range distance {
		s := make([]pq.Node, dx)
		for x := range s {
			s[x] = pq.Node{X: x, Y: y, TotalDistance: math.MaxInt32}
		}
		distance[y] = s
	}

	return cave{c, visited, distance, dy, dx, scale}
}

func shortestPath(c [][]int) int {

	cave := initCave(c)

	queue := pq.CreatePriorityQueue()
	cave.distance[0][0].TotalDistance = 0
	queue.Add(&cave.distance[0][0])

	endX, endY := cave.width - 1, cave.height - 1

	for queue.Len() > 0 {
		minNode := queue.RemoveMin()

		x, y := minNode.X, minNode.Y
		cave.visited[y][x] = true

		if x == endX && y == endY {
			return minNode.TotalDistance
		}
		updateNeighbours(&cave, &queue, x, y)
	}

	return -1
}

func updateNeighbours(cave *cave, queue *pq.PriorityQueue, x, y int) {
	baseDistance := cave.distance[y][x].TotalDistance
	updateNeighbour(cave, queue, x - 1, y, baseDistance)
	updateNeighbour(cave, queue, x + 1, y, baseDistance)
	updateNeighbour(cave, queue, x, y - 1, baseDistance)
	updateNeighbour(cave, queue, x, y + 1, baseDistance)
}

func updateNeighbour(cave *cave, queue *pq.PriorityQueue, x, y, baseDistance int) {
	if x < 0 ||
		y < 0 ||
		x >= cave.width ||
		y >= cave.height ||
		cave.visited[y][x] {
		return
	}

	newDist := baseDistance + cave.getRiskLevel(x, y)
	oldDist := cave.distance[y][x].TotalDistance
	if newDist >= oldDist {
		return
	}
	cave.distance[y][x].TotalDistance = newDist
	if oldDist == math.MaxInt32 {
		queue.Add(&cave.distance[y][x])
	} else {
		queue.Update(&cave.distance[y][x], newDist)
	}
}

func main() {
	cave := readInput()

	shortest := shortestPath(cave)
	fmt.Println(shortest)
}
