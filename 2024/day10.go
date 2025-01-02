package main

import (
	"fmt"
	"os"
	"strings"
)

var manhattanDirections = []vector{
	{1, 0},
	{-1, 0},
	{0, 1},
	{0, -1},
}

func findTrails(heightmap []string, pos *vector, previousHeight uint8) (map[string]bool, int) {
	if !pos.inBounds(0, len(heightmap)-1, 0, len(heightmap[0])-1) {
		return map[string]bool{}, 0
	}

	newHeight := heightmap[pos.x][pos.y]
	if newHeight-previousHeight != 1 {
		return map[string]bool{}, 0
	}

	if newHeight == '9' {
		return map[string]bool{
			pos.String(): true,
		}, 1
	}

	reachableTrailheads := make(map[string]bool)
	totalAmount := 0
	for _, possibility := range manhattanDirections {
		trailheads, amount := findTrails(heightmap, pos.add(&possibility), newHeight)
		totalAmount += amount
		for trailhead := range trailheads {
			reachableTrailheads[trailhead] = true
		}
	}
	return reachableTrailheads, totalAmount
}

func day10() (string, string) {

	data, err := os.ReadFile("./inputs/10")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	trailheadsScoreSum := 0
	totalTrailAmounts := 0
	for i, line := range lines {
		for j, height := range line {
			if height == '0' {
				reachablePeaks, amount := findTrails(lines, &vector{i, j}, '0'-1)
				trailheadsScoreSum += len(reachablePeaks)
				totalTrailAmounts += amount
			}
		}
	}

	return fmt.Sprintf("%d", trailheadsScoreSum), fmt.Sprintf("%d", totalTrailAmounts)

}
