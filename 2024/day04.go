package main

import (
	"fmt"
	"os"
	"strings"
)

type vector struct {
	x int
	y int
}

var directions = []vector{
	{-1, -1},
	{-1, 0},
	{-1, 1},
	{0, -1},
	{0, 1},
	{1, -1},
	{1, 0},
	{1, 1},
}

type x_masAlternatives struct {
	displacement vector
	direction    vector
}

type x_masPattern struct {
	direction    vector
	alternatives []x_masAlternatives
}

var x_masPatterns = []x_masPattern{
	{vector{1, 1}, []x_masAlternatives{
		{vector{0, 2}, vector{1, -1}},
		{vector{2, 0}, vector{-1, 1}},
	}},
	{vector{1, -1}, []x_masAlternatives{
		{vector{0, -2}, vector{1, 1}},
		{vector{2, 0}, vector{-1, -1}},
	}},
	{vector{-1, 1}, []x_masAlternatives{
		{vector{0, 2}, vector{-1, -1}},
		{vector{-2, 0}, vector{1, 1}},
	}},
	{vector{-1, -1}, []x_masAlternatives{
		{vector{0, -2}, vector{-1, 1}},
		{vector{-2, 0}, vector{1, -1}},
	}},
}

func search(grid []string, pos, dir vector, remainingLetters string) bool {
	if len(remainingLetters) == 0 {
		return true
	}

	if pos.x < 0 || pos.x >= len(grid) {
		return false
	}

	if pos.y < 0 || pos.y >= len(grid[pos.x]) {
		return false
	}

	if grid[pos.x][pos.y] != remainingLetters[0] {
		return false
	}

	return search(grid, vector{pos.x + dir.x, pos.y + dir.y}, dir, remainingLetters[1:])
}

func day04() (string, string) {

	data, err := os.ReadFile("./inputs/04")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	xmasMatches := 0
	for i := range lines {
		for j := range lines[i] {
			for _, dir := range directions {
				if search(lines, vector{i, j}, dir, "XMAS") {
					xmasMatches += 1
				}
			}
		}
	}

	x_masMatches := 0
	for i := range lines {
		for j := range lines[i] {
			for _, pattern := range x_masPatterns {
				if search(lines, vector{i, j}, pattern.direction, "MAS") {
					for _, alternative := range pattern.alternatives {
						if search(
							lines,
							vector{i + alternative.displacement.x, j + alternative.displacement.y},
							alternative.direction,
							"MAS",
						) {
							x_masMatches += 1
						}
					}

				}
			}
		}
	}

	return fmt.Sprintf("%d", xmasMatches), fmt.Sprintf("%d", x_masMatches/2)

}
