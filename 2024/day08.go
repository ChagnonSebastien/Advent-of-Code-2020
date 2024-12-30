package main

import (
	"fmt"
	"os"
	"strings"
)

func (v *vector) invert() *vector {
	return &vector{-v.x, -v.y}
}

func (v *vector) sub(other *vector) *vector {
	return v.add(other.invert())
}

func (v *vector) inBounds(minX, maxX, minY, maxY int) bool {
	return v.x >= minX && v.x <= maxX && v.y >= minY && v.y <= maxY
}

func day08() (string, string) {

	data, err := os.ReadFile("./inputs/08")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	antenas := make(map[rune][]vector)

	for i, line := range lines {
		for j, char := range line {
			if rune(char) != '.' {
				if _, ok := antenas[char]; !ok {
					antenas[char] = make([]vector, 0)
				}
				antenas[char] = append(antenas[char], vector{i, j})
			}
		}
	}

	maxX := len(lines) - 1
	maxY := len(lines[0]) - 1
	antiNodes := make(map[string]bool)
	resonantAntiNodes := make(map[string]bool)

	for _, nodes := range antenas {
		for a := 1; a < len(nodes); a += 1 {
			nodeA := nodes[a]
			for b := 0; b < a; b += 1 {
				nodeB := nodes[b]

				diff := nodeB.sub(&nodeA)
				resonantAntiNodes[nodeA.String()] = true
				antinodeA := nodeA.sub(diff)
				if antinodeA.inBounds(0, maxX, 0, maxY) {
					antiNodes[antinodeA.String()] = true

					for antinodeA.inBounds(0, maxX, 0, maxY) {
						resonantAntiNodes[antinodeA.String()] = true
						antinodeA = antinodeA.sub(diff)
					}
				}

				antinodeB := nodeB.add(diff)
				resonantAntiNodes[nodeB.String()] = true
				if antinodeB.inBounds(0, maxX, 0, maxY) {
					antiNodes[antinodeB.String()] = true

					for antinodeB.inBounds(0, maxX, 0, maxY) {
						resonantAntiNodes[antinodeB.String()] = true
						antinodeB = antinodeB.add(diff)
					}
				}
			}
		}
	}

	return fmt.Sprintf("%d", len(antiNodes)), fmt.Sprintf("%d", len(resonantAntiNodes))

}
