package main

import (
	"fmt"
	"os"
	"strings"
)

var walkingCycles = []vector{
	{-1, 0},
	{0, 1},
	{1, 0},
	{0, -1},
}

func (v *vector) add(other *vector) *vector {
	return &vector{v.x + other.x, v.y + other.y}
}

func (v *vector) equals(other *vector) bool {
	if other == nil {
		return false
	}
	return v.x == other.x && v.y == other.y
}

func (v *vector) String() string {
	return fmt.Sprintf("(%d, %d)", v.x, v.y)
}

func simulate(lines []string, customWall *vector) int {

	guardPosition := vector{0, 0}
outer:
	for i, line := range lines {
		for j, c := range line {
			if rune(c) == '^' {
				guardPosition = vector{i, j}
				break outer
			}
		}
	}

	currentWalkingCycle := 0
	visited := make(map[string]bool)
	visited[guardPosition.String()] = true
	state := make(map[string]bool)
	state[fmt.Sprintf("%s%d", guardPosition.String(), currentWalkingCycle)] = true

simulation:
	for {
		tileInFront := guardPosition.add(&walkingCycles[currentWalkingCycle])
		if tileInFront.x < 0 || tileInFront.x >= len(lines) {
			break simulation
		}
		if tileInFront.y < 0 || tileInFront.y >= len(lines[tileInFront.x]) {
			break simulation
		}
		if rune(lines[tileInFront.x][tileInFront.y]) == '#' || tileInFront.equals(customWall) {
			currentWalkingCycle = (currentWalkingCycle + 1) % len(walkingCycles)
			continue simulation
		}

		guardPosition = *tileInFront
		visited[guardPosition.String()] = true
		if _, ok := state[fmt.Sprintf("%s%d", guardPosition.String(), currentWalkingCycle)]; ok {
			return 0
		} else {
			state[fmt.Sprintf("%s%d", guardPosition.String(), currentWalkingCycle)] = true
		}
	}

	return len(visited)
}

func day06() (string, string) {

	data, err := os.ReadFile("./inputs/06")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	possibleWallPlacement := 0
	for i, line := range lines {
		for j, c := range line {
			if rune(c) == '.' {
				customWall := vector{i, j}
				if stuckInLoop := simulate(lines, &customWall); stuckInLoop == 0 {
					possibleWallPlacement += 1
				}
			}
		}
	}

	return fmt.Sprintf("%d", simulate(lines, nil)), fmt.Sprintf("%d", possibleWallPlacement)

}
