package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func solveFor(knownRealm []map[int]int, value int, depth int) int {
	if depth == 0 {
		return 1
	}

	if result, ok := knownRealm[depth-1][value]; ok {
		return result
	}

	if value == 0 {
		result := solveFor(knownRealm, 1, depth-1)
		knownRealm[depth-1][value] = result
		return result
	}

	if characters := int(math.Floor(math.Log10(float64(value)))) + 1; characters%2 == 0 {
		divider := int(math.Pow10(characters / 2))
		stoneA := value / divider
		stoneB := value % divider

		result := solveFor(knownRealm, stoneA, depth-1) + solveFor(knownRealm, stoneB, depth-1)
		knownRealm[depth-1][value] = result
		return result
	}

	result := solveFor(knownRealm, value*2024, depth-1)
	knownRealm[depth-1][value] = result
	return result
}

func day11() (string, string) {

	data, err := os.ReadFile("./inputs/11")
	if err != nil {
		panic(err)
	}

	initialArrangement := strings.Split(strings.Trim(string(data), "\n"), " ")

	cache := make([]map[int]int, 75)
	for i := 0; i < len(cache); i += 1 {
		cache[i] = make(map[int]int)
	}

	stoneAmount25 := 0
	for _, stone := range initialArrangement {
		value, _ := strconv.ParseInt(stone, 10, 32)
		result := solveFor(cache, int(value), 25)
		stoneAmount25 += result
	}

	stoneAmount75 := 0
	for _, stone := range initialArrangement {
		value, _ := strconv.ParseInt(stone, 10, 32)
		result := solveFor(cache, int(value), 75)
		stoneAmount75 += result
	}

	return fmt.Sprintf("%d", stoneAmount25), fmt.Sprintf("%d", stoneAmount75)
}
